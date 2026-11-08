//! 📦️ Contiguous owned fields project through genuine fragment-copy and retirement cursors.
use super::{SnapshotProjection,SnapshotProjectionCursor};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedFieldCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits,retained_clone_close_demands,admit_retained_clone_progress,admit_retained_clone_close}};
use std::mem::size_of;

/// 🧺️ One explicitly owned field retains its original issuer and funds every copied fragment.
pub struct OwnedFieldProjection<T:RetainedClone>{cursor:RetainedFieldCursor<T>,taken:bool,closing:bool,child_started:bool}
impl<T:RetainedClone> OwnedFieldProjection<T>{
 /// 📏️ Quotes the fixed original-field projection body before any source is inspected.
 pub fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()*2,depth:1,..Default::default()}}
 /// 🌱️ Constructs the original-field projection only after its complete birth grant.
 pub fn begin(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),ValueError>{let grant=effect.grant();let mut work=0;let result=(||{if effect.maximum_work_units()==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original projection birth work is not funded"))}let d=Self::birth_demand();if !retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"owned field projection birth is not funded"));}work=1;Ok((Self{cursor:Default::default(),taken:false,closing:false,child_started:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"owned field projection is closing, transferred or incomplete")}
impl<T:RetainedClone+SnapshotProjection<Output=T>> SnapshotProjectionCursor<T> for OwnedFieldProjection<T>{
 fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.taken{return Err(invalid());}self.cursor.advance_demands(original,body,maximum_work_units)}
 fn advance_work_demand(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.demands(original,body,maximum_work_units)?;self.cursor.advance_work_demand(original,body,maximum_work_units)}
 fn advance(&mut self,original:RetainedCloneRef<'_,T>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=effect.maximum_work_units();let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{let d=self.demands(original,body,maximum_work_units)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}let step={let turn=self.cursor.advance(original,body,maximum_work_units,effect);work+=turn.work;turn.result}?;admit_retained_clone_progress(grant,step.progress(),"original owned field projection")?;Ok(step)})();crate::RetainedWorkOutcome{result,work}}
 fn ready(&self)->bool{!self.closing&&!self.taken&&self.cursor.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}self.cursor.take_demand()}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;self.cursor.take_work_demand()}
 fn take(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>,ValueError>{if !self.ready(){return crate::RetainedWorkOutcome{result:Err(invalid()),work:0}}let before=effect.performed_work();let mut slot=None;let mut output=None;let result=(||{let reply=crate::retained_clone::OriginalEffectReply::borrow(&mut slot)?;drop(self.cursor.take(&mut output,effect,reply)?);match slot.take(){Some(Ok(RetainedCloneStep::Complete(progress)))=>{self.taken=true;Ok(output.take().map(|value|(value,progress)))},Some(Ok(RetainedCloneStep::Progress(_)))|None=>Ok(None),Some(Err(error))=>Err(error)}})();crate::RetainedWorkOutcome{result,work:effect.performed_work().saturating_sub(before)}}
 
 fn close_started(&self)->bool{self.closing}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
 fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,crate::ValueError>{let grant=effect.grant();let mut work=0;let result=(||{let demand=self.begin_close_demand()?;if demand.copy_bytes==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Default::default())}self.closing=true;work=1;Ok(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})})();crate::RetainedWorkOutcome{result,work}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if !self.child_started{return if self.cursor.close_started(){Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}else{self.cursor.begin_close_demand()}}retained_clone_close_demands::<T>(&self.cursor,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demands(body)?;if !self.child_started{return if self.cursor.close_started(){Ok(1)}else{self.cursor.begin_close_work_demand()}}self.cursor.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.child_started{if !self.cursor.close_started(){let turn=self.cursor.begin_close_paid(effect);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}self.child_started=true;work=1;let progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()};return Ok(RetainedCloneStep::Progress(progress));}let step={let turn=self.cursor.close_step(&mut effect.nested(grant));work+=turn.work();match turn{crate::retained_clone::RetainedWorkPoll::Complete(done)=>done.result,crate::retained_clone::RetainedWorkPoll::Pending{..}=>return Ok(RetainedCloneStep::Progress(Default::default()))}}?;Ok(admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original owned field projection closure")?)})();crate::RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.terminal_is_empty()}
}
impl SnapshotProjection for String{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress)>{Self::Projection::begin(effect)}
}
impl<T:RetainedClone> SnapshotProjection for Vec<T>{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress)>{Self::Projection::begin(effect)}
}

impl<T:RetainedClone> SnapshotProjection for Option<T>{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress)>{Self::Projection::begin(effect)}
}

impl<T:RetainedClone,const N:usize> SnapshotProjection for [T;N]{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress)>{Self::Projection::begin(effect)}
}
