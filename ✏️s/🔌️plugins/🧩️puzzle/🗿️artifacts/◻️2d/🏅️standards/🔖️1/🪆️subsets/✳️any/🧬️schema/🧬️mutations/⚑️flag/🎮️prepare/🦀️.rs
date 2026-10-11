//! ⚑️ Borrowed typed node and edge flag intents preserve authored optional state.
use super::{ChangeNodeLocked,ChangeNodeVisible,ChangeEdgeLocked,ChangeEdgeVisible,ChangeNodeRoot,ChangeTargetRegionHidden,ChangeTargetRegionLocked,Puzzle2dMutation};
use crate::{Puzzle2dSnapshot,standards::v1::subsets::any::schema::snapshot::lookup::{Puzzle2dLookupCursor,Puzzle2dLookupScope,Puzzle2dLookupStep}};
use semio_framework_value::{paged::PagedUtf8,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep}};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Puzzle2dFlagDisposition {Changed,NoOp,TargetMissing}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Puzzle2dFlagPlan {pub disposition:Puzzle2dFlagDisposition,pub index:Option<usize>,pub previous:Option<Option<bool>>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Puzzle2dFlagPreparationStep {Pending(RetainedCloneProgress),Complete{plan:Puzzle2dFlagPlan,progress:RetainedCloneProgress}}

pub trait Puzzle2dFlagIntent:Sync {
 const SCOPE:Puzzle2dLookupScope;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>;
 fn next(&self)->Option<bool>;
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->Option<bool>;
 fn identifier_at(snapshot:&Puzzle2dSnapshot,index:usize)->&PagedUtf8<{usize::MAX}>{match Self::SCOPE{Puzzle2dLookupScope::Node=>&snapshot.nodes.get(index).expect("immutable flag node").id,Puzzle2dLookupScope::Edge=>&snapshot.edges.get(index).expect("immutable flag edge").id,Puzzle2dLookupScope::Region=>&snapshot.target_regions.get(index).expect("immutable flag region").id,_=>unreachable!()}}
 fn place(snapshot:&mut Puzzle2dSnapshot,index:usize,value:Option<bool>)->Result<(),ValueError>;
 fn restore(id:PagedUtf8<{usize::MAX}>,previous:Option<bool>)->Puzzle2dMutation;
}
impl Puzzle2dFlagIntent for ChangeNodeLocked {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Node;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->Option<bool>{self.new_locked}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->Option<bool>{snapshot.nodes.get(index).expect("immutable flag node").locked}
 fn restore(id:PagedUtf8<{usize::MAX}>,previous:Option<bool>)->Puzzle2dMutation{Puzzle2dMutation::ChangeNodeLocked(Self{id,new_locked:previous})}
 fn place(snapshot:&mut Puzzle2dSnapshot,index:usize,value:Option<bool>)->Result<(),ValueError>{snapshot.nodes.get_mut(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"native flag candidate lost its original ordinal"))?.locked=value;Ok(())}
}
impl Puzzle2dFlagIntent for ChangeNodeVisible {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Node;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->Option<bool>{self.new_visible}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->Option<bool>{snapshot.nodes.get(index).expect("immutable flag node").visible}
 fn restore(id:PagedUtf8<{usize::MAX}>,previous:Option<bool>)->Puzzle2dMutation{Puzzle2dMutation::ChangeNodeVisible(Self{id,new_visible:previous})}
 fn place(snapshot:&mut Puzzle2dSnapshot,index:usize,value:Option<bool>)->Result<(),ValueError>{snapshot.nodes.get_mut(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"native flag candidate lost its original ordinal"))?.visible=value;Ok(())}
}
impl Puzzle2dFlagIntent for ChangeEdgeLocked {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Edge;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->Option<bool>{self.new_locked}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->Option<bool>{snapshot.edges.get(index).expect("immutable flag edge").locked}
 fn restore(id:PagedUtf8<{usize::MAX}>,previous:Option<bool>)->Puzzle2dMutation{Puzzle2dMutation::ChangeEdgeLocked(Self{id,new_locked:previous})}
 fn place(snapshot:&mut Puzzle2dSnapshot,index:usize,value:Option<bool>)->Result<(),ValueError>{snapshot.edges.get_mut(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"native flag candidate lost its original ordinal"))?.locked=value;Ok(())}
}
impl Puzzle2dFlagIntent for ChangeEdgeVisible {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Edge;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->Option<bool>{self.new_visible}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->Option<bool>{snapshot.edges.get(index).expect("immutable flag edge").visible}
 fn restore(id:PagedUtf8<{usize::MAX}>,previous:Option<bool>)->Puzzle2dMutation{Puzzle2dMutation::ChangeEdgeVisible(Self{id,new_visible:previous})}
 fn place(snapshot:&mut Puzzle2dSnapshot,index:usize,value:Option<bool>)->Result<(),ValueError>{snapshot.edges.get_mut(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"native flag candidate lost its original ordinal"))?.visible=value;Ok(())}
}

impl Puzzle2dFlagIntent for ChangeNodeRoot {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Node;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->Option<bool>{self.new_root}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->Option<bool>{snapshot.nodes.get(index).expect("immutable root flag node").root}
 fn restore(id:PagedUtf8<{usize::MAX}>,previous:Option<bool>)->Puzzle2dMutation{Puzzle2dMutation::ChangeNodeRoot(Self{id,new_root:previous})}
 fn place(snapshot:&mut Puzzle2dSnapshot,index:usize,value:Option<bool>)->Result<(),ValueError>{snapshot.nodes.get_mut(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"native root flag candidate lost its original ordinal"))?.root=value;Ok(())}
}

impl Puzzle2dFlagIntent for ChangeTargetRegionHidden {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Region;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->Option<bool>{Some(self.new_hidden)}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->Option<bool>{Some(snapshot.target_regions.get(index).expect("immutable required flag region").hidden)}
 fn restore(id:PagedUtf8<{usize::MAX}>,previous:Option<bool>)->Puzzle2dMutation{Puzzle2dMutation::ChangeTargetRegionHidden(Self{id,new_hidden:previous.expect("native required region flag")})}
 fn place(snapshot:&mut Puzzle2dSnapshot,index:usize,value:Option<bool>)->Result<(),ValueError>{let value=value.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"required region flag cannot be absent"))?;snapshot.target_regions.get_mut(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"native region flag candidate lost its original ordinal"))?.hidden=value;Ok(())}
}

impl Puzzle2dFlagIntent for ChangeTargetRegionLocked {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Region;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->Option<bool>{Some(self.new_locked)}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->Option<bool>{Some(snapshot.target_regions.get(index).expect("immutable required flag region").locked)}
 fn restore(id:PagedUtf8<{usize::MAX}>,previous:Option<bool>)->Puzzle2dMutation{Puzzle2dMutation::ChangeTargetRegionLocked(Self{id,new_locked:previous.expect("native required region flag")})}
 fn place(snapshot:&mut Puzzle2dSnapshot,index:usize,value:Option<bool>)->Result<(),ValueError>{let value=value.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"required region flag cannot be absent"))?;snapshot.target_regions.get_mut(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"native region flag candidate lost its original ordinal"))?.locked=value;Ok(())}
}

pub struct Puzzle2dFlagPreparationCursor<T:Puzzle2dFlagIntent> {source:Option<RetainedCloneBinding>,mutation:Option<RetainedCloneBinding>,lookup:Puzzle2dLookupCursor,index:Option<usize>,phase:u8,output:Option<Puzzle2dFlagPlan>,spent:bool,closing:bool,kind:std::marker::PhantomData<T>}
impl<T:Puzzle2dFlagIntent> Default for Puzzle2dFlagPreparationCursor<T>{fn default()->Self{Self{source:None,mutation:None,lookup:Puzzle2dLookupCursor::new(T::SCOPE),index:None,phase:0,output:None,spent:false,closing:false,kind:std::marker::PhantomData}}}
impl<T:Puzzle2dFlagIntent> Puzzle2dFlagPreparationCursor<T>{
 pub fn advance(&mut self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,mutation:RetainedCloneRef<'_,T>,grant:RetainedCloneGrant)->Result<Puzzle2dFlagPreparationStep,ValueError>{
  if self.closing||self.spent{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"flag preparation is closing or spent"))}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(Puzzle2dFlagPreparationStep::Pending(Default::default()))}
  if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(Puzzle2dFlagPreparationStep::Pending(progress))}if let Some(progress)=mutation.bind(&mut self.mutation,grant)?{return Ok(Puzzle2dFlagPreparationStep::Pending(progress))}
  if let Some(plan)=self.output{return Ok(Puzzle2dFlagPreparationStep::Complete{plan,progress:Default::default()})}
  if self.phase==0{return match self.lookup.advance(source,mutation.project(1,T::identifier),grant)?{Puzzle2dLookupStep::Pending(progress)=>Ok(Puzzle2dFlagPreparationStep::Pending(progress)),Puzzle2dLookupStep::Complete{location,progress}=>{self.index=location.map(|value|value.outer);self.lookup.take();self.phase=1;Ok(Puzzle2dFlagPreparationStep::Pending(progress))}}}
  let bytes=2*std::mem::size_of::<Option<bool>>();if grant.maximum_copy_bytes<bytes{return Ok(Puzzle2dFlagPreparationStep::Pending(Default::default()))}
  let previous=self.index.map(|index|T::previous(source.get(),index));let disposition=match previous{None=>Puzzle2dFlagDisposition::TargetMissing,Some(previous)if previous==mutation.get().next()=>Puzzle2dFlagDisposition::NoOp,Some(_)=>Puzzle2dFlagDisposition::Changed};
  let plan=Puzzle2dFlagPlan{disposition,index:self.index,previous};self.output=Some(plan);Ok(Puzzle2dFlagPreparationStep::Complete{plan,progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}})
 }
 pub fn take(&mut self)->Option<Puzzle2dFlagPlan>{if self.closing{return None}let output=self.output.take();if output.is_some(){self.spent=true}output}
 pub fn begin_close(&mut self){self.closing=true;self.lookup.begin_close();}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if self.output.is_some(){return Ok(std::mem::size_of_val(&self.output))}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_copy_byte_demand()}RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
 pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if self.output.is_some(){return Ok(0)}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_capacity_byte_demand(body)}RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.mutation},body)}
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if self.output.is_some(){return Ok(0)}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_release_byte_demand()}RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{if self.output.is_some(){return Ok(1)}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_depth_demand()}RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"flag preparation closure was not started"))}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.output.is_some(){let bytes=std::mem::size_of_val(&self.output);if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}self.output=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))}
  if !self.lookup.terminal_is_empty(){return self.lookup.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
  let step=RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation},grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.lookup.terminal_is_empty()&&self.source.is_none()&&self.mutation.is_none()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="↩️inverse/🦀️.rs"]
pub mod inverse;

#[path="📸️candidate/🦀️.rs"]
pub mod candidate;
