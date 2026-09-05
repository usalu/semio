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
use std::mem::{ManuallyDrop, MaybeUninit};
use std::sync::Arc;

use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

pub use semio_framework_job as job;

#[path = "🚪️lifetime/🦀️.rs"]
pub mod instance_lifetime;

#[path = "📃️page/🦀️.rs"]
pub mod byte_page;

#[path = "📤️return/🦀️.rs"]
pub mod return_page;

#[cfg(test)]
#[path = "📃️page/🧪️tests/🦀️.rs"]
mod byte_page_tests;

#[cfg(test)]
#[path = "📤️return/🧪️tests/🦀️.rs"]
mod return_page_tests;

//#region 🧬️SchemaMetadata
#[cfg(feature = "typegen")]
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
        SchemaMetadata { name: "Budget", version: 1, typescript: "export type Budget = { fuel: bigint, wall_ms: number, memory_bytes: bigint, ui_nodes: number, mailbox_len: number, max_effects: number, max_patch_bytes: number, };" },
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
            typescript: "export type JobReplayRoute = { plugin: number[], package: number[], controller: number[], tool: number[], window: bigint, document: number[], requestSchema: number[], requestVersion: number, requestDigest: number[], };",
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
        SchemaMetadata { name: "Origin", version: 1, typescript: r#"export type Origin = { "kind": "ui", window: WindowId, } | { "kind": "actor", id: ActorId, } | { "kind": "kernel" } | { "kind": "bus", topic: string, };"# },
        SchemaMetadata { name: "PackageHash", version: 1, typescript: "export type PackageHash = number[];" },
        SchemaMetadata { name: "PackageId", version: 1, typescript: "export type PackageId = string;" },
        SchemaMetadata {
            name: "Payload",
            version: 1,
            typescript: r#"export type Payload = { "kind": "event", bytes: Array<number>, } | { "kind": "suspend", operation: JobOperation, appliedProgress: bigint, } | { "kind": "resume", operation: JobOperation, checkpoint: JobCheckpoint, } | { "kind": "cancel", seq: bigint, } | { "kind": "jobStep", turn: JobTurn, };"#,
        },
        SchemaMetadata { name: "RuntimeMetricsSnapshot", version: 1, typescript: "export type RuntimeMetricsSnapshot = { kernel: KernelMetrics, actors: Array<ActorMetricsSample>, shards: Array<ShardMetricsSample>, sampled_at_ms: bigint, };" },
        SchemaMetadata { name: "SceneSnapshot", version: 1, typescript: "export type SceneSnapshot = { revision: bigint, committed_ms: bigint, patches: Array<number>, node_count: number, };" },
        SchemaMetadata { name: "ShardId", version: 1, typescript: "export type ShardId = number;" },
        SchemaMetadata { name: "ShardKind", version: 1, typescript: r#"export type ShardKind = "Native" | "WebWorker" | "Process";"# },
        SchemaMetadata { name: "ShardMetrics", version: 1, typescript: "export type ShardMetrics = { actors: number, busy_ratio: number, heartbeat_age_ms: number, };" },
        SchemaMetadata { name: "ShardMetricsSample", version: 1, typescript: "export type ShardMetricsSample = { shard: ShardId, metrics: ShardMetrics, };" },
        SchemaMetadata { name: "ShardTable", version: 1, typescript: "export type ShardTable = { kind: ShardKind, shard_count: number, exclusive_reserve: number, assignment: Record<string, ShardId>, exclusive_leases: Record<string, ActorId>, };" },
        SchemaMetadata { name: "TurnGrant", version: 1, typescript: "export type TurnGrant = { actor: ActorId, shard: ShardId, budget: Budget, envelopes: Array<Envelope>, };" },
        SchemaMetadata { name: "TurnResult", version: 1, typescript: "export type TurnResult = { ui_patches: Array<number>, effects: Array<number>, command_ingress: Array<number>, lifecycle_receipt: import(\"../🚪️lifetime/🟦️component.js\").ActorInstanceLifecycleReceipt | null, ui_patch_receipt: import(\"../🚪️lifetime/🩹️patch/🟦️component.js\").ActorUiPatchReceipt | null, next_wake: bigint | null, status: TurnStatus, usage: Usage, };" },
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
        let mut output = String::from("/** @generated by `bun nx run @semio-tech/framework-actor-rs:typegen` from 🎭️actor owned schema metadata. Do not edit. */\n\n");
        for metadata in TYPES {
            output.push_str(metadata.typescript);
            output.push_str("\n\n");
        }
        output
    }
}
//#endregion 🧬️SchemaMetadata

//#region 🧵️Pack
/// 🧵️ Hand-rolled binary codec primitives shared by every type's `pack_encode`/`pack_decode` pair
/// below — LEB128 varints, length-prefixed bytes/strings, fixed 32-byte hashes, option/vec
/// combinators. Self-contained (no dependency on `🎒️pack`, which lives in the os-product layer).
pub mod pack {
    /// 🚨️ The one error type every `pack_decode` returns.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PackError {
        Truncated(usize, &'static str),
        InvalidTag { what: &'static str, tag: u8, offset: usize },
        InvalidUtf8(&'static str, usize),
        OverlongVarint(usize),
        InvalidLifecycle(&'static str),
        InvalidUiPatchReceipt(&'static str),
    }

    impl std::fmt::Display for PackError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Truncated(offset, what) => write!(formatter, "pack: truncated at offset {offset} reading {what}"),
                Self::InvalidTag { what, tag, offset } => write!(formatter, "pack: invalid tag {tag} for {what} at offset {offset}"),
                Self::InvalidUtf8(what, offset) => write!(formatter, "pack: invalid utf8 in {what} at offset {offset}"),
                Self::OverlongVarint(offset) => write!(formatter, "pack: overlong varint at offset {offset}"),
                Self::InvalidLifecycle(reason) => write!(formatter, "pack: invalid instance lifecycle: {reason}"),
                Self::InvalidUiPatchReceipt(reason) => write!(formatter, "pack: invalid issued UI patch receipt: {reason}"),
            }
        }
    }

    impl std::error::Error for PackError {}

    pub async fn write_u8(out: &mut Vec<u8>, v: u8) {
        out.push(v);
    }
    pub async fn read_u8(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u8, PackError> {
        let byte = *bytes.get(*pos).ok_or(PackError::Truncated(*pos, what))?;
        *pos += 1;
        Ok(byte)
    }

    pub async fn write_bool(out: &mut Vec<u8>, v: bool) {
        out.push(v as u8);
    }
    pub async fn read_bool(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<bool, PackError> {
        Ok(read_u8(bytes, pos, what).await? != 0)
    }

    pub async fn write_u16(out: &mut Vec<u8>, v: u16) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    pub async fn read_u16(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u16, PackError> {
        let end = *pos + 2;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(u16::from_le_bytes(slice.try_into().expect("2 bytes")))
    }

    pub async fn write_u32(out: &mut Vec<u8>, v: u32) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    pub async fn read_u32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u32, PackError> {
        let end = *pos + 4;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(u32::from_le_bytes(slice.try_into().expect("4 bytes")))
    }

    pub async fn write_u64(out: &mut Vec<u8>, v: u64) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    pub async fn read_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u64, PackError> {
        let end = *pos + 8;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(u64::from_le_bytes(slice.try_into().expect("8 bytes")))
    }

    pub async fn write_f32(out: &mut Vec<u8>, v: f32) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    pub async fn read_f32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<f32, PackError> {
        let end = *pos + 4;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(f32::from_le_bytes(slice.try_into().expect("4 bytes")))
    }

    pub async fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
        let mut remaining = value;
        loop {
            let byte = (remaining & 0x7F) as u8;
            remaining >>= 7;
            if remaining == 0 {
                out.push(byte);
                break;
            }
            out.push(byte | 0x80);
        }
    }
    pub async fn read_varint_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u64, PackError> {
        let start = *pos;
        let mut result: u64 = 0;
        for i in 0..10usize {
            let byte = *bytes.get(*pos).ok_or(PackError::Truncated(*pos, what))?;
            *pos += 1;
            let more = byte & 0x80 != 0;
            let payload = (byte & 0x7F) as u64;
            if i == 9 && (more || payload > 1) {
                return Err(PackError::OverlongVarint(start));
            }
            result |= payload << (i as u32 * 7);
            if !more {
                return Ok(result);
            }
        }
        Err(PackError::OverlongVarint(start))
    }

    pub async fn write_bytes(out: &mut Vec<u8>, v: &[u8]) {
        write_varint_u64(out, v.len() as u64).await;
        out.extend_from_slice(v);
    }
    pub async fn read_bytes(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Vec<u8>, PackError> {
        let len = read_varint_u64(bytes, pos, what).await? as usize;
        let end = *pos + len;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(slice.to_vec())
    }

    pub async fn write_str(out: &mut Vec<u8>, v: &str) {
        write_bytes(out, v.as_bytes()).await;
    }
    pub async fn read_str(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<String, PackError> {
        let start = *pos;
        let raw = read_bytes(bytes, pos, what).await?;
        String::from_utf8(raw).map_err(|_| PackError::InvalidUtf8(what, start))
    }

    pub async fn write_hash32(out: &mut Vec<u8>, v: &[u8; 32]) {
        out.extend_from_slice(v);
    }
    pub async fn read_hash32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<[u8; 32], PackError> {
        let end = *pos + 32;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(slice.try_into().expect("32 bytes"))
    }

    /// 🪡 Concrete (non-generic) `Option<Vec<u8>>` combinators — a closure-based generic
    /// `write_opt`/`read_opt` hits rustc's "implementation of `AsyncFnOnce` is not general enough"
    /// (the returned per-call future can't be proven to outlive a HRTB-quantified borrow of `out`/
    /// `bytes`, a known async-closures rough edge). Concrete types sidestep it entirely; every
    /// `Option<Vec<u8>>` field in this file's wire format goes through these two.
    pub async fn write_opt_bytes(out: &mut Vec<u8>, v: &Option<Vec<u8>>) {
        write_bool(out, v.is_some()).await;
        if let Some(x) = v {
            write_bytes(out, x).await;
        }
    }
    pub async fn read_opt_bytes(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Option<Vec<u8>>, PackError> {
        if read_bool(bytes, pos, what).await? {
            Ok(Some(read_bytes(bytes, pos, what).await?))
        } else {
            Ok(None)
        }
    }

    /// 🪡 Same rationale as `write_opt_bytes`/`read_opt_bytes`, for `Option<u64>` (deadlines/wake times).
    pub async fn write_opt_u64(out: &mut Vec<u8>, v: &Option<u64>) {
        write_bool(out, v.is_some()).await;
        if let Some(x) = v {
            write_u64(out, *x).await;
        }
    }
    pub async fn read_opt_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Option<u64>, PackError> {
        if read_bool(bytes, pos, what).await? {
            Ok(Some(read_u64(bytes, pos, what).await?))
        } else {
            Ok(None)
        }
    }

    /// 🪡 `f` is a bare async fn item (`Type::pack_encode`), never a closure — fn items are
    /// automatically higher-ranked over their argument lifetimes, so they don't hit the
    /// `write_opt_bytes` doc's HRTB rough edge. `(item, out)` order (not `(out, item)`) is what
    /// lets `T::pack_encode`'s own `(&self, out)` signature line up without an adapter closure.
    pub async fn write_vec<T>(out: &mut Vec<u8>, v: &[T], mut f: impl AsyncFnMut(&T, &mut Vec<u8>)) {
        write_varint_u64(out, v.len() as u64).await;
        for item in v {
            f(item, out).await;
        }
    }
    pub async fn read_vec<T>(bytes: &[u8], pos: &mut usize, what: &'static str, mut f: impl AsyncFnMut(&[u8], &mut usize) -> Result<T, PackError>) -> Result<Vec<T>, PackError> {
        let len = read_varint_u64(bytes, pos, what).await? as usize;
        let mut out = Vec::with_capacity(len.min(1 << 20));
        for _ in 0..len {
            out.push(f(bytes, pos).await?);
        }
        Ok(out)
    }
}
//#endregion 🧵️Pack

//#region 📦️PackageId
/// 📦️ Stable identity of an installed plugin or plugin+extension pair: `<plugin>` or `<plugin>/<extension>`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value", transparent)]
pub struct PackageId(pub String);

impl PackageId {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_str(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_str(bytes, pos, "PackageId").await?))
    }
}

/// 🧬️ Blake3 hash of a compiled component's bytes — the compiled-cache key (`~/.semio/cache/wasmtime/...`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value", transparent)]
pub struct PackageHash(pub [u8; 32]);

impl std::fmt::Debug for PackageHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PackageHash(")?;
        for byte in &self.0[..4] {
            write!(f, "{byte:02x}")?;
        }
        write!(f, "…)")
    }
}

impl PackageHash {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_hash32(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_hash32(bytes, pos, "PackageHash").await?))
    }
}
//#endregion 📦️PackageId

//#region 🆔️ActorId
/// 🆔️ Bit-packed actor identifier: `plugin_ordinal:u16 | kind:u2 | ordinal:u32 | generation:u14`.
/// Generation makes restart-after-trap addressable without id reuse. The kernel re-exports this
/// type as `RuntimeActorId` (`kernel::ActorId` already names the presence/collab actor — never shadow).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value", transparent)]
pub struct ActorId(pub u64);

const ACTOR_ID_GENERATION_BITS: u32 = 14;
const ACTOR_ID_ORDINAL_BITS: u32 = 32;
const ACTOR_ID_KIND_BITS: u32 = 2;
const ACTOR_ID_GENERATION_MASK: u64 = (1 << ACTOR_ID_GENERATION_BITS) - 1;
const ACTOR_ID_ORDINAL_MASK: u64 = (1 << ACTOR_ID_ORDINAL_BITS) - 1;
const ACTOR_ID_KIND_MASK: u64 = (1 << ACTOR_ID_KIND_BITS) - 1;
const ACTOR_ID_ORDINAL_SHIFT: u32 = ACTOR_ID_GENERATION_BITS;
const ACTOR_ID_KIND_SHIFT: u32 = ACTOR_ID_ORDINAL_SHIFT + ACTOR_ID_ORDINAL_BITS;
const ACTOR_ID_PLUGIN_SHIFT: u32 = ACTOR_ID_KIND_SHIFT + ACTOR_ID_KIND_BITS;

impl ActorId {
    pub async fn new(plugin_ordinal: u16, kind_tag: u8, ordinal: u32, generation: u16) -> Self {
        let bits = ((plugin_ordinal as u64) << ACTOR_ID_PLUGIN_SHIFT) | (((kind_tag as u64) & ACTOR_ID_KIND_MASK) << ACTOR_ID_KIND_SHIFT) | ((ordinal as u64) << ACTOR_ID_ORDINAL_SHIFT) | ((generation as u64) & ACTOR_ID_GENERATION_MASK);
        Self(bits)
    }

    // 🚫️async: E1 pure accessor consumed by `Debug::fmt` (external trait impl below) — see R9
    pub fn plugin_ordinal(self) -> u16 {
        (self.0 >> ACTOR_ID_PLUGIN_SHIFT) as u16
    }

    // 🚫️async: E1 pure accessor consumed by `Debug::fmt` (external trait impl below) — see R9
    pub fn kind_tag(self) -> u8 {
        ((self.0 >> ACTOR_ID_KIND_SHIFT) & ACTOR_ID_KIND_MASK) as u8
    }

    // 🚫️async: E1 pure accessor consumed by `Debug::fmt` (external trait impl below) — see R9
    pub fn ordinal(self) -> u32 {
        ((self.0 >> ACTOR_ID_ORDINAL_SHIFT) & ACTOR_ID_ORDINAL_MASK) as u32
    }

    // 🚫️async: E1 pure accessor consumed by `Debug::fmt` (external trait impl below) — see R9
    pub fn generation(self) -> u16 {
        (self.0 & ACTOR_ID_GENERATION_MASK) as u16
    }

    /// 🔁️ Restart-after-trap: same plugin/kind/ordinal, generation bumped (wraps at 14 bits — a
    /// restart storm beyond 16384 generations is itself a `Disabled`-worthy condition upstream).
    pub async fn next_generation(self) -> Self {
        Self::new(self.plugin_ordinal(), self.kind_tag(), self.ordinal(), self.generation().wrapping_add(1) & (ACTOR_ID_GENERATION_MASK as u16)).await
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_u64(bytes, pos, "ActorId").await?))
    }
}

impl std::fmt::Debug for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActorId(plugin={}, kind={}, ordinal={}, gen={})", self.plugin_ordinal(), self.kind_tag(), self.ordinal(), self.generation())
    }
}
//#endregion 🆔️ActorId

//#region 🎭️ActorKind
/// 🎭️ What an actor slot represents: a running app instance, an activated extension, or a
/// background job spawned by another actor. Discriminant order matches `ActorId`'s `kind:u2` tag.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActorKind {
    PluginApp { plugin: PackageId, app_id: String, instance_id: u32 },
    Extension { plugin: PackageId, extension_id: String },
    Job { owner: ActorId, job_id: u64 },
}

impl ActorKind {
    pub async fn tag(&self) -> u8 {
        match self {
            ActorKind::PluginApp { .. } => 0,
            ActorKind::Extension { .. } => 1,
            ActorKind::Job { .. } => 2,
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            ActorKind::PluginApp { plugin, app_id, instance_id } => {
                pack::write_u8(out, 0).await;
                plugin.pack_encode(out).await;
                pack::write_str(out, app_id).await;
                pack::write_u32(out, *instance_id).await;
            }
            ActorKind::Extension { plugin, extension_id } => {
                pack::write_u8(out, 1).await;
                plugin.pack_encode(out).await;
                pack::write_str(out, extension_id).await;
            }
            ActorKind::Job { owner, job_id } => {
                pack::write_u8(out, 2).await;
                owner.pack_encode(out).await;
                pack::write_u64(out, *job_id).await;
            }
        }
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "ActorKind").await?;
        match tag {
            0 => Ok(ActorKind::PluginApp { plugin: PackageId::pack_decode(bytes, pos).await?, app_id: pack::read_str(bytes, pos, "ActorKind::app_id").await?, instance_id: pack::read_u32(bytes, pos, "ActorKind::instance_id").await? }),
            1 => Ok(ActorKind::Extension { plugin: PackageId::pack_decode(bytes, pos).await?, extension_id: pack::read_str(bytes, pos, "ActorKind::extension_id").await? }),
            2 => Ok(ActorKind::Job { owner: ActorId::pack_decode(bytes, pos).await?, job_id: pack::read_u64(bytes, pos, "ActorKind::job_id").await? }),
            other => Err(pack::PackError::InvalidTag { what: "ActorKind", tag: other, offset: *pos }),
        }
    }
}
//#endregion 🎭️ActorKind

//#region 🛣️Lane
/// 🛣️ Scheduling priority class. Ordered highest-to-lowest priority by declaration order — see
/// [`Lane::priority_rank`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub enum Lane {
    Interactive,
    UserVisible,
    Background,
    Maintenance,
}

impl Lane {
    pub const ALL: [Lane; 4] = [Lane::Interactive, Lane::UserVisible, Lane::Background, Lane::Maintenance];

    /// 0 = highest priority (Interactive) .. 3 = lowest (Maintenance).
    pub async fn priority_rank(self) -> usize {
        match self {
            Lane::Interactive => 0,
            Lane::UserVisible => 1,
            Lane::Background => 2,
            Lane::Maintenance => 3,
        }
    }

    /// ⚖️ Level-2 DRR quantum weight — biases actor selection within a plugin toward interactive work.
    // 🚫️async: E1-adjacent pure computation — its one consumer, `Scheduler::actor_weight`, is
    // itself sync for the same reason (see that fn's own tag) — see R9 residue shape 1.
    pub fn weight(self) -> u32 {
        match self {
            Lane::Interactive => 8,
            Lane::UserVisible => 4,
            Lane::Background => 2,
            Lane::Maintenance => 1,
        }
    }

    async fn tag(self) -> u8 {
        self.priority_rank().await as u8
    }

    async fn from_tag(tag: u8) -> Result<Self, pack::PackError> {
        match tag {
            0 => Ok(Lane::Interactive),
            1 => Ok(Lane::UserVisible),
            2 => Ok(Lane::Background),
            3 => Ok(Lane::Maintenance),
            other => Err(pack::PackError::InvalidTag { what: "Lane", tag: other, offset: 0 }),
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u8(out, self.tag().await).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Lane").await?;
        Self::from_tag(tag).await
    }
}
//#endregion 🛣️Lane

//#region ⚖️Budget
/// ⚖️ Per-turn resource ceilings enforced host-side. Replaces `PLUGIN_FUEL_BUDGET`.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct Budget {
    pub fuel: u64,
    pub wall_ms: u32,
    pub memory_bytes: u64,
    pub ui_nodes: u32,
    pub mailbox_len: u16,
    pub max_effects: u32,
    pub max_patch_bytes: u32,
}

impl Budget {
    /// ⚖️ Scales a `Throttled { factor }` failure stage onto this budget — fuel/wall/effects/patch
    /// bytes shrink; `mailbox_len`/`ui_nodes` ceilings are left alone (they bound queued/committed
    /// state, not this turn's spend).
    pub async fn scaled(self, factor: f32) -> Budget {
        let factor = factor.clamp(0.05, 1.0);
        Budget {
            fuel: ((self.fuel as f64) * factor as f64) as u64,
            wall_ms: ((self.wall_ms as f64) * factor as f64).max(1.0) as u32,
            memory_bytes: self.memory_bytes,
            ui_nodes: self.ui_nodes,
            mailbox_len: self.mailbox_len,
            max_effects: ((self.max_effects as f64) * factor as f64) as u32,
            max_patch_bytes: self.max_patch_bytes,
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.fuel).await;
        pack::write_u32(out, self.wall_ms).await;
        pack::write_u64(out, self.memory_bytes).await;
        pack::write_u32(out, self.ui_nodes).await;
        pack::write_u16(out, self.mailbox_len).await;
        pack::write_u32(out, self.max_effects).await;
        pack::write_u32(out, self.max_patch_bytes).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            fuel: pack::read_u64(bytes, pos, "Budget::fuel").await?,
            wall_ms: pack::read_u32(bytes, pos, "Budget::wall_ms").await?,
            memory_bytes: pack::read_u64(bytes, pos, "Budget::memory_bytes").await?,
            ui_nodes: pack::read_u32(bytes, pos, "Budget::ui_nodes").await?,
            mailbox_len: pack::read_u16(bytes, pos, "Budget::mailbox_len").await?,
            max_effects: pack::read_u32(bytes, pos, "Budget::max_effects").await?,
            max_patch_bytes: pack::read_u32(bytes, pos, "Budget::max_patch_bytes").await?,
        })
    }
}

//#region ⚖️LaneDefaults
/// ⚖️ Default [`Budget`] per [`Lane`] — Interactive 4ms/2M fuel, UserVisible 16ms, Background 50ms,
/// Maintenance 200ms (the ladder the design spec fixes explicitly); the remaining ceilings scale
/// down the same tiers so background/maintenance work can never out-spend memory/UI/mailbox room
/// an interactive turn would need.
pub mod lane_defaults {
    use super::{Budget, Lane};

    pub fn budget_for(lane: Lane) -> Budget {
        match lane {
            Lane::Interactive => Budget { fuel: 2_000_000, wall_ms: 4, memory_bytes: 64 * 1024 * 1024, ui_nodes: 20_000, mailbox_len: 256, max_effects: 64, max_patch_bytes: 262_144 },
            Lane::UserVisible => Budget { fuel: 6_000_000, wall_ms: 16, memory_bytes: 96 * 1024 * 1024, ui_nodes: 20_000, mailbox_len: 256, max_effects: 128, max_patch_bytes: 524_288 },
            Lane::Background => Budget { fuel: 20_000_000, wall_ms: 50, memory_bytes: 192 * 1024 * 1024, ui_nodes: 8_000, mailbox_len: 512, max_effects: 256, max_patch_bytes: 1_048_576 },
            Lane::Maintenance => Budget { fuel: 80_000_000, wall_ms: 200, memory_bytes: 256 * 1024 * 1024, ui_nodes: 4_000, mailbox_len: 1024, max_effects: 512, max_patch_bytes: 2_097_152 },
        }
    }
}
//#endregion ⚖️LaneDefaults
//#endregion ⚖️Budget

//#region 🪪️JobBridge
/// 🪪️ Stable operation identity carried by every actor job turn and publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct JobOperation {
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
    pub preview_sequence: u64,
    pub seed: u64,
}

impl JobOperation {
    pub fn from_job(operation: job::Operation) -> Self {
        Self { operation: operation.operation.0, base_revision: operation.base_revision.0, generation: operation.generation.0, preview_sequence: operation.preview_sequence, seed: operation.seed }
    }

    pub fn into_job(self) -> job::Operation {
        job::Operation { operation: job::OperationId(self.operation), base_revision: job::RevisionId(self.base_revision), generation: job::Generation(self.generation), preview_sequence: self.preview_sequence, seed: self.seed }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.operation).await;
        pack::write_u64(out, self.base_revision).await;
        pack::write_u64(out, self.generation).await;
        pack::write_u64(out, self.preview_sequence).await;
        pack::write_u64(out, self.seed).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            operation: pack::read_u64(bytes, pos, "JobOperation::operation").await?,
            base_revision: pack::read_u64(bytes, pos, "JobOperation::base_revision").await?,
            generation: pack::read_u64(bytes, pos, "JobOperation::generation").await?,
            preview_sequence: pack::read_u64(bytes, pos, "JobOperation::preview_sequence").await?,
            seed: pack::read_u64(bytes, pos, "JobOperation::seed").await?,
        })
    }
}

/// 📸️ Opaque resumable state plus the committed progress boundary it represents.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct JobCheckpoint {
    pub state: Vec<u8>,
    pub applied_progress: u64,
}

impl JobCheckpoint {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_bytes(out, &self.state).await;
        pack::write_u64(out, self.applied_progress).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { state: pack::read_bytes(bytes, pos, "JobCheckpoint::state").await?, applied_progress: pack::read_u64(bytes, pos, "JobCheckpoint::applied_progress").await? })
    }
}

/// 🏁️ Final persisted job state and its authoritative output candidate.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct JobCommitCandidate {
    pub state: Vec<u8>,
    pub output: Vec<u8>,
}

impl JobCommitCandidate {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_bytes(out, &self.state).await;
        pack::write_bytes(out, &self.output).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { state: pack::read_bytes(bytes, pos, "JobCommitCandidate::state").await?, output: pack::read_bytes(bytes, pos, "JobCommitCandidate::output").await? })
    }
}

/// 🚦️ Lossless actor-wire mirror of one universal `StepOutcome`.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum JobStepOutcome {
    Yield,
    PreviewReady { preview: Vec<u8> },
    CheckpointReady { checkpoint: JobCheckpoint },
    Complete { candidate: JobCommitCandidate },
    Cancelled,
    Fault { detail: Vec<u8> },
}

impl JobStepOutcome {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Self::Yield => pack::write_u8(out, 0).await,
            Self::PreviewReady { preview } => {
                pack::write_u8(out, 1).await;
                pack::write_bytes(out, preview).await;
            }
            Self::CheckpointReady { checkpoint } => {
                pack::write_u8(out, 2).await;
                checkpoint.pack_encode(out).await;
            }
            Self::Complete { candidate } => {
                pack::write_u8(out, 3).await;
                candidate.pack_encode(out).await;
            }
            Self::Cancelled => pack::write_u8(out, 4).await,
            Self::Fault { detail } => {
                pack::write_u8(out, 5).await;
                pack::write_bytes(out, detail).await;
            }
        }
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        match pack::read_u8(bytes, pos, "JobStepOutcome").await? {
            0 => Ok(Self::Yield),
            1 => Ok(Self::PreviewReady { preview: pack::read_bytes(bytes, pos, "JobStepOutcome::PreviewReady").await? }),
            2 => Ok(Self::CheckpointReady { checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(Self::Complete { candidate: JobCommitCandidate::pack_decode(bytes, pos).await? }),
            4 => Ok(Self::Cancelled),
            5 => Ok(Self::Fault { detail: pack::read_bytes(bytes, pos, "JobStepOutcome::Fault").await? }),
            tag => Err(pack::PackError::InvalidTag { what: "JobStepOutcome", tag, offset: *pos }),
        }
    }
}

/// 🎫️ One explicitly-addressed bounded job turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct JobTurn {
    pub job: u64,
    pub operation: JobOperation,
    pub step_sequence: u64,
}

impl JobTurn {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.job).await;
        self.operation.pack_encode(out).await;
        pack::write_u64(out, self.step_sequence).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { job: pack::read_u64(bytes, pos, "JobTurn::job").await?, operation: JobOperation::pack_decode(bytes, pos).await?, step_sequence: pack::read_u64(bytes, pos, "JobTurn::step_sequence").await? })
    }
}

/// 📡️ One validated, replay-addressable publication from a bounded job turn.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct JobPublication {
    pub turn: JobTurn,
    pub outcome: JobStepOutcome,
}

impl JobPublication {
    pub fn turn_status(&self) -> TurnStatus {
        match &self.outcome {
            JobStepOutcome::Yield => TurnStatus::MoreWork,
            JobStepOutcome::PreviewReady { preview } => TurnStatus::PreviewReady { preview: preview.clone(), sequence: self.turn.operation.preview_sequence.saturating_sub(1) },
            JobStepOutcome::CheckpointReady { checkpoint } => TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
            JobStepOutcome::Complete { candidate } => TurnStatus::CommitReady { candidate: candidate.clone() },
            JobStepOutcome::Cancelled => TurnStatus::Cancelled,
            JobStepOutcome::Fault { detail } => TurnStatus::Faulted { detail: detail.clone() },
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.turn.pack_encode(out).await;
        self.outcome.pack_encode(out).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { turn: JobTurn::pack_decode(bytes, pos).await?, outcome: JobStepOutcome::pack_decode(bytes, pos).await? })
    }
}

//#region 📜️FixedReplay
pub const JOB_REPLAY_RECORD_CAPACITY: usize = 256;
pub const JOB_REPLAY_RECORD_PAGE_CAPACITY: usize = 32;
pub const JOB_REPLAY_PROCESS_PAGE_CAPACITY: usize = 4_096;
pub const JOB_REPLAY_PAGE_BYTES: usize = job::JOB_PAYLOAD_PAGE_BYTES;
const JOB_REPLAY_SCALAR_TRANSFERS: u8 = 12;

static JOB_REPLAY_PROCESS_PAGES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🧭️ Versioned mounted route identity shared by capture and replay.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct JobReplayRoute {
    pub plugin: [u8; 32],
    pub package: [u8; 32],
    pub controller: [u8; 32],
    pub tool: [u8; 32],
    pub window: u64,
    pub document: [u8; 32],
    pub request_schema: [u8; 32],
    pub request_version: u16,
    pub request_digest: [u8; 32],
}

/// 🪪️ Exact fixed request identity retained from `Effect::SpawnJob` through publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct JobReplayRequest {
    pub controller: [u8; 32],
    pub tool: [u8; 32],
    pub schema: [u8; 32],
    pub version: u16,
    pub digest: [u8; 32],
}

impl JobReplayRequest {
    pub fn from_spawn(kind: &str, input: &[u8]) -> Self {
        let controller = kind.split_once('.').map_or(kind, |(controller, _)| controller);
        Self { controller: replay_digest32(controller.as_bytes()), tool: replay_digest32(kind.as_bytes()), schema: replay_digest32(b"semio.kernel.Effect.SpawnJob.v1"), version: 1, digest: replay_digest32_parts(kind.as_bytes(), input) }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_hash32(out, &self.controller).await;
        pack::write_hash32(out, &self.tool).await;
        pack::write_hash32(out, &self.schema).await;
        pack::write_u16(out, self.version).await;
        pack::write_hash32(out, &self.digest).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            controller: pack::read_hash32(bytes, pos, "JobReplayRequest::controller").await?,
            tool: pack::read_hash32(bytes, pos, "JobReplayRequest::tool").await?,
            schema: pack::read_hash32(bytes, pos, "JobReplayRequest::schema").await?,
            version: pack::read_u16(bytes, pos, "JobReplayRequest::version").await?,
            digest: pack::read_hash32(bytes, pos, "JobReplayRequest::digest").await?,
        })
    }
}

impl JobReplayRoute {
    pub const fn zeroed() -> Self {
        Self { plugin: [0; 32], package: [0; 32], controller: [0; 32], tool: [0; 32], window: 0, document: [0; 32], request_schema: [0; 32], request_version: 0, request_digest: [0; 32] }
    }
}

/// 🎯️ Observable P2d disposition recorded after the exact owner crosses the overlay boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub enum JobReplayPublicationPolicy {
    Pending,
    Accepted,
    Displaced,
    Rejected,
}

/// 📡️ Typed publication classification retained without cloning its payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub enum JobReplayPublicationKind {
    Yield,
    Preview,
    Checkpoint,
    Commit,
    Cancelled,
    Fault,
}

impl JobReplayPublicationKind {
    fn from_outcome(outcome: &JobStepOutcome) -> Self {
        match outcome {
            JobStepOutcome::Yield => Self::Yield,
            JobStepOutcome::PreviewReady { .. } => Self::Preview,
            JobStepOutcome::CheckpointReady { .. } => Self::Checkpoint,
            JobStepOutcome::Complete { .. } => Self::Commit,
            JobStepOutcome::Cancelled => Self::Cancelled,
            JobStepOutcome::Fault { .. } => Self::Fault,
        }
    }
}

/// 🚫️ Non-consuming mounted replay refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobReplayFault {
    Busy,
    Cancelled,
    Deadline,
    Capacity,
    PageBytes,
    ProcessPages,
    Stale,
    Sequence,
    Mismatch,
    Exhausted,
}

/// 🔁️ One bounded capture/replay opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobReplayStep {
    MoreWork,
    PublicationReady,
    Refused(JobReplayFault),
    Complete,
}

/// 🧹️ One exact replay close opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobReplayCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

/// 🧩️ One admitted immutable replay payload page.
struct JobReplayPage {
    storage: ManuallyDrop<Option<Box<[MaybeUninit<u8>; JOB_REPLAY_PAGE_BYTES]>>>,
    length: usize,
}

impl JobReplayPage {
    fn try_copy(bytes: &[u8]) -> Result<Self, JobReplayFault> {
        if bytes.len() > JOB_REPLAY_PAGE_BYTES {
            return Err(JobReplayFault::PageBytes);
        }
        JOB_REPLAY_PROCESS_PAGES
            .fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |pages| pages.checked_add(1).filter(|next| *next <= JOB_REPLAY_PROCESS_PAGE_CAPACITY))
            .map_err(|_| JobReplayFault::ProcessPages)?;
        let mut storage = Box::new([MaybeUninit::uninit(); JOB_REPLAY_PAGE_BYTES]);
        for (target, byte) in storage[..bytes.len()].iter_mut().zip(bytes.iter().copied()) {
            target.write(byte);
        }
        Ok(Self { storage: ManuallyDrop::new(Some(storage)), length: bytes.len() })
    }

    fn bytes(&self) -> &[u8] {
        let storage = self.storage.as_ref().expect("sealed replay page owns backing");
        unsafe { std::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), self.length) }
    }

    fn close(&mut self) -> usize {
        let storage = self.storage.take().expect("replay page closes once");
        drop(storage);
        JOB_REPLAY_PROCESS_PAGES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        self.length
    }
}

impl Drop for JobReplayPage {
    fn drop(&mut self) {
        if self.storage.is_none() {
            unsafe { ManuallyDrop::drop(&mut self.storage) };
        } else {
            debug_assert!(false, "replay page requires one-page admitted close");
        }
    }
}

/// 📇️ Scalar identity and digest fields for one deterministic replay turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobReplayRecordHeader {
    pub actor: ActorId,
    pub turn: JobTurn,
    pub ordinal: u64,
    pub granted_fuel: u64,
    pub deadline_class_ms: u32,
    pub worker_count: u16,
    pub worker_slot: u16,
    pub cancellation_observed: bool,
    pub kind: JobReplayPublicationKind,
    pub policy: JobReplayPublicationPolicy,
    pub applied_progress: u64,
    pub payload_items: u8,
    pub payload_bytes: usize,
    pub payload_digest: u64,
    pub prefix_digest: u64,
}

struct JobReplayRecord {
    header: JobReplayRecordHeader,
    pages: ManuallyDrop<[Option<JobReplayPage>; JOB_REPLAY_RECORD_PAGE_CAPACITY]>,
    page_count: usize,
    close_cursor: usize,
}

impl JobReplayRecord {
    fn new(header: JobReplayRecordHeader) -> Self {
        Self { header, pages: ManuallyDrop::new(std::array::from_fn(|_| None)), page_count: 0, close_cursor: 0 }
    }

    fn push_page(&mut self, page: JobReplayPage) -> Result<(), JobReplayPage> {
        let Some(slot) = self.pages.iter_mut().find(|slot| slot.is_none()) else { return Err(page) };
        *slot = Some(page);
        self.page_count += 1;
        Ok(())
    }

    fn close_step(&mut self, maximum_bytes: usize) -> JobReplayCloseStep {
        if self.page_count == 0 {
            return JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let index = self.close_cursor % JOB_REPLAY_RECORD_PAGE_CAPACITY;
        self.close_cursor = (self.close_cursor + 1) % JOB_REPLAY_RECORD_PAGE_CAPACITY;
        let Some(page) = self.pages[index].as_mut() else { return JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 } };
        if page.length > maximum_bytes {
            return JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let released_bytes = page.close();
        self.pages[index] = None;
        self.page_count -= 1;
        JobReplayCloseStep::Pending { released_items: 1, released_bytes }
    }

    fn terminal_is_empty(&self) -> bool {
        self.page_count == 0 && self.pages.iter().all(Option::is_none)
    }
}

impl Drop for JobReplayRecord {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.pages) };
        } else {
            debug_assert!(false, "replay record requires one-page admitted close");
        }
    }
}

struct JobReplayCapture {
    publication: ManuallyDrop<Option<JobPublication>>,
    retirement: ManuallyDrop<Option<JobPublicationOwner>>,
    record: ManuallyDrop<Option<JobReplayRecord>>,
    scalar_cursor: u8,
    payload_item: u8,
    payload_offset: usize,
    payload_digest: u64,
    expected_pages: usize,
    sealed: bool,
    acknowledged: bool,
}

impl JobReplayCapture {
    fn new(publication: JobPublication, header: JobReplayRecordHeader, expected_pages: usize) -> Self {
        Self {
            publication: ManuallyDrop::new(Some(publication)),
            retirement: ManuallyDrop::new(None),
            record: ManuallyDrop::new(Some(JobReplayRecord::new(header))),
            scalar_cursor: 0,
            payload_item: 0,
            payload_offset: 0,
            payload_digest: 0xcbf2_9ce4_8422_2325,
            expected_pages,
            sealed: false,
            acknowledged: false,
        }
    }

    fn payload(&self) -> Option<&[u8]> {
        let publication = self.publication.as_ref().expect("pending replay capture owns publication");
        match (&publication.outcome, self.payload_item) {
            (JobStepOutcome::PreviewReady { preview }, 0) => Some(preview),
            (JobStepOutcome::CheckpointReady { checkpoint }, 0) => Some(&checkpoint.state),
            (JobStepOutcome::Complete { candidate }, 0) => Some(&candidate.state),
            (JobStepOutcome::Complete { candidate }, 1) => Some(&candidate.output),
            (JobStepOutcome::Fault { detail }, 0) => Some(detail),
            _ => None,
        }
    }

    fn payload_items(&self) -> u8 {
        replay_payload_items(self.publication.as_ref().expect("pending replay capture owns publication")) as u8
    }
}

impl Drop for JobReplayCapture {
    fn drop(&mut self) {
        if self.publication.is_none() && self.retirement.is_none() && self.record.is_none() {
            unsafe {
                ManuallyDrop::drop(&mut self.publication);
                ManuallyDrop::drop(&mut self.retirement);
                ManuallyDrop::drop(&mut self.record);
            }
        } else {
            debug_assert!(false, "replay capture requires exact publication handback and record transfer");
        }
    }
}

/// 📮️ Exact live publication retained when replay capture admission refuses.
#[must_use = "rejected replay capture retains the exact live publication"]
#[derive(Debug)]
pub struct JobReplayRejected {
    pub fault: JobReplayFault,
    publication: ManuallyDrop<Option<JobPublication>>,
}

impl JobReplayRejected {
    pub fn into_publication(mut self) -> JobPublication {
        self.publication.take().expect("rejected replay publication returned once")
    }
}

impl Drop for JobReplayRejected {
    fn drop(&mut self) {
        if self.publication.is_none() {
            unsafe { ManuallyDrop::drop(&mut self.publication) };
        } else {
            debug_assert!(false, "rejected replay capture requires exact live-owner handback");
        }
    }
}

/// 📜️ Fixed generation-qualified capture authority for the mounted shard/P2d stream.
pub struct JobReplayLog {
    route: JobReplayRoute,
    generation: u64,
    records: ManuallyDrop<Box<[Option<JobReplayRecord>; JOB_REPLAY_RECORD_CAPACITY]>>,
    record_count: usize,
    reserved_records: usize,
    reserved_pages: usize,
    prefix_digest: u64,
    next_ordinal: u64,
    last_checkpoint_index: Option<usize>,
    pending: ManuallyDrop<Option<JobReplayCapture>>,
    ready_record: Option<usize>,
    replay_cursor: usize,
    replaying: bool,
    closing: bool,
    close_cursor: usize,
}

impl JobReplayLog {
    pub fn new(route: JobReplayRoute, generation: u64) -> Result<Self, JobReplayFault> {
        if generation == 0 {
            return Err(JobReplayFault::Stale);
        }
        Ok(Self {
            route,
            generation,
            records: ManuallyDrop::new(Box::new(std::array::from_fn(|_| None))),
            record_count: 0,
            reserved_records: 0,
            reserved_pages: 0,
            prefix_digest: 0xcbf2_9ce4_8422_2325,
            next_ordinal: 0,
            last_checkpoint_index: None,
            pending: ManuallyDrop::new(None),
            ready_record: None,
            replay_cursor: 0,
            replaying: false,
            closing: false,
            close_cursor: 0,
        })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn sealed_records(&self) -> usize {
        self.record_count
    }

    pub fn route(&self) -> JobReplayRoute {
        self.route
    }

    pub fn prefix_digest(&self) -> u64 {
        self.prefix_digest
    }

    pub fn record_header(&self, index: usize) -> Option<JobReplayRecordHeader> {
        self.records.get(index).and_then(Option::as_ref).map(|record| record.header)
    }

    pub fn last_checkpoint_header(&self) -> Option<JobReplayRecordHeader> {
        self.last_checkpoint_index.and_then(|index| self.record_header(index))
    }

    pub fn begin_capture(&mut self, cx: &mut job::StepContext<'_>, actor: ActorId, worker_count: u16, worker_slot: u16, granted_fuel: u64, deadline_class_ms: u32, publication: JobPublication) -> Result<(), JobReplayRejected> {
        if self.closing || cx.is_cancelled() {
            return Err(replay_rejected(JobReplayFault::Cancelled, publication));
        }
        if cx.should_yield() {
            return Err(replay_rejected(JobReplayFault::Deadline, publication));
        }
        if self.pending.is_some() || self.ready_record.is_some() {
            return Err(replay_rejected(JobReplayFault::Busy, publication));
        }
        if cx.generation().0 != publication.turn.operation.generation || cx.operation().0 != publication.turn.operation.operation || publication.turn.operation.generation != self.generation {
            return Err(replay_rejected(JobReplayFault::Stale, publication));
        }
        if self.next_ordinal == u64::MAX {
            return Err(replay_rejected(JobReplayFault::Exhausted, publication));
        }
        let Some(payload_bytes) = replay_payload_bytes(&publication) else { return Err(replay_rejected(JobReplayFault::Capacity, publication)) };
        let payload_items = replay_payload_items(&publication);
        let expected_pages = replay_payload_pages(&publication);
        if expected_pages > JOB_REPLAY_RECORD_PAGE_CAPACITY || self.record_count + self.reserved_records >= JOB_REPLAY_RECORD_CAPACITY {
            return Err(replay_rejected(JobReplayFault::Capacity, publication));
        }
        cx.consume_fuel(1);
        let header = JobReplayRecordHeader {
            actor,
            turn: publication.turn,
            ordinal: self.next_ordinal,
            granted_fuel,
            deadline_class_ms,
            worker_count,
            worker_slot,
            cancellation_observed: matches!(publication.outcome, JobStepOutcome::Cancelled),
            kind: JobReplayPublicationKind::from_outcome(&publication.outcome),
            policy: JobReplayPublicationPolicy::Pending,
            applied_progress: match &publication.outcome {
                JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.applied_progress,
                _ => publication.turn.step_sequence,
            },
            payload_items: payload_items as u8,
            payload_bytes,
            payload_digest: 0,
            prefix_digest: 0,
        };
        self.reserved_records += 1;
        self.reserved_pages += expected_pages;
        *self.pending = Some(JobReplayCapture::new(publication, header, expected_pages));
        Ok(())
    }

    pub fn capture_step(&mut self, cx: &mut job::StepContext<'_>) -> JobReplayStep {
        if self.pending.is_none() {
            return if self.ready_record.is_some() { JobReplayStep::PublicationReady } else { JobReplayStep::Complete };
        }
        if cx.is_cancelled() {
            return JobReplayStep::Refused(JobReplayFault::Cancelled);
        }
        if cx.should_yield() {
            return JobReplayStep::MoreWork;
        }
        let pending = self.pending.as_mut().expect("capture pending");
        if pending.acknowledged {
            return JobReplayStep::MoreWork;
        }
        if pending.scalar_cursor < JOB_REPLAY_SCALAR_TRANSFERS {
            cx.consume_fuel(1);
            pending.scalar_cursor += 1;
            return JobReplayStep::MoreWork;
        }
        if pending.payload_item < pending.payload_items() {
            let Some(payload) = pending.payload() else { return JobReplayStep::Refused(JobReplayFault::Mismatch) };
            if pending.payload_offset == payload.len() {
                cx.consume_fuel(1);
                pending.payload_item += 1;
                pending.payload_offset = 0;
                return JobReplayStep::MoreWork;
            }
            let end = pending.payload_offset.saturating_add(JOB_REPLAY_PAGE_BYTES).min(payload.len());
            cx.consume_fuel(1);
            if cx.is_cancelled() || cx.deadline_exceeded() {
                return JobReplayStep::Refused(if cx.is_cancelled() { JobReplayFault::Cancelled } else { JobReplayFault::Deadline });
            }
            let (page, payload_digest) = {
                let chunk = &payload[pending.payload_offset..end];
                let page = match JobReplayPage::try_copy(chunk) {
                    Ok(page) => page,
                    Err(fault) => return JobReplayStep::Refused(fault),
                };
                let payload_digest = chunk.iter().fold(pending.payload_digest, |digest, byte| replay_mix(digest, u64::from(*byte)));
                (page, payload_digest)
            };
            pending.payload_digest = payload_digest;
            pending.payload_offset = end;
            if let Err(mut page) = pending.record.as_mut().expect("capture owns record").push_page(page) {
                let _ = page.close();
                return JobReplayStep::Refused(JobReplayFault::Capacity);
            }
            return JobReplayStep::MoreWork;
        }
        cx.consume_fuel(1);
        if pending.sealed {
            return JobReplayStep::PublicationReady;
        }
        let record = pending.record.as_mut().expect("capture owns record");
        if record.page_count != pending.expected_pages {
            return JobReplayStep::Refused(JobReplayFault::Mismatch);
        }
        record.header.payload_digest = pending.payload_digest;
        record.header.prefix_digest = replay_header_digest(self.prefix_digest, &self.route, &record.header);
        if self.replaying {
            let Some(expected) = self.records.get(self.replay_cursor).and_then(Option::as_ref) else { return JobReplayStep::Refused(JobReplayFault::Sequence) };
            if !replay_record_matches(expected, record) {
                return JobReplayStep::Refused(JobReplayFault::Mismatch);
            }
            let Some(next_ordinal) = self.next_ordinal.checked_add(1) else { return JobReplayStep::Refused(JobReplayFault::Exhausted) };
            self.ready_record = Some(self.replay_cursor);
            self.prefix_digest = expected.header.prefix_digest;
            self.next_ordinal = next_ordinal;
            self.replay_cursor += 1;
        } else {
            let index = self.record_count;
            self.records[index] = pending.record.take();
            self.record_count += 1;
            if self.records[index].as_ref().is_some_and(|record| record.header.kind == JobReplayPublicationKind::Checkpoint) {
                self.last_checkpoint_index = Some(index);
            }
            self.next_ordinal = self.next_ordinal.checked_add(1).expect("ordinal exhaustion preflighted");
            self.prefix_digest = self.records[index].as_ref().expect("sealed replay record").header.prefix_digest;
            self.reserved_records -= 1;
            self.reserved_pages -= pending.expected_pages;
            self.ready_record = Some(index);
        }
        pending.sealed = true;
        JobReplayStep::PublicationReady
    }

    pub fn take_captured_publication(&mut self) -> Option<JobPublication> {
        self.ready_record?;
        self.pending.as_mut()?.publication.take()
    }

    pub fn publication_is_ready(&self) -> bool {
        self.ready_record.is_some() && self.pending.as_ref().is_some_and(|pending| pending.publication.is_some())
    }

    pub fn acknowledge_publication(&mut self, cx: &mut job::StepContext<'_>, policy: JobReplayPublicationPolicy) -> Result<(), JobReplayFault> {
        if cx.is_cancelled() || cx.should_yield() {
            return Err(if cx.is_cancelled() { JobReplayFault::Cancelled } else { JobReplayFault::Deadline });
        }
        cx.consume_fuel(1);
        let index = self.ready_record.take().ok_or(JobReplayFault::Sequence)?;
        if policy == JobReplayPublicationPolicy::Pending {
            self.ready_record = Some(index);
            return Err(JobReplayFault::Mismatch);
        }
        if self.replaying {
            if self.records[index].as_ref().ok_or(JobReplayFault::Sequence)?.header.policy != policy {
                self.ready_record = Some(index);
                return Err(JobReplayFault::Mismatch);
            }
        } else {
            self.records[index].as_mut().ok_or(JobReplayFault::Sequence)?.header.policy = policy;
        }
        let pending = self.pending.as_mut().ok_or(JobReplayFault::Sequence)?;
        if self.replaying {
            pending.acknowledged = true;
            return Ok(());
        }
        let pending = self.pending.take().ok_or(JobReplayFault::Sequence)?;
        drop(pending);
        Ok(())
    }

    pub fn maintenance_step(&mut self, cx: &mut job::StepContext<'_>) -> JobReplayStep {
        let Some(pending) = self.pending.as_mut() else { return JobReplayStep::Complete };
        if !pending.acknowledged {
            return self.capture_step(cx);
        }
        if cx.should_yield() {
            return JobReplayStep::MoreWork;
        }
        cx.consume_fuel(1);
        if let Some(record) = pending.record.as_mut() {
            if !record.terminal_is_empty() {
                let before = record.page_count;
                let _ = record.close_step(JOB_REPLAY_PAGE_BYTES);
                self.reserved_pages -= before.saturating_sub(record.page_count);
                return JobReplayStep::MoreWork;
            }
            drop(pending.record.take());
            self.reserved_records -= 1;
            return JobReplayStep::MoreWork;
        }
        let pending = self.pending.take().expect("acknowledged replay candidate");
        drop(pending);
        JobReplayStep::Complete
    }

    pub fn has_pending_work(&self) -> bool {
        self.pending.is_some() || self.ready_record.is_some() || (self.closing && !self.terminal_is_empty())
    }

    pub fn close_started(&self) -> bool {
        self.closing
    }

    pub fn begin_replay(&mut self, generation: u64) -> Result<(), JobReplayFault> {
        if generation != self.generation || self.pending.is_some() || self.ready_record.is_some() || self.closing {
            return Err(JobReplayFault::Stale);
        }
        self.replaying = true;
        self.replay_cursor = 0;
        self.prefix_digest = 0xcbf2_9ce4_8422_2325;
        self.next_ordinal = 0;
        Ok(())
    }

    pub fn expected_turn(&self) -> Option<JobTurn> {
        self.replaying.then(|| self.records.get(self.replay_cursor).and_then(Option::as_ref).map(|record| record.header.turn)).flatten()
    }

    pub fn replay_is_complete(&self) -> bool {
        self.replaying && self.replay_cursor == self.record_count && self.pending.is_none() && self.ready_record.is_none()
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self, cx: &mut job::StepContext<'_>) -> JobReplayCloseStep {
        self.closing = true;
        if cx.is_cancelled() || cx.should_yield() {
            return JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        cx.consume_fuel(1);
        if let Some(pending) = self.pending.as_mut() {
            if let Some(record) = pending.record.as_mut() {
                if !record.terminal_is_empty() {
                    return record.close_step(JOB_REPLAY_PAGE_BYTES);
                }
                drop(pending.record.take());
                return JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if pending.retirement.is_none() {
                if let Some(publication) = pending.publication.take() {
                    let total_phases = replay_payload_items(&publication) as u8 + 1;
                    *pending.retirement = Some(JobPublicationOwner { publication, phase: 0, total_phases, owner_bytes: 0 });
                    return JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
            }
            if let Some(owner) = pending.retirement.as_mut() {
                if !owner.terminal_is_empty() {
                    let (complete, released_items, released_bytes) = owner.close_one();
                    if !complete {
                        return JobReplayCloseStep::Pending { released_items, released_bytes };
                    }
                }
                *pending.retirement = None;
                return JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            let pending = self.pending.take().expect("terminal replay capture");
            drop(pending);
            self.reserved_records = 0;
            self.reserved_pages = 0;
            return JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let index = self.close_cursor % JOB_REPLAY_RECORD_CAPACITY;
        self.close_cursor = (self.close_cursor + 1) % JOB_REPLAY_RECORD_CAPACITY;
        let Some(record) = self.records[index].as_mut() else {
            return if self.terminal_is_empty() { JobReplayCloseStep::Complete } else { JobReplayCloseStep::Pending { released_items: 0, released_bytes: 0 } };
        };
        if !record.terminal_is_empty() {
            return record.close_step(JOB_REPLAY_PAGE_BYTES);
        }
        self.records[index] = None;
        self.record_count -= 1;
        JobReplayCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.record_count == 0 && self.reserved_records == 0 && self.reserved_pages == 0 && self.pending.is_none() && self.ready_record.is_none() && self.records.iter().all(Option::is_none)
    }
}

impl Drop for JobReplayLog {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            unsafe {
                ManuallyDrop::drop(&mut self.records);
                ManuallyDrop::drop(&mut self.pending);
            }
        } else {
            debug_assert!(false, "mounted replay log requires generation-qualified recovery and incremental close");
        }
    }
}

fn replay_rejected(fault: JobReplayFault, publication: JobPublication) -> JobReplayRejected {
    JobReplayRejected { fault, publication: ManuallyDrop::new(Some(publication)) }
}

fn replay_payload_items(publication: &JobPublication) -> usize {
    match &publication.outcome {
        JobStepOutcome::PreviewReady { .. } | JobStepOutcome::CheckpointReady { .. } | JobStepOutcome::Fault { .. } => 1,
        JobStepOutcome::Complete { .. } => 2,
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => 0,
    }
}

fn replay_payload_bytes(publication: &JobPublication) -> Option<usize> {
    match &publication.outcome {
        JobStepOutcome::PreviewReady { preview } => Some(preview.len()),
        JobStepOutcome::CheckpointReady { checkpoint } => Some(checkpoint.state.len()),
        JobStepOutcome::Complete { candidate } => candidate.state.len().checked_add(candidate.output.len()),
        JobStepOutcome::Fault { detail } => Some(detail.len()),
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => Some(0),
    }
}

fn replay_payload_pages(publication: &JobPublication) -> usize {
    match &publication.outcome {
        JobStepOutcome::PreviewReady { preview } => preview.len().div_ceil(JOB_REPLAY_PAGE_BYTES),
        JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.len().div_ceil(JOB_REPLAY_PAGE_BYTES),
        JobStepOutcome::Complete { candidate } => candidate.state.len().div_ceil(JOB_REPLAY_PAGE_BYTES) + candidate.output.len().div_ceil(JOB_REPLAY_PAGE_BYTES),
        JobStepOutcome::Fault { detail } => detail.len().div_ceil(JOB_REPLAY_PAGE_BYTES),
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => 0,
    }
}

fn replay_mix(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x0000_0100_0000_01b3)
}

fn replay_digest32(bytes: &[u8]) -> [u8; 32] {
    replay_digest32_parts(bytes, &[])
}

fn replay_digest32_parts(left: &[u8], right: &[u8]) -> [u8; 32] {
    let mut digest = [0u8; 32];
    for lane in 0..4 {
        let mut value = 0xcbf2_9ce4_8422_2325 ^ lane as u64;
        for byte in left.iter().copied().chain([0xff]).chain(right.iter().copied()) {
            value = replay_mix(value, u64::from(byte));
        }
        digest[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    digest
}

fn replay_header_digest(mut digest: u64, route: &JobReplayRoute, header: &JobReplayRecordHeader) -> u64 {
    for byte in route.plugin.iter().chain(route.package.iter()).chain(route.controller.iter()).chain(route.tool.iter()).chain(route.document.iter()).chain(route.request_schema.iter()).chain(route.request_digest.iter()) {
        digest = replay_mix(digest, u64::from(*byte));
    }
    for value in [
        route.window,
        u64::from(route.request_version),
        header.actor.0,
        header.turn.job,
        header.turn.operation.operation,
        header.turn.operation.base_revision,
        header.turn.operation.generation,
        header.turn.operation.seed,
        header.turn.step_sequence,
        header.ordinal,
        header.granted_fuel,
        u64::from(header.deadline_class_ms),
        u64::from(header.worker_count),
        u64::from(header.worker_slot),
        u64::from(header.cancellation_observed),
        match header.kind {
            JobReplayPublicationKind::Yield => 0,
            JobReplayPublicationKind::Preview => 1,
            JobReplayPublicationKind::Checkpoint => 2,
            JobReplayPublicationKind::Commit => 3,
            JobReplayPublicationKind::Cancelled => 4,
            JobReplayPublicationKind::Fault => 5,
        },
        header.applied_progress,
        u64::from(header.payload_items),
        header.payload_bytes as u64,
        header.payload_digest,
    ] {
        digest = replay_mix(digest, value);
    }
    digest
}

fn replay_record_matches(expected: &JobReplayRecord, actual: &JobReplayRecord) -> bool {
    expected.header.actor == actual.header.actor
        && expected.header.turn == actual.header.turn
        && expected.header.ordinal == actual.header.ordinal
        && expected.header.granted_fuel == actual.header.granted_fuel
        && expected.header.deadline_class_ms == actual.header.deadline_class_ms
        && expected.header.worker_count == actual.header.worker_count
        && expected.header.worker_slot == actual.header.worker_slot
        && expected.header.cancellation_observed == actual.header.cancellation_observed
        && expected.header.kind == actual.header.kind
        && expected.header.applied_progress == actual.header.applied_progress
        && expected.header.payload_items == actual.header.payload_items
        && expected.header.payload_bytes == actual.header.payload_bytes
        && expected.header.payload_digest == actual.header.payload_digest
        && expected.header.prefix_digest == actual.header.prefix_digest
        && expected.page_count == actual.page_count
        && expected.pages.iter().flatten().zip(actual.pages.iter().flatten()).all(|(left, right)| left.bytes() == right.bytes())
}
//#endregion 📜️FixedReplay

/// 🚫️ Publication validation failure detected before actor-visible state can change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JobPublicationError {
    OperationMismatch { active: u64, published: u64 },
    Stale { live_revision: u64, live_generation: u64 },
    StepSequence { expected: u64, published: u64 },
    PreviewSequence { before: u64, after: u64 },
    Terminal,
}

impl std::fmt::Display for JobPublicationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OperationMismatch { active, published } => write!(formatter, "job bridge operation mismatch: active={active}, published={published}"),
            Self::Stale { live_revision, live_generation } => write!(formatter, "job bridge stale revision/generation: live revision={live_revision}, generation={live_generation}"),
            Self::StepSequence { expected, published } => write!(formatter, "job bridge step sequence mismatch: expected={expected}, published={published}"),
            Self::PreviewSequence { before, after } => write!(formatter, "job bridge preview cursor mismatch: before={before}, after={after}"),
            Self::Terminal => formatter.write_str("job bridge received a turn after a terminal publication"),
        }
    }
}

impl std::error::Error for JobPublicationError {}

struct JobPayloadProjection {
    owner: Option<job::RetainedJobPayload>,
    bytes: Vec<u8>,
    page: usize,
}

impl JobPayloadProjection {
    fn new(owner: job::RetainedJobPayload) -> Self {
        let bytes = Vec::with_capacity(owner.len());
        Self { owner: Some(owner), bytes, page: 0 }
    }

    fn step(&mut self) -> (bool, bool) {
        let Some(owner) = self.owner.as_mut() else { return (true, false) };
        if owner.terminal_is_empty() {
            drop(self.owner.take());
            return (true, false);
        }
        let Some(page) = owner.page(self.page) else {
            self.page = self.page.saturating_add(1);
            return (false, false);
        };
        let length = page.len();
        self.bytes.extend_from_slice(page);
        self.page = self.page.saturating_add(1);
        let step = owner.close_step(1, length);
        assert!(matches!(step, job::JobPayloadCloseStep::Pending { released_items: 1, released_bytes } if released_bytes == length));
        let complete = owner.terminal_is_empty();
        if complete {
            drop(self.owner.take());
        }
        (complete, true)
    }

    fn take_bytes(&mut self) -> Vec<u8> {
        assert!(self.owner.is_none(), "projected job payload must be terminal before publication");
        std::mem::take(&mut self.bytes)
    }
}

enum JobOutcomeProjection {
    Preview { payload: JobPayloadProjection },
    Checkpoint { state: JobPayloadProjection, applied_progress: u64 },
    Complete { state: JobPayloadProjection, output: JobPayloadProjection, state_bytes: Option<Vec<u8>> },
    Fault { detail: JobPayloadProjection },
}

impl JobOutcomeProjection {
    fn start(outcome: job::StepOutcome) -> Result<JobStepOutcome, Self> {
        match outcome {
            job::StepOutcome::Yield => Ok(JobStepOutcome::Yield),
            job::StepOutcome::PreviewReady(preview) => Err(Self::Preview { payload: JobPayloadProjection::new(preview) }),
            job::StepOutcome::CheckpointReady(checkpoint) => Err(Self::Checkpoint { state: JobPayloadProjection::new(checkpoint.state), applied_progress: checkpoint.applied_progress }),
            job::StepOutcome::Complete(candidate) => Err(Self::Complete { state: JobPayloadProjection::new(candidate.state), output: JobPayloadProjection::new(candidate.output), state_bytes: None }),
            job::StepOutcome::Cancelled => Ok(JobStepOutcome::Cancelled),
            job::StepOutcome::Fault(fault) => Err(Self::Fault { detail: JobPayloadProjection::new(fault.detail) }),
        }
    }

    fn step(&mut self) -> Option<JobStepOutcome> {
        match self {
            Self::Preview { payload } => {
                let (complete, _) = payload.step();
                complete.then(|| JobStepOutcome::PreviewReady { preview: payload.take_bytes() })
            }
            Self::Checkpoint { state, applied_progress } => {
                let (complete, _) = state.step();
                complete.then(|| JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: state.take_bytes(), applied_progress: *applied_progress } })
            }
            Self::Complete { state, output, state_bytes } => {
                if state_bytes.is_none() {
                    let (complete, progressed) = state.step();
                    if !complete {
                        return None;
                    }
                    *state_bytes = Some(state.take_bytes());
                    if progressed {
                        return None;
                    }
                }
                let (complete, _) = output.step();
                complete.then(|| JobStepOutcome::Complete { candidate: JobCommitCandidate { state: state_bytes.take().expect("projected state"), output: output.take_bytes() } })
            }
            Self::Fault { detail } => {
                let (complete, _) = detail.step();
                complete.then(|| JobStepOutcome::Fault { detail: detail.take_bytes() })
            }
        }
    }
}

/// 🌉️ Stateful actor bridge that advances at most one interactive job step or one retained payload page per turn.
#[cfg(test)]
pub struct JobTurnBridge {
    operation: job::Operation,
    next_step_sequence: u64,
    terminal: bool,
    pending: Option<JobOutcomeProjection>,
}

#[cfg(test)]
impl JobTurnBridge {
    pub fn new(operation: job::Operation) -> Self {
        Self { operation, next_step_sequence: 0, terminal: false, pending: None }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn step<J: job::InteractiveJob + ?Sized>(
        &mut self,
        job: &mut J,
        turn: JobTurn,
        active_operation: job::OperationId,
        live_revision: job::RevisionId,
        live_generation: job::Generation,
        site: &'static str,
        stage: job::InteractiveStage,
        budget: job::StepBudget,
        cancel: job::CancelToken,
        now_us: fn() -> Option<u64>,
    ) -> Result<JobPublication, JobPublicationError> {
        if self.terminal {
            return Err(JobPublicationError::Terminal);
        }
        if turn.operation.operation != active_operation.0 || self.operation.operation != active_operation {
            return Err(JobPublicationError::OperationMismatch { active: active_operation.0, published: turn.operation.operation });
        }
        if turn.step_sequence != self.next_step_sequence {
            return Err(JobPublicationError::StepSequence { expected: self.next_step_sequence, published: turn.step_sequence });
        }
        if !matches!(job::validate_commit(&self.operation, live_revision, live_generation), job::CommitValidation::Accepted)
            || turn.operation.base_revision != live_revision.0
            || turn.operation.generation != live_generation.0
            || turn.operation.base_revision != self.operation.base_revision.0
            || turn.operation.generation != self.operation.generation.0
            || turn.operation.preview_sequence != self.operation.preview_sequence
            || turn.operation.seed != self.operation.seed
        {
            return Err(JobPublicationError::Stale { live_revision: live_revision.0, live_generation: live_generation.0 });
        }
        let outcome = if let Some(projection) = self.pending.as_mut() {
            match projection.step() {
                Some(outcome) => {
                    self.pending = None;
                    outcome
                }
                None => JobStepOutcome::Yield,
            }
        } else {
            let before = self.operation.preview_sequence;
            let outcome = job::drive_step(job, site, self.operation.operation, self.operation.generation, stage, budget, cancel, now_us, &mut self.operation.preview_sequence, &mut None);
            let after = self.operation.preview_sequence;
            let preview = matches!(outcome, job::StepOutcome::PreviewReady(_));
            if (preview && after != before.saturating_add(1)) || (!preview && after != before) {
                return Err(JobPublicationError::PreviewSequence { before, after });
            }
            match JobOutcomeProjection::start(outcome) {
                Ok(outcome) => outcome,
                Err(projection) => {
                    self.pending = Some(projection);
                    JobStepOutcome::Yield
                }
            }
        };
        let terminal = matches!(outcome, JobStepOutcome::Complete { .. } | JobStepOutcome::Cancelled | JobStepOutcome::Fault { .. });
        let publication = JobPublication { turn: JobTurn { operation: JobOperation::from_job(self.operation), ..turn }, outcome };
        self.next_step_sequence += 1;
        self.terminal = terminal;
        Ok(publication)
    }
}
//#endregion 🪪️JobBridge

//#region 🎨️JobProgressOverlay
pub const JOB_PROGRESS_ACTIVE_CAPACITY: usize = 64;
pub const JOB_PROGRESS_RETIREMENT_CAPACITY: usize = 128;
pub const JOB_PROGRESS_PAGE_MAXIMUM_BYTES: usize = 16 * 1024;
pub const JOB_PROGRESS_TOTAL_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;
pub const JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS: usize = 512;

/// 🪪️ Exact identity of one actor-visible job progress publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobProgressIdentity {
    pub actor: ActorId,
    pub job: u64,
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
    pub step_sequence: u64,
    pub preview_sequence: u64,
}

impl JobProgressIdentity {
    pub fn from_publication(actor: ActorId, publication: &JobPublication) -> Self {
        Self {
            actor,
            job: publication.turn.job,
            operation: publication.turn.operation.operation,
            base_revision: publication.turn.operation.base_revision,
            generation: publication.turn.operation.generation,
            step_sequence: publication.turn.step_sequence,
            preview_sequence: publication.turn.operation.preview_sequence,
        }
    }

    fn same_job(self, other: Self) -> bool {
        self.actor == other.actor && self.job == other.job
    }

    fn same_operation(self, other: Self) -> bool {
        self.same_job(other) && self.operation == other.operation && self.base_revision == other.base_revision && self.generation == other.generation
    }
}

/// 📡️ Scalar, coalescible progress kind retained separately from committed actor scenes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressKind {
    Yield,
    Preview,
    Checkpoint,
    CommitValidated,
    Cancelled,
    Fault,
}

impl JobProgressKind {
    fn of(outcome: &JobStepOutcome) -> Self {
        match outcome {
            JobStepOutcome::Yield => Self::Yield,
            JobStepOutcome::PreviewReady { .. } => Self::Preview,
            JobStepOutcome::CheckpointReady { .. } => Self::Checkpoint,
            JobStepOutcome::Complete { .. } => Self::CommitValidated,
            JobStepOutcome::Cancelled => Self::Cancelled,
            JobStepOutcome::Fault { .. } => Self::Fault,
        }
    }

    fn terminal(self) -> bool {
        matches!(self, Self::CommitValidated | Self::Cancelled | Self::Fault)
    }
}

/// 🔐️ Independent revision/generation authority supplied immediately before publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobProgressLiveAuthority {
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
}

impl JobProgressLiveAuthority {
    pub fn new(operation: u64, base_revision: u64, generation: u64) -> Self {
        Self { operation, base_revision, generation }
    }

    fn accepts(self, identity: JobProgressIdentity) -> bool {
        self.operation == identity.operation && self.base_revision == identity.base_revision && self.generation == identity.generation
    }
}

/// 🚫️ Fail-closed reason returned without consuming a rejected publication owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressFault {
    Cancelled,
    Budget,
    Capacity,
    Oversized,
    Stale,
    StepSequence,
    PreviewSequence,
    Busy,
    Token,
    Missing,
    CheckedOut,
    Exhausted,
}

impl std::fmt::Display for JobProgressFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "job progress overlay fault: {self:?}")
    }
}

impl std::error::Error for JobProgressFault {}

/// 📮️ Exact rejected publication owner; callers must return or close it rather than recreate input.
#[must_use = "a rejected job publication retains the exact payload owner"]
#[derive(Debug)]
pub struct JobProgressRejected {
    fault: JobProgressFault,
    publication: Option<JobPublication>,
}

impl JobProgressRejected {
    pub fn new(fault: JobProgressFault, publication: JobPublication) -> Self {
        Self { fault, publication: Some(publication) }
    }

    pub fn fault(&self) -> JobProgressFault {
        self.fault
    }

    pub fn publication(&self) -> &JobPublication {
        self.publication.as_ref().expect("rejected job publication owner already returned")
    }

    pub fn into_publication(mut self) -> JobPublication {
        self.publication.take().expect("rejected job publication owner already returned")
    }
}

#[derive(Debug)]
struct JobPublicationOwner {
    publication: JobPublication,
    phase: u8,
    total_phases: u8,
    owner_bytes: usize,
}

impl JobPublicationOwner {
    fn try_new(publication: JobPublication) -> Result<Self, (JobProgressFault, JobPublication)> {
        match publication_owner_shape(&publication, false) {
            Ok((_, owner_bytes, total_phases)) => Ok(Self { publication, phase: 0, total_phases, owner_bytes }),
            Err(fault) => Err((fault, publication)),
        }
    }

    fn close_one(&mut self) -> (bool, usize, usize) {
        let released_bytes = match (&mut self.publication.outcome, self.phase) {
            (JobStepOutcome::PreviewReady { preview }, 0) => std::mem::take(preview).capacity(),
            (JobStepOutcome::CheckpointReady { checkpoint }, 0) => std::mem::take(&mut checkpoint.state).capacity(),
            (JobStepOutcome::Complete { candidate }, 0) => std::mem::take(&mut candidate.state).capacity(),
            (JobStepOutcome::Complete { candidate }, 1) => std::mem::take(&mut candidate.output).capacity(),
            (JobStepOutcome::Fault { detail }, 0) => std::mem::take(detail).capacity(),
            _ => 0,
        };
        self.phase = self.phase.saturating_add(1);
        self.owner_bytes = self.owner_bytes.saturating_sub(released_bytes);
        (self.phase >= self.total_phases, 1, released_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase >= self.total_phases
            && self.owner_bytes == 0
            && match &self.publication.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.capacity() == 0,
                JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.capacity() == 0,
                JobStepOutcome::Complete { candidate } => candidate.state.capacity() == 0 && candidate.output.capacity() == 0,
                JobStepOutcome::Fault { detail } => detail.capacity() == 0,
                JobStepOutcome::Yield | JobStepOutcome::Cancelled => true,
            }
    }
}

fn publication_owner_shape(publication: &JobPublication, enforce_page_limit: bool) -> Result<(usize, usize, u8), JobProgressFault> {
    let capacities = match &publication.outcome {
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => ([0usize, 0usize], 0usize),
        JobStepOutcome::PreviewReady { preview } => ([preview.capacity(), 0], 1),
        JobStepOutcome::CheckpointReady { checkpoint } => ([checkpoint.state.capacity(), 0], 1),
        JobStepOutcome::Complete { candidate } => ([candidate.state.capacity(), candidate.output.capacity()], 2),
        JobStepOutcome::Fault { detail } => ([detail.capacity(), 0], 1),
    };
    if enforce_page_limit && capacities.0[..capacities.1].iter().any(|capacity| *capacity > JOB_PROGRESS_PAGE_MAXIMUM_BYTES) {
        return Err(JobProgressFault::Oversized);
    }
    let owner_bytes = capacities.0[..capacities.1].iter().try_fold(0usize, |total, capacity| total.checked_add(*capacity)).ok_or(JobProgressFault::Capacity)?;
    let total_phases = u8::try_from(capacities.1.checked_add(1).ok_or(JobProgressFault::Capacity)?).map_err(|_| JobProgressFault::Capacity)?;
    Ok((capacities.1 + 1, owner_bytes, total_phases))
}

#[derive(Debug)]
struct JobProgressSlot {
    epoch: u64,
    occupied: bool,
    published: bool,
    awaiting_ack: bool,
    checked_out: bool,
    terminal: bool,
    reservation: Option<u64>,
    identity: JobProgressIdentity,
    kind: JobProgressKind,
    applied_progress: u64,
    preview: Option<JobPublicationOwner>,
}

impl JobProgressSlot {
    fn vacant() -> Self {
        Self {
            epoch: 0,
            occupied: false,
            published: false,
            awaiting_ack: false,
            checked_out: false,
            terminal: false,
            reservation: None,
            identity: JobProgressIdentity { actor: ActorId(0), job: 0, operation: 0, base_revision: 0, generation: 0, step_sequence: 0, preview_sequence: 0 },
            kind: JobProgressKind::Yield,
            applied_progress: 0,
            preview: None,
        }
    }

    fn clear(&mut self) {
        let epoch = self.epoch;
        *self = Self::vacant();
        self.epoch = epoch;
    }
}

#[derive(Debug)]
struct JobProgressRetirementSlot {
    reservation: Option<u64>,
    owner: Option<JobPublicationOwner>,
}

#[derive(Clone, Copy, Debug)]
struct JobProgressPriorState {
    published: bool,
    terminal: bool,
    identity: JobProgressIdentity,
    kind: JobProgressKind,
    applied_progress: u64,
}

impl JobProgressRetirementSlot {
    fn vacant() -> Self {
        Self { reservation: None, owner: None }
    }
}

/// 🎟️ Linear reservation made before the publication owner crosses into the overlay.
#[must_use = "job progress admission must be published or cancelled"]
#[derive(Debug)]
pub struct JobProgressAdmission {
    token: u64,
    active_index: usize,
    active_epoch: u64,
    identity: JobProgressIdentity,
    live: JobProgressLiveAuthority,
    retirement_indices: [usize; 2],
    abort_retirement: Option<usize>,
    owner_items: usize,
    owner_bytes: usize,
}

/// ✅️ Linear ACK/abort authority for one adopted publication.
#[must_use = "an adopted job publication must be acknowledged or aborted"]
#[derive(Debug)]
pub struct JobProgressReceipt {
    identity: JobProgressIdentity,
    kind: JobProgressKind,
    applied_progress: u64,
    owner_bytes: usize,
    active_index: usize,
    active_epoch: u64,
    prior: JobProgressPriorState,
    displaced_retirement: Option<usize>,
    abort_retirement: Option<(usize, u64)>,
}

impl JobProgressReceipt {
    pub fn identity(&self) -> JobProgressIdentity {
        self.identity
    }

    pub fn kind(&self) -> JobProgressKind {
        self.kind
    }

    pub fn applied_progress(&self) -> u64 {
        self.applied_progress
    }

    pub fn owner_bytes(&self) -> usize {
        self.owner_bytes
    }
}

/// 🧹️ One truthful cleanup opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Blocked,
    Complete,
}

/// 🎨️ Fixed actor-owned preview/progress authority; committed scenes are never stored here.
pub struct JobProgressOverlayStore {
    active: [JobProgressSlot; JOB_PROGRESS_ACTIVE_CAPACITY],
    retirements: [JobProgressRetirementSlot; JOB_PROGRESS_RETIREMENT_CAPACITY],
    closing_actors: [Option<ActorId>; JOB_PROGRESS_ACTIVE_CAPACITY],
    close_all: bool,
    next_token: u64,
    owned_items: usize,
    owned_bytes: usize,
    reserved_items: usize,
    reserved_bytes: usize,
}

impl Default for JobProgressOverlayStore {
    fn default() -> Self {
        Self::new()
    }
}

impl JobProgressOverlayStore {
    pub fn new() -> Self {
        Self {
            active: std::array::from_fn(|_| JobProgressSlot::vacant()),
            retirements: std::array::from_fn(|_| JobProgressRetirementSlot::vacant()),
            closing_actors: [None; JOB_PROGRESS_ACTIVE_CAPACITY],
            close_all: false,
            next_token: 1,
            owned_items: 0,
            owned_bytes: 0,
            reserved_items: 0,
            reserved_bytes: 0,
        }
    }

    pub fn live_authority(&self, actor: ActorId, job: u64) -> Option<JobProgressLiveAuthority> {
        self.active.iter().find(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job).map(|slot| JobProgressLiveAuthority::new(slot.identity.operation, slot.identity.base_revision, slot.identity.generation))
    }

    pub fn has_visible_preview(&self, actor: ActorId, job: u64) -> bool {
        self.active.iter().any(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job && slot.preview.is_some())
    }

    pub fn owned_items(&self) -> usize {
        self.owned_items
    }

    pub fn owned_bytes(&self) -> usize {
        self.owned_bytes
    }

    pub fn begin_operation(&mut self, actor: ActorId, job: u64, live: JobProgressLiveAuthority) -> Result<(), JobProgressFault> {
        if self.close_all || self.closing_actors.iter().flatten().any(|closing| *closing == actor) {
            return Err(JobProgressFault::Cancelled);
        }
        if self.active.iter().any(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job) {
            return Err(JobProgressFault::Busy);
        }
        let slot = self.active.iter_mut().find(|slot| !slot.occupied && slot.reservation.is_none()).ok_or(JobProgressFault::Capacity)?;
        slot.epoch = slot.epoch.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
        slot.occupied = true;
        slot.published = false;
        slot.identity = JobProgressIdentity { actor, job, operation: live.operation, base_revision: live.base_revision, generation: live.generation, step_sequence: 0, preview_sequence: 0 };
        Ok(())
    }

    pub fn preflight(&mut self, cx: &mut job::StepContext<'_>, actor: ActorId, publication: &JobPublication, live: JobProgressLiveAuthority) -> Result<JobProgressAdmission, JobProgressFault> {
        let identity = JobProgressIdentity::from_publication(actor, publication);
        if cx.is_cancelled() || self.close_all || self.closing_actors.iter().flatten().any(|closing| *closing == actor) {
            return Err(JobProgressFault::Cancelled);
        }
        if cx.should_yield() {
            return Err(JobProgressFault::Budget);
        }
        if cx.operation().0 != identity.operation || cx.generation().0 != identity.generation || !live.accepts(identity) {
            return Err(JobProgressFault::Stale);
        }
        let (owner_items, owner_bytes, _) = publication_owner_shape(publication, true)?;
        let kind = JobProgressKind::of(&publication.outcome);
        let active_index = self.active.iter().position(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == publication.turn.job).ok_or(JobProgressFault::Missing)?;
        let active_epoch = self.active[active_index].epoch;
        if self.active[active_index].checked_out {
            return Err(JobProgressFault::CheckedOut);
        }
        if self.active[active_index].awaiting_ack || self.active[active_index].reservation.is_some() {
            return Err(JobProgressFault::Busy);
        }
        if active_epoch == u64::MAX {
            return Err(JobProgressFault::Exhausted);
        }
        if !self.active[active_index].identity.same_operation(identity) {
            return Err(JobProgressFault::Stale);
        }
        let prior_preview_sequence = if self.active[active_index].published {
            let expected_step = self.active[active_index].identity.step_sequence.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
            if identity.step_sequence != expected_step {
                return Err(JobProgressFault::StepSequence);
            }
            self.active[active_index].identity.preview_sequence
        } else {
            if identity.step_sequence != 0 {
                return Err(JobProgressFault::StepSequence);
            }
            0
        };
        let expected_preview = if kind == JobProgressKind::Preview { prior_preview_sequence.checked_add(1).ok_or(JobProgressFault::Exhausted)? } else { prior_preview_sequence };
        if identity.preview_sequence != expected_preview {
            return Err(JobProgressFault::PreviewSequence);
        }
        let has_preview = self.active[active_index].preview.is_some();
        let adoption_retirements = match kind {
            JobProgressKind::Preview => usize::from(has_preview),
            JobProgressKind::Checkpoint => 1,
            JobProgressKind::CommitValidated | JobProgressKind::Fault => 1 + usize::from(has_preview),
            JobProgressKind::Cancelled => usize::from(has_preview),
            JobProgressKind::Yield => 0,
        };
        let staged_preview = kind == JobProgressKind::Preview;
        let retirement_count = adoption_retirements + usize::from(staged_preview);
        if retirement_count > 2 {
            return Err(JobProgressFault::Capacity);
        }
        let mut retirement_indices = [usize::MAX; 2];
        let mut found = 0usize;
        for (index, retirement) in self.retirements.iter().enumerate() {
            if retirement.owner.is_none() && retirement.reservation.is_none() {
                retirement_indices[found] = index;
                found += 1;
                if found == retirement_count {
                    break;
                }
            }
        }
        if found != retirement_count {
            return Err(JobProgressFault::Capacity);
        }
        let next_items = self.owned_items.checked_add(self.reserved_items).and_then(|total| total.checked_add(owner_items)).ok_or(JobProgressFault::Capacity)?;
        let next_bytes = self.owned_bytes.checked_add(self.reserved_bytes).and_then(|total| total.checked_add(owner_bytes)).ok_or(JobProgressFault::Capacity)?;
        if next_items > JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS || next_bytes > JOB_PROGRESS_TOTAL_MAXIMUM_BYTES {
            return Err(JobProgressFault::Capacity);
        }
        let token = self.next_token;
        self.next_token = self.next_token.checked_add(1).ok_or(JobProgressFault::Exhausted)?;
        self.active[active_index].reservation = Some(token);
        for index in retirement_indices.iter().copied().take(retirement_count) {
            self.retirements[index].reservation = Some(token);
        }
        self.reserved_items += owner_items;
        self.reserved_bytes += owner_bytes;
        cx.consume_fuel(1);
        Ok(JobProgressAdmission { token, active_index, active_epoch, identity, live, retirement_indices, abort_retirement: staged_preview.then_some(retirement_indices[adoption_retirements]), owner_items, owner_bytes })
    }

    pub fn cancel_admission(&mut self, admission: JobProgressAdmission) -> Result<(), JobProgressFault> {
        let slot = self.active.get(admission.active_index).ok_or(JobProgressFault::Token)?;
        if slot.epoch != admission.active_epoch || slot.reservation != Some(admission.token) {
            return Err(JobProgressFault::Token);
        }
        for index in admission.retirement_indices.iter().copied().filter(|index| *index != usize::MAX) {
            let retirement = self.retirements.get(index).ok_or(JobProgressFault::Token)?;
            if retirement.reservation != Some(admission.token) || retirement.owner.is_some() {
                return Err(JobProgressFault::Token);
            }
        }
        self.active[admission.active_index].reservation = None;
        for index in admission.retirement_indices.iter().copied().filter(|index| *index != usize::MAX) {
            let retirement = &mut self.retirements[index];
            retirement.reservation = None;
        }
        self.reserved_items = self.reserved_items.saturating_sub(admission.owner_items);
        self.reserved_bytes = self.reserved_bytes.saturating_sub(admission.owner_bytes);
        Ok(())
    }

    pub fn publish_reserved(&mut self, cx: &mut job::StepContext<'_>, admission: JobProgressAdmission, publication: JobPublication, live: JobProgressLiveAuthority) -> Result<JobProgressReceipt, JobProgressRejected> {
        let identity = JobProgressIdentity::from_publication(admission.identity.actor, &publication);
        let slot_valid = self.active.get(admission.active_index).is_some_and(|slot| slot.epoch == admission.active_epoch && slot.reservation == Some(admission.token));
        if cx.is_cancelled() || cx.operation().0 != identity.operation || cx.generation().0 != identity.generation {
            let _ = self.cancel_admission(admission);
            return Err(JobProgressRejected::new(JobProgressFault::Cancelled, publication));
        }
        if !slot_valid || identity != admission.identity || live != admission.live || !live.accepts(identity) {
            let _ = self.cancel_admission(admission);
            return Err(JobProgressRejected::new(JobProgressFault::Stale, publication));
        }
        let kind = JobProgressKind::of(&publication.outcome);
        let applied_progress = match &publication.outcome {
            JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.applied_progress,
            _ => 0,
        };
        let retains_incoming = matches!(kind, JobProgressKind::Preview | JobProgressKind::Checkpoint | JobProgressKind::CommitValidated | JobProgressKind::Fault);
        let incoming = if retains_incoming {
            match JobPublicationOwner::try_new(publication) {
                Ok(owner) => Some(owner),
                Err((fault, publication)) => {
                    let _ = self.cancel_admission(admission);
                    return Err(JobProgressRejected::new(fault, publication));
                }
            }
        } else {
            drop(publication);
            None
        };
        let prior = JobProgressPriorState {
            published: self.active[admission.active_index].published,
            terminal: self.active[admission.active_index].terminal,
            identity: self.active[admission.active_index].identity,
            kind: self.active[admission.active_index].kind,
            applied_progress: self.active[admission.active_index].applied_progress,
        };
        let displaced = match kind {
            JobProgressKind::Preview | JobProgressKind::CommitValidated | JobProgressKind::Cancelled | JobProgressKind::Fault => self.active[admission.active_index].preview.take(),
            JobProgressKind::Yield | JobProgressKind::Checkpoint => None,
        };
        let mut retirement_cursor = 0usize;
        let mut displaced_retirement = None;
        if let Some(displaced) = displaced {
            let index = admission.retirement_indices[retirement_cursor];
            self.retirements[index].reservation = None;
            self.retirements[index].owner = Some(displaced);
            displaced_retirement = Some(index);
            retirement_cursor += 1;
        }
        if kind == JobProgressKind::Preview {
            self.active[admission.active_index].preview = incoming;
        } else if let Some(incoming) = incoming {
            let index = admission.retirement_indices[retirement_cursor];
            self.retirements[index].reservation = None;
            self.retirements[index].owner = Some(incoming);
        }
        let next_epoch = admission.active_epoch + 1;
        let slot = &mut self.active[admission.active_index];
        slot.epoch = next_epoch;
        slot.occupied = true;
        slot.published = true;
        slot.awaiting_ack = true;
        slot.checked_out = false;
        slot.terminal = kind.terminal();
        slot.reservation = None;
        slot.identity = identity;
        slot.kind = kind;
        slot.applied_progress = applied_progress;
        self.reserved_items = self.reserved_items.saturating_sub(admission.owner_items);
        self.reserved_bytes = self.reserved_bytes.saturating_sub(admission.owner_bytes);
        if retains_incoming {
            self.owned_items += admission.owner_items;
            self.owned_bytes += admission.owner_bytes;
        }
        let abort_retirement = admission.abort_retirement.map(|index| (index, admission.token));
        Ok(JobProgressReceipt { identity, kind, applied_progress, owner_bytes: admission.owner_bytes, active_index: admission.active_index, active_epoch: next_epoch, prior, displaced_retirement, abort_retirement })
    }

    pub fn acknowledge(&mut self, receipt: JobProgressReceipt) -> Result<(), (JobProgressFault, JobProgressReceipt)> {
        let Some(slot) = self.active.get_mut(receipt.active_index) else {
            return Err((JobProgressFault::Token, receipt));
        };
        if slot.epoch != receipt.active_epoch || !slot.occupied || !slot.awaiting_ack || slot.identity != receipt.identity {
            return Err((JobProgressFault::Token, receipt));
        }
        if let Some((index, token)) = receipt.abort_retirement {
            let Some(retirement) = self.retirements.get_mut(index) else {
                return Err((JobProgressFault::Token, receipt));
            };
            if retirement.reservation != Some(token) || retirement.owner.is_some() {
                return Err((JobProgressFault::Token, receipt));
            }
            retirement.reservation = None;
        }
        slot.awaiting_ack = false;
        if slot.terminal {
            if slot.preview.is_some() {
                slot.awaiting_ack = true;
                return Err((JobProgressFault::Token, receipt));
            }
            slot.clear();
        }
        Ok(())
    }

    pub fn abort(&mut self, receipt: JobProgressReceipt) -> Result<(), (JobProgressFault, JobProgressReceipt)> {
        let Some(slot) = self.active.get_mut(receipt.active_index) else {
            return Err((JobProgressFault::Token, receipt));
        };
        if slot.epoch != receipt.active_epoch || !slot.occupied || !slot.awaiting_ack || slot.identity != receipt.identity {
            return Err((JobProgressFault::Token, receipt));
        }
        if receipt.kind == JobProgressKind::Preview {
            let Some(owner) = slot.preview.take() else {
                return Err((JobProgressFault::Token, receipt));
            };
            let Some((index, token)) = receipt.abort_retirement else {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            };
            let Some(retirement) = self.retirements.get_mut(index) else {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            };
            if retirement.reservation != Some(token) || retirement.owner.is_some() {
                slot.preview = Some(owner);
                return Err((JobProgressFault::Token, receipt));
            }
            retirement.reservation = None;
            retirement.owner = Some(owner);
        }
        let restored_preview = if let Some(index) = receipt.displaced_retirement {
            let Some(retirement) = self.retirements.get_mut(index) else {
                return Err((JobProgressFault::Token, receipt));
            };
            let Some(owner) = retirement.owner.take() else {
                return Err((JobProgressFault::Token, receipt));
            };
            Some(owner)
        } else {
            None
        };
        slot.published = receipt.prior.published;
        slot.awaiting_ack = false;
        slot.checked_out = false;
        slot.terminal = receipt.prior.terminal;
        slot.reservation = None;
        slot.identity = receipt.prior.identity;
        slot.kind = receipt.prior.kind;
        slot.applied_progress = receipt.prior.applied_progress;
        slot.preview = restored_preview.or_else(|| slot.preview.take());
        Ok(())
    }

    pub fn retain_rejected(&mut self, rejected: JobProgressRejected) -> Result<(), JobProgressRejected> {
        let Some(index) = self.retirements.iter().position(|slot| slot.owner.is_none() && slot.reservation.is_none()) else {
            return Err(rejected);
        };
        let Ok((items, bytes, _)) = publication_owner_shape(rejected.publication(), false) else {
            return Err(rejected);
        };
        if self.owned_items.checked_add(self.reserved_items).and_then(|total| total.checked_add(items)).is_none_or(|total| total > JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS)
            || self.owned_bytes.checked_add(self.reserved_bytes).and_then(|total| total.checked_add(bytes)).is_none_or(|total| total > JOB_PROGRESS_TOTAL_MAXIMUM_BYTES)
        {
            return Err(rejected);
        }
        let publication = rejected.into_publication();
        let owner = match JobPublicationOwner::try_new(publication) {
            Ok(owner) => owner,
            Err((fault, publication)) => return Err(JobProgressRejected::new(fault, publication)),
        };
        self.owned_items = self.owned_items.saturating_add(items);
        self.owned_bytes = self.owned_bytes.saturating_add(bytes);
        self.retirements[index].owner = Some(owner);
        Ok(())
    }

    pub fn take(&mut self, actor: ActorId, job: u64) -> Result<JobProgressCheckout<'_>, JobProgressFault> {
        let slot = self.active.iter_mut().find(|slot| slot.occupied && slot.identity.actor == actor && slot.identity.job == job).ok_or(JobProgressFault::Missing)?;
        if slot.awaiting_ack || slot.checked_out {
            return Err(JobProgressFault::Busy);
        }
        let owner = slot.preview.take().ok_or(JobProgressFault::Missing)?;
        slot.checked_out = true;
        let epoch = slot.epoch;
        Ok(JobProgressCheckout { slot, owner: Some(owner), epoch })
    }

    pub fn begin_close_actor(&mut self, actor: ActorId) -> Result<(), JobProgressFault> {
        if self.closing_actors.iter().flatten().any(|current| *current == actor) {
            return Ok(());
        }
        let slot = self.closing_actors.iter_mut().find(|slot| slot.is_none()).ok_or(JobProgressFault::Capacity)?;
        *slot = Some(actor);
        Ok(())
    }

    pub fn begin_close_all(&mut self) {
        self.close_all = true;
    }

    pub fn has_close_work(&self) -> bool {
        self.retirements.iter().any(|slot| slot.owner.is_some()) || self.closing_actors.iter().any(Option::is_some) || self.close_all
    }

    pub fn close_step(&mut self, cx: &mut job::StepContext<'_>) -> JobProgressCloseStep {
        if cx.should_yield() {
            return JobProgressCloseStep::Blocked;
        }
        if let Some(index) = self.retirements.iter().position(|slot| slot.owner.is_some()) {
            cx.consume_fuel(1);
            let (terminal, released_items, released_bytes) = self.retirements[index].owner.as_mut().expect("job progress retirement owner").close_one();
            self.owned_items = self.owned_items.saturating_sub(released_items);
            self.owned_bytes = self.owned_bytes.saturating_sub(released_bytes);
            if terminal {
                let owner = self.retirements[index].owner.take().expect("terminal job progress retirement owner");
                assert!(owner.terminal_is_empty(), "terminal job progress retirement witness changed before removal");
            }
            return JobProgressCloseStep::Pending { released_items, released_bytes };
        }
        let closing_actor = self.closing_actors.iter().flatten().next().copied();
        if let Some(index) = self.active.iter().position(|slot| slot.occupied && (self.close_all || closing_actor == Some(slot.identity.actor))) {
            if self.active[index].checked_out {
                return JobProgressCloseStep::Blocked;
            }
            cx.consume_fuel(1);
            if let Some(owner) = self.active[index].preview.take() {
                let Some(retirement) = self.retirements.iter_mut().find(|slot| slot.owner.is_none() && slot.reservation.is_none()) else {
                    self.active[index].preview = Some(owner);
                    return JobProgressCloseStep::Blocked;
                };
                retirement.owner = Some(owner);
                return JobProgressCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.active[index].clear();
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(index) = self.closing_actors.iter().position(Option::is_some) {
            cx.consume_fuel(1);
            self.closing_actors[index] = None;
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.close_all {
            cx.consume_fuel(1);
            self.close_all = false;
            return JobProgressCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        JobProgressCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.active.iter().all(|slot| !slot.occupied && slot.reservation.is_none())
            && self.retirements.iter().all(|slot| slot.owner.is_none() && slot.reservation.is_none())
            && self.closing_actors.iter().all(Option::is_none)
            && !self.close_all
            && self.owned_items == 0
            && self.owned_bytes == 0
            && self.reserved_items == 0
            && self.reserved_bytes == 0
    }

    pub fn actor_terminal_is_empty(&self, actor: ActorId) -> bool {
        self.active.iter().all(|slot| !slot.occupied || slot.identity.actor != actor) && self.closing_actors.iter().flatten().all(|closing| *closing != actor) && self.retirements.iter().all(|slot| slot.owner.is_none())
    }
}

/// 🔄️ Borrowed preview owner whose Drop returns the exact owner to its original fixed slot.
pub struct JobProgressCheckout<'a> {
    slot: &'a mut JobProgressSlot,
    owner: Option<JobPublicationOwner>,
    epoch: u64,
}

impl JobProgressCheckout<'_> {
    pub fn identity(&self) -> JobProgressIdentity {
        self.slot.identity
    }

    pub fn preview(&self) -> &[u8] {
        match &self.owner.as_ref().expect("checked-out preview owner").publication.outcome {
            JobStepOutcome::PreviewReady { preview } => preview,
            _ => &[],
        }
    }

    pub fn resume(self) {}
}

impl Drop for JobProgressCheckout<'_> {
    fn drop(&mut self) {
        assert_eq!(self.slot.epoch, self.epoch, "checked-out job preview ABA");
        assert!(self.slot.checked_out && self.slot.preview.is_none(), "checked-out job preview slot changed before handback");
        self.slot.preview = self.owner.take();
        self.slot.checked_out = false;
    }
}
//#endregion 🎨️JobProgressOverlay

//#region ✉️Envelope
/// 🪟 Local, opaque window identifier ([`Origin::Ui`]'s target) — the concrete `WindowHandle`
/// lives in the kernel crate; this crate only ever routes by this bare numeric id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value", transparent)]
pub struct WindowId(pub u32);

impl WindowId {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u32(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_u32(bytes, pos, "WindowId").await?))
    }
}

/// ✉️ Who sent an [`Envelope`].
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Origin {
    Ui {
        window: WindowId,
    },
    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants, Part A): struct variant, not
    /// the newtype `Actor(ActorId)` this used to be — serde's internal tagging (`#[serde(tag =
    /// "kind")]`) cannot serialize a newtype variant whose payload is not itself a map (`ActorId` is
    /// a bare `u64` tuple struct). Latent, not live (this crate has no `serde_json` dependency and
    /// the wire uses `pack_encode`/`pack_decode`), but the generated TS mirror rendered this as an
    /// impossible `{"kind":"actor"} & bigint` intersection — see `📓️luna-serde-newtype-audit.md`.
    Actor {
        id: ActorId,
    },
    Kernel,
    Bus {
        topic: String,
    },
}

impl Origin {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Origin::Ui { window } => {
                pack::write_u8(out, 0).await;
                window.pack_encode(out).await;
            }
            Origin::Actor { id } => {
                pack::write_u8(out, 1).await;
                id.pack_encode(out).await;
            }
            Origin::Kernel => pack::write_u8(out, 2).await,
            Origin::Bus { topic } => {
                pack::write_u8(out, 3).await;
                pack::write_str(out, topic).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Origin").await?;
        match tag {
            0 => Ok(Origin::Ui { window: WindowId::pack_decode(bytes, pos).await? }),
            1 => Ok(Origin::Actor { id: ActorId::pack_decode(bytes, pos).await? }),
            2 => Ok(Origin::Kernel),
            3 => Ok(Origin::Bus { topic: pack::read_str(bytes, pos, "Origin::topic").await? }),
            other => Err(pack::PackError::InvalidTag { what: "Origin", tag: other, offset: *pos }),
        }
    }
}

/// ✉️ The message body an [`Envelope`] carries. `Event` is an opaque pack-encoded blob of the
/// kernel crate's concrete `Event` type — see the module-level seam docstring.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Payload {
    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants, Part A): struct variant, not
    /// the newtype `Event(Vec<u8>)` this used to be — serde's internal tagging cannot serialize a
    /// newtype variant whose payload is a sequence (`serde_json` errors "cannot serialize tagged
    /// newtype variant ... containing a sequence", the exact defect `JobStep::Done`/`Failed` hit
    /// earlier this ticket). Latent, not live, on THIS crate's own pack wire — see [`Origin::Actor`]'s
    /// doc for the full context and `📓️luna-serde-newtype-audit.md`.
    Event {
        bytes: Vec<u8>,
    },
    Suspend {
        operation: JobOperation,
        applied_progress: u64,
    },
    Resume {
        operation: JobOperation,
        checkpoint: JobCheckpoint,
    },
    /// 🐛️ Same class of bug as [`Payload::Event`], payload is an integer this time (`serde_json`:
    /// "cannot serialize tagged newtype variant ... containing an integer") — struct variant, not
    /// the newtype `Cancel(u64)` this used to be.
    Cancel {
        seq: u64,
    },
    JobStep {
        turn: JobTurn,
    },
    JobReplay {
        turn: JobTurn,
        request: JobReplayRequest,
        worker_count: u16,
        worker_slot: u16,
    },
}

impl Payload {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Payload::Event { bytes } => {
                pack::write_u8(out, 0).await;
                pack::write_bytes(out, bytes).await;
            }
            Payload::Suspend { operation, applied_progress } => {
                pack::write_u8(out, 1).await;
                operation.pack_encode(out).await;
                pack::write_u64(out, *applied_progress).await;
            }
            Payload::Resume { operation, checkpoint } => {
                pack::write_u8(out, 2).await;
                operation.pack_encode(out).await;
                checkpoint.pack_encode(out).await;
            }
            Payload::Cancel { seq } => {
                pack::write_u8(out, 3).await;
                pack::write_u64(out, *seq).await;
            }
            Payload::JobStep { turn } => {
                pack::write_u8(out, 4).await;
                turn.pack_encode(out).await;
            }
            Payload::JobReplay { turn, request, worker_count, worker_slot } => {
                pack::write_u8(out, 5).await;
                turn.pack_encode(out).await;
                request.pack_encode(out).await;
                pack::write_u16(out, *worker_count).await;
                pack::write_u16(out, *worker_slot).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Payload").await?;
        match tag {
            0 => Ok(Payload::Event { bytes: pack::read_bytes(bytes, pos, "Payload::Event").await? }),
            1 => Ok(Payload::Suspend { operation: JobOperation::pack_decode(bytes, pos).await?, applied_progress: pack::read_u64(bytes, pos, "Payload::Suspend::applied_progress").await? }),
            2 => Ok(Payload::Resume { operation: JobOperation::pack_decode(bytes, pos).await?, checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(Payload::Cancel { seq: pack::read_u64(bytes, pos, "Payload::Cancel").await? }),
            4 => Ok(Payload::JobStep { turn: JobTurn::pack_decode(bytes, pos).await? }),
            5 => Ok(Payload::JobReplay {
                turn: JobTurn::pack_decode(bytes, pos).await?,
                request: JobReplayRequest::pack_decode(bytes, pos).await?,
                worker_count: pack::read_u16(bytes, pos, "Payload::JobReplay::worker_count").await?,
                worker_slot: pack::read_u16(bytes, pos, "Payload::JobReplay::worker_slot").await?,
            }),
            other => Err(pack::PackError::InvalidTag { what: "Payload", tag: other, offset: *pos }),
        }
    }
}

//#region 🔑️CoalesceKey
/// 🔑️ Latest-wins-per-`(actor, key)` coalescing key. Pointer-move, resize, presence, refresh all
/// coalesce under this — 200 stale mouse-moves must never queue.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value", transparent)]
pub struct CoalesceKey(pub String);

impl CoalesceKey {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_str(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_str(bytes, pos, "CoalesceKey").await?))
    }
}
//#endregion 🔑️CoalesceKey

/// ✉️ One routed message: destination, sender, scheduling lane, an optional deadline that
/// short-circuits DRR ordering, an optional coalescing key, an optional envelope-seq this cancels,
/// and its payload.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct Envelope {
    pub to: ActorId,
    pub from: Origin,
    pub lane: Lane,
    pub seq: u64,
    pub deadline_ms: Option<u64>,
    pub coalesce: Option<CoalesceKey>,
    pub cancel_of: Option<u64>,
    pub payload: Payload,
}

impl Envelope {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.to.pack_encode(out).await;
        self.from.pack_encode(out).await;
        self.lane.pack_encode(out).await;
        pack::write_u64(out, self.seq).await;
        pack::write_opt_u64(out, &self.deadline_ms).await;
        pack::write_bool(out, self.coalesce.is_some()).await;
        if let Some(key) = &self.coalesce {
            key.pack_encode(out).await;
        }
        pack::write_opt_u64(out, &self.cancel_of).await;
        self.payload.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let to = ActorId::pack_decode(bytes, pos).await?;
        let from = Origin::pack_decode(bytes, pos).await?;
        let lane = Lane::pack_decode(bytes, pos).await?;
        let seq = pack::read_u64(bytes, pos, "Envelope::seq").await?;
        let deadline_ms = pack::read_opt_u64(bytes, pos, "Envelope::deadline_ms").await?;
        let coalesce = if pack::read_bool(bytes, pos, "Envelope::coalesce").await? { Some(CoalesceKey::pack_decode(bytes, pos).await?) } else { None };
        let cancel_of = pack::read_opt_u64(bytes, pos, "Envelope::cancel_of").await?;
        let payload = Payload::pack_decode(bytes, pos).await?;
        Ok(Self { to, from, lane, seq, deadline_ms, coalesce, cancel_of, payload })
    }
}
//#endregion ✉️Envelope

//#region 🔁️TurnResult
/// 🔁️ How a turn left the actor.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum TurnStatus {
    Idle,
    MoreWork,
    CheckpointReady {
        checkpoint: JobCheckpoint,
    },
    /// 🐛️ Struct variant, not the newtype `Faulted(Vec<u8>)` this used to be — same sequence-payload
    /// serde defect as [`Payload::Event`]; see that variant's doc for the full explanation.
    Faulted {
        detail: Vec<u8>,
    },
    PreviewReady {
        preview: Vec<u8>,
        sequence: u64,
    },
    CommitReady {
        candidate: JobCommitCandidate,
    },
    Cancelled,
}

impl TurnStatus {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            TurnStatus::Idle => pack::write_u8(out, 0).await,
            TurnStatus::MoreWork => pack::write_u8(out, 1).await,
            TurnStatus::CheckpointReady { checkpoint } => {
                pack::write_u8(out, 2).await;
                checkpoint.pack_encode(out).await;
            }
            TurnStatus::Faulted { detail } => {
                pack::write_u8(out, 3).await;
                pack::write_bytes(out, detail).await;
            }
            TurnStatus::PreviewReady { preview, sequence } => {
                pack::write_u8(out, 4).await;
                pack::write_bytes(out, preview).await;
                pack::write_u64(out, *sequence).await;
            }
            TurnStatus::CommitReady { candidate } => {
                pack::write_u8(out, 5).await;
                candidate.pack_encode(out).await;
            }
            TurnStatus::Cancelled => pack::write_u8(out, 6).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "TurnStatus").await?;
        match tag {
            0 => Ok(TurnStatus::Idle),
            1 => Ok(TurnStatus::MoreWork),
            2 => Ok(TurnStatus::CheckpointReady { checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(TurnStatus::Faulted { detail: pack::read_bytes(bytes, pos, "TurnStatus::Faulted").await? }),
            4 => Ok(TurnStatus::PreviewReady { preview: pack::read_bytes(bytes, pos, "TurnStatus::PreviewReady::preview").await?, sequence: pack::read_u64(bytes, pos, "TurnStatus::PreviewReady::sequence").await? }),
            5 => Ok(TurnStatus::CommitReady { candidate: JobCommitCandidate::pack_decode(bytes, pos).await? }),
            6 => Ok(TurnStatus::Cancelled),
            other => Err(pack::PackError::InvalidTag { what: "TurnStatus", tag: other, offset: *pos }),
        }
    }
}

/// 📊️ What one turn actually spent.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct Usage {
    pub fuel: u64,
    pub wall_us: u64,
    pub memory_bytes: u64,
}

impl Usage {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.fuel).await;
        pack::write_u64(out, self.wall_us).await;
        pack::write_u64(out, self.memory_bytes).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { fuel: pack::read_u64(bytes, pos, "Usage::fuel").await?, wall_us: pack::read_u64(bytes, pos, "Usage::wall_us").await?, memory_bytes: pack::read_u64(bytes, pos, "Usage::memory_bytes").await? })
    }
}

/// 🔁️ What a `GuestRuntime::execute_turn` (packet B1) hands back to the kernel. `ui_patches`/
/// `effects` are opaque pack-encoded `Vec<UiPatch>`/`Vec<Effect>` blobs — see the module seam docstring.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct TurnResult {
    pub ui_patches: Vec<u8>,
    pub effects: Vec<u8>,
    pub command_ingress: Vec<u8>,
    pub lifecycle_receipt: Option<instance_lifetime::ActorInstanceLifecycleReceipt>,
    pub ui_patch_receipt: Option<instance_lifetime::ActorUiPatchReceipt>,
    pub next_wake: Option<u64>,
    pub status: TurnStatus,
    pub usage: Usage,
}

impl TurnResult {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), pack::PackError> {
        let mut receipt = [0; instance_lifetime::ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES];
        let length = match self.lifecycle_receipt {
            Some(value) => instance_lifetime::ActorInstanceLifecycleWire::Receipt(value).encode(&mut receipt).map_err(pack::PackError::InvalidLifecycle)?,
            None => 0,
        };
        instance_lifetime::ActorUiPatchReceipt::validate_pairing(self.ui_patch_receipt, usize::from(!self.ui_patches.is_empty())).map_err(pack::PackError::InvalidUiPatchReceipt)?;
        let mut patch_receipt = [0; instance_lifetime::ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES];
        let patch_length = match self.ui_patch_receipt {
            Some(value) => value.encode(&mut patch_receipt).map_err(pack::PackError::InvalidUiPatchReceipt)?,
            None => 0,
        };
        pack::write_bytes(out, &self.ui_patches).await;
        pack::write_bytes(out, &self.effects).await;
        pack::write_bytes(out, &self.command_ingress).await;
        pack::write_bytes(out, &receipt[..length]).await;
        pack::write_bytes(out, &patch_receipt[..patch_length]).await;
        pack::write_opt_u64(out, &self.next_wake).await;
        self.status.pack_encode(out).await;
        self.usage.pack_encode(out).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let result = Self {
            ui_patches: pack::read_bytes(bytes, pos, "TurnResult::ui_patches").await?,
            effects: pack::read_bytes(bytes, pos, "TurnResult::effects").await?,
            command_ingress: pack::read_bytes(bytes, pos, "TurnResult::command_ingress").await?,
            lifecycle_receipt: Self::read_lifecycle_receipt(bytes, pos).await?,
            ui_patch_receipt: Self::read_ui_patch_receipt(bytes, pos).await?,
            next_wake: pack::read_opt_u64(bytes, pos, "TurnResult::next_wake").await?,
            status: TurnStatus::pack_decode(bytes, pos).await?,
            usage: Usage::pack_decode(bytes, pos).await?,
        };
        instance_lifetime::ActorUiPatchReceipt::validate_pairing(result.ui_patch_receipt, usize::from(!result.ui_patches.is_empty())).map_err(pack::PackError::InvalidUiPatchReceipt)?;
        Ok(result)
    }

    async fn read_ui_patch_receipt(bytes: &[u8], pos: &mut usize) -> Result<Option<instance_lifetime::ActorUiPatchReceipt>, pack::PackError> {
        let length = pack::read_u8(bytes, pos, "TurnResult::ui_patch_receipt.length").await? as usize;
        if length > instance_lifetime::ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES { return Err(pack::PackError::InvalidUiPatchReceipt("receipt length exceeds fixed authority")); }
        if length == 0 { return Ok(None); }
        let end = pos.checked_add(length).ok_or(pack::PackError::InvalidUiPatchReceipt("receipt offset overflow"))?;
        let receipt = instance_lifetime::ActorUiPatchReceipt::decode(bytes.get(*pos..end).ok_or(pack::PackError::Truncated(*pos, "TurnResult::ui_patch_receipt"))?).map_err(pack::PackError::InvalidUiPatchReceipt)?;
        *pos = end;
        Ok(Some(receipt))
    }

    async fn read_lifecycle_receipt(bytes: &[u8], pos: &mut usize) -> Result<Option<instance_lifetime::ActorInstanceLifecycleReceipt>, pack::PackError> {
        let length = pack::read_u8(bytes, pos, "TurnResult::lifecycle_receipt.length").await? as usize;
        if length > instance_lifetime::ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES { return Err(pack::PackError::InvalidLifecycle("receipt length exceeds fixed authority")); }
        if length == 0 { return Ok(None); }
        let end = pos.checked_add(length).ok_or(pack::PackError::InvalidLifecycle("receipt offset overflow"))?;
        let receipt = bytes.get(*pos..end).ok_or(pack::PackError::Truncated(*pos, "TurnResult::lifecycle_receipt"))?;
        let instance_lifetime::ActorInstanceLifecycleWire::Receipt(receipt) = instance_lifetime::ActorInstanceLifecycleWire::decode(receipt).map_err(pack::PackError::InvalidLifecycle)? else {
            return Err(pack::PackError::InvalidLifecycle("turn result requires a receipt"));
        };
        *pos = end;
        Ok(Some(receipt))
    }
}
//#endregion 🔁️TurnResult

//#region 📬️Mailbox
/// 🚦 What `Mailbox::enqueue` reports back — `Rejected` must always surface as a busy badge, never
/// a silent drop of a user action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Backpressure {
    Accept,
    Coalesced,
    /// 🐛️ Struct variant, not the newtype `Dropped(Lane)` this used to be. `Lane` itself serializes
    /// fine alone (a plain enum, `#[serde(tag = "kind")]` with unit variants), but wrapping it as a
    /// newtype payload of ANOTHER internally-tagged enum hits the same "cannot serialize tagged
    /// newtype variant" defect as [`Payload::Event`] — the payload type does not matter, only that
    /// it is not itself a map.
    Dropped {
        lane: Lane,
    },
    Rejected,
}

impl Backpressure {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Backpressure::Accept => pack::write_u8(out, 0).await,
            Backpressure::Coalesced => pack::write_u8(out, 1).await,
            Backpressure::Dropped { lane } => {
                pack::write_u8(out, 2).await;
                lane.pack_encode(out).await;
            }
            Backpressure::Rejected => pack::write_u8(out, 3).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Backpressure").await?;
        match tag {
            0 => Ok(Backpressure::Accept),
            1 => Ok(Backpressure::Coalesced),
            2 => Ok(Backpressure::Dropped { lane: Lane::pack_decode(bytes, pos).await? }),
            3 => Ok(Backpressure::Rejected),
            other => Err(pack::PackError::InvalidTag { what: "Backpressure", tag: other, offset: *pos }),
        }
    }
}

/// 📬️ Bounded ring per actor: one `VecDeque` per lane (so pop honors lane priority for free), a
/// coalescing scan on enqueue, and eviction of the lowest-priority nonempty lane before a hard reject.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct Mailbox {
    pub capacity: u16,
    /// 🚫️ Excluded from the TS mirror because this field is internal runtime state, and it is
    /// module-private — no TypeScript consumer can reach the per-lane rings, which cross to the web
    /// shard as pack bytes via [`Mailbox::pack_encode`], never as a structural JSON object. Emitting
    /// them would describe a shape the wire never carries. Found the first time typegen was ever run
    /// for this crate (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, T1's lease).
    lanes: [VecDeque<Envelope>; 4],
    len: u16,
}

impl Mailbox {
    pub async fn new(capacity: u16) -> Self {
        Self { capacity, lanes: Default::default(), len: 0 }
    }

    // 🚫️async: E1-adjacent pure accessor — every consumer sits inside a sync `Iterator`/`Option`
    // combinator closure or a test assertion — see R9 residue shape 1.
    pub fn len(&self) -> u16 {
        self.len
    }

    // 🚫️async: E1-adjacent pure accessor — same rationale as `len` above — see R9 residue shape 1.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // 🚫️async: E1-adjacent pure accessor — its one consumer, `Kernel::mailbox_pressure` (below),
    // sits inside a sync `Option::map` closure — see R9 residue shape 1.
    pub fn pressure(&self) -> f32 {
        if self.capacity == 0 {
            return 1.0;
        }
        self.len as f32 / self.capacity as f32
    }

    /// 📬️ `latest-wins` coalescing (replaces in place, preserving queue position — coalescing must
    /// not let a hot key jump the line), then bounded-ring enqueue with lowest-priority eviction.
    pub async fn enqueue(&mut self, envelope: Envelope) -> Backpressure {
        if let Some(key) = envelope.coalesce.clone() {
            let lane_idx = envelope.lane.priority_rank().await;
            if let Some(existing) = self.lanes[lane_idx].iter_mut().find(|e| e.coalesce.as_ref() == Some(&key)) {
                *existing = envelope;
                return Backpressure::Coalesced;
            }
        }
        if self.len >= self.capacity {
            let incoming_rank = envelope.lane.priority_rank().await;
            let victim_rank = (incoming_rank + 1..4).rev().find(|&rank| !self.lanes[rank].is_empty());
            match victim_rank {
                Some(rank) => {
                    self.lanes[rank].pop_front();
                    self.len -= 1;
                    let dropped_lane = Lane::ALL[rank];
                    self.lanes[incoming_rank].push_back(envelope);
                    self.len += 1;
                    Backpressure::Dropped { lane: dropped_lane }
                }
                None => Backpressure::Rejected,
            }
        } else {
            self.lanes[envelope.lane.priority_rank().await].push_back(envelope);
            self.len += 1;
            Backpressure::Accept
        }
    }

    /// ⏭️ Highest lane-priority envelope first, FIFO within a lane.
    pub async fn pop_next(&mut self) -> Option<Envelope> {
        for lane in self.lanes.iter_mut() {
            if let Some(envelope) = lane.pop_front() {
                self.len -= 1;
                return Some(envelope);
            }
        }
        None
    }

    /// 👀️ Earliest `deadline_ms` among all queued envelopes, for scheduler preemption checks.
    // 🚫️async: E1-adjacent pure accessor — both consumers sit inside sync `Iterator::filter_map`
    // closures (`Scheduler::tick`'s deadline-preemption and wake-at scans), which cannot await —
    // see R9 residue shape 1.
    pub fn earliest_deadline(&self) -> Option<u64> {
        self.lanes.iter().flatten().filter_map(|e| e.deadline_ms).min()
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u16(out, self.capacity).await;
        pack::write_u16(out, self.len).await;
        for lane in &self.lanes {
            pack::write_varint_u64(out, lane.len() as u64).await;
            for envelope in lane.iter() {
                envelope.pack_encode(out).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let capacity = pack::read_u16(bytes, pos, "Mailbox::capacity").await?;
        let len = pack::read_u16(bytes, pos, "Mailbox::len").await?;
        let mut lanes: [VecDeque<Envelope>; 4] = Default::default();
        for lane in lanes.iter_mut() {
            *lane = pack::read_vec(bytes, pos, "Mailbox::lane", Envelope::pack_decode).await?.into();
        }
        Ok(Self { capacity, lanes, len })
    }
}
//#endregion 📬️Mailbox

//#region 🔐️CapabilityGrant
/// 🔐️ Minimal local stand-in for `kernel::CapabilityGrant` — see the module-level seam docstring.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct CapabilityGrant {
    pub capability: String,
    pub scope: Option<Vec<u8>>,
}

impl CapabilityGrant {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_str(out, &self.capability).await;
        pack::write_opt_bytes(out, &self.scope).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { capability: pack::read_str(bytes, pos, "CapabilityGrant::capability").await?, scope: pack::read_opt_bytes(bytes, pos, "CapabilityGrant::scope").await? })
    }
}
/// 🔐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the security property of
/// the whole extension-activation design — an extension must never hold a capability its host plugin
/// lacks. A REQUESTED grant survives only when `granted` (the parent's own already-granted set)
/// carries a grant of the SAME `capability` name; an ungranted request is silently dropped, never
/// escalated or substituted. This is an actual intersection, not "grant what was asked for": the
/// output is always a subset of `requested`, bounded by `granted`. Matched by capability name only
/// (this crate's `CapabilityGrant` is a minimal pack-codeable stand-in — see its own doc comment — so
/// `scope` reconciliation belongs to the real broker at integration time, not this pure function).
pub async fn intersect_capabilities(granted: &[CapabilityGrant], requested: &[CapabilityGrant]) -> Vec<CapabilityGrant> {
    requested.iter().filter(|request| granted.iter().any(|grant| grant.capability == request.capability)).cloned().collect()
}
//#endregion 🔐️CapabilityGrant

//#region 🚑️FailurePolicy
/// 🚑️ What triggered a failure-ladder transition.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FailureSignal {
    DeadlineOverrun {
        ratio: f32,
    },
    FuelExhausted,
    MemoryLimit,
    MailboxOverflow,
    UiQuota,
    /// 🐛️ Struct variant, not the newtype `Trap(String)` this used to be — `String` is a sequence
    /// of chars as far as serde's internal tagging is concerned, so it hits the exact same "cannot
    /// serialize tagged newtype variant ... containing a sequence" defect as [`Payload::Event`].
    Trap {
        detail: String,
    },
    HeartbeatMissed {
        count: u32,
    },
    ManualReset,
}

impl FailureSignal {
    /// 🚨️ Whether this signal is severe enough to skip the warn/throttle rungs and trap outright.
    async fn is_fatal(&self) -> bool {
        matches!(self, FailureSignal::Trap { .. })
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            FailureSignal::DeadlineOverrun { ratio } => {
                pack::write_u8(out, 0).await;
                pack::write_f32(out, *ratio).await;
            }
            FailureSignal::FuelExhausted => pack::write_u8(out, 1).await,
            FailureSignal::MemoryLimit => pack::write_u8(out, 2).await,
            FailureSignal::MailboxOverflow => pack::write_u8(out, 3).await,
            FailureSignal::UiQuota => pack::write_u8(out, 4).await,
            FailureSignal::Trap { detail } => {
                pack::write_u8(out, 5).await;
                pack::write_str(out, detail).await;
            }
            FailureSignal::HeartbeatMissed { count } => {
                pack::write_u8(out, 6).await;
                pack::write_u32(out, *count).await;
            }
            FailureSignal::ManualReset => pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "FailureSignal").await?;
        match tag {
            0 => Ok(FailureSignal::DeadlineOverrun { ratio: pack::read_f32(bytes, pos, "FailureSignal::ratio").await? }),
            1 => Ok(FailureSignal::FuelExhausted),
            2 => Ok(FailureSignal::MemoryLimit),
            3 => Ok(FailureSignal::MailboxOverflow),
            4 => Ok(FailureSignal::UiQuota),
            5 => Ok(FailureSignal::Trap { detail: pack::read_str(bytes, pos, "FailureSignal::Trap").await? }),
            6 => Ok(FailureSignal::HeartbeatMissed { count: pack::read_u32(bytes, pos, "FailureSignal::count").await? }),
            7 => Ok(FailureSignal::ManualReset),
            other => Err(pack::PackError::InvalidTag { what: "FailureSignal", tag: other, offset: *pos }),
        }
    }
}

/// 🪜️ Rungs of the failure ladder, worst-consequence order.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FailureStage {
    Healthy,
    Warned,
    Throttled { factor: f32 },
    Suspended { until: u64 },
    Cancelled,
    Trapped { restarts: u32 },
    Quarantined { until: u64 },
    Disabled,
}

impl FailureStage {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            FailureStage::Healthy => pack::write_u8(out, 0).await,
            FailureStage::Warned => pack::write_u8(out, 1).await,
            FailureStage::Throttled { factor } => {
                pack::write_u8(out, 2).await;
                pack::write_f32(out, *factor).await;
            }
            FailureStage::Suspended { until } => {
                pack::write_u8(out, 3).await;
                pack::write_u64(out, *until).await;
            }
            FailureStage::Cancelled => pack::write_u8(out, 4).await,
            FailureStage::Trapped { restarts } => {
                pack::write_u8(out, 5).await;
                pack::write_u32(out, *restarts).await;
            }
            FailureStage::Quarantined { until } => {
                pack::write_u8(out, 6).await;
                pack::write_u64(out, *until).await;
            }
            FailureStage::Disabled => pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "FailureStage").await?;
        match tag {
            0 => Ok(FailureStage::Healthy),
            1 => Ok(FailureStage::Warned),
            2 => Ok(FailureStage::Throttled { factor: pack::read_f32(bytes, pos, "FailureStage::factor").await? }),
            3 => Ok(FailureStage::Suspended { until: pack::read_u64(bytes, pos, "FailureStage::until").await? }),
            4 => Ok(FailureStage::Cancelled),
            5 => Ok(FailureStage::Trapped { restarts: pack::read_u32(bytes, pos, "FailureStage::restarts").await? }),
            6 => Ok(FailureStage::Quarantined { until: pack::read_u64(bytes, pos, "FailureStage::until").await? }),
            7 => Ok(FailureStage::Disabled),
            other => Err(pack::PackError::InvalidTag { what: "FailureStage", tag: other, offset: *pos }),
        }
    }
}

/// ⏳️ Number of consecutive clean turns before `FailureState::on_clean_turn` decays one rung.
pub const FAILURE_DECAY_CLEAN_TURNS: u32 = 10;
/// 🔁️ Consecutive traps before an actor's package is quarantined.
pub const FAILURE_QUARANTINE_RESTART_THRESHOLD: u32 = 3;
/// 💓️ Consecutive missed heartbeats before a shard is treated as dead (see design §Watchdog).
pub const FAILURE_HEARTBEAT_TRAP_THRESHOLD: u32 = 3;

/// ⚖️ Exponential per-lane warn-count thresholds `[warned→throttled, throttled→suspended]` —
/// interactive actors escalate fastest (they are the most visible), maintenance actors get the
/// most slack.
async fn lane_escalation_thresholds(lane: Lane) -> [u32; 2] {
    match lane {
        Lane::Interactive => [2, 3],
        Lane::UserVisible => [3, 5],
        Lane::Background => [5, 9],
        Lane::Maintenance => [9, 17],
    }
}

/// 🚑️ Per-actor failure ladder state: current [`FailureStage`], warn/restart counters, and the
/// clean-turn counter that drives decay.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct FailureState {
    pub stage: FailureStage,
    pub clean_turns: u32,
    pub warn_count: u32,
    pub restart_count: u32,
    pub last_signal_ms: u64,
}

impl Default for FailureState {
    fn default() -> Self {
        Self::new()
    }
}

/// 🪜️ Outcome of `FailureState::on_signal` the kernel must act on beyond updating this actor's own state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureEscalation {
    /// No cross-actor action required.
    None,
    /// This actor trapped — kernel must drop + re-instantiate with `ActorId::next_generation` and restore its last checkpoint.
    Restart,
    /// Restart count crossed the threshold — kernel must quarantine every actor sharing this actor's `PackageId`.
    QuarantinePackage,
}

impl FailureState {
    // 🚫️async: E1 pure constructor consumed by `Default::default` (external trait impl above) —
    // see R9
    pub fn new() -> Self {
        Self { stage: FailureStage::Healthy, clean_turns: 0, warn_count: 0, restart_count: 0, last_signal_ms: 0 }
    }

    /// 🚑️ Applies one failure signal, returning what the kernel must do beyond this actor's own bookkeeping.
    pub async fn on_signal(&mut self, signal: &FailureSignal, lane: Lane, now_ms: u64) -> FailureEscalation {
        self.clean_turns = 0;
        self.last_signal_ms = now_ms;
        if let FailureSignal::ManualReset = signal {
            *self = Self::new();
            return FailureEscalation::None;
        }
        if signal.is_fatal().await {
            self.restart_count += 1;
            self.stage = FailureStage::Trapped { restarts: self.restart_count };
            if self.restart_count >= FAILURE_QUARANTINE_RESTART_THRESHOLD {
                self.stage = FailureStage::Quarantined { until: now_ms + quarantine_duration_ms(self.restart_count).await };
                return FailureEscalation::QuarantinePackage;
            }
            return FailureEscalation::Restart;
        }
        if let FailureSignal::HeartbeatMissed { count } = signal {
            if *count >= FAILURE_HEARTBEAT_TRAP_THRESHOLD {
                self.restart_count += 1;
                self.stage = FailureStage::Trapped { restarts: self.restart_count };
                if self.restart_count >= FAILURE_QUARANTINE_RESTART_THRESHOLD {
                    self.stage = FailureStage::Quarantined { until: now_ms + quarantine_duration_ms(self.restart_count).await };
                    return FailureEscalation::QuarantinePackage;
                }
                return FailureEscalation::Restart;
            }
        }
        self.warn_count += 1;
        let [throttle_at, suspend_at] = lane_escalation_thresholds(lane).await;
        self.stage = if self.warn_count >= suspend_at {
            FailureStage::Suspended { until: now_ms + suspend_backoff_ms(self.warn_count).await }
        } else if self.warn_count >= throttle_at {
            FailureStage::Throttled { factor: throttle_factor(self.warn_count, throttle_at).await }
        } else {
            FailureStage::Warned
        };
        FailureEscalation::None
    }

    /// ⏳️ One clean (no-signal) turn elapsed — decays one rung after `FAILURE_DECAY_CLEAN_TURNS`
    /// consecutive clean turns, or immediately promotes an expired `Suspended`/`Quarantined` timer
    /// into the next rung down.
    pub async fn on_clean_turn(&mut self, now_ms: u64) {
        self.clean_turns += 1;
        match self.stage {
            FailureStage::Warned if self.clean_turns >= FAILURE_DECAY_CLEAN_TURNS => {
                self.stage = FailureStage::Healthy;
                self.warn_count = 0;
                self.clean_turns = 0;
            }
            FailureStage::Throttled { .. } if self.clean_turns >= FAILURE_DECAY_CLEAN_TURNS => {
                self.stage = FailureStage::Warned;
                self.clean_turns = 0;
            }
            FailureStage::Suspended { until } if now_ms >= until => {
                self.stage = FailureStage::Throttled { factor: 0.5 };
                self.clean_turns = 0;
            }
            FailureStage::Quarantined { until } if now_ms >= until => {
                self.stage = FailureStage::Warned;
                self.clean_turns = 0;
                self.restart_count = 0;
            }
            _ => {}
        }
    }

    /// ▶️ Whether the scheduler may currently grant this actor a turn.
    pub async fn runnable(&self, now_ms: u64) -> bool {
        match self.stage {
            FailureStage::Healthy | FailureStage::Warned | FailureStage::Throttled { .. } => true,
            FailureStage::Suspended { until } => now_ms >= until,
            FailureStage::Cancelled | FailureStage::Trapped { .. } | FailureStage::Disabled => false,
            FailureStage::Quarantined { until } => now_ms >= until,
        }
    }

    /// ⚖️ The DRR weight/budget scale factor for the current stage — `1.0` unless `Throttled`.
    pub async fn throttle_factor(&self) -> f32 {
        match self.stage {
            FailureStage::Throttled { factor } => factor,
            _ => 1.0,
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.stage.pack_encode(out).await;
        pack::write_u32(out, self.clean_turns).await;
        pack::write_u32(out, self.warn_count).await;
        pack::write_u32(out, self.restart_count).await;
        pack::write_u64(out, self.last_signal_ms).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            stage: FailureStage::pack_decode(bytes, pos).await?,
            clean_turns: pack::read_u32(bytes, pos, "FailureState::clean_turns").await?,
            warn_count: pack::read_u32(bytes, pos, "FailureState::warn_count").await?,
            restart_count: pack::read_u32(bytes, pos, "FailureState::restart_count").await?,
            last_signal_ms: pack::read_u64(bytes, pos, "FailureState::last_signal_ms").await?,
        })
    }
}

async fn throttle_factor(warn_count: u32, throttle_at: u32) -> f32 {
    let steps_in = warn_count.saturating_sub(throttle_at);
    (0.5f32).powi(1 + steps_in as i32).max(0.05)
}

async fn suspend_backoff_ms(warn_count: u32) -> u64 {
    1_000u64.saturating_mul(1u64 << warn_count.min(10))
}

async fn quarantine_duration_ms(restart_count: u32) -> u64 {
    5_000u64.saturating_mul(1u64 << restart_count.min(10))
}
//#endregion 🚑️FailurePolicy

//#region 🗂️ActorRecord
/// 🗂️ Actor lifecycle state, driven by [`Kernel::activate`]/`suspend`/`resume` and the failure ladder.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActorStatus {
    Cold,
    Activating,
    Active,
    Suspended { checkpoint: Option<Vec<u8>> },
    Draining,
    Trapped,
    Quarantined,
    Disabled,
}

impl ActorStatus {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            ActorStatus::Cold => pack::write_u8(out, 0).await,
            ActorStatus::Activating => pack::write_u8(out, 1).await,
            ActorStatus::Active => pack::write_u8(out, 2).await,
            ActorStatus::Suspended { checkpoint } => {
                pack::write_u8(out, 3).await;
                pack::write_opt_bytes(out, checkpoint).await;
            }
            ActorStatus::Draining => pack::write_u8(out, 4).await,
            ActorStatus::Trapped => pack::write_u8(out, 5).await,
            ActorStatus::Quarantined => pack::write_u8(out, 6).await,
            ActorStatus::Disabled => pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "ActorStatus").await?;
        match tag {
            0 => Ok(ActorStatus::Cold),
            1 => Ok(ActorStatus::Activating),
            2 => Ok(ActorStatus::Active),
            3 => Ok(ActorStatus::Suspended { checkpoint: pack::read_opt_bytes(bytes, pos, "ActorStatus::Suspended").await? }),
            4 => Ok(ActorStatus::Draining),
            5 => Ok(ActorStatus::Trapped),
            6 => Ok(ActorStatus::Quarantined),
            7 => Ok(ActorStatus::Disabled),
            other => Err(pack::PackError::InvalidTag { what: "ActorStatus", tag: other, offset: *pos }),
        }
    }
}

/// 🗂️ Full snapshot of one actor — assembled on demand by [`Kernel::actor_record`] from the
/// scheduler's live entry plus this actor's kind/capabilities/status/failure/metrics bookkeeping.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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

//#region 🧩️ShardTable
/// 🧩️ Which host EXECUTION SURFACE a shard runs on — `design-runtime.md` §2's "thread, worker, or
/// process" trio, unchanged by the P1c one-pool-worker-runtime refactor EXCEPT for what `Native`
/// itself now means. A shard was never "a thread" as an identity here — `ShardTable` (below) only
/// ever pinned an [`ActorId`] to a [`ShardId`], a bookkeeping integer; this crate's own purity rule
/// means it never spawned anything. What changed in the HOST crates (`semio-framework-plugin-host`'s
/// `ShardExecutor`, `semio-framework-os`'s `NativeKernelRuntime`) is that `Native` no longer implies
/// "and therefore gets one dedicated OS thread" — a `Native` shard is now a logical affinity unit
/// (its actors' `wasmtime::Store`s are pinned to it so guest instance state stays coherent) whose
/// turns are submitted as jobs onto one process-wide `semio_framework_async::WorkerPool`, mutually
/// exclusive PER SHARD (so affinity is enforced by a lock, not by thread identity) rather than
/// concurrent by construction. `WebWorker`/`Process` were already real boundary crossings (a browser
/// worker, an OS process) unaffected by this distinction — `Native` is the one variant whose meaning
/// narrows from "a thread" to "this process, pool-scheduled."
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub enum ShardKind {
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1c): renamed from `Thread` — same process,
    /// same address space, `wasmtime::Store` instances pinned by `ShardTable::pin`'s least-loaded
    /// placement, but no OS thread of its own; the host schedules its turns onto a shared
    /// `WorkerPool`. Wire tag unchanged (`0`) — this is a rename, not a new wire variant.
    Native,
    WebWorker,
    Process,
}

impl ShardKind {
    async fn tag(self) -> u8 {
        match self {
            ShardKind::Native => 0,
            ShardKind::WebWorker => 1,
            ShardKind::Process => 2,
        }
    }
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u8(out, self.tag().await).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        match pack::read_u8(bytes, pos, "ShardKind").await? {
            0 => Ok(ShardKind::Native),
            1 => Ok(ShardKind::WebWorker),
            2 => Ok(ShardKind::Process),
            other => Err(pack::PackError::InvalidTag { what: "ShardKind", tag: other, offset: *pos }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value", transparent)]
pub struct ShardId(pub u16);

impl ShardId {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u16(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_u16(bytes, pos, "ShardId").await?))
    }
}

/// 🧮️ Host-side sizing policy: native `available_parallelism()-1` clamped `[2,8]` — pure arithmetic,
/// the actual OS query happens in the (non-pure) host binary.
pub async fn clamp_native_shard_count(available_parallelism: u16) -> u16 {
    available_parallelism.saturating_sub(1).clamp(2, 8)
}

/// 🧮️ Host-side sizing policy: web `min(hardwareConcurrency-1, 4)`.
pub async fn clamp_web_shard_count(hardware_concurrency: u16) -> u16 {
    hardware_concurrency.saturating_sub(1).clamp(1, 4)
}

/// 🔁️ `ToValue`/`FromValue` for `BTreeMap<ActorId, ShardId>` — the derive's generic `BTreeMap`
/// support is keyed on `String` only, and a bare-integer-newtype key (`ActorId`) needs its own
/// stringified-key `DslValue::Object` shape (mirrors what `serde_json` itself produces for a
/// newtype-struct map key: the inner integer, stringified).
fn actor_shard_map_to_value(map: &BTreeMap<ActorId, ShardId>) -> ::protocol::value::DslValue {
    ::protocol::value::DslValue::object(map.iter().map(|(id, shard)| (id.0.to_string(), ::protocol::value::ToValue::to_value(shard))))
}
fn actor_shard_map_from_value(value: ::protocol::value::DslValue) -> Result<BTreeMap<ActorId, ShardId>, ::protocol::value::ValueError> {
    let ::protocol::value::DslValue::Object(entries) = value else {
        return Err(::protocol::value::ValueError::new("expected object"));
    };
    entries
        .into_iter()
        .map(|(key, value)| {
            let id: u64 = key.parse().map_err(|_| ::protocol::value::ValueError::new(format!("invalid ActorId key `{key}`")))?;
            let shard: ShardId = ::protocol::value::FromValue::from_value(value).map_err(|error: ::protocol::value::ValueError| error.under(&key))?;
            Ok((ActorId(id), shard))
        })
        .collect()
}

/// 🔁️ `ToValue`/`FromValue` for `BTreeMap<ShardId, ActorId>` — same stringified-key shape as
/// [`actor_shard_map_to_value`], keyed the other way round.
fn shard_actor_map_to_value(map: &BTreeMap<ShardId, ActorId>) -> ::protocol::value::DslValue {
    ::protocol::value::DslValue::object(map.iter().map(|(shard, id)| (shard.0.to_string(), ::protocol::value::ToValue::to_value(id))))
}
fn shard_actor_map_from_value(value: ::protocol::value::DslValue) -> Result<BTreeMap<ShardId, ActorId>, ::protocol::value::ValueError> {
    let ::protocol::value::DslValue::Object(entries) = value else {
        return Err(::protocol::value::ValueError::new("expected object"));
    };
    entries
        .into_iter()
        .map(|(key, value)| {
            let id: u16 = key.parse().map_err(|_| ::protocol::value::ValueError::new(format!("invalid ShardId key `{key}`")))?;
            let actor: ActorId = ::protocol::value::FromValue::from_value(value).map_err(|error: ::protocol::value::ValueError| error.under(&key))?;
            Ok((ShardId(id), actor))
        })
        .collect()
}

/// 🧩️ Fixed shard pool. An actor is pinned to a shard; migration only happens at a quiescent point
/// via application-level checkpoint (never a raw linear-memory snapshot). The last `exclusive_reserve`
/// shards are reserved for [`ShardTable::request_exclusive`] leases.
#[derive(Clone, Debug, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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
    package: PackageId,
    lane: Lane,
    budget: Budget,
    shard: ShardId,
    mailbox: Mailbox,
    active: bool,
    throttle: f32,
    deficit: i64,
}

/// ⏱️ Result of one [`Scheduler::tick`] call: the turns granted this call, and (if nothing ran) the
/// earliest future timestamp worth ticking again for.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct Decision {
    pub run: Vec<TurnGrant>,
    pub wake_at: Option<u64>,
}

impl Decision {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_vec(out, &self.run, TurnGrant::pack_encode).await;
        pack::write_opt_u64(out, &self.wake_at).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { run: pack::read_vec(bytes, pos, "Decision::run", TurnGrant::pack_decode).await?, wake_at: pack::read_opt_u64(bytes, pos, "Decision::wake_at").await? })
    }
}

/// ⏱️ One granted turn: which actor, on which shard, with what (possibly throttle-scaled) budget,
/// and the envelopes drained from its mailbox for this turn.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
pub struct TurnGrant {
    pub actor: ActorId,
    pub shard: ShardId,
    pub budget: Budget,
    pub envelopes: Vec<Envelope>,
}

impl TurnGrant {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.actor.pack_encode(out).await;
        self.shard.pack_encode(out).await;
        self.budget.pack_encode(out).await;
        pack::write_vec(out, &self.envelopes, Envelope::pack_encode).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            actor: ActorId::pack_decode(bytes, pos).await?,
            shard: ShardId::pack_decode(bytes, pos).await?,
            budget: Budget::pack_decode(bytes, pos).await?,
            envelopes: pack::read_vec(bytes, pos, "TurnGrant::envelopes", Envelope::pack_decode).await?,
        })
    }
}

/// ⏱️ How many envelopes a single granted turn drains from its actor's mailbox in one go.
const TURN_ENVELOPE_BATCH: usize = 8;

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

    pub async fn register_actor(&mut self, actor: ActorId, package: PackageId, lane: Lane, budget: Budget, shard: ShardId) {
        if !self.plugin_order.contains(&package) {
            self.plugin_order.push(package.clone());
        }
        self.actors.insert(actor, ScheduledActor { package, lane, budget, shard, mailbox: Mailbox::new(budget.mailbox_len).await, active: true, throttle: 1.0, deficit: 0 });
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
        let mut overdue: Vec<(u64, ActorId)> = self.actors.iter().filter(|(_, e)| e.active && !e.mailbox.is_empty()).filter_map(|(id, e)| e.mailbox.earliest_deadline().filter(|d| *d <= now_ms).map(|d| (d, *id))).collect();
        overdue.sort();
        for (_, actor_id) in overdue {
            if budget_left == 0 {
                break;
            }
            if granted_this_tick.contains(&actor_id) {
                continue;
            }
            if let Some(grant) = self.drain_turn(actor_id).await {
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
                let pending_actor_ids: Vec<ActorId> = self.actors.iter().filter(|(id, e)| e.package == package && e.active && !granted_this_tick.contains(*id) && !e.mailbox.is_empty()).map(|(id, _)| *id).collect();
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
                    let mut candidates: Vec<ActorId> = self.actors.iter().filter(|(id, e)| e.package == package && e.active && !granted_this_tick.contains(*id) && !e.mailbox.is_empty()).map(|(id, _)| *id).collect();
                    candidates.sort();
                    let Some(actor_id) = self.pick_level2(&candidates).await else { break };
                    let weight = self.actors.get(&actor_id).map_or(1, Self::actor_weight);
                    if let Some(grant) = self.drain_turn(actor_id).await {
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
        let wake_at = self.actors.values().filter(|e| e.active && !e.mailbox.is_empty()).filter_map(|e| e.mailbox.earliest_deadline()).min();
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

    async fn drain_turn(&mut self, actor_id: ActorId) -> Option<TurnGrant> {
        let entry = self.actors.get_mut(&actor_id)?;
        let mut envelopes = Vec::new();
        for _ in 0..TURN_ENVELOPE_BATCH {
            match entry.mailbox.pop_next().await {
                Some(e) => envelopes.push(e),
                None => break,
            }
        }
        if envelopes.is_empty() {
            return None;
        }
        entry.deficit -= Self::actor_weight(entry);
        Some(TurnGrant { actor: actor_id, shard: entry.shard, budget: entry.budget.scaled(entry.throttle).await, envelopes })
    }
}
//#endregion ⏱️Scheduler

//#region 🖼️Scene
/// 🖼️ One committed, immutable frame of a window's UI. `patches` is an opaque pack-encoded
/// `Vec<UiPatch>` blob — see the module seam docstring; `node_count` is the host-tracked total used
/// only for [`Budget::ui_nodes`] quota accounting.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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

#[derive(Clone, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[value(crate = "::protocol::value")]
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
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownActor => formatter.write_str("unknown actor"),
            Self::NoExclusiveShard => formatter.write_str("no exclusive shard available"),
            Self::InvalidTransition => formatter.write_str("invalid status transition"),
        }
    }
}

impl std::error::Error for KernelError {}

/// 🏛️ Per-actor bookkeeping the [`Scheduler`] doesn't need to see: kind, capabilities, status,
/// failure ladder, metrics, and which window (if any) its scene patches target.
#[derive(Clone, Debug)]
struct ActorMeta {
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
#[derive(Clone, Debug)]
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
    pub async fn activate(&mut self, package: PackageId, plugin_ordinal: u16, kind: ActorKind, lane: Lane, window: Option<WindowId>, _event: ActivationEvent) -> ActorId {
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
        self.scheduler.register_actor(id, package.clone(), lane, budget, shard).await;
        self.actors.insert(id, ActorMeta { kind, package, capabilities: Vec::new(), budget, status: ActorStatus::Activating, failure: FailureState::new(), metrics: ActorMetrics::default(), window });
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

    /// ✅️ Records a turn's result against its actor: usage metrics, failure-ladder update (clean
    /// turn vs. `Faulted`), and status transition. Returns the escalation the caller (host) must
    /// act on for `Trapped`/`Quarantined` outcomes.
    pub async fn complete(&mut self, actor: ActorId, result: &TurnResult, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
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
        meta.status = ActorStatus::Suspended { checkpoint };
        self.scheduler.set_active(actor, false).await;
        Ok(())
    }

    pub async fn resume(&mut self, actor: ActorId) -> Result<(), KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
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
        let shard = self.shards.request_exclusive(actor).await.ok_or(KernelError::NoExclusiveShard)?;
        self.scheduler.set_shard(actor, shard).await;
        Ok(shard)
    }

    pub async fn release_exclusive(&mut self, actor: ActorId) {
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
    ) -> ActorId {
        let ordinal = self.next_ordinal.entry(package.clone()).or_insert(0);
        let id = ActorId::new(plugin_ordinal, kind.tag().await, *ordinal, 0).await;
        *ordinal += 1;
        let budget = lane_defaults::budget_for(lane);
        let shard = self.shards.pin_to(id, shard).await;
        self.scheduler.register_actor(id, package.clone(), lane, budget, shard).await;
        let capabilities = match parent {
            Some(parent_id) => {
                let granted = self.actors.get(&parent_id).map(|meta| meta.capabilities.clone()).unwrap_or_default();
                intersect_capabilities(&granted, &requested_capabilities).await
            }
            None => requested_capabilities,
        };
        self.actors.insert(id, ActorMeta { kind, package, capabilities, budget, status: ActorStatus::Activating, failure: FailureState::new(), metrics: ActorMetrics::default(), window });
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
mod tests {
    mod quick {
        use crate::*;
        use std::collections::VecDeque;
        use std::sync::Arc;

        //#region 🔖️ErrorContracts
        fn assert_error_contract(error: &(dyn std::error::Error + 'static), expected: &str) {
            assert_eq!(error.to_string(), expected);
            assert!(error.source().is_none());
        }

        #[semio_framework_async_macros::async_test]
        async fn owned_errors_preserve_thiserror_display_and_source_contracts() {
            let cases: Vec<(Box<dyn std::error::Error>, &str)> = vec![
                (Box::new(pack::PackError::Truncated(7, "header")), "pack: truncated at offset 7 reading header"),
                (Box::new(pack::PackError::InvalidTag { what: "Lane", tag: 9, offset: 12 }), "pack: invalid tag 9 for Lane at offset 12"),
                (Box::new(pack::PackError::InvalidUtf8("PackageId", 23)), "pack: invalid utf8 in PackageId at offset 23"),
                (Box::new(pack::PackError::OverlongVarint(31)), "pack: overlong varint at offset 31"),
                (Box::new(JobPublicationError::OperationMismatch { active: 5, published: 8 }), "job bridge operation mismatch: active=5, published=8"),
                (Box::new(JobPublicationError::Stale { live_revision: 13, live_generation: 21 }), "job bridge stale revision/generation: live revision=13, generation=21"),
                (Box::new(JobPublicationError::StepSequence { expected: 34, published: 55 }), "job bridge step sequence mismatch: expected=34, published=55"),
                (Box::new(JobPublicationError::PreviewSequence { before: 89, after: 144 }), "job bridge preview cursor mismatch: before=89, after=144"),
                (Box::new(JobPublicationError::Terminal), "job bridge received a turn after a terminal publication"),
                (Box::new(KernelError::UnknownActor), "unknown actor"),
                (Box::new(KernelError::NoExclusiveShard), "no exclusive shard available"),
                (Box::new(KernelError::InvalidTransition), "invalid status transition"),
            ];

            for (error, expected) in cases {
                assert_error_contract(error.as_ref(), expected);
            }
        }
        //#endregion 🔖️ErrorContracts

        //#region 🔖️Helpers
        async fn env(to: ActorId, lane: Lane, seq: u64) -> Envelope {
            Envelope { to, from: Origin::Kernel, lane, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![1, 2, 3] } }
        }

        async fn ok_turn() -> TurnResult {
            TurnResult { ui_patches: vec![], effects: vec![], command_ingress: vec![], lifecycle_receipt: None, ui_patch_receipt: None, next_wake: None, status: TurnStatus::Idle, usage: Usage { fuel: 100, wall_us: 50, memory_bytes: 1024 } }
        }

        fn bridge_operation() -> job::Operation {
            job::Operation::new(job::OperationId(91), job::RevisionId(7), job::Generation(3), 0x5eed)
        }

        fn bridge_turn(step_sequence: u64, preview_sequence: u64) -> JobTurn {
            let mut operation = bridge_operation();
            operation.preview_sequence = preview_sequence;
            JobTurn { job: 44, operation: JobOperation::from_job(operation), step_sequence }
        }

        fn bridge_now_us() -> Option<u64> {
            Some(10)
        }

        #[derive(Default)]
        struct ScriptJob {
            outcomes: VecDeque<JobStepOutcome>,
            calls: usize,
            pending_state: Option<job::RetainedJobPayload>,
            pending_complete: Option<JobCommitCandidate>,
            closing: bool,
        }

        impl job::InteractiveJob for ScriptJob {
            fn step(&mut self, cx: &mut job::StepContext<'_>) -> job::StepOutcome {
                self.calls += 1;
                if let Some(candidate) = self.pending_complete.take() {
                    let output = cx.payload_from_bytes(job::JobPayloadStream::CommitOutput, &candidate.output).expect("scripted output payload");
                    let state = self.pending_state.take().expect("scripted state payload");
                    return job::StepOutcome::Complete(job::CommitCandidate { state, output });
                }
                let outcome = self.outcomes.pop_front().expect("scripted bridge outcome");
                match outcome {
                    JobStepOutcome::Yield => job::StepOutcome::Yield,
                    JobStepOutcome::PreviewReady { preview } => {
                        cx.next_preview_sequence();
                        let preview = cx.payload_from_bytes(job::JobPayloadStream::Preview, &preview).expect("scripted preview payload");
                        job::StepOutcome::PreviewReady(preview)
                    }
                    JobStepOutcome::CheckpointReady { checkpoint } => {
                        let state = cx.payload_from_bytes(job::JobPayloadStream::CheckpointState, &checkpoint.state).expect("scripted checkpoint payload");
                        job::StepOutcome::CheckpointReady(job::Checkpoint { state, applied_progress: checkpoint.applied_progress })
                    }
                    JobStepOutcome::Complete { candidate } => {
                        self.pending_state = Some(cx.payload_from_bytes(job::JobPayloadStream::CommitState, &candidate.state).expect("scripted commit state"));
                        self.pending_complete = Some(candidate);
                        job::StepOutcome::Yield
                    }
                    JobStepOutcome::Cancelled => job::StepOutcome::Cancelled,
                    JobStepOutcome::Fault { detail } => {
                        let detail = cx.payload_from_bytes(job::JobPayloadStream::Fault, &detail).expect("scripted fault payload");
                        job::StepOutcome::Fault(job::JobFault { detail })
                    }
                }
            }

            fn begin_close(&mut self) {
                self.closing = true;
            }

            fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> job::InteractiveJobCloseStep {
                self.begin_close();
                if let Some(state) = self.pending_state.as_mut() {
                    return match state.close_step(maximum_items, maximum_bytes) {
                        job::JobPayloadCloseStep::Pending { released_items, released_bytes } => job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                        job::JobPayloadCloseStep::Complete => {
                            self.pending_state = None;
                            job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                        }
                    };
                }
                if maximum_items == 0 && (!self.outcomes.is_empty() || self.pending_complete.is_some()) {
                    return job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                if self.pending_complete.take().is_some() || self.outcomes.pop_front().is_some() {
                    return job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                job::InteractiveJobCloseStep::Complete
            }

            fn terminal_is_empty(&self) -> bool {
                self.closing && self.outcomes.is_empty() && self.pending_state.is_none() && self.pending_complete.is_none()
            }
        }

        struct UnsequencedPreviewJob;

        impl job::InteractiveJob for UnsequencedPreviewJob {
            fn step(&mut self, _cx: &mut job::StepContext<'_>) -> job::StepOutcome {
                job::StepOutcome::PreviewReady(job::RetainedJobPayload::empty(job::JobPayloadStream::Preview))
            }

            fn begin_close(&mut self) {}

            fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> job::InteractiveJobCloseStep {
                job::InteractiveJobCloseStep::Complete
            }

            fn terminal_is_empty(&self) -> bool {
                true
            }
        }
        //#endregion 🔖️Helpers

        //#region 🪪️JobBridge
        #[test]
        fn job_bridge_invokes_exactly_one_step_per_turn() {
            let operation = bridge_operation();
            let mut bridge = JobTurnBridge::new(operation);
            let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield, JobStepOutcome::Yield]), calls: 0, ..Default::default() };
            let publication = bridge
                .step(
                    &mut job,
                    bridge_turn(0, 0),
                    operation.operation,
                    operation.base_revision,
                    operation.generation,
                    "actor.job.one-step",
                    job::InteractiveStage::InteractiveStep,
                    job::StepBudget::new(100, 20),
                    job::root_cancel_token(),
                    bridge_now_us,
                )
                .expect("first job turn");
            assert_eq!(job.calls, 1);
            assert!(matches!(publication.outcome, JobStepOutcome::Yield));
        }

        #[test]
        fn job_bridge_preserves_checkpoint_state_and_applied_progress() {
            let operation = bridge_operation();
            let mut bridge = JobTurnBridge::new(operation);
            let checkpoint = JobCheckpoint { state: vec![4, 5, 6], applied_progress: 73 };
            let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::CheckpointReady { checkpoint: checkpoint.clone() }]), calls: 0, ..Default::default() };
            let pending = bridge
                .step(
                    &mut job,
                    bridge_turn(0, 0),
                    operation.operation,
                    operation.base_revision,
                    operation.generation,
                    "actor.job.checkpoint",
                    job::InteractiveStage::BackgroundStep,
                    job::StepBudget::new(100, 20),
                    job::root_cancel_token(),
                    bridge_now_us,
                )
                .expect("checkpoint projection admission");
            assert!(matches!(pending.outcome, JobStepOutcome::Yield));
            let publication = bridge
                .step(
                    &mut job,
                    JobTurn { step_sequence: 1, ..pending.turn },
                    operation.operation,
                    operation.base_revision,
                    operation.generation,
                    "actor.job.checkpoint",
                    job::InteractiveStage::BackgroundStep,
                    job::StepBudget::new(100, 20),
                    job::root_cancel_token(),
                    bridge_now_us,
                )
                .expect("checkpoint publication");
            assert_eq!(publication.outcome, JobStepOutcome::CheckpointReady { checkpoint: checkpoint.clone() });
            assert_eq!(publication.turn_status(), TurnStatus::CheckpointReady { checkpoint });
        }

        #[semio_framework_async_macros::async_test]
        async fn job_bridge_cancellation_is_terminal_and_skips_the_job() {
            let operation = bridge_operation();
            let mut bridge = JobTurnBridge::new(operation);
            let cancel = job::root_cancel_token();
            cancel.cancel().await;
            let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield]), calls: 0, ..Default::default() };
            let publication = bridge
                .step(&mut job, bridge_turn(0, 0), operation.operation, operation.base_revision, operation.generation, "actor.job.cancel", job::InteractiveStage::InteractiveStep, job::StepBudget::new(100, 20), cancel, bridge_now_us)
                .expect("cancel publication");
            assert_eq!(job.calls, 0);
            assert!(matches!(publication.outcome, JobStepOutcome::Cancelled));
            assert!(matches!(
                bridge.step(
                    &mut job,
                    bridge_turn(1, 0),
                    operation.operation,
                    operation.base_revision,
                    operation.generation,
                    "actor.job.cancel",
                    job::InteractiveStage::InteractiveStep,
                    job::StepBudget::new(100, 20),
                    job::root_cancel_token(),
                    bridge_now_us
                ),
                Err(JobPublicationError::Terminal)
            ));
        }

        #[test]
        fn job_bridge_rejects_stale_commit_before_work_or_publication() {
            let operation = bridge_operation();
            let mut bridge = JobTurnBridge::new(operation);
            let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![1], output: vec![2] } }]), calls: 0, ..Default::default() };
            let result = bridge.step(
                &mut job,
                bridge_turn(0, 0),
                operation.operation,
                job::RevisionId(8),
                operation.generation,
                "actor.job.stale",
                job::InteractiveStage::InteractiveStep,
                job::StepBudget::new(100, 20),
                job::root_cancel_token(),
                bridge_now_us,
            );
            assert!(matches!(result, Err(JobPublicationError::Stale { live_revision: 8, live_generation: 3 })));
            assert_eq!(job.calls, 0);
        }

        #[test]
        fn job_bridge_rejects_replayed_preview_identity_before_work() {
            let operation = bridge_operation();
            let mut bridge = JobTurnBridge::new(operation);
            let mut job = ScriptJob { outcomes: VecDeque::from([JobStepOutcome::Yield]), calls: 0, ..Default::default() };
            assert!(matches!(
                bridge.step(
                    &mut job,
                    bridge_turn(0, 1),
                    operation.operation,
                    operation.base_revision,
                    operation.generation,
                    "actor.job.replayed-preview",
                    job::InteractiveStage::InteractiveStep,
                    job::StepBudget::new(100, 20),
                    job::root_cancel_token(),
                    bridge_now_us,
                ),
                Err(JobPublicationError::Stale { .. })
            ));
            assert_eq!(job.calls, 0);
        }

        #[test]
        fn job_bridge_rejects_a_preview_without_exactly_one_sequence_advance() {
            let operation = bridge_operation();
            let mut bridge = JobTurnBridge::new(operation);
            assert!(matches!(
                bridge.step(
                    &mut UnsequencedPreviewJob,
                    bridge_turn(0, 0),
                    operation.operation,
                    operation.base_revision,
                    operation.generation,
                    "actor.job.preview-order",
                    job::InteractiveStage::InteractiveStep,
                    job::StepBudget::new(100, 20),
                    job::root_cancel_token(),
                    bridge_now_us,
                ),
                Err(JobPublicationError::PreviewSequence { before: 0, after: 0 })
            ));
        }

        #[test]
        fn mounted_fixed_replay_capture_is_deterministic_and_returns_the_exact_live_owner() {
            fn run(worker_count: u16, worker_slot: u16) -> [u64; 4] {
                let operation = bridge_operation();
                let mut bridge = JobTurnBridge::new(operation);
                let mut job = ScriptJob {
                    outcomes: VecDeque::from([
                        JobStepOutcome::Yield,
                        JobStepOutcome::PreviewReady { preview: vec![9, 8] },
                        JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![7, 6], applied_progress: 5 } },
                        JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![2, 1] } },
                    ]),
                    calls: 0,
                    ..Default::default()
                };
                let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, document: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
                let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
                let mut turn = bridge_turn(0, 0);
                loop {
                    let publication = bridge
                        .step(&mut job, turn, operation.operation, operation.base_revision, operation.generation, "actor.job.replay", job::InteractiveStage::InteractiveStep, job::StepBudget::new(100, 20), job::root_cancel_token(), bridge_now_us)
                        .expect("replay publication");
                    let terminal = matches!(&publication.outcome, JobStepOutcome::Complete { .. });
                    turn = JobTurn { step_sequence: publication.turn.step_sequence + 1, ..publication.turn };
                    let payload_identity = match &publication.outcome {
                        JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                        JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.as_ptr(),
                        JobStepOutcome::Complete { candidate } => candidate.output.as_ptr(),
                        _ => std::ptr::null(),
                    };
                    let mut preview_sequence = publication.turn.operation.preview_sequence;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut preview_sequence);
                    log.begin_capture(&mut context, ActorId(9), worker_count, worker_slot, 1, 4, publication).expect("capture admission");
                    loop {
                        let mut preview_sequence = turn.operation.preview_sequence;
                        let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut preview_sequence);
                        if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                            break;
                        }
                    }
                    let publication = log.take_captured_publication().expect("same live publication");
                    let returned_identity = match &publication.outcome {
                        JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                        JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.as_ptr(),
                        JobStepOutcome::Complete { candidate } => candidate.output.as_ptr(),
                        _ => std::ptr::null(),
                    };
                    assert_eq!(returned_identity, payload_identity);
                    drop(publication);
                    let now = job::default_now_us().expect("native test clock");
                    let mut sequence = 0;
                    let mut context = job::StepContext::new(job::OperationId(operation.operation.0), job::Generation(operation.generation.0), job::StepBudget::from_duration(1, now, 4_000).expect("test deadline"), job::root_cancel_token(), job::default_now_us, &mut sequence);
                    log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("P2d ACK");
                    if terminal {
                        break;
                    }
                }
                log.begin_replay(operation.generation.0).expect("sealed generation replay");
                let mut replay_bridge = JobTurnBridge::new(operation);
                let mut replay_job = ScriptJob {
                    outcomes: VecDeque::from([
                        JobStepOutcome::Yield,
                        JobStepOutcome::PreviewReady { preview: vec![9, 8] },
                        JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![7, 6], applied_progress: 5 } },
                        JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![2, 1] } },
                    ]),
                    calls: 0,
                    ..Default::default()
                };
                let mut replay_turn = bridge_turn(0, 0);
                loop {
                    let publication = replay_bridge
                        .step(
                            &mut replay_job,
                            replay_turn,
                            operation.operation,
                            operation.base_revision,
                            operation.generation,
                            "actor.job.replay-live",
                            job::InteractiveStage::InteractiveStep,
                            job::StepBudget::new(100, 20),
                            job::root_cancel_token(),
                            bridge_now_us,
                        )
                        .expect("replayed live publication");
                    assert_eq!(log.expected_turn(), Some(publication.turn));
                    let terminal = matches!(&publication.outcome, JobStepOutcome::Complete { .. });
                    replay_turn = JobTurn { step_sequence: publication.turn.step_sequence + 1, ..publication.turn };
                    let mut preview_sequence = publication.turn.operation.preview_sequence;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut preview_sequence);
                    log.begin_capture(&mut context, ActorId(9), worker_count, worker_slot, 1, 4, publication).expect("replay capture admission");
                    loop {
                        let mut preview_sequence = replay_turn.operation.preview_sequence;
                        let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut preview_sequence);
                        if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                            break;
                        }
                    }
                    drop(log.take_captured_publication().expect("matched replay publication"));
                    let mut preview_sequence = replay_turn.operation.preview_sequence;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut preview_sequence);
                    log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("matched replay policy");
                    while log.has_pending_work() {
                        let mut preview_sequence = replay_turn.operation.preview_sequence;
                        let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut preview_sequence);
                        let _ = log.maintenance_step(&mut context);
                    }
                    if terminal {
                        break;
                    }
                }
                assert!(log.expected_turn().is_none());
                let digests = std::array::from_fn(|index| log.record_header(index).expect("sealed record").prefix_digest);
                log.begin_close();
                while !log.terminal_is_empty() {
                    let mut preview_sequence = 0;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut preview_sequence);
                    let _ = log.close_step(&mut context);
                }
                digests
            }

            let host_default = u16::try_from(std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1)).unwrap_or(u16::MAX);
            for worker_count in [1, 2, 4, host_default] {
                assert_eq!(run(worker_count, 0), run(worker_count, 0), "1/2/4/default worker replay preserves the exact ordered publication identity");
            }
        }

        #[test]
        fn mounted_replay_records_and_replays_the_exact_cancelled_terminal_classification() {
            let operation = bridge_operation();
            let route = JobReplayRoute { plugin: [11; 32], package: [13; 32], controller: [17; 32], tool: [19; 32], window: 23, document: [29; 32], request_schema: [31; 32], request_version: 1, request_digest: [37; 32] };
            let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
            let turn = bridge_turn(0, 0);
            for replaying in [false, true] {
                let publication = JobPublication { turn, outcome: JobStepOutcome::Cancelled };
                let mut sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
                log.begin_capture(&mut context, ActorId(9), 1, 0, 1, 4, publication).expect("cancel capture admission");
                loop {
                    let mut sequence = 0;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
                    if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                        break;
                    }
                }
                assert_eq!(log.record_header(0).expect("cancel record").cancellation_observed, true);
                assert!(matches!(log.take_captured_publication().expect("exact cancelled owner").outcome, JobStepOutcome::Cancelled));
                let mut sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
                log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("cancel ACK");
                if !replaying {
                    log.begin_replay(operation.generation.0).expect("cancel replay generation");
                }
            }
            assert!(log.expected_turn().is_none());
            log.begin_close();
            while !log.terminal_is_empty() {
                let mut sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
                let _ = log.close_step(&mut context);
            }
        }

        #[test]
        fn mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged() {
            let operation = bridge_operation();
            let route = JobReplayRoute { plugin: [41; 32], package: [43; 32], controller: [47; 32], tool: [53; 32], window: 59, document: [61; 32], request_schema: [67; 32], request_version: 1, request_digest: [71; 32] };
            let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
            let publication = |generation: u64| JobPublication { turn: JobTurn { operation: JobOperation { generation, ..bridge_turn(0, 0).operation }, ..bridge_turn(0, 0) }, outcome: JobStepOutcome::PreviewReady { preview: vec![73, 79] } };

            let cancelled = publication(operation.generation.0);
            let cancelled_identity = match &cancelled.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            };
            let cancel = job::root_cancel_token();
            cancel.cancel_now();
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), cancel, bridge_now_us, &mut sequence);
            let cancelled = log.begin_capture(&mut context, ActorId(9), 1, 0, 1, 4, cancelled).expect_err("cancelled capture refuses before transfer").into_publication();
            assert!(matches!(&cancelled.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == cancelled_identity));

            let expired = publication(operation.generation.0);
            let expired_identity = match &expired.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            };
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 10), job::root_cancel_token(), bridge_now_us, &mut sequence);
            let expired = log.begin_capture(&mut context, ActorId(9), 1, 0, 1, 4, expired).expect_err("expired capture refuses before transfer").into_publication();
            assert!(matches!(&expired.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == expired_identity));

            let stale = publication(operation.generation.0 + 1);
            let stale_identity = match &stale.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            };
            let mut sequence = 0;
            let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
            let stale = log.begin_capture(&mut context, ActorId(9), 1, 0, 1, 4, stale).expect_err("stale capture refuses before transfer").into_publication();
            assert!(matches!(&stale.outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == stale_identity));
            assert_eq!(log.sealed_records(), 0);
            assert!(!log.has_pending_work());
            log.begin_close();
            assert!(log.terminal_is_empty());
        }

        #[test]
        fn mounted_replay_preserves_the_exact_fault_payload_and_prefix_across_replay() {
            let operation = bridge_operation();
            let route = JobReplayRoute { plugin: [73; 32], package: [79; 32], controller: [83; 32], tool: [89; 32], window: 97, document: [101; 32], request_schema: [103; 32], request_version: 1, request_digest: [107; 32] };
            let mut log = JobReplayLog::new(route, operation.generation.0).expect("fixed replay authority");
            let turn = bridge_turn(0, 0);
            for replaying in [false, true] {
                let publication = JobPublication { turn, outcome: JobStepOutcome::Fault { detail: vec![109, 113, 127] } };
                let payload_identity = match &publication.outcome {
                    JobStepOutcome::Fault { detail } => detail.as_ptr(),
                    _ => unreachable!(),
                };
                let mut sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
                log.begin_capture(&mut context, ActorId(9), 1, 0, 1, 4, publication).expect("fault capture admission");
                loop {
                    let mut sequence = 0;
                    let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
                    if log.capture_step(&mut context) == JobReplayStep::PublicationReady {
                        break;
                    }
                }
                let publication = log.take_captured_publication().expect("exact fault owner");
                assert!(matches!(&publication.outcome, JobStepOutcome::Fault { detail } if detail.as_ptr() == payload_identity && detail.as_slice() == [109, 113, 127]));
                let mut sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
                log.acknowledge_publication(&mut context, JobReplayPublicationPolicy::Accepted).expect("fault ACK");
                if !replaying {
                    log.begin_replay(operation.generation.0).expect("fault replay generation");
                }
            }
            assert!(log.expected_turn().is_none());
            log.begin_close();
            while !log.terminal_is_empty() {
                let mut sequence = 0;
                let mut context = job::StepContext::new(operation.operation, operation.generation, job::StepBudget::new(1, 20), job::root_cancel_token(), bridge_now_us, &mut sequence);
                let _ = log.close_step(&mut context);
            }
        }
        //#endregion 🪪️JobBridge

        //#region 🔖️PackRoundTrips
        macro_rules! round_trip {
            ($name:ident, $ty:ty, $value:expr) => {
                #[semio_framework_async_macros::async_test]
                async fn $name() {
                    let value: $ty = $value;
                    let mut bytes = Vec::new();
                    value.pack_encode(&mut bytes).await;
                    let mut pos = 0usize;
                    let decoded = <$ty>::pack_decode(&bytes, &mut pos).await.unwrap();
                    assert_eq!(pos, bytes.len());
                    assert_eq!(decoded, value);
                }
            };
        }

        round_trip!(pack_round_trip_package_id, PackageId, PackageId("s.cad/extrude".into()));
        round_trip!(pack_round_trip_package_hash, PackageHash, PackageHash([7u8; 32]));
        round_trip!(pack_round_trip_actor_id, ActorId, ActorId::new(42, 1, 99, 3).await);
        round_trip!(pack_round_trip_actor_kind, ActorKind, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "s.cad.editor".into(), instance_id: 7 });
        round_trip!(pack_round_trip_lane, Lane, Lane::UserVisible);
        round_trip!(pack_round_trip_budget, Budget, lane_defaults::budget_for(Lane::Interactive));
        round_trip!(pack_round_trip_window_id, WindowId, WindowId(5));
        round_trip!(pack_round_trip_origin, Origin, Origin::Bus { topic: "os.runtime.metrics".into() });
        round_trip!(pack_round_trip_job_operation, JobOperation, JobOperation { operation: 11, base_revision: 7, generation: 3, preview_sequence: 2, seed: 99 });
        round_trip!(pack_round_trip_job_checkpoint, JobCheckpoint, JobCheckpoint { state: vec![9, 8, 7], applied_progress: 42 });
        round_trip!(pack_round_trip_job_step_outcome, JobStepOutcome, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![6, 5], output: vec![4, 3] } });
        round_trip!(
            pack_round_trip_job_publication,
            JobPublication,
            JobPublication {
                turn: JobTurn { job: 44, operation: JobOperation { operation: 11, base_revision: 7, generation: 3, preview_sequence: 1, seed: 99 }, step_sequence: 5 },
                outcome: JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![2, 1], applied_progress: 73 } },
            }
        );
        round_trip!(
            pack_round_trip_payload,
            Payload,
            Payload::Resume { operation: JobOperation { operation: 11, base_revision: 7, generation: 3, preview_sequence: 2, seed: 99 }, checkpoint: JobCheckpoint { state: vec![9, 9, 9], applied_progress: 42 } }
        );
        round_trip!(pack_round_trip_coalesce_key, CoalesceKey, CoalesceKey("pointer-move".into()));
        round_trip!(pack_round_trip_envelope, Envelope, env(ActorId::new(1, 0, 1, 0).await, Lane::Interactive, 7).await);
        round_trip!(pack_round_trip_turn_status, TurnStatus, TurnStatus::Faulted { detail: vec![1, 2] });
        round_trip!(pack_round_trip_usage, Usage, Usage { fuel: 1, wall_us: 2, memory_bytes: 3 });
        round_trip!(pack_round_trip_turn_result, TurnResult, ok_turn().await);
        round_trip!(pack_round_trip_backpressure, Backpressure, Backpressure::Dropped { lane: Lane::Background });
        round_trip!(pack_round_trip_capability_grant, CapabilityGrant, CapabilityGrant { capability: "fs.read".into(), scope: Some(vec![1]) });
        round_trip!(pack_round_trip_failure_signal, FailureSignal, FailureSignal::HeartbeatMissed { count: 2 });
        round_trip!(pack_round_trip_failure_stage, FailureStage, FailureStage::Throttled { factor: 0.25 });
        round_trip!(pack_round_trip_failure_state, FailureState, FailureState { stage: FailureStage::Warned, clean_turns: 1, warn_count: 2, restart_count: 0, last_signal_ms: 500 });
        round_trip!(pack_round_trip_actor_status, ActorStatus, ActorStatus::Suspended { checkpoint: Some(vec![1, 2, 3]) });
        round_trip!(pack_round_trip_shard_id, ShardId, ShardId(3));
        round_trip!(pack_round_trip_shard_kind, ShardKind, ShardKind::WebWorker);
        round_trip!(pack_round_trip_decision, Decision, Decision { run: vec![TurnGrant { actor: ActorId::new(1, 0, 0, 0).await, shard: ShardId(0), budget: lane_defaults::budget_for(Lane::Background), envelopes: vec![] }], wake_at: Some(10) });
        round_trip!(
            pack_round_trip_turn_grant,
            TurnGrant,
            TurnGrant { actor: ActorId::new(2, 1, 3, 0).await, shard: ShardId(1), budget: lane_defaults::budget_for(Lane::Maintenance), envelopes: vec![env(ActorId::new(2, 1, 3, 0).await, Lane::Maintenance, 1).await] }
        );
        round_trip!(pack_round_trip_scene_snapshot, SceneSnapshot, SceneSnapshot { revision: 3, committed_ms: 12, patches: vec![9, 9], node_count: 40 });
        round_trip!(pack_round_trip_shard_metrics, ShardMetrics, ShardMetrics { actors: 3, busy_ratio: 0.5, heartbeat_age_ms: 12 });
        round_trip!(pack_round_trip_kernel_metrics, KernelMetrics, KernelMetrics { actors: 3, shards: 4, packages: 2 });
        round_trip!(
            pack_round_trip_actor_metrics_sample,
            ActorMetricsSample,
            ActorMetricsSample { id: ActorId::new(1, 0, 2, 0).await, package: PackageId("s.cad".into()), lane: Lane::UserVisible, status: ActorStatus::Active, metrics: ActorMetrics::default() }
        );
        round_trip!(pack_round_trip_shard_metrics_sample, ShardMetricsSample, ShardMetricsSample { shard: ShardId(2), metrics: ShardMetrics { actors: 5, busy_ratio: 0.4, heartbeat_age_ms: 8 } });
        round_trip!(
            pack_round_trip_runtime_metrics_snapshot,
            RuntimeMetricsSnapshot,
            RuntimeMetricsSnapshot {
                kernel: KernelMetrics { actors: 1, shards: 1, packages: 1 },
                actors: vec![ActorMetricsSample { id: ActorId::new(0, 0, 0, 0).await, package: PackageId("s.a".into()), lane: Lane::Interactive, status: ActorStatus::Active, metrics: ActorMetrics::default() }],
                shards: vec![ShardMetricsSample { shard: ShardId(0), metrics: ShardMetrics { actors: 1, busy_ratio: 1.0, heartbeat_age_ms: 0 } }],
                sampled_at_ms: 1234,
            }
        );

        #[semio_framework_async_macros::async_test]
        async fn pack_round_trip_mailbox() {
            let mut mailbox = Mailbox::new(4).await;
            mailbox.enqueue(env(ActorId::new(1, 0, 0, 0).await, Lane::Interactive, 1).await).await;
            mailbox.enqueue(env(ActorId::new(1, 0, 0, 0).await, Lane::Background, 2).await).await;
            let mut bytes = Vec::new();
            mailbox.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = Mailbox::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded.len(), mailbox.len());
            assert_eq!(decoded.capacity, mailbox.capacity);
        }

        #[semio_framework_async_macros::async_test]
        async fn pack_round_trip_actor_record() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
            let id = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let record = kernel.actor_record(id).await.unwrap();
            let mut bytes = Vec::new();
            record.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = ActorRecord::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded, record);
        }

        /// 📌️ The property the old `actor.0 % pool` silently violated: 100 actors of one plugin
        /// must SPREAD across the pool, not all land on shard 0. Every actor `activate` mints has
        /// `generation == 0`, and generation occupied the low bits, so the old modulo returned 0 for
        /// all of them — a bench-measured `perShardCounts {"0": 100}`. Asserts distribution, not
        /// mere validity: a "pin returns a shard in range" test passes when every answer is 0.
        #[semio_framework_async_macros::async_test]
        async fn pin_spreads_actors_of_one_plugin_across_the_pool() {
            let mut table = ShardTable::new(ShardKind::Native, 8, 0).await;
            let mut counts: std::collections::BTreeMap<u16, usize> = std::collections::BTreeMap::new();
            for ordinal in 0..100u32 {
                *counts.entry(table.pin(ActorId::new(7, 0, ordinal, 0).await).await.0).or_default() += 1;
            }
            assert_eq!(counts.values().sum::<usize>(), 100);
            assert_eq!(counts.len(), 8, "all 8 shards must receive actors, got {counts:?}");
            assert!(*counts.values().max().unwrap() <= 100 / 8 + 1, "no shard may exceed ceil(100/8)+1: {counts:?}");
        }

        /// 🔁️ Pinning the same actor twice is idempotent — a re-pin must not consume a second slot
        /// and skew the balance.
        #[semio_framework_async_macros::async_test]
        async fn pin_is_idempotent_for_the_same_actor() {
            let mut table = ShardTable::new(ShardKind::Native, 8, 0).await;
            let actor = ActorId::new(3, 0, 11, 0).await;
            assert_eq!(table.pin(actor).await, table.pin(actor).await);
        }

        /// 🕳️ `unpin` leaves a gap; the next `pin` must refill it rather than stride past, which is
        /// exactly what a round-robin counter would do and why placement is least-loaded.
        #[semio_framework_async_macros::async_test]
        async fn pin_refills_the_gap_left_by_unpin() {
            let mut table = ShardTable::new(ShardKind::Native, 4, 0).await;
            let mut actors: Vec<ActorId> = Vec::new();
            for ordinal in 0..8u32 {
                actors.push(ActorId::new(1, 0, ordinal, 0).await);
            }
            for actor in &actors {
                table.pin(*actor).await;
            }
            let freed = table.shard_of(actors[2]).await.unwrap();
            table.unpin(actors[2]).await;
            assert_eq!(table.pin(ActorId::new(1, 0, 99, 0).await).await, freed);
        }

        /// 🔥️ PROPERTY (terra-interactive-isolation): the mechanism this packet's mission asked for —
        /// N CPU-saturating actors sharing a shard must not receive a freshly-activated interactive
        /// actor as a co-resident. Pure/deterministic: injected `Usage`s stand in for real wall-clock
        /// turns (`ActorMetrics::is_saturating`'s whole point is never needing a clock/bench of its
        /// own), no thread/bench needed.
        #[semio_framework_async_macros::async_test]
        async fn interactive_actor_avoids_a_shard_saturated_by_cpu_bound_actors() {
            let mut kernel = Kernel::new(ShardKind::Native, 3, 0, 64).await;
            let background_package = PackageId("cpu-hog".into());
            // 🔢️ 6 Background actors round-robin 2-per-shard across 3 shards under plain least-loaded
            // `pin` (proven deterministic by `pin_spreads_actors_of_one_plugin_across_the_pool`'s own
            // reasoning) — read each one's ACTUAL shard back via `actor_record` rather than assuming
            // the exact order, so this test stays correct even if that internal tie-break ever changes.
            let mut by_shard: std::collections::BTreeMap<u16, Vec<ActorId>> = std::collections::BTreeMap::new();
            for ordinal in 0..6u32 {
                let id = kernel.activate(background_package.clone(), 1, ActorKind::PluginApp { plugin: background_package.clone(), app_id: "hog".into(), instance_id: ordinal }, Lane::Background, None, ActivationEvent::Manual).await;
                let shard = kernel.actor_record(id).await.unwrap().shard;
                by_shard.entry(shard.0).or_default().push(id);
            }
            assert_eq!(by_shard.len(), 3, "expected all 3 shards to receive background actors: {by_shard:?}");

            // 🔥️ Drive every actor on shards OTHER than the last one over its own Background budget's
            // wall_ms ceiling for 2 turns (SATURATION_MIN_TURNS) — crossing `is_saturating`'s
            // threshold. The last shard's actors are left untouched (default `ActorMetrics`, zero
            // turns), making it the one and only "clean" shard.
            let shard_ids: Vec<u16> = by_shard.keys().copied().collect();
            let (safe_shard, hot_shards) = shard_ids.split_last().unwrap();
            let hot_turn = TurnResult { ui_patches: vec![], effects: vec![], command_ingress: vec![], lifecycle_receipt: None, ui_patch_receipt: None, next_wake: None, status: TurnStatus::Idle, usage: Usage { fuel: 100, wall_us: 40_000, memory_bytes: 1024 } };
            for shard in hot_shards {
                for actor in &by_shard[shard] {
                    kernel.complete(*actor, &hot_turn, 0).await.unwrap();
                    kernel.complete(*actor, &hot_turn, 0).await.unwrap();
                }
            }

            let interactive_id = kernel.activate(PackageId("editor".into()), 2, ActorKind::PluginApp { plugin: PackageId("editor".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let interactive_shard = kernel.actor_record(interactive_id).await.unwrap().shard;
            assert_eq!(interactive_shard.0, *safe_shard, "interactive actor must land on the one shard with no CPU-saturating co-resident, got {interactive_shard:?} (hot shards: {hot_shards:?})");
        }

        #[semio_framework_async_macros::async_test]
        async fn pack_round_trip_shard_table() {
            let mut table = ShardTable::new(ShardKind::Native, 4, 1).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            table.pin(actor).await;
            table.request_exclusive(ActorId::new(2, 0, 0, 0).await).await;
            let mut bytes = Vec::new();
            table.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = ShardTable::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded.shard_of(actor).await, table.shard_of(actor).await);
        }

        #[semio_framework_async_macros::async_test]
        async fn pack_round_trip_actor_metrics() {
            let mut metrics = ActorMetrics::default();
            for i in 0..70u64 {
                metrics.record_turn(&Usage { fuel: i, wall_us: i * 3, memory_bytes: 10 }).await;
            }
            let mut bytes = Vec::new();
            metrics.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = ActorMetrics::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded, metrics);
            assert_eq!(decoded.wall_us_p95(), metrics.wall_us_p95());
        }
        //#endregion 🔖️PackRoundTrips

        //#region 🔖️SerdeRoundTrips
        /// 🎯️ terra-shard-grants, Part A. `📌️important.md` rule 12: "after fixing one variant of a
        /// serde-shape defect, sweep every sibling" — the `JobStep::Done`/`Failed` fix recorded this
        /// instruction and nobody executed it for the six siblings sitting in this crate. Each of
        /// these SERIALIZES TO BYTES AND BACK (`serde_json`, not an in-process `assert_eq!` on the
        /// original value) — that distinction is the entire point: a plain equality check on the
        /// original Rust value would pass even if `serde_json::to_vec` panics or errors partway,
        /// exactly how the `JobStep` bug hid for a full wave (its tests only ever asserted on
        /// in-process values, never on bytes that had crossed a serde boundary).
        macro_rules! serde_round_trip {
            ($name:ident, $ty:ty, $value:expr) => {
                #[semio_framework_async_macros::async_test]
                async fn $name() {
                    let value: $ty = $value;
                    let bytes = serde_json::to_vec(&value).expect("serde_json::to_vec must not error — this is the exact defect this test exists to catch");
                    let decoded: $ty = serde_json::from_slice(&bytes).expect("serde_json::from_slice must round-trip what to_vec produced");
                    assert_eq!(decoded, value);
                }
            };
        }

        serde_round_trip!(serde_round_trip_payload_event, Payload, Payload::Event { bytes: vec![1, 2, 3] });
        serde_round_trip!(serde_round_trip_payload_cancel, Payload, Payload::Cancel { seq: 42 });
        serde_round_trip!(serde_round_trip_origin_actor, Origin, Origin::Actor { id: ActorId::new(1, 0, 2, 0).await });
        serde_round_trip!(serde_round_trip_turn_status_faulted, TurnStatus, TurnStatus::Faulted { detail: b"boom".to_vec() });
        serde_round_trip!(serde_round_trip_failure_signal_trap, FailureSignal, FailureSignal::Trap { detail: "trapped".to_string() });
        serde_round_trip!(serde_round_trip_backpressure_dropped, Backpressure, Backpressure::Dropped { lane: Lane::Background });
        //#endregion 🔖️SerdeRoundTrips

        //#region 🔖️ValueRoundTrips
        /// 🌱️ RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01): the additive
        /// `#[derive(ToValue, FromValue)]` pass over this crate's serde types — proves the new trait
        /// actually round-trips at runtime (not merely type-checks) for a representative spread:
        /// a `#[value(transparent)]` newtype, a plain struct, a `tag`-carrying enum, the hand-written
        /// `ActorInstanceLifecycleReceipt` impl (enum-variant `with` fields the derive rejects), and
        /// `ShardTable`'s custom stringified-key `BTreeMap` bridge.
        macro_rules! value_round_trip {
            ($name:ident, $ty:ty, $value:expr) => {
                #[semio_framework_async_macros::async_test]
                async fn $name() {
                    let value: $ty = $value;
                    let encoded = ::protocol::value::ToValue::to_value(&value);
                    let decoded: $ty = ::protocol::value::FromValue::from_value(encoded).expect("FromValue must round-trip what ToValue produced");
                    assert_eq!(decoded, value);
                }
            };
        }

        value_round_trip!(value_round_trip_actor_id, ActorId, ActorId::new(1, 0, 2, 0).await);
        value_round_trip!(value_round_trip_package_id, PackageId, PackageId("semio.demo".to_string()));
        value_round_trip!(value_round_trip_budget, Budget, lane_defaults::budget_for(Lane::Interactive));
        value_round_trip!(value_round_trip_origin_actor, Origin, Origin::Actor { id: ActorId::new(1, 0, 2, 0).await });
        value_round_trip!(value_round_trip_payload_event, Payload, Payload::Event { bytes: vec![1, 2, 3] });
        value_round_trip!(value_round_trip_failure_stage_throttled, FailureStage, FailureStage::Throttled { factor: 0.5 });

        #[semio_framework_async_macros::async_test]
        async fn value_round_trip_actor_instance_lifecycle_receipt() {
            let value = instance_lifetime::ActorInstanceLifecycleReceipt::Accepted {
                lifetime: instance_lifetime::ActorInstanceLifetime { activation_generation: 7, instance_id: 3, guest_lifetime: 9 },
                request_sequence: 11,
                close_generation: 5,
            };
            let encoded = ::protocol::value::ToValue::to_value(&value);
            assert_eq!(encoded["kind"], ::protocol::value::DslValue::String("accepted".to_string()));
            assert_eq!(encoded["requestSequence"], ::protocol::value::DslValue::uint(11));
            assert_eq!(encoded["closeGeneration"], ::protocol::value::DslValue::String("5".to_string()));
            let decoded: instance_lifetime::ActorInstanceLifecycleReceipt = ::protocol::value::FromValue::from_value(encoded).unwrap();
            assert_eq!(decoded, value);
        }

        #[semio_framework_async_macros::async_test]
        async fn value_round_trip_shard_table_stringified_keys() {
            let mut table = ShardTable::new(ShardKind::Native, 4, 1).await;
            table.pin(ActorId::new(1, 0, 2, 0).await).await;
            let encoded = ::protocol::value::ToValue::to_value(&table);
            let ::protocol::value::DslValue::Object(entries) = &encoded else { panic!("expected object") };
            let (_, assignment) = entries.iter().find(|(k, _)| k == "assignment").expect("assignment key present");
            let ::protocol::value::DslValue::Object(assignment_entries) = assignment else { panic!("expected object") };
            assert!(assignment_entries.iter().all(|(k, _)| k.parse::<u64>().is_ok()), "keys must be stringified ActorId integers");
            let decoded: ShardTable = ::protocol::value::FromValue::from_value(encoded).unwrap();
            assert_eq!(decoded.shard_count().await, table.shard_count().await);
        }
        //#endregion 🔖️ValueRoundTrips

        //#region 🔖️ActorIdBitPacking
        #[semio_framework_async_macros::async_test]
        async fn actor_id_bit_packing_round_trips_all_fields() {
            let id = ActorId::new(0xBEEF, 2, 0xC0FFEE, 0x1234 & 0x3FFF).await;
            assert_eq!(id.plugin_ordinal(), 0xBEEF);
            assert_eq!(id.kind_tag(), 2);
            assert_eq!(id.ordinal(), 0xC0FFEE);
            assert_eq!(id.generation(), 0x1234 & 0x3FFF);
        }

        #[semio_framework_async_macros::async_test]
        async fn actor_id_next_generation_bumps_only_generation() {
            let id = ActorId::new(3, 1, 9, 5).await;
            let restarted = id.next_generation().await;
            assert_eq!(restarted.plugin_ordinal(), id.plugin_ordinal());
            assert_eq!(restarted.kind_tag(), id.kind_tag());
            assert_eq!(restarted.ordinal(), id.ordinal());
            assert_eq!(restarted.generation(), id.generation() + 1);
        }
        //#endregion 🔖️ActorIdBitPacking

        //#region 🔖️MailboxTests
        #[semio_framework_async_macros::async_test]
        async fn mailbox_coalesces_latest_wins_older_dropped() {
            let mut mailbox = Mailbox::new(10).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            for i in 0..200u64 {
                let mut e = env(actor, Lane::Interactive, i).await;
                e.coalesce = Some(CoalesceKey("pointer-move".into()));
                e.payload = Payload::Event { bytes: vec![i as u8] };
                let bp = mailbox.enqueue(e).await;
                assert!(matches!(bp, Backpressure::Accept | Backpressure::Coalesced));
            }
            assert_eq!(mailbox.len(), 1, "200 coalesced moves must never queue more than the latest");
            let popped = mailbox.pop_next().await.unwrap();
            assert_eq!(popped.payload, Payload::Event { bytes: vec![199] });
        }

        #[semio_framework_async_macros::async_test]
        async fn mailbox_backpressure_rejected_when_full_and_nothing_lower_priority() {
            let mut mailbox = Mailbox::new(2).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await, Backpressure::Accept);
            assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 2).await).await, Backpressure::Accept);
            let bp = mailbox.enqueue(env(actor, Lane::Maintenance, 3).await).await;
            assert_eq!(bp, Backpressure::Rejected, "no lower-priority lane to evict — must reject, never silently drop");
        }

        #[semio_framework_async_macros::async_test]
        async fn mailbox_backpressure_drops_lower_priority_lane_to_admit_interactive() {
            let mut mailbox = Mailbox::new(2).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await, Backpressure::Accept);
            assert_eq!(mailbox.enqueue(env(actor, Lane::Background, 2).await).await, Backpressure::Accept);
            let bp = mailbox.enqueue(env(actor, Lane::Interactive, 3).await).await;
            assert_eq!(bp, Backpressure::Dropped { lane: Lane::Maintenance });
            assert_eq!(mailbox.len(), 2);
        }

        #[semio_framework_async_macros::async_test]
        async fn mailbox_pop_next_honors_lane_priority_over_fifo() {
            let mut mailbox = Mailbox::new(10).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await;
            mailbox.enqueue(env(actor, Lane::Background, 2).await).await;
            mailbox.enqueue(env(actor, Lane::Interactive, 3).await).await;
            assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Interactive);
            assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Background);
            assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Maintenance);
        }
        //#endregion 🔖️MailboxTests

        //#region 🔖️SchedulerFairness
        #[semio_framework_async_macros::async_test]
        async fn drr_fairness_plugin_with_50_actors_does_not_starve_plugin_with_1() {
            // 🧪️ Both plugins are given abundant backlog up front (nobody runs dry mid-test), so the
            // measured split reflects level-1 DRR's PLUGIN-level fairness, not which plugin happened
            // to have offered work left when the other ran out. Without level 1, `s.busy` (50 actors)
            // would swamp `s.quiet` (1 actor) roughly 50:1 — see design §Scheduler.
            let mut scheduler = Scheduler::new(4).await;
            let busy_package = PackageId("s.busy".into());
            let quiet_package = PackageId("s.quiet".into());
            let budget = lane_defaults::budget_for(Lane::Background);
            let mut busy_actors = Vec::new();
            for i in 0..50u32 {
                let id = ActorId::new(1, 0, i, 0).await;
                scheduler.register_actor(id, busy_package.clone(), Lane::Background, budget, ShardId(0)).await;
                busy_actors.push(id);
            }
            let quiet_actor = ActorId::new(2, 0, 0, 0).await;
            scheduler.register_actor(quiet_actor, quiet_package, Lane::Background, budget, ShardId(1)).await;

            for &id in &busy_actors {
                for seq in 0..30u64 {
                    scheduler.submit(Envelope { to: id, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
                }
            }
            for seq in 0..500u64 {
                scheduler.submit(Envelope { to: quiet_actor, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
            }

            let mut busy_grants = 0u32;
            let mut quiet_grants = 0u32;
            for now in 0..100u64 {
                let decision = scheduler.tick(now).await;
                for grant in &decision.run {
                    if grant.actor == quiet_actor {
                        quiet_grants += 1;
                    } else {
                        busy_grants += 1;
                    }
                }
            }
            assert!(quiet_grants > 0, "the 1-actor plugin must never starve");
            assert!(busy_grants > 0);
            let ratio = busy_grants as f64 / quiet_grants as f64;
            assert!(ratio < 10.0, "level-1 DRR must keep PLUGIN-level share roughly comparable regardless of actor count (busy={busy_grants} quiet={quiet_grants} ratio={ratio}, naive per-actor scheduling would give ~50x)");
        }

        #[semio_framework_async_macros::async_test]
        async fn deadline_preemption_runs_before_background_drr_deficit() {
            let mut scheduler = Scheduler::new(1).await;
            let package = PackageId("s.mixed".into());
            let budget = lane_defaults::budget_for(Lane::Background);
            let bg_actor = ActorId::new(1, 0, 0, 0).await;
            let interactive_actor = ActorId::new(1, 0, 1, 0).await;
            scheduler.register_actor(bg_actor, package.clone(), Lane::Background, budget, ShardId(0)).await;
            scheduler.register_actor(interactive_actor, package, Lane::Interactive, lane_defaults::budget_for(Lane::Interactive), ShardId(0)).await;
            scheduler.submit(Envelope { to: bg_actor, from: Origin::Kernel, lane: Lane::Background, seq: 1, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
            scheduler.submit(Envelope { to: interactive_actor, from: Origin::Kernel, lane: Lane::Interactive, seq: 2, deadline_ms: Some(5), coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
            let decision = scheduler.tick(10).await;
            assert_eq!(decision.run.len(), 1);
            assert_eq!(decision.run[0].actor, interactive_actor, "an overdue interactive deadline must preempt DRR ordering");
        }
        //#endregion 🔖️SchedulerFairness

        //#region 🔖️FailureLadder
        #[semio_framework_async_macros::async_test]
        async fn failure_ladder_escalates_and_decays_back_to_healthy() {
            let mut state = FailureState::new();
            assert_eq!(state.stage, FailureStage::Healthy);

            let esc = state.on_signal(&FailureSignal::DeadlineOverrun { ratio: 1.2 }, Lane::Interactive, 0).await;
            assert_eq!(esc, FailureEscalation::None);
            assert_eq!(state.stage, FailureStage::Warned);

            let esc = state.on_signal(&FailureSignal::DeadlineOverrun { ratio: 1.4 }, Lane::Interactive, 10).await;
            assert_eq!(esc, FailureEscalation::None);
            assert!(matches!(state.stage, FailureStage::Throttled { .. }), "second interactive warn must throttle (threshold=2)");

            let esc = state.on_signal(&FailureSignal::MailboxOverflow, Lane::Interactive, 20).await;
            assert_eq!(esc, FailureEscalation::None);
            assert!(matches!(state.stage, FailureStage::Suspended { .. }), "third interactive warn must suspend (threshold=2 -> suspend_at)");

            for i in 0..200u64 {
                state.on_clean_turn(1_000_000 + i).await;
            }
            assert_eq!(state.stage, FailureStage::Healthy, "sustained clean turns must decay all the way back to Healthy, got {:?}", state.stage);
        }

        #[semio_framework_async_macros::async_test]
        async fn failure_ladder_trap_then_quarantine_is_package_wide() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
            let package = PackageId("s.flaky".into());
            let a = kernel.activate(package.clone(), 1, ActorKind::PluginApp { plugin: package.clone(), app_id: "a".into(), instance_id: 0 }, Lane::Background, None, ActivationEvent::Manual).await;
            let b = kernel.activate(package.clone(), 1, ActorKind::PluginApp { plugin: package, app_id: "b".into(), instance_id: 1 }, Lane::Background, None, ActivationEvent::Manual).await;

            for i in 0..FAILURE_QUARANTINE_RESTART_THRESHOLD {
                let faulted = TurnResult { ui_patches: vec![], effects: vec![], command_ingress: vec![], lifecycle_receipt: None, ui_patch_receipt: None, next_wake: None, status: TurnStatus::Faulted { detail: b"boom".to_vec() }, usage: Usage::default() };
                kernel.complete(a, &faulted, (i as u64) * 100).await.unwrap();
            }
            assert_eq!(kernel.actor_status(a).await, Some(&ActorStatus::Quarantined));
            assert_eq!(kernel.actor_status(b).await, Some(&ActorStatus::Quarantined), "quarantine must be package-wide, not just the trapping actor");
        }

        #[semio_framework_async_macros::async_test]
        async fn failure_ladder_manual_reset_returns_to_healthy_immediately() {
            let mut state = FailureState::new();
            state.on_signal(&FailureSignal::FuelExhausted, Lane::Background, 0).await;
            assert_ne!(state.stage, FailureStage::Healthy);
            state.on_signal(&FailureSignal::ManualReset, Lane::Background, 1).await;
            assert_eq!(state.stage, FailureStage::Healthy);
            assert_eq!(state.warn_count, 0);
        }
        //#endregion 🔖️FailureLadder

        //#region 🔖️Scene
        #[semio_framework_async_macros::async_test]
        async fn scene_revision_is_monotonic_and_reuses_snapshot_on_empty_commit() {
            let mut store = SceneStore::new();
            let budget = lane_defaults::budget_for(Lane::Interactive);
            let actor = ActorId::new(1, 0, 0, 0).await;
            let first = store.commit_frame(0).await;
            assert_eq!(first.revision, 0, "no pending patches yet -> initial empty snapshot, revision 0");

            store.apply_patch(actor, vec![1, 2, 3], 10, &budget).await.unwrap();
            let second = store.commit_frame(16).await;
            assert_eq!(second.revision, 1);
            assert!(second.committed_ms >= first.committed_ms);

            let third = store.commit_frame(32).await;
            assert_eq!(third.revision, second.revision, "nothing pending -> previous snapshot reused, same revision");
            assert!(Arc::ptr_eq(&second, &third));
        }

        #[semio_framework_async_macros::async_test]
        async fn scene_ui_node_quota_truncates_and_signals() {
            let mut store = SceneStore::new();
            let mut budget = lane_defaults::budget_for(Lane::Interactive);
            budget.ui_nodes = 100;
            let actor = ActorId::new(1, 0, 0, 0).await;
            let err = store.apply_patch(actor, vec![1], 150, &budget).await.unwrap_err();
            assert_eq!(err, FailureSignal::UiQuota);
            let snapshot = store.commit_frame(0).await;
            assert_eq!(snapshot.node_count, 100, "node count must be truncated to the budget ceiling, never exceed it");
        }

        #[semio_framework_async_macros::async_test]
        async fn scene_max_patch_bytes_rejects_oversized_patch() {
            let mut store = SceneStore::new();
            let mut budget = lane_defaults::budget_for(Lane::Interactive);
            budget.max_patch_bytes = 4;
            let actor = ActorId::new(1, 0, 0, 0).await;
            let err = store.apply_patch(actor, vec![0; 100], 1, &budget).await.unwrap_err();
            assert_eq!(err, FailureSignal::UiQuota);
        }
        //#endregion 🔖️Scene

        //#region 🔖️ThreadTransport
        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn thread_transport_duplex_send_recv_and_heartbeat() {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            kernel_side.send(b"to-shard").await;
            assert_eq!(shard_side.recv().await, Some(b"to-shard".to_vec()));
            shard_side.send(b"to-kernel").await;
            assert_eq!(kernel_side.recv().await, Some(b"to-kernel".to_vec()));
            shard_side.beat(42).await;
            assert_eq!(kernel_side.heartbeat().await, 42);
        }

        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn thread_transport_kill_stops_recv() {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            kernel_side.send(b"queued-before-kill").await;
            kernel_side.kill().await;
            assert_eq!(shard_side.recv().await, None, "a killed transport must never yield a stale message");
        }

        /// 🎯️ terra-shard-grants requirement: `recv_deadline` must return `None` on a genuine
        /// timeout (nothing ever sent) rather than blocking forever — proven with a short, bounded
        /// deadline so the test itself cannot hang. "Spawns no thread" is a property of the
        /// IMPLEMENTATION (`mpsc::Receiver::recv_timeout`, which blocks only the calling thread —
        /// see [`ThreadTransport::recv_deadline`]'s own doc), verified by code review plus the
        /// crate-wide purity grep this ticket already runs (`std::thread` must match only the
        /// header doc comment across the whole file) — deliberately NOT re-proven here via
        /// `std::thread::current()`, which would itself add a real (non-doc-comment) `std::thread`
        /// use to this file and defeat the very grep this test is meant to keep passing.
        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn recv_deadline_returns_none_on_timeout() {
            let (_kernel_side, shard_side) = ThreadTransport::new_pair().await;
            let result = shard_side.recv_deadline(std::time::Duration::from_millis(20)).await;
            assert_eq!(result, None, "nothing was ever sent — a timeout must yield None, not block forever");
        }

        /// 🎯️ The complementary case: a message sent before the deadline must still be delivered —
        /// `recv_deadline` is a bounded wait, not merely a disguised `recv()` that always returns
        /// `None` until timeout.
        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn recv_deadline_returns_the_message_when_one_arrives_before_the_timeout() {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            kernel_side.send(b"before-deadline").await;
            assert_eq!(shard_side.recv_deadline(std::time::Duration::from_millis(200)).await, Some(b"before-deadline".to_vec()));
        }

        /// 🛑️ Mirrors `thread_transport_kill_stops_recv` for the blocking variant — a killed
        /// transport must return `None` immediately, never wait out the full deadline.
        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn recv_deadline_returns_none_immediately_on_a_killed_transport() {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            kernel_side.send(b"queued-before-kill").await;
            kernel_side.kill().await;
            assert_eq!(shard_side.recv_deadline(std::time::Duration::from_millis(20)).await, None, "a killed transport must never yield a stale message");
        }
        //#endregion 🔖️ThreadTransport

        //#region 🔖️KernelFacade
        #[semio_framework_async_macros::async_test]
        async fn kernel_activate_submit_tick_complete_round_trip() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
            let window = WindowId(1);
            let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, Some(window), ActivationEvent::WindowOpen { window }).await;
            let bp = kernel.submit(&env(actor, Lane::Interactive, 1).await).await;
            assert_eq!(bp, Backpressure::Accept);
            let decision = kernel.tick(0).await;
            assert_eq!(decision.run.len(), 1);
            assert_eq!(decision.run[0].actor, actor);
            let escalation = kernel.complete(actor, &ok_turn().await, 1).await.unwrap();
            assert_eq!(escalation, FailureEscalation::None);
            assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Active));
        }

        #[semio_framework_async_macros::async_test]
        async fn kernel_suspend_resume_round_trip() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
            let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::Extension { plugin: PackageId("s.cad".into()), extension_id: "e1".into() }, Lane::Background, None, ActivationEvent::Manual).await;
            kernel.suspend(actor, Some(vec![1, 2, 3])).await.unwrap();
            assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Suspended { checkpoint: Some(vec![1, 2, 3]) }));
            kernel.resume(actor).await.unwrap();
            assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Active));
        }

        #[semio_framework_async_macros::async_test]
        async fn kernel_request_exclusive_then_release() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 1, 4).await;
            let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::Job { owner: ActorId::new(0, 0, 0, 0).await, job_id: 1 }, Lane::Background, None, ActivationEvent::Manual).await;
            let shard = kernel.request_exclusive(actor).await.unwrap();
            assert!(shard.0 >= 3, "exclusive shards must come from the reserved tail of the pool");
            kernel.release_exclusive(actor).await;
        }

        #[semio_framework_async_macros::async_test]
        async fn kernel_metrics_counts_actors_shards_packages() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
            kernel.activate(PackageId("s.a".into()), 1, ActorKind::Extension { plugin: PackageId("s.a".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;
            kernel.activate(PackageId("s.b".into()), 2, ActorKind::Extension { plugin: PackageId("s.b".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;
            let metrics = kernel.metrics().await;
            assert_eq!(metrics.actors, 2);
            assert_eq!(metrics.packages, 2);
            assert_eq!(metrics.shards, 4);
        }
        //#endregion 🔖️KernelFacade

        //#region 🔖️ExtensionActivation
        /// 📌️ terra-extension-activation: `activate_pinned` must place the extension actor on
        /// EXACTLY the parent's shard, not wherever the least-loaded heuristic would otherwise choose
        /// — the property `MessageEndpoint::Extension` traffic depends on to never cross a transport.
        #[semio_framework_async_macros::async_test]
        async fn activate_pinned_places_extension_on_parents_shard() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
            let plugin = PackageId("s.cad".into());
            let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let parent_shard = kernel.shard_of(parent).await.expect("parent must be pinned by activate");

            let ext_pkg = PackageId("s.cad.aec-extension".into());
            let extension = kernel.activate_pinned(ext_pkg, 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "aec".into() }, Lane::Background, None, ActivationEvent::Manual, parent_shard, Some(parent), vec![]).await;

            assert_eq!(kernel.shard_of(extension).await, Some(parent_shard), "extension must land on the parent's exact shard, never wherever least-loaded would pick");
        }

        /// ✂️ terra-extension-activation: deactivating a parent must remove BOTH extensions and the
        /// parent itself, leaves-first, with zero orphans left in `kernel.metrics().actors`.
        #[semio_framework_async_macros::async_test]
        async fn deactivate_parent_cascades_leaves_first_with_zero_orphans() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
            let plugin = PackageId("s.cad".into());
            let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let shard = kernel.shard_of(parent).await.unwrap();
            let e1 = kernel.activate_pinned(PackageId("s.cad.e1".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "e1".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
            let e2 = kernel.activate_pinned(PackageId("s.cad.e2".into()), 3, ActorKind::Extension { plugin: plugin.clone(), extension_id: "e2".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
            kernel.link_extension(parent, e1).await.unwrap();
            kernel.link_extension(parent, e2).await.unwrap();
            assert_eq!(kernel.metrics().await.actors, 3);

            let removed = kernel.deactivate(parent).await.unwrap();
            assert_eq!(removed.len(), 3, "parent + 2 extensions");
            assert_eq!(*removed.last().unwrap(), parent, "leaves-first: parent removed LAST");
            assert!(removed[..2].contains(&e1) && removed[..2].contains(&e2), "both extensions removed before the parent");
            assert_eq!(kernel.metrics().await.actors, 0, "zero orphans after cascade");
            assert!(kernel.actor_record(parent).await.is_none());
            assert!(kernel.actor_record(e1).await.is_none());
            assert!(kernel.actor_record(e2).await.is_none());
            assert!(kernel.children_of(parent).await.is_empty(), "link table must not resurrect a removed parent's edge");
        }

        /// 🔪️ terra-extension-activation: `kill` on the parent must cascade identically to `deactivate`
        /// — "a parent kill takes its extensions down" (design doc M6's own acceptance wording).
        #[semio_framework_async_macros::async_test]
        async fn kill_parent_takes_extensions_down() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
            let plugin = PackageId("s.flow".into());
            let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "flow".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let shard = kernel.shard_of(parent).await.unwrap();
            let e1 = kernel.activate_pinned(PackageId("s.flow.e1".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "e1".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
            kernel.link_extension(parent, e1).await.unwrap();

            let removed = kernel.kill(parent).await.unwrap();
            assert_eq!(removed, vec![e1, parent], "leaves-first order: extension then parent");
            assert_eq!(kernel.metrics().await.actors, 0);
        }

        /// 🚑️ terra-extension-activation: a single trap on an EXTENSION must restore/kill only that
        /// extension — the parent's own status is untouched. Also proves package isolation: giving
        /// each extension its OWN `PackageId` (the design choice the native cascade this test mirrors
        /// makes — see `📓️terra-extension-activation-report.md`) means even reaching the QUARANTINE
        /// threshold on the extension does not blast the parent's package, unlike two actors
        /// deliberately sharing one `PackageId` (`failure_ladder_trap_then_quarantine_is_package_wide`,
        /// which this test deliberately does not reproduce for the parent/extension pair).
        #[semio_framework_async_macros::async_test]
        async fn trapping_extension_never_faults_the_parent() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
            let plugin = PackageId("s.cad".into());
            let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            kernel.complete(parent, &ok_turn().await, 0).await.unwrap();
            assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Active));

            let shard = kernel.shard_of(parent).await.unwrap();
            let extension = kernel.activate_pinned(PackageId("s.cad.aec".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "aec".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
            kernel.link_extension(parent, extension).await.unwrap();

            let faulted = TurnResult { ui_patches: vec![], effects: vec![], command_ingress: vec![], lifecycle_receipt: None, ui_patch_receipt: None, next_wake: None, status: TurnStatus::Faulted { detail: b"boom".to_vec() }, usage: Usage::default() };
            let escalation = kernel.complete(extension, &faulted, 10).await.unwrap();
            assert_eq!(escalation, FailureEscalation::Restart, "one trap must only Restart, never quarantine");
            assert_eq!(kernel.actor_status(extension).await, Some(&ActorStatus::Trapped));
            assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Active), "the parent must be completely untouched by its extension's trap");

            // Push the SAME extension past the quarantine threshold — still must not reach the parent,
            // because this test gave the extension its own PackageId (distinct from the parent's).
            for i in 1..FAILURE_QUARANTINE_RESTART_THRESHOLD {
                let faulted = TurnResult { ui_patches: vec![], effects: vec![], command_ingress: vec![], lifecycle_receipt: None, ui_patch_receipt: None, next_wake: None, status: TurnStatus::Faulted { detail: b"boom".to_vec() }, usage: Usage::default() };
                kernel.complete(extension, &faulted, 10 + i as u64).await.unwrap();
            }
            assert_eq!(kernel.actor_status(extension).await, Some(&ActorStatus::Quarantined), "the extension itself does escalate to quarantine");
            assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Active), "package isolation: the parent must still be untouched, even at quarantine");
        }

        /// 🔐️ terra-extension-activation: the security property — a capability the parent was never
        /// granted must be ABSENT from the extension's grants, not silently escalated. Observable via
        /// `actor_record(extension).capabilities`, i.e. a broker denial, never a `KernelError`.
        #[semio_framework_async_macros::async_test]
        async fn extension_capability_grant_is_the_intersection_not_the_request() {
            let mut kernel = Kernel::new(ShardKind::Native, 2, 0, 4).await;
            let plugin = PackageId("s.cad".into());
            let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            kernel.set_capabilities(parent, vec![CapabilityGrant { capability: "fs.read".into(), scope: None }, CapabilityGrant { capability: "net.fetch".into(), scope: None }]).await.unwrap();

            let shard = kernel.shard_of(parent).await.unwrap();
            let requested = vec![CapabilityGrant { capability: "fs.read".into(), scope: None }, CapabilityGrant { capability: "fs.admin".into(), scope: None }];
            let extension = kernel.activate_pinned(PackageId("s.cad.aec".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "aec".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), requested).await;

            let record = kernel.actor_record(extension).await.expect("extension must be live");
            assert_eq!(record.capabilities.len(), 1, "only the grant the parent ALSO held may survive");
            assert_eq!(record.capabilities[0].capability, "fs.read");
            assert!(!record.capabilities.iter().any(|g| g.capability == "fs.admin"), "the parent never held fs.admin — it must be absent, not escalated");
        }

        /// 💤️▶️ terra-extension-activation: suspend cascades leaves-first (checkpoint bytes only on
        /// the root), resume cascades parent-first (the symmetric restore direction).
        #[semio_framework_async_macros::async_test]
        async fn suspend_cascade_leaves_first_resume_cascade_parent_first() {
            let mut kernel = Kernel::new(ShardKind::Native, 4, 0, 4).await;
            let plugin = PackageId("s.cad".into());
            let parent = kernel.activate(plugin.clone(), 1, ActorKind::PluginApp { plugin: plugin.clone(), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let shard = kernel.shard_of(parent).await.unwrap();
            let extension = kernel.activate_pinned(PackageId("s.cad.aec".into()), 2, ActorKind::Extension { plugin: plugin.clone(), extension_id: "aec".into() }, Lane::Background, None, ActivationEvent::Manual, shard, Some(parent), vec![]).await;
            kernel.link_extension(parent, extension).await.unwrap();

            let order = kernel.suspend_cascade(parent, Some(vec![9, 9])).await.unwrap();
            assert_eq!(order, vec![extension, parent], "leaves-first: extension suspended before parent");
            assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Suspended { checkpoint: Some(vec![9, 9]) }));
            assert_eq!(kernel.actor_status(extension).await, Some(&ActorStatus::Suspended { checkpoint: None }), "descendants carry no checkpoint bytes of their own");

            let resumed = kernel.resume_cascade(parent).await.unwrap();
            assert_eq!(resumed, vec![parent, extension], "parent-first: the symmetric restore direction");
            assert_eq!(kernel.actor_status(parent).await, Some(&ActorStatus::Active));
            assert_eq!(kernel.actor_status(extension).await, Some(&ActorStatus::Active));
        }
        //#endregion 🔖️ExtensionActivation

        //#region 🔖️RuntimeMetricsSnapshot
        /// 📈️ T1 runtime evidence: drives two real actors (different packages/lanes) through
        /// `activate`/`submit`/`tick`/`complete`, then asserts `runtime_metrics_snapshot`'s rows —
        /// package, lane, status, turns, shard — match what the kernel actually did, not a fake.
        #[semio_framework_async_macros::async_test]
        async fn runtime_metrics_snapshot_reflects_real_kernel_activity() {
            let mut kernel = Kernel::new(ShardKind::Native, 2, 0, 8).await;
            let cad = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let stdio = kernel.activate(PackageId("s.stdio".into()), 2, ActorKind::Extension { plugin: PackageId("s.stdio".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;

            kernel.submit(&env(cad, Lane::Interactive, 1).await).await;
            let decision = kernel.tick(0).await;
            assert_eq!(decision.run.len(), 1, "only `cad` has a pending envelope this tick");
            kernel.complete(cad, &ok_turn().await, 5).await.unwrap();

            let snapshot = kernel.runtime_metrics_snapshot(5).await;
            assert_eq!(snapshot.sampled_at_ms, 5);
            assert_eq!(snapshot.kernel.actors, 2);
            assert_eq!(snapshot.kernel.packages, 2);
            assert_eq!(snapshot.actors.len(), 2);

            let cad_row = snapshot.actors.iter().find(|row| row.id == cad).expect("cad row present");
            assert_eq!(cad_row.package, PackageId("s.cad".into()));
            assert_eq!(cad_row.lane, Lane::Interactive);
            assert_eq!(cad_row.status, ActorStatus::Active);
            assert_eq!(cad_row.metrics.turns, 1, "the completed turn must be counted");

            let stdio_row = snapshot.actors.iter().find(|row| row.id == stdio).expect("stdio row present");
            assert_eq!(stdio_row.package, PackageId("s.stdio".into()));
            assert_eq!(stdio_row.lane, Lane::Background);
            assert_eq!(stdio_row.metrics.turns, 0, "stdio never got a turn");

            assert!(!snapshot.shards.is_empty(), "at least one shard row for the two pinned actors");
            let total_shard_actors: u32 = snapshot.shards.iter().map(|row| row.metrics.actors).sum();
            assert_eq!(total_shard_actors, 2, "every actor is counted on exactly one shard");
        }

        #[semio_framework_async_macros::async_test]
        async fn runtime_metrics_due_gates_at_the_2hz_interval_and_always_fires_once() {
            assert!(runtime_metrics_due(None, 0).await, "never published yet must always be due");
            assert!(!runtime_metrics_due(Some(1_000), 1_200).await, "200ms since last publish is inside the 500ms window");
            assert!(runtime_metrics_due(Some(1_000), 1_500).await, "exactly the 500ms interval must fire");
            assert!(runtime_metrics_due(Some(1_000), 2_000).await, "well past the interval must fire");
        }
        //#endregion 🔖️RuntimeMetricsSnapshot

        //#region 🔖️ShardTable
        #[semio_framework_async_macros::async_test]
        async fn shard_sizing_policy_clamps_native_and_web() {
            assert_eq!(clamp_native_shard_count(1).await, 2);
            assert_eq!(clamp_native_shard_count(9).await, 8);
            assert_eq!(clamp_native_shard_count(5).await, 4);
            assert_eq!(clamp_web_shard_count(1).await, 1);
            assert_eq!(clamp_web_shard_count(9).await, 4);
        }
        //#endregion 🔖️ShardTable

        //#region 🔖️JobProgressOverlay
        fn progress_now_us() -> Option<u64> {
            Some(0)
        }

        fn with_progress_context<T>(operation: u64, generation: u64, cancel: job::CancelToken, run: impl FnOnce(&mut job::StepContext<'_>) -> T) -> T {
            let mut preview_sequence = 0;
            let mut context = job::StepContext::new(job::OperationId(operation), job::Generation(generation), job::StepBudget::new(8, 10), cancel, progress_now_us, &mut preview_sequence);
            run(&mut context)
        }

        fn progress_publication(job: u64, operation: u64, base_revision: u64, generation: u64, step_sequence: u64, preview_sequence: u64, outcome: JobStepOutcome) -> JobPublication {
            JobPublication { turn: JobTurn { job, operation: JobOperation { operation, base_revision, generation, preview_sequence, seed: 13 }, step_sequence }, outcome }
        }

        fn admit_progress(store: &mut JobProgressOverlayStore, actor: ActorId, publication: JobPublication) -> JobProgressReceipt {
            let live = store.live_authority(actor, publication.turn.job).expect("job operation was admitted before its publication");
            with_progress_context(publication.turn.operation.operation, publication.turn.operation.generation, job::root_cancel_token(), |context| {
                let admission = store.preflight(context, actor, &publication, live).expect("publication preflight");
                store.publish_reserved(context, admission, publication, live).expect("reserved publication")
            })
        }

        fn drive_progress_close(store: &mut JobProgressOverlayStore) {
            for _ in 0..(JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS + JOB_PROGRESS_ACTIVE_CAPACITY * 3 + 8) {
                if store.terminal_is_empty() {
                    return;
                }
                let step = with_progress_context(0, 0, job::root_cancel_token(), |context| store.close_step(context));
                assert!(matches!(step, JobProgressCloseStep::Pending { released_items: 0 | 1, .. } | JobProgressCloseStep::Complete));
            }
            panic!("job progress overlay did not reach its terminal witness");
        }

        #[test]
        fn job_progress_preview_is_distinct_owned_and_checked_out_drop_hands_back_exactly() {
            let actor = ActorId(7);
            let live = JobProgressLiveAuthority::new(11, 17, 23);
            let mut store = JobProgressOverlayStore::new();
            store.begin_operation(actor, 29, live).unwrap();
            let preview = Vec::from([1, 2, 3, 4]);
            let pointer = preview.as_ptr();
            let receipt = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 0, 1, JobStepOutcome::PreviewReady { preview }));
            assert_eq!(receipt.kind(), JobProgressKind::Preview);
            store.acknowledge(receipt).unwrap();
            {
                let checkout = store.take(actor, 29).unwrap();
                assert_eq!(checkout.preview().as_ptr(), pointer);
                assert_eq!(checkout.preview(), &[1, 2, 3, 4]);
            }
            assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "Drop must return the exact preview owner to its fixed slot");
            let replacement = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 1, 2, JobStepOutcome::PreviewReady { preview: vec![8, 9] }));
            store.abort(replacement).unwrap();
            assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "abort must restore the last valid preview owner rather than exposing the staged replacement");
            let checkpoint = admit_progress(&mut store, actor, progress_publication(29, 11, 17, 23, 1, 1, JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: vec![10], applied_progress: 1 } }));
            store.abort(checkpoint).unwrap();
            assert_eq!(store.take(actor, 29).unwrap().preview().as_ptr(), pointer, "aborting scalar progress must retain the prior preview without an unnecessary owner move");
            store.begin_close_actor(actor).unwrap();
            drive_progress_close(&mut store);
        }

        #[test]
        fn job_progress_fixed_capacity_and_aba_admission_fail_closed() {
            let mut store = JobProgressOverlayStore::new();
            for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
                store.begin_operation(ActorId(ordinal as u64 + 1), 5, JobProgressLiveAuthority::new(ordinal as u64 + 10, 0, 1)).unwrap();
            }
            assert_eq!(store.begin_operation(ActorId(1000), 5, JobProgressLiveAuthority::new(1000, 0, 1)), Err(JobProgressFault::Capacity));
            assert_eq!(store.begin_operation(ActorId(1), 5, JobProgressLiveAuthority::new(10, 0, 1)), Err(JobProgressFault::Busy));
            store.begin_close_all();
            drive_progress_close(&mut store);
            store.begin_operation(ActorId(1), 5, JobProgressLiveAuthority::new(99, 0, 2)).unwrap();
            assert_eq!(store.live_authority(ActorId(1), 5), Some(JobProgressLiveAuthority::new(99, 0, 2)));
            store.begin_close_all();
            drive_progress_close(&mut store);
        }

        #[test]
        fn job_progress_mounted_aggregate_item_and_byte_caps_reject_plus_one_exactly() {
            let mut item_store = JobProgressOverlayStore::new();
            for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
                let actor = ActorId(ordinal as u64 + 1);
                let live = JobProgressLiveAuthority::new(ordinal as u64 + 10, 0, 1);
                item_store.begin_operation(actor, 5, live).unwrap();
                let receipt = admit_progress(&mut item_store, actor, progress_publication(5, live.operation, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: Vec::new() }));
                item_store.acknowledge(receipt).unwrap();
            }
            for ordinal in 0..JOB_PROGRESS_RETIREMENT_CAPACITY {
                item_store
                    .retain_rejected(JobProgressRejected::new(
                        JobProgressFault::Stale,
                        progress_publication(1_000 + ordinal as u64, 2_000 + ordinal as u64, 0, 1, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: Vec::new(), output: Vec::new() } }),
                    ))
                    .unwrap();
            }
            assert_eq!(item_store.owned_items, JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS);
            let rejected = JobProgressRejected::new(JobProgressFault::Stale, progress_publication(9_999, 8_888, 0, 1, 0, 0, JobStepOutcome::Yield));
            let returned = item_store.retain_rejected(rejected).unwrap_err();
            assert_eq!(returned.publication().turn.job, 9_999);
            item_store.begin_close_all();
            drive_progress_close(&mut item_store);

            let mut byte_store = JobProgressOverlayStore::new();
            for ordinal in 0..JOB_PROGRESS_ACTIVE_CAPACITY {
                let actor = ActorId(ordinal as u64 + 101);
                let live = JobProgressLiveAuthority::new(ordinal as u64 + 301, 0, 1);
                byte_store.begin_operation(actor, 7, live).unwrap();
                let receipt = admit_progress(&mut byte_store, actor, progress_publication(7, live.operation, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: vec![3; JOB_PROGRESS_PAGE_MAXIMUM_BYTES] }));
                byte_store.acknowledge(receipt).unwrap();
            }
            for ordinal in 0..96 {
                byte_store
                    .retain_rejected(JobProgressRejected::new(
                        JobProgressFault::Stale,
                        progress_publication(
                            3_000 + ordinal,
                            4_000 + ordinal,
                            0,
                            1,
                            0,
                            0,
                            JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![4; JOB_PROGRESS_PAGE_MAXIMUM_BYTES], output: vec![5; JOB_PROGRESS_PAGE_MAXIMUM_BYTES] } },
                        ),
                    ))
                    .unwrap();
            }
            assert_eq!(byte_store.owned_bytes, JOB_PROGRESS_TOTAL_MAXIMUM_BYTES);
            let plus_one = vec![9];
            let pointer = plus_one.as_ptr();
            let returned = byte_store.retain_rejected(JobProgressRejected::new(JobProgressFault::Stale, progress_publication(7_777, 8_888, 0, 1, 0, 1, JobStepOutcome::PreviewReady { preview: plus_one }))).unwrap_err();
            assert!(matches!(&returned.publication().outcome, JobStepOutcome::PreviewReady { preview } if preview.as_ptr() == pointer));
            byte_store.begin_close_all();
            drive_progress_close(&mut byte_store);
        }

        #[test]
        fn job_progress_page_boundary_plus_one_stale_order_and_cancel_preserve_owner() {
            let actor = ActorId(31);
            let live = JobProgressLiveAuthority::new(37, 41, 43);
            let mut store = JobProgressOverlayStore::new();
            store.begin_operation(actor, 47, live).unwrap();
            let exact = vec![5; JOB_PROGRESS_PAGE_MAXIMUM_BYTES];
            let exact_pointer = exact.as_ptr();
            let exact_publication = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: exact });
            let admission = with_progress_context(37, 43, job::root_cancel_token(), |context| store.preflight(context, actor, &exact_publication, live)).unwrap();
            assert_eq!(
                match &exact_publication.outcome {
                    JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                    _ => unreachable!(),
                },
                exact_pointer
            );
            store.cancel_admission(admission).unwrap();

            let oversized = vec![7; JOB_PROGRESS_PAGE_MAXIMUM_BYTES + 1];
            let oversized_pointer = oversized.as_ptr();
            let oversized_publication = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: oversized });
            let oversized_fault = with_progress_context(37, 43, job::root_cancel_token(), |context| store.preflight(context, actor, &oversized_publication, live)).unwrap_err();
            assert_eq!(oversized_fault, JobProgressFault::Oversized);
            assert_eq!(
                match &oversized_publication.outcome {
                    JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                    _ => unreachable!(),
                },
                oversized_pointer
            );

            let stale = progress_publication(47, 37, 42, 43, 0, 1, JobStepOutcome::PreviewReady { preview: vec![1] });
            assert_eq!(with_progress_context(37, 43, job::root_cancel_token(), |context| store.preflight(context, actor, &stale, live)).unwrap_err(), JobProgressFault::Stale);
            let out_of_order = progress_publication(47, 37, 41, 43, 1, 1, JobStepOutcome::Yield);
            assert_eq!(with_progress_context(37, 43, job::root_cancel_token(), |context| store.preflight(context, actor, &out_of_order, live)).unwrap_err(), JobProgressFault::StepSequence);

            let cancelled = progress_publication(47, 37, 41, 43, 0, 1, JobStepOutcome::PreviewReady { preview: vec![9] });
            let cancelled_pointer = match &cancelled.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                _ => unreachable!(),
            };
            let cancel = job::root_cancel_token();
            cancel.cancel_now();
            assert_eq!(with_progress_context(37, 43, cancel, |context| store.preflight(context, actor, &cancelled, live)).unwrap_err(), JobProgressFault::Cancelled);
            assert_eq!(
                match &cancelled.outcome {
                    JobStepOutcome::PreviewReady { preview } => preview.as_ptr(),
                    _ => unreachable!(),
                },
                cancelled_pointer
            );
            store.begin_close_actor(actor).unwrap();
            drive_progress_close(&mut store);
        }

        #[test]
        fn job_progress_commit_validates_live_authority_and_rejected_close_is_incremental() {
            let actor = ActorId(53);
            let live = JobProgressLiveAuthority::new(59, 61, 67);
            let mut store = JobProgressOverlayStore::new();
            store.begin_operation(actor, 71, live).unwrap();
            let stale = progress_publication(71, 59, 61, 68, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![1], output: vec![2] } });
            assert_eq!(with_progress_context(59, 68, job::root_cancel_token(), |context| store.preflight(context, actor, &stale, JobProgressLiveAuthority::new(59, 61, 68))).unwrap_err(), JobProgressFault::Stale);

            let complete = progress_publication(71, 59, 61, 67, 0, 0, JobStepOutcome::Complete { candidate: JobCommitCandidate { state: vec![3], output: vec![4] } });
            let receipt = admit_progress(&mut store, actor, complete);
            assert_eq!(receipt.kind(), JobProgressKind::CommitValidated);
            store.acknowledge(receipt).unwrap();
            assert!(store.has_close_work(), "committed candidates retire outside the preview overlay");
            drive_progress_close(&mut store);

            let mut rejected_store = JobProgressOverlayStore::new();
            let detail = Vec::from([8, 9, 10]);
            let pointer = detail.as_ptr();
            let rejected = JobProgressRejected::new(JobProgressFault::Stale, progress_publication(73, 79, 83, 89, 0, 0, JobStepOutcome::Fault { detail }));
            assert_eq!(
                match &rejected.publication().outcome {
                    JobStepOutcome::Fault { detail } => detail.as_ptr(),
                    _ => unreachable!(),
                },
                pointer
            );
            rejected_store.retain_rejected(rejected).unwrap();
            let first = with_progress_context(0, 0, job::root_cancel_token(), |context| rejected_store.close_step(context));
            assert!(matches!(first, JobProgressCloseStep::Pending { released_items: 1, released_bytes } if released_bytes >= 3));
            assert!(!rejected_store.terminal_is_empty(), "publication shell retires on a distinct grant");
            drive_progress_close(&mut rejected_store);
        }

        #[test]
        fn job_progress_replay_is_deterministic_for_identical_publications() {
            let actor = ActorId(97);
            let live = JobProgressLiveAuthority::new(101, 103, 107);
            let mut left = JobProgressOverlayStore::new();
            let mut right = JobProgressOverlayStore::new();
            left.begin_operation(actor, 109, live).unwrap();
            right.begin_operation(actor, 109, live).unwrap();
            for (step, preview_sequence, byte) in [(0, 1, 11), (1, 2, 13)] {
                let left_receipt = admit_progress(&mut left, actor, progress_publication(109, 101, 103, 107, step, preview_sequence, JobStepOutcome::PreviewReady { preview: vec![byte] }));
                let right_receipt = admit_progress(&mut right, actor, progress_publication(109, 101, 103, 107, step, preview_sequence, JobStepOutcome::PreviewReady { preview: vec![byte] }));
                assert_eq!(left_receipt.identity(), right_receipt.identity());
                assert_eq!(left_receipt.kind(), right_receipt.kind());
                left.acknowledge(left_receipt).unwrap();
                right.acknowledge(right_receipt).unwrap();
            }
            assert_eq!(left.take(actor, 109).unwrap().preview(), right.take(actor, 109).unwrap().preview());
            left.begin_close_actor(actor).unwrap();
            right.begin_close_actor(actor).unwrap();
            drive_progress_close(&mut left);
            drive_progress_close(&mut right);
        }
        //#endregion 🔖️JobProgressOverlay
    }

    //#region 🔖️Typegen
    #[cfg(feature = "typegen")]
    #[semio_framework_async_macros::async_test]
    async fn exports_typescript_bindings() {
        crate::schema_metadata::validate().unwrap();
        let rendered = crate::schema_metadata::render_typescript();
        if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
            std::fs::write(path, &rendered).unwrap();
        } else {
            assert_eq!(rendered, include_str!("🤖️generated/🟦️actor.ts"));
        }
    }
    //#endregion 🔖️Typegen
}
