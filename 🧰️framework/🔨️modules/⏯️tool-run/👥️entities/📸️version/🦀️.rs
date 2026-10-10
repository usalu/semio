//! 📸️ Original immutable entity versions publish through caller-funded private clone and edit workspaces.
use super::{EntityMap,ToolRunEntityEdit,ToolRunEntityEditKind,ToolRunEntityEditGrant,ToolRunEntityEditProgress};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneSource,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ordered_map::RetainedOrderedMapCloneCursor},retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement,shared::sealed::{SealedShared,SharedIssuer}}};
use std::mem::{ManuallyDrop,size_of};

#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunEntityVersion {entities:SealedShared<EntityMap>,revision:u64}
impl ToolRunEntityVersion {
 pub fn birth_bytes()->usize{SealedShared::<EntityMap>::birth_bytes()}
 pub fn capture_copy_bytes()->usize{size_of::<Self>()}
 pub fn admit(map:EntityMap,revision:u64,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,EntityMap)>{SealedShared::admit(map,SharedIssuer::owned(),grant).map(|(entities,progress)|(Self{entities,revision},progress))}
 pub fn capture(&self,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{if grant.maximum_copy_bytes<Self::capture_copy_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity version capture requires its original fixed-body copy"));}let(entities,mut progress)=self.entities.try_duplicate(grant)?;progress.copied_bytes=Self::capture_copy_bytes();Ok((Self{entities,revision:self.revision},progress))}
 pub fn entities(&self)->&EntityMap{self.entities.get()}
 pub fn revision(&self)->u64{self.revision}
 pub fn identity(&self)->usize{self.entities.identity()}
}
#[derive(semio_framework_value::RetireOwned)]
struct State {original:Option<ToolRunEntityVersion>,source:Option<RetainedCloneSource<EntityMap,ToolRunEntityVersion>>,clone:Option<RetainedOrderedMapCloneCursor<u64,()>>,edit:Option<ToolRunEntityEdit>,edit_close:Option<ControlledRetirement<ToolRunEntityEdit>>,candidate:Option<EntityMap>,targets:Option<Vec<u64>>,kind:ToolRunEntityEditKind,next_revision:Option<u64>,capacity_bound:usize,phase:u8,cancelled:bool}
/// 🏭️ Keeps each captured original and unpublished candidate until real publication or controlled closure.
pub struct ToolRunEntityVersionEdit {state:ManuallyDrop<State>,transferred:bool}
impl ToolRunEntityVersionEdit {
 pub fn new(original:ToolRunEntityVersion,targets:Vec<u64>,kind:ToolRunEntityEditKind)->Self{let capacity_bound=ToolRunEntityEdit::<u64>::capacity_bound_for(original.entities().len(),targets.len()).unwrap_or(usize::MAX);let next_revision=original.revision.checked_add(1);Self{state:ManuallyDrop::new(State{original:Some(original),source:None,clone:Some(EntityMap::retained_clone_cursor()),edit:None,edit_close:None,candidate:None,targets:Some(targets),kind,next_revision,capacity_bound,phase:0,cancelled:false}),transferred:false}}
 pub fn complete(&self)->bool{self.state.phase==7&&!self.state.cancelled}
 pub fn cancel(&mut self){self.state.cancelled=true;if let Some(edit)=self.state.edit.as_mut(){edit.cancel();}}
 pub fn edit_capacity_bound(&self)->Result<usize,ValueError>{if self.state.capacity_bound==usize::MAX{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity version edit capacity overflow"))}else{Ok(self.state.capacity_bound)}}
 pub fn publication_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<EntityMap>()+size_of::<ToolRunEntityVersion>(),capacity_bytes:ToolRunEntityVersion::birth_bytes(),depth:1,..Default::default()}}
 pub fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{let state=&*self.state;if state.cancelled||state.phase>=7{return Ok(Default::default());}if state.next_revision.is_none(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity version revision overflow"));}Ok(match state.phase{
  0=>RetirementDemand{copy_bytes:RetainedCloneSource::<EntityMap>::borrowed_constructor_copy_bytes::<ToolRunEntityVersion>(),capacity_bytes:RetainedCloneSource::<EntityMap>::borrowed_constructor_capacity_bytes::<ToolRunEntityVersion>(),depth:1,..Default::default()},
  1=>state.clone.as_ref().unwrap().advance_demands(state.source.as_ref().unwrap().borrow(),body)?,
  2=>RetirementDemand{copy_bytes:size_of::<EntityMap>()+size_of::<Vec<u64>>()+size_of::<ToolRunEntityEditKind>(),depth:1,..Default::default()},
  3=>{let child=state.clone.as_ref().unwrap();if child.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{let copy=child.next_close_copy_byte_demand()?;RetirementDemand{copy_bytes:copy,capacity_bytes:child.next_close_capacity_byte_demand(body.max(copy))?,release_bytes:child.next_close_release_byte_demand()?,depth:child.next_close_depth_demand()?.max(1)}}},
  4=>{let child=state.edit.as_ref().unwrap();if child.complete(){RetirementDemand{depth:1,..Default::default()}}else{child.next_demands(body)?}},
  5=>RetirementDemand{copy_bytes:size_of::<EntityMap>()+size_of::<ToolRunEntityEdit>(),depth:1,..Default::default()},
  6=>{let child=state.edit_close.as_ref().unwrap();if child.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{let copy=child.next_copy_byte_demand()?;RetirementDemand{copy_bytes:copy,capacity_bytes:child.next_capacity_byte_demand(body.max(copy))?,release_bytes:child.next_release_byte_demand()?,depth:child.next_depth_demand()?}}},
  _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"entity version publisher phase is invalid")),
 })}
 pub fn advance(&mut self,grant:ToolRunEntityEditGrant)->Result<ToolRunEntityEditProgress,ValueError>{if self.state.cancelled||self.state.phase>=7||grant.retirement.maximum_items==0{return Ok(Default::default());}let demand=self.next_demands(grant.retirement.maximum_copy_bytes)?;if demand.copy_bytes>grant.retirement.maximum_copy_bytes||demand.capacity_bytes>grant.retirement.maximum_capacity_bytes||demand.release_bytes>grant.retirement.maximum_release_bytes||demand.depth>grant.retirement.maximum_depth{return Ok(Default::default());}let state=&mut *self.state;let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};match state.phase{
  0=>{let original=state.original.take().unwrap();match RetainedCloneSource::<EntityMap>::admit_borrowed(original,ToolRunEntityVersion::entities,grant.retirement){Ok((source,progress))=>{state.source=Some(source);state.phase=1;receipt=progress;},Err((error,original))=>{state.original=Some(original);return Err(error);}}},
  1=>{let step=state.clone.as_mut().unwrap().advance(state.source.as_ref().unwrap().borrow(),grant.retirement)?;receipt=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){state.phase=2;}},
  2=>{let map=state.clone.as_mut().unwrap().take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"entity clone completed without its original candidate"))?;state.edit=Some(ToolRunEntityEdit::new(map,state.targets.take().unwrap(),state.kind));state.clone.as_mut().unwrap().begin_close();state.phase=3;receipt.copied_bytes=demand.copy_bytes;},
  3=>{let child=state.clone.as_mut().unwrap();if !child.terminal_is_empty(){receipt=child.close_step(grant.retirement)?.progress();}else{state.clone=None;state.phase=4;}},
  4=>{let child=state.edit.as_mut().unwrap();if child.complete(){state.phase=5;}else{return child.advance(grant);}},
  5=>{let(map,_)=state.edit.as_mut().unwrap().take_output(grant.retirement)?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"entity edit completed without its candidate"))?;state.candidate=Some(map);let edit=state.edit.take().unwrap();match ControlledRetirement::new(edit){Ok(owner)=>state.edit_close=Some(owner),Err((error,edit))=>{state.edit=Some(edit);return Err(error);}}state.phase=6;receipt.copied_bytes=demand.copy_bytes;},
  6=>{let child=state.edit_close.as_mut().unwrap();if !child.terminal_is_empty(){receipt=child.step(grant.retirement)?.progress();}else{state.edit_close=None;state.phase=7;}},
  _=>unreachable!(),
 }Ok(ToolRunEntityEditProgress{retirement:receipt,..Default::default()})}
 pub fn take_output(&mut self,grant:RetainedCloneGrant)->Result<Option<(ToolRunEntityVersion,RetainedCloneProgress)>,ValueError>{if !self.complete(){return Ok(None);}let demand=self.publication_demands();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok(None);}let birth=SealedShared::<EntityMap>::prepare_birth(grant)?;let map=self.state.candidate.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"entity publication has no private candidate"))?;let(entities,mut progress)=birth.materialize(map,SharedIssuer::owned());progress.copied_bytes=demand.copy_bytes;self.state.phase=8;Ok(Some((ToolRunEntityVersion{entities,revision:self.state.next_revision.unwrap()},progress)))}
}
impl RetireOwned for ToolRunEntityVersionEdit {
 fn retirement(mut self)->Box<dyn RetirementCursor>{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
impl Drop for ToolRunEntityVersionEdit{fn drop(&mut self){assert!(self.transferred,"original entity version publisher requires controlled retirement");}}
