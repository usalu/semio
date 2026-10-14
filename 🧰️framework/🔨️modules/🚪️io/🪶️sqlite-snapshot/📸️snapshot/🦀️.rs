//! 🪶️ Product-neutral authored relational snapshot model.
use semio_framework_value::ValueError;
pub trait ArtifactSqliteSnapshot: Sized {
    const SQLITE_SCHEMA: &'static str;
    fn to_sqlite_database(&self, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<semio_framework_io_sqlite_snapshot::SqliteDatabase, ValueError>;
    fn from_sqlite_database(database: &semio_framework_io_sqlite_snapshot::SqliteDatabase, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, ValueError>;

}
