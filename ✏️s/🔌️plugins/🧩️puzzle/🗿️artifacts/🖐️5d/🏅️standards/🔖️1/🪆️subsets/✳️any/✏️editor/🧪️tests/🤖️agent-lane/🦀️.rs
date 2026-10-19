//! `action_prepare` previews for the verbs its coverage battery reaches on a puzzle 5d document.
//! 🤖️ The puzzle 5d editor's own tool-command job (`puzzle.5d.tool-command.v1`) on the agent lane: what the semio MCP gateway's

use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
#[allow(unused_imports)]
use crate::Puzzle5dSnapshot;
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
use semio_framework_plugin::artifact_app_laws::declared_verb_agent_divergences;
use semio_framework_plugin::artifact_app_laws::declared_verb_probe;
use semio_framework_plugin::artifact_app_laws::declared_verb_wrote_document;
use semio_framework_plugin::artifact_app_laws::probe_agent_lane;
use semio_framework_plugin::artifact_app_laws::DeclaredVerbOutcome;

/// 🤖️ The verbs the semio MCP coverage battery (`plugin-coverage-check`) tries on a fresh puzzle 5d document, in its order —
/// every one was refused `interactive-job.preview-unsupported` before the agent lane ran the editor's own job.
const BATTERY_VERBS: &[&str] = &["addPartKind", "addBrushPart", "addNode", "rotateSelection", "scaleSelection", "translateSelection", "patchFastener", "patchGrip"];

/// 🧭️ The battery verbs whose agent lane diverges from the shell lane on the boot example, pinned so a change turns this law red:
/// nothing is selected there, so the shell answers each with a notice alone and the agent lane refuses it
/// `app.command.no-effect` (the notice its structured cause) — neither lane changes the document.
const AGENT_LANE_DIVERGENCES: &[&str] = &["translateSelection", "rotateSelection", "scaleSelection"];

/// ⚖️ LAW: the agent lane runs the editor's own retained puzzle job for every battery verb — nothing refuses one before its
/// job ran — `addPartKind` previews the document write the shell publishes, and the agent lane diverges from the shell lane on
/// exactly [`AGENT_LANE_DIVERGENCES`]: the fail-closed preview refuses by name what a shell run publishes beyond the
/// transaction, never settles without it.
#[semio_framework_async_macros::async_test]
async fn the_agent_lane_runs_the_puzzle5d_editors_own_tool_command_job() {
    let probes = semio_framework_plugin::with_authoring_identity!(|identity| probe_agent_lane::<semio_framework_plugin::EditorApp<Puzzle5dPlayApp>, <Puzzle5dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(create_puzzle5d_app, None, BATTERY_VERBS, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut identity).await);
    let outcomes = probes.iter().map(|probe| format!("{}: shell {:?} / agent {:?}", probe.verb, probe.windows.iter().map(|window| &window.staged).collect::<Vec<_>>(), probe.agent)).collect::<Vec<_>>();
    for probe in &probes {
        assert!(!matches!(&probe.agent, Some(DeclaredVerbOutcome::Unreachable { code, .. }) if code != "interactive-job.agent-lane-uncarried"), "{} is refused before its job ran: {outcomes:#?}", probe.verb);
    }
    let written = declared_verb_probe(&probes, "addPartKind");
    assert!(declared_verb_wrote_document(written.agent.as_ref().expect("addPartKind is agent-facing")), "addPartKind previews its document write: {outcomes:#?}");
    assert_eq!(declared_verb_agent_divergences(&probes), AGENT_LANE_DIVERGENCES, "agent-lane divergences: {outcomes:#?}");
    for verb in AGENT_LANE_DIVERGENCES {
        assert!(matches!(&declared_verb_probe(&probes, verb).agent, Some(DeclaredVerbOutcome::Refused { code, .. }) if code == "app.command.no-effect"), "{verb} is refused as no effect: {outcomes:#?}");
    }
}
