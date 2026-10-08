//! 🔗️ Guarded immutable projections rooted in genuine retained source ownership.

use super::{RetainedCloneBinding, RetainedCloneProjection, RetainedCloneRef};
use crate::{ValueError, ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::ManuallyDrop, ptr::NonNull, sync::Arc};

/// 🧷️ Keeps a native field stable through its immutable root lease until explicit alias closure.
pub struct RetainedOwnedProjection<T: ?Sized + Sync> {
    value: NonNull<T>,
    binding: ManuallyDrop<Option<RetainedCloneBinding>>,
    projection: RetainedCloneProjection,
}

/// 🚚️ Shared native references remain immutable and their root owner is Send + Sync.
unsafe impl<T: ?Sized + Sync> Send for RetainedOwnedProjection<T> {}
/// 🪢️ Root-backed projections expose only shared references tied to their own borrow.
unsafe impl<T: ?Sized + Sync> Sync for RetainedOwnedProjection<T> {}

impl<T: ?Sized + Sync> RetainedOwnedProjection<T> {
    /// 🛡️ Requires a projection whose lease retains its actual immutable native root.
    pub(super) unsafe fn from_source(source: RetainedCloneRef<'_, T>) -> Self {
        Self { value: NonNull::from(source.get()), binding: ManuallyDrop::new(Some(RetainedCloneBinding::new(Arc::clone(source.lease),source.projection))), projection: source.projection }
    }

    /// 📖️ Borrows the projected field for the lifetime of this actual owning alias.
    pub fn borrow(&self) -> Result<RetainedCloneRef<'_, T>, ValueError> {
        let lease = self.binding.as_ref().and_then(|binding|binding.lease.as_ref()).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "owned projection is closed"))?;
        Ok(RetainedCloneRef { value: unsafe { self.value.as_ref() }, lease, projection: self.projection })
    }

    /// 🌿️ Projects another native child without allocation or reconstructed field ownership.
    pub fn project<U: ?Sized + Sync, F>(&self, discriminator: usize, project: F) -> Result<RetainedOwnedProjection<U>, ValueError>
    where F: for<'source> FnOnce(&'source T) -> &'source U {
        Ok(unsafe { RetainedOwnedProjection::from_source(self.borrow()?.project(discriminator, project)) })
    }

    /// ♻️ Releases one alias while the source authority continues to retain the root lease.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        RetainedCloneBinding::close_one(&mut self.binding,grant)
    }
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::copy_demand(&self.binding)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{RetainedCloneBinding::capacity_demand(&self.binding,body)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::release_demand(&self.binding)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{RetainedCloneBinding::depth_demand(&self.binding)}

    /// ✅️ Reports that the projection owns neither a native alias nor a root lease.
    pub fn terminal_is_empty(&self) -> bool { self.binding.is_none() }
}

impl<T: ?Sized + Sync> Drop for RetainedOwnedProjection<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "owned projection reached Drop before exact alias closure");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.binding); } }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
