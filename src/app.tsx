import React, { useState, useCallback } from "react";
import { Box } from "ink";
import type { TestConfig, TestResult } from "./types/test.js";
import { DEFAULT_CONFIG } from "./types/test.js";
import { getTheme } from "./themes/index.js";
import { loadSettings, saveSettings, saveScore } from "./config/store.js";
import { MenuScreen } from "./screens/menu-screen.js";
import { TestScreen } from "./screens/test-screen.js";
import { ResultsScreen } from "./screens/results-screen.js";
import { ScoresScreen } from "./screens/scores-screen.js";

type Screen = "menu" | "test" | "results" | "scores";

function getInitialConfig(): TestConfig {
  const saved = loadSettings();
  return { ...DEFAULT_CONFIG, ...saved };
}

export function App() {
  const [screen, setScreen] = useState<Screen>("menu");
  const [config, setConfig] = useState<TestConfig>(getInitialConfig);
  const [result, setResult] = useState<TestResult | null>(null);
  const [wpmHistory, setWpmHistory] = useState<number[]>([]);
  const [testKey, setTestKey] = useState(0);

  const theme = getTheme(config.theme);

  const handleStart = useCallback((newConfig: TestConfig) => {
    saveSettings(newConfig);
    setConfig(newConfig);
    setTestKey((k) => k + 1);
    setScreen("test");
  }, []);

  const handleFinish = useCallback(
    (testResult: TestResult, history: number[]) => {
      saveScore(testResult);
      setResult(testResult);
      setWpmHistory(history);
      setScreen("results");
    },
    [],
  );

  const handleBack = useCallback(() => {
    setScreen("menu");
  }, []);

  const handleRestart = useCallback(() => {
    setTestKey((k) => k + 1);
    setScreen("test");
  }, []);

  const handleScores = useCallback(() => {
    setScreen("scores");
  }, []);

  return (
    <Box flexDirection="column" alignItems="center" justifyContent="center" paddingY={1} minHeight={20}>
      {screen === "menu" && (
        <MenuScreen config={config} theme={theme} onStart={handleStart} onScores={handleScores} />
      )}
      {screen === "test" && (
        <TestScreen
          key={testKey}
          config={config}
          theme={theme}
          onFinish={handleFinish}
          onBack={handleBack}
        />
      )}
      {screen === "results" && result && (
        <ResultsScreen
          result={result}
          wpmHistory={wpmHistory}
          theme={theme}
          onRestart={handleRestart}
          onBack={handleBack}
        />
      )}
      {screen === "scores" && (
        <ScoresScreen theme={theme} onBack={handleBack} />
      )}
    </Box>
  );
}
