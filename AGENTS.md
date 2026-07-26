# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## General agent rules

- When users ask questions, answer them instead of doing the work.

### Shell Rules

- Always use `rm -f` (never bare `rm`)
- Before running a series of `git` commands, confirm you are in the project root; if not, `cd` there first. Then run all subsequent `git` commands from that directory without the `-C` option.

## Project Overview

A Rust web application template built with [Actix Web](https://actix.rs) and the Onion architecture.
The application code lives in the `app/` directory.
`app/` is a Cargo workspace with each crate in its own subdirectory
(`app/server/src`, `app/domain/src`, etc.) — there is no `app/src/`.
Development tooling (linting, formatting, spell checking, git hooks) is managed via [mise](https://mise.jdx.dev).

## Documentation and Rules

`README.md` and `docs/` are the comprehensive documentation: background,
rationale, and detailed steps, written for humans. `.claude/rules/` is
distilled from it — the concrete instructions an agent must follow, with the
explanation left behind. Overlap between the two is expected, and each rule
links to its comprehensive counterpart directly under its heading.

When a rule changes, update its comprehensive documentation too; a rule must
never state something the documentation contradicts.

Name files under `docs/` in lowercase kebab-case (`template-usage.md`), except
`CONTRIBUTING.md`, which keeps the uppercase name GitHub recognizes.

Rules:

- `.claude/rules/architecture.md` — Onion architecture layers and the allowed
  dependency edges between crates.
- `.claude/rules/ci.md` — GitHub Actions workflow conventions, including the
  `app/*/src/**` `paths:` filter that Rust triggers depend on.
- `.claude/rules/development.md` — mise task usage; how to fix and check after
  editing.
- `.claude/rules/testing.md` — unit vs integration tests and where each belongs.
- `.claude/rules/validation.md` — where input validation belongs and how to add
  a rule with `garde`.
