
use super::*;
use crate::mutations::{add_layer_asset, change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, create_layer, delete_layer, move_layer, remove_layer_asset, rename_layer, reorder_layers, resize_layer};
use crate::standards::v1::subsets::any::schema::empty_raster_document;
use crate::{RASTER_DOCUMENT_SCHEMA, RasterLayerNode, RasterTransform};

static RASTER_INITIALIZER_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
static RASTER_STANDALONE_RETIREMENT_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let document = empty_raster_document();
    let operation = RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
        parent_id: None,
        index: document.layers.len(),
        layer: Box::new(RasterLayerNode::Pixel {
            id: "op-binary-test".into(),
            name: "Op Binary Test".into(),
            visible: true,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: RasterTransform::default(),
            mask: None,
            width: Some(64),
            height: Some(64),
            image_key: None,
        }),
    });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).await.expect("encode");
    assert_eq!(decode_op(&bytes).await.expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn raster_document_text_round_trips_store_with_applied_operation() {
    use crate::RasterSnapshot;

    let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "doc-text-test", empty_raster_document(), None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    store
        .dispatch(store::ArtifactCommand::Apply {
            mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                parent_id: None,
                index: 1,
                layer: Box::new(RasterLayerNode::Adjustment {
                    id: "adjust-text".into(),
                    name: "Levels".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    adjustment_kind: "levels".into(),
                    params: RasterOwnedMap::new(),
                }),
            })],
            description: None,
        })
        .await
        .expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}

fn empty_raster_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> RasterStoreInitializationAuthority {
    let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-retained-load", empty_raster_document(), None);
    RasterStoreInitializationAuthority::new(envelope, operation, generation)
}

fn drive_raster_initializer(authority: &mut RasterStoreInitializationAuthority, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::StepOutcome {
    let _guard = RASTER_INITIALIZER_TEST_LOCK.lock().expect("Raster initializer test lock");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(authority, &mut context);
        if outcome.is_terminal() {
            return outcome;
        }
    }
    panic!("Raster retained initializer did not reach a bounded terminal")
}

fn close_raster_candidate(mut candidate: store::ArtifactStore<RasterSnapshot, RasterMutation>) {
    use semio_framework_plugin::ArtifactOwnedDisposer;

    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<RasterSnapshot, RasterMutation>::new();
    for _ in 0..100_000 {
        match disposer.close_step(&mut candidate, 1, RASTER_OWNED_FIELD_BYTES).expect("Raster candidate close step") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            }
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("fresh Raster candidate close unexpectedly awaited input: {reason}"),
            semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("fresh Raster candidate close unexpectedly blocked: {reason}"),
            semio_framework_plugin::PluginCloseStep::Complete => {
                assert!(disposer.terminal_is_empty(&candidate));
                drop(disposer);
                drop(candidate);
                return;
            }
        }
    }
    panic!("Raster candidate did not reach terminal-empty close")
}

fn close_raster_retirement(retirement: &mut dyn store::ErasedSnapshotRetirement) {
    for _ in 0..200_000 {
        if retirement.terminal_is_empty() {
            return;
        }
        let step = retirement.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("Raster retained owner closes after admitted saturation resumes");
        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
        }
    }
    panic!("Raster retained owner did not reach terminal-empty");
}

#[test]
fn raster_store_initializer_publishes_next_generation_and_candidate_closes_incrementally() {
    let operation = semio_framework_job::OperationId(701);
    let generation = semio_framework_job::Generation(31);
    let mut authority = empty_raster_initializer(operation, generation);
    assert!(matches!(drive_raster_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Raster candidate");
    assert_eq!(candidate.generation_now(), 32);
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_raster_candidate(candidate);
}

#[test]
fn raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
    let operation = semio_framework_job::OperationId(702);
    let generation = semio_framework_job::Generation(33);
    let mut cancelled = empty_raster_initializer(operation, generation);
    semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
    assert!(matches!(drive_raster_initializer(&mut cancelled, operation, generation), semio_framework_job::StepOutcome::Cancelled));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
    drop(cancelled);

    let mut stale = empty_raster_initializer(operation, generation);
    assert!(matches!(drive_raster_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1)), semio_framework_job::StepOutcome::Fault(_)));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
}

#[test]
fn raster_store_initializer_zero_budget_advances_no_owner_or_phase() {
    let operation = semio_framework_job::OperationId(703);
    let generation = semio_framework_job::Generation(35);
    let mut authority = empty_raster_initializer(operation, generation);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(0, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
    assert!(matches!(semio_framework_plugin::ArtifactStoreInitializationAuthority::step(&mut authority, &mut context), semio_framework_job::StepOutcome::Yield));
    assert_eq!(authority.phase, RasterStoreInitializationPhase::ValidateEnvelope);
    assert!(authority.envelope.is_some());
    semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut authority);
    assert!(matches!(drive_raster_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Cancelled));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
}

fn deeply_nested_raster_snapshot(depth: usize) -> RasterSnapshot {
    let mut layer = RasterLayerNode::Pixel { id: "leaf".into(), name: "Leaf".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None };
    for index in 0..depth {
        layer = RasterLayerNode::Group { id: format!("group-{index}"), name: format!("Group {index}"), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![layer] };
    }
    let mut snapshot = empty_raster_document();
    snapshot.layers.clear();
    snapshot.layers.push(layer);
    snapshot
}

#[test]
fn raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel() {
    let source = deeply_nested_raster_snapshot(48);
    let operation = semio_framework_job::OperationId(704);
    let generation = semio_framework_job::Generation(36);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut clone = RasterSnapshotCloneAuthority::new();
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"raster.low-fuel");
    let mut turns = 0;
    while !clone.terminal {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        assert!(!clone.step(&source, &mut digest, &mut context).expect("bounded Raster clone") || clone.terminal);
        turns += 1;
        assert!(turns < 20_000);
    }
    assert!(turns > 48, "nested clone must resume across the recursive layer depth");
    let candidate = clone.take_value().expect("bounded clone candidate");
    drop(clone);
    assert_eq!(candidate, source);
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(candidate));
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
        let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("bounded candidate retirement");
        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
        }
    }
    drop(retirement);
}

#[test]
fn raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page() {
    let _guard = RASTER_INITIALIZER_TEST_LOCK.lock().expect("mounted Raster initializer test lock");
    assert_eq!(RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0);
    let operation = semio_framework_job::OperationId(7_041);
    let generation = semio_framework_job::Generation(361);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let empty = empty_raster_document();
    let mut bounds = RasterSnapshotBoundsAuthority::new();
    for _ in 0..256 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        if bounds.step(&empty, &mut context).expect("empty Raster bounds remain admissible") {
            break;
        }
    }
    assert!(bounds.terminal);
    assert!(bounds.totals.source_bytes < RASTER_MAXIMUM_NESTED_BYTES);
    assert_eq!(bounds.totals.source_control_items, RASTER_MAXIMUM_CONTROL_BACKINGS);
    assert_eq!(bounds.totals.source_control_bytes, RASTER_MAXIMUM_CONTROL_BYTES);

    let mut source = empty_raster_document();
    source.layers.push(RasterLayerNode::Pixel {
        id: "mounted-layer".into(),
        name: "Mounted".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        mask: None,
        width: Some(1),
        height: Some(1),
        image_key: None,
    });
    for index in 0..(crate::RASTER_OWNED_MAP_PAGE_CAPACITY + 1) {
        source
            .assets
            .insert(
                format!("mounted-{index}"),
                store::ArtifactChild::new(
                    format!("child-{index}"),
                    store::os_io::ArtifactRef { artifact_id: format!("artifact-{index}"), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect("mounted-shaped source admits its second fixed map page");
    }
    let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-mounted-64-fuel", source, None);
    let mut authority = RasterStoreInitializationAuthority::new(envelope, operation, generation);
    let mut terminal = None;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(64, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(&mut authority, &mut context);
        if outcome.is_terminal() {
            terminal = Some(outcome);
            break;
        }
    }
    assert!(matches!(terminal, Some(semio_framework_job::StepOutcome::Complete(_))));
    assert_eq!(RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "normal completion returns every non-stack process control credit");
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("mounted-shaped candidate");
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_raster_candidate(candidate);
}

#[test]
fn raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner() {
    fn expired_now() -> Option<u64> {
        Some(10)
    }
    let source = deeply_nested_raster_snapshot(8);
    let operation = semio_framework_job::OperationId(705);
    let generation = semio_framework_job::Generation(37);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut clone = RasterSnapshotCloneAuthority::new();
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"raster.expired");
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, 10), cancel, expired_now, &mut preview_sequence);
    assert!(!clone.step(&source, &mut digest, &mut context).expect("expired clone yields"));
    assert_eq!(clone.phase, 0);
    assert_eq!(clone.bounds.phase, 0);
    assert!(clone.value.as_ref().expect("clone shell").layers.is_empty());
    while !clone.terminal_is_empty() {
        let _ = clone.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("expired clone closes through retained owner");
    }
    drop(clone);
}

#[test]
fn raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic() {
    let source = deeply_nested_raster_snapshot(40);
    let mutation = RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: "leaf".into(), new_name: "Renamed leaf".into() });
    let operation = semio_framework_job::OperationId(706);
    let generation = semio_framework_job::Generation(38);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut authority = RasterMutationCandidateAuthority::new();
    let mut turns = 0;
    loop {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        if authority.step(&source, &mutation, &mut context).expect("cursorized Raster mutation") {
            break;
        }
        turns += 1;
        assert!(turns < 30_000);
        let source_leaf = RasterLayerLocator::node_at(&source, RasterLayerAddress { length: 41, indices: [0; RASTER_MAXIMUM_NESTED_DEPTH] }).expect("source leaf remains reachable");
        let RasterLayerNode::Pixel { name, .. } = source_leaf else { panic!("source leaf remains a pixel") };
        assert_eq!(name, "Leaf", "the published source remains unchanged while the candidate is pending");
    }
    assert!(turns > 40);
    let candidate = authority.take().expect("mutation candidate publishes atomically");
    drop(authority);
    let mut locator = RasterLayerLocator::new();
    loop {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        if locator.step(&candidate, "leaf", &mut context).expect("candidate leaf locator") {
            break;
        }
    }
    let leaf = RasterLayerLocator::node_at(&candidate, locator.found.expect("candidate leaf")).expect("candidate leaf node");
    let RasterLayerNode::Pixel { name, .. } = leaf else { panic!("leaf remains a pixel") };
    assert_eq!(name, "Renamed leaf");
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(candidate));
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
        let _ = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("candidate closes one owner per grant");
    }
    drop(retirement);
}

#[test]
fn raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal() {
    let operation = semio_framework_job::OperationId(707);
    let generation = semio_framework_job::Generation(39);
    let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-cancel-complete", deeply_nested_raster_snapshot(24), None);
    let mut authority = RasterStoreInitializationAuthority::new(envelope, operation, generation);
    assert!(matches!(drive_raster_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
    assert!(authority.candidate.is_some());
    semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut authority);
    assert!(matches!(drive_raster_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Cancelled));
    assert!(authority.candidate.is_none());
    assert!(authority.candidate_disposer.is_none());
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
}

#[test]
fn raster_retirement_uses_allocation_capacity_and_fixed_iterative_depth() {
    let mut spare = String::with_capacity(RASTER_OWNED_FIELD_BYTES);
    spare.push('x');
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::String(spare));
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, 1).expect("insufficient byte grant retains exact string"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert!(!store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
    for _ in 0..16 {
        let _ = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("capacity-exact string and fixed frame retire");
        if store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            break;
        }
    }
    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
    drop(retirement);

    let snapshot = deeply_nested_raster_snapshot(RASTER_MAXIMUM_NESTED_DEPTH - 8);
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
    let mut turns = 0;
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
        let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("fixed iterative Raster depth retires");
        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
        }
        turns += 1;
        assert!(turns < 100_000);
    }
    drop(retirement);

    let mut value = dsl::DslValue::String(String::with_capacity(RASTER_OWNED_FIELD_BYTES));
    for _ in 0..(RASTER_MAXIMUM_NESTED_DEPTH - 8) {
        value = dsl::DslValue::Array(vec![value]);
    }
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Value(value));
    let mut turns = 0;
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
        let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("deep fixed value stack retires");
        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
        }
        turns += 1;
        assert!(turns < 100_000);
    }
    drop(retirement);
}

#[test]
fn raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone() {
    let mut totals = RasterOwnerTotals::new();
    assert!(totals.add(RASTER_MAXIMUM_NESTED_ITEMS, 0, RASTER_MAXIMUM_NESTED_ITEMS, 0).is_ok());
    assert_eq!(totals.add(1, 0, 0, 0), Err("raster-store.preflight-item-capacity"));

    let mut totals = RasterOwnerTotals::new();
    assert!(totals.add(0, RASTER_MAXIMUM_NESTED_BYTES, 0, RASTER_MAXIMUM_NESTED_BYTES).is_ok());
    assert_eq!(totals.add(0, 1, 0, 0), Err("raster-store.preflight-byte-capacity"));
}

#[test]
fn raster_retirement_page_credit_is_claimed_before_allocation_and_returned_with_backing() {
    let baseline = RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire);
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Value(dsl::DslValue::Array(vec![dsl::DslValue::String("owned".into())])));
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_CONTROL_BACKING_BYTES).expect("nested owner stages one push"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_CONTROL_BACKING_BYTES).expect("page credit is claimed before allocation"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), baseline + 1);
    assert_eq!(retirement.pending_page_credit, Some(0));
    assert!(retirement.pages[0].is_none());
    assert!(matches!(
        store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_CONTROL_BACKING_BYTES).expect("credited page allocation is a distinct transition"),
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    ));
    assert!(retirement.pending_page_credit.is_none());
    assert!(retirement.page_credits[0]);
    assert!(retirement.pages[0].is_some());
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
        let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_CONTROL_BACKING_BYTES).expect("credited page retires incrementally");
        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= RASTER_CONTROL_BACKING_BYTES);
        }
    }
    assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), baseline);
    drop(retirement);
}

#[test]
fn raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses() {
    let mut map = RasterOwnedMap::new();
    let key = String::from("exact-key");
    let key_pointer = key.as_ptr();
    map.insert(key, 7_u8).expect("one exact map entry");
    let mut removed = map.remove_entry("exact-key").expect("pair-returning removal");
    let (removed_key, removed_value) = removed.take();
    assert_eq!(removed_key.as_ptr(), key_pointer);
    assert_eq!(removed_value, 7);
    drop(removed_key);
    while let Some(page) = map.take_empty_page_backing() {
        page.release();
    }
    drop(map);

    let result = std::panic::catch_unwind(|| {
        let mut populated = RasterOwnedMap::new();
        populated.insert(String::from("must-retire"), 9_u8).expect("one populated page");
        drop(populated);
    });
    assert!(result.is_err(), "populated Raster map ordinary Drop must fail closed");
}

#[test]
fn raster_empty_asset_map_retirement_has_no_hidden_allocation_release() {
    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() };
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
    let layers = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("empty layer vector closes");
    assert!(matches!(layers, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
    let assets = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("empty fixed-page map shell closes allocation-free");
    assert!(matches!(assets, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
        let _ = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("empty snapshot reaches terminal");
    }
    drop(retirement);
}

#[test]
fn raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly() {
    let mut assets = RasterOwnedMap::new();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        assets
            .insert(
                format!("asset-{index:02}"),
                store::ArtifactChild::new(
                    format!("child-{index:02}"),
                    store::os_io::ArtifactRef { artifact_id: format!("artifact-{index:02}"), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect("fixed Raster map admits its exact item capacity");
    }
    let rejected_key = String::from("asset-overflow");
    let rejected_child_id = String::from("child-overflow");
    let key_pointer = rejected_key.as_ptr();
    let child_pointer = rejected_child_id.as_ptr();
    let rejected = assets
        .insert(
            rejected_key,
            store::ArtifactChild::new(rejected_child_id, store::os_io::ArtifactRef { artifact_id: "overflow".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } }),
        )
        .expect_err("fixed Raster map rejects capacity plus one");
    assert_eq!(rejected.key.as_ptr(), key_pointer);
    assert_eq!(rejected.value.child_id.as_ptr(), child_pointer);

    let (old_key_pointer, old_child_pointer) = {
        let (key, child) = assets.entry_at(0).expect("first admitted Raster map entry");
        (key.as_ptr(), child.child_id.as_ptr())
    };
    let mut replacement_key = String::with_capacity(64);
    replacement_key.push_str("asset-00");
    let replacement_key_pointer = replacement_key.as_ptr();
    let replacement_child_id = String::from("replacement-child");
    let replacement_child_pointer = replacement_child_id.as_ptr();
    let mut replaced = match assets
        .insert_pre_admitted(
            replacement_key,
            store::ArtifactChild::new(
                replacement_child_id,
                store::os_io::ArtifactRef { artifact_id: "replacement-artifact".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
            ),
        )
        .expect("replacement preserves fixed capacity")
    {
        RasterOwnedMapInsert::Replaced(previous) => previous,
        RasterOwnedMapInsert::Inserted => panic!("replacement returns the exact displaced pair"),
    };
    let (previous_key, previous_child) = replaced.take();
    assert_eq!(previous_key.as_ptr(), old_key_pointer);
    assert_eq!(previous_child.child_id.as_ptr(), old_child_pointer);
    let (installed_key, installed_child) = assets.entry_at(0).expect("replacement remains in stable order");
    assert_eq!(installed_key.as_ptr(), replacement_key_pointer);
    assert_eq!(installed_child.child_id.as_ptr(), replacement_child_pointer);
    let mut displaced = RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: previous_key, child: Some(previous_child) });
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&displaced) {
        let step = store::ErasedSnapshotRetirement::close_step(&mut displaced, 1, RASTER_OWNED_FIELD_BYTES).expect("displaced replacement owner closes exactly");
        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
        }
    }
    drop(displaced);

    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets };
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
    let mut page_backings = 0;
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("populated owned map retires one exact owner") {
            assert!(released_items <= 1);
            assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            if released_bytes == RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes() {
                page_backings += 1;
            }
        }
    }
    assert_eq!(page_backings, crate::RASTER_OWNED_MAP_CAPACITY / crate::RASTER_OWNED_MAP_PAGE_CAPACITY);
    drop(retirement);
}

#[test]
fn raster_observed_capacity_and_combined_retirement_depth_are_exact() {
    let mut source = String::with_capacity(64);
    source.push_str("observed");
    let candidate = raster_clone_owned_string(&source).expect("fixed-slice String construction is exact");
    assert_eq!(candidate.capacity(), candidate.len());

    let mut totals = RasterOwnerTotals::new();
    totals.add(0, 0, 0, 8).expect("base candidate credit");
    totals.observe_candidate_capacity(4, 7, 2).expect("allocator over-capacity is observed and admitted");
    assert_eq!(totals.candidate_bytes, 14);
    assert_eq!(RasterOwnerTotals::validate_control_backing_count(RASTER_MAXIMUM_CONTROL_BACKINGS), Ok(()));
    assert_eq!(RasterOwnerTotals::validate_control_backing_count(RASTER_MAXIMUM_CONTROL_BACKINGS + 1), Err("raster-store.control-backing-capacity"));
    assert_eq!(RASTER_MAXIMUM_CONTROL_BACKINGS, 64);
    assert_eq!(raster_retirement_frame_requirement(RASTER_MAXIMUM_NESTED_DEPTH, RASTER_MAXIMUM_NESTED_DEPTH), Ok(RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY));
    assert_eq!(raster_retirement_frame_requirement(RASTER_MAXIMUM_NESTED_DEPTH, RASTER_MAXIMUM_NESTED_DEPTH + 1), Err("raster-store.preflight-combined-depth"));
}

#[test]
fn raster_box_and_arc_control_backings_require_and_report_fixed_credit() {
    assert!(size_of::<RasterOwnedRetirement>() <= RASTER_CONTROL_BACKING_BYTES);
    assert!(size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES);
    assert!(size_of::<RasterSnapshotCloneAuthority>() <= RASTER_CONTROL_BACKING_BYTES);
    assert!(size_of::<RasterLayerCloneAuthority>() <= RASTER_CONTROL_BACKING_BYTES);
    assert!(size_of::<RasterDslValueCloneAuthority>() <= RASTER_CONTROL_BACKING_BYTES);
    let layer = Box::new(RasterLayerNode::Pixel { id: String::new(), name: String::new(), visible: true, opacity: 1.0, blend_mode: String::new(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None });
    let mut boxed = RasterOwnedRetirement::new(RasterRetirementOwner::BoxedLayer(Some(layer)));
    assert_eq!(boxed.control.as_ref().map(|credit| (credit.held_items, credit.held_bytes)), Some((1, RASTER_CONTROL_BACKING_BYTES)));
    assert!(matches!(
        store::ErasedSnapshotRetirement::close_step(&mut boxed, 1, RASTER_CONTROL_BACKING_BYTES - 1).expect("insufficient Box backing credit retains exact owner"),
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    ));
    assert!(matches!(
        store::ErasedSnapshotRetirement::close_step(&mut boxed, 1, RASTER_CONTROL_BACKING_BYTES).expect("fixed Box backing credit releases exact control owner"),
        store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES }
    ));
    while !store::ErasedSnapshotRetirement::terminal_is_empty(&boxed) {
        let _ = store::ErasedSnapshotRetirement::close_step(&mut boxed, 1, RASTER_OWNED_FIELD_BYTES).expect("boxed layer payload retires after its control backing");
    }
    assert!(boxed.control.is_none(), "standalone Box control credit is returned before terminal-empty");
    drop(boxed);

    let snapshot = std::sync::Arc::new(RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() });
    let mut root = store::SnapshotRetirementFactory::retire(&RasterSnapshotRetirementFactory, snapshot);
    assert!(matches!(root.close_step(1, RASTER_CONTROL_BACKING_BYTES - 1).expect("insufficient Arc credit retains root"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert!(matches!(root.close_step(1, RASTER_CONTROL_BACKING_BYTES).expect("Arc control backing is reported before root payload"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES }));
    while !root.terminal_is_empty() {
        let _ = root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("Arc root retires through exact retained owner");
    }
    assert!(root.terminal_is_empty(), "standalone Arc and inner Box control credits return before terminal-empty");
    drop(root);
}

#[test]
fn raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation() {
    let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0);
    let mut saturated = Vec::with_capacity(RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);
    for index in 0..RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY {
        let retirement = RasterOwnedRetirement::new(RasterRetirementOwner::String(format!("held-{index}")));
        assert_eq!(retirement.control.as_ref().map(|credit| (credit.held_items, credit.held_bytes)), Some((1, RASTER_CONTROL_BACKING_BYTES)));
        saturated.push(retirement);
    }
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);

    let plus_one_owner = String::from("exact-plus-one-owner");
    let plus_one_pointer = plus_one_owner.as_ptr();
    let construction = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| RasterOwnedRetirement::new(RasterRetirementOwner::String(plus_one_owner))));
    let mut plus_one = construction.expect("saturated standalone construction retains rather than panics");
    assert!(plus_one.control.is_none());
    let retained_pointer = match plus_one.root.as_ref().and_then(|frame| frame.owner.as_ref()) {
        Some(RasterRetirementOwner::String(value)) => value.as_ptr(),
        _ => panic!("saturated standalone retirement retains the exact producer owner"),
    };
    assert_eq!(retained_pointer, plus_one_pointer);
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut plus_one, 1, RASTER_OWNED_FIELD_BYTES).expect("full saturation is a retained pending result"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert!(plus_one.control.is_none());

    let mut returned = saturated.pop().expect("one saturated control is returned");
    close_raster_retirement(&mut returned);
    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&returned));
    drop(returned);
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY - 1);

    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut plus_one, 1, 0).expect("plus-one owner resumes into the returned exact control"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert_eq!(plus_one.control.as_ref().map(|credit| (credit.held_items, credit.held_bytes)), Some((1, RASTER_CONTROL_BACKING_BYTES)));
    let resumed_pointer = match plus_one.root.as_ref().and_then(|frame| frame.owner.as_ref()) {
        Some(RasterRetirementOwner::String(value)) => value.as_ptr(),
        _ => panic!("resumed standalone retirement still retains the exact producer owner"),
    };
    assert_eq!(resumed_pointer, plus_one_pointer);
    close_raster_retirement(&mut plus_one);
    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&plus_one));
    drop(plus_one);
    for retirement in &mut saturated {
        close_raster_retirement(retirement);
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(retirement));
    }
    drop(saturated);
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "held standalone control credits equal returned credits");
    assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "terminal standalone saturation leaves no page credit");
}

#[test]
fn raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase() {
    let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0);
    let mut saturated = Vec::with_capacity(RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY);
    for index in 0..RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY {
        saturated.push(RasterOwnedRetirement::new(RasterRetirementOwner::String(format!("arc-held-{index}"))));
    }
    let producer = std::sync::Arc::new(RasterSnapshot { schema: "arc-owner".into(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() });
    let producer_pointer = std::sync::Arc::as_ptr(&producer);
    let producer_witness = std::sync::Arc::downgrade(&producer);
    let construction = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| store::SnapshotRetirementFactory::retire(&RasterSnapshotRetirementFactory, producer)));
    let mut root = construction.expect("saturated Arc factory retains rather than panics");
    assert_eq!(std::sync::Arc::as_ptr(&producer_witness.upgrade().expect("saturated Arc owner remains alive")), producer_pointer);
    assert!(matches!(root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("full Arc control saturation retains exact owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));

    let mut first_return = saturated.pop().expect("one root control is returned");
    close_raster_retirement(&mut first_return);
    drop(first_return);
    assert!(matches!(root.close_step(1, 0).expect("Arc root claims the returned control without consuming its producer"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert_eq!(std::sync::Arc::as_ptr(&producer_witness.upgrade().expect("admitted Arc owner remains exact before unwrap")), producer_pointer);
    assert!(matches!(root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("Arc allocation transfers into the retained value phase"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: RASTER_CONTROL_BACKING_BYTES }));
    assert!(matches!(root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("inner Box retirement construction remains retained at saturation"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert!(matches!(root.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("saturated inner Box retirement yields without owner loss"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));

    let mut second_return = saturated.pop().expect("one inner Box control is returned");
    close_raster_retirement(&mut second_return);
    drop(second_return);
    close_raster_retirement(root.as_mut());
    assert!(root.terminal_is_empty(), "Arc owner, inner Box, and root control all reach terminal-empty");
    drop(root);
    assert!(producer_witness.upgrade().is_none(), "the exact producer allocation is retired once after every control phase");
    for retirement in &mut saturated {
        close_raster_retirement(retirement);
    }
    drop(saturated);
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "Arc and Box held controls equal returned controls");
    assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "Arc saturation leaves the retirement page process empty");
}

#[test]
fn raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
    let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
    let mut params = RasterOwnedMap::new();
    let mut first_key_pointer = std::ptr::null();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        let key = format!("key-{index:02}");
        if index == 0 {
            first_key_pointer = key.as_ptr();
        }
        let value = dsl::DslValue::Object(vec![("nested".into(), dsl::DslValue::Array(vec![dsl::DslValue::String(format!("value-{index}")), dsl::DslValue::Object(vec![("leaf".into(), dsl::DslValue::uint(index as u64))])]))]);
        params.insert(key, value).expect("maximum populated DSL map remains exactly page admitted");
    }
    assert_eq!(params.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.entry_at(0).expect("first exact map owner remains installed").0.as_ptr(), first_key_pointer);

    let plus_one_key = String::from("key-plus-one");
    let plus_one_key_pointer = plus_one_key.as_ptr();
    let plus_one_value = dsl::DslValue::String("plus-one-value".into());
    let rejected = params.insert(plus_one_key, plus_one_value).expect_err("capacity plus one returns both exact owners");
    assert_eq!(rejected.key.as_ptr(), plus_one_key_pointer);
    assert_eq!(rejected.reason, "raster-map.item-capacity");
    let mut rejected_retirement = RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected.key, value: Some(rejected.value) });

    let output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dsl::DslField::to_value(&params)));
    assert!(output.is_err(), "populated ordinary DSL output is rejected before any whole-map result exists");
    assert_eq!(params.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.entry_at(0).expect("panic keeps the first exact key/value/page owner installed").0.as_ptr(), first_key_pointer);
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut rejected_retirement, 0, 0).expect("cancellation-shaped zero grant preserves the rejected pair"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    close_raster_retirement(&mut rejected_retirement);
    drop(rejected_retirement);

    let populated_input = dsl::FieldValue::Map(vec![("forbidden".into(), dsl::FieldValue::Text("owner".into()))]);
    assert!(<RasterOwnedMap<dsl::DslValue> as dsl::DslField>::from_value(&populated_input).is_err(), "populated DSL input faults before page or semantic owner admission");
    let layer = RasterLayerNode::Adjustment { id: "dsl-output".into(), name: "DSL Output".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "nested".into(), params };
    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() };
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 0, 0).expect("cancelled populated output keeps every exact owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    close_raster_retirement(&mut retirement);
    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
    drop(retirement);
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated DSL rejection and close return every standalone control");
    assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "populated DSL rejection and close return every stack page credit");
}

#[test]
fn raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
    let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
    let mut params = RasterOwnedMap::new();
    let mut first_key_pointer = std::ptr::null();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        let key = format!("serde-key-{index:02}");
        if index == 0 {
            first_key_pointer = key.as_ptr();
        }
        let value = dsl::DslValue::Object(vec![("nested".into(), dsl::DslValue::Array(vec![dsl::DslValue::String(format!("serde-value-{index}")), dsl::DslValue::Object(vec![("leaf".into(), dsl::DslValue::uint(index as u64))])]))]);
        params.insert(key, value).expect("maximum populated serde map remains exactly page admitted");
    }
    assert_eq!(params.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.entry_at(0).expect("first serde owner remains installed").0.as_ptr(), first_key_pointer);

    let plus_one_key = String::from("serde-key-plus-one");
    let plus_one_key_pointer = plus_one_key.as_ptr();
    let rejected = params.insert(plus_one_key, dsl::DslValue::String("serde-plus-one-value".into())).expect_err("serde capacity plus one returns both exact owners");
    assert_eq!(rejected.key.as_ptr(), plus_one_key_pointer);
    assert_eq!(rejected.reason, "raster-map.item-capacity");
    let mut rejected_retirement = RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected.key, value: Some(rejected.value) });
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut rejected_retirement, 0, 0).expect("cancelled serde output preserves the rejected pair"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    close_raster_retirement(&mut rejected_retirement);
    drop(rejected_retirement);

    let layer = RasterLayerNode::Adjustment { id: "serde-output".into(), name: "Serde Output".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "nested".into(), params };
    let output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dsl::ToValue::to_value(&layer)));
    assert!(output.is_err(), "public populated dsl output faults before any whole-map result and contains panic");
    let params = match &layer {
        RasterLayerNode::Adjustment { params, .. } => params,
        _ => unreachable!("serde fixture remains an adjustment"),
    };
    assert_eq!(params.len(), crate::RASTER_OWNED_MAP_CAPACITY);
    assert_eq!(params.entry_at(0).expect("serde fault keeps the first exact owner installed").0.as_ptr(), first_key_pointer);

    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() };
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 0, 0).expect("zero-grant serde cancellation keeps every exact owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    close_raster_retirement(&mut retirement);
    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
    drop(retirement);
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated serde fault and close return every standalone control");
    assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "populated serde fault and close return every stack page credit");
    assert_eq!(RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated serde fault never claims or leaks an initialization process control");
}

#[test]
fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact() {
    let _guard = RASTER_STANDALONE_RETIREMENT_TEST_LOCK.lock().expect("Raster standalone retirement test lock");
    let mut deepest = dsl::DslValue::String("deep-output-owner".into());
    for _ in 1..RASTER_MAXIMUM_NESTED_DEPTH {
        deepest = dsl::DslValue::Array(vec![deepest]);
    }
    let mut params = RasterOwnedMap::new();
    let mut first_param_pointer = std::ptr::null();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        let key = format!("output-param-{index:02}");
        if index == 0 {
            first_param_pointer = key.as_ptr();
        }
        let value = if index == 0 { std::mem::replace(&mut deepest, dsl::DslValue::Null) } else { dsl::DslValue::String(format!("output-value-{index}")) };
        params.insert(key, value).expect("maximum populated output parameter map remains exactly admitted");
    }
    let plus_one_param_key = String::from("output-param-plus-one");
    let plus_one_param_pointer = plus_one_param_key.as_ptr();
    let plus_one_param_value = String::from("rejected-output-value");
    let plus_one_param_value_pointer = plus_one_param_value.as_ptr();
    let rejected_param = params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value)).expect_err("output parameter capacity plus one returns both exact owners");
    assert_eq!(rejected_param.key.as_ptr(), plus_one_param_pointer);
    let rejected_param_value = match &rejected_param.value {
        dsl::DslValue::String(value) => value,
        _ => unreachable!("rejected output parameter remains the exact string variant"),
    };
    assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");
    assert_eq!(rejected_param.reason, "raster-map.item-capacity");
    let mut rejected_param_retirement = RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) });
    assert!(matches!(
        store::ErasedSnapshotRetirement::close_step(&mut rejected_param_retirement, 0, 0).expect("zero-grant output parameter close preserves the rejected pair"),
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    ));
    close_raster_retirement(&mut rejected_param_retirement);
    drop(rejected_param_retirement);

    let mut assets = RasterOwnedMap::new();
    let mut first_asset_pointer = std::ptr::null();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        let key = format!("output-asset-{index:02}");
        if index == 0 {
            first_asset_pointer = key.as_ptr();
        }
        assets
            .insert(
                key,
                store::ArtifactChild::new(
                    format!("output-child-{index:02}"),
                    store::os_io::ArtifactRef { artifact_id: format!("output-artifact-{index:02}"), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect("maximum populated output asset map remains exactly admitted");
    }
    let plus_one_asset_key = String::from("output-asset-plus-one");
    let plus_one_asset_pointer = plus_one_asset_key.as_ptr();
    let plus_one_asset_child = store::ArtifactChild::new(
        "output-child-plus-one".into(),
        store::os_io::ArtifactRef { artifact_id: "output-artifact-plus-one".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
    );
    let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();
    let rejected_asset = assets.insert(plus_one_asset_key, plus_one_asset_child).expect_err("output asset capacity plus one returns both exact owners");
    assert_eq!(rejected_asset.key.as_ptr(), plus_one_asset_pointer);
    assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");
    assert_eq!(rejected_asset.reason, "raster-map.item-capacity");
    let mut rejected_asset_retirement = RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) });
    assert!(matches!(
        store::ErasedSnapshotRetirement::close_step(&mut rejected_asset_retirement, 0, 0).expect("zero-grant mounted output close preserves the rejected child pair"),
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    ));
    close_raster_retirement(&mut rejected_asset_retirement);
    drop(rejected_asset_retirement);

    let layer = RasterLayerNode::Adjustment { id: "retained-output".into(), name: "Retained Output".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "deep".into(), params };
    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets };
    assert_eq!(snapshot.require_empty_output_shell(), Err(crate::standards::v1::subsets::any::schema::snapshot::RASTER_POPULATED_OUTPUT_ERROR));
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| snapshot.require_empty_output_shell().expect(crate::standards::v1::subsets::any::schema::snapshot::RASTER_POPULATED_OUTPUT_ERROR)));
    assert!(panic.is_err(), "public DSL panic path contains the fail-closed populated output before allocation");
    let params = match &snapshot.layers[0] {
        RasterLayerNode::Adjustment { params, .. } => params,
        _ => unreachable!("output fixture remains an adjustment"),
    };
    assert_eq!(params.entry_at(0).expect("fault and panic retain the first parameter owner").0.as_ptr(), first_param_pointer);
    assert_eq!(snapshot.assets.entry_at(0).expect("all mounted exporters retain the first asset owner").0.as_ptr(), first_asset_pointer);

    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 0, 0).expect("cancelled output preserves every populated snapshot owner"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    close_raster_retirement(&mut retirement);
    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
    drop(retirement);
    assert_eq!(RASTER_STANDALONE_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated output rejection and close return every standalone control");
    assert_eq!(RASTER_RETIREMENT_PROCESS_PAGES.load(std::sync::atomic::Ordering::Acquire), 0, "populated output rejection and close return every stack page credit");
    assert_eq!(RASTER_INITIALIZATION_PROCESS_CONTROLS.load(std::sync::atomic::Ordering::Acquire), 0, "populated output rejection never claims or leaks an initialization control");
}

#[test]
fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal() {
    let mut value = dsl::DslValue::String("terminal".into());
    for _ in 1..RASTER_MAXIMUM_NESTED_DEPTH {
        value = dsl::DslValue::Array(vec![value]);
    }
    let mut params = RasterOwnedMap::new();
    params.insert("deep".into(), value).expect("one fixed parameter page");
    let mut layer = RasterLayerNode::Adjustment { id: "adjustment".into(), name: "Adjustment".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "levels".into(), params };
    for index in 1..RASTER_MAXIMUM_NESTED_DEPTH {
        layer = RasterLayerNode::Group { id: format!("group-{index}"), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![layer] };
    }
    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: vec![layer], assets: RasterOwnedMap::new() };
    let mut retirement = RasterOwnedRetirement::new(RasterRetirementOwner::Snapshot(snapshot));
    for _ in 0..200_000 {
        if store::ErasedSnapshotRetirement::terminal_is_empty(&retirement) {
            drop(retirement);
            return;
        }
        let step = store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, RASTER_OWNED_FIELD_BYTES).expect("maximum combined fixed retirement stack remains sufficient");
        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
        }
    }
    panic!("maximum combined Raster owner did not reach terminal-empty");
}

#[test]
fn raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant() {
    let mut params = RasterOwnedMap::new();
    params.insert("nested".repeat(16), dsl::DslValue::Object(vec![("array".repeat(16), dsl::DslValue::Array(vec![dsl::DslValue::String("payload".repeat(64)), dsl::DslValue::String("tail".into())]))])).expect("bounded fixture operation succeeds");
    let adjustment = RasterLayerNode::Adjustment { id: "adjustment".into(), name: "Adjustment".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "levels".into(), params };
    let mut snapshot = empty_raster_document();
    snapshot.title = Some("Nested raster".into());
    snapshot.layers.push(RasterLayerNode::Group { id: "group".into(), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![adjustment] });
    snapshot
        .assets
        .insert(
            "asset".into(),
            store::ArtifactChild::new("child".into(), store::os_io::ArtifactRef { artifact_id: "artifact".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } }),
        )
        .expect("bounded fixture operation succeeds");
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterSnapshotRetirementFactory, snapshot);
    for _ in 0..10_000 {
        match retirement.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("one nested Raster owner retires") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return;
            }
            store::SnapshotRetirementStep::Blocked => panic!("unshared Raster snapshot retirement cannot block"),
        }
    }
    panic!("nested Raster retirement did not reach terminal")
}

#[test]
fn raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant() {
    assert!(raster_clone_owned_string(&"x".repeat(RASTER_OWNED_FIELD_BYTES)).is_ok());
    assert!(raster_clone_owned_string(&"x".repeat(RASTER_OWNED_FIELD_BYTES + 1)).is_err());
    let pixel =
        || Box::new(RasterLayerNode::Pixel { id: "pixel".into(), name: "Pixel".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None });
    let mutations = vec![
        RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: Some("root".into()), index: 0, layer: pixel() }),
        RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: "pixel".into() }),
        RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id: "pixel".into(), parent_id: Some("root".into()), index: 1 }),
        RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: "pixel".into(), new_name: "Renamed".into() }),
        RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: "pixel".into(), new_visible: false }),
        RasterMutation::ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity { layer_id: "pixel".into(), new_opacity: 0.5 }),
        RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::mutation::ChangeLayerBlendMode { layer_id: "pixel".into(), new_blend_mode: "multiply".into() }),
        RasterMutation::MoveLayer(move_layer::mutation::MoveLayer { layer_id: "pixel".into(), new_x: 1.0, new_y: 2.0 }),
        RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: "pixel".into(), new_width: 2, new_height: 3 }),
        RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id: "pixel".into(), new_adjustment_kind: "levels".into() }),
        RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: "asset".into(), asset: RasterImageAsset { mime: "image/png".into(), data: vec![1, 2, 3] } }),
        RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: "asset".into() }),
    ];
    for mutation in mutations {
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterMutationRetirementFactory, mutation);
        assert!(matches!(retirement.close_step(0, RASTER_OWNED_FIELD_BYTES).expect("zero-grant Raster retirement"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        for _ in 0..100 {
            match retirement.close_step(1, RASTER_OWNED_FIELD_BYTES).expect("one Raster catalog owner retires") {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= RASTER_OWNED_FIELD_BYTES);
                }
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    break;
                }
                store::SnapshotRetirementStep::Blocked => panic!("unshared Raster mutation owner cannot block"),
            }
        }
        assert!(retirement.terminal_is_empty());
        drop(retirement);
    }
}

#[test]
fn raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner() {
    assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, 4_096);
    assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES, 64);
    assert_eq!(store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES, 262_144);
    let mut exact = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    exact[0] = 0x52;
    exact[store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES - 1] = 0x7f;
    let rejected = store::ArtifactEnvelopeDecodePage::try_from_array(exact, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1).expect_err("page cap plus one returns the exact caller owner");
    assert_eq!(rejected[0], 0x52);
    assert_eq!(rejected[store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES - 1], 0x7f);
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `RasterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip law (same pattern as `mathematical_protocol`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::RasterSnapshot;
    use protocol::{ArtifactId, Edit, SchemaId};

    let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "command-envelope-demo", empty_raster_document(), None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    store
        .dispatch(store::ArtifactCommand::Apply {
            mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                parent_id: None,
                index: 0,
                layer: Box::new(RasterLayerNode::Pixel {
                    id: "command-envelope-pixel".into(),
                    name: "Command Envelope Pixel".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: None,
                    width: Some(32),
                    height: Some(32),
                    image_key: None,
                }),
            })],
            description: None,
        })
        .await
        .expect("apply");
    let edit: &Edit<RasterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<RasterSnapshot, RasterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
