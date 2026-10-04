//! 🧺️ Controlled factories over the actual retained membership owner.
use super::*;
use crate::{DecodedValue,NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
type Result<T>=std::result::Result<T,ValueError>;
/// 🛫️ Copies actual ordered membership directly into an admitted intrinsic array.
pub(super) fn encode(value:&OrderedSet,control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(value.len())?;let mut output=Vec::<DslValue>::guard_decoded(control.allocate_vec(value.len())?);for key in value{output.get_mut().push(DslValue::String(control.copy_text(key)?));control.step()?;}Ok(DslValue::Array(output.take()))}))}
/// 🛬️ Constructs actual persistent membership with the ordered owner's paid cursor.
pub(super) fn decode(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<OrderedSet>{control.scoped_depth(64,|control|control.scoped_stage(|control|{
 let DslValue::Array(items)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"ordered native membership array required"))};
 control.begin_stage(items.len())?;
 let mut output=DecodedValue::new(OrderedMap::new(),super::super::native_controlled::retire::<()>);
 for item in items{let key=String::from_value_controlled(item,control)?;super::super::native_controlled::insert(output.get_mut(),key,(),control)?;control.step()?;}
 Ok(OrderedSet::from_map(output.take()))
}))}
