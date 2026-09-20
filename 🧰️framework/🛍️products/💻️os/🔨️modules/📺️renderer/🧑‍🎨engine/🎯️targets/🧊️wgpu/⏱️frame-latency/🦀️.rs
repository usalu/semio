use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

pub(crate) const FRAME_LATENCY_CAPACITY: usize = 256;
const FRAME_LATENCY_DOMAIN_COUNT: usize = 2;
const FRAME_LATENCY_STAGE_COUNT: usize = 27;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[repr(usize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum FrameLatencyAuthorityDomain {
    RendererFrame,
    BrowserInputBatch,
}

impl FrameLatencyAuthorityDomain {
    const ALL: [Self; FRAME_LATENCY_DOMAIN_COUNT] = [Self::RendererFrame, Self::BrowserInputBatch];

    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FrameLatencyAuthority {
    domain: FrameLatencyAuthorityDomain,
    generation: u64,
}

impl FrameLatencyAuthority {
    pub(crate) const fn renderer_frame(generation: u64) -> Self {
        Self { domain: FrameLatencyAuthorityDomain::RendererFrame, generation }
    }

    pub(crate) const fn browser_input_batch(generation: u64) -> Self {
        Self { domain: FrameLatencyAuthorityDomain::BrowserInputBatch, generation }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[repr(usize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum FrameLatencyStage {
    WireApply,
    DispatchApply,
    TransactionDrainProjectionDeltas,
    TransactionRouteIntents,
    TransactionFlushEffects,
    TransactionPresentSurface,
    ShellRefresh,
    RetainedExchange,
    TransactionReconcileTree,
    TransactionBuildRenderPackets,
    TransactionPublishSnapshot,
    PresenterBeginGpu,
    PresenterOwnership,
    PresenterEngine,
    PresenterUploads,
    PresenterStage,
    QueueSubmit,
    PresenterCloseGpu,
    PresenterAcknowledge,
    PresenterProgressAcknowledge,
    PresenterFullscreen,
    PresenterDirectives,
    PresenterAborted,
    PresenterRetirement,
    SnapshotPublish,
    WorkerTick,
    WorkerReplyEncode,
}

impl FrameLatencyStage {
    pub(crate) const ALL: [Self; FRAME_LATENCY_STAGE_COUNT] = [
        Self::WireApply,
        Self::DispatchApply,
        Self::TransactionDrainProjectionDeltas,
        Self::TransactionRouteIntents,
        Self::TransactionFlushEffects,
        Self::TransactionPresentSurface,
        Self::ShellRefresh,
        Self::RetainedExchange,
        Self::TransactionReconcileTree,
        Self::TransactionBuildRenderPackets,
        Self::TransactionPublishSnapshot,
        Self::PresenterBeginGpu,
        Self::PresenterOwnership,
        Self::PresenterEngine,
        Self::PresenterUploads,
        Self::PresenterStage,
        Self::QueueSubmit,
        Self::PresenterCloseGpu,
        Self::PresenterAcknowledge,
        Self::PresenterProgressAcknowledge,
        Self::PresenterFullscreen,
        Self::PresenterDirectives,
        Self::PresenterAborted,
        Self::PresenterRetirement,
        Self::SnapshotPublish,
        Self::WorkerTick,
        Self::WorkerReplyEncode,
    ];

    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FrameLatencyObservation {
    authority: FrameLatencyAuthority,
    stage: FrameLatencyStage,
    started_us: u64,
    duration_us: u64,
    work_items: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FrameLatencyPhaseSummary {
    authority: FrameLatencyAuthority,
    stage: FrameLatencyStage,
    first_sequence: u64,
    last_sequence: u64,
    observation_count: u64,
    first_started_us: u64,
    last_finished_us: u64,
    total_duration_us: u64,
    max_duration_us: u64,
    total_work_items: u64,
}

impl FrameLatencyPhaseSummary {
    fn new(sequence: u64, observation: FrameLatencyObservation) -> Self {
        Self {
            authority: observation.authority,
            stage: observation.stage,
            first_sequence: sequence,
            last_sequence: sequence,
            observation_count: 1,
            first_started_us: observation.started_us,
            last_finished_us: observation.started_us.saturating_add(observation.duration_us),
            total_duration_us: observation.duration_us,
            max_duration_us: observation.duration_us,
            total_work_items: u64::from(observation.work_items),
        }
    }

    fn observe(&mut self, sequence: u64, observation: FrameLatencyObservation) {
        self.last_sequence = sequence;
        self.observation_count = self.observation_count.saturating_add(1);
        self.first_started_us = self.first_started_us.min(observation.started_us);
        self.last_finished_us = self.last_finished_us.max(observation.started_us.saturating_add(observation.duration_us));
        self.total_duration_us = self.total_duration_us.saturating_add(observation.duration_us);
        self.max_duration_us = self.max_duration_us.max(observation.duration_us);
        self.total_work_items = self.total_work_items.saturating_add(u64::from(observation.work_items));
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FrameLatencyAggregate {
    observation_count: u64,
    total_duration_us: u64,
    max_duration_us: u64,
    total_work_items: u64,
}

impl FrameLatencyAggregate {
    fn observe(&mut self, observation: FrameLatencyObservation) {
        self.observation_count = self.observation_count.saturating_add(1);
        self.total_duration_us = self.total_duration_us.saturating_add(observation.duration_us);
        self.max_duration_us = self.max_duration_us.max(observation.duration_us);
        self.total_work_items = self.total_work_items.saturating_add(u64::from(observation.work_items));
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FrameLatencyStageTotal {
    authority_domain: FrameLatencyAuthorityDomain,
    stage: FrameLatencyStage,
    observation_count: u64,
    total_duration_us: u64,
    max_duration_us: u64,
    total_work_items: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FrameLatencySnapshot {
    summary_capacity: usize,
    total_observations: u64,
    summary_evictions: u64,
    refused: u64,
    stage_totals: Vec<FrameLatencyStageTotal>,
    recent_phases: Vec<FrameLatencyPhaseSummary>,
}

pub(crate) struct FrameLatencyRegistry<const N: usize> {
    recent_phases: [Option<FrameLatencyPhaseSummary>; N],
    active_indices: [[Option<usize>; FRAME_LATENCY_STAGE_COUNT]; FRAME_LATENCY_DOMAIN_COUNT],
    stage_totals: [[FrameLatencyAggregate; FRAME_LATENCY_STAGE_COUNT]; FRAME_LATENCY_DOMAIN_COUNT],
    next: usize,
    len: usize,
    total_observations: u64,
    summary_evictions: u64,
}

impl<const N: usize> FrameLatencyRegistry<N> {
    const fn new() -> Self {
        Self {
            recent_phases: [None; N],
            active_indices: [[None; FRAME_LATENCY_STAGE_COUNT]; FRAME_LATENCY_DOMAIN_COUNT],
            stage_totals: [[FrameLatencyAggregate { observation_count: 0, total_duration_us: 0, max_duration_us: 0, total_work_items: 0 }; FRAME_LATENCY_STAGE_COUNT]; FRAME_LATENCY_DOMAIN_COUNT],
            next: 0,
            len: 0,
            total_observations: 0,
            summary_evictions: 0,
        }
    }

    fn observe(&mut self, observation: FrameLatencyObservation) {
        self.total_observations = self.total_observations.saturating_add(1);
        let sequence = self.total_observations;
        let domain_index = observation.authority.domain.index();
        let stage_index = observation.stage.index();
        self.stage_totals[domain_index][stage_index].observe(observation);
        if let Some(index) = self.active_indices[domain_index][stage_index] {
            if let Some(summary) = self.recent_phases[index].as_mut().filter(|summary| summary.authority == observation.authority && summary.stage == observation.stage) {
                summary.observe(sequence, observation);
                return;
            }
            self.active_indices[domain_index][stage_index] = None;
        }
        if let Some(index) = self.recent_phases.iter().position(|summary| summary.is_some_and(|summary| summary.authority == observation.authority && summary.stage == observation.stage)) {
            self.active_indices[domain_index][stage_index] = Some(index);
            if let Some(summary) = self.recent_phases[index].as_mut() {
                summary.observe(sequence, observation);
            }
            return;
        }
        if N == 0 {
            return;
        }
        let index = self.next;
        if let Some(retired) = self.recent_phases[index] {
            let retired_active = &mut self.active_indices[retired.authority.domain.index()][retired.stage.index()];
            if *retired_active == Some(index) {
                *retired_active = None;
            }
            self.summary_evictions = self.summary_evictions.saturating_add(1);
        } else {
            self.len = self.len.saturating_add(1).min(N);
        }
        self.recent_phases[index] = Some(FrameLatencyPhaseSummary::new(sequence, observation));
        self.active_indices[domain_index][stage_index] = Some(index);
        self.next = self.next.saturating_add(1).checked_rem(N).unwrap_or(0);
    }

    fn snapshot(&self, refused: u64) -> FrameLatencySnapshot {
        let mut recent_phases = Vec::with_capacity(self.len);
        if N > 0 {
            let first = if self.len == N { self.next } else { 0 };
            for offset in 0..self.len {
                if let Some(summary) = self.recent_phases[(first + offset) % N] {
                    recent_phases.push(summary);
                }
            }
        }
        let mut stage_totals = Vec::with_capacity(FRAME_LATENCY_DOMAIN_COUNT * FRAME_LATENCY_STAGE_COUNT);
        for domain in FrameLatencyAuthorityDomain::ALL {
            for stage in FrameLatencyStage::ALL {
                let aggregate = self.stage_totals[domain.index()][stage.index()];
                if aggregate.observation_count == 0 {
                    continue;
                }
                stage_totals.push(FrameLatencyStageTotal {
                    authority_domain: domain,
                    stage,
                    observation_count: aggregate.observation_count,
                    total_duration_us: aggregate.total_duration_us,
                    max_duration_us: aggregate.max_duration_us,
                    total_work_items: aggregate.total_work_items,
                });
            }
        }
        FrameLatencySnapshot { summary_capacity: N, total_observations: self.total_observations, summary_evictions: self.summary_evictions, refused, stage_totals, recent_phases }
    }
}

static FRAME_LATENCY_REGISTRY: OnceLock<Mutex<FrameLatencyRegistry<FRAME_LATENCY_CAPACITY>>> = OnceLock::new();
static FRAME_LATENCY_REFUSED: AtomicU64 = AtomicU64::new(0);
static LATEST_FRAME_GENERATION: AtomicU64 = AtomicU64::new(0);

fn registry() -> &'static Mutex<FrameLatencyRegistry<FRAME_LATENCY_CAPACITY>> {
    FRAME_LATENCY_REGISTRY.get_or_init(|| Mutex::new(FrameLatencyRegistry::new()))
}

pub(crate) fn observe_frame_generation(generation: u64) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        LATEST_FRAME_GENERATION.fetch_max(generation, Ordering::Relaxed);
    }
}

pub(crate) fn latest_frame_generation() -> u64 {
    LATEST_FRAME_GENERATION.load(Ordering::Relaxed)
}

pub(crate) fn latest_frame_authority() -> FrameLatencyAuthority {
    FrameLatencyAuthority::renderer_frame(latest_frame_generation())
}

pub(crate) fn snapshot() -> FrameLatencySnapshot {
    let refused = FRAME_LATENCY_REFUSED.load(Ordering::Relaxed);
    match registry().lock() {
        Ok(registry) => registry.snapshot(refused),
        Err(poisoned) => poisoned.into_inner().snapshot(refused),
    }
}

pub(crate) struct FrameLatencyTimer {
    authority: FrameLatencyAuthority,
    stage: FrameLatencyStage,
    started_us: Option<u64>,
    work_items: u32,
}

impl FrameLatencyTimer {
    pub(crate) fn start(authority: FrameLatencyAuthority, stage: FrameLatencyStage, work_items: u32) -> Self {
        Self::start_if(semio_framework_trace::runtime_diagnostics_enabled(), authority, stage, work_items, semio_framework_trace::try_now_us)
    }

    fn start_if(clock_enabled: bool, authority: FrameLatencyAuthority, stage: FrameLatencyStage, work_items: u32, clock: impl FnOnce() -> Option<u64>) -> Self {
        let started_us = if clock_enabled { clock() } else { None };
        Self { authority, stage, started_us, work_items }
    }

    pub(crate) fn set_work_items(&mut self, work_items: usize) {
        self.work_items = u32::try_from(work_items).unwrap_or(u32::MAX);
    }
}

impl Drop for FrameLatencyTimer {
    fn drop(&mut self) {
        let Some(started_us) = self.started_us else { return };
        let Some(finished_us) = semio_framework_trace::try_now_us() else { return };
        let observation = FrameLatencyObservation { authority: self.authority, stage: self.stage, started_us, duration_us: finished_us.saturating_sub(started_us), work_items: self.work_items.max(1) };
        match registry().try_lock() {
            Ok(mut registry) => registry.observe(observation),
            Err(_) => {
                FRAME_LATENCY_REFUSED.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/⏱️frame-latency-diagnostics/🦀️.rs"]
mod tests;
