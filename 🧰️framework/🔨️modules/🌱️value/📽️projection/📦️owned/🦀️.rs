//! 📦️ Contiguous owned fields project through genuine fragment-copy and retirement cursors.
use super::{SnapshotProjection,SnapshotProjectionCursor};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedFieldCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits,retained_clone_close_demands,admit_retained_clone_progress,admit_retained_clone_close}};
use std::mem::size_of;

/// 🧺️ One explicitly owned field retains its original issuer and funds every copied fragment.
pub struct OwnedFieldProjection<T:RetainedClone>{cursor:RetainedFieldCursor<T>,taken:bool,closing:bool}
impl<T:RetainedClone> OwnedFieldProjection<T>{
 /// 📏️ Quotes the fixed original-field projection body before any source is inspected.
 pub fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()*2,depth:1,..Default::default()}}
 /// 🌱️ Constructs the original-field projection only after its complete birth grant.
 pub fn begin(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::birth_demand();if !retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"owned field projection birth is not funded"));}Ok((Self{cursor:Default::default(),taken:false,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"owned field projection is closing, transferred or incomplete")}
impl<T:RetainedClone> SnapshotProjectionCursor<T> for OwnedFieldProjection<T>{
 fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.taken{return Err(invalid());}self.cursor.advance_demands(original,body)}
 fn advance(&mut self,original:RetainedCloneRef<'_,T>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(original,body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}let step=self.cursor.advance(original,body,grant)?;admit_retained_clone_progress(grant,step.progress(),"original owned field projection")?;Ok(step)}
 fn ready(&self)->bool{!self.closing&&!self.taken&&self.cursor.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(RetirementDemand{copy_bytes:size_of::<T>()*2+size_of::<bool>(),depth:1,..Default::default()})}
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let value=self.cursor.take().ok_or_else(invalid)?;self.taken=true;Ok(Some((value,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 fn begin_close(&mut self){self.closing=true;self.cursor.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}retained_clone_close_demands::<T>(&self.cursor,body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}let step=self.cursor.close_step(grant)?;admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original owned field projection closure")}
 fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.terminal_is_empty()}
}
impl SnapshotProjection for String{
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn begin_projection(grant:RetainedCloneGrant)->Result<(Self::Projection,RetainedCloneProgress),ValueError>{Self::Projection::begin(grant)}
}
impl<T:RetainedClone> SnapshotProjection for Vec<T>{
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn begin_projection(grant:RetainedCloneGrant)->Result<(Self::Projection,RetainedCloneProgress),ValueError>{Self::Projection::begin(grant)}
}

impl<T:RetainedClone> SnapshotProjection for Option<T>{
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn begin_projection(grant:RetainedCloneGrant)->Result<(Self::Projection,RetainedCloneProgress),ValueError>{Self::Projection::begin(grant)}
}

impl<T:RetainedClone,const N:usize> SnapshotProjection for [T;N]{
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn begin_projection(grant:RetainedCloneGrant)->Result<(Self::Projection,RetainedCloneProgress),ValueError>{Self::Projection::begin(grant)}
}
