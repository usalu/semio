//! 🏠️ Explicit original owner selection for native codec reduction and replay stores.
use super::*;
use semio_framework_os_store_engine_owners::{BoundedArtifactRetirementFactory,DocumentStoreOwnersAdmissionError,lifted_store::ArtifactStoreCursorDisposer};
use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
/// 🧬️ A document supplies its original catalog constructor without an erased global fallback.
pub trait ArtifactCodecOwnerSource<P:ToValue,M:ToValue+FromValue+Mutation<P>> {
 fn demand()->Result<RetirementDemand,ValueError>;
 fn admit(grant:RetainedCloneGrant)->Result<(DocumentStoreOwners<P,M>,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<P,M>>;
}
/// 🧺️ Ordinary codecs retain the framework's bounded typed retirement factories.
pub struct BoundedArtifactCodecOwners;
impl<P,M> ArtifactCodecOwnerSource<P,M> for BoundedArtifactCodecOwners
where P:ToValue+ArtifactPack+semio_framework_value::retirement::RetireOwned+Sync,
M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+semio_framework_value::retirement::RetireOwned+ArtifactCanonicalJsonTree+Sync {
 fn demand()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{capacity_bytes:DocumentStoreOwners::<P,M>::source_birth_bytes::<BoundedArtifactRetirementFactory<P>,BoundedArtifactRetirementFactory<P>,BoundedArtifactRetirementFactory<M>,ArtifactStoreCursorDisposer<P,M>>()?,depth:1,..Default::default()})}
 fn admit(grant:RetainedCloneGrant)->Result<(DocumentStoreOwners<P,M>,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<P,M>>{DocumentStoreOwners::admit_source_constructor(grant,||(BoundedArtifactRetirementFactory::<P>::new(),BoundedArtifactRetirementFactory::<P>::new(),BoundedArtifactRetirementFactory::<M>::new(),ArtifactStoreCursorDisposer::<P,M>::new())).map(|(owners,progress)|(owners.with_batch_digest(),progress))}
}
/// 📏 Quotes and supplies every original work, capacity, release and depth axis.
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
/// 🌱 Constructs only codec metadata; retained semantic payload preparation keeps its caller grants.
pub fn codec_document_owners<P,M,S>()->Result<DocumentStoreOwners<P,M>,ValueError>
where P:ToValue+Send+Sync+'static,M:ToValue+FromValue+Mutation<P>+Send+Sync+'static,S:ArtifactCodecOwnerSource<P,M> {
 let(mut owners,_)=match S::admit(grant(S::demand()?)){Ok(value)=>value,Err(error)=>{if let Some(owners)=error.owners{close_unscheduled_catalog(owners)}return Err(error.error)}};
 for _ in 0..1024 {
  if owners.constructor_is_complete(){return Ok(owners)}
  let demand=match owners.constructor_demands(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES){Ok(demand)=>demand,Err(error)=>{close_unscheduled_catalog(owners);return Err(error)}};
  if let Err((error,_))=owners.admit_constructor(grant(demand)){close_unscheduled_catalog(owners);return Err(error)}
 }
 close_unscheduled_catalog(owners);
 Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"codec original owner constructor did not terminate"))
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
