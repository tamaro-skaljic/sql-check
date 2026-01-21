//! Runtime abstraction for blocking on async operations

use cfg_if::cfg_if;

pub fn block_on<F>(f: F) -> F::Output
where
    F: std::future::Future,
{
    cfg_if! {
        if #[cfg(feature = "_rt-async-global-executor")] {
            sqlx_core::rt::test_block_on(f)
        } else if #[cfg(feature = "_rt-async-std")] {
            async_std::task::block_on(f)
        } else if #[cfg(feature = "_rt-smol")] {
            sqlx_core::rt::test_block_on(f)
        } else if #[cfg(feature = "_rt-tokio")] {
            use std::sync::LazyLock;
            use tokio::runtime::{self, Runtime};

            // We need a single, persistent Tokio runtime since we're caching connections,
            // otherwise we'll get "IO driver has terminated" errors.
            static TOKIO_RT: LazyLock<Runtime> = LazyLock::new(|| {
                runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("failed to start Tokio runtime")
            });

            TOKIO_RT.block_on(f)
        } else {
            sqlx_core::rt::missing_rt(f)
        }
    }
}
