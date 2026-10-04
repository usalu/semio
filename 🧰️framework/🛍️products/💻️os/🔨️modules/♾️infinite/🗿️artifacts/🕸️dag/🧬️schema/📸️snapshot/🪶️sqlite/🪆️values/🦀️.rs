//! 🪆️ Paid iterative semantic values retain each recursive owner until transfer.
use crate::*;
use graph::manifest::{PropertyBag,PropertyValue};
use semio_framework_value::{DecodedValue,DslValue,FromValue,Number,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{artifact::{FloatColumn,FloatRow,Reconstruction},SqliteRow};
pub(super) use store::sqlite_snapshot::transfer::reserve;
use super::rows::Read;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
const NUMBER:&[FloatColumn]=&[FloatColumn::Binary64(2)];
pub(super) fn grow<T>(values:&mut Vec<T>,read:&mut Read<'_,'_,'_>)->Result<()>{
 if values.len()==values.capacity(){let count=values.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DAG value frontier overflow"))?;let mut next=reserve(count,read.control)?;for _ in 0..values.len(){read.step()?;}next.extend(std::mem::take(values));*values=next;}Ok(())
}
struct Values<T:FromValue>(Vec<(i64,T)>);
impl<T:FromValue> Drop for Values<T>{fn drop(&mut self){for(_,value)in std::mem::take(&mut self.0){T::retire_decoded(value)}}}
impl<T:FromValue> Values<T>{
 fn insert(&mut self,id:i64,value:T,read:&mut Read<'_,'_,'_>)->Result<()>{let value=DecodedValue::new(value,T::retire_decoded);grow(&mut self.0,read)?;self.0.push((id,value.take()));Ok(())}
 fn remove(&mut self,id:i64,read:&mut Read<'_,'_,'_>)->Result<T>{for index in 0..self.0.len(){read.step()?;if self.0[index].0==id{return Ok(self.0.swap_remove(index).1)}}Err(invalid("DAG value child is missing or repeatedly owned"))}
}
enum Frame<'a>{Enter(i64),Array(i64,Vec<&'a SqliteRow>),Object(i64,Vec<&'a SqliteRow>)}
fn descend<'a>(pending:&mut Vec<Frame<'a>>,frame:Frame<'a>,column:usize,read:&mut Read<'_,'_,'_>)->Result<()>{
 grow(pending,read)?;let index=pending.len();pending.push(frame);
 let count=match &pending[index]{Frame::Array(_,rows)|Frame::Object(_,rows)=>rows.len(),Frame::Enter(_)=>unreachable!()};
 for position in(0..count).rev(){let id=match &pending[index]{Frame::Array(_,rows)|Frame::Object(_,rows)=>rows[position].integer(column)?,Frame::Enter(_)=>unreachable!()};grow(pending,read)?;pending.push(Frame::Enter(id));read.step()?;}Ok(())
}
pub(super) fn property(root:i64,read:&mut Read<'_,'_,'_>)->Result<PropertyValue>{
 let mut pending=reserve(1,read.control)?;pending.push(Frame::Enter(root));let mut values=Values(Vec::new());
 while let Some(frame)=pending.pop(){match frame{
  Frame::Enter(id)=>{let row=read.take("dag_property_value",id)?;let value=match row.text(1)?{
   "null"=>PropertyValue::Null,
   "bool"=>{let row=read.child("dag_property_boolean",id)?;PropertyValue::Bool(read.boolean(row,2)?)},
   "number"=>PropertyValue::Number(FloatRow::new(read.child("dag_property_number",id)?,NUMBER)?.real(2)?),
   "string"=>{let row=read.child("dag_property_string",id)?;PropertyValue::String(read.text(row,2)?)},
   "array"=>{let rows=read.ordered("dag_property_array_element",1,id,2)?;descend(&mut pending,Frame::Array(id,rows),3,read)?;continue},
   "object"=>{let rows=read.ordered("dag_property_object_member",1,id,2)?;descend(&mut pending,Frame::Object(id,rows),4,read)?;continue},
   _=>return Err(invalid("DAG graph property variant differs"))};values.insert(id,value,read)?;},
  Frame::Array(id,rows)=>{let mut output=DecodedValue::new(PropertyValue::Array(reserve(rows.len(),read.control)?),PropertyValue::retire_decoded);for row in rows{let value=values.remove(row.integer(3)?,read)?;let PropertyValue::Array(children)=output.get_mut()else{unreachable!()};children.push(value);read.step()?;}values.insert(id,output.take(),read)?;},
  Frame::Object(id,rows)=>{let mut output=DecodedValue::new(PropertyBag::from_admitted(reserve(rows.len(),read.control)?),PropertyBag::retire_decoded);for row in rows{let key=read.text(row,3)?;if output.get_mut().contains_key(&key){return Err(invalid("DAG graph property map keys must be unique"))}let value=values.remove(row.integer(4)?,read)?;output.get_mut().insert_controlled(key,value,&mut||read.step())?;}values.insert(id,PropertyValue::Object(output.take()),read)?;}
 }read.step()?;}
 let output=DecodedValue::new(values.remove(root,read)?,PropertyValue::retire_decoded);if !values.0.is_empty(){return Err(invalid("DAG property contains unowned descendants"))}Ok(output.take())
}
pub(super) fn intrinsic(root:i64,read:&mut Read<'_,'_,'_>)->Result<DslValue>{
 let mut pending=reserve(1,read.control)?;pending.push(Frame::Enter(root));let mut values=Values(Vec::new());
 while let Some(frame)=pending.pop(){match frame{
  Frame::Enter(id)=>{let row=read.take("dag_intrinsic_value",id)?;let value=match row.text(1)?{
   "null"=>DslValue::Null,
   "bool"=>{let row=read.child("dag_intrinsic_boolean",id)?;DslValue::Bool(read.boolean(row,2)?)},
   "uint"=>{let row=read.child("dag_intrinsic_unsigned",id)?;DslValue::Number(Number::UInt(read.unsigned(row,2)?))},
   "int"=>DslValue::Number(Number::Int(read.child("dag_intrinsic_signed",id)?.integer(2)?)),
   "float"=>DslValue::Number(Number::Float(FloatRow::new(read.child("dag_intrinsic_float",id)?,NUMBER)?.real(2)?)),
   "string"=>{let row=read.child("dag_intrinsic_string",id)?;DslValue::String(read.text(row,2)?)},
   "bytes"=>{let row=read.child("dag_intrinsic_bytes",id)?;DslValue::Bytes(Reconstruction::new(read.control)?.blob(row.blob(2)?)?)},
   "array"=>{let rows=read.ordered("dag_intrinsic_array_element",1,id,2)?;descend(&mut pending,Frame::Array(id,rows),3,read)?;continue},
   "object"=>{let rows=read.ordered("dag_intrinsic_object_member",1,id,2)?;descend(&mut pending,Frame::Object(id,rows),4,read)?;continue},
   _=>return Err(invalid("DAG intrinsic variant differs"))};values.insert(id,value,read)?;},
  Frame::Array(id,rows)=>{let mut output=DecodedValue::new(DslValue::Array(reserve(rows.len(),read.control)?),DslValue::retire_decoded);for row in rows{let value=values.remove(row.integer(3)?,read)?;let DslValue::Array(children)=output.get_mut()else{unreachable!()};children.push(value);read.step()?;}values.insert(id,output.take(),read)?;},
  Frame::Object(id,rows)=>{let mut output=DecodedValue::new(DslValue::Object(reserve(rows.len(),read.control)?),DslValue::retire_decoded);for row in rows{let key=read.text(row,3)?;let value=values.remove(row.integer(4)?,read)?;let DslValue::Object(children)=output.get_mut()else{unreachable!()};children.push((key,value));read.step()?;}values.insert(id,output.take(),read)?;}
 }read.step()?;}
 let output=DecodedValue::new(values.remove(root,read)?,DslValue::retire_decoded);if !values.0.is_empty(){return Err(invalid("DAG intrinsic contains unowned descendants"))}Ok(output.take())
}
