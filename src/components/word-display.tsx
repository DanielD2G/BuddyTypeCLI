import React from "react";
import { Box, Text } from "ink";
import type { WordState } from "../types/stats.js";
import type { ThemeColors } from "../types/theme.js";

interface WordDisplayProps {
  words: WordState[];
  currentWordIndex: number;
  cursorPosition: number;
  terminalWidth: number;
  theme: ThemeColors;
  oneLine?: boolean;
}

interface Line {
  startIndex: number;
  endIndex: number;
}

function computeLines(words: WordState[], maxWidth: number): Line[] {
  const lines: Line[] = [];
  let lineStart = 0;
  let lineWidth = 0;

  for (let i = 0; i < words.length; i++) {
    // Use only target word length for stable line breaks — extra typed chars
    // may overflow slightly but lines won't reflow while typing
    const wordLen = words[i].word.length + 1; // +1 for space

    if (lineWidth + wordLen > maxWidth && lineWidth > 0) {
      lines.push({ startIndex: lineStart, endIndex: i });
      lineStart = i;
      lineWidth = 0;
    }
    lineWidth += wordLen;
  }

  if (lineStart < words.length) {
    lines.push({ startIndex: lineStart, endIndex: words.length });
  }

  return lines;
}

function renderChar(
  char: string,
  typed: string | undefined,
  isCursor: boolean,
  theme: ThemeColors,
): React.ReactNode {
  if (isCursor) {
    return (
      <Text inverse color={theme.cursor}>
        {char}
      </Text>
    );
  }

  if (typed === undefined) {
    // Not yet typed
    return <Text color={theme.textDim}>{char}</Text>;
  }

  if (typed === char) {
    return <Text color={theme.correct}>{char}</Text>;
  }

  return <Text color={theme.incorrect}>{char}</Text>;
}

function renderWord(
  wordState: WordState,
  isCurrentWord: boolean,
  cursorPos: number,
  theme: ThemeColors,
  key: number,
): React.ReactNode {
  const chars: React.ReactNode[] = [];
  const { word, typed } = wordState;

  // Render target characters
  for (let i = 0; i < word.length; i++) {
    const isCursor = isCurrentWord && i === cursorPos;
    const typedChar = i < typed.length ? typed[i] : undefined;
    chars.push(
      <React.Fragment key={i}>
        {renderChar(word[i], typedChar, isCursor, theme)}
      </React.Fragment>,
    );
  }

  // Render extra characters (typed beyond word length)
  for (let i = word.length; i < typed.length; i++) {
    const isCursor = isCurrentWord && i === cursorPos;
    if (isCursor) {
      chars.push(
        <Text key={`e${i}`} inverse color={theme.extra}>
          {typed[i]}
        </Text>,
      );
    } else {
      chars.push(
        <Text key={`e${i}`} color={theme.extra}>
          {typed[i]}
        </Text>,
      );
    }
  }

  const cursorAtEnd = isCurrentWord && cursorPos >= Math.max(word.length, typed.length);
  const separator = cursorAtEnd ? (
    <Text key="sep" inverse color={theme.cursor}>
      {" "}
    </Text>
  ) : (
    <Text key="sep"> </Text>
  );

  return (
    <Box key={key}>
      <Text>
        {chars}
        {separator}
      </Text>
    </Box>
  );
}

export function WordDisplay({
  words,
  currentWordIndex,
  cursorPosition,
  terminalWidth,
  theme,
  oneLine = false,
}: WordDisplayProps) {
  const maxWidth = Math.min(terminalWidth - 4, 120);
  const containerWidth = maxWidth + 4; // include horizontal padding

  if (oneLine) {
    const tape = computeTapeRange(words, currentWordIndex, cursorPosition, maxWidth);
    return (
      <Box flexDirection="column" paddingX={2} width={containerWidth}>
        <Box flexDirection="row" width={maxWidth}>
          {tape.leadingPad > 0 && (
            <Text key="pad">{" ".repeat(tape.leadingPad)}</Text>
          )}
          {words
            .slice(tape.startIndex, tape.endIndex)
            .map((wordState, wordIdx) => {
              const globalIdx = tape.startIndex + wordIdx;
              return renderWord(
                wordState,
                globalIdx === currentWordIndex,
                cursorPosition,
                theme,
                globalIdx,
              );
            })}
        </Box>
      </Box>
    );
  }

  const lines = computeLines(words, maxWidth);

  // Find which line the current word is on
  let currentLine = 0;
  for (let i = 0; i < lines.length; i++) {
    if (
      currentWordIndex >= lines[i].startIndex &&
      currentWordIndex < lines[i].endIndex
    ) {
      currentLine = i;
      break;
    }
  }

  // Always show exactly 3 lines for stable height.
  // Show the current line in the SECOND row (index 1) so the user can see
  // the previous line as context. This prevents jarring jumps when advancing
  // or backspacing across line boundaries.
  const VISIBLE_LINE_COUNT = oneLine ? 1 : 3;
  const startLine = oneLine ? currentLine : Math.max(0, currentLine - 1);

  const rows: React.ReactNode[] = [];
  for (let i = 0; i < VISIBLE_LINE_COUNT; i++) {
    const lineIdx = startLine + i;
    if (lineIdx < lines.length) {
      const line = lines[lineIdx];
      rows.push(
        <Box key={lineIdx} flexDirection="row" width={maxWidth}>
          {words
            .slice(line.startIndex, line.endIndex)
            .map((wordState, wordIdx) => {
              const globalIdx = line.startIndex + wordIdx;
              return renderWord(
                wordState,
                globalIdx === currentWordIndex,
                cursorPosition,
                theme,
                globalIdx,
              );
            })}
        </Box>,
      );
    } else {
      // Empty row to maintain stable height
      rows.push(
        <Box key={`empty-${i}`} width={maxWidth}>
          <Text> </Text>
        </Box>,
      );
    }
  }

  return (
    <Box flexDirection="column" paddingX={2} width={containerWidth}>
      {rows}
    </Box>
  );
}

interface TapeRange {
  startIndex: number;
  endIndex: number;
  /** Blank columns to prepend when there aren't enough past words to fill the left side */
  leadingPad: number;
}

/**
 * Tape-mode (MonkeyType-style): the cursor stays at a fixed anchor column
 * and the word ribbon slides left underneath it.
 *
 * anchor = column where the cursor character sits (≈ 35% of maxWidth).
 * Left of anchor:  already-typed / in-progress words.
 * Right of anchor: upcoming words.
 */
function computeTapeRange(
  words: WordState[],
  currentWordIndex: number,
  cursorPosition: number,
  maxWidth: number,
): TapeRange {
  // Anchor point — the column where the cursor of the active word sits.
  // 35% feels natural: enough past context without pushing upcoming words
  // too far right (MonkeyType defaults to ~50%, but in a terminal with
  // narrower width a slightly left-biased anchor reads better).
  const anchor = Math.floor(maxWidth * 0.35);

  // Width consumed by the current word up to (and including) the cursor column.
  // That portion sits to the LEFT of the anchor; the rest to the RIGHT.
  const currentWordLen = words[currentWordIndex]?.word.length ?? 0;
  const typedLen = words[currentWordIndex]?.typed.length ?? 0;
  const displayLen = Math.max(currentWordLen, typedLen);
  const cursorCol = Math.min(cursorPosition, displayLen);

  // --- Fill LEFT of anchor with past words ---
  let leftBudget = anchor - cursorCol; // columns available left of current word
  let startIndex = currentWordIndex;

  for (let i = currentWordIndex - 1; i >= 0 && leftBudget > 0; i--) {
    const w = words[i];
    const wLen = Math.max(w.word.length, w.typed.length) + 1; // +1 trailing space
    if (wLen > leftBudget) break;
    leftBudget -= wLen;
    startIndex = i;
  }

  const leadingPad = Math.max(0, leftBudget);

  // --- Fill RIGHT of anchor with upcoming words ---
  const rightAnchor = maxWidth - anchor + cursorCol; // columns from current word start to right edge
  let rightBudget = rightAnchor;
  // Account for the current word itself (full display length + space)
  rightBudget -= displayLen + 1;

  let endIndex = currentWordIndex + 1;
  for (let i = currentWordIndex + 1; i < words.length && rightBudget > 0; i++) {
    const wLen = words[i].word.length + 1;
    if (wLen > rightBudget) break;
    rightBudget -= wLen;
    endIndex = i + 1;
  }

  return { startIndex, endIndex, leadingPad };
}
