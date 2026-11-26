//! 🧪️ Original shared aliases and refused payloads remain attached through exact physical closure.
use super::super::*;
use crate::{owned::FactoryOwnedRetirement,FactoryRetirement,FactoryAuthority,FactoryPayloadRetirement,FactoryProducerReply,FactoryProducerReplyComplete,OriginalRetirementFailure,OwnedValueRetirementFactory,SharedOriginalStorage,allocation_testing::observe_retirement_allocations,driven::{poll,leaf,replied,grant_of,settle_owned,CPU},retirement::controlled::ControlledRetirement};
use std::sync::atomic::{AtomicBool,Ordering};
struct Factory {refuse:AtomicBool,payload:String}
impl FactoryPayloadRetirement for Factory {
    type CloseState=Option<ControlledRetirement<String>>;
    fn close_state_birth_bytes(&self)->usize {0}
    fn close_state_constructor_depth(&self)->usize {0}
    fn close_state_constructor_copy_bytes(&self)->usize {std::mem::size_of::<Self::CloseState>()}
    fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn close_state_preparation_work_demand(&self,_:&Self::CloseState,_:usize)->Result<u64,ValueError>{Ok(0)}
    fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:usize,_:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
    fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
    fn prepare_close_state(&self)->Self::CloseState {None}
    fn transfer_payload(value:Self,state:&mut Self::CloseState){*state=Some(ControlledRetirement::new(value.payload).unwrap_or_else(|_|unreachable!()));}
    fn close_state_demands(state:&Self::CloseState,copy:usize)->Result<RetirementDemand,ValueError>{state.as_ref().map_or(Ok(Default::default()),|owner|Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))}
    fn close_state_work_demand(state:&Self::CloseState,body:usize)->Result<u64,ValueError>{state.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)})}
    fn close_state_step(state:&mut Self::CloseState,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        let done=|result:Result<RetainedCloneStep,OriginalRetirementFailure>,work:u128|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
        let Some(owner)=state.as_mut()else{return done(Ok(RetainedCloneStep::Complete(Default::default())),0);};
        if owner.terminal_is_empty(){if effect.grant().maximum_items==0{return done(Ok(RetainedCloneStep::Progress(Default::default())),0);}match effect.performed(1){Ok(true)=>{},Ok(false)=>return done(Ok(RetainedCloneStep::Progress(Default::default())),0),Err(error)=>return done(Err(error.into()),0)}drop(state.take());return done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..Default::default()})),1);}
        owner.step(body,effect)
    }
    fn close_state_into<'reply>(state:&mut Self::CloseState,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{crate::original_factory_fixture::close_controlled_state_into(state,body,effect,reply)}
    fn recover_close_state_into<'reply>(state:&mut Self::CloseState,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{crate::original_factory_fixture::recover_controlled_state_into(state,body,effect,reply)}
    unsafe fn close_state_terminal_native_release_safe()->bool{true}
    fn close_state_terminal_is_empty(state:&Self::CloseState)->bool {state.is_none()}
}
impl ArtifactOwnedValueRetirementFactory<String> for Factory {fn retire_owned_work_demand(&self,_:&String)->Result<u64,crate::ValueError>{Ok(2)}
    fn retirement_birth_bytes(&self,_:&String)->usize {crate::retirement::controlled::controlled_retirement_birth_bytes::<String>()}
    fn retire_owned_demands(&self,value:&String)->Result<RetirementDemand,ValueError>{OwnedValueRetirementFactory::<String>::default().retire_owned_demands(value)}
    fn retire_owned<'reply>(&self,value:&mut Option<String>,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{
        let boundary=effect.performed_boundary();assert!(effect.performed(1).unwrap());
        if self.refuse.swap(false,Ordering::SeqCst){return reply.complete_observed(effect,boundary,Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture original factory refuses once").into()));}
        settle_owned(value,effect,reply,boundary)
    }
    fn recover_retire_owned<'reply>(&self,_:&mut Option<String>,_:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{reply.pending()}
}
fn text(value:&str,capacity:usize)->String {let mut value_owned=String::with_capacity(capacity);value_owned.push_str(value);value_owned}
#[test]
fn shared_factory_custody_retains_original_refused_and_empty_capacity_owners() {
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for release in law["releaseGrants"].as_array().unwrap(){
        let original=text(&row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize),row["capacity"].as_u64().unwrap()as usize);let held=original.capacity()+SharedOriginalStorage::<String>::allocation_bytes()+271+crate::retirement::arc_custody::arc_bytes::<Factory>();let maximum_turns=original.len().checked_add(1024).expect("fixture finite full-payload work bound");let pointer=original.as_ptr();let alias=SharedOriginalStorage::from_cold(original);let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(Factory {refuse:AtomicBool::new(row["refuseOnce"].as_bool().unwrap()),payload:text("",271)});
        let(mut cursor,heap)=observe_retirement_allocations(||FactorySharedRetirement::new(alias,factory));assert_eq!(heap,(0,0));assert_eq!(cursor.alias.as_ref().unwrap().get().as_ptr(),pointer);let(mut births,mut released,mut refusals)=(0,0,0);let body=copy.as_u64().unwrap()as usize;
        for turn in 0..maximum_turns {
            let(demand,heap)=observe_retirement_allocations(||cursor.demands(body).unwrap());assert_eq!(heap,(0,0));let work=ErasedSnapshotRetirement::next_computational_work(&cursor,body).unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(body),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes.max(release.as_u64().unwrap()as usize),maximum_depth:demand.depth};
            for denied in [Some(RetainedCloneGrant {maximum_items:0,..grant}),(demand.copy_bytes!=0).then_some(RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes!=0).then_some(RetainedCloneGrant {maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes!=0).then_some(RetainedCloneGrant {maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth!=0).then_some(RetainedCloneGrant {maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten(){let(step,heap)=poll(denied,work,|effect|crate::close_owner(&mut cursor,body,effect));assert_eq!(step.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));}
            let(step,heap)=poll(grant,work,|effect|crate::close_owner(&mut cursor,body,effect));match step.result {Ok(step)=>{assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes),"turn{turn}");births+=heap.0;released+=heap.1;if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;}},Err(error)=>{assert_eq!(error.primary().unwrap().message,"fixture original factory refuses once");assert_eq!(heap,(0,0));assert_eq!(cursor.owned.original().unwrap().as_ptr(),pointer);refusals+=1;}}assert!(turn+1<maximum_turns,"full original payload close exceeded its fixture-derived work bound");
        }
        assert_eq!(refusals,usize::from(row["refuseOnce"].as_bool().unwrap()));assert_eq!(released,held+births);let(_,heap)=observe_retirement_allocations(||drop(cursor));assert_eq!(heap,(0,0));
    }}}
}

/// 🪑️ Reserves queue slots under their quoted fundings until one is ready and reports the physical receipts.
fn reserve(queue:&mut semio_framework_value_retirement_queue::RetirementQueue,births:&mut usize,released:&mut usize){while !queue.has_reserved_slot(){let g=grant_of(queue.reserve_demand().unwrap());let(turn,heap,_)=leaf(g,CPU,|effect|queue.reserve_step(effect));let p=turn.result.unwrap();assert_eq!((p.retained_capacity_bytes,p.released_bytes),heap);*births+=heap.0;*released+=heap.1;}}

#[test]
fn shared_factory_in_place_admission_preserves_original_slots_and_exact_issuer_receipts(){
    use semio_framework_value_retirement_queue::RetirementQueue;
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for route in law["admissionRoutes"].as_array().unwrap(){for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){
        let payload=text(&row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize),row["capacity"].as_u64().unwrap()as usize);let pointer=payload.as_ptr();let shared=route=="shared";let originals=payload.capacity()+271+crate::retirement::arc_custody::arc_bytes::<Factory>()+if shared{SharedOriginalStorage::<String>::allocation_bytes()}else{0};
        let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(Factory{refuse:AtomicBool::new(!shared),payload:text("",271)});
        let mut births=0;let mut released=0;let mut output=None;let mut issuer=Some(factory.clone());
        let(capacity,grant)=if shared{let original=Some(SharedOriginalStorage::from_cold(payload));let demand=FactorySharedRetirement::<String>::original_admission_demands(&original);let capacity=FactorySharedRetirement::<String>::constructor_capacity_bytes();let grant=grant_of(demand);let mut original=original;
            for under in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:capacity-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(turn,heap,_)=replied(under,CPU,|effect,reply|FactorySharedRetirement::admit_original_into(&mut original,&mut issuer,&mut output,effect,reply));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(original.as_ref().unwrap().get().as_ptr(),pointer);assert!(output.is_none());assert_eq!(heap,(0,0));}
            let(turn,(a,r),_)=replied(grant,CPU,|effect,reply|FactorySharedRetirement::admit_original_into(&mut original,&mut issuer,&mut output,effect,reply));let p=turn.result.unwrap().progress();assert!(original.is_none());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;
            let mut vacant=None;let(turn,heap,_)=replied(RetainedCloneGrant::default(),CPU,|effect,reply|FactorySharedRetirement::admit_original_into(&mut original,&mut issuer,&mut vacant,effect,reply));assert!(turn.result.is_ok());assert!(vacant.is_none());assert_eq!(heap,(0,0));(capacity,grant)
        }else{let mut original=Some(payload);let demand=FactoryOwnedRetirement::<String>::original_admission_demands(&original);let capacity=FactoryOwnedRetirement::<String>::constructor_capacity_bytes();let grant=grant_of(demand);
            for under in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:capacity-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(poll,heap,_)=leaf(under,CPU,|effect|FactoryOwnedRetirement::admit_original(&mut original,&mut issuer,&mut output,effect));let RetainedWorkPoll::Complete(turn)=poll else{panic!("original admission stayed pending")};assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(original.as_ref().unwrap().as_ptr(),pointer);assert!(output.is_none());assert_eq!(heap,(0,0));}
            let(poll,(a,r),_)=leaf(grant,CPU,|effect|FactoryOwnedRetirement::admit_original(&mut original,&mut issuer,&mut output,effect));let RetainedWorkPoll::Complete(turn)=poll else{panic!("original admission stayed pending")};let p=turn.result.unwrap().progress();assert!(original.is_none());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;
            let mut vacant=None;let(poll,heap,_)=leaf(RetainedCloneGrant::default(),CPU,|effect|FactoryOwnedRetirement::admit_original(&mut original,&mut issuer,&mut vacant,effect));let RetainedWorkPoll::Complete(turn)=poll else{panic!("original admission stayed pending")};assert!(turn.result.is_ok());assert!(vacant.is_none());assert_eq!(heap,(0,0));(capacity,grant)
        };assert!(issuer.is_none());assert!(output.is_some());let p_capacity=births;assert_eq!(p_capacity,capacity);assert!(grant.maximum_capacity_bytes>=capacity);
        let mut queue=RetirementQueue::default();
        reserve(&mut queue,&mut births,&mut released);
        let place=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_depth:64,..Default::default()};let(turn,heap,_)=leaf(place,CPU,|effect|queue.admit_retirement(&mut output,effect));assert!(turn.result.unwrap().is_some());assert!(output.is_none());assert_eq!(heap,(0,0));
        reserve(&mut queue,&mut births,&mut released);
        let authority:Arc<dyn FactoryRetirement>=factory;let mut authority=Some(FactoryAuthority::new(authority));let g=grant_of(queue.admit_owned_demand(authority.as_ref().unwrap()).unwrap());let(turn,(a,r),_)=leaf(g,CPU,|effect|queue.admit_owned(&mut authority,effect));let p=turn.result.unwrap().expect("funded authority admission");assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));assert!(authority.is_none());births+=a;released+=r;
        let mut refusals=0;
        for turn in 0..1000000{if queue.terminal_is_empty(){break;}let body=copy.as_u64().unwrap()as usize;let demand=queue.next_demand(body).unwrap();let work=queue.next_computational_work(body).unwrap();let grant=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.max(body),..grant_of(demand)};let(step,heap)=poll(grant,work,|effect|queue.step(body,effect));match step.result{Ok(step)=>{let p=step.progress();assert!(p.fits(grant));assert_eq!((p.retained_capacity_bytes,p.released_bytes),heap);births+=heap.0;released+=heap.1;assert!(p.copied_items!=0||matches!(step,RetainedCloneStep::Complete(_)),"in-place admission custody blocked at {turn}");},Err(error)=>{assert_eq!(error.primary().unwrap().message,"fixture original factory refuses once");assert_eq!(heap,(0,0));refusals+=1;}}}
        assert_eq!(refusals,usize::from(!shared));assert!(queue.terminal_is_empty());assert_eq!(released,originals+births);let(_,heap)=observe_retirement_allocations(||drop(queue));assert_eq!(heap,(0,0));
    }}}
}
