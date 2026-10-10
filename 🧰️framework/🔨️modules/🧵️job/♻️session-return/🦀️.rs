//! ♻️ Returns the actual terminal session allocation after all original authority and wake custody.
use super::*;
#[path="🔐️handle/🦀️.rs"]
mod handle;
pub(crate) use handle::SessionHandle;

impl<J:InteractiveJob+'static> WorkerJobSession<J>{
 pub fn terminal_frame_release_bytes(&self)->Result<usize,ValueError>{
  if !self.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original worker session authority is not terminal"));}
  if self.inner.wake_guard.load(Ordering::Acquire)||unsafe{(&*self.inner.waker.get()).is_some()}{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original worker session retains a registered wake owner"));}
  SessionHandle::<J>::frame_bytes().checked_add(std::mem::size_of::<WorkerJobRetirementNode<J>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original session frame extent overflow"))
 }
 pub fn return_terminal(mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Self)>{
  let bytes=match self.terminal_frame_release_bytes(){Ok(bytes)=>bytes,Err(error)=>return Err((error,self))};
  if grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original worker session frame return is not funded"),self));}
  let inner=unsafe{ManuallyDrop::take(&mut self.inner)};
  match inner.try_return(){
   Ok(inner)=>{self.inner_returned=true;let retirement=unsafe{(&mut*self.retirement.get()).take().expect("terminal session retains original return node")};WORKER_JOB_RETIREMENT_SLOTS[retirement.header.slot].store(std::ptr::null_mut(),Ordering::Release);drop(inner);drop(retirement);Ok(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})}
   Err(inner)=>{self.inner=ManuallyDrop::new(inner);Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original worker session still lends a strong handle"),self))}
  }
 }
}
impl<J:InteractiveJob+'static> MountedWorkerJobSession<J>{
 pub fn terminal_frame_release_bytes(&self)->Result<usize,ValueError>{if self.checked_out.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"mounted session retains a checked-out original"));}self.session.terminal_frame_release_bytes()}
 pub fn return_terminal(self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Self)>{
  if self.checked_out.is_some(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"mounted session retains a checked-out original"),self));}
  let Self{session,ticket,checked_out}=self;
  session.return_terminal(grant).map_err(|(error,session)|(error,Self{session,ticket,checked_out}))
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
