use super::{LineEnding, TxtSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};

impl ArtifactSqliteSnapshot for TxtSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.lines.len())?;
        control.check_rows(self.lines.len().checked_add(1).ok_or("text entity count overflow")?)?;
        let line_ending = match self.line_ending { LineEnding::Lf => "lf", LineEnding::CrLf => "crlf" };
        let document_bytes = self.schema.len().checked_add(16).and_then(|count| count.checked_add(line_ending.len())).ok_or("text value size overflow")?;
        let mut value_bytes = document_bytes;
        for (ordinal, line) in self.lines.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.lines.len())?; }
            value_bytes = value_bytes.checked_add(line.len()).and_then(|count| count.checked_add(24)).ok_or("text value size overflow")?;
        }
        control.check_value_bytes(value_bytes)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        database.table_mut("text_document")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Text(self.schema.clone()), SqliteValue::Integer(i64::from(self.trailing_newline)), SqliteValue::Text(line_ending.into())] });
        let lines = database.table_mut("text_line")?;
        for (ordinal, content) in self.lines.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, ordinal, self.lines.len())?; }
            let ordinal = i64::try_from(ordinal).map_err(|error| error.to_string())?;
            let rowid = ordinal.checked_add(1).ok_or("text line identifier overflow")?;
            lines.rows.push(SqliteRow { rowid, values: vec![SqliteValue::Integer(rowid), SqliteValue::Integer(1), SqliteValue::Integer(ordinal), SqliteValue::Text(content.clone())] });
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.lines.len(), self.lines.len())?;
        Ok(database)
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let total = database.table("text_line")?.rows.len();
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?;
        control.check_rows(total.checked_add(1).ok_or("text entity count overflow")?)?;
        let document = database.table("text_document")?.single_row()?;
        if document.integer(0)? != 1 { return Err("text document identifier must be 1".into()); }
        let trailing_newline = match document.integer(2)? { 0 => false, 1 => true, _ => return Err("text trailing newline must be boolean".into()) };
        let line_ending = match document.text(3)? { "lf" => LineEnding::Lf, "crlf" => LineEnding::CrLf, _ => return Err("unknown text line ending".into()) };
        let mut lines = Vec::new();
        for row in database.table("text_line")?.ordered_rows(2)? {
            if lines.len() % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, lines.len(), total)?; }
            if row.integer(1)? != 1 { return Err("text line has an unknown document".into()); }
            lines.push(row.text(3)?.into());
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(Self { schema: document.text(1)?.into(), lines, trailing_newline, line_ending })
    }
}
