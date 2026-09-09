//! 🧵️ Query execution and its transient result share one retained operation owner.

use crate::editor::jack::query_window_config::{JackEditorWindowConfigMutation, JackEditorWindowConfigOwner, SetQuery};
use crate::editor::jack::transient::{JackResultsWindowTransientMutation, JackResultsWindowTransientOwner, ReplaceQueryResult};
use crate::editor::jack::{TrinityJackCommand, TrinityJackPlayApp};
use crate::{JackSnapshot, TRINITY_GRAPH_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::ArtifactOwnedToolJobContext;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, Emit, EphemeralEmit, Fault};

type Owner = EditorApp<TrinityJackPlayApp>;
pub(crate) const JACK_QUERY_TOOL_IDS: &[&str] = &["runQuery", "loadExampleQuery"];
const RAW_BYTES: usize = 16_384;
const QUERY_BYTES: usize = 4_096;
const QUERY_CHECKPOINT_BYTES: usize = 32;
const QUERY_REPLAY_MAXIMUM_STEPS: u64 = (QUERY_BYTES as u64) * 16_384 * 16_384 * 16_384 + 1_000_000;
const PAYLOAD_SCHEMA: &str = "trinity.jack.query-command.v1";
const LANES: &[ArtifactToolPublicationLane] = &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient];

pub(crate) struct JackQueryJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl JackQueryJobFactory {
    pub(crate) fn new(controller: &str) -> Self {
        Self { keys: JACK_QUERY_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl ToolJobFactory for JackQueryJobFactory {
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
        ToolExecutionContract::resumable(RAW_BYTES, 64, 1, 1_048_576, 7_500, 1, 1)
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
        if input.declared_bytes() > RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("query wire or checkpoint exceeds admitted ownership"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for JackQueryJobFactory {
    type Owner = Owner;
    const TOOL_IDS: &'static [&'static str] = JACK_QUERY_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "runQuery", lanes: LANES }, ArtifactToolPublicationContract { tool_id: "loadExampleQuery", lanes: LANES }];
}

pub(crate) fn build_job(request: ArtifactOwnedToolJobRequest<Owner>) -> Result<semio_framework::ToolOperationSpec, Fault> {
    let tool = TrinityJackPlayApp::command_id(&request.command);
    if tool != request.tool_id || !JACK_QUERY_TOOL_IDS.contains(&tool) {
        return Err(Fault::from("query command does not match its exact factory key"));
    }
    let view = request.context.view_state.as_ref().ok_or_else(|| Fault::from("Jack query execution requires an exact attached-window roster"))?;
    let editor_window_id = view.window_id.as_deref().ok_or_else(|| Fault::from("Jack query execution requires its originating editor window"))?;
    let editor_kind = view.window_instances.iter().find(|window| window.id == editor_window_id).map(|window| window.window_kind_id.as_str());
    if editor_kind != Some(crate::editor::jack::TRINITY_JACK_PLAY_WINDOW_EDITOR) {
        return Err(Fault::from("Jack query execution origin must be an attached editor window"));
    }
    let results_window_id = match request.command.as_ref() {
        TrinityJackCommand::RunQuery { results_window_id, .. } | TrinityJackCommand::LoadExampleQuery { results_window_id, .. } => results_window_id.as_str(),
        _ => return Err(Fault::from("Jack query execution requires an explicit results-window target")),
    };
    let results_kind = view.window_instances.iter().find(|window| window.id == results_window_id).map(|window| window.window_kind_id.as_str());
    if results_kind != Some(crate::editor::jack::TRINITY_JACK_PLAY_WINDOW_RESULTS) {
        return Err(Fault::from("Jack query execution target must be an attached results window"));
    }
    if request
        .context
        .window_transient
        .as_ref()
        .filter(|window| window.window_id() == results_window_id)
        .and_then(|window| window.get::<JackResultsWindowTransientOwner>())
        .is_none()
    {
        return Err(Fault::from("Jack query execution requires the exact targeted results-window transient snapshot"));
    }
    let editor_config = request
        .context
        .window_config
        .as_ref()
        .filter(|window| window.window_id() == editor_window_id)
        .and_then(|window| window.get::<JackEditorWindowConfigOwner>())
        .ok_or_else(|| Fault::from("Jack query execution requires the exact originating editor-window config snapshot"))?;
    let source = match request.command.as_ref() {
        TrinityJackCommand::RunQuery { query, .. } => query.as_deref().filter(|query| !query.trim().is_empty()).unwrap_or(&editor_config.jack_query),
        TrinityJackCommand::LoadExampleQuery { query, .. } => query,
        _ => unreachable!(),
    };
    if source.len() > QUERY_BYTES {
        return Err(Fault::from("query source exceeds its admitted capacity"));
    }
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
    };
    let work = Box::new(JackQueryWork::new(tool, source.to_string(), editor_window_id.to_string(), results_window_id.to_string(), operation.operation_id, operation.generation));
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
        TrinityJackPlayApp::command_id,
        RAW_BYTES,
        1,
        work,
    )?;
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation))
}

struct JackQueryWork {
    tool: &'static str,
    source: Option<String>,
    editor_window_id: Option<String>,
    results_window_id: Option<String>,
    preparation: Option<crate::executor::QueryExecutionPreparation>,
    execution: Option<Box<crate::executor::QueryExecution>>,
    operation_id: u64,
    generation: u64,
    progress: u64,
    replay_target: Option<u64>,
    finished: bool,
    closing: bool,
}

impl JackQueryWork {
    fn new(tool: &'static str, source: String, editor_window_id: String, results_window_id: String, operation_id: u64, generation: u64) -> Self {
        Self { tool, source: Some(source), editor_window_id: Some(editor_window_id), results_window_id: Some(results_window_id), preparation: None, execution: None, operation_id, generation, progress: 0, replay_target: None, finished: false, closing: false }
    }

    fn identity(&self) -> u64 {
        let tool = match self.tool {
            "runQuery" => 0x7275_6e51_7565_7279,
            "loadExampleQuery" => 0x6c6f_6164_5175_6572,
            _ => 0,
        };
        let fold = |seed: u64, value: Option<&String>| value.map_or(seed, |value| value.bytes().fold(seed, |digest, byte| digest.rotate_left(5) ^ u64::from(byte)));
        fold(fold(tool ^ self.operation_id.rotate_left(19) ^ self.generation.rotate_left(41), self.editor_window_id.as_ref()), self.results_window_id.as_ref())
    }

    fn progress(&mut self, stage: &'static str, preview: &'static [u8]) -> ArtifactCommandWorkStep<Owner> {
        self.progress = self.progress.saturating_add(1);
        if self.replay_target.is_some_and(|target| self.progress <= target) {
            if self.replay_target == Some(self.progress) {
                self.replay_target = None;
            }
            ArtifactCommandWorkStep::Replay { stage: "query-workspace-replay", preview: br#"{"en":"Restoring query","de":"Abfrage wird wiederhergestellt"}"# }
        } else {
            ArtifactCommandWorkStep::Progress { stage, preview }
        }
    }

    fn complete(
        &mut self,
        input: &ArtifactCommandInputs<'_, Owner>,
        result: Option<crate::ast::QueryResult>,
        error: Option<String>,
        mutations: Vec<crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation>,
    ) -> ArtifactCommandWorkStep<Owner> {
        self.finished = true;
        ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit: Emit {
                artifact_mutations: mutations,
                window_config_mutations: vec![semio_framework_plugin::WindowConfigMutation::of::<JackEditorWindowConfigOwner>(
                    self.editor_window_id.as_ref().expect("editor window id is retained"),
                    JackEditorWindowConfigMutation::SetQuery(SetQuery { value: self.source.as_ref().expect("query source is retained").clone() }),
                )],
                ..Default::default()
            },
            ephemeral: EphemeralEmit {
                presence: Vec::new(),
                transient: Vec::new(),
                window_transient: vec![semio_framework_plugin::WindowTransientMutation::of::<JackResultsWindowTransientOwner>(
                    self.results_window_id.as_ref().expect("results window id is retained"),
                    JackResultsWindowTransientMutation::ReplaceQueryResult(ReplaceQueryResult { execution_id: Some(input.operation.operation_id.to_string()), result, error }),
                )],
            },
        }
    }
}

impl ArtifactCommandWork<Owner> for JackQueryWork {
    fn tool_id(&self) -> &'static str {
        self.tool
    }
    fn workspace_identity(&self) -> u64 {
        self.identity()
    }
    fn extent(&self, command: &TrinityJackCommand, snapshot: &JackSnapshot, _interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<Owner>>) -> Option<usize> {
        let bytes = match command {
            TrinityJackCommand::RunQuery { query, results_window_id } => query.as_ref().map_or(0, String::len).saturating_add(results_window_id.len()),
            TrinityJackCommand::LoadExampleQuery { query, results_window_id } => query.len().saturating_add(results_window_id.len()),
            _ => return None,
        };
        let scene = snapshot.content.local_owner::<crate::JackWorkingScene>()?;
        (bytes <= QUERY_BYTES && scene.nodes.len() <= 16_384 && scene.edges.len() <= 16_384 && snapshot.manifest.node_kinds.len() <= 16_384 && snapshot.manifest.edge_kinds.len() <= 16_384 && snapshot.manifest.port_kinds.len() <= 16_384).then_some(1)
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, Owner>) -> Result<ArtifactCommandWorkStep<Owner>, Fault> {
        if self.finished || self.closing {
            return Err(Fault::from("query operation is terminal"));
        }
        if input.operation.operation_id != self.operation_id || input.operation.generation != self.generation {
            return Err(Fault::from("query operation owner changed"));
        }
        if self.preparation.is_none() && self.execution.is_none() {
            match crate::core::parse(self.source.as_ref().expect("query source is retained")) {
                Ok(query) => self.preparation = Some(crate::executor::QueryExecutionPreparation::new(query)),
                Err(error) => return Ok(self.complete(input, None, Some(error), Vec::new())),
            }
            return Ok(self.progress("query-parse", br#"{"en":"Parsing query","de":"Abfrage wird analysiert"}"#));
        }
        if let Some(preparation) = self.preparation.as_mut() {
            match preparation.step(input.snapshot, QUERY_BYTES) {
                Ok(crate::executor::QueryPreparationStep::Pending) => return Ok(self.progress("query-prepare", br#"{"en":"Preparing query","de":"Abfrage wird vorbereitet"}"#)),
                Ok(crate::executor::QueryPreparationStep::Complete(execution)) => {
                    self.execution = Some(execution);
                    self.preparation = None;
                    return Ok(self.progress("query-prepare", br#"{"en":"Preparing query","de":"Abfrage wird vorbereitet"}"#));
                }
                Err(error) => return Ok(self.complete(input, None, Some(error), Vec::new())),
            }
        }
        match self.execution.as_mut().expect("query execution is retained").step() {
            Ok(Some((result, mutations))) => Ok(self.complete(input, Some(result), None, mutations)),
            Ok(None) => Ok(self.progress("query-evaluate", br#"{"en":"Evaluating query","de":"Abfrage wird ausgewertet"}"#)),
            Err(error) => Ok(self.complete(input, None, Some(error), Vec::new())),
        }
    }
    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < QUERY_CHECKPOINT_BYTES {
            return Err(Fault::from("query checkpoint capacity"));
        }
        target[..QUERY_CHECKPOINT_BYTES].fill(0);
        target[..4].copy_from_slice(b"JQR2");
        target[8..16].copy_from_slice(&self.progress.to_le_bytes());
        target[16..24].copy_from_slice(&self.operation_id.to_le_bytes());
        target[24..32].copy_from_slice(&self.generation.to_le_bytes());
        Ok(QUERY_CHECKPOINT_BYTES)
    }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != QUERY_CHECKPOINT_BYTES || &checkpoint[..4] != b"JQR2" || checkpoint[4..8] != [0, 0, 0, 0] || self.source.is_none() || self.editor_window_id.is_none() || self.results_window_id.is_none() || self.preparation.is_some() || self.execution.is_some() || self.progress != 0 {
            return Err(Fault::from("query checkpoint is invalid for this workspace"));
        }
        let progress = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("query checkpoint progress"))?);
        let operation_id = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("query checkpoint operation"))?);
        let generation = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("query checkpoint generation"))?);
        if progress > QUERY_REPLAY_MAXIMUM_STEPS || operation_id != self.operation_id || generation != self.generation {
            return Err(Fault::from("query checkpoint owner is invalid"));
        }
        self.replay_target = (progress != 0).then_some(progress);
        Ok(())
    }
    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(preparation) = self.preparation.as_mut() {
            preparation.begin_close();
        }
        if let Some(execution) = self.execution.as_mut() {
            execution.begin_close();
        }
    }
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(execution) = self.execution.as_mut() {
            let step = execution.close_step(1, maximum_bytes).unwrap_or(store::SnapshotRetirementStep::Blocked);
            if execution.terminal_is_empty() {
                self.execution = None;
            }
            return match step {
                store::SnapshotRetirementStep::Blocked => semio_framework_job::InteractiveJobCloseStep::Blocked,
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                store::SnapshotRetirementStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
            };
        }
        if let Some(preparation) = self.preparation.as_mut() {
            let step = preparation.close_step(1, maximum_bytes).unwrap_or(store::SnapshotRetirementStep::Blocked);
            if preparation.terminal_is_empty() {
                self.preparation = None;
            }
            return match step {
                store::SnapshotRetirementStep::Blocked => semio_framework_job::InteractiveJobCloseStep::Blocked,
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                store::SnapshotRetirementStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
            };
        }
        if let Some(source) = self.source.as_ref() {
            if source.len() > maximum_bytes {
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            }
            let released_bytes = source.len();
            drop(self.source.take());
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(window_id) = self.editor_window_id.as_ref() {
            if window_id.len() > maximum_bytes {
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            }
            let released_bytes = window_id.len();
            drop(self.editor_window_id.take());
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(window_id) = self.results_window_id.as_ref() {
            if window_id.len() > maximum_bytes {
                return semio_framework_job::InteractiveJobCloseStep::Blocked;
            }
            let released_bytes = window_id.len();
            drop(self.results_window_id.take());
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.execution.is_none() && self.preparation.is_none() && self.source.is_none() && self.editor_window_id.is_none() && self.results_window_id.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
