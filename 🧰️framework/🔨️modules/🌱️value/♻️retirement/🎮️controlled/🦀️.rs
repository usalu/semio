//! 🎮️ Admitted typed-owner retirement with a native paged cursor frontier.

use super::{RetireOwned, RetirementCursor, RetirementStep};
use crate::{ValueError, ValueRefusalKind, list::PagedList, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;

/// 🚦️ Keeps retained-owner exclusion inline and refuses contention without parking or allocating.
pub struct RetainedOwnerGate<T> { occupied:std::sync::atomic::AtomicBool,value:std::cell::UnsafeCell<T> }
unsafe impl<T:Send> Send for RetainedOwnerGate<T> {}
unsafe impl<T:Send> Sync for RetainedOwnerGate<T> {}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct RetainedOwnerGateBusy;
pub struct RetainedOwnerGateGuard<'a,T> {gate:&'a RetainedOwnerGate<T>,exclusive:std::marker::PhantomData<&'a mut T>}
impl<T> RetainedOwnerGate<T> {
    pub const fn new(value:T)->Self {Self{occupied:std::sync::atomic::AtomicBool::new(false),value:std::cell::UnsafeCell::new(value)}}
    pub fn try_lock(&self)->Result<RetainedOwnerGateGuard<'_,T>,RetainedOwnerGateBusy> {
        self.occupied.compare_exchange(false,true,std::sync::atomic::Ordering::Acquire,std::sync::atomic::Ordering::Relaxed).map(|_|RetainedOwnerGateGuard{gate:self,exclusive:std::marker::PhantomData}).map_err(|_|RetainedOwnerGateBusy)
    }
    pub fn get_mut(&mut self)->&mut T {self.value.get_mut()}
    pub fn into_inner(self)->T {self.value.into_inner()}
}
impl<T> std::ops::Deref for RetainedOwnerGateGuard<'_,T> {type Target=T;fn deref(&self)->&T {unsafe{&*self.gate.value.get()}}}
impl<T> std::ops::DerefMut for RetainedOwnerGateGuard<'_,T> {fn deref_mut(&mut self)->&mut T {unsafe{&mut*self.gate.value.get()}}}
impl<T> Drop for RetainedOwnerGateGuard<'_,T> {fn drop(&mut self){self.gate.occupied.store(false,std::sync::atomic::Ordering::Release)}}

/// 🎮️ Erases only the owner type while preserving each independent retirement authority.
pub trait ErasedControlledRetirement: Send {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn step_progress(&self) -> RetainedCloneProgress;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn frame_release_bytes(&self) -> usize;
}

/// 📏️ Borrows the exact frame allocation before original typed ownership transfers.
pub const fn controlled_retirement_birth_bytes<T: RetireOwned>() -> usize { size_of::<ControlledRetirement<T>>() }

/// 🎟️ Admits the exact typed frame before allocation and returns original ownership on refusal.
pub fn admit_controlled_retirement<T: RetireOwned>(value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedControlledRetirement>, RetainedCloneProgress), (ValueError, T)> {
    admit_typed_controlled_retirement(value, grant).map(|(owner, progress)| (owner as Box<dyn ErasedControlledRetirement>, progress))
}

pub fn admit_typed_controlled_retirement<T: RetireOwned>(value: T, grant: RetainedCloneGrant) -> Result<(Box<ControlledRetirement<T>>, RetainedCloneProgress), (ValueError, T)> {
    if !T::controlled_retirement_supported() { return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner, "typed owner has no controlled retirement authority"), value)); }
    if grant.maximum_items == 0 { return Err((ValueError::literal(ValueRefusalKind::WorkLimit, "typed retirement frame requires one admitted item"), value)); }
    if grant.maximum_depth == 0 { return Err((ValueError::literal(ValueRefusalKind::DepthLimit, "typed retirement frame requires admitted depth"), value)); }
    let layout = std::alloc::Layout::new::<ControlledRetirement<T>>();
    if layout.size() > grant.maximum_capacity_bytes { return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "typed retirement frame exceeds admitted capacity"), value)); }
    let Some(pointer) = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<ControlledRetirement<T>>()) else { return Err((ValueError::literal(ValueRefusalKind::AllocationFailed, "typed retirement frame allocation failed"), value)); };
    let owner = match ControlledRetirement::new(value) { Ok(owner) => owner, Err((error, value)) => { unsafe { std::alloc::dealloc(pointer.as_ptr().cast(), layout) }; return Err((error, value)); } };
    let owner = unsafe { pointer.as_ptr().write(owner); Box::from_raw(pointer.as_ptr()) };
    Ok((owner, RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: layout.size(), released_bytes: 0 }))
}

impl<T: RetireOwned> crate::ErasedSnapshotRetirement for ControlledRetirement<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.step(grant) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Self::next_copy_byte_demand(self) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Self::next_capacity_byte_demand(self, copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Self::next_depth_demand(self) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
}

pub struct ControlledRetirement<T: RetireOwned> {
    value: ManuallyDrop<Option<T>>,
    cursors: ManuallyDrop<PagedList<Box<dyn RetirementCursor>, {usize::MAX}>>,
    step_progress: RetainedCloneProgress,
}

impl<T: RetireOwned> ControlledRetirement<T> {
    /// 🎟️ Retains the input inline without constructing a retirement scaffold.
    pub fn new(value: T) -> Result<Self, (ValueError, T)> {
        if !T::controlled_retirement_supported() { return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner, "typed owner has no controlled retirement authority"), value)); }
        Ok(Self { value: ManuallyDrop::new(Some(value)), cursors: ManuallyDrop::new(PagedList::default()), step_progress: RetainedCloneProgress::default() })
    }

    /// 👁️ Borrows the original typed owner before retirement cursor construction.
    pub fn original(&self) -> Option<&T> { self.value.as_ref() }
    /// 🔒️ Quotes whether the original can transfer before any retirement scaffold is retained.
    pub fn original_is_untouched(&self) -> bool { self.value.is_some() && self.cursors.terminal_is_empty() }
    /// ✍️ Mutates the original typed owner before retirement cursor construction.
    pub fn original_mut(&mut self) -> Option<&mut T> { self.value.as_mut() }
    /// 🎁️ Transfers an untouched original while retaining its empty admitted frame.
    pub fn take_original(&mut self) -> Option<T> { if self.cursors.terminal_is_empty() { self.value.take() } else { None } }

    pub fn terminal_is_empty(&self) -> bool { self.value.is_none() && self.cursors.terminal_is_empty() }

    /// 🧮️ Borrows the next minimum payload-work grant without releasing its physical backing.
    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if self.value.is_some() { return Ok(0); }
        if self.cursors.is_empty() || !self.cursors.has_reserved_slot() { return Ok(0); }
        let cursor = self.cursors.get(self.cursors.len() - 1).unwrap();
        if cursor.terminal_is_empty() { Ok(0) } else { cursor.next_work_byte_demand() }
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
        else if !self.cursors.has_reserved_slot() { Ok(0) }
        else { cursor.next_close_byte_demand().ok_or_else(||refusal("retirement owner has no controlled physical release demand")) }
    }

    /// 🪆️ Borrows the current frontier and nested owner depth before any capacity is admitted.
    pub fn next_depth_demand(&self) -> Result<usize, ValueError> {
        if self.terminal_is_empty() { return Ok(0); }
        if self.value.is_some() || self.cursors.is_empty() { return Ok(1); }
        let cursor = self.cursors.get(self.cursors.len() - 1).unwrap();
        let nested = if cursor.terminal_is_empty() { 0 } else { cursor.next_depth_demand()? };
        self.cursors.len().checked_add(nested).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "retirement frontier depth overflow"))
    }

    pub fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.step_progress = RetainedCloneProgress::default();
        match self.step_original(grant) {
            Ok(step) => { self.step_progress = step.progress(); Ok(step) }
            Err(error) => {
                if error.retained_progress() != RetainedCloneProgress::default() { self.step_progress = error.retained_progress(); }
                Err(error.with_retained_progress(self.step_progress))
            }
        }
    }

    /// 🧾️ The physical receipt of the last original turn, including a retained allocation on failure.
    pub fn step_progress(&self) -> RetainedCloneProgress { self.step_progress }

    fn reserve_cursor_slot(&mut self, grant: usize, reserve: impl FnOnce(&mut PagedList<Box<dyn RetirementCursor>, {usize::MAX}>, usize) -> Result<crate::list::PagedListProgress, crate::list::PagedListAllocationError>) -> Result<RetainedCloneStep, ValueError> {
        match reserve(&mut self.cursors, grant) {
            Ok(step) => {
                self.step_progress = RetainedCloneProgress { copied_items: usize::from(step.progressed), retained_capacity_bytes: step.allocated_bytes, ..Default::default() };
                Ok(RetainedCloneStep::Progress(self.step_progress))
            }
            Err(error) => {
                self.step_progress = RetainedCloneProgress { copied_items: usize::from(error.allocated_bytes != 0), retained_capacity_bytes: error.allocated_bytes, ..Default::default() };
                Err(ValueError::from(error.refusal()).with_retained_progress(self.step_progress))
            }
        }
    }

    fn step_original(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.next_depth_demand()? { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "controlled retirement frontier exceeds its admitted depth")); }
        if grant.maximum_capacity_bytes < self.next_capacity_byte_demand(grant.maximum_copy_bytes)? || grant.maximum_release_bytes < self.next_release_byte_demand()? { return Ok(RetainedCloneStep::Progress(empty)); }
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
            return self.reserve_cursor_slot(grant.maximum_capacity_bytes, PagedList::reserve_one);
        }
        if let Some(value) = self.value.as_ref() {
            let bytes = value.retirement_birth_bytes().ok_or_else(|| refusal("owned value has no controlled retirement birth authority"))?;
            if bytes > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            let cursor = self.value.take().unwrap().retirement();
            self.cursors.push_reserved(cursor).map_err(|_| refusal("retirement root lost its admitted frontier slot"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: bytes, released_bytes: 0 }));
        }
        let index = self.cursors.len() - 1;
        let minimum_work_bytes = self.cursors.get(index).unwrap().next_work_byte_demand()?;
        let work = minimum_work_bytes != 0;
        let body_bytes = grant.maximum_copy_bytes;
        let birth = self.cursors.get(index).and_then(|cursor| cursor.next_birth_bytes(body_bytes)).ok_or_else(|| refusal("retirement child has no controlled birth authority"))?;
        if work && body_bytes < minimum_work_bytes && (birth==0 || !self.cursors.get(index).unwrap().allows_admitted_narrow_work()) { return Ok(RetainedCloneStep::Progress(empty)); }
        if birth > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let cursor_grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: birth, maximum_depth: grant.maximum_depth - self.cursors.len(), ..grant };
        let (processed, bytes, progressed) = match self.cursors.get_mut(index).unwrap().close_step(cursor_grant) {
            RetirementStep::Progress(progress) => {
                self.step_progress = progress;
                if !progress.fits(cursor_grant) || (progress.copied_items!=0 && progress.retained_capacity_bytes!=birth) { return Err(refusal("typed retirement receipt exceeded or changed its declared full grant")); }
                return Ok(RetainedCloneStep::Progress(progress));
            }
            RetirementStep::Child(child) => { self.cursors.push_reserved(child).map_err(|_| refusal("retirement child lost its admitted frontier slot"))?; (0, 0, true) }
            RetirementStep::Advanced => (0, 0, true),
            RetirementStep::Failure(error) => return Err(error),
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

impl<T: RetireOwned> ErasedControlledRetirement for ControlledRetirement<T> {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { ControlledRetirement::step(self, grant) }
    fn step_progress(&self) -> RetainedCloneProgress { ControlledRetirement::step_progress(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_copy_byte_demand(self) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { ControlledRetirement::next_capacity_byte_demand(self, maximum_body_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_release_byte_demand(self) }
    fn terminal_is_empty(&self) -> bool { ControlledRetirement::terminal_is_empty(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_depth_demand(self) }
    fn frame_release_bytes(&self) -> usize { size_of::<Self>() }
}

fn refusal(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }

impl<T: RetireOwned> RetireOwned for ControlledRetirement<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(self) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

impl<T: RetireOwned> RetirementCursor for ControlledRetirement<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        match self.step(grant) {
            Err(error) => RetirementStep::Failure(error),
            Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,
            Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress),
        }
    }
    fn terminal_is_empty(&self) -> bool { ControlledRetirement::terminal_is_empty(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_depth_demand(self) }
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {self.next_copy_byte_demand()}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_close_byte_demand(&self) -> Option<usize> { self.next_release_byte_demand().ok() }
    fn next_birth_bytes(&self, maximum_bytes: usize) -> Option<usize> { self.next_capacity_byte_demand(maximum_bytes).ok() }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
