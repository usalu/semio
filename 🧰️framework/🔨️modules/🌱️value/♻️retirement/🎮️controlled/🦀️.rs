//! 🎮️ Admitted typed-owner retirement with a native paged cursor frontier.

use super::{RetireOwned, RetirementCursor, RetirementStep};
use crate::{ValueError, ValueRefusalKind, list::PagedList, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;

pub struct ControlledRetirement<T: RetireOwned> {
    value: ManuallyDrop<Option<T>>,
    cursors: ManuallyDrop<PagedList<Box<dyn RetirementCursor>, {usize::MAX}>>,
}

impl<T: RetireOwned> ControlledRetirement<T> {
    /// 🎟️ Retains the input inline without constructing a retirement scaffold.
    pub fn new(value: T) -> Result<Self, (ValueError, T)> {
        if !T::controlled_retirement_supported() { return Err((ValueError::new(ValueRefusalKind::UnsupportedOwner, "typed owner has no controlled retirement authority"), value)); }
        Ok(Self { value: ManuallyDrop::new(Some(value)), cursors: ManuallyDrop::new(PagedList::default()) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.value.is_none() && self.cursors.terminal_is_empty() }

    /// 🧮️ Borrows the next minimum payload-work grant without releasing its physical backing.
    pub fn next_copy_byte_demand(&self) -> usize {
        if self.value.is_some() || self.cursors.is_empty() || !self.cursors.has_reserved_slot() { return 0; }
        let cursor = self.cursors.get(self.cursors.len() - 1).unwrap();
        if cursor.terminal_is_empty() { 0 } else { cursor.next_work_byte_demand() }
    }

    /// 📏️ Exposes the next complete constructor allocation before the caller grants it.
    pub fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> {
        if self.terminal_is_empty() || (self.value.is_none() && (self.cursors.is_empty() || self.cursors.get(self.cursors.len() - 1).is_some_and(|cursor| cursor.terminal_is_empty()))) { return Ok(0); }
        if !self.cursors.has_reserved_slot() { return self.cursors.next_capacity_allocation_bytes(self.cursors.len() + 1).map_err(ValueError::from)?.ok_or_else(|| refusal("retirement frontier lost its capacity demand")); }
        let demand = if let Some(value) = self.value.as_ref() { value.retirement_birth_bytes() } else { self.cursors.get(self.cursors.len() - 1).and_then(|cursor| cursor.next_birth_bytes(maximum_body_bytes)) };
        demand.ok_or_else(|| refusal("retirement owner has no controlled birth authority"))
    }

    /// 📏️ Exposes the next complete physical release before the caller grants it.
    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        if self.value.is_some() || self.terminal_is_empty() { return Ok(0); }
        if self.cursors.is_empty() { return self.cursors.next_release_allocation_bytes().map_err(ValueError::from); }
        let cursor = self.cursors.get(self.cursors.len() - 1).unwrap();
        if cursor.terminal_is_empty() { cursor.terminal_release_bytes().ok_or_else(|| refusal("retirement owner has no controlled terminal release authority")) }
        else if cursor.next_work_byte_demand() != 0 { Ok(0) }
        else { Ok(cursor.next_close_byte_demand().unwrap_or(0)) }
    }

    pub fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.value.is_none() && self.cursors.is_empty() {
            let step = self.cursors.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(step.progressed), copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: step.released_allocation_bytes }));
        }
        if self.value.is_none() && self.cursors.get(self.cursors.len() - 1).is_some_and(|cursor| cursor.terminal_is_empty()) {
            let bytes = self.cursors.get(self.cursors.len() - 1).and_then(|cursor| cursor.terminal_release_bytes()).ok_or_else(|| refusal("retirement terminal scaffold has no controlled release authority"))?;
            if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            drop(self.cursors.pop());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }));
        }
        if !self.cursors.has_reserved_slot() {
            let demand = self.cursors.next_capacity_allocation_bytes(self.cursors.len() + 1).map_err(ValueError::from)?.ok_or_else(|| refusal("retirement frontier lost its capacity demand"))?;
            if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let step = self.cursors.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(step.progressed), copied_bytes: 0, retained_capacity_bytes: step.allocated_bytes, released_bytes: 0 }));
        }
        if let Some(value) = self.value.as_ref() {
            let bytes = value.retirement_birth_bytes().ok_or_else(|| refusal("owned value has no controlled retirement birth authority"))?;
            if bytes > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let cursor = self.value.take().unwrap().retirement();
            self.cursors.push_reserved(cursor).map_err(|_| refusal("retirement root lost its admitted frontier slot"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: bytes, released_bytes: 0 }));
        }
        if self.cursors.len() >= grant.maximum_depth { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "controlled retirement frontier exceeds its admitted depth")); }
        let index = self.cursors.len() - 1;
        let minimum_work_bytes = self.cursors.get(index).unwrap().next_work_byte_demand();
        let work = minimum_work_bytes != 0;
        let body_bytes = if work { grant.maximum_copy_bytes } else { grant.maximum_release_bytes };
        if work && body_bytes < minimum_work_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let birth = self.cursors.get(index).and_then(|cursor| cursor.next_birth_bytes(body_bytes)).ok_or_else(|| refusal("retirement child has no controlled birth authority"))?;
        if birth > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let (processed, bytes, progressed) = match self.cursors.get_mut(index).unwrap().close_step(body_bytes) {
            RetirementStep::Child(child) => { self.cursors.push_reserved(child).map_err(|_| refusal("retirement child lost its admitted frontier slot"))?; (0, 0, true) }
            RetirementStep::Bytes(bytes) if bytes <= grant.maximum_release_bytes => (0, bytes, true),
            RetirementStep::Bytes(_) => return Err(refusal("retirement body exceeded its admitted release grant")),
            RetirementStep::ProcessedBytes(bytes) if bytes <= grant.maximum_copy_bytes => (bytes, 0, true),
            RetirementStep::ProcessedBytes(_) => return Err(refusal("retirement body exceeded its admitted work grant")),
            RetirementStep::Complete if self.cursors.get(index).unwrap().terminal_is_empty() => (0, 0, true),
            RetirementStep::Complete => return Err(refusal("retirement body has no terminal-empty witness")),
            RetirementStep::BudgetExhausted => (0, 0, false),
        };
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progressed), copied_bytes: processed, retained_capacity_bytes: if progressed { birth } else { 0 }, released_bytes: bytes }))
    }
}

impl<T: RetireOwned> Drop for ControlledRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "controlled retirement abandoned before terminal-empty");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.value); ManuallyDrop::drop(&mut self.cursors); } }
    }
}

fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }
