//! 💰️ Authored Binary SQL ownership frontier.
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use super::BinarySnapshot;
use semio_framework_os_kernel::{sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,SqliteValue,artifact::{Cell,Projection,Reconstruction,ordered_row_refs},transfer::reserve,validate_sqlite_database_schema_controlled}};
use semio_framework_value::FromValue;
type Result<T>=std::result::Result<T,ValueError>;
const SQL:&str=include_str!("../🗄️.sql");
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn owner<T:FromValue>(value:T)->DecodedFieldOwner<T>{DecodedFieldOwner::new(value,T::retire_decoded)}

/// 📤️ Pays complete authored document and octet row ownership after semantic preflight.
pub(super) fn project(snapshot:&BinarySnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{
 let total=snapshot.bytes.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"binary entity count overflow"))?;
 control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,total)?;control.check_rows(total)?;
 control.check_value_bytes(snapshot.bytes.len().checked_mul(32).and_then(|bytes|bytes.checked_add(snapshot.schema.len())).and_then(|bytes|bytes.checked_add(8)).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"binary value extent overflow"))?)?;
 let mut output=Projection::new(SQL,control)?;output.insert("binary_document",&[Cell::Text(&snapshot.schema)])?;
 for(index,byte)in snapshot.bytes.iter().enumerate(){let ordinal=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"binary ordinal overflow"))?;output.insert("binary_byte",&[Cell::Integer(1),Cell::Integer(ordinal),Cell::Integer(i64::from(*byte))])?;}
 output.checkpoint_total(total)?;output.finish()
}
/// 📥️ Admits ordered borrowed references and concrete native octets before typed reconstruction.
pub(super) fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<BinarySnapshot>{
 validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let document=database.table("binary_document")?.single_row()?;if document.rowid!=1||document.values.len()!=2||document.integer(0)?!=1{return Err(invalid("binary document identity or width"))}
 let rows=ordered_row_refs(database.table("binary_byte")?,2,control)?;let schema=owner(Reconstruction::new(control)?.text(document.text(1)?)?);let mut bytes=owner(reserve::<u8>(rows.len(),control)?);
 for row in &rows{if row.rowid<=0||row.values.len()!=4||row.integer(0)?!=row.rowid||row.integer(1)?!=1{return Err(invalid("binary octet identity or document ownership"))}let value=u8::try_from(row.integer(3)?).map_err(|_|invalid("binary octet must fit u8"))?;Reconstruction::new(control)?.scalar()?;bytes.as_mut().push(value);}
 let snapshot=owner(BinarySnapshot{schema:schema.take(),bytes:bytes.take()});Reconstruction::new(control)?.checkpoint()?;Ok(snapshot.take())
}
