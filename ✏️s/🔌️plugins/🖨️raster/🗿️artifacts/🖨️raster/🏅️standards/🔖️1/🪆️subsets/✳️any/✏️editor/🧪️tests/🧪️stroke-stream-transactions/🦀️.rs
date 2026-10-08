//! 🌊️ The streamed brush gesture end to end through the registry-backed app and its retained route: stream ticks carry
//! new samples into the Composite window's paint tool and never touch the document or its history; the release commits
//! ONE row stamped with the paint tool's `TransactionRef` holding ONE leaf of every sample, exactly the one-shot stroke of
//! the same samples; a host fact ends an open stroke with zero trace, and a late tick of a closed press is dropped.

use super::fill_tool_transactions::{blank_layer, document_rows, retire};
use super::unit_tests::context::{app, dispatch, raster_view_state, RasterAppFixture};
use super::*;
use crate::editor::raster::commands::paint_stroke::{paint_stroke_leaf, PaintStroke, RASTER_PAINT_TOOL_ID};
use crate::standards::v1::subsets::any::io::text::mutations::apply_raster_mutation;
use semio_framework_plugin::PluginApp;

fn press(layer: &str, phase: Option<&str>, gesture: &str, xs: &[f64], ys: &[f64]) -> RasterCommand {
    RasterCommand::PaintStroke(PaintStroke { layer_id: layer.into(), tool: "brush".into(), xs: xs.to_vec(), ys: ys.to_vec(), phase: phase.map(str::to_string), reason: None, gesture: Some(gesture.into()) })
}

/// 📨️ The host forwarding `kind` for the Composite test window.
async fn host_event(app: &mut RasterAppFixture, kind: &str) {
    let under = semio_framework_plugin::ActionMeta { view_state: Some(raster_view_state()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
    let window = raster_view_state().window_id.expect("the composite test window");
    let args = semio_framework_value::DslValue::from(&serde_json::json!({ "windowId": window, "kind": kind }));
    let admitted = app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&args), &under).await.unwrap_or_else(|fault| panic!("hostEvent {kind}: {fault:?}"));
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut **app, admitted).await.unwrap_or_else(|fault| panic!("hostEvent {kind} settles: {fault:?}"));
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut **app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await.unwrap_or_else(|fault| panic!("hostEvent {kind} answer settles: {fault:?}"));
}

#[semio_framework_async_macros::async_test]
async fn a_streamed_stroke_is_one_row_holding_every_sample_and_its_ticks_never_touch_the_document() {
    let mut app = app().await;
    let layer = blank_layer(&mut app).await;
    let base = app.snapshot().expect("the base the stroke lands on");
    let rows_before = document_rows(&mut app).await.len();
    dispatch(&mut app, press(&layer, Some("stream"), "press-1", &[0.5, 1.5], &[0.5, 0.5])).await;
    dispatch(&mut app, press(&layer, Some("stream"), "press-1", &[2.5], &[1.5])).await;
    assert_eq!(document_rows(&mut app).await.len(), rows_before, "stream ticks are never history");
    let streaming = app.snapshot().expect("the committed head while streaming");
    assert_eq!(streaming, base, "a tick never touches the committed document");
    dispatch(&mut app, press(&layer, Some("commit"), "press-1", &[3.5], &[2.5])).await;
    let rows = document_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 1, "the stroke is ONE row");
    let row = rows.last().expect("the stroke row");
    assert_eq!(row.transaction.as_ref().map(|transaction| transaction.tool.as_str()), Some(RASTER_PAINT_TOOL_ID));
    assert_eq!(row.mutations.len(), 1);
    let once = paint_stroke_leaf(&PaintStroke { layer_id: layer.clone(), tool: "brush".into(), xs: vec![0.5, 1.5, 2.5, 3.5], ys: vec![0.5, 0.5, 1.5, 2.5], phase: None, reason: None, gesture: None }, &base, &RasterConfig::default()).expect("the whole stroke paints");
    let fresh = apply_raster_mutation(&base, &once).expect("the one-shot stroke applies on its base");
    let head = app.snapshot().expect("the committed head");
    assert_eq!(head, fresh, "the streamed stroke is the one-shot stroke of every streamed sample");
    dispatch(&mut app, press(&layer, Some("stream"), "press-1", &[0.5], &[2.5])).await;
    dispatch(&mut app, press(&layer, Some("commit"), "press-1", &[1.5], &[2.5])).await;
    assert_eq!(document_rows(&mut app).await.len(), rows_before + 1, "a late tick of the committed press is dropped");
    for snapshot in [base, streaming, fresh, head] {
        retire(snapshot);
    }
}

#[semio_framework_async_macros::async_test]
async fn a_host_fact_ends_the_open_stroke_with_zero_trace() {
    let mut app = app().await;
    let layer = blank_layer(&mut app).await;
    let base = app.snapshot().expect("the base");
    let rows_before = document_rows(&mut app).await.len();
    dispatch(&mut app, press(&layer, Some("stream"), "press-2", &[0.5, 1.5], &[0.5, 0.5])).await;
    host_event(&mut app, "blur").await;
    dispatch(&mut app, press(&layer, Some("commit"), "press-2", &[2.5], &[0.5])).await;
    assert_eq!(document_rows(&mut app).await.len(), rows_before, "the blurred stroke and its late release leave no row");
    let head = app.snapshot().expect("the head");
    assert_eq!(head, base, "the blurred stroke leaves the document untouched");
    dispatch(&mut app, press(&layer, Some("stream"), "press-3", &[0.5], &[0.5])).await;
    dispatch(&mut app, press(&layer, Some("commit"), "press-3", &[1.5], &[0.5])).await;
    assert_eq!(document_rows(&mut app).await.len(), rows_before + 1, "the next press paints again");
    for snapshot in [base, head] {
        retire(snapshot);
    }
}
