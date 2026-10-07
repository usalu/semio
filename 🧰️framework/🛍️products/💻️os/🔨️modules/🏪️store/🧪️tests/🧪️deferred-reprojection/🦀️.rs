//! 🐢️ Store laws of the deferred remote reprojection (G9, design §16.6, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING):
//! with a replay budget, the Report replay a remote supersession needs runs as a resumable job — the replica keeps showing
//! the history before the change, progress grows monotonically, cancelling at any step leaves the store untouched, a local
//! edit meanwhile restarts the replay, and the adoption equals the undeferred one for every case of the language-agnostic
//! supersede-replay corpus (whose independent fast-json-patch oracle is `🧪️supersede-replay/🟦️.ts`).
use super::*;
use semio_framework_diagnostic::FaultFrom;
use std::collections::HashSet;

//#region 🧰️Harness
fn set(n: i32) -> DemoMutation {
    DemoMutation::SetN(SetN { n })
}

fn add(delta: i32) -> DemoMutation {
    DemoMutation::AddN(AddN { delta })
}

async fn store_named(id: &str, n: Option<i32>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, DemoSnapshot { n }, None)).await
}

/// 🌱️ The author of `edits` and a replica holding its whole log.
async fn authored(id: &str, n: Option<i32>, edits: Vec<Vec<DemoMutation>>) -> (ArtifactStore<DemoSnapshot, DemoMutation>, Vec<crate::os_spr::MutationEnvelope>) {
    let mut author = store_named(id, n).await;
    for mutations in edits {
        author.dispatch(ArtifactCommand::Apply { mutations, transaction: None }).await.expect("a clean edit applies");
    }
    let log = author.event_log().expect("log");
    (author, log)
}

async fn replica(id: &str, n: Option<i32>, log: &[crate::os_spr::MutationEnvelope], budget: Option<usize>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    let mut store = ArtifactStore::new_with_actor(create_document_envelope("demo/v1", id, DemoSnapshot { n }, None), ActorId("deferred".into())).await;
    for event in log {
        store.ingest_remote(event.clone()).await.expect("the replica takes the log");
    }
    store.defer_remote_replays(budget.map(ReplayTurnBudget::operations));
    store
}

/// ✏️ One remote `Supersede` of `inputs`, authored by another replica far after every edit.
fn remote_supersede(id: &str, inputs: Vec<protocol::SupersededInput>, logical: u64) -> crate::os_spr::MutationEnvelope {
    let dependencies = inputs.iter().map(|input| input.target.clone()).collect();
    let transition = crate::os_spr::HistoryTransition::Supersede(protocol::TransitionSupersede { scope: None, inputs });
    crate::os_spr::history_transition_envelope(&transition, &ArtifactId(id.to_string()), &ActorId("editor".into()), dependencies, HybridLogicalTimestamp { actor: 9, physical_ms: u64::MAX / 2, logical })
}

fn replaced(target: &MutationId, operation: DemoMutation) -> protocol::SupersededInput {
    protocol::SupersededInput { target: target.clone(), replacement: protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: operation.encode_op().expect("demo operations encode") } }
}

/// 📸️ What a replica shows: its projection, revision, log, effective supersessions and durable outcomes.
fn view(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> (Option<i32>, [u8; 32], Vec<MutationId>, Vec<MutationId>, Vec<(MutationId, Option<semio_framework_diagnostic::Severity>)>) {
    let log = store.event_log().expect("log").into_iter().map(|event| event.mutation_id).collect();
    let supersessions = store.supersessions().keys().cloned().collect();
    let outcomes = store.mutation_outcomes().expect("outcomes").into_iter().map(|outcome| (outcome.mutation_id, outcome.worst)).collect();
    (store.snapshot_ref().n, store.content_revision_now(), log, supersessions, outcomes)
}

/// ⏭️ Steps the deferred reprojection to its adoption, checking that progress only grows within one replay run.
async fn drive(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, budget: usize) -> usize {
    let mut last = store.reprojection_progress().expect("a deferred change waits");
    let mut steps = 0;
    while let Some(progress) = store.step_reprojection(None).await.expect("a deferred step") {
        assert!(progress.done > last.done && progress.done - last.done <= budget as u32, "progress grows by at most one budget: {last:?} -> {progress:?}");
        assert!(progress.done <= progress.total && (last.total == 0 || progress.total == last.total), "{last:?} -> {progress:?}");
        last = progress;
        steps += 1;
        assert!(steps < 10_000, "the deferred replay finishes");
    }
    assert!(store.reprojection_progress().is_none(), "nothing waits after the adoption");
    steps
}
//#endregion 🧰️Harness

//#region 🧪️DeferredReprojectionLaws
/// 🧫️ For every case of the supersede-replay corpus, a replica that defers the replay of the remote supersession (one
/// operation per turn) keeps showing the history before it while it waits, then adopts exactly what a replica replaying
/// inside the ingest adopted — the corpus' expected state, the same revision, log, supersessions and outcomes.
#[semio_framework_async_macros::async_test]
async fn a_deferred_remote_supersession_adopts_what_an_undeferred_one_adopts() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️supersede-replay/🔣️.json")).expect("corpus parses");
    let hex = |text: &str| text.as_bytes().chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("ascii hex"), 16).expect("hex pair")).collect::<Vec<u8>>();
    let mut deferred_cases = 0;
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let initial: Option<i32> = serde_json::from_value(case["initial"]["n"].clone()).expect("initial n");
        let edits = case["edits"].as_array().expect("edits").iter().map(|edit| edit.as_array().expect("edit").iter().map(|operation| DemoMutation::from_value(operation.clone().into()).expect("corpus operation")).collect()).collect();
        let (author, log) = authored(name, initial, edits).await;
        let operations: Vec<(usize, u32, MutationId)> = author.mutation_ops().expect("operations").into_iter().map(|operation| (operation.position, operation.op_index, operation.mutation_id)).collect();
        let inputs: Vec<protocol::SupersededInput> = case["supersessions"]
            .as_array()
            .expect("supersessions")
            .iter()
            .map(|supersession| {
                let target =
                    operations.iter().find(|(position, index, _)| *position as u64 == supersession["edit"].as_u64().unwrap() && u64::from(*index) == supersession["op"].as_u64().unwrap()).map(|(_, _, id)| id.clone()).expect("a recorded operation");
                let replacement = match &supersession["replacement"] {
                    serde_json::Value::String(withdrawn) if withdrawn == "withdrawn" => protocol::InputReplacement::Withdrawn,
                    serde_json::Value::Object(input) if input.contains_key("invalid") => {
                        protocol::InputReplacement::Input { schema: input["invalid"]["schema"].as_str().expect("schema").into(), payload: hex(input["invalid"]["payloadHex"].as_str().expect("payload")) }
                    }
                    operation => protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: DemoMutation::from_value(operation.clone().into()).expect("replacement").encode_op().expect("encodes") },
                };
                protocol::SupersededInput { target, replacement }
            })
            .collect();
        let supersede = remote_supersede(name, inputs, 0);
        let mut undeferred = replica(name, initial, &log, None).await;
        undeferred.ingest_remote(supersede.clone()).await.expect("an undeferred replica adopts at once");
        assert_eq!(undeferred.snapshot_ref().n, serde_json::from_value::<Option<i32>>(case["expected"]["state"]["n"].clone()).expect("expected n"), "{name}: the corpus state");
        let mut deferred = replica(name, initial, &log, Some(1)).await;
        let before = view(&deferred);
        deferred.ingest_remote(supersede).await.expect("a deferred replica admits the change");
        if deferred.reprojection_progress().is_some() {
            assert_eq!(view(&deferred), before, "{name}: while it waits the replica shows the history before the change");
            assert!(drive(&mut deferred, 1).await > 0, "{name}: the replay steps across turns");
            deferred_cases += 1;
        }
        assert_eq!(view(&deferred), view(&undeferred), "{name}: the deferred adoption equals the undeferred one");
        test_support::assert_live_equals_replay(&deferred).await;
    }
    assert!(deferred_cases >= 5, "most corpus cases need a multi-turn replay");
}

/// ✋️ Cancelling the deferred replay after any number of steps leaves the store untouched — its projection, revision, log,
/// supersessions and outcomes are those of before the change — keeps the change admitted, and stepping again replays it
/// from the start to the undeferred adoption.
#[semio_framework_async_macros::async_test]
async fn cancelling_a_deferred_replay_at_any_step_leaves_the_store_untouched() {
    let edits: Vec<Vec<DemoMutation>> = std::iter::once(vec![set(1)]).chain((1..12).map(add).map(|operation| vec![operation])).collect();
    let (author, log) = authored("deferred-cancel", Some(0), edits).await;
    let target = author.mutation_ops().expect("operations")[0].mutation_id.clone();
    let supersede = remote_supersede("deferred-cancel", vec![replaced(&target, set(100))], 0);
    let mut undeferred = replica("deferred-cancel", Some(0), &log, None).await;
    undeferred.ingest_remote(supersede.clone()).await.expect("undeferred");
    let adopted = view(&undeferred);
    let mut total = None;
    for cancel_after in 0..13 {
        let mut store = replica("deferred-cancel", Some(0), &log, Some(1)).await;
        let before = view(&store);
        store.ingest_remote(supersede.clone()).await.expect("admitted");
        for _ in 0..cancel_after {
            if store.step_reprojection(None).await.expect("a step").is_none() {
                break;
            }
        }
        if store.reprojection_progress().is_none() {
            total.get_or_insert(cancel_after);
            assert_eq!(view(&store), adopted, "a replay that finished before the cancel is adopted");
            continue;
        }
        assert!(store.cancel_reprojection(), "a running replay cancels");
        assert_eq!(view(&store), before, "after {cancel_after} steps a cancel leaves the store untouched");
        assert_eq!(store.reprojection_progress(), Some(ReplayProgress::default()), "the change stays admitted, its replay restarts");
        assert!(!store.cancel_reprojection(), "nothing runs after a cancel");
        drive(&mut store, 1).await;
        assert_eq!(view(&store), adopted, "stepping again adopts the undeferred result");
    }
    assert!(total.is_some(), "the replay finishes within the walked steps");
}

/// 🔁️ A local edit while the replay waits lands on the history before the change and restarts the replay, whose adoption
/// is the fold of every event either replica holds; a further remote supersession joins the waiting change.
#[semio_framework_async_macros::async_test]
async fn a_local_edit_or_a_further_remote_change_restarts_the_deferred_replay() {
    let edits: Vec<Vec<DemoMutation>> = std::iter::once(vec![set(1)]).chain((1..8).map(add).map(|operation| vec![operation])).collect();
    let (author, log) = authored("deferred-restart", Some(0), edits).await;
    let ids: Vec<MutationId> = author.mutation_ops().expect("operations").into_iter().map(|operation| operation.mutation_id).collect();
    let mut deferred = replica("deferred-restart", Some(0), &log, Some(2)).await;
    deferred.ingest_remote(remote_supersede("deferred-restart", vec![replaced(&ids[0], set(10))], 0)).await.expect("admitted");
    deferred.step_reprojection(None).await.expect("a step");
    deferred.dispatch(ArtifactCommand::Apply { mutations: vec![add(100)], transaction: None }).await.expect("a local edit while the replay waits");
    assert_eq!(deferred.snapshot_ref().n, Some(129), "the local edit lands on the history before the change");
    let later = remote_supersede("deferred-restart", vec![replaced(&ids[3], add(0))], 1);
    deferred.ingest_remote(later.clone()).await.expect("a further remote change joins the waiting one");
    assert_eq!(deferred.reprojection_progress().map(|progress| progress.done <= 2), Some(true), "the replay restarted");
    drive(&mut deferred, 2).await;
    let mut undeferred = replica("deferred-restart", Some(0), &log, None).await;
    let known: HashSet<MutationId> = undeferred.event_log().expect("log").into_iter().map(|event| event.mutation_id).collect();
    for event in deferred.event_log().expect("log").into_iter().filter(|event| !known.contains(&event.mutation_id)) {
        undeferred.ingest_remote(event).await.expect("the undeferred replica takes every event");
    }
    assert_eq!(view(&deferred).0, view(&undeferred).0);
    assert_eq!(view(&deferred).3, view(&undeferred).3);
    assert_eq!(deferred.snapshot_ref().n, Some(10 + 1 + 2 + 4 + 5 + 6 + 7 + 100), "both supersessions and the local edit fold");
    test_support::assert_live_equals_replay(&deferred).await;
}

/// 📐️ The prefix-snapshot ring grows with the square root of the history (between 8 and 16 projections), so the replay a
/// remote supersession of the first operation records leaves about √N evenly strided prefixes of a long history, each
/// naming its live prefix.
#[semio_framework_async_macros::async_test]
async fn the_prefix_ring_grows_with_the_square_root_of_the_history() {
    let edits: Vec<Vec<DemoMutation>> = std::iter::once(vec![set(1)]).chain((1..100).map(|_| vec![add(1)])).collect();
    let (author, log) = authored("deferred-ring", Some(0), edits).await;
    let first = author.mutation_ops().expect("operations")[0].mutation_id.clone();
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "deferred-ring", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners());
    store.closes_on_drop();
    for event in &log {
        store.ingest_remote(event.clone()).await.expect("the replica takes the log");
        while store.maintenance_retirements_under_pressure() {
            store.maintenance_retirements_step(64, 1 << 20).expect("displaced owners retire");
        }
    }
    store.ingest_remote(remote_supersede("deferred-ring", vec![replaced(&first, set(1000))], 0)).await.expect("a remote supersession of the first operation");
    assert_eq!(store.snapshot_ref().n, Some(1099));
    let stride = 100usize.div_ceil(10);
    let lengths: Vec<usize> = store.prefix_ring.iter().map(|entry| entry.length).collect();
    assert!(lengths.len() > 8 && lengths.len() <= 10, "about √100 prefixes, more than the floor of 8: {lengths:?}");
    assert!(lengths.iter().all(|length| length % stride == 0), "strided by ⌈100 / √100⌉: {lengths:?}");
    let live = forward_prefix_digests::<DemoSnapshot, DemoMutation, _, _>(store.revision_accumulator.identity_digest, store.applied_edit_ids(), &store.envelope().vcs.edits, store.supersessions()).expect("digests");
    assert!(store.prefix_ring.iter().all(|entry| live.get(entry.length) == Some(&entry.digest)), "every retained prefix is live");
}
//#endregion 🧪️DeferredReprojectionLaws

//#region 🧪️DeferredLocalStepLaws
std::thread_local! {
    static COUNTED_FOLDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 🧮️ A demo operation that counts every fold of it (`diff`), so a law can bound the work one turn does, and names its
/// author (`None`: the store's local author) so one store can hold another actor's edits.
#[derive(Clone, Debug, PartialEq)]
struct CountedOp(DemoMutation, Option<&'static str>);

impl CountedOp {
    /// 📊️ Folds of every counted operation so far on this thread.
    fn folds() -> usize {
        COUNTED_FOLDS.with(std::cell::Cell::get)
    }
}

impl ToValue for CountedOp {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}

impl FromValue for CountedOp {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        DemoMutation::from_value(value).map(|operation| Self(operation, None))
    }
}

impl OpBinary for CountedOp {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        self.0.encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        DemoMutation::decode_op(bytes).map(|operation| Self(operation, None))
    }
}

impl OpText for CountedOp {
    fn print_op(&self) -> String {
        self.0.print_op()
    }

    fn parse_op(line: &str) -> Result<Self, TextError> {
        DemoMutation::parse_op(line).map(|operation| Self(operation, None))
    }
}

impl Mutation<DemoSnapshot> for CountedOp {
    type Diff = <DemoMutation as Mutation<DemoSnapshot>>::Diff;
    const DESCRIPTORS: &'static [crate::os_spr::MutationLeafDescriptor] = <DemoMutation as Mutation<DemoSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static crate::os_spr::MutationLeafDescriptor {
        self.0.descriptor()
    }

    fn diff(&self, base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<Self::Diff> {
        COUNTED_FOLDS.with(|folds| folds.set(folds.get() + 1));
        self.0.diff(base)
    }

    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok({
        self.0.inverse(base)?.into_iter().map(|operation| Self(operation, self.1)).collect()
    
    })
}

    fn conflict_target(&self) -> Vec<String> {
        self.0.conflict_target()
    }

    fn may_emit_foreign_steps(&self) -> bool {
        self.0.may_emit_foreign_steps()
    }

    fn author_id(&self) -> Option<ActorId> {
        self.1.map(|author| ActorId(author.into()))
    }
}

impl MemberStoreOwner<CountedOp> for DemoSnapshot {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners() -> DocumentStoreOwners<Self, CountedOp> {
        DocumentStoreOwners::new(Arc::new(DemoSnapshotRetirementFactory), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), Box::new(ArtifactStoreCursorDisposer::<DemoSnapshot, CountedOp>::new()))
    }
}

/// 📏️ The budget of every deferred law below, and the history length: well past one replay budget and the 200 mutations
/// the coordinator's N17 acceptance names.
const LOCAL_BUDGET: usize = 16;
const LONG_HISTORY: i32 = 240;

/// 🧹️ Retires every owner the last command displaced, as a runtime's maintenance turn does between commands.
fn settle(store: &mut ArtifactStore<DemoSnapshot, CountedOp>) {
    while store.maintenance_retirements_under_pressure() {
        store.maintenance_retirements_step(64, 1 << 20).expect("displaced owners retire");
    }
}

/// 🌱️ A store with a long production-shaped history (owners installed, prefix ring live): `set(1)` authored by `early`,
/// then `add(1)` × 239 authored by `later`, so `early`'s undo is an interior revert at position 0 that replays every later
/// edit. The store authors as `early` afterwards and defers local replays by `budget`.
async fn long_history(id: &str, budget: Option<ReplayTurnBudget>) -> ArtifactStore<DemoSnapshot, CountedOp> {
    let mut store = ArtifactStore::new_with_actor(create_document_envelope::<DemoSnapshot, CountedOp>("demo/v1", id, DemoSnapshot { n: Some(0) }, None), ActorId("early".into())).await;
    store.enable_convergence_early_exit();
    store.dispatch(ArtifactCommand::Apply { mutations: vec![CountedOp(set(1), Some("early"))], transaction: None }).await.expect("the early edit");
    for _ in 1..LONG_HISTORY {
        settle(&mut store);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![CountedOp(add(1), Some("later"))], transaction: None }).await.expect("a later edit");
    }
    settle(&mut store);
    store.defer_local_replays(budget);
    store
}

/// 📸️ What a counted store shows: its projection, applied length, durable outcomes, supersessions and logged events.
fn counted_view(store: &ArtifactStore<DemoSnapshot, CountedOp>) -> (Option<i32>, usize, Vec<Option<semio_framework_diagnostic::Severity>>, usize, usize, [u8; 32]) {
    let outcomes = store.mutation_outcomes().expect("outcomes").into_iter().map(|outcome| outcome.worst).collect();
    (store.snapshot_ref().n, store.applied_edit_ids().len(), outcomes, store.supersessions().len(), store.event_log().expect("log").len(), store.content_revision_now())
}

/// ⏭️ Steps a waiting local step to its end, asserting no turn folds more than one budget (plus the one edit before the
/// first changed position the replay starts from); answers the turns and the step's verdict.
async fn drive_local(store: &mut ArtifactStore<DemoSnapshot, CountedOp>) -> (usize, Result<(), VcsError>) {
    let mut turns = 0;
    loop {
        let before = CountedOp::folds();
        let stepped = store.step_reprojection(None).await;
        assert!(CountedOp::folds() - before <= LOCAL_BUDGET + 1, "turn {turns} folded {} operations", CountedOp::folds() - before);
        settle(store);
        turns += 1;
        match stepped {
            Ok(Some(_)) => assert!(turns < 10_000, "the deferred step finishes"),
            Ok(None) => return (turns, Ok(())),
            Err(error) => return (turns, Err(error)),
        }
    }
}

/// 🐌️ LAW (N17, design §16.6): with local replays deferred, every local history step over a 240-mutation history — an
/// interior undo and its redo, a supersession (finalize as overwrite), a finalize as a new alternative, a checkout away from
/// the tip and back — folds at most one budget (plus one edit) per dispatch and per turn, shows the history before it while
/// it waits (nothing logged), refuses every other history step `history.replaying`, and adopts exactly what an undeferred
/// store adopts.
#[semio_framework_async_macros::async_test]
async fn long_local_history_steps_replay_across_turns_and_adopt_what_an_undeferred_dispatch_adopts() {
    let mut deferred = long_history("local-deferred", Some(ReplayTurnBudget::operations(LOCAL_BUDGET))).await;
    let mut undeferred = long_history("local-undeferred", None).await;
    let first = |store: &ArtifactStore<DemoSnapshot, CountedOp>| store.mutation_ops().expect("operations")[0].mutation_id.clone();
    let steps: Vec<(&str, Box<dyn Fn(&ArtifactStore<DemoSnapshot, CountedOp>) -> ArtifactCommand<CountedOp>>)> = vec![
        ("interior undo", Box::new(|_: &ArtifactStore<DemoSnapshot, CountedOp>| ArtifactCommand::Undo)),
        ("its redo", Box::new(|_: &ArtifactStore<DemoSnapshot, CountedOp>| ArtifactCommand::Redo)),
        ("finalize as overwrite", Box::new(move |store: &ArtifactStore<DemoSnapshot, CountedOp>| ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: first(store), replacement: Some(CountedOp(set(100), None)) }] })),
        (
            "finalize as a new alternative",
            Box::new(move |store: &ArtifactStore<DemoSnapshot, CountedOp>| ArtifactCommand::CreateAlternativeWithSupersede {
                name: "variant".into(),
                inputs: vec![SupersedeInput { target: first(store), replacement: Some(CountedOp(set(50), None)) }],
            }),
        ),
        ("back to the trunk", Box::new(|store: &ArtifactStore<DemoSnapshot, CountedOp>| ArtifactCommand::SwitchAlternative { alternative_id: store.trunk_alternative_id() })),
    ];
    for (name, command) in &steps {
        let before = counted_view(&deferred);
        let folds = CountedOp::folds();
        let next = command(&deferred);
        let receipt = deferred.dispatch(next).await;
        assert!(receipt.is_ok(), "{name}: {receipt:?}");
        settle(&mut deferred);
        assert!(CountedOp::folds() - folds <= LOCAL_BUDGET + 1, "{name}: the dispatch folded {} operations", CountedOp::folds() - folds);
        assert!(deferred.local_step_pending(), "{name}: a long step waits for later turns");
        assert_eq!(counted_view(&deferred), before, "{name}: nothing of the step shows or is logged while it waits");
        assert_eq!(deferred.dispatch(ArtifactCommand::Undo).await.err().map(|error| error.into_fault().code.0 == "history.replaying"), Some(true), "{name}: another history step waits");
        let (turns, verdict) = drive_local(&mut deferred).await;
        assert!(verdict.is_ok(), "{name}: {verdict:?}");
        assert!(turns as i32 >= (LONG_HISTORY - 2) / LOCAL_BUDGET as i32, "{name}: the replay spreads over the turns ({turns})");
        let next = command(&undeferred);
        undeferred.dispatch(next).await.unwrap_or_else(|error| panic!("{name}: undeferred {error}"));
        settle(&mut undeferred);
        let (deferred_view, undeferred_view) = (counted_view(&deferred), counted_view(&undeferred));
        assert_eq!(
            (&deferred_view.0, deferred_view.1, &deferred_view.2, deferred_view.3, deferred_view.4),
            (&undeferred_view.0, undeferred_view.1, &undeferred_view.2, undeferred_view.3, undeferred_view.4),
            "{name}: the deferred adoption equals the undeferred one"
        );
        test_support::assert_live_equals_replay(&deferred).await;
    }
    assert_eq!(deferred.snapshot_ref().n, Some(100 + LONG_HISTORY - 1), "back on the trunk, the overwrite folds");
}

/// 🗑️ LAW (N17): a waiting local step — a finalize over a 240-mutation history — is discarded with zero trace (projection,
/// revision, outcomes, supersessions and log as before its dispatch, nothing announced), and an edit while it waits lands on
/// the history before it and restarts its replay, whose adoption folds both.
#[semio_framework_async_macros::async_test]
async fn a_waiting_local_step_is_discarded_with_zero_trace_and_an_edit_restarts_it() {
    let mut store = long_history("local-discard", Some(ReplayTurnBudget::operations(LOCAL_BUDGET))).await;
    let first = store.mutation_ops().expect("operations")[0].mutation_id.clone();
    let before = counted_view(&store);
    for steps in [0usize, 1, 5] {
        store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: first.clone(), replacement: Some(CountedOp(set(100), None)) }] }).await.expect("the finalize waits");
        for _ in 0..steps {
            assert!(store.step_reprojection(None).await.expect("a step").is_some(), "still waiting after {steps} steps");
        }
        assert!(store.discard_local_step(), "the waiting step discards");
        assert!(!store.local_step_pending() && store.reprojection_progress().is_none(), "nothing waits after a discard");
        assert_eq!(counted_view(&store), before, "a discard after {steps} steps leaves zero trace");
        assert!(!store.discard_local_step(), "nothing left to discard");
    }
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: first, replacement: Some(CountedOp(set(100), None)) }] }).await.expect("the finalize waits");
    store.step_reprojection(None).await.expect("a step");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![CountedOp(add(1000), None)], transaction: None }).await.expect("an edit while the finalize waits");
    assert_eq!(store.snapshot_ref().n, Some(LONG_HISTORY + 1000), "the edit lands on the history before the finalize");
    assert_eq!(store.reprojection_progress().map(|progress| progress.done), Some(0), "the edit restarts the finalize's replay");
    assert!(drive_local(&mut store).await.1.is_ok());
    assert_eq!(store.snapshot_ref().n, Some(100 + LONG_HISTORY - 1 + 1000), "the finalize and the edit both fold");
    test_support::assert_live_equals_replay(&store).await;
}

/// 🛑️ LAW (N17, design §16.1): a deferred finalize whose replay reports a blocking downstream error is refused once its
/// replay finishes — `Rejected { policy: Normal }` with the downstream messages, exactly the refusal an undeferred
/// dispatch answers at once — and nothing of it is recorded.
#[semio_framework_async_macros::async_test]
async fn a_deferred_finalize_whose_report_blocks_is_refused_with_nothing_recorded() {
    let mut deferred = long_history("local-blocked", Some(ReplayTurnBudget::operations(LOCAL_BUDGET))).await;
    let mut undeferred = long_history("local-blocked-undeferred", None).await;
    let withdraw_n = |store: &ArtifactStore<DemoSnapshot, CountedOp>| ArtifactCommand::Supersede {
        scope: None,
        inputs: vec![SupersedeInput { target: store.mutation_ops().expect("operations")[0].mutation_id.clone(), replacement: Some(CountedOp(DemoMutation::DeleteN(DeleteN {}), None)) }],
    };
    let next = withdraw_n(&undeferred);
    let refused = undeferred.dispatch(next).await.err().expect("an undeferred blocking finalize is refused");
    let before = counted_view(&deferred);
    let next = withdraw_n(&deferred);
    deferred.dispatch(next).await.expect("the deferred finalize waits");
    let (_, verdict) = drive_local(&mut deferred).await;
    let verdict = verdict.err().expect("the blocking finalize is refused once replayed");
    assert!(matches!(&verdict, VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages } if !messages.is_empty()), "{verdict:?}");
    assert_eq!(verdict, refused, "the same refusal as the undeferred dispatch");
    assert!(!deferred.local_step_pending() && deferred.reprojection_progress().is_none());
    assert_eq!(counted_view(&deferred), before, "nothing of the refused finalize is recorded");
}

/// 🕰️ A clock on which every fold of a counted operation takes one millisecond: what bounds a turn by wall time without
/// sleeping.
fn fold_clock_us() -> Option<u64> {
    Some(CountedOp::folds() as u64 * 1_000)
}

/// ⏱️ LAW (W2A-3, design §7 "≤ 4 ms slices"): a deferred local step yields on its turn's wall deadline long before its
/// operation cap. With every fold taking 1 ms on the turn clock and 4 ms per turn, neither the dispatch nor any later turn
/// folds more than four operations (plus the one edit before the first changed position) although the cap allows 256; a
/// runtime's own turn deadline ends a turn earlier still; the adoption equals the undeferred dispatch's.
#[semio_framework_async_macros::async_test]
async fn a_deferred_local_step_yields_on_its_wall_deadline_before_its_operation_cap() {
    const WALL_US: u64 = 4_000;
    let budget = ReplayTurnBudget { wall_us: WALL_US, operations: 256, now_us: fold_clock_us };
    let mut deferred = long_history("local-wall", Some(budget)).await;
    let mut undeferred = long_history("local-wall-undeferred", None).await;
    let folds = CountedOp::folds();
    deferred.dispatch(ArtifactCommand::Undo).await.expect("the interior undo waits");
    settle(&mut deferred);
    assert!(CountedOp::folds() - folds <= 5, "the dispatch folded {} operations in its 4 ms", CountedOp::folds() - folds);
    assert!(deferred.local_step_pending(), "the interior undo outlasts one wall budget");
    let mut turns = 0;
    loop {
        let before = CountedOp::folds();
        let deadline = (turns % 2 == 1).then(|| fold_clock_us().expect("the fold clock reads") + 2_000);
        let stepped = deferred.step_reprojection(deadline).await.expect("a wall-bounded turn");
        let folded = CountedOp::folds() - before;
        assert!(folded >= 1 && folded <= if deadline.is_some() { 3 } else { 5 }, "turn {turns} folded {folded} operations (runtime deadline {deadline:?})");
        settle(&mut deferred);
        turns += 1;
        if stepped.is_none() {
            break;
        }
        assert!(turns < 10_000, "the deferred undo finishes");
    }
    assert!(turns as i32 >= (LONG_HISTORY - 2) / 5, "the replay spreads over wall-bounded turns ({turns})");
    undeferred.dispatch(ArtifactCommand::Undo).await.expect("the undeferred undo");
    settle(&mut undeferred);
    let (deferred_view, undeferred_view) = (counted_view(&deferred), counted_view(&undeferred));
    assert_eq!(
        (&deferred_view.0, deferred_view.1, &deferred_view.2, deferred_view.3, deferred_view.4),
        (&undeferred_view.0, undeferred_view.1, &undeferred_view.2, undeferred_view.3, undeferred_view.4),
        "the wall-bounded adoption equals the undeferred one"
    );
    test_support::assert_live_equals_replay(&deferred).await;
}
//#endregion 🧪️DeferredLocalStepLaws
