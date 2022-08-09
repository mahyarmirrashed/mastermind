# Mastermind

Mastermind is a code-breaking game between a codemaker and a codebreaker implemented in the Rust programming language meant as a simple TUI game.

## How to Build

To build the project for debug mode, `cargo run` should suffice. The debug binary (with symbols) will be located at `target/debug/mastermind`.

For production-grade builds, `cargo run --release` should suffice.

If you want to error check the code with Clippy, a Rust linter, running `cargo clippy -- -W clippy::pedantic` will provide great feedback.

## Contribute

Contributions are always welcome! Please see the `ROADMAP.md` or GitHub issues for current features being worked upon. This codebase uses Conventional Commit syntax using Commitizen for easy Changelog generation.
