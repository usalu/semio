//! 🌳️ Original immutable branch projection opens one owned candidate before separately funded header adoption and sealing.
use super::{invalid,nested,child,add,receipt,tag};
use protocol::mutation::retained::retained_mutation_demand_fits;
use semio_framework_value::{RetirementDemand,ValueError,retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_snapshot_projection::snapshot_projection::{SnapshotProjection,ProjectionField};
use std::mem::{size_of,ManuallyDrop};

/// 📦️ Projection output keeps the defining boxed or inline owner until its exact admitted handoff.
pub trait BranchProjectionOutput<B>:RetireOwned{fn release_bytes()->usize;fn into_branch(self)->B;}
impl<B:RetireOwned> BranchProjectionOutput<B> for Box<B>{fn release_bytes()->usize{size_of::<B>()}fn into_branch(self)->B{*self}}
impl<T:RetireOwned,const N:usize> BranchProjectionOutput<[T;N]> for [T;N]{fn release_bytes()->usize{0}fn into_branch(self)->[T;N]{self}}
/// 🧳️ Projected branch, unboxed candidate and immutable result preserve independent cancellation custody.
pub struct RetainedBranchProjection<B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send>{binding:Option<RetainedCloneBinding>,projection:ProjectionField<B>,boxed:ManuallyDrop<Option<B::Output>>,raw:ManuallyDrop<Option<B>>,output:ManuallyDrop<Option<SealedShared<B>>>,retirement:RetainedCloneClose,phase:u8,closing:bool}
impl<B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send> Default for RetainedBranchProjection<B>{fn default()->Self{Self::new()}}
impl<B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send> RetainedBranchProjection<B>{
 pub fn new()->Self{Self{binding:None,projection:Default::default(),boxed:ManuallyDrop::new(None),raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),phase:0,closing:false}}
 pub fn raw_ready(&self)->bool{!self.closing&&self.phase==4&&self.raw.is_some()}
 pub fn ready(&self)->bool{!self.closing&&self.phase==6&&self.output.is_some()}
 pub fn demand(&self,source:RetainedCloneRef<'_,B>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||matches!(self.phase,4|6|7){return Err(invalid());}source.check_binding(&self.binding)?;match self.phase{
   0=>nested(RetirementDemand{copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()},1),
   1=>nested(self.projection.demands(source,body.saturating_sub(1))?,1),
   2=>nested(self.projection.take_output_demand()?,2*size_of::<Option<B::Output>>()+1),
   3=>Ok(RetirementDemand{copy_bytes:2*size_of::<B>()+2*size_of::<Option<B::Output>>()+2*size_of::<Option<B>>()+1,release_bytes:<B::Output as BranchProjectionOutput<B>>::release_bytes(),depth:1,..Default::default()}),
   5=>Ok(RetirementDemand{copy_bytes:2*size_of::<Option<B>>()+2*size_of::<Option<SealedShared<B>>>()+1,capacity_bytes:SealedShared::<B>::birth_bytes(),depth:1,..Default::default()}),
   _=>Err(invalid()),
  }
 }
 pub fn advance(&mut self,source:RetainedCloneRef<'_,B>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demand(source,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut p=receipt(d);match self.phase{
   0=>{p=add(source.bind(&mut self.binding,child(g,1))?.ok_or_else(invalid)?,1)?;self.phase=1;},
   1=>{p=add(self.projection.advance(source,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if self.projection.ready(){self.phase=2;}},
   2=>{let extra=2*size_of::<Option<B::Output>>()+1;let(value,progress)=self.projection.take_output(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.boxed=Some(value);p=add(progress,extra)?;self.phase=3;},
   3=>{let value=self.boxed.take().ok_or_else(invalid)?;*self.raw=Some(value.into_branch());self.phase=4;},
   5=>{let value=self.raw.take().ok_or_else(invalid)?;match SealedShared::admit(value,SharedIssuer::owned(),g){Ok((value,progress))=>{*self.output=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes;self.phase=6;},Err((error,value))=>{*self.raw=Some(value);p.retained_capacity_bytes=error.retained_progress().retained_capacity_bytes;return Err(error.with_retained_progress(p));}}},
   _=>return Err(invalid()),
  }semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original Energy branch projection and sealing")?;Ok(if self.raw_ready()||self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn exchange_demand<F>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<F>()+2*size_of::<Option<F>>()+1,depth:1,..Default::default()}}
 pub fn exchange<F:RetireOwned>(&mut self,select:for<'a>fn(&'a mut B)->&'a mut F,value:&mut Option<F>,displaced:&mut Option<F>,g:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{if !self.raw_ready()||value.is_none()||displaced.is_some(){return Err(invalid());}let d=Self::exchange_demand::<F>();if !retained_mutation_demand_fits(d,g){return Ok((false,Default::default()));}let value=value.take().ok_or_else(invalid)?;*displaced=Some(std::mem::replace(select(self.raw.as_mut().ok_or_else(invalid)?),value));Ok((true,receipt(d)))}
 pub fn exchange_with<F:RetireOwned>(&mut self,replace:fn(&mut B,F)->F,value:&mut Option<F>,displaced:&mut Option<F>,g:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{if !self.raw_ready()||value.is_none()||displaced.is_some(){return Err(invalid());}let d=Self::exchange_demand::<F>();if !retained_mutation_demand_fits(d,g){return Ok((false,Default::default()));}let value=value.take().ok_or_else(invalid)?;*displaced=Some(replace(self.raw.as_mut().ok_or_else(invalid)?,value));Ok((true,receipt(d)))}
 pub fn begin_seal(&mut self)->Result<(),ValueError>{if !self.raw_ready(){return Err(invalid());}self.phase=5;Ok(())}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(RetirementDemand{copy_bytes:2*size_of::<Option<SealedShared<B>>>()+1,depth:1,..Default::default()})}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(SealedShared<B>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}let value=self.output.take().ok_or_else(invalid)?;self.phase=7;Ok(Some((value,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;self.projection.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if !self.projection.terminal_is_empty(){return self.projection.close_demands(body);}if !self.retirement.is_empty()||self.boxed.is_some(){return self.retirement.next_owner_demands::<B::Output>(self.boxed.is_some(),&None,body);}if self.raw.is_some(){return self.retirement.next_owner_demands::<B>(true,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<B>>(true,&None,body);}RetainedCloneBinding::close_demand(&self.binding,body)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.projection.terminal_is_empty(){return self.projection.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.retirement.is_empty(){return self.retirement.step_granted(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.boxed,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.raw,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.output,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}let s=RetainedCloneBinding::close_one(&mut self.binding,g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.projection.owners_empty()&&self.boxed.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send> Drop for RetainedBranchProjection<B>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original Energy branch retains funded projection and source custody");if empty{unsafe{ManuallyDrop::drop(&mut self.boxed);ManuallyDrop::drop(&mut self.raw);ManuallyDrop::drop(&mut self.output);}}}}
