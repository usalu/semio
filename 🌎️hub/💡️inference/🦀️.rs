//! 🧭️ Hub-owned private inference lifecycle; no client snapshot or global registry authority.

#[cfg(feature = "native-artifact-execution")]
#[path = "📇️catalog/🦀️.rs"]
pub(crate) mod catalog;
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
#[cfg(feature = "sqlite")]
#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;
#[path = "🧾️wal/🦀️.rs"]
pub mod wal;
#[cfg(feature = "native-artifact-execution")]
pub use catalog::{verified_gis_map_binding, VerifiedGisMapArtifactBindingV1};
#[path = "🛂️authorization/🦀️.rs"]
pub(crate) mod authorization;
#[path = "✉️command/🦀️.rs"]
pub(crate) mod command;
#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
#[path = "🏃️runtime/🦀️.rs"]
pub mod runtime;

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceErrorV1 {
    Invalid,
    Bounds,
    Denied,
    Conflict,
    Capacity,
    Expired,
    Cancelled,
    Storage,
}

pub fn sha256(bytes: &[u8]) -> String {
    semio_framework_hash::Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

pub struct InferenceOperationControlV1 {
    deadline: Instant,
    cancelled: AtomicBool,
    interrupted: tokio::sync::Notify,
    progress: AtomicU64,
    work_limit: u64,
}

impl InferenceOperationControlV1 {
    pub fn new(lifetime_ms: u64, work_limit: u64) -> Result<Self, InferenceErrorV1> {
        if lifetime_ms == 0 || lifetime_ms > schema::JOB_MAX_LIFETIME_MS || work_limit == 0 || work_limit > 65_536 {
            return Err(InferenceErrorV1::Bounds);
        }
        let deadline = Instant::now().checked_add(Duration::from_millis(lifetime_ms)).ok_or(InferenceErrorV1::Bounds)?;
        Ok(Self { deadline, cancelled: AtomicBool::new(false), interrupted: tokio::sync::Notify::new(), progress: AtomicU64::new(0), work_limit })
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
        self.interrupted.notify_waiters();
    }

    pub fn checkpoint(&self, completed: u64) -> Result<(), InferenceErrorV1> {
        if self.cancelled.load(Ordering::Acquire) {
            return Err(InferenceErrorV1::Cancelled);
        }
        if Instant::now() >= self.deadline {
            return Err(InferenceErrorV1::Expired);
        }
        if completed > self.work_limit {
            return Err(InferenceErrorV1::Bounds);
        }
        self.progress.fetch_max(completed, Ordering::AcqRel);
        Ok(())
    }

    pub fn progress(&self) -> (u64, u64) {
        (self.progress.load(Ordering::Acquire), self.work_limit)
    }

    pub(super) async fn interruption(&self) -> InferenceErrorV1 {
        let notified = self.interrupted.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        if let Err(error) = self.checkpoint(self.progress().0) {
            return error;
        }
        tokio::select! {
            _ = &mut notified => InferenceErrorV1::Cancelled,
            _ = tokio::time::sleep_until(tokio::time::Instant::from_std(self.deadline)) => InferenceErrorV1::Expired,
        }
    }
}

pub struct InferencePrivateBytesV1(Vec<u8>);

impl InferencePrivateBytesV1 {
    pub fn new(bytes: Vec<u8>, maximum: usize) -> Result<Self, InferenceErrorV1> {
        let value = Self(bytes);
        if value.0.len() > maximum {
            return Err(InferenceErrorV1::Bounds);
        }
        Ok(value)
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for InferencePrivateBytesV1 {
    fn drop(&mut self) {
        let pointer = self.0.as_mut_ptr();
        for index in 0..self.0.capacity() {
            unsafe {
                std::ptr::write_volatile(pointer.add(index), 0);
            }
        }
        std::sync::atomic::compiler_fence(Ordering::SeqCst);
    }
}
