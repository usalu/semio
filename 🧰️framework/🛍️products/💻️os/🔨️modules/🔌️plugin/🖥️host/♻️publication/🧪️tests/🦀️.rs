//! 📤️ Exercises the original publication Vec pointer and each physical release frontier.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
#[test]
fn original_host_publication_source_preserves_native_capacity_and_all_physical_receipts(){
 const POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
 let(_,selfcheck)=observe(||drop(vec![0_u8;37]));assert_eq!(selfcheck,(37,37));
 for(length,capacity)in[(0,0),(1,8192),(8193,65536)]{
  let mut original=Vec::with_capacity(capacity);original.resize(length,7_u8);let pointer=original.as_ptr();let actual_capacity=original.capacity();
  let mut owner=GuestRelayPublication{kind:GuestRelayPublicationKind::Commit,source:std::mem::ManuallyDrop::new(original),cursor:0,copied:true,retired:false,oversized:false,delivered:false,writer:None};
  let mut total=RetainedCloneProgress::default();let mut turns=0;
  loop{
   let demand=owner.original_source_demand();
   for denied in [RetainedCloneGrant{maximum_items:0,..POLICY},RetainedCloneGrant{maximum_depth:0,..POLICY},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..POLICY},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..POLICY}]{
    if owner.source.is_empty()&&owner.source.capacity()==0{break;}if denied.maximum_copy_bytes>=demand.copy_bytes&&denied.maximum_release_bytes>=demand.release_bytes&&denied.maximum_items>0&&denied.maximum_depth>=demand.depth{continue;}
    let before=(owner.source.len(),owner.source.capacity(),owner.source.as_ptr());let(step,heap)=observe(||owner.close_original_source(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(before,(owner.source.len(),owner.source.capacity(),owner.source.as_ptr()));
   }
   let(step,heap)=observe(||owner.close_original_source(POLICY).unwrap());let progress=step.progress();assert!(progress.fits(POLICY));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));total=total.checked_add(progress).unwrap();turns+=1;
   if owner.source.capacity()!=0{assert_eq!(owner.source.as_ptr(),pointer);assert_eq!(owner.source.capacity(),actual_capacity);}
   if matches!(step,RetainedCloneStep::Complete(_)){break;}assert!(progress.copied_items>0);assert!(turns<16);
  }
  assert_eq!(total.released_bytes,actual_capacity);assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));println!("[DEBUG] actual Host publication original source length={length} capacity={actual_capacity} turns={turns} copied={} physical={} all denied0heap nativepointerstable terminalDrop0; writer/semanticJ unsupported",total.copied_bytes,total.released_bytes);
 }
}
