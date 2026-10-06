//! 🛂️ Complete borrowed Space cells precede typed construction.
use super::*;
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_value::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteDatabaseLimits,artifact::RowWriter};
fn unwrap(value:&FieldValue)->&FieldValue{match value{FieldValue::Block(value)=>value,_=>value}}
fn field(root:&RecordValue,id:u16)->Option<&FieldValue>{root.get(id).map(unwrap)}
fn text(root:&RecordValue,id:u16)->Result<&str,ValueError>{match field(root,id){Some(FieldValue::Text(value))=>Ok(value),_=>Err(fields::invalid("authored semantic text differs"))}}
fn optional(root:&RecordValue,id:u16)->Result<Cell<'_>,ValueError>{match field(root,id){Some(FieldValue::Text(value))=>Ok(Cell::Text(value)),None|Some(FieldValue::Absent)=>Ok(Cell::Null),_=>Err(fields::invalid("authored nullable semantic text differs"))}}
fn list(root:&RecordValue,id:u16)->Result<&[FieldValue],ValueError>{match field(root,id){Some(FieldValue::List(value))=>Ok(value),None|Some(FieldValue::Absent)=>Ok(&[]),_=>Err(fields::invalid("authored semantic list differs"))}}
fn row(value:&FieldValue)->Result<&RecordValue,ValueError>{match unwrap(value){FieldValue::Record(value)=>Ok(value),_=>Err(fields::invalid("authored semantic record differs"))}}
fn choice<'a>(root:&RecordValue,id:u16,labels:&'a[&'a str])->Result<&'a str,ValueError>{match field(root,id){Some(FieldValue::Enum(value))=>labels.get(*value as usize).copied().ok_or_else(||fields::invalid("authored semantic enum differs")),_=>Err(fields::invalid("authored semantic enum shape differs"))}}

fn write(root:&RecordValue,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let total=[4,5,6,7].into_iter().try_fold(1,|n,id|fields::add(n,list(root,id)?.len()))?;
 let document=p.insert("space_document",&[Cell::Text(text(root,0)?),Cell::Text(text(root,1)?),Cell::Text(choice(root,2,&["atelier","studio","archive"])?),Cell::Text(choice(root,3,&["private","public"])?)])?;p.checkpoint_total(total)?;
 for(index,value)in list(root,4)?.iter().enumerate(){let row=row(value)?;p.insert("space_user",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(text(row,0)?),Cell::Text(text(row,1)?),optional(row,2)?,Cell::Text(choice(row,3,&["author","spectator"])?)])?;p.checkpoint_total(total)?;}
 for(index,value)in list(root,5)?.iter().enumerate(){let row=row(value)?;p.insert("space_collection",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(text(row,0)?),Cell::Text(text(row,1)?),Cell::Text(text(row,2)?)])?;p.checkpoint_total(total)?;}
 for(index,value)in list(root,6)?.iter().enumerate(){let FieldValue::Text(value)=unwrap(value)else{return Err(fields::invalid("authored program text differs"))};p.insert("space_program",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(value)])?;p.checkpoint_total(total)?;}
 for(index,value)in list(root,7)?.iter().enumerate(){let row=row(value)?;let Some(FieldValue::Bool(enabled))=field(row,4)else{return Err(fields::invalid("authored extension flag differs"))};p.insert("space_extension",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(text(row,0)?),Cell::Text(text(row,1)?),Cell::Text(text(row,2)?),Cell::Text(text(row,3)?),Cell::Integer(i64::from(*enabled))])?;p.checkpoint_total(total)?;}Ok(())
}
pub(crate)fn record(root:&RecordValue,native:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let mut completed=0;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{let delta=event.completed.saturating_sub(completed);completed=event.completed;native.advance(delta).is_ok()};let mut control=SqliteSnapshotControl::new(&mut progress,limits);schema(&mut control)?;let mut p=RowWriter::borrowed(&mut control,SqliteSnapshotPhase::DecodeNative)?;write(root,&mut p)?;p.finish_borrowed()})}
