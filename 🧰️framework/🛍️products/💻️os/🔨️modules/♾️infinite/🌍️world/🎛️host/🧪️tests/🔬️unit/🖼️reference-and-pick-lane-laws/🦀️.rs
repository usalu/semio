/// 🖼️ Laws of the reference underlay, scene lanes, the pick target lane, camera framing and component overlays.

/// 🖼️ The reference underlay is OFFERED, never given away, and a refused admission is back-pressure
/// rather than a frame fault.
///
/// Two regressions meet here. Reporting the refusal as a fault killed the page on a dock-window close
/// (`frame world resource admission exceeded fixed credits`), and taking the payload to offer it only
/// once left both panes with no plan at all when that one frame's present was aborted (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` §1 and W9b's regression).
#[test]
fn a_reference_underlay_is_offered_every_render_and_a_refused_admission_is_not_a_fault() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.scene.reference_pixels.insert("/plan.jpg".into(), test_world_scene_raster(4, 2, 7)).expect("reference pixels admitted");
    for render in 0..4 {
        let offered = reference_underlay_upload(&state.scene, "/plan.jpg").expect("every render offers the payload");
        assert_eq!(offered.identity().descriptor().byte_len(), Some(32), "🖼️ render {render}");
    }
    assert_eq!(state.scene.reference_pixels.get("/plan.jpg").and_then(|raster| raster.identity.descriptor().byte_len()), Some(32), "🖼️ and the surface keeps the immutable identity while the pool owns pixels");
    assert_eq!(reference_image_aspect(&state.scene, "/plan.jpg"), 2.0, "🖼️ the dimensions answer the aspect every frame reads");
    assert!(reference_underlay_upload(&state.scene, "/missing.jpg").is_none());

    let pool = world_scene_raster_pool();
    let over_budget = (ui_wgpu::wgpu::SCENE_RASTER_ITEM_BYTES / 4 + 1) as u32;
    let descriptor = SceneRasterDescriptor { width: over_budget, height: 1, source_digest: [91, 92], source_revision: 91, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    assert!(matches!(pool.begin(descriptor, 91, SceneRasterWriteMode::Moved { capacity_bytes: descriptor.byte_len().unwrap() }), SceneRasterBegin::Refused(_)), "🧯️ the full-quality pool refuses item +1 before it owns an allocation");
}

#[test]
fn six_reference_gpu_residents_progress_through_two_cpu_slots_and_a_lost_texture_resumes_its_source() {
    fn publish(pool: &SceneRasterPool, descriptor: SceneRasterDescriptor, owner: u64, fill: u8) -> SceneRasterLease {
        for _ in 0..=2 {
            match pool.begin(descriptor, owner, SceneRasterWriteMode::Streamed) {
                SceneRasterBegin::Writer(writer) => {
                    let writer = pool.push(writer, &[fill; 8]).expect("one exact row");
                    return pool.seal(writer).expect("reference seal");
                }
                SceneRasterBegin::Backpressure("scene raster retirement is pending") => assert!(!pool.maintenance_step()),
                _ => panic!("reference CPU slot must make bounded progress"),
            }
        }
        panic!("reference CPU slot did not progress after one bounded retirement");
    }

    let limits = ui_wgpu::wgpu::SceneRasterPoolLimits { item_bytes: 16, pool_bytes: 32, slot_capacity: 2, lease_capacity_per_slot: 4, transfer_bytes: 8, retire_bytes: 8, gpu_resident_bytes: 48 };
    let pool = SceneRasterPool::try_new(limits).expect("reference pool");
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut gpu = Vec::new();
    let mut identities = Vec::new();
    for revision in 1u64..=6 {
        let url = format!("/reference-{revision}.png");
        state.scene.references.push(WorldReferenceRecord { url: Some(url.clone()), origin: Some([0.0; 3]), width_world: Some(2.0), hidden: Some(false), ..Default::default() });
        let descriptor = SceneRasterDescriptor { width: 2, height: 1, source_digest: [revision, revision.rotate_left(17)], source_revision: revision, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
        let lease = publish(&pool, descriptor, revision, revision as u8);
        let identity = lease.identity();
        state.scene.reference_pixels.insert(url.clone(), WorldSceneRaster { identity, pending: Mutex::new(Some(lease)) }).expect("reference identity");
        let upload = world_raster_upload_in_pool(&state.scene, &url, &url, false, &pool).expect("pending CPU upload");
        let mut producer = World3dBuildContext::new(WorldCursorWakeAuthority::new());
        producer.ensure_world_plane_texture(&url, upload);
        let mut input = ui_wgpu::wgpu::PreparedRenderInput::try_new(revision, revision, ui_wgpu::wgpu::DrawList::default(), None, 0.0).unwrap_or_else(|mut rejected| {
            let fault=rejected.fault();while !rejected.close_step() {}
            panic!("prepared reference input admission: {fault}");
        });
        assert!(matches!(producer.append_step(&mut input), Ok(false)), "lease enters actual prepared upload");
        assert!(matches!(producer.append_step(&mut input), Ok(false)), "producer retires its exact key");
        assert!(matches!(producer.append_step(&mut input), Ok(true)), "producer reaches terminal");
        let PreparedRenderUpload::SceneRaster { key, lease } = input.uploads.get(0).expect("actual producer publishes the scene lease") else { panic!("scene raster upload") };
        assert_eq!(key, &url);
        gpu.push(lease.release_witness().expect("CPU witness").commit_gpu(&url).expect("GPU commit"));
        assert!(world_reference_raster_ready_in_pool(&state.scene, &url, &pool));
        identities.push(identity);
        drop(input);
        let mut retired = false;
        for _ in 0..64 {
            if matches!(ui_wgpu::wgpu::PreparedRenderInput::close_abandoned_step(world_original_fixture_grant()),semio_framework_job::InteractiveJobCloseStep::Complete{..}) {
                retired = true;
                break;
            }
        }
        assert!(retired, "each submitted frame retires before the next producer admission");
    }
    assert_eq!(pool.reserved_bytes(), 16, "six resident textures retain only two CPU backing slots");
    assert_eq!(pool.gpu_resident_bytes(), 48);
    assert!(!pool.cpu_resident(identities[0]), "the oldest source pixels were evicted after later GPU commits");

    drop(gpu.remove(0));
    assert!(!world_reference_raster_ready_in_pool(&state.scene, "/reference-1.png", &pool), "GPU loss plus CPU eviction makes the exact source requestable again");
    let request = reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::ReferenceImage, "/reference-1.png").expect("lost source is requestable");
    let owner = take_next_world3d_asset(&mut state.assets).expect("lost source reaches actual fetch ownership");
    assert_eq!(owner.token(), request);
    assert_eq!(owner.url(), "/reference-1.png");
    assert_eq!(owner.kind(), WorldAssetRequestKind::ReferenceImage);
    return_world3d_asset(&mut state.assets, owner).unwrap_or_else(|_| panic!("source owner returns for bounded close"));

    let descriptor = identities[0].descriptor();
    let replacement = publish(&pool, descriptor, 100, 1);
    assert!(apply_decoded_reference_raster(&mut state.session, &mut state.authority, &mut state.scene, &mut state.assets, "/reference-1.png", replacement));
    assert!(world_reference_raster_ready_in_pool(&state.scene, "/reference-1.png", &pool));
    assert_eq!(reference_image_aspect(&state.scene, "/reference-1.png"), 2.0, "recovery preserves the decoded natural dimensions");
    state.assets.asset_io.begin_close();
    while !state.assets.asset_io.close_step() {}
    assert!(state.assets.asset_io.terminal_is_empty());
}

fn topology_mesh() -> Mesh3dLease {
    let mut data = mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2, 1, 3, 2]);
    data.face_ids = vec![10, 11];
    data.vertex_ids = vec![1, 2, 3, 4];
    data.edge_positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
    data.edge_ids = vec![5, 6];
    publish_oracle_mesh(data)
}

fn scene_with_selection(selection_json: &str) -> UiComponentSceneNode {
    scene_with_selection_and_domain(selection_json, None)
}

/// 🪟️ Like `scene_with_selection`, but also stamps `World3dScene.domain_id`/`domain_granularity_id`
/// when `domain` is `Some((domain_id, granularity_id))` — exercises the app-bound-domain path of
/// `resolved_domain_id`/`resolved_domain_granularity_id`/`resolved_item_id`.
fn scene_with_selection_and_domain(selection_json: &str, domain: Option<(&str, &str)>) -> UiComponentSceneNode {
    UiComponentSceneNode {
        presence: UiPresence::default(),
        host_id: "surface-1".into(),
        surface_id: "surface-1".into(),
        controller_id: "controller-1".into(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: Some(World3dScene {
            snapshot: None,
            camera_json: r#"{"position":[4.0,4.0,4.0],"target":[0.0,0.0,0.0],"up":[0.0,0.0,1.0],"fov":45.0}"#.into(),
            meshes_json: r#"[{"id":"mesh-1","data":{"positions":[0,0,0,1,0,0,0,1,0],"normals":[0,0,1,0,0,1,0,0,1],"indices":[0,1,2],"faceIds":[10],"vertexIds":[1,2,3],"edgePositions":[0,0,0,1,0,0],"edgeIds":[5]}}]"#.into(),
            instances_json: r#"[{"id":"obj-1","meshId":"mesh-1","position":[0,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into(),
            instances_delta_json: None,
            selection_json: selection_json.into(),
            vortices_json: None,
            attractions_json: None,
            target_volumes_json: None,
            references_json: None,
            brush_preview_json: None,
            interaction_json: None,
            engagement_preview_json: None,
            pick_targets_json: None,
            lod_json: None,
            presentation_json: None,
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            tool_run_trace: None,
            annotations: None,
            scalar_field: None,
            modelling_options: None,
            domain_id: domain.map(|(id, _)| id.to_string()),
            domain_granularity_id: domain.map(|(_, granularity)| granularity.to_string()),
            lanes: Vec::new(),
        }),
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    }
}

#[test]
fn sync_parses_scene_reference_lanes() {
    let references = r#"[{"id":"ref-masterarbeit","url":"/infinite-assets/plan.jpg","origin":[7.0,0.0,0.01],"widthWorld":50.0,"locked":true,"hidden":false}]"#;
    let mut scene = scene_with_selection("{}");
    if let Some(world) = scene.world_3d.as_mut() {
        world.references_json = Some(references.into());
    }
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.scene.references.len(), 1);
    assert_eq!(state.scene.references[0].url.as_deref(), Some("/infinite-assets/plan.jpg"));
    assert_eq!(state.scene.references[0].width_world, Some(50.0));
    assert!(!state.scene.references[0].hidden.unwrap_or(true));
}

/// 🤝️ One world surface fed the `engagementPreview` lane a construction statechart publishes —
/// the four kinds React's `EngagementPreviewLayer` draws, in the spelling `cad`'s
/// `interaction::preview_display_items` emits.
const ENGAGEMENT_PREVIEW_JSON: &str = r#"[
    {"kind":"point","role":"corner","position":[1.0,2.0,0.0]},
    {"kind":"segment","role":"edge","from":[0.0,0.0,0.0],"to":[1.0,0.0,0.0]},
    {"kind":"box-preview","cornerA":[0.0,0.0,0.0],"cornerB":[2.0,3.0,0.0],"height":4.0},
    {"kind":"linear-handle","axis":[0.0,0.0,2.0],"origin":[1.0,1.0,0.0]}
]"#;

fn scene_with_engagement_preview(preview_json: Option<&str>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection("{}");
    if let Some(world) = scene.world_3d.as_mut() {
        world.engagement_preview_json = preview_json.map(str::to_string);
    }
    scene
}

#[test]
fn sync_parses_the_engagement_preview_lane() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some(ENGAGEMENT_PREVIEW_JSON)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.scene.engagement_preview.len(), 4);
    assert_eq!(state.scene.engagement_preview[0].kind.as_deref(), Some("point"));
    assert_eq!(state.scene.engagement_preview[0].position, Some([1.0, 2.0, 0.0]));
    assert_eq!(state.scene.engagement_preview[1].from, Some([0.0, 0.0, 0.0]));
    assert_eq!(state.scene.engagement_preview[2].corner_b, Some([2.0, 3.0, 0.0]));
    assert_eq!(state.scene.engagement_preview[2].height, Some(4.0));
    assert_eq!(state.scene.engagement_preview[3].axis, Some([0.0, 0.0, 2.0]));
}

#[test]
fn an_absent_or_broken_engagement_preview_lane_leaves_the_state_empty() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(None), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.scene.engagement_preview.is_empty());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some("not json")), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.scene.engagement_preview.is_empty());
}

/// 🖍️ 3 axis crosses (2 vertices each) + 1 segment + 12 box edges + 1 handle = 6 + 2 + 24 + 2 = 34
/// line vertices, and the box wireframe extrudes UP from `cornerA.z` by `height`.
#[test]
fn the_engagement_preview_lane_paints_world_lines() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some(ENGAGEMENT_PREVIEW_JSON)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let mut lines = Vec::new();
    append_engagement_preview_lines(&state.scene, &mut lines, [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(lines.len(), 34);
    let zs: Vec<f32> = lines.iter().map(|vertex| vertex.position[2]).collect();
    assert!(zs.iter().any(|z| (*z - 4.0).abs() < 1e-6), "the box top sits at cornerA.z + height");
    assert!(lines.iter().all(|vertex| vertex.color == [1.0, 0.0, 0.0, 1.0]));
}

/// 🩹️ React floors width/depth/height at `0.05`, so a degenerate footprint still shows a sliver.
#[test]
fn a_degenerate_box_preview_keeps_the_react_minimum_extent() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some(r#"[{"kind":"box-preview","cornerA":[0.0,0.0,0.0],"cornerB":[0.0,0.0,0.0]}]"#)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let mut lines = Vec::new();
    append_engagement_preview_lines(&state.scene, &mut lines, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(lines.len(), 24);
    let max_x = lines.iter().map(|vertex| vertex.position[0]).fold(f32::MIN, f32::max);
    let min_x = lines.iter().map(|vertex| vertex.position[0]).fold(f32::MAX, f32::min);
    assert!((max_x - min_x - 0.05).abs() < 1e-6, "width floors at 0.05, got {}", max_x - min_x);
    let max_z = lines.iter().map(|vertex| vertex.position[2]).fold(f32::MIN, f32::max);
    assert!((max_z - 0.05).abs() < 1e-6, "height floors at 0.05, got {max_z}");
}

#[test]
fn an_engagement_preview_item_missing_its_geometry_is_skipped_not_faulted() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(
        &mut state,
        &scene_with_engagement_preview(Some(r#"[{"kind":"point"},{"kind":"segment","from":[0,0,0]},{"kind":"box-preview","cornerA":[0,0,0]},{"kind":"linear-handle","origin":[0,0,0]},{"kind":"unknown-kind","position":[1,1,1]}]"#)),
        Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 },
    );
    assert_eq!(state.scene.engagement_preview.len(), 5);
    let mut lines = Vec::new();
    append_engagement_preview_lines(&state.scene, &mut lines, [0.0, 0.0, 0.0, 1.0]);
    assert!(lines.is_empty());
}

//#region 🧲️PickTargetLane
/// 🧲️ One pane's `pickTargets` lane as `cad`'s `pick_target_lane_items` publishes it: a vertex on
/// the orbit target, an edge through it, and a locked face that RENDERS but is never hit.
const PICK_TARGETS_JSON: &str = r#"[
    {"kind":"object","id":"solid-1","point":[0.0,0.0,0.0],"points":[[-2.0,-2.0,-2.0],[2.0,2.0,2.0]],"typology":"wall","selectable":true,
     "style":{"color":"--muted-foreground","emissive":"--hover-panel","opacity":0.28,"lineWidth":7.0}},
    {"kind":"vertex","id":"v-1","point":[0.0,0.0,0.0],"points":[],"selectable":true,
     "style":{"color":"--foreground","emissive":"--hover-base","opacity":1.0,"lineWidth":5.0}},
    {"kind":"face","id":"f-locked","point":[0.0,0.0,0.0],"points":[[-1.0,-1.0,0.0],[1.0,1.0,0.0]],"selectable":false,
     "style":{"color":"--accent-secondary","emissive":"--hover-window","opacity":0.042,"lineWidth":4.0}}
]"#;

fn scene_with_pick_targets(pick_targets_json: Option<&str>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection_and_domain("{}", Some(("cad", "object")));
    if let Some(world) = scene.world_3d.as_mut() {
        world.pick_targets_json = pick_targets_json.map(str::to_string);
    }
    scene
}

/// 🧲️ A pane aiming straight down the world -Z axis at the origin, with the lane loaded — the
/// smallest scene in which a ray hits every pick target at once, so what the laws below observe is
/// the ORDERING, not the geometry.
fn pick_target_state(pick_targets_json: &str) -> World3dState {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = state.session.bounds;
    state.session.active_utility = "select".into();
    state.session.bound_domain_id = Some("cad".into());
    state.session.bound_domain_granularity_id = Some("object".into());
    state.selection.pick_targets = world_pick_targets_from_json(pick_targets_json);
    state
}

fn pick_target_ray(state: &World3dState) -> (Vec3, Vec3) {
    let camera = state.camera.orbit.to_camera();
    camera.ray_from_screen(200.0, 200.0, 400.0, 400.0)
}

/// 🧲️ LAW (packet W14g item 1): the `pickTargets` lane is READ, bounded, hit-tested with React's own
/// `targetRayScore` ordering, and dispatched as `interactionSelect`/`interactionHover` at the
/// target's own granularity.
///
/// ⚖️ Four things at once, because they are one mechanism: a lane that parses but never hit-tests is
/// exactly the gap this packet closed (`📓️w2f-cad-spatial-editor-wgpu.md` §5.1 — the overlay could
/// be SHOWN and not picked).
///
/// 🖱️ A real click through the retained authority — the pick target beats the instance under it.
///
/// 🎯️ `targets` is JSON TEXT on the wire (see `push_interaction_targets`), never a nested array.
#[test]
fn the_pick_target_lane_hit_tests_finest_first_skips_unselectable_rows_and_dispatches_react_payloads() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_pick_targets(Some(PICK_TARGETS_JSON)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.selection.pick_targets.len(), 3, "the lane is parsed off the scene, not only off a hand-built state");

    let mut state = pick_target_state(PICK_TARGETS_JSON);
    let (origin, direction) = pick_target_ray(&state);
    let scores: Vec<Option<f32>> = state.selection.pick_targets.iter().map(|target| world_pick_target_ray_score(target, origin, direction)).collect();
    assert!(scores.iter().all(Option::is_some), "the ray through the origin crosses every padded box");
    let vertex = scores[1].expect("vertex score");
    let object = scores[0].expect("object score");
    assert!(object < vertex, "the solid's front face is nearer than its center vertex ({object} < {vertex})");
    state.selection.pick_targets[0].points = vec![[-0.04; 3], [0.04; 3]];
    let tied_object = world_pick_target_ray_score(&state.selection.pick_targets[0], origin, direction).expect("coincident padded object score");
    assert!(vertex < tied_object, "React's spatialPickPriority makes the finest kind win at coincident padded bounds ({vertex} < {tied_object})");

    let modifiers = PointerModifiers::default();
    enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_button(200.0, 200.0, true, 0, &modifiers)).expect("press admitted");
    enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_button(200.0, 200.0, false, 0, &modifiers)).expect("release admitted");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut published: Vec<ActionDescriptor> = Vec::new();
    let mut turns = 0;
    while let Some(generation) = world3d_interaction_front_generation(&state.camera, &state.authority) {
        let step = with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context));
        assert_ne!(step, WorldInteractionAuthorityStep::Fault, "a pick-target click never faults the authority");
        published.extend(take_actions(&mut input));
        turns += 1;
        assert!(turns < 4096, "a bounded pick terminates");
        if step == WorldInteractionAuthorityStep::Idle {
            break;
        }
    }
    let select = published.iter().find(|action| action.action == "interactionSelect").expect("a sub-object pick publishes interactionSelect");
    let args = select.args.as_ref().expect("interactionSelect carries args");
    assert_eq!(args.get("domainId").and_then(semio_framework_value::DslValue::as_str), Some("cad"), "the bound domain, not the world fallback");
    let targets = args.get("targets").and_then(semio_framework_value::DslValue::as_str).expect("targets is JSON text");
    assert!(targets.contains(r#""granularity":"vertex""#), "the WINNING target's own kind is the granularity: {targets}");
    assert!(targets.contains(r#""id":"v-1""#), "the kernel entity id, not the instance id: {targets}");
    assert!(!targets.contains("f-locked"), "an unselectable (hidden/locked) row is rendered and never hit: {targets}");
}

//#region 🌫️InstanceStyleWire
/// 🌫️ LAW (packet W14g items 3 + 4): the instance lane carries the per-instance OPACITY a locked
/// object is dimmed by, and the `highlighted`/`disabled`/`celebrating` flags that reach the three
/// `MESH_STYLE_PAINT` rows nothing published before.
///
/// 🎨️ Each flag resolves to its OWN row of React's table, not to the two rows that existed here.
#[test]
fn the_instance_lane_carries_locked_opacity_and_the_three_unreachable_style_rows() {
    let instances = r##"[
        {"id":"obj-1","meshId":"mesh-1","position":[0,0,0],"color":"#ffffff","disabled":true,"opacity":0.35},
        {"id":"obj-2","meshId":"mesh-1","position":[1,0,0],"color":"#ffffff","highlighted":true},
        {"id":"obj-3","meshId":"mesh-1","position":[2,0,0],"color":"#ffffff","celebrating":true}
    ]"##;
    let mut scene = scene_with_selection("{}");
    if let Some(world) = scene.world_3d.as_mut() {
        world.instances_json = instances.into();
    }
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.selection.disabled_instance_ids.contains("obj-1"));
    assert!(state.selection.highlighted_instance_ids.contains("obj-2"));
    assert!(state.selection.celebrating_instance_ids.contains("obj-3"));
    let alpha = state.scene.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == "obj-1").map(|instance| instance.color[3]).expect("the locked instance reaches a draw");
    assert!((alpha - 0.35).abs() < 1e-3, "WORLD_LOCKED_OPACITY_SCALE multiplies the authored alpha, got {alpha}");
    let opaque = state.scene.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == "obj-2").map(|instance| instance.color[3]).expect("obj-2 reaches a draw");
    assert!((opaque - 1.0).abs() < 1e-3, "an instance with no opacity keeps its authored alpha, got {opaque}");
    assert_eq!(resolve_mesh_style(MeshStyleState { disabled: true, ..MeshStyleState::default() }), MeshStyleKind::Disabled);
    assert_eq!(resolve_mesh_style(MeshStyleState { highlighted: true, ..MeshStyleState::default() }), MeshStyleKind::Highlighted);
    assert_eq!(resolve_mesh_style(MeshStyleState { celebrating: true, ..MeshStyleState::default() }), MeshStyleKind::Celebrated);
    retire_bridged_surface(&mut state);
}

/// 🎨️ LAW: actual JSON bridge ingestion retains the authority that supplied a neutral color; the
/// semantic case crosses the snapshot as white and is resolved from the live theme only at paint.
#[test]
fn the_scene_bridge_retains_semantic_authored_and_environment_color_provenance() {
    for (id, authored, environment, expected_source) in [
        ("semantic", None, None, ui_wgpu::wgpu::SceneColorSource3d::SemanticNeutral),
        ("authored", Some("#3f80bf"), None, ui_wgpu::wgpu::SceneColorSource3d::Authored),
        ("environment", None, Some("#b82e0f"), ui_wgpu::wgpu::SceneColorSource3d::Environment),
    ] {
        let mut scene = scene_with_selection("{}");
        let world = scene.world_3d.as_mut().expect("world scene");
        world.instances_json = serde_json::json!([{
            "id": id,
            "meshId": "mesh-1",
            "position": [0, 0, 0],
            "rotation": [0, 0, 0, 1],
            "scale": [1, 1, 1],
            "color": authored,
        }])
        .to_string();
        world.environment_json = environment.map(|color| serde_json::json!({ "material": { "color": color } }).to_string());
        let mut state = World3dState::new(format!("surface-{id}"), "controller".into());
        drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 320.0, 240.0));
        let retained = state.scene.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == id).expect("bridged instance retained");
        assert_eq!(retained.material.color_source, expected_source);
        if expected_source == ui_wgpu::wgpu::SceneColorSource3d::SemanticNeutral {
            assert_eq!(retained.color, [1.0; 4], "semantic authority is not materialized as a baked gray");
        }
        retire_bridged_surface(&mut state);
    }
}
//#endregion 🌫️InstanceStyleWire

//#region 🎛️GumballConfig
/// 🎛️ LAW (packet W14g item 5): the gumball's handle set is the PLUGIN-AUTHORED `gumballConfig` when
/// the producer published one (`cad`'s dislocate utility does), the single-mode fallback otherwise,
/// and the drafting-plane subset intersects both — React's `gumballHandleEnabled`.
///
/// 🎛️ No authored config → React's `gumballConfigForTransformMode`, whose `rotate` arm offers the
/// rings ONLY. This host used to admit the three move axes unconditionally in every mode.
///
/// 📐️ A Top pane's `xy` drafting plane keeps the in-plane move/scale pair and the NORMAL rotation.
#[test]
fn the_gumball_offers_the_authored_config_then_the_transform_mode_fallback_intersected_with_the_plane() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(
        &mut state,
        &scene_with_selection(r#"{"transformMode":"transform","gumballConfig":{"moveAxes":true,"movePlanes":true,"rotate":true,"scaleAxes":false,"scalePlanes":false,"scaleUniform":false}}"#),
        Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 },
    );
    let config = world3d_gumball_config(&state.gumball);
    assert!(config.admits(GumballHandle::MoveX) && config.admits(GumballHandle::MoveXY) && config.admits(GumballHandle::RotateZ));
    assert!(!config.admits(GumballHandle::ScaleX), "an authored config that disables scaling hides AND unpicks the scale handles");

    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(r#"{"transformMode":"rotateSelection"}"#), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let config = world3d_gumball_config(&state.gumball);
    assert!(config.admits(GumballHandle::RotateX));
    assert!(!config.admits(GumballHandle::MoveX), "React's rotate mode offers no translation handle");
    assert!(!config.admits(GumballHandle::ScaleZ));

    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(r#"{"transformMode":"transform","gumballConfig":{"plane":"xy"}}"#), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let config = world3d_gumball_config(&state.gumball);
    assert!(config.admits(GumballHandle::MoveX) && config.admits(GumballHandle::MoveY) && config.admits(GumballHandle::MoveXY) && config.admits(GumballHandle::RotateZ));
    assert!(!config.admits(GumballHandle::MoveZ) && !config.admits(GumballHandle::RotateX) && !config.admits(GumballHandle::MoveYZ));
    assert!(config.admits(GumballHandle::ScaleX), "an ABSENT group key means enabled — React reads every group as `!== false`");
}
//#endregion 🎛️GumballConfig

//#region 🔘️VertexMarkerPixels
/// 🔘️ LAW (packet W14g item 5): a vertex marker is sized in SCREEN PIXELS
/// (`WORLD_VERTEX_DOT_PX`/`WORLD_VERTEX_MARK_PX`, `sizeAttenuation={false}`), so it keeps its
/// apparent size as the camera pulls away instead of collapsing to a sub-pixel world-unit cross —
/// the same defect React shipped and fixed on lowpoly.
///
/// 🩸️ The pre-W14g cross was `VERTEX_BASE_SCALE * 0.15` = 0.0075 world units, at ANY distance.
#[test]
fn vertex_markers_hold_their_pixel_size_at_every_camera_distance() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    let viewport = state.session.bounds;
    let near = {
        let mut orbit = state.camera.orbit.clone();
        orbit.distance = 4.0;
        orbit.to_camera()
    };
    let far = {
        let mut orbit = state.camera.orbit.clone();
        orbit.distance = 400.0;
        orbit.to_camera()
    };
    let centre = Vec3::new(0.0, 0.0, 0.0);
    let near_half = vertex_marker_half_extent(&near, viewport, centre, VERTEX_BASE_SCALE, None);
    let far_half = vertex_marker_half_extent(&far, viewport, centre, VERTEX_BASE_SCALE, None);
    assert!(far_half > near_half * 10.0, "a marker 100x further away spans ~100x more world units to keep its pixels ({near_half} → {far_half})");
    let near_pixels = near_half / world_units_per_pixel(&near, viewport, near.position.sub(centre).length());
    assert!((near_pixels - WORLD_VERTEX_DOT_PX * 0.5).abs() < 1e-3, "the plain dot is React's 6 px, got {}", near_pixels * 2.0);
    let mark = vertex_marker_half_extent(&near, viewport, centre, VERTEX_HOVER_SCALE, None);
    assert!(mark > near_half, "a hovered/selected marker is React's larger 11 px mark");
    assert!(far_half > 0.0075, "the old world-unit cross was sub-pixel at this distance");
}
//#endregion 🔘️VertexMarkerPixels

#[test]
fn sync_parses_selection_targets_and_active_object() {
    let selection = r#"{
            "granularity":"vertex",
            "targets":{"mesh":true,"vertex":true,"edge":false,"face":false},
            "activeObjectId":"obj-1"
        }"#;
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(selection), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.selection.selection_targets.vertex);
    assert!(!state.selection.selection_targets.edge);
    assert_eq!(state.selection.active_object_id.as_deref(), Some("obj-1"));
}

/// 🎯️ One world surface fed a `fit` lane, as the generation3d preview publishes it.
fn scene_with_fit(fit_json: Option<&str>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection("{}");
    if let Some(world) = scene.world_3d.as_mut() {
        world.meshes_json = "[]".into();
        world.instances_json = "[]".into();
        world.fit_json = fit_json.map(str::to_string);
    }
    scene
}

/// 🎯️ The generation3d `box-fillet-preview` delivery extent, as `📚️examples/📐️box-fillet-preview/🧫️fixtures/🧩️example/🔣️.json` commits it.
const BOOT_FRAME_FIT_JSON: &str = r#"{"enabled":true,"revision":7,"padding":1.12,"boundsMin":[0.0,0.0,0.0],"boundsMax":[2.0,2.0,2.0]}"#;

#[test]
fn a_world_surface_frames_the_producers_delivered_bounds_on_the_first_delivery_of_a_document() {
    let bounds = Rect { x: 0.0, y: 0.0, w: 1600.0, h: 900.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene_with_fit(Some(r#"{"enabled":true,"revision":7,"padding":1.12}"#)), bounds);
    let seed = state.camera.orbit.clone();
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.camera.orbit.distance, seed.distance, "a fit lane without an extent or geometry has nothing to frame");
    sync_world3d_state(&mut state, &scene_with_fit(Some(BOOT_FRAME_FIT_JSON)), bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    assert_eq!([state.camera.orbit.target.x, state.camera.orbit.target.y, state.camera.orbit.target.z], [1.0, 1.0, 1.0], "the framed camera looks at the delivered box centre");
    let camera = state.camera.orbit.to_camera();
    let half_vertical = (camera.fov_y * 0.5).tan();
    let aspect = bounds.w / bounds.h;
    let forward = [camera.target.x - camera.position.x, camera.target.y - camera.position.y, camera.target.z - camera.position.z];
    let forward_length = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2]).sqrt();
    let unit_forward = [forward[0] / forward_length, forward[1] / forward_length, forward[2] / forward_length];
    let right = [unit_forward[1] * camera.up.z - unit_forward[2] * camera.up.y, unit_forward[2] * camera.up.x - unit_forward[0] * camera.up.z, unit_forward[0] * camera.up.y - unit_forward[1] * camera.up.x];
    let right_length = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
    let unit_right = [right[0] / right_length, right[1] / right_length, right[2] / right_length];
    let unit_up = [unit_right[1] * unit_forward[2] - unit_right[2] * unit_forward[1], unit_right[2] * unit_forward[0] - unit_right[0] * unit_forward[2], unit_right[0] * unit_forward[1] - unit_right[1] * unit_forward[0]];
    for corner in 0..8 {
        let point = [if corner & 1 == 0 { 0.0 } else { 2.0 }, if corner & 2 == 0 { 0.0 } else { 2.0 }, if corner & 4 == 0 { 0.0 } else { 2.0 }];
        let relative = [point[0] - camera.position.x, point[1] - camera.position.y, point[2] - camera.position.z];
        let depth = relative[0] * unit_forward[0] + relative[1] * unit_forward[1] + relative[2] * unit_forward[2];
        assert!(depth > 0.0, "corner {corner} is behind the framed camera");
        let ndc_y = (relative[0] * unit_up[0] + relative[1] * unit_up[1] + relative[2] * unit_up[2]) / (depth * half_vertical);
        let ndc_x = (relative[0] * unit_right[0] + relative[1] * unit_right[1] + relative[2] * unit_right[2]) / (depth * half_vertical * aspect);
        assert!(ndc_x.abs() <= 1.0 && ndc_y.abs() <= 1.0, "corner {corner} projects to ({ndc_x}, {ndc_y}) — outside the viewport");
    }
}

#[test]
fn a_re_delivery_of_the_same_document_never_takes_back_a_camera_the_user_moved() {
    let bounds = Rect { x: 0.0, y: 0.0, w: 1600.0, h: 900.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let scene = scene_with_fit(Some(BOOT_FRAME_FIT_JSON));
    drive_scene_bridge(&mut state, &scene, bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    let framed = state.camera.orbit.clone();
    state.camera.orbit.orbit(40.0, 12.0);
    state.camera.orbit.zoom(-120.0);
    state.camera.camera_user_moved = true;
    let moved = state.camera.orbit.clone();
    state.bridge.scene_bridge_digest = None;
    drive_scene_bridge(&mut state, &scene, bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.camera.orbit.yaw, moved.yaw, "a second delivery of the same document re-framed a camera the user had moved");
    assert_eq!(state.camera.orbit.distance, moved.distance);
    assert_ne!(moved.yaw, framed.yaw, "the gesture must actually have moved the camera for this law to say anything");
    let next_document = BOOT_FRAME_FIT_JSON.replace("\"revision\":7", "\"revision\":8");
    sync_world3d_state(&mut state, &scene_with_fit(Some(&next_document)), bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    assert_eq!(state.camera.orbit.yaw, moved.yaw, "framing a new document keeps the look direction the user chose");
    assert_eq!([state.camera.orbit.target.x, state.camera.orbit.target.y, state.camera.orbit.target.z], [1.0, 1.0, 1.0]);
    assert!((state.camera.orbit.distance - framed.distance).abs() < 1e-3, "the new document is framed again, at the fitting distance");
}

#[test]
fn sync_parses_numeric_component_ids_and_hovered_component() {
    let selection = r#"{
            "granularity":"vertex",
            "componentIds":[1,2],
            "hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":3},
            "showEdges":true
        }"#;
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(selection), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.selection.granularity, "vertex");
    assert_eq!(state.selection.component_ids, vec!["1".to_string(), "2".to_string()]);
    assert_eq!(state.selection.hovered_component_id.as_deref(), Some("3"));
    assert_eq!(state.selection.hovered_component_object_id.as_deref(), Some("obj-1"));
    assert_eq!(state.selection.hovered_component_mode.as_deref(), Some("vertex"));
    assert!(state.selection.show_edges);
}

/// ⚪️ 4 from edges + 24 from 4 vertex crosses
#[test]
fn append_component_vertex_spheres_render_base_vertices() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.selection.selection_targets.vertex = true;
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let instances = append_component_vertex_spheres();
    assert_eq!(instances.len(), 0);

    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &ui_wgpu::wgpu::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert_eq!(lines.len(), 28);
}

/// 🔘️ The viewport the component-overlay laws measure vertex markers against — their size is now in
/// SCREEN PIXELS (`vertex_marker_half_extent`), so every one of them needs a camera and a rect.
const COMPONENT_OVERLAY_VIEWPORT: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

#[test]
fn append_component_overlays_highlights_only_hovered_edge() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "edge".into();
    state.selection.selection_targets.edge = true;
    state.selection.hovered_component_id = Some("5".into());
    state.selection.hovered_component_object_id = Some("obj-1".into());
    state.selection.hovered_component_mode = Some("edge".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &ui_wgpu::wgpu::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 2);
    assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
}

#[test]
fn append_component_overlays_highlights_selected_edge() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "edge".into();
    state.selection.component_ids = vec!["6".into()];
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &ui_wgpu::wgpu::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 2);
    assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
}

#[test]
fn component_mode_does_not_apply_mesh_instance_hover() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.hovered_component_object_id = Some("obj-1".into());
    state.selection.local_hover_id = None;
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
    assert!(!state.scene.draws[0].instances[0].hovered);
}

#[test]
fn runtime_draw_flags_clear_stale_instance_selected_when_selection_is_empty() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.selected_ids.clear();
    state.selection.local_hover_id = None;
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: true, material: Default::default() }], shadow_role: Default::default() });
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
    assert!(!state.scene.draws[0].instances[0].selected, "empty selection must clear a stale instancesJson selected bit");
    assert!(!state.scene.draws[0].instances[0].hovered, "empty hover must clear a stale instancesJson hovered bit");
}

#[test]
fn pick_select_emits_original_vertex_interaction_target() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::ZERO, inner.w, inner.h).expect("vertex projects");
    let action = pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, screen[0], screen[1], inner, false, false).expect("pick action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert_eq!(targets[0], serde_json::json!({"granularity":"vertex","id":"surface-1/obj-1.vertex.1"}));
}
//#endregion 🧲️PickTargetLane
