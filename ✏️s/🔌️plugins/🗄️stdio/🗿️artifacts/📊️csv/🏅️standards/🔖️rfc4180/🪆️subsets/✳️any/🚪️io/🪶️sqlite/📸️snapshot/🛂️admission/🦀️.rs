//! 🛂️ Complete borrowed CSV cells precede the controlled typed constructor.
use super::*;use semio_framework_dsl_record::{FieldValue,RecordValue};use semio_framework_value::{NativeDecodeControl,ValueRefusalKind};use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotPhase,artifact::{Cell,RowWriter}};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn field(value:&RecordValue,id:u16)->Result<&FieldValue,ValueError>{value.get(id).ok_or_else(||invalid("required native field is absent"))}
fn list(value:&FieldValue)->Result<&[FieldValue],ValueError>{match value{FieldValue::List(values)=>Ok(values),_=>Err(invalid("native list shape differs"))}}
fn record(value:&FieldValue)->Result<&RecordValue,ValueError>{match value{FieldValue::Record(value)=>Ok(value),_=>Err(invalid("native record shape differs"))}}
fn text(value:&FieldValue)->Result<&str,ValueError>{match value{FieldValue::Text(value)=>Ok(value),_=>Err(invalid("native text shape differs"))}}
fn boolean(value:&FieldValue)->Result<bool,ValueError>{match value{FieldValue::Bool(value)=>Ok(*value),_=>Err(invalid("native boolean shape differs"))}}
fn ordinal(index:usize)->Result<i64,ValueError>{i64::try_from(index).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"native ordinal overflow"))}
fn write(root:&RecordValue,total:usize,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let header=boolean(field(root,1)?)?;let rows=list(field(root,2)?)?;let document=p.insert("csv_document",&[Cell::Text(text(field(root,0)?)?),Cell::Integer(i64::from(header))])?;p.checkpoint_total(total)?;
 for(index,row)in rows.iter().enumerate(){let record_id=p.insert("csv_record",&[Cell::Integer(document),Cell::Integer(ordinal(index)?)])?;p.checkpoint_total(total)?;for(index,value)in list(field(record(row)?,0)?)?.iter().enumerate(){let cell=record(value)?;p.insert("csv_field",&[Cell::Integer(record_id),Cell::Integer(ordinal(index)?),Cell::Text(text(field(cell,0)?)?),Cell::Integer(i64::from(boolean(field(cell,1)?)?))])?;p.checkpoint_total(total)?;}}Ok(())
}
/// 🫳️ Reads every required declared slot and all exact SQL storage-class cells before binding.
pub(in super::super)fn admit(root:&RecordValue,native:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 let rows=list(field(root,2)?)?;let mut total=1usize;for row in rows{let cells=list(field(record(row)?,0)?)?;total=total.checked_add(1).and_then(|n|n.checked_add(cells.len())).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native row count overflow"))?;}
 native.scoped_stage(|native|{native.begin_stage(total)?;let mut completed=0;let mut progress=|event:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{let delta=event.completed.saturating_sub(completed);completed=event.completed;native.advance(delta).is_ok()};let mut control=SqliteSnapshotControl::new(&mut progress,limits);let bounds=control.limits();if CsvSnapshot::SQLITE_SCHEMA.len()>bounds.max_schema_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"authored native schema exceeds caller limit"))}if bounds.max_tables<3||bounds.max_columns<5{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"authored native table or column extent exceeds caller limit"))}let mut writer=RowWriter::borrowed(&mut control,SqliteSnapshotPhase::DecodeNative)?;write(root,total,&mut writer)?;writer.finish_borrowed()?;Ok(())})
}

/// 📏️ Quotes the original CSV record and field topology without creating destination ownership.
pub(in super::super)fn binding_demands(root:&RecordValue)->Result<semio_framework_value::RetainedCloneGrant,ValueError>{binding_demands_with_checkpoint(root,||Ok(()))}
fn binding_demands_with_checkpoint(root:&RecordValue,mut checkpoint:impl FnMut()->Result<(),ValueError>)->Result<semio_framework_value::RetainedCloneGrant,ValueError>{
 checkpoint()?;let schema=text(field(root,0)?)?;boolean(field(root,1)?)?;let rows=list(field(root,2)?)?;
 let overflow=||ValueError::literal(ValueRefusalKind::OwnershipLimit,"CSV original typed extent overflow");let mut items=3usize;let mut copied=std::mem::size_of::<CsvSnapshot>().checked_add(std::mem::size_of::<Vec<CsvRecord>>()).and_then(|bytes|bytes.checked_add(schema.len())).ok_or_else(overflow)?;let mut capacity=rows.len().checked_mul(std::mem::size_of::<CsvRecord>()).and_then(|bytes|bytes.checked_add(schema.len())).ok_or_else(overflow)?;
 for row in rows{checkpoint()?;let cells=list(field(record(row)?,0)?)?;items=items.checked_add(2).and_then(|count|count.checked_add(cells.len().checked_mul(2)?)).ok_or_else(overflow)?;copied=copied.checked_add(std::mem::size_of::<CsvRecord>()).and_then(|bytes|bytes.checked_add(std::mem::size_of::<Vec<CsvField>>())).ok_or_else(overflow)?;capacity=capacity.checked_add(cells.len().checked_mul(std::mem::size_of::<CsvField>()).ok_or_else(overflow)?).ok_or_else(overflow)?;
  for cell in cells{checkpoint()?;let cell=record(cell)?;boolean(field(cell,1)?)?;let bytes=text(field(cell,0)?)?.len();copied=copied.checked_add(std::mem::size_of::<CsvField>()).and_then(|total|total.checked_add(bytes)).ok_or_else(overflow)?;capacity=capacity.checked_add(bytes).ok_or_else(overflow)?;}
 }
 Ok(semio_framework_value::RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copied,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:6})
}
/// 🧵️ Receives each original CSV row and field before its children can allocate or cancel.
pub(in super::super)fn bind(root:&RecordValue,output:&mut Option<CsvSnapshot>,native:&mut NativeDecodeControl<'_>,wallet:&mut semio_framework_os_kernel::NativeSnapshotBodyWallet)->Result<(),ValueError>{
 if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"CSV native destination must be empty"))}
 let demand=binding_demands_with_checkpoint(root,||native.checkpoint())?;wallet.admit_frontier(demand)?;
 if native.owned_bytes().checked_add(demand.maximum_capacity_bytes).is_none_or(|extent|extent>native.maximum_bytes()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"CSV typed destination exceeds original native ceiling"))}
 let schema=text(field(root,0)?)?;let has_header=boolean(field(root,1)?)?;let rows=list(field(root,2)?)?;let total=rows.iter().try_fold(1usize,|count,row|count.checked_add(1).and_then(|count|count.checked_add(list(field(record(row).ok()?,0).ok()?).ok()?.len())).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"CSV typed workload overflow")))?;
 wallet.record_progress(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<CsvSnapshot>(),..Default::default()})?;
 *output=Some(CsvSnapshot{schema:String::new(),has_header,records:Vec::new()});
 let result=native.scoped_stage(|native|{
  native.begin_stage(total)?;let snapshot=output.as_mut().unwrap();wallet.copy_text_into(native,schema,&mut snapshot.schema,2)?;native.step()?;wallet.allocate_vec_into(native,rows.len(),&mut snapshot.records,2)?;
  for row in rows{let cells=list(field(record(row)?,0)?)?;let header=semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<CsvRecord>(),..Default::default()};wallet.record_progress(header)?;snapshot.records.push(CsvRecord{fields:Vec::new()});let destination=&mut snapshot.records.last_mut().unwrap().fields;wallet.allocate_vec_into(native,cells.len(),destination,4)?;native.step()?;
   for cell in cells{let cell=record(cell)?;let quoted=boolean(field(cell,1)?)?;let value=text(field(cell,0)?)?;wallet.record_progress(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<CsvField>(),..Default::default()})?;destination.push(CsvField{value:String::new(),quoted});wallet.copy_text_into(native,value,&mut destination.last_mut().unwrap().value,6)?;native.step()?;}
  }Ok::<(),ValueError>(())
 });result.map_err(|error|error.with_retained_progress(wallet.progress()))
}
