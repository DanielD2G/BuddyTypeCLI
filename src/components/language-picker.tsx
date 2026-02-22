import React, { useState, useMemo, useEffect, useRef } from "react";
import { Box, Text, useInput } from "ink";
import type { ThemeColors } from "../types/theme.js";

interface LanguagePickerProps {
  languages: string[];
  selected: string;
  theme: ThemeColors;
  onSelect: (language: string) => void;
  onCancel: () => void;
  onHighlight?: (language: string) => void;
}

const VISIBLE_ROWS = 12;

function formatName(name: string): string {
  return name.replace(/_/g, " ");
}

export function LanguagePicker({
  languages,
  selected,
  theme,
  onSelect,
  onCancel,
  onHighlight,
}: LanguagePickerProps) {
  const [search, setSearch] = useState("");
  const lastHighlightedRef = useRef<string | null>(null);
  const [cursor, setCursor] = useState(() => {
    const idx = languages.indexOf(selected);
    return idx >= 0 ? idx : 0;
  });

  const filtered = useMemo(() => {
    if (!search) return languages;
    const q = search.toLowerCase();
    return languages.filter((l) => l.toLowerCase().includes(q));
  }, [languages, search]);

  // Keep cursor in bounds when filter changes
  const safeCursor = Math.min(cursor, Math.max(0, filtered.length - 1));

  useEffect(() => {
    if (!onHighlight || filtered.length === 0) return;
    const highlighted = filtered[safeCursor];
    if (lastHighlightedRef.current === highlighted) return;
    lastHighlightedRef.current = highlighted;
    onHighlight(highlighted);
  }, [filtered, safeCursor, onHighlight]);

  // Visible window
  const halfWindow = Math.floor(VISIBLE_ROWS / 2);
  let scrollStart = Math.max(0, safeCursor - halfWindow);
  const scrollEnd = Math.min(filtered.length, scrollStart + VISIBLE_ROWS);
  if (scrollEnd - scrollStart < VISIBLE_ROWS) {
    scrollStart = Math.max(0, scrollEnd - VISIBLE_ROWS);
  }
  const visible = filtered.slice(scrollStart, scrollEnd);

  useInput((input, key) => {
    if (key.escape) {
      onCancel();
      return;
    }

    if (key.return) {
      if (filtered.length > 0) {
        onSelect(filtered[safeCursor]);
      }
      return;
    }

    if (key.upArrow) {
      setCursor((c) => Math.max(0, c - 1));
      return;
    }

    if (key.downArrow) {
      setCursor((c) => Math.min(filtered.length - 1, c + 1));
      return;
    }

    if (key.backspace || key.delete) {
      setSearch((s) => s.slice(0, -1));
      setCursor(0);
      return;
    }

    // Typeable character → search
    if (input.length === 1 && input >= " ") {
      setSearch((s) => s + input);
      setCursor(0);
    }
  });

  const hasScroll = filtered.length > VISIBLE_ROWS;
  const atTop = scrollStart === 0;
  const atBottom = scrollEnd >= filtered.length;

  return (
    <Box flexDirection="column" borderStyle="round" borderColor={theme.accent} paddingX={1}>
      {/* Search bar */}
      <Box marginBottom={1}>
        <Text color={theme.textDim}>search: </Text>
        <Text color={theme.text}>{search}</Text>
        <Text color={theme.accent}>_</Text>
      </Box>

      {/* Scroll indicator top */}
      {hasScroll && (
        <Text color={theme.textDim}>
          {atTop ? "         " : "  ▲ more "}
        </Text>
      )}

      {/* Language list */}
      {visible.map((lang, i) => {
        const globalIdx = scrollStart + i;
        const isCursor = globalIdx === safeCursor;
        const isCurrentlySelected = lang === selected;
        const display = formatName(lang);

        return (
          <Box key={lang}>
            <Text color={isCursor ? theme.accent : theme.textDim}>
              {isCursor ? " ❯ " : "   "}
            </Text>
            <Text
              color={
                isCursor
                  ? theme.accent
                  : isCurrentlySelected
                    ? theme.correct
                    : theme.text
              }
              bold={isCursor}
            >
              {display}
            </Text>
            {isCurrentlySelected && (
              <Text color={theme.correct}> ●</Text>
            )}
          </Box>
        );
      })}

      {filtered.length === 0 && (
        <Text color={theme.incorrect}>  no matches</Text>
      )}

      {/* Scroll indicator bottom */}
      {hasScroll && (
        <Text color={theme.textDim}>
          {atBottom ? "         " : "  ▼ more "}
        </Text>
      )}

      {/* Footer */}
      <Box marginTop={1}>
        <Text color={theme.textDim}>
          type to search  enter: select  esc: cancel
        </Text>
      </Box>
    </Box>
  );
}
