//! ✏️ Every catalog editor declares, parses, and replays the complete editing contract.

use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack, DslValue, Mutation, MutationDiff, OpBinary, OpText, ToValue};
use semio_framework_plugin::app::EditorSurfaceApp;
use semio_framework_plugin::{artifact_app_laws, AppActionRegistry, AppDefinition, ArtifactEditor, EditorApp, InteractiveJobClassification, PluginApp};
use semio_s_artifact_stdio_contract::editing::{SnapshotEditingEditor, SNAPSHOT_EDIT_ACTION_IDS};
use std::collections::BTreeSet;

/// 🧩️ Assembles every stdio package once: assembly publishes the artifact document schemas an editor's snapshot edits
/// resolve (`snapshot-edit.schema-unregistered` otherwise), exactly as a host that loaded the stdio packages has them.
fn stdio_packages_assembled() {
    static ASSEMBLED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    ASSEMBLED.get_or_init(|| {
        semio_hub_stdio::plugin().expect("stdio assembles");
        semio_hub_stdio_image::plugin().expect("stdio-image assembles");
        semio_hub_stdio_media::plugin().expect("stdio-media assembles");
        semio_hub_stdio_cad::plugin().expect("stdio-cad assembles");
        semio_hub_stdio_bim::plugin().expect("stdio-bim assembles");
        semio_hub_stdio_mesh::plugin().expect("stdio-mesh assembles");
        semio_hub_stdio_pdf::plugin().expect("stdio-pdf assembles");
        semio_hub_stdio_office::plugin().expect("stdio-office assembles");
        semio_hub_stdio_semio::plugin().expect("stdio-semio assembles");
        semio_hub_stdio_binary::plugin().expect("stdio-binary assembles");
    });
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/✏️editor-catalog/🔣️.json")).expect("neutral editor acceptance fixture")
}

fn snapshot_json<S: ToValue>(snapshot: &S) -> serde_json::Value {
    serde_json::from_str(&pack::json::to_json_string(&snapshot.to_value())).expect("independent snapshot oracle")
}

fn assert_detail_routes(root: &serde_json::Value, app_id: &str) {
    let mut frontier = vec![root];
    let mut controls = 0;
    while let Some(node) = frontier.pop() {
        for binding in node["bindings"].as_array().unwrap() {
            let action = &binding["action"];
            if SNAPSHOT_EDIT_ACTION_IDS.contains(&action["name"].as_str().unwrap()) {
                assert_eq!(action["scope"].as_str(), Some(app_id), "visible detail controls must address their exact subset editor");
                controls += 1;
            }
        }
        frontier.extend(node["children"].as_array().unwrap());
    }
    assert!(controls > 0, "{app_id} must render usable detail controls");
}

async fn assert_editor<E: ArtifactEditor + SnapshotEditingEditor>(definition: AppDefinition) {
    stdio_packages_assembled();
    let fixture = fixture();
    let details = fixture["detailsWindow"].as_str().unwrap();
    let details_kind = definition.window_kinds.iter().find(|kind| kind.id == details).unwrap_or_else(|| panic!("{} needs editable details", definition.id));
    for row in fixture["actions"].as_array().unwrap() {
        let id = row["id"].as_str().unwrap();
        let action = details_kind.actions.iter().find(|action| action.id == id).unwrap_or_else(|| panic!("{} details window is missing {id}", definition.id));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{} {id}", definition.id);
        assert!(E::Command::TOOL_JOB_IDS.contains(&id), "{} has no retained {id}", definition.id);
        let source = serde_json::to_string(&row["arguments"]).unwrap();
        let args: DslValue = pack::json::from_json_str(&source).expect("native action argument decoder");
        let native_json = pack::json::to_json_string(&args);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&native_json).unwrap(), row["arguments"], "independent argument codec oracle {id}");
        let command = E::command_from_action(id, Some(&args)).unwrap_or_else(|error| panic!("{} cannot parse {id}: {error:?}", definition.id));
        assert_eq!(E::command_id(&command), id);
        let bytes = command.encode_op().expect("editor command encoding");
        let replay = E::Command::decode_op(&bytes).expect("editor command replay");
        assert_eq!(E::command_id(&replay), id);
        assert_eq!(replay.encode_op().unwrap(), bytes, "replay preserves every argument of {id}");
    }
    let registry = AppActionRegistry::from_definition(&definition);
    let mut app = EditorSurfaceApp::<E>::with_registry(EditorApp::<E>::default(), registry).await;
    let family = definition.id.strip_prefix("s.stdio.").unwrap().split('@').next().unwrap();
    let edits = &fixture["snapshotEdits"];
    let case = edits.get(&definition.id).or_else(|| edits.get(family)).unwrap_or_else(|| panic!("{} needs a meaningful edit fixture", definition.id));
    let base = E::initial_snapshot();
    let before = snapshot_json(&base);
    let schema_identity = |schema: &str| schema.trim_start_matches("s.").to_string();
    assert_eq!(before["schema"].as_str().map(schema_identity), Some(schema_identity(E::DOCUMENT_SCHEMA)), "{} edits the document schema of its own snapshot model", definition.id);
    for row in fixture["rejectedActions"].as_array().unwrap() {
        let arguments: DslValue = pack::json::from_json_str(&row["arguments"].to_string()).unwrap();
        let command = E::command_from_action(row["id"].as_str().unwrap(), Some(&arguments)).expect("invalid detail is still a well-formed command");
        assert!(artifact_app_laws::reduce_editor_command::<E>(&command, &base).is_err(), "{} direct command must protect schema identity", definition.id);
        assert_eq!(snapshot_json(&base), before, "{} refused direct command preserves its snapshot", definition.id);
    }
    let mut after = before.clone();
    let path = case["path"].as_str().unwrap();
    let unchanged_arguments = serde_json::json!({ "path": path, "value": before.pointer(path).expect("no-op fixture path") });
    let unchanged_arguments: DslValue = pack::json::from_json_str(&unchanged_arguments.to_string()).unwrap();
    let unchanged_command = E::command_from_action("setSnapshotValue", Some(&unchanged_arguments)).unwrap();
    let unchanged_event = E::snapshot_edit_event(&unchanged_command).unwrap();
    assert!(E::snapshot_edit_is_admitted(unchanged_event, &base), "{} must admit unchanged field values", definition.id);
    let unchanged = artifact_app_laws::reduce_editor_command::<E>(&unchanged_command, &base).expect("unchanged field is a successful no-op");
    assert!(unchanged.artifact_mutations.is_empty(), "{} unchanged field must not create a history event", definition.id);
    *after.pointer_mut(path).unwrap_or_else(|| panic!("{} has no editable fixture path {path}", definition.id)) = case["value"].clone();
    assert_ne!(before, after, "{} fixture must change a real detail", definition.id);
    let arguments = serde_json::json!({ "path": path, "value": case["value"] });
    let arguments: DslValue = pack::json::from_json_str(&arguments.to_string()).unwrap();
    let command = E::command_from_action("setSnapshotValue", Some(&arguments)).expect("typed detail command");
    let event = E::snapshot_edit_event(&command).expect("detail event is routed");
    assert!(E::snapshot_edit_is_admitted(event, &base), "{} initial document must admit a detail edit", definition.id);
    let emit = artifact_app_laws::reduce_editor_command::<E>(&command, &base).expect("direct detail reducer accepts valid fixture");
    assert!(!emit.artifact_mutations.is_empty(), "{} must publish an artifact event", definition.id);
    let mut projected = base.clone();
    let mut inverses = Vec::new();
    for mutation in emit.artifact_mutations {
        let binary = mutation.encode_op().expect("native mutation binary encoding");
        assert_eq!(E::Mutation::decode_op(&binary).expect("native mutation binary replay").encode_op().unwrap(), binary);
        assert_eq!(E::Mutation::parse_op(&mutation.print_op()).expect("native mutation text replay").encode_op().unwrap(), binary);
        inverses.push(mutation.inverse(&projected));
        projected = mutation.diff(&projected).diff().apply(&projected).expect("native detail diff applies");
    }
    assert_eq!(snapshot_json(&projected), after, "{} native event must preserve every other detail", definition.id);
    for mutations in inverses.into_iter().rev() {
        for mutation in mutations {
            projected = mutation.diff(&projected).diff().apply(&projected).expect("native inverse applies");
        }
    }
    assert_eq!(snapshot_json(&projected), before, "{} inverse restores the complete snapshot", definition.id);
    app.bind_instance_id(artifact_app_laws::meta("local").instance_id).await;
    for locale in [semio_framework_plugin::Locale::En, semio_framework_plugin::Locale::De] {
        let view = semio_framework_plugin::ViewModel {
            active_mode_id: Some(definition.default_mode_id.clone()),
            active_window_kind_id: Some(details.into()),
            window_id: Some("editor-catalog-details".into()),
            focused_window_id: Some("editor-catalog-details".into()),
            window_instances: vec![semio_framework::ViewWindowInstance { id: "editor-catalog-details".into(), window_kind_id: details.into() }],
            locale,
            ..Default::default()
        };
        let tree = app.render(semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY, None, &view).await.expect("real localized details render");
        let projection = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("details have valid bounded UI structure");
        assert_detail_routes(&serde_json::from_str(&projection).unwrap(), &definition.id);
    }
    let arguments = match arguments { DslValue::Object(entries) => entries.into_iter().collect(), _ => unreachable!() };
    let invocation = semio_framework::manifest::ActionInvocation {
        address: semio_framework::manifest::ActionAddress {
            plugin_id: "stdio".into(),
            app_id: definition.id.clone(),
            mode_id: definition.default_mode_id.clone(),
            window_kind_id: details.into(),
            window_instance_id: "editor-catalog-details".into(),
            action_id: "setSnapshotValue".into(),
        },
        arguments,
    };
    app.handle_action_invocation(&invocation, Some(&definition.default_mode_id), &artifact_app_laws::meta("local")).await.expect("host-addressed detail action");
    artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.expect("detail action publishes");
    assert_eq!(snapshot_json(&app.snapshot().unwrap()), after, "{} addressed action changes persisted details", definition.id);
    artifact_app_laws::settle_history_verb(&mut app, "undo", 1).await;
    assert_eq!(snapshot_json(&app.snapshot().unwrap()), before, "{} undo restores every detail", definition.id);
    artifact_app_laws::settle_history_verb(&mut app, "redo", 1).await;
    assert_eq!(snapshot_json(&app.snapshot().unwrap()), after, "{} redo restores the complete edit", definition.id);
    let saved = app.snapshot().unwrap().encode_pack();
    let reopened = E::Snapshot::decode_pack(&saved).expect("saved artifact reopens");
    assert_eq!(snapshot_json(&reopened), after, "{} saved artifact preserves every edited detail", definition.id);
    let source = semio_s_artifact_stdio_contract::editing::snapshot_edit_source(&reopened);
    let source_reopened = semio_s_artifact_stdio_contract::editing::snapshot_from_edit_source::<E::Snapshot>(&source).expect("complete editable source reopens");
    assert_eq!(snapshot_json(&source_reopened), after, "{} editable source preserves every saved detail", definition.id);
    for _ in 0..65_536 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(1, semio_framework_os_kernel::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("registered editor closes retained owners");
    }
    assert!(app.close_terminal_is_empty(), "{} closes every retained factory", definition.id);
}

#[test]
fn neutral_catalog_requires_each_edit_operation_once() {
    let fixture = fixture();
    let expected = SNAPSHOT_EDIT_ACTION_IDS.iter().copied().collect::<BTreeSet<_>>();
    let actual = fixture["actions"].as_array().unwrap().iter().map(|row| row["id"].as_str().unwrap()).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), fixture["actions"].as_array().unwrap().len());
    assert_eq!(fixture["editorCount"].as_u64().unwrap() as usize, EDITOR_COUNT);
    eprintln!("[DEBUG] retained Stdio editor pairs admitted: {EDITOR_COUNT}");
}

async fn assert_sqlite_snapshot_editor<E: ArtifactEditor>() {
    use semio_framework::io::io_mechanism::{io_entries, io_identify, io_route, io_run};
    use semio_framework::io_schema::{Confidence, IoFidelity, IoPayload, SQLITE_SNAPSHOT};
    stdio_packages_assembled();
    let native = E::DIALECT.into();
    let sqlite = SQLITE_SNAPSHOT.into();
    let entries = io_entries();
    if E::Snapshot::sqlite_snapshot_codec().is_none() {
        assert!(!entries.iter().any(|entry| (entry.from == native && entry.into == sqlite) || (entry.from == sqlite && entry.into == native)), "undeclared relational capability must publish no SQLite route");
        return;
    }
    assert!(entries.iter().any(|entry| entry.from == native && entry.into == sqlite && entry.fidelity == IoFidelity::Exact));
    assert!(entries.iter().any(|entry| entry.from == sqlite && entry.into == native && entry.fidelity == IoFidelity::Exact));
    let export = io_route(&native, &sqlite, 1).await.expect("shipped editor SQLite export route").value;
    let import = io_route(&sqlite, &native, 1).await.expect("shipped editor SQLite import route").value;
    let snapshot = E::initial_snapshot();
    let original = snapshot_json(&snapshot);
    for payload in [IoPayload::Binary(snapshot.encode_pack()), IoPayload::Text(snapshot.print_dsl())] {
        let database = io_run(&export, payload.clone()).await.expect("shipped editor SQLite export").value;
        let IoPayload::Binary(bytes) = &database else { panic!("SQLite snapshot file is binary") };
        assert!(bytes.starts_with(b"SQLite format 3\0"), "snapshot export must be a SQLite file");
        assert_eq!(io_identify(&database).await, vec![(sqlite.clone(), Confidence::High)]);
        let restored = io_run(&import, database).await.expect("shipped editor SQLite import").value;
        assert_eq!(restored, payload);
        let decoded = match restored {
            IoPayload::Binary(bytes) => E::Snapshot::decode_pack(&bytes).expect("restored native pack"),
            IoPayload::Text(text) => E::Snapshot::parse_dsl(&text).expect("restored native DSL"),
        };
        assert_eq!(snapshot_json(&decoded), original, "independent snapshot oracle for {}", native.to_coordinate());
    }
}

macro_rules! editor_catalog_laws {
    ($(($name:ident, $sqlite_name:ident, $editor:ty, $definition:path)),+ $(,)?) => {
        const EDITOR_COUNT: usize = [$(stringify!($editor)),+].len();
        $(#[semio_framework_async_macros::async_test] async fn $name() { assert_editor::<$editor>($definition()).await; })+
        $(#[semio_framework_async_macros::async_test] async fn $sqlite_name() { assert_sqlite_snapshot_editor::<$editor>().await; })+
    };
}

include!("../../🤖️generated/🧪️editor-laws/🦀️.rs");
