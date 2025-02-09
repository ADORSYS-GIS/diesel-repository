# Diesel Repository Macros

A set of procedural macros for Diesel that auto‑generates repository types and CRUD implementations in a “Spring‑JPA‑like” style.

## Features

- **Repository Derivation:**  
  Use `#[derive(Repository)]` with a custom attribute to generate a repository type for your entity.

- **CRUD & Paging Macros:**  
  Annotate your entity with `#[crud_repo(...)]` and `#[paging_repo(...)]` to automatically implement repository traits such as find, insert, update, delete, and paging.

- **Async & Sync Support:**  
  Enable asynchronous (Tokio‑based) implementations via the `async` Cargo feature, or compile the synchronous version by default.

## Example

Annotate your Diesel entity as follows:

```rust
use diesel_repository::{Repository, crud_repo, paging_repo};

#[derive(
    Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Insertable, AsChangeset, Repository,
)]
#[repository(pool = "db::DbPool")]
#[diesel(table_name = crate::accounts)]
#[crud_repo(find_all, find_one, insert, update, delete)]
#[paging_repo(find_all)]
pub struct Account {
    pub id: String,
    pub sub: String,
    pub name: String,
}
```

This generates an `AccountRepo` type with implementations for the repository traits (both async and sync based on your feature flag).

Look at the projects `diesel-repository-test-async` and `diesel-repository-test-sync` for more.

## Testing

The workspace includes dedicated test packages:

- **Async Tests:**  
  Run:
  ```bash
  cargo run -p diesel-repository-test-async
  ```
- **Sync Tests:**  
  Run:
  ```bash
  cargo run -p diesel-repository-test-sync
  ```

## License

This project is dual‑licensed under the MIT and Apache-2.0 licenses.
