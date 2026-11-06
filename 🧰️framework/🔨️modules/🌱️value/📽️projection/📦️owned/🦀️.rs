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
 pub fn begin(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),ValueError>{let mut work=0;let result=(||{let d=Self::birth_demand();if !retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"owned field projection birth is not funded"));}work=1;Ok((Self{cursor:Default::default(),taken:false,closing:false,child_started:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"owned field projection is closing, transferred or incomplete")}
impl<T:RetainedClone+SnapshotProjection<Output=T>> SnapshotProjectionCursor<T> for OwnedFieldProjection<T>{
 fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.taken{return Err(invalid());}self.cursor.advance_demands(original,body)}
 fn advance_work_demand(&self,original:RetainedCloneRef<'_,T>,body:usize)->Result<u64,ValueError>{self.demands(original,body)?;self.cursor.advance_work_demand(original,body)}
 fn advance(&mut self,original:RetainedCloneRef<'_,T>,body:usize,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{let d=self.demands(original,body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}let step={let turn=self.cursor.advance(original,body,grant);work+=turn.work;turn.result}?;admit_retained_clone_progress(grant,step.progress(),"original owned field projection")?;Ok(step)})();crate::RetainedWorkOutcome{result,work}}
 fn ready(&self)->bool{!self.closing&&!self.taken&&self.cursor.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}self.cursor.take_demand()}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;self.cursor.take_work_demand()}
 fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>,ValueError>{if !self.ready(){return crate::RetainedWorkOutcome{result:Err(invalid()),work:0}}let turn=self.cursor.take(grant);if matches!(&turn.result,Ok(Some(_))){self.taken=true}turn}
 fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();self.child_started=true;}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
 fn begin_close_paid(&mut self)->crate::RetainedWorkOutcome<(),ValueError>{if self.closing{return crate::RetainedWorkOutcome{result:Ok(()),work:0}}self.closing=true;crate::RetainedWorkOutcome{result:Ok(()),work:1}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if !self.child_started{return Ok(RetirementDemand{depth:1,..Default::default()})}retained_clone_close_demands::<T>(&self.cursor,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demands(body)?;if !self.child_started{return self.cursor.begin_close_work_demand()?.checked_add(1).ok_or_else(||ValueError::invariant("owned projection close-start work overflow"))}self.cursor.close_work_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.child_started{let turn=self.cursor.begin_close_paid();work+=turn.work;turn.result?;self.child_started=true;work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}let step={let turn=self.cursor.close_step(grant);work+=turn.work;turn.result}?;Ok(admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original owned field projection closure")?)})();crate::RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.terminal_is_empty()}
}
impl SnapshotProjection for String{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress)>{Self::Projection::begin(grant)}
}
impl<T:RetainedClone> SnapshotProjection for Vec<T>{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress)>{Self::Projection::begin(grant)}
}

impl<T:RetainedClone> SnapshotProjection for Option<T>{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress)>{Self::Projection::begin(grant)}
}

impl<T:RetainedClone,const N:usize> SnapshotProjection for [T;N]{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress)>{Self::Projection::begin(grant)}
}
