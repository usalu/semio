//! 🌱️ Window partition initialization retains its exact envelope and declared native owner catalog.
use super::*;

/// 🏭️ Native Window replay uses the catalog declared by the original owner.
pub struct WindowConfigInitializationOwners<O>(std::marker::PhantomData<fn()->O>);
impl<O:WindowConfigOwner> store::os_store::document_store_owners::DocumentStoreOwnerProducer<O::State,O::Mutation> for WindowConfigInitializationOwners<O>{
 fn source_demands()->Result<RetirementDemand,ValueError>{
  let mut demand=store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<O::State,O::Mutation>()?;let child=O::preparation_source_birth();
  demand.capacity_bytes=demand.capacity_bytes.checked_add(child.capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Window original initialization catalog extent overflow"))?;
  demand.copy_bytes=2*size_of::<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>();demand.depth=demand.depth.max(child.depth);Ok(demand)
 }
 fn admit_sources(grant:RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<O::State,O::Mutation>>{O::build_store_owners(grant)}
}
/// 🎟️ The mounted command pumps this initializer with its original StepContext and closes it through the same authority.
pub type WindowConfigInitialization<O> = store::ArtifactStoreReplayInitializer<<O as WindowConfigOwner>::State,<O as WindowConfigOwner>::Mutation,WindowConfigInitializationOwners<O>>;
/// 📥️ Captures the original admitted Window envelope without constructing a cold Store or a substitute caller.
pub fn begin_window_config_initialization<O:WindowConfigOwner>(envelope:store::os_store::store_schema::ArtifactEnvelope<O::State,O::Mutation>,operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation,actor:protocol::ActorId)->WindowConfigInitialization<O>{WindowConfigInitialization::new(envelope,O::SCHEMA,operation,generation,actor)}

#[path="🎟️birth/🦀️.rs"]
mod birth;
pub use birth::WindowConfigPartitionBirth;
