//! ➰️ Retains original first-party geometry storage under supplied physical retirement authority.
use crate::{BezPath, PathEl};
use std::mem::{size_of, ManuallyDrop};
use semio_framework_value::{ErasedSnapshotRetirement, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};

pub struct CanvasPathRetirement {
    elements: ManuallyDrop<Vec<PathEl>>,
}

impl CanvasPathRetirement {
    /// 📐️ Quotes only the actual cursor frame and original vector-header transfer.
    pub fn birth_demand() -> RetirementDemand {
        RetirementDemand { copy_bytes: 2 * size_of::<Vec<PathEl>>(), capacity_bytes: size_of::<Self>(), release_bytes: 0, depth: 1 }
    }

    /// 🎟️ Keeps the exact original path on refusal and admits its cursor before transferring storage.
    pub fn admit(original: BezPath, grant: RetainedCloneGrant) -> Result<(Box<Self>, RetainedCloneProgress), (ValueError, BezPath)> {
        let demand = Self::birth_demand();
        let refusal = if grant.maximum_items == 0 { Some(ValueError::literal(ValueRefusalKind::WorkLimit, "Canvas path birth requires supplied work")) }
            else if grant.maximum_depth < demand.depth { Some(ValueError::literal(ValueRefusalKind::DepthLimit, "Canvas path birth exceeds supplied depth")) }
            else if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes { Some(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Canvas path birth exceeds supplied physical grant")) }
            else { None };
        if let Some(error) = refusal { return Err((error, original)); }
        let layout = std::alloc::Layout::new::<Self>();
        let Some(pointer) = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<Self>()) else {
            return Err((ValueError::literal(ValueRefusalKind::AllocationFailed, "Canvas path cursor allocation failed"), original));
        };
        unsafe { pointer.as_ptr().write(Self { elements: ManuallyDrop::new(original.into_elements()) }); }
        let owner = unsafe { Box::from_raw(pointer.as_ptr()) };
        Ok((owner, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: 0 }))
    }

    /// 🔎️ Borrows the still-retained original elements without copying or transferring them.
    pub fn original_elements(&self) -> &[PathEl] { &self.elements }

    /// 📦️ Observes actual vector capacity while its allocation remains retained.
    pub fn original_capacity(&self) -> usize { self.elements.capacity() }
}

impl ErasedSnapshotRetirement for CanvasPathRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Canvas path retirement requires supplied depth")); }
        if !self.elements.is_empty() {
            let copy = size_of::<PathEl>();
            if grant.maximum_copy_bytes < copy { return Ok(RetainedCloneStep::Progress(empty)); }
            self.elements.pop();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..empty }));
        }
        let release = self.next_release_byte_demand()?;
        let copy = self.next_copy_byte_demand()?;
        if grant.maximum_release_bytes < release || grant.maximum_copy_bytes < copy { return Ok(RetainedCloneStep::Progress(empty)); }
        drop(std::mem::take(&mut *self.elements));
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, released_bytes: release, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool { self.elements.is_empty() && self.elements.capacity() == 0 }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(if !self.elements.is_empty() { size_of::<PathEl>() } else if self.elements.capacity() != 0 { 2 * size_of::<Vec<PathEl>>() } else { 0 }) }
    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.elements.is_empty() { return Ok(0); }
        self.elements.capacity().checked_mul(size_of::<PathEl>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "Canvas path backing extent overflow"))
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(!self.terminal_is_empty())) }
}

impl Drop for CanvasPathRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Canvas path cursor dropped before funded original backing release");
        unsafe { ManuallyDrop::drop(&mut self.elements); }
    }
}

#[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
