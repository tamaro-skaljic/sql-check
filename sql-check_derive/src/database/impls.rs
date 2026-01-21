//! Database-specific implementations of DatabaseExt

macro_rules! impl_database_ext {
    (
        $database:path,
        $(describe-blocking: $describe:path,)?
    ) => {
        impl $crate::database::DatabaseExt for $database {
            impl_describe_blocking!($database, $($describe)?);
        }
    }
}

macro_rules! impl_describe_blocking {
    ($database:path $(,)?) => {
        fn describe_blocking(
            query: &str,
            database_url: &str,
            driver_config: &sqlx_core::config::drivers::Config,
        ) -> sqlx_core::Result<sqlx_core::describe::Describe<Self>> {
            use $crate::database::CachingDescribeBlocking;

            // This can't be a provided method because the `static` can't reference `Self`.
            static CACHE: CachingDescribeBlocking<$database> = CachingDescribeBlocking::new();

            CACHE.describe(query, database_url, driver_config)
        }
    };
    ($database:path, $describe:path) => {
        fn describe_blocking(
            query: &str,
            database_url: &str,
            driver_config: &sqlx_core::config::drivers::Config,
        ) -> sqlx_core::Result<sqlx_core::describe::Describe<Self>> {
            $describe(query, database_url, driver_config)
        }
    };
}

#[cfg(feature = "mysql")]
impl_database_ext! {
    sqlx_mysql::MySql,
}

#[cfg(feature = "postgres")]
impl_database_ext! {
    sqlx_postgres::Postgres,
}

#[cfg(feature = "sqlite")]
impl_database_ext! {
    sqlx_sqlite::Sqlite,
    describe-blocking: sqlx_sqlite::describe_blocking,
}
