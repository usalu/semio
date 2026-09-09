//! 🗄️ `db_cluster` — sharding, ownership leases + epoch failover, follower WAL/snapshot
//! replication, quorum durability, and read/preview routing for the `db` crate family. Frozen
//! contract: `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`); per-crate detail in the approved plan's Part 2, `db_cluster` row.
//!
//! 🎯️ Design choice (scope): this wave implements the mechanism every other cluster feature is
//! built on top of — a consistent-hash shard map, lease-backed ownership with real epoch failover
//! (built directly on `db_storage::LeaseStorage`/`EpochFence`, which already own the
//! fencing primitive), tail-command follower replication (built on `db_sync`'s existing
//! `ArtifactSyncState`/`BootstrapPlan` machinery), quorum-durability ack tracking, and read/preview
//! routing. `ReplicationOutcome::SnapshotTransferred` is a deliberate extension seam: this crate
//! transports raw snapshot bytes to a follower's `SnapshotStorage` (the "segment/snapshot
//! replication" the contract asks for) but does NOT decode/materialize them into live document
//! state — that is `db_snapshot`/`db_artifact`'s responsibility, and neither exists yet this wave.
//! The follower's WAL-derived frontier is honestly left un-advanced in that case rather than faked.
//!
//! 🎯️ Design choice (no `blake3` dependency): the shard-map ring only needs a well-distributed,
//! deterministic hash for placement — not a content-addressing primitive — so this crate uses a
//! small inline FNV-1a rather than adding a new direct dependency the contract's per-crate dep
//! table doesn't list for `db_cluster` (`db_core, db_actor, db_wal, db_storage, db_sync` only).
//! Every genuinely content-addressed hash in the family (payloads, snapshots, WAL chains) already
//! flows through `db_storage`/`db_wal`/`db_sync`, which this crate reuses via their public APIs
//! rather than re-deriving.
use crate::db_durability::Frontier;
use crate::*;
use db_storage::{SnapshotStorage as _, WalStorage as _};
/// @emoji 🏷️ A cluster node's identity — the consistent-hash ring's key type and the `holder`
/// string `db_storage::LeaseStorage` records ownership grants under.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeId(pub String);

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        NodeId(value.to_string())
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        NodeId(value)
    }
}

/// @emoji 🎯️ Default virtual nodes per physical node (this crate's own choice — the contract fixes
/// "consistent hash", not a vnode count): high enough that ring-position variance stays low
/// (bounded remap on membership change, see the `🧪️Tests` region's minimal-remap laws), low enough
/// that `ShardMap::owner`'s `BTreeMap` lookup stays cheap even with hundreds of physical nodes.
pub const DEFAULT_VIRTUAL_NODES: u32 = 128;

/// @emoji #⃣ A small, dependency-free 64-bit FNV-1a — see the module doc for why this crate
/// doesn't reach for `blake3` here (ring placement needs distribution, not content-addressing).
// 🚫️async: E1 pure accessor consumed synchronously (once inline inside a temp-borrowing expression) — see R9
fn fnv1a_64(bytes: &[u8]) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET_BASIS;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

/// @emoji 💍️ Consistent-hash ring mapping documents to cluster nodes: shard placement, computed
/// identically by every node from the identical membership list — no coordination needed beyond
/// agreeing on membership. Ring positions are `fnv1a_64("<node>#<vnode-index>")`; document lookups
/// hash the document id the same way and walk clockwise to the first ring entry at or past it.
#[derive(Clone, Default, Debug)]
pub struct ShardMap {
    virtual_nodes: u32,
    ring: std::collections::BTreeMap<u64, NodeId>,
}

impl ShardMap {
    /// @emoji 🆕️ An empty ring with `virtual_nodes` (clamped to at least 1) vnodes per node added.
    pub async fn new(virtual_nodes: u32) -> Self {
        Self { virtual_nodes: virtual_nodes.max(1), ring: std::collections::BTreeMap::new() }
    }

    /// @emoji ➕️ Adds `node`'s vnodes to the ring. Idempotent: re-adding an already-present node
    /// recomputes (but does not duplicate) its ring positions.
    pub async fn add_node(&mut self, node: &NodeId) {
        for vnode in 0..self.virtual_nodes {
            let key = fnv1a_64(format!("{}#{vnode}", node.0).as_bytes());
            self.ring.insert(key, node.clone());
        }
    }

    /// @emoji ➖️ Removes every one of `node`'s vnodes from the ring. Idempotent if absent. Per
    /// consistent hashing's defining law, this only changes the owner of documents that were
    /// previously owned by `node` — every other document's owner is unaffected (see `🧪️Tests`).
    pub async fn remove_node(&mut self, node: &NodeId) {
        self.ring.retain(|_, owner| owner != node);
    }

    /// @emoji 📋️ Every distinct physical node currently on the ring.
    pub async fn nodes(&self) -> std::collections::BTreeSet<NodeId> {
        self.ring.values().cloned().collect()
    }

    /// @emoji 🕳️ True iff the ring has no nodes at all.
    pub async fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }

    /// @emoji 🎯️ The node owning `document`: the first ring position at or after `document`'s hash,
    /// wrapping to the ring's lowest position past the maximum key. `None` iff the ring is empty.
    pub async fn owner(&self, document: &ArtifactId) -> Option<NodeId> {
        if self.ring.is_empty() {
            return None;
        }
        let key = fnv1a_64(document.0.as_bytes());
        self.ring.range(key..).next().or_else(|| self.ring.iter().next()).map(|(_, node)| node.clone())
    }
}
//#endregion 🔖️ShardMap

//#region 🔖️Ownership
/// @emoji ⏳️ One shard's ownership as held by this process — wraps `db_storage::LeaseStorage`
/// (already the fencing primitive, per its own doc) with the specific resource/holder/fence tuple
/// a shard-scoped write path checks before mutating anything. The primitive `resolve_split_brain`/
/// `reconcile_shard_owner` build on for failover.
#[derive(Clone, Debug)]
pub struct ShardOwnership {
    pub shard: String,
    pub holder: NodeId,
    pub fence: EpochFence,
}

impl ShardOwnership {
    /// @emoji 🤝️ Claims (or idempotently reaffirms) ownership of `shard` for `holder` — thin
    /// wrapper over `LeaseStorage::acquire` that also remembers the resulting fence locally.
    pub async fn acquire(storage: &impl db_storage::LeaseStorage, shard: &str, holder: NodeId, ttl_ms: u64, now_ms: u64) -> Result<ShardOwnership, DbError> {
        let fence = storage.acquire(shard, &holder.0, ttl_ms, now_ms).await?;
        Ok(ShardOwnership { shard: shard.to_string(), holder, fence })
    }

    /// @emoji ♻️ Extends this ownership's TTL without changing its epoch. Errors `Fenced` if
    /// another node has since won the shard (see `LeaseStorage::renew`'s doc).
    pub async fn renew(&self, storage: &impl db_storage::LeaseStorage, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        storage.renew(&self.shard, &self.holder.0, self.fence, ttl_ms, now_ms).await
    }

    /// @emoji 🕊️ Voluntarily releases this ownership (e.g. graceful shutdown / planned handoff).
    pub async fn release(&self, storage: &impl db_storage::LeaseStorage) -> Result<(), DbError> {
        storage.release(&self.shard, &self.holder.0, self.fence).await
    }

    /// @emoji ✅️ Validates a write presented under `presented` against this ownership's fence — the
    /// primitive every shard-scoped write path calls before mutating storage.
    pub async fn validate(&self, presented: EpochFence) -> Result<(), DbError> {
        self.fence.check(presented)
    }
}

/// @emoji 🧭️ Whether `shard` is currently held, and by whom — `LeaseStorage::current`'s
/// cluster-flavored projection, feeding failover detection and `reconcile_shard_owner`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OwnershipStatus {
    Held { holder: NodeId, fence: EpochFence, expires_at_ms: u64 },
    Vacant,
}

/// @emoji 👀️ Reads `shard`'s current ownership from `storage` as of `now_ms`.
pub async fn ownership_status(storage: &impl db_storage::LeaseStorage, shard: &str, now_ms: u64) -> Result<OwnershipStatus, DbError> {
    Ok(match storage.current(shard, now_ms).await? {
        Some(mut info) => {
            let status = OwnershipStatus::Held { holder: NodeId(info.holder.as_str().to_string()), fence: info.fence, expires_at_ms: info.expires_at_ms };
            info.close_step();
            status
        }
        None => OwnershipStatus::Vacant,
    })
}
//#endregion 🔖️Ownership

//#region 🔖️Replication
/// @emoji 📡️ What `replicate_document` actually did to catch a follower up — lets the caller (e.g.
/// `db_engine`, once it exists) decide whether a follow-up snapshot-materialize step is needed.
#[derive(Clone, Debug, PartialEq)]
pub enum ReplicationOutcome {
    /// @emoji ✅️ The follower was already caught up; nothing was replicated.
    UpToDate { frontier: Frontier },
    /// @emoji 🚚️ `count` commands were appended to the follower's own WAL, advancing it to
    /// `frontier`.
    TailApplied { frontier: Frontier, count: usize },
    /// @emoji 📸️ The follower fell behind the leader's retained WAL floor; `generation`'s raw
    /// snapshot bytes were copied verbatim to the follower's `SnapshotStorage`. See the module
    /// doc's "extension seam" note: this does NOT advance the follower's WAL-derived frontier —
    /// materializing the snapshot into live state is `db_snapshot`/`db_artifact`'s job.
    SnapshotTransferred { generation: u64, pack_hash: [u8; 32] },
}

/// 📡️ Replication rejection retaining the exact follower writer through recovery and close faults.
#[must_use = "replication rejection must reach terminal follower-writer cleanup"]
pub enum ReplicationRejected {
    BeforeWriter(DbError),
    WalOpen(db_wal::ArtifactWalAcquiredRejected),
    WalRelease(db_wal::ArtifactWalOpenRejected),
    RetainedWal { cause: DbError, close_error: DbError, wal: db_wal::ArtifactWal },
}

impl ReplicationRejected {
    pub fn error(&self) -> &DbError {
        match self {
            Self::BeforeWriter(error) => error,
            Self::WalOpen(rejected) => rejected.error(),
            Self::WalRelease(rejected) => rejected.error(),
            Self::RetainedWal { cause, .. } => cause,
        }
    }

    pub async fn retry_close(self) -> Result<DbError, ReplicationRejected> {
        match self {
            Self::BeforeWriter(error) => Ok(error),
            Self::WalOpen(rejected) => rejected.into_open_rejected().retry_close().await.map_err(Self::WalRelease),
            Self::WalRelease(rejected) => rejected.retry_close().await.map_err(Self::WalRelease),
            Self::RetainedWal { cause, close_error: _, mut wal } => loop {
                match wal.close_step() {
                    Ok(true) => semio_framework_async::yield_once().await,
                    Ok(false) => return Ok(cause),
                    Err(error) => {
                        return Err(Self::RetainedWal { cause, close_error: error, wal });
                    }
                }
            },
        }
    }
}

impl From<DbError> for ReplicationRejected {
    fn from(error: DbError) -> Self {
        Self::BeforeWriter(error)
    }
}

impl std::fmt::Debug for ReplicationRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ReplicationRejected").field("cause", self.error()).finish_non_exhaustive()
    }
}

/// @emoji 🔁️ Catches `document` up on `follower` against `leader`'s current state: replays both
/// sides' WALs (via `db_sync::replay_sync_state`), decides a `db_sync::BootstrapPlan`, and applies
/// it — appending missing commands to the follower's own WAL for the `Tail` case (this crate's
/// follower-WAL-consumption primitive), or copying the raw snapshot bytes for the `Snapshot` case
/// (this crate's snapshot-replication primitive; see `ReplicationOutcome::SnapshotTransferred`'s
/// doc for why that case stops short of full materialization).
pub async fn replicate_document(leader: &db_storage::DbBackend, follower: &db_storage::DbBackend, document: ArtifactId, policy: db_wal::GroupCommitPolicy, now_ms: u64) -> Result<ReplicationOutcome, ReplicationRejected> {
    let follower_storage = follower.wal().await;
    let writer = follower_storage.acquire_writer(&document).await.map_err(ReplicationRejected::BeforeWriter)?;
    let mut open_control = db_wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).map_err(ReplicationRejected::BeforeWriter)?;
    let (mut wal, _report) = db_wal::ArtifactWal::open_acquired(&follower_storage, writer, policy, now_ms, &mut open_control).await.map_err(ReplicationRejected::WalOpen)?;
    let outcome = async {
        let follower_state = db_sync::replay_sync_state(&follower_storage, document.clone()).await?;
        let leader_state = db_sync::replay_sync_state(&leader.wal().await, document.clone()).await?;
        if follower_state.frontier.head_seq >= leader_state.frontier.head_seq {
            return Ok(ReplicationOutcome::UpToDate { frontier: follower_state.frontier });
        }
        let plan = db_sync::decide_bootstrap(&leader_state, &leader.snapshot().await, Some(&follower_state.frontier)).await?;
        match plan {
            db_sync::BootstrapPlan::None => Ok(ReplicationOutcome::UpToDate { frontier: follower_state.frontier }),
            db_sync::BootstrapPlan::Tail { envelopes } => {
                let count = envelopes.len();
                let mut control = db_wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
                for envelope in &envelopes {
                    let bytes = db_sync::encode_command_envelope(envelope).await;
                    let bytes = match db_wal::WalBytes::try_admit(bytes, 1024 * 1024, &mut control).await {
                        Ok(bytes) => bytes,
                        Err(mut rejected) => {
                            while rejected.close_step()? {
                                semio_framework_async::yield_once().await;
                            }
                            return Err(rejected.into_error());
                        }
                    };
                    let mut records = db_wal::WalRecordBatch::new();
                    if let Err(mut record) = records.push(db_wal::WalRecord::Command(bytes)) {
                        while record.close_step()? {
                            semio_framework_async::yield_once().await;
                        }
                        return Err(DbError::LimitExceeded("db_cluster fixed wal record batch"));
                    }
                    let submitted = wal.submit(&follower_storage, &records, DurabilityClass::Fsync, now_ms).await;
                    let retired = async {
                        while records.close_step()? {
                            semio_framework_async::yield_once().await;
                        }
                        Ok(())
                    }
                    .await;
                    replication_retired(submitted, retired)?;
                }
                let frontier = db_sync::replay_sync_state(&follower_storage, document.clone()).await?.frontier;
                Ok(ReplicationOutcome::TailApplied { frontier, count })
            }
            db_sync::BootstrapPlan::Snapshot { generation, pages, pack_hash } => {
                let input = replication_snapshot_input(pages).await?;
                follower.snapshot().await.write_generation(&document, generation, input).await?;
                Ok(ReplicationOutcome::SnapshotTransferred { generation, pack_hash })
            }
        }
    }
    .await;
    match (outcome, wal.close().await) {
        (outcome, Ok(())) => outcome.map_err(ReplicationRejected::BeforeWriter),
        (Ok(_), Err(close_error)) => Err(ReplicationRejected::RetainedWal { cause: DbError::Unavailable("replication follower WAL close failed".to_string()), close_error, wal }),
        (Err(cause), Err(close_error)) => Err(ReplicationRejected::RetainedWal { cause, close_error, wal }),
    }
}

/// 📑️ Copies the admitted read result into an independent write owner before retiring its source task.
async fn replication_snapshot_input(mut pages: db_storage::DbIoPages) -> Result<db_storage::DbIoPages, DbError> {
    let copied = async { db_storage::db_io_copy_page_owner(&pages)?.await }.await;
    let retired = close_replication_pages(&mut pages).await;
    match (copied, retired) {
        (Ok(input), Ok(())) => Ok(input),
        (Err(error), retired) => replication_retired(Err(error), retired),
        (Ok(mut input), Err(error)) => replication_retired(Err(error), close_replication_pages(&mut input).await),
    }
}

async fn close_replication_pages(pages: &mut db_storage::DbIoPages) -> Result<(), DbError> {
    while pages.close_step()?.is_some() {
        semio_framework_async::yield_once().await;
    }
    Ok(())
}

fn replication_retired<T>(outcome: Result<T, DbError>, retired: Result<(), DbError>) -> Result<T, DbError> {
    match (outcome, retired) {
        (outcome, Ok(())) => outcome,
        (Ok(_), Err(error)) => Err(error),
        (Err(error), Err(cleanup)) => Err(DbError::Unavailable(format!("replication failed ({error}); retained cleanup failed ({cleanup})"))),
    }
}
//#endregion 🔖️Replication

//#region 🔖️Quorum
/// @emoji 🤝️ Tracks which nodes have acknowledged durability for one write (conceptually, one
/// `(document, frontier)` pair) — the cluster-side satisfaction check for
/// `DurabilityClass::Quorum(n)`. Ack-idempotent: acking the same node twice never
/// double-counts.
#[derive(Clone, Debug)]
pub struct QuorumTracker {
    threshold: u8,
    acked: std::collections::BTreeSet<NodeId>,
}

impl QuorumTracker {
    /// @emoji 🆕️ A tracker requiring `threshold` distinct acks to be satisfied.
    pub async fn new(threshold: u8) -> Self {
        Self { threshold, acked: std::collections::BTreeSet::new() }
    }

    /// @emoji ✅️ Records `node`'s ack. Returns `true` iff this call is the one that first reached
    /// the threshold (edge-triggered — the primitive `ClusterEvent::QuorumReached` fires exactly
    /// once from).
    pub async fn ack(&mut self, node: NodeId) -> bool {
        let was_satisfied = self.satisfied();
        self.acked.insert(node);
        !was_satisfied && self.satisfied()
    }

    /// @emoji 🥇️ True iff at least `threshold` distinct nodes have acked.
    // 🚫️async: E1 pure accessor consumed synchronously by tests and `ack` — see R9
    pub fn satisfied(&self) -> bool {
        self.acked.len() >= self.threshold as usize
    }

    /// @emoji 🔢️ How many distinct nodes have acked so far.
    pub async fn ack_count(&self) -> usize {
        self.acked.len()
    }
}

/// @emoji 🥇️ Whether `class` is satisfied given `replica_ack_count` distinct replica acks.
/// `Memory`/`Os`/`Fsync` are single-node durability concerns (satisfied the moment the local write
/// completes, before any cluster-level tracking is even consulted); only `Quorum(n)` needs a
/// cluster-wide ack count, which this fn gates on.
pub async fn durability_satisfied(class: DurabilityClass, replica_ack_count: usize) -> bool {
    match class {
        DurabilityClass::Quorum(n) => replica_ack_count >= n as usize,
        _ => true,
    }
}
//#endregion 🔖️Quorum

//#region 🔖️ReadRouting
/// @emoji 🔎️ What a read/preview needs from the replica it's routed to — a minimal,
/// `db_cluster`-owned projection of what `db_query`'s (not yet implemented this wave)
/// `Consistency` enum will eventually drive shard-level routing decisions with. Exactly what the
/// contract's "read/preview routing" responsibility needs: a target node, not a query result.
#[derive(Clone, Debug, PartialEq)]
pub enum ReadIntent {
    /// @emoji 🎯️ Must observe the shard's current leader (the only always-fresh replica).
    Canonical,
    /// @emoji 🏔️ Any replica whose frontier dominates `at_least` (`Frontier::dominates`) may serve
    /// it — the routing-level form of a `Consistency::AtLeast` query.
    BoundedStaleness { at_least: Frontier },
    /// @emoji 🌫️ Any replica at all, preferring the freshest — read-scaling with no consistency
    /// requirement.
    AnyReplica,
    /// @emoji 🎭️ Preview reads always target the leader: previews are ephemeral overlays that (per
    /// the contract's preview law, owned by `db_preview`, not yet implemented) only ever exist on
    /// the shard's owning actor.
    Preview,
}

/// @emoji 🧾️ One candidate replica's current state, as `route_read`'s routing decision input.
#[derive(Clone, Debug, PartialEq)]
pub struct ReplicaStatus {
    pub node: NodeId,
    pub frontier: Frontier,
    pub is_leader: bool,
}

/// @emoji 🧭️ Picks a target node for `intent` among `replicas`. Errors `Unavailable` if no
/// candidate satisfies `intent` (no leader present for `Canonical`/`Preview`, no replica meets
/// `BoundedStaleness`'s floor, or an empty replica set).
pub async fn route_read(intent: &ReadIntent, replicas: &[ReplicaStatus]) -> Result<NodeId, DbError> {
    match intent {
        ReadIntent::Canonical | ReadIntent::Preview => replicas.iter().find(|status| status.is_leader).map(|status| status.node.clone()).ok_or_else(|| DbError::Unavailable("no leader available to serve a canonical/preview read".to_string())),
        ReadIntent::BoundedStaleness { at_least } => {
            // 🎯️ A document-mismatched frontier can never dominate `at_least` — treated as
            // non-qualifying (`unwrap_or(false)`) rather than propagating the error, since a
            // routing decision over a heterogeneous replica list should skip a malformed candidate,
            // not fail the whole read.
            let mut candidates: Vec<&ReplicaStatus> = replicas.iter().filter(|status| status.frontier.dominates(at_least).unwrap_or(false)).collect();
            // 🎯️ Prefer offloading to a follower (read-scaling is the whole point of bounded
            // staleness); fall back to the leader only if no follower qualifies. Secondary sort by
            // node id for a deterministic pick among equally-eligible followers.
            candidates.sort_by(|a, b| a.is_leader.cmp(&b.is_leader).then_with(|| a.node.cmp(&b.node)));
            candidates.first().map(|status| status.node.clone()).ok_or_else(|| DbError::Unavailable("no replica meets the requested staleness bound".to_string()))
        }
        ReadIntent::AnyReplica => replicas.iter().max_by_key(|status| status.frontier.head_seq).map(|status| status.node.clone()).ok_or_else(|| DbError::Unavailable("no replica available".to_string())),
    }
}
//#endregion 🔖️ReadRouting

//#region 🔖️SplitBrain
/// @emoji ⚖️ The outcome of comparing two claimed epochs for the same shard — a strictly higher
/// epoch always wins (a newer leadership handoff supersedes an older one); equal epochs are `Tie`
/// (should not arise for a correctly-fenced single shard's two DIFFERENT claimants, but handled
/// without panicking rather than assumed unreachable).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SplitBrainOutcome {
    LocalWins,
    RemoteWins,
    Tie,
}

/// @emoji ⚖️ Compares `local`'s claimed epoch against `remote`'s.
pub async fn resolve_split_brain(local: EpochFence, remote: EpochFence) -> SplitBrainOutcome {
    match local.epoch.cmp(&remote.epoch) {
        std::cmp::Ordering::Greater => SplitBrainOutcome::LocalWins,
        std::cmp::Ordering::Less => SplitBrainOutcome::RemoteWins,
        std::cmp::Ordering::Equal => SplitBrainOutcome::Tie,
    }
}

/// @emoji 🚨️ The split-brain repair primitive: a node that believes it still owns `shard` (e.g.
/// after a network partition healed) re-validates its locally-held `claimed` ownership against
/// `storage`'s actual current state, since another node may have already won a failover while it
/// was partitioned. A still-matching holder+fence is confirmed as `LocalWins` (not a `Tie` — it's
/// not actually contested); anything else is decided by `resolve_split_brain` on the two fences.
pub async fn reconcile_shard_owner(storage: &impl db_storage::LeaseStorage, shard: &str, claimed: &ShardOwnership, now_ms: u64) -> Result<SplitBrainOutcome, DbError> {
    match ownership_status(storage, shard, now_ms).await? {
        OwnershipStatus::Vacant => Ok(SplitBrainOutcome::LocalWins),
        OwnershipStatus::Held { holder, fence, .. } if holder == claimed.holder && fence == claimed.fence => Ok(SplitBrainOutcome::LocalWins),
        OwnershipStatus::Held { fence, .. } => Ok(resolve_split_brain(claimed.fence, fence).await),
    }
}
//#endregion 🔖️SplitBrain

//#region 🔖️Coordinator
/// @emoji 📣️ A cluster-lifecycle event this crate hands to whatever supervises a shard (`db_engine`,
/// once it exists) via a `db_actor` mailbox — prioritized so a fencing loss is never queued behind
/// routine replication/quorum traffic.
#[derive(Clone, Debug, PartialEq)]
pub enum ClusterEvent {
    /// @emoji 🚨️ This node's `ShardOwnership` was fenced out by a newer epoch — must stop serving
    /// writes for the shard immediately.
    OwnershipLost { shard: String, fence: EpochFence },
    /// @emoji ✅️ A follower finished catching up to a given frontier.
    ReplicationCaughtUp { document: ArtifactId, frontier: Frontier },
    /// @emoji 🤝️ A quorum-durability threshold was just reached for a frontier.
    QuorumReached { document: ArtifactId, frontier: Frontier, acked: usize },
}

impl ClusterEvent {
    /// @emoji 🚦️ The mailbox lane this event is admitted under — see the type's own doc for why
    /// ownership loss preempts everything else.
    pub async fn priority(&self) -> Priority {
        match self {
            ClusterEvent::OwnershipLost { .. } => Priority::System,
            ClusterEvent::ReplicationCaughtUp { .. } => Priority::Recovery,
            ClusterEvent::QuorumReached { .. } => Priority::Live,
        }
    }
}

/// @emoji 📬️ A fresh `db_actor` mailbox for `ClusterEvent`s, sized per `capacities`.
pub async fn cluster_mailbox(capacities: MailboxCapacities) -> (db_actor::Address<ClusterEvent>, db_actor::Receiver<ClusterEvent>) {
    db_actor::mailbox(capacities)
}
//#endregion 🔖️Coordinator
//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
