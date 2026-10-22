use super::*;
use semio_framework_plugin::{ActionMeta, PluginApp, VcsArtifactApp};
use semio_framework_ui_locale::{AppLabels, Locale, Terminology};

#[semio_framework_async_macros::async_test]
async fn embedded_parameters_use_the_parent_input_and_exact_window_without_mutating_the_document() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧩️embedded.json")).expect("independent embedded input fixture");
    let mut app = new_app().await;
    let before = to_json_string(&app.snapshot().expect("initial payload"));
    for input in fixture["cases"].as_array().expect("embedded inputs") {
        let view = ViewModel { extension_input_json: Some(serde_json::json!({ "bodyKey": "params", "paramsJson": input.to_string() }).to_string()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        let projection = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, BODY_PARAMS, &view, app.1.cx()).await.expect("retire params tree");
        let tree: serde_json::Value = serde_json::from_str(&projection).expect("independent projection oracle");
        let children = tree["children"].as_array().expect("parameters");
        let height = &children[0]["children"][0];
        assert_eq!(height["component"]["value"].as_f64(), input["params"]["height"].as_f64());
        assert_eq!(height["bindings"][0]["action"]["scope"], input["controllerId"]);
        assert_eq!(height["bindings"][0]["args"]["key"], input["questionId"]);
        assert_eq!(height["bindings"][0]["args"]["windowId"], input["windowId"]);
        assert_eq!(height["bindings"][0]["args"]["windowKindId"], input["windowKindId"]);
        assert_eq!(children.len(), 3, "embedded parameter controls expose only parent-owned actions");
        assert_eq!(to_json_string(&app.snapshot().expect("unmodified payload")), before);
    }
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[test]
fn procedural_payload_vectors_match_the_json_oracle() {
    use protocol::{ColdMutation, Mutation, OpBinary, OpText};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔁️payload-mutations.json")).expect("independent JSON parser");
    let base: ModuleRenderPayload = semio_framework_pack_json::from_json_str(&fixture["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned base");
    assert_eq!(ModulePayloadMutation::DESCRIPTORS.len(), 1);
    const WITNESSES: [(&str, &str); 2] = [
        ("🎚️sets-params", include_str!("../🧫️fixtures/🧬️mutations/📦️set-payload/🎚️sets-params/🦠️mutation/🔣️.json")),
        ("🫙️clears-params", include_str!("../🧫️fixtures/🧬️mutations/📦️set-payload/🫙️clears-params/🦠️mutation/🔣️.json")),
    ];
    for row in fixture["cases"].as_array().expect("mutation vectors") {
        let witness = WITNESSES.iter().find(|(case, _)| row["witness"] == *case).map(|(_, text)| *text).expect("case names a committed wire witness");
        let mutation: ModulePayloadMutation = store::os_store::test_support::assert_wire_witness(witness);
        let post = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).expect("apply mutation");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&post)).expect("independent state oracle"), row["expected"]);
        assert_eq!(ModulePayloadMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
        assert_eq!(ModulePayloadMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("decode binary"), mutation);
        let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture").iter().fold(post, |current, inverse| protocol::apply_diff_cold(inverse.diff(&current).diff(), &current).expect("inverse"));
        assert_eq!(restored, base);
    }
}

#[semio_framework_async_macros::async_test]
async fn procedural_actor_descriptor_matches_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛂️actor.json")).expect("independent actor fixture");
    __semio_install_plugin_bundle();
    let manifest = __SEMIO_PLUGIN_RUNTIME.with(|runtime| ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::plugin_runtime::plugin_manifest(runtime)));
    assert_eq!(manifest.plugin_id, MODULE_PLUGIN_ID, "bundle assembly: {}", manifest.label);
    let bytes = __SEMIO_PLUGIN_RUNTIME.with(|runtime| ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::describe::describe_extension_with_apps(runtime)));
    semio_framework_plugin::plugin_runtime::extension_dispose_cold().expect("cold actor inspection retires its installed extension");
    assert!(semio_framework_plugin::plugin_runtime::extension_terminal_is_empty());
    let value = store::pack_rt::decode_wire_value(&bytes).expect("first-party wire decoder");
    let descriptor: serde_json::Value = value.into();
    assert_eq!(descriptor["role"], fixture["role"]);
    assert_eq!(descriptor["manifest"]["pluginId"], fixture["pluginId"]);
    assert_eq!(descriptor["manifest"]["apps"][0]["id"], fixture["appId"]);
    assert_eq!(descriptor["manifest"]["dependencies"][0]["pluginId"], fixture["host"]);
    let topic = &descriptor["contributions"]["topicContributions"][0];
    assert_eq!(topic["topic"], fixture["topic"]);
    assert_eq!(topic["payload"]["appId"], fixture["appId"]);
    assert_eq!(topic["payload"]["blockKind"], fixture["blockKind"]);
    let forms_topic = descriptor["contributions"]["topicContributions"].as_array().expect("topics").iter().find(|topic| topic["topic"] == fixture["formsTopic"]).expect("Forms question provider");
    assert_eq!(forms_topic["payload"]["questionKind"], fixture["blockKind"]);
    assert_eq!(forms_topic["payload"]["appId"], fixture["appId"]);
    assert_eq!(forms_topic["payload"]["label"], fixture["formsLabels"]);
    let bodies: Vec<&serde_json::Value> = descriptor["manifest"]["apps"][0]["windowKinds"].as_array().expect("window kinds").iter().map(|window| &window["bodyKey"]).collect();
    assert_eq!(bodies, fixture["windowBodies"].as_array().expect("expected window bodies").iter().collect::<Vec<_>>());
}

#[test]
fn procedural_parameter_controls_match_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎚️controls.json")).expect("independent control vectors");
    for row in fixture["cases"].as_array().expect("controls") {
        let question: PlaybookBlock = semio_framework_pack_json::from_json_str(&row["question"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned block decoder");
        let independent: PlaybookBlock = serde_json::from_value(row["question"].clone()).expect("independent block decoder");
        assert_eq!(question, independent);
        let value = parse_json(&row["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned value");
        let payload = ModuleRenderPayload {
            question_id: "parent".into(),
            controller_id: "s.forms.forms@1/*#editor".into(),
            surface: row["surface"].as_str().expect("surface").into(),
            interactive: row["interactive"].as_bool().expect("interactive"),
            ..default_payload()
        };
        let node = render_question_control(&question, &value, &payload, None).expect("semantic control");
        let control = node.children.get(0).expect("field control");
        let actual = serde_json::to_value(control).expect("independent semantic JSON oracle");
        assert_eq!(actual["component"]["type"], row["component"]);
        assert_eq!(actual["accessibility"]["label"], row["question"]["label"]);
        assert_eq!(control.disabled, !payload.interactive);
        assert_eq!(actual["disabled"].as_bool().unwrap_or(false), !payload.interactive);
        assert_eq!(actual["bindings"][0]["trigger"], "change");
        assert_eq!(actual["bindings"][0]["action"]["scope"], payload.controller_id);
        assert_eq!(actual["bindings"][0]["action"]["name"], if payload.surface == "blueprint" { "patchQuestions" } else { "setTryValue" });
        assert_eq!(actual["bindings"][0]["args"]["questionIds"], serde_json::json!(["parent"]));
        assert_eq!(actual["bindings"][0]["args"]["paramKey"], question.id);
        if let Some(expected) = row.get("expectedValue") {
            assert_eq!(&actual["component"]["value"], expected);
        }
        artifact_app_laws::project_and_retire_fixture_tree(built_to_component_tree(node)).expect("retire control tree");
    }
}

fn meta() -> ActionMeta {
    artifact_app_laws::meta("local")
}

/// 🔗️ Registered AND bound: a migrated tool command is refused with `interactive-job.live-instance`
/// unless the mounted live instance id equals the dispatching `ActionMeta`'s.
/// 🧹️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
struct ModuleTestApp(VcsArtifactApp<ModuleApp>, semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

impl std::ops::Deref for ModuleTestApp {
    type Target = VcsArtifactApp<ModuleApp>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ModuleTestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

async fn new_app() -> ModuleTestApp {
    let definition = create_module_app().await.expect("module definition").definition;
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = VcsArtifactApp::with_registry(ModuleApp, AppActionRegistry::from_definition(&definition), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    semio_framework_plugin::artifact_app_laws::settle_store_constructor(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
    app.bind_instance_id(meta().instance_id).await;
    ModuleTestApp(app, scope)
}

/// 🔁️ Admits one migrated action and settles its publication — the admission receipt alone
/// publishes nothing.
async fn act(app: &mut ModuleTestApp, action: &str, args: Option<&DslValue>) {
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, args.cloned(), &meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta().instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));
}

fn payload_json(params: Value) -> String {
    to_json_string(&ModuleRenderPayload { example_id: "hexagonal-mushroom-column".into(), params: json_to_dsl_value(&params), question_id: "q".into(), controller_id: "forms-play".into(), surface: "try".into(), interactive: true })
}

#[semio_framework_async_macros::async_test]
async fn module_app_declares_window_kinds() {
    let app = create_module_app().await.expect("MODULE_APP_ID must be a canonical surface id");
    assert_eq!(app.definition.window_kinds.len(), 2);
    assert_eq!(app.definition.window_kinds[0].id, MODULE_WINDOW_PARAMS);
    assert_eq!(app.definition.window_kinds[0].body_key, BODY_PARAMS);
    assert_eq!(app.definition.window_kinds[1].id, MODULE_WINDOW_PREVIEW);
    assert_eq!(app.definition.window_kinds[1].body_key, BODY_PREVIEW);
}

#[semio_framework_async_macros::async_test]
async fn module_manifest_contributes_building_component() {
    let mut original_caller=semio_framework_plugin::app::OriginalFixtureCaller::admit().expect("original extension fixture caller");let mut cx=original_caller.context();
    let bundle = module_extension_bundle();
    let manifest = bundle.into_manifest_cold(&mut cx).unwrap();
    assert_eq!(manifest.topic_contributions.len(), 2);
    let topic = &manifest.topic_contributions[0];
    assert_eq!(topic.topic, "playbook.blockKind");
    assert!(topic.payload.as_object().is_some());
    assert_eq!(topic.payload.get("blockKind").and_then(DslValue::as_str), Some("buildingComponent"));
    assert_eq!(topic.payload.get("paramsBodyKey").and_then(DslValue::as_str), Some(BODY_PARAMS));
    assert_eq!(topic.payload.get("previewBodyKey").and_then(DslValue::as_str), Some(BODY_PREVIEW));

    let fuel=cx.fuel_remaining();drop(cx);original_caller.retire(fuel).expect("original extension fixture source close");
}

#[semio_framework_async_macros::async_test]
async fn preview_body_emits_world_scene() {
    let mut app = new_app().await;
    let document = payload_json(semio_framework_pack_json::json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }));
    let json = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app_over(&mut app.0, BODY_PREVIEW, Some(&document), &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("preview projection");
    assert!(json.contains("world-3d"));
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn params_body_lists_flow_inputs() {
    let mut app = new_app().await;
    let json = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, BODY_PARAMS, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("params projection");
    let tree: serde_json::Value = serde_json::from_str(&json).expect("independent JSON parser");
    let fields = tree["children"].as_array().expect("column children").iter().filter(|child| child["component"]["role"] == "field").map(|child| child["component"]["label"].as_str().expect("field label")).collect::<Vec<_>>();
    assert_eq!(fields, ["Column Height", "Profile Radius", "Side Count"], "every flow input of the default fixture is a labelled field");
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn params_body_includes_media_export_buttons() {
    let mut app = new_app().await;
    let json = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, BODY_PARAMS, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("params projection");
    let tree: serde_json::Value = serde_json::from_str(&json).expect("independent JSON parser");
    let button_count = tree["children"].as_array().expect("column children").iter().filter(|child| child["component"]["type"] == "button").count();
    assert_eq!(button_count, SOLID_MEDIA_FORMATS.len() * 2);
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn export_solid_action_stashes_result_and_is_undoable() {
    let mut app = new_app().await;
    assert!(app.snapshot().expect("projection").params.get("__solidExport").is_none());
    let export_args = json_to_dsl_value(&semio_framework_pack_json::json!({ "format": "obj" }));
    act(&mut app, ACTION_EXPORT_SOLID, Some(&export_args)).await;
    assert!(app.snapshot().expect("projection").params.get("__solidExport").is_some(), "export result stashed on params via the SetPayload operation");
    artifact_app_laws::settle_history_verb(&mut app.0, "undo", meta().instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert!(app.snapshot().expect("projection").params.get("__solidExport").is_none(), "undo restores the pre-operation payload");
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn import_solid_action_stashes_result_on_params() {
    let mut app = new_app().await;
    let import_args = json_to_dsl_value(&semio_framework_pack_json::json!({ "format": "obj", "data": "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n" }));
    act(&mut app, ACTION_IMPORT_SOLID, Some(&import_args)).await;
    assert!(app.snapshot().expect("projection").params.get("__solidImport").is_some(), "import result stashed on params via the SetPayload operation");
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn import_solid_action_reports_error_when_no_data_given() {
    let mut app = new_app().await;
    let import_args = json_to_dsl_value(&semio_framework_pack_json::json!({ "format": "obj" }));
    act(&mut app, ACTION_IMPORT_SOLID, Some(&import_args)).await;
    let payload = app.snapshot().expect("projection");
    let import = payload.params.get("__solidImport").expect("import result present");
    assert!(import.get("error").is_some());
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn export_solid_declares_only_format_arg_and_materializes_default() {
    use semio_framework_plugin::app::AppActionRegistry;
    let definition = create_module_app().await.expect("MODULE_APP_ID must be a canonical surface id").definition;
    let declared = || definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()));
    let import = declared().find(|action| action.id == ACTION_IMPORT_SOLID).expect("import declared");
    assert!(import.args.iter().all(|arg| arg.id == "format"), "only `format` is a user-facing arg; `data` is file-callback populated");
    let export = declared().find(|action| action.id == ACTION_EXPORT_SOLID).expect("export declared");
    assert_eq!(export.args.len(), 1, "export exposes exactly the format choice");
    let registry = AppActionRegistry::from_definition(&definition);
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = VcsArtifactApp::with_registry(ModuleApp, registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    semio_framework_plugin::artifact_app_laws::settle_store_constructor(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
    app.bind_instance_id(meta().instance_id).await;
    let mut app = ModuleTestApp(app, scope);
    act(&mut app, ACTION_EXPORT_SOLID, None).await;
    assert!(app.snapshot().expect("projection").params.get("__solidExport").is_some(), "export result stashed under the materialized format");
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn unknown_action_yields_no_document_change() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("projection");
    assert!(semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "noSuchAction", None, &meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.is_err(), "an undeclared action is rejected rather than silently ignored");
    assert_eq!(app.snapshot().expect("snapshot"), before);
    artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn module_labels_resolve_native_english_by_default() {
    let labels = ModuleLabels::labels(Locale::En, Terminology::Native);
    assert_eq!(labels.no_flow_inputs.as_str(), "No flow inputs.");
    assert_eq!(labels.no_procedural_parameters.as_str(), "No procedural parameters.");
    let node = text_node(labels.no_procedural_parameters.as_str()).expect("label node");
    let rendered = artifact_app_laws::project_and_retire_fixture_tree(built_to_component_tree(node)).expect("label projection");
    assert!(rendered.contains("No procedural parameters."));
}

#[semio_framework_async_macros::async_test]
async fn module_labels_resolve_german_locale() {
    let labels = ModuleLabels::labels(Locale::De, Terminology::Native);
    assert_eq!(labels.no_flow_inputs.as_str(), "Keine Flow-Eingaben.");
    assert_eq!(labels.no_procedural_parameters.as_str(), "Keine prozeduralen Parameter.");
    let node = text_node(labels.no_procedural_parameters.as_str()).expect("label node");
    let rendered = artifact_app_laws::project_and_retire_fixture_tree(built_to_component_tree(node)).expect("label projection");
    assert!(rendered.contains("Keine prozeduralen Parameter."));
    assert!(!rendered.contains("No procedural parameters."));
}

//#region 🔖️DslAndOpText
#[semio_framework_async_macros::async_test]
async fn module_render_payload_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&default_payload());
    store::os_store::test_support::assert_dsl_pack_equivalence(&default_payload());
}

/// 🪞️ `serde` dropped from this crate's own types entirely (playbook fan-out, ticket
/// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS), so there is no
/// serde-oracle left to compare against here — this guards the surviving property, the
/// `ToValue`/`FromValue` codec's own round trip, matching the process batch's precedent for
/// the identical situation (`📓️serde-fanout-fem-process.md`'s `workshop_machines_round_trip_
/// through_the_first_party_json_bridge`).
#[semio_framework_async_macros::async_test]
async fn module_payload_value_codec_round_trips() {
    use semio_framework_value::{FromValue, ToValue};

    let payload = default_payload();
    let mutation = ModulePayloadMutation::SetPayload(SetPayload { payload: payload.clone() });
    let diff = ModulePayloadDiff { surface: Some("try".into()), interactive: Some(false), ..ModulePayloadDiff::default() };

    assert_eq!(ModuleRenderPayload::from_value(payload.to_value()).expect("first-party payload"), payload);
    assert_eq!(ModulePayloadMutation::from_value(mutation.to_value()).expect("first-party mutation"), mutation);
    assert_eq!(ModulePayloadDiff::from_value(diff.to_value()).expect("first-party diff"), diff);
}

#[semio_framework_async_macros::async_test]
async fn module_payload_operation_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&ModulePayloadMutation::SetPayload(SetPayload { payload: default_payload() }));
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `ModulePayloadMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside
/// this file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`). Dispatches through a standalone
/// `store::ArtifactStore` directly (this app has no separate dsl/pack/protocol crate split, so
/// there is no existing whole-store test to extend).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactCommand, ArtifactStore};

    let mut store: ArtifactStore<ModuleRenderPayload, ModulePayloadMutation> =
        ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(create_document_envelope(MODULE_DOCUMENT_SCHEMA, "playbook-module-procedural-test", default_payload(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<ModuleRenderPayload, ModulePayloadMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    let mut payload = default_payload();
    payload.interactive = false;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![ModulePayloadMutation::SetPayload(SetPayload { payload })], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    let edit: &Edit<ModulePayloadMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<ModuleRenderPayload, ModulePayloadMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    store.close_owned_unscheduled().expect("module store closes through its exact bounded owners");
}
//#endregion 🔖️CommandEnvelopeTests
//#endregion 🔖️DslAndOpText

fn close_turn(owner: &mut ModuleGeometryOwner, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> PluginLifecycleStep {
    let mut sequence = 0;
    let mut recipient = semio_framework_value::retained_clone::RetainedCloneProgress::default();
    let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut recipient);
    owner.close_step(&mut cx, grant).unwrap()
}

fn quoted_close_grant(owner: &ModuleGeometryOwner, copy_bytes: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    let demand = owner.retirement_demands(copy_bytes).expect("instance geometry quotes its next close turn");
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

#[test]
fn instance_geometry_admits_session_shell_before_terminal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🌐️geometry-lifetime/🔣️.json")).unwrap();
    let admission = &fixture["shellAdmission"];
    let complete_bytes = admission["completionByteGrant"].as_u64().unwrap() as usize;
    let mut owner = ModuleGeometryOwner::new();
    for _ in 0..1_000_000 {
        let grant = quoted_close_grant(&owner, complete_bytes);
        let step = close_turn(&mut owner, grant);
        assert!(step.progress().is_none_or(|progress| progress.fits(grant)));
        if matches!(step, PluginLifecycleStep::Complete(_)) || (owner.registry.is_none() && owner.registry_retirement.terminal_is_empty() && Session::terminal_is_empty(&owner.session)) { break; }
    }
    assert!(Session::terminal_is_empty(&owner.session));
    assert_eq!(!owner.terminal_is_empty(), admission["retainedAfterNativeDrain"].as_bool().unwrap());
    if !owner.terminal_is_empty() {
        let full = quoted_close_grant(&owner, complete_bytes);
        let tiny = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_release_bytes: 0, ..full };
        assert!(matches!(close_turn(&mut owner, tiny), PluginLifecycleStep::Progress(progress) if progress == Default::default()));
        assert_eq!(!owner.terminal_is_empty(), admission["retainedOnTinyByteGrant"].as_bool().unwrap());
        let PluginLifecycleStep::Progress(progress) = close_turn(&mut owner, full) else { panic!("granted instance shell retirement reports physical release"); };
        assert_eq!(progress.copied_items, 1);
        assert_eq!(progress.released_bytes > 0 && owner.terminal_is_empty(), admission["releasedByCompletionByteGrant"].as_bool().unwrap());
    }
    assert!(matches!(close_turn(&mut owner, Default::default()), PluginLifecycleStep::Complete(_)));
}

#[semio_framework_async_macros::async_test]
async fn instance_geometry_replays_durable_sources_and_preserves_preview_authority() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🌐️geometry-lifetime/🔣️.json")).expect("independent lifetime fixture");
    let own_fixture = parse_json(include_str!("../🧫️fixtures/🌐️geometry-lifetime/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("first-party lifetime fixture");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&json_to_string(&own_fixture)).expect("independent fixture output"), fixture);
    for row in fixture["handles"].as_array().expect("canonical handle vectors") {
        assert_eq!(is_brep_geometry_handle(row["value"].as_str().unwrap()), row["valid"].as_bool().unwrap());
    }
    let mut first = ModuleGeometryOwner::new();
    let mut second = ModuleGeometryOwner::new();
    for source in fixture["imports"].as_array().expect("durable imports") {
        let mut payload = default_payload();
        handle_import_solid(&mut first, &mut payload, source["format"].as_str().unwrap(), source["data"].as_str().unwrap());
        let stored = params_as_json(&payload.params);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&json_to_string(stored.get("__solidImport").expect("source"))).unwrap(), *source);
        let handles = imported_geometry_handles(&mut first, &stored).expect("first authority");
        assert!(!handles.is_empty());
        assert!(second.session.export_solid_json(&handles, "obj", 0.1).contains("error"));
        let replayed = imported_geometry_handles(&mut second, &stored).expect("reopened source");
        let original: serde_json::Value = serde_json::from_str(&first.session.export_solid_json(&handles, "obj", 0.1)).unwrap();
        let reopened: serde_json::Value = serde_json::from_str(&second.session.export_solid_json(&replayed, "obj", 0.1)).unwrap();
        assert!(original.get("error").is_none() && reopened.get("error").is_none());
        let original_data = original["data"].as_str().filter(|data| !data.is_empty()).expect("original exported geometry");
        let reopened_data = reopened["data"].as_str().filter(|data| !data.is_empty()).expect("reopened exported geometry");
        assert_eq!(original_data, reopened_data, "durable replay preserves geometry rather than ephemeral handles");
        assert_eq!(imported_geometry_handles(&mut second, &stored).unwrap(), replayed, "same live source reuses the instance cache");
        let before = json_to_string(&params_as_json(&payload.params));
        handle_export_solid(&mut second, &mut payload, "obj");
        assert!(!params_as_json(&payload.params).get("__solidExport").unwrap().get("error").is_some());
        assert_eq!(json_to_string(&stored), before);
    }
    let snapshot: FlowHostSnapshot = semio_framework_pack_json::from_json_str(HEX_COLUMN_LEARNING_PROGRAM_JSON, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for params in fixture["parameters"].as_array().expect("preview variants") {
        let params = parse_json(&params.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let (meshes, instances) = evaluated_preview_payload(&mut first, &snapshot, &params);
        let meshes: serde_json::Value = serde_json::from_str(&meshes).unwrap();
        let instances: serde_json::Value = serde_json::from_str(&instances).unwrap();
        assert!(!meshes.as_array().unwrap().is_empty());
        assert!(meshes.as_array().unwrap().iter().all(|mesh| mesh["id"].as_str().unwrap().starts_with("eval-")));
        for mesh in meshes.as_array().unwrap() {
            let positions = mesh["data"]["positions"].as_array().expect("evaluated mesh coordinates");
            assert!(!positions.is_empty() && positions.len() % 3 == 0);
            let mut minimum_z = f64::INFINITY;
            let mut maximum_z = f64::NEG_INFINITY;
            let mut maximum_radius = 0.0_f64;
            for vertex in positions.chunks_exact(3) {
                let [x, y, z] = std::array::from_fn(|axis| vertex[axis].as_f64().expect("independent numeric coordinates"));
                minimum_z = minimum_z.min(z);
                maximum_z = maximum_z.max(z);
                maximum_radius = maximum_radius.max(x.hypot(y));
            }
            assert!((maximum_z - minimum_z - params.get("height").unwrap().as_f64().unwrap()).abs() < 1e-5);
            assert!((maximum_radius - params.get("radius").unwrap().as_f64().unwrap()).abs() < 1e-5);
        }
        assert!(!instances.as_array().unwrap().is_empty());
    }
    snapshot.retire_cold();
    for owner in [&mut first, &mut second] {
        let paused = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..quoted_close_grant(owner, 65_536) };
        assert!(matches!(close_turn(owner, paused), PluginLifecycleStep::Progress(progress) if progress == Default::default()));
        for _ in 0..1_000_000 {
            let grant = quoted_close_grant(owner, 65_536);
            if matches!(close_turn(owner, grant), PluginLifecycleStep::Complete(_)) { break; }
        }
        assert!(owner.terminal_is_empty(), "instance geometry retirement completes");
    }
}
