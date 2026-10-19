//! 🚑️ Capability and failure policy: the capability grant with its intersection and the failure signals, stages, state and escalation with the backoff and quarantine durations. See [`super`].

use super::*;

//#region 🔐️CapabilityGrant
/// 🔐️ Minimal local stand-in for `kernel::CapabilityGrant` — see the module-level seam docstring.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct CapabilityGrant {
    pub capability: String,
    pub scope: Option<Vec<u8>>,
}

impl CapabilityGrant {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_str(out, &self.capability).await;
        semio_framework_actor_pack::write_opt_bytes(out, &self.scope).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self { capability: semio_framework_actor_pack::read_str(bytes, pos, "CapabilityGrant::capability").await?, scope: semio_framework_actor_pack::read_opt_bytes(bytes, pos, "CapabilityGrant::scope").await? })
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
                semio_framework_actor_pack::write_u8(out, 0).await;
                semio_framework_actor_pack::write_f32(out, *ratio).await;
            }
            FailureSignal::FuelExhausted => semio_framework_actor_pack::write_u8(out, 1).await,
            FailureSignal::MemoryLimit => semio_framework_actor_pack::write_u8(out, 2).await,
            FailureSignal::MailboxOverflow => semio_framework_actor_pack::write_u8(out, 3).await,
            FailureSignal::UiQuota => semio_framework_actor_pack::write_u8(out, 4).await,
            FailureSignal::Trap { detail } => {
                semio_framework_actor_pack::write_u8(out, 5).await;
                semio_framework_actor_pack::write_str(out, detail).await;
            }
            FailureSignal::HeartbeatMissed { count } => {
                semio_framework_actor_pack::write_u8(out, 6).await;
                semio_framework_actor_pack::write_u32(out, *count).await;
            }
            FailureSignal::ManualReset => semio_framework_actor_pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "FailureSignal").await?;
        match tag {
            0 => Ok(FailureSignal::DeadlineOverrun { ratio: semio_framework_actor_pack::read_f32(bytes, pos, "FailureSignal::ratio").await? }),
            1 => Ok(FailureSignal::FuelExhausted),
            2 => Ok(FailureSignal::MemoryLimit),
            3 => Ok(FailureSignal::MailboxOverflow),
            4 => Ok(FailureSignal::UiQuota),
            5 => Ok(FailureSignal::Trap { detail: semio_framework_actor_pack::read_str(bytes, pos, "FailureSignal::Trap").await? }),
            6 => Ok(FailureSignal::HeartbeatMissed { count: semio_framework_actor_pack::read_u32(bytes, pos, "FailureSignal::count").await? }),
            7 => Ok(FailureSignal::ManualReset),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "FailureSignal", tag: other, offset: *pos }),
        }
    }
}

/// 🪜️ Rungs of the failure ladder, worst-consequence order.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
            FailureStage::Healthy => semio_framework_actor_pack::write_u8(out, 0).await,
            FailureStage::Warned => semio_framework_actor_pack::write_u8(out, 1).await,
            FailureStage::Throttled { factor } => {
                semio_framework_actor_pack::write_u8(out, 2).await;
                semio_framework_actor_pack::write_f32(out, *factor).await;
            }
            FailureStage::Suspended { until } => {
                semio_framework_actor_pack::write_u8(out, 3).await;
                semio_framework_actor_pack::write_u64(out, *until).await;
            }
            FailureStage::Cancelled => semio_framework_actor_pack::write_u8(out, 4).await,
            FailureStage::Trapped { restarts } => {
                semio_framework_actor_pack::write_u8(out, 5).await;
                semio_framework_actor_pack::write_u32(out, *restarts).await;
            }
            FailureStage::Quarantined { until } => {
                semio_framework_actor_pack::write_u8(out, 6).await;
                semio_framework_actor_pack::write_u64(out, *until).await;
            }
            FailureStage::Disabled => semio_framework_actor_pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "FailureStage").await?;
        match tag {
            0 => Ok(FailureStage::Healthy),
            1 => Ok(FailureStage::Warned),
            2 => Ok(FailureStage::Throttled { factor: semio_framework_actor_pack::read_f32(bytes, pos, "FailureStage::factor").await? }),
            3 => Ok(FailureStage::Suspended { until: semio_framework_actor_pack::read_u64(bytes, pos, "FailureStage::until").await? }),
            4 => Ok(FailureStage::Cancelled),
            5 => Ok(FailureStage::Trapped { restarts: semio_framework_actor_pack::read_u32(bytes, pos, "FailureStage::restarts").await? }),
            6 => Ok(FailureStage::Quarantined { until: semio_framework_actor_pack::read_u64(bytes, pos, "FailureStage::until").await? }),
            7 => Ok(FailureStage::Disabled),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "FailureStage", tag: other, offset: *pos }),
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
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
        semio_framework_actor_pack::write_u32(out, self.clean_turns).await;
        semio_framework_actor_pack::write_u32(out, self.warn_count).await;
        semio_framework_actor_pack::write_u32(out, self.restart_count).await;
        semio_framework_actor_pack::write_u64(out, self.last_signal_ms).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self {
            stage: FailureStage::pack_decode(bytes, pos).await?,
            clean_turns: semio_framework_actor_pack::read_u32(bytes, pos, "FailureState::clean_turns").await?,
            warn_count: semio_framework_actor_pack::read_u32(bytes, pos, "FailureState::warn_count").await?,
            restart_count: semio_framework_actor_pack::read_u32(bytes, pos, "FailureState::restart_count").await?,
            last_signal_ms: semio_framework_actor_pack::read_u64(bytes, pos, "FailureState::last_signal_ms").await?,
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

/// 🚑️ The failure policy supplies the same quarantine duration to package-wide Kernel escalation.
pub async fn quarantine_duration_ms(restart_count: u32) -> u64 {
    5_000u64.saturating_mul(1u64 << restart_count.min(10))
}
//#endregion 🚑️FailurePolicy
