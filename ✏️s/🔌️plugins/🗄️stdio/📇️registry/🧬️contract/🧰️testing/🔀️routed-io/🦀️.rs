//! 🔀️ Routed IO runs and SQLite snapshot controls under original native authorities.

use super::{NATIVE_MAXIMUM_BYTES, original_grant};
use super::native_owners::{finish_decode, finish_encode, retirement_effect};
use semio_os_kernel::io::io_mechanism::{IoRunControl, io_run_with_snapshot_control};
use semio_os_kernel::io_schema::{IoPayload, IoResult, IoRoute};
use semio_os_kernel::sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotProgress};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{ErasedSnapshotRetirement as _, NativeEncodeControl, RetainedWorkPoll};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

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
        let poll = retirement_effect(grant, |effect| recipient.close_step(grant.maximum_copy_bytes, effect));
        if let RetainedWorkPoll::Complete(turn) = poll {
            turn.result.expect("original SQLite recipient closes");
        }
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
    finish_decode(&mut native.snapshot_decode().expect("original decode direction"));
    finish_encode(&mut native.snapshot_encode().expect("original encode direction"));
    drop(native);
    result
}
