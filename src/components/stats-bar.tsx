import React from "react";
import { Box, Text } from "ink";
import type { ThemeColors } from "../types/theme.js";
import type { TestPhase } from "../types/test.js";

interface StatsBarProps {
  wpm: number;
  accuracy: number;
  elapsedSeconds: number;
  remainingSeconds: number;
  phase: TestPhase;
  mode: "time" | "words";
  theme: ThemeColors;
}

export function StatsBar({
  wpm,
  accuracy,
  elapsedSeconds,
  remainingSeconds,
  phase,
  mode,
  theme,
}: StatsBarProps) {
  if (phase === "idle") {
    return (
      <Box marginBottom={1} justifyContent="center">
        <Text color={theme.textDim}>start typing...</Text>
      </Box>
    );
  }

  const timeDisplay =
    mode === "time"
      ? `${Math.ceil(remainingSeconds)}s`
      : `${Math.round(elapsedSeconds)}s`;

  return (
    <Box marginBottom={1} gap={3}>
      <Box>
        <Text bold color={theme.accent}>
          {Math.round(wpm)}
        </Text>
        <Text color={theme.stats}> wpm</Text>
      </Box>
      <Box>
        <Text bold color={theme.accent}>
          {accuracy.toFixed(1)}%
        </Text>
        <Text color={theme.stats}> acc</Text>
      </Box>
      <Box>
        <Text bold color={theme.accent}>
          {timeDisplay}
        </Text>
      </Box>
    </Box>
  );
}
