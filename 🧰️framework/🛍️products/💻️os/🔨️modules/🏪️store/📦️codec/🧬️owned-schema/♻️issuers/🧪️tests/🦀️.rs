//! 🧪️ Original envelope issuers release their exact unique empty backing after every lease returns.
use semio_framework_os_kernel::os_store::owned_schema_decode::{ArtifactEnvelopeFieldDecoderRegistry as Fields,ArtifactEnvelopeCompletedRecordRegistry as Records};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use std::sync::Arc;
const GRANT:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4};
fn observed(f:impl FnOnce()->RetainedCloneStep)->RetainedCloneStep{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);let p=step.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));step}
#[test]fn original_envelope_registry_unique_empty_release_is_exact_and_denials_preserve_identity(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(law["sameOriginalPointerOnRefusal"],true);assert_eq!(law["releasedBytesEqualOriginalBacking"],true);assert_eq!(law["denied"].as_array().unwrap().len(),4);
 let (field,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(Fields::<(),()>::new);assert_eq!((heap.requested_bytes,heap.released_bytes),(Fields::<(),()>::birth_demands().capacity_bytes,0));let (record,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(Records::<(),()>::new);assert_eq!((heap.requested_bytes,heap.released_bytes),(Records::<(),()>::birth_demands().capacity_bytes,0));let mut fields=Some(field);let mut records=Some(record);
 let field_address=Arc::as_ptr(fields.as_ref().unwrap());let record_address=Arc::as_ptr(records.as_ref().unwrap());
 let field_alias=fields.as_ref().unwrap().clone();let record_alias=records.as_ref().unwrap().clone();
 assert!(Fields::empty_release_demands(fields.as_ref().unwrap()).unwrap().is_none());assert!(Records::empty_release_demands(records.as_ref().unwrap()).unwrap().is_none());
 assert_eq!(observed(||Fields::release_empty_original(&mut fields,GRANT).unwrap()).progress(),RetainedCloneProgress::default());assert_eq!(observed(||Records::release_empty_original(&mut records,GRANT).unwrap()).progress(),RetainedCloneProgress::default());
 drop(field_alias);drop(record_alias);
 for axis in 0..4{let mut g=GRANT;match axis{0=>g.maximum_items=0,1=>g.maximum_copy_bytes=0,2=>g.maximum_release_bytes=0,_=>g.maximum_depth=0};
 assert_eq!(observed(||Fields::release_empty_original(&mut fields,g).unwrap()).progress(),RetainedCloneProgress::default());assert_eq!(observed(||Records::release_empty_original(&mut records,g).unwrap()).progress(),RetainedCloneProgress::default());assert_eq!(Arc::as_ptr(fields.as_ref().unwrap()),field_address);assert_eq!(Arc::as_ptr(records.as_ref().unwrap()),record_address);}
 let f=Fields::empty_release_demands(fields.as_ref().unwrap()).unwrap().unwrap();let r=Records::empty_release_demands(records.as_ref().unwrap()).unwrap().unwrap();assert_eq!(f.release_bytes,Fields::<(),()>::birth_demands().capacity_bytes);assert_eq!(r.release_bytes,Records::<(),()>::birth_demands().capacity_bytes);
 assert!(matches!(observed(||Fields::release_empty_original(&mut fields,GRANT).unwrap()),RetainedCloneStep::Complete(_)));assert!(matches!(observed(||Records::release_empty_original(&mut records,GRANT).unwrap()),RetainedCloneStep::Complete(_)));assert!(fields.is_none()&&records.is_none());
}
#[test]fn original_envelope_registry_weak_issuer_keeps_last_allocation_owned(){
 let mut fields=Some(Fields::<(),()>::new());let weak=Arc::downgrade(fields.as_ref().unwrap());assert!(Fields::empty_release_demands(fields.as_ref().unwrap()).unwrap().is_none());assert_eq!(observed(||Fields::release_empty_original(&mut fields,GRANT).unwrap()).progress(),RetainedCloneProgress::default());assert!(weak.upgrade().is_some());drop(weak);assert!(matches!(observed(||Fields::release_empty_original(&mut fields,GRANT).unwrap()),RetainedCloneStep::Complete(_)));
}

#[test]fn original_envelope_registry_owner_closes_the_actual_native_source_without_replacement(){
 use semio_framework_os_kernel::os_store::owned_schema_decode::ArtifactEnvelopeRegistryOwner;
 use semio_framework_job::*;
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(99231),Generation(8),RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4},default_now_us,u64::MAX,64);
 let mut owner=ArtifactEnvelopeRegistryOwner::from_original(Fields::<(),()>::new());let pointer=Arc::as_ptr(owner.get().unwrap());
 for denied in [true,false]{let grant=if denied{RetainedCloneGrant{maximum_release_bytes:0,..GRANT}}else{GRANT};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|cx.with_original_retirement(|cx|{let d=owner.close_demands()?;close_original_retained_step(cx,d,|g|owner.close_step(g))?;Ok(())})));result.unwrap();let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));if denied{assert!(!owner.custody_is_closed());assert_eq!(Arc::as_ptr(owner.get().unwrap()),pointer)}else{assert!(owner.custody_is_closed());assert_eq!(p.released_bytes,Fields::<(),()>::birth_demands().capacity_bytes);}}
 caller.retire(GRANT);
}
