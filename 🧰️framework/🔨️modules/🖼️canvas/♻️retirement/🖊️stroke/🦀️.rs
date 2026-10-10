//! 🖊️ Supplied Stroke retirement preserving the original dash buffer through canonical typed control.
use crate::Stroke;
use std::mem::size_of;
use semio_framework_value::{ErasedSnapshotRetirement, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;

pub struct CanvasStrokeRetirement {
    original: Option<Stroke>,
    active: Option<ControlledRetirement<Vec<f64>>>,
}

impl CanvasStrokeRetirement {
    /// 📐️ Quotes the leaf frame and original inline Stroke ownership transfer.
    pub fn birth_demand() -> RetirementDemand {
        RetirementDemand { copy_bytes: 2 * size_of::<Stroke>(), capacity_bytes: size_of::<Self>(), release_bytes: 0, depth: 1 }
    }
    /// 🎟️ Allocates only after all supplied birth axes admit the exact original owner.
    pub fn admit(original: Stroke, grant: RetainedCloneGrant) -> Result<(Box<Self>, RetainedCloneProgress), (ValueError, Stroke)> {
        let demand = Self::birth_demand();
        let refusal = if grant.maximum_items == 0 { Some(ValueError::literal(ValueRefusalKind::WorkLimit, "Canvas Stroke birth requires supplied work")) }
            else if grant.maximum_depth < demand.depth { Some(ValueError::literal(ValueRefusalKind::DepthLimit, "Canvas Stroke birth exceeds supplied depth")) }
            else if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes { Some(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Canvas Stroke birth exceeds supplied physical grant")) }
            else { None };
        if let Some(error) = refusal { return Err((error, original)); }
        let layout = std::alloc::Layout::new::<Self>();
        let Some(pointer) = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<Self>()) else {
            return Err((ValueError::literal(ValueRefusalKind::AllocationFailed, "Canvas Stroke cursor allocation failed"), original));
        };
        unsafe { pointer.as_ptr().write(Self { original: Some(original), active: None }); }
        let owner = unsafe { Box::from_raw(pointer.as_ptr()) };
        Ok((owner, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: 0 }))
    }
    /// 🔎️ Borrows the retained scalar style and its dash buffer before controlled transfer.
    pub fn original_stroke(&self) -> Option<&Stroke> { self.original.as_ref() }
    /// 👁️ Borrows a transferred original while the canonical cursor has not consumed it.
    pub fn controlled_original(&self) -> Option<&Vec<f64>> { self.active.as_ref().and_then(ControlledRetirement::original) }
    fn needs_dash_transfer(&self) -> bool { self.active.is_none() && self.original.as_ref().is_some_and(|original| original.dash_capacity() != 0) }
}

impl ErasedSnapshotRetirement for CanvasStrokeRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.next_depth_demand()? { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Canvas Stroke retirement exceeds supplied depth")); }
        let copy = self.next_copy_byte_demand()?;
        if grant.maximum_copy_bytes < copy || grant.maximum_capacity_bytes < self.next_capacity_byte_demand(grant.maximum_copy_bytes)? || grant.maximum_release_bytes < self.next_release_byte_demand()? { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.needs_dash_transfer() {
            let original = self.original.as_mut().unwrap();
            let value = original.take_dash_pattern();
            match ControlledRetirement::new(value) {
                Ok(active) => self.active = Some(active),
                Err((error, value)) => { original.set_dash_pattern(value); return Err(error); }
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..empty }));
        }
        if let Some(active) = self.active.as_mut().filter(|active| !active.terminal_is_empty()) {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return active.step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        drop(self.active.take());
        drop(self.original.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.original.is_none() && self.active.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if self.terminal_is_empty() { return Ok(0); }
        if self.needs_dash_transfer() { return Ok(2 * size_of::<Vec<f64>>()); }
        if let Some(active) = self.active.as_ref().filter(|active| !active.terminal_is_empty()) { return active.next_copy_byte_demand(); }
        Ok(2 * size_of::<Self>())
    }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        if let Some(active) = self.active.as_ref().filter(|active| !active.terminal_is_empty()) { return active.next_capacity_byte_demand(body); }
        Ok(0)
    }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(active) = self.active.as_ref().filter(|active| !active.terminal_is_empty()) { return active.next_release_byte_demand(); }
        Ok(0)
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        if self.terminal_is_empty() { return Ok(0); }
        if let Some(active) = self.active.as_ref().filter(|active| !active.terminal_is_empty()) {
            return active.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "Canvas Stroke child depth overflow"));
        }
        Ok(if self.needs_dash_transfer() { 2 } else { 1 })
    }
}

impl Drop for CanvasStrokeRetirement {
    fn drop(&mut self) { assert!(self.terminal_is_empty(), "Canvas Stroke cursor dropped before funded original closure"); }
}

#[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
