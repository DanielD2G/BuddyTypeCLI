use crate::types::ThemeColors;
use ratatui::style::Color;
use std::collections::HashMap;
use std::sync::LazyLock;

static THEMES: LazyLock<HashMap<String, ThemeColors>> = LazyLock::new(|| {
    let json = include_str!("../../data/themes/monkeytype_themes.json");
    let mut themes: HashMap<String, ThemeColors> =
        serde_json::from_str(json).expect("Failed to parse themes JSON");

    // Built-in overrides
    themes.insert(
        "dark".into(),
        ThemeColors {
            name: "dark".into(),
            bg: "#323437".into(),
            text: "#d1d0c5".into(),
            text_dim: "#646669".into(),
            correct: "#d1d0c5".into(),
            incorrect: "#ca4754".into(),
            extra: "#7e2a33".into(),
            cursor: "#e2b714".into(),
            accent: "#e2b714".into(),
            stats: "#646669".into(),
        },
    );
    themes.insert(
        "light".into(),
        ThemeColors {
            name: "light".into(),
            bg: "#f3f2ee".into(),
            text: "#1f2328".into(),
            text_dim: "#6b7280".into(),
            correct: "#1f2328".into(),
            incorrect: "#d14343".into(),
            extra: "#8a3232".into(),
            cursor: "#c28e00".into(),
            accent: "#0f766e".into(),
            stats: "#6b7280".into(),
        },
    );

    themes
});

pub fn get_theme(name: &str) -> &'static ThemeColors {
    THEMES.get(name).unwrap_or_else(|| &THEMES["dark"])
}

pub fn get_theme_names() -> Vec<&'static str> {
    let mut names: Vec<&str> = THEMES.keys().map(String::as_str).collect();
    names.sort();
    names
}

pub fn parse_hex(hex: &str) -> Color {
    let h = hex.trim_start_matches('#');
    if h.len() < 6 {
        return Color::White;
    }
    Color::Rgb(
        u8::from_str_radix(&h[0..2], 16).unwrap_or(0),
        u8::from_str_radix(&h[2..4], 16).unwrap_or(0),
        u8::from_str_radix(&h[4..6], 16).unwrap_or(0),
    )
}
