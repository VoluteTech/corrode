# Agent Constitution: corrode

## Project Overview
`corrode` is a terminal user interface (TUI) weather application built in Rust using `ratatui` and `crossterm`. It fetches asynchronous weather data from weather APIs and renders responsive, real-time widgets in the terminal.

## Core Tech Stack
- **Language:** Rust (Edition 2021)
- **TUI Framework:** `ratatui` + `crossterm` backend
- **Async Runtime:** `tokio`
- **HTTP Client & Serialization:** `reqwest`, `serde`, `serde_json`
- **Error Handling:** `thiserror` (for libraries/internal modules) or `anyhow` (for application-level errors)

---

## Architectural Principles & Rules

### 1. Separation of Concerns (Crucial for TUI Apps)
- **Model / Application State (`app.rs`):** Holds pure data, tab states, selected UI indices, and cached weather payload structs.
- **UI Layer (`ui.rs` or `ui/` directory):** Pure, deterministic rendering functions. Drawing functions MUST take `&App` and `&mut Frame`—they MUST NOT perform any network calls or mutating I/O.
- **Async Event / I/O Loop (`event.rs` / `api.rs`):** Weather API fetching and keyboard input handling run asynchronously. State updates must be passed back to the main UI loop using `tokio::mpsc` channels.

### 2. Rust & Coding Standards
- **Idiomatic Rust:** Prefer pattern matching, `Option`/`Result` combinators (`map`, `and_then`), and explicit error types.
- **No Unsafe Code:** Absolutely NO `unsafe` blocks unless explicitly authorized with a written justification.
- **Zero Warnings:** Code must compile cleanly with `cargo clippy -- -D warnings`.
- **Error Propagation:** Never use `.unwrap()` or `.expect()` in production paths. Always handle or propagate `Result` gracefully to avoid crashing the user's terminal session.

### 3. TUI & UX Guidelines
- **Terminal Restoration:** Ensure terminal raw mode is properly cleaned up on drop/panic so the user's shell is never corrupted.
- **Responsive Layouts:** Use `ratatui::layout` (`Layout::default().direction(...).constraints(...)`) to ensure widgets resize gracefully without overflowing small terminal windows.
- **Loading & Error States:** Always provide visual feedback (e.g., spinners or error popups) when network fetches are in flight or fail.

---

## Project Structure
- `src/main.rs`: Entry point, terminal initialization, cleanup, and main event loop.
- `src/app.rs`: Application state holding weather data, navigation, and state mutators.
- `src/ui/`: Ratatui widget definitions and layout rendering code.
- `src/api/`: Weather client structs, API endpoints, and DTO parser models.
- `src/errors.rs`: Central error definitions.

---

## Verification Requirements
Before declaring any task complete, you MUST execute the following check chain:
1. `cargo fmt --check` (Ensure formatting meets standard formatting)
2. `cargo check` (Fast syntax and type verification)
3. `cargo clippy -- -D warnings` (Strict linting check)
4. `cargo test` (Run unit and integration tests)
