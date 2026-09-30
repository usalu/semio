use super::*;
use crate::standards::riff_pcm::subsets::any::schema::mutations::set_snapshot;

/// 🧬️ Registers the document schema wav's declaration contributes — the registered contract every snapshot edit validates
/// against; a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor()]).expect("the wav document schema registers");
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

#[semio_framework_async_macros::async_test]
async fn one_mebibyte_sample_lane_edits_without_generic_value_expansion() {
    register_document_schema();
    let samples = vec![7u8; 1_048_576];
    let snapshot = WavSnapshot { data: WavData::Raw(samples.clone()), ..WavSnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: "/fmt/sampleRate".into(), value: dsl::DslValue::Number(dsl::Number::UInt(48_000)) };
    assert!(<WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let next = wavEditor_snapshot_edit(&event, &snapshot).expect("metadata edit");
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
    let event = editing::SnapshotEditEvent::SetValue { path: "/data/kind".into(), value: dsl::DslValue::String("pcm8".into()) };
    let next = wavEditor_snapshot_edit(&event, &snapshot).expect("data discriminator edit");
    assert_eq!(next.data, WavData::Pcm8(vec![1, 2]));
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("data discriminator edit emits");
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("data discriminator edit must emit one mutation") };
    let published = protocol::MutationDiff::apply(<WavMutation as protocol::Mutation<WavSnapshot>>::diff(mutation, &snapshot).diff(), &snapshot).expect("data discriminator mutation applies");
    assert_eq!(published.data, WavData::Pcm8(vec![1, 2]));
    let native = crate::standards::riff_pcm::subsets::any::io::encode_wav(&published);
    let reopened = crate::standards::riff_pcm::subsets::any::io::decode_wav(&native).expect("edited WAV reopens");
    assert_eq!(reopened.data, WavData::Pcm8(vec![1, 2]));
}

#[semio_framework_async_macros::async_test]
async fn chunk_layout_and_pad_bytes_are_visible_and_editable_details() {
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
    assert_eq!(editing::SnapshotDetailsProvider::value(&provider, &[editing::SnapshotDetailPathSegment::Key("dataPadByte".into())],), Some(editing::SnapshotDetailValue::Number(dsl::Number::UInt(0xA5))),);
    assert_eq!(
        editing::SnapshotDetailsProvider::value(&provider, &[editing::SnapshotDetailPathSegment::Key("otherChunks".into()), editing::SnapshotDetailPathSegment::Index(0), editing::SnapshotDetailPathSegment::Key("padByte".into()),],),
        Some(editing::SnapshotDetailValue::Number(dsl::Number::UInt(0x7F))),
    );
    let event = editing::SnapshotEditEvent::SetValue { path: "/dataPadByte".into(), value: dsl::DslValue::Number(dsl::Number::UInt(0x5A)) };
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("pad byte edit emits");
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("pad byte edit must emit one mutation") };
    assert!(matches!(mutation, WavMutation::PatchSnapshot(_)));
    let edited = protocol::MutationDiff::apply(<WavMutation as protocol::Mutation<WavSnapshot>>::diff(mutation, &snapshot).diff(), &snapshot).expect("pad byte mutation applies");
    assert_eq!(edited.data_pad_byte, 0x5A);
    assert_eq!(edited.chunk_order, snapshot.chunk_order);
    assert_eq!(edited.other_chunks, snapshot.other_chunks);
}

#[semio_framework_async_macros::async_test]
async fn sample_edit_set_snapshot_replays_and_inverts_without_losing_siblings() {
    let base = WavSnapshot { data: WavData::Raw(vec![1, 2, 3]), ..WavSnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: "/data/value/1".into(), value: dsl::DslValue::Number(dsl::Number::UInt(9)) };
    let next = wavEditor_snapshot_edit(&event, &base).expect("sample edit");
    assert_eq!(next.data, WavData::Raw(vec![1, 9, 3]));
    let mutation = set_snapshot::SetSnapshot { snapshot: next.clone() };
    let encoded = <WavMutation as store::OpBinary>::encode_op(&WavMutation::SetSnapshot(mutation.clone())).expect("binary encode");
    let decoded = <WavMutation as store::OpBinary>::decode_op(&encoded).expect("binary decode");
    let mut replayed = base.clone();
    crate::standards::riff_pcm::subsets::any::schema::mutations::apply_wav_mutation(&mut replayed, &decoded);
    assert_eq!(replayed, next);
    for inverse in <set_snapshot::SetSnapshot as protocol::MutationKind<WavSnapshot, WavMutation>>::inverse(&mutation, &base) {
        crate::standards::riff_pcm::subsets::any::schema::mutations::apply_wav_mutation(&mut replayed, &inverse);
    }
    assert_eq!(replayed, base);
}

#[semio_framework_async_macros::async_test]
async fn large_sample_edit_publishes_cancels_undoes_redoes_and_preserves_metadata() {
    register_document_schema();
    let sample_count = 2_097_152;
    let edit_index = 1_500_000;
    let snapshot = WavSnapshot { data: WavData::Raw(vec![7; sample_count]), ..WavSnapshot::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: format!("/data/value/{edit_index}"), value: dsl::DslValue::Number(dsl::Number::UInt(9)) };
    let started = std::time::Instant::now();
    let emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("sample edit emits");
    assert!(started.elapsed() < std::time::Duration::from_secs(2), "one sample edit of a {sample_count}-sample document took {:?}", started.elapsed());
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("sample edit must emit one mutation") };
    assert!(matches!(mutation, WavMutation::PatchData(_)));
    assert!(<WavMutation as protocol::OpBinary>::encode_op(mutation).expect("patch encodes").len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);

    let envelope = store::create_document_envelope(STDIO_WAV_DOCUMENT_SCHEMA, "wav-large-sample-publication", snapshot, None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("WAV store opens");
    store.install_document_store_owners_exact(store::bounded_artifact_store_owners::<WavSnapshot, WavMutation>());
    let factory = <WavEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("WAV retained factory");
    let generation = store.generation_now();
    let root = store.snapshot_root();
    let mut cancelled = store
        .begin_apply_batch(semio_framework_job::OperationId(1), generation, store.content_revision_now(), "wav-large-sample-cancel".into(), vec![mutation.clone()], None, store::HistoryLane::Document, Some(&factory), None)
        .expect("bounded patch cancellation candidate admits");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES };
    for _ in 0..64 {
        match store.advance_apply_batch(&mut cancelled, grant).expect("cancel candidate advances") {
            store::ArtifactStoreOneItemAdvance::Published(_) => panic!("candidate published before its cancellation point"),
            store::ArtifactStoreOneItemAdvance::Progress(_) if cancelled.staged_items() == 1 => break,
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("cancel candidate blocked"),
            _ => {}
        }
    }
    assert!(store.cancel_apply_batch(&mut cancelled));
    for _ in 0..64 {
        if matches!(cancelled.close_step(grant).expect("cancelled publication closes"), store::SnapshotRetirementStep::Complete) {
            break;
        }
    }
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
            Some("Edit WAV sample".into()),
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
    for _ in 0..64 {
        if matches!(publication.close_step(grant).expect("publication closes"), store::SnapshotRetirementStep::Complete) {
            break;
        }
    }
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

    let fmt_event = editing::SnapshotEditEvent::SetValue { path: "/fmt/sampleRate".into(), value: dsl::DslValue::Number(dsl::Number::UInt(48_000)) };
    let fmt_emit = <WavEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&fmt_event, store.snapshot_ref()).expect("metadata edit emits");
    let [fmt_mutation] = fmt_emit.artifact_mutations.as_slice() else { panic!("metadata edit must emit one mutation") };
    assert!(matches!(fmt_mutation, WavMutation::SetFmt(_)));
    let mut metadata_publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(3),
            store.generation_now(),
            store.content_revision_now(),
            "wav-large-metadata-publication".into(),
            vec![fmt_mutation.clone()],
            Some("Edit WAV metadata".into()),
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
    for _ in 0..64 {
        if matches!(metadata_publication.close_step(grant).expect("metadata publication closes"), store::SnapshotRetirementStep::Complete) {
            break;
        }
    }
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
        if matches!(semio_framework_plugin::ArtifactOwnedDisposer::close_step(&mut disposer, &mut store, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("WAV store closes"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(semio_framework_plugin::ArtifactOwnedDisposer::terminal_is_empty(&disposer, &store));
}
