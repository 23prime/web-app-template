# CI Guideline

This document describes the conventions to follow when adding or changing a
GitHub Actions workflow in this repository. It does not catalogue the
individual workflows — read `.github/workflows/` for that. Not every existing
workflow satisfies every convention below; bring one in line when you touch it.

`mise run gh-check` enforces part of these conventions with `actionlint` and
`zizmor`, and `.github/workflows/check-github-workflow.yml` runs the same check
in CI.

## Pin Actions to a Commit SHA

Reference every action by its full commit SHA, with the human-readable version
in a trailing comment:

```yaml
- uses: actions/checkout@<full-commit-sha> # <version>
```

A mutable tag such as `@v7` can be moved to a different commit by the action's
owner, so pinning by SHA is what actually fixes the code that runs. The comment
keeps the version readable and lets Renovate update both together.

## Grant the Least Privilege

Declare `permissions: {}` at the workflow level, then grant each job only what
it needs:

```yaml
permissions: {}

jobs:
  check:
    permissions:
      contents: write # push auto-fix commits
```

The empty workflow-level map drops the default token scopes, so a job that
forgets to declare `permissions` gets nothing rather than everything. Annotate
any write permission with the reason it is needed.

Pass `persist-credentials: false` to `actions/checkout` unless a later step
genuinely needs to push with the checkout credentials.

## Declare a Concurrency Group

Every workflow declares a concurrency group keyed on the workflow and the ref:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: false
```

Use `cancel-in-progress: false` for workflows that push commits back to the
branch or publish a report — cancelling those midway leaves the branch or the
Job Summary in a half-finished state. Read-only checks that are cheap to redo
can set it to `true`.

## Scope `jdx/mise-action` Installs

When a job needs only specific tools, pass `install_args` listing just those
tools:

```yaml
- uses: jdx/mise-action@<full-commit-sha> # <version>
  with:
    install: true
    install_args: "markdownlint-cli2"
    cache: true
    github_token: ${{ secrets.GITHUB_TOKEN }}
  env:
    MISE_LOCKED: "1"
```

A bare `install: true` installs every tool declared in `mise.toml`'s `[tools]`,
even ones the job never uses (e.g. `node`, `atlas`, `sqruff`, `cspell`) —
wasted CI time with no benefit to that job.

Set `MISE_LOCKED: "1"` on the `jdx/mise-action` step so mise resolves tools to
the versions recorded in `mise.lock` instead of the newest version matching the
requested range. This is what keeps CI reproducible.
`.github/workflows/check-mise-lock.yml` does not need it: it installs nothing
(`install: false`) and re-locks the file itself.

## Write `paths:` Filters Against the Real Layout

`app/` is a Cargo workspace whose crates each live in their own subdirectory, so
Rust sources are at `app/<crate>/src/**`. There is no `app/src/`:

```yaml
on:
  pull_request:
    paths:
      - "app/*/src/**" # not "app/src/**"
      - "app/Cargo.toml"
      - "app/Cargo.lock"
      - "mise.toml"
      - "mise.lock"
      - ".github/workflows/check-rust.yml"
```

`app/src/**` matches nothing, which silently disables the trigger — the
workflow simply never runs, and nothing reports an error. Include the
workflow's own file in the filter so changes to it are validated.

## Fix, Then Check

Lint workflows run the auto-fix task, push whatever it changed, and only then
run the check task:

```yaml
- name: Fix
  run: mise run md-fix

- name: Commit and Push
  uses: 23prime/simple-commit-and-push@<full-commit-sha> # <version>
  with:
    commit-message: "Auto fix by workflow"
    github-token: ${{ secrets.GITHUB_TOKEN }}

- name: Check
  run: mise run md-check
```

Mechanical problems are corrected on the branch instead of being handed back to
the author, and the following check step still fails the run for anything the
fixer could not resolve. Jobs shaped this way need `contents: write`.

## Run Tools Through mise

Call `mise run <task>` from workflow steps rather than invoking `cargo`,
`markdownlint-cli2`, `atlas`, or `cspell` directly, so CI and local development
run the same command with the same options. Run `mise tasks` to see what is
available.
