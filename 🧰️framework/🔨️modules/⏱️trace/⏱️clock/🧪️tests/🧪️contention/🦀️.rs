//! 🧪️ Held telemetry locks must not wait inside a callback or erase its watchdog authority.

use super::*;
use std::sync::mpsc;
use std::time::Duration;

//#region 🧰️Fixture
fn case(id: &str) -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧪️contention/🔣️.json")).unwrap();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧪️contention/🔣️.json")).unwrap();
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

//#region 📒️SustainedOverrun
#[test]
fn microsecond_sustained_overrun_ledger_quarantines_only_attributable_steps() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧪️contention/🔣️.json")).unwrap();
    assert_eq!(fixture["sustainedOverrunSteps"].as_u64().unwrap(), u64::from(SUSTAINED_OVERRUN_QUARANTINE_STEPS));
    let operation = allocate_operation_id();
    let generation = Generation(83);
    for law in fixture["overruns"].as_array().unwrap() {
        let mut ledger = StepOverrunLedger::new();
        let mut terminal_index = None;
        for (index, elapsed) in law["elapsedUs"].as_array().unwrap().iter().enumerate() {
            let elapsed_us = elapsed.as_u64().unwrap();
            let guard = Watchdog { site: "test.sustained-overrun", operation, generation, stage: InteractiveStage::InteractiveStep, start_us: Some(0), finished: true };
            let verdict = guard.verdict_at(Some(elapsed_us));
            assert_eq!(verdict.is_fault(), interactive_step_contract_violated(elapsed_us), "one sample stays a measurement");
            let quarantine = ledger.admit(&verdict);
            if quarantine.is_terminal() && terminal_index.is_none() {
                terminal_index = Some(index);
            }
            assert!(
                !quarantine.is_terminal() || matches!(quarantine, StepQuarantine::SustainedOverrun { consecutive } if consecutive >= SUSTAINED_OVERRUN_QUARANTINE_STEPS),
                "only a sustained run may quarantine a measured step"
            );
        }
        assert_eq!(terminal_index.map(|index| index as u64), law["terminalIndex"].as_u64(), "{}", law["id"]);
        assert_eq!(u64::from(ledger.consecutive_overruns()), law["consecutive"].as_u64().unwrap(), "{}", law["id"]);
        assert_eq!(u64::from(ledger.longest_overrun_run()), law["longestRun"].as_u64().unwrap(), "{}", law["id"]);
        assert_eq!(u64::from(ledger.total_overruns()), law["total"].as_u64().unwrap(), "{}", law["id"]);
        assert_eq!(ledger.worst_elapsed_us(), law["worstElapsedUs"].as_u64().unwrap(), "{}", law["id"]);
        eprintln!(
            "[DEBUG] sustained overrun ledger {} terminal_index={terminal_index:?} consecutive={} longest={} total={} worst={}us",
            law["id"],
            ledger.consecutive_overruns(),
            ledger.longest_overrun_run(),
            ledger.total_overruns(),
            ledger.worst_elapsed_us()
        );
    }
}

#[test]
fn microsecond_unusable_clock_reading_stays_terminal_and_resets_the_overrun_run() {
    let operation = allocate_operation_id();
    let generation = Generation(84);
    let guard = Watchdog { site: "test.sustained-overrun.clock", operation, generation, stage: InteractiveStage::InteractiveStep, start_us: Some(100), finished: true };
    let mut ledger = StepOverrunLedger::new();
    assert!(matches!(ledger.admit(&guard.verdict_at(Some(9_000))), StepQuarantine::RecordedOverrun { consecutive: 1 }));
    assert!(matches!(ledger.admit(&guard.verdict_at(None)), StepQuarantine::ClockFault(CallbackClockFault::Missing)));
    assert_eq!(ledger.consecutive_overruns(), 0, "an unmeasured step cannot count toward an attribution it never supported");
    assert!(matches!(ledger.admit(&guard.verdict_at(Some(99))), StepQuarantine::ClockFault(CallbackClockFault::Backward)));
    let (recorded, quarantines) = step_overrun_counts();
    assert!(recorded > 0);
    eprintln!("[DEBUG] process overrun counters recorded={recorded} sustained_quarantines={quarantines}");
}
//#endregion 📒️SustainedOverrun
