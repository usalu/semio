use semio_framework_value::retirement::RetireOwned;
/// 🔄️ Original window authority refresh yields until its displaced snapshot has admitted custody.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum WindowAuthorityRefreshStep{Pending(semio_framework_value::retained_clone::RetainedCloneProgress),Ready(semio_framework_value::retained_clone::RetainedCloneProgress)}
pub(crate) trait WindowRefreshSnapshot:RetireOwned{fn swap_address(&mut self,other:&mut Self);}
pub(crate) fn refresh_demands<T:WindowRefreshSnapshot>(pending:&Option<T>,retired:&semio_framework_value::retirement::queue::RetirementQueue)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
 use semio_framework_value::RetirementDemand;Ok(if pending.is_none(){RetirementDemand{depth:1,..Default::default()}}else if !retired.has_reserved_slot(){RetirementDemand{capacity_bytes:retired.next_reserve_capacity_byte_demand()?,depth:retired.len()+1,..Default::default()}}else{RetirementDemand{capacity_bytes:semio_framework_value::retirement::queue::RetirementQueue::frame_birth_bytes::<T>(),depth:retired.len()+1,..Default::default()}})
}
pub(crate) fn refresh_transfer<T:WindowRefreshSnapshot>(current:&mut T,pending:&mut Option<T>,retired:&mut semio_framework_value::retirement::queue::RetirementQueue,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<WindowAuthorityRefreshStep,semio_framework_value::ValueError>{
 use semio_framework_value::retained_clone::RetainedCloneProgress;let demand=refresh_demands(pending,retired)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(WindowAuthorityRefreshStep::Pending(Default::default()));}
 if !retired.has_reserved_slot(){return retired.reserve_step(grant).map(WindowAuthorityRefreshStep::Pending);}
 let Some(mut next)=pending.take()else{return Ok(WindowAuthorityRefreshStep::Pending(Default::default()));};current.swap_address(&mut next);let previous=std::mem::replace(current,next);match retired.admit_owned(previous,grant){Ok(progress)=>Ok(WindowAuthorityRefreshStep::Ready(progress)),Err((error,previous))=>{let mut next=std::mem::replace(current,previous);current.swap_address(&mut next);*pending=Some(next);Err(error)}}
}
