# AGENTS.md

Guidance for AI coding agents (and humans working with agents) in this repository.

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

- `eframe` / `egui` — desktop GUI (0.37)
- `cpal` — audio capture (present, not used yet)
- `cera` — Rust-native inference engine, the intended path for wake-word
  detection and speech-to-text (present in `Cargo.toml`, not used yet)
- `tokio` — async runtime (multi-thread, macros, sync, time, signal)
- `toml` — configuration parsing
- `log` / `env_logger` — logging

## Build & run

- Rust edition 2024 — requires a recent stable toolchain (`rustup update stable`).
- `cargo build` — compile
- `cargo run` — run the binary (opens a native GUI window; needs an interactive
  desktop session, so it will not work in headless CI or a plain sandbox)
- `cargo test` — run tests
- `cargo clippy` — lints
- `cargo fmt` — format (rustfmt, edition 2024)

Logging defaults to the `info` level; override with the `RUST_LOG` environment
variable (`$env:RUST_LOG="debug"` in PowerShell, `RUST_LOG=debug cargo run` on
Linux/bash).

CI (`.github/workflows/ci.yml`) runs on every push/PR to `dev` and `main`, on
both `ubuntu-latest` and `windows-latest`. It checks `cargo fmt --all -- --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo build --locked`, and
`cargo test --locked`, so run the same commands locally before pushing. Keep
`Cargo.lock` committed and in sync. Dependabot
(`.github/dependabot.yml`) opens weekly cargo and GitHub Actions update PRs
against `dev`.

## Code style & conventions

Derived from the current `src/main.rs` and `src/ui.rs`:

- Standard Rust naming: `snake_case` functions, default `rustfmt` formatting.
- Keep code idiomatic Rust 2024; run `cargo fmt` and `cargo clippy` before
  finishing work.
- Log with the `log` macros (`info!`, `warn!`, `error!`, `debug!`) instead of
  `println!` for status output. `env_logger` is initialized once, in `main`.
- Terminate through `exit_program(code)` using the `EXIT_SUCCESS` /
  `EXIT_FAILURE` constants in `main.rs`; `main` itself ends by calling
  `exit_program(EXIT_SUCCESS)`.
- No custom error-handling framework exists yet — use plain `Result`/`?` as the
  code grows, and let `clippy` guide choices.
- Async work is expected to run on `tokio` (features enabled in `Cargo.toml`).

## Notes for agents

- **Layout**: `src/main.rs` is the entry point (logging setup, exit helper,
  launching the UI); `src/ui.rs` holds the `eframe`/`egui` app (`EguiApp`).
  There are only these two files — don't assume modules that aren't there yet.
  Splitting out further modules (e.g. `config.rs`, `audio.rs`, `wake.rs`,
  `transcribe.rs`) may be appropriate as the program grows.
- **GUI dependency**: the app needs a display and native windowing. Compiling,
  `clippy`, and `test` work headless, but `cargo run` does not — don't try to
  launch it in sandboxes or CI. Keep UI code in `ui.rs` and logic separate from
  the `eframe` app so logic stays testable without a window.
- **Cross-platform code**: Windows and Linux are the supported targets. Avoid
  OS-specific APIs in shared code; gate them with `#[cfg(windows)]` /
  `#[cfg(unix)]` (or `#[cfg(target_os = ...)]`) and provide both paths. Use
  `std::path` instead of hard-coded separators, and don't assume Windows-only
  things like drive letters, `%APPDATA%`, or PowerShell. On Linux, `eframe` and
  `cpal` need system libraries (X11/Wayland, OpenGL, ALSA); if a headless build
  fails there, it's usually missing dev packages rather than a code bug. Prefer
  dependencies that support both OSes, which all current ones do.
- **Audio dependency**: microphone capture via `cpal` may fail in sandboxes, CI,
  or VMs with no audio devices. Prefer design that separates audio capture from
  logic so the logic is testable without a mic.
- **Config**: configuration will be read from `toml` files (`toml` crate). Keep
  config parsing separate from side effects.
- **Platform directories**: config/data/cache paths should go through the `dirs`
  crate (**not added yet** — add it when the first file path is needed) rather
  than hard-coded paths, e.g. an `fvcommand` folder under `dirs::config_dir()`.
  That resolves to `%APPDATA%` on Windows and `~/.config` on Linux, satisfying
  the cross-platform requirement without per-OS branches.
- There is no README yet; `CONTRIBUTING.md`, this file, and `.github/` are the
  only non-code files.

## Branch & PR workflow

Feature/issue work happens on short-lived branches named
`<did>-<thing>-<type>`, where `<did>` is the work done, in past tense
(`fixed`, `implemented`, `updated`), `<thing>` is a short kebab-case topic,
and `<type>` is `issue` or `feature` (e.g. `fixed-mic-latency-issue`).
Branch from `dev`; branches merge into `dev`,
and `dev` merges into `main`. See CONTRIBUTING.md for the full workflow.
