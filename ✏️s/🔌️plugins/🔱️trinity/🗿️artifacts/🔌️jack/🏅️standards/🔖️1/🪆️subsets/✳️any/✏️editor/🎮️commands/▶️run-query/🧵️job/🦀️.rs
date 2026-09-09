//! 🧵️ Query execution and its transient result share one retained operation owner.

use crate::editor::jack::config::JackConfigMutation;
use crate::editor::jack::transient::{JackTransientMutation, ReplaceQueryResult};
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
const LANES: &[ArtifactToolPublicationLane] = &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Transient];

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
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
    };
    let work = Box::new(JackQueryWork::new(tool, operation.operation_id, operation.generation));
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
    preparation: Option<crate::executor::QueryExecutionPreparation>,
    execution: Option<crate::executor::QueryExecution>,
    operation_id: u64,
    generation: u64,
    progress: u64,
    replay_target: Option<u64>,
    finished: bool,
    closing: bool,
}

impl JackQueryWork {
    fn new(tool: &'static str, operation_id: u64, generation: u64) -> Self {
        Self { tool, source: None, preparation: None, execution: None, operation_id, generation, progress: 0, replay_target: None, finished: false, closing: false }
    }

    fn identity(&self) -> u64 {
        let tool = match self.tool {
            "runQuery" => 0x7275_6e51_7565_7279,
            "loadExampleQuery" => 0x6c6f_6164_5175_6572,
            _ => 0,
        };
        tool ^ self.operation_id.rotate_left(19) ^ self.generation.rotate_left(41)
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
            emit: Emit { artifact_mutations: mutations, config_mutations: vec![JackConfigMutation::SetQuery(crate::editor::jack::config::SetQuery { value: self.source.as_ref().expect("query source is retained").clone() })], ..Default::default() },
            ephemeral: EphemeralEmit {
                presence: Vec::new(),
                transient: vec![JackTransientMutation::ReplaceQueryResult(ReplaceQueryResult { execution_id: Some(input.operation.operation_id.to_string()), result, error })],
                window_transient: Vec::new(),
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
            TrinityJackCommand::RunQuery { query } => query.as_ref().map_or(0, String::len),
            TrinityJackCommand::LoadExampleQuery { query } => query.len(),
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
        if self.source.is_none() {
            let source = match input.command {
                TrinityJackCommand::RunQuery { query } => query.as_deref().filter(|query| !query.trim().is_empty()).unwrap_or(&input.config.jack_query),
                TrinityJackCommand::LoadExampleQuery { query } => query,
                _ => return Err(Fault::from("query operation command is invalid")),
            };
            if source.len() > QUERY_BYTES {
                return Err(Fault::from("query source exceeds its admitted capacity"));
            }
            self.source = Some(source.to_string());
            return Ok(self.progress("query-source", br#"{"en":"Reading query","de":"Abfrage wird gelesen"}"#));
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
        if checkpoint.len() != QUERY_CHECKPOINT_BYTES || &checkpoint[..4] != b"JQR2" || checkpoint[4..8] != [0, 0, 0, 0] || self.source.is_some() || self.preparation.is_some() || self.execution.is_some() || self.progress != 0 {
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
        semio_framework_job::InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.execution.is_none() && self.preparation.is_none() && self.source.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checkpoint(work: &JackQueryWork) -> [u8; QUERY_CHECKPOINT_BYTES] {
        let mut bytes = [0_u8; QUERY_CHECKPOINT_BYTES];
        assert_eq!(<JackQueryWork as ArtifactCommandWork<Owner>>::checkpoint(work, &mut bytes).expect("query checkpoint"), QUERY_CHECKPOINT_BYTES);
        bytes
    }

    #[test]
    fn query_ownership_checkpoint_binds_exact_operation_and_generation() {
        let mut first = JackQueryWork::new("runQuery", 501, 12);
        first.progress = 37;
        let bytes = checkpoint(&first);
        let mut exact = JackQueryWork::new("runQuery", 501, 12);
        <JackQueryWork as ArtifactCommandWork<Owner>>::restore(&mut exact, &bytes).expect("exact owner restores");
        assert_eq!(exact.replay_target, Some(37));
        let mut xor_collision = JackQueryWork::new("runQuery", 4_194_805, 13);
        assert_eq!(first.identity(), xor_collision.identity(), "fixture reproduces the old XOR collision");
        assert!(<JackQueryWork as ArtifactCommandWork<Owner>>::restore(&mut xor_collision, &bytes).is_err());
        eprintln!("[DEBUG] Jack query checkpoint rejects the prior operation/generation XOR collision");
    }

    #[test]
    fn query_ownership_checkpoint_accepts_legal_long_scan_progress() {
        let mut source = JackQueryWork::new("runQuery", 700, 21);
        source.progress = 1_000_001;
        let bytes = checkpoint(&source);
        let mut restored = JackQueryWork::new("runQuery", 700, 21);
        <JackQueryWork as ArtifactCommandWork<Owner>>::restore(&mut restored, &bytes).expect("legal long scan restores");
        assert_eq!(restored.replay_target, Some(1_000_001));
        source.progress = QUERY_REPLAY_MAXIMUM_STEPS + 1;
        let rejected = checkpoint(&source);
        let mut target = JackQueryWork::new("runQuery", 700, 21);
        assert!(<JackQueryWork as ArtifactCommandWork<Owner>>::restore(&mut target, &rejected).is_err());
        eprintln!("[DEBUG] Jack query checkpoint admits legal scans above one million and rejects progress beyond the derived admission");
    }
}
