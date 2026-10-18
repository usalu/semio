//! 🧩️ Actor status and shards: the actor lifecycle status, the shard kind and identity and the shard count clamps. See [`super`].

use super::*;

//#region 🗂️ActorRecord
/// 🗂️ Actor lifecycle state, driven by [`Kernel::activate`]/`suspend`/`resume` and the failure ladder.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
            ActorStatus::Cold => semio_framework_actor_pack::write_u8(out, 0).await,
            ActorStatus::Activating => semio_framework_actor_pack::write_u8(out, 1).await,
            ActorStatus::Active => semio_framework_actor_pack::write_u8(out, 2).await,
            ActorStatus::Suspended { checkpoint } => {
                semio_framework_actor_pack::write_u8(out, 3).await;
                semio_framework_actor_pack::write_opt_bytes(out, checkpoint).await;
            }
            ActorStatus::Draining => semio_framework_actor_pack::write_u8(out, 4).await,
            ActorStatus::Trapped => semio_framework_actor_pack::write_u8(out, 5).await,
            ActorStatus::Quarantined => semio_framework_actor_pack::write_u8(out, 6).await,
            ActorStatus::Disabled => semio_framework_actor_pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "ActorStatus").await?;
        match tag {
            0 => Ok(ActorStatus::Cold),
            1 => Ok(ActorStatus::Activating),
            2 => Ok(ActorStatus::Active),
            3 => Ok(ActorStatus::Suspended { checkpoint: semio_framework_actor_pack::read_opt_bytes(bytes, pos, "ActorStatus::Suspended").await? }),
            4 => Ok(ActorStatus::Draining),
            5 => Ok(ActorStatus::Trapped),
            6 => Ok(ActorStatus::Quarantined),
            7 => Ok(ActorStatus::Disabled),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "ActorStatus", tag: other, offset: *pos }),
        }
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
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
    pub(super) async fn tag(self) -> u8 {
        match self {
            ShardKind::Native => 0,
            ShardKind::WebWorker => 1,
            ShardKind::Process => 2,
        }
    }
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u8(out, self.tag().await).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        match semio_framework_actor_pack::read_u8(bytes, pos, "ShardKind").await? {
            0 => Ok(ShardKind::Native),
            1 => Ok(ShardKind::WebWorker),
            2 => Ok(ShardKind::Process),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "ShardKind", tag: other, offset: *pos }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct ShardId(pub u16);

impl ShardId {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u16(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self(semio_framework_actor_pack::read_u16(bytes, pos, "ShardId").await?))
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
//#endregion 🧩️ShardTable
