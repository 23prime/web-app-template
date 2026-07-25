---
name: ci
description: GitHub Actions workflow conventions for this repository
paths: ".github/workflows/**"
---

# CI Workflows

## Scope `jdx/mise-action` Installs

- When a job only needs specific tools (e.g. only `rust` for a Rust-only check job), pass `install_args` to `jdx/mise-action` listing just those tools, instead of a bare `install: true`.
- A bare `install: true` installs every tool declared in `mise.toml`'s `[tools]`, even ones the job never uses (e.g. `node`, `atlas`, `sqruff`, `cspell`) — wasted CI time with no benefit to that job.
