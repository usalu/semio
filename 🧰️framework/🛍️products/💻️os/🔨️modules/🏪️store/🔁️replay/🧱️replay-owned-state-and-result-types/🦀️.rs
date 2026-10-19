//! 🧱️ Replay owned state and result types: the replay owned state with its constructor and the replay result owned state. See [`super`].

use super::*;

pub struct ReplayOwnedState<P, Mutation>
where
    Mutation: super::Mutation<P>,
{
    pub mode: ReplayMode,
    pub order: semio_framework_os_vcs::HistoryPageStack<String>,
    pub from: usize,
    pub(crate) prefix_until: usize,
    pub prefix_work_progress: bool,
    pub(super) operation_progress: bool,
    pub cursor: usize,
    pub operation: usize,
    pub schema: String,
    pub supersessions: EffectiveSupersessions,
    pub state: Option<Arc<P>>,
    pub candidate: Option<Arc<P>>,
    pub edit_messages: semio_framework_os_spr::command::ReplayMessageAccumulator,
    pub edit_inverse: semio_framework_value::list::PagedList<Mutation, {usize::MAX}>,
    pub message_settlement: Option<semio_framework_os_store_ledger::edit_message_clamp::settlement::EditMessageSettlement>,
    pub(super) closed_outcomes: usize,
    pub committed: Vec<String>,
    pub(crate) quarantined: Vec<String>,
    pub rebased_inverse: Vec<(String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>)>,
    pub replayed: Vec<semio_framework_os_spr::EditMessages>,
    pub outcomes: Vec<protocol::MutationReplayOutcome>,
    pub(crate) report_worst: Option<semio_framework_diagnostic::Severity>,
    pub unit_flags: protocol::HistoryFoldIndex<([u8; 32], usize), bool>,
    pub operation_preparation_factory: Option<Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>>,
    pub(crate) operation_preparation: Option<ArtifactReplayPreparationTask<P, Mutation>>,
    pub prepared_operation: Option<ArtifactReplayPrepared<P, Mutation>>,
    pub(crate) operation_retirement: Option<ArtifactReplayPreparationRetirement<P, Mutation>>,
    pub(crate) operation_message_settlement: Option<replay_preparation::OperationMessageSettlement>,
    pub(crate) preparation_completed: ArtifactReplayPreparationProgress,
        pub(crate) native_receipt:RetainedCloneProgress,
pub(crate) replay_retirement_factory: Option<Arc<dyn replay_preparation::ArtifactReplayRetirementFactory<P, Mutation>>>,
    pub replay_retirement: Option<replay_preparation::ReplayRetirement<P, Mutation>>,
    pub(crate) prefix_message_retirement: Option<ArtifactStoreMessageLedgerRetirement>,
    pub(crate) convergence: Option<ReplayConvergence<P>>,
    pub convergence_search: Option<usize>,
    pub(super) convergence_pending_finish: bool,
    pub converged_at: Option<usize>,
    pub(super) record_stride: usize,
    pub recorded: Vec<(usize, Arc<P>)>,
    pub drafts: protocol::HistoryInputDrafts,
    pub generation: u64,
    pub revision: [u8; 32],
    pub progress: ReplayProgress,
    pub finished: bool,
}

impl<P, M: Mutation<P>> ReplayOwnedState<P, M> {
    pub fn terminal() -> Self {
        Self {
            mode: ReplayMode::Report, order: semio_framework_os_vcs::HistoryPageStack::empty(), from: 0, prefix_until: 0,
            prefix_work_progress: false, operation_progress: false, cursor: 0, operation: 0, schema: String::new(),
            supersessions: Default::default(), state: None, candidate: None, edit_messages: Default::default(),
            edit_inverse: Default::default(), message_settlement: None, closed_outcomes: 0, committed: Vec::new(),
            quarantined: Vec::new(), rebased_inverse: Vec::new(), replayed: Vec::new(), outcomes: Vec::new(),
            report_worst: None, unit_flags: protocol::HistoryFoldIndex::new(), operation_preparation_factory: None,
            operation_preparation: None, prepared_operation: None, operation_retirement: None,
            operation_message_settlement: None, preparation_completed: Default::default(),native_receipt:Default::default(), replay_retirement_factory: None,
            replay_retirement: None, prefix_message_retirement: None, convergence: None, convergence_search: None,
            convergence_pending_finish: false, converged_at: None, record_stride: 1, recorded: Vec::new(),
            drafts: protocol::HistoryInputDrafts::new(), generation: 0, revision: [0; 32],
            progress: ReplayProgress { done: 0, total: 0 }, finished: true,
        }
    }
}

/// 🎁️ What a finished [`EditReplay`] produced: the projection it reached, the edits it kept and quarantined, their
/// rebased inverses and messages, and its per-mutation [`protocol::ReplayReport`]. Untaken owners remain attached.
pub struct ReplayResultOwnedState<P, Mutation>
where
    Mutation: super::Mutation<P>,
{
    pub unit_flags: protocol::HistoryFoldIndex<([u8; 32], usize), bool>,
    pub order: semio_framework_os_vcs::HistoryPageStack<String>,
    pub from: usize,
    pub supersessions: EffectiveSupersessions,
    pub drafts: protocol::HistoryInputDrafts,
    pub generation: u64,
    pub revision: [u8; 32],
    pub state: Option<Arc<P>>,
    pub converged_at: Option<usize>,
    pub committed: Vec<String>,
    pub(crate) quarantined: Vec<String>,
    pub rebased_inverse: Vec<(String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>)>,
    pub replayed: Vec<semio_framework_os_spr::EditMessages>,
    pub recorded: Vec<(usize, Arc<P>)>,
    pub report: protocol::ReplayReport,
}

impl<P, Mutation> ReplayResultOwnedState<P, Mutation>
where
    Mutation: super::Mutation<P>,
{
    /// ⏲️ The store generation the replay was started against.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// 📃️ The draft inputs the replay previewed.
    pub fn drafts(&self) -> &protocol::HistoryInputDrafts {
        &self.drafts
    }

    /// 🖼️ The projection the replay reached (the previous head when it converged).
    pub fn state(&self) -> Option<&Arc<P>> {
        self.state.as_ref()
    }

    /// 🎴️ Takes the reached projection; its retirement moves to the caller.
    pub fn take_state(&mut self) -> Option<Arc<P>> {
        self.state.take()
    }

    /// 🤝️ The prefix length at which the replay met the previous projection, if it did.
    pub fn converged_at(&self) -> Option<usize> {
        self.converged_at
    }

    /// ✅️ The edits the replay kept, in order.
    pub fn committed(&self) -> &[String] {
        &self.committed
    }

    /// 🚧️ The edits a `Merge` policy quarantined, in order.
    pub fn quarantined(&self) -> &[String] {
        &self.quarantined
    }

    /// 📨️ Every replayed edit's messages, in order.
    pub fn replayed(&self) -> &[semio_framework_os_spr::EditMessages] {
        &self.replayed
    }

    /// 📊️ The per-mutation outcomes.
    pub fn report(&self) -> &protocol::ReplayReport {
        &self.report
    }

    /// 🪝️ Takes the per-mutation outcomes.
    pub fn take_report(&mut self) -> protocol::ReplayReport {
        std::mem::take(&mut self.report)
    }

    /// ☑️ Takes the kept edit ids.
    pub fn take_committed(&mut self) -> Vec<String> {
        std::mem::take(&mut self.committed)
    }

    /// 🪤️ Takes the quarantined edit ids.
    pub fn take_quarantined(&mut self) -> Vec<String> {
        std::mem::take(&mut self.quarantined)
    }

    /// 📬️ Takes the replayed edits' messages.
    pub fn take_replayed(&mut self) -> Vec<semio_framework_os_spr::EditMessages> {
        std::mem::take(&mut self.replayed)
    }

    /// 🧳️ Takes the recorded prefix projections `(length, projection)`; their retirement moves to the caller.
    pub fn take_recorded(&mut self) -> Vec<(usize, Arc<P>)> {
        std::mem::take(&mut self.recorded)
    }

    /// 🔙️ Takes every kept edit's rebased inverse; their retirement moves to the caller.
    pub fn take_rebased_undo_rows(&mut self) -> Vec<(String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>)> {
        std::mem::take(&mut self.rebased_inverse)
    }
}
