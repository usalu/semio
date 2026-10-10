//! ♻️ Returns original cancellation nodes and backing allocations under independent grants.
use super::{CancelNode, CancelToken};
use semio_framework_value::{ValueError, ValueRefusalKind, retirement::{RetireOwned, RetirementCursor, RetirementStep}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
#[path="🔐️handle/🦀️.rs"]
mod handle;
pub(crate) use handle::CancelHandle;
pub struct CancelTokenRetirement { source: Option<CancelToken>, body: Option<CancelNode> }
impl CancelTokenRetirement {
 fn release_demand(&self) -> Option<usize> {
  if self.source.is_some() { return Some(CancelHandle::frame_bytes()); }
  let Some(body) = self.body.as_ref() else { return Some(0); };
  let waiters = body.waiters.try_lock().ok()?;
  if !waiters.is_empty() { return None; }
  waiters.capacity().checked_mul(std::mem::size_of::<(u64,std::task::Waker)>())
 }
 fn copy_demand(&self) -> usize { if self.source.is_some() { std::mem::size_of::<CancelNode>() } else if self.body.as_ref().is_some_and(|body|body.parent.is_some()) { std::mem::size_of::<CancelToken>() } else { 0 } }
}
impl RetirementCursor for CancelTokenRetirement {
 fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
  let Some(release) = self.release_demand() else { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original cancellation node retains a registered wake owner")); };
  let copy = self.copy_demand();
  if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy || grant.maximum_release_bytes < release { return RetirementStep::BudgetExhausted; }
  if let Some(source) = self.source.take() {
   self.body = source.0.return_original();
   return RetirementStep::Progress(RetainedCloneProgress { copied_items:1, copied_bytes: if self.body.is_some() { copy } else { 0 }, released_bytes: if self.body.is_some() { release } else { 0 }, ..Default::default() });
  }
  if let Some(body) = self.body.as_mut() {
   let waiters = body.waiters.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner);
   if waiters.capacity() != 0 { let original = std::mem::take(waiters); drop(original); return RetirementStep::Progress(RetainedCloneProgress { copied_items:1, released_bytes:release, ..Default::default() }); }
   self.source = body.parent.take(); self.body.take();
   return RetirementStep::Progress(RetainedCloneProgress { copied_items:1, copied_bytes:copy, ..Default::default() });
  }
  RetirementStep::Complete
 }
 fn terminal_is_empty(&self) -> bool { self.source.is_none() && self.body.is_none() }
 fn next_close_byte_demand(&self) -> Option<usize> { self.release_demand() }
 fn next_work_byte_demand(&self) -> Result<usize,ValueError> { self.release_demand().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original cancellation node retains a registered wake owner"))?; Ok(self.copy_demand()) }
 fn next_birth_bytes(&self,_:usize) -> Option<usize> { Some(0) }
 fn terminal_release_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<Self>()) }
}
impl RetireOwned for CancelToken {
 fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(CancelTokenRetirement { source:Some(self), body:None }) }
 fn retirement_birth_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<CancelTokenRetirement>()) }
 fn controlled_retirement_supported() -> bool { true }
 fn retirement_element_copy_bytes() -> usize { std::mem::size_of::<Self>() }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
