//! 🧵️ Rewriting retained document commands: the document tool ids and limits, the bounded document job factory and the document tool job builder.

use super::*;

//#region 🧵️RetainedDocumentCommands
/// 🧾️ Document verbs as bounded first-step tools (ticket 26/09/17/TRINITY-PLUGIN-END-TO-END): the framework
/// refuses UI dispatch of every command not classified `Migrated`, which left every rule edit dead in the
/// shell. Each verb publishes granular `RewriteRuleMutation`s on the artifact lane — the working-graph verbs (`patchNodes`,
/// `addWorkingNode`, a working-canvas `nodeGraphEdit`) ONE edit of the composed `workingGraph` child (design §20.15) — except
/// `resetRule`, a host-applied `Effect::LoadDocument`.
pub const REWRITING_DOCUMENT_TOOL_IDS: &[&str] = &["addRuleClause", "resetRule", "setActiveExample", "setParameter", "patchNodes", "nodeGraphEdit", "setLhsJson", "setRhsJson", "reorganize", "addWorkingNode"];

const REWRITING_DOCUMENT_PAYLOAD_SCHEMA: &str = "trinity.rewriting.document-command.v1";

const REWRITING_DOCUMENT_RAW_BYTES: usize = 32_768;

fn rewriting_document_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(REWRITING_DOCUMENT_RAW_BYTES, 64, 1, 262_144, 7_500)
}

fn rewriting_document_extent(command: &TrinityRewritingCommand, _snapshot: &RewritingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => surface_id.len().checked_add(operations_json.len())?,
        TrinityRewritingCommand::SetLhsJson { value } | TrinityRewritingCommand::SetRhsJson { value } => value.len(),
        TrinityRewritingCommand::SetParameter { name, value } => name.len().checked_add(value.len())?,
        TrinityRewritingCommand::AddRuleClause { kind } => kind.len(),
        TrinityRewritingCommand::SetActiveExample { example_id } => example_id.len(),
        TrinityRewritingCommand::PatchNodes { node_ids, field, value } => node_ids.iter().map(String::len).try_fold(field.len().checked_add(value.len())?, usize::checked_add)?,
        TrinityRewritingCommand::AddWorkingNode { kind, name, .. } => kind.as_ref().map_or(0, String::len).checked_add(name.as_ref().map_or(0, String::len))?.checked_add(16)?,
        TrinityRewritingCommand::ResetRule | TrinityRewritingCommand::Reorganize => 1,
        _ => return None,
    };
    (bytes <= REWRITING_DOCUMENT_RAW_BYTES).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn rewriting_document_reduce(
    command: &TrinityRewritingCommand,
    state: &RewritingSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TrinityRewritingPlayApp>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    use crate::editor::rewriting::commands;
    let children = || context.map(|context| context.children.as_ref()).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("trinity.rewriting.child-refused"), "the retained document command carries no child view"));
    Ok(match command {
        TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => commands::node_graph_edit::node_graph_edit(state, children()?, surface_id, operations_json, &operation.authoring_seed)?,
        TrinityRewritingCommand::SetLhsJson { value } => commands::set_lhs::set_lhs(state, value)?,
        TrinityRewritingCommand::SetRhsJson { value } => commands::set_rhs::set_rhs(state, value)?,
        TrinityRewritingCommand::SetParameter { name, value } => commands::set_parameter::set_parameter(state, name, value),
        TrinityRewritingCommand::AddRuleClause { kind } => commands::add_rule_clause_command::add_rule_clause_command(state, kind)?,
        TrinityRewritingCommand::ResetRule => commands::reset_rule::reset_rule(state),
        TrinityRewritingCommand::SetActiveExample { example_id } => commands::set_active_example::set_active_example(example_id),
        TrinityRewritingCommand::PatchNodes { node_ids, field, value } => commands::patch_nodes::patch_nodes(state, children()?, node_ids, interaction.selection.get("graph").map_or(&[][..], |selection| selection.ids.as_slice()), field, value)?,
        TrinityRewritingCommand::Reorganize => commands::reorganize::reorganize(state),
        TrinityRewritingCommand::AddWorkingNode { kind, name, x, y } => commands::add_working_node_command::add_working_node_command(state, children()?, kind.as_deref(), name.as_deref(), *x, *y)?,
        _ => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "the rewriting document reducer received a command outside its tool roster")),
    })
}

pub(super) struct RewritingDocumentJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl RewritingDocumentJobFactory {
    pub(super) fn new(controller: &str) -> Self {
        Self { keys: REWRITING_DOCUMENT_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for RewritingDocumentJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<TrinityRewritingPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<TrinityRewritingPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        REWRITING_DOCUMENT_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        rewriting_document_contract()
    }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: REWRITING_DOCUMENT_RAW_BYTES, checkpoint_resume: false);
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for RewritingDocumentJobFactory {
    type Owner = EditorApp<TrinityRewritingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = REWRITING_DOCUMENT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addRuleClause", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "resetRule", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setParameter", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchNodes", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLhsJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setRhsJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWorkingNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    ];
}

pub(super) fn rewriting_build_document_tool_job(request: &mut Option<semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<TrinityRewritingPlayApp>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandInputs, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
    let tool_id = TrinityRewritingPlayApp::command_id(&original.command);
    if tool_id != original.tool_id {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "Rewriting command does not match its exact registered tool").into());
    }
    if rewriting_document_extent(&original.command, &original.snapshot, &original.interaction_state) != Some(1) {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "the rewriting command exceeds the capacity of one bounded edit").into());
    }
    semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, TrinityRewritingPlayApp::command_id, REWRITING_DOCUMENT_RAW_BYTES, 1, false, |request| Box::new(BoundedArtifactCommandWork::new(tool_id, rewriting_document_reduce, rewriting_document_extent)))
}
//#endregion 🧵️RetainedDocumentCommands
