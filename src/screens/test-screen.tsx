import React from "react";
import { Box, Text, useInput } from "ink";
import type { TestConfig, TestResult } from "../types/test.js";
import type { ThemeColors } from "../types/theme.js";
import { useTypingTest } from "../hooks/use-typing-test.js";
import { useTerminalSize } from "../hooks/use-terminal-size.js";
import { WordDisplay } from "../components/word-display.js";
import { StatsBar } from "../components/stats-bar.js";

interface TestScreenProps {
  config: TestConfig;
  theme: ThemeColors;
  onFinish: (result: TestResult, wpmHistory: number[]) => void;
  onBack: () => void;
}

export function TestScreen({ config, theme, onFinish, onBack }: TestScreenProps) {
  const [restartPending, setRestartPending] = React.useState(false);
  const {
    phase,
    inputState,
    currentStats,
    wpmHistory,
    result,
    elapsedSeconds,
    remainingSeconds,
    handleInput,
    restart,
  } = useTypingTest(config);

  const { columns } = useTerminalSize();

  // Handle results transition
  React.useEffect(() => {
    if (phase === "finished" && result) {
      onFinish(result, wpmHistory);
    }
  }, [phase, result, onFinish]);

  useInput((input, key) => {
    if (key.escape) {
      setRestartPending(false);
      onBack();
      return;
    }

    if (key.tab) {
      setRestartPending(true);
      return;
    }

    if (restartPending) {
      if (key.return) {
        setRestartPending(false);
        restart();
      } else {
        setRestartPending(false);
      }
      return;
    }

    // Ink v6 maps macOS Delete key (0x7f) to key.delete, not key.backspace
    const isBackspace = key.backspace || key.delete;

    // Block backspace when disabled in config
    if (isBackspace && !config.backspace) return;

    handleInput(input, {
      backspace: isBackspace,
      ctrl: key.ctrl,
    });
  });

  return (
    <Box flexDirection="column" alignItems="center" width="100%">
      <StatsBar
        wpm={currentStats.wpm}
        accuracy={currentStats.accuracy}
        elapsedSeconds={elapsedSeconds}
        remainingSeconds={remainingSeconds}
        phase={phase}
        mode={config.mode}
        theme={theme}
      />
      <WordDisplay
        words={inputState.words}
        currentWordIndex={inputState.currentWordIndex}
        cursorPosition={inputState.cursorPosition}
        terminalWidth={columns}
        theme={theme}
        oneLine={config.oneLine}
      />
      <Box marginTop={1}>
        <Text color={theme.textDim}>
          {restartPending
            ? "enter: confirm restart  any key: cancel  esc: menu"
            : "tab: restart  esc: menu"}
        </Text>
      </Box>
    </Box>
  );
}
