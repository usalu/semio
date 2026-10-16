//! 🪗️ The instance components of `instance-history`: CommandHistory, HostEvents, Transactions (WP6, plan section 7.3).
use super::*;

/// 🎸️ Command history: the session command log, the cached history view, backfill marks and the supersede ledger.
pub struct CommandHistory<A: ArtifactApp> {
    /// 🗂️ Keyed on `(store.generation(), log_generation, history_filter)` — any of the three
    /// changing invalidates the cached snapshot/`HistoryView` pair.
    pub cache: Option<AppProjectionCache<A>>,
    /// 🧾️ Append-only session command log — see `🔖️CommandLog`. Never persisted, never
    /// truncated: undo/redo/revert push entries, they never remove any.
    pub command_log: PagedCommandLog,
    pub command_prune_generation: u64,
    pub command_prune_processed: u64,
    pub pending_command_prune: Option<MountedCommandPrune>,
    pub next_command_seq: u64,
    /// 🗂️ Bumped by `push_log_entry`/`record_command` on every log mutation (a push OR a fold)
    /// — part of the cache key so a folded ×count bump alone (no store-generation change) still
    /// invalidates a stale render.
    pub log_generation: u64,
    /// 🧾️ Sequence ids whose host-projected rows changed since the last invocation response.
    pub history_dirty_sequences: HashSet<u64>,
    /// 🗂️ The history view the next refresh patches (design §20.14); it outlives a dropped projection cache.
    pub history_view: Option<HistoryViewCache>,
    /// 🧾️ How far the command log is proven to reference every edit ([`HistoryBackfillMark`]).
    pub history_backfill: Option<HistoryBackfillMark>,
    /// 🐚️ Shell rows (`noteShellCommand`) already undone — they stay in the append-only log.
    pub shell_undone: HashSet<u64>,
    /// 🐚️ Redo stack for those shell rows (chrome order: last undone is first redone after VCS redo empties).
    pub shell_redo: Vec<ShellHistoryReplay>,
    pub history_filter: HistoryCommandFilter,
    /// ✏️ Every `Supersede` transition of the store's event log, classified per author (history edit, its undo, its
    /// redo) — derived from the transitions, so the history rows and undo of history edits survive reload.
    pub supersedes: SupersedeLedger,
}

impl<A: ArtifactApp> CommandHistory<A> {
    /// 🎻️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            cache: None,
            command_log: PagedCommandLog::new(),
            command_prune_generation: 0,
            command_prune_processed: 0,
            pending_command_prune: None,
            next_command_seq: 0,
            log_generation: 0,
            history_dirty_sequences: HashSet::new(),
            history_view: None,
            history_backfill: None,
            shell_undone: HashSet::new(),
            shell_redo: Vec::new(),
            history_filter: HistoryCommandFilter::default(),
            supersedes: SupersedeLedger::default(),
        }
    }
}

/// 🪕️ Host events: the utilities, window roster and typed operations a host event dispatch reached.
pub struct HostEvents {
    pub invocation_kind: Option<ActionKind>,
    /// 🧰️ The utility each window last showed in a dispatch's view, so a switch becomes a [`HostEvent::UtilityChanged`].
    pub host_event_utilities: HashMap<String, Option<String>>,
    /// 🪟️ The window roster the last dispatch's view carried: the windows an instance-wide [`HostEvent`] reaches.
    pub host_event_roster: Vec<semio_framework::ViewWindowInstance>,
    /// 📨️ The typed operations a [`HostEvent`] admitted: never history rows of their own.
    pub host_event_operations: HashSet<u64>,
    /// 🔁️ Set while a host event dispatches, so its own dispatch delivers none.
    pub delivering_host_event: bool,
}

impl HostEvents {
    /// 🎹️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            invocation_kind: None,
            host_event_utilities: HashMap::new(),
            host_event_roster: Vec::new(),
            host_event_operations: HashSet::new(),
            delivering_host_event: false,
        }
    }
}

/// 🎼️ Transactions: the single pending foreign transaction and its proposal.
pub struct Transactions<A: ArtifactApp> {
    /// 🔀️ The single in-flight transaction this instance has prepared into, if any (contract
    /// §5.9: "one pending transaction per instance" — a second `TransactionPrepare` while this
    /// is `Some` is rejected `transaction.instance-busy`, and `dispatch_emit` refuses any
    /// further artifact-mutating command per §5.10).
    pub pending_transaction: Option<PendingTransaction<A::Mutation>>,
    /// 🔀️ Set by `dispatch_emit` instead of applying, when a gesture's mutations carry foreign
    /// steps — drained once by `plugin_exchange` (`take_pending_transaction_proposal`) to frame
    /// `AppFrame::TransactionProposal` (contract §5.1).
    pub pending_transaction_proposal: Option<TransactionProposalDraft>,
}

impl<A: ArtifactApp> Transactions<A> {
    /// 🏺️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            pending_transaction: None,
            pending_transaction_proposal: None,
        }
    }
}
