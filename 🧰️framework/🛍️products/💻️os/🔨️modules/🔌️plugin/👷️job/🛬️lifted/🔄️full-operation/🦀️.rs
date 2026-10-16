//! 🔄️ Lift file: the generic bounded full-operation job, its phase and its tool job factory, of `🧵️retained-command/🔄️full-operation/🦀️.rs` that belong to crate `job` (WP3).
//!
//! WP3 owns both crates: `QualifiedBoundedFirstStepProof::admits` names the factory type, and the factory's `ToolJobFactory` impl names the job, so the three items move down together from `retained-command` to `job` (the plan moved the 16-line factory alone, which would have left its impl naming a `retained-command` type). Until `job` is carved, the source module re-exports them with `pub use`.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TypedCommandFullOperationPhase {
    Prepare,
    Reducer,
    Fault,
}

pub(crate) struct TypedCommandFullOperationJob<A: ArtifactApp> {
    pub(crate) operation: Option<semio_framework_job::Operation>,
    pub(crate) parent_document_id: Option<String>,
    pub(crate) command: Option<std::sync::Arc<A::Command>>,
    pub(crate) snapshot: Option<std::sync::Arc<A::Snapshot>>,
    pub(crate) config: Option<std::sync::Arc<A::Config>>,
    pub(crate) history: Option<std::sync::Arc<HistoryView>>,
    pub(crate) children: Option<std::sync::Arc<ChildContentView>>,
    pub(crate) draft: Option<std::sync::Arc<A::Draft>>,
    pub(crate) interaction_state: Option<std::sync::Arc<protocol::InteractionState>>,
    pub(crate) interaction_hover: Option<std::sync::Arc<InteractionHoverState>>,
    pub(crate) peer_presence: Option<std::sync::Arc<PeerPresenceRoot>>,
    pub(crate) presence_local: Option<store::os_store::snapshot_read::SnapshotRead<A::Presence>>,
    pub(crate) presence_peers: Option<std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>>,
    pub(crate) transient: Option<std::sync::Arc<A::Transient>>,
    pub(crate) contract: semio_framework::ToolExecutionContract,
    pub(crate) decoded_items: usize,
    pub(crate) phase: TypedCommandFullOperationPhase,
    pub(crate) prepare_cursor: u8,
    pub(crate) reducer_cursor: u8,
    pub(crate) output: Option<ArtifactToolCompletion<A>>,
    pub(crate) state: [u8; 3],
    pub(crate) fault_message: &'static [u8],
    pub(crate) pending: Option<crate::tool_contract::ReservedOutcomeKind>,
    pub(crate) outcomes: crate::tool_contract::ReservedJobOutcomes,
    pub(crate) closing: bool,
}

impl<A: ArtifactApp> TypedCommandFullOperationJob<A> {
    /// 📏️ The next close turn: the parent id text, else one retained alias per turn.
    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        if !self.outcomes.terminal_is_empty() { return self.outcomes.close_demands(); }
        Ok(self.owner_close_demand())
    }

    fn owner_close_demand(&self) -> RetirementDemand {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if let Some(parent) = self.parent_document_id.as_ref() {
            let demand = plugin_text_retirement_demand(parent);
            return if demand == RetirementDemand::default() { item } else { demand };
        }
        let retained = self.command.is_some() || self.snapshot.is_some() || self.config.is_some() || self.history.is_some() || self.children.is_some() || self.draft.is_some() || self.interaction_state.is_some() || self.interaction_hover.is_some() || self.peer_presence.is_some() || self.presence_local.is_some() || self.presence_peers.is_some() || self.transient.is_some() || self.output.is_some();
        if retained||self.operation.is_some()||!self.closing||self.pending.is_some() { item } else { RetirementDemand::default() }
    }

    pub(crate) fn checkpoint(&mut self) {
        self.state = [self.phase as u8, self.prepare_cursor, self.reducer_cursor];
        self.pending = Some(crate::tool_contract::ReservedOutcomeKind::Checkpoint { applied_progress: u64::from(self.prepare_cursor).saturating_add(u64::from(self.reducer_cursor)) });
    }

    pub(crate) fn fault(&mut self, message: &'static [u8]) {
        self.phase = TypedCommandFullOperationPhase::Fault;
        self.fault_message = message;
        self.pending = Some(crate::tool_contract::ReservedOutcomeKind::Fault);
    }
}

impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use crate::tool_contract::ReservedOutcomeKind;
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.outcomes.is_delivered() {
            self.outcomes.retire(cx)?;
            return Ok(None);
        }
        if let Some(kind) = self.pending {
            let source: &[u8] = match kind {
                ReservedOutcomeKind::Preview => b"typed-command-prepare",
                ReservedOutcomeKind::Fault => self.fault_message,
                _ => &self.state,
            };
            let outcome = self.outcomes.advance(kind, source, cx)?;
            if outcome.is_some() {
                self.pending = None;
            }
            return Ok(outcome);
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.decoded_items > self.contract.max_decoded_items || self.contract.max_work_units_per_step == 0 || self.contract.max_output_bytes == 0 {
            self.fault(b"typed command admission exceeds its fixed item, work, or output cap");
            return Ok(None);
        }
        cx.consume_fuel(1);
        let operation = self.operation.expect("tool job factory assigns operation identity");
        debug_assert_eq!(operation.operation, cx.operation());
        match self.phase {
            TypedCommandFullOperationPhase::Prepare => {
                cx.set_stage("typed-command-prepare");
                let present = match self.prepare_cursor {
                    0 => self.parent_document_id.is_some(),
                    1 => self.command.is_some(),
                    2 => self.snapshot.is_some(),
                    3 => self.config.is_some(),
                    4 => self.history.is_some(),
                    5 => self.children.is_some(),
                    6 => self.draft.is_some(),
                    7 => self.interaction_state.is_some(),
                    8 => self.interaction_hover.is_some(),
                    9 => self.peer_presence.is_some(),
                    10 => self.presence_local.is_some(),
                    11 => self.presence_peers.is_some(),
                    12 => self.transient.is_some(),
                    _ => {
                        self.phase = TypedCommandFullOperationPhase::Reducer;
                        self.checkpoint();
                        return Ok(None);
                    }
                };
                if !present {
                    self.fault(b"typed command preparation root is absent");
                    return Ok(None);
                }
                self.prepare_cursor = self.prepare_cursor.saturating_add(1);
                if self.prepare_cursor == 1 {
                    self.pending = Some(ReservedOutcomeKind::Preview);
                } else {
                    self.checkpoint();
                }
            }
            TypedCommandFullOperationPhase::Reducer => {
                cx.set_stage("typed-command-reducer");
                self.reducer_cursor = self.reducer_cursor.saturating_add(1);
                self.fault(b"generic bounded proof has no resumable app-owned reducer job");
            }
            TypedCommandFullOperationPhase::Fault => self.fault(b"typed command remains faulted"),
        }
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        self.outcomes.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep,close_original_retained_step};
        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=self.close_demand()?;
        close_original_retained_step(cx,demand,|grant|{
            self.closing=true;self.pending=None;
            if !self.outcomes.terminal_is_empty(){return self.outcomes.close_step(grant)}
            let item=RetainedCloneProgress{copied_items:1,..Default::default()};
            if let Some(parent)=self.parent_document_id.as_mut(){return Ok(RetainedCloneStep::Progress(plugin_text_close(parent,grant).unwrap_or_else(||{self.parent_document_id=None;item})))}
            macro_rules! return_alias{($($field:ident),+)=>{$(if let Some(original)=self.$field.as_ref(){if std::sync::Arc::strong_count(original)<=1{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"typed command requires its original final owner retirement cursor"))}drop(self.$field.take());return Ok(RetainedCloneStep::Progress(item));})+}}
            return_alias!(command,snapshot,config,history,children,draft,interaction_state,interaction_hover,peer_presence,presence_peers,transient);
            if self.presence_local.is_some()||self.output.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"typed command requires its original retained completion or snapshot-read retirement cursor"))}
            self.operation=None;Ok(RetainedCloneStep::Complete(item))
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demand()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand()?.depth)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outcomes.terminal_is_empty() && self.owner_terminal_is_empty()
    }
}

impl<A: ArtifactApp> TypedCommandFullOperationJob<A> {
    fn owner_terminal_is_empty(&self) -> bool {
        self.parent_document_id.is_none()
            && self.command.is_none()
            && self.snapshot.is_none()
            && self.config.is_none()
            && self.history.is_none()
            && self.children.is_none()
            && self.draft.is_none()
            && self.interaction_state.is_none()
            && self.interaction_hover.is_none()
            && self.peer_presence.is_none()
            && self.presence_local.is_none()
            && self.presence_peers.is_none()
            && self.transient.is_none()
            && self.output.is_none()
            && self.operation.is_none()
    }
}

pub(crate) struct TypedCommandFullOperationJobFactory<A: ArtifactApp> {
    pub(crate) keys: Vec<semio_framework::ToolFactoryKey>,
    pub(crate) payload_schema_id: String,
    pub(crate) proof: QualifiedBoundedFirstStepProof,
    pub(crate) marker: std::marker::PhantomData<fn() -> A>,
}

impl<A: ArtifactApp> TypedCommandFullOperationJobFactory<A> {
    pub(crate) fn from_proof(proof: QualifiedBoundedFirstStepProof) -> Option<Self> {
        if proof.owner != ToolOwnerWitness::of::<A>() || proof.row.owner != proof.owner || proof.row.factory != BOUNDED_FIRST_STEP_FACTORY || proof.row.factory_type_id.is_some() || proof.row.factory_type_name.is_some() {
            return None;
        }
        let keys = vec![proof.key()];
        let payload_schema_id = proof.schema_id();
        Some(Self { keys, payload_schema_id, proof, marker: std::marker::PhantomData })
    }
}

impl<A: ArtifactApp> semio_framework::ToolJobFactory for TypedCommandFullOperationJobFactory<A> {
    type Payload = TypedCommandFullOperationJob<A>;
    type Job = TypedCommandFullOperationJob<A>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        &self.payload_schema_id
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        self.proof.contract()
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, mut payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        payload.operation = Some(operation);
        Ok(payload)
    }
}
