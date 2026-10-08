use crate::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_ply_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PLY_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<PlyAnyEditor as ArtifactEditor>::DIALECT, PLY_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<PlyAnyEditor, crate::viewer::ply::PlyAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::PlyAnyEditor, || semio_framework_plugin::App { definition: super::create_ply_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.0/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use crate::standards::v1_0::subsets::any::schema::snapshot::{PlyElement, PlyProperty, PlyRow, PlyScalarType, PlyValue};
    let base = PlySnapshot {
        comments: vec!["first".into(), "second".into()],
        elements: vec![PlyElement { name: "vertex".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }], rows: vec![PlyRow { values: vec![PlyValue::Float(1.0)] }] }],
        ..PlySnapshot::default()
    };
    let emit = |event: editing::SnapshotEditEvent| <PlyAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let moved = emit(editing::SnapshotEditEvent::SetValue { path: "/elements/0/rows/0/values/0/value".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(2.5)) }).expect("a value edit resolves");
    let [mutation @ PlyMutation::SetRowProperty(_)] = moved.artifact_mutations.as_slice() else { panic!("a row value edit raises the row-property kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.elements[0].rows[0].values[0], PlyValue::Float(2.5));
    let reworded = emit(editing::SnapshotEditEvent::SetValue { path: "/comments/1".into(), value: semio_framework_value::DslValue::String("changed".into()) }).expect("a comment replacement resolves");
    let mut state = base.clone();
    reworded.artifact_mutations.iter().for_each(|mutation| {
        apply_mutation(&mut state, mutation);
    });
    assert_eq!(state.comments, vec!["first".to_string(), "changed".to_string()]);
    let dropped = emit(editing::SnapshotEditEvent::RemoveValue { path: "/elements/0/rows/0".into() }).expect("a row removal resolves");
    assert!(matches!(dropped.artifact_mutations.as_slice(), [PlyMutation::RemoveRow(_)]));
    let unnamed = emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("the schema stamp has no kind");
    assert_eq!(unnamed.code.0, "snapshot-edit.unsupported-path");
}
