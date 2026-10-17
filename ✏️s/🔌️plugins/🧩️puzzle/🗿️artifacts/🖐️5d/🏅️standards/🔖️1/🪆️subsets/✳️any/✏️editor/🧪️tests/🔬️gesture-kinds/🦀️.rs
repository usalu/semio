//! 🧭️ Host gesture protocol laws for the 5d editor: gestures (verbs with ids and values) become the concrete mutation
//! kinds they consist of, built from the typed base alone; every law replays the kinds forward through the central
//! applier and the recorded inverse rows last-to-first, and requires the base back.

use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;
use crate::editor::puzzle5d::commands::{delete_selection, set_selection_flag};
use crate::editor::puzzle5d::commands::add_target_volume;
#[allow(unused_imports)]
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{apply_sun, cycle_brush_candidate, duplicate_selection, engagement_abort, engagement_control_select, engagement_input, engagement_repeat_last, engagement_submit, register_brush_mesh, rotate_selection, scale_selection, select_same_kind, set_brush_placement_contact_tolerance, set_camera, set_camera_2d, set_camera_3d, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_kind_weight, set_lod_mode, set_suggestion_offset, target_brush_suggestions, translate_selection, world_relocate};
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
use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dRuntime, Puzzle5dConfigSetObjectKindWeights, Puzzle5dConfigSetVortexKindWeights};
#[allow(unused_imports)]
use crate::editor::puzzle5d::modes::edit;
#[allow(unused_imports)]
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
#[allow(unused_imports)]
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
#[allow(unused_imports)]
use crate::editor::puzzle5d::panels::{catalogue, artifact as artifact_panel, inspection, settings as settings_panel};
#[allow(unused_imports)]
use semio_s_artifact_puzzle_3d_editor::Puzzle3dInstanceOperationOwner;
#[allow(unused_imports)]
use crate::editor::puzzle5d::presence::{Puzzle5dPresence, Puzzle5dPresenceMutation};
#[allow(unused_imports)]
use crate::editor::puzzle5d::terminology::{puzzle5d_labels, puzzle5d_localized, Puzzle5dLabels};
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
use semio_framework_plugin::ArtifactEditor;
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
use semio_framework_pack_json::{parse, Value};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::sync::LazyLock;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
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
use crate::editor::puzzle5d::*;
use crate::standards::v1::subsets::any::schema::mutations::{change_part_2d_hidden, change_part_2d_locked, delete_part, disconnect_grips, remove_part_grip};

fn base() -> Puzzle5dSnapshot {
    let part = |id: &str, x: f64| serde_json::json!({ "id": id, "2d": { "x": x, "y": 0.0 }, "3d": { "origin": [x, 0.0, 0.0] }, "grips": [{ "id": "g0" }] });
    let value = serde_json::json!({
        "schema": crate::PUZZLE_5D_SCHEMA, "domain": "architecture", "meta": { "description": "" },
        "parts": [part("a", 0.0), part("b", 1.0), part("c", 2.0)],
        "fasteners": [{ "id": "f1", "source": "a:g0", "target": "b:g0" }, { "id": "f2", "source": "b:g0", "target": "c:g0" }],
    });
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed fixture")
}

fn replay(before: &Puzzle5dSnapshot, operations: &[Puzzle5dMutation]) -> (Puzzle5dSnapshot, Puzzle5dSnapshot) {
    let mut state = before.clone();
    let mut inverses = Vec::new();
    for operation in operations {
        inverses.extend(protocol::Mutation::<Puzzle5dSnapshot>::inverse(operation, &state).expect("concrete inverse"));
        state = protocol::apply_diff(protocol::Mutation::<Puzzle5dSnapshot>::diff(operation, &state).diff(), &state).expect("central apply");
    }
    let after = state.clone();
    for inverse in inverses.iter().rev() {
        state = protocol::apply_diff(protocol::Mutation::<Puzzle5dSnapshot>::diff(inverse, &state).diff(), &state).expect("central inverse apply");
    }
    (after, state)
}

#[test]
fn a_delete_gesture_on_a_part_is_one_delete_part_and_its_fasteners_ride_the_cascade() {
    let before = base();
    let operations = delete_selection::delete_selection_mutations(&before, &["b".to_string()], &[], &["f1".to_string()]);
    assert_eq!(operations, vec![delete_part("b".into())], "f1 is severed by deleting b, so it names no row of its own");
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.parts.len(), 2);
    assert!(after.fasteners.is_empty());
    assert_eq!(restored, before);
}

#[test]
fn a_delete_gesture_on_a_fastener_or_a_grip_is_its_own_concrete_kind() {
    let before = base();
    let fastener = delete_selection::delete_selection_mutations(&before, &[], &[], &["f2".to_string()]);
    assert_eq!(fastener, vec![disconnect_grips("f2".into())]);
    assert_eq!(replay(&before, &fastener).1, before);
    let grip = delete_selection::delete_selection_mutations(&before, &[], &["c:g0".to_string()], &[]);
    assert_eq!(grip, vec![remove_part_grip("c".into(), "g0".into())]);
    let (after, restored) = replay(&before, &grip);
    assert!(after.parts[2].grips.is_empty());
    assert_eq!(after.fasteners.len(), 1, "the removed grip's fastener is severed with it");
    assert_eq!(restored, before);
}

#[test]
fn a_flag_gesture_emits_one_row_per_part_whose_flag_differs() {
    let before = base();
    let hide = set_selection_flag::selection_flag_mutations(&before, PUZZLE5D_GRANULARITY_PART, &["a".to_string(), "b".to_string()], "hidden", true);
    assert_eq!(hide, vec![change_part_2d_hidden("a".into(), Some(true)), change_part_2d_hidden("b".into(), Some(true))]);
    let (after, restored) = replay(&before, &hide);
    assert_eq!(after.parts[0].part_2d.hidden, Some(true));
    assert_eq!(restored, before);
    assert!(set_selection_flag::selection_flag_mutations(&after, PUZZLE5D_GRANULARITY_PART, &["a".to_string()], "hidden", true).is_empty());
    assert_eq!(set_selection_flag::selection_flag_mutations(&before, PUZZLE5D_GRANULARITY_PART, &["c".to_string()], "locked", true), vec![change_part_2d_locked("c".into(), Some(true))]);
    assert!(set_selection_flag::selection_flag_mutations(&before, "fastener", &["f1".to_string()], "hidden", true).is_empty());
}

#[test]
fn a_cut_gesture_disconnects_before_it_deletes_and_undo_restores_the_document() {
    let before = base();
    let snapshot = Puzzle5dPlaySnapshot::new(before.clone());
    let operations = puzzle5d_cut_operations(&snapshot, &["a".to_string(), "b".to_string()], &[]);
    assert_eq!(operations, vec![disconnect_grips("f1".into()), delete_part("a".into()), delete_part("b".into())]);
    let (after, restored) = replay(&before, &operations);
    assert_eq!(after.parts.len(), 1);
    assert_eq!(restored, before);
}

#[test]
fn a_paste_gesture_is_a_create_part_per_clone_then_a_connect_grips_per_fastener() {
    let before = base();
    let document = puzzle5d_document_from_snapshot(&before).expect("host projection");
    let (parts, fasteners) = copy_selection_local(&document, &["a".to_string(), "b".to_string()], &[]);
    let (fresh_parts, fresh_fasteners) = paste_selection_local(&document, &parts, &fasteners, (48.0, 24.0));
    let operations = puzzle5d_paste_mutations(fresh_parts, fresh_fasteners).expect("materialized paste");
    assert_eq!(operations.len(), 3);
    assert!(matches!(operations[0], Puzzle5dMutation::CreatePart(_)) && matches!(operations[1], Puzzle5dMutation::CreatePart(_)) && matches!(operations[2], Puzzle5dMutation::ConnectGrips(_)), "{operations:?}");
    let (after, restored) = replay(&before, &operations);
    assert_eq!((after.parts.len(), after.fasteners.len()), (5, 3));
    assert_eq!(restored, before);
}

#[test]
fn a_painted_volume_is_one_create_target_volume_and_a_flag_gesture_on_it_is_one_change_row() {
    let before = base();
    let create = crate::editor::puzzle5d::commands::add_target_volume::add_target_volume_mutation("target-volume-1".into(), [1.1, 2.0, 2.9], 1.0, [2, 2, 2]);
    let (after, restored) = replay(&before, std::slice::from_ref(&create));
    assert_eq!(after.target_volumes.len(), 1);
    assert_eq!(restored, before);
    let hide = crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden("target-volume-1".into(), true);
    let (hidden, restored) = replay(&after, &[hide]);
    assert!(hidden.target_volumes[0].hidden);
    assert_eq!(restored, after);
}
