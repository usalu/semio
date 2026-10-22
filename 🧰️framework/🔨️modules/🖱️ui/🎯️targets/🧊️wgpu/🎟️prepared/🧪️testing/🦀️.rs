//! 🧪️ Canonical prepared process custody, bounded drive and physical close helpers for native integration laws.
use crate::*;
use semio_framework_job::{root_cancel_token, Generation, InteractiveStage, JobOutcomeBorrow, JobOutcomeKind, OperationId, StepBudget};

pub fn close_packet_step(packet: &mut PreparedRenderPacket) -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = semio_framework_job::RetainedCloneGrant { maximum_copy_bytes: 16 * 1024, ..packet.next_close_demands(16 * 1024).unwrap() };
    match packet.close_step(grant).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(packet.retirement_is_empty()); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
    }
}
pub fn close_abandoned_packet_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderPacket::next_abandoned_close_demands(16 * 1024).unwrap();
    match PreparedRenderPacket::close_abandoned_step(grant).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
    }
}
pub fn close_abandoned_mailbox_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderReceiver::next_abandoned_close_demands(16 * 1024).unwrap();
    match PreparedRenderReceiver::close_abandoned_step(grant).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
    }
}


pub fn close_job_step(job: &mut PreparedRenderJob) -> bool {
    use semio_framework_job::{InteractiveJobCloseStep as Close, RetainedCloneGrant};
    InteractiveJob::begin_close(job);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: InteractiveJob::next_close_copy_byte_demand(job).unwrap(), maximum_capacity_bytes: InteractiveJob::next_close_capacity_byte_demand(job, 1).unwrap(), maximum_release_bytes: InteractiveJob::next_close_release_byte_demand(job).unwrap(), maximum_depth: InteractiveJob::next_close_depth_demand(job).unwrap() };
    match semio_framework_job::close_with_self_funded_context(grant, |cx| InteractiveJob::close_step(job, cx)).expect("original fixture close turn") {
        Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(job.terminal_is_empty()); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
    }
}
pub fn close_abandoned_job_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderJob::next_abandoned_close_demands(1).unwrap();
    match PreparedRenderJob::close_abandoned_step(grant).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
    }
}

pub fn close_abandoned_input_step() -> bool {
    use semio_framework_job::InteractiveJobCloseStep as Close;
    let grant = PreparedRenderInput::next_abandoned_close_demands(16 * 1024).unwrap();
    match PreparedRenderInput::close_abandoned_step(grant).unwrap() {
        Close::Complete { progress } => { assert!(progress.fits(grant)); true },
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
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
        Close::Pending { progress } => { assert!(progress.fits(grant)); false },
        Close::Blocked => false,
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


pub fn observe_prepared_step(job: &mut PreparedRenderJob, operation: OperationId, generation: Generation, budget: StepBudget, cancel: semio_framework_job::CancelToken, now: fn() -> Option<u64>, preview: &mut u64, acknowledgement: RetainedCloneGrant) -> (Option<JobOutcomeKind>, RetainedCloneProgress) {
    let mut receipt = RetainedCloneProgress::default();
    let descriptor = {
        let mut cx = StepContext::new(operation, generation, budget, cancel, now, preview, &mut receipt);
        semio_framework_job::drive_step(job, &mut cx, "ui-wgpu.prepare", InteractiveStage::BackgroundStep, &mut None).unwrap().map(JobOutcomeBorrow::into_descriptor)
    };
    assert!(receipt.fits(budget.retained));
    let kind = descriptor.map(|mut descriptor| {
        let kind = descriptor.kind();
        assert_eq!(job.borrow_outcome(&descriptor).unwrap().is_terminal(), descriptor.is_terminal());
        let progress = descriptor.acknowledge(acknowledgement).progress();
        assert!(progress.fits(acknowledgement));
        assert!(descriptor.is_acknowledged());
        kind
    });
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

