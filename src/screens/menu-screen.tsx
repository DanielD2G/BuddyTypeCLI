import React, { useState, useCallback } from "react";
import { Box, Text, useInput } from "ink";
import type { TestConfig } from "../types/test.js";
import type { ThemeColors } from "../types/theme.js";
import { Header } from "../components/header.js";
import { LanguagePicker } from "../components/language-picker.js";
import { getAvailableLanguages } from "../data/languages/index.js";
import { getTheme, getThemeNames } from "../themes/index.js";

interface MenuScreenProps {
  config: TestConfig;
  theme: ThemeColors;
  onStart: (config: TestConfig) => void;
  onScores: () => void;
}

type MenuField =
  | "mode"
  | "time"
  | "words"
  | "language"
  | "theme"
  | "oneLine"
  | "punctuation"
  | "numbers"
  | "backspace";

const TIME_OPTIONS = [15, 30, 60, 120];
const WORD_OPTIONS = [10, 25, 50, 100];
const LANGUAGE_OPTIONS = getAvailableLanguages();
const THEME_OPTIONS = getThemeNames();

export function MenuScreen({ config, theme, onStart, onScores }: MenuScreenProps) {
  const [currentConfig, setConfig] = useState<TestConfig>(config);
  const [selectedField, setSelectedField] = useState<MenuField>("mode");
  const [pickerOpen, setPickerOpen] = useState<"language" | "theme" | null>(null);
  const [themePreviewBase, setThemePreviewBase] = useState<string | null>(null);
  const activeTheme = getTheme(currentConfig.theme);
  const handleThemeHighlight = useCallback((value: string) => {
    setConfig((c) => (c.theme === value ? c : { ...c, theme: value }));
  }, []);

  const fields: MenuField[] = [
    "mode",
    ...(currentConfig.mode === "time" ? ["time" as const] : ["words" as const]),
    "language",
    "theme",
    "oneLine",
    "punctuation",
    "numbers",
    "backspace",
  ];

  useInput(
    (input, key) => {
      if (input === "s" && !key.ctrl) {
        onScores();
        return;
      }

      if (key.return) {
        if (selectedField === "language") {
          setPickerOpen("language");
          return;
        }
        if (selectedField === "theme") {
          setThemePreviewBase(currentConfig.theme);
          setPickerOpen("theme");
          return;
        }
        onStart(currentConfig);
        return;
      }

      const currentIdx = fields.indexOf(selectedField);

      if (key.upArrow || (input === "k" && !key.ctrl)) {
        const newIdx = Math.max(0, currentIdx - 1);
        setSelectedField(fields[newIdx]);
        return;
      }

      if (key.downArrow || (input === "j" && !key.ctrl)) {
        const newIdx = Math.min(fields.length - 1, currentIdx + 1);
        setSelectedField(fields[newIdx]);
        return;
      }

      if (key.leftArrow || key.rightArrow || input === " ") {
        if (selectedField === "language") {
          setPickerOpen("language");
          return;
        }
        if (selectedField === "theme") {
          setThemePreviewBase(currentConfig.theme);
          setPickerOpen("theme");
          return;
        }
        handleToggle(key.rightArrow ?? false);
      }
    },
    { isActive: pickerOpen === null },
  );

  function handleToggle(forward: boolean) {
    switch (selectedField) {
      case "mode":
        setConfig((c) => ({
          ...c,
          mode: c.mode === "time" ? "words" : "time",
        }));
        break;
      case "time": {
        const idx = TIME_OPTIONS.indexOf(currentConfig.timeLimit);
        const next = forward
          ? (idx + 1) % TIME_OPTIONS.length
          : (idx - 1 + TIME_OPTIONS.length) % TIME_OPTIONS.length;
        setConfig((c) => ({ ...c, timeLimit: TIME_OPTIONS[next] }));
        break;
      }
      case "words": {
        const idx = WORD_OPTIONS.indexOf(currentConfig.wordCount);
        const next = forward
          ? (idx + 1) % WORD_OPTIONS.length
          : (idx - 1 + WORD_OPTIONS.length) % WORD_OPTIONS.length;
        setConfig((c) => ({ ...c, wordCount: WORD_OPTIONS[next] }));
        break;
      }
      case "punctuation":
        setConfig((c) => ({ ...c, punctuation: !c.punctuation }));
        break;
      case "numbers":
        setConfig((c) => ({ ...c, numbers: !c.numbers }));
        break;
      case "oneLine":
        setConfig((c) => ({ ...c, oneLine: !c.oneLine }));
        break;
      case "backspace":
        setConfig((c) => ({ ...c, backspace: !c.backspace }));
        break;
    }
  }

  function renderOption(field: MenuField, label: string, value: string) {
    const isSelected = selectedField === field;
    return (
      <Box key={field}>
        <Text color={isSelected ? activeTheme.accent : activeTheme.textDim}>
          {isSelected ? " ❯ " : "   "}
        </Text>
        <Text
          color={isSelected ? activeTheme.text : activeTheme.textDim}
          bold={isSelected}
        >
          {label}:{" "}
        </Text>
        <Text color={isSelected ? activeTheme.accent : activeTheme.stats}>{value}</Text>
      </Box>
    );
  }

  if (pickerOpen) {
    const isLanguagePicker = pickerOpen === "language";
    const options = isLanguagePicker ? LANGUAGE_OPTIONS : THEME_OPTIONS;
    const selectedValue = isLanguagePicker
      ? currentConfig.language
      : currentConfig.theme;

    return (
      <Box flexDirection="column" alignItems="center">
        <Header theme={activeTheme} />
        <LanguagePicker
          languages={options}
          selected={selectedValue}
          theme={activeTheme}
          onSelect={(value) => {
            if (isLanguagePicker) {
              setConfig((c) => ({ ...c, language: value }));
            } else {
              setConfig((c) => ({ ...c, theme: value }));
              setThemePreviewBase(null);
            }
            setPickerOpen(null);
          }}
          onHighlight={
            isLanguagePicker
              ? undefined
              : handleThemeHighlight
          }
          onCancel={() => {
            if (!isLanguagePicker && themePreviewBase) {
              setConfig((c) => ({ ...c, theme: themePreviewBase }));
              setThemePreviewBase(null);
            }
            setPickerOpen(null);
          }}
        />
      </Box>
    );
  }

  const languageDisplay = currentConfig.language.replace(/_/g, " ");
  const themeDisplay = currentConfig.theme.replace(/_/g, " ");

  return (
    <Box flexDirection="column" alignItems="center">
      <Header theme={activeTheme} />
      <Box flexDirection="column" marginBottom={1}>
        {renderOption("mode", "mode", currentConfig.mode)}
        {currentConfig.mode === "time"
          ? renderOption("time", "time", `${currentConfig.timeLimit}s`)
          : renderOption("words", "words", `${currentConfig.wordCount}`)}
        {renderOption("language", "language", `${languageDisplay}  ▸`)}
        {renderOption("theme", "theme", `${themeDisplay}  ▸`)}
        {renderOption("oneLine", "one liner", currentConfig.oneLine ? "on" : "off")}
        {renderOption(
          "punctuation",
          "punctuation",
          currentConfig.punctuation ? "on" : "off",
        )}
        {renderOption(
          "numbers",
          "numbers",
          currentConfig.numbers ? "on" : "off",
        )}
        {renderOption(
          "backspace",
          "backspace",
          currentConfig.backspace ? "on" : "off",
        )}
      </Box>
      <Box marginTop={1}>
        <Text color={activeTheme.textDim}>
          arrows/space: change  enter: start  s: scores
        </Text>
      </Box>
    </Box>
  );
}
