import type { Language } from "../../types/language.js";
import code_go from "./code_go.json" with { type: "json" };
import code_javascript from "./code_javascript.json" with { type: "json" };
import code_python from "./code_python.json" with { type: "json" };
import code_rust from "./code_rust.json" with { type: "json" };
import code_typescript from "./code_typescript.json" with { type: "json" };
import english from "./english.json" with { type: "json" };
import french from "./french.json" with { type: "json" };
import german from "./german.json" with { type: "json" };
import italian from "./italian.json" with { type: "json" };
import portuguese from "./portuguese.json" with { type: "json" };
import spanish from "./spanish.json" with { type: "json" };

const languages: Record<string, Language> = {
  "code_go": code_go as Language,
  "code_javascript": code_javascript as Language,
  "code_python": code_python as Language,
  "code_rust": code_rust as Language,
  "code_typescript": code_typescript as Language,
  "english": english as Language,
  "french": french as Language,
  "german": german as Language,
  "italian": italian as Language,
  "portuguese": portuguese as Language,
  "spanish": spanish as Language,
};

export function getLanguage(name: string): Language {
  const lang = languages[name];
  if (!lang) {
    throw new Error(
      `Unknown language: ${name}. Run ./scripts/sync-languages.sh to update.`,
    );
  }
  return lang;
}

export function getAvailableLanguages(): string[] {
  return Object.keys(languages).sort();
}
