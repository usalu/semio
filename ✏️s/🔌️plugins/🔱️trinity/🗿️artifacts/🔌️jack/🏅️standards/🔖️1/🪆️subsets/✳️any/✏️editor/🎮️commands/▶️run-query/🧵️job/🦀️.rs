//! 🧵️ Query execution and its transient result share one retained operation owner.

use crate::standards::v1::subsets::any::schema::ast::{Clause, Query, QueryResult};
use crate::standards::v1::subsets::any::schema::mutations::{TrinityGraphMutation, clear_deleted_root, set_query};
use crate::editor::jack::transient::{JackResultsWindowTransient, JackResultsWindowTransientMutation, JackResultsWindowTransientOwner, ReplaceQueryResult};
use crate::standards::v1::subsets::any::schema::executor::{QUERY_RESULT_MAXIMUM_OWNED_BYTES, QueryExecutionPreparation};
use crate::editor::jack::{TrinityJackCommand, TrinityJackPlayApp};
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::standards::v1::subsets::any::schema::graph::TRINITY_GRAPH_SCHEMA;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory};
use semio_plugin::app::{ArtifactOwnedToolJobContext, ArtifactOwnedToolJobInputs};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_job::InteractiveJobCloseStep;
use semio_plugin::{ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, EphemeralEmit, Fault, FaultCode, FaultOrigin};
use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand};
use semio_s_artifact_trinity_jack_graph_query::JackGraphQuery;
use crate::standards::v1::subsets::any::schema::content_read::JackContentRead;
use semio_plugin::app::ChildEmitPreparation;

#[path = "🪆️mutation-source/🦀️.rs"]
mod mutation_source;
use mutation_source::QueryMutationSource;

pub type Owner<C> = EditorApp<TrinityJackPlayApp<C>>;
pub const JACK_QUERY_TOOL_IDS: &[&str] = &["runQuery", "loadExampleQuery"];
const RAW_BYTES: usize = 16_384;
const QUERY_BYTES: usize = 4_096;
pub const JACK_QUERY_EXECUTION_CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(RAW_BYTES, 64, 64, 1_048_576, 7_500, 1, 1);
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
        JACK_QUERY_EXECUTION_CONTRACT
    }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: RAW_BYTES, checkpoint_resume: true);
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactOwnedToolJobFactory for JackQueryJobFactory<C>{
    type Owner = Owner<C>;
    const TOOL_IDS: &'static [&'static str] = JACK_QUERY_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "runQuery", lanes: LANES }, ArtifactToolPublicationContract { tool_id: "loadExampleQuery", lanes: LANES }];
}

struct QueryJobInputs<'a> {
    source: &'a str,
    adopts_query: bool,
    origin: &'a str,
    target: &'a str,
}

impl QueryJobInputs<'_> {
    fn owned_bytes(&self) -> usize {
        self.source.len().saturating_add(self.origin.len()).saturating_add(self.target.len())
    }
}

/// 🎯️ Borrows the query source and exact destination from the command's captured owner.
fn query_inputs<C: crate::standards::v1::subsets::any::schema::composition::JackComposition>(request: &ArtifactOwnedToolJobInputs<Owner<C>>) -> Result<QueryJobInputs<'_>, Fault> {
    let view = request.context.view_state.as_deref().ok_or_else(|| Fault::from("Jack query execution requires an exact attached-window roster"))?;
    let (target, kind) = TrinityJackPlayApp::<C>::retained_window_transient_target(request.command.as_ref(), Some(view))?
        .ok_or_else(|| Fault::from("Jack query execution requires a results-window target"))?;
    if !view.window_instances.iter().any(|window| window.id == target && window.window_kind_id == kind) {
        return Err(Fault::from("Jack query execution target must be an attached results window"));
    }
    if request.context.window_transient.as_ref()
        .filter(|window| window.window_id() == target)
        .and_then(|window| window.get::<JackResultsWindowTransientOwner>())
        .is_none()
    {
        return Err(Fault::from("Jack query execution requires the exact targeted results-window transient snapshot"));
    }
    let (source, picks_example) = match request.command.as_ref() {
        TrinityJackCommand::RunQuery { query, .. } => (query.as_deref().filter(|query| !query.trim().is_empty()).unwrap_or(&request.snapshot.query), false),
        TrinityJackCommand::LoadExampleQuery { query, .. } => (query.as_str(), true),
        _ => return Err(Fault::from("Jack query execution requires a query command")),
    };
    let limit = if picks_example { crate::standards::v1::subsets::any::schema::graph::JACK_QUERY_MAXIMUM_BYTES } else { QUERY_BYTES };
    if source.len() > limit {
        return Err(Fault::from("query source exceeds its admitted capacity"));
    }
    Ok(QueryJobInputs {
        source,
        adopts_query: picks_example && source != request.snapshot.query,
        origin: view.window_id.as_deref().unwrap_or_default(),
        target,
    })
}

pub fn build_job<C: crate::standards::v1::subsets::any::schema::composition::JackComposition>(request: &mut Option<ArtifactOwnedToolJobRequest<Owner<C>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
    use semio_plugin::__value::{ValueError, ValueRefusalKind};
    let original = request.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    let tool = TrinityJackPlayApp::<C>::command_id(original.command.as_ref());
    if tool != original.tool_id || !JACK_QUERY_TOOL_IDS.contains(&tool) {
        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "query command does not match its exact factory key").into());
    }
    let into_value = |fault: Fault| ValueError::new(ValueRefusalKind::InvalidValue, fault.message.to_string());
    let bytes = query_inputs::<C>(original).map_err(into_value)?.owned_bytes();
    let demand = RetirementDemand { copy_bytes: bytes, capacity_bytes: bytes, depth: 1, ..Default::default() };
    semio_plugin::retained_command::producer::admit_retained_tool_producer_measured_scoped(
        request, output, cx, TrinityJackPlayApp::<C>::command_id, RAW_BYTES, 1, true, demand,
        |request, scope| {
            let inputs = query_inputs::<C>(request).map_err(into_value)?;
            scope.admit_owner(demand)?;
            let source = inputs.source.to_owned();
            let origin = inputs.origin.to_owned();
            let target = inputs.target.to_owned();
            let retained_capacity_bytes = source.capacity() + origin.capacity() + target.capacity();
            let work = JackQueryWork::<C>::new(tool, source, inputs.adopts_query, origin, target, request.operation.operation.0, request.operation.generation.0);
            Ok((work, RetainedCloneProgress { copied_bytes: bytes, retained_capacity_bytes, ..Default::default() }))
        },
    )
}

/// 🚫️ A query run without the document's child view cannot read or edit the scene.
fn content_missing() -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("trinity.jack.content-missing"), "the query operation carries no child view")
}

pub struct JackQueryWork<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>{
    mutations: Option<QueryMutationSource<C>>,
    tool: &'static str,
    source: Option<String>,
    adopts_query: bool,
    origin_window_id: Option<String>,
    results_window_id: Option<String>,
    preparation: Option<QueryExecutionPreparation>,
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
    pub fn new(tool: &'static str, source: String, adopts_query: bool, origin_window_id: String, results_window_id: String, operation_id: u64, generation: u64) -> Self {
        Self { mutations: None, tool, source: Some(source), adopts_query, origin_window_id: Some(origin_window_id), results_window_id: Some(results_window_id), preparation: None, execution: None, operation_id, generation, progress: 0, replay_target: None, finished: false, closing: false, retirement: None }
    }

    pub fn identity(&self) -> u64 {
        let tool = match self.tool {
            "runQuery" => 0x7275_6e51_7565_7279,
            "loadExampleQuery" => 0x6c6f_6164_5175_6572,
            _ => 0,
        };
        let fold = |seed: u64, value: Option<&String>| value.map_or(seed, |value| value.bytes().fold(seed, |digest, byte| digest.rotate_left(5) ^ u64::from(byte)));
        fold(fold(tool ^ self.operation_id.rotate_left(19) ^ self.generation.rotate_left(41), self.origin_window_id.as_ref()), self.results_window_id.as_ref())
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
        result: Option<QueryResult>,
        error: Option<String>,
        preparations: Vec<ChildEmitPreparation>,
        root_edit: Option<TrinityGraphMutation>,
    ) -> ArtifactCommandWorkStep<Owner<C>> {
        self.finished = true;
        let query_edit = (self.adopts_query && error.is_none()).then(|| set_query(self.source.take().expect("query source is retained")));
        let emit = semio_plugin::Emit {
            artifact_mutations: root_edit.into_iter().chain(query_edit).collect(),
            child_preparations: preparations.into(),
            ..Default::default()
        };
        ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit,
            ephemeral: EphemeralEmit {
                presence: Vec::new(),
                transient: Vec::new(),
                window_transient: vec![semio_plugin::WindowTransientMutation::of::<JackResultsWindowTransientOwner>(
                    self.results_window_id.as_ref().expect("results window id is retained"),
                    JackResultsWindowTransientMutation::ReplaceQueryResult(ReplaceQueryResult { execution_id: Some(input.operation.operation_id.to_string()), result, error }),
                )],
            },
        }
    }

    fn prepare_query(&mut self, query: Query) {
        let execution_id_bytes = self.operation_id.checked_ilog10().unwrap_or(0) as usize + 1;
        let window_bytes = size_of::<JackResultsWindowTransient>() + execution_id_bytes;
        let capacity = store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES.saturating_sub(window_bytes);
        self.preparation = Some(QueryExecutionPreparation::with_result_grant(query, capacity.min(QUERY_RESULT_MAXIMUM_OWNED_BYTES)));
    }
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> ArtifactCommandWork<Owner<C>> for JackQueryWork<C>{
    fn tool_id(&self) -> &'static str {
        self.tool
    }
    fn workspace_identity(&self) -> u64 {
        self.identity()
    }
    fn extent(&self, _: &TrinityJackCommand, snapshot: &JackSnapshot, _interaction: &protocol::InteractionState, context: Option<&ArtifactOwnedToolJobContext<Owner<C>>>) -> Option<usize> {
        let scene = C::bind(snapshot, &context?.children).ok()?;
        (scene.node_count() <= 16_384 && scene.edge_count() <= 16_384 && snapshot.manifest.node_kinds.len() <= 16_384 && snapshot.manifest.edge_kinds.len() <= 16_384 && snapshot.manifest.port_kinds.len() <= 16_384).then_some(1)
    }
    fn receive_original_command(&mut self, command: &mut TrinityJackCommand, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.preparation.is_none() {
            if cx.should_yield() || cx.is_cancelled() {
                return Ok(false);
            }
            let Ok(query) = crate::standards::v1::subsets::any::schema::language_service::parse(self.source.as_ref().expect("query source is retained")) else { return Ok(true) };
            if query.clauses.iter().any(|clause| matches!(clause, Clause::Create(_) | Clause::Merge(_) | Clause::Set(_) | Clause::Delete(_))) {
                self.mutations = Some(QueryMutationSource::new());
            }
            self.prepare_query(query);
        }
        self.mutations.as_mut().map_or(Ok(true), |source| source.receive_command(command, cx))
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, Owner<C>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<Owner<C>>, semio_framework_job::OriginalTurnFailure> {
        if self.finished || self.closing {
            return Err(Fault::from("query operation is terminal").into());
        }
        if input.operation.operation_id != self.operation_id || input.operation.generation != self.generation {
            return Err(Fault::from("query operation owner changed").into());
        }
        if let Some(source) = self.mutations.as_mut().filter(|source| source.pending()) {
            if let Some((result, preparations, root)) = source.advance(input, cx)? {
                return Ok(self.complete(input, Some(result), None, preparations, root));
            }
            return Ok(self.progress("query-prepare-edit", br#"{"en":"Preparing graph edit","de":"Graphbearbeitung wird vorbereitet"}"#));
        }
        let original: Result<_, Fault> = (|| {
        if self.preparation.is_none() && self.execution.is_none() {
            match crate::standards::v1::subsets::any::schema::language_service::parse(self.source.as_ref().expect("query source is retained")) {
                Ok(query) => self.prepare_query(query),
                Err(error) => return Ok(self.complete(input, None, Some(error), Vec::new(), None)),
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
                Err(error) => return Ok(self.complete(input, None, Some(error.into_message()), Vec::new(), None)),
            }
        }
        match self.execution.as_mut().expect("query execution is retained").step() {
            Ok(Some((result, effects))) => {
                if effects.is_empty() {
                    return Ok(self.complete(input, Some(result), None, Vec::new(), None));
                }
                let root = clear_deleted_root(input.snapshot, &effects);
                self.mutations.as_mut().ok_or_else(|| Fault::from("Jack graph edits require the original query source"))?.stage(result, effects, root);
                Ok(self.progress("query-prepare-edit", br#"{"en":"Preparing graph edit","de":"Graphbearbeitung wird vorbereitet"}"#))
            }
            Ok(None) => Ok(self.progress("query-evaluate", br#"{"en":"Evaluating query","de":"Abfrage wird ausgewertet"}"#)),
            Err(error) => Ok(self.complete(input, None, Some(error.into_message()), Vec::new(), None)),
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
        if checkpoint.len() != QUERY_CHECKPOINT_BYTES || &checkpoint[..4] != b"JQR2" || checkpoint[4..8] != [0, 0, 0, 0] || self.source.is_none() || self.origin_window_id.is_none() || self.results_window_id.is_none() || self.preparation.is_some() || self.execution.is_some() || self.progress != 0 {
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
        if let Some(source) = self.mutations.as_mut() {
            source.begin_close();
        }
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
    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        if !self.closing {
            return Ok(InteractiveJobCloseStep::Blocked);
        }
        if let Some(source) = self.mutations.as_mut().filter(|source| !source.terminal_is_empty()) {
            let step = source.close_step(cx)?;
            return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
        }
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.close_demands(body)?;
        let step = semio_framework_job::close_original_effect_step(cx, demand, |effect| {
            use store::os_store::snapshot_read::{artifact_retirement_admit_owned, artifact_retirement_box_close_step};
            if self.terminal_is_empty() {
                return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 });
            }
            if self.retirement.is_some() {
                return artifact_retirement_box_close_step(&mut self.retirement, body, effect);
            }
            if let Some(execution) = self.execution.as_mut() {
                if !execution.terminal_is_empty() {
                    return execution.close_step(effect);
                }
            }
            if let Some(preparation) = self.preparation.as_mut() {
                if !preparation.terminal_is_empty() {
                    return preparation.close_step(effect);
                }
            }
            if self.execution.is_some() || self.preparation.is_some() {
                let turn = match semio_framework_value::retirement::record_retirement_work(effect, 1) {
                    Ok(work) => {
                        drop(self.execution.take());
                        drop(self.preparation.take());
                        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() };
                        RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(progress)), work }
                    }
                    Err((error, work)) => RetainedWorkOutcome { result: Err(error.into()), work },
                };
                return RetainedWorkPoll::Complete(turn);
            }
            let turn = if self.source.is_some() {
                artifact_retirement_admit_owned(&mut self.source, &mut self.retirement, effect)
            } else if self.origin_window_id.is_some() {
                artifact_retirement_admit_owned(&mut self.origin_window_id, &mut self.retirement, effect)
            } else {
                artifact_retirement_admit_owned(&mut self.results_window_id, &mut self.retirement, effect)
            };
            RetainedWorkPoll::Complete(turn).map_err(Into::into)
        })?;
        Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step })
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
        self.closing && self.mutations.as_ref().is_none_or(QueryMutationSource::terminal_is_empty) && self.execution.is_none() && self.preparation.is_none() && self.source.is_none() && self.origin_window_id.is_none() && self.results_window_id.is_none() && self.retirement.is_none()
    }
}

impl<C:crate::standards::v1::subsets::any::schema::composition::JackComposition> JackQueryWork<C>{
    /// 📏️ Quotes the next retained query work owner to close.
    fn close_demands(&self, body: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        if let Some(source) = self.mutations.as_ref().filter(|source| !source.terminal_is_empty()) {
            return source.close_demands(body);
        }
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
        if self.origin_window_id.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.origin_window_id);
        }
        store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.results_window_id)
    }
}

