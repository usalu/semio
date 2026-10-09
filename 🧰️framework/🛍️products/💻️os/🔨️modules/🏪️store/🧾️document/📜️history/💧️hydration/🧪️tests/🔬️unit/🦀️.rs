use super::*;
use std::sync::{Arc, Mutex};

fn grant_of(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

fn quoted(owner: &dyn ErasedSnapshotRetirement, body: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes: owner.next_copy_byte_demand().unwrap(), capacity_bytes: owner.next_capacity_byte_demand(body).unwrap(), release_bytes: owner.next_release_byte_demand().unwrap(), depth: owner.next_depth_demand().unwrap() }
}

fn release_grant(bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: bytes, maximum_depth: 64 }
}

fn admission_grant(bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 64 }
}

fn wide_grant() -> RetainedCloneGrant {
    admission_grant(crate::os_store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES)
}

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
    let (step, allocated, released) = crate::test_allocation::observe_backing(|| hydration.step_store(&mut cx, wide_grant()));
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
        let demand = quoted(&hydration, crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES);
        assert!(demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) <= crate::os_store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES);
        hydration.close_step(grant_of(demand)).unwrap();
    }
    assert!(hydration.terminal_is_empty());
}

struct WholeAllocation { owner: Option<Vec<u8>> }
impl ErasedSnapshotRetirement for WholeAllocation {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let Some(owner) = self.owner.as_ref() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        let extent = owner.capacity();
        if grant.maximum_items == 0 || grant.maximum_release_bytes < extent { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.owner.take();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..Default::default() }))
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owner.as_ref().map_or(0, Vec::capacity)) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none() }
}

#[test]
fn persisted_hydration_release_fold_caller_funds_exact_cleanup_without_copy_fuel_expansion() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    for logical in fixture["foldLogicalBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        for extent in fixture["foldAllocationBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
            let mut owner = WholeAllocation { owner: Some(vec![42; extent]) };
            let grant = hydration_fold_byte_grant(logical, owner.next_release_byte_demand().unwrap()).unwrap();
            assert_eq!(grant, logical.max(extent));
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(release_grant(grant)).unwrap());
            assert_eq!(step, RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..Default::default() }));
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
        assert_eq!(super::super::artifact_retirement_box_demands(active.as_ref().unwrap(), 0).unwrap().release_bytes, extent);
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(805), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
        let (pending, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_retirement(&mut active, &mut cx, release_grant(extent)).unwrap());
        assert!(pending);
        assert_eq!((allocated, released), (0, extent), "fund the retained physical allocation under its existing admission ceiling");
        assert_eq!(cx.fuel_remaining(), 0);
        assert!(active.as_ref().unwrap().terminal_is_empty());
        let frame = std::mem::size_of_val(active.as_ref().unwrap().as_ref());
        let mut cx = StepContext::new(OperationId(805), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
        let (_, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_retirement(&mut active, &mut cx, release_grant(frame)).unwrap());
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
    fn retirement_birth_bytes(&self, _: &()) -> usize { std::mem::size_of::<WholeAllocation>() }
    fn retire_owned(&self, value: (), grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, ())> {
        let birth = std::mem::size_of::<WholeAllocation>();
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < birth { return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "unit retirement birth is unfunded"), value)); }
        Ok((Box::new(WholeAllocation { owner: None }), RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: birth, ..Default::default() }))
    }
}

fn unit_factory() -> Arc<dyn crate::os_store::ArtifactOwnedValueRetirementFactory<()>> { Arc::new(UnitFactory) }

#[test]
fn persisted_hydration_release_runtime_funds_whole_physical_extent_with_one_work_unit() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    for extent in fixture["runtimeAllocationBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let mut runtime = ArtifactStoreInitializationRuntime::new("physical-hydration", "unit/v1", Arc::new(()), [7; 32], crate::os_spr::ActorId(String::new()));
        *runtime.close_active = Some(Box::new(WholeAllocation { owner: Some(vec![42; extent]) }));
        assert_eq!(runtime.next_close_release_byte_demand().unwrap(), extent);
        let factory = unit_factory();
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(807), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
        let (result, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_runtime(&mut runtime, &factory, &mut cx, release_grant(extent)));
        let work_left = cx.fuel_remaining();
        let retained_terminal = runtime.close_active.as_ref().unwrap().terminal_is_empty();
        for _ in 0..1024 {
            if runtime.terminal_is_empty() { break; }
            runtime.close_step(&factory, admission_grant(fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize)).unwrap();
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
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_retirement(&mut active, &mut cx, release_grant(fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize)));
    assert_eq!(result, Ok(true));
    assert_eq!((allocated, released), (0, 0));
    assert_eq!(cx.fuel_remaining(), 1);
    assert!(!active.as_ref().unwrap().terminal_is_empty());
    assert_eq!(active.as_ref().unwrap().next_release_byte_demand().unwrap(), extent);
    eprintln!("[DEBUG] persisted hydration denied outside-admission extent={} retained=true heapBirth=0 heapFree=0 fuelUnspent=1", extent);
}

impl ErasedSnapshotRetirement for AllocationProbe {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.observed.lock().unwrap().push((grant.maximum_items, grant.maximum_release_bytes));
        let Some(owner) = &self.owner else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        if grant.maximum_items == 0 || Arc::strong_count(owner) != 1 || owner.capacity() > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let owner = Arc::into_inner(self.owner.take().unwrap()).unwrap();
        let released_bytes = owner.capacity();
        drop(owner);
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }))
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owner.as_ref().map_or(0, |owner| owner.capacity())) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none() }
}

#[test]
fn persisted_hydration_release_runtime_refuses_extent_outside_existing_admission() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    let extent = fixture["refusedAllocationBytes"].as_u64().unwrap() as usize;
    let mut runtime = ArtifactStoreInitializationRuntime::new("refused-hydration", "unit/v1", Arc::new(()), [7; 32], crate::os_spr::ActorId(String::new()));
    *runtime.close_active = Some(Box::new(WholeAllocation { owner: Some(vec![42; extent]) }));
    let factory = unit_factory();
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(808), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_runtime(&mut runtime, &factory, &mut cx, release_grant(fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize)));
    let work_left = cx.fuel_remaining();
    let retained_demand = runtime.next_close_release_byte_demand().unwrap();
    for _ in 0..1024 {
        if runtime.terminal_is_empty() { break; }
        runtime.close_step(&factory, admission_grant(extent)).unwrap();
    }
    assert!(runtime.terminal_is_empty());
    assert_eq!(result, Ok(false));
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
        assert!(drive_hydration_retirement(&mut active, &mut cx, RetainedCloneGrant { maximum_items, ..release_grant(maximum_bytes) }).expect("a normal blocked owner remains pending"));
        assert!(active.is_some());
        let called = row["called"].as_bool().unwrap();
        let grants = observed.lock().unwrap();
        assert_eq!(serde_json::to_value(&*grants).unwrap(), if called { serde_json::json!([[maximum_items, maximum_bytes]]) } else { serde_json::json!([]) });
        drop(grants);
        let released = row["releasedBytes"].as_u64().unwrap() as usize;
        assert_eq!(active.as_ref().unwrap().terminal_is_empty(), released != 0);
        assert_eq!(fuel - cx.fuel_remaining(), u64::from(released != 0));
        drop(reader);
        let active_owner = active.as_mut().unwrap();
        while !matches!(active_owner.close_step(release_grant(bytes)).unwrap(), RetainedCloneStep::Complete(_)) {}
        assert!(active_owner.terminal_is_empty());
        active.take();
    }
    eprintln!("[DEBUG] Persisted hydration 9 neutral release cases retain fuel, cancellation, deadline, real reader aliases and 4096-byte physical ceiling");
}
