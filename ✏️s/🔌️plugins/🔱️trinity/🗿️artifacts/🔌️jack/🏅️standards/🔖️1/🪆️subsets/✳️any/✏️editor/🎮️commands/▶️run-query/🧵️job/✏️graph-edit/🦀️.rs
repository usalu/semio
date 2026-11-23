//! ✏️ Jack graph edits publish through the same qualified original child command as query mutations.

use super::*;
use super::mutation_source::QueryMutationSource;
use crate::editor::jack::commands;
use crate::standards::v1::subsets::any::schema::composition::JackComposition;
use semio_framework_job::{OriginalTurnFailure, StepContext};

pub const JACK_GRAPH_EDIT_TOOL_IDS: &[&str] = &["patchNodes", "deleteSelection"];
const RAW_BYTES: usize = 32_768;
const PAYLOAD_SCHEMA: &str = "trinity.jack.graph-edit-command.v1";
const CHECKPOINT_BYTES: usize = 32;
const REPLAY_MAXIMUM_STEPS: u64 = 1_000_000;
pub const JACK_GRAPH_EDIT_EXECUTION_CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(RAW_BYTES, 64, 64, 1_048_576, 7_500, 1, 1);
const PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "patchNodes", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Child] },
];

pub struct JackGraphEditJobFactory<C: JackComposition> {
    composition: std::marker::PhantomData<C>,
    keys: Vec<ToolFactoryKey>,
}

impl<C: JackComposition> JackGraphEditJobFactory<C> {
    pub fn new(controller: &str) -> Self {
        Self { composition: std::marker::PhantomData, keys: JACK_GRAPH_EDIT_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl<C: JackComposition> ToolJobFactory for JackGraphEditJobFactory<C> {
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
        JACK_GRAPH_EDIT_EXECUTION_CONTRACT
    }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: RAW_BYTES, checkpoint_resume: true);
}

impl<C: JackComposition> ArtifactOwnedToolJobFactory for JackGraphEditJobFactory<C> {
    type Owner = Owner<C>;
    const TOOL_IDS: &'static [&'static str] = JACK_GRAPH_EDIT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = PUBLICATION_CONTRACTS;
}

pub fn build_job<C: JackComposition>(request: &mut Option<ArtifactOwnedToolJobRequest<Owner<C>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    use semio_plugin::__value::{ValueError, ValueRefusalKind};
    let original = request.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    let tool = TrinityJackPlayApp::<C>::command_id(original.command.as_ref());
    let Some(tool) = JACK_GRAPH_EDIT_TOOL_IDS.iter().copied().find(|id| *id == tool && *id == original.tool_id) else {
        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "graph edit command does not match its exact factory key").into());
    };
    if crate::editor::jack::retained_commands::jack_retained_document_extent(&original.command, &original.snapshot, &original.interaction_state) != Some(1) {
        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "the jack command exceeds the capacity of one bounded edit").into());
    }
    semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, TrinityJackPlayApp::<C>::command_id, RAW_BYTES, 1, true, |request| JackGraphEditWork::<C>::new(tool, request.operation.operation.0, request.operation.generation.0))
}

/// 🪪️ Retains the original edit command and its qualified child authority until the one child edit is published.
pub struct JackGraphEditWork<C: JackComposition> {
    source: Option<QueryMutationSource<C>>,
    tool: &'static str,
    operation_id: u64,
    generation: u64,
    progress: u64,
    replay_target: Option<u64>,
    staged: bool,
    finished: bool,
    closing: bool,
}

impl<C: JackComposition> JackGraphEditWork<C> {
    pub fn new(tool: &'static str, operation_id: u64, generation: u64) -> Self {
        Self { source: None, tool, operation_id, generation, progress: 0, replay_target: None, staged: false, finished: false, closing: false }
    }

    fn progress(&mut self) -> ArtifactCommandWorkStep<Owner<C>> {
        self.progress = self.progress.saturating_add(1);
        const PREVIEW: &[u8] = br#"{"en":"Preparing graph edit","de":"Graphbearbeitung wird vorbereitet"}"#;
        if self.replay_target.is_some_and(|target| self.progress <= target) {
            if self.replay_target == Some(self.progress) {
                self.replay_target = None;
            }
            ArtifactCommandWorkStep::Replay { stage: "graph-edit-replay", preview: PREVIEW }
        } else {
            ArtifactCommandWorkStep::Progress { stage: "graph-edit-prepare", preview: PREVIEW }
        }
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        self.source.as_ref().filter(|source| !source.terminal_is_empty()).map_or(Ok(Default::default()), |source| source.close_demands(body))
    }
}

impl<C: JackComposition> ArtifactCommandWork<Owner<C>> for JackGraphEditWork<C> {
    fn tool_id(&self) -> &'static str {
        self.tool
    }
    fn workspace_identity(&self) -> u64 {
        self.tool.bytes().fold(self.operation_id.rotate_left(19) ^ self.generation.rotate_left(41), |digest, byte| digest.rotate_left(5) ^ u64::from(byte))
    }
    fn extent(&self, command: &TrinityJackCommand, snapshot: &JackSnapshot, interaction: &protocol::InteractionState, _: Option<&ArtifactOwnedToolJobContext<Owner<C>>>) -> Option<usize> {
        crate::editor::jack::retained_commands::jack_retained_document_extent(command, snapshot, interaction)
    }
    fn receive_original_command(&mut self, command: &mut TrinityJackCommand, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.source.is_none() {
            if cx.should_yield() || cx.is_cancelled() {
                return Ok(false);
            }
            self.source = Some(QueryMutationSource::new());
        }
        self.source.as_mut().map_or(Ok(true), |source| source.receive_command(command, cx))
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, Owner<C>>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<Owner<C>>, OriginalTurnFailure> {
        if self.finished || self.closing {
            return Err(Fault::from("graph edit operation is terminal").into());
        }
        if input.operation.operation_id != self.operation_id || input.operation.generation != self.generation {
            return Err(Fault::from("graph edit operation owner changed").into());
        }
        let source = self.source.as_mut().ok_or_else(|| Fault::from("graph edit requires its original command source"))?;
        if self.staged {
            let Some((_, preparations, root)) = source.advance(input, cx)? else { return Ok(self.progress()) };
            self.finished = true;
            return Ok(ArtifactCommandWorkStep::Complete(semio_plugin::Emit { artifact_mutations: root.into_iter().collect(), child_preparations: preparations.into(), ..Default::default() }));
        }
        let original: Result<_, Fault> = (|| {
            let children = &input.context.ok_or_else(content_missing)?.children;
            let selection = input.interaction.selection.get("ast").map_or(&[][..], |selection| selection.ids.as_slice());
            match input.command {
                TrinityJackCommand::PatchNodes { node_ids, field, value } => Ok((commands::patch_nodes::patch_nodes_effects::<C>(input.snapshot, children, node_ids, selection, field, value)?, None)),
                TrinityJackCommand::DeleteSelection => commands::delete_selection::delete_selection_effects::<C>(input.snapshot, children, selection),
                _ => Err(Fault::from("graph edit received a different command")),
            }
        })();
        let (effects, root) = original?;
        if effects.is_empty() {
            self.finished = true;
            return Ok(ArtifactCommandWorkStep::Complete(semio_plugin::Emit { artifact_mutations: root.into_iter().collect(), ..Default::default() }));
        }
        source.stage(None, effects, root);
        self.staged = true;
        Ok(self.progress())
    }
    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        match index {
            0..=3 => Some(b"JGE1"[index]),
            4..=7 => Some(0),
            8..=15 => Some(self.progress.to_le_bytes()[index - 8]),
            16..=23 => Some(self.operation_id.to_le_bytes()[index - 16]),
            24..=31 => Some(self.generation.to_le_bytes()[index - 24]),
            _ => None,
        }
    }
    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != CHECKPOINT_BYTES || &checkpoint[..4] != b"JGE1" || checkpoint[4..8] != [0, 0, 0, 0] || self.staged || self.progress != 0 {
            return Err(Fault::from("graph edit checkpoint is invalid for this workspace"));
        }
        let word = |range: std::ops::Range<usize>| checkpoint[range].try_into().map(u64::from_le_bytes).map_err(|_| Fault::from("graph edit checkpoint field"));
        let (progress, operation_id, generation) = (word(8..16)?, word(16..24)?, word(24..32)?);
        if progress > REPLAY_MAXIMUM_STEPS || operation_id != self.operation_id || generation != self.generation {
            return Err(Fault::from("graph edit checkpoint owner is invalid"));
        }
        self.replay_target = (progress != 0).then_some(progress);
        Ok(())
    }
    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(source) = self.source.as_mut() {
            source.begin_close();
        }
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if !self.closing {
            return Ok(InteractiveJobCloseStep::Blocked);
        }
        if let Some(source) = self.source.as_mut().filter(|source| !source.terminal_is_empty()) {
            let step = source.close_step(cx)?;
            return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
        }
        self.source = None;
        Ok(InteractiveJobCloseStep::Complete { progress: Default::default() })
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
        self.closing && self.source.as_ref().is_none_or(QueryMutationSource::terminal_is_empty)
    }
}
