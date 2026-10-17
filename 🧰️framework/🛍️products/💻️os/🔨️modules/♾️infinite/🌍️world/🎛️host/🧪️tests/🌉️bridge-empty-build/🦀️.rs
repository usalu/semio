//! 🧪️ host tests (🌉️bridge-empty-build): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{tool_run_trace, WorldDrawRebuildStep};
use semio_framework_os_infinite_world_scene::{step_world3d_terrain_retirement, WorldTerrainMeshPublicationStep};
use semio_framework_os_infinite_world_bridge::{begin_world3d_dynamic_retirement, world3d_draw_rebuild_terminal_is_empty, World3dSceneBridgeStep, World3dSnapshotApplyStep};
use semio_framework_os_infinite_world_interaction::step_world3d_draw_rebuild;
use semio_framework_os_infinite_world_host::{apply_runtime_draw_flags, step_world3d_dynamic_retirement, step_world3d_scene_bridge, step_world3d_snapshot, sync_world3d_state, world3d_dynamic_retirement_terminal_is_empty, World3dState};
use semio_framework_test_fixtures::fixture_text;
use ui_wgpu::wgpu::{Rect, SurfaceKind, UiComponentSceneNode, UiPresence, World3dScene};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

pub(crate) fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let grant=world_original_fixture_grant();
    let mut receipt=semio_framework_value::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX,grant),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
        &mut receipt,
    );
    let result=step(&mut context);
    assert!(context.retained_progress().fits(grant));
    result
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

//#region 🌉️World3dSceneBridge
/// 🌉️ The committed generation3d preview payload this lane drives the bridge with. Produced by the
/// real pipeline (`parse_dsl` → `FlowHost::evaluate` → `tessellate_geometry(0.05)` →
/// `preview_payload_from_eval`) in `s.procedural.generation3d`'s own example-geometry harness, whose
/// `parry3d` oracle independently confirms the same solid's volume and extent — see the fixture's
/// `provenance` block, and the twin assertion in that harness that keeps this file from rotting.
#[allow(non_snake_case)]
fn SCENE_BRIDGE_FIXTURE() -> &'static str {
    fixture_text!("../../../🧪️tests/🧫️fixtures/🌉️scene-bridge/🔣️.json")
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
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}

#[allow(non_snake_case)]
fn EMPTY_BUILD() -> &'static str {
    fixture_text!("../../../🧪️tests/🧫️fixtures/🌉️bridge-empty-build/🔣️.json")
}

fn empty_build_fixture() -> serde_json::Value {
    serde_json::from_str(EMPTY_BUILD()).expect("bridge empty-build fixture parses")
}

fn bounds() -> Rect {
    Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }
}

/// ♻️ Drives one surface's retirement ladder to terminal. `World3dState` carries no `Drop` under
/// `cfg(test)`, and snapshot slots are a PROCESS-wide fixed store — a test that walks away from a
/// sealed lease starves every other test in the binary, which is exactly what
/// `scene bridge stopped at Fault … (fault Some(Unavailable))` reads like from the other side.
fn retire(mut state: World3dState) {
    begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets);
    for turn in 0..8_192 {
        if with_world_step_context(64, |context| terrain_test_retire_then_dynamic(&mut state,context)) {
            break;
        }
        assert!(turn < 8_191, "the retirement ladder reaches terminal");
    }
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state), "the surface released every retained owner — {}", state.ingest_census());
}

/// 🌉️ Drives one staged build to a terminal step WITHOUT asserting which one, so the law reads the
/// answer rather than presuming it.
fn drive_bridge_to_terminal(state: &mut World3dState) -> World3dSceneBridgeStep {
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_scene_bridge(state, context)) {
            World3dSceneBridgeStep::Pending => {}
            terminal => return terminal,
        }
        assert!(turn < 4_095, "the bridge terminates within its turn ceiling");
    }
    unreachable!("the bridge terminates")
}

/// 🩸️ The republication the guest emits while it recomputes — same camera, so no camera page
/// can carry the build, which is exactly the shape that used to fault.
#[test]
fn a_republication_with_nothing_drawable_completes_and_keeps_the_last_geometry() {
    for case in empty_build_fixture()["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let settled = scene_bridge_fixture();
        let mut state = World3dState::new("generation3d-generate-preview".into(), "procedural.play".into());
        let scene = scene_from_bridge_fixture(&settled);
        drive_scene_bridge(&mut state, &scene, bounds());
        let settled_draws = state.scene.draws.len;
        let settled_meshes = state.scene.meshes.len;
        assert!(settled_draws > 0, "{name}: the committed payload publishes draws");

        let mut degenerate = scene_from_bridge_fixture(&settled);
        let world = degenerate.world_3d.as_mut().expect("world scene");
        world.meshes_json = case["meshesJson"].to_string();
        world.instances_json = case["instancesJson"].to_string();
        sync_world3d_state(&mut state, &degenerate, bounds());
        let step = drive_bridge_to_terminal(&mut state);

        assert_ne!(step, World3dSceneBridgeStep::Fault, "{name}: a mesh update never faults the bridge — {}", state.ingest_census());
        assert_eq!(state.session.snapshot_fault(), None, "{name}: and records no snapshot fault");
        assert_eq!(state.scene.draws.len, settled_draws, "{name}: the surface keeps the geometry it last had");
        assert_eq!(state.scene.meshes.len, settled_meshes, "{name}: and keeps its published meshes");
        retire(state);
    }
}

#[test]
fn the_same_degenerate_payload_is_never_restaged() {
    let case = empty_build_fixture()["cases"].as_array().expect("cases").first().expect("at least one case").clone();
    let settled = scene_bridge_fixture();
    let mut state = World3dState::new("generation3d-generate-preview".into(), "procedural.play".into());
    let scene = scene_from_bridge_fixture(&settled);
    drive_scene_bridge(&mut state, &scene, bounds());
    let mut degenerate = scene_from_bridge_fixture(&settled);
    let world = degenerate.world_3d.as_mut().expect("world scene");
    world.meshes_json = case["meshesJson"].to_string();
    world.instances_json = case["instancesJson"].to_string();
    sync_world3d_state(&mut state, &degenerate, bounds());
    assert_ne!(drive_bridge_to_terminal(&mut state), World3dSceneBridgeStep::Fault, "the first build completes");
    sync_world3d_state(&mut state, &degenerate, bounds());
    assert_eq!(drive_bridge_to_terminal(&mut state), World3dSceneBridgeStep::Idle, "an unchanged payload stages nothing — {}", state.ingest_census());
    retire(state);
}

#[test]
fn the_settled_payload_returns_after_a_degenerate_one() {
    let case = empty_build_fixture()["cases"].as_array().expect("cases").first().expect("at least one case").clone();
    let settled = scene_bridge_fixture();
    let mut state = World3dState::new("generation3d-generate-preview".into(), "procedural.play".into());
    let scene = scene_from_bridge_fixture(&settled);
    drive_scene_bridge(&mut state, &scene, bounds());
    let mut degenerate = scene_from_bridge_fixture(&settled);
    let world = degenerate.world_3d.as_mut().expect("world scene");
    world.meshes_json = case["meshesJson"].to_string();
    world.instances_json = case["instancesJson"].to_string();
    sync_world3d_state(&mut state, &degenerate, bounds());
    assert_ne!(drive_bridge_to_terminal(&mut state), World3dSceneBridgeStep::Fault, "the degenerate build completes");
    drive_scene_bridge(&mut state, &scene, bounds());
    assert!(state.scene.draws.len > 0, "the recomputed payload paints again — {}", state.ingest_census());
    assert_eq!(state.session.snapshot_fault(), None, "and the surface never quarantined — {}", state.ingest_census());
    retire(state);
}
