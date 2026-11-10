//! 🎟️ Genuine one-shot returned-place credits conserve one original source at quantum one.
use semio_framework_job_context::*;
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn clock()->Option<u64>{Some(0)}
fn retire_source(mut caller:OriginalJobFixtureCaller,stock:RetainedCloneGrant,extra:usize)->usize{let expected=caller.born_bytes()+extra;let mut released=0;for _ in 0..64{if caller.terminal_is_empty(){break}released+=caller.retire_step(stock).unwrap().progress().released_bytes}assert!(caller.terminal_is_empty());assert_eq!(released,expected);caller.cumulative_receipt().copied_bytes}
#[test]
fn original_return_place_keeps_foreign_busy_cancelled_and_recorded_custody_under_one_bank(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../case.schema.json")).unwrap();for(k,v)in law.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v);}let s=&law["source"];let stock=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let q=RetainedCloneGrant{maximum_items:1,..stock};
 for(index,row)in law["cases"].as_array().unwrap().iter().enumerate(){
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(7620+index as u64),Generation(2),stock,clock,s["deadlineUs"].as_u64().unwrap(),s["fuel"].as_u64().unwrap());let mut place=OriginalCallerReturnPlace::empty();let mut source_extra=0;let mut born=false;for _ in 0..32{caller.turn(q,|cx|{born=cx.admit_original_return_table()?;Ok(())}).unwrap();source_extra+=caller.receipt().retained_capacity_bytes;if born{break}}assert!(born);let before=caller.turn(q,|cx|Ok(cx.original_total_fuel_remaining())).unwrap();let mut ready=false;
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){caller.turn(q,|cx|cx.with_original_work_loan(1,cx.original_absolute_deadline_us(),|cx|{let work=cx.original_work_units_spent();let(result,heap)=observe(||place.admit(cx));ready=result?;assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(cx.original_work_units_spent()-work<=1);Ok(())})).unwrap();if ready{break}}assert!(ready);
  let held=caller.turn(q,|cx|Ok(cx.original_total_fuel_remaining())).unwrap();assert_eq!(before-held,law["boundSourceWork"].as_u64().unwrap()+law["claimWork"].as_u64().unwrap()+law["heldReturnFuel"].as_u64().unwrap());
  assert!(law["heldClaimsKeepSourceBorrowable"].as_bool().unwrap());let source=caller.turn(q,|cx|Ok(cx.original_source_parent()?.source_identity()?)).unwrap();let fuel=caller.fuel_remaining();let(turn,heap)=observe(||caller.retire_step(q));assert_eq!(turn.unwrap().progress(),Default::default());assert_eq!(caller.fuel_remaining(),fuel);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.turn(q,|cx|{assert_eq!(cx.original_source_parent()?.source_identity()?,source);Ok(())}).unwrap();
  caller.turn(q,|cx|{let mut preview=0;let mut p=RetainedCloneProgress::default();let parent=cx.original_source_parent()?;let(result,heap)=observe(||parent.borrow_context(StepBudget::new(1,cx.original_absolute_deadline_us(),q),clock,&mut preview,&mut p));assert!(result.is_err());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(place.is_admitted());assert_eq!(cx.original_total_fuel_remaining(),held);Ok(())}).unwrap();
  if row["recordReturn"].as_bool().unwrap(){let(result,heap)=observe(||place.record_return());result.unwrap();assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let mut foreign=OriginalJobFixtureCaller::admit(OperationId(7699),Generation(2),stock,clock,s["deadlineUs"].as_u64().unwrap(),s["fuel"].as_u64().unwrap());foreign.turn(q,|cx|{let before=cx.retained_progress();let work=cx.original_work_units_spent();let(result,heap)=observe(||place.settle_return(cx));assert!(result.is_err());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.original_work_units_spent(),work);assert!(place.is_admitted());Ok(())}).unwrap();foreign.retire(stock);
  if row["cancel"].as_bool().unwrap(){caller.cancel().cancel_now();}
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){if place.terminal_is_empty(){break}caller.turn(q,|cx|{let work=cx.original_work_units_spent();let(result,heap)=observe(||place.settle_return(cx));result?;assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.original_work_units_spent()-work,1);Ok(())}).unwrap();}
  assert!(place.terminal_is_empty());let after=caller.turn(q,|cx|Ok(cx.original_total_fuel_remaining())).unwrap();assert_eq!(before-after,row["finalDebit"].as_u64().unwrap());caller.turn(q,|cx|{let work=cx.original_work_units_spent();assert!(place.settle_return(cx)?);assert_eq!(cx.original_work_units_spent(),work);Ok(())}).unwrap();let(_,heap)=observe(||drop(place));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  retire_source(caller,stock,source_extra);
 }
}

/// 📐️ Runs one sized admission for `requested` bytes on a stock of `copy` bytes and reports whether it was admitted; everything is closed again with separately funded closure.
fn sized_admission_fits(index:usize,copy:usize,requested:usize)->bool{
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:copy,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(8600+index as u64),Generation(4),stock,clock,10000,128);let mut place=OriginalCallerReturnPlace::empty();let mut destination=vec![0u8;requested];let address=destination.as_mut_ptr() as usize;
 let mut admitted=false;for _ in 0..24{let result=caller.turn(q,|cx|{admitted=place.admit_sized(cx,address,requested)?;Ok(())});if result.is_err()||admitted{break}}
 caller.fund_closure(1<<16,1<<8);caller.cancel().cancel_now();for _ in 0..8{if place.terminal_is_empty(){break}caller.turn(q,|cx|{place.settle_return(cx)?;Ok(())}).unwrap();}
 for _ in 0..64{if caller.terminal_is_empty(){break}caller.retire_step(stock).unwrap();}assert!(caller.terminal_is_empty()&&place.terminal_is_empty());admitted
}
/// 📐️ The smallest stock copy that admits one sized byte is the measured non-destination cost; closure copy and items are measured from the same cycle.
fn measured_sized_admission_copy(index:usize,big:RetainedCloneGrant)->(usize,usize,usize){
 let q=RetainedCloneGrant{maximum_items:1,..big};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8590+index as u64),Generation(4),big,clock,10000,128);let birth=caller.cumulative_receipt().copied_bytes;let mut place=OriginalCallerReturnPlace::empty();let mut destination=[0u8;1];let address=destination.as_mut_ptr() as usize;
 let mut admitted=false;for _ in 0..24{caller.turn(q,|cx|{admitted=place.admit_sized(cx,address,1)?;Ok(())}).unwrap();if admitted{break}}assert!(admitted);
 caller.cancel().cancel_now();for _ in 0..8{if place.terminal_is_empty(){break}caller.turn(q,|cx|{place.settle_return(cx)?;Ok(())}).unwrap();}assert!(place.terminal_is_empty());
 let before=caller.cumulative_receipt();for _ in 0..64{if caller.terminal_is_empty(){break}caller.retire_step(big).unwrap();}assert!(caller.terminal_is_empty());let after=caller.cumulative_receipt();
 let upper=before.copied_bytes+4096;let(mut low,mut high)=(birth,upper);while low<high{let mid=low+(high-low)/2;if sized_admission_fits(index,mid,1){high=mid}else{low=mid+1}}
 (low,after.copied_bytes-before.copied_bytes,after.copied_items-before.copied_items)
}
#[test]
fn sized_return_receipt_charges_only_the_original_captured_destination(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../sized-case.json")).unwrap();
 for (index,row) in law["cases"].as_array().unwrap().iter().enumerate(){
  let metadata=semio_framework_job_payload::OriginalReturnEscrowAccounting::UPDATE;let local_ack=std::mem::size_of::<u8>()+std::mem::size_of::<u32>();let big=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let measured=measured_sized_admission_copy(index,big);let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:measured.0-1+law["grantBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(8800+index as u64),Generation(4),stock,clock,10000,128);let mut place=OriginalCallerReturnPlace::empty();let mut destination=[0u8;48];let address=destination.as_mut_ptr() as usize;let requested=row["requested"].as_u64().unwrap() as usize;
  let mut source_extra=0;let mut admitted=false;let mut refused=false;for _ in 0..24{let result=caller.turn(q,|cx|{admitted=place.admit_sized(cx,address,requested)?;Ok(())});source_extra+=caller.receipt().retained_capacity_bytes;if result.is_err(){refused=true;break}if admitted{break}}
  assert_eq!(admitted,row["admitted"].as_bool().unwrap());assert_eq!(refused,!admitted);
  if admitted{
   caller.turn(q,|cx|{let snapshot=semio_framework_job_context::testkit::return_budget_snapshot(cx);assert!(snapshot.conserved());assert_eq!(snapshot.held_copy_bytes,requested+2*metadata+local_ack);Ok(())}).unwrap();
   let mut foreign=OriginalJobFixtureCaller::admit(OperationId(8899),Generation(4),stock,clock,10000,128);
   foreign.turn(q,|cx|{let progress=cx.retained_progress();assert!(place.qualify_sized(cx,address,requested).is_err());assert!(place.settle_return(cx).is_err());assert_eq!(cx.retained_progress(),progress);Ok(())}).unwrap();foreign.retire(stock);
   caller.turn(q,|cx|{place.qualify_sized(cx,address,requested)?;assert!(place.qualify_sized(cx,address+1,requested).is_err());assert!(place.qualify_sized(cx,address,requested+1).is_err());Ok(())}).unwrap();assert!(place.record_sized_return(address+1,requested).is_err());
   if row["captured"].as_bool().unwrap(){place.record_sized_return(address,requested).unwrap();caller.turn(q,|cx|place.qualify_sized(cx,address,requested)).unwrap();}else{caller.cancel().cancel_now();}
   let mut copied=0;let return_grant=if row["captured"].as_bool().unwrap(){RetainedCloneGrant{maximum_copy_bytes:0,..q}}else{q};for _ in 0..8{if place.terminal_is_empty(){break}caller.turn(return_grant,|cx|{let before=cx.retained_progress();let reserved_before=cx.reserved_return_progress();place.settle_return(cx)?;let actual=cx.retained_progress().copied_bytes-before.copied_bytes;let reserved=cx.reserved_return_progress().copied_bytes-reserved_before.copied_bytes;assert_eq!(actual,reserved);copied+=actual;Ok(())}).unwrap();}
   assert!(place.terminal_is_empty());assert_eq!(copied,row["spent"].as_u64().unwrap() as usize+if row["captured"].as_bool().unwrap(){2*metadata+local_ack}else{metadata+local_ack});
  }
  caller.fund_closure(measured.1,measured.2);retire_source(caller,stock,source_extra);
 }
}

#[test]
fn sized_return_cleanup_can_reserve_after_cancellation_on_the_original_retirement_loan(){
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8810),Generation(5),stock,clock,10000,128);let mut place=OriginalCallerReturnPlace::empty();let mut destination=[0u8;48];let address=destination.as_mut_ptr() as usize;caller.cancel().cancel_now();let mut source_extra=0;let mut admitted=false;
 for _ in 0..24{caller.turn(q,|cx|cx.with_original_retirement(|cx|{admitted=place.admit_sized(cx,address,destination.len())?;Ok(())})).unwrap();source_extra+=caller.receipt().retained_capacity_bytes;if admitted{break}}assert!(admitted);
 place.record_sized_return(address,destination.len()).unwrap();let mut copied=0;for _ in 0..8{if place.terminal_is_empty(){break}caller.turn(q,|cx|cx.with_original_retirement(|cx|{let before=cx.retained_progress();place.settle_return(cx)?;copied+=cx.retained_progress().copied_bytes-before.copied_bytes;Ok(())})).unwrap();}assert!(place.terminal_is_empty());let metadata=semio_framework_job_payload::OriginalReturnEscrowAccounting::UPDATE;assert_eq!(copied,destination.len()+2*metadata+std::mem::size_of::<u8>()+std::mem::size_of::<u32>());retire_source(caller,stock,source_extra);
}

#[test]
fn sized_return_epoch_watermark_is_paid_only_at_actual_birth(){
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8820),Generation(6),stock,clock,10000,128);let mut place=OriginalCallerReturnPlace::empty();let mut target=[0u8;8];let destination=target.as_mut_ptr()as usize;let mut watermark=0u64;let mut source_extra=0;
 caller.turn(q,|cx|{assert!(place.admit_sized_with_epoch(cx,destination,target.len(),&mut watermark).is_err());assert_eq!(watermark,0);Ok(())}).unwrap();let mut born=false;for _ in 0..24{caller.turn(q,|cx|{cx.bind_original_source_epoch(1)?;born=cx.admit_original_return_table()?;Ok(())}).unwrap();source_extra+=caller.receipt().retained_capacity_bytes;if born{break}}assert!(born);
 caller.turn(q,|cx|{cx.bind_original_source_epoch(2)?;let before=cx.retained_progress();assert!(place.admit_sized_with_epoch(cx,destination,target.len(),&mut watermark)?);let copy=cx.retained_progress().copied_bytes-before.copied_bytes;assert_eq!(copy,semio_framework_job_context::testkit::original_return_source::original_return_claim_copy_bytes()+std::mem::size_of::<Option<OriginalSourceBinding>>()+std::mem::size_of::<Option<(usize,usize)>>()+std::mem::size_of::<u64>());assert_eq!(watermark,2);Ok(())}).unwrap();
 caller.turn(q,|cx|{cx.bind_original_source_epoch(3)?;let before=cx.retained_progress();assert!(place.admit_sized_with_epoch(cx,destination,target.len(),&mut watermark)?);assert_eq!(cx.retained_progress(),before);assert_eq!(watermark,2);Ok(())}).unwrap();
 caller.turn(q,|cx|{cx.bind_original_source_epoch(1)?;assert!(place.admit_sized_with_epoch(cx,destination,target.len(),&mut watermark).is_err());assert_eq!(watermark,2);Ok(())}).unwrap();for _ in 0..8{if place.terminal_is_empty(){break}caller.turn(q,|cx|{cx.bind_original_source_epoch(3)?;place.settle_return(cx)?;Ok(())}).unwrap()}assert!(place.terminal_is_empty());retire_source(caller,stock,source_extra);
}

#[test]
fn sized_return_core_wire_publisher_is_source_owned(){
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8830),Generation(8),stock,clock,10000,128);let mut source_extra=0;let mut ready=false;for _ in 0..32{caller.turn(q,|cx|{cx.bind_original_source_epoch(1)?;if !cx.admit_original_core_wire_delivery()?{return Ok(())}ready=cx.admit_original_escrow_publisher()?;Ok(())}).unwrap();source_extra+=caller.receipt().retained_capacity_bytes;if ready{break}}assert!(ready);caller.turn(q,|cx|{cx.bind_original_source_epoch(2)?;let before=cx.retained_progress();assert!(cx.admit_original_escrow_publisher()?);assert_eq!(cx.retained_progress(),before);Ok(())}).unwrap();retire_source(caller,stock,source_extra);
}

#[test]
fn sized_return_publisher_failed_post_is_fenced(){
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:8};let q=RetainedCloneGrant{maximum_items:1,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(8840),Generation(9),stock,clock,10000,128);let mut source_extra=0;let mut table=false;
 for _ in 0..32{caller.turn(q,|cx|{cx.bind_original_source_epoch(1)?;if !cx.admit_original_core_wire_delivery()?{return Ok(())}table=cx.admit_original_return_table()?;Ok(())}).unwrap();source_extra+=caller.receipt().retained_capacity_bytes;if table{break}}assert!(table);
 let failure=caller.turn(q,|cx|{cx.bind_original_source_epoch(2)?;let maximum=testkit::retained_grant_mut(cx).maximum_items;testkit::set_loan_receipt_refusal(true);let result=cx.admit_original_escrow_publisher();testkit::retained_grant_mut(cx).maximum_items=maximum;let failure=result.expect_err("injected publisher receipt refusal");assert_eq!(failure.retained_progress().copied_items,1);let before=cx.retained_progress();assert!(cx.admit_original_escrow_publisher().is_err());assert_eq!(cx.retained_progress(),before);Ok(failure)}).unwrap();source_extra+=caller.receipt().retained_capacity_bytes;assert!(failure.primary_value().is_some());assert!(failure.retained_progress().retained_capacity_bytes>0);retire_source(caller,stock,source_extra);
}

#[test]
fn sized_preissue_requires_paid_metadata_and_retains_capture(){
 let stock=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:10000000,maximum_capacity_bytes:10000000,maximum_release_bytes:10000000,maximum_depth:16};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(7817),Generation(2),stock,clock,100000,100000);
 let mut place=OriginalCallerReturnPlace::empty();let mut extra=0;let mut ready=false;let mut cell=0u64;let destination=&mut cell as *mut u64 as usize;let extent=std::mem::size_of::<u64>();
 for _ in 0..64{caller.turn(stock,|cx|{ready=place.admit_sized(cx,destination,extent)?;Ok(())}).unwrap();extra+=caller.receipt().retained_capacity_bytes;if ready{break}}assert!(ready);assert!(!place.capture_is_preissued());
 let(items,bytes,work)=OriginalCallerReturnPlace::preissue_demand();let low=RetainedCloneGrant{maximum_items:items-1,..stock};
 caller.turn(low,|cx|{let before=cx.retained_progress();assert!(!place.preissue_capture(cx,destination,extent)?);assert_eq!(cx.retained_progress(),before);assert!(!place.capture_is_preissued());Ok(())}).unwrap();
 caller.turn(stock,|cx|{let before=cx.retained_progress();let start=cx.original_work_units_spent();assert!(place.preissue_capture(cx,destination,extent)?);let after=cx.retained_progress();assert_eq!(after.copied_items-before.copied_items,items);assert_eq!(after.copied_bytes-before.copied_bytes,bytes);assert_eq!(cx.original_work_units_spent()-start,u128::from(work));assert!(place.capture_is_preissued());Ok(())}).unwrap();
 cell=7;assert_eq!(cell,7);for _ in 0..64{if place.terminal_is_empty(){break}caller.turn(stock,|cx|{place.settle_return(cx)?;Ok(())}).unwrap()}assert!(place.terminal_is_empty());retire_source(caller,stock,extra);
}

#[test]
fn sized_preissue_cancel_returns_unused_capture_without_destination_copy(){
 let stock=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:10000000,maximum_capacity_bytes:10000000,maximum_release_bytes:10000000,maximum_depth:16};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(7818),Generation(2),stock,clock,100000,100000);let mut place=OriginalCallerReturnPlace::empty();let mut extra=0;let mut ready=false;let mut cell=0u64;let destination=&mut cell as *mut u64 as usize;let extent=std::mem::size_of::<u64>();
 for _ in 0..64{caller.turn(stock,|cx|{ready=place.admit_sized(cx,destination,extent)?;Ok(())}).unwrap();extra+=caller.receipt().retained_capacity_bytes;if ready{break}}assert!(ready);
 caller.turn(stock,|cx|{assert!(place.preissue_capture(cx,destination,extent)?);Ok(())}).unwrap();let(items,bytes,work)=OriginalCallerReturnPlace::cancel_preissue_demand();let low=RetainedCloneGrant{maximum_items:items-1,..stock};
 caller.turn(low,|cx|{assert!(!place.cancel_preissued_capture(cx,destination,extent)?);assert!(place.capture_is_preissued());Ok(())}).unwrap();
 caller.turn(stock,|cx|{let before=cx.retained_progress();let start=cx.original_work_units_spent();assert!(place.cancel_preissued_capture(cx,destination,extent)?);let after=cx.retained_progress();assert_eq!(after.copied_items-before.copied_items,items);assert_eq!(after.copied_bytes-before.copied_bytes,bytes);assert_eq!(cx.original_work_units_spent()-start,u128::from(work));assert!(!place.capture_is_preissued());Ok(())}).unwrap();
 caller.turn(stock,|cx|{let before=cx.reserved_return_progress();assert!(place.settle_return(cx)?);let after=cx.reserved_return_progress();assert_eq!(after.copied_bytes-before.copied_bytes,semio_framework_job_payload::OriginalReturnEscrowAccounting::witness_copy(false,true,0).unwrap());Ok(())}).unwrap();assert_eq!(cell,0);assert!(place.terminal_is_empty());retire_source(caller,stock,extra);
}

#[test]
fn core_wire_sized_claim_metadata_stays_within_its_portable_ceiling_and_the_quote_equals_it(){
 let actual=OriginalCallerReturnPlace::core_wire_sized_claim_actual_bytes().unwrap();assert!(actual<=OriginalSizedClaimReserve::CEILING,"sized claim metadata grew to {actual} B past the {} B portable ceiling on this target",OriginalSizedClaimReserve::CEILING);
 assert_eq!(OriginalCallerReturnPlace::core_wire_sized_claim_reserve(0).unwrap().immediate_copy_bytes,OriginalSizedClaimReserve::CEILING);
}
#[test]
fn core_wire_sized_claim_reserve_checks_actual_target_and_overflow(){
 let reserve=OriginalCallerReturnPlace::core_wire_sized_claim_reserve(8).expect("actual target fits the portable CoreWire ceiling");assert_eq!((reserve.items,reserve.immediate_copy_bytes,reserve.lifetime_copy_bytes,reserve.work,reserve.depth),(3,OriginalSizedClaimReserve::CEILING,OriginalSizedClaimReserve::CEILING+8,3,1));assert!(OriginalCallerReturnPlace::core_wire_sized_claim_reserve(usize::MAX).is_err());
}
