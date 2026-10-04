//! 🪶️ Explicit TXT entities and controlled external UTF-8 carriers.
use super::{LineEnding, TxtSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{Cell, Projection, Reconstruction, ordered_row_refs}, validate_sqlite_database_schema, SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase}, ArtifactSqliteSnapshot};
use semio_framework_value::{native_decoding::{NativeDecodeControl, NativeDecodeProgress}, native_encoding::{NativeEncodeControl, NativeEncodeProgress}};

fn native_length(snapshot: &TxtSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<usize, ValueError> {
    control.checkpoint(SqliteSnapshotPhase::EncodeNative, 0, snapshot.lines.len())?;
    control.check_rows(snapshot.lines.len().checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "text entity count overflow"))?)?;
    let separators = snapshot.lines.len().saturating_sub(1).checked_add(usize::from(snapshot.trailing_newline)).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "text separator count overflow"))?;
    let mut bytes = separators.checked_mul(snapshot.line_ending.as_str().len()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "text body length overflow"))?;
    for (ordinal, line) in snapshot.lines.iter().enumerate() {
        bytes = bytes.checked_add(line.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "text body length overflow"))?;
        if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::EncodeNative, ordinal, snapshot.lines.len())?; }
    }
    Ok(bytes)
}

fn count_lines(content: &str, ending: LineEnding, native: &mut NativeDecodeControl<'_>) -> Result<usize, ValueError> {
    native.begin_stage(content.len())?;
    let mut count = 1usize;
    let mut previous = 0u8;
    for chunk in content.as_bytes().chunks(65536) {
        for &byte in chunk {
            if byte == b'\n' && (ending == LineEnding::Lf || previous == b'\r') { count = count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "text line count overflow"))?; }
            previous = byte;
        }
        native.advance(chunk.len())?;
    }
    Ok(count)
}

use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};
impl ArtifactSqliteSnapshot for TxtSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        (|| -> Result<_, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.lines.len())?;
        let rows = self.lines.len().checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "text entity count overflow"))?;
        control.check_rows(rows)?;
        let ending = match self.line_ending { LineEnding::Lf => "lf", LineEnding::CrLf => "crlf" };
        let mut bytes = self.schema.len().checked_add(16 + ending.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "text value size overflow"))?;
        for (ordinal, line) in self.lines.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, ordinal, self.lines.len())?; }
            bytes = bytes.checked_add(24).and_then(|count| count.checked_add(line.len())).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "text value size overflow"))?;
        }
        control.check_value_bytes(bytes)?;
        let mut projection = Projection::new(Self::SQLITE_SCHEMA, control)?;
        let root = projection.insert("text_document", &[Cell::Text(&self.schema), Cell::Integer(i64::from(self.trailing_newline)), Cell::Text(ending)])?;
        for (ordinal, line) in self.lines.iter().enumerate() {
            projection.insert("text_line", &[Cell::Integer(root), Cell::Integer(i64::try_from(ordinal).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "text ordinal overflow"))?), Cell::Text(line)])?;
            if ordinal % 256 == 0 { projection.checkpoint_total(rows)?; }
        }
        projection.checkpoint_total(rows)?;
        projection.finish()
    
        })()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        (|| -> Result<_, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("text_document")?.single_row()?;
        let root = document.integer(0)?;
        if root <= 0 || document.rowid != root || document.values.len() != 4 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "invalid text document identity or width")); }
        let trailing_newline = match document.integer(2)? { 0 => false, 1 => true, _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "text trailing newline must be boolean")) };
        let line_ending = match document.text(3)? { "lf" => LineEnding::Lf, "crlf" => LineEnding::CrLf, _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown text line ending")) };
        let rows = ordered_row_refs(database.table("text_line")?, 2, control)?;
        let mut restore = Reconstruction::new(control)?;
        let schema = restore.text(document.text(1)?)?;
        let mut lines = Vec::new();
        lines.try_reserve_exact(rows.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "text line allocation failed"))?;
        for row in rows {
            if row.rowid <= 0 || row.integer(0)? != row.rowid || row.integer(1)? != root || row.values.len() != 4 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "invalid text line identity, relationship or width")); }
            lines.push(restore.text(row.text(3)?)?);
            restore.checkpoint()?;
        }
        restore.checkpoint()?;
        Ok(Self { schema, lines, trailing_newline, line_ending })
    
        })()
    }

    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        let limits = control.limits();
        control.checkpoint(SqliteSnapshotPhase::DecodeNative, 0, 0)?;
        let size = match payload { store::io_schema::IoPayload::Binary(bytes) => bytes.len(), store::io_schema::IoPayload::Text(text) => text.len() };
        if size > limits.max_file_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native text input exceeds file byte limit")); }
        let mut progress = |event: NativeDecodeProgress| control.checkpoint(SqliteSnapshotPhase::DecodeNative, event.completed, event.total).is_ok();
        let mut native = NativeDecodeControl::new(limits.max_value_bytes, &mut progress);
        let body = match payload {
            store::io_schema::IoPayload::Binary(bytes) => {
                let bytes = store::semio_format::unwrap_binary_controlled(bytes, "stdio.txt", store::semio_format::Component::Pack, 1, &mut native).map_err(store::semio_format::SemioError::into_value_error)?;
                native.borrow_text(bytes)?
            }
            store::io_schema::IoPayload::Text(text) => text.as_str(),
        };
        native.begin_stage(body.len())?;
        let mut ending = LineEnding::Lf;
        let mut previous = 0u8;
        for chunk in body.as_bytes().chunks(65536) {
            for &byte in chunk { if previous == b'\r' && byte == b'\n' { ending = LineEnding::CrLf; } previous = byte; }
            native.advance(chunk.len())?;
        }
        let sep = ending.as_str();
        let trailing_newline = !body.is_empty() && body.ends_with(sep);
        let content = if trailing_newline { &body[..body.len() - sep.len()] } else { body };
        let count = if body.is_empty() { 0 } else { count_lines(content, ending, &mut native)? };
        if count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"text entity count overflow"))? > limits.max_rows { return Err(ValueError::new(ValueRefusalKind::WorkLimit,"native text exceeds row limit")); }
        let mut lines = native.allocate_vec::<String>(count)?;
        let schema = native.copy_text(crate::STDIO_TXT_DOCUMENT_SCHEMA)?;
        if !body.is_empty() {
            for line in content.split(sep) { lines.push(native.copy_text(line)?); }
        }
        native.checkpoint()?;
        Ok(Self { schema, lines, trailing_newline, line_ending: ending })
    }

    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
        let body_size = native_length(self, control)?;
        let limits = control.limits();
        let prefix = match encoding { SnapshotEncoding::Binary => store::semio_format::declared_envelope_prefix_len("stdio.txt", store::semio_format::Component::Pack, 1)?, SnapshotEncoding::Text => 0 };
        if body_size.checked_add(prefix).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"text file size overflow"))? > limits.max_file_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native text output exceeds file byte limit")); }
        let mut progress = |event: NativeEncodeProgress| control.checkpoint(SqliteSnapshotPhase::EncodeNative, event.completed, event.total).is_ok();
        let mut native = NativeEncodeControl::new(limits.max_value_bytes, &mut progress);
        native.begin_stage(body_size)?;
        let mut body = native.allocate_vec::<u8>(body_size)?;
        let sep = self.line_ending.as_str().as_bytes();
        for (ordinal, line) in self.lines.iter().enumerate() {
            if ordinal != 0 { body.extend_from_slice(sep); native.advance(sep.len())?; }
            for chunk in line.as_bytes().chunks(65536) { body.extend_from_slice(chunk); native.advance(chunk.len())?; }
        }
        if self.trailing_newline { body.extend_from_slice(sep); native.advance(sep.len())?; }
        native.checkpoint()?;
        match encoding {
            SnapshotEncoding::Binary => Ok(store::io_schema::IoPayload::Binary(store::semio_format::wrap_binary_controlled("stdio.txt", store::semio_format::Component::Pack, 1, &body, &mut native)?)),
            SnapshotEncoding::Text => Ok(store::io_schema::IoPayload::Text(String::from_utf8(body).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"invalid native text output"))?)),
        }
    }

    fn preflight_sqlite_snapshot_encoding(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        let size = native_length(self, control)?;
        let prefix = match encoding { SnapshotEncoding::Binary => store::semio_format::declared_envelope_prefix_len("stdio.txt", store::semio_format::Component::Pack, 1)?, SnapshotEncoding::Text => 0 };
        let physical = size.checked_add(prefix).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"text file size overflow"))?;
        if physical > control.limits().max_file_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native text output exceeds file byte limit")); }
        control.check_value_bytes(if encoding == SnapshotEncoding::Binary { size.checked_add(physical).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native text ownership overflow"))? } else { size })
    }

    fn validate_sqlite_snapshot_subset(&self, dialect: &store::io_schema::ArtifactDialect, database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        use store::io_schema::IoError;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(IoError::from_value_error)?;
        if dialect.artifact_kind != "s.stdio.txt" || dialect.standard != "utf-8" || dialect.subset != "*" { return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"unsupported owned TXT SQLite dialect"))); }
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(IoError::from_value_error)?;
        control.check_database(database, SqliteSnapshotPhase::ProjectSnapshot).map_err(IoError::from_value_error)?;
        Ok(store::io_schema::IoOutcome::clean(()))
    }
}
