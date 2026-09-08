
use super::*;
use semio_framework_job::{Generation, InteractiveStage, OperationId, StepBudget, drive_step, root_cancel_token};

static ATLAS_TEST_LOCK: Mutex<()> = Mutex::new(());

fn atlas_test_guard() -> std::sync::MutexGuard<'static, ()> {
    match ATLAS_TEST_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn drain_abandoned_atlases() {
    while !PreparedAtlasPages::close_abandoned_step() {}
}

fn drain_abandoned_preparations() {
    loop {
        let inputs = PreparedRenderInput::close_abandoned_step();
        let jobs = PreparedRenderJob::close_abandoned_step();
        let mailboxes = PreparedRenderReceiver::close_abandoned_step();
        let packets = PreparedRenderPacket::close_abandoned_step();
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
        let outcome = drive_step(job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
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
fn stale_generation_does_not_consume_a_prepared_raster_page() {
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
    let _guard = atlas_test_guard();
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
    let _guard = atlas_test_guard();
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
    let _guard = atlas_test_guard();
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
    let _guard = atlas_test_guard();
    drain_abandoned_atlases();
    let mut owner = match PreparedAtlasPages::try_new(4, 2, 4, 32) {
        Ok(owner) => owner,
        Err(fault) => panic!("abandonment owner: {fault}"),
    };
    assert!(owner.push_page(&[7; 32], 0).is_ok());
    drop(owner);
    let mut turns = 0;
    while !PreparedAtlasPages::close_abandoned_step() {
        turns += 1;
        assert!(turns < 16, "one page, backing owner, and four permit fields must converge");
    }
    assert!(turns >= 7);
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn interrupted_atlas_close_rejoins_the_same_abandonment_authority() {
    let _guard = atlas_test_guard();
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
    while !PreparedAtlasPages::close_abandoned_step() {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(turns >= 7);
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn atlas_allocation_refusal_preserves_the_packed_permit_ledger() {
    let _guard = atlas_test_guard();
    drain_abandoned_atlases();
    let before = PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire);
    assert!(matches!(PreparedAtlasPages::try_new(0, 1, 4, 4), Err("atlas dimensions do not fit fixed page credits")));
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), before);
}

#[test]
fn atlas_contended_permit_attempts_are_nonblocking_and_poison_free() {
    let _guard = atlas_test_guard();
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
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(11), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
        assert!(matches!(outcome, StepOutcome::Yield));
        assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().retained_source.len(), expected);
    }
    assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().retained_source.as_ptr(), retained_pointer);
    while !job.close_step() {}
    assert!(job.terminal_is_empty());
}

#[test]
fn raster_ledger_exact_item_and_generation_slot_caps_reject_plus_one() {
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
    let (mut producer, _) = PreparedRasterProducer::try_admit("governed".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    assert!(producer.bind_frame_generation(3));
    let source_pointer = producer.source.as_ptr();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(producer).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;

    let zero = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(0, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(zero, StepOutcome::Yield));
    let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
    assert_eq!(retained.source.as_ptr(), source_pointer);
    assert!(retained.pages.as_ref().unwrap().slots.is_empty());

    let expired = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 1), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(expired, StepOutcome::Yield));
    let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
    assert_eq!(retained.source.as_ptr(), source_pointer);
    assert!(retained.pages.as_ref().unwrap().slots.is_empty());

    while !job.close_step() {}
    assert!(job.terminal_is_empty());
}

#[test]
fn cancellation_retires_large_upload_incrementally_before_terminal_empty() {
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4_096], width: 64, height: 64 }).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    assert!(!job.close_step());
    assert!(!job.terminal_is_empty());
    let mut turns = 1;
    while !job.close_step() {
        turns += 1;
        assert!(turns < 5_000);
    }
    assert!(turns > 4_096);
    assert!(job.terminal_is_empty());
}

fn assert_send<T: Send>() {}

#[test]
fn prepared_packet_is_send_owned_data() {
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
    let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
    let receiver = job.receiver().expect("prepared receiver clone");
    std::thread::spawn(move || {
        let mut job = job;
        let mut preview = 0;
        loop {
            let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
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
}

#[test]
fn preparation_yields_at_the_configured_item_budget() {
    let mut draw = DrawList::default();
    draw.layers.extend((0..4).map(|_| DrawLayer::default()));
    let input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    let first = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(first, StepOutcome::Yield));
}

#[test]
fn preparation_completes_across_bounded_steps() {
    let mut draw = DrawList::default();
    draw.layers.extend((0..2).map(|_| DrawLayer::default()));
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("prepared packet");
    assert_eq!((packet.scene_revision, packet.preview_generation), (7, 3));
    while !packet.retire_step() {}
    while !PreparedRenderJob::close_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn preparation_rejects_a_stale_generation_before_publication() {
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 2, DrawList::default(), None, 0.0), 8);
    let mut preview = 0;
    let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
}

#[semio_framework_async_macros::async_test]
async fn preparation_observes_cancellation_without_replacing_a_packet() {
    let cancel = root_cancel_token();
    cancel.cancel().await;
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 8);
    let mut preview = 0;
    let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), cancel, now_ms, &mut preview, &mut None);
    assert!(matches!(outcome, StepOutcome::Cancelled));
    assert!(job.take_packet().is_none());
}

#[test]
fn stale_packet_rejection_preserves_the_last_valid_packet() {
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    assert!(gate.acknowledge_presented(witness).expect("first presenter acknowledgement").is_empty());
    let stale = packet(6, 3);
    assert!(matches!(gate.validate(&stale, 7, 3), Err(PreparedRenderRejection::StaleRevision { .. })));
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
}

#[test]
fn generation_rejection_happens_before_presentation() {
    let gate = PreparedRenderGate::default();
    assert!(matches!(gate.validate(&packet(7, 2), 7, 3), Err(PreparedRenderRejection::StaleGeneration { .. })));
}

#[test]
fn device_loss_retains_the_last_valid_packet() {
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
    let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
    assert_eq!(gate.retain_after_device_loss(), Some((7, 3)));
}

#[test]
fn presenter_ack_is_exact_one_shot_and_preserves_old_until_acknowledged() {
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
    while !previous.retire_step() {}
    assert!(previous.retirement_is_empty());
}

#[test]
fn missing_ack_and_abort_return_the_exact_candidate_without_replacing_last_valid() {
    let mut gate = PreparedRenderGate::default();
    let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
    let _missing = gate.stage_presented(packet(8, 4)).ok().expect("pending presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    let candidate = gate.abort_pending().expect("exact pending packet handback");
    assert_eq!((candidate.scene_revision, candidate.preview_generation), (8, 4));
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
}

#[test]
fn pending_presenter_witness_rejects_superseding_packet_with_exact_owner() {
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
    assert!(!returned.retire_step(), "one close grant retires only one admitted pixel page");
    assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) if pixels.len() == 1));
}

#[test]
fn gate_close_requires_pending_and_last_valid_packet_handback_before_terminal_scalars() {
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
    let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
    assert!(!gate.close_step(), "last-valid owner prevents gate terminalization");
    let mut last = gate.take_last_valid().expect("last-valid owner handback");
    while !last.retire_step() {}
    assert!(!gate.close_step(), "first scalar grant retires only the sequence");
    assert!(gate.close_step(), "second scalar grant publishes the terminal witness");
    assert!(gate.terminal_is_empty());
    assert!(matches!(gate.validate(&packet(8, 4), 8, 4), Err(PreparedRenderRejection::Closing)));
}

#[test]
fn upload_byte_cap_faults_before_packet_publication() {
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    input.limits.max_upload_bytes = 3;
    assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4], width: 2, height: 2 }).is_ok());
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !PreparedRenderJob::close_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn eviction_byte_cap_faults_before_packet_publication() {
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    input.limits.max_upload_bytes = 3;
    assert!(input.try_push_eviction(PreparedRenderEviction::Mesh { key: "mesh".into() }).is_ok());
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !PreparedRenderJob::close_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn draw_item_cap_faults_before_packet_publication() {
    let mut draw = DrawList::default();
    draw.layers[0].ui_instances.push(crate::wgpu::draw_types::UiInstance::solid([0.0; 4], crate::wgpu::theme::Rgba::new(0.0, 0.0, 0.0, 0.0)));
    let mut input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
    input.limits.max_draw_items = 0;
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert!(job.take_packet().is_none());
    while !PreparedRenderJob::close_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn input_drop_hands_back_exact_process_permits_for_incremental_close() {
    let _guard = atlas_test_guard();
    drain_abandoned_preparations();
    let input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    drop(input);
    assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    let mut turns = 0;
    while !PreparedRenderInput::close_abandoned_step() {
        turns += 1;
        assert!(turns < 128);
    }
    assert!(turns > 4);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn worker_panic_hands_back_the_exact_job_and_mailbox_owners() {
    let _guard = atlas_test_guard();
    drain_abandoned_preparations();
    let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _owner = job;
        panic!("hostile worker interruption");
    }));
    assert!(result.is_err());
    let mut turns = 0;
    while !PreparedRenderJob::close_abandoned_step() {
        turns += 1;
        assert!(turns < 256);
    }
    assert!(turns > 4);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    assert!(PREPARED_RENDER_MAILBOX.iter().all(|slot| slot.packet.load(Ordering::Acquire).is_null()));
}

#[test]
fn packet_drop_retires_nested_backings_and_permit_scalars_separately() {
    let _guard = atlas_test_guard();
    drain_abandoned_preparations();
    let mut owner = packet(7, 3);
    owner.draw.push_solid([0.0, 0.0, 8.0, 8.0], crate::wgpu::theme::Rgba::new(1.0, 0.0, 0.0, 1.0));
    drop(owner);
    let mut turns = 0;
    while !PreparedRenderPacket::close_abandoned_step() {
        turns += 1;
        assert!(turns < 256);
    }
    assert!(turns > 8);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn fixed_command_pages_reject_max_plus_one_without_consuming_the_owner() {
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
    let _guard = atlas_test_guard();
    drain_abandoned_preparations();
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 4.0, 4.0], crate::wgpu::theme::Rgba::new(0.0, 1.0, 0.0, 1.0));
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    let mut preview = 0;
    let mut steps = 0;
    loop {
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(41), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 10), root_cancel_token(), now_ms, &mut preview, &mut None);
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
    assert!(
        (0..packet.commands.len())
            .filter_map(|index| packet.commands.get(index))
            .any(|command| { command.kind == PreparedRenderCommandKind::Tessellate && command.draw_cursor == Some(DrawMeasureCursor::LayerUi { layer: 0, item: 0, overlay: false }) && !command.packet_overlay })
    );
    while !packet.retire_step() {}
    while !job.close_step() {}
}
