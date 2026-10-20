//! 🪟️ Drawing editor unit laws over windows: direct pick, selection opacity, labels, window status and the declared io.

use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_plugin::ArtifactView;
use semio_framework::MediaPayload;

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_up_direct_pick_selects_inline() {
    let (mut app, meta) = inline_selection_app().await;
    // 🔖 The default document's one layer is an empty-segment path (no bounds to hit-test against
    let initial_id = first_layer_id(&app);
    settled(&mut app, DrawingCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &meta).await;
    settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &meta).await;
    let rect_id = last_layer_id(&app);
    settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta).await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 110.0, y: 110.0, width: 200.0, height: 200.0, shift: false, ctrl: false, meta: false, cancelled: false }), &meta).await;
    assert!(result.mutations.is_empty(), "a direct pick is not a document operation");
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { .. })), "interactionSelect is folded in-reactor, never handed to the host: {:?}", receipt.effects);
    assert_eq!(selected_strokes(&app).await, vec![rect_id], "the picked rect is selected inside the carrying operation");
    artifact_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn set_selected_opacity_reads_the_framework_interaction_selection() {
    let (mut app, meta) = inline_selection_app().await;
    let id = first_layer_id(&app);
    let targets = serde_json::to_string(&vec![serde_json::json!({ "granularity": DRAWING_INTERACTION_GRANULARITY, "id": id })]).unwrap();
    let admission = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "domainId": DRAWING_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" }))), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("select");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admission).await.expect("selection publication settles");
    assert_eq!(selected_strokes(&app).await, vec![id.clone()], "the retained opacity command reads the framework-owned selection published for its canvas window");
    let (result, receipt) = settled(&mut app, DrawingCommand::SetSelectedOpacity(set_selected_opacity::SetSelectedOpacity { value: 0.25 }), &meta).await;
    assert!(result.mutations.is_empty(), "the migrated route retains its operation until publication");
    assert_artifact_publication_units(&receipt, 2);
    assert!((semio_s_artifact_draw_drawing_model::layer_base(&app.snapshot().unwrap().layers[0]).opacity - 0.25).abs() < f64::EPSILON);
}

#[semio_framework_async_macros::async_test]
async fn drawing_labels_resolve_native_by_default() {
    let mut app = drawing_app().await;
    let json = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, DRAWING_PLAY_BODY_LAYERS, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("retire semantic tree");
    assert!(json.contains("Add Path"));
    assert!(json.contains("Add Rectangle"));
    assert!(!json.contains("Pfad hinzufügen"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_labels_translate_panels_in_german() {
    let mut app = drawing_app().await;
    let view_state = ViewModel { locale: semio_framework_ui_locale::Locale::De, ..ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) };
    let layers_json = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, DRAWING_PLAY_BODY_LAYERS, &view_state, app.1.cx()).await.expect("retire layers tree");
    assert!(layers_json.contains("Pfad hinzufügen"));
    assert!(layers_json.contains("Rechteck hinzufügen"));
    assert!(!layers_json.contains("Add Path"));
    let catalogue_json = semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, DRAWING_PLAY_BODY_CATALOGUE, &view_state, app.1.cx()).await.expect("retire catalogue tree");
    assert!(catalogue_json.contains("\"Ellipse\""));
    assert!(catalogue_json.contains("Nachzeichnung"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_window_status_tracks_live_selection_in_both_explicit_locales() {
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧮️status/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut witnesses=0;
    for locale in [semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Locale::De] {
        let language=if locale==semio_framework_ui_locale::Locale::En {"en"} else {"de"};
        let labels=law["labels"].as_array().unwrap().iter().find(|row|row["locale"]==language).unwrap();
        for terminology in [semio_framework_ui_locale::Terminology::Native,semio_framework_ui_locale::Terminology::Reuse] {
            for row in law["cases"].as_array().unwrap() {
                let (mut app,mut meta)=inline_selection_app().await;
                let view=meta.view_state.as_mut().unwrap();view.locale=locale;view.terminology=terminology;
                let layers=(0..row["layers"].as_u64().unwrap()).map(|index|{let mut layer=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Status")).to_string().into()).expect("nonempty authored identity"), "Status");semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).id=format!("layer-{index}").into();layer}).collect::<Vec<_>>();
                let snapshot=DrawingSnapshot{id:"selection-status".into(),layers:layers.into_iter().collect(),..Default::default()};load_drawing_fixture(&mut app,&snapshot);
                for (step,selection) in row["selections"].as_array().unwrap().iter().enumerate() {
                    let targets=selection.as_array().unwrap().iter().map(|index|serde_json::json!({"granularity":DRAWING_INTERACTION_GRANULARITY,"id":format!("layer-{}",index.as_u64().unwrap())})).collect::<Vec<_>>();
                    let args=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"domainId":DRAWING_INTERACTION_DOMAIN,"targets":serde_json::to_string(&targets).unwrap(),"merge":"replace","method":"pick"}));
                    let admission=semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(args.clone()), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
                    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admission).await.unwrap();
                    assert_eq!(selected_strokes(&app).await.len(),selection.as_array().unwrap().len());
                    let engagements=app.window_engagements(meta.view_state.as_ref().unwrap()).await;
                    let engagement=engagements.get("drawing-canvas").expect("addressed canvas chrome");
                    assert_eq!(engagement.status.as_ref().unwrap()[0].text,row["expected"][language][step].as_str().unwrap(),"{}:{language}:{step}",row["id"]);
                    assert_eq!(engagement.input.as_ref().unwrap().placeholder.as_deref(),labels["layerName"].as_str());
                    assert_eq!(app.snapshot().unwrap(),snapshot);
                    witnesses+=1;
                }
                let unaddressed=ViewModel::new(locale,terminology);
                assert!(app.window_engagements(&unaddressed).await.is_empty());
                artifact_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");assert!(app.close_terminal_is_empty());
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn drawing_io_declares_vector_out_and_export_media_covers_both_ports() {
    let mut app = drawing_app().await;
    let meta = artifact_laws::meta("local");
    let (_, receipt) = settled(&mut app, DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &meta).await;
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::with_operation(&projection, &history,semio_s_artifact_draw_drawing_editor::identity_test::operation());
    let vector = DrawingPlayApp::export_media("vector:out", &doc).expect("vector:out");
    let MediaPayload::Structured { schema, json } = vector.payload else { panic!("expected structured svg payload") };
    assert_eq!(schema, "2d.drawing");
    assert!(json.starts_with("<svg"));
    assert!(DrawingPlayApp::export_media("artifact:out", &doc).is_ok());
    assert!(matches!(DrawingPlayApp::export_media("unknown:out", &doc), Err(MediaError::NotImplemented)));
}
