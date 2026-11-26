use super::{RetainedOriginalFailurePublication,OriginalTurnFailure,RetainedCloneGrant,RetainedCloneProgress,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView};
use semio_framework_job::OriginalJobFixtureCaller;
use semio_framework_diagnostic::Fault;
use semio_framework_value::FromValue;
fn original_clock()->Option<u64>{Some(1000)}
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
fn delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}}

pub(super) fn fault_publication_batches_paid_original_note_projection(){
 let transcript:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/🧪️tests/lifecycle-retry.json")).unwrap();
 let hex=transcript["packHex"].as_str().unwrap();
 let expected:Vec<u8>=(0..hex.len()).step_by(2).map(|i|u8::from_str_radix(&hex[i..i+2],16).unwrap()).collect();
 assert_eq!(expected.len(),395);
 let decoded:serde_json::Value=semio_os_kernel::os_pack::pack_rt::decode_wire_value(&expected).unwrap().into();
 let original=OriginalTurnFailure::from(Fault::from_value(serde_json::from_value(decoded["primary"]["original"].clone()).unwrap()).unwrap());
 let stock=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:64};
 let grant=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:262144,maximum_capacity_bytes:262144,maximum_release_bytes:262144,..stock};
 let run=|batched|{
  let mut caller=OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(795),semio_framework_job::Generation(1),stock,original_clock,100_000_000,10_000_000);
  caller.admit_return_table();
  let mut publication=RetainedOriginalFailurePublication::new();
  let mut descriptor=None;
  let destination=std::mem::size_of::<Option<JobOutcomeDescriptor>>();
  caller.retirement_turn(RetainedCloneGrant{maximum_items:0,..grant},|cx|{
   let before=(cx.retained_progress(),cx.fuel_remaining());
   assert!(publication.advance_batched_with_cursor(&original,cx,destination)?.is_none());
   assert_eq!((cx.retained_progress(),cx.fuel_remaining()),before);
   assert!(publication.terminal_is_empty());
   Ok(())
  }).unwrap();
  caller.retirement_turn(grant,|cx|{
   cx.with_original_work_loan(32,original_clock().unwrap(),|cx|{
    let before=(cx.retained_progress(),cx.fuel_remaining());
    assert!(cx.deadline_exceeded());
    let outcome=if batched{publication.advance_batched_with_cursor(&original,cx,destination)?}else{publication.advance_with_cursor(&original,cx,destination)?};
    assert!(outcome.is_none());
    let paid=delta(before.0,cx.retained_progress());
    assert_eq!((paid.copied_items,paid.retained_capacity_bytes,paid.released_bytes,before.1-cx.fuel_remaining()),(1,0,0,1));
    Ok(())
   })
  }).unwrap();
  let mut turns=1;let mut charged=RetainedCloneProgress::default();
  for _ in 0..4096{
   turns+=1;
   let (result,heap)=observe(||caller.retirement_turn(grant,|cx|{
    let before=cx.retained_progress();
    let outcome=if batched{publication.advance_batched_with_cursor(&original,cx,destination)?}else{publication.advance_with_cursor(&original,cx,destination)?};
    descriptor=outcome.map(JobOutcomeBorrow::into_descriptor);
    Ok(delta(before,cx.retained_progress()))
   }));
   let progress=result.unwrap();
   assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));
   charged=charged.checked_add(progress).unwrap();
   if descriptor.is_some(){break}
  }
  let mut descriptor=descriptor.expect("same Source published the actual fault");
  let bytes={
   let JobOutcomeView::Fault{detail,..}=publication.borrow_outcome(&descriptor).unwrap()else{panic!("expected the real fault payload")};
   assert_eq!(detail.page_count(),1);
   detail.page(0).unwrap().to_vec()
  };
  caller.retirement_turn(grant,|cx|{
   let step=descriptor.acknowledge(cx.retained_grant()).result?;
   cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(())
  }).unwrap();
  assert!(descriptor.is_acknowledged());
  assert!((0..4096).any(|_|caller.retirement_turn(grant,|cx|publication.close_on_source(cx)).unwrap()));
  caller.retire(stock);
  (bytes,turns,charged)
 };
 let(single,old_turns,old_charge)=run(false);
 let(batched,new_turns,new_charge)=run(true);
 assert_eq!(single,expected);assert_eq!(batched,expected);
 assert_eq!(new_charge.retained_capacity_bytes,old_charge.retained_capacity_bytes);
 assert_eq!(new_charge.released_bytes,old_charge.released_bytes);
 assert!(new_charge.copied_items<old_charge.copied_items&&new_charge.copied_bytes<old_charge.copied_bytes);
 assert!(new_turns*6<old_turns,"expected materially fewer funded calls: {old_turns} -> {new_turns}");
 println!("authentic fault bytes={} paid calls {old_turns} -> {new_turns}",batched.len());
}

#[test]
fn batched_original_failure_publication_matches_the_genuine_note_packet(){fault_publication_batches_paid_original_note_projection()}
