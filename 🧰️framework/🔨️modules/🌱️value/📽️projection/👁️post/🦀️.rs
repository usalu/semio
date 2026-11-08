//! 👁️ Actual projected post custody lends original reads and returns the same owner before mutable commit.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneSource,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits}};
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
/// 🧳️ Specializes the common Source owner to projected post access.
pub struct OriginalPostRead<S:RetireOwned+Sync+'static>{owner:crate::retained_clone::OriginalSourceOwner<S,S>}
impl<S:RetireOwned+Sync+'static> Default for OriginalPostRead<S>{fn default()->Self{Self{owner:Default::default()}}}
impl<S:RetireOwned+Sync+'static> OriginalPostRead<S>{
 fn mode(mode:PagedOneItemPostMode)->crate::retained_clone::OriginalSourceMode{match mode{PagedOneItemPostMode::Original=>crate::retained_clone::OriginalSourceMode::Original,PagedOneItemPostMode::Mutable=>crate::retained_clone::OriginalSourceMode::Unique}}
 pub fn capture_demand()->RetirementDemand{crate::retained_clone::OriginalSourceOwner::<S,S>::capture_demand()}
 pub fn capture_into<'r>(&mut self,input:&mut Option<S>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::retained_clone::RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::retained_clone::RetainedCloneStep,ValueError>,ValueError>{self.owner.capture_into(input,effect,reply)}
 pub fn projection_drive_required(&self)->Result<bool,ValueError>{self.owner.projection_drive_required()}
 pub fn projection_frame(&mut self,effect:&crate::RetainedAllocationEffect<'_>)->Result<&mut crate::retained_clone::OriginalProjectionFrame<S,S>,ValueError>{self.owner.projection_frame(effect)}
 pub fn demand(&self,mode:PagedOneItemPostMode,body:usize)->Result<Option<RetirementDemand>,ValueError>{self.owner.demand(Self::mode(mode),body)}
 pub fn work_demand(&self,mode:PagedOneItemPostMode,body:usize)->Result<u64,ValueError>{self.owner.work_demand(Self::mode(mode),body)}
 pub fn failure(&self)->Option<&ValueError>{self.owner.failure()}
 pub fn failure_demand(&self)->RetirementDemand{self.owner.failure_demand()}
 pub fn take_failure_into<'r>(&mut self,output:&mut Option<ValueError>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::retained_clone::RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::retained_clone::RetainedCloneStep,ValueError>,ValueError>{self.owner.take_failure_into(output,effect,reply)}
 pub fn prepare_into<'r>(&mut self,mode:PagedOneItemPostMode,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.owner.prepare_into(Self::mode(mode),body,effect,reply)}
 pub fn recover_prepare_into<'r>(&mut self,mode:PagedOneItemPostMode,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.owner.recover_prepare_into(Self::mode(mode),body,effect,reply)}
 pub fn prepare_original_into<'r>(&mut self,mode:PagedOneItemPostMode,body:usize,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.owner.prepare_original_into(Self::mode(mode),body,original,effect,reply)}
 pub fn recover_prepare_original_into<'r>(&mut self,mode:PagedOneItemPostMode,body:usize,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::retained_clone::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.owner.recover_prepare_original_into(Self::mode(mode),body,original,effect,reply)}
 pub fn requires_original_birth(&self)->bool{self.owner.requires_original_birth()}
 pub fn original_birth_step(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.owner.original_birth_step(original,grant)}
 pub fn requires_original_control(&self)->bool{self.owner.requires_original_control()}
 pub fn read(&self,mode:PagedOneItemPostMode)->Result<PagedOneItemPostRead<'_,S>,ValueError>{match mode{PagedOneItemPostMode::Original=>self.owner.original().map(PagedOneItemPostRead::Original),PagedOneItemPostMode::Mutable=>self.owner.unique().map(PagedOneItemPostRead::Mutable)}}
 pub fn access(&mut self,mode:PagedOneItemPostMode)->Result<PagedOneItemPost<'_,S>,ValueError>{match mode{PagedOneItemPostMode::Original=>self.owner.original().map(PagedOneItemPost::Original),PagedOneItemPostMode::Mutable=>self.owner.unique_mut().map(PagedOneItemPost::Mutable)}}
 pub fn return_demand(&self)->RetirementDemand{self.owner.return_demand()}
 pub fn return_into<'r>(&mut self,output:&mut Option<S>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::retained_clone::RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::retained_clone::RetainedCloneStep,ValueError>,ValueError>{self.owner.return_into(output,effect,reply)}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
