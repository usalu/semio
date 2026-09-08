//! 🧵️ Query execution and its transient result share one retained operation owner.

use crate::editor::jack::{JackConfig, JackConfigMutation, TrinityJackCommand, TrinityJackPlayApp};
use crate::editor::jack::transient::{JackTransientMutation, ReplaceQueryResult};
use crate::{JackSnapshot, TRINITY_GRAPH_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobContext, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, Emit, EphemeralEmit, Fault};

type Owner = EditorApp<TrinityJackPlayApp>;
pub(crate) const JACK_QUERY_TOOL_IDS: &[&str] = &["runQuery", "loadExampleQuery"];
const RAW_BYTES: usize = 16_384;
const QUERY_BYTES: usize = 8_192;
const PAYLOAD_SCHEMA: &str = "trinity.jack.query-command.v1";
const LANES: &[ArtifactToolPublicationLane] = &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient];

pub(crate) struct JackQueryJobFactory { keys: Vec<ToolFactoryKey> }

impl JackQueryJobFactory {
    pub(crate) fn new(controller: &str) -> Self { Self { keys: JACK_QUERY_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() } }
}

impl ToolJobFactory for JackQueryJobFactory {
    type Payload = ArtifactRetainedCommandPayload<Owner>;
    type Job = ArtifactRetainedCommandJob<Owner>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::resumable(RAW_BYTES, 64, 1, 1_048_576, 7_500, 1, 1) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > RAW_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("query wire or checkpoint exceeds admitted ownership"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for JackQueryJobFactory {
    type Owner = Owner;
    const TOOL_IDS: &'static [&'static str] = JACK_QUERY_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "runQuery", lanes: LANES },
        ArtifactToolPublicationContract { tool_id: "loadExampleQuery", lanes: LANES },
    ];
}

pub(crate) fn build_job(request: ArtifactOwnedToolJobRequest<Owner>) -> Result<semio_framework::ToolOperationSpec, Fault> {
    let tool = TrinityJackPlayApp::command_id(&request.command);
    if tool != request.tool_id || !JACK_QUERY_TOOL_IDS.contains(&tool) { return Err(Fault::from("query command does not match its exact factory key")); }
    let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id.clone(), operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision };
    let work = Box::new(JackQueryWork { tool, source: None, execution: None, finished: false, closing: false });
    let payload = ArtifactRetainedCommandPayload::try_new(
        ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion },
        TrinityJackPlayApp::command_id, RAW_BYTES, 1, work,
    )?;
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation))
}

struct JackQueryWork {
    tool: &'static str,
    source: Option<String>,
    execution: Option<crate::executor::QueryExecution>,
    finished: bool,
    closing: bool,
}

impl JackQueryWork {
    fn complete(&mut self, input: &ArtifactCommandInputs<'_, Owner>, result: Option<crate::ast::QueryResult>, error: Option<String>, mutations: Vec<crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation>) -> ArtifactCommandWorkStep<Owner> {
        self.finished = true;
        ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit: Emit { artifact_mutations: mutations, config_mutations: vec![JackConfigMutation::SetQuery(crate::editor::jack::config::SetQuery { value: self.source.as_ref().expect("query source is retained").clone() })], ..Default::default() },
            ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![JackTransientMutation::ReplaceQueryResult(ReplaceQueryResult { execution_id: Some(input.operation.operation_id.to_string()), result, error })] },
        }
    }
}

impl ArtifactCommandWork<Owner> for JackQueryWork {
    fn tool_id(&self) -> &'static str { self.tool }
    fn extent(&self, command: &TrinityJackCommand, _snapshot: &JackSnapshot, _interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<Owner>>) -> Option<usize> {
        let bytes = match command { TrinityJackCommand::RunQuery { query } => query.as_ref().map_or(0, String::len), TrinityJackCommand::LoadExampleQuery { query } => query.len(), _ => return None };
        (bytes <= QUERY_BYTES).then_some(1)
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, Owner>) -> Result<ArtifactCommandWorkStep<Owner>, Fault> {
        if self.finished || self.closing { return Err(Fault::from("query operation is terminal")); }
        if self.source.is_none() {
            let source = match input.command {
                TrinityJackCommand::RunQuery { query } => query.as_deref().filter(|query| !query.trim().is_empty()).unwrap_or(&input.config.jack_query),
                TrinityJackCommand::LoadExampleQuery { query } => query,
                _ => return Err(Fault::from("query operation command is invalid")),
            };
            if source.len() > QUERY_BYTES { return Err(Fault::from("query source exceeds its admitted capacity")); }
            self.source = Some(source.to_string());
            return Ok(ArtifactCommandWorkStep::Progress { stage: "query-source", preview: br#"{"en":"Reading query","de":"Abfrage wird gelesen"}"# });
        }
        if self.execution.is_none() {
            let prepared = crate::Graph::from_fixture(input.snapshot.clone()).map_err(|error| error.to_string()).and_then(|graph| crate::core::parse(self.source.as_ref().expect("query source is retained")).map(|query| crate::executor::QueryExecution::new(graph, query)));
            match prepared {
                Ok(execution) => self.execution = Some(execution),
                Err(error) => return Ok(self.complete(input, None, Some(error), Vec::new())),
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "query-prepare", preview: br#"{"en":"Preparing query","de":"Abfrage wird vorbereitet"}"# });
        }
        match self.execution.as_mut().expect("query execution is retained").step() {
            Ok(Some((result, mutations))) => Ok(self.complete(input, Some(result), None, mutations)),
            Ok(None) => Ok(ArtifactCommandWorkStep::Progress { stage: "query-evaluate", preview: br#"{"en":"Evaluating query","de":"Abfrage wird ausgewertet"}"# }),
            Err(error) => Ok(self.complete(input, None, Some(error), Vec::new())),
        }
    }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 { return semio_framework_job::InteractiveJobCloseStep::Blocked; }
        if self.execution.take().is_some() || self.source.take().is_some() { return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }; }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.execution.is_none() && self.source.is_none() }
}
