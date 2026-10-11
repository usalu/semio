//! 🧪️ Raster owned-host laws: the retained initializer, the stepwise clone and candidate authorities and their displaced-owner retirement.

use super::*;
use crate::mutations::rename_layer;
use crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_document;
use crate::{RasterTransform, RASTER_DOCUMENT_SCHEMA};
use semio_framework_job::{JobOutcomeBorrow, OperationId};
use semio_framework_plugin::{ArtifactOwnedDisposer, ArtifactStoreInitializationAuthority};

static RASTER_INITIALIZER_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn test_wallet() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 64 }
}

fn exact_grant(demand: semio_framework_value::RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RASTER_OWNED_FIELD_BYTES.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

#[derive(Debug, PartialEq, Eq)]
enum Terminal {
    Complete,
    Cancelled,
    Fault,
}

fn empty_raster_initializer(operation: OperationId, generation: semio_framework_job::Generation) -> RasterStoreInitializationAuthority {
    let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-retained-load", empty_raster_document(), None);
    RasterStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))
}

fn step_initializer(authority: &mut RasterStoreInitializationAuthority, operation: OperationId, generation: semio_framework_job::Generation, fuel: u64, cancel: &semio_framework_job::CancelToken, preview_sequence: &mut u64) -> Option<Terminal> {
    let mut progress = RetainedCloneProgress::default();
    let budget = semio_framework_job::StepBudget::new(fuel, u64::MAX, test_wallet());
    let mut context = semio_framework_job::StepContext::new(operation, generation, budget, cancel.clone(), semio_framework_job::default_now_us, preview_sequence, &mut progress);
    match ArtifactStoreInitializationAuthority::step(authority, &mut context).expect("Raster initializer turn") {
        Some(JobOutcomeBorrow::Complete { .. }) => Some(Terminal::Complete),
        Some(JobOutcomeBorrow::Cancelled { .. }) => Some(Terminal::Cancelled),
        Some(JobOutcomeBorrow::Fault { .. }) => Some(Terminal::Fault),
        _ => None,
    }
}

fn drive_raster_initializer(authority: &mut RasterStoreInitializationAuthority, operation: OperationId, generation: semio_framework_job::Generation, fuel: u64) -> Terminal {
    let _guard = RASTER_INITIALIZER_TEST_LOCK.lock().expect("Raster initializer test lock");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        if let Some(terminal) = step_initializer(authority, operation, generation, fuel, &cancel, &mut preview_sequence) {
            return terminal;
        }
    }
    panic!("Raster retained initializer did not reach a bounded terminal")
}

fn close_authority(authority: &mut RasterStoreInitializationAuthority) {
    ArtifactStoreInitializationAuthority::begin_close(authority);
    for _ in 0..200_000 {
        if ArtifactStoreInitializationAuthority::terminal_is_empty(authority) {
            return;
        }
        let demand = ArtifactStoreInitializationAuthority::retirement_demands(authority, RASTER_OWNED_FIELD_BYTES).expect("Raster initializer quotes its next close turn");
        ArtifactStoreInitializationAuthority::close_step(authority, exact_grant(demand)).expect("Raster initializer closes within its quoted grant");
    }
    panic!("Raster initializer did not reach terminal-empty");
}

fn close_raster_candidate(mut candidate: store::ArtifactStore<RasterSnapshot, RasterMutation>) {
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<RasterSnapshot, RasterMutation>::new();
    for _ in 0..200_000 {
        if disposer.terminal_is_empty(&candidate) {
            drop(disposer);
            drop(candidate);
            return;
        }
        let demand = disposer.retirement_demands(&candidate, RASTER_OWNED_FIELD_BYTES).expect("Raster candidate quotes its next close turn");
        disposer.close_step(&mut candidate, exact_grant(demand)).expect("Raster candidate close step");
    }
    panic!("Raster candidate did not reach terminal-empty close")
}

fn close_slot(slot: &mut RasterRetirementSlot) {
    for _ in 0..200_000 {
        if slot.is_empty() {
            return;
        }
        let demand = slot.demands(RASTER_OWNED_FIELD_BYTES).expect("Raster slot quotes its next close turn");
        slot.close(exact_grant(demand)).expect("Raster slot closes within its quoted grant");
    }
    panic!("Raster retirement slot did not reach terminal-empty");
}

#[test]
fn raster_history_edit_initializer_aliases_genesis_and_publishes_next_generation_and_candidate_closes_incrementally() {
    let operation = OperationId(701);
    let generation = semio_framework_job::Generation(31);
    let mut authority = empty_raster_initializer(operation, generation);
    let genesis = authority.envelope.as_ref().expect("retained Raster envelope").vcs.genesis.facts().share_snapshot();
    assert_eq!(drive_raster_initializer(&mut authority, operation, generation, 4_096), Terminal::Complete);
    let candidate = ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Raster candidate");
    assert_eq!(candidate.generation_now(), 32);
    assert!(std::sync::Arc::ptr_eq(&genesis, &candidate.snapshot_owner()));
    drop(genesis);
    close_authority(&mut authority);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_raster_candidate(candidate);
}

#[test]
fn raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
    let operation = OperationId(702);
    let generation = semio_framework_job::Generation(33);
    let mut cancelled = empty_raster_initializer(operation, generation);
    ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
    assert_eq!(drive_raster_initializer(&mut cancelled, operation, generation, 4_096), Terminal::Cancelled);
    close_authority(&mut cancelled);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
    drop(cancelled);

    let mut stale = empty_raster_initializer(operation, generation);
    assert_eq!(drive_raster_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1), 4_096), Terminal::Fault);
    close_authority(&mut stale);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
}

#[test]
fn raster_store_initializer_zero_budget_advances_no_owner_or_phase() {
    let operation = OperationId(703);
    let generation = semio_framework_job::Generation(35);
    let mut authority = empty_raster_initializer(operation, generation);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    assert_eq!(step_initializer(&mut authority, operation, generation, 0, &cancel, &mut preview_sequence), None);
    assert_eq!(authority.phase, RasterStoreInitializationPhase::ValidateEnvelope);
    assert!(authority.envelope.is_some());
    ArtifactStoreInitializationAuthority::request_cancel(&mut authority);
    assert_eq!(drive_raster_initializer(&mut authority, operation, generation, 4_096), Terminal::Cancelled);
    close_authority(&mut authority);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
}

fn deeply_nested_raster_snapshot(depth: usize) -> RasterSnapshot {
    let mut layer = RasterLayerNode::Pixel { id: "leaf".into(), name: "Leaf".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None };
    for index in 0..depth {
        layer = RasterLayerNode::Group { id: format!("group-{index}"), name: format!("Group {index}"), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: vec![layer] };
    }
    let mut snapshot = empty_raster_document();
    snapshot.layers.clear();
    snapshot.layers.push(layer);
    snapshot
}

fn with_context<R>(operation: OperationId, generation: semio_framework_job::Generation, fuel: u64, now: fn() -> Option<u64>, deadline: u64, run: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> R) -> R {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut progress = RetainedCloneProgress::default();
    let budget = semio_framework_job::StepBudget::new(fuel, deadline, test_wallet());
    let mut context = semio_framework_job::StepContext::new(operation, generation, budget, cancel, now, &mut preview_sequence, &mut progress);
    run(&mut context)
}

#[test]
fn raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel() {
    let source = deeply_nested_raster_snapshot(RASTER_MAXIMUM_NESTED_DEPTH - 8);
    let operation = OperationId(704);
    let generation = semio_framework_job::Generation(36);
    let mut clone = RasterSnapshotCloneAuthority::new();
    let mut turns = 0;
    while !clone.terminal {
        let done = with_context(operation, generation, 1, semio_framework_job::default_now_us, u64::MAX, |context| clone.step(&source, context).expect("bounded Raster clone"));
        assert!(!done || clone.terminal);
        turns += 1;
        assert!(turns < 20_000);
    }
    assert!(turns > 48, "nested clone must resume across the recursive layer depth");
    let candidate = clone.take_value().expect("bounded clone candidate");
    drop(clone);
    assert_eq!(candidate, source);
    retire_raster_value_cold(candidate);
    retire_raster_value_cold(source);
}

#[test]
fn raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page() {
    let _guard = RASTER_INITIALIZER_TEST_LOCK.lock().expect("mounted Raster initializer test lock");
    let operation = OperationId(7_041);
    let generation = semio_framework_job::Generation(361);
    let empty = empty_raster_document();
    let mut bounds = RasterSnapshotBoundsAuthority::new();
    for _ in 0..256 {
        if with_context(operation, generation, 64, semio_framework_job::default_now_us, u64::MAX, |context| bounds.step(&empty, context).expect("empty Raster bounds remain admissible")) {
            break;
        }
    }
    assert!(bounds.terminal);
    assert!(bounds.totals.source_bytes < RASTER_MAXIMUM_NESTED_BYTES);

    let mut source = empty_raster_document();
    source.layers.push(RasterLayerNode::Pixel { id: "mounted-layer".into(), name: "Mounted".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(1), height: Some(1), image_key: None });
    for index in 0..(crate::RASTER_OWNED_MAP_PAGE_CAPACITY + 1) {
        source
            .assets
            .insert(
                format!("mounted-{index}"),
                store::ArtifactChild::new(
                    format!("child-{index}"),
                    semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("artifact-{index}"), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect("mounted-shaped source admits its second fixed map page");
    }
    retire_raster_value_cold(empty);
    let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-mounted-64-fuel", source, None);
    let mut authority = RasterStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut terminal = None;
    for _ in 0..100_000 {
        if let Some(outcome) = step_initializer(&mut authority, operation, generation, 64, &cancel, &mut preview_sequence) {
            terminal = Some(outcome);
            break;
        }
    }
    assert_eq!(terminal, Some(Terminal::Complete));
    let candidate = ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("mounted-shaped candidate");
    close_authority(&mut authority);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_raster_candidate(candidate);
}

#[test]
fn raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner() {
    fn expired_now() -> Option<u64> {
        Some(10)
    }
    let source = deeply_nested_raster_snapshot(8);
    let mut clone = RasterSnapshotCloneAuthority::new();
    let finished = with_context(OperationId(705), semio_framework_job::Generation(37), 1, expired_now, 10, |context| clone.step(&source, context).expect("expired clone yields"));
    assert!(!finished);
    assert_eq!(clone.phase, 0);
    assert_eq!(clone.bounds.phase, 0);
    assert!(clone.value.as_ref().expect("clone shell").layers.is_empty());
    for _ in 0..200_000 {
        if clone.terminal_is_empty() {
            break;
        }
        let demand = clone.close_demands(RASTER_OWNED_FIELD_BYTES).expect("expired clone quotes its close turn");
        clone.close_step(exact_grant(demand)).expect("expired clone closes through retained owner");
    }
    assert!(clone.terminal_is_empty());
    drop(clone);
    retire_raster_value_cold(source);
}

#[test]
fn raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic() {
    let source = deeply_nested_raster_snapshot(RASTER_MAXIMUM_NESTED_DEPTH - 8);
    let mutation = RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: "leaf".into(), new_name: "Renamed leaf".into() });
    let operation = OperationId(706);
    let generation = semio_framework_job::Generation(38);
    let mut authority = RasterMutationCandidateAuthority::new();
    let mut turns = 0;
    loop {
        if with_context(operation, generation, 1, semio_framework_job::default_now_us, u64::MAX, |context| authority.step(&source, &mutation, context).expect("cursorized Raster mutation")) {
            break;
        }
        turns += 1;
        assert!(turns < 30_000);
        let source_leaf = RasterLayerLocator::node_at(&source, RasterLayerAddress { length: RASTER_MAXIMUM_NESTED_DEPTH - 7, indices: [0; RASTER_MAXIMUM_NESTED_DEPTH] }).expect("source leaf remains reachable");
        let RasterLayerNode::Pixel { name, .. } = source_leaf else { panic!("source leaf remains a pixel") };
        assert_eq!(name, "Leaf", "the published source remains unchanged while the candidate is pending");
    }
    assert!(turns > 40);
    let candidate = authority.take().expect("mutation candidate publishes atomically");
    drop(authority);
    let mut locator = RasterLayerLocator::new();
    loop {
        if with_context(operation, generation, 1, semio_framework_job::default_now_us, u64::MAX, |context| locator.step(&candidate, "leaf", context).expect("candidate leaf locator")) {
            break;
        }
    }
    let leaf = RasterLayerLocator::node_at(&candidate, locator.found.expect("candidate leaf")).expect("candidate leaf node");
    let RasterLayerNode::Pixel { name, .. } = leaf else { panic!("leaf remains a pixel") };
    assert_eq!(name, "Renamed leaf");
    retire_raster_value_cold(candidate);
    retire_raster_value_cold(source);
}

#[test]
fn raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal() {
    let operation = OperationId(707);
    let generation = semio_framework_job::Generation(39);
    let envelope = store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "raster-cancel-complete", deeply_nested_raster_snapshot(RASTER_MAXIMUM_NESTED_DEPTH - 8), None);
    let mut authority = RasterStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
    assert_eq!(drive_raster_initializer(&mut authority, operation, generation, 4_096), Terminal::Complete);
    assert!(authority.candidate.is_some());
    ArtifactStoreInitializationAuthority::request_cancel(&mut authority);
    assert_eq!(drive_raster_initializer(&mut authority, operation, generation, 4_096), Terminal::Cancelled);
    assert!(authority.candidate.is_none());
    close_authority(&mut authority);
    assert!(ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
}

#[test]
fn raster_displaced_owners_retire_through_exact_demand_quoted_grants() {
    let mut spare = String::with_capacity(RASTER_OWNED_FIELD_BYTES);
    spare.push('x');
    let mut slot = RasterRetirementSlot::new();
    slot.put(RasterDisplaced::String(spare));
    assert!(!slot.is_empty());
    let empty = slot.close(RetainedCloneGrant { maximum_items: 0, ..test_wallet() }).expect("a zero-item grant retains the exact string");
    assert_eq!(empty.progress(), RetainedCloneProgress::default());
    assert!(!slot.is_empty());
    let starved = slot.close(RetainedCloneGrant { maximum_capacity_bytes: 0, ..test_wallet() }).expect("an unfunded birth retains the exact string");
    assert_eq!(starved.progress(), RetainedCloneProgress::default());
    close_slot(&mut slot);

    let mut value = semio_framework_value::DslValue::String(String::with_capacity(RASTER_OWNED_FIELD_BYTES));
    for _ in 0..(RASTER_MAXIMUM_NESTED_DEPTH - 8) {
        value = semio_framework_value::DslValue::Array(vec![value]);
    }
    slot.put(RasterDisplaced::Value(value));
    close_slot(&mut slot);

    slot.put(RasterDisplaced::Snapshot(deeply_nested_raster_snapshot(RASTER_MAXIMUM_NESTED_DEPTH - 8)));
    close_slot(&mut slot);
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
fn raster_observed_capacity_and_combined_depth_are_exact() {
    let mut source = String::with_capacity(64);
    source.push_str("observed");
    let candidate = raster_clone_owned_string(&source).expect("fixed-slice String construction is exact");
    assert_eq!(candidate.capacity(), candidate.len());
    assert!(raster_clone_owned_string(&"x".repeat(RASTER_OWNED_FIELD_BYTES)).is_ok());
    assert!(raster_clone_owned_string(&"x".repeat(RASTER_OWNED_FIELD_BYTES + 1)).is_err());

    let mut totals = RasterOwnerTotals::new();
    totals.add(0, 0, 0, 8).expect("base candidate credit");
    totals.observe_candidate_capacity(4, 7, 2).expect("allocator over-capacity is observed and admitted");
    assert_eq!(totals.candidate_bytes, 14);
    assert_eq!(raster_combined_depth_requirement(RASTER_MAXIMUM_NESTED_DEPTH, RASTER_MAXIMUM_NESTED_DEPTH), Ok(RASTER_COMBINED_DEPTH_CAPACITY));
    assert_eq!(raster_combined_depth_requirement(RASTER_MAXIMUM_NESTED_DEPTH, RASTER_MAXIMUM_NESTED_DEPTH + 1), Err("raster-store.preflight-combined-depth"));
}

#[test]
fn raster_populated_owned_map_retires_every_entry_and_page_backing() {
    let mut assets = RasterOwnedMap::new();
    for index in 0..crate::RASTER_OWNED_MAP_CAPACITY {
        assets
            .insert(
                format!("asset-{index:02}"),
                store::ArtifactChild::new(
                    format!("child-{index:02}"),
                    semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("artifact-{index:02}"), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() } },
                ),
            )
            .expect("fixed Raster map admits its exact item capacity");
    }
    let snapshot = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets };
    retire_raster_value_cold(snapshot);
}
