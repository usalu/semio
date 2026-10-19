/// 🧲️ Laws of marquee previews, selection previews, dynamic retirement, retained draw rebuild and the url mesh bridge.

#[test]
fn marquee_preview_respects_pick_bounds_offset() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 100.0, y: 50.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = inner;
    state.selection.marquee_points = vec![[110.0, 60.0], [490.0, 450.0]];
    update_marquee_preview(&mut state.camera, &mut state.selection, &mut state.scene, inner);
    assert!(!state.selection.marquee_preview_ids.is_empty(), "preview ids: {:?}", state.selection.marquee_preview_ids);
}

#[test]
fn marquee_crossing_includes_partial_overlap_window_does_not() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "mesh".into();
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let view_proj = camera.view_proj(800.0, 800.0);
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for corner in [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]] {
        let screen = ui_scene::math::project_point(view_proj, Vec3::from_array(corner), inner.w, inner.h).expect("screen");
        min_x = min_x.min(screen[0]);
        min_y = min_y.min(screen[1]);
        max_x = max_x.max(screen[0]);
        max_y = max_y.max(screen[1]);
    }
    let center_x = (min_x + max_x) * 0.5;
    let center_y = (min_y + max_y) * 0.5;
    state.selection.marquee_points = vec![[inner.x + min_x, inner.y + min_y], [inner.x + center_x, inner.y + center_y]];
    update_marquee_preview(&mut state.camera, &mut state.selection, &mut state.scene, inner);
    assert!(state.selection.marquee_preview_ids.is_empty(), "window marquee should not select partially enclosed mesh");
    state.selection.marquee_points = vec![[inner.x + center_x, inner.y + min_y], [inner.x + min_x, inner.y + center_y]];
    update_marquee_preview(&mut state.camera, &mut state.selection, &mut state.scene, inner);
    assert!(!state.selection.marquee_preview_ids.is_empty(), "crossing marquee should select partially enclosed mesh");
}

#[test]
fn marquee_component_mode_emits_exact_interaction_targets() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.selection.component_ids = vec!["1".into()];
    state.selection.marquee_points = vec![[10.0, 10.0], [390.0, 390.0]];
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let action = marquee_select_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, true, false).expect("marquee action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert!(!targets.as_array().unwrap().is_empty());
    assert!(targets.as_array().unwrap().iter().all(|target| target["granularity"] == "vertex" && target["id"].as_str().unwrap().starts_with("surface-1/obj-1.vertex.")));
}

#[test]
fn click_release_routes_to_pick_select_instead_of_empty_marquee() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.active_utility = "select".into();
    state.selection.granularity = "mesh".into();
    state.selection.marquee_active = true;
    state.selection.marquee_points = vec![[120.0, 140.0]];
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let action = handle_world3d_pointer_button(&mut state, 120.0, 140.0, false, 0, &PointerModifiers::default()).expect("click should pick");
    assert_eq!(action.action, "interactionSelect");
    assert!(!state.selection.marquee_active);
}

#[test]
fn marquee_face_preview_and_overlay_use_logical_face_ids() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = bounds;
    state.session.pick_bounds = bounds;
    state.selection.marquee_points = vec![[390.0, 10.0], [10.0, 390.0]];
    update_marquee_preview(&mut state.camera, &mut state.selection, &mut state.scene, bounds);
    assert!(state.selection.marquee_preview_ids.iter().any(|id| id == "10" || id == "11"), "preview ids: {:?}", state.selection.marquee_preview_ids);
    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &semio_framework_ui_theme::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(!lines.is_empty(), "face marquee preview should draw triangle edge lines");
}

#[test]
fn fresh_scene_selection_without_hover_clears_previous_component_hover() {
    let selection = r#"{"granularity":"face","componentIds":[10]}"#;
    let scene = scene_with_selection(selection);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.hovered_component_id = Some("11".into());
    state.selection.hovered_component_object_id = Some("obj-1".into());
    state.selection.hovered_component_mode = Some("face".into());
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.selection.hovered_component_id,None);
    assert_eq!(state.selection.hovered_component_object_id,None);
    assert_eq!(state.selection.hovered_component_mode,None);
}

#[test]
fn apply_world_action_preview_updates_component_hover_and_selection() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0;4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();
    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionHover".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": serde_json::json!([{"granularity":"vertex","id":"surface-1/obj-1.vertex.2"}]).to_string(),
                "channel": "pointer",
            })),
        },
    );
    assert_eq!(state.selection.hovered_component_id.as_deref(), Some("2"));
    assert_eq!(state.selection.hovered_component_mode.as_deref(), Some("vertex"));
    assert!(state.selection.local_hover_id.is_none());

    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": serde_json::json!([{"granularity":"vertex","id":"surface-1/obj-1.vertex.4"}]).to_string(),
                "method": "pick",
                "merge": "replace",
            })),
        },
    );
    assert_eq!(state.selection.component_ids, vec!["4".to_string()]);
    assert_eq!(state.selection.granularity, "vertex");
}

#[test]
fn preview_survives_sync_when_scene_json_unchanged() {
    let selection = r#"{"granularity":"vertex","componentIds":[1],"hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":2}}"#;
    let scene = scene_with_selection(selection);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0;4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();
    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": serde_json::json!([{"granularity":"vertex","id":"surface-1/obj-1.vertex.5"}]).to_string(),
                "method": "pick",
                "merge": "replace",
            })),
        },
    );
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.selection.component_ids, vec!["5".to_string()]);
}

#[test]
fn typed_camera_snapshot_matches_current_camera_fixture_without_production_parsing() {
    let mut scene = scene_with_selection("{}");
    let mut page = semio_framework_ui_declaration::ui::World3dSnapshotPage::new(World3dSnapshotPageKind::Camera);
    page.push_item(World3dSnapshotItem { numbers: [4.0, 4.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], number_len: 12, ..Default::default() }).unwrap();
    page.seal().unwrap();
    let descriptor = semio_framework_ui_declaration::ui::World3dSnapshotDescriptor { revision: 5, generation: 7, page_count: 1, item_count: 1, byte_count: 0, draw_count: 0, draw_instance_count: 0, draw_byte_count: 0 };
    let token = semio_framework_ui_declaration::ui::world3d_snapshot_begin(descriptor).unwrap();
    semio_framework_ui_declaration::ui::world3d_snapshot_admit_page(token, page).unwrap();
    let lease = semio_framework_ui_declaration::ui::world3d_snapshot_seal(token).unwrap();
    scene.world_3d.as_mut().unwrap().snapshot = Some(lease);

    let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    let mut typed = World3dState::new("surface".into(), "controller".into());
    sync_world3d_state(&mut typed, &scene, bounds);
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| step_world3d_snapshot(&mut typed, context)) == World3dSnapshotApplyStep::Complete {
            break;
        }
        assert!(turns < 8);
    }
    let oracle: serde_json::Value = serde_json::from_str(&scene.world_3d.as_ref().unwrap().camera_json).expect("camera fixture");
    let vector = |key: &str| Vec3::new(oracle[key][0].as_f64().unwrap() as f32, oracle[key][1].as_f64().unwrap() as f32, oracle[key][2].as_f64().unwrap() as f32);
    let expected = OrbitController::from_camera(&Camera3d {
        position: vector("position"),
        target: vector("target"),
        up: vector("up"),
        fov_y: oracle["fov"].as_f64().unwrap() as f32 * std::f32::consts::PI / 180.0,
        near: ui_scene::math::WORLD_ORBIT_CAMERA_NEAR,
        far: ui_scene::math::WORLD_ORBIT_CAMERA_MIN_FAR,
        projection: ui_scene::math::CameraProjection3d::Perspective,
        zoom: oracle["zoom"].as_f64().unwrap_or(1.0) as f32,
    })
    .to_camera();
    let actual = typed.camera.orbit.to_camera();
    assert_eq!(actual.position, expected.position);
    assert_eq!(actual.target, expected.target);
    assert_eq!(actual.up, expected.up);
    assert_eq!(actual.fov_y, expected.fov_y);
    assert_eq!(typed.bridge.snapshot_lease, Some(lease));

    semio_framework_ui_declaration::ui::world3d_snapshot_begin_close(lease).unwrap();
    assert!(!semio_framework_ui_declaration::ui::world3d_snapshot_close_step(lease).unwrap());
    assert!(semio_framework_ui_declaration::ui::world3d_snapshot_close_step(lease).unwrap());
}

/// 🗑️ A url the renderer's decoder refused is offered ONCE. Both request loops re-derive their
/// missing urls from the scene every frame (`missing_mesh_urls`, `reference_image_urls`), so without
/// this ledger one refused response is re-fetched for the lifetime of the surface — the shape
/// `terrain_tile_misses` already gives DEM tiles and React gives a failed texture url.
#[test]
fn a_refused_asset_url_is_never_offered_again_by_its_surface() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.scene.mesh_source_urls.insert("mesh".into(), "/mesh/🧊️refused.glb".into());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances: Vec::new(), shadow_role: Default::default() }).expect("one scene draw");
    state.scene.references.push(WorldReferenceRecord { url: Some("/plan.jpg".into()), origin: Some([0.0; 3]), width_world: Some(1.0), hidden: Some(false), ..Default::default() });
    let offered = |state: &World3dState| -> (Vec<String>, Vec<String>) {
        let meshes = state.scene.draws.iter().filter(|draw| !state.scene.meshes.contains_key(&draw.mesh_key)).filter_map(|draw| state.scene.mesh_source_urls.get(&draw.mesh_key).cloned()).filter(|url| !state.assets.asset_url_misses.contains(url)).collect();
        let references = state.scene.references.iter().filter_map(|reference| reference.url.clone()).filter(|url| !world_reference_raster_ready(&state.scene, url)).filter(|url| !state.assets.asset_url_misses.contains(url)).collect();
        (meshes, references)
    };
    assert_eq!(offered(&state), (vec!["/mesh/🧊️refused.glb".to_string()], vec!["/plan.jpg".to_string()]), "both loops offer an un-refused url");
    assert!(!world3d_asset_url_missed(&state.assets, "/mesh/🧊️refused.glb"));
    state.assets.pending_glb_urls.insert("/mesh/🧊️refused.glb".into());
    state.assets.pending_image_urls.insert("/plan.jpg".into());

    mark_world3d_asset_miss(&mut state.assets, "/mesh/🧊️refused.glb");
    mark_world3d_asset_miss(&mut state.assets, "/plan.jpg");
    assert!(world3d_asset_url_missed(&state.assets, "/mesh/🧊️refused.glb"));
    assert!(world3d_asset_url_missed(&state.assets, "/plan.jpg"));
    assert!(!world3d_asset_url_missed(&state.assets, "/mesh/🧊️other.glb"));
    assert!(!state.assets.pending_glb_urls.contains("/mesh/🧊️refused.glb"));
    assert!(!state.assets.pending_image_urls.contains("/plan.jpg"));
    assert_eq!(offered(&state), (Vec::new(), Vec::new()), "neither loop offers a refused url again");
}

#[test]
fn dynamic_world_owners_retire_one_nested_owner_per_grant_to_terminal_empty() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0; 3 * 128], vec![0.0; 3 * 128], vec![0; 3 * 64]));
    state.scene.meshes.insert("mesh".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances: (0..32).map(|index| Instance3d { component_source: None, id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }).collect(), shadow_role: Default::default() });
    state.scene.reference_pixels.insert("reference".into(), test_world_scene_raster(64, 64, 0));
    let paint = test_scene_raster_lease(64, 64, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 0);
    state.scene.mesh_paint_textures.insert("paint".into(), WorldSceneRaster { identity: paint.identity(), pending: Mutex::new(Some(paint)) });
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    assert!(!begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    assert!(!with_world_step_context(0, |context| terrain_test_retire_then_dynamic(&mut state,context)));
    let mut turns = 0;
    while !with_world_step_context(1, |context| terrain_test_retire_then_dynamic(&mut state,context)) {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(turns >= 8, "each registry transition and opaque owner transfer consumes a distinct grant");
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
}

fn admit_draw_fixture_instance(state: &mut World3dState, draw: u16, id: &str) -> Result<(), WorldDynamicFault> {
    world3d_draw_rebuild_admit_instance(&mut state.scene, draw, id, Mat4::identity(), [1.0; 4], false, false)
}

fn draw_fixture_bytes(draws: &[(&str, &[&str])]) -> u32 {
    draws.iter().map(|(mesh, instances)| mesh.len() + size_of::<SceneDraw3d>() + instances.iter().map(|id| id.len() + size_of::<Instance3d>()).sum::<usize>()).sum::<usize>() as u32
}

#[test]
fn retained_draw_rebuild_preserves_mixed_group_and_instance_fifo_then_swaps_atomically() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let bytes = draw_fixture_bytes(&[("mesh-a", &["a-0", "a-1"]), ("mesh-b", &["b-0"])]);
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 2, instance_count: 3, byte_count: bytes }).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state.scene, "mesh-a", 1, 2).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state.scene, "mesh-b", 2, 1).unwrap();
    admit_draw_fixture_instance(&mut state, 0, "a-0").unwrap();
    admit_draw_fixture_instance(&mut state, 0, "a-1").unwrap();
    admit_draw_fixture_instance(&mut state, 1, "b-0").unwrap();
    world3d_draw_rebuild_seal(&mut state.scene).unwrap();
    assert!(state.scene.draws.is_empty(), "sealed drafts never publish partially");
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)) == WorldDrawRebuildStep::Pending {
        turns += 1;
        assert!(turns < 16);
    }
    assert_eq!(state.scene.draws.iter().map(|draw| draw.mesh_key.as_str()).collect::<Vec<_>>(), vec!["mesh-a", "mesh-b"]);
    assert_eq!(state.scene.draws[0].instances.iter().map(|instance| instance.id.as_str()).collect::<Vec<_>>(), vec!["a-0", "a-1"]);
    assert_eq!(state.scene.draws[1].instances[0].id, "b-0");
    assert_eq!(state.scene.draw_generation, 1);
}

#[test]
fn retained_draw_rebuild_rejects_byte_and_identifier_plus_one_before_owner_publication() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 0, byte_count: 0 }).unwrap();
    let rejected = world3d_draw_rebuild_admit_draw(&mut state.scene, "mesh", 1, 0).expect_err("byte +1 leaves borrowed mesh owner at producer");
    assert_eq!(rejected, WorldDynamicFault::ByteCapacity);
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state.scene, context)) {}
    let long = "x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY + 1);
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 0, byte_count: WORLD_DYNAMIC_DRAW_BYTE_CAPACITY as u32 }).unwrap();
    let rejected = world3d_draw_rebuild_admit_draw(&mut state.scene, &long, 1, 0).expect_err("ID +1 leaves borrowed mesh owner at producer");
    assert_eq!(rejected, WorldDynamicFault::IdCapacity);
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state.scene, context)) {}
}

#[test]
fn retained_draw_rebuild_rejects_aggregate_instance_plus_one_before_draft_ownership() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let result = begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: (WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY + 1) as u32, byte_count: WORLD_DYNAMIC_DRAW_BYTE_CAPACITY as u32 });
    assert_eq!(result, Err(WorldDynamicFault::InstanceCapacity));
    assert!(world3d_draw_rebuild_terminal_is_empty(&state.scene));
}

#[test]
fn retained_draw_rebuild_stale_and_interrupted_close_never_publish() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let bytes = draw_fixture_bytes(&[("mesh", &["one", "two"])]);
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 2, byte_count: bytes }).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state.scene, "mesh", 1, 2).unwrap();
    admit_draw_fixture_instance(&mut state, 0, "one").unwrap();
    admit_draw_fixture_instance(&mut state, 0, "two").unwrap();
    world3d_draw_rebuild_seal(&mut state.scene).unwrap();
    state.session.interaction_revision = 1;
    assert_eq!(with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)), WorldDrawRebuildStep::Stale);
    assert!(!with_world_step_context(0, |context| close_world3d_draw_rebuild_step(&mut state.scene, context)));
    let mut turns = 0;
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state.scene, context)) {
        turns += 1;
        assert!(turns < 8);
    }
    assert!(state.scene.draws.is_empty());
    assert!(world3d_draw_rebuild_terminal_is_empty(&state.scene));
}

#[test]
fn pick_viewport_uses_render_bounds_not_pick_clip_offset() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let bounds = Rect { x: 0.0, y: 50.0, w: 400.0, h: 400.0 };
    let clip = Rect { x: 0.0, y: 100.0, w: 400.0, h: 400.0 };
    state.session.bounds = bounds;
    state.session.pick_bounds = clip;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_scene::math::project_point(ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, bounds.w, bounds.h), Vec3::ZERO, bounds.w, bounds.h).expect("vertex projects");
    let global_x = bounds.x + screen[0];
    let global_y = bounds.y + screen[1];
    let picked = pick_component_at(&state.session, &state.camera, &state.selection, &state.scene, global_x, global_y, bounds).expect("vertex pick respects render viewport");
    assert_eq!(picked.1, "1");
}

#[test]
fn append_component_face_overlay_lines_include_hovered_face() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.hovered_component_mode = Some("face".into());
    state.selection.hovered_component_id = Some("10".into());
    state.selection.hovered_component_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &semio_framework_ui_theme::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 6, "hovered face should emit triangle edge lines, got {}", lines.len());
}

#[test]
fn pick_component_at_face_mode_uses_ray_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let mesh_ref = state.scene.meshes.get("mesh-1").expect("mesh");
    let tri = world_mesh_triangle(*mesh_ref, 0).expect("triangle");
    let centroid = mesh_vertex(*mesh_ref, tri[0]).expect("first vertex").add(mesh_vertex(*mesh_ref, tri[1]).expect("second vertex")).add(mesh_vertex(*mesh_ref, tri[2]).expect("third vertex")).scale(1.0 / 3.0);
    let screen = ui_scene::math::project_point(ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), centroid, inner.w, inner.h).expect("face centroid projects");
    let picked = pick_component_at(&state.session, &state.camera, &state.selection, &state.scene, screen[0], screen[1], inner).expect("face pick");
    assert_eq!(picked.0, "face");
    assert_eq!(picked.2, "obj-1");
}

#[test]
fn pick_component_at_edge_mode_uses_ray_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "edge".into();
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let edge = state.scene.meshes.get("mesh-1").expect("mesh").edge(0).expect("edge");
    let a = Vec3::new(edge[0][0], edge[0][1], edge[0][2]);
    let b = Vec3::new(edge[1][0], edge[1][1], edge[1][2]);
    let mid = a.add(b).scale(0.5);
    let screen = ui_scene::math::project_point(ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), mid, inner.w, inner.h).expect("edge midpoint projects");
    let picked = pick_component_at(&state.session, &state.camera, &state.selection, &state.scene, screen[0], screen[1], inner).expect("edge pick");
    assert_eq!(picked.0, "edge");
    assert_eq!(picked.2, "obj-1");
}

#[test]
fn mesh_pool_release_clears_at_zero_refcount() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.mesh_pool.acquire("mesh-1".into());
    assert!(state.scene.mesh_pool.release(&"mesh-1".into()));
    assert!(!state.scene.mesh_pool.contains(&"mesh-1".to_string()));
    state.scene.mesh_pool.acquire("mesh-1".into());
    state.scene.mesh_pool.acquire("mesh-1".into());
    assert!(!state.scene.mesh_pool.release(&"mesh-1".into()));
    assert!(state.scene.mesh_pool.contains(&"mesh-1".to_string()));
    assert!(state.scene.mesh_pool.release(&"mesh-1".into()));
    assert!(!state.scene.mesh_pool.contains(&"mesh-1".to_string()));
}

#[test]
fn resolve_physical_mesh_id_picks_closest_lod_url() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.mesh_lod_catalog.insert("tower".into(), vec![WorldMeshLodEntry { lod: 1.0, url: "https://example.com/tower-high.glb".into() }, WorldMeshLodEntry { lod: 100.0, url: "https://example.com/tower-low.glb".into() }]);
    let detailed = resolve_physical_mesh_id(&state.scene, "tower", 2.0);
    let coarse = resolve_physical_mesh_id(&state.scene, "tower", 200.0);
    assert_eq!(detailed, "mesh:tower-high");
    assert_eq!(coarse, "mesh:tower-low");
}

fn publish_retained_draw_fixture(state: &mut World3dState, vector: &serde_json::Value) {
    let mesh = vector["mesh"].as_str().unwrap();
    let instances = vector["instances"].as_array().unwrap();
    let ids: Vec<_> = instances.iter().map(|instance| instance["id"].as_str().unwrap()).collect();
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: state.scene.draw_generation + 1, revision: state.session.interaction_revision, draw_count: 1, instance_count: instances.len() as u32, byte_count: draw_fixture_bytes(&[(mesh, &ids)]) })
        .unwrap();
    world3d_draw_rebuild_admit_draw(&mut state.scene, mesh, vector["version"].as_u64().unwrap(), instances.len() as u16).unwrap();
    for instance in instances {
        let color = std::array::from_fn(|index| instance["color"][index].as_f64().unwrap() as f32);
        world3d_draw_rebuild_admit_instance(&mut state.scene, 0, instance["id"].as_str().unwrap(), Mat4::identity(), color, false, false).unwrap();
    }
    world3d_draw_rebuild_seal(&mut state.scene).unwrap();
    for _ in 0..16 {
        match with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)) {
            WorldDrawRebuildStep::Complete => return,
            WorldDrawRebuildStep::Pending => {}
            other => panic!("draw fixture failed: {other:?}"),
        }
    }
    panic!("draw fixture exceeded its bounded turn count");
}

#[test]
fn retained_draw_rebuild_keeps_url_backed_asset_authority() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🔣️draws.json")).unwrap();
    let vector = &fixture["draws"][1];
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let token = reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::Glb, vector["url"].as_str().unwrap()).unwrap();
    publish_retained_draw_fixture(&mut state, vector);
    assert!(!state.scene.meshes.contains_key(vector["mesh"].as_str().unwrap()));
    assert!(state.scene.placeholder_build.is_none(), "retained draws never replace an admitted asset request with a primitive");
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("exact retained asset request");
    assert_eq!(owner.token(), token);
    assert_eq!(owner.url(), vector["url"].as_str().unwrap());
    owner.begin_close();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}
    assert!(state.assets.asset_io.terminal_is_empty());
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    while !with_world_step_context(1, |context| terrain_test_retire_then_dynamic(&mut state,context)) {}
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
}

/// 🥅️ The puzzle3d mesh wire: two built-in KINDS, then one GLB named by URL. The bridge must keep
/// all three and every instance placed on them.
fn url_mesh_wire_scene() -> UiComponentSceneNode {
    let mut scene = scene_with_selection_and_domain("{}", None);
    let world = scene.world_3d.as_mut().expect("world scene");
    world.meshes_json = r#"[{"id":"box","kind":"box"},{"id":"vortex-marker","kind":"vortex-marker"},{"id":"mesh:🧊️left","url":"/mesh/🧊️left.glb"}]"#.into();
    world.instances_json = r#"[{"id":"obj-box","meshId":"box","position":[0,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]},{"id":"obj-left","meshId":"mesh:🧊️left","position":[2,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into();
    scene
}

const URL_MESH_ID: &str = "mesh:🧊️left";

const URL_MESH_URL: &str = "/mesh/🧊️left.glb";

/// 🧹️ Retires everything a bridged surface holds — snapshot leases live in ONE process-wide
/// fixed pool, so a test that publishes one and walks away starves every later test with
/// `World3dSnapshotFault::Unavailable`.
fn retire_bridged_surface(state: &mut World3dState) {
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    for _ in 0..4_096 {
        if with_world_step_context(1, |context| terrain_test_retire_then_dynamic(state,context)) {
            break;
        }
    }
    assert!(world3d_dynamic_retirement_terminal_is_empty(state), "the surface must reach terminal empty");
}

/// 🥽️🌉️ A mesh the wire declares by URL keeps its draw and its instances, and is left PENDING
/// rather than stood in for.
///
/// 🩸️ `World3dSceneBridgePhase::Parse` used to `retain` only meshes that already carried triangles,
/// then drop every instance whose mesh was gone — so puzzle3d's `{"id":"mesh:…","url":"/mesh/….glb"}`
/// entries AND its `{"id":"box","kind":"box"}` entries vanished with every object placed on them
/// (`state-draws=0 state-instances=0`, `dumpMeshStats` all `indices:0`). Ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3a-asset-decoder-boot-fault.md` §6.
#[test]
fn the_scene_bridge_keeps_url_and_kind_declared_meshes_with_their_instances() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    assert_eq!(state.session.snapshot_fault, None);
    let drawn: Vec<(String, Vec<String>)> = state.scene.draws.iter().map(|draw| (draw.mesh_key.clone(), draw.instances.iter().map(|instance| instance.id.clone()).collect())).collect();
    assert_eq!(drawn, vec![("box".to_string(), vec!["obj-box".to_string()]), (URL_MESH_ID.to_string(), vec!["obj-left".to_string()])], "both the kind mesh and the url mesh keep their draw and their instances");

    assert!(state.scene.meshes.contains_key("box"), "a built-in kind resolves through the placeholder ladder both renderers own");
    assert!(!state.scene.meshes.contains_key(URL_MESH_ID), "a url mesh is owed by the asset pipeline and must NOT be stood in for: a placeholder suppresses its own fetch forever");
    assert_eq!(state.scene.mesh_source_urls.get(URL_MESH_ID).map(String::as_str), Some(URL_MESH_URL), "the bridge binds the mesh id to the url the fetch loop reads");
    retire_bridged_surface(&mut state);
}

/// 🎨️ LAW: geometry provenance selects React's standard-material pair inside one delivered scene:
/// inline geometry remains `0/1`, while the adjacent GLB consumes the environment override.
#[test]
fn one_scene_resolves_distinct_inline_and_glb_standard_materials() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎨️scene-shading/🔣️.json")).unwrap();
    let mut scene = url_mesh_wire_scene();
    scene.world_3d.as_mut().unwrap().environment_json = Some(serde_json::json!({ "material": fixture["mixedMaterialScope"]["environment"] }).to_string());
    let mut state = World3dState::new("surface-material".into(), "controller-material".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    let environment = environment_scene_material(&state.scene.environment);
    assert_eq!(world3d_standard_material_for_mesh(&state.scene, "box", environment), [0.0, 1.0]);
    assert_eq!(world3d_standard_material_for_mesh(&state.scene, URL_MESH_ID, environment), [0.35, 0.58]);
    assert_eq!(world3d_standard_material_for_mesh(&state.scene, URL_MESH_ID, SceneMaterial3d::default()), [0.0, 1.0]);
    retire_bridged_surface(&mut state);
}

/// 🥽️📡️ The url lane end to end inside the surface: the draw offers its url exactly once, the
/// decoded mesh lands under the wire's own id, and the instance draws from then on.
#[test]
fn a_url_declared_mesh_reserves_one_glb_fetch_and_draws_once_its_mesh_lands() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    assert!(state.assets.pending_glb_urls.contains(URL_MESH_URL), "the live predicate that keeps a per-frame offer idempotent");
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("exactly one admitted GLB request");
    assert_eq!(owner.url(), URL_MESH_URL);
    assert_eq!(owner.kind(), WorldAssetRequestKind::Glb);
    assert!(take_next_world3d_asset(&mut state.assets).is_none(), "a re-offered url never doubles its request");

    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.session.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, URL_MESH_URL, mesh).expect("the decoded GLB publishes under the wire's mesh id");
    let resident = *state.scene.meshes.get(URL_MESH_ID).expect("the fetched GLB is resident under the id the wire named");
    let schema = resident.schema().expect("resident mesh schema");
    assert_eq!((schema.vertices, schema.indices), (3, 3), "the resident mesh carries real positions and indices");
    assert!(!state.assets.pending_glb_urls.contains(URL_MESH_URL), "a landed mesh stops being pending");

    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    assert!(take_next_world3d_asset(&mut state.assets).is_none(), "a resident mesh is never re-fetched");
    let draw = state.scene.draws.iter().find(|draw| draw.mesh_key == URL_MESH_ID).expect("the url mesh keeps its draw");
    assert_eq!(draw.instances.len(), 1, "the instance draws now that its mesh is resident");

    owner.begin_close();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}
    retire_bridged_surface(&mut state);
}

/// 🗑️ A refused GLB is a miss, not an endless retry.
#[test]
fn a_refused_mesh_url_is_never_offered_again_by_its_surface() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("admitted GLB request");
    owner.begin_close();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}

    mark_world3d_asset_miss(&mut state.assets, URL_MESH_URL);
    assert!(!state.assets.pending_glb_urls.contains(URL_MESH_URL), "a refusal clears the pending mark it replaces");
    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    assert!(take_next_world3d_asset(&mut state.assets).is_none(), "a refused url is never offered again");
    retire_bridged_surface(&mut state);
}

/// 👻️ A ghost over a mesh the surface is still fetching draws through the shared `box` primitive, not
/// under that mesh's own id — minting a stand-in there would make the id resident and silence
/// `offer_missing_mesh_fetches` for the mesh the ghost is standing in for. Once the GLB lands the
/// ghost switches to the real mesh, exactly like React's `BrushPreviewGhost`.
#[test]
fn a_ghost_never_stands_in_under_the_id_of_a_mesh_the_surface_is_still_fetching() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    assert_eq!(ghost_mesh_id(&state.scene, Some(URL_MESH_URL)), "box", "a pending url ghosts as the shared primitive");
    assert_eq!(ghost_mesh_id(&state.scene, None), "box");
    assert_eq!(ghost_mesh_id(&state.scene, Some("/mesh/🧊️unrelated.glb")), "mesh:🧊️unrelated", "a url the scene never named keeps its own lazy ghost key");

    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.session.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, URL_MESH_URL, mesh).expect("the decoded GLB publishes");
    assert_eq!(ghost_mesh_id(&state.scene, Some(URL_MESH_URL)), URL_MESH_ID, "once the mesh lands the ghost draws the real thing");
    retire_bridged_surface(&mut state);
}

/// 🌉️ Drives the staged bridge to its sealed lease, then the snapshot apply ladder to `Complete`.
pub(crate) fn drive_scene_bridge(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_scene_bridge(state, context)) {
            World3dSceneBridgeStep::Complete => break,
            World3dSceneBridgeStep::Pending => {}
            step => panic!("scene bridge stopped at {step:?} on turn {turn} (fault {:?})", state.session.snapshot_fault),
        }
        assert!(turn < 4_095, "scene bridge did not seal within its turn ceiling");
    }
    sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        assert_ne!(with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)), WorldDrawRebuildStep::Fault, "draw rebuild faulted on turn {turn}");
        match with_world_step_context(64, |context| step_world3d_snapshot(state, context)) {
            World3dSnapshotApplyStep::Complete | World3dSnapshotApplyStep::Idle => break,
            World3dSnapshotApplyStep::Pending => {}
            step => panic!("snapshot apply stopped at {step:?} on turn {turn} (fault {:?})", state.session.snapshot_fault),
        }
        assert!(turn < 4_095, "snapshot apply did not complete within its turn ceiling");
    }
    for turn in 0..4_096 {
        let draw_step = with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context));
        assert_ne!(draw_step, WorldDrawRebuildStep::Fault, "draw retirement faulted on turn {turn}");
        let bridge_step = with_world_step_context(64, |context| step_world3d_scene_bridge(state, context));
        assert_ne!(bridge_step, World3dSceneBridgeStep::Fault, "bridge retirement faulted on turn {turn}");
        if world3d_draw_rebuild_terminal_is_empty(&state.scene) && state.bridge.scene_bridge_retired.is_none() && state.bridge.scene_bridge_close.is_none() && state.bridge.scene_bridge.is_none() {
            break;
        }
        assert!(turn < 4_095, "scene bridge retirement did not complete within its turn ceiling");
    }
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
}

fn terrain_test_grant()->semio_framework_value::retained_clone::RetainedCloneGrant {semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64}}

pub(crate) fn terrain_test_retire_then_dynamic(state:&mut World3dState,context:&mut semio_framework_job::StepContext<'_>)->bool {
    let grant=terrain_test_grant();
    let terrain=step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,grant,context);
    assert!(terrain.ownership.fits(grant));
    match terrain.step {
        WorldTerrainMeshPublicationStep::Idle=>step_world3d_dynamic_retirement(state,context),
        WorldTerrainMeshPublicationStep::Pending|WorldTerrainMeshPublicationStep::Complete=>false,
        WorldTerrainMeshPublicationStep::Fault(fault)=>panic!("typed terrain retirement refusal: {fault:?}"),
    }
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(read_static("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}
