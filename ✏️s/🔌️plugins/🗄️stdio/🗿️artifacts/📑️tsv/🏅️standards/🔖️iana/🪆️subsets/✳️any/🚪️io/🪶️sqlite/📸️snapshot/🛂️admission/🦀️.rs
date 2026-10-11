//! 🛂️ Complete borrowed TSV cells precede the controlled typed constructor.
use super::*;use semio_framework_dsl_record::{FieldValue,RecordValue};use semio_framework_value::{NativeDecodeControl,ValueRefusalKind};use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotPhase,artifact::{Cell,RowWriter}};
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn field(value:&RecordValue,id:u16)->Result<&FieldValue,ValueError>{value.get(id).ok_or_else(||invalid("required native field is absent"))}
fn list(value:&FieldValue)->Result<&[FieldValue],ValueError>{match value{FieldValue::List(values)=>Ok(values),_=>Err(invalid("native list shape differs"))}}
fn record(value:&FieldValue)->Result<&RecordValue,ValueError>{match value{FieldValue::Record(value)=>Ok(value),_=>Err(invalid("native record shape differs"))}}
fn text(value:&FieldValue)->Result<&str,ValueError>{match value{FieldValue::Text(value)=>Ok(value),_=>Err(invalid("native text shape differs"))}}
fn boolean(value:&FieldValue)->Result<bool,ValueError>{match value{FieldValue::Bool(value)=>Ok(*value),_=>Err(invalid("native boolean shape differs"))}}
fn ordinal(index:usize)->Result<i64,ValueError>{i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"native ordinal overflow"))}
/// 📏️ Describes exact original typed slots and UTF8 payload before any destination birth.
pub(in super::super)fn binding_demands(root:&RecordValue)->Result<semio_framework_value::RetainedCloneGrant,ValueError>{binding_demands_with_checkpoint(root,||Ok(()))}
fn binding_demands_with_checkpoint(root:&RecordValue,mut checkpoint:impl FnMut()->Result<(),ValueError>)->Result<semio_framework_value::RetainedCloneGrant,ValueError>{
 checkpoint()?;
 let rows=list(field(root,1)?)?;let schema=text(field(root,0)?)?;boolean(field(root,2)?)?;if !matches!(field(root,3)?,FieldValue::Enum(0)|FieldValue::Enum(1)){return Err(invalid("TSV native ending ordinal differs"))}
 let overflow=||ValueError::literal(ValueRefusalKind::OwnershipLimit,"TSV original typed extent overflow");let mut items=3usize;let mut copied=std::mem::size_of::<TsvSnapshot>().checked_add(std::mem::size_of::<Vec<Vec<String>>>()).and_then(|n|n.checked_add(schema.len())).ok_or_else(overflow)?;let mut capacity=rows.len().checked_mul(std::mem::size_of::<Vec<String>>()).and_then(|n|n.checked_add(schema.len())).ok_or_else(overflow)?;
 for row in rows{checkpoint()?;let cells=list(row)?;items=items.checked_add(1).and_then(|n|n.checked_add(cells.len())).ok_or_else(overflow)?;let slots=cells.len().checked_mul(std::mem::size_of::<String>()).ok_or_else(overflow)?;capacity=capacity.checked_add(slots).ok_or_else(overflow)?;copied=copied.checked_add(2*std::mem::size_of::<Vec<String>>()).and_then(|n|n.checked_add(slots)).ok_or_else(overflow)?;for cell in cells{checkpoint()?;let bytes=text(cell)?.len();capacity=capacity.checked_add(bytes).ok_or_else(overflow)?;copied=copied.checked_add(bytes).ok_or_else(overflow)?;}}
 Ok(semio_framework_value::RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copied,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:4})
}
/// 🧵️ Registers every typed child before birth and returns actual progress to its original wallet.
pub(in super::super)fn bind(root:&RecordValue,output:&mut Option<TsvSnapshot>,native:&mut NativeDecodeControl<'_>,wallet:&mut semio_framework_os_kernel::NativeSnapshotBodyWallet)->Result<(),ValueError>{
 if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"TSV native destination must be empty"))}
 wallet.admit_frontier(binding_demands_with_checkpoint(root,||native.checkpoint())?)?;
 let schema=text(field(root,0)?)?;let rows=list(field(root,1)?)?;let trailing_newline=boolean(field(root,2)?)?;let line_ending=match field(root,3)?{FieldValue::Enum(0)=>LineEnding::Lf,FieldValue::Enum(1)=>LineEnding::Crlf,_=>return Err(invalid("TSV native ending ordinal differs"))};
 let total=rows.iter().try_fold(1usize,|n,row|n.checked_add(1).and_then(|n|n.checked_add(list(row).ok()?.len())).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"TSV typed workload overflow")))?;
 *output=Some(TsvSnapshot{schema:String::new(),records:Vec::new(),trailing_newline,line_ending});
 let mut progress=semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<TsvSnapshot>(),..Default::default()};
 let result=native.scoped_stage(|native|{
  native.begin_stage(total)?;let snapshot=output.as_mut().unwrap();let result=native.copy_text_into(schema,&mut snapshot.schema);if result.is_ok()||snapshot.schema.capacity()>0{progress.copied_items+=1;}progress.copied_bytes+=snapshot.schema.len();progress.retained_capacity_bytes+=snapshot.schema.capacity();result?;native.step()?;
  snapshot.records=native.allocate_vec(rows.len())?;progress.copied_items+=1;progress.copied_bytes+=std::mem::size_of::<Vec<Vec<String>>>();progress.retained_capacity_bytes+=snapshot.records.capacity()*std::mem::size_of::<Vec<String>>();
  for row in rows{let cells=list(row)?;snapshot.records.push(Vec::new());progress.copied_items+=1;progress.copied_bytes+=std::mem::size_of::<Vec<String>>();let destination=snapshot.records.last_mut().unwrap();*destination=native.allocate_vec(cells.len())?;progress.copied_bytes+=std::mem::size_of::<Vec<String>>();progress.retained_capacity_bytes+=destination.capacity()*std::mem::size_of::<String>();native.step()?;
   for cell in cells{destination.push(String::new());progress.copied_items+=1;progress.copied_bytes+=std::mem::size_of::<String>();let target=destination.last_mut().unwrap();let result=native.copy_text_into(text(cell)?,target);progress.copied_bytes+=target.len();progress.retained_capacity_bytes+=target.capacity();result?;native.step()?;}
  }Ok::<(),ValueError>(())
 });
 wallet.record_progress(progress).map_err(|error|error.with_retained_progress(progress))?;result.map_err(|error|error.with_retained_progress(progress))
}

fn write(root:&RecordValue,total:usize,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let ending=match field(root,3)?{FieldValue::Enum(0)=>"lf",FieldValue::Enum(1)=>"crlf",_=>return Err(invalid("TSV native ending ordinal differs"))};let trailing=boolean(field(root,2)?)?;let rows=list(field(root,1)?)?;let document=p.insert("tsv_document",&[Cell::Text(text(field(root,0)?)?),Cell::Integer(i64::from(trailing)),Cell::Text(ending)])?;p.checkpoint_total(total)?;
 for(index,row)in rows.iter().enumerate(){let record_id=p.insert("tsv_record",&[Cell::Integer(document),Cell::Integer(ordinal(index)?)])?;p.checkpoint_total(total)?;for(index,value)in list(row)?.iter().enumerate(){p.insert("tsv_field",&[Cell::Integer(record_id),Cell::Integer(ordinal(index)?),Cell::Text(text(value)?)])?;p.checkpoint_total(total)?;}}Ok(())
}
/// 🫳️ Reads every required declared slot and all exact SQL storage-class cells before binding.
pub(in super::super)fn admit(root:&RecordValue,native:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 let rows=list(field(root,1)?)?;let mut total=1usize;for row in rows{let cells=list(row)?;total=total.checked_add(1).and_then(|n|n.checked_add(cells.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"native row count overflow"))?;}
 native.scoped_stage(|native|{native.begin_stage(total)?;let mut completed=0;let mut progress=|event:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{let delta=event.completed.saturating_sub(completed);completed=event.completed;native.advance(delta).is_ok()};let mut control=SqliteSnapshotControl::new(&mut progress,limits);let bounds=control.limits();if TsvSnapshot::SQLITE_SCHEMA.len()>bounds.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"authored native schema exceeds caller limit"))}if bounds.max_tables<3||bounds.max_columns<4{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"authored native table or column extent exceeds caller limit"))}let mut writer=RowWriter::borrowed(&mut control,SqliteSnapshotPhase::DecodeNative)?;write(root,total,&mut writer)?;writer.finish_borrowed()?;Ok(())})
}
