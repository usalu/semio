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

fn pixel_region_command() -> PngEditCommand {
    let number = |value| dsl::DslValue::Number(dsl::Number::UInt(value));
    pngEditor_command_from_action(
        patch_pixel_region::ACTION_ID,
        Some(&dsl::DslValue::object([
            ("x".into(), number(1)),
            ("y".into(), number(1)),
            ("width".into(), number(2)),
            ("height".into(), number(1)),
            ("red".into(), number(10)),
            ("green".into(), number(20)),
            ("blue".into(), number(30)),
            ("alpha".into(), number(128)),
        ])),
    )
    .expect("typed pixel region action")
}

fn pixel_region_snapshot() -> PngSnapshot {
    PngSnapshot { width: 4, height: 3, pixels: [1, 2, 3, 255].repeat(12), ..PngSnapshot::default() }
}

fn drive_pixel_region(command: &PngEditCommand, snapshot: &PngSnapshot) -> Vec<PngMutation> {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "png-pixel-region-test".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "authoring-seed-test".into() };
    let mut work = patch_pixel_region::PatchPixelRegionWork::default();
    assert!(work.extent(command, snapshot, &interaction, None).is_some());
    for _ in 0..patch_pixel_region::CAPACITY.invertible_items() + 3 {
        match work.step(&ArtifactCommandInputs { command, snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation }).expect("pixel region work step") {
            ArtifactCommandWorkStep::Progress { preview, .. } => assert!(std::str::from_utf8(preview).expect("localized progress").contains("de")),
            ArtifactCommandWorkStep::Complete(emit) => return emit.artifact_mutations,
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => panic!("unexpected pixel region work step"),
        }
    }
    panic!("pixel region work did not complete")
}

#[semio_framework_async_macros::async_test]
async fn image_window_exposes_a_typed_localized_pixel_region_action() {
    let definition = create_png_editor();
    let action = definition.actions.iter().find(|action| action.id == patch_pixel_region::ACTION_ID).expect("pixel region app action");
    assert_eq!(action.args.len(), 8);
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert!(action.args.iter().all(|argument| argument.required));
    assert!(action.args.iter().all(|argument| matches!(argument.schema, semio_framework_plugin::ArgSchema::Number { integer: true, .. })));
    let window = definition.window_kinds.iter().find(|window| window.id == main::WINDOW_KIND_ID).expect("image window");
    let window_action = window.actions.iter().find(|action| action.id == patch_pixel_region::ACTION_ID).expect("visible image action");
    assert_eq!(window_action.args.len(), 8);
    assert_eq!(window_action.label.resolve(semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::En), "Paint Pixel Region");
    assert_eq!(window_action.label.resolve(semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::De), "Pixelbereich malen");
}

#[test]
fn retained_pixel_region_publishes_exact_patch_pixels_and_native_png() {
    use protocol::{Mutation, MutationDiff};
    let base = pixel_region_snapshot();
    let mutations = drive_pixel_region(&pixel_region_command(), &base);
    assert!(!mutations.is_empty());
    assert!(mutations.iter().all(|mutation| matches!(mutation, PngMutation::PatchPixels(_))));
    let mut edited = base.clone();
    for mutation in &mutations {
        edited = mutation.diff(&edited).diff().apply(&edited).expect("apply pixel region patch");
    }
    let expected = [
        1, 2, 3, 255, 1, 2, 3, 255, 1, 2, 3, 255, 1, 2, 3, 255,
        1, 2, 3, 255, 10, 20, 30, 128, 10, 20, 30, 128, 1, 2, 3, 255,
        1, 2, 3, 255, 1, 2, 3, 255, 1, 2, 3, 255, 1, 2, 3, 255,
    ];
    assert_eq!(edited.pixels, expected);
    let native = crate::io::encode_png(&edited).expect("pixel-edited PNG encodes");
    let reopened = crate::io::decode_png(&native).expect("pixel-edited PNG reopens");
    assert_eq!(reopened.pixels, expected);
}

#[test]
fn pixel_region_rejects_invalid_bounds_and_cancellation_discards_unpublished_patches() {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let snapshot = PngSnapshot { width: 1_024, height: 512, pixels: vec![7; 1_024 * 512 * 4], ..PngSnapshot::default() };
    let invalid = PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion { x: 1_024, y: 0, width: 1, height: 1, red: 0, green: 0, blue: 0, alpha: 0 }));
    let interaction = protocol::InteractionState::default();
    let invalid_work = patch_pixel_region::PatchPixelRegionWork::default();
    assert_eq!(invalid_work.extent(&invalid, &snapshot, &interaction, None), None);

    let command = PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion { x: 1, y: 0, width: 1, height: 512, red: 9, green: 8, blue: 7, alpha: 6 }));
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "png-pixel-cancel".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "authoring-seed-test".into() };
    let mut work = patch_pixel_region::PatchPixelRegionWork::default();
    let input = ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
    assert!(matches!(work.step(&input).expect("prepare"), ArtifactCommandWorkStep::Progress { .. }));
    assert!(matches!(work.step(&input).expect("first patch"), ArtifactCommandWorkStep::Progress { .. }));
    work.begin_close();
    while !work.terminal_is_empty() {
        assert!(!matches!(work.close_step(1, patch_pixel_region::PATCH_PAYLOAD_BYTES), semio_framework_job::InteractiveJobCloseStep::Blocked));
    }
    assert!(snapshot.pixels.iter().all(|value| *value == 7));
}

#[test]
fn retained_pixel_region_accepts_dci_4k_raster_with_bounded_patch_work() {
    use protocol::{Mutation, MutationDiff, OpBinary};
    let width = 4_096;
    let height = 2_160;
    let raster_bytes = width * height * 4;
    assert_eq!(raster_bytes, patch_pixel_region::MAXIMUM_RASTER_BYTES);
    let snapshot = PngSnapshot { width: width as u32, height: height as u32, pixels: vec![7; raster_bytes], ..PngSnapshot::default() };
    let command = PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion { x: width as u32 - 1, y: height as u32 - 1, width: 1, height: 1, red: 9, green: 8, blue: 7, alpha: 6 }));
    let mutations = drive_pixel_region(&command, &snapshot);
    assert_eq!(mutations.len(), 1);
    assert!(mutations[0].encode_op().expect("bounded DCI 4K patch encodes").len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    let edited = mutations[0].diff(&snapshot).diff().apply(&snapshot).expect("bounded DCI 4K patch applies");
    assert_eq!(&edited.pixels[raster_bytes - 4..], &[9, 8, 7, 6]);
    assert!(edited.pixels[..raster_bytes - 4].iter().all(|value| *value == 7));
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
