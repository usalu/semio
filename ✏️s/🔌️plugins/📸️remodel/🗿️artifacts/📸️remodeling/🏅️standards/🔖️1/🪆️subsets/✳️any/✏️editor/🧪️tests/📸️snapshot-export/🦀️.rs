//! 📸️ The checked snapshot port exports the current complete record and preserves the saved document.

use super::*;
use pack::snapshot::ArtifactPack;
use semio_plugin::artifact_app_laws::{fixture_close_grant, poll_fixture_media_export, retain_fixture_media_export_result, take_fixture_media_export_chunk, OriginalFixtureCaller};
use semio_plugin::{ArtifactMediaExportPoll, ArtifactOutputChunks, PluginApp};

#[semio_framework_async_macros::async_test]
async fn checked_snapshot_media_export_matches_complete_materialized_record() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📸️snapshot-export/🔣️.json")).expect("snapshot export vectors");
    assert_eq!(<RemodelingSnapshot as pack::snapshot::ArtifactDsl>::envelope_id(), fixture["envelopeId"].as_str().unwrap());
    let cuts = std::iter::once(None).chain(fixture["cancelAfterTurns"].as_array().unwrap().iter().map(|cut| Some(cut.as_u64().unwrap() as usize)));
    for cut in cuts {
        let mut app = app().await;
        let edited = dispatch(&mut app, RemodelingCommand::AddStream(add_stream::AddStream { name: fixture["streamName"].as_str().unwrap().into(), kind: "imageSequence".into(), camera_id: String::new() })).await;
        assert!(edited.edited_document(), "fixture edit must publish before export");
        let before = app.snapshot().expect("current snapshot");
        assert!(before.streams.iter().any(|stream| stream.name == fixture["streamName"].as_str().unwrap()));
        let expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&before)).expect("independent complete snapshot JSON parser");
        let document = app.0.document_identity().expect("saved document identity");
        let revision = app.0.document_base_revision().expect("saved document revision");
        let close = fixture_close_grant(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
        let poll_grant = if cut.is_some() { semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..close } } else { close };
        let mut canonical: Option<Vec<u8>> = None;
        for _ in 0..if cut.is_none() { 2 } else { 1 } {
            let mut submitted = None;
            for _ in 0..OriginalFixtureCaller::maximum_turns() {
                if app.0.submit_media_export_checked(fixture["port"].as_str().unwrap(), &document, revision, None, &mut submitted, app.1.cx()).await.expect("checked snapshot submission") { break; }
                app.0.maintenance_step(app.1.cx(), close).expect("snapshot submission maintenance");
            }
            let handle = submitted.expect("snapshot handle");
            let mut progress = 0;
            let mut cancelled = false;
            let mut terminal = false;
            for turn in 0..OriginalFixtureCaller::maximum_turns() {
                if cut == Some(turn) { app.0.cancel_media_export(&handle).await.expect("cancel original snapshot export"); cancelled = true; }
                match poll_fixture_media_export(&mut app.0, &handle, app.1.cx(), poll_grant).await.expect("snapshot export poll") {
                    ArtifactMediaExportPoll::Running { applied_progress, .. } => { assert!(applied_progress >= progress); progress = applied_progress; }
                    ArtifactMediaExportPoll::Cancelled => { assert!(cancelled); terminal = true; break; }
                    ArtifactMediaExportPoll::Complete(result) => {
                        assert!(!cancelled, "cancelled export cannot publish bytes");
                        assert_eq!(result.schema, fixture["schema"].as_str().unwrap());
                        assert_eq!(result.mime_type, fixture["mimeType"].as_str().unwrap());
                        let status = retain_fixture_media_export_result(&mut app.0, &handle, result, app.1.cx(), close).await.expect("retain snapshot output");
                        let mut bytes = Vec::new();
                        let mut done = false;
                        for _ in 0..=(ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES / ArtifactOutputChunks::CHUNK_BYTES) {
                            if let Some(chunk) = take_fixture_media_export_chunk(&mut app.0, &handle, &mut done, app.1.cx(), close).await.expect("snapshot output chunk") { bytes.extend_from_slice(&chunk); }
                            if done { break; }
                        }
                        assert!(done, "snapshot output drained to terminal");
                        assert_eq!(bytes.len() as u64, status.total_bytes);
                        let restored = RemodelingSnapshot::decode_pack(&bytes).expect("whole native snapshot record");
                        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&restored)).expect("independent decoded snapshot JSON parser");
                        assert_eq!(actual, expected, "every declared persistent snapshot field survives");
                        if let Some(first) = canonical.as_ref() { assert_eq!(&bytes, first, "unchanged materialized snapshots have canonical bytes across operations"); } else { canonical = Some(bytes); }
                        terminal = true;
                        break;
                    }
                    ArtifactMediaExportPoll::Failed(reason) => panic!("snapshot export failed: {reason}"),
                }
            }
            assert!(terminal, "snapshot export reaches terminal");
            assert_eq!(cancelled, cut.is_some(), "every declared cancellation cut must be reached");
            assert_eq!(app.0.document_identity().unwrap(), document);
            assert_eq!(app.0.document_base_revision().unwrap(), revision);
            let after: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&app.snapshot().unwrap())).expect("independent final snapshot JSON parser");
            assert_eq!(after, expected, "export does not edit the document");
        }
    }
}
