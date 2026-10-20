//! 🧳️ Source-bound page reads retain one original lease and resume each indivisible page or row action.
use super::{Page,PersistentList,PERSISTENT_PAGE_FANOUT,fits};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};
use std::mem::size_of;
const DEPTH:usize=usize::BITS as usize/4;
struct Frame<T:crate::retirement::RetireOwned+Send+Sync+'static>{page:*const Page<T>,next:usize}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Copy for Frame<T>{}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Clone for Frame<T>{fn clone(&self)->Self{*self}}
/// 🪪️ Every row projection uses the supplied original source; the fixed page pointers remain alive through its bound lease.
pub struct PersistentListRead<T:crate::retirement::RetireOwned+Send+Sync+'static>{original:Option<RetainedCloneBinding>,stack:[Option<Frame<T>>;DEPTH],depth:usize,index:usize,started:bool,done:bool,closing:bool,cleared:bool}
unsafe impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Send for PersistentListRead<T>{}
unsafe impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Sync for PersistentListRead<T>{}
/// 👁️ A returned row borrows its genuine immutable source and transfers no row payload owner.
pub enum PersistentListReadStep<'a,T:crate::retirement::RetireOwned+Send+Sync+'static>{Progress(RetainedCloneProgress),Row(RetainedCloneProgress,usize,RetainedCloneRef<'a,T>),Complete(RetainedCloneProgress)}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> PersistentListReadStep<'_,T>{pub fn progress(&self)->RetainedCloneProgress{match self{Self::Progress(p)|Self::Row(p,..)|Self::Complete(p)=>*p}}}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> PersistentListRead<T>{
 /// 🌱️ The enclosing owner admits these fixed cursor headers in its own constructor quote.
 pub fn new()->Self{Self{original:None,stack:[None;DEPTH],depth:0,index:0,started:false,done:false,closing:false,cleared:false}}
 /// 📏️ Normal reads admit one source alias, page frame, stack transition or genuine row projection.
 pub fn demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable read already closing"));}
  original.check_binding(&self.original)?;
  if self.original.is_none(){return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.done{return Ok(Default::default());}
  let copy_bytes=size_of::<Option<Frame<T>>>()*2+size_of::<usize>()*3+size_of::<bool>()*2+size_of::<RetainedCloneRef<'_,T>>()*2;
  Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})
 }
 /// ▶️ Enter each page once and yield each original row once without a borrowed iterator or payload copy.
 pub fn advance<'a>(&mut self,original:RetainedCloneRef<'a,PersistentList<T>>,grant:RetainedCloneGrant)->Result<PersistentListReadStep<'a,T>,ValueError>{
  let d=self.demand(original)?;if !fits(d,grant){return Ok(PersistentListReadStep::Progress(Default::default()));}
  if let Some(p)=original.bind(&mut self.original,grant)?{return Ok(PersistentListReadStep::Progress(p));}
  if self.done{return Ok(PersistentListReadStep::Complete(Default::default()));}
  let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if !self.started{self.started=true;if let Some(root)=original.get().root.as_ref(){self.stack[0]=Some(Frame{page:root.get(),next:0});self.depth=1;}else{self.done=true;}return Ok(if self.done{PersistentListReadStep::Complete(p)}else{PersistentListReadStep::Progress(p)});}
  if self.depth==0{self.done=true;return Ok(PersistentListReadStep::Complete(p));}
  let slot=self.depth-1;let frame=self.stack[slot].as_mut().expect("original read frame");let page=unsafe{&*frame.page};
  match page{
   Page::Leaf{rows,len}=>{if frame.next==*len{self.stack[slot]=None;self.depth-=1;return Ok(PersistentListReadStep::Progress(p));}let row=rows[frame.next].as_ref().expect("dense original row").get()as*const T;frame.next+=1;let index=self.index;self.index+=1;let row=original.project(index,|_|unsafe{&*row});Ok(PersistentListReadStep::Row(p,index,row))},
   Page::Branch{children,..}=>{if frame.next==PERSISTENT_PAGE_FANOUT||children[frame.next].is_none(){self.stack[slot]=None;self.depth-=1;return Ok(PersistentListReadStep::Progress(p));}let child=children[frame.next].as_ref().expect("dense original child").get();frame.next+=1;if self.depth==DEPTH{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"immutable read exceeds admitted page depth"));}self.stack[self.depth]=Some(Frame{page:child,next:0});self.depth+=1;Ok(PersistentListReadStep::Progress(p))},
  }
 }
 /// 🛑️ Page addresses clear before the original source lease can retire.
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Clears fixed borrowed frames before settling the exact source authority.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.cleared{return Ok(RetirementDemand{copy_bytes:size_of::<[Option<Frame<T>>;DEPTH]>()+size_of::<usize>()+size_of::<bool>(),depth:1,..Default::default()});}RetainedCloneBinding::close_demand(&self.original,body)}
 /// 🪜️ Original rows and pages keep their existing issuers throughout every cancellation frontier.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable read closure not started"));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.cleared{self.stack=[None;DEPTH];self.depth=0;self.cleared=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}RetainedCloneBinding::close_one(&mut self.original,grant)}
 /// 🫙️ No original page address or source alias survives terminal closure.
 pub fn terminal_is_empty(&self)->bool{self.cleared&&self.original.is_none()&&self.depth==0}
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Drop for PersistentListRead<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"immutable read requires original source closure");}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
