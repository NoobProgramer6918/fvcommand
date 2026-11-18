# AGENTS.md

Guidance for AI coding agents (and humans working with agents) in this repository.

If you are an LLM, use this file and [CONTRIBUTING.md](CONTRIBUTING.md) as guidelines/rules/tips for contributing to this project. `CONTRIBUTING.md` is the source of truth for setup, CI, platform support, project layout, and the branch/PR workflow; this file only adds what agents need on top of it.

## Project overview

fvcommand is a cross-platform (Windows and Linux, in that order of priority)
voice-command tool written in Rust. It is intended to capture microphone audio
with `cpal` and act on spoken commands, but the current code is only a skeleton:
a small `eframe`/`egui` desktop window that shows "Hello World!", with
`env_logger` logging and an exit-code helper. Wake-word detection and speech
transcription are planned to run through `cera` (a Rust-native inference
engine) rather than the previously used `rustpotter`/`transcribe-rs`, which were
removed from `Cargo.toml`; no inference code exists yet.

Current dependencies:

- `eframe` / `egui` — desktop GUI (0.37). Code imports `eframe::egui`, so the
  separate `egui` entry in `Cargo.toml` is currently redundant; don't treat it
  as a second crate to import from.
- `cpal` — audio capture (present, not used yet)
- `cera` — Rust-native inference engine, the intended path for wake-word
  detection and speech-to-text (present in `Cargo.toml`, not used yet)
- `tokio` — async runtime (multi-thread, macros, sync, time, signal)
- `toml` — configuration parsing
- `log` / `env_logger` — logging

## Build & run

Toolchain setup, the build/run commands, logging, and the exact CI checks live
in [CONTRIBUTING.md](CONTRIBUTING.md#getting-started) and
[CI & checks](CONTRIBUTING.md#ci--checks). Run those checks before finishing
work — a formatting diff or any clippy warning fails the build.

## Code style & conventions

- Standard Rust naming: `snake_case` functions, default `rustfmt` formatting.
- Keep code idiomatic Rust 2024; run the CI checks (see
  [CONTRIBUTING.md](CONTRIBUTING.md#ci--checks)) before finishing work.
- Log with the `log` macros (`info!`, `warn!`, `error!`, `debug!`) instead of
  `println!` for status output. `env_logger` is initialized once, in `main`.
- Terminate through `exit_program(code)` using the `EXIT_SUCCESS` /
  `EXIT_FAILURE` constants in `main.rs`; `main` itself ends by calling
  `exit_program(EXIT_SUCCESS)`.
- No custom error-handling framework exists yet — use plain `Result`/`?` as the
  code grows, and let `clippy` guide choices.
- Async work is expected to run on `tokio` (features enabled in `Cargo.toml`).

## Notes for agents

- **Layout**: see [Project layout](CONTRIBUTING.md#project-layout). There are
  only those files — don't assume modules that aren't there yet. Splitting out
  further modules (e.g. `config.rs`, `audio.rs`, `wake.rs`, `transcribe.rs`)
  may be appropriate as the program grows.
- **GUI dependency**: the app needs a display and native windowing. Compiling,
  `clippy`, and `test` work headless, but launching it does not — don't try to
  run the app in sandboxes or CI. Keep UI code in `ui.rs` and logic separate
  from the `eframe` app so logic stays testable without a window.
- **Cross-platform code**: see
  [Platform support](CONTRIBUTING.md#platform-support).
- **Audio dependency**: microphone capture via `cpal` may fail in sandboxes, CI,
  or VMs with no audio devices. Prefer design that separates audio capture from
  logic so the logic is testable without a mic.
- **Config**: configuration will be read from `toml` files (`toml` crate). Keep
  config parsing separate from side effects.
- There is no README yet; `CONTRIBUTING.md`, this file, and `.github/` are the
  only non-code files.

## Branch & PR workflow

See [Branching](CONTRIBUTING.md#branching) in `CONTRIBUTING.md`.
