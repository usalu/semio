use super::*;

/// ♻️ Takes ownership of a faulting step outcome and returns its retained page to the ledger.
/// `RetainedJobPayload::drop` asserts one-page close (`🧵️job/🦀️.rs`), so a fault detail dropped on the
/// floor raises a second panic during unwinding and aborts the whole binary — every production caller
/// closes it, and so must every assertion that consumes one.
fn faulted(outcome: StepOutcome) -> bool {
    let StepOutcome::Fault(mut fault) = outcome else { return false };
    while !fault.detail.terminal_is_empty() {
        fault.detail.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    true
}

use crate::editor::puzzle3d::precompute::geometry::{collision_body_from_buffers, OwnerReservationLimit, DOCUMENT_CELL_MEMBER_SLOTS, DOCUMENT_CELL_SLOTS, DOCUMENT_OWNER_PAGE_BYTES, FIXED_OWNER_PAGE_BYTES, FIXED_OWNER_SLOTS};
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use crate::standards::v1::subsets::any::schema::{BrushKindWeights, Fixture, KindCatalogBundle, ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexProps};
use semio_framework_job::{root_cancel_token, Generation, OperationId, RevisionId, StepBudget};
use std::time::{Duration, Instant};

/// 🎯️ What a fresh editor asks for — the product default, never a planner ceiling.
const TEST_REQUESTED_COUNT: usize = 100;

fn empty_builder() -> FillBuilder {
    let scene = Arc::new(SceneConfig {
        fixture: Fixture::default(),
        kind_catalogs: Some(KindCatalogBundle::default()),
        kind_compatibility: Vec::new(),
        overlap_budget: 0.0,
        seed: 17,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(OperationId(1), RevisionId(1), Generation(1), 17), TEST_REQUESTED_COUNT)
}

fn test_context<'a>(builder: &FillBuilder, cancel: semio_framework_job::CancelToken, sequence: &'a mut u64) -> StepContext<'a> {
    fn now() -> Option<u64> {
        Some(0)
    }
    StepContext::new(builder.operation.operation, builder.operation.generation, StepBudget::new(100, 10), cancel, now, sequence)
}

#[test]
fn constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently() {
    #[derive(Clone, Copy)]
    enum HostileRoot {
        FixtureObjects,
        FixtureAttractions,
        FixtureTargetVolumes,
        Meshes,
        CatalogObjects,
        CatalogVortices,
        CatalogCables,
        KindCompatibility,
        ObjectWeights,
        VortexWeights,
    }
    let object = |index| FixtureObject { id: format!("object-{index:02}"), object_kind: None, anchor: Default::default(), mesh_url: None, origin: [0.0; 3], orientation: None, scale: None, vortices: Vec::new() };
    let body = collision_body_from_buffers(&[0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 4.0, 0.0], &[0, 1, 2]).expect("body");
    let roots = |branch: HostileRoot, count| {
        let mut scene =
            SceneConfig { fixture: Fixture::default(), kind_catalogs: Some(KindCatalogBundle::default()), kind_compatibility: Vec::new(), overlap_budget: 0.0, seed: 31, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        let mut meshes = HashMap::new();
        match branch {
            HostileRoot::FixtureObjects => scene.fixture.objects.extend((0..count).map(object)),
            HostileRoot::FixtureAttractions => scene.fixture.attractions.extend((0..count).map(|index| AttractionProps {
                id: format!("attraction-{index:02}"),
                attracting: format!("a-{index:02}"),
                attracted: format!("b-{index:02}"),
                gap: 0.0,
                shift: 0.0,
                rise: 0.0,
                rotation: 0.0,
                turn: 0.0,
                tilt: 0.0,
                x: 0.0,
                y: 0.0,
            })),
            HostileRoot::FixtureTargetVolumes => scene.fixture.target_volumes.extend((0..count).map(|index| WorldVolumeProps { id: format!("volume-{index:02}"), origin: [0.0; 3], orientation: None, scale: None })),
            HostileRoot::Meshes => meshes.extend((0..count).map(|index| (format!("mesh-{index:02}"), body.clone()))),
            HostileRoot::CatalogObjects => scene.kind_catalogs.as_mut().expect("catalogs").objects.extend((0..count).map(|index| ObjectKind { id: format!("catalog-object-{index:02}"), ..Default::default() })),
            HostileRoot::CatalogVortices => scene.kind_catalogs.as_mut().expect("catalogs").vortices.extend((0..count).map(|index| VortexKindCatalog { id: format!("catalog-vortex-{index:02}"), ..Default::default() })),
            HostileRoot::CatalogCables => scene.kind_catalogs.as_mut().expect("catalogs").cables.extend((0..count).map(|index| CableKindCatalog { id: format!("catalog-cable-{index:02}"), ..Default::default() })),
            HostileRoot::KindCompatibility => {
                scene.kind_compatibility.extend((0..count).map(|index| KindCompatEntry { source: format!("compat-{index:02}"), target: format!("target-{index:02}"), bidirectional: false, important: false, specificity: None }))
            }
            HostileRoot::ObjectWeights => scene.weights.object_weights.extend((0..count).map(|index| (format!("object-weight-{index:04}"), index as f64 + 0.25))),
            HostileRoot::VortexWeights => scene.weights.vortex_weights.extend((0..count).map(|index| (format!("vortex-weight-{index:04}"), index as f64 + 0.5))),
        }
        FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes))
    };
    let branches = [
        (HostileRoot::FixtureObjects, "fixture-objects", DOCUMENT_OBJECT_SLOTS),
        (HostileRoot::FixtureAttractions, "fixture-attractions", DOCUMENT_ATTRACTION_SLOTS),
        (HostileRoot::FixtureTargetVolumes, "fixture-target-volumes", DOCUMENT_VOLUME_SLOTS),
        (HostileRoot::Meshes, "meshes", DOCUMENT_KIND_SLOTS),
        (HostileRoot::CatalogObjects, "catalog-objects", DOCUMENT_KIND_SLOTS),
        (HostileRoot::CatalogVortices, "catalog-vortices", DOCUMENT_KIND_SLOTS),
        (HostileRoot::CatalogCables, "catalog-cables", DOCUMENT_KIND_SLOTS),
        (HostileRoot::KindCompatibility, "kind-compatibility", DOCUMENT_KIND_SLOTS),
        (HostileRoot::ObjectWeights, "object-weights", DOCUMENT_KIND_SLOTS),
        (HostileRoot::VortexWeights, "vortex-weights", DOCUMENT_KIND_SLOTS),
    ];
    for (offset, (branch, expected_branch, cap)) in branches.into_iter().enumerate() {
        let operation = Operation::new(OperationId(31 + offset as u64), RevisionId(1), Generation(1), 31);
        let mut accepted = FillBuilder::begin_preparation(roots(branch, cap), operation, TEST_REQUESTED_COUNT);
        let mut turns = 0;
        while accepted.stage != FillJobStage::PrepareTargets {
            accepted.prepare_one();
            turns += 1;
            assert!(turns < 16 * DOCUMENT_OBJECT_SLOTS, "{expected_branch} cap preparation must advance in bounded turns");
        }
        assert!(turns >= cap, "{expected_branch} cap must be installed cooperatively");

        let mut rejected = FillBuilder::begin_preparation(roots(branch, cap + 1), operation, TEST_REQUESTED_COUNT);
        let (actual_branch, exact_index, exact_owner, exact_weight) = rejected.preparation_refusal_owner_for_test().expect("attributable omitted owner");
        assert_eq!(actual_branch, expected_branch);
        assert_eq!(exact_index, cap);
        assert!(!exact_owner.is_empty());
        let omitted_object_weight = format!("object-weight-{cap:04}");
        let omitted_vortex_weight = format!("vortex-weight-{cap:04}");
        match branch {
            HostileRoot::ObjectWeights => assert_eq!((exact_owner.as_str(), exact_weight), (omitted_object_weight.as_str(), Some(cap as f64 + 0.25))),
            HostileRoot::VortexWeights => assert_eq!((exact_owner.as_str(), exact_weight), (omitted_vortex_weight.as_str(), Some(cap as f64 + 0.5))),
            _ => assert_eq!(exact_weight, None),
        }
        assert_eq!(
            (
                rejected.base.objects.len(),
                rejected.base.attractions.len(),
                rejected.base.target_volumes.len(),
                rejected.catalogs.objects.len(),
                rejected.catalogs.vortices.len(),
                rejected.catalogs.cables.len(),
                rejected.kind_compatibility.len(),
                rejected.meshes.len(),
                rejected.weights.object_weights.len(),
                rejected.weights.vortex_weights.len()
            ),
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        );
        let mut preview_sequence = 0;
        let mut context = test_context(&rejected, root_cancel_token(), &mut preview_sequence);
        let StepOutcome::Fault(fault) = rejected.step(&mut context) else { panic!("{expected_branch} cap + 1 faults the planner before it installs anything") };
        assert_eq!(fault.detail.single_page(), Some(format!("preparation-capacity:{expected_branch}:{cap}").as_bytes()));
        assert!(faulted(StepOutcome::Fault(fault)));
        assert!(faulted(rejected.step(&mut context)), "the refusal is permanent: every later step faults again");
        assert_eq!(
            (
                rejected.base.objects.len(),
                rejected.base.attractions.len(),
                rejected.base.target_volumes.len(),
                rejected.catalogs.objects.len(),
                rejected.catalogs.vortices.len(),
                rejected.catalogs.cables.len(),
                rejected.kind_compatibility.len(),
                rejected.meshes.len(),
                rejected.weights.object_weights.len(),
                rejected.weights.vortex_weights.len()
            ),
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        );
    }
}

#[test]
fn stale_generation_stops_preparation_before_installing_any_entry() {
    let mut builder = empty_builder();
    let before = (builder.base.objects.len(), builder.placed.len(), builder.placed_lookup.len());
    let mut sequence = 0;
    let mut context = StepContext::new(builder.operation.operation, Generation(builder.operation.generation.0 + 1), StepBudget::new(1, 1), root_cancel_token(), || Some(0), &mut sequence);
    assert!(faulted(builder.step(&mut context)));
    assert_eq!((builder.base.objects.len(), builder.placed.len(), builder.placed_lookup.len()), before);
}

#[test]
fn all_fill_fixed_collections_store_max_entries_in_the_credited_page_and_return_plus_one() {
    fn map_boundary<V>(mut value: impl FnMut(usize) -> V) {
        let mut map = FixedOwnerMap::<String, V>::new();
        let page = map.backing_ptr().expect("actual fixed page");
        let credit = map.backing_credit().expect("credited fixed page");
        assert_eq!(credit, (1, FixedOwnerMap::<String, V>::page_bytes()));
        assert!(credit.1 <= FIXED_OWNER_PAGE_BYTES);
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(matches!(map.try_insert(format!("key-{index:02}"), value(index)), Ok(FixedOwnerMapInsert::Inserted)));
        }
        let rejected = String::from("key-plus-one");
        let rejected_ptr = rejected.as_ptr();
        let Err((rejected, _)) = map.try_insert(rejected, value(FIXED_OWNER_SLOTS)) else { panic!("cap + 1 must reject") };
        assert_eq!(rejected.as_ptr(), rejected_ptr, "cap + 1 returns the identical key owner");
        assert_eq!(map.backing_ptr(), Some(page), "no second backing can be allocated");
        for _ in 0..FIXED_OWNER_SLOTS {
            drop(map.pop_first().expect("one semantic owner per close grant"));
            assert_eq!(map.backing_ptr(), Some(page));
        }
        assert!(map.retire_backing(), "the same actual slot page returns after semantic owners");
        assert!(map.terminal_owners_empty());
    }

    fn set_boundary() {
        let mut set = FixedOwnerSet::<String>::new();
        let page = set.backing_ptr().expect("actual fixed page");
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(matches!(set.try_insert(format!("set-{index:02}")), Ok(FixedOwnerSetInsert::Inserted)));
        }
        let rejected = String::from("set-plus-one");
        let rejected_ptr = rejected.as_ptr();
        let Err(rejected) = set.try_insert(rejected) else { panic!("cap + 1 must reject") };
        assert_eq!(rejected.as_ptr(), rejected_ptr, "cap + 1 returns the identical set owner");
        assert_eq!(set.backing_ptr(), Some(page));
        for _ in 0..FIXED_OWNER_SLOTS {
            drop(set.pop_first().expect("one semantic owner per close grant"));
        }
        assert!(set.retire_backing());
        assert!(set.terminal_owners_empty());
    }

    fn vec_boundary() {
        let mut values = FixedOwnerVec::<String>::new();
        let page = values.backing_ptr().expect("actual fixed vector page");
        for index in 0..FIXED_OWNER_SLOTS {
            assert!(values.try_push(format!("vector-{index:02}")).is_ok());
        }
        let rejected = String::from("vector-plus-one");
        let rejected_ptr = rejected.as_ptr();
        let Err(rejected) = values.try_push(rejected) else { panic!("vector cap + 1 must reject") };
        assert_eq!(rejected.as_ptr(), rejected_ptr, "cap + 1 returns the exact omitted vector owner");
        assert_eq!(values.backing_ptr(), Some(page));
        for _ in 0..FIXED_OWNER_SLOTS {
            drop(values.pop().expect("one semantic vector owner per close grant"));
        }
        assert!(values.retire_backing());
        assert!(values.terminal_owners_empty());
    }

    map_boundary(|index| index);
    map_boundary(|index| vec![BrushCompatibleCandidate { object_kind_id: format!("cache-{index}"), source_vortex_index: index }]);
    set_boundary();
    map_boundary(|index| index as f64);
    map_boundary(|index| index as f64);
    let body = collision_body_from_buffers(&[0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 4.0, 0.0], &[0, 1, 2]).expect("body");
    map_boundary(|_| body.clone());
    set_boundary();
    set_boundary();
    map_boundary(|index| BrushCompatibleCandidate { object_kind_id: format!("cross-{index}"), source_vortex_index: index });
    map_boundary(|index| BrushCompatibleCandidate { object_kind_id: format!("same-{index}"), source_vortex_index: index });
    vec_boundary();

    let mut cache = FixedOwnerMap::<String, Vec<BrushCompatibleCandidate>>::new();
    for index in 0..FIXED_OWNER_SLOTS {
        assert!(matches!(cache.try_insert(format!("cache-{index:02}"), Vec::new()), Ok(FixedOwnerMapInsert::Inserted)));
    }
    let rejected_key = String::from("cache-plus-one");
    let rejected_key_ptr = rejected_key.as_ptr();
    let rejected_value = vec![BrushCompatibleCandidate { object_kind_id: "identical-value".into(), source_vortex_index: 0 }];
    let rejected_value_ptr = rejected_value.as_ptr();
    let rejected_nested_ptr = rejected_value[0].object_kind_id.as_ptr();
    let Err((rejected_key, rejected_value)) = cache.try_insert(rejected_key, rejected_value) else { panic!("cache cap + 1") };
    assert_eq!(rejected_key.as_ptr(), rejected_key_ptr);
    assert_eq!(rejected_value.as_ptr(), rejected_value_ptr, "cap + 1 returns the identical nested value owner");
    assert_eq!(rejected_value[0].object_kind_id.as_ptr(), rejected_nested_ptr);
    drop(rejected_key);
    drop(rejected_value);
    for _ in 0..FIXED_OWNER_SLOTS {
        drop(cache.pop_first().expect("one retained cache entry per close grant"));
    }
    assert!(cache.retire_backing());
    assert!(cache.terminal_owners_empty());
}

#[test]
fn occupied_fixed_slot_returns_the_distinct_input_owners_without_replacing_stored_owners() {
    let mut map = FixedOwnerMap::<String, Vec<String>>::new();
    let mut stored_key = String::with_capacity(64);
    stored_key.push_str("equal-key");
    let stored_key_ptr = stored_key.as_ptr();
    let stored_value = vec![String::from("stored-value")];
    let stored_value_ptr = stored_value.as_ptr();
    assert!(matches!(map.try_insert(stored_key, stored_value), Ok(FixedOwnerMapInsert::Inserted)));

    let mut input_key = String::with_capacity(256);
    input_key.push_str("equal-key");
    let input_key_ptr = input_key.as_ptr();
    let input_value = vec![String::from("input-value")];
    let input_value_ptr = input_value.as_ptr();
    let Ok(FixedOwnerMapInsert::Occupied { input_key, input_value }) = map.try_insert(input_key, input_value) else { panic!("equal key must return a typed occupied outcome") };
    assert_eq!(input_key.as_ptr(), input_key_ptr);
    assert_eq!(input_value.as_ptr(), input_value_ptr);
    let (retained_key, retained_value) = map.iter().next().expect("stored owner remains retained");
    assert_eq!(retained_key.as_ptr(), stored_key_ptr);
    assert_eq!(retained_value.as_ptr(), stored_value_ptr);

    drop(input_key);
    drop(input_value);
    drop(map.pop_first().expect("stored pair retires as one semantic owner"));
    assert!(map.retire_backing(), "actual page retires only after its stored pair");
    assert!(map.terminal_owners_empty());
}

#[test]
fn cancellation_is_observed_before_the_next_transition() {
    let mut builder = empty_builder();
    let cancel = root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut context = test_context(&builder, cancel, &mut sequence);
    assert_eq!(builder.step(&mut context), StepOutcome::Cancelled);
}

#[test]
fn stale_generation_faults_without_progress() {
    fn now() -> Option<u64> {
        Some(0)
    }
    let mut builder = empty_builder();
    let mut sequence = 0;
    let mut context = StepContext::new(OperationId(builder.operation.operation.0), Generation(builder.operation.generation.0 + 1), StepBudget::new(100, 10), root_cancel_token(), now, &mut sequence);
    let StepOutcome::Fault(fault) = builder.step(&mut context) else { panic!("a stale generation must fault") };
    assert_eq!(fault.detail.single_page(), Some(b"stale-fill-operation".as_slice()));
    assert!(faulted(StepOutcome::Fault(fault)));
    assert_eq!(builder.operation.base_revision, RevisionId(1));
}

/// 📏️ Turns an EMPTY scene's cursorized planner needs to reach `Complete`: seven preparation stages
/// (three of them walking three empty roots apiece), then target preparation and the stall, one
/// bounded unit per turn. Doubled as headroom for a stage split.
const EMPTY_FILL_TRANSITION_TURNS: usize = 64;

#[test]
fn empty_fill_transition_stays_below_watchdog_ceiling() {
    let mut builder = empty_builder();
    let mut sequence = 0;
    for _ in 0..EMPTY_FILL_TRANSITION_TURNS {
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let started = Instant::now();
        let _ = builder.step(&mut context);
        assert!(started.elapsed() < Duration::from_millis(8));
        if matches!(builder.stage, FillJobStage::Complete(_)) {
            break;
        }
    }
    assert!(matches!(builder.stage, FillJobStage::Complete(_)));
}

#[test]
fn adversarial_broad_phase_fill_is_end_to_end_resumable_below_eight_ms() {
    let representation = |id: &str| ObjectKindRepresentation { id: id.into(), name: String::new(), url: "/stress/box.glb".into(), mime: String::new(), tags: Vec::new(), lod: None, description: String::new() };
    let candidate_vortex = ObjectKindVortexTemplate { vortex_kind: Some("port-a".into()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() };
    let catalogs = KindCatalogBundle {
        objects: vec![
            ObjectKind { id: "Host".into(), representations: vec![representation("host")], scale: None, vortices: Vec::new() },
            ObjectKind { id: "Obstacle".into(), representations: vec![representation("obstacle")], scale: None, vortices: Vec::new() },
            ObjectKind { id: "Placed".into(), representations: vec![representation("placed")], scale: None, vortices: vec![candidate_vortex] },
        ],
        vortices: Vec::new(),
        cables: Vec::new(),
    };
    let host = FixtureObject {
        id: "host".into(),
        object_kind: Some("Host".into()),
        anchor: Default::default(),
        mesh_url: Some("/stress/box.glb".into()),
        origin: [0.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        vortices: vec![VortexProps { id: "v0".into(), vortex_kind: Some("port-a".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
    };
    let mut objects = vec![host];
    objects.extend((0..30).map(|index| FixtureObject {
        id: format!("obstacle-{index:04}"),
        object_kind: Some("Obstacle".into()),
        anchor: Default::default(),
        mesh_url: Some("/stress/box.glb".into()),
        origin: [10_000.0 + index as f64 * 16.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        vortices: Vec::new(),
    }));
    let positions = [-4.0, -4.0, 0.0, 4.0, -4.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 8.0];
    let indices = [0, 1, 2, 0, 1, 3, 1, 2, 3, 2, 0, 3];
    let body = collision_body_from_buffers(&positions, &indices).expect("stress body");
    let meshes = HashMap::from([("/stress/box.glb".to_string(), body)]);
    let scene = Arc::new(SceneConfig {
        fixture: Fixture { objects, attractions: Vec::new(), target_volumes: Vec::new() },
        kind_catalogs: Some(catalogs),
        kind_compatibility: Vec::new(),
        overlap_budget: 0.0,
        seed: 29,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    let mut builder = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(meshes)), Operation::new(OperationId(29), RevisionId(1), Generation(1), 29), TEST_REQUESTED_COUNT);
    let mut sequence = 0;
    let started = Instant::now();
    let mut first_candidate = None;
    let mut max_step = Duration::ZERO;
    for _ in 0..50_000 {
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let step_started = Instant::now();
        let outcome = builder.step(&mut context);
        let step_elapsed = step_started.elapsed();
        max_step = max_step.max(step_elapsed);
        assert!(step_elapsed < Duration::from_millis(8), "stage {:?} reached the 8ms ceiling", builder.stage);
        if first_candidate.is_none() && builder.current_preview.is_some() {
            first_candidate = Some(started.elapsed());
        }
        if outcome.is_terminal() {
            break;
        }
    }
    assert!(first_candidate.is_some_and(|elapsed| elapsed < Duration::from_millis(50)), "adversarial fill did not publish its first candidate within 50ms: {first_candidate:?}");
    assert!(matches!(builder.stage, FillJobStage::Complete(_)));
    assert_eq!(builder.sequence.len(), 1);
}

#[test]
fn document_capacities_match_the_language_neutral_capacity_law() {
    let law: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("language-neutral fill run law");
    let capacities = &law["laws"]["documentCapacities"];
    let declared = |field: &str| capacities[field].as_u64().unwrap_or_else(|| panic!("{field} capacity")) as usize;
    assert_eq!([declared("bookkeepingSlots"), declared("bookkeepingPageBytes"), declared("documentPageBytes")], [FIXED_OWNER_SLOTS, FIXED_OWNER_PAGE_BYTES, DOCUMENT_OWNER_PAGE_BYTES]);
    assert_eq!(
        [declared("objectSlots"), declared("attractionSlots"), declared("vortexSlots"), declared("volumeSlots"), declared("kindSlots"), declared("candidateSlots"), declared("cellSlots"), declared("cellMemberSlots")],
        [DOCUMENT_OBJECT_SLOTS, DOCUMENT_ATTRACTION_SLOTS, DOCUMENT_VORTEX_SLOTS, DOCUMENT_VOLUME_SLOTS, DOCUMENT_KIND_SLOTS, DOCUMENT_CANDIDATE_SLOTS, DOCUMENT_CELL_SLOTS, DOCUMENT_CELL_MEMBER_SLOTS]
    );
    let nakagin = |field: &str| capacities["nakagin"][field].as_u64().unwrap_or_else(|| panic!("nakagin {field}")) as usize;
    assert!(nakagin("objects") < DOCUMENT_OBJECT_SLOTS, "the flagship fixture leaves the object capacity room to plan into");
    assert!(nakagin("attractions") < DOCUMENT_ATTRACTION_SLOTS);
    assert!(nakagin("vortices") <= DOCUMENT_VORTEX_SLOTS && nakagin("objects") <= nakagin("vortices"), "the measured vortices-per-object ratio backs the vortex capacity");
    assert!(nakagin("objectKinds").max(nakagin("vortexKinds")).max(nakagin("compatibilityRows")) <= DOCUMENT_KIND_SLOTS);
    assert!(DOCUMENT_CELL_SLOTS > FIXED_OWNER_SLOTS && DOCUMENT_OBJECT_SLOTS > FIXED_OWNER_SLOTS, "document capacities are never the bookkeeping batch");
}

const NAKAGIN_OBJECTS: usize = 180;
const NAKAGIN_OBJECT_KINDS: usize = 12;
const NAKAGIN_VORTEX_KINDS: usize = 18;
const NAKAGIN_COMPATIBILITY_ROWS: usize = 14;
const NAKAGIN_MESH_URL: &str = "/nakagin/capsule.glb";

/// 🏢️ The flagship fixture at document scale — 180 capsules, 360 attractions, 12 object kinds, 18
/// vortex kinds, 14 compatibility rows — as the fill lane's own preparation roots. Two laws drive
/// it: one on an unconstrained guest, one under a fragmented guest's reservation ceiling.
fn nakagin_scale_roots() -> FillPreparationRoots {
    let template = ObjectKindVortexTemplate { vortex_kind: Some("port-00".into()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() };
    let catalogs = KindCatalogBundle {
        objects: (0..NAKAGIN_OBJECT_KINDS)
            .map(|index| ObjectKind {
                id: format!("capsule-kind-{index:02}"),
                representations: vec![ObjectKindRepresentation { id: format!("capsule-representation-{index:02}"), name: String::new(), url: NAKAGIN_MESH_URL.into(), mime: String::new(), tags: Vec::new(), lod: None, description: String::new() }],
                scale: None,
                vortices: vec![template.clone()],
            })
            .collect(),
        vortices: (0..NAKAGIN_VORTEX_KINDS).map(|index| VortexKindCatalog { id: format!("port-{index:02}"), ..Default::default() }).collect(),
        cables: Vec::new(),
    };
    let objects: Vec<FixtureObject> = (0..NAKAGIN_OBJECTS)
        .map(|index| FixtureObject {
            id: format!("capsule-{index:03}"),
            object_kind: Some(format!("capsule-kind-{:02}", index % NAKAGIN_OBJECT_KINDS)),
            anchor: Default::default(),
            mesh_url: Some(NAKAGIN_MESH_URL.into()),
            origin: [(index % 12) as f64 * 64.0, (index / 12) as f64 * 64.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            vortices: vec![
                VortexProps { id: "v0".into(), vortex_kind: Some("port-00".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) },
                VortexProps { id: "v1".into(), vortex_kind: Some("port-00".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]) },
            ],
        })
        .collect();
    let attraction = |index: usize, vortex: &str| AttractionProps {
        id: format!("cable-{vortex}-{index:03}"),
        attracting: puzzle3d_vortex_full_id(&format!("capsule-{index:03}"), vortex),
        attracted: puzzle3d_vortex_full_id(&format!("capsule-{:03}", (index + 1) % NAKAGIN_OBJECTS), vortex),
        gap: 0.0,
        shift: 0.0,
        rise: 0.0,
        rotation: 0.0,
        turn: 0.0,
        tilt: 0.0,
        x: 0.0,
        y: 0.0,
    };
    let attractions: Vec<AttractionProps> = (0..NAKAGIN_OBJECTS).map(|index| attraction(index, "connected-a")).chain((0..NAKAGIN_OBJECTS).map(|index| attraction(index, "connected-b"))).collect();
    let kind_compatibility: Vec<KindCompatEntry> = (0..NAKAGIN_COMPATIBILITY_ROWS)
        .map(|index| KindCompatEntry { source: format!("port-{index:02}"), target: format!("port-{index:02}"), bidirectional: true, important: false, specificity: Some("vortex".into()) })
        .collect();
    let body = collision_body_from_buffers(&[-4.0, -4.0, 0.0, 4.0, -4.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 8.0], &[0, 1, 2, 0, 1, 3, 1, 2, 3, 2, 0, 3]).expect("capsule body");
    let scene = Arc::new(SceneConfig {
        fixture: Fixture { objects, attractions, target_volumes: Vec::new() },
        kind_catalogs: Some(catalogs),
        kind_compatibility,
        overlap_budget: 0.0,
        seed: 43,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    FillPreparationRoots::new(scene, Arc::new(HashMap::from([(NAKAGIN_MESH_URL.to_string(), body)])))
}

/// 🏢️ Drives a document-scale preparation until it places its first object, refusing to accept a
/// fault, a capacity refusal, or a terminal outcome that placed nothing.
fn drive_nakagin_scale_fill() -> FillBuilder {
    let mut builder = FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(43), RevisionId(1), Generation(1), 43), TEST_REQUESTED_COUNT);
    assert!(builder.preparation_capacity_refusal.is_none(), "a Nakagin-scale document must not be refused before preparation starts");
    let mut sequence = 0;
    let mut turns = 0;
    while builder.sequence.is_empty() {
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let outcome = builder.step(&mut context);
        turns += 1;
        let terminal = outcome.is_terminal();
        assert!(!faulted(outcome), "Nakagin-scale fill faulted at stage {:?} after {turns} turns: {:?}", builder.stage, builder.last_rejection);
        assert!(!terminal || !builder.sequence.is_empty(), "Nakagin-scale fill ended after {turns} turns without placing an object: {:?}", builder.last_rejection);
        assert!(turns < 400_000, "Nakagin-scale fill did not place an object in bounded turns");
    }
    builder
}

#[test]
fn nakagin_scale_fill_is_not_refused_and_places_at_least_one_object() {
    let builder = drive_nakagin_scale_fill();
    assert_eq!((builder.base.objects.len(), builder.base.attractions.len()), (NAKAGIN_OBJECTS, 2 * NAKAGIN_OBJECTS));
    assert_eq!((builder.catalogs.objects.len(), builder.catalogs.vortices.len(), builder.kind_compatibility.len()), (NAKAGIN_OBJECT_KINDS, NAKAGIN_VORTEX_KINDS, NAKAGIN_COMPATIBILITY_ROWS));
    assert_eq!(builder.placed_lookup.len(), NAKAGIN_OBJECTS + 1);
    assert_eq!(builder.appended_objects.len(), 1);
}

/// ⚖️ LAW: a fill session on a guest that refuses every contiguous request over
/// `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` still prepares a Nakagin-scale document and places an
/// object — no owner of this lane asks for a single block the fragmented guest cannot serve.
///
/// 🧊️ A wasm guest runs on one linear memory that grows and never shrinks, served by `dlmalloc` with
/// a 64 KiB granularity: the FIRST request a fragmented or nearly-full guest refuses is one larger
/// than a granularity unit. The fill session's own owners used to be exactly that — one 432 KiB
/// block for `FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>`, one 96 KiB block for the
/// collision entry map — so ~44 s into a Nakagin fill run the guest refused them and the whole plan
/// was abandoned (`CollisionMutationStep::Rejected(Capacity)`, ticket 26/09/02 build #29 and W-F6
/// §8 item 2). A native suite cannot exhaust a 512 MiB linear memory, so the law installs the
/// fragmented guest as a reservation policy instead.
#[test]
fn nakagin_scale_fill_places_an_object_under_a_fragmented_guest_reservation_ceiling() {
    let _fragmented = OwnerReservationLimit::install(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, usize::MAX);
    let builder = drive_nakagin_scale_fill();
    assert_eq!((builder.base.objects.len(), builder.base.attractions.len()), (NAKAGIN_OBJECTS, 2 * NAKAGIN_OBJECTS));
    assert_eq!(builder.appended_objects.len(), 1, "the fragmented guest still places its first object");
    assert!(builder.preparation_capacity_refusal.is_none());
}

/// 🔁️ Drives a planner until it settles — either it reached what was asked for or it named a stall.
fn drive_until_settled(builder: &mut FillBuilder, turns: usize) {
    let mut sequence = 0;
    for _ in 0..turns {
        if matches!(builder.stage, FillJobStage::Complete(_)) {
            return;
        }
        let mut context = test_context(builder, root_cancel_token(), &mut sequence);
        let outcome = builder.step(&mut context);
        assert!(!faulted(outcome), "a settled planner never faults: {:?} / end {:?}", builder.last_rejection, builder.end());
    }
    panic!("planner did not settle in {turns} turns at stage {:?} with {} placements", builder.stage, builder.sequence.len());
}

fn nakagin_plan(requested: usize, turns: usize) -> FillBuilder {
    let mut builder = FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(61), RevisionId(1), Generation(1), 43), requested);
    drive_until_settled(&mut builder, turns);
    builder
}

/// ⚖️ LAW (the dev's fill procedure, ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS 2026-09-14): the planner draws a free
/// vortex by distribution and tests compatible candidates there — after a refusal the next candidate is tested at the SAME
/// vortex — until one fits (placed, its vortex consumed) or none is left (the vortex is marked and never drawn again). It
/// stalls only once every vortex of the pool is consumed or marked, keeping every placement made so far.
#[test]
fn fill_plan_retries_one_drawn_vortex_until_it_places_or_is_marked_and_never_draws_a_marked_vortex() {
    let (roots, _, _) = example_fill_roots("nakagin", 1);
    let mut builder = FillBuilder::begin_preparation(roots, Operation::new(OperationId(101), RevisionId(1), Generation(1), 1), 1_000_000);
    builder.observe_run();
    let (mut sequence, mut events) = (0, Vec::new());
    let (mut last_target, mut open): (Option<String>, Option<String>) = (None, None);
    let mut marked = std::collections::HashSet::new();
    let (mut placements, mut retries) = (0usize, 0usize);
    for _ in 0..50_000_000 {
        if matches!(builder.stage, FillJobStage::Complete(_)) {
            break;
        }
        let mut context = test_context(&builder, root_cancel_token(), &mut sequence);
        let outcome = builder.step(&mut context);
        assert!(!faulted(outcome), "the planner never faults: {:?}", builder.last_rejection);
        let current = builder.current_target.as_ref().map(|target| target.full_id.clone());
        builder.swap_run_events(&mut events);
        for event in &events {
            match event {
                FillRunEvent::Constructed { .. } => {
                    let target = current.clone().or_else(|| last_target.clone()).expect("a candidate is constructed at a drawn vortex");
                    assert!(!marked.contains(&target), "the marked vortex {target} is never drawn again");
                    if let Some(open) = &open {
                        assert_eq!(open, &target, "a refused candidate is followed by another candidate at the same vortex");
                        retries += 1;
                    }
                    open = Some(target);
                }
                FillRunEvent::Accepted => {
                    open = None;
                    placements += 1;
                }
                FillRunEvent::VortexMarked { .. } => {
                    let target = last_target.clone().expect("a marked vortex was drawn");
                    assert!(marked.insert(target), "a vortex is marked once");
                    open = None;
                }
                _ => {}
            }
        }
        if current.is_some() {
            last_target = current;
        }
    }
    assert_eq!(builder.end(), Some(FillPlanEnd::Stalled(FillStall::NoFreePlacement)), "a million requested objects on Nakagin end in the no-free-placement stall");
    assert_eq!(builder.sequence.len(), placements, "the partial plan keeps every placement");
    assert!(placements > 0 && !marked.is_empty() && retries > 0, "the law needs placements ({placements}), marks ({}) and retries at one vortex ({retries})", marked.len());
    assert!(builder.target_weights.iter().all(|weight| *weight == 0.0), "the plan stalls only once no pool vortex can be drawn");
    assert_eq!(builder.targets.len(), placements + marked.len(), "every pool vortex ended consumed by exactly one placement or marked");
}

fn plan_identity(builder: &FillBuilder) -> Vec<(String, String, usize)> {
    builder.sequence.iter().map(|payload| (payload.target_vortex_full_id.clone(), payload.object_kind_id.clone(), payload.source_vortex_index)).collect()
}

/// ⚖️ LAW: asking for more never rewinds the RNG stream — the longer plan keeps the shorter one as
/// its exact prefix, so raising the count continues the placements the user already sees instead of
/// replanning them somewhere else.
#[test]
fn raising_the_requested_count_continues_the_plan_as_an_exact_prefix() {
    const SHORT: usize = 100;
    const LONG: usize = 150;
    let mut raised = nakagin_plan(SHORT, 4_000_000);
    assert_eq!(raised.sequence.len(), SHORT, "the short plan stops exactly at what was asked for");
    assert_eq!(raised.requested_count(), SHORT);
    let short_identity = plan_identity(&raised);
    let short_rng = raised.rng_state;

    raised.set_requested_count(LONG);
    assert_eq!((raised.requested_count(), raised.stage, raised.end()), (LONG, FillJobStage::SelectTarget, None), "a completed planner wakes up at target selection over its resident vortex pool");
    assert_eq!(raised.rng_state, short_rng, "raising never rewinds the stream");
    drive_until_settled(&mut raised, 4_000_000);
    assert_eq!(raised.sequence.len(), LONG);

    let long = nakagin_plan(LONG, 4_000_000);
    assert_eq!(long.sequence.len(), LONG);
    assert_eq!(plan_identity(&raised), plan_identity(&long), "the resumed plan and the one long plan are the same sequence");
    assert_eq!(&plan_identity(&long)[..SHORT], short_identity.as_slice(), "the short plan is the long plan's prefix");
}

/// ⚖️ LAW: asking for less hands the surplus tail back — the plan itself shrinks, each discarded
/// placement withdraws its own collision owner, and asking for more again continues from there.
#[test]
fn lowering_the_requested_count_discards_the_planned_tail_and_raising_continues() {
    const PLANNED: usize = 100;
    const LOWERED: usize = 60;
    const RAISED: usize = 80;
    let mut builder = nakagin_plan(PLANNED, 4_000_000);
    assert_eq!(builder.sequence.len(), PLANNED);
    let owners_before = builder.placed_lookup.len();

    builder.set_requested_count(LOWERED);
    assert_eq!((builder.requested_count(), builder.stage), (LOWERED, FillJobStage::RetractTail));
    drive_until_settled(&mut builder, 4_000_000);
    assert_eq!((builder.sequence.len(), builder.appended_objects.len(), builder.appended_attractions.len()), (LOWERED, LOWERED, LOWERED));
    assert_eq!(builder.placed_lookup.len(), owners_before - (PLANNED - LOWERED), "every discarded placement withdrew its own spatial owner");
    assert_eq!(builder.stage, FillJobStage::Complete(FillPlanEnd::Reached), "a plan that already holds what was asked for stops there");

    builder.set_requested_count(RAISED);
    drive_until_settled(&mut builder, 4_000_000);
    assert_eq!(builder.sequence.len(), RAISED, "raising after a discard keeps planning instead of stalling");
}

//#region ⏯️FillRunJob
use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle3dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use semio_framework_tool_run::{ToolRunId, ToolRunTraceCursor, ToolRunTraceStore, TOOL_RUN_TRACE_PAGE_BYTES_MAX};

const FILL_RUN_FIXTURE: &str = include_str!("../../🧫️fixtures/🎞️fill-run.json");
const FILL_RUN_BOX_SCALE: f32 = 4.0;

fn fill_run_identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 7, run: 1 }, [3; 32])
}

fn never() -> Option<u64> {
    Some(0)
}

/// 🏙️ A shipped example document through the app's own scene bridge, every mesh identity backed by the
/// app's scaled box fallback; answers the roots, the sorted mesh lane and each mesh's raw positions.
fn example_fill_roots(document: &str, seed: u32) -> (FillPreparationRoots, Vec<String>, Vec<f32>) {
    let text = match document {
        "nakagin" => crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT,
        "concrete-forest" => crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT,
        other => panic!("unknown example document {other}"),
    };
    let snapshot = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example parses");
    let envelope = crate::editor::puzzle3d::scene_from_snapshot(&snapshot, Default::default(), "fill");
    let mut scene: SceneConfig = dsl::FromValue::from_value(crate::editor::puzzle3d::scene_config_value(&envelope)).expect("scene config decodes");
    scene.seed = seed;
    let fallback = semio_framework_plugin::mesh_from_kind(crate::editor::puzzle3d::PUZZLE3D_FALLBACK_MESH_KIND);
    let positions: Vec<f32> = fallback.positions.iter().map(|value| value * FILL_RUN_BOX_SCALE).collect();
    let body = collision_body_from_buffers(&positions, &fallback.indices).expect("fallback body");
    let mut lane = crate::editor::puzzle3d::collect_mesh_urls(&envelope.fixture);
    lane.push(crate::editor::puzzle3d::PUZZLE3D_FALLBACK_MESH_KIND.to_string());
    lane.sort();
    lane.dedup();
    let meshes = lane.iter().map(|url| (url.clone(), body.clone())).collect();
    (FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes)), lane, positions)
}

fn fill_run_job(roots: FillPreparationRoots, lane: Vec<String>, seed: u64, requested: usize) -> FillRunJob {
    FillRunJob::new(FillBuilder::begin_preparation(roots, Operation::new(OperationId(71), RevisionId(1), Generation(1), seed), requested), fill_run_identity(), lane, [0; 32])
}

fn close_payload(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

/// 🚦️ What one run job turn handed its driver, with every retained page returned to its ledger.
#[derive(Debug)]
enum FillRunTurn {
    Tick(ToolRunTick),
    Checkpoint(Vec<u8>),
    Complete,
    Yield,
}

fn settle_fill_run_outcome(outcome: StepOutcome) -> FillRunTurn {
    match outcome {
        StepOutcome::PreviewReady(payload) => {
            let page = payload.single_page().expect("a tick is one payload page").to_vec();
            close_payload(payload);
            assert!(page.len() <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES && page.len() <= TOOL_RUN_TRACE_PAGE_BYTES_MAX);
            FillRunTurn::Tick(ToolRunTick::decode(&page).expect("tick decodes"))
        }
        StepOutcome::CheckpointReady(checkpoint) => {
            let bytes = checkpoint.state.single_page().expect("checkpoint page").to_vec();
            close_payload(checkpoint.state);
            FillRunTurn::Checkpoint(bytes)
        }
        StepOutcome::Complete(candidate) => {
            close_payload(candidate.state);
            close_payload(candidate.output);
            FillRunTurn::Complete
        }
        StepOutcome::Yield => FillRunTurn::Yield,
        other => {
            let described = format!("{other:?}");
            assert!(faulted(other), "unexpected run job outcome {described}");
            panic!("the fill run job faulted: {described}");
        }
    }
}

fn fill_run_turn(job: &mut FillRunJob, fuel: u64, sequence: &mut u64) -> FillRunTurn {
    let operation = job.operation();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX), root_cancel_token(), never, sequence);
    settle_fill_run_outcome(job.step(&mut context))
}

/// 🪞️ The ledger side of a run as the contract folds it: provisional ops and entities, the resident
/// trace, and every verdict in the order the ticks carried it.
struct FillRunMirror {
    ops: Vec<Vec<u8>>,
    entities: Vec<u64>,
    trace: ToolRunTraceStore,
    verdicts: Vec<(u64, ToolRunVerdict, u16)>,
    steps: Vec<ToolRunStep>,
    retractions: Vec<u32>,
    checkpoints: Vec<Vec<u8>>,
    last_sequence: Option<u64>,
    progress: Option<ToolRunProgress>,
    ticks: usize,
}

impl FillRunMirror {
    fn new() -> Self {
        Self { ops: Vec::new(), entities: Vec::new(), trace: ToolRunTraceStore::new(fill_run_identity()), verdicts: Vec::new(), steps: Vec::new(), retractions: Vec::new(), checkpoints: Vec::new(), last_sequence: None, progress: None, ticks: 0 }
    }

    fn apply(&mut self, tick: ToolRunTick) {
        assert!(self.last_sequence.is_none_or(|last| tick.sequence > last), "tick sequences are monotone");
        self.last_sequence = Some(tick.sequence);
        self.ticks += 1;
        if let Some(retract_to) = tick.retract_to {
            self.retractions.push(retract_to);
            self.ops.truncate(retract_to as usize);
            self.entities.truncate((retract_to / FILL_RUN_OPS_PER_PLACEMENT) as usize);
        }
        self.ops.extend(tick.append_ops);
        self.entities.extend(tick.append_entities);
        assert_eq!(self.ops.len(), self.entities.len() * FILL_RUN_OPS_PER_PLACEMENT as usize, "every placement carries exactly its two ops and one entity");
        for page in &tick.trace {
            for op in &page.ops {
                if let ToolRunTraceOp::Upsert { key, verdict, reason, .. } = op {
                    if *verdict != ToolRunVerdict::Testing {
                        self.verdicts.push((*key, *verdict, *reason));
                    }
                }
            }
            self.trace.apply_page(page).expect("pages of the one run and generation");
        }
        self.steps.extend(tick.steps);
        if tick.progress.is_some() {
            self.progress = tick.progress;
        }
    }

    fn drive(&mut self, job: &mut FillRunJob, fuel: u64, turns: usize) -> usize {
        let mut sequence = 0;
        for turn in 0..turns {
            match fill_run_turn(job, fuel, &mut sequence) {
                FillRunTurn::Tick(tick) => self.apply(tick),
                FillRunTurn::Checkpoint(bytes) => self.checkpoints.push(bytes),
                FillRunTurn::Complete => return turn + 1,
                FillRunTurn::Yield => {}
            }
        }
        panic!("the fill run job did not complete in {turns} turns at stage {:?}", job.builder().stage);
    }

    fn verdict_words(&self) -> Vec<String> {
        self.verdicts.iter().map(|(_, verdict, reason)| format!("{}:{}", verdict.as_str(), FillRunReason::from_code(*reason).map_or("framework", FillRunReason::id))).collect()
    }
}

fn fill_run_summary(job: &FillRunJob, mirror: &FillRunMirror, prefix: usize) -> serde_json::Value {
    let [tested, locked, collisions, rejected, marked] = job.counters();
    let stall = mirror.steps.iter().rev().find(|step| step.kind == ToolRunStepKind::Warning).and_then(|step| FillRunReason::from_code(step.reason)).map(FillRunReason::id);
    serde_json::json!({
        "verdictPrefix": mirror.verdict_words().into_iter().take(prefix).collect::<Vec<_>>(),
        "tested": tested,
        "locked": locked,
        "collisions": collisions,
        "rejected": rejected,
        "marked": marked,
        "appendOps": mirror.ops.len(),
        "appendEntities": mirror.entities.len(),
        "checkpoints": mirror.checkpoints.len(),
        "stall": stall,
    })
}

/// ⚖️ LAW (language-neutral fixture `🎞️fill-run.json`): a seeded shipped document and a requested count
/// produce exactly the declared verdict prefix, counters, op and entity counts — and the laws that hold
/// for every run: two ops and one entity per placement, one `success` per placement, one `danger` per
/// collision, one `warning` per rule refusal, ops alternating `create_object` / `connect_vortices` whose
/// entity is the created object's own id digest.
#[test]
fn fill_run_job_matches_the_language_neutral_fill_run_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    assert_eq!(fixture["laws"]["opsPerPlacement"].as_u64(), Some(u64::from(FILL_RUN_OPS_PER_PLACEMENT)));
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🔣️.json")).expect("schema");
    let vocabulary = &schema["$defs"]["Puzzle3dFillRun"]["x-semio-toolRun"];
    assert_eq!(vocabulary["stages"].as_array().map(|stages| stages.iter().filter_map(serde_json::Value::as_str).collect::<Vec<_>>()), Some(FillRunStage::ALL.iter().map(|stage| stage.id()).collect()));
    assert_eq!(vocabulary["counters"].as_array().map(|counters| counters.iter().filter_map(serde_json::Value::as_str).collect::<Vec<_>>()), Some(FillRunCounter::ALL.iter().map(|counter| counter.id()).collect()));
    let reasons: Vec<(u64, String, String)> = vocabulary["reasons"].as_array().expect("reasons").iter().map(|reason| (reason["code"].as_u64().expect("code"), reason["id"].as_str().expect("id").into(), reason["verdict"].as_str().expect("verdict").into())).collect();
    assert_eq!(reasons, FillRunReason::ALL.iter().map(|reason| (u64::from(reason.code()), reason.id().to_string(), reason.verdict().as_str().to_string())).collect::<Vec<_>>());
    assert_eq!(vocabulary["checkpoint"]["bytes"].as_u64(), Some(FillRunCheckpoint::BYTES as u64));
    let mut disagreements = Vec::new();
    for case in fixture["cases"].as_array().expect("cases") {
        let document = case["document"].as_str().expect("document");
        let seed = case["seed"].as_u64().expect("seed");
        let requested = case["requested"].as_u64().expect("requested") as usize;
        let (roots, lane, _) = example_fill_roots(document, seed as u32);
        let mut job = fill_run_job(roots, lane, seed, requested);
        let mut mirror = FillRunMirror::new();
        mirror.drive(&mut job, u64::MAX, 1_000_000);
        let expected = &case["expected"];
        let prefix = expected["verdictPrefix"].as_array().map_or(0, Vec::len);
        let actual = fill_run_summary(&job, &mirror, prefix);
        if &actual != expected {
            disagreements.push(format!("{document} seed {seed} requested {requested}: actual {actual}"));
        }
        let [tested, locked, collisions, rejected, marked] = job.counters();
        let count = |wanted: ToolRunVerdict| mirror.verdicts.iter().filter(|(_, verdict, _)| *verdict == wanted).count() as u64;
        assert_eq!((count(ToolRunVerdict::Success), count(ToolRunVerdict::Danger), count(ToolRunVerdict::Warning)), (locked, collisions + marked, rejected));
        assert_eq!(mirror.verdicts.len() as u64, tested + marked, "every constructed candidate reached exactly one verdict and every marked vortex one danger record");
        let candidates = mirror.trace.len() as u64 - marked;
        assert!(mirror.trace.len() as u64 >= marked && candidates <= 1, "only the last tested candidate stays resident next to every marked vortex: {candidates} candidates, {marked} marked");
        for (index, pair) in mirror.ops.chunks(2).enumerate() {
            let Ok(Puzzle3dMutation::CreateObject(create)) = crate::standards::v1::subsets::any::schema::mutations::binary::decode_op(&pair[0]) else { panic!("op {} is create_object", 2 * index) };
            let Ok(Puzzle3dMutation::ConnectVortices(connect)) = crate::standards::v1::subsets::any::schema::mutations::binary::decode_op(&pair[1]) else { panic!("op {} is connect_vortices", 2 * index + 1) };
            assert_eq!(mirror.entities[index], fill_run_entity(&create.object.id));
            assert_eq!(connect.attracted.split(':').next(), Some(create.object.id.as_str()), "the attraction docks the created object");
            assert_eq!(create.object.id, job.builder().appended_objects[index].id);
        }
        let progress = mirror.progress.as_ref().expect("progress");
        assert_eq!((progress.state, progress.completed, progress.total), (ToolRunState::Complete, locked, Some(requested as u64)));
        assert_eq!(progress.counters.iter().map(|counter| counter.value).collect::<Vec<_>>(), vec![tested, locked, collisions, rejected, marked]);
    }
    assert!(disagreements.is_empty(), "the fill run fixture disagrees:\n{}", disagreements.join("\n"));
}

fn parry_hull(pose: &Pose3d, positions: &[f32]) -> parry3d::shape::ConvexPolyhedron {
    let points: Vec<parry3d::math::Point<f32>> = positions
        .chunks(3)
        .map(|vertex| {
            let world = pose.transform_point(&crate::editor::puzzle3d::precompute::geometry::Point3d::new(vertex[0], vertex[1], vertex[2]));
            parry3d::math::Point::new(world.x(), world.y(), world.z())
        })
        .collect();
    parry3d::shape::ConvexPolyhedron::from_convex_hull(&points).expect("box hull")
}

/// 📦️ Overlap volume of two world-space hulls by `parry3d` point containment on a regular grid over
/// their bounding-box intersection, plus that intersection's volume.
fn parry_overlap(a: &parry3d::shape::ConvexPolyhedron, b: &parry3d::shape::ConvexPolyhedron, cells: usize) -> (f64, f64) {
    use parry3d::query::PointQuery;
    use parry3d::shape::Shape;
    let (left, right) = (a.compute_local_aabb(), b.compute_local_aabb());
    let min = left.mins.sup(&right.mins);
    let max = left.maxs.inf(&right.maxs);
    let size = max - min;
    if size.iter().any(|extent| *extent <= 0.0) {
        return (0.0, 0.0);
    }
    let box_volume = f64::from(size.x) * f64::from(size.y) * f64::from(size.z);
    let mut inside = 0usize;
    for x in 0..cells {
        for y in 0..cells {
            for z in 0..cells {
                let at = |index: usize, axis: usize| min[axis] + size[axis] * ((index as f32 + 0.5) / cells as f32);
                let point = parry3d::math::Point::new(at(x, 0), at(y, 1), at(z, 2));
                inside += usize::from(a.contains_local_point(&point) && b.contains_local_point(&point));
            }
        }
    }
    (box_volume * inside as f64 / (cells * cells * cells) as f64, box_volume)
}

/// 🧥️ The mesh identity every second body of an own-mesh document variant carries instead of its kind's.
const FILL_RUN_OWN_MESH_URL: &str = "/own-mesh/body.glb";

/// 🧥️ A shipped example document in which every second body carries its own mesh `FILL_RUN_OWN_MESH_URL`, `scale`
/// times the box its kind renders; answers the roots, the mesh lane and each mesh identity's raw positions.
fn own_mesh_fill_roots(document: &str, seed: u32, scale: f32) -> (FillPreparationRoots, Vec<String>, HashMap<String, Vec<f32>>) {
    let (roots, mut lane, positions) = example_fill_roots(document, seed);
    let mut scene = (*roots.scene).clone();
    for object in scene.fixture.objects.iter_mut().step_by(2) {
        object.mesh_url = Some(FILL_RUN_OWN_MESH_URL.into());
    }
    let own: Vec<f32> = positions.iter().map(|value| value * scale).collect();
    let fallback = semio_framework_plugin::mesh_from_kind(crate::editor::puzzle3d::PUZZLE3D_FALLBACK_MESH_KIND);
    let mut meshes = (*roots.meshes).clone();
    meshes.insert(FILL_RUN_OWN_MESH_URL.into(), collision_body_from_buffers(&own, &fallback.indices).expect("own mesh body"));
    lane.push(FILL_RUN_OWN_MESH_URL.into());
    lane.sort();
    lane.dedup();
    let mut by_url: HashMap<String, Vec<f32>> = lane.iter().map(|url| (url.clone(), positions.clone())).collect();
    by_url.insert(FILL_RUN_OWN_MESH_URL.into(), own);
    (FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes)), lane, by_url)
}

/// ⚖️ What one oracle comparison decided.
#[derive(Default)]
struct FillRunOracleTally {
    decisive: usize,
    ambiguous: usize,
    collisions: usize,
    fits: usize,
    disagreements: Vec<String>,
}

/// ⚖️ ORACLE (`parry3d`): every candidate the run marked `danger` (solid overlap) or `success` (fits) — the first
/// `records` of them — is recomputed against every body placed before it — the document's own bodies plus the run's
/// earlier placements, the docking host excluded — as exact convex hulls whose pairwise overlap volume `parry3d`
/// measures by point containment. A document body's hull is built from the mesh it renders, its own `meshUrl` first,
/// independently of the planner's resolution. The planner collides when one pair overlaps beyond the scene's overlap
/// budget, estimated from `COLLISION_SAMPLES` samples of the pair's bounding-box intersection; a verdict is decisive
/// when the true overlap is at least twice (collision) or at most half (fit) the budget and the sample estimate cannot
/// plausibly land across it, or when parry separates the hulls outright.
fn fill_run_parry3d_tally(roots: FillPreparationRoots, lane: Vec<String>, positions: &HashMap<String, Vec<f32>>, seed: u64, requested: usize, records: usize) -> FillRunOracleTally {
    const COLLISION_SAMPLES: f64 = 512.0;
    const DECISIVE_HITS: f64 = 16.0;
    const GRID_CELLS: usize = 16;
    let scene = roots.scene.clone();
    let budget = scene.overlap_budget;
    let mut job = fill_run_job(roots, lane, seed, requested);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    let placed = &job.builder().placed;
    let base = placed.len() - job.builder().sequence.len();
    assert_eq!(base, scene.fixture.objects.len(), "every document body is a collision body");
    let box_positions = &positions[crate::editor::puzzle3d::PUZZLE3D_FALLBACK_MESH_KIND];
    let rendered = |url: Option<&str>| url.map(str::trim).filter(|url| !url.is_empty()).and_then(|url| positions.get(url)).unwrap_or(box_positions);
    let bodies: Vec<(String, parry3d::shape::ConvexPolyhedron)> = scene
        .fixture
        .objects
        .iter()
        .map(|object| (object.id.clone(), parry_hull(&pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale), rendered(object.mesh_url.as_deref()))))
        .chain(placed[base..].iter().map(|entry| (entry.object_id.clone(), parry_hull(&entry.world, rendered(Some(&entry.mesh_url))))))
        .collect();
    let identity = parry3d::math::Isometry::identity();
    let mut tally = FillRunOracleTally::default();
    for record in job.verdicts.iter().filter(|record| matches!(record.reason, FillRunReason::SolidOverlap | FillRunReason::Fits)).take(records) {
        let candidate = parry_hull(&pose_isometry(record.origin, record.orientation, &None), rendered(Some(&record.mesh_url)));
        let (mut collides, mut uncertain) = (false, false);
        for (_, other) in bodies[..base + record.placements_before].iter().filter(|(id, _)| Some(id) != record.host.as_ref()) {
            if !parry3d::bounding_volume::BoundingVolume::intersects(&parry3d::shape::Shape::compute_local_aabb(&candidate), &parry3d::shape::Shape::compute_local_aabb(other)) || parry3d::query::distance(&identity, &candidate, &identity, other).expect("convex distance") > 0.0 {
                continue;
            }
            let (volume, box_volume) = parry_overlap(&candidate, other, GRID_CELLS);
            let expected_hits = COLLISION_SAMPLES * volume / box_volume.max(f64::MIN_POSITIVE);
            let threshold_hits = COLLISION_SAMPLES * budget / box_volume.max(f64::MIN_POSITIVE);
            if volume >= 2.0 * budget && expected_hits >= DECISIVE_HITS {
                collides = true;
            } else if !(volume <= 0.5 * budget && threshold_hits >= DECISIVE_HITS) {
                uncertain = true;
            }
        }
        let ours = record.reason == FillRunReason::SolidOverlap;
        tally.collisions += usize::from(ours);
        tally.fits += usize::from(!ours);
        if !collides && uncertain {
            tally.ambiguous += 1;
            continue;
        }
        tally.decisive += 1;
        if ours != collides {
            tally.disagreements.push(format!("candidate {} ours={:?} parry collides={collides}", record.key, record.reason));
        }
    }
    tally
}

/// ⚖️ ORACLE (`parry3d`, see [`fill_run_parry3d_tally`]): the shipped Concrete Forest law and the own-mesh variants
/// of Concrete Forest and Nakagin, whose bodies render meshes that differ from their kinds'. Every decisive verdict
/// of every document agrees, each document keeps at most one in ten verdicts inside the sampling band, and the
/// documents together decide both collisions and fits.
#[test]
fn fill_run_job_collision_verdicts_agree_with_the_parry3d_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    let oracle = &fixture["laws"]["parryOracle"];
    let (mut collisions, mut fits) = (0, 0);
    let mut documents = vec![(oracle, false)];
    documents.extend(oracle["ownMeshVariants"].as_array().expect("own mesh variants").iter().map(|variant| (variant, true)));
    for (law, own_mesh) in documents {
        let (document, seed, requested) = (law["document"].as_str().expect("document"), law["seed"].as_u64().expect("seed"), law["requested"].as_u64().expect("requested") as usize);
        let (roots, lane, positions) = if own_mesh {
            own_mesh_fill_roots(document, seed as u32, law["scale"].as_f64().expect("own mesh scale") as f32)
        } else {
            let (roots, lane, positions) = example_fill_roots(document, seed as u32);
            let by_url = lane.iter().map(|url| (url.clone(), positions.clone())).collect();
            (roots, lane, by_url)
        };
        let tally = fill_run_parry3d_tally(roots, lane, &positions, seed, requested, law["records"].as_u64().map_or(usize::MAX, |records| records as usize));
        let name = format!("{document} seed {seed}{}", if own_mesh { " own-mesh" } else { "" });
        assert!(tally.disagreements.is_empty(), "{name}: {} of {} decisive verdicts disagree with parry3d:\n{}", tally.disagreements.len(), tally.decisive, tally.disagreements.join("\n"));
        assert!(tally.decisive > 0 && tally.ambiguous * 10 <= tally.decisive, "{name}: at most one in ten verdicts may fall inside the sampling band: {} of {}", tally.ambiguous, tally.decisive);
        collisions += tally.collisions;
        fits += tally.fits;
    }
    assert!(collisions > 0 && fits > 0, "the oracle documents must decide both collisions ({collisions}) and fits ({fits})");
}

/// ⚖️ LAW: a long run (at least the fixture's `candidates` tested) delivers every trace record — the ledger's resident
/// store and a renderer that only ever reads byte-budgeted deltas through its echoed cursor hold exactly the key set the
/// job reported (every marked vortex and the ONE candidate still on screen — each newly tested candidate retires the
/// previous one), with the verdict the job reported last.
#[test]
fn fill_run_job_delivers_every_trace_record_of_a_long_run() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    let law = &fixture["laws"]["delivery"];
    let minimum = law["candidates"].as_u64().expect("candidates");
    let seed = law["seed"].as_u64().expect("seed");
    let (roots, lane, _) = example_fill_roots(law["document"].as_str().expect("document"), seed as u32);
    let mut job = fill_run_job(roots, lane, seed, law["requested"].as_u64().expect("requested") as usize);
    let mut ledger = FillRunMirror::new();
    let mut renderer = ToolRunTraceStore::new(fill_run_identity());
    let mut cursor: Option<ToolRunTraceCursor> = None;
    let mut expected: HashMap<u64, ToolRunVerdict> = HashMap::new();
    let mut sequence = 0;
    let budget = law["deltaBudgetBytes"].as_u64().expect("delta budget") as usize;
    for _ in 0..10_000_000 {
        match fill_run_turn(&mut job, 64, &mut sequence) {
            FillRunTurn::Tick(tick) => {
                for op in tick.trace.iter().flat_map(|page| &page.ops) {
                    match op {
                        ToolRunTraceOp::Upsert { key, verdict, .. } => {
                            expected.insert(*key, *verdict);
                        }
                        ToolRunTraceOp::Retire { key } => {
                            expected.remove(key);
                        }
                        ToolRunTraceOp::Clear => expected.clear(),
                    }
                }
                ledger.apply(tick);
                let delta = ledger.trace.delta_after(cursor, budget);
                if delta.clear {
                    renderer = ToolRunTraceStore::new(fill_run_identity());
                }
                for page in &delta.pages {
                    renderer.apply_ops(&page.ops);
                }
                cursor = Some(ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next });
            }
            FillRunTurn::Checkpoint(_) | FillRunTurn::Yield => {}
            FillRunTurn::Complete => break,
        }
        if job.counters()[0] >= minimum {
            break;
        }
    }
    loop {
        let delta = ledger.trace.delta_after(cursor, budget);
        if delta.clear {
            renderer = ToolRunTraceStore::new(fill_run_identity());
        }
        let caught_up = delta.pages.is_empty();
        for page in &delta.pages {
            renderer.apply_ops(&page.ops);
        }
        cursor = Some(ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next });
        if caught_up {
            break;
        }
    }
    assert!(job.counters()[0] >= minimum, "the delivery law needs at least {minimum} tested candidates, the run reached {:?}", job.counters());
    let keys = |store: &ToolRunTraceStore| store.records().map(|(key, record)| (key, record.verdict)).collect::<HashMap<_, _>>();
    assert!(expected.len() as u64 >= job.counters()[4] && expected.len() as u64 <= job.counters()[4] + 1, "marked vortices plus at most the current candidate stay resident: {} records, counters {:?}", expected.len(), job.counters());
    assert_eq!(keys(&ledger.trace), expected, "the ledger holds every reported record");
    assert_eq!(keys(&renderer), expected, "a cursor-driven renderer holds every reported record");
}

/// ⏱️ Turn clock of the interactive law: records where the wall slice expired in the first cold run
/// and replays exactly those expiries in the later runs, so every run takes the same bounded turns.
#[derive(Default)]
struct FillRunTurnClock {
    reads: u64,
    deadline: u64,
    first_expired: Option<u64>,
    replay_expiry: Option<u64>,
}

thread_local! {
    static FILL_RUN_TURN_CLOCK: std::cell::RefCell<FillRunTurnClock> = std::cell::RefCell::new(FillRunTurnClock::default());
}

fn fill_run_recording_clock() -> Option<u64> {
    let now = semio_framework_job::default_now_us();
    FILL_RUN_TURN_CLOCK.with(|clock| {
        let mut clock = clock.borrow_mut();
        clock.reads += 1;
        if clock.first_expired.is_none() && now.is_none_or(|now| now >= clock.deadline) {
            clock.first_expired = Some(clock.reads);
        }
    });
    now
}

fn fill_run_replaying_clock() -> Option<u64> {
    FILL_RUN_TURN_CLOCK.with(|clock| {
        let mut clock = clock.borrow_mut();
        clock.reads += 1;
        Some(if clock.replay_expiry.is_some_and(|expiry| clock.reads >= expiry) { u64::MAX } else { 0 })
    })
}

/// 🪞️ The tool run ledger's overlay append of one op (`📋️tool-run-contract.md` §2.7.2): decode, diff against the
/// running overlay, apply — `None` for an op the overlay no longer admits.
fn fold_overlay_op(overlay: &Puzzle3dPlaySnapshot, bytes: &[u8]) -> Option<Puzzle3dPlaySnapshot> {
    use protocol::MutationDiff;
    let op = <Puzzle3dMutation as protocol::OpBinary>::decode_op(bytes).ok()?;
    let outcome = protocol::Mutation::<Puzzle3dPlaySnapshot>::diff(&op, overlay);
    if !outcome.is_applicable(protocol::MergePolicy::default()) {
        return None;
    }
    outcome.diff().apply(overlay).ok()
}

/// ⏱️ LAW (red→green row 1, the successor of the deleted `fillBuildTick` law): on the shipped Nakagin document,
/// over at least 771 turns, (a) every `drive_step` of the fill run job under the interactive lane's wall slice
/// and (b) every overlay append of one tick's `appendOps` — the O(k) fold the tool run ledger performs — stay
/// below the artifact's unchanged 2 ms budget. Like the artifact's other interactive laws it takes each turn's
/// best of several cold runs; the first run slices by the real clock and the others replay its slice
/// boundaries exactly, so every run yields the same ticks in the same order.
#[test]
fn fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    let law = &fixture["laws"]["interactive"];
    let minimum_turns = law["turns"].as_u64().expect("turns") as usize;
    let budget = Duration::from_micros(law["budgetUs"].as_u64().expect("budget"));
    let runs = law["coldRuns"].as_u64().expect("cold runs") as usize;
    let seed = law["seed"].as_u64().expect("seed");
    let mut expiries: Vec<Option<u64>> = Vec::new();
    let mut best: Vec<Duration> = Vec::new();
    let mut best_appends: Vec<Duration> = Vec::new();
    let mut counters = [0; 5];
    for run in 0..runs {
        let (roots, lane, _) = example_fill_roots(law["document"].as_str().expect("document"), seed as u32);
        let mut overlay = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT).expect("example parses"))).into());
        let mut append = 0;
        let mut job = fill_run_job(roots, lane, seed, law["requested"].as_u64().expect("requested") as usize);
        let operation = job.operation();
        let mut sequence = 0;
        let mut verdict = None;
        for turn in 0.. {
            let recording = run == 0;
            assert!(recording || turn < expiries.len(), "cold run {run} took more turns than the recorded run");
            let (step_budget, clock): (StepBudget, fn() -> Option<u64>) = if recording {
                let start = semio_framework_job::default_now_us().expect("clock");
                FILL_RUN_TURN_CLOCK.with(|clock| *clock.borrow_mut() = FillRunTurnClock { deadline: start + semio_framework_job::INTERACTIVE_LANE_WALL_US, ..FillRunTurnClock::default() });
                (StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, start + semio_framework_job::INTERACTIVE_LANE_WALL_US), fill_run_recording_clock)
            } else {
                FILL_RUN_TURN_CLOCK.with(|clock| *clock.borrow_mut() = FillRunTurnClock { replay_expiry: expiries[turn], ..FillRunTurnClock::default() });
                (StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, 1), fill_run_replaying_clock)
            };
            let started = Instant::now();
            let outcome = semio_framework_job::drive_step(&mut job, "puzzle3d-fill-run", operation.operation, operation.generation, semio_framework_job::InteractiveStage::InteractiveStep, step_budget, root_cancel_token(), clock, &mut sequence, &mut verdict);
            let elapsed = started.elapsed();
            if recording {
                expiries.push(FILL_RUN_TURN_CLOCK.with(|clock| clock.borrow().first_expired));
                best.push(elapsed);
            } else {
                best[turn] = best[turn].min(elapsed);
            }
            match settle_fill_run_outcome(outcome) {
                FillRunTurn::Complete => {
                    assert!(recording || turn + 1 == expiries.len(), "cold run {run} completed after {} turns, the recorded run after {}", turn + 1, expiries.len());
                    break;
                }
                FillRunTurn::Tick(tick) if !tick.append_ops.is_empty() => {
                    let started = Instant::now();
                    for bytes in &tick.append_ops {
                        overlay = fold_overlay_op(&overlay, bytes).expect("every appended op applies onto the overlay it was planned against");
                    }
                    let elapsed = started.elapsed();
                    if recording {
                        best_appends.push(elapsed);
                    } else {
                        best_appends[append] = best_appends[append].min(elapsed);
                    }
                    append += 1;
                }
                _ => {}
            }
        }
        counters = job.counters();
    }
    let worst_append = best_appends.iter().max().copied().unwrap_or(Duration::ZERO);
    assert!(!best_appends.is_empty(), "the measured run appended at least one tick of provisional ops");
    assert!(worst_append < budget, "the worst overlay append of one tick's appendOps took {worst_append:?}, over {budget:?}");
    let (turn, worst) = best.iter().enumerate().max_by_key(|(_, elapsed)| **elapsed).map_or((0, Duration::ZERO), |(turn, elapsed)| (turn + 1, *elapsed));
    assert!(best.len() >= minimum_turns, "the law measures at least {minimum_turns} turns, the run took {}", best.len());
    assert!(counters[1] > 0 && counters[0] > counters[1], "the measured run tested and placed objects: {counters:?}");
    assert!(worst < budget, "fill run job worst drive_step {worst:?} at turn {turn} of {} exceeds {budget:?}", best.len());
}

/// ⚖️ LAW: with one unit of fuel a run job step publishes exactly one visible unit — the `testing` upsert of the
/// candidate it just constructed, or that candidate's final verdict — so every tested candidate is on screen as
/// `testing` in a tick of its own before the tick that marks it (collision, fit or refusal).
#[test]
fn fill_run_job_step_with_one_unit_of_fuel_shows_each_candidate_before_its_verdict() {
    let mut job = FillRunJob::new(FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(73), RevisionId(1), Generation(1), 43), 24), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()], [0; 32]);
    let mut sequence = 0;
    let (mut shown, mut verdicts) = (0u64, 0u64);
    let mut under_test: Option<u64> = None;
    for _ in 0..1_000_000 {
        match fill_run_turn(&mut job, 1, &mut sequence) {
            FillRunTurn::Tick(tick) => {
                let ops: Vec<&ToolRunTraceOp> = tick.trace.iter().flat_map(|page| &page.ops).collect();
                let finals: Vec<u64> = ops.iter().filter_map(|op| match op {
                    ToolRunTraceOp::Upsert { key, verdict, .. } if *verdict != ToolRunVerdict::Testing => Some(*key),
                    _ => None,
                }).collect();
                let testing: Vec<u64> = ops.iter().filter_map(|op| match op {
                    ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Testing, .. } => Some(*key),
                    _ => None,
                }).collect();
                assert!(finals.len() + testing.len() <= 1, "one fuel unit is one visible unit, tick {} carried testing {testing:?} and verdicts {finals:?}", tick.sequence);
                if let Some(&key) = testing.first() {
                    under_test = Some(key);
                    shown += 1;
                }
                if let Some(&key) = finals.first() {
                    assert_eq!(under_test.take(), Some(key), "tick {} marks a candidate an earlier tick showed under test", tick.sequence);
                    verdicts += 1;
                }
            }
            FillRunTurn::Checkpoint(_) | FillRunTurn::Yield => {}
            FillRunTurn::Complete => break,
        }
    }
    assert_eq!((shown, verdicts), (job.counters()[0], job.counters()[0]), "every tested candidate was shown, then marked");
    assert_eq!(job.counters()[1], 24);
}

/// ⚖️ LAW: the run shows ONE candidate at a time. After every tick the resident trace holds the marked vortices and at
/// most one candidate record — the newest tested one, `testing` while under test, then its collision (`danger`) or fit
/// (`success`) — because each newly constructed candidate retires the previous one. A placement stays visible as its
/// provisional document instance, never as a pile of past collision records.
#[test]
fn fill_run_job_keeps_only_the_current_candidate_on_screen() {
    // 🧫️ The fixture's concrete-forest seed 3 run: 40 placements, collisions and marked vortices all occur.
    let (roots, lane, _) = example_fill_roots("concrete-forest", 3);
    let mut job = fill_run_job(roots, lane, 3, 40);
    let mut mirror = FillRunMirror::new();
    let mut sequence = 0;
    let mut newest: Option<u64> = None;
    let (mut ticks, mut danger_shown) = (0usize, false);
    for _ in 0..1_000_000 {
        match fill_run_turn(&mut job, 8, &mut sequence) {
            FillRunTurn::Tick(tick) => {
                for op in tick.trace.iter().flat_map(|page| &page.ops) {
                    if let ToolRunTraceOp::Upsert { key, reason, .. } = op {
                        if *reason != FillRunReason::VortexExhausted.code() {
                            newest = Some(newest.map_or(*key, |current| current.max(*key)));
                        }
                    }
                }
                mirror.apply(tick);
                ticks += 1;
                let candidates: Vec<(u64, ToolRunVerdict)> = mirror.trace.records().filter(|(_, record)| record.reason != FillRunReason::VortexExhausted.code()).map(|(key, record)| (key, record.verdict)).collect();
                assert!(candidates.len() <= 1, "tick {ticks}: only the current candidate may be on screen, resident {candidates:?}");
                if let Some((key, verdict)) = candidates.first() {
                    assert_eq!(Some(*key), newest, "tick {ticks}: the resident candidate is the newest tested one");
                    danger_shown |= *verdict == ToolRunVerdict::Danger;
                }
            }
            FillRunTurn::Checkpoint(bytes) => mirror.checkpoints.push(bytes),
            FillRunTurn::Yield => {}
            FillRunTurn::Complete => break,
        }
    }
    let [tested, locked, collisions, _, marked] = job.counters();
    assert_eq!(locked, 40);
    assert!(marked > 0, "the law needs a marked vortex next to the current candidate");
    assert!(collisions > 0 && danger_shown, "the law needs a colliding candidate on screen ({collisions} collisions)");
    assert!(tested > locked + 1, "the law needs more tested candidates than the one on screen ({tested} tested)");
    assert_eq!(mirror.trace.records().filter(|(_, record)| record.reason == FillRunReason::VortexExhausted.code()).count() as u64, marked, "every marked vortex stays marked");
}

fn nakagin_scale_run(requested: usize) -> (FillRunJob, FillRunMirror) {
    let mut job = FillRunJob::new(FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(79), RevisionId(1), Generation(1), 43), requested), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()], [0; 32]);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    (job, mirror)
}

/// ⚖️ LAW: a completed run resumed from its own checkpoint with a raised count continues the same
/// deterministic sequence — ops, entities and verdicts equal one run to the raised count — and resumed
/// with a lowered count retracts the provisional tail to exactly the lowered count; a foreign or
/// malformed checkpoint is refused.
#[test]
fn fill_run_job_resume_raise_continues_the_sequence_and_lower_retracts_the_tail() {
    const SHORT: usize = 30;
    const LONG: usize = 45;
    const LOWERED: usize = 12;
    let (mut raised, mut raised_mirror) = nakagin_scale_run(SHORT);
    assert_eq!(raised_mirror.checkpoints.len(), SHORT, "one checkpoint per placement");
    let checkpoint = raised_mirror.checkpoints.last().cloned().expect("checkpoint");
    assert_eq!(FillRunCheckpoint::decode(&checkpoint), Some(raised.checkpoint()), "the last checkpoint is where the completed run stands");
    let short_ops = raised_mirror.ops.clone();
    raised.resume(&checkpoint, LONG).expect("resume raise");
    raised_mirror.drive(&mut raised, u64::MAX, 1_000_000);
    let (long, long_mirror) = nakagin_scale_run(LONG);
    assert_eq!(raised_mirror.ops, long_mirror.ops, "the raised run appends the same ops as one long run");
    assert_eq!(raised_mirror.entities, long_mirror.entities);
    assert_eq!(raised_mirror.verdicts, long_mirror.verdicts, "and reaches the same verdicts in the same order");
    assert_eq!(&long_mirror.ops[..short_ops.len()], short_ops.as_slice());
    assert_eq!(raised.counters(), long.counters());

    let (mut lowered, mut lowered_mirror) = nakagin_scale_run(SHORT);
    let checkpoint = lowered_mirror.checkpoints.last().cloned().expect("checkpoint");
    let before = lowered_mirror.ops.clone();
    lowered.resume(&checkpoint, LOWERED).expect("resume lower");
    lowered_mirror.drive(&mut lowered, u64::MAX, 1_000_000);
    assert_eq!(lowered_mirror.retractions.iter().min().copied(), Some(LOWERED as u32 * FILL_RUN_OPS_PER_PLACEMENT), "the lowered run retracts to exactly the lowered count");
    assert_eq!(lowered_mirror.ops.as_slice(), &before[..LOWERED * 2]);
    assert_eq!((lowered_mirror.entities.len(), lowered.counters()[1], lowered.builder().sequence.len()), (LOWERED, LOWERED as u64, LOWERED));
    assert!(
        lowered_mirror.trace.records().filter(|(_, record)| record.verdict == ToolRunVerdict::Success).all(|(key, _)| lowered.placement_keys.iter().any(|(kept, _)| *kept == key)),
        "no retracted placement's success record stays resident"
    );

    let mut foreign = FillRunCheckpoint::decode(&checkpoint).expect("checkpoint");
    foreign.next_key += 1_000;
    assert_eq!(lowered.resume(&foreign.encode(), SHORT), Err(FillRunResumeError::Foreign));
    assert_eq!(lowered.resume(&checkpoint[..8], SHORT), Err(FillRunResumeError::Malformed));
}

fn nakagin_scale_builder(operation: u64, requested: usize) -> FillBuilder {
    FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(operation), RevisionId(1), Generation(1), 43), requested)
}

/// ⏪️ LAW: a run job rebuilt from its predecessor's last checkpoint — the framework closes a run's job on
/// every settings change and builds a new one, whose tick sequence starts over under the newer generation —
/// replays silently to that checkpoint, with no tick leaving it
/// on the way, and then continues exactly like the resident job: a raise appends the long run's ops and
/// verdicts, a lower retracts to the lowered count. A job whose inputs differ restarts instead: its first tick
/// clears the trace and retracts every provisional op, and it then runs like a fresh run.
#[test]
fn fill_run_job_rebuilt_from_a_checkpoint_replays_silently_and_continues_like_the_resident_job() {
    const SHORT: usize = 30;
    const LONG: usize = 45;
    const LOWERED: usize = 12;
    let lane = || vec![NAKAGIN_MESH_URL.to_string()];
    let (_, mut raised) = nakagin_scale_run(SHORT);
    let checkpoint = FillRunCheckpoint::decode(raised.checkpoints.last().expect("checkpoint")).expect("checkpoint decodes");
    let provisional = raised.ops.len() as u32;
    let mut rebuilt = FillRunJob::replaying(nakagin_scale_builder(81, checkpoint.requested as usize), fill_run_identity(), lane(), checkpoint, LONG, provisional);
    raised.last_sequence = None;
    raised.drive(&mut rebuilt, u64::MAX, 1_000_000);
    let (long, long_mirror) = nakagin_scale_run(LONG);
    assert!(raised.retractions.is_empty(), "a raise retracts nothing");
    assert_eq!(raised.ops, long_mirror.ops, "the rebuilt run appends exactly the long run's ops after its predecessor's");
    assert_eq!(raised.entities, long_mirror.entities);
    assert_eq!(raised.verdicts, long_mirror.verdicts, "the replay published no verdict twice and missed none");
    assert_eq!(rebuilt.counters(), long.counters());

    let (_, mut lowered) = nakagin_scale_run(SHORT);
    let before = lowered.ops.clone();
    let mut rebuilt = FillRunJob::replaying(nakagin_scale_builder(82, checkpoint.requested as usize), fill_run_identity(), lane(), checkpoint, LOWERED, provisional);
    lowered.last_sequence = None;
    lowered.drive(&mut rebuilt, u64::MAX, 1_000_000);
    assert_eq!(lowered.retractions.iter().min().copied(), Some(LOWERED as u32 * FILL_RUN_OPS_PER_PLACEMENT), "the rebuilt lower retracts to exactly the lowered count");
    assert_eq!(lowered.ops.as_slice(), &before[..LOWERED * FILL_RUN_OPS_PER_PLACEMENT as usize]);
    assert!(
        lowered.trace.records().filter(|(_, record)| record.verdict == ToolRunVerdict::Success).all(|(key, _)| rebuilt.placement_keys.iter().any(|(kept, _)| *kept == key)),
        "no retracted placement's success record stays resident"
    );

    let (_, mut restarted) = nakagin_scale_run(SHORT);
    let mut foreign = checkpoint;
    foreign.inputs = [9; 32];
    let mut rebuilt = FillRunJob::restarting(nakagin_scale_builder(83, LOWERED), fill_run_identity(), lane(), foreign.inputs, provisional);
    restarted.last_sequence = None;
    restarted.drive(&mut rebuilt, u64::MAX, 1_000_000);
    let (_, fresh) = nakagin_scale_run(LOWERED);
    assert_eq!(restarted.retractions.first().copied(), Some(0), "a restart retracts every provisional op first");
    assert_eq!((restarted.ops.clone(), restarted.entities.clone()), (fresh.ops.clone(), fresh.entities.clone()), "and then runs like a fresh run");
    assert_eq!(restarted.trace.len(), fresh.trace.len(), "its first tick cleared the predecessor's trace");
}

/// 🚧️ LAW (P4e): a document too large for the planner's fixed pages publishes ONE `danger` step naming the
/// refused capacity through the run's tick, and the next step faults the run — nothing was placed.
#[test]
fn fill_run_job_capacity_refusal_publishes_a_danger_step_before_faulting() {
    let objects = (0..=DOCUMENT_OBJECT_SLOTS).map(|index| FixtureObject { id: format!("rejected-{index:04}"), object_kind: None, anchor: Default::default(), mesh_url: None, origin: [0.0; 3], orientation: None, scale: None, vortices: Vec::new() }).collect();
    let scene = Arc::new(SceneConfig { fixture: Fixture { objects, attractions: Vec::new(), target_volumes: Vec::new() }, kind_catalogs: Some(KindCatalogBundle::default()), kind_compatibility: Vec::new(), overlap_budget: 0.0, seed: 37, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() });
    let builder = FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(OperationId(37), RevisionId(9), Generation(11), 37), TEST_REQUESTED_COUNT);
    let mut job = FillRunJob::new(builder, fill_run_identity(), Vec::new(), [0; 32]);
    let mut sequence = 0;
    let FillRunTurn::Tick(tick) = fill_run_turn(&mut job, u64::MAX, &mut sequence) else { panic!("the refusal is published as a tick first") };
    assert!(tick.append_ops.is_empty(), "nothing is placed");
    assert_eq!(tick.steps.iter().map(|step| (step.kind, step.reason, step.args.clone())).collect::<Vec<_>>(), vec![(ToolRunStepKind::Danger, FillRunReason::ArtifactCapacity.code(), vec![ToolRunStepArg::Unsigned(DOCUMENT_OBJECT_SLOTS as u64)])]);
    let operation = job.operation();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), root_cancel_token(), never, &mut sequence);
    let StepOutcome::Fault(fault) = job.step(&mut context) else { panic!("the step after the danger step faults the run") };
    assert_eq!(fault.detail.single_page(), Some(format!("preparation-capacity:fixture-objects:{DOCUMENT_OBJECT_SLOTS}").as_bytes()));
    assert!(faulted(StepOutcome::Fault(fault)));
}

/// 🧱️ LAW: the placements a finalize revalidation rebuilds from the ledger's provisional ops alone are the
/// run's own placements, in op order — same object, attraction, entity and mesh-lane subject — keyed
/// consecutively from the allocated first key; a list that is not whole placements is refused.
#[test]
fn fill_run_placements_rebuild_the_provisional_placements_from_their_ops() {
    let (roots, lane, _) = example_fill_roots("concrete-forest", 7);
    let mut job = fill_run_job(roots, lane.clone(), 7, 5);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    let ops: Vec<Puzzle3dMutation> = mirror.ops.iter().map(|bytes| <Puzzle3dMutation as protocol::OpBinary>::decode_op(bytes).expect("op decodes")).collect();
    let rebuilt = fill_run_placements(&ops, job.mesh_lane(), REVALIDATE_FIRST_KEY).expect("whole placements");
    let original = job.provisional_placements();
    assert_eq!(rebuilt.len(), 5);
    for (index, (rebuilt, original)) in rebuilt.iter().zip(&original).enumerate() {
        assert_eq!((&rebuilt.object.id, &rebuilt.attraction, rebuilt.entity), (&original.object.id, &original.attraction, original.entity));
        assert_eq!(rebuilt.key, REVALIDATE_FIRST_KEY + index as u64, "placements take consecutive keys from the allocated range");
        let (ToolRunTraceSubject::Instance3d { mesh: rebuilt_mesh, .. }, ToolRunTraceSubject::Instance3d { mesh: original_mesh, .. }) = (rebuilt.subject, original.subject) else { panic!("instance subjects") };
        assert_eq!(rebuilt_mesh, original_mesh, "the rebuilt subject indexes the same mesh lane entry");
    }
    assert!(fill_run_placements(&ops[..ops.len() - 1], job.mesh_lane(), REVALIDATE_FIRST_KEY).is_none(), "half a placement is refused");
}

/// 🔑️ The first key of the range a revalidation's trace key allocator hands the placement law.
const REVALIDATE_FIRST_KEY: u64 = 4_096;

fn drive_revalidation(job: &mut FillRevalidateJob, operation: Operation) -> Vec<ToolRunTick> {
    let mut ticks = Vec::new();
    let mut sequence = 0;
    for _ in 0..1_000_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), root_cancel_token(), never, &mut sequence);
        match settle_fill_run_outcome(job.step(&mut context)) {
            FillRunTurn::Tick(tick) => ticks.push(tick),
            FillRunTurn::Complete => return ticks,
            FillRunTurn::Checkpoint(_) | FillRunTurn::Yield => {}
        }
    }
    panic!("revalidation did not complete");
}

/// ⚖️ LAW: revalidating provisional placements against an unchanged head keeps every one; against a head
/// that gained a body where one placement stands, that placement turns `danger` with the framework
/// conflict reason, the final tick retracts to it and re-appends every later survivor's exact ops and
/// entity, and one `danger` conflict step counts the conflicts. Its trace shows one placement at a time and none once done.
#[test]
fn fill_revalidate_job_retracts_conflicting_placements_and_reappends_survivors() {
    const PLACED: usize = 8;
    const INTRUDED: usize = 3;
    let (job, mirror) = nakagin_scale_run(PLACED);
    let placements = job.provisional_placements();
    assert_eq!(placements.len(), PLACED);
    let operation = Operation::new(OperationId(83), RevisionId(2), Generation(1), 43);
    let head = nakagin_scale_roots();
    let mut clean = FillRevalidateJob::new(operation, fill_run_identity(), FillPreparationRoots::new(head.scene.clone(), head.meshes.clone()), placements.clone(), 1_000);
    let ticks = drive_revalidation(&mut clean, operation);
    assert!(clean.conflicts().iter().all(|conflict| !conflict) && clean.conflicts().len() == PLACED);
    assert!(ticks.iter().all(|tick| tick.retract_to.is_none() && tick.append_ops.is_empty()));
    // 👁️ One placement on screen at a time, none after the finish: a finalize never leaves a verdict mesh over every
    // committed object.
    let mut screen = ToolRunTraceStore::new(fill_run_identity());
    for tick in &ticks {
        for page in &tick.trace {
            screen.apply_ops(&page.ops);
            assert!(screen.len() <= 1, "revalidation shows one placement at a time, resident {:?}", screen.records().map(|(key, record)| (key, record.verdict)).collect::<Vec<_>>());
        }
    }
    assert_eq!(screen.len(), 0, "the finished revalidation leaves no verdict record on screen");

    let mut scene = (*head.scene).clone();
    let mut intruder = placements[INTRUDED].object.clone();
    intruder.id = "intruder".into();
    intruder.vortices.clear();
    scene.fixture.objects.push(intruder);
    let mut intruded = FillRevalidateJob::new(operation, fill_run_identity(), FillPreparationRoots::new(Arc::new(scene), head.meshes.clone()), placements.clone(), 1_000);
    let ticks = drive_revalidation(&mut intruded, operation);
    let conflicts = intruded.conflicts().to_vec();
    assert!(conflicts[INTRUDED], "the intruded placement conflicts: {conflicts:?}");
    let first = conflicts.iter().position(|conflict| *conflict).expect("a conflict");
    let last = ticks.last().expect("final tick");
    assert_eq!(last.retract_to, Some(first as u32 * FILL_RUN_OPS_PER_PLACEMENT));
    let survivors: Vec<usize> = (first..PLACED).filter(|index| !conflicts[*index]).collect();
    assert_eq!(last.append_ops, survivors.iter().flat_map(|index| mirror.ops[index * 2..index * 2 + 2].to_vec()).collect::<Vec<_>>(), "survivors re-append their exact ops");
    assert_eq!(last.append_entities, survivors.iter().map(|index| mirror.entities[*index]).collect::<Vec<_>>());
    let conflict_count = conflicts.iter().filter(|conflict| **conflict).count() as u64;
    assert!(last.steps.iter().any(|step| step.kind == ToolRunStepKind::Danger && step.reason == TOOL_RUN_REASON_CONFLICT && step.args == vec![ToolRunStepArg::Unsigned(conflict_count)]));
    let danger: Vec<u64> = ticks.iter().flat_map(|tick| tick.trace.iter().flat_map(|page| page.ops.clone())).filter_map(|op| match op {
        ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Danger, reason: TOOL_RUN_REASON_CONFLICT, .. } => Some(key),
        _ => None,
    }).collect();
    assert_eq!(danger, conflicts.iter().zip(&placements).filter(|(conflict, _)| **conflict).map(|(_, placement)| placement.key).collect::<Vec<_>>());
}

/// 🧊️ A cube of half extent 4 shifted `offset` along x, as raw positions and indices.
fn own_mesh_cube(offset: f32) -> (Vec<f32>, Vec<u32>) {
    (
        [-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0].iter().enumerate().map(|(index, unit)| unit * 4.0 + if index % 3 == 0 { offset } else { 0.0 }).collect(),
        vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2],
    )
}

fn own_mesh_kind(id: &str, url: &str, vortices: Vec<ObjectKindVortexTemplate>) -> ObjectKind {
    ObjectKind { id: id.into(), representations: vec![ObjectKindRepresentation { id: id.to_lowercase(), name: String::new(), url: url.into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices }
}

/// 🧥️ The `ownMesh` law's scene (`🎞️fill-run.json`): one host vortex, two compatible kinds, and with `blocker` a
/// body of the host's kind carrying its own mesh where both candidates dock, while the kind's mesh is the same cube
/// shifted `kind_offset` away.
fn own_mesh_roots(blocker: bool, kind_offset: f32) -> FillPreparationRoots {
    let host = FixtureObject {
        id: "host".into(),
        object_kind: Some("Host".into()),
        anchor: Default::default(),
        mesh_url: None,
        origin: [12.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        vortices: vec![VortexProps { id: "v0".into(), vortex_kind: Some("port-a".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
    };
    let mut objects = vec![host.clone()];
    if blocker {
        objects.push(FixtureObject { id: "blocker".into(), mesh_url: Some("/test/blocker.glb".into()), vortices: Vec::new(), ..host });
    }
    let port = || vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".into()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() }];
    let scene = SceneConfig {
        fixture: Fixture { attractions: Vec::new(), target_volumes: Vec::new(), objects },
        kind_catalogs: Some(KindCatalogBundle {
            objects: vec![own_mesh_kind("Host", "/test/host-kind.glb", Vec::new()), own_mesh_kind("Near", "/test/near.glb", port()), own_mesh_kind("Late", "/test/late.glb", port())],
            vortices: vec![VortexKindCatalog { id: "port-a".into(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".into(), default_cable_kind: None, ..Default::default() }],
            cables: vec![CableKindCatalog { id: "cable.link".into(), default_attraction_kind: None, ..Default::default() }],
        }),
        kind_compatibility: vec![KindCompatEntry { source: "port-b".into(), target: "port-a".into(), bidirectional: true, important: false, specificity: Some("vortex".into()) }],
        overlap_budget: 0.02,
        seed: 1,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    };
    let body = |offset: f32| {
        let (positions, indices) = own_mesh_cube(offset);
        collision_body_from_buffers(&positions, &indices).expect("cube body")
    };
    let meshes = [("/test/host-kind.glb", body(kind_offset)), ("/test/blocker.glb", body(0.0)), ("/test/near.glb", body(0.0)), ("/test/late.glb", body(0.0))].into_iter().map(|(url, body)| (url.to_string(), body)).collect();
    FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes))
}

fn own_mesh_lane() -> Vec<String> {
    ["/test/blocker.glb", "/test/host-kind.glb", "/test/late.glb", "/test/near.glb"].map(str::to_string).to_vec()
}

/// 🐛️ Red→green (`📓️wave-W2-C.md` §7.2, language-neutral law `ownMesh` of `🎞️fill-run.json`): a placed body carrying
/// its own mesh collides with that mesh, not its kind's. The fill planner resolved document bodies by kind, so a body
/// whose kind renders a different (here shifted) mesh never blocked the candidates docking into it and they read `fits`;
/// the revalidation job re-tested provisional placements against the same wrong body. Both now resolve a placed
/// object's own mesh first, the renderer's law.
#[test]
fn fill_run_and_revalidation_collide_with_a_placed_body_carrying_its_own_mesh() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    let law = &fixture["laws"]["ownMesh"];
    let kind_offset = law["kindMeshOffset"].as_f64().expect("kind mesh offset") as f32;
    let run = |blocker: bool, requested: usize| {
        let mut job = FillRunJob::new(FillBuilder::begin_preparation(own_mesh_roots(blocker, kind_offset), Operation::new(OperationId(97), RevisionId(1), Generation(1), 1), requested), fill_run_identity(), own_mesh_lane(), [0; 32]);
        let mut mirror = FillRunMirror::new();
        mirror.drive(&mut job, u64::MAX, 1_000_000);
        (job, mirror)
    };
    let (clear, _) = run(false, 1);
    let placements = clear.provisional_placements();
    for case in law["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let blocker = case["blocker"].as_bool().expect("blocker");
        let expected = &case["expected"];
        let (job, mirror) = run(blocker, case["requested"].as_u64().expect("requested") as usize);
        let [_, locked, collisions, _, _] = job.counters();
        let stall = mirror.steps.iter().rev().find(|step| step.kind == ToolRunStepKind::Warning).and_then(|step| FillRunReason::from_code(step.reason)).map(FillRunReason::id);
        let mut actual = serde_json::json!({ "verdicts": mirror.verdict_words(), "locked": locked, "collisions": collisions, "stall": stall });
        let operation = Operation::new(OperationId(98), RevisionId(2), Generation(1), 1);
        let mut revalidation = FillRevalidateJob::new(operation, fill_run_identity(), own_mesh_roots(blocker, kind_offset), placements.clone(), 1_000);
        drive_revalidation(&mut revalidation, operation);
        actual["revalidateConflicts"] = serde_json::json!(revalidation.conflicts());
        assert_eq!(&actual, expected, "{name}");
    }
}
//#endregion ⏯️FillRunJob

/// ⚖️ LAW: a rule refusal is a `warning` record, never a collision — a document whose target volume
/// lies away from every open vortex refuses each constructed candidate as `outside-target-volume`,
/// appends nothing and ends with the `no-free-placement` warning step.
#[test]
fn fill_run_job_reports_rule_refusals_as_warnings_and_the_stall_as_a_warning_step() {
    let roots = nakagin_scale_roots();
    let mut scene = (*roots.scene).clone();
    scene.fixture.target_volumes.push(WorldVolumeProps { id: "elsewhere".into(), origin: [1.0e6, 1.0e6, 1.0e6], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None });
    let mut job = FillRunJob::new(FillBuilder::begin_preparation(FillPreparationRoots::new(Arc::new(scene), roots.meshes.clone()), Operation::new(OperationId(89), RevisionId(1), Generation(1), 43), 10), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()], [0; 32]);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    let [tested, locked, collisions, rejected, marked] = job.counters();
    assert!(tested > 0 && tested == rejected && locked == 0 && collisions == 0 && marked > 0, "{:?}", job.counters());
    assert!(mirror.verdicts.iter().all(|(_, verdict, reason)| (*verdict == ToolRunVerdict::Warning && *reason == FillRunReason::OutsideTargetVolume.code()) || (*verdict == ToolRunVerdict::Danger && *reason == FillRunReason::VortexExhausted.code())));
    assert_eq!(mirror.verdicts.iter().filter(|(_, _, reason)| *reason == FillRunReason::VortexExhausted.code()).count() as u64, marked, "every vortex whose candidates were all refused is marked once");
    assert!(mirror.ops.is_empty() && mirror.entities.is_empty());
    let last = mirror.steps.last().expect("stall step");
    assert_eq!((last.kind, last.reason, last.args.clone()), (ToolRunStepKind::Warning, FillRunReason::NoFreePlacement.code(), vec![ToolRunStepArg::Unsigned(0)]));
    assert_eq!(mirror.progress.as_ref().map(|progress| progress.state), Some(ToolRunState::Complete));
}


/// 🏁️ How a run ended as its ledger saw it: `None` when the last step is `success:requested-reached` over a met request, else
/// the declared stall reason its `warning` step names. A run that ended without a step, with an undeclared reason, or with
/// zero verdicts and no stall reason fails the law.
fn fill_run_visible_end(name: &str, job: &FillRunJob, mirror: &FillRunMirror, requested: usize) -> Option<&'static str> {
    let [tested, locked, _, _, _] = job.counters();
    assert_eq!(mirror.progress.as_ref().map(|progress| progress.state), Some(ToolRunState::Complete), "{name}: the run's last progress is complete");
    let last = mirror.steps.last().unwrap_or_else(|| panic!("{name}: the run ended with counters {:?} and no step", job.counters()));
    assert_eq!(last.args, vec![ToolRunStepArg::Unsigned(locked)], "{name}: the terminal step carries the placement count");
    match (last.kind, FillRunReason::from_code(last.reason)) {
        (ToolRunStepKind::Success, Some(FillRunReason::RequestedReached)) => {
            assert!(locked >= requested as u64 && tested >= locked && requested > 0, "{name}: requested-reached with {locked} of {requested} placed and {tested} tested");
            None
        }
        (ToolRunStepKind::Warning, Some(reason)) if FillStall::ALL.iter().any(|stall| stall.reason() == reason) => Some(reason.id()),
        (kind, reason) => panic!("{name}: the run ended with {kind:?} {reason:?} after {tested} verdicts instead of a declared terminal reason"),
    }
}

/// 🏁️ LAW (language-neutral fixture `🎞️fill-run.json` `laws.visibleEnd`, ticket lane W1-H): every fill run ends where the user
/// can see why — every shipped case and every own-mesh variant ends with `success:requested-reached` or a `warning` step
/// naming a declared stall reason, never with zero verdicts and no reason. The own-mesh Nakagin variant packs more bodies
/// into one spatial cell than one bookkeeping page holds; it must still be planned and match its declared outcome.
#[test]
fn fill_run_ends_visibly_with_a_declared_reason_for_every_case_and_own_mesh_variant() {
    let fixture: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let (document, seed, requested) = (case["document"].as_str().expect("document"), case["seed"].as_u64().expect("seed"), case["requested"].as_u64().expect("requested") as usize);
        let (roots, lane, _) = example_fill_roots(document, seed as u32);
        let mut job = fill_run_job(roots, lane, seed, requested);
        let mut mirror = FillRunMirror::new();
        mirror.drive(&mut job, u64::MAX, 1_000_000);
        let name = format!("{document} seed {seed} requested {requested}");
        assert_eq!(fill_run_visible_end(&name, &job, &mirror, requested), case["expected"]["stall"].as_str(), "{name}");
    }
    let mut disagreements = Vec::new();
    for case in fixture["laws"]["visibleEnd"]["cases"].as_array().expect("visible end cases") {
        let (document, seed, requested, scale) = (case["document"].as_str().expect("document"), case["seed"].as_u64().expect("seed"), case["requested"].as_u64().expect("requested") as usize, case["ownMeshScale"].as_f64().expect("own mesh scale") as f32);
        let (roots, lane, _) = own_mesh_fill_roots(document, seed as u32, scale);
        let mut job = fill_run_job(roots, lane, seed, requested);
        let mut mirror = FillRunMirror::new();
        mirror.drive(&mut job, u64::MAX, 1_000_000);
        let name = format!("{document} seed {seed} requested {requested} own-mesh {scale}");
        let stall = fill_run_visible_end(&name, &job, &mirror, requested);
        let [tested, locked, collisions, rejected, marked] = job.counters();
        assert!(tested > 0 || stall.is_some(), "{name}: zero verdicts always name their stall");
        let actual = serde_json::json!({ "tested": tested, "locked": locked, "collisions": collisions, "rejected": rejected, "marked": marked, "stall": stall });
        if actual != case["expected"] {
            disagreements.push(format!("{name}: actual {actual}"));
        }
    }
    assert!(disagreements.is_empty(), "the visible-end law disagrees:\n{}", disagreements.join("\n"));
}

/// 🚧️ LAW (ticket lane W1-H): an artifact page that fills mid-plan ends the run with the `artifact-capacity` warning step over
/// what was already placed, the candidate under test reaching its verdict first — never a bare `Complete` the ledger cannot
/// explain.
#[test]
fn fill_run_job_mid_plan_capacity_stall_ends_with_a_visible_warning_step() {
    let (roots, lane, _) = example_fill_roots("concrete-forest", 7);
    let mut job = fill_run_job(roots, lane, 7, 8);
    let mut mirror = FillRunMirror::new();
    let mut sequence = 0;
    loop {
        match fill_run_turn(&mut job, u64::MAX, &mut sequence) {
            FillRunTurn::Tick(tick) => mirror.apply(tick),
            FillRunTurn::Checkpoint(bytes) => {
                mirror.checkpoints.push(bytes);
                break;
            }
            FillRunTurn::Complete => panic!("the run completed before its first placement"),
            FillRunTurn::Yield => {}
        }
    }
    job.builder.collection_over_capacity = true;
    mirror.drive(&mut job, u64::MAX, 1_000);
    assert_eq!(fill_run_visible_end("mid-plan capacity", &job, &mirror, 8), Some(FillRunReason::ArtifactCapacity.id()));
    let [tested, locked, _, _, _] = job.counters();
    assert_eq!((locked, mirror.verdicts.len() as u64), (1, tested), "one placement, and every tested candidate reached its verdict");
}
