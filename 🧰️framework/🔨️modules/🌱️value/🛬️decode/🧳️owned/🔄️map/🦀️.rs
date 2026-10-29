//! 🔄️ Authored native wrappers adopt completed original typed fields without rebuilding their payloads.
use crate::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,retirement::RetireOwned};
use super::{NativeReceive,NativeReceiver,NativeDecodeControl,RetainedCloneRef};
use std::{marker::PhantomData,mem::size_of};
/// 🧬️ Maps only a complete actual typed owner through an infallible authored ownership transfer.
pub trait NativeReceiveMap<T>:RetireOwned{fn from_native(value:T)->Self;}
/// 🪡️ A wrapper retains its defining receiver through the same original input and funded cleanup.
pub struct NativeMappedReceiver<I:?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>{inner:T::Receiver,marker:PhantomData<fn(&I)->U>}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::DepthLimit,"native mapped receiving depth overflow"))?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g}}
impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>> NativeMappedReceiver<I,T,U>{
 /// 📐️ Adds the real wrapper ancestor to its defining child's original constructor quote.
 pub fn birth()->Result<RetirementDemand,ValueError>{nested(T::receive_birth_demand())}
 /// 🎟️ Refusal constructs no child or wrapper owner.
 pub fn begin(g:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let d=Self::birth()?;if !crate::retained_clone::retained_clone_grant_admits(g,d){return Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native wrapper constructor requires its complete original quote"))}let(inner,p)=T::begin_receive(child(g))?;Ok((Self{inner,marker:PhantomData},p))}
}
impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>> NativeReceiver<I,U> for NativeMappedReceiver<I,T,U>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{nested(self.inner.demands(input,body)?)}
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demands(input,body)?;if !crate::retained_clone::retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.inner.advance(input,control,body,child(g))}
 fn output_ready(&self)->bool{self.inner.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{let mut d=nested(self.inner.take_demand()?)?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<U>()).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native wrapper transfer extent overflow"))?;Ok(d)}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(U,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !self.output_ready()||!crate::retained_clone::retained_clone_grant_admits(g,d){return Ok(None)}let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<U>(),..child(g)};let Some((value,mut p))=self.inner.take(child)?else{return Ok(None)};p.copied_bytes+=size_of::<U>();Ok(Some((U::from_native(value),p)))}
 fn begin_close(&mut self){self.inner.begin_close()}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{nested(self.inner.close_demands(body)?)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !crate::retained_clone::retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.inner.close_step(child(g))}
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
