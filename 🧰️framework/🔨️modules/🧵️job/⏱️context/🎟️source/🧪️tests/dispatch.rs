//! 🧪️ Genuine Source admission issues distinct dispatches and preserves one token through nested loans.
use semio_framework_job_context::{Generation,OperationId,RetainedCloneGrant};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
fn clock()->Option<u64>{Some(1)}
#[test]
fn genuine_source_dispatches_are_distinct_and_nested_loans_keep_the_original(){
 let grant=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:16*1024*1024,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:128};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(81),Generation(3),grant,clock,1_000_000,1_000_000);
 let first=caller.turn(grant,|cx|{let original=cx.original_source_turn_witness()?.identity();cx.with_retained_loan(grant,|nested|{assert_eq!(nested.original_source_turn_identity(),Some(original));assert!(nested.original_source_turn_witness()?.matches(original));Ok(())})?;Ok(original)}).unwrap();
 let second=caller.turn(grant,|cx|Ok(cx.original_source_turn_witness()?.identity())).unwrap();assert_ne!(first,second);
 let mut interrupted=None;let panic=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||caller.turn(grant,|cx|{interrupted=Some(cx.original_source_turn_witness()?.identity());panic!("original Source dispatch interrupted");#[allow(unreachable_code)]Ok::<_,semio_framework_job_context::OriginalTurnFailure>(())})));assert!(panic.is_err());
 let after=caller.turn(grant,|cx|Ok(cx.original_source_turn_witness()?.identity())).unwrap();assert_ne!(interrupted,Some(after));assert_ne!(second,after);assert!(caller.retire(grant)>0);
}
