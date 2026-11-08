//! 🧪️ Canonical prepared process custody, bounded drive and physical close helpers for native integration laws.
use crate::*;
use semio_framework_job::{root_cancel_token, Generation, InteractiveStage, JobOutcomeKind, OperationId, StepBudget};

pub fn close_packet_step(packet: &mut PreparedRenderPacket) -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = semio_framework_job::RetainedCloneGrant { maximum_copy_bytes: 16 * 1024, ..packet.next_close_demands(16 * 1024).unwrap() };
    match prepared_close_authored(grant, |effect| packet.close_step(effect)).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(packet.retirement_is_empty()); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); assert!(progress != Default::default(), "prepared close made no progress under its exact demanded grant"); false },
        Close::Blocked => panic!("prepared close is blocked: a close that cannot progress must be a refusal, not a retry"),
    }
}
pub fn close_abandoned_packet_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderPacket::next_abandoned_close_demands(16 * 1024).unwrap();
    match prepared_close_authored(grant, PreparedRenderPacket::close_abandoned_step).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); assert!(progress != Default::default(), "prepared close made no progress under its exact demanded grant"); false },
        Close::Blocked => panic!("prepared close is blocked: a close that cannot progress must be a refusal, not a retry"),
    }
}
pub fn close_abandoned_mailbox_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderReceiver::next_abandoned_close_demands(16 * 1024).unwrap();
    match prepared_close_authored(grant, PreparedRenderReceiver::close_abandoned_step).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); assert!(progress != Default::default(), "prepared close made no progress under its exact demanded grant"); false },
        Close::Blocked => panic!("prepared close is blocked: a close that cannot progress must be a refusal, not a retry"),
    }
}


pub fn close_job_step(job: &mut PreparedRenderJob,caller:&mut semio_framework_job::OriginalJobFixtureCaller) -> bool {
    use semio_framework_job::{InteractiveJobCloseStep as Close, RetainedCloneGrant};
    InteractiveJob::begin_close(job);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: InteractiveJob::next_close_copy_byte_demand(job).unwrap(), maximum_capacity_bytes: InteractiveJob::next_close_capacity_byte_demand(job, 1).unwrap(), maximum_release_bytes: InteractiveJob::next_close_release_byte_demand(job).unwrap(), maximum_depth: InteractiveJob::next_close_depth_demand(job).unwrap() };
    match caller.turn(grant, |cx| InteractiveJob::close_step(job, cx)).expect("original fixture close turn") {
        Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(job.terminal_is_empty()); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); assert!(progress != Default::default(), "prepared close made no progress under its exact demanded grant"); false },
        Close::Blocked => panic!("prepared close is blocked: a close that cannot progress must be a refusal, not a retry"),
    }
}
pub fn close_abandoned_job_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderJob::next_abandoned_close_demands(1).unwrap();
    match prepared_close_authored(grant, PreparedRenderJob::close_abandoned_step).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); assert!(progress != Default::default(), "prepared close made no progress under its exact demanded grant"); false },
        Close::Blocked => panic!("prepared close is blocked: a close that cannot progress must be a refusal, not a retry"),
    }
}

pub fn close_abandoned_input_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderInput::next_abandoned_close_demands(16 * 1024).unwrap();
    match prepared_close_authored(grant, PreparedRenderInput::close_abandoned_step).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); assert!(progress != Default::default(), "prepared close made no progress under its exact demanded grant"); false },
        Close::Blocked => panic!("prepared close is blocked: a close that cannot progress must be a refusal, not a retry"),
    }
}

pub fn atlas_fixture_authority() -> PreparedAtlasAuthority {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 16777216, maximum_depth: 64 };
    PreparedAtlasAuthority { normal: grant, retirement: grant }
}

pub fn atlas_fixture_owner(width: u32, height: u32, channels: u8, bytes: usize) -> Result<PreparedAtlasPages, ValueError> {
    let authority = atlas_fixture_authority();
    let (mut owner, progress) = PreparedAtlasPages::try_new(width, height, channels, bytes, authority)?;
    assert!(progress.fits(authority.normal));
    let progress = owner.admit_original_slots(authority.normal)?;
    assert!(progress.fits(authority.normal));
    Ok(owner)
}

pub fn atlas_fixture_page(owner: &mut PreparedAtlasPages, source: &[u8], row: u32) -> Result<bool, ValueError> {
    let grant = atlas_fixture_authority().normal;
    let (complete, progress) = owner.push_page(source, row, grant)?;
    assert!(progress.fits(grant));
    Ok(complete)
}

pub fn close_abandoned_atlas_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = atlas_fixture_authority().retirement;
    match PreparedAtlasPages::close_abandoned_step(grant).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); assert!(progress != Default::default(), "prepared close made no progress under its exact demanded grant"); false },
        Close::Blocked => panic!("prepared close is blocked: a close that cannot progress must be a refusal, not a retry"),
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
pub fn prepared_process_guard() -> std::sync::MutexGuard<'static, ()> {
    let guard = match PREPARED_PROCESS_TEST_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    drain_abandoned_preparations();
    drain_abandoned_atlases();
    guard
}

pub fn drain_abandoned_atlases() {
    while !close_abandoned_atlas_step() {}
}

pub fn drain_abandoned_preparations() {
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

pub fn now_ms() -> Option<u64> {
    Some(1)
}


/// 🔭️ Drives one prepared step on an admitted original fixture caller and acknowledges its outcome through the genuine return origin.
pub fn observe_prepared_step(job: &mut PreparedRenderJob, operation: OperationId, generation: Generation, budget: StepBudget, cancel: semio_framework_job::CancelToken, now: fn() -> Option<u64>, preview: &mut u64, acknowledgement: RetainedCloneGrant) -> (Option<JobOutcomeKind>, RetainedCloneProgress) {
    use semio_framework_job::{OriginalJobFixtureCaller, OriginalJobReturn, OriginalJobReturnOrigin};
    let stock = close_source::STOCK;
    let mut caller = OriginalJobFixtureCaller::admit(operation, generation, stock, now, u64::MAX, 1 << 20);
    if cancel.is_cancelled_now() { caller.cancel().cancel_now(); }
    let mut origin = OriginalJobReturnOrigin::empty();
    let mut returned = None;
    let mut receipt = RetainedCloneProgress::default();
    let mut ready_after_setup = false;
    for _ in 0..64 {
        let mut ready = false;
        caller.turn(stock, |cx| cx.with_original_work_loan(16, cx.original_absolute_deadline_us(), |cx| { ready = origin.admit(cx)?; Ok(()) })).expect("prepared return origin setup turn");
        if ready { ready_after_setup = true; break; }
    }
    assert!(ready_after_setup, "prepared return origin never became ready");
    caller.turn(budget.retained, |cx| cx.with_original_work_loan(budget.fuel, budget.deadline_us, |cx| {
        let before = cx.retained_progress();
        let mut verdict = None;
        match semio_framework_job::drive_step(job, cx, origin.receiver()?, "ui-wgpu.prepare", InteractiveStage::BackgroundStep, &mut verdict)? {
            OriginalJobReturn::Outcome(reply) => returned = Some(reply.into_descriptor()),
            OriginalJobReturn::Preparing | OriginalJobReturn::Pending => {},
            OriginalJobReturn::Continuation(_) => panic!("prepared render job returned a failure continuation"),
        }
        let after = cx.retained_progress();
        receipt = RetainedCloneProgress { copied_items: after.copied_items - before.copied_items, copied_bytes: after.copied_bytes - before.copied_bytes, retained_capacity_bytes: after.retained_capacity_bytes - before.retained_capacity_bytes, released_bytes: after.released_bytes - before.released_bytes };
        Ok(())
    })).expect("prepared original drive turn");
    assert!(receipt.fits(budget.retained));
    let mut kind = None;
    if returned.is_some() {
        for _ in 0..64 {
            let received = caller.turn(budget.retained, |cx| origin.receive_outcome(cx, &mut returned)).expect("prepared outcome receive turn");
            if received && returned.is_none() { break; }
        }
        let descriptor = origin.outcome().expect("received prepared outcome");
        kind = Some(descriptor.kind());
        assert_eq!(job.borrow_outcome(descriptor).unwrap().is_terminal(), descriptor.is_terminal());
        if kind == Some(JobOutcomeKind::PreviewReady) { *preview += 1; }
        for _ in 0..64 {
            if caller.turn(acknowledgement, |cx| origin.acknowledge_outcome(cx)).expect("prepared outcome acknowledgement turn") { break; }
        }
        assert!(origin.outcome().is_none());
    }
    for _ in 0..4096 {
        if origin.terminal_is_empty() { break; }
        caller.turn(stock, |cx| origin.close_step(cx).map(|_| ())).expect("prepared return origin close turn");
    }
    assert!(origin.terminal_is_empty());
    drop(origin);
    for _ in 0..64 {
        if caller.terminal_is_empty() { break; }
        caller.retire_step(stock).expect("prepared fixture root close");
    }
    assert!(caller.terminal_is_empty());
    let total = caller.cumulative_receipt();
    assert_eq!(total.retained_capacity_bytes + receipt.released_bytes, total.released_bytes + receipt.retained_capacity_bytes, "every byte the receipts retained is released by the origin or the roots, except what this drive turn handed to the job's own custody: {total:?}");
    drop(caller);
    (kind, receipt)
}

pub fn observe_prepared_terminal(job: &mut PreparedRenderJob, operation: OperationId, generation: Generation, budget: StepBudget, cancel: semio_framework_job::CancelToken, now: fn() -> Option<u64>, preview: &mut u64, acknowledgement: RetainedCloneGrant) -> (Option<JobOutcomeKind>, RetainedCloneProgress) {
    for _ in 0..4_096 {
        let result = observe_prepared_step(job, operation, generation, budget, cancel.clone(), now, preview, acknowledgement);
        if result.0.is_some_and(|kind| matches!(kind, JobOutcomeKind::Complete | JobOutcomeKind::Cancelled | JobOutcomeKind::Fault)) { return result; }
    }
    panic!("prepared render job did not reach a terminal outcome within fixed test credits");
}

pub fn drive_preparation_until_terminal(job: &mut PreparedRenderJob) -> Option<JobOutcomeKind> {
    let mut preview = 0;
    for _ in 0..4_096 {
        let (outcome,original_retained_receipt) = observe_prepared_step(job, OperationId(1), Generation(3), StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
        assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
        if !matches!(outcome, None | Some(JobOutcomeKind::Yield)) {
            return outcome;
        }
    }
    panic!("prepared render job did not reach a terminal outcome within fixed test credits");
}

/// 🧫️ Builds one armed packet under the independently authored fixed test authority, so white-box laws never touch private packet custody.
pub fn packet(revision: u64, generation: u64) -> PreparedRenderPacket {
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
        draw: semio_ui_draw_contract::DrawList::default(),
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

/// 📮️ Observes that no render mailbox slot still owns a packet.
pub fn render_mailbox_is_empty() -> bool {
    PREPARED_RENDER_MAILBOX.iter().all(|slot| slot.packet.load(std::sync::atomic::Ordering::Acquire).is_null())
}

#[path="../../../../../🧵️job/🧪️testing/♻️close/🦀️.rs"]
pub mod close_source;
