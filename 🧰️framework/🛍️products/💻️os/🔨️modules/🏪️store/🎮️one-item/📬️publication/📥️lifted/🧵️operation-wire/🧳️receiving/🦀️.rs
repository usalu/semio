//! 🧳️ Installed native operation codecs erase concrete receiver layout while preserving original typed sources.
use super::ArtifactOperationWire;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,DslValue,RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedOwnedProjection};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeRetirementDecodeControl,NativeReceive};
use semio_framework_pack_json::{OriginalJsonReceiving,ArtifactCanonicalJsonTree};
use std::{mem::size_of,sync::Arc};
/// 📬️ The concrete vocabulary keeps its native parser, typed receiver and issuer until explicit closure.
pub trait ArtifactOriginalOperationReceiving<M>:Send{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn ready(&self)->bool;
 fn owned_handoff_pending(&self)->bool;
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>,ValueError> where M:Sync;
 fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
 fn allocation_bytes(&self)->usize;
}
pub struct OriginalOperationReceivingAdmission<M>{pub batch_retirement:crate::lifted_member_owned_batch::BatchRetirementIssuer<M>,pub demands:fn()->RetirementDemand,pub admit:fn(&mut Option<RetainedOwnedProjection<Vec<u8>>>,&mut Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,&mut NativeDecodeControl<'_>,RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>}
impl<M> Copy for OriginalOperationReceivingAdmission<M>{}
impl<M> Clone for OriginalOperationReceivingAdmission<M>{fn clone(&self)->Self{*self}}
impl<M:NativeReceive<DslValue>+ArtifactCanonicalJsonTree+Send+Sync+'static> ArtifactOriginalOperationReceiving<M> for OriginalJsonReceiving<M>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.advance(control,body,grant)}
 fn ready(&self)->bool{self.ready()}
 fn owned_handoff_pending(&self)->bool{self.owned_handoff_pending()}
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>,ValueError>{self.take(grant)}
 fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.take_owned_demands(body)}
 fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.take_owned_into(output,control,body,grant)}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_step(control,grant)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
 fn allocation_bytes(&self)->usize{size_of::<Self>()}
}
fn demands<M:NativeReceive<DslValue>+Sync>()->RetirementDemand{let d=OriginalJsonReceiving::<M>::constructor_demand();RetirementDemand{copy_bytes:d.copy_bytes+size_of::<Box<dyn ArtifactOriginalOperationReceiving<M>>>(),capacity_bytes:size_of::<OriginalJsonReceiving<M>>(),..d}}
fn admit<M:NativeReceive<DslValue>+ArtifactCanonicalJsonTree+Send+Sync+'static>(wire:&mut Option<RetainedOwnedProjection<Vec<u8>>>,output:&mut Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 if output.is_some()||wire.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original operation receiver requires its retained wire and empty output"))}let d=demands::<M>();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;
 match OriginalJsonReceiving::admit(wire.take().unwrap(),factory,b"op",|original:&M|original,control,grant){Ok((receiver,_))=>{*output=Some(Box::new(receiver));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})},Err((error,original))=>{*wire=Some(original);Err(error)}}
}
impl<M:NativeReceive<DslValue>+ArtifactCanonicalJsonTree+Send+Sync+'static> ArtifactOperationWire<M>{
 /// 🧬️ Registration binds one actual typed receiving implementation to this wire vocabulary.
 pub fn with_original_receiving(mut self)->Self{self.original_receiving=Some(OriginalOperationReceivingAdmission{batch_retirement:crate::lifted_member_owned_batch::BatchRetirementIssuer::typed(),demands:demands::<M>,admit:admit::<M>});self}
}
