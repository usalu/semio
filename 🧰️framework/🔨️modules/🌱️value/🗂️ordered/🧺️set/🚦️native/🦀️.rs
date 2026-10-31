//! 🧺️ Controlled factories over the actual retained membership owner.
use super::*;
use crate::{NativeEncodeControl};
type Result<T>=std::result::Result<T,ValueError>;
/// 🛫️ Copies actual ordered membership directly into an admitted intrinsic array.
pub(super) fn encode(value:&OrderedSet,control:&mut NativeEncodeControl<'_>)->Result<DslValue>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(value.len())?;let mut output=Vec::<DslValue>::guard_decoded(control.allocate_vec(value.len())?);for key in value{output.get_mut().push(DslValue::String(control.copy_text(key)?));control.step()?;}Ok(DslValue::Array(output.take()))}))}
