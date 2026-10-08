//! 🔗️ Guarded immutable projections rooted in genuine retained source ownership.

use super::{RetainedCloneLeaseOwner, RetainedCloneProjection, RetainedCloneRef};
use crate::{SnapshotRetirementStep, ValueError, ValueRefusalKind};
use std::{mem::ManuallyDrop, ptr::NonNull, sync::Arc};

/// 🧷️ Keeps a native field stable through its immutable root lease until explicit alias closure.
pub struct RetainedOwnedProjection<T: ?Sized + Sync> {
    value: NonNull<T>,
    lease: ManuallyDrop<Option<Arc<RetainedCloneLeaseOwner>>>,
    projection: RetainedCloneProjection,
}

/// 🚚️ Shared native references remain immutable and their root owner is Send + Sync.
unsafe impl<T: ?Sized + Sync> Send for RetainedOwnedProjection<T> {}
/// 🪢️ Root-backed projections expose only shared references tied to their own borrow.
unsafe impl<T: ?Sized + Sync> Sync for RetainedOwnedProjection<T> {}

impl<T: ?Sized + Sync> RetainedOwnedProjection<T> {
    /// 🛡️ Requires a projection whose lease retains its actual immutable native root.
    pub(super) unsafe fn from_source(source: RetainedCloneRef<'_, T>) -> Self {
        Self { value: NonNull::from(source.get()), lease: ManuallyDrop::new(Some(Arc::clone(source.lease))), projection: source.projection }
    }

    /// 📖️ Borrows the projected field for the lifetime of this actual owning alias.
    pub fn borrow(&self) -> Result<RetainedCloneRef<'_, T>, ValueError> {
        let lease = self.lease.as_ref().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "owned projection is closed"))?;
        Ok(RetainedCloneRef { value: unsafe { self.value.as_ref() }, lease, projection: self.projection })
    }

    /// 🌿️ Projects another native child without allocation or reconstructed field ownership.
    pub fn project<U: ?Sized + Sync, F>(&self, discriminator: usize, project: F) -> Result<RetainedOwnedProjection<U>, ValueError>
    where F: for<'source> FnOnce(&'source T) -> &'source U {
        Ok(unsafe { RetainedOwnedProjection::from_source(self.borrow()?.project(discriminator, project)) })
    }

    /// ♻️ Releases one alias while the source authority continues to retain the root lease.
    pub fn close_step(&mut self, maximum_items: usize) -> Result<SnapshotRetirementStep, ValueError> {
        let Some(lease) = self.lease.as_ref() else { return Ok(SnapshotRetirementStep::Complete); };
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if Arc::strong_count(lease) <= 1 { return Ok(SnapshotRetirementStep::Blocked); }
        drop(self.lease.take());
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    /// ✅️ Reports that the projection owns neither a native alias nor a root lease.
    pub fn terminal_is_empty(&self) -> bool { self.lease.is_none() }
}

impl<T: ?Sized + Sync> Drop for RetainedOwnedProjection<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "owned projection reached Drop before exact alias closure");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.lease); } }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
