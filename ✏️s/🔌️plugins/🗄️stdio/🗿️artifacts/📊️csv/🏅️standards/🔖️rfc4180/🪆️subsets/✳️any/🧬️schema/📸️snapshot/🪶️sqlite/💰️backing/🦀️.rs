//! 💰️ Authored tabular projection and typed reconstruction under cumulative admission.
use super::{CsvSnapshot,CsvRecord,CsvField};
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use semio_framework_os_kernel::sqlite_snapshot::SqliteDatabase;
use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl;
use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
use semio_framework_os_kernel::sqlite_snapshot::ValueError;
use semio_framework_os_kernel::sqlite_snapshot::ValueRefusalKind;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Projection;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Reconstruction;
use semio_framework_os_kernel::sqlite_snapshot::transfer::reserve;
use semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled;
use semio_framework_value::FromValue;
#[path="🔍️rows/🦀️.rs"] mod rows;
const SQL:&str=include_str!("../🗄️.sql");
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn ordinal(value:usize)->Result<i64>{i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"tabular ordinal overflow"))}
fn owner<T:FromValue>(value:T)->DecodedFieldOwner<T>{semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,T::retire_decoded)}
fn collection<T:FromValue>(count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<DecodedFieldOwner<Vec<T>>>{Ok(owner(reserve(count,control)?))}
/// 📏️ Counts the actual authored row/value workload before the first owned entity.
pub(super) fn forecast(snapshot:&CsvSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize>{
 control.checkpoint(phase,0,snapshot.records.len())?;let mut domain=snapshot.records.len();
 for(index,record)in snapshot.records.iter().enumerate(){if index>0&&index%256==0{control.checkpoint(phase,0,snapshot.records.len())?;}domain=domain.checked_add(record.fields.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"tabular row count overflow"))?;}
 control.checkpoint(phase,0,domain)?;let total=domain.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"tabular row count overflow"))?;control.check_rows(total)?;
 let mut bytes=snapshot.schema.len().checked_add(16).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"tabular value extent overflow"))?;
 for(index,record)in snapshot.records.iter().enumerate(){if index%256==0{control.checkpoint(phase,0,domain)?;}bytes=bytes.checked_add(24).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"tabular value extent overflow"))?;
  for(index,field)in record.fields.iter().enumerate(){if index%256==0{control.checkpoint(phase,0,domain)?;}bytes=bytes.checked_add(32).and_then(|bytes|bytes.checked_add(field.value.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"tabular value extent overflow"))?;}
 }control.check_value_bytes(bytes)?;control.checkpoint(phase,0,domain)?;Ok(total)
}
/// 📤️ Owns every authored row/cell through the shared paid projection frontier.
pub(super) fn project(snapshot:&CsvSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{
 let total=forecast(snapshot,control,SqliteSnapshotPhase::ProjectSnapshot)?;let mut output=Projection::new(SQL,control)?;output.insert("csv_document",&[Cell::Text(&snapshot.schema),Cell::Integer(i64::from(snapshot.has_header))])?;
 for(index,record)in snapshot.records.iter().enumerate(){let record_id=output.insert("csv_record",&[Cell::Integer(1),Cell::Integer(ordinal(index)?)])?;for(index,field)in record.fields.iter().enumerate(){output.insert("csv_field",&[Cell::Integer(record_id),Cell::Integer(ordinal(index)?),Cell::Text(&field.value),Cell::Integer(i64::from(field.quoted))])?;}output.checkpoint_total(total)?;}output.checkpoint_total(total)?;output.finish()
}
/// 📥️ Pays exact borrowed indexes and typed owner collections before literal field binding.
pub(super) fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<CsvSnapshot>{
 validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let document=database.table("csv_document")?.single_row()?;if document.rowid!=1||document.integer(0)?!=1{return Err(invalid("tabular document identity must be 1"))}
 Reconstruction::new(control)?.scalar()?;let has_header=match document.integer(2)?{0=>false,1=>true,_=>return Err(invalid("tabular document flag must be boolean"))};
 let index=rows::Rows::new(database.table("csv_record")?,database.table("csv_field")?,5,control)?;let schema=owner(Reconstruction::new(control)?.text(document.text(1)?)?);let mut records=collection::<CsvRecord>(index.records.len(),control)?;
 for record in &index.records{let children=index.fields(record.rowid,control)?;let mut fields=collection::<CsvField>(children.len(),control)?;for row in children{Reconstruction::new(control)?.scalar()?;let quoted=match row.integer(4)?{0=>false,1=>true,_=>return Err(invalid("CSV field quote flag must be boolean"))};fields.as_mut().push(CsvField{value:Reconstruction::new(control)?.text(row.text(3)?)?,quoted});}records.as_mut().push(CsvRecord{fields:fields.take()});Reconstruction::new(control)?.checkpoint()?;}
 let snapshot=owner(CsvSnapshot{schema:schema.take(),records:records.take(),has_header});Reconstruction::new(control)?.checkpoint()?;Ok(snapshot.take())
}
