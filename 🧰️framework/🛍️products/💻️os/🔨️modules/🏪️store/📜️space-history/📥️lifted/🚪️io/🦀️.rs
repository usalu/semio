//! 📥️ Items of `serialization` that belong to `os-store-space-history` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🚪️io/🦀️.rs`.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::ValueError;
use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
/// 🪶️ A snapshot's handwritten relational model, independently interpretable through SQLite. The core hooks live here; the derived codec builder is `ArtifactSqliteSnapshotOperations` in the codec operations layer.
pub trait ArtifactSqliteSnapshotHost: pack::snapshot::ArtifactSqliteSnapshot {
    /// 🫴️ Receives original SQL row ownership before any projection effect can refuse; the default runs the authored projection under the original owner, whose control carries the operation workspace recipient the codec installed.
    fn to_sqlite_database_receiving(&self,control:&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<semio_framework_io_sqlite_snapshot::SqliteDatabase,ValueError>{crate::sqlite_snapshot_workspace::project(self,control,owner)}
    /// 🪆️ Receives original semantic tree ownership before any reconstruction effect can refuse; the default runs the authored reconstruction under the original owner, whose control carries the operation workspace recipient the codec installed.
    fn from_sqlite_database_receiving(database:&semio_framework_io_sqlite_snapshot::SqliteDatabase,control:&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError> {crate::sqlite_snapshot_workspace::reconstruct(database,control,owner)}

    /// ♻️ Retires a snapshot through its declared owner after an erased conversion.
    fn retire_sqlite_snapshot(self) {
        drop(self);
    }

    /// 🛬️ Materializes native snapshot fields under the caller's limits and cancellation control.
    fn decode_sqlite_snapshot_native(_payload: &semio_framework_io_schema::IoPayload, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        native_owner.native().checkpoint()?;
        control.checkpoint(semio_framework_io_sqlite_snapshot::SqliteSnapshotPhase::DecodeNative, 0, 1)?;
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "snapshot owner has no controlled native decoding implementation"))
    }

    /// 🛫️ Emits native state through an owner-declared bounded and cancellable encoder.
    fn encode_sqlite_snapshot_native(&self, _encoding: semio_framework_io_sqlite_snapshot::SnapshotEncoding, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<semio_framework_io_schema::IoPayload, ValueError> {
        native_owner.native().checkpoint()?;
        control.checkpoint(semio_framework_io_sqlite_snapshot::SqliteSnapshotPhase::EncodeNative, 0, 1)?;
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "snapshot owner has no controlled native encoding implementation"))
    }

    /// 🚧️ Checks the owner's native encoding expansion before printing or packing any fields.
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: semio_framework_io_sqlite_snapshot::SnapshotEncoding, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        control.checkpoint(semio_framework_io_sqlite_snapshot::SqliteSnapshotPhase::EncodeNative, 0, 1)?;
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "snapshot owner has no bounded native encoding preflight"))
    }

    /// 🛡️ Validates the exact owned subset without lowering fields into a native wire format.
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _database: &semio_framework_io_sqlite_snapshot::SqliteDatabase, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> semio_framework_io_schema::IoResult<()> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        control.checkpoint(semio_framework_io_sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(semio_framework_io_schema::IoError::from_value_error)?;
        if dialect.subset == "*" {
            return Ok(semio_framework_io_schema::IoOutcome::clean(()));
        }
        Err(semio_framework_io_schema::IoError::from_value_error(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("owned snapshot subset {} has no semantic validator", dialect.to_coordinate()))))
    }

    /// 🛂️ Validates the authoritative subset with original decode receiving authority; the default applies the authored subset policy.
    fn validate_sqlite_snapshot_subset_decoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&semio_framework_io_sqlite_snapshot::SqliteDatabase,control:&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_io_schema::IoResult<()>{crate::sqlite_snapshot_workspace::validate_decoding(self,dialect,database,control,native)}
    /// 🛃️ Validates the authoritative subset with original encode receiving authority; the default applies the authored subset policy.
    fn validate_sqlite_snapshot_subset_encoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&semio_framework_io_sqlite_snapshot::SqliteDatabase,control:&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotEncodeOwner<'_,'_>)->semio_framework_io_schema::IoResult<()>{crate::sqlite_snapshot_workspace::validate_encoding(self,dialect,database,control,native)}
}
