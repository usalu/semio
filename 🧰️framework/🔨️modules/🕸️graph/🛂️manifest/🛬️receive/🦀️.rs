//! 🛬️ Graph properties receive their dynamic subtree once, quoted exactly, as sorted contiguous members.
use super::{PropertyBag,PropertyValue};
use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::{NativeDefault,tree::{NativeValueSource,bulk::{NativeBulkKind,NativeBulkReceiver,NativeBulkValue,NATIVE_BULK_MAXIMUM_DEPTH,native_bulk_kind,native_bulk_measure}}}};
use std::mem::size_of;

fn refusal(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,text)}
fn quote(original:&dyn NativeValueSource)->Result<RetirementDemand,ValueError>{
 let mut capacity=0;
 native_bulk_measure(original,0,size_of::<PropertyValue>(),size_of::<(String,PropertyValue)>(),&mut capacity)?;
 Ok(RetirementDemand{copy_bytes:size_of::<PropertyValue>(),capacity_bytes:capacity,depth:1,..Default::default()})
}
fn value(original:&dyn NativeValueSource,depth:usize,control:&mut NativeDecodeControl<'_>)->Result<PropertyValue,ValueError>{
 if depth>NATIVE_BULK_MAXIMUM_DEPTH{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"graph property subtree exceeds depth limit"))}
 control.step()?;
 Ok(match native_bulk_kind(original)?{
  NativeBulkKind::Null=>PropertyValue::Null,
  NativeBulkKind::Bool=>PropertyValue::Bool(original.boolean()?),
  NativeBulkKind::Number=>PropertyValue::Number(original.number()?.as_f64()),
  NativeBulkKind::Text=>PropertyValue::String(control.copy_text(original.text()?)?),
  NativeBulkKind::Array=>{let count=original.sequence_len()?;let mut items=control.allocate_vec::<PropertyValue>(count)?;for index in 0..count{items.push(value(original.sequence_child(index)?,depth+1,control)?)}PropertyValue::Array(items)}
  NativeBulkKind::Object=>PropertyValue::Object(bag(original,depth,control)?),
 })
}
fn bag(original:&dyn NativeValueSource,depth:usize,control:&mut NativeDecodeControl<'_>)->Result<PropertyBag,ValueError>{
 let count=original.record_len()?;
 let mut members=control.allocate_vec::<(String,PropertyValue)>(count)?;
 for index in 0..count{let key=control.copy_text(original.record_name(index)?)?;members.push((key,value(original.record_child(index)?,depth+1,control)?))}
 members.sort_unstable_by(|left,right|left.0.cmp(&right.0));
 if members.windows(2).any(|pair|pair[0].0==pair[1].0){return Err(refusal("graph property object repeats a key"))}
 Ok(PropertyBag::from_admitted(members))
}
impl<I:NativeValueSource+?Sized> NativeBulkValue<I> for PropertyValue{
 fn read_demand(original:&I)->Result<RetirementDemand,ValueError>{quote(original.as_source())}
 fn read(original:&I,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{value(original.as_source(),0,control)}
}
impl<I:NativeValueSource+?Sized> NativeBulkValue<I> for PropertyBag{
 fn read_demand(original:&I)->Result<RetirementDemand,ValueError>{quote(original.as_source())}
 fn read(original:&I,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{bag(original.as_source(),0,control)}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for PropertyValue{
 type Receiver=NativeBulkReceiver<I,Self>;
 fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth()}
 fn begin_receive(grant:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(grant)}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for PropertyBag{
 type Receiver=NativeBulkReceiver<I,Self>;
 fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth()}
 fn begin_receive(grant:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(grant)}
}
impl NativeDefault for PropertyBag{
 fn native_default_demand()->RetirementDemand{semio_framework_value_native_owned::native_decoding::owned::tree::fixed(2*size_of::<Self>())}
 fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{
  let demand=Self::native_default_demand();
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Err(refusal("graph property default exceeds its grant"))}
  Ok((Self::new(),semio_framework_value_native_owned::native_decoding::owned::tree::receipt(demand)))
 }
}
