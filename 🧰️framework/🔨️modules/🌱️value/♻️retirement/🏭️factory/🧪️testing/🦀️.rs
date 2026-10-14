use crate::{FactoryPayloadRetirement,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError};
use crate::retirement::controlled::ControlledRetirement;
pub(crate) struct OwnedFactory {pub(crate) payload:Vec<u8>}
#[derive(crate::FactoryPayloadRetirement)]
#[value(crate="crate")]
pub(crate) struct ScalarFactory {pub(crate) tag:u64,pub(crate) name:&'static str}
impl FactoryPayloadRetirement for OwnedFactory {
    type CloseState=Option<ControlledRetirement<Vec<u8>>>;
    fn close_state_birth_bytes(&self)->usize {0}
    fn close_state_constructor_depth(&self)->usize {0}
    fn close_state_constructor_copy_bytes(&self)->usize {0}
    fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:crate::RetainedCloneGrant)->Result<crate::RetainedCloneStep,crate::ValueError>{Ok(crate::RetainedCloneStep::Complete(Default::default()))}
    fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
    fn prepare_close_state(&self)->Self::CloseState {None}
    fn transfer_payload(value:Self,state:&mut Self::CloseState) {*state=Some(ControlledRetirement::new(value.payload).unwrap_or_else(|_|panic!("Vec payload supports controlled retirement")));}
    fn close_state_demands(state:&Self::CloseState,copy:usize)->Result<RetirementDemand,ValueError> {
        state.as_ref().map_or(Ok(RetirementDemand::default()),|owner|Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))
    }
    fn close_state_step(state:&mut Self::CloseState,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let Some(owner)=state.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
        if owner.terminal_is_empty(){if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}drop(state.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        owner.step(grant)
    }
    fn close_state_terminal_is_empty(state:&Self::CloseState)->bool {state.is_none()}
}
pub(crate) fn grant(capacity:usize,release:usize,depth:usize)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:262144,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}}
