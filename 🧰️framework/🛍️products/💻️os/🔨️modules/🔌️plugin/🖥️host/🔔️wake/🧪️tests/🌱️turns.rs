use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller,close_original_effect_step};
use semio_framework_job_payload::JobPayloadAuthority;
use semio_framework_value::RetainedWorkPoll;

fn close_payload_authority(authority:&mut JobPayloadAuthority,operation:OperationId,generation:Generation,grant:RetainedCloneGrant)->RetainedCloneProgress{
 let stock=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 let mut caller=OriginalJobFixtureCaller::admit(operation,generation,stock,||Some(1),1000,4096);
 let demand=authority.retirement_demands();
 let fuel_before=caller.fuel_remaining();
 let work=authority.retirement_work_demand().unwrap();
 let(step,heap)=observe(||caller.turn(grant,|cx|close_original_effect_step(cx,demand,|effect|RetainedWorkPoll::Complete(authority.close_step(effect)))).unwrap());
 let progress=step.progress();
 assert!(authority.terminal_is_empty());
 assert_eq!(progress,caller.receipt());
 assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);
 assert_eq!(caller.fuel_remaining(),fuel_before-work);
 caller.retire(stock);
 progress
}

#[test]
fn original_host_context_loan_preserves_same_recipient_and_returns_cumulative_wake_once(){
 use semio_framework_job::{StepContext,StepBudget};
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let p=&law["policy"];let grant=RetainedCloneGrant{maximum_items:p["items"].as_u64().unwrap()as usize,maximum_copy_bytes:p["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:p["release"].as_u64().unwrap()as usize,maximum_depth:p["depth"].as_u64().unwrap()as usize};let operation=OperationId(99205);let generation=Generation(3);let cancel=semio_framework_job::root_cancel_token();let(_,warm)=observe(||drop(vec![0u8;37]));assert_eq!(warm,(37,37));let((mut authority,birth),heap)=observe(||{let mut actual=None;JobPayloadAuthority::admit(operation,generation,grant,&mut actual).result.unwrap().unwrap()});assert_eq!((birth.retained_capacity_bytes,birth.released_bytes),heap);let ledger=GuestRelayWakeReceiptLedger::new();let ledger_pointer=&ledger as*const GuestRelayWakeReceiptLedger;let mut sequence=0;let source=[7u8;41];let mut destination=[0u8;41];let source_pointer=source.as_ptr();let destination_pointer=destination.as_ptr();
 for epoch in [70,71,72]{let mut recipient=RetainedCloneProgress{copied_bytes:23,..Default::default()};let recipient_pointer=&mut recipient as*mut RetainedCloneProgress;let mut context=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,grant),&cancel,semio_framework_job::logical_now_us,&mut sequence,&mut recipient,&authority).unwrap();let original=context.retained_grant();let(mut loan,heap)=observe(||GuestRelayWakeContextTurn::admit(&ledger,epoch,&mut context).unwrap().unwrap());assert_eq!(heap,(0,0));let quoted=ledger.checkout(7,generation.0).unwrap().unwrap();assert_eq!(quoted.grant,original);assert!(ledger.checkout(8,generation.0).unwrap().is_none());let(done,heap)=observe(||loan.acknowledge().unwrap());assert!(!done);assert_eq!(heap,(0,0));assert_eq!(&ledger as*const _,ledger_pointer);
 let(_,heap)=observe(||destination.copy_from_slice(&source));assert_eq!(heap,(0,0));assert_eq!(destination,source);assert_eq!(source.as_ptr(),source_pointer);assert_eq!(destination.as_ptr(),destination_pointer);let progress=RetainedCloneProgress{copied_items:1,copied_bytes:41,..Default::default()};let rejected=GuestRelayWakeReceipt{epoch,slot:7,generation:generation.0,grant:original,progress:RetainedCloneProgress{copied_bytes:original.maximum_copy_bytes+1,..progress}};let(result,heap)=observe(||ledger.collect(&rejected));assert!(result.is_err());assert_eq!(heap,(0,0));let receipt=GuestRelayWakeReceipt{epoch,slot:7,generation:generation.0,grant:original,progress};assert!(ledger.collect(&receipt).unwrap());let(done,heap)=observe(||loan.acknowledge().unwrap());assert!(done);assert_eq!(heap,(0,0));let(done,heap)=observe(||loan.acknowledge().unwrap());assert!(done);assert_eq!(heap,(0,0));assert!(loan.terminal_is_empty());let(_,heap)=observe(||drop(loan));assert_eq!(heap,(0,0));assert_eq!(context.retained_progress(),RetainedCloneProgress{copied_items:1,copied_bytes:64,..Default::default()});assert_eq!(context.retained_grant().maximum_items,0);assert_eq!(context.retained_grant().maximum_copy_bytes,grant.maximum_copy_bytes-64);drop(context);assert_eq!(&mut recipient as*mut _,recipient_pointer);assert_eq!(recipient.copied_bytes,64);assert!(ledger.begin_turn(epoch,grant).is_err());}
  let release=close_payload_authority(&mut authority,operation,generation,grant);assert_eq!(birth.retained_capacity_bytes,release.released_bytes);let(_,heap)=observe(||{drop(authority);drop(ledger)});assert_eq!(heap,(0,0));
}
#[test]
fn original_host_driver_treasury_never_renews_and_rejects_receipts_outside_the_independent_admission_policy(){
 let policy:GuestRelayDriverPolicy=semio_framework_pack_json::from_json_str(include_str!("../📃️policy/🧪️tests/🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut original=GuestRelayDriverTreasury::new(policy);let original_pointer=&original as*const _;let initial=original.original_remaining();
 let denied=RetainedCloneProgress{copied_items:1,copied_bytes:policy.admission.maximum_copy_bytes+1,..Default::default()};let(result,heap)=observe(||original.record_admission(denied));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(original.original_remaining(),initial);assert_eq!(&original as*const _,original_pointer);
 let op=semio_framework_job::OperationId(71);let generation=semio_framework_job::Generation(3);let(admitted,heap)=observe(||{let mut actual=None;semio_framework_job_payload::JobPayloadAuthority::admit(op,generation,policy.admission,&mut actual).result.unwrap()});let(mut authority,birth)=admitted.unwrap();assert_eq!((birth.retained_capacity_bytes,birth.released_bytes),heap);original.record_admission(birth).unwrap();
 let source=[83u8;41];let mut destination=[0u8;41];let source_pointer=source.as_ptr();let destination_pointer=destination.as_ptr();let(turn,heap)=observe(||original.begin_turn().unwrap().unwrap());assert_eq!(heap,(0,0));let held_remaining=original.original_remaining();assert!(original.begin_turn().unwrap().is_none());let mismatched=GuestRelayDriverTurnGrant{epoch:turn.epoch+1,..turn};let(result,heap)=observe(||original.return_turn(mismatched,Default::default()));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(original.original_remaining(),held_remaining);
 let(_,heap)=observe(||destination.copy_from_slice(&source));assert_eq!(heap,(0,0));let progress=RetainedCloneProgress{copied_items:1,copied_bytes:41,..Default::default()};original.return_turn(turn,progress).unwrap();assert_eq!(source.as_ptr(),source_pointer);assert_eq!(destination.as_ptr(),destination_pointer);assert_eq!(source,destination);let after=original.original_remaining();assert_eq!(after.maximum_items,initial.maximum_items-birth.copied_items-1);assert_eq!(after.maximum_copy_bytes,initial.maximum_copy_bytes-birth.copied_bytes-41);assert_eq!(after.maximum_capacity_bytes,initial.maximum_capacity_bytes-birth.retained_capacity_bytes);assert_eq!(after.maximum_release_bytes,initial.maximum_release_bytes);
  let(result,heap)=observe(||original.return_turn(turn,progress));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(original.original_remaining(),after);assert!(original.terminal_is_empty());
  let close_turn=original.begin_drive_turn().unwrap().unwrap();
  let release=close_payload_authority(&mut authority,op,generation,close_turn.grant);
  original.return_turn(close_turn,release).unwrap();
  let remaining=original.original_remaining();
  assert_eq!(remaining.maximum_items,after.maximum_items-release.copied_items);
  assert_eq!(remaining.maximum_copy_bytes,after.maximum_copy_bytes-release.copied_bytes);
  assert_eq!(remaining.maximum_capacity_bytes,after.maximum_capacity_bytes-release.retained_capacity_bytes);
  assert_eq!(remaining.maximum_release_bytes,after.maximum_release_bytes-release.released_bytes);
  assert_eq!(birth.retained_capacity_bytes,release.released_bytes);let(_,heap)=observe(||{drop(authority);drop(original)});assert_eq!(heap,(0,0));
}

#[test]
fn original_host_actor_and_wake_returns_share_one_finite_treasury_and_exact_checkout(){
 use semio_framework_actor::{RetainedTurnInput,RetainedTurnReceipt};
 let mut policy:GuestRelayDriverPolicy=semio_framework_pack_json::from_json_str(include_str!("../📃️policy/🧪️tests/🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let law:serde_json::Value=serde_json::from_str(include_str!("../📃️policy/⚛️actor/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 policy.drive.maximum_items=law["driveItems"].as_u64().unwrap()as usize;
 policy.drive.maximum_copy_bytes=law["driveCopyBytes"].as_u64().unwrap()as usize;
 policy.wake.maximum_copy_bytes=law["wakeCopyBytes"].as_u64().unwrap()as usize;
 let (birth,birth_work)=semio_framework_actor::retained_context::OriginalActorContextSlot::bootstrap_birth_reserve();
 policy.admission.maximum_copy_bytes=policy.admission.maximum_copy_bytes.max(birth.copy_bytes);
 policy.admission.maximum_capacity_bytes=policy.admission.maximum_capacity_bytes.max(birth.capacity_bytes);
 policy.root.turn_fuel=policy.root.turn_fuel.max(birth_work);
 let original=GuestRelayWakeDriver::new(policy,||Some(1000),Box::new(||false)).unwrap();
 assert_eq!(original.original_remaining().unwrap(),policy.operation);
 assert!(original.begin_actor_turn(None).unwrap().is_none());
 let mut admitted=false;
 for _ in 0..64{
  admitted=original.admit_root_step().unwrap();
  if admitted{break}
 }
 assert!(admitted);
 let balances=|grant:RetainedCloneGrant|[grant.maximum_items,grant.maximum_copy_bytes,grant.maximum_capacity_bytes,grant.maximum_release_bytes,grant.maximum_depth];
 let post_admission=balances(original.original_remaining().unwrap());
 let unsigned=|value:&serde_json::Value|usize::try_from(value.as_u64().unwrap()).unwrap();
 let mut epoch_offset=None;
 let source=[83u8;41];
 let mut destination=[0u8;41];
 let pointers=(source.as_ptr(),destination.as_ptr());
 for row in law["receipts"].as_array().unwrap(){
  let relative_epoch=row["epoch"].as_u64().unwrap();
  let [copied_items,copied_bytes,retained_capacity_bytes,released_bytes]=std::array::from_fn(|i|unsigned(&row["spent"][i]));
  let spent=RetainedCloneProgress{copied_items,copied_bytes,retained_capacity_bytes,released_bytes};
  let grant=std::array::from_fn(|i|unsigned(&row["grant"][i]));
  let before=original.original_remaining().unwrap();
  if row["phase"]=="actor"{
   let(input,heap)=observe(||original.begin_actor_turn(None).unwrap().unwrap());
   assert_eq!(heap,(0,0));
   let offset=*epoch_offset.get_or_insert_with(||input.epoch.checked_sub(relative_epoch).unwrap());
   assert!(offset>0);
   assert_eq!((input.operation,input.generation,input.epoch),(law["operation"].as_u64().unwrap(),law["generation"].as_u64().unwrap(),offset+relative_epoch));
   assert_eq!(balances(original.actor_budget(input).unwrap().retained_policy),grant);
   assert!(original.checkout(7,input.generation).unwrap().is_none());
   assert!(original.begin_actor_turn(None).unwrap().is_none());
   let receipt=input.return_original(spent,0).unwrap();
   for forged in [
    RetainedTurnReceipt{input:RetainedTurnInput{operation:input.operation+1,..input},..receipt},
    RetainedTurnReceipt{input:RetainedTurnInput{generation:input.generation+1,..input},..receipt},
    RetainedTurnReceipt{input:RetainedTurnInput{epoch:input.epoch+1,..input},..receipt},
    RetainedTurnReceipt{remaining:RetainedCloneGrant{maximum_depth:receipt.remaining.maximum_depth+1,..receipt.remaining},..receipt},
    RetainedTurnReceipt{spent:RetainedCloneProgress{copied_items:input.grant.maximum_items+1,..spent},..receipt},
   ]{
    let(result,heap)=observe(||original.return_actor_turn(forged));
    assert!(result.is_err());
    assert_eq!(heap,(0,0));
    assert_eq!(original.original_remaining().unwrap(),before);
   }
   assert_eq!(observe(||destination.copy_from_slice(&source)).1,(0,0));
   assert_eq!(observe(||original.return_actor_turn(receipt).unwrap()),(true,(0,0)));
   let(duplicate,heap)=observe(||original.return_actor_turn(receipt));
   assert!(duplicate.is_err());
   assert_eq!(heap,(0,0));
  }else{
   let generation=law["generation"].as_u64().unwrap();
   let(input,heap)=observe(||original.checkout(7,generation).unwrap().unwrap());
   assert_eq!(heap,(0,0));
   let epoch=epoch_offset.unwrap()+relative_epoch;
   assert_eq!(input.epoch,epoch);
   assert_eq!(balances(input.grant),grant);
   assert!(original.begin_actor_turn(None).unwrap().is_none());
   let receipt=GuestRelayWakeReceipt{epoch,slot:7,generation,grant:input.grant,progress:spent};
   assert_eq!(observe(||destination.copy_from_slice(&source)).1,(0,0));
   assert_eq!(observe(||original.collect(&receipt).unwrap()),(true,(0,0)));
  }
  assert_eq!((source.as_ptr(),destination.as_ptr()),pointers);
  assert_eq!(source,destination);
  assert_eq!(balances(original.original_remaining().unwrap()),[
   before.maximum_items-spent.copied_items,
   before.maximum_copy_bytes-spent.copied_bytes,
   before.maximum_capacity_bytes-spent.retained_capacity_bytes,
   before.maximum_release_bytes-spent.released_bytes,
   before.maximum_depth,
  ]);
 }
 let admission_delta=std::array::from_fn::<_,5,_>(|i|balances(policy.operation)[i]-post_admission[i]);
 let expected=std::array::from_fn(|i|unsigned(&law["remaining"][i])-admission_delta[i]);
 assert_eq!(balances(original.original_remaining().unwrap()),expected);
 for _ in 0..64{if original.close_root_step().unwrap(){break}}
 assert!(original.operation_terminal_is_empty().unwrap());
 assert_eq!(observe(||drop(original)).1,(0,0));
}
