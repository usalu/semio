//! 🧪️ Borrowed contexts retain the exact private Arc count while public admission laws live independently.
use super::*;
#[test]
fn original_payload_authority_private_borrow_does_not_clone_its_ledger(){
 let stock=RetainedCloneGrant{maximum_items:1000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 let mut caller=crate::OriginalJobFixtureCaller::admit(OperationId(99203),Generation(7),stock,||Some(1),u64::MAX,1000);
 let mut original=None;
 for _ in 0..17{
  let((pointer,count),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1},|cx|{let observation=(Arc::as_ptr(&cx.payload_ledger),Arc::strong_count(&cx.payload_ledger));cx.consume_fuel(1);Ok(observation)}).unwrap());
  assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.receipt(),Default::default());
  match original{Some(expected)=>assert_eq!((pointer,count),expected),None=>original=Some((pointer,count))}
 }
 caller.retire(stock);
}


#[test]
fn original_payload_builder_initializes_the_real_staged_slot_in_each_empty_carrier(){
 let grant=RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128};let mut caller=crate::OriginalJobFixtureCaller::admit(OperationId(99204),Generation(7),grant,||Some(1),u64::MAX,4096);
 caller.turn(grant,|cx|{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:RetainedPayloadBuilder::empty_carrier_copy_bytes(),..Default::default()})?;let empty=RetainedPayloadBuilder::new(JobPayloadStream::CommitState);assert!(empty.staged.is_none());assert!(empty.terminal_is_empty());assert_eq!(empty.staged_page_len(),None);drop(empty);Ok(())}).unwrap();let mut storage=MaybeUninit::<RetainedPayloadBuilder>::uninit();
 caller.turn(grant,|cx|{let progress=RetainedCloneProgress{copied_items:1,copied_bytes:RetainedPayloadBuilder::empty_carrier_copy_bytes(),..Default::default()};cx.consume_retained(progress)?;unsafe{RetainedPayloadBuilder::initialize_empty_at(storage.as_mut_ptr(),JobPayloadStream::CommitOutput);}Ok(())}).unwrap();
 let initialized=unsafe{storage.assume_init()};assert!(initialized.staged.is_none());assert!(initialized.terminal_is_empty());assert_eq!(initialized.staged_page_remaining(),0);drop(initialized);
 let metadata=size_of::<JobPayloadStream>()+size_of::<usize>()+2*size_of::<bool>()+size_of::<Option<(Arc<JobPayloadOperationLedger>,JobPayloadPageSource,usize)>>();assert_eq!(RetainedPayloadBuilder::empty_carrier_copy_bytes(),metadata);assert_eq!(RetainedPayloadBuilder::boxed_birth_demands().copy_bytes,metadata+2*size_of::<Box<RetainedPayloadBuilder>>());caller.retire(grant);
}
