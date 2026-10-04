//! 🌿️ Iterative actual property variants retain all owned backing and partial values.
use super::{PropertyBag,PropertyValue};
use semio_framework_value::{DecodedValue,DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,Number,ValueError,ValueRefusalKind};
use semio_framework_dsl_record::NativeSchemaControl;
fn grow<T,C:NativeSchemaControl>(values:&mut Vec<T>,control:&mut C)->Result<(),ValueError>{if values.len()==values.capacity(){let count=values.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"property frontier overflow"))?;let mut next=control.allocate_vec(count)?;for _ in 0..values.len(){control.step()?;}next.extend(std::mem::take(values));*values=next;}Ok(())}
fn push<T:FromValue,C:NativeSchemaControl>(values:&mut Vec<T>,value:T,control:&mut C)->Result<(),ValueError>{let value=DecodedValue::new(value,T::retire_decoded);grow(values,control)?;values.push(value.take());Ok(())}
fn retire_values<T:FromValue>(values:Vec<T>){for value in values{T::retire_decoded(value)}}
enum Decode<'a>{Enter(&'a DslValue),Array(usize),Object(&'a[(String,DslValue)])}
pub(super) fn decode(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<PropertyValue,ValueError>{
 control.scoped_stage(|control|{
  control.begin_stage(0)?;let mut pending=control.allocate_vec(1)?;pending.push(Decode::Enter(value));let mut values=DecodedValue::new(Vec::new(),retire_values::<PropertyValue>);
  while let Some(frame)=pending.pop(){match frame{
   Decode::Enter(value)=>{let value=match value{
    DslValue::Null=>PropertyValue::Null,DslValue::Bool(value)=>PropertyValue::Bool(*value),DslValue::Number(value)=>PropertyValue::Number(value.as_f64()),DslValue::String(value)=>PropertyValue::String(control.copy_text(value)?),
    DslValue::Bytes(bytes)=>{let mut output=control.allocate_vec(bytes.len())?;for byte in bytes{output.push(PropertyValue::Number(f64::from(*byte)));control.step()?;}PropertyValue::Array(output)},
    DslValue::Array(items)=>{grow(&mut pending,control)?;pending.push(Decode::Array(items.len()));for value in items.iter().rev(){grow(&mut pending,control)?;pending.push(Decode::Enter(value));control.step()?;}continue},
    DslValue::Object(items)=>{grow(&mut pending,control)?;pending.push(Decode::Object(items));for(_,value)in items.iter().rev(){grow(&mut pending,control)?;pending.push(Decode::Enter(value));control.step()?;}continue}
   };push(values.get_mut(),value,control)?;},
   Decode::Array(count)=>{let mut output=DecodedValue::new(PropertyValue::Array(control.allocate_vec(count)?),retire);let PropertyValue::Array(items)=output.get_mut()else{unreachable!()};for _ in 0..count{items.push(PropertyValue::Null);control.step()?;}for index in(0..count).rev(){items[index]=values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"property child missing"))?;control.step()?;}push(values.get_mut(),output.take(),control)?;},
   Decode::Object(fields)=>{let mut children=DecodedValue::new(control.allocate_vec(fields.len())?,retire_values::<PropertyValue>);for _ in fields{children.get_mut().push(PropertyValue::Null);control.step()?;}for index in(0..fields.len()).rev(){children.get_mut()[index]=values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"property child missing"))?;control.step()?;}let mut output=DecodedValue::new(PropertyBag::from_admitted(control.allocate_vec(fields.len())?),PropertyBag::retire_decoded);for(index,(key,_))in fields.iter().enumerate(){let key=control.copy_text(key)?;let value=std::mem::take(&mut children.get_mut()[index]);output.get_mut().admitted_insert(key,value,control)?;}push(values.get_mut(),PropertyValue::Object(output.take()),control)?;}
  }control.step()?;}
  if values.get_mut().len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"property root cardinality differs"))}control.checkpoint()?;Ok(values.get_mut().pop().unwrap())
 })
}
enum Encode<'a>{Enter(&'a PropertyValue),Array(usize),Object(&'a PropertyBag)}
pub(super) fn encode(value:&PropertyValue,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
 control.scoped_stage(|control|{
  control.begin_stage(0)?;let mut pending=control.allocate_vec(1)?;pending.push(Encode::Enter(value));let mut values=DecodedValue::new(Vec::new(),retire_values::<DslValue>);
  while let Some(frame)=pending.pop(){match frame{
   Encode::Enter(value)=>{let value=match value{
    PropertyValue::Null=>DslValue::Null,PropertyValue::Bool(value)=>DslValue::Bool(*value),PropertyValue::Number(value)=>DslValue::Number(Number::Float(*value)),PropertyValue::String(value)=>DslValue::String(control.copy_text(value)?),
    PropertyValue::Array(items)=>{grow(&mut pending,control)?;pending.push(Encode::Array(items.len()));for value in items.iter().rev(){grow(&mut pending,control)?;pending.push(Encode::Enter(value));control.step()?;}continue},
    PropertyValue::Object(items)=>{grow(&mut pending,control)?;pending.push(Encode::Object(items));for(_,value)in items.iter().rev(){grow(&mut pending,control)?;pending.push(Encode::Enter(value));control.step()?;}continue}
   };push(values.get_mut(),value,control)?;},
   Encode::Array(count)=>{let mut output=DslValue::Array(control.allocate_vec(count)?).guard_encoded();let DslValue::Array(items)=output.get_mut()else{unreachable!()};for _ in 0..count{items.push(DslValue::Null);control.step()?;}for index in(0..count).rev(){items[index]=values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"property child missing"))?;control.step()?;}push(values.get_mut(),output.take(),control)?;},
   Encode::Object(fields)=>{let mut output=DslValue::object_encoding_controlled(fields.len(),control)?;for key in fields.keys(){let key=control.copy_text(key)?;output.get_mut().push((key,DslValue::Null));control.step()?;}for index in(0..fields.len()).rev(){output.get_mut()[index].1=values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"property child missing"))?;control.step()?;}push(values.get_mut(),DslValue::Object(output.take()),control)?;}
  }control.step()?;}
  if values.get_mut().len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"property root cardinality differs"))}control.checkpoint()?;Ok(values.get_mut().pop().unwrap())
 })
}
pub(super) fn retire(value:PropertyValue){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !matches!(cursor.close_step(256,usize::MAX).expect("property retirement failed"),semio_framework_value::SnapshotRetirementStep::Complete){}assert!(cursor.terminal_is_empty());}
