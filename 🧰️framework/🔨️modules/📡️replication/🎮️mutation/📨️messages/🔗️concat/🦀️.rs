//! 🔗️ Original diagnostic batches join in authored order without copying their strings.
use crate::{MutationMessage, MutationMessageRetirement};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::{ManuallyDrop, size_of};

/// 🧺️ Original backing, pending rows and joined output retain explicit cancellation custody.
pub struct MutationMessageConcat<const N:usize> {
 inputs:ManuallyDrop<[Option<std::vec::IntoIter<MutationMessage>>;N]>,
 capacities:[usize;N],
 output:ManuallyDrop<Option<Vec<MutationMessage>>>,
 pending:ManuallyDrop<Option<MutationMessage>>,
 active:Option<MutationMessageRetirement>,
 total:usize,
 index:usize,
 allocated:bool,
 closing:bool,
 taken:bool,
}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
impl<const N:usize> MutationMessageConcat<N>{
 /// 📦️ Captures only fixed original vector headers.
 pub fn constructor_demand()->RetirementDemand{scalar(size_of::<Self>()+size_of::<[Vec<MutationMessage>;N]>())}
 /// 🌱️ Unfunded admission returns every original batch untouched.
 pub fn admit(batches:[Vec<MutationMessage>;N],grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,[Vec<MutationMessage>;N])>{
  let d=Self::constructor_demand();if N>16||!fits(d,grant){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original diagnostic batch capture requires its fixed grant"),batches));}
  let Some(total)=batches.iter().try_fold(0usize,|n,b|n.checked_add(b.len()))else{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"joined diagnostic count overflow"),batches));};
  let capacities=std::array::from_fn(|i|batches[i].capacity()*size_of::<MutationMessage>());
  Ok((Self{inputs:ManuallyDrop::new(batches.map(|b|Some(b.into_iter()))),capacities,output:ManuallyDrop::new(None),pending:ManuallyDrop::new(None),active:None,total,index:0,allocated:false,closing:false,taken:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 /// 📏️ Each turn quotes one backing birth, original row transfer or empty backing release.
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.taken{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original diagnostic join is closed"));}
  if !self.allocated{return Ok(RetirementDemand{capacity_bytes:self.total.checked_mul(size_of::<MutationMessage>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"joined diagnostic layout overflow"))?,..scalar(size_of::<Vec<MutationMessage>>())});}
  if self.pending.is_some(){return Ok(scalar(size_of::<MutationMessage>()*2+size_of::<Option<u32>>()));}
  if self.index<N {if self.inputs[self.index].as_ref().is_some_and(|i|i.len()>0){return Ok(scalar(size_of::<MutationMessage>()*2));}return Ok(RetirementDemand{release_bytes:self.capacities[self.index],..scalar(size_of::<Option<std::vec::IntoIter<MutationMessage>>>()+size_of::<usize>())});}
  Ok(scalar(size_of::<Vec<MutationMessage>>()*2+size_of::<bool>()))
 }
 /// 🪜️ Original strings and target vectors move once into their ordered publication rows.
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demand()?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if !self.allocated{*self.output=Some(Vec::with_capacity(self.total));self.allocated=true;p.retained_capacity_bytes=self.output.as_ref().unwrap().capacity()*size_of::<MutationMessage>();if p.retained_capacity_bytes>grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"joined backing exceeded admitted capacity").with_retained_progress(p));}return Ok(RetainedCloneStep::Progress(p));}
  if let Some(mut row)=self.pending.take(){if row.op_index.is_none(){row.op_index=Some(0);}self.output.as_mut().unwrap().push(row);return Ok(RetainedCloneStep::Progress(p));}
  if self.index<N {if let Some(row)=self.inputs[self.index].as_mut().unwrap().next(){*self.pending=Some(row);}else{drop(self.inputs[self.index].take());p.released_bytes=d.release_bytes;self.index+=1;}return Ok(RetainedCloneStep::Progress(p));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 /// 📤️ Transfers the original joined vector under its exact header quote.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<MutationMessage>,RetainedCloneProgress)>,ValueError>{
  if self.closing||self.taken||!self.allocated||self.index<N||self.pending.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original diagnostic join is not ready"));}let d=self.demand()?;if !fits(d,grant){return Ok(None);}self.taken=true;Ok(Some((self.output.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 }
 /// 🛑️ Cancellation keeps all original batches and partial output under typed close.
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Quotes exactly one active row or its original empty backing.
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{
  if let Some(a)=self.active.as_ref(){return Ok(RetirementDemand{copy_bytes:a.next_copy_byte_demand(),capacity_bytes:a.next_capacity_byte_demand(0)?,release_bytes:a.next_release_byte_demand(),depth:a.next_depth_demand()?});}
  if self.pending.is_some()||self.output.as_ref().is_some_and(|v|!v.is_empty()){return Ok(scalar(size_of::<MutationMessage>()+size_of::<MutationMessageRetirement>()));}
  if let Some(v)=self.output.as_ref(){return Ok(RetirementDemand{release_bytes:v.capacity()*size_of::<MutationMessage>(),..scalar(size_of::<Option<Vec<MutationMessage>>>() )});}
  for (i,input) in self.inputs.iter().enumerate(){if let Some(input)=input{return Ok(if input.len()>0{scalar(size_of::<MutationMessage>()+size_of::<MutationMessageRetirement>())}else{RetirementDemand{release_bytes:self.capacities[i],..scalar(size_of::<Option<std::vec::IntoIter<MutationMessage>>>() )}});}}
  Ok(Default::default())
 }
 /// 🪦️ Typed row retirement precedes each paid original backing release.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original diagnostic join closure not started"));}let d=self.close_demand()?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(a)=self.active.as_mut(){let p=a.close_step(grant);if a.terminal_is_empty(){self.active=None;}return Ok(RetainedCloneStep::Progress(p));}
  let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()};
  if let Some(row)=self.pending.take().or_else(||self.output.as_mut().and_then(Vec::pop)){self.active=Some(MutationMessageRetirement::new(row));return Ok(RetainedCloneStep::Progress(p));}
  if self.output.is_some(){drop(self.output.take());return Ok(RetainedCloneStep::Progress(p));}
  for input in self.inputs.iter_mut(){if let Some(rows)=input{if let Some(row)=rows.next(){self.active=Some(MutationMessageRetirement::new(row));}else{drop(input.take());}return Ok(RetainedCloneStep::Progress(p));}}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 /// 🫙️ No batch backing, joined vector, original row or retirement child remains.
 pub fn terminal_is_empty(&self)->bool{self.inputs.iter().all(Option::is_none)&&self.output.is_none()&&self.pending.is_none()&&self.active.is_none()}
}
impl<const N:usize> Drop for MutationMessageConcat<N>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original diagnostic join requires funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.inputs);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.pending);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"] mod tests;
