//! 🪜️ Entry, lifecycle focus and retirement phases of the reactor turn: the borrowed retained input admission, the one lifecycle command per turn and the bounded close ladders that run before the turn's event. See [`super`].

use super::*;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;

/// 🚪️ Admits the borrowed retained input of one poll: it must name the context's own operation and generation, the context's remaining grant (reduced by this turn's origin refresh and its return stock) must lie inside the input grant, and no refused publication may still own its output.
pub(super) fn admit_turn_entry(budget: semio_framework::kernel::Budget, cx: &semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    budget.retained.validate()?;
    let input = budget.retained.grant;
    let held = cx.retained_grant();
    let inside = held.maximum_items <= input.maximum_items && held.maximum_copy_bytes <= input.maximum_copy_bytes && held.maximum_capacity_bytes <= input.maximum_capacity_bytes && held.maximum_release_bytes <= input.maximum_release_bytes && held.maximum_depth <= input.maximum_depth;
    if budget.retained.operation != cx.operation().0 || budget.retained.generation != cx.generation().0 || !inside {
        return Err(reactor_close_fault("original reactor retained input does not match its borrowed context").into());
    }
    if REFUSED_PUBLICATION.with(|owner|owner.borrow().is_some()){return Err(reactor_close_fault("original refused publication still owns its output and receipts").into());}
    trace_turn_phase_retention("enter");
    Ok(())
}

/// 🎯️ Admits the one lifecycle command this turn serves (close request, acknowledgement staging, admission advance) and returns the instance whose lifecycle the turn advances.
pub(super) fn admit_lifecycle_focus<PA: PluginApp>(runtime: &PluginRuntime<PA>, events: &Option<Event>) -> Result<Option<u32>, semio_framework_job::OriginalTurnFailure> {
    let mut focus = None;
    for event in events.iter() {
        let instance = match event {
            Event::InstanceOpen { request, .. } => Some(request.instance_id),
            Event::InstanceClose(request) => Some(request.lifetime.instance_id),
            Event::InstanceLifecycleAck(ack) => Some(match ack.receipt {
                ActorInstanceLifecycleReceipt::Captured { lifetime, .. } | ActorInstanceLifecycleReceipt::Accepted { lifetime, .. } | ActorInstanceLifecycleReceipt::Retired { lifetime, .. } => lifetime.instance_id,
            }),
            _ => None,
        };
        if let Some(instance) = instance {
            if focus.replace(instance).is_some() {
                return Err(reactor_close_fault("one lifecycle command is admitted per turn").into());
            }
        }
    }
    for event in events.iter() {
        match event {
            Event::InstanceClose(request) => {
                let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
                let slot = lifetimes.get_mut(request.lifetime.instance_id).ok_or_else(|| reactor_close_fault("close lifetime absent"))?;
                slot.cell.validate_close(*request).map_err(reactor_close_fault)?;
                slot.cell.owner_mut().ok_or_else(|| reactor_close_fault("close native owner absent"))?.request_close(*request, runtime, &ReactorClosePeers)?;
            }
            Event::InstanceLifecycleAck(ack) => {
                let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
                lifetimes.get_mut(focus.expect("ACK focus")).ok_or_else(|| reactor_close_fault("ACK lifetime absent"))?.cell.stage_ack(*ack).map_err(reactor_close_fault)?;
            }
            _ => {}
        }
    }
    if focus.is_none() {
        focus = runtime.guest_lifetimes.borrow_mut().next_work();
    }
    if let Some(instance) = focus {
        let mut lifetimes = runtime.guest_lifetimes.borrow_mut();
        if let Some(slot) = lifetimes.get_mut(instance) {
            if let Some(admission) = slot.cell.owner_mut().map(|owner| owner.advance_admission(runtime, &ReactorClosePeers)).transpose()?.flatten() {
                slot.cell.record_close_admission(admission.0, admission.1).map_err(reactor_close_fault)?;
            }
        }
    }
    Ok(focus)
}

/// 🧯️ Drives the bounded close ladders of one turn (reactor close, cold pair close, patch and document retirement, close cleanup, live cleanup pressure steps) and reports whether close cleanup still has work.
///
/// 🚪️ A close-cleanup verdict names ONE already-retired lifetime, so it may not kill the turn that
/// every OTHER instance in this actor shares.
///
/// 🐛️ It did: `?` here turned instance 5's `plugin.internal.zero-progress` into a shard worker fault
/// for the whole turn, the host trapped the actor, and instance 6 — a sibling pane mid-boot in the
/// same shard — went to Plugin Recovery with it (six-pane demonstrator boot, ticket 26/08/28). The
/// instance whose cleanup faulted is already gone and has already left quarantine; what is left is a
/// diagnostic, not a live window's crash.
///
/// 🌡️ One fair live-maintenance step per turn — and a burst while the stepped instance reports
/// displaced-owner pressure, bounded by the turn's retirement deadline. A results-window
/// playback (a coalesced window-config amend per frame, three displaced owners each) outran the
/// single step's one owner per 26-stage rotation and saturated its 1 024-slot queue after
/// ~340 frames (ticket 26/09/16/FEM-3D-INTERACTIVE-FEATURE-COMPLETE).
/// An ingress-free turn can also repeat the live pump for a funded document archive.
pub(super) fn drive_turn_retirement<PA: PluginApp>(runtime: &PluginRuntime<PA>, idle_ingress: bool, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    cx.with_original_retirement(|cx|{
    retire_while_progress_fallible(cx, REACTOR_CLOSE_UNITS_PER_TURN, step_reactor_close)?;
    retire_while_progress_bounded(cx, REACTOR_CLOSE_UNITS_PER_TURN, || COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().advance_close_one()));
    PATCHES.with(|patches| -> Result<(),semio_framework_job::OriginalTurnFailure> {
        for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
            if cx.retirement_should_yield() {
                break;
            }
            if patches.close_step(cx)? {
                break;
            }
        }
        Ok(())
    })?;
    for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if cx.retirement_should_yield(){break}
        cx.consume_fuel(1);
        if semio_ui_runtime::close_surface_reconcile_handback_one().map_err(reactor_close_fault)? {
            break;
        }
        if cx.retirement_should_yield() {
            break;
        }
    }
    retire_until_complete(cx, || ui_contract::close_ui_document_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("queued document retirement remains valid").complete);
    retire_until_complete(cx, semio_ui_contract_validation::close_ui_patch_owner_one);
    retire_until_complete(cx, || ui_contract::close_ui_value_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("exact UI value retirement queue remains valid").complete);
    retire_until_complete(cx, ui_contract::close_built_node_page_one);
    retire_while_progress(cx, || semio_framework::kernel::close_ui_turn_patch_owner_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT));
    retire_while_progress(cx, || {
        matches!(semio_framework::kernel::close_ui_turn_patch_transport_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT), Ok(semio_framework::kernel::UiTurnPatchTransportProgress::Pending { .. }))
    });
    with_pending_patches(|pending| pending.borrow_mut().advance_rejection(|surface, generation| PATCHES.with(|patches| patches.mark_rejected(surface, generation))));
    step_pending_patch_close(cx)?;
    let close_cleanup_work=semio_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(runtime,cx)?;
    use semio_plugin_runtime_instances::inst_cleanup_turns as cleanup;
    let archive_started = idle_ingress.then(guest_turn_executing_us).flatten();
    for pump in 0..cleanup::LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN {
        if cx.retirement_should_yield() { break; }
        if pump != 0 && !cleanup::plugin_live_cleanup_under_pressure(runtime) {
            if archive_started.is_none() || !archive_maintenance_slice_open(
                idle_ingress, cx.is_cancelled(), cx.should_yield(), archive_started, guest_turn_executing_us(),
                || semio_plugin_runtime_ops::ops_document::plugin_document_archive_maintenance_is_runnable(runtime, cx.retained_grant()),
            )? { break; }
        }
        cx.consume_fuel(1);
        if !cleanup::plugin_step_live_cleanup(runtime, cx)? { break; }
    }
    Ok(close_cleanup_work)
    })
}

/// 🩹️ Spends the turn's patch close units on the pending patch owner until one unit completes or the turn must yield.
///
/// 🧹️ The SECOND publication retirement of this turn, and the one that matters for the crossing
/// count: the first ran before the events, so every `Event::PatchAck` this turn carried left its
/// slot `Acknowledged` with nothing to retire it until the NEXT turn — one host round trip per
/// published surface whose only work was freeing a slot (the `<n>:ea---` half of the two-state
/// ping-pong, `📓️reactor-reconcile-spin-2026-09-14.md` §1). Bounded by the same units and the same
/// wall deadline as the first run, so a turn cut short still answers `MoreWork` through
/// `PendingPatchAuthority::has_retiring` and finishes on the next one.
pub(super) fn step_pending_patch_close(cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if cx.should_yield() {
            break;
        }
        if with_pending_patches(|pending| pending.borrow_mut().close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT)).map_err(reactor_close_fault)? {
            break;
        }
    }
    Ok(())
}
