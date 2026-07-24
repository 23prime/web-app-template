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
- `container` — holds the `domain` trait objects (e.g. `Arc<dyn UserRepository>`)
  that `presentation` consumes to wire them into use cases. Depends only on
  `domain`; it does not know about `infrastructure`. `use_case` itself never
  depends on `container` — it receives domain abstractions directly through
  its own function signatures.
- `server` — composition root: picks the concrete `infrastructure`
  implementations, constructs the `container` with them, starts Actix Web, and
  wires middleware. Depends on every other crate.

## Dependency Rule

- Allowed dependency edges:
  - `use_case`, `infrastructure`, `container` → `domain`
  - `presentation` → `domain`, `use_case`, `container`
  - `server` → `domain`, `use_case`, `infrastructure`, `presentation`, `container`
- `domain` must never depend on any other layer.
- `use_case`, `infrastructure`, and `container` must not depend on each
  other — all three depend only on `domain`, and are connected in `server`,
  which is the only crate that references concrete `infrastructure` types.
- New cross-layer trait implementations belong in `infrastructure`; new
  application logic belongs in `use_case`; new business rules and types belong
  in `domain`.
