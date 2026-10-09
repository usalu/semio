//! 🔭️ Synchronous reborrow preserves the original receipt on completion, refusal and unwind.
use super::{NativeDecodeControl,NativeDecodeProgress};
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,owned:&'owner mut usize,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.owned=self.control.owned_bytes;*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;}}
pub(super) fn run<T,E>(original:&mut NativeDecodeControl<'_>,observer:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
 let mut callback=|event|(*original.callback)(event)&&observer(event);
 let control=NativeDecodeControl{scope_maximum:original.scope_maximum,receiving:original.receiving,maximum_bytes:original.maximum_bytes,owned_bytes:original.owned_bytes,completed:original.completed,total:original.total,started:original.started,stage:original.stage,depth:original.depth,callback:&mut callback,retirement:original.retirement.as_deref_mut(),allocation:match &mut original.allocation{super::allocation::Binding::Local=>super::allocation::Binding::Local,super::allocation::Binding::Forwarded(port)=>super::allocation::Binding::Forwarded(&mut **port)},};
 let mut scope=Scope{control,maximum:&mut original.maximum_bytes,scope_maximum:&mut original.scope_maximum,owned:&mut original.owned_bytes,completed:&mut original.completed,total:&mut original.total,started:&mut original.started,stage:&mut original.stage,depth:&mut original.depth};
 operation(&mut scope.control)
}
