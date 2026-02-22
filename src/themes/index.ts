import type { ThemeColors } from "../types/theme.js";
import { monkeytypeThemes } from "../data/themes/monkeytype-themes.js";

// Built-in themes (overrides for MonkeyType defaults or custom additions)
const builtinThemes: Record<string, ThemeColors> = {
  dark: {
    name: "dark",
    bg: "#323437",
    text: "#d1d0c5",
    textDim: "#646669",
    correct: "#d1d0c5",
    incorrect: "#ca4754",
    extra: "#7e2a33",
    cursor: "#e2b714",
    accent: "#e2b714",
    stats: "#646669",
  },
  light: {
    name: "light",
    bg: "#f3f2ee",
    text: "#1f2328",
    textDim: "#6b7280",
    correct: "#1f2328",
    incorrect: "#d14343",
    extra: "#8a3232",
    cursor: "#c28e00",
    accent: "#0f766e",
    stats: "#6b7280",
  },
};

// Merge: built-in themes take priority over MonkeyType themes
const themes: Record<string, ThemeColors> = {
  ...monkeytypeThemes,
  ...builtinThemes,
};

export function getTheme(name: string): ThemeColors {
  return themes[name] ?? themes["dark"];
}

export function getThemeNames(): string[] {
  return Object.keys(themes).sort();
}
