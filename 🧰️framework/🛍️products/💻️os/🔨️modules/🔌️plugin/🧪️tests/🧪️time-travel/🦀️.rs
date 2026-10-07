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
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture value is JSON"))
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

    fn host_event(event: &HostEvent) -> Option<TestMutation> {
        TOY_HOST_EVENTS.with(|events| events.borrow_mut().push(event.clone()));
        None
    }

    fn entity_label(_snapshot: &TestSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        let child = id.split('!').next().filter(|child| !child.starts_with("anon") && kinds.iter().any(|kind| kind == "s.test.child"))?;
        Some(LocalizedLabel::native(&format!("Child {child}"), &format!("Kind {child}")))
    }

    fn tool_intent_kinds(tool: &str) -> &'static [&'static str] {
        if tool.ends_with("#gesture") {
            &["set-label"]
        } else {
            &[]
        }
    }

    fn catalogue_example_document(example_id: &str) -> Option<Result<String, Fault>> {
        let example = TestSnapshot { count: 5, ..TestSnapshot::default() };
        (example_id == "five").then(|| Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&example)))))
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
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> ArtifactMutationOutcome<TestMutation, TestConfigMutation, NoDraftMutation> {
        Ok(Emit { artifact_mutations: vec![command.clone()], ..Default::default() })
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={} label={}", doc.snapshot.count, doc.snapshot.label)))
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
    seeded_app_as(fixture, text(&fixture["actor"])).await
}

/// 🪪️ Opens the toy under its acting actor while preserving each neutral seed edit's author.
async fn seeded_app_as(fixture: &Value, actor: &str) -> ToyApp {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.into())).await;
    assert_eq!(app.store.local_actor_id(), &protocol::ActorId(actor.into()));
    for op in fixture["seed"].as_array().expect("seed") {
        publish_as(&mut app, text(&fixture["actor"]), seed_op(op));
    }
    app.refresh_cache().await.expect("the command log backfills the seed");
    app
}

fn meta(fixture: &Value) -> ActionMeta {
    ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(text(&fixture["actor"])) }
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
    let tree = app.render("time-travel.main", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.unwrap_or_else(|fault| panic!("render: {fault:?}"));
    artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error| panic!("project: {error}"))
}

async fn render_history(app: &mut ToyApp, locale: Locale) -> Value {
    let view = ViewModel { locale, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
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
        "withdrawRow" | "restore" => {
            let mutation = seeded_mutation(app, value.as_u64().expect("edit index") as usize);
            verb(app, fixture, if name == "restore" { "historyEditRestore" } else { "historyEditWithdraw" }, vec![("mutationId".into(), DslValue::String(mutation))]).await
        }
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
                let problem = expected.get("nextProblem").and_then(Value::as_u64).map(|index| seeded_mutation(&app, index as usize));
                assert_eq!(status.next_problem.as_ref().map(|problem| (problem.mutation_id.clone(), problem.store.clone())), problem.map(|mutation| (mutation, None)), "{id}: the next problem");
                assert_eq!(status.next_problem.is_some(), status.blocking, "{id}: a review names its next problem exactly while it blocks");
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
    let mut fresh = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into())).await;
    for (index, op) in fixture["seed"].as_array().expect("seed").iter().enumerate() {
        let op = if index == 1 { SetLabel { value: "b".into() }.into() } else { seed_op(op) };
        fresh.store.dispatch(ArtifactCommand::Apply { mutations: vec![op], transaction: None }).await.expect("fresh edit");
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
    assert_eq!(
        (status.review, status.rerunnable, status.fault.as_deref()),
        (Some(semio_framework::kernel::HistoryTimeTravelReview::NeedsReplay), true, Some(semio_framework_time_travel::TIME_TRAVEL_CANCELLED_CODE)),
        "a cancelled replay needs a rerun, never reads as no changes"
    );
    let history = render_history(&mut app, Locale::En).await;
    assert!(find_node(&history, "framework.history.timeTravel.rerun").is_some_and(|rerun| rerun["disabled"] != Value::Bool(true)), "the band offers Replay again");
    let rerun = verb(&mut app, &fixture, "historyEditRerun", Vec::new()).await;
    assert_eq!(rejected(&rerun), None);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying, "rerun replays the kept drafts");
    run_step(&mut app, &fixture, &serde_json::json!({ "replay": "clean" })).await;
    assert_eq!(app.time_travel.status().and_then(|status| status.review), Some(semio_framework::kernel::HistoryTimeTravelReview::Ready));
    let mut remote = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("remote".into())).await;
    let (remote_backbone, mut remote_probe) = MemoryBackbone::pair("time-travel-remote-peer", "time-travel-remote-peer").await;
    remote.attach_backbone(store::Backbones::Memory(remote_backbone)).await.expect("attach remote probe");
    assert_eq!(remote.store.local_actor_id(), &protocol::ActorId("remote".into()));
    remote.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.store.generation();
    app.tick_backbone().await.expect("ingest remote edit");
    assert!(app.store.generation() > generation, "the remote edit is ingested while the session reviews");
    pump_until(&mut app, "the moved base replays again", |app| app.time_travel.session().stage == TimeTravelStage::Reviewing && app.time_travel.session().report.is_some() && app.time_travel.session().base.content_revision == app.store.content_revision())
        .await;
    assert!(render_body(&mut app).await.contains("label=b"), "the replayed head keeps the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    drop((probe, remote_probe));
    close(&mut app);
    close(&mut remote);
}

/// ⚖️ LAW: one committed tool transaction is one edit and one history row carrying its `TransactionRef`, labelled from
/// its mutations when it has no description, with one editable mutation row per op.
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
    close(&mut app);
}

/// ⚖️ LAW (design §19.1): a tool that declares intent leaf kinds labels its transaction's row by the first operation of
/// such a kind — a support leaf before it counts only in `(+N)` — on the authoring replica and after a text and a pack
/// reload (the stamped tool id and the leaf kinds are persisted); a tool declaring none keeps the first leaf's label.
#[semio_framework_async_macros::async_test]
async fn a_transaction_row_reads_its_declared_intent_leaf_before_and_after_reload() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let support_then_intent = || -> Vec<TestMutation> { vec![SetCount { value: 2 }.into(), SetLabel { value: "t".into() }.into()] };
    assert_eq!(support_then_intent().iter().map(|op| protocol::SemanticMutation::<TestSnapshot>::semantics(op).kind).collect::<Vec<_>>(), ["set-count", "set-label"]);
    let gesture = protocol::TransactionRef { id: "tx-00000000000000c1".into(), tool: format!("{}#gesture", ToyHistoryApp::APP_ID) };
    let select = protocol::TransactionRef { id: "tx-00000000000000c2".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    for transaction in [&gesture, &select] {
        app.dispatch_emit("select", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), support_then_intent()), &meta(&fixture)).await.expect("the transaction publishes");
    }
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into())).await;
    artifact_app_laws::load_document_text(&mut reloaded, &app.document_text().await.expect("document text")).await.expect("text reload");
    reloaded.refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into())).await;
    artifact_app_laws::load_document(&mut repacked, &app.document_pack().await.expect("document pack")).await.expect("pack reload");
    repacked.refresh_cache().await.expect("the repacked log backfills");
    for (who, app) in [("authored", &mut app), ("text reload", &mut reloaded), ("pack reload", &mut repacked)] {
        let patch = app.history_patch(true).await.expect("history patch");
        let label = |transaction: &protocol::TransactionRef| {
            let row = patch.upserts.iter().find(|entry| entry.transaction.as_ref().is_some_and(|reference| reference.id == transaction.id)).unwrap_or_else(|| panic!("{who}: the row of {}", transaction.id));
            (row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string())
        };
        assert_eq!(label(&gesture), ("Set label to t (+1)".to_string(), "Beschriftung auf t setzen (+1)".to_string()), "{who}: the declared intent leaf labels the row");
        assert_eq!(label(&select), ("Set count to 2 (+1)".to_string(), "Anzahl auf 2 setzen (+1)".to_string()), "{who}: an undeclared tool keeps the first leaf");
    }
    for app in [&mut app, &mut reloaded, &mut repacked] {
        close(app);
    }
}

/// 🏷️ LAW (design §20.6): the runtime's catalogue route (`setActiveExample` of an app that declares none) carries no
/// hand-written description — its verb reads the framework's own label in every locale, any row it records reads that
/// label — and its emit still loads the example.
#[semio_framework_async_macros::async_test]
async fn the_catalogue_route_reads_the_framework_label_never_a_hand_written_one() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let label = app.verb_label(CATALOGUE_EXAMPLE_ACTION_ID).expect("the framework labels its runtime verb");
    assert_eq!((label.resolve(Terminology::Native, Locale::En), label.resolve(Terminology::Native, Locale::De)), ("Load example", "Beispiel laden"));
    let result = app.dispatch_action(CATALOGUE_EXAMPLE_ACTION_ID, Some(&dsl(&serde_json::json!({ "exampleId": "five" }))), &meta(&fixture)).await.expect("the catalogue route loads the example");
    assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })), "the example is loaded: {:?}", result.requested_effects);
    let patch = app.history_patch(true).await.expect("history patch");
    assert!(patch.upserts.iter().filter(|row| row.action_id == CATALOGUE_EXAMPLE_ACTION_ID).all(|row| row.label.resolve(Terminology::Native, Locale::En) == "Load example"), "no row reads a hand-written label");
    close(&mut app);
}

/// 🪟️ The history body as a host shows it with history row `seq`'s mutation window open on `rows` rows from `offset`.
async fn render_history_window(app: &mut ToyApp, seq: u64, offset: u32, rows: u32) -> Value {
    let request = semio_framework::TreeWindowRequest { body_key: FRAMEWORK_HISTORY_BODY_KEY.into(), node_key: time_travel::history_row_window_path(seq), open: Some(true), offset, rows };
    let view = ViewModel { tree_windows: vec![request], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let tree = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &view).await.unwrap_or_else(|fault| panic!("history render: {fault:?}"));
    serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(tree).expect("history projection")).expect("history projection parses")
}

/// 🧒️ The mutation ids of the mutation rows a rendered history row materialised, in row order.
fn mutation_row_ids(row: &Value) -> Vec<String> {
    row["children"].as_array().into_iter().flatten().filter_map(|child| child["key"].as_str()?.strip_prefix("framework.history.mutation.").map(str::to_string)).collect()
}

/// ⚖️ LAW (gaps N1, N15): every mutation of a long transaction is reachable from its history row — the row is a tree window
/// over all of them (the flagged one first, then the projected ones, then the rest of its edit read on demand) while the
/// wire stays bounded; a row holding a warning opens by default with the warning first, a row without one stays closed,
/// and a window opened past the projection shows the 33rd…40th mutations with their Edit action, which opens the editor on
/// them. The Edit action is disabled, naming why and activating nothing, exactly where `Begin` is refused: a changed draft
/// still open, a running replay; it is enabled again in the review.
#[semio_framework_async_macros::async_test]
async fn every_mutation_of_a_long_transaction_is_reachable_and_edit_follows_the_begin_law() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let mut ops: Vec<TestMutation> = (1..40).map(|value| SetCount { value }.into()).collect();
    ops.push(SetLabel { value: head(&app).1 }.into());
    let transaction = protocol::TransactionRef { id: "tx-0000000000000040".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    app.dispatch_emit("select", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), ops), &meta(&fixture)).await.expect("the long transaction publishes");
    let row = app.history_patch(true).await.expect("history patch").upserts.into_iter().find(|entry| entry.transaction.as_ref().is_some_and(|reference| reference.id == transaction.id)).expect("the transaction row");
    let ids: Vec<String> = app.store.mutation_ops().expect("applied operations").iter().filter(|op| row.edit_id.as_deref() == Some(op.edit_id)).map(|op| op.mutation_id.0.clone()).collect();
    assert_eq!((row.op_count, ids.len()), (40, 40), "one edit of forty operations");
    assert!(
        row.mutations.len() < 40 && row.mutations.iter().any(|mutation| mutation.mutation_id == ids[39] && mutation.worst == Some(semio_framework_diagnostic::Severity::Warning)),
        "the wire stays bounded and keeps the flagged no-op: {}",
        row.mutations.len()
    );
    let key = format!("framework.history.entry.{}", row.seq);

    let unrequested = render_history(&mut app, Locale::En).await;
    let opened = find_node(&unrequested, &key).unwrap_or_else(|| panic!("the transaction row: {unrequested}"));
    assert_eq!((opened["component"]["window"]["total"].as_u64(), mutation_row_ids(opened).first()), (Some(40), Some(&ids[39])), "a row holding a warning opens by default, the warning first, announcing all forty: {opened}");
    let seed_row = app.history_patch(true).await.expect("history patch").upserts.into_iter().find(|entry| entry.edit_id.is_some() && entry.transaction.is_none() && !entry.mutations.is_empty()).expect("a seed row");
    assert!(mutation_row_ids(find_node(&unrequested, &format!("framework.history.entry.{}", seed_row.seq)).expect("the seed row")).is_empty(), "a row without an outcome stays closed");
    let first = render_history_window(&mut app, row.seq, 0, 8).await;
    assert_eq!(mutation_row_ids(find_node(&first, &key).expect("the open row")), [&ids[39..], &ids[..7]].concat(), "the flagged mutation leads, then op order");
    let last = render_history_window(&mut app, row.seq, 32, 8).await;
    let tail = find_node(&last, &key).expect("the row opened past its projection");
    assert_eq!(mutation_row_ids(tail), ids[31..39].to_vec(), "the 33rd to 40th rows: the last projected one, then the rest of the edit");
    let edited = find_node(tail, &format!("framework.history.mutation.{}", ids[38])).expect("the 39th mutation row");
    assert_eq!(
        (edited["component"]["rowActions"][0]["verb"].as_str(), edited["component"]["target"]["args"]["mutationId"].as_str(), edited["component"]["target"]["activation"].as_str()),
        (Some("historyEditBegin"), Some(ids[38].as_str()), Some("historyEditBegin")),
        "{edited}"
    );

    let begun = app.handle_action("historyEditBegin", Some(&dsl(&edited["component"]["target"]["args"])), &meta(&fixture)).await.expect("Edit dispatches");
    assert_eq!(rejected(&begun), None, "{:?}", begun.output);
    assert_eq!(app.time_travel.editor().map(|editor| editor.target.0.clone()), Some(ids[38].clone()), "Edit opens the 39th mutation");
    let edit_action = |body: &Value, id: &str| {
        find_node(body, &format!("framework.history.mutation.{id}")).map(|row| (row["component"]["rowActions"][0].clone(), row["component"]["target"]["activation"].clone())).unwrap_or_else(|| panic!("the row of {id}: {body}"))
    };
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] != Value::Bool(true) && activation.as_str() == Some("historyEditBegin"), "an unchanged draft lets Edit switch targets: {action}");
    verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::uint(77))]).await;
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(
        action["disabled"] == Value::Bool(true) && activation.is_null() && action["label"].as_str() == Some("Edit") && action["reason"].as_str().is_some_and(|reason| reason.starts_with("Blocked")),
        "a changed draft blocks Edit, naming why: {action}"
    );
    verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] == Value::Bool(true) && activation.is_null() && action["label"].as_str() == Some("Edit") && action["reason"].as_str() == Some("Not possible right now"), "a running replay disables Edit, naming why: {action}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] != Value::Bool(true) && activation.as_str() == Some("historyEditBegin") && action["label"].as_str() == Some("Edit"), "the review edits again: {action}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🌊️ LAW (design §15, transaction-scoped amend): a streamed tool transaction's ticks grow ONE edit and ONE history row
/// carrying its ref; while it is open a plain edit is refused `toolTransaction.open` and time travel `timeTravel.busy`; an
/// empty commit closes it with every op stamped; an aborted one leaves no edit, no row and the revision of before; streams
/// naming no transaction and aborts carrying mutations break the shape rule.
#[semio_framework_async_macros::async_test]
async fn a_streamed_tool_transaction_is_one_row_and_its_abort_leaves_none() {
    type ToyEmit = Emit<TestMutation, TestConfigMutation, NoDraftMutation>;
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let edits = app.store.envelope().vcs.edits.len();
    let streamed = protocol::TransactionRef { id: "tx-00000000000000aa".into(), tool: format!("{}#import", ToyHistoryApp::APP_ID) };
    for value in [2, 3, 4] {
        app.dispatch_emit("select", ToyEmit::stream_transaction(streamed.clone(), vec![SetCount { value }.into()]), &meta(&fixture)).await.expect("a tick streams");
    }
    assert_eq!((app.store.envelope().vcs.edits.len(), app.store.open_transaction().map(|open| open.transaction.clone())), (edits + 1, Some(streamed.clone())), "the ticks grow one open edit");
    let plain = app.dispatch_emit("select", ToyEmit::mutations(vec![SetCount { value: 9 }.into()]), &meta(&fixture)).await;
    assert_eq!(plain.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.open")), "a plain edit waits for the open transaction");
    let first = seeded_mutation(&app, 0);
    let busy = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))]).await;
    assert_eq!(rejected(&busy), Some("timeTravel.busy"), "time travel waits for the open transaction");
    app.dispatch_emit("select", ToyEmit::commit_transaction(streamed.clone(), Vec::new()), &meta(&fixture)).await.expect("an empty commit closes the edit");
    assert!(app.store.open_transaction().is_none());
    let tail = app.store.envelope().vcs.edits.last().expect("the committed edit");
    assert!(tail.forwards.len() == 3 && tail.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&streamed)), "one edit, every op stamped");
    let patch = app.history_patch(true).await.expect("history patch");
    let rows: Vec<_> = patch.upserts.iter().filter(|entry| entry.transaction.as_ref().is_some_and(|reference| reference.id == streamed.id)).collect();
    assert_eq!((rows.len(), rows.first().map(|row| row.mutations.len())), (1, Some(3)), "one row with one mutation row per op");
    let (edits, revision) = (app.store.envelope().vcs.edits.len(), app.store.content_revision_now());
    let aborted = protocol::TransactionRef { id: "tx-00000000000000bb".into(), tool: streamed.tool.clone() };
    for value in [5, 6] {
        app.dispatch_emit("select", ToyEmit::stream_transaction(aborted.clone(), vec![SetCount { value }.into()]), &meta(&fixture)).await.expect("a tick streams");
    }
    app.dispatch_emit("select", ToyEmit::abort_transaction(aborted.clone()), &meta(&fixture)).await.expect("the abort reverts the open edit");
    assert_eq!((app.store.envelope().vcs.edits.len(), app.store.content_revision_now(), app.store.open_transaction().is_none()), (edits, revision, true), "zero trace");
    let patch = app.history_patch(true).await.expect("history patch");
    let held: HashSet<&str> = app.store.envelope().vcs.edits.iter().map(|edit| edit.id.as_str()).collect();
    assert!(patch.upserts.iter().all(|entry| entry.transaction.as_ref().is_none_or(|reference| reference.id != aborted.id) && entry.edit_id.as_deref().is_none_or(|edit_id| held.contains(edit_id))), "no row of the aborted transaction remains");
    let unnamed = ToyEmit { transaction: None, transaction_phase: TransactionPhase::Stream, ..ToyEmit::mutations(vec![SetCount { value: 7 }.into()]) };
    assert_eq!(app.dispatch_emit("select", unnamed, &meta(&fixture)).await.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.shape")));
    let carrying = ToyEmit { artifact_mutations: vec![SetCount { value: 8 }.into()], ..ToyEmit::abort_transaction(aborted) };
    assert_eq!(app.dispatch_emit("select", carrying, &meta(&fixture)).await.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.shape")));
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
    let handles = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(semio_framework::ArgSchema::Object { fields: vec![angle] }), min_items: None, max_items: None },
        ..semio_framework::ActionArgDef::text("/handles", LocalizedLabel::native("Handles", "Griffe"))
    };
    let inputs = vec![
        semio_framework::ActionArgDef::object("/pivot", LocalizedLabel::native("Pivot", "Drehpunkt"), vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("X", "X"))]),
        semio_framework::ActionArgDef::vector("/offset", LocalizedLabel::native("Offset", "Versatz"), 2),
        handles,
    ];
    let payload = dsl(&serde_json::json!({ "pivot": { "x": 1.0 }, "offset": [0.0, 0.0], "handles": [{ "angle": 0.0 }, { "angle": 90.0 }] }));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/pivot/x").is_some_and(|(input, element)| input.id == "/x" && !element));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/offset/1").is_some_and(|(input, element)| input.id == "/offset" && element));
    assert!(
        time_travel::time_travel_input_at(&inputs, &payload, "/handles/1/angle").is_some_and(|(input, element)| input.id == "/angle" && !element && matches!(input.schema, semio_framework::ArgSchema::Number { .. })),
        "a field inside an array of objects is addressable"
    );
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/1").is_some_and(|(input, element)| !element && matches!(input.schema, semio_framework::ArgSchema::Object { .. })), "an item takes the array's item schema");
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/x/angle").is_none() && time_travel::time_travel_input_at(&inputs, &payload, "/missing").is_none());
    let rows = time_travel::time_travel_input_rows(&inputs, &payload);
    assert_eq!(
        rows.iter().map(|row| row.pointer.as_str()).collect::<Vec<_>>(),
        ["/pivot/x", "/offset", "/handles", "/handles/0", "/handles/0/angle", "/handles/1", "/handles/1/angle"],
        "objects flatten into field rows; a list is its own row, then every item's row and fields"
    );
    assert_eq!(
        (rows[2].list, rows[2].item, rows[3].list, rows[3].item, rows[4].item),
        (Some(time_travel::TimeTravelList { len: 2, addable: true, max: None }), None, None, Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: None }), None)
    );
    assert_eq!((rows[6].label.resolve(Terminology::Native, Locale::En), rows[6].label.resolve(Terminology::Native, Locale::De), rows[6].value.as_f64()), ("Handles 2 \u{b7} Angle", "Griffe 2 \u{b7} Winkel", Some(90.0)));
}

/// ✂️ LAW (design §6, gap N2): a list input of any item kind is reachable item by item — its own row adds an item until
/// `maxItems`, every item's row removes it while more than `minItems` remain — and the list edits of `historyEditInput`
/// insert before an index or after the last item (`-`) and remove by index; a new item starts from its schema default.
#[test]
fn list_inputs_add_and_remove_items_within_their_bounds() {
    let point = semio_framework::ArgSchema::Object { fields: vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("X", "X")).required(), semio_framework::ActionArgDef::text("/tag", LocalizedLabel::native("Tag", "Markierung"))] };
    let points = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(point.clone()), min_items: Some(1), max_items: Some(2) },
        ..semio_framework::ActionArgDef::text("/points", LocalizedLabel::native("Points", "Punkte"))
    };
    let weights = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(semio_framework::ArgSchema::number(Some(0.0), Some(1.0), None, false)), min_items: None, max_items: None },
        ..semio_framework::ActionArgDef::text("/weights", LocalizedLabel::native("Weights", "Gewichte"))
    };
    let inputs = vec![points, weights];
    let mut value = dsl(&serde_json::json!({ "points": [{ "x": 1.0 }, { "x": 2.0 }], "weights": [0.5] }));
    let rows = time_travel::time_travel_input_rows(&inputs, &value);
    assert_eq!(rows.iter().map(|row| row.pointer.as_str()).collect::<Vec<_>>(), ["/points", "/points/0", "/points/0/x", "/points/0/tag", "/points/1", "/points/1/x", "/points/1/tag", "/weights", "/weights/0"]);
    assert_eq!(rows[0].list, Some(time_travel::TimeTravelList { len: 2, addable: false, max: Some(2) }), "two points fill maxItems");
    assert_eq!((rows[1].item, rows[4].item), (Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: Some(1) }), Some(time_travel::TimeTravelListItem { index: 1, removable: true, min: Some(1) })));
    assert!(matches!(rows[8].input.schema, semio_framework::ArgSchema::Number { .. }) && rows[8].item == Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: None }), "a number item is its own row: {:?}", rows[8]);
    assert!(time_travel::time_travel_pointer_insert(&mut value, "/weights/-", DslValue::float(0.25)));
    assert!(time_travel::time_travel_pointer_insert(&mut value, "/weights/0", DslValue::float(0.75)));
    assert!(time_travel::time_travel_pointer_remove(&mut value, "/points/0"));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/weights").and_then(DslValue::as_array).map(|items| items.iter().filter_map(DslValue::as_f64).collect::<Vec<_>>()), Some(vec![0.75, 0.5, 0.25]));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/points/0/x").and_then(DslValue::as_f64), Some(2.0));
    assert!(
        !time_travel::time_travel_pointer_insert(&mut value, "/weights/9", DslValue::Null)
            && !time_travel::time_travel_pointer_remove(&mut value, "/weights/3")
            && !time_travel::time_travel_pointer_remove(&mut value, "/missing/0")
            && !time_travel::time_travel_pointer_insert(&mut value, "/points/0/x/-", DslValue::Null)
    );
    let rows = time_travel::time_travel_input_rows(&inputs, &value);
    assert_eq!(
        (rows[0].list, rows[1].item),
        (Some(time_travel::TimeTravelList { len: 1, addable: true, max: Some(2) }), Some(time_travel::TimeTravelListItem { index: 0, removable: false, min: Some(1) })),
        "one point left: addable, not removable below minItems"
    );
    let item = time_travel::time_travel_default_value(&point);
    assert_eq!((time_travel::time_travel_pointer_get(&item, "/x").and_then(DslValue::as_f64), time_travel::time_travel_pointer_get(&item, "/tag")), (Some(0.0), None), "a new point holds its required fields only");
    let mut floor = semio_framework::ArgSchema::number(Some(0.0), Some(4.0), Some(0.5), false);
    if let semio_framework::ArgSchema::Number { min_exclusive, .. } = &mut floor {
        *min_exclusive = true;
    }
    assert_eq!(time_travel::time_travel_default_value(&floor).as_f64(), Some(0.5), "an excluded floor is stepped over");
    let limits = [(8, true, Locale::En), (8, true, Locale::De), (1, false, Locale::En), (1, false, Locale::De), (1, true, Locale::En), (3, false, Locale::De)].map(|(limit, max, locale)| time_travel::time_travel_item_limit_text(limit, max, locale));
    assert_eq!(limits, ["Maximum 8 items", "Höchstens 8 Einträge", "Minimum 1 item", "Mindestens 1 Eintrag", "Maximum 1 item", "Mindestens 3 Einträge"].map(str::to_string), "a disabled list edit names its bound");
}

/// 🔀️ A union payload's inputs resolve against the variant the draft is in: the reader groups every variant's fields by
/// the variant's value (a glTF `create-node` declares `/value` once per variant), the editor shows the variant selector
/// and the active variant's fields only (a hidden restore diff never), switching the selector drops the members only
/// other variants declare, and layout `group`s of a payload that is no union hide nothing.
#[test]
fn union_inputs_resolve_against_the_active_variant() {
    let option = |value: &str, en: &str, de: &str| semio_framework::ActionArgOption { value: value.into(), label: LocalizedLabel::native(en, de) };
    let phase = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::String { options: vec![option("apply", "Apply", "Anwenden"), option("restore", "Restore", "Wiederherstellen")], option_source: None, min_len: None, max_len: None, pattern: None, format: None },
        required: true,
        ..semio_framework::ActionArgDef::text("/phase", LocalizedLabel::native("Mutation phase", "Mutationsphase"))
    };
    let parameters = semio_framework::ActionArgDef {
        group: Some("apply".into()),
        ..semio_framework::ActionArgDef::object("/value", LocalizedLabel::native("Parameters", "Parameter"), vec![semio_framework::ActionArgDef::index("/position", LocalizedLabel::native("Insert position", "Einfügeposition"))])
    };
    let weight = semio_framework::ActionArgDef { group: Some("apply".into()), ..semio_framework::ActionArgDef::number("/weight", LocalizedLabel::native("Weight", "Gewicht")) };
    let diff = semio_framework::ActionArgDef {
        group: Some("restore".into()),
        presentation: Some(semio_framework::ArgPresentation::Hidden),
        schema: semio_framework::ArgSchema::Any,
        ..semio_framework::ActionArgDef::text("/value", LocalizedLabel::native("Restore diff", "Wiederherstellungs-Diff"))
    };
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
    let laid_out = vec![
        semio_framework::ActionArgDef { group: Some("position".into()), ..semio_framework::ActionArgDef::number("/newX", LocalizedLabel::native("X", "X")) },
        semio_framework::ActionArgDef { group: Some("target".into()), ..semio_framework::ActionArgDef::text("/id", LocalizedLabel::native("Target region", "Zielregion")) },
    ];
    assert!(time_travel::time_travel_variant_selector(&laid_out).is_none());
    assert_eq!(time_travel::time_travel_input_rows(&laid_out, &dsl(&serde_json::json!({ "newX": 1.0, "id": "r" }))).len(), 2, "layout groups hide nothing");
}

/// 🗝️ LAW (design §20.11): an `optionSource` choice offers the keys of the object its template leads to in the previewed
/// document, each `{field}` segment taking the edited payload's member — an object's key, or the record of an array whose own
/// `field` equals it; a key names itself by its record's `label`/`name`, else the glossary, else the key. The row's own value stays an option when the document lacks it — the editor adds no enum, so the
/// fold refuses an unknown key — and a template the payload cannot fill offers nothing (a plain text field).
#[test]
fn sourced_options_are_the_keys_of_the_previewed_document() {
    let schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "additionalProperties": false,
        "required": ["mutation", "id", "channel"],
        "properties": {
            "mutation": { "const": "changeWidgetInput" },
            "id": { "type": "string", "minLength": 1, "x-semio-ui": { "label": { "en": "Widget", "de": "Widget" } } },
            "channel": { "type": "string", "minLength": 1, "x-semio-ui": { "widget": "select", "label": { "en": "Channel", "de": "Kanal" }, "optionSource": { "snapshot": "/hostSnapshot/widgets/{id}/params" } } }
        }
    }"#;
    let inputs = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document).expect("the sourced choice reads");
    let document = dsl(&serde_json::json!({ "hostSnapshot": { "widgets": [{ "kind": "source", "id": "w0" }, { "kind": "operator", "id": "w/1", "params": { "gain": { "label": { "en": "Gain", "de": "Verstärkung" } }, "zorp": 1.0 } }] } }));
    let resolved = |payload: serde_json::Value| {
        let payload = dsl(&payload);
        let mut rows = time_travel::time_travel_input_rows(&inputs, &payload);
        time_travel::time_travel_resolve_options(&mut rows, &payload, &document);
        rows.into_iter().find(|row| row.pointer == "/channel").expect("the channel row")
    };
    let keyed = dsl(&serde_json::json!({ "groups": { "a/b": { "x": 1 } }, "list": [{ "n": 7, "v": { "y": 2 } }] }));
    assert_eq!(time_travel::time_travel_option_target("/groups/{g}", &dsl(&serde_json::json!({ "g": "a/b" })), &keyed), Some(&dsl(&serde_json::json!({ "x": 1 }))), "a filled segment is an object key, never re-parsed as a pointer");
    assert_eq!(time_travel::time_travel_option_target("/list/{n}/v", &dsl(&serde_json::json!({ "n": 7 })), &keyed), Some(&dsl(&serde_json::json!({ "y": 2 }))), "on an array a filled segment selects the record whose own member matches");
    assert_eq!(time_travel::time_travel_option_target("/list/0/v", &DslValue::Null, &keyed), Some(&dsl(&serde_json::json!({ "y": 2 }))), "a literal segment indexes");
    assert_eq!(time_travel::time_travel_option_target("/list/{n}/v", &dsl(&serde_json::json!({ "n": 8 })), &keyed), None, "no record matches");
    let channel = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "id": "w/1", "channel": "gain" }));
    let semio_framework::ActionArgControl::Select { options } = channel.input.control() else { panic!("a sourced choice is a select: {:?}", channel.input.control()) };
    let named = options.iter().map(|option| (option.value.as_str(), option.label.resolve(Terminology::Native, Locale::En).to_string(), option.label.resolve(Terminology::Native, Locale::De).to_string())).collect::<Vec<_>>();
    assert_eq!(named, [("gain", "Gain".to_string(), "Verstärkung".to_string()), ("zorp", "zorp".to_string(), "zorp".to_string())], "keys in document order, named by their record");
    let stale = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "id": "w/1", "channel": "gone" }));
    assert!(
        matches!(&stale.input.schema, semio_framework::ArgSchema::String { options, option_source: Some(_), .. } if options.iter().map(|option| option.value.as_str()).collect::<Vec<_>>() == ["gain", "zorp", "gone"]),
        "the recorded key stays selectable for the fold to judge: {:?}",
        stale.input.schema
    );
    assert!(!format!("{:?}", stale.input.json_schema(Terminology::Native, Locale::En)).contains("enum"), "the editor never narrows the payload schema");
    let unfilled = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "channel": "gain" }));
    assert!(
        matches!(unfilled.input.control(), semio_framework::ActionArgControl::Select { ref options } if options.iter().map(|option| option.value.as_str()).collect::<Vec<_>>() == ["gain"]),
        "an unfillable template offers only the recorded key: {:?}",
        unfilled.input.control()
    );
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
    app.resolve_time_travel_snaps(&mut panel, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let resolved = &panel.editor.as_ref().expect("the draft editor").rows[0].input.schema;
    assert!(matches!(resolved, semio_framework::ArgSchema::Number { snaps, snap_source: None, .. } if *snaps == vec![0.0, 5.0, 10.0, 15.0, 20.0]), "the previewed document's count is the grid: {resolved:?}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🔗️ LAW (design §16.4): the open draft's `Reference` inputs are its draft references per selection domain — what every
/// render seam hands the app (`InteractionView::draft_references`) so it highlights what the edited mutation acts on: a
/// list and a single reference of one domain merge in draft order with each id once, a reference without a domain and a
/// number contribute nothing, and nothing is referenced before the session opens or once it exits.
#[semio_framework_async_macros::async_test]
async fn the_open_draft_references_its_reference_inputs_per_domain() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    assert!(app.time_travel.draft_references().is_empty(), "no session references nothing");
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let reference = |pointer: &str, domain: Option<&str>, many: bool| semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Reference { kinds: Vec::new(), domain: domain.map(str::to_string), granularity: None, many, min_items: None, max_items: None, id_type: semio_framework::ReferenceIdType::String },
        ..semio_framework::ActionArgDef::number(pointer, LocalizedLabel::native(pointer, pointer))
    };
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![
        reference("/targets", Some("items"), true),
        reference("/anchor", Some("items"), false),
        reference("/cells", Some("cells"), true),
        reference("/loose", None, true),
        semio_framework::ActionArgDef::number("/dx", LocalizedLabel::native("dx", "dx")),
    ];
    editor.value = dsl(&serde_json::json!({ "targets": ["a", "b", "a"], "anchor": "c", "cells": ["k"], "loose": ["z"], "dx": 4.0 }));
    let references = app.time_travel.draft_references();
    assert_eq!(references.get("items").map(Vec::as_slice), Some(["a".to_string(), "b".to_string(), "c".to_string()].as_slice()), "{references:?}");
    assert_eq!(references.get("cells").map(Vec::as_slice), Some(["k".to_string()].as_slice()), "{references:?}");
    assert_eq!(references.len(), 2, "only declared domains are referenced: {references:?}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    assert!(app.time_travel.draft_references().is_empty(), "an exited session references nothing");
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
    editor.inputs =
        vec![semio_framework::ActionArgDef { nullable: true, ..semio_framework::ActionArgDef::number("/scale", LocalizedLabel::native("Scale", "Maßstab")) }, semio_framework::ActionArgDef::number("/width", LocalizedLabel::native("Width", "Breite"))];
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

/// ⚖️ LAW (design §7, gap N2): while a history edit is open the body leads with the session band and the draft editor, then
/// the alternatives, the actions and the commands; without a session the actions lead. The editor's inputs are a tree
/// window over every input row, so a payload of any size stays reachable; the first rows are materialised the moment the
/// section shows.
#[semio_framework_async_macros::async_test]
async fn the_session_leads_the_history_body_and_the_editor_inputs_are_a_window_over_every_row() {
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
    assert_eq!(inputs["component"]["window"]["total"].as_u64(), Some(3), "the inputs are a window over every row: {inputs}");
    for pointer in ["dx", "dy", "dz"] {
        assert!(find_node(inputs, &format!("framework.history.editor.input.{pointer}.row")).is_some(), "the {pointer} row is materialised: {inputs}");
    }
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🧾️ LAW: unchanged Accept is refused through the app route, preserving the draft, preview and committed store.
#[semio_framework_async_macros::async_test]
async fn accepting_an_unchanged_input_retains_the_open_editor_and_preview() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let session = app.time_travel.session().clone();
    let pending = session.pending.as_ref().expect("the open draft");
    assert!(session.unchanged(pending), "opening preserves the draft baseline: {pending:?}");
    assert!(!app.time_travel.panel().expect("native history panel").editor.expect("native editor availability").changed, "unchanged availability comes from the authoritative draft baseline");
    let input = app.time_travel.editor().expect("the draft editor").value.clone();
    let body = render_body(&mut app).await;
    let generation = app.store.generation();
    for (locale, reason) in [(Locale::En, "Change an input before accepting"), (Locale::De, "Vor dem Übernehmen eine Eingabe ändern")] {
        let history = render_history(&mut app, locale).await;
        let accept = find_node(&history, "framework.history.editor.accept").expect("the Accept control");
        let row = find_node(&history, "framework.history.editor.accept.row").expect("the Accept row");
        assert_eq!(accept["disabled"], Value::Bool(true), "{locale:?}: {accept}");
        assert!(row.to_string().contains(reason), "{locale:?}: {row}");
        assert_ne!(find_node(&history, "framework.history.editor.discard").expect("the Discard control")["disabled"], Value::Bool(true));
    }
    let refused = verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    assert_eq!(rejected(&refused), Some(text(&fixture["refusals"]["unchanged"])));
    assert_eq!(app.time_travel.session(), &session);
    assert_eq!(app.time_travel.editor().expect("the draft editor stays open").value, input);
    assert_eq!(render_body(&mut app).await, body);
    assert_eq!(app.store.generation(), generation);
    for (value, changed) in [(7, true), (1, false)] {
        run_step(&mut app, &fixture, &serde_json::json!({ "input": { "path": "/value", "value": value } })).await;
        let history = render_history(&mut app, Locale::En).await;
        let accept = find_node(&history, "framework.history.editor.accept").expect("the Accept control");
        assert_eq!(accept["disabled"] == Value::Bool(true), !changed, "{accept}");
    }
    let discarded = verb(&mut app, &fixture, "historyEditDiscard", Vec::new()).await;
    assert_eq!(rejected(&discarded), None);
    assert!(app.time_travel.status().is_none());
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
    app.store.dispatch(ArtifactCommand::Apply { mutations: long, transaction: None }).await.expect("a long edit applies");
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
    let under = ActionMeta { view_state: Some(ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }), ..artifact_app_laws::meta(&actor) };
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
    remote.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }).await.expect("a remote edit");
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
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into())).await;
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
/// typed `history.ledger-not-replayable` naming only its blocking messages; any other refusal keeps its own fault.
#[test]
fn a_blocking_ledger_replay_crosses_the_guest_boundary_as_ledger_not_replayable() {
    let fault = replay_envelopes_fault(store::VcsError::Rejected {
        policy: protocol::MergePolicy::Normal,
        messages: vec![protocol::MutationMessage::error("mutation.target-missing", "gone"), protocol::MutationMessage::warning("mutation.clamped", "clamped")],
    });
    assert_eq!(fault.code, FaultCode::new("history.ledger-not-replayable"));
    assert!(fault.message.contains("mutation.target-missing") && !fault.message.contains("mutation.clamped"), "{}", fault.message);
    assert_ne!(replay_envelopes_fault(store::VcsError::Deserialize("malformed".into())).code, FaultCode::new("history.ledger-not-replayable"));
}

/// 🧲️ A `Config`/`Snapshot` snap source becomes numbers on the wire: the grid multiples inside the travel range when at
/// most one fixed list of them fits, else the grid spacing as the step; a colour input takes `#rrggbb` text.
#[test]
fn snap_sources_resolve_to_numbers_and_colours_take_hex_text() {
    let grid = |min: f64, max: f64| semio_framework::ArgSchema::Number {
        min: Some(min),
        min_exclusive: false,
        max: Some(max),
        max_exclusive: false,
        step: None,
        integer: false,
        unit: None,
        snaps: Vec::new(),
        snap_source: Some(semio_framework::SnapSource::Config { key: "gridFactor".into() }),
        soft_min: None,
        soft_max: None,
        precision: None,
        display_unit: None,
        display_factor: None,
        scale: None,
    };
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
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.into())).await;
    let (backbone, probe) = MemoryBackbone::pair(uri, uri).await;
    app.attach_backbone(store::Backbones::Memory(backbone)).await.expect("attach the replica backbone");
    assert_eq!(app.store.local_actor_id(), &protocol::ActorId(actor.to_string()));
    if seed {
        for op in fixture["seed"].as_array().expect("seed") {
            app.store.dispatch(ArtifactCommand::Apply { mutations: vec![seed_op(op)], transaction: None }).await.expect("seed edit applies");
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

/// 🔀️ [`relay`], then drives `to`'s turns until the relayed change is adopted — its replay may span turns (gap N17).
async fn relay_adopted(from: &mut MemoryBackbone, into: &mut MemoryBackbone, to: &mut ToyApp) {
    relay(from, into, to).await;
    pump_until(to, "the relayed change is adopted", |app| app.store.reprojection_progress().is_none()).await;
    to.refresh_cache().await.expect("the command log backfills the adopted change");
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
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
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
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone())).await;
    assert_eq!(reloaded.store.local_actor_id(), &protocol::ActorId(actor.clone()));
    artifact_app_laws::load_document_text(&mut reloaded, &local.document_text().await.expect("document text")).await.expect("text reload");
    reloaded.refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into())).await;
    assert_eq!(repacked.store.local_actor_id(), &protocol::ActorId("reader".into()));
    artifact_app_laws::load_document(&mut repacked, &local.document_pack().await.expect("document pack")).await.expect("pack reload");
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
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote).await;
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place").await;
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote).await;
    let edits = local.store.envelope().vcs.edits.len();

    history_verb(&mut local, &actor, "undo", None).await;
    assert_eq!(head(&local), (5, "a".to_string()), "the undo restores the original input");
    let rows = history_edit_rows(&mut local).await;
    assert_eq!(english(&rows), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    assert!(rows[0].applied && !rows[0].revertible && !rows[1].applied && !rows[1].revertible, "the undone history edit reads undone");
    assert!(!seeded_mutation_row(&mut local, 1).await.superseded, "an input restored to its original is not superseded");
    assert_eq!(local.store.envelope().vcs.edits.len(), edits, "the undo is a history transition, never a document edit");
    assert!(local.history_patch(true).await.expect("patch").can_redo, "the undone history edit can be redone");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), (5, "a".to_string()), "the remote replica folds the restoring supersede");
    assert_eq!(english(&history_edit_rows(&mut remote).await), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);

    history_verb(&mut local, &actor, "redo", None).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the redo re-authors the edited input");
    let rows = history_edit_rows(&mut local).await;
    assert_eq!(english(&rows), ["History edit redone — overwrite: 1 mutation", "History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    assert!(rows[0].applied && !rows[1].applied && rows[2].applied && rows[2].revertible, "the redone history edit is in effect again");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), (5, "b".to_string()));

    local.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 7 }.into()], transaction: None }).await.expect("a newer document edit");
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
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), (7, "a".to_string()), "the replicas converge");

    history_verb(&mut local, &actor, "redo", None).await;
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote).await;
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
    eprintln!("[DEBUG] Alternative history undo authoring pending={}", app.time_travel.authoring_pending());
    pump_until(&mut app, "the alternative undo lands", |app| !app.time_travel.authoring_pending()).await;
    assert_eq!(head(&app), (5, "a".to_string()), "the undo restores the original within the alternative");
    let rows = history_edit_rows(&mut app).await;
    assert_eq!(english(&rows), ["History edit undone — alternative Variant: 1 mutation", "History edited — alternative Variant: 1 mutation"]);
    assert_eq!(rows[1].label.resolve(Terminology::Native, Locale::De), "Verlauf bearbeitet — Alternative Variant: 1 Mutation");
    assert!(
        app.store.supersessions().values().all(|supersession| supersession.scope.as_deref() == Some(variant.as_str()) && Some(supersession.transition_id.as_str()) == rows[0].transition_id.as_deref()),
        "the restoring supersede is scoped to the alternative"
    );
    assert_eq!(app.store.envelope().active_alternative_id.as_deref(), Some(variant.as_str()), "the alternative stays active");
    history_verb(&mut app, &actor, "redo", None).await;
    pump_until(&mut app, "the alternative redo lands", |app| !app.time_travel.authoring_pending()).await;
    assert_eq!(head(&app), (5, "b".to_string()));
    drop(probe);
    close(&mut app);
}

//#region 🧭️AcceptanceLaws
/// 🖱️ Selects `children` in the toy's `test.slot` domain at the `child` granularity, as an interaction verb would.
async fn select_children(app: &mut ToyApp, children: &[String]) {
    let selection = protocol::DomainSelection { granularity: "child".into(), ids: children.to_vec(), anchor_id: None };
    let state = protocol::InteractionState { selection: BTreeMap::from([("test.slot".to_string(), selection)]), ..protocol::InteractionState::default() };
    app.interaction_store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![InteractionConfigMutation::set_state(state)], lane: HistoryLane::Interaction, transaction: None }).await.expect("the selection lands");
}

/// 🪧️ LAW (gap N3): a reference chip no app hook names reads the framework's generic default — the entity's own name where
/// the document gives one (a label, name, title or text; data or an `{en, de}` pair; a blank one names nothing), else its
/// kind word from the framework input-label glossary (or the kind spelled out) and a short id, in every locale; in a German
/// shell the toy's anonymous child reads "Kindelement anon-1".
#[semio_framework_async_macros::async_test]
async fn reference_chips_fall_back_to_the_document_name_then_the_kind_and_a_short_id() {
    let both = |label: &LocalizedLabel| (label.resolve(Terminology::Native, Locale::En).to_string(), label.resolve(Terminology::Native, Locale::De).to_string());
    let pair = |en: &str, de: &str| (en.to_string(), de.to_string());
    let document = dsl(&serde_json::json!({ "nodes": [{ "id": "a", "label": "Alpha" }, { "id": "b", "name": { "en": "Bee", "de": "Biene" } }, { "id": "c", "text": "  " }], "edges": [{ "id": "c", "title": "Gamma" }] }));
    let names = time_travel::time_travel_entity_names(&document, &["a", "b", "c", "z"].into_iter().collect());
    assert_eq!(names.iter().map(|(id, label)| (id.as_str(), both(label))).collect::<Vec<_>>(), vec![("a", pair("Alpha", "Alpha")), ("b", pair("Bee", "Biene")), ("c", pair("Gamma", "Gamma"))], "z is not in the document");
    let fallback = |kinds: &[&str], id: &str| both(&time_travel::time_travel_reference_fallback_label(&kinds.iter().map(|kind| kind.to_string()).collect::<Vec<_>>(), id));
    assert_eq!(fallback(&["s.test.child"], "anon-1!s.test.child@native/*"), pair("Child anon-1", "Kindelement anon-1"));
    assert_eq!(fallback(&["node", "edge"], "n1"), pair("Node n1", "Knoten n1"), "the first declared kind names it");
    assert_eq!(fallback(&["vortex.targetRegion"], "0123456789abcdefXYZ"), pair("Target region 0123456789ab\u{2026}", "Target region 0123456789ab\u{2026}"), "an unknown kind is spelled out, a long id cut");
    assert_eq!(fallback(&[], "x"), pair("x", "x"));
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for step in [serde_json::json!({ "begin": 2 }), serde_json::json!({ "input": { "path": "/children", "value": ["anon-1!s.test.child@native/*", "child-2!s.test.child@native/*"] } })] {
        run_step(&mut app, &fixture, &step).await;
    }
    let history = render_history(&mut app, Locale::De).await;
    let chip = |index: usize| find_node(&history, &format!("framework.history.editor.input.children.chip.{index}.row")).and_then(|chip| chip["component"]["label"].as_str()).unwrap_or_else(|| panic!("chip {index}: {history}")).to_string();
    assert_eq!((chip(0), chip(1)), ("Kindelement anon-1".to_string(), "Kind child-2".to_string()), "the generic default where the app names nothing, the app's own label elsewhere");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🎯️ "Use selection" reads the reference input's declared domain at its granularity: every selected id up to
/// `maxItems` for a many-reference, the first for a single one, typed by the input's id type (an id the type cannot
/// spell is skipped); an input without a domain is no selection target, and a missing, empty or other-granularity
/// selection selects nothing.
#[test]
fn selection_values_follow_the_reference_domain_granularity_count_and_id_type() {
    use semio_framework::{ArgSchema, ReferenceIdType};
    use time_travel::{time_travel_selection_value, TimeTravelActionRefusal};
    let reference = |domain: Option<&str>, many: bool, max_items: Option<u32>, id_type: ReferenceIdType| semio_framework::ActionArgDef {
        schema: ArgSchema::Reference { kinds: vec!["node".into()], domain: domain.map(str::to_string), granularity: Some("node".into()), many, min_items: None, max_items, id_type },
        ..semio_framework::ActionArgDef::number("/targets", LocalizedLabel::native("Targets", "Ziele"))
    };
    let selection = |granularity: &str, ids: &[&str]| protocol::DomainSelection { granularity: granularity.into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: None };
    let text = |id: &str| DslValue::String(id.into());
    let many = reference(Some("board"), true, Some(2), ReferenceIdType::String);
    assert_eq!(time_travel_selection_value(&many, Some(&selection("node", &["a", "b", "c"]))), Ok(DslValue::Array(vec![text("a"), text("b")])), "a many-reference takes the selection up to maxItems");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), true, None, ReferenceIdType::String), Some(&selection("node", &["a", "b", "c"]))), Ok(DslValue::Array(vec![text("a"), text("b"), text("c")])), "without maxItems every selected id");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), false, None, ReferenceIdType::String), Some(&selection("node", &["a", "b"]))), Ok(text("a")), "a single reference takes the first");
    assert_eq!(time_travel_selection_value(&reference(None, true, None, ReferenceIdType::String), Some(&selection("node", &["a"]))), Err(TimeTravelActionRefusal::UnknownInput), "an input without a domain is no selection target");
    assert_eq!(time_travel_selection_value(&semio_framework::ActionArgDef::number("/dx", LocalizedLabel::native("dx", "dx")), Some(&selection("node", &["a"]))), Err(TimeTravelActionRefusal::UnknownInput), "a number is no reference");
    assert_eq!(time_travel_selection_value(&many, None), Err(TimeTravelActionRefusal::NoSelection), "the domain selects nothing");
    assert_eq!(time_travel_selection_value(&many, Some(&selection("region", &["a"]))), Err(TimeTravelActionRefusal::NoSelection), "a selection at another granularity");
    assert_eq!(time_travel_selection_value(&many, Some(&selection("node", &[]))), Err(TimeTravelActionRefusal::NoSelection), "an empty selection");
    let integers = reference(Some("board"), true, None, ReferenceIdType::Integer);
    assert_eq!(time_travel_selection_value(&integers, Some(&selection("node", &["7", "x", "-3"]))), Ok(DslValue::Array(vec![DslValue::uint(7), DslValue::int(-3)])), "integer ids stage the integers they spell");
    assert_eq!(time_travel_selection_value(&integers, Some(&selection("node", &["x"]))), Err(TimeTravelActionRefusal::NoSelection), "no id the type can spell");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), false, None, ReferenceIdType::Integer), Some(&selection("node", &["x", "12"]))), Ok(DslValue::uint(12)), "a single integer reference skips text it cannot spell");
}

/// ⚖️ LAW (design §16.4, R14, R31): a blocked review is resolved by editing the failing mutation's targets — Next problem
/// opens the first blocking mutation from the review; "Use selection" refuses while nothing is selected, then drafts the
/// selected ids; the chips read the app's entity labels (German shell: German labels), every id is a chip row of the
/// reference's tree window (gap N2), a chip's remove binding removes its item; Accept replays and the review is ready.
#[semio_framework_async_macros::async_test]
async fn editing_targets_through_the_selection_resolves_a_blocked_review() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for step in [serde_json::json!({ "begin": 2 }), serde_json::json!({ "input": { "path": "/children", "value": ["not a uri"] } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "fatal" })] {
        run_step(&mut app, &fixture, &step).await;
    }
    let failing = seeded_mutation(&app, 2);
    let panel = app.time_travel.panel().expect("a blocked review");
    assert_eq!((panel.review, panel.next_problem.as_deref()), (Some(semio_framework_time_travel::TimeTravelReview::Blocked), Some(failing.as_str())), "Next problem names the first blocking mutation");
    let history = render_history(&mut app, Locale::En).await;
    let next = find_node(&history, "framework.history.timeTravel.nextProblem").unwrap_or_else(|| panic!("the Next problem button: {history}"));
    assert_eq!(next["bindings"][0]["action"]["name"].as_str(), Some("historyEditBegin"));
    let begun = app.handle_action("historyEditBegin", Some(&dsl(&next["bindings"][0]["args"])), &meta(&fixture)).await.expect("Next problem dispatches");
    assert_eq!(rejected(&begun), None, "{:?}", begun.output);
    assert_eq!(app.time_travel.editor().map(|editor| editor.target.0.as_str()), Some(failing.as_str()), "Next problem opens the failing mutation");
    let path = || vec![("path".to_string(), DslValue::String("/children".into()))];
    assert_eq!(rejected(&verb(&mut app, &fixture, "historyEditUseSelection", path()).await), Some("timeTravel.no-selection"), "nothing is selected yet");
    let children: Vec<String> = (1..=10).map(|index| format!("child-{index}!s.test.child@native/*")).collect();
    select_children(&mut app, &children).await;
    let drafted = verb(&mut app, &fixture, "historyEditUseSelection", path()).await;
    assert_eq!(rejected(&drafted), None, "{:?}", drafted.output);
    let drafted_children = |app: &ToyApp| app.time_travel.editor().and_then(|editor| editor.value.get("children").cloned());
    assert_eq!(drafted_children(&app), Some(DslValue::Array(children.iter().map(|child| DslValue::String(child.clone())).collect())), "the selection is the draft");
    let history = render_history(&mut app, Locale::De).await;
    let first = find_node(&history, "framework.history.editor.input.children.chip.0.row").unwrap_or_else(|| panic!("the first chip: {history}"));
    assert_eq!(first["component"]["label"].as_str(), Some("Kind child-1"), "a chip reads the entity label in the shell's locale: {first}");
    let references = find_node(&history, "framework.history.editor.input.children.row").unwrap_or_else(|| panic!("the reference row: {history}"));
    assert!(references["component"]["window"]["total"].as_u64().is_some_and(|total| total >= 11), "Use selection and one chip per id, all ten reachable: {references}");
    assert!(find_node(&history, "framework.history.editor.input.children.chip.9.row").is_some_and(|chip| chip.to_string().contains("Kind child-10")), "the tenth chip is a row too: {history}");
    let chip = find_node(&history, "framework.history.editor.input.children.chip.0").unwrap_or_else(|| panic!("the first chip's Remove: {history}"));
    assert_eq!(chip["bindings"][0]["args"]["edit"].as_str(), Some("remove"), "a chip removes its item by index: {chip}");
    let removed = app.handle_action("historyEditInput", Some(&dsl(&chip["bindings"][0]["args"])), &meta(&fixture)).await.expect("the chip removes");
    assert_eq!(rejected(&removed), None, "{:?}", removed.output);
    assert_eq!(drafted_children(&app), Some(DslValue::Array(children[1..].iter().map(|child| DslValue::String(child.clone())).collect())), "removing a chip drafts the list without it");
    for step in [serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step).await;
    }
    let status = app.time_travel.status().expect("a review");
    assert_eq!((status.review, status.blocking, status.worst), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, None), "edited targets resolve the review: {status:?}");
    assert!(seeded_mutation_row(&mut app, 2).await.worst.is_none(), "the edited mutation applies cleanly");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// ⚖️ LAW (design §16.5, R20): an upstream edit that turns a downstream mutation into a no-op reviews `ready` (a Warning
/// never blocks) with that mutation's Warning marked as introduced by the edit; finalizing keeps the Warning on its
/// history row — on the authoring replica, after a text reload and after a pack reload, in English and German.
#[semio_framework_async_macros::async_test]
async fn a_warning_an_edit_introduces_stays_visible_after_finalize_and_reload() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone())).await;
    assert_eq!(app.store.local_actor_id(), &protocol::ActorId(actor.clone()));
    for value in ["a", "b"] {
        app.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetLabel { value: value.into() }.into()], transaction: None }).await.expect("a label edit applies");
    }
    app.refresh_cache().await.expect("the command log backfills");
    assert!(seeded_mutation_row(&mut app, 1).await.worst.is_none(), "before the edit the downstream label changes the document");
    for step in [serde_json::json!({ "begin": 0 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step).await;
    }
    let status = app.time_travel.status().expect("a review");
    assert_eq!((status.review, status.blocking, status.worst), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, Some(semio_framework_diagnostic::Severity::Warning)), "a warning never blocks finalizing");
    let downstream = seeded_mutation_row(&mut app, 1).await;
    assert!(downstream.introduced && downstream.worst == Some(semio_framework_diagnostic::Severity::Warning) && downstream.messages.iter().any(|message| message.code == "mutation.no-op"), "the edit introduced the no-op: {downstream:?}");
    assert!(!seeded_mutation_row(&mut app, 0).await.introduced, "the edited mutation itself raises nothing new");
    let history = render_history(&mut app, Locale::En).await.to_string();
    assert!(history.contains("New since this edit") && history.contains("Warning: No change"), "{history}");
    for step in [serde_json::json!({ "finalize": null }), serde_json::json!({ "commit": { "choice": "overwrite" } })] {
        run_step(&mut app, &fixture, &step).await;
    }
    pump_until(&mut app, "the finalize retires", |app| !app.time_travel.has_pending_work()).await;
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone())).await;
    assert_eq!(reloaded.store.local_actor_id(), &protocol::ActorId(actor.clone()));
    artifact_app_laws::load_document_text(&mut reloaded, &app.document_text().await.expect("document text")).await.expect("text reload");
    reloaded.refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into())).await;
    assert_eq!(repacked.store.local_actor_id(), &protocol::ActorId("reader".into()));
    artifact_app_laws::load_document(&mut repacked, &app.document_pack().await.expect("document pack")).await.expect("pack reload");
    repacked.refresh_cache().await.expect("the repacked log backfills");
    for (who, app) in [("finalized", &mut app), ("text reload", &mut reloaded), ("pack reload", &mut repacked)] {
        let row = seeded_mutation_row(app, 1).await;
        assert!(row.worst == Some(semio_framework_diagnostic::Severity::Warning) && row.messages.iter().any(|message| message.code == "mutation.no-op") && !row.introduced, "{who}: the warning persists without a session: {row:?}");
        assert_eq!(head(app).1, "b", "{who}: the edited head");
        for (locale, line) in [(Locale::En, "Warning: No change"), (Locale::De, "Warnung: Keine Änderung")] {
            let history = render_history(app, locale).await.to_string();
            assert!(history.contains(line), "{who} {locale:?}: the row names the warning in words: {history}");
        }
    }
    for app in [&mut app, &mut reloaded, &mut repacked] {
        close(app);
    }
}

/// 🌿️ The history transitions `app` gained since it held `before` of them, decoded.
fn added_transitions(app: &ToyApp, before: usize) -> Vec<store::os_spr::HistoryTransition> {
    app.store.envelope().transitions[before..].iter().map(|envelope| store::os_spr::history_transition_from_envelope(envelope).expect("a transition decodes").expect("a history transition")).collect()
}

/// ⚖️ LAW (R15): from a clean review the user begins another mutation and accepts it, so the session holds both drafts;
/// one finalize commits exactly ONE unscoped `Supersede` naming both, and the head equals a fresh fold of the edited log.
#[semio_framework_async_macros::async_test]
async fn several_drafts_from_a_review_finalize_as_one_overwrite_supersede() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let before = app.store.envelope().transitions.len();
    commit_scenario(&mut app, &fixture, "several-drafts-accumulated-from-a-review-finalize-once").await;
    let added = added_transitions(&app, before);
    let [store::os_spr::HistoryTransition::Supersede(supersede)] = added.as_slice() else { panic!("an overwrite is one Supersede: {added:?}") };
    let targets: BTreeSet<String> = supersede.inputs.iter().map(|input| input.target.0.clone()).collect();
    assert_eq!((supersede.scope.as_deref(), targets), (None, [0, 1].map(|index| seeded_mutation(&app, index)).into_iter().collect()), "one unscoped supersede names both drafts");
    let mut fresh = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), app.store.local_actor_id().clone()).await;
    for (index, op) in fixture["seed"].as_array().expect("seed").iter().enumerate() {
        let op = match index {
            0 => SetCount { value: 7 }.into(),
            1 => SetLabel { value: "b".into() }.into(),
            _ => seed_op(op),
        };
        fresh.store.dispatch(ArtifactCommand::Apply { mutations: vec![op], transaction: None }).await.expect("fresh edit");
    }
    assert_eq!(app.store.snapshot().expect("edited head"), fresh.store.snapshot().expect("fresh head"));
    close(&mut app);
    close(&mut fresh);
}

/// ⚖️ LAW (design §2, §4): a new alternative is one `Branch` followed by one `Supersede` scoped to that alternative
/// (after the checkpoint of any pending edit), and the new alternative is current.
#[semio_framework_async_macros::async_test]
async fn a_new_alternative_is_one_branch_then_one_scoped_supersede() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let before = app.store.envelope().transitions.len();
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original").await;
    let added = added_transitions(&app, before);
    let alternative = app.store.envelope().active_alternative_id.clone().expect("the new alternative is current");
    let (checkpoints, tail) = added.split_at(added.len().saturating_sub(2));
    assert!(checkpoints.iter().all(|transition| matches!(transition, store::os_spr::HistoryTransition::Commit(..))), "only a checkpoint of pending edits precedes the branch: {added:?}");
    assert!(
        matches!(tail, [store::os_spr::HistoryTransition::Branch { .. }, store::os_spr::HistoryTransition::Supersede(supersede)] if supersede.scope.as_deref() == Some(alternative.as_str())),
        "Branch, then a Supersede scoped to the new alternative: {added:?}"
    );
    close(&mut app);
}
/// 🧱️ `replica` with one long edit after the seed, so a replay downstream of the seed exceeds one turn's budget.
async fn long_replica(fixture: &Value, uri: &str, actor: &str) -> (ToyApp, MemoryBackbone) {
    let (mut app, probe) = replica(fixture, uri, actor, true).await;
    let long: Vec<TestMutation> = (0..3_000).map(|value| SetCount { value }.into()).collect();
    app.store.dispatch(ArtifactCommand::Apply { mutations: long, transaction: None }).await.expect("a long edit applies");
    app.refresh_cache().await.expect("the long edit is logged");
    (app, probe)
}

/// ⚖️ LAW (design §16.6, G9): a remote history change whose replay exceeds one turn's budget waits while the replica shows
/// the history before it, its progress rides the history wire (`reprojection`) and the body's reprojection section; Cancel replay
/// pauses it (driver turns leave it alone), Replay again resumes it, and the adoption equals the author's head. The remote
/// store replays one operation per turn, so the seed's downstream suffix already spans several turns.
#[semio_framework_async_macros::async_test]
async fn a_long_remote_history_change_replays_over_turns_and_pauses_on_cancel() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "remote-replay-local", &actor, true).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "remote-replay-remote", "remote", false).await;
    remote.store.defer_remote_replays(Some(store::ReplayTurnBudget::operations(1)));
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), head(&local), "the remote replica holds the seed");
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place").await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    let status = remote.reprojection_status().expect("the remote history change waits for its replay");
    assert!(status.total > 0 && status.done < status.total && !status.paused && status.kind == semio_framework::kernel::HistoryReprojectionKind::Remote && status.fault.is_none(), "{status:?}");
    assert_eq!(head(&remote).1, "a", "the replica shows the history before the change");
    assert_eq!(remote.history_patch(true).await.expect("history patch").reprojection.as_ref().map(|remote| remote.total), Some(status.total), "the wire carries the remote replay");
    let body = render_history(&mut remote, Locale::En).await;
    assert!(find_node(&body, "framework.history.reprojection.status").is_some_and(|row| row.to_string().contains("Replaying a remote history change")), "{body}");
    let cancelled = verb(&mut remote, &fixture, "historyEditCancelReplay", Vec::new()).await;
    assert_eq!(rejected(&cancelled), None, "{:?}", cancelled.output);
    assert!(remote.time_travel.reprojection_paused(), "the remote replay is paused");
    for _ in 0..4 {
        remote.advance_typed_operation_publication().await.expect("driver turn");
        while remote.take_typed_operation_ui_progress().is_some() {}
    }
    assert!(remote.time_travel.reprojection_paused() && !remote.time_travel_has_pending_work(), "once its patch shipped, a paused remote replay is no driver work");
    assert_eq!(head(&remote).1, "a", "paused, the change is not adopted");
    let body = render_history(&mut remote, Locale::De).await;
    assert!(find_node(&body, "framework.history.reprojection.rerun").is_some() && body.to_string().contains("pausiert"), "{body}");
    let resumed = verb(&mut remote, &fixture, "historyEditRerun", Vec::new()).await;
    assert_eq!(rejected(&resumed), None, "{:?}", resumed.output);
    assert_eq!(rejected(&verb(&mut remote, &fixture, "historyEditRerun", Vec::new()).await), Some("timeTravel.illegal"), "nothing is paused any more");
    pump_until(&mut remote, "the remote change is adopted", |app| app.store.reprojection_progress().is_none()).await;
    assert_eq!(head(&remote), head(&local), "the adoption equals the author's head");
    assert!(remote.reprojection_status().is_none(), "nothing waits any more");
    drop((local_probe, remote_probe));
    close(&mut local);
    close(&mut remote);
}

/// ⚖️ LAW (design §16.6, G9): the undo of a finalize over a long downstream history is authored resumably — the verb answers
/// at once, history editing is busy while its replay runs, and the driver commits it: the original input is back and the
/// undo row reads like every other.
#[semio_framework_async_macros::async_test]
async fn the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = long_replica(&fixture, "long-undo", &actor).await;
    commit_scenario(&mut app, &fixture, "overwrite-supersedes-in-place").await;
    assert_eq!(head(&app).1, "b");
    history_verb(&mut app, &actor, "undo", None).await;
    if app.time_travel.authoring_pending() {
        let first = seeded_mutation(&app, 0);
        let busy = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))]).await;
        assert_eq!(rejected(&busy), Some("timeTravel.busy"), "history editing waits for the authoring");
    }
    pump_until(&mut app, "the undo lands", |app| !app.time_travel.authoring_pending()).await;
    app.refresh_cache().await.expect("history refresh");
    assert_eq!(head(&app).1, "a", "the undo restores the original input");
    assert_eq!(english(&history_edit_rows(&mut app).await), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    drop(probe);
    close(&mut app);
}
/// 🔁️ A history verb as `actor`, admitted and settled, answering the fault code of a refusal (`None`: applied).
async fn history_verb_refusal(app: &mut ToyApp, actor: &str, action: &str) -> Option<String> {
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let settled = match app.handle_action(action, None, &meta).await {
        Ok(admitted) => crate::app::settle_framework_reserved_admission(app, admitted).await,
        Err(fault) => Err(fault),
    };
    settled.err().map(|fault| fault.code.0)
}

/// 🧮️ What a history step may change: the head, the content revision, the edits, the transitions and the history rows.
async fn history_trace(app: &mut ToyApp) -> ((i32, String), [u8; 32], usize, usize, usize) {
    let rows = app.history_patch(true).await.expect("history patch").upserts.len();
    (head(app), app.store.content_revision_now(), app.store.envelope().vcs.edits.len(), app.store.envelope().transitions.len(), rows)
}

/// ✏️ Publishes a foreign downstream edit per value and drains displaced owners under bounded pressure.
async fn apply_other_edits(app: &mut ToyApp, count: i32) {
    for value in 0..count {
        publish_as(app, "downstream", SetCount { value }.into());
        for _ in 0..4_096 {
            if !app.store.maintenance_retirements_under_pressure() || !matches!(app.store.maintenance_retirements_step(64, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).expect("the store retires what the edits displaced"), store::SnapshotRetirementStep::Pending { .. }) {
                break;
            }
        }
    }
}

/// 🪶️ Publishes one edit with an explicit author while preserving the opened instance actor.
fn publish_as(app: &mut ToyApp, actor: &str, mutation: TestMutation) {
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 };
    let mut publication = app
        .store
        .begin_apply_batch(semio_framework_job::OperationId(1), app.store.generation_now(), app.store.content_revision_now(), actor.into(), vec![mutation], store::HistoryLane::Document, app.artifact_one_item_factory.as_ref(), None)
        .unwrap_or_else(|rejected| panic!("the edit is admitted: {}", rejected.into_owners().0));
    let published = (0..65_536).any(|_| matches!(app.store.advance_apply_batch(&mut publication, grant).expect("the edit publishes"), store::ArtifactStoreOneItemAdvance::Published(_)));
    assert!(published && publication.acknowledge(), "the edit is published and acknowledged");
    publication.begin_close();
    let closed = (0..4_096).any(|_| publication.close_step(grant).expect("the publication closes") == store::SnapshotRetirementStep::Complete);
    assert!(closed && publication.terminal_is_empty(), "the publication retires every owner");
}

/// ⚰️ One edit of `author` ([`publish_as`]) under `count` one-operation edits of the fixture actor
/// ([`apply_other_edits`]): the history an interior undo of `author` replays through. Proves what it built: exactly one
/// history row is `author`'s and it is not the newest.
async fn bury_edit_of(app: &mut ToyApp, author: &str, count: i32) {
    assert_eq!(app.store.local_actor_id(), &protocol::ActorId(author.into()), "the buried edit belongs to the opened actor");
    publish_as(app, author, SetLabel { value: "buried".into() }.into());
    apply_other_edits(app, count).await;
    app.refresh_cache().await.expect("backfill");
    let authors: Vec<Option<String>> = app.history_patch(true).await.expect("history patch").upserts.into_iter().map(|row| row.author).collect();
    let own = authors.iter().filter(|row| row.as_deref() == Some(author)).count();
    assert!(own == 1 && authors.first().and_then(|row| row.as_deref()) != Some(author), "one interior row is {author}'s: {own} of {} rows, newest {:?}", authors.len(), authors.first());
}

/// ⚖️ LAW (gap N17): an interior undo over a long downstream history — the author's own edit under 600 edits of another
/// author — is a local history step the runtime replays over driver turns:
/// - the undo answers at once, the head, the edits and the history rows unchanged;
/// - the wire carries `reprojection.kind = step`, the body reads "Replaying history" / "Verlauf wird neu angewendet";
/// - each turn replays at most `TIME_TRAVEL_REPLAY_OPERATIONS`;
/// - meanwhile undo and redo answer `history.replaying` and `historyEditBegin` answers `timeTravel.busy`;
/// - the adoption equals the undo of a store that replays inside the dispatch, and records the undo row.
/// Cancel replay instead drops the step with zero trace: head, revision, edits, transitions and history rows as before.
#[semio_framework_async_macros::async_test]
async fn an_interior_undo_over_a_long_history_replays_over_turns_and_cancel_leaves_zero_trace() {
    let fixture = fixture();
    let actor = "author".to_string();
    let mut undeferred = seeded_app_as(&fixture, &actor).await;
    undeferred.store.defer_local_replays(None);
    bury_edit_of(&mut undeferred, &actor, 600).await;
    history_verb(&mut undeferred, &actor, "undo", None).await;
    let undone = undeferred.store.snapshot().expect("the undeferred undo");
    for cancel in [true, false] {
        let mut app = seeded_app_as(&fixture, &actor).await;
        bury_edit_of(&mut app, &actor, 600).await;
        let before = history_trace(&mut app).await;
        history_verb(&mut app, &actor, "undo", None).await;
        assert!(app.store.local_step_pending(), "the interior undo waits for its replay");
        assert_eq!(history_trace(&mut app).await, before, "nothing of the step shows before its adoption");
        let status = app.history_patch(true).await.expect("history patch").reprojection.expect("the wire carries the waiting step");
        assert!(status.kind == semio_framework::kernel::HistoryReprojectionKind::Step && status.total > 0 && !status.paused && status.fault.is_none(), "{status:?}");
        for (locale, line) in [(Locale::En, "Replaying history"), (Locale::De, "Verlauf wird neu angewendet")] {
            let body = render_history(&mut app, locale).await;
            assert!(find_node(&body, "framework.history.reprojection.status").is_some_and(|row| row.to_string().contains(line)), "{locale:?}: {body}");
        }
        assert_eq!(history_verb_refusal(&mut app, &actor, "undo").await.as_deref(), Some("history.replaying"), "a further history step waits");
        assert_eq!(history_verb_refusal(&mut app, &actor, "redo").await.as_deref(), Some("history.replaying"));
        let first = seeded_mutation(&app, 0);
        assert_eq!(rejected(&verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))]).await), Some("timeTravel.busy"), "history editing waits for the step");
        if cancel {
            let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new()).await;
            assert_eq!(rejected(&cancelled), None, "{:?}", cancelled.output);
            assert!(!app.store.local_step_pending() && app.reprojection_status().is_none(), "the step is gone");
            for _ in 0..4 {
                app.advance_typed_operation_publication().await.expect("driver turn");
            }
            assert_eq!(history_trace(&mut app).await, before, "a cancelled step leaves zero trace");
        } else {
            let mut turns = 0;
            while app.store.local_step_pending() {
                let done = app.store.reprojection_progress().map_or(0, |progress| progress.done);
                app.advance_typed_operation_publication().await.expect("driver turn");
                while app.take_typed_operation_ui_progress().is_some() {}
                let after = app.store.reprojection_progress().map_or(u32::MAX, |progress| progress.done);
                assert!(after == u32::MAX || after.saturating_sub(done) as usize <= time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, "one turn replays at most one budget: {done} → {after}");
                turns += 1;
                assert!(turns < 64, "the step is adopted");
            }
            assert!(turns >= 2, "a 600-operation downstream spans several turns ({turns})");
            assert_eq!(app.store.snapshot().expect("the adopted undo"), undone, "the adoption equals the undeferred undo");
            let (_, _, edits, _, rows) = history_trace(&mut app).await;
            assert_eq!((edits, rows), (before.2, before.4 + 1), "the adoption records the undo row");
        }
        close(&mut app);
    }
    close(&mut undeferred);
}

/// ⚖️ LAW (decision §4 of `📓️api-stepped-document-load.md`): a pure command's document lane hydrates the HEAD snapshot of
/// a 240-edit document without its history (`PluginApp::hydrate_pure_head` takes the head pack alone, so no history can
/// ride it) — the store holds no edit, the head is the source head, nothing folds; in that head-only mode undo, a history
/// edit and the history query refuse `pure.history-unavailable`, and the whole-document archive load of the history (the
/// only load path) lifts it over real polls, landing the source head and all 240 edits.
#[semio_framework_async_macros::async_test]
async fn a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut source = seeded_app_as(&fixture, &actor).await;
    for value in 0..240 {
        source.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], transaction: None }).await.expect("a source edit");
    }
    let head = source.store.snapshot().expect("the source head");
    let files = source.document_pack().await.expect("the source pair");
    let mut pure = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into())).await;
    let code = |fault: Fault| fault.code.0;
    PluginApp::hydrate_pure_head(&mut pure, &head.encode_pack()).await.expect("the head-only lane hydrates");
    assert_eq!(pure.store.snapshot().expect("the pure head"), head, "the pure document is the source head");
    assert!(pure.store.envelope().vcs.edits.is_empty() && pure.time_travel.history_unavailable(), "no history was folded or kept");
    assert_eq!(history_verb_refusal(&mut pure, &actor, "undo").await.as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE));
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(&actor) };
    let begin = pure.handle_action("historyEditBegin", Some(&DslValue::object([("mutationId".to_string(), DslValue::String("any".into()))])), &meta).await;
    assert_eq!(begin.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "a history edit needs the history");
    assert_eq!(pure.history_snapshot().await.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "so does the history query");
    let envelope = pure.store.envelope();
    let dialect: ArtifactDialect = <ToyHistoryApp as ArtifactApp>::DIALECT.into();
    let parent_spr = store::stamp_document_spr_identity(&files.spr, &envelope.id, <ToyHistoryApp as ArtifactApp>::DOCUMENT_SCHEMA, &dialect, envelope.owner.as_ref()).await.expect("the load identity stamp");
    let operation = 0x51;
    PluginApp::begin_document_archive_load(&mut pure, operation, protocol::DocumentArchivePack { parent_pack: files.pack.clone(), parent_spr, members: Vec::new() }).expect("the archive load is admitted");
    let mut polls = 0;
    let status = loop {
        let status = PluginApp::poll_document_archive_load(&mut pure, operation).await.expect("the archive load status");
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            break status;
        }
        polls += 1;
        assert!(polls < 1_000_000, "the archive load reaches a terminal state");
    };
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "the archive load lands: {}", if status.fault.is_empty() { String::new() } else { semio_framework_diagnostic::decode_fault_bytes(&status.fault).describe() });
    PluginApp::acknowledge_document_archive_load(&mut pure, operation).expect("the terminal load is released");
    assert!(!pure.time_travel.history_unavailable() && pure.history_snapshot().await.is_ok(), "the archive load with history lifts head-only mode");
    assert_eq!((pure.store.snapshot().expect("the loaded head"), pure.store.envelope().vcs.edits.len()), (head, source.store.envelope().vcs.edits.len()), "the archive load lands the source head and its whole history (seed + 240 edits)");
    close(&mut pure);
    close(&mut source);
}

//#endregion 🧭️AcceptanceLaws

//#region 🗝️EditorKeys
/// 🗝️ The shared band corpus whose `editorKeys` every shell's focus resolution reads (React `timeTravelFocusElementV1`).
const TIME_TRAVEL_BAND_CORPUS_JSON: &str = include_str!("../../../📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json");

/// ⚖️ LAW (design §16, G13): the draft editor's node keys are exactly the shared corpus's — the History panel tab the body
/// mounts in, Accept inside its row, and per input pointer (object fields flattened) one row holding its control — so a shell's
/// focus lands on the first input, else on Accept, whichever shell renders the body.
#[semio_framework_async_macros::async_test]
async fn the_draft_editor_keys_are_the_shared_corpus_keys_every_shell_focuses() {
    let corpus: Value = serde_json::from_str(TIME_TRAVEL_BAND_CORPUS_JSON).expect("band corpus parses");
    let keys = &corpus["editorKeys"];
    assert_eq!(text(&keys["panel"]), ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID, "the body mounts in the History panel tab");
    let rows = keys["inputs"].as_array().expect("editor input keys");
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = vec![
        semio_framework::ActionArgDef::number("/dx", LocalizedLabel::native("dx", "dx")),
        semio_framework::ActionArgDef::object("/pivot", LocalizedLabel::native("Pivot", "Drehpunkt"), vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("x", "x"))]),
    ];
    editor.value = dsl(&serde_json::json!({ "dx": 1.0, "pivot": { "x": 2.0 } }));
    let history = render_history(&mut app, Locale::En).await;
    let accept_row = find_node(&history, text(&keys["accept"]["row"])).unwrap_or_else(|| panic!("the Accept row: {history}"));
    assert!(find_node(accept_row, text(&keys["accept"]["control"])).is_some(), "Accept sits in its row: {accept_row}");
    let inputs = find_node(&history, "framework.history.editor.inputs").expect("the inputs section");
    let produced: Vec<&str> = inputs["children"].as_array().expect("input rows").iter().filter_map(|row| row["key"].as_str()).collect();
    assert_eq!(produced, rows.iter().map(|row| text(&row["row"])).collect::<Vec<_>>(), "one row per corpus pointer, in order");
    for row in rows {
        let holder = find_node(inputs, text(&row["row"])).expect("the input row");
        assert!(find_node(holder, text(&row["control"])).is_some(), "{} edits through {}: {holder}", text(&row["pointer"]), text(&row["control"]));
    }
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}
//#endregion 🗝️EditorKeys

//#region 🩹️HistoryViewPatch
/// ✏️ Applies one toy edit straight on the document store — a row the command log backfills, like a seeded or ingested edit.
async fn apply_toy_edit(app: &mut ToyApp, value: i32) {
    app.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], transaction: None }).await.expect("toy edit applies");
}

/// 🧮️ Refreshes the history and answers how many rows it built and whether the cached view equals a full rebuild.
async fn refreshed_rows(app: &mut ToyApp) -> usize {
    HISTORY_ROWS_BUILT.with(|built| built.set(0));
    app.refresh_cache().await.expect("history refreshes");
    let built = HISTORY_ROWS_BUILT.with(std::cell::Cell::get);
    let cached = std::sync::Arc::clone(&app.cache.as_ref().expect("history cache").3);
    assert_eq!(cached.as_ref(), &app.build_history_view(None).await, "the patched history equals a full rebuild");
    built
}

/// ⚖️ LAW (design §20.14, audit W2A-1): over a long history, appending an edit, undoing it, redoing it and re-rendering rebuild
/// at most the touched rows (the new row, the previous newest row and the previous top's row) — never the history — and the
/// patched view always equals a full rebuild; a history edit (a supersede transition) rebuilds the whole view.
#[semio_framework_async_macros::async_test]
async fn the_history_view_patches_only_the_rows_a_change_touched() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for value in 0..200 {
        apply_toy_edit(&mut app, value).await;
    }
    let seeded = app.command_log.len();
    let built = refreshed_rows(&mut app).await;
    let rows = app.command_log.len();
    assert_eq!(rows, seeded + 200, "the 200 edits are backfilled rows");
    assert!(built <= 200 + 2, "the first refresh after 200 backfilled edits builds the new rows once ({built})");
    assert_eq!(refreshed_rows(&mut app).await, 0, "a refresh without a change builds no row");
    apply_toy_edit(&mut app, 1_000).await;
    assert!(refreshed_rows(&mut app).await <= 3, "an appended edit rebuilds its own row and its neighbours only");
    app.store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert!(refreshed_rows(&mut app).await <= 3, "an undo rebuilds the undone row and the uncovered top only");
    app.store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    assert!(refreshed_rows(&mut app).await <= 3, "a redo rebuilds the redone row and the covered top only");
    let _ = render_history(&mut app, Locale::En).await;
    assert_eq!(refreshed_rows(&mut app).await, 0, "rendering the history builds no row");
    let target = app.store.mutation_ops().expect("mutation ops")[0].mutation_id.clone();
    app.store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![store::SupersedeInput { target, replacement: Some(SetCount { value: 7 }.into()) }] }).await.expect("a history edit lands");
    assert!(refreshed_rows(&mut app).await >= rows, "a history edit rebuilds the whole view");
    close(&mut app);
}

/// ⚖️ LAW (audit W2A-11): a row whose operations past the cap carry outcomes projects its most severe one, so the row's worst
/// severity is the worst over every operation of its edit, however many are flagged; the projection stays bounded and in op
/// order.
#[test]
fn the_projected_rows_carry_the_worst_severity_of_the_whole_edit() {
    use semio_framework_diagnostic::Severity;
    for (len, error_at) in [(200, 199), (200, 33), (65, 64), (40, 39), (10, 5)] {
        let flagged = (0..len).filter(|index| *index >= HISTORY_ROW_MUTATION_ROWS || *index == error_at).map(|index| (index, Some(if index == error_at { Severity::Error } else { Severity::Warning })));
        let shown = time_travel::history_projected_operations(len, flagged);
        assert!(shown.contains(&error_at), "{len} operations: the Error at {error_at} is projected");
        assert!(shown.len() <= 2 * HISTORY_ROW_MUTATION_ROWS, "{len} operations: the projection stays bounded");
        assert!(shown.windows(2).all(|pair| pair[0] < pair[1]), "{len} operations: the projection is in op order");
        assert!(shown.iter().take(len.min(HISTORY_ROW_MUTATION_ROWS)).copied().eq(0..len.min(HISTORY_ROW_MUTATION_ROWS)), "{len} operations: the first ones always lead");
    }
}
//#endregion 🩹️HistoryViewPatch

//#region ⏱️WallDeadline
thread_local! {
    /// ⏱️ A clock on this test thread that advances one millisecond per read: every replayed operation "costs" 1 ms.
    static COUNTED_CLOCK_US: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// ⏱️ Reads [`COUNTED_CLOCK_US`], advancing it by one millisecond.
fn counted_clock() -> Option<u64> {
    Some(COUNTED_CLOCK_US.with(|now| {
        now.set(now.get() + 1_000);
        now.get()
    }))
}

/// ⚖️ LAW (design §20.14, audit W2A-3): a deferred history step ends its driver turn at the turn's wall deadline, never at
/// the operation cap — with every clock read costing 1 ms, an interior undo over 600 downstream edits replays a handful of
/// them per 4 ms turn (well under the 256-operation cap), over many turns, and still lands.
#[semio_framework_async_macros::async_test]
async fn a_deferred_history_step_ends_its_turn_at_the_wall_deadline_not_an_operation_count() {
    let fixture = fixture();
    let actor = "author".to_string();
    let mut app = seeded_app_as(&fixture, &actor).await;
    app.time_travel.set_turn_clock(counted_clock);
    app.store.defer_local_replays(Some(store::ReplayTurnBudget { wall_us: TIME_TRAVEL_TURN_WALL_US, operations: time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, now_us: counted_clock }));
    bury_edit_of(&mut app, &actor, 600).await;
    history_verb(&mut app, &actor, "undo", None).await;
    assert!(app.store.local_step_pending(), "the interior undo waits for its replay");
    let mut turns = 0usize;
    while app.store.local_step_pending() {
        let done = app.store.reprojection_progress().map_or(0, |progress| progress.done);
        app.advance_typed_operation_publication().await.expect("driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
        let after = app.store.reprojection_progress().map_or(u32::MAX, |progress| progress.done);
        assert!(after == u32::MAX || after.saturating_sub(done) <= 8, "a 4 ms turn of 1 ms operations replays a handful: {done} → {after}");
        turns += 1;
        assert!(turns < 1_000, "the step is adopted");
    }
    assert!(turns >= 600 / 8, "600 operations of 1 ms span many 4 ms turns ({turns})");
    close(&mut app);
}
//#endregion ⏱️WallDeadline

//#region 🪪️ActorIdentity
/// 🫵️ The author of every edit of `app`'s document, oldest first.
fn edit_authors(app: &ToyApp) -> Vec<Option<String>> {
    app.store.envelope().vcs.edits.iter().map(|edit| edit.actor.clone()).collect()
}

/// ⚖️ LAW (design §22.6, runtime half of §22.34): an opened instance acts as its admitted actor — at its open, across a
/// reload and on the revert route.
/// - construction binds the admitted actor before genesis; [`PluginApp::bind_actor`] verifies that same authority;
/// - a whole-document reload keeps the instance's actor although the loaded history ends in another author's edit, so an
///   undo there takes nothing back; a head-only pure hydrate keeps it too;
/// - the revert route acts as the reverting actor: the store acts as that actor afterwards and another actor's edit stays
///   applied; an undo takes back the acting actor's own edit.
#[semio_framework_async_macros::async_test]
async fn an_opened_instance_acts_as_its_admitted_actor_across_reload_and_on_the_revert_route() {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("ada".into())).await;
    assert_eq!(app.store.local_actor_id(), &protocol::ActorId("ada".into()), "construction binds the admitted actor");
    PluginApp::bind_actor(&mut app, "ada").await;
    assert_eq!(app.store.local_actor_id(), &protocol::ActorId("ada".into()), "the admitted actor is the instance's");
    publish_as(&mut app, "ada", SetCount { value: 1 }.into());
    app.refresh_cache().await.expect("the log backfills the instance's own edit");
    publish_as(&mut app, "grace", SetCount { value: 2 }.into());
    app.refresh_cache().await.expect("the log backfills the other actor's edit");
    assert_eq!(edit_authors(&app), [Some("ada".to_string()), Some("grace".to_string())], "each edit names the actor that published it");
    let other = app.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).expect("the other actor's edit");

    let mut reader = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into())).await;
    PluginApp::bind_actor(&mut reader, "reader").await;
    artifact_app_laws::load_document(&mut reader, &app.document_pack().await.expect("document pack")).await.expect("pack reload");
    reader.refresh_cache().await.expect("the reloaded log backfills");
    assert_eq!((reader.store.local_actor_id(), head(&reader).0), (&protocol::ActorId("reader".into()), 2), "a reload keeps the instance's actor although the history ends in another author's edit");
    history_verb(&mut reader, "reader", "undo", None).await;
    assert_eq!((head(&reader).0, reader.store.applied_edit_ids().len()), (2, 2), "an undo never takes back another author's edit");
    let pure_head = reader.store.snapshot().expect("the reloaded head").encode_pack();
    PluginApp::hydrate_pure_head(&mut reader, &pure_head).await.expect("the head-only lane hydrates");
    assert_eq!(reader.store.local_actor_id(), &protocol::ActorId("reader".into()), "a head-only hydrate keeps the instance's actor");

    let own = app.history_patch(true).await.expect("history patch").upserts.into_iter().find(|row| row.author.as_deref() == Some("ada")).expect("the instance's own row").seq;
    history_verb(&mut app, "ada", "revertToCommand", Some(DslValue::object([("entrySeq".to_string(), DslValue::uint(own))]))).await;
    assert_eq!(app.store.local_actor_id(), &protocol::ActorId("ada".into()), "the revert route acts as the reverting actor");
    assert!(app.store.applied_edit_ids().iter().any(|id| *id == other), "a revert never takes back another actor's edit");
    history_verb(&mut app, "grace", "undo", None).await;
    pump_until(&mut app, "the undo lands", |app| !app.store.local_step_pending()).await;
    assert!(!app.store.applied_edit_ids().iter().any(|id| *id == other), "an undo takes back the acting actor's own edit");
    close(&mut reader);
    close(&mut app);
}

/// ⚖️ LAW (design §22.34, the store half — red until a plain `Apply` authors as the store's actor): every route authors
/// as its acting actor. A route without an actor of its own (the text ingest) authors as the instance's admitted actor; a
/// route with one (the plain emit route) authors as its separately admitted user instance.
#[semio_framework_async_macros::async_test]
async fn every_route_authors_as_its_acting_actor() {
    let acting = |actor: &str| ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("ada".into())).await;
    PluginApp::bind_actor(&mut app, "ada").await;
    let first: TestMutation = SetCount { value: 1 }.into();
    PluginApp::ingest_operations_text(&mut app, &::protocol::OpText::print_op(&first)).await.expect("the text ingest applies");
    let ingested = edit_authors(&app);
    let pack = app.document_pack().await.expect("Ada's authoritative document");
    let mut grace = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("grace".into())).await;
    artifact_app_laws::load_document(&mut grace, &pack).await.expect("Grace opens Ada's shared history");
    grace.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::mutations(vec![SetCount { value: 2 }.into()]), &acting("grace")).await.expect("the emit publishes");
    let emitted = edit_authors(&grace);
    close(&mut app);
    close(&mut grace);
    assert_eq!(ingested, [Some("ada".to_string())], "a route without an actor of its own authors as the instance's");
    assert_eq!(emitted, [Some("ada".to_string()), Some("grace".to_string())], "the plain emit route authors as its acting actor");
}
//#endregion 🪪️ActorIdentity

//#region 🪣️SharedSchemaDocuments
/// 🪣️ A payload schema whose one input references the framework value schema (its numeric transport).
const SHARED_REFERENCE_SCHEMA: &str = r#"{"type":"object","additionalProperties":false,"required":["opacity"],"properties":{"opacity":{"$ref":"https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport","x-semio-ui":{"label":{"en":"Opacity","de":"Deckkraft"}}}}}"#;

/// ⚖️ LAW (design §23): the framework's own shared schema documents are in every instance's input-schema resolver — a
/// registryless instance, for which no plugin assembly published anything, holds the framework value schema, the io
/// vocabulary and the store's document model (child, owner, link, blob); an input that references the value schema reads
/// as one input and the draft editor renders its control.
#[semio_framework_async_macros::async_test]
async fn an_input_that_references_a_shared_framework_schema_resolves_on_any_instance() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for id in ["framework/value/schema.json", "framework/io/schema.json", "os/store/child/schema.json", "os/store/child/owner/schema.json", "os/store/link/schema.json", "os/store/blob/schema.json"] {
        let id = format!("https://json.schemas.assets.semio-tech.com/{id}");
        assert!(semio_framework::registered_input_schema_document(&id).is_some(), "{id} is always in the runtime resolver");
    }
    let inputs = semio_framework::mutation_input_defs(SHARED_REFERENCE_SCHEMA, &semio_framework::registered_input_schema_document).unwrap_or_else(|error| panic!("the reference resolves: {error:?}"));
    assert_eq!(inputs.len(), 1, "the referenced value is one input: {inputs:?}");
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = inputs;
    editor.value = dsl(&serde_json::json!({ "opacity": 0.5 }));
    let history = render_history(&mut app, Locale::De).await;
    let row = find_node(&history, "framework.history.editor.input.opacity").unwrap_or_else(|| panic!("the input's control: {history}"));
    assert!(history.to_string().contains("Deckkraft"), "the control carries the input's own label: {row}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}
//#endregion 🪣️SharedSchemaDocuments

//#region 🎚️HistoryFilter
/// ⚖️ LAW (schema-first, one vocabulary): every option `setHistoryCommandFilter` declares is a value its dispatch accepts and
/// the history wire echoes as `commandFilter`, the history body's select offers exactly those values, and an undeclared value
/// is refused instead of silently showing every row.
#[semio_framework_async_macros::async_test]
async fn every_declared_history_filter_option_is_dispatched_and_echoed() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    let definition = semio_framework::set_history_command_filter_action_definition();
    let declared: Vec<String> = match &definition.args[0].schema {
        semio_framework::ArgSchema::String { options, .. } => options.iter().map(|option| option.value.clone()).collect(),
        other => panic!("the filter is a choice: {other:?}"),
    };
    assert_eq!(declared, HistoryCommandFilter::ALL.map(|filter| filter.value().to_string()), "the declared options are the runtime's filters, in order");
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(&actor) };
    for value in &declared {
        let args = DslValue::Object(vec![("value".into(), DslValue::String(value.clone()))]);
        let admitted = app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&args), &meta).await.unwrap_or_else(|fault| panic!("{value}: {fault:?}"));
        crate::app::settle_framework_reserved_admission(&mut app, admitted).await.unwrap_or_else(|fault| panic!("{value}: {fault:?}"));
        assert_eq!(&app.history_patch(true).await.expect("history patch").command_filter, value, "the wire echoes the declared option");
        let body = render_history(&mut app, Locale::En).await;
        let select = find_node(&body, "framework.history.filter.control").expect("the filter select");
        for option in &declared {
            assert!(select.to_string().contains(&format!("\"{option}\"")), "the select offers {option}: {select}");
        }
    }
    let undeclared = DslValue::Object(vec![("value".into(), DslValue::String("withoutOperations".into()))]);
    let refused = match app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&undeclared), &meta).await {
        Ok(admitted) => crate::app::settle_framework_reserved_admission(&mut app, admitted).await.err(),
        Err(fault) => Some(fault),
    };
    assert!(refused.is_some(), "an undeclared filter value is refused");
    close(&mut app);
}
//#endregion 🎚️HistoryFilter


//#region ☑️LongOptionRows
/// ☑️ LAW (audit W1E-2, conformance `💬️row-semantics` `selectedRows`): a choice with more options than one fixed list holds is
/// windowed option rows, each a tree row holding its choose button and stating its choice as `selected` — the chosen option
/// `Some(true)`, every other `Some(false)` — so both renderers announce (`aria-selected`, the mirror's `selected`) and paint it,
/// never by its icon alone.
#[test]
fn long_option_rows_state_their_choice_as_selected() {
    let option = |index: usize| semio_framework::ActionArgOption { value: format!("o{index}"), label: LocalizedLabel::native(&format!("Option {index}"), &format!("Option {index}")) };
    let mode = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::String { options: (0..40).map(option).collect(), option_source: None, min_len: None, max_len: None, pattern: None, format: None },
        required: true,
        ..semio_framework::ActionArgDef::text("/mode", LocalizedLabel::native("Mode", "Modus"))
    };
    let value = dsl(&serde_json::json!({ "mode": "o3" }));
    let editor = TimeTravelEditorPanel { target: "m-1".into(), label: LocalizedLabel::native("Set mode", "Modus setzen"), rows: time_travel::time_travel_input_rows(&[mode], &value), editable: true, inputs_refused: None, withdrawn: false, outcome: Vec::new(), refused: None, changed: false, reference_labels: Default::default() };
    let panel = TimeTravelPanel { status: Default::default(), store: None, stage: TimeTravelStage::Editing, pending_after: None, finalize_refusal: None, review: None, rerun_refusal: None, begin_refusal: None, next_problem: None, editor: Some(editor.clone()), outcomes: Default::default(), edited: Default::default(), accepted: Default::default() };
    let sections = time_travel::time_travel_editor_sections(&panel, &editor, &TreeWindows::unhosted(), "toy", Locale::En).expect("the editor builds");
    fn option_rows(node: &BuiltNode, rows: &mut Vec<(String, Option<bool>)>) {
        if let semio_framework_ui_contract::Component::TreeItem(props) = &node.component {
            if node.key.as_str().contains(".option.") && node.key.as_str().ends_with(".row") {
                rows.push((node.key.as_str().to_string(), props.selected));
            }
        }
        node.children.iter().for_each(|child| option_rows(child, rows));
    }
    let mut rows = Vec::new();
    sections.iter().for_each(|section| option_rows(section, &mut rows));
    assert!(rows.len() >= 4 && rows.iter().any(|(_, selected)| *selected == Some(true)), "the first paint holds the chosen option among several: {rows:?}");
    for (key, selected) in &rows {
        assert_eq!(*selected, Some(key == "framework.history.editor.input.mode.option.3.row"), "{key}");
    }
}
//#endregion ☑️LongOptionRows

//#region 📡️RemoteEditWhileEditing
/// ⚖️ LAW (live fault F4, session half; design §7: remote ingests keep arriving → `BaseMoved`): a remote edit ingested
/// while a draft is being edited moves the base and nothing else. The session stays `Editing` with its draft, its editor
/// and its generation, so an input stamped with the generation the editor's controls carried before the base move is
/// not stale; the remote edit is listed downstream of the edited mutation as not applied while editing, and nothing
/// waits for a replay (a tail edit is adopted at ingest); Accept replays the remote edit too — the reviewed head holds
/// the draft and the remote edit — and the overwrite converges on the other replica.
#[semio_framework_async_macros::async_test]
async fn a_remote_edit_while_editing_keeps_the_draft_and_accept_replays_it_too() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "remote-while-editing-local", &actor, true).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "remote-while-editing-remote", "remote", false).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut local, &fixture, &step).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let (target, generation) = (seeded_mutation(&local, 1), local.time_travel.session().generation);
    let drafted = local.time_travel.session().pending.clone().expect("a pending draft");

    remote.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }).await.expect("the remote edit");
    relay_adopted(&mut remote_probe, &mut local_probe, &mut local).await;
    pump_until(&mut local, "the base move reaches the session", |app| app.time_travel.session().base.content_revision == app.store.content_revision()).await;
    let arrived = local.store.mutation_ops().expect("applied operations").last().map(|op| op.mutation_id.0.clone()).expect("the remote operation");
    assert!(arrived != seeded_mutation(&local, 3), "the remote edit is applied in this replica's store");

    let session = local.time_travel.session();
    assert_eq!((session.stage, session.generation, session.pending.as_ref()), (TimeTravelStage::Editing, generation, Some(&drafted)), "the base move keeps the stage, the generation and the draft");
    assert_eq!(local.time_travel.editor().map(|editor| (editor.target.0.clone(), editor.value.get("value").and_then(DslValue::as_str).map(str::to_string))), Some((target.clone(), Some("b".to_string()))), "the editor keeps its drafted value");
    assert!(local.reprojection_status().is_none(), "a tail edit is adopted at ingest: nothing waits for a replay");
    let wire = local.history_patch(true).await.expect("history patch").upserts.into_iter().flat_map(|row| row.mutations).find(|mutation| mutation.mutation_id == arrived).expect("the remote edit is a history row");
    assert!(wire.pending && wire.label.resolve(Terminology::Native, Locale::En) == "Set count to 9", "the remote edit is downstream and not applied in the preview: {wire:?}");
    let seq = local.history_patch(true).await.expect("history patch").upserts.iter().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == arrived)).map(|entry| entry.seq).expect("the history row of the remote edit");
    let body = render_history_window(&mut local, seq, 0, 8).await;
    let row = find_node(&body, &format!("framework.history.mutation.{arrived}")).unwrap_or_else(|| panic!("the mutation row of the remote edit: {body}"));
    assert!(row.to_string().contains("Not applied while editing"), "the row says so in words: {row}");
    assert!(render_body(&mut local).await.contains("count=1 label=b"), "the preview stays the state before the edited mutation with the draft");

    let typed = verb(&mut local, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("c".into())), ("generation".into(), DslValue::uint(u64::from(generation)))]).await;
    assert_eq!(rejected(&typed), None, "the user's next input, stamped with the generation from before the base move, is not stale");

    let accepted = verb(&mut local, &fixture, "historyEditAccept", Vec::new()).await;
    assert_eq!(rejected(&accepted), None, "{:?}", accepted.output);
    pump_until(&mut local, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    let status = local.time_travel.status().expect("a reviewing session");
    assert_eq!((status.review, status.blocking, status.accepted_count), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, 1), "the replay over the remote edit is clean");
    assert!(render_body(&mut local).await.contains("count=9 label=c"), "the reviewed head holds the draft and the remote edit");

    for (action, args) in [("historyEditFinalize", Vec::new()), ("historyEditCommit", vec![("choice".to_string(), DslValue::String("overwrite".into()))])] {
        let result = verb(&mut local, &fixture, action, args).await;
        assert_eq!(rejected(&result), None, "{action}: {:?}", result.output);
    }
    assert_eq!(head(&local), (9, "c".to_string()), "the overwrite lands on the remote edit");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote).await;
    assert_eq!(head(&remote), head(&local), "the other replica adopts the history edit");
    pump_until(&mut local, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    drop((local_probe, remote_probe));
    close(&mut local);
    close(&mut remote);
}

/// ⚖️ LAW (design §22.24, live fault F4): the base a session watches is the content revision of its store, never the
/// store's local generation. A backbone attaching and detaching while a draft is edited moves that generation and no
/// event: driver turns deliver no base move — the stage, the session generation, the base and the draft are what they
/// were — and an input stamped with the generation from before is accepted, not stale.
#[semio_framework_async_macros::async_test]
async fn a_backbone_attach_and_detach_while_editing_is_no_base_move() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut app, &fixture, &step).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let before = app.time_travel.session().clone();
    let (store_generation, revision) = (app.store.generation(), app.store.content_revision());
    let (backbone, probe) = MemoryBackbone::pair("attach-while-editing", "attach-while-editing").await;
    app.attach_backbone(store::Backbones::Memory(backbone)).await.expect("the backbone attaches");
    app.detach_backbone().await.expect("the backbone detaches");
    assert!(app.store.generation() > store_generation && app.store.content_revision() == revision, "the port moved the store's local generation and no event");
    for _ in 0..4 {
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.time_travel.session(), &before, "no base move: the stage, the generation, the base and the draft are what they were");
    let typed = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("c".into())), ("generation".into(), DslValue::uint(u64::from(before.generation)))]).await;
    assert_eq!(rejected(&typed), None, "an input stamped before the port change is accepted");
    assert!(render_body(&mut app).await.contains("count=1 label=c"), "the preview follows the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    drop(probe);
    close(&mut app);
}

/// ⚖️ LAW (design §22.24, store wave RB): a replay finished before a port change still commits. A backbone attaching and
/// detaching while the session reviews its finished replay moves the store's local generation and no event: driver turns
/// deliver no base move and start no second replay, and Finalize → Overwrite commits the head reviewed — no
/// `timeTravel.stale`, the session closes with its commit.
#[semio_framework_async_macros::async_test]
async fn a_replay_finished_before_a_backbone_attach_and_detach_still_commits() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        if let Some(result) = run_step(&mut app, &fixture, &step).await {
            assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
        }
    }
    let before = app.time_travel.session().clone();
    assert_eq!(before.stage, TimeTravelStage::Reviewing, "the replay finished");
    let (store_generation, revision) = (app.store.generation(), app.store.content_revision());
    let (backbone, probe) = MemoryBackbone::pair("attach-while-reviewing", "attach-while-reviewing").await;
    app.attach_backbone(store::Backbones::Memory(backbone)).await.expect("the backbone attaches");
    app.detach_backbone().await.expect("the backbone detaches");
    assert!(app.store.generation() > store_generation && app.store.content_revision() == revision, "the port moved the store's local generation and no event");
    for _ in 0..4 {
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.time_travel.session(), &before, "no base move: the review stands and no second replay started");
    for step in [serde_json::json!({ "finalize": null }), serde_json::json!({ "commit": { "choice": "overwrite" } })] {
        let result = run_step(&mut app, &fixture, &step).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: the replay finished before the port change still commits: {:?}", result.output);
    }
    pump_until(&mut app, "the finalize retires", |app| !app.time_travel.has_pending_work()).await;
    assert!(app.time_travel.status().is_none(), "the session closed with its commit: {:?}", app.time_travel.session().stage);
    assert_eq!(head(&app), (5, "b".to_string()), "the committed document is the head reviewed");
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "every window shows the committed edit");
    drop(probe);
    close(&mut app);
}
//#endregion 📡️RemoteEditWhileEditing

//#region 🎞️SessionEdgeScope
/// ⚖️ LAW (live probe: the `choosing` edge; design §20.14, per-render work is O(change)): a history-edit verb re-publishes
/// every window body only when the document a window shows was swapped — opening the session (committed → preview), a
/// draft (a new preview), a finished replay (preview → head), leaving (→ committed). The edges that swap nothing —
/// an accepted draft starting its replay, the finalize prompt opening and closing, a replay run again — re-publish the
/// history body alone; and a replay run again from a review keeps showing the head reviewed last, never the committed
/// document.
#[semio_framework_async_macros::async_test]
async fn a_session_edge_republishes_every_window_only_when_the_shown_document_swaps() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let full = |result: &InvocationResult| matches!(result.ui_scope, UiDirtyScope::Full);
    let history_only = |result: &InvocationResult| matches!(&result.ui_scope, UiDirtyScope::Partial { window_bodies, panel_bodies, .. } if window_bodies.is_empty() && panel_bodies.len() == 1 && panel_bodies[0] == FRAMEWORK_HISTORY_BODY_KEY);
    let mutation = seeded_mutation(&app, 1);
    let begun = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))]).await;
    assert!(rejected(&begun).is_none() && full(&begun), "opening shows the preview in every window: {:?}", begun.ui_scope);
    let drafted = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("b".into()))]).await;
    assert!(rejected(&drafted).is_none() && full(&drafted), "a draft is a new preview: {:?}", drafted.ui_scope);
    let accepted = verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    assert!(rejected(&accepted).is_none() && history_only(&accepted), "an accepted draft keeps its preview while it replays: {:?}", accepted.ui_scope);
    assert!(render_body(&mut app).await.contains("count=1 label=b"), "the preview stays while the replay runs");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the review shows the replayed head");

    let prompted = verb(&mut app, &fixture, "historyEditFinalize", Vec::new()).await;
    assert!(rejected(&prompted).is_none() && history_only(&prompted), "the finalize prompt swaps no document: {:?}", prompted.ui_scope);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Choosing);
    let back = verb(&mut app, &fixture, "historyEditBack", Vec::new()).await;
    assert!(rejected(&back).is_none() && history_only(&back), "leaving the prompt swaps no document: {:?}", back.ui_scope);

    let first = seeded_mutation(&app, 0);
    for (action, args) in [("historyEditBegin", vec![("mutationId".to_string(), DslValue::String(first))]), ("historyEditInput", vec![("path".to_string(), DslValue::String("/value".into())), ("value".to_string(), DslValue::uint(7))]), ("historyEditAccept", Vec::new())] {
        let result = verb(&mut app, &fixture, action, args).await;
        assert_eq!(rejected(&result), None, "{action}: {:?}", result.output);
    }
    let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new()).await;
    assert!(rejected(&cancelled).is_none() && full(&cancelled), "a cancelled replay leaves its preview for the head reviewed last: {:?}", cancelled.ui_scope);
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the cancelled review shows the head reviewed last");
    let rerun = verb(&mut app, &fixture, "historyEditRerun", Vec::new()).await;
    assert!(rejected(&rerun).is_none() && history_only(&rerun), "a replay run again keeps the head reviewed last: {:?}", rerun.ui_scope);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let replaying = render_body(&mut app).await;
    assert!(replaying.contains("count=5 label=b") && !replaying.contains(text(&fixture["committedBody"])), "no window falls back to the committed document while it replays again: {replaying}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the review shows the replayed head");

    let exited = verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    assert!(rejected(&exited).is_none() && full(&exited), "leaving shows the committed document in every window: {:?}", exited.ui_scope);
    assert!(render_body(&mut app).await.contains(text(&fixture["committedBody"])));
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}
//#endregion 🎞️SessionEdgeScope

//#region 🚫️RowWithdraw
/// 📍️ The rendered mutation row of seeded edit `index`, its history row opened as a host window shows it.
async fn seeded_mutation_node(app: &mut ToyApp, index: usize) -> Value {
    let id = seeded_mutation(app, index);
    let seq = app.history_patch(true).await.expect("history patch").upserts.iter().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == id)).map(|entry| entry.seq).expect("the history row of the seeded edit");
    let body = render_history_window(app, seq, 0, 8).await;
    find_node(&body, &format!("framework.history.mutation.{id}")).cloned().unwrap_or_else(|| panic!("the mutation row of seeded edit {index}: {body}"))
}

/// 🎬️ The row actions of a rendered mutation row: each verb, label, whether it is disabled and why.
fn row_actions(row: &Value) -> Vec<(String, String, bool, Option<String>)> {
    let text = |value: &Value| value.as_str().map(str::to_string);
    row["component"]["rowActions"].as_array().into_iter().flatten().map(|action| (text(&action["verb"]).unwrap_or_default(), text(&action["label"]).unwrap_or_default(), action["disabled"] == Value::Bool(true), text(&action["reason"]))).collect()
}

/// ⚖️ LAW (design §22.1, coordinator decision "Restore"): every applied mutation row offers Withdraw beside Edit on the one
/// row target. A row's Withdraw opens the session on a withdrawn draft (the preview is the state before the mutation, the
/// store untouched); on the mutation being edited it stays the editor's own while every other row names why it is blocked;
/// a running replay refuses it naming why; a mutation holding an accepted draft offers Restore in Withdraw's place and is
/// no longer withdrawable on the wire; restoring the only accepted draft leaves history editing with zero trace.
#[semio_framework_async_macros::async_test]
async fn a_mutation_row_offers_withdraw_and_restore_takes_an_accepted_draft_back() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let generation = app.store.generation();
    let (first, target) = (seeded_mutation(&app, 0), seeded_mutation(&app, 3));
    let enabled = |verb: &str, label: &str| (verb.to_string(), label.to_string(), false, None);
    let idle = seeded_mutation_node(&mut app, 3).await;
    assert_eq!(row_actions(&idle), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditWithdraw", "Withdraw")], "Edit stays first, Withdraw follows: {idle}");
    assert_eq!((idle["component"]["target"]["args"]["mutationId"].as_str(), idle["component"]["target"]["activation"].as_str()), (Some(target.as_str()), Some("historyEditBegin")), "one row target for both actions: {idle}");
    let wire = seeded_mutation_row(&mut app, 3).await;
    assert!(wire.editable && wire.withdrawable && !wire.withdrawn, "the wire row is withdrawable");

    let withdrawn = app.handle_action("historyEditWithdraw", Some(&dsl(&idle["component"]["target"]["args"])), &meta(&fixture)).await.expect("the row's Withdraw dispatches");
    assert_eq!(rejected(&withdrawn), None, "{:?}", withdrawn.output);
    let session = app.time_travel.session();
    assert_eq!((session.stage, session.pending.as_ref().map(|pending| (pending.target.mutation.0.clone(), pending.replacement.clone()))), (TimeTravelStage::Editing, Some((target.clone(), protocol::InputReplacement::Withdrawn))), "the session opens on a withdrawn draft");
    assert_eq!((render_body(&mut app).await.contains("count=1 label=a"), app.store.generation()), (true, generation), "the preview is the state before the mutation; the store is untouched");
    let editing = row_actions(&seeded_mutation_node(&mut app, 3).await);
    assert_eq!((editing[0].2, editing[0].3.as_deref().is_some_and(|reason| reason.starts_with("Blocked")), &editing[1]), (true, true, &enabled("historyEditWithdraw", "Withdraw")), "the withdrawn draft blocks Edit; Withdraw stays the editor's own: {editing:?}");
    let other = row_actions(&seeded_mutation_node(&mut app, 0).await);
    assert!(other.iter().all(|action| action.2 && action.3.as_deref().is_some_and(|reason| reason.starts_with("Blocked"))), "another row names the open draft: {other:?}");
    let blocked = verb(&mut app, &fixture, "historyEditWithdraw", vec![("mutationId".into(), DslValue::String(first.clone()))]).await;
    assert_eq!(rejected(&blocked), Some("timeTravel.blocked"), "the verb refuses what the row disables");

    verb(&mut app, &fixture, "historyEditAccept", Vec::new()).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let replaying = row_actions(&seeded_mutation_node(&mut app, 0).await);
    assert!(replaying.iter().all(|action| action.2 && action.3.as_deref() == Some("Not possible right now")), "a running replay disables Edit and Withdraw, naming why: {replaying:?}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying).await;
    let status = app.time_travel.status().expect("a session is open");
    assert_eq!((status.accepted_count, status.blocking, status.next_problem.is_none()), (1, false, true), "the withdrawal replays clean");
    assert_eq!(row_actions(&seeded_mutation_node(&mut app, 3).await), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditRestore", "Restore")], "an accepted draft offers Restore in Withdraw's place");
    assert_eq!(row_actions(&seeded_mutation_node(&mut app, 0).await), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditWithdraw", "Withdraw")], "the review withdraws other rows again");
    let wire = seeded_mutation_row(&mut app, 3).await;
    assert!(wire.withdrawn && !wire.withdrawable && wire.edited, "the replayed row reads withdrawn and is no longer withdrawable");
    let unaccepted = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(first.clone()))]).await;
    assert_eq!(rejected(&unaccepted), Some("timeTravel.illegal"), "a mutation without an accepted draft has nothing to restore");

    let restored = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(target.clone()))]).await;
    assert_eq!(rejected(&restored), None, "{:?}", restored.output);
    assert!(app.time_travel.status().is_none(), "restoring the only accepted draft leaves history editing");
    assert_eq!((render_body(&mut app).await.contains(text(&fixture["committedBody"])), app.store.generation()), (true, generation), "zero trace: the committed document, the store untouched");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// ⚖️ LAW (design §22.1, §22.20): a mutation row always names Edit and, unless it is withdrawn, Withdraw — or Restore on a
/// mutation holding an accepted draft. A mutation without editable inputs keeps Withdraw and reads why Edit is refused, a
/// mutation the store's supersede law lets nobody withdraw reads that, a viewer reads why both are refused, in both
/// locales; only an offered Edit is the row's activation.
#[test]
fn a_mutation_row_names_why_edit_or_withdraw_is_refused() {
    let entry = |editable: bool, withdrawable: bool, withdrawn: bool| semio_framework::kernel::HistoryMutationEntry {
        mutation_id: "e-1#0".into(),
        position: 0,
        op_index: 0,
        label: LocalizedLabel::native("Set label", "Beschriftung setzen"),
        worst: None,
        messages: Vec::new(),
        superseded: withdrawn,
        withdrawn,
        editable,
        withdrawable,
        pending: false,
        edited: false,
        introduced: false,
        store: None,
    };
    let row = |entry: &semio_framework::kernel::HistoryMutationEntry, locale: Locale, read_only: bool, refusals: time_travel::MutationRowRefusals| {
        let row = history_panel_mutation_row(entry, "toy", locale, read_only, refusals).expect("the row builds");
        let semio_framework_ui_contract::Component::TreeItem(props) = &row.component else { panic!("a mutation row is a tree row") };
        let actions: Vec<(String, Option<String>)> = props.row_actions.iter().map(|action| (action.verb.as_str().to_string(), action.reason.as_ref().map(|reason| reason.0.as_str().to_string()))).collect();
        assert!(props.row_actions.iter().all(|action| action.disabled == action.reason.is_some()), "a disabled action names why: {actions:?}");
        (actions, props.target.as_ref().and_then(|target| target.activation.as_ref().map(|verb| verb.as_str().to_string())))
    };
    let offered = |verb: &str| (verb.to_string(), None);
    let refused = |verb: &str, reason: &str| (verb.to_string(), Some(reason.to_string()));
    let idle = time_travel::MutationRowRefusals::default();
    assert_eq!(row(&entry(true, true, false), Locale::En, false, idle), (vec![offered("historyEditBegin"), offered("historyEditWithdraw")], Some("historyEditBegin".to_string())), "an editable mutation");
    assert_eq!(
        row(&entry(false, true, false), Locale::En, false, idle),
        (vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), offered("historyEditWithdraw")], None),
        "a mutation without editable inputs keeps Withdraw and activates nothing"
    );
    assert_eq!(
        row(&entry(false, true, false), Locale::De, false, idle).0,
        vec![refused("historyEditBegin", "Die Eingaben dieser Mutation können nicht bearbeitet werden"), offered("historyEditWithdraw")],
        "the reason is the reader's language"
    );
    assert_eq!(
        row(&entry(false, false, false), Locale::En, false, idle).0,
        vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), refused("historyEditWithdraw", "This mutation cannot be withdrawn here")],
        "the store's supersede law refuses the withdrawal"
    );
    assert_eq!(
        row(&entry(false, false, false), Locale::En, true, idle),
        (vec![refused("historyEditBegin", "History cannot be edited in a read-only view"), refused("historyEditWithdraw", "History cannot be edited in a read-only view")], None),
        "a viewer reads why"
    );
    assert_eq!(row(&entry(true, false, true), Locale::En, false, idle).0, vec![offered("historyEditBegin")], "a withdrawn mutation offers no second withdrawal");
    let accepted = time_travel::MutationRowRefusals { accepted: true, ..idle };
    assert_eq!(row(&entry(false, false, true), Locale::En, false, accepted).0, vec![refused("historyEditBegin", "The inputs of this mutation cannot be edited"), offered("historyEditRestore")], "an accepted draft is restored from its row");
    let replaying = time_travel::MutationRowRefusals { accepted: true, restore: Some(semio_framework_time_travel::TimeTravelLabel::RefusalIllegal), ..idle };
    assert_eq!(row(&entry(false, false, true), Locale::En, false, replaying).0[1], refused("historyEditRestore", "Not possible right now"), "outside the review Restore names why");
}

/// 🫥️ A payload schema whose one input is hidden: a draft editor would hold no row for it (design §22.20).
const HIDDEN_ONLY_SCHEMA: &str = r#"{"type":"object","additionalProperties":false,"required":["value"],"properties":{"value":{"type":"string","x-semio-ui":{"widget":"hidden","label":{"en":"Label","de":"Beschriftung"}}}}}"#;

/// 🪵️ A document operation over the toy kinds with two kinds an editor cannot repair — the label kind, whose payload
/// schema hides its only input and which refuses to apply on a negative count, and the children kind, which declares no
/// input schema and has no inverse under a count below -1: the smallest operations an upstream edit breaks without inputs
/// to edit.
#[derive(Clone, Debug, PartialEq)]
struct InertLabelOp(TestMutation);

impl semio_framework_value::ToValue for InertLabelOp {
    fn to_value(&self) -> DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}

impl semio_framework_value::FromValue for InertLabelOp {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <TestMutation as semio_framework_value::FromValue>::from_value(value).map(Self)
    }
}

impl ::protocol::OpBinary for InertLabelOp {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::protocol::OpBinary::encode_op(&self.0)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        <TestMutation as ::protocol::OpBinary>::decode_op(bytes).map(Self)
    }
}

impl ::protocol::OpText for InertLabelOp {
    fn print_op(&self) -> String {
        ::protocol::OpText::print_op(&self.0)
    }

    fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        <TestMutation as ::protocol::OpText>::parse_op(line).map(Self)
    }
}

impl store::Mutation<TestSnapshot> for InertLabelOp {
    type Diff = <TestMutation as store::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = <TestMutation as store::Mutation<TestSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        store::Mutation::<TestSnapshot>::descriptor(&self.0)
    }

    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::Diff> {
        match &self.0 {
            TestMutation::SetLabel(_) if base.count < 0 => ::protocol::MutationOutcome::error("mutation.target-mismatch", "a label needs a count that is not negative", ["count"]),
            operation => store::Mutation::<TestSnapshot>::diff(operation, base),
        }
    }

    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        if matches!(self.0, TestMutation::SetSlotChildren(_)) && base.count < -1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "children have no inverse under a count below -1"));
        }
        Ok(store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect())
    }

    fn conflict_target(&self) -> Vec<String> {
        store::Mutation::<TestSnapshot>::conflict_target(&self.0)
    }

    fn may_emit_foreign_steps(&self) -> bool {
        false
    }

    fn input_schema(&self) -> Option<&'static str> {
        match &self.0 {
            TestMutation::SetLabel(_) => Some(HIDDEN_ONLY_SCHEMA),
            TestMutation::SetSlotChildren(_) => None,
            operation => store::Mutation::<TestSnapshot>::input_schema(operation),
        }
    }

    fn payload_value(&self) -> DslValue {
        store::Mutation::<TestSnapshot>::payload_value(&self.0)
    }

    fn with_payload_value(&self, value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        store::Mutation::<TestSnapshot>::with_payload_value(&self.0, value).map(Self)
    }
}

/// ⏭️ Replays `drafts` from `from` on `document` to completion through its history-edit owners and answers the report.
async fn replayed_report(owners: &mut time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, document: &mut ArtifactStore<TestSnapshot, InertLabelOp>, drafts: &BTreeMap<MutationId, protocol::InputReplacement>, from: &MutationId) -> protocol::ReplayReport {
    let started = owners.run(document, time_travel::TimeTravelStoreCommand::StartReplay { drafts: drafts.clone(), from: from.clone() }).await.expect("the replay starts");
    assert!(matches!(started, time_travel::TimeTravelStoreOutput::ReplayStarted(true)), "the store starts the Report replay");
    for _ in 0..65_536 {
        match owners.run(document, time_travel::TimeTravelStoreCommand::StepReplay { deadline_us: u64::MAX, clock: semio_framework_job::default_now_us }).await.expect("a replay slice") {
            time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Completed(report)) => return report,
            time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Pending { .. }) => {}
            _ => panic!("the replay faulted"),
        }
    }
    panic!("the replay never completed");
}

/// 🌰️ The schema id of the [`InertLabelOp`] documents.
const INERT_SCHEMA: &str = "semio.test.inert-label/v1";

/// 🥊️ A document store over [`InertLabelOp`] holding count 1, label a, no children and count 5 — one edit each, authored
/// by `local` — with the mutation ids of those four operations.
async fn inert_document() -> (ArtifactStore<TestSnapshot, InertLabelOp>, Vec<MutationId>) {
    let genesis = store::create_document_envelope::<TestSnapshot, InertLabelOp>(INERT_SCHEMA, "inert-label", TestSnapshot::default(), None);
    let mut document = Box::pin(ArtifactStore::new(genesis, protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into()))).await.expect("the document store");
    document.install_document_store_owners_exact(bounded_document_store_owners::<TestSnapshot, InertLabelOp>());
    assert_eq!(document.local_actor_id(), &protocol::ActorId("local".to_string()));
    for operation in [TestMutation::SetCount(SetCount { value: 1 }), TestMutation::SetLabel(SetLabel { value: "a".into() }), TestMutation::SetSlotChildren(SetSlotChildren { children: Vec::new() }), TestMutation::SetCount(SetCount { value: 5 })] {
        Box::pin(document.dispatch(ArtifactCommand::Apply { mutations: vec![InertLabelOp(operation)], transaction: None })).await.expect("a seed edit applies");
    }
    let ids = document.mutation_ops().expect("applied operations").iter().map(|op| op.mutation_id.clone()).collect();
    (document, ids)
}

/// 🪤️ Settles and retires the history-edit owners of an [`inert_document`], then closes it under its exact grant.
fn close_inert(mut owners: time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, mut document: ArtifactStore<TestSnapshot, InertLabelOp>) {
    use time_travel::TimeTravelOwners;
    owners.settle(&document, TimeTravelStage::Inactive).expect("the owners settle");
    for _ in 0..65_536 {
        if owners.retire_step(64, usize::MAX).expect("the owners retire").is_none() {
            break;
        }
    }
    assert!(owners.terminal_is_empty(), "the history-edit owners retire everything they held");
    for _ in 0..65_536 {
        match document.close_owned_step(64, 4096).expect("the document store closes under its exact grant") {
            store::SnapshotRetirementStep::Pending { .. } => {}
            store::SnapshotRetirementStep::Blocked => panic!("the document store has no external owner"),
            store::SnapshotRetirementStep::Complete => {
                assert!(document.close_owned_terminal_is_empty());
                return;
            }
        }
    }
    panic!("the document store did not close");
}

/// ⚖️ LAW (design §22.1, §22.20): a blocking mutation without editable inputs is always resolvable. A mutation whose
/// payload schema hides every input, and one that declares no input schema, are not editable and still withdrawable (never
/// for a viewer). An upstream edit makes the hidden-only one fail; opening it for editing is refused
/// `timeTravel.not-editable` (an editor with zero rows never opens); opening it to withdraw it is admitted by the store's
/// supersede law on an editor without inputs; the replay with the withdrawal no longer blocks and finalizes as one
/// overwrite whose head holds the edit and skips the withdrawn operation.
#[semio_framework_async_macros::async_test]
async fn a_blocking_mutation_without_editable_inputs_is_withdrawn_and_the_review_becomes_ready() {
    use time_travel::{TimeTravelActionRefusal, TimeTravelCommit, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
    let schema = INERT_SCHEMA;
    let (mut document, ids) = inert_document().await;
    assert!(semio_framework::mutation_input_defs(HIDDEN_ONLY_SCHEMA, &semio_framework::registered_input_schema_document).is_ok_and(|inputs| inputs.len() == 1 && inputs[0].presentation == Some(semio_framework::ArgPresentation::Hidden)), "the hidden-only schema reads as one hidden input");
    let (count, label, children) = (ids[0].clone(), ids[1].clone(), ids[2].clone());
    {
        let ops = document.mutation_ops().expect("applied operations");
        let unlabelled = HashMap::new();
        let row = |index: usize, viewer: bool| {
            let view = time_travel::history_mutation_view_of::<TestSnapshot, InertLabelOp>(&ops[index], None, &unlabelled, &|_: &InertLabelOp| LocalizedLabel::data("operation"), viewer, None);
            (view.editable, view.withdrawable)
        };
        assert_eq!((row(0, false), row(1, false), row(2, false)), ((true, true), (false, true), (false, true)), "hidden-only inputs and no input schema: not editable, still withdrawable");
        assert_eq!((row(0, true), row(1, true), row(2, true)), ((false, false), (false, false), (false, false)), "a viewer edits and withdraws nothing");
    }

    let mut owners = TimeTravelStoreState::<TestSnapshot, InertLabelOp>::new(|_| LocalizedLabel::data("operation"));
    let negative = protocol::InputReplacement::Input { schema: schema.to_string(), payload: ::protocol::OpBinary::encode_op(&InertLabelOp(TestMutation::SetCount(SetCount { value: -1 }))).expect("the edited count encodes") };
    let mut drafts = BTreeMap::from([(count.clone(), negative)]);
    let blocked = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    let problem = blocked.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst))).map(|outcome| outcome.mutation_id.clone());
    assert_eq!((blocked.blocks_finalize(), problem), (true, Some(label.clone())), "the edited count breaks the label operation downstream: {blocked:?}");

    for (target, why) in [(&label, "an editor with zero rows never opens"), (&children, "an operation without an input schema has no editor")] {
        let refused = owners.run(&mut document, TimeTravelStoreCommand::Open { target: target.clone(), current: None, withdraw: false }).await.expect("the open runs");
        assert!(matches!(refused, TimeTravelStoreOutput::Opened(Err(TimeTravelActionRefusal::NotEditable))), "{why}");
    }
    let TimeTravelStoreOutput::Opened(Ok((editor, original))) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: label.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true }).await.expect("the open runs") else {
        panic!("the store's supersede law admits withdrawing an operation without editable inputs");
    };
    assert!(editor.inputs.is_empty() && editor.inputs_refused.is_none() && editor.target == label, "its editor holds no inputs and names no schema fault");
    assert!(matches!(original, protocol::InputReplacement::Input { .. }), "its original input is its recorded operation");
    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true)).await.expect("the session adopts the kind");

    drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
    let ready = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(!ready.blocks_finalize() && ready.outcomes.iter().any(|outcome| outcome.mutation_id == label && outcome.withdrawn), "withdrawing the failing operation leaves a report that finalizes: {ready:?}");
    let committed = owners.run(&mut document, TimeTravelStoreCommand::Commit { drafts: drafts.clone(), finalization: store::HistoryFinalization::Overwrite, actor: None }).await.expect("the commit runs");
    assert!(matches!(committed, TimeTravelStoreOutput::Committed(TimeTravelCommit::Finalized { .. })), "the finished replay commits as one overwrite");
    let head = document.snapshot().expect("head");
    assert_eq!((head.count, head.label.as_str()), (5, ""), "the head holds the edit and skips the withdrawn operation");

    close_inert(owners, document);
}

/// ⚖️ LAW (design §22.5, §22.17): one inverse failure is one mutation's fatal, never a faulted session. An upstream edit
/// under which a downstream operation has no inverse leaves the Report replay complete: that operation's outcome is one
/// `Fatal` `mutation.inverse-refused`, the operation after it is still replayed and applies, and the report blocks
/// finalizing. Its row says so in words — "Fatal: Cannot be reversed" / "Kritisch: Nicht umkehrbar", never the code — and
/// is the way out: withdrawing it, with the other operation the same edit broke, leaves a report that finalizes as one
/// overwrite.
#[semio_framework_async_macros::async_test]
async fn an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves() {
    use time_travel::{TimeTravelCommit, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
    let (mut document, ids) = inert_document().await;
    let (count, label, children, tail) = (ids[0].clone(), ids[1].clone(), ids[2].clone(), ids[3].clone());
    let mut owners = TimeTravelStoreState::<TestSnapshot, InertLabelOp>::new(|_| LocalizedLabel::data("operation"));
    let lower = protocol::InputReplacement::Input { schema: INERT_SCHEMA.to_string(), payload: ::protocol::OpBinary::encode_op(&InertLabelOp(TestMutation::SetCount(SetCount { value: -2 }))).expect("the edited count encodes") };
    let mut drafts = BTreeMap::from([(count.clone(), lower)]);
    let outcome_of = |report: &protocol::ReplayReport, id: &MutationId| report.outcomes.iter().find(|outcome| outcome.mutation_id == *id).cloned().expect("an outcome per replayed operation");
    let blocked = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    let refused = outcome_of(&blocked, &children);
    assert_eq!((refused.worst, refused.messages.iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>()), (Some(semio_framework_diagnostic::Severity::Fatal), vec!["mutation.inverse-refused"]), "the refused inverse is one fatal on its mutation: {blocked:?}");
    assert!(blocked.blocks_finalize() && outcome_of(&blocked, &tail).worst.is_none(), "the replay went on past the refusal and the report blocks: {blocked:?}");
    {
        let ops = document.mutation_ops().expect("applied operations");
        let unlabelled = HashMap::new();
        let view = time_travel::history_mutation_view_of::<TestSnapshot, InertLabelOp>(&ops[2], Some(&refused), &unlabelled, &|_: &InertLabelOp| LocalizedLabel::data("operation"), false, None);
        let row = time_travel::history_mutation_entry(&view, None);
        assert_eq!((time_travel::history_mutation_description(&row, Locale::En), time_travel::history_mutation_description(&row, Locale::De)), ("Fatal: Cannot be reversed".to_string(), "Kritisch: Nicht umkehrbar".to_string()), "the row names the refusal in words, never the code");
        assert!(view.withdrawable && !view.editable, "the row offers the way out");
    }

    let TimeTravelStoreOutput::Opened(Ok(_)) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: children.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true }).await.expect("the open runs") else {
        panic!("the store's supersede law admits withdrawing the operation without an inverse");
    };
    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true)).await.expect("the session adopts the kind");
    drafts.insert(children.clone(), protocol::InputReplacement::Withdrawn);
    let half = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(half.blocks_finalize() && outcome_of(&half, &children).withdrawn && outcome_of(&half, &children).worst.is_none(), "withdrawn, the refusal is gone and the other broken operation still blocks: {half:?}");
    drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
    let ready = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(!ready.blocks_finalize(), "every broken operation withdrawn, the report finalizes: {ready:?}");
    let committed = owners.run(&mut document, TimeTravelStoreCommand::Commit { drafts: drafts.clone(), finalization: store::HistoryFinalization::Overwrite, actor: None }).await.expect("the commit runs");
    assert!(matches!(committed, TimeTravelStoreOutput::Committed(TimeTravelCommit::Finalized { .. })), "the finished replay commits as one overwrite");
    let head = document.snapshot().expect("head");
    assert_eq!((head.count, head.label.as_str()), (5, ""), "the head holds the edit and skips the withdrawn operations");
    close_inert(owners, document);
}

/// ⚖️ LAW (design §22.6): the user-input path answers, it never panics. Every history-edit verb with missing, unknown
/// and wrongly typed arguments is answered — a result or a fault with a code — without a session, where the stage, the
/// document and its revision stay what they were, and inside one that edits a draft, where the session stays what it was.
#[semio_framework_async_macros::async_test]
async fn hostile_history_edit_input_is_answered_never_panicked_on() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    let text_arg = |key: &str, value: &str| (key.to_string(), DslValue::String(value.to_string()));
    let garbage = || vec![("mutationId".to_string(), DslValue::uint(7)), ("path".to_string(), DslValue::uint(3)), ("value".to_string(), DslValue::Null), ("generation".to_string(), DslValue::String("x".into())), text_arg("choice", "sideways"), text_arg("name", ""), text_arg("store", "no-such-store")];
    let answered = |action: &str, answer: Result<InvocationResult, Fault>| {
        if let Err(fault) = answer {
            assert!(!fault.code.0.is_empty(), "{action}: a refusal carries its code: {fault:?}");
        }
    };

    let before = (head(&app), app.store.content_revision_now());
    for action in semio_framework::HISTORY_EDIT_ACTION_IDS {
        for args in [Vec::new(), vec![text_arg("mutationId", "no-such-mutation")], vec![("mutationId".to_string(), DslValue::uint(7))], vec![text_arg("mutationId", "no-such-mutation"), text_arg("store", "no-such-store")], garbage()] {
            let answer = app.handle_action(action, Some(&DslValue::Object(args.clone())), &meta(&fixture)).await;
            answered(action, answer);
            assert_eq!((app.time_travel.session().stage, head(&app), app.store.content_revision_now()), (TimeTravelStage::Inactive, before.0.clone(), before.1), "{action} {args:?}: nothing opened and nothing changed");
        }
    }

    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut app, &fixture, &step).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let session = app.time_travel.session().clone();
    let far = u64::from(session.generation) + 4_000_000;
    let hostile: Vec<(&str, Vec<(String, DslValue)>)> = vec![
        ("historyEditBegin", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditBegin", vec![("mutationId".to_string(), DslValue::uint(7))]),
        ("historyEditBegin", garbage()),
        ("historyEditWithdraw", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditRestore", Vec::new()),
        ("historyEditRestore", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditInput", Vec::new()),
        ("historyEditInput", vec![text_arg("path", "/no/such/input"), ("value".to_string(), DslValue::Null)]),
        ("historyEditInput", vec![("path".to_string(), DslValue::uint(3)), ("value".to_string(), DslValue::uint(3))]),
        ("historyEditInput", vec![text_arg("path", "/value"), text_arg("value", "c"), ("generation".to_string(), DslValue::uint(far))]),
        ("historyEditInput", garbage()),
        ("historyEditCommit", vec![text_arg("choice", "sideways")]),
        ("historyEditFinalize", Vec::new()),
        ("historyEditRerun", Vec::new()),
        ("historyEditCancelReplay", Vec::new()),
    ];
    for (action, args) in hostile {
        let answer = app.handle_action(action, Some(&DslValue::Object(args.clone())), &meta(&fixture)).await;
        answered(action, answer);
        assert_eq!(app.time_travel.session(), &session, "{action} {args:?}: the session is what it was");
    }
    assert!(render_body(&mut app).await.contains("count=1 label=b"), "the draft preview stands");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}
//#endregion 🚫️RowWithdraw

//#region 🖐️GestureSlot
#[path = "../🧪️gesture/🦀️.rs"]
mod gesture_laws;
//#endregion 🖐️GestureSlot

//#region 📁️FolderReloadRoute
#[path = "../🧪️folder-reload-route/🦀️.rs"]
mod folder_reload_route;
//#endregion 📁️FolderReloadRoute
