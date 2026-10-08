use crate::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_avi_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, AVI_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<AviEditor as ArtifactEditor>::DIALECT, AVI_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn playback_export_route_is_registered_cancellable_and_fully_retired() {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<AviEditor>, _>(async { semio_framework_plugin::App { definition: create_avi_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let handle = app.submit_media_export(semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID).await.expect("registered AVI playback producer");
    app.cancel_media_export(&handle).await.expect("cancel live AVI producer");
    assert!(app.poll_media_export(&handle).await.is_err(), "cancelled handle transfers to bounded close");
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(1, 16_384).expect("bounded AVI close");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn playback_export_route_streams_the_real_fixture_with_exact_mime_and_bytes() {
    use semio_framework_plugin::{app::ArtifactMediaExportPoll, PluginApp};
    let source = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎬️.avi");
    let snapshot = crate::standards::v1_0::subsets::any::io::decode_avi(source).expect("real AVI fixture decodes");
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<AviEditor>, _>(async { semio_framework_plugin::App { definition: create_avi_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&snapshot, STDIO_AVI_DOCUMENT_SCHEMA) else {
        panic!("AVI snapshot builds one document load");
    };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("host loads the real AVI document");
    let handle = app.submit_media_export(semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID).await.expect("registered AVI playback producer");
    let mut completed = None;
    for _ in 0..10_000 {
        match app.poll_media_export(&handle).await.expect("AVI playback poll") {
            ArtifactMediaExportPoll::Running { .. } => semio_framework_async::yield_once().await,
            ArtifactMediaExportPoll::Complete(result) => {
                completed = Some(result);
                break;
            }
            ArtifactMediaExportPoll::Cancelled => panic!("AVI playback export cancelled unexpectedly"),
            ArtifactMediaExportPoll::Failed(detail) => panic!("AVI playback export failed: {detail}"),
        }
    }
    let result = completed.expect("AVI playback export completes within bounded polls");
    assert_eq!(result.mime_type, crate::standards::v1_0::subsets::any::io::playback::MIME_TYPE);
    assert_eq!(result.schema, crate::standards::v1_0::subsets::any::io::playback::MEDIA_SCHEMA);
    assert_eq!(result.media_type, crate::standards::v1_0::subsets::any::io::playback::MEDIA_TYPE);
    app.retain_media_export_result(&handle, result).await.expect("retain exact AVI output authority");
    let mut exported = Vec::new();
    while let Some(chunk) = app.take_media_export_chunk(&handle).await.expect("take AVI playback page") {
        assert!(chunk.len() <= semio_framework_plugin::app::ArtifactOutputChunks::CHUNK_BYTES);
        exported.extend_from_slice(&chunk);
    }
    assert_eq!(exported, source);
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(1, 16_384).expect("bounded AVI close");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::AviEditor, || semio_framework_plugin::App { definition: super::create_avi_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl");

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_plugin_media_and_isolates_fresh_owner_history() {
    use semio_framework_plugin::plugin_app_close_prelude::{MediaArtifact, MediaArtifactDescriptor};
    use semio_framework_plugin::{artifact_app_laws, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
    let source = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎬️.avi");
    let codec = <AviEditor as ArtifactEditor>::natural_file_codec().expect("paired natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.avi@1.0", ".avi", "video/x-msvideo", true));
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
    let initial = <AviEditor as ArtifactEditor>::initial_snapshot();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<AviEditor>, _>(async { semio_framework_plugin::App { definition: create_avi_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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
    assert_eq!(app.snapshot().expect("refused AVI preserves the owner"), opened);
    let saved = app.produce_media(NATURAL_FILE_PORT).await.expect("registered natural save");
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    assert_eq!(saved.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let oracle = semio_s_artifact_stdio_avi_test_oracle::standards::v1_0::subsets::hdrl::oracle_identity_round_trip(&saved.data).expect("riff and the independent AVI codec reopen and write the AVI export");
    let observed = semio_s_artifact_stdio_avi_test_oracle::standards::v1_0::subsets::hdrl::project_avi_1_0(&saved.data).expect("project AVI export");
    let expected = semio_s_artifact_stdio_avi_test_oracle::standards::v1_0::subsets::hdrl::project_avi_1_0(&oracle).expect("project independent AVI output");
    assert_eq!(observed, expected);
    let mut reopened = artifact_app_laws::new_registered_app::<EditorApp<AviEditor>, _>(async { semio_framework_plugin::App { definition: create_avi_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use crate::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviStream};
    let base = AviSnapshot { streams: vec![AviStream { chunks: vec![AviChunk { fourcc: "00dc".into(), data: vec![1], keyframe: false }], ..AviStream::default() }], ..AviSnapshot::default() };
    let emit = |event: editing::SnapshotEditEvent| <AviEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let header = emit(editing::SnapshotEditEvent::SetValue { path: "/mainHeader/width".into(), value: semio_framework_value::DslValue::uint(640) }).expect("a header field edit resolves");
    let [mutation @ AviMutation::SetMainHeader(_)] = header.artifact_mutations.as_slice() else { panic!("a main header edit raises the main-header kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.main_header.width, 640);
    let key = emit(editing::SnapshotEditEvent::SetValue { path: "/streams/0/chunks/0/keyframe".into(), value: semio_framework_value::DslValue::Bool(true) }).expect("a keyframe flag edit resolves");
    assert!(matches!(key.artifact_mutations.as_slice(), [AviMutation::SetChunkKeyframe(_)]));
    let content = emit(editing::SnapshotEditEvent::SetValue { path: "/streams/0/chunks/0/fourcc".into(), value: semio_framework_value::DslValue::String("01wb".into()) }).expect("a chunk content edit resolves");
    let mut state = base.clone();
    content.artifact_mutations.iter().for_each(|mutation| {
        apply_mutation(&mut state, mutation);
    });
    assert_eq!(state.streams[0].chunks[0].fourcc, "01wb");
    assert_eq!(emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("no kind").code.0, "snapshot-edit.unsupported-path");
}
