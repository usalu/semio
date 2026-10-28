//! 🎛️ The retained command job: payload schema, extent, reduce and the bounded command work.

use super::*;
#[path = "../🧮️native-extent/🦀️.rs"]
mod native_extent;
use native_extent::native_tree_extent;

//#region 🧵️RetainedCommands
const ENERGY_MODEL_RETAINED_PAYLOAD_SCHEMA: &str = "energy.model.tool-command.v1";
pub(super) const ENERGY_MODEL_RETAINED_RAW_BYTES: usize = 262_144;
/// 🧮️ One retained work item covers one whole-model edit; the extent guard below refuses a document
/// whose addressable entity count would exceed the bounded first step.
pub(super) const ENERGY_MODEL_RETAINED_WORK_ITEMS: usize = 65_536;

pub(super) fn energy_model_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(ENERGY_MODEL_RETAINED_RAW_BYTES, ENERGY_MODEL_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 🧮️ Refuses a first step whose document is larger than the declared retained extent, so an
/// oversized model is rejected at admission rather than truncated mid-edit.
pub(super) fn energy_model_extent(command: &EnergyModelEditorCommand, snapshot: &EnergyModelSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&command.action_id()) {
        return None;
    }
    let mut remaining = ENERGY_MODEL_RETAINED_WORK_ITEMS;
    if let EnergyModelEditorCommand::ApplyNativeMutation { mutation, .. } = command { return mutation.as_ref().map(|_| 1); }
    native_tree_extent(&snapshot.model, &mut remaining, 0)?;
    Some(1)
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
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &energy_publication_contracts();

}
//#endregion 🧵️RetainedCommands

/// 📬️ Authored and schema-derived native tools retain exact, compile-time publication authority.
const fn energy_publication_contracts() -> [ArtifactToolPublicationContract; ENERGY_MODEL_RETAINED_TOOL_IDS.len()] {
    let mut rows = [ArtifactToolPublicationContract { tool_id: "", lanes: &[ArtifactToolPublicationLane::Artifact] }; ENERGY_MODEL_RETAINED_TOOL_IDS.len()];
    let mut index = 0;
    while index < ENERGY_MODEL_RETAINED_TOOL_IDS.len() {
        let tool = ENERGY_MODEL_RETAINED_TOOL_IDS[index];
        rows[index].tool_id = tool;
        if tool_id_matches(tool, SET_ACTIVE_EXAMPLE_ACTION_ID) { rows[index].lanes = &[ArtifactToolPublicationLane::HostOnly]; }
        if tool_id_matches(tool, simulation::SET_SETTINGS_ACTION_ID) || tool_id_matches(tool, simulation::SET_RESULT_FIELD_ACTION_ID) { rows[index].lanes = &[ArtifactToolPublicationLane::Config]; }
        if tool_id_matches(tool, model_window::SET_CAMERA_ACTION_ID) { rows[index].lanes = &[ArtifactToolPublicationLane::WindowConfig]; }
        index += 1;
    }
    rows
}

const fn tool_id_matches(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() { return false; }
    let mut index = 0;
    while index < left.len() { if left[index] != right[index] { return false; } index += 1; }
    true
}

#[path="🧬️original/🦀️.rs"]mod original;
pub(super)use original::EnergyModelCommandWork;
