use super::*;
use crate::editor::drawing::modes::edit::windows::canvas::transient::{DrawingCanvasWindowTransient, DrawingCanvasWindowTransientMutation, DrawingCanvasWindowTransientOwner};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn block_on_drawing_windows<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[test]
fn drawing_canvas_window_ownership_runtime_isolates_reloads_and_restores_exact_instances() {
    std::thread::Builder::new()
        .name("drawing-canvas-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| block_on_drawing_windows(async {
            use crate::editor::drawing::commands::{engagement_input, set_camera};
            use crate::editor::drawing::{create_drawing_app, DrawingCommand, DrawingPlayApp, DRAWING_PLAY_BODY_COMPOSITE};
            use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

            async fn dispatch(app: &mut VcsArtifactApp<EditorApp<DrawingPlayApp>>, command: DrawingCommand, view: &ViewModel) -> Result<(usize, usize), String> {
                let meta = ActionMeta { instance_id: 91, view_state: Some(view.clone()), ..artifact_app_laws::meta("drawing-window-ownership") };
                app.dispatch_typed(command, &meta).await.map_err(|error| format!("{error:?}"))?;
                let receipt = artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map_err(|error| format!("{error:?}"))?;
                Ok((
                    receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count(),
                    receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowTransient).count(),
                ))
            }

            async fn scene(app: &mut VcsArtifactApp<EditorApp<DrawingPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::Canvas2dScene, String> {
                let tree = app.render(DRAWING_PLAY_BODY_COMPOSITE, None, view).await.map_err(|error| format!("{error:?}"))?;
                let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                artifact_app_laws::decode_fixture_scene(&json).map_err(str::to_string)
            }

            let manifest = || App { definition: create_drawing_app(), examples: Vec::new() };
            let view = ViewModel {
                window_instances: vec![
                    ViewWindowInstance { id: "drawing-left".into(), window_kind_id: DrawingCanvasWindowConfigOwner::WINDOW_KIND_ID.into() },
                    ViewWindowInstance { id: "drawing-right".into(), window_kind_id: DrawingCanvasWindowConfigOwner::WINDOW_KIND_ID.into() },
                    ViewWindowInstance { id: "drawing-properties".into(), window_kind_id: "drawing-properties".into() },
                ],
                ..Default::default()
            };
            let left = view.for_window_instance("drawing-left").unwrap();
            let right = view.for_window_instance("drawing-right").unwrap();
            let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<DrawingPlayApp>>(manifest).await);
            app.bind_instance_id(91).await;
            let outcome: Result<(), String> = async {
                let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let mut lanes = (0, 0);
                for (context, camera) in [
                    (&left, store::Viewport2d { x: 18.0, y: -9.0, zoom: 2.5 }),
                    (&right, store::Viewport2d { x: -7.0, y: 31.0, zoom: 0.5 }),
                ] {
                    let next = dispatch(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera }), context).await?;
                    lanes.0 += next.0;
                    lanes.1 += next.1;
                }
                let next = dispatch(&mut app, DrawingCommand::EngagementInput(engagement_input::EngagementInput { value: "Layer A".into() }), &left).await?;
                lanes.0 += next.0;
                lanes.1 += next.1;
                if lanes != (2, 1) { return Err(format!("Drawing exact-window lane count changed: {lanes:?}")); }
                let left_scene = scene(&mut app, &left).await?;
                let right_scene = scene(&mut app, &right).await?;
                if (left_scene.camera_x, left_scene.camera_y, left_scene.zoom) != (18.0, -9.0, 2.5)
                    || (right_scene.camera_x, right_scene.camera_y, right_scene.zoom) != (-7.0, 31.0, 0.5)
                {
                    return Err("Drawing camera crossed exact Canvas window partitions".into());
                }
                let transient = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("Drawing Canvas transient owner missing")?;
                if transient.get::<DrawingCanvasWindowTransientOwner>().map(|value| value.engagement_input.as_str()) != Some("Layer A") {
                    return Err("Drawing engagement input did not reach the exact Canvas transient".into());
                }
                let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                if document_before.pack != document_after.pack || document_before.spr != document_after.spr { return Err("Drawing window publication changed document bytes".into()); }
                let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                app.load_document_pack(&document_before).await.map_err(|error| format!("{error:?}"))?;
                let cleared = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("Drawing Canvas transient owner missing after reload")?;
                if cleared.get::<DrawingCanvasWindowTransientOwner>() != Some(&DrawingCanvasWindowTransient::default()) { return Err("Drawing Canvas transient survived same-byte reload".into()); }
                for context in [&left, &right] {
                    if app.window_config_generation(context).await.map_err(|error| format!("{error:?}"))?.is_none() { return Err("Drawing Canvas config was lost during reload".into()); }
                }
                let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<DrawingPlayApp>>(manifest).await);
                reopened.bind_instance_id(92).await;
                for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                let reopened_left = scene(&mut reopened, &left).await?;
                let reopened_right = scene(&mut reopened, &right).await?;
                artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                if reopened_left.camera_x != left_scene.camera_x || reopened_right.camera_x != right_scene.camera_x { return Err("Drawing Canvas config changed during restore".into()); }
                let stale = ViewModel { window_id: Some("lost-drawing".into()), window_instances: view.window_instances.clone(), ..Default::default() };
                if addressed(&stale, DrawingCanvasWindowConfig::default()).is_ok() { return Err("Drawing accepted stale window identity".into()); }
                let wrong = view.for_window_instance("drawing-properties").unwrap();
                if addressed(&wrong, DrawingCanvasWindowConfig::default()).is_ok() { return Err("Drawing accepted a non-Canvas window identity".into()); }
                Ok(())
            }
            .await;
            if let Err(error) = &outcome { eprintln!("[DEBUG] Drawing exact-window runtime failure before close: {error}"); }
            artifact_app_laws::close_registered_fixture_app(&mut *app);
            outcome.expect("Drawing exact-window runtime law");
            eprintln!("[DEBUG] Drawing runtime isolated two Canvas windows, restored persisted Viewport2d config, cleared transient on reload, and preserved document bytes");
        }))
        .expect("spawn Drawing window ownership law")
        .join()
        .expect("Drawing window ownership law thread");
}

#[test]
fn drawing_canvas_window_ownership_matches_neutral_fixture_and_codecs() {
    use store::{ArtifactDsl, ArtifactPack};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let base_config: DrawingCanvasWindowConfig = dsl::json::from_json_str(&fixture["baseConfig"].to_string()).unwrap();
    let next_config: DrawingCanvasWindowConfig = dsl::json::from_json_str(&fixture["nextConfig"].to_string()).unwrap();
    let config_mutation: DrawingCanvasWindowConfigMutation = dsl::json::from_json_str(&fixture["configMutation"].to_string()).unwrap();
    let after = config_mutation.diff(&base_config).diff().apply(&base_config).unwrap();
    assert_eq!(after, next_config);
    let restored = config_mutation.inverse(&base_config).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
    assert_eq!(restored, base_config);
    assert_eq!(DrawingCanvasWindowConfig::parse_dsl(&base_config.print_dsl()).unwrap(), base_config);
    assert_eq!(DrawingCanvasWindowConfig::decode_pack(&base_config.encode_pack()).unwrap(), base_config);
    assert_eq!(DrawingCanvasWindowConfigMutation::parse_op(&config_mutation.print_op()).unwrap(), config_mutation);
    assert_eq!(DrawingCanvasWindowConfigMutation::decode_op(&config_mutation.encode_op().unwrap()).unwrap(), config_mutation);

    let base_transient: DrawingCanvasWindowTransient = dsl::json::from_json_str(&fixture["baseTransient"].to_string()).unwrap();
    let next_transient: DrawingCanvasWindowTransient = dsl::json::from_json_str(&fixture["nextTransient"].to_string()).unwrap();
    let transient_mutation: DrawingCanvasWindowTransientMutation = dsl::json::from_json_str(&fixture["transientMutation"].to_string()).unwrap();
    let after = transient_mutation.diff(&base_transient).diff().apply(&base_transient).unwrap();
    assert_eq!(after, next_transient);
    let restored = transient_mutation.inverse(&base_transient).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
    assert_eq!(restored, base_transient);
    assert_eq!(DrawingCanvasWindowTransient::parse_dsl(&base_transient.print_dsl()).unwrap(), base_transient);
    assert_eq!(DrawingCanvasWindowTransient::decode_pack(&base_transient.encode_pack()).unwrap(), base_transient);
    assert_eq!(DrawingCanvasWindowTransientMutation::parse_op(&transient_mutation.print_op()).unwrap(), transient_mutation);
    assert_eq!(DrawingCanvasWindowTransientMutation::decode_op(&transient_mutation.encode_op().unwrap()).unwrap(), transient_mutation);
    eprintln!("[DEBUG] Drawing Canvas config/transient matched neutral fixture, inverse, DSL, Pack, text-op, and binary-op laws");
}
