use super::pending::with_state as with_pending;
use super::turn::{drive_reconcile_within, more_work_drive_budget_ms, more_work_drive_steps, reconcile_arms_turn, MEASURED_GUEST_TURN_COST_MS, PATCH_CLOSE_UNITS_PER_TURN, PATCH_RETIREMENT_BYTES_PER_UNIT, PATCH_RETIREMENT_ITEMS_PER_UNIT, REACTOR_TURN_EXECUTOR_HOLD_MS};
use super::*;
use semio_framework::kernel::{ActorInstanceLifetime, ActorUiPatchReceipt};
use semio_framework_ui_runtime::{ComponentTree, TreeNode};
use std::cell::Cell;

/// 🌲️ The cheapest admissible retained tree — this suite prices CROSSINGS, never bytes.
fn tree(text: &str) -> ComponentTree {
    let value = ui_contract::UiText::try_from_str(text).expect("bounded fixture text");
    let root = TreeNode::try_new("root", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(value), emphasize: None, data_attributes: None })).expect("bounded fixture tree");
    ComponentTree { root }
}

fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🚚️more-work-drive.json")).expect("more-work drive contract fixture")
}

fn number(fixture: &serde_json::Value, key: &str) -> u64 {
    fixture[key].as_u64().unwrap_or_else(|| panic!("the more-work drive contract declares no {key}"))
}

fn far_deadline() -> std::time::Instant {
    std::time::Instant::now() + std::time::Duration::from_secs(60)
}

fn declared_steps() -> usize {
    let fixture = contract();
    more_work_drive_steps(number(&fixture, "hostGrantWallMs"), number(&fixture, "measuredGuestTurnCostMs"))
}

/// 🧾️ Why the worker's drive crossed back, named so a law asserts the REASON rather than only the
/// count. Twin of `🎭️actor/🖼️wire-turn/🟦️.ts`'s `driveShardTurnMoreWorkV1` stop tags.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DriveStop {
    Carried,
    Idle,
    Steps,
    Input,
    Closed,
}

/// 🔁️ One guest turn's answer, as the drive reads it: did it carry anything the host must see, and
/// does the reactor still owe another turn.
#[derive(Clone, Copy)]
struct TurnOutcome {
    carried: bool,
    arms: bool,
}

/// 🚚️ The worker-owned drive, as the test model of the loop
/// `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` emits into `🟨️shard-worker.js`.
///
/// It lives here, not in production Rust, for the same reason `🔬️reconcile-spin`'s `settle_tracker`
/// does: the production owner of this loop is the browser worker, and this module's job is to price
/// what the REACTOR makes that loop cost. Everything the loop is bounded BY — the step ceiling and its
/// wall budget — comes from production code ([`more_work_drive_steps`]) and the shared fixture, so the
/// two implementations cannot drift on the only numbers that matter.
fn drive_more_work(steps: usize, mut turn: impl FnMut() -> TurnOutcome, input_pending: &Cell<bool>, live: &Cell<bool>) -> (usize, DriveStop) {
    let mut spent = 0usize;
    loop {
        let outcome = turn();
        spent += 1;
        if outcome.carried {
            return (spent, DriveStop::Carried);
        }
        if !outcome.arms {
            return (spent, DriveStop::Idle);
        }
        if input_pending.get() {
            return (spent, DriveStop::Input);
        }
        if !live.get() {
            return (spent, DriveStop::Closed);
        }
        if spent >= steps {
            return (spent, DriveStop::Steps);
        }
    }
}

/// 🎯️ THE law: a guest that answers `MoreWork` with NOTHING, k times in a row, costs the host ONE
/// crossing.
///
/// 🐛️ Before this lane every one of those k answers was a host round trip — a POST, a poll, a
/// structured clone and a main-thread pickup — because in the browser the host round trip IS the
/// guest's pump. Measured on the procedural 3d React door 2026-09-15: **32.9 of 60.7 worker crossings
/// per `flowEvalTick` hop posted no events and returned no patch**
/// (`📓️ui-turn-patch-batching-2026-09-15.md` §7 item 2). The silent-turn hold of
/// `📓️reactor-reconcile-spin-2026-09-14.md` §3.5 was supposed to absorb them and could not: its wall
/// was [`REACTOR_TURN_EXECUTOR_HOLD_MS`] while one whole turn measures
/// [`MEASURED_GUEST_TURN_COST_MS`], so it admitted exactly one extra poll. The bound is now a STEP
/// CEILING derived from that measurement, and the same k answers cost one crossing.
#[test]
fn a_guest_answering_more_work_k_times_with_nothing_costs_one_host_crossing() {
    let fixture = contract();
    let steps = declared_steps();
    let expected = number(&fixture, "crossingsPerSilentRun") as usize;
    let quiet = Cell::new(false);
    let alive = Cell::new(true);
    for run in fixture["silentTurnRuns"].as_array().expect("declared silent turn runs").iter().map(|value| value.as_u64().expect("silent turn count") as usize) {
        assert!(run < steps, "the fixture's silent run of {run} must fit the derived ceiling of {steps} with room for the turn that carries");
        let remaining = Cell::new(run);
        let turns = Cell::new(0usize);
        let (spent, stop) = drive_more_work(
            steps,
            || {
                turns.set(turns.get() + 1);
                if remaining.get() == 0 {
                    return TurnOutcome { carried: true, arms: true };
                }
                remaining.set(remaining.get() - 1);
                TurnOutcome { carried: false, arms: true }
            },
            &quiet,
            &alive,
        );
        assert_eq!(stop, DriveStop::Carried, "a drive of {steps} steps must absorb {run} silent turns, spent {spent}");
        assert_eq!(turns.get(), run + 1, "the drive must run every silent turn plus the one that carried");
        eprintln!("[DEBUG] more-work-drive silent={run} crossings={expected} turns={} steps={steps}", turns.get());
    }
    assert_eq!(expected, 1, "the contract this law is named after declares exactly one crossing per silent run");
}

/// ⛔️ The host's continuation ceiling is a bound on CROSSINGS, and the drive may only make that bound
/// cover MORE guest work — never less.
///
/// `PluginRuntime`'s command-ingress drain and `PLUGIN_UI_CONTINUATION_LIMIT` both count host round
/// trips, so a guest that needs k turns to reach its terminal used to need k of them. Under the drive
/// it needs `ceil(k / steps)`, which is never more than k and is `steps` times fewer at the ceiling —
/// so one ceiling of continuations covers `hostContinuationCeiling × steps` guest turns. This law is
/// what fails if a future drive ever turns a bounded host loop into an unbounded one.
#[test]
fn the_hosts_continuation_ceiling_covers_more_guest_turns_under_the_drive_never_fewer() {
    let fixture = contract();
    let steps = declared_steps();
    let ceiling = number(&fixture, "hostContinuationCeiling") as usize;
    let quiet = Cell::new(false);
    let alive = Cell::new(true);
    for needed in 1..=64usize {
        let turns = Cell::new(0usize);
        let mut crossings = 0usize;
        loop {
            crossings += 1;
            assert!(crossings <= needed, "a guest needing {needed} turns cost {crossings} host continuations — the drive may never cost MORE than the host-polled shape");
            let (_spent, stop) = drive_more_work(
                steps,
                || {
                    turns.set(turns.get() + 1);
                    TurnOutcome { carried: turns.get() >= needed, arms: true }
                },
                &quiet,
                &alive,
            );
            if stop == DriveStop::Carried {
                break;
            }
            assert_eq!(stop, DriveStop::Steps, "only a spent ceiling may end a drive that carried nothing here");
        }
        assert_eq!(crossings, needed.div_ceil(steps), "a guest needing {needed} turns must cost ceil({needed}/{steps}) host continuations");
        assert_eq!(turns.get(), needed, "the drive must run exactly the turns the guest needed");
    }
    assert_eq!(ceiling * steps, ceiling.checked_mul(steps).expect("the ceiling's guest-turn reach"), "the reach is the ceiling priced in drive steps");
    eprintln!("[DEBUG] more-work-drive continuation ceiling={ceiling} steps={steps} guest-turn reach={}", ceiling * steps);
}

/// 📬️ A host-owned input pending behind the drive — an ingress message, a cancel, a view-state change
/// — ends it within ONE turn, so the drive can never delay what the host is waiting to say.
#[test]
fn a_pending_host_input_interrupts_the_drive_within_one_turn() {
    let fixture = contract();
    let steps = declared_steps();
    let within = number(&fixture, "ingressInterruptTurns") as usize;
    let alive = Cell::new(true);
    for arrives_after in 0..steps {
        let turns = Cell::new(0usize);
        let pending = Cell::new(arrives_after == 0);
        let (spent, stop) = drive_more_work(
            steps,
            || {
                turns.set(turns.get() + 1);
                if turns.get() >= arrives_after {
                    pending.set(true);
                }
                TurnOutcome { carried: false, arms: true }
            },
            &pending,
            &alive,
        );
        assert_eq!(stop, DriveStop::Input, "an input pending after {arrives_after} turns must end the drive");
        assert!(spent <= arrives_after.max(1) + within, "the drive ran {spent} turns after an input that arrived at turn {arrives_after}, past the declared {within}-turn interrupt");
    }
    eprintln!("[DEBUG] more-work-drive ingress interrupt within={within} steps={steps}");
}

/// 🛑️ A cancellation mid-drive — the actor disposed, or re-activated under a new generation — ends the
/// drive on the turn it lands, never on the next turn the guest happens to publish.
#[test]
fn a_cancellation_mid_drive_ends_the_drive_on_the_turn_it_lands() {
    let steps = declared_steps();
    let quiet = Cell::new(false);
    for closes_after in 1..=steps {
        let turns = Cell::new(0usize);
        let alive = Cell::new(true);
        let (spent, stop) = drive_more_work(
            steps,
            || {
                turns.set(turns.get() + 1);
                if turns.get() >= closes_after {
                    alive.set(false);
                }
                TurnOutcome { carried: false, arms: true }
            },
            &quiet,
            &alive,
        );
        if closes_after >= steps {
            assert!(matches!(stop, DriveStop::Closed | DriveStop::Steps), "a close at the ceiling stops the drive either way, got {stop:?}");
            continue;
        }
        assert_eq!(stop, DriveStop::Closed, "a close landing after {closes_after} turns must end the drive");
        assert_eq!(spent, closes_after, "the drive ran {spent} turns against a close at {closes_after}");
    }
    eprintln!("[DEBUG] more-work-drive cancellation steps={steps}");
}

/// ⏱️ Hold coherence: the reactor's own executor slice and what a whole guest turn costs are two
/// different numbers, and a drive bounded by the FIRST cannot absorb a single turn priced by the
/// SECOND. This is the law that fails if anyone re-derives the worker's budget from the reactor's hold
/// again, and the one to re-read when the measured turn cost moves.
#[test]
fn the_worker_drive_budget_is_derived_from_the_measured_turn_cost_not_from_the_reactors_hold() {
    let fixture = contract();
    let hold = number(&fixture, "reactorExecutorHoldMs");
    let cost = number(&fixture, "measuredGuestTurnCostMs");
    let grant = number(&fixture, "hostGrantWallMs");
    assert_eq!(hold, REACTOR_TURN_EXECUTOR_HOLD_MS, "the fixture and the reactor must name the same executor slice");
    assert_eq!(cost, MEASURED_GUEST_TURN_COST_MS, "the fixture and the reactor must name the same measured turn cost");
    assert!(cost > hold, "a turn costs more than the reactor's own slice — that is why the hold is not a drive budget");
    assert_eq!(hold / cost, 0, "a wall-only hold of {hold} ms admits no further {cost} ms turn at all; that is the incoherence this lane removed");
    let steps = more_work_drive_steps(grant, cost);
    assert_eq!(steps, number(&fixture, "driveStepCeiling") as usize, "the declared step ceiling must be the derivation, never a constant");
    assert_eq!(more_work_drive_budget_ms(grant, cost), number(&fixture, "driveBudgetMs"), "the declared drive budget must be the WHOLE grant, not the grant rounded down to whole measured turns");
    assert_eq!(more_work_drive_budget_ms(grant, cost), grant, "a drive spends the wall the host granted it — rounding it down to steps x cost crosses back empty with the grant unspent");
    assert_eq!(more_work_drive_budget_ms(cost - 1, cost), cost, "a grant smaller than one measured turn still buys one whole turn");
    assert_eq!(more_work_drive_steps(cost - 1, cost), 1, "a grant smaller than one turn still admits one turn");
    assert_eq!(more_work_drive_steps(grant, 0), 1, "an unmeasured turn cost still admits one turn");
    eprintln!("[DEBUG] more-work-drive coherence hold={hold}ms cost={cost}ms grant={grant}ms steps={steps} budget={}ms", more_work_drive_budget_ms(grant, cost));
}

/// 🎚️ Readiness is per ALLOCATION, not global: a finished output publishes while a DIFFERENT
/// surface's lower-generation reconcile job is still running, and each surface's own publications
/// still leave the guest in strictly increasing revision order.
///
/// 🐛️ `next_ready_index` used to compare the chosen output against the MINIMUM generation over every
/// slot still holding a producer or a job, so one slow surface serialized every other surface's
/// publication. That is the term `📓️ui-turn-patch-batching-2026-09-15.md` §7.1 names: 18 of 25
/// measured boot turns had exactly ONE surface ready while the wire could already carry sixteen.
#[test]
fn a_ready_output_is_not_blocked_by_another_allocations_running_job() {
    let own = instance_lifetime::NativeCloseKey::fixture(71, 1);
    let other = instance_lifetime::NativeCloseKey::fixture(71, 2);
    let admissible = |ready_generation: u64, in_flight: &[(instance_lifetime::NativeCloseKey, u64)]| patches::ready_output_is_admissible(own, ready_generation, in_flight.iter().copied());
    assert!(admissible(5, &[]), "nothing in flight admits the output");
    assert!(!admissible(5, &[(own, 4)]), "the SAME allocation's older work still holds its own output back");
    assert!(admissible(5, &[(own, 5)]), "the same allocation's equal generation is this output's own producer, not an earlier one");
    assert!(admissible(5, &[(own, 6)]), "the same allocation's NEWER work never holds an older output back");
    assert!(admissible(5, &[(other, 1)]), "another allocation's older job must not serialize this one — the widening");
    assert!(admissible(5, &[(other, 1), (other, 2), (other, 3)]), "no number of foreign allocations gates this output");
    assert!(!admissible(5, &[(other, 1), (own, 4)]), "a foreign allocation never masks this allocation's own earlier work");
    for ready_generation in 0..8u64 {
        for in_flight_generation in 0..8u64 {
            assert_eq!(admissible(ready_generation, &[(own, in_flight_generation)]), in_flight_generation >= ready_generation, "own g{in_flight_generation} against ready g{ready_generation}");
            assert!(admissible(ready_generation, &[(other, in_flight_generation)]), "foreign g{in_flight_generation} against ready g{ready_generation}");
        }
    }
    eprintln!("[DEBUG] more-work-drive readiness gate: per-allocation, 64 generation pairs checked on both sides");
}

/// 🎚️ The widened gate through the PRODUCTION ladder: eight surfaces driven one reconcile opportunity
/// per turn each publish exactly once and in strictly increasing revision order on their own
/// publication line. `overlapped` is printed rather than asserted — the drive extracts a ready output
/// inside the same opportunity that produced it, so the state the law above tests directly is rarely
/// observable from outside; what this law adds is that the widened gate did not reorder or lose a
/// publication on the real ladder.
#[test]
fn the_widened_readiness_gate_keeps_every_surfaces_own_publication_order() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let instance = 71u32;
    let tracker = patches::PatchTracker::new();
    for index in 0..8 {
        tracker.begin(format!("{instance}:surface-{index}"), tree("body")).expect("mounted admission");
    }
    let mut overlapped = 0usize;
    let mut revisions: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    let mut published = 0usize;
    let mut sequence = 0u64;
    let mut turns = 0usize;
    for _ in 0..8_192 {
        turns += 1;
        if let Some((key, generation)) = tracker.ready_patch_key().expect("ready key") {
            if tracker.in_flight_generations().iter().any(|(other, other_generation)| *other != key && *other_generation < generation) {
                overlapped += 1;
            }
        }
        drive_reconcile_within(&tracker, 1, far_deadline()).expect("one-opportunity reconcile drive");
        with_pending(|pending| {
            let mut pending = pending.borrow_mut();
            for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
                if pending.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("bounded retirement") {
                    break;
                }
            }
        });
        while let Some(patch) = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication") {
            sequence += 1;
            published += 1;
            let surface = patch.surface.0.to_string();
            let revision = patch.revision.0;
            let previous = revisions.insert(surface.clone(), revision);
            assert!(!matches!(previous, Some(earlier) if earlier >= revision), "surface {surface} published revision {revision} after {previous:?}");
            let receipt = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: instance, guest_lifetime: 1 }, patch_sequence: sequence };
            with_pending(|pending| pending.borrow_mut().stage_emission(receipt, std::iter::once(&patch))).expect("staged emission");
            with_pending(|pending| pending.borrow_mut().commit_emission());
            assert!(with_pending(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| tracker.mark_published_ack(ack))).expect("issued ack"));
            retire_host_patch(patch);
        }
        if published == 8 && !reconcile_arms_turn(&tracker) {
            break;
        }
    }
    eprintln!("[DEBUG] more-work-drive readiness ladder turns={turns} published={published} overlapped={overlapped} surfaces={}", revisions.len());
    assert_eq!(published, 8, "every mounted surface must publish exactly once");
    assert_eq!(revisions.len(), 8, "every mounted surface must be its own publication line");
    drain_pending_authority();
}

/// 🚚️ The production ladder under the drive: the same eight surfaces, driven one reconcile
/// opportunity per turn — the shape a wall-bound turn really has — cost strictly fewer host crossings
/// when the worker owns the pump, and not one of those crossings carries nothing.
#[test]
fn the_production_reconcile_ladder_costs_fewer_crossings_when_the_worker_owns_the_pump() {
    let host_polled = settle_one_opportunity_per_turn(72, 8, 1);
    let worker_driven = settle_one_opportunity_per_turn(73, 8, declared_steps());
    eprintln!(
        "[DEBUG] more-work-drive ladder host-polled crossings={} turns={} idle={} | worker-driven crossings={} turns={} idle={}",
        host_polled.crossings, host_polled.turns, host_polled.idle, worker_driven.crossings, worker_driven.turns, worker_driven.idle
    );
    assert_eq!(host_polled.published, 8);
    assert_eq!(worker_driven.published, 8);
    assert!(worker_driven.crossings * (declared_steps() - 1) <= host_polled.crossings, "the worker drive cost {} crossings against the host-polled {} at a ceiling of {}", worker_driven.crossings, host_polled.crossings, declared_steps());
    assert!(worker_driven.idle < host_polled.idle / declared_steps() + 1, "a worker-driven settle spent {} crossings carrying nothing against the host-polled {}", worker_driven.idle, host_polled.idle);
    assert_eq!(worker_driven.turns, host_polled.turns, "the drive moves turns off the wire, it does not remove them");
}

/// 🧾️ A page CUT SHORT by its byte budget must leave nothing unstaged, lose nothing and duplicate
/// nothing — the trap a large surface hits and a small one never does.
///
/// 🐛️ `take_turn_patch_page` returns the patch that did not fit through `hand_back_turn`, which puts
/// it back INTO its borrowed cell. `stage_emission` counted every borrowed cell against the page's
/// entries, so a cut page always had one more borrowed cell than entries and the whole turn faulted
/// `plugin.reactor-close-authority: pending patch emission left a borrowed publication unstaged`.
/// generation3d's surfaces never cut a page; puzzle 3d's world-3d surface cut every one, and its
/// plugin trapped on `setActiveExample`/`windowResize` with nothing mounted (reported by
/// INTERACTIVE-TOOLS-VISIBLE-PROCESS on :6013, 2026-09-15). The cut page's own laws
/// (`🧺️turn-patch-batch`) never saw it because they drive `take_one`/`hand_back_turn` directly and
/// never stage a receipt against a cut page.
#[test]
fn a_page_cut_by_its_byte_budget_stages_exactly_what_it_delivered_and_loses_nothing() {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let instance = 74u32;
    let tracker = patches::PatchTracker::new();
    for index in 0..4 {
        tracker.begin(format!("{instance}:surface-{index}"), tree("body")).expect("mounted admission");
    }
    for _ in 0..1_000_000 {
        drive_reconcile_within(&tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
        if !tracker.has_drivable_work() {
            break;
        }
    }
    let unit = {
        let probe = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication").expect("one queued publication");
        let bytes = semio_framework::kernel::ui_patch_turn_bytes(&probe);
        with_pending(|pending| pending.borrow_mut().hand_back_turn(probe)).expect("a probed patch returns to its own cell");
        bytes
    };
    let mut delivered: Vec<(String, u64)> = Vec::new();
    let mut sequence = 0u64;
    let mut pages = 0usize;
    let mut cuts = 0usize;
    for _ in 0..64 {
        // 🧾️ A budget of exactly ONE patch: every page after the first entry is cut, which is the
        // shape a world-3d surface produces against the real per-lane budget.
        let page = turn_patch_page_of_two(unit);
        if page.is_empty() {
            break;
        }
        pages += 1;
        assert!(page.len() <= 2, "a one-unit budget admits its first patch and at most one more");
        if with_pending(|pending| (0..semio_framework::kernel::UI_TURN_PATCHES_MAXIMUM).any(|cell| pending.borrow().borrowed_patch(cell).is_some())) {
            cuts += 1;
        }
        sequence += 1;
        let receipt = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: instance, guest_lifetime: 1 }, patch_sequence: sequence };
        with_pending(|pending| pending.borrow_mut().stage_emission(receipt, page.iter())).expect("a cut page stages exactly what it delivered");
        with_pending(|pending| pending.borrow_mut().commit_emission());
        for patch in page {
            delivered.push((patch.surface.0.to_string(), patch.revision.0));
            let surface = patch.surface.0.to_string();
            let revision = patch.revision.0;
            assert!(with_pending(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| tracker.mark_published_ack(ack))).expect("issued ack"));
            retire_host_patch(patch);
        }
        with_pending(|pending| {
            let mut pending = pending.borrow_mut();
            for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
                if pending.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("bounded retirement") {
                    break;
                }
            }
        });
        drive_reconcile_within(&tracker, reconcile_step_opportunities(u64::MAX), far_deadline()).expect("reconcile drive");
    }
    eprintln!("[DEBUG] more-work-drive cut-page unit={unit}B pages={pages} cuts={cuts} delivered={delivered:?}");
    assert_eq!(delivered.len(), 4, "every mounted surface must be delivered exactly once across the cut pages");
    let mut surfaces: Vec<&String> = delivered.iter().map(|(surface, _)| surface).collect();
    surfaces.sort();
    surfaces.dedup();
    assert_eq!(surfaces.len(), 4, "a cut page must not duplicate a publication: {delivered:?}");
    assert!(cuts >= 1, "the fixture must actually CUT a page — a page that fits never exercises the handback that caused the trap");
    assert!(pages >= 2, "the fixture must actually cut at least one page, not deliver everything in one");
    drain_pending_authority();
}

/// 🧾️ One turn's page under a byte budget that admits its first entry and at most one more — the
/// production ladder (`take_turn_patch_page`) with the budget dialled down to a measured patch unit.
fn turn_patch_page_of_two(unit_bytes: usize) -> Vec<ui_contract::UiPatch> {
    let page = super::turn::take_turn_patch_page(unit_bytes.saturating_add(unit_bytes / 2)).expect("turn patch page");
    page.into_iter().collect()
}

#[derive(Default)]
struct DriveLedger {
    crossings: usize,
    turns: usize,
    idle: usize,
    published: usize,
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

/// 🔁️ `settlePluginTurn` with the worker drive inside it: one crossing carries the acknowledgements
/// the previous crossing earned, then the worker runs up to `steps` guest turns without crossing and
/// hands back the first one that carried something. `steps = 1` is the host-polled shape this lane
/// replaced.
fn settle_one_opportunity_per_turn(instance: u32, surfaces: usize, steps: usize) -> DriveLedger {
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = patches::PatchTracker::new();
    for index in 0..surfaces {
        tracker.begin(format!("{instance}:surface-{index}"), tree("body")).expect("mounted admission");
    }
    let mut ledger = DriveLedger::default();
    let mut acknowledgements: Vec<(ActorUiPatchReceipt, String, u64)> = Vec::new();
    let sequence = Cell::new(0u64);
    let quiet = Cell::new(false);
    let alive = Cell::new(true);
    loop {
        ledger.crossings += 1;
        assert!(ledger.crossings < 4_096, "the ladder never quiesced: {} | {}", tracker.debug_state(), with_pending(|pending| pending.borrow().debug_state()));
        let carried_events = !acknowledgements.is_empty();
        for (receipt, surface, revision) in acknowledgements.drain(..) {
            assert!(with_pending(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| tracker.mark_published_ack(ack))).expect("issued ack"));
        }
        let issued: std::cell::RefCell<Vec<(ActorUiPatchReceipt, String, u64)>> = std::cell::RefCell::new(Vec::new());
        let turns = Cell::new(0usize);
        let armed = Cell::new(true);
        let (_spent, _stop) = drive_more_work(
            steps,
            || {
                turns.set(turns.get() + 1);
                with_pending(|pending| {
                    let mut pending = pending.borrow_mut();
                    for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
                        if pending.close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("bounded retirement") {
                            break;
                        }
                    }
                });
                drive_reconcile_within(&tracker, 1, far_deadline()).expect("one-opportunity reconcile drive");
                let mut carried = false;
                while let Some(patch) = with_pending(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES)).expect("publication") {
                    sequence.set(sequence.get() + 1);
                    carried = true;
                    let receipt = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: instance, guest_lifetime: 1 }, patch_sequence: sequence.get() };
                    with_pending(|pending| pending.borrow_mut().stage_emission(receipt, std::iter::once(&patch))).expect("staged emission");
                    with_pending(|pending| pending.borrow_mut().commit_emission());
                    issued.borrow_mut().push((receipt, patch.surface.0.to_string(), patch.revision.0));
                    retire_host_patch(patch);
                }
                armed.set(reconcile_arms_turn(&tracker));
                TurnOutcome { carried, arms: armed.get() }
            },
            &quiet,
            &alive,
        );
        ledger.turns += turns.get();
        let published_here = issued.borrow().len();
        ledger.published += published_here;
        acknowledgements.extend(issued.into_inner());
        if !carried_events && published_here == 0 {
            ledger.idle += 1;
        }
        if !armed.get() && acknowledgements.is_empty() {
            break;
        }
    }
    drain_pending_authority();
    ledger
}
