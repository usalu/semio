//! 👥️ Unmounted native context allocation, concurrency and final-policy ownership laws.
use super::*;
use crate::test_allocation::observe;
use std::sync::{Arc,Barrier};
use std::sync::atomic::{AtomicUsize,Ordering};

#[test]
fn pending_physical_claim_excludes_a_second_allocator_before_the_first_returns(){
 let drops=Arc::new(AtomicUsize::new(0));let bytes=std::mem::size_of::<TransportCell<Sentinel>>();
 let maximum=std::mem::size_of::<ContextCell<Policy>>()+bytes;
 let context=PackTransportContext::try_new(TransportAdmission::new(maximum,0),Policy(drops)).ok().unwrap();
 let base=context.committed_bytes();let entered=Barrier::new(2);let finish=Barrier::new(2);let calls=AtomicUsize::new(0);
 let (first,second)=std::thread::scope(|scope|{
  let worker=scope.spawn(||reserve_error_with::<Sentinel,_>(&context,PackTransportCategory::NativeIo,PackRetryDisposition::Never,|storage|{calls.fetch_add(1,Ordering::SeqCst);let allocated=storage.try_reserve_exact(1).is_ok();entered.wait();finish.wait();allocated}));
  entered.wait();let pending=context.committed_bytes();
  let second=reserve_error_with::<Sentinel,_>(&context,PackTransportCategory::HttpBody,PackRetryDisposition::Never,|storage|{calls.fetch_add(1,Ordering::SeqCst);storage.try_reserve_exact(1).is_ok()});
  finish.wait();let first=worker.join().unwrap();assert_eq!(pending,base+bytes);(first,second)
 });
 assert_eq!(calls.load(Ordering::SeqCst),1);
 assert_eq!(second.err().unwrap().refusal_kind(),Some(ValueRefusalKind::OwnershipLimit));
 let first=first.unwrap();assert_eq!(context.committed_bytes(),base+first.allocated_bytes());drop(first);assert_eq!(context.committed_bytes(),base);
 let next=context.reserve::<Sentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).unwrap();drop(next);assert_eq!(context.committed_bytes(),base);
 eprintln!("[DEBUG] pending source credit prevented a second actual allocator before the first completed");
}

struct PanickingPolicy(Arc<AtomicUsize>);
impl PackTransportPolicy for PanickingPolicy{
 fn checkpoint(&self,event:PackTransportProgress)->Result<(),ValueError>{if event.phase==PackTransportPhase::AfterReserve&&self.0.compare_exchange(1,0,Ordering::SeqCst,Ordering::SeqCst).is_ok(){std::panic::panic_any("original source policy panic");}Ok(())}
 fn progress(&self,event:PackTransportProgress){if event.phase==PackTransportPhase::ReservationCommitted&&self.0.compare_exchange(2,0,Ordering::SeqCst,Ordering::SeqCst).is_ok(){std::panic::panic_any("original source policy panic");}}
}

#[test]
fn policy_panics_refund_only_their_charge_and_allow_a_subsequent_reservation(){
 for mode in [1,2]{
  let armed=Arc::new(AtomicUsize::new(0));let context=PackTransportContext::try_new(TransportAdmission::new(4096,17),PanickingPolicy(armed.clone())).ok().unwrap();
  let base=context.committed_bytes();let unrelated=context.reserve::<Sentinel>(PackTransportCategory::HttpRequest,PackRetryDisposition::Never).unwrap();let unrelated_bytes=unrelated.allocated_bytes();
  armed.store(mode,Ordering::SeqCst);
  let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||context.reserve::<Sentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never)));
  assert_eq!(result.err().unwrap().downcast_ref::<&'static str>(),Some(&"original source policy panic"));
  assert_eq!(context.committed_bytes(),base+unrelated_bytes);
  let next=context.reserve::<Sentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).unwrap();assert_eq!(context.committed_bytes(),base+unrelated_bytes+next.allocated_bytes());drop(next);
  assert_eq!(context.committed_bytes(),base+unrelated_bytes);
  let cause=unrelated.publish(Sentinel);drop(cause);assert_eq!(context.committed_bytes(),base+unrelated_bytes);
 }
 eprintln!("[DEBUG] two actual callback panic phases refunded only their live reservation and preserved unrelated published charge");
}

struct Policy(Arc<AtomicUsize>);
impl PackTransportPolicy for Policy{fn checkpoint(&self,_:PackTransportProgress)->Result<(),semio_framework_value::ValueError>{Ok(())}fn progress(&self,_:PackTransportProgress){}}
impl Drop for Policy{fn drop(&mut self){self.0.fetch_add(1,Ordering::SeqCst);}}
#[derive(Debug)]
struct Sentinel;
impl std::fmt::Display for Sentinel{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("same misleading cancellation/limit prose")}}
impl std::error::Error for Sentinel{}

struct LifecyclePolicy{phase:PackTransportPhase,armed:Arc<std::sync::atomic::AtomicBool>,drops:Arc<AtomicUsize>}
impl PackTransportPolicy for LifecyclePolicy{
 fn checkpoint(&self,event:PackTransportProgress)->Result<(),ValueError>{if self.phase==PackTransportPhase::ContextAfter&&event.phase==self.phase&&self.armed.swap(false,Ordering::SeqCst){std::panic::panic_any("original lifecycle panic");}Ok(())}
 fn progress(&self,event:PackTransportProgress){if self.phase!=PackTransportPhase::ContextAfter&&event.phase==self.phase&&self.armed.swap(false,Ordering::SeqCst){std::panic::panic_any("original lifecycle panic");}}
}
impl Drop for LifecyclePolicy{fn drop(&mut self){self.drops.fetch_add(1,Ordering::SeqCst);}}
#[derive(Debug)]
struct DropSentinel{drops:Arc<AtomicUsize>,panic:bool}
impl std::fmt::Display for DropSentinel{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("original source")}}
impl Error for DropSentinel{}
impl Drop for DropSentinel{fn drop(&mut self){self.drops.fetch_add(1,Ordering::SeqCst);if self.panic{std::panic::panic_any("original source destructor panic");}}}

#[test]
fn original_lifecycle_panics_drop_owned_context_and_source_once(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧭️cause/🔌️capture/👥️context/💥️lifecycle/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let phase=match row["phase"].as_str().unwrap(){"contextAfter"=>PackTransportPhase::ContextAfter,"contextReady"=>PackTransportPhase::ContextReady,"published"=>PackTransportPhase::SourcePublished,"closed"=>PackTransportPhase::ContextClosed,_=>panic!("unknown closed lifecycle phase")};
  let drops=Arc::new(AtomicUsize::new(0));let source_drops=Arc::new(AtomicUsize::new(0));let armed=Arc::new(std::sync::atomic::AtomicBool::new(true));
  let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
   let context=PackTransportContext::try_new(TransportAdmission::new(4096,16),LifecyclePolicy{phase,armed,drops:drops.clone()}).ok().unwrap();
   if phase==PackTransportPhase::SourcePublished{let reserve=context.reserve::<DropSentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).unwrap();reserve.publish(DropSentinel{drops:source_drops.clone(),panic:false});}
   drop(context);
  }));
  assert_eq!(result.err().unwrap().downcast_ref::<&'static str>(),Some(&"original lifecycle panic"));assert_eq!(drops.load(Ordering::SeqCst),1);
  assert_eq!(source_drops.load(Ordering::SeqCst),row["expected"]["errorFrees"].as_u64().unwrap() as usize);
 }
 let drops=Arc::new(AtomicUsize::new(0));let source_drops=Arc::new(AtomicUsize::new(0));
 let context=PackTransportContext::try_new(TransportAdmission::new(4096,16),LifecyclePolicy{phase:PackTransportPhase::ContextClosed,armed:Arc::new(std::sync::atomic::AtomicBool::new(true)),drops:drops.clone()}).ok().unwrap();
 let error=context.reserve::<DropSentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).unwrap().publish(DropSentinel{drops:source_drops.clone(),panic:true});drop(context);
 let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||drop(error)));
 assert_eq!(result.err().unwrap().downcast_ref::<&'static str>(),Some(&"original source destructor panic"));assert_eq!(source_drops.load(Ordering::SeqCst),1);assert_eq!(drops.load(Ordering::SeqCst),1);
 eprintln!("[DEBUG] four lifecycle callback panics and concrete source Drop panic retained their original panic and dropped policy/source once");
}

#[repr(align(128))]
#[derive(Debug)]
struct AlignedSentinel(Arc<AtomicUsize>);
impl std::fmt::Display for AlignedSentinel{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("aligned source")}}
impl Error for AlignedSentinel{}
impl Drop for AlignedSentinel{fn drop(&mut self){self.0.fetch_add(1,Ordering::SeqCst);}}

#[test]
fn aligned_source_identity_survives_actual_concurrent_final_clone_drops(){
 let drops=Arc::new(AtomicUsize::new(0));let source_drops=Arc::new(AtomicUsize::new(0));
 let context=PackTransportContext::try_new(TransportAdmission::new(4096,0),Policy(drops.clone())).ok().unwrap();
 let error=context.reserve::<AlignedSentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).unwrap().publish(AlignedSentinel(source_drops.clone()));
 let source=error.source_error().downcast_ref::<AlignedSentinel>().unwrap() as *const AlignedSentinel;
 assert_eq!(source as usize%std::mem::align_of::<AlignedSentinel>(),0);
 let original=source as usize;let barrier=Barrier::new(3);let second=error.clone();drop(context);
 std::thread::scope(|scope|{
  scope.spawn(||{assert_eq!(error.source_error().downcast_ref::<AlignedSentinel>().unwrap() as *const AlignedSentinel as usize,original);barrier.wait();drop(error);});
  scope.spawn(||{assert_eq!(second.source_error().downcast_ref::<AlignedSentinel>().unwrap() as *const AlignedSentinel as usize,original);barrier.wait();drop(second);});
  barrier.wait();
 });
 assert_eq!(source_drops.load(Ordering::SeqCst),1);assert_eq!(drops.load(Ordering::SeqCst),1);
 eprintln!("[DEBUG] overaligned source retained identity across concurrent final-owner Drop; zero-size Sentinel is covered by other context laws");
}

#[test]
fn fallible_context_creation_returns_original_policy_and_admission_on_refusal(){
 let drops=Arc::new(AtomicUsize::new(0));let policy=Policy(drops.clone());
 let result=create_with(TransportAdmission::new(4096,13),policy,|_|false).err().unwrap();
 assert_eq!(result.admission.committed_bytes(),13);assert!(Arc::ptr_eq(&result.policy.0,&drops));
 assert!(matches!(result.cause,TransportContextRefusalCause::Admission(TransportCaptureRefusal{kind:ValueRefusalKind::AllocationFailed,allocated_bytes:0,physical_bytes:0,..})));
 assert_eq!(drops.load(Ordering::SeqCst),0);drop(result);assert_eq!(drops.load(Ordering::SeqCst),1);
 let bytes=std::mem::size_of::<ContextCell<Policy>>();
 let policy=Policy(drops.clone());
 let result=create_with(TransportAdmission::new(bytes,0),policy,|storage|storage.try_reserve_exact(2).is_ok()).err().unwrap();
 assert_eq!(result.admission.committed_bytes(),0);
 assert!(matches!(result.cause,TransportContextRefusalCause::Admission(TransportCaptureRefusal{kind:ValueRefusalKind::OwnershipLimit,allocated_bytes:0,physical_bytes,..}) if physical_bytes>=bytes*2));
}

#[test]
fn interleaved_unused_reserve_rolls_back_only_its_charge_and_source_retains_final_policy(){
 let drops=Arc::new(AtomicUsize::new(0));
 let context=PackTransportContext::try_new(TransportAdmission::new(4096,17),Policy(drops.clone())).ok().unwrap();
 let base=context.committed_bytes();assert_eq!(base,17+context.allocated_bytes());
 let a=context.reserve::<Sentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).unwrap();
 let a_bytes=a.allocated_bytes();
 let b=context.reserve::<Sentinel>(PackTransportCategory::HttpBody,PackRetryDisposition::Never).unwrap();
 let b_bytes=b.allocated_bytes();assert_eq!(context.committed_bytes(),base+a_bytes+b_bytes);
 drop(a);assert_eq!(context.committed_bytes(),base+b_bytes);
 let error=b.publish(Sentinel);let (_,cloned_bytes)=observe(||context.clone());assert_eq!(cloned_bytes,0);
 let (copy,copy_bytes)=observe(||error.clone());assert_eq!(copy_bytes,0);
 drop(error);assert_eq!(context.committed_bytes(),base+b_bytes);
 drop(context);assert_eq!(drops.load(Ordering::SeqCst),0);
 drop(copy);assert_eq!(drops.load(Ordering::SeqCst),1);
}

#[test]
fn actual_concurrent_reservations_release_the_ledger_lock_before_operation(){
 let drops=Arc::new(AtomicUsize::new(0));let context=PackTransportContext::try_new(TransportAdmission::new(4096,0),Policy(drops)).ok().unwrap();
 let base=context.committed_bytes();let barrier=Arc::new(Barrier::new(2));
 let published=std::thread::scope(|scope|{
  let a_context=context.clone();let a_barrier=barrier.clone();
  let unused=scope.spawn(move||{let reservation=a_context.reserve::<Sentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).unwrap();a_barrier.wait();let bytes=reservation.allocated_bytes();drop(reservation);bytes});
  let b_context=context.clone();let b_barrier=barrier.clone();
  let owner=scope.spawn(move||{let reservation=b_context.reserve::<Sentinel>(PackTransportCategory::HttpRequest,PackRetryDisposition::Transient).unwrap();b_barrier.wait();let bytes=reservation.allocated_bytes();(reservation.publish(Sentinel),bytes)});
  let (error,bytes)=owner.join().unwrap();let unused_bytes=unused.join().unwrap();assert!(unused_bytes>0);(error,bytes)
 });
 assert_eq!(context.committed_bytes(),base+published.1);drop(published.0);assert_eq!(context.committed_bytes(),base+published.1);
}

struct ReentrantPolicy{alias:Arc<Mutex<Option<PackTransportContext>>>,visits:Arc<AtomicUsize>}
impl PackTransportPolicy for ReentrantPolicy{
 fn checkpoint(&self,_:PackTransportProgress)->Result<(),semio_framework_value::ValueError>{
  if let Some(context)=self.alias.lock().unwrap().as_ref(){let _=context.committed_bytes();self.visits.fetch_add(1,Ordering::SeqCst);}
  Ok(())
 }
 fn progress(&self,_:PackTransportProgress){}
}

#[test]
fn progress_callbacks_can_observe_same_context_and_cancellation_prevents_reserve(){
 let alias=Arc::new(Mutex::new(None));let visits=Arc::new(AtomicUsize::new(0));
 let context=PackTransportContext::try_new(TransportAdmission::new(4096,0),ReentrantPolicy{alias:alias.clone(),visits:visits.clone()}).ok().unwrap();
 *alias.lock().unwrap()=Some(context.clone());
 let reserve=context.reserve::<Sentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).unwrap();drop(reserve);
 assert!(visits.load(Ordering::SeqCst)>0);
 alias.lock().unwrap().take();let before=context.committed_bytes();context.cancel();
 let refusal=context.reserve::<Sentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).err().unwrap();
 assert_eq!(refusal.refusal_kind(),Some(ValueRefusalKind::Canceled));assert_eq!(context.committed_bytes(),before);
}

struct RefusingPolicy{phase:PackTransportPhase,error:Mutex<Option<semio_framework_value::ValueError>>}
impl PackTransportPolicy for RefusingPolicy{
 fn checkpoint(&self,progress:PackTransportProgress)->Result<(),semio_framework_value::ValueError>{if progress.phase==self.phase{Err(self.error.lock().unwrap().take().unwrap())}else{Ok(())}}
 fn progress(&self,_:PackTransportProgress){}
}

#[test]
fn all_original_typed_policy_kinds_and_message_allocations_survive_context_refusal(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧭️cause/🔌️capture/👥️context/🎛️policy/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let kind=match row["kind"].as_str().unwrap(){"invalidValue"=>ValueRefusalKind::InvalidValue,"canceled"=>ValueRefusalKind::Canceled,"ownershipLimit"=>ValueRefusalKind::OwnershipLimit,"allocationFailed"=>ValueRefusalKind::AllocationFailed,"workLimit"=>ValueRefusalKind::WorkLimit,"depthLimit"=>ValueRefusalKind::DepthLimit,"unsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"invariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("unknown closed policy kind")};
  let phase=match row["phase"].as_str().unwrap(){"contextBefore"=>PackTransportPhase::ContextBefore,"contextAfter"=>PackTransportPhase::ContextAfter,"before"=>PackTransportPhase::BeforeReserve,"after"=>PackTransportPhase::AfterReserve,_=>panic!("unknown closed policy phase")};
  let original=semio_framework_value::ValueError::new(kind,row["message"].as_str().unwrap());let pointer=original.message.as_ptr();
  let policy=RefusingPolicy{phase,error:Mutex::new(Some(original))};
  match PackTransportContext::try_new(TransportAdmission::new(4096,16),policy){
   Err(fault)=>{let TransportContextRefusalCause::Value(error)=fault.cause else{panic!("owned creation refusal was erased")};assert_eq!(error.kind,kind);assert_eq!(error.message.as_ptr(),pointer);assert_eq!(fault.admission.committed_bytes(),16);},
   Ok(context)=>{
    let before=context.committed_bytes();let error=context.reserve::<Sentinel>(PackTransportCategory::NativeIo,PackRetryDisposition::Never).err().unwrap();
    let PackError::Refusal(crate::PackRefusal::ValueRefusal(cause))=&error else{panic!("owned operation policy refusal was erased")};
    assert_eq!(cause.kind,kind);assert_eq!(cause.message.as_ptr(),pointer);assert_eq!(context.committed_bytes(),before);
    assert!(std::ptr::eq(std::error::Error::source(&error).unwrap().downcast_ref::<semio_framework_value::ValueError>().unwrap(),cause));
   }
  }
 }
 eprintln!("[DEBUG] native context draft retained32 original typed policy causes and message allocations");
}
