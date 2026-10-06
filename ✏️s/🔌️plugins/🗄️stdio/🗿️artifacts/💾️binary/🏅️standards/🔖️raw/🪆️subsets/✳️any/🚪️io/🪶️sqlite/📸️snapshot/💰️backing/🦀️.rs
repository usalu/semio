//! 💰️ Authored Binary SQL ownership frontier.
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use super::BinarySnapshot;
use semio_framework_os_kernel::{sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,SqliteValue,artifact::{Cell,RowWriter,Reconstruction,ordered_row_refs},transfer::reserve,validate_sqlite_database_schema_controlled}};
use semio_framework_value::FromValue;
type Result<T>=std::result::Result<T,ValueError>;
const SQL:&str=include_str!("../🗄️.sql");
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn owner<T:FromValue>(value:T)->DecodedFieldOwner<T>{DecodedFieldOwner::new(value,T::retire_decoded)}

fn authored_schema(control:&SqliteSnapshotControl<'_>)->Result<()>{let limits=control.limits();if SQL.len()>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"authored binary schema exceeds caller limit"))}if limits.max_tables<2||limits.max_columns<4{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"authored binary table or column extent exceeds caller limit"))}Ok(())}
fn write_rows(schema:&str,count:usize,bytes:impl IntoIterator<Item=Result<u8>>,output:&mut RowWriter<'_,'_>)->Result<()>{
 let total=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"binary entity count overflow"))?;output.check_rows(total)?;let document=output.insert("binary_document",&[Cell::Text(schema)])?;output.checkpoint_total(total)?;
 for(index,byte)in bytes.into_iter().enumerate(){let ordinal=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"binary ordinal overflow"))?;output.insert("binary_byte",&[Cell::Integer(document),Cell::Integer(ordinal),Cell::Integer(i64::from(byte?))])?;output.checkpoint_total(total)?;}Ok(())
}
/// 🫳️ Admits the actual raw or validated hexadecimal octets before typed ownership.
pub(super)fn native_cells(schema:&str,count:usize,bytes:impl IntoIterator<Item=Result<u8>>,control:&mut SqliteSnapshotControl<'_>)->Result<()>{authored_schema(control)?;let mut output=RowWriter::borrowed(control,SqliteSnapshotPhase::DecodeNative)?;write_rows(schema,count,bytes,&mut output)?;output.finish_borrowed()}
/// 📏️ Visits the same authored cells without allocating a candidate database.
pub(super)fn semantic(snapshot:&BinarySnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<()>{authored_schema(control)?;let mut output=RowWriter::borrowed(control,phase)?;write_rows(&snapshot.schema,snapshot.bytes.len(),snapshot.bytes.iter().copied().map(Ok),&mut output)?;output.finish_borrowed()}
/// 📤️ Pays the identical authored row visitor through the actual SQL projection.
pub(super)fn project(snapshot:&BinarySnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{semantic(snapshot,control,SqliteSnapshotPhase::ProjectSnapshot)?;let mut output=RowWriter::new(SQL,control)?;write_rows(&snapshot.schema,snapshot.bytes.len(),snapshot.bytes.iter().copied().map(Ok),&mut output)?;output.finish()}
/// 📥️ Admits ordered borrowed references and concrete native octets before typed reconstruction.
pub(super) fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<BinarySnapshot>{
 validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let document=database.table("binary_document")?.single_row()?;if document.rowid!=1||document.values.len()!=2||document.integer(0)?!=1{return Err(invalid("binary document identity or width"))}
 let rows=ordered_row_refs(database.table("binary_byte")?,2,control)?;let schema=owner(Reconstruction::new(control)?.text(document.text(1)?)?);let mut bytes=owner(reserve::<u8>(rows.len(),control)?);
 for row in &rows{if row.rowid<=0||row.values.len()!=4||row.integer(0)?!=row.rowid||row.integer(1)?!=1{return Err(invalid("binary octet identity or document ownership"))}let value=u8::try_from(row.integer(3)?).map_err(|_|invalid("binary octet must fit u8"))?;Reconstruction::new(control)?.scalar()?;bytes.as_mut().push(value);}
 let snapshot=owner(BinarySnapshot{schema:schema.take(),bytes:bytes.take()});Reconstruction::new(control)?.checkpoint()?;Ok(snapshot.take())
}
