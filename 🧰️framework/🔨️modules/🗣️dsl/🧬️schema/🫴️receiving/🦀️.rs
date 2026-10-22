//! 🫴️ Typed fields retain every partial destination inside the original native receiving owner.
use crate::{FieldValue,RecordValue};
pub use semio_framework_io_control::NativeSnapshotDecodeOwner;
use semio_framework_value::{DslValue,FromValue,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind,retirement::RetireOwned};
#[path="📥️owned/🦀️.rs"]
pub mod owned;
pub use owned::{DslOwnedReceivingField,DslOwnedReceivingRecord,DslOwnedReceivingVariants,OwnedFieldReceiving,OwnedSlot};

/// 🌱️ Requires an actual original destination implementation rather than a cold typed binder.
pub trait DslReceivingField:RetireOwned+Sized{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>;
}
/// 📑️ Binds an original borrowed record while retaining every completed typed field.
pub trait DslReceivingRecord:DslReceivingField{
 fn receive_record(record:&RecordValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>;
}
/// 🌿️ Dispatches literal variant identities without constructing an owned metadata roster.
pub trait DslReceivingVariants:RetireOwned+Sized{
 fn receive_named_record(keyword:&str,record:&RecordValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>;
}
/// 🎟️ Admits actual field movements through the original callback and independent copy currency.
pub fn work(owner:&mut NativeSnapshotDecodeOwner<'_,'_>,bytes:usize,depth:usize)->Result<(),ValueError>{
 owner.native().checkpoint()?;let remaining=owner.remaining_grant();
 if remaining.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"typed receiving requires original work authority"))}
 if depth>remaining.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"typed receiving exceeds original depth authority"))}
 if bytes>remaining.maximum_copy_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"typed receiving exceeds original copy authority"))}
 owner.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"typed receiving field differs from its declared shape")}
macro_rules! integer{
 ($variant:ident;$($ty:ty),+)=>{$(impl DslReceivingField for $ty{fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{work(owner,size_of::<Self>(),1)?;match value{FieldValue::$variant(value)=>Self::try_from(*value).map_err(|_|invalid()),_=>Err(invalid())}}})+};
}
integer!(Int;i8,i16,i32,i64,isize);
integer!(UInt;u8,u16,u32,u64,usize);
impl DslReceivingField for bool{fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{work(owner,size_of::<Self>(),1)?;match value{FieldValue::Bool(value)=>Ok(*value),_=>Err(invalid())}}}
impl DslReceivingField for f64{fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{work(owner,size_of::<Self>(),1)?;match value{FieldValue::Float(value)=>Ok(*value),_=>Err(invalid())}}}
impl DslReceivingField for f32{fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{work(owner,size_of::<Self>(),1)?;let FieldValue::Float(value)=value else{return Err(invalid())};let bits=value.to_bits();if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err(invalid())}Ok(Self::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff)))}else{Ok(*value as Self)}}}
impl DslReceivingField for DslValue{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{let FieldValue::Value(inner)=value else{return Err(invalid())};owner.receive::<DslValue,DslValue>(|_,native,_|DslValue::from_value_controlled(inner,native))}
}
impl DslReceivingField for String{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{let FieldValue::Text(text)=value else{return Err(invalid())};owner.receive::<String,String>(|slot,native,body|{*slot=Some(String::new());body.copy_text_into(native,text,slot.as_mut().unwrap(),1)?;Ok(slot.take().unwrap())})}
}
impl<T:DslReceivingField> DslReceivingField for Option<T>{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{let FieldValue::Block(inner)=value else{return Err(invalid())};let FieldValue::Record(record)=inner.as_ref()else{return Err(invalid())};Self::receive_record(record,owner)}
}
impl<T:DslReceivingField> DslReceivingRecord for Option<T>{
 fn receive_record(record:&RecordValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{if record.fields.len()>1||record.fields.keys().any(|id|*id!=0){return Err(invalid())}match record.get(0){None|Some(FieldValue::Absent)=>{work(owner,size_of::<Self>(),1)?;Ok(None)},Some(value)=>owner.receive_nested::<Option<T>,Self>(|slot,original|{*slot=Some(None);*slot.as_mut().unwrap()=Some(T::receive_value(value,original)?);work(original,size_of::<Self>(),1)?;Ok(slot.take().unwrap())})}}
}
#[derive(semio_framework_value::RetireOwned)]
struct Sequence<T:RetireOwned>{values:Vec<T>,child:Option<T>}
impl<T:DslReceivingField> DslReceivingField for Vec<T>{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{
  let length=match value{FieldValue::List(values)|FieldValue::Tuple(values)=>values.len(),FieldValue::Bytes64(values)=>values.len(),_=>return Err(invalid())};
  owner.receive_nested::<Sequence<T>,Self>(|slot,original|{
   *slot=Some(Sequence{values:Vec::new(),child:None});
   let backing=original.receive::<Vec<T>,Vec<T>>(|slot,native,body|{*slot=Some(Vec::new());body.allocate_vec_into(native,length,slot.as_mut().unwrap(),1)?;Ok(slot.take().unwrap())})?;slot.as_mut().unwrap().values=backing;
   for index in 0..length{
    let frame=slot.as_mut().unwrap();frame.child=Some(match value{FieldValue::List(values)|FieldValue::Tuple(values)=>T::receive_value(&values[index],original)?,FieldValue::Bytes64(values)=>T::receive_value(&FieldValue::UInt(u64::from(values[index])),original)?,_=>unreachable!()});
    work(original,size_of::<T>(),1)?;frame.values.push(frame.child.take().unwrap());
   }
   work(original,size_of::<Self>(),1)?;Ok(std::mem::take(&mut slot.as_mut().unwrap().values))
  })
 }
}
impl<T:DslReceivingField> DslReceivingField for Box<T>{
 fn receive_value(value:&FieldValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{owner.receive_nested::<BoxFrame<T>,Self>(|slot,original|{*slot=Some(BoxFrame{child:None,output:None});slot.as_mut().unwrap().child=Some(T::receive_value(value,original)?);let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_capacity_bytes:size_of::<T>(),maximum_release_bytes:0,maximum_depth:1};let remaining=original.remaining_grant();if demand.maximum_capacity_bytes>remaining.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"typed box receiving exceeds original capacity authority"))}work(original,demand.maximum_copy_bytes,1)?;original.native().charge(demand.maximum_capacity_bytes)?;let frame=slot.as_mut().unwrap();frame.output=Some(Box::new(frame.child.take().unwrap()));original.record_progress(RetainedCloneProgress{retained_capacity_bytes:size_of::<T>(),..Default::default()})?;Ok(frame.output.take().unwrap())})}
}
#[derive(semio_framework_value::RetireOwned)]
struct BoxFrame<T:RetireOwned>{child:Option<T>,output:Option<Box<T>>}

