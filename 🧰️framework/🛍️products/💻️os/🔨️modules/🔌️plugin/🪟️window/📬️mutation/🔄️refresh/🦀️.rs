use semio_framework_value::retirement::RetireOwned;
/// 🔄️ Original window authority refresh yields until its displaced snapshot has admitted custody.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum WindowAuthorityRefreshStep{Pending(semio_framework_value::retained_clone::RetainedCloneProgress),Ready(semio_framework_value::retained_clone::RetainedCloneProgress)}
#[doc(hidden)]
pub trait WindowRefreshSnapshot:RetireOwned{fn swap_address(&mut self,other:&mut Self);}
#[doc(hidden)]
pub fn refresh_demands<T:WindowRefreshSnapshot>(pending:&Option<T>,retired:&semio_framework_value_retirement_queue::RetirementQueue)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
 use semio_framework_value::RetirementDemand;Ok(if pending.is_none(){RetirementDemand{depth:1,..Default::default()}}else if !retired.has_reserved_slot(){RetirementDemand{capacity_bytes:retired.next_reserve_capacity_byte_demand()?,depth:retired.len()+1,..Default::default()}}else{RetirementDemand{capacity_bytes:semio_framework_value_retirement_queue::RetirementQueue::frame_birth_bytes::<T>(),depth:retired.len()+1,..Default::default()}})
}
#[doc(hidden)]
pub fn refresh_transfer<T:WindowRefreshSnapshot>(current:&mut T,pending:&mut Option<T>,retired:&mut semio_framework_value_retirement_queue::RetirementQueue,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<WindowAuthorityRefreshStep,semio_framework_value::ValueError>{
 use semio_framework_value::RetainedWorkOutcome;
 let idle=||RetainedWorkOutcome{result:Ok(WindowAuthorityRefreshStep::Pending(Default::default())),work:0};
 let demand=match refresh_demands(pending,retired){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
 if effect.maximum_work_units()==0||{let grant=effect.grant();grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth}{return idle()}
 if !retired.has_reserved_slot(){let turn=retired.reserve_step(effect);return RetainedWorkOutcome{result:turn.result.map(WindowAuthorityRefreshStep::Pending),work:turn.work}}
 let Some(mut next)=pending.take()else{return idle()};
 current.swap_address(&mut next);let mut previous=Some(std::mem::replace(current,next));
 let turn=retired.admit_owned(&mut previous,effect);
 let restore=|current:&mut T,pending:&mut Option<T>,previous:Option<T>|if let Some(previous)=previous{let mut next=std::mem::replace(current,previous);current.swap_address(&mut next);*pending=Some(next);};
 let result=match turn.result{Ok(Some(progress))=>Ok(WindowAuthorityRefreshStep::Ready(progress)),Ok(None)=>{restore(current,pending,previous);Ok(WindowAuthorityRefreshStep::Pending(Default::default()))},Err(error)=>{restore(current,pending,previous);Err(error)}};
 RetainedWorkOutcome{result,work:turn.work}
}
