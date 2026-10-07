//! 🏠️ Artifact document-store and publication authorities.

use crate::DagSnapshot;
use store::PackError;
use crate::standards::v1::subsets::any::io::binary::snapshot::{DagSnapshot};
pub type DagStore = store::ArtifactStore<crate::DagSnapshot, crate::schema::mutations::DagMutation>;

/// 🔐️ Opens a Dag store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `DagStore::new` can be
/// read but never mutated, undone or closed. The app installs the same catalog through
/// `build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_dag_store(envelope: DagEnvelope, actor: protocol::ActorId) -> Result<OwnedDagStore, store::VcsError> {
    let mut store = DagStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<crate::DagSnapshot, crate::schema::mutations::DagMutation>());
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
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Dag document store closes through its exact bounded owners");
        }
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
