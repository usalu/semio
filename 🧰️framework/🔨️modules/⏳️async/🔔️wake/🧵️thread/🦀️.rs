//! 🧵️ Native Thread wakes retain an original TLS-held handle and refuse closure outside its issuer thread.
use std::{cell::Cell,mem::{ManuallyDrop,size_of},sync::Arc,task::{RawWaker,RawWakerVTable,Waker}};
use semio_framework_value::{ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
thread_local!{static ISSUER:Cell<Option<std::thread::ThreadId>>=const{Cell::new(None)};}
pub struct OriginalThreadWake{thread:std::thread::Thread,id:std::thread::ThreadId}
impl std::task::Wake for OriginalThreadWake{fn wake(self:Arc<Self>){self.thread.unpark();}fn wake_by_ref(self:&Arc<Self>){self.thread.unpark();}}
struct ThreadWakeRetirement{thread:ManuallyDrop<Option<std::thread::Thread>>,id:Option<std::thread::ThreadId>}
impl RetireOwned for OriginalThreadWake{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(ThreadWakeRetirement{thread:ManuallyDrop::new(Some(self.thread)),id:Some(self.id)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ThreadWakeRetirement>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
impl RetirementCursor for ThreadWakeRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete}
  let copy=self.next_work_byte_demand().unwrap();
  if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return RetirementStep::BudgetExhausted}
  if !ISSUER.with(|issuer|issuer.get()==self.id){return RetirementStep::BudgetExhausted}
  drop(self.thread.take());self.id.take();RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})
 }
 fn terminal_is_empty(&self)->bool{self.thread.is_none()&&self.id.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.terminal_is_empty(){0}else{size_of::<Option<std::thread::Thread>>()+size_of::<std::thread::Thread>()+size_of::<Option<std::thread::ThreadId>>()})}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for ThreadWakeRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original thread handle abandoned its issuer TLS lease");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.thread);}}}}
fn raw(original:Arc<OriginalThreadWake>)->RawWaker{RawWaker::new(Arc::into_raw(original).cast(),&VTABLE)}
static VTABLE:RawWakerVTable=RawWakerVTable::new(clone,wake,wake_by_ref,release);
unsafe fn clone(pointer:*const())->RawWaker{unsafe{Arc::increment_strong_count(pointer.cast::<OriginalThreadWake>());}RawWaker::new(pointer,&VTABLE)}
unsafe fn wake(pointer:*const()){std::task::Wake::wake(unsafe{Arc::from_raw(pointer.cast::<OriginalThreadWake>())});}
unsafe fn wake_by_ref(pointer:*const()){unsafe{&*pointer.cast::<OriginalThreadWake>()}.thread.unpark();}
unsafe fn release(pointer:*const()){drop(unsafe{Arc::from_raw(pointer.cast::<OriginalThreadWake>())});}

/// 🪪️ Creates the concrete original wake for an already initialized native caller thread.
pub fn original_thread_waker()->Waker{let thread=std::thread::current();let id=thread.id();ISSUER.with(|issuer|issuer.set(Some(id)));unsafe{Waker::from_raw(raw(Arc::new(OriginalThreadWake{thread,id})))}}
/// 🎟️ Borrows the original lease quote without interpreting any foreign std Waker allocation.
pub const fn original_thread_wake_lease_copy_bytes()->usize{size_of::<Arc<OriginalThreadWake>>()}
/// 🔎️ Borrows the exact first-party vtable identity without obtaining a wake lease.
pub fn original_thread_wake_is_issued(original:&Waker)->bool{std::ptr::eq(original.vtable(),&VTABLE)}
/// 🔗️ Retains only this native issuer's original Arc lease after full admission.
pub fn admit_original_thread_wake(original:&Waker,grant:RetainedCloneGrant)->Result<Option<(Arc<OriginalThreadWake>,RetainedCloneProgress)>,ValueError>{
 if !original_thread_wake_is_issued(original){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"foreign wake has no original native Thread authority"))}
 let copy=original_thread_wake_lease_copy_bytes();if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None)}
 let pointer=original.data().cast::<OriginalThreadWake>();unsafe{Arc::increment_strong_count(pointer);}let source=unsafe{Arc::from_raw(pointer)};Ok(Some((source,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})))
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
