//! 🫙️ Original schema, table census and partial rows stay under one caller-owned projection prefix.
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,artifact::{ProjectionStorage,RowWriter,project_owned}};
use semio_framework_value::ValueError;
#[path="🪪️names/🦀️.rs"]mod names;
pub(crate)use names::NameIndex;

/// 🧮️ Traverses the same authored scalar rows twice while preserving every original auxiliary allocation.
pub(crate)fn project_rows_owned(sql:&str,control:&mut SqliteSnapshotControl<'_>,mut visit:impl FnMut(&mut RowWriter<'_,'_>)->Result<(),ValueError>)->Result<SqliteDatabase,ValueError>{
 project_owned(control,ProjectionStorage::empty,|storage,control|{
  let mut census=RowWriter::census(storage,sql,control)?;visit(&mut census)?;census.finish_census()?;
  let mut output=RowWriter::new_into(storage,control)?;visit(&mut output)?;output.finish_into()
 },ProjectionStorage::take_database)
}
