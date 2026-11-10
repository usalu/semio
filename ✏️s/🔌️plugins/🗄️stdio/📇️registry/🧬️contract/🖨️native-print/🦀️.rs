//! 🖨️ Original-caller authority for complete native text printing of a snapshot.
//!
//! @see 🏪️store `NativeSnapshotEncodeOwner` — the owner every SQLite snapshot encoder receives.

use semio_os_kernel::sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotProgress};
use semio_os_kernel::NativeSnapshotEncodeOwner;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::retained_clone::OriginalEffectReply;
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant, ValueError};

/// ⛽️ The host fuel each printing retirement turn is funded with.
const PRINT_FUEL: u64 = 1 << 16;

/// 🎟️ The full five-axis ownership grant a printing caller funds its own physical close from.
const PRINT_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: u32::MAX as usize, maximum_copy_bytes: u32::MAX as usize, maximum_capacity_bytes: u32::MAX as usize, maximum_release_bytes: u32::MAX as usize, maximum_depth: 1 << 16 };

/// 🖨️ Runs `operation` as the original caller of a native snapshot encode: it installs the empty native retirement recipient, then closes it through host-funded retirement turns before returning.
pub fn print_as_original_caller<T>(limits: SqliteDatabaseLimits, operation: impl FnOnce(&mut SqliteSnapshotControl<'_>, &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<T, ValueError>) -> Result<T, ValueError> {
    let mut progress = |_: SqliteSnapshotProgress| true;
    let mut native_progress = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(limits.max_allocation_bytes, &mut native_progress);
    native.install_retirement_recipient(&mut recipient)?;
    let result = {
        let mut control = SqliteSnapshotControl::new(&mut progress, limits);
        let mut owner = NativeSnapshotEncodeOwner::new(&mut native, PRINT_GRANT);
        operation(&mut control, &mut owner)
    };
    let mut answer = None;
    let mut pending = false;
    let mut closed = Ok(());
    while closed.is_ok() && native.has_retirement_owner() {
        let turn = OriginalRetirementPump::new().turn(PRINT_FUEL, u64::MAX, || Some(0), PRINT_GRANT).and_then(|mut turn| {
            turn.with_effect(|effect| {
                if answer.is_none() {
                    let reply = OriginalEffectReply::borrow(&mut answer)?;
                    let call = if pending { native.recover_close_retirement_recipient_into(PRINT_GRANT.maximum_copy_bytes, effect, reply) } else { native.close_retirement_recipient_into(PRINT_GRANT.maximum_copy_bytes, effect, reply) };
                    call?;
                }
                Ok::<_, ValueError>(())
            })
        });
        closed = match turn {
            Ok(Some((Ok(()), _))) => match answer.take() {
                Some(Ok(_)) => { pending = false; Ok(()) }
                Some(Err(failure)) => Err(failure.into_value_error()),
                None => { pending = true; Ok(()) }
            },
            Ok(Some((Err(error), _))) => Err(error),
            Ok(None) => Ok(()),
            Err(error) => Err(error),
        };
    }
    match closed {
        Ok(()) => result,
        Err(error) => {
            std::mem::forget(recipient);
            result.and(Err(error))
        }
    }
}
