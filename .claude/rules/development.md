---
name: development
description: mise task usage rules for this repository
---

# Development

## Use mise Tasks — Required, Not Optional

- Never invoke the underlying tools directly (`cargo`, `markdownlint-cli2`, `sqruff`, `atlas`, `docker compose`, etc.). Always run the equivalent `mise run <task>` command instead.
- Before running any command that isn't in the list below, check for an existing task first (`mise tasks`) — tasks are defined either in `mise.toml` or as files under `mise/tasks/`.
- If no task exists for what you need, ask the user before running the raw command — do not silently fall back to it.

Key tasks:

- `mise run setup` — first-time setup (installs tools and git hooks)
- `mise run rs-run` — run the application
- `mise run rs-fix` — auto-fix Rust code (clippy + fmt)
- `mise run rs-check` — check Rust code (clippy + fmt + tests)

## Fix and Check After Editing Files

After editing any file, run:

```bash
mise run fix
mise run check
```

For Rust files specifically:

```bash
mise run rs-fix
mise run rs-check
```
