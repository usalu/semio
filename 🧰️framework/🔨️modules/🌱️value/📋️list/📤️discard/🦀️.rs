//! 📤️ One no-drop payload tail changes only its original leaf and root length headers.
use super::*;
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedCloneProgress,RetainedWorkOutcome,RetirementDemand};
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct PagedListDiscardDemand{pub items:usize,pub physical:RetirementDemand}
impl<T,const N:usize>PagedList<T,N>{
 pub fn plain_tail_discard_demand(&self,maximum_items:usize,body:usize)->Result<PagedListDiscardDemand,PagedListError>{
  if std::mem::needs_drop::<T>()||size_of::<T>()==0{return Err(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"plain tail discard requires a nonzero payload without drop glue"})}
  if self.length==0||maximum_items==0{return Ok(Default::default())}
  let leaf=self.leaf(self.length-1).ok_or(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"plain tail lost its original leaf"})?;
  let headers=2*size_of::<usize>();let items=((body.saturating_sub(headers))/size_of::<T>()).max(1).min(maximum_items).min(leaf.len());
  let copy_bytes=items.checked_mul(size_of::<T>()).and_then(|bytes|bytes.checked_add(headers)).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"plain tail copy extent overflow"})?;
  Ok(PagedListDiscardDemand{items,physical:RetirementDemand{copy_bytes,depth:self.next_pop_depth_demand()?,..Default::default()}})
 }
 pub fn discard_plain_tail(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{
  let grant=effect.grant();let demand=match self.plain_tail_discard_demand(grant.maximum_items,grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  if demand.items==0||effect.maximum_work_units()==0||!semio_framework_value_control_contract::retained_clone_grant_admits(grant,demand.physical){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let before=effect.performed_work();let admitted=effect.performed(1);let work=effect.performed_work()-before;match admitted{Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"plain tail exceeded its original work action")),work},Err(error)=>return RetainedWorkOutcome{result:Err(error),work}}
  let leaf=self.leaf_mut(self.length-1).expect("quoted original plain leaf");leaf.truncate(leaf.len()-demand.items);self.length-=demand.items;
  RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:demand.items,copied_bytes:demand.physical.copy_bytes,..Default::default()}),work}
 }
}
