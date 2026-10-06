//! 🛂️ Complete borrowed Block3d semantic SQL cells before typed native construction.
use super::{Cell,Cells,RowWriter,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError};
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::NativeDecodeControl;
use store::sqlite_snapshot::SqliteDatabaseLimits;
fn required(r:&R,id:u16)->Result<&F,ValueError>{r.get(id).ok_or_else(||super::invalid("Block3d required native field absent"))}
fn record(value:&F)->Result<&R,ValueError>{match value{F::Record(r)=>Ok(r),F::Block(value)=>record(value),_=>Err(super::invalid("Block3d native record required"))}}
fn list(value:&F)->Result<&[F],ValueError>{match value{F::List(values)=>Ok(values),_=>Err(super::invalid("Block3d native list required"))}}
fn text(value:&F)->Result<&str,ValueError>{match value{F::Text(value)=>Ok(value),_=>Err(super::invalid("Block3d native text required"))}}
fn scalar(value:&F)->Result<f64,ValueError>{match value{F::Float(value)=>Ok(*value),_=>Err(super::invalid("Block3d native binary64 required"))}}
fn optional(value:Option<&F>)->Result<Cell<'_>,ValueError>{match value{None|Some(F::Absent)=>Ok(Cell::Null),Some(value)=>text(value).map(Cell::Text)}}
fn vector(value:&F)->Result<[f64;3],ValueError>{let F::Tuple(values)=value else{return Err(super::invalid("Block3d native vector required"))};if values.len()!=3{return Err(super::invalid("Block3d native vector width differs"))}Ok([scalar(&values[0])?,scalar(&values[1])?,scalar(&values[2])?])}
fn strings<const N:usize>(r:&R,ids:[u16;N])->Result<[Cell<'_>;N],ValueError>{let mut cells=[Cell::Null;N];for(index,id)in ids.into_iter().enumerate(){cells[index]=Cell::Text(text(required(r,id)?)?)}Ok(cells)}
fn identity(parent:i64,index:usize)->Result<[Cell<'static>;2],ValueError>{Ok([Cell::Integer(parent),super::ordinal(index)?])}
fn attribute(r:&R,parent:i64,index:usize,out:&mut RowWriter<'_,'_>,table:&str)->Result<(),ValueError>{let key=strings(r,[0,1])?;let owner=identity(parent,index)?;out.insert(table,&[owner[0],owner[1],key[0],key[1],optional(r.get(2))?])?;Ok(())}
fn count(root:&R)->Result<usize,ValueError>{let mut total=5usize;for id in[2,4,5,6,7,8]{total=total.checked_add(list(required(root,id)?)?.len()).ok_or_else(||super::invalid("Block3d native entity extent overflow"))?}for row in list(required(root,2)?)?{let r=record(row)?;for id in[3,6]{total=total.checked_add(list(required(r,id)?)?.len()).ok_or_else(||super::invalid("Block3d native representation extent overflow"))?}}Ok(total)}
fn write(root:&R,out:&mut RowWriter<'_,'_>,total:usize)->Result<(),ValueError>{
 let doc=out.insert("block3_document",&[Cell::Text(text(required(root,0)?)?)])?;
 let kind=record(required(root,1)?)?;let fields=strings(kind,[0,1,2,4])?;out.insert("block3_kind",&[Cell::Integer(doc),fields[0],fields[1],fields[2],optional(kind.get(3))?,fields[3],optional(kind.get(5))?,optional(kind.get(6))?])?;
 let child=record(required(root,3)?)?;let target=record(required(child,1)?)?;let words=strings(target,[1,2,3])?;
 super::catalog_dialect(text(required(target,1)?)?,text(required(target,2)?)?,text(required(target,3)?)?)?;
 out.insert("block3_catalog_child",&[Cell::Integer(doc),Cell::Text(text(required(child,0)?)?),Cell::Text(text(required(target,0)?)?),words[0],words[1],words[2]])?;
 for(index,row)in list(required(root,2)?)?.iter().enumerate(){let r=record(row)?;let owner=identity(doc,index)?;let s=strings(r,[0,1,5])?;let parent=out.insert("block3_representation",&[owner[0],owner[1],s[0],s[1],optional(r.get(2))?,optional(r.get(4))?,s[2]])?;for(index,tag)in list(required(r,3)?)?.iter().enumerate(){let owner=identity(parent,index)?;out.insert("block3_representation_tag",&[owner[0],owner[1],Cell::Text(text(tag)?)])?;}for(index,a)in list(required(r,6)?)?.iter().enumerate(){attribute(record(a)?,parent,index,out,"block3_representation_attribute")?}}
 for(index,row)in list(required(root,4)?)?.iter().enumerate(){let r=record(row)?;let owner=identity(doc,index)?;let s=strings(r,[0,1,2,3,4])?;out.insert("block3_vortex_kind_extra",&[owner[0],owner[1],s[0],s[1],s[2],s[3],s[4]])?;}
 for(index,row)in list(required(root,5)?)?.iter().enumerate(){let r=record(row)?;let owner=identity(doc,index)?;let s=strings(r,[0,1])?;let mut cells=Cells::new(&[owner[0],owner[1],s[0],s[1]]);cells.xyz(vector(required(r,2)?)?);cells.xyz(vector(required(r,3)?)?);cells.float(scalar(required(r,4)?)?);cells.push(optional(r.get(5))?);cells.insert(out,7)?;}
 for(index,row)in list(required(root,6)?)?.iter().enumerate(){let r=record(row)?;let owner=identity(doc,index)?;let s=strings(r,[0,1,2])?;let F::Bool(value)=required(r,3)? else{return Err(super::invalid("Block3d native Boolean required"))};out.insert("block3_compatibility",&[owner[0],owner[1],s[0],s[1],s[2],Cell::Integer(i64::from(*value))])?;}
 for(index,row)in list(required(root,7)?)?.iter().enumerate(){attribute(record(row)?,doc,index,out,"block3_attribute")?;}
 for(index,row)in list(required(root,8)?)?.iter().enumerate(){let r=record(row)?;let owner=identity(doc,index)?;let s=strings(r,[0,1])?;out.insert("block3_author",&[owner[0],owner[1],s[0],s[1],optional(r.get(2))?])?;}
 let camera=record(required(root,9)?)?;let mut cells=Cells::new(&[Cell::Integer(doc)]);cells.xyz(vector(required(camera,0)?)?);cells.xyz(vector(required(camera,1)?)?);cells.float(scalar(required(camera,2)?)?);cells.insert(out,11)?;
 out.insert("block3_meta",&[Cell::Integer(doc),Cell::Text(text(required(record(required(root,10)?)?,0)?)?)])?;out.checkpoint_total(total)?;Ok(())
}
/// 🫳️ Admit copied limits against actual cells under the same caller's native progress.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 let total=count(root)?;native.scoped_stage(|native|{native.begin_stage(total)?;let mut last=0;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{let delta=event.completed.saturating_sub(last);last=last.max(event.completed);native.advance(delta).is_ok()};let mut control=SqliteSnapshotControl::new(&mut progress,limits);super::admit_schema(&control)?;control.check_rows(total)?;let mut out=RowWriter::borrowed(&mut control,SqliteSnapshotPhase::DecodeNative)?;write(root,&mut out,total)?;out.finish_borrowed()?;Ok(())})
}
