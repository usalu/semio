/// 🌐️ World3d window, pointer and orbit laws: the preview window attach, the graph domain selection dispatches, the orbit and wheel camera payload and the bounded thread stack.

/// 🧱️ The paint law, keyed by surface id so it can run twice in one binary — once on libtest's own
/// thread and once on the deliberately 2 MiB lane below, without the two sharing a surface.
fn world3d_preview_window_law(surface_id: &str) {
    let fixture = fixture();
    let scene = world3d_preview_scene(surface_id, &fixture);
    let bounds = Rect { x: 24.0, y: 36.0, w: 800.0, h: 600.0 };

    let painted = attach_and_settle_world3d(&scene, bounds, surface_id);

    let expect = &fixture["world3d"]["expect"];
    let state = painted.world3d_states.get(surface_id).expect("attached world state");
    assert_eq!(state.snapshot_fault(), None, "the production attach reaches a published snapshot without faulting");
    let pass = painted.draw.scene_passes.first().expect("the World3d host paints a real 3-D pass into the window draw list");
    assert_eq!(pass.viewport, [bounds.x, bounds.y, bounds.w, bounds.h], "the pass is sized to the window body");
    assert_eq!(pass.draws.len(), expect["draws"].as_u64().expect("draws") as usize, "each original surface and wire mesh retains its draw");

    let draw = pass.draws.iter().find(|draw| Some(draw.mesh_key.as_str()) == expect["drawMeshKey"].as_str()).expect("published solid draw");
    assert_eq!(draw.mesh_key, expect["drawMeshKey"].as_str().expect("mesh key"));
    let instance_ids: Vec<&str> = draw.instances.iter().map(|instance| instance.id.as_str()).collect();
    let expected_ids: Vec<&str> = expect["instanceIds"].as_array().expect("instance ids").iter().map(|id| id.as_str().expect("id")).collect();
    assert_eq!(instance_ids, expected_ids, "instance ids stay channel-qualified through the production paint");
    let mesh = state.mesh_lease(&draw.mesh_key).expect("published mesh geometry");
    let schema = mesh.schema().expect("mesh schema");
    assert_eq!(schema.indices / 3, expect["triangles"].as_u64().expect("triangles") as u32, "the drawn mesh is the tessellated solid, not the 12-triangle placeholder box");
    assert_eq!(schema.vertices, expect["vertices"].as_u64().expect("vertices") as u32);
    assert!(draw.instances.first().expect("instance").selected, "the scene's own selection document reaches the painted instance");
    drop_world3d_states(painted.world3d_states);
}

#[test]
fn world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid() {
    world3d_preview_window_law("world3d-attach-draw");
}

/// 🖱️ The pick law, keyed by surface id for the same reason as `world3d_preview_window_law`.
///
/// 🎯️ `targets` is a JSON TEXT arg, never a nested array — the world authority writes it through
/// bounded string credits (`INTERACTION_TARGETS_OPEN`, `🌍️world/🦀️.rs:6343`), so the law parses it.
///
/// ⚖️ A pick reports the scene's interaction granularity, matching the shared React gesture oracle.
///
/// 🎯️ Aims through the fixture's solid interior away from the independently pickable extrusion axis.
///
/// 🖱️ A left press opens the marquee gesture and the RELEASE is what picks — the same
/// press-then-click contract React's World3dHost binds, so the lane drives both halves.
fn world3d_pointer_down_law(surface_id: &str) {
    let fixture = fixture();
    let scene = world3d_preview_scene_with_selection(surface_id, &fixture, json!({ "method": "pick", "mode": "replace", "ids": [] }).to_string());
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut states = attach_and_settle_world3d(&scene, bounds, surface_id).world3d_states;
    let state = states.get_mut(surface_id).expect("attached world state");

    let camera = state.camera.orbit.to_camera();
    let point = &fixture["world3d"]["expect"]["pickWorldPosition"];
    let point = ui_wgpu::wgpu::Vec3::new(point[0].as_f64().unwrap() as f32, point[1].as_f64().unwrap() as f32, point[2].as_f64().unwrap() as f32);
    let screen = semio_framework_ui_scene_picking::project_point(camera.view_proj(bounds.w, bounds.h), point, bounds.w, bounds.h).expect("the original solid pick target projects");
    let tolerance = fixture["world3d"]["expect"]["pickEdgeScreenTolerancePx"].as_f64().unwrap() as f32;
    for mesh in fixture["world3d"]["meshesJson"].as_array().unwrap() {
        if mesh["data"]["indices"].as_array().is_some_and(|indices| !indices.is_empty()) { continue; }
        if let Some(edges) = mesh["data"]["edgePositions"].as_array() {
            for edge in edges.chunks_exact(6) {
                let project = |index: usize| semio_framework_ui_scene_picking::project_point(camera.view_proj(bounds.w, bounds.h), ui_wgpu::wgpu::Vec3::new(edge[index].as_f64().unwrap() as f32, edge[index + 1].as_f64().unwrap() as f32, edge[index + 2].as_f64().unwrap() as f32), bounds.w, bounds.h).unwrap();
                let (a, b) = (project(0usize), project(3usize));
                let distance = semio_framework_ui_scene_picking::screen_segment_distance(screen[0], screen[1], a[0], a[1], b[0], b[1]);
                assert!(distance > tolerance, "original solid aim intersects independent wire screen tolerance: {distance} <= {tolerance}");
            }
        }
    }
    let mut actions = publish_world3d_intent(state, WorldInteractionIntent::pointer_button(screen[0], screen[1], true, 0, &PointerModifiers::default()));
    actions.extend(publish_world3d_intent(state, WorldInteractionIntent::pointer_button(screen[0], screen[1], false, 0, &PointerModifiers::default())));
    drop_world3d_states(states);

    let expected_id = fixture["world3d"]["expect"]["instanceIds"][0].as_str().expect("instance id");
    let select = actions.iter().find(|action| action.action == "interactionSelect").expect("a pointer-down over an instance publishes interactionSelect");
    assert_eq!(select.controller_id, "generation3d");
    let args = select.args.clone().expect("select args");
    assert_eq!(args["domainId"].as_str(), Some("graph"), "the window's own bound domain, never the OS `world` fallback");
    assert_eq!(args["merge"].as_str(), Some("replace"));
    assert_eq!(args["method"].as_str(), Some("pick"));
    let targets: Value = serde_json::from_str(args["targets"].as_str().expect("targets is written as JSON text")).expect("targets text parses");
    assert_eq!(targets[0]["granularity"], fixture["world3d"]["domainGranularityId"]);
    let expected_channel_id = expected_id.split_once('#').map_or(expected_id, |(channel, _)| channel);
    let oracle = r#"import{PerspectiveCamera,Vector3,BufferGeometry,Float32BufferAttribute,Mesh,MeshBasicMaterial,DoubleSide,Raycaster,LineSegments,LineBasicMaterial}from'three';const f=JSON.parse(process.argv[1]),c=f.cameraJson,camera=new PerspectiveCamera(c.fov,800/600,.01,1000);camera.position.fromArray(c.position);camera.up.fromArray(c.up);camera.lookAt(new Vector3().fromArray(c.target));camera.updateMatrixWorld();const p=new Vector3().fromArray(f.expect.pickWorldPosition).project(camera),ray=new Raycaster();ray.params.Line.threshold=f.expect.pickEdgeScreenTolerancePx*2*Math.tan(c.fov*Math.PI/360)*camera.position.distanceTo(new Vector3().fromArray(c.target))/600;ray.setFromCamera(p,camera);const meshes=f.instancesJson.flatMap(i=>{const d=f.meshesJson.find(m=>m.id===i.meshId).data,g=new BufferGeometry();g.setAttribute('position',new Float32BufferAttribute(d.indices?.length?d.positions:d.edgePositions??d.positions,3));if(d.indices?.length)g.setIndex(d.indices);const m=d.indices?.length?new Mesh(g,new MeshBasicMaterial({side:DoubleSide})):new LineSegments(g,new LineBasicMaterial());m.position.fromArray(i.position);m.quaternion.fromArray(i.rotation);m.scale.fromArray(i.scale);m.name=i.interactionId;m.updateMatrixWorld();return[m]});const hit=ray.intersectObjects(meshes)[0];if(!hit)throw Error('original solid point misses');process.stdout.write(hit.object.name);"#;
    let witness = std::process::Command::new("bun").args(["-e", oracle, &fixture["world3d"].to_string()]).output().expect("independent Three solid pick oracle");
    assert!(witness.status.success(), "Three solid pick oracle: {}", String::from_utf8_lossy(&witness.stderr));
    assert_eq!(String::from_utf8(witness.stdout).unwrap(), expected_channel_id);
    assert_eq!(targets[0]["id"].as_str(), Some(expected_channel_id), "the target is the bare channel-qualified id React's World3dHost dispatches — never `surfaceId/id` and never the render id");
}

#[test]
fn world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches() {
    world3d_pointer_down_law("world3d-attach-pick");
}

/// 📷️ The camera law, keyed by surface id for the same reason as `world3d_preview_window_law`.
///
/// 🪟️ `windowId`, never `surfaceId` — `worldCameraSetCameraDispatchArgs` addresses the WINDOW
/// instance, and the guest camera value has no `fov` member: React sends `{position, target,
/// zoom, up?}` and a stray key fails the whole `camera` deserialization.
///
/// 🖱️ Orbit is `button == 2 && (alt || meta)` — the same chord React's World3dHost binds; a bare
/// right-drag pans instead, and a plain left-drag is a marquee, not a camera move.
fn world3d_orbit_and_wheel_law(surface_id: &str) {
    let fixture = fixture();
    let scene = world3d_preview_scene(surface_id, &fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut states = attach_and_settle_world3d(&scene, bounds, surface_id).world3d_states;
    let state = states.get_mut(surface_id).expect("attached world state");
    let before = state.camera.orbit.to_camera().position.to_array();

    let wheel = publish_world3d_intent(state, WorldInteractionIntent::wheel(400.0, 300.0, -120.0, &PointerModifiers::default()));
    let zoomed = wheel.iter().find(|action| action.action == "setCamera").expect("a wheel over the preview publishes setCamera");
    assert_eq!(zoomed.controller_id, "generation3d");
    let args = zoomed.args.clone().expect("camera args");
    assert_eq!(args["windowId"].as_str(), Some(surface_id));
    assert!(args.get("surfaceId").is_none(), "the pre-fix `surfaceId` address is gone, not merely joined by `windowId`");
    assert!(args["camera"]["position"].as_array().is_some_and(|axes| axes.len() == 3), "generation3d's 📷️set-camera payload carries a 3-axis position");
    assert!(args["camera"]["target"].as_array().is_some_and(|axes| axes.len() == 3));
    assert_eq!(args["camera"]["zoom"].as_f64(), Some(1.0), "a perspective orbit reports React's identity zoom");
    assert_eq!(args["camera"]["up"].as_array().map(<[_]>::len), Some(3));
    assert!(args["camera"].get("fov").is_none(), "`fov` is not part of the React `setCamera` camera value");
    let after = state.camera.orbit.to_camera().position.to_array();
    let moved = (0..3).any(|axis| (after[axis] - before[axis]).abs() > 1.0e-4);
    assert!(moved, "the wheel actually moved the orbit camera the action reports: {before:?} -> {after:?}");

    let dragged = publish_world3d_intent(state, WorldInteractionIntent::pointer_move(420.0, 320.0, 20.0, 20.0, true, 2, &PointerModifiers { alt: true, ..PointerModifiers::default() }));
    let orbited = dragged.iter().find(|action| action.action == "setCamera").expect("an orbit drag publishes setCamera");
    let orbit_args = orbited.args.clone().expect("orbit camera args");
    assert_eq!(orbit_args["windowId"].as_str(), Some(surface_id));
    let orbited_position = state.camera.orbit.to_camera().position.to_array();
    let turned = (0..3).any(|axis| (orbited_position[axis] - after[axis]).abs() > 1.0e-4);
    assert!(turned, "the orbit drag actually turned the camera: {after:?} -> {orbited_position:?}");
    drop_world3d_states(states);
}

#[test]
fn world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload() {
    world3d_orbit_and_wheel_law("world3d-attach-camera");
}

/// 🧵️ Runs all three World3d laws on a lane with an EXPLICIT 2 MiB stack — the budget
/// `std::thread` hands a spawned thread, and the budget the native frame-build pool worker that
/// actually drives `AppFrameTransactionPhase::World3dSnapshot` runs on (`WorkerPool` spawns its
/// workers with no `stack_size` at `🧰️framework/🔨️modules/⏳️async/🦀️.rs:1780`, and
/// `🧵️frame-job/🦀️.rs:474` submits the frame build to it).
///
/// `Builder::stack_size` overrides `RUST_MIN_STACK`, so the repo runner's blanket 128 MiB floor
/// (`runCargoTestBudgeted`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🟦️.ts`)
/// cannot hide a re-inflated frame here the way it hid the 5 664 768-byte
/// `AdmittedSurfaceMap::<World3dState>::default` array literal until 2026-09-12. Behavioural, not a
/// source-shape assertion: any future frame that grows back past the budget aborts this lane.
#[test]
fn world3d_engine_surface_laws_fit_a_bounded_thread_stack() {
    let lane = std::thread::Builder::new()
        .name("world3d-bounded-stack".to_string())
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            world3d_preview_window_law("world3d-bounded-draw");
            world3d_pointer_down_law("world3d-bounded-pick");
            world3d_orbit_and_wheel_law("world3d-bounded-camera");
        })
        .expect("the bounded-stack lane spawns");
    if let Err(panic) = lane.join() {
        std::panic::resume_unwind(panic);
    }
}
