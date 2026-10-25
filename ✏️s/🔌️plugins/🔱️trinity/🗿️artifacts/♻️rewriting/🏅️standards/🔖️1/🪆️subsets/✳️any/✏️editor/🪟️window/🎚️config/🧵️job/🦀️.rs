//! 🧵️ Retained publication of bounded Rewriting window configuration commands.

use crate::editor::rewriting::{TrinityRewritingCommand, TrinityRewritingPlayApp};
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
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
        TrinityRewritingCommand::SetViewport { surface_id, viewport } => crate::editor::rewriting::commands::set_viewport::set_viewport(surface_id, viewport, view),
        TrinityRewritingCommand::SetLodMode { value } => crate::editor::rewriting::commands::set_lod_mode::set_lod_mode(value, view),
        _ => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "the rewriting window reducer received a command outside its tool roster")),
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
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: RAW_BYTES, checkpoint_resume: false);

}

impl ArtifactOwnedToolJobFactory for RewritingWindowConfigJobFactory {
    type Owner = EditorApp<TrinityRewritingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::standards::v1::subsets::any::schema::model::REWRITE_RULE_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] =
        &[ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] }, ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::WindowConfig] }];
}

pub fn build_job(request: &mut Option<ArtifactOwnedToolJobRequest<Owner>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    if !TOOL_IDS.contains(&original.tool_id.as_str()) {
        return Ok(false);
    }
    if TrinityRewritingPlayApp::command_id(&original.command) != original.tool_id {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "Rewriting command does not match its exact registered tool").into());
    }
    if extent(&original.command, &original.snapshot, &original.interaction_state) != Some(1) {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "the rewriting command exceeds the capacity of one bounded edit").into());
    }
    semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, TrinityRewritingPlayApp::command_id, RAW_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(TrinityRewritingPlayApp::command_id(&request.command), reduce, extent))
}
