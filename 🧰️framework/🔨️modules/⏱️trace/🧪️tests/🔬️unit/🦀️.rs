
use super::*;

//#region 📈️PercentileRing
#[test]
fn percentile_ring_orders_samples_correctly() {
    let mut ring = PercentileRing::new();
    for value in [100u64, 200, 300, 400, 500, 600, 700, 800, 900, 1000] {
        ring.record(value);
    }
    assert_eq!(ring.len(), 10);
    assert!(!ring.is_empty());
    assert_eq!(ring.p50(), 600);
    assert_eq!(ring.p95(), 1000);
    assert_eq!(ring.p99(), 1000);
}

#[test]
fn percentile_ring_wraps_past_capacity_keeping_newest() {
    let mut ring = PercentileRing::new();
    let total = SAMPLE_RING_CAPACITY as u64 * 2;
    for value in 0..total {
        ring.record(value);
    }
    assert_eq!(ring.len(), SAMPLE_RING_CAPACITY);
    let oldest_retained = total - SAMPLE_RING_CAPACITY as u64;
    let n = SAMPLE_RING_CAPACITY as u64;
    assert_eq!(ring.p50() as u64, oldest_retained + n / 2);
    assert_eq!(ring.p99() as u64, oldest_retained + n - 1);
}

#[test]
fn percentile_ring_empty_reads_as_zero() {
    let ring = PercentileRing::new();
    assert!(ring.is_empty());
    assert_eq!(ring.p50(), 0);
    assert_eq!(ring.p95(), 0);
    assert_eq!(ring.p99(), 0);
}
//#endregion 📈️PercentileRing

//#region 🐕️Watchdog
#[test]
fn watchdog_reports_contract_violation_on_overrun() {
    let before = Watchdog::violation_count();
    let operation = allocate_operation_id();
    let generation = Generation(1);
    {
        let _guard = Watchdog::start("test.watchdog.overrun", operation, generation, InteractiveStage::InteractiveStep);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert!(Watchdog::violation_count() > before, "expected a contract violation to be recorded");
    let violation = Watchdog::violations().into_iter().rev().find(|violation| violation.operation == operation).expect("violation for this operation must be queryable");
    assert_eq!(violation.site, "test.watchdog.overrun");
    assert_eq!(violation.stage, InteractiveStage::InteractiveStep);
    assert!(violation.elapsed_us >= INTERACTIVE_STEP_CEILING_US);
}

#[test]
fn watchdog_stays_silent_under_ceiling() {
    let before = Watchdog::violation_count();
    let operation = allocate_operation_id();
    {
        let _guard = Watchdog::start("test.watchdog.under-ceiling", operation, Generation(1), InteractiveStage::UiEvent);
    }
    assert_eq!(Watchdog::violation_count(), before, "a fast guard must never be reported as a violation");
}
//#endregion 🐕️Watchdog

//#region 🧵️ThreadRole
#[test]
fn thread_role_registers_and_asserts() {
    register_worker_thread(7);
    assert_eq!(current_role(), ThreadRole::Worker(7));
    assert!(is_worker_thread());
    assert!(!is_ui_thread());
    assert_worker_thread();

    register_ui_thread();
    assert_eq!(current_role(), ThreadRole::Ui);
    assert!(is_ui_thread());
    assert!(!is_worker_thread());
    assert_ui_thread();
}

#[test]
fn io_boundary_thread_registers_distinct_from_worker_and_ui() {
    register_io_boundary_thread("process-shard-reader");
    assert_eq!(current_role(), ThreadRole::IoBoundary("process-shard-reader"));
    assert!(is_io_boundary_thread());
    assert!(!is_worker_thread());
    assert!(!is_ui_thread());
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "not the UI thread")]
fn assert_ui_thread_panics_off_ui_thread() {
    register_worker_thread(3);
    assert_ui_thread();
}
//#endregion 🧵️ThreadRole

//#region 📊️Counters
#[test]
fn counters_snapshot_reflects_updates() {
    let workers = WorkerCounters::new();
    assert_eq!(workers.worker_started(), 1);
    assert_eq!(workers.worker_started(), 2);
    assert_eq!(workers.worker_finished(), 1);
    assert_eq!(workers.active(), 1);

    let permits = PermitLedger::new();
    assert_eq!(permits.acquire(), 1);
    assert_eq!(permits.release(), 0);
    assert_eq!(permits.occupancy(), 0);

    let queue = QueueCounter::new();
    assert_eq!(queue.enqueued(64), QueueCounterSnapshot { items: 1, bytes: 64 });
    assert_eq!(queue.enqueued(32), QueueCounterSnapshot { items: 2, bytes: 96 });
    assert_eq!(queue.dequeued(64), QueueCounterSnapshot { items: 1, bytes: 32 });
    assert_eq!(queue.snapshot(), QueueCounterSnapshot { items: 1, bytes: 32 });
}
//#endregion 📊️Counters

//#region 🛰️Trace
#[test]
fn trace_follows_one_operation_start_to_preview_to_commit() {
    let operation = allocate_operation_id();
    let generation = Generation(1);
    record_operation_started(operation, generation);
    record_stage_changed(operation, generation, "gathering-input");
    record_preview_published(operation, generation);
    record_checkpoint(operation, generation);
    record_committed(operation, generation);

    let events = trace_snapshot_for(operation);
    let stages: Vec<TraceStage> = events.iter().map(|event| event.stage).collect();
    assert_eq!(stages, vec![TraceStage::Started, TraceStage::StageChanged { label: "gathering-input" }, TraceStage::PreviewPublished, TraceStage::Checkpoint, TraceStage::Committed]);
    assert!(events.windows(2).all(|pair| pair[0].sequence < pair[1].sequence));
    assert!(events.iter().all(|event| event.generation == generation));

    assert!(preview_latency_us(operation).is_some());
}

#[test]
fn cancellation_latency_measures_requested_to_observed() {
    let operation = allocate_operation_id();
    let generation = Generation(1);
    record_operation_started(operation, generation);
    record_cancel_requested(operation, generation);
    std::thread::sleep(std::time::Duration::from_millis(1));
    record_cancelled(operation, generation);

    let latency = cancellation_latency_us(operation).expect("cancellation latency must be queryable end to end");
    assert!(latency > 0, "expected the sleep between cancel-requested and cancelled to show up");
}

#[test]
fn latency_helpers_are_none_before_their_events_land() {
    let operation = allocate_operation_id();
    assert_eq!(preview_latency_us(operation), None);
    assert_eq!(cancellation_latency_us(operation), None);
}
//#endregion 🛰️Trace

//#region 🕰️Clock
#[test]
fn clock_is_monotonically_non_decreasing() {
    let first = now_us();
    let second = now_us();
    assert!(second >= first);
}
//#endregion 🕰️Clock
