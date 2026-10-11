//! 🏠️ Artifact document-store and publication authorities.

use store::PackError;
use crate::DagSnapshot;
pub type DagStore = store::ArtifactStore<crate::DagSnapshot, crate::schema::mutations::DagMutation>;

/// 🔐️ Opens a Dag store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `DagStore::new` can be
/// read but never mutated, undone or closed. The app installs the same catalog through
/// `build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_dag_store(envelope: DagEnvelope, actor: protocol::ActorId) -> Result<OwnedDagStore, store::VcsError> {
    let mut store = DagStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(store::funded_bounded_artifact_store_owners::<crate::DagSnapshot, crate::schema::mutations::DagMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    Ok(OwnedDagStore(store))
}

/// 🔚 A standalone Dag store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedDagStore(DagStore);

impl OwnedDagStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        self.0.close_owned_unscheduled().expect("Dag document store closes through its exact bounded owners");
    }
}

impl std::ops::Deref for OwnedDagStore {
    type Target = DagStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedDagStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedDagStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}

pub type DagEnvelope = store::ArtifactEnvelope<crate::DagSnapshot, crate::schema::mutations::DagMutation>;
