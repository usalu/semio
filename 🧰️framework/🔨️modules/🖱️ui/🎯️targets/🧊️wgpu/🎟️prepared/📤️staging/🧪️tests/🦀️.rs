use super::*;

/// 🧫️ Drives one original staging turn under the independently authored fixed test authority.
fn close_original_staging(close: impl FnOnce(RetainedCloneGrant) -> Result<CloseStep, ValueError>) -> bool {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧷️prepared-staging/🎟️close-grant.json")).unwrap();
    let n = |key: &str| law["grant"][key].as_u64().unwrap() as usize;
    let grant = RetainedCloneGrant { maximum_items: n("maximumItems"), maximum_copy_bytes: n("maximumCopyBytes"), maximum_capacity_bytes: n("maximumCapacityBytes"), maximum_release_bytes: n("maximumReleaseBytes"), maximum_depth: n("maximumDepth") };
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| close(grant));
    let (complete, progress) = match result.unwrap() { CloseStep::Complete { progress } => (true, progress), CloseStep::Pending { progress } => (false, progress), CloseStep::Blocked => (false, Default::default()), CloseStep::Refused { kind, progress } => panic!("original fixed staging close refused: {kind:?} {progress:?}") };
    assert!(!heap.overflowed);
    assert!(progress.fits(grant));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
    complete
}

pub(super) fn atlas_fixture_authority() -> PreparedAtlasAuthority {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 16777216, maximum_depth: 64 };
    PreparedAtlasAuthority { normal: grant, retirement: grant }
}

pub(super) fn atlas_fixture_owner(width: u32, height: u32, channels: u8, bytes: usize) -> Result<PreparedAtlasPages, ValueError> {
    let authority = atlas_fixture_authority();
    let (mut owner, progress) = PreparedAtlasPages::try_new(width, height, channels, bytes, authority)?;
    assert!(progress.fits(authority.normal));
    let progress = owner.admit_original_slots(authority.normal)?;
    assert!(progress.fits(authority.normal));
    Ok(owner)
}

pub(super) fn atlas_fixture_page(owner: &mut PreparedAtlasPages, source: &[u8], row: u32) -> Result<bool, ValueError> {
    let grant = atlas_fixture_authority().normal;
    let (complete, progress) = owner.push_page(source, row, grant)?;
    assert!(progress.fits(grant));
    Ok(complete)
}

fn close_abandoned_atlas_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = atlas_fixture_authority().retirement;
    match PreparedAtlasPages::close_abandoned_step(grant) {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
        Close::Refused { kind, progress } => panic!("original abandoned atlas refused admitted close: {kind:?}, progress: {progress:?}"),
    }
}


static PREPARED_PROCESS_TEST_LOCK: Mutex<()> = Mutex::new(());
pub(super) fn prepared_process_guard() -> std::sync::MutexGuard<'static, ()> {
    let guard = PREPARED_PROCESS_TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    drain_abandoned_atlases();
    guard
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
    while !close_original_staging(|grant| producer.close_step(grant)) {}
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
    assert!(!close_original_staging(|grant| rejected.close_step(grant)), "one rejection grant retires one source page only");
    while !close_original_staging(|grant| rejected.close_step(grant)) {}
}

#[test]
fn atlas_page_cap_plus_one_faults_before_process_credit_transfer() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let height = 33_554_433_u32;
    let byte_len = 33_554_433_usize;
    assert!(matches!(atlas_fixture_owner(1, height, 1, byte_len), Err(ref error) if error.message.as_ref() == "atlas page or byte credits exceeded"));
    let mut admitted = match atlas_fixture_owner(4, 2, 4, 32) {
        Ok(admitted) => admitted,
        Err(fault) => panic!("fixed atlas admission faulted: {fault}"),
    };
    let mut turns = 0;
    while !matches!(admitted.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }) {
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
    let mut pages = match atlas_fixture_owner(4, 2, 4, source.len()) {
        Ok(pages) => pages,
        Err(fault) => panic!("fixed atlas admission faulted: {fault}"),
    };
    assert!(matches!(atlas_fixture_page(&mut pages, &source, 0), Ok(true)));
    assert_eq!(pages.page(0), Some((&source[..], 0, 2)));
    assert!(!matches!(pages.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }));
    assert_eq!(pages.len(), 0);
    assert!(!pages.terminal_is_empty());
    let mut turns = 0;
    while !matches!(pages.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }) {
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
        *owner = Some(match atlas_fixture_owner(1, 1, 1, 1) {
            Ok(owner) => owner,
            Err(fault) => panic!("one exact process item: {fault}"),
        });
    }
    assert!(matches!(atlas_fixture_owner(1, 1, 1, 1), Err(ref error) if error.message.as_ref() == "atlas process permit credits exhausted"));
    for owner in owners.iter_mut().filter_map(Option::as_mut) {
        while !matches!(owner.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }) {}
        assert!(owner.terminal_is_empty());
    }
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn abandoned_atlas_schedules_the_same_incremental_close_authority() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let mut owner = match atlas_fixture_owner(4, 2, 4, 32) {
        Ok(owner) => owner,
        Err(fault) => panic!("abandonment owner: {fault}"),
    };
    assert!(atlas_fixture_page(&mut owner, &[7; 32], 0).is_ok());
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
    let mut owner = match atlas_fixture_owner(1, u32::try_from(source.len()).unwrap_or(u32::MAX), 1, source.len()) {
        Ok(owner) => owner,
        Err(fault) => panic!("two-page abandonment owner: {fault}"),
    };
    assert!(matches!(atlas_fixture_page(&mut owner, &source, 0), Ok(false)));
    assert!(matches!({ let row = owner.next_row(); atlas_fixture_page(&mut owner, &source, row) }, Ok(true)));
    assert!(!matches!(owner.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }));
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
    assert!(matches!(atlas_fixture_owner(0, 1, 4, 4), Err(ref error) if error.message.as_ref() == "atlas dimensions do not fit fixed page credits"));
    assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), before);
}

#[test]
fn atlas_contended_permit_attempts_are_nonblocking_and_poison_free() {
    let _guard = prepared_process_guard();
    drain_abandoned_atlases();
    let handles = std::array::from_fn::<_, 8, _>(|_| {
        std::thread::spawn(|| match atlas_fixture_owner(1, 1, 1, 1) {
            Ok(mut owner) => {
                while !matches!(owner.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }) {}
                owner.terminal_is_empty()
            }
            Err(ref error) if error.message.as_ref() == "atlas process permit credits exhausted" => true,
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
    while !close_original_staging(|grant| rejected.close_step(grant)) {}
    assert!(rejected.terminal_is_empty());

    let reservation = PreparedRasterReservation::try_reserve("item-plus-one".into()).expect("initial plus-one reservation");
    let mut rejected = reservation.claim(4_096, 1_025).expect_err("sixteen MiB plus one row");
    assert_eq!(rejected.fault(), "raster producer exceeded fixed item or byte credits");
    while !close_original_staging(|grant| rejected.close_step(grant)) {}
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
    while !close_original_staging(|grant| rejected.close_step(grant)) {}

    let reservation = PreparedRasterReservation::try_reserve_source(String::new(), source_bytes + 1).expect("plus one source is initially retained");
    let mut rejected = reservation.claim(4_096, height as u32).expect_err("simultaneous source and decoded peak plus one must fail before decode");
    assert_eq!(rejected.fault(), "raster producer exact credit resize failed");
    while !close_original_staging(|grant| rejected.close_step(grant)) {}
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
    assert!(!close_original_staging(|grant| first.close_step(grant)));
    assert!(first.pages.as_ref().expect("closing pages").slots.is_empty(), "first close grant retires only the built page");
    assert_eq!(first.source.len(), PREPARED_RASTER_PAGE_BYTES * 2, "source remains fully owned after the page grant");
    while !close_original_staging(|grant| first.close_step(grant)) {}
    let (mut second, _) = PreparedRasterProducer::try_admit("second".into(), vec![2; 4], 1, 1).expect("reused slot admission");
    let second_epoch = second.pages.as_ref().expect("second pages").source_generation();
    assert_eq!(second_epoch.slot(), first_epoch.slot(), "released fixed slot is reused");
    assert!(second_epoch.epoch() > first_epoch.epoch(), "reused fixed slot advances its generation");
    second.begin_close();
    while !close_original_staging(|grant| second.close_step(grant)) {}
}

fn drain_abandoned_atlases() { while !close_abandoned_atlas_step() {} }
