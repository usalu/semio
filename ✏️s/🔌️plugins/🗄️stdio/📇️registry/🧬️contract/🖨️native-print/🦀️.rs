//! 🖨️ Original-caller authority for complete native text printing of a snapshot.
//!
//! @see 🏪️store `NativeSnapshotEncodeOwner` — the owner every SQLite snapshot encoder receives.
//! @see 🪶️sqlite-snapshot `SqliteSnapshotControl::run_as_original_caller` — the SQL half of the same funding rule.

use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotProgress};
use semio_framework_os_kernel::NativeSnapshotEncodeOwner;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant, ValueError};

/// 🎟️ The full five-axis ownership grant a printing caller funds its own physical close from.
const PRINT_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: u32::MAX as usize, maximum_copy_bytes: u32::MAX as usize, maximum_capacity_bytes: u32::MAX as usize, maximum_release_bytes: u32::MAX as usize, maximum_depth: 1 << 16 };

/// 🖨️ Runs `operation` as the original caller of a native snapshot encode: it installs the empty native retirement recipient and the SQL recipient, then closes both before returning.
pub fn print_as_original_caller<T>(limits: SqliteDatabaseLimits, operation: impl FnOnce(&mut SqliteSnapshotControl<'_>, &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<T, ValueError>) -> Result<T, ValueError> {
    let mut progress = |_: SqliteSnapshotProgress| true;
    let mut native_progress = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(limits.max_allocation_bytes, &mut native_progress);
    native.install_retirement_recipient(&mut recipient)?;
    let result = SqliteSnapshotControl::run_as_original_caller(&mut progress, limits, |control| {
        let mut owner = NativeSnapshotEncodeOwner::new(&mut native, PRINT_GRANT);
        operation(control, &mut owner)
    });
    let mut closed = Ok(());
    while closed.is_ok() && native.has_retirement_owner() {
        closed = native.close_retirement_recipient(PRINT_GRANT).map(|_| ());
    }
    match closed {
        Ok(()) => result,
        Err(error) => {
            std::mem::forget(recipient);
            result.and(Err(error))
        }
    }
}
