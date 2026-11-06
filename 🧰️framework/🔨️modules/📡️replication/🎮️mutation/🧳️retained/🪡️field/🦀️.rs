//! 🪡️ Typed field copying and owned exchange retain only authored and displaced children.
#[path="🟰️change/🦀️.rs"]
pub mod change;
pub use change::{RetainedFieldChange};
pub use change::record::{RetainedRecordChange, RetainedRecordChangeField, RetainedRecordFieldProjection, RetainedRecordFieldChange, RetainedRecordChangeOwner, boxed_record_field};
use super::ApplyCapability;
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
    /// 🏗️ Quotes the initialized lazy cursor, empty value and close owner, and local flags.
    pub fn embedded_birth_work()->Result<u64,ValueError>{RetainedFieldCursor::<T>::embedded_birth_work()?.checked_add(1).and_then(|n|n.checked_add(1)).and_then(|n|n.checked_add(4)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained field embedded birth work overflow"))}

    /// 📏️ Quotes the original typed source without cloning sibling row fields.
    pub fn demand(&self, body: usize, maximum_work_units:u64) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.consumed { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained field cannot quote after transfer or closure")); }
        if self.ready { return Ok(Default::default()); }
        if self.clone_complete { let work=self.cursor.take_work_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained field output work overflow"))?;if maximum_work_units<work{return Ok(Default::default())}let take = self.cursor.take_demand()?; return Ok(RetirementDemand { copy_bytes: take.copy_bytes.max(std::mem::size_of::<T>().saturating_mul(2)), depth: take.depth.max(1), ..take }); }
        self.cursor.advance_demands(body, maximum_work_units)
    }

    /// ⛽️ Quotes the actual next immutable field copy or output transfer.
    pub fn advance_work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.demand(body,maximum_work_units)?;if self.ready{return Ok(0)}if self.clone_complete{let work=self.cursor.take_work_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained field output work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}else{self.cursor.advance_work_demand(body,maximum_work_units)}}

    /// 🧵️ Copies one physically admitted fragment from its genuine immutable source.
    pub fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome{
        let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
            let demand=self.demand(body,maximum_work_units)?;if !super::retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.ready{return Ok(RetainedCloneStep::Complete(Default::default()))}if self.advance_work_demand(body,maximum_work_units)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
            if self.clone_complete{let turn=self.cursor.take(grant);work=turn.work;let(value,_)=turn.result?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained field completion omitted its original owner"))?;work+=1;*self.value=Some(value);self.ready=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            let turn=self.cursor.advance(source,body,maximum_work_units,grant,actual);work=turn.work;let step=turn.result?;admit_retained_clone_progress(grant,step.progress(),"retained authored field")?;if matches!(step,RetainedCloneStep::Complete(_)){self.clone_complete=true;return Ok(RetainedCloneStep::Progress(step.progress()))}Ok(step)
        })();semio_framework_value::RetainedWorkOutcome{result,work}
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
    pub fn begin_close(&mut self) { self.closing = true; }
    /// 🚦️ Quotes the local field flag before touching the nested clone cursor.
    pub fn begin_close_demand(&self)->RetirementDemand{if self.closing{RetirementDemand::default()}else{RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()}}}
    pub fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
    pub fn close_started(&self)->bool{self.closing}
    pub fn begin_close_paid(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress,ValueError>{let demand=self.begin_close_demand();if self.closing{return semio_framework_value::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if !super::retained_mutation_demand_fits(demand,grant){return semio_framework_value::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}self.closing=true;semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),work:1}}

    /// ♻️ Quotes the next original field or source alias retirement without an eager drop.
    pub fn close_demand(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if !self.retirement.is_empty() || self.value.is_some() { return self.retirement.next_owner_demands::<T>(self.value.is_some(), &None, maximum_body_bytes); }
        if !self.cursor.close_started(){return self.cursor.begin_close_demand()}
        self.cursor.close_demands(maximum_body_bytes)
    }

    /// ⛽️ Quotes the selected field owner or source-alias close work.
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained field closure was not started"))}self.close_demand(body)?;if !self.retirement.is_empty()||self.value.is_some(){self.retirement.next_owner_work::<T>(self.value.is_some(),&None,body)}else if !self.cursor.close_started(){self.cursor.begin_close_work_demand()}else{self.cursor.close_work_demand(body)}}

    /// 🪜️ Returns displaced ownership and immutable source aliases under separate physical grants.
    pub fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome{
        let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
            if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained field closure was not started").into())}let demand=self.close_demand(grant.maximum_copy_bytes)?;if !super::retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
            if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant,actual);work=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()))}let turn=self.retirement.begin_granted(&mut self.value,grant,actual);work=turn.work;if let Some(step)=turn.result?{return Ok(step)}
            if !self.cursor.close_started(){let turn=self.cursor.begin_close_paid(grant);work=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?))}
            let turn=self.cursor.close_step(grant,actual);work+=turn.work;let step=turn.result?;Ok(admit_retained_clone_close(grant,step,self.cursor.terminal_is_empty(),"retained field source closure")?)
        })();semio_framework_value::RetainedWorkOutcome{result,work}
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
