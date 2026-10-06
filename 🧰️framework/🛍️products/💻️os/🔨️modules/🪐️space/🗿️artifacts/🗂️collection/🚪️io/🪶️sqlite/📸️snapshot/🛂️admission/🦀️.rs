//! 🛂️ Complete borrowed Collection cells preserve intrinsic Statements bodies.
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

fn write(root:&RecordValue,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let(folders,entries)=(list(root,2)?,list(root,3)?);let total=fields::add(fields::add(1,folders.len())?,entries.len().checked_mul(2).ok_or_else(||fields::invalid("collection semantic count overflow"))?)?;
 let document=p.insert("collection_document",&[Cell::Text(text(root,0)?),Cell::Text(text(root,1)?)])?;p.checkpoint_total(total)?;
 for(index,value)in folders.iter().enumerate(){let row=row(value)?;p.insert("collection_folder",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(text(row,0)?),optional(row,1)?,Cell::Text(text(row,2)?)])?;p.checkpoint_total(total)?;}
 for(index,value)in entries.iter().enumerate(){let row=row(value)?;let Some(FieldValue::Statements(items))=field(row,4)else{return Err(fields::invalid("collection native body requires Statements"))};let[(kind,body)]=items.as_slice()else{return Err(fields::invalid("collection native body arity differs"))};if !matches!(kind.as_str(),"document"|"blob"){return Err(fields::invalid("collection native body kind differs"))}
 let entry=p.insert("collection_entry",&[Cell::Integer(document),Cell::Integer(fields::ordinal(index)?),Cell::Text(text(row,0)?),optional(row,1)?,Cell::Text(text(row,2)?),Cell::Text(text(row,3)?),Cell::Text(kind)])?;p.checkpoint_total(total)?;
 match kind.as_str(){"document"=>p.insert_key("collection_document_body",entry,&[Cell::Text(text(body,0)?),Cell::Text(text(body,1)?)])?,"blob"=>{let Some(FieldValue::UInt(size))=field(body,1)else{return Err(fields::invalid("collection native blob size differs"))};p.insert_key("collection_blob_body",entry,&[Cell::Text(text(body,0)?),Cell::Integer((size>>32)as i64),Cell::Integer((size&0xffffffff)as i64),Cell::Text(text(body,2)?)])?;},_=>unreachable!()}p.checkpoint_total(total)?;}Ok(())
}
pub(crate)fn record(root:&RecordValue,native:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let mut completed=0;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{let delta=event.completed.saturating_sub(completed);completed=event.completed;native.advance(delta).is_ok()};let mut control=SqliteSnapshotControl::new(&mut progress,limits);schema(&mut control)?;let mut p=RowWriter::borrowed(&mut control,SqliteSnapshotPhase::DecodeNative)?;write(root,&mut p)?;p.finish_borrowed()})}
