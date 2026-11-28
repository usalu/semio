use super::*;

async fn restore_archive(app: &mut impl semio_plugin::PluginApp, operation: u64, archive: protocol::DocumentArchivePack, cx: &mut semio_plugin::StepContext<'_>) -> Result<(), String> {
    use semio_plugin::artifact_app_laws as laws;
    laws::admit_document_archive_load(app, operation, archive, cx).map_err(|error| format!("{error:?}"))?;
    laws::settle_document_archive_load(app, operation, &mut laws::fixture_identity(), cx).await.map_err(|error| format!("{error:?}"))
}

#[semio_framework_async_macros::async_test]
async fn drawing_viewer_restores_edited_archive_and_preserves_history() {
    use semio_s_artifact_draw_drawing::editor_manifest::create_drawing_app;
    use semio_s_artifact_draw_drawing_editor::DrawingCommand;
    use semio_s_artifact_draw_drawing_editor::DrawingPlayApp;
    use semio_s_artifact_draw_drawing_editor_commands::add_layer::AddLayer;
    use semio_plugin::{artifact_app_laws as laws, App, EditorApp, PluginApp};
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🕰️history/🔣️.json")).unwrap();
    let (author, observer_name) = (fixture["author"].as_str().unwrap(), fixture["observer"].as_str().unwrap());
    let meta = laws::meta(author);
    let mut editor = Box::new(laws::new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }, semio_os_kernel::ActorId(author.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await);
    let mut viewer = Box::new(laws::new_app_with_registry::<ViewerApp<DrawingViewer>>(|| App { definition: create_drawing_viewer(), examples: Vec::new() }, semio_os_kernel::ActorId(author.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await);
    let mut reopened = Box::new(laws::new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }, semio_os_kernel::ActorId(author.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await);
    let mut observer = Box::new(laws::new_app_with_registry::<EditorApp<DrawingPlayApp>>(|| App { definition: create_drawing_app(), examples: Vec::new() }, semio_os_kernel::ActorId(observer_name.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await);
    editor.bind_instance_id(meta.instance_id).await;
    viewer.bind_instance_id(meta.instance_id + 1).await;
    reopened.bind_instance_id(meta.instance_id + 2).await;
    observer.bind_instance_id(meta.instance_id + 3).await;
    let outcome: Result<(), String> = async {
        let before = editor.snapshot().map_err(|error| format!("{error:?}"))?.layers.len();
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut editor, &DrawingCommand::AddLayer(AddLayer { kind: fixture["kind"].as_str().unwrap().into() }), &meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|error| format!("{error:?}"))?;
        laws::settle_registered_typed_operation(&mut *editor, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|error| format!("{error:?}"))?;
        let created_id = semio_s_artifact_draw_drawing_model::layer_id(editor.snapshot().map_err(|error| format!("{error:?}"))?.layers.last().ok_or("created rectangle is missing")?).to_string();
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut editor, &DrawingCommand::PatchLayers(semio_s_artifact_draw_drawing_editor_commands::patch_layers::PatchLayers { index:None, layer_ids: vec![created_id.clone()], field: "name".into(), value: fixture["after"].as_str().unwrap().into() }), &meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|error| format!("{error:?}"))?;
        laws::settle_registered_typed_operation(&mut *editor, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|error| format!("{error:?}"))?;
        let expected = editor.document_archive().await.map_err(|error| format!("{error:?}"))?;
        restore_archive(&mut *viewer, 91, expected.clone(), scope.cx()).await?;
        let actual = viewer.document_archive().await.map_err(|error| format!("{error:?}"))?;
        if actual != expected { return Err("viewer changed edited drawing or retained history bytes".into()); }
        if !viewer.snapshot().map_err(|error| format!("{error:?}"))?.layers.iter().any(|layer| matches!(layer, semio_s_artifact_draw_drawing_model::DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")) { return Err("restored viewer omitted the authored rectangle".into()); }
        restore_archive(&mut *observer, 93, actual.clone(), scope.cx()).await?;
        restore_archive(&mut *reopened, 92, actual, scope.cx()).await?;
        if reopened.document_archive().await.map_err(|error| format!("{error:?}"))? != expected { return Err("returning editor changed the transferred archive".into()); }
        for row in fixture["actions"].as_array().unwrap() {
            let action=row["verb"].as_str().unwrap();let count=before+row["layerDelta"].as_u64().unwrap() as usize;let name=row["name"].as_str();
            let (replica, instance) = if row["actor"] == "observer" { (&mut observer, meta.instance_id + 3) } else { (&mut reopened, meta.instance_id + 2) };
            let history_meta = semio_plugin::ActionMeta { instance_id: instance, ..laws::meta(fixture[row["actor"].as_str().unwrap()].as_str().unwrap()) };
            let admitted = semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut **replica, action, None, &history_meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.map_err(|error| format!("{error:?}"))?;
            semio_plugin::app::settle_framework_reserved_admission(&mut **replica, admitted, scope.cx()).await.map_err(|error| format!("{error:?}"))?;
            laws::settle_registered_typed_operation(&mut **replica, history_meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|error| format!("{error:?}"))?;
            let snapshot = replica.snapshot().map_err(|error| format!("{error:?}"))?;
            if snapshot.layers.len() != count { return Err(format!("{action} lost the pre-switch rectangle history")); }
            let actual_name=semio_s_artifact_draw_drawing_model::find_drawing_layer(&snapshot, &created_id).map(|layer| semio_s_artifact_draw_drawing_model::layer_base(layer).name.to_string_owner());
            if actual_name.as_deref() != name { return Err(format!("{action} lost the pre-switch rectangle name: expected {name:?}, received {actual_name:?}")); }
        }
        Ok(())
    }.await;
    laws::close_registered_fixture_app(&mut *observer, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
    laws::close_registered_fixture_app(&mut *reopened, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
    laws::close_registered_fixture_app(&mut *viewer, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
    laws::close_registered_fixture_app(&mut *editor, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
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
    use semio_plugin::ArtifactOwnedToolJobFactory;
    let definition = create_drawing_viewer();
    let window = definition.window_kinds.iter().find(|window| window.id == canvas::WINDOW_KIND_ID).unwrap();
    let action = window.actions.iter().find(|action| action.id == "setCamera").unwrap();
    assert_eq!(action.kind,semio_plugin::ActionKind::View);
    assert_eq!(action.semantics.execution.interactive_job,InteractiveJobClassification::Migrated);
    assert_eq!(<DrawingViewCommand as protocol::OpBinary>::TOOL_JOB_IDS,DRAWING_VIEW_TOOL_IDS);
    let contracts = <DrawingViewCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(contracts.len(),canvas::HOST_ACTION_IDS.len());
    let camera = contracts.iter().find(|contract| contract.tool_id == "setCamera").unwrap();
    assert_eq!(camera.lanes,&[ArtifactToolPublicationLane::WindowConfig]);
    assert!(contracts.iter().filter(|contract| contract.tool_id != "setCamera").all(|contract| contract.lanes == [ArtifactToolPublicationLane::HostOnly]));
}

/// ⚖️ LAW: the viewer's canvas window kind declares every verb the shared `Canvas2d` surface dispatches (pointer, double click, catalogue drag, camera sync), each inert or window-config only, so no hover is dropped as undeclared.
#[test]
fn drawing_viewer_canvas_declares_every_action_its_surface_dispatches() {
    use semio_plugin::ArtifactOwnedToolJobFactory;
    let definition = create_drawing_viewer();
    let window = definition.window_kinds.iter().find(|window| window.id == canvas::WINDOW_KIND_ID).unwrap();
    for dispatched in ["setCamera","canvasPointerDown","canvasPointerMove","canvasPointerUp","canvasDoubleClick","canvasDragOver","canvasDragLeave","canvasDrop"] {
        let action = window.actions.iter().find(|action| action.id == dispatched).unwrap_or_else(|| panic!("viewer canvas does not declare {dispatched}"));
        assert_eq!(action.kind,semio_plugin::ActionKind::View);
        assert_eq!(action.semantics.execution.interactive_job,InteractiveJobClassification::Migrated);
        assert!(<DrawingViewCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS.contains(&dispatched));
        let command = command_from_action(dispatched,Some(&semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(r#"{"camera":{"x":0,"y":0,"zoom":1}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        assert_eq!(command.action_id(),dispatched);
    }
    assert_eq!(window.actions.len(),canvas::HOST_ACTION_IDS.len());
}

#[test]
fn drawing_viewer_host_input_commands_publish_nothing() {
    for dispatched in canvas::HOST_INPUT_ACTION_IDS {
        let command = command_from_action(dispatched,None).unwrap();
        let emit = camera_emit(&command,None).unwrap();
        assert!(emit.window_config_mutations.is_empty());
    }
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
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::artifact_app_laws::new_viewer::<DrawingViewer>(semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
    semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut app, canvas::BODY_KEY, &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native), scope.cx()).await.expect("the viewer renders and projects its default document");
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
}
