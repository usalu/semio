//! 🧪️ The original canonical issuer posts every physical birth and return through one actual creation source.
use super::*;
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;

fn now()->Option<u64>{Some(1000)}

#[doc(hidden)]
pub fn cancellation_issuer_original_creation_physical_birth_and_close(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let paid=fixture["cases"].as_array().unwrap().iter().find(|row|row["expected"]=="admitted").unwrap();
 let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:134217728,maximum_capacity_bytes:268435456,maximum_release_bytes:536870912,maximum_depth:4096};let phase=RetainedCloneGrant{maximum_items:paid["items"].as_u64().unwrap()as usize,maximum_capacity_bytes:paid["capacityBytes"].as_u64().unwrap()as usize,maximum_copy_bytes:65536,maximum_release_bytes:16777216,maximum_depth:4096};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(fixture["sourceIdentity"][0].as_u64().unwrap()),Generation(fixture["sourceIdentity"][1].as_u64().unwrap()),stock,now,1500,32768);caller.audit_heap();
 let(mut born,mut released)=(0usize,0usize);let mut cx=caller.lend_context();let identity=cx.original_recipient_identity();
 let(mut issuer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(ToolCancellationIssuer::empty);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(issuer.terminal_is_empty());assert!(issuer.handle().is_none());
 for _ in 0..32{
  if issuer.ready(){break}
  for denied in [RetainedCloneGrant{maximum_items:0,..phase},RetainedCloneGrant{maximum_capacity_bytes:0,..phase}]{
   if denied.maximum_items>0&&(issuer.admission.source.is_none()||issuer.admission.state.is_some()){continue}
   let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|{let original=same.original_source_parent()?;issuer.admit_step(original,same)}));
   assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
  }
  let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|{let original=same.original_source_parent()?;issuer.admit_step(original,same)}));assert!(!result.unwrap());let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));assert_eq!(cx.original_recipient_identity(),identity);
 }
 assert!(issuer.ready());let before=cx.retained_progress();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let borrowed=issuer.handle().unwrap();assert!(Arc::ptr_eq(&borrowed.state,&issuer.issued.as_ref().unwrap().state))});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(issuer.admission.source.as_ref().unwrap().cancellation().unwrap()as*const _,cx.original_source_parent().unwrap().cancellation().unwrap()as*const _);

 for keyed in [false,true]{
  let key=ToolOperationKey{app_instance_id:71,document:ArtifactDocumentAuthority(71),operation_id:OperationId(if keyed{72}else{71}),base_revision:semio_framework_job::RevisionId(1),generation:Generation(3)};let mut lease=None;
  for turn in 0..32{
   let denied=RetainedCloneGrant{maximum_items:0,..phase};let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|{lease=if keyed{issuer.begin_keyed(&key,same)?}else{issuer.begin(&key,same)?};Ok(lease.is_some())}));assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
   let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|{lease=if keyed{issuer.begin_keyed(&key,same)?}else{issuer.begin(&key,same)?};Ok(lease.is_some())}));let ready=result.unwrap();assert_eq!(ready,lease.is_some());let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));
   if turn==0{
    let mut foreign=OriginalJobFixtureCaller::admit(cx.operation(),cx.generation(),stock,now,1500,32768);let mut foreign_lease=None;
    let mut foreign_cx=foreign.lend_context();let before=foreign_cx.retained_progress();let fuel=foreign_cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign_cx.with_retained_loan(phase,|same|{foreign_lease=if keyed{issuer.begin_keyed(&key,same)?}else{issuer.begin(&key,same)?};Ok(foreign_lease.is_some())}));assert!(result.is_err());assert!(foreign_lease.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(foreign_cx.retained_progress(),before);assert_eq!(foreign_cx.fuel_remaining(),fuel);drop(result);let remaining=foreign_cx.fuel_remaining();drop(foreign_cx);foreign.settle_lent_fuel(remaining);
    foreign.retire_exact();
   }
   if lease.is_some(){break}assert!(turn<31,"original keyed={keyed} cancellation lease birth stalled");
  }
  assert!(lease.is_some());let mut close=None;
  for turn in 0..128{
   let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|->Result<_,OriginalTurnFailure>{let grant=same.retained_grant();let(progress,posted)=if let Some(owner)=close.as_mut(){let owner:&mut Box<crate::ToolCancellationLeaseRetirement>=owner;if owner.terminal_is_empty(){(crate::original_lease_cursor_release(&mut close,grant)?,false)}else{let handle=issuer.handle().unwrap();let demand=owner.retirement_demands(handle,grant.maximum_copy_bytes)?;(semio_framework_job::close_original_effect_step(same,demand,|effect|owner.close_step(handle,effect))?.progress(),true)}}else{let(owner,progress)=crate::original_lease_cursor_admit(&mut lease,grant)?;close=owner;(progress,false)};if !posted{same.consume_retained(progress)?;if progress.copied_items>0{same.consume_fuel(1)}}Ok(progress)}));let progress=result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(cx.retained_progress(),before.checked_add(progress).unwrap());born+=heap.requested_bytes;released+=heap.released_bytes;if lease.is_none()&&close.is_none(){break}assert!(turn<127,"original keyed={keyed} lease close stalled");
  }
  assert_eq!(issuer.active_operation_count(),0);
 }
 for turn in 0..128{
  let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|issuer.renew_scope_generation(same)));let ready=result.unwrap();let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));if ready{break}assert!(turn<127,"original paid cancellation renewal stalled");
 }
 for turn in 0..128{if issuer.terminal_is_empty(){break}let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(phase,|same|{let original=same.original_source_parent()?;issuer.close_step(original,same)}));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(cx.retained_progress(),before.checked_add(progress).unwrap());assert_eq!(cx.original_recipient_identity(),identity);born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn<127,"original canonical issuer close stalled");}
 assert!(issuer.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(issuer));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let remaining=cx.fuel_remaining();drop(cx);caller.settle_lent_fuel(remaining);caller.retire_exact();assert_eq!(born,released);
}

/// 🚧️ An issuer whose birth was never paid refuses a scope renewal typed after its bounded number of unready answers, instead of answering `false` forever.
#[doc(hidden)]
pub fn cancellation_issuer_unready_renewal_refuses_typed_after_its_bound(){
 let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:134217728,maximum_capacity_bytes:268435456,maximum_release_bytes:536870912,maximum_depth:4096};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock,now,1500,32768);let mut cx=caller.lend_context();
 let mut issuer=ToolCancellationIssuer::empty();assert!(!issuer.ready());
 for attempt in 1..ToolCancellationIssuer::MAXIMUM_UNREADY_RENEWALS{assert!(!issuer.renew_scope_generation(&mut cx).unwrap(),"attempt {attempt} stays a plain refusal below the bound");}
 let refusal=issuer.renew_scope_generation(&mut cx).unwrap_err();assert!(format!("{refusal:?}").contains("never became ready"));
 let remaining=cx.fuel_remaining();drop(cx);drop(issuer);caller.settle_lent_fuel(remaining);caller.retire_exact();
}
