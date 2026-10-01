use super::{LineEnding, TsvSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};

impl ArtifactSqliteSnapshot for TsvSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.records.len())?;
        let mut total = self.records.len();
        for (ordinal, record) in self.records.iter().enumerate() {
            if ordinal > 0 && ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.records.len())?; }
            total = total.checked_add(record.len()).ok_or("TSV entity count overflow")?;
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?;
        control.check_rows(total.checked_add(1).ok_or("TSV entity count overflow")?)?;
        let line_ending = match self.line_ending { LineEnding::Lf => "lf", LineEnding::Crlf => "crlf" };
        let mut value_bytes = self.schema.len().checked_add(16).and_then(|count| count.checked_add(line_ending.len())).ok_or("TSV value size overflow")?;
        for (index, record) in self.records.iter().enumerate() {
            if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?; }
            value_bytes = value_bytes.checked_add(24).ok_or("TSV value size overflow")?;
            for (ordinal, field) in record.iter().enumerate() {
                if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?; }
                value_bytes = value_bytes.checked_add(24).and_then(|count| count.checked_add(field.len())).ok_or("TSV value size overflow")?;
            }
        }
        control.check_value_bytes(value_bytes)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        database.table_mut("tsv_document")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Text(self.schema.clone()), SqliteValue::Integer(i64::from(self.trailing_newline)), SqliteValue::Text(line_ending.into())] });
        let mut record_rows = Vec::new();
        let mut field_rows = Vec::new();
        for (ordinal, record) in self.records.iter().enumerate() {
            if (record_rows.len() + field_rows.len()) % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, record_rows.len() + field_rows.len(), total)?; }
            let ordinal = i64::try_from(ordinal).map_err(|error| error.to_string())?;
            let record_id = ordinal.checked_add(1).ok_or("TSV record identifier overflow")?;
            record_rows.push(SqliteRow { rowid: record_id, values: vec![SqliteValue::Integer(record_id), SqliteValue::Integer(1), SqliteValue::Integer(ordinal)] });
            for (field_ordinal, field) in record.iter().enumerate() {
                if (record_rows.len() + field_rows.len()) % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, record_rows.len() + field_rows.len(), total)?; }
                let field_ordinal = i64::try_from(field_ordinal).map_err(|error| error.to_string())?;
                let field_id = i64::try_from(field_rows.len()).map_err(|error| error.to_string())?.checked_add(1).ok_or("TSV field identifier overflow")?;
                field_rows.push(SqliteRow { rowid: field_id, values: vec![SqliteValue::Integer(field_id), SqliteValue::Integer(record_id), SqliteValue::Integer(field_ordinal), SqliteValue::Text(field.clone())] });
            }
        }
        database.table_mut("tsv_record")?.rows = record_rows;
        database.table_mut("tsv_field")?.rows = field_rows;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, total, total)?;
        Ok(database)
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let total = database.table("tsv_record")?.rows.len().checked_add(database.table("tsv_field")?.rows.len()).ok_or("TSV entity count overflow")?;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?;
        control.check_rows(total.checked_add(1).ok_or("TSV entity count overflow")?)?;
        let document = database.table("tsv_document")?.single_row()?;
        if document.integer(0)? != 1 { return Err("TSV document identifier must be 1".into()); }
        let trailing_newline = match document.integer(2)? { 0 => false, 1 => true, _ => return Err("TSV trailing newline must be boolean".into()) };
        let line_ending = match document.text(3)? { "lf" => LineEnding::Lf, "crlf" => LineEnding::Crlf, _ => return Err("unknown TSV line ending".into()) };
        let record_table = database.table("tsv_record")?;
        let record_ids: BTreeSet<_> = record_table.rows.iter().map(|row| row.integer(0)).collect::<Result<_, _>>()?;
        let mut fields = BTreeMap::<i64, Vec<(i64, &SqliteRow)>>::new();
        for (index, row) in database.table("tsv_field")?.rows.iter().enumerate() {
            if index % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?; }
            let record_id = row.integer(1)?;
            if !record_ids.contains(&record_id) { return Err("TSV field has an unknown record".into()); }
            fields.entry(record_id).or_default().push((row.integer(2)?, row));
        }
        let mut records = Vec::new();
        let mut completed = 0;
        for row in record_table.ordered_rows(2)? {
            completed += 1;
            if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; }
            if row.integer(1)? != 1 { return Err("TSV record has an unknown document".into()); }
            let mut ordered_fields = fields.remove(&row.integer(0)?).unwrap_or_default();
            ordered_fields.sort_by_key(|(ordinal, _)| *ordinal);
            let mut record_fields = Vec::new();
            for (index, (ordinal, field)) in ordered_fields.into_iter().enumerate() {
                completed += 1;
                if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; }
                if i64::try_from(index).map_err(|error| error.to_string())? != ordinal { return Err("TSV field ordinals must be contiguous".into()); }
                record_fields.push(field.text(3)?.into());
            }
            records.push(record_fields);
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(Self { schema: document.text(1)?.into(), records, trailing_newline, line_ending })
    }
}
