//! 🪢️ Ordered sparse children settle on genuine unpublished originals before root publication.
use super::{RetainedDiffFragment,RetainedDiffOwner,RetainedDiffPreflightAdvance,RetainedDiffPreflightStep,retained_diff_birth_demand,begin_retained_diff,preflight_retained_diff_into,recover_preflight_retained_diff_into,commit_retained_diff};
use super::super::{binding_demand,conclude,native_child,retained_mutation_demand_fits,settle,Done,Flight,Reply};
use super::super::super::{ApplyCapability,MutationMessage};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetirementDemand, RetirementPollExit, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneRef, RetainedCloneBinding, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value_snapshot_projection::snapshot_projection::{SnapshotProjection, ProjectionField, ProjectionOutput, post::{OriginalPostRead,PagedOneItemPostMode}};
use semio_framework_value_persistent::original_vector_concat::OriginalVectorConcat;
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"ordered retained diff lost its original sequence frontier")}
/// 🧳️ One sparse edit stays direct; longer sequences own every typed intermediate and diagnostic.
pub struct RetainedDiffSequence<P:SnapshotProjection+Send+'static,D:RetainedDiffFragment<P>>{
 child:ManuallyDrop<Option<RetainedDiffOwner<P,D>>>,projection:ProjectionField<P>,
  candidate:ManuallyDrop<Option<P::Output>>,candidate_read:OriginalPostRead<P::Output>,previous:OriginalPostRead<P::Output>,
 pending_messages:ManuallyDrop<Option<Vec<MutationMessage>>>,output_messages:ManuallyDrop<Option<Vec<MutationMessage>>>,
 messages:OriginalVectorConcat<MutationMessage>,message_retirement:RetainedCloneClose,candidate_retirement:RetainedCloneClose,
 flight:Flight,diff:Option<RetainedCloneBinding>,original:Option<RetainedCloneBinding>,ordinal:usize,phase:u8,single:bool,admitted:bool,closing:bool,
}
impl<P:SnapshotProjection+Send+'static,D:RetainedDiffFragment<P>> Default for RetainedDiffSequence<P,D>{fn default()->Self{Self::new()}}
impl<P:SnapshotProjection+Send+'static,D:RetainedDiffFragment<P>> RetainedDiffSequence<P,D>{
  pub fn new()->Self{Self{child:ManuallyDrop::new(None),projection:Default::default(),candidate:ManuallyDrop::new(None),candidate_read:Default::default(),previous:Default::default(),pending_messages:ManuallyDrop::new(None),output_messages:ManuallyDrop::new(None),messages:Default::default(),message_retirement:Default::default(),candidate_retirement:Default::default(),flight:Default::default(),diff:None,original:None,ordinal:0,phase:0,single:false,admitted:true,closing:false}}
 fn fixed()->usize{size_of::<usize>()*10+size_of::<bool>()*3+size_of::<u8>()*2+size_of::<Vec<MutationMessage>>()*2+size_of::<OriginalPostRead<P::Output>>()*2}
 fn scalar()->RetirementDemand{RetirementDemand{copy_bytes:Self::fixed(),depth:1,..Default::default()}}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(Self::fixed()).ok_or_else(invalid)?;demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn child_grant(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed(),maximum_depth:grant.maximum_depth-1,..grant}}
 fn receipt(mut progress:RetainedCloneProgress)->RetainedCloneProgress{progress.copied_bytes+=Self::fixed();progress}
  fn current<'a>(previous:&'a OriginalPostRead<P::Output>,original:RetainedCloneRef<'a,P>)->Result<RetainedCloneRef<'a,P>,ValueError>{if previous.terminal_is_empty(){return Ok(original)}Ok(previous.read(PagedOneItemPostMode::Original)?.original()?.project(0,ProjectionOutput::projected_value))}
 fn sources(&self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>)->Result<(),ValueError>{if self.diff.is_some()&&!diffs.check_binding(&self.diff)?||self.original.is_some()&&!original.check_binding(&self.original)?{return Err(invalid());}Ok(())}
 /// ⚖️ Root preflight never changes the externally supplied unpublished candidate.
 pub fn preflight_demand(&self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>17{return Err(invalid());}self.sources(diffs,original)?;
   if !self.single&&matches!(self.phase,6|8){if let Some(demand)=self.candidate_read.demand(if self.phase==6{PagedOneItemPostMode::Original}else{PagedOneItemPostMode::Mutable},body)?{return Self::nested(demand);}}
   if self.phase==3&&!self.previous.terminal_is_empty(){if let Some(demand)=self.previous.demand(PagedOneItemPostMode::Original,body)?{return Self::nested(demand);}}
  let demand=match self.phase{
   0=>RetirementDemand{copy_bytes:diffs.binding_copy_bytes(),depth:1,..Default::default()},
   1=>RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()},
   2|11|12|17=>Default::default(),
    3=>self.projection.demands(Self::current(&self.previous,original)?,body,maximum_work_units)?,
    4=>if self.candidate.is_none(){self.projection.take_output_demand()?}else{OriginalPostRead::<P::Output>::capture_demand()},
    13=>if !self.projection.close_started(){self.projection.begin_close_demand()?}else{self.projection.close_demands(body)?},
   5=>retained_diff_birth_demand::<P,D>(),
    6=>{let source=Self::current(&self.previous,original)?;let candidate=if self.single{post}else{self.candidate_read.read(PagedOneItemPostMode::Original)?.original()?.project(0,ProjectionOutput::projected_value)};self.child.as_ref().ok_or_else(invalid)?.preflight_demand(diffs.project(self.ordinal,|rows|&rows[self.ordinal]),source,candidate,body,maximum_work_units)?},
   7=>self.messages.push_demand()?,
    8=>self.child.as_ref().ok_or_else(invalid)?.commit_demand(diffs.project(self.ordinal,|rows|&rows[self.ordinal]),Self::current(&self.previous,original)?,self.candidate_read.read(PagedOneItemPostMode::Mutable)?.get().projected_value(),body)?,
   9=>if self.child.as_ref().ok_or_else(invalid)?.terminal_is_empty(){Default::default()}else{self.child.as_ref().ok_or_else(invalid)?.close_demand(body)?},
    10=>if !self.previous.terminal_is_empty(){match self.previous.demand(PagedOneItemPostMode::Mutable,body)?{Some(demand)=>demand,None=>self.previous.return_demand()}}else if self.candidate.is_some()||!self.candidate_retirement.is_empty(){self.candidate_retirement.next_owner_demands::<P::Output>(self.candidate.is_some(),&None,body)?}else{Default::default()},
   14=>self.messages.finish_demand()?,
   15=>self.messages.demand(body)?,
   16=>self.messages.take_demand()?,
   _=>return Err(invalid()),
  };Self::nested(demand)
 }
  pub fn preflight_into<'r>(&mut self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{self.preflight_reply(diffs,original,post,body,maximum_work_units,effect,receiving,reply,false)}
  /// 🔁️ Recovers the same pending ordered preflight child action.
  pub fn recover_preflight_into<'r>(&mut self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{self.preflight_reply(diffs,original,post,body,maximum_work_units,effect,receiving,reply,true)}
  fn preflight_reply<'r>(&mut self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:RetainedCloneRef<'_,P>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedDiffPreflightAdvance,RetirementPollExit>=(||{
   macro_rules! turn {($value:expr)=>{{let turn=$value;work+=turn.work;turn.result.map_err(|error|{let error:OriginalRetirementFailure=error.into();if turn.work==0{error}else{let progress=Self::receipt(error.retained_progress());error.with_retained_progress(progress)}})?}};}
   macro_rules! native {($call:expr)=>{{let before=work;native_child(effect,&mut work,$call)?.map_err(|error|{let error:OriginalRetirementFailure=error.into();if work==before{error}else{let progress=Self::receipt(error.retained_progress());error.with_retained_progress(progress)}})?}};}
   macro_rules! routed {($route:expr,|$r:ident| $fresh:expr,$again:expr)=>{{let before=work;retained_routed!(self.flight,effect,recover,$route,body,maximum_work_units,None,work,|$r| $fresh,$again).map_err(|error|{let error:OriginalRetirementFailure=error.into();if work==before{error}else{let progress=Self::receipt(error.retained_progress());error.with_retained_progress(progress)}})?}};}
   let hint=if recover{self.flight.route()}else{None};
   if !recover&&!self.flight.idle(){return Err(invalid().into())}
   let demand=if recover{Default::default()}else{let demand=self.preflight_demand(diffs,original,post,body,maximum_work_units)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}demand};
   let child_grant=Self::child_grant(grant);let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
   let mode=if self.phase==6{PagedOneItemPostMode::Original}else{PagedOneItemPostMode::Mutable};
   if hint==Some(20)||(!recover&&!self.single&&matches!(self.phase,6|8)&&self.candidate_read.demand(mode,body)?.is_some()){let step=routed!(20,|r| self.candidate_read.prepare_into(mode,body,&mut effect.nested(child_grant),r),self.candidate_read.recover_prepare_into(mode,body,&mut effect.nested(child_grant),r));return Ok(RetainedDiffPreflightAdvance::Progress(Self::receipt(step.progress())));}
   if hint==Some(21)||(!recover&&self.phase==3&&!self.previous.terminal_is_empty()&&self.previous.demand(PagedOneItemPostMode::Original,body)?.is_some()){let step=routed!(21,|r| self.previous.prepare_into(PagedOneItemPostMode::Original,body,&mut effect.nested(child_grant),r),self.previous.recover_prepare_into(PagedOneItemPostMode::Original,body,&mut effect.nested(child_grant),r));return Ok(RetainedDiffPreflightAdvance::Progress(Self::receipt(step.progress())));}
   match self.phase{
    0=>{progress=Self::receipt(turn!(diffs.bind(&mut self.diff,&mut effect.nested(child_grant))).ok_or_else(invalid)?);self.phase=1;},
    1=>{progress=Self::receipt(turn!(original.bind(&mut self.original,&mut effect.nested(child_grant))).ok_or_else(invalid)?);self.phase=2;},
    2=>{work+=1;self.single=diffs.get().len()==1;self.phase=if diffs.get().is_empty(){14}else if self.single{5}else{3};},
    3=>{let step=routed!(3,|r| self.projection.advance_into(Self::current(&self.previous,original)?,body,maximum_work_units,&mut effect.nested(child_grant),r),self.projection.recover_advance_into(Self::current(&self.previous,original)?,body,maximum_work_units,&mut effect.nested(child_grant),r));progress=Self::receipt(step.progress());if self.projection.ready(){self.phase=4;}},
    4=>{if self.candidate.is_none(){let step=native!(|e,r|self.projection.take_output_into(&mut *self.candidate,&mut e.nested(child_grant),r));progress=Self::receipt(step.progress());}else{let step=native!(|e,r|self.candidate_read.capture_into(&mut *self.candidate,&mut e.nested(child_grant),r));progress=Self::receipt(step.progress());if self.candidate.is_none(){self.phase=13;}}},
    13=>{if !self.projection.close_started(){let step=native!(|e,r|self.projection.begin_close_into(&mut e.nested(child_grant),r));progress=Self::receipt(step.progress());}else if !self.projection.owners_empty()||hint==Some(13){let step=routed!(13,|r| self.projection.close_into(body,&mut effect.nested(child_grant),r),self.projection.recover_close_into(body,&mut effect.nested(child_grant),r));progress=Self::receipt(step.progress());}else{work+=1;self.projection=Default::default();self.phase=5;}},
    5=>{let(child,receipt)=begin_retained_diff::<P,D>(child_grant).ok_or_else(invalid)?;work+=1;*self.child=Some(child);progress=Self::receipt(receipt);self.phase=6;},
    6=>{
     let source=Self::current(&self.previous,original)?;let candidate=if self.single{post}else{self.candidate_read.read(PagedOneItemPostMode::Original)?.original()?.project(0,ProjectionOutput::projected_value)};
     let step=routed!(6,|r| preflight_retained_diff_into(self.child.as_mut().ok_or_else(invalid)?,diffs.project(self.ordinal,|rows|&rows[self.ordinal]),source,candidate,body,maximum_work_units,&mut effect.nested(child_grant),receiving,r),recover_preflight_retained_diff_into(self.child.as_mut().ok_or_else(invalid)?,diffs.project(self.ordinal,|rows|&rows[self.ordinal]),source,candidate,body,maximum_work_units,&mut effect.nested(child_grant),receiving,r));progress=Self::receipt(step.progress());
     match step{RetainedDiffPreflightStep::Admitted(_,messages)=>{*self.pending_messages=Some(messages);self.phase=7;},RetainedDiffPreflightStep::Refused(_,messages)=>{*self.pending_messages=Some(messages);self.admitted=false;self.phase=7;},RetainedDiffPreflightStep::Progress(_)=>{}}
    },
    7=>{let step=native!(|e,r|self.messages.push_into(&mut *self.pending_messages,&mut e.nested(child_grant),r));progress=Self::receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.pending_messages.take();self.phase=if !self.admitted||self.single{14}else{8};}},
    8=>{let source=Self::current(&self.previous,original)?;let step=turn!(commit_retained_diff(self.child.as_mut().ok_or_else(invalid)?,diffs.project(self.ordinal,|rows|&rows[self.ordinal]),source,self.candidate_read.access(PagedOneItemPostMode::Mutable)?.mutable()?.projected_value_mut(),&mut effect.nested(child_grant)));progress=Self::receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.child.as_mut().ok_or_else(invalid)?.begin_close();self.phase=9;}},
    9=>{if self.child.as_ref().ok_or_else(invalid)?.terminal_is_empty()&&hint.is_none(){work+=1;self.child.take();self.phase=if self.ordinal+1==diffs.get().len(){14}else{10};}else{if !recover&&self.child.as_ref().ok_or_else(invalid)?.requires_original_control(){return Err(invalid().into())}let step=routed!(9,|r| self.child.as_mut().ok_or_else(invalid)?.close_into(body,&mut effect.nested(child_grant),r),self.child.as_mut().ok_or_else(invalid)?.recover_close_into(body,&mut effect.nested(child_grant),r));progress=Self::receipt(step.progress());}},
    10=>{if hint==Some(10)||(!self.previous.terminal_is_empty()&&self.previous.demand(PagedOneItemPostMode::Mutable,body)?.is_some()){let step=routed!(10,|r| self.previous.prepare_into(PagedOneItemPostMode::Mutable,body,&mut effect.nested(child_grant),r),self.previous.recover_prepare_into(PagedOneItemPostMode::Mutable,body,&mut effect.nested(child_grant),r));progress=Self::receipt(step.progress());}else if !self.previous.terminal_is_empty(){let step=native!(|e,r|self.previous.return_into(&mut *self.candidate,&mut e.nested(child_grant),r));progress=Self::receipt(step.progress());}else if hint==Some(11)||!self.candidate_retirement.is_empty(){let step=routed!(11,|r| self.candidate_retirement.step_into(body,&mut effect.nested(child_grant),r),self.candidate_retirement.recover_step_into(body,&mut effect.nested(child_grant),r));progress=Self::receipt(step.progress());}else if self.candidate.is_some(){if let Some(step)=turn!(self.candidate_retirement.begin_granted(&mut self.candidate,&mut effect.nested(child_grant))){progress=Self::receipt(step.progress());}}else{work+=1;self.phase=11;}},
    11=>{work+=1;std::mem::swap(&mut self.previous,&mut self.candidate_read);self.phase=12;},
    12=>{work+=1;self.ordinal+=1;self.phase=3;},
    14=>{progress=Self::receipt(native!(|e,r|self.messages.finish_into(&mut e.nested(child_grant),r)).progress());if self.messages.demand(body).is_ok(){self.phase=15;}},
    15=>{let step=native!(|e,r|self.messages.advance_into(body,&mut e.nested(child_grant),r));progress=Self::receipt(step.progress());if self.messages.ready(){self.phase=16;}},
    16=>{let step=native!(|e,r|self.messages.take_into(&mut *self.output_messages,&mut e.nested(child_grant),r));progress=Self::receipt(step.progress());if self.output_messages.is_some(){self.phase=17;return Ok(RetainedDiffPreflightAdvance::Ready(progress,self.admitted));}},
    17=>return Ok(RetainedDiffPreflightAdvance::Ready(Default::default(),self.admitted)),
    _=>return Err(invalid().into()),
   }Ok(RetainedDiffPreflightAdvance::Progress(progress))
  })();conclude(effect,base,result,work,RetainedDiffPreflightAdvance::progress,reply)}
 /// 📨️ The already funded joined original diagnostics transfer without copying payloads.
 pub fn take_messages(&mut self)->Vec<MutationMessage>{assert!(self.phase==17,"ordered original diagnostics require settled preflight");self.output_messages.take().expect("ordered original diagnostic transfer occurs once")}
 pub fn commit_demand(&self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=17||!self.admitted{return Err(invalid());}self.sources(diffs,original)?;
  if self.single{return Self::nested(self.child.as_ref().ok_or_else(invalid)?.commit_demand(diffs.project(0,|rows|&rows[0]),original,post,body.saturating_sub(Self::fixed()))?);}
  Ok(RetirementDemand{copy_bytes:Self::fixed()+if !self.candidate_read.terminal_is_empty(){size_of::<P>()*3}else{0},depth:1,..Default::default()})
 }
 /// 🔑️ Publication uses only the outer central capability after complete ordered preflight.
 /// ⛽️ Quotes the selected central child or the original candidate exchange.
 pub fn commit_work_demand(&self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:&P,body:usize)->Result<u64,ValueError>{self.commit_demand(diffs,original,post,body)?;if self.single{self.child.as_ref().ok_or_else(invalid)?.commit_work_demand(diffs.project(0,|rows|&rows[0]),original,post,body.saturating_sub(Self::fixed()))}else{Ok(1)}}
 pub fn commit(&mut self,diffs:RetainedCloneRef<'_,Vec<D>>,original:RetainedCloneRef<'_,P>,post:&mut P,effect:&mut RetainedAllocationEffect<'_>,_capability:ApplyCapability)->OriginalRetirementOutcome{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
  let demand=self.commit_demand(diffs,original,post,grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.single{let turn=commit_retained_diff(self.child.as_mut().ok_or_else(invalid)?,diffs.project(0,|rows|&rows[0]),original,post,&mut effect.nested(Self::child_grant(grant)));work+=turn.work;let step=turn.result.map_err(|error|if turn.work==0{error}else{let progress=Self::receipt(error.retained_progress());error.with_retained_progress(progress)})?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=18;}return Ok(match step{RetainedCloneStep::Progress(p)=>RetainedCloneStep::Progress(Self::receipt(p)),RetainedCloneStep::Complete(p)=>RetainedCloneStep::Complete(Self::receipt(p))});}
   work+=1;if !self.candidate_read.terminal_is_empty(){std::mem::swap(self.candidate_read.access(PagedOneItemPostMode::Mutable)?.mutable()?.projected_value_mut(),post);}self.phase=18;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 })();settle(effect,base,result,work)}
  pub fn begin_close(&mut self){self.closing=true;if let Some(child)=self.child.as_mut(){child.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}let body=body.saturating_sub(Self::fixed());
  let demand=if let Some(child)=self.child.as_ref(){if child.terminal_is_empty(){Default::default()}else{child.close_demand(body)?}}
  else if !self.projection.owners_empty(){if self.projection.close_started(){self.projection.close_demands(body)?}else{self.projection.begin_close_demand()?}}
   else if !self.projection.owners_empty(){if self.projection.close_started(){self.projection.close_demands(body)?}else{self.projection.begin_close_demand()?}}
   else if !self.messages.terminal_is_empty(){if self.messages.begin_close_demand().copy_bytes!=0{self.messages.begin_close_demand()}else{self.messages.close_demand(body)?}}
  else if !self.message_retirement.is_empty()||self.pending_messages.is_some(){self.message_retirement.next_owner_demands::<Vec<MutationMessage>>(self.pending_messages.is_some(),&None,body)?}
  else if self.output_messages.is_some(){self.message_retirement.next_owner_demands::<Vec<MutationMessage>>(true,&None,body)?}
   else if !self.previous.terminal_is_empty(){match self.previous.demand(PagedOneItemPostMode::Mutable,body)?{Some(demand)=>demand,None=>self.previous.return_demand()}}
   else if !self.candidate_read.terminal_is_empty(){match self.candidate_read.demand(PagedOneItemPostMode::Mutable,body)?{Some(demand)=>demand,None=>self.candidate_read.return_demand()}}
  else if self.candidate.is_some()||!self.candidate_retirement.is_empty(){self.candidate_retirement.next_owner_demands::<P::Output>(self.candidate.is_some(),&None,body)?}
  else if self.original.is_some(){binding_demand(&self.original,body)?}
  else if self.diff.is_some(){binding_demand(&self.diff,body)?}
  else{return Ok(Default::default());};Self::nested(demand)
 }
  pub fn requires_original_birth(&self)->bool{if !self.closing||!self.flight.idle(){return false}if let Some(c)=self.child.as_ref(){return !c.terminal_is_empty()&&c.requires_original_birth()}if !self.projection.owners_empty(){return self.projection.close_started()&&self.projection.requires_original_birth()}if !self.messages.terminal_is_empty(){return self.messages.begin_close_demand().copy_bytes==0&&self.messages.requires_original_birth()}if !self.message_retirement.is_empty(){return self.message_retirement.requires_original_birth()}if !self.previous.terminal_is_empty(){return self.previous.requires_original_birth()}if !self.candidate_read.terminal_is_empty(){return self.candidate_read.requires_original_birth()}!self.candidate_retirement.is_empty()&&self.candidate_retirement.requires_original_birth()}
  pub fn requires_original_control(&self)->bool{if let Some(loaned)=self.flight.loaned(){return loaned}if let Some(c)=self.child.as_ref(){return !c.terminal_is_empty()&&c.requires_original_control()}if !self.projection.owners_empty(){return self.projection.close_started()&&self.projection.requires_original_control()}if !self.messages.terminal_is_empty(){return false}if !self.message_retirement.is_empty(){return self.message_retirement.requires_original_control()}if !self.previous.terminal_is_empty(){return self.previous.requires_original_control()}if !self.candidate_read.terminal_is_empty(){return self.candidate_read.requires_original_control()}!self.candidate_retirement.is_empty()&&self.candidate_retirement.requires_original_control()}
  pub fn close_original_birth_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if let Some(c)=self.child.as_mut().filter(|c|!c.terminal_is_empty()){return c.close_original_birth_step(original,grant)}if !self.projection.owners_empty(){return self.projection.close_original_birth_step(original,grant)}if !self.messages.terminal_is_empty(){return self.messages.close_original_birth_step(original,grant)}if !self.message_retirement.is_empty(){return self.message_retirement.step_original_birth(original,grant)}if !self.previous.terminal_is_empty(){return self.previous.original_birth_step(original,grant)}if !self.candidate_read.terminal_is_empty(){return self.candidate_read.original_birth_step(original,grant)}self.candidate_retirement.step_original_birth(original,grant)}
  pub fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(None,body,effect,reply,false)}
  pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(None,body,effect,reply,true)}
  pub fn close_original_into<'r>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
  pub fn recover_close_original_into<'r>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}
  fn close_reply<'r>(&mut self,mut original:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   macro_rules! turn {($value:expr)=>{{let turn=$value;work+=turn.work;turn.result.map_err(|error|{let error:OriginalRetirementFailure=error.into();if turn.work==0{error}else{let progress=Self::receipt(error.retained_progress());error.with_retained_progress(progress)}})?}};}
   macro_rules! poll {($value:expr)=>{match RetirementPollExit::settled($value){Ok(outcome)=>turn!(outcome),Err(exit)=>return Err(exit)}};}
   macro_rules! native {($call:expr)=>{{let before=work;native_child(effect,&mut work,$call)?.map_err(|error|{let error:OriginalRetirementFailure=error.into();if work==before{error}else{let progress=Self::receipt(error.retained_progress());error.with_retained_progress(progress)}})?}};}
   if !self.closing{return Err(invalid().into());}
   let child_grant=Self::child_grant(grant);let mut route:u8=0;
   if recover{route=self.flight.route().ok_or_else(invalid)?;}else{
    if !self.flight.idle(){return Err(invalid().into())}
    if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
    let demand=self.close_demand(body)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
    if self.requires_original_birth(){return Err(invalid().into())}
    if self.requires_original_control()&&original.is_none(){return Err(invalid().into())}
    if let Some(child)=self.child.as_mut(){if child.terminal_is_empty(){work+=1;self.child.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}route=if child.requires_original_control(){31}else{30};}
    else if !self.projection.owners_empty(){if !self.projection.close_started(){let step=native!(|e,r|self.projection.begin_close_into(&mut e.nested(child_grant),r));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}route=if self.projection.requires_original_control(){33}else{32};}
    else if !self.messages.terminal_is_empty(){if self.messages.begin_close_demand().copy_bytes!=0{let step=native!(|e,r|self.messages.begin_close_into(&mut e.nested(child_grant),r));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}route=if original.is_some(){35}else{34};}
    else if !self.message_retirement.is_empty(){route=if self.message_retirement.requires_original_control(){37}else{36};}
    else if let Some(step)=turn!(self.message_retirement.begin_granted(&mut self.pending_messages,&mut effect.nested(child_grant))){return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
    else if let Some(step)=turn!(self.message_retirement.begin_granted(&mut self.output_messages,&mut effect.nested(child_grant))){return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
    else if !self.previous.terminal_is_empty(){if self.previous.demand(PagedOneItemPostMode::Mutable,body)?.is_some(){route=if self.previous.requires_original_control(){39}else{38};}else{let step=native!(|e,r|self.previous.return_into(&mut *self.candidate,&mut e.nested(child_grant),r));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}}
    else if !self.candidate_read.terminal_is_empty(){if self.candidate_read.demand(PagedOneItemPostMode::Mutable,body)?.is_some(){route=if self.candidate_read.requires_original_control(){41}else{40};}else{let step=native!(|e,r|self.candidate_read.return_into(&mut *self.candidate,&mut e.nested(child_grant),r));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}}
    else if !self.candidate_retirement.is_empty(){route=if self.candidate_retirement.requires_original_control(){43}else{42};}
    else if let Some(step)=turn!(self.candidate_retirement.begin_granted(&mut self.candidate,&mut effect.nested(child_grant))){return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
    else if self.original.is_some(){let step=poll!(RetainedCloneBinding::close_one(&mut self.original,body,&mut effect.nested(child_grant)));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
    else{let step=poll!(RetainedCloneBinding::close_one(&mut self.diff,body,&mut effect.nested(child_grant)));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
   }
   let loan=if matches!(route,31|33|35|37|39|41|43){original.as_deref_mut()}else{None};let missing=||ValueError::invariant("ordered retained diff original controller missing");
   let before=work;
   let step=retained_routed!(self.flight,effect,recover,route,body,0,loan,work,|r| match route{
     30=>self.child.as_mut().ok_or_else(invalid)?.close_into(body,&mut effect.nested(child_grant),r),
     31=>self.child.as_mut().ok_or_else(invalid)?.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,&mut effect.nested(child_grant),r),
     32=>self.projection.close_into(body,&mut effect.nested(child_grant),r),
     33=>self.projection.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,&mut effect.nested(child_grant),r),
     34=>self.messages.close_into(body,&mut effect.nested(child_grant),r),
     35=>self.messages.close_original_into(body,original.as_deref_mut().ok_or_else(missing)?,&mut effect.nested(child_grant),r),
     36=>self.message_retirement.step_into(body,&mut effect.nested(child_grant),r),
     37=>self.message_retirement.step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,&mut effect.nested(child_grant),r),
     38=>self.previous.prepare_into(PagedOneItemPostMode::Mutable,body,&mut effect.nested(child_grant),r),
     39=>self.previous.prepare_original_into(PagedOneItemPostMode::Mutable,body,original.as_deref_mut().ok_or_else(missing)?,&mut effect.nested(child_grant),r),
     40=>self.candidate_read.prepare_into(PagedOneItemPostMode::Mutable,body,&mut effect.nested(child_grant),r),
     41=>self.candidate_read.prepare_original_into(PagedOneItemPostMode::Mutable,body,original.as_deref_mut().ok_or_else(missing)?,&mut effect.nested(child_grant),r),
     42=>self.candidate_retirement.step_into(body,&mut effect.nested(child_grant),r),
     43=>self.candidate_retirement.step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,&mut effect.nested(child_grant),r),
     _=>Err(invalid())},match route{
     30=>self.child.as_mut().ok_or_else(invalid)?.recover_close_into(body,&mut effect.nested(child_grant),r),
     31=>self.child.as_mut().ok_or_else(invalid)?.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,&mut effect.nested(child_grant),r),
     32=>self.projection.recover_close_into(body,&mut effect.nested(child_grant),r),
     33=>self.projection.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,&mut effect.nested(child_grant),r),
     34=>self.messages.recover_close_into(body,&mut effect.nested(child_grant),r),
     35=>self.messages.recover_close_original_into(body,original.as_deref_mut().ok_or_else(missing)?,&mut effect.nested(child_grant),r),
     36=>self.message_retirement.recover_step_into(body,&mut effect.nested(child_grant),r),
     37=>self.message_retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,&mut effect.nested(child_grant),r),
     38=>self.previous.recover_prepare_into(PagedOneItemPostMode::Mutable,body,&mut effect.nested(child_grant),r),
     39=>self.previous.recover_prepare_original_into(PagedOneItemPostMode::Mutable,body,original.as_deref_mut().ok_or_else(missing)?,&mut effect.nested(child_grant),r),
     40=>self.candidate_read.recover_prepare_into(PagedOneItemPostMode::Mutable,body,&mut effect.nested(child_grant),r),
     41=>self.candidate_read.recover_prepare_original_into(PagedOneItemPostMode::Mutable,body,original.as_deref_mut().ok_or_else(missing)?,&mut effect.nested(child_grant),r),
     42=>self.candidate_retirement.recover_step_into(body,&mut effect.nested(child_grant),r),
     43=>self.candidate_retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,&mut effect.nested(child_grant),r),
     _=>Err(invalid())}).map_err(|error|{let error:OriginalRetirementFailure=error.into();if work==before{error}else{let progress=Self::receipt(error.retained_progress());error.with_retained_progress(progress)}})?;
   Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())))
  })();conclude(effect,base,result,work,|step|step.progress(),reply)}
  fn empty(&self)->bool{self.child.is_none()&&self.candidate_read.terminal_is_empty()&&self.previous.terminal_is_empty()&&self.candidate.is_none()&&self.pending_messages.is_none()&&self.output_messages.is_none()&&self.projection.owners_empty()&&self.messages.terminal_is_empty()&&self.message_retirement.is_empty()&&self.candidate_retirement.is_empty()&&self.original.is_none()&&self.diff.is_none()&&self.flight.idle()}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.empty()}
}
impl<P:SnapshotProjection+Send+'static,D:RetainedDiffFragment<P>> Drop for RetainedDiffSequence<P,D>{fn drop(&mut self){let empty=self.empty();assert!(std::thread::panicking()||empty,"ordered retained diff requires paid child, source, diagnostic and candidate closure");if empty{unsafe{ManuallyDrop::drop(&mut self.child);ManuallyDrop::drop(&mut self.candidate);ManuallyDrop::drop(&mut self.pending_messages);ManuallyDrop::drop(&mut self.output_messages);}}}}
