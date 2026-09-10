mod runtime_close_budget_tests {
    use super::*;

    #[test]
    fn repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit() {
        let stalled = AtomicU8::new(0);
        for _ in 0..1_024 {
            assert_eq!(runtime_close_nonterminal_status(true, None, &stalled), RuntimeCloseStatus::Ready);
        }
        assert_eq!(stalled.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn structural_zero_progress_exhausts_its_exact_close_credit() {
        let stalled = AtomicU8::new(0);
        for _ in 1..RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT {
            assert_eq!(runtime_close_nonterminal_status(false, Some(crate::app::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }), &stalled), RuntimeCloseStatus::Ready);
        }
        assert_eq!(runtime_close_nonterminal_status(false, Some(crate::app::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }), &stalled), RuntimeCloseStatus::Fault(RuntimeCleanupFault::ZeroProgress));
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
            for _ in 0..1_024 {
                assert_eq!(runtime_close_nonterminal_status(false, Some(progress), &stalled), RuntimeCloseStatus::ExternalWait);
            }
            assert_eq!(stalled.load(Ordering::SeqCst), 0);
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
