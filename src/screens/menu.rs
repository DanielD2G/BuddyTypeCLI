use crate::app::App;
use crate::data::languages::get_available_languages;
use crate::data::themes::{get_theme_names, parse_hex};
use crate::types::{TestConfig, TestMode, ThemeColors};
use crate::ui::header::{Header, HEADER_HEIGHT};
use crate::ui::picker::{Picker, PickerResult, PickerState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Frame;

const TIME_OPTIONS: &[u32] = &[15, 30, 60, 120];
const WORD_OPTIONS: &[u32] = &[10, 25, 50, 100];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuField {
    Mode,
    Time,
    Words,
    Language,
    Theme,
    OneLine,
    Punctuation,
    Numbers,
    Backspace,
}

pub struct MenuState {
    pub selected_field: MenuField,
    pub picker: Option<PickerKind>,
    pub theme_preview_base: Option<String>,
}

pub enum PickerKind {
    Language(PickerState),
    Theme(PickerState),
}

pub enum MenuAction {
    None,
    Start(TestConfig),
    UpdateConfig(TestConfig),
    Scores,
}

impl MenuState {
    pub fn new(_config: &TestConfig) -> Self {
        Self {
            selected_field: MenuField::Mode,
            picker: None,
            theme_preview_base: None,
        }
    }
}

pub fn field_count(config: &TestConfig) -> usize {
    get_fields(config).len()
}

fn get_fields(config: &TestConfig) -> Vec<MenuField> {
    let mut fields = vec![MenuField::Mode];
    match config.mode {
        TestMode::Time => fields.push(MenuField::Time),
        TestMode::Words => fields.push(MenuField::Words),
    }
    fields.extend([
        MenuField::Language,
        MenuField::Theme,
        MenuField::OneLine,
        MenuField::Punctuation,
        MenuField::Numbers,
        MenuField::Backspace,
    ]);
    fields
}

pub fn handle_key(state: &mut MenuState, key: KeyEvent, config: &TestConfig) -> MenuAction {
    // If picker is open, delegate to it
    if let Some(ref mut picker_kind) = state.picker {
        match picker_kind {
            PickerKind::Language(ps) => {
                let result = ps.handle_key(key);
                match result {
                    PickerResult::Selected(val) => {
                        let mut new_config = config.clone();
                        new_config.language = val;
                        state.picker = None;
                        return MenuAction::UpdateConfig(new_config);
                    }
                    PickerResult::Cancelled => {
                        state.picker = None;
                    }
                    _ => {}
                }
                return MenuAction::None;
            }
            PickerKind::Theme(ps) => {
                let result = ps.handle_key(key);
                match result {
                    PickerResult::Selected(val) => {
                        let mut new_config = config.clone();
                        new_config.theme = val;
                        state.picker = None;
                        state.theme_preview_base = None;
                        return MenuAction::UpdateConfig(new_config);
                    }
                    PickerResult::Cancelled => {
                        if let Some(base) = state.theme_preview_base.take() {
                            let mut new_config = config.clone();
                            new_config.theme = base;
                            state.picker = None;
                            return MenuAction::UpdateConfig(new_config);
                        }
                        state.picker = None;
                    }
                    PickerResult::Highlighted(val) => {
                        let mut new_config = config.clone();
                        new_config.theme = val;
                        return MenuAction::UpdateConfig(new_config);
                    }
                    _ => {}
                }
                return MenuAction::None;
            }
        }
    }

    let fields = get_fields(config);
    let current_idx = fields
        .iter()
        .position(|f| *f == state.selected_field)
        .unwrap_or(0);

    match key.code {
        KeyCode::Char('s') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            return MenuAction::Scores;
        }
        KeyCode::Enter => {
            if state.selected_field == MenuField::Language {
                let items: Vec<String> =
                    get_available_languages().into_iter().map(String::from).collect();
                state.picker = Some(PickerKind::Language(PickerState::new(
                    items,
                    config.language.clone(),
                )));
                return MenuAction::None;
            }
            if state.selected_field == MenuField::Theme {
                state.theme_preview_base = Some(config.theme.clone());
                let items: Vec<String> =
                    get_theme_names().into_iter().map(String::from).collect();
                state.picker = Some(PickerKind::Theme(PickerState::new(
                    items,
                    config.theme.clone(),
                )));
                return MenuAction::None;
            }
            return MenuAction::Start(config.clone());
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let new_idx = current_idx.saturating_sub(1);
            state.selected_field = fields[new_idx];
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let new_idx = (current_idx + 1).min(fields.len() - 1);
            state.selected_field = fields[new_idx];
        }
        KeyCode::Left | KeyCode::Right | KeyCode::Char(' ') => {
            if state.selected_field == MenuField::Language {
                let items: Vec<String> =
                    get_available_languages().into_iter().map(String::from).collect();
                state.picker = Some(PickerKind::Language(PickerState::new(
                    items,
                    config.language.clone(),
                )));
                return MenuAction::None;
            }
            if state.selected_field == MenuField::Theme {
                state.theme_preview_base = Some(config.theme.clone());
                let items: Vec<String> =
                    get_theme_names().into_iter().map(String::from).collect();
                state.picker = Some(PickerKind::Theme(PickerState::new(
                    items,
                    config.theme.clone(),
                )));
                return MenuAction::None;
            }
            let forward = key.code == KeyCode::Right;
            let mut new_config = config.clone();
            match state.selected_field {
                MenuField::Mode => {
                    new_config.mode = match config.mode {
                        TestMode::Time => TestMode::Words,
                        TestMode::Words => TestMode::Time,
                    };
                }
                MenuField::Time => {
                    let idx = TIME_OPTIONS
                        .iter()
                        .position(|&t| t == config.time_limit)
                        .unwrap_or(0);
                    let next = if forward {
                        (idx + 1) % TIME_OPTIONS.len()
                    } else {
                        (idx + TIME_OPTIONS.len() - 1) % TIME_OPTIONS.len()
                    };
                    new_config.time_limit = TIME_OPTIONS[next];
                }
                MenuField::Words => {
                    let idx = WORD_OPTIONS
                        .iter()
                        .position(|&w| w == config.word_count)
                        .unwrap_or(0);
                    let next = if forward {
                        (idx + 1) % WORD_OPTIONS.len()
                    } else {
                        (idx + WORD_OPTIONS.len() - 1) % WORD_OPTIONS.len()
                    };
                    new_config.word_count = WORD_OPTIONS[next];
                }
                MenuField::OneLine => new_config.one_line = !config.one_line,
                MenuField::Punctuation => new_config.punctuation = !config.punctuation,
                MenuField::Numbers => new_config.numbers = !config.numbers,
                MenuField::Backspace => new_config.backspace = !config.backspace,
                _ => {}
            }
            return MenuAction::UpdateConfig(new_config);
        }
        _ => {}
    }
    MenuAction::None
}

pub fn render(frame: &mut Frame, area: Rect, app: &App, theme: &ThemeColors) {
    let accent = parse_hex(&theme.accent);
    let text_color = parse_hex(&theme.text);
    let dim = parse_hex(&theme.text_dim);
    let stats_color = parse_hex(&theme.stats);

    // Layout: header, menu options, footer
    let chunks = Layout::vertical([
        Constraint::Length(HEADER_HEIGHT),
        Constraint::Min(10),
        Constraint::Length(2),
    ])
    .split(area);

    // Header
    frame.render_widget(Header { theme }, chunks[0]);

    // If picker is open, render picker
    if let Some(ref picker_kind) = app.menu_state.picker {
        let picker_area = centered_rect(40, 20, chunks[1]);
        match picker_kind {
            PickerKind::Language(ps) => {
                frame.render_widget(Picker { state: ps, theme }, picker_area);
            }
            PickerKind::Theme(ps) => {
                frame.render_widget(Picker { state: ps, theme }, picker_area);
            }
        }
        return;
    }

    // Menu options
    let fields = get_fields(&app.config);
    let menu_area = centered_rect(50, fields.len() as u16, chunks[1]);

    for (i, field) in fields.iter().enumerate() {
        if i as u16 >= menu_area.height {
            break;
        }
        let is_selected = *field == app.menu_state.selected_field;

        let (label, value) = match field {
            MenuField::Mode => ("mode", format!("{}", match app.config.mode {
                TestMode::Time => "time",
                TestMode::Words => "words",
            })),
            MenuField::Time => ("time", format!("{}s", app.config.time_limit)),
            MenuField::Words => ("words", format!("{}", app.config.word_count)),
            MenuField::Language => (
                "language",
                format!("{}  ▸", app.config.language.replace('_', " ")),
            ),
            MenuField::Theme => (
                "theme",
                format!("{}  ▸", app.config.theme.replace('_', " ")),
            ),
            MenuField::OneLine => (
                "one liner",
                if app.config.one_line { "on" } else { "off" }.into(),
            ),
            MenuField::Punctuation => (
                "punctuation",
                if app.config.punctuation { "on" } else { "off" }.into(),
            ),
            MenuField::Numbers => (
                "numbers",
                if app.config.numbers { "on" } else { "off" }.into(),
            ),
            MenuField::Backspace => (
                "backspace",
                if app.config.backspace { "on" } else { "off" }.into(),
            ),
        };

        let prefix = if is_selected { " > " } else { "   " };
        let prefix_style =
            Style::default().fg(if is_selected { accent } else { dim });
        let label_style = if is_selected {
            Style::default().fg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(dim)
        };
        let value_style =
            Style::default().fg(if is_selected { text_color } else { stats_color });

        let line = Line::from(vec![
            Span::styled(prefix, prefix_style),
            Span::styled(format!("{label}: "), label_style),
            Span::styled(value, value_style),
        ]);

        let y = menu_area.y + i as u16;
        frame.render_widget(line, Rect::new(menu_area.x, y, menu_area.width, 1));
    }

    // Footer
    let footer = Line::from(Span::styled(
        "arrows/space: change  enter: start  s: scores",
        Style::default().fg(dim),
    ));
    let footer_width = footer.width() as u16;
    let fx = area.x + area.width.saturating_sub(footer_width) / 2;
    frame.render_widget(
        footer,
        Rect::new(fx, chunks[2].y, chunks[2].width, 1),
    );
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(
        x,
        y,
        width.min(area.width),
        height.min(area.height),
    )
}
