// ⏪️ Support module 1 of the split test module `🧪️time-travel`: the fixtures that more than one piece uses, as real definitions the pieces import.
use semio_framework_plugin_decl_emission::{ActionMeta, AsyncTaskRun, AsyncTaskSource, Emit, HostEvent, ToolCommandCatalog};
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_app::{ArtifactApp, NaturalFileDecodeCursor};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, DraftView, InteractionView, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
#[cfg(test)]
use semio_framework_ui_locale::{Label, Locale, LocalizedLabel, Terminology};
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_job::StepContext;
use semio_framework_ui_declaration::ui::SurfaceKind;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_builder as builder;
use semio_framework_plugin_window_transient::built_text_to_component_tree;
use semio_framework_os_kernel::io;
use semio_framework_plugin_ownership::transient_publication::transient_store_disposer;
use semio_framework::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_slot_children::SetSlotChildren;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::TestConfig;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_time_travel::TimeTravelStage;
use store::os_store::backbone::Backbone;


#[doc(hidden)]
pub const TIME_TRAVEL_FIXTURE_JSON: &str = include_str!("../🧫️fixtures/🧫️time-travel/🔣️.json");

#[doc(hidden)]
pub fn fixture() -> Value {
    serde_json::from_str(TIME_TRAVEL_FIXTURE_JSON).expect("time-travel fixture parses")
}

#[doc(hidden)]
pub fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, got {value}"))
}

#[doc(hidden)]
pub fn dsl(value: &Value) -> DslValue {
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture value is JSON"))
}

//#region 🧸️ToyHistoryApp
thread_local! {
    /// 📨️ Every host event the toy app received on this test thread, in order.
    pub static TOY_HOST_EVENTS: std::cell::RefCell<Vec<HostEvent>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[doc(hidden)]
#[derive(Default)]
pub struct ToyHistoryApp;

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
        _engines: &semio_framework_2d_compute::EngineHandles,
    ) -> ArtifactMutationOutcome<TestMutation, TestConfigMutation, NoDraftMutation> {
        Ok(Emit { artifact_mutations: vec![command.clone()], ..Default::default() })
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={} label={}", doc.snapshot.count, doc.snapshot.label)))
    }

    fn build_document_store_owners(grant:RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(bounded_document_store_owners::<Self::Snapshot, Self::Mutation>(grant))
    }

    fn build_config_store_owners(grant:RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        Some(bounded_config_store_owners::<Self::Config, Self::ConfigMutation>(grant))
    }

    fn build_draft_store_owners(grant:RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Draft, Self::DraftMutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Draft, Self::DraftMutation>>> {
        Some(bounded_document_store_owners::<Self::Draft, Self::DraftMutation>(grant))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
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
#[doc(hidden)]
pub type ToyApp = VcsArtifactApp<ToyHistoryApp>;

thread_local!{static ORIGINAL_HISTORY_RECEIPTS:std::cell::RefCell<Vec<(semio_framework_value::retained_clone::RetainedCloneGrant,semio_framework_value::retained_clone::RetainedCloneProgress)>>=const{std::cell::RefCell::new(Vec::new())};}
thread_local!{static ORIGINAL_PREPARATION_RECEIPTS:std::cell::RefCell<Vec<(semio_framework_job::OperationId,semio_framework_value::retained_clone::RetainedCloneGrant,semio_framework_value::retained_clone::RetainedCloneProgress)>>=const{std::cell::RefCell::new(Vec::new())};}
#[doc(hidden)]
pub fn collect_original_history_command_receipt(app:&mut ToyApp){
 if let Some((operation,grant,receipt))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,app.mounted_policy.preparation);assert!(receipt.fits(grant));assert!(grant.maximum_depth>0||receipt==Default::default());ORIGINAL_PREPARATION_RECEIPTS.with(|receipts|receipts.borrow_mut().push((operation,grant,receipt)));}
 if let Some((grant,receipt))=app.take_history_command_receipt(){assert_eq!(grant,app.mounted_policy.maintenance);assert!(receipt.fits(grant));ORIGINAL_HISTORY_RECEIPTS.with(|receipts|receipts.borrow_mut().push((grant,receipt)));}
}

/// 🎟️ Reads independent original caller ceilings before any native fixture work.
#[doc(hidden)]
pub fn original_time_travel_fixture_caller()->(semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,usize){
 let law:Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json")).unwrap();let declared=&law["native"]["grant"];let grant=semio_framework_value::RetainedCloneGrant{maximum_items:declared[0].as_u64().unwrap()as usize,maximum_copy_bytes:declared[1].as_u64().unwrap()as usize,maximum_capacity_bytes:declared[2].as_u64().unwrap()as usize,maximum_release_bytes:(declared[3].as_u64().unwrap()as usize).max(1_048_576),maximum_depth:declared[4].as_u64().unwrap()as usize};
 (semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:grant,maintenance:grant,close:grant}.validate().unwrap(),law["native"]["maximumIdentityBytes"].as_u64().unwrap()as usize)
}

#[doc(hidden)]
pub async fn toy_manifest() -> App {
    let builder = App::builder(ToyHistoryApp::APP_ID, LocalizedLabel::data("Time Travel Fixture"))
        .await
        .document(["state"])
        .mode("edit", LocalizedLabel::data("Edit"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::data("Main"), "time-travel.main", SurfaceKind::Canvas2d, IconName::AppWindow)
        .await;
    App::from_builder(builder).await
}

#[doc(hidden)]
pub fn seed_op(op: &Value) -> TestMutation {
    match text(&op["kind"]) {
        "setCount" => SetCount { value: op["value"].as_i64().expect("count") as i32 }.into(),
        "setLabel" => SetLabel { value: text(&op["value"]).to_string() }.into(),
        _ => SetSlotChildren { children: op["value"].as_array().expect("children").iter().map(|child| text(child).to_string()).collect() }.into(),
    }
}

/// 🌱️ A registered toy instance holding the fixture's seed, one edit per op, authored by the fixture actor.
#[doc(hidden)]
pub async fn seeded_app(fixture: &Value, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> ToyApp {
    seeded_app_as(fixture, text(&fixture["actor"]), mounted_policy, identity).await
}

/// 🪪️ Opens the toy under its acting actor while preserving each neutral seed edit's author.
#[doc(hidden)]
pub async fn seeded_app_as(fixture: &Value, actor: &str, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> ToyApp {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.into()), mounted_policy, identity).await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(actor.into()));
    for op in fixture["seed"].as_array().expect("seed") {
        publish_as(&mut app, text(&fixture["actor"]), seed_op(op));
    }
    app.render_ctx().refresh_cache().await.expect("the command log backfills the seed");
    app
}

#[doc(hidden)]
pub fn meta(fixture: &Value) -> ActionMeta {
    ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(text(&fixture["actor"])) }
}

/// ✏️ The mutation id of seeded edit `index` (each seeded edit holds one op).
#[doc(hidden)]
pub fn seeded_mutation(app: &ToyApp, index: usize) -> String {
    app.stores.store.mutation_ops().expect("mutation ops")[index].mutation_id.0.clone()
}

#[doc(hidden)]
pub async fn verb(app: &mut ToyApp, fixture: &Value, action: &str, args: Vec<(String, DslValue)>, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> InvocationResult {
    app.handle_action(action, Some(&DslValue::Object(args)), &meta(fixture), identity).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"))
}

#[doc(hidden)]
pub fn rejected(result: &InvocationResult) -> Option<&str> {
    result.output.get("rejected").and_then(DslValue::as_str)
}

#[doc(hidden)]
pub async fn pump_until(app: &mut ToyApp, what: &str, done: impl Fn(&ToyApp) -> bool, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) {
    assert_eq!(app.mounted_policy,mounted_policy);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return;
        }
        app.advance_typed_operation_publication(identity,cx).await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
        collect_original_history_command_receipt(app);
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("{what} never settled; stage {:?}", app.time_travel.session().stage);
}

#[doc(hidden)]
pub async fn render_body(app: &mut ToyApp) -> String {
    let tree = app.render("time-travel.main", None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.unwrap_or_else(|fault| panic!("render: {fault:?}"));
    artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error| panic!("project: {error}"))
}

#[doc(hidden)]
pub async fn render_history(app: &mut ToyApp, locale: Locale) -> Value {
    let view = ViewModel { locale, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let tree = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &view).await.unwrap_or_else(|fault| panic!("history render: {fault:?}"));
    serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(tree).expect("history projection")).expect("history projection parses")
}

#[doc(hidden)]
pub fn find_node<'a>(node: &'a Value, key: &str) -> Option<&'a Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| find_node(child, key))
}

/// ⏯️ Runs one fixture step through the real verbs (a `replay` pumps the per-turn Report replay to completion).
#[doc(hidden)]
pub async fn run_step(app: &mut ToyApp, fixture: &Value, step: &Value, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Option<InvocationResult> {
    let (name, value) = step.as_object().and_then(|step| step.iter().next()).expect("one-key step");
    Some(match name.as_str() {
        "begin" => {
            let mutation = seeded_mutation(app, value.as_u64().expect("edit index") as usize);
            verb(app, fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))], mounted_policy, identity).await
        }
        "input" => verb(app, fixture, "historyEditInput", vec![("path".into(), dsl(&value["path"])), ("value".into(), dsl(&value["value"]))], mounted_policy, identity).await,
        "withdraw" => verb(app, fixture, "historyEditWithdraw", Vec::new(), mounted_policy, identity).await,
        "withdrawRow" | "restore" => {
            let mutation = seeded_mutation(app, value.as_u64().expect("edit index") as usize);
            verb(app, fixture, if name == "restore" { "historyEditRestore" } else { "historyEditWithdraw" }, vec![("mutationId".into(), DslValue::String(mutation))], mounted_policy, identity).await
        }
        "accept" => verb(app, fixture, "historyEditAccept", Vec::new(), mounted_policy, identity).await,
        "discard" => {
            let result=verb(app, fixture, "historyEditDiscard", Vec::new(), mounted_policy, identity).await;
            if rejected(&result).is_none(){pump_until(app,"original Discard publication settles",|app| !app.time_travel.has_pending_work(), mounted_policy, identity,cx).await;}
            result
        },
        "finalize" => verb(app, fixture, "historyEditFinalize", Vec::new(), mounted_policy, identity).await,
        "exit" => verb(app, fixture, "historyEditExit", Vec::new(), mounted_policy, identity).await,
        "commit" => {
            let mut args = Vec::new();
            if let Some(name) = value["name"].as_str() {
                args.push(("name".to_string(), DslValue::String(name.to_string())));
            }
            if let Some(choice) = value["choice"].as_str() {
                args.push(("choice".to_string(), DslValue::String(choice.to_string())));
            }
            verb(app, fixture, "historyEditCommit", args, mounted_policy, identity).await
        }
        "replay" => {
            pump_until(app, "replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, identity,cx).await;
            return None;
        }
        other => panic!("unknown fixture step {other}"),
    })
}

/// 🧹️ Closes the instance and proves the history-edit ledger retired every owner.
#[doc(hidden)]
pub fn close(app: &mut ToyApp,cx:&mut semio_framework_job::StepContext<'_>) {
    collect_original_history_command_receipt(app);
    let mounted_policy=app.mounted_policy;artifact_app_laws::close_registered_fixture_app(app,mounted_policy,cx).expect("original history fixture paid terminal close");
    assert!(app.time_travel.terminal_is_empty(), "the history-edit ledger retires every owner on close");
}

/// 🏁️ Runs fixture scenario `id`, which ends in a commit, on `app`.
#[doc(hidden)]
pub async fn commit_scenario(app: &mut ToyApp, fixture: &Value, id: &str, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let scenario = fixture["scenarios"].as_array().expect("scenarios").iter().find(|scenario| text(&scenario["id"]) == id).unwrap_or_else(|| panic!("scenario {id}"));
    for step in scenario["steps"].as_array().expect("steps") {
        if let Some(result) = run_step(app, fixture, step, mounted_policy, identity,cx).await {
            assert_eq!(rejected(&result), None, "{id}: step {step} was refused");
        }
    }
    pump_until(app, "the finalize retires", |app| !app.time_travel.has_pending_work(), mounted_policy, identity,cx).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[doc(hidden)]
pub fn head(app: &ToyApp) -> (i32, String) {
    let snapshot = app.stores.store.snapshot().expect("head");
    (snapshot.count, snapshot.label.clone())
}
/// 🧱️ One long edit applied in batches that each fit the Store's fixed displaced-owner retirement authority, drained between batches.
#[doc(hidden)]
pub async fn apply_long_edit(app: &mut ToyApp, count: i32, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    const BATCH: i32 = 128;
    let mut start = 0;
    while start < count {
        let end = (start + BATCH).min(count);
        let batch: Vec<TestMutation> = (start..end).map(|value| SetCount { value }.into()).collect();
        app.stores.store.dispatch(ArtifactCommand::Apply { mutations: batch, transaction: None }, identity).await.expect("a long edit applies");
        semio_framework_plugin_testing_laws::artifact_app_laws::drain_store_displaced_retirements(&mut app.stores.store);
        start = end;
    }
}

/// 🪶️ Publishes one edit with an explicit author while preserving the opened instance actor.
#[doc(hidden)]
pub fn publish_as(app: &mut ToyApp, actor: &str, mutation: TestMutation) {
    let mut receiving_job=semio_framework_plugin_testing_laws::artifact_app_laws::FixtureOriginalJob::new(semio_framework::kernel::RetainedTurnInput{operation:71,generation:3,epoch:1,grant:app.mounted_policy.close});
    let grant = store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(app.mounted_policy.preparation);
    let mut publication = app
        .stores.store
        .begin_apply_batch(semio_framework_job::OperationId(1), app.stores.store.generation_now(), app.stores.store.content_revision_now(), actor.into(), vec![mutation], store::os_store::store_schema::HistoryLane::Document, app.stores.artifact_one_item_factory.as_ref(), None)
        .unwrap_or_else(|rejected| panic!("the edit is admitted: {}", rejected.into_owners().0));
    let published = (0..65_536).any(|_| matches!(app.stores.store.advance_apply_batch(&mut publication, grant).expect("the edit publishes"), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_)));
    assert!(published && publication.acknowledge(), "the edit is published and acknowledged");
    publication.begin_close();
    let closed = (0..4_096).any(|_|{let grant=app.mounted_policy.close;let step=publication.close_step(grant).expect("the publication closes");assert!(step.progress().fits(grant));matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_))});
    assert!(closed && publication.terminal_is_empty(), "the publication retires every owner");
}
//#endregion ⏱️WallDeadline

/// 🫵️ The author of every edit of `app`'s document, oldest first.
#[doc(hidden)]
pub fn edit_authors(app: &ToyApp) -> Vec<Option<String>> {
    app.stores.store.envelope().vcs.edits.iter().map(|edit| edit.actor.as_ref().map(|actor| actor.as_str().to_owned())).collect()
}

/// 🎬️ The row actions of a rendered mutation row: each verb, label, whether it is disabled and why.
#[doc(hidden)]
pub fn row_actions(row: &Value) -> Vec<(String, String, bool, Option<String>)> {
    let text = |value: &Value| value.as_str().map(str::to_string);
    row["component"]["rowActions"].as_array().into_iter().flatten().map(|action| (text(&action["verb"]).unwrap_or_default(), text(&action["label"]).unwrap_or_default(), action["disabled"] == Value::Bool(true), text(&action["reason"]))).collect()
}