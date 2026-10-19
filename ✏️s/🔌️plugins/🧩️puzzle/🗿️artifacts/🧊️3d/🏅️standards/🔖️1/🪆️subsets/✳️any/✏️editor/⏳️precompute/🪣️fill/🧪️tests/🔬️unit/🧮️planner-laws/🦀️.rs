/// 🧮️ Planner laws: the constructor cap and plus-one handback, the fixed collection pages, the cooperative transitions, the work and watchdog ceilings, the capacity law and the Nakagin-scale plan.

#[test]
fn constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently() {
    #[derive(Clone, Copy)]
    enum HostileRoot {
        EngineSceneObjects,
        SceneAttractions,
        SceneTargetVolumes,
        Meshes,
        CatalogObjects,
        CatalogVortices,
        CatalogCables,
        KindCompatibility,
        ObjectWeights,
        VortexWeights,
    }
    let object = |index| EngineSceneObject { id: format!("object-{index:02}"), object_kind: None, anchor: Default::default(), mesh_url: None, origin: [0.0; 3], orientation: None, scale: None, vortices: Vec::new() };
    let body = collision_body_from_buffers(&[0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 4.0, 0.0], &[0, 1, 2]).expect("body");
    let roots = |branch: HostileRoot, count| {
        let mut scene =
            SceneConfig { scene_snapshot: EngineSceneSnapshot::default(), kind_catalogs: Some(KindCatalogBundle::default()), kind_compatibility: Vec::new(), contact_tolerance: 0.0, seed: 31, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        let mut meshes = FillMeshMap::new();
        match branch {
            HostileRoot::EngineSceneObjects => scene.scene_snapshot.objects.extend((0..count).map(object)),
            HostileRoot::SceneAttractions => scene.scene_snapshot.attractions.extend((0..count).map(|index| AttractionProps {
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
            HostileRoot::SceneTargetVolumes => scene.scene_snapshot.target_volumes.extend((0..count).map(|index| WorldVolumeProps { id: format!("volume-{index:02}"), origin: [0.0; 3], orientation: None, scale: None })),
            HostileRoot::Meshes => {for index in 0..count{meshes.cold_insert(format!("mesh-{index:02}"),body.clone());}},
            HostileRoot::CatalogObjects => scene.kind_catalogs.as_mut().expect("catalogs").objects.extend((0..count).map(|index| ObjectKind { id: format!("catalog-object-{index:02}"), ..Default::default() })),
            HostileRoot::CatalogVortices => scene.kind_catalogs.as_mut().expect("catalogs").vortices.extend((0..count).map(|index| VortexKindCatalog { id: format!("catalog-vortex-{index:02}"), ..Default::default() })),
            HostileRoot::CatalogCables => scene.kind_catalogs.as_mut().expect("catalogs").cables.extend((0..count).map(|index| CableKindCatalog { id: format!("catalog-cable-{index:02}"), ..Default::default() })),
            HostileRoot::KindCompatibility => {
                scene.kind_compatibility.extend((0..count).map(|index| KindCompatEntry { source: format!("compat-{index:02}"), target: format!("target-{index:02}"), bidirectional: false, important: false, specificity: None }))
            }
            HostileRoot::ObjectWeights => {for index in 0..count{scene.weights.object_weights.cold_insert(format!("object-weight-{index:04}"),index as f64+0.25);}},
            HostileRoot::VortexWeights => {for index in 0..count{scene.weights.vortex_weights.cold_insert(format!("vortex-weight-{index:04}"),index as f64+0.5);}},
        }
        FillPreparationRoots::new(Arc::new(scene), Arc::new(meshes))
    };
    let branches = [
        (HostileRoot::EngineSceneObjects, "scene_snapshot-objects", DOCUMENT_OBJECT_SLOTS),
        (HostileRoot::SceneAttractions, "scene-attractions", DOCUMENT_ATTRACTION_SLOTS),
        (HostileRoot::SceneTargetVolumes, "scene-target-volumes", DOCUMENT_VOLUME_SLOTS),
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
        let mut accepted = FillBuilderFixture::new(FillBuilder::begin_preparation(roots(branch, cap), operation, TEST_REQUESTED_COUNT));
        let mut turns = 0;
        while accepted.stage != FillJobStage::PrepareTargets {
            accepted.prepare_one();
            turns += 1;
            assert!(turns < 16 * DOCUMENT_OBJECT_SLOTS, "{expected_branch} cap preparation must advance in bounded turns");
        }
        assert!(turns >= cap, "{expected_branch} cap must be installed cooperatively");

        let mut rejected = FillBuilderFixture::new(FillBuilder::begin_preparation(roots(branch, cap + 1), operation, TEST_REQUESTED_COUNT));
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
        let JobTurn::Fault(fault) = rejected.advance(100,10) else { panic!("{expected_branch} cap + 1 faults the planner before it installs anything") };
        assert_eq!(fault.as_slice(), format!("preparation-capacity:{expected_branch}:{cap}").as_bytes());
        assert!(faulted(JobTurn::Fault(fault)));
        assert!(faulted(rejected.advance(100,10)), "the refusal is permanent: every later step faults again");
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
    assert!(faulted(builder.advance_foreign(1,1)));
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
    builder.cancel();
    assert_eq!(builder.advance(100,10), JobTurn::Cancelled);
}

#[test]
fn stale_generation_faults_without_progress() {
    fn now() -> Option<u64> {
        Some(0)
    }
    let mut builder = empty_builder();
    let JobTurn::Fault(fault) = builder.advance_foreign(100,10) else { panic!("a stale generation must fault") };
    assert_eq!(fault.as_slice(), b"stale-fill-operation".as_slice());
    assert!(faulted(JobTurn::Fault(fault)));
    assert_eq!(builder.operation.base_revision, RevisionId(1));
}

/// 📏️ Turns an EMPTY scene's cursorized planner needs to reach `Complete`: seven preparation stages
/// (three of them walking three empty roots apiece), then target preparation and the stall, one
/// bounded unit per turn. Doubled as headroom for a stage split.
const EMPTY_FILL_TRANSITION_TURNS: usize = 64;

#[test]
fn empty_fill_transition_stays_below_watchdog_ceiling() {
    let mut builder = empty_builder();
    for _ in 0..EMPTY_FILL_TRANSITION_TURNS {
        let before = precompute_work_done();
        let _ = builder.advance(100,10);
        let work = precompute_work_done() - before;
        assert!(work <= FILL_STEP_WORK_CEILING, "one empty-scene step cost {work} units of primitive work, over {FILL_STEP_WORK_CEILING}");
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
    let host = EngineSceneObject {
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
    objects.extend((0..30).map(|index| EngineSceneObject {
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
    let meshes = FillMeshMap::from([("/stress/box.glb".to_string(), body)]);
    let scene = Arc::new(SceneConfig {
        scene_snapshot: EngineSceneSnapshot { objects, attractions: Vec::new(), target_volumes: Vec::new() },
        kind_catalogs: Some(catalogs),
        kind_compatibility: Vec::new(),
        contact_tolerance: 0.0,
        seed: 29,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    let mut builder = FillBuilderFixture::new(FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(meshes)), Operation::new(OperationId(29), RevisionId(1), Generation(1), 29), TEST_REQUESTED_COUNT));
    let started = precompute_work_done();
    let mut first_candidate = None;
    for _ in 0..50_000 {
        let before = precompute_work_done();
        let outcome = builder.advance(100,10);
        let work = precompute_work_done() - before;
        assert!(work <= FILL_STEP_WORK_CEILING, "stage {:?} cost {work} units of primitive work, over {FILL_STEP_WORK_CEILING}", builder.stage);
        if first_candidate.is_none() && builder.current_preview.is_some() {
            first_candidate = Some(precompute_work_done() - started);
        }
        if outcome.is_terminal() {
            break;
        }
    }
    assert!(first_candidate.is_some_and(|work| work <= FILL_FIRST_CANDIDATE_WORK_CEILING), "adversarial fill did not publish its first candidate within {FILL_FIRST_CANDIDATE_WORK_CEILING} units: {first_candidate:?}");
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
    assert!(nakagin("objects") < DOCUMENT_OBJECT_SLOTS, "the flagship scene_snapshot leaves the object capacity room to plan into");
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

/// 🏢️ The flagship scene_snapshot at document scale — 180 capsules, 360 attractions, 12 object kinds, 18
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
    let objects: Vec<EngineSceneObject> = (0..NAKAGIN_OBJECTS)
        .map(|index| EngineSceneObject {
            id: format!("capsule-{index:03}"),
            object_kind: Some(format!("capsule-kind-{:02}", index % NAKAGIN_OBJECT_KINDS)),
            anchor: Default::default(),
            mesh_url: Some(NAKAGIN_MESH_URL.into()),
            origin: [(index % 12) as f64 * 64.0, (index / 12) as f64 * 64.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            vortices: vec![
                VortexProps { id: "v0".into(), vortex_kind: Some("port-00".into()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) },
                // 🧲️ On the capsule's apex, pointing out: a placement docks there flush. At the base center pointing INTO the
                // body it made every placement a duplicate of its host, which only passed while the host was never tested.
                VortexProps { id: "v1".into(), vortex_kind: Some("port-00".into()), position: [0.0, 0.0, 8.0], direction: Some([0.0, 0.0, 1.0]) },
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
        scene_snapshot: EngineSceneSnapshot { objects, attractions, target_volumes: Vec::new() },
        kind_catalogs: Some(catalogs),
        kind_compatibility,
        contact_tolerance: 0.0,
        seed: 43,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    FillPreparationRoots::new(scene, Arc::new(FillMeshMap::from([(NAKAGIN_MESH_URL.to_string(), body)])))
}

/// 🏢️ Drives a document-scale preparation until it places its first object, refusing to accept a
/// fault, a capacity refusal, or a terminal outcome that placed nothing.
fn drive_nakagin_scale_fill() -> FillBuilderFixture {
    let mut builder = FillBuilderFixture::new(FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(43), RevisionId(1), Generation(1), 43), TEST_REQUESTED_COUNT));
    assert!(builder.preparation_capacity_refusal.is_none(), "a Nakagin-scale document must not be refused before preparation starts");
    let mut turns = 0;
    while builder.sequence.is_empty() {
        let outcome = builder.advance(100,10);
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
/// block for `FixedOwnerVec<EngineSceneObject, DOCUMENT_OBJECT_SLOTS>`, one 96 KiB block for the
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
fn drive_until_settled(builder: &mut FillBuilderFixture, turns: usize) {
    for _ in 0..turns {
        if matches!(builder.stage, FillJobStage::Complete(_)) {
            return;
        }
        let outcome = builder.advance(100,10);
        assert!(!faulted(outcome), "a settled planner never faults: {:?} / end {:?}", builder.last_rejection, builder.end());
    }
    panic!("planner did not settle in {turns} turns at stage {:?} with {} placements", builder.stage, builder.sequence.len());
}

fn nakagin_plan(requested: usize, turns: usize) -> FillBuilderFixture {
    let mut builder = FillBuilderFixture::new(FillBuilder::begin_preparation(nakagin_scale_roots(), Operation::new(OperationId(61), RevisionId(1), Generation(1), 43), requested));
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
    let mut builder = FillBuilderFixture::new(FillBuilder::begin_preparation(roots, Operation::new(OperationId(101), RevisionId(1), Generation(1), 1), 1_000_000));
    builder.observe_run();
    let mut events=Vec::new();
    let (mut last_target, mut open): (Option<String>, Option<String>) = (None, None);
    let mut marked = std::collections::HashSet::new();
    let (mut placements, mut retries) = (0usize, 0usize);
    for _ in 0..50_000_000 {
        if matches!(builder.stage, FillJobStage::Complete(_)) {
            break;
        }
        let outcome = builder.advance(100,10);
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
