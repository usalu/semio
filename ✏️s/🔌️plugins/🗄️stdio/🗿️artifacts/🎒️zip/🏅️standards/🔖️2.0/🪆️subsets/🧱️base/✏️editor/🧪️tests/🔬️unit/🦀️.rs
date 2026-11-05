use crate::editor::zip::base::modes::edit::windows::main;
use crate::{ZipMutation, ZipSnapshot};
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use crate::apply_mutation;
use crate::editor::zip::base::*;

#[semio_framework_async_macros::async_test]
async fn create_zip_any_editor_builds_a_definition_for_the_editor_role() {
    let def = create_zip_any_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, ZIP_ANY_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ZipAnyEditor as ArtifactEditor>::DIALECT, ZIP_ANY_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_zip_any_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

semio_plugin::history_edit_acceptance_law!("stdio", crate::editor::zip::base::ZipAnyEditor, || semio_plugin::App { definition: crate::editor::zip::base::create_zip_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_member() {
    
    use crate::schema::snapshot::ZipEntry;
    let base = ZipSnapshot { entries: vec![ZipEntry { name: "a.txt".into(), data: vec![1], ..ZipEntry::default() }, ZipEntry { name: "b.txt".into(), data: vec![2], ..ZipEntry::default() }], ..ZipSnapshot::default() };
    let emit = |event: semio_s_artifact_stdio_contract::editing::SnapshotEditEvent| <ZipAnyEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
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

#[semio_framework_async_macros::async_test]
async fn unsafe_member_names_are_refused_by_rename_and_insert_and_safe_names_pass() {
    use crate::schema::snapshot::ZipEntry;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛡️unsafe-names/🔣️.json")).expect("neutral unsafe-name fixture");
    let base = ZipSnapshot { entries: vec![ZipEntry { name: "a.txt".into(), data: vec![1], ..ZipEntry::default() }], ..ZipSnapshot::default() };
    let emit = |event: semio_s_artifact_stdio_contract::editing::SnapshotEditEvent| <ZipAnyEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).and_then(crate::editor::editing::require_safe_names);
    for case in fixture["names"].as_array().expect("names") {
        let name = case["name"].as_str().expect("name");
        let safe = case["safe"].as_bool().expect("safe");
        let renamed = emit(semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/entries/0/name".into(), value: semio_framework_value::DslValue::String(name.into()) });
        let inserted = emit(semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::InsertValue { path: "/entries/-".into(), value: semio_framework_value::ToValue::to_value(&ZipEntry { name: name.into(), ..ZipEntry::default() }) });
        assert_eq!(renamed.is_ok(), safe, "rename {name:?}");
        assert_eq!(inserted.is_ok(), safe, "insert {name:?}");
        if !safe {
            assert_eq!(renamed.err().expect("refused").code.0, "stdio.zip.name-unsafe");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn natural_zip_files_preserve_unedited_members_through_an_edit_and_agree_with_the_independent_reader() {
    use crate::schema::snapshot::ZipEntry;
    let codec = <ZipAnyEditor as ArtifactEditor>::natural_file_codec().expect("zip declares its natural file");
    assert_eq!((codec.extension, codec.media_type, codec.binary), (".zip", "application/zip", true));
    let stored = crate::schema::snapshot::ZipEntryMetadata { compression_method: 0, ..Default::default() };
    let base = ZipSnapshot { entries: vec![ZipEntry { name: "a.txt".into(), data: b"alpha alpha alpha".to_vec(), ..ZipEntry::default() }, ZipEntry { name: "s.bin".into(), data: vec![1, 2, 3], metadata: stored }], comment: "kept".into(), schema: crate::STDIO_ZIP_DOCUMENT_SCHEMA.into(), ..ZipSnapshot::default() };
    let bytes = <ZipAnyEditor as ArtifactEditor>::encode_natural_file(&base).expect("encode");
    let opened = <ZipAnyEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("decode");
    assert_eq!(opened, base);
    let renamed = ZipMutation::RenameEntry(crate::schema::mutations::rename_entry::RenameEntry { name: "a.txt".into(), new_name: "b.txt".into() });
    let mut edited = opened.clone();
    apply_mutation(&mut edited, &renamed);
    let reopened = <ZipAnyEditor as ArtifactEditor>::decode_natural_file(&<ZipAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("encode edited")).expect("reopen");
    assert_eq!(reopened.entries[0].name, "b.txt");
    assert_eq!(reopened.entries[1], base.entries[1]);
    assert_eq!(reopened.entries[0].metadata, base.entries[0].metadata);
    assert_eq!(reopened.comment, "kept");
    let mut oracle = zip::ZipArchive::new(std::io::Cursor::new(<ZipAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("encode"))).expect("independent reader");
    assert_eq!(oracle.by_index(1).expect("member").compression(), zip::CompressionMethod::Stored);
    assert!(<ZipAnyEditor as ArtifactEditor>::decode_natural_file(&bytes[..bytes.len() / 2]).is_err());
}
