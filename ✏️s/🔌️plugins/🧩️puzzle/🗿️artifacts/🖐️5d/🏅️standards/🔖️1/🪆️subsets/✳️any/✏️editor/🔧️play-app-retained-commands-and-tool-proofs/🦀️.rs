//! 🔧️ Play app, retained commands and tool proofs: the puzzle5d play app with its render body, the retained command job factory and the bounded first-step tool proofs. See [`super`].

use super::*;

//#region 🔖️PlayApp
// 🧩️ B1: Puzzle-5d play app. Stateless: the persisted document (bare `Puzzle5dDocument` json) lives in
// the wrapping `VcsArtifactApp`'s document store, the ephemeral view state in the wrapping store's real,
// VCS-tracked `Puzzle5dConfig` artifact (see `🦀️config.rs`) — every read comes from `cfg.snapshot`, every write
// flows out as a `Puzzle5dConfigMutation` in the returned `Emit` — and what an instance retains across turns
// (the brush suggestions link) in its instance operation owner.
// Each action mutates a transient {@link Puzzle5dScene}, then emits the granular operation delta.
// Undo/redo/checkpoints are handled by the wrapper.
#[derive(Default)]
pub struct Puzzle5dPlayApp;

impl Puzzle5dPlayApp {
    /// 🖱️ The ONE context-menu implementation — `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here, differing only in whether `interaction` carries a live
    /// `vortex`-domain read or the empty default.
    pub(super) fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle5dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let Some(labels) = puzzle5d_labels(view_state) else { return Vec::new() };
        let window_id = view_state.window_id.as_deref().unwrap_or(crate::editor::puzzle5d::entities::PUZZLE5D_WORLD_WINDOW_KIND_ID);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let mut selection = Puzzle5dContextSelection::from_surface(request.surface.as_ref());
        selection.fill_from_interaction(interaction);
        puzzle5d_context_menu_items(&envelope, &selection, labels, view_state, registry)
    }
}

//#region 🧵️RetainedCommands
pub struct Puzzle5dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
    pub(super) contract: ToolExecutionContract,
}

impl Puzzle5dRetainedCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self {
            keys: PUZZLE5D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(),
            contract: ToolExecutionContract::resumable(PUZZLE5D_RETAINED_RAW_BYTES, PUZZLE5D_RETAINED_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        }
    }
}

impl ToolJobFactory for Puzzle5dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Puzzle5dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle5dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE5D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        self.contract
    }

    semio_framework_plugin::retained_command_factory_admission!(maximum_raw_bytes: |payload| payload.maximum_raw_bytes, checkpoint_resume: true);

    
}

impl ArtifactOwnedToolJobFactory for Puzzle5dRetainedCommandJobFactory {
    type Owner = EditorApp<Puzzle5dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE5D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PUZZLE5D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidate", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidateBack", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementControlSelect", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "exportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importSnapshot", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "openAddPartDialog", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openImportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "selectSameKindSelection", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "targetBrushSuggestions", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "openVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "closeVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "hoverSuggestion", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "acceptSuggestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addBrushPart", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addPartKind", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "applyBoardEvents", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "createFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "editFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchGrip", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchPart", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "proximityConnect", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "registerBrushMesh", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "retargetFastener", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "worldRelocate", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setCamera2d", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setCamera3d", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSuggestionOffset", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementContactTolerance", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setProximityRadius", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setChunkSize", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setPartKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGripKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSpacing", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProjection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProjectionParam", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGripShow", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGripDirection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSelectableKind", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodAutomatic", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodDepthVariable", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodManual", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "relocateTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setVoxelDims", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation















//#endregion 📬️StorePreparation

//#region 📜️ToolProofs
/// 📜️ The retained command catalog's own bounded first-step proofs — one per
/// `PUZZLE5D_RETAINED_TOOL_IDS` entry, all joined to the single concrete
/// `Puzzle5dRetainedCommandJobFactory`.
pub(super) struct Puzzle5dRetainedCommandProofs;

impl Puzzle5dRetainedCommandProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle5dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle5d@1/*#editor",
        artifact_schema: "puzzle.5d",
        factory: "Puzzle5dRetainedCommandJobFactory",
        factory_type: Puzzle5dRetainedCommandJobFactory,
        contract: ToolExecutionContract::resumable(PUZZLE5D_RETAINED_RAW_BYTES, PUZZLE5D_RETAINED_DECODED_ITEMS, 1, 262_144, 7_500, 1, 1),
        tools: [
            "canvasPointerDown",
            "cycleBrushCandidate",
            "cycleBrushCandidateBack",
            "worldPointerDown",
            "deleteSelection",
            "duplicateSelection",
            "engagementAbort",
            "engagementControlSelect",
            "engagementInput",
            "engagementRepeatLast",
            "engagementSubmit",
            "exportSnapshot",
            "importSnapshot",
            "openAddPartDialog",
            "openImportSnapshot",
            "selectSameKindSelection",
            "setFillCount",
            "setSelectionFlag",
            "setSelectionHidden",
            "setSelectionLocked",
            "targetBrushSuggestions",
            "openVortexSuggestions",
            "closeVortexSuggestions",
            "hoverSuggestion",
            "acceptSuggestion",
            "focusSelection",
            "addBrushPart",
            "addNode",
            "addPartKind",
            "applyBoardEvents",
            "createFastener",
            "deleteFastener",
            "editFastener",
            "patchFastener",
            "patchGrip",
            "patchPart",
            "proximityConnect",
            "registerBrushMesh",
            "retargetFastener",
            "rotateSelection",
            "scaleSelection",
            "setActiveExample",
            "translateSelection",
            "worldRelocate",
            "setCamera",
            "setCamera2d",
            "setCamera3d",
            "setGridFactor",
            "setGridSnapEnabled",
            "setLodMode",
            "setSuggestionOffset",
            "toggleSun",
            "setSunAzimuth",
            "setSunElevation",
            "setSunIntensity",
            "setBrushPlacementContactTolerance",
            "setProximityRadius",
            "setChunkSize",
            "setPartKindWeight",
            "setGripKindWeight",
            "setGridVisible",
            "setGridSpacing",
            "setProjection",
            "setProjectionParam",
            "setGripShow",
            "setGripDirection",
            "setSelectableKind",
            "setLodAutomatic",
            "setLodDepthVariable",
            "setLodManual",
            "setTransformGumballFlag",
            "addTargetVolume",
            "deleteTargetVolume",
            "relocateTargetVolume",
            "setTargetVolumeFlag",
            "setTargetVolumeHidden",
            "setTargetVolumeLocked",
            "setVoxelDims"
        ]
    }
}

/// 📜️ The two framework-injected host-configuration verbs. Deliberately GENERIC proofs (no
/// `factory_type`): they are not app-owned retained tools — `Puzzle5dRetainedCommandJobFactory` never
/// claims them — they only need the wire admission and output budget `dispatch_action`'s
/// host-configuration branch asks for before it applies `host_configuration_mutation`. Without them the
/// shell's `setActiveTool`/`setActiveUtility` fall through to `admit_command_json` and fail closed with
/// `interactive-job.missing-factory`, so the fill TOOL could not even be armed.
pub(super) struct Puzzle5dHostConfigurationProofs;

impl Puzzle5dHostConfigurationProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle5dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle5d@1/*#editor",
        artifact_schema: "puzzle.5d",
        factory: "BoundedFirstStepCommandJobFactory",
        contract: ToolExecutionContract::resumable(8_192, 8, 1, 8_192, 7_500, 1, 1),
        tools: ["setActiveTool", "setActiveUtility"]
    }
}
//#endregion 📜️ToolProofs
//#endregion 🔖️PlayApp
