import React from "react";
import { Box, Text } from "ink";
import type { ThemeColors } from "../types/theme.js";

interface HeaderProps {
  theme: ThemeColors;
}

export function Header({ theme }: HeaderProps) {
  return (
    <Box flexDirection="column" alignItems="center" marginBottom={1}>
      <Text bold color={theme.accent}>
        {"  ___           _     _      _____                 "}
      </Text>
      <Text bold color={theme.accent}>
        {" | _ ) _  _  __| | __| |_  _|_   _|  _ _ __  ___  "}
      </Text>
      <Text bold color={theme.accent}>
        {" | _ \\| || |/ _` |/ _` | || | | | | | | | '_ \\/ -_)"}
      </Text>
      <Text bold color={theme.accent}>
        {" |___/ \\_,_|\\__,_|\\__,_|\\_, | |_|  \\_, | .__/\\___|"}
      </Text>
      <Text bold color={theme.accent}>
        {"                       |__/   |__/|_|           "}
      </Text>
    </Box>
  );
}
