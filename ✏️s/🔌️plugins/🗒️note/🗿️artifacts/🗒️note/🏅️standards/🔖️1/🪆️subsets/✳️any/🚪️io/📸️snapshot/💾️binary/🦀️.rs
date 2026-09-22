//! 📦️ Note artifact — binary document surface + laws (constitutional: pack).

use crate::NoteSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `NoteSnapshot` to its binary pack form.
pub fn encode(document: &NoteSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `NoteSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<NoteSnapshot, PackError> {
    <NoteSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🔖️Store
pub type NoteEnvelope = store::ArtifactEnvelope<crate::NoteSnapshot, crate::schema::mutations::NoteMutation>;
pub type NoteStore = store::ArtifactStore<crate::NoteSnapshot, crate::schema::mutations::NoteMutation>;

/// 🔐️ Opens a Note store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `NoteStore::new` can be
/// read but never mutated, undone or closed. The app installs the same catalog through
/// `build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_note_store(envelope: NoteEnvelope) -> Result<OwnedNoteStore, store::VcsError> {
    let mut store = NoteStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<crate::NoteSnapshot, crate::schema::mutations::NoteMutation>());
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
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Note document store closes through its exact bounded owners");
        }
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
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
