//! 🗂️ Controlled native factories for the actual persistent ordered owner.
use super::*;
use crate::{DecodedValue,DslValue,FromValue,NativeEncodeControl,ToValue,ValueError};
type Result<T>=std::result::Result<T,ValueError>;
/// 🛫️ Visits the actual ordered root and admits every owned output cell.
pub(super) fn encode<V:ToValue>(value:&OrderedMap<V>,control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(value.len())?;let mut output=DecodedValue::new(control.allocate_vec::<(String,DslValue)>(value.len())?,|items|{for(_,value)in items{DslValue::retire_decoded(value)}});for(key,value)in value{let key=control.copy_text(key)?;let value=value.to_value_controlled(control)?;output.get_mut().push((key,value));control.step()?;}Ok(DslValue::Object(output.take()))}))}
