use super::*;
use std::sync::{Arc, Mutex};

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
    let mut original_retained_progress_1 = semio_framework_value::RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(811), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, crate::os_store::component::tests::physical_test_close_grant()), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut original_retained_progress_1);
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
        hydration.close_step(crate::os_store::component::tests::physical_test_close_grant()).unwrap();
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
        let mut original_retained_progress_2 = semio_framework_value::RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(805), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, crate::os_store::component::tests::physical_test_close_grant()), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut original_retained_progress_2);
        let (pending, allocated, released) = crate::test_allocation::observe_backing(|| drive_hydration_retirement(&mut active, &mut cx, release_grant(extent)).unwrap());
        assert!(pending);
        assert_eq!((allocated, released), (0, extent), "fund the retained physical allocation under its existing admission ceiling");
        assert_eq!(cx.fuel_remaining(), 0);
        assert!(active.as_ref().unwrap().terminal_is_empty());
        let frame = std::mem::size_of_val(active.as_ref().unwrap().as_ref());
        let mut original_retained_progress_3 = semio_framework_value::RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(805), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, crate::os_store::component::tests::physical_test_close_grant()), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut original_retained_progress_3);
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
        let mut runtime = ArtifactStoreInitializationRuntime::new("physical-hydration", "unit/v1", Arc::new(()), [7; 32], crate::os_spr::ActorId(semio_framework_value::SharedUtf8::default()));
        *runtime.close_active = Some(Box::new(WholeAllocation { owner: Some(vec![42; extent]) }));
        assert_eq!(runtime.next_close_release_byte_demand().unwrap(), extent);
        let factory = unit_factory();
        let mut sequence = 0;
        let mut original_retained_progress_4 = semio_framework_value::RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(807), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, crate::os_store::component::tests::physical_test_close_grant()), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut original_retained_progress_4);
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
    let mut original_retained_progress_5 = semio_framework_value::RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(806), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, crate::os_store::component::tests::physical_test_close_grant()), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut original_retained_progress_5);
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
    let mut runtime = ArtifactStoreInitializationRuntime::new("refused-hydration", "unit/v1", Arc::new(()), [7; 32], crate::os_spr::ActorId(semio_framework_value::SharedUtf8::default()));
    *runtime.close_active = Some(Box::new(WholeAllocation { owner: Some(vec![42; extent]) }));
    let factory = unit_factory();
    let mut sequence = 0;
    let mut original_retained_progress_6 = semio_framework_value::RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(808), Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, crate::os_store::component::tests::physical_test_close_grant()), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut original_retained_progress_6);
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
        let mut original_retained_progress_7 = semio_framework_value::RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(803), Generation(1), semio_framework_job::StepBudget::new(fuel, deadline, crate::os_store::component::tests::physical_test_close_grant()), cancel, || Some(1), &mut sequence, &mut original_retained_progress_7);
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

#[test]
fn member_hydration_ready_auxiliary_retains_empty_capacity_until_paid_close() {
    type Snapshot = super::super::tests::DemoSnapshot;
    type Mutation = super::super::fixture_mutations::demo::DemoMutation;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    let policy = &fixture["readyAuxiliaryPolicy"];
    let grant = RetainedCloneGrant { maximum_items: policy["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: policy["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: policy["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: policy["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: policy["maximumDepth"].as_u64().unwrap() as usize };
    let extent = fixture["readyAuxiliaryCapacity"].as_u64().unwrap() as usize;
    let original_text=String::with_capacity(extent);
    let (admitted,source_allocated,source_released)=crate::test_allocation::observe_backing(||semio_framework_value::SharedUtf8::admit(original_text,grant));
    let (actor,source_receipt)=admitted.unwrap_or_else(|_|panic!("original actor frame admission"));
    assert!(source_receipt.fits(grant));assert_eq!((source_allocated,source_released),(source_receipt.retained_capacity_bytes,0));
    let original_owned_bytes=extent+source_allocated;
    let mut owner = RetainedPersistedDocumentHydration::<Snapshot, Mutation> {
        actor: ManuallyDrop::new(crate::os_spr::ActorId(actor)),
        pack: ManuallyDrop::new(None), initial: ManuallyDrop::new(None), history: ManuallyDrop::new(None),
        fold_job: ManuallyDrop::new(None), normalized_transitions: ManuallyDrop::new(None), normalized_conflicts: ManuallyDrop::new(None),
        replay_ids: ManuallyDrop::new(None), replay_order: ManuallyDrop::new(None), replay_total: 0,
        pin_refs: ManuallyDrop::new(None), loaded_result: ManuallyDrop::new(None), edit_lookup: ManuallyDrop::new(None), mutation_lookup: ManuallyDrop::new(None),
        target_source: ManuallyDrop::new(None), target_decoder: ManuallyDrop::new(None), pending_target: ManuallyDrop::new(None), target_address: ManuallyDrop::new(None),
        fold_completed: 0, progress_high_water: std::cell::Cell::new(0), fold: ManuallyDrop::new(None),
        expected: ManuallyDrop::new(None), owner: ManuallyDrop::new(None), schema: ManuallyDrop::new(None), envelope: ManuallyDrop::new(None),
        runtime: ManuallyDrop::new(None), initialization_catalog: ManuallyDrop::new(None), replay: ManuallyDrop::new(None), owners: ManuallyDrop::new(None),
        pending_edit: ManuallyDrop::new(None), pending_messages: ManuallyDrop::new(None), active: ManuallyDrop::new(None),
        operation: OperationId(814), generation: Generation(1), expires_at_us: u64::MAX,
        target: PersistedDocumentHydrationTarget::Store { generation: 0 }, phase: Phase::Finish,
        edit_index: 0, operation_index: 0, record_index: 0, pin_group_index: 0, pin_index: 0, pack_scanned: 0,
        pack_hasher: semio_framework_hash::Hasher::new(), pack_digest: None, diagnostic: None, terminal: true,
    };
    assert_eq!(owner.actor.0.len(), 0);
    assert_eq!(owner.actor.0.original_allocation_bytes(), original_owned_bytes);
    assert!(!owner.terminal_is_empty());
    let original = owner.actor.0.as_ptr();
    for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }] {
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(denied).unwrap());
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!((allocated, released), (0, 0));
        assert_eq!(owner.actor.0.as_ptr(), original);
        assert!(owner.active.is_none());
        assert!(!owner.terminal_is_empty());
    }
    let (birth, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(grant).unwrap());
    assert!(birth.progress().fits(grant));
    assert_eq!(birth.progress().retained_capacity_bytes, allocated);
    assert!(allocated > 0);
    assert_eq!(released, 0);
    assert_eq!(owner.actor.0.original_allocation_bytes(), 0);
    assert!(owner.active.is_some());
    let mut total_born = allocated;
    let mut total_released = released;
    for _ in 0..64 {
        if owner.terminal_is_empty() { break; }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(step.progress().retained_capacity_bytes, allocated);
        assert_eq!(step.progress().released_bytes, released);
        assert!(owner.terminal_is_empty() || step.progress() != RetainedCloneProgress::default());
        total_born += allocated;
        total_released += released;
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(total_released, original_owned_bytes + total_born);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(owner));
    assert_eq!((allocated, released), (0, 0));
    eprintln!("[DEBUG] ready hydration retains original empty actor capacity={} without catalog; every birth/backing/frame paid, denied identity preserved", extent);
}

#[test]
fn funded_verified_genesis_preserves_original_refusal_and_every_shared_backing_receipt() {
    use crate::os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
    use semio_framework_value::retirement::{shared::SharedControlledRetirement, controlled::ControlledRetirement};
    fn close(mut snapshot: impl ErasedSnapshotRetirement, mut pack: impl ErasedSnapshotRetirement, policy: RetainedCloneGrant, expected: usize) {
        let mut freed = 0;
        for _ in 0..64 {
            if snapshot.terminal_is_empty() && pack.terminal_is_empty() { break; }
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| if !snapshot.terminal_is_empty() { snapshot.close_step(policy).unwrap() } else { pack.close_step(policy).unwrap() });
            assert!(step.progress().fits(policy));
            assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            assert_eq!(allocated, 0); freed += released;
        }
        assert!(snapshot.terminal_is_empty() && pack.terminal_is_empty());
        assert_eq!(freed, expected);
        let (_, allocated, released) = crate::test_allocation::observe_backing(|| { drop(snapshot); drop(pack); });
        assert_eq!((allocated, released), (0, 0));
        eprintln!("[DEBUG] verified Genesis original refusal custody / funded shared owner complete paid free={freed}, terminal0heap");
    }

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    let row = &fixture["verifiedGenesis"]; let supplied = &row["policy"];
    let policy = RetainedCloneGrant { maximum_items: supplied["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: supplied["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: supplied["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: supplied["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: supplied["maximumDepth"].as_u64().unwrap() as usize };
    let demand = AdmittedArtifactGenesis::<String>::verified_pack_birth_demand().unwrap();
    let make = || { let mut snapshot = String::with_capacity(row["snapshotCapacityBytes"].as_u64().unwrap() as usize); snapshot.push_str(row["text"].as_str().unwrap()); let mut pack = Vec::with_capacity(row["packCapacityBytes"].as_u64().unwrap() as usize); pack.extend_from_slice(snapshot.as_bytes()); (snapshot, pack) };
    for denied in [RetainedCloneGrant { maximum_items: 0, ..policy }, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..policy }, RetainedCloneGrant { maximum_depth: 0, ..policy }, policy] {
        let (snapshot, pack) = make();
        let original = (snapshot.as_ptr(), pack.as_ptr());
        let original_bytes = snapshot.capacity() + pack.capacity();
        let (result, allocated, released) = crate::test_allocation::observe_backing(|| AdmittedArtifactGenesis::admit_verified_pack(snapshot, pack, [7; 32], denied));
        assert_eq!(released, 0);
        match result {
            Err((error, snapshot, pack)) => {
                assert!(denied != policy);
                assert!(matches!(error.kind, ValueRefusalKind::WorkLimit | ValueRefusalKind::OwnershipLimit | ValueRefusalKind::DepthLimit));
                assert_eq!((allocated, released), (0, 0));
                assert_eq!((snapshot.as_ptr(), pack.as_ptr()), original);
                assert_eq!(serde_json::to_value(snapshot.as_str()).unwrap(), serde_json::json!(row["text"].as_str().unwrap()));
                assert_eq!(pack.as_slice(), snapshot.as_bytes());
                close(ControlledRetirement::new(snapshot).unwrap(), ControlledRetirement::new(pack).unwrap(), policy, original_bytes);
            }
            Ok((genesis, progress)) => {
                assert_eq!(denied, policy);
                assert!(progress.fits(policy));
                assert_eq!(progress.retained_capacity_bytes, allocated);
                assert_eq!(allocated, demand.capacity_bytes);
                assert_eq!(genesis.facts().digest(), [7; 32]);
                let (snapshot, pack) = genesis.into_owners();
                assert_eq!((snapshot.as_ptr(), pack.as_ptr()), original);
                assert_eq!(serde_json::to_value(snapshot.as_str()).unwrap(), serde_json::json!(row["text"].as_str().unwrap()));
                assert_eq!(pack.as_slice(), snapshot.as_bytes());
                close(SharedControlledRetirement::lease(snapshot), SharedControlledRetirement::lease(pack), policy, original_bytes + allocated);
            }
        }
    }
}

#[test]
fn message_ledger_phased_birth_cancellation_preserves_exact_original_pages() {
    use crate::os_store::ArtifactEditMessageLedger;
    use super::super::ArtifactEditMessageLedgerRetirement;
    let plain: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    let row = &plain["messageLedgerBirth"]; let supplied = &row["policy"];
    let policy = RetainedCloneGrant { maximum_items: supplied["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: supplied["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: supplied["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: supplied["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: supplied["maximumDepth"].as_u64().unwrap() as usize };
    for cut in row["cancelCuts"].as_array().unwrap() {
        let (mut ledger, born, freed) = crate::test_allocation::observe_backing(ArtifactEditMessageLedger::empty);
        assert_eq!((born, freed), (0, 0));
        let mut total = 0;
        for _ in 0..cut.as_u64().unwrap() {
            let demand = ledger.admission_demands().unwrap();
            if ledger.admission_is_complete() { break; }
            assert!(demand.capacity_bytes <= policy.maximum_capacity_bytes);
            for denied in [RetainedCloneGrant { maximum_items: 0, ..policy }, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..policy }] {
                let (progress, born, freed) = crate::test_allocation::observe_backing(|| ledger.admit_next(denied).unwrap());
                assert_eq!((progress, born, freed), (RetainedCloneProgress::default(), 0, 0));
                assert_eq!(ledger.admission_demands().unwrap(), demand);
            }
            let (progress, born, freed) = crate::test_allocation::observe_backing(|| ledger.admit_next(policy).unwrap());
            assert!(progress.fits(policy));
            assert_eq!(progress.retained_capacity_bytes, demand.capacity_bytes);
            assert_eq!((born, freed), (progress.retained_capacity_bytes, 0));
            total += born;
        }
        if ledger.admission_is_complete() {
            let mut tickets = [None; 3];
            let mut original = [std::ptr::null(); 3];
            for (index, name) in row["ids"].as_array().unwrap().iter().enumerate() {
                let (id, born, freed) = crate::test_allocation::observe_backing(|| { let mut id = String::with_capacity(37); id.push_str(name.as_str().unwrap()); id });
                assert_eq!((born, freed), (id.capacity(), 0)); total += born; original[index] = id.as_ptr();
                let (result, born, freed) = crate::test_allocation::observe_backing(|| ledger.try_push(crate::os_spr::EditMessages { edit_id: id, messages: Vec::new() }));
                assert_eq!((born, freed), (0, 0));
                let ticket = result.unwrap_or_else(|_| panic!("original admitted fixed ticket remains available"));
                assert_eq!(ticket.slot as usize, index);
                assert_eq!(ledger.get(ticket).unwrap().edit_id.as_ptr(), original[index]); tickets[index] = Some(ticket);
            }
            let old = tickets[1].unwrap();
            let original_entry = ledger.remove(old).unwrap();
            assert_eq!(original_entry.edit_id.as_ptr(), original[1]); assert!(ledger.get(old).is_none());
            let (result, born, freed) = crate::test_allocation::observe_backing(|| ledger.try_push(original_entry));
            assert_eq!((born, freed), (0, 0));
            let replacement = result.unwrap_or_else(|_| panic!("same original free slot accepts its retained entry"));
            assert_eq!((replacement.slot, replacement.generation), (old.slot, old.generation + 1));
            assert_eq!(ledger.get(replacement).unwrap().edit_id.as_ptr(), original[1]);
            assert_eq!(serde_json::to_value(ledger.iter().map(|entry| entry.edit_id.as_str()).collect::<Vec<_>>()).unwrap(), row["afterReinsert"]);
        }
        let mut retirement = ArtifactEditMessageLedgerRetirement::new(ledger);
        let mut freed_total = 0;
        for _ in 0..512 {
            if retirement.terminal_is_empty() { break; }
            let demand = retirement.next_release_byte_demand().unwrap();
            assert!(demand <= policy.maximum_release_bytes);
            let (denied, born, freed) = crate::test_allocation::observe_backing(|| retirement.close_step(RetainedCloneGrant { maximum_release_bytes: 0, ..policy }).unwrap());
            assert_eq!((denied.progress(), born, freed), (RetainedCloneProgress::default(), 0, 0));
            let (step, born, freed) = crate::test_allocation::observe_backing(|| retirement.close_step(policy).unwrap());
            assert!(step.progress().fits(policy));
            assert_eq!((born, freed), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            assert_eq!(born, 0); freed_total += freed;
        }
        assert!(retirement.terminal_is_empty()); assert_eq!(freed_total, total);
        let (_, born, freed) = crate::test_allocation::observe_backing(|| drop(retirement));
        assert_eq!((born, freed), (0, 0));
        eprintln!("[DEBUG] original message pages cancellation cut={} born={} paidFree={} terminal0heap", cut, total, freed_total);
    }
}

#[test]
fn member_envelope_phased_birth_cancel_cuts_conserve_original_catalog_allocations() {
    use semio_framework_value::retirement::OwnedValueRetirementFactory;
    use crate::os_store::{empty_document_envelope_from_genesis_owners, ArtifactOwnedValueRetirementFactory};
    use super::super::ArtifactStoreEnvelopeRetirement;
    use crate::os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
    let plain: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️physical-demand/🔣️.json")).unwrap();
    let row = &plain["envelopeBirth"]; let supplied = &row["policy"];
    let policy = RetainedCloneGrant { maximum_items: supplied["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: supplied["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: supplied["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: supplied["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: supplied["maximumDepth"].as_u64().unwrap() as usize };
    for cut in row["cancelCuts"].as_array().unwrap() {
        let (setup, setup_born, setup_freed) = crate::test_allocation::observe_backing(|| {
            let snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(OwnedValueRetirementFactory::<String>::default());
            let mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(OwnedValueRetirementFactory::<String>::default());
            (snapshots, mutations)
        });
        assert_eq!(setup_freed, 0);
        let schema = row["metadata"]["schema"].as_str().unwrap().to_owned();
        let id = row["metadata"]["id"].as_str().unwrap().to_owned();
        let metadata_bytes = schema.capacity() + id.capacity();
        let original = (schema.as_ptr(), id.as_ptr());
        let (genesis, genesis_born, genesis_freed) = crate::test_allocation::observe_backing(|| AdmittedArtifactGenesis::admit_verified_pack(String::new(), Vec::new(), [9; 32], policy).unwrap_or_else(|_| panic!("fixed fixture policy funds two original Genesis frames")).0);
        assert_eq!(genesis_freed, 0);
        let (mut envelope, born, freed) = crate::test_allocation::observe_backing(|| empty_document_envelope_from_genesis_owners::<String, String>(schema, id, genesis, None));
        assert_eq!((born, freed), (0, 0));
        assert_eq!((envelope.schema.as_ptr(), envelope.id.as_ptr()), original);
        let mut all_born = setup_born + genesis_born;
        for _ in 0..cut.as_u64().unwrap() {
            let demand = envelope.document_admission_demands().unwrap();
            if envelope.document_admission_is_complete() { break; }
            assert!(demand.capacity_bytes <= policy.maximum_capacity_bytes);
            for denied in [RetainedCloneGrant { maximum_items: 0, ..policy }, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..policy }] {
                let (progress, born, freed) = crate::test_allocation::observe_backing(|| envelope.admit_document_next(denied).unwrap());
                assert_eq!((progress, born, freed), (RetainedCloneProgress::default(), 0, 0));
                assert_eq!(envelope.document_admission_demands().unwrap(), demand);
                assert_eq!((envelope.schema.as_ptr(), envelope.id.as_ptr()), original);
            }
            let (progress, born, freed) = crate::test_allocation::observe_backing(|| envelope.admit_document_next(policy).unwrap());
            assert!(progress.fits(policy)); assert_eq!((born, freed), (progress.retained_capacity_bytes, 0));
            assert_eq!(born, demand.capacity_bytes); all_born += born;
        }
        if cut.as_u64().unwrap() == row["totalBirthTurns"].as_u64().unwrap() { assert!(envelope.document_admission_is_complete()); assert_eq!(envelope.document_admission_demands().unwrap(), Default::default()); }
        let mut owner = ArtifactStoreEnvelopeRetirement::new(envelope, setup.0, setup.1);
        let mut freed_total = 0;
        for _ in 0..1024 {
            if owner.terminal_is_empty() { break; }
            let (step, born, freed) = crate::test_allocation::observe_backing(|| owner.close_step(policy).unwrap());
            assert!(step.progress().fits(policy)); assert_eq!((born, freed), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            all_born += born; freed_total += freed;
        }
        assert!(owner.terminal_is_empty()); assert_eq!(freed_total, all_born + metadata_bytes);
        let (_, born, freed) = crate::test_allocation::observe_backing(|| drop(owner)); assert_eq!((born, freed), (0, 0));
        eprintln!("[DEBUG] member envelope original phased cancellation cut={} born={} paidFree={} metadata={} terminal0heap", cut, all_born, freed_total, metadata_bytes);
    }
}
