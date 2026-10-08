//! 🛂️ Complete borrowed history cells are admitted before typed owner construction.
use super::*;
use crate::sqlite_snapshot::{SqliteDatabaseLimits,artifact::RowWriter};
use semio_framework_value::{DslValue,NativeDecodeControl,Number};
fn required<'a>(value:&'a DslValue,key:&str)->Result<&'a DslValue,ValueError>{
 let DslValue::Object(fields)=value else{return Err(fields::invalid("history semantic object is required"))};
 fields.iter().find(|(name,_)|name==key).map(|(_,value)|value).ok_or_else(||fields::invalid("history semantic field is required"))
}
fn optional<'a>(value:&'a DslValue,key:&str)->Result<Option<&'a str>,ValueError>{
 let DslValue::Object(fields)=value else{return Err(fields::invalid("history semantic object is required"))};
 match fields.iter().find(|(name,_)|name==key).map(|(_,value)|value){
  None|Some(DslValue::Null)=>Ok(None),Some(value)=>Ok(Some(text(value)?)),
 }
}
fn text(value:&DslValue)->Result<&str,ValueError>{match value{DslValue::String(text)=>Ok(text),_=>Err(fields::invalid("history semantic text is required"))}}
fn label<'a>(value:&'a DslValue,key:&str)->Result<&'a str,ValueError>{text(required(value,key)?)}
fn list(value:&DslValue)->Result<&[DslValue],ValueError>{match value{DslValue::Array(rows)=>Ok(rows),_=>Err(fields::invalid("history semantic list is required"))}}
fn children<'a>(value:&'a DslValue,key:&str)->Result<&'a [DslValue],ValueError>{list(required(value,key)?)}
fn unsigned(value:&DslValue)->Result<u64,ValueError>{match value{DslValue::Number(Number::UInt(value))=>Ok(*value),DslValue::Number(Number::Int(value))if *value>=0=>Ok(*value as u64),_=>Err(fields::invalid("history semantic clock word is required"))}}
fn write(value:&DslValue,total:usize,p:&mut RowWriter<'_, '_>)->Result<(),ValueError>{
 let checkpoints=children(value,"checkpoints")?;let alternatives=children(value,"alternatives")?;
 let document=p.insert("space_history_document",&[optional(value,"activeAlternativeId")?.map(Cell::Text).unwrap_or(Cell::Null)])?;p.checkpoint_total(total)?;
 for(index,row)in checkpoints.iter().enumerate(){
  let checkpoint=p.insert("space_history_checkpoint",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(label(row,"id")?),optional(row,"parentId")?.map(Cell::Text).unwrap_or(Cell::Null),Cell::Text(label(row,"message")?)])?;p.checkpoint_total(total)?;
  let clock=required(row,"timestamp")?;let actor=pair(unsigned(required(clock,"actor")?)?);let physical=pair(unsigned(required(clock,"physical_ms")?)?);let logical=pair(unsigned(required(clock,"logical")?)?);
  p.insert_key("space_history_timestamp",checkpoint,&[actor[0],actor[1],physical[0],physical[1],logical[0],logical[1]])?;p.checkpoint_total(total)?;
  for(index,author)in children(row,"authors")?.iter().enumerate(){
   p.insert("space_history_author",&[Cell::Integer(checkpoint),Cell::Integer(fields::ordinal(index)?),Cell::Text(label(author,"id")?),Cell::Text(label(author,"name")?),optional(author,"avatar")?.map(Cell::Text).unwrap_or(Cell::Null)])?;p.checkpoint_total(total)?;
  }
  for(index,pin)in children(row,"members")?.iter().enumerate(){
   p.insert("space_history_member_pin",&[Cell::Integer(checkpoint),Cell::Integer(fields::ordinal(index)?),Cell::Text(label(pin,"documentId")?),Cell::Text(label(pin,"checkpointId")?),Cell::Text(optional(pin,"alternativeId")?.unwrap_or(""))])?;p.checkpoint_total(total)?;
  }
 }
 for(index,row)in alternatives.iter().enumerate(){
  let alternative=p.insert("space_history_alternative",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(label(row,"id")?),Cell::Text(label(row,"name")?)])?;p.checkpoint_total(total)?;
  for(index,id)in children(row,"checkpointIds")?.iter().enumerate(){
   p.insert("space_history_alternative_checkpoint",&[Cell::Integer(alternative),Cell::Integer(fields::ordinal(index)?),Cell::Text(text(id)?)])?;p.checkpoint_total(total)?;
  }
 }Ok(())
}
pub(crate) fn intrinsic(value:&DslValue,native:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 let mut total=1usize;
 for row in children(value,"checkpoints")?{total=fields::add(total,fields::add(2,fields::add(children(row,"authors")?.len(),children(row,"members")?.len())?)?)?;}
 for row in children(value,"alternatives")?{total=fields::add(total,fields::add(1,children(row,"checkpointIds")?.len())?)?;}
 if total>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"space history native row ownership exceeds caller limit"))}
 native.scoped_stage(|native|{
  native.begin_stage(0)?;let mut completed=0;
  let mut callback=|event:crate::sqlite_snapshot::SqliteSnapshotProgress|{let delta=event.completed.saturating_sub(completed);completed=event.completed;native.advance(delta).is_ok()};
  let mut control=SqliteSnapshotControl::new(&mut callback,limits);schema(&mut control)?;
  let mut writer=RowWriter::borrowed(&mut control,SqliteSnapshotPhase::DecodeNative)?;
  write(value,total,&mut writer)?;writer.finish_borrowed()
 })
}
