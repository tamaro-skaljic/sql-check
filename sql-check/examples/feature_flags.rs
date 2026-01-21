//! Example showing the difference between check feature enabled and disabled
//!
//! This example demonstrates how the check! macro behaves differently based
//! on whether the `check` feature is enabled or disabled.
//!
//! Run with check enabled (validates SQL):
//! ```bash
//! DATABASE_URL="sqlite::memory:" cargo run --example feature_flags --features sqlite,_rt-tokio,_tls-rustls-ring-webpki
//! ```
//!
//! Run with check disabled (no validation):
//! ```bash
//! cargo run --example feature_flags --no-default-features
//! ```

use sql_check::check;

fn main() {
    println!("SQL Check - Feature Flags Example");
    println!("==================================\n");

    #[cfg(feature = "check")]
    {
        println!("✓ The 'check' feature is ENABLED");
        println!("  SQL queries will be validated at compile time");
        println!(
            "  DATABASE_URL: {}\n",
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "NOT SET".to_string())
        );
    }

    #[cfg(not(feature = "check"))]
    {
        println!("✗ The 'check' feature is DISABLED");
        println!("  SQL queries will NOT be validated");
        println!("  The macro will just return the SQL string as-is\n");
    }

    // This SQL is valid and will work regardless of the feature flag
    let valid_sql = check!("SELECT 1");
    println!("Valid SQL: {}", valid_sql);

    #[cfg(not(feature = "check"))]
    {
        // When check feature is disabled, even invalid SQL will compile
        // because no validation is performed
        let invalid_sql = check!("THIS IS NOT VALID SQL");
        println!("Invalid SQL (no validation): {}", invalid_sql);
        println!("\n⚠️  Warning: Invalid SQL compiled because check feature is disabled");
    }

    #[cfg(feature = "check")]
    {
        println!("\n✅ SQL validated at compile time!");

        // Uncomment to see compile-time error:
        // let invalid_sql = check!("THIS IS NOT VALID SQL");
        println!("\n💡 Tip: Uncomment the invalid SQL line to see a compile-time error");
    }

    println!("\n📚 Use Cases:");
    println!("  - Development: Enable 'check' feature for validation");
    println!("  - Production: Disable 'check' feature for faster builds");
}
