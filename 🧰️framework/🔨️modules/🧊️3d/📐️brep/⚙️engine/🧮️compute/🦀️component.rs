// #region compute
//! ⚙️ Block the current thread until an async kernel call completes.

use std::future::Future;

/// ⏳️ Block the current thread until an async kernel call completes.
pub fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    pollster::block_on(future)
}
// #endregion compute
