# SQL Check

Compile-time SQL validation extracted from SQLx.

This crate provides a `check!` macro that validates SQL queries at compile time by connecting to a database and ensuring the query is valid.

## Features

- **Compile-time SQL validation**: Catch SQL errors before your code runs
- **Feature-gated**: Enable/disable validation with a feature flag
- **Multi-database support**: PostgreSQL, MySQL, and SQLite
- **Zero runtime overhead**: Validation happens only at compile time

## Usage

Add `sql-check` to your `Cargo.toml`:

```toml
[dependencies]
sql-check = "0.9.0-alpha.1"

# Enable database support
sql-check = { version = "0.9.0-alpha.1", features = ["postgres"] }
```

> [!IMPORTANT]
> SQL Check uses the same feature flags as SQLx for database, async runtime, and TLS support. See [Feature Flags](https://github.com/launchbadge/sqlx/tree/1dd526a2ed67fa763766e670c30b1ce3b152a42e?tab=readme-ov-file#cargo-feature-flags) for details.
>
> SQL Check's versioning aligns with SQLx. If you, for whatever reason, use both in your project, ensure you use the same version of SQL Check as you're using for SQLx.

### Basic Example

```rust
use sql_check::check;

// This will be validated at compile time
let sql = check!("SELECT * FROM users WHERE id = 1");

// Use the SQL string with your database library
let result = database.execute(sql).await?;
```

### Feature Flags

#### Core Features

- **`check`** (default): Enables compile-time SQL validation
  - When enabled: Validates SQL by connecting to database, then returns the SQL string
  - When disabled: Macro becomes a no-op and just returns the SQL string

#### Database Support

- **`postgres`**: PostgreSQL support
- **`mysql`**: MySQL support
- **`sqlite`**: SQLite support

#### Runtime Support (choose one)

- **`_rt-tokio`**: Use Tokio runtime (recommended)
- **`_rt-async-std`**: Use async-std runtime
- **`_rt-smol`**: Use smol runtime
- **`_rt-async-global-executor`**: Use async-global-executor runtime

#### TLS Support (choose one)

- **`_tls-rustls-ring-webpki`**: Use rustls with ring and webpki
- **`_tls-rustls-aws-lc-rs`**: Use rustls with AWS-LC
- **`_tls-rustls-ring-native-roots`**: Use rustls with ring and native roots
- **`_tls-native-tls`**: Use native-tls

### Example with Features

```toml
[dependencies]
sql-check = { 
    version = "0.9.0-alpha.1", 
    features = ["postgres", "_rt-tokio", "_tls-rustls-ring-webpki"] 
}
```

## Environment Variables

The macro requires the following environment variables:

- **`DATABASE_URL`**: Connection string for your database
  - PostgreSQL: `postgres://user:pass@host/db`
  - MySQL: `mysql://user:pass@host/db`
  - SQLite: `sqlite://path/to/db.sqlite`

### Example

```bash
# Set DATABASE_URL before building
export DATABASE_URL="postgres://user:pass@localhost/mydb"
cargo build
```

## Disabling Validation (Production)

For production builds, or when you don't have a database connection, disable the `check` feature:

```toml
[dependencies]
sql-check = { version = "0.9.0-alpha.1", default-features = false }
```

Or use cargo build flags:

```bash
cargo build --release --no-default-features
```

## How It Works

1. **At Compile Time**: The macro connects to your database
2. **Sends PREPARE**: Database parses and validates the SQL
3. **Returns Metadata**: Database returns column and parameter information
4. **Compilation Success**: If valid, macro expands to the SQL string
5. **Compilation Error**: If invalid, compilation fails with database error

## Example Error

```rust
let sql = check!("SELECT invalid_column FROM nonexistent_table");
```

**Compile Error:**

```text
error: SQL validation failed: relation "nonexistent_table" does not exist
  --> src/main.rs:5:15
   |
5  |     let sql = check!("SELECT invalid_column FROM nonexistent_table");
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## Comparison with query! Macro

<!-- markdownlint-disable MD060 -->

| Feature        | SQL Check `check!`      | SQLx `query!` |
| -------------- | ----------------------- | ------------- |
| SQL Validation | ✅                      | ✅            |
| Type Checking  | ❌                      | ✅            |
| Returns        | SQL String              | Typed Query   |
| Use Case       | Compile-time validation | Full ORM      |

The `check!` macro is ideal when you:

- Want SQL validation without SQLx's full macro system
- Use a different database library (e.g., tokio-postgres, diesel)
- Need just validation, not type generation

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

This crate is extracted from [SQLx](https://github.com/launchbadge/sqlx) (based on the [Extract Compile Time SQL Statement Verification into its own macro crate](https://github.com/launchbadge/sqlx/issues/4148) issue).

Contributions are welcome!
