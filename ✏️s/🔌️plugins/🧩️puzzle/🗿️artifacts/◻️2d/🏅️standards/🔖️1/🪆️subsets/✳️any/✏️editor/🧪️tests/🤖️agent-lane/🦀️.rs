//! 🤖️ The puzzle 2d editor's own tool-command job (`board.ports.directed.v1.tool-command.v1`) on the agent lane: what the semio MCP gateway's
//! `action_prepare` previews for the verbs its coverage battery reaches on a puzzle 2d document.

use crate::editor::puzzle2d::window;
#[allow(unused_imports)]
use crate::editor::puzzle2d::snapshot::{puzzle2d_scene_for, puzzle2d_scene_with, PUZZLE2D_GENERIC_TOOL_IDS, PUZZLE2D_RETAINED_TOOL_IDS, PUZZLE2D_ADD_NODE_DIALOG_ID, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, Puzzle2dInteractionSnapshot, Puzzle2dScene, puzzle2d_active_utility, puzzle2d_addresses_locked_entity, puzzle2d_entity_hidden, puzzle2d_entity_label, puzzle2d_interaction_definition, puzzle2d_node_kind_arg, puzzle2d_select_scope, puzzle2d_selected_node_ids, puzzle2d_selection_is_locked, puzzle2d_window_and_engagements_scope, puzzle2d_window_only_scope};
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::board_host::drain_board_events_json;
#[allow(unused_imports)]
use crate::editor::puzzle2d::recorder::{puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from, puzzle2d_paste_operations_on};
#[allow(unused_imports)]
use crate::editor::puzzle2d::commands::set_active_example;
#[allow(unused_imports)]
use crate::editor::puzzle2d::config::{Puzzle2dConfig, Puzzle2dConfigMutation};
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::{BoardHost, Point};
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::tools::fill;
#[allow(unused_imports)]
use crate::editor::puzzle2d::precompute::fill as fill_run;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::{area_brush as area_brush_utility, brush as brush_utility, select as select_utility};
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
#[allow(unused_imports)]
use crate::editor::puzzle2d::panels::{artifact, catalogue, inspection, settings};
#[allow(unused_imports)]
use crate::editor::puzzle2d::presence::{Puzzle2dPresence, Puzzle2dPresenceMutation};
#[allow(unused_imports)]
use crate::editor::puzzle2d::window::{Puzzle2dWindowConfig, Puzzle2dWindowTransient};
#[allow(unused_imports)]
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
#[allow(unused_imports)]
use semio_framework::kernel::UiDirtyScope;
#[allow(unused_imports)]
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::ActionRef;
#[allow(unused_imports)]
use semio_framework_plugin::AppIo;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactEditor;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactPresentation;
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
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DialogDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionTarget;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionVerb;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MediaClass;
#[allow(unused_imports)]
use semio_framework_plugin::MediaForm;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortDirection;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortSpec;
#[allow(unused_imports)]
use semio_framework_plugin::MediaType;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::PortMultiplicity;
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
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_pack_json::{json, Value};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet};
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest};
#[allow(unused_imports)]
use crate::editor::puzzle2d::kit::*;
#[allow(unused_imports)]
use crate::editor::puzzle2d::retained::*;
#[allow(unused_imports)]
use crate::editor::puzzle2d::session::*;
use crate::editor::puzzle2d::*;
use semio_framework_plugin::artifact_app_laws::{declared_verb_agent_divergences, declared_verb_probe, declared_verb_wrote_document, probe_agent_lane, DeclaredVerbOutcome};

/// 🤖️ The verbs the semio MCP coverage battery (`plugin-coverage-check`) tries on a fresh puzzle 2d document, in its order —
/// every one was refused `interactive-job.preview-unsupported` before the agent lane ran the editor's own job.
const BATTERY_VERBS: &[&str] = &["addNode", "rotateSelection", "scaleSelection", "translateSelection", "createEdge", "focusSelection", "redrawHandles", "relocateTargetRegion"];

/// 🧭️ The battery verbs whose agent lane diverges from the shell lane on the boot example, pinned so a change turns this law red:
/// nothing is selected there, so the shell answers each with a notice alone and the agent lane refuses it
/// `app.command.no-effect` (the notice its structured cause) — neither lane changes the document.
const AGENT_LANE_DIVERGENCES: &[&str] = &["createEdge"];

/// ⚖️ LAW: the agent lane runs the editor's own retained puzzle job for every battery verb — nothing refuses one before its
/// job ran — `addNode` previews the document write the shell publishes, and the agent lane diverges from the shell lane on
/// exactly [`AGENT_LANE_DIVERGENCES`]: the fail-closed preview refuses by name what a shell run publishes beyond the
/// transaction, never settles without it.
#[semio_framework_async_macros::async_test]
async fn the_agent_lane_runs_the_puzzle2d_editors_own_tool_command_job() {
    let probes = probe_agent_lane::<semio_framework_plugin::EditorApp<Puzzle2dPlayApp>, <Puzzle2dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(create_puzzle2d_app, None, BATTERY_VERBS, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    let outcomes = probes.iter().map(|probe| format!("{}: shell {:?} / agent {:?}", probe.verb, probe.windows.iter().map(|window| &window.staged).collect::<Vec<_>>(), probe.agent)).collect::<Vec<_>>();
    for probe in &probes {
        assert!(!matches!(&probe.agent, Some(DeclaredVerbOutcome::Unreachable { code, .. }) if code != "interactive-job.agent-lane-uncarried"), "{} is refused before its job ran: {outcomes:#?}", probe.verb);
    }
    let written = declared_verb_probe(&probes, "addNode");
    assert!(declared_verb_wrote_document(written.agent.as_ref().expect("addNode is agent-facing")), "addNode previews its document write: {outcomes:#?}");
    assert_eq!(declared_verb_agent_divergences(&probes), AGENT_LANE_DIVERGENCES, "agent-lane divergences: {outcomes:#?}");
    for verb in AGENT_LANE_DIVERGENCES {
        assert!(matches!(&declared_verb_probe(&probes, verb).agent, Some(DeclaredVerbOutcome::Refused { code, .. }) if code == "app.command.no-effect"), "{verb} is refused as no effect: {outcomes:#?}");
    }
}
