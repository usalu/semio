use super::*;
use crate::editor::remodeling::modes::analyze::windows::report::config::{self as report_config, RemodelingReportWindowConfig, RemodelingReportWindowConfigMutation, RemodelingReportWindowConfigOwner};
use crate::editor::remodeling::modes::capture::windows::frames::config::{self as frames_config, RemodelingFramesWindowConfig, RemodelingFramesWindowConfigMutation, RemodelingFramesWindowConfigOwner};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn block_on_remodel_windows<F: std::future::Future>(future: F) -> F::Output {
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

fn assert_config_codecs<S, M>(base: &S, mutation: &M)
where
    S: ArtifactDsl + ArtifactPack + MutationDiff<S> + Clone + std::fmt::Debug + PartialEq,
    M: Mutation<S, Diff = S> + OpText + OpBinary + Clone + std::fmt::Debug + PartialEq,
{
    let after = mutation.diff(base).diff().apply(base).unwrap();
    let restored = mutation.inverse(base).into_iter().fold(after.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
    assert_eq!(restored, *base);
    assert_eq!(M::parse_op(&mutation.print_op()).unwrap(), *mutation);
    assert_eq!(M::decode_op(&mutation.encode_op().unwrap()).unwrap(), *mutation);
    assert_eq!(S::parse_dsl(&after.print_dsl()).unwrap(), after);
    assert_eq!(S::decode_pack(&after.encode_pack()).unwrap(), after);
}

#[test]
fn remodel_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let model_base: RemodelingModelWindowConfig = dsl::json::from_json_str(&fixture["base"]["model"].to_string()).unwrap();
    let frames_base: RemodelingFramesWindowConfig = dsl::json::from_json_str(&fixture["base"]["frames"].to_string()).unwrap();
    let report_base: RemodelingReportWindowConfig = dsl::json::from_json_str(&fixture["base"]["report"].to_string()).unwrap();
    for row in fixture["mutations"].as_array().unwrap() {
        match row["owner"].as_str().unwrap() {
            "model" => assert_config_codecs(&model_base, &dsl::json::from_json_str::<RemodelingModelWindowConfigMutation>(&row["mutation"].to_string()).unwrap()),
            "frames" => assert_config_codecs(&frames_base, &dsl::json::from_json_str::<RemodelingFramesWindowConfigMutation>(&row["mutation"].to_string()).unwrap()),
            "report" => assert_config_codecs(&report_base, &dsl::json::from_json_str::<RemodelingReportWindowConfigMutation>(&row["mutation"].to_string()).unwrap()),
            owner => panic!("unknown fixture owner {owner}"),
        }
    }
    eprintln!("[DEBUG] Remodel Model, Frames, and Report window mutations matched the neutral fixture, inverse, text, binary, DSL, and Pack laws");
}

#[test]
fn remodel_window_ownership_runtime_isolates_renders_and_reopens_six_windows() {
    std::thread::Builder::new()
        .name("remodel-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| block_on_remodel_windows(async {
            use crate::editor::remodeling::commands::{import_frame_payload, set_camera, set_frame_cursor, set_layer_visibility, set_report_table};
            use crate::editor::remodeling::modes::{analyze, capture, model};
            use crate::editor::remodeling::{create_remodeling_app, RemodelingCommand, RemodelingPlayApp};
            use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

            async fn dispatch(app: &mut VcsArtifactApp<EditorApp<RemodelingPlayApp>>, command: RemodelingCommand, view: &ViewModel) -> Result<usize, String> {
                let command_id = command.command_id();
                let meta = ActionMeta { instance_id: 88, view_state: Some(view.clone()), ..artifact_app_laws::meta("remodel-window-ownership") };
                app.dispatch_typed(command, &meta).await.map_err(|error| format!("{command_id}: {error:?}"))?;
                let receipt = artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map_err(|error| format!("{command_id}: {error:?}"))?;
                Ok(receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count())
            }

            async fn render(app: &mut VcsArtifactApp<EditorApp<RemodelingPlayApp>>, body: &str, view: &ViewModel) -> Result<String, String> {
                let tree = app.render(body, None, view).await.map_err(|error| format!("{error:?}"))?;
                artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)
            }

            async fn model_scene(app: &mut VcsArtifactApp<EditorApp<RemodelingPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::World3dScene, String> {
                artifact_app_laws::decode_fixture_scene(&render(app, model::windows::model::REMODELING_PLAY_BODY_MAIN, view).await?).map_err(str::to_string)
            }

            async fn frames_scene(app: &mut VcsArtifactApp<EditorApp<RemodelingPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::Canvas2dScene, String> {
                artifact_app_laws::decode_fixture_scene(&render(app, capture::windows::frames::REMODELING_PLAY_BODY_FRAMES, view).await?).map_err(str::to_string)
            }

            async fn report_scene(app: &mut VcsArtifactApp<EditorApp<RemodelingPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::TableScene, String> {
                artifact_app_laws::decode_fixture_scene(&render(app, analyze::windows::report::REMODELING_PLAY_BODY_REPORT, view).await?).map_err(str::to_string)
            }

            let manifest = || App { definition: create_remodeling_app(), examples: Vec::new() };
            let roster = vec![
                ViewWindowInstance { id: "remodel-model-left".into(), window_kind_id: RemodelingModelWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "remodel-model-right".into(), window_kind_id: RemodelingModelWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "remodel-frames-left".into(), window_kind_id: RemodelingFramesWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "remodel-frames-right".into(), window_kind_id: RemodelingFramesWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "remodel-report-left".into(), window_kind_id: RemodelingReportWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "remodel-report-right".into(), window_kind_id: RemodelingReportWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "remodel-other".into(), window_kind_id: "other-window".into() },
            ];
            let view = ViewModel { window_instances: roster, ..Default::default() };
            let model_left = view.for_window_instance("remodel-model-left").unwrap();
            let model_right = view.for_window_instance("remodel-model-right").unwrap();
            let frames_left = view.for_window_instance("remodel-frames-left").unwrap();
            let frames_right = view.for_window_instance("remodel-frames-right").unwrap();
            let report_left = view.for_window_instance("remodel-report-left").unwrap();
            let report_right = view.for_window_instance("remodel-report-right").unwrap();
            let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<RemodelingPlayApp>>(manifest).await);
            app.bind_instance_id(88).await;
            let outcome: Result<(), String> = async {
                let payload = import_frame_payload::checker_data_url(8, 8, 2).await;
                dispatch(&mut app, RemodelingCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload, name: "left.png".into(), index: 0 }), &frames_left).await?;
                let stream_id = app.snapshot().map_err(|error| format!("{error:?}"))?.streams.first().ok_or("Remodel fixture stream was not admitted")?.id.clone();
                let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let publications =
                    dispatch(&mut app, RemodelingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport3dOrbit { position: [12.0, -8.0, 4.0], target: [1.0, 2.0, 3.0], zoom: 1.25, up: None } }), &model_left).await?
                    + dispatch(&mut app, RemodelingCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: "mesh".into(), visible: false }), &model_right).await?
                    + dispatch(&mut app, RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some(stream_id), frame_index: 0 }), &frames_left).await?
                    + dispatch(&mut app, RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("missing-stream".into()), frame_index: 9 }), &frames_right).await?
                    + dispatch(&mut app, RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: "cameras".into() }), &report_left).await?
                    + dispatch(&mut app, RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: "gcps".into() }), &report_right).await?;
                if publications != 6 { return Err(format!("Remodel exact-window lane count changed: {publications}")); }
                let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                if document_before.pack != document_after.pack || document_before.spr != document_after.spr { return Err("Remodel window commands changed document bytes".into()); }
                let left_model = model_scene(&mut app, &model_left).await?;
                let right_model = model_scene(&mut app, &model_right).await?;
                let left_frames = frames_scene(&mut app, &frames_left).await?;
                let right_frames = frames_scene(&mut app, &frames_right).await?;
                let left_report = report_scene(&mut app, &report_left).await?;
                let right_report = report_scene(&mut app, &report_right).await?;
                let left_camera: store::Viewport3dOrbit = dsl::json::from_json_str(&left_model.camera_json).map_err(|error| format!("{error:?}"))?;
                if left_model == right_model { return Err("Remodel Model rendered identical scenes for two isolated window configs".into()); }
                if left_camera != (store::Viewport3dOrbit { position: [12.0, -8.0, 4.0], target: [1.0, 2.0, 3.0], zoom: 1.25, up: None }) {
                    return Err(format!("Remodel Model rendered the wrong left camera: {left_camera:?}"));
                }
                if right_model.instances_json != "[]" { return Err(format!("Remodel Model rendered a mesh instance hidden by the right config: {}", right_model.instances_json)); }
                if !left_frames.layers_json.contains("data:image/png;base64") || right_frames.layers_json != "[]" { return Err("Remodel Frames render did not consume isolated cursors".into()); }
                if !left_report.columns_json.contains("RMS (px)") || !right_report.columns_json.contains("Observations") { return Err("Remodel Report render did not consume isolated table selection".into()); }
                let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                if packs.len() != 6 { return Err(format!("Remodel persisted {} exact window packs instead of six", packs.len())); }
                let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<RemodelingPlayApp>>(manifest).await);
                reopened.bind_instance_id(89).await;
                reopened.load_document_pack(&document_before).await.map_err(|error| format!("{error:?}"))?;
                for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                let reopened_model_left = model_scene(&mut reopened, &model_left).await?;
                let reopened_model_right = model_scene(&mut reopened, &model_right).await?;
                let reopened_frames_left = frames_scene(&mut reopened, &frames_left).await?;
                let reopened_frames_right = frames_scene(&mut reopened, &frames_right).await?;
                let reopened_report_left = report_scene(&mut reopened, &report_left).await?;
                let reopened_report_right = report_scene(&mut reopened, &report_right).await?;
                artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                if reopened_model_left != left_model || reopened_model_right != right_model || reopened_frames_left != left_frames || reopened_frames_right != right_frames || reopened_report_left != left_report || reopened_report_right != right_report {
                    return Err("Remodel exact window configuration changed during reopen".into());
                }
                let stale = ViewModel { window_id: Some("remodel-missing".into()), window_instances: view.window_instances.clone(), ..Default::default() };
                if addressed(&stale, RemodelingModelWindowConfig::default()).is_ok() { return Err("Remodel Model owner accepted stale window identity".into()); }
                if frames_config::addressed(&report_left, RemodelingFramesWindowConfig::default()).is_ok() { return Err("Remodel Frames owner accepted Report window identity".into()); }
                if report_config::addressed(&model_left, RemodelingReportWindowConfig::default()).is_ok() { return Err("Remodel Report owner accepted Model window identity".into()); }
                Ok(())
            }.await;
            if let Err(error) = &outcome { eprintln!("[DEBUG] Remodel exact-window runtime failure before close: {error}"); }
            artifact_app_laws::close_registered_fixture_app(&mut *app);
            outcome.expect("Remodel exact-window runtime law");
            eprintln!("[DEBUG] Remodel retained commands isolated, rendered, and reopened two concrete windows for each of three owners");
        }))
        .expect("spawn Remodel window ownership law")
        .join()
        .expect("Remodel window ownership law thread");
}
