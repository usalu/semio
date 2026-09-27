use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_png_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, PNG_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<PngEditor as ArtifactEditor>::DIALECT, PNG_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_every_typed_snapshot_edit_action() {
    let definition = create_png_editor();
    for action_id in semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_ACTION_IDS {
        let action = definition.actions.iter().find(|action| action.id == *action_id).expect("typed snapshot edit action");
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    }
}

#[test]
fn snapshot_detail_edit_round_trips_through_native_history_and_codecs() {
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    let base = crate::schema::demo_png_snapshot();
    let event = editing::SnapshotEditEvent::SetValue { path: "/gama".into(), value: dsl::DslValue::Number(dsl::Number::UInt(50_000)) };
    let emit = <PngEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect("valid typed detail edit");
    let mutation = emit.artifact_mutations.into_iter().next().expect("one whole-snapshot mutation");

    let text = mutation.print_op();
    assert_eq!(PngMutation::parse_op(&text).expect("text replay"), mutation);
    let binary = mutation.encode_op().expect("binary encode");
    assert_eq!(PngMutation::decode_op(&binary).expect("binary replay"), mutation);

    let outcome = mutation.diff(&base);
    let edited = outcome.diff().apply(&base).expect("apply forward diff");
    assert_eq!(edited.gama, Some(50_000));
    assert_eq!(edited.pixels, base.pixels);
    let native = crate::io::encode_png(&edited).expect("edited PNG encodes to its native format");
    let reopened = crate::io::decode_png(&native).expect("edited native PNG reopens");
    assert_eq!(reopened.gama, edited.gama);
    assert_eq!(reopened.pixels, edited.pixels);

    let inverse = mutation.inverse(&base);
    assert_eq!(inverse.len(), 1);
    let restored = inverse[0].diff(&edited).diff().apply(&edited).expect("apply inverse history event");
    assert_eq!(restored, base);
}

#[test]
fn typed_snapshot_source_preserves_ancillary_and_unknown_chunk_details() {
    let mut base = crate::schema::demo_png_snapshot();
    base.gama = Some(u32::MAX - 1);
    base.unknown_chunks.push(crate::schema::snapshot::PngChunk { kind: *b"vpAg", data: vec![0, 1, 127, 128, 255] });
    let source = editing::snapshot_edit_source(&base);
    let event = editing::SnapshotEditEvent::ReplaceSource { source };
    let emit = <PngEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect("lossless typed source");
    let PngMutation::SetSnapshot(mutation) = &emit.artifact_mutations[0] else { panic!("whole snapshot mutation") };
    assert_eq!(mutation.snapshot, base);
    let native = crate::io::encode_png(&mutation.snapshot).expect("typed snapshot encodes to native PNG");
    let reopened = crate::io::decode_png(&native).expect("native PNG with ancillary data reopens");
    assert_eq!(reopened.gama, mutation.snapshot.gama);
    assert_eq!(reopened.unknown_chunks, mutation.snapshot.unknown_chunks);
}

#[semio_framework_async_macros::async_test]
async fn large_raster_metadata_and_pixel_edits_publish_and_replay_compactly() {
    use protocol::OpBinary;

    let pixel_count = 2_097_152;
    let edit_index = 1_500_000;
    let mut snapshot = crate::schema::demo_png_snapshot();
    snapshot.width = 1_024;
    snapshot.height = 512;
    snapshot.pixels = vec![7; pixel_count];
    let envelope = store::create_document_envelope(STDIO_PNG_DOCUMENT_SCHEMA, "png-large-raster-publication", snapshot, None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("PNG store opens");
    store.install_document_store_owners_exact(store::bounded_artifact_store_owners::<PngSnapshot, PngMutation>());
    let factory = <PngEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("PNG retained factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES };

    let gamma_event = editing::SnapshotEditEvent::SetValue { path: "/gama".into(), value: dsl::DslValue::Number(dsl::Number::UInt(50_000)) };
    let gamma_emit = <PngEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&gamma_event, store.snapshot_ref()).expect("gamma edit emits");
    let [gamma] = gamma_emit.artifact_mutations.as_slice() else { panic!("one gamma mutation") };
    assert!(matches!(gamma, PngMutation::ChangeGamma(_)));
    assert!(gamma.encode_op().expect("gamma encodes").len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    let mut gamma_publication = store.begin_apply_batch(semio_framework_job::OperationId(11), store.generation_now(), store.content_revision_now(), "png-large-gamma".into(), vec![gamma.clone()], Some("Edit PNG gamma".into()), store::HistoryLane::Document, Some(&factory)).expect("compact gamma admits against large raster");
    let mut gamma_published = false;
    for _ in 0..64 {
        match store.advance_apply_batch(&mut gamma_publication, grant).expect("gamma publication advances") {
            store::ArtifactStoreOneItemAdvance::Published(_) => { gamma_published = true; break; }
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("admitted gamma edit blocked"),
            _ => {}
        }
    }
    assert!(gamma_published);
    assert_eq!(store.snapshot_ref().gama, Some(50_000));
    assert_eq!(store.snapshot_ref().pixels.len(), pixel_count);
    assert!(gamma_publication.acknowledge());
    for _ in 0..64 { if matches!(gamma_publication.close_step(grant).expect("gamma publication closes"), store::SnapshotRetirementStep::Complete) { break; } }
    assert!(gamma_publication.terminal_is_empty());
    drop(gamma_publication);
    store.dispatch(store::ArtifactCommand::Undo).await.expect("gamma undo");
    assert_eq!(store.snapshot_ref().gama, Some(45_455));
    assert_eq!(store.snapshot_ref().pixels.len(), pixel_count);
    store.dispatch(store::ArtifactCommand::Redo).await.expect("gamma redo");
    assert_eq!(store.snapshot_ref().gama, Some(50_000));

    let pixel_event = editing::SnapshotEditEvent::SetValue { path: format!("/pixels/{edit_index}"), value: dsl::DslValue::Number(dsl::Number::UInt(9)) };
    let pixel_emit = <PngEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&pixel_event, store.snapshot_ref()).expect("pixel edit emits");
    let [pixel] = pixel_emit.artifact_mutations.as_slice() else { panic!("one pixel mutation") };
    assert!(matches!(pixel, PngMutation::PatchPixels(_)));
    assert!(pixel.encode_op().expect("pixel patch encodes").len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    let mut pixel_publication = store.begin_apply_batch(semio_framework_job::OperationId(12), store.generation_now(), store.content_revision_now(), "png-large-pixel".into(), vec![pixel.clone()], Some("Edit PNG pixel".into()), store::HistoryLane::Document, Some(&factory)).expect("compact pixel patch admits against large raster");
    let mut pixel_published = false;
    for _ in 0..64 {
        match store.advance_apply_batch(&mut pixel_publication, grant).expect("pixel publication advances") {
            store::ArtifactStoreOneItemAdvance::Published(_) => { pixel_published = true; break; }
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("admitted pixel edit blocked"),
            _ => {}
        }
    }
    assert!(pixel_published);
    assert_eq!(store.snapshot_ref().pixels.len(), pixel_count);
    assert_eq!(store.snapshot_ref().pixels[edit_index], 9);
    assert_eq!(store.snapshot_ref().gama, Some(50_000));
    assert!(pixel_publication.acknowledge());
    for _ in 0..64 { if matches!(pixel_publication.close_step(grant).expect("pixel publication closes"), store::SnapshotRetirementStep::Complete) { break; } }
    assert!(pixel_publication.terminal_is_empty());
    drop(pixel_publication);
    store.dispatch(store::ArtifactCommand::Undo).await.expect("pixel undo");
    assert_eq!(store.snapshot_ref().pixels[edit_index], 7);
    assert_eq!(store.snapshot_ref().gama, Some(50_000));
    store.dispatch(store::ArtifactCommand::Redo).await.expect("pixel redo");
    assert_eq!(store.snapshot_ref().pixels[edit_index], 9);

    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<PngSnapshot, PngMutation>::new();
    for _ in 0..100_000 {
        if matches!(semio_framework_plugin::ArtifactOwnedDisposer::close_step(&mut disposer, &mut store, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("PNG store closes"), semio_framework_plugin::PluginCloseStep::Complete) { break; }
    }
    assert!(semio_framework_plugin::ArtifactOwnedDisposer::terminal_is_empty(&disposer, &store));
}
