//! 🗂️ Controlled native factories for the actual persistent ordered owner.
use super::*;
use crate::{DecodedValue,DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,ToValue,ValueError,ValueRefusalKind};
type Result<T>=std::result::Result<T,ValueError>;
/// 🛫️ Visits the actual ordered root and admits every owned output cell.
pub(super) fn encode<V:ToValue>(value:&OrderedMap<V>,control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(value.len())?;let mut output=DecodedValue::new(control.allocate_vec::<(String,DslValue)>(value.len())?,|items|{for(_,value)in items{DslValue::retire_decoded(value)}});for(key,value)in value{let key=control.copy_text(key)?;let value=value.to_value_controlled(control)?;output.get_mut().push((key,value));control.step()?;}Ok(DslValue::Object(output.take()))}))}
/// ♻️ Releases actual node ownership and dispatches each final value to its typed retirement.
pub(super) fn retire<V:FromValue>(value:OrderedMap<V>){let mut cursor=value.retire();loop{let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().expect("ordered native cold retirement demand"),maximum_depth:cursor.next_depth_demand()};match cursor.advance(grant){RetirementStep::OwnedValue(value)=>V::retire_decoded(value),RetirementStep::Complete=>break,RetirementStep::Failure(error)=>panic!("ordered native retirement refused: {error}"),_=>{}}}}
/// 🌱️ Returns the allocation-free actual empty root after cancellation admission.
pub(super) fn empty<V>(control:&mut NativeDecodeControl<'_>)->Result<OrderedMap<V>>{control.checkpoint()?;Ok(OrderedMap::new())}
/// 🛂️ Admits the concrete shared key/value and the owner's known node/path frontiers.
pub(super) fn insert<V:FromValue>(map:&mut OrderedMap<V>,key:String,value:V,control:&mut NativeDecodeControl<'_>)->Result<()>{
 let value=value.guard_decoded();
 let metadata=SharedOwner::<String>::allocation_bytes().checked_add(SharedOwner::<V>::allocation_bytes()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ordered native shared payload backing overflow"))?;
 control.charge(metadata)?;
 let mut cursor=map.begin_set(key,value.take());
 let mut removed=None;
 let result=(||{while !cursor.is_complete(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:cursor.next_capacity_byte_demand()?,maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand()};cursor.advance_insert_controlled(grant,control)?;}let replacement=cursor.take_result().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"ordered native result missing after completion"))?;removed=cursor.take_removed();retire(std::mem::replace(map,replacement));Ok(())})();
 cursor.begin_close();
 loop{let physical=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().expect("ordered native update close demand"),maximum_depth:cursor.next_close_depth_demand()};match cursor.close_step(physical){RetirementStep::OwnedValue(value)=>V::retire_decoded(value),RetirementStep::Complete=>break,RetirementStep::Failure(error)=>panic!("ordered native close refused: {error}"),_=>{}}}
 if let Some(mut value)=removed{if let Some(value)=value.release_step(RetainedCloneGrant::one_release_turn(value.next_release_byte_demand(),1)).expect("ordered native removed header release").value{V::retire_decoded(value);}}
 result
}
/// 🛬️ Constructs the actual persistent map directly from borrowed intrinsic fields.
pub(super) fn decode<V:FromValue>(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<OrderedMap<V>>{control.scoped_depth(64,|control|control.scoped_stage(|control|{
 let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"ordered native object required"))};
 control.begin_stage(fields.len())?;
 let mut output=DecodedValue::new(OrderedMap::new(),retire::<V>);
 for(key,value)in fields{let key=control.copy_text(key)?;let value=V::from_value_controlled(value,control)?;insert(output.get_mut(),key,value,control)?;control.step()?;}
 Ok(output.take())
}))}
