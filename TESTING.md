# Testing sql-check

This document explains how to run the sql-check test suite locally.

## Prerequisites

The sql-check crate supports three databases: SQLite, PostgreSQL, and MySQL. Each requires different setup.

### SQLite

No setup required. SQLite tests use an in-memory database.

### PostgreSQL

1. Install PostgreSQL (version 14 or later recommended)
2. Start the PostgreSQL server
3. Create a test database:

   ```bash
   createdb sqlcheck
   ```

### MySQL

1. Install MySQL (version 8 or later recommended)
2. Start the MySQL server
3. Create a test database:

   ```bash
   mysql -u root -p -e "CREATE DATABASE sqlcheck;"
   ```

## Running Tests

### SQLite Tests

```bash
DATABASE_URL="sqlite::memory:" cargo test --features sqlite,_rt-tokio,_tls-rustls-ring-webpki --tests
```

### PostgreSQL Tests

```bash
DATABASE_URL="postgres://username:password@localhost:5432/sqlcheck" cargo test \
  --features postgres,_rt-tokio,_tls-rustls-ring-webpki --tests
```

Replace `username` and `password` with your PostgreSQL credentials.

### MySQL Tests

```bash
DATABASE_URL="mysql://root:password@localhost:3306/sqlcheck" cargo test \
  --features mysql,_rt-tokio,_tls-rustls-ring-webpki --tests
```

Replace `root` and `password` with your MySQL credentials.

### Tests Without Validation

To run tests without compile-time SQL validation (check feature disabled):

```bash
cargo test --no-default-features --tests
```

## Compile-Fail Tests

The compile-fail tests verify that invalid SQL produces compilation errors. They use the `trybuild` crate and are located in `tests/compile_fail/`.

Each database has its own set of compile-fail tests with database-specific error messages:

- `tests/compile_fail/sqlite/` - SQLite error messages
- `tests/compile_fail/postgres/` - PostgreSQL error messages
- `tests/compile_fail/mysql/` - MySQL error messages

### Updating Expected Error Messages

When the error message format changes or when setting up for a new database, run tests with `TRYBUILD=overwrite` to capture actual error messages:

```bash
# SQLite
DATABASE_URL="sqlite::memory:" TRYBUILD=overwrite cargo test \
  --features sqlite,_rt-tokio,_tls-rustls-ring-webpki compile_fail_tests_sqlite

# PostgreSQL
DATABASE_URL="postgres://username:password@localhost:5432/sqlcheck" TRYBUILD=overwrite cargo test \
  --features postgres,_rt-tokio,_tls-rustls-ring-webpki compile_fail_tests_postgres

# MySQL
DATABASE_URL="mysql://root:password@localhost:3306/sqlcheck" TRYBUILD=overwrite cargo test \
  --features mysql,_rt-tokio,_tls-rustls-ring-webpki compile_fail_tests_mysql
```

After running with `TRYBUILD=overwrite`, review the generated `.stderr` files and commit them.

## Async Runtime Options

Replace `_rt-tokio` with one of the following to test different async runtimes:

- `_rt-tokio` - Tokio runtime
- `_rt-async-std` - async-std runtime
- `_rt-smol` - smol runtime
- `_rt-async-global-executor` - async-global-executor runtime

## TLS Options

Replace `_tls-rustls-ring-webpki` with one of the following:

- `_tls-native-tls` - Native TLS (requires OpenSSL on Linux)
- `_tls-rustls-ring-webpki` - Rustls with ring and webpki roots
- `_tls-rustls-ring-native-roots` - Rustls with ring and native roots
- `_tls-rustls-aws-lc-rs` - Rustls with AWS LC

## CI Integration

The CI workflow runs tests for each database separately:

- SQLite tests run with `sqlite::memory:` (no external dependencies)
- PostgreSQL tests run against a PostgreSQL Docker container
- MySQL tests run against a MySQL Docker container

Each database's compile-fail tests only run when that database's feature is enabled, ensuring the correct error messages are validated.
