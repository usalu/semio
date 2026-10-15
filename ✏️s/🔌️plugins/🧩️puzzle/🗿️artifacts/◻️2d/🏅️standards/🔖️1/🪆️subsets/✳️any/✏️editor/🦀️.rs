//! 🧩️ Puzzle 2d play app — the plugin's 2d play app: its `ArtifactApp` impl (dispatch-only), the
//! transient `Puzzle2dScene` bundle its command/panel/window nodes mutate and render, the shared
//! snapshot helpers they build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.
//!
//! 📸️ The native play root owns a canonical typed snapshot and a derived first-party host view.
//! Commands publish typed deltas; tool runs append their typed mutations to the operation ledger.

#[path="📸️snapshot/🦀️.rs"]
pub mod snapshot;

#[path="📼️recorder/🦀️.rs"]
pub mod recorder;

use crate::editor::puzzle2d::snapshot::{PUZZLE2D_ADD_NODE_DIALOG_ID, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, PUZZLE2D_PROXIMITY_GESTURE_MAX, Puzzle2dInteractionSnapshot, Puzzle2dScene, apply_host_events, drain_board_events_json, puzzle2d_active_utility, puzzle2d_addresses_locked_entity, puzzle2d_clear_selection_write, puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from, puzzle2d_entity_hidden, puzzle2d_entity_label, puzzle2d_handle_kind, puzzle2d_interaction_definition, puzzle2d_kinds_compatible, puzzle2d_load_document_effect, puzzle2d_node_kind_arg, puzzle2d_occupied_handles, puzzle2d_paste_operations_on, puzzle2d_select_scope, puzzle2d_selected_node_ids, puzzle2d_selection_is_locked, puzzle2d_selection_write, puzzle2d_window_and_engagements_scope, puzzle2d_window_only_scope, sync_host_fixture_content, sync_host_runtime_state};
use crate::editor::puzzle2d::config::PUZZLE2D_LOD_MODE_AUTOMATIC;
use crate::editor::puzzle2d::commands::{
    accept_suggestion, add_node, add_target_region, apply_board_events, close_handle_suggestions, create_edge, cycle_candidate, delete_edge, delete_selection, delete_target_region, duplicate_selection, engagement_abort, engagement_control_select, engagement_input, engagement_repeat_last, engagement_submit,
    export_snapshot, focus_selection, force_layout, hover_suggestion, target_brush_suggestions, import_snapshot, lod_scale_json, open_handle_suggestions, open_import_snapshot, patch_inspector, proximity_connect, relocate_target_region, rotate_selection, scale_selection, select_same_kind, set_active_example,
    set_area_brush_size, set_brush_kind_weights, set_brush_node_size, set_brush_placement_contact_tolerance, set_brush_placement_overlap_budget, set_camera, set_fill_count, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode_for_pane, set_proximity_radius,
    set_selectable_kind, set_selection_flag, set_suggestion_offset, set_target_region_flag, set_transform_gumball_flag, translate_selection,
};
use crate::editor::puzzle2d::config::{Puzzle2dConfig, Puzzle2dConfigMutation, Puzzle2dPlayRuntime};
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::engine::{handle_position_on_circle, handle_position_on_rectangle, BoardHost, Point, Puzzle2dExtension};
use crate::editor::puzzle2d::modes::edit;
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::precompute::fill as fill_run;
pub use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord, SelectToolRequest};
use semio_framework_tool_machine::{ChartGesture, GesturePhase};
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::{area_brush as area_brush_utility, brush as brush_utility, select as select_utility};
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::panels::{artifact, catalogue, inspection, settings};
use crate::editor::puzzle2d::presence::{Puzzle2dPresence, Puzzle2dPresenceMutation};
use crate::editor::puzzle2d::terminology::puzzle2d_labels;
pub use crate::editor::puzzle2d::terminology::{puzzle2d_localized, puzzle2d_localized_phrase};
use crate::editor::puzzle2d::window::{self, Puzzle2dWindowConfig, Puzzle2dWindowTransient};
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;

use crate::standards::v1::subsets::any::schema::mutations::{Puzzle2dMutation};
use crate::editor::puzzle2d::recorder::Puzzle2dRecorder;

use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactPresentation;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
// 🕹️ `InteractionView` — see puzzle3d's identical import comment (missing top-level re-export from
// `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_plugin::app::{EphemeralEmit, InteractionView};
use semio_framework_pack_json::{json, Value};
#[cfg(test)] use semio_framework_pack_json::{Object};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use semio_framework_2d::compute::EngineHandles;
//#region 🔖️Constants
//#endregion 🔖️Constants
//#region 🔖️Scene
//#endregion 🔖️Scene
//#region 🔖️BoardReads
//#region 🎯️TargetRegions
//#endregion 🎯️TargetRegions
//#endregion 🔖️BoardReads
//#region 📋️Clipboard
//#endregion 📋️Clipboard
//#region 🔖️BoardHostSync
//#endregion 🔖️BoardHostSync
//#region 🔖️UiScopes
//#endregion 🔖️UiScopes
//#region 🔖️Puzzle2dCommand
//#endregion 🔖️Puzzle2dCommand
//#region 🧵️ReservedJobs
use semio_framework_job::{InteractiveJob, StepContext};
use semio_framework_plugin::app::ArtifactToolCompletionRejection;
use semio_framework_plugin::{ArtifactReservedJob, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, MediaPayload};
//#endregion 🧵️ReservedJobs

#[path = "🕹️interaction/🦀️.rs"]
mod interaction;
pub use interaction::*;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
use kit::*;

#[path = "🧵️retained/🦀️.rs"]
mod retained;
use retained::*;

#[path = "🖌️session/🦀️.rs"]
mod session;
use session::*;

//#region 🔖️PlayApp
/// 🧩️ Puzzle-2d play app. Owns the `BoardHost` engine; the persisted document (the bare snapshot json)
/// lives in the wrapping `VcsArtifactApp`'s operation store and the view state in `Puzzle2dConfig`.
#[derive(Default, Clone, Copy)]
pub struct Puzzle2dPlayApp;

impl Puzzle2dPlayApp {
    fn scene_for(snapshot: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str) -> Puzzle2dScene {
        Puzzle2dScene { board_snapshot: snapshot, runtime, active_utility: active_utility.into(), interaction: Puzzle2dInteractionSnapshot::default() }
    }

    fn scene_with(snapshot: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str, interaction: Puzzle2dInteractionSnapshot) -> Puzzle2dScene {
        Puzzle2dScene { board_snapshot: snapshot, runtime, active_utility: active_utility.into(), interaction }
    }

    /// 🖼️ ONE render body for both `render` entry points: the window transient and the live selection
    /// are what the request-context path adds; the bare path passes their defaults.
    fn render_body(
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        window_transient: &Puzzle2dWindowTransient,
        interaction: Puzzle2dInteractionSnapshot,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let runtime = window::runtime(cfg.snapshot, &window_config, window_transient, Some(window_kind));
        let active_utility = puzzle2d_active_utility(Some(view_state));
        let snapshot = doc.snapshot.value().clone();
        let document_json = snapshot.to_string();
        let envelope = Self::scene_with(snapshot, runtime, active_utility, interaction);
        let labels = puzzle2d_labels(view_state);
        // 🪟️ One `TreeWindows` per render, read off the host's `ViewModel::tree_windows` for exactly
        // the body being rendered — every panel container below shares its first-paint row budget.
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            overview::BODY_KEY => overview::render(&document_json, &envelope)?,
            detail::BODY_KEY => detail::render(&document_json, &envelope)?,
            selection::BODY_KEY => selection::render(&document_json, &envelope)?,
            artifact::PUZZLE2D_PLAY_BODY_LAYERS => artifact::render(&envelope, labels, &windows)?,
            catalogue::PUZZLE2D_PLAY_BODY_CATALOGUE => catalogue::render(&envelope, labels, &windows)?,
            inspection::PUZZLE2D_PLAY_BODY_PROPERTIES => inspection::render(&envelope, labels, &windows)?,
            settings::PUZZLE2D_PLAY_BODY_SETTINGS => settings::render(&envelope, labels, settings::panel_window_id(view_state))?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}

//#region 🧵️RetainedCommands
/// 🧵️ The 2D editor actions that carry an exact app-owned tool proof. Only these may declare
/// [`InteractiveJobClassification::Migrated`] — UI dispatch resolves a controller/owner/factory/tool
/// /schema proof for every migrated verb, and one without a registered factory is refused outright.
pub(crate) const PUZZLE2D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "forceLayout",
    "addNode",
    "applyBoardEvents",
    "reorganize",
    "closeHandleSuggestions",
    "acceptSuggestion",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "openHandleSuggestions",
    "hoverSuggestion",
    "targetBrushSuggestions",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementSubmit",
    "focusSelection",
    "lodScaleJson",
    "patchInspectorNodes",
    "redrawHandles",
    "selectSameKind",
    "setBrushKindWeights",
    "setBrushNodeSize",
    "setCamera",
    "setFillCount",
    "setGridFactor",
    "setGridSnapEnabled",
    "setGridVisible",
    "setSelectableKind",
    "setBrushPlacementContactTolerance",
    "setBrushPlacementOverlapBudget",
    "engagementRepeatLast",
    "openAddNodeDialog",
    "setLodModeForPane",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "exportSnapshot",
    "importSnapshot",
    "openImportSnapshot",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "setSuggestionOffset",
    "setTransformGumballFlag",
    "addTargetRegion",
    "deleteTargetRegion",
    "relocateTargetRegion",
    "setTargetRegionFlag",
    "setTargetRegionHidden",
    "setTargetRegionLocked",
    "setAreaBrushSize",
    "createEdge",
    "deleteEdge",
    "proximityConnect",
    "setProximityRadius",
];

/// 🧭️ The two framework-injected host-configuration verbs. They are `InteractiveJobClassification::Migrated`
/// in the manifest and [`Puzzle2dPlayApp::host_configuration_mutation`] resolves each to one Config
/// mutation, so they need a generated tool id — without which `validate_tool_job_rows` refuses
/// [`Puzzle2dHostConfigurationProofs`]' generic bounded proofs and `dispatch_action` fails closed.
/// They carry NO app-owned factory, so they stay out of [`PUZZLE2D_RETAINED_TOOL_IDS`], which is what
/// keys [`Puzzle2dRetainedCommandJobFactory`] and what the retained-jobs snapshot pins.
pub(crate) const PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS: &[&str] = &["setActiveTool", "setActiveUtility"];

/// 🗂️ Every generated tool id this app declares: the app-owned retained verbs plus the two
/// host-configuration verbs. `OpBinary::TOOL_JOB_IDS` — the list the framework joins against the
/// migrated manifest rows.
pub(crate) const PUZZLE2D_TOOL_JOB_IDS: [&str; PUZZLE2D_RETAINED_TOOL_IDS.len() + PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len()] = {
    let mut ids = [""; PUZZLE2D_RETAINED_TOOL_IDS.len() + PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len()];
    let mut index = 0;
    while index < PUZZLE2D_RETAINED_TOOL_IDS.len() {
        ids[index] = PUZZLE2D_RETAINED_TOOL_IDS[index];
        index += 1;
    }
    let mut host = 0;
    while host < PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS.len() {
        ids[index + host] = PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS[host];
        host += 1;
    }
    ids
};
const PUZZLE2D_RETAINED_PAYLOAD_SCHEMA: &str = "board.ports.directed.v1.tool-command.v1";

/// 🎬️ The retained verbs whose whole completion is [`puzzle2d_dispatch_emit`] — one `🎮️commands/*`
/// arm run over a rebuilt scene/board host, then the same document delta and config snapshot
/// `handle` derives. Everything outside this list carries a bespoke `Work` (
/// `forceLayout`/`reorganize`) or an isolated reducer (`addNode`, `setActiveExample`).
const PUZZLE2D_GENERIC_TOOL_IDS: &[&str] = &[
    "closeHandleSuggestions",
    "acceptSuggestion",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "openHandleSuggestions",
    "hoverSuggestion",
    "targetBrushSuggestions",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementSubmit",
    "focusSelection",
    "patchInspectorNodes",
    "selectSameKind",
    "setBrushKindWeights",
    "setBrushNodeSize",
    "setCamera",
    "setFillCount",
    "setGridFactor",
    "setGridSnapEnabled",
    "setGridVisible",
    "setSelectableKind",
    "setBrushPlacementContactTolerance",
    "setBrushPlacementOverlapBudget",
    "engagementRepeatLast",
    "openAddNodeDialog",
    "setLodModeForPane",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "setSuggestionOffset",
    "setTransformGumballFlag",
    "addTargetRegion",
    "deleteTargetRegion",
    "relocateTargetRegion",
    "setTargetRegionFlag",
    "setTargetRegionHidden",
    "setTargetRegionLocked",
    "setAreaBrushSize",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "importSnapshot",
    "openImportSnapshot",
    "createEdge",
    "deleteEdge",
    "proximityConnect",
    "setProximityRadius",
];

/// 🫙️ The one verb whose `🎮️commands/*` arm is empty by construction — `lodScaleJson` only reads a
/// pure engine LOD table and declares [`UiDirtyScope::None`]. It publishes nothing, so it completes
/// through `NoopPuzzleCommandWork` under a solo `HostOnly` contract rather than the dispatch pipeline.
const PUZZLE2D_HOST_ONLY_TOOL_IDS: &[&str] = &["lodScaleJson"];

/// 📏️ Raw wire bytes ONE retained 2d command may carry — one whole `importSnapshot` file, escaped, plus its envelope
/// (`PUZZLE_IMPORT_RAW_BYTES`, the widest command this route admits; a whole-board `applyBoardEvents` select over
/// Nakagin's 180 ids is far below it), the same figure the 3d factory admits.
const PUZZLE2D_COMMAND_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;
const PUZZLE2D_COMMAND_DECODED_ITEMS: usize = 16_384;

struct Puzzle2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
    contract: semio_framework::ToolExecutionContract,
}

impl Puzzle2dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self {
            keys: PUZZLE2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(),
            contract: semio_framework::ToolExecutionContract::resumable(PUZZLE2D_COMMAND_RAW_BYTES, PUZZLE2D_COMMAND_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        }
    }
}

impl ToolJobFactory for Puzzle2dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Puzzle2dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        self.contract
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > payload.maximum_raw_bytes || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Puzzle 2d retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Puzzle2dRetainedCommandJobFactory {
    type Owner = EditorApp<Puzzle2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_BOARD_SNAPSHOT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "forceLayout", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "redrawHandles", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "closeHandleSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidate", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidateBack", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "targetBrushSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "openHandleSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "hoverSuggestion", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementControlSelect", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setBrushKindWeights", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setBrushNodeSize", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSelectableKind", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementContactTolerance", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementOverlapBudget", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "openAddNodeDialog", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setLodModeForPane", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "exportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openImportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setSuggestionOffset", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addTargetRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteTargetRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "relocateTargetRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetRegionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetRegionHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetRegionLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setAreaBrushSize", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "applyBoardEvents", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "acceptSuggestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "patchInspectorNodes", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "lodScaleJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "selectSameKind", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "createEdge", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteEdge", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "proximityConnect", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setProximityRadius", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}

//#endregion 🧵️RetainedCommands

//#region 📜️ToolProofs

//#region 🧵️ReservedJobs
/// 📜️ The retained command catalog's own bounded first-step proofs — one per
/// [`PUZZLE2D_RETAINED_TOOL_IDS`] entry, all joined to the single concrete
/// [`Puzzle2dRetainedCommandJobFactory`].
struct Puzzle2dRetainedCommandProofs;

impl Puzzle2dRetainedCommandProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle2d@1/*#editor",
        artifact_schema: "board.ports.directed.v1",
        factory: "Puzzle2dRetainedCommandJobFactory",
        factory_type: Puzzle2dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(PUZZLE2D_COMMAND_RAW_BYTES, PUZZLE2D_COMMAND_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        tools: [
            "setActiveExample",
            "forceLayout",
            "addNode",
            "applyBoardEvents",
            "reorganize",
            "closeHandleSuggestions",
            "acceptSuggestion",
            "cycleBrushCandidate",
            "cycleBrushCandidateBack",
            "openHandleSuggestions",
            "hoverSuggestion",
            "targetBrushSuggestions",
            "deleteSelection",
            "duplicateSelection",
            "engagementAbort",
            "engagementControlSelect",
            "engagementInput",
            "engagementRepeatLast",
            "engagementSubmit",
            "focusSelection",
            "lodScaleJson",
            "openAddNodeDialog",
            "patchInspectorNodes",
            "redrawHandles",
            "selectSameKind",
            "setBrushKindWeights",
            "setBrushNodeSize",
            "setBrushPlacementContactTolerance",
            "setBrushPlacementOverlapBudget",
            "setCamera",
            "setFillCount",
            "setGridFactor",
            "setGridSnapEnabled",
            "setGridVisible",
            "setLodModeForPane",
            "setSelectableKind",
            "setSelectionFlag",
            "setSelectionHidden",
            "setSelectionLocked",
            "setSuggestionOffset",
            "setTransformGumballFlag",
            "addTargetRegion",
            "deleteTargetRegion",
            "relocateTargetRegion",
            "setTargetRegionFlag",
            "setTargetRegionHidden",
            "setTargetRegionLocked",
            "setAreaBrushSize",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "exportSnapshot",
            "importSnapshot",
            "openImportSnapshot",
            "createEdge",
            "deleteEdge",
            "proximityConnect",
            "setProximityRadius",
        ]
    }
}

/// 📜️ The two framework-injected host-configuration verbs. Deliberately GENERIC proofs (no
/// `factory_type`): they are not app-owned retained tools — [`Puzzle2dRetainedCommandJobFactory`]
/// never claims them — they only need the wire admission and output budget `dispatch_action`'s
/// host-configuration branch asks for before it applies `host_configuration_mutation`.
struct Puzzle2dHostConfigurationProofs;

impl Puzzle2dHostConfigurationProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle2d@1/*#editor",
        artifact_schema: "board.ports.directed.v1",
        factory: "BoundedFirstStepCommandJobFactory",
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 8, 1, 8_192, 7_500, 1, 1),
        tools: ["setActiveTool", "setActiveUtility"]
    }
}
//#endregion 🧵️ReservedJobs
//#endregion 📜️ToolProofs

//#region 🧵️ReservedJobs
impl ArtifactEditor for Puzzle2dPlayApp {
    const DIALECT: Dialect = crate::standards::v1::subsets::any::schema::PUZZLE2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE2D_BOARD_SNAPSHOT_SCHEMA;
    type Snapshot = Puzzle2dPlaySnapshot;
    type Mutation = Puzzle2dMutation;
    type Config = Puzzle2dConfig;
    type ConfigMutation = Puzzle2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle2dPresence;
    type PresenceMutation = Puzzle2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle2dCommand;

    /// 🐁️ The six framework interaction verbs answered out of this app's OWN narrow scopes instead of
    /// the framework's blanket [`UiDirtyScope::Full`]. A hover fires on every pointermove, so it
    /// repaints the three canvas panes alone — never the panels, the engagement bar, the measures or
    /// the labels; a pick additionally repaints the outliner/inspector rows that highlight it. A verb
    /// that touched a domain this app does not declare answers `None` and keeps the framework's widest scope.
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        if domains.is_empty() || domains.iter().any(|domain| *domain != PUZZLE2D_INTERACTION_DOMAIN) {
            return None;
        }
        Some(match verb {
            InteractionVerb::Hover => puzzle2d_window_only_scope(),
            InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll => puzzle2d_select_scope(),
            InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity => puzzle2d_window_and_engagements_scope(),
        })
    }

    /// 🗺️ The `vortex` domain's whole pickable universe, in board order: every node with its handles under it, every
    /// edge and every target region (a region answers to the node granularity, as `puzzle2d_selection_targets`
    /// classifies it) — what `selectAll` enumerates and what keeps a selection inside the document. A host fact never
    /// reaches this editor: the window's gesture slot ends an open gesture on a blur, a lost capture, a utility switch,
    /// a closing window, a frozen document and a moved base.
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
        let document = doc.snapshot.typed();
        let mut ordered = Vec::new();
        for node in &document.nodes {
            ordered.push(semio_framework_plugin::TopologyNode { id: node.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_NODE.into(), parent: None });
            ordered.extend(node.handles.iter().map(|handle| semio_framework_plugin::TopologyNode { id: handle.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_HANDLE.into(), parent: Some(node.id.to_string_owner()) }));
        }
        ordered.extend(document.edges.iter().map(|edge| semio_framework_plugin::TopologyNode { id: edge.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_EDGE.into(), parent: None }));
        ordered.extend(document.target_regions.iter().map(|region| semio_framework_plugin::TopologyNode { id: region.id.to_string_owner(), granularity: PUZZLE2D_GRANULARITY_NODE.into(), parent: None }));
        Ok(semio_framework_plugin::InteractionTopology { domains: std::collections::BTreeMap::from([(PUZZLE2D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered })]) })
    }

    /// 🪧️ A history-edit reference chip names its board entity as the outliner does ([`puzzle2d_entity_label`]).
    fn entity_label(snapshot: &Puzzle2dPlaySnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        puzzle2d_entity_label(snapshot.value(), kinds, id)
    }



    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle2dConfigStorePreparationFactory))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle2dArtifactStorePreparationFactory))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }


    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Puzzle 2d presence is three inline camera scalars, so the default root is its exact empty terminal.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default Puzzle2d presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window::register_config(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window::register_transient(registry)
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle2d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle2dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::puzzle2d::config::schema::app_schema_descriptor())
    }

    /// 🚀️ The empty snapshot, nothing else. The example documents are `LazyLock`s the example load forces
    /// when it is actually switched to: warming them here parsed the Concrete Forest and Nakagin documents on
    /// every construction of this app — every mount, and every `codec` call of a hub creating or validating a
    /// document, where the interpreter paid for two documents the snapshot never reads.
    fn initial_snapshot() -> Puzzle2dPlaySnapshot {
        Puzzle2dPlaySnapshot::new(crate::standards::v1::subsets::any::schema::empty_puzzle2d_snapshot())
    }

    /// 🏷️ Maps each `Puzzle2dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle2dCommand) -> &'static str {
        command.action_id()
    }

    /// 📢️ The localized notices of this guest's refusal codes ([`puzzle2d_fault_notices`]).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        puzzle2d_fault_notices()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        if let Some(flag) = puzzle2d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle2d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        Puzzle2dCommand::try_from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 2D action '{action}'")))
    }

    /// 🎬️ Routes the example load through its own `Work` before document materialization; every other
    /// command runs the one shared [`puzzle2d_dispatch_emit`] pipeline the retained generic reduce runs.
    fn handle(
        command: &Puzzle2dCommand,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, Self::DraftMutation>, Fault> {
        let config = cfg.snapshot;
        let (action, _args) = (command.action_id(), command.args());
        if action == "setActiveExample" {
            return puzzle2d_active_example_emit(command, doc.snapshot, config);
        }
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_transient = Puzzle2dWindowTransient::default();
        let window_kind = view_state.and_then(window::kind_for_view).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        let base_revision = doc.operation_optional().map(|operation| puzzle2d_revision_hex(&operation.canonical_base_revision)).unwrap_or_default();
        puzzle2d_dispatch_emit(command, doc.snapshot, config, &window_config, &window_transient, window_kind, view_state, puzzle2d_active_utility(view_state), interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), authoring_seed, &base_revision, &semio_framework_plugin::app::GestureSlot::detached(), doc.operation_optional().cloned())
            .map(|(emit, _)| emit)
    }

    /// 🧭️ The two framework-injected host-configuration verbs, resolved to no Config mutation of this
    /// app's own. The shell owns their session state and forwards the resolved value here so the app
    /// may clear its scratch; puzzle2d's active utility IS host view state
    /// (`puzzle2d_active_utility` reads `ViewModel::active_utility_id`) and its brush slot lives in
    /// the WINDOW TRANSIENT lane, which this Config-typed seam cannot address — the transient is
    /// rebuilt from the next dispatch anyway. Without this hook (and the proofs below) both verbs fell
    /// through to `admit_command_json` and failed closed with `interactive-job.missing-factory`, so
    /// the Fill tool tab could not even be selected.
    fn host_configuration_mutation(_action: &str, _args: Option<&semio_framework_value::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok(None)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle2dRetainedCommandProofs::bounded_first_step_tool_proofs();
        proofs.extend(Puzzle2dHostConfigurationProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PUZZLE2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "puzzle2d-command-tool-mismatch"));
        }
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Self>>> = match request.command.action_id() {
            "setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new("setActiveExample", puzzle2d_active_example_reduce, puzzle2d_active_example_extent)),
            "forceLayout" => Box::new(Puzzle2dForceLayoutWork::default()),
            "reorganize" => Box::new(Puzzle2dForceLayoutWork::new("reorganize")),
            "addNode" => Box::new(Puzzle2dWindowCommandWork::new("addNode", puzzle2d_add_node_extent, request.authoring_seed.clone(), puzzle2d_revision_hex(&request.canonical_base_revision))),
            "exportSnapshot" => Box::new(Puzzle2dExportWork::default()),
            "applyBoardEvents" => Box::new(Puzzle2dWindowCommandWork::new("applyBoardEvents", puzzle2d_board_events_extent, request.authoring_seed.clone(), puzzle2d_revision_hex(&request.canonical_base_revision))),
            generic if PUZZLE2D_GENERIC_TOOL_IDS.contains(&generic) => Box::new(Puzzle2dWindowCommandWork::new(generic, puzzle2d_generic_extent, request.authoring_seed.clone(), puzzle2d_revision_hex(&request.canonical_base_revision))),
            host_only if PUZZLE2D_HOST_ONLY_TOOL_IDS.contains(&host_only) => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(host_only)),
            "redrawHandles" => Box::new(Puzzle2dRedrawHandlesWork::default()),
            _ => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), "puzzle2d-command-tool-unmapped")),
        };
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            Puzzle2dCommand::action_id,
            PUZZLE2D_COMMAND_RAW_BYTES,
            crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// ⏯️ Builds the fill tool run's jobs (`ToolRunDefinition.runJob` / `.revalidateJob`): the run job over the run's
    /// base with the config's requested count, resuming from the ledger's checkpoint and provisional ops, and the
    /// revalidate job over the committed head.
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        if request.tool_id != fill::TOOL_ID {
            return Ok(None);
        }
        Ok(Some(match request.purpose {
            semio_framework_plugin::ToolRunJobPurpose::Run => {
                Box::new(fill_run::Puzzle2dFillRunJob::new(request.identity, request.snapshot, crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_SUGGESTION_OFFSET, request.config.fill_count, request.checkpoint, request.provisional).map_err(Fault::from)?)
            }
            // 🚧️ The placement-tuning settings reach the run through `ToolRunSettingsReads` — the
            // framework rebuilds this job whenever either pointer changes, so the net slack a
            // revalidation tests with is always the one the settings panel currently shows.
            semio_framework_plugin::ToolRunJobPurpose::Revalidate => Box::new(fill_run::Puzzle2dFillRevalidateJob::new(
                request.identity,
                request.snapshot,
                request.provisional,
                request.checkpoint,
                request.config.contact_tolerance - request.config.brush_placement_overlap_budget,
            )),
        }))
    }

    /// 🔌️ Declares puzzle2d's typed media I/O surface — the implicit document ports plus `kit:in` and
    /// `design:out`. `kit:in`'s producer is `Block2dPlayApp`'s `"catalog:out"` port, whose `kit.catalog`
    /// payload is literally a puzzle2d manifest fragment (`portKinds`/`wireKinds`/`edgeKinds`/
    /// `nodeKinds`/`kindCompatibility` — `crate::standards::v1::subsets::any::schema::inferences::puzzle2d_manifest_fragment`),
    /// so this is a 2d↔2d vocabulary, not block3d's object/vortex-kind one; `Puzzle2dImportJob` does the
    /// normalization one bounded row per step.
    fn io() -> Option<AppIo> {
        let io = ::semio_framework_async::poll::resolve_ready(AppIo::from_artifact(
            "puzzle.2d",
            MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
            ArtifactPresentation { id: "2d.puzzle".into(), name: "2D Puzzle".into(), dimension: "2d".into(), component_kind: "puzzle2d".into() },
        ));
        Some(::semio_framework_async::poll::resolve_ready(io.with_ports(vec![
            MediaPortSpec {
                id: PUZZLE2D_IMPORT_PORT.into(),
                label: "Kit Catalog".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                kind_id: Some("kit.catalog".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
            MediaPortSpec {
                id: "design:out".into(),
                label: "Puzzle Design".into(),
                direction: MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
                kind_id: Some("2d.puzzle".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ])))
    }

    /// 📋️ The `MediaType` a puzzle 2d copy writes and a paste accepts — without it the framework's
    /// injected `copy`/`cut`/`paste` actions silently no-op.
    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::TwoD, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        let marks = Puzzle2dInteractionSnapshot::from_interaction(interaction);
        puzzle2d_copy_fragment_from(doc.snapshot.value(), &puzzle2d_selected_node_ids(doc.snapshot.value(), marks.selected_ids()))
    }

    fn cut_operations(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle2dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle2dMutation> {
        let marks = Puzzle2dInteractionSnapshot::from_interaction(interaction);
        let selected = puzzle2d_selected_node_ids(doc.snapshot.value(), marks.selected_ids());
        if puzzle2d_selection_is_locked(doc.snapshot.value(), &selected) {
            return Vec::new();
        }
        puzzle2d_cut_operations_from(doc.snapshot, &selected)
    }

    fn paste_operations(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle2dMutation>, ClipboardError> {
        puzzle2d_paste_operations_on(doc.snapshot, fragment, placement).map(|(mutations, _)| mutations)
    }

    /// 🎞️ The reserved routes puzzle2d owns: `import-media` (the framework registers no importer on an
    /// app's behalf, and every inbound media delivery goes through `dispatch_import_media` →
    /// `build_artifact_reserved_media_job`, never the unbounded one-shot `ArtifactApp::import_media`
    /// seam) and the three clipboard verbs, whose fragment vocabulary is this artifact's own
    /// `puzzle.2d.clipboard.v1` — see [`Puzzle2dClipboardJob`].
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") {
            return Ok(Some(ArtifactReservedToolJob::new(Puzzle2dClipboardJob::new(request))));
        }
        if request.tool_id.as_str() != PUZZLE2D_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("puzzle2d import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("puzzle2d import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(Puzzle2dImportJob::new(request, port, media))))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &Puzzle2dWindowTransient::default(), Puzzle2dInteractionSnapshot::default())
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        Self::render_body(body_key, doc, cfg, view_state, &window::transient_from_view(transient), Puzzle2dInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let labels = puzzle2d_labels(view_state);
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = Self::scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        HashMap::from([(window_id.to_string(), edit::puzzle2d_engagement(&envelope, &puzzle_board_host(), window_kind, labels))])
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_transient = window::transient_from_view(transient);
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = Self::scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &window_transient, Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        HashMap::from([(window_id.to_string(), edit::puzzle2d_engagement(&envelope, &puzzle_board_host(), window_kind, puzzle2d_labels(view_state)))])
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = puzzle2d_labels(view_state);
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = Self::scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        let measures = match window_kind {
            crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID => detail::window_measures(&envelope, labels),
            crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID => selection::window_measures(&envelope, labels),
            _ => overview::window_measures(&envelope, labels),
        };
        HashMap::from([(window_id.to_string(), measures)])
    }

    fn tool_measures(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID);
        let envelope = Self::scene_for(doc.snapshot.value().clone(), window::runtime(cfg.snapshot, &window_config, &Puzzle2dWindowTransient::default(), Some(window_kind)), puzzle2d_active_utility(Some(view_state)));
        let labels = puzzle2d_labels(view_state);
        HashMap::from([(fill::TOOL_ID.to_string(), vec![fill::measures(&envelope, labels)])])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        _cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, view_state, &Puzzle2dInteractionSnapshot::default(), registry)
    }

    /// 🕹️ The context menu reads the AUTHORITATIVE framework-owned selection when the surface names
    /// none of its own — a right-click on an outliner row or an empty board still targets what the
    /// user selected; the surface's own hits/selection win when present, so a right-click on an
    /// unselected node still targets that node.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        _cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, view_state, &Puzzle2dInteractionSnapshot::from_interaction(interaction), registry)
    }
}

impl Puzzle2dPlayApp {
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle2dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let mut selected: Vec<String> = request.surface.as_ref().map(|surface| surface.selection.iter().flat_map(|g| g.ids.iter().cloned()).collect()).unwrap_or_default();
        if selected.is_empty() {
            selected = interaction.selected.clone();
        }
        ::semio_framework_async::poll::resolve_ready(puzzle2d_context_menu_items(registry, doc.snapshot.value(), &selected, view_state))
    }
}
//#endregion 🧵️ReservedJobs
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/engagement-bound
/// vocabulary dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn puzzle2d_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 📢️ The localized notice of every refusal code the puzzle2d guest names (design §20.12): a set-verb's missing flag value.
fn puzzle2d_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 1]> = std::sync::LazyLock::new(|| {
        [
            ("puzzle2d.action.flag-value-required", LocalizedLabel::native("Choose on or off for this setting.", "Für diese Einstellung Ein oder Aus wählen.")),
        ]
    });
    &*NOTICES
}

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setTargetRegionLocked{locked}`, …) — the row target's explicit next state, so a stale view sets a value and never flips one.
fn puzzle2d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" | "setTargetRegionHidden" => Some("hidden"),
        "setSelectionLocked" | "setTargetRegionLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit identity a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
fn puzzle2d_flag_value_action(id: &str, category: &str, identity: ActionArgDef, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    puzzle2d_internal_action(id, label, ActionKind::Mutation).with_category(category).with_args([identity, ActionArgDef::toggle(flag, value).required()])
}

/// 🎭️✏️ `.example_source(...)` (×2, concrete-forest + nakagin) and `.workflow("puzzle2d", …)` were
/// dropped, not ported — `EditorBuilder` has neither method (contract §2.4: `.editor::<E>(def:
/// AppDefinition)` only takes the definition, `App.examples` has no seam on this builder). Flagged to
/// the coordinator, not silently lost; see `📚️examples/🎬️demo-session` for this subset's own example
/// facet, the likely intended replacement mechanism.
pub fn create_puzzle2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(Puzzle2dPlayApp::DIALECT)
            .document(["semio", "puzzle", "2d"])
            .artifact_kind(crate::standards::v1::subsets::any::schema::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "2d"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE2D_PLAY_MODE_EDIT)
            .window_kind_def(overview::definition())
            .window_kind_def(detail::definition())
            .window_kind_def(selection::definition())
            .interaction(puzzle2d_interaction_definition())
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE2D_INTERACTION_DOMAIN)])
            .panel_tab_def(artifact::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .panel_tab_def(settings::definition())
            // ✏️ Palette-visible content operations.
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            // 🗣️ Locale and terminology are real user-facing settings verbs (mirrors puzzle3d's own
            // `.view_action` pair); the labels stay inline `LocalizedLabel::native` like every other
            // action here, since `puzzle2d_localized` resolves a `Puzzle2dLabels` field, not a phrase.
            // 🗂️ Referenced by `puzzle2d_context_menu_items` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteSelection")
            .keybinding("delete,backspace", "deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).with_category("create"))
            .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
            .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Mutation).with_category("view"))
            // 🔄️ Transform verbs over the live selection — the 2d twins of puzzle3d's gumball commits.
            .action_with(ActionDefinition::bounded_catalog("translateSelection", puzzle2d_localized(|l| l.translate), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::bounded_catalog("rotateSelection", puzzle2d_localized(|l| l.rotate), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::bounded_catalog("scaleSelection", puzzle2d_localized(|l| l.scale), ActionKind::Mutation).with_category("transform"))
            .keybinding("mod+d", "duplicateSelection")
            // 📤📥 Fixture round trip: a JSON download of the document and a file-picker import.
            .action_with(ActionDefinition::bounded_catalog("exportSnapshot", puzzle2d_localized(|l| l.export), ActionKind::Shell).with_category("file"))
            .action_with(ActionDefinition::bounded_catalog("openImportSnapshot", puzzle2d_localized(|l| l.import), ActionKind::Shell).with_category("file"))
            .action_with(puzzle2d_internal_action("importSnapshot", LocalizedLabel::native("Import Fixture", "Fixture importieren"), ActionKind::Mutation))
            // 🔗️ Edge CRUD over two handles — the host-dispatchable twin of the board engine's own
            // drag-connect gesture, and the drop-time auto-connect's programmatic entry point.
            .action_with(ActionDefinition::bounded_catalog("createEdge", puzzle2d_localized(|l| l.connect), ActionKind::Mutation).with_category("create"))
            .action_with(ActionDefinition::bounded_catalog("deleteEdge", puzzle2d_localized(|l| l.disconnect), ActionKind::Mutation).with_category("selection"))
            .action_destructive("deleteEdge")
            .action_with(ActionDefinition::bounded_catalog("proximityConnect", LocalizedLabel::native("Connect Nearby", "In der Nähe verbinden"), ActionKind::Mutation).with_category("create"))
            .action_with(puzzle2d_internal_action("setProximityRadius", puzzle2d_localized(|l| l.proximity_radius), ActionKind::View).with_category("settings"))
            // 👁️ Palette-visible ephemeral view/selection commands.
            .action_with(ActionDefinition::bounded_catalog("selectSameKind", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            // 🔧️ Internal content operations — inspector/panel/board/import-bound, not palette commands.
            .action_with(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(puzzle2d_flag_value_action("setSelectionHidden", "settings", ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle2d_flag_value_action("setSelectionLocked", "settings", ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(puzzle2d_internal_action("patchInspectorNodes", LocalizedLabel::native("Patch Inspector Nodes", "Inspektorknoten aktualisieren"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("redrawHandles", LocalizedLabel::native("Redraw Handles", "Anschlüsse neu zeichnen"), ActionKind::Mutation))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw") })
            .action_with(puzzle2d_internal_action("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("acceptSuggestion", puzzle2d_localized(|l| l.accept_suggestion), ActionKind::Mutation))
            // 🖱️ Internal pointer/gesture/engagement view vocabulary — pure runtime/host state, emit no operations.
            // 🎥️ `setCamera` is session-only view state, so it belongs in this View-kind group.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand") })
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            // ⌨️ A submitted engagement line ("move 50 25", "connect …") EDITS the document — its own
            // publication contract declares the Artifact lane — so it is a `Mutation`, exactly as
            // `📐️cad` and `🏭️process`'s identical verb declare it. Declared `View`, kind discipline
            // refused every submitted line at dispatch with "View-kind command 'engagementSubmit' must
            // not emit operations", i.e. the command line could not move, connect or place anything.
            .action_with(puzzle2d_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand") })
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand") })
            .action_with(puzzle2d_internal_action("setLodModeForPane", LocalizedLabel::native("Set LOD Mode For Pane", "LOD-Modus für Bereich festlegen"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View, "grid-3x3") })
            .action_with(puzzle2d_internal_action("setBrushKindWeights", LocalizedLabel::native("Set Brush Kind Weights", "Pinsel-Artgewichte festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushNodeSize", LocalizedLabel::native("Set Brush Node Size", "Pinsel-Knotengröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformationsgriff festlegen"), ActionKind::View))
            // 🎯️ Target regions — the flat analogue of puzzle3d's target volumes. `addTargetRegion` is
            // the Area Brush's Alt+click commit and the only palette-visible one; the three
            // entity-scoped verbs stay off the palette exactly as puzzle3d keeps its own off it.
            .mutation("addTargetRegion", puzzle2d_localized_phrase(|l| l.target_region, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("deleteTargetRegion", LocalizedLabel::native("Delete Target Region", "Zielbereich löschen"), ActionKind::Mutation).with_category("targets") })
            .action_destructive("deleteTargetRegion")
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setTargetRegionFlag", LocalizedLabel::native("Set Target Region Flag", "Zielbereichsmarkierung festlegen"), ActionKind::Mutation).with_category("targets") })
            .action_with(puzzle2d_flag_value_action("setTargetRegionHidden", "targets", ActionArgDef::text("id", puzzle2d_localized(|l| l.id)), "hidden", LocalizedLabel::native("Set Target Region Hidden", "Zielbereich verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle2d_flag_value_action("setTargetRegionLocked", "targets", ActionArgDef::text("id", puzzle2d_localized(|l| l.id)), "locked", LocalizedLabel::native("Set Target Region Locked", "Zielbereich gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(puzzle2d_internal_action("relocateTargetRegion", LocalizedLabel::native("Relocate Target Region", "Zielbereich verlagern"), ActionKind::Mutation))
            .action_with(puzzle2d_internal_action("setAreaBrushSize", LocalizedLabel::native("Set Area Brush Size", "Flächenpinselgröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("cycleBrushCandidate", puzzle2d_localized(|l| l.cycle_candidate), ActionKind::View))
            .action_with(puzzle2d_internal_action("cycleBrushCandidateBack", puzzle2d_localized(|l| l.cycle_candidate_back), ActionKind::View))
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .action_with(puzzle2d_internal_action("targetBrushSuggestions", puzzle2d_localized(|l| l.target_suggestions), ActionKind::View))
            .action_with(puzzle2d_internal_action("hoverSuggestion", puzzle2d_localized(|l| l.hover_suggestion), ActionKind::View))
            .action_audience("hoverSuggestion", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(puzzle2d_internal_action("openHandleSuggestions", puzzle2d_localized(|l| l.suggest_nodes), ActionKind::View))
            .action_with(puzzle2d_internal_action("closeHandleSuggestions", puzzle2d_localized(|l| l.close_suggestions), ActionKind::View))
            .action_with(puzzle2d_internal_action("lodScaleJson", LocalizedLabel::native("LOD Scale Json", "LOD-Skalierung-Json"), ActionKind::View))
            // 🌐️🎯️ Window-option verbs: the grid's show/hide toggle and the per-granularity pick filter.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster einblenden"), ActionKind::View, "layout-grid") })
            .action_with(puzzle2d_internal_action("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"), ActionKind::View))
            // 🚧️🫂️ Placement tuning — the two settings-panel steppers a brush/fill collision test reads.
            .action_with(puzzle2d_internal_action("setBrushPlacementContactTolerance", LocalizedLabel::native("Set Brush Placement Contact Tolerance", "Kontakttoleranz der Pinselplatzierung festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushPlacementOverlapBudget", LocalizedLabel::native("Set Brush Placement Overlap Budget", "Überlappungsbudget der Pinselplatzierung festlegen"), ActionKind::View))
            // 🔂️ The engagement bar's repeat-last: one more placement from the armed fill tool.
            .action_with(puzzle2d_internal_action("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"), ActionKind::View))
            // 🗨️ Shell-only effect (no document mutation): opens the declared "addNode" dialog. The
            // user-facing "Add Node…" row is this one, so the parametrized `addNode` verb below stays
            // the dialog's/catalogue row's/drop's target rather than a palette entry of its own.
            .action_with(ActionDefinition::bounded_catalog("openAddNodeDialog", puzzle2d_localized_phrase(|l| l.node, |w| format!("Add {w}…"), |w| format!("{w} hinzufügen…")), ActionKind::Shell).with_category("create"))
            // 📝️ Staged palette args for the two content commands that need a target.
            .action_args("addNode", vec![puzzle2d_node_kind_arg()])
            .action_args("translateSelection", vec![
                ActionArgDef::number("dx", LocalizedLabel::native("Δx", "Δx")).default_value(&0.0),
                ActionArgDef::number("dy", LocalizedLabel::native("Δy", "Δy")).default_value(&0.0),
            ])
            .action_args("rotateSelection", vec![ActionArgDef::number("angle", puzzle2d_localized(|l| l.angle)).required().default_value(&90.0)])
            .action_args("scaleSelection", vec![ActionArgDef::number("factor", LocalizedLabel::native("Factor", "Faktor")).required().default_value(&1.5)])
            .action_args("createEdge", vec![
                ActionArgDef::text("source", puzzle2d_localized(|l| l.source)).required(),
                ActionArgDef::text("target", puzzle2d_localized(|l| l.target)).required(),
            ])
            .action_args("deleteEdge", vec![ActionArgDef::text("id", puzzle2d_localized(|l| l.id)).required()])
            .action_args("setProximityRadius", vec![ActionArgDef::number("value", puzzle2d_localized(|l| l.proximity_radius)).required().default_value(&crate::editor::puzzle2d::config::PUZZLE2D_DEFAULT_PROXIMITY_RADIUS)])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, puzzle2d_localized(|l| l.example_concrete_forest)),
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin Capsule Tower")),
                ]).required().default_value(&PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID),
            ])
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyBoardEvents", InteractiveJobClassification::Migrated)
            .action_interactive_job("closeHandleSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("acceptSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("openHandleSuggestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("forceLayout", InteractiveJobClassification::Migrated)
            .action_interactive_job("lodScaleJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchInspectorNodes", InteractiveJobClassification::Migrated)
            .action_interactive_job("redrawHandles", InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
            .action_interactive_job("createEdge", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteEdge", InteractiveJobClassification::Migrated)
            .action_interactive_job("proximityConnect", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushKindWeights", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushNodeSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridFactor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodModeForPane", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSnapshot", InteractiveJobClassification::Migrated)
            .action_destructive("exportSnapshot")
            .action_interactive_job("importSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSuggestionOffset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionFlag", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetRegionLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAreaBrushSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementContactTolerance", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementOverlapBudget", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddNodeDialog", InteractiveJobClassification::Migrated)
            // 🗨️ The dialog `openAddNodeDialog` opens, driving the existing `addNode` operation's LIVE
            // `kind` select.
            .dialog(
                DialogDefinition::new(PUZZLE2D_ADD_NODE_DIALOG_ID, puzzle2d_localized_phrase(|l| l.node, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")), ActionRef::new("addNode"))
                    .body(puzzle2d_localized_phrase(|l| l.node, |w| format!("Choose the kind of {w} to add to the board."), |_w| "Wählen Sie die Art zum Hinzufügen.".to_string()))
                    .args(vec![puzzle2d_node_kind_arg()])
                    .submit_label(LocalizedLabel::native("Add", "Hinzufügen")),
            )
            // 🧰️ Canvas utilities — one exclusive set, active utility host-owned (never a document
            // operation); bound to the interactive overview pane by that window's own definition.
            .utility(select_utility::definition(puzzle2d_localized(|l| l.select)))
            .utility(brush_utility::definition(puzzle2d_localized(|l| l.brush)))
            .utility(area_brush_utility::definition(puzzle2d_localized(|l| l.area_brush)))
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
            .tool(fill::definition(puzzle2d_localized(|l| l.fill)))
            .default_layout(edit::layout())
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 2D puzzle with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 2D-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected node from the 2D puzzle together with its connections.", "Löscht alle ausgewählten Knoten samt ihrer Verbindungen aus dem 2D-Puzzle."))
            .action_describe("duplicateSelection", LocalizedLabel::native("Adds a copy of every selected node next to the original; nothing happens without a selection.", "Fügt neben jedem ausgewählten Knoten eine Kopie hinzu; ohne Auswahl geschieht nichts."))
            .action_describe("focusSelection", LocalizedLabel::native("Frames the camera on the selected nodes; only the view changes.", "Richtet die Kamera auf die ausgewählten Knoten aus; nur die Ansicht ändert sich."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected nodes by dx and dy.", "Verschiebt die ausgewählten Knoten um dx und dy."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected nodes by the given angle.", "Dreht die ausgewählten Knoten um den angegebenen Winkel."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected nodes by the given factor.", "Skaliert die ausgewählten Knoten um den angegebenen Faktor."))
            .action_describe("exportSnapshot", LocalizedLabel::native("Writes the whole 2D puzzle as JSON to a downloaded file named after the active example on the user's machine.", "Schreibt das gesamte 2D-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."))
            .action_describe("openImportSnapshot", LocalizedLabel::native("Opens the host's file picker for a 2D puzzle JSON file; the chosen file then replaces the whole puzzle.", "Öffnet die Dateiauswahl des Hosts für eine 2D-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."))
            .action_describe("importSnapshot", LocalizedLabel::native("Replaces the whole 2D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 2D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen."))
            .action_describe("setSelectionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given or selected nodes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten Knoten."))
            .action_describe("setSelectionHidden", LocalizedLabel::native("Sets the given or selected nodes hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Knoten oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setSelectionLocked", LocalizedLabel::native("Sets the given or selected nodes locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Knoten oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("acceptSuggestion", LocalizedLabel::native("Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."))
            .action_describe("addNode", LocalizedLabel::native("Adds a node of the given kind (a 2D block kind) to the puzzle at x, y.", "Fügt dem Puzzle an x, y einen Knoten der angegebenen Art (einer 2D-Blockart) hinzu."))
            .action_describe("forceLayout", LocalizedLabel::native("Lays out every node of the 2D puzzle with a force-directed layout, overwriting their positions.", "Ordnet alle Knoten des 2D-Puzzles mit einem kraftbasierten Layout an und überschreibt ihre Positionen."))
            .action_describe("createEdge", LocalizedLabel::native("Connects two handles of two nodes (source and target) with an edge.", "Verbindet zwei Griffe zweier Knoten (Quelle und Ziel) mit einer Kante."))
            .action_describe("deleteEdge", LocalizedLabel::native("Removes one edge by id, disconnecting its two handles.", "Entfernt eine Kante anhand ihrer Id und trennt ihre zwei Griffe."))
            .action_describe("proximityConnect", LocalizedLabel::native("Connects every pair of compatible handles that lie within the proximity radius of each other.", "Verbindet jedes Paar verträglicher Griffe, die innerhalb des Näheradius beieinander liegen."))
            .action_describe("selectSameKind", LocalizedLabel::native("Extends the selection to every node of the same kind as the selected one.", "Erweitert die Auswahl auf alle Knoten derselben Art wie der ausgewählte."))
            .action_describe("patchInspectorNodes", LocalizedLabel::native("Sets one field on the given nodes, or on handles addressed by id (such as a handle's angle or radius); an empty id list addresses every node.", "Setzt ein Feld auf den angegebenen Knoten oder per Id adressierten Griffen (etwa Winkel oder Radius eines Griffs); eine leere Id-Liste adressiert alle Knoten."))
            .action_describe("redrawHandles", LocalizedLabel::native("Re-aims every connected handle at the edge it carries, updating the handle angles.", "Richtet jeden verbundenen Griff neu auf seine Kante aus und aktualisiert die Griffwinkel."))
            .action_describe("reorganize", LocalizedLabel::native("Lays out every node of the 2D puzzle automatically, overwriting their manual positions.", "Ordnet alle Knoten des 2D-Puzzles automatisch an und überschreibt ihre manuellen Positionen."))
            .action_describe("addTargetRegion", LocalizedLabel::native("Adds a rectangular target region at the given origin and size (the area brush size by default).", "Fügt an Ursprung und Größe (standardmäßig der Flächenpinselgröße) einen rechteckigen Zielbereich hinzu."))
            .action_describe("deleteTargetRegion", LocalizedLabel::native("Deletes one target region by id.", "Löscht einen Zielbereich anhand seiner Id."))
            .action_describe("setTargetRegionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given target regions.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielbereichen."))
            .action_describe("setTargetRegionHidden", LocalizedLabel::native("Sets the given or selected target regions hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Zielbereiche oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setTargetRegionLocked", LocalizedLabel::native("Sets the given or selected target regions locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Zielbereiche oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("relocateTargetRegion", LocalizedLabel::native("Moves or resizes one target region to the given rectangle.", "Verschiebt oder skaliert einen Zielbereich auf das angegebene Rechteck."))
            .action_describe("openAddNodeDialog", LocalizedLabel::native("Opens the Add Node dialog to pick a node kind to place.", "Öffnet den Dialog Knoten hinzufügen, um eine zu setzende Knotenart zu wählen."))
            .action_audience("applyBoardEvents", semio_framework_plugin::CapabilityAudience::Input)
            .action_destructive("importSnapshot")
            .action_destructive("forceLayout")
            .action_destructive("reorganize")
            .build_definition()
}

// 🗂️ `Puzzle2dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-2d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle2dPlayApp>()` on
// `crate::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle2d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone.
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ The one puzzle2d-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
/// instead of re-deriving a store/dispatch/render scaffold of its own.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;

#[cfg(test)]
#[path = "🧪️tests/🤖️agent-lane/🦀️.rs"]
mod agent_lane_tests;

/// 📋️ The clipboard route's own laws — copy/cut/paste round trips, the locked refusal, and the
/// single-edit/undo discipline every one of them owes.
#[cfg(test)]
#[path = "🧪️tests/🔬️clipboard/🦀️.rs"]
mod clipboard_tests;

/// 🔒️ The lock's own laws — delete, the board drag rows, the rotate ring, the three transform verbs
/// and an inspector patch each refuse a locked entity with exactly ONE sentence and no edit.
#[cfg(test)]
#[path = "🧪️tests/🔬️locks/🦀️.rs"]
mod lock_tests;

/// 🛠️ The select tool's own laws — one drag, one rotation, one nudge, one HUD move, one inspector delta is ONE
/// tool transaction (one edit, one row, one ref, one parametric leaf), and a cancelled gesture leaves zero trace.
#[cfg(test)]
#[path = "🧪️tests/🧪️select-tool-transactions/🦀️.rs"]
mod select_tool_transaction_tests;

/// 🧱️ The board tools' laws — every board gesture that is not a selection transform is ONE tool transaction of its
/// tool, a transient flush commits nothing, and the `vortex` topology names every board entity.
#[cfg(test)]
#[path = "🧪️tests/🧪️board-tools/🦀️.rs"]
mod board_tool_tests;

/// 📼️ The recorded-kinds laws — every board-editing command emits the concrete kinds of its user action, undo restores the
/// board, and a whole-document import leaves through the load path instead of a mutation.
#[cfg(test)]
#[path = "🧪️tests/🧪️recorded-kinds/🦀️.rs"]
mod recorded_kind_tests;

/// ⏪️ The select tool's leaves under time travel — the `🧫️select-tool-history` corpus through the app and a standalone
/// store: every input editable, the preview and its highlight, blocking errors resolved by editing, warnings kept.
#[cfg(test)]
#[path = "🧪️tests/🧪️select-tool-history/🦀️.rs"]
mod select_tool_history_tests;

/// ⏪️ The generic history-edit runtime with puzzle 2d as its witness — the `🧫️history-edit-runtime` corpus: drafts from a
/// review, replay progress and cancel, warnings after finalize and reload, Fatal/Error resolved by withdraw and by "Use
/// selection", overwrite and new alternative undone and redone, and a long remote history change paused and resumed.
#[cfg(test)]
#[path = "🧪️tests/🧪️history-edit-runtime/🦀️.rs"]
mod history_edit_runtime_tests;
//#endregion 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🧵️retained-wiring/🦀️.rs"]
mod retained_wiring_tests;
