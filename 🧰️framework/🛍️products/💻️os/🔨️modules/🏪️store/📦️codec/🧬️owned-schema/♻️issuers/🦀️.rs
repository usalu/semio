//! 🧷️ Native envelope issuers retain their actual shared registry until final physical closure.
use super::*;
use semio_framework_value::RetirementDemand;
use std::mem::ManuallyDrop;
/// 🎟️ Defining registries expose only their exact unique empty physical release.
pub trait ArtifactEnvelopeRegistryRelease:Sized{
 fn empty_release_demands(owner:&Arc<Self>)->Result<Option<RetirementDemand>,ValueError>;
 fn release_empty_original(owner:&mut Option<Arc<Self>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
}
impl<P,M> ArtifactEnvelopeRegistryRelease for ArtifactEnvelopeFieldDecoderRegistry<P,M>{
 fn empty_release_demands(owner:&Arc<Self>)->Result<Option<RetirementDemand>,ValueError>{Self::empty_release_demands(owner)}
 fn release_empty_original(owner:&mut Option<Arc<Self>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::release_empty_original(owner,grant)}
}
impl<P,M> ArtifactEnvelopeRegistryRelease for ArtifactEnvelopeCompletedRecordRegistry<P,M>{
 fn empty_release_demands(owner:&Arc<Self>)->Result<Option<RetirementDemand>,ValueError>{Self::empty_release_demands(owner)}
 fn release_empty_original(owner:&mut Option<Arc<Self>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::release_empty_original(owner,grant)}
}
/// 🧳️ Borrowing the original issuer never substitutes another registry during final closure.
pub struct ArtifactEnvelopeRegistryOwner<R:ArtifactEnvelopeRegistryRelease>{original:ManuallyDrop<Option<Arc<R>>>}
impl<R:ArtifactEnvelopeRegistryRelease> ArtifactEnvelopeRegistryOwner<R>{
 pub fn from_original(original:Arc<R>)->Self{Self{original:ManuallyDrop::new(Some(original))}}
 pub fn get(&self)->Option<&Arc<R>>{self.original.as_ref()}
 pub fn custody_is_closed(&self)->bool{self.original.is_none()}
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{self.original.as_ref().map_or(Ok(Default::default()),|owner|R::empty_release_demands(owner).map(Option::unwrap_or_default))}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{R::release_empty_original(&mut self.original,grant)}
}
impl<R:ArtifactEnvelopeRegistryRelease> std::ops::Deref for ArtifactEnvelopeRegistryOwner<R>{type Target=Arc<R>;fn deref(&self)->&Self::Target{self.original.as_ref().expect("closed envelope issuer cannot admit or borrow another decoder")}}
impl<R:ArtifactEnvelopeRegistryRelease> Drop for ArtifactEnvelopeRegistryOwner<R>{fn drop(&mut self){assert!(std::thread::panicking()||self.custody_is_closed(),"original envelope issuer must release its exact backing before its containing instance")}}
