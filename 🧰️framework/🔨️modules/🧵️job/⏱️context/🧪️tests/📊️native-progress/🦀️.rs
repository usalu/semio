//! 📊️ The fixture observes actual original decode progress and cancels only through its admitted alias.
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,testkit,StepContext,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::{cell::RefCell,rc::Rc,sync::Arc};
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn quantum()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(751),Generation(6),stock(),||Some(0),u64::MAX,1000000)}
fn measured(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<(),OriginalTurnFailure>){
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(quantum(),decoder,run));result.unwrap();let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(caller.stock(),stock());
}
fn account<R>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut StepContext<'_>)->R)->R{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||run(cx));cx.consume_retained(RetainedCloneProgress{retained_capacity_bytes:heap.requested_bytes,released_bytes:heap.released_bytes,..Default::default()}).unwrap();result}
fn checkpoint(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 measured(caller,decoder,|cx|{let mut out=None;account(cx,|cx|cx.with_native_receiving(&mut out,|native,_|native.checkpoint()))?;account(cx,|_|drop(out));cx.consume_receiving_retained(cx.receiving_issued_grant().unwrap(),RetainedCloneProgress::default())?;Ok(())});
}
fn close(mut caller:OriginalJobFixtureCaller,mut decoder:OriginalJobDecodeOwner){
 while !decoder.terminal_is_empty(){caller.turn(quantum(),|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();}caller.retire(quantum());
}
#[test]
fn actual_original_progress_observer_has_no_verdict_and_cancels_at_completed_256(){
 let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let token=Arc::new(caller.admit_alias(quantum()));let observing=Arc::clone(&token);
 let seen=Rc::new(RefCell::new(Vec::with_capacity(8)));let observing_seen=Rc::clone(&seen);
 let mut completed_work=0;
 testkit::with_native_decode_progress(Box::new(move|progress|{
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{observing_seen.borrow_mut().push(progress.completed);if progress.completed==256{observing.cancel_now();}});
  assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }),||measured(&mut caller,&mut decoder,|cx|{
  let mut out=None;account(cx,|cx|cx.with_native_receiving(&mut out,|native,_|{
   let identity=(native.ledger_identity(),native.retirement_recipient_identity());assert!(identity.1.is_some());native.begin_stage(0)?;
   for _ in 0..256{assert!(native.prepare_work(1)?);native.commit_work(1)?;completed_work+=1;native.step()?;}
   Ok(())
  }))?;
  let issued=cx.receiving_issued_grant().unwrap();cx.consume_receiving_retained(issued,Default::default())?;cx.consume_fuel(completed_work);let taken=out.take().unwrap();let failure=account(cx,|_|taken.call.unwrap_err());assert_eq!(failure.operation.or(failure.context).unwrap().kind,ValueRefusalKind::Canceled);Ok(())
 }));
 assert_eq!(completed_work,256);assert!(seen.borrow().contains(&256));assert!(token.is_cancelled_now());
 caller.return_alias(Arc::try_unwrap(token).ok().expect("observer releases only its external borrow"),quantum());close(caller,decoder);
}
#[test]
fn scoped_native_progress_observers_restore_nested_and_unwound_owners(){
 let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let outer=Rc::new(RefCell::new(Vec::with_capacity(8)));let inner=Rc::new(RefCell::new(Vec::with_capacity(8)));let observing=Rc::clone(&outer);
 testkit::with_native_decode_progress(Box::new(move|progress|{observing.borrow_mut().push(progress.completed);}),||{
  checkpoint(&mut caller,&mut decoder);let observing=Rc::clone(&inner);
  testkit::with_native_decode_progress(Box::new(move|progress|{observing.borrow_mut().push(progress.completed);}),||checkpoint(&mut caller,&mut decoder));
  let unwound=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||testkit::with_native_decode_progress(Box::new(|_|panic!("unreached nested observer")),||panic!("scoped fixture unwind"))));assert!(unwound.is_err());
  checkpoint(&mut caller,&mut decoder);
 });
 assert_eq!(outer.borrow().as_slice(),[0,0]);assert_eq!(inner.borrow().as_slice(),[0]);close(caller,decoder);
}
