use super::*;

fn close_packet_step(packet: &mut PreparedRenderPacket) -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = semio_framework_job::RetainedCloneGrant { maximum_copy_bytes: 16 * 1024, ..packet.next_close_demands(16 * 1024).unwrap() };
    match packet.close_step(grant) {
        Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(packet.retirement_is_empty()); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
        Close::Refused { kind, progress } => panic!("original packet refused admitted close: {kind:?}, progress: {progress:?}"),
    }
}
fn close_abandoned_packet_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderPacket::next_abandoned_close_demands(16 * 1024).unwrap();
    match PreparedRenderPacket::close_abandoned_step(grant) {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
        Close::Refused { kind, progress } => panic!("original abandoned packet refused admitted close: {kind:?}, progress: {progress:?}"),
    }
}
fn close_abandoned_mailbox_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderReceiver::next_abandoned_close_demands(16 * 1024).unwrap();
    match PreparedRenderReceiver::close_abandoned_step(grant) {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
        Close::Refused { kind, progress } => panic!("original abandoned mailbox refused admitted close: {kind:?}, progress: {progress:?}"),
    }
}


fn close_job_step(job: &mut PreparedRenderJob) -> bool {
    use semio_framework_job::{InteractiveJobCloseStep as Close, RetainedCloneGrant};
    InteractiveJob::begin_close(job);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: InteractiveJob::next_close_capacity_byte_demand(job, 1).unwrap(), maximum_release_bytes: InteractiveJob::next_close_release_byte_demand(job).unwrap(), maximum_depth: InteractiveJob::next_close_depth_demand(job).unwrap() };
    match InteractiveJob::close_step(job, grant) {
        Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(job.terminal_is_empty()); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
        Close::Refused { kind, progress } => panic!("original prepared owner refused admitted close: {kind:?}, progress: {progress:?}"),
    }
}
fn close_abandoned_job_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderJob::next_abandoned_close_demands(1).unwrap();
    match PreparedRenderJob::close_abandoned_step(grant) {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
        Close::Refused { kind, progress } => panic!("original abandoned owner refused admitted close: {kind:?}, progress: {progress:?}"),
    }
}

fn close_abandoned_input_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderInput::next_abandoned_close_demands(16 * 1024).unwrap();
    match PreparedRenderInput::close_abandoned_step(grant) {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
        Close::Refused { kind, progress } => panic!("original abandoned input refused admitted close: {kind:?}, progress: {progress:?}"),
    }
}

fn close_abandoned_atlas_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedAtlasPages::next_abandoned_close_demands(16 * 1024).unwrap();
    match PreparedAtlasPages::close_abandoned_step(grant) {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
        Close::Refused { kind, progress } => panic!("original abandoned atlas refused admitted close: {kind:?}, progress: {progress:?}"),
    }
}


#[cfg(all(feature = "wgpu-engine", not(target_arch = "wasm32")))]
#[path = "../♻️physical-job-close/🎟️prepared/🦀️.rs"]
mod physical_job_close_laws;
use semio_framework_job::{drive_step, root_cancel_token, Generation, InteractiveStage, OperationId, StepBudget};

#[test]
fn prepared_animation_receipt_measures_actual_normal_and_overlay_primitives() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🖌️render/⏱️schedule/🧫️fixtures/🎞️animation/🔣️.json")).unwrap();
    for overlay_route in [false, true] {
        for row in fixture["primitives"].as_array().unwrap() {
            let mut draw = DrawList::default();
            let mut overlay = DrawList::default();
            for (list, field) in [(&mut draw, "draw"), (&mut overlay, "overlay")] {
                for kind in row[field].as_array().unwrap() {
                    let mut instance = crate::wgpu::draw_types::UiInstance::solid([0.0, 0.0, 24.0, 24.0], crate::wgpu::theme::Rgba::new(1.0, 1.0, 1.0, 1.0));
                    instance.params[2] = kind.as_f64().unwrap() as f32;
                    if overlay_route {
                        list.layers[0].overlay_ui_instances.push(instance);
                    } else {
                        list.layers[0].ui_instances.push(instance);
                    }
                }
            }
            let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, Some(overlay), 0.0), 1);
            assert!(matches!(drive_preparation_until_terminal(&mut job), StepOutcome::Complete(_)));
            let mut packet = job.take_packet().expect("measured packet");
            assert_eq!(packet.has_animated_primitives(), row["active"].as_bool().unwrap(), "{}", row["id"]);
            while !close_packet_step(&mut packet) {}
            while !close_job_step(&mut job) {}
        }
    }
}

static PREPARED_PROCESS_TEST_LOCK: Mutex<()> = Mutex::new(());

/// 🔒️ EVERY law in this case takes this guard, first statement, no exceptions.
///
/// The prepared ladder's credits are PROCESS-wide by design — `PREPARED_RENDER_PROCESS_PERMITS`, the
/// atlas page pool and the four abandonment rings `drain_abandoned_preparations` drains are one
/// budget for the whole binary, because that is the budget a real process has. A law that reads a
/// process counter (`PREPARED_RENDER_PROCESS_PERMITS.load(..) == 0`) or counts retirement grants
/// therefore measures every other law that is running at the same time, and under `cargo test` with
/// more than one test thread it flakes: `packet_drop_retires_nested_backings_and_permit_scalars_separately`
/// and `pending_presenter_witness_rejects_superseding_packet_with_exact_owner` were the two that
/// showed it (`📓️w9a`), but only eleven of this case's thirty-nine laws took the lock, so ANY of the
/// other twenty-eight could have been the perturber. Serialising the whole case is what makes the
/// measurements mean what they say, and it costs nothing: none of these laws blocks.
///
/// Holding the lock is only half of it. An owner a law drops lands in an ABANDONMENT ring and keeps
/// its permits until something grants it a close step, so a law that ends with owners still queued
/// hands the next one an already-spent budget (`prepared render process permits exhausted (held items
/// 4/64 …)`). The guard therefore drains both rings before it hands the lock back: every law starts
/// from a quiescent process, whatever the law before it left behind and whatever order the runner
/// picked.
fn prepared_process_guard() -> std::sync::MutexGuard<'static, ()> {
    let guard = match PREPARED_PROCESS_TEST_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    drain_abandoned_preparations();
    drain_abandoned_atlases();
    guard
}

fn drain_abandoned_atlases() {
    while !close_abandoned_atlas_step() {}
}

fn drain_abandoned_preparations() {
    loop {
        let inputs = close_abandoned_input_step();
        let jobs = close_abandoned_job_step();
        let mailboxes = close_abandoned_mailbox_step();
        let packets = close_abandoned_packet_step();
        if inputs && jobs && mailboxes && packets {
            break;
        }
    }
}

fn now_ms() -> Option<u64> {
    Some(1)
}

fn drive_preparation_until_terminal(job: &mut PreparedRenderJob) -> StepOutcome {
    let mut preview = 0;
    for _ in 0..4_096 {
        let (outcome,original_retained_receipt) = drive_step(job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);
        assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
        if !matches!(outcome, StepOutcome::Yield) {
            return outcome;
        }
    }
    panic!("prepared render job did not reach a terminal outcome within fixed test credits");
}

fn packet(revision: u64, generation: u64) -> PreparedRenderPacket {
    let mut directives = PreparedRenderDirectives::default();
    assert!(directives.try_push(RenderDirective::PreservePreviousOnFailure).is_ok());
    let packet = PreparedRenderPacket {
        has_animated_primitives: false,
        scene_revision: revision,
        preview_generation: generation,
        damage: PreparedRenderScissors::default(),
        clips: PreparedRenderScissors::default(),
        directives,
        uploads: PreparedRenderUploads::default(),
        evictions: PreparedRenderEvictions::default(),
        draw: DrawList::default(),
        overlay: None,
        commands: PreparedRenderCommandPages::default(),
        time_seconds: 0.0,
        usage: PreparedRenderUsage::default(),
        limits: PreparedRenderLimits::default(),
        permit: PreparedRenderProcessPermit::try_reserve(0, 0),
        retirement_phase: 0,
        abandonment_slot: u8::MAX,
    };
    match packet.try_arm_abandonment() {
        Ok(packet) => packet,
        Err(_) => panic!("test packet abandonment slot"),
    }
}

fn retire_raster_upload(mut upload: PreparedRenderUpload) {
    let PreparedRenderUpload::RasterPages { key, pixels } = &mut upload else { panic!("paged raster upload") };
    while !pixels.retire_with_key_step(key) {}
    assert!(pixels.terminal_is_empty());
}

#[test]
fn paged_raster_producer_advances_one_page_and_moves_page_identity() {
    let _guard = prepared_process_guard();
    let source = vec![7; PREPARED_RASTER_PAGE_BYTES * 2];
    let source_pointer = source.as_ptr();
    let (mut producer, published_key) = PreparedRasterProducer::try_admit("two-pages".into(), source, 4_096, 2).expect("exact two-page admission");
    assert_eq!(published_key, "two-pages");
    assert!(producer.bind_frame_generation(9));
    assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending));
    assert_eq!(producer.pages.as_ref().expect("retained pages").slots.len(), 1);
    assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending));
    assert_eq!(producer.pages.as_ref().expect("retained pages").slots.len(), 2);
    assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending), "source backing retires on its own grant");
    let first = producer.pages.as_ref().and_then(|pages| pages.page_pointer(0)).expect("first page identity");
    let upload = match producer.step(9) {
        PreparedRasterProducerStep::Complete(upload) => upload,
        _ => panic!("completed page handoff"),
    };
    assert_eq!(first, source_pointer, "page view borrows the exact decoder backing");
    assert!(matches!(&upload, PreparedRenderUpload::RasterPages { pixels, .. } if pixels.page_pointer(0) == Some(first) && pixels.frame_generation() == 9));
    retire_raster_upload(upload);
}

#[test]
fn paged_raster_content_identity_is_stable_and_changes_with_one_pixel_byte() {
    fn complete(mut producer: PreparedRasterProducer, generation: u64) -> PreparedRenderUpload {
        assert!(producer.bind_frame_generation(generation));
        for _ in 0..8 {
            if let PreparedRasterProducerStep::Complete(upload) = producer.step(generation) {
                return upload;
            }
        }
        panic!("bounded two-page producer did not complete");
    }

    let _guard = prepared_process_guard();
    let first = vec![7; PREPARED_RASTER_PAGE_BYTES * 2];
    let same = first.clone();
    let mut changed = first.clone();
    *changed.last_mut().expect("changed byte") = 8;
    let (first, _) = PreparedRasterProducer::try_admit("first".into(), first, 4_096, 2).expect("first producer");
    let (same, _) = PreparedRasterProducer::try_admit("same".into(), same, 4_096, 2).expect("same producer");
    let (changed, _) = PreparedRasterProducer::try_admit("changed".into(), changed, 4_096, 2).expect("changed producer");
    let first = complete(first, 31);
    let same = complete(same, 32);
    let changed = complete(changed, 33);
    let PreparedRenderUpload::RasterPages { pixels: first_pixels, .. } = &first else { panic!("first pages") };
    let PreparedRenderUpload::RasterPages { pixels: same_pixels, .. } = &same else { panic!("same pages") };
    let PreparedRenderUpload::RasterPages { pixels: changed_pixels, .. } = &changed else { panic!("changed pages") };
    assert_eq!(first_pixels.content_identity(), same_pixels.content_identity());
    assert_ne!(first_pixels.content_identity(), changed_pixels.content_identity());
    retire_raster_upload(first);
    retire_raster_upload(same);
    retire_raster_upload(changed);
}

#[test]
fn stale_generation_does_not_consume_a_prepared_raster_page() {
    let _guard = prepared_process_guard();
    let (mut producer, _) = PreparedRasterProducer::try_admit("stale".into(), vec![3; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page admission");
    assert!(producer.bind_frame_generation(11));
    let retained = producer.source.len();
    assert!(matches!(producer.step(12), PreparedRasterProducerStep::Fault(_)));
    assert_eq!(producer.source.len(), retained);
    assert!(producer.pages.as_ref().expect("retained pages").slots.is_empty());
    producer.begin_close();
    while !producer.close_step() {}
    assert!(producer.terminal_is_empty());
}

#[test]
fn raster_cap_plus_one_rejects_the_exact_source_before_page_allocation() {
    let _guard = prepared_process_guard();
    let source = vec![5; PREPARED_RASTER_PAGE_BYTES + 4];
    let pointer = source.as_ptr();
    let mut rejected = PreparedRasterProducer::try_admit("wide".into(), source, 4_097, 1).expect_err("row cap plus one");
    assert_eq!(rejected.source.as_ptr(), pointer);
    assert!(rejected.fault().contains("fixed item or byte credits"));
    assert!(!rejected.close_step(), "one rejection grant retires one source page only");
    while !rejected.close_step() {}
}

#[test]
fn atlas_page_cap_plus_one_faults_before_process_credit_transfer() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let height = 33_554_433_u32;
    let byte_len = 33_554_433_usize;
    assert!(matches!(PreparedAtlasPages::try_new(1, height, 1, byte_len), Err("atlas page or byte credits exceeded")));
    let mut admitted = match PreparedAtlasPages::try_new(4, 2, 4, 32) {
        Ok(admitted) => admitted,
        Err(fault) => panic!("fixed atlas admission faulted: {fault}"),
    };
    let mut turns = 0;
    while !admitted.close_step() {
        turns += 1;
    }
    assert!(turns >= 6, "fixed backing and four permit scalars close independently");
    assert!(admitted.terminal_is_empty());
}

#[test]
fn atlas_close_releases_one_fixed_page_then_its_exact_credit() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let source = [9_u8; 32];
    let mut pages = match PreparedAtlasPages::try_new(4, 2, 4, source.len()) {
        Ok(pages) => pages,
        Err(fault) => panic!("fixed atlas admission faulted: {fault}"),
    };
    assert!(matches!(pages.push_page(&source, 0), Ok(true)));
    assert_eq!(pages.page(0), Some((&source[..], 0, 2)));
    assert!(!pages.close_step());
    assert_eq!(pages.len(), 0);
    assert!(!pages.terminal_is_empty());
    let mut turns = 0;
    while !pages.close_step() {
        turns += 1;
    }
    assert!(turns >= 6, "slot backing and permit fields each consume a distinct grant");
    assert!(pages.terminal_is_empty());
}

#[test]
fn atlas_process_item_max_plus_one_is_nonblocking_and_recovers_every_permit() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let mut owners: [Option<PreparedAtlasPages>; PREPARED_ATLAS_PROCESS_ITEMS] = std::array::from_fn(|_| None);
    for owner in &mut owners {
        *owner = Some(match PreparedAtlasPages::try_new(1, 1, 1, 1) {
            Ok(owner) => owner,
            Err(fault) => panic!("one exact process item: {fault}"),
        });
    }
    assert!(matches!(PreparedAtlasPages::try_new(1, 1, 1, 1), Err("atlas process permit credits exhausted")));
    for owner in owners.iter_mut().filter_map(Option::as_mut) {
        while !owner.close_step() {}
        assert!(owner.terminal_is_empty());
    }
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn abandoned_atlas_schedules_the_same_incremental_close_authority() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let mut owner = match PreparedAtlasPages::try_new(4, 2, 4, 32) {
        Ok(owner) => owner,
        Err(fault) => panic!("abandonment owner: {fault}"),
    };
    assert!(owner.push_page(&[7; 32], 0).is_ok());
    drop(owner);
    let mut turns = 0;
    while !close_abandoned_atlas_step() {
        turns += 1;
        assert!(turns < 16, "one page, backing owner, and four permit fields must converge");
    }
    assert!(turns >= 7);
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn interrupted_atlas_close_rejoins_the_same_abandonment_authority() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let source = vec![3_u8; PREPARED_ATLAS_PAGE_BYTES + 1];
    let mut owner = match PreparedAtlasPages::try_new(1, u32::try_from(source.len()).unwrap_or(u32::MAX), 1, source.len()) {
        Ok(owner) => owner,
        Err(fault) => panic!("two-page abandonment owner: {fault}"),
    };
    assert!(matches!(owner.push_page(&source, 0), Ok(false)));
    assert!(matches!(owner.push_page(&source, owner.next_row()), Ok(true)));
    assert!(!owner.close_step());
    assert_eq!(owner.len(), 1);
    drop(owner);
    let mut turns = 0;
    while !close_abandoned_atlas_step() {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(turns >= 7);
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn atlas_allocation_refusal_preserves_the_packed_permit_ledger() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let before = PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire);
    assert!(matches!(PreparedAtlasPages::try_new(0, 1, 4, 4), Err("atlas dimensions do not fit fixed page credits")));
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), before);
}

#[test]
fn atlas_contended_permit_attempts_are_nonblocking_and_poison_free() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let handles = std::array::from_fn::<_, 8, _>(|_| {
        std::thread::spawn(|| match PreparedAtlasPages::try_new(1, 1, 1, 1) {
            Ok(mut owner) => {
                while !owner.close_step() {}
                owner.terminal_is_empty()
            }
            Err("atlas process permit credits exhausted") => true,
            Err(_) => false,
        })
    });
    for handle in handles {
        assert!(match handle.join() {
            Ok(closed_or_refused) => closed_or_refused,
            Err(_) => false,
        });
    }
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn raster_item_bytes_exact_and_plus_one_are_claimed_before_materialization() {
    let _guard = prepared_process_guard();
    let reservation = PreparedRasterReservation::try_reserve("item-exact".into()).expect("initial exact reservation");
    let reservation = reservation.claim(4_096, 1_024).expect("sixteen MiB operation claim");
    let mut rejected = reservation.reject("test retirement", Vec::new());
    while !rejected.close_step() {}
    assert!(rejected.terminal_is_empty());

    let reservation = PreparedRasterReservation::try_reserve("item-plus-one".into()).expect("initial plus-one reservation");
    let mut rejected = reservation.claim(4_096, 1_025).expect_err("sixteen MiB plus one row");
    assert_eq!(rejected.fault(), "raster producer exceeded fixed item or byte credits");
    while !rejected.close_step() {}
    assert!(rejected.terminal_is_empty());
}

#[test]
fn raster_simultaneous_source_decode_peak_exact_and_plus_one() {
    let _guard = prepared_process_guard();
    let height = 1_023usize;
    let decoded_bytes = PREPARED_RASTER_PAGE_BYTES * height;
    let page_slot_bytes = size_of::<PreparedRasterPage>() * height;
    let source_peak_bytes = PREPARED_RASTER_PRODUCER_BYTES - decoded_bytes - page_slot_bytes;
    assert_eq!(source_peak_bytes % 2, 0, "exact source workspace boundary");
    let source_bytes = source_peak_bytes / 2;

    let reservation = PreparedRasterReservation::try_reserve_source(String::new(), source_bytes).expect("exact simultaneous source reservation");
    let reservation = reservation.claim(4_096, height as u32).expect("source plus retained parse plus decoded backing and page slots exactly fit");
    assert_eq!(reservation.credit.as_ref().unwrap().bytes, PREPARED_RASTER_PRODUCER_BYTES);
    let mut rejected = reservation.reject("exact peak retirement", Vec::new());
    while !rejected.close_step() {}

    let reservation = PreparedRasterReservation::try_reserve_source(String::new(), source_bytes + 1).expect("plus one source is initially retained");
    let mut rejected = reservation.claim(4_096, height as u32).expect_err("simultaneous source and decoded peak plus one must fail before decode");
    assert_eq!(rejected.fault(), "raster producer exact credit resize failed");
    while !rejected.close_step() {}
}

#[test]
fn retained_codec_source_moves_once_and_retires_one_page_per_governed_step() {
    let _guard = prepared_process_guard();
    let decoded = vec![7; PREPARED_RASTER_PAGE_BYTES];
    let retained_source = vec![9; PREPARED_RASTER_PAGE_BYTES * 2];
    let retained_pointer = retained_source.as_ptr();
    let reservation = PreparedRasterReservation::try_reserve_source("retained-source".into(), retained_source.capacity()).expect("source workspace admitted before decode");
    let reservation = reservation.claim(4_096, 1).expect("decoded owner credited in addition to retained source");
    let (mut producer, _) = reservation.finalize(decoded, retained_source, 4_096, 1).expect("exact retained source owner");
    assert_eq!(producer.retained_source.as_ptr(), retained_pointer);
    assert!(producer.bind_frame_generation(3));

    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(producer).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    for expected in [PREPARED_RASTER_PAGE_BYTES * 2, PREPARED_RASTER_PAGE_BYTES * 2, PREPARED_RASTER_PAGE_BYTES, 0] {
        let (outcome,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(11), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);
        assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
        assert!(matches!(outcome, StepOutcome::Yield));
        assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().retained_source.len(), expected);
    }
    assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().retained_source.as_ptr(), retained_pointer);
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

/// 🌑️ LAW: the bounded caster prepass completes before every receiver, then the color pass
/// keeps React's textured underlay → opaque → lines → translucent order.
#[test]
fn an_enabled_shadow_pass_measures_every_caster_before_its_receivers() {
    let _guard = prepared_process_guard();
    use crate::wgpu::kernel_3d_scene::{Instance3d, LineDraw3d, LineVertex3d, SceneCurvilinear3d, SceneDraw3d, SceneMaterialDraw3d, SceneMaterialKind3d, ScenePass3d, SceneShadowRole3d, TexturedDraw3d, TexturedInstance3d};
    let mut draw = DrawList::default();
    let instance = Instance3d { component_source: None, id: String::new(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    draw.push_scene_pass(ScenePass3d {
        viewport: [0.0, 0.0, 100.0, 40.0],
        shadow: crate::wgpu::kernel_3d_scene::SceneShadow3d { enabled: true, ..Default::default() },
        shadow_draws: vec![SceneDraw3d { mesh_key: "caster".into(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: SceneShadowRole3d { casts: true, receives: true } }],
        draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: Default::default() }],
        translucent_draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: Default::default() }],
        material_draws: vec![
            SceneMaterialDraw3d { mesh_key: "painted".into(), mesh_version: 3, first_index: 0, index_count: u32::MAX, instances: vec![instance.clone()], material: SceneMaterialKind3d::Painted { texture_key: "paint-map".into() }, translucent: false },
            SceneMaterialDraw3d {
                mesh_key: "celebrated".into(),
                mesh_version: 4,
                first_index: 0,
                index_count: u32::MAX,
                instances: vec![instance.clone()],
                material: SceneMaterialKind3d::Celebration { stops: [[1.0; 4]; 3], angle: 0.5 },
                translucent: true,
            },
        ],
        line_draws: vec![LineDraw3d { vertices: vec![LineVertex3d { position: [0.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] }] }],
        textured_draws: vec![TexturedDraw3d { instances: vec![TexturedInstance3d { texture_key: String::new(), model: instance.model, background: [0.0; 4], appearance: [0.85, 0.0, 0.0, 0.0] }] }],
        curvilinear: Some(SceneCurvilinear3d { fov_radians: 2.0, strength: 0.75 }),
        ..Default::default()
    });

    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut order = Vec::new();
    for _ in 0..64 {
        let label = match cursor {
            DrawMeasureCursor::PassShadowBegin(..) | DrawMeasureCursor::PassShadowInstance { .. } => "shadow",
            DrawMeasureCursor::PassTextured { .. } | DrawMeasureCursor::PassTexturedInstance { .. } | DrawMeasureCursor::PassTexturedKey { .. } => "textured",
            DrawMeasureCursor::PassDraw { translucent, .. } | DrawMeasureCursor::PassDrawKey { translucent, .. } | DrawMeasureCursor::PassInstance { translucent, .. } | DrawMeasureCursor::PassInstanceKey { translucent, .. } => {
                if translucent {
                    "translucent"
                } else {
                    "opaque"
                }
            }
            DrawMeasureCursor::PassMaterial { translucent, .. }
            | DrawMeasureCursor::PassMaterialMeshKey { translucent, .. }
            | DrawMeasureCursor::PassMaterialTextureKey { translucent, .. }
            | DrawMeasureCursor::PassMaterialInstance { translucent, .. }
            | DrawMeasureCursor::PassMaterialInstanceKey { translucent, .. } => {
                if translucent {
                    "material-translucent"
                } else {
                    "material-opaque"
                }
            }
            DrawMeasureCursor::PassLine { .. } | DrawMeasureCursor::PassLineVertex { .. } => "lines",
            DrawMeasureCursor::PassPostprocess { .. } => "world-postprocess",
            _ => "other",
        };
        if order.last() != Some(&label) && label != "other" {
            order.push(label);
        }
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() || matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    assert_eq!(order, vec!["shadow", "textured", "material-opaque", "opaque", "lines", "translucent", "material-translucent", "world-postprocess"], "the image-space remap runs after every world color receiver");
}

/// 🌐️ LAW: the prepared scalar ladder owns exactly one grid item after textured references
/// and before opaque geometry, while ordinary overlays retain the line-vertex lane.
#[test]
fn a_procedural_grid_is_one_prepared_scalar_between_textures_and_opaque_geometry() {
    let _guard = prepared_process_guard();
    use crate::wgpu::kernel_3d_scene::{Instance3d, LineDraw3d, LineVertex3d, ProceduralGrid3d, SceneDraw3d, ScenePass3d, TexturedDraw3d, TexturedInstance3d};
    let instance = Instance3d { component_source: None, id: String::new(), model: Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() };
    let grid = ProceduralGrid3d { plane_z: 0.001, camera_plane_projection: [2.0, -3.0, 0.001], cell_size: 1.0, fade_distance: 8.0, cell_color: [0.2, 0.3, 0.4] };
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d {
        procedural_grid: Some(grid),
        textured_draws: vec![TexturedDraw3d { instances: vec![TexturedInstance3d { texture_key: String::new(), model: instance.model, background: [0.0; 4], appearance: [1.0, 0.0, 0.0, 0.0] }] }],
        draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance], shadow_role: Default::default() }],
        line_draws: vec![LineDraw3d { vertices: vec![LineVertex3d { position: [0.0; 3], color: [1.0; 4] }, LineVertex3d { position: [1.0, 0.0, 0.0], color: [1.0; 4] }] }],
        ..Default::default()
    });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut measured = Vec::new();
    for _ in 0..64 {
        let current = cursor;
        let Some(usage) = PreparedRenderJob::next_draw_usage(&draw, &mut cursor) else { break };
        measured.push((current, usage));
        if matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    let grid_rows: Vec<_> = measured.iter().enumerate().filter(|(_, (cursor, _))| matches!(cursor, DrawMeasureCursor::PassGrid { pass: 0 })).collect();
    assert_eq!(grid_rows.len(), 1, "one retained grid becomes exactly one prepared scalar");
    assert_eq!(grid_rows[0].1 .1, PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<ProceduralGrid3d>(), ..Default::default() }, "the grid scalar owns its exact retained bytes in one cancellable step");
    let textured = measured.iter().position(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassTexturedInstance { .. })).expect("textured reference scalar");
    let grid_index = grid_rows[0].0;
    let opaque = measured.iter().position(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassInstance { translucent: false, .. })).expect("opaque instance scalar");
    let line_vertices = measured.iter().filter(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassLineVertex { .. })).count();
    assert!(textured < grid_index && grid_index < opaque, "textured references precede the grid and opaque geometry follows it");
    assert_eq!(line_vertices, 2, "ordinary overlays retain the line-vertex lane");
}

#[test]
fn ellipse_scene_pass_snapshots_backdrop_clears_inside_then_composites_last() {
    use crate::wgpu::kernel_3d_scene::{ScenePass3d, SceneViewportMask3d};
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d { viewport_mask: SceneViewportMask3d::Ellipse, clear_color: Some([0.1, 0.2, 0.3, 0.7]), ..Default::default() });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut scalars = Vec::new();
    for _ in 0..8 {
        let Some(_) = PreparedRenderJob::next_draw_usage(&draw, &mut cursor) else { break };
        scalars.push(cursor);
        if matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    assert_eq!(scalars[0], DrawMeasureCursor::PassBackdropSnapshot { pass: 0 });
    assert_eq!(scalars[1], DrawMeasureCursor::PassSceneClear { pass: 0 });
    assert!(scalars.contains(&DrawMeasureCursor::PassPostprocess { pass: 0 }));
    assert_eq!(scalars.iter().filter(|cursor| matches!(cursor, DrawMeasureCursor::PassPostprocess { .. })).count(), 1);

    let mut rectangular = DrawList::default();
    rectangular.push_scene_pass(ScenePass3d::default());
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut postprocess = 0;
    while PreparedRenderJob::next_draw_usage(&rectangular, &mut cursor).is_some() && !matches!(cursor, DrawMeasureCursor::Complete) {
        postprocess += usize::from(matches!(cursor, DrawMeasureCursor::PassPostprocess { .. } | DrawMeasureCursor::PassBackdropSnapshot { .. } | DrawMeasureCursor::PassSceneClear { .. }));
    }
    assert_eq!(postprocess, 0, "ordinary rectangular worlds add no capture, clear, or postprocess scalar");
}



#[test]
fn a_disabled_shadow_never_publishes_a_gpu_shadow_scalar() {
    let _guard = prepared_process_guard();
    use crate::wgpu::kernel_3d_scene::{Instance3d, SceneDraw3d, ScenePass3d};
    let instance = Instance3d { component_source: None, id: String::new(), model: Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() };
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d { draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance], shadow_role: Default::default() }], ..Default::default() });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    for _ in 0..32 {
        assert!(!matches!(cursor, DrawMeasureCursor::PassShadowBegin(..) | DrawMeasureCursor::PassShadowInstance { .. }));
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() || matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
}

/// 🎟️ A raster producer must be bound to the frame it is published into BEFORE it is pushed, and the
/// job's refusal must be readable — `StepOutcome::Fault` carries an empty payload here, so
/// [`PreparedRenderJob::fault`] is the only way a driver can name it
/// (`📓️w7b-presenter-one-frame-per-boot.md` §1).
#[test]
fn an_unbound_raster_producer_refuses_its_prepared_job_with_a_readable_fault() {
    let _guard = prepared_process_guard();
    let (unbound, _) = PreparedRasterProducer::try_admit("unbound".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(unbound).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    let (outcome,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(4, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert_eq!(job.fault(), Some("raster producer generation is stale"));
    while !close_job_step(&mut job) {}

    let (mut bound, _) = PreparedRasterProducer::try_admit("bound".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    assert!(bound.bind_frame_generation(3));
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(bound).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let (outcome,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(4, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(outcome, StepOutcome::Yield));
    assert_eq!(job.fault(), None);
    while !close_job_step(&mut job) {}
}

#[test]
fn raster_ledger_exact_item_and_generation_slot_caps_reject_plus_one() {
    let _guard = prepared_process_guard();
    let mut ledger = PreparedRasterLedger::default();
    let item = ledger.reserve(PREPARED_RASTER_PRODUCER_ITEMS, 1).expect("exact aggregate items");
    assert!(ledger.reserve(1, 0).is_none(), "aggregate item cap plus one");
    assert!(ledger.release(&item));

    let bytes = ledger.reserve(1, PREPARED_RASTER_PRODUCER_BYTES).expect("exact aggregate bytes");
    assert!(ledger.reserve(0, 1).is_none(), "aggregate byte cap plus one");
    assert!(ledger.release(&bytes));

    let mut credits = Vec::with_capacity(PREPARED_RASTER_PRODUCER_CAPACITY);
    for _ in 0..PREPARED_RASTER_PRODUCER_CAPACITY {
        credits.push(ledger.reserve(1, 1).expect("exact fixed generation slot"));
    }
    assert!(ledger.reserve(1, 1).is_none(), "generation slot cap plus one");
    for credit in credits {
        assert!(ledger.release(&credit));
    }
    assert_eq!((ledger.items, ledger.bytes), (0, 0));
}

#[test]
fn raster_credit_epoch_rejects_aba_and_cancel_retires_one_owner_per_grant() {
    let _guard = prepared_process_guard();
    let (mut first, _) = PreparedRasterProducer::try_admit("first".into(), vec![1; PREPARED_RASTER_PAGE_BYTES * 2], 4_096, 2).expect("first admission");
    let first_epoch = first.pages.as_ref().expect("first pages").source_generation();
    assert!(first.bind_frame_generation(3));
    assert!(matches!(first.step(3), PreparedRasterProducerStep::Pending));
    first.begin_close();
    assert!(!first.close_step());
    assert!(first.pages.as_ref().expect("closing pages").slots.is_empty(), "first close grant retires only the built page");
    assert_eq!(first.source.len(), PREPARED_RASTER_PAGE_BYTES * 2, "source remains fully owned after the page grant");
    while !first.close_step() {}
    let (mut second, _) = PreparedRasterProducer::try_admit("second".into(), vec![2; 4], 1, 1).expect("reused slot admission");
    let second_epoch = second.pages.as_ref().expect("second pages").source_generation();
    assert_eq!(second_epoch.slot(), first_epoch.slot(), "released fixed slot is reused");
    assert!(second_epoch.epoch() > first_epoch.epoch(), "reused fixed slot advances its generation");
    second.begin_close();
    while !second.close_step() {}
}

#[test]
fn zero_fuel_and_expired_deadline_advance_no_raster_page_or_allocation() {
    let _guard = prepared_process_guard();
    let (mut producer, _) = PreparedRasterProducer::try_admit("governed".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    assert!(producer.bind_frame_generation(3));
    let source_pointer = producer.source.as_ptr();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(producer).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;

    let (zero,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(0, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);

    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(zero, StepOutcome::Yield));
    let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
    assert_eq!(retained.source.as_ptr(), source_pointer);
    assert!(retained.pages.as_ref().unwrap().slots.is_empty());

    let (expired,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 1,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);

    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(expired, StepOutcome::Yield));
    let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
    assert_eq!(retained.source.as_ptr(), source_pointer);
    assert!(retained.pages.as_ref().unwrap().slots.is_empty());

    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn cancellation_retires_large_upload_incrementally_before_terminal_empty() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4_096], width: 64, height: 64 }).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    assert!(!close_job_step(&mut job));
    assert!(!job.terminal_is_empty());
    let mut turns = 1;
    while !close_job_step(&mut job) {
        turns += 1;
        assert!(turns < 5_000);
    }
    assert!(turns > 4_096);
    assert!(job.terminal_is_empty());
}

fn assert_send<T: Send>() {}

#[test]
fn prepared_packet_is_send_owned_data() {
    let _guard = prepared_process_guard();
    assert_send::<PreparedRenderPacket>();
    assert_send::<PreparedRenderJob>();
    assert_send::<PreparedRenderReceiver>();
    assert_send::<PreparedRenderGate>();
    let packet = packet(7, 3);
    let identity = std::thread::spawn(move || (packet.scene_revision, packet.preview_generation)).join().expect("worker packet");
    assert_eq!(identity, (7, 3));
}

#[test]
fn receiver_survives_worker_ownership_of_the_job() {
    let _guard = prepared_process_guard();
    let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
    let receiver = job.receiver().expect("prepared receiver clone");
    std::thread::spawn(move || {
        let mut job = job;
        let mut preview = 0;
        loop {
            let (outcome,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);
            assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
            if outcome.is_terminal() {
                assert!(matches!(outcome, StepOutcome::Complete(_)));
                break;
            }
        }
    })
    .join()
    .expect("worker preparation");
    let packet = receiver.take_latest().expect("prepared packet handoff");
    assert_eq!((packet.scene_revision(), packet.preview_generation()), (7, 3));
    assert!(receiver.take_latest().is_none());
    drop(packet);
    drain_abandoned_preparations();
}

#[test]
fn preparation_yields_at_the_configured_item_budget() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers.extend((0..4).map(|_| DrawLayer::default()));
    let input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    let (first,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(first, StepOutcome::Yield));
    drop(job);
    drain_abandoned_preparations();
}

#[test]
fn preparation_completes_across_bounded_steps() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers.extend((0..2).map(|_| DrawLayer::default()));
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("prepared packet");
    assert_eq!((packet.scene_revision, packet.preview_generation), (7, 3));
    while !close_packet_step(&mut packet) {}
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn preparation_rejects_a_stale_generation_before_publication() {
    let _guard = prepared_process_guard();
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 2, DrawList::default(), None, 0.0), 8);
    let mut preview = 0;
    let (outcome,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    drop(job);
    drain_abandoned_preparations();
}

#[semio_framework_async_macros::async_test]
async fn preparation_observes_cancellation_without_replacing_a_packet() {
    let cancel = root_cancel_token();
    cancel.cancel().await;
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 8);
    let mut preview = 0;
    let (outcome,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel, now_ms, &mut preview, &mut None);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(outcome, StepOutcome::Cancelled));
    assert!(job.take_packet().is_none());
    drop(job);
    drain_abandoned_preparations();
}

#[test]
fn stale_packet_rejection_preserves_the_last_valid_packet() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    assert!(gate.acknowledge_presented(witness).expect("first presenter acknowledgement").is_empty());
    let stale = packet(6, 3);
    assert!(matches!(gate.validate(&stale, 7, 3), Err(PreparedRenderRejection::StaleRevision { .. })));
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
}

#[test]
fn generation_rejection_happens_before_presentation() {
    let _guard = prepared_process_guard();
    let gate = PreparedRenderGate::default();
    assert!(matches!(gate.validate(&packet(7, 2), 7, 3), Err(PreparedRenderRejection::StaleGeneration { .. })));
}

#[test]
fn device_loss_retains_the_last_valid_packet() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
    let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
    assert_eq!(gate.retain_after_device_loss(), Some((7, 3)));
}

#[test]
fn presenter_ack_is_exact_one_shot_and_preserves_old_until_acknowledged() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
    let second = gate.stage_presented(packet(8, 4)).ok().expect("second presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "candidate is not visible before acknowledgement");
    let stale = PreparedPresenterWitness { sequence: second.sequence.saturating_add(1), scene_revision: second.scene_revision, preview_generation: second.preview_generation };
    assert!(gate.acknowledge_presented(stale).is_err());
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    let duplicate = PreparedPresenterWitness { sequence: second.sequence, scene_revision: second.scene_revision, preview_generation: second.preview_generation };
    let mut replacement = gate.acknowledge_presented(second).expect("exact second acknowledgement");
    assert_eq!(gate.last_valid_identity(), Some((8, 4)));
    assert_eq!(replacement.previous.as_ref().map(|packet| (packet.scene_revision, packet.preview_generation)), Some((7, 3)));
    assert!(gate.acknowledge_presented(duplicate).is_err(), "duplicate acknowledgement is stale after publication");
    let mut previous = replacement.take_previous().expect("old last-valid owner");
    while !close_packet_step(&mut previous) {}
    assert!(previous.retirement_is_empty());
}

#[test]
fn accepted_a_stale_b_abort_and_accepted_c_preserve_exact_presenter_owners() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
    let _missing = gate.stage_presented(packet(8, 4)).ok().expect("pending presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    let mut stale = gate.abort_pending().expect("exact stale B packet handback");
    assert_eq!((stale.scene_revision, stale.preview_generation), (8, 4));
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "aborting B preserves accepted A");

    let third = gate.stage_presented(packet(9, 5)).ok().expect("successor C presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "C remains private before acknowledgement");
    let mut replacement = gate.acknowledge_presented(third).expect("successor C acknowledgement");
    assert_eq!(gate.last_valid_identity(), Some((9, 5)), "C alone replaces A after exact acknowledgement");
    let mut first = replacement.take_previous().expect("accepted A owner handback");
    assert_eq!((first.scene_revision, first.preview_generation), (7, 3));
    while !close_packet_step(&mut stale) {}
    while !close_packet_step(&mut first) {}
    let mut third = gate.take_last_valid().expect("accepted C owner handback");
    while !close_packet_step(&mut third) {}
    while !gate.close_step() {}
    assert!(gate.terminal_is_empty());
}

#[test]
fn pending_presenter_witness_rejects_superseding_packet_with_exact_owner() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let _pending = gate.stage_presented(packet(7, 3)).ok().expect("pending presenter witness");
    let mut superseding = packet(8, 4);
    assert!(superseding.uploads.try_push(PreparedRenderUpload::GlyphAtlas { pixels: vec![7; 16_385], width: 1, height: 1 }).is_ok());
    let pixels = match superseding.uploads.get(0) {
        Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) => pixels.as_ptr(),
        _ => unreachable!(),
    };
    let mut returned = match gate.stage_presented(superseding) {
        Ok(_) => panic!("second presenter witness must fail closed"),
        Err(packet) => packet,
    };
    assert_eq!((returned.scene_revision, returned.preview_generation), (8, 4));
    assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels: returned_pixels, .. }) if returned_pixels.as_ptr() == pixels));
    assert!(!close_packet_step(&mut returned), "one close grant retires only one admitted pixel page");
    assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) if pixels.len() == 1));
}

#[test]
fn gate_close_requires_pending_and_last_valid_packet_handback_before_terminal_scalars() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
    let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
    assert!(!gate.close_step(), "last-valid owner prevents gate terminalization");
    let mut last = gate.take_last_valid().expect("last-valid owner handback");
    while !close_packet_step(&mut last) {}
    assert!(!gate.close_step(), "first scalar grant retires only the sequence");
    assert!(gate.close_step(), "second scalar grant publishes the terminal witness");
    assert!(gate.terminal_is_empty());
    assert!(matches!(gate.validate(&packet(8, 4), 8, 4), Err(PreparedRenderRejection::Closing)));
}

#[test]
fn upload_byte_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    input.limits.max_upload_bytes = 3;
    assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4], width: 2, height: 2 }).is_ok());
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn eviction_byte_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    input.limits.max_upload_bytes = 3;
    assert!(input.try_push_eviction(PreparedRenderEviction::Mesh { key: "mesh".into() }).is_ok());
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn draw_item_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers[0].ui_instances.push(crate::wgpu::draw_types::UiInstance::solid([0.0; 4], crate::wgpu::theme::Rgba::new(0.0, 0.0, 0.0, 0.0)));
    let mut input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
    input.limits.max_draw_items = 0;
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn input_drop_hands_back_exact_process_permits_for_incremental_close() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    drop(input);
    assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    let mut turns = 0;
    while !close_abandoned_input_step() {
        turns += 1;
        assert!(turns < 128);
    }
    assert!(turns > 4);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn worker_panic_hands_back_the_exact_job_and_mailbox_owners() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _owner = job;
        panic!("hostile worker interruption");
    }));
    assert!(result.is_err());
    let mut turns = 0;
    while !close_abandoned_job_step() {
        turns += 1;
        assert!(turns < 256);
    }
    assert!(turns > 4);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    assert!(PREPARED_RENDER_MAILBOX.iter().all(|slot| slot.packet.load(Ordering::Acquire).is_null()));
}

#[test]
fn packet_drop_retires_nested_backings_and_permit_scalars_separately() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let mut owner = packet(7, 3);
    owner.draw.push_solid([0.0, 0.0, 8.0, 8.0], crate::wgpu::theme::Rgba::new(1.0, 0.0, 0.0, 1.0));
    drop(owner);
    let mut turns = 0;
    while !close_abandoned_packet_step() {
        turns += 1;
        assert!(turns < 256);
    }
    assert!(turns > 8);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn fixed_command_pages_reject_max_plus_one_without_consuming_the_owner() {
    let _guard = prepared_process_guard();
    let mut commands = PreparedRenderCommandPages::default();
    for source in 0..PREPARED_RENDER_COMMAND_PAGES * PREPARED_RENDER_COMMAND_PAGE_ITEMS {
        let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source, digest: source as u64, draw_cursor: Some(DrawMeasureCursor::Complete), packet_overlay: false };
        assert!(commands.try_push(command).is_ok());
    }
    let rejected = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source: usize::MAX, digest: u64::MAX, draw_cursor: Some(DrawMeasureCursor::Complete), packet_overlay: true };
    let returned = match commands.try_push(rejected) {
        Ok(()) => panic!("command cap plus one must refuse"),
        Err(returned) => returned,
    };
    assert_eq!((returned.source, returned.digest, returned.packet_overlay), (usize::MAX, u64::MAX, true));
    let mut turns = 0;
    while !commands.close_step() {
        turns += 1;
    }
    assert!(turns >= PREPARED_RENDER_COMMAND_PAGES * PREPARED_RENDER_COMMAND_PAGE_ITEMS);
    assert!(commands.terminal_is_empty());
}

#[test]
fn tessellation_commands_retain_exact_scalar_and_overlay_cursors() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 4.0, 4.0], crate::wgpu::theme::Rgba::new(0.0, 1.0, 0.0, 1.0));
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    let mut preview = 0;
    let mut steps = 0;
    loop {
        let (outcome,original_retained_receipt) = drive_step(&mut job, "ui-wgpu.prepare", OperationId(41), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, &mut None);
        assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
        steps += 1;
        if outcome.is_terminal() {
            assert!(matches!(outcome, StepOutcome::Complete(_)));
            break;
        }
        assert!(steps < 128);
    }
    let mut packet = match job.take_packet() {
        Some(packet) => packet,
        None => panic!("prepared packet handoff"),
    };
    assert!((0..packet.commands.len())
        .filter_map(|index| packet.commands.get(index))
        .any(|command| { command.kind == PreparedRenderCommandKind::Tessellate && command.draw_cursor == Some(DrawMeasureCursor::LayerUi { layer: 0, item: 0, overlay: false }) && !command.packet_overlay }));
    while !close_packet_step(&mut packet) {}
    while !close_job_step(&mut job) {}
}

#[test]
fn prepared_measurement_retains_scene_and_overlay_raster_cursors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).expect("retained Select/overlay raster fixture");
    let key = fixture["image"]["sharedKey"].as_str().expect("shared raster key");
    let mut draw = DrawList::default();
    draw.push_raster_quad(key, [0.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.begin_overlay_route();
    draw.push_raster_quad(key, [4.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.end_overlay_route();
    let mut cursor = DrawMeasureCursor::LayerHeader(0);
    let mut measured = Vec::new();
    for _ in 0..64 {
        let current = cursor;
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() {
            break;
        }
        measured.push(current);
        if cursor == DrawMeasureCursor::Complete {
            break;
        }
    }
    assert!(measured.contains(&DrawMeasureCursor::LayerRaster { layer: 0, raster: 0, overlay: false }));
    assert!(measured.contains(&DrawMeasureCursor::LayerRaster { layer: 0, raster: 0, overlay: true }));
    let raster_order: Vec<_> = measured
        .iter()
        .filter_map(|cursor| match cursor {
            DrawMeasureCursor::LayerRaster { overlay, .. } => Some(*overlay),
            _ => None,
        })
        .collect();
    for row in fixture["image"]["draws"].as_array().expect("draw order rows") {
        let ordinal = usize::try_from(row["expectedOrdinal"].as_u64().expect("expected ordinal")).expect("ordinal fits");
        assert_eq!(raster_order.get(ordinal).copied(), Some(row["route"].as_str() == Some("overlay")));
    }
}

#[test]
fn prepared_packet_publishes_main_overlay_and_top_overlay_raster_owners() {
    let _guard = prepared_process_guard();
    let mut packet = packet(7, 3);
    packet.draw.push_raster_quad("main-image", [0.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.draw.begin_overlay_route();
    packet.draw.push_raster_quad("inline-overlay-image", [4.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.draw.end_overlay_route();
    let mut overlay = DrawList::default();
    overlay.push_raster_quad("top-overlay-image", [8.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.overlay = Some(overlay);
    let mut cursor = PreparedRasterKeepCursorV1::default();
    let mut keys = Vec::new();
    for _ in 0..64 {
        match packet.raster_keep_step(&mut cursor) {
            PreparedRasterKeepStepV1::Pending => {}
            PreparedRasterKeepStepV1::Key(key) => keys.push(key.to_owned()),
            PreparedRasterKeepStepV1::Complete => break,
        }
    }
    assert_eq!(keys, ["main-image", "inline-overlay-image", "top-overlay-image"]);
    while !close_packet_step(&mut packet) {}
}







#[cfg(feature = "wgpu-engine")]
#[path = "../🔬️targets-wgpu-prepared-engine-unit/🦀️.rs"]
mod engine_tests;

#[test]
fn prepared_render_common_close_funds_declared_frontiers_without_losing_original_owners() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️prepared-close/🔣️.json")).unwrap();
    for row in fixture["grants"].as_array().unwrap() {
        let grant = semio_framework_job::RetainedCloneGrant {
            maximum_items: row["items"].as_u64().unwrap() as usize,
            maximum_copy_bytes: row["copy"].as_u64().unwrap() as usize,
            maximum_capacity_bytes: row["capacity"].as_u64().unwrap() as usize,
            maximum_release_bytes: row["release"].as_u64().unwrap() as usize,
            maximum_depth: row["depth"].as_u64().unwrap() as usize,
        };
        let atlas = &fixture["atlas"];
        let pixels = vec![7; atlas["byteLength"].as_u64().unwrap() as usize];
        let mut pages = PreparedAtlasPages::try_new(atlas["width"].as_u64().unwrap() as u32, atlas["height"].as_u64().unwrap() as u32, atlas["channels"].as_u64().unwrap() as u8, pixels.len()).unwrap();
        pages.push_page(&pixels, 0).unwrap();
        let mut retained_input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        retained_input.uploads.try_push(PreparedRenderUpload::GlyphAtlasPages { pixels: pages }).unwrap();
        let mut job = PreparedRenderJob::new(retained_input, 1);
        let input = job.input.as_ref().unwrap() as *const PreparedRenderInput;
        assert!(job.commands.take().unwrap().terminal_is_empty());
        assert_eq!(InteractiveJob::close_step(&mut job, grant), semio_framework_job::InteractiveJobCloseStep::Blocked);
        InteractiveJob::begin_close(&mut job);
        let outcome = InteractiveJob::close_step(&mut job, grant);
        let expected = semio_framework_job::RetainedCloneProgress { copied_items: row["expectedItems"].as_u64().unwrap() as usize, ..Default::default() };
        assert_eq!(outcome, semio_framework_job::InteractiveJobCloseStep::Pending { progress: expected });
        assert!(outcome.progress().fits(grant));
        assert_eq!(job.input.as_ref().unwrap() as *const PreparedRenderInput, input);
        assert_eq!(job.commands.is_some(), row["commandsRetained"].as_bool().unwrap());
        assert!(!PreparedRenderJob::terminal_is_empty(&job));
        assert!(InteractiveJob::next_close_release_byte_demand(&job).is_ok());
        let mut complete = false;
        for _ in 0..10_000 { if close_job_step(&mut job) { complete = true; break; } }
        assert!(complete);
        assert!(PreparedRenderJob::terminal_is_empty(&job));
        assert_eq!(InteractiveJob::close_step(&mut job, grant), semio_framework_job::InteractiveJobCloseStep::Complete { progress: semio_framework_job::RetainedCloneProgress::default() });
        println!("[DEBUG] prepared close {} declared=true originalInput=true fullGrantCleanup=true", row["name"].as_str().unwrap());
    }
}
