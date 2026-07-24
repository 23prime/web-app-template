---
name: architecture
description: Onion architecture layers and dependency rules for the app/ workspace
---

# Architecture

This project follows the Onion architecture. `app/` is a Cargo workspace; each
layer lives in its own crate under `app/<layer>/src`.

## Layers and Responsibilities

- `domain` — entities, value objects, domain errors, and repository trait
  definitions (e.g. `UserRepository` in `domain/src/user.rs`). No dependency
  on any other layer.
- `use_case` — application business logic (e.g. `login`, `create_user`,
  `get_user`). Depends only on `domain`.
- `infrastructure` — concrete adapters to external systems (e.g. the
  Postgres-backed `UserRepository` implementation). Depends only on `domain`.
- `presentation` — Actix Web HTTP handlers; maps requests/responses to
  `use_case` calls. Depends on `domain`, `use_case`, and `container`.
- `container` — dependency injection: wires concrete `infrastructure`
  implementations to the trait objects `presentation`/`use_case` consume.
  Depends only on `domain`.
- `server` — composition root: builds the `container`, starts Actix Web, wires
  middleware. Depends on every other crate.

## Dependency Rule

- A crate may only depend on crates strictly inside it (closer to `domain`):
  `domain` ← `use_case` / `infrastructure` ← `presentation` / `container` ← `server`.
- `domain` must never depend on any other layer.
- `use_case` and `infrastructure` must not depend on each other — both depend
  only on `domain`, and are connected through `container`/`server`.
- New cross-layer trait implementations belong in `infrastructure`; new
  application logic belongs in `use_case`; new business rules and types belong
  in `domain`.
