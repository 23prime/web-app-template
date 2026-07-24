---
name: testing
description: Unit vs integration test classification and where to write each
---

# Testing

This project uses two test types. Pick the type based on what you're
verifying, not on convenience.

## Unit Tests

- Location: `#[cfg(test)] mod tests` blocks co-located with the code under
  test (e.g. `app/use_case/src/login.rs`).
- Use for: pure logic that doesn't require a running server or database —
  input validation, domain rules, use case branching.
- Run via `mise run rs-check` (`cargo test`).

## Integration Tests

- Location: `tests/runn/*.yml`, using the [runn](https://github.com/k1LoW/runn)
  framework.
- Use for: end-to-end behavior of API endpoints over HTTP, against a running
  server and real database — request/response shape, auth flows, status
  codes.
- Run via `mise run integration-test`.

## Choosing Between Them

- If the behavior can be tested without an HTTP request or a database
  connection, write a unit test.
- If the behavior only manifests through the full request/response cycle
  (routing, middleware, serialization, DB round-trip), write a `runn`
  integration test instead of trying to unit-test it in isolation.
