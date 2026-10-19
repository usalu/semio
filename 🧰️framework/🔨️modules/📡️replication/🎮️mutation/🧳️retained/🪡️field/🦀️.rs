//! 🪡️ Typed field copying and owned exchange retain only authored and displaced children.
#[path="🟰️change/🦀️.rs"]
pub mod change;
pub use change::{RetainedFieldChange};
pub use change::record::{RetainedRecordChange, RetainedRecordChangeField, RetainedRecordFieldChange, RetainedRecordChangeOwner, boxed_record_field};
use super::super::ApplyCapability;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, RetainedFieldCursor, admit_retained_clone_close, admit_retained_clone_progress};

/// 📦️ A staged authored field exchanges ownership once and closes its displaced child explicitly.
pub struct RetainedFieldPreparation<T: RetainedClone> {
    cursor: RetainedFieldCursor<T>,
    value: std::mem::ManuallyDrop<Option<T>>,
    retirement: RetainedCloneClose,
    ready: bool,
    clone_complete: bool,
    consumed: bool,
    closing: bool,
}

impl<T: RetainedClone> Default for RetainedFieldPreparation<T> {
    fn default() -> Self { Self { cursor: Default::default(), value: std::mem::ManuallyDrop::new(None), retirement: Default::default(), ready: false, clone_complete: false, consumed: false, closing: false } }
}

impl<T: RetainedClone> RetainedFieldPreparation<T> {
    /// 📏️ Quotes the original typed source without cloning sibling row fields.
    pub fn demand(&self, source: RetainedCloneRef<'_, T>, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.consumed { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained field cannot quote after transfer or closure")); }
        if self.ready { return Ok(Default::default()); }
        if self.clone_complete { return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<T>().saturating_mul(2), depth: 1, ..Default::default() }); }
        self.cursor.advance_demands(source, body)
    }

    /// 🧵️ Copies one physically admitted fragment from its genuine immutable source.
    pub fn advance(&mut self, source: RetainedCloneRef<'_, T>, body:usize,grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.demand(source, body)?;
        if !super::retained_mutation_demand_fits(demand, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.ready { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.clone_complete {
            *self.value = Some(self.cursor.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "retained field completion omitted its original owner"))?);
            self.ready = true;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        let step = self.cursor.advance(source, body, grant)?;
        admit_retained_clone_progress(grant, step.progress(), "retained authored field")?;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            self.clone_complete = true;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(step)
    }

    /// 👁️ Borrows a prepared field during atomic preflight without constructing a candidate row.
    pub fn value(&self) -> Option<&T> { if self.ready && !self.consumed { self.value.as_ref() } else { None } }

    /// 📐️ Quotes an ownership exchange of two fixed field headers.
    pub fn exchange_demand(&self) -> Result<RetirementDemand, ValueError> {
        if !self.ready || self.consumed || self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained field is not ready for exchange")); }
        Ok(RetirementDemand { copy_bytes: std::mem::size_of::<T>().checked_mul(2).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "retained field exchange layout overflow"))?, capacity_bytes: 0, release_bytes: 0, depth: 1 })
    }

    /// 🔑️ Exchanges only validated authored ownership under the central mutation capability.
    pub fn install(&mut self, field: &mut T, _capability: ApplyCapability, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let demand = self.exchange_demand()?;
        if !super::retained_mutation_demand_fits(demand, grant) { return Ok(Default::default()); }
        let authored = self.value.take().expect("ready authored field");
        *self.value = Some(std::mem::replace(field, authored));
        self.consumed = true;
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    /// 📤️ Transfers an inverse or diff field under its original fixed-header copy grant.
    pub fn take(&mut self, grant: RetainedCloneGrant) -> Result<(Option<T>, RetainedCloneProgress), ValueError> {
        let demand = self.exchange_demand()?;
        if !super::retained_mutation_demand_fits(demand, grant) { return Ok((None, Default::default())); }
        self.consumed = true;
        Ok((self.value.take(), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    /// 🛑️ Starts typed cleanup before retiring a candidate or displaced field.
    pub fn begin_close(&mut self) { self.closing = true; self.cursor.begin_close(); }

    /// ♻️ Quotes the next original field or source alias retirement without an eager drop.
    pub fn close_demand(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if !self.retirement.is_empty() || self.value.is_some() { return self.retirement.next_owner_demands::<T>(self.value.is_some(), &None, maximum_body_bytes); }
        self.cursor.close_demands(maximum_body_bytes)
    }

    /// 🪜️ Returns displaced ownership and immutable source aliases under separate physical grants.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained field closure was not started")); }
        let demand = self.close_demand(grant.maximum_copy_bytes)?;
        if !super::retained_mutation_demand_fits(demand, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.retirement.is_empty() { return self.retirement.step_granted(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(step) = self.retirement.begin_granted(&mut self.value, grant)? { return Ok(step); }
        let step = self.cursor.close_step(grant)?;
        admit_retained_clone_close(grant, step, self.cursor.terminal_is_empty(), "retained field source closure")
    }

    /// 🫙️ Reports that candidate, displaced field and all original aliases have returned.
    pub fn terminal_is_empty(&self) -> bool { self.value.is_none() && self.retirement.is_empty() && self.cursor.terminal_is_empty() }
}

impl<T: RetainedClone> Drop for RetainedFieldPreparation<T> {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(std::thread::panicking() || empty, "retained field preparation requires typed complete closure"); if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.value); } } }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
