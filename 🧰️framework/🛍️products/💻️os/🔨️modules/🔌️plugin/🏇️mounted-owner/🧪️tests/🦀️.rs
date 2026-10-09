//! 🧪️ The portable ownership corpus exercises the real job context and canonical physical currencies.

use super::*;
use semio_framework_job::{CancelToken, StepBudget};
use serde_json::Value;
use std::cell::Cell;

#[global_allocator]
static MOUNTED_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

thread_local! { static CLOCK: Cell<Option<u64>> = const { Cell::new(None) }; }
fn now_us() -> Option<u64> { CLOCK.with(Cell::get) }

trait MountedFixtureOracleV1 {
    fn word(&self, field: &str) -> Result<usize, ()>;
    fn decimal(&self, field: &str) -> Result<u64, ()>;
    fn grant(&self) -> Result<RetainedCloneGrant, ()>;
    fn identity(&self) -> Result<MountedOwnerIdentityV1, ()>;
}

impl MountedFixtureOracleV1 for Value {
    fn word(&self, field: &str) -> Result<usize, ()> { self[field].as_u64().filter(|value| *value <= u32::MAX as u64).map(|value| value as usize).ok_or(()) }
    fn decimal(&self, field: &str) -> Result<u64, ()> {
        let value = self[field].as_str().ok_or(())?;
        if value.is_empty() || value.len() > 20 || (value.len() > 1 && value.starts_with('0')) || !value.bytes().all(|byte| byte.is_ascii_digit()) { return Err(()); }
        value.parse().map_err(|_| ())
    }
    fn grant(&self) -> Result<RetainedCloneGrant, ()> {
        Ok(RetainedCloneGrant { maximum_items: self.word("maximumItems")?, maximum_copy_bytes: self.word("maximumCopyBytes")?, maximum_capacity_bytes: self.word("maximumCapacityBytes")?, maximum_release_bytes: self.word("maximumReleaseBytes")?, maximum_depth: self.word("maximumDepth")? })
    }
    fn identity(&self) -> Result<MountedOwnerIdentityV1, ()> {
        let identity = MountedOwnerIdentityV1 { instance_id: u32::try_from(self.word("instanceId")?).map_err(|_| ())?, operation: OperationId(self.decimal("operationId")?), generation: Generation(self.decimal("generation")?) };
        if identity.instance_id == 0 || identity.operation.0 == 0 { return Err(()); }
        Ok(identity)
    }
}

fn verdict(observation: &Value) -> Result<&'static str, ()> {
    let identity = observation["identity"].identity()?;
    let expected = observation["expectedIdentity"].identity()?;
    let grant = observation["grant"].grant()?;
    let schedule = &observation["schedule"];
    let now = schedule.decimal("nowUs")?;
    let deadline = schedule.decimal("deadlineUs")?;
    let fuel = schedule.word("fuel")? as u64;
    let clock_available = schedule["clockAvailable"].as_bool().ok_or(())?;
    CLOCK.with(|clock| clock.set(clock_available.then_some(now)));
    let cancel = CancelToken::root_now();
    if schedule["cancelled"].as_bool().ok_or(())? { cancel.cancel_now(); }
    let mut sequence = 0;
    let mut retained_progress = RetainedCloneProgress::default();
    let mut context = StepContext::new(identity.operation, identity.generation, StepBudget::new(fuel, deadline, grant), cancel, now_us, &mut sequence, &mut retained_progress);
    let phase = match observation["phase"].as_str().ok_or(())? { "preparation" => MountedOwnerPhaseV1::Preparation, "maintenance" => MountedOwnerPhaseV1::Maintenance, "close" => MountedOwnerPhaseV1::Close, _ => return Err(()) };
    let policy = MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant };
    let mut turn = match MountedOwnerTurnV1::admit(identity.instance_id, expected, phase, policy, &mut context) {
        Ok(Some(turn)) => turn,
        Ok(None) => return Ok("held"),
        Err(error) => return Ok(if error.kind == ValueRefusalKind::Canceled { "cancelled" } else { "refused" }),
    };
    let demand = &observation["demand"];
    let demand = RetirementDemand { copy_bytes: demand.word("copyBytes")?, capacity_bytes: demand.word("capacityBytes")?, release_bytes: demand.word("releaseBytes")?, depth: demand.word("depth")? };
    if !turn.admits(demand).map_err(|_| ())? { return Ok("held"); }
    let receipt = &observation["receipt"];
    let receipt = RetainedCloneProgress { copied_items: receipt.word("copiedItems")?, copied_bytes: receipt.word("copiedBytes")?, retained_capacity_bytes: receipt.word("retainedCapacityBytes")?, released_bytes: receipt.word("releasedBytes")? };
    let result = observation["result"].as_str().ok_or(())?;
    let complete = result == "complete";
    if turn.record(receipt, complete, observation["terminalIsEmpty"].as_bool().ok_or(())?).is_err() { return Ok("refused"); }
    Ok(match result {
        "complete" => "complete",
        "blocked" | "awaiting-input" if observation["wakeOwned"].as_bool().ok_or(())? && receipt == RetainedCloneProgress::default() => if result == "blocked" { "blocked" } else { "awaiting-input" },
        "blocked" | "awaiting-input" | "refused" => "refused",
        "progress" => "progress",
        _ => return Err(()),
    })
}

#[test]
fn complete_portable_mounted_turn_corpus() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("authored portable mounted fixture");
    let vectors = fixture["vectors"].as_array().expect("complete vectors");
    assert_eq!(vectors.len(), 28);
    let mut results = Vec::with_capacity(vectors.len());
    for vector in vectors {
        let actual = verdict(&vector["observation"]).expect("valid portable observation");
        assert_eq!(actual, vector["expected"].as_str().expect("expected verdict"), "portable mounted vector {}", vector["id"]);
        results.push(serde_json::json!({ "id": vector["id"], "actual": actual }));
    }
    for vector in fixture["invalid"].as_array().expect("malformed vectors") { assert!(verdict(&vector["observation"]).is_err(), "malformed mounted vector {}", vector["id"]); }
    let path = std::env::var_os("SEMIO_MOUNTED_OWNER_RESULTS").expect("explicit mounted result path");
    let path = std::path::PathBuf::from(path);
    assert!(path.is_absolute(), "result authority must be absolute");
    assert!(path.to_string_lossy().encode_utf16().count() <= 256, "result path must fit Windows");
    std::fs::create_dir_all(path.parent().expect("result parent")).expect("owned result directory");
    std::fs::write(path, serde_json::to_vec(&results).expect("bounded native verdicts")).expect("owned native result output");
    println!("[DEBUG] Actual mounted turn: all27 original observations plus actual initial store generation0 and eight malformed identities/grants/clocks executed against original StepContext");
}

#[test]
fn cumulative_receipts_share_original_context_fuel_and_physical_authority() {
    CLOCK.with(|clock| clock.set(Some(10)));
    let grant = RetainedCloneGrant { maximum_items: 2, maximum_copy_bytes: 8, maximum_capacity_bytes: 16, maximum_release_bytes: 32, maximum_depth: 3 };
    let policy = MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant };
    let identity = MountedOwnerIdentityV1 { instance_id: 7, operation: OperationId(1u64 << 62), generation: Generation(u64::MAX) };
    let mut sequence = 0;
    let mut retained_progress = RetainedCloneProgress::default();
    let mut context = StepContext::new(identity.operation, identity.generation, StepBudget::new(2, 100, grant), CancelToken::root_now(), now_us, &mut sequence, &mut retained_progress);
    let mut turn = MountedOwnerTurnV1::admit(7, identity, MountedOwnerPhaseV1::Preparation, policy, &mut context).unwrap().unwrap();
    turn.record(RetainedCloneProgress { copied_items: 1, copied_bytes: 4, retained_capacity_bytes: 8, released_bytes: 16 }, false, false).unwrap();
    assert_eq!(turn.grant(), RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4, maximum_capacity_bytes: 8, maximum_release_bytes: 16, maximum_depth: 3 });
    assert_eq!(turn.context().fuel_remaining(), 1);
    turn.record(RetainedCloneProgress { copied_items: 1, copied_bytes: 4, retained_capacity_bytes: 8, released_bytes: 16 }, true, true).unwrap();
    assert_eq!(turn.context().fuel_remaining(), 0);
    assert!(!turn.admits(RetirementDemand::default()).unwrap());
}

#[test]
fn explicit_phase_policy_corpus_preserves_legitimate_empty_authority() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let policies = fixture["policies"].as_array().unwrap();
    assert_eq!(policies.len(), 4);
    for vector in policies {
        let value = &vector["value"];
        let admitted = (|| -> Result<MountedOwnerPolicyV1, ()> {
            let fields = value.as_object().ok_or(())?;
            if fields.len() != 3 || !["preparation", "maintenance", "close"].into_iter().all(|field| fields.contains_key(field)) { return Err(()); }
            let policy = MountedOwnerPolicyV1 { preparation: value["preparation"].grant()?, maintenance: value["maintenance"].grant()?, close: value["close"].grant()? };
            policy.validate().map_err(|_| ())
        })();
        assert_eq!(admitted.is_ok(), vector["valid"].as_bool().unwrap(), "portable policy {}", vector["id"]);
    }
    println!("[DEBUG] Allfour mandatory phase policy vectors executed without default authority");
}

#[test]
fn nested_job_receipts_preserve_original_scheduling_debit() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let vectors = fixture["nestedJobs"].as_array().unwrap();
    assert_eq!(vectors.len(), 8);
    let mut results = Vec::with_capacity(vectors.len());
    for vector in vectors {
        let value = &vector["observation"];
        let identity = value["identity"].identity().unwrap();
        let grant = value["grant"].grant().unwrap();
        let policy = MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant };
        CLOCK.with(|clock| clock.set(Some(value["schedule"].decimal("nowUs").unwrap())));
        let mut sequence = 0;
        let mut retained_progress = RetainedCloneProgress::default();
        let mut context = StepContext::new(identity.operation, identity.generation, StepBudget::new(value["schedule"].word("fuel").unwrap() as u64, value["schedule"].decimal("deadlineUs").unwrap(), grant), CancelToken::root_now(), now_us, &mut sequence, &mut retained_progress);
        let mut turn = MountedOwnerTurnV1::admit(identity.instance_id, identity, MountedOwnerPhaseV1::Preparation, policy, &mut context).unwrap().unwrap();
        let receipt = &value["receipt"];
        let receipt = RetainedCloneProgress { copied_items: receipt.word("copiedItems").unwrap(), copied_bytes: receipt.word("copiedBytes").unwrap(), retained_capacity_bytes: receipt.word("retainedCapacityBytes").unwrap(), released_bytes: receipt.word("releasedBytes").unwrap() };
        let result = value["result"].as_str().unwrap();
        let actual = if turn.advance_job(|context, _grant| {
            context.consume_fuel(value.word("usedFuel").unwrap() as u64);
            context.consume_retained(receipt)?;
            Ok(MountedOwnerJobReceiptV1 { progress: receipt, complete: result == "complete", terminal_is_empty: value["terminalIsEmpty"].as_bool().unwrap() })
        }).is_ok() { result } else { "refused" };
        assert_eq!(actual, vector["expected"].as_str().unwrap(), "nested original scheduling vector {}", vector["id"]);
        assert_eq!(turn.context().fuel_remaining(), vector.word("expectedFuel").unwrap() as u64);
        results.push(serde_json::json!({ "id": vector["id"], "actual": actual, "remainingFuel": turn.context().fuel_remaining() }));
    }
    let path = std::path::PathBuf::from(std::env::var_os("SEMIO_MOUNTED_OWNER_NESTED_RESULTS").expect("explicit nested mounted result path"));
    assert!(path.is_absolute() && path.to_string_lossy().encode_utf16().count() <= 256, "nested result authority must be absolute and bounded");
    std::fs::create_dir_all(path.parent().unwrap()).expect("owned nested result directory");
    std::fs::write(path, serde_json::to_vec(&results).unwrap()).expect("owned nested result output");
    println!("[DEBUG] Eight original job scheduling receipts retained every physical currency without double debit");
}

fn context_receipt(value: &Value) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: value.word("copiedItems").unwrap(), copied_bytes: value.word("copiedBytes").unwrap(), retained_capacity_bytes: value.word("retainedCapacityBytes").unwrap(), released_bytes: value.word("releasedBytes").unwrap() }
}

#[test]
fn mounted_original_context_receipts_preserve_every_independent_axis() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🧾️context/🔣️.json")).unwrap();
    let identity = MountedOwnerIdentityV1 { instance_id: 7, operation: OperationId(1u64 << 62), generation: Generation(9) };
    CLOCK.with(|clock| clock.set(Some(10)));
    let mut results = Vec::new();
    for vector in fixture["vectors"].as_array().unwrap() {
        let policy_grant = vector["policy"].grant().unwrap();
        let caller_grant = vector["context"].grant().unwrap();
        let receipt = context_receipt(&vector["receipt"]);
        let already = context_receipt(&vector["already"]);
        let mut recipient = RetainedCloneProgress::default();
        let mut sequence = 0;
        let mut context = StepContext::new(identity.operation, identity.generation, StepBudget::new(4, 100, caller_grant), CancelToken::root_now(), now_us, &mut sequence, &mut recipient);
        context.consume_retained(already).unwrap();
        let policy = MountedOwnerPolicyV1 { preparation: policy_grant, maintenance: policy_grant, close: policy_grant };
        let remaining = if let Some(mut turn) = MountedOwnerTurnV1::admit(7, identity, MountedOwnerPhaseV1::Preparation, policy, &mut context).unwrap() {
            turn.record(receipt, false, false).unwrap();
            assert_eq!(turn.context().retained_progress(), already.checked_add(receipt).unwrap(), "original recipient {}", vector["id"]);
            turn.grant()
        } else {
            assert_eq!(policy_grant.maximum_items.min(caller_grant.maximum_items), 0);
            assert_eq!(receipt, RetainedCloneProgress::default());
            RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: policy_grant.maximum_copy_bytes.min(caller_grant.maximum_copy_bytes), maximum_capacity_bytes: policy_grant.maximum_capacity_bytes.min(caller_grant.maximum_capacity_bytes), maximum_release_bytes: policy_grant.maximum_release_bytes.min(caller_grant.maximum_release_bytes), maximum_depth: policy_grant.maximum_depth.min(caller_grant.maximum_depth) }
        };
        assert_eq!(remaining, vector["remaining"].grant().unwrap(), "original remaining authority {}", vector["id"]);
        results.push(serde_json::json!({"id":vector["id"],"remaining":{"maximumItems":remaining.maximum_items,"maximumCopyBytes":remaining.maximum_copy_bytes,"maximumCapacityBytes":remaining.maximum_capacity_bytes,"maximumReleaseBytes":remaining.maximum_release_bytes,"maximumDepth":remaining.maximum_depth}}));
        println!("[DEBUG] mounted original context {} remaining={remaining:?} actual={:?}", vector["id"], context.retained_progress());
    }
    for vector in fixture["nestedJobs"].as_array().unwrap() {
        let grant = fixture["vectors"][0]["context"].grant().unwrap();
        let receipt = context_receipt(&vector["receipt"]);
        let mut recipient = RetainedCloneProgress::default();
        let mut sequence = 0;
        let mut context = StepContext::new(identity.operation, identity.generation, StepBudget::new(4, 100, grant), CancelToken::root_now(), now_us, &mut sequence, &mut recipient);
        let policy = MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant };
        let mut turn = MountedOwnerTurnV1::admit(7, identity, MountedOwnerPhaseV1::Preparation, policy, &mut context).unwrap().unwrap();
        let accepted = turn.advance_job(|context, original| {
            assert_eq!(original, grant);
            context.consume_fuel(1);
            match vector["debit"].as_str().unwrap() {
                "child" => context.consume_retained(receipt)?,
                "partial" => context.consume_retained(RetainedCloneProgress { copied_items: receipt.copied_items, ..Default::default() })?,
                "foreign" => context.consume_retained(RetainedCloneProgress { copied_items: receipt.copied_items, copied_bytes: 1, ..Default::default() })?,
                "owner" => {},
                _ => unreachable!(),
            }
            Ok(MountedOwnerJobReceiptV1 { progress: receipt, complete: false, terminal_is_empty: false })
        }).is_ok();
        assert_eq!(accepted, vector["debit"] == "child", "exact original nested receipt {}", vector["id"]);
        if accepted { assert_eq!(turn.context().retained_progress(), receipt); assert_eq!(turn.progress(), receipt); }
        results.push(serde_json::json!({"id":vector["id"],"accepted":accepted}));
        println!("[DEBUG] mounted original nested {} accepted={accepted} actual={:?}", vector["id"], turn.context().retained_progress());
    }
    let path = std::path::PathBuf::from(std::env::var_os("SEMIO_MOUNTED_OWNER_CONTEXT_RESULTS").expect("explicit original context result path"));
    assert!(path.is_absolute() && path.to_string_lossy().encode_utf16().count() <= 256);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, serde_json::to_vec(&results).unwrap()).unwrap();
}

#[test]
fn original_mounted_turn_debits_actual_context_receipt_once_and_keeps_fuel_independent(){
    let fixture:Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["contextReceipt"];let grant=law["grant"].grant().unwrap();CLOCK.with(|clock|clock.set(Some(10)));let identity=MountedOwnerIdentityV1{instance_id:7,operation:OperationId(99117),generation:Generation(13)};let policy=MountedOwnerPolicyV1{preparation:grant,maintenance:grant,close:grant};let mut sequence=0;let mut actual=RetainedCloneProgress::default();let mut context=StepContext::new(identity.operation,identity.generation,StepBudget::new(law.word("fuel").unwrap() as u64,100,grant),CancelToken::root_now(),now_us,&mut sequence,&mut actual);let mut turn=MountedOwnerTurnV1::admit(7,identity,MountedOwnerPhaseV1::Preparation,policy,&mut context).unwrap().unwrap();let mut source=Vec::<u8>::new();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.reserve_exact(64));assert_eq!(source.capacity(),64);assert_eq!((heap.requested_bytes,heap.released_bytes),(64,0));let pointer=source.as_ptr();let birth=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:64,..Default::default()};turn.record(birth,false,false).unwrap();assert_eq!(turn.context().retained_progress(),birth);
    let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:law["source"].as_str().unwrap().len(),..Default::default()};turn.advance_job(|context,incoming|{assert_eq!(incoming,context.retained_grant());source.extend_from_slice(law["source"].as_str().unwrap().as_bytes());context.consume_fuel(law.word("childFuel").unwrap() as u64);context.consume_retained(receipt)?;Ok(MountedOwnerJobReceiptV1{progress:receipt,complete:false,terminal_is_empty:false})}).unwrap();assert_eq!(source.as_ptr(),pointer);assert_eq!(turn.context().retained_progress(),birth.checked_add(receipt).unwrap());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(source));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,64));let release=RetainedCloneProgress{copied_items:1,released_bytes:64,..Default::default()};turn.record(release,true,true).unwrap();assert_eq!(turn.grant(),law["remaining"].grant().unwrap());assert_eq!(turn.context().retained_grant(),turn.grant());assert_eq!(turn.context().fuel_remaining(),law.word("remainingFuel").unwrap() as u64);assert_eq!(turn.progress(),turn.context().retained_progress());eprintln!("[DEBUG] original mounted same context born64/copy23/free64 accepted once, original source pointer unchanged, child fuel5 independent from one physical item");
}

#[test]
fn original_actor_capture_uses_full_grant_and_retains_cancellation_custody(){
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let fixture:Value=serde_json::from_str(include_str!("../🧫️fixtures/🎭️actor/🔣️.json")).unwrap();
 let amounts=fixture["nativeGrant"].as_array().unwrap();let amount=|n:usize|usize::try_from(amounts[n].as_u64().unwrap()).unwrap();
 let grant=RetainedCloneGrant{maximum_items:amount(0),maximum_copy_bytes:amount(1),maximum_capacity_bytes:amount(2),maximum_release_bytes:amount(3),maximum_depth:amount(4)};
 let mut observations=Vec::new();
 for vector in fixture["cases"].as_array().unwrap(){
  let source=if let Some(text)=vector["text"].as_str(){text.to_owned()}else{vector["repeat"].as_str().unwrap().repeat(vector["bytes"].as_u64().unwrap()as usize)};
  let original_pointer=source.as_ptr();let mut slot:Option<ControlledRetirement<semio_framework_value::SharedUtf8>>=None;
  let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||actor_capture::actor_capture_demand(&source).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||actor_capture::admit_actor_capture(&mut slot,&source,denied).unwrap());assert_eq!(result,None);assert!(slot.is_none());assert_eq!(source.as_ptr(),original_pointer);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||actor_capture::admit_actor_capture(&mut slot,&source,grant).unwrap());let receipt=result.unwrap();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(slot.as_ref().unwrap().original().unwrap().as_str(),source);assert_eq!(source.as_ptr(),original_pointer);
  let capture_pointer=slot.as_ref().unwrap().original().unwrap().as_ptr();let(mut births,mut releases,mut turns)=(heap.requested_bytes,heap.released_bytes,1);
  let(again,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||actor_capture::admit_actor_capture(&mut slot,&source,grant).unwrap());assert_eq!(again,None);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(slot.as_ref().unwrap().original().unwrap().as_ptr(),capture_pointer);
  while !slot.as_ref().unwrap().terminal_is_empty(){
   assert!(turns<1000);let owner=slot.as_mut().unwrap();
   let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{owner.next_copy_byte_demand().unwrap();owner.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap();owner.next_release_byte_demand().unwrap();owner.next_depth_demand().unwrap();});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());let actual=step.progress();assert!(actual.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(actual.retained_capacity_bytes,actual.released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;
  }
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(slot.take()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(births,releases);
  observations.push(serde_json::json!({"id":vector["id"],"textBytes":source.len(),"copyBytes":receipt.copied_bytes,"capacityBytes":receipt.retained_capacity_bytes,"releaseBytes":receipt.released_bytes,"turns":turns,"births":births,"releases":releases,"deniedHeapBytes":0,"dropHeapBytes":0}));
  eprintln!("[DEBUG] mounted original actor {} source={} capture={} turns={} birth={} release={} undergrantHeap0 terminalDrop0",vector["id"],original_pointer as usize,capture_pointer as usize,turns,births,releases);
 }
 let parent=std::path::PathBuf::from(std::env::var_os("SEMIO_MOUNTED_OWNER_RESULTS").expect("explicit original mounted output authority"));let path=parent.parent().unwrap().join("actor.json");assert!(path.is_absolute()&&path.to_string_lossy().encode_utf16().count()<=256);std::fs::create_dir_all(path.parent().unwrap()).unwrap();std::fs::write(path,serde_json::to_vec(&observations).unwrap()).unwrap();
}

#[test]
fn original_mounted_failed_producer_retains_actual_external_and_turn_receipt() {
    CLOCK.with(|clock|clock.set(Some(10)));
    for false_complete in [false,true] {
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:if false_complete{64}else{0},maximum_release_bytes:0,maximum_depth:1};
        let policy=MountedOwnerPolicyV1{preparation:grant,maintenance:grant,close:grant};let identity=MountedOwnerIdentityV1{instance_id:7,operation:OperationId(98145),generation:Generation(17)};
        let mut sequence=0;let mut external=RetainedCloneProgress::default();
        let mut context=StepContext::new(identity.operation,identity.generation,StepBudget::new(7,100,grant),CancelToken::root_now(),now_us,&mut sequence,&mut external);
        let mut turn=MountedOwnerTurnV1::admit(7,identity,MountedOwnerPhaseV1::Preparation,policy,&mut context).unwrap().unwrap();
        let(source,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Vec::<u8>::with_capacity(64));let pointer=source.as_ptr();let actual=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth.requested_bytes,..Default::default()};assert_eq!(actual.retained_capacity_bytes,64);
        let(error,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||turn.record(actual,false_complete,false).unwrap_err());
        assert_eq!(error.retained_progress(),actual);assert_eq!(turn.progress(),actual);assert_eq!(turn.context().retained_progress(),actual);assert_eq!(turn.context().fuel_remaining(),6);assert_eq!(turn.grant().maximum_capacity_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ptr(),pointer);
        drop(turn);drop(context);assert_eq!(external,actual);let(_,closed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(source));assert_eq!(closed.released_bytes,actual.retained_capacity_bytes);
    }
    println!("[DEBUG] Original mounted failure and false Complete retain actual64 effects in both original recipients; refusal0/0, same source pointer, fuel6, physical final release64");
}

#[test]
fn original_mounted_nested_failure_receives_actual_context_delta_once() {
 CLOCK.with(|clock|clock.set(Some(10)));
 for false_complete in [false,true]{
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:64,maximum_release_bytes:0,maximum_depth:1};let policy=MountedOwnerPolicyV1{preparation:grant,maintenance:grant,close:grant};let identity=MountedOwnerIdentityV1{instance_id:7,operation:OperationId(98146),generation:Generation(17)};
  let mut sequence=0;let mut external=RetainedCloneProgress::default();let mut context=StepContext::new(identity.operation,identity.generation,StepBudget::new(7,100,grant),CancelToken::root_now(),now_us,&mut sequence,&mut external);let mut turn=MountedOwnerTurnV1::admit(7,identity,MountedOwnerPhaseV1::Preparation,policy,&mut context).unwrap().unwrap();
  let(source,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Vec::<u8>::with_capacity(64));let pointer=source.as_ptr();let actual=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth.requested_bytes,..Default::default()};assert_eq!(actual.retained_capacity_bytes,64);
  let(error,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||turn.advance_job(|cx,_|{cx.consume_fuel(1);cx.consume_retained(actual)?;if false_complete{Ok(MountedOwnerJobReceiptV1{progress:actual,complete:true,terminal_is_empty:false})}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original failed child").with_retained_progress(actual))}}).unwrap_err());
  assert_eq!(error.retained_progress(),actual);assert_eq!(turn.progress(),actual);assert_eq!(turn.context().retained_progress(),actual);assert_eq!(turn.context().fuel_remaining(),6);assert_eq!(turn.grant().maximum_capacity_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ptr(),pointer);
  drop(turn);drop(context);assert_eq!(external,actual);let(_,closed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(source));assert_eq!(closed.released_bytes,64);
 }
 println!("[DEBUG] Actual nested failure and false Complete preserved original64 context debit in mounted recipient once; same source pointer, fuel6, denial0/0, release64");
}

#[test]
fn original_mounted_alias_custody_measures_each_final_backing_payload_and_shell() {
 use super::alias_retirement::MountedOwnerAliasRetirementV1;use std::sync::Arc;use super::alias_retirement::MountedOwnerCellV1;
 struct Payload{bytes:Option<Vec<u8>>}
 trait OriginalPayload:Send{fn bytes(&self)->&Option<Vec<u8>>;fn bytes_mut(&mut self)->&mut Option<Vec<u8>>;}
 impl OriginalPayload for Payload{fn bytes(&self)->&Option<Vec<u8>>{&self.bytes}fn bytes_mut(&mut self)->&mut Option<Vec<u8>>{&mut self.bytes}}
 let policy=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:65536,maximum_depth:1};let mut rows=Vec::new();
 for aliases in [0,1,3]{
  let(source,born)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Arc::new(MountedOwnerCellV1::new(Box::new(Payload{bytes:Some(Vec::with_capacity(64))}) as Box<dyn OriginalPayload>)));let pointer=source.try_lock().unwrap().bytes().as_ref().unwrap().as_ptr();let shared=(0..aliases).map(|_|Arc::clone(&source)).collect::<Vec<_>>();
  let(mut cursor,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||MountedOwnerAliasRetirementV1::new(source));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(MountedOwnerAliasRetirementV1::<dyn OriginalPayload>::constructor_demands(),RetirementDemand{depth:1,..Default::default()});
  let demand=cursor.demands();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..policy},|owner|owner.bytes().is_none()).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(policy,|owner|owner.bytes().is_none()).unwrap());assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes);let arc=step.progress().released_bytes;
  if aliases>0{assert!(cursor.terminal_is_empty());assert_eq!(arc,0);for alias in &shared{assert_eq!(alias.try_lock().unwrap().bytes().as_ref().unwrap().as_ptr(),pointer);}drop(cursor);let mut iter=shared.into_iter();let final_alias=iter.next().unwrap();for alias in iter{drop(alias);}cursor=MountedOwnerAliasRetirementV1::new(final_alias);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(policy,|owner|owner.bytes().is_none()).unwrap());assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes);}else{drop(shared);}
  let arc=cursor.demands();assert_eq!(cursor.unique().unwrap().bytes().as_ref().unwrap().as_ptr(),pointer);assert!(!cursor.terminal_is_empty());let(blocked,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(policy,|owner|owner.bytes().is_none()).unwrap());assert_eq!(blocked.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor.unique_mut().unwrap().bytes_mut().take()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,64));let shell=cursor.demands().release_bytes;assert_eq!(shell,arc.release_bytes);
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(policy,|owner|owner.bytes().is_none()).unwrap());assert!(cursor.terminal_is_empty());assert_eq!(step.progress().released_bytes,shell);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,shell));let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let backing=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<MountedOwnerCellV1<dyn OriginalPayload>>();assert_eq!(born.requested_bytes,backing+64+shell);rows.push(serde_json::json!({"aliases":aliases,"birth":born.requested_bytes,"release":backing+64+shell,"arc":backing,"payload":64,"shell":shell,"constructor":0,"denied":0,"terminal":0,"sharedClosed":false}));
  println!("[DEBUG] Original alias count{aliases} same source{:?}: birth{} separately released backing{backing}/payload64/shell{shell}, denied0/0 constructor0/0 terminalDrop0",pointer,born.requested_bytes);
 }
 let parent=std::path::PathBuf::from(std::env::var_os("SEMIO_MOUNTED_OWNER_RESULTS").expect("explicit mounted output authority"));let path=parent.parent().unwrap().join("alias.json");assert!(path.to_string_lossy().encode_utf16().count()<=256);std::fs::write(path,serde_json::to_vec(&rows).unwrap()).unwrap();
}

#[test]
fn original_mounted_inline_gate_keeps_exclusion_poison_and_same_payload_without_hidden_backing(){
 use super::alias_retirement::{MountedOwnerCellV1,MountedOwnerCellErrorV1,MountedOwnerAliasRetirementV1};use std::sync::Arc;
 let(source,born)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Arc::new(MountedOwnerCellV1::new(Box::new(Some(Vec::<u8>::with_capacity(64))))));let pointer=source.try_lock().unwrap().as_ref().as_ref().unwrap().as_ptr();
 let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let guard=source.try_lock().unwrap();assert!(matches!(source.try_lock(),Err(MountedOwnerCellErrorV1::Busy)));assert_eq!(guard.as_ref().as_ref().unwrap().as_ptr(),pointer);drop(guard);});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let child=Arc::clone(&source);let other=std::thread::spawn(move||{let guard=child.try_lock().unwrap();assert_eq!(guard.as_ref().as_ref().unwrap().capacity(),64);});other.join().unwrap();
 let original=Arc::clone(&source);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let _guard=original.try_lock().unwrap();panic!("original gate producer panic")})).is_err());drop(original);assert!(matches!(source.try_lock(),Err(MountedOwnerCellErrorV1::Poisoned)));
 let mut cursor=MountedOwnerAliasRetirementV1::new(source);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:65536,maximum_depth:1};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(grant,Option::is_none).unwrap());assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes);assert_eq!(cursor.unique().unwrap().as_ref().unwrap().as_ptr(),pointer);let mut released=heap.released_bytes;
 let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor.unique_mut().unwrap().take()));assert_eq!(heap.released_bytes,64);released+=heap.released_bytes;let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(grant,Option::is_none).unwrap());assert_eq!(heap.released_bytes,step.progress().released_bytes);released+=heap.released_bytes;assert_eq!(born.requested_bytes,released);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 println!("[DEBUG] Original inline gate source{:?} retained cross-thread exclusion and poisoned payload, source birth{} exact release{}; normal gate0/0 and terminalDrop0",pointer,born.requested_bytes,released);
}
