use super::*;
use crate::retirement::controlled::ControlledRetirement;
struct NativeLeaf{payload:String,pointer:usize}
impl FactoryPayloadRetirement for NativeLeaf{
 type CloseState=Option<ControlledRetirement<String>>;
 fn close_state_birth_bytes(&self)->usize{0}
 fn close_state_constructor_depth(&self)->usize{0}
 fn close_state_constructor_copy_bytes(&self)->usize{0}
 fn prepare_close_state(&self)->Self::CloseState{None}
 fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}
 fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
 fn transfer_payload(value:Self,state:&mut Self::CloseState){assert_eq!(value.payload.as_ptr()as usize,value.pointer);*state=Some(ControlledRetirement::new(value.payload).unwrap());}
 fn close_state_demands(state:&Self::CloseState,body:usize)->Result<RetirementDemand,ValueError>{match state{None=>Ok(Default::default()),Some(owner)if owner.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()}),Some(owner)=>Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}}
 fn close_state_step(state:&mut Self::CloseState,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if state.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=Self::close_state_demands(state,grant.maximum_copy_bytes)?;if !permits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}if state.as_ref().unwrap().terminal_is_empty(){*state=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}state.as_mut().unwrap().step(grant)}
 fn close_state_terminal_is_empty(state:&Self::CloseState)->bool{state.is_none()}
}
#[derive(crate::FactoryPayloadRetirement)]
struct NativeParent{#[factory_child] child:Arc<dyn FactoryRetirement>,tag:usize}
fn denied_grants(d:RetirementDemand,g:RetainedCloneGrant)->[Option<RetainedCloneGrant>;5]{[Some(RetainedCloneGrant{maximum_items:0,..g}),d.copy_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_copy_bytes:v,..g}),d.capacity_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_capacity_bytes:v,..g}),d.release_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_release_bytes:v,..g}),d.depth.checked_sub(1).map(|v|RetainedCloneGrant{maximum_depth:v,..g})]}
#[test]
fn original_factory_tickets_prepare_same_children_and_cancel_with_fixed_full_grants(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
 for levels in law["children"].as_array().unwrap(){for repeats in law["payloadRepeats"].as_array().unwrap(){let levels=levels.as_u64().unwrap()as usize;let repeats=repeats.as_u64().unwrap()as usize;
  for cancel in 0..=levels*4+1 {
   let(source,born)=crate::value::observe_retirement_allocations(||{let payload="original🧩".repeat(repeats);let pointer=payload.as_ptr()as usize;let mut source:Arc<dyn FactoryRetirement>=Arc::new(NativeLeaf{payload,pointer});for tag in 0..levels{source=Arc::new(NativeParent{child:source,tag});}source});let address=Arc::as_ptr(&source)as*const()as usize;let d=RetirementDemand{copy_bytes:source.factory_retirement_copy_byte_demand(),capacity_bytes:source.factory_retirement_birth_bytes(),depth:source.factory_retirement_depth_demand(),..Default::default()};let mut source=Some(source);
   let expected_copy=if levels==0{size_of::<super::super::FactoryTicket<NativeLeaf>>()+size_of::<<NativeLeaf as FactoryPayloadRetirement>::CloseState>()}else{size_of::<super::super::FactoryTicket<NativeParent>>()+size_of::<<NativeParent as FactoryPayloadRetirement>::CloseState>()};assert_eq!(d.copy_bytes,expected_copy);
   for denied in denied_grants(d,grant).into_iter().flatten(){let(result,heap)=crate::value::observe_retirement_allocations(||source.take().unwrap().preborn_factory_retirement(denied));let e=result.err().expect("original factory admission must refuse before birth");assert_eq!(e.progress,Default::default());assert!(e.ticket.is_none());assert_eq!(heap,(0,0));source=e.original;assert_eq!(Arc::as_ptr(source.as_ref().unwrap())as*const()as usize,address);}
   let((ticket,p),heap)=crate::value::observe_retirement_allocations(||source.take().unwrap().preborn_factory_retirement(grant).unwrap());assert_eq!(p.copied_bytes,d.copy_bytes);assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));let mut slot=Some(ticket);let(mut births,mut releases,mut turns)=(born.0+heap.0,born.1+heap.1,0);
   while turns<cancel&&!slot.as_ref().unwrap().preparation_is_complete(){let ticket=slot.as_mut().unwrap();let(d,heap)=crate::value::observe_retirement_allocations(||ticket.preparation_demands(grant.maximum_copy_bytes).unwrap());assert!(d.copy_bytes>0);assert_eq!(heap,(0,0));for denied in denied_grants(d,grant).into_iter().flatten(){let(step,heap)=crate::value::observe_retirement_allocations(||ticket.prepare_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}let(step,heap)=crate::value::observe_retirement_allocations(||ticket.prepare_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.0;releases+=heap.1;turns+=1;}
   while let Some(ticket)=slot.as_ref(){let(d,heap)=crate::value::observe_retirement_allocations(||factory_ticket_demands(ticket,grant.maximum_copy_bytes).unwrap());assert_eq!(heap,(0,0));for denied in denied_grants(d,grant).into_iter().flatten(){let(step,heap)=crate::value::observe_retirement_allocations(||close_factory_ticket(&mut slot,denied));assert_eq!(heap,(0,0));match step{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(e)=>assert_eq!(e.retained_progress(),Default::default())}}let(step,heap)=crate::value::observe_retirement_allocations(||close_factory_ticket(&mut slot,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.0;releases+=heap.1;turns+=1;assert!(turns<100000);}
   let(_,heap)=crate::value::observe_retirement_allocations(||drop(slot));assert_eq!(heap,(0,0));assert_eq!(births,releases);println!("[DEBUG] original factory levels{levels} repeats{repeats} cancel{cancel} turns{turns} pointer{address} physical{births}/{releases} fixed5currency source/partialTicket retained, terminalDrop0");
  }
 }}
}
