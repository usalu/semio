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
 fn place(self,target:&mut MaybeUninit<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Self)>;
}
impl<T:RetireOwned+Sync> ProjectionOutput<T> for T{
 fn projected_value(&self)->&T{self}
 fn projected_value_mut(&mut self)->&mut T{self}
 fn placement_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>()*2,depth:1,..Default::default()}}
 fn place(self,target:&mut MaybeUninit<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Self)>{let d=<Self as ProjectionOutput<T>>::placement_demand();if !retained_clone_grant_admits(grant,d){return Err((invalid("original projection field placement is not funded"),self));}target.write(self);Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
}
impl<T:RetireOwned+Sync> ProjectionOutput<T> for Box<T>{
 fn projected_value(&self)->&T{self.as_ref()}
 fn projected_value_mut(&mut self)->&mut T{self.as_mut()}
 fn placement_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>()*2+size_of::<Self>()*2,release_bytes:size_of::<T>(),depth:1,..Default::default()}}
 fn place(self,target:&mut MaybeUninit<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Self)>{let d=<Self as ProjectionOutput<T>>::placement_demand();if !retained_clone_grant_admits(grant,d){return Err((invalid("original boxed projection placement is not funded"),self));}target.write(*self);Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})}
}
/// 🧭️ Snapshot preparation declares its original immutable projection issuer instead of inheriting a whole-owner clone.
pub trait SnapshotProjection:RetireOwned+Sync+Sized{
 type Output:ProjectionOutput<Self>;
 type Projection:SnapshotProjectionCursor<Self>;
 fn projection_birth_demand()->RetirementDemand;
 fn begin_projection(grant:RetainedCloneGrant)->Result<(Self::Projection,RetainedCloneProgress),ValueError>;
}
/// 🧳️ Original-bound cursors own every staged candidate through refusal, transfer and cancellation.
pub trait SnapshotProjectionCursor<S:SnapshotProjection>:Send+'static{
 fn demands(&self,original:RetainedCloneRef<'_,S>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,original:RetainedCloneRef<'_,S>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(S::Output,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// 🌳️ Fixed immutable roots project original authorities without traversing or cloning variable payloads.
pub trait ImmutableRootProjection:RetireOwned+Sync+Sized{
 fn root_projection_demand(original:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>;
 fn project_root(original:RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>;
}
/// 🎟️ One original root binding and one retained candidate have independently paid output and closure frontiers.
pub struct ImmutableRootCursor<S:ImmutableRootProjection>{binding:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<S>>,retirement:RetainedCloneClose,projected:bool,admitted:bool,closing:bool}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
impl<S:ImmutableRootProjection> SnapshotProjection for S{
 type Output=Self;
 type Projection=ImmutableRootCursor<S>;
 fn projection_birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self::Projection>()*2,depth:1,..Default::default()}}
 fn begin_projection(grant:RetainedCloneGrant)->Result<(Self::Projection,RetainedCloneProgress),ValueError>{let d=Self::projection_birth_demand();if !retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable projection cursor birth is not funded"));}Ok((ImmutableRootCursor{binding:None,output:ManuallyDrop::new(None),retirement:Default::default(),projected:false,admitted:false,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
}
impl<S:ImmutableRootProjection> SnapshotProjectionCursor<S> for ImmutableRootCursor<S>{
 fn demands(&self,original:RetainedCloneRef<'_,S>,_body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.projected{return Err(invalid("immutable projection is closing or already prepared"));}
  if !original.check_binding(&self.binding)?{return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
  S::root_projection_demand(original)
 }
 fn advance(&mut self,original:RetainedCloneRef<'_,S>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demands(original,body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(progress)=original.bind(&mut self.binding,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
  let Some((output,progress))=S::project_root(original,grant)?else{return Err(invalid("funded original root issuer omitted its candidate"));};
  *self.output=Some(output);self.projected=true;
  let progress=admit_retained_clone_progress(grant,progress,"original immutable root projection")?;
  if progress.copied_items==0{return Err(invalid("immutable root issuer omitted original work receipt"));}
  self.admitted=true;Ok(RetainedCloneStep::Complete(progress))
 }
 fn ready(&self)->bool{!self.closing&&self.admitted&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("immutable projection has no admitted output"));}Ok(RetirementDemand{copy_bytes:size_of::<S>()*2,depth:1,..Default::default()})}
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(S,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}Ok(Some((self.output.take().ok_or_else(||invalid("immutable original output disappeared"))?,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("immutable projection closure has not begun"));}self.retirement.next_owner_demands::<S>(self.output.is_some(),&self.binding,body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let step=if !self.retirement.is_empty(){self.retirement.step_granted(grant)?}else if let Some(step)=self.retirement.begin_granted(&mut self.output,grant)?{step}else{crate::retained_clone::close_retained_binding(&mut self.binding,grant)?};
  admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original immutable projection closure")
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.binding.is_none()&&self.retirement.is_empty()}
}
impl<S:ImmutableRootProjection> Drop for ImmutableRootCursor<S>{fn drop(&mut self){assert!(std::thread::panicking()||self.output.is_none()&&self.binding.is_none()&&self.retirement.is_empty(),"immutable projection retains original candidate or binding");}}
impl<T:Send+Sync+'static> ImmutableRootProjection for SealedShared<T>{
 fn root_projection_demand(_original:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Self>()*3,depth:1,..Default::default()})}
 fn project_root(original:RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let d=Self::root_projection_demand(original)?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Self>()*2,..grant};let(value,_)=original.get().try_duplicate(child)?;Ok(Some((value,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
}
macro_rules! scalar_roots{($($kind:ty),*$(,)?)=>{$(impl ImmutableRootProjection for $kind{
 fn root_projection_demand(_original:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Self>()*2,depth:1,..Default::default()})}
 fn project_root(original:RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let d=Self::root_projection_demand(original)?;Ok(retained_clone_grant_admits(grant,d).then(||(*original.get(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 })*};}
scalar_roots!((),bool,char,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize,f32,f64);
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;


#[path="👁️post/🦀️.rs"]
pub mod post;
