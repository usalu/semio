//! 🧮️ Sparse indexed edits retain paid coordinate directories and stream original headers without shifts.
use crate::{HeapSortCursor,HeapSortWork,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};

/// 📐️ Base coordinates address removals/modifications; addition coordinates address the resulting collection.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct IndexedEditShape{pub base_len:usize,pub removed:usize,pub modified:usize,pub added:usize}
/// 🎚️ One original authored coordinate directory.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedEditKind{Removed,Modified,Added}
impl IndexedEditKind{fn index(self)->usize{match self{Self::Removed=>0,Self::Modified=>1,Self::Added=>2}}fn next(self)->Option<Self>{match self{Self::Removed=>Some(Self::Modified),Self::Modified=>Some(Self::Added),Self::Added=>None}}}
/// 🪪️ Original ordinal remains the authored payload address after sorting coordinates.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct IndexedEditEntry{pub index:usize,pub ordinal:usize}
crate::artifact_retire_leaf!(IndexedEditEntry);
/// 🚫️ Invalid sparse coordinates refuse before any candidate header changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedEditFault{Remove,Modify,Add}
/// 🚦️ One funded directory action, settled admission or settled semantic refusal.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedEditStep{Progress(RetainedCloneProgress),Admitted(RetainedCloneProgress),Refused(IndexedEditFault,RetainedCloneProgress)}
impl IndexedEditStep{pub fn progress(self)->RetainedCloneProgress{match self{Self::Progress(p)|Self::Admitted(p)|Self::Refused(_,p)=>p}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{Reserve,Fill,Sort,Validate,Settled}

/// 🧳️ The caller retains the original immutable coordinate sources and admits every action before advancing.
pub struct IndexedEditPlan{shape:IndexedEditShape,orders:ManuallyDrop<[Option<Vec<IndexedEditEntry>>;3]>,kind:IndexedEditKind,phase:Phase,cursor:usize,removal:usize,sort:HeapSortCursor,fault:Option<IndexedEditFault>,output_len:Option<usize>,closing:bool,retirement:RetainedCloneClose}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"indexed edit plan lacks its live admitted phase")}
impl IndexedEditPlan{
 pub fn new(shape:IndexedEditShape)->Self{Self{shape,orders:ManuallyDrop::new([None,None,None]),kind:IndexedEditKind::Removed,phase:Phase::Reserve,cursor:0,removal:0,sort:HeapSortCursor::new(0),fault:None,output_len:None,closing:false,retirement:Default::default()}}
 pub fn shape(&self)->IndexedEditShape{self.shape}
 pub fn settled(&self)->bool{self.phase==Phase::Settled}
 pub fn ready(&self)->bool{self.settled()&&self.fault.is_none()&&!self.closing}
 pub fn fault(&self)->Option<IndexedEditFault>{self.fault}
 pub fn output_len(&self)->Result<usize,ValueError>{if !self.ready(){return Err(invalid());}self.output_len.ok_or_else(invalid)}
 pub fn entries(&self,kind:IndexedEditKind)->Result<&[IndexedEditEntry],ValueError>{if !self.ready(){return Err(invalid());}Ok(self.orders[kind.index()].as_deref().unwrap_or(&[]))}
 fn count(&self)->usize{match self.kind{IndexedEditKind::Removed=>self.shape.removed,IndexedEditKind::Modified=>self.shape.modified,IndexedEditKind::Added=>self.shape.added}}
 fn fixed()->usize{size_of::<Self>()*2+size_of::<IndexedEditEntry>()*4}
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.settled(){return Err(invalid());}
  let capacity_bytes=if self.phase==Phase::Reserve&&!(self.kind==IndexedEditKind::Removed&&self.shape.removed>self.shape.base_len){self.count().checked_mul(size_of::<IndexedEditEntry>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed coordinate directory extent overflow"))?}else{0};
  Ok(RetirementDemand{copy_bytes:Self::fixed(),capacity_bytes,depth:1,..Default::default()})
 }
 fn refuse(&mut self,p:RetainedCloneProgress)->IndexedEditStep{let fault=match self.kind{IndexedEditKind::Removed=>IndexedEditFault::Remove,IndexedEditKind::Modified=>IndexedEditFault::Modify,IndexedEditKind::Added=>IndexedEditFault::Add};self.fault=Some(fault);self.phase=Phase::Settled;IndexedEditStep::Refused(fault,p)}
 pub fn advance(&mut self,grant:RetainedCloneGrant,coordinate:impl FnOnce(IndexedEditKind,usize)->usize)->Result<IndexedEditStep,ValueError>{
  let demand=self.demand()?;if !retained_clone_grant_admits(grant,demand){return Ok(IndexedEditStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};let kind=self.kind.index();
  match self.phase{
   Phase::Reserve=>{
    if self.kind==IndexedEditKind::Removed&&self.shape.removed>self.shape.base_len{return Ok(self.refuse(progress));}
    let mut order=Vec::new();order.try_reserve_exact(self.count()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed coordinate directory reservation refused"))?;progress.retained_capacity_bytes=order.capacity()*size_of::<IndexedEditEntry>();self.orders[kind]=Some(order);self.phase=Phase::Fill;
   },
   Phase::Fill=>{
    if self.cursor<self.count(){let entry=IndexedEditEntry{index:coordinate(self.kind,self.cursor),ordinal:self.cursor};self.orders[kind].as_mut().unwrap().push(entry);self.cursor+=1;}
    else{self.sort=HeapSortCursor::new(self.count());self.phase=Phase::Sort;}
   },
   Phase::Sort=>{
    let order=self.orders[kind].as_mut().unwrap();match self.sort.next(){
     HeapSortWork::Complete=>{self.cursor=0;self.phase=Phase::Validate;},
     HeapSortWork::Compare{left,right}=>{let a=order[left];let b=order[right];self.sort.advance(Some((a.index,a.ordinal).cmp(&(b.index,b.ordinal)))).map_err(|_|invalid())?;},
     HeapSortWork::Swap{left,right}=>{order.swap(left,right);self.sort.advance(None).map_err(|_|invalid())?;},
     HeapSortWork::Advance=>self.sort.advance(None).map_err(|_|invalid())?,
    }
   },
   Phase::Validate=>{
    if self.cursor==self.count(){
     if let Some(next)=self.kind.next(){self.kind=next;self.cursor=0;self.phase=Phase::Reserve;}
     else{self.output_len=Some(self.shape.base_len.checked_sub(self.shape.removed).and_then(|n|n.checked_add(self.shape.added)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed final cardinality overflow"))?);self.phase=Phase::Settled;return Ok(IndexedEditStep::Admitted(progress));}
    }else{
     let order=self.orders[kind].as_ref().unwrap();let row=order[self.cursor];let removed=self.orders[0].as_ref().unwrap();
     if self.kind==IndexedEditKind::Modified&&removed.get(self.removal).is_some_and(|r|r.index<row.index){self.removal+=1;return Ok(IndexedEditStep::Progress(progress));}
     let duplicate=self.cursor>0&&order[self.cursor-1].index==row.index;
     let bad=duplicate||match self.kind{IndexedEditKind::Removed=>row.index>=self.shape.base_len,IndexedEditKind::Modified=>row.index>=self.shape.base_len||removed.get(self.removal).is_some_and(|r|r.index==row.index),IndexedEditKind::Added=>row.index>self.shape.base_len-self.shape.removed+self.cursor};
     if bad{return Ok(self.refuse(progress));}self.cursor+=1;
    }
   },
   Phase::Settled=>return Err(invalid()),
  }
  Ok(IndexedEditStep::Progress(progress))
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}self.retirement.next_owner_demands::<Vec<IndexedEditEntry>>(self.orders.iter().any(Option::is_some),&None,body)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid());}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  for order in self.orders.iter_mut(){if let Some(step)=self.retirement.begin_granted(order,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.orders.iter().all(Option::is_none)&&self.retirement.is_empty()}
}
impl Drop for IndexedEditPlan{fn drop(&mut self){let empty=self.orders.iter().all(Option::is_none)&&self.retirement.is_empty();assert!(std::thread::panicking()||empty,"indexed edit directories require funded original closure");if empty{unsafe{ManuallyDrop::drop(&mut self.orders);}}}}

/// 🚚️ One original survivor/removal or authored addition is transferred per admitted output action.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IndexedOutputWork{Original{index:usize},Removed{index:usize},Added{ordinal:usize},Complete}
/// 🧭️ Fixed output traversal never inserts, removes or shifts an existing collection.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct IndexedOutputCursor{base:usize,output:usize,removed:usize,added:usize}
impl IndexedOutputCursor{
 pub fn next(&self,plan:&IndexedEditPlan)->Result<IndexedOutputWork,ValueError>{
  let removed=plan.entries(IndexedEditKind::Removed)?;let added=plan.entries(IndexedEditKind::Added)?;
  if removed.get(self.removed).is_some_and(|entry|entry.index==self.base){return Ok(IndexedOutputWork::Removed{index:self.base});}
  if let Some(entry)=added.get(self.added).filter(|entry|entry.index==self.output){return Ok(IndexedOutputWork::Added{ordinal:entry.ordinal});}
  if self.base<plan.shape.base_len{return Ok(IndexedOutputWork::Original{index:self.base});}
  if self.output!=plan.output_len()?||self.added!=added.len(){return Err(invalid());}Ok(IndexedOutputWork::Complete)
 }
 pub fn advance(&mut self,plan:&IndexedEditPlan)->Result<(),ValueError>{match self.next(plan)?{IndexedOutputWork::Original{..}=>{self.base+=1;self.output+=1;},IndexedOutputWork::Removed{..}=>{self.base+=1;self.removed+=1;},IndexedOutputWork::Added{..}=>{self.output+=1;self.added+=1;},IndexedOutputWork::Complete=>return Err(invalid())}Ok(())}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
