//! 📽️ Required immutable snapshot projections retain original roots and separately fund every owner transfer.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,shared_issuance::SealedShared,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits,admit_retained_clone_progress,admit_retained_clone_close}};
use std::mem::{ManuallyDrop,size_of};
pub use std::mem::MaybeUninit;
#[path="📋️field/🦀️.rs"]
mod field;
pub use field::ProjectionField;
#[path="📦️owned/🦀️.rs"]
mod owned;
pub use owned::OwnedFieldProjection;
mod output_placement{pub trait Sealed<T>{}impl<T> Sealed<T> for T{}impl<T> Sealed<T> for Box<T>{}}
/// 🪑️ Only the firstparty value and boxed-value output forms can initialize an admitted field.
/// The private seal fixes placement behavior; refusals return the same owner and leave the destination unchanged.
pub trait ProjectionOutput<T>:output_placement::Sealed<T>+RetireOwned+Sync+Sized{
 fn projected_value(&self)->&T;
 fn projected_value_mut(&mut self)->&mut T;
 fn placement_demand()->RetirementDemand;
 fn place(self,target:&mut MaybeUninit<T>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneProgress,(ValueError,Self)>;
}
impl<T:RetireOwned+Sync> ProjectionOutput<T> for T{
 fn projected_value(&self)->&T{self}
 fn projected_value_mut(&mut self)->&mut T{self}
 fn placement_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>()*2,depth:1,..Default::default()}}
 fn place(self,target:&mut MaybeUninit<T>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneProgress,(ValueError,Self)>{let mut work=0;let result=(||{let d=<Self as ProjectionOutput<T>>::placement_demand();if !retained_clone_grant_admits(grant,d){return Err((invalid("original projection field placement is not funded"),self));}work+=1;target.write(self);Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})})();crate::RetainedWorkOutcome{result,work}}
}
impl<T:RetireOwned+Sync> ProjectionOutput<T> for Box<T>{
 fn projected_value(&self)->&T{self.as_ref()}
 fn projected_value_mut(&mut self)->&mut T{self.as_mut()}
 fn placement_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>()*2+size_of::<Self>()*2,release_bytes:size_of::<T>(),depth:1,..Default::default()}}
 fn place(self,target:&mut MaybeUninit<T>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneProgress,(ValueError,Self)>{let mut work=0;let result=(||{let d=<Self as ProjectionOutput<T>>::placement_demand();if !retained_clone_grant_admits(grant,d){return Err((invalid("original boxed projection placement is not funded"),self));}work+=1;target.write(*self);Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})})();crate::RetainedWorkOutcome{result,work}}
}
/// 🧭️ Snapshot preparation declares its original immutable projection issuer instead of inheriting a whole-owner clone.
pub trait SnapshotProjection:RetireOwned+Sync+Sized{
 type Output:ProjectionOutput<Self>;
 type Projection:SnapshotProjectionCursor<Self>;
 fn projection_birth_demand()->RetirementDemand;
 fn projection_birth_work_demand()->Result<u64,ValueError>;
 fn begin_projection(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress),ValueError>;
}
/// 🧳️ Original-bound cursors own every staged candidate through refusal, transfer and cancellation.
pub trait SnapshotProjectionCursor<S:SnapshotProjection>:Send+'static{
 fn demands(&self,original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 /// ⛽️ Quotes the actual selected immutable-source action before it moves.
 fn advance_work_demand(&self,original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn advance(&mut self,original:RetainedCloneRef<'_,S>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalRetirementFailure>;
 fn ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take_work_demand(&self)->Result<u64,ValueError>;
 fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(S::Output,RetainedCloneProgress)>,ValueError>;
 
 /// ⛽️ Quotes the selected close-start action before changing cursor ownership.
 fn close_started(&self)->bool;
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"projection cursor lacks an original close-start work quote"))}
 /// 🚦️ Starts the quoted closure and reports the actual original work.
 fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"cursor lacks an original close-start physical quote"))}
fn begin_close_paid(&mut self,_grant:crate::RetainedCloneGrant)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,ValueError>{crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"projection cursor lacks an original close-start action")),work:0}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalRetirementFailure>;
 fn terminal_is_empty(&self)->bool;
}
/// 🌳️ Fixed immutable roots project original authorities without traversing or cloning variable payloads.
pub trait ImmutableRootProjection:RetireOwned+Sync+Sized{
 fn root_projection_demand(original:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>;
 fn root_projection_work_demand(original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>;
 fn project_root(original:RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>,ValueError>;
}
/// 🎟️ One original root binding and one retained candidate have independently paid output and closure frontiers.
pub struct ImmutableRootCursor<S:ImmutableRootProjection>{binding:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<S>>,retirement:RetainedCloneClose,projected:bool,admitted:bool,closing:bool}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
impl<S:ImmutableRootProjection> SnapshotProjection for S{
 type Output=Self;
 type Projection=ImmutableRootCursor<S>;
 fn projection_birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self::Projection>()*2,depth:1,..Default::default()}}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn begin_projection(effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self::Projection,RetainedCloneProgress),ValueError>{let grant=effect.grant();let mut work=0;let result=(||{if effect.maximum_work_units()==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"immutable projection birth requires original work"))}let d=Self::projection_birth_demand();if !retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable projection cursor birth is not funded"));}work+=1;Ok((ImmutableRootCursor{binding:None,output:ManuallyDrop::new(None),retirement:Default::default(),projected:false,admitted:false,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}}
}
impl<S:ImmutableRootProjection> SnapshotProjectionCursor<S> for ImmutableRootCursor<S>{
 fn demands(&self,original:RetainedCloneRef<'_,S>,_body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.projected{return Err(invalid("immutable projection is closing or already prepared"));}
  if !original.check_binding(&self.binding)?{return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
  S::root_projection_demand(original)
 }
 fn advance_work_demand(&self,original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.demands(original,body,maximum_work_units)?;if !original.check_binding(&self.binding)?{return Ok(1)}S::root_projection_work_demand(original)}
 fn advance(&mut self,original:RetainedCloneRef<'_,S>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalRetirementFailure>{let grant=effect.grant();let maximum_work_units=effect.maximum_work_units();let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{
  let selected=self.advance_work_demand(original,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedCloneStep::Progress(Default::default()))}let d=self.demands(original,body,maximum_work_units)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(progress)={let turn=original.bind(&mut self.binding,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
  let Some((output,progress))={let turn=S::project_root(original,grant);work+=turn.work;turn.result}?else{return Err(invalid("funded original root issuer omitted its candidate").into());};
  *self.output=Some(output);self.projected=true;
  let progress=admit_retained_clone_progress(grant,progress,"original immutable root projection")?;
  if progress.copied_items==0{return Err(invalid("immutable root issuer omitted original work receipt").into());}
  self.admitted=true;Ok(RetainedCloneStep::Complete(progress))
 })();crate::RetainedWorkOutcome{result,work}}
 fn ready(&self)->bool{!self.closing&&self.admitted&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("immutable projection has no admitted output"));}Ok(RetirementDemand{copy_bytes:size_of::<S>()*2,depth:1,..Default::default()})}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(S,RetainedCloneProgress)>,ValueError>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}work+=1;Ok(Some((self.output.take().ok_or_else(||invalid("immutable original output disappeared"))?,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))})();crate::RetainedWorkOutcome{result,work}}
 
 fn close_started(&self)->bool{self.closing}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
 fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_paid(&mut self,grant:crate::RetainedCloneGrant)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,crate::ValueError>{let mut work=0;let result=(||{let demand=self.begin_close_demand()?;if demand.copy_bytes==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Default::default())}self.closing=true;work=1;Ok(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})})();crate::RetainedWorkOutcome{result,work}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("immutable projection closure has not begun"));}self.retirement.next_owner_demands::<S>(self.output.is_some(),&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid("immutable projection closure has not begun"))}self.retirement.next_owner_work::<S>(self.output.is_some(),&self.binding,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalRetirementFailure>{let grant=effect.grant();let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let step=if !self.retirement.is_empty(){{let turn=self.retirement.step_granted(effect);work+=turn.work;turn.result}?}else if let Some(step)={let turn=self.retirement.begin_granted(&mut self.output,effect);work+=turn.work;turn.result}?{step}else{{let turn=crate::retained_clone::close_retained_binding(&mut self.binding,effect);work+=turn.work;turn.result}?};
  Ok(admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original immutable projection closure")?)
 })();crate::RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.binding.is_none()&&self.retirement.is_empty()}
}
impl<S:ImmutableRootProjection> Drop for ImmutableRootCursor<S>{fn drop(&mut self){assert!(std::thread::panicking()||self.output.is_none()&&self.binding.is_none()&&self.retirement.is_empty(),"immutable projection retains original candidate or binding");}}
impl<T:Send+Sync+'static> ImmutableRootProjection for SealedShared<T>{
 fn root_projection_demand(_original:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Self>()*3,depth:1,..Default::default()})}
 fn root_projection_work_demand(original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>{if original.get().terminal_is_empty(){return Err(invalid("original shared root has transferred"))}Ok(1)}
 fn project_root(original:RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>,ValueError>{let mut work=0;let result=(||{let d=Self::root_projection_demand(original)?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Self>()*2,..grant};if original.get().terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original shared root has transferred"))}let turn=original.get().try_duplicate(child);work+=turn.work;let(value,_)=turn.result?;Ok(Some((value,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))})();crate::RetainedWorkOutcome{result,work}}
}
macro_rules! scalar_roots{($($kind:ty),*$(,)?)=>{$(impl ImmutableRootProjection for $kind{
 fn root_projection_demand(_original:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Self>()*2,depth:1,..Default::default()})}
 fn root_projection_work_demand(_original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>{Ok(1)}
 fn project_root(original:RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>,ValueError>{let mut work=0;let result=(||{let d=Self::root_projection_demand(original)?;Ok(retained_clone_grant_admits(grant,d).then(||{work+=1;(*original.get(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}))})();crate::RetainedWorkOutcome{result,work}}
 })*};}
scalar_roots!((),bool,char,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize,f32,f64);
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;


#[path="👁️post/🦀️.rs"]
pub mod post;

#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod actual_work_tests;
