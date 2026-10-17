//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 2 of the cut by subject).

use super::*;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ExactDemoInitialSnapshotRetirementFactory(#[factory_child] Arc<std::sync::atomic::AtomicUsize>);

impl ArtifactOwnedValueRetirementFactory<DemoSnapshot> for ExactDemoInitialSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _: &DemoSnapshot) -> usize { std::mem::size_of::<ExactDemoSnapshotRetirement>() }
    fn retire_owned(&self, value: DemoSnapshot, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, DemoSnapshot)> {
        admit_artifact_retirement(value,grant,|original|ExactDemoSnapshotRetirement { owner: std::mem::ManuallyDrop::new(None),value: std::mem::ManuallyDrop::new(Some(original)),child: std::mem::ManuallyDrop::new(None),completed: std::mem::ManuallyDrop::new(Some(Arc::clone(&self.0))),completion_recorded: false })
    }
}


#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct CountingOwnedPreparationFactory {
    #[factory_owned]
    inner: DemoOneItemPreparationFactory,
    #[factory_child]
    preflights: Arc<std::sync::atomic::AtomicUsize>,
}

impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for CountingOwnedPreparationFactory {
    fn begin_batch_digest(&self,edit:&mut Option<Box<Edit<DemoMutation>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ArtifactStoreBatchDigest<DemoMutation>>,RetainedCloneProgress)>,ValueError>{admit_artifact_batch_digest(edit,grant)}
    fn preflight(&self, mutation: &DemoMutation, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        self.preflights.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.inner.preflight(mutation, lane)
    }
    fn begin_demand(&self, mutation: &DemoMutation, lane: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> { self.inner.begin_demand(mutation, lane) }
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>)> {
        let base = request.base.get() as *const DemoSnapshot;
        let authority = Arc::as_ptr(&request.authority);
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| self.inner.begin(request, grant));
        match &result {
            Ok((_, progress)) => { assert!(progress.fits(grant.retained_grant())); assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes)); }
            Err((_, original)) => { assert_eq!(original.base.get() as *const DemoSnapshot, base); assert_eq!(Arc::as_ptr(&original.authority), authority); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); }
        }
        result
    }
}

#[semio_framework_async_macros::async_test]
async fn member_owned_batch_admission_defers_whole_source_census_and_retains_physical_backing() {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for count in fixture["census"]["rowCounts"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        let preflights = Arc::new(AtomicUsize::new(0));
        let mut member = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "bounded-source", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        let owners = complete_test_catalog(demo_base_store_owners().with_one_item_preparation(Arc::new(CountingOwnedPreparationFactory { inner: DemoOneItemPreparationFactory::admissible(), preflights: Arc::clone(&preflights) }),physical_test_close_grant()));
        member.install_document_store_owners_exact(owners).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        let values = (0..count).map(|index| DemoMutation::SetN(SetN { n: fixture["values"][index % 3].as_i64().unwrap() as i32 })).collect::<Vec<_>>();
        let backing = values.capacity() * std::mem::size_of::<DemoMutation>();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
        let (mutations, _) = MemberStoreOwnedBatch::try_new(values, grant).unwrap_or_else(|_| panic!("exact funded source"));
        let mut request = MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(717), expected_generation: member.generation_now(), expected_revision: member.content_revision_now(), actor: fixture["metadata"]["actor"].as_str().unwrap().into(), group_id: None, transaction: None, mutations };
        let mut publication = member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).expect("bounded source admission").expect("funded publication birth");
        assert_eq!(preflights.load(Ordering::SeqCst), 0, "admission cannot scan every source row before the first caller grant");
        assert_eq!(member.snapshot_ref().n, Some(0));
        assert!(matches!(member.prepare_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: 4096, ..one_item_test_policy() }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
        assert_eq!(preflights.load(Ordering::SeqCst), 0);
        for row in 0..count {
            let before = preflights.load(Ordering::SeqCst);
            assert!(matches!(member.prepare_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).unwrap(), ArtifactStoreOneItemPreparationStep::Progress(_, _)));
            assert_eq!(preflights.load(Ordering::SeqCst) - before, fixture["census"]["maximumPreflightsPerTurn"].as_u64().unwrap() as usize);
            assert_eq!(preflights.load(Ordering::SeqCst), row + 1);
            assert_eq!(member.snapshot_ref().n, Some(0));
        }
        assert!(matches!(member.prepare_one_item_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).unwrap(), ArtifactStoreOneItemPreparationStep::Progress(_, _)));
        assert_eq!(preflights.load(Ordering::SeqCst), count);
        publication.begin_close();
        assert!(!publication.terminal_is_empty());
        let mut released = 0;
        let close_grant = RetainedCloneGrant { maximum_release_bytes: fixture["census"]["maximumCloseBytes"].as_u64().unwrap() as usize, ..physical_test_close_grant() };
        for _ in 0..65_536 {
            if publication.terminal_is_empty() { break; }
            let demand = publication.retirement_demands(close_grant.maximum_copy_bytes).unwrap();
            assert!(demand.copy_bytes <= close_grant.maximum_copy_bytes && demand.capacity_bytes <= close_grant.maximum_capacity_bytes && demand.release_bytes <= close_grant.maximum_release_bytes && demand.depth <= close_grant.maximum_depth);
            if demand.release_bytes > 0 {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| publication.close_step(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..close_grant }).unwrap());
                assert_eq!(step.progress(), RetainedCloneProgress::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert!(!publication.terminal_is_empty());
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| publication.close_step(close_grant).unwrap());
            assert!(step.progress().fits(close_grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            released += step.progress().released_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { assert!(publication.terminal_is_empty()); }
        }
        assert!(publication.terminal_is_empty());
        assert!(released >= backing, "original source backing must be physically returned under a whole same-turn grant");
        for _ in 0..1024 { if request.mutations.terminal_is_empty() { break; } assert!(request.mutations.close_granted(grant).unwrap().progress().fits(grant)); }
        assert!(request.mutations.terminal_is_empty());
        for _ in 0..65_536 { if matches!(SpaceMember::close_owned_step(&mut member, physical_test_close_grant()).unwrap(), RetainedCloneStep::Complete(_)) { break; } }
        assert!(SpaceMember::close_owned_terminal_is_empty(&member));
    }
}

#[test]
fn member_open_partial_parse_and_initialization_owners_retire_exactly() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Observed<T: semio_framework_value::retirement::RetireOwned> {
        inner: semio_framework_value::retirement::controlled::ControlledRetirement<T>,
        count: Arc<AtomicUsize>,
        counted: bool,
    }
    impl<T: semio_framework_value::retirement::RetireOwned> ErasedSnapshotRetirement for Observed<T> {
        fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
            let depth = self.next_depth_demand()?;
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if grant.maximum_depth < depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "observed original owner depth is not funded")); }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - usize::from(!self.counted), ..grant };
            let step = self.inner.step(child)?;
            if self.inner.terminal_is_empty() && !self.counted { self.count.fetch_add(1, Ordering::SeqCst); self.counted = true; }
            Ok(step)
        }
        fn terminal_is_empty(&self) -> bool { self.counted && self.inner.terminal_is_empty() }
        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_copy_byte_demand() }
        fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.inner.next_capacity_byte_demand(body) }
        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_release_byte_demand() }
        fn next_depth_demand(&self) -> Result<usize, ValueError> { self.inner.next_depth_demand()?.checked_add(usize::from(!self.counted)).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "observed original owner depth overflow")) }
    }
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct SnapshotFactory(#[factory_child] Arc<AtomicUsize>);
    impl ArtifactOwnedValueRetirementFactory<DemoSnapshot> for SnapshotFactory {
        fn retirement_birth_bytes(&self, _: &DemoSnapshot) -> usize { std::mem::size_of::<Observed<DemoSnapshot>>() }
        fn retire_owned(&self, value: DemoSnapshot, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, DemoSnapshot)> {
            if !<DemoSnapshot as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported() { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original observed typed owner has no controlled authority"), value)); }
            admit_artifact_retirement(value, grant, |value| Observed { inner: semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_| panic!("pure supported original observed owner")), count: self.0.clone(), counted: false })
        }
    }
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct MutationFactory(#[factory_child] Arc<AtomicUsize>);
    impl ArtifactOwnedValueRetirementFactory<DemoMutation> for MutationFactory {
        fn retirement_birth_bytes(&self, _: &DemoMutation) -> usize { std::mem::size_of::<Observed<DemoMutation>>() }
        fn retire_owned(&self, value: DemoMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, DemoMutation)> {
            if !<DemoMutation as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported() { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original observed typed owner has no controlled authority"), value)); }
            admit_artifact_retirement(value, grant, |value| Observed { inner: semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_| panic!("pure supported original observed owner")), count: self.0.clone(), counted: false })
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/🚪️open/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["retention"].as_array().unwrap() {
        let snapshots = Arc::new(AtomicUsize::new(0));
        let mutations = Arc::new(AtomicUsize::new(0));
        let source_bytes = DocumentStoreOwners::<DemoSnapshot, DemoMutation>::source_birth_bytes::<DemoSnapshotRetirementFactory, SnapshotFactory, MutationFactory, ArtifactStoreCursorDisposer<DemoSnapshot, DemoMutation>>().unwrap();
        let (owners, progress) = DocumentStoreOwners::admit_source_constructor(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: source_bytes, maximum_depth: 1, ..Default::default() }, || (DemoSnapshotRetirementFactory, SnapshotFactory(snapshots.clone()), MutationFactory(mutations.clone()), ArtifactStoreCursorDisposer::new())).unwrap_or_else(|_| panic!("funded original observed catalog source"));
        assert_eq!(progress.retained_capacity_bytes, source_bytes);
        let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).unwrap();
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(&[1, 97, 83]).unwrap()).unwrap();
        pages.seal().unwrap();
        let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "member-retained".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
        let request = MemberOpenRequest::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), 1000, expected, None, pages, ActorId(fixture["openedActor"].as_str().unwrap().into())).admit(1).unwrap_or_else(|_| panic!("admitted request fixture"));
        let mut retained = member_open::MemberStoreOpenRetained::new(request, owners);
        let stage = row["stage"].as_str().unwrap();
        if stage != "input" {
            let history = semio_framework_os_spr::HistoryLog {
                doc_id: "member-retained".into(),
                schema: "demo/v1".into(),
                edits: [semio_framework_os_spr::HistoryEdit {
                    line: None,
                    id: "raw-edit".into(),
                    actor: Some("raw-actor".into()),
                    started_at: "started".into(),
                    finished_at: Some("finished".into()),
                    verb: None,
                    ops: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(vec![0xff; 129]) }],
                    inverse: Vec::new().into(),
                    meta: Some(vec![semio_framework_os_spr::HistoryOpMeta {
                        dependencies: vec!["dependency".into()],
                        payload_hash: Some([3; 32]),
                        origin: semio_framework_os_spr::MutationOrigin::Transaction { initiator: semio_framework_os_spr::ForeignTarget { artifact_id: "initiator".into(), artifact_kind: "s.test.member".into(), dialect: Some("1/*".into()) } },
                        ..Default::default()
                    }]),
                    lane: None,
                }].into_iter().collect(),
                composition: Some(semio_framework_os_spr::HistoryComposition { owner: Some(("parent".into(), "slot".into(), "member-retained".into())), dialect: Some(("s.test.member".into(), "1".into(), "*".into())) }),
                ..Default::default()
            };
            retained.stage_history(history).unwrap_or_else(|_| panic!("raw decoded model stays owned before typed hydration"));
            let packed = DemoSnapshot { n: Some(7) }.encode_pack();
            let initial = DemoSnapshot::decode_pack(&packed).unwrap();
            if matches!(stage, "envelope" | "initialization") {
                let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-retained", initial, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).into_owners();
                retained.stage_envelope(envelope).unwrap_or_else(|_| panic!("unique parsed owner slot"));
                if stage == "initialization" {
                    let current = DemoSnapshot::decode_pack(&packed).unwrap();
                    let runtime = ArtifactStoreInitializationRuntime::new("member-retained", "demo/v1", Arc::new(current), [7; 32], ActorId(fixture["openedActor"].as_str().unwrap().into()));
                    retained.stage_runtime(runtime).unwrap_or_else(|_| panic!("unique initialized owner slot"));
                }
            } else {
                retained.stage_initial(initial).unwrap_or_else(|_| panic!("unique typed snapshot slot"));
            }
        }
        if matches!(stage, "forward" | "inverse") {
            let encoded = DemoMutation::SetN(SetN { n: 7 }).encode_op().unwrap();
            let forwards = vec![DemoMutation::decode_op(&encoded).unwrap()];
            let inverse = if stage == "inverse" { vec![DemoMutation::decode_op(&encoded).unwrap()] } else { Vec::new() };
            retained
                .stage_edit(Edit {
                    line: None,
                    id: "pending-edit".into(),
                    actor: Some("actor".into()),
                    forwards,
                    inverse: inverse.into(),
                    mutation_meta: Vec::new(),
                    verb: None,
                    sequence_number: 1,
                    started_at: "now".into(),
                    finished_at: None,
                })
                .unwrap_or_else(|_| panic!("unique partial edit slot"));
            assert!(DemoMutation::decode_op(&[0xff]).is_err(), "next malformed operation must not consume the retained edit");
        }
        let before = retained.retained_typed_owners();
        if row["cancelled"].as_bool().unwrap() {
            let cancel = semio_framework_job::root_cancel_token();
            cancel.cancel_now();
            let mut sequence = 0;
            let mut original_retained_progress_7=semio_framework_value::RetainedCloneProgress::default();
        let cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, 999, schema_test_close_grant()), cancel, || Some(1), &mut sequence, &mut original_retained_progress_7);
            assert_eq!(retained.check_step_authority(&cx), Err(MemberOpenDiagnostic::Cancelled));
        } else {
            retained.reject(if stage == "initialization" { MemberOpenDiagnostic::Initialization } else { MemberOpenDiagnostic::Malformed });
        }
        assert_eq!(retained.retained_input_bytes(), 3);
        assert_eq!(retained.retained_typed_owners(), before);
        assert_eq!(retained.close_step(RetainedCloneGrant::default()).unwrap().progress(), RetainedCloneProgress::default());
        assert_eq!(retained.retained_typed_owners(), before);
        assert_eq!((snapshots.load(Ordering::SeqCst), mutations.load(Ordering::SeqCst)), (0, 0));
        let demand_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
        let admission = demand_fixture["admissionBytes"].as_u64().unwrap() as usize;
        let mut turns = 0;
        let mut released = 0;
        let mut demands = std::collections::BTreeSet::new();
        while !retained.terminal_is_empty() {
            let copy_bytes = retained.next_copy_byte_demand().unwrap();
            let demand = semio_framework_value::RetirementDemand { copy_bytes, capacity_bytes: retained.next_capacity_byte_demand(copy_bytes).unwrap(), release_bytes: retained.next_release_byte_demand().unwrap(), depth: retained.next_depth_demand().unwrap() };
            assert_eq!(retained.next_release_byte_demand().unwrap(), demand.release_bytes, "inquiry must retain the current phase");
            let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() };
            assert!(demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth, "member-open actual frontier exceeds independently declared admission policy");
            demands.insert((demand.copy_bytes, demand.capacity_bytes, demand.release_bytes, demand.depth));
            assert_eq!(retained.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!(retained.next_release_byte_demand().unwrap(), demand.release_bytes);
            let step = retained.close_step(grant).unwrap();
            assert!(step.progress().fits(grant));
            released += step.progress().released_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { assert!(retained.terminal_is_empty()); }
            turns += 1;
            assert!(turns < 10_000, "member-open exact caller grants must converge for {}: demands={demands:?}", row["id"]);
        }
        assert_eq!(snapshots.load(Ordering::SeqCst), row["snapshots"].as_u64().unwrap() as usize, "{}", row["id"]);
        assert_eq!(mutations.load(Ordering::SeqCst), row["mutations"].as_u64().unwrap() as usize, "{}", row["id"]);
    }
}


#[semio_framework_async_macros::async_test]
async fn retained_member_publication_rejects_wrong_owner_staleness_and_cancels_without_commit() {
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() };
    let mut member = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "same-id", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    let mut wrong = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "same-id", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    member.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    wrong.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    member.closes_on_drop();
    wrong.closes_on_drop();
    let request = |generation, revision| MemberStoreOneItemWireRequest {
        operation: semio_framework_job::OperationId(800),
        expected_generation: generation,
        expected_revision: revision,
        actor: "member-test".into(),
        group_id: Some("cancel-group".into()),
        wire: MemberStoreOneItemWire { schema: "demo.member.json-number".into(), bytes: b"123".to_vec() },
    };
    let missing = request(wrong.generation(), wrong.content_revision_now());
    let original = missing.wire.bytes.as_ptr();
    let rejected = wrong.begin_one_item_wire_publication(missing).err().expect("missing member factory fails closed");
    assert_eq!(rejected.mutations.first().expect("rejection returns its exact admitted owners").bytes.as_ptr(), original, "rejection returns the exact input owner");
    let mut stale = member.begin_one_item_wire_publication(request(member.generation(), member.content_revision_now())).unwrap_or_else(|_| panic!("owned wire admits"));
    assert!(wrong.advance_one_item_publication(&mut *stale, grant).is_err(), "matching schema/id/revision cannot impersonate the exact store owner");
    assert_eq!(stale.progress().completed_bytes, 0);
    let mut replacement = member.begin_member_apply_batch(semio_framework_job::OperationId(801), member.generation(), member.content_revision_now(), "member-test".into(), vec![DemoMutation::SetN(SetN { n: 9 })]).unwrap();
    for _ in 0..262_144 {
        if matches!(member.advance_apply_batch(&mut replacement, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)) {
            break;
        }
    }
    assert!(replacement.acknowledge());
    close_durable_publication(&mut replacement);
    assert!(member.advance_one_item_publication(&mut *stale, grant).is_err());
    close_erased_member_publication(&mut *stale, grant);
    for steps in [0, 1, 3, 5] {
        let before = member.one_item_publication_identity();
        let mut cancelled = member.begin_one_item_wire_publication(request(before.0, before.1)).unwrap_or_else(|_| panic!("cancel owner admits"));
        for _ in 0..steps {
            assert!(!matches!(member.advance_one_item_publication(&mut *cancelled, grant).unwrap(), ArtifactStoreOneItemAdvance::Published(_)));
        }
        cancelled.begin_close();
        close_erased_member_publication(&mut *cancelled, grant);
        assert_eq!(member.one_item_publication_identity(), before);
        assert_eq!(member.snapshot().unwrap().n, Some(9));
    }
    close_demo_artifact_store(&mut member);
    close_demo_artifact_store(&mut wrong);
}

#[semio_framework_async_macros::async_test]
async fn retained_member_group_preparation_reserves_real_history_without_partial_visibility_and_aborts_stale_owners() {
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() };
    let mut members = [
        ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "group-first", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await,
        ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "group-second", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await,
    ];
    for member in &mut members {
        member.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        member.closes_on_drop();
    }
    let roots = [members[0].snapshot_root(), members[1].snapshot_root()];
    let before = [members[0].one_item_publication_identity(), members[1].one_item_publication_identity()];
    let mut publications = Vec::new();
    for (index, member) in members.iter().enumerate() {
        publications.push(
            member
                .begin_one_item_wire_publication(MemberStoreOneItemWireRequest {
                    operation: semio_framework_job::OperationId(900),
                    expected_generation: before[index].0,
                    expected_revision: before[index].1,
                    actor: "group-test".into(),
                    group_id: Some("reserved-group".into()),
                    wire: MemberStoreOneItemWire { schema: "demo.member.json-number".into(), bytes: if index == 0 { b"17".to_vec() } else { b"23".to_vec() } },
                })
                .unwrap_or_else(|_| panic!("group member wire admits")),
        );
    }
    for index in 0..members.len() {
        let mut reserved = false;
        for _ in 0..262_144 {
            let step = members[index].prepare_one_item_publication(&mut *publications[index], grant).expect("one group preparation or reservation turn");
            for other in 0..members.len() {
                assert_eq!(members[other].one_item_publication_identity(), before[other]);
                assert!(members[other].envelope().vcs.edits.is_empty());
                assert!(members[other].applied_edit_ids().is_empty());
                assert_eq!(members[other].snapshot().unwrap().n, Some(0));
                assert!(Arc::ptr_eq(&roots[other], &members[other].snapshot_root()));
            }
            if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(_, _)) {
                reserved = true;
                break;
            }
        }
        assert!(reserved, "member must reach real history and retirement reservation under maximum grant");
        assert!(members[index].advance_one_item_publication(&mut *publications[index], grant).is_err(), "ordinary publication cannot consume a group-reserved candidate");
    }
    let mut competing = members[0].begin_member_apply_batch(semio_framework_job::OperationId(901), before[0].0, before[0].1, "other-test".into(), vec![DemoMutation::SetN(SetN { n: 99 })]).unwrap();
    for _ in 0..262_144 {
        if competing.phase() == ArtifactStoreOneItemPublicationPhase::Publishing {
            break;
        }
        members[0].advance_apply_batch(&mut competing, grant).unwrap();
    }
    assert!(members[0].advance_apply_batch(&mut competing, grant).is_err(), "the real reserved edit slot excludes a competing append before any visible mutation");
    close_durable_publication(&mut competing);
    members[1].invalidate_after_external_resource_change().unwrap();
    assert!(members[1].prepare_one_item_publication(&mut *publications[1], grant).is_err(), "freshness is revalidated even after reservation");
    assert!(members[1].abort_one_item_publication(&mut *publications[0], grant.retained_grant()).is_err(), "only the exact member can release its reservation");
    for index in 0..members.len() {
        publications[index].begin_close();
        assert_eq!(publications[index].close_step(grant.retained_grant()).unwrap(), RetainedCloneStep::Progress(Default::default()), "generic close cannot silently abandon live member reservations");
        for _ in 0..65_536 {
            if matches!(members[index].abort_one_item_publication(&mut *publications[index], grant.retained_grant()).unwrap(), RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(publications[index].terminal_is_empty());
        assert_eq!(members[index].snapshot().unwrap().n, Some(0));
        assert!(members[index].envelope().vcs.edits.is_empty());
        let reservation = members[index].reserve_edit_history_slot().expect("aborted group returns the exact ledger reservation");
        assert!(members[index].envelope.vcs.edits.cancel_reservation(reservation.history).is_ok());
        members[index].displaced_retirements.release_owner_slots(reservation.rejected_owner).unwrap();
    }
    drop(roots);
    for member in &mut members {
        close_demo_artifact_store(member);
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_member_common_root_copies_history_under_grant_and_publishes_three_lanes_once() {
    const IDENTITY_CEILING: usize = 4096 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/📬️publication/🤝️group/🧪️tests/🧫️fixtures/📚️history-cases/🔣️.json")).unwrap();
    let grant = ArtifactStoreOneItemGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: fixture["maximumBytes"].as_u64().unwrap() as usize, ..one_item_test_policy() };
    for row in fixture["cases"].as_array().unwrap() {
        for commit in [false, true] {
            let history = row["historyEntries"].as_u64().unwrap() as usize;
            let mut members = [
                ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "group-parent", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await,
                ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "group-child", DemoSnapshot { n: Some(10) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await,
                ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "group-existing", DemoSnapshot { n: Some(100) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await,
            ];
            for (lane, member) in members.iter_mut().enumerate() {
                member.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
                for entry in 1..=history {
                    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: [0, 10, 100][lane] + entry as i32 })], transaction: None }, &mut identity).await.unwrap();
                }
            }
            let before = members.each_ref().map(|member| member.one_item_publication_identity());
            let roots = members.each_ref().map(|member| member.snapshot_root());
            let mut publications: Vec<Box<dyn ErasedMemberStoreOneItemPublication>> = members.iter().enumerate().map(|(lane, member)| {
                member.begin_one_item_wire_publication(MemberStoreOneItemWireRequest {
                    operation: semio_framework_job::OperationId(917), expected_generation: before[lane].0, expected_revision: before[lane].1, actor: "actor:common-root".into(), group_id: Some("7".repeat(64)),
                    wire: MemberStoreOneItemWire { schema: "demo.member.json-number".into(), bytes: row["after"][lane].as_i64().unwrap().to_string().into_bytes() },
                }).unwrap_or_else(|_| panic!("exact retained group input admits"))
            }).collect();
            for lane in 0..3 {
                let mut prepared = false;
                for _ in 0..262_144 {
                    if matches!(members[lane].prepare_one_item_publication(&mut *publications[lane], grant).unwrap(), ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { prepared = true; break; }
                }
                assert!(prepared);
            }
            let mut decision = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
            let visibility = decision.view();
            let mut turns = [0usize; 3];
            for lane in 0..3 {
                assert_eq!(members[lane].stage_one_item_publication(&mut *publications[lane], &visibility, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_copy_bytes: 0, ..one_item_test_policy() }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked);
                let mut staged = false;
                let mut completed = publications[lane].progress().completed_items;
                for _ in 0..65_536 {
                    assert!(publications[lane].next_group_byte_demand() <= grant.maximum_copy_bytes);
                    let step = members[lane].stage_one_item_publication(&mut *publications[lane], &visibility, grant).unwrap();
                    turns[lane] += 1;
                    let checkpoint = publications[lane].progress();
                    assert!(checkpoint.completed_items >= completed);
                    completed = checkpoint.completed_items;
                    for other in 0..3 {
                        assert_eq!(members[other].one_item_publication_identity(), before[other]);
                        assert_eq!(members[other].snapshot().unwrap().n, Some(row["before"][other].as_i64().unwrap() as i32));
                        assert_eq!(members[other].applied_edit_ids().len(), history);
                        assert_eq!(members[other].envelope().vcs.edits.len(), history);
                        assert!(Arc::ptr_eq(&roots[other], &members[other].snapshot_root()));
                    }
                    if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { staged = true; break; }
                }
                assert!(staged);
                assert!(turns[lane] >= history.saturating_mul(3));
            }
            let mut prepared_reads = Vec::new();
            for lane in 0..3 {
                for denied in [ArtifactStoreOneItemGrant { maximum_items: 0, ..grant }, ArtifactStoreOneItemGrant { maximum_depth: ErasedSnapshotRead::prepared_read_demands().depth - 1, ..grant }] {
                    let (read, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| members[lane].snapshot_read_erased_for_publication(&mut *publications[lane], &visibility, denied).unwrap());
                    assert!(read.is_none()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert!(Arc::ptr_eq(&roots[lane], &members[lane].snapshot_root()));
                }
                assert!(members[(lane + 1) % 3].snapshot_read_erased_for_publication(&mut *publications[lane], &visibility, grant).is_err());
                let (prepared, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| members[lane].snapshot_read_erased_for_publication(&mut *publications[lane], &visibility, grant).unwrap().unwrap());
                let mut prepared = prepared;
                assert_eq!(prepared.3, RetainedCloneProgress { copied_items: 1, ..Default::default() }); assert_eq!((heap.requested_bytes, heap.released_bytes), (prepared.3.retained_capacity_bytes, prepared.3.released_bytes));
                assert_eq!(prepared.0, before[lane].0 + 1);
                assert_ne!(prepared.1, before[lane].1);
                assert_eq!(prepared.2.get::<DemoSnapshot>().unwrap().n, Some(row["after"][lane].as_i64().unwrap() as i32));
                let (projection,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||members[lane].child_restore_projection_for_read(&prepared.2));
                assert!(projection.unwrap().is_empty());
                assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                assert!(members[(lane+1)%3].child_restore_projection_for_read(&prepared.2).is_err());
                let original=prepared.2.owner.replace(Arc::new(17u8));
                assert!(members[lane].child_restore_projection_for_read(&prepared.2).is_err());
                prepared.2.owner=original;
                assert_eq!(members[lane].one_item_publication_identity(), before[lane]);
                prepared_reads.push(prepared);
            }
            if commit {
                assert!(decision.commit());
                for lane in 0..3 {
                    assert_eq!(members[lane].snapshot().unwrap().n, Some(row["after"][lane].as_i64().unwrap() as i32));
                    assert_eq!(members[lane].one_item_publication_identity().0, before[lane].0 + 1);
                    assert_eq!(members[lane].envelope().vcs.edits.len(), history + 1);
                    assert_eq!(members[lane].applied_edit_ids().len(), history + 1);
                    assert_eq!(members[lane].one_item_publication_identity(), (prepared_reads[lane].0, prepared_reads[lane].1));
                }
                for lane in 0..3 {
                    let (adopted, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                        for _ in 0..128 {
                            if matches!(members[lane].adopt_one_item_publication(&mut *publications[lane], &visibility, grant).unwrap(), ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { return true; }
                        }
                        false
                    });
                    assert!(adopted);
                    assert_eq!(heap.requested_bytes, fixture["decisionRequestedBytes"].as_u64().unwrap() as usize);
                    assert_eq!(heap.released_bytes, fixture["decisionReleasedBytes"].as_u64().unwrap() as usize);
                    assert!(publications[lane].acknowledge());
                    close_erased_member_publication(&mut *publications[lane], grant);
                }
            } else {
                for lane in (0..3).rev() {
                    let (_, _, snapshot, receipt) = prepared_reads.pop().unwrap();
                    assert_eq!(receipt, RetainedCloneProgress { copied_items: 1, ..Default::default() });
                    let rejected = members[(lane + 1) % 3].return_prepared_snapshot_read_erased(&mut *publications[lane], &visibility, snapshot, grant).unwrap_err();
                    assert_eq!(rejected.1.get::<DemoSnapshot>().unwrap().n, Some(row["after"][lane].as_i64().unwrap() as i32));
                    let rejected = members[lane].return_prepared_snapshot_read_erased(&mut *publications[lane], &visibility, rejected.1, ArtifactStoreOneItemGrant { maximum_items: 0, ..grant }).unwrap_err();
                    assert_eq!(rejected.0.kind,semio_framework_value::ValueRefusalKind::WorkLimit);
                    let rejected = members[lane].return_prepared_snapshot_read_erased(&mut *publications[lane], &visibility, rejected.1, ArtifactStoreOneItemGrant { maximum_depth: 0, ..grant }).unwrap_err();
                    assert_eq!(rejected.0.kind,semio_framework_value::ValueRefusalKind::DepthLimit);
                    assert_eq!(rejected.1.get::<DemoSnapshot>().unwrap().n, Some(row["after"][lane].as_i64().unwrap() as i32));
                    assert_eq!(members[lane].one_item_publication_identity(), before[lane]);
                    let returned = members[lane].return_prepared_snapshot_read_erased(&mut *publications[lane], &visibility, rejected.1, ArtifactStoreOneItemGrant { maximum_release_bytes: ErasedSnapshotRead::LEASE_ALLOCATION_BYTES, ..grant }).unwrap_or_else(|_| panic!("funded exact private lease return"));
                    assert_eq!(returned, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: ErasedSnapshotRead::LEASE_ALLOCATION_BYTES, ..Default::default() }));
                    let (returned_retirement, _) = members[lane].take_returned_snapshot_read_retirement(grant.retained_grant()).unwrap();
                    if let Some(mut retirement) = returned_retirement {
                        for _ in 0..4096 {
                            if retirement.terminal_is_empty() { break; }
                            let body = retirement.next_copy_byte_demand().unwrap();
                            let step_grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: body, maximum_capacity_bytes: retirement.next_capacity_byte_demand(body).unwrap(), maximum_release_bytes: retirement.next_release_byte_demand().unwrap(), maximum_depth: retirement.next_depth_demand().unwrap() };
                            retirement.close_step(step_grant).unwrap();
                        }
                        assert!(retirement.terminal_is_empty());
                    }
                    assert!(members[lane].snapshot_read_leases_terminal_is_empty());
                }
                assert!(decision.abort());
                for lane in 0..3 {
                    for _ in 0..65_536 {
                        if matches!(members[lane].abort_one_item_publication(&mut *publications[lane], grant.retained_grant()).unwrap(), RetainedCloneStep::Complete(_)) { break; }
                    }
                    assert!(publications[lane].terminal_is_empty());
                    assert_eq!(members[lane].one_item_publication_identity(), before[lane]);
                    assert_eq!(members[lane].snapshot().unwrap().n, Some(row["before"][lane].as_i64().unwrap() as i32));
                }
            }
            drop(prepared_reads);
            drop(roots);
            drop(publications);
            for member in &mut members {
                let mut closed = false;
                for _ in 0..262_144 {
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| member.close_owned_step(grant.retained_grant()).unwrap());
                    assert!(step.progress().fits(grant.retained_grant())); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                    if matches!(step, RetainedCloneStep::Complete(_)) { closed = true; break; }
                }
                assert!(closed && member.close_owned_terminal_is_empty());
            }
        }
    }
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}


#[test]
fn factory_preborn_tickets_retire_sole_snapshot_weak_without_losing_upgrade_witness() {
    use semio_framework_value::FactoryRetirement;
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../🔨️modules/🌱️value/♻️retirement/🏭️factory/🧪️tests/🧫️fixtures/🔣️.json"))).unwrap();
    let row = fixture["soleSnapshotWeak"].as_array().unwrap().iter().find(|row| row["pointerBytes"].as_u64().unwrap() as usize == std::mem::size_of::<usize>()).unwrap();
    assert_eq!(std::mem::size_of::<DemoSnapshot>(), row["snapshotBytes"].as_u64().unwrap() as usize);
    assert_eq!(demo_snapshot_frame_bytes(), row["frameBytes"].as_u64().unwrap() as usize);
    let grant=physical_test_close_grant();
    for live in [false, true] {
        let mut snapshot = Some(Arc::new(DemoSnapshot { n: Some(7) }));
        let witness = Arc::new(DemoPublishedRootWitness::new());
        witness.publish(snapshot.as_ref().unwrap());
        let upgraded = witness.lock().unwrap().as_ref().and_then(std::sync::Weak::upgrade).unwrap();
        assert!(Arc::ptr_eq(&upgraded, snapshot.as_ref().unwrap()));
        drop(upgraded);
        if !live {
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(snapshot.take()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let birth = witness.factory_retirement_birth_bytes();
        let ((ticket,receipt), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| witness.clone().preborn_factory_retirement(grant).unwrap_or_else(|error|panic!("funded witness birth refused: {}",error.error)));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (birth, 0));assert_eq!(receipt.retained_capacity_bytes,birth);assert!(receipt.fits(grant));
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(witness));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut slot = Some(ticket);
        let mut released = 0;
        for _ in 0..16 {
            let Some(owner) = slot.as_ref() else { break; };
            let demand=semio_framework_value::factory_ticket_demands(owner,grant.maximum_copy_bytes).unwrap();
            if demand.release_bytes!=0 {
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::close_factory_ticket(&mut slot,RetainedCloneGrant { maximum_release_bytes:demand.release_bytes-1,..grant }));
                assert_eq!(step.unwrap().progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::close_factory_ticket(&mut slot,grant).unwrap());
            assert!(step.progress().fits(grant));let physical=step.progress().released_bytes;
            assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,physical));
            released += physical;
        }
        assert!(slot.is_none());
        let witness_frame = std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<DemoPublishedRootWitness>()).unwrap().0.pad_to_align().size();
        assert_eq!(released, witness_frame + birth + if live { 0 } else { demo_snapshot_frame_bytes() });
        if let Some(snapshot) = snapshot {
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(snapshot));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, demo_snapshot_frame_bytes()));
        }
    }
}


#[test]
fn member_demo_preparation_source_prices_original_issuer_frames_and_paid_terminal_closure() {
    let grant=physical_test_close_grant();
    let source_bytes=semio_framework_value::factory_arc_birth_bytes::<DemoOneItemPreparationFactory>()+semio_framework_value::factory_arc_birth_bytes::<DemoPublishedRootWitness>()+semio_framework_value::factory_arc_birth_bytes::<DemoSnapshotRetirementFactory>()+semio_framework_value::factory_arc_birth_bytes::<DemoMutationRetirementFactory>();
    let birth=semio_framework_value::retained_clone::RetainedCloneBirthDemand {capacity_bytes:source_bytes,depth:2}.admit(grant).expect("independent source policy admits all original factory frames before construction");
    let (factory,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Arc::new(DemoOneItemPreparationFactory::admissible()));
    assert!(birth.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(source_bytes,0));
    let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;let mut owner=semio_framework_value::FactoryAuthority::new(factory);let mut released=0;
    for _ in 0..128 {
        if owner.terminal_is_empty(){break;}
        let demand=owner.demands(grant.maximum_copy_bytes).unwrap();
        if demand.release_bytes!=0 {
            let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(RetainedCloneGrant {maximum_release_bytes:demand.release_bytes-1,..grant}).unwrap());
            assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        }
        let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());
        assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));released+=step.progress().released_bytes;
    }
    assert!(owner.terminal_is_empty());assert!(released>=source_bytes);
}


#[semio_framework_async_macros::async_test]
async fn derived_child_reads_require_the_exact_store_frontier_and_genuine_operation_or_history_fold() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪆️derived-read/🔣️.json")).expect("closed language-neutral origin corpus");
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "derived-source", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let mut foreign = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "derived-foreign", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    foreign.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let base = store.derived_snapshot_head();
    let(snapshot_owner,supplier)=original_snapshot_ports_without_representation_bounds(&store);
    assert!(Arc::ptr_eq(base.snapshot_owner(),&snapshot_owner));assert!(Arc::ptr_eq(supplier.unwrap(),store.owned_snapshot_retirement_factory().unwrap()));drop(snapshot_owner);
    let operation = DemoMutation::SetN(SetN { n: 9 });
    let derived = store.derive_provisional_snapshot(&base, &operation).expect("genuine source").expect("applicable operation");
    <DemoMutation as Mutation<DemoSnapshot>>::retire_cold(operation);
    assert_eq!(derived.snapshot().n, Some(9));
    assert_eq!(store.snapshot_ref().n, Some(0));
    assert!(foreign.snapshot_read_derived(&derived).is_err(), "same concrete type and event numbers do not grant another Store authority");
    let read = store.snapshot_read_derived(&derived).expect("exact Store source");
    let actual = read.get::<DemoSnapshot>().expect("complete concrete derived projection");
    assert!(std::ptr::eq(actual, derived.snapshot()));
    assert_eq!(actual.n, Some(9));
    let portable: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(derived.snapshot())).expect("own portable JSON output");
    assert_eq!(portable, fixture["derived"]);
    assert_eq!(portable, serde_json::to_value(derived.snapshot()).expect("independent complete Serde output"));
    drop(read);
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 6 })], transaction: None }, &mut identity).await.expect("actual event publication");
    assert!(store.snapshot_read_derived(&derived).is_err(), "a prior derived source cannot be relabelled with the new frontier");
    drive_retirement_terminal(admit_demo_alias_retirement(&store,derived.into_snapshot_owner()));
    drive_retirement_terminal(admit_demo_alias_retirement(&store,base.into_snapshot_owner()));
    let mut replay = Some(store.begin_derived_report_replay(protocol::HistoryInputDrafts::new(), None).expect("genuine history replay"));
    let owner = replay.as_ref().expect("retained replay") as *const _;
    let plan = replay.as_ref().unwrap().plan.as_ref().unwrap() as *const _;
    let registry = replay.as_ref().unwrap().registry.identity();
    assert!(store.finish_derived_report_replay(&mut replay).is_err(), "unfinished admission retains its actual owner");
    assert_eq!(owner, replay.as_ref().expect("unfinished replay retained") as *const _);
    assert_eq!(plan, replay.as_ref().unwrap().plan.as_ref().unwrap() as *const _);
    assert_eq!(registry, replay.as_ref().unwrap().registry.identity());
    loop {
        let demand=replay.as_ref().unwrap().planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).unwrap();
        let grant=physical_test_close_grant();
        assert!(demand.copy_bytes<=grant.maximum_copy_bytes && demand.capacity_bytes<=grant.maximum_capacity_bytes && demand.release_bytes<=grant.maximum_release_bytes && demand.depth<=grant.maximum_depth);
        if matches!(store.step_derived_report_replay(replay.as_mut().expect("retained replay"), grant, &mut || false).expect("own actual ledger"),ReplayStep::Finished(_)) {break;}
    }
    assert!(foreign.finish_derived_report_replay(&mut replay).is_err(), "foreign finishing cannot consume the source owner");
    assert_eq!(owner, replay.as_ref().expect("foreign-refused replay retained") as *const _);
    assert_eq!(registry, replay.as_ref().unwrap().registry.identity());
    assert_eq!(replay.as_ref().unwrap().replay.as_ref().unwrap().state.as_ref().unwrap().n, Some(6));
    let (result, head) = store.finish_derived_report_replay(&mut replay).expect("sealed completed replay");
    assert!(replay.is_none());
    let head = head.expect("complete actual history projection");
    assert_eq!(head.snapshot().n, Some(6));
    let portable: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(head.snapshot())).expect("own portable history JSON output");
    assert_eq!(portable, fixture["published"]);
    assert_eq!(portable, serde_json::to_value(head.snapshot()).expect("independent complete history Serde output"));
    assert!(Arc::ptr_eq(head.snapshot_owner(), result.state().expect("same replay result")));
    assert!(foreign.snapshot_read_derived(&head).is_err());
    let mut result = result;
    let state = result.take_state().expect("the replay result still aliases its reached projection");
    result.retire_unscheduled().expect("the inspected replay retires from its own quoted demands");
    drive_retirement_terminal(admit_demo_alias_retirement(&store,state));
    drive_retirement_terminal(admit_demo_alias_retirement(&store,head.into_snapshot_owner()));
    let mut stale = Some(store.begin_derived_report_replay(protocol::HistoryInputDrafts::new(), None).expect("second actual replay"));
    let owner = stale.as_ref().expect("retained stale replay") as *const _;
    let plan = stale.as_ref().unwrap().plan.as_ref().unwrap() as *const _;
    let registry = stale.as_ref().unwrap().registry.identity();
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }, &mut identity).await.expect("new actual frontier");
    assert!(store.finish_derived_report_replay(&mut stale).is_err(), "stale admission cannot consume the actual replay owner");
    assert_eq!(owner, stale.as_ref().expect("stale replay retained") as *const _);
    assert_eq!(plan, stale.as_ref().unwrap().plan.as_ref().unwrap() as *const _);
    assert_eq!(registry, stale.as_ref().unwrap().registry.identity());
    assert!(stale.as_ref().unwrap().replay.is_none(), "stale planning never begins projection work");
    drop(stale);
    close_demo_artifact_store(&mut store);
    close_demo_artifact_store(&mut foreign);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn apply_undo_redo_transfers_each_snapshot_root_to_one_exact_retirement_owner() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let genesis = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "exact-tail-transfer", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(complete_test_catalog(DocumentStoreOwners::admit_source_constructor(physical_test_close_grant(), || (ExactDemoSnapshotRetirementFactory(Arc::clone(&completed)), ExactDemoInitialSnapshotRetirementFactory(Arc::clone(&genesis)), DemoMutationRetirementFactory, ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new())))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo");
    store.dispatch(ArtifactCommand::Redo, &mut identity).await.expect("redo");
    assert_eq!(store.snapshot_ref().n, Some(7));
    close_demo_artifact_store(&mut store);
    assert_eq!(completed.load(std::sync::atomic::Ordering::SeqCst), 2, "post-apply and post-redo allocations retire exactly once");
    assert_eq!(genesis.load(std::sync::atomic::Ordering::SeqCst), 1, "the retained pre-redo tail is the envelope's exact genesis allocation");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🪪️ Drains one operation wire source through the production prepared-operation cursor into its exact bytes.
fn drain_operation_wire(source: prepared_operation_wire::ArtifactPreparedOperationSource<'_>) -> Vec<u8> {
    let mut cursor = prepared_operation_wire::ArtifactPreparedOperationCursor::default();
    let mut bytes = Vec::new();
    for _ in 0..1 << 20 {
        let mut span = [0u8; 64];
        let capacity = cursor.next_capacity_byte_demand().expect("operation wire capacity demand");
        let step = cursor.advance(source, &mut span, RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: capacity, maximum_release_bytes: 0, maximum_depth: canonical_edit::ARTIFACT_CANONICAL_JSON_DEPTH }).unwrap_or_else(|error| panic!("operation wire turn refused: {}", error.reason));
        bytes.extend_from_slice(&span[..step.written_bytes]);
        if step.complete { break; }
    }
    let demand = cursor.retirement_demands().expect("operation wire retirement demand");
    cursor.close(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1), ..Default::default() }).expect("operation wire cursor closes");
    bytes
}

#[test]
fn the_canonical_operation_wire_of_every_demo_leaf_is_the_header_and_the_independent_serde_document() {
    let semantic = prepared_operation_wire::ArtifactOperationWire::<DemoMutation>::semantic::<DemoSnapshot>();
    let schemaless = prepared_operation_wire::ArtifactOperationWire::<DemoMutation>::canonical();
    for (mutation, kind) in [
        (DemoMutation::SetN(SetN { n: 7 }), "set-n"),
        (DemoMutation::AddN(AddN { delta: -3 }), "add-n"),
        (DemoMutation::DeleteN(DeleteN {}), "delete-n"),
        (DemoMutation::AssignN(AssignN { n: Some(2) }), "assign-n"),
        (DemoMutation::AssignN(AssignN { n: None }), "assign-n"),
    ] {
        let mut expected = b"op".to_vec();
        expected.extend_from_slice(&serde_json::to_vec(&mutation).expect("serde oracle"));
        assert_eq!(drain_operation_wire(semantic.source(&mutation).expect("a Tree mutation has a canonical wire")), expected, "{mutation:?}");
        assert_eq!(drain_operation_wire(schemaless.source(&mutation).expect("a Tree mutation has a canonical wire")), expected, "{mutation:?}");
        assert_eq!(semantic.schema(&mutation), Some(("n", kind)), "{mutation:?}");
        assert_eq!(schemaless.schema(&mutation), None);
    }
}

#[semio_framework_async_macros::async_test]
async fn a_store_without_a_catalog_cannot_author_and_a_catalog_alone_gives_it_the_canonical_wire() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity: semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("original authoring identity");
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "catalog-wire", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    assert!(store.operation_wire().is_none(), "a bare store holds no wire");
    let refusal = store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect_err("a store that received neither a catalog nor a wire cannot author");
    assert!(format!("{refusal:?}").contains("authoring requires an installed canonical operation wire or an original domain codec"), "{refusal:?}");
    store.install_document_store_owners_exact(demo_base_store_owners()).unwrap_or_else(|(error, _)| panic!("funded base catalog install refused: {error}"));
    assert!(store.operation_wire().is_some(), "the catalog carries the canonical wire of its mutation vocabulary");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("a catalog store authors through the wire its catalog carried");
    let (forward_rows, _) = store.authored_edit_operation_bytes(&mut identity).expect("rows author").expect("the applied edit has rows");
    let mut expected = b"op".to_vec();
    expected.extend_from_slice(&serde_json::to_vec(&DemoMutation::SetN(SetN { n: 3 })).expect("serde oracle"));
    assert_eq!(forward_rows, vec![expected]);
    close_demo_artifact_store(&mut store);
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn a_store_with_no_preparation_factory_authors_apply_undo_redo_and_its_rows_through_its_installed_canonical_wire() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity: semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("original authoring identity");
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "canonical-wire", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_operation_wire(prepared_operation_wire::ArtifactOperationWire::semantic::<DemoSnapshot>());
    store.install_document_store_owners_exact(demo_base_store_owners()).unwrap_or_else(|(error, _)| panic!("funded base catalog install refused: {error}"));
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }, &mut identity).await.expect("apply authors through the canonical wire");
    let (forward_wires, inverse_wires) = store.authored_edit_operation_bytes(&mut identity).expect("rows author").expect("the applied edit has rows");
    let (forwards, inverses, meta) = store.edit_mutations().expect("the applied edit");
    let oracle = |rows: Vec<&DemoMutation>| rows.into_iter().map(|row| { let mut bytes = b"op".to_vec(); bytes.extend_from_slice(&serde_json::to_vec(row).expect("serde oracle")); bytes }).collect::<Vec<_>>();
    assert_eq!(forward_wires, oracle(forwards.iter().collect()));
    assert_eq!(inverse_wires, oracle((0..inverses.len()).map(|index| inverses.get(index).expect("inverse row")).collect()));
    assert_eq!(forward_wires.len(), 1);
    assert!(meta[0].payload_hash.is_some() && meta[0].mutation_id.is_some(), "the authored identity and payload hash are minted from the wire");
    store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo");
    assert_eq!(store.snapshot_ref().n, Some(0));
    store.dispatch(ArtifactCommand::Redo, &mut identity).await.expect("redo");
    assert_eq!(store.snapshot_ref().n, Some(7));
    close_demo_artifact_store(&mut store);
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn store_close_waits_for_a_live_snapshot_read_then_retires_its_returned_owner() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "live-reader-close", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None }, &mut identity).await.expect("apply");
    let read = store.snapshot_read().expect("exact live snapshot read");
    let mut blocked = false;
    for _ in 0..4_096 {
        let grant = RetainedCloneGrant { maximum_copy_bytes: 512, ..physical_test_close_grant() };
        match SpaceMember::close_owned_step(&mut store, grant).expect("store close advances to the reader boundary") {
            RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default() => { blocked = true; break; }
            RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant)),
            RetainedCloneStep::Complete(_) => panic!("a live snapshot read must retain its exact root"),
        }
    }
    assert!(blocked, "store close observes the live reader before detaching its current root");
    drop(read);
    close_demo_artifact_store(&mut store);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[test]
fn returned_snapshot_read_releases_an_alias_before_the_displaced_root_and_retires_a_unique_fallback() {
    let grant=physical_test_close_grant();
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let canonical = Arc::new(DemoSnapshot { n: Some(1) });
    let factory=ExactDemoSnapshotRetirementFactory(Arc::clone(&completed));
    let(mut displaced,progress)=factory.retire(Arc::clone(&canonical),grant).expect("counted original cursor birth is funded");
    assert_eq!(progress.retained_capacity_bytes,std::mem::size_of::<ExactDemoSnapshotRetirement>());
    assert_eq!(displaced.close_step(grant).expect("displaced root retains original external alias").progress(),RetainedCloneProgress::default());
    let mut returned = ReturnedSnapshotReadRetirement::new(canonical, Arc::new(DemoInitialSnapshotRetirementFactory));
    let first=returned.close_step(grant).expect("returned read releases only its original alias");
    assert_eq!(first.progress(),RetainedCloneProgress { copied_items: 1,..Default::default() });
    assert!(!returned.terminal_is_empty(),"original factory capability remains separately owned");
    for _ in 0..32 {if returned.terminal_is_empty(){break;}let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||returned.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
    assert!(returned.terminal_is_empty());drop(returned);
    drive_retirement_terminal(displaced);
    assert_eq!(completed.load(std::sync::atomic::Ordering::SeqCst), 1);
    let(unique,progress)=ReturnedSnapshotReadRetirement::admit(Arc::new(DemoSnapshot { n: Some(2) }),Arc::new(DemoInitialSnapshotRetirementFactory),grant).unwrap_or_else(|(error,_,_)|panic!("unique original shared cursor birth refused: {error}"));
    assert_eq!(progress.retained_capacity_bytes,ReturnedSnapshotReadRetirement::<DemoSnapshot>::constructor_capacity_bytes());
    drive_retirement_terminal(unique);
}


#[semio_framework_async_macros::async_test]
async fn store_close_releases_a_returned_read_before_its_displaced_root() {
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let genesis = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "returned-before-displaced", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(fixture_authoring_catalog(physical_test_close_grant(), ExactDemoSnapshotRetirementFactory(Arc::clone(&completed)), ExactDemoInitialSnapshotRetirementFactory(Arc::clone(&genesis)), DemoMutationRetirementFactory)).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let read = store.snapshot_read().expect("read captures the pre-edit root");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None }).await.expect("apply displaces the captured root");
    drop(read);
    close_demo_artifact_store(&mut store);
    assert_eq!(completed.load(std::sync::atomic::Ordering::SeqCst), 1, "the edited current root retires exactly once");
    assert_eq!(genesis.load(std::sync::atomic::Ordering::SeqCst), 1, "the displaced pre-edit root is the envelope's exact retained genesis allocation");
}

#[semio_framework_async_macros::async_test]
async fn erased_member_snapshot_read_releases_its_alias_before_the_live_current_root() {
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let genesis = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "erased-live-reader", DemoSnapshot { n: Some(3) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(fixture_authoring_catalog(physical_test_close_grant(), ExactDemoSnapshotRetirementFactory(Arc::clone(&completed)), ExactDemoInitialSnapshotRetirementFactory(Arc::clone(&genesis)), DemoMutationRetirementFactory)).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let grant = physical_test_close_grant();
    let mut read = Some(SpaceMember::snapshot_read_erased_now(&store).expect("erased member read"));
    let (retirement, admission) = SpaceMember::retire_snapshot_read_erased(&mut store, &mut read, grant).expect("erased read admission is funded by its grant");
    assert!(read.is_none() && admission.fits(grant));
    let mut retirement = retirement.expect("admitted erased read retirement");
    let step = retirement.close_step(grant).expect("erased read releases its alias");
    assert!(step.progress().fits(grant) && step.progress().copied_items == 1);
    assert!(!retirement.terminal_is_empty(),"actual factory capability remains separately owned after alias release");
    drive_retirement_terminal(retirement);
    close_demo_artifact_store(&mut store);
    assert_eq!(completed.load(std::sync::atomic::Ordering::SeqCst), 0, "the initial current alias owns no distinct snapshot allocation");
    assert_eq!(genesis.load(std::sync::atomic::Ordering::SeqCst), 1, "the envelope retires the exact initial allocation once");
}

#[semio_framework_async_macros::async_test]
async fn retained_genesis_aliases_retire_each_decoded_allocation_once() {
    let text = include_str!("../../🧫️fixtures/🌱️retained-genesis-aliases/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(text).expect("independent fixture oracle");
    let neutral = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("first-party neutral fixture");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&semio_framework_pack_json::to_dsl_value(&neutral))).unwrap(), fixture);
    for case in fixture["cases"].as_array().unwrap() {
        let snapshots = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let genesis = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", case["name"].as_str().unwrap(), DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        store.install_document_store_owners_exact(fixture_authoring_catalog(physical_test_close_grant(), ExactDemoSnapshotRetirementFactory(Arc::clone(&snapshots)), ExactDemoInitialSnapshotRetirementFactory(Arc::clone(&genesis)), DemoMutationRetirementFactory)).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        let commands = case["commands"].as_array().unwrap();
        let mut tail = None;
        for (index, command) in commands.iter().enumerate() {
            if index + 1 == commands.len() && case["tailReader"].as_bool().unwrap_or(false) {
                tail = Some(store.snapshot_read().expect("registered external reader of the root the last edit displaces"));
            }
            let command = match command.as_str().unwrap() { "apply" => ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }, "applyNext" => ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 8 })], transaction: None }, "undo" => ArtifactCommand::Undo, "redo" => ArtifactCommand::Redo, other => panic!("unknown neutral command {other}") };
            test_support::dispatch_test_command(&mut store, command).await.expect("neutral command executes");
        }
        assert_eq!(serde_json::to_value(store.snapshot_ref()).unwrap(), case["final"]);
        let read = case["liveReader"].as_bool().unwrap_or(false).then(|| store.snapshot_read().expect("registered external reader"));
        if read.is_some() || tail.is_some() {
            let blocked = close_stalls_at_reader_boundary(&mut store);
            assert!(blocked);
            assert!(!SpaceMember::close_owned_terminal_is_empty(&store));
            assert_eq!(genesis.load(std::sync::atomic::Ordering::SeqCst), 0);
        }
        drop(read);
        drop(tail);
        close_demo_artifact_store(&mut store);
        assert_eq!(snapshots.load(std::sync::atomic::Ordering::SeqCst) as u64, case["snapshotRetirements"].as_u64().unwrap());
        assert_eq!(genesis.load(std::sync::atomic::Ordering::SeqCst) as u64, case["genesisRetirements"].as_u64().unwrap());
    }
}

/// 🚧️ Drives a store close under the authored physical policy until a funded turn reaches its live external reader boundary.
fn close_stalls_at_reader_boundary(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>) -> bool {
    let copy = physical_test_close_grant().maximum_copy_bytes;
    for _ in 0..16_384 {
        store.close_owned_demands(copy).expect("current independent Store close frontier");
        let grant = physical_test_close_grant();
        match store.close_owned_step(grant).expect("store close advances to the reader boundary") {
            RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default() => {  return true },
            RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant)),
            RetainedCloneStep::Complete(_) => panic!("a live snapshot read must retain its exact root"),
        }
    }
    false
}


fn original_snapshot_ports_without_representation_bounds<P,M>(store:&semio_framework_os_store_engine::lifted_store::ArtifactStore<P,M>)->(Arc<P>,Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,ValueError>)
where P:Clone+ToValue+FromValue+Send+Sync+'static,M:Clone+ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+'static,
{(store.snapshot_owner(),store.owned_snapshot_retirement_factory())}
