use crate::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_dwg_ac1018_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DWG_AC1018_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DwgAc1018Editor as ArtifactEditor>::DIALECT, DWG_AC1018_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<DwgAc1018Editor, crate::viewer::dwg_ac1018::DwgAc1018Viewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::DwgAc1018Editor, || semio_framework_plugin::App { definition: super::create_dwg_ac1018_editor(), examples: Vec::new() }, "../../🏅️standards/4️⃣ac1018/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_block() {
    
    let base = crate::standards::v_ac1024::engine::demo_dwg_snapshot();
    let title = editing::SnapshotEditEvent::SetValue { path: "/summary/title".into(), value: semio_framework_value::DslValue::String("renamed".into()) };
    let emit = <DwgAc1018Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&title, &base).expect("a summary edit resolves");
    let [mutation @ DwgMutation::SetSummary(_)] = emit.artifact_mutations.as_slice() else { panic!("a summary edit raises the summary kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.summary.title, "renamed");
    let codepage = editing::SnapshotEditEvent::SetValue { path: "/codepage".into(), value: semio_framework_value::DslValue::uint(u64::from(base.codepage) + 1) };
    let emit = <DwgAc1018Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&codepage, &base).expect("a preamble edit resolves");
    let [mutation @ DwgMutation::SetVersionInfo(_)] = emit.artifact_mutations.as_slice() else { panic!("a preamble edit raises the version-info kind") };
    apply_mutation(&mut state, mutation);
    assert_eq!(state.codepage, base.codepage + 1);
    let stamp = editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) };
    assert_eq!(<DwgAc1018Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&stamp, &base).expect_err("the schema stamp has no kind").code.0, "snapshot-edit.unsupported-path");
}
