use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_mp3_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, MP3_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Mp3Editor as ArtifactEditor>::DIALECT, MP3_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn playback_export_route_is_registered_cancellable_and_fully_retired() {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<Mp3Editor>, _>(async { semio_framework_plugin::App { definition: create_mp3_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let handle = app.submit_media_export(crate::standards::mpeg1_layer3::subsets::any::io::playback::PORT_ID).await.expect("registered MP3 playback producer");
    app.cancel_media_export(&handle).await.expect("cancel live MP3 producer");
    assert!(app.poll_media_export(&handle).await.is_err(), "cancelled handle transfers to bounded close");
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(1, 16_384).expect("bounded MP3 close");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn playback_export_route_streams_the_real_fixture_with_exact_mime_and_bytes() {
    use semio_framework_plugin::{app::ArtifactMediaExportPoll, PluginApp};
    let source = include_bytes!("../../../🧫️fixtures/🔊️.mp3");
    let snapshot = crate::standards::mpeg1_layer3::subsets::any::io::decode_mp3(source).expect("real MP3 fixture decodes");
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<Mp3Editor>, _>(async { semio_framework_plugin::App { definition: create_mp3_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&snapshot, STDIO_MP3_DOCUMENT_SCHEMA) else {
        panic!("MP3 snapshot builds one document load");
    };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("host loads the real MP3 document");
    let handle = app.submit_media_export(crate::standards::mpeg1_layer3::subsets::any::io::playback::PORT_ID).await.expect("registered MP3 playback producer");
    let mut completed = None;
    for _ in 0..10_000 {
        match app.poll_media_export(&handle).await.expect("MP3 playback poll") {
            ArtifactMediaExportPoll::Running { .. } => semio_framework_async::yield_once().await,
            ArtifactMediaExportPoll::Complete(result) => {
                completed = Some(result);
                break;
            }
            ArtifactMediaExportPoll::Cancelled => panic!("MP3 playback export cancelled unexpectedly"),
            ArtifactMediaExportPoll::Failed(detail) => panic!("MP3 playback export failed: {detail}"),
        }
    }
    let result = completed.expect("MP3 playback export completes within bounded polls");
    assert_eq!(result.mime_type, crate::standards::mpeg1_layer3::subsets::any::io::playback::MIME_TYPE);
    assert_eq!(result.schema, crate::standards::mpeg1_layer3::subsets::any::io::playback::MEDIA_SCHEMA);
    assert_eq!(result.media_type, crate::standards::mpeg1_layer3::subsets::any::io::playback::MEDIA_TYPE);
    app.retain_media_export_result(&handle, result).await.expect("retain exact MP3 output authority");
    let mut exported = Vec::new();
    while let Some(chunk) = app.take_media_export_chunk(&handle).await.expect("take MP3 playback page") {
        assert!(chunk.len() <= semio_framework_plugin::app::ArtifactOutputChunks::CHUNK_BYTES);
        exported.extend_from_slice(&chunk);
    }
    assert_eq!(exported, source);
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(1, 16_384).expect("bounded MP3 close");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::Mp3Editor, || semio_framework_plugin::App { definition: super::create_mp3_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_plugin_media_and_isolates_fresh_owner_history() {
    use semio_framework_plugin::plugin_app_close_prelude::{MediaArtifact, MediaArtifactDescriptor};
    use semio_framework_plugin::{artifact_app_laws, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
    let source = include_bytes!("../../../🧫️fixtures/🔊️.mp3");
    let codec = <Mp3Editor as ArtifactEditor>::natural_file_codec().expect("paired natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.mp3@mpeg1-layer3", ".mp3", "audio/mpeg", true));
    let artifact = MediaArtifact {
        descriptor: MediaArtifactDescriptor {
            edge_id: None,
            port_id: Some(NATURAL_FILE_PORT.into()),
            kind_id: Some(codec.format_kind.into()),
            media_type: None,
            wire: MediaWireFormat::Binary { format_kind: codec.format_kind.into() },
            blob_hash: None,
        },
        data: source.to_vec(),
    };
    let initial = <Mp3Editor as ArtifactEditor>::initial_snapshot();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<Mp3Editor>, _>(async { semio_framework_plugin::App { definition: create_mp3_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let mut outside = artifact.clone();
    outside.data.push(0x7f);
    app.consume_media(NATURAL_FILE_PORT, artifact).await.expect("registered natural import");
    artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.expect("natural import publishes");
    let opened = app.snapshot().expect("opened snapshot").clone();
    let refused = match app.consume_media(NATURAL_FILE_PORT, outside).await {
        Err(_) => true,
        Ok(_) => artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.is_err(),
    };
    assert!(refused);
    assert_eq!(app.snapshot().expect("refused MP3 preserves the owner"), opened);
    let saved = app.produce_media(NATURAL_FILE_PORT).await.expect("registered natural save");
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    assert_eq!(saved.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let oracle = semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::oracle_round_trip(&saved.data).expect("id3 and the independent MPEG walker reopen and write the MP3 export");
    let observed = semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::project_mp3(&saved.data).expect("project MP3 export");
    let expected = semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::project_mp3(&oracle).expect("project independent MP3 output");
    assert_eq!(observed, expected);
    let mut reopened = artifact_app_laws::new_registered_app::<EditorApp<Mp3Editor>, _>(async { semio_framework_plugin::App { definition: create_mp3_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    reopened.bind_instance_id(2).await;
    reopened.consume_media(NATURAL_FILE_PORT, saved).await.expect("fresh owner imports exported bytes");
    artifact_app_laws::settle_registered_typed_operation(&mut reopened, 2).await.expect("fresh owner import publishes");
    let reopened_snapshot = reopened.snapshot().expect("reopened snapshot").clone();
    assert_eq!(reopened_snapshot, opened);
    artifact_app_laws::settle_history_verb(&mut app, "undo", 1).await;
    assert_eq!(app.snapshot().expect("source undo snapshot"), initial);
    assert_eq!(reopened.snapshot().expect("reopened snapshot remains isolated"), reopened_snapshot);
    artifact_app_laws::settle_history_verb(&mut app, "redo", 1).await;
    assert_eq!(app.snapshot().expect("source redo snapshot"), opened);
    artifact_app_laws::settle_history_verb(&mut reopened, "undo", 2).await;
    assert_eq!(reopened.snapshot().expect("reopened undo snapshot"), initial);
    assert_eq!(app.snapshot().expect("source remains isolated"), opened);
    artifact_app_laws::settle_history_verb(&mut reopened, "redo", 2).await;
    assert_eq!(reopened.snapshot().expect("reopened redo snapshot"), reopened_snapshot);
    artifact_app_laws::close_registered_fixture_app(&mut reopened);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}
