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
use semio_framework_value::{ErasedSnapshotRetirement as _, NativeEncodeControl, OriginalRetirementFailure, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value_testing_admission::{bank_run, driven, BankWallet, EffectBank, RetainedWorkPoll};

/// 📏️ Native allocation ceiling the original test callers grant.
pub const NATIVE_MAXIMUM_BYTES: usize = 1 << 30;

/// 🎟️ The generous five-axis ownership grant of the original test caller.
pub const fn original_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4_096 }
}

/// 📏️ Body bytes every original close turn may borrow.
const CLOSE_BODY: usize = 4_096;

/// ⚙️ Work units every original close turn may perform.
const CLOSE_WORK: u64 = 1 << 20;

/// 🏦️ The finite effect wallet the original test caller closes its owners from.
pub fn original_bank() -> EffectBank {
    EffectBank::new(BankWallet::new(original_grant(), 1 << 40))
}

/// 🧾️ Takes the progress a settled close turn paid; a pending turn paid nothing.
fn settled(poll: RetainedWorkPoll<RetainedCloneStep, OriginalRetirementFailure>, what: &str) -> RetainedCloneProgress {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.expect(what).progress(),
        RetainedWorkPoll::Pending { .. } => RetainedCloneProgress::default(),
    }
}

/// ♻️ Runs one funded close turn of the original caller and returns its paid progress.
fn close_turn(bank: &mut EffectBank, what: &str, close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<RetainedCloneStep, OriginalRetirementFailure>) -> RetainedCloneProgress {
    settled(bank_run(bank, original_grant(), CLOSE_WORK, close).0, what)
}

/// ♻️ Runs one funded reply-shaped close turn of the original caller and returns its paid progress.
fn encode_turn(bank: &mut EffectBank, what: &str, native: &mut NativeEncodeControl<'_>) -> RetainedCloneProgress {
    settled(driven::<OriginalRetirementFailure>(bank, original_grant(), CLOSE_WORK, |effect, reply, recover| if recover { native.recover_close_retirement_recipient_into(CLOSE_BODY, effect, reply) } else { native.close_retirement_recipient_into(CLOSE_BODY, effect, reply) }).0, what)
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
    let mut bank = original_bank();
    while native.has_retirement_owner() {
        close_turn(&mut bank, "original decode recipient closes", |effect| native.close_retirement_recipient(CLOSE_BODY, effect));
    }
    result
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
    let mut bank = original_bank();
    while native.has_retirement_owner() {
        encode_turn(&mut bank, "original encode recipient closes", &mut native);
    }
    result
}

/// 🗄️ Runs `operation` under a SQLite snapshot control whose caller installed the original retirement recipient, then closes it.
pub fn with_sqlite_recipient<T>(limits: SqliteDatabaseLimits, accept: &mut dyn FnMut(SqliteSnapshotProgress) -> bool, operation: impl FnOnce(&mut SqliteSnapshotControl<'_>) -> T) -> T {
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let result = {
        let mut control = SqliteSnapshotControl::new(accept, limits);
        control.install_native_retirement_recipient(&mut recipient).expect("original SQLite recipient installs");
        operation(&mut control)
    };
    let mut bank = original_bank();
    while !recipient.terminal_is_empty() {
        close_turn(&mut bank, "original SQLite recipient closes", |effect| recipient.close_step(CLOSE_BODY, effect));
    }
    result
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
    let mut bank = original_bank();
    {
        let mut owner = native.snapshot_decode().expect("original decode direction");
        while owner.native().has_retirement_owner() {
            let progress = close_turn(&mut bank, "original decode recipient closes", |effect| owner.native().close_retirement_recipient(CLOSE_BODY, effect));
            owner.record_progress(progress).expect("original decode close is funded");
        }
    }
    let mut owner = native.snapshot_encode().expect("original encode direction");
    while owner.native().has_retirement_owner() {
        let progress = encode_turn(&mut bank, "original encode recipient closes", owner.native());
        owner.record_progress(progress).expect("original encode close is funded");
    }
}
