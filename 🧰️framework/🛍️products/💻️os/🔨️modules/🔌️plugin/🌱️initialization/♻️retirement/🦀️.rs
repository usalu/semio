//! 🌱️ The framework-default bounded store initializer: a generic ledger replay that loads the completed envelope's document.
use super::*;
use semio_framework_value::{ToValue,FromValue,retirement::RetireOwned};
/// 🌱️ Captures the completed envelope for the framework-default replay initializer ([`store::ReferenceArtifactStoreReplayInitializer`]): genesis
/// snapshot plus the applied history ledger, folded one operation per turn under the supersessions, cancellable at every turn and
/// closed by exact retirement. An app whose initialization is more than a ledger replay supplies its own authority instead.
pub fn bounded_document_store_initialization_job<P,M,C>(envelope:ArtifactEnvelope<P,M>,schema:&'static str,operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation,actor:ActorId)->ArtifactStoreInitializationJob<P,M>
where P:ToValue+RetireOwned+Send+Sync+'static,M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+RetireOwned+Send+store::ArtifactCanonicalJsonTree+'static,C:store::os_store::document_store_owners::DocumentStoreOwnerProducer<P,M>{
 ArtifactStoreInitializationJob::new(Box::new(store::ArtifactStoreReplayInitializer::<P,M,C>::new(envelope,schema,operation,generation,actor)))
}
