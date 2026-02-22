import React from "react";
import { Box, Text } from "ink";
import type { ThemeColors } from "../types/theme.js";

interface ResultsChartProps {
  wpmHistory: number[];
  theme: ThemeColors;
  terminalWidth?: number;
  height?: number;
}

export function ResultsChart({
  wpmHistory,
  theme,
  terminalWidth = 50,
  height = 8,
}: ResultsChartProps) {
  const width = Math.min(terminalWidth - 10, 70);
  if (wpmHistory.length < 2) {
    return null;
  }

  const maxWpm = Math.max(...wpmHistory, 1);
  const minWpm = Math.min(...wpmHistory);

  // Resample to fit width
  const samples: number[] = [];
  for (let i = 0; i < width; i++) {
    const idx = Math.floor((i / width) * wpmHistory.length);
    samples.push(wpmHistory[idx]);
  }

  const avgWpm = wpmHistory.reduce((a, b) => a + b, 0) / wpmHistory.length;

  // Build rows from top to bottom
  const rows: string[] = [];
  const blocks = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

  for (let row = height - 1; row >= 0; row--) {
    let line = "";
    for (let col = 0; col < samples.length; col++) {
      const normalized = (samples[col] - 0) / (maxWpm - 0);
      const barHeight = normalized * height;

      if (barHeight >= row + 1) {
        line += "█";
      } else if (barHeight > row) {
        const frac = barHeight - row;
        const blockIdx = Math.min(Math.floor(frac * blocks.length), blocks.length - 1);
        line += blocks[blockIdx];
      } else {
        line += " ";
      }
    }
    rows.push(line);
  }

  return (
    <Box flexDirection="column" marginY={1}>
      <Text color={theme.textDim}>wpm over time</Text>
      <Box flexDirection="column">
        {rows.map((row, i) => (
          <Text key={i} color={theme.accent}>
            {row}
          </Text>
        ))}
      </Box>
      <Text color={theme.textDim}>
        {"─".repeat(width)}
      </Text>
      <Box gap={2}>
        <Text color={theme.textDim}>
          max: {Math.round(maxWpm)}
        </Text>
        <Text color={theme.textDim}>
          avg: {Math.round(avgWpm)}
        </Text>
        <Text color={theme.textDim}>
          min: {Math.round(minWpm)}
        </Text>
      </Box>
    </Box>
  );
}
