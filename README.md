# BuddyType

A [MonkeyType](https://monkeytype.com) clone that runs entirely in your terminal.

<p align="center">
  <img src="assets/screenshot.png" alt="BuddyType menu screen" width="700">
</p>

## Quick start

```bash
npm install -g buddytype
buddytype
```

Requires **Node.js 20+**.

### From source

```bash
git clone https://github.com/DanielD2G/BuddyTypeCLI.git
cd BuddyTypeCLI
npm install
npm run build
npm link
buddytype
```

## Features

- **Two modes** - Time-based (15 / 30 / 60 / 120 seconds) or word count (10 / 25 / 50 / 100 words)
- **11 languages** - English, Spanish, French, German, Italian, Portuguese, and code syntax for JavaScript, TypeScript, Python, Rust, Go
- **50+ themes** - Built-in dark & light plus the full MonkeyType community theme collection
- **Live metrics** - WPM, raw WPM, accuracy, and timer update on every keystroke
- **Consistency score** - Uses MonkeyType's kogasa function to measure typing steadiness
- **Score history** - Persists your last 100 results locally for review
- **Punctuation & numbers** - Toggle extra difficulty on any language
- **Full backspace support** - Delete characters, clear words with Ctrl+Backspace, or go back to previous words

## Controls

### During a test

| Key | Action |
| --- | --- |
| *any character* | Start typing (test begins on first key) |
| `Space` | Submit current word, advance to next |
| `Backspace` | Delete last character or return to previous word |
| `Ctrl+Backspace` | Clear entire current word |
| `Tab` | Restart the test |
| `Esc` | Return to menu |

### Menu

| Key | Action |
| --- | --- |
| `Up` / `k` | Move up |
| `Down` / `j` | Move down |
| `Left` / `Right` | Cycle option values |
| `Enter` | Start test with current settings |
| `s` | View score history |

## How WPM is calculated

All formulas match [MonkeyType](https://monkeytype.com):

| Metric | Formula |
| --- | --- |
| **WPM** (net) | `(correctWordChars / 5) / minutes` — only fully correct words count |
| **Raw WPM** | `(allTypedChars / 5) / minutes` |
| **Accuracy** | `correctKeypresses / totalKeypresses * 100` |
| **Consistency** | Kogasa function: `100 * (1 - tanh(cov + cov^3/3 + cov^5/5))` where `cov` is the coefficient of variation of per-second WPM samples |

## Architecture

```
src/
├── engine/         Pure TypeScript — zero React/Ink imports
│   ├── timer.ts            Timer state machine
│   ├── word-generator.ts   Power-law word sampling + punctuation/numbers
│   ├── input-processor.ts  Keystroke handling & character tracking
│   └── stats-calculator.ts WPM, accuracy, consistency calculations
├── hooks/          React hooks bridging engine → component state
├── components/     Reusable UI pieces
├── screens/        Full-screen views (menu, test, results, scores)
├── data/           Word lists (MonkeyType-compatible JSON) & themes
├── config/         Local persistence (settings + scores)
└── types/          Shared TypeScript interfaces
```

The engine layer is **completely decoupled from the UI** — every function is pure, stateless, and tested with plain vitest.

## Development

```bash
npm run dev          # Run with tsx (hot reload)
npm run build        # Production build with tsup
npm start            # Run the production build
npm test             # Run tests once
npm run test:watch   # Tests in watch mode
npm run lint         # Type-check with tsc --noEmit
npm run format       # Format with Prettier
```

## Tech stack

| Layer | Technology |
| --- | --- |
| Runtime | Node.js 20+ |
| Language | TypeScript 5 (strict mode) |
| Terminal UI | Ink 6 (React 19 for CLIs) |
| Styling | Chalk 5 |
| Build | tsup (ESM, sourcemaps) |
| Tests | Vitest |
| Persistence | conf (OS-native config path) |

## Acknowledgments

Word lists and theme definitions are sourced from [MonkeyType](https://github.com/monkeytypegame/monkeytype), licensed under GPL-3.0.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE) — the same license as MonkeyType, whose word lists and themes are included in this project.
