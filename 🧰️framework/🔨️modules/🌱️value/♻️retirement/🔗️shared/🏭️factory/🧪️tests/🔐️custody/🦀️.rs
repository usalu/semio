//! 🧪️ Original weak aliases and refused payloads remain attached through exact physical closure.
use super::super::*;
use crate::{FactoryPayloadRetirement,value::observe_retirement_allocations,retirement::controlled::{ControlledRetirement,admit_typed_controlled_retirement,controlled_retirement_birth_bytes}};
use std::sync::atomic::{AtomicBool,Ordering};
struct Factory {refuse:AtomicBool,payload:String}
impl FactoryPayloadRetirement for Factory {
    type CloseState=Option<ControlledRetirement<String>>;
    fn close_state_birth_bytes(&self)->usize {0}
    fn close_state_constructor_depth(&self)->usize {0}
    fn prepare_close_state(&self)->Self::CloseState {None}
    fn transfer_payload(value:Self,state:&mut Self::CloseState){*state=Some(ControlledRetirement::new(value.payload).unwrap_or_else(|_|unreachable!()));}
    fn close_state_demands(state:&Self::CloseState,copy:usize)->Result<RetirementDemand,ValueError>{state.as_ref().map_or(Ok(Default::default()),|owner|Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))}
    fn close_state_step(state:&mut Self::CloseState,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let Some(owner)=state.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};if owner.terminal_is_empty(){if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}drop(state.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..Default::default()}));}owner.step(grant)}
    fn close_state_terminal_is_empty(state:&Self::CloseState)->bool {state.is_none()}
}
impl ArtifactOwnedValueRetirementFactory<String> for Factory {
    fn retirement_birth_bytes(&self,_:&String)->usize {controlled_retirement_birth_bytes::<String>()}
    fn retire_owned(&self,value:String,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,String)>{if self.refuse.swap(false,Ordering::SeqCst){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture original factory refuses once"),value));}admit_typed_controlled_retirement(value,grant).map(|(owner,progress)|(owner as Box<dyn ErasedSnapshotRetirement>,progress))}
}
fn text(value:&str,capacity:usize)->String {let mut value_owned=String::with_capacity(capacity);value_owned.push_str(value);value_owned}
#[test]
fn shared_factory_custody_retains_original_weak_refused_and_empty_capacity_owners() {
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for release in law["releaseGrants"].as_array().unwrap(){
        let original=text(&row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize),row["capacity"].as_u64().unwrap()as usize);let held=original.capacity()+crate::retirement::shared::arc_bytes::<String>()+271+crate::retirement::shared::arc_bytes::<Factory>();let maximum_turns=original.len().checked_add(1024).expect("fixture finite full-payload work bound");let pointer=original.as_ptr();let alias=Arc::new(original);let weak=row["weak"].as_bool().unwrap().then(||Arc::downgrade(&alias));let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(Factory {refuse:AtomicBool::new(row["refuseOnce"].as_bool().unwrap()),payload:text("",271)});
        let(mut cursor,heap)=observe_retirement_allocations(||FactorySharedRetirement::new(alias,factory));assert_eq!(heap,(0,0));assert_eq!(cursor.alias.as_ref().unwrap().as_ptr(),pointer);let(mut births,mut released,mut refusals)=(0,0,0);
        if weak.is_some(){let demand=cursor.demands(0).unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.alias.as_ref().unwrap().as_ptr(),pointer);let(_,heap)=observe_retirement_allocations(||drop(weak));assert_eq!(heap,(0,0));}
        for turn in 0..maximum_turns {
            let(demand,heap)=observe_retirement_allocations(||cursor.demands(copy.as_u64().unwrap()as usize).unwrap());assert_eq!(heap,(0,0));let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(copy.as_u64().unwrap()as usize),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes.max(release.as_u64().unwrap()as usize),maximum_depth:demand.depth};
            for denied in [Some(RetainedCloneGrant {maximum_items:0,..grant}),(demand.copy_bytes!=0).then_some(RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes!=0).then_some(RetainedCloneGrant {maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes!=0).then_some(RetainedCloneGrant {maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth!=0).then_some(RetainedCloneGrant {maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||cursor.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
            let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant));match step {Ok(step)=>{assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes),"turn{turn}");births+=heap.0;released+=heap.1;if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;}},Err(error)=>{assert_eq!(error.message,"fixture original factory refuses once");assert_eq!(heap,(0,0));assert_eq!(cursor.pending.as_ref().unwrap().as_ptr(),pointer);refusals+=1;}}assert!(turn+1<maximum_turns,"full original payload close exceeded its fixture-derived work bound");
        }
        assert_eq!(refusals,usize::from(row["refuseOnce"].as_bool().unwrap()));assert_eq!(released,held+births);let(_,heap)=observe_retirement_allocations(||drop(cursor));assert_eq!(heap,(0,0));
    }}}
    eprintln!("[DEBUG] shared factory giant original pointers/empty retained-capacity/weak original aliases/refused factory payload preserved, actual factory payload+all old Arc/ticket backing release exact, independent undergrants0heap, terminal Drop0heap");
}


#[test]
fn shared_factory_in_place_admission_preserves_original_slots_and_exact_issuer_receipts(){
    use crate::retirement::{queue::RetirementQueue,admit_original_owned_retirement};
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for route in law["admissionRoutes"].as_array().unwrap(){for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){
        let payload=text(&row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize),row["capacity"].as_u64().unwrap()as usize);let pointer=payload.as_ptr();let shared=route=="shared";let originals=payload.capacity()+271+crate::retirement::shared::arc_bytes::<Factory>()+if shared{crate::retirement::shared::arc_bytes::<String>()}else{0};
        let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(Factory{refuse:AtomicBool::new(!shared),payload:text("",271)});
        let capacity=if shared{FactorySharedRetirement::<String>::constructor_capacity_bytes()}else{factory.retirement_birth_bytes(&payload)};
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1};let mut births=0;let mut released=0;
        let (frame,p)=if shared{
            let mut original=Some(Arc::new(payload));let strong=Arc::strong_count(&factory);
            for under in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:capacity-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (result,heap)=observe_retirement_allocations(||FactorySharedRetirement::admit_original(&mut original,&factory,under));assert!(result.is_err());assert_eq!(original.as_ref().unwrap().as_ptr(),pointer);assert_eq!(Arc::strong_count(&factory),strong);assert_eq!(heap,(0,0));}
            let (result,(a,r))=observe_retirement_allocations(||FactorySharedRetirement::admit_original(&mut original,&factory,grant).unwrap());let (frame,p)=result.unwrap();assert!(original.is_none());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;
            let (none,heap)=observe_retirement_allocations(||FactorySharedRetirement::admit_original(&mut original,&factory,RetainedCloneGrant::default()).unwrap());assert!(none.is_none());assert_eq!(heap,(0,0));(frame,p)
        }else{
            let mut original=Some(payload);
            for under in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:capacity-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (result,heap)=observe_retirement_allocations(||admit_original_owned_retirement(&mut original,factory.as_ref(),under));assert!(result.is_err());assert_eq!(original.as_ref().unwrap().as_ptr(),pointer);assert_eq!(heap,(0,0));}
            let (refused,heap)=observe_retirement_allocations(||admit_original_owned_retirement(&mut original,factory.as_ref(),grant));assert!(refused.is_err());assert_eq!(original.as_ref().unwrap().as_ptr(),pointer);assert_eq!(heap,(0,0));
            let (result,(a,r))=observe_retirement_allocations(||admit_original_owned_retirement(&mut original,factory.as_ref(),grant).unwrap());let (frame,p)=result.unwrap();assert!(original.is_none());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;
            let (none,heap)=observe_retirement_allocations(||admit_original_owned_retirement(&mut original,factory.as_ref(),RetainedCloneGrant::default()).unwrap());assert!(none.is_none());assert_eq!(heap,(0,0));(frame,p)
        };assert!(p.fits(grant));assert_eq!(p.retained_capacity_bytes,capacity);
        let mut queue=RetirementQueue::default();
        while !queue.has_reserved_slot(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:queue.next_reserve_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:queue.len()+1};let (p,(a,r))=observe_retirement_allocations(||queue.reserve_step(grant).unwrap());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;}
        let (_,heap)=observe_retirement_allocations(||queue.admit_retirement(frame,RetainedCloneGrant{maximum_items:1,maximum_depth:queue.len()+1,..Default::default()}).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));
        while !queue.has_reserved_slot(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:queue.next_reserve_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:queue.len()+1};let (p,(a,r))=observe_retirement_allocations(||queue.reserve_step(grant).unwrap());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;}
        let authority:Arc<dyn FactoryRetirement>=factory;let authority=FactoryAuthority::new(authority);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:RetirementQueue::frame_birth_bytes::<FactoryAuthority>(),maximum_release_bytes:0,maximum_depth:queue.len()+1};let (p,(a,r))=observe_retirement_allocations(||queue.admit_owned(authority,grant).map_err(|(error,_)|error).unwrap());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;
        for turn in 0..1000000{if queue.terminal_is_empty(){break;}let body=copy.as_u64().unwrap()as usize;let demand=(queue.next_copy_byte_demand().unwrap(),queue.next_capacity_byte_demand(body).unwrap(),queue.next_release_byte_demand().unwrap(),queue.next_depth_demand().unwrap());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.0.max(body),maximum_capacity_bytes:demand.1,maximum_release_bytes:demand.2,maximum_depth:demand.3};let (step,(a,r))=observe_retirement_allocations(||queue.step(grant).unwrap());let p=step.progress();assert!(p.fits(grant));assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;assert!(p.copied_items!=0||matches!(step,RetainedCloneStep::Complete(_)),"in-place admission custody blocked at {turn}");}
        assert!(queue.terminal_is_empty());assert_eq!(released,originals+births);let (_,heap)=observe_retirement_allocations(||drop(queue));assert_eq!(heap,(0,0));
        eprintln!("[DEBUG] In-place original factory admission route={route} copy={copy} original={originals} births={births} physical={released} refusedOriginalPointer=true terminalDrop=0");
    }}}
}
