//! ✏️ Every catalog editor declares, parses, and replays the complete editing contract.

use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack, Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_value::{DslValue, ToValue};
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
    serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot.to_value())).expect("independent snapshot oracle")
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
        let args: DslValue = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("native action argument decoder");
        let native_json = semio_framework_pack_json::to_json_string(&args);
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
        let arguments: DslValue = semio_framework_pack_json::from_json_str(&row["arguments"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let command = E::command_from_action(row["id"].as_str().unwrap(), Some(&arguments)).expect("invalid detail is still a well-formed command");
        assert!(artifact_app_laws::reduce_editor_command::<E>(&command, &base).is_err(), "{} direct command must protect schema identity", definition.id);
        assert_eq!(snapshot_json(&base), before, "{} refused direct command preserves its snapshot", definition.id);
    }
    let mut after = before.clone();
    let path = case["path"].as_str().unwrap();
    let unchanged_arguments = serde_json::json!({ "path": path, "value": before.pointer(path).expect("no-op fixture path") });
    let unchanged_arguments: DslValue = semio_framework_pack_json::from_json_str(&unchanged_arguments.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let unchanged_command = E::command_from_action("setSnapshotValue", Some(&unchanged_arguments)).unwrap();
    let unchanged_event = E::snapshot_edit_event(&unchanged_command).unwrap();
    assert!(E::snapshot_edit_is_admitted(unchanged_event, &base), "{} must admit unchanged field values", definition.id);
    let unchanged = artifact_app_laws::reduce_editor_command::<E>(&unchanged_command, &base).expect("unchanged field is a successful no-op");
    assert!(unchanged.artifact_mutations.is_empty(), "{} unchanged field must not create a history event", definition.id);
    *after.pointer_mut(path).unwrap_or_else(|| panic!("{} has no editable fixture path {path}", definition.id)) = case["value"].clone();
    assert_ne!(before, after, "{} fixture must change a real detail", definition.id);
    let arguments = serde_json::json!({ "path": path, "value": case["value"] });
    let arguments: DslValue = semio_framework_pack_json::from_json_str(&arguments.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
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
        inverses.push(mutation.inverse(&projected).expect("valid native detail inverse"));
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
    for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
        let view = semio_framework_plugin::ViewModel {
            active_mode_id: Some(definition.default_mode_id.clone()),
            active_window_kind_id: Some(details.into()),
            window_id: Some("editor-catalog-details".into()),
            focused_window_id: Some("editor-catalog-details".into()),
            window_instances: vec![semio_framework::ViewWindowInstance { id: "editor-catalog-details".into(), window_kind_id: details.into() }],
            locale,
            ..semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native)
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
}

/// 🗄️ Confirms each public exported file through SQLite independently of its native provider.
fn independent_sqlite_catalog_file(bytes: &[u8], native: &semio_framework::io_schema::ArtifactDialect, encoding: &str) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/✏️editor-catalog/🪶️sqlite/🔣️.json")).expect("neutral physical SQLite contract");
    let script = r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));try{console.log(JSON.stringify({integrity:db.query('PRAGMA integrity_check').all(),foreignKeys:db.query('PRAGMA foreign_key_check').all(),metadata:db.query('SELECT artifact_kind,standard,subset,schema_version,native_encoding FROM semio_snapshot ORDER BY id').all(),domainTables:db.query("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT IN ('semio_snapshot','sqlite_sequence') ORDER BY name").all()}));}finally{db.close();}"#;
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("independent SQLite reader");
    child.stdin.take().unwrap().write_all(bytes).expect("physical exported file sent to independent reader");
    let output = child.wait_with_output().expect("independent SQLite reader completion");
    assert!(output.status.success(), "{} independent SQLite reader: {}", native.to_coordinate(), String::from_utf8_lossy(&output.stderr));
    let actual: serde_json::Value = serde_json::from_slice(&output.stdout).expect("independent physical SQLite evidence");
    assert_eq!(actual["integrity"], law["integrity"]);
    assert_eq!(actual["foreignKeys"], law["foreignKeys"]);
    assert_eq!(actual["metadata"], serde_json::json!([{"artifact_kind":native.artifact_kind,"standard":native.standard,"subset":native.subset,"schema_version":law["schemaVersion"],"native_encoding":encoding}]));
    let tables = actual["domainTables"].as_array().unwrap();
    assert!(tables.len() >= law["minimumDomainTables"].as_u64().unwrap() as usize, "{} must expose authored domain tables", native.to_coordinate());
    eprintln!("[DEBUG] Stdio catalog physical SQLite dialect={} native={} bytes={} domain_tables={} integrity=ok foreign_keys=0", native.to_coordinate(), encoding, bytes.len(), tables.len());
}

async fn assert_sqlite_snapshot_editor<E: ArtifactEditor>() {
    use semio_framework::io::io_mechanism::{io_entries, io_identify, io_route, io_run};
    use semio_framework::io_schema::{Confidence, IoFidelity, IoPayload, SQLITE_SNAPSHOT};
    stdio_packages_assembled();
    let native: semio_framework::io_schema::ArtifactDialect = E::DIALECT.into();
    let sqlite = SQLITE_SNAPSHOT.into();
    let entries = io_entries();
    assert!(E::Snapshot::sqlite_snapshot_codec().is_some(), "every shipped artifact snapshot must declare a semantic SQLite owner: {}", native.to_coordinate());
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
        independent_sqlite_catalog_file(bytes, &native, match &payload { IoPayload::Binary(_) => "binary", IoPayload::Text(_) => "text" });
        assert_eq!(io_identify(&database).await, vec![(sqlite.clone(), Confidence::High)]);
        let restored = io_run(&import, database).await.expect("shipped editor SQLite import").value;
        if restored != payload {
            use semio_framework_os_kernel as store;
            if let (IoPayload::Binary(before),IoPayload::Binary(after))=(&payload,&restored){
                let (before_envelope,before_body)=store::semio_format::unwrap_binary(before).unwrap();
                let (after_envelope,after_body)=store::semio_format::unwrap_binary(after).unwrap();
                eprintln!("[DEBUG] catalog native seam coordinate={} envelopes_equal={} original_body={} restored_body={} first={:?}",native.to_coordinate(),before_envelope==after_envelope,before_body.len(),after_body.len(),before_body.iter().zip(&after_body).position(|(left,right)|left!=right));
                if let Some(spec)=E::Snapshot::record_spec(){
                    let original_record=pack::record::decode_document(&before_body,&spec,&pack::record::DecodeOptions::default()).unwrap().0;
                    let restored_record=pack::record::decode_document(&after_body,&spec,&pack::record::DecodeOptions::default()).unwrap().0;
                    eprintln!("[DEBUG] catalog Record equal={} original={:?} restored={:?}",original_record==restored_record,original_record,restored_record);
                }
                let decoded=E::Snapshot::decode_pack(after).unwrap();
                eprintln!("[DEBUG] catalog full typed owner equal={} coordinate={}",snapshot_json(&decoded)==original,native.to_coordinate());
            }
        }
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
