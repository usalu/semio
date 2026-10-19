//! 🪢️ Ordered sparse children settle on genuine unpublished originals before root publication.
use super::{RetainedDiffFragment,RetainedDiffOwner,RetainedDiffPreflightAdvance,RetainedDiffPreflightStep,retained_diff_birth_demand,begin_retained_diff,preflight_retained_diff,commit_retained_diff};
use super::super::{binding_demand,retained_mutation_demand_fits};
use super::super::super::{ApplyCapability,MutationMessage};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, snapshot_projection::{SnapshotProjection, ProjectionField}, retained_clone::{RetainedCloneRef, RetainedCloneBinding, RetainedCloneSource, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value_persistent::original_vector_concat::OriginalVectorConcat;
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"ordered retained diff lost its original sequence frontier")}
/// 🧳️ One sparse edit stays direct; longer sequences own every typed intermediate and diagnostic.
pub struct RetainedDiffSequence<P:SnapshotProjection+Send+'static,D:RetainedDiffFragment<P>>{
 child:ManuallyDrop<Option<RetainedDiffOwner<P,D>>>,projection:ProjectionField<P>,
 candidate:ManuallyDrop<Option<P>>,intermediate:ManuallyDrop<Option<RetainedCloneSource<P>>>,
 pending_messages:ManuallyDrop<Option<Vec<MutationMessage>>>,output_messages:ManuallyDrop<Option<Vec<MutationMessage>>>,
 messages:OriginalVectorConcat<MutationMessage>,message_retirement:RetainedCloneClose,candidate_retirement:RetainedCloneClose,
 diff:Option<RetainedCloneBinding>,original:Option<RetainedCloneBinding>,ordinal:usize,phase:u8,single:bool,admitted:bool,closing:bool,
}
impl<P:SnapshotProjection+Send+'static,D:RetainedDiffFragment<P>> Default for RetainedDiffSequence<P,D>{fn default()->Self{Self::new()}}
impl<P:SnapshotProjection+Send+'static,D:RetainedDiffFragment<P>> RetainedDiffSequence<P,D>{
 pub fn new()->Self{Self{child:ManuallyDrop::new(None),projection:Default::default(),candidate:ManuallyDrop::new(None),intermediate:ManuallyDrop::new(None),pending_messages:ManuallyDrop::new(None),output_messages:ManuallyDrop::new(None),messages:Default::default(),message_retirement:Default::default(),candidate_retirement:Default::default(),diff:None,original:None,ordinal:0,phase:0,single:false,admitted:true,closing:false}}
 fn fixed()->usize{size_of::<usize>()*10+size_of::<bool>()*3+size_of::<u8>()*2+size_of::<Vec<MutationMessage>>()*2}
 fn scalar()->RetirementDemand{RetirementDemand{copy_bytes:Self::fixed(),depth:1,..Default::default()}}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(Self::fixed()).ok_or_else(invalid)?;demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn child_grant(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed(),maximum_depth:grant.maximum_depth-1,..grant}}
 fn receipt(mut progress:RetainedCloneProgress)->RetainedCloneProgress{progress.copied_bytes+=Self::fixed();progress}
 fn current<'a>(intermediate:&'a Option<RetainedCloneSource<P>>,original:RetainedCloneRef<'a,P>)->RetainedCloneRef<'a,P>{intermediate.as_ref().map_or(original,RetainedCloneSource::borrow)}
 fn sources(&self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>)->Result<(),ValueError>{if self.diff.is_some()&&!diffs.check_binding(&self.diff)?||self.original.is_some()&&!original.check_binding(&self.original)?{return Err(invalid());}Ok(())}
 /// ⚖️ Root preflight never changes the externally supplied unpublished candidate.
 pub fn preflight_demand(&self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>17{return Err(invalid());}self.sources(diffs,original)?;
  let demand=match self.phase{
   0=>RetirementDemand{copy_bytes:diffs.binding_copy_bytes(),depth:1,..Default::default()},
   1=>RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()},
   2|12|17=>Default::default(),
   3=>self.projection.demands(Self::current(&self.intermediate,original),body)?,
   4=>self.projection.take_demand()?,
   5=>retained_diff_birth_demand::<P,D>(),
   6=>{let source=Self::current(&self.intermediate,original);let candidate=if self.single{post}else{self.candidate.as_ref().ok_or_else(invalid)?};self.child.as_ref().ok_or_else(invalid)?.preflight_demand(diffs.project(self.ordinal,|rows|&rows[self.ordinal]),source,candidate,body)?},
   7=>self.messages.push_demand()?,
   8=>self.child.as_ref().ok_or_else(invalid)?.commit_demand(diffs.project(self.ordinal,|rows|&rows[self.ordinal]),Self::current(&self.intermediate,original),self.candidate.as_ref().ok_or_else(invalid)?,body)?,
   9=>if self.child.as_ref().ok_or_else(invalid)?.terminal_is_empty(){Default::default()}else{self.child.as_ref().unwrap().close_demand(body)?},
   10=>if let Some(source)=self.intermediate.as_ref(){if source.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<RetainedCloneSource<P>>(),depth:1,..Default::default()}}else{source.next_close_demand(body)?}}else{Default::default()},
   11=>{let birth=RetainedCloneSource::<P>::owned_constructor_demand::<()>();RetirementDemand{copy_bytes:RetainedCloneSource::<P>::constructor_copy_bytes()+size_of::<P>()*2,capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()}},
   14=>self.messages.finish_demand()?,
   15=>self.messages.demand(body)?,
   16=>self.messages.take_demand()?,
   _=>return Err(invalid()),
  };Self::nested(demand)
 }
 pub fn preflight(&mut self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:&P,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<RetainedDiffPreflightAdvance,ValueError>{
  let demand=self.preflight_demand(diffs,original,post,body)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}
  let child_grant=Self::child_grant(grant);let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  match self.phase{
   0=>{progress=Self::receipt(diffs.bind(&mut self.diff,child_grant)?.ok_or_else(invalid)?);self.phase=1;},
   1=>{progress=Self::receipt(original.bind(&mut self.original,child_grant)?.ok_or_else(invalid)?);self.phase=2;},
   2=>{self.single=diffs.get().len()==1;self.phase=if diffs.get().is_empty(){14}else if self.single{5}else{3};},
   3=>{let step=self.projection.advance(Self::current(&self.intermediate,original),body,child_grant)?;progress=Self::receipt(step.progress());if self.projection.ready(){self.phase=4;}},
   4=>{let(candidate,receipt)=self.projection.take(child_grant)?.ok_or_else(invalid)?;*self.candidate=Some(candidate);self.projection.begin_close();progress=Self::receipt(receipt);self.phase=5;},
   5=>{let(child,receipt)=begin_retained_diff::<P,D>(child_grant).ok_or_else(invalid)?;*self.child=Some(child);progress=Self::receipt(receipt);self.phase=6;},
   6=>{
    let source=Self::current(&self.intermediate,original);let candidate=if self.single{post}else{self.candidate.as_ref().ok_or_else(invalid)?};
    let step=preflight_retained_diff(self.child.as_mut().ok_or_else(invalid)?,diffs.project(self.ordinal,|rows|&rows[self.ordinal]),source,candidate,body,child_grant,receiving).map_err(|error|{let receipt=error.retained_progress();error.with_retained_progress(if receipt==RetainedCloneProgress::default(){receipt}else{Self::receipt(receipt)})})?;progress=Self::receipt(step.progress());
    match step{RetainedDiffPreflightStep::Admitted(_,messages)=>{*self.pending_messages=Some(messages);self.phase=7;},RetainedDiffPreflightStep::Refused(_,messages)=>{*self.pending_messages=Some(messages);self.admitted=false;self.phase=7;},RetainedDiffPreflightStep::Progress(_)=>{}}
   },
   7=>{let(pushed,receipt)=self.messages.push(self.pending_messages.as_mut().ok_or_else(invalid)?,child_grant)?;progress=Self::receipt(receipt);if pushed{self.pending_messages.take();self.phase=if !self.admitted||self.single{14}else{8};}},
   8=>{let source=Self::current(&self.intermediate,original);let step=commit_retained_diff(self.child.as_mut().ok_or_else(invalid)?,diffs.project(self.ordinal,|rows|&rows[self.ordinal]),source,self.candidate.as_mut().ok_or_else(invalid)?,child_grant)?;progress=Self::receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.child.as_mut().unwrap().begin_close();self.phase=9;}},
   9=>{let child=self.child.as_mut().ok_or_else(invalid)?;if !child.terminal_is_empty(){progress=Self::receipt(child.close_step(child_grant)?.progress());}else{self.child.take();self.phase=if self.ordinal+1==diffs.get().len(){14}else{10};}},
   10=>{if let Some(source)=self.intermediate.as_mut(){if !source.terminal_is_empty(){progress=Self::receipt(source.close_step(child_grant)?.progress());}else{self.intermediate.take();self.phase=11;}}else{self.phase=11;}},
   11=>{let candidate=self.candidate.take().ok_or_else(invalid)?;match RetainedCloneSource::admit_owned(candidate,(),RetainedCloneGrant{maximum_copy_bytes:child_grant.maximum_copy_bytes-size_of::<P>()*2,..child_grant}){Ok((source,receipt))=>{*self.intermediate=Some(source);progress=Self::receipt(receipt);progress.copied_bytes+=size_of::<P>()*2;self.phase=12;},Err((error,candidate,_))=>{*self.candidate=Some(candidate);return Err(error);}}},
   12=>{self.ordinal+=1;self.projection=Default::default();self.phase=3;},
   14=>{progress=Self::receipt(self.messages.finish(child_grant)?.progress());self.phase=15;},
   15=>{let step=self.messages.advance(child_grant)?;progress=Self::receipt(step.progress());if self.messages.ready(){self.phase=16;}},
   16=>{let(messages,receipt)=self.messages.take(child_grant)?.ok_or_else(invalid)?;*self.output_messages=Some(messages);progress=Self::receipt(receipt);self.phase=17;return Ok(RetainedDiffPreflightAdvance::Ready(progress,self.admitted));},
   17=>return Ok(RetainedDiffPreflightAdvance::Ready(progress,self.admitted)),
   _=>return Err(invalid()),
  }Ok(RetainedDiffPreflightAdvance::Progress(progress))
 }
 /// 📨️ The already funded joined original diagnostics transfer without copying payloads.
 pub fn take_messages(&mut self)->Vec<MutationMessage>{assert!(self.phase==17,"ordered original diagnostics require settled preflight");self.output_messages.take().expect("ordered original diagnostic transfer occurs once")}
 pub fn commit_demand(&self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=17||!self.admitted{return Err(invalid());}self.sources(diffs,original)?;
  if self.single{return Self::nested(self.child.as_ref().ok_or_else(invalid)?.commit_demand(diffs.project(0,|rows|&rows[0]),original,post,body.saturating_sub(Self::fixed()))?);}
  Ok(RetirementDemand{copy_bytes:Self::fixed()+if self.candidate.is_some(){size_of::<P>()*3}else{0},depth:1,..Default::default()})
 }
 /// 🔑️ Publication uses only the outer central capability after complete ordered preflight.
 pub fn commit(&mut self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:&mut P,grant:RetainedCloneGrant,_capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{
  let demand=self.commit_demand(diffs,original,post,grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.single{let step=commit_retained_diff(self.child.as_mut().ok_or_else(invalid)?,diffs.project(0,|rows|&rows[0]),original,post,Self::child_grant(grant))?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=18;}return Ok(match step{RetainedCloneStep::Progress(p)=>RetainedCloneStep::Progress(Self::receipt(p)),RetainedCloneStep::Complete(p)=>RetainedCloneStep::Complete(Self::receipt(p))});}
  if let Some(candidate)=self.candidate.as_mut(){std::mem::swap(candidate,post);}self.phase=18;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(child)=self.child.as_mut(){child.begin_close();}self.projection.begin_close();self.messages.begin_close();}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}let body=body.saturating_sub(Self::fixed());
  let demand=if let Some(child)=self.child.as_ref(){if child.terminal_is_empty(){Default::default()}else{child.close_demand(body)?}}
  else if !self.projection.terminal_is_empty(){self.projection.close_demands(body)?}
  else if !self.messages.terminal_is_empty(){self.messages.close_demand(body)?}
  else if !self.message_retirement.is_empty()||self.pending_messages.is_some(){self.message_retirement.next_owner_demands::<Vec<MutationMessage>>(self.pending_messages.is_some(),&None,body)?}
  else if self.output_messages.is_some(){self.message_retirement.next_owner_demands::<Vec<MutationMessage>>(true,&None,body)?}
  else if let Some(source)=self.intermediate.as_ref(){if source.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<RetainedCloneSource<P>>(),depth:1,..Default::default()}}else{source.next_close_demand(body)?}}
  else if self.candidate.is_some()||!self.candidate_retirement.is_empty(){self.candidate_retirement.next_owner_demands::<P>(self.candidate.is_some(),&None,body)?}
  else if self.original.is_some(){binding_demand(&self.original,body)?}
  else if self.diff.is_some(){binding_demand(&self.diff,body)?}
  else{return Ok(Default::default());};Self::nested(demand)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid());}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}let child_grant=Self::child_grant(grant);
  if let Some(child)=self.child.as_mut(){if !child.terminal_is_empty(){return child.close_step(child_grant).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}self.child.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  let step=if !self.projection.terminal_is_empty(){self.projection.close_step(child_grant)?}
  else if !self.messages.terminal_is_empty(){self.messages.close_step(child_grant)?}
  else if !self.message_retirement.is_empty(){self.message_retirement.step_granted(child_grant)?}
  else if let Some(step)=self.message_retirement.begin_granted(&mut self.pending_messages,child_grant)?{step}
  else if let Some(step)=self.message_retirement.begin_granted(&mut self.output_messages,child_grant)?{step}
  else if let Some(source)=self.intermediate.as_mut(){if !source.terminal_is_empty(){source.close_step(child_grant)?}else{self.intermediate.take();RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<RetainedCloneSource<P>>(),..Default::default()})}}
  else if !self.candidate_retirement.is_empty(){self.candidate_retirement.step_granted(child_grant)?}
  else if let Some(step)=self.candidate_retirement.begin_granted(&mut self.candidate,child_grant)?{step}
  else if self.original.is_some(){RetainedCloneBinding::close_one(&mut self.original,child_grant)?}
  else{RetainedCloneBinding::close_one(&mut self.diff,child_grant)?};
  Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())))
 }
 fn empty(&self)->bool{self.child.is_none()&&self.candidate.is_none()&&self.intermediate.is_none()&&self.pending_messages.is_none()&&self.output_messages.is_none()&&self.projection.owners_empty()&&self.messages.terminal_is_empty()&&self.message_retirement.is_empty()&&self.candidate_retirement.is_empty()&&self.original.is_none()&&self.diff.is_none()}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.empty()}
}
impl<P:SnapshotProjection+Send+'static,D:RetainedDiffFragment<P>> Drop for RetainedDiffSequence<P,D>{fn drop(&mut self){let empty=self.empty();assert!(std::thread::panicking()||empty,"ordered retained diff requires paid child, source, diagnostic and candidate closure");if empty{unsafe{ManuallyDrop::drop(&mut self.child);ManuallyDrop::drop(&mut self.candidate);ManuallyDrop::drop(&mut self.intermediate);ManuallyDrop::drop(&mut self.pending_messages);ManuallyDrop::drop(&mut self.output_messages);}}}}
