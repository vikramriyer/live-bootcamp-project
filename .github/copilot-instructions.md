# Copilot Instructions

## Code Quality & Design
- Follow SOLID principles and Rust idioms (ownership, borrowing, lifetimes)
- Use traits for shared behavior, prefer composition over inheritance
- Implement custom error types with `Error`, `Display`, and `From` traits
- Use the type system to make invalid states unrepresentable
- Apply Repository pattern for data access, dependency injection via constructors
- Keep functions small and focused, use semantic types over primitives

## Code Style
- Write idiomatic Rust, prioritize clarity over cleverness
- Add doc comments for public APIs with examples
- Use descriptive names, clear module boundaries, appropriate visibility
- Prefer iterators and functional approaches, use `Result<T, E>` for fallible operations
- Avoid `unwrap()` except in tests

## Testing & Validation
After implementing changes, automatically run:
1. `cargo check` - verify compilation
2. `cargo test` - run tests
3. `cargo clippy -- -W clippy::all` - check for issues

Write unit tests for new functions, test behavior not implementation. Use `#[cfg(test)]` modules, mock dependencies with traits, test both happy and error paths.

## Implementation
- Start simple, refactor if needed
- Avoid premature optimization and over-engineering  
- Use standard library before external crates
- Implement `Debug` for all types, `Clone` only when needed
- Handle all `Result`/`Option` types explicitly with meaningful errors
- Prefer static dispatch unless flexibility needed
- Look at existing code to understand project style

## Async Code
- Use `async`/`await` consistently, avoid blocking in async contexts
- Prefer `tokio` runtime, use `Arc<Mutex<T>>` for shared async state
- Be mindful of cancellation safety

Run tests and checks automatically after changes - include results in response.