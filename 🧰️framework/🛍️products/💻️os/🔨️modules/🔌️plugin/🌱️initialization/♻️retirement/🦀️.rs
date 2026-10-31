//! 🌱️ The framework-default bounded store initializer: a generic ledger replay that loads the completed envelope's document.
use super::*;
use semio_framework_value::{ToValue,FromValue,retirement::RetireOwned};
/// 🧢️ Captures the completed envelope for the framework-default replay initializer ([`store::ReferenceArtifactStoreReplayInitializer`]): genesis
/// snapshot plus the applied history ledger, folded one operation per turn under the supersessions, cancellable at every turn and
/// closed by exact retirement. An app whose initialization is more than a ledger replay supplies its own authority instead.
pub fn bounded_document_store_initialization_job<P,M,C>(envelope:ArtifactEnvelope<P,M>,schema:&'static str,operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation,actor:ActorId)->ArtifactStoreInitializationJob<P,M>
where P: RetireOwned + store::StoreState,M: RetireOwned + store::ArtifactCanonicalJsonTree + store::StoreMutationFor<P>,C:store::os_store::document_store_owners::DocumentStoreOwnerProducer<P,M> {
 ArtifactStoreInitializationJob::new(Box::new(store::ArtifactStoreReplayInitializer::<P,M,C>::new(envelope,schema,operation,generation,actor)))
}

/// 🎟️ The framework-minted publication token of one document-store replacement: the exact `(operation, generation, base child-content generation)` the replacement was admitted on, carried by the active replacement through the maintenance ladder to `ArtifactApp::validate_document_store_publication`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentStorePublicationToken {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_child_content_generation: u64,
}

/// 🔭️ The live state a publication token is validated against: the replacement operation still retained, the live parent-store generation and the live child-content generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentStoreLivePublication {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    child_content_generation: u64,
}

impl DocumentStoreLivePublication {
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, child_content_generation: u64) -> Self {
        Self { operation, generation, child_content_generation }
    }
}

impl DocumentStorePublicationToken {
    #[doc(hidden)]
    pub fn mint(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, base_child_content_generation: u64) -> Self {
        Self { operation, generation, base_child_content_generation }
    }

    pub fn operation(&self) -> semio_framework_job::OperationId {
        self.operation
    }

    pub fn generation(&self) -> semio_framework_job::Generation {
        self.generation
    }

    pub fn base_child_content_generation(&self) -> u64 {
        self.base_child_content_generation
    }

    /// 🔐️ Fails closed on a publication minted for another operation, on another parent-store generation (ABA) or on another child-content base than the live store.
    pub fn validate_live(&self, live: DocumentStoreLivePublication) -> Result<(), Fault> {
        let refuse = |code: &'static str, message: &'static str| Err(Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new(code), message));
        if self.operation != live.operation {
            return refuse("artifact-store.publication-token-wrong-operation", "document-store publication token was minted for another replacement operation");
        }
        if self.generation != live.generation {
            return refuse("artifact-store.publication-token-stale", "document-store publication token was minted on another generation than the live store");
        }
        if self.base_child_content_generation != live.child_content_generation {
            return refuse("artifact-store.publication-token-wrong-base", "document-store publication token was minted on another child-content base than the live store");
        }
        Ok(())
    }
}

/// 📏️ Quotes the native catalog of a family with explicit cold reference semantics: its shared cold issuer installs the publication and the replay authority.
pub fn cold_replay_document_store_owners_source_demands<P,M>()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>
where P:Clone+semio_framework_value::ToValue+pack::snapshot::ArtifactPack+RetireOwned+Send+Sync+'static,M:ToValue+FromValue+store::Mutation<P>+store::ColdMutation<P>+protocol::OpBinary+protocol::OpText+RetireOwned+store::ArtifactCanonicalJsonTree+Send+Sync+'static{
 store::os_store::document_store_owners::bounded_artifact_store_owners_shared_semantic_source_demands::<P,M,store::ColdMutationApplyFactory<P,M>>(semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0},0)
}
/// 🧊️ Admits the native catalog whose shared cold issuer lets history-bearing documents replay through the framework-default initializer.
pub fn cold_replay_document_store_owners<P,M>(grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<P,M>,semio_framework_value::retained_clone::RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<P,M>>
where P:Clone+semio_framework_value::ToValue+pack::snapshot::ArtifactPack+RetireOwned+Send+Sync+'static,M:ToValue+FromValue+store::Mutation<P>+store::ColdMutation<P>+protocol::OpBinary+protocol::OpText+RetireOwned+store::ArtifactCanonicalJsonTree+Send+Sync+'static{
 store::os_store::document_store_owners::bounded_artifact_store_owners_with_shared_semantic_preparation(grant,semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0},0,store::ColdMutationApplyFactory::<P,M>::default)
}
