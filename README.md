# epubreader-cli

A terminal-based EPUB reader built with [ratatui](https://github.com/ratatui-org/ratatui).

![epub-reader screenshot](example.png)
![epub-reader screenshot (English)](example2.png)

Left panel shows the table of contents (`t` to toggle); right panel shows the chapter content with a scrollbar. The status bar at the bottom shows available keybindings.

## Features

- Read EPUB files in the terminal
- Chapter navigation with table of contents
- Keyboard-driven, vim-style controls

## Usage

```
epub-reader <file.epub>
```

## Keybindings

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `Space` / `PageDown` | Scroll down 20 lines |
| `PageUp` | Scroll up 20 lines |
| `Home` | Go to top |
| `End` | Go to bottom |
| `n` / `→` | Next chapter |
| `p` / `←` | Previous chapter |
| `t` | Toggle table of contents |
| `Enter` (in TOC) | Jump to selected chapter |

## Installation

```bash
cargo install epubreader-cli
```

This builds in release mode and installs the binary to `~/.cargo/bin/`, which is already on your PATH if you have Rust installed. Works on Linux, macOS, and Windows.

To uninstall:

```bash
cargo uninstall epubreader-cli
```

## Dependencies

- [epub](https://crates.io/crates/epub) — EPUB parsing
- [html2text](https://crates.io/crates/html2text) — HTML to plain text conversion
- [ratatui](https://crates.io/crates/ratatui) — terminal UI
- [crossterm](https://crates.io/crates/crossterm) — cross-platform terminal control
