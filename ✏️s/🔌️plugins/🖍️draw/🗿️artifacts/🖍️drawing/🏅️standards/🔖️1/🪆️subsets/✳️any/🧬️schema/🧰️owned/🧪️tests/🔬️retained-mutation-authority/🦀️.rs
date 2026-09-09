use super::*;
use crate::mutations::{
    CreateLayer, DeleteLayer, DuplicateLayer, RenameLayer, ReorderLayer, ReplaceLayerFill, ReplaceLayerStroke, SetLayerBlendMode, SetLayerBooleanOperation, SetLayerLocked, SetLayerOpacity, SetLayerVisible, UpdateLayerTraceParams,
    UpdateLayerTransform,
};

fn admit_string_destination(value: &mut String) {
    if value.capacity() < DRAWING_OWNED_FIELD_BYTES {
        value.try_reserve_exact(DRAWING_OWNED_FIELD_BYTES.saturating_sub(value.len())).expect("Drawing fixture string destination is pre-admitted");
    }
    assert!(value.capacity() >= DRAWING_OWNED_FIELD_BYTES);
}

fn admit_layer_string_destinations(layer: &mut DrawingLayerNode) {
    let base = crate::schema::layer_base_mut(layer);
    admit_string_destination(&mut base.id);
    admit_string_destination(&mut base.name);
    admit_string_destination(&mut base.blend_mode);
    match layer {
        DrawingLayerNode::Group(group) => {
            for child in &mut group.children {
                admit_layer_string_destinations(child);
            }
        }
        DrawingLayerNode::Boolean(boolean) => admit_string_destination(&mut boolean.operation),
        _ => {}
    }
}

fn initialize_drawing_mutation_arena_pool_for_test() {
    let operation = semio_framework_job::OperationId(7_900);
    let generation = semio_framework_job::Generation(79);
    let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).expect("fixed Drawing arena bootstrap job admission");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..1_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        match job.step(&mut context) {
            DrawingMutationArenaBootstrapStep::Ready => return,
            DrawingMutationArenaBootstrapStep::Pending { advanced_items } => assert_eq!(advanced_items, 1),
            DrawingMutationArenaBootstrapStep::Blocked => {}
            DrawingMutationArenaBootstrapStep::Cancelled => panic!("Drawing mutation arena bootstrap fixture was unexpectedly cancelled"),
            DrawingMutationArenaBootstrapStep::Fault(error) => panic!("Drawing mutation arena pool initialization faulted: {error}"),
        }
    }
    panic!("Drawing mutation arena pool initialization did not terminate")
}

fn nested_snapshot() -> DrawingSnapshot {
    initialize_drawing_mutation_arena_pool_for_test();
    let mut snapshot = crate::schema::default_drawing_document("drawing-retained-mutation", None);
    let shape = crate::schema::create_drawing_shape_layer_rect("Shape");
    let boolean = crate::schema::create_drawing_boolean_layer("Boolean", "union", vec![crate::schema::layer_id(&shape).into()]);
    let trace = crate::schema::create_drawing_trace_layer("Trace", "asset-a");
    let mut group = crate::schema::create_drawing_group_layer("Group");
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(shape);
        value.children.push(boolean);
        value.children.push(trace);
    }
    admit_layer_string_destinations(&mut group);
    snapshot.layers.push(group);
    snapshot.assets.insert("asset-a".into(), DrawingImageAsset { mime: "image/png".into(), data: "AA==".into(), width: Some(1), height: Some(1) });
    snapshot
}

fn drain_snapshot(value: DrawingSnapshot) {
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawingSnapshotRetirementFactory, value);
    for _ in 0..100_000 {
        match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES).expect("Drawing snapshot retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("owned Drawing snapshot retirement cannot block"),
        }
    }
    panic!("Drawing snapshot retirement did not terminate")
}

fn drain_mutation(value: DrawingMutation) {
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&DrawingMutationRetirementFactory, value);
    for _ in 0..100_000 {
        match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES).expect("Drawing mutation retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("owned Drawing mutation retirement cannot block"),
        }
    }
    panic!("Drawing mutation retirement did not terminate")
}

fn close_candidate(authority: &mut DrawingMutationCandidateAuthority, mut source: Option<&mut DrawingSnapshot>) {
    for _ in 0..100_000 {
        match authority.close_step(source.as_deref_mut(), DRAWING_OWNED_FIELD_BYTES).expect("Drawing candidate close") {
            store::SnapshotRetirementStep::Complete => {
                assert!(authority.terminal_is_empty());
                return;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= DRAWING_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("owned Drawing candidate close cannot block"),
        }
    }
    panic!("Drawing candidate close did not terminate")
}

fn apply(mut source: DrawingSnapshot, mutation: &DrawingMutation) -> Result<DrawingSnapshot, (DrawingSnapshot, &'static str)> {
    initialize_drawing_mutation_arena_pool_for_test();
    let operation = semio_framework_job::OperationId(8_001);
    let generation = semio_framework_job::Generation(81);
    let mut authority = DrawingMutationCandidateAuthority::try_new(operation, generation).expect("Drawing candidate fixed owner arenas admit");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..200_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        match authority.step(&mut source, mutation, &mut context) {
            Ok(true) => {
                authority.take().expect("Drawing mutation overlay exact terminal commit witness");
                assert!(authority.terminal_is_empty());
                drop(authority);
                return Ok(source);
            }
            Ok(false) => {}
            Err(error) => {
                close_candidate(&mut authority, Some(&mut source));
                drop(authority);
                return Err((source, error));
            }
        }
    }
    close_candidate(&mut authority, Some(&mut source));
    drop(authority);
    drain_snapshot(source);
    panic!("Drawing mutation candidate did not terminate")
}

fn live_reservation(source: &mut DrawingSnapshot, mutation: &DrawingMutation) -> Result<DrawingMutationAggregateReservation, &'static str> {
    initialize_drawing_mutation_arena_pool_for_test();
    let operation = semio_framework_job::OperationId(8_004);
    let generation = semio_framework_job::Generation(84);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut authority = DrawingMutationCandidateAuthority::try_new(operation, generation)?;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        authority.step(source, mutation, &mut context)?;
        if let Some(reservation) = authority.reservation {
            close_candidate(&mut authority, Some(source));
            drop(authority);
            return Ok(reservation);
        }
    }
    close_candidate(&mut authority, Some(source));
    drop(authority);
    Err("drawing-store.test-mutation-preflight-incomplete")
}

fn digest(mutation: &DrawingMutation) -> Result<[u8; 32], &'static str> {
    let mut authority = DrawingMutationDigestAuthority::new();
    let mut output = store::ArtifactStoreInitializationDigest::new(b"drawing.test.mutation");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(8_002),
            semio_framework_job::Generation(82),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        match authority.step(mutation, &mut output, &mut context) {
            Ok(true) => {
                assert!(authority.terminal_is_empty());
                drop(authority);
                return Ok(output.finish());
            }
            Ok(false) => {}
            Err(error) => {
                while !authority.terminal_is_empty() {
                    authority.close_step(DRAWING_OWNED_FIELD_BYTES).expect("Drawing digest rejection closes exactly");
                }
                drop(authority);
                return Err(error);
            }
        }
    }
    panic!("Drawing mutation digest did not terminate")
}

fn rich_layer() -> DrawingLayerNode {
    let mut group = crate::schema::create_drawing_group_layer("Digest Group");
    let base = crate::schema::layer_base_mut(&mut group);
    base.visible = false;
    base.locked = true;
    base.opacity = 0.75;
    base.blend_mode = "multiply".into();
    base.transform = crate::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5 };
    base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.25, color: [0.1, 0.2, 0.3, 0.4] }] });
    base.attributes.stroke = Some(StrokeStyle { color: [0.5, 0.6, 0.7, 0.8], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0, 2.0]) });
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(crate::schema::create_drawing_shape_layer_rect("Shape"));
        value.children.push(crate::schema::create_drawing_path_layer(
            "Path",
            vec![
                PathSegment::Move { to: [1.0, 2.0] },
                PathSegment::Line { to: [3.0, 4.0] },
                PathSegment::Quad { ctrl: [5.0, 6.0], to: [7.0, 8.0] },
                PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 14.0] },
                PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 17.0, large_arc: true, sweep: false, to: [18.0, 19.0] },
                PathSegment::Close,
            ],
        ));
        value.children.push(crate::schema::create_drawing_text_layer("Text"));
        value.children.push(crate::schema::create_drawing_image_layer("Image", "asset-reference"));
        value.children.push(crate::schema::create_drawing_boolean_layer("Boolean", "union", vec!["a".into(), "b".into()]));
        value.children.push(crate::schema::create_drawing_trace_layer("Trace", "trace-source"));
    }
    group
}

fn create_digest(layer: DrawingLayerNode) -> [u8; 32] {
    let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(3), layer: Box::new(layer) });
    let output = digest(&mutation).expect("rich Drawing create mutation hashes");
    drain_mutation(mutation);
    output
}

fn assert_mutation_digest_distinct(left: DrawingMutation, right: DrawingMutation) {
    let left_digest = digest(&left).expect("left Drawing mutation hashes");
    let right_digest = digest(&right).expect("right Drawing mutation hashes");
    assert_ne!(left_digest, right_digest, "changing one Drawing mutation semantic field changes the retained SHA-256 authority");
    drain_mutation(left);
    drain_mutation(right);
}

fn rich_child(layer: &mut DrawingLayerNode, index: usize) -> &mut DrawingLayerNode {
    let DrawingLayerNode::Group(group) = layer else { panic!("rich Drawing fixture root remains a Group") };
    group.children.get_mut(index).expect("rich Drawing fixture child")
}

#[test]
fn retained_drawing_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners() {
    let source = nested_snapshot();
    let group = crate::schema::layer_id(source.layers.last().expect("group")).to_string();
    let (shape, boolean, trace) = match source.layers.last().expect("group") {
        DrawingLayerNode::Group(value) => (crate::schema::layer_id(&value.children[0]).to_string(), crate::schema::layer_id(&value.children[1]).to_string(), crate::schema::layer_id(&value.children[2]).to_string()),
        _ => unreachable!("Drawing fixture group remains exact"),
    };
    let mutations = vec![
        DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: shape.clone(), visible: false }),
        DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: shape.clone(), locked: true }),
        DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: shape.clone(), opacity: 0.5 }),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: shape.clone(), blend_mode: "multiply".into() }),
        DrawingMutation::RenameLayer(RenameLayer { layer_id: shape.clone(), new_name: "Renamed".into() }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: shape.clone(), transform: crate::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5 } }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill {
            layer_id: shape.clone(),
            fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }] }),
        }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: shape.clone(), stroke: Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0, 2.0]) }) }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: boolean, boolean_operation: "subtract".into() }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: trace, params: crate::DrawingTraceParams { threshold: 0.4, simplify_epsilon: 1.2 } }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some(group.clone()), index: Some(1), layer: Box::new(crate::schema::create_drawing_path_layer("Created", vec![PathSegment::Move { to: [0.0, 0.0] }])) }),
        DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: shape.clone() }),
        DrawingMutation::DeleteLayer(DeleteLayer { layer_id: shape.clone() }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: shape, parent_id: None, index: 0 }),
    ];
    for mutation in mutations {
        let value = apply(nested_snapshot(), &mutation).expect("retained Drawing mutation applies");
        drain_snapshot(value);
    }
    drain_snapshot(source);
}

#[test]
fn retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba() {
    let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing process arena pool claims exact bytes and items before operation admission");
    let mut first = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let candidate = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_100 + index as u64), semio_framework_job::Generation(100 + index as u64), pool.clone())
            .expect("each fixed Drawing process arena slot admits exactly once");
        first.push((
            candidate.arena_slot,
            candidate.arena_generation,
            candidate.container_reverse.as_ref().expect("reverse arena owner").as_ptr(),
            candidate.container_output.as_ref().expect("output arena owner").as_ptr(),
            candidate.overlay_pages.as_ref().expect("page arena owner")[0].as_ptr(),
            candidate.duplicate_id_owner.as_ref().expect("duplicate id owner").as_ptr(),
            candidate,
        ));
    }
    match DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_999), semio_framework_job::Generation(999), pool.clone()) {
        Err(error) => assert_eq!(error, "drawing-store.mutation-arena-pool-saturated"),
        Ok(_) => panic!("fixed Drawing arena pool must reject capacity +1"),
    }
    for entry in &mut first {
        for phase in 0..4 {
            assert_eq!(entry.6.return_arena_owner().expect("one fixed Drawing root returns per opportunity"), Some(phase == 3));
            let state = pool.state.try_lock().expect("isolated Drawing pool is uncontended");
            let slot = &state.slots[entry.0];
            let returned = usize::from(slot.reverse.is_some()) + usize::from(slot.output.is_some()) + usize::from(slot.pages.is_some()) + usize::from(slot.duplicate_id.is_some());
            assert_eq!(returned, phase + 1, "exactly one fixed arena root returns per grant");
            assert_eq!(slot.leased, phase < 3, "slot becomes available only after the fourth exact owner returns");
        }
        close_candidate(&mut entry.6, None);
    }
    let first_witnesses: Vec<_> = first.iter().map(|entry| (entry.0, entry.1, entry.2, entry.3, entry.4, entry.5)).collect();
    for entry in first {
        drop(entry.6);
    }

    let mut second = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let candidate = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(8_200 + index as u64), semio_framework_job::Generation(200 + index as u64), pool.clone()).expect("returned Drawing arena slot re-admits");
        second.push(candidate);
    }
    for candidate in &second {
        let witness = first_witnesses.iter().find(|entry| entry.0 == candidate.arena_slot).expect("same fixed Drawing arena slot returns");
        assert!(candidate.arena_generation > witness.1, "slot generation advances to reject stale ABA returns");
        assert_eq!(candidate.container_reverse.as_ref().expect("reverse arena returned").as_ptr(), witness.2);
        assert_eq!(candidate.container_output.as_ref().expect("output arena returned").as_ptr(), witness.3);
        assert_eq!(candidate.overlay_pages.as_ref().expect("page arena returned")[0].as_ptr(), witness.4);
        assert_eq!(candidate.duplicate_id_owner.as_ref().expect("duplicate owner returned").as_ptr(), witness.5);
    }
    for candidate in &mut second {
        close_candidate(candidate, None);
    }
    for candidate in second {
        drop(candidate);
    }
}

fn step_arena_bootstrap(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> Result<bool, &'static str> {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_902), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
    bootstrap.step(&mut context)
}

fn close_arena_bootstrap_step(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> store::SnapshotRetirementStep {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(7_903), semio_framework_job::Generation(79), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
    bootstrap.close_step(&mut context)
}

fn close_arena_bootstrap(bootstrap: &mut DrawingMutationArenaPoolBootstrap) -> usize {
    let mut released_roots = 0;
    for _ in 0..10_000 {
        match close_arena_bootstrap_step(bootstrap) {
            store::SnapshotRetirementStep::Pending { released_items, .. } => {
                assert!(released_items <= 1, "Drawing arena bootstrap releases at most one exact root per grant");
                released_roots += released_items;
            }
            store::SnapshotRetirementStep::Complete => {
                assert!(bootstrap.terminal_is_empty());
                return released_roots;
            }
            store::SnapshotRetirementStep::Blocked => panic!("isolated Drawing arena bootstrap never blocks"),
        }
    }
    panic!("Drawing arena bootstrap retirement did not terminate")
}

fn build_arena_bootstrap_owners(bootstrap: &mut DrawingMutationArenaPoolBootstrap) {
    for _ in 0..10_000 {
        if bootstrap.owner == DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            return;
        }
        assert_eq!(step_arena_bootstrap(bootstrap), Ok(false));
    }
    panic!("Drawing arena bootstrap did not construct its fixed owner catalog")
}

#[test]
fn retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant() {
    let allocations = DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20;
    for failure_at in 0..allocations {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::new(Some(failure_at), None, usize::MAX, usize::MAX);
        let fault = loop {
            match step_arena_bootstrap(&mut bootstrap) {
                Ok(false) => {}
                Ok(true) => panic!("injected Drawing arena allocation failure was not observed"),
                Err(error) => break error,
            }
        };
        assert_eq!(fault, "drawing-store.mutation-arena-bootstrap-injected-allocation");
        assert_eq!(bootstrap.allocation, failure_at + 1);
        assert_eq!(close_arena_bootstrap(&mut bootstrap), failure_at, "every successfully constructed Vec/String/page root is handed to the retained fault cursor");
        drop(bootstrap);
    }
}

#[test]
fn retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close() {
    for owner in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::new(None, Some(owner), usize::MAX, usize::MAX);
        let fault = loop {
            match step_arena_bootstrap(&mut bootstrap) {
                Ok(false) => {}
                Ok(true) => panic!("injected Drawing arena bundle failure was not observed"),
                Err(error) => break error,
            }
        };
        assert_eq!(fault, "drawing-store.mutation-arena-bootstrap-injected-owner");
        assert_eq!(bootstrap.owner, owner + 1);
        assert_eq!(close_arena_bootstrap(&mut bootstrap), (owner + 1) * 20, "every completed bundle remains in the retained construction-fault owner");
        drop(bootstrap);
    }
}

#[test]
fn retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool() {
    let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().expect("fixed Drawing arena bootstrap claim"));
    let mut turns = 0;
    while !bootstrap.ready {
        let allocation = bootstrap.allocation;
        assert!(bootstrap.take_pool().is_none(), "an incomplete Drawing arena bootstrap cannot publish an operation-admission pool");
        assert!(matches!(step_arena_bootstrap(&mut bootstrap), Ok(false) | Ok(true)));
        assert!(bootstrap.allocation.saturating_sub(allocation) <= 1, "one governed bootstrap turn allocates at most one retained root");
        turns += 1;
        assert!(turns < 1_000);
    }
    let pool = bootstrap.take_pool().expect("terminal Drawing arena bootstrap publishes the fixed process pool");
    assert_eq!(pool.state.try_lock().expect("isolated Drawing arena pool is uncontended").slots.len(), DRAWING_MUTATION_ARENA_POOL_CAPACITY);
    drop(bootstrap);
    drop(pool);
}

#[test]
fn retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close() {
    let mut exact = DrawingMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
    build_arena_bootstrap_owners(&mut exact);
    let admitted_items = exact.admitted_items;
    let admitted_bytes = exact.admitted_bytes;
    exact.maximum_items = admitted_items;
    exact.maximum_bytes = admitted_bytes;
    assert_eq!(step_arena_bootstrap(&mut exact), Ok(true), "allocator-returned Drawing arena capacities admit at the exact boundary");
    assert_eq!(close_arena_bootstrap(&mut exact), DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20);
    drop(exact);

    for (maximum_items, maximum_bytes) in [(admitted_items - 1, admitted_bytes), (admitted_items, admitted_bytes - 1)] {
        let mut rejected = DrawingMutationArenaPoolBootstrap::new(None, None, usize::MAX, usize::MAX);
        build_arena_bootstrap_owners(&mut rejected);
        rejected.maximum_items = maximum_items;
        rejected.maximum_bytes = maximum_bytes;
        assert_eq!(step_arena_bootstrap(&mut rejected), Err("drawing-store.mutation-arena-pool-capacity"));
        assert_eq!(close_arena_bootstrap(&mut rejected), DRAWING_MUTATION_ARENA_POOL_CAPACITY * 20, "aggregate +1 rejection retains all eighty exact roots until cursorized close");
        drop(rejected);
    }
}

#[test]
fn retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation() {
    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let guard = state.try_lock().expect("isolated Drawing request fixture owns the inert process metadata");
    let witness = match &*guard {
        DrawingMutationArenaProcessState::Inert => (0, 0),
        DrawingMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
        DrawingMutationArenaProcessState::Ready(_) => (2, 0),
        DrawingMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
        DrawingMutationArenaProcessState::Fault(_) => (4, 0),
    };
    assert_eq!(request_drawing_mutation_arena_pool(), DrawingMutationArenaPoolAvailability::Contended);
    assert_eq!(request_drawing_mutation_arena_pool(), DrawingMutationArenaPoolAvailability::Contended, "a second app request coalesces fixed metadata without allocation");
    match borrow_drawing_mutation_arena() {
        Err(error) => assert_eq!(error, DrawingMutationArenaBorrowError::Contended),
        Ok(_) => panic!("borrow under process contention cannot expose an arena owner"),
    }
    let after = match &*guard {
        DrawingMutationArenaProcessState::Inert => (0, 0),
        DrawingMutationArenaProcessState::Building(bootstrap) => (1, bootstrap.allocation),
        DrawingMutationArenaProcessState::Ready(_) => (2, 0),
        DrawingMutationArenaProcessState::Retiring(bootstrap) => (3, bootstrap.allocation),
        DrawingMutationArenaProcessState::Fault(_) => (4, 0),
    };
    assert_eq!(after, witness, "default/request/borrow cannot advance a bootstrap allocation while no governed job owns the process turn");
    drop(guard);
}

#[test]
fn retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed() {
    let operation = semio_framework_job::OperationId(7_904);
    let generation = semio_framework_job::Generation(79);
    let mut job = DrawingMutationArenaBootstrapJob::new(operation, generation).expect("fixed Drawing bootstrap admission claim");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut exhausted = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(0, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
    assert_eq!(job.step(&mut exhausted), DrawingMutationArenaBootstrapStep::Blocked, "zero-budget bootstrap cannot allocate or retire");

    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let guard = state.try_lock().expect("isolated Drawing bootstrap fixture owns process contention");
    let mut contended = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
    assert_eq!(job.step(&mut contended), DrawingMutationArenaBootstrapStep::Blocked, "process contention leaves the exact bootstrap owner untouched");
    drop(guard);

    let local_operation = semio_framework_job::OperationId(7_905);
    let local_generation = semio_framework_job::Generation(79);
    let mut local_job = DrawingMutationArenaBootstrapJob::new(local_operation, local_generation).expect("local Drawing bootstrap job claims its fixed admission");
    let local_cancel = semio_framework_job::root_cancel_token();
    let mut local_preview_sequence = 0;
    let mut local_state = DrawingMutationArenaProcessState::Inert;
    for _ in 0..3 {
        let mut admitted = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX), local_cancel.clone(), semio_framework_job::default_now_us, &mut local_preview_sequence);
        assert_eq!(local_job.step_locked(&mut local_state, &mut admitted), DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 });
    }
    let allocated = match &local_state {
        DrawingMutationArenaProcessState::Building(bootstrap) => bootstrap.allocation,
        _ => panic!("three governed Drawing bootstrap turns retain one partially allocated bundle"),
    };
    assert_eq!(allocated, 1, "only admitted worker turns may advance allocation boundaries");
    local_cancel.cancel_now();
    for _ in 0..100 {
        let mut cancelled = semio_framework_job::StepContext::new(local_operation, local_generation, semio_framework_job::StepBudget::new(1, u64::MAX), local_cancel.clone(), semio_framework_job::default_now_us, &mut local_preview_sequence);
        match local_job.step_locked(&mut local_state, &mut cancelled) {
            DrawingMutationArenaBootstrapStep::Pending { advanced_items } => assert!(advanced_items <= 1),
            DrawingMutationArenaBootstrapStep::Cancelled => break,
            DrawingMutationArenaBootstrapStep::Blocked => {}
            DrawingMutationArenaBootstrapStep::Ready | DrawingMutationArenaBootstrapStep::Fault(_) => panic!("cancelled partial Drawing bootstrap must retire to exact Cancelled"),
        }
    }
    assert!(local_job.terminal);

    let pool = DrawingMutationArenaPool::try_new().expect("isolated fixed Drawing pool admits exact saturation fixture");
    let mut candidates = Vec::new();
    for index in 0..DRAWING_MUTATION_ARENA_POOL_CAPACITY {
        candidates
            .push(DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_910 + index as u64), semio_framework_job::Generation(80 + index as u64), pool.clone()).expect("each fixed Drawing pool slot admits once"));
    }
    assert!(matches!(DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(7_999), semio_framework_job::Generation(99), pool), Err("drawing-store.mutation-arena-pool-saturated")));
    for candidate in &mut candidates {
        close_candidate(candidate, None);
    }
    for candidate in candidates {
        drop(candidate);
    }
}

#[test]
fn retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty() {
    let mut layer = crate::schema::create_drawing_path_layer("leaf", Vec::new());
    for depth in 0..=DRAWING_MAXIMUM_LAYER_DEPTH {
        let mut parent = crate::schema::create_drawing_group_layer(&format!("depth-{depth}"));
        if let DrawingLayerNode::Group(value) = &mut parent {
            value.children.push(layer);
        }
        layer = parent;
    }
    let mut source = crate::schema::default_drawing_document("drawing-depth-plus-one", None);
    source.layers = vec![layer];
    let mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "missing".into(), visible: false });
    let (source, error) = apply(source, &mutation).expect_err("retained Drawing depth +1 authority rejects");
    assert_eq!(error, "drawing-store.preflight-depth-capacity");
    drain_snapshot(source);

    let source = nested_snapshot();
    let mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1), new_name: "hostile".into() });
    let (source, _) = apply(source, &mutation).expect_err("hostile Drawing field rejects");
    drain_snapshot(source);
}

#[test]
fn retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner() {
    let mut snapshot = crate::schema::default_drawing_document("rebuild-reservation", None);
    snapshot.layers = vec![crate::schema::create_drawing_path_layer("first", Vec::new()), crate::schema::create_drawing_path_layer("second", Vec::new())];
    let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(crate::schema::create_drawing_path_layer("pending", Vec::new())) });
    let reservation = live_reservation(&mut snapshot, &mutation).expect("live Drawing rebuild reservation admitted");
    let source = std::mem::take(&mut snapshot.layers);
    let DrawingMutation::CreateLayer(mut create) = mutation else { unreachable!() };
    let pending = *std::mem::replace(&mut create.layer, Box::new(crate::schema::create_drawing_path_layer("retired-placeholder", Vec::new())));
    drain_mutation(DrawingMutation::CreateLayer(create));
    drain_snapshot(snapshot);
    let mut reverse = Vec::new();
    let mut output = Vec::new();
    reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Drawing reverse arena");
    output.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("fixed Drawing output arena");
    let source_owner = source.as_ptr();
    let reverse_owner = reverse.as_ptr();
    let output_owner = output.as_ptr();
    let mut authority = DrawingContainerRebuildAuthority::new(source, Some(0), Some(1), Some(pending), reverse, output, reservation).unwrap_or_else(|_| panic!("fixed Drawing rebuild admitted"));
    assert!(authority.take().is_none(), "false terminal cannot expose a partially rebuilt owner");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..3 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(8_005),
            semio_framework_job::Generation(85),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        assert!(!authority.step(&mut context).expect("Drawing rebuild advances before interruption"));
    }
    let mut rollback_turns = 0;
    while !authority.rollback_step().expect("Drawing rebuild rollback advances one exact owner") {
        rollback_turns += 1;
    }
    assert_eq!(rollback_turns, authority.move_count, "one recorded owner move rolls back per close grant");
    let restored = authority.source.take().expect("original Drawing source owner returns");
    let pending = authority.pending.take().expect("pending Drawing owner returns");
    let reverse = authority.reverse.take().expect("reverse Drawing scratch owner returns");
    let output = authority.output.take().expect("output Drawing scratch owner returns");
    assert_eq!(restored.as_ptr(), source_owner);
    assert_eq!(reverse.as_ptr(), reverse_owner);
    assert_eq!(output.as_ptr(), output_owner);
    assert!(authority.removed.is_none());
    authority.finish_handoff().expect("Drawing rebuild rollback reaches exact terminal handoff");
    assert!(authority.terminal_is_empty());
    drop(authority);
    drop(reverse);
    drop(output);
    drain_mutation(DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(pending) }));
    let mut restored_snapshot = crate::schema::default_drawing_document("restored-rebuild", None);
    restored_snapshot.layers = restored;
    drain_snapshot(restored_snapshot);
}

#[test]
fn retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot() {
    for phase in 0..=3 {
        for stale in [false, true] {
            let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing rollback pool admits exact owners");
            let mut source = crate::schema::default_drawing_document("rebuild-rollback", None);
            source.layers.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).expect("Drawing rollback fixture pre-admits original live container backing");
            for index in 0..3 {
                source.layers.push(crate::schema::create_drawing_path_layer(&format!("source-{index}"), Vec::new()));
            }
            let source_owner = source.layers.as_ptr();
            let source_ids: Vec<_> = source.layers.iter().map(|layer| crate::schema::layer_id(layer).to_string()).collect();
            let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: Some(1), layer: Box::new(crate::schema::create_drawing_path_layer("pending", Vec::new())) });
            let operation = semio_framework_job::OperationId(8_500 + phase as u64);
            let generation = semio_framework_job::Generation(850 + phase as u64);
            let mut authority = DrawingMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Drawing rollback candidate borrows one exact pool slot");
            let slot = authority.arena_slot;
            let arena_generation = authority.arena_generation;
            let reverse_owner = authority.container_reverse.as_ref().expect("Drawing rollback reverse owner").as_ptr();
            let output_owner = authority.container_output.as_ref().expect("Drawing rollback output owner").as_ptr();
            let page_catalog_owner = authority.overlay_pages.as_ref().expect("Drawing rollback page catalog owner").as_ptr();
            let page_owners: [usize; DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY] = std::array::from_fn(|index| authority.overlay_pages.as_ref().expect("Drawing rollback page owner")[index].as_ptr() as usize);
            let duplicate_owner = authority.duplicate_id_owner.as_ref().expect("Drawing rollback duplicate owner").as_ptr();
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview_sequence = 0;
            for _ in 0..100_000 {
                if authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)) {
                    break;
                }
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing rollback fixture reaches every internal rebuild phase"));
            }
            assert!(authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(phase)));
            if !stale {
                cancel.cancel_now();
            }
            let mut rejected = semio_framework_job::StepContext::new(
                operation,
                if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel,
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }));
            close_candidate(&mut authority, Some(&mut source));
            assert_eq!(source.layers.as_ptr(), source_owner, "rollback restores the exact original live Vec backing");
            assert_eq!(source.layers.iter().map(|layer| crate::schema::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "rollback restores exact FIFO layer order");
            drop(authority);

            let mut reused = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 100), semio_framework_job::Generation(generation.0 + 100), pool.clone())
                .expect("rolled-back Drawing pool slot re-admits immediately");
            assert_eq!(reused.arena_slot, slot);
            assert!(reused.arena_generation > arena_generation);
            assert_eq!(reused.container_reverse.as_ref().expect("returned reverse owner").as_ptr(), reverse_owner);
            assert_eq!(reused.container_output.as_ref().expect("returned output owner").as_ptr(), output_owner);
            assert_eq!(reused.overlay_pages.as_ref().expect("returned page catalog owner").as_ptr(), page_catalog_owner);
            assert_eq!(std::array::from_fn::<_, DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY, _>(|index| reused.overlay_pages.as_ref().expect("returned page owner")[index].as_ptr() as usize), page_owners);
            assert_eq!(reused.duplicate_id_owner.as_ref().expect("returned duplicate owner").as_ptr(), duplicate_owner);
            close_candidate(&mut reused, None);
            drop(reused);
            drain_mutation(mutation);
            drain_snapshot(source);
        }
    }
}

#[test]
fn retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots() {
    for stale in [false, true] {
        let pool = DrawingMutationArenaPool::try_new().expect("isolated Drawing reorder rollback pool admits exact owners");
        let mut source = nested_snapshot();
        let (group_id, target, source_owner, source_ids) = match source.layers.last_mut().expect("Drawing reorder rollback group") {
            DrawingLayerNode::Group(group) => {
                group.children.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY.saturating_sub(group.children.len())).expect("Drawing reorder rollback fixture pre-admits the nested live container");
                (group.base.id.clone(), crate::schema::layer_id(&group.children[0]).to_string(), group.children.as_ptr(), group.children.iter().map(|layer| crate::schema::layer_id(layer).to_string()).collect::<Vec<_>>())
            }
            _ => unreachable!("Drawing reorder rollback fixture remains a group"),
        };
        let mutation = DrawingMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 });
        let operation = semio_framework_job::OperationId(8_700);
        let generation = semio_framework_job::Generation(870);
        let mut authority = DrawingMutationCandidateAuthority::try_new_from_pool(operation, generation, pool.clone()).expect("Drawing reorder rollback candidate borrows one exact pool slot");
        let slot = authority.arena_slot;
        let reverse_owner = authority.container_reverse.as_ref().expect("Drawing reorder reverse owner").as_ptr();
        let output_owner = authority.container_output.as_ref().expect("Drawing reorder output owner").as_ptr();
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            if authority.rebuild_role == Some(DrawingContainerRebuildRole::Destination) && authority.rebuild.as_ref().is_some_and(|rebuild| rebuild.recorded_move_in_phase(2)) {
                break;
            }
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing reorder rollback fixture reaches destination rebuild after source handoff"));
        }
        assert_eq!(authority.rebuild_role, Some(DrawingContainerRebuildRole::Destination));
        assert!(authority.source_undo.is_some(), "source handoff keeps the exact insertion undo authority until destination publication");
        if !stale {
            cancel.cancel_now();
        }
        let mut rejected = semio_framework_job::StepContext::new(
            operation,
            if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation },
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel,
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }));
        close_candidate(&mut authority, Some(&mut source));
        drop(authority);
        let DrawingLayerNode::Group(group) = source.layers.last().expect("Drawing reorder rollback group remains retained") else { unreachable!("Drawing reorder rollback group remains a group") };
        assert_eq!(group.children.as_ptr(), source_owner, "source undo restores the exact nested live Vec backing");
        assert_eq!(group.children.iter().map(|layer| crate::schema::layer_id(layer)).collect::<Vec<_>>(), source_ids.iter().map(String::as_str).collect::<Vec<_>>(), "source undo restores the exact nested FIFO order");

        let mut reused = DrawingMutationCandidateAuthority::try_new_from_pool(semio_framework_job::OperationId(operation.0 + 1), semio_framework_job::Generation(generation.0 + 1), pool).expect("reorder rollback returns the exact pool slot");
        assert_eq!(reused.arena_slot, slot);
        assert_eq!(reused.container_reverse.as_ref().expect("returned reorder reverse owner").as_ptr(), reverse_owner);
        assert_eq!(reused.container_output.as_ref().expect("returned reorder output owner").as_ptr(), output_owner);
        close_candidate(&mut reused, None);
        drop(reused);
        drain_mutation(mutation);
        drain_snapshot(source);
    }
}

#[test]
fn retained_drawing_schema_digest_distinguishes_every_nested_semantic_field() {
    let baseline = rich_layer();
    let baseline_digest = create_digest(baseline.clone());
    let mut variants = Vec::new();

    let modifiers: &[fn(&mut DrawingLayerNode)] = &[
        |value| crate::schema::layer_base_mut(value).id = "different-id".into(),
        |value| crate::schema::layer_base_mut(value).name = "different-name".into(),
        |value| crate::schema::layer_base_mut(value).transform.x = 9.0,
        |value| crate::schema::layer_base_mut(value).transform.y = 9.0,
        |value| crate::schema::layer_base_mut(value).transform.scale_x = 9.0,
        |value| crate::schema::layer_base_mut(value).transform.scale_y = 9.0,
        |value| crate::schema::layer_base_mut(value).attributes.fill = None,
        |value| {
            if let Some(FillStyle::RadialGradient { cx, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                *cx = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { cy, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                *cy = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { r, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                *r = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { stops, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                stops[0].offset = 0.75;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { stops, .. }) = &mut crate::schema::layer_base_mut(value).attributes.fill {
                stops[0].color[2] = 0.9;
            }
        },
        |value| crate::schema::layer_base_mut(value).attributes.stroke = None,
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").color[0] = 0.9,
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").width = 9.0,
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").cap = "square".into(),
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").join = "round".into(),
        |value| crate::schema::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").dash.as_mut().expect("dash")[0] = 9.0,
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.rect.as_mut().expect("rect").width = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.ellipse = Some(crate::DrawingEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.circle = Some(crate::DrawingCircle { cx: 1.0, cy: 2.0, r: 3.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.line = Some(crate::DrawingLine { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.polygon = Some(crate::DrawingPolygon { points: vec![[1.0, 2.0], [3.0, 4.0]] });
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[0] = PathSegment::Line { to: [1.0, 2.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[1] = PathSegment::Line { to: [9.0, 4.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[2] = PathSegment::Quad { ctrl: [9.0, 6.0], to: [7.0, 8.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[3] = PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 20.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.x = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.y = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.content = "different text".into();
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.size = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Image(image) = rich_child(value, 3) {
                image.height = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Trace(trace) = rich_child(value, 5) {
                trace.params.simplify_epsilon = 9.0;
            }
        },
        |value| {
            let DrawingLayerNode::Group(group) = value else { unreachable!() };
            group.children.swap(0, 1);
        },
    ];
    for modifier in modifiers {
        let mut value = baseline.clone();
        modifier(&mut value);
        variants.push(value);
    }

    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).visible = true;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).locked = false;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).opacity = 0.5;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).blend_mode = "screen".into();
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).transform.rotation = 0.75;
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::Solid { color: [0.9, 0.2, 0.3, 0.4] });
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::LinearGradient { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.8, 0.4] }] });
    variants.push(value);
    let mut value = baseline.clone();
    crate::schema::layer_base_mut(&mut value).attributes.stroke = Some(StrokeStyle { color: [0.9, 0.6, 0.7, 0.8], width: 3.0, cap: "square".into(), join: "round".into(), dash: Some(vec![2.0, 3.0]) });
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Path(path) = &mut group.children[1] {
            path.segments[4] = PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 18.0, large_arc: false, sweep: true, to: [20.0, 19.0] };
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Image(image) = &mut group.children[3] {
            image.image_key = "other-asset-reference".into();
            image.width = 2.0;
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Boolean(boolean) = &mut group.children[4] {
            boolean.operation = "subtract".into();
            boolean.children.swap(0, 1);
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Trace(trace) = &mut group.children[5] {
            trace.source_key = "other-trace-source".into();
            trace.params.threshold = 0.25;
        }
    }
    variants.push(value);

    for variant in variants {
        assert_ne!(create_digest(variant), baseline_digest, "every Drawing layer scalar, style, geometry, order, and asset reference changes the SHA-256 semantic authority");
    }

    let id = "layer".to_string();
    let all_payloads = [
        DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: id.clone(), visible: false }),
        DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: id.clone(), locked: true }),
        DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: id.clone(), opacity: 0.25 }),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: id.clone(), blend_mode: "screen".into() }),
        DrawingMutation::RenameLayer(RenameLayer { layer_id: id.clone(), new_name: "renamed".into() }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: id.clone(), transform: crate::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: id.clone(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: id.clone(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 2.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: id.clone(), boolean_operation: "intersect".into() }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: id.clone(), params: crate::DrawingTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(2), layer: Box::new(baseline.clone()) }),
        DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: id.clone() }),
        DrawingMutation::DeleteLayer(DeleteLayer { layer_id: id.clone() }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: id, parent_id: Some("parent".into()), index: 3 }),
    ];
    let mut digests = std::collections::HashSet::new();
    for payload in all_payloads {
        assert!(digests.insert(digest(&payload).expect("all fourteen Drawing mutation payloads hash distinctly")));
        drain_mutation(payload);
    }
    assert_mutation_digest_distinct(DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: false }), DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: true }));
    assert_mutation_digest_distinct(DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: false }), DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: true }));
    assert_mutation_digest_distinct(DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.25 }), DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.5 }));
    assert_mutation_digest_distinct(
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "multiply".into() }),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "screen".into() }),
    );
    assert_mutation_digest_distinct(DrawingMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "left".into() }), DrawingMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "right".into() }));
    assert_mutation_digest_distinct(
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: crate::DrawingTransform { x: 6.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0 } }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: None }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.3, 0.4] }] }) }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 9.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.75, color: [0.1, 0.2, 0.8, 0.4] }] }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: None }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: "round".into(), join: "bevel".into(), dash: Some(vec![1.0]) }) }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.9, 0.2, 0.3, 0.4], width: 2.0, cap: "square".into(), join: "round".into(), dash: Some(vec![2.0]) }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "union".into() }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "subtract".into() }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: crate::DrawingTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: crate::DrawingTraceParams { threshold: 0.75, simplify_epsilon: 1.5 } }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(baseline.clone()) }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(1), layer: Box::new(baseline.clone()) }),
    );
    assert_mutation_digest_distinct(DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: "left".into() }), DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: "right".into() }));
    assert_mutation_digest_distinct(DrawingMutation::DeleteLayer(DeleteLayer { layer_id: "left".into() }), DrawingMutation::DeleteLayer(DeleteLayer { layer_id: "right".into() }));
    assert_mutation_digest_distinct(
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: None, index: 0 }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: Some("parent".into()), index: 1 }),
    );
    drain_snapshot(DrawingSnapshot { layers: vec![baseline], ..crate::schema::default_drawing_document("digest-owner", None) });
}

#[test]
fn retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback() {
    let exact_source = nested_snapshot();
    let exact_owner = exact_source.layers.as_ptr();
    let exact_target = match exact_source.layers.last().expect("Drawing exact-boundary group") {
        DrawingLayerNode::Group(group) => crate::schema::layer_id(&group.children[0]).to_string(),
        _ => unreachable!("Drawing exact-boundary group remains exact"),
    };
    let exact = DrawingMutation::RenameLayer(RenameLayer { layer_id: exact_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES) });
    let exact_source = apply(exact_source, &exact).expect("an exact 4096-byte retained overlay page is admitted");
    assert_eq!(exact_source.layers.as_ptr(), exact_owner, "exact boundary publication retains the source container owner");
    drain_mutation(exact);
    drain_snapshot(exact_source);

    let plus_source = nested_snapshot();
    let plus_owner = plus_source.layers.as_ptr();
    let plus_target = match plus_source.layers.last().expect("Drawing +1 group") {
        DrawingLayerNode::Group(group) => crate::schema::layer_id(&group.children[0]).to_string(),
        _ => unreachable!("Drawing +1 group remains exact"),
    };
    let plus_one = DrawingMutation::RenameLayer(RenameLayer { layer_id: plus_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1) });
    let (plus_source, error) = apply(plus_source, &plus_one).expect_err("4096 +1 retained overlay page rejects");
    assert_eq!(error, "drawing-store.mutation-field-capacity");
    assert_eq!(plus_source.layers.as_ptr(), plus_owner, "+1 rejection returns the exact source owner without partial publication");
    drain_mutation(plus_one);
    drain_snapshot(plus_source);

    let mut source = crate::schema::default_drawing_document("aggregate-owner", None);
    let mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: crate::schema::layer_id(&source.layers[0]).into(), visible: false });
    let mut last_admitted = None;
    for index in 0..DRAWING_MUTATION_AGGREGATE_ITEMS {
        source.layers.push(crate::schema::create_drawing_path_layer(&format!("layer-{index}"), Vec::new()));
        let owner = source.layers.as_ptr();
        match live_reservation(&mut source, &mutation) {
            Ok(reservation) => {
                assert_eq!(source.layers.as_ptr(), owner, "live aggregate census never replaces the exact source owner");
                last_admitted = Some((source.layers.len(), reservation.total_items().expect("live item total"), reservation.total_bytes().expect("live byte total")));
            }
            Err("drawing-store.mutation-aggregate-item-capacity" | "drawing-store.mutation-aggregate-byte-capacity") => {
                assert_eq!(source.layers.as_ptr(), owner, "aggregate +1 rejection returns the exact source backing");
                break;
            }
            Err(error) => panic!("unexpected live Drawing aggregate rejection: {error}"),
        }
    }
    let (admitted_layers, admitted_items, admitted_bytes) = last_admitted.expect("at least one live aggregate owner is admitted");
    assert_eq!(source.layers.len(), admitted_layers + 1, "the first additional real layer owner is the +1 rejection");
    assert!(admitted_items <= DRAWING_MUTATION_AGGREGATE_ITEMS && admitted_bytes <= DRAWING_MUTATION_AGGREGATE_BYTES);
    let last_valid_id = source.id.clone();
    let (source, error) = apply(source, &mutation).expect_err("aggregate +1 rejects");
    assert_eq!(error, "drawing-store.mutation-aggregate-item-capacity");
    assert_eq!(source.id, last_valid_id, "aggregate rejection returns the exact source authority without partial publication");
    drain_mutation(mutation);
    drain_snapshot(source);
}

#[test]
fn retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision() {
    fn duplicate_id(id: &str, name: &str) -> String {
        let mut source = crate::schema::default_drawing_document("duplicate-framing", None);
        let mut layer = crate::schema::create_drawing_path_layer(name, Vec::new());
        let base = crate::schema::layer_base_mut(&mut layer);
        base.id.clear();
        base.id.push_str(id);
        admit_layer_string_destinations(&mut layer);
        source.layers = vec![layer];
        let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: id.into() });
        let source = apply(source, &mutation).expect("framed duplicate mutation applies");
        let duplicate = source.layers.get(1).map(crate::schema::layer_id).expect("duplicated layer remains retained").to_string();
        drain_mutation(mutation);
        drain_snapshot(source);
        duplicate
    }

    assert_ne!(duplicate_id("ab", "c"), duplicate_id("a", "bc"), "separate id/name length frames prevent concatenation collisions");
}

#[test]
fn retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner() {
    let mut source = crate::schema::default_drawing_document("duplicate-name-owner", None);
    let mut layer = crate::schema::create_drawing_path_layer("Layer", Vec::new());
    admit_layer_string_destinations(&mut layer);
    let target = crate::schema::layer_id(&layer).to_string();
    let original_name_owner = crate::schema::layer_base_mut(&mut layer).name.as_ptr();
    source.layers = vec![layer];
    let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: target });
    let source = apply(source, &mutation).expect("duplicate name suffix uses only pre-admitted destination and fixed scratch page");
    assert_eq!(crate::schema::layer_base(&source.layers[0]).name.as_ptr(), original_name_owner, "last-valid name backing remains exact");
    assert_eq!(crate::schema::layer_base(&source.layers[1]).name, "Layer copy");
    drain_mutation(mutation);
    drain_snapshot(source);

    let mut rejected = crate::schema::default_drawing_document("duplicate-name-rejected", None);
    let layer = crate::schema::create_drawing_path_layer("Layer", Vec::new());
    let target = crate::schema::layer_id(&layer).to_string();
    rejected.layers = vec![layer];
    let exact_owner = rejected.layers.as_ptr();
    let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: target });
    let (rejected, error) = apply(rejected, &mutation).expect_err("unadmitted duplicate destination rejects without allocating after operation admission");
    assert_eq!(error, "drawing-store.duplicate-destination-capacity");
    assert_eq!(rejected.layers.as_ptr(), exact_owner, "duplicate rejection returns the exact source container owner");
    drain_mutation(mutation);
    drain_snapshot(rejected);
}

#[test]
fn retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid() {
    let stages = [
        DrawingMutationCandidatePhase::PreflightSource,
        DrawingMutationCandidatePhase::PreflightMutation,
        DrawingMutationCandidatePhase::BindOverlay,
        DrawingMutationCandidatePhase::LocatePrimary,
        DrawingMutationCandidatePhase::LocateSecondary,
        DrawingMutationCandidatePhase::PrepareOwnedValue,
        DrawingMutationCandidatePhase::Apply,
        DrawingMutationCandidatePhase::RebuildSource,
        DrawingMutationCandidatePhase::LocateDestination,
        DrawingMutationCandidatePhase::RebuildDestination,
        DrawingMutationCandidatePhase::Complete,
    ];
    for stage in stages {
        for stale in [false, true] {
            let mut source = nested_snapshot();
            let (group_id, target) = match source.layers.last().expect("Drawing group") {
                DrawingLayerNode::Group(group) => (group.base.id.clone(), crate::schema::layer_id(&group.children[0]).to_string()),
                _ => unreachable!("Drawing fixture group remains exact"),
            };
            let last_valid_id = source.id.clone();
            let mutation = match stage {
                DrawingMutationCandidatePhase::LocateSecondary => DrawingMutation::CreateLayer(CreateLayer { parent_id: Some(group_id), index: Some(0), layer: Box::new(crate::schema::create_drawing_path_layer("cancel-create", Vec::new())) }),
                DrawingMutationCandidatePhase::RebuildSource | DrawingMutationCandidatePhase::LocateDestination => DrawingMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 }),
                _ => DrawingMutation::DuplicateLayer(DuplicateLayer { layer_id: target }),
            };
            let operation = semio_framework_job::OperationId(8_003);
            let generation = semio_framework_job::Generation(83);
            let mut authority = DrawingMutationCandidateAuthority::try_new(operation, generation).expect("Drawing candidate fixed owner arenas admit");
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview_sequence = 0;
            for _ in 0..100_000 {
                if authority.phase == stage {
                    break;
                }
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing stage fixture advances"));
            }
            assert_eq!(authority.phase, stage, "fixture reaches each replay/candidate/container stage exactly");
            if !stale {
                cancel.cancel_now();
            }
            let mut rejected = semio_framework_job::StepContext::new(
                operation,
                if stale { semio_framework_job::Generation(84) } else { generation },
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel,
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }),);
            close_candidate(&mut authority, Some(&mut source));
            drop(authority);
            assert_eq!(source.id, last_valid_id, "cancel/stale close never publishes a partial candidate");
            drain_mutation(mutation);
            drain_snapshot(source);
        }
    }
}
