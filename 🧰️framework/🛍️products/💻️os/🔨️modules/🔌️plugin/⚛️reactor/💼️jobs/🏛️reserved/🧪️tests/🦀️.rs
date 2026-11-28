//! 🧪️ Actual original Reserved bodies share neutral bytes, one-unit work and paid source closure.
use super::*;
use std::mem::size_of;
use semio_plugin_testing_laws::OriginalWorkFixture;
use crate::{BoundedJob,JobStep};
use semio_framework_job::{OriginalConstructionDemand,OriginalConstructionAdmission,admit_original_construction_settled,original_construction_settled_turn_quote,close_original_retained_step};
use semio_framework_value::RetainedCloneStep;
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128}}
use crate::registry_driver::{widest,source_grant};
/// 🏭️ The grant and fuel one turn needs to settle the output construction of a body that copies `length` bytes: the settled construction quote, never the one-unit grant of the copy steps.
fn construction_turn(length:usize)->(RetainedCloneGrant,u64){let(quote,fuel)=original_construction_settled_turn_quote::<Vec<u8>>(OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:length,release_bytes:0,depth:1},1).unwrap();(widest(grant(),quote),fuel.max(1))}
fn caller()->OriginalWorkFixture{OriginalWorkFixture::with_source(semio_framework_job::OperationId(71),semio_framework_job::Generation(3),stock(),||Some(1),1000000,100000)}
fn difference(after:RetainedCloneProgress,before:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}}
fn birth(owner:&mut OriginalWorkFixture,input:Vec<u8>)->Box<FrameworkReservedBoundedJob>{
 let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:size_of::<FrameworkReservedBoundedJob>(),release_bytes:0,depth:1};
 let(turn,fuel)=original_construction_settled_turn_quote::<Box<FrameworkReservedBoundedJob>>(demand,1).unwrap();
 let(mut output,mut admission,mut input)=(None,OriginalConstructionAdmission::default(),Some(input));
 for _ in 0..16{
  let settled=owner.work_turn(turn,fuel,|cx|{let binding=semio_framework_job::OriginalSourceBinding::borrow(cx);admit_original_construction_settled(cx,&mut admission,&mut output,demand,1,|_,_|semio_framework_value::RetainedWorkOutcome{result:Ok((Box::new(FrameworkReservedBoundedJob::new(input.take().expect("one construction call"),None,binding)),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:size_of::<FrameworkReservedBoundedJob>(),..Default::default()})),work:1})}).unwrap();
  if settled{return output.take().unwrap()}
 }
 panic!("the reserved body must settle within its quoted turns")
}
fn funded(demand:RetirementDemand)->RetainedCloneGrant{crate::registry_driver::fund_demand(grant(),demand)}
fn slot_demand(slot:&super::super::JobSlot,failed:bool)->RetirementDemand{let copy=grant().maximum_copy_bytes;if failed{slot.failed_owner.as_ref().unwrap().retirement_demands(copy).unwrap()}else{match slot.body.as_ref(){Some(super::super::JobBody::Bounded(owner))=>owner.retirement_demands(copy).unwrap(),_=>Default::default()}}}
fn close_body(owner:&mut OriginalWorkFixture,body:&mut Option<Box<FrameworkReservedBoundedJob>>){
 for _ in 0..10000{if body.as_ref().unwrap().terminal_drop_is_shallow(){break}let turn=funded(body.as_ref().unwrap().retirement_demands(grant().maximum_copy_bytes).unwrap());let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(turn,1,|cx|cx.with_original_retirement(|cx|{let before=cx.retained_progress();body.as_mut().unwrap().close_step(cx)?;Ok(difference(cx.retained_progress(),before))})).unwrap());assert!(progress.fits(turn));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
 assert!(body.as_ref().unwrap().terminal_drop_is_shallow());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(grant(),1,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,RetirementDemand{release_bytes:size_of::<FrameworkReservedBoundedJob>(),depth:1,..Default::default()},|_|{drop(body.take());semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:size_of::<FrameworkReservedBoundedJob>(),..Default::default()})),work:1}})?;Ok(())})).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,size_of::<FrameworkReservedBoundedJob>()));
}
fn close_output(owner:&mut OriginalWorkFixture,output:&mut Option<Vec<u8>>){let mut closing=None;for _ in 0..10000{let turn=source_grant(grant(),output,&closing);if owner.work_turn(turn,1,|cx|cx.with_original_retirement(|cx|super::super::close_original(output,&mut closing,cx))).unwrap(){return}}panic!("original output close must finish")}
#[test]
fn reserved_registry_original_body_bytes_and_actual_receipts_share_one_source(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in law["cases"].as_array().unwrap(){let input:Vec<u8>=serde_json::from_value(row["input"].clone()).unwrap();let expected:Vec<u8>=serde_json::from_value(row["output"].clone()).unwrap();let(step_grant,step_fuel)=construction_turn(input.len());let mut owner=caller();let mut body=Some(birth(&mut owner,input));let mut output=None;
  let mut foreign=caller();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.work_turn(grant(),1,|cx|body.as_mut().unwrap().step_original(cx)));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));foreign.finish();
  for _ in 0..10000{let((step,progress,spent,phase),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(step_grant,step_fuel,|cx|{let phase=body.as_ref().unwrap().phase;let before=cx.retained_progress();let fuel=cx.fuel_remaining();let step=body.as_mut().unwrap().step_original(cx)?;Ok((step,difference(cx.retained_progress(),before),fuel-cx.fuel_remaining(),phase))}).unwrap());assert!(progress.fits(step_grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if progress!=Default::default()&&phase!=1{assert_eq!(spent,1)}match step{JobStep::Running(None)=>{},JobStep::Done(bytes)=>{output=Some(bytes);break},_=>panic!("original reserved body produced unrelated output")}}
  assert_eq!(output.as_ref().unwrap(),&expected);assert!(body.as_ref().unwrap().source.is_none()&&body.as_ref().unwrap().source_close.is_none());close_body(&mut owner,&mut body);close_output(&mut owner,&mut output);owner.finish();
 }
}
#[test]
fn reserved_registry_original_body_work_denials_preserve_input_and_last_unit_progress(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in law["work"].as_array().unwrap(){let mut owner=caller();let mut body=Some(birth(&mut owner,vec![97,98,99]));let pointer=body.as_ref().unwrap().source.as_ref().unwrap().0.as_ptr();if row["cancelled"].as_bool().unwrap(){owner.cancel().cancel_now();}let policy=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,..grant()};let((advanced,remaining),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(policy,row["fuel"].as_u64().unwrap(),|cx|{let mut run=|cx:&mut semio_framework_job::StepContext<'_>|{let before=cx.retained_progress();assert!(matches!(body.as_mut().unwrap().step_original(cx)?,JobStep::Running(None)));Ok((cx.retained_progress()!=before,cx.fuel_remaining()))};if row["expired"].as_bool().unwrap(){let fuel=cx.fuel_remaining();cx.with_original_work_loan(fuel,0,run)}else{run(cx)}}).unwrap());assert_eq!(advanced,row["advanced"].as_bool().unwrap());assert_eq!(remaining,row["remainingFuel"].as_u64().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(pointer,body.as_ref().unwrap().source.as_ref().unwrap().0.as_ptr());close_body(&mut owner,&mut body);owner.finish();}
}

#[test]
fn reserved_registry_original_partial_output_cancel_keeps_backings_until_same_source_close(){
 let mut owner=caller();let mut body=Some(birth(&mut owner,vec![81;65]));for _ in 0..100{if body.as_ref().unwrap().phase==2{break}let(construct,fuel)=construction_turn(65);owner.work_turn(construct,fuel,|cx|body.as_mut().unwrap().step_original(cx)).unwrap();}assert_eq!(body.as_ref().unwrap().phase,2);
 owner.work_turn(RetainedCloneGrant{maximum_copy_bytes:3,..grant()},1,|cx|body.as_mut().unwrap().step_original(cx)).unwrap();assert_eq!(body.as_ref().unwrap().output.as_ref().unwrap().len(),3);let source=body.as_ref().unwrap().source.as_ref().unwrap().0.as_ptr();let output=body.as_ref().unwrap().output.as_ref().unwrap().as_ptr();owner.cancel().cancel_now();
 let((progress,remaining),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(grant(),1,|cx|{let before=cx.retained_progress();assert!(matches!(body.as_mut().unwrap().step_original(cx)?,JobStep::Running(None)));Ok((difference(cx.retained_progress(),before),cx.fuel_remaining()))}).unwrap());assert_eq!(progress,Default::default());assert_eq!(remaining,1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source,body.as_ref().unwrap().source.as_ref().unwrap().0.as_ptr());assert_eq!(output,body.as_ref().unwrap().output.as_ref().unwrap().as_ptr());close_body(&mut owner,&mut body);owner.finish();
}

std::thread_local!{static RESERVED_CLOCK:std::cell::Cell<u64>=const{std::cell::Cell::new(1)};}
/// ⏳️ The Source's own absolute deadline in the fixture clock's microseconds; the Source admits its decoder only while its clock is further from it than one turn's wall quantum, so the expiry laws step the clock one past it.
const RESERVED_DEADLINE:u64=1_000_000;
fn reserved_now()->Option<u64>{Some(RESERVED_CLOCK.with(std::cell::Cell::get))}
#[test]
fn reserved_registry_original_expired_clock_closes_partial_output_without_renewal(){
 RESERVED_CLOCK.with(|clock|clock.set(1));let mut owner=OriginalWorkFixture::with_source(semio_framework_job::OperationId(71),semio_framework_job::Generation(3),stock(),reserved_now,RESERVED_DEADLINE,100000);let mut body=Some(birth(&mut owner,vec![81;65]));
 for _ in 0..100{if body.as_ref().unwrap().phase==2{break}let(construct,fuel)=construction_turn(65);owner.work_turn(construct,fuel,|cx|body.as_mut().unwrap().step_original(cx)).unwrap();}assert_eq!(body.as_ref().unwrap().phase,2);owner.work_turn(RetainedCloneGrant{maximum_copy_bytes:3,..grant()},1,|cx|body.as_mut().unwrap().step_original(cx)).unwrap();assert_eq!(body.as_ref().unwrap().output.as_ref().unwrap().len(),3);
 RESERVED_CLOCK.with(|clock|clock.set(RESERVED_DEADLINE+1));let((before,after),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(grant(),1,|cx|{assert!(cx.should_yield());let before=cx.retained_progress();assert!(matches!(body.as_mut().unwrap().step_original(cx)?,JobStep::Running(None)));assert!(!body.as_mut().unwrap().close_step(cx)?);Ok((before,cx.retained_progress()))}).unwrap());assert_eq!(before,after);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 for(fuel,items)in[(0,1),(1,0)]{let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(RetainedCloneGrant{maximum_items:items,..grant()},fuel,|cx|cx.with_original_retirement(|cx|{assert!(cx.should_yield());let before=cx.retained_progress();assert!(!body.as_mut().unwrap().close_step(cx)?);Ok(difference(cx.retained_progress(),before))})).unwrap());assert_eq!(progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
 close_body(&mut owner,&mut body);assert_eq!(reserved_now(),Some(RESERVED_DEADLINE+1));owner.finish();assert_eq!(reserved_now(),Some(RESERVED_DEADLINE+1));
}

#[test]
fn reserved_registry_original_expired_clock_closes_failed_and_bounded_slot_owners(){
 for failed in[false,true]{RESERVED_CLOCK.with(|clock|clock.set(1));let mut owner=OriginalWorkFixture::with_source(semio_framework_job::OperationId(71),semio_framework_job::Generation(3),stock(),reserved_now,RESERVED_DEADLINE,100000);let original=birth(&mut owner,vec![81;65]);let binding=original.binding;let original:Box<dyn BoundedJob>=original;let(body,failed_owner)=if failed{(None,Some(original))}else{(Some(super::super::JobBody::Bounded(original)),None)};
  let mut slot=super::super::JobSlot{binding,job:71,identity:(71,3),source:None,source_close:None,body,failed_owner,outcome:None,outcome_close:None,rejected_output:None,rejected_output_close:None,cancelled:false,cancellation_fault_started:false,cancellation_signalled:false};RESERVED_CLOCK.with(|clock|clock.set(RESERVED_DEADLINE+1));
  let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(grant(),1,|cx|{let before=cx.retained_progress();assert!(!if failed{super::super::close_failed_owner(&mut slot,cx)?}else{super::super::close_body(&mut slot,cx)?});Ok(difference(cx.retained_progress(),before))}).unwrap());assert_eq!(progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for _ in 0..10000{if slot.body.is_none()&&slot.failed_owner.is_none(){break}let turn=funded(slot_demand(&slot,failed));let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(turn,1,|cx|cx.with_original_retirement(|cx|{assert!(cx.should_yield());let before=cx.retained_progress();if failed{super::super::close_failed_owner(&mut slot,cx)?;}else{super::super::close_body(&mut slot,cx)?;}Ok(difference(cx.retained_progress(),before))})).unwrap());assert!(progress.fits(turn));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
  assert!(slot.body.is_none()&&slot.failed_owner.is_none());assert_eq!(reserved_now(),Some(RESERVED_DEADLINE+1));owner.finish();assert_eq!(reserved_now(),Some(RESERVED_DEADLINE+1));
 }
}

#[test]
fn reserved_registry_original_global_close_rejects_foreign_source_and_keeps_expired_clock(){
 use semio_framework_actor::OriginalActorJobProvider;
 crate::tests::warm_registry();
 RESERVED_CLOCK.with(|clock|clock.set(1));let mut owner=OriginalWorkFixture::with_source(semio_framework_job::OperationId(71),semio_framework_job::Generation(3),stock(),reserved_now,RESERVED_DEADLINE,100000);let mut foreign=OriginalWorkFixture::with_source(semio_framework_job::OperationId(71),semio_framework_job::Generation(3),stock(),reserved_now,RESERVED_DEADLINE,100000);let job=65571;let mut admission=Some(super::super::OriginalJobAdmission{job,kind:FRAMEWORK_RESERVED_JOB_KIND.into(),input:Some(vec![81;65]),checkpoint:None});
 let((admitted,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(grant(),1,|cx|{let before=cx.retained_progress();let admitted=semio_framework_async::poll::resolve_ready(super::super::start_job(&mut admission,cx))?;Ok((admitted,difference(cx.retained_progress(),before)))}).unwrap());assert!(admitted&&admission.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));let mut provider=super::super::RegisteredOriginalActorJobs::borrow_original(71,3);
 RESERVED_CLOCK.with(|clock|clock.set(RESERVED_DEADLINE+1));let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.work_turn(grant(),1,|cx|{assert!(super::super::job_admission_demands(job,cx).is_err());assert!(provider.cancel(job,cx).is_err());assert!(provider.step(job,cx).is_err());assert!(provider.close(job,cx).is_err());Ok(())}).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(super::super::original_job_is_cancelled(job),Some(false));foreign.finish();
 let mut closed=false;for _ in 0..10000{let turn=owner.work_turn(grant(),1,|cx|Ok(crate::registry_driver::closing_grant(job,grant(),true,cx)?)).unwrap();let((done,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(turn,1,|cx|{assert!(cx.should_yield());let before=cx.retained_progress();let done=provider.close(job,cx)?;Ok((done,difference(cx.retained_progress(),before)))}).unwrap());assert!(progress.fits(turn));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if done{closed=true;break}}
 assert!(closed);assert!(!super::super::has_owned_jobs(71,3));assert_eq!(reserved_now(),Some(RESERVED_DEADLINE+1));owner.finish();assert_eq!(reserved_now(),Some(RESERVED_DEADLINE+1));
}

#[test]
fn reserved_registry_original_child_release_denial_preserves_last_work_unit(){
 let mut owner=caller();let mut source=Some(vec![81u8;65]);let mut closing=None;let mut found=false;
 for _ in 0..10000{if closing.as_ref().is_some_and(|close:&ControlledRetirement<Vec<u8>>|close.next_demand(grant().maximum_copy_bytes).unwrap().release_bytes>0){found=true;break}let turn=source_grant(grant(),&source,&closing);let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(turn,1,|cx|cx.with_original_retirement(|cx|{let before=cx.retained_progress();super::super::close_original(&mut source,&mut closing,cx)?;Ok(difference(cx.retained_progress(),before))})).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
 assert!(found);let((progress,fuel),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(RetainedCloneGrant{maximum_release_bytes:0,..source_grant(grant(),&source,&closing)},1,|cx|cx.with_original_retirement(|cx|{let before=cx.retained_progress();assert!(!super::super::close_original(&mut source,&mut closing,cx)?);Ok((difference(cx.retained_progress(),before),cx.fuel_remaining()))})).unwrap());assert_eq!(progress,Default::default());assert_eq!(fuel,1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 for _ in 0..10000{if source.is_none()&&closing.is_none(){break}let turn=source_grant(grant(),&source,&closing);let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.work_turn(turn,1,|cx|cx.with_original_retirement(|cx|{let before=cx.retained_progress();super::super::close_original(&mut source,&mut closing,cx)?;Ok(difference(cx.retained_progress(),before))})).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}assert!(source.is_none()&&closing.is_none());owner.finish();
}

/// 📏️ The most host crossings (`step-job` calls, each with its own turn issue, receipt and result message) one selection or tool-switch dispatch may spend on its reserved job; the measured 3D click needed about 130 before the job step was coarsened.
const SELECTION_JOB_CROSSINGS_MAX:u32=2;
/// 🎚️ Wide per-crossing fuel, so only the micro step cap and the grant bound a crossing.
const CROSSING_FUEL:u64=1<<16;
fn crossings_to_done(payload:usize,step:fn(u64,&mut semio_framework_job::StepContext<'_>)->Result<Option<JobStep>,OriginalTurnFailure>)->(u32,Vec<u8>){
 let mut input=FRAMEWORK_RESERVED_JOB_MAGIC.to_vec();let body:Vec<u8>=(0..payload).map(|index|index as u8).collect();input.extend_from_slice(&[0u8;8]);input.extend_from_slice(&body);
 let mut owner=OriginalWorkFixture::with_source(semio_framework_job::OperationId(71),semio_framework_job::Generation(3),stock(),||Some(1),1000000,10_000_000);let job=65601;let mut admission=Some(super::super::OriginalJobAdmission{job,kind:FRAMEWORK_RESERVED_JOB_KIND.into(),input:Some(input),checkpoint:None});
 let mut admitted=false;for _ in 0..1000{admitted=owner.work_turn(stock(),CROSSING_FUEL,|cx|Ok(semio_framework_async::poll::resolve_ready(super::super::start_job(&mut admission,cx))?)).unwrap();if admitted{break}}assert!(admitted);
 let mut crossings=0;for _ in 0..1000{crossings+=1;if let Some(JobStep::Done(bytes))=owner.work_turn(stock(),CROSSING_FUEL,|cx|step(job,cx)).unwrap(){owner.finish();assert_eq!(bytes,body);return(crossings,bytes)}}
 panic!("the reserved job must reach its terminal step")
}
#[test]
fn a_selection_or_tool_switch_reserved_job_settles_in_a_bounded_number_of_crossings(){
 for payload in [0usize,200,6144]{
  let(coarse,_)=crossings_to_done(payload,super::super::abi::step_coarse);
  assert!(coarse<=SELECTION_JOB_CROSSINGS_MAX,"{payload} payload bytes took {coarse} crossings");
  let(micro,_)=crossings_to_done(payload,super::super::abi::step);
  assert!(micro as usize>=8+payload.div_ceil(64),"one micro step per magic byte (8) and 64-byte page is what the coarse crossing folds ({micro})");
  assert!(payload<6144||micro>coarse*20);
 }
}

fn reserved_input(payload:usize)->Vec<u8>{let mut input=FRAMEWORK_RESERVED_JOB_MAGIC.to_vec();input.extend_from_slice(&[0u8;8]);input.extend((0..payload).map(|index|index as u8));input}
fn coarse_owner(job:u64,payload:usize)->OriginalWorkFixture{
 let mut owner=OriginalWorkFixture::with_source(semio_framework_job::OperationId(71),semio_framework_job::Generation(3),stock(),||Some(1),1000000,10_000_000);let mut admission=Some(super::super::OriginalJobAdmission{job,kind:FRAMEWORK_RESERVED_JOB_KIND.into(),input:Some(reserved_input(payload)),checkpoint:None});
 let mut admitted=false;for _ in 0..1000{admitted=owner.work_turn(stock(),CROSSING_FUEL,|cx|Ok(semio_framework_async::poll::resolve_ready(super::super::start_job(&mut admission,cx))?)).unwrap();if admitted{break}}assert!(admitted);owner
}
/// 🚀️ One coarse crossing under `grant` and an absolute `deadline`: its step, what it retained and how much fuel it spent.
fn crossing(owner:&mut OriginalWorkFixture,job:u64,grant:RetainedCloneGrant,deadline:u64)->(Option<JobStep>,RetainedCloneProgress,u64){
 owner.turn(grant,|cx|{let before=cx.retained_progress();let fuel=cx.fuel_remaining().min(CROSSING_FUEL);let mut step=None;cx.with_original_work_loan(fuel,deadline,|cx|{step=super::super::abi::step_coarse(job,cx)?;Ok(())})?;Ok((step,difference(cx.retained_progress(),before),fuel-cx.fuel_remaining().min(fuel)))}).unwrap()
}
/// 🏁️ Drives coarse crossings until the job is done, returning how many it took and its exact bytes.
fn settle(owner:OriginalWorkFixture,job:u64)->(u32,Vec<u8>){let mut owner=owner;for crossings in 1..=64{if let(Some(JobStep::Done(bytes)),..)=crossing(&mut owner,job,stock(),u64::MAX){owner.finish();return(crossings,bytes)}}panic!("the reserved job must settle within a bounded number of coarse crossings")}
/// ⏳️ A coarse crossing hands the slice back at an expired deadline before it runs any micro step, and the same job then settles under a live one.
#[test]
fn a_coarse_crossing_runs_no_micro_step_at_an_expired_deadline(){
 let job=65611;let mut owner=coarse_owner(job,6144);
 let(step,progress,spent)=crossing(&mut owner,job,stock(),0);
 assert!(step.is_none());assert_eq!(progress,Default::default());assert_eq!(spent,0);
 let(_,bytes)=settle(owner,job);assert_eq!(bytes,(0..6144).map(|index|index as u8).collect::<Vec<u8>>());
}
/// 🧱️ A crossing whose micro step makes no byte progress returns instead of spinning, and the same job later settles when funded.
#[test]
fn a_coarse_crossing_returns_when_its_micro_step_makes_no_progress(){
 let job=65612;let mut owner=coarse_owner(job,200);
 let(step,progress,spent)=crossing(&mut owner,job,RetainedCloneGrant{maximum_copy_bytes:0,..stock()},u64::MAX);
 assert!(step.is_none());assert_eq!(progress.copied_bytes,0);assert!(spent<=COARSE_STALL_FUEL_MAX);
 let(_,bytes)=settle(owner,job);assert_eq!(bytes.len(),200);
}
/// ⛽️ The most fuel a starved crossing may spend on the one micro step that made no byte progress before it stops.
const COARSE_STALL_FUEL_MAX:u64=4;
/// 🪜️ A payload beyond the micro step cap spans several capped crossings and still delivers the exact bytes.
#[test]
fn a_coarse_crossing_is_capped_and_a_long_payload_spans_several_crossings(){
 let job=65613;let payload=64*(super::super::abi::COARSE_STEP_CAP as usize+64);
 let(crossings,bytes)=settle(coarse_owner(job,payload),job);
 assert_eq!(bytes,(0..payload).map(|index|index as u8).collect::<Vec<u8>>());assert!(crossings>=2,"{payload} bytes need more than one capped crossing, took {crossings}");
}
