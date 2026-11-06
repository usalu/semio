//! 🎯️ Each authored action selects its actual leaf receiver while retaining the original argument source.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_value::{DslValue,NativeDecodeControl,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,retained_clone::RetainedCloneSource};
use semio_framework_value_native_owned::{NativeReceive,native_decoding::OriginalReceivingPrimary};
use std::{marker::PhantomData,mem::size_of};
/// 🏭️ Static authored leaf authority quotes and constructs only its own concrete receiving frame.
pub struct NativeActionFactory<M>{pub birth:fn()->RetirementDemand,pub admit:fn(RetainedCloneGrant)->Result<(Box<dyn NativeActionReceiving<M>>,RetainedCloneProgress),ValueError>}
impl<M> Copy for NativeActionFactory<M>{}
impl<M> Clone for NativeActionFactory<M>{fn clone(&self)->Self{*self}}
/// 🧬️ Typed action custody preserves the same source until receiving and every initialized child close.
pub trait NativeActionReceiving<M>:Send{
 fn capture_demands(&self)->Result<RetirementDemand,ValueError>;
 fn capture(&mut self,source:&mut Option<RetainedCloneSource<DslValue>>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn receiver_birth_demands(&self)->Result<RetirementDemand,ValueError>;
 fn begin_receiver(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn take_demands(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(M,RetainedCloneProgress)>,ValueError>;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
 fn release_demands(&self)->RetirementDemand;
}
struct Leaf<L:NativeReceive<DslValue>,M>{inner:Option<NativeReceivingOriginal<DslValue,L>>,marker:PhantomData<fn()->M>}
fn invalid()->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native action leaf ownership phase changed")}
fn flat(error:OriginalReceivingPrimary)->ValueError{error.primary.or(error.quote_refusal).or(error.allocation_refusal).unwrap_or_else(invalid)}
impl<M:'static> NativeActionFactory<M>{
 /// 🌱️ A concrete leaf's receiver remains unborn until its separately funded original-source capture.
 pub const fn of<L:NativeReceive<DslValue>+Into<M>+'static>()->Self{Self{birth:birth::<L,M>,admit:admit::<L,M>}}
}
fn birth<L:NativeReceive<DslValue>,M>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Leaf<L,M>>(),capacity_bytes:size_of::<Leaf<L,M>>(),depth:1,..Default::default()}}
fn admit<L:NativeReceive<DslValue>+Into<M>+'static,M:'static>(g:RetainedCloneGrant)->Result<(Box<dyn NativeActionReceiving<M>>,RetainedCloneProgress),ValueError>{let d=birth::<L,M>();if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Err(invalid())}Ok((Box::new(Leaf::<L,M>{inner:None,marker:PhantomData}),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))}
impl<L:NativeReceive<DslValue>+Into<M>+'static,M:'static> NativeActionReceiving<M> for Leaf<L,M>{
 fn capture_demands(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<NativeReceivingOriginal<DslValue,L>>(),depth:1,..Default::default()})}
 fn capture(&mut self,source:&mut Option<RetainedCloneSource<DslValue>>,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.capture_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(Default::default())}let original=source.take().ok_or_else(invalid)?;self.inner=Some(NativeReceivingOriginal::new(original));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
 fn receiver_birth_demands(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn begin_receiver(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{Ok(Default::default())}
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.inner.as_ref().ok_or_else(invalid)?.demands(body)}
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.inner.as_mut().ok_or_else(invalid)?.advance(control,body,g).result.map_err(flat)}
 fn ready(&self)->bool{self.inner.as_ref().is_some_and(|inner|inner.output_ready())}
 fn take_demands(&self)->Result<RetirementDemand,ValueError>{let mut d=self.inner.as_ref().ok_or_else(invalid)?.take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<M>()).ok_or_else(invalid)?;Ok(d)}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(M,RetainedCloneProgress)>,ValueError>{let d=self.take_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(None)}Ok(self.inner.as_mut().ok_or_else(invalid)?.take(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<M>(),..g}).result?.map(|(value,mut p)|{p.copied_bytes+=size_of::<M>();(value.into(),p)}))}
 fn begin_close(&mut self){if let Some(inner)=self.inner.as_mut(){inner.begin_close()}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.inner.as_ref(){Some(inner)=>inner.close_demands(body),None=>Ok(Default::default())}}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match self.inner.as_mut(){Some(inner)=>inner.close_step(g).result,None=>Ok(RetainedCloneStep::Complete(Default::default()))}}
 fn terminal_is_empty(&self)->bool{self.inner.as_ref().is_none_or(|inner|inner.terminal_is_empty())}
 fn release_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl<L:NativeReceive<DslValue>,M> Drop for Leaf<L,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.inner.as_ref().is_none_or(|inner|inner.terminal_is_empty()),"native action leaf requires original funded closure")}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
