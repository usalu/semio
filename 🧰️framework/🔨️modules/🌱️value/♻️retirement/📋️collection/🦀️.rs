//! 📋️ Original vector and deque owners keep bounded logical items and independently paid backing headers.
use super::*;

#[doc(hidden)]
pub struct Collection<T:RetireOwned>{original:ManuallyDrop<Vec<T>>,pending:ManuallyDrop<Option<T>>}
impl<T:RetireOwned> RetirementCursor for Collection<T>{
 native_retirement_reply!();
 fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.next_demand(body)?;Ok(u64::from(!self.terminal_is_empty()))}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(value)=self.pending.as_ref(){return retirement_cursor_birth_demand(value)}if !self.original.is_empty()&&!std::mem::needs_drop::<T>()&&T::retirement_element_copy_bytes()!=0&&size_of::<T>()!=0{let count=(body.saturating_sub(size_of::<usize>())/size_of::<T>()).max(1).min(self.original.len());return Ok(crate::RetirementDemand{copy_bytes:count*size_of::<T>()+size_of::<usize>(),depth:1,..Default::default()})}if !self.original.is_empty(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<Vec<T>>(),depth:1,..Default::default()})}Ok(crate::RetirementDemand{copy_bytes:if self.terminal_is_empty(){0}else{size_of::<Vec<T>>()},release_bytes:self.original.capacity()*size_of::<T>(),depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
 
 fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.original.is_empty()&&(self.original.capacity()==0||size_of::<T>()==0)}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> Collection<T>{
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete(0)}let grant=effect.grant();let demand=match self.next_demand(body){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};if effect.maximum_work_units()==0||grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}if self.pending.is_some(){return retirement_input_step(&mut self.pending,effect)}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};if !self.original.is_empty()&&!std::mem::needs_drop::<T>()&&T::retirement_element_copy_bytes()!=0&&size_of::<T>()!=0&&grant.maximum_copy_bytes>=size_of::<usize>()+size_of::<T>(){let count=((demand.copy_bytes-size_of::<usize>())/size_of::<T>()).min(grant.maximum_items);let copied_bytes=count*size_of::<T>()+size_of::<usize>();let next=self.original.len()-count;self.original.truncate(next);return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:count,copied_bytes,..Default::default()},work)}if !self.original.is_empty(){*self.pending=self.original.pop();return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}if demand.release_bytes!=0{drop(std::mem::take(&mut*self.original));return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()},work)}RetirementStep::Complete(0)}
}

impl<T:RetireOwned> Drop for Collection<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"owned collection retired before terminal-empty");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.pending)}}}}
impl<T:RetireOwned> RetireOwned for Vec<T>{type RetirementCursor=Collection<T>;fn retirement_seed(self)->Self::RetirementCursor{Collection{original:ManuallyDrop::new(self),pending:ManuallyDrop::new(None)}}fn retirement_ready(&self)->bool{self.iter().all(RetireOwned::retirement_ready)}fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}}
#[doc(hidden)]
pub struct DequeCollection<T:RetireOwned>{original:ManuallyDrop<std::collections::VecDeque<T>>,pending:ManuallyDrop<Option<T>>}
impl<T:RetireOwned> RetirementCursor for DequeCollection<T>{
 native_retirement_reply!();
 fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.next_demand(body)?;Ok(u64::from(!self.terminal_is_empty()))}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(value)=self.pending.as_ref(){return retirement_cursor_birth_demand(value)}if !self.original.is_empty()&&!std::mem::needs_drop::<T>()&&T::retirement_element_copy_bytes()!=0&&size_of::<T>()!=0{let count=(body.saturating_sub(size_of::<usize>())/size_of::<T>()).max(1).min(self.original.len());return Ok(crate::RetirementDemand{copy_bytes:count*size_of::<T>()+size_of::<usize>(),depth:1,..Default::default()})}if !self.original.is_empty(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<std::collections::VecDeque<T>>(),depth:1,..Default::default()})}Ok(crate::RetirementDemand{copy_bytes:if self.terminal_is_empty(){0}else{size_of::<std::collections::VecDeque<T>>()},release_bytes:self.original.capacity()*size_of::<T>(),depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
 
 fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.original.is_empty()&&(self.original.capacity()==0||size_of::<T>()==0)}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> DequeCollection<T>{
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete(0)}let grant=effect.grant();let demand=match self.next_demand(body){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};if effect.maximum_work_units()==0||grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}if self.pending.is_some(){return retirement_input_step(&mut self.pending,effect)}let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};if !self.original.is_empty()&&!std::mem::needs_drop::<T>()&&T::retirement_element_copy_bytes()!=0&&size_of::<T>()!=0&&grant.maximum_copy_bytes>=size_of::<usize>()+size_of::<T>(){let count=((demand.copy_bytes-size_of::<usize>())/size_of::<T>()).min(grant.maximum_items);let copied_bytes=count*size_of::<T>()+size_of::<usize>();let next=self.original.len()-count;self.original.truncate(next);return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:count,copied_bytes,..Default::default()},work)}if !self.original.is_empty(){*self.pending=self.original.pop_front();return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)}if demand.release_bytes!=0{drop(std::mem::take(&mut*self.original));return RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()},work)}RetirementStep::Complete(0)}
}

impl<T:RetireOwned> Drop for DequeCollection<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"owned collection retired before terminal-empty");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.pending)}}}}
impl<T:RetireOwned> RetireOwned for std::collections::VecDeque<T>{type RetirementCursor=DequeCollection<T>;fn retirement_seed(self)->Self::RetirementCursor{DequeCollection{original:ManuallyDrop::new(self),pending:ManuallyDrop::new(None)}}fn retirement_ready(&self)->bool{self.iter().all(RetireOwned::retirement_ready)}fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{path.supports::<T>()}}




