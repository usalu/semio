use super::*;
use semio_framework_value::{ControlledRetirement,retained_clone::{RetainedCloneStep,RetainedCloneProgress}};
use std::sync::{Arc,atomic::{AtomicBool,AtomicUsize,Ordering}};

#[test]
fn shard_original_identity_loans_preserve_partial_children_and_actor_ledgers(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../⚡️effects/🧫️fixtures/🔣️.json")).unwrap();
 let policy:ShardIdentityPolicy=serde_json::from_value(fixture["identityPolicy"].clone()).unwrap();
 let source="文🌠".repeat(20000);let allow=Arc::new(AtomicBool::new(false));let original_allow=Arc::clone(&allow);
 let calls=Arc::new(AtomicUsize::new(0));let original_calls=Arc::clone(&calls);
 let ledger=Arc::new(AtomicUsize::new(0));let original_ledger=Arc::clone(&ledger);
 let mut first=OriginalShardIdentity::issue(policy,Box::new(move |event:NativeEncodeProgress|{original_calls.fetch_add(1,Ordering::Relaxed);original_allow.load(Ordering::Relaxed)||event.total!=140000||event.completed<65535}),Box::new(move |request:NativeEncodeAllocation|{assert_eq!(request.owned_bytes,original_ledger.load(Ordering::Relaxed));assert_eq!(request.next_owned_bytes,request.owned_bytes+request.bytes);original_ledger.store(request.next_owned_bytes,Ordering::Relaxed);Ok(())})).unwrap();
 let mut second=OriginalShardIdentity::issue(policy,Box::new(|_|true),Box::new(|_|Ok(()))).unwrap();
 let second_before=second.owned_bytes();let original_recipient=std::ptr::from_ref(first.recipient.as_ref().get_ref());
 let mut identity=first.loan().unwrap();
 let (result,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||identity.encode(|native|native.with_retirement_owner(std::mem::size_of::<ControlledRetirement<String>>(),|native|{let mut original=ControlledRetirement::new(String::new()).ok().unwrap();let result=native.copy_text_into(&source,original.original_mut().unwrap());assert_eq!(original.original_mut().unwrap().as_bytes(),&source.as_bytes()[..65535]);(result,Some(Box::new(original)as Box<dyn ErasedSnapshotRetirement>))})));
 assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert!(birth.requested_bytes>0);assert_eq!(birth.released_bytes,0);
 first.receipt=Some(identity.pause_forwarded().unwrap().detach().unwrap());let retained=first.owned_bytes();assert_eq!(retained,ledger.load(Ordering::Relaxed));assert_eq!(second.owned_bytes(),second_before);assert!(first.recipient.has_owner());
 let mut other=second.loan().unwrap();other.encode::<_,ValueError>(|native|native.charge(7)).unwrap();second.receipt=Some(other.pause_forwarded().unwrap().detach().unwrap());assert_eq!(second.owned_bytes(),second_before+7);assert_eq!(first.owned_bytes(),retained);
 allow.store(true,Ordering::Relaxed);let mut released=0;let mut copied=0;let mut closing_birth=0;let mut turns=0;
 while first.recipient.has_owner(){
  let mut identity=first.loan().unwrap();let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||identity.encode(|native|native.close_retirement_recipient(policy.grant)));let step=step.unwrap();let progress=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};assert!(progress.fits(policy.grant));assert_eq!(heap.released_bytes,progress.released_bytes);assert!(heap.requested_bytes<=progress.retained_capacity_bytes);released+=progress.released_bytes;closing_birth+=heap.requested_bytes;copied+=progress.copied_items;first.receipt=Some(identity.pause_forwarded().unwrap().detach().unwrap());turns+=1;assert!(turns<100000);
 }
 assert_eq!(released,birth.requested_bytes+closing_birth);assert_eq!(std::ptr::from_ref(first.recipient.as_ref().get_ref()),original_recipient);assert_eq!(first.owned_bytes(),ledger.load(Ordering::Relaxed));assert!(copied>0);assert!(calls.load(Ordering::Relaxed)>1);assert_eq!(serde_json::to_value(&source[..65535]).unwrap(),serde_json::Value::String(String::from_utf8(source.as_bytes()[..65535].to_vec()).unwrap()));
 println!("[DEBUG] Original shard identity partialUTF8=65535 admitted={} copiedItems={copied} released={released} turns={turns} actorLedgerIndependent=true sameOriginalRecipient=true independentSerde=true",first.owned_bytes());
}

#[test]
fn shard_original_identity_slot_requires_each_original_positive_axis(){
 let policy:ShardIdentityPolicy=serde_json::from_str(include_str!("../⚙️configuration/🔣️.json")).unwrap();let bytes=std::mem::size_of::<NativeEncodeRetirementRecipient>();
 for axis in ["items","copy","capacity","depth"]{let mut denied=policy;match axis{"items"=>denied.grant.maximum_items=0,"copy"=>denied.grant.maximum_copy_bytes=bytes-1,"capacity"=>denied.grant.maximum_capacity_bytes=bytes-1,_=>denied.grant.maximum_depth=0};let count=Arc::new(AtomicUsize::new(0));let original=Arc::clone(&count);let observer=Box::new(|_|true);let allocation=Box::new(move |_|{original.fetch_add(1,Ordering::Relaxed);Ok(())});let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OriginalShardIdentity::issue(denied,observer,allocation));assert!(matches!(result,Err(ref error)if error.error.kind==ValueRefusalKind::OwnershipLimit));assert_eq!(count.load(Ordering::Relaxed),0);assert_eq!(heap.requested_bytes,0);println!("[DEBUG] Original shard slot {axis} one-short refused before receiving birth; port requests0");}
}
