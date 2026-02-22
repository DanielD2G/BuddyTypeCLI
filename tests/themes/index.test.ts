import { describe, it, expect } from "vitest";
import { getTheme, getThemeNames } from "../../src/themes/index.js";

describe("getTheme", () => {
  it("returns the dark theme", () => {
    const theme = getTheme("dark");
    expect(theme.name).toBe("dark");
    expect(theme.bg).toBe("#323437");
    expect(theme.cursor).toBe("#e2b714");
  });

  it("returns the light theme", () => {
    const theme = getTheme("light");
    expect(theme.name).toBe("light");
    expect(theme.bg).toBe("#f3f2ee");
  });

  it("falls back to dark for unknown theme", () => {
    const theme = getTheme("nonexistent-theme");
    expect(theme.name).toBe("dark");
  });

  it("returns a theme with all required properties", () => {
    const theme = getTheme("dark");
    expect(theme).toHaveProperty("name");
    expect(theme).toHaveProperty("bg");
    expect(theme).toHaveProperty("text");
    expect(theme).toHaveProperty("textDim");
    expect(theme).toHaveProperty("correct");
    expect(theme).toHaveProperty("incorrect");
    expect(theme).toHaveProperty("extra");
    expect(theme).toHaveProperty("cursor");
    expect(theme).toHaveProperty("accent");
    expect(theme).toHaveProperty("stats");
  });
});

describe("getThemeNames", () => {
  it("returns an array of theme names", () => {
    const names = getThemeNames();
    expect(Array.isArray(names)).toBe(true);
    expect(names.length).toBeGreaterThan(0);
  });

  it("includes built-in themes", () => {
    const names = getThemeNames();
    expect(names).toContain("dark");
    expect(names).toContain("light");
  });

  it("returns sorted names", () => {
    const names = getThemeNames();
    const sorted = [...names].sort();
    expect(names).toEqual(sorted);
  });
});
