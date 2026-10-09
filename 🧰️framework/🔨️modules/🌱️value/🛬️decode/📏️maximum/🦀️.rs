//! 📏️ A borrowed incoming ceiling survives retained turns, nested observers and unwind.
use super::NativeDecodeControl;
use crate::{ValueError,ValueRefusalKind};
struct Scope<'owner,'control>{control:&'owner mut NativeDecodeControl<'control>,maximum:usize,bound:Option<usize>}
impl Drop for Scope<'_, '_>{fn drop(&mut self){self.control.maximum_bytes=self.maximum;self.control.scope_maximum=self.bound;}}
pub(super) fn run<'control,T,E:From<ValueError>>(control:&mut NativeDecodeControl<'control>,maximum:usize,operation:impl FnOnce(&mut NativeDecodeControl<'control>)->Result<T,E>)->Result<T,E>{
 let parent=control.maximum_bytes;let bound=control.scope_maximum;let narrowed=parent.min(maximum);
 if control.owned_bytes>narrowed{return Err(E::from(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decode ownership exceeds stage limit")));}
 control.maximum_bytes=narrowed;control.scope_maximum=Some(bound.map_or(narrowed,|parent|parent.min(narrowed)));
 let scope=Scope{control,maximum:parent,bound};operation(scope.control)
}
