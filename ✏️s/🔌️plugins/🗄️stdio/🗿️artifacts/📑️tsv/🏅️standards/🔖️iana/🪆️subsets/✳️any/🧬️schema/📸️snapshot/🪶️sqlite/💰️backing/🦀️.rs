//! 💰️ Authored tabular projection and typed reconstruction under cumulative admission.
use super::{TsvSnapshot,LineEnding};
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
pub(super) fn forecast(snapshot:&TsvSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize>{
 control.checkpoint(phase,0,snapshot.records.len())?;let mut domain=snapshot.records.len();
 for(index,record)in snapshot.records.iter().enumerate(){if index>0&&index%256==0{control.checkpoint(phase,0,snapshot.records.len())?;}domain=domain.checked_add(record.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"tabular row count overflow"))?;}
 control.checkpoint(phase,0,domain)?;let total=domain.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"tabular row count overflow"))?;control.check_rows(total)?;
 let mut bytes=snapshot.schema.len().checked_add(16+match snapshot.line_ending{LineEnding::Lf=>2,LineEnding::Crlf=>4}).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"tabular value extent overflow"))?;
 for(index,record)in snapshot.records.iter().enumerate(){if index%256==0{control.checkpoint(phase,0,domain)?;}bytes=bytes.checked_add(24).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"tabular value extent overflow"))?;
  for(index,field)in record.iter().enumerate(){if index%256==0{control.checkpoint(phase,0,domain)?;}bytes=bytes.checked_add(24).and_then(|bytes|bytes.checked_add(field.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"tabular value extent overflow"))?;}
 }control.check_value_bytes(bytes)?;control.checkpoint(phase,0,domain)?;Ok(total)
}
/// 📤️ Owns every authored row/cell through the shared paid projection frontier.
pub(super) fn project(snapshot:&TsvSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{
 let total=forecast(snapshot,control,SqliteSnapshotPhase::ProjectSnapshot)?;let mut output=Projection::new(SQL,control)?;output.insert("tsv_document",&[Cell::Text(&snapshot.schema),Cell::Integer(i64::from(snapshot.trailing_newline)),Cell::Text(match snapshot.line_ending{LineEnding::Lf=>"lf",LineEnding::Crlf=>"crlf"})])?;
 for(index,record)in snapshot.records.iter().enumerate(){let record_id=output.insert("tsv_record",&[Cell::Integer(1),Cell::Integer(ordinal(index)?)])?;for(index,field)in record.iter().enumerate(){output.insert("tsv_field",&[Cell::Integer(record_id),Cell::Integer(ordinal(index)?),Cell::Text(field)])?;}output.checkpoint_total(total)?;}output.checkpoint_total(total)?;output.finish()
}
/// 📥️ Pays exact borrowed indexes and typed owner collections before literal field binding.
pub(super) fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<TsvSnapshot>{
 validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let document=database.table("tsv_document")?.single_row()?;if document.rowid!=1||document.integer(0)?!=1{return Err(invalid("tabular document identity must be 1"))}
 Reconstruction::new(control)?.scalar()?;let trailing_newline=match document.integer(2)?{0=>false,1=>true,_=>return Err(invalid("tabular document flag must be boolean"))};let line_ending=match document.text(3)?{"lf"=>LineEnding::Lf,"crlf"=>LineEnding::Crlf,_=>return Err(invalid("unknown TSV line ending"))};Reconstruction::new(control)?.scalar()?;
 let index=rows::Rows::new(database.table("tsv_record")?,database.table("tsv_field")?,4,control)?;let schema=owner(Reconstruction::new(control)?.text(document.text(1)?)?);let mut records=collection::<Vec<String>>(index.records.len(),control)?;
 for record in &index.records{let children=index.fields(record.rowid,control)?;let mut fields=collection::<String>(children.len(),control)?;for row in children{fields.as_mut().push(Reconstruction::new(control)?.text(row.text(3)?)?);}records.as_mut().push(fields.take());Reconstruction::new(control)?.checkpoint()?;}
 let snapshot=owner(TsvSnapshot{schema:schema.take(),records:records.take(),trailing_newline,line_ending});Reconstruction::new(control)?.checkpoint()?;Ok(snapshot.take())
}
