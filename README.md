# zed-nomo

[Zed](https://zed.dev) extension for the [Nomo](https://github.com/nomo-lang)
programming language.

## Features

- Diagnostics and completion via the [`nomo-lsp`](https://github.com/nomo-lang/nomo-lsp) language server
- Syntax highlighting via [`tree-sitter-nomo`](https://github.com/nomo-lang/tree-sitter-nomo)

## Requirements

Build and install the language server so it is on your `PATH`:

```bash
cd ../nomo-lsp
cargo install --path .
```

## Install as a dev extension

1. Push `tree-sitter-nomo` and set the real commit SHA in `extension.toml`
   under `[grammars.nomo].rev`.
2. In Zed: `zed: install dev extension` and select this directory.

## Development

```bash
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --release
```
