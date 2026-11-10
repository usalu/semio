//! 📄️ Original job pages and their actual operation ledger close in independent admitted turns.
use super::*;

use semio_framework_value_retained_clone::{RetirementDemand,RetainedWorkOutcome};

/// 📬️ See `README.md`.
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
    /// 🚚 See `README.md`.
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

/// 🛟️ See `README.md`.
pub struct JobPayloadSlot { pub(super) original: MaybeUninit<RetainedJobPayload>, pub(super) present: bool }
impl JobPayloadSlot {
    pub fn from_payload(payload:RetainedJobPayload)->Self {Self{original:MaybeUninit::new(payload),present:true}}
    pub fn original(&self)->Option<&RetainedJobPayload>{if self.present{Some(unsafe{self.original.assume_init_ref()})}else{None}}
    pub fn original_mut(&mut self)->Option<&mut RetainedJobPayload>{if self.present{Some(unsafe{self.original.assume_init_mut()})}else{None}}
    pub fn is_empty(&self)->bool{!self.present}
    /// 🚒️ See `README.md`.
    pub fn return_original(&mut self)->Option<RetainedJobPayload>{if !self.present{return None;}self.present=false;Some(unsafe{self.original.assume_init_read()})}
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let Some(payload)=self.original()else{return Ok(Default::default());};if payload.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}else{parent(payload.retirement_demands()?)}}
    pub fn close_step(&mut self,effect:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let grant=effect.grant();
        if self.is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
        let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
        if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
        let Some(payload)=self.original_mut()else{return RetainedWorkOutcome{result:Err(semio_framework_value_retained_clone::ValueError::invariant("step payload owner is absent")),work:0}};
        if payload.terminal_is_empty(){unsafe{self.original.assume_init_drop();}self.present=false;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1};}
        let turn=payload.close_step(&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}));RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}
    }
}
impl std::fmt::Debug for JobPayloadSlot{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_struct("JobPayloadSlot").field("original",&self.original()).finish()}}
impl Drop for JobPayloadSlot{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"job payload slot abandoned its original carrier");}}

fn parent(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"job payload parent depth overflow"))?;Ok(demand)}


impl StepOutcome {
    pub fn next_close_byte_demand(&self)->usize{self.retirement_demands().expect("original job outcome demand").release_bytes}
    /// 📏️ See `README.md`.
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let child=match self{Self::Yield|Self::Cancelled=>return Ok(Default::default()),Self::PreviewReady(payload)=>payload,Self::CheckpointReady(checkpoint)=>&checkpoint.state,Self::Complete(candidate)if !candidate.state.terminal_is_empty()=>&candidate.state,Self::Complete(candidate)=>&candidate.output,Self::Fault(fault)=>&fault.detail};if self.terminal_is_empty(){Ok(Default::default())}else{parent(child.retirement_demands()?)}}
    /// 📪️ See `README.md`.
    pub fn close_step(&mut self,effect:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let grant=effect.grant();
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
        let demand=match self.retirement_demands(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
        if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let effect=&mut effect.nested(child);let turn=match self{Self::Yield|Self::Cancelled=>return RetainedWorkOutcome{result:Err(semio_framework_value_retained_clone::ValueError::invariant("yield and cancelled step outcomes own no payload")),work:0},Self::PreviewReady(payload)=>payload.close_step(effect),Self::CheckpointReady(checkpoint)=>checkpoint.state.close_step(effect),Self::Complete(candidate)if !candidate.state.terminal_is_empty()=>candidate.state.close_step(effect),Self::Complete(candidate)=>candidate.output.close_step(effect),Self::Fault(fault)=>fault.detail.close_step(effect)};
        let terminal=self.terminal_is_empty();RetainedWorkOutcome{result:turn.result.map(|step|if terminal{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())}),work:turn.work}
    }
}

/// 🪧️ See `README.md`.
pub fn step_outcome_slot_retirement_demands(slot:&JobOutcomeSlot)->Result<RetirementDemand,ValueError>{
    let Some(outcome)=slot.original()else{return Ok(Default::default());};
    if outcome.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()});}
    parent(outcome.retirement_demands()?)
}

/// 📭️ See `README.md`.
pub fn close_step_outcome_slot(slot:&mut JobOutcomeSlot,effect:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let grant=effect.grant();
    if slot.is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
    let demand=match step_outcome_slot_retirement_demands(slot){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
    if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
    let Some(outcome)=slot.original_mut()else{return RetainedWorkOutcome{result:Err(semio_framework_value_retained_clone::ValueError::invariant("step outcome owner is absent")),work:0}};
    if outcome.terminal_is_empty(){slot.remove_terminal();return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1};}
    let turn=outcome.close_step(&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}));RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}
}




