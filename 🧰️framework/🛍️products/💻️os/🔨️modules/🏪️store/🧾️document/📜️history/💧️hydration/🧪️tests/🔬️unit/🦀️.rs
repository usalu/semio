use super::*;
use std::sync::{Arc, Mutex};

#[test]
fn persisted_hydration_release_store_entry_refuses_foreign_target_without_consuming_original_owners() {
    type Snapshot = super::super::tests::DemoSnapshot;
    type Mutation = super::super::fixture_mutations::demo::DemoMutation;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️store-entry/🔣️.json")).unwrap();
    let pack = fixture["pack"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let expected = semio_framework_pack_json::from_json_str::<ArtifactRef>(&fixture["expected"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut hydration = RetainedPersistedDocumentHydration::<Snapshot, Mutation>::from_decoded_pack(
        Snapshot::default(), pack, [7; 32], crate::os_spr::HistoryLog::default(), expected, None, "demo/v1".into(),
        super::super::tests::demo_closable_store_owners(), OperationId(811), Generation(1), u64::MAX,
        PersistedDocumentHydrationTarget::Envelope, crate::os_spr::ActorId("store-entry".into()),
    );
    let pack_identity = hydration.pack.as_ref().unwrap().as_ptr();
    let history_identity = Arc::as_ptr(hydration.history.as_ref().unwrap());
    let snapshot_identity = hydration.initial.as_ref().unwrap() as *const Snapshot;
    let catalog_identity = hydration.owners.as_ref().unwrap() as *const DocumentStoreOwners<Snapshot, Mutation>;
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(811), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
    let (step, allocated, released) = crate::test_allocation::observe_backing(|| hydration.step_store(&mut cx));
    assert!(matches!(step, PersistedDocumentStoreHydrationStep::Rejected(MemberOpenDiagnostic::Initialization)));
    assert_eq!((allocated, released), (fixture["allocatedBytes"].as_u64().unwrap() as usize, fixture["releasedBytes"].as_u64().unwrap() as usize));
    assert_eq!(cx.fuel_remaining(), 1);
    assert_eq!(hydration.pack.as_ref().unwrap().as_ptr(), pack_identity);
    assert_eq!(Arc::as_ptr(hydration.history.as_ref().unwrap()), history_identity);
    assert_eq!(hydration.initial.as_ref().unwrap() as *const Snapshot, snapshot_identity);
    assert_eq!(hydration.owners.as_ref().unwrap() as *const DocumentStoreOwners<Snapshot, Mutation>, catalog_identity);
    assert!(!hydration.terminal_is_empty());
    eprintln!("[DEBUG] Store-only hydration refuses Envelope before work/allocation; original catalog/pack/snapshot/history identities retained");
    for _ in 0..fixture["maximumCloseTurns"].as_u64().unwrap() {
        if hydration.terminal_is_empty() { break; }
        let demand = hydration.next_close_byte_demand();
        assert!(demand <= crate::os_store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES);
        hydration.close_step(1, crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES.max(demand)).unwrap();
    }
    assert!(hydration.terminal_is_empty());
}

struct WholeAllocation { owner: Option<Vec<u8>> }
impl ErasedSnapshotRetirement for WholeAllocation {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        let Some(owner) = self.owner.as_ref() else { return Ok(SnapshotRetirementStep::Complete); };
        if maximum_items == 0 || maximum_bytes < owner.capacity() { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        let extent = owner.capacity();
        self.owner.take();
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: extent })
    }
    fn next_close_byte_demand(&self) -> usize { self.owner.as_ref().map_or(0, Vec::capacity) }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none() }
}

#[test]
fn persisted_hydration_release_fold_caller_funds_exact_cleanup_without_copy_fuel_expansion() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    for logical in fixture["foldLogicalBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        for extent in fixture["foldAllocationBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
            let mut owner = WholeAllocation { owner: Some(vec![42; extent]) };
            let grant = hydration_fold_byte_grant(logical, owner.next_close_byte_demand()).unwrap();
            assert_eq!(grant, logical.max(extent));
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(1, grant).unwrap());
            assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 1, released_bytes: extent });
            assert_eq!((allocated, released), (0, extent));
            assert_eq!(hydration_fold_byte_grant(logical, 0), Some(logical));
            eprintln!("[DEBUG] Fold caller cleanup logical={} wholePhysical={} receipt={} work1 no logical work on cleanup turn", logical, extent, released);
        }
        assert_eq!(hydration_fold_byte_grant(logical, fixture["refusedAllocationBytes"].as_u64().unwrap() as usize), None);
    }
}

#[test]
fn persisted_hydration_release_funds_actual_whole_allocation_and_separate_terminal_frame() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    for extent in fixture["allocationBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        assert!(extent <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
        let mut active: Option<Box<dyn ErasedSnapshotRetirement>> = Some(Box::new(WholeAllocation { owner: Some(vec![42; extent]) }));
        assert_eq!(super::super::artifact_retirement_box_byte_demand(active.as_ref().unwrap()), extent);
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(805), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
        let (pending, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_retirement(&mut active, &mut cx).unwrap());
        assert!(pending);
        assert_eq!((allocated, released), (0, extent), "fund the retained physical allocation under its existing admission ceiling");
        assert_eq!(cx.fuel_remaining(), 0);
        assert!(active.as_ref().unwrap().terminal_is_empty());
        let frame = std::mem::size_of_val(active.as_ref().unwrap().as_ref());
        let mut cx = StepContext::new(OperationId(805), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
        let (_, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_retirement(&mut active, &mut cx).unwrap());
        assert_eq!((allocated, released), (0, frame));
        assert!(active.is_none());
        eprintln!("[DEBUG] persisted hydration whole physical extent={} release={} terminalFrame={} fuel1 admission262144", extent, extent, frame);
    }
}

struct AllocationProbe {
    owner: Option<Arc<Vec<u8>>>,
    observed: Arc<Mutex<Vec<(usize, usize)>>>,
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct UnitFactory;
impl crate::os_store::ArtifactOwnedValueRetirementFactory<()> for UnitFactory {
    fn retire_owned(&self, _: ()) -> Box<dyn ErasedSnapshotRetirement> { Box::new(WholeAllocation { owner: None }) }
}

#[test]
fn persisted_hydration_release_runtime_funds_whole_physical_extent_with_one_work_unit() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    for extent in fixture["runtimeAllocationBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let mut runtime = ArtifactStoreInitializationRuntime::new("physical-hydration", "unit/v1", Arc::new(()), [7; 32], crate::os_spr::ActorId(String::new()));
        *runtime.close_active = Some(Box::new(WholeAllocation { owner: Some(vec![42; extent]) }));
        assert_eq!(runtime.next_close_byte_demand(), extent);
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(807), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
        let (result, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_runtime(&mut runtime, &UnitFactory, &mut cx));
        let work_left = cx.fuel_remaining();
        let retained_terminal = runtime.close_active.as_ref().unwrap().terminal_is_empty();
        for _ in 0..1024 {
            if runtime.terminal_is_empty() { break; }
            runtime.close_step(&UnitFactory, 1, fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize).unwrap();
        }
        assert!(runtime.terminal_is_empty());
        assert_eq!(result, Ok(false));
        assert_eq!((allocated, released), (0, extent));
        assert_eq!(work_left, 0);
        assert!(retained_terminal);
        eprintln!("[DEBUG] hydration envelope runtime exactPhysical={} workFuel1 heapBirth0 terminalFrameSeparate=true", extent);
    }
}

#[test]
fn persisted_hydration_release_refuses_extent_outside_existing_admission() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    let extent = fixture["refusedAllocationBytes"].as_u64().unwrap() as usize;
    let mut active: Option<Box<dyn ErasedSnapshotRetirement>> = Some(Box::new(WholeAllocation { owner: Some(vec![42; extent]) }));
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(806), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_retirement(&mut active, &mut cx));
    assert_eq!(result, Err(MemberOpenDiagnostic::Capacity));
    assert_eq!((allocated, released), (0, 0));
    assert_eq!(cx.fuel_remaining(), 1);
    assert!(!active.as_ref().unwrap().terminal_is_empty());
    assert_eq!(active.as_ref().unwrap().next_close_byte_demand(), extent);
    eprintln!("[DEBUG] persisted hydration denied outside-admission extent={} retained=true heapBirth=0 heapFree=0 fuelUnspent=1", extent);
}

impl ErasedSnapshotRetirement for AllocationProbe {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        self.observed.lock().unwrap().push((maximum_items, maximum_bytes));
        let Some(owner) = &self.owner else { return Ok(SnapshotRetirementStep::Complete) };
        if maximum_items == 0 || Arc::strong_count(owner) != 1 || owner.capacity() > maximum_bytes { return Ok(SnapshotRetirementStep::Blocked); }
        let owner = Arc::into_inner(self.owner.take().unwrap()).unwrap();
        let released_bytes = owner.capacity();
        drop(owner);
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes })
    }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none() }
}

#[test]
fn persisted_hydration_release_runtime_refuses_extent_outside_existing_admission() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    let extent = fixture["refusedAllocationBytes"].as_u64().unwrap() as usize;
    let mut runtime = ArtifactStoreInitializationRuntime::new("refused-hydration", "unit/v1", Arc::new(()), [7; 32], crate::os_spr::ActorId(String::new()));
    *runtime.close_active = Some(Box::new(WholeAllocation { owner: Some(vec![42; extent]) }));
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(808), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_runtime(&mut runtime, &UnitFactory, &mut cx));
    let work_left = cx.fuel_remaining();
    let retained_demand = runtime.next_close_byte_demand();
    for _ in 0..1024 {
        if runtime.terminal_is_empty() { break; }
        runtime.close_step(&UnitFactory, 1, extent).unwrap();
    }
    assert!(runtime.terminal_is_empty());
    assert_eq!(result, Err(MemberOpenDiagnostic::Capacity));
    assert_eq!((allocated, released), (0, 0));
    assert_eq!((work_left, retained_demand), (1, extent));
    eprintln!("[DEBUG] hydration envelope runtime deniedPhysical={} retained=true workFuel1Unspent heapBirth0 heapFree0", extent);
}

#[test]
fn persisted_hydration_release_pages_are_independent_of_step_fuel() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️release-pages/🔣️.json")).unwrap();
    let maximum_items = fixture["maximumReleaseItems"].as_u64().unwrap() as usize;
    let maximum_bytes = fixture["maximumReleaseBytes"].as_u64().unwrap() as usize;
    for row in fixture["cases"].as_array().unwrap() {
        let bytes = row["allocationBytes"].as_u64().unwrap() as usize;
        let owner = Arc::new(vec![42; bytes]);
        assert_eq!(owner.capacity(), bytes);
        let reader = (row["readers"].as_u64().unwrap() != 0).then(|| Arc::clone(&owner));
        let observed = Arc::new(Mutex::new(Vec::new()));
        let mut active: Option<Box<dyn ErasedSnapshotRetirement>> = Some(Box::new(AllocationProbe { owner: Some(owner), observed: Arc::clone(&observed) }));
        let fuel = row["fuel"].as_u64().unwrap();
        let cancel = semio_framework_job::root_cancel_token();
        if row["cancelled"].as_bool().unwrap() { cancel.cancel_now(); }
        let deadline = if row["expired"].as_bool().unwrap() { 1 } else { u64::MAX };
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(803), Generation(1), semio_framework_job::StepBudget::new(fuel, deadline), cancel, || Some(1), &mut sequence);
        assert!(drive_hydration_retirement(&mut active, &mut cx).expect("a normal blocked owner remains pending"));
        assert!(active.is_some());
        let called = row["called"].as_bool().unwrap();
        let grants = observed.lock().unwrap();
        assert_eq!(serde_json::to_value(&*grants).unwrap(), if called { serde_json::json!([[maximum_items, maximum_bytes]]) } else { serde_json::json!([]) });
        drop(grants);
        let released = row["releasedBytes"].as_u64().unwrap() as usize;
        assert_eq!(active.as_ref().unwrap().terminal_is_empty(), released != 0);
        assert_eq!(fuel - cx.fuel_remaining(), u64::from(called));
        drop(reader);
        let active_owner = active.as_mut().unwrap();
        while active_owner.close_step(1, bytes).unwrap() != SnapshotRetirementStep::Complete {}
        assert!(active_owner.terminal_is_empty());
        active.take();
    }
    eprintln!("[DEBUG] Persisted hydration 9 neutral release cases retain fuel, cancellation, deadline, real reader aliases and 4096-byte physical ceiling");
}
