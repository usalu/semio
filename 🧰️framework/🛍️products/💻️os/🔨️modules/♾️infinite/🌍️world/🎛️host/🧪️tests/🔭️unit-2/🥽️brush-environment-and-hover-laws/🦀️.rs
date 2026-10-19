/// 🥽️ Laws of the glb asset and brush mesh announce, environment and shadows, terrain, the context menu and domain-bound previews.

//#region 🔖️WorldInteractionVerbs
//#region TerrainTests
//#region GlbAssetTests
fn allocate_oracle_mesh(token:Mesh3dWriteToken) {
    loop {
        let grant=world_mesh_ownership_grant(0,ui_scene::math::mesh3d_allocate_capacity_byte_demand(token).unwrap(),0,1);
        let step=mesh3d_allocate_step(token,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

fn world_component_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<u32> {
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 8, gesture).expect("component marquee cursor");
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)) == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 8_000);
    }
    let mut ids = Vec::new();
    for item in 0..usize::from(cursor.results.lens[0]) {
        let WorldMarqueeResult::Component { id, .. } = cursor.results.pages[0][item].expect("component marquee result") else {
            panic!("component marquee produced object result");
        };
        ids.push(id);
    }
    ids.sort_unstable();
    ids
}

/// 📨️ Drives one gumball publication job to completion and answers what it published (nothing for a `Skip`).
fn world_gumball_publish(state: &World3dState, gesture: WorldGumballGesture, phase: WorldGumballPhase) -> Option<ActionDescriptor> {
    let mut job = WorldGumballCommitJob::new(8, gesture, phase);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    for _ in 0..4096 {
        if with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).expect("bounded gumball publication") == WorldInteractionStep::Complete {
            break;
        }
    }
    assert!(job.terminal_is_empty(), "the publication retires its tokens");
    let mut actions = take_actions(&mut input);
    assert!(actions.len() <= 1, "one publication is at most one dispatch");
    actions.pop()
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

/// 🎯️ The generation3d `box-fillet-preview` delivery extent, as `📚️examples/📐️box-fillet-preview/🧫️fixtures/🧩️example/🔣️.json` commits it.
const BOOT_FRAME_FIT_JSON: &str = r#"{"enabled":true,"revision":7,"padding":1.12,"boundsMin":[0.0,0.0,0.0],"boundsMax":[2.0,2.0,2.0]}"#;

fn draw_fixture_bytes(draws: &[(&str, &[&str])]) -> u32 {
    draws.iter().map(|(mesh, instances)| mesh.len() + size_of::<SceneDraw3d>() + instances.iter().map(|id| id.len() + size_of::<Instance3d>()).sum::<usize>()).sum::<usize>() as u32
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

//#endregion GlbAssetTests

//#region 🥽️BrushMeshAnnounceTests
/// 🧫️ React's measured `registerBrushMesh` rows and the arg shapes it sends, from the 2026-09-18
/// side-by-side run (`🗑️generated/w11a-parity-run-14/steps.json`).
#[allow(non_snake_case)]
fn BRUSH_MESH_REGISTRATION() -> &'static str {
    read_static("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🥽️brush-mesh-registration.json")
}

/// 🔒️ The announcement registry is process-wide by design (React's is a module singleton), so the
/// laws below take it one at a time whatever `--test-threads` says.
static BRUSH_MESH_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn brush_mesh_fixture() -> serde_json::Value {
    serde_json::from_str(BRUSH_MESH_REGISTRATION()).expect("brush mesh registration fixture parses")
}

/// 🧫️ A mesh whose payload spans SEVERAL pages, so the run's positions-then-indices split falls
/// inside a page the way a real GLB's does: 400 vertices (1 200 position values) plus 900 indices is
/// 2 100 values, three pages at [`WORLD_BRUSH_MESH_PAGE_VALUES`].
fn brush_mesh_oracle() -> TerrainMeshOracleData {
    let vertices = 400_u32;
    let positions: Vec<f32> = (0..vertices).flat_map(|vertex| [vertex as f32, (vertex % 7) as f32, (vertex % 13) as f32]).collect();
    let normals: Vec<f32> = (0..vertices).flat_map(|_| [0.0, 0.0, 1.0]).collect();
    let indices: Vec<u32> = (0..900_u32).map(|index| index % vertices).collect();
    mesh_oracle_from_buffers(positions, normals, indices)
}

/// 🧫️ A bridged world pane whose url-declared mesh is RESIDENT — the state React reaches when its
/// `useLoader(GLTFLoader, …)` resolves and `BrushMeshRegistrar` fires.
fn resident_brush_mesh_surface(surface_id: &str) -> World3dState {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new(surface_id.into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });
    let mesh = publish_oracle_mesh_at_revision(brush_mesh_oracle(), state.session.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, URL_MESH_URL, mesh).expect("the decoded GLB publishes under the wire's mesh id");
    state
}

fn brush_mesh_text(action: &ActionDescriptor, key: &str) -> Option<String> {
    action.args.as_ref().and_then(|args| args.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string)
}

fn brush_mesh_number(action: &ActionDescriptor, key: &str) -> Option<u64> {
    action.args.as_ref().and_then(|args| args.get(key)).and_then(semio_framework_value::DslValue::as_f64).map(|value| value as u64)
}

fn brush_mesh_carries(action: &ActionDescriptor, key: &str) -> bool {
    action.args.as_ref().and_then(|args| args.get(key)).is_some()
}

fn drain_brush_mesh_run(state: &mut World3dState) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    for _ in 0..(WORLD_BRUSH_MESH_MAX_PAGES as usize + 2) {
        let Some(action) = step_world3d_brush_mesh_announce(&mut state.session, &mut state.scene, &mut state.assets) else { break };
        actions.push(action);
    }
    actions
}

/// 🥽️ LAW: a resident mesh emits exactly ONE `registerBrushMesh` per window instance per revision,
/// with React's own arg shape — and never a second one in the same frame or the next.
///
/// 🩸️ The wgpu host dispatched nothing at all for family B of the parity diff, so the guest's brush
/// and volume-brush utilities had no collision body on this renderer while React announced on all
/// seven measured steps (`📓️w11a-prepared-world-mesh-missing.md` §6 family B).
#[test]
fn a_resident_mesh_announces_one_register_brush_mesh_per_window_with_reacts_args() {
    let _guard = BRUSH_MESH_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_world3d_brush_mesh_registry();
    let fixture = brush_mesh_fixture();
    let windows: Vec<String> = fixture["windows"].as_array().expect("window list").iter().map(|entry| entry.as_str().expect("window id").to_string()).collect();
    assert_eq!(windows.len(), 2, "React journals both window instances on every measured step");

    let mut first = resident_brush_mesh_surface(&windows[0]);
    let run = drain_brush_mesh_run(&mut first);
    assert!(!run.is_empty(), "the first window pages the geometry it just made resident");
    assert!(run.iter().all(|action| action.action == fixture["action"].as_str().expect("action id")), "every row is a registerBrushMesh");
    assert_eq!(brush_mesh_text(&run[0], "surfaceId").as_deref(), Some(windows[0].as_str()), "React sends the pane's own surface id");
    assert_eq!(brush_mesh_text(&run[0], "windowId").as_deref(), Some(windows[0].as_str()), "and scopes the announcement to the window instance that owns the mesh");
    assert_eq!(brush_mesh_text(&run[0], "url").as_deref(), Some(URL_MESH_URL));
    assert_eq!(brush_mesh_number(&run[0], "pageCount"), Some(run.len() as u64), "the run declares its own length on every page");
    let digest = brush_mesh_text(&run[0], "digest").expect("the identity is the digest the guest verifies the run against");
    assert_eq!(digest.len(), 64, "unkeyed BLAKE3, hex");
    assert!(step_world3d_brush_mesh_announce(&mut first.session, &mut first.scene, &mut first.assets).is_none(), "an announced identity is never re-announced at the same revision");
    assert!(!world3d_brush_mesh_announce_pending(&first.scene, &first.assets), "and the surface owes the guest nothing more");

    let mut second = resident_brush_mesh_surface(&windows[1]);
    let sibling = drain_brush_mesh_run(&mut second);
    assert_eq!(sibling.len(), 1, "a sibling window announces the identity the registry already holds by id alone");
    assert_eq!(brush_mesh_text(&sibling[0], "windowId").as_deref(), Some(windows[1].as_str()), "scoped to ITS window, which is why React journals a row per window");
    assert_eq!(brush_mesh_text(&sibling[0], "url").as_deref(), Some(URL_MESH_URL));
    assert_eq!(brush_mesh_text(&sibling[0], "digest"), Some(digest), "by the digest the first window put into the guest");
    assert!(!brush_mesh_carries(&sibling[0], "page"), "and with no payload at all");
    assert!(!brush_mesh_carries(&sibling[0], "positionsB64"));

    retire_bridged_surface(&mut first);
    retire_bridged_surface(&mut second);
    reset_world3d_brush_mesh_registry();
}

/// 🥽️ LAW: a page run carries the mesh's little-endian positions followed by its little-endian
/// indices, positions filling each page first — the exact layout the guest's
/// `decode_brush_mesh_page_values` reassembles and its `brush_mesh_digest` hashes.
#[test]
fn a_brush_mesh_run_pages_positions_then_indices_under_the_announced_digest() {
    let _guard = BRUSH_MESH_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_world3d_brush_mesh_registry();
    let oracle = brush_mesh_oracle();
    let mut expected: Vec<u8> = oracle.positions.iter().flat_map(|value| value.to_le_bytes()).collect();
    expected.extend(oracle.indices.iter().flat_map(|value| value.to_le_bytes()));

    let mut state = resident_brush_mesh_surface("puzzle3d-main-top");
    let run = drain_brush_mesh_run(&mut state);
    assert!(run.len() > 2, "the oracle spans several pages, so the positions-then-indices split falls inside one");
    assert!(run.iter().filter(|action| brush_mesh_carries(action, "positionsB64") && brush_mesh_carries(action, "indicesB64")).count() == 1, "exactly one page carries the seam");
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    for action in &run {
        assert_eq!(brush_mesh_text(action, "digest"), Some(semio_framework_hash::hash_bytes(&expected)), "every page names the identity the whole run hashes to");
        if let Some(encoded) = brush_mesh_text(action, "positionsB64") {
            positions.extend(base64_codec::base64_standard_decode(&encoded).expect("page payload is base64"));
        }
        if let Some(encoded) = brush_mesh_text(action, "indicesB64") {
            indices.extend(base64_codec::base64_standard_decode(&encoded).expect("page payload is base64"));
        }
    }
    positions.extend(indices);
    assert_eq!(positions, expected, "the run reassembles into the resident mesh's own geometry, positions first");

    retire_bridged_surface(&mut state);
    reset_world3d_brush_mesh_registry();
}

/// 🚚️ LAW: the guest's own two published facts drive every re-announcement — a residency that fell
/// is a restarted guest and voids every claim, and a standing `meshReuploadUrls` entry is an
/// identity whose bytes the guest is waiting for. React folds the identical pair into
/// `brushMeshRevisions` (`🌐️World3dHost/🟦️.tsx`).
#[test]
fn a_guest_restart_or_a_reupload_request_makes_every_surface_announce_again() {
    let _guard = BRUSH_MESH_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_world3d_brush_mesh_registry();
    let mut state = resident_brush_mesh_surface("puzzle3d-main-top");
    assert!(drain_brush_mesh_run(&mut state).iter().any(|action| brush_mesh_carries(action, "positionsB64")), "the first run pages the bytes");
    assert!(!world3d_brush_mesh_announce_pending(&state.scene, &state.assets));

    sync_world3d_brush_mesh_feedback(&mut state.assets, Some(r#"{"meshResidency":7,"meshReuploadUrls":[]}"#));
    assert!(!world3d_brush_mesh_announce_pending(&state.scene, &state.assets), "a climbing residency is the guest confirming what it holds, not news");

    sync_world3d_brush_mesh_feedback(&mut state.assets, Some(&format!(r#"{{"meshResidency":7,"meshReuploadUrls":["{URL_MESH_URL}"]}}"#)));
    assert!(world3d_brush_mesh_announce_pending(&state.scene, &state.assets), "a standing request is an identity the guest cannot serve");
    let reupload = drain_brush_mesh_run(&mut state);
    assert!(reupload.iter().any(|action| brush_mesh_carries(action, "positionsB64")), "a claimed request takes the PAGE path, never the id-only one");

    sync_world3d_brush_mesh_feedback(&mut state.assets, Some(r#"{"meshResidency":0,"meshReuploadUrls":[]}"#));
    assert!(world3d_brush_mesh_announce_pending(&state.scene, &state.assets), "a residency below the high-water mark is proof of a restarted guest");
    assert!(drain_brush_mesh_run(&mut state).iter().any(|action| brush_mesh_carries(action, "positionsB64")), "and the voided claim pages the bytes again");

    retire_bridged_surface(&mut state);
    reset_world3d_brush_mesh_registry();
}

/// 🪟️ LAW: every React-measured trigger is answered per WINDOW INSTANCE — a rebuilt surface starts
/// with an empty announced set and announces again, which is exactly what `window-reopen`,
/// `role-viewer` and `role-editor` journal on both panes.
#[test]
fn every_react_measured_trigger_announces_once_per_window_instance() {
    let _guard = BRUSH_MESH_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_world3d_brush_mesh_registry();
    let fixture = brush_mesh_fixture();
    let steps = fixture["reactSteps"].as_array().expect("measured steps").clone();
    assert_eq!(steps.len(), 7, "the parity run measured seven family-B steps");

    for step in &steps {
        let per_window = step["perWindow"].as_object().expect("rows per window");
        for (window, rows) in per_window {
            assert!(rows.as_u64().is_some_and(|rows| rows > 0), "React journals at least one row per window on {}", step["step"]);
            let mut state = resident_brush_mesh_surface(window);
            let announced = drain_brush_mesh_run(&mut state);
            assert!(!announced.is_empty(), "the wgpu twin answers {} on {window}", step["step"]);
            for action in &announced {
                assert_eq!(brush_mesh_text(action, "windowId").as_deref(), Some(window.as_str()), "every row is scoped to the window that owns the mesh");
            }
            assert!(step_world3d_brush_mesh_announce(&mut state.session, &mut state.scene, &mut state.assets).is_none(), "and no duplicate follows in the same frame");
            retire_bridged_surface(&mut state);
        }
    }
    reset_world3d_brush_mesh_registry();
}
//#endregion 🥽️BrushMeshAnnounceTests

#[test]
fn world_environment_reaches_the_actual_scene_pass() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🌞️scene-lighting/🔣️.json")).unwrap();
    let shadow_fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🌑️scene-shadows/🔣️.json")).unwrap();
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().expect("world fixture");
    world.environment_json = Some(fixture["worldEnvironment"].to_string());
    world.camera_json = serde_json::json!({
        "position": fixture["iconRenderRequest"]["camera"]["position"],
        "target": fixture["iconRenderRequest"]["camera"]["target"],
        "up": [0, 0, 1],
        "fov": 45
    })
    .to_string();

    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let bounds = Rect::new(0.0, 0.0, 128.0, 96.0);
    drive_scene_bridge(&mut state, &scene, bounds);
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = semio_framework_ui_draw_contract::DrawList::default();
    let mut atlas = semio_framework_ui_text::FontAtlas::builtin();
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let theme = semio_framework_ui_theme::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = semio_framework_ui_wgpu_widget_kit::WidgetContext {
        draw: &mut draw,
        overlay: None,
        atlas: &mut atlas,
        icons: None,
        input: &mut input,
        theme: &theme,
        scroll_offsets: &mut scroll,
        collapsed_sections: &mut collapsed,
        open_selects: &mut selects,
        interaction_maps: None,
        pick_clip: None,
        viewport_height: 96.0,
    };
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::World);

    let pass = draw.scene_passes.last().expect("World3d publishes its real pass");
    let environment = &fixture["worldEnvironment"];
    let ambient = parse_color(environment["ambient"]["color"].as_str().unwrap());
    let sun = parse_color(environment["sun"]["color"].as_str().unwrap());
    let emissive = parse_color(environment["material"]["emissive"].as_str().unwrap());
    assert_eq!(pass.camera_position, [4.0, -3.0, 8.0]);
    assert_eq!(pass.lighting.ambient_color, <[f32; 3]>::try_from(&ambient[..3]).unwrap());
    assert_eq!(pass.lighting.ambient_intensity, 0.42);
    assert_eq!(pass.lighting.sun_color, <[f32; 3]>::try_from(&sun[..3]).unwrap());
    assert_eq!(pass.lighting.sun_intensity, 1.7);
    assert!(pass.lighting.sun_enabled);
    assert_eq!(pass.neutral_material.metalness, 0.63);
    assert_eq!(pass.neutral_material.roughness, 0.27);
    assert_eq!(pass.neutral_material.emissive, <[f32; 3]>::try_from(&emissive[..3]).unwrap());
    assert_eq!(pass.neutral_material.emissive_intensity, 2.4);
    assert!(pass.shadow.enabled);
    assert_eq!(pass.shadow.map_size, WORLD_SHADOW_MAP_SIZE);
    assert!((pass.light_dir[0] + 0.4531539).abs() < 1e-5);
    assert!((pass.light_dir[1] - 0.7848856).abs() < 1e-5);
    assert!((pass.light_dir[2] - 0.42261827).abs() < 1e-5);
    let texture_matrix = ui_scene::math::directional_shadow_texture_matrix(pass.light_dir);
    for (actual, expected) in texture_matrix.into_iter().zip(shadow_fixture["oracle"]["shadowMatrix"].as_array().unwrap()) {
        assert!((actual - expected.as_f64().unwrap() as f32).abs() < 2e-6, "shadow matrix drifted from Three: {actual} != {expected}");
    }
    retire_bridged_surface(&mut state);
}

#[test]
fn offscreen_light_visible_glb_is_retained_only_in_the_actual_shadow_channel() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🌑️scene-shadow-parity/🔣️.json")).unwrap();
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().expect("world fixture");
    world.camera_json = serde_json::json!({
        "position": fixture["frusta"]["mainCamera"]["position"],
        "target": fixture["frusta"]["mainCamera"]["target"],
        "up": fixture["frusta"]["mainCamera"]["up"],
        "fov": fixture["frusta"]["mainCamera"]["fov"],
    })
    .to_string();
    world.environment_json = Some(serde_json::json!({
        "sun": { "enabled": true, "azimuth": fixture["frusta"]["sun"]["azimuth"], "elevation": fixture["frusta"]["sun"]["elevation"] },
        "shadow": { "enabled": true, "opacity": fixture["profiles"]["world"]["opacityInput"], "softness": fixture["profiles"]["world"]["softnessInput"] },
    })
    .to_string());

    let mut state = World3dState::new("surface-shadow".into(), "controller-shadow".into());
    let bounds = Rect::new(0.0, 0.0, 128.0, 128.0);
    drive_scene_bridge(&mut state, &scene, bounds);
    state.scene.meshes.insert("shadow-mesh".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("mesh lease admitted");
    state.scene.mesh_versions.insert("shadow-mesh".into(), 7).expect("mesh version admitted");
    state.scene
        .draws
        .push(SceneDraw3d {
            mesh_key: "shadow-mesh".into(),
            mesh_version: 7,
            instances: vec![
                Instance3d { component_source: None, id: "offscreen-caster".into(), model: Instance3d::model_from_trs([-7.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [0.25, 0.25, 0.25]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() },
                Instance3d { component_source: None, id: "visible-receiver".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [0.25, 0.25, 0.25]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() },
            ],
            shadow_role: Default::default(),
        })
        .expect("source draw admitted");

    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = semio_framework_ui_draw_contract::DrawList::default();
    let mut atlas = semio_framework_ui_text::FontAtlas::builtin();
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let theme = semio_framework_ui_theme::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = semio_framework_ui_wgpu_widget_kit::WidgetContext {
        draw: &mut draw,
        overlay: None,
        atlas: &mut atlas,
        icons: None,
        input: &mut input,
        theme: &theme,
        scroll_offsets: &mut scroll,
        collapsed_sections: &mut collapsed,
        open_selects: &mut selects,
        interaction_maps: None,
        pick_clip: None,
        viewport_height: 128.0,
    };
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::World);

    let pass = draw.scene_passes.last().expect("actual scene pass");
    let color_ids: Vec<&str> = pass.draws.iter().chain(&pass.translucent_draws).flat_map(|draw| draw.instances.iter().map(|instance| instance.id.as_str())).collect();
    let shadow_ids: Vec<&str> = pass.shadow_draws.iter().flat_map(|draw| draw.instances.iter().map(|instance| instance.id.as_str())).collect();
    assert!(!color_ids.contains(&"offscreen-caster") && color_ids.contains(&"visible-receiver"), "the main camera owns only its visible GLB instances");
    assert!(shadow_ids.contains(&"offscreen-caster") && shadow_ids.contains(&"visible-receiver"), "the light frustum retains the offscreen caster and visible receiver");
    assert!(pass.shadow_draws.iter().all(|draw| draw.shadow_role == SceneShadowRole3d { casts: true, receives: true }));
    assert!(pass.draws.iter().all(|draw| draw.shadow_role == SceneShadowRole3d { casts: true, receives: true }));
    assert_eq!(pass.shadow.map_size, fixture["profiles"]["world"]["mapSize"].as_u64().unwrap() as u32);
    assert!(gpu.has_mesh_request("shadow-mesh", 7), "an offscreen-only caster participates in this frame's mesh upload ownership");
    assert!(state.scene.mesh_pool.contains(&"shadow-mesh".to_string()), "the offscreen-only caster participates in retained mesh ownership");
    retire_bridged_surface(&mut state);
}

#[test]
fn retained_draw_rebuild_preserves_prepared_material_colors_from_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🔣️draws.json")).unwrap();
    for vector in fixture["draws"].as_array().unwrap() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        publish_retained_draw_fixture(&mut state, vector);
        let expected = vector["instances"].as_array().unwrap();
        assert_eq!(state.scene.draws[0].mesh_key, vector["mesh"].as_str().unwrap());
        assert_eq!(state.scene.draws[0].mesh_version, vector["version"].as_u64().unwrap());
        assert_eq!(state.scene.draws[0].instances.len(), expected.len());
        for (actual, expected) in state.scene.draws[0].instances.iter().zip(expected) {
            let color: [f32; 4] = serde_json::from_value(expected["color"].clone()).unwrap();
            assert_eq!(actual.id, expected["id"].as_str().unwrap());
            assert_eq!(actual.color, color);
        }
        assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
        while !with_world_step_context(1, |context| terrain_test_retire_then_dynamic(&mut state,context)) {}
        assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
    }
}

/// 📡️ The queue is not a to-do list nobody reads: an uncached visible tile is ADMITTED into the
/// bounded world-asset pipeline, so the renderer host's generic fetch pump drains it exactly as it
/// drains GLB meshes and reference images. Before this wiring the queue had no consumer outside a
/// unit test and terrain never loaded in production at all.
///
/// 🕸️ The tile mesh is built by a BOUNDED cursor — ONE scalar write per turn — so the draw appears
/// many turns after the bytes land, not on the very next frame. The turns are driven directly
/// (`step_world_terrain_mesh`) rather than through whole `sync_terrain_state` frames, which would
/// re-serialize every visible tile's mesh JSON per step.
#[test]
fn sync_terrain_state_queues_fetch_for_uncached_tile_and_builds_after_upload() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.terrain.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 9.7382, project_origin_lat: 52.3759, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    apply_terrain_style_if_changed_state(&mut state.session, &mut state.scene, &mut state.terrain);
    let camera = Camera3d { position: Vec3::new(0.0, 0.0, 300.0), target: Vec3::ZERO, up: Vec3::new(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0, projection: ui_scene::math::CameraProjection3d::Perspective, zoom: 1.0 };
    let (tile_draws, evicted) = sync_terrain_state(&mut state.session, &mut state.authority, &mut state.scene, &mut state.terrain, &mut state.assets, &camera);
    assert!(tile_draws.is_empty(), "no elevation data uploaded yet, nothing to draw");
    assert!(evicted.is_empty(), "nothing was cached yet, nothing to evict");
    assert!(!state.terrain.pending_terrain_tile_urls.is_empty(), "an uncached visible tile should be queued for byte-fetch");

    let owner = take_next_world3d_asset(&mut state.assets).expect("a visible uncached DEM tile is admitted as a fetch request");
    let (z, x, y) = match owner.kind() {
        WorldAssetRequestKind::Terrain { z, x, y } => (z, x, y),
        other => panic!("a DEM tile is admitted as a Terrain request, got {other:?}"),
    };
    assert_eq!(owner.url(), terrain_tile_url("/dem/{z}/{x}/{y}.png", z, x, y));
    return_world3d_asset(&mut state.assets, owner).expect("the admitted owner returns to its claim");
    let value = (100.0_f64 + 32768.0).round() as i64;
    let r = ((value >> 8) & 0xff) as u8;
    let g = (value - ((r as i64) << 8)).clamp(0, 255) as u8;
    let mut image = image::RgbaImage::new(256, 256);
    for pixel in image.pixels_mut() {
        *pixel = image::Rgba([r, g, 0, 255]);
    }
    let mut bytes = Vec::new();
    image::DynamicImage::ImageRgba8(image).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
    assert!(apply_world3d_terrain_tile_bytes(&mut state.terrain, z, x, y, &bytes), "fetched DEM bytes reach the terrain session");
    assert!(!state.terrain.pending_terrain_tile_urls.contains_key(&terrain_tile_url("/dem/{z}/{x}/{y}.png", z, x, y)), "an applied tile leaves the pending set");

    sync_terrain_state(&mut state.session, &mut state.authority, &mut state.scene, &mut state.terrain, &mut state.assets, &camera);
    assert!(state.terrain.terrain_pending.is_some() || state.terrain.terrain_build.is_some(), "an uploaded tile opens its mesh-build owner");
    let mut turns = 0;
    while state.terrain.terrain_pending.is_some() || state.terrain.terrain_build.is_some() {
        with_world_step_context(1,|context|{let result=step_world3d_terrain(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, &mut state.terrain,terrain_test_grant(),context);assert!(result.ownership.fits(terrain_test_grant()));assert!(!matches!(result.step,WorldTerrainMeshPublicationStep::Fault(_)));});
        turns += 1;
        assert!(turns < 1_000_000, "an uploaded tile converges to its vertex-coloured mesh");
    }
    let (tile_draws_after_upload, _) = sync_terrain_state(&mut state.session, &mut state.authority, &mut state.scene, &mut state.terrain, &mut state.assets, &camera);
    assert_eq!(tile_draws_after_upload.len(), 1, "one draw per tile — the ramp rides the mesh's own vertex colours, not ten flat bands");
}

#[test]
fn apply_terrain_style_if_changed_state_purges_stale_meshes_on_origin_change() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.terrain.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 0.0, project_origin_lat: 0.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    assert!(apply_terrain_style_if_changed_state(&mut state.session, &mut state.scene, &mut state.terrain).is_empty(), "first application has nothing to purge");
    let mesh_key = terrain_tile_mesh_key(&state.session.surface_id, 10, 1, 2);
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, mesh_key, publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])));
    state.terrain.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 5.0, project_origin_lat: 5.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    let purged = apply_terrain_style_if_changed_state(&mut state.session, &mut state.scene, &mut state.terrain);
    assert_eq!(purged.len(), 1, "an origin change should purge the previously-cached terrain mesh");
    assert!(state.scene.meshes.is_empty());
}
//#endregion TerrainTests

//#region ContextMenuTests
#[test]
fn resolve_world_context_menu_target_prioritizes_vortex_over_object_over_reference() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.local_hover_id = Some("reference:site-plan.png".into());
    assert_eq!(resolve_world_context_menu_target(&state.selection), Some(("reference", "site-plan.png".to_string())));

    state.selection.hovered_component_mode = Some("face".into());
    state.selection.hovered_component_object_id = Some("obj-1".into());
    assert_eq!(resolve_world_context_menu_target(&state.selection), Some(("object", "obj-1".to_string())));

    state.selection.hovered_vortex_id = Some("vortex-1".into());
    assert_eq!(resolve_world_context_menu_target(&state.selection), Some(("vortex", "vortex-1".to_string())));
}

#[test]
fn resolve_world_context_menu_target_is_none_without_any_hover() {
    let state = World3dState::new("surface-1".into(), "controller-1".into());
    assert_eq!(resolve_world_context_menu_target(&state.selection), None);
}

/// 🖱️📋️ **The dead-verb law.** A right click on a world surface publishes NO action of its own. The
/// menu is the SHELL's (`open_context_menu`, whose `resolve_context_menu_surface` fills a World3d
/// surface's hits/selection from `world3d_context_menu_surface`), and React deleted its own
/// `contextMenuAt` twin when the target moved onto the menu REQUEST — dispatching it anyway only
/// produced an `undeclaredActionDiagnostic` drop (`world3dContextMenuSurfaceV1`'s docstring). What a
/// release DOES owe is the unconditional camera report three's `OrbitControls` fires on every
/// pointer-up (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
#[test]
fn right_click_publishes_no_menu_verb_and_still_reports_the_camera() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    state.selection.hovered_vortex_id = Some("vortex-1".into());
    handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
    let action = handle_world3d_pointer_button(&mut state, 200.0, 200.0, false, 2, &PointerModifiers::default()).expect("a right release reports the camera");
    assert_eq!(action.action, "setCamera");
    assert!(action.args.expect("args").get("camera").is_some(), "the report carries the pose, never a menu target");
}

#[test]
fn right_drag_reports_the_camera_like_every_other_release() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    state.selection.hovered_vortex_id = Some("vortex-1".into());
    handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
    let action = handle_world3d_pointer_button(&mut state, 260.0, 260.0, false, 2, &PointerModifiers::default()).expect("right release should still sync camera");
    assert_eq!(action.action, "setCamera", "a right-drag reports the camera, and no renderer opens a menu from a drag");
}
//#endregion ContextMenuTests

/// 🕹️ default `granularity` ("object") is neither component-mode nor "mesh" — this is the
/// plain `world`-domain item pick path (see `pick_select_action`'s final fallback branch).
#[test]
fn pick_select_emits_batched_interaction_select_for_plain_object_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_scene::math::project_point(ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::new(0.25, 0.25, 0.0), inner.w, inner.h).expect("object projects");
    let action = pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, screen[0], screen[1], inner, true, false).expect("pick action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!(WORLD_INTERACTION_DOMAIN_ID));
    assert_eq!(args["method"], json!("pick"));
    assert_eq!(args["merge"], json!("additive"), "shift modifier maps to the canonical MergeMode label");
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0]["granularity"], json!(WORLD_ITEM_GRANULARITY_ID));
    assert_eq!(targets[0]["id"], json!("surface-1/obj-1"), "item target id is surfaceId/objectId (PathDelimited)");
}

#[test]
fn marquee_select_emits_batched_interaction_select_with_rectangle_method() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.selection.marquee_points = vec![[0.0, 0.0], [400.0, 400.0]];
    let action = marquee_select_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, false, true).expect("marquee action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["method"], json!("rectangle"));
    assert_eq!(args["merge"], json!("subtractive"), "ctrl modifier maps to the canonical MergeMode label");
    assert!(state.selection.marquee_points.is_empty(), "marquee is consumed after gathering targets");
}

/// 🖱️ Moving off the instance clears — empty `targets` is `next_hover`'s clear signal.
#[test]
fn pick_hover_emits_interaction_hover_and_clears_when_nothing_hit() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_scene::math::project_point(ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::new(0.25, 0.25, 0.0), inner.w, inner.h).expect("object projects");
    let action = pick_hover_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, screen[0], screen[1], inner).expect("hover action");
    assert_eq!(action.action, "interactionHover");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!(WORLD_INTERACTION_DOMAIN_ID));
    assert_eq!(args["channel"], json!("pointer"));
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets[0]["id"], json!("surface-1/obj-1"));

    let action = pick_hover_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, 5.0, 5.0, inner).expect("clear action");
    assert_eq!(action.action, "interactionHover");
    let args = action.args.expect("args");
    assert_eq!(args["targets"].as_str(), Some("[]"));
}

#[test]
fn apply_world_action_preview_applies_interaction_select_and_hover_for_world_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": serde_json::json!([{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-1" }]).to_string(),
                "merge": "replace",
                "method": "pick",
            })),
        },
    );
    assert_eq!(state.selection.selected_ids, vec!["obj-1".to_string()], "surfaceId/ prefix is stripped for this surface");

    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionHover".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "channel": "pointer",
                "targets": serde_json::json!([{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-2" }]).to_string(),
            })),
        },
    );
    assert_eq!(state.selection.local_hover_id.as_deref(), Some("obj-2"));

    apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &ActionDescriptor { controller_id: "controller-1".into(), action: "interactionHover".into(), args: Some(semio_framework::dsl_value!({ "domainId": WORLD_INTERACTION_DOMAIN_ID, "channel": "pointer", "targets": serde_json::json!([]).to_string() })) });
    assert!(state.selection.local_hover_id.is_none(), "empty targets clears hover");
}

#[test]
fn pick_select_emits_bare_id_into_bound_app_domain_when_window_binds_one() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bound_domain_id = Some("cad".into());
    state.session.bound_domain_granularity_id = Some("object".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_scene::math::project_point(ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::new(0.25, 0.25, 0.0), inner.w, inner.h).expect("object projects");
    let action = pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, screen[0], screen[1], inner, false, false).expect("pick action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("cad"), "targets the window's bound app domain, not the OS `world` fallback");
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("object"), "uses the bound domain's own granularity, not `item`");
    assert_eq!(targets[0]["id"], json!("obj-1"), "bare id — a bound domain is single-surface-scoped, no surfaceId/ prefix");
}

#[test]
fn pick_hover_emits_bare_id_into_bound_app_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bound_domain_id = Some("cad".into());
    state.session.bound_domain_granularity_id = Some("object".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_scene::math::project_point(ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::new(0.25, 0.25, 0.0), inner.w, inner.h).expect("object projects");
    let action = pick_hover_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, screen[0], screen[1], inner).expect("hover action");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("cad"));
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("object"));
    assert_eq!(targets[0]["id"], json!("obj-1"));
}

#[test]
fn marquee_select_emits_bare_ids_into_bound_app_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bound_domain_id = Some("features".into());
    state.session.bound_domain_granularity_id = Some("pin".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.selection.marquee_points = vec![[0.0, 0.0], [400.0, 400.0]];
    let action = marquee_select_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, false, false).expect("marquee action");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("features"));
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("pin"));
    assert_eq!(targets[0]["id"], json!("obj-1"));
}

/// 🚫️ An action for the OS `world` fallback domain must NOT apply once this window is bound to
/// its own app domain — otherwise the same click could ever light up two selection universes.
#[test]
fn apply_world_action_preview_respects_bound_app_domain_and_ignores_other_domains() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bound_domain_id = Some("cad".into());
    state.session.bound_domain_granularity_id = Some("object".into());
    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({ "domainId": WORLD_INTERACTION_DOMAIN_ID, "targets": serde_json::json!([{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-1" }]).to_string(), "merge": "replace", "method": "pick" })),
        },
    );
    assert!(state.selection.selected_ids.is_empty(), "an unbound-domain action must not apply once this window binds its own domain");

    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor { controller_id: "controller-1".into(), action: "interactionSelect".into(), args: Some(semio_framework::dsl_value!({ "domainId": "cad", "targets": serde_json::json!([{ "granularity": "object", "id": "obj-1" }]).to_string(), "merge": "replace", "method": "pick" })) },
    );
    assert_eq!(state.selection.selected_ids, vec!["obj-1".to_string()], "bare id applies as-is — no surfaceId/ stripping for a bound domain");
}

#[test]
fn sync_world3d_state_captures_scene_bound_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let scene = scene_with_selection_and_domain("{}", Some(("cad", "object")));
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.session.bound_domain_id.as_deref(), Some("cad"));
    assert_eq!(state.session.bound_domain_granularity_id.as_deref(), Some("object"));
    assert_eq!(resolved_domain_id(&state.session), "cad");
    assert_eq!(resolved_domain_granularity_id(&state.session), "object");
}
//#endregion 🔖️WorldInteractionVerbs
