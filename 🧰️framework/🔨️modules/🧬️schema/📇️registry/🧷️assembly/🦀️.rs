//! 🧷️ One process-wide publication barrier shared by every registry.
use std::sync::{Mutex, MutexGuard, OnceLock, TryLockError};

/// 🔐️ Holds the exclusive registration phase until it is dropped.
pub struct Transaction {
    _guard: MutexGuard<'static, ()>,
}

/// 🚫️ Refuses a publication barrier poisoned by a writer panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("registry assembly transaction unavailable")
    }
}

impl std::error::Error for Error {}

fn lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// 🧱️ Begins the exclusive phase accepted by registration APIs.
#[must_use]
pub fn begin() -> Result<Transaction, Error> {
    Ok(Transaction { _guard: lock().lock().map_err(|_| Error::Unavailable)? })
}

/// 🚦️ Attempts the exclusive phase without blocking an interactive caller.
#[must_use]
pub fn try_begin() -> Result<Option<Transaction>, Error> {
    match lock().try_lock() {
        Ok(guard) => Ok(Some(Transaction { _guard: guard })),
        Err(TryLockError::WouldBlock) => Ok(None),
        Err(TryLockError::Poisoned(_)) => Err(Error::Unavailable),
    }
}
