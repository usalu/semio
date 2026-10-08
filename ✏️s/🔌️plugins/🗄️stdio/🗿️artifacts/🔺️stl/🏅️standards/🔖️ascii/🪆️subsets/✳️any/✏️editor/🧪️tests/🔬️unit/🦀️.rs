use crate::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_stl_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, STL_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<StlAnyEditor as ArtifactEditor>::DIALECT, STL_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<StlAnyEditor, crate::viewer::stl::StlAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::StlAnyEditor, || semio_framework_plugin::App { definition: super::create_stl_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️ascii/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use crate::standards::v_ascii::subsets::any::schema::snapshot::StlTriangle;
    let base = StlSnapshot { triangles: vec![StlTriangle::default(), StlTriangle::default()], ..StlSnapshot::default() };
    let emit = |event: editing::SnapshotEditEvent| <StlAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let renamed = emit(editing::SnapshotEditEvent::SetValue { path: "/solidName".into(), value: semio_framework_value::DslValue::String("part".into()) }).expect("a name edit resolves");
    assert!(matches!(renamed.artifact_mutations.as_slice(), [StlMutation::SetSolidName(_)]));
    let nudged = emit(editing::SnapshotEditEvent::SetValue { path: "/triangles/1/vertices/0/2".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(2.5)) }).expect("a vertex edit resolves");
    let [mutation @ StlMutation::SetTriangleVertices(_)] = nudged.artifact_mutations.as_slice() else { panic!("a vertex edit raises the vertices kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.triangles[1].vertices[0][2], 2.5);
    let whole = emit(editing::SnapshotEditEvent::SetValue { path: "/triangles/0".into(), value: semio_framework_value::ToValue::to_value(&StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[1.0, 0.0, 0.0]; 3] }) }).expect("a whole-triangle set resolves");
    assert!(matches!(whole.artifact_mutations.as_slice(), [StlMutation::SetTriangleNormal(_), StlMutation::SetTriangleVertices(_)]));
    let unnamed = emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("the schema stamp has no kind");
    assert_eq!(unnamed.code.0, "snapshot-edit.unsupported-path");
}
