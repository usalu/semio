//! 🔧️ Play app, retained commands and tool proofs: the puzzle2d play app with its render body, the retained command job factory and the bounded first-step tool proofs. See [`super`].

use super::*;
use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window;
use crate::mirror::editor::puzzle2d::modes::edit::windows::overview;
use crate::mirror::editor::puzzle2d::modes::edit::windows::detail;
use crate::mirror::editor::puzzle2d::modes::edit::windows::selection;
use crate::mirror::editor::puzzle2d::panels::catalogue;
use crate::mirror::editor::puzzle2d::panels::inspection;
use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::settings;
use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window::Puzzle2dWindowTransient;

//#region 🔖️PlayApp
/// 🧩️ Puzzle-2d play app. Owns the `BoardHost` engine; the persisted document (the bare snapshot json)
/// lives in the wrapping `VcsArtifactApp`'s operation store and the view state in `Puzzle2dConfig`.
#[derive(Default, Clone, Copy)]
pub struct Puzzle2dPlayApp;

impl Puzzle2dPlayApp {
    /// 🖼️ ONE render body for both `render` entry points: the window transient and the live selection
    /// are what the request-context path adds; the bare path passes their defaults.
    pub(super) fn render_body(
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        window_transient: &Puzzle2dWindowTransient,
        interaction: Puzzle2dInteractionSnapshot,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let window_config = window::config_from_view_or_document(cfg, doc.snapshot.value());
        let window_kind = window::kind_for_view(view_state).unwrap_or(semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::WINDOW_KIND_ID);
        let runtime = window::runtime(cfg.snapshot, &window_config, window_transient, Some(window_kind));
        let active_utility = puzzle2d_active_utility(Some(view_state));
        let snapshot = doc.snapshot.value().clone();
        let document_json = snapshot.to_string();
        let envelope = puzzle2d_scene_with(snapshot, runtime, active_utility, interaction);
        let labels = puzzle2d_labels(view_state);
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            overview::BODY_KEY => overview::render(&document_json, &envelope)?,
            detail::BODY_KEY => detail::render(&document_json, &envelope)?,
            crate::standards::v1::subsets::any::schema::mutations::selection::BODY_KEY => selection::render(&document_json, &envelope)?,
            crate::editor::puzzle2d::config::PUZZLE2D_PLAY_BODY_LAYERS => artifact::render(&envelope, labels, &windows)?,
            catalogue::PUZZLE2D_PLAY_BODY_CATALOGUE => catalogue::render(&envelope, labels, &windows)?,
            crate::editor::puzzle2d::config::PUZZLE2D_PLAY_BODY_PROPERTIES => inspection::render(&envelope, labels, &windows)?,
            settings::PUZZLE2D_PLAY_BODY_SETTINGS => settings::render(&envelope, labels, settings::panel_window_id(view_state))?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}

//#region 🧵️RetainedCommands
const PUZZLE2D_RETAINED_PAYLOAD_SCHEMA: &str = "board.ports.directed.v1.tool-command.v1";

/// 🫙️ The one verb whose `🎮️commands/*` arm is empty by construction — `lodScaleJson` only reads a
/// pure engine LOD table and declares [`UiDirtyScope::None`]. It publishes nothing, so it completes
/// through `NoopPuzzleCommandWork` under a solo `HostOnly` contract rather than the dispatch pipeline.
pub(super) const PUZZLE2D_HOST_ONLY_TOOL_IDS: &[&str] = &["lodScaleJson"];

/// 📏️ Raw wire bytes ONE retained 2d command may carry — one whole `importSnapshot` file, escaped, plus its envelope
/// (`PUZZLE_IMPORT_RAW_BYTES`, the widest command this route admits; a whole-board `applyBoardEvents` select over
/// Nakagin's 180 ids is far below it), the same figure the 3d factory admits.
pub(super) const PUZZLE2D_COMMAND_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;

const PUZZLE2D_COMMAND_DECODED_ITEMS: usize = 16_384;

pub(super) struct Puzzle2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
    contract: semio_framework::ToolExecutionContract,
}

impl Puzzle2dRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
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

    semio_framework_plugin_retained_command::retained_command_factory_admission!(maximum_raw_bytes: |payload| payload.maximum_raw_bytes, checkpoint_resume: true);

    
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
pub(super) struct Puzzle2dRetainedCommandProofs;

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
pub(super) struct Puzzle2dHostConfigurationProofs;

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
//#endregion 🔖️PlayApp
