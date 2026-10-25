//! 📈️ Runtime metrics: the actor, shard and kernel metrics with their samples, the runtime metrics snapshot and the publish interval check. See [`super`].

use super::*;

//#region 📈️Metrics
/// 🍩️ Fixed-size ring of the last 64 per-turn wall-clock samples, for a p95 estimate cheap enough
/// to keep on every actor.
/// 📈️ Ring capacity for [`ActorMetrics`]'s wall-clock sample window — chosen as a plain `usize`
/// (not a const generic array) because serde's built-in array support tops out at 32 elements.
const WALL_US_RING_CAPACITY: usize = 64;

/// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): minimum recorded turns
/// before [`ActorMetrics::is_saturating`] will answer `true` — one slow turn (a cold cache, a GC
/// pause) must never flip an actor "hot"; only a SUSTAINED pattern does.
const SATURATION_MIN_TURNS: u64 = 2;

/// 🌡️ [`ActorMetrics::is_saturating`] fires once [`ActorMetrics::wall_us_p95`] reaches this percent
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

    pub fn wall_us_p95(&self) -> u32 {
        if self.wall_us_ring_len == 0 {
            return 0;
        }
        let mut samples: Vec<u32> = self.wall_us_ring[..self.wall_us_ring_len as usize].to_vec();
        samples.sort_unstable();
        let idx = ((samples.len() as f32) * 0.95).floor() as usize;
        samples[idx.min(samples.len() - 1)]
    }

    /// 🪣️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): observed-behaviour
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

/// 📊️ Sampled by `Kernel::metrics()`; the host publishes this as bus topic `os.runtime.metrics` at 2Hz.
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

/// 🧫️ One shard's row for the `os.runtime.metrics` publication. `metrics.heartbeat_age_ms` is left at
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

/// 📸️ The exact payload [`KernelMetrics`]'s own doc comment promises: "the host publishes this as bus
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

/// ⏰️ Pure cadence gate for a host's `os.runtime.metrics` publish loop — clock injected via `now_ms`
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
