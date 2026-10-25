//! 🧰️ Retained commands: the tool id catalog, the retained command job factory with its publication contracts and the bounded first-step proofs.

use super::*;

//#region 🧵️RetainedCommands
pub const PUZZLE3D_RETAINED_TOOL_IDS: &[&str] = &[
    "openAddObjectDialog",
    "worldPointerDown",
    "setActiveExample",
    "setFillCount",
    "addTargetVolume",
    "acceptSuggestion",
    "addBrushObject",
    "addObjectKind",
    "createAttraction",
    "deleteAttraction",
    "deleteSelection",
    "deleteTargetVolume",
    "duplicateSelection",
    "exportSnapshot",
    "importSnapshot",
    "openImportSnapshot",
    "patchInspector",
    "rotateSelection",
    "scaleSelection",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "setTargetVolumeFlag",
    "translateSelection",
    "worldRelocate",
    "closeVortexSuggestions",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementRepeatLast",
    "engagementSubmit",
    "focusSelection",
    "hoverSuggestion",
    "openVortexSuggestions",
    "registerBrushMesh",
    "relocateTargetVolume",
    "selectSameKindSelection",
    "setBrushPlacementContactTolerance",
    "setCamera",
    "setChunkSize",
    "setGridSnapEnabled",
    "setGridSpacing",
    "setGridVisible",
    "setLodAutomatic",
    "setLodDepthVariable",
    "setLodManual",
    "setObjectKindWeight",
    "setProjection",
    "setProjectionParam",
    "setProximityRadius",
    "setSelectableKind",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setTransformGumballFlag",
    "setVortexDirection",
    "setVortexKindWeight",
    "setVortexShow",
    "setVoxelDims",
    "targetBrushSuggestions",
    "toggleSun",
];

/// 📏️ Raw wire bytes ONE retained 3d command may carry — one whole `importSnapshot` file, escaped, plus its envelope.
pub(super) const PUZZLE3D_IMPORT_RAW_BYTES: usize = semio_s_plugin_puzzle_retained::PUZZLE_IMPORT_RAW_BYTES;
const PUZZLE3D_IMPORT_DECODED_ITEMS: usize = 16_384;

pub struct Puzzle3dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
    pub contract: semio_framework::ToolExecutionContract,
}

impl Puzzle3dRetainedCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self {
            keys: PUZZLE3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(),
            contract: semio_framework::ToolExecutionContract::resumable(PUZZLE3D_IMPORT_RAW_BYTES, PUZZLE3D_IMPORT_DECODED_ITEMS, 1, semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_OUTPUT_BYTES, semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        }
    }
}

impl ToolJobFactory for Puzzle3dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Puzzle3dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Puzzle3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PUZZLE3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        self.contract
    }

    semio_framework_plugin_retained_command::retained_command_factory_admission!(maximum_raw_bytes: |payload| payload.maximum_raw_bytes, checkpoint_resume: true);

    
}

impl ArtifactOwnedToolJobFactory for Puzzle3dRetainedCommandJobFactory {
    type Owner = EditorApp<Puzzle3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PUZZLE3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "openAddObjectDialog", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFillCount", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "addTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "acceptSuggestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addBrushObject", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "addObjectKind", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "createAttraction", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteAttraction", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "deleteTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "duplicateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "exportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importSnapshot", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "openImportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "patchInspector", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSelectionLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setTargetVolumeFlag", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "worldRelocate", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "closeVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidate", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "cycleBrushCandidateBack", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementControlSelect", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient, ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "hoverSuggestion", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "openVortexSuggestions", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "registerBrushMesh", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "relocateTargetVolume", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "selectSameKindSelection", lanes: &[ArtifactToolPublicationLane::Interaction] },
        ArtifactToolPublicationContract { tool_id: "setBrushPlacementContactTolerance", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setChunkSize", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridSpacing", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodAutomatic", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodDepthVariable", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setLodManual", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setObjectKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setProjection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProjectionParam", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setProximityRadius", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSelectableKind", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setVortexDirection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setVortexKindWeight", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setVortexShow", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setVoxelDims", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "targetBrushSuggestions", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📜️ToolProofs
/// 📜️ The retained command catalog's own bounded first-step proofs — one per
/// `PUZZLE3D_RETAINED_TOOL_IDS` entry, all joined to the single concrete
/// `Puzzle3dRetainedCommandJobFactory`.
pub(super) struct Puzzle3dRetainedCommandProofs;

impl Puzzle3dRetainedCommandProofs {
    pub(crate) fn proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        Self::bounded_first_step_tool_proofs()
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle3d@1/*#editor",
        artifact_schema: "puzzle.3d",
        factory: "Puzzle3dRetainedCommandJobFactory",
        factory_type: Puzzle3dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(PUZZLE3D_IMPORT_RAW_BYTES, PUZZLE3D_IMPORT_DECODED_ITEMS, 1, semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_OUTPUT_BYTES, semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_STEP_MICROS, 1, 1),
        tools: [
            "openAddObjectDialog", "worldPointerDown", "setActiveExample", "setFillCount",
            "addTargetVolume",
            "acceptSuggestion", "addBrushObject", "addObjectKind", "createAttraction", "deleteAttraction", "deleteSelection", "deleteTargetVolume", "duplicateSelection", "exportSnapshot", "importSnapshot", "openImportSnapshot", "patchInspector", "rotateSelection", "scaleSelection", "setSelectionFlag", "setSelectionHidden", "setSelectionLocked", "setTargetVolumeFlag", "translateSelection", "worldRelocate", "relocateTargetVolume",
            "closeVortexSuggestions", "cycleBrushCandidate", "cycleBrushCandidateBack", "engagementAbort", "engagementControlSelect", "engagementInput", "engagementRepeatLast", "engagementSubmit", "focusSelection", "hoverSuggestion", "openVortexSuggestions", "registerBrushMesh", "selectSameKindSelection", "setBrushPlacementContactTolerance", "setCamera", "setChunkSize", "setGridSnapEnabled", "setGridSpacing", "setGridVisible", "setLodAutomatic", "setLodDepthVariable", "setLodManual", "setObjectKindWeight", "setProjection", "setProjectionParam", "setProximityRadius", "setSelectableKind", "setSunAzimuth", "setSunElevation", "setSunIntensity", "setTransformGumballFlag", "setVortexDirection", "setVortexKindWeight", "setVortexShow", "setVoxelDims", "targetBrushSuggestions", "toggleSun",
        ]
    }
}

/// 📜️ The two framework-injected host-configuration verbs. Deliberately GENERIC proofs (no
/// `factory_type`): they are not app-owned retained tools — `Puzzle3dRetainedCommandJobFactory` never
/// claims them and `retained_command_catalog_excludes_framework_owned_shared_actions` pins that — they
/// only need the wire admission and output budget `dispatch_action`'s host-configuration branch asks
/// for before it applies `host_configuration_mutation`'s single Config mutation.
pub(super) struct Puzzle3dHostConfigurationProofs;

impl Puzzle3dHostConfigurationProofs {
    pub(crate) fn proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        Self::bounded_first_step_tool_proofs()
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Puzzle3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.puzzle.puzzle3d@1/*#editor",
        artifact_schema: "puzzle.3d",
        factory: "BoundedFirstStepCommandJobFactory",
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 8, 1, 8_192, 7_500, 1, 1),
        tools: ["setActiveTool", "setActiveUtility"]
    }
}
//#endregion 📜️ToolProofs
