//! ⏪️ History-edit session laws of the plugin runtime (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §4, §7,
//! §10), driven by the language-agnostic fixture `🧫️fixtures/🧫️time-travel/🔣️.json` whose scenarios the TypeScript
//! oracle (`🟦️.ts` beside this file) also runs through the independent time-travel reducer twin: the preview is the
//! state before the edited mutation with its draft and nothing downstream, accepting replays downstream per turn, an
//! Error or Fatal blocks finalizing until withdrawn, overwrite and new alternative commit once, exit leaves zero trace,
//! artifact-lane and history-lane verbs are frozen, a remote edit re-replays, and rows keep one tool transaction.

use super::*;
use crate::test_app_mutation_fixture::{SetCount, SetLabel, SetSlotChildren, TestConfig, TestConfigMutation, TestMutation, TestSnapshot};
use semio_framework_time_travel::TimeTravelStage;
use store::{Backbone, BackboneMessage, MemoryBackbone};

const TIME_TRAVEL_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🧫️time-travel/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(TIME_TRAVEL_FIXTURE_JSON).expect("time-travel fixture parses")
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, got {value}"))
}

fn dsl(value: &Value) -> DslValue {
    dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(&value.to_string()).expect("fixture value is JSON"))
}

//#region 🧸️ToyHistoryApp
thread_local! {
    /// 📨️ Every host event the toy app received on this test thread, in order.
    static TOY_HOST_EVENTS: std::cell::RefCell<Vec<HostEvent>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[derive(Default)]
struct ToyHistoryApp;

impl ArtifactApp for ToyHistoryApp {
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.time-travel", standard: StandardId("1"), subset: SubsetId::ANY };
    const APP_ID: &'static str = "s.test.time-travel@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-time-travel/v1";
    type Snapshot = TestSnapshot;
    type Mutation = TestMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TestMutation;

    fn mutation_label(op: &TestMutation) -> Option<LocalizedLabel> {
        Some(protocol::SemanticMutation::label(op))
    }

    fn host_event(event: &HostEvent) -> Option<TestMutation> {
        TOY_HOST_EVENTS.with(|events| events.borrow_mut().push(event.clone()));
        None
    }

    async fn initial_snapshot() -> TestSnapshot {
        TestSnapshot::default()
    }

    async fn handle(
        command: &TestMutation,
        _doc: &ArtifactView<'_, TestSnapshot>,
        _cfg: &ConfigView<'_, TestConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<TestMutation, TestConfigMutation, NoDraftMutation> {
        Ok(Emit { artifact_mutations: vec![command.clone()], ..Default::default() })
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={} label={}", doc.snapshot.count, doc.snapshot.label)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(bounded_document_store_owners::<Self::Draft, Self::DraftMutation>())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("time-travel-doc", 4_096))
    }

    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(mutation_fixture::no_state::presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(mutation_fixture::no_state::transient_store_disposer())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(mutation_fixture::no_state::presence_peer_retirement_factory())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(mutation_fixture::no_state::presence_local_root_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(mutation_fixture::no_state::transient_local_root_retirement_factory())
    }
}
//#endregion 🧸️ToyHistoryApp

//#region 🧰️Harness
type ToyApp = VcsArtifactApp<ToyHistoryApp>;

async fn toy_manifest() -> App {
    let builder = App::builder(ToyHistoryApp::APP_ID, LocalizedLabel::data("Time Travel Fixture"))
        .await
        .document(["state"])
        .mode("edit", LocalizedLabel::data("Edit"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::data("Main"), "time-travel.main", SurfaceKind::Canvas2d, IconName::AppWindow)
        .await;
    App::from_builder(builder).await
}

fn seed_op(op: &Value) -> TestMutation {
    match text(&op["kind"]) {
        "setCount" => SetCount { value: op["value"].as_i64().expect("count") as i32 }.into(),
        "setLabel" => SetLabel { value: text(&op["value"]).to_string() }.into(),
        _ => SetSlotChildren { children: op["value"].as_array().expect("children").iter().map(|child| text(child).to_string()).collect() }.into(),
    }
}

/// 🌱️ A registered toy instance holding the fixture's seed, one edit per op, authored by the fixture actor.
async fn seeded_app(fixture: &Value) -> ToyApp {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    app.store.set_local_actor_id(Some(text(&fixture["actor"]).to_string())).expect("local actor");
    for op in fixture["seed"].as_array().expect("seed") {
        app.store.dispatch(ArtifactCommand::Apply { mutations: vec![seed_op(op)], description: None, transaction: None }).await.expect("seed edit applies");
    }
    app.refresh_cache().await.expect("the command log backfills the seed");
    app
}

fn meta(fixture: &Value) -> ActionMeta {
    ActionMeta { view_state: Some(ViewModel::default()), ..artifact_app_laws::meta(text(&fixture["actor"])) }
}

/// ✏️ The mutation id of seeded edit `index` (each seeded edit holds one op).
fn seeded_mutation(app: &ToyApp, index: usize) -> String {
    app.store.mutation_ops().expect("mutation ops")[index].mutation_id.0.clone()
}

async fn verb(app: &mut ToyApp, fixture: &Value, action: &str, args: Vec<(String, DslValue)>) -> InvocationResult {
    app.handle_action(action, Some(&DslValue::Object(args)), &meta(fixture)).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"))
}

fn rejected(result: &InvocationResult) -> Option<&str> {
    result.output.get("rejected").and_then(DslValue::as_str)
}

async fn pump_until(app: &mut ToyApp, what: &str, done: impl Fn(&ToyApp) -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return;
        }
        app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("{what} never settled; stage {:?}", app.time_travel.session().stage);
}

async fn render_body(app: &mut ToyApp) -> String {
    let tree = app.render("time-travel.main", None, &ViewModel::default()).await.unwrap_or_else(|fault| panic!("render: {fault:?}"));
    artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error| panic!("project: {error}"))
}

async fn render_history(app: &mut ToyApp, locale: Locale) -> Value {
    let view = ViewModel { locale, ..ViewModel::default() };
    let tree = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &view).await.unwrap_or_else(|fault| panic!("history render: {fault:?}"));
    serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(tree).expect("history projection")).expect("history projection parses")
}

fn find_node<'a>(node: &'a Value, key: &str) -> Option<&'a Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| find_node(child, key))
}

/// ⏯️ Runs one fixture step through the real verbs (a `replay` pumps the per-turn Report replay to completion).
async fn run_step(app: &mut ToyApp, fixture: &Value, step: &Value) -> Option<InvocationResult> {
    let (name, value) = step.as_object().and_then(|step| step.iter().next()).expect("one-key step");
    Some(match name.as_str() {
        "begin" => {
            let mutation = seeded_mutation(app, value.as_u64().expect("edit index") as usize);
            verb(app, fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))]).await
        }
        "input" => verb(app, fixture, "historyEditInput", vec![("path".into(), dsl(&value["path"])), ("value".into(), dsl(&value["value"]))]).await,
        "withdraw" => verb(app, fixture, "historyEditWithdraw", Vec::new()).await,
        "accept" => verb(app, fixture, "historyEditAccept", Vec::new()).await,
        "discard" => verb(app, fixture, "historyEditDiscard", Vec::new()).await,
        "finalize" => verb(app, fixture, "historyEditFinalize", Vec::new()).await,
        "exit" => verb(app, fixture, "historyEditExit", Vec::new()).await,
        "commit" => {
            let mut args = Vec::new();
            if let Some(name) = value["name"].as_str() {
                args.push(("name".to_string(), DslValue::String(name.to_string())));
            }
            if let Some(choice) = value["choice"].as_str() {
                args.push(("choice".to_string(), DslValue::String(choice.to_string())));
            }
            verb(app, fixture, "historyEditCommit", args).await
        }
        "replay" => {
            pump_until(app, "replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
            return None;
        }
        other => panic!("unknown fixture step {other}"),
    })
}

/// 🧹️ Closes the instance and proves the history-edit ledger retired every owner.
fn close(app: &mut ToyApp) {
    artifact_app_laws::close_registered_fixture_app(app);
    assert!(app.time_travel.terminal_is_empty(), "the history-edit ledger retires every owner on close");
}
//#endregion 🧰️Harness

/// ⚖️ LAW: every fixture scenario ends in its status and renders its body; accepted verbs never error, the committed
/// document only changes through a commit, and pending/superseded/withdrawn rows are exactly the fixture's.
#[semio_framework_async_macros::async_test]
async fn every_fixture_scenario_reaches_its_status_body_and_rows() {
    let fixture = fixture();
    for scenario in fixture["scenarios"].as_array().expect("scenarios") {
        let id = text(&scenario["id"]);
        let mut app = seeded_app(&fixture).await;
        let generation = app.store.generation();
        let mut commits = false;
        for step in scenario["steps"].as_array().expect("steps") {
            commits |= step.get("commit").is_some();
            if let Some(result) = run_step(&mut app, &fixture, step).await {
                assert_eq!(rejected(&result), None, "{id}: step {step} was refused: {:?}", result.output);
            }
        }
        let status = app.time_travel.status();
        match scenario["status"].as_object() {
            None => assert!(status.is_none(), "{id}: the session closed"),
            Some(expected) => {
                let status = status.unwrap_or_else(|| panic!("{id}: a session is open"));
                assert_eq!(serde_json::to_value(status.stage).expect("stage"), expected["stage"], "{id}: stage");
                assert_eq!(status.blocking, expected["blocking"].as_bool().expect("blocking"), "{id}: blocking");
                assert_eq!(u64::from(status.accepted_count), expected["acceptedCount"].as_u64().expect("accepted"), "{id}: accepted drafts");
            }
        }
        if let Some(refusal) = scenario.get("finalizeRefusal") {
            assert_eq!(app.time_travel.session().finalize_refusal().map(|refusal| refusal.code()), refusal.as_str(), "{id}: finalize refusal");
        }
        assert_eq!(render_body(&mut app).await.contains(text(&scenario["body"])), true, "{id}: body {}", render_body(&mut app).await);
        if !commits {
            assert_eq!(app.store.generation(), generation, "{id}: no commit, no store change");
        }
        let patch = app.history_patch(true).await.expect("history patch");
        let rows: Vec<&semio_framework::kernel::HistoryMutationEntry> = patch.upserts.iter().rev().flat_map(|entry| entry.mutations.iter()).collect();
        let mutations: Vec<String> = (0..fixture["seed"].as_array().expect("seed").len()).map(|index| seeded_mutation(&app, index)).collect();
        for (key, flag) in [("pending", 0usize), ("superseded", 1), ("withdrawn", 2)] {
            let Some(expected) = scenario.get(key).and_then(Value::as_array) else { continue };
            let expected: Vec<&str> = expected.iter().map(|index| mutations[index.as_u64().expect("index") as usize].as_str()).collect();
            let actual: Vec<&str> = rows.iter().filter(|row| [row.pending, row.superseded, row.withdrawn][flag]).map(|row| row.mutation_id.as_str()).collect();
            assert_eq!(actual, expected, "{id}: {key} rows");
        }
        if let Some(name) = scenario["alternative"].as_str() {
            let envelope = app.store.envelope();
            let alternative = envelope.vcs.alternatives.iter().find(|alternative| alternative.name == name).unwrap_or_else(|| panic!("{id}: alternative {name} exists"));
            assert_eq!(envelope.active_alternative_id.as_deref(), Some(alternative.id.as_str()), "{id}: the new alternative is active");
            assert!(app.store.supersessions().values().all(|supersession| supersession.scope.as_deref() == Some(alternative.id.as_str())), "{id}: the supersession is scoped to the new alternative");
        }
        if scenario["superseded"].as_array().is_some_and(|superseded| !superseded.is_empty()) && scenario["alternative"].is_null() {
            assert!(app.store.supersessions().values().all(|supersession| supersession.scope.is_none()), "{id}: overwrite is unscoped");
        }
        pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
        close(&mut app);
    }
}

/// ⚖️ LAW: an overwrite leaves exactly the document a fresh store folding the edited log reaches.
#[semio_framework_async_macros::async_test]
async fn an_overwrite_head_equals_a_fresh_fold_of_the_edited_log() {
    let fixture = fixture();
    let scenario = fixture["scenarios"].as_array().expect("scenarios").iter().find(|scenario| text(&scenario["id"]) == "overwrite-supersedes-in-place").expect("overwrite scenario");
    let mut app = seeded_app(&fixture).await;
    for step in scenario["steps"].as_array().expect("steps") {
        run_step(&mut app, &fixture, step).await;
    }
    let mut fresh = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    for (index, op) in fixture["seed"].as_array().expect("seed").iter().enumerate() {
        let op = if index == 1 { SetLabel { value: "b".into() }.into() } else { seed_op(op) };
        fresh.store.dispatch(ArtifactCommand::Apply { mutations: vec![op], description: None, transaction: None }).await.expect("fresh edit");
    }
    assert_eq!(app.store.snapshot().expect("edited head"), fresh.store.snapshot().expect("fresh head"));
    close(&mut app);
    close(&mut fresh);
}

/// ⚖️ LAW: finalizing opens the framework-injected dialog seeded with the localized default name; the injected
/// definition names the commit and back verbs, and a viewer manifest declares neither the verbs nor the dialog.
#[semio_framework_async_macros::async_test]
async fn finalize_opens_the_injected_dialog_with_a_localized_default_name() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step).await;
    }
    let result = verb(&mut app, &fixture, "historyEditFinalize", Vec::new()).await;
    let Some(Effect::OpenDialog { dialog_id, args, .. }) = result.requested_effects.first() else { panic!("finalize opens a dialog: {:?}", result.requested_effects) };
    assert_eq!(dialog_id, semio_framework::HISTORY_EDIT_FINALIZE_DIALOG_ID);
    assert_eq!(args.as_ref().and_then(|args| args.get("name")).and_then(DslValue::as_str), Some("Edited history"), "the default name is localized, never a literal");
    let manifest = toy_manifest().await;
    let definition = &manifest.definition;
    assert!(definition.dialogs.iter().any(|dialog| dialog.id == semio_framework::HISTORY_EDIT_FINALIZE_DIALOG_ID), "every editor declares the finalize dialog");
    assert!(semio_framework::HISTORY_EDIT_ACTION_IDS.iter().all(|id| definition.actions.iter().any(|action| action.id == *id)), "every editor declares the history-edit verbs");
    assert!(result.history_patch.as_ref().and_then(|patch| patch.time_travel.as_ref()).is_some_and(|status| status.stage == semio_framework::kernel::HistoryTimeTravelStage::Choosing), "the verb result carries the choosing status");
    verb(&mut app, &fixture, "historyEditBack", Vec::new()).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing, "cancel goes back to reviewing, never discards");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// ⚖️ LAW: while a history edit is open, artifact emits, child emits and history-lane verbs are refused with
/// `timeTravel.frozen`; view verbs keep working; a stale generation and an invalid input value are silent refusals
/// that leave the draft untouched.
#[semio_framework_async_macros::async_test]
async fn an_open_session_freezes_document_verbs_and_refuses_stale_or_invalid_drafts() {
    let fixture = fixture();
    let refusals = &fixture["refusals"];
    let mut app = seeded_app(&fixture).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let emitted = app.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { artifact_mutations: vec![SetCount { value: 3 }.into()], ..Default::default() }, &meta(&fixture)).await;
    assert_eq!(emitted.err().map(|fault| fault.code), Some(FaultCode::new(text(&refusals["frozen"]))), "an artifact emit is frozen");
    for action in refusals["frozenVerbs"].as_array().expect("frozen verbs") {
        let refused = app.handle_action(text(action), None, &meta(&fixture)).await;
        assert_eq!(refused.err().map(|fault| fault.code), Some(FaultCode::new(text(&refusals["frozen"]))), "{action} is frozen");
    }
    let filtered = app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&DslValue::object([("value".to_string(), DslValue::String("all".into()))])), &meta(&fixture)).await;
    assert!(filtered.is_ok(), "a view verb keeps working: {filtered:?}");
    let generation = app.time_travel.session().generation;
    let stale = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::uint(9)), ("generation".into(), DslValue::uint(u64::from(generation) + 1))]).await;
    assert_eq!(rejected(&stale), Some(text(&refusals["stale"])));
    let invalid = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), dsl(&refusals["invalidInputValue"]))]).await;
    assert_eq!(rejected(&invalid), Some(text(&refusals["invalidInput"])));
    assert!(app.time_travel.editor().is_some_and(|editor| editor.refused.is_some()), "the refused value names its reason on the editor");
    assert_eq!(app.time_travel.session().generation, generation, "refusals leave the session untouched");
    let history = render_history(&mut app, Locale::De).await;
    assert!(find_node(&history, "framework.history.timeTravel").is_some(), "the band leads the history body");
    assert!(find_node(&history, "framework.history.editor").is_some(), "the draft editor is shown while editing");
    assert!(find_node(&history, "framework.history.editor.input.value").is_some(), "the input's control is derived from its descriptor");
    assert!(history.to_string().contains("Verlaufsbearbeitung"), "German copy in a German shell");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    assert!(app.handle_action("undo", None, &meta(&fixture)).await.is_ok(), "exit lifts the freeze");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// ⚖️ LAW: cancelling a running replay keeps the drafts and reviews without a report; a remote edit arriving while
/// reviewing moves the base and replays again, and the reviewed head includes the remote edit.
#[semio_framework_async_macros::async_test]
async fn cancel_keeps_drafts_and_a_remote_edit_replays_again() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let (backbone, mut probe) = MemoryBackbone::pair("time-travel-remote", "time-travel-remote").await;
    app.attach_backbone(store::Backbones::Memory(backbone)).await.expect("attach probe backbone");
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null })] {
        run_step(&mut app, &fixture, &step).await;
    }
    let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new()).await;
    assert_eq!(rejected(&cancelled), None);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    assert!(app.time_travel.session().report.is_none() && app.time_travel.session().accepted.len() == 1, "the draft stays, no report");
    let status = app.time_travel.status().expect("a reviewing session");
    assert_eq!((status.review, status.rerunnable, status.fault.as_deref()), (Some(semio_framework::kernel::HistoryTimeTravelReview::NeedsReplay), true, Some(semio_framework_time_travel::TIME_TRAVEL_CANCELLED_CODE)), "a cancelled replay needs a rerun, never reads as no changes");
    let history = render_history(&mut app, Locale::En).await;
    assert!(find_node(&history, "framework.history.timeTravel.rerun").is_some_and(|rerun| rerun["disabled"] != Value::Bool(true)), "the band offers Replay again");
    let rerun = verb(&mut app, &fixture, "historyEditRerun", Vec::new()).await;
    assert_eq!(rejected(&rerun), None);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying, "rerun replays the kept drafts");
    run_step(&mut app, &fixture, &serde_json::json!({ "replay": "clean" })).await;
    assert_eq!(app.time_travel.status().and_then(|status| status.review), Some(semio_framework::kernel::HistoryTimeTravelReview::Ready));
    let mut remote = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    let (remote_backbone, mut remote_probe) = MemoryBackbone::pair("time-travel-remote-peer", "time-travel-remote-peer").await;
    remote.attach_backbone(store::Backbones::Memory(remote_backbone)).await.expect("attach remote probe");
    remote.store.set_local_actor_id(Some("remote".into())).expect("remote actor");
    remote.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], description: None, transaction: None }).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.store.generation();
    app.tick_backbone().await.expect("ingest remote edit");
    assert!(app.store.generation() > generation, "the remote edit is ingested while the session reviews");
    pump_until(&mut app, "the moved base replays again", |app| app.time_travel.session().stage == TimeTravelStage::Reviewing && app.time_travel.session().report.is_some() && app.time_travel.session().base.store_generation == app.store.generation()).await;
    assert!(render_body(&mut app).await.contains("label=b"), "the replayed head keeps the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    drop((probe, remote_probe));
    close(&mut app);
    close(&mut remote);
}

/// ⚖️ LAW: one committed tool transaction is one edit and one history row carrying its `TransactionRef`, labelled from
/// its mutations when it has no description, with one editable mutation row per op; a transaction riding a coalesced
/// amend is refused.
#[semio_framework_async_macros::async_test]
async fn a_committed_tool_transaction_is_one_row_with_its_reference_and_mutation_rows() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let edits = app.store.envelope().vcs.edits.len();
    let transaction = protocol::TransactionRef { id: "tx-0123456789abcdef".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    let emit = Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), vec![SetCount { value: 2 }.into(), SetLabel { value: "t".into() }.into()]);
    app.dispatch_emit("select", emit, &meta(&fixture)).await.expect("the transaction publishes");
    assert_eq!(app.store.envelope().vcs.edits.len(), edits + 1, "one transaction is one edit");
    let patch = app.history_patch(true).await.expect("history patch");
    let row = patch.upserts.iter().find(|entry| entry.transaction.is_some()).expect("the transaction row");
    assert_eq!(row.transaction.as_ref().map(|reference| (reference.id.as_str(), reference.tool.as_str())), Some((transaction.id.as_str(), transaction.tool.as_str())));
    assert_eq!(row.mutations.len(), 2, "one mutation row per op");
    assert!(row.mutations.iter().all(|mutation| mutation.editable), "both leaves declare an input schema");
    assert_eq!(row.label.resolve(Terminology::Native, Locale::En), "Set count to 2 (+1)", "the row reads its mutations' labels");
    assert_eq!(row.key(), format!("edit:{}", row.edit_id.as_deref().expect("edit id")));
    assert!(!row.op_lines.is_empty(), "the wire row keeps its printed operations");
    let history = render_history(&mut app, Locale::En).await;
    let mut rendered = find_node(&history, &format!("framework.history.entry.{}", row.seq)).unwrap_or_else(|| panic!("the transaction row: {history}")).clone();
    rendered.as_object_mut().expect("a node").remove("children");
    assert!(row.op_lines.iter().all(|line| !rendered.to_string().contains(line.as_str())), "labelled mutation children replace the raw op-lines description: {rendered}");
    let coalesced = Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { coalesce_key: Some("drag".into()), ..Emit::commit_transaction(transaction, vec![SetCount { value: 3 }.into()]) };
    assert_eq!(app.dispatch_emit("select", coalesced, &meta(&fixture)).await.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.shape")));
    close(&mut app);
}

/// 🧭️ The pointer and coercion helpers behind `historyEditInput`: object members and array slots are addressed by
/// RFC 6901 pointers, host values take the input's declared shape, and a pointer into a scalar is refused.
#[test]
fn input_pointers_and_host_values_take_the_declared_shape() {
    let mut value = DslValue::object([("pivot".to_string(), DslValue::object([("x".to_string(), DslValue::float(1.0))])), ("targets".to_string(), DslValue::Array(vec![DslValue::String("a".into())]))]);
    assert!(time_travel::time_travel_pointer_set(&mut value, "/pivot/x", DslValue::float(2.0)));
    assert!(time_travel::time_travel_pointer_set(&mut value, "/targets/1", DslValue::String("b".into())));
    assert!(!time_travel::time_travel_pointer_set(&mut value, "/pivot/x/deeper", DslValue::Null));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/pivot/x").and_then(DslValue::as_f64), Some(2.0));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/targets/1").and_then(DslValue::as_str), Some("b"));
    let count = semio_framework::ActionArgDef::index("/value", LocalizedLabel::native("Value", "Wert"));
    assert_eq!(time_travel::time_travel_coerce(&count, false, &DslValue::String("4".into())), Some(DslValue::uint(4)));
    assert_eq!(time_travel::time_travel_coerce(&count, false, &DslValue::String("four".into())), None);
    let flag = semio_framework::ActionArgDef::toggle("/on", LocalizedLabel::native("On", "An"));
    assert_eq!(time_travel::time_travel_coerce(&flag, false, &DslValue::String("true".into())), Some(DslValue::Bool(true)));
    let angle = semio_framework::ActionArgDef::number("/angle", LocalizedLabel::native("Angle", "Winkel"));
    let handles = semio_framework::ActionArgDef { schema: semio_framework::ArgSchema::Array { items: Box::new(semio_framework::ArgSchema::Object { fields: vec![angle] }), min_items: None, max_items: None }, ..semio_framework::ActionArgDef::text("/handles", LocalizedLabel::native("Handles", "Griffe")) };
    let inputs = vec![semio_framework::ActionArgDef::object("/pivot", LocalizedLabel::native("Pivot", "Drehpunkt"), vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("X", "X"))]), semio_framework::ActionArgDef::vector("/offset", LocalizedLabel::native("Offset", "Versatz"), 2), handles];
    let payload = dsl(&serde_json::json!({ "pivot": { "x": 1.0 }, "offset": [0.0, 0.0], "handles": [{ "angle": 0.0 }, { "angle": 90.0 }] }));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/pivot/x").is_some_and(|(input, element)| input.id == "/x" && !element));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/offset/1").is_some_and(|(input, element)| input.id == "/offset" && element));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/1/angle").is_some_and(|(input, element)| input.id == "/angle" && !element && matches!(input.schema, semio_framework::ArgSchema::Number { .. })), "a field inside an array of objects is addressable");
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/1").is_some_and(|(input, element)| !element && matches!(input.schema, semio_framework::ArgSchema::Object { .. })), "an item takes the array's item schema");
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/x/angle").is_none() && time_travel::time_travel_input_at(&inputs, &payload, "/missing").is_none());
    let rows = time_travel::time_travel_input_rows(&inputs, &payload);
    assert_eq!(rows.iter().map(|row| row.pointer.as_str()).collect::<Vec<_>>(), ["/pivot/x", "/offset", "/handles/0/angle", "/handles/1/angle"], "objects and arrays of objects flatten into field rows");
    assert_eq!((rows[3].label.resolve(Terminology::Native, Locale::En), rows[3].label.resolve(Terminology::Native, Locale::De), rows[3].value.as_f64()), ("Handles 2 \u{b7} Angle", "Griffe 2 \u{b7} Winkel", Some(90.0)));
}

/// 🔀️ A union payload's inputs resolve against the variant the draft is in: the reader groups every variant's fields by
/// the variant's value (a glTF `create-node` declares `/value` once per variant), the editor shows the variant selector
/// and the active variant's fields only (a hidden restore diff never), switching the selector drops the members only
/// other variants declare, and layout `group`s of a payload that is no union hide nothing.
#[test]
fn union_inputs_resolve_against_the_active_variant() {
    let option = |value: &str, en: &str, de: &str| semio_framework::ActionArgOption { value: value.into(), label: LocalizedLabel::native(en, de) };
    let phase = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::String { options: vec![option("apply", "Apply", "Anwenden"), option("restore", "Restore", "Wiederherstellen")], min_len: None, max_len: None, pattern: None, format: None },
        required: true,
        ..semio_framework::ActionArgDef::text("/phase", LocalizedLabel::native("Mutation phase", "Mutationsphase"))
    };
    let parameters = semio_framework::ActionArgDef { group: Some("apply".into()), ..semio_framework::ActionArgDef::object("/value", LocalizedLabel::native("Parameters", "Parameter"), vec![semio_framework::ActionArgDef::index("/position", LocalizedLabel::native("Insert position", "Einfügeposition"))]) };
    let weight = semio_framework::ActionArgDef { group: Some("apply".into()), ..semio_framework::ActionArgDef::number("/weight", LocalizedLabel::native("Weight", "Gewicht")) };
    let diff = semio_framework::ActionArgDef { group: Some("restore".into()), presentation: Some(semio_framework::ArgPresentation::Hidden), schema: semio_framework::ArgSchema::Any, ..semio_framework::ActionArgDef::text("/value", LocalizedLabel::native("Restore diff", "Wiederherstellungs-Diff")) };
    let union = vec![phase, parameters, weight, diff];
    assert_eq!(time_travel::time_travel_variant_selector(&union).map(|selector| selector.id.as_str()), Some("/phase"));
    let applying = dsl(&serde_json::json!({ "phase": "apply", "value": { "position": 2 }, "weight": 1.0 }));
    let restoring = dsl(&serde_json::json!({ "phase": "restore", "value": { "nodes": [] } }));
    assert!(time_travel::time_travel_input_at(&union, &applying, "/value/position").is_some_and(|(input, _)| input.id == "/position"), "the apply variant's parameters are addressable");
    assert!(time_travel::time_travel_input_at(&union, &restoring, "/value").is_some_and(|(input, _)| input.group.as_deref() == Some("restore")), "the restore variant's /value wins while restoring");
    assert!(time_travel::time_travel_input_at(&union, &restoring, "/value/position").is_none() && time_travel::time_travel_input_at(&union, &restoring, "/weight").is_none(), "another variant's fields are not addressable");
    let pointers = |value: &DslValue| time_travel::time_travel_input_rows(&union, value).into_iter().map(|row| row.pointer).collect::<Vec<_>>();
    assert_eq!(pointers(&applying), ["/phase", "/value/position", "/weight"]);
    assert_eq!(pointers(&restoring), ["/phase"], "a hidden restore diff is never a row");
    let switched = time_travel::time_travel_switch_variant(&union, &union[0], &applying, "restore");
    assert_eq!(switched, dsl(&serde_json::json!({ "phase": "restore", "value": { "position": 2 } })), "members only other variants declare are dropped");
    let laid_out = vec![semio_framework::ActionArgDef { group: Some("position".into()), ..semio_framework::ActionArgDef::number("/newX", LocalizedLabel::native("X", "X")) }, semio_framework::ActionArgDef { group: Some("target".into()), ..semio_framework::ActionArgDef::text("/id", LocalizedLabel::native("Target region", "Zielregion")) }];
    assert!(time_travel::time_travel_variant_selector(&laid_out).is_none());
    assert_eq!(time_travel::time_travel_input_rows(&laid_out, &dsl(&serde_json::json!({ "newX": 1.0, "id": "r" }))).len(), 2, "layout groups hide nothing");
}

/// 🧲️ LAW: every `snapSource` is numbers before a host sees it. Puzzle 2d's position inputs (`newX`, `snapSource:
/// {config: "gridFactor"}`, here read by the real input reader) step by the configured grid factor, a bounded input
/// snaps to the factor's multiples, and a `Snapshot` pointer reads the document the open session previews.
#[semio_framework_async_macros::async_test]
async fn snap_sources_resolve_from_the_grid_factor_and_the_previewed_document() {
    let schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "https://json.schemas.assets.semio-tech.com/framework/os/plugin/test/snap-source/schema.json",
        "type": "object",
        "additionalProperties": false,
        "required": ["mutation", "newX", "radius"],
        "properties": {
            "mutation": { "const": "moveTargetRegion" },
            "newX": { "type": "number", "x-semio-ui": { "widget": "stepper", "label": { "en": "X", "de": "X" }, "step": 1, "precision": 2, "snapSource": { "config": "gridFactor" } } },
            "radius": { "type": "number", "minimum": 0, "maximum": 10, "x-semio-ui": { "widget": "slider", "label": { "en": "Radius", "de": "Radius" }, "snapSource": { "config": "gridFactor" } } }
        }
    }"#;
    let inputs = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document).expect("the puzzle-shaped schema reads");
    let mut rows = time_travel::time_travel_input_rows(&inputs, &dsl(&serde_json::json!({ "newX": 3.0, "radius": 1.0 })));
    let config = dsl(&serde_json::json!({ "gridFactor": 0.5 }));
    time_travel::time_travel_resolve_snaps(&mut rows, |source| match source {
        semio_framework::SnapSource::Config { key } => time_travel::time_travel_pointer_get(&config, &format!("/{key}")).and_then(DslValue::as_f64),
        _ => None,
    });
    assert!(matches!(rows[0].input.control(), semio_framework::ActionArgControl::Stepper { step: Some(step), .. } if step == 0.5), "newX steps by the grid factor: {:?}", rows[0].input.control());
    assert!(matches!(&rows[1].input.schema, semio_framework::ArgSchema::Number { snaps, snap_source: None, .. } if snaps.len() == 21 && snaps[1] == 0.5 && snaps[20] == 10.0), "a bounded radius snaps to the grid: {:?}", rows[1].input.schema);
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 3 })).await;
    let mut panel = app.time_travel.panel().expect("an editing session");
    let editor = panel.editor.as_mut().expect("the draft editor");
    let mut count = semio_framework::ArgSchema::number(Some(0.0), Some(20.0), None, false);
    if let semio_framework::ArgSchema::Number { snap_source, .. } = &mut count {
        *snap_source = Some(semio_framework::SnapSource::Snapshot { pointer: "/count".into() });
    }
    editor.rows[0].input.schema = count;
    app.resolve_time_travel_snaps(&mut panel, &ViewModel::default());
    let resolved = &panel.editor.as_ref().expect("the draft editor").rows[0].input.schema;
    assert!(matches!(resolved, semio_framework::ArgSchema::Number { snaps, snap_source: None, .. } if *snaps == vec![0.0, 5.0, 10.0, 15.0, 20.0]), "the previewed document's count is the grid: {resolved:?}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🎨️ LAW: the draft editor renders a colour input with the UI contract's `color_input` recipe (swatch, hex text and,
/// for four components, the opacity slider bound to the fourth component) and passes a vector input's snaps to every
/// axis; a hex commit keeps the stored opacity and the slider sets it alone.
#[semio_framework_async_macros::async_test]
async fn colour_inputs_render_the_contract_recipe_and_vector_axes_keep_their_snaps() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let mut offset = semio_framework::ActionArgDef::vector("/offset", LocalizedLabel::native("Offset", "Versatz"), 2);
    if let semio_framework::ArgSchema::Vector { snaps, .. } = &mut offset.schema {
        *snaps = vec![0.0, 0.5];
    }
    let tint = semio_framework::ActionArgDef { presentation: Some(semio_framework::ArgPresentation::Color), ..semio_framework::ActionArgDef::vector("/tint", LocalizedLabel::native("Tint", "Tönung"), 4) };
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![tint, offset];
    editor.value = dsl(&serde_json::json!({ "tint": [1.0, 0.5, 0.0, 0.25], "offset": [0.0, 0.0] }));
    let history = render_history(&mut app, Locale::De).await;
    let colour = find_node(&history, "framework.history.editor.input.tint").unwrap_or_else(|| panic!("the colour recipe: {history}"));
    let text = colour.to_string();
    assert!(text.contains("#ff8000") && text.contains("#ff800040") && text.contains("Deckkraft") && text.contains("0.25"), "swatch, hex with alpha and the opacity slider: {colour}");
    let vector = find_node(&history, "framework.history.editor.input.offset").unwrap_or_else(|| panic!("the vector input: {history}"));
    assert!(vector.to_string().contains("0.5"), "every axis carries the vector's snaps: {vector}");
    let inputs = app.time_travel.editor().expect("editor").inputs.clone();
    let value = app.time_travel.editor().expect("editor").value.clone();
    let (hex_input, hex_element) = time_travel::time_travel_input_at(&inputs, &value, "/tint").expect("the colour input");
    assert_eq!(time_travel::time_travel_coerce(&hex_input, hex_element, &DslValue::String("#00ff00".into())).and_then(|value| value.as_array().map(<[DslValue]>::len)), Some(3), "a hex without alpha leaves the opacity to the stored value");
    let (alpha_input, alpha_element) = time_travel::time_travel_input_at(&inputs, &value, "/tint/3").expect("the opacity component");
    assert_eq!(time_travel::time_travel_coerce(&alpha_input, alpha_element, &DslValue::float(0.5)).and_then(|value| value.as_f64()), Some(0.5), "the slider sets the fourth component alone");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🧽️ LAW: a nullable input carries an accessible Clear button ("Clear <label>", en/de) that drafts `null` at the input's
/// pointer; while the input is null the row reads as cleared; a non-nullable input has none.
#[semio_framework_async_macros::async_test]
async fn a_nullable_input_offers_a_clear_control_that_drafts_null() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let scale = semio_framework::ActionArgDef { nullable: true, ..semio_framework::ActionArgDef::number("/scale", LocalizedLabel::native("Scale", "Maßstab")) };
    let width = semio_framework::ActionArgDef::number("/width", LocalizedLabel::native("Width", "Breite"));
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![scale, width];
    editor.value = dsl(&serde_json::json!({ "scale": 2.0, "width": 1.0 }));
    let history = render_history(&mut app, Locale::De).await;
    let clear = find_node(&history, "framework.history.editor.input.scale.clear").unwrap_or_else(|| panic!("the clear control: {history}"));
    assert!(clear.to_string().contains("Maßstab leeren"), "a Clear named for its input: {clear}");
    assert!(clear.to_string().contains(semio_framework::HISTORY_EDIT_INPUT_ACTION_ID) && clear.to_string().contains("/scale"), "Clear drafts the input's pointer: {clear}");
    assert!(find_node(&history, "framework.history.editor.input.width.clear").is_none(), "a non-nullable input has no Clear");
    let inputs = app.time_travel.editor().expect("editor").inputs.clone();
    let value = app.time_travel.editor().expect("editor").value.clone();
    let (input, element) = time_travel::time_travel_input_at(&inputs, &value, "/scale").expect("the nullable input");
    assert_eq!(time_travel::time_travel_coerce(&input, element, &DslValue::Null), Some(DslValue::Null), "Clear drafts null");
    app.time_travel.editor_mut().expect("the draft editor").value = dsl(&serde_json::json!({ "scale": null, "width": 1.0 }));
    let history = render_history(&mut app, Locale::En).await;
    let cleared = find_node(&history, "framework.history.editor.input.scale.clear").expect("the clear control");
    assert!(cleared.to_string().contains("Clear Scale"), "Clear stays named in every locale: {cleared}");
    assert!(find_node(&history, "framework.history.editor.input.scale.row").is_some_and(|row| row.to_string().contains("Cleared (no value)")), "the row reads as cleared");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🌲️ LAW: the band and the draft editor speak the one tree vocabulary both hosts mount — tree sections of rows, each row
/// holding at most one child that is not a row, and that child one single control (the kinds a wgpu row mounts as its
/// control; React mounts any): no text, progress bar or group is a row's child, and a second control is a nested row.
#[semio_framework_async_macros::async_test]
async fn the_band_and_the_editor_rows_hold_at_most_one_single_control() {
    const SINGLE_CONTROLS: [&str; 9] = ["input", "select", "toggle", "button", "keyValueList", "slider", "numberStepper", "ring", "iconSelect"];
    fn violations(node: &Value, found: &mut Vec<String>) {
        let children = node["children"].as_array().map(Vec::as_slice).unwrap_or_default();
        if node["component"]["type"].as_str() == Some("treeItem") {
            let controls: Vec<&str> = children.iter().filter_map(|child| child["component"]["type"].as_str()).filter(|kind| *kind != "treeItem").collect();
            if controls.len() > 1 || controls.iter().any(|kind| !SINGLE_CONTROLS.contains(kind)) {
                found.push(format!("{}: {controls:?}", node["key"]));
            }
        }
        for child in children {
            violations(child, found);
        }
    }
    fn sections(history: &Value) -> Vec<String> {
        let mut found = Vec::new();
        for key in ["framework.history.timeTravel", "framework.history.editor", "framework.history.editor.inputs"] {
            if let Some(section) = find_node(history, key) {
                assert_eq!(section["component"]["type"].as_str(), Some("treeSection"), "{key} is a tree section");
                violations(section, &mut found);
            }
        }
        found
    }
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![semio_framework::ActionArgDef { nullable: true, ..semio_framework::ActionArgDef::number("/scale", LocalizedLabel::native("Scale", "Maßstab")) }, semio_framework::ActionArgDef::number("/width", LocalizedLabel::native("Width", "Breite"))];
    editor.value = dsl(&serde_json::json!({ "scale": 2.0, "width": 1.0 }));
    let history = render_history(&mut app, Locale::En).await;
    assert!(find_node(&history, "framework.history.editor.input.scale.clear").is_some(), "the nullable input keeps its Clear: {history}");
    assert_eq!(sections(&history), Vec::<String>::new(), "editing rows");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step).await;
    }
    let history = render_history(&mut app, Locale::De).await;
    assert!(find_node(&history, "framework.history.timeTravel.finalize").is_some(), "the review offers Finalize: {history}");
    assert_eq!(sections(&history), Vec::<String>::new(), "reviewing rows");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// ⚖️ LAW (design §7): while a history edit is open the body leads with the session band and the draft editor, then the
/// alternatives, the actions and the commands; without a session the actions lead. The editor's inputs are never windowed:
/// the section carries no window, so every input row is materialised the moment it shows.
#[semio_framework_async_macros::async_test]
async fn the_session_leads_the_history_body_and_the_editor_inputs_are_all_materialised() {
    fn sections(body: &Value) -> Vec<&str> {
        body["children"].as_array().expect("sections").iter().filter_map(|section| section["key"].as_str()).collect()
    }
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    assert_eq!(sections(&render_history(&mut app, Locale::En).await), ["framework.history.actions", "framework.history.commands"]);
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = ["/dx", "/dy", "/dz"].map(|pointer| semio_framework::ActionArgDef::number(pointer, LocalizedLabel::native(pointer, pointer))).to_vec();
    editor.value = dsl(&serde_json::json!({ "dx": 1.0, "dy": 2.0, "dz": 3.0 }));
    let body = render_history(&mut app, Locale::En).await;
    assert_eq!(sections(&body), ["framework.history.timeTravel", "framework.history.editor", "framework.history.editor.inputs", "framework.history.actions", "framework.history.commands"]);
    let inputs = find_node(&body, "framework.history.editor.inputs").expect("the inputs section");
    assert!(inputs["component"]["window"].is_null(), "the inputs are never windowed: {inputs}");
    for pointer in ["dx", "dy", "dz"] {
        assert!(find_node(inputs, &format!("framework.history.editor.input.{pointer}.row")).is_some(), "the {pointer} row is materialised: {inputs}");
    }
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🧯️ LAW: the input path fails closed — when the edited leaf's payload schema compiles no validator, a draft is refused
/// with the typed `timeTravel.schema-unavailable` naming the reason on the editor, and the draft and session stay
/// untouched; never an unvalidated payload.
#[semio_framework_async_macros::async_test]
async fn a_schema_without_a_validator_refuses_every_draft() {
    let fixture = fixture();
    let refusals = &fixture["refusals"];
    let mut app = seeded_app(&fixture).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let generation = app.time_travel.session().generation;
    let before = app.time_travel.editor().expect("the draft editor").value.clone();
    app.time_travel.editor_mut().expect("the draft editor").refuse_validator("unresolved $ref");
    let refused = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::uint(9))]).await;
    assert_eq!(rejected(&refused), Some(text(&refusals["schemaUnavailable"])), "no validator, no draft");
    let editor = app.time_travel.editor().expect("the draft editor");
    assert!(editor.refused.as_ref().is_some_and(|(path, reason)| path == "/value" && reason.contains("unresolved $ref")), "the editor names why: {:?}", editor.refused);
    assert_eq!((&editor.value, app.time_travel.session().generation, app.time_travel.session().pending.as_ref().map(|pending| app.time_travel.session().unchanged(pending))), (&before, generation, Some(true)), "the draft and session are untouched");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// ⚖️ LAW: replay progress reaches every host without a dispatch — a driver turn that moves the replay ships a fresh
/// `HistoryPatch` whose `timeTravel` carries `done ≤ total` on the unsolicited UI progress frame, and the completion
/// ships the reviewing status the same way.
#[semio_framework_async_macros::async_test]
async fn replay_progress_rides_the_unsolicited_ui_frame_with_the_session_status() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let long: Vec<TestMutation> = (0..5_000).map(|value| SetCount { value }.into()).collect();
    app.store.dispatch(ArtifactCommand::Apply { mutations: long, description: None, transaction: None }).await.expect("a long edit applies");
    app.refresh_cache().await.expect("backfill");
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null })] {
        run_step(&mut app, &fixture, &step).await;
    }
    while app.take_typed_operation_ui_progress().is_some() {}
    let mut stages = Vec::new();
    let mut cursor = app.log_generation;
    for _ in 0..10_000 {
        app.advance_typed_operation_publication().await.expect("driver turn");
        while let Some(progress) = app.take_typed_operation_ui_progress() {
            let Some(patch) = progress.history_patch else { continue };
            assert!(patch.cursor > cursor, "every shipped patch advances the cursor");
            cursor = patch.cursor;
            let status = patch.time_travel.expect("the patch carries the session status");
            if status.stage == semio_framework::kernel::HistoryTimeTravelStage::Replaying {
                assert!(status.done.zip(status.total).is_some_and(|(done, total)| done <= total && total > 0), "progress is bounded: {status:?}");
            }
            stages.push(status.stage);
        }
        if app.time_travel.session().stage == TimeTravelStage::Reviewing && !app.time_travel.has_pending_work() {
            break;
        }
    }
    assert!(stages.contains(&semio_framework::kernel::HistoryTimeTravelStage::Replaying), "at least one progress status shipped: {stages:?}");
    assert_eq!(stages.last(), Some(&semio_framework::kernel::HistoryTimeTravelStage::Reviewing), "the completion ships the reviewing status");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// ⚖️ LAW: a stage change ships its session status in the turn it happens — Begin from a review bumps the generation,
/// and the editing stage with that generation rides both the command reply and the unsolicited UI progress frame of the
/// same turn (the one carrier of a retained-surface intent, whose reply has no patch), even while the review's snapshots
/// still retire; a verb stamped with the published generation applies, one stamped with the previous is stale.
#[semio_framework_async_macros::async_test]
async fn a_stage_change_ships_its_generation_in_the_same_turn_and_a_verb_stamped_with_it_applies() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step).await;
    }
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    while app.take_typed_operation_ui_progress().is_some() {}
    let reviewed = app.time_travel.session().generation;
    let mutation = seeded_mutation(&app, 2);
    let begun = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))]).await;
    let generation = app.time_travel.session().generation;
    assert_ne!(generation, reviewed, "Begin from a review bumps the generation");
    let status = |patch: &semio_framework::kernel::HistoryPatch| patch.time_travel.as_ref().map(|status| (status.stage, status.generation));
    let editing = Some((semio_framework::kernel::HistoryTimeTravelStage::Editing, generation));
    assert_eq!(begun.history_patch.as_ref().and_then(status), editing, "the reply publishes the new generation with the stage");
    let progress = app.take_typed_operation_ui_progress().expect("the stage change owes this turn a UI progress frame");
    assert_eq!(progress.history_patch.as_ref().and_then(status), editing, "the progress frame publishes it before any driver turn");
    assert!(progress.history_patch.as_ref().is_some_and(|patch| !patch.upserts.is_empty()), "it carries the rows the session overlay moved");
    assert!(app.time_travel.has_pending_work(), "the review's snapshots still retire behind it");
    let generation_arg = |value: u32| vec![(semio_framework::HISTORY_EDIT_ARG_GENERATION.to_string(), DslValue::uint(u64::from(value)))];
    let stale = verb(&mut app, &fixture, "historyEditDiscard", generation_arg(reviewed)).await;
    assert_eq!(rejected(&stale), Some("timeTravel.stale"), "the previous generation is stale");
    let discarded = verb(&mut app, &fixture, "historyEditDiscard", generation_arg(generation)).await;
    assert_eq!(rejected(&discarded), None, "the published generation applies: {:?}", discarded.output);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    let progress = app.take_typed_operation_ui_progress().expect("the return to the review ships at once too");
    assert_eq!(progress.history_patch.as_ref().and_then(status), Some((semio_framework::kernel::HistoryTimeTravelStage::Reviewing, app.time_travel.session().generation)));
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🌿️ LAW: the history body lists every line — the trunk (read "Main line", the original history) and the alternative a
/// finalize kept — with its current marker, author and branch time and whether it carries history edits, in English and
/// German; the other line offers Switch, whose rendered target dispatches `switchAlternative{alternativeId}`, and
/// switching reprojects: the edited alternative applies its history edit, the trunk shows the original.
#[semio_framework_async_macros::async_test]
async fn the_alternatives_section_lists_each_alternative_and_switching_reprojects_its_history_edits() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = replica(&fixture, "history-alternatives", &actor, true).await;
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original").await;
    assert_eq!(head(&app).1, "b", "the edited alternative is current");
    let history = app.test_history().await;
    let listed: Vec<(&str, bool, bool)> = history.alternatives.iter().map(|alternative| (alternative.name.as_str(), alternative.current, alternative.edited)).collect();
    assert_eq!(listed, vec![("", false, false), ("Variant", true, true)], "the trunk first, then the kept edit");
    let (trunk, variant) = (history.alternatives[0].id.clone(), history.alternatives[1].id.clone());
    assert_eq!(history.alternatives[1].author.as_deref(), Some(actor.as_str()));
    assert!(history.alternatives[1].branched_at.as_deref().is_some_and(|at| at.ends_with('Z')), "{:?}", history.alternatives[1]);
    let row_key = |id: &str| format!("framework.history.alternative.{id}");
    for (locale, main_line, current, edited, switch) in [(Locale::En, "Main line", "Current", "Edited history", "Switch"), (Locale::De, "Hauptlinie", "Aktuell", "Bearbeiteter Verlauf", "Wechseln")] {
        let body = render_history(&mut app, locale).await;
        let section = find_node(&body, "framework.history.alternatives").unwrap_or_else(|| panic!("the alternatives section: {body}"));
        assert_eq!(section["component"]["type"].as_str(), Some("treeSection"));
        let trunk_row = find_node(section, &row_key(&trunk)).expect("the trunk row");
        assert!(trunk_row.to_string().contains(main_line), "{locale:?}: {trunk_row}");
        let action = &trunk_row["component"]["rowActions"][0];
        assert_eq!((action["verb"].as_str(), action["label"].as_str()), (Some(semio_framework::SWITCH_ALTERNATIVE_ACTION_ID), Some(switch)), "{locale:?}: {trunk_row}");
        assert_eq!(trunk_row["component"]["target"]["activation"].as_str(), Some(semio_framework::SWITCH_ALTERNATIVE_ACTION_ID), "Enter on the row switches");
        let variant_row = find_node(section, &row_key(&variant)).expect("the variant row");
        assert!(variant_row.to_string().contains(current) && variant_row.to_string().contains(edited) && variant_row.to_string().contains(&actor), "{locale:?}: {variant_row}");
        assert!(variant_row["component"]["rowActions"].as_array().is_none_or(Vec::is_empty), "the current line offers no switch");
    }
    let body = render_history(&mut app, Locale::En).await;
    let args = find_node(&body, &row_key(&trunk)).expect("the trunk row")["component"]["target"]["args"].clone();
    assert_eq!(args[semio_framework::SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID].as_str(), Some(trunk.as_str()));
    history_verb(&mut app, &actor, semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, Some(dsl(&args))).await;
    assert_eq!(head(&app).1, "a", "the trunk shows the original history");
    let history = app.test_history().await;
    assert_eq!(history.alternatives.iter().map(|alternative| alternative.current).collect::<Vec<_>>(), vec![true, false]);
    history_verb(&mut app, &actor, semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, Some(dsl(&serde_json::json!({ "alternativeId": variant })))).await;
    assert_eq!(head(&app).1, "b", "switching back applies the history edit again");
    drop(probe);
    close(&mut app);
}

/// 📨️ LAW: opening a history edit delivers `TimeTravelFrozen` to every window of the view once (retargeting the open
/// session delivers none), and a remote edit moving the base delivers `BaseMoved` to every window of the last roster.
#[semio_framework_async_macros::async_test]
async fn opening_a_history_edit_and_a_remote_edit_deliver_host_events_to_every_window() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "host-events-local", &actor, true).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "host-events-remote", "remote", false).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    let roster = vec![semio_framework::ViewWindowInstance { id: "pane-a".into(), window_kind_id: "main".into() }, semio_framework::ViewWindowInstance { id: "pane-b".into(), window_kind_id: "main".into() }];
    let under = ActionMeta { view_state: Some(ViewModel { window_instances: roster, ..ViewModel::default() }), ..artifact_app_laws::meta(&actor) };
    let every = |event: fn(String) -> HostEvent| vec![event("pane-a".into()), event("pane-b".into())];
    TOY_HOST_EVENTS.with(|events| events.borrow_mut().clear());
    for index in [0, 1] {
        let mutation = seeded_mutation(&local, index);
        let begun = local.handle_action("historyEditBegin", Some(&DslValue::Object(vec![("mutationId".into(), DslValue::String(mutation))])), &under).await.expect("begin");
        assert_eq!(rejected(&begun), None);
    }
    assert_eq!(TOY_HOST_EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut())), every(|window_id| HostEvent::TimeTravelFrozen { window_id }), "only opening the session freezes the windows");
    local.handle_action("historyEditExit", None, &under).await.expect("exit");
    pump_until(&mut local, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    remote.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], description: None, transaction: None }).await.expect("a remote edit");
    TOY_HOST_EVENTS.with(|events| events.borrow_mut().clear());
    relay(&mut remote_probe, &mut local_probe, &mut local).await;
    assert_eq!(TOY_HOST_EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut())), every(|window_id| HostEvent::BaseMoved { window_id }), "the remote edit moved the base");
    drop((local_probe, remote_probe));
    close(&mut local);
    close(&mut remote);
}

/// ⚖️ LAW: a noted shell command keeps its label in every locale (an `{en, de}` pair), plain text stays data, a
/// detail is appended in every locale; undo is enabled wherever its chord is, even with nothing local to undo.
#[semio_framework_async_macros::async_test]
async fn noted_shell_commands_keep_every_locale_and_undo_matches_its_chord() {
    let fixture = fixture();
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    let history = render_history(&mut app, Locale::En).await;
    let undo = find_node(&history, "framework.history.undo").expect("undo row");
    assert_ne!(undo["disabled"], Value::Bool(true), "undo is enabled like its chord: the host routes a remote undo this guest cannot see");
    let args = DslValue::object([
        ("commandId".to_string(), DslValue::String("shell.export".into())),
        ("detail".to_string(), DslValue::String("a.png".into())),
        ("label".to_string(), DslValue::object([("de".to_string(), DslValue::String("Exportieren".into())), ("en".to_string(), DslValue::String("Export".into()))])),
    ]);
    let admitted = app.handle_action(NOTE_SHELL_COMMAND_ACTION_ID, Some(&args), &meta(&fixture)).await.expect("noteShellCommand admits");
    crate::app::settle_framework_reserved_admission(&mut app, admitted).await.expect("noteShellCommand settles");
    let patch = app.history_patch(true).await.expect("history patch");
    let row = patch.upserts.iter().find(|entry| entry.action_id == "shell.export").expect("the noted row");
    assert_eq!(row.label.resolve(Terminology::Native, Locale::En), "Export - a.png");
    assert_eq!(row.label.resolve(Terminology::Native, Locale::De), "Exportieren - a.png");
    assert!(viewer_rejects_action(semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID) && viewer_rejects_action("undo"), "a viewer rejects every history-edit verb");
    close(&mut app);
}

/// ⚖️ LAW: a ledger whose check-in replay blocks under `MergePolicy::Normal` crosses the guest codec boundary as the
/// typed `ledger-not-replayable` naming only its blocking messages; any other refusal keeps its own fault.
#[test]
fn a_blocking_ledger_replay_crosses_the_guest_boundary_as_ledger_not_replayable() {
    let fault = replay_envelopes_fault(store::VcsError::Rejected { policy: protocol::MergePolicy::Normal, messages: vec![protocol::MutationMessage::error("mutation.target-missing", "gone"), protocol::MutationMessage::warn("mutation.clamped", "clamped")] });
    assert_eq!(fault.code, FaultCode::new("ledger-not-replayable"));
    assert!(fault.message.contains("mutation.target-missing") && !fault.message.contains("mutation.clamped"), "{}", fault.message);
    assert_ne!(replay_envelopes_fault(store::VcsError::Deserialize("malformed".into())).code, FaultCode::new("ledger-not-replayable"));
}

/// 🧲️ A `Config`/`Snapshot` snap source becomes numbers on the wire: the grid multiples inside the travel range when at
/// most one fixed list of them fits, else the grid spacing as the step; a colour input takes `#rrggbb` text.
#[test]
fn snap_sources_resolve_to_numbers_and_colours_take_hex_text() {
    let grid = |min: f64, max: f64| semio_framework::ArgSchema::Number { min: Some(min), min_exclusive: false, max: Some(max), max_exclusive: false, step: None, integer: false, unit: None, snaps: Vec::new(), snap_source: Some(semio_framework::SnapSource::Config { key: "gridFactor".into() }), soft_min: None, soft_max: None, precision: None, display_unit: None, display_factor: None, scale: None };
    let mut fine = grid(-1.0, 1.0);
    time_travel::time_travel_apply_snap_spacing(&mut fine, Some(0.5));
    assert!(matches!(&fine, semio_framework::ArgSchema::Number { snaps, snap_source: None, step: None, .. } if *snaps == vec![-1.0, -0.5, 0.0, 0.5, 1.0]), "{fine:?}");
    let mut wide = grid(0.0, 1_000.0);
    time_travel::time_travel_apply_snap_spacing(&mut wide, Some(0.5));
    assert!(matches!(&wide, semio_framework::ArgSchema::Number { snaps, snap_source: None, step: Some(step), .. } if snaps.is_empty() && *step == 0.5), "{wide:?}");
    let mut unresolved = grid(0.0, 1.0);
    time_travel::time_travel_apply_snap_spacing(&mut unresolved, None);
    assert!(matches!(&unresolved, semio_framework::ArgSchema::Number { snap_source: None, step: None, .. }), "an unresolvable source never reaches a host");
    assert_eq!(time_travel::time_travel_color_components("#ff8000").map(|components| components.len()), Some(3));
    assert_eq!(time_travel::time_travel_color_components("#ff800040").map(|components| components.len()), Some(4), "a text with alpha sets the opacity too");
    assert_eq!(time_travel::time_travel_color_components("#f80").map(|components| components.len()), Some(3), "the contract's short form parses");
    let colour = semio_framework::ActionArgDef { presentation: Some(semio_framework::ArgPresentation::Color), ..semio_framework::ActionArgDef::vector("/tint", LocalizedLabel::native("Tint", "Tönung"), 4) };
    assert!(time_travel::time_travel_coerce(&colour, false, &DslValue::String("#00ff00".into())).is_some_and(|value| value.as_array().is_some_and(|components| components.len() == 3)), "an rgb pick keeps the alpha apart");
    assert_eq!(time_travel::time_travel_coerce(&colour, false, &DslValue::String("green".into())), None);
}

//#region 🪞️Replicas
/// 🪞️ A registered toy replica on its own memory backbone, authoring as `actor`; `seed` dispatches the fixture seed
/// through its store, so every seed edit leaves in its outbox.
async fn replica(fixture: &Value, uri: &str, actor: &str, seed: bool) -> (ToyApp, MemoryBackbone) {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    let (backbone, probe) = MemoryBackbone::pair(uri, uri).await;
    app.attach_backbone(store::Backbones::Memory(backbone)).await.expect("attach the replica backbone");
    app.store.set_local_actor_id(Some(actor.to_string())).expect("replica actor");
    if seed {
        for op in fixture["seed"].as_array().expect("seed") {
            app.store.dispatch(ArtifactCommand::Apply { mutations: vec![seed_op(op)], description: None, transaction: None }).await.expect("seed edit applies");
        }
    }
    app.refresh_cache().await.expect("the command log backfills");
    (app, probe)
}

/// 🔀️ Delivers every event batch `from` published since the last relay into `to` (through its probe `into`) and ingests it.
async fn relay(from: &mut MemoryBackbone, into: &mut MemoryBackbone, to: &mut ToyApp) {
    for message in from.receive().await.expect("replica outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        into.send(message).await.expect("relay an event batch");
    }
    to.tick_backbone().await.expect("ingest the relayed events");
    to.refresh_cache().await.expect("the command log backfills the relayed events");
}

/// 🏁️ Runs fixture scenario `id`, which ends in a commit, on `app`.
async fn commit_scenario(app: &mut ToyApp, fixture: &Value, id: &str) {
    let scenario = fixture["scenarios"].as_array().expect("scenarios").iter().find(|scenario| text(&scenario["id"]) == id).unwrap_or_else(|| panic!("scenario {id}"));
    for step in scenario["steps"].as_array().expect("steps") {
        if let Some(result) = run_step(app, fixture, step).await {
            assert_eq!(rejected(&result), None, "{id}: step {step} was refused");
        }
    }
    pump_until(app, "the finalize retires", |app| !app.time_travel.has_pending_work()).await;
}

/// ↩️ Runs a framework history verb (`undo`, `redo`, `revertToCommand`) as `actor` through its reserved admission.
async fn history_verb(app: &mut ToyApp, actor: &str, action: &str, args: Option<DslValue>) {
    let meta = ActionMeta { view_state: Some(ViewModel::default()), ..artifact_app_laws::meta(actor) };
    let admitted = app.handle_action(action, args.as_ref(), &meta).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    crate::app::settle_framework_reserved_admission(app, admitted).await.unwrap_or_else(|fault| panic!("{action} settles: {fault:?}"));
    app.refresh_cache().await.expect("history refresh");
}

/// 🧾️ The history-transition rows of `app`, newest first.
async fn history_edit_rows(app: &mut ToyApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    app.history_patch(true).await.expect("history patch").upserts.into_iter().filter(|row| row.transition_id.is_some()).collect()
}

fn english(rows: &[semio_framework::kernel::HistoryEntry]) -> Vec<&str> {
    rows.iter().map(|row| row.label.resolve(Terminology::Native, Locale::En)).collect()
}

/// ✏️ The history mutation row of seeded op `index`.
async fn seeded_mutation_row(app: &mut ToyApp, index: usize) -> semio_framework::kernel::HistoryMutationEntry {
    let target = seeded_mutation(app, index);
    app.history_patch(true).await.expect("history patch").upserts.into_iter().flat_map(|row| row.mutations).find(|mutation| mutation.mutation_id == target).expect("the seeded mutation row")
}

fn head(app: &ToyApp) -> (i32, String) {
    let snapshot = app.store.snapshot().expect("head");
    (snapshot.count, snapshot.label.clone())
}
//#endregion 🪞️Replicas

/// ⚖️ LAW: a finalized history edit is its own history row, derived from the store's `Supersede` transition on the
/// authoring replica, on a replica that ingested it and after a text or pack reload — labelled in every locale with its
/// scope and mutation count, its author and time — and the edited mutation reads superseded from the store alike.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_is_its_own_row_locally_remotely_and_after_reload() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "history-edit-row-local", &actor, true).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "history-edit-row-remote", "remote", false).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), (5, "a".to_string()), "the remote replica holds the seed");
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place").await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    reloaded.store.set_local_actor_id(Some(actor.clone())).expect("the author reopens the document");
    reloaded.load_document_text(&local.document_text().await.expect("document text")).await.expect("text reload");
    reloaded.refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    repacked.store.set_local_actor_id(Some("reader".into())).expect("another reader opens the document");
    repacked.load_document_pack(&local.document_pack().await.expect("document pack")).await.expect("pack reload");
    repacked.refresh_cache().await.expect("the repacked log backfills");
    for (who, app, revertible) in [("local", &mut local, true), ("remote", &mut remote, false), ("text reload", &mut reloaded, true), ("pack reload", &mut repacked, false)] {
        let rows = history_edit_rows(app).await;
        assert_eq!(english(&rows), ["History edited — overwrite: 1 mutation"], "{who}: one row per history edit, never a runtime duplicate");
        let row = &rows[0];
        assert_eq!(row.label.resolve(Terminology::Native, Locale::De), "Verlauf bearbeitet — überschrieben: 1 Mutation", "{who}");
        assert_eq!(row.author.as_deref(), Some(actor.as_str()), "{who}: the author");
        assert!(row.timestamp.parse::<u64>().is_ok_and(|ms| ms > 0), "{who}: the time, {}", row.timestamp);
        assert_eq!((row.kind.as_str(), row.action_id.as_str(), row.applied, row.revertible), ("history", semio_framework::HISTORY_EDIT_COMMIT_ACTION_ID, true, revertible), "{who}");
        assert_eq!(row.key(), format!("transition:{}", row.transition_id.as_deref().expect("transition id")), "{who}: the row folds under its transition");
        let mutation = seeded_mutation_row(app, 1).await;
        assert!(mutation.superseded && !mutation.withdrawn, "{who}: the edited mutation reads superseded from the store");
        assert_eq!(head(app), (5, "b".to_string()), "{who}: the edited head");
        let history = render_history(app, Locale::De).await;
        assert!(find_node(&history, &format!("framework.history.entry.{}", row.seq)).is_some(), "{who}: the panel shows the row");
    }
    drop((local_probe, remote_probe));
    for app in [&mut local, &mut remote, &mut reloaded, &mut repacked] {
        close(app);
    }
}

/// ⚖️ LAW: undo of a finalize is a new `Supersede` restoring the previous effective input (design §2), redo re-authors
/// the edit, both through the store's one supersede path and ingested alike by another replica; a newer own document
/// edit is undone first and redone last; the row's Backwards takes the edit back without touching document edits; and
/// another author can neither undo nor revert it.
#[semio_framework_async_macros::async_test]
async fn undo_and_redo_of_a_finalize_author_restoring_supersedes_on_both_replicas() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "history-edit-undo-local", &actor, true).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "history-edit-undo-remote", "remote", false).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place").await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    let edits = local.store.envelope().vcs.edits.len();

    history_verb(&mut local, &actor, "undo", None).await;
    assert_eq!(head(&local), (5, "a".to_string()), "the undo restores the original input");
    let rows = history_edit_rows(&mut local).await;
    assert_eq!(english(&rows), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    assert!(rows[0].applied && !rows[0].revertible && !rows[1].applied && !rows[1].revertible, "the undone history edit reads undone");
    assert!(!seeded_mutation_row(&mut local, 1).await.superseded, "an input restored to its original is not superseded");
    assert_eq!(local.store.envelope().vcs.edits.len(), edits, "the undo is a history transition, never a document edit");
    assert!(local.history_patch(true).await.expect("patch").can_redo, "the undone history edit can be redone");
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), (5, "a".to_string()), "the remote replica folds the restoring supersede");
    assert_eq!(english(&history_edit_rows(&mut remote).await), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);

    history_verb(&mut local, &actor, "redo", None).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the redo re-authors the edited input");
    let rows = history_edit_rows(&mut local).await;
    assert_eq!(english(&rows), ["History edit redone — overwrite: 1 mutation", "History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    assert!(rows[0].applied && !rows[1].applied && rows[2].applied && rows[2].revertible, "the redone history edit is in effect again");
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), (5, "b".to_string()));

    local.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 7 }.into()], description: None, transaction: None }).await.expect("a newer document edit");
    local.refresh_cache().await.expect("the newer edit is logged");
    history_verb(&mut local, &actor, "undo", None).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the newer own document edit is undone first");
    history_verb(&mut local, &actor, "undo", None).await;
    assert_eq!(head(&local), (5, "a".to_string()), "then the history edit");
    history_verb(&mut local, &actor, "redo", None).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the history edit undone last is redone first");
    history_verb(&mut local, &actor, "redo", None).await;
    assert_eq!(head(&local), (7, "b".to_string()), "then the document edit");

    let edited = history_edit_rows(&mut local).await.into_iter().rev().find(|row| row.revertible).expect("the revertible history edit row");
    history_verb(&mut local, &actor, REVERT_TO_COMMAND_ACTION_ID, Some(DslValue::object([("entrySeq".to_string(), DslValue::float(edited.seq as f64))]))).await;
    assert_eq!(head(&local), (7, "a".to_string()), "Backwards takes the history edit back and leaves the document edits");
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), (7, "a".to_string()), "the replicas converge");

    history_verb(&mut local, &actor, "redo", None).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!((head(&local), head(&remote)), ((7, "b".to_string()), (7, "b".to_string())));
    let foreign = history_edit_rows(&mut remote).await;
    assert!(foreign.iter().all(|row| !row.revertible && row.author.as_deref() == Some(actor.as_str())), "another author's history edits are never revertible here");
    history_verb(&mut remote, "remote", "undo", None).await;
    let original = foreign.iter().rev().find(|row| row.label.resolve(Terminology::Native, Locale::En).starts_with("History edited")).expect("the history edit row");
    history_verb(&mut remote, "remote", REVERT_TO_COMMAND_ACTION_ID, Some(DslValue::object([("entrySeq".to_string(), DslValue::float(original.seq as f64))]))).await;
    assert_eq!(head(&remote), (7, "b".to_string()), "another author can neither undo nor revert the history edit");
    drop((local_probe, remote_probe));
    close(&mut local);
    close(&mut remote);
}

/// ⚖️ LAW: a history edit committed as a new alternative is undone and redone within that alternative: its rows name
/// the alternative, the restoring supersede is scoped to it, and the alternative stays active.
#[semio_framework_async_macros::async_test]
async fn an_alternative_history_edit_is_undone_within_its_alternative() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = replica(&fixture, "history-edit-alternative", &actor, true).await;
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original").await;
    let variant = app.store.envelope().active_alternative_id.clone().expect("the new alternative is active");
    assert_eq!(english(&history_edit_rows(&mut app).await), ["History edited — alternative Variant: 1 mutation"]);
    history_verb(&mut app, &actor, "undo", None).await;
    assert_eq!(head(&app), (5, "a".to_string()), "the undo restores the original within the alternative");
    let rows = history_edit_rows(&mut app).await;
    assert_eq!(english(&rows), ["History edit undone — alternative Variant: 1 mutation", "History edited — alternative Variant: 1 mutation"]);
    assert_eq!(rows[1].label.resolve(Terminology::Native, Locale::De), "Verlauf bearbeitet — Alternative Variant: 1 Mutation");
    assert!(app.store.supersessions().values().all(|supersession| supersession.scope.as_deref() == Some(variant.as_str()) && Some(supersession.transition_id.as_str()) == rows[0].transition_id.as_deref()), "the restoring supersede is scoped to the alternative");
    assert_eq!(app.store.envelope().active_alternative_id.as_deref(), Some(variant.as_str()), "the alternative stays active");
    history_verb(&mut app, &actor, "redo", None).await;
    assert_eq!(head(&app), (5, "b".to_string()));
    drop(probe);
    close(&mut app);
}
