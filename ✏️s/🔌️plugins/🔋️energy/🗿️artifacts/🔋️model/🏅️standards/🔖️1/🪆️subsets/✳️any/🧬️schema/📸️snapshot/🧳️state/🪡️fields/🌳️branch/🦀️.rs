//! 🌳️ Original immutable branch projection opens one owned candidate before separately funded header adoption and sealing.
use super::{invalid,nested,child,add,receipt,tagged};
use protocol::{retained_mutation_demand_fits,retained::{Done,Flight,Reply,conclude,native_child}};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_snapshot_projection::snapshot_projection::{SnapshotProjection,ProjectionField};
use std::mem::{size_of,ManuallyDrop};

/// 📦️ Projection output keeps the defining boxed or inline owner until its exact admitted handoff.
pub trait BranchProjectionOutput<B>:RetireOwned{fn release_bytes()->usize;fn into_branch(self)->B;}
impl<B:RetireOwned> BranchProjectionOutput<B> for Box<B>{fn release_bytes()->usize{size_of::<B>()}fn into_branch(self)->B{*self}}
impl<T:RetireOwned,const N:usize> BranchProjectionOutput<[T;N]> for [T;N]{fn release_bytes()->usize{0}fn into_branch(self)->[T;N]{self}}
/// 🧳️ Projected branch, unboxed candidate and immutable result preserve independent cancellation custody.
pub struct RetainedBranchProjection<B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send>{binding:Option<RetainedCloneBinding>,projection:ProjectionField<B>,boxed:ManuallyDrop<Option<B::Output>>,raw:ManuallyDrop<Option<B>>,output:ManuallyDrop<Option<SealedShared<B>>>,retirement:RetainedCloneClose,flight:Flight,phase:u8,closing:bool}
impl<B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send> Default for RetainedBranchProjection<B>{fn default()->Self{Self::new()}}
impl<B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send> RetainedBranchProjection<B>{
 pub fn new()->Self{Self{binding:None,projection:Default::default(),boxed:ManuallyDrop::new(None),raw:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),flight:Default::default(),phase:0,closing:false}}
 pub fn raw_ready(&self)->bool{!self.closing&&self.phase==4&&self.raw.is_some()}
 pub fn ready(&self)->bool{!self.closing&&self.phase==6&&self.output.is_some()}
 pub fn demand(&self,source:RetainedCloneRef<'_,B>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||matches!(self.phase,4|6|7)||!self.flight.idle(){return Err(invalid());}source.check_binding(&self.binding)?;match self.phase{
   0=>nested(source.binding_demand(&self.binding)?.ok_or_else(invalid)?,1),
   1=>nested(self.projection.demands(source,body.saturating_sub(1),maximum_work_units)?,1),
   2=>nested(self.projection.take_output_demand()?,2*size_of::<Option<B::Output>>()+1),
   3=>Ok(RetirementDemand{copy_bytes:2*size_of::<B>()+2*size_of::<Option<B::Output>>()+2*size_of::<Option<B>>()+1,release_bytes:<B::Output as BranchProjectionOutput<B>>::release_bytes(),depth:1,..Default::default()}),
   5=>{let mut d=SealedShared::<B>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<B>>()+1).ok_or_else(invalid)?;Ok(d)},
   _=>Err(invalid()),
  }
 }
 pub fn advance_work_demand(&self,source:RetainedCloneRef<'_,B>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
  self.demand(source,body,maximum_work_units)?;
  match self.phase{0=>source.binding_work_demand(&self.binding),1=>self.projection.advance_work_demand(source,body.saturating_sub(1),maximum_work_units),2|3=>Ok(1),5=>Ok(SealedShared::<B>::source_admission_work_demand()),_=>Err(invalid())}
 }
 pub fn advance_reply<'r>(&mut self,source:RetainedCloneRef<'_,B>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   if recover{self.flight.route().ok_or_else(invalid)?;}else{
    if !self.flight.idle(){return Err(invalid().into())}
    let demand=self.demand(source,body,maximum_work_units)?;let units=self.advance_work_demand(source,body,maximum_work_units)?;
    if maximum_work_units==0||units==0||effect.remaining_work_units()?<units||!retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    match self.phase{
     0=>{let turn=source.bind(&mut self.binding,effect);work+=turn.work;let progress=turn.result?.ok_or_else(invalid)?;self.phase=1;return Ok(RetainedCloneStep::Progress(add(progress,1)?))},
     2=>{let projection=&mut self.projection;let boxed=&mut *self.boxed;let step=native_child(effect,&mut work,|e,r|projection.take_output_into(boxed,e,r))??;self.phase=3;return Ok(RetainedCloneStep::Progress(add(step.progress(),2*size_of::<Option<B::Output>>()+1)?))},
     3=>{work=1;let value=self.boxed.take().ok_or_else(invalid)?;*self.raw=Some(value.into_branch());self.phase=4;return Ok(RetainedCloneStep::Progress(receipt(demand)))},
     5=>{let turn=SealedShared::admit_from(&mut self.raw,SharedIssuer::owned(),&mut self.output,effect);work+=turn.work;let step=turn.result?;if self.output.is_none(){return Ok(RetainedCloneStep::Progress(step.progress()))}self.phase=6;return Ok(RetainedCloneStep::Complete(add(step.progress(),2*size_of::<Option<B>>()+1)?))},
     _=>{}
    }
   }
   let nested_body=body.saturating_sub(1);let before=work;
   let step=protocol::retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| self.projection.advance_into(source,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r),self.projection.recover_advance_into(source,nested_body,maximum_work_units,&mut effect.nested(child(g,1)),r));
   let step=step.map_err(|error|tagged(error,work!=before))?;
   if self.projection.ready(){self.phase=2;}
   Ok(RetainedCloneStep::Progress(add(step.progress(),1)?))
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn exchange_demand<F>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<F>()+2*size_of::<Option<F>>()+1,depth:1,..Default::default()}}
 pub fn exchange<F:RetireOwned>(&mut self,select:for<'a>fn(&'a mut B)->&'a mut F,value:&mut Option<F>,displaced:&mut Option<F>,effect:&mut RetainedAllocationEffect<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{if !self.raw_ready()||value.is_none()||displaced.is_some(){return Err(invalid());}let d=Self::exchange_demand::<F>();if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok((false,Default::default()));}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Energy branch exchange exceeded its action"));}let value=value.take().ok_or_else(invalid)?;*displaced=Some(std::mem::replace(select(self.raw.as_mut().ok_or_else(invalid)?),value));Ok((true,receipt(d)))}
 pub fn exchange_with<F:RetireOwned>(&mut self,replace:fn(&mut B,F)->F,value:&mut Option<F>,displaced:&mut Option<F>,effect:&mut RetainedAllocationEffect<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{if !self.raw_ready()||value.is_none()||displaced.is_some(){return Err(invalid());}let d=Self::exchange_demand::<F>();if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok((false,Default::default()));}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Energy branch exchange exceeded its action"));}let value=value.take().ok_or_else(invalid)?;*displaced=Some(replace(self.raw.as_mut().ok_or_else(invalid)?,value));Ok((true,receipt(d)))}
 pub fn begin_seal(&mut self)->Result<(),ValueError>{if !self.raw_ready(){return Err(invalid());}self.phase=5;Ok(())}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(RetirementDemand{copy_bytes:2*size_of::<Option<SealedShared<B>>>()+1,depth:1,..Default::default()})}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(SealedShared<B>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None);}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Energy branch take exceeded its action"));}let value=self.output.take().ok_or_else(invalid)?;self.phase=7;Ok(Some((value,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if !self.projection.close_started(){return nested(self.projection.begin_close_demand()?,0);}
  if !self.projection.terminal_is_empty(){return nested(self.projection.close_demands(body.saturating_sub(1))?,1);}
  if !self.retirement.is_empty()||self.boxed.is_some(){return self.retirement.next_owner_demands::<B::Output>(self.boxed.is_some(),&None,body);}if self.raw.is_some(){return self.retirement.next_owner_demands::<B>(true,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<SealedShared<B>>(true,&None,body);}RetainedCloneBinding::close_demand(&self.binding,body)
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demand(body)?;
  if !self.projection.close_started(){return self.projection.begin_close_work_demand();}
  if !self.projection.terminal_is_empty(){return self.projection.close_work_demand(body.saturating_sub(1));}
  if !self.retirement.is_empty()||self.boxed.is_some(){return self.retirement.next_owner_work::<B::Output>(self.boxed.is_some(),&None,body);}if self.raw.is_some(){return self.retirement.next_owner_work::<B>(true,&None,body);}if self.output.is_some(){return self.retirement.next_owner_work::<SealedShared<B>>(true,&None,body);}RetainedCloneBinding::close_work_demand(&self.binding,body)
 }
 pub fn requires_original_birth(&self)->bool{
  if !self.closing||!self.flight.idle(){return false}
  if !self.projection.terminal_is_empty(){return self.projection.close_started()&&self.projection.requires_original_birth()}
  if !self.retirement.is_empty(){return self.retirement.requires_original_birth()}
  if self.boxed.is_some()||self.raw.is_some()||self.output.is_some(){return false}RetainedCloneBinding::requires_original_birth(&self.binding)
 }
 pub fn requires_original_control(&self)->bool{
  if let Some(loaned)=self.flight.loaned(){return loaned}
  if !self.projection.terminal_is_empty(){return self.projection.close_started()&&self.projection.requires_original_control()}
  if !self.retirement.is_empty(){return self.retirement.requires_original_control()}
  if self.boxed.is_some()||self.raw.is_some()||self.output.is_some(){return false}RetainedCloneBinding::requires_original_control(&self.binding)
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if !self.projection.terminal_is_empty(){return self.projection.close_original_birth_step(original,grant)}
  if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,grant)}
  RetainedCloneBinding::close_original_birth_step(&mut self.binding,original,grant)
 }
 close_entries!();
 pub fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut route:u8=0;
   if recover{route=self.flight.route().ok_or_else(invalid)?;}else{
    if !self.flight.idle(){return Err(invalid().into())}
    let demand=self.close_demand(body)?;if !retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if self.requires_original_birth(){return Err(invalid().into())}
    if self.requires_original_control()&&original.is_none(){return Err(invalid().into())}
    if !self.projection.close_started(){route=1;}
    else if !self.projection.terminal_is_empty(){route=if self.projection.requires_original_control(){3}else{2};}
    else if !self.retirement.is_empty(){route=if self.retirement.requires_original_control(){5}else{4};}
    else if self.boxed.is_some(){let turn=self.retirement.begin_granted(&mut self.boxed,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}return Err(invalid().into())}
    else if self.raw.is_some(){let turn=self.retirement.begin_granted(&mut self.raw,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}return Err(invalid().into())}
    else if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}return Err(invalid().into())}
    else if self.binding.is_some(){route=if RetainedCloneBinding::requires_original_control(&self.binding){7}else{6};}
    else{return Ok(RetainedCloneStep::Complete(Default::default()))}
   }
   let nested_body=body.saturating_sub(1);let loan=if matches!(route,3|5|7){original.as_deref_mut()}else{None};let before=work;
   let missing=||ValueError::invariant("original Energy branch close lacks its original controller");
   let step=protocol::retained_routed!(self.flight,effect,recover,route,body,0,loan,work,|r| match route{
    1=>self.projection.begin_close_into(&mut effect.nested(child(g,0)),r),
    2=>self.projection.close_into(nested_body,&mut effect.nested(child(g,1)),r),
    3=>self.projection.close_original_into(original.as_deref_mut().ok_or_else(missing)?,nested_body,&mut effect.nested(child(g,1)),r),
    4=>self.retirement.step_into(body,effect,r),
    5=>self.retirement.step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    6=>RetainedCloneBinding::close_into(&mut self.binding,body,effect,r),
    7=>RetainedCloneBinding::close_original_into(&mut self.binding,original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    _=>Err(invalid())},
    match route{
    1=>self.projection.recover_begin_close_into(&mut effect.nested(child(g,0)),r),
    2=>self.projection.recover_close_into(nested_body,&mut effect.nested(child(g,1)),r),
    3=>self.projection.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,nested_body,&mut effect.nested(child(g,1)),r),
    4=>self.retirement.recover_step_into(body,effect,r),
    5=>self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    6=>RetainedCloneBinding::recover_close_into(&mut self.binding,body,effect,r),
    7=>RetainedCloneBinding::recover_close_original_into(&mut self.binding,original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
    _=>Err(invalid())});
   let step=step.map_err(|error|tagged(error,work!=before))?;
   Ok(RetainedCloneStep::Progress(step.progress()))
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.projection.owners_empty()&&self.boxed.is_none()&&self.raw.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.flight.idle()}
}
impl<B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send> Drop for RetainedBranchProjection<B>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original Energy branch retains funded projection and source custody");if empty{unsafe{ManuallyDrop::drop(&mut self.boxed);ManuallyDrop::drop(&mut self.raw);ManuallyDrop::drop(&mut self.output);}}}}
