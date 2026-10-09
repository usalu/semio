//! 🧪️ Original send-only payloads and provider-refused values retain exact physical custody.
use super::*;
use crate::{observe_retirement_allocations,retirement::{RetireOwned,RetirementCursor,admit_owned_retirement,owned_retirement_birth_bytes,shared::arc_bytes},FactoryPayloadRetirement};
use std::sync::atomic::{AtomicBool,Ordering};
struct RefusingFactory{refuse:AtomicBool}
impl FactoryPayloadRetirement for RefusingFactory{
 type CloseState=bool;
 fn close_state_birth_bytes(&self)->usize{0}
 fn close_state_constructor_depth(&self)->usize{0}
 fn close_state_constructor_copy_bytes(&self)->usize {std::mem::size_of::<Self::CloseState>()}
 fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
 fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:crate::RetainedCloneGrant)->Result<crate::RetainedCloneStep,crate::ValueError>{Ok(crate::RetainedCloneStep::Complete(Default::default()))}
 fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
 fn prepare_close_state(&self)->bool{false}
 fn transfer_payload(_:Self,state:&mut bool){*state=true;}
 fn close_state_demands(_: &bool,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_state_step(state:&mut bool,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{assert!(*state);Ok(RetainedCloneStep::Complete(Default::default()))}
 fn close_state_terminal_is_empty(state:&bool)->bool{*state}
}
impl ArtifactOwnedValueRetirementFactory<String> for RefusingFactory{
 fn retirement_birth_bytes(&self,_:&String)->usize{owned_retirement_birth_bytes::<String>()}
 fn retire_owned(&self,original:String,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,String)>{if self.refuse.swap(false,Ordering::SeqCst){Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture issuer refuses once"),original))}else{admit_owned_retirement(original,grant)}}
}
#[test]
fn factory_owned_retirement_preserves_original_provider_refusal_and_exact_terminal_allocations(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){
  let text=row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);let mut payload=String::with_capacity(row["capacity"].as_u64().unwrap()as usize);payload.push_str(&text);let pointer=payload.as_ptr();let mut original=Some(payload);let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(RefusingFactory{refuse:AtomicBool::new(true)});let strong=Arc::strong_count(&factory);let held=original.as_ref().unwrap().capacity()+arc_bytes::<RefusingFactory>();
  let bytes=FactoryOwnedRetirement::<String>::constructor_capacity_bytes();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1};
  for under in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (result,heap)=observe_retirement_allocations(||FactoryOwnedRetirement::admit_original(&mut original,&factory,under));assert!(result.is_err());assert_eq!(original.as_ref().unwrap().as_ptr(),pointer);assert_eq!(Arc::strong_count(&factory),strong);assert_eq!(heap,(0,0));}
  let (result,(a,r))=observe_retirement_allocations(||FactoryOwnedRetirement::admit_original(&mut original,&factory,grant).unwrap());let (mut frame,p)=result.unwrap();assert!(original.is_none());assert_eq!(frame.original.as_ref().unwrap().as_ptr(),pointer);assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));assert_eq!(a,bytes);let mut births=a;let mut released=r;let mut refusals=0;
  let (_,heap)=observe_retirement_allocations(||drop(factory));assert_eq!(heap,(0,0));
  for turn in 0..1000000{
   let body=copy.as_u64().unwrap()as usize;let (demand,heap)=observe_retirement_allocations(||frame.demands(body).unwrap());assert_eq!(heap,(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(body),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for under in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten(){let (step,heap)=observe_retirement_allocations(||frame.close_step(under).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
   let (step,(a,r))=observe_retirement_allocations(||frame.close_step(grant));match step{Ok(step)=>{let p=step.progress();assert!(p.fits(grant));assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;},Err(error)=>{assert_eq!(error.message,"original fixture issuer refuses once");assert_eq!(frame.original.as_ref().unwrap().as_ptr(),pointer);assert_eq!((a,r),(0,0));refusals+=1;}}if frame.terminal_is_empty(){break;}assert!(turn<999999);
   if turn==cut.as_u64().unwrap()as usize{let (same,heap)=observe_retirement_allocations(||frame);frame=same;assert_eq!(heap,(0,0));}
  }
  assert_eq!(refusals,1);assert!(frame.terminal_is_empty());let mut erased=Some(frame as Box<dyn ErasedSnapshotRetirement>);let (demand,heap)=observe_retirement_allocations(||crate::factory_ticket_demands(erased.as_ref().unwrap(),0).unwrap());assert_eq!(heap,(0,0));assert_eq!(demand.release_bytes,bytes);let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:bytes,maximum_depth:demand.depth,..Default::default()};let (small,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut erased,RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant}).unwrap());assert_eq!(small.progress(),Default::default());assert_eq!(heap,(0,0));let (done,(a,r))=observe_retirement_allocations(||crate::close_factory_ticket(&mut erased,grant).unwrap());assert!(done.progress().fits(grant));assert_eq!((done.progress().retained_capacity_bytes,done.progress().released_bytes),(a,r));births+=a;released+=r;assert!(erased.is_none());assert_eq!(released,held+births);let (_,heap)=observe_retirement_allocations(||drop((erased,original)));assert_eq!(heap,(0,0));
  eprintln!("[DEBUG] Factory owned original pointer/capacity={} copy={copy} cancellation={cut} providerRefusal=1 births={births} physical={released} terminalDrop=0",row["capacity"]);
 }}}
}
struct SendOnly{payload:String,marker:std::cell::Cell<u8>}
impl RetireOwned for SendOnly{
 fn retirement(self)->Box<dyn RetirementCursor>{assert_eq!(self.marker.get(),7);self.payload.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.payload.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
}
#[test]
fn factory_owned_retirement_accepts_actual_send_only_payloads(){
 let mut original=Some(SendOnly{payload:String::with_capacity(8192),marker:std::cell::Cell::new(7)});let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<SendOnly>>=Arc::new(crate::retirement::OwnedValueRetirementFactory::<SendOnly>::default());let held=8192+arc_bytes::<crate::retirement::OwnedValueRetirementFactory<SendOnly>>();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:FactoryOwnedRetirement::<SendOnly>::constructor_capacity_bytes(),maximum_depth:1,..Default::default()};let (result,(a,r))=observe_retirement_allocations(||FactoryOwnedRetirement::admit_original(&mut original,&factory,grant).unwrap());let (frame,p)=result.unwrap();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));let mut births=a;let mut released=r;let (_,heap)=observe_retirement_allocations(||drop(factory));assert_eq!(heap,(0,0));let mut frame=Some(frame as Box<dyn ErasedSnapshotRetirement>);
 for turn in 0..1000000{if frame.is_none(){break;}let demand=crate::factory_ticket_demands(frame.as_ref().unwrap(),1).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(1),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let (step,(a,r))=observe_retirement_allocations(||crate::close_factory_ticket(&mut frame,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));births+=a;released+=r;assert!(turn<999999);}
 assert!(frame.is_none());assert_eq!(released,held+births);let (_,heap)=observe_retirement_allocations(||drop((frame,original)));assert_eq!(heap,(0,0));eprintln!("[DEBUG] Factory owned genuine Cell send-only payload empty8192 retained backing; exact original issuer/frame/payload releases; terminalDrop=0");
}
