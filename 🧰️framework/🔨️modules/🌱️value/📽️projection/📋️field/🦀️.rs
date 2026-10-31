//! 📋️ Composite immutable projection fields retain original child candidates through paid handoff and cancellation.
use super::{SnapshotProjection,SnapshotProjectionCursor,ProjectionOutput};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits,admit_retained_clone_progress,admit_retained_clone_close}};
use std::mem::{size_of,ManuallyDrop,MaybeUninit};
/// 🧺️ Every lazy original child, returned value and retirement cursor keeps explicit custody.
pub struct ProjectionField<T:SnapshotProjection>{cursor:ManuallyDrop<Option<T::Projection>>,value:ManuallyDrop<Option<T::Output>>,retirement:RetainedCloneClose,ready:bool,closing:bool}
impl<T:SnapshotProjection> Default for ProjectionField<T>{fn default()->Self{Self{cursor:ManuallyDrop::new(None),value:ManuallyDrop::new(None),retirement:Default::default(),ready:false,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable field projection is closing, moved or missing its original child")}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"immutable field nested depth overflow"))?;Ok(demand)}
impl<T:SnapshotProjection> ProjectionField<T>{
 pub fn owners_empty(&self)->bool{self.cursor.is_none()&&self.value.is_none()&&self.retirement.is_empty()}
 pub fn ready(&self)->bool{!self.closing&&self.ready&&self.value.is_some()}
 pub fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.ready{return Err(invalid());}
  let Some(cursor)=self.cursor.as_ref()else{return nested(T::projection_birth_demand());};
  if self.value.is_none(){return nested(if cursor.ready(){cursor.take_demand()?}else{cursor.demands(original,body)?});}
  if cursor.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<T::Projection>(),depth:1,..Default::default()});}
  nested(cursor.close_demands(body)?)
 }
 pub fn advance(&mut self,original:RetainedCloneRef<'_,T>,body:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,ValueError>{let mut work=0;let result=(||{
  let d=self.demands(original,body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  let Some(cursor)=self.cursor.as_mut()else{let(cursor,progress)={let turn=T::begin_projection(child);work+=turn.work;turn.result}?;*self.cursor=Some(cursor);return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(child,progress,"immutable original field cursor birth")?));};
  if self.value.is_none(){
   if !cursor.ready(){let step={let turn=cursor.advance(original,body,child);work+=turn.work;turn.result}?;return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(child,step.progress(),"immutable original field projection")?));}
   let Some((value,progress))={let turn=cursor.take(child);work+=turn.work;turn.result}?else{return Err(invalid());};*self.value=Some(value);cursor.begin_close();let progress=admit_retained_clone_progress(child,progress,"immutable original field output transfer")?;return Ok(RetainedCloneStep::Progress(progress));
  }
  if cursor.terminal_is_empty(){*self.cursor=None;work+=1;self.ready=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
  let step={let turn=cursor.close_step(child);work+=turn.work;turn.result}?;Ok(RetainedCloneStep::Progress(admit_retained_clone_close(child,step,cursor.terminal_is_empty(),"immutable original field projection close")?.progress()))
 })();crate::RetainedWorkOutcome{result,work}}
 /// 👐️ Quotes moving the initialized associated output without materializing its projected payload.
 pub fn take_output_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(RetirementDemand{copy_bytes:size_of::<T::Output>()*2,depth:1,..Default::default()})}
 /// 🎁️ Transfers the same associated owner only after its complete handle grant is admitted.
 pub fn take_output(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(T::Output,RetainedCloneProgress)>,ValueError>{let mut work=0;let result=(||{let d=self.take_output_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let value=self.value.take().ok_or_else(invalid)?;work+=1;Ok(Some((value,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))})();crate::RetainedWorkOutcome{result,work}}
 pub fn place_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(<T::Output as ProjectionOutput<T>>::placement_demand())}
 pub fn place(&mut self,target:&mut MaybeUninit<T>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{let mut work=0;let result=(||{let d=self.place_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let value=self.value.take().ok_or_else(invalid)?;match {let turn=value.place(target,grant);work+=turn.work;turn.result}{Ok(progress)=>Ok(Some(progress)),Err((error,value))=>{*self.value=Some(value);Err(error)}}})();crate::RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(cursor)=self.cursor.as_mut(){cursor.begin_close();}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<T::Projection>(),depth:1,..Default::default()})}else{nested(cursor.close_demands(body)?)};}
  self.retirement.next_owner_demands::<T::Output>(self.value.is_some(),&None,body)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,ValueError>{let mut work=0;let result=(||{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(cursor)=self.cursor.as_mut(){if cursor.terminal_is_empty(){*self.cursor=None;work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};return {let turn=cursor.close_step(child);work+=turn.work;turn.result}.and_then(|step|admit_retained_clone_close(child,step,cursor.terminal_is_empty(),"immutable original field cancellation")).map(|step|RetainedCloneStep::Progress(step.progress()));}
  let step=if !self.retirement.is_empty(){{let turn=self.retirement.step_granted(grant);work+=turn.work;turn.result}?}else{{let turn=self.retirement.begin_granted(&mut self.value,grant);work+=turn.work;turn.result}?.ok_or_else(invalid)?};Ok(RetainedCloneStep::Progress(step.progress()))
 })();crate::RetainedWorkOutcome{result,work}}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.is_none()&&self.value.is_none()&&self.retirement.is_empty()}
}
impl<T:SnapshotProjection> Drop for ProjectionField<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.cursor.is_none()&&self.value.is_none()&&self.retirement.is_empty(),"immutable field projection retains its original owner");}}
