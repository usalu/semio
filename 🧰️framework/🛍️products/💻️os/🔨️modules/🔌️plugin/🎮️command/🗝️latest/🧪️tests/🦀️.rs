//! 🗝️ Native original key framing, funded turns, actual receipts and physical backing conservation.
use crate::command_admission::original_latest_key::ToolLatestWinsKeyCopy;
use semio_framework_job::{Generation,OperationId,OriginalJobDecodeOwner,OriginalJobFixtureCaller,OriginalTurnFailure,StepContext,default_now_us};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retained_clone::{RetainedCloneClose,RetainedCloneStep};
use semio_framework_value_ordered::ordered::SharedOwner;
use semio_plugin_job::original_steps::{begin,retired};
use semio_plugin_ownership::bounded_ownership::plugin_settle_value;
const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:4096};
const G:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096};
fn grant(law:&serde_json::Value)->RetainedCloneGrant{let policy=&law["policy"];RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize}}
fn turn<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{caller.turn_with_decoder(grant,decoder,run).unwrap()}
fn retire<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,value:T){
 let mut value=Some(value);let mut close=RetainedCloneClose::default();
 for _ in 0..256{if value.is_none()&&close.is_empty(){return}turn(caller,decoder,grant,|cx|cx.with_original_retirement(|cx|{let body=grant.maximum_copy_bytes;let d=if !close.is_empty(){close.demands(body)?}else{close.next_owner_demands::<T>(true,&None,body)?};retired(cx,d,|effect|begin(&mut close,&mut value,effect))?;Ok(())}))}
 panic!("retirement did not finish")
}
fn finish(mut caller:OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant){while !decoder.terminal_is_empty(){caller.turn(grant,|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();}caller.retire(grant);}
#[test]
fn original_latest_key_admission_encoding_cancellation_preserve_funded_turns(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let policy=grant(&law);
 for row in law["cases"].as_array().unwrap(){
  let instance=row["instance"].as_u64().unwrap()as u32;let parts:[&str;4]=std::array::from_fn(|index|row["parts"][index].as_str().unwrap());
  let mut expected=format!("{instance:08x}");for part in parts{expected.push_str(&format!("{:016x}",part.len()));for byte in part.as_bytes(){expected.push_str(&format!("{byte:02x}"));}}
  for cut in[0,1,7,usize::MAX]{
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(950),Generation(1),STOCK,default_now_us,u64::MAX,1<<30);let mut decoder=OriginalJobDecodeOwner::empty();
   let denied=[RetainedCloneGrant{maximum_items:0,..policy},RetainedCloneGrant{maximum_depth:0,..policy}];
   let mut copy=None;for grant in denied{assert!(!turn(&mut caller,&mut decoder,grant,|cx|ToolLatestWinsKeyCopy::admit(&mut copy,instance,parts,cx)));assert!(copy.is_none());}
   assert!(turn(&mut caller,&mut decoder,policy,|cx|ToolLatestWinsKeyCopy::admit(&mut copy,instance,parts,cx)));
   let pointer=copy.as_ref().unwrap().key.as_ref().unwrap().as_ptr();let mut turns=0;
   while !copy.as_ref().unwrap().ready()&&turns<cut{
    let before={let copy=copy.as_ref().unwrap();(copy.part,copy.prefix,copy.offset,copy.key.as_ref().unwrap().len())};
    for grant in denied{assert!(!turn(&mut caller,&mut decoder,grant,|cx|copy.as_mut().unwrap().advance(parts,cx)));let copy=copy.as_ref().unwrap();assert_eq!((copy.part,copy.prefix,copy.offset,copy.key.as_ref().unwrap().len()),before);}
    turn(&mut caller,&mut decoder,policy,|cx|copy.as_mut().unwrap().advance(parts,cx));turns+=1;assert!(turns<100000);
    assert_eq!(copy.as_ref().unwrap().key.as_ref().unwrap().as_ptr(),pointer);
   }
   if copy.as_ref().unwrap().ready(){
    assert_eq!(copy.as_ref().unwrap().key.as_ref().unwrap(),&expected);
    let mut shared:Option<SharedOwner<String>>=None;
    for grant in denied{turn(&mut caller,&mut decoder,grant,|cx|copy.as_mut().unwrap().take_key(&mut shared,cx));assert!(shared.is_none());assert_eq!(copy.as_ref().unwrap().key.as_ref().unwrap().as_ptr(),pointer);}
    for _ in 0..8{if shared.is_some(){break}turn(&mut caller,&mut decoder,policy,|cx|copy.as_mut().unwrap().take_key(&mut shared,cx));}
    let shared=shared.expect("the finished key is admitted into one shared owner");assert_eq!(shared.as_bytes().as_ptr(),pointer);assert_eq!(*shared,expected);assert!(copy.as_ref().unwrap().key.is_none());
    retire(&mut caller,&mut decoder,G,shared);
   }
   for _ in 0..8{if copy.as_ref().unwrap().terminal_is_empty(){break}turn(&mut caller,&mut decoder,policy,|cx|cx.with_original_retirement(|cx|copy.as_mut().unwrap().close_step(cx)));}
   assert!(copy.as_ref().unwrap().terminal_is_empty());copy.take();finish(caller,&mut decoder,G);
  }
 }
}
fn effect<R>(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(Option<R>,usize,usize){
 let mut heap=(0,0);let result=caller.turn(grant,|cx|Ok(cx.with_original_effect(grant,|_,effect|{let(result,observed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||operation(effect));heap=(observed.requested_bytes,observed.released_bytes);result})?.map(|(result,_)|result))).unwrap();(result,heap.0,heap.1)
}
fn take(caller:&mut OriginalJobFixtureCaller,cursor:&mut ToolLatestWinsKeyCopy,grant:RetainedCloneGrant)->(Option<(SharedOwner<String>,RetainedCloneProgress)>,usize,usize){let(result,requested,released)=effect(caller,grant,|effect|cursor.take_key(effect).unwrap());(result.flatten(),requested,released)}
#[test]
fn original_latest_key_admission_encoding_cancellation_preserve_actual_receipts(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(1),Generation(1),STOCK,default_now_us,u64::MAX,1<<30);let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let policy=&law["policy"];let grant=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
 for row in law["cases"].as_array().unwrap(){let instance=row["instance"].as_u64().unwrap()as u32;let parts:[&str;4]=std::array::from_fn(|index|row["parts"][index].as_str().unwrap());let mut expected=format!("{instance:08x}");for part in parts{expected.push_str(&format!("{:016x}",part.len()));for byte in part.as_bytes(){expected.push_str(&format!("{byte:02x}"));}}
  for cut in[0,1,7,usize::MAX]{for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:expected.len()-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ToolLatestWinsKeyCopy::admit_original(instance,parts,denied).unwrap());assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let((mut cursor,birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ToolLatestWinsKeyCopy::admit_original(instance,parts,grant).unwrap().unwrap());assert_eq!(birth.copied_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(birth.retained_capacity_bytes,birth.released_bytes));let pointer=cursor.key.as_ref().unwrap().as_ptr();let mut born=heap.requested_bytes;let mut released=heap.released_bytes;let mut turns=0;
   while cursor.part<=4&&turns<cut{let before=(cursor.part,cursor.prefix,cursor.offset,cursor.key.as_ref().unwrap().len());let mut denied_grants=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];let copy=cursor.next_copy_byte_demand(parts);if copy>0{denied_grants.push(RetainedCloneGrant{maximum_copy_bytes:copy-1,..grant});}for denied in denied_grants{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(parts,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((cursor.part,cursor.prefix,cursor.offset,cursor.key.as_ref().unwrap().len()),before);}
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(parts,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cursor.key.as_ref().unwrap().as_ptr(),pointer);turns+=1;assert!(turns<100000);
   }
   if cursor.part>4{assert_eq!(cursor.key.as_ref().unwrap(),&expected);let demand=semio_framework_value_ordered::ordered::SharedOwner::<String>::allocation_bytes();for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:demand-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,requested,released)=take(&mut caller,&mut cursor,denied);assert!(result.is_none());assert_eq!((requested,released),(0,0));assert_eq!(cursor.key.as_ref().unwrap().as_ptr(),pointer);}
    let(taken,requested,freed)=take(&mut caller,&mut cursor,grant);let(shared,receipt)=taken.unwrap();assert_eq!((requested,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(shared.as_bytes().as_ptr(),pointer);born+=requested;released+=freed;
    let mut slot=Some(shared);let mut close=RetainedCloneClose::default();for _ in 0..64{if slot.is_none()&&close.is_empty(){break}let(_,requested,freed)=effect(&mut caller,grant,|effect|{if close.is_empty(){assert!(close.begin_granted(&mut slot,effect).result.unwrap().is_some())}else{plugin_settle_value(close.step_granted(4096,effect)).unwrap();}});born+=requested;released+=freed;}assert!(slot.is_none()&&close.is_empty());
   }
   if cursor.key.is_none(){assert_eq!(born,released);continue}
   let demand=cursor.retirement_demands();for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(denied));assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cursor.key.as_ref().unwrap().as_ptr(),pointer);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(grant));assert!(matches!(step,RetainedCloneStep::Complete(_)));let receipt=step.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));released+=heap.released_bytes;assert_eq!(born,released);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
 }
 caller.retire(STOCK);
}
