---
name: validation
description: Where input validation belongs across layers and how to add a rule with garde
paths: "app/*/src/**"
---

# Validation

> Comprehensive documentation: [docs/validation.md](../../docs/validation.md)

## Where Validation Belongs

- `presentation` — structural validity only, handled by `serde` (type errors,
  missing required fields). It must not know business rules.
- `use_case` — business rules (empty strings, length limits, formats) via
  [`garde`](https://crates.io/crates/garde).
- `domain` — invariants that must always hold, enforced in constructors and
  value objects that return `Result`. The domain must never depend on `garde`.

`garde` validation in the use case runs *before* domain constructors, so format
checks live in the use case and "cannot exist in an invalid state" checks live
in the domain.

## Adding a Validation Rule

1. Derive `garde::Validate` on the use case's Input struct and annotate the
   field (e.g. `#[garde(length(min = 1))]`, `#[garde(email)]`).
2. Call `input.validate().map_err(<Error>::Validation)?` as the first statement
   of `execute()`.
3. Ensure the use case's error enum has a `Validation(garde::Report)` variant.
   A use case that returned a domain error directly needs its own error enum.
4. Add a unit test in the use case crate that triggers the rule and asserts the
   error is `Validation`.
5. Run `mise run rs-check`.

## HTTP Response

- Validation failures return `422 Unprocessable Entity` with
  `{"error": "validation_error", "fields": {"<field>": ["<message>"]}}`.
- Build the `fields` map with the shared `validation_fields` helper in
  `presentation/src/lib.rs`; do not format `garde::Report` in each handler.
- Do not `impl actix_web::ResponseError` for use case errors — both the error
  type and the trait are foreign to `presentation`, so the orphan rule forbids
  it. The helper achieves the same deduplication.
