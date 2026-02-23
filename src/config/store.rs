use crate::types::{ScoreEntry, TestConfig, TestResult};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    ProjectDirs::from("", "", "buddytype")
        .map(|d| d.config_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn settings_path() -> PathBuf {
    config_dir().join("settings.json")
}

fn scores_path() -> PathBuf {
    config_dir().join("scores.json")
}

pub fn load_settings() -> TestConfig {
    let path = settings_path();
    match fs::read_to_string(&path) {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => TestConfig::default(),
    }
}

pub fn save_settings(config: &TestConfig) {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(&path, json);
    }
}

pub fn save_score(result: &TestResult) {
    let entry = ScoreEntry {
        wpm: result.wpm,
        raw_wpm: result.raw_wpm,
        accuracy: result.accuracy,
        consistency: result.consistency,
        language: result.config.language.clone(),
        mode: result.config.mode,
        duration: match result.config.mode {
            crate::types::TestMode::Time => result.config.time_limit,
            crate::types::TestMode::Words => result.config.word_count,
        },
        date: chrono::Local::now().to_rfc3339(),
    };

    let mut scores = get_scores();
    scores.insert(0, entry);
    scores.truncate(100);

    let path = scores_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(&scores) {
        let _ = fs::write(&path, json);
    }
}

pub fn get_scores() -> Vec<ScoreEntry> {
    let path = scores_path();
    match fs::read_to_string(&path) {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
