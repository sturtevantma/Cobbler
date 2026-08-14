# File Structure & Conventions

This document describes the file layout, naming conventions, and organizational practices to follow when contributing to Cobbler.

---

## Directory Layout

```
Cobbler/
├── Cargo.toml          # Package manifest and dependency declarations
├── Cargo.lock          # Auto-generated lockfile — commit for binaries, not libraries
├── FILE_STRUCTURE.md   # This document
├── README.md           # Project overview and quick-start guide
├── src/
│   ├── main.rs         # Binary entry point
│   ├── lib.rs          # Library root (add when splitting binary/library)
│   ├── config/         # Configuration loading and types
│   ├── models/         # Domain data structures
│   ├── services/       # Business logic / core algorithms
│   └── utils/          # Shared helper functions
└── tests/
    └── integration/    # Integration tests (each file is a separate test crate)
```

> Add new top-level modules as subdirectories under `src/` with their own `mod.rs` (or as a single `.rs` file for small modules).

---

## Naming Conventions

### Files & Modules
| Item | Convention | Example |
|------|-----------|---------|
| Source files | `snake_case.rs` | `user_profile.rs` |
| Module directories | `snake_case/` with `mod.rs` | `src/services/mod.rs` |
| Test files (unit) | Same file as the module under `#[cfg(test)]` | `src/models/user.rs` |
| Integration tests | `snake_case.rs` under `tests/` | `tests/integration/auth_flow.rs` |
| Markdown docs | `UPPER_SNAKE_CASE.md` for repo-level docs | `FILE_STRUCTURE.md`, `README.md` |

### Rust Identifiers
| Item | Convention | Example |
|------|-----------|---------|
| Types / Traits / Enums | `UpperCamelCase` | `UserProfile`, `ConfigError` |
| Enum variants | `UpperCamelCase` | `NotFound`, `ParseError` |
| Functions / Methods | `snake_case` | `load_config()` |
| Variables / Parameters | `snake_case` | `user_id` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_RETRIES` |
| Statics | `SCREAMING_SNAKE_CASE` | `DEFAULT_TIMEOUT` |
| Macros | `snake_case!` | `log_error!()` |
| Crate / package name | `snake_case` | `cobbler` |
| Feature flags (`Cargo.toml`) | `kebab-case` | `feature = "async-runtime"` |

---

## Module Organization

- Each module should have a single, clear responsibility.
- Prefer splitting large modules into submodules rather than creating very long files.
- Re-export public items at the module root (`mod.rs` or `lib.rs`) so consumers have a stable API surface.
- Keep `main.rs` minimal — wire up components here; put logic in `lib.rs` or dedicated modules.

---

## Adding Dependencies

1. Add dependencies via `cargo add <crate>` to keep `Cargo.toml` consistent.
2. Group dependencies in `Cargo.toml` by purpose with inline comments:
   ```toml
   [dependencies]
   # Async runtime
   tokio = { version = "1", features = ["full"] }

   # Serialization
   serde = { version = "1", features = ["derive"] }
   ```
3. Keep dev-only and build-only crates under `[dev-dependencies]` and `[build-dependencies]` respectively.

---

## Testing

- **Unit tests** live in the same file as the code under a `#[cfg(test)]` module.
- **Integration tests** live in `tests/` and test the public API of the crate.
- Test function names should describe the scenario: `test_<function>_<scenario>`.
  ```rust
  #[test]
  fn test_parse_config_missing_field() { ... }
  ```

---

## General Practices

- Run `cargo fmt` before committing to enforce consistent formatting.
- Run `cargo clippy` and address all warnings before opening a PR.
- Keep public items documented with `///` doc comments.
- Avoid `unwrap()` and `expect()` in production code; propagate errors with `?`.
