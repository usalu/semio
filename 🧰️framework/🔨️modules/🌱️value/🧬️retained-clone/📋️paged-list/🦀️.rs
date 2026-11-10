//! 📋️ The trait owner binds actual borrowed element producers to the native paged engine.
use super::*;
use semio_framework_value_retirement::OriginalRetirementPoll;
use semio_framework_value_paged_list::PagedList;
use semio_framework_value_paged_clone::{PagedListCloneEngine,PagedCloneElement};
#[doc(hidden)]
#[repr(transparent)]
pub struct PagedListElement<T:RetainedClone>(T::Cursor);
impl<T:RetainedClone> PagedCloneElement<T> for PagedListElement<T>{
 fn new()->Self{Self(T::retained_clone_cursor())}
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{let native=|element:&Self|element.0.terminal_is_empty()&&element.0.native_terminal_release().is_some_and(|release|release.validates(&element.0));native(self).then(||unsafe{RetainedCursorRelease::native(native)})}
 fn output_ready(&self)->bool{self.0.output_ready()}
 fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{self.0.advance_demands(source,body,maximum)}
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<u64,ValueError>{self.0.advance_work_demand(source,body,maximum)}
 fn advance_selection(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<(RetirementDemand,u64),ValueError>{let selected=self.0.captured_advance_selection(source,body,maximum)?;Ok((selected.demand(),selected.work()))}
 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.0.advance_into(source,body,maximum,effect,reply)}
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.0.recover_advance_into(source,body,maximum,effect,reply)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.0.take_demand()}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.0.take_work_demand()}
 fn take<'r>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.0.take(output,effect,reply)}
 fn recover_take<'r>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.0.recover_take(output,effect,reply)}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{self.0.begin_close_demand()}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.0.begin_close_work_demand()}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.0.begin_close_into(effect,reply)}
 fn recover_begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.0.recover_begin_close_into(effect,reply)}
 fn close_started(&self)->bool{self.0.close_started()}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.0.close_work_demand(body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.0.close_into(body,effect,reply)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.0.recover_close_into(body,effect,reply)}
 fn requires_original_birth(&self)->bool{self.0.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.0.close_original_birth_step(original,grant)}
 fn requires_original_control(&self)->bool{self.0.requires_original_control()}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.0.close_original_into(original,body,effect,reply)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.0.recover_close_original_into(original,body,effect,reply)}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
}
pub type PagedListCursor<T:RetainedClone,const N:usize>=PagedListCloneEngine<T,PagedListElement<T>,N>;
impl<T:RetainedClone,const N:usize> RetainedClone for PagedList<T,N>{type Cursor=PagedListCursor<T,N>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl<T:RetainedClone,const N:usize> RetainedCloneCursor<PagedList<T,N>> for PagedListCursor<T,N>{
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{PagedListCloneEngine::native_terminal_release(self)}
 fn output_ready(&self)->bool{PagedListCloneEngine::output_ready(self)}
 fn advance_demands(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{PagedListCloneEngine::advance_demands(self,source,body,maximum)}
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum:u64)->Result<u64,ValueError>{PagedListCloneEngine::advance_work_demand(self,source,body,maximum)}
 fn advance_selection(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{let (demand,work)=PagedListCloneEngine::advance_selection(self,source,body,maximum)?;Ok(RetainedCloneSelection::new(demand,work))}
 fn captured_advance_selection(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{<Self as RetainedCloneCursor<PagedList<T,N>>>::advance_selection(self,source,body,maximum)}
 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{PagedListCloneEngine::advance_into(self,source,body,maximum,effect,reply)}
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{PagedListCloneEngine::recover_advance_into(self,source,body,maximum,effect,reply)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{PagedListCloneEngine::take_demand(self)}
 fn take_work_demand(&self)->Result<u64,ValueError>{PagedListCloneEngine::take_work_demand(self)}
 fn take<'r>(&mut self,output:&mut Option<PagedList<T,N>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{PagedListCloneEngine::take(self,output,effect,reply)}
 fn recover_take<'r>(&mut self,output:&mut Option<PagedList<T,N>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{PagedListCloneEngine::recover_take(self,output,effect,reply)}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{PagedListCloneEngine::begin_close_demand(self)}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{PagedListCloneEngine::begin_close_work_demand(self)}
 fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{PagedListCloneEngine::begin_close_paid(self,effect)}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{PagedListCloneEngine::begin_close_into(self,effect,reply)}
 fn recover_begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{PagedListCloneEngine::recover_begin_close_into(self,effect,reply)}
 fn close_started(&self)->bool{PagedListCloneEngine::close_started(self)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{PagedListCloneEngine::close_demands(self,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{PagedListCloneEngine::close_work_demand(self,body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{PagedListCloneEngine::close_into(self,body,effect,reply)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{PagedListCloneEngine::recover_close_into(self,body,effect,reply)}
 fn requires_original_birth(&self)->bool{PagedListCloneEngine::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{PagedListCloneEngine::close_original_birth_step(self,original,grant)}
 fn requires_original_control(&self)->bool{PagedListCloneEngine::requires_original_control(self)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{PagedListCloneEngine::close_original_into(self,original,body,effect,reply)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{PagedListCloneEngine::recover_close_original_into(self,original,body,effect,reply)}
 fn terminal_is_empty(&self)->bool{PagedListCloneEngine::terminal_is_empty(self)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{PagedListCloneEngine::next_close_depth_demand(self)}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{PagedListCloneEngine::next_close_copy_byte_demand(self)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{PagedListCloneEngine::next_close_capacity_byte_demand(self,body)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{PagedListCloneEngine::next_close_release_byte_demand(self)}
}
impl<T:RetainedClone,const N:usize> RetainedCloneDemandCursor<PagedList<T,N>> for PagedListCursor<T,N>{fn normal_demands(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{self.advance_demands(source,body,maximum)}}
