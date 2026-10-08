//! 📷️ Viewer camera traces retain exact window ownership and preserve artifact bytes.
use super::*;
use crate::viewer::drawing::{create_drawing_viewer,DrawingViewCommand,DrawingViewer};
use semio_framework_plugin::{artifact_app_laws,ActionMeta,App,PluginApp,VcsArtifactApp,ViewerApp,ViewModel,ViewWindowInstance,WindowConfigOwner};

async fn scene(app: &mut VcsArtifactApp<ViewerApp<DrawingViewer>>,view: &ViewModel) -> Result<semio_framework_plugin::Canvas2dScene,String> {
    let tree = app.render(super::super::BODY_KEY,None,view).await.map_err(|error| format!("{error:?}"))?;
    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
    artifact_app_laws::decode_fixture_scene(&json).map_err(str::to_string)
}

#[semio_framework_async_macros::async_test]
async fn drawing_viewer_camera_ownership_and_restore() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let manifest = || App { definition: create_drawing_viewer(),examples: Vec::new() };
    let mut app = Box::new(artifact_app_laws::new_app_with_registry::<ViewerApp<DrawingViewer>>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await);
    app.bind_instance_id(91).await;
    let view = ViewModel { window_instances: ["left","right"].into_iter().map(|id| ViewWindowInstance { id: id.into(),window_kind_id: DrawingViewerCanvasWindowConfigOwner::WINDOW_KIND_ID.into() }).collect(),..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<ViewerApp<DrawingViewer>>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await);
    reopened.bind_instance_id(92).await;
    let outcome: Result<(),String> = async {
        let before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
        for step in fixture["steps"].as_array().ok_or("camera fixture steps missing")? {
            let context = view.for_window_instance(step["window"].as_str().ok_or("camera fixture window missing")?).ok_or("camera fixture window unknown")?;
            let command = DrawingViewCommand::SetCamera { camera: step["camera"].to_string() };
            let bytes = protocol::OpBinary::encode_op(&command).map_err(|error| format!("{error:?}"))?;
            let decoded = <DrawingViewCommand as protocol::OpBinary>::decode_op(&bytes).map_err(|error| format!("{error:?}"))?;
            if decoded != command { return Err("viewer camera command binary roundtrip changed payload".into()); }
            let meta = ActionMeta { instance_id: 91,view_state: Some(context),..artifact_app_laws::meta("drawing-viewer-camera") };
            app.dispatch_typed(command,&meta).await.map_err(|error| format!("{error:?}"))?;
            let receipt = artifact_app_laws::settle_registered_typed_operation(&mut *app,91).await.map_err(|error| format!("{error:?}"))?;
            if receipt.lanes != vec![semio_framework_plugin::app::TypedOperationResultLane::WindowConfig,semio_framework_plugin::app::TypedOperationResultLane::Ui,semio_framework_plugin::app::TypedOperationResultLane::Terminal] { return Err(format!("viewer camera published unexpected lanes: {:?}",receipt.lanes)); }
        }
        let after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
        if before.pack != after.pack || before.spr != after.spr { return Err("viewer navigation changed artifact bytes".into()); }
        let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
        for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
        for (id,value) in fixture["expected"].as_object().ok_or("camera fixture expectations missing")? {
            let context = view.for_window_instance(id).ok_or("expected camera window unknown")?;
            let expected: DrawingViewerCanvasWindowConfig = semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("{error:?}"))?;
            for target in [&mut app,&mut reopened] {
                let scene = scene(target,&context).await?;
                if scene.framing.is_some() { return Err(format!("stored viewer camera requested another fit: {id}")); }
                if (scene.camera_x,scene.camera_y,scene.zoom) != (expected.viewport.x,expected.viewport.y,expected.viewport.zoom) { return Err(format!("viewer camera changed across windows or restoration: {id}")); }
            }
        }
        Ok(())
    }.await;
    artifact_app_laws::close_registered_fixture_app(&mut *reopened);
    artifact_app_laws::close_registered_fixture_app(&mut *app);
    outcome.expect("Drawing viewer camera ownership and restoration");
}

#[test]
fn drawing_viewer_camera_refuses_stale_or_wrong_windows() {
    let config = DrawingViewerCanvasWindowConfig::default();
    assert!(addressed(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),config.clone()).is_err());
    let view = ViewModel { window_id: Some("gone".into()),..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    assert!(addressed(&view,config.clone()).is_err());
    let view = ViewModel { window_id: Some("foreign".into()),window_instances: vec![ViewWindowInstance { id: "foreign".into(),window_kind_id: "other".into() }],..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    assert!(addressed(&view,config).is_err());
}

/// ⚖️ The viewer camera configuration's concrete inverse sums to exactly the negative of its sparse diff, and `between` is its state delta.
#[semio_framework_async_macros::async_test]
async fn config_inverse_sums_to_the_negative_diff() {
    let base = DrawingViewerCanvasWindowConfig::default();
    let next = DrawingViewerCanvasWindowConfig { viewport: store::Viewport2d { x: 18.0, y: -9.0, zoom: 2.5 }, framed: true };
    let mutation = DrawingViewerCanvasWindowConfigMutation::Set { viewport: next.viewport.clone(), framed: next.framed };
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<DrawingViewerCanvasWindowConfig, super::DrawingViewerCanvasWindowConfigDiff>(&base, &next).await;
}
