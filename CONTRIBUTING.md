# Contributing to fvcommand

Rough guide. Don't stress too much, but do follow the branch stuff below — that's the important part.

## Getting started

- Clone the repo
- Install a recent **stable Rust** toolchain (`rustup update stable` — the project uses edition 2024, so an older rustc won't cut it)
- `cargo build` — the first build pulls in the GUI, audio, and inference stacks, so it's noticeably slower than a plain toy project. The dependency list and versions live in `Cargo.toml`, with notes in `AGENTS.md`
- `cargo run` — opens a small native window that just says "Hello World!". It's still the skeleton: no microphone, wake word, or transcription yet, and it needs a real desktop session (not headless/CI)
- Logging goes to the console at `info` by default; set `RUST_LOG=debug` on Linux/bash (or `$env:RUST_LOG="debug"` in PowerShell) for more detail

## Platform support

Windows and Linux are the supported targets (Windows first, Linux close behind). Keep it that way:

- Don't hard-code Windows paths, drive letters, `%APPDATA%`, or shell commands — use `std::path` and keep shell-specific advice out of the code.
- When something really is OS-specific, put it behind `#[cfg(windows)]` / `#[cfg(unix)]` with a working path for both.
- On Linux, `eframe`/`egui` and `cpal` need system dev packages (X11/Wayland, OpenGL, ALSA). A Linux build error about missing headers is usually that, not your code.
- Config/data/cache paths should use the `dirs` crate (**not added yet**) so the same code lands in `%APPDATA%` on Windows and `~/.config` on Linux — don't invent per-OS path logic.
- CI builds on both Windows and Ubuntu, so a PR that only works on one OS will show up red.

## Project layout

- `src/main.rs` — entry point: logger setup, `EXIT_SUCCESS`/`EXIT_FAILURE` + `exit_program`
- `src/ui.rs` — the `eframe`/`egui` app (`EguiApp`)

That's the whole codebase right now. Keep UI code in `ui.rs` and put logic
elsewhere as it grows (audio, wake word, transcription, config), so it stays
testable without a window or a mic.

## Branching

All work happens on **issue-specific branches** named:

```
<did>-<thing>-<type>
```

- `<did>` — the work this branch is for, e.g. `fix`, `implement`, `update`
- `<thing>` — short kebab-case topic, e.g. `mic-latency`, `wake-word-config`
- `<type>` — `issue` for a bug fix, `feature` for new features.

Examples: `fix-mic-latency-issue`, `implement-config-file-feature`.

**`<type>` is optional** for work that is neither a bug fix nor a new feature — docs, chores, CI, refactors. Then the name is just `<did>-<thing>`, e.g. `update-markdowns`.

Branch names are past tense while commit subjects are imperative (see **Commits & PRs**) — that asymmetry is deliberate.

- **One branch = one issue/feature.** No kitchen-sink branches.
- Branch from `dev`.
- Merge flow: `<did>-<thing>-<type>` → `dev` → `main`.
  - `dev` is the integration branch, `main` is stable/released.
  - PRs from these branches target `dev`; PRs from `dev` target `main`.
- **Never commit directly to `main`** (and prefer not to `dev` directly either).
- Branches and PRs that predate this rule aren't being renamed — it applies to work started from now on.

## CI & checks

Every push and PR to `dev` or `main` runs `.github/workflows/ci.yml` on **Ubuntu and Windows**. It runs, in this order:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo build --locked`
- `cargo test --locked`

Run the same locally before pushing (clippy warnings fail CI, not just warn). Keep `Cargo.lock` committed — CI uses `--locked`, so a stale lockfile fails the build; after editing `Cargo.toml`, run `cargo build` and commit the lockfile bump with the change. A red check on one OS only usually means platform-specific code or missing Linux dev packages.

There are no tests yet, so `cargo test` passes trivially — that doesn't mean logic is covered. Add tests when you add logic.

[Dependabot](.github/dependabot.yml) opens weekly PRs for crate and GitHub Actions updates against `dev`. They're grouped for minor/patch bumps; review and merge them rather than ignoring them.

## Commits & PRs

- PRs should link the issue they fix.
- Before merge, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --locked` must pass by not having errors — see **[CI & checks](#ci--checks)**.

## Reporting issues

Rough checklist:

- What happened vs. what you expected to happen
- How to reproduce it (steps)
- Relevant config / log output
- OS (Windows or Linux) + audio hardware context (this app is heading toward microphone capture, so "which mic / sound card / OS" will matter; for now also note whether the window opened at all and what the console log said)

## Agents

**Using LLM's to help you fix or write code is allowed**, **but** the code must meet quality standards, be working, and follow coding conventions by
the time it's being pushed.
You also need to know what the code is doing. If you ask an agent to write code, and it outputs vulnerable or non-working code,
that's on you.

Agents working in this repo: read [AGENTS.md](AGENTS.md) first.
