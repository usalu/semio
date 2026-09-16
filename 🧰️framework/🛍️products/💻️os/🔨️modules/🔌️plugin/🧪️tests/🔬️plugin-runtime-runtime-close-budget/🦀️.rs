mod runtime_close_budget_tests {
    use super::*;

    fn stall_state() -> (AtomicU8, AtomicU64) {
        (AtomicU8::new(0), AtomicU64::new(0))
    }

    fn zero_progress() -> Option<crate::app::PluginCloseStep> {
        Some(crate::app::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 })
    }

    #[test]
    fn repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit() {
        let (stalled, since) = stall_state();
        for tick in 0..1_024 {
            assert_eq!(runtime_close_stall_verdict(true, None, &stalled, &since, Some(tick)), (RuntimeCloseStatus::Ready, 0));
        }
        assert_eq!(stalled.load(Ordering::SeqCst), 0);
        assert_eq!(since.load(Ordering::SeqCst), 0);
    }

    /// ⏱️ LAW: the zero-progress verdict is priced in BOTH currencies it claims — consecutive fruitless
    /// steps AND real elapsed microseconds. Before 2026-09-16 the step count was the whole rule, so a
    /// close ladder that answered `Pending { 0, 0 }` eight times within the same microsecond faulted
    /// with `plugin.internal.zero-progress` and the message `(elapsed 0us, ceiling 8000us)` — a stall
    /// credit reported as exhausted before any time had passed. Reproduced here with a virtual clock.
    #[test]
    fn structural_zero_progress_cannot_fault_before_its_stall_credit_elapses() {
        let (stalled, since) = stall_state();
        for _ in 0..1_024 {
            assert_eq!(runtime_close_stall_verdict(false, zero_progress(), &stalled, &since, Some(1)), (RuntimeCloseStatus::Ready, 0));
        }
        assert!(stalled.load(Ordering::SeqCst) >= RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT);
    }

    #[test]
    fn structural_zero_progress_exhausts_its_exact_close_credit() {
        let (stalled, since) = stall_state();
        for step in 1..u64::from(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT) {
            assert_eq!(runtime_close_stall_verdict(false, zero_progress(), &stalled, &since, Some(1 + step)).0, RuntimeCloseStatus::Ready);
        }
        let (status, credit_us) = runtime_close_stall_verdict(false, zero_progress(), &stalled, &since, Some(2 + RUNTIME_CLOSE_STALL_CREDIT_US));
        assert_eq!(status, RuntimeCloseStatus::Fault(RuntimeCleanupFault::ZeroProgress));
        assert!(credit_us >= RUNTIME_CLOSE_STALL_CREDIT_US, "a stall verdict must quote the credit it actually spent, not 0us");
    }

    /// 🚪️ LAW: a step that recorded NO ladder reading is an absence of evidence, not evidence of a
    /// stall — a cancelled cleanup turn, or a close with nothing left to retire at all, must never buy
    /// structural stall debt. A cancelled or faulted outcome is terminal through `PriorOutcome`.
    #[test]
    fn unobserved_close_step_never_buys_structural_stall_credit() {
        let (stalled, since) = stall_state();
        for tick in 0..1_024 {
            assert_eq!(runtime_close_stall_verdict(false, None, &stalled, &since, Some(tick * RUNTIME_CLOSE_STALL_CREDIT_US)), (RuntimeCloseStatus::Ready, 0));
        }
        assert_eq!(stalled.load(Ordering::SeqCst), 0);
        assert_eq!(since.load(Ordering::SeqCst), 0);
    }

    /// ⏳️ LAW: a pending close authority is a WAIT, never a fault — the close ladder keeps stepping
    /// until its external owner lets go, and the wait never consumes structural livelock credit.
    /// Before 2026-09-10 the close job turned `Blocked` into `StepOutcome::Fault`, which surfaced as
    /// `plugin.reactor-close-authority: native close terminal unavailable` and left generation3d
    /// unable to ever reach `Retired`.
    #[test]
    fn pending_close_authority_waits_without_consuming_structural_close_credit() {
        for progress in [crate::app::PluginCloseStep::Blocked { reason: "injected permanent external wait" }, crate::app::PluginCloseStep::AwaitingInput { reason: "typed operation awaits its exact host result ACK" }] {
            let stalled = AtomicU8::new(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT - 1);
            let since = AtomicU64::new(1);
            for tick in 0..1_024 {
                assert_eq!(runtime_close_stall_verdict(false, Some(progress), &stalled, &since, Some(tick * RUNTIME_CLOSE_STALL_CREDIT_US)), (RuntimeCloseStatus::ExternalWait, 0));
            }
            assert_eq!(stalled.load(Ordering::SeqCst), 0);
            assert_eq!(since.load(Ordering::SeqCst), 0);
        }
    }

    /// 🚪️ LAW: real retirement resets the stall clock — one released item after a long fruitless run
    /// puts the ladder back on full credit, so a slow-but-moving close can never be killed.
    #[test]
    fn released_ownership_resets_the_structural_stall_clock() {
        let (stalled, since) = stall_state();
        for step in 0..u64::from(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT) {
            let _ = runtime_close_stall_verdict(false, zero_progress(), &stalled, &since, Some(1 + step));
        }
        assert_eq!(runtime_close_stall_verdict(false, Some(crate::app::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }), &stalled, &since, Some(1_000_000)), (RuntimeCloseStatus::Ready, 0));
        assert_eq!(stalled.load(Ordering::SeqCst), 0);
        assert_eq!(runtime_close_stall_verdict(false, zero_progress(), &stalled, &since, Some(1_000_000 + RUNTIME_CLOSE_STALL_CREDIT_US)).0, RuntimeCloseStatus::Ready);
    }

    /// 🚪️ LAW: every cleanup-taxonomy verdict names ONE retired lifetime, so the reactor turn every
    /// other instance of the same actor shares may contain it instead of dying with it. Before
    /// 2026-09-16 the turn propagated it with `?`, the host trapped the actor, and a sibling pane
    /// booting in the same shard went to Plugin Recovery with the instance that had closed.
    #[test]
    fn every_cleanup_verdict_is_instance_scoped_and_nothing_else_is() {
        for cause in RUNTIME_CLEANUP_FAULTS {
            assert!(runtime_cleanup_fault_is_instance_scoped(cause.vector().code), "{} must be containable", cause.vector().code);
        }
        for code in ["plugin.internal", "plugin.reactor-turn-deadline", "plugin.internal.zero", ""] {
            assert!(!runtime_cleanup_fault_is_instance_scoped(code));
        }
    }

    #[test]
    fn permanently_blocked_live_cleanup_faults_without_claiming_released_ownership() {
        let stalled = AtomicU32::new(0);
        let blocked = Some(crate::app::PluginCloseStep::Blocked { reason: "injected permanent external wait" });
        for _ in 1..RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT {
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, blocked, &stalled), RuntimeMaintenanceStatus::Ready);
        }
        assert_eq!(runtime_live_cleanup_nonterminal_status(false, blocked, &stalled), RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress));
        assert_eq!(stalled.load(Ordering::SeqCst), RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT);
    }

    #[test]
    fn exact_host_input_wait_resets_structural_live_cleanup_stall_credit() {
        let stalled = AtomicU32::new(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT - 1);
        let awaiting_input = Some(crate::app::PluginCloseStep::AwaitingInput { reason: "typed operation awaits its exact host result ACK" });
        for _ in 0..1_024 {
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, awaiting_input, &stalled), RuntimeMaintenanceStatus::Ready);
        }
        assert_eq!(stalled.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn contended_live_cleanup_does_not_consume_structural_stall_credit() {
        let stalled = AtomicU32::new(0);
        for _ in 0..1_024 {
            assert_eq!(runtime_live_cleanup_nonterminal_status(true, None, &stalled), RuntimeMaintenanceStatus::Ready);
        }
        assert_eq!(stalled.load(Ordering::SeqCst), 0);
    }
}
