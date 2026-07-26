# Testing Guideline

This document explains the two kinds of tests in this project, how to decide
which one a given behavior belongs in, and how to run them.

Coverage and mutation testing are described separately in
[coverage.md](coverage.md) and [mutation-testing.md](mutation-testing.md).

## Test Types

| Type | Location | Runs against |
| --- | --- | --- |
| Unit | `#[cfg(test)] mod tests` next to the code under test | The crate alone, no server or database |
| Integration | `tests/runn/*.yml` | A running server and a real database over HTTP |

## Choosing Between Them

Pick the type based on what the behavior needs in order to be observed, not on
which test is easier to write.

- If the behavior can be observed without an HTTP request or a database
  connection, write a unit test: input validation, domain invariants, use case
  branching, configuration parsing.
- If the behavior only manifests through the full request/response cycle —
  routing, middleware, serialization, status codes, auth flows, a database
  round-trip — write a `runn` integration test instead of reconstructing the
  stack in isolation.

A rule of thumb: a unit test tells you a rule is correct, an integration test
tells you the rule is actually reachable from the outside.

## Unit Tests

Unit tests live in the crate they test, in a `#[cfg(test)] mod tests` block at
the bottom of the file — for example `app/use_case/src/create_user.rs`.

Use cases depend on repository traits from `domain`, so a unit test supplies a
hand-written fake implementing the trait rather than a mocking library:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct CapturingUserRepo {
        saved: Mutex<Vec<User>>,
    }

    #[async_trait]
    impl UserRepository for CapturingUserRepo {
        async fn find_by_id(&self, _: &UserId) -> Result<Option<User>, UserError> {
            panic!("not expected")
        }
        // ...
    }
}
```

Make the methods a test does not expect to be called `panic!`, so an unintended
call fails loudly instead of silently returning a default.

Run them with:

```bash
mise run rs-check
```

The task runs `cargo clippy`, `cargo fmt --check`, and `cargo test` together, so
it is also the command to run after editing any Rust file.

## Integration Tests

Integration tests are [runn](https://github.com/k1LoW/runn) scenarios under
`tests/runn/`, one file per API area (`auth.yml`, `health.yml`, `users.yml`).

A scenario declares an HTTP runner, then a sequence of named steps, each with a
request and a `test` expression evaluated against the response:

```yaml
desc: Auth API
runners:
  req: http://localhost:8080
steps:
  login_success:
    desc: POST /auth/login with correct credentials returns 200 with access_token
    req:
      /auth/login:
        post:
          body:
            application/json:
              email: bob@example.com
              password: password123
    test: current.res.status == 200 && current.res.body.access_token != ""
```

Give every step a `desc` that states the expected outcome, and assert on the
response body, not only on the status code.

Steps within a file run in order and share state, so a scenario that needs data
can create it in an earlier step (`setup_user` in `auth.yml`) rather than
depending on seeded rows.

Run them with:

```bash
mise run integration-test
```

Docker is the only prerequisite beyond the mise-managed tools. The task starts
the database container, creates and migrates the `app_test` database, truncates
its tables, starts the server, waits for `/health`, runs every scenario, and
shuts the server and container down afterwards.
