use serde::{Deserialize, Serialize};
use std::time::Instant;

// ── Character / Word state ──────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CharResult {
    pub correct: bool,
    pub extra: bool,
}

#[derive(Debug, Clone)]
pub struct WordState {
    pub word: String,
    pub typed: String,
    pub chars: Vec<CharResult>,
    pub completed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct StatsSnapshot {
    pub wpm: f64,
    pub raw_wpm: f64,
    pub accuracy: f64,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub extra_chars: usize,
    pub missed_chars: usize,
    #[allow(dead_code)] // used by tests
    pub elapsed_seconds: f64,
}

// ── Test configuration ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestMode {
    Time,
    Words,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub mode: TestMode,
    pub time_limit: u32,
    pub word_count: u32,
    pub language: String,
    pub theme: String,
    pub one_line: bool,
    pub punctuation: bool,
    pub numbers: bool,
    pub backspace: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            mode: TestMode::Time,
            time_limit: 30,
            word_count: 25,
            language: "english".into(),
            theme: "dark".into(),
            one_line: false,
            punctuation: false,
            numbers: false,
            backspace: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestPhase {
    Idle,
    Active,
    Finished,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub wpm: u32,
    pub raw_wpm: u32,
    pub accuracy: f64,
    pub consistency: f64,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub extra_chars: usize,
    pub missed_chars: usize,
    pub total_words: usize,
    pub correct_words: usize,
    pub elapsed_seconds: f64,
    pub config: TestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub wpm: u32,
    pub raw_wpm: u32,
    pub accuracy: f64,
    pub consistency: f64,
    pub language: String,
    pub mode: TestMode,
    pub duration: u32,
    pub date: String,
}

// ── Language ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // fields deserialized from JSON, used by tests
pub struct Language {
    pub name: String,
    #[serde(default)]
    pub ordered_by_frequency: Option<bool>,
    #[serde(default)]
    pub no_lazy_mode: Option<bool>,
    pub words: Vec<String>,
}

// ── Theme ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // name deserialized from JSON
pub struct ThemeColors {
    pub name: String,
    pub bg: String,
    pub text: String,
    pub text_dim: String,
    pub correct: String,
    pub incorrect: String,
    pub extra: String,
    pub cursor: String,
    pub accent: String,
    pub stats: String,
}

// ── Input state ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct InputState {
    pub words: Vec<WordState>,
    pub current_word_index: usize,
    pub cursor_position: usize,
    pub finished: bool,
    pub historical_error_chars: usize,
    pub keypress_correct: usize,
    pub keypress_incorrect: usize,
}

// ── Timer state ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TimerState {
    pub start_time: Option<Instant>,
    pub elapsed_ms: f64,
    pub limit_ms: Option<f64>,
    pub running: bool,
    pub expired: bool,
}

// ── Generator config ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub language: String,
    pub count: usize,
    pub punctuation: bool,
    pub numbers: bool,
}

// ── Screen enum ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    Test,
    Results,
    Scores,
}
