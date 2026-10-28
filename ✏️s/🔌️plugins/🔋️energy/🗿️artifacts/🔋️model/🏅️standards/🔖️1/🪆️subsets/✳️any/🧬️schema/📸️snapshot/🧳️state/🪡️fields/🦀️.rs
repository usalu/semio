//! 🪡️ Native authored fields become sealed immutable values through original retained preparation.
use protocol::mutation::retained::{RetainedFieldPreparation,retained_mutation_demand_fits};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedClone,RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{size_of,ManuallyDrop};

/// 🧳️ The actual authored source, prepared field and unpublished sealed value close independently after every prefix.
pub struct RetainedSealedField<T:RetainedClone+Send+Sync>{binding:Option<RetainedCloneBinding>,preparation:Option<Box<RetainedFieldPreparation<T>>>,raw:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<SealedShared<T>>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<T:RetainedClone+Send+Sync> Default for RetainedSealedField<T>{fn default()->Self{Self::new()}}
impl<T:RetainedClone+Send+Sync> RetainedSealedField<T>{
 pub fn new()->Self{Self{binding:None,preparation:None,raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==6&&self.output.is_some()}
 pub fn demand(&self,source:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=6{return Err(invalid());}source.check_binding(&self.binding)?;
  match self.phase{
   0=>nested(RetirementDemand{copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()},1),
   1=>Ok(RetirementDemand{copy_bytes:2*size_of::<RetainedFieldPreparation<T>>()+2*size_of::<Option<Box<RetainedFieldPreparation<T>>>>()+1,capacity_bytes:size_of::<RetainedFieldPreparation<T>>(),depth:1,..Default::default()}),
   2=>{let p=self.preparation.as_ref().ok_or_else(invalid)?;if p.value().is_some(){nested(p.exchange_demand()?,2*size_of::<Option<T>>()+1)}else{nested(p.demand(source,body.saturating_sub(1))?,1)}},
   3=>{let p=self.preparation.as_ref().ok_or_else(invalid)?;if p.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:1,depth:1,..Default::default()})}else{nested(p.close_demand(body.saturating_sub(1))?,1)}},
   4=>Ok(preparation_release::<T>()),
   5=>Ok(RetirementDemand{copy_bytes:2*size_of::<Option<T>>()+2*size_of::<Option<SealedShared<T>>>()+1,capacity_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()}),
   _=>Err(invalid()),
  }
 }
 pub fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demand(source,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut p=receipt(d);
  match self.phase{
   0=>{p=add(source.bind(&mut self.binding,child(g,1))?.ok_or_else(invalid)?,1)?;self.phase=1;},
   1=>{self.preparation=Some(Box::new(RetainedFieldPreparation::default()));self.phase=2;},
   2=>{let prepared=self.preparation.as_mut().ok_or_else(invalid)?;if prepared.value().is_some(){let extra=2*size_of::<Option<T>>()+1;let(value,progress)=prepared.take(child(g,extra)).map_err(|e|tag(e,extra))?;*self.raw=value;prepared.begin_close();self.phase=3;p=add(progress,extra)?;}else{p=add(prepared.advance(source,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;}},
   3=>{let prepared=self.preparation.as_mut().ok_or_else(invalid)?;if prepared.terminal_is_empty(){self.phase=4;}else{p=add(prepared.close_step(child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;}},
   4=>{drop(self.preparation.take());self.phase=5;},
   5=>{let value=self.raw.take().ok_or_else(invalid)?;match SealedShared::admit(value,SharedIssuer::owned(),g){Ok((value,progress))=>{*self.output=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes;self.phase=6;},Err((error,value))=>{*self.raw=Some(value);p.retained_capacity_bytes=error.retained_progress().retained_capacity_bytes;return Err(error.with_retained_progress(p));}}},
   _=>return Err(invalid()),
  }
  semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original Energy authored field sealing")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(RetirementDemand{copy_bytes:2*size_of::<Option<SealedShared<T>>>()+1,depth:1,..Default::default()})}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(SealedShared<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}let value=self.output.take().ok_or_else(invalid)?;self.phase=7;Ok(Some((value,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(p)=self.preparation.as_mut(){p.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}if let Some(p)=self.preparation.as_ref(){return if p.terminal_is_empty(){Ok(preparation_release::<T>())}else{p.close_demand(body)};}
  if !self.retirement.is_empty()||self.raw.is_some(){return self.retirement.next_owner_demands::<T>(self.raw.is_some(),&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body);}RetainedCloneBinding::close_demand(&self.binding,body)
 }
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(p)=self.preparation.as_mut(){if p.terminal_is_empty(){drop(self.preparation.take());return Ok(RetainedCloneStep::Progress(receipt(d)));}return p.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.raw,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.output,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}let s=RetainedCloneBinding::close_one(&mut self.binding,g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})
 }
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.preparation.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<T:RetainedClone+Send+Sync> Drop for RetainedSealedField<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original Energy authored field retains funded preparation and source custody");if empty{unsafe{ManuallyDrop::drop(&mut self.raw);ManuallyDrop::drop(&mut self.output);}}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Energy authored field lost its required source or phase")}
fn preparation_release<T:RetainedClone>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Box<RetainedFieldPreparation<T>>>>()+1,release_bytes:size_of::<RetainedFieldPreparation<T>>(),depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g}}
fn add(mut p:RetainedCloneProgress,extra:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(extra).ok_or_else(invalid)?;Ok(p)}
fn tag(error:ValueError,extra:usize)->ValueError{let p=error.retained_progress();match add(p,extra){Ok(p)=>error.with_retained_progress(p),Err(failure)=>failure.with_retained_progress(p)}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🌳️branch/🦀️.rs"]mod branch;
pub use branch::{RetainedBranchProjection,BranchProjectionOutput};

#[path="🔎️key/🦀️.rs"]mod entity_lookup;
pub use entity_lookup::{EntityRow,RetainedEntityLookup};
