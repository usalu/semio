use crate::editor::zip::iso21320::modes::edit::windows::main;
use crate::{ZipMutation, ZipSnapshot};
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use crate::apply_mutation;
use crate::editor::zip::iso21320::*;

#[semio_framework_async_macros::async_test]
async fn create_zip_iso21320_editor_builds_a_definition_for_the_editor_role() {
    let def = create_zip_iso21320_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, ZIP_ISO21320_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ZipIso21320Editor as ArtifactEditor>::DIALECT, ZIP_ISO21320_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_zip_iso21320_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn editor_mounts_the_bounded_zip_preparation_route() {
    use semio_s_artifact_stdio_contract::editing::BoundedNativeEditingEditor;

    assert!(ZipIso21320Editor::native_edit_preparation_route("stdio-zip-iso21320-snapshot-edit").is_some());
}

semio_plugin::history_edit_acceptance_law!("stdio", crate::editor::zip::iso21320::ZipIso21320Editor, || semio_plugin::App { definition: crate::editor::zip::iso21320::create_zip_iso21320_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_member() {
    
    use crate::schema::snapshot::ZipEntry;
    let base = ZipSnapshot { entries: vec![ZipEntry { name: "a.txt".into(), data: vec![1], ..ZipEntry::default() }, ZipEntry { name: "b.txt".into(), data: vec![2], ..ZipEntry::default() }], ..ZipSnapshot::default() };
    let emit = |event: semio_s_artifact_stdio_contract::editing::SnapshotEditEvent| <ZipIso21320Editor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let renamed = emit(semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/entries/1/name".into(), value: semio_framework_value::DslValue::String("c.txt".into()) }).expect("a rename resolves");
    let [mutation @ ZipMutation::RenameEntry(_)] = renamed.artifact_mutations.as_slice() else { panic!("a name edit raises the rename kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.entries[1].name, "c.txt");
    let removed = emit(semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::RemoveValue { path: "/entries/0".into() }).expect("a member removal resolves");
    assert!(matches!(removed.artifact_mutations.as_slice(), [ZipMutation::RemoveEntry(_)]));
    let added = emit(semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::InsertValue { path: "/entries/1".into(), value: semio_framework_value::ToValue::to_value(&ZipEntry { name: "m.txt".into(), ..ZipEntry::default() }) }).expect("a member insertion resolves");
    let mut state = base.clone();
    added.artifact_mutations.iter().for_each(|mutation| {
        apply_mutation(&mut state, mutation);
    });
    assert_eq!(state.entries.iter().map(|entry| entry.name.as_str()).collect::<Vec<_>>(), ["a.txt", "m.txt", "b.txt"]);
    assert_eq!(emit(semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("no kind").code.0, "snapshot-edit.unsupported-path");
}

#[test]
fn natural_file_codec_preserves_exact_iso_owner_and_refuses_nonconforming_zip() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let codec = ZipIso21320Editor::natural_file_codec().expect("ISO ZIP declares its native source");
    let actual = serde_json::json!({
        "formatKind": codec.format_kind, "extension": codec.extension,
        "mediaType": codec.media_type, "binary": codec.binary
    });
    assert_eq!(actual, law["codec"]);
    assert_eq!({ let dialect = ZipIso21320Editor::DIALECT; format!("{}@{}/{}#editor", dialect.artifact_kind, dialect.standard.0, dialect.subset.0) }, law["owner"].as_str().unwrap());
    assert_eq!(ZipIso21320Editor::DOCUMENT_SCHEMA, law["artifactSchema"].as_str().unwrap());
    let bytes = include_bytes!("../../../🧪️tests/🧫️fixtures/🏷️rename-entry-applied/➡️after.zip");
    let snapshot = ZipIso21320Editor::decode_natural_file(bytes).unwrap();
    let members: Vec<_> = snapshot.entries.iter().map(|entry| serde_json::json!({"name": entry.name, "bytes": entry.data})).collect();
    assert_eq!(serde_json::to_value(members).unwrap(), law["members"]);
    let encoded = ZipIso21320Editor::encode_natural_file(&snapshot).unwrap();
    let reparsed = ZipIso21320Editor::decode_natural_file(&encoded).unwrap();
    assert_eq!(reparsed, snapshot);
    assert!(ZipIso21320Editor::decode_natural_file(include_bytes!("🧫️fixtures/⛔️bzip2.zip")).is_err());
    let mut encrypted = snapshot;
    encrypted.entries[0].metadata.local.flags |= 1;
    encrypted.entries[0].metadata.central.flags |= 1;
    assert!(ZipIso21320Editor::encode_natural_file(&encrypted).is_err());
}
