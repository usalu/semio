use crate::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_obj_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, OBJ_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ObjAnyEditor as ArtifactEditor>::DIALECT, OBJ_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<ObjAnyEditor, crate::viewer::obj::ObjAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::ObjAnyEditor, || semio_framework_plugin::App { definition: super::create_obj_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️3.0/🪆️subsets/📐️geometry");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_row() {
    
    use crate::standards::v3_0::subsets::any::schema::snapshot::{ObjGroup, ObjVertex};
    let base = ObjSnapshot { vertices: vec![ObjVertex::default(), ObjVertex::default()], groups: vec![ObjGroup { name: "walls".into(), faces: vec![0] }], ..ObjSnapshot::default() };
    let emit = |event: editing::SnapshotEditEvent| <ObjAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let moved = emit(editing::SnapshotEditEvent::SetValue { path: "/vertices/1/x".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(2.0)) }).expect("a vertex edit resolves");
    let [mutation @ ObjMutation::SetVertex(_)] = moved.artifact_mutations.as_slice() else { panic!("a vertex coordinate raises the vertex kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.vertices[1].x, 2.0);
    let renamed = emit(editing::SnapshotEditEvent::SetValue { path: "/groups/0/name".into(), value: semio_framework_value::DslValue::String("floors".into()) }).expect("a group rename resolves");
    let mut state = base.clone();
    renamed.artifact_mutations.iter().for_each(|mutation| {
        apply_mutation(&mut state, mutation);
    });
    assert_eq!(state.groups[0].name, "floors");
    let dropped = emit(editing::SnapshotEditEvent::RemoveValue { path: "/vertices/0".into() }).expect("a vertex removal resolves");
    assert!(matches!(dropped.artifact_mutations.as_slice(), [ObjMutation::RemoveVertex(_)]));
    assert_eq!(emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("no kind").code.0, "snapshot-edit.unsupported-path");
}
