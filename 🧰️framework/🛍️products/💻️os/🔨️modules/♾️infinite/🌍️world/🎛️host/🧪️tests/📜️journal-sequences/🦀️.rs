//! 🧪️ host tests (📜️journal-sequences): compiled against the `testkit` surface of the world crates.

use semio_os_infinite_world_model::{world_mesh_ownership_grant, WorldInteractionIntent};
use semio_os_infinite_world_cursors::WorldInteractionAuthorityStep;
use semio_framework_test_fixtures::fixture_text;
#[cfg(any(test, feature = "testkit"))]
use semio_os_infinite_world_scene::TerrainMeshOracleData;
use semio_os_infinite_world_bridge::world_navigation_snapshot;
use semio_os_infinite_world_interaction::{enqueue_world3d_event, store_mesh, world3d_hover_clear_is_owed, world3d_hover_is_published, world3d_interaction_front_generation, world3d_pointer_move_folds};
use semio_os_infinite_world_host::{step_world3d_interaction, World3dState};
use semio_ui_declaration::layout::ActionDescriptor;
use ui_scene::math::{CameraProjection3d, Instance3d, Mat4, mesh3d_allocate_step, mesh3d_begin, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, SceneDraw3d};
use semio_ui_wgpu_input::PointerModifiers;
use semio_ui_theme::geometry::Rect;
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

pub(crate) fn take_actions(input: &mut semio_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}


use semio_os_infinite_world_host::with_world_step_context;
use semio_os_infinite_world_host::{allocate_oracle_mesh, mesh_oracle_from_buffers, publish_oracle_mesh, publish_oracle_mesh_at_revision};


fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}

#[allow(non_snake_case)]
fn JOURNAL_SEQUENCES() -> &'static str {
    fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/📜️journal-sequences.json")
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(JOURNAL_SEQUENCES()).expect("journal sequence fixture parses")
}

fn step_case(id: &str) -> serde_json::Value {
    fixture()["steps"].as_array().expect("step list").iter().find(|case| case["id"] == id).unwrap_or_else(|| panic!("fixture step {id}")).clone()
}

fn point(name: &str) -> [f32; 2] {
    let value = fixture()["surface"][name].as_array().expect("point").iter().map(|entry| entry.as_f64().expect("number") as f32).collect::<Vec<f32>>();
    [value[0], value[1]]
}

fn expected(id: &str, renderer: &str) -> Vec<String> {
    step_case(id)[renderer].as_array().expect("journal").iter().map(|entry| entry.as_str().expect("action id").to_string()).collect()
}

/// 🧫️ A pane whose pick rect covers the probe's aim point, with one small instance sitting on the
/// orbit target — the smallest scene that answers a hover, a pick and a camera gesture, and small
/// enough that a pan carries the target off it, which is what makes the hover CLEAR observable.
fn journey_state() -> World3dState {
    let mut state = World3dState::new("puzzle3d-main-perspective".into(), "controller-1".into());
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 560.0, h: 360.0 };
    state.session.pick_bounds = state.session.bounds;
    state.session.active_utility = "select".into();
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let data = mesh_oracle_from_buffers(vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 0.5, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2, 0, 2, 3]);
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), publish_oracle_mesh(data));
    let mesh_version = *state.scene.mesh_versions.get("mesh").expect("mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { component_source: None, id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state
}

/// 🏁️ Drives every queued intent — and the camera settle the drained queue owes — to idle, answering
/// with the ORDERED action ids published along the way.
fn journal(state: &mut World3dState) -> Vec<String> {
    journal_actions(state).into_iter().map(|action| action.action).collect()
}

fn journal_actions(state: &mut World3dState) -> Vec<ActionDescriptor> {
    let mut input = semio_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut published = Vec::new();
    let mut turns = 0;
    while let Some(generation) = world3d_interaction_front_generation(&state.camera, &state.authority) {
        let step = with_world_step_context(1, |context| step_world3d_interaction(state, generation, &mut input, context));
        assert_ne!(step, WorldInteractionAuthorityStep::Fault, "a probe gesture never faults the authority");
        published.extend(take_actions(&mut input));
        turns += 1;
        assert!(turns < 4096, "a bounded journey terminates");
        if step == WorldInteractionAuthorityStep::Idle {
            break;
        }
    }
    published
}

fn enqueue(state: &mut World3dState, intent: WorldInteractionIntent) {
    enqueue_world3d_event(&mut state.selection, &mut state.authority, intent).expect("the world interaction queue admits a probe intent");
}

#[test]
fn engagement_pointer_uses_the_live_selection_camera_clip_and_window_address() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🖱️engagement-pointer.json")).unwrap();
    let surface = fixture["surfaceId"].as_str().unwrap();
    let window = fixture["windowId"].as_str().unwrap();
    let rect = |name: &str| {
        let [x, y, w, h] = serde_json::from_value::<[f32; 4]>(fixture[name].clone()).unwrap();
        Rect::new(x, y, w, h)
    };
    let vector = |value: &serde_json::Value| {
        let [x, y, z] = serde_json::from_value::<[f32; 3]>(value.clone()).unwrap();
        ui_scene::math::Vec3::new(x, y, z)
    };
    for case in fixture["cases"].as_array().unwrap() {
        let mut state = World3dState::new(surface.into(), "process3d".into());
        state.session.window_instance_id = Some(window.into());
        state.session.bounds = rect("viewport");
        state.session.pick_bounds = rect("pickBounds");
        state.session.active_utility = "select".into();
        if case["declared"] == true {
            state.session.declared_action_ids = vec!["worldPointerMove".into(), "worldPointerDown".into()];
        }
        let mut camera = state.camera.orbit.to_camera();
        camera.position = vector(&case["camera"]);
        camera.target = vector(&case["target"]);
        camera.up = ui_scene::math::Vec3::new(0.0, 0.0, 1.0);
        state.camera.orbit = ui_scene::math::OrbitController::from_camera(&camera);
        let selection = serde_json::json!({ "engagementSessionActive": case["active"] });
        semio_os_infinite_world_bridge::sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &selection.to_string());
        let [x, y] = serde_json::from_value::<[f32; 2]>(case["point"].clone()).unwrap();
        let modifiers = PointerModifiers { shift: true, meta: true, ..PointerModifiers::default() };
        enqueue(&mut state, WorldInteractionIntent::pointer_move(x + 1.0, y, 0.0, 0.0, false, 0, &modifiers));
        enqueue(&mut state, WorldInteractionIntent::pointer_move(x, y, 0.0, 0.0, false, 0, &modifiers));
        enqueue(&mut state, WorldInteractionIntent::pointer_button(x, y, true, 0, &modifiers));
        let actions = journal_actions(&mut state).into_iter().filter(|action| matches!(action.action.as_str(), "worldPointerMove" | "worldPointerDown")).collect::<Vec<_>>();
        let Some(expected) = case["position"].as_array() else {
            assert!(actions.is_empty(), "{}", case["id"]);
            continue;
        };
        assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["worldPointerMove", "worldPointerDown"], "{}", case["id"]);
        for action in &actions {
            assert_eq!(action.controller_id, "process3d");
            let args = serde_json::to_value(action.args.as_ref().unwrap()).unwrap();
            assert_eq!(args["windowId"], window);
            assert_eq!(args["surfaceId"], surface);
            assert_eq!(args["pane"], fixture["pane"]);
            let position = args["position"].as_array().unwrap();
            assert_eq!(position.len(), expected.len());
            for (actual, target) in position.iter().zip(expected) {
                assert!((actual.as_f64().unwrap() - target.as_f64().unwrap()).abs() < 0.001, "{}", case["id"]);
            }
            if action.action == "worldPointerDown" {
                assert_eq!(args["shiftKey"], true);
                assert_eq!(args["ctrlKey"], false);
                assert_eq!(args["metaKey"], true);
            }
        }
        semio_os_infinite_world_bridge::sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, "{}");
        enqueue(&mut state, WorldInteractionIntent::pointer_button(x, y, false, 0, &modifiers));
        assert_eq!(journal(&mut state), ["setCamera"], "release after a delivered engagement ends");
    }
}

/// 🖱️ The probe's own drag: one move onto the aim point, a press, eight interpolated moves, a release.
fn drag(state: &mut World3dState, button: i16, to: [f32; 2]) {
    let modifiers = PointerModifiers::default();
    let from = point("aim");
    enqueue(state, WorldInteractionIntent::pointer_move(from[0], from[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(from[0], from[1], true, button, &modifiers));
    let mut last = from;
    for sample in 1..=8 {
        let next = [from[0] + (to[0] - from[0]) * sample as f32 / 8.0, from[1] + (to[1] - from[1]) * sample as f32 / 8.0];
        enqueue(state, WorldInteractionIntent::pointer_move(next[0], next[1], next[0] - last[0], next[1] - last[1], true, button, &modifiers));
        last = next;
    }
    enqueue(state, WorldInteractionIntent::pointer_button(last[0], last[1], false, button, &modifiers));
}

/// 🖱️ The probe's own click: one move onto the aim point, a press and a release at that same point.
fn click(state: &mut World3dState, button: i16) {
    let modifiers = PointerModifiers::default();
    let at = point("aim");
    enqueue(state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(at[0], at[1], true, button, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(at[0], at[1], false, button, &modifiers));
}

#[test]
fn native_terrain_brush_journals_one_typed_edit_only_for_a_complete_stroke() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/📜️journal-sequences/🏔️terrain-stroke.json")).unwrap();
    let viewport = |value| serde_json::from_value::<[f32; 2]>(value).unwrap();
    let dimensions = law["viewport"].clone();
    let modifiers = PointerModifiers::default();
    for case in law["cases"].as_array().unwrap() {
        let mut state = journey_state();
        let [w, h] = viewport(dimensions.clone());
        state.session.bounds = Rect::new(0.0, 0.0, w, h);
        state.session.pick_bounds = state.session.bounds;
        state.session.window_instance_id = Some("gis-terrain-window".into());
        let brush = |value| serde_json::json!({
            "activeUtility": "raiseTerrain",
            "terrainBrush": { "action": "editElevation", "arguments": { "operation": "raise-lower", "radius": law["radius"], "value": value } }
        }).to_string();
        state.terrain.set_terrain_interaction(Some(&brush(law["value"].as_i64().unwrap())));
        let terrain = |height| serde_json::json!({
            "elevationGrid": { "bounds": law["bounds"], "rowsMillimetres": vec![vec![i32::from(height) * 1000; 33]; 33] }
        }).to_string();
        state.terrain.set_terrain_json(Some(&terrain(0)));
        semio_os_infinite_world_scene::apply_terrain_style_if_changed_state(&mut state.session, &mut state.scene, &mut state.terrain);
        let mut camera = state.camera.orbit.to_camera();
        camera.position = ui_scene::math::Vec3::new(0.0, 0.0, 500.0);
        camera.target = ui_scene::math::Vec3::ZERO;
        camera.up = ui_scene::math::Vec3::new(0.0, 1.0, 0.0);
        camera.zoom = 1.0;
        camera.projection = CameraProjection3d::Orthographic;
        state.camera.orbit = ui_scene::math::OrbitController::from_camera(&camera);
        state.camera.projection_spec = semio_ui_viewport::Viewport3dProjectionSpec {
            mode: semio_ui_viewport::Viewport3dProjectionMode::Orthographic {},
            orientation: semio_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_ui_viewport::Viewport3dOrthographicView::Top },
        };
        let id = case["id"].as_str().unwrap();
        let points = case["points"].as_array().unwrap();
        let [x, y] = viewport(points[0].clone());
        enqueue(&mut state, WorldInteractionIntent::pointer_button(x, y, true, 0, &modifiers));
        assert!(journal_actions(&mut state).is_empty(), "{id} before movement");
        let (mut x, mut y) = (x, y);
        for point in points.iter().skip(1) {
            let [next_x, next_y] = viewport(point.clone());
            let intent = WorldInteractionIntent::pointer_move(next_x, next_y, next_x - x, next_y - y, true, 0, &modifiers);
            semio_os_infinite_world_interaction::enqueue_world3d_events(&mut state.authority, [intent]).unwrap();
            (x, y) = (next_x, next_y);
        }
        assert!(journal_actions(&mut state).is_empty(), "{id} before release");
        match case["beforeRelease"].as_str() {
            Some("captureLost") => enqueue(&mut state, WorldInteractionIntent::cancel(semio_os_infinite_world_model::WorldCancelReason::CaptureLost)),
            Some("blur") => enqueue(&mut state, WorldInteractionIntent::cancel(semio_os_infinite_world_model::WorldCancelReason::Blur)),
            Some("brushChange") => state.terrain.set_terrain_interaction(Some(&brush(12))),
            Some("sourceChange") => state.terrain.set_terrain_json(Some(&terrain(1))),
            None => {}
            _ => panic!("unknown brush interruption"),
        }
        enqueue(&mut state, WorldInteractionIntent::pointer_button(x, y, false, 0, &modifiers));
        let actions = journal_actions(&mut state);
        if case["start"].is_null() {
            assert!(actions.is_empty(), "{id} leaves no journal entry");
            continue;
        }
        let [action] = actions.as_slice() else { panic!("{id} must publish one elevation edit") };
        assert_eq!(action.action, "editElevation");
        assert_eq!(action.controller_id, state.session.controller_id);
        let args = serde_json::to_value(action.args.as_ref().unwrap()).unwrap();
        assert_eq!(args["windowId"], "gis-terrain-window");
        assert_eq!(args["surfaceId"].as_str(), Some(state.session.surface_id.as_str()));
        assert_eq!(args["operation"], "raise-lower");
        assert_eq!(args["radius"], law["radius"]);
        assert_eq!(args["value"], law["value"]);
        assert_eq!(args["column"], case["start"][0]);
        assert_eq!(args["row"], case["start"][1]);
        assert_eq!(args["stroke"], case["stroke"]);
    }
}

#[test]
fn native_terrain_position_drag_reuses_the_editor_action() {
    use semio_framework_test_fixtures::repo_text;
    use ui_scene::math::{projection_spec_project_point, projection_spec_view_proj, Vec3};
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let fixture: serde_json::Value = serde_json::from_str(repo_text!("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🗺️overlay/🧪️tests/🧫️fixtures/🖱️move-position.json")).unwrap();
    let grid: serde_json::Value = serde_json::from_str(repo_text!("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🧪️tests/🧫️fixtures/📦️elevation-grid/🔣️.json")).unwrap();
    let [west, south, east, north]: [f64; 4] = serde_json::from_value(grid["bounds"].clone()).unwrap();
    let [lon0, lat0]: [f64; 2] = serde_json::from_value(grid["origin"].clone()).unwrap();
    let size = grid["samples"]["size"].as_u64().unwrap() as usize;
    let modifiers = PointerModifiers::default();
    for case in fixture["cases"].as_array().unwrap() {
        let mut state = journey_state();
        state.session.bounds = Rect::new(0.0, 0.0, 1000.0, 1000.0);
        state.session.pick_bounds = state.session.bounds;
        let interaction = serde_json::json!({"terrainMovePosition": fixture["descriptor"], "activeUtility": "terrainMovePosition"});
        state.terrain.set_terrain_interaction(Some(&interaction.to_string()));
        let samples = &grid["samples"];
        let number = |name: &str| samples[name].as_i64().unwrap();
        let mut rows: Vec<Vec<i64>> = (0..size).map(|row| (0..size).map(|column|
            1000 * (number("southwest") + number("northwardStep") * row as i64 + number("eastwardStep") * column as i64)
        ).collect()).collect();
        let raised = &grid["pick"]["raisedSample"];
        rows[raised["row"].as_u64().unwrap() as usize][raised["column"].as_u64().unwrap() as usize] += 1000 * raised["meters"].as_i64().unwrap();
        let mut elevation_grid = serde_json::json!({"bounds": grid["bounds"], "rowsMillimetres": rows});
        if let Ok([column, row]) = serde_json::from_value::<[usize; 2]>(case["missingSample"].clone()) {
            elevation_grid["rowsMillimetres"][row][column] = 0.into();
            let ordinal = row * size + column;
            let mut bytes = vec![0u8; (size * size).div_ceil(8)];
            bytes[ordinal / 8] = 1 << (ordinal % 8);
            elevation_grid["missingSamples"] = bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>().into();
        }
        let descriptor = serde_json::json!({
            "projectOriginLon": lon0, "projectOriginLat": lat0, "exaggeration": grid["exaggeration"],
            "elevationGrid": elevation_grid,
            "overlays": {"positions": [
                {"id": "rear", "name": "Rear", "coordinates": fixture["marker"]["coordinates"]},
                {"id": fixture["marker"]["id"], "name": fixture["marker"]["label"], "coordinates": fixture["marker"]["coordinates"], "elevation": fixture["marker"]["elevation"]}
            ], "routes": [], "regions": []}
        });
        state.terrain.set_terrain_json(Some(&descriptor.to_string()));
        semio_os_infinite_world_scene::apply_terrain_style_if_changed_state(&mut state.session, &mut state.scene, &mut state.terrain);
        state.terrain.terrain_built_tiles.insert((0, 0, 0));
        state.terrain.terrain_visible_tiles.insert((0, 0, 0));
        let marker = state.terrain.tile_overlays((0, 0, 0)).unwrap().positions[1].point;
        let mut camera = state.camera.orbit.to_camera();
        camera.position = Vec3::new(0.0, 0.0, 1000.0);
        camera.target = Vec3::ZERO;
        camera.up = Vec3::new(0.0, 1.0, 0.0);
        camera.zoom = 0.35;
        camera.projection = CameraProjection3d::Orthographic;
        state.camera.orbit = ui_scene::math::OrbitController::from_camera(&camera);
        state.camera.projection_spec = semio_ui_viewport::Viewport3dProjectionSpec {
            mode: semio_ui_viewport::Viewport3dProjectionMode::Orthographic {},
            orientation: semio_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_ui_viewport::Viewport3dOrthographicView::Top },
        };
        let camera = state.camera.orbit.to_camera();
        let spec = state.camera.projection_spec;
        let view = projection_spec_view_proj(&camera, spec, 1000.0, 1000.0);
        let project = |point| projection_spec_project_point(view, spec, point, 1000.0, 1000.0).unwrap();
        let [down_x, down_y] = project(Vec3::new(marker[0], marker[1], marker[2] + 20.0));
        let down = [down_x, down_y - 7.0];
        let ray = &grid["pick"]["rays"][case["rayIndex"].as_u64().unwrap() as usize];
        let [col, row, _]: [f64; 3] = serde_json::from_value(ray["originSample"].clone()).unwrap();
        let lon = west + (east - west) * col / (size - 1) as f64;
        let lat = south + (north - south) * row / (size - 1) as f64;
        let up = if case["noTravel"] == true { down } else {
            project(Vec3::new(((lon - lon0) * 111_320.0 * lat0.to_radians().cos()) as f32, ((lat - lat0) * 111_320.0) as f32, ray["height"].as_f64().unwrap_or(200.0) as f32))
        };
        enqueue(&mut state, WorldInteractionIntent::pointer_button(down[0], down[1], true, 0, &modifiers));
        assert!(journal_actions(&mut state).is_empty());
        if case["cancel"] == true {
            enqueue(&mut state, WorldInteractionIntent::cancel(semio_os_infinite_world_model::WorldCancelReason::CaptureLost));
        }
        if case["stale"] == true {
            state.terrain.set_terrain_json(Some(&format!("{} ", descriptor)));
        }
        enqueue(&mut state, WorldInteractionIntent::pointer_button(up[0], up[1], false, 0, &modifiers));
        let actions = journal_actions(&mut state);
        let expected = &case["action"];
        if expected.is_null() {
            assert!(actions.is_empty(), "{}", case["name"]);
        } else {
            let [action] = actions.as_slice() else { panic!("one terrain position action expected") };
            assert_eq!(action.action, fixture["descriptor"]["action"].as_str().unwrap());
            let args = serde_json::to_value(action.args.as_ref().unwrap()).unwrap();
            assert_eq!(args["id"], expected["id"]);
            for axis in ["longitude", "latitude"] {
                assert!((args[axis].as_f64().unwrap() - expected[axis].as_f64().unwrap()).abs() < 1e-6, "{axis}");
            }
        }
    }
}

#[test]
fn an_orbit_drag_journals_the_reference_sequence_and_moves_no_camera() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let before = state.camera.orbit.clone();
    drag(&mut state, 0, point("dragTo"));
    assert_eq!(journal(&mut state), expected("orbit-drag", "wgpu"));
    assert_eq!(state.camera.orbit.target, before.target, "the LEFT button drives no camera on either renderer");
    assert_eq!(state.camera.orbit.distance, before.distance);
}

#[test]
fn a_pan_drag_journals_one_navigation_note_and_one_debounced_camera_report() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let before = state.camera.orbit.target;
    drag(&mut state, 1, point("panTo"));
    let published = journal(&mut state);
    assert_eq!(published, expected("pan-drag", "wgpu"));
    assert_eq!(published.iter().filter(|action| *action == "setCamera").count(), 1, "eight pointer moves owe ONE report, not eight");
    assert_ne!(state.camera.orbit.target, before);
}

#[test]
fn a_wheel_zoom_journals_exactly_the_reference_sequence() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let at = point("aim");
    let delta = fixture()["surface"]["wheelDelta"].as_f64().expect("wheel delta") as f32;
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::wheel(at[0], at[1], delta, &modifiers));
    let published = journal(&mut state);
    assert_eq!(published, expected("zoom-wheel", "wgpu"));
    assert_eq!(published, expected("zoom-wheel", "react")[..published.len()], "the wheel step is React's own sequence up to the trailing hover only a multi-instance scene can change");
}

#[test]
fn a_pick_journals_one_selection_and_the_unconditional_camera_report() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let before = state.camera.orbit.clone();
    click(&mut state, 0);
    let published = journal(&mut state);
    assert_eq!(published, expected("pick-instance", "wgpu"));
    assert_eq!(state.camera.orbit.distance, before.distance, "a pick moves no camera — and still owes the report every pointer-up owes");
}

#[test]
fn a_right_click_journals_no_menu_verb_at_all() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    click(&mut state, 2);
    let published = journal(&mut state);
    assert_eq!(published, expected("context-menu", "wgpu"));
    assert!(!published.iter().any(|action| action.contains("ontextMenu")), "the menu is the shell's; React deleted its own `contextMenuAt` and this lane deleted `worldContextMenuAt` with it");
}

/// 🖱️🎯️ **The non-primary press selects too.** R3F raises the canvas's `onPointerMissed` for ANY
/// button — on the `pointerdown` that resolved no event-handling object and again on the click — and
/// `🌐️World3dHost/🟦️.tsx`'s `handleEmptyClick` publishes `interactionSelect` from each. A primary
/// press arms the marquee, whose `wasMarqueeDragRef` swallows the first of the two; a middle or right
/// press arms nothing, which is exactly why React's `pan-drag` and `context-menu` journal TWO
/// selections and its `orbit-drag` one.
///
/// 🩸️ This lane published none for either: the middle press retired as a gesture-less intent and the
/// right press only opened the shell's menu, so pressing a pane with anything but the left button
/// never touched the selection (`🗑️generated/w12c-parity-run-19/steps.json` steps 20 and 23,
/// ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W13c). One selection, not two: a second
/// selection of the same target is a duplicate, not parity, and the probe's verdict is set-based.
#[test]
fn a_non_primary_press_publishes_the_selection_react_publishes_for_any_button() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    for button in [1_i16, 2] {
        let mut state = journey_state();
        let at = point("aim");
        let modifiers = PointerModifiers::default();
        enqueue(&mut state, WorldInteractionIntent::pointer_button(at[0], at[1], true, button, &modifiers));
        let published = journal(&mut state);
        assert_eq!(published, ["interactionSelect"], "🎯️ button {button}: the press alone publishes one selection and nothing else");
    }

    let mut state = journey_state();
    drag(&mut state, 1, point("panTo"));
    assert_eq!(journal(&mut state), expected("pan-drag", "wgpu"), "🎯️ and the whole middle drag is React's own sequence");

    let mut state = journey_state();
    click(&mut state, 2);
    assert_eq!(journal(&mut state), expected("context-menu", "wgpu"));
}

/// 🖌️ The press verb belongs to the SELECT lane only. Component mode, the brush utility and vertex
/// granularity each own their own press route (`WorldComponentPickCursor`, `WorldObjectPickCursor`),
/// and a second selection published beside them would be this lane inventing a verb React never
/// sends — `handleEmptyClick` returns early under `paintMode` for the same reason.
#[test]
fn a_non_primary_press_publishes_nothing_outside_the_select_lane() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    for (utility, granularity) in [("brush", "mesh"), ("select", "vertex"), ("surfaceBrush", "mesh")] {
        let mut state = journey_state();
        state.session.active_utility = utility.into();
        state.selection.granularity = granularity.into();
        let at = point("aim");
        let modifiers = PointerModifiers::default();
        enqueue(&mut state, WorldInteractionIntent::pointer_button(at[0], at[1], true, 1, &modifiers));
        assert!(!journal(&mut state).iter().any(|action| action == "interactionSelect"), "🖌️ {utility}/{granularity} owns its own press verbs");
    }
}

/// 🫧 **The ingress fold.** The frame worker drains its input queue far faster than the retained
/// authority answers it, so a burst of moves arrives with nothing stepped in between — measured live
/// as `queued=10` in front of ONE eight-move drag, ten full pick/plan turns for one gesture
/// (`🗑️generated/w12c-parity-run-19/wgpu/console.txt`, ticket 26/09/17 packet W13c §3). React never
/// sees a move per DOM event either: its transport keeps one sample per pointer identity per
/// animation frame, and its hover dispatcher keeps at most one round trip outstanding and coalesces
/// the rest onto the latest target.
///
/// ⚖️ What the fold may NOT do is move the gesture, and this is the law that says so: the same drag
/// is driven twice — once folded, once on a LASSO pane, where `world3d_pointer_move_folds` refuses —
/// and the two journals, their order and the settled camera must be identical while the queue depth
/// is not. `dx`/`dy` accumulate and both camera operations integrate their delta linearly
/// (`OrbitController::pan` along an orientation-only basis, `orbit` straight onto yaw/pitch), and
/// `x`/`y` take the newest sample, which is the point React would have raycast.
#[test]
fn a_burst_of_pointer_moves_folds_onto_the_latest_without_moving_the_gesture() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut folded = journey_state();
    drag(&mut folded, 1, point("panTo"));
    let folded_depth = folded.authority.interaction_authority.as_ref().expect("authority").queue.len;
    let folded_journal = journal(&mut folded);

    let mut whole = journey_state();
    whole.selection.selection_method = "lasso".into();
    assert!(!world3d_pointer_move_folds(&whole.selection), "🫧 a lasso marquee IS its own path, so its samples are never folded");
    drag(&mut whole, 1, point("panTo"));
    let whole_depth = whole.authority.interaction_authority.as_ref().expect("authority").queue.len;
    let whole_journal = journal(&mut whole);

    assert_eq!(folded_journal, expected("coalesced-drag", "wgpu"));
    assert_eq!(folded_journal, whole_journal, "🫧 the fold publishes the same journal in the same order");
    assert_eq!(u32::from(whole_depth) - u32::from(folded_depth), 7, "🫧 eight moves are delivered as one — the fold is the whole point");
    assert!((folded.camera.orbit.target.x - whole.camera.orbit.target.x).abs() < 1e-3 && (folded.camera.orbit.target.y - whole.camera.orbit.target.y).abs() < 1e-3 && (folded.camera.orbit.target.z - whole.camera.orbit.target.z).abs() < 1e-3, "🫧 and the settled camera is the same camera");

    let mut painting = journey_state();
    painting.selection.interaction_mode = "paint".into();
    painting.selection.paint_stroke_active = true;
    assert!(!world3d_pointer_move_folds(&painting.selection), "🫧 a live paint stroke raycasts once per move, so a folded move would leave a gap in the run");
}

/// 🫧 A fold never reaches the FRONT — a live cursor may already be half-stepping it — and never
/// crosses a change of button, of `down` or of the modifier state, each of which is a different
/// gesture.
#[test]
fn a_fold_never_touches_the_front_or_crosses_a_gesture_boundary() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let modifiers = PointerModifiers::default();
    let mut state = journey_state();
    let at = point("aim");
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 4.0, at[1], 4.0, 0.0, false, 0, &modifiers));
    assert_eq!(state.authority.interaction_authority.as_ref().expect("authority").queue.len, 2, "🫧 the only queued move IS the front, so the second one queues behind it");

    let mut state = journey_state();
    enqueue(&mut state, WorldInteractionIntent::pointer_button(at[0], at[1], true, 1, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 4.0, at[1], 4.0, 0.0, true, 1, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 8.0, at[1], 4.0, 0.0, true, 1, &modifiers));
    assert_eq!(state.authority.interaction_authority.as_ref().expect("authority").queue.len, 2, "🫧 two drag moves behind the press fold into one");
    let shifted = PointerModifiers { shift: true, ..PointerModifiers::default() };
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 12.0, at[1], 4.0, 0.0, true, 1, &shifted));
    assert_eq!(state.authority.interaction_authority.as_ref().expect("authority").queue.len, 3, "🫧 a modifier change starts a new intent");
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 16.0, at[1], 4.0, 0.0, false, 1, &shifted));
    assert_eq!(state.authority.interaction_authority.as_ref().expect("authority").queue.len, 4, "🫧 and so does lifting the button");
}

/// 📐️ **The orthographic Top pane answers the same journal.** Pan, wheel and pick are
/// projection-agnostic all the way down: `OrbitController::pan` scales by `1 / zoom` under the
/// parallel frustum and `zoom` scales the frustum instead of dollying, `world3d_camera_zoom` reports
/// that live scale (a perspective pane reports the identity), and the classifier takes React's own
/// orthographic branch — `after.zoom / before.zoom` rather than a dolly ratio
/// (`captureNavigationSnapshot`). So a Top pane journals pan/zoom/pick exactly as the perspective one
/// does, with a `zoom` in the report that actually moves.
#[test]
fn an_orthographic_pane_journals_the_same_sequences_with_a_live_zoom() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    state.camera.orbit.projection = CameraProjection3d::Orthographic;
    state.camera.orbit.zoom = 40.0;
    let before = world_navigation_snapshot(&state.camera);
    assert!(before.parallel && (before.zoom - 40.0).abs() < 1e-6, "a parallel pane reports its live frustum scale, never the perspective identity");
    drag(&mut state, 1, point("panTo"));
    assert_eq!(journal(&mut state), expected("pan-drag", "wgpu"));

    let mut state = journey_state();
    state.camera.orbit.projection = CameraProjection3d::Orthographic;
    state.camera.orbit.zoom = 40.0;
    let at = point("aim");
    let delta = fixture()["surface"]["wheelDelta"].as_f64().expect("wheel delta") as f32;
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::wheel(at[0], at[1], delta, &modifiers));
    assert_eq!(journal(&mut state), expected("zoom-wheel", "wgpu"));
    assert_ne!(state.camera.orbit.zoom, 40.0, "a parallel wheel moves the frustum scale, which is the number its report carries");

    let mut state = journey_state();
    state.camera.orbit.projection = CameraProjection3d::Orthographic;
    state.camera.orbit.zoom = 40.0;
    click(&mut state, 0);
    assert_eq!(journal(&mut state), expected("pick-instance", "wgpu"));
}

/// 🚪️ **The chrome-press law.** A pointer the shell's chrome owns hands this lane exactly one thing:
/// the clear of the hover it has published. No move, no press, no release and no wheel — React's
/// canvas receives none of those while a DOM layer above it is under the pointer, and the only event
/// it does receive is r3f's `onPointerOut`.
///
/// 🩸️ The eight journey steps of `📓️w11a-prepared-world-mesh-missing.md` §6 family A each journalled
/// an `interactionHover` AND an `interactionSelect` the reference never emits, and the stray select
/// carried empty `targets` — pressing a navbar panel tab CLEARED the world selection. The routing
/// that stops it is `🐚️Shell`'s `PointerCapture`/`pointer_owner_at`; what this law pins is the other
/// half: that the leave those two hand down is a hover clear and nothing more.
#[test]
fn a_chrome_owned_pointer_hands_the_world_lane_one_hover_clear_and_nothing_else() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    assert_eq!(journal(&mut state), ["interactionHover"], "🚪️ the pane publishes a hover while the pointer is still its own");
    assert!(world3d_hover_is_published(&state.selection) && state.selection.local_hover_id.is_some());

    let before = state.camera.orbit.clone();
    enqueue(&mut state, WorldInteractionIntent::pointer_leave(at[0], at[1]));
    let published = journal(&mut state);
    assert_eq!(published, expected("chrome-press", "wgpu"), "🚪️ a leave answers the published hover and nothing else");
    assert_eq!(published, expected("chrome-press", "react"), "🚪️ which is exactly what React's `onPointerOut` publishes");
    assert!(!published.iter().any(|action| action == "interactionSelect" || action == "setCamera"), "🚪️ the press and the release never reached this lane, so neither the selection nor the camera moved");
    assert!(state.selection.local_hover_id.is_none() && !world3d_hover_is_published(&state.selection), "🚪️ and the hover is CLEAR, which is what the guest reads");
    assert_eq!(state.camera.orbit.target, before.target);
    assert_eq!(state.camera.orbit.distance, before.distance);

    enqueue(&mut state, WorldInteractionIntent::pointer_leave(at[0], at[1]));
    assert!(journal(&mut state).is_empty(), "🚪️ a leave over an already-clear surface journals nothing at all — React does not re-clear a hover it never had");
}

/// 🚪️ A leave aimed OUTSIDE the surface's own pick rect — the pointer travelled to the navbar, to a
/// sibling pane, off the canvas — is answered, never refused. Every pick cursor is built through
/// `pointer_in_pick_rect`, so a leave that had to run one would fault the frame, which is what
/// `📓️w9c-behaviour-parity-run-2.md` §7b measured a stale aim doing to the whole page.
#[test]
fn a_leave_from_outside_the_pick_rect_clears_the_hover_without_faulting() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    assert_eq!(journal(&mut state), ["interactionHover"]);
    let outside = state.session.bounds.x + state.session.bounds.w + 64.0;
    enqueue(&mut state, WorldInteractionIntent::pointer_leave(outside, 4.0));
    assert_eq!(journal(&mut state), expected("chrome-press", "wgpu"));
    assert!(state.selection.local_hover_id.is_none());
}

/// 🫥️ LAW: a draw naming a mesh the guest has NOT published yet is SKIPPED by both interaction
/// cursors — the registry build and the ray pick — so the pane keeps hovering, picking and reporting
/// its camera exactly as if that draw were not there.
///
/// 🩸️ Both cursors answered an empty probe slot with `WorldInteractionStep::Fault`, which the frame
/// driver turns into `world3d retained interaction authority faulted` and the browser shell into a
/// DEAD PAGE. The guest publishes its draw list and its mesh leases in separate deliveries, so every
/// glb-bearing document has frames where a draw names a lease still landing: measured live at t≈9.4 s
/// on the first pointer move after the introduction tour was dismissed, which killed the page before
/// a single journey step could be measured (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c`,
/// `🗑️generated/w12c-bisect-world/wgpu/console.txt`).
///
/// ⚖️ React has no such failure: a `GlbInstanceMesh` whose loader has not resolved renders nothing and
/// raycasts to nothing. This is the interaction twin of W11a's "a miss is a skipped draw, not a
/// quarantine".
#[test]
fn a_draw_whose_mesh_has_not_landed_is_skipped_by_every_interaction_cursor() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let resident = journal_of_a_click(&mut journey_state());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh:🧊️still-loading".into(), mesh_version: 7, instances: vec![Instance3d { component_source: None, id: "unlanded".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);

    assert_eq!(journal_of_a_click(&mut state), resident, "🫥️ the unpublished draw changes nothing the pane journals");
    assert!(!state.interaction.interaction_meshes.faulted && !state.interaction.interaction_objects.faulted, "🫥️ and nothing is quarantined");
}

/// 🖱️ The probe's own click — one move onto the aim point, a press and a release at the same point —
/// answered to idle.
fn journal_of_a_click(state: &mut World3dState) -> Vec<String> {
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    enqueue(state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(at[0], at[1], true, 0, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(at[0], at[1], false, 0, &modifiers));
    journal(state)
}

/// 🚪️ **The per-FRAME clear is idempotent.** A caller driven by frames, not by pointer events, must
/// ask `world3d_hover_clear_is_owed` and not `world3d_hover_is_published`: the published hover only
/// goes clear when the leave is ANSWERED, so while one waits in the queue the published predicate
/// stays true and the caller enqueues another, every frame.
///
/// 🩸️ Measured live at the journey's `context-menu` step: `queued=64 blocked=true` on
/// `puzzle3d-main-perspective`, the bounded intent queue full of identical leaves, then
/// `bounded frame input action authority faulted` and a dead page — with every step from 23 on
/// answering `renderer exposes no dumpChrome` (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `🗑️generated/w13c-parity-run-20`).
#[test]
fn a_frame_driven_hover_clear_is_owed_once_and_never_saturates_the_queue() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    assert_eq!(journal(&mut state), ["interactionHover"]);
    assert!(world3d_hover_clear_is_owed(&state.selection, &state.authority), "🚪️ a published hover with no leave in flight owes one");

    let outside = [state.session.bounds.x - 1.0, state.session.bounds.y - 1.0];
    for frame in 0..256 {
        if world3d_hover_clear_is_owed(&state.selection, &state.authority) {
            enqueue(&mut state, WorldInteractionIntent::pointer_leave(outside[0], outside[1]));
        }
        let depth = state.authority.interaction_authority.as_ref().expect("authority").queue.len;
        assert!(depth <= 1, "🚪️ frame {frame}: at most ONE leave is ever in flight, never a queue full of them (depth {depth})");
    }
    assert!(
        !world3d_hover_clear_is_owed(&state.selection, &state.authority) && world3d_hover_is_published(&state.selection),
        "🚪️ nothing more is owed while the leave waits, although the hover is still PUBLISHED — which is exactly why the published predicate cannot gate a frame-driven caller"
    );

    assert_eq!(journal(&mut state), expected("modal-open", "wgpu"), "🚪️ and answering it publishes React's own single hover clear");
    assert!(!world3d_hover_clear_is_owed(&state.selection, &state.authority), "🚪️ after which nothing is owed and no later frame enqueues anything");
    assert_eq!(state.authority.interaction_authority.as_ref().expect("authority").queue.len, 0);
}

#[test]
fn leaving_the_viewport_reveals_the_graph_hover_after_a_local_miss() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    state.selection.scene_hovered_ids = Some(["graph@geometry#0".into()].into_iter().collect());
    state.selection.local_instance_hover = true;
    let outside = state.session.bounds.x + state.session.bounds.w + 64.0;
    enqueue(&mut state, WorldInteractionIntent::pointer_leave(outside, 4.0));
    assert!(journal(&mut state).is_empty());
    assert!(state.selection.projected_instance_hovers().unwrap().contains("graph@geometry#0"));

    let at = point("aim");
    let modifiers = PointerModifiers::default();
    state.session.window_instance_id = Some("preview:1".into());
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    let [hit]: [ActionDescriptor; 1] = journal_actions(&mut state).try_into().expect("one hover action");
    assert_eq!(hit.args.unwrap()["windowId"].as_str(), Some("preview:1"));
    enqueue(&mut state, WorldInteractionIntent::pointer_leave(outside, 4.0));
    let [leave]: [ActionDescriptor; 1] = journal_actions(&mut state).try_into().expect("one clear action");
    let args = leave.args.unwrap();
    assert_eq!(args["windowId"].as_str(), Some("preview:1"));
    assert_eq!(args["targets"].as_str(), Some("[]"));
    assert!(state.selection.projected_instance_hovers().unwrap().contains("graph@geometry#0"));
}

#[test]
fn leaving_before_a_queued_ray_finishes_still_clears_its_hover() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let mut state = journey_state();
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    for _ in 0..2 {
        enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
        assert!(world3d_hover_clear_is_owed(&state.selection, &state.authority));
        enqueue(&mut state, WorldInteractionIntent::pointer_leave(-1.0, -1.0));
        assert!(!world3d_hover_clear_is_owed(&state.selection, &state.authority));
    }
    let result = journal_actions(&mut state);
    assert_eq!(result.len(), 4);
    for (index, action) in result.iter().enumerate() {
        assert_eq!(action.action, "interactionHover");
        assert_eq!(action.args.as_ref().unwrap()["targets"].as_str() == Some("[]"), index % 2 == 1);
    }
}
