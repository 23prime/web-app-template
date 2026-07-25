# Coverage Guideline

This document explains how test coverage is measured and reported in this project.

## Tooling

- [`cargo-llvm-cov`](https://crates.io/crates/cargo-llvm-cov) measures line, region, and
  branch coverage using LLVM source-based code coverage.
- Pinned in `mise.toml`'s `[tools]`; installed via `mise install`.

## Policy

- Report only — coverage never fails CI.
- Coverage is always displayed in the workflow's Job Summary.
- `::warning::` is emitted when line coverage drops below **90%**.
- No external service integration (e.g. Codecov); everything is self-contained.

## Running Locally

```bash
mise run rs-coverage
```

This runs `cargo llvm-cov --workspace --all-features` and writes:

| File | Purpose |
| --- | --- |
| `app/coverage-summary.txt` | Human-readable table, same content posted to the CI Job Summary |
| `app/coverage.json` | Machine-readable summary; CI reads `.data[0].totals.lines.percent` for the threshold check |

Both files are gitignored.

## Requirement: `llvm-tools` Component

`cargo-llvm-cov` needs the `llvm-tools` rustup component. Locally:

```bash
rustup component add llvm-tools-preview
```

CI installs it automatically via `actions-rust-lang/setup-rust-toolchain`'s `components: llvm-tools`.

## CI Workflow

`.github/workflows/coverage.yml` runs on `push` / `pull_request`, following the
`paths` filter pattern used by `check-rust.yml` (`app/*/src/**`, not `app/src/**`).

Steps:

1. `mise run rs-coverage`
2. Append `app/coverage-summary.txt` to the GitHub Actions Job Summary
3. Read `app/coverage.json`'s line coverage percentage and emit `::warning::` if it is below 90%

## Step-by-Step: Checking Coverage for a Change

1. Run `mise run rs-coverage`
2. Check the `TOTAL` row's `Lines` cover column in `app/coverage-summary.txt`
3. Look at per-file rows to find uncovered code paths
