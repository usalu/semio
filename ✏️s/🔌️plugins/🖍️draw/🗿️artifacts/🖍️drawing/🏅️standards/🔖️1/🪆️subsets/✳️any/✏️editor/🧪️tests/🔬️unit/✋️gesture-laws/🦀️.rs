//! ✋️ Drawing editor unit laws over gestures: rectangle drag, pen draft, escape cancel, direct drag publication and repeated shape gestures.

use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

#[semio_framework_async_macros::async_test]
async fn shape_rect_drag_commits_one_layer_and_requests_utility_reset() {
    let mut app = drawing_app().await;
    let utility_meta = meta_with_utility("shapeRect");
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 500.0,
            y: 400.0,
            width: 1000.0,
            height: 800.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 600.0, y: 500.0, width: 1000.0, height: 800.0, samples: Vec::new() }), &utility_meta).await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false, cancelled: false }), &utility_meta).await;
    assert!(result.mutations.is_empty(), "the migrated gesture retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().unwrap();
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Shape(shape) if shape.shape_kind == "rect")));
    assert!(
        matches!(
            receipt.effects.as_slice(),
            [Effect::SetActiveUtility { window_id, utility_id }] if window_id == DRAWING_PLAY_WINDOW_CANVAS && utility_id == "selectDirect"
        ),
        "the canvas returns to select-direct via a host effect, not a document operation"
    );
}

#[semio_framework_async_macros::async_test]
async fn pen_draft_commits_path_layer_on_enter() {
    let mut app = drawing_app().await;
    let utility_meta = meta_with_utility("pen");
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 400.0,
            y: 300.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await;
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 500.0,
            y: 300.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await;
    let (result, receipt) = settled(&mut app, DrawingCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}), &utility_meta).await;
    assert!(result.mutations.is_empty(), "the migrated gesture retains its operation until publication");
    assert_one_artifact_publication(&receipt);
    let projection = app.snapshot().unwrap();
    assert!(projection.layers.iter().any(|layer| matches!(layer, DrawingLayerNode::Path(path) if !path.segments.is_empty())));
    let created=projection.layers.last().unwrap();
    assert_eq!(selected_strokes(&app).await,vec![layer_id(created).to_string()]);
    assert!(semio_s_artifact_draw_drawing_model::layer_base(created).attributes.stroke.as_ref().is_some_and(|stroke|stroke.width>0.0));
    assert!(matches!(receipt.effects.as_slice(), [Effect::SetActiveUtility { utility_id, .. }] if utility_id == "selectDirect"));
}

#[semio_framework_async_macros::async_test]
async fn canvas_escape_cancels_draft_without_committing() {
    let mut app = drawing_app().await;
    let before = app.snapshot().unwrap().layers.len();
    let utility_meta = meta_with_utility("pen");
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 400.0,
            y: 300.0,
            width: 800.0,
            height: 600.0,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &utility_meta,
    )
    .await;
    let (result, _) = settled(&mut app, DrawingCommand::CanvasEscape(canvas_escape::CanvasEscape {}), &utility_meta).await;
    assert!(result.mutations.is_empty());
    assert_eq!(app.snapshot().unwrap().layers.len(), before);
}

#[semio_framework_async_macros::async_test]
async fn direct_drag_projects_without_editing_and_publishes_only_on_release() {
    let surface_behavior: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🎬️surface-behavior/🔣️.json")).expect("shared surface behavior fixture");
    let law = surface_behavior["cases"].as_array().unwrap().iter().find(|entry| entry["id"] == "draw-canvas2d").expect("Draw surface behavior");
    assert_eq!(law["appId"], "s.draw.drawing@1/*#editor");
    assert_eq!(law["terminalPolicy"], "cancelled-action-discards-draft");
    assert_eq!(law["publishedOnCancel"][0]["action"], "canvasPointerUp");
    assert_eq!(law["publishedOnCancel"][0]["cancelled"], true);
    for cancelled in [true, false] {
        let (mut app, mut meta) = inline_selection_app().await;
        std::sync::Arc::make_mut(meta.view_state.as_mut().unwrap()).active_utility_id = Some("selectDirect".into());
        let mut layer = semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Drag target")).to_string().into()).expect("nonempty authored identity"), "Drag target");
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).attributes.fill=Some(semio_s_artifact_draw_drawing_model::FillStyle::Solid{color:[0.0,0.0,0.0,1.0]});
        let id = layer_id(&layer).to_string();
        let snapshot = DrawingSnapshot { id: "direct-drag".into(), layers: vec![layer].into(), ..Default::default() };
        load_drawing_fixture(&mut app, &snapshot).await;
        settled(&mut app, DrawingCommand::SetCamera(set_camera::SetCamera { camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta).await;
        let before = app.snapshot().unwrap();
        let view = meta.view_state.as_ref().unwrap();
        let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, view).await.unwrap());
        let records: Vec<serde_json::Value> = serde_json::from_str(&scene.layers_json).unwrap();
        let original = records.iter().find(|record| record["id"] == id).unwrap()["transform"].clone();
        let (_, down) = settled(&mut app, DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, ..Default::default() }), &meta).await;
        assert!(!down.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
        assert_eq!(app.snapshot().unwrap(), before);
        let (_, moved) = settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 430.0, y: 320.0, width: 800.0, height: 600.0, samples: vec![[410.0,310.0],[430.0,320.0]] }), &meta).await;
        assert!(!moved.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
        assert_eq!(app.snapshot().unwrap(), before);
        let scene = canvas_scene(rendered_drawing_canvas(&mut app, None, view).await.unwrap());
        let records: Vec<serde_json::Value> = serde_json::from_str(&scene.layers_json).unwrap();
        let preview = &records.iter().find(|record| record["id"] == id).unwrap()["transform"];
        assert_eq!(preview[4].as_f64().unwrap(), original[4].as_f64().unwrap() + 30.0);
        assert_eq!(preview[5].as_f64().unwrap(), original[5].as_f64().unwrap() + 20.0);
        let (_, released) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 430.0, y: 320.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled }), &meta).await;
        if cancelled {
            assert!(!released.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
            assert_eq!(app.snapshot().unwrap(), before);
            assert_eq!(law["artifactAfterCancel"], "unchanged");
            let (_, stale_move) = settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 450.0, y: 340.0, width: 800.0, height: 600.0, samples: vec![[450.0,340.0]] }), &meta).await;
            let (_, stale_up) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 450.0, y: 340.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &meta).await;
            assert!(!stale_move.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
            assert!(!stale_up.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
            assert_eq!(app.snapshot().unwrap(), before, "a stale move/up that crosses the renderer fence still cannot mutate the Draw document");
            settled(&mut app, DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, ..Default::default() }), &meta).await;
            settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 430.0, y: 320.0, width: 800.0, height: 600.0, samples: vec![[430.0,320.0]] }), &meta).await;
            let (_, fresh) = settled(&mut app, DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 430.0, y: 320.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false, cancelled: false }), &meta).await;
            assert_one_artifact_publication(&fresh);
            let after = app.snapshot().unwrap();
            let transform = &semio_s_artifact_draw_drawing_model::layer_base(&after.layers[0]).transform;
            assert!((transform.x-30.0).abs()<1e-10&&(transform.y-20.0).abs()<1e-10);
        } else {
            assert_one_artifact_publication(&released);
            assert_eq!(law["freshGestureResult"], "one-artifact-publication");
            let after = app.snapshot().unwrap();
            let transform = &semio_s_artifact_draw_drawing_model::layer_base(&after.layers[0]).transform;
            assert!((transform.x-30.0).abs()<1e-10);
            assert!((transform.y-20.0).abs()<1e-10);
        }
    }
}

fn drawing_composite_shape_meta() -> semio_plugin::ActionMeta {
    use semio_plugin::{ViewSessionIdentity, ViewWindowInstance};
    let view = ViewModel {
        active_utility_by_window_id: [(DRAWING_PLAY_WINDOW_CANVAS.to_string(), Some("shapeRect".to_string()))].into(),
        window_id: Some(DRAWING_PLAY_WINDOW_CANVAS.into()),
        active_window_kind_id: Some(DRAWING_PLAY_WINDOW_CANVAS.into()),
        active_utility_id: Some("shapeRect".into()),
        focused_window_id: Some(DRAWING_PLAY_WINDOW_CANVAS.into()),
        window_instances: vec![ViewWindowInstance { id: DRAWING_PLAY_WINDOW_CANVAS.into(), window_kind_id: DRAWING_PLAY_WINDOW_CANVAS.into() }],
        session_identity: Some(ViewSessionIdentity { user_id: "draw-repeat-owner".into(), display_name: "Draw Repeat Owner".into() }),
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    semio_plugin::ActionMeta { view_state: Some(view.into()), ..artifact_laws::meta("local") }
}

fn drawing_gesture_observation(meta: &semio_plugin::ActionMeta, receipt: &artifact_laws::TypedOperationFixtureReceipt) -> String {
    let view = meta.view_state.as_ref().expect("drawing composite view");
    format!(
        "utility={} window={:?} session={:?} operations={:?} revisions={:?} lanes={:?} completions={}",
        drawing_active_utility(view),
        view.window_id,
        view.session_identity,
        receipt.completion_operations,
        receipt.revisions,
        receipt.lanes,
        receipt.completions
    )
}

fn has_direct_select_reset(receipt: &artifact_laws::TypedOperationFixtureReceipt) -> bool {
    receipt.effects.iter().any(|effect| {
        matches!(effect, Effect::SetActiveUtility { window_id, utility_id } if window_id == DRAWING_PLAY_WINDOW_CANVAS && utility_id == DRAWING_DEFAULT_UTILITY)
    })
}

/// 🖱️ Draw10's real retained-owner order: idle hover, rectangle drag, published revision, a fresh
/// per-window view rearm, then one non-overlapping rectangle drag through the same production owner.
#[semio_framework_async_macros::async_test]
async fn repeated_shape_rect_gestures_from_fresh_published_views_commit_distinct_layers_and_reset_twice() {
    let mut app = drawing_app().await;
    let first_meta = drawing_composite_shape_meta();
    let first_view = first_meta.view_state.as_ref().expect("first drawing-composite view");
    let initial_tree = rendered_drawing_canvas(&mut app, None, first_view).await.expect("initial drawing-composite render");
    drop(initial_tree);
    let before = app.snapshot().expect("initial Drawing snapshot").layers.len();
    let width = 1587.0;
    let height = 907.0;

    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 396.75, y: 317.45, width, height, samples: Vec::new() }), &first_meta).await;
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 396.75,
            y: 317.45,
            width,
            height,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &first_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 603.06, y: 453.5, width, height, samples: Vec::new() }), &first_meta).await;
    let (_first_up, first_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 603.06, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
        &first_meta,
    )
    .await;
    let first_observation = drawing_gesture_observation(&first_meta, &first_receipt);
    let first_layer_count = app.snapshot().expect("first published Drawing snapshot").layers.len();
    let first_reset = has_direct_select_reset(&first_receipt);

    let second_meta = drawing_composite_shape_meta();
    let second_view = second_meta.view_state.as_ref().expect("fresh post-publication drawing-composite view");
    let refreshed_tree = rendered_drawing_canvas(&mut app, None, second_view).await.expect("post-publication drawing-composite render");
    drop(refreshed_tree);
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 825.24, y: 317.45, width, height, samples: Vec::new() }), &second_meta).await;
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 825.24,
            y: 317.45,
            width,
            height,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &second_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 1031.55, y: 453.5, width, height, samples: Vec::new() }), &second_meta).await;
    let (_second_up, second_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 1031.55, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
        &second_meta,
    )
    .await;
    let second_observation = drawing_gesture_observation(&second_meta, &second_receipt);
    let second_reset = has_direct_select_reset(&second_receipt);
    let second_layer_count = app.snapshot().expect("twice-published Drawing snapshot").layers.len();

    let third_meta = drawing_composite_shape_meta();
    let third_view = third_meta.view_state.as_ref().expect("fresh identical-geometry drawing-composite view");
    let third_tree = rendered_drawing_canvas(&mut app, None, third_view).await.expect("identical-geometry drawing-composite render");
    drop(third_tree);
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 825.24, y: 317.45, width, height, samples: Vec::new() }), &third_meta).await;
    settled(
        &mut app,
        DrawingCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
            x: 825.24,
            y: 317.45,
            width,
            height,
            shift: false,
            ctrl: false,
            meta: false,
            generation: None,
            checkpoint_completed_work: None,
            checkpoint_pending_work: None,
            ..Default::default()
        }),
        &third_meta,
    )
    .await;
    settled(&mut app, DrawingCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { shift: false, alt: false, x: 1031.55, y: 453.5, width, height, samples: Vec::new() }), &third_meta).await;
    let (_third_up, third_receipt) = settled(
        &mut app,
        DrawingCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { alt: false, x: 1031.55, y: 453.5, width, height, shift: false, ctrl: false, meta: false, cancelled: false }),
        &third_meta,
    )
    .await;
    let third_observation = drawing_gesture_observation(&third_meta, &third_receipt);
    let third_reset = has_direct_select_reset(&third_receipt);

    let projection = app.snapshot().expect("three-times-published Drawing snapshot");
    artifact_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
    retire_owned_fixture_view(first_meta);
    retire_owned_fixture_view(second_meta);
    retire_owned_fixture_view(third_meta);
    assert_eq!(first_layer_count, before + 1, "first rectangle publication: {first_observation}");
    assert!(first_reset, "first PointerUp returns its exact drawing-composite utility to Direct Select: {first_observation}");
    assert!(second_reset, "second PointerUp returns its exact drawing-composite utility to Direct Select: {second_observation}");
    assert_eq!(second_layer_count, before + 2, "two retained-owner gestures publish two layers; first=[{first_observation}] second=[{second_observation}]");
    assert!(third_reset, "identical-geometry PointerUp returns its exact drawing-composite utility to Direct Select: {third_observation}");
    assert_eq!(projection.layers.len(), before + 3, "a separate identical-geometry creation still publishes a third layer; second=[{second_observation}] third=[{third_observation}]");
    let [DrawingLayerNode::Shape(first), DrawingLayerNode::Shape(second), DrawingLayerNode::Shape(third)] = [projection.layers.get(before).unwrap(),projection.layers.get(before+1).unwrap(),projection.layers.get(before+2).unwrap()] else {
        panic!("all retained-owner additions are shape layers; first=[{first_observation}] second=[{second_observation}] third=[{third_observation}]")
    };
    let first_rect = first.rect.as_ref().expect("first rectangle geometry");
    let second_rect = second.rect.as_ref().expect("second rectangle geometry");
    let third_rect = third.rect.as_ref().expect("identical rectangle geometry");
    assert_eq!((first.shape_kind.eq_str("rect"), second.shape_kind.eq_str("rect"), third.shape_kind.eq_str("rect")), (true, true, true));
    assert_ne!(first.base.id, second.base.id, "two commits own distinct layer identities; first=[{first_observation}] second=[{second_observation}]");
    assert!(first_rect.x + first_rect.width < second_rect.x, "the two physical coordinate ranges remain non-overlapping; first={first_rect:?} second={second_rect:?}");
    assert_eq!(second_rect, third_rect, "the third creation intentionally repeats the second geometry");
    assert_ne!(second.base.id, third.base.id, "separate identical-geometry creations own distinct identities; second=[{second_observation}] third=[{third_observation}]");
}
