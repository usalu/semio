//! 🔗️ Fallible shared storage records real allocator attempts before original slot transfer.
pub use std::sync::Arc as SharedAllocation;
use std::{sync::Arc,mem::{size_of,MaybeUninit}};
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone_grant_admits};

pub fn shared_allocation_bytes<T>()->usize{std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<T>()).map_or(usize::MAX,|(layout,_)|layout.pad_to_align().size())}
fn add(a:usize,b:usize)->Result<usize,ValueError>{a.checked_add(b).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"shared allocation copy extent overflow"))}
pub fn shared_uninitialized_demand<T>()->Result<RetirementDemand,ValueError>{let capacity_bytes=shared_allocation_bytes::<T>();if capacity_bytes==usize::MAX{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"shared allocation layout overflow"))}Ok(RetirementDemand{copy_bytes:add(add(2*size_of::<usize>(),size_of::<Arc<MaybeUninit<T>>>())?,add(size_of::<Option<Arc<MaybeUninit<T>>>>(),2*size_of::<Option<usize>>())?)?,capacity_bytes,depth:1,..Default::default()})}
pub fn shared_allocation_value_demand<T>()->Result<RetirementDemand,ValueError>{let mut d=shared_uninitialized_demand::<T>()?;d.copy_bytes=add(d.copy_bytes,add(size_of::<T>(),size_of::<Arc<T>>())?)?;Ok(d)}
pub fn shared_allocation_slot_demand<T>()->Result<RetirementDemand,ValueError>{let mut d=shared_uninitialized_demand::<T>()?;d.copy_bytes=add(d.copy_bytes,add(size_of::<Option<T>>(),add(size_of::<T>(),size_of::<Option<Arc<T>>>())?)?)?;Ok(d)}

/// 🧰️ Allocates one genuine shared header and returns the exact original input on refusal.
pub fn admit_shared_allocation<T>(value:T,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<(SharedAllocation<T>,RetainedCloneProgress),(ValueError,T)>{
 if actual.is_some(){return RetainedWorkOutcome{result:Err((ValueError::invariant("shared allocation requires an empty actual marker"),value)),work:0}}
 let kind=if grant.maximum_items==0{Some(ValueRefusalKind::WorkLimit)}else if grant.maximum_depth==0{Some(ValueRefusalKind::DepthLimit)}else if grant.maximum_capacity_bytes<shared_allocation_bytes::<T>(){Some(ValueRefusalKind::OwnershipLimit)}else if shared_allocation_value_demand::<T>().is_ok_and(|demand|grant.maximum_copy_bytes<demand.copy_bytes){Some(ValueRefusalKind::OwnershipLimit)}else{None};
 if let Some(kind)=kind{return RetainedWorkOutcome{result:Err((ValueError::literal(kind,"shared allocation birth is not funded"),value)),work:0}}
 *actual=Some(0);
 let mut storage=match Arc::<T>::try_new_uninit(){Ok(storage)=>{*actual=Some(shared_allocation_bytes::<T>());storage},Err(_)=>{*actual=Some(0);return RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::AllocationFailed,"shared allocation failed"),value)),work:1}}};
 let Some(slot)=Arc::get_mut(&mut storage) else{return RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::AllocationFailed,"new shared storage is not unique"),value)),work:1}};slot.write(value);
 let owner=unsafe{storage.assume_init()};
 RetainedWorkOutcome{result:Ok((owner,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:shared_allocation_bytes::<T>(),..Default::default()})),work:2}
}
pub fn try_uninitialized_arc<T>(grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<(Arc<MaybeUninit<T>>,RetainedCloneProgress)> {
 if actual.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("shared allocation marker is occupied")),work:0}}
 let d=match shared_uninitialized_demand::<T>(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
 if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"shared uninitialized allocation is not funded")),work:0}}
 *actual=Some(0);match Arc::<T>::try_new_uninit(){Ok(storage)=>{*actual=Some(d.capacity_bytes);RetainedWorkOutcome{result:Ok((storage,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:1}},Err(_)=>RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"shared allocation failed")),work:1}}
}

pub fn admit_shared_allocation_from<T>(source:&mut Option<T>,target:&mut Option<SharedAllocation<T>>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome {
 if target.is_some()||actual.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("shared original recipient or allocation marker is occupied")),work:0}}
 if source.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
 let d=match shared_allocation_slot_demand::<T>(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
 if !retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
 let turn=try_uninitialized_arc::<T>(grant,actual);let mut storage=match turn.result{Ok((storage,_))=>storage,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:turn.work}};
 let value=source.take().expect("shared original was validated before allocation");Arc::get_mut(&mut storage).expect("fresh shared allocation has one owner").write(value);*target=Some(unsafe{storage.assume_init()});
 RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:turn.work+3}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// ⛽️ One bounded native acquisition includes opaque Arc headers and resident publication.
pub const fn shared_uninitialized_work_demand()->u64{1}
/// 📥️ Publishes the genuine empty Arc before this native allocation primitive returns.
pub fn try_uninitialized_arc_into<T>(output:&mut Option<Arc<MaybeUninit<T>>>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
 let before=effect.performed_boundary();let result=(||{
  if output.is_some()||effect.actual().is_some(){return Err(ValueError::invariant("shared resident output or allocation marker is occupied"))}
  let demand=shared_uninitialized_demand::<T>()?;
  if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<shared_uninitialized_work_demand(){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !effect.performed(shared_uninitialized_work_demand())?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared acquisition exceeded its original work receipt"))}
  *effect.actual()=Some(0);
  match Arc::<T>::try_new_uninit(){
   Ok(storage)=>{*effect.actual()=Some(demand.capacity_bytes);*output=Some(storage);Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))}
   Err(_)=>Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"shared allocation failed").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<usize>>(),..Default::default()}))
  }
 })();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same original shared acquisition receipt")}
}
/// 📏️ Prices the retained empty handle and native counters before physical release.
pub fn uninitialized_arc_release_demand<T>(output:&Option<Arc<MaybeUninit<T>>>)->RetirementDemand{if output.is_none(){Default::default()}else{RetirementDemand{copy_bytes:size_of::<Option<Arc<MaybeUninit<T>>>>()+2*size_of::<usize>(),release_bytes:shared_allocation_bytes::<T>(),depth:1,..Default::default()}}}
/// 🧹️ Releases only a physically unique empty Arc through its same original effect.
pub fn release_uninitialized_arc_from<T>(output:&mut Option<Arc<MaybeUninit<T>>>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
 let before=effect.performed_boundary();let result=(||{
  if output.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=uninitialized_arc_release_demand(output);
  if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared release exceeded its original work receipt"))}
  if Arc::get_mut(output.as_mut().expect("retained empty shared output")).is_none(){return Ok(RetainedCloneStep::Progress(Default::default()))}
  drop(output.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
 })();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same original shared release receipt")}
}

/// 📐️ Quotes the original payload, both resident handles and native uniqueness counters.
pub fn shared_arc_publication_demand<T>()->Result<RetirementDemand,ValueError>{let copy=add(size_of::<Option<T>>(),size_of::<T>())?;let copy=add(copy,size_of::<Option<Arc<MaybeUninit<T>>>>())?;let copy=add(copy,add(size_of::<Arc<T>>(),size_of::<Option<Arc<T>>>())?)?;Ok(RetirementDemand{copy_bytes:add(copy,2*size_of::<usize>())?,depth:1,..Default::default()})}
/// ⛽️ One uniqueness operation precedes the two original payload and handle transfers.
pub const fn shared_arc_publication_work_demand()->u64{3}
/// 📤️ Initializes only the same unique backing and deposits the original typed Arc before return.
pub fn publish_uninitialized_arc_from<T>(source:&mut Option<Arc<MaybeUninit<T>>>,input:&mut Option<T>,output:&mut Option<Arc<T>>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
 let before=effect.performed_boundary();let result=(||{
  if source.is_none()||input.is_none()||output.is_some(){return Err(ValueError::invariant("shared publication requires its original backing, input and empty output"))}
  let demand=shared_arc_publication_demand::<T>()?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<shared_arc_publication_work_demand(){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared publication uniqueness exceeded original work"))}
  let Some(storage)=Arc::get_mut(source.as_mut().expect("funded shared publication backing"))else{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<usize>(),..Default::default()}))};
  if !effect.performed(2)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared publication transfer exceeded original work").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<usize>(),..Default::default()}))}
  storage.write(input.take().expect("funded original shared payload"));let storage=source.take().expect("initialized original shared backing");*output=Some(unsafe{storage.assume_init()});Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 })();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same original shared publication receipt")}
}
