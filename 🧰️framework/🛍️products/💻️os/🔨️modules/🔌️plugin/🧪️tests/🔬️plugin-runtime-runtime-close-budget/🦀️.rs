mod runtime_close_budget_tests {
    use super::*;

    #[test]
    fn every_original_ownership_receipt_stops_the_typed_continuation_before_another_unit() {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 65_536, maximum_release_bytes: 262_144, maximum_depth: 64 };
        for receipt in [RetainedCloneProgress::default(), RetainedCloneProgress { copied_items: 1, copied_bytes: 31, retained_capacity_bytes: 64, released_bytes: 128 }] {
            assert!(receipt.fits(grant));
            for owner in ["preparation", "history", "tool-run"] {
                let mut output = PluginExchangeOutput::default();
                assert!(!TypedOperationGrant::UNIT.spent(&output));
                match owner {
                    "history" => output.history_command_receipt = Some((grant, receipt)),
                    "tool-run" => output.tool_run_receipt = Some((grant, receipt)),
                    _ => output.preparation_receipt = Some((semio_framework_job::OperationId(77), grant, receipt)),
                }
                let (stopped, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| TypedOperationGrant::UNIT.spent(&output));
                assert!(stopped);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                match owner {
                    "history" => assert_eq!(output.history_command_receipt, Some((grant, receipt))),
                    "tool-run" => assert_eq!(output.tool_run_receipt, Some((grant, receipt))),
                    _ => assert_eq!(output.preparation_receipt, Some((semio_framework_job::OperationId(77), grant, receipt))),
                }
                eprintln!("[DEBUG] actual typed continuation owner={owner} originalGrant={grant:?} receipt={receipt:?} stopped={stopped} pureHeap={heap:?}");
            }
        }
    }

    fn stall_state() -> (AtomicU8, AtomicU64) {
        (AtomicU8::new(0), AtomicU64::new(0))
    }

    fn zero_progress() -> Option<crate::app::PluginLifecycleStep> {
        Some(crate::app::PluginLifecycleStep::Progress(RetainedCloneProgress::default()))
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
        for progress in [crate::app::PluginLifecycleStep::Blocked { reason: "injected permanent external wait" }, crate::app::PluginLifecycleStep::AwaitingInput { reason: "typed operation awaits its exact host result ACK" }] {
            let stalled = AtomicU8::new(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT - 1);
            let since = AtomicU64::new(1);
            for tick in 0..1_024 {
                assert_eq!(runtime_close_stall_verdict(false, Some(progress), &stalled, &since, Some(tick * RUNTIME_CLOSE_STALL_CREDIT_US)), (RuntimeCloseStatus::ExternalWait, 0));
            }
            assert_eq!(stalled.load(Ordering::SeqCst), 0);
            assert_eq!(since.load(Ordering::SeqCst), 0);
        }
    }

    /// 🚪️ LAW: real retirement on ANY independent currency resets the stall clock — one handed-off item,
    /// copied byte, retained capacity byte or released byte after a long fruitless run puts the ladder
    /// back on full credit, so a slow-but-moving close can never be killed.
    #[test]
    fn retired_ownership_on_any_axis_resets_the_structural_stall_clock() {
        let moved = [
            RetainedCloneProgress { copied_items: 1, ..Default::default() },
            RetainedCloneProgress { copied_bytes: 1, ..Default::default() },
            RetainedCloneProgress { retained_capacity_bytes: 1, ..Default::default() },
            RetainedCloneProgress { released_bytes: 1, ..Default::default() },
        ];
        for receipt in moved {
            let (stalled, since) = stall_state();
            for step in 0..u64::from(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT) {
                let _ = runtime_close_stall_verdict(false, zero_progress(), &stalled, &since, Some(1 + step));
            }
            assert_eq!(runtime_close_stall_verdict(false, Some(crate::app::PluginLifecycleStep::Progress(receipt)), &stalled, &since, Some(1_000_000)), (RuntimeCloseStatus::Ready, 0));
            assert_eq!(stalled.load(Ordering::SeqCst), 0);
            assert_eq!(runtime_close_stall_verdict(false, zero_progress(), &stalled, &since, Some(1_000_000 + RUNTIME_CLOSE_STALL_CREDIT_US)).0, RuntimeCloseStatus::Ready);
        }
        assert_eq!(runtime_close_stall_verdict(false, Some(crate::app::PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })), &stalled, &since, Some(1_000_000)), (RuntimeCloseStatus::Ready, 0));
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
        let blocked = Some(crate::app::PluginLifecycleStep::Blocked { reason: "injected permanent external wait" });
        for _ in 1..RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT {
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, blocked, &stalled), RuntimeMaintenanceStatus::Ready);
        }
        assert_eq!(runtime_live_cleanup_nonterminal_status(false, blocked, &stalled), RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress));
        assert_eq!(stalled.load(Ordering::SeqCst), RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT);
    }

    #[test]
    fn exact_host_input_wait_resets_structural_live_cleanup_stall_credit() {
        let stalled = AtomicU32::new(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT - 1);
        let awaiting_input = Some(crate::app::PluginLifecycleStep::AwaitingInput { reason: "typed operation awaits its exact host result ACK" });
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

    #[test]
    fn live_maintenance_caller_funds_exact_retained_physical_extent_without_more_work() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/📏️live-physical-demand/🔣️.json")).unwrap();
        let original = RetainedCloneGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: fixture["ordinaryGrantBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: fixture["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: fixture["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: fixture["maximumDepth"].as_u64().unwrap() as usize };
        let policy = crate::MountedOwnerPolicyV1 { preparation: original, maintenance: original, close: original }.validate().unwrap().maintenance;
        assert_eq!(fixture["ordinaryGrantBytes"].as_u64().unwrap() as usize, policy.maximum_copy_bytes);
        assert_eq!(fixture["maximumCapacityBytes"].as_u64().unwrap() as usize, policy.maximum_capacity_bytes);
        assert_eq!(fixture["maximumReleaseBytes"].as_u64().unwrap() as usize, policy.maximum_release_bytes);
        assert_eq!(fixture["maximumDepth"].as_u64().unwrap() as usize, policy.maximum_depth);
        let grant = RetainedCloneGrant { maximum_release_bytes: fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize, ..policy };
        for extent in fixture["allocationBytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
            let mut owner = Some(Vec::<u8>::with_capacity(extent));
            assert_eq!(owner.as_ref().unwrap().capacity(), extent);
            let pointer = owner.as_ref().unwrap().as_ptr();
            let demand = RetirementDemand { release_bytes: extent, depth: 1, ..Default::default() };
            for under in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: extent - 1, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
                let mut called = 0;
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| runtime_lifecycle_step(demand, under, |_| {
                    called += 1;
                    unreachable!("undergrant called the original lifecycle owner")
                }));
                assert_eq!(called, 0);
                assert!(matches!(result, Ok(crate::app::PluginLifecycleStep::Progress(progress)) if progress == RetainedCloneProgress::default()));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.as_ref().unwrap().as_ptr(), pointer);
            }
            let mut called = 0;
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| runtime_lifecycle_step(demand, grant, |actual| {
                called += 1;
                assert_eq!(actual, grant);
                drop(owner.take());
                Ok((crate::app::PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..Default::default() }), owner.is_none()))
            }));
            assert_eq!(called, fixture["maximumItems"].as_u64().unwrap() as usize);
            assert!(owner.is_none());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
            assert!(matches!(result, Ok(crate::app::PluginLifecycleStep::Complete(progress)) if progress.released_bytes == extent && progress.copied_bytes == 0 && progress.fits(grant)));
            eprintln!("[DEBUG] actual runtime lifecycle originalExtent={extent} copyGrant={} capacityGrant={} releaseGrant={} called={called} heap={:?} terminalDrop=0", grant.maximum_copy_bytes, grant.maximum_capacity_bytes, grant.maximum_release_bytes, heap);
        }
        let extent = fixture["refusedAllocationBytes"].as_u64().unwrap() as usize;
        let mut owner = Some(Vec::<u8>::with_capacity(extent));
        let pointer = owner.as_ref().unwrap().as_ptr();
        let mut work = 0;
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| runtime_lifecycle_step(RetirementDemand { release_bytes: extent, depth: 1, ..Default::default() }, grant, |_| {
            work += 1;
            unreachable!("fixed original release grant was enlarged")
        }));
        assert!(matches!(result, Ok(crate::app::PluginLifecycleStep::Progress(progress)) if progress == RetainedCloneProgress::default()));
        assert_eq!(owner.as_ref().unwrap().as_ptr(), pointer);
        assert_eq!(work, fixture["refusedWorkItems"].as_u64().unwrap() as usize);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let actual = RetainedCloneGrant { maximum_release_bytes: extent, ..grant };
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| runtime_lifecycle_step(RetirementDemand { release_bytes: extent, depth: 1, ..Default::default() }, actual, |received| {
            assert_eq!(received, actual);
            drop(owner.take());
            Ok((crate::app::PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..Default::default() }), owner.is_none()))
        }));
        assert!(matches!(result, Ok(crate::app::PluginLifecycleStep::Complete(progress)) if progress.released_bytes == extent));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
        eprintln!("[DEBUG] actual runtime lifecycle one-below release originalPointer retained0heap; independently admitted exact release={extent} physical={}", heap.released_bytes);
    }

    #[test]
    fn every_actual_progress_currency_resets_close_and_live_stall_credit() {
        for receipt in [RetainedCloneProgress { copied_items: 1, ..Default::default() }, RetainedCloneProgress { copied_bytes: 1, ..Default::default() }, RetainedCloneProgress { retained_capacity_bytes: 1, ..Default::default() }, RetainedCloneProgress { released_bytes: 1, ..Default::default() }] {
            let (stalled, since) = (AtomicU8::new(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT), AtomicU64::new(1));
            let progress = Some(crate::app::PluginLifecycleStep::Progress(receipt));
            assert_eq!(runtime_close_stall_verdict(false, progress, &stalled, &since, Some(1_000_000)), (RuntimeCloseStatus::Ready, 0));
            assert_eq!(stalled.load(Ordering::SeqCst), 0);
            assert_eq!(since.load(Ordering::SeqCst), 0);
            let live = AtomicU32::new(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT);
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, progress, &live), RuntimeMaintenanceStatus::Ready);
            assert_eq!(live.load(Ordering::SeqCst), 0);
        }
        eprintln!("[DEBUG] actual runtime all4 receipt currencies independently reset structural close/live stall credit");
    }
}
