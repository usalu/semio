//! 🛬️ Original caller authorities for artifact tests: the native snapshot owners and SQLite recipient every snapshot codec receives from its caller.
//!
//! Mounted by `#[path]` into the test modules of artifact crates that depend on `semio_os_kernel` and `semio_framework_value`.
//!
//! @see 🏪️store `NativeSnapshotDecodeOwner` / `NativeSnapshotEncodeOwner`, `SqliteSnapshotControl::install_native_retirement_recipient`.

use semio_os_kernel::io::io_mechanism::{io_run_with_snapshot_control, IoRunControl};
use semio_os_kernel::io_schema::{IoPayload, IoResult, IoRoute};
use semio_os_kernel::sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotProgress};
use semio_os_kernel::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{ErasedSnapshotRetirement as _, NativeEncodeControl, OriginalRetirementFailure, OriginalRetirementPoll, ReplyStash, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneStep, RetainedWorkPoll};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_job::OriginalRetirementPump;

/// 📏️ Native allocation ceiling the original test callers grant.
pub const NATIVE_MAXIMUM_BYTES: usize = 1 << 30;

/// 🎟️ The generous five-axis ownership grant of the original test caller.
pub const fn original_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4_096 }
}

/// 🔒️ Turns an owner-less close may stay pending or denied before the fixture declares the recipient stalled.
const STALL_TURNS: usize = 4_096;

type Settled = Result<RetainedCloneStep, OriginalRetirementFailure>;

/// 🪙️ Runs one host-funded owner-less close turn; `None` means the turn was denied or the child stayed pending with its state kept.
fn pump_turn(grant: RetainedCloneGrant, close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll) -> Option<Settled> {
    let mut turn = OriginalRetirementPump::new().turn(1 << 16, u64::MAX, || Some(0), grant).expect("original retirement turn issues");
    match turn.with_effect(close).expect("original retirement effect runs")?.0 {
        RetainedWorkPoll::Complete(turn) => Some(turn.result),
        RetainedWorkPoll::Pending { .. } => None,
    }
}

/// 📬️ Runs one reply-shaped encoder close turn, recovering a child an earlier turn left pending.
fn encode_turn(native: &mut NativeEncodeControl<'_>, stash: &mut ReplyStash<OriginalRetirementFailure>, grant: RetainedCloneGrant) -> Option<Settled> {
    let body = grant.maximum_copy_bytes;
    pump_turn(grant, |effect| {
        let answer = stash
            .run(|reply, recover| {
                (if recover { native.recover_close_retirement_recipient_into(body, effect, reply) } else { native.close_retirement_recipient_into(body, effect, reply) }).map(drop)
            })
            .expect("original encode recipient close lends its reply");
        match answer {
            Some(result) => RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result, work: 0 }),
            None => RetainedWorkPoll::Pending { work: 0 },
        }
    })
}

/// 🧲️ Runs `operation` under an original native decode owner and closes every retired owner afterwards.
pub fn with_decoding<T>(operation: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut callback = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original decode recipient installs");
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant);
    let result = operation(&mut owner);
    drop(owner);
    for turn in 0..=STALL_TURNS {
        if !native.has_retirement_owner() {
            return result;
        }
        assert!(turn < STALL_TURNS, "original decode recipient stalled");
        let body = grant.maximum_copy_bytes;
        if let Some(step) = pump_turn(grant, |effect| native.close_retirement_recipient(body, effect)) {
            step.expect("original decode recipient closes");
        }
    }
    unreachable!()
}

/// 🛫️ Runs `operation` under an original native encode owner and closes every retired owner afterwards.
pub fn with_encoding<T>(operation: impl FnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut callback = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original encode recipient installs");
    let mut owner = NativeSnapshotEncodeOwner::new(&mut native, grant);
    let result = operation(&mut owner);
    drop(owner);
    let mut stash = ReplyStash::default();
    for turn in 0..=STALL_TURNS {
        if !native.has_retirement_owner() {
            return result;
        }
        assert!(turn < STALL_TURNS, "original encode recipient stalled");
        if let Some(step) = encode_turn(&mut native, &mut stash, grant) {
            step.expect("original encode recipient closes");
        }
    }
    unreachable!()
}

/// 🗄️ Runs `operation` under a SQLite snapshot control whose caller installed the original retirement recipient, then closes it.
pub fn with_sqlite_recipient<T>(limits: SqliteDatabaseLimits, accept: &mut dyn FnMut(SqliteSnapshotProgress) -> bool, operation: impl FnOnce(&mut SqliteSnapshotControl<'_>) -> T) -> T {
    let grant = original_grant();
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let result = {
        let mut control = SqliteSnapshotControl::new(accept, limits);
        control.install_native_retirement_recipient(&mut recipient).expect("original SQLite recipient installs");
        operation(&mut control)
    };
    for turn in 0..=STALL_TURNS {
        if recipient.terminal_is_empty() {
            return result;
        }
        assert!(turn < STALL_TURNS, "original SQLite recipient stalled");
        let body = grant.maximum_copy_bytes;
        if let Some(step) = pump_turn(grant, |effect| recipient.close_step(body, effect)) {
            step.expect("original SQLite recipient closes");
        }
    }
    unreachable!()
}

/// 🚏️ Executes a routed IO payload under original native and SQLite authorities, then closes both native directions.
pub async fn io_run(route: &IoRoute, payload: IoPayload, limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> IoResult<IoPayload> {
    let grant = original_grant();
    let mut receive = |_| true;
    let mut emit = |_| true;
    let mut receive_recipient = NativeDecodeRetirementRecipient::new();
    let mut emit_recipient = NativeEncodeRetirementRecipient::new();
    let mut receive_control = NativeDecodeControl::new(NATIVE_MAXIMUM_BYTES, &mut receive);
    let mut emit_control = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut emit);
    receive_control.install_retirement_recipient(&mut receive_recipient).expect("original decode recipient installs");
    emit_control.install_retirement_recipient(&mut emit_recipient).expect("original encode recipient installs");
    let mut native = IoRunControl::new(&mut receive_control, &mut emit_control, grant);
    let mut snapshot = SqliteSnapshotControl::new(progress, limits);
    let result = io_run_with_snapshot_control(route, &mut Some(payload), &mut native, &mut snapshot).await;
    close_original(&mut native);
    drop(native);
    result
}

/// 🧹️ Closes both native directions of one routed run through host-funded turns charged to their original grants.
fn close_original(native: &mut IoRunControl<'_, '_>) {
    {
        let mut owner = native.snapshot_decode().expect("original decode direction");
        for turn in 0..=STALL_TURNS {
            if !owner.native().has_retirement_owner() {
                break;
            }
            assert!(turn < STALL_TURNS, "original decode direction stalled");
            let grant = owner.remaining_grant();
            let body = grant.maximum_copy_bytes;
            if let Some(step) = pump_turn(grant, |effect| owner.native().close_retirement_recipient(body, effect)) {
                let progress = match &step {
                    Ok(step) => step.progress(),
                    Err(error) => error.retained_progress(),
                };
                owner.record_progress(progress).expect("original decode close is funded");
                step.expect("original decode recipient closes");
            }
        }
    }
    let mut owner = native.snapshot_encode().expect("original encode direction");
    let mut stash = ReplyStash::default();
    for turn in 0..=STALL_TURNS {
        if !owner.native().has_retirement_owner() {
            break;
        }
        assert!(turn < STALL_TURNS, "original encode direction stalled");
        let grant = owner.remaining_grant();
        if let Some(step) = encode_turn(owner.native(), &mut stash, grant) {
            let progress = match &step {
                Ok(step) => step.progress(),
                Err(error) => error.retained_progress(),
            };
            owner.record_progress(progress).expect("original encode close is funded");
            step.expect("original encode recipient closes");
        }
    }
}
