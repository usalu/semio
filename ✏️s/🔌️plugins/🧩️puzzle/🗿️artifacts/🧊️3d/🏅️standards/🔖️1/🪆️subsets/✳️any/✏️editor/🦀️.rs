//! 🧊️ Puzzle 3d play app — the plugin's 3d editor surface: its `ArtifactEditor` impl (dispatch-only), the
//! structural-twin scene_snapshot document model its command/panel/window nodes mutate and render, the
//! attraction resolver that keeps every attracted object's pose derived from its attracting root,
//! and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/🧊️main`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle3dPlaySnapshot` newtype over a bare
//! `serde_json::Value` scene_snapshot (see `crate::standards::v1::subsets::any::io::text::mutations`'s `🔖️ValueBridge`), not the typed
//! `Puzzle3dSnapshot` — the `Puzzle3dSceneSnapshot` model below is this app's own structural twin of it,
//! and each action emits the concrete typed mutation kinds of the gesture it carries (`🎮️commands/<group>`),
//! never a delta of two scenes.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_retained_action::{PUZZLE3D_RETAINED_PAYLOAD_SCHEMA, Puzzle3dAcceptSuggestionWork, Puzzle3dAddBrushObjectWork, Puzzle3dFocusSelectionWork, Puzzle3dInstanceBinding, Puzzle3dPrecomputeCommandWork, Puzzle3dTransformWork, Puzzle3dWindowCommandWork, puzzle3d_retained_extent, puzzle3d_retained_reduce};
use {semio_s_artifact_puzzle_3d_retained_document::Puzzle3dAddObjectKindWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dCreateAttractionWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dKindWeightWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dPatchInspectorWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dSetActiveExampleWork};
use semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action;
use {semio_s_artifact_puzzle_3d_editor_session::PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_view_session_key};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_DEFAULT_UTILITY, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_KIND, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_HOVER_CHANNEL, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_entity_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_fault_notices, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_action, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_argument, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_kind_ids, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_paste_operations, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_active_utility, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_play_snapshot, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_selected_objects_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_upsert_catalog_rows, semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot};
use semio_s_artifact_puzzle_3d_commands_document::world_relocate;
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime};
use semio_s_artifact_puzzle_3d_editor_modes as edit;
use semio_s_artifact_puzzle_3d_editor_modes::tools::fill as fill_tool;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use semio_s_artifact_puzzle_3d_editor_panels::{catalogue, artifact, inspection, settings as settings_panel};
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsOwner;
use semio_s_artifact_puzzle_3d_editor_presence as presence;
use semio_s_artifact_puzzle_3d_editor_presence::{Puzzle3dPresence, Puzzle3dPresenceMutation};
use semio_s_artifact_puzzle_3d_editor_terminology::{puzzle3d_labels, puzzle3d_localized, puzzle3d_localized_phrase, Puzzle3dLabels};
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;

use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, StepContext};
use semio_framework_plugin::panel_tab_element_id;
use semio_framework_plugin::panel_tab_first_draggable_element_id;
use semio_framework_plugin::window_element_id;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
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
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractionWrite;
use semio_framework_plugin::IntroductionDefinition;
use semio_framework_plugin::IntroductionInteraction;
use semio_framework_plugin::IntroductionPlacement;
use semio_framework_plugin::IntroductionStepDefinition;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
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
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_2d::compute::EngineHandles;
// 🎭️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (contract §2.1): `ArtifactEditor`
// replaces `ArtifactApp` as the authoring trait; `EditorApp<E>` is the runtime `ArtifactApp`
// adapter, needed by this file's own test harness (`VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>`).
// 🕹️ `InteractionView`'s canonical home is `semio_framework_plugin::app`; it is now also re-exported
// at that crate's root alongside `InteractionWrite`, and this path keeps the module-qualified spelling
// the other reference implementations (`process3d`, `generation3d`) use.

use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use semio_framework_plugin::app::{EphemeralEmit, InteractionView};
use std::collections::{HashMap, HashSet};

//#region 🔖️Document

//#region 🔖️Quaternions
pub use {semio_s_artifact_puzzle_3d_model::quat_from_axis_angle, semio_s_artifact_puzzle_3d_model::quat_mul, semio_s_artifact_puzzle_3d_model::quat_rotate_vector};

//#endregion 🔖️Quaternions

//#region 🔖️PlayApp

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
const PUZZLE3D_IMPORT_RAW_BYTES: usize = semio_s_plugin_puzzle_retained::PUZZLE_IMPORT_RAW_BYTES;
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
            return Err((ToolJobFactoryError::new("Puzzle 3d retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
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

//#region 📬️StorePreparation
const PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 32_768;

struct Puzzle3dConfigStorePreparation {
    owners: store::OneItemOwners<Puzzle3dConfig, Puzzle3dConfigMutation>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    phase: u8,
    cancelled: bool,
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle3dConfigStorePreparationFactory;

/// 🌉️ `serde_json::to_writer` streamed into a byte-counting `Write` sink so an oversize config could
/// abort mid-encode without materializing it; the in-house `semio_framework_pack_json::to_json_string` has no
/// streaming writer, so this measures the fully-encoded string's byte length against the same
/// `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES` bound instead — same bound, same error, one buffer instead
/// of zero (`Puzzle3dConfig` is a small, fixed-shape record, never large enough for this to matter).
fn puzzle3d_config_store_bounded_bytes(value: &Puzzle3dConfig) -> Result<usize, String> {
    let encoded = to_json_string(value);
    if encoded.len() > PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Puzzle3d Config Store root exceeds its fixed envelope".to_string());
    }
    Ok(encoded.len())
}

/// 📏️ The exact retained byte cost of one Config mutation, derived from its own encoded payload and
/// bounded by the single `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES` envelope every Config lane publication
/// obeys. Every `Puzzle3dConfigMutation` variant is admissible: an allowlist here silently killed the
/// whole `Puzzle3dScalarConfigWork` family (camera, projection, sun, LOD, grid, selectable kinds,
/// proximity radius, chunk size, voxel dims, gumball flags, vortex show/direction, contact tolerance,
/// suggestion menu, engagement input) plus both kind-weight routes, because
/// `Puzzle3dConfigStorePreparation::advance` rejects whatever this returns `None` for and the
/// operation's publication lease is cancelled behind it. Only an oversize payload is refused now, and
/// `Snapshot`'s cost stays the whole-config cost it always was — its payload *is* the config.
fn puzzle3d_config_store_mutation_bytes(mutation: &Puzzle3dConfigMutation) -> Option<usize> {
    let encoded = to_json_string(mutation).len();
    (encoded <= PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES).then_some(encoded)
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::{Mutation as _, MutationDiff as _};
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle3d Config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation lost its mutation owner"))?;
                if puzzle3d_config_store_mutation_bytes(mutation).is_none() {
                    return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Puzzle3d Config preparation rejected its exact mutation envelope"));
                }
                let completed_bytes = puzzle3d_config_store_bounded_bytes(base.get()).map_err(|message| ValueError::new(ValueRefusalKind::OwnershipLimit, message))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle3d Config mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = completed_bytes;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle3dConfig, Puzzle3dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle3dConfig, Puzzle3dConfigMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle3dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Puzzle3dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle3d Config preparation rejected its lane".into());
        }
        let retained_bytes = puzzle3d_config_store_mutation_bytes(mutation).ok_or_else(|| "Puzzle3d Config preparation rejected its exact mutation".to_string())?;
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
    }

    fn begin_demand(&self, _mutation: &Puzzle3dConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle3dConfigStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dConfig, Puzzle3dConfigMutation, Puzzle3dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dConfig, Puzzle3dConfigMutation, Puzzle3dConfigMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle3d Config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle3dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

// 🧩️ The Artifact-lane (`Puzzle3dPlaySnapshot`/`Puzzle3dMutation`) sibling of the preparation
// factory above — mirrors `Puzzle5dStorePreparationFactory`
// (`🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`), the one other puzzle-family
// app whose retained commands publish to the Artifact lane through this same trait. Unlike the
// Config preparation above, an arbitrary `Puzzle3dMutation` variant is admitted generically via
// `protocol::Mutation`/`protocol::MutationDiff` rather than an allowlist match, because
// `setActiveExample`'s work loop (`Puzzle3dSetActiveExampleWork`) emits many different mutation
// kinds (delete/create object, attraction, target volume, reference, compatibility, domain,
// catalogs). Each kind is batched to `PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK` and the Complete emit
// is ONE document-replacement edit (no coalesce key), not one store commit per item.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle3dArtifactStorePreparationFactory;

struct Puzzle3dArtifactStorePreparation {
    owners: store::OneItemOwners<Puzzle3dPlaySnapshot, Puzzle3dMutation>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    phase: u8,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle3dMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): a selection
    /// leaf one setter per changed pose field of every record its attraction re-solve carries along, a scaling one per
    /// target, a removal the record and the attractions it severs.
    fn preflight(&self, mutation: &Puzzle3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle3d Artifact preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<Puzzle3dPlaySnapshot, _>(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &Puzzle3dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle3dArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dPlaySnapshot, Puzzle3dMutation, Puzzle3dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle3dPlaySnapshot, Puzzle3dMutation, Puzzle3dMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle3dArtifactStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::{Mutation as _, MutationDiff as _};
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle3d Artifact preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation lost its mutation owner"))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle3d Artifact mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = 1;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle3d Artifact preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle3dPlaySnapshot, Puzzle3dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle3dPlaySnapshot, Puzzle3dMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️StorePreparation

//#region 📜️ToolProofs
/// 📜️ The retained command catalog's own bounded first-step proofs — one per
/// `PUZZLE3D_RETAINED_TOOL_IDS` entry, all joined to the single concrete
/// `Puzzle3dRetainedCommandJobFactory`.
struct Puzzle3dRetainedCommandProofs;

impl Puzzle3dRetainedCommandProofs {
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
struct Puzzle3dHostConfigurationProofs;

impl Puzzle3dHostConfigurationProofs {
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

/// ♻️ The owners one `Puzzle3dClipboardJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dClipboardOwners {
    pub args: Option<semio_framework_value::DslValue>,
    interaction: Option<semio_framework::InteractionState>,
    hover: Option<semio_framework_plugin::app::InteractionHoverState>,
    pub tool_id: String,
    pub snapshot: Option<std::sync::Arc<Puzzle3dPlaySnapshot>>,
    raw_wire: Vec<u8>,
    completion: Option<semio_framework_plugin::app::ArtifactToolCompletion<EditorApp<Puzzle3dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle3dPlayApp>>>,
}

/// 📋️ One-step reserved copy/cut/paste job — the framework route is an empty stub unless the app owns this producer.
struct Puzzle3dClipboardJob {
    pub tool_id: String,
    pub snapshot: Option<std::sync::Arc<Puzzle3dPlaySnapshot>>,
    raw_wire: Vec<u8>,
    pub args: Option<semio_framework_value::DslValue>,
    interaction: Option<semio_framework::InteractionState>,
    hover: Option<semio_framework_plugin::app::InteractionHoverState>,
    completion: Option<semio_framework_plugin::app::ArtifactToolCompletion<EditorApp<Puzzle3dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle3dPlayApp>>>,
    outbox: semio_s_plugin_puzzle_jobs::JobOutbox,
    closing: bool,
    owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dClipboardOwners>,
}

impl Puzzle3dClipboardJob {
    pub fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle3dPlayApp>>) -> Self {
        let ArtifactReservedToolInput::Action { args, interaction, hover } = request.input else { unreachable!("clipboard routes carry action input") };
        Self { tool_id: request.tool_id, snapshot: Some(request.snapshot), raw_wire: request.raw_wire, args, interaction: Some(interaction), hover: Some(hover), completion: Some(request.completion), rejection: None, outbox: Default::default(), closing: false, owners: Default::default() }
    }

    pub fn emit(&mut self) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation, NoDraftMutation> {
        let (Some(interaction), Some(hover)) = (self.interaction.as_ref(), self.hover.as_ref()) else {
            return Emit::default();
        };
        let args = self.args.as_ref();
        let Some(snapshot) = self.snapshot.clone() else { return Emit::default() };
        let scene_snapshot = puzzle3d_scene_snapshot_from_play_snapshot(snapshot.as_ref());
        let marks = Puzzle3dInteractionSnapshot::from_state(&interaction, &hover);
        match self.tool_id.as_str() {
            // 🧾️ `copy` answers a selection it cannot resolve with `Emit::default()`, i.e. ZERO effects —
            // the same shape as "the hotkey never reached the guest" (ticket 26/09/02/PUZZLE-3D-END-TO-END
            // wave B47 §4).
            "copy" => {
                let selected = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
                match puzzle3d_copy_fragment_from(&scene_snapshot, selected) {
                    Ok(fragment) => Emit { effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() },
                    Err(_) => Emit::default(),
                }
            }
            "cut" => {
                let objects = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
                let Ok(fragment) = puzzle3d_copy_fragment_from(&scene_snapshot, objects) else { return Emit::default() };
                let mutations = puzzle3d_cut_operations_from(snapshot.typed(), &marks);
                Emit { artifact_mutations: mutations, effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() }
            },
            "paste" => {
                let Some(args) = args else { return Emit::default() };
                let Some(fragment) = args.get("fragment").and_then(|value| FromValue::from_value(value.clone()).ok()) else { return Emit::default() };
                let placement = PastePlacement::default();
                match puzzle3d_paste_operations(&fragment, &placement) {
                    Ok(mutations) => Emit { artifact_mutations: mutations, ..Default::default() },
                    Err(_) => Emit::default(),
                }
            }
            _ => Emit::default(),
        }
    }
}

impl InteractiveJob for Puzzle3dClipboardJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            semio_s_plugin_puzzle_jobs::OutboxPhase::Building => return self.outbox.advance(cx),
            semio_s_plugin_puzzle_jobs::OutboxPhase::Delivered | semio_s_plugin_puzzle_jobs::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            semio_s_plugin_puzzle_jobs::OutboxPhase::Idle => {}
        }
        let turn = self.turn(cx);
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle3dClipboardOwners { args: self.args.take(), interaction: self.interaction.take(), hover: self.hover.take(), tool_id: std::mem::take(&mut self.tool_id), snapshot: self.snapshot.take(), raw_wire: std::mem::take(&mut self.raw_wire), completion: self.completion.take(), rejection: self.rejection.take() });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.begin_close();
        semio_s_plugin_puzzle_jobs::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl Puzzle3dClipboardJob {
    pub fn turn(&mut self, cx: &StepContext<'_>) -> semio_s_plugin_puzzle_jobs::JobTurn {
        if self.closing || cx.is_cancelled() {
            return semio_s_plugin_puzzle_jobs::JobTurn::Cancelled;
        }
        if !self.raw_wire.is_empty() {
            return semio_s_plugin_puzzle_jobs::JobTurn::Prepare(std::mem::take(&mut self.raw_wire));
        }
        let emit = self.emit();
        let Some(completion) = self.completion.as_ref() else { return semio_s_plugin_puzzle_jobs::JobTurn::Fault(b"puzzle3d-clipboard-completion-owner".to_vec()) };
        if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
            self.rejection = Some(rejected);
            return semio_s_plugin_puzzle_jobs::JobTurn::Fault(b"puzzle3d-clipboard-completion-rejected".to_vec());
        }
        semio_s_plugin_puzzle_jobs::JobTurn::Complete
    }
}

impl ArtifactReservedJob for Puzzle3dClipboardJob {}

/// 🧩️ The puzzle-3d play editor: the `ArtifactEditor` the framework mounts as `EditorApp<Puzzle3dPlayApp>`. It holds no
/// state of its own — every call checks the document instance's [`Puzzle3dSession`] out of the session registry, and
/// the persisted document lives in the wrapping `VcsArtifactApp`'s operation store.
#[derive(Clone, Copy, Debug, Default)]
pub struct Puzzle3dPlayApp;

impl ArtifactEditor for Puzzle3dPlayApp {
    const DIALECT: Dialect = semio_s_artifact_puzzle_3d_model::PUZZLE3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA;
    type Snapshot = Puzzle3dPlaySnapshot;
    type Mutation = Puzzle3dMutation;
    type Config = Puzzle3dConfig;
    type ConfigMutation = Puzzle3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle3dPresence;
    type PresenceMutation = Puzzle3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle3dCommand;

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_config(registry)
    }

    /// 🎮️ The six framework interaction verbs, answered out of this app's OWN scope table
    /// ([`Puzzle3dScopeClass::Interaction`]) instead of the framework's blanket `UiDirtyScope::Full`: a
    /// hover repaints the world body alone, a pick adds the inspector + outliner rows + history panel,
    /// a mode/granularity switch only the window's Select chrome. A verb that touched any domain this
    /// app does not declare — or none at all — answers `None`, and the framework keeps its widest scope.
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        let declared = !domains.is_empty() && domains.iter().all(|domain| *domain == PUZZLE3D_INTERACTION_DOMAIN);
        declared.then(|| puzzle3d_scope(Puzzle3dScopeClass::Interaction(verb)))
    }

    /// 🪧️ A history-edit reference chip names its world entity as the outliner does ([`puzzle3d_entity_label`]).
    fn entity_label(snapshot: &Puzzle3dPlaySnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        puzzle3d_entity_label(snapshot.typed(), kinds, id)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_transient(registry)
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    /// 🧽️ puzzle3d owns no draft lane, but `VcsArtifactApp::close_step` still walks it: every one of
    /// its seven owned lanes is mandatory, and the `interaction-store` lane behind this one never runs
    /// until the draft/presence/transient disposers exist.
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(presence::Puzzle3dPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(presence::Puzzle3dPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(presence::puzzle3d_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    /// 🧭️ The two framework-injected host-configuration verbs, resolved to one event-sourced Config
    /// mutation each instead of a retained tool operation. The shell owns their session state and then
    /// forwards the resolved value here so the app can clear/prepare its scratch
    /// (`🏛️ShellHost/🟦️.tsx`'s `SET_ACTIVE_TOOL_ACTION_ID`/`SET_ACTIVE_UTILITY_ACTION_ID` branches);
    /// without this hook both verbs fell through to `admit_command_json` and failed closed with
    /// `interactive-job.missing-factory`, so the fill tool could not even be selected.
    ///
    /// 🧰️ `setActiveUtility` needs no variant of its own: puzzle3d's active utility IS host view state
    /// (`puzzle3d_scene_active_utility` reads `ViewModel.active_utility_by_window_id`), and the only
    /// app-owned state a utility switch invalidates is that window's engagement input.
    fn host_configuration_mutation(_action: &str, _args: Option<&semio_framework_value::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok(None)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle3dRetainedCommandProofs::bounded_first_step_tool_proofs();
        proofs.extend(Puzzle3dHostConfigurationProofs::bounded_first_step_tool_proofs());
        proofs
    }

    /// ⏯️ The fill tool's run and finalize revalidation jobs and the brush's read-only suggestions run
    /// (`📋️tool-run-contract.md` §3.7).
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        match request.tool_id {
            utilities::brush::UTILITY_ID => utilities::brush::build_run_job(request),
            _ => Ok(None),
        }
    }

    /// 🎯️ The fill tool's run and finalize revalidation jobs, retargeted in place across base and count changes.
    fn build_retargetable_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<Box<dyn semio_framework_plugin::ToolRunRetargetableJob<Self::Config>>>, Fault> {
        fill_tool::build_run_job(request)
    }

    /// 📢️ The localized notices of this guest's refusal codes ([`puzzle3d_fault_notices`]).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        puzzle3d_fault_notices()
    }

    /// 🧠️ The instance-retained owner the brush suggestions link lives in.
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Puzzle3dInstanceOperationOwner::default())
    }

    /// 🚦️ Starts, wakes or aborts the brush suggestions run for the vortex the link points at
    /// ([`utilities::brush::run_effects`]).
    fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>, _view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(utilities::brush::run_effects(&mut owner.brush_suggestions, doc.tool_run()))).unwrap_or_default()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle3dRetainedCommandJobFactory::new(&controller))?;
        Ok(())
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PUZZLE3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "puzzle3d-command-tool-mismatch"));
        }
        let tool_id = request.command.action_id();
        let instance = Puzzle3dInstanceBinding { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id.clone(), owner: request.instance_operation_owner.clone() };
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Self>>> = match tool_id {
            "translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => Box::new(Puzzle3dTransformWork::new(tool_id, request.authoring_seed.clone())),
            "patchInspector" if puzzle3d_inspector_origin_nudge(request.command.args()).is_some() => Box::new(Puzzle3dTransformWork::new(tool_id, request.authoring_seed.clone())),
            "patchInspector" => Box::new(Puzzle3dPatchInspectorWork::default()),
            "createAttraction" => Box::new(Puzzle3dCreateAttractionWork::default()),
            "setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default()),
            "addBrushObject" => Box::new(Puzzle3dAddBrushObjectWork::default()),
            "addObjectKind" => Box::new(Puzzle3dAddObjectKindWork::default()),
            "engagementAbort" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance).with_tool_run(request.context.tool_run().cloned())),
            "engagementRepeatLast" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance)),
            "engagementSubmit" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance)),
            "setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle3dKindWeightWork::new(tool_id)),
            "acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default().bound(&instance)),
            "cycleBrushCandidate" | "cycleBrushCandidateBack" | "registerBrushMesh" | "setFillCount" => Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id).bound(&instance)),
            "focusSelection" => Box::new(Puzzle3dFocusSelectionWork::default()),
            "setCamera"
            | "setProjection"
            | "setProjectionParam"
            | "toggleSun"
            | "setSunAzimuth"
            | "setSunElevation"
            | "setSunIntensity"
            | "setLodAutomatic"
            | "setLodDepthVariable"
            | "setLodManual"
            | "setGridVisible"
            | "setGridSnapEnabled"
            | "setGridSpacing"
            | "setSelectableKind"
            | "setProximityRadius"
            | "setChunkSize"
            | "setVoxelDims"
            | "setTransformGumballFlag"
            | "setVortexShow"
            | "setVortexDirection"
            | "setBrushPlacementContactTolerance"
            | "openVortexSuggestions"
            | "closeVortexSuggestions"
            | "targetBrushSuggestions"
            | "hoverSuggestion"
            | "engagementControlSelect"
            | "engagementInput" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance)),
            "exportSnapshot" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance)),
            "openImportSnapshot" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance)),
            "importSnapshot" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance)),
            "addTargetVolume" => Box::new(Puzzle3dWindowCommandWork::new(tool_id).bound(&instance)),
            "worldPointerDown" => Box::new(semio_s_plugin_puzzle_retained::NoopPuzzleCommandWork::new(tool_id)),
            _ => Box::new(semio_s_plugin_puzzle_retained::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce::<Puzzle3dPlayApp>, puzzle3d_retained_extent)),
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
            Puzzle3dCommand::action_id,
            PUZZLE3D_IMPORT_RAW_BYTES,
            semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle3d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle3dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_puzzle_3d_editor_config::schema::app_schema_descriptor())
    }

    /// 🚀️ The `default_scene_snapshot()` (Concrete Forest) document, nothing else.
    ///
    /// 🐛️ This used to pre-warm `NAKAGIN_EXAMPLE_SNAPSHOT` first — 128 755 B of DSL parsed into a typed
    /// scene_snapshot that the boot document never reads, paid by every mount before its first frame. The
    /// static is a `LazyLock`: the example that needs it forces it when it is actually switched to.
    /// It then built a scene and synced a whole precompute session (mesh fallbacks, collision scene) into a
    /// throwaway `Puzzle3dPlayApp` dropped on return: every rendering app syncs its own session before its
    /// first body (`with_puzzle3d_session_for`), so that work reached no one — while every `codec` call of a hub
    /// creating or validating a document paid for it in the interpreter.
    fn initial_snapshot() -> Puzzle3dPlaySnapshot {
        Puzzle3dPlaySnapshot::new(puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("authored initial snapshot admits"))
    }

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::ThreeD, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        puzzle3d_copy_fragment(doc, interaction)
    }

    fn cut_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle3dMutation> {
        puzzle3d_cut_operations(doc, interaction)
    }

    fn paste_operations(_doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
        puzzle3d_paste_operations(fragment, placement)
    }

    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if !matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") {
            return Ok(None);
        }
        Ok(Some(ArtifactReservedToolJob::new(Puzzle3dClipboardJob::new(request))))
    }

    /// 🏷️ Maps each `Puzzle3dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle3dCommand) -> &'static str {
        command.action_id()
    }

    /// 🎯️ Maps the host's transitional `{action,args}` wire onto Puzzle 3D's closed command
    /// enum until React and wgpu send `OpBinary` command bytes directly.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        if let Some(flag) = puzzle3d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle3d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        Puzzle3dCommand::from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 3D action '{action}'")))
    }

    /// 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle3dCommand`.
    fn handle(
        command: &Puzzle3dCommand,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, Fault> {
        let interaction = Puzzle3dInteractionSnapshot::from_interaction(interaction);
        let window = window_ownership::config_from_view(cfg);
        let runtime = window_ownership::runtime(cfg.snapshot, &window, &window_ownership::Puzzle3dWindowTransient::default(), view_state);
        let window_id = puzzle3d_addressed_window_id(view_state, None, command.window_id(), &runtime.window_ids);
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        Ok(with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |session| puzzle3d_handle_action::<Puzzle3dPlayApp>(session, authoring_seed, command, Some(window_id), doc.snapshot, &runtime, view_state, &interaction).0))
    }

    /// 🕹️ `vortex` domain topology (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// objects and every root-level entity kind (vortex marker, attraction, target volume, reference,
    /// catalogue kind) as one flat forest, except object-owned vortex markers whose parent is the
    /// object they mark — the one real nesting relationship this app's document carries, replacing
    /// what `hoveredVortexFullId`'s ad hoc highlighting used to do by hand.
    /// 🔭️ Read off the SAME projection every render publishes (`puzzle3d_scene_snapshot_from_projection`),
    /// never a second decoding of the document: `protocol::validate_state` prunes every selected id
    /// this topology does not contain, so a universe narrower than what the viewport paints makes a
    /// pick land on the leftover and then vanish — with no error anywhere. One authority is what
    /// makes "the user can see it" and "the user can pick it" the same statement
    /// (26/09/02/PUZZLE-3D-END-TO-END wave B9 lane 2).
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS.with(|counter| counter.set(counter.get().saturating_add(1)));
        let snapshot = puzzle3d_scene_snapshot_from_document(doc.snapshot.typed());
        let mut ordered = Vec::new();
        for object in &snapshot.objects {
            ordered.push(semio_framework_plugin::TopologyNode { id: object.id.clone(), granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), parent: None });
            for vortex in &object.vortices {
                ordered.push(semio_framework_plugin::TopologyNode { id: puzzle3d_vortex_full_id(&object.id, &vortex.id), granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), parent: Some(object.id.clone()) });
            }
        }
        for attraction in &snapshot.attractions {
            ordered.push(semio_framework_plugin::TopologyNode { id: attraction.id.clone(), granularity: PUZZLE3D_GRANULARITY_ATTRACTION.into(), parent: None });
        }
        for volume in &snapshot.target_volumes {
            ordered.push(semio_framework_plugin::TopologyNode { id: volume.id.clone(), granularity: PUZZLE3D_GRANULARITY_TARGET_VOLUME.into(), parent: None });
        }
        for reference in &snapshot.references {
            ordered.push(semio_framework_plugin::TopologyNode { id: reference.id.clone(), granularity: PUZZLE3D_GRANULARITY_REFERENCE.into(), parent: None });
        }
        for kind in puzzle3d_kind_ids(&snapshot, "objects") {
            ordered.push(semio_framework_plugin::TopologyNode { id: kind, granularity: PUZZLE3D_GRANULARITY_KIND.into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PUZZLE3D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered });
        semio_framework_plugin::InteractionTopology { domains }
    
})())
}

    /// 🔌️ Declares puzzle3d's typed media I/O surface — the implicit document ports plus the flagship
    /// `kit:in` seam: an input port accepting `Kit×Type` media tagged `kit.catalog`, fanning IN from
    /// potentially many producers (`multiplicity: Many`).
    fn io() -> Option<AppIo> {
        Some(puzzle3d_io())
    }

    /// 🎞️ `kit:in` seam: normalizes an incoming `kit.catalog` fragment (`objectKinds`/`vortexKinds`/
    /// `cableKinds`/`attractionKinds`/`kindCompatibility`) into puzzle3d's own `meta.kind_catalogs`
    /// vocabulary (`objects`/`vortices`/`cables`/`attractions`) and upserts it (keyed by row `id`,
    /// deterministic/order-independent — safe for `multiplicity: Many` fan-in) via the same
    /// concrete `replace-kind-catalogs` and `connect-`/`disconnect-kind-compatibility` kinds, so this never mutates
    /// anything directly — only real, undoable operations.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "kit:in" {
            return Err(MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "kit:in only accepts a Structured (JSON) payload".into()));
        };
        let fragment: Value = parse(json.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let before = doc.snapshot.typed();
        let scene_snapshot = puzzle3d_scene_snapshot_from_document(before);

        let mut catalogs: semio_framework_value::DslValue = scene_snapshot.meta.kind_catalogs.clone().unwrap_or_else(|| {
            semio_framework_value::DslValue::object([
                ("objects".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                ("vortices".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                ("cables".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                ("attractions".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
            ])
        });
        puzzle3d_upsert_catalog_rows(&mut catalogs, "objects", fragment.get("objectKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "vortices", fragment.get("vortexKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "cables", fragment.get("cableKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "attractions", fragment.get("attractionKinds"));
        let kind_catalogs: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs> = Some(<semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs as FromValue>::from_value(catalogs).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?);
        let mut operations = Vec::new();
        if kind_catalogs != before.meta.kind_catalogs {
            operations.push(semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::replace_kind_catalogs(kind_catalogs));
        }

        if let Some(incoming_compat) = fragment.get("kindCompatibility").and_then(Value::as_array) {
            let mut rows = before.meta.kind_compatibility.clone();
            for incoming in incoming_compat {
                let row = <semio_s_artifact_puzzle_3d_model::Puzzle3dKindCompatibility as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(incoming)).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let connect = semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, None);
                match rows.iter().position(|entry| entry.source == row.source && entry.target == row.target) {
                    None => {
                        operations.push(connect);
                        rows.push(row);
                    }
                    Some(index) if rows[index] != row => {
                        operations.push(semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
                        operations.push(connect);
                        rows[index] = row;
                    }
                    Some(_) => {}
                }
            }
        }

        Ok(Emit::mutations(operations))
    }

    /// 🕹️ Plain `render` is the no-interaction entry point the framework only reaches through
    /// `render_with_request_context`'s default body — this app overrides that, so every real render
    /// carries a live selection. Kept as a thin delegate against an empty snapshot rather than a
    /// second body, so there is exactly one render implementation
    /// ([`Puzzle3dPlayApp::render_body`]).
    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(None, body_key, doc, cfg, view_state, &window_ownership::Puzzle3dWindowTransient::default(), &Puzzle3dInteractionSnapshot::default())
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): resolves the live `vortex` domain
    /// (selection + `"pointer"` hover) once per render and threads it into the whole body — the world
    /// scene's selection/gumball/vortex-marker paint, the suggestion popup and the inspection panel's
    /// per-entity field groups all read it instead of ever storing selection in this app again.
    fn render_with_request_context(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let window_transient = window_ownership::transient_from_view(transient);
        Self::render_body(Some(owner), body_key, doc, cfg, view_state, &window_transient, &Puzzle3dInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    /// 🕹️ The context menu reads the AUTHORITATIVE framework-owned selection, not only the
    /// client-supplied `request.surface.selection`: `World3dHost` only ever puts its painted object
    /// ids (and component ids) into that field, so a selected vortex / target volume / reference would
    /// otherwise never reach a menu row. The surface's own hits/selection still win when present, so a
    /// right-click on an unselected entity still targets what was clicked.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle3dInteractionSnapshot::from_interaction(interaction), registry)
    }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |app| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = main::scene_for(doc.snapshot, &runtime, Some(view_state), window_id);
            HashMap::from([(window_id.to_string(), main::engagement(&envelope, labels, doc.tool_run()))])
        })
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), Some(view_state));
        with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |app| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = main::scene_for(doc.snapshot, &runtime, Some(view_state), window_id);
            HashMap::from([(window_id.to_string(), main::engagement(&envelope, labels, doc.tool_run()))])
        })
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        Self::window_measures_body(doc, cfg, view_state)
    }

    fn tool_measures(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |app| {
            let wid = view_state.window_id.as_deref().unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = main::scene_for(doc.snapshot, &runtime, Some(view_state), wid);
            HashMap::from([(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.to_string(), fill_tool::measures(&envelope, labels, doc.tool_run()))])
        })
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle3dInteractionSnapshot::default(), registry)
    }
}

//#endregion 🔖️PlayApp

//#region 🔖️Manifest
/// 🔌️ Declares puzzle3d's typed media I/O surface — the implicit document ports plus the flagship
/// `kit:in` seam: an input port accepting `Kit×Type` media tagged `kit.catalog`, fanning IN from
/// potentially many producers (`multiplicity: Many`).
///
/// 🎯️ A free function, not an inline `ArtifactApp::io()` body, because BOTH the trait method and the
/// `AppBuilder` need it: the trait method serves the runtime, while `.io(..)` on the builder is what
/// puts `document_schema` into the published `AppDefinition`. Inlining it in only the trait method
/// left the manifest's `io` empty, so a host reading the manifest could not route a document to this
/// surface at all — caught by the demonstrator bundle's `every_pane_declares_a_document_schema`.
pub fn puzzle3d_io() -> AppIo {
    ::semio_framework_async::poll::resolve_ready(
        ::semio_framework_async::poll::resolve_ready(AppIo::from_artifact(
            "puzzle.3d",
            MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
            semio_framework_plugin::ArtifactPresentation { id: "3d.puzzle".into(), name: "3D Puzzle".into(), dimension: "3d".into(), component_kind: "puzzle3d".into() },
        ))
        .with_ports(vec![MediaPortSpec {
            id: "kit:in".into(),
            label: "Kit Catalog".into(),
            direction: MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: PortMultiplicity::Many,
        }]),
    )
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain declaration —
/// one granularity per previously-distinct `Puzzle3dSelection` bag (object/vortex/attraction/
/// targetVolume/reference) plus `kind` for the catalogue's rows. `Topology` hierarchy (see
/// `Puzzle3dPlayApp::interaction_topology`) makes the object→vortex-marker nesting available for a
/// future transitive hover; every other granularity is a flat root today.
fn puzzle3d_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, label: LocalizedLabel, icon: &str| GranularityDefinition { id: id.into(), label, icon_id: icon.into() };
    InteractionDefinition {
        id: PUZZLE3D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Vortex", "Vortex"),
        granularities: vec![
            granularity(PUZZLE3D_GRANULARITY_OBJECT, puzzle3d_localized(|l| l.object), "box"),
            granularity(PUZZLE3D_GRANULARITY_VORTEX, puzzle3d_localized(|l| l.vortex), "sparkles"),
            granularity(PUZZLE3D_GRANULARITY_ATTRACTION, puzzle3d_localized(|l| l.attraction), "link"),
            granularity(PUZZLE3D_GRANULARITY_TARGET_VOLUME, puzzle3d_localized(|l| l.target_volume), "box-select"),
            granularity(PUZZLE3D_GRANULARITY_REFERENCE, puzzle3d_localized(|l| l.reference), "image"),
            granularity(PUZZLE3D_GRANULARITY_KIND, puzzle3d_localized(|l| l.kind), "layers"),
        ],
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![PUZZLE3D_HOVER_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🗂️ Fixed ceiling on the object-kind rows the `addObjectKind` arg form and the "Add Object" dialog
/// offer — the manifest is minted once per process, so this select is built eagerly and must be
/// bounded independently of how wide a catalog a future example declares.
pub const PUZZLE3D_OBJECT_KIND_OPTIONS_MAX: usize = 64;

/// 🗂️ The object kinds the "Add Object" dialog (and the standalone `addObjectKind` arg form) offers —
/// read from the declared examples' own `meta.kindCatalogs`, exactly the rows
/// `📌️panels/🛍️catalogue` renders, deduplicated across examples in catalog order. `AppDefinition`
/// is built once per process and never sees the live document, and `setActiveExample` is this
/// editor's only document source (see `📓️2026-09-09-user-feature-checklist.md` §24: there is no
/// import), so the union of the declared example catalogs IS the reachable kind set — not the single
/// literal `"Object"` option this select used to hardcode, which could not add a single real kind of
/// either example.
///
/// 🖐️ Those rows are AUTHORED below rather than derived, in the two shipped examples' own catalog
/// order, and pinned to the documents by `shipped_object_kinds_are_the_two_examples_own_catalog_rows`.
///
/// 🐛️ Deriving it dereferenced `CONCRETE_FOREST_EXAMPLE_SNAPSHOT` and `NAKAGIN_EXAMPLE_SNAPSHOT`,
/// i.e. parsed 136 395 B of authored DSL into two typed `Puzzle3dSceneSnapshot`s — on the
/// `AppDefinition` path, which is the `describe()` path AND every actor boot. Measured natively on
/// 2026-09-22 (slice PZ2, `🗑️generated/pz2-native-profile-*.txt`): `create_puzzle3d_app()` cost
/// 231 ms cold against ~1 ms with those statics warm, so ALL of it was this one select.
pub const PUZZLE3D_SHIPPED_OBJECT_KINDS: &[(&str, &str)] = &[
    ("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
    ("Hexagonal Cut Concrete Forest Right", "Hexagonal Cut Concrete Forest Right"),
    ("Base", "Base"),
    ("Bridge", "Bridge"),
    ("Capital", "Capital"),
    ("Capsule With Balcony Backslash", "Capsule With Balcony Backslash"),
    ("Capsule With Balcony J", "Capsule With Balcony J"),
    ("Capsule With Balcony L", "Capsule With Balcony L"),
    ("Capsule With Balcony P", "Capsule With Balcony P"),
    ("Capsule With Balcony S", "Capsule With Balcony S"),
    ("Capsule With Balcony Slash", "Capsule With Balcony Slash"),
    ("First Storey Tambour", "First Storey Tambour"),
    ("Last Storey Tambour", "Last Storey Tambour"),
    ("Tambour", "Tambour"),
];

/// 🗂️ The `objectKind` select's options, mapped from [`PUZZLE3D_SHIPPED_OBJECT_KINDS`].
fn puzzle3d_object_kind_options() -> Vec<ActionArgOption> {
    PUZZLE3D_SHIPPED_OBJECT_KINDS.iter().take(PUZZLE3D_OBJECT_KIND_OPTIONS_MAX).map(|(id, label)| ActionArgOption::new(*id, LocalizedLabel::data(*label))).collect()
}

/// 🗂️ The kind the `objectKind` select stages when nothing is picked — the first catalog row, never a
/// literal id no catalog declares.
fn puzzle3d_default_object_kind(options: &[ActionArgOption]) -> String {
    options.first().map(|option| option.value.clone()).unwrap_or_default()
}

/// 🗂️ The one `objectKind` select both the standalone `addObjectKind` arg form and the "Add Object"
/// dialog declare — built twice from the same catalog so the two forms can never drift apart.
fn puzzle3d_object_kind_arg() -> ActionArgDef {
    let options = puzzle3d_object_kind_options();
    let default = puzzle3d_default_object_kind(&options);
    ActionArgDef::select("objectKind", puzzle3d_localized(|l| l.kind), options).default_value(&default)
}

/// 🎭️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (contract §2.4): `Editor::builder`
/// derives the surface id from `DIALECT`+`ROLE` (no hand-written label/id), returns `AppDefinition`
/// (not `App`), and has no `.example(...)`/`.workflow(...)` methods — the old
/// `.example(PUZZLE3D_EXAMPLE_CONCRETE_FOREST, …)` / `.example(PUZZLE3D_EXAMPLE_NAKAGIN, …)` /
/// `.workflow("puzzle3d", "Puzzle 3D", "model")` calls this builder used to end with are DROPPED
/// here, not ported (contract §7.4's `App { definition, examples }` split: `.editor::<E>(def)` only
/// takes the definition, so `AppBuilder`'s own `examples` vec is discarded either way) — flagged as a
/// known gap for the coordinator, not silently lost.
pub fn create_puzzle3d_app() -> semio_framework_plugin::AppDefinition {
    let envelope = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    Editor::builder(semio_s_artifact_puzzle_3d_model::PUZZLE3D_DIALECT)
            .document(["semio", "puzzle", "3d"])
            .artifact_kind(semio_s_artifact_puzzle_3d_model::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Aggregator"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE3D_PLAY_MODE_EDIT)
            .io(puzzle3d_io())
            .window_kind_def(main::definition(&envelope, &Puzzle3dLabels::NATIVE_EN))
            .interaction(puzzle3d_interaction_definition())
            .window_kind_interactions(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE3D_INTERACTION_DOMAIN)])
            .default_layout(edit::layout())
            .panel_tab_def(artifact::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .panel_tab_def(settings_panel::definition())
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .keybinding("f", "focusSelection")
            // 🔧️ Document-mutating operations (emit the concrete kinds of the gesture).
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            // 🗨️ Shell-only effect (no document mutation): opens the "addObject" dialog. Declared HERE, in
            // front of every other create verb, because the shell fallback menu and the ribbon keep only
            // the first `CONTEXT_MENU_PRIMARY_BUDGET` leaves at top level and fold the rest into "More ›" —
            // the user-facing "Add Object…" row has to be the one that opens the declared dialog.
            .action_with(ActionDefinition::bounded_catalog("openAddObjectDialog", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}…"), |w| format!("{w} hinzufügen…")), ActionKind::Shell).category("create"))
            // 🌱️ The dialog's (and the catalogue row's / the viewport drop's) parametrized verb, never a
            // user-facing menu row of its own: its `objectKind` select IS the dialog, so offering it in the
            // palette/menu published a SECOND row carrying the identical "Add Object…" label that opened a
            // bare action pane instead (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 7).
            .action_with(ActionDefinition::bounded_catalog("addObjectKind", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")), ActionKind::Mutation).category("create").in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).category("selection"))
            .action_destructive("deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).category("create"))
            .action_with(ActionDefinition::bounded_catalog("exportSnapshot", LocalizedLabel::native("Export", "Exportieren"), ActionKind::Shell).category("file"))
            .action_with(ActionDefinition::bounded_catalog("importSnapshot", LocalizedLabel::native("Import", "Importieren"), ActionKind::Mutation).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("openImportSnapshot", LocalizedLabel::native("Import", "Importieren"), ActionKind::Shell).category("file"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("worldRelocate", puzzle3d_localized_phrase(|l| l.object, |w| format!("Relocate {w}"), |w| format!("{w} verlagern")))
            .action_with(ActionDefinition::bounded_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).category("hand"))
            .action_with(puzzle3d_flag_value_action("setSelectionHidden", "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle3d_flag_value_action("setSelectionLocked", "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .mutation("patchInspector", LocalizedLabel::native("Patch Inspector", "Inspektor aktualisieren"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"))
            .mutation("createAttraction", puzzle3d_localized_phrase(|l| l.attraction, |w| format!("Create {w}"), |w| format!("{w} erstellen")))
            // 🎯️ Entity-scoped and therefore NOT palette/shell-menu vocabulary: `delete_attraction` acts
            // on the `id` its own context-menu row carries, and the action declares no args, so a generic
            // surface can only ever dispatch it empty. Offering it anyway put "Delete Attraction" /
            // "Delete Target Volume" in the shell fallback menu of a plain OBJECT selection (measured
            // 2026-09-09 21:05) where they name nothing and do nothing. The attraction's own row in
            // `puzzle3d_context_menu_items` is unaffected — it supplies the id.
            .action_with(ActionDefinition::bounded_catalog("deleteAttraction", puzzle3d_localized_phrase(|l| l.attraction, |w| format!("Delete {w}"), |w| format!("{w} löschen")), ActionKind::Mutation).category("targets").in_palette(false))
            .action_destructive("deleteAttraction")
            .mutation("addTargetVolume", puzzle3d_localized_phrase(|l| l.target_volume, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")))
            .action_with(ActionDefinition::bounded_catalog("deleteTargetVolume", LocalizedLabel::native("Delete Target Volume", "Zielvolumen löschen"), ActionKind::Mutation).category("targets").in_palette(false))
            .action_destructive("deleteTargetVolume")
            .action_with(ActionDefinition::bounded_catalog("setTargetVolumeFlag", LocalizedLabel::native("Set Target Volume Flag", "Zielvolumenmarkierung festlegen"), ActionKind::Mutation).category("targets").in_palette(false))
            .mutation("addBrushObject", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add Brush {w}"), |w| format!("Pinsel-{w} hinzufügen")))
            .mutation("acceptSuggestion", LocalizedLabel::native("Accept Suggestion", "Vorschlag annehmen"))
            // 👁️ Ephemeral view state — selection, hover, camera scratch, utility-parameter runtime.
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"), ActionKind::View, "scan"))
            .action_with(ActionDefinition::new("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), ActionKind::View, "scan"))
            .view_action("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"))
            .action_with(ActionDefinition::bounded_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).category("selection"))
            .view_action("setVortexShow", puzzle3d_localized_phrase(|l| l.vortex_show, |w| format!("Set {w}"), |w| format!("{w} festlegen")))
            .view_action("setVortexDirection", puzzle3d_localized_phrase(|l| l.vortex_direction, |w| format!("Set {w}"), |w| format!("{w} festlegen")))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .view_action("setLodAutomatic", LocalizedLabel::native("Set Lod Automatic", "Detailstufe automatisch"))
            .view_action("setLodDepthVariable", LocalizedLabel::native("Set Lod Depth Variable", "Detailstufen-Tiefe festlegen"))
            .view_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster anzeigen"))
            .view_action("setLodManual", LocalizedLabel::native("Set Lod Manual", "Detailstufe manuell"))
            .action_with(ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3"))
            .view_action("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"))
            .view_action("setProximityRadius", LocalizedLabel::native("Set Proximity Radius", "Näheradius festlegen"))
            .view_action("setChunkSize", LocalizedLabel::native("Set Chunk Size", "Blockgröße festlegen"))
            .view_action("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"))
            .action_with(ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand"))
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand"))
            .view_action("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformieren-Griff festlegen"))
            .view_action("setVoxelDims", LocalizedLabel::native("Set Voxel Dims", "Voxel-Abmessungen festlegen"))
            .mutation("relocateTargetVolume", LocalizedLabel::native("Relocate Target Volume", "Zielvolumen verlagern"))
            .view_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .view_action("setBrushPlacementContactTolerance", LocalizedLabel::native("Set Brush Placement Contact Tolerance", "Pinsel-Kontakttoleranz festlegen"))
            .view_action("setObjectKindWeight", puzzle3d_localized_phrase(|l| l.object, |w| format!("Set {w} Kind Weight"), |w| format!("{w}-Art-Gewicht festlegen")))
            .view_action("setVortexKindWeight", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Set {w} Kind Weight"), |w| format!("{w}-Art-Gewicht festlegen")))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("cycleBrushCandidateBack", LocalizedLabel::native("Cycle Brush Candidate Back", "Pinselkandidat rückwärts wechseln"))
            .action_with(ActionDefinition::bounded_catalog("openVortexSuggestions", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Open {w} Suggestions"), |w| format!("{w}-Vorschläge öffnen")), ActionKind::View).category("tools"))
            .view_action("closeVortexSuggestions", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Close {w} Suggestions"), |w| format!("{w}-Vorschläge schließen")))
            .view_action("hoverSuggestion", LocalizedLabel::native("Hover Suggestion", "Vorschlag überfahren"))
            .action_audience("hoverSuggestion", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("targetBrushSuggestions", LocalizedLabel::native("Target Brush Suggestions", "Pinselvorschläge ausrichten"))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            // 📝️ Staged argument forms for the panel-visible create/query actions (P1).
            .action_args("addObjectKind", vec![puzzle3d_object_kind_arg()])
            // 🧰️ Flat per-window set of utilities; no utility is active until the host presses one — the
            // transform gumball exposes translate and rotate together via Move/Rotate flags.
            .utility(semio_s_artifact_puzzle_3d_editor_transform::definition())
            .utility(utilities::brush::definition(puzzle3d_localized(|l| l.brush)))
            .utility(utilities::volume_brush::definition(puzzle3d_localized(|l| l.volume_brush)))
            .utility(utilities::world_relocate::definition())
            .window_kind_utilities(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, vec![
                semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID.into(),
                utilities::brush::UTILITY_ID.into(),
                utilities::volume_brush::UTILITY_ID.into(),
                utilities::world_relocate::UTILITY_ID.into(),
            ])
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility — it keeps
            // its viewport interaction via `Puzzle3dConfig::active_tool_id`.
            .tool(fill_tool::definition(puzzle3d_localized(|l| l.fill)))
            .mode_tools(edit::PUZZLE3D_PLAY_MODE_EDIT, vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID))])
            // 🎓️ Reference introduction: a short first-run walkthrough of the viewport, the catalogue
            // panel, adding an object, and the Transform utility.
            .introduction(IntroductionDefinition {
                title: puzzle3d_localized_phrase(|l| l.window_main, |w| format!("Welcome to {w}"), |w| format!("Willkommen bei {w}")),
                steps: vec![
                    IntroductionStepDefinition::new(
                        "welcome",
                        puzzle3d_localized_phrase(|l| l.window_main, |w| format!("Welcome to {w}"), |w| format!("Willkommen bei {w}")),
                        LocalizedLabel::native(
                            "A quick tour of the viewport, utilities, and panels before you start composing.",
                            "Eine kurze Tour durch Ansicht, Hilfsmittel und Paneele, bevor Sie mit dem Zusammenfügen beginnen.",
                        ),
                    ),
                    IntroductionStepDefinition::new(
                        "viewport",
                        LocalizedLabel::native("The Viewport", "Die 3D-Ansicht"),
                        LocalizedLabel::native(
                            "This is your 3D scene — orbit, pan, and zoom to look around.",
                            "Das ist Ihre 3D-Szene — orbitieren, verschieben und zoomen Sie, um sich umzusehen.",
                        ),
                    )
                        .introduce(window_element_id(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID))
                        .interact(vec![
                            ::semio_framework_async::poll::resolve_ready(IntroductionInteraction::zoom(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, "Zoom")),
                            ::semio_framework_async::poll::resolve_ready(IntroductionInteraction::pan(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, "Pan")),
                            ::semio_framework_async::poll::resolve_ready(IntroductionInteraction::orbit(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, "Orbit")),
                        ]),
                    IntroductionStepDefinition::new(
                        "catalogue",
                        LocalizedLabel::native("The Catalogue", "Der Katalog"),
                        puzzle3d_localized_phrase(|l| l.objects, |w| format!("Browse the {w} available to place from here."), |w| format!("Durchstöbern Sie hier die verfügbaren {w}.")),
                    )
                        .introduce(panel_tab_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID))
                        .placement(IntroductionPlacement::Right),
                    IntroductionStepDefinition::new(
                        "add-object",
                        puzzle3d_localized_phrase(|l| l.object, |w| format!("Add a {w}"), |w| format!("{w} hinzufügen")),
                        puzzle3d_localized_phrase(
                            |l| l.object,
                            |w| format!("Drag the first {w} from the catalogue into the viewport."),
                            |_w| "Ziehen Sie den ersten Eintrag per Drag-and-Drop aus dem Katalog in die 3D-Ansicht.".to_string(),
                        ),
                    )
                        .introduce(panel_tab_first_draggable_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID))
                        .show(vec![panel_tab_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID), window_element_id(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)])
                        .placement(IntroductionPlacement::Right)
                        .interact(vec![::semio_framework_async::poll::resolve_ready(IntroductionInteraction::action("addObjectKind", "Add an object"))]),
                    IntroductionStepDefinition::new(
                        "transform-utility",
                        puzzle3d_localized_phrase(|l| l.objects, |w| format!("Transform {w}"), |w| format!("{w} transformieren")),
                        puzzle3d_localized_phrase(
                            |l| l.objects,
                            |w| format!("Activate the Transform utility to move and rotate {w} in the scene."),
                            |w| format!("Aktivieren Sie das Transformieren-Hilfsmittel, um {w} zu verschieben und zu drehen."),
                        ),
                    )
                        .introduce(semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID)
                        .show(vec![window_element_id(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)])
                        .interact(vec![::semio_framework_async::poll::resolve_ready(IntroductionInteraction::utility(semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID, "Activate Transform"))]),
                ],
            })
            // 🗨️ Reference dialog opened by `openAddObjectDialog`, driving the existing `addObjectKind`
            // operation's `objectKind` select arg.
            .dialog(
                DialogDefinition::new(
                    "addObject",
                    puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")),
                    ActionRef::new("addObjectKind"),
                )
                    .body(puzzle3d_localized_phrase(
                        |l| l.object,
                        |w| format!("Choose the kind of {w} to add to the scene."),
                        |_w| "Wählen Sie die Art zum Hinzufügen.".to_string(),
                    ))
                    .args(vec![puzzle3d_object_kind_arg().required()])
                    .submit_label(LocalizedLabel::native("Add", "Hinzufügen")),
            )
            .action_interactive_job("acceptSuggestion", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addBrushObject", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addObjectKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("closeVortexSuggestions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("createAttraction", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteAttraction", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_destructive("exportSnapshot")
            .action_interactive_job("importSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddObjectDialog", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openVortexSuggestions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("patchInspector", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("registerBrushMesh", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKindSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementContactTolerance", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setChunkSize", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSpacing", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodAutomatic", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodDepthVariable", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodManual", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setObjectKindWeight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionHidden", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionLocked", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexDirection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexKindWeight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexShow", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVoxelDims", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("worldRelocate", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 3D puzzle with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 3D-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected object from the 3D puzzle together with its connections.", "Löscht alle ausgewählten Objekte samt ihrer Verbindungen aus dem 3D-Puzzle."))
            .action_describe("duplicateSelection", LocalizedLabel::native("Adds a copy of every selected object next to the original; nothing happens without a selection.", "Fügt neben jedem ausgewählten Objekt eine Kopie hinzu; ohne Auswahl geschieht nichts."))
            .action_describe("focusSelection", LocalizedLabel::native("Frames the camera on the selected objects; only the view changes.", "Richtet die Kamera auf die ausgewählten Objekte aus; nur die Ansicht ändert sich."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected objects by dx and dy.", "Verschiebt die ausgewählten Objekte um dx und dy."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected objects by the given angle.", "Dreht die ausgewählten Objekte um den angegebenen Winkel."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected objects by the given factor.", "Skaliert die ausgewählten Objekte um den angegebenen Faktor."))
            .action_describe("exportSnapshot", LocalizedLabel::native("Writes the whole 3D puzzle as JSON to a downloaded file named after the active example on the user's machine.", "Schreibt das gesamte 3D-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."))
            .action_describe("openImportSnapshot", LocalizedLabel::native("Opens the host's file picker for a 3D puzzle JSON file; the chosen file then replaces the whole puzzle.", "Öffnet die Dateiauswahl des Hosts für eine 3D-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."))
            .action_describe("importSnapshot", LocalizedLabel::native("Replaces the whole 3D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 3D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen."))
            .action_describe("setSelectionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given or selected objects.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten Objekte."))
            .action_describe("setSelectionHidden", LocalizedLabel::native("Sets the given or selected objects hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Objekte oder zeigt sie, genau auf den übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setSelectionLocked", LocalizedLabel::native("Sets the given or selected objects locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Objekte oder entsperrt sie, genau auf den übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("acceptSuggestion", LocalizedLabel::native("Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."))
            .action_describe("selectSameKindSelection", LocalizedLabel::native("Extends the selection to every piece of the same kind as the selected one.", "Erweitert die Auswahl auf alle Teile derselben Art wie das ausgewählte."))
            .action_describe("toggleSun", LocalizedLabel::native("Switches the 3D view's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the 3D view's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Ansicht scheint; nur die Ansicht ändert sich."))
            .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the 3D view's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Ansicht über dem Horizont steht; nur die Ansicht ändert sich."))
            .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the 3D view's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Ansicht fest; nur die Ansicht ändert sich."))
            .action_describe("setLodAutomatic", LocalizedLabel::native("Turns automatic level of detail in the 3D view on or off; only the view changes.", "Schaltet die automatische Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setLodDepthVariable", LocalizedLabel::native("Turns depth-dependent level of detail in the 3D view on or off; only the view changes.", "Schaltet die tiefenabhängige Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setLodManual", LocalizedLabel::native("Sets a fixed level of detail for the 3D view; only the view changes.", "Legt eine feste Detailstufe für die 3D-Ansicht fest; nur die Ansicht ändert sich."))
            .action_describe("setGridVisible", LocalizedLabel::native("Shows or hides the placement grid in the 3D view; only the view changes.", "Blendet das Platzierungsraster in der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setGridSnapEnabled", LocalizedLabel::native("Turns snapping to the placement grid on or off for moves in the 3D view.", "Schaltet das Fangen am Platzierungsraster für Verschiebungen in der 3D-Ansicht ein oder aus."))
            .action_describe("setGridSpacing", LocalizedLabel::native("Sets the spacing of the placement grid.", "Legt den Abstand des Platzierungsrasters fest."))
            .action_describe("setProximityRadius", LocalizedLabel::native("Sets how close two pieces must come for a move or Connect Nearby to join their connection points.", "Legt fest, wie nahe sich zwei Teile kommen müssen, damit ein Verschieben oder In der Nähe verbinden ihre Anschlusspunkte verbindet."))
            .action_describe("setChunkSize", LocalizedLabel::native("Sets the size of the spatial chunks the 3D view loads and draws pieces in; only the view changes.", "Legt die Größe der räumlichen Blöcke fest, in denen die 3D-Ansicht Teile lädt und zeichnet; nur die Ansicht ändert sich."))
            .action_describe("setSelectableKind", LocalizedLabel::native("Sets whether pieces, connection points or attractions can be picked in the 3D view.", "Legt fest, ob Teile, Anschlusspunkte oder Anziehungen in der 3D-Ansicht gewählt werden können."))
            .action_describe("setTransformGumballFlag", LocalizedLabel::native("Switches one option of the transform gumball (such as translate, rotate or scale handles) on or off.", "Schaltet eine Option des Transformationsgriffs (etwa Verschiebe-, Dreh- oder Skaliergriffe) ein oder aus."))
            .action_describe("setVoxelDims", LocalizedLabel::native("Sets one dimension (width, depth or height) of the voxel volume the fill tool packs pieces into.", "Legt eine Abmessung (Breite, Tiefe oder Höhe) des Voxelvolumens fest, in das das Füllwerkzeug Teile packt."))
            .action_describe("relocateTargetVolume", LocalizedLabel::native("Moves one target volume to a new position.", "Verschiebt ein Zielvolumen an eine neue Position."))
            .action_describe("setFillCount", LocalizedLabel::native("Sets how many pieces the fill tool places into the target volume.", "Legt fest, wie viele Teile das Füllwerkzeug in das Zielvolumen setzt."))
            .action_describe("setBrushPlacementContactTolerance", LocalizedLabel::native("Sets how much overlap (0 to 1) the placement brush tolerates between a suggested piece and existing ones.", "Legt fest, wie viel Überlappung (0 bis 1) der Platzierungspinsel zwischen einem vorgeschlagenen und vorhandenen Teilen zulässt."))
            .action_describe("cycleBrushCandidate", LocalizedLabel::native("Switches the placement brush to the next suggested piece.", "Schaltet den Platzierungspinsel zum nächsten vorgeschlagenen Teil."))
            .action_describe("cycleBrushCandidateBack", LocalizedLabel::native("Switches the placement brush to the previous suggested piece.", "Schaltet den Platzierungspinsel zum vorherigen vorgeschlagenen Teil."))
            .action_describe("openVortexSuggestions", LocalizedLabel::native("Opens the list of pieces that fit at one connection point.", "Öffnet die Liste der Teile, die an einen Anschlusspunkt passen."))
            .action_describe("closeVortexSuggestions", LocalizedLabel::native("Closes the list of suggested pieces.", "Schließt die Liste der vorgeschlagenen Teile."))
            .action_describe("addTargetVolume", LocalizedLabel::native("Adds a target volume, a box region the fill tool packs pieces into.", "Fügt ein Zielvolumen hinzu, einen Quaderbereich, in den das Füllwerkzeug Teile packt."))
            .action_describe("deleteTargetVolume", LocalizedLabel::native("Deletes one target volume by id.", "Löscht ein Zielvolumen anhand seiner Id."))
            .action_describe("setTargetVolumeFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given target volumes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielvolumen."))
            .action_describe("openAddObjectDialog", LocalizedLabel::native("Opens the Add Object dialog to pick an object kind to place.", "Öffnet den Dialog Objekt hinzufügen, um eine zu setzende Objektart zu wählen."))
            .action_describe("addObjectKind", LocalizedLabel::native("Adds an object of the given kind (a 3D block kind) to the puzzle.", "Fügt dem Puzzle ein Objekt der angegebenen Art (einer 3D-Blockart) hinzu."))
            .action_describe("worldRelocate", LocalizedLabel::native("Moves one object to a new position and connects it to nearby compatible connection points.", "Verschiebt ein Objekt an eine neue Position und verbindet es mit verträglichen Anschlusspunkten in der Nähe."))
            .action_describe("patchInspector", LocalizedLabel::native("Sets one inspector field on the given or selected objects or connection points.", "Setzt ein Inspektorfeld auf den angegebenen oder ausgewählten Objekten oder Anschlusspunkten."))
            .action_describe("createAttraction", LocalizedLabel::native("Creates an attraction that pulls two compatible connection points (vortices) of different objects together.", "Erstellt eine Anziehung, die zwei verträgliche Anschlusspunkte (Wirbel) verschiedener Objekte zusammenzieht."))
            .action_describe("deleteAttraction", LocalizedLabel::native("Deletes one attraction by id, releasing its two connection points.", "Löscht eine Anziehung anhand ihrer Id und gibt ihre zwei Anschlusspunkte frei."))
            .action_describe("addBrushObject", LocalizedLabel::native("Places an object with the placement brush at the given contact point, as the brush suggests.", "Setzt mit dem Platzierungspinsel ein Objekt am angegebenen Kontaktpunkt, wie der Pinsel es vorschlägt."))
            .action_describe("setVortexShow", LocalizedLabel::native("Sets whether connection points are shown always or only on selected objects; only the view changes.", "Legt fest, ob Anschlusspunkte immer oder nur an ausgewählten Objekten gezeigt werden; nur die Ansicht ändert sich."))
            .action_describe("setVortexDirection", LocalizedLabel::native("Sets how connection point directions are drawn; only the view changes.", "Legt fest, wie die Richtungen der Anschlusspunkte gezeichnet werden; nur die Ansicht ändert sich."))
            .action_describe("setObjectKindWeight", LocalizedLabel::native("Sets how often the fill tool and brush pick one object kind relative to the others.", "Legt fest, wie oft Füllwerkzeug und Pinsel eine Objektart im Verhältnis zu den anderen wählen."))
            .action_describe("setVortexKindWeight", LocalizedLabel::native("Sets how strongly one connection point kind is preferred when suggesting placements.", "Legt fest, wie stark eine Anschlusspunktart bei Platzierungsvorschlägen bevorzugt wird."))
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setProjection", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setProjectionParam", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("engagementRepeatLast", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementControlSelect", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("targetBrushSuggestions", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("registerBrushMesh", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("setTransformGumballFlag", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("importSnapshot")
            .build_definition()
}

//#endregion 🔖️Manifest


impl Puzzle3dPlayApp {
    /// 🖱️ The ONE context-menu implementation — `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle3dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        let Some(labels) = puzzle3d_labels(view_state) else { return Vec::new() };
        let wid = view_state.window_id.as_deref().unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
        let active_utility = puzzle3d_scene_active_utility(&config, Some(view_state), Some(wid));
        let envelope = scene_from_snapshot(doc.snapshot.typed(),config,&active_utility);
        let mut selection = Puzzle3dContextSelection::from_surface(request.surface.as_ref());
        selection.fill_from_interaction(interaction);
        puzzle3d_context_menu_items(&envelope, &selection, labels, view_state, registry)
    }

    /// 🖼️ The ONE render implementation — both `ArtifactEditor::render` and
    /// `render_with_request_context` funnel here, differing only in whether `interaction` carries a
    /// live `vortex`-domain read or the empty default.
    pub fn render_body(
        owner: Option<&ArtifactInstanceOperationOwnerHandle>,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &window_ownership::Puzzle3dWindowTransient,
        interaction: &Puzzle3dInteractionSnapshot,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), transient, Some(view_state));
        let node = with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |app| -> semio_framework_plugin::UiAssemblyResult<_> {
            let (base_body_key, window_id_from_key) = body_key.split_once(':').map_or((body_key, None), |(b, w)| (b, Some(w)));
            let config = &runtime;
            let wid = puzzle3d_addressed_window_id(Some(view_state), window_id_from_key, None, &config.window_ids);
            let envelope = main::scene_for(doc.snapshot, config, Some(view_state), wid);
            {
                let mut precompute = app.precompute.borrow_mut();
                sync_precompute_session(&mut precompute, &envelope);
            }
            let precompute = app.precompute.borrow();
            // 🪣️ What the viewport shows is exactly the document the framework hands in: while a fill run
            // holds provisional placements, `doc.snapshot` is the tool run overlay (committed ⊕ provisional).
            // The precompute session and every body render off this ONE decoded scene.
            let labels = puzzle3d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle3d has no authored label set for the host's locale/terminology axes"))?;
            match base_body_key {
                main::BODY_KEY => {
                    let provisional = doc.tool_run().map(|run| std::sync::Arc::clone(&run.provisional_entities)).unwrap_or_default();
                    let (instances_json, meshes_json, instances_delta_json) = app.geometry_jsons(&envelope.scene_snapshot, &provisional, &interaction.referenced);
                    let menu_target = envelope.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.as_str());
                    let suggestions = menu_target.and_then(|target| owner?.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(owner.brush_suggestions.found(target).cloned())).ok().flatten());
                    main::render(&envelope, &precompute, instances_json, meshes_json, instances_delta_json, interaction, suggestions.as_ref())
                }
                // 🪟️ One `TreeWindows` read per body: the host's open/scroll state for exactly the body
                // being rendered, so every container stamps its full `total` and materialises only the
                // rows on screen (📓️design-virtualised-tree.md §5, §8).
                artifact::BODY_KEY => artifact::render(&envelope.scene_snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact::BODY_KEY)),
                catalogue::BODY_KEY => catalogue::render(&envelope, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, catalogue::BODY_KEY)),
                inspection::BODY_KEY => inspection::render(&envelope, interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection::BODY_KEY)),
                settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, wid),
                _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d unknown-body label admission failed")),
            }
        })?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    /// 🪟️ The ONE per-window-instance chrome implementation `ArtifactEditor::window_measures` funnels into.
    fn window_measures_body(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let config = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &config, |app| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = main::scene_for(doc.snapshot, &config, Some(view_state), window_id);
            HashMap::from([(window_id.to_string(), main::window_measures(&envelope, labels))])
        })
    }
}

use semio_s_artifact_puzzle_3d_commands_document::delete_selection;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;

/// ✂️ Copies then deletes the selected objects as one mutation list.
fn puzzle3d_cut_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, interaction: &InteractionView<'_>) -> Vec<Puzzle3dMutation> {
    puzzle3d_cut_operations_from(doc.snapshot.typed(), &Puzzle3dInteractionSnapshot::from_interaction(interaction))
}

pub fn puzzle3d_cut_operations_from(before: &Puzzle3dSnapshot, marks: &Puzzle3dInteractionSnapshot) -> Vec<Puzzle3dMutation> {
    let host = puzzle3d_scene_snapshot_from_document(before);
    let ids: Vec<String> = puzzle3d_selected_objects_from(marks, &host).into_iter().map(|object| object.id).collect();
    delete_selection::delete_selection_mutations(before, &ids, &HashSet::new(), &[], &[], &[])
}

//#region 🌳️Modules
#[path = "🌉️wasm/🦀️.rs"]
pub mod wasm;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo_session {
        #[path = "📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod component;
        pub use component::*;
    }
}
//#endregion 🌳️Modules
