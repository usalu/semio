//! 🏁️ The original guard checks admission and includes optional telemetry in its terminal window.
use super::*;
use std::cell::{Cell, RefCell};

//#region 🕰️ClockFixture
thread_local! {
    static READINGS: RefCell<Option<[Option<u64>; 4]>> = const { RefCell::new(None) };
    static INDEX: Cell<usize> = const { Cell::new(0) };
    static SAMPLE_AT_TERMINAL: Cell<Option<usize>> = const { Cell::new(None) };
}
fn sample_count() -> Option<usize> {
    site_registry().try_lock().ok().map(|registry| registry.iter().flatten().find(|(site, _)| *site == "test.input-tail").map_or(0, |(_, ring)| ring.len()))
}
fn clock() -> Option<u64> {
    READINGS.with(|readings| match *readings.borrow() {
        Some(readings) => INDEX.with(|index| {
            let offset = index.get();
            index.set(offset + 1);
            if offset == 3 { SAMPLE_AT_TERMINAL.with(|sample| sample.set(sample_count())); }
            readings.get(offset).copied().flatten()
        }),
        None => default_clock_us(),
    })
}
struct ClockScope;
impl ClockScope {
    fn enter(readings: [Option<u64>; 4]) -> Self {
        install_clock(clock).unwrap();
        INDEX.with(|index| index.set(0));
        SAMPLE_AT_TERMINAL.with(|sample| sample.set(None));
        READINGS.with(|target| *target.borrow_mut() = Some(readings));
        Self
    }
}
impl Drop for ClockScope {
    fn drop(&mut self) { READINGS.with(|target| *target.borrow_mut() = None); }
}
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🔣️.json")).unwrap() }
//#endregion 🕰️ClockFixture

//#region 🧪️Tail
#[test]
fn watchdog_tail_uses_the_original_guard_for_admission_and_terminal() {
    let fixture = fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let readings = std::array::from_fn(|index| row["clock"][index].as_u64());
        let _clock = ClockScope::enter(readings);
        let operation = OperationId(731);
        let generation = Generation(51);
        let samples_before = sample_count().unwrap();
        let guard = Watchdog::start("test.input-tail", operation, generation, InteractiveStage::UiEvent);
        let admission = guard.admission_checkpoint();
        let admission_verdict = admission.verdict();
        assert_eq!(admission_verdict.is_fault(), row["admissionFault"].as_bool().unwrap());
        let terminal = admission.finish_after_telemetry();
        assert_eq!(terminal.is_fault(), row["terminalFault"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(terminal.elapsed_us(), row["terminalElapsed"].as_u64());
        assert_eq!(terminal.operation(), operation);
        assert_eq!(terminal.generation(), generation);
        assert_eq!(INDEX.with(Cell::get), 4);
        assert_eq!(SAMPLE_AT_TERMINAL.with(Cell::get), Some(samples_before + 1));
        eprintln!("[DEBUG] watchdog-tail case={} admissionFault={} terminalFault={} elapsed={:?}", row["name"], admission_verdict.is_fault(), terminal.is_fault(), terminal.elapsed_us());
    }
}

#[test]
fn watchdog_tail_a_previous_success_cannot_change_a_later_same_operation_result() {
    let operation = OperationId(731);
    let generation = Generation(51);
    let previous = {
        let _clock = ClockScope::enter([Some(100), Some(8099), Some(8099), Some(8099)]);
        let guard = Watchdog::start("test.input-tail", operation, generation, InteractiveStage::UiEvent);
        let admission = guard.admission_checkpoint();
        assert!(!admission.verdict().is_fault());
        admission.finish_after_telemetry()
    };
    let _clock = ClockScope::enter([Some(100), Some(8099), Some(8099), Some(8100)]);
    let guard = Watchdog::start("test.input-tail", operation, generation, InteractiveStage::UiEvent);
    let admission = guard.admission_checkpoint();
    assert!(!admission.verdict().is_fault());
    let terminal = admission.finish_after_telemetry();
    assert!(!previous.is_fault());
    assert!(terminal.is_fault());
    assert_eq!(terminal.elapsed_us(), Some(8000));
    assert_eq!(previous.operation(), terminal.operation());
    assert_eq!(previous.generation(), terminal.generation());
}

#[test]
fn watchdog_tail_held_optional_telemetry_keeps_terminal_fault_without_waiting() {
    let site = site_registry().lock().unwrap_or_else(PoisonError::into_inner);
    let violations = violation_ring().lock().unwrap_or_else(PoisonError::into_inner);
    let (returned, verdict) = std::thread::scope(|scope| {
        let (sent, received) = std::sync::mpsc::sync_channel(1);
        let worker = scope.spawn(move || {
            let _clock = ClockScope::enter([Some(100), Some(8099), Some(8100), Some(8101)]);
            let guard = Watchdog::start("test.input-tail", OperationId(731), Generation(51), InteractiveStage::UiEvent);
            let admission = guard.admission_checkpoint();
            let admission_verdict = admission.verdict();
            let terminal = admission.finish_after_telemetry();
            sent.send(()).unwrap();
            (admission_verdict, terminal)
        });
        let returned = received.recv_timeout(std::time::Duration::from_millis(100)).is_ok();
        drop(violations);
        drop(site);
        (returned, worker.join().unwrap())
    });
    assert!(returned);
    assert!(!verdict.0.is_fault());
    assert!(verdict.1.is_fault());
    assert_eq!(verdict.1.elapsed_us(), Some(8001));
}
//#endregion 🧪️Tail
