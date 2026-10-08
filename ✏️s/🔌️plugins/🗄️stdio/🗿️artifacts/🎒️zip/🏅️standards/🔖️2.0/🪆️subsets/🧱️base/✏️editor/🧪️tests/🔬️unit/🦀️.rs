use crate::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_zip_any_editor_builds_a_definition_for_the_editor_role() {
    let def = create_zip_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
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

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::ZipAnyEditor, || semio_framework_plugin::App { definition: super::create_zip_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base");

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
    assert_eq!(emit(semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("no kind").code.0, "snapshot-edit.unsupported-path");
}
