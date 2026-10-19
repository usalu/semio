use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;
use semio_framework_pack_json::{parse, Value};
#[allow(unused_imports)]
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{apply_sun, cycle_brush_candidate, delete_selection, duplicate_selection, engagement_abort, engagement_control_select, engagement_input, engagement_repeat_last, engagement_submit, register_brush_mesh, rotate_selection, scale_selection, select_same_kind, set_brush_placement_contact_tolerance, set_camera, set_camera_2d, set_camera_3d, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_kind_weight, set_lod_mode, set_selection_flag, set_suggestion_offset, target_brush_suggestions, translate_selection, world_relocate};
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{set_grid_spacing, set_grid_visible, set_grip_direction, set_grip_show, set_lod_automatic, set_lod_depth_variable, set_lod_manual, set_projection, set_selectable_kind, set_transform_gumball_flag};
#[allow(unused_imports)]
use crate::editor::puzzle5d::commands::{add_target_volume, delete_target_volume, relocate_target_volume, set_target_volume_flag, set_voxel_dims};
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
use crate::editor::puzzle5d::*;
use semio_framework_test_fixtures::fixture_text;

#[test]
fn puzzle5d_host_admission_neutral_projection_and_granular_inverse_match_independent_json() {
    let text = fixture_text!("../../🧫️fixtures/🔣️.json");
    let corpus = parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
    let independent: serde_json::Value = serde_json::from_str(text).expect("independent neutral JSON");
    let admit = |name: &str| {
        let mut document: Puzzle5dDocument = puzzle5d_record_from_projection(corpus[name].clone()).expect("host record");
        document.schema = PUZZLE5D_SCHEMA.into();
        puzzle5d_snapshot_from_document(&document).expect("typed admission")
    };
    let before = admit("before");
    let after = admit("after");
    assert_eq!(after.parts[0].part_2d.x, independent["after"]["parts"][0]["2d"]["x"].as_f64().expect("independent coordinate"));
    use crate::standards::v1::subsets::any::schema::mutations::create_part;
    use crate::standards::v1::subsets::any::schema::mutations::delete_part;
    use crate::standards::v1::subsets::any::schema::mutations::move_part_2d;
    use crate::standards::v1::subsets::any::schema::mutations::move_part_3d;
    let operations = vec![delete_part("p1".into()), move_part_2d("p2".into(), 9.0, 0.0), move_part_3d("p2".into(), [9.0, 0.0, 0.0]), create_part(after.parts[1].clone(), None)];
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::CreatePart(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::DeletePart(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::MovePart2d(_))));
    let mut state = before.clone();
    let mut inverses = Vec::new();
    for operation in &operations {
        inverses.extend(protocol::Mutation::<Puzzle5dSnapshot>::inverse(operation, &state).expect("typed inverse"));
        state = protocol::apply_diff_cold(protocol::Mutation::<Puzzle5dSnapshot>::diff(operation, &state).diff(), &state).expect("typed intent");
    }
    assert_eq!(state, after);
    for inverse in inverses.iter().rev() {
        state = protocol::apply_diff_cold(protocol::Mutation::<Puzzle5dSnapshot>::diff(inverse, &state).diff(), &state).expect("typed inverse intent");
    }
    assert_eq!(state, before);
    let projected = puzzle5d_document_from_snapshot(&after).expect("owned native projection");
    assert_eq!(puzzle5d_snapshot_from_document(&projected).expect("readmitted"), after);
}

#[test]
fn puzzle5d_host_admission_refuses_wrong_schema_and_malformed_owned_inputs() {
    let corpus = parse(fixture_text!("../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral refusal fixture");
    let mut document = empty_document();
    document.schema = corpus["refusedSchema"].as_str().expect("neutral schema").into();
    assert!(puzzle5d_snapshot_from_document(&document).is_err());
    assert!(puzzle5d_paste_placement(&semio_framework_pack_json::json!({"anchor":corpus["refusedAnchor"].clone()})).is_err());
    assert!(puzzle5d_paste_placement(&semio_framework_pack_json::json!({"position":corpus["refusedPosition"].clone()})).is_err());
    document.schema = PUZZLE5D_SCHEMA.into();
    document.meta = Some(corpus["refusedMeta"].clone());
    assert!(puzzle5d_snapshot_from_document(&document).is_err());
    document.meta = None;
    document.kind_catalogs = Some(corpus["refusedCatalogs"].clone());
    assert!(puzzle5d_snapshot_from_document(&document).is_err());
    document.kind_catalogs = None;
    let mut part: Puzzle5dPart = puzzle5d_record_from_projection(semio_framework_pack_json::json!({"id":"p1","partKind":"kind"})).expect("host part");
    part.part_3d.scale = Some(corpus["refusedScale"].clone());
    document.parts.push(part);
    assert!(puzzle5d_snapshot_from_document(&document).is_err());
}
