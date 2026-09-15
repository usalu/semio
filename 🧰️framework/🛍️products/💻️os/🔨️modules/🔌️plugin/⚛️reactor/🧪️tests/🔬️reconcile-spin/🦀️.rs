use super::pending::with_state as with_pending;
use super::turn::{drive_reconcile_within, reconcile_arms_turn, PATCH_CLOSE_UNITS_PER_TURN, PATCH_RETIREMENT_BYTES_PER_UNIT, PATCH_RETIREMENT_ITEMS_PER_UNIT};
use super::*;
use semio_framework::kernel::{ActorInstanceLifetime, ActorUiPatchReceipt};
use semio_framework_ui_runtime::{ComponentTree, TreeNode};

/// 🌲️ The cheapest admissible retained tree: this suite prices CROSSINGS, and a crossing costs the
/// same whether the surface carries one text leaf or a Nakagin-scale world
/// (`🔬️reconcile-budget` prices the bytes).
fn tree(text: &str) -> ComponentTree {
    let value = ui_contract::UiText::try_from_str(text).expect("bounded fixture text");
    let root = TreeNode::try_new("root", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(value), emphasize: None, data_attributes: None })).expect("bounded fixture tree");
    ComponentTree { root }
}

fn budget() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔁️reconcile-spin.json")).expect("reconcile spin budget fixture")
}

fn far_deadline() -> std::time::Instant {
    std::time::Instant::now() + std::time::Duration::from_secs(60)
}

fn drain_pending_authority() {
    with_pending(|pending| {
        let mut pending = pending.borrow_mut();
        for _ in 0..1_000_000 {
            if pending.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("bounded pending retirement") {
                return;
            }
        }
        panic!("pending authority never retired");
    });
}

/// 🧾️ One host↔guest crossing, recorded by what it carried. `carried` is the whole point: the target
/// this suite guards is "every crossing carries events or a patch".
#[derive(Default)]
struct CrossingLedger {
    crossings: usize,
    with_patch: usize,
    with_events: usize,
    idle: usize,
}

/// 🔁️ The browser loop, exactly as `PluginRuntime.settlePluginTurn` drives it: post a turn carrying
/// the acknowledgements the previous turn earned, run the guest's own publication ladder, take the
/// patches one at a time through `take_one` (the batching ladder above it is priced by
/// `🧺️turn-patch-batch`), and stop when the guest stops answering
/// `MoreWork` with nothing left to acknowledge. Every lap is one worker round trip.
fn settle_surfaces(instance: u32, count: usize) -> (CrossingLedger, usize) {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = patches::PatchTracker::new();
    for index in 0..count {
        tracker.begin(format!("{instance}:surface-{index}"), tree("body")).expect("mounted admission");
    }
    settle_tracker(&tracker, instance)
}

/// 🔁️ The same loop over an already-mounted tracker, so a law that set the tracker up differently
/// still converges through the production ladder instead of leaking a borrowed publication turn.
fn settle_tracker(tracker: &patches::PatchTracker, instance: u32) -> (CrossingLedger, usize) {
    let mut ledger = CrossingLedger::default();
    let mut acknowledgements: Vec<(ActorUiPatchReceipt, String, u64)> = Vec::new();
    let mut sequence = 0u64;
    let mut published = 0usize;
    loop {
        ledger.crossings += 1;
        assert!(ledger.crossings < 4_096, "the publication ladder never quiesced: {} | {}", tracker.debug_state(), with_pending(|pending| pending.borrow().debug_state()));
        let carried_events = !acknowledgements.is_empty();
        if carried_events {
            ledger.with_events += 1;
        }
        with_pending(|pending| {
            let mut pending = pending.borrow_mut();
            for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
                if pending.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("bounded retirement") {
                    break;
                }
            }
        });
        for (receipt, surface, revision) in acknowledgements.drain(..) {
            assert!(
                with_pending(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| tracker.mark_published_ack(ack))).expect("issued ack"),
                "the host's acknowledgement of {surface}@{revision} must be admitted"
            );
        }
        with_pending(|pending| {
            let mut pending = pending.borrow_mut();
            for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
                if pending.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("bounded retirement") {
                    break;
                }
            }
        });
        drive_reconcile_within(tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
        let taken = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication");
        let carried_patch = taken.is_some();
        if let Some(patch) = taken {
            sequence += 1;
            published += 1;
            ledger.with_patch += 1;
            let receipt = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: instance, guest_lifetime: 1 }, patch_sequence: sequence };
            with_pending(|pending| pending.borrow_mut().stage_emission(receipt, std::iter::once(&patch))).expect("staged emission");
            with_pending(|pending| pending.borrow_mut().commit_emission());
            acknowledgements.push((receipt, patch.surface.0.to_string(), patch.revision.0));
            let mut owner = ui_contract::UiPendingPatch::default();
            *owner.source_mut().expect("writable payload") = Some(patch);
            for _ in 0..1_000_000 {
                if owner.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("host-side patch retirement").complete {
                    break;
                }
            }
        }
        let more = reconcile_arms_turn(tracker);
        if !carried_events && !carried_patch {
            ledger.idle += 1;
        }
        if !more && acknowledgements.is_empty() {
            break;
        }
    }
    drain_pending_authority();
    (ledger, published)
}

/// 🎯️ THE law: N pending retained surfaces converge in a bounded number of host round trips, and the
/// bound is per SURFACE, not per publication phase.
///
/// 🐛️ Before 2026-09-14 one published surface cost four crossings: one to move the patch from the
/// slot into the turn handback, one to hand it out, one to carry the host's acknowledgement, one to
/// retire the acknowledged slot — and the last two were free, because both happen inside a turn the
/// host is already paying for. Measured in the browser on the procedural 3d React door: 111 crossings
/// per `flowEvalTick` hop, 89 of them posting no events and returning no patch
/// (`📓️reactor-reconcile-spin-2026-09-14.md` §2).
#[test]
fn n_pending_retained_surfaces_converge_in_bounded_crossings_that_each_carry_something() {
    let fixture = budget();
    let per_surface = fixture["crossingsPerSurface"].as_u64().expect("declared crossing budget") as usize;
    for count in fixture["surfaceCounts"].as_array().expect("declared surface counts").iter().map(|value| value.as_u64().expect("surface count") as usize) {
        let (ledger, published) = settle_surfaces(40 + count as u32, count);
        assert_eq!(published, count, "every mounted surface must publish exactly once");
        assert!(ledger.crossings <= count * per_surface + 1, "{count} surfaces cost {} crossings against the declared {per_surface} per surface", ledger.crossings);
        assert!(ledger.with_patch >= count, "{count} surfaces published {} patch-carrying crossings", ledger.with_patch);
        assert!(ledger.idle <= fixture["idleCrossingsAllowed"].as_u64().expect("declared idle allowance") as usize, "{count} surfaces cost {} crossings that carried neither an event nor a patch", ledger.idle);
        eprintln!("[DEBUG] reconcile-spin surfaces={count} crossings={} patch={} events={} idle={}", ledger.crossings, ledger.with_patch, ledger.with_events, ledger.idle);
    }
}

/// 📤️ A patch that reached the turn handback leaves the guest on the SAME turn. The old two-call
/// shape spent one whole host round trip moving it from its slot into the handback and answering
/// `None`.
#[test]
fn a_published_patch_leaves_the_guest_on_the_turn_that_published_it() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = patches::PatchTracker::new();
    tracker.begin("51:surface-0".to_string(), tree("body")).expect("mounted admission");
    for _ in 0..1_024 {
        drive_reconcile_within(&tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
        if with_pending(|pending| pending.borrow().has_undelivered()) {
            break;
        }
    }
    let first = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication");
    assert!(first.is_some(), "the first take of a queued publication must hand out its patch: {}", with_pending(|pending| pending.borrow().debug_state()));
    let mut owner = ui_contract::UiPendingPatch::default();
    *owner.source_mut().expect("writable payload") = first;
    for _ in 0..1_000_000 {
        if owner.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("retirement").complete {
            break;
        }
    }
    drain_pending_authority();
}

/// 🛑️ A turn with a ready output it cannot publish — every publication slot taken, so the release
/// depends on an acknowledgement that arrives as its OWN event-carrying turn — must neither spend an
/// opportunity on it nor arm another turn for it. Arming there is what bought a host round trip per
/// turn that carried nothing (`📓️reactor-reconcile-spin-2026-09-14.md` §1, family 2).
#[test]
fn a_host_blocked_ready_output_spends_no_opportunity_and_arms_nothing() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let instance = 52u32;
    let mut issued: Vec<(ActorUiPatchReceipt, String, u64)> = Vec::new();
    let mut sequence = 0u64;
    while with_pending(|pending| pending.borrow().has_capacity()) {
        sequence += 1;
        let surface = ui_contract::SurfaceId::try_from(format!("{instance}:filler-{sequence}")).expect("fixture surface id");
        with_pending(|pending| pending.borrow_mut().push_external(ui_contract::UiPatch { surface, base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(sequence), ops: Default::default() })).expect("external publication");
        let patch = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication").expect("queued filler");
        let receipt = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: instance, guest_lifetime: 1 }, patch_sequence: sequence };
        with_pending(|pending| pending.borrow_mut().stage_emission(receipt, std::iter::once(&patch))).expect("staged emission");
        with_pending(|pending| pending.borrow_mut().commit_emission());
        issued.push((receipt, patch.surface.0.to_string(), patch.revision.0));
        retire_host_patch(patch);
    }
    with_pending(|pending| assert!(pending.borrow().phases().iter().all(|phase| *phase == pending::PendingPatchPhase::Issued), "every filler slot must be host-blocked"));
    let tracker = patches::PatchTracker::new();
    tracker.begin(format!("{instance}:surface-0"), tree("body")).expect("mounted admission");
    for _ in 0..1_000_000 {
        drive_reconcile_within(&tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
        if !tracker.has_drivable_work() {
            break;
        }
    }
    assert!(tracker.has_publishable_work(), "the fixture must leave one ready output behind: {}", tracker.debug_state());
    let spent = drive_reconcile_within(&tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
    let arms = reconcile_arms_turn(&tracker);
    eprintln!("[DEBUG] reconcile-spin host-blocked spent={spent} arms={arms} tracker={}", tracker.debug_state());
    assert_eq!(spent, 0, "a saturated publication authority with nothing drivable must spend no opportunity: {}", tracker.debug_state());
    assert!(!arms, "a ready output the HOST is blocking must not arm another turn: {} | {}", tracker.debug_state(), with_pending(|pending| pending.borrow().debug_state()));
    for (receipt, surface, revision) in issued {
        assert!(with_pending(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |_| Ok(true))).expect("issued ack"));
    }
    drain_pending_authority();
    settle_tracker(&tracker, instance);
}

/// 🧹️ Retires one patch the way the HOST does once it has admitted it, so a law that publishes many
/// of them does not hold their owners open.
fn retire_host_patch(patch: ui_contract::UiPatch) {
    let mut owner = ui_contract::UiPendingPatch::default();
    *owner.source_mut().expect("writable payload") = Some(patch);
    for _ in 0..1_000_000 {
        if owner.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("host-side patch retirement").complete {
            return;
        }
    }
    panic!("host patch retirement never completed");
}

/// ⏱️ The hold law, unchanged by this wave and asserted rather than assumed: a drive whose wall
/// deadline is already spent stops within one deadline stride, never at the 1 024-opportunity
/// ceiling.
#[test]
fn the_reconcile_drive_respects_the_turns_wall_hold() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture = budget();
    let stride = fixture["deadlineStride"].as_u64().expect("declared deadline stride") as usize;
    let tracker = patches::PatchTracker::new();
    for index in 0..4 {
        tracker.begin(format!("53:surface-{index}"), tree("body")).expect("mounted admission");
    }
    let spent = drive_reconcile_within(&tracker, reconcile_step_opportunities(u64::MAX), std::time::Instant::now()).expect("reconcile drive");
    assert!(spent <= stride, "an expired hold must stop the drive inside one {stride}-opportunity stride; spent {spent}");
    eprintln!("[DEBUG] reconcile-spin expired-hold spent={spent} stride={stride}");
    let (ledger, published) = settle_tracker(&tracker, 53);
    assert_eq!(published, 4, "a drive cut short by its hold still converges on the turns that follow it");
    assert_eq!(ledger.idle, 0, "resuming after a spent hold must not cost a crossing that carries nothing");
}

/// 🛑️ A close that lands MID-drive stops the publication of that instance's surfaces and leaves the
/// turn armed only for the retirement it owes — never for a surface that will never publish again.
#[test]
fn a_cancellation_mid_drive_stops_the_publication_and_still_quiesces() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let instance = 54u32;
    let tracker = patches::PatchTracker::new();
    for index in 0..3 {
        tracker.begin(format!("{instance}:surface-{index}"), tree("body")).expect("mounted admission");
    }
    drive_reconcile_within(&tracker, 8, far_deadline()).expect("partial reconcile drive");
    let key = instance_lifetime::NativeCloseKey::fixture(instance, 1);
    tracker.reserve_close_instance(key).expect("exact close reservation");
    tracker.activate_close_instance(key).expect("activate retained close");
    with_pending(|pending| pending.borrow_mut().reserve_close_instance(key)).expect("pending close reservation");
    with_pending(|pending| pending.borrow_mut().activate_close_instance(key)).expect("activate pending close");
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 4_096, "a cancelled instance never quiesced: {} | {}", tracker.debug_state(), with_pending(|pending| pending.borrow().debug_state()));
        drive_reconcile_within(&tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
        with_pending(|pending| {
            let mut pending = pending.borrow_mut();
            for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
                if pending.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("bounded retirement") {
                    break;
                }
            }
        });
        for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
            if tracker.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT) {
                break;
            }
        }
        if tracker.close_instance_complete(key).expect("exact retained close receipt") {
            tracker.release_close_instance(key).expect("retained close release");
        }
        if with_pending(|pending| pending.borrow().close_instance_complete(key)).expect("exact pending close receipt") {
            with_pending(|pending| pending.borrow_mut().release_close_instance(key)).expect("pending close release");
        }
        if !reconcile_arms_turn(&tracker) {
            break;
        }
        if with_pending(|pending| pending.borrow().has_undelivered()) {
            let taken = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication");
            if let Some(patch) = taken {
                let mut owner = ui_contract::UiPendingPatch::default();
                *owner.source_mut().expect("writable payload") = Some(patch);
                for _ in 0..1_000_000 {
                    if owner.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("retirement").complete {
                        break;
                    }
                }
            }
        }
    }
    eprintln!("[DEBUG] reconcile-spin cancellation quiesced in {turns} turns");
    assert!(!reconcile_arms_turn(&tracker), "a cancelled, retired instance must not arm the turn");
}

/// 📨️ The publication phases partition exactly: what the guest owes a turn for is
/// `has_undelivered() ∪ has_retiring()`, and an `Issued` slot — the host's half of the handshake — is
/// in neither.
#[test]
fn the_pending_publication_phases_partition_what_arms_a_turn() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = patches::PatchTracker::new();
    tracker.begin("55:surface-0".to_string(), tree("body")).expect("mounted admission");
    for _ in 0..1_024 {
        drive_reconcile_within(&tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
        if with_pending(|pending| pending.borrow().has_undelivered()) {
            break;
        }
    }
    with_pending(|pending| {
        let pending = pending.borrow();
        assert_eq!(pending.phases(), vec![pending::PendingPatchPhase::Queued]);
        assert!(pending.has_undelivered() && !pending.has_retiring() && pending.has_unpublished());
    });
    let patch = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication").expect("queued patch");
    let receipt = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: 55, guest_lifetime: 1 }, patch_sequence: 1 };
    with_pending(|pending| pending.borrow_mut().stage_emission(receipt, std::iter::once(&patch))).expect("staged emission");
    with_pending(|pending| pending.borrow_mut().commit_emission());
    with_pending(|pending| {
        let pending = pending.borrow();
        assert_eq!(pending.phases(), vec![pending::PendingPatchPhase::Issued]);
        assert!(!pending.has_undelivered() && !pending.has_retiring() && !pending.has_unpublished(), "an issued publication is the HOST's turn, not the guest's");
    });
    assert!(with_pending(|pending| pending.borrow_mut().apply_issued_ack(receipt, &patch.surface.0, patch.revision.0, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| tracker.mark_published_ack(ack))).expect("issued ack"));
    with_pending(|pending| {
        let pending = pending.borrow();
        assert_eq!(pending.phases(), vec![pending::PendingPatchPhase::Acknowledged]);
        assert!(!pending.has_undelivered() && pending.has_retiring() && pending.has_unpublished());
    });
    let mut owner = ui_contract::UiPendingPatch::default();
    *owner.source_mut().expect("writable payload") = Some(patch);
    for _ in 0..1_000_000 {
        if owner.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("retirement").complete {
            break;
        }
    }
    drain_pending_authority();
    with_pending(|pending| {
        let pending = pending.borrow();
        assert!(pending.phases().is_empty() && !pending.has_unpublished(), "a retired publication leaves no phase behind");
    });
}
