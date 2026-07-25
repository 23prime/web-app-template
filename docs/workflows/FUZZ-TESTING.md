# Fuzz Testing Guideline

This document explains how fuzzing is run and reported in this project.

## Tooling

- [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) drives `libFuzzer` to generate
  random inputs and look for panics and other crashes.
- `cargo-fuzz` requires the **nightly** toolchain. It is **not** listed in `mise.toml`'s
  `[tools]` (which assumes the stable toolchain used by the rest of the project); the
  `rs-fuzz` mise task installs the nightly toolchain and `cargo-fuzz` itself via `rustup`
  and `cargo install`. `check-rust` / `integration-test` are unaffected and stay on stable.

## Scope

- Fuzz crate: `app/use_case/fuzz`, a standalone workspace (not a member of the `app/`
  workspace, per `cargo-fuzz`'s convention) that depends on `use_case`.
- Targets fuzz **use_case input validation** (`garde`-based validation), which is pure
  logic requiring no DB:
  - `create_user_input` — fuzzes `CreateUserInput::validate()`
  - `login_input` — fuzzes `LoginInput::validate()`
- This is a scaffold: additional targets can be added under
  `app/use_case/fuzz/fuzz_targets/` and registered as a `[[bin]]` in
  `app/use_case/fuzz/Cargo.toml`.

## Policy

- Unlike Coverage / Mutation Testing's "report only" behavior, a crash **fails CI**.
- Pull requests run each target as a short smoke run (**60s**); the daily schedule
  (`0 0 * * *` UTC) runs each target longer (**300s**).
- On failure, the contents of `app/use_case/fuzz/artifacts/<target>/` are uploaded as a CI
  artifact for download and local reproduction. This directory is only populated when
  `cargo-fuzz` actually finds a crash; a failure during toolchain setup or compilation
  produces no artifact (the upload step tolerates that via `if-no-files-found: ignore`).

## Running Locally

```bash
mise run rs-fuzz -- create_user_input 60
mise run rs-fuzz -- login_input 60
```

The first argument is the target name (matching a `[[bin]]` name in
`app/use_case/fuzz/Cargo.toml`); the second is the run duration in seconds.

`app/use_case/fuzz/{target,corpus,artifacts,coverage}` are gitignored (via
`app/use_case/fuzz/.gitignore`).

## CI Workflow

`.github/workflows/fuzz-testing.yml` runs on `pull_request` and on a daily `schedule`,
following the `paths` filter pattern used by `check-rust.yml` (`app/*/src/**`, not
`app/src/**`), plus `app/use_case/fuzz/**`. It fans out over both targets via a matrix.

Steps per target:

1. On `pull_request`: `mise run rs-fuzz -- <target> 60`.
   On `schedule`: `mise run rs-fuzz -- <target> 300`.
2. On failure, upload `app/use_case/fuzz/artifacts/<target>/` as a CI artifact.

## Step-by-Step: Investigating a Crash

1. Download the `fuzz-artifacts-<target>` artifact from the failed workflow run, or
   reproduce locally by re-running `mise run rs-fuzz -- <target> <seconds>` until it
   crashes.
2. Find the crash input file under `app/use_case/fuzz/artifacts/<target>/`.
3. Reproduce it directly:

   ```bash
   cd app/use_case
   cargo +nightly fuzz run <target> fuzz/artifacts/<target>/<crash-file>
   ```

4. Fix the underlying panic and re-run to confirm the crash no longer reproduces.
