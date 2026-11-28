//! 📦️ Unit tests of runtime-instances; the work package moves the tests of its sources here.

#[cfg(test)]
mod close_stall_laws {
    use crate::inst_cleanup_turns::runtime_close_stall_verdict;
    use semio_plugin_reactor_lifecycle::life_cleanup_pumps::runtime_cleanup_reading_is_starved;
    use crate::inst_opening::{RUNTIME_CLOSE_STALL_CREDIT_US, RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT};
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
    use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use semio_plugin_reactor_lifecycle::life_cleanup_vocabulary::{RuntimeCleanupFault, RuntimeCloseStatus};
    use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

    fn stall_state() -> (AtomicU8, AtomicU64) {
        (AtomicU8::new(0), AtomicU64::new(0))
    }

    fn zero_progress() -> Option<PluginLifecycleStep> {
        Some(PluginLifecycleStep::Progress(RetainedCloneProgress::default()))
    }

    fn starved_clock_cx<R>(now_us: fn() -> Option<u64>, run: impl FnOnce(&semio_framework_job::StepContext<'_>) -> R) -> R {
        let (mut sequence, mut receipt) = (0u64, RetainedCloneProgress::default());
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 65_536, maximum_release_bytes: 262_144, maximum_depth: 64 };
        let cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(7), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(64, 8_000, grant), semio_framework_job::root_cancel_token(), now_us, &mut sequence, &mut receipt);
        run(&cx)
    }

    /// 🌫️ LAW: a close step whose own budget the HOST spent (a loaded browser stretching the turn past its deadline) read nothing from the ladder,
    /// so its empty reading is host scheduling latency, never a stall. Before 2026-11-25 such steps bought stall debt and a ladder that was merely
    /// waiting faulted `plugin.internal.zero-progress` at `elapsed 10500us, ceiling 8000us` while retiring the predecessor after Open File.
    /// Reproduced with an injected clock that is already past every step's deadline.
    #[test]
    fn host_starved_empty_close_steps_never_buy_structural_stall_credit() {
        let (stalled, since) = stall_state();
        for step in 0..1_024u64 {
            let contended = starved_clock_cx(|| Some(10_500), |cx| runtime_cleanup_reading_is_starved(zero_progress(), cx.should_yield()));
            assert!(contended, "a step past its deadline reads nothing");
            assert_eq!(runtime_close_stall_verdict(contended, zero_progress(), &stalled, &since, Some(1 + step * 1_500)), (RuntimeCloseStatus::Ready, 0));
        }
        assert_eq!((stalled.load(Ordering::SeqCst), since.load(Ordering::SeqCst)), (0, 0));
    }

    fn ladder_receipt(unit_runs: bool, remaining: &mut u32) -> Option<PluginLifecycleStep> {
        if unit_runs && *remaining > 0 {
            *remaining -= 1;
            return Some(PluginLifecycleStep::Progress(RetainedCloneProgress { released_bytes: 1, ..Default::default() }));
        }
        zero_progress()
    }

    /// 🌫️ LAW: a STARVED clock, where every turn starts already past its deadline, still completes a cleanup of `UNITS` rungs across turns without a fault. The turns whose inner step ran out of clock read empty and are classified through the production witness (`should_yield`), so they buy no debt; the turns that move a rung clear the run. Reproduced with an injected clock past every deadline, 4 096 turns, and a ladder that only moves on every third turn.
    #[test]
    fn a_starved_clock_completes_the_cleanup_across_turns_without_a_fault() {
        const UNITS: u32 = 40;
        let (stalled, since) = stall_state();
        let (mut remaining, mut turns, mut faulted) = (UNITS, 0u64, false);
        while remaining > 0 && turns < 4_096 {
            let moves = turns % 3 == 2;
            let reading = ladder_receipt(moves, &mut remaining);
            let contended = starved_clock_cx(|| Some(10_500), |cx| {
                assert!(cx.should_yield(), "every turn starts past its deadline");
                runtime_cleanup_reading_is_starved(reading, true)
            });
            let (status, _) = runtime_close_stall_verdict(contended, reading, &stalled, &since, Some(1 + turns * 9_000));
            faulted |= matches!(status, RuntimeCloseStatus::Fault(_));
            turns += 1;
        }
        assert_eq!(remaining, 0, "the cleanup completes across starved turns");
        assert!(!faulted, "an expired clock yields and retries, it never faults");
    }

    /// 🌫️ LAW: the same starved clock, live pump: turns that read empty past the deadline never fault however many pass, and turns that move a rung keep the run clear.
    #[test]
    fn a_starved_clock_never_faults_the_live_pump_across_turns() {
        use crate::inst_cleanup_turns::runtime_live_cleanup_nonterminal_status;
        use semio_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeMaintenanceStatus;
        let (stalled, since) = (std::sync::atomic::AtomicU32::new(0), AtomicU64::new(0));
        let seen = AtomicU64::new(0);
        let mut remaining = u32::MAX;
        for turn in 0..4_096u64 {
            let reading = ladder_receipt(turn % 3 == 2, &mut remaining);
            let contended = starved_clock_cx(|| Some(10_500), |cx| runtime_cleanup_reading_is_starved(reading, cx.should_yield()));
            let (status, _) = runtime_live_cleanup_nonterminal_status(contended, reading, &stalled, &since, Some(1 + turn * 9_000), 0, &seen);
            assert_eq!(status, RuntimeMaintenanceStatus::Ready, "turn {turn}");
        }
    }

    /// 🧱️ LAW: a REAL stall, a body that never progresses while funded, is still refused with `plugin.internal.zero-progress` after the deterministic bound, however little wall time that takes and with the deadline never reached. Close and live alike.
    #[test]
    fn a_funded_body_that_never_progresses_faults_after_the_deterministic_bound() {
        use crate::inst_cleanup_turns::runtime_live_cleanup_nonterminal_status;
        use semio_plugin_reactor_lifecycle::life_cleanup_vocabulary::{RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT, RuntimeMaintenanceStatus};
        let (stalled, since) = stall_state();
        let mut first_fault = None;
        for turn in 1..=4 * u64::from(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT) {
            let contended = starved_clock_cx(|| Some(1), |cx| runtime_cleanup_reading_is_starved(zero_progress(), cx.should_yield()));
            assert!(!contended, "the funded turn holds its budget");
            if matches!(runtime_close_stall_verdict(contended, zero_progress(), &stalled, &since, Some(1)).0, RuntimeCloseStatus::Fault(RuntimeCleanupFault::ZeroProgress)) {
                first_fault.get_or_insert(turn);
            }
        }
        assert_eq!(first_fault, Some(u64::from(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT)));
        let (live, live_since, seen) = (std::sync::atomic::AtomicU32::new(0), AtomicU64::new(0), AtomicU64::new(0));
        let mut first_live_fault = None;
        for turn in 1..=2 * u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT) {
            let contended = starved_clock_cx(|| Some(1), |cx| runtime_cleanup_reading_is_starved(zero_progress(), cx.should_yield()));
            if runtime_live_cleanup_nonterminal_status(contended, zero_progress(), &live, &live_since, Some(1), 0, &seen).0 == RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress) {
                first_live_fault.get_or_insert(turn);
            }
        }
        assert_eq!(first_live_fault, Some(u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT)));
    }

    /// 🌫️ LAW: only a step that still held its budget when it read an empty ladder is evidence; a wedged ladder polled with full budget still faults,
    /// and a starved reading that carries real progress or an external wait is never reclassified.
    #[test]
    fn full_budget_empty_close_steps_still_fault_and_other_readings_are_never_starved() {
        let (stalled, since) = stall_state();
        let mut verdict = (RuntimeCloseStatus::Ready, 0);
        for step in 0..u64::from(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT) {
            let contended = starved_clock_cx(|| Some(1), |cx| runtime_cleanup_reading_is_starved(zero_progress(), cx.should_yield()));
            assert!(!contended);
            verdict = runtime_close_stall_verdict(contended, zero_progress(), &stalled, &since, Some(1 + step * RUNTIME_CLOSE_STALL_CREDIT_US));
        }
        assert_eq!(verdict.0, RuntimeCloseStatus::Fault(RuntimeCleanupFault::ZeroProgress));
        for reading in [None, Some(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })), Some(PluginLifecycleStep::Blocked { reason: "external" }), Some(PluginLifecycleStep::AwaitingInput { reason: "ack" })] {
            assert!(!runtime_cleanup_reading_is_starved(reading, true));
        }
    }


    /// 🌫️ LAW: a live maintenance pass the turn had already starved (a funded decode job ran the turn past its deadline, then the repeated live pump read an empty ladder over many milliseconds) is scheduling latency, never a stall. Reproduced with an injected clock past every deadline and a virtual clock that advances 1.5 ms per reading for 1 024 readings; the same pump with its budget held still faults.
    #[test]
    fn host_starved_empty_live_steps_never_buy_structural_stall_credit() {
        use crate::inst_cleanup_turns::runtime_live_cleanup_nonterminal_status;
        use semio_plugin_reactor_lifecycle::life_cleanup_vocabulary::{RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT, RuntimeMaintenanceStatus};
        use std::sync::atomic::AtomicU32;
        let (stalled, since) = (AtomicU32::new(0), AtomicU64::new(0));
        for step in 0..1_024u64 {
            let contended = starved_clock_cx(|| Some(10_500), |cx| runtime_cleanup_reading_is_starved(zero_progress(), cx.should_yield()));
            assert!(contended, "a live pass past its deadline reads nothing");
            assert_eq!(runtime_live_cleanup_nonterminal_status(contended, zero_progress(), &stalled, &since, Some(1 + step * 1_500), 0, &std::sync::atomic::AtomicU64::new(0)), (RuntimeMaintenanceStatus::Ready, 0));
        }
        assert_eq!((stalled.load(Ordering::SeqCst), since.load(Ordering::SeqCst)), (0, 0));
        let mut verdict = RuntimeMaintenanceStatus::Ready;
        for step in 0..u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT) {
            let contended = starved_clock_cx(|| Some(1), |cx| runtime_cleanup_reading_is_starved(zero_progress(), cx.should_yield()));
            assert!(!contended);
            verdict = runtime_live_cleanup_nonterminal_status(contended, zero_progress(), &stalled, &since, Some(1 + step * 1_500), 0, &std::sync::atomic::AtomicU64::new(0)).0;
        }
        assert_eq!(verdict, RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress));
    }

    /// 🪫️ LAW: a live pass whose retained item or depth stock the turn already spent reads an empty ladder (the worker session answers `Idle` when it cannot step), which is scheduling latency even with the clock inside the deadline. A pass holding its full stock is judged as before.
    #[test]
    fn live_passes_with_a_spent_item_or_depth_stock_are_starved_but_funded_passes_are_not() {
        use semio_plugin_reactor_lifecycle::life_cleanup_pumps::runtime_live_pass_budget_spent;
        let spent = |grant: RetainedCloneGrant| {
            let (mut sequence, mut receipt) = (0u64, RetainedCloneProgress::default());
            let cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(7), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(64, 8_000, grant), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut receipt);
            runtime_live_pass_budget_spent(&cx)
        };
        let full = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 65_536, maximum_release_bytes: 262_144, maximum_depth: 64 };
        assert!(!spent(full));
        assert!(spent(RetainedCloneGrant { maximum_items: 0, ..full }));
        assert!(spent(RetainedCloneGrant { maximum_depth: 0, ..full }));
        assert!(runtime_cleanup_reading_is_starved(zero_progress(), spent(RetainedCloneGrant { maximum_items: 0, ..full })));
    }
}
