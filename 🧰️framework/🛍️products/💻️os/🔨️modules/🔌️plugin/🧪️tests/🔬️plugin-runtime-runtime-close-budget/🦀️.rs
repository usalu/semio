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
                    _ => output.preparation_receipt = Some((77, grant, receipt)),
                }
                let (stopped, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| TypedOperationGrant::UNIT.spent(&output));
                assert!(stopped);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                match owner {
                    "history" => assert_eq!(output.history_command_receipt, Some((grant, receipt))),
                    "tool-run" => assert_eq!(output.tool_run_receipt, Some((grant, receipt))),
                    _ => assert_eq!(output.preparation_receipt, Some((77, grant, receipt))),
                }
            }
        }
    }

    fn stall_state() -> (AtomicU8, AtomicU64) {
        (AtomicU8::new(0), AtomicU64::new(0))
    }

    #[doc(hidden)]
pub fn zero_progress() -> Option<semio_plugin_ownership::bounded_ownership::PluginLifecycleStep> {
        Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress::default()))
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
        for progress in [semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { reason: "injected permanent external wait" }, semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput { reason: "typed operation awaits its exact host result ACK" }] {
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
            assert_eq!(runtime_close_stall_verdict(false, Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(receipt)), &stalled, &since, Some(1_000_000)), (RuntimeCloseStatus::Ready, 0));
            assert_eq!(stalled.load(Ordering::SeqCst), 0);
            assert_eq!(runtime_close_stall_verdict(false, zero_progress(), &stalled, &since, Some(1_000_000 + RUNTIME_CLOSE_STALL_CREDIT_US)).0, RuntimeCloseStatus::Ready);
        }
        let (stalled, since) = stall_state();
        assert_eq!(runtime_close_stall_verdict(false, Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })), &stalled, &since, Some(1_000_000)), (RuntimeCloseStatus::Ready, 0));
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

    fn live_stall_state() -> (AtomicU32, AtomicU64) {
        (AtomicU32::new(0), AtomicU64::new(0))
    }

    #[test]
    fn permanently_fruitless_live_cleanup_faults_after_its_stall_credit_without_claiming_released_ownership() {
        let (stalled, since) = live_stall_state();
        let blocked = Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        for step in 1..u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT) {
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, blocked, &stalled, &since, Some(1 + step), 0, &std::sync::atomic::AtomicU64::new(0)).0, RuntimeMaintenanceStatus::Ready);
        }
        let (status, credit_us) = runtime_live_cleanup_nonterminal_status(false, blocked, &stalled, &since, Some(2 + RUNTIME_LIVE_STALL_CREDIT_US), 0, &std::sync::atomic::AtomicU64::new(0));
        assert_eq!(status, RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress));
        assert!(credit_us >= RUNTIME_LIVE_STALL_CREDIT_US, "a live stall verdict must quote the credit it actually spent, not 0us");
        assert_eq!(stalled.load(Ordering::SeqCst), RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT);
    }

    /// 🛑️ LAW: a cancelled live cleanup outcome carries no reading, so it clears the stall run instead of feeding it; a pump that restarts a cancelled session every turn cannot reach `ZeroProgress`.
    #[test]
    fn cancelled_live_cleanup_outcomes_clear_the_stall_run_and_never_fault() {
        let (stalled, since) = live_stall_state();
        let blocked = Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        for step in 1..u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT) {
            runtime_live_cleanup_nonterminal_status(false, blocked, &stalled, &since, Some(1 + step), 0, &std::sync::atomic::AtomicU64::new(0));
        }
        for turn in 0..4 * u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT) {
            let now = 10 * RUNTIME_CLOSE_STALL_CREDIT_US * (turn + 1);
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, None, &stalled, &since, Some(now), 0, &std::sync::atomic::AtomicU64::new(0)), (RuntimeMaintenanceStatus::Ready, 0));
            assert_eq!((stalled.load(Ordering::SeqCst), since.load(Ordering::SeqCst)), (0, 0));
        }
    }

    /// 🛬️ LAW (boot): a live cleanup Blocked behind a whole-document load that keeps advancing its instance epoch never faults, however many turns and however much wall time pass; the same cleanup with a frozen instance epoch faults after its stall credit.
    #[test]
    fn live_cleanup_fruitless_behind_a_progressing_load_never_faults_but_a_frozen_instance_does() {
        let blocked = Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        let (stalled, since) = live_stall_state();
        let seen = AtomicU64::new(0);
        for turn in 1..=4 * u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT) {
            let now = 1 + turn * RUNTIME_CLOSE_STALL_CREDIT_US;
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, blocked, &stalled, &since, Some(now), turn, &seen).0, RuntimeMaintenanceStatus::Ready);
        }
        let frozen = 4 * u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT);
        let base = 1_000_000_000;
        let mut verdict = RuntimeMaintenanceStatus::Ready;
        for step in 0..2 * u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT) {
            verdict = runtime_live_cleanup_nonterminal_status(false, blocked, &stalled, &since, Some(base + step * RUNTIME_LIVE_STALL_CREDIT_US / 64), frozen, &seen).0;
        }
        assert_eq!(verdict, RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress));
    }

    /// 🔎️ LAW: a live zero-progress fault names the last maintenance reading and the last progressing owner, so a browser console line says what the cleanup waited behind.
    #[test]
    fn a_live_zero_progress_fault_names_its_blocking_reading_and_last_owner() {
        let fault = semio_plugin_reactor_lifecycle::life_cleanup_vocabulary::runtime_cleanup_fault_with_reading("live", RuntimeCleanupFault::ZeroProgress, 1, 64_000, "blocked: snapshot retirement waits for its exact live media owner; last progressing owner: host-crossing");
        assert!(fault.to_string().contains("(last reading: blocked: snapshot retirement waits for its exact live media owner; last progressing owner: host-crossing)"), "{fault}");
    }

    /// ⏱️ LAW: the live pump's zero-progress verdict is priced in BOTH currencies, like the close ladder's. An ingress-free reactor turn repeats the live pump up to 32 times while a funded document archive is runnable, so a booting natural-file import read 256 fruitless steps inside the same microsecond and faulted with `plugin.internal.zero-progress (elapsed 0us, ceiling 8000us)`. Reproduced with a virtual clock.
    #[test]
    fn live_zero_progress_cannot_fault_before_its_stall_credit_elapses() {
        let zero = Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        for progress in [zero] {
            let (stalled, since) = live_stall_state();
            for _ in 0..4 * RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT {
                assert_eq!(runtime_live_cleanup_nonterminal_status(false, progress, &stalled, &since, Some(1), 0, &std::sync::atomic::AtomicU64::new(0)).0, RuntimeMaintenanceStatus::Ready);
            }
            assert!(stalled.load(Ordering::SeqCst) >= RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT);
        }
    }

    #[test]
    fn live_zero_progress_without_a_clock_never_faults() {
        let (stalled, since) = live_stall_state();
        let zero = Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        for _ in 0..4 * RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT {
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, zero, &stalled, &since, None, 0, &std::sync::atomic::AtomicU64::new(0)), (RuntimeMaintenanceStatus::Ready, 0));
        }
        assert_eq!(since.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn exact_host_input_wait_resets_structural_live_cleanup_stall_credit() {
        let stalled = AtomicU32::new(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT - 1);
        let since = AtomicU64::new(1);
        let awaiting_input = Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput { reason: "typed operation awaits its exact host result ACK" });
        for tick in 0..1_024 {
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, awaiting_input, &stalled, &since, Some(tick * RUNTIME_CLOSE_STALL_CREDIT_US), 0, &std::sync::atomic::AtomicU64::new(0)), (RuntimeMaintenanceStatus::Ready, 0));
        }
        assert_eq!(stalled.load(Ordering::SeqCst), 0);
        assert_eq!(since.load(Ordering::SeqCst), 0);
    }

    fn fixture_step(name: &str) -> Option<semio_plugin_ownership::bounded_ownership::PluginLifecycleStep> {
        use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep as Step;
        match name {
            "empty" => Some(Step::Progress(RetainedCloneProgress::default())),
            "progress" => Some(Step::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })),
            "complete" => Some(Step::Complete(RetainedCloneProgress::default())),
            "blocked" => Some(Step::Blocked { reason: "waits for another owner" }),
            "awaiting" => Some(Step::AwaitingInput { reason: "waits for its original host" }),
            _ => None,
        }
    }

    /// 🩻️ LAW (vectors `🩻️live-stall-reading`): the live reading is judged by its cause. Starved readings buy no debt, waiting on an external owner (a stage 14 or 24 archive or replacement phase, `Blocked`) never faults however many readings and microseconds pass, and only a funded fruitless step faults after the limit and credit.
    #[test]
    fn live_readings_are_judged_by_their_cause_over_language_agnostic_vectors() {
        use semio_plugin_reactor_lifecycle::life_cleanup_pumps::{RuntimeMaintenanceReading as Reading, runtime_maintenance_reading};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🩻️live-stall-reading/🔣️.json")).unwrap();
        assert_eq!(fixture["zeroProgressLimit"].as_u64().unwrap(), u64::from(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT));
        for case in fixture["cases"].as_array().unwrap() {
            let expected = match case["reading"].as_str().unwrap() { "fruitless" => Reading::Fruitless, "starved" => Reading::Starved, "waiting" => Reading::Waiting, _ => Reading::Progress };
            assert_eq!(runtime_maintenance_reading(case["starved"].as_bool().unwrap(), fixture_step(case["step"].as_str().unwrap())), expected, "{case}");
        }
        for sequence in fixture["sequences"].as_array().unwrap() {
            let (stalled, since, seen) = (AtomicU32::new(0), AtomicU64::new(0), AtomicU64::new(0));
            let (count, span) = (sequence["count"].as_u64().unwrap(), sequence["spanUs"].as_u64().unwrap());
            let mut faulted = false;
            for index in 0..count {
                let verdict = runtime_live_cleanup_nonterminal_status(sequence["starved"].as_bool().unwrap(), fixture_step(sequence["step"].as_str().unwrap()), &stalled, &since, Some(1 + index * span / count), 0, &seen);
                faulted |= verdict.0 == RuntimeMaintenanceStatus::Fault(RuntimeCleanupFault::ZeroProgress);
            }
            assert_eq!(faulted, sequence["faults"].as_bool().unwrap(), "{sequence}");
        }
    }

    #[test]
    fn contended_live_cleanup_does_not_consume_structural_stall_credit() {
        let (stalled, since) = live_stall_state();
        for tick in 0..1_024 {
            assert_eq!(runtime_live_cleanup_nonterminal_status(true, None, &stalled, &since, Some(tick), 0, &std::sync::atomic::AtomicU64::new(0)), (RuntimeMaintenanceStatus::Ready, 0));
        }
        assert_eq!(stalled.load(Ordering::SeqCst), 0);
        assert_eq!(since.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn live_maintenance_caller_funds_exact_retained_physical_extent_without_more_work() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/📏️live-physical-demand/🔣️.json")).unwrap();
        let original = RetainedCloneGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: fixture["ordinaryGrantBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: fixture["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: fixture["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: fixture["maximumDepth"].as_u64().unwrap() as usize };
        let policy = semio_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: original, maintenance: original, close: original }.validate().unwrap().maintenance;
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
                assert!(matches!(result, Ok(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress)) if progress == RetainedCloneProgress::default()));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.as_ref().unwrap().as_ptr(), pointer);
            }
            let mut called = 0;
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| runtime_lifecycle_step(demand, grant, |actual| {
                called += 1;
                assert_eq!(actual, grant);
                drop(owner.take());
                Ok((semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..Default::default() }), owner.is_none()))
            }));
            assert_eq!(called, fixture["maximumItems"].as_u64().unwrap() as usize);
            assert!(owner.is_none());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
            assert!(matches!(result, Ok(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress)) if progress.released_bytes == extent && progress.copied_bytes == 0 && progress.fits(grant)));
        }
        let extent = fixture["refusedAllocationBytes"].as_u64().unwrap() as usize;
        let mut owner = Some(Vec::<u8>::with_capacity(extent));
        let pointer = owner.as_ref().unwrap().as_ptr();
        let mut work = 0;
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| runtime_lifecycle_step(RetirementDemand { release_bytes: extent, depth: 1, ..Default::default() }, grant, |_| {
            work += 1;
            unreachable!("fixed original release grant was enlarged")
        }));
        assert!(matches!(result, Ok(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress)) if progress == RetainedCloneProgress::default()));
        assert_eq!(owner.as_ref().unwrap().as_ptr(), pointer);
        assert_eq!(work, fixture["refusedWorkItems"].as_u64().unwrap() as usize);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let actual = RetainedCloneGrant { maximum_release_bytes: extent, ..grant };
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| runtime_lifecycle_step(RetirementDemand { release_bytes: extent, depth: 1, ..Default::default() }, actual, |received| {
            assert_eq!(received, actual);
            drop(owner.take());
            Ok((semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..Default::default() }), owner.is_none()))
        }));
        assert!(matches!(result, Ok(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress)) if progress.released_bytes == extent));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, extent));
    }

    #[test]
    fn every_actual_progress_currency_resets_close_and_live_stall_credit() {
        for receipt in [RetainedCloneProgress { copied_items: 1, ..Default::default() }, RetainedCloneProgress { copied_bytes: 1, ..Default::default() }, RetainedCloneProgress { retained_capacity_bytes: 1, ..Default::default() }, RetainedCloneProgress { released_bytes: 1, ..Default::default() }] {
            let (stalled, since) = (AtomicU8::new(RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT), AtomicU64::new(1));
            let progress = Some(semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(receipt));
            assert_eq!(runtime_close_stall_verdict(false, progress, &stalled, &since, Some(1_000_000)), (RuntimeCloseStatus::Ready, 0));
            assert_eq!(stalled.load(Ordering::SeqCst), 0);
            assert_eq!(since.load(Ordering::SeqCst), 0);
            let (live, live_since) = (AtomicU32::new(RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT), AtomicU64::new(1));
            assert_eq!(runtime_live_cleanup_nonterminal_status(false, progress, &live, &live_since, Some(1_000_000), 0, &std::sync::atomic::AtomicU64::new(0)), (RuntimeMaintenanceStatus::Ready, 0));
            assert_eq!(live.load(Ordering::SeqCst), 0);
            assert_eq!(live_since.load(Ordering::SeqCst), 0);
        }
    }
}
