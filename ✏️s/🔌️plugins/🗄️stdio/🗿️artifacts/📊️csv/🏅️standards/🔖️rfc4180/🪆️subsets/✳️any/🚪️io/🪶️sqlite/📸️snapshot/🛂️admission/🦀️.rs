//! 🛂️ Complete borrowed CSV cells precede the controlled typed constructor.
use super::*;use semio_framework_dsl_record::{FieldValue,RecordValue};use semio_framework_value::{NativeDecodeControl,ValueRefusalKind};use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotPhase,artifact::{Cell,RowWriter}};
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn field(value:&RecordValue,id:u16)->Result<&FieldValue,ValueError>{value.get(id).ok_or_else(||invalid("required native field is absent"))}
fn list(value:&FieldValue)->Result<&[FieldValue],ValueError>{match value{FieldValue::List(values)=>Ok(values),_=>Err(invalid("native list shape differs"))}}
fn record(value:&FieldValue)->Result<&RecordValue,ValueError>{match value{FieldValue::Record(value)=>Ok(value),_=>Err(invalid("native record shape differs"))}}
fn text(value:&FieldValue)->Result<&str,ValueError>{match value{FieldValue::Text(value)=>Ok(value),_=>Err(invalid("native text shape differs"))}}
fn boolean(value:&FieldValue)->Result<bool,ValueError>{match value{FieldValue::Bool(value)=>Ok(*value),_=>Err(invalid("native boolean shape differs"))}}
fn ordinal(index:usize)->Result<i64,ValueError>{i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"native ordinal overflow"))}
fn write(root:&RecordValue,total:usize,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let header=boolean(field(root,1)?)?;let rows=list(field(root,2)?)?;let document=p.insert("csv_document",&[Cell::Text(text(field(root,0)?)?),Cell::Integer(i64::from(header))])?;p.checkpoint_total(total)?;
 for(index,row)in rows.iter().enumerate(){let record_id=p.insert("csv_record",&[Cell::Integer(document),Cell::Integer(ordinal(index)?)])?;p.checkpoint_total(total)?;for(index,value)in list(field(record(row)?,0)?)?.iter().enumerate(){let cell=record(value)?;p.insert("csv_field",&[Cell::Integer(record_id),Cell::Integer(ordinal(index)?),Cell::Text(text(field(cell,0)?)?),Cell::Integer(i64::from(boolean(field(cell,1)?)?))])?;p.checkpoint_total(total)?;}}Ok(())
}
/// 🫳️ Reads every required declared slot and all exact SQL storage-class cells before binding.
pub(in super::super)fn admit(root:&RecordValue,native:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 let rows=list(field(root,2)?)?;let mut total=1usize;for row in rows{let cells=list(field(record(row)?,0)?)?;total=total.checked_add(1).and_then(|n|n.checked_add(cells.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"native row count overflow"))?;}
 native.scoped_stage(|native|{native.begin_stage(total)?;let mut completed=0;let mut progress=|event:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{let delta=event.completed.saturating_sub(completed);completed=event.completed;native.advance(delta).is_ok()};let mut control=SqliteSnapshotControl::new(&mut progress,limits);let bounds=control.limits();if CsvSnapshot::SQLITE_SCHEMA.len()>bounds.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"authored native schema exceeds caller limit"))}if bounds.max_tables<3||bounds.max_columns<5{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"authored native table or column extent exceeds caller limit"))}let mut writer=RowWriter::borrowed(&mut control,SqliteSnapshotPhase::DecodeNative)?;write(root,total,&mut writer)?;writer.finish_borrowed()?;Ok(())})
}
