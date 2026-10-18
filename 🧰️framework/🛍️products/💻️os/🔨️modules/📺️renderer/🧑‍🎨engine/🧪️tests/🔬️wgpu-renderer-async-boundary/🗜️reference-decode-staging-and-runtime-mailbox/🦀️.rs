/// 🗜️ Reference decode staging and runtime mailbox: the retained image decoder reference plan, the staging token, writer and pool reuse laws, the native reference decode cancellation, the structure scanners, the runtime mailbox and the presenter interaction restore.

/// 🖼️ The reference underlay the puzzle3d playground ships
/// (`/infinite-assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`, 2275×2560 = 23 296 000
/// straight-RGBA bytes) must DECODE — React's `WorldReferenceLayer` paints it — and a genuine pixel
/// bomb must be REFUSED as one missing asset, never as a frame fault that quarantines the surface.
#[test]
fn retained_image_decoder_admits_a_reference_plan_and_rejects_a_pixel_bomb_without_faulting() {
    fn jpeg(width: u16, height: u16) -> Vec<u8> {
        let [height_high, height_low] = height.to_be_bytes();
        let [width_high, width_low] = width.to_be_bytes();
        vec![0xff, 0xd8, 0xff, 0xc0, 0, 11, 8, height_high, height_low, width_high, width_low, 1, 1, 0x11, 0, 0xff, 0xd9]
    }
    fn png(width: u32, height: u32) -> Vec<u8> {
        [b"\x89PNG\r\n\x1a\n".as_slice(), &[0, 0, 0, 13], b"IHDR", &width.to_be_bytes(), &height.to_be_bytes(), &[8, 6, 0, 0, 0], &[0; 4], &[0, 0, 0, 1], b"IDAT", &[0], &[0; 4], &[0, 0, 0, 0], b"IEND", &[0; 4]].concat()
    }
    fn feed(bytes: &[u8], cursor: &mut RendererAssetFormatCursor) -> Result<(), &'static str> {
        for block in bytes.chunks(RENDERER_ASSET_PARSE_BLOCK_BYTES) {
            cursor.feed(block)?;
        }
        cursor.finish()
    }
    fn probe_image(bytes: &[u8]) -> (WorldAssetIoAuthority, RendererAssetProbe) {
        let mut authority = WorldAssetIoAuthority::default();
        authority.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "reference.jpg", bytes.len()).unwrap();
        let mut owner = authority.take_next().unwrap();
        owner.push_page(WorldAssetResponsePage::try_from_owned(bytes.to_vec()).unwrap()).unwrap();
        owner.seal().unwrap();
        authority.return_owner(owner).unwrap();
        let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).expect("completed probe owner");
        (authority, RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)))
    }
    fn drive(bytes: &[u8]) -> (WorldAssetIoAuthority, RendererAssetProbe, RendererAssetProbeStep) {
        let (authority, mut probe) = probe_image(bytes);
        for _ in 0..4_096 {
            match probe.step() {
                RendererAssetProbeStep::Pending => {}
                step => return (authority, probe, step),
            }
        }
        panic!("bounded image probe never reached a terminal step");
    }

    let plan = jpeg(2275, 2560);
    let mut cursor = RendererAssetFormatCursor::new(WorldAssetRequestKind::ReferenceImage, &plan, plan.len()).expect("JPEG format cursor");
    feed(&plan, &mut cursor).expect("a 2275x2560 reference plan is a legal image, not a bomb");
    let plan = png(2275, 2560);
    let mut cursor = RendererAssetFormatCursor::new(WorldAssetRequestKind::ReferenceImage, &plan, plan.len()).expect("PNG format cursor");
    feed(&plan, &mut cursor).expect("the same dimensions decode through the PNG scanner");

    assert_eq!(RENDERER_ASSET_PIXEL_BYTES, 64 * 1024 * 1024, "the ceiling is 16 megapixels of straight RGBA");
    let bomb = jpeg(4097, 4097);
    let mut cursor = RendererAssetFormatCursor::new(WorldAssetRequestKind::ReferenceImage, &bomb, bomb.len()).expect("JPEG format cursor");
    assert_eq!(feed(&bomb, &mut cursor), Err("JPEG dimensions exceeded fixed pixel credits"));
    let bomb = png(4097, 4097);
    let mut cursor = RendererAssetFormatCursor::new(WorldAssetRequestKind::ReferenceImage, &bomb, bomb.len()).expect("PNG format cursor");
    assert_eq!(feed(&bomb, &mut cursor), Err("PNG dimensions exceeded fixed pixel credits"));

    let (mut authority, mut probe, step) = drive(&jpeg(2275, 2560));
    assert!(matches!(step, RendererAssetProbeStep::Ready), "the playground's own reference plan reaches Ready");
    probe.begin_close();
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!("shared probe") };
    authority.finish(owner).unwrap();

    let (mut authority, mut probe, step) = drive(&jpeg(4097, 4097));
    assert!(matches!(step, RendererAssetProbeStep::Reject("JPEG dimensions exceeded fixed pixel credits")), "a refused image is ONE missing asset — a Fault here quarantines the whole surface");
    let (detail, kind, url) = probe.take_rejection().expect("a rejection carries the lane its miss belongs to");
    assert_eq!(detail, "JPEG dimensions exceeded fixed pixel credits");
    assert_eq!(kind, WorldAssetRequestKind::ReferenceImage);
    assert_eq!(url, "reference.jpg", "the url is captured BEFORE begin_close clears it");
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!("shared probe") };
    authority.finish(owner).unwrap();
    assert!(authority.terminal_is_empty());

    for source in [br#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>"#.as_slice(), b"GIF89a\x01\0\x01\0".as_slice()] {
        let (mut authority, mut probe, step) = drive(source);
        assert!(matches!(step, RendererAssetProbeStep::Ready), "React-accepted reference media reaches the target decoder rather than being narrowed at the shared probe");
        probe.begin_close();
        while !probe.close_step() {}
        let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!("shared probe") };
        authority.finish(owner).unwrap();
    }
}

#[test]
fn reference_decode_staging_refuses_a_fifth_live_token_without_evicting_the_first() {
    let mut io = WorldAssetIoAuthority::default();
    let tokens = (0..5).map(|index| io.reserve(1, index + 1, WorldAssetRequestKind::ReferenceImage, &format!("reference-{index}.png"), 4).expect("bounded reference token")).collect::<Vec<_>>();
    let mut staged = StagedReferenceImageAuthority::new();
    let pool = SceneRasterPool::new();
    for (index, token) in tokens.iter().copied().take(4).enumerate() {
        let descriptor = SceneRasterDescriptor { width: 1, height: 1, source_digest: [index as u64 + 1, index as u64 + 101], source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
        assert_eq!(staged.begin(&pool, token, format!("reference-{index}.png"), descriptor, index as u64 + 1).expect("begin"), StagedReferenceImageAuthority::WRITER);
        assert!(staged.push(&pool, token, 0, &[index as u8, 0, 0, 255]).expect("exact row"));
        assert!(staged.seal(&pool, token).expect("seal"));
    }
    let fifth = SceneRasterDescriptor { width: 1, height: 1, source_digest: [5, 105], source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    assert_eq!(staged.begin(&pool, tokens[4], "reference-4.png".into(), fifth, 5).expect("bounded staging refusal"), StagedReferenceImageAuthority::BUSY, "the fifth live token waits instead of evicting");
    let first = staged.take(tokens[0], "reference-0.png").expect("first token remains staged");
    first.with_rows(0, 4, |bytes, rows| assert_eq!((bytes, rows), (&[0, 0, 0, 255][..], 1))).expect("immutable first row");
    assert!(first.release());
    for token in tokens.iter().copied().skip(1).take(3) {
        staged.discard(&pool, token);
    }
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty(), "the token fixture returns every request owner after proving staged residency");
}

#[test]
fn reference_decode_staging_grants_one_writer_and_reuses_only_after_seal() {
    let mut io = WorldAssetIoAuthority::default();
    let token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "shared.png", 4).expect("source token");
    let descriptor = SceneRasterDescriptor { width: 1, height: 1, source_digest: [7, 11], source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    let pool = SceneRasterPool::new();
    let mut staged = StagedReferenceImageAuthority::new();

    assert_eq!(staged.begin(&pool, token, "shared.png".into(), descriptor, 1).expect("first begin"), StagedReferenceImageAuthority::WRITER);
    assert_eq!(staged.begin(&pool, token, "shared.png".into(), descriptor, 1).expect("reentrant begin"), StagedReferenceImageAuthority::BUSY);
    assert!(staged.push(&pool, token, 0, &[7, 11, 13, 255]).expect("only writer pixels"));
    assert!(staged.seal(&pool, token).expect("exact seal"));
    assert_eq!(staged.begin(&pool, token, "shared.png".into(), descriptor, 1).expect("sealed begin"), StagedReferenceImageAuthority::REUSED);

    let lease = staged.take(token, "shared.png").expect("sealed lease remains available");
    lease.with_rows(0, 4, |bytes, rows| assert_eq!((bytes, rows), (&[7, 11, 13, 255][..], 1))).expect("immutable source");
    assert!(lease.release());
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty());
}

#[test]
fn reference_decode_staging_reports_exact_pool_reuse_before_browser_decode() {
    let mut io = WorldAssetIoAuthority::default();
    let first_token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "shared.png", 4).expect("first source token");
    let second_token = io.reserve(2, 1, WorldAssetRequestKind::ReferenceImage, "shared.png", 4).expect("second source token");
    let descriptor = SceneRasterDescriptor { width: 1, height: 1, source_digest: [7, 11], source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    let pool = SceneRasterPool::new();
    let mut staged = StagedReferenceImageAuthority::new();
    assert_eq!(staged.begin(&pool, first_token, "shared.png".into(), descriptor, 1).expect("first begin"), StagedReferenceImageAuthority::WRITER);
    assert!(staged.push(&pool, first_token, 0, &[7, 11, 13, 255]).expect("first pixels"));
    assert!(staged.seal(&pool, first_token).expect("first seal"));
    let first = staged.take(first_token, "shared.png").expect("first ready lease");
    assert!(first.release());

    assert_eq!(staged.begin(&pool, second_token, "shared.png".into(), descriptor, 2).expect("second begin"), StagedReferenceImageAuthority::REUSED);
    assert!(staged.seal(&pool, second_token).expect("reused seal is terminal without another pixel stream"));
    let second = staged.take(second_token, "shared.png").expect("reused ready lease");
    second.with_rows(0, 4, |bytes, rows| assert_eq!((bytes, rows), (&[7, 11, 13, 255][..], 1))).expect("reused immutable source");
    assert!(second.release());

    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
fn native_reference_runtime() -> AppMailbox {
    RuntimeMailbox::new(AppRuntime {
        atlas: FontAtlas::builtin(),
        icons: IconAtlas::default(),
        icon_rebuild: None,
        icon_raster_scale: 1.0,
        interaction: None,
        checkout: Default::default(),
        draw: DrawList::default(),
        overlay: DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: FrameActionOwners::default(),
        pending_frame_maintenance_refusal: None,
        plugin_modules_root: Default::default(),
        native_plugin_mtimes: Default::default(),
        native_hot_swap_scan: None,
        native_hot_swap_modified: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    })
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_reference_decode_cancellation_keeps_encoded_and_decoded_owners_until_bounded_close() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    let retirement = &law["referenceDecodeRetirement"];
    let mut io = WorldAssetIoAuthority::default();
    let token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "reference.png", 4).expect("reference token");
    let input_bytes = retirement["phaseZero"]["encodedBytes"].as_u64().unwrap() as usize;
    let pixel_bytes = retirement["phaseZero"]["decodedPixelBytes"].as_u64().unwrap() as usize;
    let job = NativeReferenceDecodeJob::new(token, "reference.png".into(), vec![7; input_bytes]);
    job.rearm(DecodedReferenceImage { width: u32::try_from(pixel_bytes / 4).unwrap(), height: 1, source_digest: [71, 73], pixels: vec![11; pixel_bytes] });
    job.cancel();
    let retained_input = job.input.lock().expect("decode input").as_ref().map(Vec::len);
    let retained_pixels = job.decoded.lock().expect("decoded reference image").as_ref().map(|decoded| decoded.pixels.len());
    let cancelled = job.cancelled.load(Ordering::Acquire);
    assert_eq!(job.close_step(1, 0), NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes: retirement["zeroGrantReleasedBytes"].as_u64().unwrap() as usize });
    let mut released = Vec::new();
    let mut terminal = false;
    for _ in 0..32 {
        match job.close_step(retirement["maximumItems"].as_u64().unwrap() as usize, law["pageBytes"].as_u64().unwrap() as usize) {
            NativeReferenceDecodeCloseStep::Pending { released_bytes, .. } if released_bytes > 0 => released.push(released_bytes),
            NativeReferenceDecodeCloseStep::Pending { .. } | NativeReferenceDecodeCloseStep::Busy => {}
            NativeReferenceDecodeCloseStep::Complete => {
                terminal = true;
                break;
            }
        }
    }
    assert!(LIBRARY_SOURCE().contains("renderer_worker_pool().try_submit(semio_framework_async::Lane::Maintenance, job)"), "native reference decode submits only to the bounded maintenance lane");
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty(), "the token fixture returns its request owner after proving job cancellation");
    assert!(cancelled, "cancellation must fence worker publication before retirement starts");
    assert_eq!(retained_input, Some(input_bytes), "cancellation cannot destroy the encoded owner before an explicit bounded close grant");
    assert_eq!(retained_pixels, Some(pixel_bytes), "cancellation cannot destroy the decoded pixel owner before an explicit bounded close grant");
    let expected = retirement["phaseZero"]["decodedPageReleases"].as_array().unwrap().iter().chain(retirement["phaseZero"]["encodedPageReleases"].as_array().unwrap()).map(|value| value.as_u64().unwrap() as usize).collect::<Vec<_>>();
    assert_eq!(released, expected, "every close turn releases at most the neutral one-page grant, decoded before encoded");
    assert_eq!(released.iter().sum::<usize>(), retirement["phaseZero"]["releasedBytes"].as_u64().unwrap() as usize);
    assert!(terminal);
    assert!(job.input.lock().expect("terminal decode input").is_none());
    assert!(job.decoded.lock().expect("terminal decoded reference image").is_none());
    assert!(job.output.lock().expect("terminal decode output").is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_reference_decode_phase_two_waiting_output_stays_in_the_mailbox_after_one_close_turn() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    let retirement = &law["referenceDecodeRetirement"]["phaseTwoWaiting"];
    let mut io = WorldAssetIoAuthority::default();
    let token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "waiting-reference.png", 4).expect("reference token");
    let job = Arc::new(NativeReferenceDecodeJob::new(token, "waiting-reference.png".into(), vec![13; retirement["encodedBytes"].as_u64().unwrap() as usize]));
    let pixel_bytes = retirement["decodedPixelBytes"].as_u64().unwrap() as usize;
    *job.output.lock().expect("decode output") = Some(NativeReferenceDecodeOutput::Waiting(DecodedReferenceImage { width: u32::try_from(pixel_bytes / 4).unwrap(), height: 1, source_digest: [79, 83], pixels: vec![17; pixel_bytes] }));
    job.phase.store(2, Ordering::Release);
    let runtime = native_reference_runtime();
    *runtime.0.native_reference_decode.lock().expect("native reference decode slot") = Some(job);
    let _ = runtime.close_renderer_asset_step();
    let retained_after_one_turn = runtime.0.native_reference_decode.lock().expect("native reference decode slot after one close turn").is_some();
    let mut close_turns = 1;
    while runtime.0.native_reference_decode.lock().expect("native reference decode terminal probe").is_some() && close_turns < 32 {
        let _ = runtime.close_renderer_asset_step();
        close_turns += 1;
    }
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty(), "the token fixture returns its request owner after observing mailbox retention");
    assert_eq!(retained_after_one_turn, retirement["retainedAfterOneTurn"].as_bool().unwrap(), "one close turn cannot clear a phase-two waiting decode whose pixels exceed one page");
    assert_eq!(close_turns, retirement["closeTurns"].as_u64().unwrap() as usize, "the mailbox retains the exact owner through output handback, decoded pages, metadata, encoded pages, metadata, and terminal acknowledgement");
    assert!(runtime.0.native_reference_decode.lock().expect("native reference decode terminal").is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_reference_decode_running_worker_returns_its_encoded_and_decoded_owners_before_bounded_close() {
    use std::time::{Duration, Instant};

    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    let retirement = &law["referenceDecodeRetirement"]["phaseZero"];
    let input_bytes = retirement["encodedBytes"].as_u64().unwrap() as usize;
    let pixel_bytes = retirement["decodedPixelBytes"].as_u64().unwrap() as usize;
    let mut io = WorldAssetIoAuthority::default();
    let token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "worker-reference.png", 4).expect("reference token");
    let job = Arc::new(NativeReferenceDecodeJob::new(token, "worker-reference.png".into(), vec![19; input_bytes]));
    job.rearm(DecodedReferenceImage { width: u32::try_from(pixel_bytes / 4).unwrap(), height: 1, source_digest: [89, 97], pixels: vec![23; pixel_bytes] });
    let entered = Arc::new(std::sync::Barrier::new(2));
    let release = Arc::new(std::sync::Barrier::new(2));
    *job.worker_barriers.lock().expect("native reference worker barrier") = Some((entered.clone(), release.clone()));
    assert!(job.try_schedule(native_reference_runtime()), "the real maintenance worker accepts the retained decode owner");
    entered.wait();
    job.cancel();
    assert_eq!(job.close_step(1, law["pageBytes"].as_u64().unwrap() as usize), NativeReferenceDecodeCloseStep::Busy, "a checked-out worker owner cannot be stolen by close");
    release.wait();
    let deadline = Instant::now() + Duration::from_secs(10);
    while job.phase.load(Ordering::Acquire) == 1 && Instant::now() < deadline {
        std::thread::yield_now();
    }
    let retained_input = job.input.lock().expect("worker-returned input").as_ref().map(Vec::len);
    let retained_pixels = job.decoded.lock().expect("worker-returned decoded image").as_ref().map(|decoded| decoded.pixels.len());
    for _ in 0..32 {
        if matches!(job.close_step(1, law["pageBytes"].as_u64().unwrap() as usize), NativeReferenceDecodeCloseStep::Complete) {
            break;
        }
    }
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty());
    assert_eq!(retained_input, Some(input_bytes), "the worker must hand the encoded owner back before bounded close");
    assert_eq!(retained_pixels, Some(pixel_bytes), "the worker must hand the decoded pixel owner back before bounded close");
}

#[test]
fn retained_asset_structure_scanners_resume_at_arbitrary_block_boundaries() {
    let png = [b"\x89PNG\r\n\x1a\n".as_slice(), &[0, 0, 0, 13], b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0], &[0; 4], &[0, 0, 0, 1], b"IDAT", &[0], &[0; 4], &[0, 0, 0, 0], b"IEND", &[0; 4]].concat();
    let mut png_cursor = PngStructureCursor::new(png.len());
    for bytes in png.chunks(3) {
        png_cursor.feed(bytes).expect("bounded PNG block");
    }
    png_cursor.finish().expect("PNG terminal");

    let jpeg = [0xff, 0xd8, 0xff, 0xc0, 0, 11, 8, 0, 1, 0, 1, 1, 1, 0x11, 0, 0xff, 0xd9];
    let mut jpeg_cursor = JpegStructureCursor::new(jpeg.len());
    for bytes in jpeg.chunks(2) {
        jpeg_cursor.feed(bytes).expect("bounded JPEG block");
    }
    jpeg_cursor.finish().expect("JPEG terminal");

    let protobuf = [0x1a, 0x02, 0x08, 0x01];
    let mut protobuf_cursor = ProtobufStructureCursor::new(protobuf.len());
    for byte in protobuf {
        protobuf_cursor.feed(&[byte]).expect("bounded protobuf byte");
    }
    protobuf_cursor.finish().expect("protobuf terminal");
}

#[test]
fn retained_asset_structure_scanners_reject_pixel_and_varint_capacity_plus_one() {
    let png = [b"\x89PNG\r\n\x1a\n".as_slice(), &[0, 0, 0, 13], b"IHDR", &[0, 0, 0x10, 1, 0, 0, 0x10, 0, 8, 6, 0, 0, 0]].concat();
    let mut png_cursor = PngStructureCursor::new(png.len() + 4);
    assert!(png_cursor.feed(&png).is_err());

    let mut protobuf_cursor = ProtobufStructureCursor::new(11);
    assert!(protobuf_cursor.feed(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x02]).is_err());
}

#[test]
fn runtime_mailbox_reserves_completion_capacity_and_coalesces_only_matching_keys() {
    let completion = |key: Option<&'static str>, revision: u64| RuntimeCompletion { key, revision, requires_interaction: false, restores_interaction: false, apply: RuntimeApply::Resize { width: 1.0, height: 1.0, dpr: 1.0 } };
    let mut queue = RuntimeCompletionQueue::new();
    for revision in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
        assert!(queue.enqueue(completion(None, revision as u64)));
    }
    assert!(!queue.enqueue(completion(None, 10_000)));
    assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY - 1);

    let mut queue = RuntimeCompletionQueue::new();
    for revision in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
        assert!(queue.enqueue(completion(Some("refresh"), revision as u64)));
    }
    assert!(queue.enqueue(completion(Some("refresh"), 10_000)));
    assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY - 1);
    assert_eq!(queue.ready.back().expect("latest refresh").revision, 10_000);

    let mut queue = RuntimeCompletionQueue::new();
    for _ in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
        assert!(queue.reserve(None));
    }
    assert!(!queue.reserve(None));
    assert!(queue.reserve_interaction());
    assert!(!queue.reserve_interaction());
    queue.finish(completion(None, 20_000));
    assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY);
}
