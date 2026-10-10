//! 🔐️ Returns each actual permit alias through its original pre-admitted claim slot.
use semio_framework_value::{ValueError, ValueRefusalKind, retirement::{RetireOwned, RetirementCursor, RetirementStep}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use std::sync::{Arc, atomic::{AtomicU8,AtomicPtr,Ordering}};
pub struct PublicationClaim { state: AtomicU8, returned: AtomicPtr<PublicationClaim> }
pub struct PublicationClaimHandle { inner: Arc<PublicationClaim> }
impl PublicationClaim {
 pub fn new()->PublicationClaimHandle { PublicationClaimHandle { inner:Arc::new(Self { state:AtomicU8::new(0),returned:AtomicPtr::new(std::ptr::null_mut()) }) } }
 pub fn cancel(&self) { self.state.fetch_or(2,Ordering::AcqRel); }
 pub fn is_cancelled(&self)->bool { self.state.load(Ordering::Acquire)&2!=0 }
 pub fn finish(&self) { self.state.fetch_or(4,Ordering::AcqRel); }
 pub fn is_finished(&self)->bool { self.state.load(Ordering::Acquire)&4!=0 }
 fn is_claimed(&self)->bool { self.state.load(Ordering::Acquire)&1!=0 }
}
impl PublicationClaimHandle {
 pub fn same_original(left:&Self,right:&Self)->bool { Arc::ptr_eq(&left.inner,&right.inner) }
 fn frame_bytes()->usize { semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<PublicationClaim>() }
 fn has_returned_alias(&self)->bool { !self.inner.returned.load(Ordering::Acquire).is_null() }
 fn drain_returned_alias(&self)->bool {
  let pointer=self.inner.returned.swap(std::ptr::null_mut(),Ordering::AcqRel);
  if pointer.is_null(){return false;}
  assert_eq!(pointer.cast_const(),Arc::as_ptr(&self.inner),"original claim return belongs to the live canonical source");
  unsafe{drop(Arc::from_raw(pointer));}true
 }
 pub fn try_claim(&self)->Option<PublicationPermit> {
  self.drain_returned_alias();
  self.inner.state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).ok()?;
  Some(PublicationPermit { claim:Some(self.clone()) })
 }
 fn return_permit(self) {
  let pointer=Arc::into_raw(self.inner).cast_mut();
  let original=unsafe{&*pointer};
  assert!(original.returned.compare_exchange(std::ptr::null_mut(),pointer,Ordering::Release,Ordering::Relaxed).is_ok(),"sole original permit owns its pre-admitted return slot");
  let previous=original.state.fetch_and(!1,Ordering::AcqRel);
  assert_ne!(previous&1,0,"original permit returns before releasing its claim bit");
 }
 fn return_original(self)->Option<PublicationClaim> { Arc::into_inner(self.inner) }
}
impl Clone for PublicationClaimHandle { fn clone(&self)->Self { Self { inner:Arc::clone(&self.inner) } } }
impl std::ops::Deref for PublicationClaimHandle { type Target=PublicationClaim; fn deref(&self)->&PublicationClaim { &self.inner } }
pub struct PublicationPermit { claim:Option<PublicationClaimHandle> }
impl Drop for PublicationPermit { fn drop(&mut self){if let Some(original)=self.claim.take(){original.return_permit();}} }
struct ClaimRetirement { source:Option<PublicationClaimHandle> }
impl RetirementCursor for ClaimRetirement {
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {
  let Some(source)=self.source.as_ref()else{return RetirementStep::Complete;};
  if source.is_claimed(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original publication claim still has its permit"));}
  let returned=source.has_returned_alias();let copy=if returned{std::mem::size_of::<usize>()}else{std::mem::size_of::<PublicationClaim>()};let release=if returned{0}else{PublicationClaimHandle::frame_bytes()};
  if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release||grant.maximum_depth==0{return RetirementStep::BudgetExhausted;}
  if returned {source.drain_returned_alias();return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()});}
  let original=self.source.take().unwrap().return_original();let unique=original.is_some();if let Some(original)=original{assert!(original.returned.load(Ordering::Acquire).is_null(),"original claim frame cannot return a live alias");drop(original);}
  RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:if unique{copy}else{0},released_bytes:if unique{release}else{0},..Default::default()})
 }
 fn terminal_is_empty(&self)->bool { self.source.is_none() }
 fn next_close_byte_demand(&self)->Option<usize> {Some(self.source.as_ref().map_or(0,|source|if source.has_returned_alias(){0}else{PublicationClaimHandle::frame_bytes()}))}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{let Some(source)=self.source.as_ref()else{return Ok(0);};if source.is_claimed(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original publication claim still has its permit"));}Ok(if source.has_returned_alias(){std::mem::size_of::<usize>()}else{std::mem::size_of::<PublicationClaim>()})}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

impl RetireOwned for PublicationClaimHandle {
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(ClaimRetirement{source:Some(self)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<ClaimRetirement>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{std::mem::size_of::<Self>()}
}

