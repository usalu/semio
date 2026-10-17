/// 🌉️ Laws of the scene bridge, camera framing and fit ownership, transparent draws and projection selection.

//#region 🌉️World3dSceneBridge
/// 🌉️ The committed generation3d preview payload this lane drives the bridge with. Produced by the
/// real pipeline (`parse_dsl` → `FlowHost::evaluate` → `tessellate_geometry(0.05)` →
/// `preview_payload_from_eval`) in `s.procedural.generation3d`'s own example-geometry harness, whose
/// `parry3d` oracle independently confirms the same solid's volume and extent — see the fixture's
/// `provenance` block, and the twin assertion in that harness that keeps this file from rotting.
#[allow(non_snake_case)]
fn SCENE_BRIDGE_FIXTURE() -> &'static str {
    read_static("../../../🧪️tests/🧫️fixtures/🌉️scene-bridge/🔣️.json")
}

pub(crate) fn scene_bridge_fixture() -> serde_json::Value {
    serde_json::from_str(SCENE_BRIDGE_FIXTURE()).expect("scene bridge fixture parses")
}

/// 🎬️ Builds the exact `World3dScene` the generation3d preview window publishes for the fixture.
pub(crate) fn scene_from_bridge_fixture(fixture: &serde_json::Value) -> UiComponentSceneNode {
    let mut scene = scene_with_selection_and_domain("{}", Some((fixture["domainId"].as_str().expect("domain"), fixture["domainGranularityId"].as_str().expect("granularity"))));
    let world = scene.world_3d.as_mut().expect("world scene");
    world.meshes_json = fixture["meshesJson"].to_string();
    world.instances_json = fixture["instancesJson"].to_string();
    world.camera_json = fixture["cameraJson"].to_string();
    world.selection_json = fixture["selectionJson"].to_string();
    world.environment_json = Some(fixture["environmentJson"].to_string());
    scene
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

#[allow(non_snake_case)]
fn CAMERA_FRAMING_FIXTURE() -> &'static str {
    read_static("../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🎥️world3d-camera-framing/🔣️.json")
}

fn camera_framing_fixture() -> serde_json::Value {
    serde_json::from_str(CAMERA_FRAMING_FIXTURE()).expect("camera framing fixture parses")
}

fn camera_framing_scene(fixture: &serde_json::Value, camera: &str, geometry: bool, revision: Option<u64>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().expect("world scene");
    world.meshes_json = if geometry { serde_json::Value::Array(vec![fixture["mesh"].clone()]).to_string() } else { "[]".into() };
    world.instances_json = if geometry { fixture["instances"].to_string() } else { "[]".into() };
    let mut delivered=fixture["cameras"][camera].clone();delivered["projectionFrame"]=fixture["framePolicies"]["delivered"].clone();
    world.camera_json = delivered.to_string();
    let mut fit = fixture["fit"].clone();
    if let Some(revision) = revision {
        fit["revision"] = serde_json::Value::from(revision);
    }
    world.fit_json = Some(fit.to_string());
    scene
}

fn drive_camera_fit(state: &mut World3dState) -> usize {
    let mut pending = 0;
    for turn in 0..128 {
        match step_world3d_camera_fit(state) {
            World3dCameraFitStep::Applied => return pending,
            World3dCameraFitStep::Pending | World3dCameraFitStep::Stale => pending += 1,
            World3dCameraFitStep::Idle => panic!("camera fit became idle before applying on turn {turn}"),
        }
    }
    panic!("camera fit did not finish inside its fixed fixture ceiling")
}

fn assert_camera_fit(state: &World3dState, expected: &serde_json::Value) {
    let camera = state.camera.orbit.to_camera();
    let actual_position = [camera.position.x, camera.position.y, camera.position.z];
    let actual_target = [camera.target.x, camera.target.y, camera.target.z];
    for axis in 0..3 {
        assert!((actual_position[axis] - expected["position"][axis].as_f64().expect("position") as f32).abs() < 1e-4, "position axis {axis}: {actual_position:?}");
        assert!((actual_target[axis] - expected["target"][axis].as_f64().expect("target") as f32).abs() < 1e-5, "target axis {axis}: {actual_target:?}");
    }
    assert!((state.camera.orbit.distance - expected["distance"].as_f64().expect("distance") as f32).abs() < 1e-4);
    assert!((state.camera.orbit.zoom - expected["zoom"].as_f64().expect("zoom") as f32).abs() < 1e-5,"original camera zoom {} expected {}",state.camera.orbit.zoom,expected["zoom"]);
}

/// 🎥️ The neutral fixture enters through the actual JSON scene bridge, publishes a real mesh lease,
/// and advances one transformed AABB corner per retained step before matching Three's Box3 oracle.
#[test]
fn live_scene_mesh_bounds_frame_both_camera_families_like_current_react() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    for (camera, expected) in [("perspective", "perspective"), ("orthographicTop", "orthographicTop")] {
        let scene = camera_framing_scene(&fixture, camera, true, None);
        let mut state = World3dState::new(format!("surface-{camera}"), "controller".into());
        drive_scene_bridge(&mut state, &scene, viewport);
        let pending = drive_camera_fit(&mut state);
        assert!(pending >= fixture["instances"].as_array().unwrap().len() * 8, "each transformed AABB corner consumes its own retained step");
        assert_camera_fit(&state, &fixture["expect"][expected]);
    }
}

/// ⏰️ Two resident panes that first measured an empty generation sleep without polling, then
/// both re-arm when their actual draw generations seal and keep the renderer awake until every
/// transformed corner has been consumed.
#[test]
fn resident_generation_wakes_every_visible_camera_fit_until_both_complete() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let mut panes = [
        (World3dState::new("surface-top".into(), "controller".into()), "orthographicTop", "orthographicTopAfterRawContent", "delayedContent"),
        (World3dState::new("surface-perspective".into(), "controller".into()), "perspective", "perspective", "delivered"),
        (World3dState::new("surface-perspective-content".into(), "controller".into()), "perspective", "perspectiveThreePoint", "delayedContent"),
    ];
    for (state, camera, _, policy) in &mut panes {
        let mut scene=camera_framing_scene(&fixture,camera,false,None);let world=scene.world_3d.as_mut().unwrap();let mut delivered:serde_json::Value=serde_json::from_str(&world.camera_json).unwrap();delivered["projectionFrame"]=fixture["framePolicies"][*policy].clone();world.camera_json=delivered.to_string();
        drive_scene_bridge(state,&scene,viewport);
        assert_eq!(step_world3d_camera_fit(state), World3dCameraFitStep::Idle);
        assert!(!world3d_cursor_work_pending(state), "an exactly witnessed empty generation sleeps");
    }
    for (state, camera, _, policy) in &mut panes {
        let mut scene=camera_framing_scene(&fixture,camera,true,None);let world=scene.world_3d.as_mut().unwrap();let mut delivered:serde_json::Value=serde_json::from_str(&world.camera_json).unwrap();delivered["projectionFrame"]=fixture["framePolicies"][*policy].clone();world.camera_json=delivered.to_string();
        drive_scene_bridge(state,&scene,viewport);
        assert!(world3d_cursor_work_pending(state), "a sealed draw generation re-arms its own fit lane");
    }

    let ceiling = fixture["instances"].as_array().unwrap().len() * 8 + 2;
    let mut applied = [false; 3];
    for turn in 0..ceiling {
        for (index, (state, _, _, _)) in panes.iter_mut().enumerate() {
            if !applied[index] {
                applied[index] = step_world3d_camera_fit(state) == World3dCameraFitStep::Applied;
            }
        }
        if applied.into_iter().all(|complete| complete) {
            break;
        }
        assert!(panes.iter().any(|(state, _, _, _)| world3d_cursor_work_pending(state)), "turn {turn}: unfinished retained camera work owns the next frame");
    }
    assert!(applied.into_iter().all(|complete| complete), "both visible panes finish inside one exact corner walk");
    for (state, _, expected, _) in &panes {
        assert_camera_fit(state, &fixture["expect"][expected]);
        assert!(!world3d_cursor_work_pending(state), "an applied fit releases its wake");
    }
    for (state,_,_,_) in &mut panes {retire_bridged_surface(state);}
}

/// 📐️ A dock template lands after the producer camera lease and before auto-fit. Its look
/// direction owns the fitted eye, while the measured geometry owns target and distance.
#[test]
fn initial_projection_template_seeds_the_actual_delivered_camera_before_fit() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let mut state = World3dState::new("surface-template".into(), "controller".into());
    let spec = ui_wgpu::wgpu::default_projection_spec();
    assert!(!apply_world3d_initial_projection_seed(&mut state.session, &mut state.camera, &mut state.scene, &mut state.bridge, spec), "a template never races ahead of the delivered camera lease");

    drive_scene_bridge(&mut state, &camera_framing_scene(&fixture, "perspective", true, None), viewport);
    assert!(apply_world3d_initial_projection_seed(&mut state.session, &mut state.camera, &mut state.scene, &mut state.bridge, spec));
    assert!(!apply_world3d_initial_projection_seed(&mut state.session, &mut state.camera, &mut state.scene, &mut state.bridge, spec), "the same pane consumes its initial template once");
    drive_camera_fit(&mut state);
    assert_camera_fit(&state, &fixture["expect"]["perspectiveThreePoint"]);
}

/// 📦️ The live Shell sees the applied snapshot lease one transaction before its sealed draw
/// rebuild publishes. Its initial projection seed is camera-only: it must preserve that rebuild,
/// because the url-backed draw is also the sole ledger that admits the document's GLB request.
#[test]
fn initial_projection_seed_preserves_the_delivered_url_draw_asset_request_and_loaded_fit() {
    let mut scene = url_mesh_wire_scene();
    let world = scene.world_3d.as_mut().expect("world scene");
    world.instances_json = r#"[{"id":"obj-left","meshId":"mesh:🧊️left","position":[2,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into();
    world.fit_json = Some(r#"{"enabled":true,"revision":7,"padding":1.12}"#.into());
    let bounds = Rect::new(0.0, 0.0, 800.0, 600.0);
    let mut state = World3dState::new("surface-url-template".into(), "controller".into());

    sync_world3d_state(&mut state, &scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_scene_bridge(&mut state, context)) {
            World3dSceneBridgeStep::Complete => break,
            World3dSceneBridgeStep::Pending => {}
            step => panic!("url bridge stopped at {step:?} on turn {turn}"),
        }
        assert!(turn < 4_095, "url bridge did not seal inside its fixture ceiling");
    }
    assert_eq!(state.scene.instance_positions.get("obj-left"), Some(&[2.0, 0.0, 0.0]), "the bridge retains React's raw pre-asset position for projection framing");
    sync_world3d_state(&mut state, &scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_snapshot(&mut state, context)) {
            World3dSnapshotApplyStep::Complete => break,
            World3dSnapshotApplyStep::Pending => {}
            step => panic!("url snapshot stopped at {step:?} on turn {turn}"),
        }
        assert!(turn < 4_095, "url snapshot did not apply inside its fixture ceiling");
    }
    assert!(state.scene.draw_rebuild.is_some(), "the applied delivery retains its sealed rebuild for the next transaction");
    let top_spec = semio_framework_ui_viewport::Viewport3dProjectionSpec { mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {}, orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top } };
    assert!(apply_world3d_initial_projection_seed(&mut state.session, &mut state.camera, &mut state.scene, &mut state.bridge, top_spec));
    sync_world3d_state(&mut state, &scene, bounds);
    assert!(!state.camera.projection_frame_owed, "the exact Top document sync consumes its content-frame request");
    assert!((state.camera.orbit.target.x - 2.0).abs() < 1e-5, "the pre-asset Top frame targets the raw instance position");
    assert!(state.scene.draw_rebuild.is_some(), "the following document sync keeps the camera-only revision on the same delivery");
    for turn in 0..64 {
        match with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)) {
            WorldDrawRebuildStep::Complete => break,
            WorldDrawRebuildStep::Pending => {}
            step => panic!("camera-only seed invalidated the delivered rebuild at {step:?} on turn {turn}"),
        }
        assert!(turn < 63, "url draw rebuild did not publish inside its exact fixture ceiling");
    }
    let draw = state.scene.draws.iter().find(|draw| draw.mesh_key == URL_MESH_ID).expect("url draw survives the template seed");
    assert_eq!(draw.instances.iter().map(|instance| instance.id.as_str()).collect::<Vec<_>>(), ["obj-left"]);

    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("the surviving draw admits one GLB request");
    assert_eq!((owner.kind(), owner.url()), (WorldAssetRequestKind::Glb, URL_MESH_URL));
    assert!(take_next_world3d_asset(&mut state.assets).is_none(), "the delivered url is admitted exactly once");
    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.session.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, URL_MESH_URL, mesh).expect("the requested GLB publishes into the delivered draw");
    drive_camera_fit(&mut state);
    assert!(state.camera.fit_applied_key.is_some(), "the loaded mesh generation completes the retained auto-fit");
    assert!(state.camera.orbit.target.x > 2.0 && state.camera.orbit.target.y > 0.0, "the fit measures the translated loaded mesh rather than a placeholder origin");

    owner.begin_close();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}
    retire_bridged_surface(&mut state);
}

/// 🎥️ Empty geometry leaves the key unconsumed; the same revision applies when its real mesh arrives.
#[test]
fn delayed_geometry_retires_stale_measurement_without_fitting_a_placeholder_origin() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let empty = camera_framing_scene(&fixture, "perspective", false, None);
    let mut state = World3dState::new("surface-delayed".into(), "controller".into());
    drive_scene_bridge(&mut state, &empty, viewport);
    let seed = state.camera.orbit.clone();
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.camera.fit_applied_key, None, "an empty group does not consume React's fit key");
    assert_eq!(state.camera.orbit.target, seed.target, "no placeholder origin is framed");

    let loaded = camera_framing_scene(&fixture, "perspective", true, None);
    drive_scene_bridge(&mut state, &loaded, viewport);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Pending);
    state.scene.geometry_generation = state.scene.geometry_generation.wrapping_add(1);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Stale, "a residency change retires the partial measurement");
    drive_camera_fit(&mut state);
    assert_camera_fit(&state, &fixture["expect"]["perspective"]);
}

/// 🧭️ Genuine navigation consumes an owed content frame before late geometry arrives.
#[test]
fn world_camera_content_frame_preserves_original_user_orbit() {
    let fixture=camera_framing_fixture();let viewport=Rect::new(0.0,0.0,fixture["viewport"][0].as_f64().unwrap() as f32,fixture["viewport"][1].as_f64().unwrap() as f32);
    let mut scene=camera_framing_scene(&fixture,"perspective",false,None);let world=scene.world_3d.as_mut().unwrap();let mut delivered:serde_json::Value=serde_json::from_str(&world.camera_json).unwrap();delivered["projectionFrame"]=fixture["framePolicies"]["delayedContent"].clone();world.camera_json=delivered.to_string();
    let mut state=World3dState::new("surface-user-frame".into(),"controller".into());drive_scene_bridge(&mut state,&scene,viewport);
    assert_eq!(step_world3d_camera_fit(&mut state),World3dCameraFitStep::Idle);assert!(state.camera.projection_frame_owed);
    state.camera.orbit.orbit(0.4,-0.2);state.camera.camera_user_moved=true;let moved=state.camera.orbit.clone();
    let mut loaded=camera_framing_scene(&fixture,"perspective",true,None);loaded.world_3d.as_mut().unwrap().camera_json=scene.world_3d.as_ref().unwrap().camera_json.clone();drive_scene_bridge(&mut state,&loaded,viewport);
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert_eq!(state.camera.orbit.yaw,moved.yaw);assert_eq!(state.camera.orbit.pitch,moved.pitch);assert_eq!(state.camera.orbit.target,moved.target);
    assert!(!state.camera.projection_frame_owed);assert_eq!(step_world3d_camera_fit(&mut state),World3dCameraFitStep::Idle);
}

/// 🎥️ User orbit owns one revision/seed; a new document revision or external seed owns a new frame.
#[test]
fn camera_fit_ownership_preserves_user_orbit_until_revision_or_seed_changes() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let scene = camera_framing_scene(&fixture, "perspective", true, None);
    let mut state = World3dState::new("surface-owner".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, viewport);
    drive_camera_fit(&mut state);
    state.camera.orbit.orbit(0.4, -0.2);
    state.camera.camera_user_moved = true;
    let moved = state.camera.orbit.clone();
    sync_world3d_state(&mut state, &scene, viewport);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.camera.orbit.yaw, moved.yaw, "same owner never takes back a user orbit");

    let next_revision = camera_framing_scene(&fixture, "perspective", true, Some(fixture["fit"]["revision"].as_u64().unwrap() + 1));
    sync_world3d_state(&mut state, &next_revision, viewport);
    assert!(drive_camera_fit(&mut state) >= fixture["instances"].as_array().unwrap().len() * 8, "a new revision remeasures resident geometry through the bounded cursor");
    assert_eq!(state.camera.orbit.yaw, moved.yaw, "reframing preserves the user's look direction");
    state.camera.orbit.orbit(-0.3, 0.1);
    state.camera.camera_user_moved = true;

    let mut next_seed_fixture = fixture.clone();
    next_seed_fixture["cameras"]["perspective"]["position"][0] = serde_json::Value::from(12.0);
    let next_seed = camera_framing_scene(&next_seed_fixture, "perspective", true, Some(fixture["fit"]["revision"].as_u64().unwrap() + 1));
    drive_scene_bridge(&mut state, &next_seed, viewport);
    drive_camera_fit(&mut state);
    assert!(state.camera.fit_applied_key.is_some(), "an external camera seed changes the fit key and owns a fresh frame");
}

#[test]
fn scene_bridge_renders_the_generation3d_preview_payload_into_a_snapshot() {
    let fixture = scene_bridge_fixture();
    let scene = scene_from_bridge_fixture(&fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);

    let expect = &fixture["expect"];
    assert_eq!(state.session.snapshot_fault, None, "the bridge must publish without faulting");
    assert_eq!(state.scene.draws.iter().count(), expect["draws"].as_u64().expect("draws") as usize, "original surface and wire meshes each retain their draw");
    let draw = state.scene.draws.iter().find(|draw| Some(draw.mesh_key.as_str()) == expect["drawMeshKey"].as_str()).expect("published solid draw");
    assert_eq!(draw.mesh_key, expect["drawMeshKey"].as_str().expect("mesh key"));
    let instance_ids: Vec<&str> = draw.instances.iter().map(|instance| instance.id.as_str()).collect();
    let expected_instance_ids: Vec<&str> = expect["instanceIds"].as_array().expect("instance ids").iter().map(|id| id.as_str().expect("instance id")).collect();
    assert_eq!(instance_ids, expected_instance_ids, "instance ids stay channel-qualified across the wire");

    let mesh = *state.scene.meshes.get(&draw.mesh_key).expect("published mesh geometry");
    let schema = mesh.schema().expect("mesh schema");
    assert_eq!(schema.indices / 3, expect["triangles"].as_u64().expect("triangles") as u32, "the bridged mesh carries the tessellated triangle count, not a placeholder box");
    assert_eq!(schema.vertices, expect["vertices"].as_u64().expect("vertices") as u32);

    for id in expect["wireOnlyMeshesRetained"].as_array().expect("wire meshes") {
        let key = id.as_str().expect("mesh id");
        let wire = state.scene.meshes.get(key).expect("original wire mesh is retained").schema().unwrap();
        assert_eq!(wire.indices, 0, "wire geometry has no synthetic triangles");
        assert!(wire.edges > 0);
        assert!(state.scene.draws.iter().any(|draw|draw.mesh_key == key));
    }
}

/// 🎯️ The delivery that arrives WITH a first framing must still publish its draws.
///
/// ⚖️ The fit now waits until the delivery and its draw rebuild are complete. It must remain ordered
/// there: framing during `sync_world3d_state` advanced `interaction_revision` while the delivery's
/// mesh items still carried the earlier revision, so the rebuild answered `WorldDrawRebuildStep::Stale`
/// and nothing ever began another — the surface
/// keeps its meshes and zero draws for the life of the document. Measured on 6118 as
/// `state-meshes=3 apply=false rebuild=false state-draws=0 draws=0` on every generation3d example in
/// both roles, with `world3d-editor` falling 75/115 → 62/115 the hour the fit lane landed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-regressions-sweep-2026-09-15.md`).
#[test]
fn a_first_framing_never_strands_the_deliverys_own_draw_rebuild() {
    let fixture = scene_bridge_fixture();
    let mut scene = scene_from_bridge_fixture(&fixture);
    scene.world_3d.as_mut().expect("world scene").fit_json = Some(BOOT_FRAME_FIT_JSON.to_string());
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 1600.0, h: 900.0 });
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    assert_eq!(state.session.snapshot_fault, None, "a framed delivery must apply without faulting");
    assert!(state.camera.fit_applied_key.is_some(), "the delivery's own fit lane framed this document once");
    assert_eq!(state.scene.draws.iter().count(), fixture["expect"]["draws"].as_u64().expect("draws") as usize, "the framing must not strand the delivery's draw rebuild");
    assert!(state.session.interaction_revision >= state.bridge.snapshot_lease.expect("applied lease").revision, "fit advances only after the applied delivery owns the surface revision");
}

/// 🟩️ A `provisional: true` instance record — the producer's stamp from `ArtifactView::tool_run()` (tool run
/// contract §4.1 layer 1) — fills `provisional_instance_ids`, so the draw pass paints it with the provisional
/// token; records without the flag stay ordinary instances.
#[test]
fn scene_bridge_collects_the_provisional_instance_flag_the_producer_stamps() {
    let mut fixture = scene_bridge_fixture();
    let bridged: Vec<String> = fixture["expect"]["instanceIds"].as_array().expect("instance ids").iter().map(|id| id.as_str().expect("instance id").to_string()).collect();
    let provisional = bridged.first().expect("a bridged instance").clone();
    for instance in fixture["instancesJson"].as_array_mut().expect("instances") {
        if instance["id"].as_str() == Some(provisional.as_str()) {
            instance["provisional"] = serde_json::Value::Bool(true);
        }
    }
    let scene = scene_from_bridge_fixture(&fixture);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });
    assert_eq!(state.selection.provisional_instance_ids, HashSet::from([provisional]), "exactly the stamped record is provisional");
}

/// 🧭️ Only a world surface bound to a window AND holding an applied trace lane owes that window a cursor echo.
#[test]
fn world_surfaces_echo_their_trace_cursor_under_their_window_instance() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/⏯️tool-run/🧪️tests/🧫️fixtures/📼️trace-pages.json")).expect("trace pages fixture");
    let bytes: Vec<u8> = fixture["deltas"][0]["hex"].as_str().expect("delta hex").as_bytes().chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex"), 16).expect("hex")).collect();
    let delta = semio_framework_tool_run::ToolRunTraceDelta::decode(&bytes).expect("fixture delta decodes");
    let lane = base64_codec::base64_url_encode(&bytes);
    let mut bound = World3dState::new("surface-bound".into(), "controller".into());
    bound.trace.tool_run_trace_window_id = Some("world-left".into());
    bound.trace.tool_run_trace.apply_lane(Some(&lane)).expect("lane applies");
    let mut unbound = World3dState::new("surface-unbound".into(), "controller".into());
    unbound.trace.tool_run_trace.apply_lane(Some(&lane)).expect("lane applies");
    let mut idle = World3dState::new("surface-idle".into(), "controller".into());
    idle.trace.tool_run_trace_window_id = Some("world-right".into());
    let echoed = world3d_tool_run_trace_cursors([&bound, &unbound, &idle]);
    let expected = semio_framework_tool_run::ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next };
    assert_eq!(echoed, std::collections::HashMap::from([("world-left".to_string(), expected)]));
}

#[test]
fn scene_bridge_honours_selection_hover_camera_and_sun() {
    let fixture = scene_bridge_fixture();
    let scene = scene_from_bridge_fixture(&fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);

    let expect = &fixture["expect"];
    assert_eq!(state.selection.selected_ids, expect["selectedIds"].as_array().expect("selected").iter().map(|id| id.as_str().expect("id").to_string()).collect::<Vec<_>>());
    assert_eq!(state.selection.local_hover_id.as_deref(), expect["hoveredId"].as_str());
    let draw = state.scene.draws.iter().find(|draw| Some(draw.mesh_key.as_str()) == expect["drawMeshKey"].as_str()).expect("published solid draw");
    let instance = draw.instances.first().expect("published instance");
    assert!(instance.selected, "the scene's own selection document must paint the instance selected");

    let camera = state.camera.orbit.to_camera();
    let expected_camera = expect["cameraPosition"].as_array().expect("camera");
    for axis in 0..3 {
        assert!((f64::from(camera.position.to_array()[axis]) - expected_camera[axis].as_f64().expect("axis")).abs() < 1.0e-4, "camera axis {axis}: {:?}", camera.position);
    }

    let sun = environment_light_dir(&state.scene.environment);
    let expected_sun = expect["sunDirection"].as_array().expect("sun");
    for axis in 0..3 {
        assert!((f64::from(sun[axis]) - expected_sun[axis].as_f64().expect("axis")).abs() < 1.0e-5, "sun axis {axis}: {sun:?}");
    }
}

/// 🎯️ Aim through the fixture camera's own target, which is the centre of the prism's base face
/// — the one point guaranteed both inside the solid and inside the 45° frustum.
#[test]
fn scene_bridge_binds_the_apps_interaction_domain_for_world_picking() {
    let fixture = scene_bridge_fixture();
    let scene = scene_from_bridge_fixture(&fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    state.session.pick_bounds = bounds;

    assert_eq!(resolved_domain_id(&state.session), fixture["domainId"].as_str().expect("domain"));
    assert_eq!(resolved_domain_granularity_id(&state.session), fixture["domainGranularityId"].as_str().expect("granularity"));
    let instance_id = state.scene.draws.iter().find(|draw| Some(draw.mesh_key.as_str()) == fixture["expect"]["drawMeshKey"].as_str()).expect("published solid draw").instances.first().expect("published instance").id.clone();
    assert_eq!(resolved_item_id(&state.session, &instance_id), instance_id, "a bound app domain addresses the bare channel-qualified id — never `surfaceId/id`");
    let interaction_id = state.interaction.instance_interaction_ids.get(&instance_id).expect("the scene bridge retains the instance's topology target").clone();
    assert_eq!(interaction_id, "extrude@solid");

    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, bounds.w, bounds.h), Vec3::ZERO, bounds.w, bounds.h).expect("camera target projects");
    let hit = pick_instance_at(&state.session, &state.camera, &state.scene, screen[0], screen[1], bounds);
    assert_eq!(hit.as_deref(), Some(instance_id.as_str()), "the bridged geometry is what a world pick actually hits");
    let select = pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, screen[0], screen[1], bounds, false, false).expect("pick emits an action");
    assert_eq!(select.action, "interactionSelect");
    let args = select.args.expect("pick args");
    assert_eq!(args["domainId"].as_str(), fixture["domainId"].as_str());
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert_eq!(targets[0]["granularity"].as_str(), fixture["domainGranularityId"].as_str());
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert_eq!(targets[0]["id"].as_str(), Some(interaction_id.as_str()));

    state.selection.local_hover_id = None;
    let hover = pick_hover_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, screen[0], screen[1], bounds).expect("hover emits an action");
    assert_eq!(hover.action, "interactionHover");
    let args = hover.args.expect("hover args");
    assert_eq!(args["domainId"].as_str(), fixture["domainId"].as_str());
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert_eq!(targets[0]["id"].as_str(), Some(interaction_id.as_str()));
}
//#endregion 🌉️World3dSceneBridge

/// 🎨️ LAW: decoded paint pixels become resident only for the exact mesh generation shape that can
/// consume them; codec work remains outside the frame-owned World state.
#[test]
fn decoded_mesh_paint_publication_requires_rgba_and_matching_uvs() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut painted = triangle_mesh_oracle();
    painted.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
    state.scene.meshes.insert("painted".into(), publish_oracle_mesh(painted)).expect("painted mesh admitted");
    state.scene.mesh_versions.insert("painted".into(), 1).expect("painted revision admitted");
    let paint = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 96);
    assert!(apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "painted", DecodedMeshPaintImage { raster: paint }));
    let resident = state.scene.mesh_paint_textures.get("painted").expect("decoded paint pixels became resident");
    assert_eq!((resident.identity.descriptor().width, resident.identity.descriptor().height), (1, 1));

    state.scene.meshes.insert("plain".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("plain mesh admitted");
    state.scene.mesh_versions.insert("plain".into(), 1).expect("plain revision admitted");
    let plain = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "plain", DecodedMeshPaintImage { raster: plain }), "a mesh without one UV per vertex cannot sample a paint map");
    let stale = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 2, uv_revision: 2, uv_count: 3 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "painted", DecodedMeshPaintImage { raster: stale }), "paint from another mesh generation cannot cross the resident topology");
    let wrong_uvs = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 2 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "painted", DecodedMeshPaintImage { raster: wrong_uvs }), "paint UV cardinality is exact");
    let wrong_profile = test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "painted", DecodedMeshPaintImage { raster: wrong_profile }), "a reference raster cannot cross the mesh-paint profile seal");
}

/// 🫧 LAW: retained transparent Standard objects use Three's transformed bounding-sphere-centre
/// projected depth and preserve producer insertion order for exact ties.
#[test]
fn transparent_standard_draws_match_the_recorded_three_projected_order() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎨️scene-shading/🔣️.json")).unwrap();
    let camera_json = &fixture["scene"]["camera"];
    let array3 = |value: &serde_json::Value| {
        let values = value.as_array().expect("three scalars");
        [values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32]
    };
    let camera = Camera3d {
        position: Vec3::new(array3(&camera_json["position"])[0], array3(&camera_json["position"])[1], array3(&camera_json["position"])[2]),
        target: Vec3::new(array3(&camera_json["target"])[0], array3(&camera_json["target"])[1], array3(&camera_json["target"])[2]),
        up: Vec3::new(array3(&camera_json["up"])[0], array3(&camera_json["up"])[1], array3(&camera_json["up"])[2]),
        fov_y: (camera_json["fov"].as_f64().unwrap() as f32).to_radians(),
        near: 0.01,
        far: 100.0,
        projection: CameraProjection3d::Perspective,
        zoom: 1.0,
    };
    let view_proj = camera.view_proj(64.0, 64.0);

    for case in fixture["depthOrderCases"].as_array().unwrap() {
        let mut state = World3dState::new(case["id"].as_str().unwrap().into(), "controller".into());
        let mut draws = Vec::new();
        for layer in case["layers"].as_array().unwrap() {
            let id = layer["id"].as_str().unwrap();
            let center = array3(&layer["localCenter"]);
            let position = array3(&layer["position"]);
            let mesh = mesh_oracle_from_buffers(
                vec![center[0] - 0.5, center[1] - 0.5, center[2], center[0] + 0.5, center[1] - 0.5, center[2], center[0], center[1] + 0.5, center[2]],
                vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
                vec![0, 1, 2],
            );
            state.scene.meshes.insert(id.into(), publish_oracle_mesh(mesh)).expect("sort mesh admitted");
            draws.push(SceneMaterialDraw3d {
                mesh_key: id.into(),
                mesh_version: 0,
                first_index: 0,
                index_count: u32::MAX,
                instances: vec![Instance3d { component_source: None,
                    id: id.into(),
                    model: Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0; 3]),
                    color: [1.0, 1.0, 1.0, layer["opacity"].as_f64().unwrap() as f32],
                    selected: false,
                    hovered: false,
                    material: Default::default(),
                }],
                material: SceneMaterialKind3d::Standard,
                translucent: true,
            });
        }
        sort_world3d_translucent_material_draws(&state.scene.meshes, view_proj, &mut draws);
        let actual: Vec<&str> = draws.iter().flat_map(|draw| draw.instances.iter().map(|instance| instance.id.as_str())).collect();
        let expected: Vec<&str> = case["expectedDrawOrder"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
        assert_eq!(actual, expected, "{} follows the actual Three order", case["id"].as_str().unwrap());
        retire_bridged_surface(&mut state);
    }
}

/// 🔘️ LAW: every mesh key the gumball draws is ENSURED on the GPU in the same build, and the plane it
/// draws is pinned in the mesh pool.
///
/// The defect: `append_gumball_geometry` pushed three translucent `SceneDraw3d`s keyed
/// `gumball-plane` and never called `gpu.ensure_mesh` for it — the one thing every other world draw
/// site does. On the GPU a missing mesh is a FATAL `present_step` fault ("prepared frame submit step:
/// prepared world mesh was missing"), not a skipped draw, so the first frame after ANY selection
/// killed the whole frame loop. It sat unreachable while the wgpu host dropped the reserved tool job
/// that applies a selection; the instant that job ran, the app died on the first click
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-spawn-job-effect-2026-09-13.md`).
#[test]
fn gumball_draws_ensure_the_plane_mesh_they_reference() {
    let mut lines = Vec::new();
    let mut translucent = Vec::new();
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.active_utility = "select".into();
    state.scene.meshes.insert(GUMBALL_PLANE_MESH.into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("plane lease admitted");
    state.scene.mesh_versions.insert(GUMBALL_PLANE_MESH.into(), 7).expect("plane version admitted");
    state.scene.meshes.insert("mesh-1".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("body lease admitted");
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: false, material: Default::default() }], shadow_role: Default::default() }).expect("body draw admitted");
    state.selection.selected_ids = vec!["obj-1".into()];
    append_gumball_geometry(&mut lines, &mut translucent, &mut gpu, &state.selection, &state.gumball, &state.scene, &Camera3d::default(), &state.scene.meshes, &state.scene.mesh_versions);

    assert_eq!(translucent.len(), 3, "the gumball draws one quad per axis plane");
    let requested: Vec<(String, u64)> = gpu.mesh_requests[..gpu.mesh_request_len].iter().flatten().cloned().collect();
    for draw in &translucent {
        assert!(requested.iter().any(|(key, version)| key == &draw.mesh_key && *version == draw.mesh_version), "gumball draw {}@{} was never ensured on the GPU; requested={requested:?}", draw.mesh_key, draw.mesh_version);
    }
    assert!(requested.iter().all(|(key, _)| key == GUMBALL_PLANE_MESH), "the gumball ensures nothing but its own plane: {requested:?}");
}

/// 🎥️ LAW: the diagnostics row reports the LIVE orbit — the pose the pane is looking through this
/// frame, its projection family and its framing orientation — not the wire camera the guest
/// published.
///
/// 🩸️ `dumpMeshStats` filled `camera` from `World3dScene.camera_json` and nothing else, so every
/// local camera move was invisible to a probe: a `Projection` pane switch to `Orthographic` reported
/// the delivered perspective pose, unchanged, and read as a dead control
/// (`📓️w8b-orthographic-camera-and-3d-parity.md` §7.5).
#[test]
fn the_live_camera_row_reports_the_orbit_and_not_the_wire() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    let before = world3d_live_camera_json(&state.session, &state.camera, &state.selection, &state.scene);
    assert_eq!(before["projection"]["mode"]["kind"], "threePoint", "🎥️ an untouched pane opens the canonical three-point perspective mode");
    assert_eq!(before["userMoved"], serde_json::Value::Bool(false));

    let top_spec = semio_framework_ui_viewport::Viewport3dProjectionSpec { mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {}, orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top } };
    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, top_spec));
    let after = world3d_live_camera_json(&state.session, &state.camera, &state.selection, &state.scene);
    assert_eq!(after["projection"]["mode"]["kind"], "orthographic", "🎥️ the switch is visible in the row the probe reads");
    assert_eq!(after["projection"]["orientation"]["type"], "cardinal", "🎥️ and so is the plane its framing measures in");
    assert!(after["zoom"].as_f64().unwrap_or_default() > 1.0, "🎥️ a parallel pane reports its real frustum scale: {}", after["zoom"]);

    state.camera.orbit.target = Vec3::new(3.5, 0.0, 0.005);
    let moved = world3d_live_camera_json(&state.session, &state.camera, &state.selection, &state.scene);
    let target: Vec<f64> = moved["target"].as_array().expect("the row carries a target").iter().filter_map(serde_json::Value::as_f64).collect();
    assert!((target[0] - 3.5).abs() < 1e-6 && target[1].abs() < 1e-6 && (target[2] - 0.005).abs() < 1e-6, "🎥️ the row follows the orbit, not the wire: {target:?}");
}

/// 📐️ LAW: local projection selections acquire the spec without rearming content framing. React's
/// viewport owner remains mounted and `WorldProjectionSnapDriver` retains the accepted pose.
///
/// 📐️ And the wire never takes the selection back. The settle a press queues publishes
/// `setCamera`, whose payload carries no `projection` member at all, so the guest's echo always
/// arrives in the DELIVERED family — applying it whole is how the switch undid itself in one
/// round trip.
#[test]
fn a_local_projection_selection_retains_viewport_ownership() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    state.camera.camera_user_moved = true;
    state.camera.projection_frame_key = Some(7);
    state.camera.projection_frame_zoom = Some(3.0);

    let top = semio_framework_ui_viewport::Viewport3dProjectionSpec { mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {}, orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top } };
    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, top), "📐️ the press moves the pane");
    assert!(!state.camera.projection_frame_owed, "📐️ local selection retains the viewport owner instead of invoking the content framer");
    assert!(state.camera.projection_selected, "📐️ and holds its own spec from here on, React's `externalPendingProjectionSpec`");
    assert_eq!(state.camera.projection_frame_key, Some(7));
    assert_eq!(state.camera.projection_frame_zoom, Some(3.0));
    assert_eq!(state.camera.projection_spec, top);

    assert!(!apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, top), "📐️ pressing the SAME row again changes nothing");
    let cabinet = semio_framework_ui_viewport::Viewport3dProjectionSpec { mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Oblique { variant: semio_framework_ui_viewport::Viewport3dObliqueVariant::Cabinet, angle: 45.0, depth_scale: 0.5 }, orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Free {} };
    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, cabinet), "📐️ but a row with the same family and another plane does — Cabinet after Orthographic");
    assert_eq!(state.camera.projection_spec, cabinet, "📐️ a free oblique retains every framing and shear parameter");

    let mut wired = World3dState::new("surface".into(), "controller".into());
    wired.session.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    assert!(apply_world3d_projection_spec(&mut wired.session, &mut wired.camera, &mut wired.scene, top));
    let echo = OrbitController { projection: CameraProjection3d::Perspective, target: Vec3::new(7.0, 0.0, 0.01), ..OrbitController::default() };
    let held = OrbitController { projection: wired.camera.orbit.projection, ..echo.clone() };
    assert_eq!(held.projection, CameraProjection3d::Orthographic, "📐️ the echoed pose lands, the echoed FAMILY does not");
    assert_eq!(held.target, echo.target, "📐️ everything else the wire says still wins");
}

/// 🐟️ LAW: Top → Curvilinear keeps the accepted Top eye/target/distance/zoom while replacing the
/// projection family and FOV. This is the exact mounted React transition after initial framing.
#[test]
fn curvilinear_selection_retains_the_accepted_top_pose_and_zoom() {
    let mut state = World3dState::new("surface-curvilinear-frame".into(), "controller".into());
    state.session.bounds = Rect::new(0.0, 0.0, 400.0, 800.0);
    state.camera.orbit.projection = CameraProjection3d::Orthographic;
    state.camera.orbit.target = Vec3::new(5.4054055, 2.3406067, 1.5015);
    state.camera.orbit.distance = 19.855713;
    state.camera.orbit.zoom = 6.1791667;
    state.camera.projection_spec = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {},
        orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top },
    };
    state.camera.projection_selected = true;
    let accepted = state.camera.orbit.clone();
    let spec = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Curvilinear { fov: 120.0, strength: 1.0, mapping: semio_framework_ui_viewport::Viewport3dCurvilinearMapping::Fisheye },
        orientation: state.camera.projection_spec.orientation,
    };

    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, spec));
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    let camera = state.camera.orbit.to_camera();
    assert_eq!(camera.target.to_array(), accepted.target.to_array());
    assert_eq!(camera.position.to_array(), accepted.to_camera().position.to_array());
    assert_eq!(state.camera.orbit.distance, accepted.distance);
    assert_eq!(state.camera.orbit.zoom, accepted.zoom);
    assert!((camera.fov_y.to_degrees() - 120.0).abs() <= 2e-5);
    assert_eq!(camera.projection, CameraProjection3d::Perspective);
    assert!(!state.camera.projection_frame_owed);
}

/// 📥️ LAW: a delivered full projection spec owns one frame after its camera snapshot is accepted.
/// The bridge exposes instance bounds before the staged camera page lands, so consuming the frame any
/// earlier lets snapshot apply overwrite it. A later document-bound change stays put until another
/// external seed or explicit projection selection rearms the debt.
#[test]
fn delivered_projection_spec_frames_once_after_accepted_camera() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let spec = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {},
        orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top },
    };
    let mut scene = camera_framing_scene(&fixture, "perspective", true, None);
    scene.world_3d.as_mut().expect("world scene").camera_json = serde_json::json!({
        "position": [40, -30, 20],
        "target": [0, 0, 0],
        "up": [0, 0, 1],
        "fov": 50,
        "zoom": 1,
        "projection": spec,
    })
    .to_string();
    let mut state = World3dState::new("surface-delivered-projection".into(), "controller".into());
    state.camera.camera_user_moved = true;

    drive_scene_bridge(&mut state, &scene, viewport);
    assert_eq!(state.camera.projection_spec, spec, "the typed delivered spec survives the scene bridge");
    assert!(state.camera.projection_frame_owed, "an accepted external projection seed owns one content frame");
    assert!(!state.camera.camera_user_moved, "external camera reattach releases the previous viewport owner");

    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert!(!state.camera.projection_frame_owed, "the accepted camera consumes the debt exactly once");
    let accepted = state.camera.orbit.clone();
    state.scene.instance_positions.insert("late-growth".into(), [1_000.0, 1_000.0, 1_000.0]);
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert_eq!(state.camera.orbit.target.to_array(), accepted.target.to_array());
    assert_eq!(state.camera.orbit.to_camera().position.to_array(), accepted.to_camera().position.to_array());
}
