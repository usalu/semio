//! 🏷️ History row labels survive a reload, driven by the language-agnostic fixture
//! `🧫️fixtures/🧫️history-label-reload/🔣️.json` whose cases the TypeScript twin (`🟦️.ts` beside this file) validates with
//! Ajv and derives with its own implementation of the label rule: each command dispatches through the real typed lane,
//! and its row reads the same English and German label live, after a text reload and after a pack reload, because the
//! projection only reads persisted facts (the edit's locale-neutral `verb`, its description and its operations).

use super::*;
use crate::test_app_mutation_fixture::{SetCount, SetLabel, TestConfig, TestConfigMutation, TestMutation, TestSnapshot};
use serde_json::Value;

const HISTORY_LABEL_RELOAD_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/🧫️history-label-reload/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(HISTORY_LABEL_RELOAD_FIXTURE_JSON).expect("history-label-reload fixture parses")
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, got {value}"))
}

//#region 🧸️LabelReloadApp
/// 🎮️ The fixture app's commands: an app-described rename under an undeclared verb, a retitle whose declared verb
/// outranks its description, an undescribed two-operation reset under a declared verb, an undescribed single count and
/// an undescribed two-operation pair under an undeclared verb.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslOps)]
enum LabelReloadCommand {
    Rename { value: String },
    Retitle { value: String },
    Reset { value: i32 },
    Count { value: i32 },
    Pair { value: i32 },
}

impl ::protocol::OpBinary for LabelReloadCommand {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        ::dsl::variants_binary::decode_op(bytes)
    }
}

/// 🧪️ The fixture case's command.
fn command(case: &Value) -> LabelReloadCommand {
    let value = &case["command"]["value"];
    let number = || value.as_i64().expect("integer command value") as i32;
    match text(&case["command"]["kind"]) {
        "rename" => LabelReloadCommand::Rename { value: text(value).to_string() },
        "retitle" => LabelReloadCommand::Retitle { value: text(value).to_string() },
        "reset" => LabelReloadCommand::Reset { value: number() },
        "count" => LabelReloadCommand::Count { value: number() },
        "pair" => LabelReloadCommand::Pair { value: number() },
        other => panic!("unknown fixture command {other}"),
    }
}

fn set_label(value: &str) -> TestMutation {
    SetLabel { value: value.to_string() }.into()
}

fn set_count(value: i32) -> TestMutation {
    SetCount { value }.into()
}

#[derive(Default)]
struct LabelReloadApp;

impl ArtifactApp for LabelReloadApp {
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.history-label-reload", standard: StandardId("1"), subset: SubsetId::ANY };
    const APP_ID: &'static str = "s.test.history-label-reload@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-history-label-reload/v1";
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
    type Command = LabelReloadCommand;

    fn mutation_label(op: &TestMutation) -> Option<LocalizedLabel> {
        Some(protocol::SemanticMutation::label(op))
    }

    async fn command_id(command: &LabelReloadCommand) -> &'static str {
        match command {
            LabelReloadCommand::Rename { .. } => "renameDescribed",
            LabelReloadCommand::Retitle { .. } => "retitle",
            LabelReloadCommand::Reset { .. } => "reset",
            LabelReloadCommand::Count { .. } => "count",
            LabelReloadCommand::Pair { .. } => "pairUndeclared",
        }
    }

    async fn initial_snapshot() -> TestSnapshot {
        TestSnapshot::default()
    }

    async fn handle(
        command: &LabelReloadCommand,
        _doc: &ArtifactView<'_, TestSnapshot>,
        _cfg: &ConfigView<'_, TestConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<TestMutation, TestConfigMutation, NoDraftMutation> {
        Ok(match command {
            LabelReloadCommand::Rename { value } => Emit { artifact_mutations: vec![set_label(value)], description: Some(format!("Renamed to {value}")), ..Default::default() },
            LabelReloadCommand::Retitle { value } => Emit { artifact_mutations: vec![set_label(value)], description: Some(format!("Retitled to {value}")), ..Default::default() },
            LabelReloadCommand::Reset { value } => Emit { artifact_mutations: vec![set_count(*value), set_label("reset")], ..Default::default() },
            LabelReloadCommand::Count { value } => Emit { artifact_mutations: vec![set_count(*value)], ..Default::default() },
            LabelReloadCommand::Pair { value } => Emit { artifact_mutations: vec![set_count(*value), set_label("pair")], ..Default::default() },
        })
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
        Some(bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("history-label-reload-doc", 4_096))
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
//#endregion 🧸️LabelReloadApp

//#region 🧰️Harness
type LabelReloadInstance = VcsArtifactApp<LabelReloadApp>;

/// 🛂️ The fixture app's manifest, declaring every registry verb of the fixture with its English and German label.
async fn manifest(fixture: &Value) -> App {
    let mut builder = App::builder(LabelReloadApp::APP_ID, LocalizedLabel::native("History Label Reload", "Verlaufsbeschriftung neu laden"))
        .await
        .document(["state"])
        .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::native("Main", "Haupt"), "history-label-reload.main", SurfaceKind::Canvas2d, IconName::AppWindow)
        .await;
    for row in fixture["registry"].as_array().expect("registry") {
        builder = builder.mutation(text(&row["verb"]), LocalizedLabel::native(text(&row["label"]["en"]), text(&row["label"]["de"]))).await;
    }
    App::from_builder(builder).await
}

/// 📜️ The edit rows of `app` in log order with their action id and English and German label.
async fn edit_rows(app: &mut LabelReloadInstance) -> Vec<(String, String, String)> {
    app.refresh_cache().await.expect("history refresh");
    let mut rows: Vec<_> = app.history_patch(true).await.expect("history patch").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    rows.sort_by_key(|row| row.seq);
    rows.into_iter().map(|row| (row.action_id.clone(), row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string())).collect()
}

/// 🆕️ A fresh registered instance of the fixture app, opened by `actor`.
async fn open(fixture: &Value, actor: &str) -> LabelReloadInstance {
    let mut app = artifact_app_laws::new_registered_app::<LabelReloadApp, _>(manifest(fixture)).await;
    app.store.set_local_actor_id(Some(actor.to_string())).expect("local actor");
    app
}
//#endregion 🧰️Harness

/// ⚖️ LAW: every fixture command is one history row whose action id is its verb and whose English and German label are
/// the fixture's, live, after a text reload and after a pack reload — an app-described, a verb-described and a
/// leaf-described row alike — and the reloaded edits keep their verbs.
#[semio_framework_async_macros::async_test]
async fn history_labels_survive_a_text_and_a_pack_reload_in_every_locale() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().expect("cases");
    let mut live = open(&fixture, "author").await;
    let receiver = artifact_app_laws::meta("author").instance_id;
    for case in cases {
        let meta = ActionMeta { view_state: Some(ViewModel::default()), ..artifact_app_laws::meta("author") };
        live.dispatch_typed(command(case), &meta).await.unwrap_or_else(|fault| panic!("{}: {fault:?}", text(&case["id"])));
        artifact_app_laws::settle_registered_typed_operation(&mut live, receiver).await.unwrap_or_else(|fault| panic!("{} settles: {fault:?}", text(&case["id"])));
    }
    let expected: Vec<(String, String, String)> = cases.iter().map(|case| (text(&case["verb"]).to_string(), text(&case["expected"]["en"]).to_string(), text(&case["expected"]["de"]).to_string())).collect();
    let verbs: Vec<Option<String>> = cases.iter().map(|case| Some(text(&case["verb"]).to_string())).collect();
    let mut reloaded = open(&fixture, "author").await;
    reloaded.load_document_text(&live.document_text().await.expect("document text")).await.expect("text reload");
    let mut repacked = open(&fixture, "reader").await;
    repacked.load_document_pack(&live.document_pack().await.expect("document pack")).await.expect("pack reload");
    for (who, app) in [("live", &mut live), ("text reload", &mut reloaded), ("pack reload", &mut repacked)] {
        assert_eq!(edit_rows(app).await, expected, "{who}: every row keeps its verb and its English and German label");
        assert_eq!(app.store.envelope().vcs.edits.iter().map(|edit| edit.verb.clone()).collect::<Vec<_>>(), verbs, "{who}: every edit carries the verb that authored it");
    }
    for app in [&mut live, &mut reloaded, &mut repacked] {
        artifact_app_laws::close_registered_fixture_app(app);
    }
}
