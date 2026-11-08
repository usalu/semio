//! 🫴️ Typed fields retain partial destinations, performed work and every original refusal.
use crate::{FieldValue,RecordValue};
pub use semio_framework_io_control::{NativeSnapshotDecodeOwner,NativeSnapshotFailure,NativeSnapshotEffectProgress,NativeSnapshotDecodeEffectQuote,NativeSnapshotDecodeWorkQuote,NativeSnapshotEncodeWorkQuote};
use semio_framework_value::{DslValue,FromValue,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::RetireOwned};
#[path="📥️owned/🦀️.rs"]
pub mod owned;
pub use owned::{DslOwnedReceivingField,DslOwnedReceivingRecord,DslOwnedReceivingVariants,OwnedFieldReceiving,OwnedSlot};

/// 🌱️ Requires caller-owned destination custody and the actual producer work outcome.
pub trait DslReceivingField:RetireOwned+Sized{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>;
}
/// 📑️ Retains every completed field until the complete record can be published.
pub trait DslReceivingRecord:DslReceivingField{
 fn receive_record(record:&RecordValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>;
}
/// 🌿️ Dispatches literal variant identities without constructing an owned roster.
pub trait DslReceivingVariants:RetireOwned+Sized{
 fn receive_named_record(keyword:&str,record:&RecordValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>;
}
pub use semio_framework_io_control::{NativeSnapshotWork as ReceivingWork,native_snapshot_turn as receiving_turn};
/// 🎟️ Admits actual field movements through independent original copy authority.
pub fn work(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,bytes:usize,depth:usize)->Result<(),ValueError>{
 owner.native().checkpoint()?;let remaining=owner.remaining_grant();
 if remaining.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"typed receiving requires original work authority"))}
 if depth>remaining.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"typed receiving exceeds original depth authority"))}
 if bytes>remaining.maximum_copy_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"typed receiving exceeds original copy authority"))}
 owner.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"typed receiving field differs from its declared shape")}
fn refused<T>()->RetainedWorkOutcome<T,NativeSnapshotFailure>{RetainedWorkOutcome{result:Err(invalid().into()),work:0}}
macro_rules! scalar{
 ($ty:ty,$value:ident,$expression:expr)=>{impl DslReceivingField for $ty{
  fn receive_value($value:&FieldValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
   receiving_turn(|meter|{meter.admit(owner.native())?;let value=($expression).map_err(NativeSnapshotFailure::from)?;meter.take(owner.receive_copy(output,value))?;Ok(())})
  }
 }};
}
macro_rules! integer{
 ($variant:ident;$($ty:ty),+)=>{$(scalar!($ty,value,match value{FieldValue::$variant(value)=>Self::try_from(*value).map_err(|_|invalid()),_=>Err(invalid())});)+};
}
integer!(Int;i8,i16,i32,i64,isize);
integer!(UInt;u8,u16,u32,u64,usize);
scalar!(bool,value,match value{FieldValue::Bool(value)=>Ok(*value),_=>Err(invalid())});
scalar!(f64,value,match value{FieldValue::Float(value)=>Ok(*value),_=>Err(invalid())});
scalar!(f32,value,(||{let FieldValue::Float(value)=value else{return Err(invalid())};let bits=value.to_bits();if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err(invalid())}Ok(Self::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff)))}else{Ok(*value as Self)}})());
impl DslReceivingField for DslValue{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let FieldValue::Value(inner)=value else{return refused()};
  owner.receive::<DslValue,DslValue,NativeSnapshotFailure>(output,&mut |_,result,_marker,_failure,native,_|receiving_turn(|meter|{*result=Some(meter.measured(native,|native|DslValue::from_value_controlled(inner,native))?);Ok(())}))
 }
}
impl DslReceivingField for String{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let FieldValue::Text(text)=value else{return refused()};
  owner.receive::<String,String,NativeSnapshotFailure>(output,&mut |slot,result,_marker,_failure,native,body|receiving_turn(|meter|{
   let bytes=size_of::<String>();let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;meter.take(quote.run(_marker,_failure,&mut |performed,physical|{*slot=Some(String::new());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())}))?;_marker.reset_completed()?;body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_depth:1,..Default::default()})?;
   meter.measured(native,|native|body.copy_text_receiving(native,text,slot.as_mut().unwrap(),1))?;if slot.is_none()||result.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed receiving lost its retained output slot").into())}let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()})?;meter.take(quote.run(_marker,_failure,&mut |performed,physical|{*result=slot.take();*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())}))?;Ok(())
  }))
 }
}
impl<T:DslReceivingField> DslReceivingField for Option<T>{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let FieldValue::Block(inner)=value else{return refused()};let FieldValue::Record(record)=inner.as_ref()else{return refused()};Self::receive_record(record,output,owner)
 }
}
impl<T:DslReceivingField> DslReceivingRecord for Option<T>{
 fn receive_record(record:&RecordValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if record.fields.len()>1||record.fields.keys().any(|id|*id!=0){return refused()}
  match record.get(0){
   None|Some(FieldValue::Absent)=>owner.receive_none(output),
   Some(value)=>owner.receive_nested::<Option<T>,Self,NativeSnapshotFailure>(output,&mut |slot,result,_marker,_failure,original|receiving_turn(|meter|{
    let bytes=size_of::<Option<T>>();meter.take(original.effect(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()},_marker,_failure,&mut |performed,physical|{*slot=Some(None);*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())}))?;_marker.reset_completed()?;meter.take(T::receive_value(value,slot.as_mut().unwrap(),original))?;
    if slot.as_ref().is_none_or(Option::is_none)||result.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed optional lost its retained output").into())}
    meter.take(original.move_existing(slot,result,_marker,_failure))?;Ok(())
   }))
  }
 }
}
impl<T:DslReceivingField+Copy,const N:usize> DslReceivingField for [T;N]{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let FieldValue::Tuple(values)=value else{return refused()};if values.len()!=N{return refused()}
  receiving_turn(|meter|{
   let mut slots:[Option<T>;N]=std::array::from_fn(|_|None);
   for(slot,value)in slots.iter_mut().zip(values){meter.take(T::receive_value(value,slot,owner))?;}
   meter.take(owner.receive_copy(output,slots.map(|slot|slot.expect("fixed array receiving filled every slot"))))?;Ok(())
  })
 }
}
/// 🎁️ A newtype over a borrowed-family field receives its inner value in place, validates it, and wraps it in one fixed effect.
pub fn receive_wrapped<I:DslReceivingField,W:RetireOwned>(value:&FieldValue,output:&mut Option<W>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,check:fn(&I)->Result<(),ValueError>,wrap:fn(I)->W)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
 owner.receive_nested::<I,W,NativeSnapshotFailure>(output,&mut |slot,result,marker,failure,original|receiving_turn(|meter|{
  meter.take(I::receive_value(value,slot,original))?;
  check(slot.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"typed wrapper lost its retained inner value"))?)?;
  meter.take(original.effect(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},marker,failure,&mut |performed,physical|{*result=slot.take().map(wrap);*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())}))?;Ok(())
 }))
}
#[derive(semio_framework_value::RetireOwned)]
struct Sequence<T:RetireOwned>{backing:Option<Vec<T>>,child:Option<T>,effect:semio_framework_io_control::NativeSnapshotEffectProgress,unwind:Option<NativeSnapshotFailure>}
impl<T:DslReceivingField> DslReceivingField for Vec<T>{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let length=match value{FieldValue::List(values)|FieldValue::Tuple(values)=>values.len(),FieldValue::Bytes64(values)=>values.len(),_=>return refused()};
  owner.receive_nested::<Sequence<T>,Self,NativeSnapshotFailure>(output,&mut |slot,result,_marker,_failure,original|receiving_turn(|meter|{
   let bytes=size_of::<Sequence<T>>();meter.take(original.effect(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()},_marker,_failure,&mut |performed,physical|{*slot=Some(Sequence{backing:None,child:None,effect:Default::default(),unwind:None});*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())}))?;_marker.reset_completed()?;let frame=slot.as_mut().unwrap();
   meter.take(original.receive::<Vec<T>,Vec<T>,NativeSnapshotFailure>(&mut frame.backing,&mut |slot,result,_marker,_failure,native,body|receiving_turn(|meter|{
    let header=size_of::<Vec<T>>();let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:header,maximum_depth:1,..Default::default()})?;meter.take(quote.run(_marker,_failure,&mut |performed,physical|{*slot=Some(Vec::new());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:header,..Default::default()};Ok(())}))?;_marker.reset_completed()?;let bytes=length.checked_mul(size_of::<T>()).ok_or_else(invalid)?;
    body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()})?;
    meter.measured(native,|native|body.allocate_vec_receiving(native,length,slot.as_mut().unwrap(),1))?;if slot.is_none()||result.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed receiving lost its retained output slot").into())}let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()})?;meter.take(quote.run(_marker,_failure,&mut |performed,physical|{*result=slot.take();*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())}))?;Ok(())
   })))?;
   for index in 0..length{
    meter.take(match value{FieldValue::List(values)|FieldValue::Tuple(values)=>T::receive_value(&values[index],&mut frame.child,original),FieldValue::Bytes64(values)=>T::receive_value(&FieldValue::UInt(u64::from(values.retained_bytes()[index])),&mut frame.child,original),_=>unreachable!()})?;
    if frame.child.is_none()||frame.backing.as_ref().is_none_or(|values|values.len()==values.capacity()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed sequence lost its admitted destination").into())}
    let Sequence{backing,child,effect,unwind}=&mut *frame;meter.take(original.effect(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},effect,unwind,&mut |performed,physical|{backing.as_mut().unwrap().push(child.take().unwrap());*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())}))?;effect.reset_completed()?;
   }
   if frame.backing.is_none()||result.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed sequence lost its publication slot").into())}
   let Sequence{backing,effect,unwind,..}=&mut *frame;meter.take(original.move_existing(backing,result,effect,unwind))?;Ok(())
  }))
 }
}
impl<T:DslReceivingField> DslReceivingField for Box<T>{
 fn receive_value(value:&FieldValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  owner.receive_nested::<BoxFrame<T>,Self,NativeSnapshotFailure>(output,&mut |slot,result,_marker,_failure,original|receiving_turn(|meter|{
   let bytes=size_of::<BoxFrame<T>>();meter.take(original.effect(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()},_marker,_failure,&mut |performed,physical|{*slot=Some(BoxFrame{child:None,output:None});*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())}))?;_marker.reset_completed()?;let frame=slot.as_mut().unwrap();meter.take(T::receive_value(value,&mut frame.child,original))?;
   let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_capacity_bytes:size_of::<T>(),maximum_release_bytes:0,maximum_depth:1};
   if demand.maximum_capacity_bytes>original.remaining_grant().maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"typed box receiving exceeds original capacity authority").into())}
   meter.admit(original.native())?;work(original,demand.maximum_copy_bytes,1)?;original.native().charge(demand.maximum_capacity_bytes)?;
   frame.output=Some(Box::new(frame.child.take().unwrap()));
   original.record_progress(RetainedCloneProgress{retained_capacity_bytes:size_of::<T>(),..Default::default()})?;if frame.output.is_none()||result.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed box lost its retained output").into())}
   meter.take(original.move_existing(&mut frame.output,result,_marker,_failure))?;Ok(())
  }))
 }
}
#[derive(semio_framework_value::RetireOwned)]
struct BoxFrame<T:RetireOwned>{child:Option<T>,output:Option<Box<T>>}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
