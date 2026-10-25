//! 🎛️ The retained command job: payload schema, extent, reduce and the bounded command work.

use super::*;

//#region 🧵️RetainedCommands
const ENERGY_MODEL_RETAINED_PAYLOAD_SCHEMA: &str = "energy.model.tool-command.v1";
pub(super) const ENERGY_MODEL_RETAINED_RAW_BYTES: usize = 65_536;
/// 🧮️ One retained work item covers one whole-model edit; the extent guard below refuses a document
/// whose addressable entity count would exceed the bounded first step.
pub(super) const ENERGY_MODEL_RETAINED_WORK_ITEMS: usize = 65_536;

fn energy_model_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(ENERGY_MODEL_RETAINED_RAW_BYTES, ENERGY_MODEL_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 🧮️ Refuses a first step whose document is larger than the declared retained extent, so an
/// oversized model is rejected at admission rather than truncated mid-edit.
pub(super) fn energy_model_extent(command: &EnergyModelEditorCommand, snapshot: &EnergyModelSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&command.action_id()) {
        return None;
    }
    let model = &snapshot.model;
    let items = model.zones.len().saturating_add(model.spaces.len()).saturating_add(model.surfaces.len()).saturating_add(model.fenestrations.len()).saturating_add(model.materials.len()).saturating_add(model.constructions.len());
    (items <= ENERGY_MODEL_RETAINED_WORK_ITEMS).then_some(1)
}

#[allow(clippy::needless_pass_by_value)]
pub(super) fn energy_model_reduce(
    command: &EnergyModelEditorCommand,
    snapshot: &EnergyModelSnapshot,
    _config: &EnergyModelConfig,
    history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EnergyModelEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<EnergyModelMutation, EnergyModelConfigMutation, NoDraftMutation>, Fault> {
    if let Some(camera) = camera_emit(command, _context.and_then(|context| context.view_state.as_ref())) {
        return camera;
    }
    reduce(command, &ArtifactView::with_operation(snapshot, history, operation.clone()))
}

pub(super) struct EnergyModelCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl EnergyModelCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: ENERGY_MODEL_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for EnergyModelCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<EnergyModelEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<EnergyModelEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        ENERGY_MODEL_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        energy_model_contract()
    }

    semio_framework_plugin_retained_command::retained_command_factory_admission!(maximum_raw_bytes: ENERGY_MODEL_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for EnergyModelCommandJobFactory {
    type Owner = EditorApp<EnergyModelEditor>;
    const TOOL_IDS: &'static [&'static str] = ENERGY_MODEL_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: SET_NODE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_CELL_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: CREATE_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: RENAME_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: DELETE_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: CREATE_SURFACE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: DELETE_SURFACE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_THERMOSTAT_SETPOINTS_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_SITE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_RUN_PERIOD_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: simulation::SET_SETTINGS_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: simulation::SET_RESULT_FIELD_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: model_window::SET_CAMERA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: SET_SURFACE_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_FENESTRATION_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_ZONE_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_GAS_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_CONSTRUCTION_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
}
//#endregion 🧵️RetainedCommands
