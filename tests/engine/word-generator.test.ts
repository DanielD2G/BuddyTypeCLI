import { describe, it, expect } from "vitest";
import { generateWords } from "../../src/engine/word-generator.js";
import { getLanguage } from "../../src/data/languages/index.js";

describe("getLanguage", () => {
  it("loads the english word list", () => {
    const lang = getLanguage("english");
    expect(lang.name).toBe("english");
    expect(lang.bcp47).toBe("en");
    expect(lang.orderedByFrequency).toBe(true);
    expect(lang.words.length).toBeGreaterThanOrEqual(100);
  });
});

describe("generateWords", () => {
  it("generates the requested number of words", () => {
    const words = generateWords({
      language: "english",
      count: 25,
      punctuation: false,
      numbers: false,
    });
    expect(words).toHaveLength(25);
  });

  it("all words come from the word list", () => {
    const lang = getLanguage("english");
    const words = generateWords({
      language: "english",
      count: 50,
      punctuation: false,
      numbers: false,
    });
    for (const word of words) {
      expect(lang.words).toContain(word);
    }
  });

  it("applies punctuation when enabled", () => {
    let hasPunctuation = false;
    for (let attempt = 0; attempt < 10; attempt++) {
      const words = generateWords({
        language: "english",
        count: 50,
        punctuation: true,
        numbers: false,
      });
      const joined = words.join(" ");
      if (/[.,]/.test(joined)) {
        hasPunctuation = true;
        break;
      }
    }
    expect(hasPunctuation).toBe(true);
  });

  it("includes numbers when enabled", () => {
    let hasNumbers = false;
    for (let attempt = 0; attempt < 10; attempt++) {
      const words = generateWords({
        language: "english",
        count: 50,
        punctuation: false,
        numbers: true,
      });
      if (words.some((w) => /^\d+$/.test(w))) {
        hasNumbers = true;
        break;
      }
    }
    expect(hasNumbers).toBe(true);
  });

  it("generates exactly 1 word", () => {
    const words = generateWords({
      language: "english",
      count: 1,
      punctuation: false,
      numbers: false,
    });
    expect(words).toHaveLength(1);
    expect(words[0].length).toBeGreaterThan(0);
  });

  it("generates 100 words", () => {
    const words = generateWords({
      language: "english",
      count: 100,
      punctuation: false,
      numbers: false,
    });
    expect(words).toHaveLength(100);
  });

  it("works with code languages", () => {
    const words = generateWords({
      language: "code_javascript",
      count: 20,
      punctuation: false,
      numbers: false,
    });
    expect(words).toHaveLength(20);
    words.forEach((w) => expect(w.length).toBeGreaterThan(0));
  });

  it("works with non-english languages", () => {
    const words = generateWords({
      language: "spanish",
      count: 15,
      punctuation: false,
      numbers: false,
    });
    expect(words).toHaveLength(15);
  });

  it("punctuation capitalizes first word", () => {
    let hasCapital = false;
    for (let attempt = 0; attempt < 20; attempt++) {
      const words = generateWords({
        language: "english",
        count: 10,
        punctuation: true,
        numbers: false,
      });
      if (words[0][0] === words[0][0].toUpperCase() && words[0][0] !== words[0][0].toLowerCase()) {
        hasCapital = true;
        break;
      }
    }
    expect(hasCapital).toBe(true);
  });

  it("no numbers appear when numbers disabled", () => {
    const words = generateWords({
      language: "english",
      count: 100,
      punctuation: false,
      numbers: false,
    });
    const hasNumbers = words.some((w) => /^\d+$/.test(w));
    expect(hasNumbers).toBe(false);
  });
});
