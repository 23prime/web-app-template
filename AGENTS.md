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
When writing GitHub Actions `paths:` filters for Rust source changes, use
`app/*/src/**`, not `app/src/**` (the latter never matches anything and
silently disables the trigger).
Development tooling (linting, formatting, spell checking, git hooks) is managed via [mise](https://mise.jdx.dev).

See `.claude/rules/development.md`, `.claude/rules/architecture.md`, and
`.claude/rules/testing.md` for development workflow, architecture, and
testing rules.
