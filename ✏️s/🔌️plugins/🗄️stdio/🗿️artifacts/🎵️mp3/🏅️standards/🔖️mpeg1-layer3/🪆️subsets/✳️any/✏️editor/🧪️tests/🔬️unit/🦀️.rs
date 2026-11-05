use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use crate::{MP3_DIALECT, STDIO_MP3_DOCUMENT_SCHEMA};
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_s_artifact_stdio_contract::editing;
use crate::editor::mp3::*;

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
    use semio_plugin::PluginApp;
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::new_registered_app::<EditorApp<Mp3Editor>, _>(async { semio_plugin::App { definition: create_mp3_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let handle = semio_plugin::artifact_app_laws::submit_fixture_media_export(&mut app, crate::standards::mpeg1_layer3::subsets::any::io::playback::PORT_ID, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("registered MP3 playback producer");
    app.cancel_media_export(&handle).await.expect("cancel live MP3 producer");
    assert!(semio_plugin::artifact_app_laws::poll_fixture_media_export(&mut app, &handle, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.is_err(), "cancelled handle transfers to bounded close");
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(scope.cx(), semio_s_artifact_stdio_contract::editing::ample_close_grant()).expect("bounded MP3 close");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn playback_export_route_streams_the_real_fixture_with_exact_mime_and_bytes() {
    use semio_plugin::{app::ArtifactMediaExportPoll, PluginApp};
    let source = include_bytes!("../../../🧪️tests/🧫️fixtures/🔊️.mp3");
    let snapshot = crate::standards::mpeg1_layer3::subsets::any::io::decode_mp3(source).expect("real MP3 fixture decodes");
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::new_registered_app::<EditorApp<Mp3Editor>, _>(async { semio_plugin::App { definition: create_mp3_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&snapshot, STDIO_MP3_DOCUMENT_SCHEMA) else {
        panic!("MP3 snapshot builds one document load");
    };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).expect("host loads the real MP3 document");
    let handle = semio_plugin::artifact_app_laws::submit_fixture_media_export(&mut app, crate::standards::mpeg1_layer3::subsets::any::io::playback::PORT_ID, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("registered MP3 playback producer");
    let mut completed = None;
    for _ in 0..10_000 {
        match semio_plugin::artifact_app_laws::poll_fixture_media_export(&mut app, &handle, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("MP3 playback poll") {
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
    semio_plugin::artifact_app_laws::retain_fixture_media_export_result(&mut app, &handle, result, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("retain exact MP3 output authority");
    let mut exported = Vec::new();
    let mut download_terminal = false;
    while let Some(chunk) = semio_plugin::artifact_app_laws::take_fixture_media_export_chunk(&mut app, &handle, &mut download_terminal, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("take MP3 playback page") {
        assert!(chunk.len() <= semio_plugin::app::ArtifactOutputChunks::CHUNK_BYTES);
        exported.extend_from_slice(&chunk);
    }
    assert_eq!(exported, source);
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(scope.cx(), semio_s_artifact_stdio_contract::editing::ample_close_grant()).expect("bounded MP3 close");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

semio_plugin::history_edit_acceptance_law!("stdio", crate::editor::mp3::Mp3Editor, || semio_plugin::App { definition: crate::editor::mp3::create_mp3_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_plugin_media_and_isolates_fresh_owner_history() {
    use semio_plugin::plugin_app_close_prelude::{MediaArtifact, MediaArtifactDescriptor};
    use semio_plugin::{artifact_app_laws, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
    let source = include_bytes!("../../../🧪️tests/🧫️fixtures/🔊️.mp3");
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
    let initial = <Mp3Editor as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<Mp3Editor>, _>(async { semio_plugin::App { definition: create_mp3_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let mut outside = artifact.clone();
    outside.data.push(0x7f);
    app.consume_media(NATURAL_FILE_PORT, artifact, scope.cx()).await.expect("registered natural import");
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).expect("natural import publishes");
    let opened = app.snapshot().expect("opened snapshot").clone();
    let refused = match app.consume_media(NATURAL_FILE_PORT, outside, scope.cx()).await {
        Err(_) => true,
        Ok(_) => semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).is_err(),
    };
    assert!(refused);
    assert_eq!(app.snapshot().expect("refused MP3 preserves the owner"), opened);
    let saved = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |native_owner| semio_plugin::artifact_app_laws::produce_fixture_media(&mut app, NATURAL_FILE_PORT, native_owner, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).await.expect("registered natural save");
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    assert_eq!(saved.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let oracle = semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::oracle_round_trip(&saved.data).expect("id3 and the independent MPEG walker reopen and write the MP3 export");
    let observed = semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::project_mp3(&saved.data).expect("project MP3 export");
    let expected = semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::project_mp3(&oracle).expect("project independent MP3 output");
    assert_eq!(observed, expected);
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut reopened = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<Mp3Editor>, _>(async { semio_plugin::App { definition: create_mp3_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    reopened.bind_instance_id(2).await;
    reopened.consume_media(NATURAL_FILE_PORT, saved, scope.cx()).await.expect("fresh owner imports exported bytes");
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut reopened, 2, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).expect("fresh owner import publishes");
    let reopened_snapshot = reopened.snapshot().expect("reopened snapshot").clone();
    assert_eq!(reopened_snapshot, opened);
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "undo", 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().expect("source undo snapshot"), initial);
    assert_eq!(reopened.snapshot().expect("reopened snapshot remains isolated"), reopened_snapshot);
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "redo", 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().expect("source redo snapshot"), opened);
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut reopened, "undo", 2, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(reopened.snapshot().expect("reopened undo snapshot"), initial);
    assert_eq!(app.snapshot().expect("source remains isolated"), opened);
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut reopened, "redo", 2, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(reopened.snapshot().expect("reopened redo snapshot"), reopened_snapshot);
    artifact_app_laws::close_registered_fixture_app(&mut reopened, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
    artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_block() {
    let base = Mp3Snapshot::default();
    let emit = |event: editing::SnapshotEditEvent| <Mp3Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let unchanged = emit(editing::SnapshotEditEvent::SetValue { path: "/frames".into(), value: semio_framework_value::DslValue::Array(Vec::new()) }).expect("an unchanged frame list resolves");
    assert!(unchanged.artifact_mutations.is_empty(), "an edit that changes nothing publishes nothing");
    assert_eq!(emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("no kind").code.0, "snapshot-edit.unsupported-path");
}

#[semio_framework_async_macros::async_test]
async fn cut_frames_plans_one_patch_for_the_whole_frame_range_and_refuses_everything_else() {
    use crate::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation;
    let snapshot = crate::standards::mpeg1_layer3::subsets::any::io::decode_mp3(include_bytes!("../../../🧪️tests/🧫️fixtures/🔊️.mp3")).expect("real MP3 fixture decodes");
    let frames = snapshot.frames.len();
    match cut_frames_mutation(&snapshot, " 10 - 110 ").expect("an in-range cut plans") {
        Mp3Mutation::PatchFrames(patch) => assert_eq!((patch.index, patch.remove_count, patch.frames.len()), (10, 100, 0)),
        other => panic!("a cut must plan exactly one frame patch, not {other:?}"),
    }
    for refused in ["5-5", "9-3", "x", "4", &format!("0-{}", frames + 1)] {
        assert_eq!(cut_frames_mutation(&snapshot, refused).expect_err("an invalid range is refused").code.0, "stdio.mp3.frame-range", "{refused}");
    }
    let action = cut_frames_action();
    assert_eq!(action.id, CUT_FRAMES_ACTION_ID);
    assert!(action.in_palette, "the cut must be reachable from the command palette");
}
