# Mutation Testing Guideline

This document explains how mutation testing is run and reported in this project.

## Tooling

- [`cargo-mutants`](https://mutants.rs/) introduces small code mutations (e.g. flipping
  operators, changing return values) and checks whether the test suite catches them.
  A surviving ("missed") mutant marks a gap that line coverage alone cannot reveal.
- Pinned in `mise.toml`'s `[tools]`; installed via `mise install`.

## Policy

- Report only — mutation testing never fails CI.
- Pull requests test only the mutants that overlap with changed lines
  (`--in-diff`, diffed against the PR base branch); the daily schedule
  (`0 0 * * *` UTC) tests the full suite.
- Surviving mutants are always listed in the workflow's Job Summary.
- `::warning::` is emitted when the mutation score drops below **80%**, on
  either trigger.

## Running Locally

```bash
mise run rs-mutants        # full suite
mise run rs-mutants-diff   # only mutants overlapping app/mutants.diff
```

`rs-mutants-diff` expects a diff file at `app/mutants.diff`. Generate one against
the default branch before running it:

```bash
git -C app diff --relative origin/main...HEAD > app/mutants.diff
mise run rs-mutants-diff
```

`--relative` strips the `app/` prefix from the diff paths so they match the
file paths `cargo-mutants` sees when it runs with `app/` as its working
directory; a diff generated from the repository root (with an `app/`
prefix) would not overlap with any mutant and silently test nothing.

Both tasks write their results to `app/mutants.out/`:

| File | Purpose |
| --- | --- |
| `outcomes.json` | Machine-readable summary; CI reads `.caught` and `.missed` for the score |
| `missed.txt` | Names of mutants the test suite did not catch |
| `caught.txt`, `timeout.txt`, `unviable.txt` | Other outcome categories |

`app/mutants.out*` and `app/mutants.diff` are gitignored.

## Mutation Score

```text
score = caught / (caught + missed) * 100
```

Mutants that fail to build (`unviable`) or time out (`timeout`) are excluded
from the score, since they don't indicate a test gap.

## CI Workflow

`.github/workflows/mutation-testing.yml` runs on `pull_request` and on a daily
`schedule`, following the `paths` filter pattern used by `check-rust.yml`
(`app/*/src/**`, not `app/src/**`).

Steps:

1. On `pull_request`: diff against the base branch (`git diff
   origin/$GITHUB_BASE_REF...HEAD -- app`) and run `mise run rs-mutants-diff`.
   On `schedule`: run `mise run rs-mutants`.
2. Append `app/mutants.out/outcomes.json`'s summary and `missed.txt`'s
   contents to the GitHub Actions Job Summary.
3. Compute the score from `outcomes.json` and emit `::warning::` if it is
   below 80%.

## Step-by-Step: Investigating a Missed Mutant

1. Run `mise run rs-mutants` (or `rs-mutants-diff` for a quick, local check).
2. Find the function and mutation in `app/mutants.out/missed.txt`.
3. Add or strengthen a test that asserts the actual computed value, not just
   that the call succeeds — this is what usually lets a mutant slip through.
4. Re-run and confirm the mutant now appears in `caught.txt`.
