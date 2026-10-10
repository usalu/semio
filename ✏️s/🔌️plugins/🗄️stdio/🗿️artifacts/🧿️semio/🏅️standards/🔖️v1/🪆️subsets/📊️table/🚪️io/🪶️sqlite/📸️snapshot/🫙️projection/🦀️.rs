//! 📊️ Original table projection retains shared scalar value pages and every partial SQL field.
use super::{SemioTableSnapshot,number,kind,VALUES};
use crate::standards::v1::subsets::value::io::sqlite::snapshot::projection::{TreeScratch,visit_tree};
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,RowWriter,ProjectionStorage,project_owned,owned_workspace}};
use semio_framework_value::ValueError;

/// 🫳️ Shares one original paged value traversal across every cell in both authored passes.
pub(crate)fn visit(snapshot:&SemioTableSnapshot,tree:&mut TreeScratch,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("semio_table_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,column)in snapshot.columns.iter().enumerate(){out.insert("semio_table_column",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&column.name),Cell::Text(kind(column.kind))])?;}
 for(ordinal,row)in snapshot.rows.iter().enumerate(){let id=out.insert("semio_table_row",&[Cell::Integer(1),Cell::Integer(number(ordinal)?)])?;for(column,value)in row.cells.iter().enumerate(){let value=visit_tree(value,VALUES,None,tree,out)?;out.insert("semio_table_cell",&[Cell::Integer(id),Cell::Integer(number(column)?),Cell::Integer(value)])?;}}
 Ok(())
}

#[derive(semio_framework_value::RetireOwned)]
struct Prefix{projection:ProjectionStorage,tree:TreeScratch}

/// 🧮️ Admits the actual table, cell and scalar traversal owners before constructing any original output.
pub(super)fn project(snapshot:&SemioTableSnapshot,sql:&str,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 project_owned(control,||Prefix{projection:ProjectionStorage::empty(),tree:TreeScratch::empty()},|prefix,control|{
  let mut census=RowWriter::census(&mut prefix.projection,sql,control)?;visit(snapshot,&mut prefix.tree,&mut census)?;census.finish_census()?;
  let mut output=RowWriter::new_into(&mut prefix.projection,control)?;visit(snapshot,&mut prefix.tree,&mut output)?;output.finish_into()
 },|prefix|prefix.projection.take_database())
}

/// 🎟️ Keeps all original scalar pages through native admission in the caller's actual phase.
pub(super)fn admit(snapshot:&SemioTableSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 owned_workspace(control,phase,TreeScratch::empty,|tree,control|{
  let mut writer=RowWriter::borrowed(control,phase)?;visit(snapshot,tree,&mut writer)?;writer.finish_borrowed()
 },|_|())
}
