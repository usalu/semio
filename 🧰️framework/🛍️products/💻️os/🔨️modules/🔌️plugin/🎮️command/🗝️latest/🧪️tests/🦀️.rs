//! 🗝️ Native original key framing, funded turns, actual receipts and physical backing conservation.
use crate::command_admission::original_latest_key::ToolLatestWinsKeyCopy;
use semio_framework_job::{Generation,OperationId,OriginalJobDecodeOwner,OriginalJobFixtureCaller,OriginalTurnFailure,StepContext,default_now_us};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
use std::mem::size_of;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retained_clone::{RetainedCloneClose,RetainedCloneStep};
use semio_framework_value_ordered::ordered::SharedOwner;
use semio_plugin_job::original_steps::{begin,retired_poll};
const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:4096};
const G:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096};
fn grant(law:&serde_json::Value)->RetainedCloneGrant{let policy=&law["policy"];RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize}}
fn turn<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{caller.turn_with_decoder(grant,decoder,run).unwrap()}
fn retire<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,value:T){
 let mut value=Some(value);let mut close=RetainedCloneClose::default();
 for _ in 0..256{if value.is_none()&&close.is_empty(){return}turn(caller,decoder,grant,|cx|cx.with_original_retirement(|cx|{let body=grant.maximum_copy_bytes;let d=if !close.is_empty(){close.demands(body)?}else{close.next_owner_demands::<T>(true,&None,body)?};retired_poll(cx,d,|effect|begin(&mut close,&mut value,effect))?;Ok(())}))}
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
fn observed<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->(R,RetainedCloneProgress){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,run));let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes),"actual latest key receipt {receipt:?}");(result.unwrap(),receipt)}
#[test]
fn original_latest_key_admission_encoding_cancellation_preserve_actual_receipts(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let grant=grant(&law);
 for row in law["cases"].as_array().unwrap(){
  let instance=row["instance"].as_u64().unwrap()as u32;let parts:[&str;4]=std::array::from_fn(|index|row["parts"][index].as_str().unwrap());
  let mut expected=format!("{instance:08x}");for part in parts{expected.push_str(&format!("{:016x}",part.len()));for byte in part.as_bytes(){expected.push_str(&format!("{byte:02x}"));}}
  for cut in[0,1,7,usize::MAX]{
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(951),Generation(1),STOCK,default_now_us,u64::MAX,1<<30);let mut decoder=OriginalJobDecodeOwner::empty();let(mut born,mut released)=(0,0);let mut tally=|receipt:RetainedCloneProgress|{born+=receipt.retained_capacity_bytes;released+=receipt.released_bytes};
   let demand=ToolLatestWinsKeyCopy::birth_demands(parts).unwrap();let mut copy=None;
   for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(admitted,receipt)=observed(&mut caller,&mut decoder,denied,|cx|ToolLatestWinsKeyCopy::admit(&mut copy,instance,parts,cx));assert!(!admitted&&copy.is_none());assert_eq!(receipt,Default::default());}
   let(admitted,receipt)=observed(&mut caller,&mut decoder,grant,|cx|ToolLatestWinsKeyCopy::admit(&mut copy,instance,parts,cx));assert!(admitted);assert_eq!(receipt.copied_bytes,size_of::<ToolLatestWinsKeyCopy>());tally(receipt);
   let pointer=copy.as_ref().unwrap().key.as_ref().unwrap().as_ptr();let mut turns=0;
   while !copy.as_ref().unwrap().ready()&&turns<cut{let(_,receipt)=observed(&mut caller,&mut decoder,grant,|cx|copy.as_mut().unwrap().advance(parts,cx));assert!(receipt.fits(grant));assert_eq!(copy.as_ref().unwrap().key.as_ref().unwrap().as_ptr(),pointer);tally(receipt);turns+=1;assert!(turns<100000);}
   if copy.as_ref().unwrap().ready(){
    assert_eq!(copy.as_ref().unwrap().key.as_ref().unwrap(),&expected);let mut shared:Option<SharedOwner<String>>=None;
    for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(_,receipt)=observed(&mut caller,&mut decoder,denied,|cx|copy.as_mut().unwrap().take_key(&mut shared,cx));assert!(shared.is_none());assert_eq!(receipt,Default::default());assert_eq!(copy.as_ref().unwrap().key.as_ref().unwrap().as_ptr(),pointer);}
    for _ in 0..8{if shared.is_some(){break}let(_,receipt)=observed(&mut caller,&mut decoder,grant,|cx|copy.as_mut().unwrap().take_key(&mut shared,cx));tally(receipt);}
    let shared=shared.expect("the finished key is admitted into one shared owner");assert_eq!(shared.as_bytes().as_ptr(),pointer);
    let mut slot=Some(shared);let mut close=RetainedCloneClose::default();
    for _ in 0..64{if slot.is_none()&&close.is_empty(){break}let(_,receipt)=observed(&mut caller,&mut decoder,G,|cx|cx.with_original_retirement(|cx|{let body=G.maximum_copy_bytes;let d=if !close.is_empty(){close.demands(body)?}else{close.next_owner_demands::<SharedOwner<String>>(true,&None,body)?};retired_poll(cx,d,|effect|begin(&mut close,&mut slot,effect))?;Ok(())}));tally(receipt);}
    assert!(slot.is_none()&&close.is_empty());
   }
   for _ in 0..8{if copy.as_ref().unwrap().terminal_is_empty(){break}let(_,receipt)=observed(&mut caller,&mut decoder,grant,|cx|cx.with_original_retirement(|cx|copy.as_mut().unwrap().close_step(cx)));tally(receipt);}
   assert!(copy.as_ref().unwrap().terminal_is_empty());copy.take();assert_eq!(born,released,"every retained key byte is released by its receipted close");finish(caller,&mut decoder,G);
  }
 }
}
