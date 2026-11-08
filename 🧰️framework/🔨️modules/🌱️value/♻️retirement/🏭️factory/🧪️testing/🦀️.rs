use crate::{FactoryPayloadRetirement,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError};
use crate::retirement::controlled::ControlledRetirement;
pub struct OwnedFactory {pub payload:Vec<u8>}
#[derive(crate::FactoryPayloadRetirement)]
#[value(crate="crate")]
pub struct ScalarFactory {pub tag:u64,pub name:&'static str}
impl FactoryPayloadRetirement for OwnedFactory {
    type CloseState=Option<ControlledRetirement<Vec<u8>>>;
    fn close_state_birth_bytes(&self)->usize {0}
    fn close_state_constructor_depth(&self)->usize {0}
    fn close_state_constructor_copy_bytes(&self)->usize {0}
    fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn close_state_preparation_work_demand(&self,_:&Self::CloseState,_:usize)->Result<u64,crate::ValueError>{Ok(0)}
    fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkPoll{crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
    fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
    fn prepare_close_state(&self)->Self::CloseState {None}
    fn transfer_payload(value:Self,state:&mut Self::CloseState) {*state=Some(ControlledRetirement::new(value.payload).unwrap_or_else(|_|panic!("Vec payload supports controlled retirement")));}
    fn close_state_demands(state:&Self::CloseState,copy:usize)->Result<RetirementDemand,ValueError> {
        state.as_ref().map_or(Ok(RetirementDemand::default()),|owner|Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))
    }
    fn close_state_work_demand(state:&Self::CloseState,body:usize)->Result<u64,ValueError> {state.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)})}
    fn close_state_step(state:&mut Self::CloseState,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll {
        let done=|result:Result<RetainedCloneStep,crate::OriginalRetirementFailure>,work:u128|crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result,work});
        let Some(owner)=state.as_mut()else{return done(Ok(RetainedCloneStep::Complete(Default::default())),0);};
        if owner.terminal_is_empty(){
            if effect.grant().maximum_items==0{return done(Ok(RetainedCloneStep::Progress(Default::default())),0);}
            match effect.performed(1){Ok(true)=>{},Ok(false)=>return done(Ok(RetainedCloneStep::Progress(Default::default())),0),Err(error)=>return done(Err(error.into()),0)}
            drop(state.take());
            return done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..Default::default()})),1);
        }
        owner.step(effect)
    }
    fn close_state_terminal_is_empty(state:&Self::CloseState)->bool {state.is_none()}
}
pub fn grant(capacity:usize,release:usize,depth:usize)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:262144,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}}
