//! 🖐️ Runtime laws of the framework-owned gesture slot (design §22.10 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING):
//! every window owns ONE slot in the instance's `GestureLedger`; a dispatch drives its window's tool against a copy of the
//! slot and the slot follows only when the dispatch publishes; every render reads committed ⊕ the open gestures; and host
//! facts end a gesture in the runtime with zero trace — an opened history edit `frozen` for every window — for an app that
//! answers no host event of its own (the toy history app); the slot owns the host press identity (a late release of an
//! ended press leaves zero trace). The drive itself is pinned by the tool-machine corpus
//! (`🧫️gesture-drive-law`: `rows`, `hostEvents`, `slots`).

use super::*;
use semio_framework_tool_machine::{GesturePhase, GestureState, GestureTool, ToolAbortReason, ToolRefusal, ToolStep};

fn gesture_grants()->(semio_framework_value::retained_clone::RetainedCloneGrant,semio_framework_value::retained_clone::RetainedCloneGrant){
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::shared::sealed::SealedShared};
 (RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<GestureSlot<TestMutation>>::birth_bytes(),maximum_depth:1,..Default::default()},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<SealedShared<GestureSlot<TestMutation>>>(),maximum_depth:1,..Default::default()})
}
fn retire_gesture_capture<T:semio_framework_value::retirement::RetireOwned>(original:T){
 use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
 let mut owner=ControlledRetirement::new(original).unwrap_or_else(|(_,original)|{let _original=std::mem::ManuallyDrop::new(original);panic!("original gesture fixture retirement was refused");});
 for _ in 0..100000 {if owner.terminal_is_empty(){return;}let copy=owner.next_copy_byte_demand().unwrap().max(4096);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};assert!(owner.step(grant).unwrap().progress().fits(grant));}
 panic!("original gesture fixture retained physical owners");
}
const NUDGE_TOOL: &str = "s.test.time-travel@1/*#editor#nudge";

/// 🧮️ The law's streamed tool: every tick replaces its ONE absolute leaf, the release commits it, and its persisted form is
/// the framework's `GestureState`.
struct NudgeTool {
    verb: String,
    authoring_seed: String,
    base_revision: String,
    transaction: Option<protocol::TransactionRef>,
    leaf: Option<TestMutation>,
}

impl GestureTool for NudgeTool {
    type Gesture = GestureState<TestMutation>;
    type Tick = TestMutation;
    type Mutation = TestMutation;

    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        Ok(Self { verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string(), transaction: None, leaf: None })
    }

    fn resume(gesture: &GestureState<TestMutation>) -> Result<Self, ToolRefusal> {
        let [(_, leaf)] = gesture.entries.as_slice() else { return Err(ToolRefusal::Closed) };
        Ok(Self { verb: gesture.verb.clone(), authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone(), transaction: Some(gesture.transaction.clone()), leaf: Some(leaf.clone()) })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base_revision
    }

    fn abort(&mut self, _reason: ToolAbortReason) {
        self.transaction = None;
        self.leaf = None;
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<TestMutation>) -> Result<ToolStep<TestMutation>, ToolRefusal> {
        if tick.is_some() {
            self.leaf = tick;
        }
        let Some(leaf) = self.leaf.clone() else { return Ok(ToolStep::Idle) };
        let transaction = match self.transaction.clone() {
            Some(transaction) => transaction,
            None => protocol::TransactionRef::mint(&ActorId(self.authoring_seed.clone()), &HybridLogicalTimestamp { actor: 0, physical_ms: 1, logical: 0 }, NUDGE_TOOL),
        };
        if phase == GesturePhase::Stream {
            self.transaction = Some(transaction);
            return Ok(ToolStep::Open);
        }
        self.transaction = None;
        self.leaf = None;
        Ok(ToolStep::Committed(transaction, vec![leaf]))
    }

    fn persist(self) -> Option<GestureState<TestMutation>> {
        let (transaction, leaf) = (self.transaction?, self.leaf?);
        Some(GestureState {
            states: vec!["streaming".to_string()],
            verb: self.verb,
            press: String::new(),
            authoring_seed: self.authoring_seed,
            base_revision: self.base_revision,
            transaction,
            entries: vec![("leaf".to_string(), leaf)],
            context: DslValue::Null,
        })
    }
}

fn two_panes() -> Vec<ViewWindowInstance> {
    vec![ViewWindowInstance { id: "pane-a".into(), window_kind_id: "main".into() }, ViewWindowInstance { id: "pane-b".into(), window_kind_id: "main".into() }]
}

fn in_window(actor: &str, window: &str) -> ActionMeta {
    let view = ViewModel { window_id: Some(window.to_string()), window_instances: two_panes(), ..ViewModel::new(Locale::En, Terminology::Native) };
    ActionMeta { view_state: Some(view), ..artifact_app_laws::meta(actor) }
}

/// 🚃️ One stream tick of `window` setting the count to `value`, as the dispatch `operation` that publishes: admitted on the
/// live revision, driven against its slot, settled like a publishing completion. Answers what the tick committed.
fn stream(app: &mut ToyApp, actor: &str, operation: u64, window: &str, value: i32) -> Option<(protocol::TransactionRef, Vec<TestMutation>)> {
    let base = app.store.content_revision_now();
    let slot = app.admit_gesture_slot(operation, &base, &in_window(actor, window),gesture_grants().0,gesture_grants().1).expect("funded original gesture admission");
    let committed = slot.drive::<NudgeTool>(None, "nudge", GesturePhase::Stream, Some(SetCount { value }.into()), "seed").expect("a stream tick is never refused");
    assert!(app.settle_gesture_slot(operation, true), "the dispatch drove its slot");
    retire_gesture_capture(slot);
    committed
}

fn open_windows(app: &ToyApp) -> Vec<String> {
    app.tool_machines.gestures().windows().map(str::to_string).collect()
}

/// 🧊️ LAW (design §22.10, the brief's Frozen law): two windows hold a gesture in flight — persisted by one dispatch, resumed
/// by the next, previewed by every render over an untouched committed document. Opening a history edit ends both `frozen`
/// in the runtime: no slot, no overlay, no edit, no store change — although the app answers no host event at all — and a
/// tick that arrives while the document is frozen opens nothing.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_ends_every_open_gesture_with_zero_trace_for_an_app_without_a_frozen_arm() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    assert!(ToyHistoryApp::host_event(&HostEvent::TimeTravelFrozen { window_id: "pane-a".into() }).is_none(), "the app has no Frozen arm of its own");
    let (generation, committed, edits) = (app.store.generation(), head(&app), app.edit_transactions().len());
    assert_eq!(stream(&mut app, &actor, 900, "pane-a", 40), None, "a tick publishes nothing");
    assert_eq!(stream(&mut app, &actor, 901, "pane-a", 41), None);
    assert_eq!(stream(&mut app, &actor, 902, "pane-b", 42), None);
    assert_eq!(open_windows(&app), vec!["pane-a", "pane-b"]);
    let resumed = app.tool_machines.gestures().open("pane-a").expect("pane-a holds its gesture").clone();
    assert_eq!((resumed.verb.as_str(), resumed.entries.as_slice()), ("nudge", [("leaf".to_string(), TestMutation::from(SetCount { value: 41 }))].as_slice()), "the second tick resumed the first tick's gesture");
    assert_eq!(app.rendered_snapshot().count, 42, "every render reads committed ⊕ the open gestures, window by window");
    assert_eq!((app.store.generation(), head(&app), app.edit_transactions().len()), (generation, committed.clone(), edits), "an open gesture never touches the committed document");
    TOY_HOST_EVENTS.with(|events| events.borrow_mut().clear());
    let mutation = seeded_mutation(&app, 0);
    let begun = app.handle_action("historyEditBegin", Some(&DslValue::Object(vec![("mutationId".into(), DslValue::String(mutation))])), &in_window(&actor, "pane-a")).await.expect("begin");
    assert_eq!(rejected(&begun), None);
    assert_eq!(TOY_HOST_EVENTS.with(|events| events.borrow().len()), 2, "the app was told once per window and answered nothing");
    assert!(app.tool_machines.gestures().is_empty(), "the history edit ended every window's gesture");
    assert_eq!(app.rendered_snapshot().count, committed.0, "no overlay is left");
    assert_eq!(stream(&mut app, &actor, 903, "pane-b", 77), None);
    assert!(app.tool_machines.gestures().is_empty(), "a tick on the frozen document opens no gesture");
    assert_eq!((app.store.generation(), head(&app), app.edit_transactions().len()), (generation, committed, edits), "zero trace: no edit, no store change");
    app.handle_action("historyEditExit", None, &in_window(&actor, "pane-a")).await.expect("exit");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🛎️ LAW (design §22.10): a window's host fact ends only that window's gesture, in the runtime — the host-forwarded blur
/// and lost capture through the real `hostEvent` verb, with the app answering nothing — and the render falls back to the
/// committed document once the last gesture is gone.
#[semio_framework_async_macros::async_test]
async fn a_windows_host_fact_ends_only_its_own_gesture() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    let (generation, committed) = (app.store.generation(), head(&app));
    stream(&mut app, &actor, 900, "pane-a", 40);
    stream(&mut app, &actor, 901, "pane-b", 42);
    let host = |window: &str, kind: &str| DslValue::Object(vec![(semio_framework::HOST_EVENT_ARG_WINDOW_ID.to_string(), DslValue::String(window.to_string())), (semio_framework::HOST_EVENT_ARG_KIND.to_string(), DslValue::String(kind.to_string()))]);
    app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&host("pane-a", semio_framework::HOST_EVENT_KIND_BLUR)), &in_window(&actor, "pane-a")).await.expect("blur");
    assert_eq!(open_windows(&app), vec!["pane-b"], "the blur ended pane-a's gesture only");
    assert_eq!(app.rendered_snapshot().count, 42);
    app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&host("pane-b", semio_framework::HOST_EVENT_KIND_CAPTURE_LOST)), &in_window(&actor, "pane-b")).await.expect("capture lost");
    assert!(app.tool_machines.gestures().is_empty());
    assert_eq!(app.rendered_snapshot().count, committed.0, "the render reads the committed document again");
    assert_eq!((app.store.generation(), head(&app)), (generation, committed), "zero trace");
    close(&mut app);
}

/// 🪪️ LAW (press identity): the slot owns a gesture's host press — a late release of the press a blur ended commits
/// nothing and opens nothing, although the tool would commit a release at rest; a tick of another press interrupts the open
/// gesture and takes the slot; and a dispatch the runtime dropped still logs as one that drove its slot.
#[semio_framework_async_macros::async_test]
async fn a_late_release_of_a_press_the_runtime_ended_leaves_zero_trace() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    let base = app.store.content_revision_now();
    let dispatch = |app: &mut ToyApp, operation: u64, press: &str, phase: GesturePhase, value: i32| {
        let slot = app.admit_gesture_slot(operation, &base, &in_window(&actor, "pane-a"),gesture_grants().0,gesture_grants().1).expect("funded original gesture admission");
        let committed = slot.drive::<NudgeTool>(Some(press), "nudge", phase, Some(SetCount { value }.into()), "seed").expect("the dispatch is never refused");
        assert!(app.settle_gesture_slot(operation, true), "even a dropped dispatch drove its slot");
        retire_gesture_capture(slot);
        committed
    };
    assert_eq!(dispatch(&mut app, 900, "p1", GesturePhase::Stream, 40), None);
    assert_eq!(app.tool_machines.gestures().open("pane-a").map(|gesture| gesture.press.as_str()), Some("p1"), "the gesture belongs to the press that opened it");
    let blur = DslValue::Object(vec![(semio_framework::HOST_EVENT_ARG_WINDOW_ID.to_string(), DslValue::String("pane-a".into())), (semio_framework::HOST_EVENT_ARG_KIND.to_string(), DslValue::String(semio_framework::HOST_EVENT_KIND_BLUR.into()))]);
    app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&blur), &in_window(&actor, "pane-a")).await.expect("blur");
    assert_eq!(app.tool_machines.gestures().closed("pane-a"), Some("p1"), "the blur closed the press");
    assert_eq!(dispatch(&mut app, 901, "p1", GesturePhase::Commit, 41), None, "the late release commits nothing");
    assert_eq!(dispatch(&mut app, 902, "p1", GesturePhase::Stream, 42), None);
    assert!(app.tool_machines.gestures().is_empty(), "and a late tick opens nothing");
    assert_eq!(dispatch(&mut app, 903, "p2", GesturePhase::Stream, 50), None);
    assert_eq!(dispatch(&mut app, 904, "p3", GesturePhase::Stream, 60), None);
    assert_eq!(app.tool_machines.gestures().open("pane-a").map(|gesture| (gesture.press.as_str(), gesture.entries.len())), Some(("p3", 1)), "another press interrupted the open one and took the slot");
    assert_eq!(app.tool_machines.gestures().closed("pane-a"), Some("p2"));
    let (_, mutations) = dispatch(&mut app, 905, "p3", GesturePhase::Commit, 61).expect("the release of the open press commits");
    assert_eq!(mutations, vec![TestMutation::from(SetCount { value: 61 })]);
    assert!(app.tool_machines.gestures().is_empty());
    close(&mut app);
}

/// 🖋️ LAW (design §22.10): the slot follows a dispatch only when it publishes and only while the window still holds the
/// gesture the dispatch was admitted on — a faulted dispatch decides nothing, and of two dispatches admitted on the same
/// slot the first to publish wins; a release commits the gesture as ONE transaction and clears the slot.
#[semio_framework_async_macros::async_test]
async fn the_slot_follows_only_a_publishing_dispatch_admitted_on_the_gesture_it_holds() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    let base = app.store.content_revision_now();
    let faulted = app.admit_gesture_slot(900, &base, &in_window(&actor, "pane-a"),gesture_grants().0,gesture_grants().1).expect("funded original gesture admission");
    faulted.drive::<NudgeTool>(None, "nudge", GesturePhase::Stream, Some(SetCount { value: 7 }.into()), "seed").expect("a tick");
    assert!(app.settle_gesture_slot(900, false));
    assert!(app.tool_machines.gestures().is_empty(), "a dispatch that does not publish decides nothing");
    let (first, second) = (app.admit_gesture_slot(901, &base, &in_window(&actor, "pane-a"),gesture_grants().0,gesture_grants().1).expect("funded original gesture admission"), app.admit_gesture_slot(902, &base, &in_window(&actor, "pane-a"),gesture_grants().0,gesture_grants().1).expect("funded original gesture admission"));
    first.drive::<NudgeTool>(None, "nudge", GesturePhase::Stream, Some(SetCount { value: 8 }.into()), "seed").expect("a tick");
    second.drive::<NudgeTool>(None, "nudge", GesturePhase::Stream, Some(SetCount { value: 9 }.into()), "seed").expect("a tick");
    assert!(app.settle_gesture_slot(901, true) && app.settle_gesture_slot(902, true));
    assert_eq!(app.rendered_snapshot().count, 8, "the dispatch admitted on a slot that moved since is dropped");
    assert!(!app.settle_gesture_slot(902, true), "a slot is settled once");
    let release = app.admit_gesture_slot(903, &base, &in_window(&actor, "pane-a"),gesture_grants().0,gesture_grants().1).expect("funded original gesture admission");
    let (transaction, mutations) = release.drive::<NudgeTool>(None, "nudge", GesturePhase::Commit, Some(SetCount { value: 10 }.into()), "seed").expect("the release").expect("the release commits");
    assert_eq!((transaction.tool.as_str(), mutations), (NUDGE_TOOL, vec![TestMutation::from(SetCount { value: 10 })]), "ONE transaction of the net leaf");
    assert!(app.settle_gesture_slot(903, true));
    assert!(app.tool_machines.gestures().is_empty(), "the release cleared the slot");
    let emit: Emit<TestMutation, TestConfigMutation, NoDraftMutation> = gesture_emit(Some((transaction.clone(), vec![SetCount { value: 10 }.into()])), "seed");
    assert_eq!((emit.transaction, emit.artifact_mutations.len()), (Some(transaction), 1), "the committed gesture publishes as ONE stamped edit");
    retire_gesture_capture((faulted,first,second,release));
    close(&mut app);
}
