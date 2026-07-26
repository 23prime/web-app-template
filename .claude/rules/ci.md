---
name: ci
description: GitHub Actions workflow conventions for this repository
paths: ".github/workflows/**"
---

# CI Workflows

> Comprehensive documentation: [docs/ci.md](../../docs/ci.md)

## Scope `jdx/mise-action` Installs

- When a job only needs specific tools (e.g. only `rust` for a Rust-only check job), pass `install_args` to `jdx/mise-action` listing just those tools, instead of a bare `install: true`.
- A bare `install: true` installs every tool declared in `mise.toml`'s `[tools]`, even ones the job never uses (e.g. `node`, `atlas`, `sqruff`, `cspell`) — wasted CI time with no benefit to that job.

## Write `paths:` Filters Against the Real Layout

- For Rust source changes, use `app/*/src/**`, not `app/src/**`. Each crate has its own subdirectory (`app/server/src`, `app/domain/src`, etc.); there is no `app/src/`.
- `app/src/**` matches nothing, so the trigger is silently disabled — the workflow never runs and nothing reports an error.
- Include the workflow's own file in its `paths:` filter so changes to it are validated.
