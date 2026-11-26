//! 🪗️ The instance components of `instance-history`: CommandHistory, HostEvents, Transactions (WP6, plan section 7.3).
use semio_framework::{ActionKind, FaultFrom};
use semio_plugin_document_media::AppProjectionCache;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_store_lifecycle::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_plugin_decl_emission::Emit;
use std::future::Future;
use std::collections::{HashMap, HashSet};
use semio_plugin_command::{HistoryBackfillMark, HistoryViewCache};
use semio_plugin_decl_views::history_view::{HistoryCommandFilter, ShellHistoryReplay};
use store::{MemberFactory, Mutation, SpaceMember};
use semio_plugin_publication_mounted::command_mounted::MountedCommandPrune;
use protocol::OpText;
use semio_plugin_publication_mounted::mounted_child_group_receipt_group::command::PagedCommandLog;
use semio_plugin_job::{PendingTransaction, TransactionProposalDraft};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_time_travel::SupersedeLedger;
use semio_ui_contract::*;

/// 🎸️ Command history: the session command log, the cached history view, backfill marks and the supersede ledger.
pub struct CommandHistory<A: ArtifactApp> {
    /// 🗂️ Keyed on `(store.generation(), log_generation, history_filter)` — any of the three
    /// changing invalidates the cached snapshot/`HistoryView` pair.
    pub cache: Option<AppProjectionCache<A>>,
    /// 🤝️ The cached snapshot's funded shared-owner retirement: the last alias hands its pages back instead of dropping them.
    pub cache_retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    /// 🅿️ Invalidated caches that were the last alias of their snapshot or configuration; their sealed pages wait here for [`CommandHistory::retire_cache_step`] instead of dropping plainly.
    pub cache_parked: Vec<AppProjectionCache<A>>,
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
            cache_parked: Vec::new(),
            command_log: PagedCommandLog::new(),
            command_prune_generation: 0,
            command_prune_processed: 0,
            pending_command_prune: None,
            next_command_seq: 0,
            log_generation: 0,
            cache_retirement: None,
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

/// 🗂️ The three shared roots of one projection cache, handed to their funded retirement together.
pub type CacheRetirementRoots<A> = (
    semio_framework_value::retirement::arc_custody::ArcLease<<A as ArtifactApp>::Snapshot>,
    semio_framework_value::retirement::arc_custody::ArcLease<<A as ArtifactApp>::Config>,
    semio_framework_value::retirement::arc_custody::ArcLease<semio_plugin_decl_views::history_view::HistoryView>,
);

impl<A: ArtifactApp> CommandHistory<A> {
    /// 🗑️ Invalidates the projection cache; a cache that is the last alias of its snapshot or configuration parks for [`Self::retire_cache_step`] instead of dropping its sealed pages plainly.
    pub fn invalidate_cache(&mut self) {
        let Some(cache) = self.cache.take() else { return };
        if std::sync::Arc::strong_count(&cache.1) == 1 { self.cache_parked.push(cache) }
    }

    /// ⏳️ Whether a parked cache or its funded retirement still holds pages.
    pub fn cache_parked_pending(&self) -> bool { !self.cache_parked.is_empty() || self.cache_retirement.is_some() }

    /// 📏️ Quotes the next cache retirement step; the live cache counts only when `live` (the close ladder).
    pub fn cache_retirement_demand(&self, live: bool, body: usize) -> Result<Option<semio_framework_value::RetirementDemand>, semio_framework_value::ValueError> {
        if let Some(owner) = self.cache_retirement.as_ref() { return semio_framework_value::factory_ticket_demands(owner, body).map(Some) }
        if self.cache_parked.is_empty() && !(live && self.cache.is_some()) { return Ok(None) }
        let bytes = semio_framework_value::owned_retirement_birth_bytes::<CacheRetirementRoots<A>>();
        Ok(Some(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<Option<CacheRetirementRoots<A>>>() + bytes, capacity_bytes: bytes, depth: 1, ..Default::default() }))
    }

    /// ♻️ Hands one parked (or, when `live`, the live) cache to its funded retirement and closes that retirement step by step.
    pub fn retire_cache_step(&mut self, live: bool, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<Option<semio_framework_value::retained_clone::RetainedCloneProgress>, semio_framework_value::ValueError> {
        use semio_framework_value::retirement::arc_custody::ArcLease;
        if self.cache_retirement.is_some() {
            let poll = semio_framework_value::close_factory_ticket(&mut self.cache_retirement, effect.grant().maximum_copy_bytes, effect);
            return semio_plugin_ownership::bounded_ownership::plugin_settle_value(poll).map(|step| Some(step.progress()));
        }
        let parked = !self.cache_parked.is_empty();
        let next = if parked { self.cache_parked.pop() } else if live { self.cache.take() } else { None };
        let Some((key, snapshot, config, history)) = next else { return Ok(None) };
        let mut roots = Some((ArcLease::new(snapshot), ArcLease::new(config), ArcLease::new(history)));
        let admission = semio_framework_value::admit_owned_retirement(&mut roots, effect);
        if let Some((ArcLease(snapshot), ArcLease(config), ArcLease(history))) = roots {
            let cache = (key, snapshot.unwrap(), config.unwrap(), history.unwrap());
            if parked { self.cache_parked.push(cache) } else { self.cache = Some(cache) }
        }
        Ok(Some(match admission.result? { Some((owner, progress)) => { self.cache_retirement = Some(owner); progress } None => Default::default() }))
    }
}

/// 📣️ A marker occupies the same fixed physical slot as its genuine typed operation reservation.
pub struct HostEventOperations{slots:[Option<u64>;semio_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS]}
impl Default for HostEventOperations{fn default()->Self{Self{slots:[None;semio_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS]}}}
impl HostEventOperations{
 pub fn insert(&mut self,operation:u64){let index=operation as usize%self.slots.len();self.slots[index]=Some(operation);}
 pub fn remove(&mut self,operation:&u64)->bool{let index=*operation as usize%self.slots.len();if self.slots[index]!=Some(*operation){return false}self.slots[index]=None;true}
}
/// 🪕️ Host events: the utilities, window roster and typed operations a host event dispatch reached.
pub struct HostEvents {
    pub routes: semio_plugin_decl_emission::original_view::routing::OriginalHostEventRoutes,
    pub action_delivery:semio_plugin_decl_emission::original_view::delivery::OriginalHostEventDelivery,
    pub action_event_complete:bool,
    pub action_event_removed:bool,
    pub invocation_kind: Option<ActionKind>,
    /// 🧰️ The utility each window last showed in a dispatch's view, so a switch becomes a [`HostEvent::UtilityChanged`].
    pub host_event_utilities: HashMap<String, Option<String>>,
    /// 🪟️ The window roster the last dispatch's view carried: the windows an instance-wide [`HostEvent`] reaches.
    pub host_event_roster: Vec<semio_framework::ViewWindowInstance>,
    /// 📨️ The typed operations a [`HostEvent`] admitted: never history rows of their own.
    pub host_event_operations: HostEventOperations,
    /// 📬️ The window facts waiting for their original typed delivery, front first; the front is the one in flight while `delivering`.
    pub queue: std::collections::VecDeque<semio_plugin_decl_emission::HostEvent>,
    /// 🏃️ Set from the first queued delivery until the queue has drained through the shared typed source.
    pub delivering: bool,
    /// 💬️ Set while the `hostEvent` action itself waits for its queued fact: the reply follows the last delivery.
    pub reply: bool,
    /// 🔁️ The resolved task follow-ups waiting for their original typed delivery.
    pub resumes: semio_plugin_command::command_admission::original_typed_command::TaskResumes,
}

impl HostEvents {
    /// 🎹️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            invocation_kind: None,
            routes: Default::default(),
            action_delivery:Default::default(),
            action_event_complete:false,
            action_event_removed:false,
            host_event_utilities: HashMap::new(),
            host_event_roster: Vec::new(),
            host_event_operations: HostEventOperations::default(),
            queue: std::collections::VecDeque::new(),
            delivering: false,
            reply: false,
            resumes: semio_plugin_command::command_admission::original_typed_command::TaskResumes::new(),
        }
    }

    semio_framework_value::initialization::original_empty_fields!(OWN_INITIAL_FIELDS, own_field_demands, initialize_own_field_at;
        invocation_kind = None,
        action_delivery = Default::default(),
        action_event_complete = false,
        action_event_removed = false,
        host_event_utilities = HashMap::new(),
        host_event_roster = Vec::new(),
        host_event_operations = HostEventOperations::default(),
        queue = std::collections::VecDeque::new(),
        delivering = false,
        reply = false,
        resumes = semio_plugin_command::command_admission::original_typed_command::TaskResumes::new(),
    );

    /// 📋️ Counts both route and host-event fields in their final-address inventory.
    pub const INITIAL_FIELDS: usize = Self::OWN_INITIAL_FIELDS + Self::ROUTE_FIELDS;
    const ROUTE_FIELDS: usize = semio_plugin_decl_emission::original_view::routing::OriginalHostEventRoutes::INITIAL_FIELDS;

    /// 📐️ Quotes one independently funded host-event or route field.
    pub fn initial_field_demands(field: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if field < Self::ROUTE_FIELDS {
            return semio_plugin_decl_emission::original_view::routing::OriginalHostEventRoutes::initial_field_demands(field);
        }
        Self::own_field_demands(field - Self::ROUTE_FIELDS)
    }

    /// 🌱️ Places one selected field in the final HostEvents allocation.
    /// # Safety
    /// The selected field must be uninitialized within the caller's exclusive, aligned original allocation.
    pub unsafe fn initialize_field_at(target: *mut Self, field: usize, offset: usize, count: usize) -> Result<bool, semio_framework_value::ValueError> {
        if field < Self::ROUTE_FIELDS {
            return unsafe { semio_plugin_decl_emission::original_view::routing::OriginalHostEventRoutes::initialize_field_at(std::ptr::addr_of_mut!((*target).routes), field, offset, count) };
        }
        unsafe { Self::initialize_own_field_at(target, field - Self::ROUTE_FIELDS, offset, count) }
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
