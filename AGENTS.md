# AGENTS.md

Guidance for AI coding agents (and humans working with agents) in this repository.

## Project overview

fvcommand is a voice-command tool for Windows written in Rust. It is planned to
capture microphone audio with `cpal`, detect a wake word with `rustpotter`,
transcribe spoken commands with `transcribe-rs` (ONNX backend), and act on them,
using `tokio` as the async runtime and `toml` for configuration. As of now the
implementation is just the `cargo` starter skeleton in `src/main.rs` — the
dependencies above describe the intended feature set, not yet-written code.

## Build & run

- Rust edition 2024 — requires a recent stable toolchain (`rustup update stable`).
- `cargo build` — compile
- `cargo run` — run the binary
- `cargo test` — run tests
- `cargo clippy` — lints
- `cargo fmt` — format (rustfmt, edition 2024)

Note: `transcribe-rs` with the `onnx` feature may pull in ONNX Runtime native
binaries; the first build can be slow and may download/link native artifacts.

## Code style & conventions

Derived from the current `src/main.rs` (which is still minimal):

- Standard Rust naming: `snake_case` functions, default `rustfmt` formatting.
- Keep code idiomatic Rust 2024; run `cargo fmt` and `cargo clippy` before
  finishing work.
- No custom error-handling framework exists yet — use plain `Result`/`?` as the
  code grows, and let `clippy` guide choices.
- Async work is expected to run on `tokio` (multi-thread runtime, macros,
  sync/time/signal features enabled in `Cargo.toml`).

## Notes for agents

- **Single-file layout**: all source lives in `src/main.rs`. Splitting into
  modules (e.g. `config.rs`, `audio.rs`, `wake.rs`, `transcribe.rs`) may be
  appropriate as the program grows; don't be surprised by the small file count,
  and don't assume structure that isn't there yet.
- **tokio**: the runtime is a dependency with `rt-multi-thread` and `macros`
  features; expect `#[tokio::main]` and async entry points.
- **Hardware dependency**: microphone/audio capture via `cpal` may fail in
  sandboxes, CI, or VMs with no audio devices. Prefer design that separates
  audio capture from logic so the logic is testable without a mic.
- **Config**: configuration is read from `toml` files (`toml` crate). Keep
  config parsing separate from side effects.
- There is no CI or README yet; `CONTRIBUTING.md` and this file are the only docs.

## Branch & PR workflow

Feature/issue work happens on short-lived branches named
`<did>-<thing>-<type>`, where `<did>` is the work done, in past tense
(`fixed`, `implemented`, `updated`), `<thing>` is a short kebab-case topic,
and `<type>` is `issue` or `feature` (e.g. `fixed-mic-latency-issue`).
Branch from `dev`; branches merge into `dev`,
and `dev` merges into `main`. See CONTRIBUTING.md for the full workflow.
