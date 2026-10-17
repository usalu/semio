//! 🪶️ Product-neutral authored relational snapshot model.
use semio_framework_value::ValueError;
pub trait ArtifactSqliteSnapshot: Sized {
    const SQLITE_SCHEMA: &'static str;
    /// 🗄️ The DDL every consumer validates against; an owner whose DDL a bound child contributes at runtime overrides it, every other owner is its const.
    fn sqlite_schema() -> &'static str {
        Self::SQLITE_SCHEMA
    }
    fn to_sqlite_database(&self, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<semio_framework_io_sqlite_snapshot::SqliteDatabase, ValueError>;
    fn from_sqlite_database(database: &semio_framework_io_sqlite_snapshot::SqliteDatabase, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, ValueError>;

}
