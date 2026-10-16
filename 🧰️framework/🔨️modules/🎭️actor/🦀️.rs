//! 🎭️ Pooled actor/plugin runtime kernel — domain-neutral, pure: no I/O, no clock (callers pass
//! `now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread` in this file. Transports are
//! injected via the [`ShardTransport`] trait; the one exception, [`ThreadTransport`], lives behind
//! `#[cfg(not(target_arch = "wasm32"))]` because it is built on `std::sync::mpsc`.
//!
//! 🪡 **Opaque seam**: `Effect`, `Event` and `UiPatch` are owned by a parallel packet in
//! `🎠️kernel` and are NOT defined here. This crate carries them as pack-encoded `Vec<u8>` inside
//! [`TurnResult`] (`ui_patches`, `effects`) and [`Payload::Event`] — the kernel crate encodes/decodes
//! the concrete typed collections; this crate only ever moves already-encoded bytes. The same seam
//! applies to `CapabilityGrant`: `kernel::CapabilityGrant` (`🛂️manifest/🦀️.rs`) is an
//! application-layer type this framework-tier crate must not depend on (would invert the
//! framework → os-product layering), so [`CapabilityGrant`] here is a minimal, local, pack-codeable
//! stand-in reconciled by the `B1-host-native` packet at integration time.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md` §1.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::sync::Arc;

use semio_framework_value_derive::{FromValue, ToValue};

pub use semio_framework_job as job;
pub use semio_framework_actor_retained_turn as retained_turn;
pub use semio_framework_actor_retained_context as retained_context;
pub use semio_framework_actor_pack as pack;
pub use semio_framework_actor_vocabulary::*;
pub use semio_framework_actor_job_publication::*;
pub use retained_turn::{OriginalActorParkRefusal, OriginalActorParkedTurn, OriginalActorParkedOutcome, OriginalActorParkedReturn, IssuedRetainedBatch, IssuedRetainedRecipient, OriginalActorTurnLoan, OriginalActorTurnWork, RetainedTurnInput, RetainedTurnReceipt, OriginalActorInput, ActorGuestPhase, OriginalActorGuestReturn, OriginalActorJobProvider, OriginalJobStep, OriginalJobOutputState, OriginalJobOutcome, OriginalJobOutputChunk, OriginalJobOutputReply, OriginalJobOutputAckReply, OriginalJobSource, OriginalJobSourceCursor, OriginalJobTurnLoan, OriginalJobBudget, OriginalJobStartReply, OriginalJobStepReply, OriginalJobCancelReply, OriginalJobCallFailure};
pub use retained_turn::{OriginalGuestExecutionBudget, OriginalGuestExecutionReceipt, OriginalJobSourceWireCursor, OriginalJobSourceWireHeader};

#[path = "🎠️activation-reservation/🦀️.rs"]
pub mod activation;

//#region 🧬️SchemaMetadata
#[cfg(test)]
pub mod schema_metadata {
    use std::collections::HashSet;

    /// 🧬️ One versioned wire type and its owned TypeScript projection.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SchemaMetadata {
        pub name: &'static str,
        pub version: u16,
        pub typescript: &'static str,
    }

    pub const TYPES: &[SchemaMetadata] = &[
        SchemaMetadata { name: "ActivationEvent", version: 1, typescript: r#"export type ActivationEvent = { "kind": "manual" } | { "kind": "windowOpen", window: WindowId, } | { "kind": "restart" };"# },
        SchemaMetadata { name: "ActorId", version: 1, typescript: "export type ActorId = bigint;" },
        SchemaMetadata {
            name: "ActorKind",
            version: 1,
            typescript: r#"export type ActorKind = { "kind": "pluginApp", plugin: PackageId, appId: string, instanceId: number, } | { "kind": "extension", plugin: PackageId, extensionId: string, } | { "kind": "job", owner: ActorId, jobId: bigint, };"#,
        },
        SchemaMetadata {
            name: "ActorMetrics",
            version: 1,
            typescript: "export type ActorMetrics = { turns: bigint, fuel_total: bigint, wall_us_total: bigint, wall_us_ring: Array<number>, wall_us_ring_len: number, wall_us_ring_pos: number, memory_bytes: bigint, mailbox_len: number, mailbox_lag_ms: number, coalesced: bigint, dropped: bigint, traps: number, restarts: number, stage: FailureStage, shard: ShardId, };",
        },
        SchemaMetadata { name: "ActorMetricsSample", version: 1, typescript: "export type ActorMetricsSample = { id: ActorId, package: PackageId, lane: Lane, status: ActorStatus, metrics: ActorMetrics, };" },
        SchemaMetadata {
            name: "ActorRecord",
            version: 1,
            typescript: "export type ActorRecord = { id: ActorId, kind: ActorKind, package: PackageId, shard: ShardId, capabilities: Array<CapabilityGrant>, budget: Budget, mailbox: Mailbox, status: ActorStatus, failure: FailureState, metrics: ActorMetrics, };",
        },
        SchemaMetadata {
            name: "ActorStatus",
            version: 1,
            typescript: r#"export type ActorStatus = { "kind": "cold" } | { "kind": "activating" } | { "kind": "active" } | { "kind": "suspended", checkpoint: Array<number> | null, } | { "kind": "draining" } | { "kind": "trapped" } | { "kind": "quarantined" } | { "kind": "disabled" };"#,
        },
        SchemaMetadata { name: "Backpressure", version: 1, typescript: r#"export type Backpressure = { "kind": "accept" } | { "kind": "coalesced" } | { "kind": "dropped", lane: Lane, } | { "kind": "rejected" };"# },
        SchemaMetadata { name: "Budget", version: 1, typescript: "export type Budget = { fuel: bigint, wall_ms: number, memory_bytes: bigint, ui_nodes: number, mailbox_len: number, max_effects: number, max_patch_bytes: number, max_frames: number, };" },
        SchemaMetadata { name: "CapabilityGrant", version: 1, typescript: "export type CapabilityGrant = { capability: string, scope: Array<number> | null, };" },
        SchemaMetadata { name: "CoalesceKey", version: 1, typescript: "export type CoalesceKey = string;" },
        SchemaMetadata { name: "Decision", version: 1, typescript: "export type Decision = { run: Array<TurnGrant>, wake_at: bigint | null, };" },
        SchemaMetadata {
            name: "Envelope",
            version: 1,
            typescript: "export type Envelope = { to: ActorId, from: Origin, lane: Lane, seq: bigint, deadline_ms: bigint | null, coalesce: CoalesceKey | null, cancel_of: bigint | null, payload: Payload, };",
        },
        SchemaMetadata {
            name: "FailureSignal",
            version: 1,
            typescript: r#"export type FailureSignal = { "kind": "deadlineOverrun", ratio: number, } | { "kind": "fuelExhausted" } | { "kind": "memoryLimit" } | { "kind": "mailboxOverflow" } | { "kind": "uiQuota" } | { "kind": "trap", detail: string, } | { "kind": "heartbeatMissed", count: number, } | { "kind": "manualReset" };"#,
        },
        SchemaMetadata {
            name: "FailureStage",
            version: 1,
            typescript: r#"export type FailureStage = { "kind": "healthy" } | { "kind": "warned" } | { "kind": "throttled", factor: number, } | { "kind": "suspended", until: bigint, } | { "kind": "cancelled" } | { "kind": "trapped", restarts: number, } | { "kind": "quarantined", until: bigint, } | { "kind": "disabled" };"#,
        },
        SchemaMetadata { name: "FailureState", version: 1, typescript: "export type FailureState = { stage: FailureStage, clean_turns: number, warn_count: number, restart_count: number, last_signal_ms: bigint, };" },
        SchemaMetadata { name: "JobCheckpoint", version: 1, typescript: "export type JobCheckpoint = { state: Array<number>, applied_progress: bigint, };" },
        SchemaMetadata { name: "JobCommitCandidate", version: 1, typescript: "export type JobCommitCandidate = { state: Array<number>, output: Array<number>, };" },
        SchemaMetadata { name: "JobOperation", version: 1, typescript: "export type JobOperation = { operation: bigint, base_revision: bigint, generation: bigint, preview_sequence: bigint, seed: bigint, };" },
        SchemaMetadata {
            name: "JobProgressIdentity",
            version: 1,
            typescript: "export type JobProgressIdentity = { actor: ActorId, job: bigint, operation: bigint, base_revision: bigint, generation: bigint, step_sequence: bigint, preview_sequence: bigint, };",
        },
        SchemaMetadata { name: "JobProgressKind", version: 1, typescript: r#"export type JobProgressKind = "yield" | "preview" | "checkpoint" | "commitValidated" | "cancelled" | "fault";"# },
        SchemaMetadata { name: "JobProgressReceipt", version: 1, typescript: "export type JobProgressReceipt = { identity: JobProgressIdentity, kind: JobProgressKind, applied_progress: bigint, owner_bytes: number, };" },
        SchemaMetadata { name: "JobPublication", version: 1, typescript: "export type JobPublication = { turn: JobTurn, outcome: JobStepOutcome, };" },
        SchemaMetadata { name: "JobReplayLog", version: 2, typescript: "export type JobReplayLog = { route: JobReplayRoute, sealedRecords: number, };" },
        SchemaMetadata { name: "JobReplayRequest", version: 1, typescript: "export type JobReplayRequest = { controller: number[], tool: number[], schema: number[], version: number, digest: number[], };" },
        SchemaMetadata {
            name: "JobReplayRoute",
            version: 1,
            typescript: "export type JobReplayRoute = { plugin: number[], package: number[], controller: number[], tool: number[], window: bigint, artifact: number[], requestSchema: number[], requestVersion: number, requestDigest: number[], };",
        },
        SchemaMetadata {
            name: "JobStepOutcome",
            version: 1,
            typescript: r#"export type JobStepOutcome = { "kind": "yield" } | { "kind": "previewReady", preview: Array<number>, } | { "kind": "checkpointReady", checkpoint: JobCheckpoint, } | { "kind": "complete", candidate: JobCommitCandidate, } | { "kind": "cancelled" } | { "kind": "fault", detail: Array<number>, };"#,
        },
        SchemaMetadata { name: "JobTurn", version: 1, typescript: "export type JobTurn = { job: bigint, operation: JobOperation, step_sequence: bigint, };" },
        SchemaMetadata { name: "KernelMetrics", version: 1, typescript: "export type KernelMetrics = { actors: number, shards: number, packages: number, };" },
        SchemaMetadata { name: "Lane", version: 1, typescript: r#"export type Lane = "Interactive" | "UserVisible" | "Background" | "Maintenance";"# },
        SchemaMetadata { name: "Mailbox", version: 1, typescript: "export type Mailbox = { capacity: number, len: number, };" },
        SchemaMetadata { name: "OriginalActorTurnWork", version: 1, typescript: "export type OriginalActorTurnWork = { clockBinding: bigint, absoluteDeadlineUs: bigint, fuel: bigint };" },
        SchemaMetadata { name: "OriginalActorTurnLoan", version: 1, typescript: "export type OriginalActorTurnLoan = { retainedPolicy: import(\"../../../🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts\").RetainedCloneGrant, workFuel: bigint, deadlineUs: bigint };" },
        SchemaMetadata { name: "Origin", version: 1, typescript: r#"export type Origin = { "kind": "ui", window: WindowId, } | { "kind": "actor", id: ActorId, } | { "kind": "kernel" } | { "kind": "bus", topic: string, };"# },
        SchemaMetadata { name: "PackageHash", version: 1, typescript: "export type PackageHash = number[];" },
        SchemaMetadata { name: "PackageId", version: 1, typescript: "export type PackageId = string;" },
        SchemaMetadata {
            name: "Payload",
            version: 1,
            typescript: r#"export type Payload = { "kind": "event", bytes: Array<number>, } | { "kind": "suspend", operation: JobOperation, appliedProgress: bigint, } | { "kind": "resume", operation: JobOperation, checkpoint: JobCheckpoint, } | { "kind": "cancel", seq: bigint, } | { "kind": "jobStep", turn: JobTurn, };"#,
        },
        SchemaMetadata { name: "RetainedTurnInput", version: 3, typescript: "export type RetainedTurnInput = { operation: bigint, generation: bigint, epoch: bigint, work: OriginalActorTurnWork, grant: import(\"../../../🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts\").RetainedCloneGrant, };" },
        SchemaMetadata { name: "RetainedTurnReceipt", version: 3, typescript: "export type RetainedTurnReceipt = { input: RetainedTurnInput, spent: { copiedItems: number, copiedBytes: number, retainedCapacityBytes: number, releasedBytes: number }, workUnitsSpent: bigint, remainingFuel: bigint, remaining: import(\"../../../🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts\").RetainedCloneGrant, };" },
        SchemaMetadata { name: "RuntimeMetricsSnapshot", version: 1, typescript: "export type RuntimeMetricsSnapshot = { kernel: KernelMetrics, actors: Array<ActorMetricsSample>, shards: Array<ShardMetricsSample>, sampled_at_ms: bigint, };" },
        SchemaMetadata { name: "SceneSnapshot", version: 1, typescript: "export type SceneSnapshot = { revision: bigint, committed_ms: bigint, patches: Array<number>, node_count: number, };" },
        SchemaMetadata { name: "ShardId", version: 1, typescript: "export type ShardId = number;" },
        SchemaMetadata { name: "ShardKind", version: 1, typescript: r#"export type ShardKind = "Native" | "WebWorker" | "Process";"# },
        SchemaMetadata { name: "ShardMetrics", version: 1, typescript: "export type ShardMetrics = { actors: number, busy_ratio: number, heartbeat_age_ms: number, };" },
        SchemaMetadata { name: "ShardMetricsSample", version: 1, typescript: "export type ShardMetricsSample = { shard: ShardId, metrics: ShardMetrics, };" },
        SchemaMetadata { name: "ShardTable", version: 1, typescript: "export type ShardTable = { kind: ShardKind, shard_count: number, exclusive_reserve: number, assignment: Record<string, ShardId>, exclusive_leases: Record<string, ActorId>, };" },
        SchemaMetadata { name: "TurnGrant", version: 2, typescript: "export type TurnGrant = { actor: ActorId, shard: ShardId, budget: Budget, retained: RetainedTurnInput, loan: OriginalActorTurnLoan, envelopes: Array<Envelope>, };" },
        SchemaMetadata {
            name: "TurnResult",
            version: 1,
            typescript: "export type TurnResult = { retained_receipt: RetainedTurnReceipt, ui_patches: Array<number>, effects: Array<number>, command_ingress: Array<number>, cold_pair_ingress: ColdPairIngressStatus, lifecycle_receipt: import(\"../../🚪️lifetime/🟦️.ts\").ActorInstanceLifecycleReceipt | null, ui_patch_receipt: import(\"../../🚪️lifetime/🩹️patch/🟦️.ts\").ActorUiPatchReceipt | null, next_wake: bigint | null, status: TurnStatus, usage: Usage, };",
        },
        SchemaMetadata {
            name: "ColdPairIngressStatus",
            version: 1,
            typescript: r#"export type ColdPairIngressStatus = { "kind": "idle" } | { "kind": "pageAccepted", cursor: ColdDocumentPairCursor, } | { "kind": "backpressure", cursor: ColdDocumentPairCursor, } | { "kind": "loading", cursor: ColdDocumentPairCursor, } | { "kind": "applied", receipt: ColdDocumentPairApplied, } | { "kind": "fault", cursor: ColdDocumentPairCursor, fault: Array<number>, };"#,
        },
        SchemaMetadata {
            name: "ColdDocumentPairCursor",
            version: 1,
            typescript: "export type ColdDocumentPairCursor = { lifetime: import(\"../../🚪️lifetime/🟦️.ts\").ActorInstanceLifetime, transfer_generation: bigint, page_index: number, page_count: number, };",
        },
        SchemaMetadata {
            name: "ColdDocumentPairFrontier",
            version: 1,
            typescript: "export type ColdDocumentPairFrontier = { document_id: string, head_edit_ordinal: bigint, head_edit_id: string, last_commit_seq: bigint, chain_sha256: Array<number>, };",
        },
        SchemaMetadata {
            name: "ColdDocumentPairApplied",
            version: 1,
            typescript: "export type ColdDocumentPairApplied = { lifetime: import(\"../../🚪️lifetime/🟦️.ts\").ActorInstanceLifetime, transfer_generation: bigint, baseline_frontier: ColdDocumentPairFrontier, aggregate_sha256: Array<number>, };",
        },
        SchemaMetadata {
            name: "TurnStatus",
            version: 1,
            typescript: r#"export type TurnStatus = { "kind": "idle" } | { "kind": "moreWork" } | { "kind": "checkpointReady", checkpoint: JobCheckpoint, } | { "kind": "faulted", detail: Array<number>, } | { "kind": "previewReady", preview: Array<number>, sequence: bigint, } | { "kind": "commitReady", candidate: JobCommitCandidate, } | { "kind": "cancelled" };"#,
        },
        SchemaMetadata { name: "Usage", version: 1, typescript: "export type Usage = { fuel: bigint, wall_us: bigint, memory_bytes: bigint, };" },
        SchemaMetadata { name: "WindowId", version: 1, typescript: "export type WindowId = number;" },
    ];

    /// 🔍️ Rejects unversioned, duplicate, or name-mismatched schema rows before generation.
    pub fn validate() -> Result<(), String> {
        let mut names = HashSet::with_capacity(TYPES.len());
        for metadata in TYPES {
            if metadata.version == 0 {
                return Err(format!("schema `{}` has version zero", metadata.name));
            }
            if !names.insert(metadata.name) {
                return Err(format!("duplicate schema `{}`", metadata.name));
            }
            let prefix = format!("export type {} = ", metadata.name);
            if !metadata.typescript.starts_with(&prefix) {
                return Err(format!("schema `{}` declaration has a mismatched name", metadata.name));
            }
        }
        Ok(())
    }

    /// 🟦️ Renders the stable language projection consumed by the TypeScript package.
    pub fn render_typescript() -> String {
        let mut output = String::from("/** @generated by `bun repo run @semio-tech/framework-actor-rs:typegen` from 🎭️actor owned schema metadata. Do not edit. */\n\n");
        for metadata in TYPES {
            output.push_str(metadata.typescript);
            output.push_str("\n\n");
        }
        output
    }
}
//#endregion 🧬️SchemaMetadata

/// 🗂️ Full snapshot of one actor — assembled on demand by [`Kernel::actor_record`] from the
/// scheduler's live entry plus this actor's kind/capabilities/status/failure/metrics bookkeeping.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct ActorRecord {
    pub id: ActorId,
    pub kind: ActorKind,
    pub package: PackageId,
    pub shard: ShardId,
    pub capabilities: Vec<CapabilityGrant>,
    pub budget: Budget,
    pub mailbox: Mailbox,
    pub status: ActorStatus,
    pub failure: FailureState,
    pub metrics: ActorMetrics,
}

impl ActorRecord {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.id.pack_encode(out).await;
        self.kind.pack_encode(out).await;
        self.package.pack_encode(out).await;
        self.shard.pack_encode(out).await;
        pack::write_vec(out, &self.capabilities, CapabilityGrant::pack_encode).await;
        self.budget.pack_encode(out).await;
        self.mailbox.pack_encode(out).await;
        self.status.pack_encode(out).await;
        self.failure.pack_encode(out).await;
        self.metrics.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            id: ActorId::pack_decode(bytes, pos).await?,
            kind: ActorKind::pack_decode(bytes, pos).await?,
            package: PackageId::pack_decode(bytes, pos).await?,
            shard: ShardId::pack_decode(bytes, pos).await?,
            capabilities: pack::read_vec(bytes, pos, "ActorRecord::capabilities", CapabilityGrant::pack_decode).await?,
            budget: Budget::pack_decode(bytes, pos).await?,
            mailbox: Mailbox::pack_decode(bytes, pos).await?,
            status: ActorStatus::pack_decode(bytes, pos).await?,
            failure: FailureState::pack_decode(bytes, pos).await?,
            metrics: ActorMetrics::pack_decode(bytes, pos).await?,
        })
    }
}
//#endregion 🗂️ActorRecord

/// 🔁️ `ToValue`/`FromValue` for `BTreeMap<ActorId, ShardId>` — the derive's generic `BTreeMap`
/// support is keyed on `String` only, and a bare-integer-newtype key (`ActorId`) needs its own
/// stringified-key `DslValue::Object` shape (mirrors what `serde_json` itself produces for a
/// newtype-struct map key: the inner integer, stringified).
fn actor_shard_map_to_value(map: &BTreeMap<ActorId, ShardId>) -> ::semio_framework_value::DslValue {
    ::semio_framework_value::DslValue::object(map.iter().map(|(id, shard)| (id.0.to_string(), ::semio_framework_value::ToValue::to_value(shard))))
}
fn actor_shard_map_from_value(value: ::semio_framework_value::DslValue) -> Result<BTreeMap<ActorId, ShardId>, ::semio_framework_value::ValueError> {
    let ::semio_framework_value::DslValue::Object(entries) = value else {
        return Err(::semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected object"));
    };
    entries
        .into_iter()
        .map(|(key, value)| {
            let id: u64 = key.parse().map_err(|_| ::semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid ActorId key `{key}`")))?;
            let shard: ShardId = ::semio_framework_value::FromValue::from_value(value).map_err(|error: ::semio_framework_value::ValueError| error.under(&key))?;
            Ok((ActorId(id), shard))
        })
        .collect()
}

/// 🔁️ `ToValue`/`FromValue` for `BTreeMap<ShardId, ActorId>` — same stringified-key shape as
/// [`actor_shard_map_to_value`], keyed the other way round.
fn shard_actor_map_to_value(map: &BTreeMap<ShardId, ActorId>) -> ::semio_framework_value::DslValue {
    ::semio_framework_value::DslValue::object(map.iter().map(|(shard, id)| (shard.0.to_string(), ::semio_framework_value::ToValue::to_value(id))))
}
fn shard_actor_map_from_value(value: ::semio_framework_value::DslValue) -> Result<BTreeMap<ShardId, ActorId>, ::semio_framework_value::ValueError> {
    let ::semio_framework_value::DslValue::Object(entries) = value else {
        return Err(::semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected object"));
    };
    entries
        .into_iter()
        .map(|(key, value)| {
            let id: u16 = key.parse().map_err(|_| ::semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid ShardId key `{key}`")))?;
            let actor: ActorId = ::semio_framework_value::FromValue::from_value(value).map_err(|error: ::semio_framework_value::ValueError| error.under(&key))?;
            Ok((ShardId(id), actor))
        })
        .collect()
}

/// 🧩️ Fixed shard pool. An actor is pinned to a shard; migration only happens at a quiescent point
/// via application-level checkpoint (never a raw linear-memory snapshot). The last `exclusive_reserve`
/// shards are reserved for [`ShardTable::request_exclusive`] leases.
#[derive(Clone, Debug, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct ShardTable {
    pub kind: ShardKind,
    shard_count: u16,
    exclusive_reserve: u16,
    #[value(serialize_with = "actor_shard_map_to_value", deserialize_with = "actor_shard_map_from_value")]
    assignment: BTreeMap<ActorId, ShardId>,
    #[value(serialize_with = "shard_actor_map_to_value", deserialize_with = "shard_actor_map_from_value")]
    exclusive_leases: BTreeMap<ShardId, ActorId>,
}

impl ShardTable {
    pub async fn new(kind: ShardKind, shard_count: u16, exclusive_reserve: u16) -> Self {
        let exclusive_reserve = exclusive_reserve.min(2).min(shard_count.saturating_sub(1));
        Self { kind, shard_count: shard_count.max(1), exclusive_reserve, assignment: BTreeMap::new(), exclusive_leases: BTreeMap::new() }
    }

    pub async fn shard_count(&self) -> u16 {
        self.shard_count
    }

    /// 📌️ Pins an actor to the least-loaded shard of the non-exclusive pool, idempotently.
    ///
    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b): this was `actor.0 % pool`, which reads the
    /// LOW bits of the packed id — but the layout is `plugin_ordinal:u16 | kind:u2 | ordinal:u32 |
    /// generation:u14`, so those low bits are `generation`, which is **0 for every freshly activated
    /// actor**. Every actor pinned to shard 0; the scale bench measured `perShardCounts {"0": 100}`
    /// against 8 configured shards. The pooled-shard mechanism this ticket exists to build was
    /// distributing nothing, and no test caught it because "pin returns a valid shard" passes
    /// perfectly when every answer is 0.
    ///
    /// Least-loaded rather than a hash, because the balance bound is a REQUIREMENT, not a
    /// preference: `📓️design-workforce.md` §4's budget 3 demands no shard exceed
    /// `ceil(actors/K)+1`, which a hash cannot guarantee (its bucket variance exceeds that bound
    /// well before 100 actors) and exact balancing gives by construction. Least-loaded also refills
    /// the gaps [`unpin`] leaves, which a round-robin counter would stride straight past. Ties break
    /// on lowest shard id, so placement is deterministic — no clock, no RNG, crate stays pure.
    pub async fn pin(&mut self, actor: ActorId) -> ShardId {
        if let Some(existing) = self.assignment.get(&actor) {
            return *existing;
        }
        let shard = self.least_loaded(&BTreeSet::new()).await;
        self.assignment.insert(actor, shard);
        shard
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): same least-loaded
    /// placement as [`Self::pin`], factored so [`Self::pin_avoiding`] can reuse the identical
    /// count/tie-break arithmetic with a subset of shards excluded — this is why `pin`'s own budget-3
    /// balance guarantee (`pin_spreads_actors_of_one_plugin_across_the_pool`) is untouched by this
    /// packet: called with an empty `avoid` set, `pin`'s behaviour is byte-for-byte what it was before
    /// this method existed.
    async fn least_loaded(&self, avoid: &BTreeSet<ShardId>) -> ShardId {
        let pool = self.shard_count.saturating_sub(self.exclusive_reserve).max(1);
        let mut load = vec![0usize; pool as usize];
        for shard in self.assignment.values() {
            if (shard.0 as usize) < load.len() {
                load[shard.0 as usize] += 1;
            }
        }
        let chosen = load
            .iter()
            .enumerate()
            .filter(|(index, _)| !avoid.contains(&ShardId(*index as u16)))
            .min_by_key(|(index, count)| (**count, *index))
            .or_else(|| load.iter().enumerate().min_by_key(|(index, count)| (**count, *index)))
            .map_or(0, |(index, _)| index);
        ShardId(chosen as u16)
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): [`Self::pin`],
    /// restricted to shards NOT in `avoid` — "reserve headroom so an interactive actor is never
    /// co-resident with N CPU-bound ones," concretely: `Kernel::activate` calls this instead of
    /// `pin` for `Lane::Interactive` actors, passing every shard `Kernel::saturated_shards` currently
    /// judges CPU-saturating (observed `ActorMetrics::is_saturating`, never a fixture/profile name).
    /// Falls back to the ordinary unrestricted least-loaded shard when EVERY general-pool shard is in
    /// `avoid` — a fully-saturated pool must still admit the actor; `Backpressure::Rejected` (mailbox
    /// capacity) already owns "no room" semantics elsewhere, this method is never the place a
    /// placement request goes unanswered. Idempotent for an already-pinned actor, exactly like `pin`.
    pub async fn pin_avoiding(&mut self, actor: ActorId, avoid: &BTreeSet<ShardId>) -> ShardId {
        if let Some(existing) = self.assignment.get(&actor) {
            return *existing;
        }
        let shard = self.least_loaded(avoid).await;
        self.assignment.insert(actor, shard);
        shard
    }

    /// 📌️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): pins `actor` to an
    /// EXACT shard, bypassing `pin`/`pin_avoiding`'s least-loaded heuristic entirely. The extension-
    /// activation primitive: an extension actor pins to its PARENT's shard so parent↔extension
    /// `MessageEndpoint::Extension` traffic never crosses a transport (`Kernel::activate_pinned`'s own
    /// doc). Idempotent for an already-pinned actor, matching `pin`/`pin_avoiding`. `shard` is clamped
    /// into `[0, shard_count)` defensively — every real caller derives it from another actor's own
    /// live assignment, so an out-of-range value would only ever come from a caller bug, and clamping
    /// keeps that bug a misplacement rather than a silently-dropped actor.
    pub async fn pin_to(&mut self, actor: ActorId, shard: ShardId) -> ShardId {
        if let Some(existing) = self.assignment.get(&actor) {
            return *existing;
        }
        let shard = ShardId(shard.0.min(self.shard_count.saturating_sub(1)));
        self.assignment.insert(actor, shard);
        shard
    }

    pub async fn unpin(&mut self, actor: ActorId) {
        self.assignment.remove(&actor);
        self.exclusive_leases.retain(|_, held_by| *held_by != actor);
    }

    pub async fn shard_of(&self, actor: ActorId) -> Option<ShardId> {
        self.assignment.get(&actor).copied()
    }

    /// 🔒️ Leases one of the reserved exclusive shards to `actor` for the duration of foreground
    /// work — a lease, not a permanent per-plugin worker. `None` when none are free.
    pub async fn request_exclusive(&mut self, actor: ActorId) -> Option<ShardId> {
        if self.exclusive_reserve == 0 {
            return None;
        }
        let base = self.shard_count - self.exclusive_reserve;
        for offset in 0..self.exclusive_reserve {
            let candidate = ShardId(base + offset);
            if let std::collections::btree_map::Entry::Vacant(entry) = self.exclusive_leases.entry(candidate) {
                entry.insert(actor);
                self.assignment.insert(actor, candidate);
                return Some(candidate);
            }
        }
        None
    }

    pub async fn release_exclusive(&mut self, actor: ActorId) {
        self.exclusive_leases.retain(|_, held_by| *held_by != actor);
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.kind.pack_encode(out).await;
        pack::write_u16(out, self.shard_count).await;
        pack::write_u16(out, self.exclusive_reserve).await;
        pack::write_varint_u64(out, self.assignment.len() as u64).await;
        for (actor, shard) in &self.assignment {
            actor.pack_encode(out).await;
            shard.pack_encode(out).await;
        }
        pack::write_varint_u64(out, self.exclusive_leases.len() as u64).await;
        for (shard, actor) in &self.exclusive_leases {
            shard.pack_encode(out).await;
            actor.pack_encode(out).await;
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let kind = ShardKind::pack_decode(bytes, pos).await?;
        let shard_count = pack::read_u16(bytes, pos, "ShardTable::shard_count").await?;
        let exclusive_reserve = pack::read_u16(bytes, pos, "ShardTable::exclusive_reserve").await?;
        let assignment_len = pack::read_varint_u64(bytes, pos, "ShardTable::assignment").await? as usize;
        let mut assignment_pairs = Vec::with_capacity(assignment_len.min(1 << 20));
        for _ in 0..assignment_len {
            assignment_pairs.push((ActorId::pack_decode(bytes, pos).await?, ShardId::pack_decode(bytes, pos).await?));
        }
        let lease_len = pack::read_varint_u64(bytes, pos, "ShardTable::exclusive_leases").await? as usize;
        let mut lease_pairs = Vec::with_capacity(lease_len.min(1 << 20));
        for _ in 0..lease_len {
            lease_pairs.push((ShardId::pack_decode(bytes, pos).await?, ActorId::pack_decode(bytes, pos).await?));
        }
        Ok(Self { kind, shard_count, exclusive_reserve, assignment: assignment_pairs.into_iter().collect(), exclusive_leases: lease_pairs.into_iter().collect() })
    }
}
//#endregion 🧩️ShardTable

//#region ⏱️Scheduler
/// ⏱️ One actor's scheduling entry inside [`Scheduler`]: everything the DRR algorithm needs that
/// isn't shared plugin-level bookkeeping.
#[derive(Clone, Debug)]
struct ScheduledActor {
    retained: RetainedTurnInput,
    loan:OriginalActorTurnLoan,
    actual_received_work:u128,
    issued: Option<RetainedTurnInput>,
    returned: bool,
    actual_received_spent:semio_framework_value::RetainedCloneProgress,
    refused_return:Option<retained_turn::OriginalActorGuestReturn>,
    continuation:bool,
    package: PackageId,
    lane: Lane,
    budget: Budget,
    shard: ShardId,
    mailbox: Mailbox,
    active: bool,
    throttle: f32,
    deficit: i64,
}

impl ScheduledActor {
    fn can_issue(&self) -> bool {
        self.active && self.issued.is_none() && self.refused_return.is_none() && (self.continuation||!self.mailbox.is_empty()) && self.retained.validate().is_ok() && self.loan.validate_for(self.retained).is_ok() && self.loan.retained_policy.maximum_items!=0 && self.loan.work_fuel!=0 && self.retained.grant.maximum_items != 0 && (!self.returned || self.retained.epoch.checked_add(1).is_some())
    }
}

/// ⏱️ Result of one [`Scheduler::tick`] call: the turns granted this call, and (if nothing ran) the
/// earliest future timestamp worth ticking again for.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct Decision {
    pub run: Vec<TurnGrant>,
    pub wake_at: Option<u64>,
}

impl Decision {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), pack::PackError> {
        for grant in &self.run { grant.retained.validate().map_err(|_| pack::PackError::InvalidRetainedTurn("operation identity is absent"))?; }
        pack::write_varint_u64(out, self.run.len() as u64).await;
        for grant in &self.run { grant.pack_encode(out).await?; }
        pack::write_opt_u64(out, &self.wake_at).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { run: pack::read_vec(bytes, pos, "Decision::run", TurnGrant::pack_decode).await?, wake_at: pack::read_opt_u64(bytes, pos, "Decision::wake_at").await? })
    }
}

/// ⏱️ One granted turn: which actor, on which shard, with what (possibly throttle-scaled) budget,
/// and the envelopes drained from its mailbox for this turn.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct TurnGrant {
    pub actor: ActorId,
    pub shard: ShardId,
    pub budget: Budget,
    pub retained: RetainedTurnInput,
    pub loan:OriginalActorTurnLoan,
    pub envelopes: Vec<Envelope>,
}

impl TurnGrant {
    /// 🎟️ Borrows the scheduler's exact original input without changing its wallet or epoch.
    pub fn original_input(&self) -> &RetainedTurnInput { &self.retained }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), pack::PackError> {
        self.retained.validate().map_err(|_| pack::PackError::InvalidRetainedTurn("operation identity is absent"))?;
        self.actor.pack_encode(out).await;
        self.shard.pack_encode(out).await;
        self.budget.pack_encode(out).await?;
        self.retained.pack_encode(out).await?;self.loan.pack_encode(out).await;
        pack::write_vec(out, &self.envelopes, Envelope::pack_encode).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            actor: ActorId::pack_decode(bytes, pos).await?,
            shard: ShardId::pack_decode(bytes, pos).await?,
            budget: Budget::pack_decode(bytes, pos).await?,
            retained: RetainedTurnInput::pack_decode(bytes, pos).await?,
            loan:OriginalActorTurnLoan::pack_decode(bytes,pos).await?,
            envelopes: pack::read_vec(bytes, pos, "TurnGrant::envelopes", Envelope::pack_decode).await?,
        })
    }
}

/// ⏱️ How many envelopes a single granted turn drains from its actor's mailbox in one go.

/// ⏱️ Hierarchical deficit round-robin: level 1 fairness across plugins (packages), level 2 across
/// each plugin's own actors. Without level 1, a plugin with 50 actors would take 50x the share of a
/// plugin with one. Owns every actor's mailbox — `submit`/`tick`/`complete` are its only entry points.
#[derive(Clone, Debug, Default)]
pub struct Scheduler {
    actors: BTreeMap<ActorId, ScheduledActor>,
    plugin_deficit: HashMap<PackageId, i64>,
    plugin_order: Vec<PackageId>,
    plugin_cursor: usize,
    grants_per_tick: u32,
}

impl Scheduler {
    pub async fn new(grants_per_tick: u32) -> Self {
        Self { actors: BTreeMap::new(), plugin_deficit: HashMap::new(), plugin_order: Vec::new(), plugin_cursor: 0, grants_per_tick: grants_per_tick.max(1) }
    }

    pub async fn register_actor(&mut self, actor: ActorId, package: PackageId, lane: Lane, budget: Budget, shard: ShardId, retained: RetainedTurnInput,loan:OriginalActorTurnLoan) {
        self.register_actor_state(actor, package, lane, budget, shard, true, retained,loan).await;
    }

    async fn register_actor_state(&mut self, actor: ActorId, package: PackageId, lane: Lane, budget: Budget, shard: ShardId, active: bool, retained: RetainedTurnInput,loan:OriginalActorTurnLoan) {
        if !self.plugin_order.contains(&package) {
            self.plugin_order.push(package.clone());
        }
        self.actors.insert(actor, ScheduledActor { retained,loan,actual_received_work:0, issued: None, returned: false, actual_received_spent:Default::default(),refused_return:None,continuation:false,package, lane, budget, shard, mailbox: Mailbox::new(budget.mailbox_len).await, active, throttle: 1.0, deficit: 0 });
    }

    pub async fn unregister_actor(&mut self, actor: ActorId) {
        self.actors.remove(&actor);
    }

    pub async fn set_active(&mut self, actor: ActorId, active: bool) {
        if let Some(entry) = self.actors.get_mut(&actor) {
            entry.active = active;
        }
    }

    pub async fn set_shard(&mut self, actor: ActorId, shard: ShardId) {
        if let Some(entry) = self.actors.get_mut(&actor) {
            entry.shard = shard;
        }
    }

    pub async fn set_throttle(&mut self, actor: ActorId, factor: f32) {
        if let Some(entry) = self.actors.get_mut(&actor) {
            entry.throttle = factor;
        }
    }

    pub async fn mailbox_pressure(&self, actor: ActorId) -> Option<f32> {
        self.actors.get(&actor).map(|e| e.mailbox.pressure())
    }

    /// 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): the one piece of an actor's scheduling entry
    /// [`Kernel::actor_metrics_samples`] needs that [`ActorMeta`] doesn't itself carry — `lane` is
    /// fixed at [`Scheduler::register_actor`] and never changes, so this is a plain lookup.
    // 🚫️async: E1-adjacent pure accessor — its one consumer sits inside a sync `Iterator::map`
    // closure (`Kernel::actor_metrics_samples`) — see R9 residue shape 1.
    pub fn lane_of(&self, actor: ActorId) -> Option<Lane> {
        self.actors.get(&actor).map(|e| e.lane)
    }

    pub async fn submit(&mut self, envelope: Envelope) -> Backpressure {
        match self.actors.get_mut(&envelope.to) {
            Some(entry) => entry.mailbox.enqueue(envelope).await,
            None => Backpressure::Rejected,
        }
    }

    // 🚫️async: E1-adjacent pure computation — every consumer sits inside a sync `Iterator::map`/
    // `Option::map_or`/subtract-assign call (DRR quantum/deficit accounting), which cannot await —
    // see R9 residue shape 1.
    fn actor_weight(entry: &ScheduledActor) -> i64 {
        ((entry.lane.weight() as f32) * entry.throttle.max(0.05)).round().max(1.0) as i64
    }

    /// ⏱️ Pure DRR tick. `now_ms` gates deadline preemption and actor runnability (suspend/quarantine
    /// timers); all other state is internal and persists across calls.
    pub async fn tick(&mut self, now_ms: u64) -> Decision {
        let mut run = Vec::new();
        let mut granted_this_tick: BTreeSet<ActorId> = BTreeSet::new();
        let mut budget_left = self.grants_per_tick;

        //#region 🔖️DeadlinePreemption
        // 🚨️ Interactive envelopes past (or nearest) their deadline short-circuit ahead of DRR order.
        let mut overdue: Vec<(u64, ActorId)> = self.actors.iter().filter(|(_, e)| e.can_issue()).filter_map(|(id, e)| e.mailbox.earliest_deadline().filter(|d| *d <= now_ms).map(|d| (d, *id))).collect();
        overdue.sort();
        for (_, actor_id) in overdue {
            if budget_left == 0 {
                break;
            }
            if granted_this_tick.contains(&actor_id) {
                continue;
            }
            if let Some(grant) = self.drain_original_call(actor_id).await {
                run.push(grant);
                granted_this_tick.insert(actor_id);
                budget_left -= 1;
            }
        }
        //#endregion 🔖️DeadlinePreemption

        //#region 🔖️DrrRounds
        if budget_left > 0 && !self.plugin_order.is_empty() {
            let plugin_count = self.plugin_order.len();
            let mut idle_streak = 0usize;
            while budget_left > 0 && idle_streak < plugin_count {
                let package = self.plugin_order[self.plugin_cursor % plugin_count].clone();
                self.plugin_cursor = (self.plugin_cursor + 1) % plugin_count;
                let pending_actor_ids: Vec<ActorId> = self.actors.iter().filter(|(id, e)| e.package == package && e.can_issue() && !granted_this_tick.contains(*id)).map(|(id, _)| *id).collect();
                if pending_actor_ids.is_empty() {
                    self.plugin_deficit.insert(package, 0);
                    idle_streak += 1;
                    continue;
                }
                idle_streak = 0;
                let quantum = pending_actor_ids.iter().filter_map(|id| self.actors.get(id)).map(Self::actor_weight).max().unwrap_or(1);
                let mut deficit = *self.plugin_deficit.entry(package.clone()).or_insert(0);
                deficit += quantum;
                while deficit > 0 && budget_left > 0 {
                    let mut candidates: Vec<ActorId> = self.actors.iter().filter(|(id, e)| e.package == package && e.can_issue() && !granted_this_tick.contains(*id)).map(|(id, _)| *id).collect();
                    candidates.sort();
                    let Some(actor_id) = self.pick_level2(&candidates).await else { break };
                    let weight = self.actors.get(&actor_id).map_or(1, Self::actor_weight);
                    if let Some(grant) = self.drain_original_call(actor_id).await {
                        run.push(grant);
                        granted_this_tick.insert(actor_id);
                        budget_left -= 1;
                        deficit -= weight;
                    } else {
                        break;
                    }
                }
                self.plugin_deficit.insert(package.clone(), deficit);
            }
        }
        //#endregion 🔖️DrrRounds

        // ⏰️ Earliest deadline still sitting in any active actor's mailbox after this tick's grants
        // were drained — a hint for "check again by this time," not a guarantee of a grant then.
        let wake_at = self.actors.values().filter(|e| e.can_issue()).filter_map(|e| e.mailbox.earliest_deadline()).min();
        Decision { run, wake_at }
    }

    /// ⏱️ Level-2 DRR: rotate a per-actor deficit within the candidate set, weighted by lane (and
    /// throttle). Returns the next actor to grant, if any candidate has enough accrued deficit —
    /// otherwise grants everyone one quantum so the first call in a burst always makes progress.
    async fn pick_level2(&mut self, candidates: &[ActorId]) -> Option<ActorId> {
        if candidates.is_empty() {
            return None;
        }
        for &id in candidates {
            let weight = self.actors.get(&id).map_or(1, Self::actor_weight);
            let entry = self.actors.get_mut(&id).unwrap();
            entry.deficit += weight;
        }
        candidates.iter().copied().max_by_key(|id| self.actors.get(id).map_or(0, |e| e.deficit))
    }

    async fn drain_original_call(&mut self, actor_id: ActorId) -> Option<TurnGrant> {
        let entry = self.actors.get_mut(&actor_id)?;
        if entry.issued.is_some() || entry.refused_return.is_some() || (!entry.continuation&&entry.mailbox.is_empty()) || entry.retained.validate().is_err() || entry.loan.validate_for(entry.retained).is_err() || entry.loan.retained_policy.maximum_items==0 || entry.loan.work_fuel==0 || entry.retained.grant.maximum_items == 0 { return None; }
        let epoch = if entry.returned { entry.retained.epoch.checked_add(1)? } else { entry.retained.epoch };
        let retained = RetainedTurnInput { epoch, ..entry.retained };
        let mut envelopes = Vec::new();
        if entry.continuation{envelopes.push(Envelope{to:actor_id,from:Origin::Kernel,lane:entry.lane,seq:retained.epoch,deadline_ms:None,coalesce:None,cancel_of:None,payload:Payload::ContinueOriginalCall})}else if let Some(original)=entry.mailbox.pop_next().await{envelopes.push(original)}
        if envelopes.is_empty() {
            return None;
        }
        entry.retained = retained;
        entry.issued = Some(retained);
        entry.deficit -= Self::actor_weight(entry);
        Some(TurnGrant { actor: actor_id, shard: entry.shard, budget: entry.budget.scaled(entry.throttle).await, retained, loan:entry.loan,envelopes })
    }
}
//#endregion ⏱️Scheduler

//#region 🖼️Scene
/// 🖼️ One committed, immutable frame of a window's UI. `patches` is an opaque pack-encoded
/// `Vec<UiPatch>` blob — see the module seam docstring; `node_count` is the host-tracked total used
/// only for [`Budget::ui_nodes`] quota accounting.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct SceneSnapshot {
    pub revision: u64,
    pub committed_ms: u64,
    pub patches: Vec<u8>,
    pub node_count: u32,
}

impl SceneSnapshot {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.revision).await;
        pack::write_u64(out, self.committed_ms).await;
        pack::write_bytes(out, &self.patches).await;
        pack::write_u32(out, self.node_count).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            revision: pack::read_u64(bytes, pos, "SceneSnapshot::revision").await?,
            committed_ms: pack::read_u64(bytes, pos, "SceneSnapshot::committed_ms").await?,
            patches: pack::read_bytes(bytes, pos, "SceneSnapshot::patches").await?,
            node_count: pack::read_u32(bytes, pos, "SceneSnapshot::node_count").await?,
        })
    }
}

/// 🖼️ Per-window scene store: a builder accumulates patches between frame boundaries;
/// [`SceneStore::commit_frame`] publishes a new immutable snapshot once per boundary. If nothing
/// was pending, the previous snapshot is reused verbatim (same `Arc`, same revision) — the UI
/// thread never waits on a plugin that missed the deadline.
#[derive(Clone, Debug)]
pub struct SceneStore {
    current: Arc<SceneSnapshot>,
    pending: Vec<(ActorId, Vec<u8>, u32)>,
    pending_node_delta: u32,
}

impl Default for SceneStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneStore {
    // 🚫️async: E1 pure constructor consumed by `Default::default` (external trait impl above) —
    // see R9
    pub fn new() -> Self {
        Self { current: Arc::new(SceneSnapshot::default()), pending: Vec::new(), pending_node_delta: 0 }
    }

    pub async fn current(&self) -> Arc<SceneSnapshot> {
        self.current.clone()
    }

    /// 🖼️ Stages one actor's patch for the next commit, enforcing `budget.max_patch_bytes` and
    /// `budget.ui_nodes` host-side (no guest trust). Over quota: the node-count contribution is
    /// truncated to the remaining headroom (never counted past the ceiling) and `UiQuota` is
    /// returned so the caller can warn/truncate the typed patch itself.
    pub async fn apply_patch(&mut self, actor: ActorId, patch_bytes: Vec<u8>, node_delta: u32, budget: &Budget) -> Result<(), FailureSignal> {
        if patch_bytes.len() as u32 > budget.max_patch_bytes {
            return Err(FailureSignal::UiQuota);
        }
        let committed = self.current.node_count;
        let allowed = budget.ui_nodes.saturating_sub(committed + self.pending_node_delta);
        if node_delta > allowed {
            self.pending.push((actor, patch_bytes, allowed));
            self.pending_node_delta += allowed;
            return Err(FailureSignal::UiQuota);
        }
        self.pending.push((actor, patch_bytes, node_delta));
        self.pending_node_delta += node_delta;
        Ok(())
    }

    /// 🖼️ Publishes a new snapshot once per frame boundary. Empty pending set: returns the current
    /// snapshot unchanged (same revision) — reused because the frame had nothing new to show.
    pub async fn commit_frame(&mut self, now_ms: u64) -> Arc<SceneSnapshot> {
        if self.pending.is_empty() {
            return self.current.clone();
        }
        let mut merged = Vec::new();
        for (_, bytes, _) in &self.pending {
            merged.extend_from_slice(bytes);
        }
        let node_count = self.current.node_count + self.pending_node_delta;
        let snapshot = Arc::new(SceneSnapshot { revision: self.current.revision + 1, committed_ms: now_ms, patches: merged, node_count });
        self.pending.clear();
        self.pending_node_delta = 0;
        self.current = snapshot.clone();
        snapshot
    }
}
//#endregion 🖼️Scene

//#region 📈️Metrics
/// 📈️ Fixed-size ring of the last 64 per-turn wall-clock samples, for a p95 estimate cheap enough
/// to keep on every actor.
/// 📈️ Ring capacity for [`ActorMetrics`]'s wall-clock sample window — chosen as a plain `usize`
/// (not a const generic array) because serde's built-in array support tops out at 32 elements.
const WALL_US_RING_CAPACITY: usize = 64;

/// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): minimum recorded turns
/// before [`ActorMetrics::is_saturating`] will answer `true` — one slow turn (a cold cache, a GC
/// pause) must never flip an actor "hot"; only a SUSTAINED pattern does.
const SATURATION_MIN_TURNS: u64 = 2;

/// 🔥️ [`ActorMetrics::is_saturating`] fires once [`ActorMetrics::wall_us_p95`] reaches this percent
/// of the actor's OWN declared [`Budget::wall_ms`] — a p95, not the latest sample, so one fast turn
/// cannot mask a sustained pattern of near-budget turns.
const SATURATION_THRESHOLD_PERCENT: u64 = 70;

#[derive(Clone, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct ActorMetrics {
    pub turns: u64,
    pub fuel_total: u64,
    pub wall_us_total: u64,
    wall_us_ring: Vec<u32>,
    wall_us_ring_len: u8,
    wall_us_ring_pos: u8,
    pub memory_bytes: u64,
    pub mailbox_len: u16,
    pub mailbox_lag_ms: u32,
    pub coalesced: u64,
    pub dropped: u64,
    pub traps: u32,
    pub restarts: u32,
    pub stage: FailureStage,
    pub shard: ShardId,
}

impl std::fmt::Debug for ActorMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActorMetrics").field("turns", &self.turns).field("wall_us_p95", &self.wall_us_p95()).field("stage", &self.stage).finish()
    }
}

impl PartialEq for ActorMetrics {
    fn eq(&self, other: &Self) -> bool {
        self.turns == other.turns
            && self.fuel_total == other.fuel_total
            && self.wall_us_total == other.wall_us_total
            && self.wall_us_ring == other.wall_us_ring
            && self.memory_bytes == other.memory_bytes
            && self.stage == other.stage
            && self.shard == other.shard
    }
}

impl Default for ActorMetrics {
    fn default() -> Self {
        Self {
            turns: 0,
            fuel_total: 0,
            wall_us_total: 0,
            wall_us_ring: vec![0; WALL_US_RING_CAPACITY],
            wall_us_ring_len: 0,
            wall_us_ring_pos: 0,
            memory_bytes: 0,
            mailbox_len: 0,
            mailbox_lag_ms: 0,
            coalesced: 0,
            dropped: 0,
            traps: 0,
            restarts: 0,
            stage: FailureStage::Healthy,
            shard: ShardId(0),
        }
    }
}

impl ActorMetrics {
    pub async fn record_turn(&mut self, usage: &Usage) {
        self.turns += 1;
        self.fuel_total += usage.fuel;
        self.wall_us_total += usage.wall_us;
        self.wall_us_ring[self.wall_us_ring_pos as usize] = usage.wall_us.min(u32::MAX as u64) as u32;
        self.wall_us_ring_pos = (self.wall_us_ring_pos + 1) % WALL_US_RING_CAPACITY as u8;
        self.wall_us_ring_len = (self.wall_us_ring_len + 1).min(WALL_US_RING_CAPACITY as u8);
        self.memory_bytes = usage.memory_bytes;
    }

    // 🚫️async: E1 pure computation consumed by `Debug::fmt` (external trait impl above) — see R9
    pub fn wall_us_p95(&self) -> u32 {
        if self.wall_us_ring_len == 0 {
            return 0;
        }
        let mut samples: Vec<u32> = self.wall_us_ring[..self.wall_us_ring_len as usize].to_vec();
        samples.sort_unstable();
        let idx = ((samples.len() as f32) * 0.95).floor() as usize;
        samples[idx.min(samples.len() - 1)]
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): observed-behaviour
    /// CPU-saturation signal — purely `record_turn`'s own tracked `wall_us_p95` against `budget`'s
    /// OWN declared `wall_ms` ceiling, never a fixture/profile name (this crate has no such concept
    /// and must not gain one — see the module doc's purity rule). Used by
    /// `Kernel::saturated_shards`/`ShardTable::pin_avoiding` to keep freshly-activated interactive
    /// actors off a shard a background actor is already monopolizing.
    pub async fn is_saturating(&self, budget: &Budget) -> bool {
        if self.turns < SATURATION_MIN_TURNS {
            return false;
        }
        let budget_us = (budget.wall_ms as u64) * 1000;
        if budget_us == 0 {
            return false;
        }
        (self.wall_us_p95() as u64) * 100 >= budget_us * SATURATION_THRESHOLD_PERCENT
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.turns).await;
        pack::write_u64(out, self.fuel_total).await;
        pack::write_u64(out, self.wall_us_total).await;
        for sample in &self.wall_us_ring {
            pack::write_u32(out, *sample).await;
        }
        pack::write_u8(out, self.wall_us_ring_len).await;
        pack::write_u8(out, self.wall_us_ring_pos).await;
        pack::write_u64(out, self.memory_bytes).await;
        pack::write_u16(out, self.mailbox_len).await;
        pack::write_u32(out, self.mailbox_lag_ms).await;
        pack::write_u64(out, self.coalesced).await;
        pack::write_u64(out, self.dropped).await;
        pack::write_u32(out, self.traps).await;
        pack::write_u32(out, self.restarts).await;
        self.stage.pack_encode(out).await;
        self.shard.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let turns = pack::read_u64(bytes, pos, "ActorMetrics::turns").await?;
        let fuel_total = pack::read_u64(bytes, pos, "ActorMetrics::fuel_total").await?;
        let wall_us_total = pack::read_u64(bytes, pos, "ActorMetrics::wall_us_total").await?;
        let mut wall_us_ring = Vec::with_capacity(WALL_US_RING_CAPACITY);
        for _ in 0..WALL_US_RING_CAPACITY {
            wall_us_ring.push(pack::read_u32(bytes, pos, "ActorMetrics::wall_us_ring").await?);
        }
        let wall_us_ring_len = pack::read_u8(bytes, pos, "ActorMetrics::wall_us_ring_len").await?;
        let wall_us_ring_pos = pack::read_u8(bytes, pos, "ActorMetrics::wall_us_ring_pos").await?;
        Ok(Self {
            turns,
            fuel_total,
            wall_us_total,
            wall_us_ring,
            wall_us_ring_len,
            wall_us_ring_pos,
            memory_bytes: pack::read_u64(bytes, pos, "ActorMetrics::memory_bytes").await?,
            mailbox_len: pack::read_u16(bytes, pos, "ActorMetrics::mailbox_len").await?,
            mailbox_lag_ms: pack::read_u32(bytes, pos, "ActorMetrics::mailbox_lag_ms").await?,
            coalesced: pack::read_u64(bytes, pos, "ActorMetrics::coalesced").await?,
            dropped: pack::read_u64(bytes, pos, "ActorMetrics::dropped").await?,
            traps: pack::read_u32(bytes, pos, "ActorMetrics::traps").await?,
            restarts: pack::read_u32(bytes, pos, "ActorMetrics::restarts").await?,
            stage: FailureStage::pack_decode(bytes, pos).await?,
            shard: ShardId::pack_decode(bytes, pos).await?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct ShardMetrics {
    pub actors: u32,
    pub busy_ratio: f32,
    pub heartbeat_age_ms: u32,
}

impl ShardMetrics {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u32(out, self.actors).await;
        pack::write_f32(out, self.busy_ratio).await;
        pack::write_u32(out, self.heartbeat_age_ms).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { actors: pack::read_u32(bytes, pos, "ShardMetrics::actors").await?, busy_ratio: pack::read_f32(bytes, pos, "ShardMetrics::busy_ratio").await?, heartbeat_age_ms: pack::read_u32(bytes, pos, "ShardMetrics::heartbeat_age_ms").await? })
    }
}

/// 📈️ Sampled by `Kernel::metrics()`; the host publishes this as bus topic `os.runtime.metrics` at 2Hz.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct KernelMetrics {
    pub actors: u32,
    pub shards: u32,
    pub packages: u32,
}

impl KernelMetrics {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u32(out, self.actors).await;
        pack::write_u32(out, self.shards).await;
        pack::write_u32(out, self.packages).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { actors: pack::read_u32(bytes, pos, "KernelMetrics::actors").await?, shards: pack::read_u32(bytes, pos, "KernelMetrics::shards").await?, packages: pack::read_u32(bytes, pos, "KernelMetrics::packages").await? })
    }
}

//#region 🗒️RuntimeMetricsSnapshot
/// 🗒️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one live actor's row for the `os.runtime.metrics`
/// publication — [`ActorMetrics`] joined with the kernel-level bookkeeping ([`PackageId`]/[`Lane`]/
/// [`ActorStatus`]) it doesn't itself carry. Built by [`Kernel::actor_metrics_samples`].
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct ActorMetricsSample {
    pub id: ActorId,
    pub package: PackageId,
    pub lane: Lane,
    pub status: ActorStatus,
    pub metrics: ActorMetrics,
}

impl ActorMetricsSample {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.id.pack_encode(out).await;
        self.package.pack_encode(out).await;
        self.lane.pack_encode(out).await;
        self.status.pack_encode(out).await;
        self.metrics.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            id: ActorId::pack_decode(bytes, pos).await?,
            package: PackageId::pack_decode(bytes, pos).await?,
            lane: Lane::pack_decode(bytes, pos).await?,
            status: ActorStatus::pack_decode(bytes, pos).await?,
            metrics: ActorMetrics::pack_decode(bytes, pos).await?,
        })
    }
}

/// 🗒️ One shard's row for the `os.runtime.metrics` publication. `metrics.heartbeat_age_ms` is left at
/// its `Default` (0) by [`Kernel::shard_metrics_samples`] — the pure crate has no clock/transport of
/// its own (`important.md`'s purity rule), so a host overlays the real value from its own
/// `ShardTransport::heartbeat()` reading before publishing.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct ShardMetricsSample {
    pub shard: ShardId,
    pub metrics: ShardMetrics,
}

impl ShardMetricsSample {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.shard.pack_encode(out).await;
        self.metrics.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { shard: ShardId::pack_decode(bytes, pos).await?, metrics: ShardMetrics::pack_decode(bytes, pos).await? })
    }
}

/// 🗒️ The exact payload [`KernelMetrics`]'s own doc comment promises: "the host publishes this as bus
/// topic `os.runtime.metrics` at 2Hz." Built by [`Kernel::runtime_metrics_snapshot`], which takes
/// `sampled_at_ms` as a parameter rather than reading a clock — the crate core has none (transports
/// and time are injected, per this crate's own `Cargo.toml` description).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct RuntimeMetricsSnapshot {
    pub kernel: KernelMetrics,
    pub actors: Vec<ActorMetricsSample>,
    pub shards: Vec<ShardMetricsSample>,
    pub sampled_at_ms: u64,
}

impl RuntimeMetricsSnapshot {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.kernel.pack_encode(out).await;
        pack::write_vec(out, &self.actors, ActorMetricsSample::pack_encode).await;
        pack::write_vec(out, &self.shards, ShardMetricsSample::pack_encode).await;
        pack::write_u64(out, self.sampled_at_ms).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            kernel: KernelMetrics::pack_decode(bytes, pos).await?,
            actors: pack::read_vec(bytes, pos, "RuntimeMetricsSnapshot::actors", ActorMetricsSample::pack_decode).await?,
            shards: pack::read_vec(bytes, pos, "RuntimeMetricsSnapshot::shards", ShardMetricsSample::pack_decode).await?,
            sampled_at_ms: pack::read_u64(bytes, pos, "RuntimeMetricsSnapshot::sampled_at_ms").await?,
        })
    }
}

/// ⏱️ 2Hz, matching [`KernelMetrics`]'s doc comment. A `const`, not a config knob — no caller of
/// [`runtime_metrics_due`] currently needs a different cadence.
pub const RUNTIME_METRICS_PUBLISH_INTERVAL_MS: u64 = 500;

/// ⏱️ Pure cadence gate for a host's `os.runtime.metrics` publish loop — clock injected via `now_ms`
/// (never read internally), so a host can drive this off whatever tick source it already has (a
/// native thread's loop timer, a web `requestAnimationFrame`/`setInterval`). `None` (never published
/// yet) is always due.
pub async fn runtime_metrics_due(last_published_ms: Option<u64>, now_ms: u64) -> bool {
    match last_published_ms {
        None => true,
        Some(last) => now_ms.saturating_sub(last) >= RUNTIME_METRICS_PUBLISH_INTERVAL_MS,
    }
}
//#endregion 🗒️RuntimeMetricsSnapshot
//#endregion 📈️Metrics

//#region 🚚️ShardTransport
/// 🚚️ Injected duplex byte transport to one shard. `ThreadTransport` is the only impl living in
/// this crate (native, `std::sync::mpsc`); `WorkerTransport` (postMessage) and `ProcessTransport`
/// (stdio) are host-supplied — all three carry the same `Envelope`/`TurnResult` pack encoding.
pub trait ShardTransport: Send {
    async fn send(&self, bytes: &[u8]);
    async fn recv(&self) -> Option<Vec<u8>>;
    async fn heartbeat(&self) -> u64;
    async fn kill(&self);
}

//#region 🔖️ThreadTransport
#[cfg(not(target_arch = "wasm32"))]
mod thread_transport {
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{Arc, Mutex};

    use super::ShardTransport;

    /// 🚚️ Native duplex transport over `std::sync::mpsc` — one end held by the kernel, the mirror
    /// end by the shard loop. Gated to non-wasm32 only; the pure crate core never spawns threads
    /// itself (the shard loop that reads/writes the mirror end lives in `plugin/host`, packet B1).
    pub struct ThreadTransport {
        outbound: Sender<Vec<u8>>,
        inbound: Mutex<Receiver<Vec<u8>>>,
        heartbeat_ms: Arc<AtomicU64>,
        killed: Arc<AtomicBool>,
    }

    impl ThreadTransport {
        /// 🔗️ Builds both ends of one duplex link, sharing the heartbeat/kill flags.
        pub async fn new_pair() -> (ThreadTransport, ThreadTransport) {
            let (tx_a, rx_a) = std::sync::mpsc::channel();
            let (tx_b, rx_b) = std::sync::mpsc::channel();
            let heartbeat_ms = Arc::new(AtomicU64::new(0));
            let killed = Arc::new(AtomicBool::new(false));
            let side_a = ThreadTransport { outbound: tx_a, inbound: Mutex::new(rx_b), heartbeat_ms: heartbeat_ms.clone(), killed: killed.clone() };
            let side_b = ThreadTransport { outbound: tx_b, inbound: Mutex::new(rx_a), heartbeat_ms, killed };
            (side_a, side_b)
        }

        /// 💓️ Records a heartbeat timestamp — called by whichever side is standing in for the
        /// shard loop's `Atomics.store` in the real web transport.
        pub async fn beat(&self, now_ms: u64) {
            self.heartbeat_ms.store(now_ms, Ordering::SeqCst);
        }

        /// ⏳️ Blocking receive with a timeout — `ShardExecutor` (terra-shard-grants,
        /// `🖥️host/🧵️shard/🧵️executor/🦀️.rs`) parks here between pumps instead of a caller-driven
        /// busy-poll loop. `mpsc::Receiver::recv_timeout` blocks only the CALLING thread; it spawns
        /// no thread of its own, so the purity grep this crate's core is verified against (no
        /// `std::thread` in `🦀️.rs`) still holds. Not part of the [`ShardTransport`]
        /// trait — `WorkerTransport`/`ProcessTransport` have their own, different ways of parking
        /// (a JS event loop, a blocking `read`), so this stays a `ThreadTransport`-only inherent
        /// method, the same shape `Self::beat` already uses for its own extra, non-trait surface.
        pub async fn recv_deadline(&self, timeout: std::time::Duration) -> Option<Vec<u8>> {
            if self.killed.load(Ordering::SeqCst) {
                return None;
            }
            self.inbound.lock().ok().and_then(|rx| rx.recv_timeout(timeout).ok())
        }

        /// 🤝️ Takes one currently buffered frame without parking the caller.
        pub fn try_recv_now(&self) -> Option<Vec<u8>> {
            if self.killed.load(Ordering::SeqCst) {
                return None;
            }
            self.inbound.lock().ok().and_then(|rx| rx.try_recv().ok())
        }

        /// 🛡️ Attempts one owned send without suspension and returns the exact frame when the
        /// transport is terminal or disconnected.
        pub fn send_now(&self, bytes: Vec<u8>) -> Result<(), Vec<u8>> {
            if self.killed.load(Ordering::SeqCst) {
                return Err(bytes);
            }
            self.outbound.send(bytes).map_err(|error| error.0)
        }
    }

    impl ShardTransport for ThreadTransport {
        async fn send(&self, bytes: &[u8]) {
            let _ = self.send_now(bytes.to_vec());
        }

        async fn recv(&self) -> Option<Vec<u8>> {
            self.try_recv_now()
        }

        async fn heartbeat(&self) -> u64 {
            self.heartbeat_ms.load(Ordering::SeqCst)
        }

        async fn kill(&self) {
            self.killed.store(true, Ordering::SeqCst);
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub use thread_transport::ThreadTransport;
//#endregion 🔖️ThreadTransport
//#endregion 🚚️ShardTransport

//#region 🏛️Kernel
/// 🏛️ What activated an actor.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActivationEvent {
    Manual,
    WindowOpen { window: WindowId },
    Restart,
}

/// 🚨️ Errors the [`Kernel`] façade's fallible operations return.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelError {
    UnknownActor,
    NoExclusiveShard,
    InvalidTransition,
    TurnNotIssued,
    InvalidRetainedReceipt,
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownActor => formatter.write_str("unknown actor"),
            Self::NoExclusiveShard => formatter.write_str("no exclusive shard available"),
            Self::InvalidTransition => formatter.write_str("invalid status transition"),
            Self::TurnNotIssued => formatter.write_str("actor turn was not issued"),
            Self::InvalidRetainedReceipt => formatter.write_str("original actor retained receipt does not match issued authority"),
        }
    }
}

impl std::error::Error for KernelError {}

/// 🏛️ Per-actor bookkeeping the [`Scheduler`] doesn't need to see: kind, capabilities, status,
/// failure ladder, metrics, and which window (if any) its scene patches target.
#[derive(Clone, Debug)]
struct ActorMeta {
    reservation: Option<Arc<activation::ReservationAuthority>>,
    transport_key: Option<activation::ActorShardKey>,
    kind: ActorKind,
    package: PackageId,
    capabilities: Vec<CapabilityGrant>,
    budget: Budget,
    status: ActorStatus,
    failure: FailureState,
    metrics: ActorMetrics,
    window: Option<WindowId>,
}

/// 🏛️ The one-implementation, three-host runtime façade: `submit`/`tick`/`complete`/`activate`/
/// `suspend`/`resume`/`request_exclusive`/`commit_frame`/`metrics`. Composes [`Scheduler`],
/// [`ShardTable`], per-window [`SceneStore`]s and the failure ladder over one actor registry.
#[derive(Debug)]
pub struct Kernel {
    scheduler: Scheduler,
    shards: ShardTable,
    scenes: HashMap<WindowId, SceneStore>,
    actors: HashMap<ActorId, ActorMeta>,
    next_ordinal: HashMap<PackageId, u32>,
    /// 🔗️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): explicit parent→children
    /// edge table for extension cascade — see `//#region 🔗️ExtensionActivation` below. Kept explicit
    /// rather than re-derived from `ActorKind::Extension.plugin` on every cascade so that (a) cascade is
    /// O(children) instead of an O(actors) scan, and (b) it survives multiple *instances* of one plugin
    /// (several `PluginApp` actors sharing a `PackageId`, each with its own extension subtree) without
    /// the `plugin: PackageId` field alone being able to disambiguate which instance a child belongs to.
    links: HashMap<ActorId, Vec<ActorId>>,
}

impl Kernel {
    pub async fn new(shard_kind: ShardKind, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32) -> Self {
        Self { scheduler: Scheduler::new(grants_per_tick).await, shards: ShardTable::new(shard_kind, shard_count, exclusive_reserve).await, scenes: HashMap::new(), actors: HashMap::new(), next_ordinal: HashMap::new(), links: HashMap::new() }
    }

    /// ▶️ Instantiates a fresh actor for `kind` under `package`/`lane`, pins it to a shard, and
    /// returns its freshly minted [`ActorId`] (generation 0).
    ///
    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): `Lane::Interactive`
    /// alone pays [`Self::saturated_shards`]'s avoidance cost, via [`ShardTable::pin_avoiding`] rather
    /// than [`ShardTable::pin`] — every other lane pins exactly as before this packet (see
    /// `pin_avoiding`'s own doc for why: a wholesale avoid-saturated-shards policy for EVERY lane
    /// would just relocate the count imbalance `pin`'s own budget-3 guarantee depends on, not fix
    /// interactive latency).
    pub async fn activate(&mut self, package: PackageId, plugin_ordinal: u16, kind: ActorKind, lane: Lane, window: Option<WindowId>, _event: ActivationEvent, retained: RetainedTurnInput,loan:OriginalActorTurnLoan) -> ActorId {
        let ordinal = self.next_ordinal.entry(package.clone()).or_insert(0);
        let id = ActorId::new(plugin_ordinal, kind.tag().await, *ordinal, 0).await;
        *ordinal += 1;
        let budget = lane_defaults::budget_for(lane);
        let shard = if lane == Lane::Interactive {
            let avoid = self.saturated_shards().await;
            self.shards.pin_avoiding(id, &avoid).await
        } else {
            self.shards.pin(id).await
        };
        self.scheduler.register_actor(id, package.clone(), lane, budget, shard, retained,loan).await;
        self.actors.insert(id, ActorMeta { reservation: None, transport_key: None, kind, package, capabilities: Vec::new(), budget, status: ActorStatus::Activating, failure: FailureState::new(), metrics: ActorMetrics::default(), window });
        id
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): every shard
    /// currently hosting at least one actor [`ActorMetrics::is_saturating`] judges CPU-saturating —
    /// purely from this kernel's own tracked state (turns already completed via [`Self::complete`]),
    /// never a fixture/profile name. `activate`'s own doc explains why only `Lane::Interactive`
    /// consults this.
    async fn saturated_shards(&self) -> BTreeSet<ShardId> {
        let mut out = BTreeSet::new();
        for (id, meta) in &self.actors {
            if meta.metrics.is_saturating(&meta.budget).await {
                if let Some(shard) = self.shards.shard_of(*id).await {
                    out.insert(shard);
                }
            }
        }
        out
    }

    pub async fn submit(&mut self, envelope: &Envelope) -> Backpressure {
        let backpressure = self.scheduler.submit(envelope.clone()).await;
        if let Some(meta) = self.actors.get_mut(&envelope.to) {
            match backpressure {
                Backpressure::Coalesced => meta.metrics.coalesced += 1,
                Backpressure::Dropped { .. } => meta.metrics.dropped += 1,
                _ => {}
            }
            if let Some(pressure) = self.scheduler.mailbox_pressure(envelope.to).await {
                meta.metrics.mailbox_len = (pressure * meta.budget.mailbox_len as f32) as u16;
            }
        }
        backpressure
    }

    pub async fn tick(&mut self, now_ms: u64) -> Decision {
        self.scheduler.tick(now_ms).await
    }

    /// 📮️ Borrows the scheduler's current original mailbox count without constructing a snapshot.
    pub fn mailbox_len(&self, actor: ActorId) -> Option<u16> {
        self.scheduler.actors.get(&actor).map(|entry| entry.mailbox.len())
    }

    /// 🎟️ Borrows the original remaining input independently of scheduling ceilings.
    pub fn retained_turn(&self, actor: ActorId) -> Option<RetainedTurnInput> {
        self.scheduler.actors.get(&actor).map(|entry| entry.retained)
    }

    pub fn retained_turn_loan(&self,actor:ActorId)->Option<OriginalActorTurnLoan>{self.scheduler.actors.get(&actor).map(|entry|entry.loan)}
    pub fn original_call_work_received(&self,actor:ActorId)->Option<u128>{self.scheduler.actors.get(&actor).map(|entry|entry.actual_received_work)}

    /// 📥️ Adopts one actual raw export receipt before another input or semantic result is admitted.
    pub fn receive_original_call(&mut self,actor:ActorId,returned:retained_turn::OriginalActorGuestReturn,continuation:bool)->Result<RetainedTurnReceipt,KernelError>{
        let entry=self.scheduler.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        if entry.refused_return.is_some(){return Err(KernelError::InvalidRetainedReceipt)}
        let issued=entry.issued.ok_or(KernelError::TurnNotIssued)?;
        match entry.actual_received_spent.checked_add(returned.spent){Ok(spent)=>entry.actual_received_spent=spent,Err(_)=>{entry.refused_return=Some(returned);return Err(KernelError::InvalidRetainedReceipt)}}
        entry.actual_received_work=match entry.actual_received_work.checked_add(u128::from(returned.work_units_spent)){Some(work)=>work,None=>{entry.refused_return=Some(returned);return Err(KernelError::InvalidRetainedReceipt)}};
        let receipt=match issued.return_original(returned.spent,returned.work_units_spent){Ok(receipt)if returned.input==issued&&returned.claimed_remaining.is_none_or(|remaining|remaining==receipt.remaining)&&returned.claimed_remaining_fuel.is_none_or(|remaining|remaining==receipt.remaining_fuel)=>receipt,_=>{entry.refused_return=Some(returned);return Err(KernelError::InvalidRetainedReceipt)}};
        let loan=match entry.loan.remaining(returned.spent,returned.work_units_spent){Ok(loan)=>loan,Err(_)=>{entry.refused_return=Some(returned);return Err(KernelError::InvalidRetainedReceipt)}};
        entry.loan=loan;entry.retained=RetainedTurnInput{grant:receipt.remaining,work:OriginalActorTurnWork{fuel:receipt.remaining_fuel,..issued.work},..issued};entry.issued=None;entry.returned=true;entry.continuation=continuation;Ok(receipt)
    }

    /// 🧾️ Borrows actual raw spending and any refused owner before a scheduler continuation.
    pub fn original_call_receiving(&self,actor:ActorId)->Option<(semio_framework_value::RetainedCloneProgress,Option<&retained_turn::OriginalActorGuestReturn>)>{self.scheduler.actors.get(&actor).map(|entry|(entry.actual_received_spent,entry.refused_return.as_ref()))}

    /// ✅️ Records a turn's result against its actor: usage metrics, failure-ladder update (clean
    /// turn vs. `Faulted`), and status transition. Returns the escalation the caller (host) must
    /// act on for `Trapped`/`Quarantined` outcomes.
    pub async fn complete(&mut self, actor: ActorId, result: &TurnResult, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        self.receive_original_call(actor,retained_turn::OriginalActorGuestReturn{input:result.retained_receipt.input,spent:result.retained_receipt.spent,work_units_spent:result.retained_receipt.work_units_spent,claimed_remaining:Some(result.retained_receipt.remaining),claimed_remaining_fuel:Some(result.retained_receipt.remaining_fuel)},matches!(result.status,TurnStatus::MoreWork))?;
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        if meta.reservation.is_some() {
            return Err(KernelError::InvalidTransition);
        }
        meta.metrics.record_turn(&result.usage).await;
        let escalation = match &result.status {
            TurnStatus::Faulted { detail } => {
                let detail_string = String::from_utf8_lossy(detail).into_owned();
                meta.failure.on_signal(&FailureSignal::Trap { detail: detail_string }, Lane::Interactive, now_ms).await
            }
            _ => {
                meta.failure.on_clean_turn(now_ms).await;
                FailureEscalation::None
            }
        };
        meta.status = match &meta.failure.stage {
            FailureStage::Trapped { restarts } => {
                meta.metrics.traps += 1;
                meta.metrics.restarts = *restarts;
                ActorStatus::Trapped
            }
            FailureStage::Quarantined { .. } => ActorStatus::Quarantined,
            FailureStage::Disabled => ActorStatus::Disabled,
            FailureStage::Cancelled => ActorStatus::Draining,
            _ => ActorStatus::Active,
        };
        meta.metrics.stage = meta.failure.stage;
        let throttle = meta.failure.throttle_factor();
        let package = meta.package.clone();
        self.scheduler.set_throttle(actor, throttle.await).await;
        if let FailureEscalation::QuarantinePackage = escalation {
            self.quarantine_package(&package, now_ms).await;
        }
        Ok(escalation)
    }

    /// 🚑️ Package-wide quarantine: every actor sharing `package` is stopped, regardless of its own
    /// individual failure history.
    async fn quarantine_package(&mut self, package: &PackageId, now_ms: u64) {
        let affected: Vec<ActorId> = self.actors.iter().filter(|(_, m)| &m.package == package).map(|(id, _)| *id).collect();
        for actor in affected {
            if let Some(meta) = self.actors.get_mut(&actor) {
                meta.failure.stage = FailureStage::Quarantined { until: now_ms + quarantine_duration_ms(meta.failure.restart_count.max(1)).await };
                meta.status = ActorStatus::Quarantined;
                meta.metrics.stage = meta.failure.stage;
            }
            self.scheduler.set_active(actor, false).await;
        }
    }

    pub async fn suspend(&mut self, actor: ActorId, checkpoint: Option<Vec<u8>>) -> Result<(), KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        if meta.reservation.is_some() {
            return Err(KernelError::InvalidTransition);
        }
        meta.status = ActorStatus::Suspended { checkpoint };
        self.scheduler.set_active(actor, false).await;
        Ok(())
    }

    pub async fn resume(&mut self, actor: ActorId) -> Result<(), KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        if meta.reservation.is_some() {
            return Err(KernelError::InvalidTransition);
        }
        match meta.status {
            ActorStatus::Suspended { .. } => {
                meta.status = ActorStatus::Active;
                self.scheduler.set_active(actor, true).await;
                Ok(())
            }
            _ => Err(KernelError::InvalidTransition),
        }
    }

    pub async fn request_exclusive(&mut self, actor: ActorId) -> Result<ShardId, KernelError> {
        if !self.actors.contains_key(&actor) {
            return Err(KernelError::UnknownActor);
        }
        if self.actors.get(&actor).is_some_and(|meta| meta.reservation.is_some() || meta.transport_key.is_some()) {
            return Err(KernelError::InvalidTransition);
        }
        let shard = self.shards.request_exclusive(actor).await.ok_or(KernelError::NoExclusiveShard)?;
        self.scheduler.set_shard(actor, shard).await;
        Ok(shard)
    }

    pub async fn release_exclusive(&mut self, actor: ActorId) {
        if self.actors.get(&actor).is_some_and(|meta| meta.reservation.is_some() || meta.transport_key.is_some()) {
            return;
        }
        self.shards.release_exclusive(actor).await;
        if let Some(shard) = self.shards.shard_of(actor).await {
            self.scheduler.set_shard(actor, shard).await;
        }
    }

    /// 🖼️ Routes an actor's `TurnResult.ui_patches`/estimated node-delta into its window's
    /// [`SceneStore`], enforcing quota. No-op (Ok) when the actor has no associated window.
    pub async fn apply_scene_patch(&mut self, actor: ActorId, patch_bytes: Vec<u8>, node_delta: u32) -> Result<(), FailureSignal> {
        let Some(meta) = self.actors.get(&actor) else { return Ok(()) };
        let Some(window) = meta.window else { return Ok(()) };
        let budget = meta.budget;
        self.scenes.entry(window).or_default().apply_patch(actor, patch_bytes, node_delta, &budget).await
    }

    /// 🖼️ Commits every window's pending patches at a frame boundary.
    pub async fn commit_frame(&mut self, now_ms: u64) -> HashMap<WindowId, Arc<SceneSnapshot>> {
        let mut out = HashMap::with_capacity(self.scenes.len());
        for (window, store) in self.scenes.iter_mut() {
            out.insert(*window, store.commit_frame(now_ms).await);
        }
        out
    }

    pub async fn scene_of(&self, window: WindowId) -> Option<Arc<SceneSnapshot>> {
        match self.scenes.get(&window) {
            Some(s) => Some(s.current().await),
            None => None,
        }
    }

    pub async fn metrics(&self) -> KernelMetrics {
        let packages: BTreeSet<&PackageId> = self.actors.values().map(|m| &m.package).collect();
        KernelMetrics { actors: self.actors.len() as u32, shards: self.shards.shard_count().await as u32, packages: packages.len() as u32 }
    }

    /// 🗂️ Assembles a full [`ActorRecord`] snapshot from the scheduler's live entry and this
    /// actor's own bookkeeping — a convenience for hosts/tests, not itself part of the required façade.
    pub async fn actor_record(&self, actor: ActorId) -> Option<ActorRecord> {
        let meta = self.actors.get(&actor)?;
        let shard = self.shards.shard_of(actor).await.unwrap_or(ShardId(0));
        let mailbox = Mailbox::new(meta.budget.mailbox_len).await;
        Some(ActorRecord {
            id: actor,
            kind: meta.kind.clone(),
            package: meta.package.clone(),
            shard,
            capabilities: meta.capabilities.clone(),
            budget: meta.budget,
            mailbox,
            status: meta.status.clone(),
            failure: meta.failure.clone(),
            metrics: meta.metrics.clone(),
        })
    }

    pub async fn actor_status(&self, actor: ActorId) -> Option<&ActorStatus> {
        self.actors.get(&actor).map(|m| &m.status)
    }

    pub async fn actor_failure(&self, actor: ActorId) -> Option<&FailureState> {
        self.actors.get(&actor).map(|m| &m.failure)
    }

    /// 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one [`ActorMetricsSample`] per live actor —
    /// the per-actor rows of the `os.runtime.metrics` publication (`KernelMetrics`'s own doc comment).
    pub async fn actor_metrics_samples(&self) -> Vec<ActorMetricsSample> {
        self.actors.iter().map(|(id, meta)| ActorMetricsSample { id: *id, package: meta.package.clone(), lane: self.scheduler.lane_of(*id).unwrap_or(Lane::Background), status: meta.status.clone(), metrics: meta.metrics.clone() }).collect()
    }

    /// 📈️ One [`ShardMetricsSample`] per shard holding at least one actor. `busy_ratio` is the
    /// fraction of that shard's actors currently [`ActorStatus::Active`] — computable purely from
    /// kernel-tracked status, no clock needed. `heartbeat_age_ms` is left at 0; see
    /// [`ShardMetricsSample`]'s own doc comment for why a host must overlay it.
    pub async fn shard_metrics_samples(&self) -> Vec<ShardMetricsSample> {
        let mut per_shard: HashMap<ShardId, (u32, u32)> = HashMap::new();
        for (id, meta) in &self.actors {
            let shard = self.shards.shard_of(*id).await.unwrap_or(ShardId(0));
            let entry = per_shard.entry(shard).or_insert((0, 0));
            entry.1 += 1;
            if meta.status == ActorStatus::Active {
                entry.0 += 1;
            }
        }
        per_shard.into_iter().map(|(shard, (active, total))| ShardMetricsSample { shard, metrics: ShardMetrics { actors: total, busy_ratio: if total > 0 { active as f32 / total as f32 } else { 0.0 }, heartbeat_age_ms: 0 } }).collect()
    }

    /// 📈️ Assembles the full `os.runtime.metrics` payload — [`Kernel::metrics`] plus every actor's and
    /// shard's sample — for a host to pack-encode and publish. `sampled_at_ms` is the caller's clock
    /// reading, never read internally (this crate's purity rule: transports and time are injected).
    pub async fn runtime_metrics_snapshot(&self, sampled_at_ms: u64) -> RuntimeMetricsSnapshot {
        RuntimeMetricsSnapshot { kernel: self.metrics().await, actors: self.actor_metrics_samples().await, shards: self.shard_metrics_samples().await, sampled_at_ms }
    }

    //#region 🔗️ExtensionActivation
    /// 📌️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): [`Self::activate`],
    /// widened for extension activation — pins to an EXACT `shard` via [`ShardTable::pin_to`] instead
    /// of the least-loaded heuristic (so an extension always lands on its parent's shard — see
    /// module design doc M6), and computes this actor's initial `capabilities` via
    /// [`intersect_capabilities`] against `parent`'s own already-granted set when `parent` is
    /// `Some` — the never-escalate-past-the-parent security property. `parent` is `None` for a
    /// top-level activation with a caller-supplied grant set (e.g. a `PluginApp` whose capabilities
    /// the host's own broker already resolved); passing `Some` is what makes this the extension path.
    /// This method does **not** itself record the [`Self::link_extension`] parent→child edge — call
    /// both from the host cascade so the two concerns (placement/capability vs. cascade topology)
    /// stay independently testable.
    #[allow(clippy::too_many_arguments)]
    pub async fn activate_pinned(
        &mut self,
        package: PackageId,
        plugin_ordinal: u16,
        kind: ActorKind,
        lane: Lane,
        window: Option<WindowId>,
        _event: ActivationEvent,
        shard: ShardId,
        parent: Option<ActorId>,
        requested_capabilities: Vec<CapabilityGrant>,
        retained: RetainedTurnInput,
        loan:OriginalActorTurnLoan,
    ) -> ActorId {
        let ordinal = self.next_ordinal.entry(package.clone()).or_insert(0);
        let id = ActorId::new(plugin_ordinal, kind.tag().await, *ordinal, 0).await;
        *ordinal += 1;
        let budget = lane_defaults::budget_for(lane);
        let shard = self.shards.pin_to(id, shard).await;
        self.scheduler.register_actor(id, package.clone(), lane, budget, shard, retained,loan).await;
        let capabilities = match parent {
            Some(parent_id) => {
                let granted = self.actors.get(&parent_id).map(|meta| meta.capabilities.clone()).unwrap_or_default();
                intersect_capabilities(&granted, &requested_capabilities).await
            }
            None => requested_capabilities,
        };
        self.actors.insert(id, ActorMeta { reservation: None, transport_key: None, kind, package, capabilities, budget, status: ActorStatus::Activating, failure: FailureState::new(), metrics: ActorMetrics::default(), window });
        id
    }

    /// 🔐️ Records/replaces an already-live actor's granted [`CapabilityGrant`] set — how a host's
    /// broker (or, in tests, the caller) attaches the capabilities a `PluginApp` was actually granted
    /// so that a later [`Self::activate_pinned`] can intersect an extension's requests against them.
    pub async fn set_capabilities(&mut self, actor: ActorId, grants: Vec<CapabilityGrant>) -> Result<(), KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        meta.capabilities = grants;
        Ok(())
    }

    /// 🧭️ Convenience: an actor's pinned shard without paying [`Self::actor_record`]'s full
    /// `ActorRecord` assembly cost (that allocates a fresh empty [`Mailbox`] every call).
    pub async fn shard_of(&self, actor: ActorId) -> Option<ShardId> {
        self.shards.shard_of(actor).await
    }

    /// 🔗️ Records a parent→child cascade edge. Both actors must already be live; the child is
    /// typically an `ActorKind::Extension` just returned by [`Self::activate_pinned`], but this is not
    /// itself enforced — cascade topology is a separate concern from actor kind (see this method's own
    /// region doc).
    pub async fn link_extension(&mut self, parent: ActorId, child: ActorId) -> Result<(), KernelError> {
        if !self.actors.contains_key(&parent) || !self.actors.contains_key(&child) {
            return Err(KernelError::UnknownActor);
        }
        if [parent, child].into_iter().any(|actor| self.actors.get(&actor).is_some_and(|meta| meta.reservation.is_some())) {
            return Err(KernelError::InvalidTransition);
        }
        self.links.entry(parent).or_default().push(child);
        Ok(())
    }

    /// 🔗️ The direct children [`Self::link_extension`] recorded for `parent` — empty for a leaf or an
    /// actor with no extensions.
    pub async fn children_of(&self, parent: ActorId) -> Vec<ActorId> {
        self.links.get(&parent).cloned().unwrap_or_default()
    }

    /// 🌳️ Iterative (never self-recursive — R10's "self-recursive async fn" residue shape applies to
    /// `async fn`s exactly as much as sync ones, and this crate's own rule bans the `Box::pin` escape
    /// hatch where a loop will do) post-order walk of `root`'s cascade subtree: every descendant
    /// appears strictly before its own parent, terminating with `root` itself last — i.e. leaves-first.
    /// Shared by every cascading lifecycle op below; [`Self::resume_cascade`] reverses this order to
    /// get the symmetric parent-first restore direction the design calls for.
    async fn subtree_leaves_first(&self, root: ActorId) -> Vec<ActorId> {
        let mut order = Vec::new();
        let mut stack = vec![(root, false)];
        while let Some((node, expanded)) = stack.pop() {
            if expanded {
                order.push(node);
                continue;
            }
            stack.push((node, true));
            for child in self.children_of(node).await {
                stack.push((child, false));
            }
        }
        order
    }

    /// ✂️ Shared removal primitive behind [`Self::deactivate`] and [`Self::kill`] — full teardown of
    /// `root`'s ENTIRE cascade subtree, leaves-first: `Scheduler::unregister_actor` (drops the DRR
    /// entry + mailbox), `ShardTable::unpin` (frees the shard slot/any exclusive lease), removes the
    /// `ActorMeta` and this actor's own outgoing link-table row. Also scrubs every removed id out of
    /// every OTHER actor's children list, so a subtree removal can never leave a dangling edge pointing
    /// at an id [`Self::actor_record`] can no longer resolve. Returns the removed ids leaves-first —
    /// the caller's evidence that the cascade actually walked the whole subtree, not just `root`.
    async fn cascade_remove(&mut self, root: ActorId) -> Vec<ActorId> {
        let order = self.subtree_leaves_first(root).await;
        for &id in &order {
            self.scheduler.unregister_actor(id).await;
            self.shards.unpin(id).await;
            self.actors.remove(&id);
            self.links.remove(&id);
        }
        for children in self.links.values_mut() {
            children.retain(|child| !order.contains(child));
        }
        order
    }

    /// ✂️ Graceful cascade teardown: `root` and every extension hanging off it (transitively),
    /// leaves-first, with **zero orphans** — no descendant is ever left registered after its ancestor
    /// is gone. The call site this ticket's own design names: a plugin closing, or an extension being
    /// uninstalled while live. Errors `UnknownActor` if `root` is not (or no longer) live.
    pub async fn deactivate(&mut self, root: ActorId) -> Result<Vec<ActorId>, KernelError> {
        if !self.actors.contains_key(&root) {
            return Err(KernelError::UnknownActor);
        }
        Ok(self.cascade_remove(root).await)
    }

    /// 🔪️ The failure ladder's cascade teardown — same leaves-first, zero-orphan removal as
    /// [`Self::deactivate`] (this crate's pure kernel state has no separate "abrupt vs. graceful"
    /// axis: both fully retire the subtree; a host distinguishes them only by WHEN it calls each —
    /// `deactivate` for user/uninstall-driven teardown, `kill` for the failure ladder giving up on an
    /// unrecoverable actor). Kept as its own named entry point rather than an alias so call sites read
    /// as intent, and so a future host-side distinction (e.g. a forced `ShardTransport::kill` versus a
    /// graceful drain) has an unambiguous kernel-level hook to attach to. "Parent kill takes its
    /// extensions down" (design doc M6's acceptance wording) is exactly this method called on the
    /// parent.
    pub async fn kill(&mut self, root: ActorId) -> Result<Vec<ActorId>, KernelError> {
        if !self.actors.contains_key(&root) {
            return Err(KernelError::UnknownActor);
        }
        Ok(self.cascade_remove(root).await)
    }

    /// 💤️ Cascading [`Self::suspend`], leaves-first: every descendant is suspended before `root`
    /// itself. This is ALSO this crate's "checkpoint cascade" — [`ActorStatus::Suspended`] already
    /// carries an optional checkpoint payload, so "checkpoint" and "suspend-with-bytes" are the same
    /// kernel operation; only `root` receives the caller's `checkpoint` bytes (a per-descendant
    /// checkpoint is a host/guest-runtime concern — the pure kernel only ever records the bytes it is
    /// handed, never produces them). Descendants suspend with `checkpoint: None`.
    pub async fn suspend_cascade(&mut self, root: ActorId, checkpoint: Option<Vec<u8>>) -> Result<Vec<ActorId>, KernelError> {
        if !self.actors.contains_key(&root) {
            return Err(KernelError::UnknownActor);
        }
        let order = self.subtree_leaves_first(root).await;
        for &id in &order {
            let bytes = if id == root { checkpoint.clone() } else { None };
            self.suspend(id, bytes).await?;
        }
        Ok(order)
    }

    /// ▶️ Cascading [`Self::resume`], PARENT-first — the symmetric restore direction (design doc M6's
    /// web-mirror doc: "symmetric cascade, restore: parent first"): a child's shard/mailbox is only
    /// useful once its parent is running again. Skips any descendant that is not currently
    /// [`ActorStatus::Suspended`] rather than erroring, since a partial cascade (only some children were
    /// ever suspended) must still resume everything that legitimately can.
    pub async fn resume_cascade(&mut self, root: ActorId) -> Result<Vec<ActorId>, KernelError> {
        if !self.actors.contains_key(&root) {
            return Err(KernelError::UnknownActor);
        }
        let mut order = self.subtree_leaves_first(root).await;
        order.reverse();
        let mut resumed = Vec::new();
        for &id in &order {
            if matches!(self.actor_status(id).await, Some(ActorStatus::Suspended { .. })) {
                self.resume(id).await?;
                resumed.push(id);
            }
        }
        Ok(resumed)
    }
    //#endregion 🔗️ExtensionActivation
}
//#endregion 🏛️Kernel

#[cfg(test)]
#[global_allocator]
static ACTOR_TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🎟️retained-turn/🦀️.rs"]
mod retained_turn_tests;
