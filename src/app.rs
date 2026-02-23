use crate::config::store::{load_settings, save_score, save_settings};
use crate::data::themes::get_theme;
use crate::engine::input_processor::create_input_state;
use crate::engine::stats_calculator::{calculate_consistency, calculate_stats};
use crate::engine::timer::{
    create_timer, get_elapsed_seconds, start_timer, tick_timer,
};
use crate::engine::word_generator::generate_words;
use crate::screens::{menu, results, scores, test};
use crate::types::*;
use crate::ui::header::HEADER_HEIGHT;
use crate::ui::results_chart::CHART_HEIGHT;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::Frame;
use std::time::Instant;

pub struct App {
    pub screen: Screen,
    pub config: TestConfig,
    // Test state
    pub phase: TestPhase,
    pub input_state: Option<InputState>,
    pub words: Vec<String>,
    pub timer: TimerState,
    pub current_stats: StatsSnapshot,
    pub wpm_history: Vec<f64>,
    pub result: Option<TestResult>,
    pub restart_pending: bool,
    // Last tick stats tracking
    last_stats_tick: Option<Instant>,
    // Results
    pub result_wpm_history: Vec<f64>,
    // Terminal size
    pub terminal_width: u16,
    pub terminal_height: u16,
    // Menu state
    pub menu_state: menu::MenuState,
    // Scores state
    pub scores_scroll: usize,
    // Flag for quit
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let config = load_settings();
        Self {
            screen: Screen::Menu,
            menu_state: menu::MenuState::new(&config),
            config,
            phase: TestPhase::Idle,
            input_state: None,
            words: Vec::new(),
            timer: create_timer(None),
            current_stats: StatsSnapshot::default(),
            wpm_history: Vec::new(),
            result: None,
            restart_pending: false,
            last_stats_tick: None,
            result_wpm_history: Vec::new(),
            terminal_width: 80,
            terminal_height: 24,
            scores_scroll: 0,
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Ctrl+C always quits
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return;
        }

        match self.screen {
            Screen::Menu => self.handle_menu_key(key),
            Screen::Test => self.handle_test_key(key),
            Screen::Results => self.handle_results_key(key),
            Screen::Scores => self.handle_scores_key(key),
        }
    }

    pub fn handle_resize(&mut self, w: u16, h: u16) {
        self.terminal_width = w;
        self.terminal_height = h;
    }

    pub fn tick(&mut self) {
        if self.screen != Screen::Test || self.phase != TestPhase::Active {
            return;
        }

        let now = Instant::now();
        self.timer = tick_timer(self.timer.clone(), now);

        // Tick stats every second
        let should_tick_stats = self
            .last_stats_tick
            .map(|t| now.duration_since(t).as_secs_f64() >= 1.0)
            .unwrap_or(true);

        if should_tick_stats {
            self.last_stats_tick = Some(now);
            if let Some(ref input) = self.input_state {
                let elapsed = get_elapsed_seconds(&self.timer);
                let stats = calculate_stats(input, elapsed);
                // Only record WPM history after 1s to avoid near-zero division spikes
                if elapsed >= 1.0 {
                    self.wpm_history.push(stats.raw_wpm);
                }
                self.current_stats = stats;
            }
        }

        // Handle timer expiry (time mode)
        if self.timer.expired {
            self.finish_test();
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let theme = get_theme(&self.config.theme);
        let area = frame.area();

        // Set background color
        let bg = crate::data::themes::parse_hex(&theme.bg);
        let bg_style = ratatui::style::Style::default().bg(bg);
        frame.render_widget(ratatui::widgets::Clear, area);
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                frame.buffer_mut().set_style(
                    ratatui::layout::Rect::new(x, y, 1, 1),
                    bg_style,
                );
            }
        }

        // Compute centered content area per screen
        let content = self.centered_content_area(area);

        match self.screen {
            Screen::Menu => menu::render(frame, content, self, theme),
            Screen::Test => test::render(frame, content, self, theme),
            Screen::Results => results::render(frame, content, self, theme),
            Screen::Scores => scores::render(frame, content, self, theme),
        }
    }

    fn centered_content_area(&self, area: Rect) -> Rect {
        let max_width: u16 = 120;
        let content_height = match self.screen {
            Screen::Menu => {
                if self.menu_state.picker.is_some() {
                    // Picker needs more vertical space
                    area.height.saturating_sub(4)
                } else {
                    let field_count = menu::field_count(&self.config);
                    HEADER_HEIGHT + field_count as u16 + 4
                }
            }
            Screen::Test => 12,
            Screen::Results => 2 + 3 + 2 + 2 + CHART_HEIGHT + 2,
            Screen::Scores => area.height.saturating_sub(4),
        };
        let w = max_width.min(area.width);
        let h = content_height.min(area.height);
        let x = area.x + area.width.saturating_sub(w) / 2;
        let y = area.y + area.height.saturating_sub(h) / 2;
        Rect::new(x, y, w, h)
    }

    // ── Menu ────────────────────────────────────────────────────────

    fn handle_menu_key(&mut self, key: KeyEvent) {
        let result = menu::handle_key(&mut self.menu_state, key, &self.config);
        match result {
            menu::MenuAction::None => {}
            menu::MenuAction::Start(new_config) => {
                save_settings(&new_config);
                self.config = new_config;
                self.start_test();
            }
            menu::MenuAction::UpdateConfig(new_config) => {
                self.config = new_config;
            }
            menu::MenuAction::Scores => {
                self.scores_scroll = 0;
                self.screen = Screen::Scores;
            }
        }
    }

    // ── Test ────────────────────────────────────────────────────────

    fn start_test(&mut self) {
        let count = match self.config.mode {
            TestMode::Words => self.config.word_count as usize,
            TestMode::Time => 100,
        };
        let words = generate_words(&GeneratorConfig {
            language: self.config.language.clone(),
            count,
            punctuation: self.config.punctuation,
            numbers: self.config.numbers,
        });
        self.words = words.clone();
        self.input_state = Some(create_input_state(&words));
        let limit = match self.config.mode {
            TestMode::Time => Some(self.config.time_limit),
            TestMode::Words => None,
        };
        self.timer = create_timer(limit);
        self.phase = TestPhase::Idle;
        self.current_stats = StatsSnapshot::default();
        self.wpm_history = Vec::new();
        self.result = None;
        self.restart_pending = false;
        self.last_stats_tick = None;
        self.screen = Screen::Test;
    }

    fn handle_test_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Esc {
            self.restart_pending = false;
            self.screen = Screen::Menu;
            self.menu_state = menu::MenuState::new(&self.config);
            return;
        }

        if key.code == KeyCode::Tab {
            self.restart_pending = true;
            return;
        }

        if self.restart_pending {
            if key.code == KeyCode::Enter {
                self.restart_pending = false;
                self.start_test();
            } else {
                self.restart_pending = false;
            }
            return;
        }

        if self.phase == TestPhase::Finished {
            return;
        }

        let is_backspace = key.code == KeyCode::Backspace || key.code == KeyCode::Delete;
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        // Block backspace when disabled in config
        if is_backspace && !self.config.backspace {
            return;
        }

        let input = match key.code {
            KeyCode::Char(c) => {
                if is_ctrl {
                    return; // Don't process ctrl+char as typing (except ctrl+backspace)
                }
                c.to_string()
            }
            KeyCode::Backspace | KeyCode::Delete => String::new(),
            _ => return,
        };

        if let Some(state) = self.input_state.take() {
            let new_state =
                crate::engine::input_processor::process_keystroke(state, &input, is_backspace, is_ctrl);

            // Start on first keystroke
            if self.phase == TestPhase::Idle {
                self.phase = TestPhase::Active;
                self.timer = start_timer(self.timer.clone(), Instant::now());
            }

            // In words mode, check if all words are completed
            if self.config.mode == TestMode::Words && new_state.finished {
                self.input_state = Some(new_state);
                self.finish_test();
                return;
            }

            self.input_state = Some(new_state);
        }
    }

    fn finish_test(&mut self) {
        self.phase = TestPhase::Finished;
        if let Some(ref input) = self.input_state {
            let elapsed = get_elapsed_seconds(&self.timer);
            let stats = calculate_stats(input, elapsed);
            let consistency = calculate_consistency(&self.wpm_history);

            let mut correct_words = 0;
            let mut total_words = 0;
            for w in &input.words {
                if w.completed {
                    total_words += 1;
                    if w.typed == w.word {
                        correct_words += 1;
                    }
                }
            }

            let result = TestResult {
                wpm: stats.wpm.round() as u32,
                raw_wpm: stats.raw_wpm.round() as u32,
                accuracy: stats.accuracy,
                consistency,
                correct_chars: stats.correct_chars,
                incorrect_chars: stats.incorrect_chars,
                extra_chars: stats.extra_chars,
                missed_chars: stats.missed_chars,
                total_words,
                correct_words,
                elapsed_seconds: elapsed,
                config: self.config.clone(),
            };

            save_score(&result);
            self.result = Some(result);
            self.result_wpm_history = self.wpm_history.clone();
            self.screen = Screen::Results;
        }
    }

    // ── Results ─────────────────────────────────────────────────────

    fn handle_results_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => self.start_test(),
            KeyCode::Esc => {
                self.screen = Screen::Menu;
                self.menu_state = menu::MenuState::new(&self.config);
            }
            _ => {}
        }
    }

    // ── Scores ──────────────────────────────────────────────────────

    fn handle_scores_key(&mut self, key: KeyEvent) {
        let scores = crate::config::store::get_scores();
        let visible_rows = self.terminal_height.saturating_sub(8).max(5) as usize;

        match key.code {
            KeyCode::Esc => {
                self.screen = Screen::Menu;
                self.menu_state = menu::MenuState::new(&self.config);
            }
            KeyCode::Up => {
                self.scores_scroll = self.scores_scroll.saturating_sub(1);
            }
            KeyCode::Down => {
                let max = scores.len().saturating_sub(visible_rows);
                self.scores_scroll = (self.scores_scroll + 1).min(max);
            }
            _ => {}
        }
    }
}
