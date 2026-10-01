//! 🔤️ Ordered Semio text runs and inline marks as their own relational entities.

use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;

use semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text;
use super::{SemioTextMark, SemioTextMarkKind, SemioTextRun, SemioTextSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use std::collections::{BTreeMap, BTreeSet};

fn integer(value: usize) -> Result<i64, String> { i64::try_from(value).map_err(|error| error.to_string()) }
fn text(value: &str) -> SqliteValue { SqliteValue::Text(value.into()) }
fn add(value: &mut usize, amount: usize) -> Result<(), String> { *value = value.checked_add(amount).ok_or("Semio text size overflow")?; Ok(()) }
fn kind(value: SemioTextMarkKind) -> &'static str { match value { SemioTextMarkKind::Bold => "bold", SemioTextMarkKind::Italic => "italic", SemioTextMarkKind::Code => "code", SemioTextMarkKind::Link => "link" } }
fn identity(row: &SqliteRow, columns: usize) -> Result<(), String> { if row.values.len() != columns || row.integer(0)? != row.rowid { Err("Semio text row identity or column count is invalid".into()) } else { Ok(()) } }

impl ArtifactSqliteSnapshot for SemioTextSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> { let mut bound = Bound::new("", control)?; self.native_fields(&mut bound)?; bound.finish() }

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="text"){return Err(String::from("Semio owned snapshot dialect differs from its dedicated semantic subset").into());}
let row=database.table("semio_text_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("Semio owned document identity differs from projected semantic fields").into());}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
        let mut total = self.runs.len().checked_add(1).ok_or("Semio text row count overflow")?; let mut bytes = self.schema.len().checked_add(8).ok_or("Semio text size overflow")?; control.check_rows(total)?; control.check_value_bytes(bytes)?; let mut checked = 0usize;
        for run in &self.runs {
            add(&mut total, run.marks.len())?; control.check_rows(total)?; add(&mut bytes, 24)?; add(&mut bytes, run.language.len())?; add(&mut bytes, run.content.len())?; control.check_value_bytes(bytes)?;
            for mark in &run.marks { add(&mut bytes, 24 + kind(mark.kind).len())?; add(&mut bytes, mark.href.len())?; control.check_value_bytes(bytes)?; checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?; } }
            checked += 1; if checked % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?; }
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?; let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        database.table_mut("semio_text_document")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), text(&self.schema)] }); let mut completed = 1usize;
        for (ordinal, run) in self.runs.iter().enumerate() {
            let id = integer(ordinal + 1)?; control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, completed, total)?;
            database.table_mut("semio_text_run")?.rows.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), SqliteValue::Integer(1), SqliteValue::Integer(integer(ordinal)?), text(&run.language), text(&run.content)] }); completed += 1;
            for (ordinal, mark) in run.marks.iter().enumerate() { if mark.href.len() > 65536 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, completed, total)?; } let rows = &mut database.table_mut("semio_text_mark")?.rows; let mark_id = integer(rows.len() + 1)?; rows.push(SqliteRow { rowid: mark_id, values: vec![SqliteValue::Integer(mark_id), SqliteValue::Integer(id), SqliteValue::Integer(integer(ordinal)?), text(kind(mark.kind)), text(&mark.href)] }); completed += 1; if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, completed, total)?; } }
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, total, total)?; Ok(database)
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> { Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) }
}

impl SemioTextSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>, declared_schema: &str) -> Result<Self, String> {

        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?; validate_sqlite_database_schema(database, declared_schema, control.limits()).map_err(|error| error.to_string())?;
        let document = database.table("semio_text_document")?.single_row()?; identity(document, 2)?; if document.rowid != 1 { return Err("Semio text document requires identifier 1".into()); }
        let total = database.tables.iter().map(|table| table.rows.len()).sum(); let mut completed = 1usize; let mut ids = BTreeSet::new(); let ordered = database.table("semio_text_run")?.ordered_rows(2)?;
        for row in &ordered { identity(row, 5)?; if row.integer(1)? != 1 || !ids.insert(row.rowid) { return Err("Semio text run document or identity is invalid".into()); } row.text(3)?; row.text(4)?; }
        let mut marks = BTreeMap::<i64, Vec<&SqliteRow>>::new(); let mut mark_ids = BTreeSet::new();
        for row in &database.table("semio_text_mark")?.rows { identity(row, 5)?; let run = row.integer(1)?; if !ids.contains(&run) || !mark_ids.insert(row.rowid) || row.integer(2)? < 0 { return Err("Semio text mark run, identity or ordinal is invalid".into()); } row.text(3)?; row.text(4)?; marks.entry(run).or_default().push(row); completed += 1; if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed.min(total), total)?; } }
        let mut runs = Vec::new(); completed = 1;
        for row in ordered {
            control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; let mut ordered_marks = marks.remove(&row.rowid).unwrap_or_default(); ordered_marks.sort_by_key(|row| row.integer(2).unwrap_or(-1)); let mut native_marks = Vec::new();
            for (ordinal, mark) in ordered_marks.into_iter().enumerate() { if mark.integer(2)? != integer(ordinal)? { return Err("Semio text mark ordinals must be contiguous and zero-based".into()); } let kind = match mark.text(3)? { "bold" => SemioTextMarkKind::Bold, "italic" => SemioTextMarkKind::Italic, "code" => SemioTextMarkKind::Code, "link" => SemioTextMarkKind::Link, _ => return Err("Semio text mark kind is invalid".into()) }; if mark.text(4)?.len() > 65536 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } native_marks.push(SemioTextMark { kind, href: reconstruct_text(control,mark.text(4)?)? }); completed += 1; if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } }
            runs.push(SemioTextRun { language: reconstruct_text(control,row.text(3)?)?, content: reconstruct_text(control,row.text(4)?)?, marks: native_marks }); completed += 1;
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?; Ok(Self { schema: reconstruct_text(control,document.text(1)?)?, runs })
    }
}

impl SemioTextSnapshot {
    /// 📏️ Checks this owner's explicit native fields before allocating an encoding.
    pub fn native_fields(&self, b: &mut Bound<'_, '_>) -> Result<(), String> { b.text(&self.schema)?; b.entities(self.runs.len())?; for run in &self.runs { b.text(&run.language)?; b.text(&run.content)?; b.entities(run.marks.len())?; for mark in &run.marks { b.scalars(1)?; b.text(&mark.href)?; } } Ok(()) }
}
