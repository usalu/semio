//! 🪟️ Heterogeneous Window registration composes original typed sources without duplicating their owner algorithms.
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,admit_original_construction,close_original_retained_step,OriginalConstructionDemand};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_plugin_window_config::{WindowConfigOwner,WindowConfigOwnerRegistry,OriginalWindowOwnerRegistration};
use semio_plugin_window_transient::window_transient::{WindowTransientOwner,WindowTransientOwnerRegistry,OriginalWindowTransientOwnerRegistration};
use std::{mem::{size_of,ManuallyDrop},marker::PhantomData};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn frame_birth<S>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<S>(),capacity_bytes:size_of::<S>(),depth:1,..Default::default()}}
fn frame_release<S>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<S>>>(),release_bytes:size_of::<S>(),depth:1,..Default::default()}}
fn release_frame<S>(source:&mut Option<Box<S>>,cx:&mut StepContext<'_>,d:RetirementDemand)->Result<(),OriginalTurnFailure>{
 if source.is_none()||d.release_bytes!=size_of::<S>()||d.copy_bytes<size_of::<Option<Box<S>>>(){return Err(invalid("original Window source release changed its actual frame").into())}
 close_original_retained_step(cx,d,|_|{*source=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}})?;Ok(())
}
/// 🎟️ One existing concrete owner admission supplies its real registry, original source and closure.
pub trait OriginalWindowAdmission:Send+'static{
 type Registry:Send+'static;
 fn registry_identity(registry:&Self::Registry)->u64;
 fn admit(registry:&mut Self::Registry,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure> where Self:Sized;
 fn advance(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn close_step(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
 fn boxed_release_demands()->RetirementDemand where Self:Sized;
}
impl<O:WindowConfigOwner> OriginalWindowAdmission for OriginalWindowOwnerRegistration<O>{
 type Registry=WindowConfigOwnerRegistry;
 fn registry_identity(registry:&Self::Registry)->u64{registry.lifetime}
 fn admit(registry:&mut Self::Registry,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{registry.admit_original_owner::<O>(cx,output)}
 fn advance(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.step(registry,cx)}
 fn close_step(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{OriginalWindowOwnerRegistration::<O>::close_step(self,registry,cx)}
 fn terminal_is_empty(&self)->bool{OriginalWindowOwnerRegistration::<O>::terminal_is_empty(self)}
 fn boxed_release_demands()->RetirementDemand{OriginalWindowOwnerRegistration::<O>::boxed_release_demands()}
}
impl<O:WindowTransientOwner> OriginalWindowAdmission for OriginalWindowTransientOwnerRegistration<O>{
 type Registry=WindowTransientOwnerRegistry;
 fn registry_identity(registry:&Self::Registry)->u64{registry.original_lifetime()}
 fn admit(registry:&mut Self::Registry,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{registry.admit_original_owner::<O>(cx,output)}
 fn advance(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.step(registry,cx)}
 fn close_step(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{OriginalWindowTransientOwnerRegistration::<O>::close_step(self,registry,cx)}
 fn terminal_is_empty(&self)->bool{OriginalWindowTransientOwnerRegistration::<O>::terminal_is_empty(self)}
 fn boxed_release_demands()->RetirementDemand{OriginalWindowTransientOwnerRegistration::<O>::boxed_release_demands()}
}
/// 🧺️ An authored type sequence retains each heterogeneous source through complete registration and cancellation.
pub trait OriginalWindowSet:Send+'static{
 type Registry:Send+'static;
 fn admit(registry:&mut Self::Registry,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure> where Self:Sized;
 fn advance(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn ready(&self)->bool;
 fn close_step(&mut self,registry:Option<&mut Self::Registry>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
 fn boxed_release_demands()->RetirementDemand where Self:Sized;
}
/// 📚️ The head registers before its tail, while donated registries remain exclusively with the parent.
pub struct OriginalWindowSequence<H:OriginalWindowAdmission,T:OriginalWindowSet<Registry=H::Registry>>{
 binding:OriginalSourceBinding,registry:(usize,u64),head:ManuallyDrop<Option<Box<H>>>,tail:ManuallyDrop<Option<Box<T>>>,phase:u8,closing:bool,
}
impl<H:OriginalWindowAdmission,T:OriginalWindowSet<Registry=H::Registry>> OriginalWindowSequence<H,T>{
 fn validate(&self,registry:Option<&H::Registry>,cx:&StepContext<'_>)->Result<(),ValueError>{if !self.binding.matches(cx)||registry.is_some_and(|registry|self.registry!=(registry as*const _ as usize,H::registry_identity(registry))){return Err(invalid("original Window sequence changed its caller or registry"))}Ok(())}
 fn set_phase(&mut self,next:u8,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let d=RetirementDemand{copy_bytes:1,depth:1,..Default::default()};let mut paid=None;admit_original_construction(cx,&mut paid,construction(d),|_|{self.phase=next;RetainedWorkOutcome{result:Ok(((),receipt(d))),work:1}})?;Ok(())}
}
impl<H:OriginalWindowAdmission,T:OriginalWindowSet<Registry=H::Registry>> OriginalWindowSet for OriginalWindowSequence<H,T>{
 type Registry=H::Registry;
 fn admit(registry:&mut Self::Registry,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(invalid("original Window sequence recipient is occupied").into())}
  let identity=H::registry_identity(registry);if identity==0{return Err(invalid("original Window registry identity is absent").into())}
  let d=frame_birth::<Self>();let binding=OriginalSourceBinding::borrow(cx);let registry=(registry as*mut _ as usize,identity);
  admit_original_construction(cx,output,construction(d),|_|RetainedWorkOutcome{result:Ok((Box::new(Self{binding,registry,head:ManuallyDrop::new(None),tail:ManuallyDrop::new(None),phase:0,closing:false}),receipt(d))),work:1})?;Ok(output.is_some())
 }
 fn advance(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(Some(registry),cx)?;if self.closing{return Err(invalid("original Window sequence is closing").into())}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  match self.phase{
   0=>if let Some(head)=self.head.as_mut(){if head.terminal_is_empty(){self.set_phase(1,cx)?;}else{head.advance(registry,cx)?;}}else{H::admit(registry,cx,&mut self.head)?;},
   1=>if let Some(tail)=self.tail.as_mut(){if tail.ready(){self.set_phase(2,cx)?;}else{tail.advance(registry,cx)?;}}else{T::admit(registry,cx,&mut self.tail)?;},
   2=>{},_=>return Err(invalid("original Window sequence phase is invalid").into()),
  }Ok(self.ready())
 }
 fn ready(&self)->bool{self.phase==2&&!self.closing}
 fn close_step(&mut self,mut registry:Option<&mut Self::Registry>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(registry.as_deref(),cx)?;
  cx.with_original_retirement(|cx|{
   if !self.closing{let d=RetirementDemand{copy_bytes:1,depth:1,..Default::default()};close_original_retained_step(cx,d,|_|{self.closing=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}})?;return Ok(false)}
   if let Some(head)=self.head.as_mut(){if head.terminal_is_empty(){release_frame(&mut self.head,cx,H::boxed_release_demands())?;}else{head.close_step(registry.as_deref_mut().ok_or_else(||invalid("original Window head lost its undonated registry"))?,cx)?;}return Ok(false)}
   if let Some(tail)=self.tail.as_mut(){if tail.terminal_is_empty(){release_frame(&mut self.tail,cx,T::boxed_release_demands())?;}else{tail.close_step(registry.as_deref_mut(),cx)?;}return Ok(false)}
   Ok(self.terminal_is_empty())
  })
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.head.is_none()&&self.tail.is_none()}
 fn boxed_release_demands()->RetirementDemand{frame_release::<Self>()}
}
impl<H:OriginalWindowAdmission,T:OriginalWindowSet<Registry=H::Registry>> Drop for OriginalWindowSequence<H,T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Window sequence retains every source until paid closure")}}
/// 📭️ Explicitly ends a typed Window sequence without inventing an owner or a registry.
pub struct OriginalNoWindows<R:Send+'static>{binding:OriginalSourceBinding,registry:usize,closing:bool,marker:PhantomData<fn()->R>}
impl<R:Send+'static> OriginalWindowSet for OriginalNoWindows<R>{
 type Registry=R;
 fn admit(registry:&mut R,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{if output.is_some(){return Err(invalid("original empty Window source recipient is occupied").into())}let d=frame_birth::<Self>();let binding=OriginalSourceBinding::borrow(cx);let registry=registry as*mut _ as usize;admit_original_construction(cx,output,construction(d),|_|RetainedWorkOutcome{result:Ok((Box::new(Self{binding,registry,closing:false,marker:PhantomData}),receipt(d))),work:1})?;Ok(output.is_some())}
 fn advance(&mut self,registry:&mut R,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx)||self.registry!=registry as*mut _ as usize||self.closing{return Err(invalid("original empty Window source changed its caller or registry").into())}Ok(true)}
 fn ready(&self)->bool{!self.closing}
 fn close_step(&mut self,registry:Option<&mut R>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx)||registry.is_some_and(|registry|self.registry!=registry as*mut _ as usize){return Err(invalid("original empty Window closure changed its caller or registry").into())}if !self.closing{let d=RetirementDemand{copy_bytes:1,depth:1,..Default::default()};close_original_retained_step(cx,d,|_|{self.closing=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}})?;}Ok(self.closing)}
 fn terminal_is_empty(&self)->bool{self.closing}
 fn boxed_release_demands()->RetirementDemand{frame_release::<Self>()}
}
impl<R:Send+'static> Drop for OriginalNoWindows<R>{fn drop(&mut self){assert!(std::thread::panicking()||self.closing,"original empty Window source requires paid terminal closure")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
