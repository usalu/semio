//! 📋️ Actual bounded component source/candidate copy execution.
use crate::*;
use semio_framework_ui_contract_values::copy_protocol::*;
use std::mem::{size_of,ManuallyDrop};
//#region 🎟️ExactRootOwner
struct OwnedComponent {
    candidate: Option<Component>,
    source: Option<Component>,
    byte_candidate: Vec<u8>,
}

pub struct UiComponentCopy {
    owned: ManuallyDrop<OwnedComponent>,
    path: [usize; COPY_DEPTH],
    retirement: UiTypedRetirementCursor,
    complete: bool,
    closing: bool,
}

impl UiComponentCopy {
    pub fn new(source: Component) -> Self {
        Self { owned: ManuallyDrop::new(OwnedComponent { source: Some(source), candidate: None, byte_candidate: Vec::new() }), path: [0; COPY_DEPTH], retirement: Default::default(), complete: false, closing: false }
    }
    pub fn source(&self) -> Option<&Component> {
        if self.closing {
            None
        } else {
            self.owned.source.as_ref()
        }
    }
    pub fn candidate(&self) -> Option<&Component> {
        if self.closing || !self.complete {
            None
        } else {
            self.owned.candidate.as_ref()
        }
    }
    pub fn next_allocation_bytes(&self) -> Result<usize, PagedListError> {
        if self.closing {
            return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "component copy is closing" });
        }
        if self.owned.byte_candidate.capacity() != 0 {
            return Ok(0);
        }
        match (&self.owned.source, &self.owned.candidate) {
            (Some(source), Some(target)) if !self.complete => source.allocation(target, &self.path),
            _ => Ok(0),
        }
    }
    /// 🎟️ Admits only the next backing allocation; no initialized payload bytes share this turn.
    pub fn reserve_next(&mut self, allocation: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        let requested = self.next_allocation_bytes().map_err(|refusal| PagedListAllocationError { allocated_bytes: 0, kind: refusal.kind, reason: refusal.reason })?;
        if requested == 0 || allocation < requested {
            return Ok(Default::default());
        }
        let owned = &mut *self.owned;
        let source = owned.source.as_ref().ok_or_else(|| error("component allocation source is missing"))?;
        let candidate = owned.candidate.as_mut().ok_or_else(|| error("component allocation candidate is missing"))?;
        source.copy_one(candidate, &mut self.path, &mut owned.byte_candidate, allocation, 0)
    }
    pub fn advance(&mut self, items: usize, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        if self.closing {
            return Err(error("component copy is closing"));
        }
        if self.complete {
            return Ok(done());
        }
        if items == 0 || work == 0 {
            return Ok(Default::default());
        }
        let owned = &mut *self.owned;
        let source = owned.source.as_ref().ok_or_else(|| error("component copy source is missing"))?;
        if owned.candidate.is_none() {
            if work < size_of::<Component>() {
                return Ok(Default::default());
            }
            owned.candidate = Some(source.empty_like());
            return Ok(progress(size_of::<Component>()));
        }
        let step = source.copy_one(owned.candidate.as_mut().unwrap(), &mut self.path, &mut owned.byte_candidate, allocation, work)?;
        self.complete = step.complete;
        Ok(step)
    }
    pub fn take_completed(&mut self) -> Option<(Component, Component)> {
        if !self.complete || self.closing || self.owned.source.is_none() || self.owned.candidate.is_none() {
            return None;
        }
        let owned = &mut *self.owned;
        Some((owned.source.take()?, owned.candidate.take()?))
    }
    /// 📤️ One exact completed root is transferred only after its independent move grant.
    pub fn take_completed_source_with_grant(&mut self, bytes: usize) -> Option<Component> {
        if !self.complete || self.closing || bytes < size_of::<Component>() {
            return None;
        }
        self.owned.source.take()
    }
    /// 📤️ Candidate transfer leaves the source owner available for a separate return or retirement.
    pub fn take_completed_candidate_with_grant(&mut self, bytes: usize) -> Option<Component> {
        if !self.complete || self.closing || bytes < size_of::<Component>() {
            return None;
        }
        self.owned.candidate.take()
    }
    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.terminal_is_empty() {
            return Ok(UiValueRetirementStep { complete: true, ..Default::default() });
        }
        if items == 0 || bytes == 0 {
            return Ok(Default::default());
        }
        self.closing = true;
        self.retirement.advance(&mut *self.owned, items, bytes)
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.owned.source.is_none() && self.owned.candidate.is_none() && self.owned.byte_candidate.capacity() == 0 && (!self.closing || self.retirement.terminal_is_empty())
    }
}

impl UiTypedRetire for OwnedComponent {
    const DEPTH: usize = 1 + <Option<Component> as UiTypedRetire>::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (field, path) = path.split_first_mut().ok_or("component copy retirement exceeds schema depth")?;
        if *field == 0 {
            if !self.byte_candidate.is_empty() {
                let released = self.byte_candidate.len().min(bytes);
                self.byte_candidate.truncate(self.byte_candidate.len() - released);
                return Ok(UiValueRetirementStep { progressed: true, released_bytes: released, ..Default::default() });
            }
            let released = self.byte_candidate.capacity() != 0;
            drop(std::mem::take(&mut self.byte_candidate));
            *field = 1;
            return Ok(UiValueRetirementStep { progressed: true, released_items: usize::from(released), ..Default::default() });
        }
        let mut step = match *field {
            1 => self.candidate.retire_typed(path, value, bytes)?,
            2 => self.source.retire_typed(path, value, bytes)?,
            _ => return Ok(UiValueRetirementStep { complete: true, progressed: true, ..Default::default() }),
        };
        if step.complete {
            *field += 1;
            path.fill(0);
        }
        step.complete = *field == 3;
        Ok(step)
    }
}

impl Drop for UiComponentCopy {
    fn drop(&mut self) {
        if !self.terminal_is_empty() && !std::thread::panicking() {
            panic!("component copy requires exact source and candidate retirement");
        }
    }
}
//#endregion 🎟️ExactRootOwner

#[cfg(test)]
#[path = "🧪️tests/🔬️bytes/🦀️.rs"]
mod byte_tests;

#[cfg(test)]
#[path="🧪️tests/🪞️copy/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🧪️tests/🔬️bytes/🦀️.rs"]
mod bytes_tests;
