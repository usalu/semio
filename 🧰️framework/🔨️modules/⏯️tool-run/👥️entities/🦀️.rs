//! 👥️ Caller-owned ToolRun entity edit workspace and exact original retirement.
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedAllocationEffect,RetainedWorkPoll,RetainedClone,RetainedCloneGrant,RetainedCloneProgress,StagedSourceAdmission,ordered_map::{BoundedOrd,BoundedOrdGrant,BoundedOrdProgress,RetainedOrderedMap,RetainedOrderedMapInsertCursor,RetainedOrderedMapInsertGrant,RetainedOrderedMapRemoveCursor,RetainedOrderedMapRemoveGrant,RETAINED_ORDERED_MAP_PAGE_CAPACITY}},retirement::RetireOwned};
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
use std::mem::{ManuallyDrop,size_of};

pub type EntityMap=RetainedOrderedMap<u64,()>;

/// 🏁️ Settles one finished retirement poll into its physical receipt; a refused or suspended close is a fault of this workspace.
pub fn settle_close(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneProgress,ValueError>{match poll{RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>Ok(step.progress()),Err(failure)=>{let progress=failure.retained_progress();drop(failure);Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"entity workspace close was refused").with_retained_progress(progress))}},RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("entity workspace close stayed pending"))}}

/// ⛽️ Records one sealed-primitive turn's work on the surrounding effect; a refused debit is a workspace fault.
pub fn record_work(effect:&mut RetainedAllocationEffect<'_>,work:u128)->Result<(),ValueError>{let work=u64::try_from(work).map_err(|_|ValueError::invariant("entity workspace work exceeds a turn"))?;if effect.performed(work)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"entity workspace work is not funded"))}}

/// 🧱️ Reserves one exact vector backing as the single allocation attempt of this effect and records its genuine size.
pub fn reserve_vec<T>(count:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<Vec<T>,ValueError>{
 let actual=effect.actual();if actual.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"entity workspace allocation attempt is occupied"));}
 let mut vector=Vec::new();let reserved=vector.try_reserve_exact(count);*actual=Some(if reserved.is_ok(){vector.capacity()*size_of::<T>()}else{0});
 reserved.map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"entity workspace backing allocation failed"))?;Ok(vector)
}

/// 🏷️ Admits one sealed original under its quoted work; a refused quote hands the value back, surplus beyond the quote is debited, and a refused surplus rolls the fresh admission back to its value.
pub fn admit_sealed<T:RetireOwned+Send+Sync+'static>(value:T,effect:&mut RetainedAllocationEffect<'_>)->Result<(SealedShared<T>,RetainedCloneProgress),(ValueError,T)>{
 let quote=u128::from(SealedShared::<T>::birth_work_demand());if let Err(error)=record_work(effect,quote){return Err((error,value));}
 let turn=SealedShared::admit(value,SharedIssuer::owned(),effect.grant(),effect.actual());let surplus=turn.work.saturating_sub(quote);
 if surplus>0{if let Err(error)=record_work(effect,surplus){return Err((error,match turn.result{Err((_,value))=>value,Ok((sealed,_))=>semio_framework_value_shared_issuance::cold_take_exclusive(sealed)}));}}
 turn.result
}

/// 🪞️ Duplicates one sealed alias; the alias turn debits its own quoted work on the effect exactly once, a refused quote duplicates nothing.
pub fn duplicate_sealed<T:Send+Sync+'static>(source:&SealedShared<T>,effect:&mut RetainedAllocationEffect<'_>)->Result<(SealedShared<T>,RetainedCloneProgress),ValueError>{
 source.try_duplicate(effect).result
}

/// 📤️ Publishes one sealed original through a funded birth and publication, paying the birth back when refused.
pub fn publish_sealed<T:RetireOwned+Send+Sync+'static>(effect:&mut RetainedAllocationEffect<'_>,value:impl FnOnce()->Result<T,ValueError>)->Result<(SealedShared<T>,RetainedCloneProgress),ValueError>{
 let grant=effect.grant();
 let turn=SealedShared::<T>::prepare_birth(grant,effect.actual());record_work(effect,turn.work)?;let(mut birth,born)=turn.result?;
 let publication=match birth.publication(grant){Ok(Some(publication))=>publication,Ok(None)=>{let turn=birth.close_step(effect);record_work(effect,turn.work)?;return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed publication is not funded"));},Err(error)=>{let turn=birth.close_step(effect);record_work(effect,turn.work)?;return Err(error);}};
 let value=match value(){Ok(value)=>value,Err(error)=>{drop(publication);let turn=birth.close_step(effect);record_work(effect,turn.work)?;return Err(error);}};
 let turn=publication.commit(value,SharedIssuer::owned());record_work(effect,turn.work)?;let(shared,mut progress)=turn.result?;progress.retained_capacity_bytes=born.retained_capacity_bytes;Ok((shared,progress))
}

#[path="📸️version/🦀️.rs"]
pub mod version;
#[path="🧾️provenance/🦀️.rs"]
pub mod provenance;
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum ToolRunEntityEditKind {Append,Retract}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ToolRunEntityEditGrant {pub retirement:RetainedCloneGrant,pub comparison:BoundedOrdGrant,pub maximum_moved_items:usize,pub maximum_moved_bytes:usize,pub maximum_capacity_bytes:usize}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ToolRunEntityEditProgress {pub retirement:RetainedCloneProgress,pub comparison:BoundedOrdProgress,pub moved_items:usize,pub moved_bytes:usize,pub retained_capacity_bytes:usize}
impl ToolRunEntityEditProgress {pub fn fits(self,grant:ToolRunEntityEditGrant)->bool {self.retirement.fits(grant.retirement)&&self.comparison.fits(grant.comparison)&&self.moved_items<=grant.maximum_moved_items&&self.moved_bytes<=grant.maximum_moved_bytes&&self.retained_capacity_bytes<=grant.maximum_capacity_bytes}}
#[derive(semio_framework_value::RetireOwned)]
enum Editor<K:BoundedOrd+RetainedClone+Copy> {Append(RetainedOrderedMapInsertCursor<K,()>),Retract(RetainedOrderedMapRemoveCursor<K,()>)}
impl<K:BoundedOrd+RetainedClone+Copy> Editor<K> {
 fn output_ready(&self)->bool{match self{Self::Append(owner)=>owner.output_ready()||owner.refused_workspace_ready(),Self::Retract(owner)=>owner.output_ready()}}
 fn take(&mut self)->Option<RetainedOrderedMap<K,()>>{match self{Self::Append(owner)=>if owner.refused_workspace_ready(){owner.take_refused_workspace()}else{owner.take()},Self::Retract(owner)=>owner.take()}}
 fn begin_close(&mut self){match self{Self::Append(owner)=>{owner.begin_close();},Self::Retract(owner)=>{owner.begin_close();}}}
 fn terminal_is_empty(&self)->bool{match self{Self::Append(owner)=>owner.terminal_is_empty(),Self::Retract(owner)=>owner.terminal_is_empty()}}
 fn advance_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self{Self::Append(owner)=>owner.advance_retirement_demands(body),Self::Retract(owner)=>owner.advance_retirement_demands(body)}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self{Self::Append(owner)=>{let copy=owner.next_close_copy_byte_demand()?;Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_close_capacity_byte_demand(body.max(copy))?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?})},Self::Retract(owner)=>{let copy=owner.next_close_copy_byte_demand()?;Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_close_capacity_byte_demand(body.max(copy))?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?})}}}
 fn advance(&mut self,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<ToolRunEntityEditProgress,ValueError>{match self{Self::Append(owner)=>{let progress=match owner.advance(RetainedOrderedMapInsertGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes},effect).result?{semio_framework_value::retained_clone::ordered_map::RetainedOrderedMapInsertStep::Progress(progress)|semio_framework_value::retained_clone::ordered_map::RetainedOrderedMapInsertStep::Complete{progress,..}=>progress};Ok(ToolRunEntityEditProgress{retirement:progress.retirement,comparison:progress.comparison,moved_items:progress.moved_items,moved_bytes:progress.moved_bytes,retained_capacity_bytes:progress.retained_capacity_bytes})},Self::Retract(owner)=>{let progress=owner.advance(RetainedOrderedMapRemoveGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes},effect).result?.progress();Ok(ToolRunEntityEditProgress{retirement:progress.retirement,comparison:progress.comparison,moved_items:progress.moved_items,moved_bytes:progress.moved_bytes,..Default::default()})}}}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{settle_close(match self{Self::Append(owner)=>owner.close_step(effect),Self::Retract(owner)=>owner.close_step(effect)})}
}
#[derive(semio_framework_value::RetireOwned)]
#[doc(hidden)]
pub struct State<K:BoundedOrd+RetainedClone+Copy> {map:Option<RetainedOrderedMap<K,()>>,targets:Vec<K>,index:usize,kind:ToolRunEntityEditKind,editor:Option<Editor<K>>,phase:u8,capacity_bound:usize,cancelled:bool}
/// 👥️ Owns genuine map leaves and target backing until a funded transfer or close.
pub struct ToolRunEntityEdit<K:BoundedOrd+RetainedClone+Copy=u64> {state:ManuallyDrop<State<K>>,transferred:bool}
impl<K:BoundedOrd+RetainedClone+Copy> ToolRunEntityEdit<K> {
 pub fn capacity_bound_for(entries:usize,targets:usize)->Option<usize>{entries.checked_add(targets).and_then(|len|len.checked_add(RETAINED_ORDERED_MAP_PAGE_CAPACITY-1)).map(|len|len/RETAINED_ORDERED_MAP_PAGE_CAPACITY).and_then(|pages|pages.checked_mul(2)).and_then(|pages|pages.checked_add(1)).and_then(|pages|pages.checked_mul(size_of::<Vec<(K,())>>())).map(|bytes|bytes.max(RETAINED_ORDERED_MAP_PAGE_CAPACITY*size_of::<(K,())>()))}
 pub fn new(map:RetainedOrderedMap<K,()>,targets:Vec<K>,kind:ToolRunEntityEditKind)->Self{let capacity_bound=Self::capacity_bound_for(map.len(),targets.len()).unwrap_or(usize::MAX);Self{state:ManuallyDrop::new(State{map:Some(map),targets,index:0,kind,editor:None,phase:0,capacity_bound,cancelled:false}),transferred:false}}
 pub fn targets_identity(&self)->usize{self.state.targets.as_ptr()as usize}
 pub fn edit_capacity_bound(&self)->Result<usize,ValueError>{if self.state.capacity_bound==usize::MAX{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity edit capacity overflow"))}else{Ok(self.state.capacity_bound)}}
 pub fn complete(&self)->bool{self.state.phase==3&&!self.state.cancelled}
 pub fn cancel(&mut self){self.state.cancelled=true;}
 pub fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{let state=&*self.state;if state.cancelled||state.phase==3{return Ok(Default::default());}if state.phase==2 {return match state.editor.as_ref(){Some(owner)if !owner.terminal_is_empty()=>owner.close_demands(body),_=>Ok(RetirementDemand{depth:1,..Default::default()})};}if let Some(owner)=state.editor.as_ref(){if owner.output_ready(){return Ok(RetirementDemand{copy_bytes:size_of::<RetainedOrderedMap<K,()>>(),depth:1,..Default::default()});}let demand=owner.advance_demands(body)?;return owner.advance_demands(body.max(demand.copy_bytes));}if state.index==state.targets.len(){return Ok(RetirementDemand{depth:1,..Default::default()});}Ok(match state.kind{ToolRunEntityEditKind::Append=>RetirementDemand{copy_bytes:RetainedOrderedMapInsertCursor::<K,()>::constructor_copy_bytes(),capacity_bytes:RetainedOrderedMapInsertCursor::<K,()>::constructor_capacity_bytes(),depth:1,..Default::default()},ToolRunEntityEditKind::Retract=>RetirementDemand{copy_bytes:RetainedOrderedMapRemoveCursor::<K,()>::constructor_copy_bytes(),capacity_bytes:RetainedOrderedMapRemoveCursor::<K,()>::constructor_capacity_bytes(),depth:1,..Default::default()}})}
 pub fn advance(&mut self,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<ToolRunEntityEditProgress,ValueError>{
  if grant.retirement.maximum_items==0||self.state.cancelled||self.state.phase==3{return Ok(Default::default());}
  let demand=self.next_demands(grant.retirement.maximum_copy_bytes)?;if demand.copy_bytes>grant.retirement.maximum_copy_bytes||demand.capacity_bytes>grant.retirement.maximum_capacity_bytes||demand.release_bytes>grant.retirement.maximum_release_bytes||demand.depth>grant.retirement.maximum_depth{return Ok(Default::default());}
  let state=&mut *self.state;
  if state.phase==2{let owner=state.editor.as_mut().unwrap();if !owner.terminal_is_empty(){return Ok(ToolRunEntityEditProgress{retirement:owner.close_step(effect)?,..Default::default()});}state.editor=None;state.index+=1;state.phase=0;return Ok(ToolRunEntityEditProgress{retirement:RetainedCloneProgress{copied_items:1,..Default::default()},..Default::default()});}
  if let Some(owner)=state.editor.as_mut(){if owner.output_ready(){state.map=owner.take();if state.map.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"entity editor lost its original output"));}owner.begin_close();state.phase=2;return Ok(ToolRunEntityEditProgress{retirement:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<RetainedOrderedMap<K,()>>(),..Default::default()},..Default::default()});}return owner.advance(grant,effect);}
  if state.index==state.targets.len(){state.phase=3;return Ok(ToolRunEntityEditProgress{retirement:RetainedCloneProgress{copied_items:1,..Default::default()},..Default::default()});}
  let target=state.targets[state.index];let map=state.map.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"entity editor requires its original map"))?;
  let receipt=match state.kind{ToolRunEntityEditKind::Append=>match RetainedOrderedMapInsertCursor::admit(map,target,(),grant.retirement).result{Ok((owner,receipt))=>{state.editor=Some(Editor::Append(owner));receipt},Err((error,map,_,_))=>{state.map=Some(map);return Err(error);}},ToolRunEntityEditKind::Retract=>match RetainedOrderedMapRemoveCursor::admit(map,target,grant.retirement).result{Ok((owner,receipt))=>{state.editor=Some(Editor::Retract(owner));receipt},Err((error,map,_))=>{state.map=Some(map);return Err(error);}}};state.phase=1;Ok(ToolRunEntityEditProgress{retirement:receipt,..Default::default()})
 }
 pub fn take_output(&mut self,grant:RetainedCloneGrant)->Result<Option<(RetainedOrderedMap<K,()>,RetainedCloneProgress)>,ValueError>{if !self.complete()||self.state.map.is_none()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<RetainedOrderedMap<K,()>>()||grant.maximum_depth==0{return Ok(None);}Ok(Some((self.state.map.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<RetainedOrderedMap<K,()>>(),..Default::default()})))}
}
impl<K:BoundedOrd+RetainedClone+Copy> RetireOwned for ToolRunEntityEdit<K> {
 type RetirementCursor=<State<K> as RetireOwned>::RetirementCursor;
 fn retirement_seed(mut self)->Self::RetirementCursor{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
impl<K:BoundedOrd+RetainedClone+Copy> Drop for ToolRunEntityEdit<K> {fn drop(&mut self){assert!(self.transferred,"original entity edit workspace requires controlled retirement");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[global_allocator]
static TOOL_RUN_ENTITIES_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[cfg(test)]
mod sealed_quote_tests{
 use super::*;
 /// 🧪️ The sealed birth quote equals the work a funded admission performs, so no surplus debit exists to drop.
 #[test]
 fn sealed_birth_quote_is_exact(){
  fn exact<T:RetireOwned+Send+Sync+'static>(value:T){
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:8};
   let mut actual=None;let turn=SealedShared::admit(value,SharedIssuer::owned(),grant,&mut actual);
   assert_eq!(turn.work,u128::from(SealedShared::<T>::birth_work_demand()));
   std::mem::forget(turn.result.map_err(|(error,_)|error).unwrap());
  }
  exact(String::from("draw.command"));exact(vec![1u8,2,3]);exact(7u64);
 }
}
