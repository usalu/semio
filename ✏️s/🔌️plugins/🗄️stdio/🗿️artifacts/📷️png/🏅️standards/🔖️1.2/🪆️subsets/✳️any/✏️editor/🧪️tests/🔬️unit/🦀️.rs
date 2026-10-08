use super::*;

fn rgba_snapshot(width: u32, height: u32, pixels: Vec<u8>) -> PngSnapshot {
    let projection = crate::standards::v1_2::subsets::any::io::PngProjection {
        width,
        height,
        bit_depth: 8,
        color_type: crate::schema::snapshot::PngColorType::Rgba,
        interlace: false,
        plte: None,
        trns: None,
        gama: None,
        chrm: None,
        srgb: None,
        phys: None,
        time: None,
        bkgd: None,
        text_chunks: Vec::new(),
        pixels,
        chunk_order: vec![crate::standards::v1_2::subsets::any::io::PngChunkMarker::Ihdr, crate::standards::v1_2::subsets::any::io::PngChunkMarker::Idat, crate::standards::v1_2::subsets::any::io::PngChunkMarker::Iend],
        unknown_chunks: Vec::new(),
    };
    crate::standards::v1_2::subsets::any::io::decode_png(&crate::standards::v1_2::subsets::any::io::author_png_projection(&projection).unwrap()).unwrap()
}

fn pixel_region_command() -> PngEditCommand {
    let number = |value| semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value));
    pngEditor_command_from_action(patch_pixel_region::ACTION_ID, Some(&semio_framework_value::DslValue::object([
        ("x".into(), number(1)), ("y".into(), number(1)), ("width".into(), number(2)), ("height".into(), number(1)),
        ("red".into(), number(10)), ("green".into(), number(20)), ("blue".into(), number(30)), ("alpha".into(), number(128)),
    ]))).unwrap()
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
        let mut sequence = 0;
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match work.step(&ArtifactCommandInputs { snapshot_owner: None, command, snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation }, &mut cx).unwrap() {
            ArtifactCommandWorkStep::Progress { preview, .. } => assert!(std::str::from_utf8(preview).unwrap().contains("de")),
            ArtifactCommandWorkStep::Complete(emit) => return emit.artifact_mutations,
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::CompleteWithEphemeral { .. } | ArtifactCommandWorkStep::CompleteDownload { .. } => panic!("unexpected pixel region work step"),
        }
    }
    panic!("pixel region work did not complete")
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let definition = create_png_editor();
    assert_eq!(definition.role, semio_framework::AppRole::Editor);
    assert_eq!(definition.dialect, PNG_DIALECT.into());
    for action_id in semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_ACTION_IDS {
        let action = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == *action_id).unwrap();
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    }
}

#[semio_framework_async_macros::async_test]
async fn image_window_exposes_a_typed_localized_pixel_region_action() {
    let definition = create_png_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == main::WINDOW_KIND_ID).unwrap();
    let action = window.actions.iter().find(|action| action.id == patch_pixel_region::ACTION_ID).unwrap();
    assert_eq!(action.args.len(), 8);
    assert!(action.args.iter().all(|argument| argument.required && matches!(argument.schema, semio_framework_plugin::ArgSchema::Number { integer: true, .. })));
    assert_eq!(action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Paint Pixel Region");
    assert_eq!(action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Pixelbereich malen");
}

#[semio_framework_async_macros::async_test]
async fn image_window_exposes_accessible_native_profile_paint_actions() {
    let definition = create_png_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == main::WINDOW_KIND_ID).unwrap();
    let expected = [
        (paint_native_region::INDEXED_ACTION_ID, 5, "Paint Palette Index Region", "Palettenindexbereich malen"),
        (paint_native_region::GRAYSCALE_ACTION_ID, 5, "Paint Grayscale Region", "Graustufenbereich malen"),
        (paint_native_region::GRAYSCALE_ALPHA_ACTION_ID, 6, "Paint Grayscale Alpha Region", "Graustufen-Alpha-Bereich malen"),
        (paint_native_region::RGB_ACTION_ID, 7, "Paint Native RGB Region", "Nativen RGB-Bereich malen"),
        (paint_native_region::RGBA_ACTION_ID, 8, "Paint Native RGBA Region", "Nativen RGBA-Bereich malen"),
    ];
    for (id, count, english, german) in expected {
        let action = window.actions.iter().find(|action| action.id == id).unwrap();
        assert_eq!(action.args.len(), count);
        assert!(action.args.iter().all(|argument| argument.required && !argument.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).is_empty() && !argument.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).is_empty()));
        assert_eq!(action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), english);
        assert_eq!(action.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), german);
    }
}

#[test]
fn native_profile_action_parses_checked_samples_and_rejects_mismatch() {
    let number = |value| semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value));
    let arguments = semio_framework_value::DslValue::object([
        ("x".into(), number(0)), ("y".into(), number(0)), ("width".into(), number(1)), ("height".into(), number(1)),
        ("gray".into(), number(0x1234)),
    ]);
    let command = pngEditor_command_from_action(paint_native_region::GRAYSCALE_ACTION_ID, Some(&arguments)).unwrap();
    let PngEditCommand::Native(PngNativeEditCommand::PaintNativeRegion(command)) = command else { panic!("native PNG command") };
    assert_eq!(command.paint, crate::standards::v1_2::subsets::any::schema::snapshot::PngNativePaint::grayscale(0x1234));
    let indexed = crate::standards::v1_2::subsets::any::io::decode_png(include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/indexed-2bit-duplicate-palette.png")).unwrap();
    assert!(crate::standards::v1_2::subsets::any::schema::operations::validate_native_paint(&indexed, command.region, command.paint).unwrap_err().contains("profile"));
}

#[test]
fn native_profile_action_defaults_are_valid_for_eight_and_sixteen_bit_sources() {
    let definitions = paint_native_region::actions();
    let arguments = |id: &str| {
        let action = definitions.iter().find(|action| action.id == id).unwrap();
        semio_framework_value::DslValue::object(action.args.iter().map(|argument| (argument.id.clone(), argument.default.clone().expect("native paint argument default"))))
    };
    let rgba = pngEditor_command_from_action(paint_native_region::RGBA_ACTION_ID, Some(&arguments(paint_native_region::RGBA_ACTION_ID))).unwrap();
    let rgba8 = crate::standards::v1_2::subsets::any::io::decode_png(include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-adam7.png")).unwrap();
    let PngEditCommand::Native(PngNativeEditCommand::PaintNativeRegion(rgba)) = rgba else { panic!("RGBA native paint") };
    assert!(crate::standards::v1_2::subsets::any::schema::operations::validate_native_paint(&rgba8, rgba.region, rgba.paint).is_ok());
    assert_eq!(rgba.paint.fourth, u16::from(u8::MAX));

    let grayscale = pngEditor_command_from_action(paint_native_region::GRAYSCALE_ACTION_ID, Some(&arguments(paint_native_region::GRAYSCALE_ACTION_ID))).unwrap();
    let grayscale16 = crate::standards::v1_2::subsets::any::io::decode_png(include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/precision-16bit-gray.png")).unwrap();
    let PngEditCommand::Native(PngNativeEditCommand::PaintNativeRegion(grayscale)) = grayscale else { panic!("grayscale native paint") };
    assert!(crate::standards::v1_2::subsets::any::schema::operations::validate_native_paint(&grayscale16, grayscale.region, grayscale.paint).is_ok());
}

#[test]
fn retained_native_profile_work_publishes_one_revision_guarded_mutation() {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let reader = std::sync::Arc::new(crate::standards::v1_2::subsets::any::io::decode_png(include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/precision-16bit-gray.png")).unwrap());
    let snapshot = reader.as_ref();
    let command = PngEditCommand::Native(PngNativeEditCommand::PaintNativeRegion(paint_native_region::PaintNativeRegion {
        region: crate::standards::v1_2::subsets::any::schema::snapshot::PngRegion { x: 1, y: 0, width: 1, height: 1 },
        paint: crate::standards::v1_2::subsets::any::schema::snapshot::PngNativePaint::grayscale(0x1234),
    }));
    let config = NoConfig::default(); let history = semio_framework_plugin::HistoryView::empty(); let interaction = protocol::InteractionState::default(); let hover = Default::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "png-native-region".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "png-native".into() };
    let input = ArtifactCommandInputs { command: &command, snapshot, snapshot_owner: Some(&reader), config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
    let mut work = paint_native_region::PaintNativeRegionWork::new(paint_native_region::GRAYSCALE_ACTION_ID);
    let mut sequence = 0; let mut mutations = None;
    for _ in 0..10_000 {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match work.step(&input, &mut cx).unwrap() {
            ArtifactCommandWorkStep::Progress { preview, .. } => assert!(std::str::from_utf8(preview).unwrap().contains("de")),
            ArtifactCommandWorkStep::Complete(emit) => { mutations = Some(emit.artifact_mutations); break; }
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::CompleteWithEphemeral { .. } | ArtifactCommandWorkStep::CompleteDownload { .. } => panic!("unexpected native PNG work step"),
        }
    }
    let mutations = mutations.expect("native PNG work completes");
    assert_eq!(mutations.len(), 1);
    let PngMutation::PaintNativeSamples(mutation) = &mutations[0] else { panic!("native PNG mutation") };
    assert_eq!(mutation.revision, crate::standards::v1_2::subsets::any::schema::operations::png_revision(&snapshot));
    use protocol::{Mutation, MutationDiff};
    let edited = protocol::apply_diff(&mutations[0].diff(&snapshot).diff(), &snapshot).unwrap();
    assert_eq!(crate::standards::v1_2::subsets::any::schema::operations::png_native_pixel(&edited, 1, 0).unwrap(), vec![0x1234]);
    work.begin_close();
    let probe = work.close_step(1, 1);
    assert!(matches!(probe, semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= 1));
    for _ in 0..100_000 {
        if work.terminal_is_empty() { break; }
        work.close_step(1, usize::MAX);
    }
    assert!(work.terminal_is_empty());
}

#[test]
fn retained_pixel_region_publishes_one_revision_guarded_edit_and_native_png() {
    use protocol::{Mutation, MutationDiff};
    let base = rgba_snapshot(4, 3, [1, 2, 3, 255].repeat(12));
    let mutations = drive_pixel_region(&pixel_region_command(), &base);
    assert_eq!(mutations.len(), 1);
    let PngMutation::PatchPixels(patch) = &mutations[0] else { panic!("typed region mutation") };
    assert_eq!(patch.revision, crate::standards::v1_2::subsets::any::schema::operations::png_revision(&base));
    let edited = protocol::apply_diff(&mutations[0].diff(&base).diff(), &base).unwrap();
    let pixels = crate::standards::v1_2::subsets::any::io::project_png(&crate::standards::v1_2::subsets::any::io::encode_png(&edited).unwrap()).unwrap().pixels;
    assert_eq!(&pixels[20..28], &[10, 20, 30, 128, 10, 20, 30, 128]);
    assert_eq!(crate::standards::v1_2::subsets::any::io::decode_png(&crate::standards::v1_2::subsets::any::io::encode_png(&edited).unwrap()).unwrap(), edited);
}

#[test]
fn natural_file_route_exports_edited_and_committed_native_profiles_exactly() {
    let edited = rgba_snapshot(2, 1, vec![7, 8, 9, 255, 21, 22, 23, 128]);
    let bytes = <PngEditor as ArtifactEditor>::encode_natural_file(&edited).expect("PNG natural bytes");
    let mut independent = png::Decoder::new(std::io::Cursor::new(&bytes)).read_info().expect("png crate accepts exported bytes");
    let mut samples = vec![0; independent.output_buffer_size().expect("bounded PNG output")];
    let frame = independent.next_frame(&mut samples).expect("png crate decodes samples");
    assert_eq!((frame.width, frame.height, frame.color_type), (2, 1, png::ColorType::Rgba));
    assert_eq!(&samples[..frame.buffer_size()], &[7, 8, 9, 255, 21, 22, 23, 128]);
    let reopened = <PngEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("PNG natural bytes reopen");
    assert_eq!(reopened, edited);

    let fixtures: &[(&str, &[u8], png::ColorType, png::BitDepth, bool)] = &[
        ("16-bit grayscale", include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/precision-16bit-gray.png"), png::ColorType::Grayscale, png::BitDepth::Sixteen, false),
        ("indexed duplicate palette", include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/indexed-2bit-duplicate-palette.png"), png::ColorType::Indexed, png::BitDepth::Two, false),
        ("Adam7 RGBA", include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-adam7.png"), png::ColorType::Rgba, png::BitDepth::Eight, true),
    ];
    for &(name, source, color_type, bit_depth, interlaced) in fixtures {
        let snapshot = <PngEditor as ArtifactEditor>::decode_natural_file(source).unwrap_or_else(|error| panic!("{name}: natural decode failed: {error}"));
        assert_eq!(crate::standards::v1_2::subsets::any::io::decode_png(&crate::standards::v1_2::subsets::any::io::encode_png(&snapshot).unwrap()).unwrap(),snapshot,"{name}: owned import/export roundtrip");
        let exported = <PngEditor as ArtifactEditor>::encode_natural_file(&snapshot).unwrap_or_else(|error| panic!("{name}: natural encode failed: {error}"));
        assert_eq!(crate::standards::v1_2::subsets::any::io::decode_png(&exported).unwrap(), snapshot, "{name}: natural export preserves the owned image");
        let mut decoder = png::Decoder::new(std::io::Cursor::new(&exported));
        decoder.set_transformations(png::Transformations::IDENTITY);
        let mut independent = decoder.read_info().unwrap_or_else(|error| panic!("{name}: png crate rejected natural export: {error}"));
        assert_eq!((independent.info().color_type, independent.info().bit_depth, independent.info().interlaced), (color_type, bit_depth, interlaced), "{name}: native profile changed");
        let mut decoded = vec![0; independent.output_buffer_size().expect("bounded PNG output")];
        independent.next_frame(&mut decoded).unwrap_or_else(|error| panic!("{name}: png crate sample decode failed: {error}"));
    }
}

#[test]
fn pixel_region_rejects_invalid_bounds_and_cancellation_publishes_nothing() {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
    let snapshot = rgba_snapshot(1_024, 512, vec![7; 1_024 * 512 * 4]);
    let invalid = PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion { x: 1_024, y: 0, width: 1, height: 1, red: 0, green: 0, blue: 0, alpha: 0 }));
    let interaction = protocol::InteractionState::default();
    assert_eq!(patch_pixel_region::PatchPixelRegionWork::default().extent(&invalid, &snapshot, &interaction, None), None);
    let command = PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion { x: 1, y: 0, width: 1, height: 512, red: 9, green: 8, blue: 7, alpha: 6 }));
    let config = NoConfig::default(); let history = semio_framework_plugin::HistoryView::empty(); let hover = Default::default();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "png-pixel-cancel".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "authoring-seed-test".into() };
    let input = ArtifactCommandInputs { snapshot_owner: None, command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
    let mut work = patch_pixel_region::PatchPixelRegionWork::default();
    let mut sequence = 0; let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(work.step(&input, &mut cx).unwrap(), ArtifactCommandWorkStep::Progress { .. }));
    assert!(matches!(work.step(&input, &mut cx).unwrap(), ArtifactCommandWorkStep::Progress { .. }));
    work.begin_close();
    assert!(matches!(work.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete));
    assert!(work.terminal_is_empty());
    assert!(crate::standards::v1_2::subsets::any::io::project_png(&crate::standards::v1_2::subsets::any::io::encode_png(&snapshot).unwrap()).unwrap().pixels.iter().all(|value| *value == 7));
}

#[test]
fn retained_pixel_region_preserves_full_128_patch_admission() {
    use semio_s_artifact_stdio_contract::editing::raster::{RasterRegion, RasterRegionLimits, RasterRegionPlan};
    let width = 4_096; let height = 2_048; let pixels = vec![7; width * height * 4];
    let plan = RasterRegionPlan::new(width as u32, height as u32, pixels.len(), RasterRegion { x: 0, y: 0, width: width as u32, height: height as u32, color: [9, 8, 7, 6] }, RasterRegionLimits { maximum_raster_bytes: patch_pixel_region::MAXIMUM_RASTER_BYTES, maximum_patch_bytes: patch_pixel_region::PATCH_PAYLOAD_BYTES, maximum_patches: patch_pixel_region::CAPACITY.invertible_items() }).unwrap();
    assert_eq!(plan.patch_count(), 128);
    let last = plan.patch(&pixels, 127).unwrap().unwrap();
    assert_eq!(last.index + last.pixels.len(), pixels.len());
}

#[test]
fn retained_pixel_region_accepts_dci_4k_raster_with_bounded_work() {
    use protocol::OpBinary;
    use semio_s_artifact_stdio_contract::editing::raster::{RasterRegion, RasterRegionLimits, RasterRegionPlan};
    let width = 4_096usize; let height = 2_160usize; let raster_bytes = width * height * 4;
    let pixels = vec![7; raster_bytes];
    let plan = RasterRegionPlan::new(width as u32, height as u32, pixels.len(), RasterRegion { x: width as u32 - 1, y: height as u32 - 1, width: 1, height: 1, color: [9, 8, 7, 6] }, RasterRegionLimits { maximum_raster_bytes: patch_pixel_region::MAXIMUM_RASTER_BYTES, maximum_patch_bytes: patch_pixel_region::PATCH_PAYLOAD_BYTES, maximum_patches: patch_pixel_region::CAPACITY.invertible_items() }).unwrap();
    assert_eq!(plan.patch_count(), 1);
    let patch = plan.patch(&pixels, 0).unwrap().unwrap();
    assert_eq!(patch.index, raster_bytes - width * 4);
    assert_eq!(patch.pixels.len(), width * 4);
    assert!(patch.pixels[..patch.pixels.len() - 4].iter().all(|byte| *byte == 7));
    assert_eq!(&patch.pixels[patch.pixels.len() - 4..], &[9, 8, 7, 6]);
    let mutation = PngMutation::PatchPixels(crate::schema::mutations::PatchPixelsMutation { revision: "0000000000000000".into(), x: width as u32 - 1, y: height as u32 - 1, width: 1, height: 1, red: 9, green: 8, blue: 7, alpha: 6 });
    assert!(mutation.encode_op().unwrap().len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
}

#[test]
fn registered_pixel_region_factory_cancellation_retires_without_publication() {
    use protocol::OpBinary;
    use semio_framework_job::{Generation, InteractiveJob, Operation, OperationId, RevisionId, StepBudget, StepContext, StepOutcome, JOB_PAYLOAD_PAGE_BYTES};
    use semio_framework_plugin::action_bus::{ActionBus, ToolOperationSpec, ToolWirePage, TOOL_WIRE_PAGE_BYTES};
    use semio_framework_plugin::app::ArtifactToolCompletion;
    use std::sync::Arc;
    let snapshot = Arc::new(rgba_snapshot(1_024, 512, vec![7; 1_024 * 512 * 4]));
    let command = PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion { x: 1, y: 0, width: 1, height: 512, red: 9, green: 8, blue: 7, alpha: 6 }));
    let wire = command.encode_op().unwrap(); let completion = ArtifactToolCompletion::<EditorApp<PngEditor>>::test_new(); let consumer = completion.clone();
    let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command, snapshot: snapshot.clone(), config: Arc::new(NoConfig::default()), history: Arc::new(semio_framework_plugin::HistoryView::empty()), interaction_state: Arc::new(Default::default()), interaction_hover: Arc::new(Default::default()), context: None, operation: AppOperationContext { app_instance_id: 1, parent_document_id: "png-registered-cancel".into(), operation_id: 41, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "png-cancel".into() }, completion }, pngEditor_command_id, patch_pixel_region::MAXIMUM_RAW_BYTES, patch_pixel_region::CAPACITY.work_items(), Box::new(patch_pixel_region::PatchPixelRegionWork::default())).unwrap();
    let bus = ActionBus::new(); bus.register(PngPixelRegionFactory::new("png-cancel")).unwrap();
    let (admission, mut input) = bus.begin_exact_wire("png-cancel", patch_pixel_region::ACTION_ID, patch_pixel_region::PAYLOAD_SCHEMA, wire.len()).unwrap();
    for page in wire.chunks(TOOL_WIRE_PAGE_BYTES) { input.admit_page(ToolWirePage::try_copy_from(page).unwrap()).map_err(|(fault, _)| fault).unwrap(); } input.seal().unwrap();
    let operation = Operation::new(OperationId(41), RevisionId(2), Generation(3), 5);
    let spec = ToolOperationSpec::new("png-cancel", patch_pixel_region::ACTION_ID, patch_pixel_region::PAYLOAD_SCHEMA, payload, operation);
    let mut dispatched = bus.dispatch_wire_retained_with_spec(&admission, input, None, spec).map_err(|rejected| rejected.error).unwrap();
    let mut sequence = 0; let mut patched = false;
    for _ in 0..32 {
        let mut cx = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        let mut outcome = dispatched.job.step(&mut cx); assert!(matches!(outcome, StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) | StepOutcome::Yield));
        patched = cx.stage() == "png-pixel-region-patch"; while !outcome.terminal_is_empty() { outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES); } if patched { break; }
    }
    assert!(patched); dispatched.job.begin_close();
    for _ in 0..10_000 { if dispatched.job.terminal_is_empty() { break; } dispatched.job.close_step(1, JOB_PAYLOAD_PAGE_BYTES); }
    assert!(dispatched.job.terminal_is_empty()); assert!(consumer.test_take_emit().unwrap().is_none());
}

#[semio_framework_async_macros::async_test]
async fn registered_pixel_region_refuses_an_equal_length_later_snapshot() {
    use protocol::OpText;
    use semio_framework_plugin::{artifact_app_laws, PluginApp};
    let mut expected = rgba_snapshot(1_024, 512, vec![7; 1_024 * 512 * 4]);
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<PngEditor>, _>(async { semio_framework_plugin::App { definition: create_png_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&expected, STDIO_PNG_DOCUMENT_SCHEMA) else { panic!("PNG fixture load") };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let revision = app.test_document_revision(); let number = |value| semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value));
    let arguments = semio_framework_value::DslValue::object([("x".into(), number(0)), ("y".into(), number(0)), ("width".into(), number(1_024)), ("height".into(), number(512)), ("red".into(), number(9)), ("green".into(), number(8)), ("blue".into(), number(7)), ("alpha".into(), number(6))]);
    let meta = artifact_app_laws::meta("local"); app.handle_action(patch_pixel_region::ACTION_ID, Some(&arguments), &meta).await.unwrap(); assert_eq!(app.test_document_revision(), revision);
    let concurrent = crate::standards::v1_2::subsets::any::schema::operations::paint_rgba8_region_controlled(&expected, &crate::standards::v1_2::subsets::any::schema::operations::png_revision(&expected), crate::standards::v1_2::subsets::any::schema::snapshot::PngRegion { x: 0, y: 0, width: 1, height: 1 }, [31, 32, 33, 34], &mut |_, _| true).unwrap();
    let mutation = PngMutation::ReplaceImage(crate::schema::mutations::ReplaceImage { image: concurrent.image.clone() });
    app.ingest_operations_text(&mutation.print_op()).await.unwrap(); expected = concurrent;
    let fault = match artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await { Err(fault) => fault, Ok(_) => panic!("stale snapshot was accepted") };
    assert!(fault.message.contains("stale immutable document root")); assert_eq!(app.snapshot().unwrap(), expected);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[test]
fn metadata_edit_and_inverse_preserve_exact_source_authority() {
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    let base = crate::standards::v1_2::subsets::any::io::decode_png(include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-multi-idat-private.png")).unwrap();
    let mutation = PngMutation::ChangeGamma(crate::schema::mutations::ChangeGammaMutation { revision: crate::standards::v1_2::subsets::any::schema::operations::png_revision(&base), gama: Some(50_000) });
    assert_eq!(PngMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert_eq!(PngMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    let edited = protocol::apply_diff(&mutation.diff(&base).diff(), &base).unwrap(); assert_eq!(crate::standards::v1_2::subsets::any::io::project_png(&crate::standards::v1_2::subsets::any::io::encode_png(&edited).unwrap()).unwrap().gama, Some(50_000));
    let restored = protocol::apply_diff(&mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&edited).diff(), &edited).unwrap(); assert_eq!(restored, base);
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::PngEditor, || semio_framework_plugin::App { definition: super::create_png_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.2/🪆️subsets/✳️any");
