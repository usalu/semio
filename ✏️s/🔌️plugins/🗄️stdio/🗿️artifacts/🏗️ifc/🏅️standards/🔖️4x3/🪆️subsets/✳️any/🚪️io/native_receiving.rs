//! 🫴️ One value-producing body runs as one funded native snapshot turn over the original owner; refusals keep their original cause.
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, RetainedWorkOutcome, RetireOwned, ValueError, ValueRefusalKind};
use semio_os_kernel::io::{io_mechanism::IoRunControl, io_schema::IoError};
use semio_os_kernel::{native_snapshot_turn, NativeSnapshotBodyWallet, NativeSnapshotWork, NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotFailure};

/// 🧾️ Folds one refused native snapshot turn into its value refusal.
pub fn failure(failure: NativeSnapshotFailure) -> ValueError {
    ValueError::new(failure.kind(), failure.message().to_string())
}
/// 🚫️ Folds one refused native snapshot turn into the route refusal it carries.
pub fn refused(error: NativeSnapshotFailure) -> IoError {
    IoError::from_value_error(failure(error))
}
fn missing() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "native receiving published no output")
}
/// 📥️ Receives one decoded value under the original decode owner.
pub fn decoded<T: RetireOwned, O: RetireOwned>(owner: &mut NativeSnapshotDecodeOwner<'_, '_>, body: impl FnOnce(&mut Option<T>, &mut NativeDecodeControl<'_>, &mut NativeSnapshotBodyWallet) -> Result<O, ValueError>) -> Result<O, ValueError> {
    let (mut output, mut body) = (None, Some(body));
    owner
        .receive::<T, O, NativeSnapshotFailure>(&mut output, &mut |slot, result, _, _, native, wallet| {
            native_snapshot_turn(|work| {
                work.performed();
                *result = Some(body.take().expect("native receiving body runs once")(slot, native, wallet)?);
                Ok(())
            })
        })
        .result
        .map_err(failure)?;
    output.ok_or_else(missing)
}
/// 📤️ Publishes one encoded value under the original encode owner.
pub fn encoded<T: RetireOwned, O: RetireOwned>(owner: &mut NativeSnapshotEncodeOwner<'_, '_>, body: impl FnOnce(&mut Option<T>, &mut NativeEncodeControl<'_>, &mut NativeSnapshotBodyWallet) -> Result<O, ValueError>) -> Result<O, ValueError> {
    let (mut output, mut body) = (None, Some(body));
    owner
        .receive::<T, O, NativeSnapshotFailure>(&mut output, &mut |slot, result, _, _, native, wallet| {
            native_snapshot_turn(|work| {
                work.performed();
                *result = Some(body.take().expect("native receiving body runs once")(slot, native, wallet)?);
                Ok(())
            })
        })
        .result
        .map_err(failure)?;
    output.ok_or_else(missing)
}
/// 🪆️ Runs one route body as a nested receiving frame; the body's own refusal returns with its diagnostics.
pub fn nested<T: RetireOwned, O: RetireOwned>(control: &mut IoRunControl<'_, '_>, body: impl FnOnce(&mut Option<T>, &mut IoRunControl<'_, '_>, &mut NativeSnapshotWork) -> Result<O, IoError>) -> Result<O, IoError> {
    let (mut output, mut body, mut refusal) = (None, Some(body), None);
    control
        .receive_nested::<T, O, NativeSnapshotFailure>(&mut output, &mut |slot, result, _, _, original| {
            native_snapshot_turn(|work| {
                work.performed();
                match body.take().expect("native receiving body runs once")(slot, original, work) {
                    Ok(value) => *result = Some(value),
                    Err(error) => refusal = Some(error),
                }
                Ok(())
            })
        })
        .result
        
        .map_err(refused)?;
    match refusal {
        Some(error) => Err(error),
        None => output.ok_or_else(|| IoError::from_value_error(missing())),
    }
}
/// 🪆️ Runs one value-producing route body as a nested receiving frame.
pub fn nested_value<T: RetireOwned, O: RetireOwned>(control: &mut IoRunControl<'_, '_>, body: impl FnOnce(&mut Option<T>, &mut IoRunControl<'_, '_>) -> Result<O, ValueError>) -> Result<O, IoError> {
    nested(control, |slot, original, _| body(slot, original).map_err(IoError::from_value_error))
}
/// 🧾️ Observes one delegated native turn; the child commits its own work to the original native ledger.
pub fn settled<T>(turn: RetainedWorkOutcome<T, NativeSnapshotFailure>) -> Result<T, ValueError> {
    turn.result.map_err(failure)
}
/// 🪆️ Receives one nested value under the original decode owner.
pub fn decoded_nested<'c, T: RetireOwned, O: RetireOwned>(owner: &mut NativeSnapshotDecodeOwner<'_, 'c>, body: impl FnOnce(&mut Option<T>, &mut NativeSnapshotDecodeOwner<'_, 'c>) -> Result<O, ValueError>) -> Result<O, ValueError> {
    let (mut output, mut body) = (None, Some(body));
    owner
        .receive_nested::<T, O, NativeSnapshotFailure>(&mut output, &mut |slot, result, _, _, original| {
            native_snapshot_turn(|work| {
                work.performed();
                *result = Some(body.take().expect("native receiving body runs once")(slot, original)?);
                Ok(())
            })
        })
        .result
        .map_err(failure)?;
    output.ok_or_else(missing)
}
/// 🌳️ Runs one staged body under the original decode owner.
pub fn scoped<'c, O>(owner: &mut NativeSnapshotDecodeOwner<'_, 'c>, body: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, 'c>) -> Result<O, ValueError>) -> Result<O, ValueError> {
    settled(owner.scoped_stage(|owner| {
        native_snapshot_turn(|work| {
            work.performed();
            Ok(body(owner)?)
        })
    }))
}
