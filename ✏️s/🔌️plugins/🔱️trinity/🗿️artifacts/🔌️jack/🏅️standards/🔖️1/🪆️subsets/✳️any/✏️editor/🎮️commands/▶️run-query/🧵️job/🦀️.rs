//! 🧵️ Query execution and its transient result share one retained operation owner.

use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::editor::jack::transient::{JackResultsWindowTransientMutation, JackResultsWindowTransientOwner, ReplaceQueryResult};
use crate::editor::jack::{TrinityJackCommand, TrinityJackPlayApp};
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::standards::v1::subsets::any::schema::graph::TRINITY_GRAPH_SCHEMA;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::ArtifactOwnedToolJobContext;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::{AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, EphemeralEmit, Fault, FaultCode, FaultOrigin};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::RetirementDemand;
use semio_s_artifact_trinity_jack_graph_query::JackGraphQuery;
use crate::standards::v1::subsets::any::schema::content_read::JackContentRead;
use semio_framework_plugin::app::ChildEmitPreparation;

pub type Owner<C> = EditorApp<TrinityJackPlayApp<C>>;
pub const JACK_QUERY_TOOL_IDS: &[&str] = &["runQuery", "loadExampleQuery"];
const RAW_BYTES: usize = 16_384;
const QUERY_BYTES: usize = 4_096;
pub const QUERY_CHECKPOINT_BYTES: usize = 32;
pub const QUERY_REPLAY_MAXIMUM_STEPS: u64 = (QUERY_BYTES as u64) * 16_384 * 16_384 * 16_384 + 1_000_000;
const PAYLOAD_SCHEMA: &str = "trinity.jack.query-command.v1";
const LANES: &[ArtifactToolPublicationLane] = &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Child, ArtifactToolPublicationLane::WindowTransient];

pub struct JackQueryJobFactory<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>{
    composition:std::marker::PhantomData<C>,
    keys: Vec<ToolFactoryKey>,
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> JackQueryJobFactory<C>{
    pub fn new(controller: &str) -> Self {
        Self { composition:std::marker::PhantomData,keys: JACK_QUERY_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ToolJobFactory for JackQueryJobFactory<C>{
    type Payload = ArtifactRetainedCommandPayload<Owner<C>>;
    type Job = ArtifactRetainedCommandJob<Owner<C>>;
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
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: RAW_BYTES, checkpoint_resume: true);
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactOwnedToolJobFactory for JackQueryJobFactory<C>{
    type Owner = Owner<C>;
    const TOOL_IDS: &'static [&'static str] = JACK_QUERY_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "runQuery", lanes: LANES }, ArtifactToolPublicationContract { tool_id: "loadExampleQuery", lanes: LANES }];
}

pub fn build_job<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>(request: &mut Option<ArtifactOwnedToolJobRequest<Owner<C>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    let tool = TrinityJackPlayApp::<C>::command_id(&original.command);
    if tool != original.tool_id || !JACK_QUERY_TOOL_IDS.contains(&tool) {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "query command does not match its exact factory key").into());
    }
    semio_framework_plugin::retained_command::producer::admit_retained_tool_producer_measured(request, output, cx, TrinityJackPlayApp::<C>::command_id, RAW_BYTES, 1, true, Default::default(), |request, _identity| {
        let work = (|| -> Result<_, Fault> {
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
        let source = match request.command.as_ref() {
            TrinityJackCommand::RunQuery { query, .. } => query.as_deref().filter(|query| !query.trim().is_empty()).unwrap_or(&request.snapshot.query),
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
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let adopts_query = matches!(request.command.as_ref(), TrinityJackCommand::LoadExampleQuery { .. }) && source != request.snapshot.query && source.len() <= crate::standards::v1::subsets::any::schema::graph::JACK_QUERY_MAXIMUM_BYTES;
        let work = JackQueryWork::<C>::new(tool, source.to_string(), adopts_query, editor_window_id.to_string(), results_window_id.to_string(), operation.operation_id, operation.generation);
            Ok(work)
        })()
        .map_err(|fault| semio_framework_plugin::__value::ValueError::new(semio_framework_plugin::__value::ValueRefusalKind::InvalidValue, fault.message.to_string()))?;
        Ok((work, Default::default()))
    })
}

/// 🚫️ A query run without the document's child view cannot read or edit the scene.
fn content_missing() -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("trinity.jack.content-missing"), "the query operation carries no child view")
}

pub struct JackQueryWork<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>{
    composition:std::marker::PhantomData<C>,
    tool: &'static str,
    source: Option<String>,
    adopts_query: bool,
    editor_window_id: Option<String>,
    results_window_id: Option<String>,
    preparation: Option<crate::standards::v1::subsets::any::schema::executor::QueryExecutionPreparation>,
    execution: Option<Box<crate::standards::v1::subsets::any::schema::executor::QueryExecution>>,
    operation_id: u64,
    generation: u64,
    pub progress: u64,
    pub replay_target: Option<u64>,
    finished: bool,
    closing: bool,
    retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> JackQueryWork<C>{
    pub fn new(tool: &'static str, source: String, adopts_query: bool, editor_window_id: String, results_window_id: String, operation_id: u64, generation: u64) -> Self {
        Self { composition:std::marker::PhantomData,tool, source: Some(source), adopts_query, editor_window_id: Some(editor_window_id), results_window_id: Some(results_window_id), preparation: None, execution: None, operation_id, generation, progress: 0, replay_target: None, finished: false, closing: false, retirement: None }
    }

    pub fn identity(&self) -> u64 {
        let tool = match self.tool {
            "runQuery" => 0x7275_6e51_7565_7279,
            "loadExampleQuery" => 0x6c6f_6164_5175_6572,
            _ => 0,
        };
        let fold = |seed: u64, value: Option<&String>| value.map_or(seed, |value| value.bytes().fold(seed, |digest, byte| digest.rotate_left(5) ^ u64::from(byte)));
        fold(fold(tool ^ self.operation_id.rotate_left(19) ^ self.generation.rotate_left(41), self.editor_window_id.as_ref()), self.results_window_id.as_ref())
    }

    fn progress(&mut self, stage: &'static str, preview: &'static [u8]) -> ArtifactCommandWorkStep<Owner<C>> {
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
        input: &ArtifactCommandInputs<'_, Owner<C>>,
        result: Option<crate::standards::v1::subsets::any::schema::ast::QueryResult>,
        error: Option<String>,
        preparations:Vec<ChildEmitPreparation>,
    ) -> ArtifactCommandWorkStep<Owner<C>> {
        self.finished = true;
        let mut emit=semio_framework_plugin::Emit{child_preparations:preparations.into(),..Default::default()};
        emit.artifact_mutations = self.adopts_query.then(|| set_query(self.source.as_ref().expect("query source is retained").clone())).into_iter().collect();
        ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit,
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

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactCommandWork<Owner<C>> for JackQueryWork<C>{
    fn tool_id(&self) -> &'static str {
        self.tool
    }
    fn workspace_identity(&self) -> u64 {
        self.identity()
    }
    fn extent(&self, command: &TrinityJackCommand, snapshot: &JackSnapshot, _interaction: &protocol::InteractionState, context: Option<&ArtifactOwnedToolJobContext<Owner<C>>>) -> Option<usize> {
        let bytes = match command {
            TrinityJackCommand::RunQuery { query, results_window_id } => query.as_ref().map_or(0, String::len).saturating_add(results_window_id.len()),
            TrinityJackCommand::LoadExampleQuery { query, results_window_id } => query.len().saturating_add(results_window_id.len()),
            _ => return None,
        };
        let scene = C::bind(snapshot, &context?.children).ok()?;
        (bytes <= QUERY_BYTES && scene.node_count() <= 16_384 && scene.edge_count() <= 16_384 && snapshot.manifest.node_kinds.len() <= 16_384 && snapshot.manifest.edge_kinds.len() <= 16_384 && snapshot.manifest.port_kinds.len() <= 16_384).then_some(1)
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, Owner<C>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<Owner<C>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.finished || self.closing {
            return Err(Fault::from("query operation is terminal"));
        }
        if input.operation.operation_id != self.operation_id || input.operation.generation != self.generation {
            return Err(Fault::from("query operation owner changed"));
        }
        if self.preparation.is_none() && self.execution.is_none() {
            match crate::standards::v1::subsets::any::schema::language_service::parse(self.source.as_ref().expect("query source is retained")) {
                Ok(query) => self.preparation = Some(crate::standards::v1::subsets::any::schema::executor::QueryExecutionPreparation::new(query)),
                Err(error) => return Ok(self.complete(input, None, Some(error), Vec::new())),
            }
            return Ok(self.progress("query-parse", br#"{"en":"Parsing query","de":"Abfrage wird analysiert"}"#));
        }
        if let Some(preparation) = self.preparation.as_mut() {
            let content = C::bind(input.snapshot, &input.context.ok_or_else(content_missing)?.children)?;
            match preparation.step(input.snapshot, &content, QUERY_BYTES) {
                Ok(crate::standards::v1::subsets::any::schema::executor::QueryPreparationStep::Pending) => return Ok(self.progress("query-prepare", br#"{"en":"Preparing query","de":"Abfrage wird vorbereitet"}"#)),
                Ok(crate::standards::v1::subsets::any::schema::executor::QueryPreparationStep::Complete(execution)) => {
                    self.execution = Some(execution);
                    self.preparation = None;
                    return Ok(self.progress("query-prepare", br#"{"en":"Preparing query","de":"Abfrage wird vorbereitet"}"#));
                }
                Err(error) => return Ok(self.complete(input, None, Some(error.into_message()), Vec::new())),
            }
        }
        match self.execution.as_mut().expect("query execution is retained").step() {
            Ok(Some((result, effects))) => {
                let preparations=if effects.is_empty(){Vec::new()}else{C::bind(input.snapshot,&input.context.ok_or_else(content_missing)?.children)?.prepare_effects(&effects).map_err(|error|Fault::from(error.into_message()))?};
                Ok(self.complete(input, Some(result), None, preparations))
            }
            Ok(None) => Ok(self.progress("query-evaluate", br#"{"en":"Evaluating query","de":"Abfrage wird ausgewertet"}"#)),
            Err(error) => Ok(self.complete(input, None, Some(error.into_message()), Vec::new())),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        match index {
            0..=3 => Some(b"JQR2"[index]),
            4..=7 => Some(0),
            8..=15 => Some(self.progress.to_le_bytes()[index - 8]),
            16..=23 => Some(self.operation_id.to_le_bytes()[index - 16]),
            24..=31 => Some(self.generation.to_le_bytes()[index - 24]),
            _ => None,
        }
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
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if !self.closing{return Ok(InteractiveJobCloseStep::Blocked)}
        let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
            if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}
            if self.retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}
            if let Some(execution)=self.execution.as_mut(){
                if execution.terminal_is_empty(){drop(self.execution.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))}
                return execution.close_step(grant)
            }
            if let Some(preparation)=self.preparation.as_mut(){
                if preparation.terminal_is_empty(){drop(self.preparation.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
                return preparation.close_step(grant)
            }
            if self.source.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.source,&mut self.retirement,grant)}
            if self.editor_window_id.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.editor_window_id,&mut self.retirement,grant)}
            store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.results_window_id,&mut self.retirement,grant)
        })?;
        Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.depth)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.execution.is_none() && self.preparation.is_none() && self.source.is_none() && self.editor_window_id.is_none() && self.results_window_id.is_none() && self.retirement.is_none()
    }
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> JackQueryWork<C>{
    /// 📏️ Quotes the next close turn in the fixed order: child cursor, execution, preparation, then each owned string.
    fn close_demands(&self, body: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if let Some(execution) = self.execution.as_ref() {
            return if execution.terminal_is_empty() { Ok(RetirementDemand { release_bytes: size_of::<crate::standards::v1::subsets::any::schema::executor::QueryExecution>(), depth: 1, ..Default::default() }) } else { execution.close_demands(body) };
        }
        if let Some(preparation) = self.preparation.as_ref() {
            return if preparation.terminal_is_empty() { Ok(RetirementDemand { copy_bytes: size_of::<Option<crate::standards::v1::subsets::any::schema::executor::QueryExecutionPreparation>>(), depth: 1, ..Default::default() }) } else { preparation.close_demands(body) };
        }
        if self.source.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.source);
        }
        if self.editor_window_id.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.editor_window_id);
        }
        store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.results_window_id)
    }
}

