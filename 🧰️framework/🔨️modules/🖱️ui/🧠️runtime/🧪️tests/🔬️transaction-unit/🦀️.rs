
use super::*;
use std::cell::Cell;

//#region 🔖️Fixtures

fn surface(value: &str) -> ui_contract::SurfaceId {
    ui_contract::SurfaceId::try_from(value).expect("bounded fixture surface")
}

fn ui_text(value: &str) -> ui_contract::UiText {
    ui_contract::UiText::try_from_str(value).expect("bounded fixture text")
}

struct Model {
    value: i32,
}

struct FakePresenter {
    model: crate::Entity<Model>,
    count: i32,
}

impl crate::Present for FakePresenter {
    fn present(&self, cx: &mut crate::PresentCx<'_>) -> crate::ComponentTree {
        let model = cx.read(&self.model);
        let label = format!("{}:{}", self.count, model.value);
        crate::ComponentTree::new(
            crate::TreeNode::try_new("root", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from(label).expect("bounded fixture label"), emphasize: None, data_attributes: None })).expect("bounded fixture node"),
        )
    }
}

impl crate::HandleIntent for FakePresenter {
    fn on_intent(&mut self, intent: &ui_contract::UiIntent, cx: &mut crate::Context<'_, Self>) -> crate::DispatchOutcome {
        match intent.trigger {
            ui_contract::Trigger::Delta => {
                self.count += 1;
                cx.notify();
                crate::DispatchOutcome::HandledWith { commands: vec![test_command(intent.seq)], deferred: vec![] }
            }
            ui_contract::Trigger::Commit => {
                cx.notify();
                crate::DispatchOutcome::HandledWith {
                    commands: vec![],
                    deferred: vec![crate::DeferredOp::PublishPresence(ui_contract::PresenceUpdate {
                        surface: intent.surface.clone(),
                        node_key: intent.node_key.to_string(),
                        own: ui_contract::OwnPresence { hovered: true, ..Default::default() },
                        peers: vec![],
                        ttl_ms: 4_000,
                    })],
                }
            }
            _ => crate::DispatchOutcome::Unhandled,
        }
    }
}

struct FakeDelta {
    target: crate::Entity<Model>,
    value: i32,
}

impl crate::ProjectionDelta for FakeDelta {
    type Key = crate::EntityId;
    fn key(&self) -> crate::EntityId {
        self.target.id()
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn apply_fake_delta(store: &mut crate::EntityStore, delta: FakeDelta) {
    let FakeDelta { target, value } = delta;
    store.update(&target, |model, cx| {
        model.value = value;
        cx.notify();
    });
}

struct AlwaysAcceptsSink;
impl crate::CommandSink for AlwaysAcceptsSink {
    fn try_send(&self, _command: crate::Command) -> Result<(), crate::SinkFull> {
        Ok(())
    }
}

struct FailsAfterSink {
    calls: Cell<u32>,
    accepts: u32,
}
impl crate::CommandSink for FailsAfterSink {
    fn try_send(&self, _command: crate::Command) -> Result<(), crate::SinkFull> {
        let seen = self.calls.get();
        self.calls.set(seen + 1);
        if seen < self.accepts { Ok(()) } else { Err(crate::SinkFull) }
    }
}

fn test_command(seq: u64) -> crate::Command {
    crate::Command { id: crate::CommandId(seq), correlation: crate::CorrelationId(seq), payload: ui_contract::UiValue::Null }
}

fn test_intent(surface: ui_contract::SurfaceId, node: &crate::Entity<FakePresenter>, revision: ui_contract::UiRevision, trigger: ui_contract::Trigger, seq: u64) -> ui_contract::UiIntent {
    ui_contract::UiIntent { surface, revision, node: ui_contract::UiNodeId(node.id().0), node_key: ui_text("root"), trigger, action: ui_contract::ActionId::try_v1("test", "act").expect("bounded fixture action"), args: None, input: None, seq }
}

fn test_runtime() -> UiRuntime<AlwaysAcceptsSink, FakeDelta> {
    UiRuntime::new(crate::CommandGateway::new(10, AlwaysAcceptsSink), 16, apply_fake_delta)
}

fn register_test_surface<S: crate::CommandSink>(runtime: &mut UiRuntime<S, FakeDelta>, surface: ui_contract::SurfaceId) -> (crate::Entity<FakePresenter>, crate::Entity<Model>) {
    let model = runtime.store_mut().insert(Model { value: 0 });
    let presenter = runtime.store_mut().insert(FakePresenter { model: model.clone(), count: 0 });
    runtime.register_surface(surface.clone(), presenter.clone(), crate::SurfaceReconciler::new(surface));
    (presenter, model)
}

fn step_once<S: crate::CommandSink, D: crate::ProjectionDelta>(transaction: &mut FrameTransaction, runtime: &mut UiRuntime<S, D>, fuel: u64) -> FrameTransactionStep {
    fn clock() -> Option<u64> {
        Some(0)
    }
    let mut preview_sequence = 0;
    let mut cx = semio_framework_job::StepContext::new(
        semio_framework_job::allocate_operation_id(),
        semio_framework_job::Generation(0),
        semio_framework_job::StepBudget::new(fuel, u64::MAX),
        semio_framework_job::CancelToken::root_now(),
        clock,
        &mut preview_sequence,
    );
    transaction.step(runtime, &mut cx)
}

fn drive_stepped<S: crate::CommandSink, D: crate::ProjectionDelta>(transaction: &mut FrameTransaction, runtime: &mut UiRuntime<S, D>, fuel: u64) -> (Transacted, usize) {
    let mut yields = 0;
    loop {
        match step_once(transaction, runtime, fuel) {
            FrameTransactionStep::Yield { .. } => yields += 1,
            FrameTransactionStep::Published(output) => return (output, yields),
            FrameTransactionStep::Cancelled(_) => unreachable!("live test token"),
        }
    }
}

//#endregion 🔖️Fixtures

//#region 🔖️IntentMutatesAndPatches
#[test]
fn an_intent_mutates_entity_state_and_the_following_transact_emits_a_patch() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let surface = surface("s");
    let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
    runtime.transact(0); // 🌱️ unconditional first present, baseline revision 1

    runtime.submit_intent(test_intent(surface.clone(), &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Delta, 1));
    let transacted = runtime.transact(0);

    assert_eq!(transacted.patches.len(), 1);
    assert_eq!(transacted.patches[0].surface, surface);
    assert!(!transacted.patches[0].ops.is_empty());
    assert_eq!(transacted.commands, vec![test_command(1)]);
}
//#endregion 🔖️IntentMutatesAndPatches

//#region 🔖️StaleIntentDropped
#[test]
fn a_stale_revision_intent_is_dropped_and_produces_no_patch_and_no_command() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let surface = surface("s");
    let (presenter, model) = register_test_surface(&mut runtime, surface.clone());
    runtime.transact(0); // revision 1

    // 🐌️ advance the surface's revision independently of any intent, so the gap exceeds tolerance
    runtime.store_mut().update(&model, |m, cx| {
        m.value = 1;
        cx.notify();
    });
    runtime.transact(0); // revision 2
    runtime.store_mut().update(&model, |m, cx| {
        m.value = 2;
        cx.notify();
    });
    let bumped = runtime.transact(0); // revision 3
    assert_eq!(bumped.patches.len(), 1);

    runtime.submit_intent(test_intent(surface, &presenter, ui_contract::UiRevision(0), ui_contract::Trigger::Delta, 99));
    let transacted = runtime.transact(0);

    assert!(transacted.patches.is_empty(), "a stale intent must never reach the reconciler as a patch");
    assert!(transacted.commands.is_empty(), "a stale intent's handler must never run, so its command must never appear");
}
//#endregion 🔖️StaleIntentDropped

//#region 🔖️BulkCoalescing
#[test]
fn a_bulk_projection_update_touching_one_surface_many_times_yields_exactly_one_patch() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let surface = surface("s");
    let (_presenter, model) = register_test_surface(&mut runtime, surface);
    runtime.transact(0); // baseline

    for value in 1..=5 {
        runtime.push_delta(FakeDelta { target: model.clone(), value }).expect("fits: same key coalesces");
    }
    let transacted = runtime.transact(0);

    assert_eq!(transacted.patches.len(), 1, "a burst of same-key deltas must still yield exactly one patch");
}
//#endregion 🔖️BulkCoalescing

//#region 🔖️UnreadEntityProducesNoPatch
#[test]
fn an_entity_notified_but_not_read_by_any_surface_produces_no_patch() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let surface = surface("s");
    register_test_surface(&mut runtime, surface);
    runtime.transact(0); // baseline: establishes the surface's real read set

    let untracked = runtime.store_mut().insert(0i32);
    runtime.store_mut().update(&untracked, |value, cx| {
        *value += 1;
        cx.notify();
    });
    let transacted = runtime.transact(0);

    assert!(transacted.patches.is_empty(), "an entity no surface reads must never dirty a present");
}
//#endregion 🔖️UnreadEntityProducesNoPatch

//#region 🔖️EffectStorm
#[test]
fn the_effect_fixpoint_terminates_and_a_pathological_observer_hits_the_storm_budget() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let entity = runtime.store_mut().insert(0i32);
    let looping = entity.clone();
    let _subscription = runtime.store_mut().update(&entity, |_, cx| cx.observe(&looping, |_, cx2| cx2.notify()));
    runtime.store_mut().update(&entity, |_, cx| cx.notify());

    let transacted = runtime.transact(0);

    assert_eq!(transacted.faults.len(), 1);
    match &transacted.faults[0] {
        TransactFault::EffectStorm { cycles, pending_notify, .. } => {
            assert_eq!(*cycles, EFFECT_STORM_BUDGET);
            assert!(pending_notify.contains(&entity.id()), "the fault must name the still-looping entity");
        }
        TransactFault::CreditsExceeded { .. } => panic!("default credits must not be exhausted"),
    }
}
//#endregion 🔖️EffectStorm

//#region 🔖️GatewayBackpressure
#[test]
fn a_full_command_mailbox_surfaces_backpressure_without_blocking_the_transaction() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime: UiRuntime<FailsAfterSink, FakeDelta> = UiRuntime::new(crate::CommandGateway::new(10, FailsAfterSink { calls: Cell::new(0), accepts: 1 }), 16, apply_fake_delta);
    let surface = surface("s");
    let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
    runtime.transact(0);

    runtime.submit_intent(test_intent(surface.clone(), &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Delta, 1));
    runtime.submit_intent(test_intent(surface, &presenter, ui_contract::UiRevision(2), ui_contract::Trigger::Delta, 2));
    let transacted = runtime.transact(0);

    assert_eq!(transacted.commands.len(), 1, "only the sink-accepted command should be reported, without the transaction blocking or panicking");
}
//#endregion 🔖️GatewayBackpressure

//#region 🔖️NextWake
#[test]
fn next_wake_ms_is_none_when_idle_and_some_earliest_when_a_deadline_is_pending() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    assert_eq!(runtime.transact(0).next_wake_ms, None);

    runtime.request_wake(500);
    runtime.request_wake(200);
    assert_eq!(runtime.transact(100).next_wake_ms, Some(200));
    assert_eq!(runtime.transact(250).next_wake_ms, Some(500));
    assert_eq!(runtime.transact(9_999).next_wake_ms, None);
}
//#endregion 🔖️NextWake

//#region 🔖️PresenceOwnChannel
#[test]
fn presence_flushes_on_its_own_channel_and_never_appears_in_a_patch() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let surface = surface("s");
    let (presenter, _model) = register_test_surface(&mut runtime, surface.clone());
    runtime.transact(0);

    runtime.submit_intent(test_intent(surface, &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Commit, 7));
    let transacted = runtime.transact(0);

    assert_eq!(transacted.presence.len(), 1);
    assert!(transacted.presence[0].own.hovered);
    assert!(transacted.patches.is_empty(), "a Commit trigger that only publishes presence must not itself produce a document patch");
}
//#endregion 🔖️PresenceOwnChannel

//#region 🔖️IndependentSurfaces
#[test]
fn two_surfaces_are_independent_dirtying_one_does_not_re_present_the_other() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let surface_a = surface("a");
    let surface_b = surface("b");
    let (_presenter_a, model_a) = register_test_surface(&mut runtime, surface_a.clone());
    let (_presenter_b, _model_b) = register_test_surface(&mut runtime, surface_b);
    let baseline = runtime.transact(0);
    assert_eq!(baseline.patches.len(), 2, "both freshly registered surfaces present once");

    runtime.store_mut().update(&model_a, |m, cx| {
        m.value = 42;
        cx.notify();
    });
    let transacted = runtime.transact(0);

    assert_eq!(transacted.patches.len(), 1);
    assert_eq!(transacted.patches[0].surface, surface_a);
}
//#endregion 🔖️IndependentSurfaces

//#region 🔖️ResumableStress
#[test]
fn one_fuel_slices_bound_an_intent_storm_and_preserve_fifo_output() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime: UiRuntime<AlwaysAcceptsSink, FakeDelta> = UiRuntime::new(crate::CommandGateway::new(128, AlwaysAcceptsSink), 16, apply_fake_delta);
    let surface = surface("s");
    let (presenter, _) = register_test_surface(&mut runtime, surface.clone());
    runtime.transact(0);
    for seq in 0..32 {
        runtime.submit_intent(test_intent(surface.clone(), &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Delta, seq));
    }

    let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
    let (output, yields) = drive_stepped(&mut transaction, &mut runtime, 1);

    assert!(yields >= 64, "routing and command submission must each yield as independent units");
    assert_eq!(output.commands, (0..32).map(test_command).collect::<Vec<_>>());
}

#[test]
fn an_effect_storm_remains_resumable_and_retains_the_cycle_fault_semantics() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let entity = runtime.store_mut().insert(0i32);
    let looping = entity.clone();
    let _subscription = runtime.store_mut().update(&entity, |_, cx| cx.observe(&looping, |_, cx2| cx2.notify()));
    runtime.store_mut().update(&entity, |_, cx| cx.notify());

    let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
    let (output, yields) = drive_stepped(&mut transaction, &mut runtime, 1);

    assert!(yields >= EFFECT_STORM_BUDGET as usize);
    assert!(matches!(output.faults.as_slice(), [TransactFault::EffectStorm { cycles: EFFECT_STORM_BUDGET, .. }]));
}

#[test]
fn repeated_new_input_supersedes_staged_presentation_without_losing_an_accepted_command() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    let surface = surface("s");
    let (presenter, model) = register_test_surface(&mut runtime, surface.clone());
    runtime.transact(0);
    runtime.submit_intent(test_intent(surface.clone(), &presenter, ui_contract::UiRevision(1), ui_contract::Trigger::Delta, 7));

    let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
    while transaction.stage() < FrameTransactionStage::ReconcileTree {
        assert!(matches!(step_once(&mut transaction, &mut runtime, 1), FrameTransactionStep::Yield { .. }));
    }
    for value in 1..=8 {
        runtime.push_delta(FakeDelta { target: model.clone(), value }).expect("coalesced resize-style input");
        assert!(matches!(step_once(&mut transaction, &mut runtime, 1), FrameTransactionStep::Yield { .. }));
    }
    runtime.push_delta(FakeDelta { target: model, value: 99 }).expect("latest input");
    let (output, _) = drive_stepped(&mut transaction, &mut runtime, 1);

    assert_eq!(output.commands, vec![test_command(7)]);
    assert_eq!(output.patches.len(), 1);
    let snapshot = runtime.surfaces.get(&surface).expect("surface").reconciler.snapshot();
    assert!(serde_json::to_string(&snapshot).expect("snapshot json").contains("1:99"), "only the newest projection may be published");
}

#[test]
fn cancellation_discards_an_active_node_cursor_without_advancing_the_surface_revision() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    fn clock() -> Option<u64> {
        Some(0)
    }
    let mut runtime = test_runtime();
    let surface = surface("s");
    register_test_surface(&mut runtime, surface.clone());
    let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
    while transaction.active_reconcile.is_none() {
        assert!(matches!(step_once(&mut transaction, &mut runtime, 1), FrameTransactionStep::Yield { .. }));
    }
    assert_eq!(runtime.surfaces.get(&surface).expect("surface").current_revision(), ui_contract::UiRevision(0));

    let token = semio_framework_job::CancelToken::root_now();
    token.cancel_now();
    let mut preview_sequence = 0;
    let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), token, clock, &mut preview_sequence);
    assert!(matches!(transaction.step(&mut runtime, &mut cx), FrameTransactionStep::Cancelled(_)));
    assert_eq!(runtime.surfaces.get(&surface).expect("surface").current_revision(), ui_contract::UiRevision(0));

    let (output, _) = drive_stepped(&mut transaction, &mut runtime, 1);
    assert_eq!(output.patches.len(), 1, "a later live slice must re-present the discarded candidate");
    assert_eq!(output.patches[0].revision, ui_contract::UiRevision(1));
}

#[test]
fn deterministic_surface_order_is_independent_of_hash_map_insertion_order() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    fn build(order: [&str; 2]) -> UiRuntime<AlwaysAcceptsSink, FakeDelta> {
        let mut runtime = test_runtime();
        for surface in order {
            register_test_surface(&mut runtime, ui_contract::SurfaceId::try_from(surface).expect("bounded fixture surface"));
        }
        runtime
    }

    let mut left = build(["b", "a"]);
    let mut right = build(["a", "b"]);
    let (left_output, _) = drive_stepped(&mut FrameTransaction::new(FrameTransactionLimits::default()), &mut left, 1);
    let (right_output, _) = drive_stepped(&mut FrameTransaction::new(FrameTransactionLimits::default()), &mut right, 1);

    assert_eq!(serde_json::to_string(&left_output.patches).unwrap(), serde_json::to_string(&right_output.patches).unwrap());
    assert_eq!(left_output.patches.iter().map(|patch| patch.surface.0.as_str()).collect::<Vec<_>>(), vec!["a", "b"]);
}

#[test]
fn an_expired_wall_clock_budget_returns_before_consuming_input() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    fn clock() -> Option<u64> {
        Some(10)
    }
    let mut runtime = test_runtime();
    let target = runtime.store_mut().insert(Model { value: 0 });
    runtime.push_delta(FakeDelta { target, value: 1 }).expect("delta");
    let mut transaction = FrameTransaction::new(FrameTransactionLimits::default());
    let mut preview_sequence = 0;
    let mut cx =
        semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, 10), semio_framework_job::CancelToken::root_now(), clock, &mut preview_sequence);

    assert!(matches!(transaction.step(&mut runtime, &mut cx), FrameTransactionStep::Yield { usage: FrameTransactionUsage { items: 0, .. }, .. }));
    assert_eq!(runtime.inbox.len(), 1);
}

#[test]
fn transaction_canonical_job_preserves_independent_node_credit() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔄️transaction/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut runtime = test_runtime();
        register_test_surface(&mut runtime, surface("node-credit"));
        let mut transaction =
            FrameTransaction::new(FrameTransactionLimits { max_items: fixture["maximumItems"].as_u64().unwrap() as usize, max_nodes: row["maximumNodes"].as_u64().unwrap() as usize, max_bytes: fixture["maximumBytes"].as_u64().unwrap() as usize });
        let (output, _) = drive_stepped(&mut transaction, &mut runtime, 1);
        assert_eq!(output.patches.len(), row["patches"].as_u64().unwrap() as usize);
        assert_eq!(output.faults.iter().any(|fault| matches!(fault, TransactFault::CreditsExceeded { .. })), row["creditFault"].as_bool().unwrap());
    }
}

#[test]
fn hard_credits_fault_before_any_candidate_snapshot_is_published() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut runtime = test_runtime();
    register_test_surface(&mut runtime, surface("s"));
    let mut transaction = FrameTransaction::new(FrameTransactionLimits { max_items: 0, max_nodes: 0, max_bytes: 0 });
    let (output, _) = drive_stepped(&mut transaction, &mut runtime, 1);

    assert!(output.patches.is_empty());
    assert!(matches!(output.faults.as_slice(), [TransactFault::CreditsExceeded { .. }]));
    assert_eq!(runtime.surfaces.get(&surface("s")).unwrap().current_revision(), ui_contract::UiRevision(0));
}
//#endregion 🔖️ResumableStress
