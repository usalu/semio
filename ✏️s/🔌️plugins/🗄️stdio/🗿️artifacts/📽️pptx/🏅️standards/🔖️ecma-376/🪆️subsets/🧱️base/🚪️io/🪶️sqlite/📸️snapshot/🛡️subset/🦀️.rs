//! 🛡️ SQLite admission delegates to the owned profile validator.
use crate::schema::snapshot::PptxSnapshot;
use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};
use semio_framework_diagnostic::Severity;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::subset::check;
pub(crate) fn validate(snapshot: &PptxSnapshot, subset: &str, control: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
    let limits = control.limits();
    let mut callback = |event: semio_framework_value::native_encoding::NativeEncodeProgress| control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, event.completed, event.total).is_ok();
    let mut native = NativeEncodeControl::new(limits.max_value_bytes, &mut callback);
    let diagnostics = check(snapshot, subset, &mut native).map_err(store::io_schema::IoError::from_value_error)?;
    if diagnostics.iter().any(|item| matches!(item.severity, Severity::Error | Severity::Fatal)) {
        Err(store::io_schema::IoError { cause: ValueError::new(ValueRefusalKind::InvalidValue, "PPTX owned snapshot violates its exact profile"), diagnostics })
    } else {
        Ok(store::io_schema::IoOutcome { value: (), diagnostics })
    }
}
