import React, { useState } from "react";
import { Box, Text, useInput } from "ink";
import type { ThemeColors } from "../types/theme.js";
import type { ScoreEntry } from "../types/test.js";
import { getScores } from "../config/store.js";
import { useTerminalSize } from "../hooks/use-terminal-size.js";

interface ScoresScreenProps {
  theme: ThemeColors;
  onBack: () => void;
}

export function ScoresScreen({ theme, onBack }: ScoresScreenProps) {
  const scores = getScores();
  const { rows } = useTerminalSize();
  const [scrollOffset, setScrollOffset] = useState(0);

  // Find best WPM for highlighting
  const bestWpm = scores.length > 0 ? Math.max(...scores.map((s) => s.wpm)) : 0;

  const visibleRows = Math.max(rows - 8, 5);

  useInput((_input, key) => {
    if (key.escape) {
      onBack();
      return;
    }
    if (key.upArrow) {
      setScrollOffset((o) => Math.max(0, o - 1));
    }
    if (key.downArrow) {
      setScrollOffset((o) => Math.min(Math.max(0, scores.length - visibleRows), o + 1));
    }
  });

  if (scores.length === 0) {
    return (
      <Box flexDirection="column" alignItems="center" paddingY={1}>
        <Text bold color={theme.accent}>
          scores
        </Text>
        <Box marginY={2}>
          <Text color={theme.textDim}>no scores yet — complete a test first</Text>
        </Box>
        <Text color={theme.textDim}>esc: back to menu</Text>
      </Box>
    );
  }

  const visible = scores.slice(scrollOffset, scrollOffset + visibleRows);

  function formatDate(iso: string): string {
    const d = new Date(iso);
    return d.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  return (
    <Box flexDirection="column" alignItems="center" paddingY={1}>
      <Text bold color={theme.accent}>
        scores
      </Text>
      <Box marginY={1} flexDirection="column">
        {/* Header */}
        <Box gap={1}>
          <Text color={theme.textDim}>{pad("#", 4)}</Text>
          <Text color={theme.textDim}>{pad("WPM", 6)}</Text>
          <Text color={theme.textDim}>{pad("Raw", 6)}</Text>
          <Text color={theme.textDim}>{pad("Acc", 7)}</Text>
          <Text color={theme.textDim}>{pad("Lang", 12)}</Text>
          <Text color={theme.textDim}>{pad("Mode", 10)}</Text>
          <Text color={theme.textDim}>Date</Text>
        </Box>
        <Text color={theme.textDim}>{"─".repeat(65)}</Text>
        {/* Rows */}
        {visible.map((score, i) => {
          const idx = scrollOffset + i + 1;
          const isBest = score.wpm === bestWpm;
          const wpmColor = isBest ? theme.accent : theme.text;

          return (
            <Box key={scrollOffset + i} gap={1}>
              <Text color={theme.textDim}>{pad(String(idx), 4)}</Text>
              <Text bold color={wpmColor}>
                {pad(String(score.wpm), 6)}
              </Text>
              <Text color={theme.text}>{pad(String(score.rawWpm), 6)}</Text>
              <Text color={theme.text}>{pad(`${score.accuracy.toFixed(1)}%`, 7)}</Text>
              <Text color={theme.text}>{pad(score.language.replace(/_/g, " "), 12)}</Text>
              <Text color={theme.text}>
                {pad(
                  `${score.mode} ${score.duration}${score.mode === "time" ? "s" : "w"}`,
                  10,
                )}
              </Text>
              <Text color={theme.textDim}>{formatDate(score.date)}</Text>
            </Box>
          );
        })}
      </Box>
      {scores.length > visibleRows && (
        <Text color={theme.textDim}>
          showing {scrollOffset + 1}-{Math.min(scrollOffset + visibleRows, scores.length)} of{" "}
          {scores.length} — arrows to scroll
        </Text>
      )}
      <Box marginTop={1}>
        <Text color={theme.textDim}>esc: back to menu</Text>
      </Box>
    </Box>
  );
}

function pad(str: string, len: number): string {
  return str.padEnd(len);
}
