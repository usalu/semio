use super::*;

fn register_mp4_snapshot_schema() {
    semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::isobmff::subsets::any::schema::mp4_artifact_schema_descriptor()).expect("schema descriptor publication");
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_mp4_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, MP4_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Mp4Editor as ArtifactEditor>::DIALECT, MP4_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn playback_export_route_is_registered_cancellable_and_fully_retired() {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<Mp4Editor>, _>(async { semio_framework_plugin::App { definition: create_mp4_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let handle = app.submit_media_export(semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID).await.expect("registered MP4 playback producer");
    app.cancel_media_export(&handle).await.expect("cancel live MP4 producer");
    assert!(app.poll_media_export(&handle).await.is_err(), "cancelled handle transfers to bounded close");
    let mut last = None;
    for _ in 0..10_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        last = Some(app.close_step(1, 16_384).expect("bounded MP4 close"));
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty(), "MP4 cancellation close remained at {last:?}");
}

#[semio_framework_async_macros::async_test]
async fn playback_export_route_streams_the_real_fixture_with_exact_mime_and_bytes() {
    use semio_framework_plugin::{app::ArtifactMediaExportPoll, PluginApp};
    let source = include_bytes!("../../../🧫️fixtures/🎬️.mp4");
    let snapshot = crate::standards::isobmff::subsets::any::io::decode_mp4(source).expect("real MP4 fixture decodes");
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<Mp4Editor>, _>(async { semio_framework_plugin::App { definition: create_mp4_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&snapshot, STDIO_MP4_DOCUMENT_SCHEMA) else {
        panic!("MP4 snapshot builds one document load");
    };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("host loads the real MP4 document");
    let handle = app.submit_media_export(semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID).await.expect("registered MP4 playback producer");
    let mut completed = None;
    for _ in 0..100_000 {
        match app.poll_media_export(&handle).await.expect("MP4 playback poll") {
            ArtifactMediaExportPoll::Running { .. } => semio_framework_async::yield_once().await,
            ArtifactMediaExportPoll::Complete(result) => {
                completed = Some(result);
                break;
            }
            ArtifactMediaExportPoll::Cancelled => panic!("MP4 playback export cancelled unexpectedly"),
            ArtifactMediaExportPoll::Failed(detail) => panic!("MP4 playback export failed: {detail}"),
        }
    }
    let result = completed.expect("MP4 playback export completes within bounded polls");
    assert_eq!(result.mime_type, crate::standards::isobmff::subsets::any::io::playback::MIME_TYPE);
    assert_eq!(result.schema, crate::standards::isobmff::subsets::any::io::playback::MEDIA_SCHEMA);
    assert_eq!(result.media_type, crate::standards::isobmff::subsets::any::io::playback::MEDIA_TYPE);
    app.retain_media_export_result(&handle, result).await.expect("retain exact MP4 output authority");
    let mut exported = Vec::new();
    while let Some(chunk) = app.take_media_export_chunk(&handle).await.expect("take MP4 playback page") {
        assert!(chunk.len() <= semio_framework_plugin::app::ArtifactOutputChunks::CHUNK_BYTES);
        exported.extend_from_slice(&chunk);
    }
    assert_eq!(exported, source);
    let mut last = None;
    for _ in 0..100_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        last = Some(app.close_step(1, 16_384).expect("bounded MP4 close"));
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty(), "MP4 completed export close remained at {last:?}");
}

#[test]
fn large_payload_metadata_edit_uses_compact_native_event_and_exact_inverse_admission() {
    register_mp4_snapshot_schema();
    let mut snapshot = Mp4Snapshot::default();
    let mut track = Mp4Track::default();
    track.track_id = 1;
    track.samples.push(Mp4Sample { data: vec![7; 2 * 1_024 * 1_024], duration: 1_000, cts_offset: 0, sync: true });
    snapshot.tracks.push(track);
    let event = editing::SnapshotEditEvent::SetValue { path: "/ftyp/minorVersion".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(42)) };
    assert!(<Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let emit = <Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("ftyp edit emits");
    let [Mp4Mutation::SetFtyp(payload)] = emit.artifact_mutations.as_slice() else { panic!("large-payload ftyp edit must remain compact") };
    assert_eq!(payload.ftyp.minor_version, 42);
    let bytes = <Mp4Mutation as protocol::OpBinary>::encode_op(&emit.artifact_mutations[0]).expect("compact event encodes");
    assert!(bytes.len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    let next = protocol::MutationDiff::apply(<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::diff(&emit.artifact_mutations[0], &snapshot).diff(), &snapshot).expect("compact event applies");
    assert_eq!(next.ftyp.minor_version, 42);
    assert_eq!(next.tracks[0].samples[0].data, snapshot.tracks[0].samples[0].data);
    let inverse = <Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::inverse(&emit.artifact_mutations[0], &snapshot).expect("valid retained mutation inverse fixture");
    assert!(inverse.iter().all(|mutation| <Mp4Mutation as protocol::OpBinary>::encode_op(mutation).is_ok_and(|bytes| bytes.len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)));
    let restored = inverse.into_iter().fold(next.clone(), |current, mutation| protocol::MutationDiff::apply(<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::diff(&mutation, &current).diff(), &current).expect("ftyp inverse applies"));
    assert_eq!(restored, snapshot);
    let native = crate::standards::isobmff::subsets::any::io::encode_mp4(&next);
    let reopened = crate::standards::isobmff::subsets::any::io::decode_mp4(&native).expect("edited native MP4 reopens");
    assert_eq!(reopened.ftyp.minor_version, 42);
    assert_eq!(reopened.tracks[0].samples[0].data, snapshot.tracks[0].samples[0].data);
    let remove = editing::SnapshotEditEvent::RemoveValue { path: "/tracks/0/samples/0".into() };
    assert!(<Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&remove, &snapshot), "bounded admission must not clone or encode the addressed payload");
    assert!(<Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&remove, &snapshot).is_err(), "an oversized exact inverse must still be refused before publication");
    assert_eq!(snapshot.tracks[0].samples[0].data, vec![7; 2 * 1_024 * 1_024]);
}

#[test]
fn payload_detail_edits_publish_the_exact_requested_value() {
    register_mp4_snapshot_schema();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
    let mut snapshot = Mp4Snapshot::default();
    let mut track = Mp4Track::default();
    track.samples.push(Mp4Sample { data: vec![7, 9], duration: 1000, cts_offset: 0, sync: true });
    snapshot.tracks.push(track);
    let base: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("/tracks/0/samples/0/data{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") {
            *from = format!("/tracks/0/samples/0/data{}", from.as_str().unwrap()).into();
        }
        let event: editing::SnapshotEditEvent = semio_framework_pack_json::from_json_str(&event.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let emitted = <Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::MutationDiff::apply(<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut("/tracks/0/samples/0/data").unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::Mp4Editor, || semio_framework_plugin::App { definition: super::create_mp4_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_plugin_media_and_isolates_fresh_owner_history() {
    use semio_framework_plugin::plugin_app_close_prelude::{MediaArtifact, MediaArtifactDescriptor};
    use semio_framework_plugin::{artifact_app_laws, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
    let source = include_bytes!("../../../🧫️fixtures/🎬️.mp4");
    let codec = <Mp4Editor as ArtifactEditor>::natural_file_codec().expect("paired natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.mp4@isobmff", ".mp4", "video/mp4", true));
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
    let initial = <Mp4Editor as ArtifactEditor>::initial_snapshot();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<Mp4Editor>, _>(async { semio_framework_plugin::App { definition: create_mp4_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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
    assert_eq!(app.snapshot().expect("refused MP4 preserves the owner"), opened);
    let saved = app.produce_media(NATURAL_FILE_PORT).await.expect("registered natural save");
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    assert_eq!(saved.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let oracle = semio_s_artifact_stdio_mp4_test_oracle::standards::v_isobmff::subsets::any::oracle_identity_round_trip(&saved.data).expect("mp4 independently reopens and muxes the MP4 export");
    let observed = semio_s_artifact_stdio_mp4_test_oracle::standards::v_isobmff::subsets::any::project_mp4_mutation(&saved.data).expect("project MP4 export");
    let expected = semio_s_artifact_stdio_mp4_test_oracle::standards::v_isobmff::subsets::any::project_mp4_mutation(&oracle).expect("project independent MP4 output");
    assert_eq!(observed, expected);
    let mut reopened = artifact_app_laws::new_registered_app::<EditorApp<Mp4Editor>, _>(async { semio_framework_plugin::App { definition: create_mp4_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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
