# Copilot Instructions

## Code Quality & Design Patterns

- Follow SOLID principles and Rust idioms (ownership, borrowing, lifetimes)
- Prefer composition over inheritance, use traits for shared behavior
- Implement error handling with custom error types that implement `Error`, `Display`, and where applicable, `From` traits
- Use the type system to make invalid states unrepresentable
- Apply the Repository pattern for data access, keeping database logic separate from business logic
- Use dependency injection through constructor parameters or builder patterns
- Prefer `Arc<dyn Trait>` over concrete types for better testability
- Keep functions small and focused on a single responsibility
- Use semantic types over primitives (e.g., `UserId(u32)` instead of bare `u32`)

## Code Style

- Write idiomatic Rust following the official style guide
- Keep code DRY but prioritize clarity over cleverness
- Add doc comments (`///`) for public APIs, include examples where helpful
- Use descriptive variable names, avoid unnecessary abbreviations
- Structure code with clear module boundaries and sensible visibility modifiers
- Prefer iterators and functional approaches over manual loops where appropriate
- Use `Result<T, E>` for fallible operations, avoid `unwrap()` except in tests

## Testing & Validation

- After implementing new functionality, automatically run:
  1. `cargo check` to verify the code compiles
  2. `cargo test` to run unit tests
  3. `cargo clippy -- -W clippy::all` to check for common mistakes
- Write unit tests for new functions, aim for testing behavior not implementation
- Use `#[cfg(test)]` modules for test code
- Create minimal test fixtures, avoid test interdependencies
- Mock external dependencies using traits and test doubles
- Include both happy path and error case tests

## Implementation Guidelines

- Start with the simplest working solution, then refactor if needed
- Avoid premature optimization and over-engineering
- Use standard library types before reaching for external crates
- When using external crates, prefer well-maintained, widely-used options
- Implement `Debug` for all types, `Clone` only when needed
- Use `#[derive()]` for common traits when possible
- Handle all `Result` and `Option` types explicitly, provide meaningful error messages
- Prefer static dispatch (generics) over dynamic dispatch (trait objects) unless flexibility is needed
- Look at the existing code before suggesting changes to understand the style

## Async Code

- Use `async`/`await` consistently, avoid blocking operations in async contexts
- Prefer `tokio` for async runtime when needed
- Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared state in async code
- Be mindful of cancellation safety in async operations

Do not ask for permission to run tests or checks - just run them after implementing changes and include the results in your response.