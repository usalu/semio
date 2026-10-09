//! 🌳️ Funded projection of an original borrowed tree into the execution owner.
use crate::{Tree,Neuron,Synapse,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};
#[derive(semio_framework_value::RetireOwned)]
struct CopyOwners{output:Option<Tree>,neuron:Option<Neuron>,synapse:Option<Synapse>,buffer:Vec<u8>,child:Option<Box<BudgetedTreeCopy>>,nested:Option<Tree>}
/// 📋️ Copies only execution strings and retains the same original immutable parameter leases.
pub struct BudgetedTreeCopy{owners:ManuallyDrop<Option<CopyOwners>>,retirement:Option<ControlledRetirement<CopyOwners>>,source:usize,node:usize,edge:usize,phase:u8,closing:bool,progress:RetainedCloneProgress}
impl BudgetedTreeCopy{
 /// 🌱️ Creates an empty inline projection frontier with no heap birth or release.
 pub fn new()->Self{Self{owners:ManuallyDrop::new(Some(CopyOwners{output:Some(Tree::default()),neuron:None,synapse:None,buffer:Vec::new(),child:None,nested:None})),retirement:None,source:0,node:0,edge:0,phase:0,closing:false,progress:Default::default()}}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 fn original(&self,source:&Tree)->Result<(),ValueError>{if self.source!=0&&self.source!=source as *const Tree as usize{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tree projection source changed"))}Ok(())}
 fn demand<T:RetireOwned>(owner:&ControlledRetirement<T>,copy:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
 fn text<'a>(&self,source:&'a Tree)->Option<&'a str>{match self.phase{2=>Some(&source.neurons[self.node].id),3=>Some(&source.neurons[self.node].kind),12=>Some(&source.synapses[self.edge].id),13=>Some(&source.synapses[self.edge].from),14=>Some(&source.synapses[self.edge].to),15=>Some(&source.synapses[self.edge].from_port),16=>Some(&source.synapses[self.edge].to_port),_=>None}}
 /// 🪙️ Quotes the actual next empty-buffer, vector, or original recursive child event.
 pub fn next_demands(&self,source:&Tree,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original tree projection is closing"))}self.original(source)?;let owners=self.owners.as_ref().unwrap();let mut demand=RetirementDemand{depth:1,..Default::default()};
  if let Some(text)=self.text(source){demand.capacity_bytes=if owners.buffer.capacity()<text.len(){text.len()}else{0};demand.copy_bytes=usize::from(owners.buffer.capacity()>=text.len()&&owners.buffer.len()<text.len());}
  match self.phase{0=>{if owners.output.as_ref().unwrap().neurons.capacity()<source.neurons.len(){demand.capacity_bytes=source.neurons.len().checked_mul(size_of::<Neuron>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original neuron allocation extent overflows"))?}},4=>{if source.neurons[self.node].tree.is_some(){demand.capacity_bytes=size_of::<Self>()}},5=>{let mut child=owners.child.as_ref().unwrap().next_demands(source.neurons[self.node].tree.as_deref().unwrap(),copy)?;child.depth=child.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original nested projection depth overflows"))?;return Ok(child)},6=>demand.capacity_bytes=size_of::<Tree>(),7=>{let child=owners.child.as_ref().unwrap();if child.terminal_is_empty(){demand.release_bytes=size_of::<Self>()}else{let mut child=child.next_close_demands(copy)?;child.depth=child.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original nested closure depth overflows"))?;return Ok(child)}},10=>{if owners.output.as_ref().unwrap().synapses.capacity()<source.synapses.len(){demand.capacity_bytes=source.synapses.len().checked_mul(size_of::<Synapse>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original synapse allocation extent overflows"))?}},_=>{}}Ok(demand)
 }
 /// ⏱️ Advances one original projection event and preserves actual accepted effects through errors.
 pub fn step(&mut self,source:&Tree,grant:RetainedCloneGrant)->Result<Option<Tree>,ValueError>{self.progress=Default::default();let result=self.advance(source,grant);result.map_err(|error|error.with_retained_progress(self.progress))}
 fn advance(&mut self,source:&Tree,grant:RetainedCloneGrant)->Result<Option<Tree>,ValueError>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original tree projection is closing"))}if grant.maximum_items==0{return Ok(None)}self.original(source)?;let demand=self.next_demands(source,grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original tree projection exceeds admitted depth"))}if !matches!(self.phase,5|7)&&(grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes){return Ok(None)}self.source=source as *const Tree as usize;
  if let Some(text)=self.text(source){let phase=self.phase;let owners=self.owners.as_mut().unwrap();self.progress.copied_items=1;if owners.buffer.capacity()<text.len(){owners.buffer.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tree string allocation refused"))?;self.progress.retained_capacity_bytes=owners.buffer.capacity();}else if owners.buffer.len()<text.len(){let offset=owners.buffer.len();let count=(text.len()-offset).min(grant.maximum_copy_bytes);owners.buffer.extend_from_slice(&text.as_bytes()[offset..offset+count]);self.progress.copied_bytes=count;}else{let text=unsafe{String::from_utf8_unchecked(std::mem::take(&mut owners.buffer))};match phase{2=>owners.neuron.as_mut().unwrap().id=text,3=>owners.neuron.as_mut().unwrap().kind=text,12=>owners.synapse.as_mut().unwrap().id=text,13=>owners.synapse.as_mut().unwrap().from=text,14=>owners.synapse.as_mut().unwrap().to=text,15=>owners.synapse.as_mut().unwrap().from_port=text,16=>owners.synapse.as_mut().unwrap().to_port=text,_=>unreachable!()}self.phase+=1;}return Ok(None)}
  let owners=self.owners.as_mut().unwrap();self.progress.copied_items=1;match self.phase{
   0=>{let output=owners.output.as_mut().unwrap();if output.neurons.capacity()<source.neurons.len(){output.neurons.try_reserve_exact(source.neurons.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original neuron vector allocation refused"))?;self.progress.retained_capacity_bytes=output.neurons.capacity()*size_of::<Neuron>();}self.phase=1;},
   1=>{if let Some(node)=source.neurons.get(self.node){owners.neuron=Some(Neuron{id:String::new(),kind:String::new(),params:node.params.clone(),tree:None});self.phase=2;}else{self.phase=10;}},
   4=>{if source.neurons[self.node].tree.is_some(){owners.child=Some(Box::new(Self::new()));self.progress.retained_capacity_bytes=size_of::<Self>();self.phase=5;}else{self.phase=9;}},
   5=>{let child=owners.child.as_mut().unwrap();let result=child.step(source.neurons[self.node].tree.as_deref().unwrap(),RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});self.progress=child.step_progress();if let Some(tree)=result?{owners.nested=Some(tree);self.phase=6;}},
   6=>{owners.neuron.as_mut().unwrap().tree=Some(Box::new(owners.nested.take().unwrap()));self.progress.retained_capacity_bytes=size_of::<Tree>();owners.child.as_mut().unwrap().begin_close();self.phase=7;},
   7=>{let child=owners.child.as_mut().unwrap();if child.terminal_is_empty(){if grant.maximum_release_bytes<size_of::<Self>(){self.progress=Default::default();return Ok(None)}drop(owners.child.take());self.progress.released_bytes=size_of::<Self>();self.phase=9;}else{let result=child.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});self.progress=child.step_progress();result?;}},
   9=>{owners.output.as_mut().unwrap().neurons.push(owners.neuron.take().unwrap());self.node+=1;self.phase=1;},
   10=>{let output=owners.output.as_mut().unwrap();if output.synapses.capacity()<source.synapses.len(){output.synapses.try_reserve_exact(source.synapses.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original synapse vector allocation refused"))?;self.progress.retained_capacity_bytes=output.synapses.capacity()*size_of::<Synapse>();}self.phase=11;},
   11=>{if source.synapses.get(self.edge).is_some(){owners.synapse=Some(Synapse{id:String::new(),from:String::new(),to:String::new(),from_port:String::new(),to_port:String::new()});self.phase=12;}else{self.phase=18;}},
   17=>{owners.output.as_mut().unwrap().synapses.push(owners.synapse.take().unwrap());self.edge+=1;self.phase=11;},
   18=>{self.phase=255;return Ok(owners.output.take())},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tree projection already handed off")),
  }Ok(None)
 }
 /// 🛑️ Signals the retained recursive child while keeping all accepted execution owners.
 pub fn begin_close(&mut self){self.closing=true;if let Some(child)=self.owners.as_mut().and_then(|owners|owners.child.as_mut()){child.begin_close();}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return Self::demand(owner,copy)}Ok(RetirementDemand{depth:usize::from(self.owners.is_some()),..Default::default()})}
 /// 🧹️ Hands off the exact typed projection fields before advancing their original closure.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.progress=Default::default();let result=self.close_source(grant);if let Ok(step)=&result{self.progress=step.progress()}result.map_err(|error|error.with_retained_progress(self.progress))}
 fn close_source(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tree projection close was not requested"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(owner)=self.retirement.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retirement=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original projection handoff requires admitted depth"))}self.retirement=Some(ControlledRetirement::new(self.owners.take().unwrap()).unwrap_or_else(|_|unreachable!("original projection source schema declares every field")));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.owners.is_none()&&self.retirement.is_none()}
}
impl Default for BudgetedTreeCopy{fn default()->Self{Self::new()}}
impl Drop for BudgetedTreeCopy{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original tree projection must finish explicit closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners)}}}}
struct CopyRetirement(BudgetedTreeCopy);
impl RetirementCursor for CopyRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.0.close_step(grant){Ok(step) if step.progress()!=RetainedCloneProgress::default()=>RetirementStep::Progress(step.progress()),Ok(RetainedCloneStep::Complete(_))=>RetirementStep::Complete,Ok(_)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for BudgetedTreeCopy{
 fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(CopyRetirement(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<CopyRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
