use super::pending::with_state as with_pending;
use super::turn::{drive_reconcile_within, reconcile_arms_turn, take_turn_patch_page, turn_patch_budget_bytes, PATCH_CLOSE_UNITS_PER_TURN, PATCH_RETIREMENT_BYTES_PER_UNIT, PATCH_RETIREMENT_ITEMS_PER_UNIT};
use super::*;
use semio_framework::kernel::{ActorInstanceLifetime, ActorUiPatchReceipt, UiTurnPatchTransfer, UiTurnPatches, UI_TURN_PATCHES_MAXIMUM, UI_TURN_PATCH_BUDGET_BYTES, UI_TURN_PATCH_OWNER_BYTES};
use semio_framework_ui_runtime::{ComponentTree, TreeNode};

/// 🌲️ The cheapest admissible retained tree — this suite prices CROSSINGS and publication ORDER, not
/// document width.
fn tree(text: &str) -> ComponentTree {
    let value = ui_contract::UiText::try_from_str(text).expect("bounded fixture text");
    let root = TreeNode::try_new("root", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(value), emphasize: None, data_attributes: None })).expect("bounded fixture tree");
    ComponentTree { root }
}

fn budget() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧺️turn-patch-batch.json")).expect("turn patch batch budget fixture")
}

fn far_deadline() -> std::time::Instant {
    std::time::Instant::now() + std::time::Duration::from_secs(60)
}

fn generous_budget() -> usize {
    turn_patch_budget_bytes(semio_framework::kernel::Budget { fuel: u64::MAX, deadline_ms: 1_000, max_effects: 64, max_patch_bytes: u32::MAX, max_frames: 8 })
}

fn retire_pending_authority() {
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

/// 🧹️ Retires one patch the way the HOST does once it has admitted it.
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

/// 📤️ Drains a turn page the way the host does, reporting what it carried in publication order.
fn drain_page(page: &mut UiTurnPatches) -> Vec<(String, u64)> {
    let mut carried = Vec::new();
    loop {
        let mut named = None;
        match page.try_transfer_one(|patch| {
            named = Some((patch.surface.0.to_string(), patch.revision.0));
            Ok::<ui_contract::UiPatch, ui_contract::UiPatch>(patch)
        }) {
            UiTurnPatchTransfer::Transferred(patch) => {
                carried.push(named.expect("named publication"));
                retire_host_patch(patch);
            }
            UiTurnPatchTransfer::Empty => break,
            UiTurnPatchTransfer::Refused => panic!("a host drain of an owned turn page is never refused"),
        }
    }
    carried
}

#[derive(Default)]
struct CrossingLedger {
    crossings: usize,
    with_patch: usize,
    with_events: usize,
    idle: usize,
}

/// 🔁️ The browser loop as `PluginRuntime.settlePluginTurn` drives it AFTER this lane: post the
/// acknowledgements the previous crossing earned — all of them, in one event list — run the guest's
/// publication ladder, and take the whole page the turn may carry instead of one patch.
fn settle_batched(tracker: &patches::PatchTracker, instance: u32, budget_bytes: usize) -> (CrossingLedger, Vec<(String, u64)>) {
    let mut ledger = CrossingLedger::default();
    let mut acknowledgements: Vec<(ActorUiPatchReceipt, String, u64)> = Vec::new();
    let mut delivered: Vec<(String, u64)> = Vec::new();
    let mut sequence = 0u64;
    loop {
        ledger.crossings += 1;
        assert!(ledger.crossings < 4_096, "the publication ladder never quiesced: {} | {}", tracker.debug_state(), with_pending(|pending| pending.borrow().debug_state()));
        let carried_events = !acknowledgements.is_empty();
        if carried_events {
            ledger.with_events += 1;
        }
        retire_pending_authority();
        for (receipt, surface, revision) in acknowledgements.drain(..) {
            assert!(
                with_pending(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| tracker.mark_published_ack(ack))).expect("issued ack"),
                "the host's acknowledgement of {surface}@{revision} must be admitted"
            );
        }
        retire_pending_authority();
        drive_reconcile_within(tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
        let mut page = take_turn_patch_page(budget_bytes).expect("turn patch page");
        let carried_patch = !page.is_empty();
        if carried_patch {
            ledger.with_patch += 1;
            sequence += 1;
            let receipt = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: instance, guest_lifetime: 1 }, patch_sequence: sequence };
            with_pending(|pending| pending.borrow_mut().stage_emission(receipt, page.iter())).expect("staged emission");
            with_pending(|pending| pending.borrow_mut().commit_emission());
            for carried in drain_page(&mut page) {
                acknowledgements.push((receipt, carried.0.clone(), carried.1));
                delivered.push(carried);
            }
        }
        if !carried_events && !carried_patch {
            ledger.idle += 1;
        }
        if !reconcile_arms_turn(tracker) && acknowledgements.is_empty() {
            break;
        }
    }
    retire_pending_authority();
    (ledger, delivered)
}

/// 🎯️ THE law: N surfaces that are ready together converge in TWO crossings — one that carries every
/// patch, one that carries every acknowledgement — and not one crossing carries nothing.
///
/// 🐛️ Until 2026-09-15 `UI_TURN_PATCHES_MAXIMUM` was 1, so the floor was N + 1 crossings by wire
/// contract, measured in the browser at 58.60 worker crossings per `flowEvalTick` hop on the
/// procedural 3d React door (`📓️ui-turn-patch-batching-2026-09-15.md` §5).
#[test]
fn n_ready_surfaces_converge_in_two_crossings_when_their_patches_fit_the_budget() {
    let fixture = budget();
    let ceiling = fixture["crossingsPerBatch"].as_u64().expect("declared crossing budget") as usize;
    for count in fixture["surfaceCounts"].as_array().expect("declared surface counts").iter().map(|value| value.as_u64().expect("surface count") as usize) {
        let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
        let instance = 70 + count as u32;
        let tracker = patches::PatchTracker::new();
        for index in 0..count {
            tracker.begin(format!("{instance}:surface-{index}"), tree("body")).expect("mounted admission");
        }
        let (ledger, delivered) = settle_batched(&tracker, instance, generous_budget());
        assert_eq!(delivered.len(), count, "every mounted surface must publish exactly once");
        assert!(ledger.crossings <= ceiling, "{count} surfaces cost {} crossings against the declared {ceiling}", ledger.crossings);
        assert_eq!(ledger.with_patch, 1, "{count} surfaces that fit the budget must travel in ONE patch-carrying crossing");
        assert!(ledger.idle <= fixture["idleCrossingsAllowed"].as_u64().expect("declared idle allowance") as usize, "{count} surfaces cost {} crossings that carried neither an event nor a patch", ledger.idle);
        eprintln!("[DEBUG] turn-patch-batch surfaces={count} crossings={} patch={} events={} idle={}", ledger.crossings, ledger.with_patch, ledger.with_events, ledger.idle);
    }
}

/// 🧾️ The publication ORDER of one instance's queued patches is the page's order, and a page the
/// caller refuses returns every one of them to its own reserved slot — the next page reads back
/// byte-identically, in the same order, exactly once.
#[test]
fn a_refused_turn_page_returns_every_patch_to_its_own_slot_in_publication_order() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture = budget();
    let count = fixture["batchSurfaces"].as_u64().expect("declared batch surfaces") as usize;
    let instance = 79u32;
    let expected = queue_external(instance, count);
    let mut page = take_turn_patch_page(generous_budget()).expect("turn patch page");
    assert_eq!(page.iter().map(|patch| (patch.surface.0.to_string(), patch.revision.0)).collect::<Vec<_>>(), expected, "the page carries the queued publications in order");
    loop {
        match page.try_transfer_one(|patch| with_pending(|pending| pending.borrow_mut().hand_back_turn(patch))) {
            UiTurnPatchTransfer::Transferred(()) => {}
            UiTurnPatchTransfer::Empty => break,
            UiTurnPatchTransfer::Refused => panic!("a refused page returns every patch to its reserved slot"),
        }
    }
    let mut again = take_turn_patch_page(generous_budget()).expect("turn patch page");
    assert_eq!(again.iter().map(|patch| (patch.surface.0.to_string(), patch.revision.0)).collect::<Vec<_>>(), expected, "a returned page reads back in the same publication order");
    eprintln!("[DEBUG] turn-patch-batch handback count={count} order={expected:?}");
    drain_page(&mut again);
    retire_pending_authority();
}

/// 📏️ The BYTE budget, not a count, is the admission rule: a page cut by the budget carries what fit,
/// the patch that did not fit travels on the NEXT turn, and the two pages together deliver every
/// publication exactly once and in order.
#[test]
fn a_patch_over_the_turn_byte_budget_travels_on_the_next_turn_and_still_applies_exactly() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture = budget();
    let count = fixture["batchSurfaces"].as_u64().expect("declared batch surfaces") as usize;
    let fit = fixture["budgetPatchesThatFit"].as_u64().expect("declared budget width") as usize;
    let instance = 80u32;
    let expected = queue_external(instance, count);
    let unit = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication").expect("queued publication");
    let cost = semio_framework::kernel::ui_patch_turn_bytes(&unit);
    with_pending(|pending| pending.borrow_mut().hand_back_turn(unit)).expect("probe returns to its slot");
    let mut delivered = Vec::new();
    for turn in 0..count {
        let mut page = take_turn_patch_page(cost * fit).expect("turn patch page");
        assert!(!page.is_empty(), "turn {turn} of a queued batch must still make progress");
        assert!(page.len() <= fit, "the byte budget admitted {} patches against the declared {fit}", page.len());
        delivered.extend(drain_page(&mut page));
        if delivered.len() == count {
            break;
        }
    }
    assert_eq!(delivered, expected, "a batch the budget cut still delivers every publication exactly once, in order");
    eprintln!("[DEBUG] turn-patch-batch split count={count} fit={fit} unit={cost}B delivered={}", delivered.len());
    retire_pending_authority();
}

/// 🚦️ The FIRST ready patch is admitted whatever the budget says — otherwise a publication larger
/// than the whole budget could never leave the guest and its surface would never converge.
#[test]
fn the_first_ready_patch_is_admitted_whatever_the_budget_says() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let fixture = budget();
    let instance = 81u32;
    let expected = queue_external(instance, 3);
    let mut page = take_turn_patch_page(fixture["overBudgetBytes"].as_u64().expect("declared over-budget grant") as usize).expect("turn patch page");
    assert_eq!(page.len(), 1, "a budget no publication fits still admits the first one");
    assert_eq!(drain_page(&mut page), expected[..1], "and it is the oldest queued publication");
    eprintln!("[DEBUG] turn-patch-batch first-always-admitted len=1");
    let mut rest = take_turn_patch_page(generous_budget()).expect("turn patch page");
    assert_eq!(drain_page(&mut rest), expected[1..], "the rest follow in order");
    retire_pending_authority();
}

/// 🧨️ A turn never becomes a contiguous guest request past the declared ceiling: the page is parked
/// BY VALUE in every transport slot and handback, so its own extent is the block the guest's
/// `dlmalloc` granularity has to fund — and the capacity is READ OFF that ceiling, never chosen.
#[test]
fn a_turn_patch_page_never_asks_the_guest_for_more_than_one_contiguous_ceiling() {
    assert_eq!(UI_TURN_PATCHES_MAXIMUM, semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES);
    assert!(UI_TURN_PATCHES_MAXIMUM > 1, "a page that carries one patch is the floor this lane removed");
    assert!(size_of::<UiTurnPatches>() <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, "the turn page is {} B against a {} B contiguous ceiling", size_of::<UiTurnPatches>(), semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);
    assert_eq!(UI_TURN_PATCH_BUDGET_BYTES, semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES / 4);
    let window = |declared: usize| turn_patch_budget_bytes(semio_framework::kernel::Budget { fuel: 0, deadline_ms: 0, max_effects: 0, max_patch_bytes: declared as u32, max_frames: 0 });
    for declared in budget()["laneDeclarations"].as_array().expect("declared lane budgets").iter().map(|value| value.as_u64().expect("lane budget") as usize) {
        assert!(declared >= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES && declared <= UI_TURN_PATCH_BUDGET_BYTES, "a lane declaring {declared} B is outside the budget window");
        assert_eq!(window(declared), declared, "a lane inside the window is admitted verbatim");
    }
    assert_eq!(window(0), semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, "a lane below the floor is lifted to one contiguous ceiling");
    assert_eq!(window(u32::MAX as usize), UI_TURN_PATCH_BUDGET_BYTES, "a lane above the ceiling is cut to the declared budget");
    eprintln!("[DEBUG] turn-patch-batch capacity={UI_TURN_PATCHES_MAXIMUM} page={}B ceiling={}B budget={UI_TURN_PATCH_BUDGET_BYTES}B", size_of::<UiTurnPatches>(), semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);
}

/// 📥️ Queues `count` external publications on one instance and reports the order they must come back.
fn queue_external(instance: u32, count: usize) -> Vec<(String, u64)> {
    let mut expected = Vec::new();
    for index in 0..count {
        let name = format!("{instance}:surface-{index}");
        let surface = ui_contract::SurfaceId::try_from(name.clone()).expect("fixture surface id");
        let revision = index as u64 + 1;
        with_pending(|pending| pending.borrow_mut().push_external(ui_contract::UiPatch { surface, base_revision: ui_contract::UiRevision(revision - 1), revision: ui_contract::UiRevision(revision), ops: Default::default() })).expect("external publication");
        expected.push((name, revision));
    }
    expected
}
