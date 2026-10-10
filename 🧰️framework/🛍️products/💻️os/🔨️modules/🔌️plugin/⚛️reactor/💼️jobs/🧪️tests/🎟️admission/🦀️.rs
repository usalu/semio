//! 🪺️ The actual original registry transfers only paid handles, including its first cold admission.
use super::*;
use semio_framework_job::{Generation,OperationId,StepBudget,StepContext,root_cancel_token};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};

/// 🧫️ Cold authority setup and source construction precede the measured actual registry scope.
#[test]
fn original_job_registry_cold_admission_denial_and_saturation_preserve_sources(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();
 let raw=fixture["text"].as_str().unwrap().repeat(fixture["repeat"].as_u64().unwrap()as usize).into_bytes();
 let g=&fixture["registryGrant"];let grant=RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize};
 let operation=OperationId(fixture["caller"]["operation"].as_u64().unwrap());let generation=Generation(fixture["caller"]["generation"].as_u64().unwrap());let cancel=root_cancel_token();let mut sequence=0;
 let mut source=Some(OriginalJobAdmission{job:1,kind:JOB_KIND_INFER.into(),input:Some(raw.clone()),checkpoint:None});let original=source.as_ref().unwrap().input.as_ref().unwrap().as_ptr();
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
  let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,denied),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::__async::poll::resolve_ready(start_job(&mut source,&mut cx)));
  assert!(!result.unwrap());assert_eq!(source.as_ref().unwrap().input.as_ref().unwrap().as_ptr(),original);assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
 for job in 1..=JOB_SLOTS as u64{
  if job!=1{source=Some(OriginalJobAdmission{job,kind:JOB_KIND_INFER.into(),input:Some(raw.clone()),checkpoint:None})}
  let pointer=source.as_ref().unwrap().input.as_ref().unwrap().as_ptr();let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::__async::poll::resolve_ready(start_job(&mut source,&mut cx)));
  assert!(result.unwrap());assert!(source.is_none());assert_eq!(cx.retained_progress(),RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  JOBS.with(|slots|{let slots=slots.borrow();let slot=slots.iter().flatten().find(|slot|slot.job==job).unwrap();assert_eq!(slot.source.as_ref().unwrap().input.as_ref().unwrap().as_ptr(),pointer)});
 }
 source=Some(OriginalJobAdmission{job:JOB_SLOTS as u64+1,kind:JOB_KIND_INFER.into(),input:Some(raw),checkpoint:None});let pointer=source.as_ref().unwrap().input.as_ref().unwrap().as_ptr();let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::__async::poll::resolve_ready(start_job(&mut source,&mut cx)));
 assert!(!result.unwrap());assert_eq!(source.as_ref().unwrap().input.as_ref().unwrap().as_ptr(),pointer);assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(cx);
 let cleanup=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};
 JOBS.with(|slots|{let mut slots=slots.borrow_mut();for entry in slots.iter_mut(){let Some(slot)=entry.as_mut()else{continue};for _ in 0..10000{let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,cleanup),cancel.clone(),||Some(1),&mut sequence,&mut receipt);if close_body(slot,&mut cx).unwrap()&&close_original(&mut slot.source,&mut slot.source_close,&mut cx).unwrap(){break}}assert!(slot.body.is_none()&&slot.source.is_none()&&slot.source_close.is_none());*entry=None}});
 let mut closing=None;for _ in 0..10000{let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,cleanup),cancel.clone(),||Some(1),&mut sequence,&mut receipt);if close_original(&mut source,&mut closing,&mut cx).unwrap(){break}}assert!(source.is_none()&&closing.is_none());
 eprintln!("[DEBUG] original job registry coldAdmissionSystem0=true deniedSourceSame=true saturatedSourceSame=true exactMetadataReceipt=true; cleanup outside measured admission scope");
}

/// 🔬️ Every real source and static candidate byte needs its own original copy turn.
#[test]
fn original_job_kind_lookup_preserves_denied_frontiers_and_whole_sources(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️extension/🧫️fixtures/📨️invoke/🔣️.json")).unwrap();
 let factory=builtin_factory(JOB_KIND_IO_SNIFF).unwrap();for kind in ["semio.artifact-infer","semio.artifact-infer-long","未知","未知🌊x"]{assert!(register_bounded_job_kind(kind,factory))}
 let operation=OperationId(fixture["caller"]["operation"].as_u64().unwrap());let generation=Generation(fixture["caller"]["generation"].as_u64().unwrap());
 let value=&fixture["executionGrant"];let original_grant=RetainedCloneGrant{maximum_items:value["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:value["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:value["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:value["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap()as usize};let mut cases=0;
 for row in fixture["lookupCases"].as_array().unwrap(){for copy in [1,3,64]{for cut in [None,Some(0),Some(3),Some(17)]{
  let mut source=Some(row["input"].as_str().unwrap().to_string());let pointer=source.as_ref().unwrap().as_ptr();let cancel=root_cancel_token();let mut sequence=0;let mut cursor=OriginalJobKindLookup::new();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};let mut terminal=false;
  for turn in 0..10000{
   if cut==Some(turn){cancel.cancel_now()}
   let demand=cursor.demand(source.as_ref().unwrap());let before=(cursor.epoch,cursor.index,cursor.offset,cursor.left,cursor.candidate.map(|candidate|candidate.kind));
   for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant}]{
    if denied.maximum_items>0&&denied.maximum_depth>0&&demand.maximum_copy_bytes==0{continue}
    let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,denied),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(source.as_ref().unwrap(),&mut cx));assert!(matches!(step.unwrap(),OriginalJobKindLookupStep::Pending));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((cursor.epoch,cursor.index,cursor.offset,cursor.left,cursor.candidate.map(|candidate|candidate.kind)),before);assert_eq!(source.as_ref().unwrap().as_ptr(),pointer);
   }
   let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(source.as_ref().unwrap(),&mut cx));let step=step.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(cx.retained_progress().fits(grant));assert!(cx.retained_progress().copied_bytes<=1);assert_eq!(source.as_ref().unwrap().as_ptr(),pointer);
   if cancel.is_cancelled_now(){assert!(matches!(step,OriginalJobKindLookupStep::Pending));assert_eq!(cx.retained_progress(),RetainedCloneProgress::default());terminal=true;break}
   match step{OriginalJobKindLookupStep::Pending=>{},OriginalJobKindLookupStep::Found(candidate)=>{let expected=row["matched"].as_u64().unwrap()as usize;assert_eq!(candidate.kind,row["candidates"][expected].as_str().unwrap());terminal=true;break},OriginalJobKindLookupStep::Missing=>{assert!(row["matched"].is_null());terminal=true;break}}
  }
  assert!(terminal);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut closing=None;let mut emptied=false;
  for _ in 0..10000{let mut receipt=Default::default();let mut cx=StepContext::new(operation,generation,StepBudget::new(1,u64::MAX,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,..original_grant}),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_original(&mut source,&mut closing,&mut cx));assert_eq!((heap.requested_bytes,heap.released_bytes),(cx.retained_progress().retained_capacity_bytes,cx.retained_progress().released_bytes));if result.unwrap(){emptied=true;break}}
  assert!(emptied&&source.is_none()&&closing.is_none());cases+=1;
 }}}
 eprintln!("[DEBUG] original job kind lookup cases={cases} deniedFrontierSame=true originalPointerSame=true separateSourceAndStaticByteTurns=true SystemConservation=true terminalDrop0=true");
}
