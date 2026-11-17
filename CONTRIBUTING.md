# Contributing to fvcommand

Rough guide. Don't stress too much, but do follow the branch stuff below — that's the important part.

## Getting started

- Clone the repo
- Install a recent **stable Rust** toolchain (`rustup update stable` — the project uses edition 2024, so an older rustc won't cut it)
- `cargo build` — first build pulls in cpal, rustpotter, tokio, toml, transcribe-rs (transcribe-rs with the `onnx` feature can make the first build slow)
- `cargo run` — this will eventually be a voice-command app wanting a microphone, but right now it's just the starter skeleton, so it just prints hello

## Branching (please read this one)

All work happens on **issue-specific branches** named:

```
<did>-<thing>-<type>
```

- `<did>` — the work done, in past tense, e.g. `fixed`, `implemented`, `updated`
- `<thing>` — short kebab-case topic, e.g. `mic-latency`, `wake-word-config`
- `<type>` — `issue` or `feature`

Examples: `fixed-mic-latency-issue`, `implemented-config-file-feature`.

- **One branch = one issue/feature.** No kitchen-sink branches.
- Branch from `dev`.
- Merge flow: `<did>-<thing>-<type>` → `dev` → `main`.
  - `dev` is the integration branch, `main` is stable/released.
  - PRs from these branches target `dev`; PRs from `dev` target `main`.
- **Never commit directly to `main`** (and prefer not to `dev` directly either).

## Commits & PRs

- Small, focused commits.
- Imperative subject lines ("fix mic buffer size", not "fixed the mic buffer").
- PRs should link the issue they fix.
- Before merge, make sure `cargo fmt`, `cargo clippy`, and `cargo test` all pass.

## Reporting issues

Rough checklist:

- What happened vs. what you expected to happen
- How to reproduce it (steps)
- Relevant config / log output
- OS + audio hardware context (this app does microphone capture, so "which mic / sound card / OS" matters a lot)

## Agents

Agents working in this repo: read `AGENTS.md` first.
