//! 💰️ Authored Deflate reconstruction ownership frontier.
use super::{DeflateSnapshot,DeflateLevelHint};
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use semio_framework_os_kernel::{sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,SqliteValue,artifact::{Cell,Projection,Reconstruction,ordered_row_refs},transfer::reserve,validate_sqlite_database_schema_controlled}};
use semio_framework_value::FromValue;
type Result<T>=std::result::Result<T,ValueError>;
const SQL:&str=include_str!("../🗄️.sql");
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn owner<T:FromValue>(value:T)->DecodedFieldOwner<T>{DecodedFieldOwner::new(value,T::retire_decoded)}

/// 📤️ Pays complete authored Deflate document and payload rows after semantic bounds.
pub(super) fn project(snapshot:&DeflateSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{
 control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,snapshot.payload.len())?;if snapshot.compression_method>15||snapshot.window_bits>15{return Err(invalid("Deflate CMF fields must fit their declared four-bit nibbles"))}let total=snapshot.payload.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate row count overflow"))?;control.check_rows(total)?;let bytes=snapshot.payload.len().checked_mul(32).and_then(|bytes|bytes.checked_add(if snapshot.dict_id.is_some(){40}else{32})).and_then(|bytes|bytes.checked_add(snapshot.schema.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Deflate value extent overflow"))?;control.check_value_bytes(bytes)?;
 let mut output=Projection::new(SQL,control)?;output.insert("deflate_document",&[Cell::Text(&snapshot.schema),Cell::Integer(i64::from(snapshot.compression_method)),Cell::Integer(i64::from(snapshot.window_bits)),Cell::Integer(i64::from(snapshot.compression_level_hint.to_bits())),snapshot.dict_id.map_or(Cell::Null,|value|Cell::Integer(i64::from(value)))])?;for(index,byte)in snapshot.payload.iter().enumerate(){output.insert("deflate_payload_byte",&[Cell::Integer(1),Cell::Integer(i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Deflate ordinal overflow"))?),Cell::Integer(i64::from(*byte))])?;}output.checkpoint_total(total)?;output.finish()
}
/// 📥️ Pays ordered row references, literal schema and concrete payload octets before final native ownership.
pub(super) fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<DeflateSnapshot>{
 validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let document=database.table("deflate_document")?.single_row()?;if document.rowid!=1||document.values.len()!=6||document.integer(0)?!=1{return Err(invalid("Deflate document identity or width"))}
 let compression_method=u8::try_from(document.integer(2)?).map_err(|_|invalid("Deflate CMF method requires u8"))?;let window_bits=u8::try_from(document.integer(3)?).map_err(|_|invalid("Deflate CMF window requires u8"))?;if compression_method>15||window_bits>15{return Err(invalid("Deflate CMF fields must fit their four-bit nibbles"))}
 let hint=u8::try_from(document.integer(4)?).map_err(|_|invalid("Deflate level hint requires u8"))?;if hint>3{return Err(invalid("Deflate level hint exceeds two bits"))}let dict_id=match document.values.get(5){Some(SqliteValue::Null)=>None,Some(SqliteValue::Integer(value))=>Some(u32::try_from(*value).map_err(|_|invalid("Deflate dictionary identifier requires u32"))?),_=>return Err(invalid("Deflate dictionary identifier requires INTEGER or NULL"))};
 for _ in 0..4{Reconstruction::new(control)?.scalar()?;}
 let rows=ordered_row_refs(database.table("deflate_payload_byte")?,2,control)?;let schema=owner(Reconstruction::new(control)?.text(document.text(1)?)?);let mut payload=owner(reserve::<u8>(rows.len(),control)?);
 for row in &rows{if row.rowid<=0||row.values.len()!=4||row.integer(0)?!=row.rowid||row.integer(1)?!=1{return Err(invalid("Deflate payload identity or document ownership"))}let value=u8::try_from(row.integer(3)?).map_err(|_|invalid("Deflate payload octet must fit u8"))?;Reconstruction::new(control)?.scalar()?;payload.as_mut().push(value);}
 let snapshot=owner(DeflateSnapshot{schema:schema.take(),compression_method,window_bits,compression_level_hint:DeflateLevelHint::from_bits(hint),dict_id,payload:payload.take()});Reconstruction::new(control)?.checkpoint()?;Ok(snapshot.take())
}
