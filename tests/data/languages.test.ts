import { describe, it, expect } from "vitest";
import {
  getLanguage,
  getAvailableLanguages,
} from "../../src/data/languages/index.js";

describe("getLanguage", () => {
  it("loads the english word list", () => {
    const lang = getLanguage("english");
    expect(lang.name).toBe("english");
    expect(lang.words.length).toBeGreaterThan(100);
  });

  it("throws for unknown language", () => {
    expect(() => getLanguage("klingon")).toThrow("Unknown language: klingon");
  });

  it("loads code languages", () => {
    const js = getLanguage("code_javascript");
    expect(js.name).toBe("code_javascript");
    expect(js.words.length).toBeGreaterThan(0);

    const ts = getLanguage("code_typescript");
    expect(ts.name).toBe("code_typescript");

    const py = getLanguage("code_python");
    expect(py.name).toBe("code_python");
  });

  it("all words are non-empty strings", () => {
    const lang = getLanguage("english");
    for (const word of lang.words) {
      expect(typeof word).toBe("string");
      expect(word.length).toBeGreaterThan(0);
    }
  });
});

describe("getAvailableLanguages", () => {
  it("returns an array of language names", () => {
    const langs = getAvailableLanguages();
    expect(Array.isArray(langs)).toBe(true);
    expect(langs.length).toBeGreaterThan(0);
  });

  it("includes expected languages", () => {
    const langs = getAvailableLanguages();
    expect(langs).toContain("english");
    expect(langs).toContain("spanish");
    expect(langs).toContain("french");
    expect(langs).toContain("code_javascript");
    expect(langs).toContain("code_typescript");
    expect(langs).toContain("code_python");
  });

  it("returns sorted names", () => {
    const langs = getAvailableLanguages();
    const sorted = [...langs].sort();
    expect(langs).toEqual(sorted);
  });

  it("all listed languages are loadable", () => {
    const langs = getAvailableLanguages();
    for (const name of langs) {
      const lang = getLanguage(name);
      expect(lang.words.length).toBeGreaterThan(0);
    }
  });
});
