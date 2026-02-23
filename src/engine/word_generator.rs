use crate::data::languages::get_language;
use crate::types::GeneratorConfig;
use rand::Rng;

pub fn generate_words(config: &GeneratorConfig) -> Vec<String> {
    let language = get_language(&config.language);
    let word_list = &language.words;
    let mut rng = rand::rng();
    let mut result = Vec::with_capacity(config.count);

    for _ in 0..config.count {
        // Power-law distribution: favor frequent words (beginning of list)
        let r: f64 = rng.random();
        let index = (r.powf(1.5) * word_list.len() as f64).floor() as usize;
        let index = index.min(word_list.len() - 1);
        let mut word = word_list[index].clone();

        // Numbers mode: ~8% chance to replace with a number
        if config.numbers && rng.random::<f64>() < 0.08 {
            let digits = rng.random_range(1..=4);
            let max = 10_u32.pow(digits);
            word = rng.random_range(0..max).to_string();
            result.push(word);
            continue;
        }

        result.push(word);
    }

    if config.punctuation {
        apply_punctuation(&mut result, &mut rng);
    }

    result
}

fn apply_punctuation(words: &mut [String], rng: &mut impl Rng) {
    let mut sentence_start = true;

    for i in 0..words.len() {
        // Capitalize first word of sentence
        if sentence_start && !words[i].is_empty() {
            let mut chars = words[i].chars();
            if let Some(first) = chars.next() {
                words[i] = first.to_uppercase().to_string() + chars.as_str();
            }
            sentence_start = false;
        }

        // End of sentence: period ~12% chance
        if i > 0 && rng.random::<f64>() < 0.12 {
            words[i].push('.');
            sentence_start = true;
            continue;
        }

        // Comma: ~6% chance mid-sentence
        if !sentence_start && rng.random::<f64>() < 0.06 {
            words[i].push(',');
        }
    }
}
