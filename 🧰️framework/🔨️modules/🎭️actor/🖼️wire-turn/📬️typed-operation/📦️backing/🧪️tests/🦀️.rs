use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,StepContext,OriginalTurnFailure,OriginalConstructionDemand,OriginalConstructionAdmission,OriginalConstructionStatus,OperationId,Generation,admit_original_construction,close_original_retained_effect_step};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(44779),Generation(8),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096},||Some(1),u64::MAX,1<<20)}
thread_local!{static BANK:std::cell::RefCell<semio_framework_value_testing_admission::EffectBank>=std::cell::RefCell::new(semio_framework_value_testing_admission::EffectBank::new(semio_framework_value_testing_admission::Wallet::new(RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<20},1<<40)));}
fn fx<T>(grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->T)->T{BANK.with(|bank|{let mut bank=bank.borrow_mut();let mut run=Some(run);let mut slot=None;let(poll,_)=semio_framework_value_testing_admission::driven::<ValueError>(&mut bank,RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<20},1<<20,|effect,reply,_|{slot=Some((run.take().expect("single driven action"))(&mut effect.nested(grant)));Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))});match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.expect("driven action"),_=>panic!("driven action stayed pending")};slot.take().expect("driven action ran")})}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096}}
#[track_caller]
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));
 let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()
}
fn bind(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){for _ in 0..16{if caller.turn(stock(),|cx|Ok(decoder.admit_step(cx)?)).unwrap(){break}}assert!(decoder.source_loan_admitted())}
fn construct<O>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,output:&mut Option<O>,d:RetirementDemand,build:&mut dyn FnMut()->(RetainedWorkOutcome<(O,RetainedCloneProgress)>,usize)){
 let mut admission=OriginalConstructionAdmission::default();
 for _ in 0..32{
  let status=turn(caller,decoder,stock(),|cx|admit_original_construction(cx,output,&mut admission,construction(d),1,|_,actual|{let(outcome,bytes)=build();*actual=Some(bytes);outcome}));
  if status==OriginalConstructionStatus::Ready{break}assert_eq!(status,OriginalConstructionStatus::Pending);
 }
 assert!(output.is_some(),"admitted original construction completed");
 for _ in 0..10000{if admission.terminal_is_empty(){return}turn(caller,decoder,stock(),|cx|cx.with_original_retirement(|cx|{admission.close_step(cx)?;Ok(())}));}
 panic!("construction control close must terminate")
}
fn advance(owner:&mut OriginalResultPageBacking,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,body:usize){
 let d=if owner.terminal_is_empty(){OriginalResultPageBacking::birth_demands()}else{owner.initialization_demands(body).unwrap()};
 let g=grant(d);let mut done=None::<()>;
 construct(caller,decoder,&mut done,d,&mut||{let step=if owner.terminal_is_empty(){owner.birth_step(g)}else{owner.initialize_step(g)};let capacity=step.result.as_ref().map_or(0,|step|step.progress().retained_capacity_bytes);(RetainedWorkOutcome{work:step.work,result:step.result.map(|step|((),step.progress()))},capacity)});
}
fn denied(owner:&mut OriginalResultPageBacking,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,d:RetirementDemand,closing:bool,seen:&mut[bool;5]){
 let g=grant(d);
 for axis in 0..5{
  let quantity=[1,d.copy_bytes,d.capacity_bytes,d.release_bytes,d.depth][axis];if quantity==0{continue}
  let mut denied=g;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=0,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1}
  let before=owner.initialized_len();
  turn(caller,decoder,stock(),|cx|{
   let step=if closing{fx(denied,|effect|owner.close_step(effect))}else if owner.terminal_is_empty(){owner.birth_step(denied)}else{owner.initialize_step(denied)};
   assert_eq!(step.work,0);assert_eq!(step.result?.progress(),Default::default());Ok(())
  });assert_eq!(owner.initialized_len(),before);seen[axis]=true;
 }
}
fn close(owner:&mut OriginalResultPageBacking,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,seen:&mut[bool;5]){
 for _ in 0..65536{
  if owner.terminal_is_empty(){return}
  let d=owner.close_demands(4096).unwrap();denied(owner,caller,decoder,d,true,seen);
  fx(grant(d),|effect|owner.close_step(effect)).result.unwrap();
 }
 panic!("original result backing close stalled")
}
fn retire(mut caller:OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 for _ in 0..65536{if decoder.terminal_is_empty(){break}caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();}
 assert!(decoder.terminal_is_empty());caller.retire(stock());
}

#[test]
fn original_result_backing_birth_initialization_handoff_and_partial_close_are_physically_funded(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/📃️result-backing/🔣️.json")).unwrap();
 let mut seen=[false;5];
 for target in law["closeAfter"].as_array().unwrap().iter().map(|row|row.as_u64().unwrap()as usize){
  let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();bind(&mut caller,&mut decoder);let mut owner=OriginalResultPageBacking::empty();
  denied(&mut owner,&mut caller,&mut decoder,OriginalResultPageBacking::birth_demands(),false,&mut seen);advance(&mut owner,&mut caller,&mut decoder,4096);
  let pointer=owner.initialized_bytes().unwrap().as_ptr();
  for body in [0,size_of::<usize>()-1,size_of::<usize>()]{
   let d=owner.initialization_demands(body).unwrap();assert!(d.copy_bytes>body);
   let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.initialize_step(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(step.work,0);assert_eq!(step.result.unwrap().progress(),Default::default());assert_eq!(owner.initialized_len(),0);
  }
  while owner.initialized_len()<target{let remaining=target-owner.initialized_len();advance(&mut owner,&mut caller,&mut decoder,remaining.min(2048)+size_of::<usize>());assert_eq!(owner.initialized_bytes().unwrap().as_ptr(),pointer);}
  assert_eq!(owner.initialized_len(),target);
  if target==4096{
   owner.initialized_bytes().unwrap()[..3].copy_from_slice(b"abc");
   let d=owner.page_demands().unwrap();let g=grant(d);let token=TypedOperationResultToken{receiver:1,operation:7,generation:8,sequence:0,attempt:0};
   let denied=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-1,..g};
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.take_page(token,TypedOperationResultLane::Terminal,3,denied));assert!(result.result.unwrap().is_none());assert_eq!(result.work,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.initialized_bytes().unwrap().as_ptr(),pointer);
   let mut page=None;
   construct(&mut caller,&mut decoder,&mut page,d,&mut||{let turn=owner.take_page(token,TypedOperationResultLane::Terminal,3,g);(RetainedWorkOutcome{work:turn.work,result:turn.result.map(|value|value.expect("admitted original page handoff"))},0)});
   assert!(owner.terminal_is_empty());let mut page=page.unwrap();assert_eq!(page.bytes(),b"abc");assert_eq!(page.bytes().as_ptr(),pointer);
   for _ in 0..65536{if page.terminal_is_empty(){break}let d=page.next_close_demand(4096).unwrap();fx(grant(d),|effect|page.close_step(effect)).unwrap();}
   assert!(page.terminal_is_empty());
  }else{
   caller.cancel().cancel_now();
   close(&mut owner,&mut caller,&mut decoder,&mut seen);
  }
  assert!(owner.terminal_is_empty());retire(caller,&mut decoder);
 }
 assert!(seen.into_iter().all(|seen|seen),"all five real currencies deny an unchanged owner");
}

#[test]
fn original_result_backing_survives_borrowed_unwind_and_can_be_born_in_cancelled_retirement(){
 let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();bind(&mut caller,&mut decoder);let mut owner=OriginalResultPageBacking::empty();let d=OriginalResultPageBacking::birth_demands();caller.cancel().cancel_now();
 turn(&mut caller,&mut decoder,stock(),|cx|cx.with_original_retirement(|cx|{close_original_retained_effect_step(cx,d,|effect|owner.birth_step(effect.grant()).result)?;Ok(())}));
 assert_eq!(owner.initialized_len(),0);assert!(owner.initialized_bytes().is_some());
 let pointer=owner.initialized_bytes().unwrap().as_ptr();
 assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let _retained=&mut owner;panic!("borrowed producer unwind");})).is_err());
 assert_eq!(owner.initialized_bytes().unwrap().as_ptr(),pointer);
 let mut seen=[false;5];close(&mut owner,&mut caller,&mut decoder,&mut seen);assert!(owner.terminal_is_empty());retire(caller,&mut decoder);
}
