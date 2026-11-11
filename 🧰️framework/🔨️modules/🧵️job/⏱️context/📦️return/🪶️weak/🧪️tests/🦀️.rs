//! 🪶️ An actual borrowed strong parent returns its exact Weak without a new alias or heap effect.
use semio_framework_value_testing_allocator as _;
use semio_framework_job::{CancelToken, Generation, OperationId, OriginalCallerSourceParent, RetainedCloneGrant, RetainedCloneProgress, StepBudget};
use semio_framework_job_payload::{JobPayloadAuthority};
use semio_framework_job_testing_caller::{bank_run,bank_source_parent};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use std::sync::Arc;

fn now()->Option<u64>{Some(1000)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128}}
fn bank()->EffectBank{EffectBank::new(Wallet::from_profile(&serde_json::json!({"items":1<<20,"copyBytes":1<<30,"capacityBytes":1<<30,"releaseBytes":1<<30,"depth":1<<20,"fuel":1<<20})))}
fn source_parent(bank:&mut EffectBank)->OriginalCallerSourceParent{let grant=grant();let(cancel,_)=CancelToken::admit_root(grant).result.unwrap().unwrap();let(authority,_)=JobPayloadAuthority::admit(OperationId(17),Generation(9),grant,&mut None).result.unwrap().unwrap();bank_source_parent(bank,authority,cancel)}
fn retire(bank:&mut EffectBank,mut parent:OriginalCallerSourceParent){for _ in 0..64{if parent.terminal_is_empty(){return}let demand=parent.retirement_demands().unwrap();let issued=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let work=parent.retirement_work_demand().unwrap();let(_,admitted)=bank_run(bank,issued,work,|effect|parent.close_step(effect));assert_eq!(admitted,Some(true));}panic!("original source parent did not finish its paid close")}

/// 🧾️ Missing, foreign and denied witnesses preserve the Weak; its exact funded parent returns metadata only.
#[test]
fn original_native_weak_return_preserves_denied_source_and_posts_exact_metadata(){
 let mut bank=bank();let parent=source_parent(&mut bank);let mut sequence=0;let mut receipt=RetainedCloneProgress::default();let mut cx=parent.borrow_context(StepBudget::new(64,1500,grant()),now,&mut sequence,&mut receipt).unwrap();cx.bind_original_source_epoch(77).unwrap();let allocation=Arc::new(91u64);let foreign=Arc::new(92u64);let mut source=Some(Arc::downgrade(&allocation));let address=source.as_ref().unwrap().as_ptr();let recipient=cx.original_recipient_identity();
 let(result,heap)=observe_heap(||cx.return_original_native_weak(&mut source));assert!(result.is_err());assert_eq!((heap.0,heap.1),(0,0));assert_eq!(source.as_ref().unwrap().as_ptr(),address);
 let(result,heap)=observe_heap(||cx.with_original_native_weak_parent(&foreign,|same|same.return_original_native_weak(&mut source)));assert!(result.is_err());assert_eq!((heap.0,heap.1),(0,0));assert_eq!(source.as_ref().unwrap().as_ptr(),address);
 for loan in [RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{let(result,heap)=observe_heap(||cx.with_retained_loan(loan,|same|same.with_original_native_weak_parent(&allocation,|same|same.return_original_native_weak(&mut source))));assert_eq!(result.unwrap(),None);assert_eq!((heap.0,heap.1),(0,0));assert_eq!(source.as_ref().unwrap().as_ptr(),address);}
 let(result,heap)=observe_heap(||cx.with_original_native_weak_parent(&allocation,|same|{same.with_original_native_weak_source(source.as_ref().unwrap(),|original,_|{assert_eq!(*original,91);Ok(())})?;same.return_original_native_weak(&mut source)}));let progress=result.unwrap().unwrap();assert_eq!(progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((heap.0,heap.1),(0,0));assert!(source.is_none());assert_eq!(cx.retained_progress(),progress);assert_eq!(cx.original_recipient_identity(),recipient);assert_eq!(cx.fuel_remaining(),63);assert_eq!(Arc::strong_count(&allocation),1);assert_eq!(Arc::weak_count(&allocation),0);assert_eq!(cx.original_source_epoch(),Some(77));
 let local=Arc::new(std::cell::Cell::new(15u64));let mut local_source=Some(Arc::downgrade(&local));let(result,heap)=observe_heap(||cx.with_original_native_weak_parent(&local,|same|{same.with_original_native_weak_source(local_source.as_ref().unwrap(),|original,_|{original.set(16);Ok(())})?;same.return_original_native_weak(&mut local_source)}));assert_eq!(result.unwrap().unwrap(),progress);assert_eq!((heap.0,heap.1),(0,0));assert_eq!(local.get(),16);assert!(local_source.is_none());drop(cx);retire(&mut bank,parent);
}

/// 🪢️ Nested and unwound scopes restore the original witness; escaped context copies expire with their lexical parent.
#[test]
fn original_native_weak_nested_unwind_and_context_swap_expire_scope(){
 let mut bank=bank();let parent=source_parent(&mut bank);let foreign_parent=source_parent(&mut bank);let mut sequence=0;let mut receipt=Default::default();let mut foreign_sequence=0;let mut foreign_receipt=Default::default();let mut cx=parent.borrow_context(StepBudget::new(64,1500,grant()),now,&mut sequence,&mut receipt).unwrap();let allocation=Arc::new(91u64);let foreign=Arc::new(92u64);let mut source=Some(Arc::downgrade(&allocation));
 cx.with_original_native_weak_parent(&allocation,|same|{let refusal=same.with_original_native_weak_parent(&foreign,|same|same.with_original_native_weak_source(source.as_ref().unwrap(),|_,_|Ok(())));assert!(refusal.is_err());same.with_original_native_weak_source(source.as_ref().unwrap(),|original,_|{assert_eq!(*original,91);Ok(())})}).unwrap();assert!(cx.with_original_native_weak_source(source.as_ref().unwrap(),|_,_|Ok(())).is_err());
 let denied=cx.with_original_native_weak_parent(&allocation,|same|Ok(std::thread::scope(|scope|scope.spawn(||{let(result,heap)=observe_heap(||same.with_original_native_weak_source(source.as_ref().unwrap(),|_,_|Ok(())));assert_eq!((heap.0,heap.1),(0,0));result.is_err()}).join().unwrap()))).unwrap();assert!(denied);assert!(source.is_some());
 let panic=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let _=cx.with_original_native_weak_parent(&allocation,|_|->Result<(),semio_framework_job::OriginalTurnFailure>{panic!("original witness unwinds")});}));assert!(panic.is_err());assert!(cx.return_original_native_weak(&mut source).is_err());assert!(source.is_some());
 let mut replacement=foreign_parent.borrow_context(StepBudget::new(64,1500,grant()),now,&mut foreign_sequence,&mut foreign_receipt).unwrap();let refused=cx.with_original_native_weak_parent(&allocation,|same|{std::mem::swap(same,&mut replacement);Ok(())});assert!(refused.is_err());assert!(replacement.return_original_native_weak(&mut source).is_err());assert!(source.is_some());std::mem::swap(&mut cx,&mut replacement);drop(replacement);
 cx.with_original_native_weak_parent(&allocation,|same|same.return_original_native_weak(&mut source)).unwrap().unwrap();drop(cx);retire(&mut bank,foreign_parent);retire(&mut bank,parent);
}
fn observe_heap<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
