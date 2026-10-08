//! ♻️ Original typed catalog owners conserve actual independent constructor and physical release receipts.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};

#[test]
fn bounded_value_retirement_preserves_original_owner_and_physical_receipts_under_every_copy_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️bounded-value-retirement/🔣️.json")).unwrap();
    let body = fixture["body"].as_str().unwrap().repeat(fixture["repeat"].as_u64().unwrap() as usize);
    for case in fixture["cases"].as_array().unwrap() {
        let copy = case["maximumCopyBytes"].as_u64().unwrap() as usize;
        for shared in [false, true] {
            let factory = BoundedArtifactRetirementFactory::<String>::new();
            let owner = body.clone();
            let pointer = owner.as_ptr();
            let held = owner.capacity();
            let (mut slot, mut born) = if shared {
                let owner = Arc::new(owner);
                let original = Arc::as_ptr(&owner);
                let capacity = SnapshotRetirementFactory::retirement_birth_bytes(&factory, &owner);
                let grant = RetainedCloneGrant::one_capacity_turn(capacity, 1);
                let (refusal, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| SnapshotRetirementFactory::retire(&factory, owner, RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..grant }));
                let (_, owner) = match refusal { Err(refusal) => refusal, Ok(_) => panic!("whole constructor undergrant must retain original shared owner") };
                assert_eq!(Arc::as_ptr(&owner), original);
                assert_eq!(owner.as_ptr(), pointer);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| SnapshotRetirementFactory::retire(&factory, owner, grant));
                let (cursor, progress) = match result { Ok(admitted) => admitted, Err(_) => panic!("exact original shared constructor") };
                assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
                (Some(cursor), progress.retained_capacity_bytes)
            } else {
                let capacity = ArtifactOwnedValueRetirementFactory::retirement_birth_bytes(&factory, &owner);
                let grant = RetainedCloneGrant::one_capacity_turn(capacity, 1);
                let (refusal, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| factory.retire_owned(owner, RetainedCloneGrant { maximum_items: 0, ..grant }));
                let (_, owner) = match refusal { Err(refusal) => refusal, Ok(_) => panic!("zero item must retain original owned value") };
                assert_eq!(owner.as_ptr(), pointer);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| factory.retire_owned(owner, grant));
                let (cursor, progress) = match result { Ok(admitted) => admitted, Err(_) => panic!("exact original typed constructor") };
                assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
                (Some(cursor), progress.retained_capacity_bytes)
            };
            let mut physical = 0;
            let mut turns = 0;
            while let Some(cursor) = slot.as_ref() {
                let demand = artifact_retirement_box_demands(cursor, copy).unwrap();
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
                for refusal in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant })].into_iter().flatten() {
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| artifact_retirement_box_close_step(&mut slot, refusal));
                    assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| artifact_retirement_box_close_step(&mut slot, grant));
                let progress = step.unwrap().progress();
                assert!(progress.fits(grant));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
                born += progress.retained_capacity_bytes;
                physical += progress.released_bytes;
                turns += 1;
                assert!(turns <= 32768);
            }
            assert!(physical >= held + born);
            eprintln!("[DEBUG] original bounded catalog shared={shared} copy={copy} held={held} actual birth={born} physical={physical} turns={turns} no fabricated page debt");
        }
    }
}

#[test]
fn original_vcs_retirement_preserves_real_history_visibility_genesis_and_full_physical_receipts() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["vcsRetirement"];
    let forwards: Vec<u64> = serde_json::from_value(law["forwards"].clone()).unwrap();
    let inverse: Vec<u64> = serde_json::from_value(law["inverse"].clone()).unwrap();
    for with_edit in [false, true] {
        let (vcs, born, released) = crate::test_allocation::observe_backing(|| {
            let mut pack = Vec::<u8>::with_capacity(law["packCapacity"].as_u64().unwrap() as usize);
            pack.extend(law["pack"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8));
            let genesis = AdmittedArtifactGenesis::from_decoded_pack(law["snapshot"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize), pack);
            let mut edits = ArtifactHistoryLedger::new();
            if with_edit {
                let mut reverse = semio_framework_value::list::PagedList::new();
                for value in &inverse { reverse.try_push(*value).unwrap(); }
                let edit = Edit { id: law["edit"].as_str().unwrap().to_owned(), actor: Some(law["actor"].as_str().unwrap().to_owned()), line: None, forwards: forwards.clone(), inverse: reverse, mutation_meta: Vec::with_capacity(3), verb: None, sequence_number: 1, started_at: "started".into(), finished_at: Some("finished".into()) };
                let mut publisher = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
                let view = publisher.view();
                let reservation = edits.reserve_group_one(&view).unwrap();
                edits.stage_group_reserved(reservation, edit, &view).unwrap_or_else(|_| panic!("original edit suffix"));
                assert!(publisher.commit());
                edits.adopt_group(&view).unwrap();
            }
            ArtifactVcs { genesis, edits, changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() }
        });
        assert_eq!(released, 0);
        let snapshot_pointer = vcs.genesis.facts().snapshot().as_ptr();
        let initial: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(BoundedArtifactRetirementFactory::<String>::new());
        let mutation: Arc<dyn ArtifactOwnedValueRetirementFactory<u64>> = Arc::new(BoundedArtifactRetirementFactory::<u64>::new());
        let factory_bytes = semio_framework_value::factory_arc_birth_bytes::<BoundedArtifactRetirementFactory<String>>() + semio_framework_value::factory_arc_birth_bytes::<BoundedArtifactRetirementFactory<u64>>();
        let (mut owner, allocated, released) = crate::test_allocation::observe_backing(|| ArtifactStoreVcsRetirement::new(vcs, initial, mutation));
        assert_eq!((allocated, released), (0, law["constructorDropBytes"].as_u64().unwrap() as usize));
        assert_eq!(owner.vcs.as_ref().unwrap().genesis.facts().snapshot().as_ptr(), snapshot_pointer);
        if with_edit { assert_eq!(owner.vcs.as_ref().unwrap().edits.last().unwrap().forwards, forwards); }
        let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
        let mut during = 0;
        let mut physical = 0;
        let mut turns = 0;
        while !owner.terminal_is_empty() {
            let demand = owner.demands(copy).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            for denial in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }), (demand.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant })].into_iter().flatten() {
                let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(denial));
                assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
                assert_eq!((allocated, released), (0, 0));
            }
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(grant));
            let progress = step.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
            during += allocated;
            physical += released;
            turns += 1;
            assert!(turns <= 32768);
        }
        assert_eq!(physical, born + during + factory_bytes);
        let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(owner));
        assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
        eprintln!("[DEBUG] original VCS edit={with_edit} same snapshot pointer/full-u64 mutation rows; exact native history, visibility, Pack and factory physical={physical} System births={} turns={turns} terminalDrop0heap", born + during + factory_bytes);
    }
}

#[test]
fn original_displaced_queue_preserves_each_child_receipt_and_retains_maintenance_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["displacedQueue"];
    let (mut queue, born, released) = crate::test_allocation::observe_backing(|| {
        let mut queue = ArtifactStoreDisplacedRetirements::new();
        let value = law["body"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize);
        queue.push_reserved(Box::new(ArtifactStoreStringRetirement::new(value)));
        queue
    });
    assert_eq!(released, 0);
    let capacity = queue.owners.capacity();
    let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
    let mut physical = 0;
    let mut during = 0;
    let mut turns = 0;
    while !queue.terminal_is_empty() {
        let demand = queue.demands(copy).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        for denied in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (demand.release_bytes > 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant })].into_iter().flatten() {
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| queue.close_step(denied));
            assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((allocated, released), (0, 0));
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| queue.close_step(grant));
        let progress = step.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
        physical += released;
        during += allocated;
        assert_eq!(queue.owners.capacity(), capacity);
        turns += 1;
        assert!(turns < 32768);
    }
    let reservation = queue.reserve_owner_slots(1).unwrap();
    let zero = RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 0 };
    assert_eq!(queue.close_step(zero).unwrap().progress(), RetainedCloneProgress::default());
    assert!(!queue.terminal_is_empty());
    let refused = queue.demands(copy).unwrap_err();
    assert_eq!(refused.kind, semio_framework_value::ValueRefusalKind::WorkLimit);
    queue.release_owner_slots(reservation).unwrap();
    let bytes = queue.backing_release_byte_demand().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: bytes, maximum_depth: 1 };
    let (step, allocated, released) = crate::test_allocation::observe_backing(|| queue.release_empty_backing_step(RetainedCloneGrant { maximum_release_bytes: bytes - 1, ..grant }));
    assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
    assert_eq!((allocated, released), (0, 0));
    let (step, allocated, released) = crate::test_allocation::observe_backing(|| queue.release_empty_backing_step(grant));
    assert_eq!(step.unwrap(), RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
    assert_eq!((allocated, released), (0, bytes));
    physical += released;
    assert_eq!(physical, born + during);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(queue));
    assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] original displaced queue turns={turns} physical={physical} same fixed maintenance backing and full terminal child receipts");
}

#[test]
fn original_sparse_lane_authority_matches_sorted_oracle_and_retains_original_physical_backing() {
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["sparseLaneAuthority"];
    let oracle: BTreeMap<&str, &str> = law["entries"].as_array().unwrap().iter().map(|row| (row[0].as_str().unwrap(), row[1].as_str().unwrap())).collect();
    let (mut lanes, allocated, released) = crate::test_allocation::observe_backing(|| {
        let mut lanes = protocol::HistoryFoldIndex::new();
        for row in law["entries"].as_array().unwrap() {
            let lane = match row[1].as_str().unwrap() { "document" => HistoryLane::Document, "interaction" => HistoryLane::Interaction, _ => panic!("declared lane fixture") };
            lanes.insert(row[0].as_str().unwrap().to_owned(), lane);
        }
        lanes
    });
    let original = allocated - released;
    assert_eq!(lanes.len(), oracle.len());
    for ((actual, lane), (expected, value)) in lanes.iter().zip(oracle.iter()) {
        assert_eq!(actual.as_str(), *expected);
        assert_eq!(*lane, if *value == "document" { HistoryLane::Document } else { HistoryLane::Interaction });
    }
    let mut rows: [Option<ControlledRetirement<(String, HistoryLane)>>; 3] = std::array::from_fn(|_| None);
    for (index, expected) in law["tailOrder"].as_array().unwrap().iter().enumerate() {
        let pointer = lanes.iter().next_back().unwrap().0.as_ptr();
        let (entry, allocated, released) = crate::test_allocation::observe_backing(|| lanes.pop_last().unwrap());
        assert_eq!((allocated, released), (law["popBirthBytes"].as_u64().unwrap() as usize, law["popReleaseBytes"].as_u64().unwrap() as usize));
        assert_eq!(entry.0.as_ptr(), pointer);
        assert_eq!(entry.0, expected.as_str().unwrap());
        rows[index] = Some(ControlledRetirement::new(entry).unwrap_or_else(|_| panic!("original lane row declares retirement")));
    }
    assert!(lanes.is_empty());
    assert!(!lanes.terminal_is_empty());
    let mut index = ControlledRetirement::new(lanes).unwrap_or_else(|_| panic!("original index declares retirement"));
    let mut born = 0;
    let mut physical = 0;
    for cursor in std::iter::once(&mut index as &mut dyn ErasedSnapshotRetirement).chain(rows.iter_mut().map(|row| row.as_mut().unwrap() as &mut dyn ErasedSnapshotRetirement)) {
        let mut turns = 0;
        while !cursor.terminal_is_empty() {
            let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: cursor.next_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: cursor.next_release_byte_demand().unwrap(), maximum_depth: cursor.next_depth_demand().unwrap() };
            for denial in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (grant.maximum_release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant })].into_iter().flatten() {
                let (step, allocated, released) = crate::test_allocation::observe_backing(|| cursor.close_step(denial));
                assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
                assert_eq!((allocated, released), (0, 0));
            }
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| cursor.close_step(grant));
            let progress = step.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
            born += allocated;
            physical += released;
            turns += 1;
            assert!(turns < 32768);
        }
    }
    assert_eq!(physical, original + born);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop((index, rows)));
    assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] original sparse lane index same sorted facts/tail owners, source={original} birth={born} full physical={physical} finalDrop0");
}

#[test]
fn original_repository_history_entry_close_conserves_raw_box_value_and_factory_custody() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["repositoryHistoryEntry"];
    let body = law["integer"].as_str().unwrap();
    let expected: u64 = serde_json::from_str(body).unwrap();
    let actual: u64 = semio_framework_pack_json::from_json_str(body, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(actual, expected);
    let (mut owner, allocated, released) = crate::test_allocation::observe_backing(|| {
        let factory: Arc<dyn ArtifactOwnedValueRetirementFactory<u64>> = Arc::new(BoundedArtifactRetirementFactory::<u64>::new());
        let mut owner = ArtifactRepositoryHistoryEntryAuthority::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), OwnedSchemaPath::ROOT, factory);
        *owner.value = Some(actual);
        owner
    });
    let original = allocated - released;
    let raw = owner.raw.as_ref().unwrap().as_ptr();
    let factory = Arc::as_ptr(owner.retirement_factory.as_ref().unwrap());
    let mut born = 0;
    let mut physical = 0;
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
        let demand = owner.close_demands(copy).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        for denied in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }), (demand.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant })].into_iter().flatten() {
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(denied));
            assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((allocated, released), (0, 0));
            if let Some(held) = owner.raw.as_ref() { assert_eq!(held.as_ptr(), raw); }
            if let Some(held) = owner.retirement_factory.as_ref() { assert!(std::ptr::addr_eq(Arc::as_ptr(held), factory)); }
            assert_eq!(owner.close_demands(copy).unwrap(), demand);
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(grant));
        let step = step.unwrap();
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); }
        born += allocated;
        physical += released;
        turns += 1;
        assert!(turns < 16384);
    }
    assert_eq!(physical, original + born);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(owner));
    assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] original repository history entry full-u64={actual} raw+factory source={original} births={born} exact physical={physical} turns={turns} terminalDrop0");
}

#[test]
fn original_schema_history_array_close_releases_same_decoder_ledger_and_factory_roots() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["repositoryHistoryEntry"];
    let (mut owner, allocated, released) = crate::test_allocation::observe_backing(|| {
        let factory: Arc<dyn ArtifactOwnedValueRetirementFactory<u64>> = Arc::new(BoundedArtifactRetirementFactory::<u64>::new());
        let decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<u64>> = Arc::new(ArtifactRepositoryHistoryEntryDecoder::<u64>::new());
        let mut owner = OwnedSchemaBoundedArrayAuthority::new(OwnedSchemaPath::ROOT, factory, decoder);
        *owner.active_decoder = Some(owner.decoder.as_ref().unwrap().begin_entry(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), OwnedSchemaPath::ROOT, Arc::clone(owner.retirement_factory.as_ref().unwrap())));
        owner
    });
    let original = allocated - released;
    let decoder = owner.active_decoder.as_ref().unwrap().as_ref() as *const dyn ArtifactOwnedHistoryEntryAuthority<u64>;
    let mut born = 0;
    let mut physical = 0;
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
        let demand = owner.close_demands(copy).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        for denied in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }), (demand.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant })].into_iter().flatten() {
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(denied));
            assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((allocated, released), (0, 0));
            if let Some(held) = owner.active_decoder.as_ref() { assert!(std::ptr::addr_eq(held.as_ref() as *const dyn ArtifactOwnedHistoryEntryAuthority<u64>, decoder)); }
            assert_eq!(owner.close_demands(copy).unwrap(), demand);
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(grant));
        let step = step.unwrap();
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); }
        born += allocated;
        physical += released;
        turns += 1;
        assert!(turns < 32768);
    }
    assert_eq!(physical, original + born);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(owner));
    assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] original schema array same decoder+ledger+factory original={original} born={born} physical={physical} turns={turns} terminalDrop0");
}

#[test]
fn original_retained_member_open_close_conserves_supplied_birth_and_physical_release() {
    use super::tests::DemoSnapshot;
    use super::fixture_mutations::demo::DemoMutation;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧩️composition/🚪️open/📏️retirement/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["retainedMemberClose"];
    let demand = bounded_artifact_store_owners_birth_demand::<DemoSnapshot, DemoMutation>().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: demand.capacity_bytes, maximum_depth: demand.depth, ..Default::default() };
    let (mut owner, allocated, released) = crate::test_allocation::observe_backing(|| {
        let (owners, receipt) = bounded_artifact_store_owners::<DemoSnapshot, DemoMutation>(grant).unwrap_or_else(|_| panic!("exact original catalog source ingress"));
        assert!(receipt.fits(grant));
        assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
        let input = law["input"].as_str().unwrap().as_bytes();
        let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: input.len() }).unwrap();
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(input).unwrap()).unwrap_or_else(|_| panic!("original input page"));
        pages.seal().unwrap();
        let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: law["artifactId"].as_str().unwrap().to_owned(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "demo".to_owned(), standard: "1".to_owned(), subset: "any".to_owned() } };
        let request = member_open::MemberOpenRequest::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), u64::MAX, target, None, pages, crate::os_spr::ActorId(law["actor"].as_str().unwrap().to_owned()));
        let mut owner = member_open::MemberStoreOpenRetained::new(request, owners);
        owner.stage_initial(DemoSnapshot::default()).unwrap_or_else(|_| panic!("original pending snapshot"));
        owner
    });
    let original = allocated - released;
    let mut born = 0;
    let mut physical = 0;
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy.max(owner.next_copy_byte_demand().unwrap()), maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: owner.next_depth_demand().unwrap() };
        for denied in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (grant.maximum_release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant }), (grant.maximum_capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: grant.maximum_capacity_bytes - 1, ..grant })].into_iter().flatten() {
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(denied));
            assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((allocated, released), (0, 0));
            assert_eq!(owner.next_release_byte_demand().unwrap(), grant.maximum_release_bytes);
            assert_eq!(owner.next_capacity_byte_demand(copy).unwrap(), grant.maximum_capacity_bytes);
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(grant));
        let step = step.unwrap();
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); }
        born += allocated;
        physical += released;
        turns += 1;
        assert!(turns < 32768);
    }
    assert_eq!(physical, original + born);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(owner));
    assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] original retained member-open source={original} funded cursor births={born} full physical={physical} turns={turns} terminalDrop0");
}

#[test]
fn original_operation_rows_close_conserves_last_factory_and_retired_string_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["repositoryHistoryEntry"];
    let body = law["integer"].as_str().unwrap();
    let actual: u64 = semio_framework_pack_json::from_json_str(body, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(actual, serde_json::from_str::<u64>(body).unwrap());
    let (mut owner, allocated, released) = crate::test_allocation::observe_backing(|| {
        let factory: Arc<dyn ArtifactOwnedValueRetirementFactory<u64>> = Arc::new(BoundedArtifactRetirementFactory::<u64>::new());
        ArtifactStoreOperationRowsRetirement::new(vec![actual], factory).with_identity(body.to_owned()).with_strings(vec![body.to_owned()])
    });
    let original = allocated - released;
    let mut born = 0;
    let mut physical = 0;
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize;
        let demand = owner.demands(copy).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        for denied in [Some(RetainedCloneGrant { maximum_items: 0, ..grant }), (demand.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }), (demand.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant })].into_iter().flatten() {
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(denied));
            assert_eq!(step.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((allocated, released), (0, 0));
            assert_eq!(owner.demands(copy).unwrap(), demand);
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| owner.close_step(grant));
        let step = step.unwrap();
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); }
        born += allocated;
        physical += released;
        turns += 1;
        assert!(turns < 32768);
    }
    assert_eq!(physical, original + born);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(owner));
    assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] original operation rows same last factory+identity+retired String Vec source={original} births={born} full physical={physical} turns={turns} terminalDrop0");
}
