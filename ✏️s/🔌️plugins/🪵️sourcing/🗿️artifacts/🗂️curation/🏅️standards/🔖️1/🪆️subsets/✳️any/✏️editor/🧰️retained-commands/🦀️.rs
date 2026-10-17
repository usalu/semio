//! 🧰️ Retained commands: the tool id catalog, the bounded contract, the retained reducer, the bounded command job factory and the finish emit.

use super::*;

//#region 🧵️RetainedCommands
/// 🧵️ Every UI-reachable command of this app, all on the retained bounded first-step lane. A command
/// left off this list is unreachable at runtime, not merely untested: `validate_ui_dispatch_classification`
/// rejects any dispatch whose registry classification is not `Migrated`, and `qualified_tool_proof`
/// refuses a typed command that owns no tool proof — so the eight ids that used to sit in a
/// `BATCH_ONLY_PENDING_REWRITE` list (the whole curation vocabulary plus the module filter and both
/// whole-document replacements) could not be invoked from the browser at all.
pub const SOURCING_CURATION_BOUNDED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "loadDocumentJson",
    "stockFromCatalogue",
    "curationAdd",
    "curationSetCount",
    "curationRemove",
    "dropOnPool",
    "dropOnCurated",
    "setFilterQuery",
    "setFilterModule",
    "setFilterTypology",
    "setFilterMinAvailability",
    "sortTable",
    "setContributions",
    "setGridInstanceDisplay",
];
const SOURCING_CURATION_RETAINED_SCHEMA: &str = "sourcing.curation/v1.tool-command.v1";
pub const SOURCING_CURATION_RETAINED_RAW_BYTES: usize = 8_192;

/// 📏️ The raw-wire ceiling one retained tool id is admitted against. `setContributions` is the ONE
/// retained command whose wire is a host pack, not a gesture — it carries the whole capability
/// closure the receiver's registry `consumes` row names — so it is priced on
/// [`semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES`], never on the gesture envelope.
pub fn sourcing_curation_retained_raw_bytes(tool_id: &str) -> usize {
    if tool_id == "setContributions" {
        semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES
    } else {
        SOURCING_CURATION_RETAINED_RAW_BYTES
    }
}
pub const SOURCING_CURATION_RETAINED_WORK_ITEMS: usize = 1;

/// 🧾️ The ONE execution contract every retained sourcing tool is admitted under. Its wire ceiling is
/// the contributions pack, because one of those tools carries a host pack rather than a gesture and a
/// factory declares one contract for every tool it serves, which the proof catalogue must join
/// exactly. Widening the ADMISSION does not widen what a gesture may send: the payload reserves, and
/// the wire factory refuses past, `sourcing_curation_retained_raw_bytes(tool_id)`.
pub fn sourcing_curation_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, 64, 1, 16_384, 7_500)
}

pub fn sourcing_curation_bounded_extent(command: &SourcingCurationCommand, _snapshot: &CurationSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SOURCING_CURATION_BOUNDED_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub fn sourcing_curation_retained_reduce(
    command: &SourcingCurationCommand,
    snapshot: &CurationSnapshot,
    config: &SourcingCurationConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SourcingCurationApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, NoDraftMutation>, Fault> {
    if !SOURCING_CURATION_BOUNDED_TOOL_IDS.contains(&command.command_id()) { return Err(Fault::from("sourcing-curation-retained-route-mismatch")); }
    let window = _context.and_then(|context| context.window_config.as_ref());
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window };
    let emit = command.dispatch(&doc, &cfg)?;
    let view = _context.and_then(|context| context.view_state.as_ref());
    sourcing_curation_finish_emit(command, emit, window, view)
}

pub struct SourcingCurationBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl SourcingCurationBoundedCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: SOURCING_CURATION_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SourcingCurationBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<SourcingCurationApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<SourcingCurationApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SOURCING_CURATION_RETAINED_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        sourcing_curation_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > payload.maximum_raw_bytes || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Sourcing bounded command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for SourcingCurationBoundedCommandJobFactory {
    type Owner = EditorApp<SourcingCurationApp>;
    const TOOL_IDS: &'static [&'static str] = SOURCING_CURATION_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATION_SCHEMA;
    /// 🛤️ The lane each tool publishes on. `HostOnly` is a whole-document replacement carried as an
    /// `Effect::LoadDocument` rather than a store edit (`loadDocumentJson`/`stockFromCatalogue` share
    /// `setActiveExample`'s `reset_document_effect` path); `Artifact` is the curated-selection
    /// vocabulary, admitted by `SourcingCurationArtifactPreparationFactory`; `Config` is view state.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "loadDocumentJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "stockFromCatalogue", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "curationAdd", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "curationSetCount", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "curationRemove", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "dropOnPool", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "dropOnCurated", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setFilterQuery", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFilterModule", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFilterTypology", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFilterMinAvailability", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "sortTable", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setGridInstanceDisplay", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ];
}

fn sourcing_grid_window_mutation(command: &SourcingCurationCommand, _config: &grid_config::GridWindowConfig) -> Option<GridWindowConfigMutation> {
    match command {
        SourcingCurationCommand::SetGridInstanceDisplay(payload) if !payload.value.is_empty() => {
            Some(GridWindowConfigMutation::SetInstanceDisplay(grid_config::GridSetInstanceDisplay { instance_display: payload.value.clone() }))
        }
        _ => None,
    }
}

pub fn sourcing_curation_finish_emit(
    command: &SourcingCurationCommand,
    mut emit: Emit<SourcingMutation, SourcingCurationConfigMutation, NoDraftMutation>,
    window: Option<&semio_framework_plugin::WindowConfigSnapshot>,
    view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, NoDraftMutation>, Fault> {
    let grid_cfg = grid_config::from_snapshot(window);
    if let (Some(view), Some(mutation)) = (view_state, sourcing_grid_window_mutation(command, &grid_cfg)) {
        emit.window_config_mutations.push(grid_config::addressed(view, mutation)?);
    }
    Ok(emit)
}

//#endregion 🧵️RetainedCommands
