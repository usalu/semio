use super::*;
use crate::editor::remodeling::commands::{add_stream, import_abort, import_frame_payload, import_video_bytes_payload, import_video_done, remove_stream, set_stream_sync};
use crate::editor::remodeling::modes::{capture, model};
use crate::editor::remodeling::unit_tests::context::{app, dispatch, test_window, RemodelingApp};
use semio_framework_plugin::artifact_app_laws::{meta, settle_history_verb, settle_registered_typed_operation};
use semio_framework_plugin::PluginApp;
use crate::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn import_frame_payload_creates_a_stream_and_asset() {
    let mut app = app().await;
    verify_import_checker_stream(&mut app, 3).await;
    let scene = app.snapshot().expect("projection");
    assert_eq!(scene.streams.len(), 1, "one importFrames batch creates exactly one stream");
    assert_eq!(scene.streams[0].frames.len(), 3);
    assert_eq!(scene.assets.len(), 3);
}

/// 🎞️ In-process video import (the `ImportVideoBytesPayload` fallback path): a tiny synthesized
/// MJPEG mp4 must decode into a new video stream whose frame count matches what was muxed in.
#[semio_framework_async_macros::async_test]
#[cfg(feature = "video-mp4")]
async fn import_video_bytes_payload_extracts_frames_in_process() {
    let mut app = app().await;
    // 🎯️ `IngestParams::default().frame_sample_stride == 5`; force stride 1 so all 5 synthesized
    // frames are kept (a stride-sampling test belongs to the video engine topic file, not here).
    dispatch(&mut app, RemodelingCommand::SetIngestParams(crate::editor::remodeling::commands::set_ingest_params::SetIngestParams { frame_sample_stride: 1, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 })).await;
    dispatch(&mut app, RemodelingCommand::ImportVideoBytesPayload(import_video_bytes_payload::ImportVideoBytesPayload { payload: checker_video_data_url(5, 32, 32, 4).await, name: "clip.mp4".into() })).await;
    let scene = app.snapshot().expect("projection");
    assert_eq!(scene.streams.len(), 1);
    assert_eq!(scene.streams[0].kind, MediaKind::Video);
    assert_eq!(scene.streams[0].frames.len(), 5);
    assert_eq!(scene.assets.len(), 5);
}

/// 🎞️ Host-decoded video import path: `ImportVideoFramePayload` ticks followed by `ImportVideoDone`
/// must accumulate into one stream and write `VideoSource` provenance.
#[semio_framework_async_macros::async_test]
async fn import_video_frame_payload_then_done_writes_one_stream_with_video_source() {
    let mut app = app().await;
    for index in 0..4u32 {
        dispatch(&mut app, RemodelingCommand::ImportVideoFramePayload(ImportVideoFramePayload { payload: checker_data_url_jpeg(24, 24, 3).await, name: "clip.mp4".into(), index, frame_index: index, timestamp_ms: f64::from(index) * 100.0 })).await;
    }
    dispatch(&mut app, RemodelingCommand::ImportVideoDone(import_video_done::ImportVideoDone { name: "clip.mp4".into(), duration_ms: 400.0, frame_count: 4, width: 24, height: 24, codec: "mjpeg".into() })).await;
    let scene = app.snapshot().expect("projection");
    assert_eq!(scene.streams.len(), 1);
    assert_eq!(scene.streams[0].kind, MediaKind::Video);
    assert!(scene.streams[0].source.is_some());
    assert_eq!(scene.streams[0].source.as_ref().expect("video source").frame_count, 4);
}

#[semio_framework_async_macros::async_test]
async fn add_remove_and_sync_streams_edit_the_stream_list() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::AddStream(add_stream::AddStream { name: "Front".into(), kind: "video".into(), camera_id: String::new() })).await;
    let stream_id = app.snapshot().expect("projection").streams[0].id.clone();
    dispatch(&mut app, RemodelingCommand::SetStreamSync(set_stream_sync::SetStreamSync { stream_id: stream_id.clone(), sync_offset_ms: 12.5 })).await;
    assert_eq!(app.snapshot().expect("projection").streams[0].sync_offset_ms, 12.5);
    dispatch(&mut app, RemodelingCommand::RemoveStream(remove_stream::RemoveStream { stream_id })).await;
    assert!(app.snapshot().expect("projection").streams.is_empty());
}

//#region 🧾️ImportTransaction
async fn video_tick(app: &mut RemodelingApp, index: u32) {
    dispatch(app, RemodelingCommand::ImportVideoFramePayload(ImportVideoFramePayload { payload: checker_data_url_jpeg(24, 24, 3).await, name: "clip.mp4".into(), index, frame_index: index, timestamp_ms: f64::from(index) * 100.0 })).await;
}

async fn video_done(app: &mut RemodelingApp) {
    dispatch(app, RemodelingCommand::ImportVideoDone(import_video_done::ImportVideoDone { name: "clip.mp4".into(), duration_ms: 300.0, frame_count: 3, width: 24, height: 24, codec: "mjpeg".into() })).await;
}

fn shape(app: &RemodelingApp) -> (usize, usize, usize) {
    let scene = app.snapshot().expect("projection");
    (scene.streams.len(), scene.streams.iter().map(|stream| stream.frames.len()).sum(), scene.assets.len())
}

/// 🧾️ Every tick, the commit and the abort of one import name ONE transaction minted from the stream it builds, stamped
/// with the import tool; two streams are two transactions.
#[semio_framework_async_macros::async_test]
async fn an_import_names_one_transaction_minted_from_its_stream() {
    let transaction = import_transaction("stream-7");
    assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
    assert_eq!(transaction.tool, REMODELING_IMPORT_TOOL_ID);
    assert_eq!(import_transaction("stream-7"), transaction);
    assert_ne!(import_transaction("stream-8").id, transaction.id);
}

/// 📥️ A still import of three files is ONE edit: one undo removes the whole stream with its assets, one redo restores it.
#[semio_framework_async_macros::async_test]
async fn a_still_import_is_one_edit_one_undo_and_one_redo() {
    let mut app = app().await;
    verify_import_checker_stream(&mut app, 3).await;
    assert_eq!(shape(&app), (1, 3, 3));
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
    assert_eq!(shape(&app), (0, 0, 0), "one undo reverts the whole import");
    settle_history_verb(&mut *app, "redo", meta("local").instance_id).await;
    assert_eq!(shape(&app), (1, 3, 3), "one redo restores the whole import");
}

/// 🎞️ A host-decoded video import shows every streamed frame at once, and its done commits frames plus provenance as
/// ONE edit: one undo removes all of it.
#[semio_framework_async_macros::async_test]
async fn a_video_import_streams_its_frames_and_commits_one_edit() {
    let mut app = app().await;
    for index in 0..3u32 {
        video_tick(&mut app, index).await;
        assert_eq!(shape(&app).1, index as usize + 1, "the open edit shows frame {index} at once");
    }
    video_done(&mut app).await;
    assert!(app.snapshot().expect("projection").streams[0].source.is_some());
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
    assert_eq!(shape(&app), (0, 0, 0), "one undo reverts the whole video import");
}

/// 🚧️ While an import streams, every other document verb of the app is refused with `toolTransaction.open`; the import
/// goes on and commits.
#[semio_framework_async_macros::async_test]
async fn an_open_import_refuses_every_other_document_verb() {
    let mut app = app().await;
    video_tick(&mut app, 0).await;
    let admitted = app.dispatch_typed(RemodelingCommand::AddStream(add_stream::AddStream { name: "Side".into(), kind: "video".into(), camera_id: String::new() }), &meta("local")).await;
    let refusal = match admitted {
        Err(fault) => fault,
        Ok(_) => settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.err().expect("a foreign edit while the import is open is refused"),
    };
    assert!(format!("{refusal:?}").contains("toolTransaction.open"), "{refusal:?}");
    video_tick(&mut app, 1).await;
    video_done(&mut app).await;
    assert_eq!(shape(&app), (1, 2, 2));
}

/// 🛑️ An aborted import leaves zero trace: the document is the one before it, no edit is left to undo, and the next verb
/// is admitted again.
#[semio_framework_async_macros::async_test]
async fn an_aborted_import_leaves_zero_trace() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::AddStream(add_stream::AddStream { name: "Front".into(), kind: "video".into(), camera_id: String::new() })).await;
    let before = app.snapshot().expect("projection");
    video_tick(&mut app, 0).await;
    video_tick(&mut app, 1).await;
    assert_eq!(shape(&app), (2, 2, 2), "the open import shows its frames");
    dispatch(&mut app, RemodelingCommand::ImportAbort(import_abort::ImportAbort { reason: None })).await;
    assert_eq!(app.snapshot().expect("projection"), before, "the abort reverts the open import");
    video_tick(&mut app, 2).await;
    video_done(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), before, "a tick and a done arriving after the abort are dropped, never appended to another stream");
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
    assert_eq!(shape(&app), (0, 0, 0), "the only edit left to undo is the stream added before the import");
    dispatch(&mut app, RemodelingCommand::ImportAbort(import_abort::ImportAbort { reason: None })).await;
    assert_eq!(shape(&app), (0, 0, 0), "an abort with nothing open is no edit");
}
/// 🪟️ A second window of the test view: closing it must leave the test window's import alone.
const OTHER_WINDOW: &str = "remodeling-other-window";

/// 📨️ The host forwarding `kind` for `window_id`, under a view holding the import's Frames test window and a Model window.
async fn host_event(app: &mut RemodelingApp, window_id: &str, kind: &str) {
    let mut view = test_window(capture::windows::frames::REMODELING_PLAY_WINDOW_FRAMES);
    view.window_instances.push(semio_framework_plugin::ViewWindowInstance { id: OTHER_WINDOW.into(), window_kind_id: model::windows::model::REMODELING_PLAY_WINDOW_MAIN.into() });
    let under = semio_framework_plugin::ActionMeta { view_state: Some(view), ..meta("local") };
    let args = dsl::DslValue::from(&serde_json::json!({ "windowId": window_id, "kind": kind }));
    let admitted = app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&args), &under).await.unwrap_or_else(|fault| panic!("hostEvent {kind}: {fault:?}"));
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut **app, admitted).await.unwrap_or_else(|fault| panic!("hostEvent {kind} settles: {fault:?}"));
    settle_registered_typed_operation(&mut **app, meta("local").instance_id).await.unwrap_or_else(|fault| panic!("hostEvent {kind} answer settles: {fault:?}"));
}

/// 🪟️ Closing the window an import streams in aborts it with zero trace: the document is the one before it, a late tick
/// and a late done are dropped, and the only edit left to undo is the one before the import.
#[semio_framework_async_macros::async_test]
async fn the_importing_window_closing_aborts_its_import_with_zero_trace() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::AddStream(add_stream::AddStream { name: "Front".into(), kind: "video".into(), camera_id: String::new() })).await;
    let before = app.snapshot().expect("projection");
    video_tick(&mut app, 0).await;
    video_tick(&mut app, 1).await;
    assert_eq!(shape(&app), (2, 2, 2), "the open import shows its frames");
    host_event(&mut app, "remodeling-test-window", "retiring").await;
    assert_eq!(app.snapshot().expect("projection"), before, "the closing window aborted its import");
    video_tick(&mut app, 2).await;
    video_done(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), before, "the host's late tick and done are dropped");
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
    assert_eq!(shape(&app), (0, 0, 0), "the only edit left to undo is the stream added before the import");
}

/// 🪟️ Another window closing, a blur or a lost capture leaves an import running; it commits as one edit.
#[semio_framework_async_macros::async_test]
async fn other_host_facts_leave_an_import_running() {
    let mut app = app().await;
    video_tick(&mut app, 0).await;
    host_event(&mut app, OTHER_WINDOW, "retiring").await;
    host_event(&mut app, "remodeling-test-window", "blur").await;
    host_event(&mut app, "remodeling-test-window", "captureLost").await;
    assert_eq!(shape(&app), (1, 1, 1), "the import is still open");
    video_tick(&mut app, 1).await;
    video_done(&mut app).await;
    assert_eq!(shape(&app), (1, 2, 2));
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
    assert_eq!(shape(&app), (0, 0, 0), "the import committed as one edit");
}

/// 🖼️ A multi-file pick whose first file does not decode mints its OWN stream at the first file that does — it never
/// appends to a stream an earlier edit created — and still commits as one edit.
#[semio_framework_async_macros::async_test]
async fn a_pick_whose_first_file_does_not_decode_mints_its_own_stream() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::AddStream(add_stream::AddStream { name: "Front".into(), kind: "video".into(), camera_id: String::new() })).await;
    let still = |payload: String, index: u32| RemodelingCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload, name: format!("frame-{index}.png"), index, total: 3 });
    dispatch(&mut app, still("not-a-data-url".into(), 0)).await;
    dispatch(&mut app, still(import_frame_payload::checker_data_url(24, 24, 3).await, 1)).await;
    dispatch(&mut app, still(import_frame_payload::checker_data_url(24, 24, 3).await, 2)).await;
    let scene = app.snapshot().expect("projection");
    assert_eq!(scene.streams.len(), 2);
    assert!(scene.streams[0].frames.is_empty(), "the earlier stream is untouched");
    assert_eq!(scene.streams[1].frames.len(), 2);
    settle_history_verb(&mut *app, "undo", meta("local").instance_id).await;
    assert_eq!(shape(&app), (1, 0, 0), "one undo reverts the whole pick");
}

/// 🚫️ A multi-file pick dispatched without a window is refused at its first file: its tool state needs the picking window.
#[semio_framework_async_macros::async_test]
async fn a_windowless_multi_file_pick_is_refused() {
    let mut app = app().await;
    let first = RemodelingCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload: import_frame_payload::checker_data_url(24, 24, 3).await, name: "frame-0.png".into(), index: 0, total: 2 });
    let refusal = match app.dispatch_typed(first, &meta("local")).await {
        Err(fault) => fault,
        Ok(_) => settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.err().expect("a windowless multi-file pick is refused"),
    };
    assert!(format!("{refusal:?}").contains("remodeling-import-window-required"), "{refusal:?}");
    assert_eq!(shape(&app), (0, 0, 0));
}
//#endregion 🧾️ImportTransaction
