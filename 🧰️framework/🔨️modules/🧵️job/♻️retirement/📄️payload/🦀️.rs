//! 📄️ Original job pages and their actual operation ledger close in independent admitted turns.
use super::*;
pub(super) use semio_framework_job_payload::{close_ledger,ledger_demands};
use semio_framework_value::{RetirementDemand,RetirementTurnError,advance_retirement_turn};

/// 📬️ Original inline outcome with separately admitted presence and in-place terminal removal.
pub struct JobOutcomeSlot {
    original: MaybeUninit<StepOutcome>,
    present: bool,
}

impl JobOutcomeSlot {
    pub fn empty() -> Self { Self { original: MaybeUninit::uninit(), present: false } }
    pub fn from_outcome(outcome: StepOutcome) -> Self { Self { original: MaybeUninit::new(outcome), present: true } }
    pub fn retain(&mut self, outcome: StepOutcome) -> Result<(), StepOutcome> {
        if self.present { return Err(outcome); }
        self.original.write(outcome);
        self.present = true;
        Ok(())
    }
    pub fn original(&self) -> Option<&StepOutcome> { if self.present { Some(unsafe { self.original.assume_init_ref() }) } else { None } }
    pub fn original_mut(&mut self) -> Option<&mut StepOutcome> { if self.present { Some(unsafe { self.original.assume_init_mut() }) } else { None } }
    pub fn is_empty(&self) -> bool { !self.present }
    /// 🚚 Transfers the original carrier by value, separately from bounded in-place retirement.
    pub fn return_original(&mut self) -> Option<StepOutcome> {
        if !self.present { return None; }
        self.present = false;
        Some(unsafe { self.original.assume_init_read() })
    }
    fn remove_terminal(&mut self) {
        assert!(self.original().is_some_and(StepOutcome::terminal_is_empty));
        unsafe { self.original.assume_init_drop(); }
        self.present = false;
    }
}

impl Default for JobOutcomeSlot { fn default() -> Self { Self::empty() } }
impl std::fmt::Debug for JobOutcomeSlot { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.debug_struct("JobOutcomeSlot").field("original", &self.original()).finish() } }
impl Drop for JobOutcomeSlot { fn drop(&mut self) { assert!(std::thread::panicking() || self.is_empty(), "job outcome slot abandoned its original carrier"); } }

/// 🛟️ Original inline fault payload whose terminal carrier is dropped in place under its presence grant.
pub struct JobPayloadSlot { pub(super) original: MaybeUninit<RetainedJobPayload>, pub(super) present: bool }
impl JobPayloadSlot {
    pub fn from_payload(payload:RetainedJobPayload)->Self {Self{original:MaybeUninit::new(payload),present:true}}
    pub fn original(&self)->Option<&RetainedJobPayload>{if self.present{Some(unsafe{self.original.assume_init_ref()})}else{None}}
    pub fn original_mut(&mut self)->Option<&mut RetainedJobPayload>{if self.present{Some(unsafe{self.original.assume_init_mut()})}else{None}}
    pub fn is_empty(&self)->bool{!self.present}
    /// 🚒️ Transfers the actual fault payload by value outside its bounded retirement frontier.
    pub fn return_original(&mut self)->Option<RetainedJobPayload>{if !self.present{return None;}self.present=false;Some(unsafe{self.original.assume_init_read()})}
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let Some(payload)=self.original()else{return Ok(Default::default());};if payload.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}else{parent(payload.retirement_demands()?)}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands()?;turn(demand,grant,|grant|{
        let Some(payload)=self.original_mut()else{return Ok((RetainedCloneStep::Complete(Default::default()),true));};
        if payload.terminal_is_empty(){unsafe{self.original.assume_init_drop();}self.present=false;return Ok((RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()}),true));}
        let step=payload.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?;Ok((RetainedCloneStep::Progress(step.progress()),false))
    })}
}
impl std::fmt::Debug for JobPayloadSlot{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_struct("JobPayloadSlot").field("original",&self.original()).finish()}}
impl Drop for JobPayloadSlot{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"job payload slot abandoned its original carrier");}}

fn turn(demand:RetirementDemand,grant:RetainedCloneGrant,owner:impl FnOnce(RetainedCloneGrant)->Result<(RetainedCloneStep,bool),ValueError>)->Result<RetainedCloneStep,ValueError>{advance_retirement_turn(demand,grant,owner).map_err(|error|match error{RetirementTurnError::Owner(error)|RetirementTurnError::Receipt(error)=>error})}
fn parent(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"job payload parent depth overflow"))?;Ok(demand)}


impl StepOutcome {
    pub fn next_close_byte_demand(&self)->usize{self.retirement_demands().expect("original job outcome demand").release_bytes}
    /// 📏️ Borrows the actual selected result payload and its parent depth without reconstructing it.
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let child=match self{Self::Yield|Self::Cancelled=>return Ok(Default::default()),Self::PreviewReady(payload)=>payload,Self::CheckpointReady(checkpoint)=>&checkpoint.state,Self::Complete(candidate)if !candidate.state.terminal_is_empty()=>&candidate.state,Self::Complete(candidate)=>&candidate.output,Self::Fault(fault)=>&fault.detail};if self.terminal_is_empty(){Ok(Default::default())}else{parent(child.retirement_demands()?)}}
    /// 📬️ Preserves every actual receipt while a second success payload remains retained.
    pub fn close_step(&mut self, grant: RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands()?;turn(demand,grant,|grant|{if self.terminal_is_empty(){return Ok((RetainedCloneStep::Complete(Default::default()),true));}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=match self{Self::Yield|Self::Cancelled=>unreachable!(),Self::PreviewReady(payload)=>payload.close_step(child)?,Self::CheckpointReady(checkpoint)=>checkpoint.state.close_step(child)?,Self::Complete(candidate)if !candidate.state.terminal_is_empty()=>candidate.state.close_step(child)?,Self::Complete(candidate)=>candidate.output.close_step(child)?,Self::Fault(fault)=>fault.detail.close_step(child)?};let terminal=self.terminal_is_empty();let step=if terminal{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())};Ok((step,terminal))})}
}

/// 🪧️ Quotes the original payload frontier or its separately retained inline outcome header.
pub fn step_outcome_slot_retirement_demands(slot:&JobOutcomeSlot)->Result<RetirementDemand,ValueError>{
    let Some(outcome)=slot.original()else{return Ok(Default::default());};
    if outcome.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()});}
    parent(outcome.retirement_demands()?)
}

/// 📭️ Closes original payloads before releasing their header under a separate caller-funded turn.
pub fn close_step_outcome_slot(slot:&mut JobOutcomeSlot,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
    let demand=step_outcome_slot_retirement_demands(slot)?;
    turn(demand,grant,|grant|{
        let Some(outcome)=slot.original_mut()else{return Ok((RetainedCloneStep::Complete(Default::default()),true));};
        if outcome.terminal_is_empty(){slot.remove_terminal();return Ok((RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()}),true));}
        let step=outcome.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?;
        Ok((RetainedCloneStep::Progress(step.progress()),false))
    })
}
