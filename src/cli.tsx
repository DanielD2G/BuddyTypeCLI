import React from "react";
import { render } from "ink";
import { App } from "./app.js";

// Hide cursor for cleaner look
const showCursor = () => {
  process.stdout.write("\x1B[?25h");
};
const hideCursor = () => {
  process.stdout.write("\x1B[?25l");
};

hideCursor();

// Restore cursor on exit
process.on("exit", showCursor);
process.on("SIGINT", () => {
  showCursor();
  process.exit(0);
});
process.on("SIGTERM", () => {
  showCursor();
  process.exit(0);
});

// Clear screen
process.stdout.write("\x1B[2J\x1B[H");

render(<App />);
