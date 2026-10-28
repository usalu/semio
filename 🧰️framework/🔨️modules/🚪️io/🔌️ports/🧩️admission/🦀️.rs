//! 🧩️ A registered child admits its genuine immutable typed snapshot behind one original-bound owner.
use std::any::Any;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use super::VectorCanvas;
pub type ChildAdmissionSource<'a>=RetainedCloneRef<'a,dyn Any+Send+Sync>;
/// 🪪️ Erasure preserves the issued original source and explicit projected path.
pub fn erase_child_admission_source<T:Any+Send+Sync>(source:RetainedCloneRef<'_,T>,tag:usize)->ChildAdmissionSource<'_>{source.project(tag,|value|value as &(dyn Any+Send+Sync))}
/// 🧷️ The child owner reborrows its actual native snapshot without allocating or decoding another graph.
pub fn project_child_admission_source<T:Any+Send+Sync>(source:ChildAdmissionSource<'_>,tag:usize)->Result<RetainedCloneRef<'_,T>,ValueError>{if !source.get().is::<T>(){return Err(refusal("Child admission source has another native owner type"))}Ok(source.project(tag,|value|value.downcast_ref::<T>().expect("checked child native type")))}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
/// 🧾️ A canvas is exposed only after the original owner's authored dimensions pass admission.
pub struct ChildAdmissionAdvance{pub progress:RetainedCloneProgress,pub done:bool,pub error:Option<&'static str>,pub canvas:Option<VectorCanvas>}
/// 🛂️ Canonical semantic admission belongs to the child, independently of rendering and external decoding.
pub trait EmbeddedChildAdmissionCursorV1:Send{
 fn next_demand(&self,source:ChildAdmissionSource<'_>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,source:ChildAdmissionSource<'_>,grant:RetainedCloneGrant)->Result<ChildAdmissionAdvance,ValueError>;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// 🏷️ Registered editor/member capabilities bind this required owner admission producer.
pub trait EmbeddedChildAdmitterV1:Send+Sync{
 fn birth_demand(&self)->RetirementDemand;
 fn begin(&self,grant:RetainedCloneGrant)->Result<(EmbeddedChildAdmission,RetainedCloneProgress),ValueError>;
}
/// 🧳️ Opaque cursor backing retains its exact physical birth until child closure completes.
pub struct EmbeddedChildAdmission{cursor:Option<Box<dyn EmbeddedChildAdmissionCursorV1>>,bytes:usize,closing:bool}
impl EmbeddedChildAdmission{
 const OVERHEAD:usize=std::mem::size_of::<Self>()*2;
 pub fn constructor_demand<C:EmbeddedChildAdmissionCursorV1+'static>()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<C>()+Self::OVERHEAD,capacity_bytes:std::mem::size_of::<C>(),depth:1,..Default::default()}}
 pub fn admit<C:EmbeddedChildAdmissionCursorV1+'static>(cursor:C,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,C)>{let d=Self::constructor_demand::<C>();if !retained_clone_grant_admits(grant,d){return Err((refusal("Child admission cursor birth is unfunded"),cursor))}Ok((Self{cursor:Some(Box::new(cursor)),bytes:d.capacity_bytes,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))}
 fn nested(d:RetirementDemand)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:d.copy_bytes.checked_add(Self::OVERHEAD).ok_or_else(||refusal("Child admission copy extent overflow"))?,depth:d.depth.checked_add(1).ok_or_else(||refusal("Child admission depth overflow"))?,..d})}
 fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(Self::OVERHEAD),maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
 pub fn next_demand(&self,source:ChildAdmissionSource<'_>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(refusal("Child admission is closing"))}Self::nested(self.cursor.as_ref().ok_or_else(||refusal("Child admission is closed"))?.next_demand(source,body.saturating_sub(Self::OVERHEAD))?)}
 pub fn advance(&mut self,source:ChildAdmissionSource<'_>,grant:RetainedCloneGrant)->Result<ChildAdmissionAdvance,ValueError>{let d=self.next_demand(source,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(ChildAdmissionAdvance{progress:Default::default(),done:false,error:None,canvas:None})}let child=Self::child(grant);let result=self.cursor.as_mut().unwrap().advance(source,child);match result{Ok(mut result)=>{if !result.progress.fits(child){return Err(refusal("Child admission receipt exceeds the original remaining grant").with_retained_progress(RetainedCloneProgress{copied_items:result.progress.copied_items.max(1),copied_bytes:result.progress.copied_bytes.saturating_add(Self::OVERHEAD),..result.progress}))}result.progress.copied_items=result.progress.copied_items.max(1);result.progress.copied_bytes+=Self::OVERHEAD;Ok(result)},Err(error)=>{let mut p=error.retained_progress();p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(Self::OVERHEAD);Err(error.with_retained_progress(p))}}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(cursor)=self.cursor.as_mut(){cursor.begin_close()}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.cursor.as_ref(){None=>Ok(Default::default()),Some(cursor)if cursor.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:self.bytes.checked_add(Self::OVERHEAD).ok_or_else(||refusal("Child admission close extent overflow"))?,release_bytes:self.bytes,depth:1,..Default::default()}),Some(cursor)=>Self::nested(cursor.close_demand(body.saturating_sub(Self::OVERHEAD))?)}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(refusal("Child admission close has not begun"))}let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let Some(cursor)=self.cursor.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};if cursor.terminal_is_empty(){self.cursor=None;let bytes=std::mem::take(&mut self.bytes);return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:bytes,..Default::default()}))}let child=Self::child(grant);match cursor.close_step(child){Ok(step)=>{let mut p=step.progress();if !p.fits(child){return Err(refusal("Child admission close receipt exceeds original grant").with_retained_progress(RetainedCloneProgress{copied_items:p.copied_items.max(1),copied_bytes:p.copied_bytes.saturating_add(Self::OVERHEAD),..p}))}p.copied_items=p.copied_items.max(1);p.copied_bytes+=Self::OVERHEAD;Ok(RetainedCloneStep::Progress(p))},Err(error)=>{let mut p=error.retained_progress();p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(Self::OVERHEAD);Err(error.with_retained_progress(p))}}}
 pub fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.bytes==0}
}
impl Drop for EmbeddedChildAdmission{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Child admission cursor box requires funded paired closure")}}

#[cfg(test)]#[path="🧪️tests.rs"]mod tests;

#[path="📖️provider/🦀️.rs"]pub mod provider;
pub use provider::{ChildAdmissionAddress,ChildAdmissionStamp,ChildAdmissionQualificationV1,ChildAdmissionRead,ChildAdmissionProviderV1};

#[path="🧵️vector/🦀️.rs"]pub mod vector;
pub use vector::{EmbeddedChildVectorSourceV1,VectorGraphAdvance,VectorGraphSourceV1};
