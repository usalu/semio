//! 🧪️ Feature-gated narrow observers of the document arena, handback table and resident ledger for the companion laws.
use super::*;

/// 🧷️ Opaque identity of one arena slot a lease holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentHandle(UiDocumentHandle);

impl DocumentHandle {
    pub fn slot(self) -> usize { self.0.slot }
}

/// 🔎️ The slot, released and claimed state of a lease.
pub fn lease_state(lease: &UiDocumentLease) -> (Option<DocumentHandle>, bool, bool) { (lease.handle.map(DocumentHandle), lease.released, lease.claimed) }
pub fn handle(lease: &UiDocumentLease) -> Option<DocumentHandle> { lease.handle.map(DocumentHandle) }
/// 🔒️ Holds the process-wide document arena lock until the returned guard drops.
pub fn hold_document_arena() -> impl Drop { UI_DOCUMENT_ARENA.lock().unwrap() }
/// 🔒️ Holds the process-wide resident ledger lock until the returned guard drops.
pub fn hold_resident_ledger() -> impl Drop { resident::RESIDENT_LEDGER.lock().unwrap() }
/// 📖️ Reads the node entries of a live slot; `None` when the handle no longer names one.
pub fn with_slot_entries<R>(handle: DocumentHandle, read: impl FnOnce(&UiSnapshotNodes) -> R) -> Option<R> {
    let arena = UI_DOCUMENT_ARENA.lock().unwrap();
    arena.slot(handle.0).map(|slot| read(&slot.nodes.entries))
}
pub fn first_component_address(handle: DocumentHandle) -> usize {
    let arena = UI_DOCUMENT_ARENA.lock().unwrap();
    &arena.slot(handle.0).unwrap().nodes.get_index(0).unwrap().component as *const _ as usize
}
pub fn slot_present(handle: DocumentHandle) -> bool { with_ui_document_arena(|arena| arena.slot(handle.0).is_some()) }
pub fn slot_active(handle: DocumentHandle) -> bool { with_ui_document_arena(|arena| arena.active(handle.0)) }
pub fn slot_retirement_claimed_with_len(handle: DocumentHandle, len: usize) -> bool { with_ui_document_arena(|arena| arena.slot(handle.0).is_some_and(|slot| slot.retirement_claimed && slot.nodes.len() == len)) }
pub fn handback_pending(handle: DocumentHandle) -> bool { typed_retirement::DOCUMENT_HANDBACKS.has_slot_pending(handle.0.slot) }
/// 🎟️ The slot and epoch a resident permit holds.
pub fn permit_key(permit: &UiResidentPermit) -> Option<(usize, u64)> { permit.key.map(|key| (key.slot, key.epoch)) }
pub fn leaf_record(id: u64, key: &str) -> UiNodeRecord { UiNodeRecord::with_component(UiNodeId(id), crate::UiText::try_from_str(key).unwrap(), crate::Component::Separator(crate::SeparatorProps {})) }
