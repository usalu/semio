//! 🧪️ Held telemetry locks must not wait inside a callback or erase its watchdog authority.

use super::*;
use std::sync::mpsc;
use std::time::Duration;

//#region 🧰️Fixture
fn case(id: &str) -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️contention.json")).unwrap();
    fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == id).unwrap().clone()
}

fn callback_returns_while_held<T: Send>(callback: impl FnOnce() -> T + Send, release: impl FnOnce()) -> (bool, T) {
    std::thread::scope(|scope| {
        let (started_tx, started_rx) = mpsc::sync_channel(1);
        let (done_tx, done_rx) = mpsc::sync_channel(1);
        let worker = scope.spawn(move || {
            started_tx.send(()).unwrap();
            let result = callback();
            done_tx.send(()).unwrap();
            result
        });
        started_rx.recv().unwrap();
        let returned = done_rx.recv_timeout(Duration::from_millis(100)).is_ok();
        release();
        (returned, worker.join().unwrap())
    })
}

fn held_watchdog(id: &str) {
    let law = case(id);
    let operation = allocate_operation_id();
    let generation = Generation(37);
    let watchdog = Watchdog::start("test.contention.watchdog", operation, generation, InteractiveStage::InteractiveStep);
    std::thread::sleep(Duration::from_millis(10));
    let (returned, verdict) = match law["lock"].as_str().unwrap() {
        "site" => {
            let guard = site_registry().lock().unwrap_or_else(PoisonError::into_inner);
            callback_returns_while_held(|| watchdog.finish(), || drop(guard))
        }
        "violation" => {
            let guard = violation_ring().lock().unwrap_or_else(PoisonError::into_inner);
            callback_returns_while_held(|| watchdog.finish(), || drop(guard))
        }
        _ => unreachable!(),
    };
    let violation = verdict.violation();
    assert_eq!(verdict.operation(), operation);
    assert_eq!(verdict.generation(), generation);
    assert!(verdict.is_fault());
    assert_eq!(violation.is_some(), law["violationRetained"].as_bool().unwrap());
    assert!(violation.is_some_and(|row| interactive_step_contract_violated(row.elapsed_us)));
    eprintln!("[DEBUG] telemetry contention {id} returns_while_held={returned} exact_violation_retained=true");
    assert_eq!(returned, law["returnsWhileHeld"].as_bool().unwrap());
}
//#endregion 🧰️Fixture

//#region 🔒️Contention
#[test]
fn microsecond_telemetry_contention_timer_site_does_not_wait() {
    let law = case("timer-site-held");
    let timer = StepTimer::start("test.contention.timer");
    let guard = site_registry().lock().unwrap_or_else(PoisonError::into_inner);
    let (returned, ()) = callback_returns_while_held(|| drop(timer), || drop(guard));
    eprintln!("[DEBUG] telemetry contention timer-site-held returns_while_held={returned}");
    assert_eq!(returned, law["returnsWhileHeld"].as_bool().unwrap());
}

#[test]
fn microsecond_telemetry_contention_watchdog_site_preserves_fault_without_waiting() {
    held_watchdog("watchdog-site-held");
}

#[test]
fn microsecond_telemetry_contention_watchdog_violation_preserves_fault_without_waiting() {
    held_watchdog("watchdog-violation-held");
}

#[test]
fn microsecond_telemetry_contention_event_returns_exact_event_without_waiting() {
    let law = case("event-ring-held");
    let operation = allocate_operation_id();
    let generation = Generation(37);
    let guard = trace_ring().lock().unwrap_or_else(PoisonError::into_inner);
    let (returned, event) = callback_returns_while_held(|| record_failed(operation, generation), || drop(guard));
    let event = event.expect("real-clock exact event exists despite optional ring contention");
    assert_eq!(event.operation, operation);
    assert_eq!(event.generation, generation);
    assert_eq!(event.stage, TraceStage::Failed);
    assert!(event.sequence > 0);
    assert_eq!(law["returnedEvent"], true);
    eprintln!("[DEBUG] telemetry contention event-ring-held returns_while_held={returned} exact_event=true");
    assert_eq!(returned, law["returnsWhileHeld"].as_bool().unwrap());
}

#[test]
fn microsecond_telemetry_exact_verdict_survives_saturation_and_invalid_clock() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️contention.json")).unwrap();
    let operation = allocate_operation_id();
    let generation = Generation(51);
    for law in fixture["verdicts"].as_array().unwrap() {
        let guard = Watchdog { site: "test.exact-verdict", operation, generation, stage: InteractiveStage::InteractiveStep, start_us: law["start"].as_u64(), finished: true };
        assert_eq!(guard.is_admitted(), law["start"].as_u64().is_some());
        let verdict = guard.report(law["end"].as_u64());
        assert_eq!(verdict.is_fault(), law["fault"].as_bool().unwrap());
        assert_eq!(verdict.clock_fault().map(|fault| format!("{fault:?}")), law["clockFault"].as_str().map(str::to_owned));
    }
    let guard = Watchdog { site: "test.exact-verdict", operation, generation, stage: InteractiveStage::InteractiveStep, start_us: Some(100), finished: true };
    let verdict = guard.report(Some(8_100));
    {
        let mut ring = violation_ring().lock().unwrap_or_else(PoisonError::into_inner);
        for _ in 0..VIOLATION_RING_CAPACITY + 1 {
            ring.push(ContractViolation { site: "test.unrelated", operation: OperationId(0), generation, stage: InteractiveStage::UiEvent, elapsed_us: 8_001 });
        }
        let retained = guard.report(Some(8_100));
        assert_eq!(retained, verdict);
    }
    assert_eq!(verdict.violation().unwrap().elapsed_us, 8_000);
    assert!(verdict.is_fault());
    assert_eq!(guard.report(Some(99)).clock_fault(), Some(CallbackClockFault::Backward));
    assert_eq!(guard.report(None).clock_fault(), Some(CallbackClockFault::Missing));
    eprintln!("[DEBUG] exact callback verdict survives full/contended telemetry and rejects backward/missing clocks");
}
//#endregion 🔒️Contention
