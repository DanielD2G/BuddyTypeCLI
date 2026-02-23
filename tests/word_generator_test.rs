use buddytype::data::languages::get_language;
use buddytype::engine::word_generator::generate_words;
use buddytype::types::GeneratorConfig;

#[test]
fn loads_english_word_list() {
    let lang = get_language("english");
    assert_eq!(lang.name, "english");
    assert!(lang.words.len() >= 100);
}

#[test]
fn generates_requested_number_of_words() {
    let words = generate_words(&GeneratorConfig {
        language: "english".into(),
        count: 25,
        punctuation: false,
        numbers: false,
    });
    assert_eq!(words.len(), 25);
}

#[test]
fn all_words_come_from_word_list() {
    let lang = get_language("english");
    let words = generate_words(&GeneratorConfig {
        language: "english".into(),
        count: 50,
        punctuation: false,
        numbers: false,
    });
    for word in &words {
        assert!(
            lang.words.contains(word),
            "Word '{}' not in language list",
            word
        );
    }
}

#[test]
fn applies_punctuation_when_enabled() {
    let mut has_punctuation = false;
    for _ in 0..10 {
        let words = generate_words(&GeneratorConfig {
            language: "english".into(),
            count: 50,
            punctuation: true,
            numbers: false,
        });
        let joined = words.join(" ");
        if joined.contains('.') || joined.contains(',') {
            has_punctuation = true;
            break;
        }
    }
    assert!(has_punctuation);
}

#[test]
fn includes_numbers_when_enabled() {
    let mut has_numbers = false;
    for _ in 0..10 {
        let words = generate_words(&GeneratorConfig {
            language: "english".into(),
            count: 50,
            punctuation: false,
            numbers: true,
        });
        if words.iter().any(|w| w.chars().all(|c| c.is_ascii_digit())) {
            has_numbers = true;
            break;
        }
    }
    assert!(has_numbers);
}

#[test]
fn generates_exactly_1_word() {
    let words = generate_words(&GeneratorConfig {
        language: "english".into(),
        count: 1,
        punctuation: false,
        numbers: false,
    });
    assert_eq!(words.len(), 1);
    assert!(!words[0].is_empty());
}

#[test]
fn generates_100_words() {
    let words = generate_words(&GeneratorConfig {
        language: "english".into(),
        count: 100,
        punctuation: false,
        numbers: false,
    });
    assert_eq!(words.len(), 100);
}

#[test]
fn works_with_code_languages() {
    let words = generate_words(&GeneratorConfig {
        language: "code_javascript".into(),
        count: 20,
        punctuation: false,
        numbers: false,
    });
    assert_eq!(words.len(), 20);
    for w in &words {
        assert!(!w.is_empty());
    }
}

#[test]
fn works_with_non_english_languages() {
    let words = generate_words(&GeneratorConfig {
        language: "spanish".into(),
        count: 15,
        punctuation: false,
        numbers: false,
    });
    assert_eq!(words.len(), 15);
}

#[test]
fn punctuation_capitalizes_first_word() {
    let mut has_capital = false;
    for _ in 0..20 {
        let words = generate_words(&GeneratorConfig {
            language: "english".into(),
            count: 10,
            punctuation: true,
            numbers: false,
        });
        let first_char = words[0].chars().next().unwrap();
        if first_char.is_uppercase() {
            has_capital = true;
            break;
        }
    }
    assert!(has_capital);
}

#[test]
fn no_numbers_when_numbers_disabled() {
    let words = generate_words(&GeneratorConfig {
        language: "english".into(),
        count: 100,
        punctuation: false,
        numbers: false,
    });
    let has_numbers = words.iter().any(|w| w.chars().all(|c| c.is_ascii_digit()));
    assert!(!has_numbers);
}
