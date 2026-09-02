//! 🔗️ Exact binding-list copy ownership with separate allocation, clone and placement turns.

use super::*;
use std::mem::{size_of, ManuallyDrop};

//#region 📋️OwnedCopy
#[derive(Default)]
struct OwnedBindings {
    source: crate::UiNodeBindings,
    candidate: crate::UiNodeBindings,
    pending: Option<ActionBinding>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiBindingsCopyProgress {
    pub complete: bool,
    pub progressed: bool,
    pub allocated_bytes: usize,
    pub copied_bytes: usize,
    pub placed_bytes: usize,
}

pub struct UiBindingsCopy {
    owned: ManuallyDrop<OwnedBindings>,
    retirement: UiTypedRetirementCursor,
    closing: bool,
    returned: u8,
}

impl UiBindingsCopy {
    pub fn new(source: crate::UiNodeBindings) -> Self {
        Self { owned: ManuallyDrop::new(OwnedBindings { source, ..Default::default() }), retirement: Default::default(), closing: false, returned: 0 }
    }

    pub fn candidate(&self) -> Option<&crate::UiNodeBindings> { (!self.closing && self.returned & 2 == 0).then_some(&self.owned.candidate) }
    pub fn source(&self) -> Option<&crate::UiNodeBindings> { (!self.closing && self.returned & 1 == 0).then_some(&self.owned.source) }
    pub fn candidate_allocated_bytes(&self) -> usize { self.owned.candidate.allocated_bytes() }
    pub fn source_allocated_bytes(&self) -> usize { self.owned.source.allocated_bytes() }

    pub fn next_allocation_bytes(&self) -> Result<usize, &'static str> {
        if self.closing { return Err("binding copy is closing"); }
        if self.owned.pending.is_some() || self.is_complete() { return Ok(0); }
        self.owned.candidate.next_allocation_bytes()
    }

    pub fn is_complete(&self) -> bool { !self.closing && (self.returned != 0 || self.owned.pending.is_none() && self.owned.source.len() == self.owned.candidate.len()) }

    pub fn advance(&mut self, items: usize, allocation_bytes: usize, copy_bytes: usize) -> Result<UiBindingsCopyProgress, UiFixedListAllocationError> {
        let rejected = |reason| UiFixedListAllocationError { allocated_bytes: 0, reason };
        if self.closing { return Err(rejected("binding copy is closing")); }
        if self.is_complete() { return Ok(UiBindingsCopyProgress { complete: true, ..Default::default() }); }
        if items == 0 { return Ok(UiBindingsCopyProgress::default()); }
        let owned = &mut *self.owned;
        if owned.pending.is_some() {
            let step = owned.candidate.try_place_reserved(&mut owned.pending, copy_bytes).map_err(rejected)?;
            return Ok(UiBindingsCopyProgress { complete: self.is_complete(), progressed: step.progressed, placed_bytes: step.placed_bytes, ..Default::default() });
        }
        if !owned.candidate.has_reserved_slot() {
            let step = owned.candidate.try_reserve_one(allocation_bytes)?;
            return Ok(UiBindingsCopyProgress { progressed: step.progressed, allocated_bytes: step.allocated_bytes, ..Default::default() });
        }
        if copy_bytes < size_of::<ActionBinding>() { return Ok(UiBindingsCopyProgress::default()); }
        let binding = owned.source.get(owned.candidate.len()).ok_or_else(|| rejected("binding source ordinal is missing"))?;
        let Some(binding) = clone_binding_one(binding).map_err(rejected)? else { return Ok(UiBindingsCopyProgress::default()); };
        owned.pending = Some(binding);
        Ok(UiBindingsCopyProgress { progressed: true, copied_bytes: size_of::<ActionBinding>(), ..Default::default() })
    }

    pub fn take_completed(&mut self) -> Option<(crate::UiNodeBindings, crate::UiNodeBindings)> {
        if self.closing || self.returned != 0 || !self.is_complete() { return None; }
        self.returned = 3;
        Some((std::mem::take(&mut self.owned.source), std::mem::take(&mut self.owned.candidate)))
    }

    /// 📤️ Transfers one completed exact root without borrowing a previous child's work grant.
    pub fn take_completed_source_with_grant(&mut self, bytes: usize) -> Option<crate::UiNodeBindings> {
        if !self.is_complete() || self.returned & 1 != 0 || bytes < size_of::<crate::UiNodeBindings>() { return None; }
        self.returned |= 1;
        Some(std::mem::take(&mut self.owned.source))
    }

    /// 📤️ Keeps the other root in this owner until its own granted transfer or typed retirement.
    pub fn take_completed_candidate_with_grant(&mut self, bytes: usize) -> Option<crate::UiNodeBindings> {
        if !self.is_complete() || self.returned & 2 != 0 || bytes < size_of::<crate::UiNodeBindings>() { return None; }
        self.returned |= 2;
        Some(std::mem::take(&mut self.owned.candidate))
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.terminal_is_empty() { return Ok(UiValueRetirementStep { complete: true, ..Default::default() }); }
        if items == 0 || bytes == 0 { return Ok(UiValueRetirementStep::default()); }
        self.closing = true;
        self.retirement.advance(&mut *self.owned, items, bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owned.source.terminal_is_empty() && self.owned.candidate.terminal_is_empty() && self.owned.pending.is_none() && (!self.closing || self.retirement.terminal_is_empty())
    }
}

impl Drop for UiBindingsCopy {
    fn drop(&mut self) {
        if !self.terminal_is_empty() && !std::thread::panicking() { panic!("binding copy requires exact source, candidate and pending retirement"); }
    }
}

impl UiTypedRetire for OwnedBindings {
    const DEPTH: usize = 1 + <crate::UiNodeBindings as UiTypedRetire>::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (field, path) = path.split_first_mut().ok_or("binding copy retirement exceeds schema depth")?;
        let mut step = match *field {
            0 => self.candidate.retire_typed(path, value, bytes)?,
            1 => self.pending.retire_typed(path, value, bytes)?,
            2 => self.source.retire_typed(path, value, bytes)?,
            _ => return Ok(UiValueRetirementStep { complete: true, progressed: true, ..Default::default() }),
        };
        if step.complete { *field += 1; path.fill(0); }
        step.complete = *field == 3;
        Ok(step)
    }
}
//#endregion 📋️OwnedCopy

//#region 🎟️AliasAdmission
fn clone_binding_one(source: &ActionBinding) -> Result<Option<ActionBinding>, &'static str> {
    let args = match source.args.as_ref() {
        None => None,
        Some(UiValue::Null) => Some(UiValue::Null),
        Some(UiValue::Bool(value)) => Some(UiValue::Bool(*value)),
        Some(UiValue::Number(value)) => Some(UiValue::Number(*value)),
        Some(UiValue::Text(value)) => Some(UiValue::Text(value.clone())),
        Some(value @ (UiValue::List(_) | UiValue::Map(_))) => {
            let mut arena = match UI_VALUE_ARENA.try_lock() {
                Ok(arena) => arena,
                Err(std::sync::TryLockError::WouldBlock) => return Ok(None),
                Err(std::sync::TryLockError::Poisoned(_)) => return Err("binding copy arena is poisoned"),
            };
            Some(arena.try_clone_value(value).ok_or("binding copy exact alias admission failed")?)
        }
    };
    Ok(Some(ActionBinding { trigger: source.trigger, action: source.action.clone(), args, capability: source.capability.clone() }))
}
//#endregion 🎟️AliasAdmission
