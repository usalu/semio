use super::*;

async fn restore_archive(app: &mut impl semio_framework_plugin::PluginApp, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), String> {
    app.begin_document_archive_load(operation, archive).map_err(|error| format!("{error:?}"))?;
    for _ in 0..100_000 {
        let status = Box::pin(app.poll_document_archive_load(operation)).await.map_err(|error| format!("{error:?}"))?;
        match status.state {
            protocol::DocumentArchiveLoadState::Ready => return app.acknowledge_document_archive_load(operation).map_err(|error| format!("{error:?}")),
            protocol::DocumentArchiveLoadState::Fault | protocol::DocumentArchiveLoadState::Cancelled => return Err(format!("drawing archive restore failed: {status:?}")),
            _ => {}
        }
        app.maintenance_step(1, store::OWNED_SCHEMA_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
        std::thread::yield_now();
    }
    Err("drawing archive restore exceeded its work bound".into())
}

#[semio_framework_async_macros::async_test]
async fn drawing_viewer_restores_edited_archive_and_preserves_history() {
    use crate::editor::drawing::{create_drawing_app, DrawingCommand, DrawingPlayApp, commands::add_layer::AddLayer};
    use semio_framework_plugin::{artifact_app_laws as laws, App, EditorApp, PluginApp};
    let mut editor = Box::new(laws::new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }).await);
    let mut viewer = Box::new(laws::new_app_with_registry::<ViewerApp<DrawingViewer>>(|| App { definition: create_drawing_viewer(), examples: Vec::new() }).await);
    let mut reopened = Box::new(laws::new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }).await);
    let meta = laws::meta("drawing-role-archive");
    editor.bind_instance_id(meta.instance_id).await;
    viewer.bind_instance_id(meta.instance_id + 1).await;
    reopened.bind_instance_id(meta.instance_id + 2).await;
    let outcome: Result<(), String> = async {
        let before = editor.snapshot().map_err(|error| format!("{error:?}"))?.layers.len();
        editor.dispatch_typed(DrawingCommand::AddLayer(AddLayer { kind: "shape:rect".into() }), &meta).await.map_err(|error| format!("{error:?}"))?;
        laws::settle_registered_typed_operation(&mut *editor, meta.instance_id).await.map_err(|error| format!("{error:?}"))?;
        let created_id = crate::schema::layer_id(editor.snapshot().map_err(|error| format!("{error:?}"))?.layers.last().ok_or("created rectangle is missing")?).to_string();
        editor.dispatch_typed(DrawingCommand::PatchLayers(crate::editor::drawing::commands::patch_layers::PatchLayers { layer_ids: vec![created_id.clone()], field: "name".into(), value: "Retained Drawing".into() }), &meta).await.map_err(|error| format!("{error:?}"))?;
        laws::settle_registered_typed_operation(&mut *editor, meta.instance_id).await.map_err(|error| format!("{error:?}"))?;
        let expected = editor.document_archive().await.map_err(|error| format!("{error:?}"))?;
        restore_archive(&mut *viewer, 91, expected.clone()).await?;
        let actual = viewer.document_archive().await.map_err(|error| format!("{error:?}"))?;
        if actual != expected { return Err("viewer changed edited drawing or retained history bytes".into()); }
        if !viewer.snapshot().map_err(|error| format!("{error:?}"))?.layers.iter().any(|layer| matches!(layer, crate::DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")) { return Err("restored viewer omitted the authored rectangle".into()); }
        restore_archive(&mut *reopened, 92, actual).await?;
        if reopened.document_archive().await.map_err(|error| format!("{error:?}"))? != expected { return Err("returning editor changed the transferred archive".into()); }
        let history_meta = semio_framework_plugin::ActionMeta { instance_id: meta.instance_id + 2, ..laws::meta("drawing-role-archive") };
        for (action, count, name) in [("undo", before + 1, Some("Rectangle")), ("undo", before, None), ("redo", before + 1, Some("Rectangle")), ("redo", before + 1, Some("Retained Drawing"))] {
            let admitted = reopened.handle_action(action, None, &history_meta).await.map_err(|error| format!("{error:?}"))?;
            semio_framework_plugin::app::settle_framework_reserved_admission(&mut *reopened, admitted).await.map_err(|error| format!("{error:?}"))?;
            laws::settle_registered_typed_operation(&mut *reopened, history_meta.instance_id).await.map_err(|error| format!("{error:?}"))?;
            let snapshot = reopened.snapshot().map_err(|error| format!("{error:?}"))?;
            if snapshot.layers.len() != count { return Err(format!("{action} lost the pre-switch rectangle history")); }
            if crate::schema::find_drawing_layer(&snapshot, &created_id).map(|layer| crate::schema::layer_base(layer).name.as_str()) != name { return Err(format!("{action} lost the pre-switch rectangle name")); }
        }
        Ok(())
    }.await;
    laws::close_registered_fixture_app(&mut *reopened);
    laws::close_registered_fixture_app(&mut *viewer);
    laws::close_registered_fixture_app(&mut *editor);
    outcome.expect("viewer preserves edited document archive");
}

#[semio_framework_async_macros::async_test]
async fn create_drawing_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_drawing_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, DRAWING_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DrawingViewer as ArtifactViewer>::DIALECT, DRAWING_DIALECT);
}

#[test]
fn drawing_viewer_camera_is_declared_with_only_local_window_publication() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let definition = create_drawing_viewer();
    let window = definition.window_kinds.iter().find(|window| window.id == canvas::WINDOW_KIND_ID).unwrap();
    let action = window.actions.iter().find(|action| action.id == "setCamera").unwrap();
    assert_eq!(action.kind,semio_framework_plugin::ActionKind::View);
    assert_eq!(action.semantics.execution.interactive_job,InteractiveJobClassification::Migrated);
    assert_eq!(<DrawingViewCommand as protocol::OpBinary>::TOOL_JOB_IDS,DRAWING_VIEW_TOOL_IDS);
    let contracts = <DrawingViewCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(contracts.len(),1);
    assert_eq!(contracts[0].lanes,&[ArtifactToolPublicationLane::WindowConfig]);
}

#[test]
fn drawing_viewer_camera_payload_validation_matches_the_host_shape() {
    let valid = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(r#"{"camera":{"x":1,"y":2,"zoom":3}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(command_from_action("setCamera",Some(&valid)).is_ok());
    assert!(command_from_action("editPath",Some(&valid)).is_err());
    assert!(command_from_action("setCamera",None).is_err());
    let invalid = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(r#"{"camera":{"x":1,"y":2,"zoom":0}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(command_from_action("setCamera",Some(&invalid)).is_err());
}

/// ⚖️ LAW: the viewer opens, renders and closes its document through the artifact's OWN owner catalogue
/// — the framework's generic bounded owners drop an owned snapshot root plainly, which is the guest
/// panic S15 measured on the generation2d viewer (session 11); this viewer had the same gap.
#[semio_framework_async_macros::async_test]
async fn the_viewer_opens_and_closes_its_document_through_the_artifacts_owners() {
    let mut app = semio_framework_plugin::artifact_app_laws::new_viewer::<DrawingViewer>().await;
    let tree = semio_framework_plugin::PluginApp::render(&mut app, canvas::BODY_KEY, None, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("the viewer renders its default document");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("the canvas projects");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
