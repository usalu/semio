//! 🗄️ `db_cluster` — sharding, ownership leases + epoch failover, follower WAL/snapshot
//! replication, quorum durability, and read/preview routing for the `db` crate family. Frozen
//! contract: `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
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
use db_storage::SnapshotStorage as _;
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
        Some(info) => OwnershipStatus::Held { holder: NodeId(info.holder), fence: info.fence, expires_at_ms: info.expires_at_ms },
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

/// @emoji 🔁️ Catches `document` up on `follower` against `leader`'s current state: replays both
/// sides' WALs (via `db_sync::replay_sync_state`), decides a `db_sync::BootstrapPlan`, and applies
/// it — appending missing commands to the follower's own WAL for the `Tail` case (this crate's
/// follower-WAL-consumption primitive), or copying the raw snapshot bytes for the `Snapshot` case
/// (this crate's snapshot-replication primitive; see `ReplicationOutcome::SnapshotTransferred`'s
/// doc for why that case stops short of full materialization).
pub async fn replicate_document(leader: &db_storage::DbBackend, follower: &db_storage::DbBackend, document: ArtifactId, policy: db_wal::GroupCommitPolicy, now_ms: u64) -> Result<ReplicationOutcome, DbError> {
    let follower_state = db_sync::replay_sync_state(&follower.wal().await, document.clone()).await?;
    let leader_state = db_sync::replay_sync_state(&leader.wal().await, document.clone()).await?;
    if follower_state.frontier.head_seq >= leader_state.frontier.head_seq {
        return Ok(ReplicationOutcome::UpToDate { frontier: follower_state.frontier });
    }
    let plan = db_sync::decide_bootstrap(&leader_state, &leader.snapshot().await, Some(&follower_state.frontier)).await?;
    match plan {
        db_sync::BootstrapPlan::None => Ok(ReplicationOutcome::UpToDate { frontier: follower_state.frontier }),
        db_sync::BootstrapPlan::Tail { envelopes } => {
            let count = envelopes.len();
            let (mut wal, _report) = db_wal::ArtifactWal::open(&follower.wal().await, document.clone(), policy, now_ms).await?;
            for envelope in &envelopes {
                let bytes = db_sync::encode_command_envelope(envelope);
                wal.submit(&follower.wal().await, &[db_wal::WalRecord::Command(bytes.await)], DurabilityClass::Fsync, now_ms).await?;
            }
            let frontier = db_sync::replay_sync_state(&follower.wal().await, document).await?.frontier;
            Ok(ReplicationOutcome::TailApplied { frontier, count })
        }
        db_sync::BootstrapPlan::Snapshot { generation, bytes, pack_hash } => {
            let pages = db_storage::DbIoPages::try_new(bytes).map_err(|_| DbError::LimitExceeded("replicated snapshot pages"))?;
            follower.snapshot().await.write_generation(&document, generation, pages).await?;
            Ok(ReplicationOutcome::SnapshotTransferred { generation, pack_hash })
        }
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
mod tests {
    use super::*;

    //#region 🔖️ShardMap
    #[semio_framework_async_macros::async_test]
    async fn shard_map_owner_is_stable_for_a_fixed_ring() {
        let mut map = ShardMap::new(32).await;
        map.add_node(&NodeId::from("node-a")).await;
        map.add_node(&NodeId::from("node-b")).await;
        map.add_node(&NodeId::from("node-c")).await;
        let doc: ArtifactId = "doc-42".into();
        assert_eq!(map.owner(&doc).await, map.owner(&doc).await);
        assert!(map.owner(&doc).await.is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn shard_map_owner_is_none_for_an_empty_ring() {
        let map = ShardMap::new(32).await;
        assert_eq!(map.owner(&"doc-1".into()).await, None);
    }

    #[semio_framework_async_macros::async_test]
    async fn shard_map_removing_a_node_only_remaps_documents_it_owned() {
        let mut map = ShardMap::new(64).await;
        let (a, b, c) = (NodeId::from("node-a"), NodeId::from("node-b"), NodeId::from("node-c"));
        map.add_node(&a).await;
        map.add_node(&b).await;
        map.add_node(&c).await;

        let docs: Vec<ArtifactId> = (0..200).map(|i| ArtifactId(format!("doc-{i}"))).collect();
        let before: Vec<NodeId> = docs.iter().map(|doc| db_actor::block_on(map.owner(doc)).unwrap()).collect();

        map.remove_node(&b).await;
        let after: Vec<NodeId> = docs.iter().map(|doc| db_actor::block_on(map.owner(doc)).unwrap()).collect();

        for (prior, later) in before.iter().zip(after.iter()) {
            if *prior == b {
                assert_ne!(later, &b, "a document owned by the removed node must move to a remaining node");
            } else {
                assert_eq!(prior, later, "a document not owned by the removed node must keep its owner");
            }
        }
        assert!(before.contains(&b), "sanity: node-b must have owned at least one sample document before removal");
    }

    #[semio_framework_async_macros::async_test]
    async fn shard_map_adding_a_node_remaps_only_a_minority_of_documents() {
        let mut map = ShardMap::new(128).await;
        map.add_node(&NodeId::from("node-a")).await;
        map.add_node(&NodeId::from("node-b")).await;
        map.add_node(&NodeId::from("node-c")).await;

        let docs: Vec<ArtifactId> = (0..1000).map(|i| ArtifactId(format!("doc-{i}"))).collect();
        let before: Vec<NodeId> = docs.iter().map(|doc| db_actor::block_on(map.owner(doc)).unwrap()).collect();

        map.add_node(&NodeId::from("node-d")).await;
        let after: Vec<NodeId> = docs.iter().map(|doc| db_actor::block_on(map.owner(doc)).unwrap()).collect();

        let moved = before.iter().zip(after.iter()).filter(|(prior, later)| prior != later).count();
        assert!(moved > 0, "adding a node should move at least some documents to it");
        assert!(moved < docs.len() / 2, "adding one of four nodes should remap well under half the keys, got {moved}/{}", docs.len());
    }
    //#endregion 🔖️ShardMap

    //#region 🔖️Ownership
    #[semio_framework_async_macros::async_test]
    async fn shard_ownership_acquire_renew_and_validate_round_trip() {
        let storage = db_storage::MemoryStorage::new().await;
        let owner = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-a"), 1_000, 0)).unwrap();
        assert_eq!(owner.fence, EpochFence::INITIAL);
        assert!(owner.validate(EpochFence::INITIAL).await.is_ok());
        assert!(owner.validate(EpochFence::INITIAL.next()).await.is_err());

        db_actor::block_on(owner.renew(&storage, 1_000, 500)).unwrap();
        assert_eq!(db_actor::block_on(ownership_status(&storage, "shard-0", 500)).unwrap(), OwnershipStatus::Held { holder: NodeId::from("node-a"), fence: EpochFence::INITIAL, expires_at_ms: 1_500 });
    }

    #[semio_framework_async_macros::async_test]
    async fn ownership_status_reports_vacant_before_any_acquire() {
        let storage = db_storage::MemoryStorage::new().await;
        assert_eq!(db_actor::block_on(ownership_status(&storage, "shard-0", 0)).unwrap(), OwnershipStatus::Vacant);
    }

    #[semio_framework_async_macros::async_test]
    async fn shard_ownership_release_frees_the_resource_for_a_fresh_acquire() {
        let storage = db_storage::MemoryStorage::new().await;
        let owner = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-a"), 1_000, 0)).unwrap();
        db_actor::block_on(owner.release(&storage)).unwrap();
        assert_eq!(db_actor::block_on(ownership_status(&storage, "shard-0", 0)).unwrap(), OwnershipStatus::Vacant);

        let reacquired = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-b"), 1_000, 0)).unwrap();
        assert_eq!(reacquired.fence, EpochFence::INITIAL);
    }

    #[semio_framework_async_macros::async_test]
    async fn failover_via_lease_expiry_bumps_the_epoch_and_hands_off_to_the_new_leader() {
        let storage = db_storage::MemoryStorage::new().await;
        let stale = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-a"), 100, 0)).unwrap();
        assert_eq!(stale.fence, EpochFence::INITIAL);

        let fresh = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-b"), 100, 200)).unwrap();
        assert_eq!(fresh.fence, EpochFence::INITIAL.next());
        assert_eq!(db_actor::block_on(reconcile_shard_owner(&storage, "shard-0", &stale, 200)).unwrap(), SplitBrainOutcome::RemoteWins);
    }
    //#endregion 🔖️Ownership

    //#region 🔖️Replication
    async fn sample_envelope(id: &str, seq: u64) -> protocol::MutationEnvelope {
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: protocol::ArtifactId("doc-1".to_string()),
            actor: protocol::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId("diff.v1".to_string()), payload: seq.to_le_bytes().to_vec() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: protocol::HybridLogicalTimestamp::new(1, seq),
        }
    }

    async fn seed_leader_wal(storage: &db_storage::MemoryStorage, document: &ArtifactId, count: u64) {
        let mut wal = db_actor::block_on(db_wal::ArtifactWal::create(storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
        for i in 0..count {
            let envelope = sample_envelope(&format!("op-{i}"), i).await;
            let bytes = db_sync::encode_command_envelope(&envelope).await;
            db_actor::block_on(wal.submit(storage, &[db_wal::WalRecord::Command(bytes)], DurabilityClass::Fsync, i)).unwrap();
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn replicate_document_applies_missing_tail_commands_to_a_fresh_follower() {
        let leader = db_storage::MemoryStorage::new().await;
        let follower = db_storage::MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_leader_wal(&leader, &document, 4).await;
        let leader: db_storage::DbBackend = db_storage::DbBackend::Memory(leader);
        let follower: db_storage::DbBackend = db_storage::DbBackend::Memory(follower);

        let outcome = db_actor::block_on(replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
        match outcome {
            ReplicationOutcome::TailApplied { frontier, count } => {
                assert_eq!(count, 4);
                assert_eq!(frontier.head_seq, 4);
            }
            other => panic!("expected TailApplied, got {other:?}"),
        }

        let follower_state = db_actor::block_on(async { db_sync::replay_sync_state(&follower.wal().await, document).await }).unwrap();
        assert_eq!(follower_state.commands.len(), 4);
        assert_eq!(follower_state.commands[0].mutation_id.0, "op-0");
        assert_eq!(follower_state.commands[3].mutation_id.0, "op-3");
    }

    #[semio_framework_async_macros::async_test]
    async fn replicate_document_reports_up_to_date_once_a_follower_catches_up() {
        let leader = db_storage::MemoryStorage::new().await;
        let follower = db_storage::MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_leader_wal(&leader, &document, 2).await;
        let leader: db_storage::DbBackend = db_storage::DbBackend::Memory(leader);
        let follower: db_storage::DbBackend = db_storage::DbBackend::Memory(follower);

        let first = db_actor::block_on(replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
        assert!(matches!(first, ReplicationOutcome::TailApplied { count: 2, .. }));

        let second = db_actor::block_on(replicate_document(&leader, &follower, document, db_wal::GroupCommitPolicy::default(), 100)).unwrap();
        assert!(matches!(second, ReplicationOutcome::UpToDate { .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn replicate_document_transfers_a_snapshot_when_the_follower_is_below_the_retained_floor() {
        let leader = db_storage::MemoryStorage::new().await;
        let follower = db_storage::MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_leader_wal(&leader, &document, 5).await;

        let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1u8; 32], epoch: 0 };
        {
            let (mut wal, _report) = db_actor::block_on(db_wal::ArtifactWal::open(&leader, document.clone(), db_wal::GroupCommitPolicy::default(), 1_000)).unwrap();
            db_actor::block_on(wal.submit(&leader, &[db_wal::WalRecord::SnapshotPub { generation: 9, frontier: floor_frontier }], DurabilityClass::Fsync, 1_000)).unwrap();
        }
        db_actor::block_on(db_storage::SnapshotStorage::write_generation(&leader, &document, 9, b"snapshot-bytes")).unwrap();
        let leader: db_storage::DbBackend = db_storage::DbBackend::Memory(leader);
        let follower: db_storage::DbBackend = db_storage::DbBackend::Memory(follower);

        let outcome = db_actor::block_on(replicate_document(&leader, &follower, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
        match outcome {
            ReplicationOutcome::SnapshotTransferred { generation, pack_hash } => {
                assert_eq!(generation, 9);
                assert_ne!(pack_hash, [0u8; 32]);
            }
            other => panic!("expected SnapshotTransferred, got {other:?}"),
        }
        // 🪡 `DbBackend` itself carries no blanket `SnapshotStorage` impl (only its variant
        // payloads do — `MemoryStorage`, `FsStorage<R>`, …), so the read must go through the
        // matched-out `Memory` payload, not the enum wrapper `replicate_document` above took by reference.
        let db_storage::DbBackend::Memory(ref follower_storage) = follower else { panic!("expected a Memory backend") };
        let copied = db_actor::block_on(db_storage::SnapshotStorage::read_generation(follower_storage, &document, 9)).unwrap();
        assert_eq!(copied, b"snapshot-bytes");
    }
    //#endregion 🔖️Replication

    //#region 🔖️Quorum
    #[semio_framework_async_macros::async_test]
    async fn quorum_tracker_is_ack_idempotent_and_edge_triggers_exactly_once() {
        let mut tracker = QuorumTracker::new(2).await;
        assert!(!tracker.satisfied());
        assert!(!tracker.ack(NodeId::from("node-a")).await);
        assert!(!tracker.ack(NodeId::from("node-a")).await, "acking the same node twice must not count twice");
        assert_eq!(tracker.ack_count().await, 1);
        assert!(tracker.ack(NodeId::from("node-b")).await, "the second distinct ack should cross the threshold");
        assert!(tracker.satisfied());
        assert!(!tracker.ack(NodeId::from("node-c")).await, "already satisfied, so a further ack is not edge-triggering");
    }

    #[semio_framework_async_macros::async_test]
    async fn durability_satisfied_only_gates_the_quorum_class_on_ack_count() {
        assert!(durability_satisfied(DurabilityClass::Memory, 0).await);
        assert!(durability_satisfied(DurabilityClass::Os, 0).await);
        assert!(durability_satisfied(DurabilityClass::Fsync, 0).await);
        assert!(!durability_satisfied(DurabilityClass::Quorum(3), 2).await);
        assert!(durability_satisfied(DurabilityClass::Quorum(3), 3).await);
    }
    //#endregion 🔖️Quorum

    //#region 🔖️ReadRouting
    async fn replica(node: &str, head_seq: u64, is_leader: bool) -> ReplicaStatus {
        ReplicaStatus { node: NodeId::from(node), frontier: Frontier { document: "doc-1".into(), head_seq, commit_seq: head_seq, chain_hash: [0u8; 32], epoch: 0 }, is_leader }
    }

    #[semio_framework_async_macros::async_test]
    async fn route_read_canonical_requires_the_leader() {
        let replicas = vec![replica("follower-1", 10, false).await, replica("leader", 10, true).await];
        assert_eq!(route_read(&ReadIntent::Canonical, &replicas).await.unwrap(), NodeId::from("leader"));
        assert!(route_read(&ReadIntent::Canonical, &[replica("follower-1", 10, false).await]).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn route_read_preview_also_requires_the_leader() {
        assert_eq!(route_read(&ReadIntent::Preview, &[replica("leader", 5, true).await]).await.unwrap(), NodeId::from("leader"));
    }

    #[semio_framework_async_macros::async_test]
    async fn route_read_bounded_staleness_prefers_a_qualifying_follower_over_the_leader() {
        let at_least = Frontier { document: "doc-1".into(), head_seq: 5, commit_seq: 5, chain_hash: [0u8; 32], epoch: 0 };
        let qualifying = vec![replica("leader", 10, true).await, replica("follower-1", 8, false).await];
        assert_eq!(route_read(&ReadIntent::BoundedStaleness { at_least: at_least.clone() }, &qualifying).await.unwrap(), NodeId::from("follower-1"));

        let none_qualify = vec![replica("leader", 10, true).await, replica("follower-1", 2, false).await];
        assert_eq!(route_read(&ReadIntent::BoundedStaleness { at_least }, &none_qualify).await.unwrap(), NodeId::from("leader"));
    }

    #[semio_framework_async_macros::async_test]
    async fn route_read_bounded_staleness_errors_when_nothing_qualifies() {
        let at_least = Frontier { document: "doc-1".into(), head_seq: 5, commit_seq: 5, chain_hash: [0u8; 32], epoch: 0 };
        assert!(route_read(&ReadIntent::BoundedStaleness { at_least }, &[replica("follower-1", 1, false).await]).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn route_read_any_replica_picks_the_freshest() {
        let replicas = vec![replica("follower-1", 3, false).await, replica("leader", 10, true).await, replica("follower-2", 7, false).await];
        assert_eq!(route_read(&ReadIntent::AnyReplica, &replicas).await.unwrap(), NodeId::from("leader"));
    }

    #[semio_framework_async_macros::async_test]
    async fn route_read_errors_on_an_empty_replica_set() {
        assert!(route_read(&ReadIntent::AnyReplica, &[]).await.is_err());
    }
    //#endregion 🔖️ReadRouting

    //#region 🔖️SplitBrain
    #[semio_framework_async_macros::async_test]
    async fn resolve_split_brain_prefers_the_higher_epoch() {
        let low = EpochFence::INITIAL;
        let high = low.next();
        assert_eq!(resolve_split_brain(high, low).await, SplitBrainOutcome::LocalWins);
        assert_eq!(resolve_split_brain(low, high).await, SplitBrainOutcome::RemoteWins);
        assert_eq!(resolve_split_brain(low, low).await, SplitBrainOutcome::Tie);
    }

    #[semio_framework_async_macros::async_test]
    async fn reconcile_shard_owner_confirms_a_still_valid_local_claim() {
        let storage = db_storage::MemoryStorage::new().await;
        let owner = db_actor::block_on(ShardOwnership::acquire(&storage, "shard-0", NodeId::from("node-a"), 1_000, 0)).unwrap();
        assert_eq!(db_actor::block_on(reconcile_shard_owner(&storage, "shard-0", &owner, 0)).unwrap(), SplitBrainOutcome::LocalWins);
    }

    #[semio_framework_async_macros::async_test]
    async fn reconcile_shard_owner_reports_vacant_shard_as_uncontested_local_win() {
        let storage = db_storage::MemoryStorage::new().await;
        let owner = ShardOwnership { shard: "shard-0".to_string(), holder: NodeId::from("node-a"), fence: EpochFence::INITIAL };
        assert_eq!(db_actor::block_on(reconcile_shard_owner(&storage, "shard-0", &owner, 0)).unwrap(), SplitBrainOutcome::LocalWins);
    }
    //#endregion 🔖️SplitBrain

    //#region 🔖️Coordinator
    #[semio_framework_async_macros::async_test]
    async fn cluster_mailbox_drains_higher_priority_events_before_lower_ones() {
        let (address, receiver) = cluster_mailbox(MailboxCapacities::uniform(8)).await;
        let live = ClusterEvent::QuorumReached { document: "doc-1".into(), frontier: Frontier::genesis("doc-1".into()), acked: 2 };
        let recovery = ClusterEvent::ReplicationCaughtUp { document: "doc-1".into(), frontier: Frontier::genesis("doc-1".into()) };
        let system = ClusterEvent::OwnershipLost { shard: "shard-0".to_string(), fence: EpochFence::INITIAL };

        address.try_send(live.priority().await, live.clone()).unwrap();
        address.try_send(recovery.priority().await, recovery.clone()).unwrap();
        address.try_send(system.priority().await, system.clone()).unwrap();

        assert_eq!(receiver.try_recv().unwrap().payload, system);
        assert_eq!(receiver.try_recv().unwrap().payload, recovery);
        assert_eq!(receiver.try_recv().unwrap().payload, live);
        assert!(receiver.try_recv().is_none());
    }
    //#endregion 🔖️Coordinator
}
//#endregion 🧪️Tests
