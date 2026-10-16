//! 🧰️ Original caller authorities for stdio artifact tests: native snapshot owners, mounted owner policy and routed IO runs.
//!
//! @see 🏪️store `NativeSnapshotDecodeOwner` / `NativeSnapshotEncodeOwner` — the owners every SQLite snapshot codec receives.

use semio_framework_os_kernel::io::io_mechanism::{io_run_with_snapshot_control, IoRunControl};
use semio_framework_os_kernel::io_schema::{IoPayload, IoResult, IoRoute};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotProgress};
use semio_framework_os_kernel::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_plugin::MountedOwnerPolicyV1;
use semio_framework_value::native_decoding::NativeDecodeRetirementRecipient;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{ErasedSnapshotRetirement as _, NativeDecodeControl, NativeDecodeProgress, NativeEncodeControl, RetainedCloneGrant};

/// 📏️ Native allocation ceiling the original test callers grant.
pub const NATIVE_MAXIMUM_BYTES: usize = 1 << 30;

/// 🎟️ The generous five-axis ownership grant of the original test caller.
pub const fn original_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4_096 }
}

/// ♾️ A grant no test step can exhaust, for jobs whose retained work the test does not meter.
pub const fn unbounded_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }
}

/// 🛂️ The explicit mounted owner policy every test instance runs under: the shipped stdio hub policy, whose item budget batches the interactive publication ladder and whose capacity admits one whole-snapshot apply of the largest shipped fixture.
pub const fn mounted_policy() -> MountedOwnerPolicyV1 {
    MountedOwnerPolicyV1::INTERACTIVE_COMPONENT
}

/// 🚢️ The language-agnostic shipped-fleet SQLite owner law (authored natural texts and exact domain rows), owned by stdio and shared with its TypeScript twin, so that a composition test reads it through this crate and never through a stdio fixture path.
pub const SHIPPED_FLEET_SQLITE_OWNERS: &str = include_str!("../../../🧪️tests/🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json");

/// 🛬️ Runs `operation` under an original native decode owner and closes every retired owner afterwards.
pub fn with_decoding<T>(operation: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut callback = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original decode recipient installs");
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant);
    let result = operation(&mut owner);
    drop(owner);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant).expect("original decode recipient closes");
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
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant).expect("original encode recipient closes");
    }
    result
}

/// 🛫️ Runs an async `operation` under an original native encode owner and closes every retired owner afterwards.
pub async fn with_encoding_async<T>(operation: impl AsyncFnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut callback = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original encode recipient installs");
    let mut owner = NativeSnapshotEncodeOwner::new(&mut native, grant);
    let result = operation(&mut owner).await;
    drop(owner);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant).expect("original encode recipient closes");
    }
    result
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
    while !recipient.terminal_is_empty() {
        recipient.close_step(grant).expect("original SQLite recipient closes");
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

/// 🧹️ Closes both native directions of one routed run through their original grants.
fn close_original(native: &mut IoRunControl<'_, '_>) {
    {
        let mut owner = native.snapshot_decode().expect("original decode direction");
        while owner.native().has_retirement_owner() {
            let grant = owner.remaining_grant();
            let result = owner.native().close_retirement_recipient(grant);
            let progress = match &result {
                Ok(step) => step.progress(),
                Err(error) => error.retained_progress(),
            };
            owner.record_progress(progress).expect("original decode close is funded");
            result.expect("original decode recipient closes");
        }
    }
    let mut owner = native.snapshot_encode().expect("original encode direction");
    while owner.native().has_retirement_owner() {
        let grant = owner.remaining_grant();
        let result = owner.native().close_retirement_recipient(grant);
        let progress = match &result {
            Ok(step) => step.progress(),
            Err(error) => error.retained_progress(),
        };
        owner.record_progress(progress).expect("original encode close is funded");
        result.expect("original encode recipient closes");
    }
}

/// 🛬️ A bare native decode control with its original retirement recipient installed; the recipient is leaked so the control can be built inline at a call site.
pub fn decode_control<'a>(maximum_bytes: usize, callback: &'a mut dyn FnMut(NativeDecodeProgress) -> bool) -> NativeDecodeControl<'a> {
    let recipient: &'a mut NativeDecodeRetirementRecipient = Box::leak(Box::new(NativeDecodeRetirementRecipient::new()));
    let mut native = NativeDecodeControl::new(maximum_bytes, callback);
    native.install_retirement_recipient(recipient).expect("original decode recipient installs");
    native
}
