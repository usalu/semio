//! 📋️ The clone trait owner binds actual typed elements to the fixed-page engine.
use super::*;
use crate::{OriginalRetirementOutcome,OriginalRetirementPoll};
use semio_framework_value_paged_list::PagedList;
use semio_framework_value_paged_clone::{PagedListCloneEngine,PagedCloneElement};
#[doc(hidden)]
#[repr(transparent)]
pub struct PagedListElement<T:RetainedClone>(T::Cursor);
impl<T:RetainedClone>PagedCloneElement<T> for PagedListElement<T>{fn new()->Self{Self(T::retained_clone_cursor())}
fn output_ready(&self)->bool{self.0.output_ready()}
fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.0.advance_demands(source,body,maximum_work_units)}
fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.0.advance_work_demand(source,body,maximum_work_units)}
fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementOutcome{self.0.advance(source,body,maximum_work_units,effect)}
fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.0.take_demand()}
fn take_work_demand(&self)->Result<u64,ValueError>{self.0.take_work_demand()}
fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{self.0.take(output,effect,reply)}
fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{self.0.begin_close_demand()}
fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.0.begin_close_work_demand()}
fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{self.0.begin_close_paid(effect)}
fn close_started(&self)->bool{self.0.close_started()}
fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.close_demands(body)}
fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.0.close_work_demand(body)}
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();self.0.close_step(&mut effect.nested(grant))}
fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
}
pub type PagedListCursor<T:RetainedClone,const N:usize>=PagedListCloneEngine<T,PagedListElement<T>,N>;
impl<T:RetainedClone,const N:usize>RetainedClone for PagedList<T,N>{type Cursor=PagedListCursor<T,N>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl<T:RetainedClone,const N:usize>RetainedCloneCursor<PagedList<T,N>> for PagedListCursor<T,N>{
fn output_ready(&self)->bool{PagedListCloneEngine::output_ready(self)}
fn advance_demands(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{PagedListCloneEngine::advance_demands(self,source,body,maximum_work_units)}
fn advance_work_demand(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{PagedListCloneEngine::advance_work_demand(self,source,body,maximum_work_units)}
fn advance(&mut self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementOutcome{PagedListCloneEngine::advance(self,source,body,maximum_work_units,effect)}
fn take_demand(&self)->Result<RetirementDemand,ValueError>{PagedListCloneEngine::take_demand(self)}
fn take_work_demand(&self)->Result<u64,ValueError>{PagedListCloneEngine::take_work_demand(self)}
fn take<'reply>(&mut self,output:&mut Option<PagedList<T,N>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{PagedListCloneEngine::take(self,output,effect,reply)}
fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{PagedListCloneEngine::begin_close_demand(self)}
fn begin_close_work_demand(&self)->Result<u64,ValueError>{PagedListCloneEngine::begin_close_work_demand(self)}
fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{PagedListCloneEngine::begin_close_paid(self,effect)}
fn close_started(&self)->bool{PagedListCloneEngine::close_started(self)}
fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{PagedListCloneEngine::close_demands(self,body)}
fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{PagedListCloneEngine::close_work_demand(self,body)}
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();PagedListCloneEngine::close_step(self,&mut effect.nested(grant))}
fn terminal_is_empty(&self)->bool{PagedListCloneEngine::terminal_is_empty(self)}
fn next_close_depth_demand(&self)->Result<usize,ValueError>{PagedListCloneEngine::next_close_depth_demand(self)}
fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{PagedListCloneEngine::next_close_copy_byte_demand(self)}
fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{PagedListCloneEngine::next_close_release_byte_demand(self)}
fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{PagedListCloneEngine::next_close_capacity_byte_demand(self,body)}
}
