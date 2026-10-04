//! 🧪️ Unmounted native pre-reservation laws; no native compilation or runtime verdict.
use super::*;
use std::sync::atomic::{AtomicUsize,Ordering};
use std::sync::Arc;

#[derive(Debug)]
struct Sentinel(Arc<AtomicUsize>);
impl std::fmt::Display for Sentinel{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("same deliberately misleading permanent cancellation prose")}}
impl std::error::Error for Sentinel{}
impl Drop for Sentinel{fn drop(&mut self){self.0.fetch_add(1,Ordering::SeqCst);}}

#[test]
fn pre_reserved_transport_shared_cell_keeps_concrete_identity_category_retry_and_last_owner_drop(){
 let mut admission=TransportAdmission::new(4096,11);
 let mut allocator_calls=0;
 let (reservation,reservation_requested_bytes)=crate::test_allocation::observe(||reserve_with::<Sentinel,_,_>(&mut admission,PackTransportCategory::HttpBody,PackRetryDisposition::Never,|_,_|true,|storage|{allocator_calls+=1;storage.try_reserve_exact(2).is_ok()}).unwrap());
 let allocated=reservation.allocated_bytes();
 assert!(reservation_requested_bytes>0);
 assert_eq!(reservation_requested_bytes,allocated);
 let releases=Arc::new(AtomicUsize::new(0));
 let error=reservation.publish(Sentinel(releases.clone()));
 assert_eq!(admission.committed_bytes(),11+allocated);
 let original=error.source_error() as *const _ as *const ();
 let (copy,clone_bytes)=crate::test_allocation::observe(||error.clone());
 assert_eq!(clone_bytes,0);
 let (projection,projection_bytes)=crate::test_allocation::observe(||{
  let pack=crate::PackError::TransportFailure(copy);
  let cause=std::error::Error::source(&pack).unwrap() as *const _ as *const ();
  let kind=pack.cause_kind();let refusal=pack.refusal_kind();
  (pack,cause,kind,refusal)
 });
 assert_eq!(projection_bytes,0);
 let (projected,projected_source,projected_kind,projected_refusal)=projection;
 assert_eq!(projected_source,original);
 assert_eq!(projected_kind,crate::PackCauseKind::Transport(PackTransportCategory::HttpBody));
 assert_eq!(projected_refusal,None);
 let crate::PackError::TransportFailure(copy)=projected else{panic!("transport source projection erased")};
 assert_eq!(copy.source_error() as *const _ as *const (),original);
 assert!(copy.source_error().downcast_ref::<Sentinel>().is_some());
 assert_eq!(copy.category(),PackTransportCategory::HttpBody);
 assert_eq!(copy.retry(),PackRetryDisposition::Never);
 assert_eq!(copy.allocated_bytes(),allocated);
 assert_eq!(allocator_calls,1);
 drop(error);assert_eq!(releases.load(Ordering::SeqCst),0);
 drop(copy);assert_eq!(releases.load(Ordering::SeqCst),1);
 assert_eq!(admission.committed_bytes(),11+allocated);
 eprintln!("[DEBUG] native transport draft retained one concrete source in {allocated} physical cell bytes");
}

#[test]
fn provider_admission_refusals_prevent_io_and_unused_reserves_roll_back(){
 let size=std::mem::size_of::<TransportCell<Sentinel>>();
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧭️cause/🔌️capture/🚦️provider/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let scale=|value:u64|usize::try_from(value).unwrap()*size/64;
  let before=scale(row["ownedBefore"].as_u64().unwrap());
  let mut admission=TransportAdmission::new(scale(row["maximumBytes"].as_u64().unwrap()),before);
  let category=match row["category"].as_str().unwrap(){"nativeIo"=>PackTransportCategory::NativeIo,"httpRequest"=>PackTransportCategory::HttpRequest,"httpBody"=>PackTransportCategory::HttpBody,_=>panic!("unknown category")};
  let retry=match row["retry"].as_str().unwrap(){"never"=>PackRetryDisposition::Never,"transient"=>PackRetryDisposition::Transient,_=>panic!("unknown retry")};
  let mut operations=0;
  {
  let result=reserve_with::<Sentinel,_,_>(&mut admission,category,retry,|_,_|true,|storage|storage.try_reserve_exact(usize::try_from(row["allocatedBytes"].as_u64().unwrap()/64).unwrap()).is_ok());
  match result{
   Err(refusal)=>{assert_eq!(refusal.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(refusal.allocated_bytes,0);},
   Ok(reservation)=>{
    operations+=1;
    if row["outcome"].as_str().unwrap()=="success"{drop(reservation);}else{let error=reservation.publish(Sentinel(Arc::new(AtomicUsize::new(0))));assert_eq!(error.category(),category);assert_eq!(error.retry(),retry);drop(error);}
   }
  }
  assert_eq!(operations,row["expected"]["operationCalls"].as_u64().unwrap());
  }
  assert_eq!(admission.committed_bytes(),scale(row["expected"]["committedAfter"].as_u64().unwrap()));
 }
 for stage in [TransportCaptureStage::Before,TransportCaptureStage::After]{
  let mut admission=TransportAdmission::new(size*4,3);
  let refusal=reserve_with::<Sentinel,_,_>(&mut admission,PackTransportCategory::NativeIo,PackRetryDisposition::Never,|current,_|current!=stage,|storage|storage.try_reserve_exact(1).is_ok()).err().unwrap();
  assert_eq!(refusal.kind,ValueRefusalKind::Canceled);assert_eq!(refusal.allocated_bytes,0);assert_eq!(admission.committed_bytes(),3);
  if stage==TransportCaptureStage::After{assert!(refusal.physical_bytes>=size);}else{assert_eq!(refusal.physical_bytes,0);}
 }
 let mut admission=TransportAdmission::new(size*2,0);
 let refused=reserve_with::<Sentinel,_,_>(&mut admission,PackTransportCategory::NativeIo,PackRetryDisposition::Never,|_,_|true,|_|false).err().unwrap();
 assert_eq!(refused.kind,ValueRefusalKind::AllocationFailed);assert_eq!(admission.committed_bytes(),0);
 let undergrant=reserve_with::<Sentinel,_,_>(&mut admission,PackTransportCategory::NativeIo,PackRetryDisposition::Never,|_,_|true,|_|true).err().unwrap();
 assert_eq!(undergrant.kind,ValueRefusalKind::InvariantViolated);assert_eq!(admission.committed_bytes(),0);
 let overgrant=reserve_with::<Sentinel,_,_>(&mut admission,PackTransportCategory::NativeIo,PackRetryDisposition::Never,|_,_|true,|storage|storage.try_reserve_exact(3).is_ok()).err().unwrap();
 assert_eq!(overgrant.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(overgrant.allocated_bytes,0);assert!(overgrant.physical_bytes>=size*3);assert_eq!(admission.committed_bytes(),0);
 eprintln!("[DEBUG] native transport draft covered pre-operation admission, actual-capacity refusal and rollback");
}
