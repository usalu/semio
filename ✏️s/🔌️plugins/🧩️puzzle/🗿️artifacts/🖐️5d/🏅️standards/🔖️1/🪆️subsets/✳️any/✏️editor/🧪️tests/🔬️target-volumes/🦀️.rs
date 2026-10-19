//! ⌨️ 5d editor used to lack entirely. Every law drives the REAL app through the same typed-command
//! 🧊️ Laws for the target-volume family and the Volume Brush — the one fill-adjacent 3d feature the
//! channel the shell uses, so a verb that is registered but unreachable fails here.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::editor::puzzle5d::commands::add_target_volume;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::editor::puzzle5d::terminology::puzzle5d_labels;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_pack_json::Value;
#[allow(unused_imports)]
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
#[allow(unused_imports)]
use crate::Puzzle5dSnapshot;
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{apply_sun, cycle_brush_candidate, delete_selection, duplicate_selection, engagement_abort, engagement_control_select, engagement_input, engagement_repeat_last, engagement_submit, register_brush_mesh, rotate_selection, scale_selection, select_same_kind, set_brush_placement_contact_tolerance, set_camera, set_camera_2d, set_camera_3d, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_kind_weight, set_lod_mode, set_selection_flag, set_suggestion_offset, target_brush_suggestions, translate_selection, world_relocate};
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{set_grid_spacing, set_grid_visible, set_grip_direction, set_grip_show, set_lod_automatic, set_lod_depth_variable, set_lod_manual, set_projection, set_selectable_kind, set_transform_gumball_flag};
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{delete_target_volume, relocate_target_volume, set_target_volume_flag, set_voxel_dims};
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{set_chunk_size, set_proximity_radius};
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{accept_suggestion, close_vortex_suggestions, hover_suggestion, open_vortex_suggestions};
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{export_snapshot, import_snapshot, open_add_part_dialog, open_import_snapshot};
#[allow(unused_imports)]
use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dConfigSetObjectKindWeights, Puzzle5dConfigSetVortexKindWeights};
#[allow(unused_imports)]
use crate::editor::puzzle5d::modes::edit;
#[allow(unused_imports)]
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
#[allow(unused_imports)]
use crate::editor::puzzle5d::panels::{catalogue, artifact as artifact_panel, inspection, settings as settings_panel};
#[allow(unused_imports)]
use semio_s_artifact_puzzle_3d_editor::Puzzle3dInstanceOperationOwner;
#[allow(unused_imports)]
use crate::editor::puzzle5d::presence::{Puzzle5dPresence, Puzzle5dPresenceMutation};
#[allow(unused_imports)]
use crate::editor::puzzle5d::terminology::{puzzle5d_localized, Puzzle5dLabels};
#[allow(unused_imports)]
use crate::editor::puzzle5d::window as window_ownership;
#[allow(unused_imports)]
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PasteAnchor, PastePlacement, UiDirtyScope};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDescriptor;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::AppIo;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactPresentation;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactReservedJob;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactReservedToolInput;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactReservedToolJob;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactReservedToolJobRequest;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolCompletion;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::EphemeralEmit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::DialogDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::GranularityDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::HoverSpec;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionTarget;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionWrite;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::Media;
#[allow(unused_imports)]
use semio_framework_plugin::MediaClass;
#[allow(unused_imports)]
use semio_framework_plugin::MediaError;
#[allow(unused_imports)]
use semio_framework_plugin::MediaForm;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortDirection;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortSpec;
#[allow(unused_imports)]
use semio_framework_plugin::MediaType;
#[allow(unused_imports)]
use semio_framework_plugin::MergeMode;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::PortMultiplicity;
#[allow(unused_imports)]
use semio_framework_plugin::ToolRef;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMethod;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMode;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionSpec;
#[allow(unused_imports)]
use semio_framework_plugin::ToolExecutionContract;
#[allow(unused_imports)]
use semio_framework_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagement;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
#[allow(unused_imports)]
use semio_framework_job::{InteractiveJob, Operation, StepContext};
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_value::{FromValue, ToValue};
#[allow(unused_imports)]
use semio_framework_pack_json::parse;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::sync::LazyLock;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
#[allow(unused_imports)]
use crate::editor::puzzle5d::session::*;
#[allow(unused_imports)]
use crate::editor::puzzle5d::retained::*;
#[allow(unused_imports)]
use crate::editor::puzzle5d::gestures::*;
#[allow(unused_imports)]
use crate::editor::puzzle5d::kit::*;
#[allow(unused_imports)]
use crate::editor::puzzle5d::dispatch::puzzle5d_handle_action;
#[allow(unused_imports)]
use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_inspector_nudge;
use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;

use crate::editor::puzzle5d::*;
use crate::editor::puzzle5d::testkit::*;
use semio_framework_plugin::PluginApp;

fn volumes(app: &Puzzle5dTestApp) -> Vec<semio_framework_pack_json::Value> {
    projection_of(app).get("targetVolumes").and_then(semio_framework_pack_json::Value::as_array).cloned().unwrap_or_default()
}

fn first_volume_id(app: &Puzzle5dTestApp) -> String {
    volumes(app).first().and_then(|volume| volume.get("id")).and_then(semio_framework_pack_json::Value::as_str).expect("a painted target volume").to_string()
}

fn axes(volume: &semio_framework_pack_json::Value, field: &str) -> Vec<f64> {
    volume.get(field).and_then(semio_framework_pack_json::Value::as_array).map(|values| values.iter().map(|value| value.as_f64().unwrap_or(f64::NAN)).collect()).unwrap_or_default()
}

/// 🧊️ Alt+click paints exactly ONE grid-snapped box, sized by the world pane's own voxel dims, and
/// undo takes it away again — the whole Volume Brush gesture as one artifact edit.
#[semio_framework_async_macros::async_test]
async fn add_target_volume_paints_one_grid_snapped_box_and_undo_removes_it() {
    let mut app = app();
    assert!(volumes(&app).is_empty(), "a fresh document constrains nothing");
    dispatch(&mut app, "addTargetVolume", Some(&semio_framework_pack_json::json!({ "origin": [3.4, -2.1, 0.0] })), None).expect("addTargetVolume");
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
        dispatch(&mut app, "setVoxelDims", Some(&semio_framework_pack_json::json!({ "axis": axis, "value": value })), Some(world3d::WINDOW_KIND_ID)).expect("setVoxelDims");
    }
    dispatch(&mut app, "addTargetVolume", Some(&semio_framework_pack_json::json!({ "origin": [0.0, 0.0, 0.0] })), Some(world3d::WINDOW_KIND_ID)).expect("addTargetVolume");
    let spacing = crate::editor::puzzle5d::config::Puzzle5dRuntime::default().grid_spacing;
    let painted = volumes(&app);
    assert_eq!(painted.len(), 1);
    assert_eq!(axes(&painted[0], "scale"), vec![PUZZLE5D_VOXEL_DIM_MAX * spacing, PUZZLE5D_VOXEL_DIM_MIN * spacing, 9.0 * spacing], "the clamped slider values size the paint");
}

/// 🚩️ The outliner's two row set-verbs write exactly one flag each, and the inspector's named-flag verb writes nothing
/// for an unknown flag name.
#[semio_framework_async_macros::async_test]
async fn set_target_volume_flag_writes_one_flag_at_a_time() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&semio_framework_pack_json::json!({ "origin": [0.0, 0.0, 0.0] })), None).expect("addTargetVolume");
    let id = first_volume_id(&app);
    dispatch(&mut app, "setTargetVolumeHidden", Some(&semio_framework_pack_json::json!({ "id": id.as_str(), "hidden": true })), None).expect("hide");
    assert_eq!(volumes(&app)[0].get("hidden").and_then(semio_framework_pack_json::Value::as_bool), Some(true));
    assert_ne!(volumes(&app)[0].get("locked").and_then(semio_framework_pack_json::Value::as_bool), Some(true), "hiding must not also lock");
    dispatch(&mut app, "setTargetVolumeLocked", Some(&semio_framework_pack_json::json!({ "id": id.as_str(), "locked": true })), None).expect("lock");
    assert_eq!(volumes(&app)[0].get("locked").and_then(semio_framework_pack_json::Value::as_bool), Some(true));
    dispatch(&mut app, "setTargetVolumeFlag", Some(&semio_framework_pack_json::json!({ "id": id.as_str(), "flag": "elsewhere", "value": false })), None).expect("unknown flag");
    assert_eq!(volumes(&app)[0].get("hidden").and_then(semio_framework_pack_json::Value::as_bool), Some(true), "an unknown flag name writes nothing");
}

/// 🙈️ LAW: an outliner row's set-verbs are idempotent by value — replaying `{verb}{identity, flag: true}` (a stale view's
/// second click) leaves exactly the first dispatch's ONE document edit and a byte-identical document, for the part verbs
/// and the target-volume verbs alike; and a set-verb without its value is refused at the command boundary, never defaulted.
#[semio_framework_async_macros::async_test]
async fn outliner_set_verbs_are_idempotent_by_value_and_refuse_a_missing_value() {
    fn edits(result: &semio_framework_plugin::InvocationResult) -> usize {
        result.history_patch.as_ref().map_or(0, |patch| patch.upserts.iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).map(|entry| entry.seq).collect::<std::collections::BTreeSet<_>>().len())
    }
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&semio_framework_pack_json::json!({ "origin": [0.0, 0.0, 0.0] })), None).expect("addTargetVolume");
    let volume = first_volume_id(&app);
    let part = first_part_id(&app);
    for (verb, flag, bare, args) in [
        ("setSelectionHidden", "hidden", semio_framework_pack_json::json!({ "entity": PUZZLE5D_GRANULARITY_PART, "ids": [part.as_str()] }), semio_framework_pack_json::json!({ "entity": PUZZLE5D_GRANULARITY_PART, "hidden": true, "ids": [part.as_str()] })),
        ("setSelectionLocked", "locked", semio_framework_pack_json::json!({ "entity": PUZZLE5D_GRANULARITY_PART, "ids": [part.as_str()] }), semio_framework_pack_json::json!({ "entity": PUZZLE5D_GRANULARITY_PART, "ids": [part.as_str()], "locked": true })),
        ("setTargetVolumeHidden", "hidden", semio_framework_pack_json::json!({ "id": volume.as_str() }), semio_framework_pack_json::json!({ "hidden": true, "id": volume.as_str() })),
        ("setTargetVolumeLocked", "locked", semio_framework_pack_json::json!({ "id": volume.as_str() }), semio_framework_pack_json::json!({ "id": volume.as_str(), "locked": true })),
    ] {
        assert!(<Puzzle5dPlayApp as ArtifactEditor>::command_from_action(verb, Some((semio_framework_pack_json::to_dsl_value(&bare)).clone()), None).is_err(), "{verb} without its {flag} value must be refused");
        let first = dispatch(&mut app, verb, Some(&args), None).unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        assert_eq!(edits(&first), 1, "{verb} commits exactly one document edit");
        let set = projection_of(&app);
        let replay = dispatch(&mut app, verb, Some(&args), None).unwrap_or_else(|fault| panic!("{verb} replay: {fault:?}"));
        assert_eq!(edits(&replay), 0, "replaying {verb} with the same value commits no second edit");
        assert_eq!(projection_of(&app), set, "replaying {verb} with the same value leaves the document byte-identical");
    }
}

/// 🚚️ The gumball pushes a whole pose in ONE gesture, and a LOCKED volume refuses the push.
#[semio_framework_async_macros::async_test]
async fn relocate_target_volume_writes_the_whole_pose_and_a_locked_volume_refuses() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&semio_framework_pack_json::json!({ "origin": [0.0, 0.0, 0.0] })), None).expect("addTargetVolume");
    let id = first_volume_id(&app);
    let after = semio_framework_pack_json::json!({ "volumeId": id.as_str(), "after": { "position": [4.0, 5.0, 6.0], "quaternion": [0.0, 0.0, 1.0, 0.0], "scale": [2.0, 3.0, 4.0] } });
    dispatch(&mut app, "relocateTargetVolume", Some(&after), None).expect("relocate");
    let moved = volumes(&app);
    assert_eq!(axes(&moved[0], "origin"), vec![4.0, 5.0, 6.0]);
    assert_eq!(axes(&moved[0], "orientation"), vec![0.0, 0.0, 1.0, 0.0]);
    assert_eq!(axes(&moved[0], "scale"), vec![2.0, 3.0, 4.0]);
    dispatch(&mut app, "setTargetVolumeFlag", Some(&semio_framework_pack_json::json!({ "id": id.as_str(), "flag": "locked", "value": true })), None).expect("lock");
    let refused = semio_framework_pack_json::json!({ "volumeId": id.as_str(), "after": { "position": [9.0, 9.0, 9.0] } });
    dispatch(&mut app, "relocateTargetVolume", Some(&refused), None).expect("a locked volume is a refusal, not a fault");
    assert_eq!(axes(&volumes(&app)[0], "origin"), vec![4.0, 5.0, 6.0], "a locked volume does not move");
}

/// 🪦️ Deleting the addressed volume leaves every other one where it was.
#[semio_framework_async_macros::async_test]
async fn delete_target_volume_removes_only_the_addressed_one() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&semio_framework_pack_json::json!({ "origin": [0.0, 0.0, 0.0] })), None).expect("first");
    dispatch(&mut app, "addTargetVolume", Some(&semio_framework_pack_json::json!({ "origin": [8.0, 0.0, 0.0] })), None).expect("second");
    assert_eq!(volumes(&app).len(), 2);
    let id = first_volume_id(&app);
    dispatch(&mut app, "deleteTargetVolume", Some(&semio_framework_pack_json::json!({ "id": id.as_str() })), None).expect("delete");
    let left = volumes(&app);
    assert_eq!(left.len(), 1);
    assert_ne!(left[0].get("id").and_then(semio_framework_pack_json::Value::as_str), Some(id.as_str()));
}

/// 🖼️ The world pane publishes its volumes on the framework's own `targetVolumesJson` lane, and the
/// board pane publishes their projected flat rectangles — one box, painted in both panes.
#[semio_framework_async_macros::async_test]
async fn both_panes_paint_the_same_volume() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&semio_framework_pack_json::json!({ "origin": [4.0, 2.0, 0.0] })), None).expect("addTargetVolume");
    let id = first_volume_id(&app);
    let envelope = scene_from_projection(&projection_of(&app), crate::editor::puzzle5d::config::Puzzle5dRuntime::default(), PUZZLE5D_DEFAULT_UTILITY);
    let world = world3d::world_target_volumes_json(&envelope.document);
    assert!(world.contains(id.as_str()), "the painted volume must reach the world pane's own target-volume lane, got {world}");
    let board = board2d::puzzle5d_board_scene(&envelope).snapshot_json;
    assert!(board.contains("targetRegions"), "the board scene must carry the projected regions");
    assert!(board.contains(id.as_str()), "the projected rectangle must reach the board scene, got {board}");
}

/// 📐️ The board rectangle is the volume's own footprint under the ONE board↔world map this artifact
/// places paired parts with — never a second persisted pose.
#[test]
fn the_flat_rectangle_is_the_volume_footprint_under_the_shared_board_world_map() {
    let volume = Puzzle5dTargetVolume { id: "volume-1".into(), origin: [2.0, -3.0, 1.0], orientation: None, scale: Some(semio_framework_pack_json::json!([4.0, 6.0, 2.0])), hidden: false, locked: false };
    let [x, y, width, height] = target_volume_flat_rect(&volume);
    let to_flat = 1.0 / PUZZLE5D_FLAT_TO_WORLD;
    assert_eq!([x, y], [2.0 * to_flat, 3.0 * to_flat], "the board's Y axis points the other way, exactly as the flat-position inference reads it");
    assert_eq!([width, height], [4.0 * to_flat, 6.0 * to_flat]);
}

/// 🪣️ The planner the 5d fill run delegates to really receives the constraint: the bridged puzzle 3d
/// snapshot carries every volume the 5d document holds, hidden ones included.
#[test]
fn target_volumes_reach_the_planner_snapshot() {
    let mut document = empty_document();
    document.target_volumes = vec![
        Puzzle5dTargetVolume { id: "volume-1".into(), origin: [1.0, 2.0, 3.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some(semio_framework_pack_json::json!(2.0)), hidden: false, locked: false },
        Puzzle5dTargetVolume { id: "volume-hidden".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(semio_framework_pack_json::json!([1.0, 2.0, 3.0])), hidden: true, locked: true },
    ];
    let snapshot = crate::editor::puzzle5d::precompute::puzzle3d_snapshot(&document, None).expect("planner snapshot");
    let bridged = snapshot.typed().target_volumes.clone();
    assert_eq!(bridged.len(), 2, "a hidden volume still constrains the planner");
    assert_eq!(bridged[0].origin, [1.0, 2.0, 3.0]);
    assert_eq!(bridged[0].scale, Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Uniform(2.0)));
    assert_eq!(bridged[1].scale, Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3([1.0, 2.0, 3.0])));
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
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("labels");
    let measures = world3d::utilities::volume_brush::voxel_dim_measures(&crate::editor::puzzle5d::config::Puzzle5dRuntime::default(), labels);
    assert_eq!(measures.len(), 3, "width, depth and height");
}

/// 🧮️ A painted volume is ONE `create-target-volume` the command arm publishes, grid-snapped and sized by the voxel
/// dimensions — the hinge between the command arm and the artifact lane.
#[test]
fn a_painted_volume_is_one_create_target_volume() {
    let mutation = crate::editor::puzzle5d::commands::add_target_volume::add_target_volume_mutation("volume-1".into(), [1.1, 2.0, 2.9], 1.0, [2, 2, 2]);
    match mutation {
        Puzzle5dMutation::CreateTargetVolume(created) => {
            assert_eq!(created.target_volume.origin, [1.0, 2.0, 3.0]);
            assert_eq!(created.target_volume.scale, Some(crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale::Vec3([2.0, 2.0, 2.0])));
        }
        other => panic!("expected one create-target-volume, got {other:?}"),
    }
}
