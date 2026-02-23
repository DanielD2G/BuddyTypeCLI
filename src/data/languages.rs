use crate::types::Language;
use std::collections::HashMap;
use std::sync::LazyLock;

static LANGUAGES: LazyLock<HashMap<String, Language>> = LazyLock::new(|| {
    let files: &[(&str, &str)] = &[
        ("code_go", include_str!("../../data/languages/code_go.json")),
        (
            "code_javascript",
            include_str!("../../data/languages/code_javascript.json"),
        ),
        (
            "code_python",
            include_str!("../../data/languages/code_python.json"),
        ),
        (
            "code_rust",
            include_str!("../../data/languages/code_rust.json"),
        ),
        (
            "code_typescript",
            include_str!("../../data/languages/code_typescript.json"),
        ),
        ("english", include_str!("../../data/languages/english.json")),
        ("french", include_str!("../../data/languages/french.json")),
        ("german", include_str!("../../data/languages/german.json")),
        ("italian", include_str!("../../data/languages/italian.json")),
        (
            "portuguese",
            include_str!("../../data/languages/portuguese.json"),
        ),
        ("spanish", include_str!("../../data/languages/spanish.json")),
    ];

    let mut map = HashMap::new();
    for (name, json) in files {
        let lang: Language =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("Failed to parse {name}: {e}"));
        map.insert(name.to_string(), lang);
    }
    map
});

pub fn get_language(name: &str) -> &'static Language {
    LANGUAGES
        .get(name)
        .unwrap_or_else(|| panic!("Unknown language: {name}"))
}

pub fn get_available_languages() -> Vec<&'static str> {
    let mut names: Vec<&str> = LANGUAGES.keys().map(String::as_str).collect();
    names.sort();
    names
}
