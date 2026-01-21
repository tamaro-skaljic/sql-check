//! Example of using sql-check with MySQL
//!
//! This example demonstrates how to use the check! macro to validate SQL queries
//! at compile time before using them with your database library.
//!
//! Prerequisites:
//! - MySQL database running
//! - DATABASE_URL environment variable set
//!
//! Run with:
//! ```bash
//! DATABASE_URL="mysql://user:pass@localhost/mydb" cargo run --example mysql --features mysql,_rt-tokio,_tls-rustls-ring-webpki
//! ```

use sql_check::check;

fn main() {
    println!("SQL Check - MySQL Example");
    println!("==========================\n");

    // Valid SQL queries - these will be validated at compile time
    let select_query = check!("SELECT 1");
    println!("✓ Valid SELECT query: {}", select_query);

    let select_with_column = check!("SELECT NOW()");
    println!("✓ Valid SELECT with function: {}", select_with_column);

    // More complex queries
    let create_table = check!(
        "CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    );
    println!("✓ Valid CREATE TABLE query validated");

    let insert_query = check!(
        "INSERT INTO users (name, email) VALUES (?, ?)"
    );
    println!("✓ Valid INSERT query: {}", insert_query);

    let update_query = check!(
        "UPDATE users SET name = ? WHERE id = ?"
    );
    println!("✓ Valid UPDATE query: {}", update_query);

    let delete_query = check!(
        "DELETE FROM users WHERE id = ?"
    );
    println!("✓ Valid DELETE query: {}", delete_query);

    println!("\n✅ All SQL queries validated successfully at compile time!");
    println!("   You can now use these SQL strings with any MySQL client library.");

    // Example: The following would cause a compile-time error
    // Uncomment to see the error:
    // let invalid = check!("SELECT * FROM nonexistent_table");
    // let syntax_error = check!("SELECT FROM WHERE");
}
