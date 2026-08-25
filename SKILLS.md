# Skills: corrode

## Skill: verify-codebase
- **Trigger:** Use after making any code edits to ensure the project compiles and follows Rust standards.
- **Steps:**
  1. Execute `cargo fmt --check`
  2. Execute `cargo clippy -- -D warnings`
  3. Execute `cargo test`

## Skill: run-dev-build
- **Trigger:** Use when testing the application executable locally.
- **Steps:**
  1. Run `cargo run`
  2. Verify stdout prints cleanly and handles mock arguments without crashing.
