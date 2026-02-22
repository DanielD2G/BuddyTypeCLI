import { getLanguage } from "../data/languages/index.js";

export interface GeneratorConfig {
  language: string;
  count: number;
  punctuation: boolean;
  numbers: boolean;
}

export function generateWords(config: GeneratorConfig): string[] {
  const language = getLanguage(config.language);
  const { words: wordList } = language;
  const result: string[] = [];

  for (let i = 0; i < config.count; i++) {
    // Power-law distribution: favor frequent words (beginning of list)
    const index = Math.floor(Math.random() ** 1.5 * wordList.length);
    let word = wordList[index];

    // Numbers mode: ~8% chance to replace with a number
    if (config.numbers && Math.random() < 0.08) {
      const digits = Math.floor(Math.random() * 4) + 1;
      word = String(Math.floor(Math.random() * 10 ** digits));
      result.push(word);
      continue;
    }

    result.push(word);
  }

  if (config.punctuation) {
    return applyPunctuation(result);
  }

  return result;
}

function applyPunctuation(words: string[]): string[] {
  const result = [...words];
  let sentenceStart = true;

  for (let i = 0; i < result.length; i++) {
    // Capitalize first word of sentence
    if (sentenceStart && result[i].length > 0) {
      result[i] = result[i][0].toUpperCase() + result[i].slice(1);
      sentenceStart = false;
    }

    // End of sentence: period every 5-12 words
    if (i > 0 && Math.random() < 0.12) {
      result[i] = result[i] + ".";
      sentenceStart = true;
      continue;
    }

    // Comma: ~6% chance mid-sentence
    if (!sentenceStart && Math.random() < 0.06) {
      result[i] = result[i] + ",";
    }
  }

  return result;
}
