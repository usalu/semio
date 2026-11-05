//! 🫴️ Typed fields retain partial destinations, performed work and every original refusal.
use crate::{FieldValue,RecordValue};
pub use semio_framework_io_control::{NativeSnapshotDecodeOwner,NativeSnapshotFailure};
use semio_framework_value::{DslValue,FromValue,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::RetireOwned};
#[path="📥️owned/🦀️.rs"]
pub mod owned;
pub use owned::{DslOwnedReceivingField,DslOwnedReceivingRecord,DslOwnedReceivingVariants,OwnedFieldReceiving,OwnedSlot};

/// 🌱️ Requires original destination custody and the actual producer work outcome.
pub trait DslReceivingField:RetireOwned+Sized{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>;
}
/// 📑️ Retains every completed field until the complete record can be published.
pub trait DslReceivingRecord:DslReceivingField{
 fn receive_record(record:&RecordValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>;
}
/// 🌿️ Dispatches literal variant identities without constructing an owned roster.
pub trait DslReceivingVariants:RetireOwned+Sized{
 fn receive_named_record(keyword:&str,record:&RecordValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>;
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
  fn receive_value($value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
   receiving_turn(|meter|{meter.admit(owner.native())?;let value=($expression).map_err(NativeSnapshotFailure::from)?;work(owner,size_of::<Self>(),1)?;Ok(value)})
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
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
  let FieldValue::Value(inner)=value else{return refused()};
  owner.receive::<DslValue,DslValue,NativeSnapshotFailure>(|_,native,_|receiving_turn(|meter|{native.checkpoint()?;meter.performed();DslValue::from_value_controlled(inner,native).map_err(Into::into)}))
 }
}
impl DslReceivingField for String{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
  let FieldValue::Text(text)=value else{return refused()};
  owner.receive::<String,String,NativeSnapshotFailure>(|slot,native,body|receiving_turn(|meter|{
   *slot=Some(String::new());body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_depth:1,..Default::default()})?;
   native.checkpoint()?;meter.performed();body.copy_text_into(native,text,slot.as_mut().unwrap(),1)?;Ok(slot.take().unwrap())
  }))
 }
}
impl<T:DslReceivingField> DslReceivingField for Option<T>{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
  let FieldValue::Block(inner)=value else{return refused()};let FieldValue::Record(record)=inner.as_ref()else{return refused()};Self::receive_record(record,owner)
 }
}
impl<T:DslReceivingField> DslReceivingRecord for Option<T>{
 fn receive_record(record:&RecordValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
  if record.fields.len()>1||record.fields.keys().any(|id|*id!=0){return refused()}
  match record.get(0){
   None|Some(FieldValue::Absent)=>receiving_turn(|meter|{meter.admit(owner.native())?;work(owner,size_of::<Self>(),1)?;Ok(None)}),
   Some(value)=>owner.receive_nested::<Option<T>,Self,NativeSnapshotFailure>(|slot,original|receiving_turn(|meter|{
    *slot=Some(None);*slot.as_mut().unwrap()=Some(meter.take(T::receive_value(value,original))?);
    meter.admit(original.native())?;work(original,size_of::<Self>(),1)?;Ok(slot.take().unwrap())
   }))
  }
 }
}
impl<T:DslReceivingField+Copy,const N:usize> DslReceivingField for [T;N]{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
  let FieldValue::Tuple(values)=value else{return refused()};if values.len()!=N{return refused()}
  receiving_turn(|meter|{
   meter.admit(owner.native())?;work(owner,size_of::<Self>(),1)?;
   let mut slots:[Option<T>;N]=std::array::from_fn(|_|None);
   for(slot,value)in slots.iter_mut().zip(values){*slot=Some(meter.take(T::receive_value(value,owner))?);}
   Ok(slots.map(|slot|slot.expect("fixed array receiving filled every slot")))
  })
 }
}
#[derive(semio_framework_value::RetireOwned)]
struct Sequence<T:RetireOwned>{values:Vec<T>,child:Option<T>}
impl<T:DslReceivingField> DslReceivingField for Vec<T>{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
  let length=match value{FieldValue::List(values)|FieldValue::Tuple(values)=>values.len(),FieldValue::Bytes64(values)=>values.len(),_=>return refused()};
  owner.receive_nested::<Sequence<T>,Self,NativeSnapshotFailure>(|slot,original|receiving_turn(|meter|{
   *slot=Some(Sequence{values:Vec::new(),child:None});
   let backing=original.receive::<Vec<T>,Vec<T>,NativeSnapshotFailure>(|slot,native,body|receiving_turn(|meter|{
    *slot=Some(Vec::new());let bytes=length.checked_mul(size_of::<T>()).ok_or_else(invalid)?;
    body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()})?;
    native.checkpoint()?;meter.performed();body.allocate_vec_into(native,length,slot.as_mut().unwrap(),1)?;Ok(slot.take().unwrap())
   }));
   slot.as_mut().unwrap().values=meter.take(backing)?;
   for index in 0..length{
    let frame=slot.as_mut().unwrap();
    frame.child=Some(meter.take(match value{FieldValue::List(values)|FieldValue::Tuple(values)=>T::receive_value(&values[index],original),FieldValue::Bytes64(values)=>T::receive_value(&FieldValue::UInt(u64::from(values.retained_bytes()[index])),original),_=>unreachable!()})?);
    meter.admit(original.native())?;work(original,size_of::<T>(),1)?;frame.values.push(frame.child.take().unwrap());
   }
   meter.admit(original.native())?;work(original,size_of::<Self>(),1)?;Ok(std::mem::take(&mut slot.as_mut().unwrap().values))
  }))
 }
}
impl<T:DslReceivingField> DslReceivingField for Box<T>{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
  owner.receive_nested::<BoxFrame<T>,Self,NativeSnapshotFailure>(|slot,original|receiving_turn(|meter|{
   *slot=Some(BoxFrame{child:None,output:None});slot.as_mut().unwrap().child=Some(meter.take(T::receive_value(value,original))?);
   let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_capacity_bytes:size_of::<T>(),maximum_release_bytes:0,maximum_depth:1};
   if demand.maximum_capacity_bytes>original.remaining_grant().maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"typed box receiving exceeds original capacity authority").into())}
   meter.admit(original.native())?;work(original,demand.maximum_copy_bytes,1)?;original.native().charge(demand.maximum_capacity_bytes)?;
   let frame=slot.as_mut().unwrap();frame.output=Some(Box::new(frame.child.take().unwrap()));
   original.record_progress(RetainedCloneProgress{retained_capacity_bytes:size_of::<T>(),..Default::default()})?;Ok(frame.output.take().unwrap())
  }))
 }
}
#[derive(semio_framework_value::RetireOwned)]
struct BoxFrame<T:RetireOwned>{child:Option<T>,output:Option<Box<T>>}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
