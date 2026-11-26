//! 🧰️ Drawing retained mutation law kit: the string destination admission, the nested and byte bound snapshots, the drain, close, apply and digest helpers shared by the laws.

use super::*;
use semio_framework_value_clone_source_testing::FixtureSource;

fn admit_string_destination(value: &mut DrawingNativeText) {
    let length = value.len();
    let mut chunks = std::mem::take(value).into_retained_chunks();
    if chunks.is_empty() { chunks.push(String::with_capacity(DRAWING_OWNED_FIELD_BYTES)); }
    let first = chunks.get_mut(0).unwrap();
    if first.capacity() < DRAWING_OWNED_FIELD_BYTES { first.try_reserve_exact(DRAWING_OWNED_FIELD_BYTES.saturating_sub(first.len())).expect("native Drawing fixture chunk backing admits its exact destination extent"); }
    assert!(first.capacity() >= DRAWING_OWNED_FIELD_BYTES);
    *value = DrawingNativeText::from_retained_chunks(chunks, length).unwrap();
}

pub(super) fn reserve_native_layer_slots(value: &mut DrawingNativeLayers, minimum: usize) {
    while value.capacity() < minimum { value.reserve_one_cold(usize::MAX).unwrap_or_else(|_| panic!("cold native Drawing fixture page allocation admits")); }
}

pub(super) fn exact_single_layer(layer: DrawingLayerNode) -> DrawingNativeLayers {
    let mut layers = DrawingNativeLayers::default();
    while !layers.has_reserved_slot() {
        layers.reserve_exact_capacity_one_cold(1, usize::MAX).expect("one exact native layer slot");
    }
    assert!(layers.push_reserved(layer).is_ok());
    layers
}

pub(super) fn native_layer_backing(value: &DrawingNativeLayers) -> usize { value.first().map_or(0, |layer| layer as *const _ as usize) }

pub(super) fn native_text_backing(value: &semio_framework_value::paged::PagedUtf8<{usize::MAX}>) -> usize { value.retained_chunks().first().map_or(0, |chunk| chunk.as_ptr() as usize) }

pub(super) fn admit_layer_string_destinations(layer: &mut DrawingLayerNode) {
    let base = semio_s_artifact_draw_drawing_model::layer_base_mut(layer);
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

pub(super) fn initialize_drawing_mutation_arena_pool_for_test() {
    let operation = semio_framework_job::OperationId(7_900);
    let generation = semio_framework_job::Generation(79);
    let mut job = DrawingMutationArenaBootstrapJob::new(None).expect("fixed Drawing arena bootstrap job admission");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..1_000 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
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

pub(super) fn nested_snapshot() -> DrawingSnapshot {
    initialize_drawing_mutation_arena_pool_for_test();
    let mut snapshot = semio_s_artifact_draw_drawing_model::default_drawing_document("drawing-retained-mutation", None);
    let shape = semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape");
    let boolean = semio_s_artifact_draw_drawing_model::create_drawing_boolean_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Boolean")).to_string().into()).expect("nonempty authored identity"), "Boolean", "union", vec![semio_s_artifact_draw_drawing_model::layer_id(&shape).clone()].into());
    let trace = semio_s_artifact_draw_drawing_model::create_drawing_trace_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace", "asset-a");
    let mut group = semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(shape);
        value.children.push(boolean);
        value.children.push(trace);
    }
    snapshot.layers.push(group);
    snapshot.layers.iter_mut().for_each(admit_layer_string_destinations);
    snapshot.assets.insert("asset-a", DrawingImageAsset { width: 1, height: 1, samples: vec![[0,0,0,0];1].into() });
    snapshot
}

pub(super) fn document_over_byte_bound_snapshot() -> DrawingSnapshot {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::paged::PagedUtf8;
    use semio_framework_value_persistent::list::PagedList;
    let (mut snapshot, allocation) = observe(|| semio_s_artifact_draw_drawing_model::default_drawing_document("drawing-document-plus-one", None));
    assert!(!allocation.overflowed);
    let base = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    let (empty_title, allocation) = observe(|| PagedUtf8::<{usize::MAX}>::from_retained_chunks(PagedList::try_from_iter((0..64).map(|_| String::new())).unwrap(), 0).unwrap());
    assert!(!allocation.overflowed);
    let scaffold = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    drop(empty_title);
    let mut remaining = DRAWING_MAXIMUM_NESTED_BYTES.checked_add(1).unwrap().checked_sub(size_of::<DrawingSnapshot>() + base + scaffold).unwrap();
    let spare = remaining;
    let capacities: [usize; 64] = std::array::from_fn(|_| {
        let capacity = remaining.min(DRAWING_OWNED_FIELD_BYTES);
        remaining -= capacity;
        capacity
    });
    assert_eq!(remaining, 0);
    let (title, allocation) = observe(|| PagedUtf8::<{usize::MAX}>::from_retained_chunks(PagedList::try_from_iter(capacities.into_iter().map(String::with_capacity)).unwrap(), 0).unwrap());
    assert!(!allocation.overflowed);
    let retained = allocation.requested_bytes.checked_sub(allocation.released_bytes).unwrap();
    assert_eq!(retained, scaffold + spare);
    assert_eq!(size_of::<DrawingSnapshot>() + base + retained, DRAWING_MAXIMUM_NESTED_BYTES + 1, "independent allocator witness materializes the exact D + 1 retained-source boundary");
    snapshot.title = Some(title);
    snapshot
}

#[path = "../../🎟️retained-effect/🦀️.rs"]
mod fixture_effect;
pub(super) use fixture_effect::{effect_from_bank, effect_turn};

pub(super) fn close_decoded<T: semio_framework_value::retirement::RetireOwned>(owner: &mut DrawingDecodedFieldRetirement<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
    let body = grant.maximum_copy_bytes;
    let work = owner.owner.next_computational_work(body).unwrap().max(1);
    effect_turn(grant, work, |effect| {
        store::os_store::snapshot_read::settle_close_poll(owner.step(body, effect))
    })
}

fn drain_decoded<T: semio_framework_value::retirement::RetireOwned>(value: T) {
    let mut owner = DrawingDecodedFieldRetirement::try_new(value).unwrap_or_else(|_| panic!("Drawing original owns typed retirement"));
    for _ in 0..100_000 {
        if owner.terminal_is_empty() { return; }
        let grant = owner.next_grant().expect("Drawing owner retirement grant");
        close_decoded(&mut owner, grant).expect("Drawing original retirement");
    }
    panic!("Drawing original retirement did not terminate")
}

pub(super) fn drain_snapshot(value: DrawingSnapshot) { drain_decoded(value); }

pub(super) fn drain_mutation(value: DrawingMutation) { drain_decoded(value); }

/// 🚪️ Retries the bounded candidate closer until its original ownership and arena slots are returned.
pub(super) fn close_candidate(authority: &mut DrawingMutationCandidateAuthority, mut source: Option<&mut DrawingSnapshot>) {
    for _ in 0..100_000 {
        let demand = authority.close_demands(DRAWING_OWNED_FIELD_BYTES).expect("Drawing candidate demand");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let denied = RetainedCloneGrant { maximum_items: 0, ..grant };
        assert_eq!(effect_turn(denied, 0, |effect| authority.close_step(source.as_deref_mut(), effect)).unwrap().progress(), RetainedCloneProgress::default());
        let step = effect_turn(grant, DRAWING_OWNED_FIELD_BYTES as u64, |effect| authority.close_step(source.as_deref_mut(), effect)).expect("Drawing candidate close");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(authority.terminal_is_empty());
            return;
        }
        if step.progress() == RetainedCloneProgress::default() { std::thread::yield_now(); }
    }
    panic!("Drawing candidate close did not terminate")
}

/// 🧮️ Borrows a candidate from the shared process pool the way the production initializer does: a
/// transiently contended pool (another law holds it for one bounded borrow) is borrowed again.
pub(super) fn borrowed_candidate(replacement: Option<semio_framework_job::ReplacementIdentity>) -> Result<DrawingMutationCandidateAuthority, &'static str> {
    for _ in 0..1_000_000 {
        match DrawingMutationCandidateAuthority::try_new(replacement) {
            Err(DrawingMutationArenaBorrowError::Contended) => std::thread::yield_now(),
            other => return other.map_err(DrawingMutationArenaBorrowError::as_str),
        }
    }
    Err("drawing-store.mutation-arena-pool-contended")
}

/// 🧪️ Holds one mutation under original source custody, together with the sealed original snapshot a duplicate reads its clone source from.
pub(super) struct HeldMutation {
    mutation: FixtureSource<DrawingMutation>,
    base: Option<FixtureSource<DrawingSnapshot>>,
}

impl HeldMutation {
    pub(super) fn new(mutation: &DrawingMutation, source: &DrawingSnapshot) -> Self {
        let base = matches!(mutation, DrawingMutation::DuplicateLayer(_)).then(|| FixtureSource::from_owner(source.clone()));
        Self { mutation: FixtureSource::from_owner(mutation.clone()), base }
    }

    pub(super) fn step(&self, authority: &mut DrawingMutationCandidateAuthority, source: &mut DrawingSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        authority.step_with_custody(source, self.base.as_ref().map(|base| base.borrow()), self.mutation.borrow(), cx)
    }

    pub(super) fn step_replacement(&self, authority: &mut DrawingMutationCandidateAuthority, source: &mut DrawingSnapshot, current: semio_framework_job::ReplacementIdentity, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        cx.with_replacement_current(current, |cx| self.step(authority, source, cx))
    }
}

pub(super) fn apply(mut source: DrawingSnapshot, mutation: &DrawingMutation) -> Result<DrawingSnapshot, (DrawingSnapshot, &'static str)> {
    initialize_drawing_mutation_arena_pool_for_test();
    let operation = semio_framework_job::OperationId(8_001);
    let generation = semio_framework_job::Generation(81);
    let held = HeldMutation::new(mutation, &source);
    let mut authority = borrowed_candidate(None).expect("Drawing candidate fixed owner arenas admit");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..200_000 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(MUTATION_STEP_FUEL, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
        match held.step(&mut authority, &mut source, &mut context) {
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
    let phase = authority.phase;
    close_candidate(&mut authority, Some(&mut source));
    drop(authority);
    drain_snapshot(source);
    panic!("Drawing mutation candidate did not terminate from {phase:?}")
}

pub(super) fn live_workset(source: &mut DrawingSnapshot, mutation: &DrawingMutation) -> Result<DrawingMutationWorksetPlan, &'static str> {
    initialize_drawing_mutation_arena_pool_for_test();
    let operation = semio_framework_job::OperationId(8_004);
    let generation = semio_framework_job::Generation(84);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let held = HeldMutation::new(mutation, source);
    let mut authority = borrowed_candidate(None)?;
    for _ in 0..100_000 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(MUTATION_STEP_FUEL, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
        held.step(&mut authority, source, &mut context)?;
        if let Some(workset) = authority.workset {
            close_candidate(&mut authority, Some(source));
            drop(authority);
            return Ok(workset);
        }
    }
    close_candidate(&mut authority, Some(source));
    drop(authority);
    Err("drawing-store.test-mutation-preflight-incomplete")
}

pub(super) fn planned_clone_workset(source: &mut DrawingSnapshot, mutation: &DrawingMutation) -> Result<(DrawingMutationWorksetPlan, DrawingCloneWorkTotals), &'static str> {
    initialize_drawing_mutation_arena_pool_for_test();
    let operation = semio_framework_job::OperationId(8_006);
    let generation = semio_framework_job::Generation(86);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let held = HeldMutation::new(mutation, source);
    let mut authority = borrowed_candidate(None)?;
    for _ in 0..100_000 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(MUTATION_STEP_FUEL, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
        held.step(&mut authority, source, &mut context)?;
        if authority.phase == DrawingMutationCandidatePhase::BindOverlay {
            let workset = authority.workset.ok_or("drawing-store.test-clone-workset-missing")?;
            let totals = authority.clone_work.as_ref().and_then(DrawingLayerCloneWorkAuthority::totals).ok_or("drawing-store.test-clone-work-false-terminal")?;
            if authority.has_bound_overlay() {
                return Err("drawing-store.test-clone-bound-overlay-too-early");
            }
            close_candidate(&mut authority, Some(source));
            drop(authority);
            return Ok((workset, totals));
        }
    }
    close_candidate(&mut authority, Some(source));
    drop(authority);
    Err("drawing-store.test-clone-work-preflight-incomplete")
}

pub(super) fn digest(mutation: &DrawingMutation) -> Result<[u8; 32], &'static str> {
    let mut authority = DrawingMutationDigestAuthority::new();
    let mut output = store::ArtifactStoreInitializationDigest::new(b"drawing.test.mutation");
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(8_002),
            semio_framework_job::Generation(82),
            semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
            &mut context_retained,
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
                    let _ = authority.close_step();
                }
                drop(authority);
                return Err(error);
            }
        }
    }
    panic!("Drawing mutation digest did not terminate")
}

pub(super) fn rich_layer() -> DrawingLayerNode {
    let mut group = semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Digest Group")).to_string().into()).expect("nonempty authored identity"), "Digest Group");
    let base = semio_s_artifact_draw_drawing_model::layer_base_mut(&mut group);
    base.visible = false;
    base.locked = true;
    base.opacity = 0.75;
    base.blend_mode = "multiply".into();
    base.transform = semio_s_artifact_draw_drawing_model::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 0.5, shear: 0.75 };
    base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.25, color: [0.1, 0.2, 0.3, 0.4] }].into() });
    base.attributes.stroke = Some(StrokeStyle { color: [0.5, 0.6, 0.7, 0.8], width: 2.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Round, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Bevel, dash: Some(vec![1.0, 2.0].into()) });
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape"));
        value.children.push(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Path")).to_string().into()).expect("nonempty authored identity"), 
            "Path",
            vec![
                PathSegment::Move { to: [1.0, 2.0] },
                PathSegment::Line { to: [3.0, 4.0] },
                PathSegment::Quad { ctrl: [5.0, 6.0], to: [7.0, 8.0] },
                PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 14.0] },
                PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 17.0, large_arc: true, sweep: false, to: [18.0, 19.0] },
                PathSegment::Close,
            ].into(),
        ));
        value.children.push(semio_s_artifact_draw_drawing_model::create_drawing_text_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Text")).to_string().into()).expect("nonempty authored identity"), "Text"));
        value.children.push(semio_s_artifact_draw_drawing_model::create_drawing_image_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image", "asset-reference"));
        value.children.push(semio_s_artifact_draw_drawing_model::create_drawing_boolean_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Boolean")).to_string().into()).expect("nonempty authored identity"), "Boolean", "union", vec![semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::from("a"), semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::from("b")].into()));
        value.children.push(semio_s_artifact_draw_drawing_model::create_drawing_trace_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace", "trace-source"));
    }
    group
}

pub(super) fn create_digest(layer: DrawingLayerNode) -> [u8; 32] {
    let mutation = DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(3), layer: Box::new(layer) });
    let output = digest(&mutation).expect("rich Drawing create mutation hashes");
    drain_mutation(mutation);
    output
}

pub(super) fn assert_mutation_digest_distinct(left: DrawingMutation, right: DrawingMutation) {
    let left_digest = digest(&left).expect("left Drawing mutation hashes");
    let right_digest = digest(&right).expect("right Drawing mutation hashes");
    assert_ne!(left_digest, right_digest, "changing one Drawing mutation semantic field changes the retained SHA-256 authority");
    drain_mutation(left);
    drain_mutation(right);
}

pub(super) fn rich_child(layer: &mut DrawingLayerNode, index: usize) -> &mut DrawingLayerNode {
    let DrawingLayerNode::Group(group) = layer else { panic!("rich Drawing fixture root remains a Group") };
    group.children.get_mut(index).expect("rich Drawing fixture child")
}

pub(super) fn duplicate_assignments(document:&DrawingSnapshot,target:&DrawingNativeText)->semio_framework_value_persistent::list::PagedList<semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment,{usize::MAX}>{
 let node=semio_s_artifact_draw_drawing_model::find_drawing_layer(document,target).expect("authored duplicate source exists");let mut callback=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut callback);
 semio_s_artifact_draw_drawing_io::text::identity::clone::admit_clone_identities(node," copy",&mut control).expect("fixture duplicate identity admission")
}
