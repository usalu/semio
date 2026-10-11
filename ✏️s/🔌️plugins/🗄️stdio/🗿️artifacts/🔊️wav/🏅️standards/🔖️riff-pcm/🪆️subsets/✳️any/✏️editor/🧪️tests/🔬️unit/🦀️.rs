use super::*;

/// 🧬️ Registers the document schema wav's declaration contributes — the registered contract every snapshot edit validates
/// against; a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor()]).expect("the wav document schema registers");
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_wav_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, WAV_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<WavEditor as ArtifactEditor>::DIALECT, WAV_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_registers_every_natural_audio_action_as_retained_work() {
    let definition = create_wav_editor();
    for action_id in edit_audio::TOOL_IDS {
        let action = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == *action_id).expect("natural audio action");
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    }
}

fn quoted_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

fn drain_publication(publication: &mut store::ArtifactStoreBatchPublication<WavSnapshot, WavMutation>) {
    for _ in 0..4_096 {
        if publication.terminal_is_empty() {
            return;
        }
        let demand = publication.retirement_demands(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).expect("WAV publication close demand");
        let step = publication.close_step(quoted_grant(demand)).expect("WAV publication closes");
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            return;
        }
    }
    panic!("WAV publication did not retire");
}

fn published(event: &editing::SnapshotEditEvent, base: &WavSnapshot) -> WavSnapshot {
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, base).expect("the edit resolves to a kind");
    let mut next = base.clone();
    emit.artifact_mutations.iter().for_each(|mutation| {
        crate::apply_mutation(&mut next, mutation);
    });
    next
}

#[semio_framework_async_macros::async_test]
async fn one_mebibyte_sample_lane_edits_without_generic_value_expansion() {
    register_document_schema();
    let samples = vec![7u8; 1_048_576];
    let snapshot = WavSnapshot { data: WavData::Raw(samples.clone()), ..WavSnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: "/fmt/sampleRate".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(48_000)) };
    assert!(<WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let next = published(&event, &snapshot);
    assert_eq!(next.fmt.sample_rate, 48_000);
    assert_eq!(next.data, WavData::Raw(samples));
}

#[semio_framework_async_macros::async_test]
async fn data_kind_edit_publishes_the_requested_variant_and_reopens_natively() {
    register_document_schema();
    let mut snapshot = WavSnapshot::default();
    snapshot.fmt.bits_per_sample = 8;
    snapshot.fmt.byte_rate = snapshot.fmt.sample_rate;
    snapshot.fmt.block_align = 1;
    snapshot.data = WavData::Raw(vec![1, 2]);
    let event = editing::SnapshotEditEvent::SetValue { path: "/data/kind".into(), value: semio_framework_value::DslValue::String("pcm8".into()) };
    let next = published(&event, &snapshot);
    assert_eq!(next.data, WavData::Pcm8(vec![1, 2]));
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("data discriminator edit emits");
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("data discriminator edit must emit one mutation") };
    let published = protocol::apply_diff(<WavMutation as protocol::Mutation<WavSnapshot>>::diff(mutation, &snapshot).diff(), &snapshot).expect("data discriminator mutation applies");
    assert_eq!(published.data, WavData::Pcm8(vec![1, 2]));
    let native = crate::standards::riff_pcm::subsets::any::io::encode_wav(&published);
    let reopened = crate::standards::riff_pcm::subsets::any::io::decode_wav(&native).expect("edited WAV reopens");
    assert_eq!(reopened.data, WavData::Pcm8(vec![1, 2]));
}

#[semio_framework_async_macros::async_test]
async fn chunk_layout_and_pad_bytes_are_visible_details_and_an_unaddressed_path_is_refused() {
    register_document_schema();
    let snapshot = WavSnapshot {
        data: WavData::Raw(vec![7]),
        data_pad_byte: 0xA5,
        other_chunks: vec![crate::standards::riff_pcm::subsets::any::schema::snapshot::RiffChunk { fourcc: "JUNK".into(), data: vec![1, 2, 3], pad_byte: 0x7F }],
        chunk_order: vec![
            crate::standards::riff_pcm::subsets::any::schema::snapshot::WavChunkRef::Other(0),
            crate::standards::riff_pcm::subsets::any::schema::snapshot::WavChunkRef::Format,
            crate::standards::riff_pcm::subsets::any::schema::snapshot::WavChunkRef::Samples,
        ],
        ..WavSnapshot::default()
    };
    let provider = WavDetailsProvider::new(&snapshot);
    assert_eq!(editing::SnapshotDetailsProvider::value(&provider, &[editing::SnapshotDetailPathSegment::Key("dataPadByte".into())],), Some(editing::SnapshotDetailValue::Number(semio_framework_value::Number::UInt(0xA5))),);
    assert_eq!(
        editing::SnapshotDetailsProvider::value(&provider, &[editing::SnapshotDetailPathSegment::Key("otherChunks".into()), editing::SnapshotDetailPathSegment::Index(0), editing::SnapshotDetailPathSegment::Key("padByte".into()),],),
        Some(editing::SnapshotDetailValue::Number(semio_framework_value::Number::UInt(0x7F))),
    );
    let event = editing::SnapshotEditEvent::SetValue { path: "/dataPadByte".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(0x5A)) };
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("a pad byte edit resolves to the pad-byte kind");
    assert!(matches!(emit.artifact_mutations.as_slice(), [WavMutation::SetPadBytes(_)]));
    assert_eq!(published(&event, &snapshot).data_pad_byte, 0x5A);
    let schema = editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) };
    let refused = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&schema, &snapshot).expect_err("no kind edits the schema identity");
    assert_eq!(refused.code.0, "snapshot-edit.unsupported-path");
}

#[semio_framework_async_macros::async_test]
async fn sample_edit_patch_data_replays_and_inverts_without_losing_siblings() {
    register_document_schema();
    let base = WavSnapshot { data: WavData::Raw(vec![1, 2, 3]), ..WavSnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: "/data/value/1".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(9)) };
    let next = published(&event, &base);
    assert_eq!(next.data, WavData::Raw(vec![1, 9, 3]));
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect("sample edit emits");
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("sample edit must emit one mutation") };
    let encoded = <WavMutation as store::OpBinary>::encode_op(mutation).expect("binary encode");
    let decoded = <WavMutation as store::OpBinary>::decode_op(&encoded).expect("binary decode");
    let mut replayed = base.clone();
    crate::apply_mutation(&mut replayed, &decoded);
    assert_eq!(replayed, next);
    for inverse in <WavMutation as protocol::Mutation<WavSnapshot>>::inverse(mutation, &base).expect("valid retained mutation inverse fixture") {
        crate::apply_mutation(&mut replayed, &inverse);
    }
    assert_eq!(replayed, base);
}

#[semio_framework_async_macros::async_test]
async fn large_sample_edit_publishes_cancels_undoes_redoes_and_preserves_metadata() {
    register_document_schema();
    let sample_count = 2_097_152;
    let edit_index = 1_500_000;
    let snapshot = WavSnapshot { data: WavData::Raw(vec![7; sample_count]), ..WavSnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: format!("/data/value/{edit_index}"), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(9)) };
    let started = std::time::Instant::now();
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("sample edit emits");
    assert!(started.elapsed() < std::time::Duration::from_secs(2), "one sample edit of a {sample_count}-sample document took {:?}", started.elapsed());
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("sample edit must emit one mutation") };
    assert!(matches!(mutation, WavMutation::PatchData(_)));
    assert!(<WavMutation as protocol::OpBinary>::encode_op(mutation).expect("patch encodes").len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);

    let envelope = store::create_document_envelope(STDIO_WAV_DOCUMENT_SCHEMA, "wav-large-sample-publication", snapshot, None);
    let mut store = store::ArtifactStore::new(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("WAV store opens");
    store.install_document_store_owners_exact(store::funded_bounded_artifact_store_owners::<WavSnapshot, WavMutation>().expect("funded WAV owners")).map_err(|(error, _)| error).expect("WAV owners install");
    let factory = <WavEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("WAV retained factory");
    let generation = store.generation_now();
    let root = store.snapshot_root();
    let mut cancelled = store
        .begin_apply_batch(semio_framework_job::OperationId(1), generation, store.content_revision_now(), "wav-large-sample-cancel".into(), vec![mutation.clone()], store::HistoryLane::Document, Some(&factory), None)
        .expect("bounded patch cancellation candidate admits");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, maximum_capacity_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, maximum_release_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, maximum_depth: 64 };
    for _ in 0..64 {
        match store.advance_apply_batch(&mut cancelled, grant).expect("cancel candidate advances") {
            store::ArtifactStoreOneItemAdvance::Published(_) => panic!("candidate published before its cancellation point"),
            store::ArtifactStoreOneItemAdvance::Progress(_) if cancelled.staged_items() == 1 => break,
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("cancel candidate blocked"),
            _ => {}
        }
    }
    assert!(store.cancel_apply_batch(&mut cancelled));
    drain_publication(&mut cancelled);
    assert!(cancelled.terminal_is_empty());
    drop(cancelled);
    assert_eq!(store.generation_now(), generation);
    assert!(std::sync::Arc::ptr_eq(&root, &store.snapshot_root()));

    let mut publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(2),
            store.generation_now(),
            store.content_revision_now(),
            "wav-large-sample-publication".into(),
            vec![mutation.clone()],
            store::HistoryLane::Document,
            Some(&factory),
            None,
        )
        .expect("bounded patch admits despite the one-mebibyte base snapshot");
    let mut published = false;
    for _ in 0..64 {
        match store.advance_apply_batch(&mut publication, grant).expect("retained publication advances") {
            store::ArtifactStoreOneItemAdvance::Published(_) => {
                published = true;
                break;
            }
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("admitted WAV patch blocked"),
            _ => {}
        }
    }
    assert!(published);
    let WavData::Raw(samples) = &store.snapshot_ref().data else { panic!("raw samples retained") };
    assert_eq!(samples.len(), sample_count);
    assert_eq!(samples[edit_index], 9);
    assert!(publication.acknowledge());
    drain_publication(&mut publication);
    assert!(publication.terminal_is_empty());
    drop(publication);

    store.dispatch(store::ArtifactCommand::Undo).await.expect("patch undo");
    let WavData::Raw(samples) = &store.snapshot_ref().data else { panic!("raw samples retained after undo") };
    assert_eq!(samples.len(), sample_count);
    assert_eq!(samples[edit_index], 7);
    store.dispatch(store::ArtifactCommand::Redo).await.expect("patch redo");
    let WavData::Raw(samples) = &store.snapshot_ref().data else { panic!("raw samples retained after redo") };
    assert_eq!(samples.len(), sample_count);
    assert_eq!(samples[edit_index], 9);

    let fmt_event = editing::SnapshotEditEvent::SetValue { path: "/fmt/sampleRate".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(48_000)) };
    let fmt_emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&fmt_event, store.snapshot_ref()).expect("metadata edit emits");
    let [fmt_mutation] = fmt_emit.artifact_mutations.as_slice() else { panic!("metadata edit must emit one mutation") };
    assert!(matches!(fmt_mutation, WavMutation::SetFmt(_)), "one format field publishes as the format leaf");
    let mut metadata_publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(3),
            store.generation_now(),
            store.content_revision_now(),
            "wav-large-metadata-publication".into(),
            vec![fmt_mutation.clone()],
            store::HistoryLane::Document,
            Some(&factory),
            None,
        )
        .expect("small metadata mutation admits against a two-mebibyte snapshot");
    let mut metadata_published = false;
    for _ in 0..64 {
        match store.advance_apply_batch(&mut metadata_publication, grant).expect("metadata publication advances") {
            store::ArtifactStoreOneItemAdvance::Published(_) => {
                metadata_published = true;
                break;
            }
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("admitted WAV metadata edit blocked"),
            _ => {}
        }
    }
    assert!(metadata_published);
    assert_eq!(store.snapshot_ref().fmt.sample_rate, 48_000);
    let WavData::Raw(samples) = &store.snapshot_ref().data else { panic!("raw samples retained after metadata edit") };
    assert_eq!(samples.len(), sample_count);
    assert_eq!(samples[edit_index], 9);
    assert!(metadata_publication.acknowledge());
    drain_publication(&mut metadata_publication);
    assert!(metadata_publication.terminal_is_empty());
    drop(metadata_publication);
    store.dispatch(store::ArtifactCommand::Undo).await.expect("metadata undo");
    assert_eq!(store.snapshot_ref().fmt.sample_rate, 44_100);
    store.dispatch(store::ArtifactCommand::Redo).await.expect("metadata redo");
    assert_eq!(store.snapshot_ref().fmt.sample_rate, 48_000);
    let native = crate::standards::riff_pcm::subsets::any::io::encode_wav(store.snapshot_ref());
    let reopened = crate::standards::riff_pcm::subsets::any::io::decode_wav(&native).expect("edited native WAV reopens");
    assert_eq!(reopened.fmt.sample_rate, 48_000);
    assert_eq!(crate::standards::riff_pcm::subsets::any::io::encode_wav(&reopened), native, "native WAV bytes stabilize after reopen");

    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<WavSnapshot, WavMutation>::new();
    for _ in 0..100_000 {
        let demand = semio_framework_plugin::ArtifactOwnedDisposer::retirement_demands(&disposer, &store, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("WAV store close demand");
        let funded = quoted_grant(demand);
        let step = semio_framework_plugin::ArtifactOwnedDisposer::close_step(&mut disposer, &mut store, funded).expect("WAV store closes");
        if matches!(step, semio_framework_plugin::PluginLifecycleStep::Complete(_)) {
            break;
        }
    }
    assert!(semio_framework_plugin::ArtifactOwnedDisposer::terminal_is_empty(&disposer, &store));
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::WavEditor, || semio_framework_plugin::App { definition: super::create_wav_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_plugin_media_and_isolates_fresh_owner_history() {
    use semio_framework_plugin::plugin_app_close_prelude::{MediaArtifact, MediaArtifactDescriptor};
    use semio_framework_plugin::{artifact_app_laws, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
    let source = crate::standards::riff_pcm::subsets::any::io::try_encode_wav(&WavSnapshot {
        fmt: crate::standards::riff_pcm::subsets::any::schema::snapshot::WavFmt { channels: 1, sample_rate: 8_000, byte_rate: 16_000, block_align: 2, bits_per_sample: 16, ..Default::default() },
        data: WavData::Pcm16(vec![0, 1_024, -1_024, 0]),
        ..WavSnapshot::default()
    })
    .expect("neutral natural WAV fixture encodes");
    let codec = <WavEditor as ArtifactEditor>::natural_file_codec().expect("paired natural codec");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.wav@riff-pcm", ".wav", "audio/wav", true));
    let artifact = MediaArtifact {
        descriptor: MediaArtifactDescriptor {
            edge_id: None,
            port_id: Some(NATURAL_FILE_PORT.into()),
            kind_id: Some(codec.format_kind.into()),
            media_type: None,
            wire: MediaWireFormat::Binary { format_kind: codec.format_kind.into() },
            blob_hash: None,
        },
        data: source,
    };
    let initial = <WavEditor as ArtifactEditor>::initial_snapshot();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<WavEditor>, _>(async { semio_framework_plugin::App { definition: create_wav_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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
    assert_eq!(app.snapshot().expect("refused WAV preserves the owner"), opened);
    let saved = app.produce_media(NATURAL_FILE_PORT).await.expect("registered natural save");
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    assert_eq!(saved.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let oracle = semio_s_artifact_stdio_wav_test_oracle::standards::v_riff_pcm::subsets::any::oracle_identity_round_trip(&saved.data).expect("riff independently reopens and writes the WAV export");
    let observed = semio_s_artifact_stdio_wav_test_oracle::standards::v_riff_pcm::subsets::any::project_wav_mutation(&saved.data).expect("project WAV export");
    let expected = semio_s_artifact_stdio_wav_test_oracle::standards::v_riff_pcm::subsets::any::project_wav_mutation(&oracle).expect("project independent WAV output");
    assert_eq!(observed, expected);
    let mut reopened = artifact_app_laws::new_registered_app::<EditorApp<WavEditor>, _>(async { semio_framework_plugin::App { definition: create_wav_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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
