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
    let action = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "set-vertex").expect("set-vertex action");
    assert_eq!(action.args.len(), 4);
    assert!(action.args.iter().all(|argument| argument.required));
}

#[semio_framework_async_macros::async_test]
async fn set_vertex_refuses_duplicate_mesh_and_primitive_ids() {
    use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
    use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};

    let primitive = SemioPrimitive { id: "primitive-a".into(), positions: vec![SemioPoint3::default()], ..Default::default() };
    let args = SemioMeshSetVertexArgs { mesh_id: "mesh-a".into(), primitive_id: "primitive-a".into(), vertex_index: 0, point: [1.0, 2.0, 3.0] };
    let mut duplicate_mesh = SemioMeshSnapshot::default();
    duplicate_mesh.meshes = vec![SemioMesh { id: "mesh-a".into(), primitives: vec![primitive.clone()] }, SemioMesh { id: "mesh-a".into(), primitives: vec![primitive.clone()] }];
    assert!(move_vertex_mutation(&duplicate_mesh, &args).expect_err("duplicate mesh id").describe().contains("ambiguous"));

    let mut duplicate_primitive = SemioMeshSnapshot::default();
    duplicate_primitive.meshes = vec![SemioMesh { id: "mesh-a".into(), primitives: vec![primitive.clone(), primitive] }];
    assert!(move_vertex_mutation(&duplicate_primitive, &args).expect_err("duplicate primitive id").describe().contains("ambiguous"));
}

#[test]
fn encoded_set_vertex_admission_counts_both_target_ids() {
    let command = editing::SnapshotEditingCommand::Native(SemioMeshEditCommand::SetVertex(SemioMeshSetVertexArgs { mesh_id: "mesh".repeat(29), primitive_id: "primitive".repeat(31), vertex_index: 7, point: [1.0, 2.0, 3.0] }));
    let encoded = protocol::OpBinary::encode_op(&command).expect("encode set-vertex");
    assert_eq!(editing::admit_bounded_native_command(&command, encoded.len()).expect("exact admission"), encoded.len());
    assert!(editing::admit_bounded_native_command(&command, encoded.len() - 1).is_err());
}

#[semio_framework_async_macros::async_test]
async fn registered_set_vertex_publishes_refuses_a_missing_target_and_undoes_redoes() {
    use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
    use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = SemioMeshSnapshot::default();
    original.meshes.push(SemioMesh { id: "mesh-a".into(), primitives: vec![SemioPrimitive { id: "primitive-a".into(), positions: vec![SemioPoint3 { x: 0.0, y: 1.0, z: 2.0 }], ..Default::default() }] });
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<SemioMeshEditor>, _>(async { semio_framework_plugin::App { definition: create_semio_mesh_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, SEMIO_MESH_DOCUMENT_SCHEMA) else { panic!("mesh fixture produces a document load") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let meta = artifact_app_laws::meta("local");
    let arguments = |primitive_id: &str, point: [f64; 3]| {
        DslValue::object([
            ("meshId".into(), DslValue::String("mesh-a".into())),
            ("primitiveId".into(), DslValue::String(primitive_id.into())),
            ("vertexIndex".into(), DslValue::float(0.0)),
            ("point".into(), DslValue::Array(point.into_iter().map(DslValue::float).collect())),
        ])
    };

    let edit = arguments("primitive-a", [3.0, 4.0, 5.0]);
    app.handle_action("set-vertex", Some(&edit), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    assert_eq!(app.snapshot().unwrap().meshes[0].primitives[0].positions[0], SemioPoint3 { x: 3.0, y: 4.0, z: 5.0 });
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), original);
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap().meshes[0].primitives[0].positions[0], SemioPoint3 { x: 3.0, y: 4.0, z: 5.0 });

    let missing = arguments("missing", [7.0, 8.0, 9.0]);
    let history_before = semio_framework::io::resolve_ready(app.history_snapshot()).expect("history before refused edit").upserts.len();
    app.handle_action("set-vertex", Some(&missing), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(app.snapshot().unwrap().meshes[0].primitives[0].positions[0], SemioPoint3 { x: 3.0, y: 4.0, z: 5.0 });
    assert_eq!(semio_framework::io::resolve_ready(app.history_snapshot()).expect("history after refused edit").upserts.len(), history_before);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn registered_set_vertex_refuses_duplicate_targets_without_history() {
    use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
    use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let primitive = SemioPrimitive { id: "primitive-a".into(), positions: vec![SemioPoint3::default()], ..Default::default() };
    for duplicate in [
        SemioMeshSnapshot { meshes: vec![SemioMesh { id: "mesh-a".into(), primitives: vec![primitive.clone()] }, SemioMesh { id: "mesh-a".into(), primitives: vec![primitive.clone()] }], ..Default::default() },
        SemioMeshSnapshot { meshes: vec![SemioMesh { id: "mesh-a".into(), primitives: vec![primitive.clone(), primitive.clone()] }], ..Default::default() },
    ] {
        let mut app = artifact_app_laws::new_registered_app::<EditorApp<SemioMeshEditor>, _>(async { semio_framework_plugin::App { definition: create_semio_mesh_editor(), examples: Vec::new() } }).await;
        let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&duplicate, SEMIO_MESH_DOCUMENT_SCHEMA) else { panic!("mesh fixture produces a document load") };
        app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
        let meta = artifact_app_laws::meta("local");
        let history_before = semio_framework::io::resolve_ready(app.history_snapshot()).expect("initial history").upserts.len();
        let args = DslValue::object([
            ("meshId".into(), DslValue::String("mesh-a".into())),
            ("primitiveId".into(), DslValue::String("primitive-a".into())),
            ("vertexIndex".into(), DslValue::float(0.0)),
            ("point".into(), DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0)])),
        ]);
        app.handle_action("set-vertex", Some(&args), &meta).await.unwrap();
        assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
        assert_eq!(app.snapshot().unwrap(), duplicate);
        assert_eq!(semio_framework::io::resolve_ready(app.history_snapshot()).expect("history after duplicate refusal").upserts.len(), history_before);
        artifact_app_laws::close_registered_fixture_app(&mut app);
    }
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", SemioMeshEditor, || semio_framework_plugin::App { definition: create_semio_mesh_editor(), examples: Vec::new() }, "../..");
