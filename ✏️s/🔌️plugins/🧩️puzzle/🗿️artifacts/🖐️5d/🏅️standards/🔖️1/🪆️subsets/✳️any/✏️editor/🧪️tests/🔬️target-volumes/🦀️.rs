//! 🧊️ Laws for the target-volume family and the Volume Brush — the one fill-adjacent 3d feature the
//! 5d editor used to lack entirely. Every law drives the REAL app through the same typed-command
//! channel the shell uses, so a verb that is registered but unreachable fails here.

use super::*;
use crate::editor::puzzle5d::unit_tests::context::*;
use semio_framework_plugin::PluginApp;

fn volumes(app: &Puzzle5dTestApp) -> Vec<dsl::os_pack::json::Value> {
    projection_of(app).get("targetVolumes").and_then(dsl::os_pack::json::Value::as_array).cloned().unwrap_or_default()
}

fn first_volume_id(app: &Puzzle5dTestApp) -> String {
    volumes(app).first().and_then(|volume| volume.get("id")).and_then(dsl::os_pack::json::Value::as_str).expect("a painted target volume").to_string()
}

fn axes(volume: &dsl::os_pack::json::Value, field: &str) -> Vec<f64> {
    volume.get(field).and_then(dsl::os_pack::json::Value::as_array).map(|values| values.iter().map(|value| value.as_f64().unwrap_or(f64::NAN)).collect()).unwrap_or_default()
}

/// 🧊️ Alt+click paints exactly ONE grid-snapped box, sized by the world pane's own voxel dims, and
/// undo takes it away again — the whole Volume Brush gesture as one artifact edit.
#[semio_framework_async_macros::async_test]
async fn add_target_volume_paints_one_grid_snapped_box_and_undo_removes_it() {
    let mut app = app();
    assert!(volumes(&app).is_empty(), "a fresh document constrains nothing");
    dispatch(&mut app, "addTargetVolume", Some(&dsl::json!({ "origin": [3.4, -2.1, 0.0] })), None).expect("addTargetVolume");
    let painted = volumes(&app);
    assert_eq!(painted.len(), 1, "one Alt+click paints exactly one volume");
    let spacing = crate::editor::puzzle5d::config::Puzzle5dRuntime::default().grid_spacing;
    let expected_origin = [3.4, -2.1, 0.0].map(|axis: f64| (axis / spacing).round() * spacing);
    assert_eq!(axes(&painted[0], "origin"), expected_origin.to_vec(), "the painted origin is grid-snapped");
    let [w, d, h] = PUZZLE5D_DEFAULT_VOXEL_DIMS;
    assert_eq!(axes(&painted[0], "scale"), vec![f64::from(w) * spacing, f64::from(d) * spacing, f64::from(h) * spacing], "the painted extent is the voxel extent in grid units");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert!(volumes(&app).is_empty(), "undo removes the whole painted volume");
}

/// 🧯️ A dispatch with no usable origin REFUSES with one localized notice instead of completing
/// silently — the defect that made the 3d Alt+click indistinguishable from a dead gesture.
#[semio_framework_async_macros::async_test]
async fn add_target_volume_without_an_origin_answers_a_notice() {
    let mut app = app();
    let result = dispatch(&mut app, "addTargetVolume", None, None).expect("a missing origin is a refusal, not a fault");
    assert!(volumes(&app).is_empty(), "a refused paint writes no volume");
    assert!(format!("{:?}", result.requested_effects).contains("Notify"), "the refusal is surfaced as a transient notice, got {:?}", result.requested_effects);
}

/// 📐️ Each voxel axis is clamped into the band the world-window config schema declares, and the next
/// paint is sized by what the sliders actually hold.
#[semio_framework_async_macros::async_test]
async fn set_voxel_dims_clamps_each_axis_and_sizes_the_next_paint() {
    let mut app = app();
    for (axis, value) in [("w", 4096.0), ("d", -7.0), ("h", 9.0)] {
        dispatch(&mut app, "setVoxelDims", Some(&dsl::json!({ "axis": axis, "value": value })), Some(world3d::WINDOW_KIND_ID)).expect("setVoxelDims");
    }
    dispatch(&mut app, "addTargetVolume", Some(&dsl::json!({ "origin": [0.0, 0.0, 0.0] })), Some(world3d::WINDOW_KIND_ID)).expect("addTargetVolume");
    let spacing = crate::editor::puzzle5d::config::Puzzle5dRuntime::default().grid_spacing;
    let painted = volumes(&app);
    assert_eq!(painted.len(), 1);
    assert_eq!(axes(&painted[0], "scale"), vec![PUZZLE5D_VOXEL_DIM_MAX * spacing, PUZZLE5D_VOXEL_DIM_MIN * spacing, 9.0 * spacing], "the clamped slider values size the paint");
}

/// 🚩️ The outliner's two row toggles write exactly one flag each, and an unknown flag writes nothing.
#[semio_framework_async_macros::async_test]
async fn set_target_volume_flag_writes_one_flag_at_a_time() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&dsl::json!({ "origin": [0.0, 0.0, 0.0] })), None).expect("addTargetVolume");
    let id = first_volume_id(&app);
    dispatch(&mut app, "setTargetVolumeFlag", Some(&dsl::json!({ "id": id.as_str(), "flag": "hidden", "value": true })), None).expect("hide");
    assert_eq!(volumes(&app)[0].get("hidden").and_then(dsl::os_pack::json::Value::as_bool), Some(true));
    assert_ne!(volumes(&app)[0].get("locked").and_then(dsl::os_pack::json::Value::as_bool), Some(true), "hiding must not also lock");
    dispatch(&mut app, "setTargetVolumeFlag", Some(&dsl::json!({ "id": id.as_str(), "flag": "locked", "value": true })), None).expect("lock");
    assert_eq!(volumes(&app)[0].get("locked").and_then(dsl::os_pack::json::Value::as_bool), Some(true));
    dispatch(&mut app, "setTargetVolumeFlag", Some(&dsl::json!({ "id": id.as_str(), "flag": "elsewhere", "value": false })), None).expect("unknown flag");
    assert_eq!(volumes(&app)[0].get("hidden").and_then(dsl::os_pack::json::Value::as_bool), Some(true), "an unknown flag name writes nothing");
}

/// 🚚️ The gumball pushes a whole pose in ONE gesture, and a LOCKED volume refuses the push.
#[semio_framework_async_macros::async_test]
async fn relocate_target_volume_writes_the_whole_pose_and_a_locked_volume_refuses() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&dsl::json!({ "origin": [0.0, 0.0, 0.0] })), None).expect("addTargetVolume");
    let id = first_volume_id(&app);
    let after = dsl::json!({ "volumeId": id.as_str(), "after": { "position": [4.0, 5.0, 6.0], "quaternion": [0.0, 0.0, 1.0, 0.0], "scale": [2.0, 3.0, 4.0] } });
    dispatch(&mut app, "relocateTargetVolume", Some(&after), None).expect("relocate");
    let moved = volumes(&app);
    assert_eq!(axes(&moved[0], "origin"), vec![4.0, 5.0, 6.0]);
    assert_eq!(axes(&moved[0], "orientation"), vec![0.0, 0.0, 1.0, 0.0]);
    assert_eq!(axes(&moved[0], "scale"), vec![2.0, 3.0, 4.0]);
    dispatch(&mut app, "setTargetVolumeFlag", Some(&dsl::json!({ "id": id.as_str(), "flag": "locked", "value": true })), None).expect("lock");
    let refused = dsl::json!({ "volumeId": id.as_str(), "after": { "position": [9.0, 9.0, 9.0] } });
    dispatch(&mut app, "relocateTargetVolume", Some(&refused), None).expect("a locked volume is a refusal, not a fault");
    assert_eq!(axes(&volumes(&app)[0], "origin"), vec![4.0, 5.0, 6.0], "a locked volume does not move");
}

/// 🪦️ Deleting the addressed volume leaves every other one where it was.
#[semio_framework_async_macros::async_test]
async fn delete_target_volume_removes_only_the_addressed_one() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&dsl::json!({ "origin": [0.0, 0.0, 0.0] })), None).expect("first");
    dispatch(&mut app, "addTargetVolume", Some(&dsl::json!({ "origin": [8.0, 0.0, 0.0] })), None).expect("second");
    assert_eq!(volumes(&app).len(), 2);
    let id = first_volume_id(&app);
    dispatch(&mut app, "deleteTargetVolume", Some(&dsl::json!({ "id": id.as_str() })), None).expect("delete");
    let left = volumes(&app);
    assert_eq!(left.len(), 1);
    assert_ne!(left[0].get("id").and_then(dsl::os_pack::json::Value::as_str), Some(id.as_str()));
}

/// 🖼️ The world pane publishes its volumes on the framework's own `targetVolumesJson` lane, and the
/// board pane publishes their projected flat rectangles — one box, painted in both panes.
#[semio_framework_async_macros::async_test]
async fn both_panes_paint_the_same_volume() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&dsl::json!({ "origin": [4.0, 2.0, 0.0] })), None).expect("addTargetVolume");
    let id = first_volume_id(&app);
    let envelope = scene_from_projection(&projection_of(&app), crate::editor::puzzle5d::config::Puzzle5dRuntime::default(), PUZZLE5D_DEFAULT_UTILITY);
    let world = world3d::world_target_volumes_json(&envelope.document);
    assert!(world.contains(id.as_str()), "the painted volume must reach the world pane's own target-volume lane, got {world}");
    let board = board2d::puzzle5d_board_scene(&envelope).fixture_json;
    assert!(board.contains("targetRegions"), "the board scene must carry the projected regions");
    assert!(board.contains(id.as_str()), "the projected rectangle must reach the board scene, got {board}");
}

/// 📐️ The board rectangle is the volume's own footprint under the ONE board↔world map this artifact
/// places paired parts with — never a second persisted pose.
#[test]
fn the_flat_rectangle_is_the_volume_footprint_under_the_shared_board_world_map() {
    let volume = Puzzle5dTargetVolume { id: "volume-1".into(), origin: [2.0, -3.0, 1.0], orientation: None, scale: Some(serde_json::json!([4.0, 6.0, 2.0])), hidden: false, locked: false };
    let [x, y, width, height] = target_volume_flat_rect(&volume);
    let to_flat = 1.0 / PUZZLE5D_FLAT_TO_WORLD;
    assert_eq!([x, y], [2.0 * to_flat, 3.0 * to_flat], "the board's Y axis points the other way, exactly as add_palette_part reads it");
    assert_eq!([width, height], [4.0 * to_flat, 6.0 * to_flat]);
}

/// 🪣️ The planner the 5d fill run delegates to really receives the constraint: the bridged puzzle 3d
/// snapshot carries every volume the 5d document holds, hidden ones included.
#[test]
fn target_volumes_reach_the_planner_snapshot() {
    let mut document = empty_document();
    document.target_volumes = vec![
        Puzzle5dTargetVolume { id: "volume-1".into(), origin: [1.0, 2.0, 3.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some(serde_json::json!(2.0)), hidden: false, locked: false },
        Puzzle5dTargetVolume { id: "volume-hidden".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(serde_json::json!([1.0, 2.0, 3.0])), hidden: true, locked: true },
    ];
    let snapshot = crate::editor::puzzle5d::precompute::puzzle3d_snapshot(&document, None).expect("planner snapshot");
    let bridged = snapshot.typed().target_volumes.clone();
    assert_eq!(bridged.len(), 2, "a hidden volume still constrains the planner");
    assert_eq!(bridged[0].origin, [1.0, 2.0, 3.0]);
    assert_eq!(bridged[0].scale, Some(semio_s_artifact_puzzle_3d::Puzzle3dScale::Uniform(2.0)));
    assert_eq!(bridged[1].scale, Some(semio_s_artifact_puzzle_3d::Puzzle3dScale::Vec3([1.0, 2.0, 3.0])));
    assert!(bridged[1].hidden && bridged[1].locked);
}

/// 🧰️ The Volume Brush is a real armed utility of the world window, and its Utility Options are the
/// three voxel sliders — the chrome an operator needs to size what Alt+click paints.
#[test]
fn the_volume_brush_is_bound_to_the_world_window_with_its_three_voxel_sliders() {
    let definition = create_puzzle5d_app();
    assert!(definition.utilities.iter().any(|utility| utility.id.as_str() == world3d::utilities::volume_brush::UTILITY_ID), "the app must register the volume brush");
    let bound = |kind: &str| definition.window_kinds.iter().find(|window| window.id == kind).expect("a declared window").utilities.iter().any(|utility| utility.as_str() == world3d::utilities::volume_brush::UTILITY_ID);
    assert!(bound(world3d::WINDOW_KIND_ID), "the world window must bind the volume brush");
    assert!(!bound(board2d::WINDOW_KIND_ID), "the board pane paints projections, it does not paint volumes");
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::default()).expect("labels");
    let measures = world3d::utilities::volume_brush::voxel_dim_measures(&crate::editor::puzzle5d::config::Puzzle5dRuntime::default(), labels);
    assert_eq!(measures.len(), 3, "width, depth and height");
}

/// 🧮️ A painted volume is ONE `create-target-volume` in the semantic delta the editor publishes — the
/// hinge between the command arm (which edits the play document) and the artifact lane.
#[test]
fn a_painted_volume_is_one_create_target_volume_in_the_delta() {
    let before = value_from_document(&empty_document());
    let mut after = empty_document();
    after.target_volumes.push(Puzzle5dTargetVolume { id: "volume-1".into(), origin: [1.0, 2.0, 3.0], orientation: None, scale: Some(serde_json::json!([2.0, 2.0, 2.0])), hidden: false, locked: false });
    let operations = puzzle5d_operations_from_document_change(&before, &after);
    assert_eq!(operations.len(), 1, "expected exactly one create-target-volume, got {operations:?}");
    assert!(matches!(operations[0], Puzzle5dMutation::CreateTargetVolume(_)), "got {operations:?}");
}
