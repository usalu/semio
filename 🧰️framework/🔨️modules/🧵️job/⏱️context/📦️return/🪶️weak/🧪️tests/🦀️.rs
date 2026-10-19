//! 🪶️ An actual borrowed strong parent returns its exact Weak without a new alias or heap effect.
use semio_framework_job::{CancelToken, Generation, OperationId, OriginalCallerSourceParent, RetainedCloneGrant, RetainedCloneProgress, StepBudget};
use semio_framework_job_payload::{JobPayloadAuthority};
use std::sync::Arc;

#[global_allocator]
static WEAK_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

fn now()->Option<u64>{Some(1000)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128}}
fn source_parent()->OriginalCallerSourceParent{let grant=grant();let(cancel,_)=CancelToken::admit_root(grant).unwrap().unwrap();let(authority,_)=JobPayloadAuthority::admit(OperationId(17),Generation(9),grant).unwrap().unwrap();OriginalCallerSourceParent::admit(authority,cancel,grant).ok().unwrap().0}
fn retire(mut parent:OriginalCallerSourceParent){for _ in 0..64{if parent.terminal_is_empty(){return}parent.close_step(grant()).unwrap();}panic!("original source parent did not finish its paid close")}

/// 🧾️ Missing, foreign and denied witnesses preserve the Weak; its exact funded parent returns metadata only.
#[test]
fn original_native_weak_return_preserves_denied_source_and_posts_exact_metadata(){
 let parent=source_parent();let mut sequence=0;let mut receipt=RetainedCloneProgress::default();let mut cx=parent.borrow_context(StepBudget::new(64,1500,grant()),now,&mut sequence,&mut receipt).unwrap();cx.bind_original_source_epoch(77).unwrap();let allocation=Arc::new(91u64);let foreign=Arc::new(92u64);let mut source=Some(Arc::downgrade(&allocation));let address=source.as_ref().unwrap().as_ptr();let recipient=cx.original_recipient_identity();
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.return_original_native_weak(&mut source));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ref().unwrap().as_ptr(),address);
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_native_weak_parent(&foreign,|same|same.return_original_native_weak(&mut source)));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ref().unwrap().as_ptr(),address);
 for loan in [RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|same.with_original_native_weak_parent(&allocation,|same|same.return_original_native_weak(&mut source))));assert_eq!(result.unwrap(),None);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ref().unwrap().as_ptr(),address);}
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_native_weak_parent(&allocation,|same|{same.with_original_native_weak_source(source.as_ref().unwrap(),|original,_|{assert_eq!(*original,91);Ok(())})?;same.return_original_native_weak(&mut source)}));let progress=result.unwrap().unwrap();assert_eq!(progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(source.is_none());assert_eq!(cx.retained_progress(),progress);assert_eq!(cx.original_recipient_identity(),recipient);assert_eq!(cx.fuel_remaining(),63);assert_eq!(Arc::strong_count(&allocation),1);assert_eq!(Arc::weak_count(&allocation),0);assert_eq!(cx.original_source_epoch(),Some(77));
 let local=Arc::new(std::cell::Cell::new(15u64));let mut local_source=Some(Arc::downgrade(&local));let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_native_weak_parent(&local,|same|{same.with_original_native_weak_source(local_source.as_ref().unwrap(),|original,_|{original.set(16);Ok(())})?;same.return_original_native_weak(&mut local_source)}));assert_eq!(result.unwrap().unwrap(),progress);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(local.get(),16);assert!(local_source.is_none());drop(cx);retire(parent);
}

/// 🪢️ Nested and unwound scopes restore the original witness; escaped context copies expire with their lexical parent.
#[test]
fn original_native_weak_nested_unwind_and_context_swap_expire_scope(){
 let parent=source_parent();let foreign_parent=source_parent();let mut sequence=0;let mut receipt=Default::default();let mut foreign_sequence=0;let mut foreign_receipt=Default::default();let mut cx=parent.borrow_context(StepBudget::new(64,1500,grant()),now,&mut sequence,&mut receipt).unwrap();let allocation=Arc::new(91u64);let foreign=Arc::new(92u64);let mut source=Some(Arc::downgrade(&allocation));
 cx.with_original_native_weak_parent(&allocation,|same|{let refusal=same.with_original_native_weak_parent(&foreign,|same|same.with_original_native_weak_source(source.as_ref().unwrap(),|_,_|Ok(())));assert!(refusal.is_err());same.with_original_native_weak_source(source.as_ref().unwrap(),|original,_|{assert_eq!(*original,91);Ok(())})}).unwrap();assert!(cx.with_original_native_weak_source(source.as_ref().unwrap(),|_,_|Ok(())).is_err());
 let denied=cx.with_original_native_weak_parent(&allocation,|same|Ok(std::thread::scope(|scope|scope.spawn(||{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||same.with_original_native_weak_source(source.as_ref().unwrap(),|_,_|Ok(())));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));result.is_err()}).join().unwrap()))).unwrap();assert!(denied);assert!(source.is_some());
 let panic=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let _=cx.with_original_native_weak_parent(&allocation,|_|->Result<(),semio_framework_job::OriginalTurnFailure>{panic!("original witness unwinds")});}));assert!(panic.is_err());assert!(cx.return_original_native_weak(&mut source).is_err());assert!(source.is_some());
 let mut replacement=foreign_parent.borrow_context(StepBudget::new(64,1500,grant()),now,&mut foreign_sequence,&mut foreign_receipt).unwrap();let refused=cx.with_original_native_weak_parent(&allocation,|same|{std::mem::swap(same,&mut replacement);Ok(())});assert!(refused.is_err());assert!(replacement.return_original_native_weak(&mut source).is_err());assert!(source.is_some());std::mem::swap(&mut cx,&mut replacement);drop(replacement);
 cx.with_original_native_weak_parent(&allocation,|same|same.return_original_native_weak(&mut source)).unwrap().unwrap();drop(cx);retire(foreign_parent);retire(parent);
}
