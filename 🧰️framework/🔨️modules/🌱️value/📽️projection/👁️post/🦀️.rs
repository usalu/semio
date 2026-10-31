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
pub struct OriginalPostRead<S:RetireOwned+Sync>{admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<S,S>>>,source:ManuallyDrop<Option<RetainedCloneSource<S,S>>>}
impl<S:RetireOwned+Sync>Default for OriginalPostRead<S>{fn default()->Self{Self{admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None)}}}
impl<S:RetireOwned+Sync>OriginalPostRead<S>{
 fn movement_bytes()->usize{size_of::<Self>()*2+size_of::<Option<S>>()*2+size_of::<RetainedCloneSource<S,S>>()*2}
 fn child_error(error:ValueError)->ValueError{let mut progress=error.retained_progress();progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.saturating_add(Self::movement_bytes());error.with_retained_progress(progress)}
 pub fn demand(&self,post:&Option<S>,mode:PagedOneItemPostMode,body:usize)->Result<Option<RetirementDemand>,ValueError>{
  if post.is_some()==(self.source.is_some()||self.admission.is_some()){return Err(invalid("exactly one original post owner is required"));}
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
 pub fn prepare(&mut self,post:&mut Option<S>,mode:PagedOneItemPostMode,body:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{let mut work=0;let result=(||{
  let Some(demand)=self.demand(post,mode,body)?else{return Ok(None);};
  if !retained_clone_grant_admits(grant,demand){return Ok(Some(Default::default()));}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::movement_bytes(),maximum_depth:grant.maximum_depth-1,..grant};
  let mut progress=match mode{
   PagedOneItemPostMode::Original=>match self.admission.as_mut(){
    None=>{let original=post.take().ok_or_else(||invalid("original projected post is absent"))?;*self.admission=Some(RetainedCloneSource::<S,S>::admission_borrowed(original,|value:&S|value));work+=1;RetainedCloneProgress{copied_items:1,..Default::default()}},
    Some(admission)=>{let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,child)}else{admission.advance(child)};work+=turn.work;let step=turn.result.map_err(Self::child_error)?;if admission.terminal_is_empty(){*self.admission=None;}step.progress()}
   },
   PagedOneItemPostMode::Mutable=>if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,child);work+=turn.work;let step=turn.result.map_err(Self::child_error)?;if let Some((original,()))=returned{*post=Some(original);}if admission.terminal_is_empty(){*self.admission=None;}step.progress()}else{   match {let turn=self.source.as_mut().ok_or_else(||invalid("original post source is absent"))?.take_authority(child);work+=turn.work;turn.result}.map_err(Self::child_error)?{RetainedCloneSourceTake::Pending(progress)=>progress,RetainedCloneSourceTake::Ready(original,progress)=>{*post=Some(original);*self.source=None;progress}}},
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
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.admission.is_none()}
}
impl<S:RetireOwned+Sync>Drop for OriginalPostRead<S>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original post must return to its exact snapshot issuer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.source);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
