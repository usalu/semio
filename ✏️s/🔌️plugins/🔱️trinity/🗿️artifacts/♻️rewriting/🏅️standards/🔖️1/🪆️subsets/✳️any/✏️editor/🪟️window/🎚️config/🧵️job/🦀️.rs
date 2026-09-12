//! 🧵️ Retained publication of bounded Rewriting window configuration commands.

use crate::editor::rewriting::{TrinityRewritingCommand, TrinityRewritingPlayApp};
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, ArtifactOwnedToolJobRequest};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, Emit, Fault, HistoryView, InteractiveJobClassification, NoConfig, NoConfigMutation, NoDraftMutation,
    ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError,
};

type Owner = EditorApp<TrinityRewritingPlayApp>;
pub const TOOL_IDS: &[&str] = &["nodeGraphViewport", "setLodMode"];
const PAYLOAD_SCHEMA: &str = "trinity.rewriting.window-command.v1";
const RAW_BYTES: usize = 4096;

pub fn contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(RAW_BYTES, 64, 1, 8192, 7500)
}

fn extent(command: &TrinityRewritingCommand, _snapshot: &RewritingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        TrinityRewritingCommand::SetViewport { viewport, .. } if viewport.validate().is_ok() => Some(1),
        TrinityRewritingCommand::SetLodMode { value } if value.chars().take(65).count() <= 64 => Some(1),
        _ => None,
    }
}

fn reduce(
    command: &TrinityRewritingCommand,
    _snapshot: &RewritingSnapshot,
    _config: &NoConfig,
    _history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&ArtifactOwnedToolJobContext<Owner>>,
    _operation: &AppOperationContext,
) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let view = context.and_then(|context| context.view_state.as_ref());
    match command {
        TrinityRewritingCommand::SetViewport { surface_id, viewport } => crate::editor::rewriting::commands::set_viewport(surface_id, viewport, view),
        TrinityRewritingCommand::SetLodMode { value } => crate::editor::rewriting::commands::set_lod_mode(value, view),
        _ => Err(Fault::from("rewriting-window-command-route-mismatch")),
    }
}

pub struct RewritingWindowConfigJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl RewritingWindowConfigJobFactory {
    pub fn new(controller: &str) -> Self {
        Self { keys: TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl ToolJobFactory for RewritingWindowConfigJobFactory {
    type Payload = ArtifactRetainedCommandPayload<Owner>;
    type Job = ArtifactRetainedCommandJob<Owner>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Rewriting window command rejects oversized wire or a checkpoint"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for RewritingWindowConfigJobFactory {
    type Owner = EditorApp<TrinityRewritingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::REWRITE_RULE_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] =
        &[ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] }, ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::WindowConfig] }];
}

pub fn build_job(request: ArtifactOwnedToolJobRequest<Owner>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
    if !TOOL_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    if TrinityRewritingPlayApp::command_id(&request.command) != request.tool_id || extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
        return Err(Fault::from("rewriting-window-command-mismatch-or-capacity"));
    }
    let work: Box<dyn ArtifactCommandWork<Owner>> = Box::new(BoundedArtifactCommandWork::new(TrinityRewritingPlayApp::command_id(&request.command), reduce, extent));
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
    };
    let payload = ArtifactRetainedCommandPayload::try_new(
        ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation,
            completion: request.completion,
        },
        TrinityRewritingPlayApp::command_id,
        RAW_BYTES,
        1,
        work,
    )?;
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
