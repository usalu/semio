//! 🗄️ `db_core` — foundation of the `db` crate family (event-sourced document database server
//! engine): local identity newtypes, the single `DbError` every `db_*` crate returns, corruption/
//! resource limits, `DurabilityClass`, sync-relevant frontier types (`Frontier`/`FrontierDelta`/
//! `ResumeToken`), the `EpochFence` split-brain gate, mailbox `Priority`, config/profiles,
//! `DbCapabilities`, the `VersionGraph` vcs seam, and the `Emit` observability seam. Frozen
//! contract: `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: this crate depends on `pack_core` only (per the contract's per-crate deps
//! table) — no `protocol` dependency. The frozen `db` facade's stable API types `Frontier`/
//! `CommandReceipt` are expressed against `protocol::DocumentId`/`protocol::OperationId`; this
//! crate defines its own protocol-decoupled `DocumentId`/`ActorId` newtypes with the same
//! single-`String` shape so a higher crate (which does depend on `protocol`) can convert 1:1
//! (`protocol::DocumentId(id.0)` / `db_core::DocumentId(pid.0)`) without any lossy translation.
//! This keeps `db_core` (along with `db_state`, `db_actor`'s mailbox core, and `db_conflict`)
//! `wasm32-unknown-unknown`-clean, per the contract's "no cdylib" / wasm-clean note.

//#region 🔖️Ids
/// @emoji 🪪️ A document's identity, decoupled from `protocol::DocumentId` (see module doc) but
/// sharing its single-`String` shape so conversions at the `db`/`protocol` boundary are lossless.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DocumentId(pub String);

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for DocumentId {
    fn from(value: &str) -> Self {
        DocumentId(value.to_string())
    }
}

impl From<String> for DocumentId {
    fn from(value: String) -> Self {
        DocumentId(value)
    }
}

/// @emoji 👤️ An actor's (author's) identity, decoupled from `protocol::ActorId` — see
/// `DocumentId`'s doc for the shared-shape conversion rationale.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ActorId(pub String);

impl std::fmt::Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ActorId {
    fn from(value: &str) -> Self {
        ActorId(value.to_string())
    }
}

impl From<String> for ActorId {
    fn from(value: String) -> Self {
        ActorId(value)
    }
}

/// @emoji 🔁️ A document actor's supervision generation (bumped on every restart by `db_actor`'s
/// `OneForOne`/`OneForAll`/`Escalate` supervision). `DocumentHandle` (the `db` facade's stable
/// API) carries one alongside its mailbox sender so a handle obtained before a restart fails
/// loudly (`DbError::StaleGeneration`) instead of silently talking to a dead mailbox.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct GenerationId(pub u64);

impl GenerationId {
    /// @emoji 🌱️ The generation of a freshly spawned actor that has never restarted.
    pub const INITIAL: GenerationId = GenerationId(0);

    /// @emoji ⏭️ The next generation after a supervised restart.
    pub fn next(self) -> GenerationId {
        GenerationId(self.0 + 1)
    }
}
//#endregion 🔖️Ids

//#region 🔖️Errors
/// @emoji 🚨️ The one error type every `db_*` public fn returns; never leaks `std::io::Error` (or
/// any other foreign error type) — every crate below `db_document` wraps its own errors into this.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum DbError {
    #[error("io error: {0}")]
    Io(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("already exists: {0}")]
    AlreadyExists(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("fenced: expected epoch {expected}, got {actual}")]
    Fenced { expected: u64, actual: u64 },
    #[error("stale generation: expected {expected:?}, got {actual:?}")]
    StaleGeneration { expected: GenerationId, actual: GenerationId },
    #[error("unavailable: {0}")]
    Unavailable(String),
    #[error("timeout: {0}")]
    Timeout(String),
    #[error("corrupt: {0}")]
    Corrupt(String),
    #[error("closed")]
    Closed,
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("not implemented: {0}")]
    Unimplemented(&'static str),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<pack_core::PackError> for DbError {
    /// @emoji 🔀️ `db_wal`/`db_snapshot` sit directly on top of `pack`/`protocol`'s `.spr`/`.spk`
    /// containers; this lets them use `?` without hand-rolling the same mapping repeatedly.
    /// Corruption-flavored `PackError` variants map to `DbError::Corrupt`, resource-flavored ones
    /// to `DbError::LimitExceeded`/`Io`, and schema mismatches to `DbError::InvalidArgument`.
    fn from(err: pack_core::PackError) -> Self {
        match err {
            pack_core::PackError::Io(message) => DbError::Io(message),
            pack_core::PackError::LimitExceeded(what) => DbError::LimitExceeded(what),
            pack_core::PackError::Schema(message) => DbError::InvalidArgument(message),
            other => DbError::Corrupt(other.to_string()),
        }
    }
}
//#endregion 🔖️Errors

//#region 🔖️Limits
/// @emoji 🛡️ Corruption/resource-hardening ceilings the `db` family validates against before
/// allocating (mirrors `pack_core::PackLimits`'s stated invariant) — every decoder/mailbox/query
/// path in the family checks a length against these before growing a buffer.
#[derive(Clone, Debug)]
pub struct DbLimits {
    pub max_command_bytes: u64,
    pub max_batch_commands: u32,
    pub max_payload_bytes: u64,
    pub max_query_bytes: u64,
    pub max_mailbox_depth: u32,
    pub max_open_documents: u32,
    pub max_preview_ttl_ms: u64,
}

impl Default for DbLimits {
    fn default() -> Self {
        Self {
            max_command_bytes: 8 * 1024 * 1024,
            max_batch_commands: 4_096,
            max_payload_bytes: 256 * 1024 * 1024,
            max_query_bytes: 4 * 1024 * 1024,
            max_mailbox_depth: 65_536,
            max_open_documents: 100_000,
            max_preview_ttl_ms: 5 * 60 * 1_000,
        }
    }
}

/// @emoji 📏️ Validates `len` against `max` BEFORE the caller allocates anything sized by it —
/// shared by every length check across the `db` family so the "validate before allocating"
/// invariant has exactly one implementation to audit.
pub fn check_len(len: u64, max: u64, what: &'static str) -> Result<(), DbError> {
    if len > max {
        return Err(DbError::LimitExceeded(what));
    }
    Ok(())
}
//#endregion 🔖️Limits

//#region 🔖️Durability
/// @emoji 💾️ How durably a command's effects are guaranteed to survive a crash before its
/// `CommandReceipt` is returned. Ordered strongest-last: `Memory < Os < Fsync < Quorum(n)`
/// (`Quorum` variants order among themselves by acknowledging-replica count `n`) — group-commit
/// batching in `db_wal` computes `max()` over the durability classes requested by the commands in
/// one batch to decide how hard to push the flush.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DurabilityClass {
    /// @emoji 🧠️ Visible to readers once applied in-process; no persistence guarantee at all.
    #[default]
    Memory,
    /// @emoji 🗂️ Written to the WAL and handed to the OS (`write(2)`), not yet `fsync`ed.
    Os,
    /// @emoji 🔒️ `fsync`ed to local storage before the receipt is returned.
    Fsync,
    /// @emoji 🤝️ Acknowledged `fsync`ed by at least `n` cluster replicas (`db_cluster`).
    Quorum(u8),
}

impl DurabilityClass {
    /// @emoji 🥇️ A total order key: `(tier, quorum_n)`, so `Ord`/`PartialOrd` can be derived from
    /// arithmetic comparison rather than a hand-written match ladder.
    fn rank(&self) -> (u8, u8) {
        match self {
            DurabilityClass::Memory => (0, 0),
            DurabilityClass::Os => (1, 0),
            DurabilityClass::Fsync => (2, 0),
            DurabilityClass::Quorum(n) => (3, *n),
        }
    }
}

impl PartialOrd for DurabilityClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DurabilityClass {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}
//#endregion 🔖️Durability

//#region 🔖️Frontier
/// @emoji 🧭️ A document's sync-relevant position: how far its WAL/commit sequence has advanced,
/// its commit chain's current tip hash, and the fencing epoch it was produced under. Mirrors the
/// `db` facade's frozen `Frontier{document, head_seq, commit_seq, chain_hash, epoch}` shape
/// exactly (see module doc for the `DocumentId` conversion rationale).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Frontier {
    pub document: DocumentId,
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub epoch: u64,
}

impl Frontier {
    /// @emoji 🌱️ The frontier of a freshly created, empty document.
    pub fn genesis(document: DocumentId) -> Frontier {
        Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 }
    }

    /// @emoji 🔑️ Reinterprets `chain_hash` as a `pack_core::ContentHash` — the family hashes
    /// pack-style throughout; this is the bridge for callers that want the typed/`Display`able
    /// form instead of a raw array.
    pub fn chain_hash_typed(&self) -> pack_core::ContentHash {
        pack_core::ContentHash(self.chain_hash)
    }

    /// @emoji 🏔️ True iff `self` has observed everything `other` has (same document, `>=` on
    /// every sequence/epoch field) — the law `Consistency::AtLeast(frontier)` query resolution
    /// checks against a document's current frontier.
    pub fn dominates(&self, other: &Frontier) -> Result<bool, DbError> {
        if self.document != other.document {
            return Err(DbError::InvalidArgument(format!(
                "frontier document mismatch: {} vs {}",
                self.document, other.document
            )));
        }
        Ok(self.head_seq >= other.head_seq && self.commit_seq >= other.commit_seq && self.epoch >= other.epoch)
    }
}

/// @emoji 📐️ The gap between two frontiers of the SAME document — `db_sync`'s unit of "how much
/// missing-command transfer does a replica need".
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FrontierDelta {
    pub document: DocumentId,
    pub from_head_seq: u64,
    pub to_head_seq: u64,
    pub commands: u64,
}

impl FrontierDelta {
    /// @emoji ➖️ Computes the delta from `from` to `to`. Errors on a document mismatch or on `to`
    /// being behind `from` (a delta only ever moves a replica forward).
    pub fn between(from: &Frontier, to: &Frontier) -> Result<FrontierDelta, DbError> {
        if from.document != to.document {
            return Err(DbError::InvalidArgument(format!(
                "frontier document mismatch: {} vs {}",
                from.document, to.document
            )));
        }
        if to.head_seq < from.head_seq {
            return Err(DbError::InvalidArgument(format!(
                "to frontier (head_seq {}) is behind from frontier (head_seq {})",
                to.head_seq, from.head_seq
            )));
        }
        Ok(FrontierDelta {
            document: from.document.clone(),
            from_head_seq: from.head_seq,
            to_head_seq: to.head_seq,
            commands: to.head_seq - from.head_seq,
        })
    }

    /// @emoji 🕳️ True iff the two frontiers were already equal (nothing to transfer).
    pub fn is_empty(&self) -> bool {
        self.commands == 0
    }
}

/// @emoji 🎫️ An opaque, serialized `Frontier` a replica hands back on reconnect so `db_sync` can
/// resume exactly where it left off, instead of re-negotiating from scratch. Deliberately
/// text-encoded (not a bincode/serde blob) so it stays diffable in logs and stable across a
/// `Frontier` field-order change — the wire format is this crate's own choice (the contract
/// leaves the exact encoding unspecified), versioned via a leading `v1` tag.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResumeToken(String);

impl ResumeToken {
    /// @emoji ✍️ Encodes `frontier` as `v1|<document>|<head_seq>|<commit_seq>|<epoch>|<hex chain_hash>`.
    /// Rejects a document id containing `|` (would make the encoding ambiguous to decode).
    pub fn encode(frontier: &Frontier) -> Result<ResumeToken, DbError> {
        if frontier.document.0.contains('|') {
            return Err(DbError::InvalidArgument(
                "document id must not contain '|' to be resume-token safe".to_string(),
            ));
        }
        let mut hex = String::with_capacity(64);
        for byte in frontier.chain_hash {
            use std::fmt::Write;
            let _ = write!(hex, "{byte:02x}");
        }
        Ok(ResumeToken(format!(
            "v1|{}|{}|{}|{}|{}",
            frontier.document, frontier.head_seq, frontier.commit_seq, frontier.epoch, hex
        )))
    }

    /// @emoji 📖️ Inverse of `encode`. Rejects an unknown version tag, a wrong field count, or a
    /// malformed hex/decimal field, always returning `DbError` rather than panicking.
    pub fn decode(&self) -> Result<Frontier, DbError> {
        let mut parts = self.0.split('|');
        let malformed = || DbError::Corrupt("malformed resume token".to_string());

        let version = parts.next().ok_or_else(malformed)?;
        if version != "v1" {
            return Err(DbError::Corrupt(format!("unsupported resume token version {version:?}")));
        }
        let document = parts.next().ok_or_else(malformed)?.to_string();
        let head_seq = parts.next().ok_or_else(malformed)?.parse::<u64>().map_err(|_| malformed())?;
        let commit_seq = parts.next().ok_or_else(malformed)?.parse::<u64>().map_err(|_| malformed())?;
        let epoch = parts.next().ok_or_else(malformed)?.parse::<u64>().map_err(|_| malformed())?;
        let hex = parts.next().ok_or_else(malformed)?;
        if parts.next().is_some() {
            return Err(malformed());
        }
        if hex.len() != 64 {
            return Err(malformed());
        }
        let mut chain_hash = [0u8; 32];
        for (i, slot) in chain_hash.iter_mut().enumerate() {
            *slot = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).map_err(|_| malformed())?;
        }
        Ok(Frontier { document: DocumentId(document), head_seq, commit_seq, chain_hash, epoch })
    }

    /// @emoji 🔍️ Borrows the token's wire form, e.g. for embedding in a `protocol_wire::Hello`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
//#endregion 🔖️Frontier

//#region 🔖️Fencing
/// @emoji 🚧️ The split-brain gate: a monotonic epoch a `CatalogStorage::cas_root` write must
/// present to succeed. A writer that lost leadership (its epoch superseded by a newer one) gets
/// `DbError::Fenced` on its next write instead of silently corrupting the catalog root — the
/// primitive `db_cluster`'s ownership-lease failover builds on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EpochFence {
    pub epoch: u64,
}

impl EpochFence {
    /// @emoji 🌱️ The fence a document's catalog entry starts at before any leadership handoff.
    pub const INITIAL: EpochFence = EpochFence { epoch: 0 };

    /// @emoji ⏭️ The fence a new leader claims after winning an ownership lease.
    pub fn next(self) -> EpochFence {
        EpochFence { epoch: self.epoch + 1 }
    }

    /// @emoji ✅️ Compare-and-swap gate: succeeds only if `self` (the epoch presented by the
    /// writer) exactly matches `current` (the epoch stamped on the stored root). Any mismatch —
    /// stale writer OR a writer somehow ahead of the stored root — is fenced, since the latter
    /// indicates the caller read a root written concurrently under a different epoch.
    pub fn check(self, current: EpochFence) -> Result<(), DbError> {
        if self.epoch == current.epoch {
            Ok(())
        } else {
            Err(DbError::Fenced { expected: current.epoch, actual: self.epoch })
        }
    }
}
//#endregion 🔖️Fencing

//#region 🔖️Priority
/// @emoji 🚦️ The six bounded mailbox lanes every document actor's inbox is split into
/// (`db_actor`'s deficit-round-robin scheduler drains them by weight; admission sheds the lowest
/// first under backpressure). Declaration order is priority order, highest first.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Priority {
    /// @emoji 🛑️ Supervision/control messages (shutdown, generation bump) — never shed, never queued behind anything.
    System,
    /// @emoji 🩹️ WAL replay / crash-recovery traffic, run before the actor accepts ordinary work.
    Recovery,
    /// @emoji ✍️ Ordinary command submissions (the actor's core job).
    Command,
    /// @emoji 🔎️ One-shot queries against canonical/historical state.
    Query,
    /// @emoji 📡️ Live-query change notifications to subscribers.
    Live,
    /// @emoji 🌫️ Ephemeral preview publishes — lowest priority, the only lane ever shed under
    /// backpressure (previews are never durable and never allowed to delay a command, per the
    /// contract's preview law).
    Preview,
}

impl Priority {
    /// @emoji 📋️ Every lane, in priority order — the shape `db_actor`'s mailbox array indexes by.
    pub const ALL: [Priority; 6] =
        [Priority::System, Priority::Recovery, Priority::Command, Priority::Query, Priority::Live, Priority::Preview];

    /// @emoji 🔢️ A dense `0..6` index matching declaration order, for array-indexed mailbox storage.
    pub fn rank(self) -> usize {
        match self {
            Priority::System => 0,
            Priority::Recovery => 1,
            Priority::Command => 2,
            Priority::Query => 3,
            Priority::Live => 4,
            Priority::Preview => 5,
        }
    }

    /// @emoji ✂️ True only for `Preview` — the contract's "shed-previews-first admission" law: a
    /// full mailbox drops the oldest preview rather than ever rejecting/blocking a higher lane.
    pub fn sheddable(self) -> bool {
        matches!(self, Priority::Preview)
    }

    /// @emoji ⚖️ Default deficit-round-robin weight per lane (this crate's own choice — the
    /// contract fixes the lane set and shedding law, not the exact weights). Halves lane-to-lane
    /// so a starved low lane still makes bounded progress without letting `Preview` traffic
    /// compete meaningfully with `Command`.
    pub fn default_weight(self) -> u32 {
        match self {
            Priority::System => 64,
            Priority::Recovery => 32,
            Priority::Command => 16,
            Priority::Query => 8,
            Priority::Live => 4,
            Priority::Preview => 1,
        }
    }
}
//#endregion 🔖️Priority

//#region 🔖️Capabilities
/// @emoji 🧰️ What a particular `Database` instance supports — negotiated at `open` time from the
/// storage backend's own `StorageCapabilities` (`db_storage`) plus enabled Cargo features, and
/// surfaced to clients (e.g. so `framework/sync` knows whether to offer preview publishing).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DbCapabilities {
    pub preview: bool,
    pub historical_query: bool,
    pub live_query: bool,
    pub cluster: bool,
    pub max_durability: DurabilityClass,
}
//#endregion 🔖️Capabilities

//#region 🔖️Config
/// @emoji 🎛️ Which of the family's built-in default profiles a `Database::open` call selects —
/// `db_config`-equivalent defaults live entirely in this crate (see `DbConfig::for_profile`) so
/// every crate constructing a config in a test gets the same baseline.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Profile {
    /// @emoji 🧪️ Deterministic, low-latency defaults for unit/integration tests: `Memory`
    /// durability (no real fsync cost), tight limits (catches runaway fixtures fast).
    Test,
    /// @emoji 🛠️ A local developer loop: `Os` durability (survives a process crash, not a power
    /// loss), generous limits.
    Dev,
    /// @emoji 🏭️ Production defaults: `Fsync` durability, the family's full resource ceilings.
    Prod,
}

/// @emoji 🚦️ Per-`Priority`-lane mailbox bounds, indexed by `Priority::rank`.
#[derive(Clone, Copy, Debug)]
pub struct MailboxCapacities([u32; 6]);

impl MailboxCapacities {
    /// @emoji 🟰️ The same bound for every lane.
    pub fn uniform(capacity: u32) -> Self {
        Self([capacity; 6])
    }

    /// @emoji 📖️ The bound for `priority`'s lane.
    pub fn get(&self, priority: Priority) -> u32 {
        self.0[priority.rank()]
    }

    /// @emoji ✏️ Overrides the bound for `priority`'s lane.
    pub fn set(&mut self, priority: Priority, capacity: u32) {
        self.0[priority.rank()] = capacity;
    }
}

impl Default for MailboxCapacities {
    fn default() -> Self {
        Self::uniform(1_024)
    }
}

/// @emoji ⚙️ Everything a `Database::open` needs beyond the storage backend itself: limits,
/// default durability, capability negotiation inputs, and mailbox sizing.
#[derive(Clone, Debug)]
pub struct DbConfig {
    pub profile: Profile,
    pub limits: DbLimits,
    pub default_durability: DurabilityClass,
    pub capabilities: DbCapabilities,
    pub mailbox_capacities: MailboxCapacities,
}

impl DbConfig {
    /// @emoji 🏗️ Builds the family's well-justified defaults for `profile` (see `Profile`'s doc
    /// for the reasoning behind each choice) — the starting point every `Database::open_at`
    /// (zero-touch) call and every crate's tests should build from rather than hand-rolling limits.
    pub fn for_profile(profile: Profile) -> DbConfig {
        let (default_durability, limits, mailbox_capacity) = match profile {
            Profile::Test => (DurabilityClass::Memory, DbLimits { max_command_bytes: 64 * 1024, max_batch_commands: 64, ..DbLimits::default() }, 64),
            Profile::Dev => (DurabilityClass::Os, DbLimits::default(), 1_024),
            Profile::Prod => (DurabilityClass::Fsync, DbLimits::default(), 65_536),
        };
        DbConfig {
            profile,
            limits,
            default_durability,
            capabilities: DbCapabilities {
                preview: true,
                historical_query: true,
                live_query: true,
                cluster: matches!(profile, Profile::Prod),
                max_durability: default_durability,
            },
            mailbox_capacities: MailboxCapacities::uniform(mailbox_capacity),
        }
    }
}
//#endregion 🔖️Config

//#region 🔖️VersionGraph
/// @emoji 📝️ One committed, content-addressed change to record in the version graph — the
/// `VersionGraph::record_change` argument shape, deliberately vcs-type-free (see trait doc).
#[derive(Clone, Debug)]
pub struct ChangeRecord {
    pub parent: Option<String>,
    pub content_hash: pack_core::ContentHash,
    pub author: ActorId,
    pub message: String,
    pub timestamp_ms: u64,
}

/// @emoji 🏁️ A checkpoint (a named, authored group of changes) to record — the
/// `VersionGraph::checkpoint` argument shape.
#[derive(Clone, Debug)]
pub struct CheckpointRequest {
    pub parent_checkpoint: Option<String>,
    pub change_ids: Vec<String>,
    pub message: String,
    pub authors: Vec<ActorId>,
    pub timestamp_ms: u64,
}

/// @emoji 🌿️ The vcs seam: per the contract's hard dependency rule, only `db_engine` (behind the
/// `vcs` Cargo feature) may depend on the `vcs` crate — every crate below it, including
/// `db_document` (which drives commits), talks to version history ONLY through this
/// `vcs`-type-free trait. `db_engine` supplies the real implementation over `vcs::DocumentVcs*`;
/// anything vcs-agnostic (e.g. a deployment with the `vcs` feature disabled) can supply
/// `NullVersionGraph` instead.
pub trait VersionGraph: Send + Sync {
    /// @emoji 📝️ Records `change` against `document`, returning its assigned change id.
    fn record_change(&self, document: &DocumentId, change: ChangeRecord) -> Result<String, DbError>;

    /// @emoji 🏁️ Records a checkpoint over previously-recorded changes, returning its assigned
    /// content-addressed checkpoint id (`vcs`'s own concern how that id is derived).
    fn checkpoint(&self, document: &DocumentId, request: CheckpointRequest) -> Result<String, DbError>;

    /// @emoji 🔀️ The nearest common ancestor checkpoint of `a` and `b`, or `None` if they share
    /// none (disjoint histories).
    fn merge_base(&self, document: &DocumentId, a: &str, b: &str) -> Result<Option<String>, DbError>;

    /// @emoji 🎯️ The current head checkpoint id of `alternative`, or `None` if it has none yet.
    fn head(&self, document: &DocumentId, alternative: &str) -> Result<Option<String>, DbError>;
}

/// @emoji 🚫️ A `VersionGraph` that answers every call with `DbError::Unimplemented` rather than
/// panicking — the extension seam this crate offers for a `vcs`-feature-disabled deployment (or a
/// unit test that doesn't need real version history). Genuinely a placeholder, not a fake: it
/// never silently drops a change, it always tells the caller version history isn't wired up.
#[derive(Clone, Copy, Default, Debug)]
pub struct NullVersionGraph;

impl VersionGraph for NullVersionGraph {
    fn record_change(&self, _document: &DocumentId, _change: ChangeRecord) -> Result<String, DbError> {
        Err(DbError::Unimplemented("VersionGraph is not wired up (vcs feature disabled)"))
    }

    fn checkpoint(&self, _document: &DocumentId, _request: CheckpointRequest) -> Result<String, DbError> {
        Err(DbError::Unimplemented("VersionGraph is not wired up (vcs feature disabled)"))
    }

    fn merge_base(&self, _document: &DocumentId, _a: &str, _b: &str) -> Result<Option<String>, DbError> {
        Err(DbError::Unimplemented("VersionGraph is not wired up (vcs feature disabled)"))
    }

    fn head(&self, _document: &DocumentId, _alternative: &str) -> Result<Option<String>, DbError> {
        Err(DbError::Unimplemented("VersionGraph is not wired up (vcs feature disabled)"))
    }
}
//#endregion 🔖️VersionGraph

//#region 🔖️Emit
/// @emoji 🏷️ One field attached to an `EmitEvent`, kept as a small closed set of primitive
/// shapes (no dynamic `Any`) so a sink can serialize/aggregate without reflection.
#[derive(Clone, Debug)]
pub enum EmitField {
    U64(u64),
    I64(i64),
    F64(f64),
    Bool(bool),
    Text(String),
}

/// @emoji 📣️ One observability event: a stable name plus an optional document scope and a small
/// bag of typed fields. `Emit::emit` takes this by value (not by reference) since a mailbox-
/// adjacent hot path may hand it across a thread boundary to a sink.
#[derive(Clone, Debug)]
pub struct EmitEvent {
    pub name: &'static str,
    pub document: Option<DocumentId>,
    pub fields: Vec<(&'static str, EmitField)>,
}

impl EmitEvent {
    /// @emoji 🆕️ A bare event with `name` and no document/fields yet.
    pub fn new(name: &'static str) -> Self {
        Self { name, document: None, fields: Vec::new() }
    }

    /// @emoji 🪪️ Scopes the event to `document` (builder-style).
    pub fn with_document(mut self, document: DocumentId) -> Self {
        self.document = Some(document);
        self
    }

    /// @emoji ➕️ Appends one field (builder-style).
    pub fn field(mut self, key: &'static str, value: EmitField) -> Self {
        self.fields.push((key, value));
        self
    }
}

/// @emoji 📡️ The observability seam: every `db_*` crate that wants to emit a metric/span/log
/// event takes `&dyn Emit` (or `Arc<dyn Emit>`) rather than depending on `db_observe` directly —
/// inverts the dependency so `db_core..db_cluster` stay `db_observe`-free while `db_observe`'s
/// real sinks (structured/audit JSON-lines, metric registries) implement this trait.
pub trait Emit: Send + Sync {
    fn emit(&self, event: EmitEvent);
}

/// @emoji 🔇️ An `Emit` that discards every event — the default when no observability sink is
/// configured, and a convenient no-op for tests that don't care about emitted events.
#[derive(Clone, Copy, Default, Debug)]
pub struct NullEmit;

impl Emit for NullEmit {
    fn emit(&self, _event: EmitEvent) {}
}
//#endregion 🔖️Emit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Ids
    #[test]
    fn document_id_and_actor_id_convert_and_display() {
        let document: DocumentId = "doc-1".into();
        assert_eq!(document.to_string(), "doc-1");
        assert_eq!(document, DocumentId::from("doc-1".to_string()));

        let actor: ActorId = "actor-1".into();
        assert_eq!(actor.to_string(), "actor-1");
    }

    #[test]
    fn generation_id_next_is_strictly_monotonic() {
        let g0 = GenerationId::INITIAL;
        let g1 = g0.next();
        let g2 = g1.next();
        assert!(g0 < g1);
        assert!(g1 < g2);
        assert_eq!(g0, GenerationId(0));
        assert_eq!(g2, GenerationId(2));
    }
    //#endregion 🔖️Ids

    //#region 🔖️Errors
    #[test]
    fn pack_error_conversion_never_panics_and_maps_by_category() {
        let corrupt: DbError = pack_core::PackError::BadMagic.into();
        assert!(matches!(corrupt, DbError::Corrupt(_)));

        let limit: DbError = pack_core::PackError::LimitExceeded("too big").into();
        assert_eq!(limit, DbError::LimitExceeded("too big"));

        let io: DbError = pack_core::PackError::Io("disk full".to_string()).into();
        assert_eq!(io, DbError::Io("disk full".to_string()));

        let schema: DbError = pack_core::PackError::Schema("bad field".to_string()).into();
        assert_eq!(schema, DbError::InvalidArgument("bad field".to_string()));
    }
    //#endregion 🔖️Errors

    //#region 🔖️Limits
    #[test]
    fn check_len_rejects_over_limit_before_any_allocation_would_happen() {
        assert!(check_len(10, 100, "test").is_ok());
        assert_eq!(check_len(101, 100, "test"), Err(DbError::LimitExceeded("test")));
    }
    //#endregion 🔖️Limits

    //#region 🔖️Durability
    #[test]
    fn durability_class_orders_memory_below_os_below_fsync_below_quorum() {
        assert!(DurabilityClass::Memory < DurabilityClass::Os);
        assert!(DurabilityClass::Os < DurabilityClass::Fsync);
        assert!(DurabilityClass::Fsync < DurabilityClass::Quorum(1));
        assert!(DurabilityClass::Quorum(1) < DurabilityClass::Quorum(3));
        assert_eq!(DurabilityClass::default(), DurabilityClass::Memory);

        let mut classes = vec![DurabilityClass::Quorum(2), DurabilityClass::Memory, DurabilityClass::Fsync, DurabilityClass::Os];
        classes.sort();
        assert_eq!(classes, vec![DurabilityClass::Memory, DurabilityClass::Os, DurabilityClass::Fsync, DurabilityClass::Quorum(2)]);
    }

    #[test]
    fn durability_class_batch_max_picks_strongest_requested() {
        let requested = [DurabilityClass::Os, DurabilityClass::Memory, DurabilityClass::Fsync];
        let strongest = requested.into_iter().max().unwrap();
        assert_eq!(strongest, DurabilityClass::Fsync);
    }
    //#endregion 🔖️Durability

    //#region 🔖️Frontier
    fn sample_frontier(document: &str, head_seq: u64, commit_seq: u64, epoch: u64) -> Frontier {
        let mut chain_hash = [0u8; 32];
        chain_hash[0] = head_seq as u8;
        Frontier { document: document.into(), head_seq, commit_seq, chain_hash, epoch }
    }

    #[test]
    fn frontier_genesis_is_all_zero() {
        let frontier = Frontier::genesis("doc-1".into());
        assert_eq!(frontier.head_seq, 0);
        assert_eq!(frontier.commit_seq, 0);
        assert_eq!(frontier.epoch, 0);
        assert_eq!(frontier.chain_hash, [0u8; 32]);
    }

    #[test]
    fn frontier_chain_hash_typed_bridges_to_pack_core_content_hash() {
        let frontier = sample_frontier("doc-1", 5, 5, 0);
        let typed = frontier.chain_hash_typed();
        assert_eq!(typed.0, frontier.chain_hash);
    }

    #[test]
    fn frontier_dominates_requires_same_document_and_all_fields_at_least() {
        let earlier = sample_frontier("doc-1", 3, 3, 0);
        let later = sample_frontier("doc-1", 5, 5, 0);
        assert!(later.dominates(&earlier).unwrap());
        assert!(!earlier.dominates(&later).unwrap());
        assert!(later.dominates(&later).unwrap());

        let other_document = sample_frontier("doc-2", 5, 5, 0);
        assert!(matches!(later.dominates(&other_document), Err(DbError::InvalidArgument(_))));
    }

    #[test]
    fn frontier_delta_between_computes_gap_and_rejects_backwards_or_mismatched() {
        let from = sample_frontier("doc-1", 3, 3, 0);
        let to = sample_frontier("doc-1", 8, 8, 0);
        let delta = FrontierDelta::between(&from, &to).unwrap();
        assert_eq!(delta.from_head_seq, 3);
        assert_eq!(delta.to_head_seq, 8);
        assert_eq!(delta.commands, 5);
        assert!(!delta.is_empty());

        let same = FrontierDelta::between(&from, &from).unwrap();
        assert!(same.is_empty());

        assert!(FrontierDelta::between(&to, &from).is_err());

        let other_document = sample_frontier("doc-2", 8, 8, 0);
        assert!(FrontierDelta::between(&from, &other_document).is_err());
    }

    #[test]
    fn resume_token_round_trips_through_encode_decode() {
        let frontier = sample_frontier("doc-1", 42, 41, 7);
        let token = ResumeToken::encode(&frontier).unwrap();
        let decoded = token.decode().unwrap();
        assert_eq!(decoded, frontier);
        assert!(token.as_str().starts_with("v1|doc-1|42|41|7|"));
    }

    #[test]
    fn resume_token_encode_rejects_pipe_in_document_id() {
        let frontier = sample_frontier("doc|1", 1, 1, 0);
        assert!(ResumeToken::encode(&frontier).is_err());
    }

    #[test]
    fn resume_token_decode_rejects_malformed_input_without_panicking() {
        assert!(matches!(ResumeToken("garbage".to_string()).decode(), Err(DbError::Corrupt(_))));
        assert!(matches!(ResumeToken("v2|doc|1|1|1|00".to_string()).decode(), Err(DbError::Corrupt(_))));
        assert!(matches!(ResumeToken("v1|doc|notanumber|1|1|00".to_string()).decode(), Err(DbError::Corrupt(_))));
        let short_hash = format!("v1|doc-1|1|1|1|{}", "ab".repeat(10));
        assert!(matches!(ResumeToken(short_hash).decode(), Err(DbError::Corrupt(_))));
    }
    //#endregion 🔖️Frontier

    //#region 🔖️Fencing
    #[test]
    fn epoch_fence_check_accepts_matching_epoch_and_rejects_stale_or_ahead() {
        let current = EpochFence::INITIAL.next().next();
        assert!(current.check(current).is_ok());

        let stale = EpochFence::INITIAL;
        assert_eq!(stale.check(current), Err(DbError::Fenced { expected: current.epoch, actual: stale.epoch }));

        let ahead = current.next();
        assert_eq!(ahead.check(current), Err(DbError::Fenced { expected: current.epoch, actual: ahead.epoch }));
    }

    #[test]
    fn epoch_fence_next_is_monotonic() {
        let mut fence = EpochFence::INITIAL;
        for expected in 1..=5u64 {
            fence = fence.next();
            assert_eq!(fence.epoch, expected);
        }
    }
    //#endregion 🔖️Fencing

    //#region 🔖️Priority
    #[test]
    fn priority_rank_matches_declaration_order() {
        for (index, priority) in Priority::ALL.iter().enumerate() {
            assert_eq!(priority.rank(), index);
        }
        assert!(Priority::System < Priority::Command);
        assert!(Priority::Command < Priority::Preview);
    }

    #[test]
    fn only_preview_is_sheddable() {
        for priority in Priority::ALL {
            assert_eq!(priority.sheddable(), priority == Priority::Preview);
        }
    }

    #[test]
    fn default_weights_are_strictly_decreasing_by_priority_order() {
        let weights: Vec<u32> = Priority::ALL.iter().map(|priority| priority.default_weight()).collect();
        for window in weights.windows(2) {
            assert!(window[0] > window[1], "weights must strictly decrease: {weights:?}");
        }
    }
    //#endregion 🔖️Priority

    //#region 🔖️Config
    #[test]
    fn mailbox_capacities_get_set_round_trip_per_lane() {
        let mut capacities = MailboxCapacities::uniform(10);
        assert_eq!(capacities.get(Priority::Command), 10);
        capacities.set(Priority::Preview, 2);
        assert_eq!(capacities.get(Priority::Preview), 2);
        assert_eq!(capacities.get(Priority::System), 10);
    }

    #[test]
    fn profile_defaults_order_durability_test_below_dev_below_prod() {
        let test_config = DbConfig::for_profile(Profile::Test);
        let dev_config = DbConfig::for_profile(Profile::Dev);
        let prod_config = DbConfig::for_profile(Profile::Prod);
        assert!(test_config.default_durability < dev_config.default_durability);
        assert!(dev_config.default_durability < prod_config.default_durability);
        assert!(!test_config.capabilities.cluster);
        assert!(prod_config.capabilities.cluster);
        assert_eq!(test_config.capabilities.max_durability, test_config.default_durability);
    }

    #[test]
    fn test_profile_has_tighter_limits_than_prod() {
        let test_config = DbConfig::for_profile(Profile::Test);
        let prod_config = DbConfig::for_profile(Profile::Prod);
        assert!(test_config.limits.max_command_bytes < prod_config.limits.max_command_bytes);
        assert!(test_config.limits.max_batch_commands < prod_config.limits.max_batch_commands);
    }
    //#endregion 🔖️Config

    //#region 🔖️VersionGraph
    #[test]
    fn null_version_graph_never_panics_always_reports_unimplemented() {
        let graph = NullVersionGraph;
        let document: DocumentId = "doc-1".into();
        let change = ChangeRecord {
            parent: None,
            content_hash: pack_core::ContentHash([0u8; 32]),
            author: "actor-1".into(),
            message: "msg".to_string(),
            timestamp_ms: 0,
        };
        assert!(matches!(graph.record_change(&document, change), Err(DbError::Unimplemented(_))));

        let checkpoint = CheckpointRequest { parent_checkpoint: None, change_ids: vec![], message: "msg".to_string(), authors: vec![], timestamp_ms: 0 };
        assert!(matches!(graph.checkpoint(&document, checkpoint), Err(DbError::Unimplemented(_))));
        assert!(matches!(graph.merge_base(&document, "a", "b"), Err(DbError::Unimplemented(_))));
        assert!(matches!(graph.head(&document, "main"), Err(DbError::Unimplemented(_))));
    }

    #[test]
    fn version_graph_trait_object_is_dyn_compatible() {
        let graph: Box<dyn VersionGraph> = Box::new(NullVersionGraph);
        let document: DocumentId = "doc-1".into();
        assert!(graph.head(&document, "main").is_err());
    }
    //#endregion 🔖️VersionGraph

    //#region 🔖️Emit
    struct RecordingEmit {
        events: std::sync::Mutex<Vec<EmitEvent>>,
    }

    impl Emit for RecordingEmit {
        fn emit(&self, event: EmitEvent) {
            self.events.lock().unwrap().push(event);
        }
    }

    #[test]
    fn emit_trait_object_records_events_with_fields_and_document() {
        let sink = RecordingEmit { events: std::sync::Mutex::new(Vec::new()) };
        let emit: &dyn Emit = &sink;
        emit.emit(
            EmitEvent::new("command.applied")
                .with_document("doc-1".into())
                .field("bytes", EmitField::U64(128))
                .field("ok", EmitField::Bool(true)),
        );
        let events = sink.events.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "command.applied");
        assert_eq!(events[0].document, Some(DocumentId::from("doc-1")));
        assert_eq!(events[0].fields.len(), 2);
    }

    #[test]
    fn null_emit_discards_without_panicking() {
        let emit = NullEmit;
        emit.emit(EmitEvent::new("noop"));
    }
    //#endregion 🔖️Emit
}
//#endregion 🧪️Tests
