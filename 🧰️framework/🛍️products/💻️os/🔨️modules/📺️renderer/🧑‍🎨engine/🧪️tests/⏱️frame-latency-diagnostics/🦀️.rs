use super::*;
use serde::Deserialize;
use std::cell::Cell;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    production_capacity: usize,
    oracle_capacity: usize,
    required_stages: Vec<FrameLatencyStage>,
    observations: Vec<FixtureObservation>,
    expected: FixtureExpected,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureObservation {
    authority: FrameLatencyAuthority,
    stage: FrameLatencyStage,
    started_us: u64,
    duration_us: u64,
    work_items: u32,
    repeat: u32,
    spacing_us: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureExpected {
    total_observations: u64,
    summary_evictions: u64,
    recent_phases: Vec<FrameLatencyPhaseSummary>,
    stage_totals: Vec<FrameLatencyStageTotal>,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/⏱️frame-latency/🔣️.json")).expect("frame latency fixture")
}

#[test]
fn fixed_registry_matches_neutral_phase_coalescing_and_cumulative_totals() {
    let fixture = fixture();
    assert_eq!(fixture.production_capacity, FRAME_LATENCY_CAPACITY);
    assert_eq!(fixture.oracle_capacity, 4);
    assert_eq!(fixture.required_stages, FrameLatencyStage::ALL);
    let mut registry = FrameLatencyRegistry::<4>::new();
    for row in fixture.observations {
        for offset in 0..row.repeat {
            registry.observe(FrameLatencyObservation {
                authority: row.authority,
                stage: row.stage,
                started_us: row.started_us.saturating_add(u64::from(offset).saturating_mul(row.spacing_us)),
                duration_us: row.duration_us,
                work_items: row.work_items,
            });
        }
    }
    let snapshot = registry.snapshot(0);
    assert_eq!(snapshot.total_observations, fixture.expected.total_observations);
    assert_eq!(snapshot.summary_evictions, fixture.expected.summary_evictions);
    assert_eq!(snapshot.recent_phases, fixture.expected.recent_phases);
    assert_eq!(snapshot.stage_totals, fixture.expected.stage_totals);
}

#[test]
fn equal_generation_numbers_keep_distinct_authority_domains() {
    let mut registry = FrameLatencyRegistry::<4>::new();
    for authority in [FrameLatencyAuthority::renderer_frame(8), FrameLatencyAuthority::browser_input_batch(8)] {
        registry.observe(FrameLatencyObservation { authority, stage: FrameLatencyStage::WorkerTick, started_us: 1, duration_us: 2, work_items: 1 });
    }
    let snapshot = registry.snapshot(0);
    assert_eq!(snapshot.recent_phases.len(), 2);
    assert_ne!(snapshot.recent_phases[0].authority.domain, snapshot.recent_phases[1].authority.domain);
}

#[test]
fn an_interleaved_live_phase_reuses_its_authority_generation_summary() {
    let mut registry = FrameLatencyRegistry::<4>::new();
    for generation in [8, 9, 8] {
        registry.observe(FrameLatencyObservation { authority: FrameLatencyAuthority::renderer_frame(generation), stage: FrameLatencyStage::TransactionRouteIntents, started_us: generation, duration_us: 1, work_items: 1 });
    }
    let snapshot = registry.snapshot(0);
    assert_eq!(snapshot.recent_phases.len(), 2);
    assert_eq!(snapshot.recent_phases[0].authority.generation, 8);
    assert_eq!(snapshot.recent_phases[0].observation_count, 2);
    assert_eq!(snapshot.recent_phases[0].last_sequence, 3);
}

#[test]
fn disabled_timer_does_not_read_the_clock_or_initialize_diagnostics() {
    let reads = Cell::new(0_u32);
    let timer = FrameLatencyTimer::start_if(false, FrameLatencyAuthority::browser_input_batch(9), FrameLatencyStage::WorkerTick, 1, || {
        reads.set(reads.get() + 1);
        Some(10)
    });
    assert_eq!(reads.get(), 0);
    assert!(timer.started_us.is_none());
}

#[test]
fn transaction_and_presenter_state_machines_cover_every_phase_stage() {
    use crate::{AppPresentPhase as Present, FrameTransactionStage as Transaction};
    assert_eq!(Transaction::DrainProjectionDeltas.latency_stage(), FrameLatencyStage::TransactionDrainProjectionDeltas);
    assert_eq!(Transaction::RouteIntents.latency_stage(), FrameLatencyStage::TransactionRouteIntents);
    assert_eq!(Transaction::FlushEffects.latency_stage(), FrameLatencyStage::TransactionFlushEffects);
    assert_eq!(Transaction::PresentSurface.latency_stage(), FrameLatencyStage::TransactionPresentSurface);
    assert_eq!(Transaction::ReconcileTree.latency_stage(), FrameLatencyStage::TransactionReconcileTree);
    assert_eq!(Transaction::BuildRenderPackets.latency_stage(), FrameLatencyStage::TransactionBuildRenderPackets);
    assert_eq!(Transaction::PublishSnapshot.latency_stage(), FrameLatencyStage::TransactionPublishSnapshot);
    assert_eq!(Present::Fullscreen.latency_stage(), FrameLatencyStage::PresenterFullscreen);
    assert_eq!(Present::Engine.latency_stage(), FrameLatencyStage::PresenterEngine);
    assert_eq!(Present::BeginGpu.latency_stage(), FrameLatencyStage::PresenterBeginGpu);
    assert_eq!(Present::Ownership.latency_stage(), FrameLatencyStage::PresenterOwnership);
    assert_eq!(Present::Uploads.latency_stage(), FrameLatencyStage::PresenterUploads);
    assert_eq!(Present::Stage.latency_stage(), FrameLatencyStage::PresenterStage);
    assert_eq!(Present::Render.latency_stage(), FrameLatencyStage::QueueSubmit);
    assert_eq!(Present::CloseGpu.latency_stage(), FrameLatencyStage::PresenterCloseGpu);
    assert_eq!(Present::Acknowledge.latency_stage(), FrameLatencyStage::PresenterAcknowledge);
    assert_eq!(Present::ProgressAcknowledge.latency_stage(), FrameLatencyStage::PresenterProgressAcknowledge);
    assert_eq!(Present::Aborted.latency_stage(), FrameLatencyStage::PresenterAborted);
    assert_eq!(Present::Directives.latency_stage(), FrameLatencyStage::PresenterDirectives);
}
