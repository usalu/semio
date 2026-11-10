//! 🔭️ Original native observer scope reborrows its actual control.
#[macro_export]
macro_rules! original_native_decode_observer { () => {
use super::{NativeDecodeControl,NativeDecodeProgress};
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,ledger:&'owner mut Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,work_state:&'owner mut super::work::WorkState}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.ledger=self.control.ledger.take();*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.work_state=self.control.work.state.take();}}
pub(super) fn run<T,E>(original:&mut NativeDecodeControl<'_>,observer:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
 let mut callback=|event|(*original.callback)(event)&&observer(event);
 let mut forwarded=match &mut original.allocation{super::allocation::Binding::Local=>None,super::allocation::Binding::Forwarded(port)=>Some(&mut **port)};
 let mut construction=original.construction.as_mut().map(|loan|loan.reborrow());
 let mut work_loan=original.work.recipient.as_mut().map(|loan|loan.reborrow());
 let control=NativeDecodeControl{work:super::work::Work{state:original.work.state.take(),recipient:work_loan.as_mut()},ledger:original.ledger.take(),scope_maximum:original.scope_maximum,construction:construction.as_mut(),receiving:original.receiving,maximum_bytes:original.maximum_bytes,completed:original.completed,total:original.total,started:original.started,stage:original.stage,depth:original.depth,callback:&mut callback,source:original.source,retirement:original.retirement.as_deref_mut(),allocation:match forwarded.as_mut(){Some(port)=>super::allocation::Binding::Forwarded(&mut **port),None=>super::allocation::Binding::Local},};
 let mut scope=Scope{control,maximum:&mut original.maximum_bytes,scope_maximum:&mut original.scope_maximum,ledger:&mut original.ledger,completed:&mut original.completed,total:&mut original.total,started:&mut original.started,stage:&mut original.stage,depth:&mut original.depth,work_state:&mut original.work.state};
 operation(&mut scope.control)
}

} }
