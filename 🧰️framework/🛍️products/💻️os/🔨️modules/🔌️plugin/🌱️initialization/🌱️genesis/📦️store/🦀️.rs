//! 📦️ A retained Store slot preserves its original allocation through native assembly and separately paid shell closure.
use super::*;
/// 🧳️ The slot moves a fixed pointer header; its native payload never crosses an aggregate instance move.
pub struct OriginalStoreSlot<P: store::StoreState,M: store::StoreMutationFor<P>>{owner:ManuallyDrop<Option<Box<store::ArtifactStore<P,M>>>>}
impl<P: store::StoreState,M: store::StoreMutationFor<P>> OriginalStoreSlot<P,M>{
 pub fn from_original(owner:Box<store::ArtifactStore<P,M>>)->Self{Self{owner:ManuallyDrop::new(Some(owner))}}
 pub fn terminal_is_empty(&self)->bool{self.owner.is_none()}
 pub fn into_original(mut self)->Box<store::ArtifactStore<P,M>>{self.owner.take().expect("original Store slot was already handed off or closed")}
 pub fn box_close_demands(&self)->RetirementDemand{if self.owner.is_some(){RetirementDemand{copy_bytes:size_of::<Option<Box<store::ArtifactStore<P,M>>>>(),release_bytes:size_of::<store::ArtifactStore<P,M>>(),depth:1,..Default::default()}}else{Default::default()}}
}
impl<P: store::StoreState,M: store::StoreMutationFor<P>> OriginalStoreSlot<P,M>{
 pub fn box_close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.box_close_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(owner)=self.owner.as_ref(){if !owner.close_owned_store_terminal_is_empty(){return Err(invalid("original Store slot cannot release a retained payload"))}self.owner.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}Ok(RetainedCloneStep::Complete(Default::default()))}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{cx.with_original_retirement(|cx|{let Some(owner)=self.owner.as_ref()else{return Ok(true)};if owner.close_owned_store_terminal_is_empty(){let d=self.box_close_demands();close_original_retained_step(cx,d,|grant|self.box_close_step(grant))?;}else{let d=owner.close_owned_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|grant|self.owner.as_mut().unwrap().close_owned_store_step(grant))?;}Ok(self.terminal_is_empty())})}
}
impl<P: store::StoreState,M: store::StoreMutationFor<P>> std::ops::Deref for OriginalStoreSlot<P,M>{type Target=store::ArtifactStore<P,M>;fn deref(&self)->&Self::Target{self.owner.as_deref().expect("live original Store slot remains allocated")}}
impl<P: store::StoreState,M: store::StoreMutationFor<P>> std::ops::DerefMut for OriginalStoreSlot<P,M>{fn deref_mut(&mut self)->&mut Self::Target{self.owner.as_deref_mut().expect("live original Store slot remains allocated")}}
impl<P: store::StoreState,M: store::StoreMutationFor<P>> Drop for OriginalStoreSlot<P,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Store slot requires payload handoff or paid physical shell closure")}}
