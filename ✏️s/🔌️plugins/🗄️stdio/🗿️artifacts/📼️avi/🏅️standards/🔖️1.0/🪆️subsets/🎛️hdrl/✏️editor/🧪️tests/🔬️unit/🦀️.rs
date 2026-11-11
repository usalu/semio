use crate::standards::v1_0::subsets::any::schema::mutations::AviMutation;
use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use crate::{AVI_DIALECT, STDIO_AVI_DOCUMENT_SCHEMA};
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_s_artifact_stdio_contract::editing;
use crate::apply_mutation;
use crate::editor::avi::*;

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
    use semio_plugin::PluginApp;
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::new_registered_app::<EditorApp<AviEditor>, _>(async { semio_plugin::App { definition: create_avi_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let handle = semio_plugin::artifact_app_laws::submit_fixture_media_export(&mut app, semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("registered AVI playback producer");
    app.cancel_media_export(&handle).await.expect("cancel live AVI producer");
    assert!(semio_plugin::artifact_app_laws::poll_fixture_media_export(&mut app, &handle, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.is_err(), "cancelled handle transfers to bounded close");
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(scope.cx(), semio_s_artifact_stdio_contract::editing::ample_close_grant()).expect("bounded AVI close");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn playback_export_route_streams_the_real_fixture_with_exact_mime_and_bytes() {
    use semio_plugin::{app::ArtifactMediaExportPoll, PluginApp};
    let source = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎬️.avi");
    let snapshot = crate::standards::v1_0::subsets::any::io::decode_avi(source).expect("real AVI fixture decodes");
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::new_registered_app::<EditorApp<AviEditor>, _>(async { semio_plugin::App { definition: create_avi_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&snapshot, STDIO_AVI_DOCUMENT_SCHEMA) else {
        panic!("AVI snapshot builds one document load");
    };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).expect("host loads the real AVI document");
    let handle = semio_plugin::artifact_app_laws::submit_fixture_media_export(&mut app, semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("registered AVI playback producer");
    let mut completed = None;
    for _ in 0..10_000 {
        match semio_plugin::artifact_app_laws::poll_fixture_media_export(&mut app, &handle, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("AVI playback poll") {
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
    semio_plugin::artifact_app_laws::retain_fixture_media_export_result(&mut app, &handle, result, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("retain exact AVI output authority");
    let mut exported = Vec::new();
    let mut download_terminal = false;
    while let Some(chunk) = semio_plugin::artifact_app_laws::take_fixture_media_export_chunk(&mut app, &handle, &mut download_terminal, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await.expect("take AVI playback page") {
        assert!(chunk.len() <= semio_plugin::app::ArtifactOutputChunks::CHUNK_BYTES);
        exported.extend_from_slice(&chunk);
    }
    assert_eq!(exported, source);
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(scope.cx(), semio_s_artifact_stdio_contract::editing::ample_close_grant()).expect("bounded AVI close");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

semio_plugin::history_edit_acceptance_law!("stdio", crate::editor::avi::AviEditor, || semio_plugin::App { definition: crate::editor::avi::create_avi_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl");

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_plugin_media_and_isolates_fresh_owner_history() {
    use semio_plugin::plugin_app_close_prelude::{MediaArtifact, MediaArtifactDescriptor};
    use semio_plugin::{artifact_app_laws, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
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
    let initial = <AviEditor as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<AviEditor>, _>(async { semio_plugin::App { definition: create_avi_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let mut outside = artifact.clone();
    outside.data.push(0x7f);
    semio_plugin::artifact_app_laws::consume_media(&mut app, NATURAL_FILE_PORT, artifact, scope.cx()).await.expect("registered natural import");
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).expect("natural import publishes");
    let opened = app.snapshot().expect("opened snapshot").clone();
    let refused = match semio_plugin::artifact_app_laws::consume_media(&mut app, NATURAL_FILE_PORT, outside, scope.cx()).await {
        Err(_) => true,
        Ok(_) => semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).is_err(),
    };
    assert!(refused);
    assert_eq!(app.snapshot().expect("refused AVI preserves the owner"), opened);
    let saved = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |native_owner| semio_plugin::artifact_app_laws::produce_fixture_media(&mut app, NATURAL_FILE_PORT, native_owner, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).await.expect("registered natural save");
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    assert_eq!(saved.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let oracle = semio_s_artifact_stdio_avi_test_oracle::standards::v1_0::subsets::hdrl::oracle_identity_round_trip(&saved.data).expect("riff and the independent AVI codec reopen and write the AVI export");
    let observed = semio_s_artifact_stdio_avi_test_oracle::standards::v1_0::subsets::hdrl::project_avi_1_0(&saved.data).expect("project AVI export");
    let expected = semio_s_artifact_stdio_avi_test_oracle::standards::v1_0::subsets::hdrl::project_avi_1_0(&oracle).expect("project independent AVI output");
    assert_eq!(observed, expected);
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut reopened = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<AviEditor>, _>(async { semio_plugin::App { definition: create_avi_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    reopened.bind_instance_id(2).await;
    semio_plugin::artifact_app_laws::consume_media(&mut reopened, NATURAL_FILE_PORT, saved, scope.cx()).await.expect("fresh owner imports exported bytes");
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
    assert_eq!(emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("no kind").code.0, "snapshot-edit.unsupported-path");
}

#[semio_framework_async_macros::async_test]
async fn cut_range_plans_one_removal_run_at_a_stable_index_and_refuses_everything_else() {
    use crate::standards::v1_0::subsets::any::schema::mutations::AviMutation;
    let snapshot = crate::standards::v1_0::subsets::any::io::decode_avi(include_bytes!("../../../🧪️tests/🧫️fixtures/🎬️.avi")).expect("real AVI fixture decodes");
    let chunks = snapshot.streams[0].chunks.len();
    let planned = cut_range_mutations(&snapshot, " 3 - 8 ").expect("an in-range cut plans");
    assert_eq!(planned.len(), 5);
    assert!(planned.iter().all(|mutation| matches!(mutation, AviMutation::RemoveChunk(removal) if removal.stream_index == 0 && removal.index == 3)));
    for refused in ["5-5", "9-3", "x", "+1-4", "4", &format!("0-{}", chunks + 1)] {
        assert_eq!(cut_range_mutations(&snapshot, refused).expect_err("an invalid range is refused").code.0, "stdio.avi.cut-range", "{refused}");
    }
    assert_eq!(cut_frames_action().id, CUT_FRAMES_ACTION_ID);
}
