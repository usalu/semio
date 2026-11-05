use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,StepContext,OriginalTurnFailure,OriginalConstructionDemand,OperationId,Generation,admit_original_construction,close_original_retained_step};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(44779),Generation(8),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096},||Some(1),u64::MAX,1<<20)}
#[track_caller]
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));
 let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(g));result.unwrap()
}
fn advance(owner:&mut OriginalResultPageBacking,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,body:usize){
 let d=if owner.terminal_is_empty(){OriginalResultPageBacking::birth_demands()}else{owner.initialization_demands(body).unwrap()};
 let g=grant(d);
 turn(caller,decoder,g,|cx|{
  let mut done=None;
  assert!(admit_original_construction(cx,&mut done,construction(d),|_|{let step=if owner.terminal_is_empty(){owner.birth_step(g)}else{owner.initialize_step(g)};RetainedWorkOutcome{work:step.work,result:step.result.map(|step|((),step.progress()))}})?);
  Ok(())
 });
}
fn denied(owner:&mut OriginalResultPageBacking,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,d:RetirementDemand,closing:bool,seen:&mut[bool;5]){
 let g=grant(d);
 for axis in 0..5{
  let quantity=[1,d.copy_bytes,d.capacity_bytes,d.release_bytes,d.depth][axis];if quantity==0{continue}
  let mut denied=g;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=0,2=>denied.maximum_capacity_bytes-=1,3=>denied.maximum_release_bytes-=1,_=>denied.maximum_depth-=1}
  let before=owner.initialized_len();
  turn(caller,decoder,denied,|cx|{let step=if closing{owner.close_step(denied)}else if owner.terminal_is_empty(){owner.birth_step(denied)}else{owner.initialize_step(denied)};assert_eq!(step.work,0);let p=step.result?.progress();assert_eq!(p,Default::default());cx.consume_retained(p)?;Ok(())});assert_eq!(owner.initialized_len(),before);seen[axis]=true;
 }
}
fn close(owner:&mut OriginalResultPageBacking,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,seen:&mut[bool;5]){
 for _ in 0..65536{
  if owner.terminal_is_empty(){return}
  let d=owner.close_demands(4096).unwrap();denied(owner,caller,decoder,d,true,seen);let g=grant(d);
  turn(caller,decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|owner.close_step(issued))?;Ok(())}));
 }
 panic!("original result backing close stalled")
}
fn retire(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 for _ in 0..65536{
  if decoder.terminal_is_empty(){break}
  let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096};
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(g,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(g.maximum_copy_bytes)?;close_original_retained_step(cx,d,|issued|decoder.close_step(issued))?;Ok(())})));
  let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap();
 }
 assert!(decoder.terminal_is_empty());caller.retire(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096});
}

#[test]
fn original_result_backing_birth_initialization_handoff_and_partial_close_are_physically_funded(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/📃️result-backing/🔣️.json")).unwrap();
 let mut seen=[false;5];
 for target in law["closeAfter"].as_array().unwrap().iter().map(|row|row.as_u64().unwrap()as usize){
  let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=OriginalResultPageBacking::empty();
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
   turn(&mut caller,&mut decoder,g,|cx|{assert!(admit_original_construction(cx,&mut page,construction(d),|_|{let turn=owner.take_page(token,TypedOperationResultLane::Terminal,3,g);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|value|value.expect("admitted original page handoff"))}})?);Ok(())});
   assert!(owner.terminal_is_empty());let mut page=page.unwrap();assert_eq!(page.bytes(),b"abc");assert_eq!(page.bytes().as_ptr(),pointer);
   for _ in 0..65536{if page.terminal_is_empty(){break}let d=page.next_close_demand(4096).unwrap();let g=grant(d);turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|RetainedWorkOutcome{result:page.close_step(issued),work:1})?;Ok(())}));}
   assert!(page.terminal_is_empty());
  }else{
   caller.cancel().cancel_now();
   close(&mut owner,&mut caller,&mut decoder,&mut seen);
  }
  assert!(owner.terminal_is_empty());retire(&mut caller,&mut decoder);
 }
 assert!(seen.into_iter().all(|seen|seen),"all five real currencies deny an unchanged owner");
}

#[test]
fn original_result_backing_survives_borrowed_unwind_and_can_be_born_in_cancelled_retirement(){
 let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=OriginalResultPageBacking::empty();let d=OriginalResultPageBacking::birth_demands();caller.cancel().cancel_now();
 turn(&mut caller,&mut decoder,grant(d),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|owner.birth_step(issued))?;Ok(())}));
 assert_eq!(owner.initialized_len(),0);assert!(owner.initialized_bytes().is_some());
 let pointer=owner.initialized_bytes().unwrap().as_ptr();
 assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let _retained=&mut owner;panic!("borrowed producer unwind");})).is_err());
 assert_eq!(owner.initialized_bytes().unwrap().as_ptr(),pointer);
 let mut seen=[false;5];close(&mut owner,&mut caller,&mut decoder,&mut seen);assert!(owner.terminal_is_empty());retire(&mut caller,&mut decoder);
}
