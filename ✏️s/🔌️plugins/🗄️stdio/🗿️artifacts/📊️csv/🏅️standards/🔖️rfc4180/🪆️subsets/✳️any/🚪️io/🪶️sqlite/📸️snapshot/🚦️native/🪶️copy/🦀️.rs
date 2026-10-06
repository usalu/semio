//! 🪶️ Unmounted authored SQLite mapping adds paid literal-field copies after its owning baseline.
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::{CsvField, CsvRecord, CsvSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

impl ArtifactSqliteSnapshot for CsvSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("../../🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.records.len())?;
        let mut total = self.records.len();
        for (ordinal, record) in self.records.iter().enumerate() {
            if ordinal > 0 && ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.records.len())?; }
            total = total.checked_add(record.fields.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV entity count overflow"))?;
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?;
        control.check_rows(total.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV entity count overflow"))?)?;
        let mut value_bytes = self.schema.len().checked_add(16).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV value size overflow"))?;
        for (index, record) in self.records.iter().enumerate() {
            if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?; }
            value_bytes = value_bytes.checked_add(24).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV value size overflow"))?;
            for (ordinal, field) in record.fields.iter().enumerate() {
                if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?; }
                value_bytes = value_bytes.checked_add(32).and_then(|count| count.checked_add(field.value.len())).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV value size overflow"))?;
            }
        }
        control.check_value_bytes(value_bytes)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("csv_document")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Text(semio_framework_os_kernel::sqlite_snapshot::artifact::project_text(control, &self.schema)?), SqliteValue::Integer(i64::from(self.has_header))] });
        let mut record_rows = Vec::new();
        let mut field_rows = Vec::new();
        for (ordinal, record) in self.records.iter().enumerate() {
            if (record_rows.len() + field_rows.len()) % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, record_rows.len() + field_rows.len(), total)?; }
            let ordinal = i64::try_from(ordinal).map_err(|error|invalid(error.to_string()))?;
            let record_id = ordinal.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV record identifier overflow"))?;
            record_rows.push(SqliteRow { rowid: record_id, values: vec![SqliteValue::Integer(record_id), SqliteValue::Integer(1), SqliteValue::Integer(ordinal)] });
            for (field_ordinal, field) in record.fields.iter().enumerate() {
                if (record_rows.len() + field_rows.len()) % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, record_rows.len() + field_rows.len(), total)?; }
                let field_ordinal = i64::try_from(field_ordinal).map_err(|error|invalid(error.to_string()))?;
                let field_id = i64::try_from(field_rows.len()).map_err(|error|invalid(error.to_string()))?.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV field identifier overflow"))?;
                field_rows.push(SqliteRow { rowid: field_id, values: vec![SqliteValue::Integer(field_id), SqliteValue::Integer(record_id), SqliteValue::Integer(field_ordinal), SqliteValue::Text(semio_framework_os_kernel::sqlite_snapshot::artifact::project_text(control, &field.value)?), SqliteValue::Integer(i64::from(field.quoted))] });
            }
        }
        database.table_mut("csv_record")?.rows = record_rows;
        database.table_mut("csv_field")?.rows = field_rows;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, total, total)?;
        Ok(database)
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let total = database.table("csv_record")?.rows.len().checked_add(database.table("csv_field")?.rows.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV entity count overflow"))?;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?;
        control.check_rows(total.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"CSV entity count overflow"))?)?;
        let document = database.table("csv_document")?.single_row()?;
        if document.integer(0)? != 1 { return Err(invalid("CSV document identifier must be 1")); }
        let has_header = match document.integer(2)? { 0 => false, 1 => true, _ => return Err(invalid("CSV header flag must be boolean")) };
        let record_table = database.table("csv_record")?;
        let record_ids: BTreeSet<_> = record_table.rows.iter().map(|row| row.integer(0)).collect::<Result<_, _>>()?;
        let mut fields = BTreeMap::<i64, Vec<(i64, &SqliteRow)>>::new();
        for (index, row) in database.table("csv_field")?.rows.iter().enumerate() {
            if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            let record_id = row.integer(1)?;
            if !record_ids.contains(&record_id) { return Err(invalid("CSV field has an unknown record")); }
            fields.entry(record_id).or_default().push((row.integer(2)?, row));
        }
        let mut records = Vec::new();
        let mut completed = 0;
        for row in record_table.ordered_rows(2)? {
            completed += 1;
            if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; }
            if row.integer(1)? != 1 { return Err(invalid("CSV record has an unknown document")); }
            let mut ordered_fields = fields.remove(&row.integer(0)?).unwrap_or_default();
            ordered_fields.sort_by_key(|(ordinal, _)| *ordinal);
            let mut record_fields = Vec::new();
            for (index, (ordinal, field)) in ordered_fields.into_iter().enumerate() {
                completed += 1;
                if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; }
                if i64::try_from(index).map_err(|error|invalid(error.to_string()))? != ordinal { return Err(invalid("CSV field ordinals must be contiguous")); }
                let quoted = match field.integer(4)? { 0 => false, 1 => true, _ => return Err(invalid("CSV field quote flag must be boolean")) };
                record_fields.push(CsvField { value: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control, field.text(3)?)?, quoted });
            }
            records.push(CsvRecord { fields: record_fields });
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(Self { schema: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control, document.text(1)?)?, has_header, records })
    }
}
