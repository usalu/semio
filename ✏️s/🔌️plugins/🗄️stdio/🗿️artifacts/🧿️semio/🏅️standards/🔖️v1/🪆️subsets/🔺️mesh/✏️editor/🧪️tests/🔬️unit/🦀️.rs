use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_semio_mesh_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, SEMIO_MESH_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SemioMeshEditor as ArtifactEditor>::DIALECT, SEMIO_MESH_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<SemioMeshEditor, crate::viewer::semio_mesh::SemioMeshViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn set_vertex_requires_stable_targets_and_finite_coordinates() {
    let args = DslValue::Object(vec![
        ("meshId".into(), DslValue::String("mesh-a".into())),
        ("primitiveId".into(), DslValue::String("primitive-a".into())),
        ("vertexIndex".into(), DslValue::float(2.0)),
        ("point".into(), DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0)])),
    ]);
    let command = SemioMeshEditor::command_from_action("set-vertex", Some(&args)).expect("valid command");
    assert_eq!(command, editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetVertex(SemioMeshSetVertexArgs { mesh_id: "mesh-a".into(), primitive_id: "primitive-a".into(), vertex_index: 2, point: [1.0, 2.0, 3.0] })));
    assert!(SemioMeshEditor::command_from_action("set-vertex", Some(&DslValue::Object(vec![]))).is_err());
    let definition = create_semio_mesh_editor();
    let action = definition.actions.iter().find(|action| action.id == "set-vertex").expect("set-vertex action");
    assert_eq!(action.args.len(), 4);
    assert!(action.args.iter().all(|argument| argument.required));
}
