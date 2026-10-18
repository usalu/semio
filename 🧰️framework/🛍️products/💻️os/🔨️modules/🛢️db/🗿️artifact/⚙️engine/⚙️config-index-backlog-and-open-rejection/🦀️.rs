//! ⚙️ Config, index backlog and open rejection: the engine config and struct, the index backlog and consistency resolver and the open rejection. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Engine
/// ⚙️ Construction-time configuration for one `ArtifactEngine`: `db_engine` sets `limits` and
/// `preview_ttl_ms` and spreads `..Default::default()` for the rest.
pub struct ArtifactEngineConfig {
    pub limits: DbLimits,
    /// 🔐️ The real authz/dedupe/DoS-budget gate `submit` admits every not-yet-committed
    /// envelope through and records its committed ones in — see `db_security::SecurityGate::admit_commands`
    /// and `record_committed`. Keyed per-envelope by a `Principal`
    /// synthesized from that envelope's own `actor` (a permissive `"member"` role, `"default"`
    /// tenant) — `SubmitOptions` stays durability-only (see its doc) so this crate's dedupe/authz
    /// story does not require a caller to separately authenticate every submit call.
    pub security: db_security::SecurityGate,
    // 🔀️ dedyn-emit-runtime, O1/R11(c): every real call site (this crate's own `default()`,
    // `db_engine::document_engine_config`'s `other_defaults.emit` spread) constructs `NullEmit` and
    // nothing else — the field is stored but never actually called (`grep '.emit(' this crate: zero
    // hits). Unlike `db_security::SecurityGate` (which genuinely needs `E: Emit` generic so its own
    // tests can inject a `RecordingEmit`), there is no second implementor anywhere in this crate's
    // call graph, so O1 takes the "exactly one impl" branch: concrete `NullEmit`, no `dyn`, no
    // generic param added to this config type.
    pub emit: Arc<NullEmit>,
    pub preview_ttl_ms: u64,
    /// 🧬️ Projection factory: `submit`'s project step registers a fresh
    /// `db_projection::ProjectionEngine` from this on every call it needs one (see `🔖️Engine`'s doc
    /// for why a factory rather than a stored, already-built engine — `db_projection::
    /// ProjectionEngine::new`'s borrowed-`IndexStorage` + owned-`Vec<E>` shape does not
    /// semio_compose_rs with `ArtifactEngine` owning its storage as `Arc<dyn DbStorage>` without
    /// becoming self-referential). Defaults to no projections registered — `db_projection::
    /// NoProjections` (dedyn-fw-os-guestruntime, O1/R1: no first-party `dyn ErasedProjection`
    /// trait object) is uninhabited, so this factory can never actually return anything today; not
    /// one call site repo-wide overrides it with anything else. The day a real caller wants to
    /// register a projection here, it swaps `NoProjections` for its own closed `ErasedProjection`
    /// enum (R11) — this field's own `dyn Fn` closure stays (`dyn Fn` is R1-legal, a std trait).
    pub projections: Arc<dyn Fn() -> Vec<db_projection::NoProjections> + Send + Sync>,
}

impl Default for ArtifactEngineConfig {
    fn default() -> Self {
        let limits = DbLimits::default();
        let policy = db_security::RoleBasedPolicy::new().with_grant(db_security::Grant::allow("member", &["**"], &[db_security::Action::Read, db_security::Action::Write]));
        ArtifactEngineConfig {
            preview_ttl_ms: limits.max_preview_ttl_ms,
            limits,
            security: db_security::SecurityGate::new(policy, db_security::ReplayGuard::new(60_000, 4_096), db_security::BudgetRegistry::new(100_000, 100_000), Arc::new(NullEmit)),
            emit: Arc::new(NullEmit),
            projections: Arc::new(Vec::new),
        }
    }
}

/// 🎭️ The document authority's real, synchronous pipeline: one open document's WAL,
/// materialized state, causal dependency bookkeeping, previews and index backlog — everything
/// `ArtifactAuthority` (the `db_actor`-mailbox wrapper below) drives in finite process-pool turns.
/// The engine is moved only between serialized turns; callers can also use it directly wherever a
/// mailbox is unnecessary (for example this crate's own tests).
pub struct ArtifactEngine {
    pub document: ArtifactId,
    pub(super) protocol_document: protocol::ArtifactId,
    pub storage: Arc<db_storage::DbBackend>,
    pub wal: db_wal::ArtifactWal,
    pub state: DocumentState,
    pub(super) applied: HashMap<String, protocol::MutationEnvelope>,
    pub(super) applied_receipts: HashMap<String, CommandReceipt>,
    pub(super) applied_receipt_order: VecDeque<String>,
    pub(super) durable_group_receipts: HashMap<String, store::durable_group::DurableOwnedGroupJournalReceiptV1>,
    pub(super) durable_group_edit_ids: HashSet<String>,
    pub(super) recent_targets: HashMap<String, Vec<String>>,
    pub(super) actor_seq: HashMap<String, u64>,
    pub frontier: Frontier,
    pub head_edit_id: Option<protocol::MutationId>,
    pub(super) previews: db_preview::PreviewStore,
    pub(super) recent_touches: VecDeque<db_conflict::CommandTouch>,
    pub(super) live_queries: HashMap<u64, db_query::LiveQuery>,
    pub(super) next_live_query_id: u64,
    pub(super) index_backlog: ArtifactIndexBacklog,
    pub config: ArtifactEngineConfig,
}

/// 🗂️ Index entries of commits that are durable in the WAL but not yet written as index runs.
/// A commit appends to it; after the commit's receipt was sent the runner writes every full run
/// (`db_index::RUN_ENTRIES_MAX` entries) through the kinds' owned runs — no listing, level folds
/// bounded per append — so the four index kinds cost one run write per 64 entries and no Ack waits
/// for index I/O. The WAL stays the source of truth: nothing here is durable, and opening a document
/// refills the backlog from the WAL suffix past what each kind's newest run already records.
#[derive(Default)]
pub(super) struct ArtifactIndexBacklog {
    pub(super) commands: VecDeque<(u64, db_index::RecordLocation)>,
    inverses: VecDeque<(u64, db_index::RecordLocation)>,
    actor_seqs: VecDeque<(ActorId, u64, u64)>,
    pub(super) frontiers: VecDeque<Frontier>,
    pub(super) runs: Option<ArtifactIndexRuns>,
}

/// 🗂️ The four kinds' owned runs, listed once on the backlog's first write and kept by its appends; dropped after
/// a failed write, so the next write lists again what storage really holds.
struct ArtifactIndexRuns {
    pub(super) commands: db_index::OwnedRuns,
    inverses: db_index::OwnedRuns,
    actor_seqs: db_index::OwnedRuns,
    pub(super) frontiers: db_index::OwnedRuns,
}

/// 🧾️ How far each index kind of one document already records (its newest run's highest
/// command or commit seq): the replay on open re-queues exactly what lies beyond.
#[derive(Clone, Copy, Default)]
pub(super) struct ArtifactIndexWatermarks {
    pub(super) commands: u64,
    inverses: u64,
    actor_seqs: u64,
    pub(super) frontiers: u64,
}

/// 🛑️ The most entries one index kind may keep waiting for a run write: a `submit` that finds more writes
/// them first, and refuses transiently — before any WAL write — while index storage keeps failing.
pub(super) const INDEX_BACKLOG_ENTRIES_MAX: usize = 64 * db_index::RUN_ENTRIES_MAX;

/// 🧾️ How many batch receipts the engine keeps for a whole-batch resend to be answered with its original
/// receipt; an older resend is answered by the per-envelope dedupe with the current frontier.
pub(super) const APPLIED_RECEIPTS_MAX: usize = 1_024;

impl ArtifactIndexBacklog {
    pub(super) async fn watermarks(storage: &impl db_io_core::IndexStorage, document: &ArtifactId) -> Result<ArtifactIndexWatermarks, DbError> {
        Ok(ArtifactIndexWatermarks {
            commands: db_index::CommandIndex::new(storage, document.clone()).await.indexed_through().await?,
            inverses: db_index::InverseIndex::new(storage, document.clone()).await.indexed_through().await?,
            actor_seqs: db_index::ActorSeqIndex::new(storage, document.clone()).await.indexed_through().await?,
            frontiers: db_index::FrontierIndex::new(storage, document.clone()).await.indexed_through().await?,
        })
    }

    pub(super) fn queue_command(&mut self, watermarks: &ArtifactIndexWatermarks, seq: u64, location: db_index::RecordLocation, actor: ActorId, actor_seq: u64) {
        if seq > watermarks.commands {
            self.commands.push_back((seq, location));
        }
        if seq > watermarks.inverses {
            self.inverses.push_back((seq, location));
        }
        if seq > watermarks.actor_seqs {
            self.actor_seqs.push_back((actor, actor_seq, seq));
        }
    }

    pub(super) fn queue_frontier(&mut self, watermarks: &ArtifactIndexWatermarks, frontier: &Frontier) {
        if frontier.commit_seq > watermarks.frontiers {
            self.frontiers.push_back(frontier.clone());
        }
    }

    pub(super) fn pending(&self) -> usize {
        self.commands.len().max(self.inverses.len()).max(self.actor_seqs.len()).max(self.frontiers.len())
    }

    /// 🚚️ Writes this backlog's runs: every full one, or everything including a partial tail
    /// (`drain`) before WAL history may be deleted. Each written run leaves the backlog at once, so a
    /// failed write resumes exactly where it stopped, and forgets the owned runs, which the next write
    /// lists again.
    pub(super) async fn flush(&mut self, storage: &impl db_io_core::IndexStorage, document: &ArtifactId, drain: bool) -> Result<(), DbError> {
        let written = self.flush_owned(storage, document, drain).await;
        if written.is_err() {
            self.runs = None;
        }
        written
    }

    async fn flush_owned(&mut self, storage: &impl db_io_core::IndexStorage, document: &ArtifactId, drain: bool) -> Result<(), DbError> {
        let full = db_index::RUN_ENTRIES_MAX;
        let take = |len: usize| if len >= full { full } else if drain { len } else { 0 };
        if [self.commands.len(), self.inverses.len(), self.actor_seqs.len(), self.frontiers.len()].into_iter().all(|len| take(len) == 0) {
            return Ok(());
        }
        let commands = db_index::CommandIndex::new(storage, document.clone()).await;
        let inverses = db_index::InverseIndex::new(storage, document.clone()).await;
        let actor_seqs = db_index::ActorSeqIndex::new(storage, document.clone()).await;
        let frontiers = db_index::FrontierIndex::new(storage, document.clone()).await;
        let runs = match self.runs.as_mut() {
            Some(runs) => runs,
            None => self.runs.insert(ArtifactIndexRuns { commands: commands.owned_runs().await?, inverses: inverses.owned_runs().await?, actor_seqs: actor_seqs.owned_runs().await?, frontiers: frontiers.owned_runs().await? }),
        };
        while take(self.commands.len()) > 0 {
            let count = take(self.commands.len());
            let run: Vec<_> = self.commands.iter().take(count).copied().collect();
            commands.record_owned_run(&mut runs.commands, &run).await?;
            self.commands.drain(..count);
        }
        while take(self.inverses.len()) > 0 {
            let count = take(self.inverses.len());
            let run: Vec<_> = self.inverses.iter().take(count).copied().collect();
            inverses.record_owned_run(&mut runs.inverses, &run).await?;
            self.inverses.drain(..count);
        }
        while take(self.actor_seqs.len()) > 0 {
            let count = take(self.actor_seqs.len());
            let run: Vec<_> = self.actor_seqs.iter().take(count).cloned().collect();
            actor_seqs.record_owned_run(&mut runs.actor_seqs, &run).await?;
            self.actor_seqs.drain(..count);
        }
        while take(self.frontiers.len()) > 0 {
            let count = take(self.frontiers.len());
            let run: Vec<_> = self.frontiers.iter().take(count).cloned().collect();
            frontiers.record_owned_run(&mut runs.frontiers, &run).await?;
            self.frontiers.drain(..count);
        }
        Ok(())
    }
}

/// 🧭️ Consistency resolution of a mounted document: its current frontier is the engine's
/// own, and a historical frontier is found among the not-yet-written backlog before the index.
pub(super) struct ArtifactConsistencyResolver<'a, S: db_io_core::IndexStorage> {
    pub current: Frontier,
    pub(super) pending: &'a VecDeque<Frontier>,
    pub commits: db_index::CommitIndex<'a, S>,
    pub(super) frontiers: db_index::FrontierIndex<'a, S>,
}

impl<S: db_io_core::IndexStorage> db_query::ConsistencyResolver for ArtifactConsistencyResolver<'_, S> {
    async fn current_frontier(&self) -> Result<Frontier, DbError> {
        Ok(self.current.clone())
    }

    async fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError> {
        let command_seq = self.commits.lookup(commit_id).await?.ok_or_else(|| DbError::NotFound(format!("unknown commit id {commit_id:?}")))?;
        if let Some(frontier) = self.pending.iter().find(|frontier| frontier.commit_seq == command_seq) {
            return Ok(frontier.clone());
        }
        self.frontiers.lookup(command_seq).await?.ok_or_else(|| DbError::NotFound(format!("no frontier recorded at command_seq {command_seq}")))
    }
}

/// 🧯️ Engine construction rejection that keeps every acquired WAL owner until bounded terminal close.
#[must_use = "engine construction rejection must reach terminal retained cleanup"]
pub enum ArtifactEngineOpenRejected {
    BeforeWal(DbError),
    WalOpen(db_wal::ArtifactWalOpenRejected),
    RetainedWal { cause: DbError, close_error: Option<DbError>, wal: db_wal::ArtifactWal },
}

impl ArtifactEngineOpenRejected {
    pub(super) fn retained_wal(cause: DbError, wal: db_wal::ArtifactWal) -> Self {
        Self::RetainedWal { cause, close_error: None, wal }
    }

    pub(super) fn retained_wal_after_cleanup_error(cause: DbError, close_error: DbError, wal: db_wal::ArtifactWal) -> Self {
        Self::RetainedWal { cause, close_error: Some(close_error), wal }
    }

    pub fn error(&self) -> &DbError {
        match self {
            Self::BeforeWal(cause) => cause,
            Self::WalOpen(rejected) => rejected.error(),
            Self::RetainedWal { cause, .. } => cause,
        }
    }

    pub fn cleanup_error(&self) -> Option<&DbError> {
        match self {
            Self::WalOpen(rejected) => rejected.release_error(),
            Self::RetainedWal { close_error, .. } => close_error.as_ref(),
            Self::BeforeWal(_) => None,
        }
    }

    pub fn has_retained_writer(&self) -> bool {
        match self {
            Self::BeforeWal(_) => false,
            Self::WalOpen(rejected) => rejected.has_release_owner(),
            Self::RetainedWal { .. } => true,
        }
    }

    pub async fn retry_close(self) -> Result<DbError, ArtifactEngineOpenRejected> {
        match self {
            Self::BeforeWal(cause) => Ok(cause),
            Self::WalOpen(rejected) => rejected.retry_close().await.map_err(Self::WalOpen),
            Self::RetainedWal { cause, close_error: _, mut wal } => match wal.close().await {
                Ok(()) => Ok(cause),
                Err(error) => Err(Self::RetainedWal { cause, close_error: Some(error), wal }),
            },
        }
    }
}

impl From<DbError> for ArtifactEngineOpenRejected {
    fn from(error: DbError) -> Self {
        Self::BeforeWal(error)
    }
}

impl From<db_wal::ArtifactWalOpenRejected> for ArtifactEngineOpenRejected {
    fn from(rejected: db_wal::ArtifactWalOpenRejected) -> Self {
        Self::WalOpen(rejected)
    }
}

impl std::fmt::Debug for ArtifactEngineOpenRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ArtifactEngineOpenRejected").field("cause", self.error()).field("cleanup_error", &self.cleanup_error()).field("has_retained_writer", &self.has_retained_writer()).finish()
    }
}
//#endregion 🔖️Engine
//#endregion 🔖️Ids
