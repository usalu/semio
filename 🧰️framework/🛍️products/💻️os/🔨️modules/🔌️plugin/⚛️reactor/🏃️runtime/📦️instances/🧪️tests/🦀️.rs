//! 📦️ Unit tests of runtime-instances; the work package moves the tests of its sources here.

#[cfg(test)]
mod close_stall_laws {
    use crate::inst_cleanup_turns::runtime_close_stall_verdict;
    use semio_plugin_reactor_lifecycle::life_cleanup_pumps::runtime_close_reading_is_starved;
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
            let contended = starved_clock_cx(|| Some(10_500), |cx| runtime_close_reading_is_starved(zero_progress(), cx.retirement_should_yield()));
            assert!(contended, "a step past its deadline reads nothing");
            assert_eq!(runtime_close_stall_verdict(contended, zero_progress(), &stalled, &since, Some(1 + step * 1_500)), (RuntimeCloseStatus::Ready, 0));
        }
        assert_eq!((stalled.load(Ordering::SeqCst), since.load(Ordering::SeqCst)), (0, 0));
    }

    /// 🌫️ LAW: only a step that still held its budget when it read an empty ladder is evidence; a wedged ladder polled with full budget still faults,
    /// and a starved reading that carries real progress or an external wait is never reclassified.
    #[test]
    fn full_budget_empty_close_steps_still_fault_and_other_readings_are_never_starved() {
        let (stalled, since) = stall_state();
        let mut verdict = (RuntimeCloseStatus::Ready, 0);
        for step in 0..u64::from(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT) {
            let contended = starved_clock_cx(|| Some(1), |cx| runtime_close_reading_is_starved(zero_progress(), cx.retirement_should_yield()));
            assert!(!contended);
            verdict = runtime_close_stall_verdict(contended, zero_progress(), &stalled, &since, Some(1 + step * RUNTIME_CLOSE_STALL_CREDIT_US));
        }
        assert_eq!(verdict.0, RuntimeCloseStatus::Fault(RuntimeCleanupFault::ZeroProgress));
        for reading in [None, Some(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })), Some(PluginLifecycleStep::Blocked { reason: "external" }), Some(PluginLifecycleStep::AwaitingInput { reason: "ack" })] {
            assert!(!runtime_close_reading_is_starved(reading, true));
        }
    }

}
