import React from "react";
import { Box, Text, useInput } from "ink";
import type { TestResult } from "../types/test.js";
import type { ThemeColors } from "../types/theme.js";
import { ResultsChart } from "../components/results-chart.js";
import { useTerminalSize } from "../hooks/use-terminal-size.js";

interface ResultsScreenProps {
  result: TestResult;
  wpmHistory: number[];
  theme: ThemeColors;
  onRestart: () => void;
  onBack: () => void;
}

export function ResultsScreen({
  result,
  wpmHistory,
  theme,
  onRestart,
  onBack,
}: ResultsScreenProps) {
  const { columns } = useTerminalSize();
  useInput((_input, key) => {
    if (key.tab) {
      onRestart();
      return;
    }
    if (key.escape) {
      onBack();
    }
  });

  return (
    <Box flexDirection="column" alignItems="center" paddingX={2}>
      {/* Main WPM */}
      <Box marginBottom={1}>
        <Text bold color={theme.accent} dimColor={false}>
          {result.wpm}
        </Text>
        <Text color={theme.stats}> wpm</Text>
      </Box>

      {/* Stats grid */}
      <Box gap={4} marginBottom={1}>
        <Box flexDirection="column" alignItems="center">
          <Text bold color={theme.text}>
            {result.rawWpm}
          </Text>
          <Text color={theme.textDim}>raw</Text>
        </Box>
        <Box flexDirection="column" alignItems="center">
          <Text bold color={theme.text}>
            {result.accuracy.toFixed(1)}%
          </Text>
          <Text color={theme.textDim}>accuracy</Text>
        </Box>
        <Box flexDirection="column" alignItems="center">
          <Text bold color={theme.text}>
            {result.consistency.toFixed(1)}%
          </Text>
          <Text color={theme.textDim}>consistency</Text>
        </Box>
        <Box flexDirection="column" alignItems="center">
          <Text bold color={theme.text}>
            {result.elapsedSeconds.toFixed(1)}s
          </Text>
          <Text color={theme.textDim}>time</Text>
        </Box>
      </Box>

      {/* Character breakdown */}
      <Box gap={3} marginBottom={1}>
        <Box>
          <Text color={theme.correct}>{result.correctChars}</Text>
          <Text color={theme.textDim}> correct</Text>
        </Box>
        <Box>
          <Text color={theme.incorrect}>{result.incorrectChars}</Text>
          <Text color={theme.textDim}> incorrect</Text>
        </Box>
        <Box>
          <Text color={theme.extra}>{result.extraChars}</Text>
          <Text color={theme.textDim}> extra</Text>
        </Box>
        <Box>
          <Text color={theme.stats}>{result.missedChars}</Text>
          <Text color={theme.textDim}> missed</Text>
        </Box>
      </Box>

      {/* Words */}
      <Box marginBottom={1}>
        <Text color={theme.text}>
          {result.correctWords}/{result.totalWords}
        </Text>
        <Text color={theme.textDim}> words correct</Text>
      </Box>

      {/* WPM Chart */}
      <ResultsChart wpmHistory={wpmHistory} theme={theme} terminalWidth={columns} />

      {/* Navigation */}
      <Box marginTop={1}>
        <Text color={theme.textDim}>
          tab: restart  esc: menu
        </Text>
      </Box>
    </Box>
  );
}
