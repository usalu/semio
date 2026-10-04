//! 🪶️ Unmounted authored SQLite mapping adds paid literal-field copies after its owning baseline.
use super::BinarySnapshot;
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

impl ArtifactSqliteSnapshot for BinarySnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("../../🪶️sqlite/🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.bytes.len())?;
        control.check_rows(self.bytes.len().checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"binary entity count overflow"))?)?;
        control.check_value_bytes(self.bytes.len().checked_mul(32).and_then(|count| count.checked_add(self.schema.len())).and_then(|count| count.checked_add(8)).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"binary value size overflow"))?)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("binary_document")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Text(semio_framework_os_kernel::sqlite_snapshot::artifact::project_text(control, &self.schema)?)] });
        let bytes = database.table_mut("binary_byte")?;
        for (ordinal, value) in self.bytes.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, ordinal, self.bytes.len())?; }
            let ordinal = i64::try_from(ordinal).map_err(|error|invalid(error.to_string()))?;
            let rowid = ordinal.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"binary byte identifier overflow"))?;
            bytes.rows.push(SqliteRow { rowid, values: vec![SqliteValue::Integer(rowid), SqliteValue::Integer(1), SqliteValue::Integer(ordinal), SqliteValue::Integer(i64::from(*value))] });
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.bytes.len(), self.bytes.len())?;
        Ok(database)
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let total = database.table("binary_byte")?.rows.len();
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?;
        control.check_rows(total.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"binary entity count overflow"))?)?;
        let document = database.table("binary_document")?.single_row()?;
        if document.integer(0)? != 1 { return Err(invalid("binary document identifier must be 1")); }
        let mut bytes = Vec::new();
        for row in database.table("binary_byte")?.ordered_rows(2)? {
            if bytes.len() % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, bytes.len(), total)?; }
            if row.integer(1)? != 1 { return Err(invalid("binary byte has an unknown document")); }
            bytes.push(u8::try_from(row.integer(3)?).map_err(|error|invalid(error.to_string()))?);
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(Self { schema: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control, document.text(1)?)?, bytes })
    }
}
