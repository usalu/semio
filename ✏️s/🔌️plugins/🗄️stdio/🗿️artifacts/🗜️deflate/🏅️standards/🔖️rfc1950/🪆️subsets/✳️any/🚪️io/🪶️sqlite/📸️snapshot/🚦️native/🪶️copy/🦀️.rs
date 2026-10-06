//! 🪶️ Unmounted authored SQLite mapping adds paid literal-field copies after its owning baseline.
use crate::standards::v_rfc1950::subsets::any::schema::snapshot::*;
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{Cell, Projection}, validate_sqlite_database_schema, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

impl ArtifactSqliteSnapshot for DeflateSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("../../🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        if self.compression_method > 15 || self.window_bits > 15 { return Err(invalid("Deflate CMF fields must fit their declared four-bit nibbles")); }
        let mut out = Projection::new(Self::SQLITE_SCHEMA, control)?;
        out.insert("deflate_document", &[Cell::Text(&self.schema), Cell::Integer(i64::from(self.compression_method)), Cell::Integer(i64::from(self.window_bits)), Cell::Integer(i64::from(self.compression_level_hint.to_bits())), self.dict_id.map_or(Cell::Null, |value| Cell::Integer(i64::from(value)))])?;
        for (index, value) in self.payload.iter().enumerate() { out.insert("deflate_payload_byte", &[Cell::Integer(1), Cell::Integer(i64::try_from(index).map_err(|error|invalid(error.to_string()))?), Cell::Integer(i64::from(*value))])?; }
        out.finish()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("deflate_document")?.single_row()?;
        if document.rowid != 1 || document.integer(0)? != 1 { return Err(invalid("Deflate document identifier must be 1")); }
        let hint = u8::try_from(document.integer(4)?).map_err(|error|invalid(error.to_string()))?;
        if !(0..=15).contains(&document.integer(2)?) || !(0..=15).contains(&document.integer(3)?) { return Err(invalid("Deflate CMF fields must fit their declared four-bit nibbles")); }
        if hint > 3 { return Err(invalid("Deflate compression level hint exceeds two bits")); }
        let dict_id = match document.values.get(5) { Some(SqliteValue::Null) => None, Some(SqliteValue::Integer(value)) => Some(u32::try_from(*value).map_err(|error|invalid(error.to_string()))?), _ => return Err(invalid("Deflate dictionary identifier requires INTEGER or NULL")) };
        let mut payload = Vec::new();
        let rows = database.table("deflate_payload_byte")?.ordered_rows(2)?;
        for (index, row) in rows.iter().enumerate() { if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, rows.len())?; } if row.rowid <= 0 || row.integer(0)? != row.rowid || row.integer(1)? != 1 { return Err(invalid("Deflate payload byte has invalid identity or parent")); } payload.push(u8::try_from(row.integer(3)?).map_err(|error|invalid(error.to_string()))?); }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, rows.len(), rows.len())?;
        Ok(Self { schema: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control, document.text(1)?)?, compression_method: u8::try_from(document.integer(2)?).map_err(|error|invalid(error.to_string()))?, window_bits: u8::try_from(document.integer(3)?).map_err(|error|invalid(error.to_string()))?, compression_level_hint: DeflateLevelHint::from_bits(hint), dict_id, payload })
    }
}
