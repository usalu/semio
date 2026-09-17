use super::*;
use crate::editor::remodeling::panels::media;
use crate::editor::remodeling::unit_tests::context::{app_with_registry, dispatch, render_json};
use crate::editor::remodeling::RemodelingCommand;
use semio_framework_plugin::artifact_app_laws;

/// 🖥️ The mounted path the react host drives: a registry-backed app answers before its bounded retained
/// publication lands, so the settle is what makes the stream visible — to the snapshot AND to the Media
/// panel rendered afterwards.
#[semio_framework_async_macros::async_test]
async fn a_settled_add_stream_appends_one_uncalibrated_stream_the_media_panel_counts() {
    let mut app = app_with_registry().await;
    let before = render_json(&mut app, media::REMODELING_PLAY_BODY_MEDIA).await.to_string();
    assert!(before.contains("Streams: 0"), "the boot document carries no stream: {before}");
    let dispatched = dispatch(&mut app, RemodelingCommand::AddStream(AddStream { name: "Clip".into(), kind: "video".into(), camera_id: String::new() })).await;
    assert!(dispatched.edited_document(), "the document lane published: {:?}", dispatched.lanes);
    let after = app.snapshot().expect("snapshot after");
    let stream = after.streams.last().expect("the new stream");
    assert_eq!((after.streams.len(), stream.name.as_str(), stream.kind, stream.camera_id.as_deref(), stream.frames.len()), (1, "Clip", MediaKind::Video, None, 0));
    let panel = render_json(&mut app, media::REMODELING_PLAY_BODY_MEDIA).await.to_string();
    assert!(panel.contains("Streams: 1"), "the Media panel counts the settled stream: {panel}");
}

/// 🚫️ The manifest's Actions form used to default `cameraId` to `cam-0`: on a document without that
/// calibration `CreateStream::diff` answered a FATAL outcome, and the bounded lane journaled a
/// `create-stream` edit whose head equalled its base (the Media panel kept "Streams: 0" while the
/// history ledger and undo both claimed a stream). The handler refuses an unknown camera up front.
#[semio_framework_async_macros::async_test]
async fn an_unknown_camera_is_refused_before_the_publication() {
    let mut app = app_with_registry().await;
    let instance = artifact_app_laws::meta("local").instance_id;
    app.dispatch_typed(RemodelingCommand::AddStream(AddStream { name: "Stream".into(), kind: "image-sequence".into(), camera_id: "cam-0".into() }), &artifact_app_laws::meta("local")).await.expect("the bounded route admits the command; the handler faults inside the publication");
    let fault = match artifact_app_laws::settle_registered_typed_operation(&mut *app, instance).await {
        Err(fault) => fault,
        Ok(receipt) => panic!("an unknown camera is a lane fault, never a phantom edit: {:?}", receipt.lanes),
    };
    assert!(fault.message.contains("remodeling.stream.unknown-camera"), "{fault:?}");
    assert_eq!(app.snapshot().expect("snapshot after").streams.len(), 0);
}
