//! 🏠️ Artifact document-store and publication authorities.

use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError};
use store::os_store::document_store_owners::{
    bounded_artifact_store_owners_shared_semantic_source_demands,
    bounded_artifact_store_owners_with_shared_semantic_preparation,
    DocumentStoreOwners, DocumentStoreOwnersAdmissionError,
};

type Snapshot = crate::NoteSnapshot;
type Mutation = crate::schema::mutations::NoteMutation;
type NoteSemanticProducer = store::ColdMutationApplyFactory<Snapshot, Mutation>;
pub type NoteStoreOwnersAdmission =
    Result<(DocumentStoreOwners<Snapshot, Mutation>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<Snapshot, Mutation>>;

const PRODUCER_CHILDREN: RetainedCloneBirthDemand = RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 };

/// 📏️ Quotes the single Note semantic producer and its exact document owner catalog.
pub fn note_store_owners_source_demands() -> Result<RetirementDemand, ValueError> {
    bounded_artifact_store_owners_shared_semantic_source_demands::<Snapshot, Mutation, NoteSemanticProducer>(PRODUCER_CHILDREN, 0)
}

/// 🔁️ Admits Note publication and replay through the same bounded semantic producer.
pub fn admit_note_store_owners(grant: RetainedCloneGrant) -> NoteStoreOwnersAdmission {
    bounded_artifact_store_owners_with_shared_semantic_preparation(grant, PRODUCER_CHILDREN, 0, NoteSemanticProducer::default)
}

pub type NoteStore = store::ArtifactStore<Snapshot, Mutation>;

/// 🔐️ Opens a Note store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `NoteStore::new` can be
/// read but never mutated, undone or closed. The app's funded catalog supplies these retirement owners through
/// `build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_note_store(envelope: NoteEnvelope, actor: protocol::ActorId) -> Result<OwnedNoteStore, store::VcsError> {
    let mut store = NoteStore::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor).await?;
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Snapshot, Mutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    Ok(OwnedNoteStore(store))
}

/// 🔚 A standalone Note store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedNoteStore(NoteStore);

impl OwnedNoteStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        self.0.close_owned_unscheduled().expect("Note document store closes through its exact bounded owners");
    }
}

impl std::ops::Deref for OwnedNoteStore {
    type Target = NoteStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedNoteStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedNoteStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}

pub type NoteEnvelope = store::os_store::store_schema::ArtifactEnvelope<crate::NoteSnapshot, crate::schema::mutations::NoteMutation>;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
