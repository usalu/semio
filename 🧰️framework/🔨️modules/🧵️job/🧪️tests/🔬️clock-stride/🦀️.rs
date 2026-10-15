use semio_framework_job::*;
use std::sync::atomic::{AtomicU64, Ordering};

static FAKE_NOW_US: AtomicU64 = AtomicU64::new(0);
static FAKE_READS: AtomicU64 = AtomicU64::new(0);
static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn fake_now_us() -> Option<u64> {
    FAKE_READS.fetch_add(1, Ordering::SeqCst);
    Some(FAKE_NOW_US.load(Ordering::SeqCst))
}

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🪜️clock-stride-law.json")).expect("clock-stride law fixture")
}

/// 🏃️ Runs units of `unit_cost_us` in one step that began at `start_us` until the step's clock
/// reports the deadline; answers (real reads, overshoot past the deadline in µs).
fn run_window(stride: &mut ClockStride, start_us: u64, window_us: u64, unit_cost_us: u64) -> (u64, u64) {
    FAKE_NOW_US.store(start_us, Ordering::SeqCst);
    FAKE_READS.store(0, Ordering::SeqCst);
    let deadline_us = start_us + window_us;
    stride.begin(Some(start_us));
    loop {
        FAKE_NOW_US.fetch_add(unit_cost_us, Ordering::SeqCst);
        if stride.read(fake_now_us).is_none_or(|now_us| now_us >= deadline_us) {
            return (FAKE_READS.load(Ordering::SeqCst), FAKE_NOW_US.load(Ordering::SeqCst) - deadline_us);
        }
    }
}

#[test]
fn a_clock_stride_reads_about_once_per_interval_and_bounds_its_overshoot() {
    let _serial = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let law = law();
    assert_eq!(ClockStride::TARGET_READ_INTERVAL_US, law["targetReadIntervalUs"].as_u64().unwrap());
    assert_eq!(u64::from(ClockStride::MAXIMUM_STRIDE), law["maximumStride"].as_u64().unwrap());
    for case in law["cases"].as_array().unwrap() {
        let mut stride = ClockStride::new();
        let (reads, overshoot) = run_window(&mut stride, 1_000_000, case["windowUs"].as_u64().unwrap(), case["unitCostUs"].as_u64().unwrap());
        assert!(reads <= case["maxReads"].as_u64().unwrap(), "{}: {reads} reads", case["name"]);
        assert!(overshoot <= case["maxOvershootUs"].as_u64().unwrap(), "{}: {overshoot} µs past the deadline", case["name"]);
    }
}

#[test]
fn a_new_step_starts_at_its_drivers_reading_and_keeps_the_calibrated_stride() {
    let _serial = SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let law = law();
    let mut stride = ClockStride::new();
    let mut start_us = 5_000_000;
    let _ = run_window(&mut stride, start_us, 2_000, 2);
    for _ in 0..law["newStep"]["windows"].as_u64().unwrap() {
        start_us += 1_000_000;
        let before = stride.stride();
        FAKE_NOW_US.store(start_us + 7, Ordering::SeqCst);
        FAKE_READS.store(0, Ordering::SeqCst);
        stride.begin(Some(start_us));
        assert_eq!(stride.latest_us(), Some(start_us), "the step's first reading is its driver's entry reading");
        assert_eq!(stride.stride(), before, "a new step keeps the calibrated stride");
        if before > 1 {
            assert_eq!(stride.read(fake_now_us), Some(start_us));
            assert_eq!(FAKE_READS.load(Ordering::SeqCst), law["newStep"]["readsForTheFirstCall"].as_u64().unwrap(), "the first in-step call costs no read of its own");
        }
        let _ = run_window(&mut stride, start_us, 2_000, 2);
    }
}

/// 🧮️ The logical clock advances exactly one microsecond per read and belongs to its thread: a law driven by it measures
/// clock reads, never the wall time a descheduled thread loses.
#[test]
fn the_logical_clock_advances_one_microsecond_per_read_per_thread() {
    let first = logical_now_us().expect("logical clock reads");
    assert_eq!(logical_now_us(), Some(first + 1));
    assert_eq!(logical_now_us(), Some(first + 2));
    let other = std::thread::spawn(|| (logical_now_us(), logical_now_us())).join().expect("logical clock thread");
    assert_eq!(other, (Some(1), Some(2)));
    assert_eq!(logical_now_us(), Some(first + 3));
}
