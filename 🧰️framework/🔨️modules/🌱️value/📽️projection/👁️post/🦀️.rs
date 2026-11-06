//! 👁️ Actual projected post custody lends original reads and returns the same owner before mutable commit.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSourceTake,retained_clone_grant_admits,admit_retained_clone_progress}};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum PagedOneItemPostMode{Original,Mutable}
pub enum PagedOneItemPostRead<'a,S>{Original(RetainedCloneRef<'a,S>),Mutable(&'a S)}
impl<'a,S>PagedOneItemPostRead<'a,S>{
 pub fn get(self)->&'a S{match self{Self::Original(value)=>value.get(),Self::Mutable(value)=>value}}
 pub fn original(self)->Result<RetainedCloneRef<'a,S>,ValueError>{match self{Self::Original(value)=>Ok(value),Self::Mutable(_)=>Err(invalid("original post read is required"))}}
}
pub enum PagedOneItemPost<'a,S>{Original(RetainedCloneRef<'a,S>),Mutable(&'a mut S)}
impl<'a,S>PagedOneItemPost<'a,S>{
 pub fn read(&self)->PagedOneItemPostRead<'_,S>{match self{Self::Original(value)=>PagedOneItemPostRead::Original(*value),Self::Mutable(value)=>PagedOneItemPostRead::Mutable(value)}}
 pub fn original(self)->Result<RetainedCloneRef<'a,S>,ValueError>{match self{Self::Original(value)=>Ok(value),Self::Mutable(_)=>Err(invalid("original post read is required"))}}
 pub fn mutable(self)->Result<&'a mut S,ValueError>{match self{Self::Mutable(value)=>Ok(value),Self::Original(_)=>Err(invalid("mutable post access requires all original reads to close"))}}
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
/// 🧳️ Only the genuine projected owner is sealed; cancellation returns it to its original snapshot issuer.
pub struct OriginalPostRead<S:RetireOwned+Sync>{admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<S,S>>>,source:ManuallyDrop<Option<RetainedCloneSource<S,S>>>,actual:Option<usize>}
impl<S:RetireOwned+Sync>Default for OriginalPostRead<S>{fn default()->Self{Self{admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),actual:None}}}
impl<S:RetireOwned+Sync>OriginalPostRead<S>{
 fn movement_bytes()->usize{size_of::<Self>()*2+size_of::<Option<S>>()*2+size_of::<RetainedCloneSource<S,S>>()*2}
 fn child_error(error:crate::OriginalRetirementFailure,performed:bool)->crate::OriginalRetirementFailure{if !performed{return error}let mut progress=error.retained_progress();progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.saturating_add(Self::movement_bytes());error.with_retained_progress(progress)}
 pub fn demand(&self,post:&Option<S>,mode:PagedOneItemPostMode,body:usize)->Result<Option<RetirementDemand>,ValueError>{
  if post.is_some()==(self.source.is_some()||self.admission.is_some()){return Err(invalid("exactly one original post owner is required"));}
  if self.actual.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<usize>>(),depth:1,..Default::default()}))}
  let mut demand=match(mode,self.source.as_ref(),self.admission.as_ref()){
   (PagedOneItemPostMode::Original,None,None)=>RetirementDemand{depth:1,..Default::default()},
   (PagedOneItemPostMode::Original,None,Some(admission))=>admission.next_demand()?,
   (PagedOneItemPostMode::Mutable,Some(source),None)=>source.next_take_demand(body.saturating_sub(Self::movement_bytes()))?,
   (PagedOneItemPostMode::Mutable,None,Some(admission))=>admission.next_return_demand()?,
   _=>return Ok(None),
  };
  demand.copy_bytes=demand.copy_bytes.checked_add(Self::movement_bytes()).ok_or_else(||invalid("original post movement quote overflow"))?;
  demand.depth=demand.depth.checked_add(1).ok_or_else(||invalid("original post depth quote overflow"))?;
  Ok(Some(demand))
 }
 /// \u26FD\uFE0F Quotes the actual selected source admission or original-authority handback work.
 pub fn work_demand(&self,post:&Option<S>,mode:PagedOneItemPostMode,body:usize)->Result<u64,ValueError>{if self.demand(post,mode,body)?.is_none(){return Ok(0)}if self.actual.is_some(){return Ok(1)}match mode{PagedOneItemPostMode::Original=>match self.admission.as_ref(){None=>Ok(1),Some(admission)if admission.phase()=="ready"=>Ok(1),Some(admission)=>admission.advance_work_demand()},PagedOneItemPostMode::Mutable=>if let Some(admission)=self.admission.as_ref(){if admission.terminal_is_empty(){Ok(1)}else{admission.return_work_demand()}}else{self.source.as_ref().ok_or_else(||invalid("original post source is absent"))?.next_take_work_demand(body.saturating_sub(Self::movement_bytes()))}}}
 pub fn prepare(&mut self,post:&mut Option<S>,mode:PagedOneItemPostMode,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,crate::OriginalRetirementFailure>{let grant=effect.grant();let maximum_work_units=effect.maximum_work_units();let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{
  if self.work_demand(post,mode,body)?>maximum_work_units{return Ok(Some(Default::default()))}
  let Some(demand)=self.demand(post,mode,body)?else{return Ok(None);};
  if !retained_clone_grant_admits(grant,demand){return Ok(Some(Default::default()));}
  if self.actual.is_some(){self.actual=None;work=1;return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<usize>>(),..Default::default()}))}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::movement_bytes(),maximum_depth:grant.maximum_depth-1,..grant};
  let mut progress=match mode{
   PagedOneItemPostMode::Original=>match self.admission.as_mut(){
    None=>{let original=post.take().ok_or_else(||invalid("original projected post is absent"))?;*self.admission=Some(RetainedCloneSource::<S,S>::admission_borrowed(original,|value:&S|value));work+=1;RetainedCloneProgress{copied_items:1,..Default::default()}},
    Some(admission)=>{let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,child)}else{admission.advance(&mut effect.reborrow(child,maximum_work_units)?)};work+=turn.work;let step=turn.result.map_err(|error|Self::child_error(error.into(),turn.work!=0))?;if admission.terminal_is_empty(){if turn.work==0{work+=1}*self.admission=None;}step.progress()}
   },
   PagedOneItemPostMode::Mutable=>if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,&mut effect.reborrow(child,maximum_work_units.saturating_sub(work))?);work+=turn.work;let step=turn.result.map_err(|error|Self::child_error(error.into(),turn.work!=0))?;if let Some((original,()))=returned{*post=Some(original);}if admission.terminal_is_empty(){if turn.work==0{work+=1}*self.admission=None;}step.progress()}else{   match {let turn=self.source.as_mut().ok_or_else(||invalid("original post source is absent"))?.take_authority(&mut effect.reborrow(child,maximum_work_units.saturating_sub(work))?);work+=turn.work;turn.result}.map_err(|error|Self::child_error(error,work!=0))?{RetainedCloneSourceTake::Pending(progress)=>progress,RetainedCloneSourceTake::Ready(original,progress)=>{*post=Some(original);*self.source=None;progress}}},
  };
  progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.checked_add(Self::movement_bytes()).ok_or_else(||invalid("original post receipt overflow"))?;
  Ok(Some(admit_retained_clone_progress(grant,progress,"actual projected post original custody")?))
 })();crate::RetainedWorkOutcome{result,work}}
 pub fn read<'a>(&'a self,post:&'a Option<S>,mode:PagedOneItemPostMode)->Result<PagedOneItemPostRead<'a,S>,ValueError>{
  if self.demand(post,mode,0)?.is_some(){return Err(invalid("original post access has an unpaid ownership frontier"));}
  Ok(match mode{PagedOneItemPostMode::Original=>PagedOneItemPostRead::Original(self.source.as_ref().unwrap().try_borrow()?),PagedOneItemPostMode::Mutable=>PagedOneItemPostRead::Mutable(post.as_ref().unwrap())})
 }
 pub fn access<'a>(&'a self,post:&'a mut Option<S>,mode:PagedOneItemPostMode)->Result<PagedOneItemPost<'a,S>,ValueError>{
  if self.demand(post,mode,0)?.is_some(){return Err(invalid("original post access has an unpaid ownership frontier"));}
  Ok(match mode{PagedOneItemPostMode::Original=>PagedOneItemPost::Original(self.source.as_ref().unwrap().try_borrow()?),PagedOneItemPostMode::Mutable=>PagedOneItemPost::Mutable(post.as_mut().unwrap())})
 }
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.admission.is_none()&&self.actual.is_none()}
}
impl<S:RetireOwned+Sync>Drop for OriginalPostRead<S>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original post must return to its exact snapshot issuer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.source);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
