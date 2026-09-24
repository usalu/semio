//! 📦️ DAG artifact — binary codec (`impl store::ArtifactPack for DagSnapshot`) over the framework
//! `semio_framework_artifact_infinite_dag::DagSnapshot` record, plus the artifact-facing `encode`/`decode`
//! wrappers and the pack↔dsl equivalence law.

use crate::DagSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `DagSnapshot` to its binary pack form.
pub fn encode(document: &DagSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `DagSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<DagSnapshot, PackError> {
    <DagSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🔖️ArtifactPack
/// 📦️ Packs the owned graph through the framework `DagSnapshot` derived record (the text facet's twin),
/// so the kind's pack-schema identity is that record's spec and the child owner is re-derived on decode.
impl store::ArtifactPack for DagSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        self.validate().map_err(PackError::Schema)?;
        semio_framework_artifact_infinite_dag::DagSnapshot::from(self).encode_pack_with(options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let snapshot: Self = <semio_framework_artifact_infinite_dag::DagSnapshot as store::ArtifactPack>::decode_pack_with(bytes, options)?.into();
        snapshot.validate().map_err(PackError::Schema)?;
        Ok(snapshot)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        <semio_framework_artifact_infinite_dag::DagSnapshot as store::ArtifactPack>::record_spec()
    }
}
//#endregion 🔖️ArtifactPack

//#region 🔖️Store
pub type DagEnvelope = store::ArtifactEnvelope<crate::DagSnapshot, crate::schema::mutations::DagMutation>;
pub type DagStore = store::ArtifactStore<crate::DagSnapshot, crate::schema::mutations::DagMutation>;

/// 🔐️ Opens a Dag store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `DagStore::new` can be
/// read but never mutated, undone or closed. The app installs the same catalog through
/// `build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_dag_store(envelope: DagEnvelope) -> Result<OwnedDagStore, store::VcsError> {
    let mut store = DagStore::new(envelope).await?;
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
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
