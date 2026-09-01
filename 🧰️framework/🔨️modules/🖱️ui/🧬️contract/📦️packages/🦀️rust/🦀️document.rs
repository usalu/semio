//! @emoji 📄️ `UiSnapshot` / `UiNodeRecord` / `UiNodeId` / `UiPatch` / `UiPatchOp` and the revision
//! model — the flat, id-keyed document every renderer reads and every reconciler writes. No type in
//! this file nests another node inline; a node only ever refers to a child by [`UiNodeId`], which is
//! what keeps the whole surface schema-projectable (see the crate's `📦️glue.rs` header).
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

//#region 🔖️Document

//#region 🆔️Ids
/// 🪧️ A render surface address — today's dotted strings, e.g. `"note.play.navigator"`.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SurfaceId(pub crate::UiText);

impl AsRef<str> for SurfaceId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for SurfaceId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::UiText::try_from_string(value).map(Self)
    }
}

impl<'a> TryFrom<&'a str> for SurfaceId {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        crate::UiText::try_from_str(value).map(Self).ok_or(value)
    }
}

/// 🔢️ A node's identity within one [`SurfaceId`] — monotonic per surface, never reused, so a stale
/// reference to a removed node is always distinguishable from a fresh node at the same tree position.
///
/// The TypeScript type is pinned to `number`, not a Rust-centric `bigint` projection for `u64`: serde writes
/// this as a plain JSON number, so `JSON.parse` hands JavaScript a `number` at runtime and a `bigint`
/// declaration would be a type that never actually occurs. Ids are per-surface and monotonic, so the
/// 2^53 exact-integer ceiling is unreachable in practice — a surface would have to mint nine
/// quadrillion nodes to reach it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UiNodeId(pub u64);

/// 🔢️ A snapshot's wire revision — advances by one per accepted [`UiPatch`]; a patch whose
/// `base_revision` does not match the receiver's current revision is rejected whole.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UiRevision(pub u64);

impl UiRevision {
    /// ⏭️ The next revision after this one.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn try_next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
}

/// 🏭️ Per-surface monotonic [`UiNodeId`] source — the only legitimate way to mint one. Never resets
/// and never yields the same id twice, so an id is a stable identity for the lifetime of its surface.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiNodeIdAllocator(u64);

impl UiNodeIdAllocator {
    /// 🏭️ Mints the next unused [`UiNodeId`] for this surface.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn try_allocate(&mut self) -> Option<UiNodeId> {
        let id = UiNodeId(self.0);
        self.0 = self.0.checked_add(1)?;
        Some(id)
    }
}
//#endregion 🆔️Ids

//#region 🌳️Snapshot
pub const UI_DOCUMENT_NODES: usize = 128;
pub const UI_NODE_BINDINGS: usize = 32;
pub const UI_DOCUMENT_PATCH_OPS: usize = UI_DOCUMENT_NODES * 9 + 1;
pub type UiNodeChildren = crate::UiFixedList<UiNodeId, UI_DOCUMENT_NODES>;
pub type UiNodeBindings = crate::UiFixedList<crate::ActionBinding, UI_NODE_BINDINGS>;
pub type UiSnapshotNodes = crate::UiFixedList<UiNodeRecord, UI_DOCUMENT_NODES>;
#[path = "../../♻️retirement/📋️patch/🦀️component.rs"]
mod patch_storage;
pub use patch_storage::{UiPatchAllocationError, UiPatchOps, UiPendingPatchOp};
#[path = "../../♻️retirement/📋️patch/📨️pending/📄️whole/🦀️component.rs"]
mod pending_whole_patch;
pub use pending_whole_patch::UiPendingPatch;

#[cfg(test)]
#[path = "../../♻️retirement/📋️patch/🧪️tests/🦀️.rs"]
mod patch_storage_tests;

pub fn credited_bindings(source: &UiNodeBindings) -> Option<UiNodeBindings> {
    let mut bindings = UiNodeBindings::default();
    for binding in source.iter() {
        bindings.try_push(binding.credited_clone()?).ok()?;
    }
    Some(bindings)
}
/// 🎞️ The transient visual emphasis a node is entering — orthogonal to `activity`/`disabled`. A node
/// carrying neither is in its steady state; the renderer clears this once the transition has played.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransitionHint {
    Introducing,
    Celebrating,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_false(value: &bool) -> bool {
    !*value
}

/// 📦️ One row of the flat node table. Never nests another record — children are addressed by
/// [`UiNodeId`] only, so a patch can `Upsert` or `Remove` exactly one node without touching its
/// neighbours or ancestors.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiNodeRecord {
    pub id: UiNodeId,
    /// 🔑️ Reconciliation key, unique only among this node's own siblings (not surface-wide).
    pub key: crate::UiText,
    pub component: crate::Component,
    pub layout: crate::LayoutSpec,
    pub style: crate::StyleSpec,
    pub activity: crate::Activity,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<TransitionHint>,
    pub accessibility: crate::AccessibilitySpec,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub bindings: UiNodeBindings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<crate::MenuRef>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub children: UiNodeChildren,
}

impl UiNodeRecord {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            id: self.id,
            key: self.key.clone(),
            component: self.component.credited_clone()?,
            layout: self.layout.clone(),
            style: self.style,
            activity: self.activity,
            disabled: self.disabled,
            transition: self.transition,
            accessibility: self.accessibility.clone(),
            bindings: credited_bindings(&self.bindings)?,
            menu: match self.menu.as_ref() {
                Some(menu) => Some(menu.credited_clone()?),
                None => None,
            },
            children: self.children.clone(),
        })
    }
}

/// 📸️ A complete, self-contained render of one surface at one revision — the payload a fresh
/// subscriber receives before any [`UiPatch`] applies. `nodes` is an unordered flat table; tree shape
/// lives entirely in `root` plus each record's own `children`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSnapshot {
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: UiNodeId,
    pub nodes: UiSnapshotNodes,
    /// 📐️ Bumped by the layout engine whenever geometry may have changed for reasons a patch does not
    /// itself carry (e.g. a host window resize) — renderers use this to decide whether cached layout
    /// results are still trustworthy without diffing every record.
    pub layout_epoch: u64,
}
//#endregion 🌳️Snapshot

//#region 🩹️Patch
/// 🩹️ One mutation to a single node (or the root pointer) in an already-received [`UiSnapshot`].
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiPatchOp {
    Upsert(UiNodeRecord),
    SetComponent {
        id: UiNodeId,
        component: crate::Component,
    },
    SetLayout {
        id: UiNodeId,
        layout: crate::LayoutSpec,
    },
    SetActivity {
        id: UiNodeId,
        activity: crate::Activity,
        disabled: bool,
    },
    SetChildren {
        id: UiNodeId,
        children: UiNodeChildren,
    },
    SetStyle {
        id: UiNodeId,
        style: crate::StyleSpec,
    },
    SetAccessibility {
        id: UiNodeId,
        accessibility: crate::AccessibilitySpec,
    },
    SetBindings {
        id: UiNodeId,
        bindings: UiNodeBindings,
    },
    SetMenu {
        id: UiNodeId,
        menu: Option<crate::MenuRef>,
    },
    /// 🗑️ Removes the node and its whole orphaned subtree. A struct variant, not a newtype: an
    /// internally-tagged enum cannot serialize a newtype whose payload is not a map, and a bare
    /// `UiNodeId` is an integer — that shape compiles clean and fails only at runtime.
    Remove {
        id: UiNodeId,
    },
    SetRoot {
        id: UiNodeId,
    },
}

impl UiPatchOp {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(match self {
            Self::Upsert(record) => Self::Upsert(record.credited_clone()?),
            Self::SetComponent { id, component } => Self::SetComponent { id: *id, component: component.credited_clone()? },
            Self::SetLayout { id, layout } => Self::SetLayout { id: *id, layout: layout.clone() },
            Self::SetActivity { id, activity, disabled } => Self::SetActivity { id: *id, activity: *activity, disabled: *disabled },
            Self::SetChildren { id, children } => Self::SetChildren { id: *id, children: children.clone() },
            Self::SetStyle { id, style } => Self::SetStyle { id: *id, style: *style },
            Self::SetAccessibility { id, accessibility } => Self::SetAccessibility { id: *id, accessibility: accessibility.clone() },
            Self::SetBindings { id, bindings } => Self::SetBindings { id: *id, bindings: credited_bindings(bindings)? },
            Self::SetMenu { id, menu } => Self::SetMenu {
                id: *id,
                menu: match menu.as_ref() {
                    Some(menu) => Some(menu.credited_clone()?),
                    None => None,
                },
            },
            Self::Remove { id } => Self::Remove { id: *id },
            Self::SetRoot { id } => Self::SetRoot { id: *id },
        })
    }
}

/// 🩹️ A revisioned batch of [`UiPatchOp`]s. Applies atomically: `base_revision` must equal the
/// receiver's current revision or the whole batch is rejected (never partially applied), and success
/// advances the receiver to `revision`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiPatch {
    pub surface: SurfaceId,
    pub base_revision: UiRevision,
    pub revision: UiRevision,
    pub ops: UiPatchOps,
}
//#endregion 🩹️Patch

//#region 🗄️SnapshotState
/// 🗄️ The retained receiver-side projection of a surface: current revision, root, and every live
/// node keyed by id. Read-only here by design — the transactional `apply_patch` (base-revision check
/// → shadow map → validate → commit-or-reject-whole) is packet `contract-action`'s in `🦀️limits.rs`;
/// this type only has to be public and constructible for that code to build on top of it.
#[derive(Debug, Default, PartialEq)]
pub struct UiSnapshotState {
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: Option<UiNodeId>,
    pub nodes: UiNodeTable,
}

#[derive(Debug, Default, PartialEq)]
pub struct UiNodeTable {
    entries: UiSnapshotNodes,
}

impl UiNodeTable {
    pub fn get(&self, id: &UiNodeId) -> Option<&UiNodeRecord> {
        self.entries.iter().find(|record| &record.id == id)
    }

    pub fn get_mut(&mut self, id: &UiNodeId) -> Option<&mut UiNodeRecord> {
        self.entries.iter_mut().find(|record| &record.id == id)
    }

    pub fn try_insert(&mut self, record: UiNodeRecord) -> Result<Option<UiNodeRecord>, UiNodeRecord> {
        if let Some(current) = self.get_mut(&record.id) {
            return Ok(Some(std::mem::replace(current, record)));
        }
        self.entries.try_push(record).map(|()| None)
    }

    pub fn remove(&mut self, id: &UiNodeId) -> Option<UiNodeRecord> {
        let index = self.entries.iter().position(|record| &record.id == id)?;
        self.entries.swap_remove(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&UiNodeId, &UiNodeRecord)> {
        self.entries.iter().map(|record| (&record.id, record))
    }

    pub fn values(&self) -> impl Iterator<Item = &UiNodeRecord> {
        self.entries.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &UiNodeId> {
        self.entries.iter().map(|record| &record.id)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get_index(&self, index: usize) -> Option<&UiNodeRecord> {
        self.entries.get(index)
    }
}

impl UiSnapshotState {
    /// 🌱️ An empty state for `surface`, at revision zero with no root yet.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(surface: SurfaceId) -> Self {
        Self { surface, revision: UiRevision::default(), root: None, nodes: UiNodeTable::default() }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn get(&self, id: UiNodeId) -> Option<&UiNodeRecord> {
        self.nodes.get(&id)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn root(&self) -> Option<UiNodeId> {
        self.root
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn revision(&self) -> UiRevision {
        self.revision
    }

    pub fn children_of(&self, id: UiNodeId) -> impl Iterator<Item = UiNodeId> + '_ {
        self.nodes.get(&id).into_iter().flat_map(|record| record.children.iter().copied())
    }

    pub fn credited_clone(&self) -> Option<Self> {
        let mut nodes = UiNodeTable::default();
        for record in self.nodes.values() {
            nodes.try_insert(record.credited_clone()?).ok()?;
        }
        Some(Self { surface: self.surface.clone(), revision: self.revision, root: self.root, nodes })
    }

    /// 🌲️ Depth-first ids rooted at `id` (`id` itself first), via an explicit stack — no recursive
    /// call, matching the flat-table design this whole crate is built around.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn iter_subtree(&self, id: UiNodeId) -> UiSubtreeIter<'_> {
        let mut stack = crate::UiFixedList::default();
        let rejected = stack.try_push(id).err();
        UiSubtreeIter { state: self, stack, rejected }
    }
}

impl From<UiSnapshot> for UiSnapshotState {
    fn from(snapshot: UiSnapshot) -> Self {
        Self { surface: snapshot.surface, revision: snapshot.revision, root: Some(snapshot.root), nodes: UiNodeTable { entries: snapshot.nodes } }
    }
}

/// 🌲️ Iterator produced by [`UiSnapshotState::iter_subtree`] — preorder, stack-driven, non-recursive.
pub struct UiSubtreeIter<'a> {
    state: &'a UiSnapshotState,
    stack: crate::UiFixedList<UiNodeId, UI_DOCUMENT_NODES>,
    rejected: Option<UiNodeId>,
}

impl<'a> Iterator for UiSubtreeIter<'a> {
    type Item = UiNodeId;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn next(&mut self) -> Option<UiNodeId> {
        if let Some(rejected) = self.rejected.take() {
            return Some(rejected);
        }
        let id = self.stack.pop()?;
        if let Some(record) = self.state.nodes.get(&id) {
            for child in record.children.iter().rev() {
                if let Err(rejected) = self.stack.try_push(*child) {
                    self.rejected = Some(rejected);
                    break;
                }
            }
        }
        Some(id)
    }
}
//#endregion 🗄️SnapshotState

//#region 🪪️DocumentLease
#[path = "../../🎟️resident/🦀️component.rs"]
mod resident;
pub use resident::*;
#[cfg(test)]
#[path = "../../🎟️resident/📄️root/🧪️component.rs"]
mod resident_root_tests;
pub const UI_DOCUMENT_LEASE_SLOTS: usize = UI_RESIDENT_SLOTS;
pub const UI_DOCUMENT_LEASE_ALIASES: u64 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UiDocumentHandle {
    slot: usize,
    epoch: u64,
    generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentBuildError {
    InvalidGeneration,
    ArenaFull,
    StaleHandle,
    DuplicateNode,
    NodeCapacity,
    MissingRoot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentLeaseError {
    StaleHandle,
    Closing,
    AliasCapacity,
    PageCapacity,
    NodeIdentity,
    Contended,
    Poisoned,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiDocumentLeaseHeader {
    pub generation: u64,
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: UiNodeId,
    pub layout_epoch: u64,
    pub node_count: usize,
}

#[derive(Debug)]
pub struct UiDocumentNodePage {
    generation: u64,
    revision: UiRevision,
    index: usize,
    record: UiNodeRecord,
}

impl UiDocumentNodePage {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn revision(&self) -> UiRevision {
        self.revision
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn record(&self) -> &UiNodeRecord {
        &self.record
    }

    pub fn into_record(self) -> UiNodeRecord {
        self.record
    }
}

#[derive(Debug)]
struct UiDocumentSlot {
    resident: Option<UiResidentPermit>,
    epoch: u64,
    generation: u64,
    surface: Option<SurfaceId>,
    revision: UiRevision,
    root: Option<UiNodeId>,
    layout_epoch: u64,
    nodes: UiNodeTable,
    aliases: u64,
    occupied: bool,
    complete: bool,
    retiring: bool,
    retire_scalar: u8,
    retirement_claimed: bool,
    retirement: crate::UiTypedRetirementCursor,
}

impl Default for UiDocumentSlot {
    fn default() -> Self {
        Self::empty()
    }
}

impl UiDocumentSlot {
    const fn empty() -> Self {
        Self { resident: None, epoch: 0, generation: 0, surface: None, revision: UiRevision(0), root: None, layout_epoch: 0, nodes: UiNodeTable { entries: UiSnapshotNodes::empty() }, aliases: 0, occupied: false, complete: false, retiring: false, retire_scalar: 0, retirement_claimed: false, retirement: crate::UiTypedRetirementCursor::empty() }
    }
}

struct UiDocumentArena {
    slots: [UiDocumentSlot; UI_DOCUMENT_LEASE_SLOTS],
    close_cursor: usize,
}

impl Default for UiDocumentArena {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| UiDocumentSlot::default()), close_cursor: 0 }
    }
}

static UI_DOCUMENT_ARENA: Mutex<UiDocumentArena> = Mutex::new(UiDocumentArena { slots: [const { UiDocumentSlot::empty() }; UI_DOCUMENT_LEASE_SLOTS], close_cursor: 0 });

pub(crate) const fn resident_static_backing_bytes() -> usize {
    std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
}

fn with_ui_document_arena<T>(f: impl FnOnce(&mut UiDocumentArena) -> T) -> T {
    let mut arena = UI_DOCUMENT_ARENA.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut arena)
}

impl UiDocumentArena {
    fn slot(&self, handle: UiDocumentHandle) -> Option<&UiDocumentSlot> {
        let slot = self.slots.get(handle.slot)?;
        (slot.occupied && slot.epoch == handle.epoch && slot.generation == handle.generation).then_some(slot)
    }

    fn slot_mut(&mut self, handle: UiDocumentHandle) -> Option<&mut UiDocumentSlot> {
        let slot = self.slots.get_mut(handle.slot)?;
        (slot.occupied && slot.epoch == handle.epoch && slot.generation == handle.generation).then_some(slot)
    }

    fn reserve(&mut self, generation: u64, surface: SurfaceId, revision: UiRevision, root: Option<UiNodeId>, layout_epoch: u64) -> Result<UiDocumentHandle, (UiDocumentBuildError, SurfaceId)> {
        if generation == 0 {
            return Err((UiDocumentBuildError::InvalidGeneration, surface));
        }
        let mut resident = None;
        if !matches!(UiResidentPermit::try_reserve(UiResidentLimits { items: UI_RESIDENT_SURFACE_ITEMS, bytes: UI_RESIDENT_SURFACE_BYTES }, &mut resident, UiResidentPermit::required_reservation_bytes()), Ok(true)) { return Err((UiDocumentBuildError::ArenaFull, surface)); }
        let key = resident.as_ref().unwrap().root_key().unwrap();
        let slot_index = key.slot;
        let epoch = key.epoch;
        if self.slots[slot_index].occupied || self.slots[slot_index].resident.is_some() {
            let _ = resident.as_mut().unwrap().close_step(1);
            return Err((UiDocumentBuildError::StaleHandle, surface));
        }
        self.slots[slot_index] = UiDocumentSlot { resident, epoch, generation, surface: Some(surface), revision, root, layout_epoch, aliases: 1, occupied: true, ..UiDocumentSlot::empty() };
        Ok(UiDocumentHandle { slot: slot_index, epoch, generation })
    }

    fn push(&mut self, handle: UiDocumentHandle, record: UiNodeRecord) -> Result<(), (UiDocumentBuildError, UiNodeRecord)> {
        let Some(slot) = self.slot_mut(handle) else { return Err((UiDocumentBuildError::StaleHandle, record)) };
        if slot.retiring || slot.complete {
            return Err((UiDocumentBuildError::StaleHandle, record));
        }
        if slot.nodes.get(&record.id).is_some() {
            return Err((UiDocumentBuildError::DuplicateNode, record));
        }
        slot.nodes.try_insert(record).map(|_| ()).map_err(|record| (UiDocumentBuildError::NodeCapacity, record))
    }

    fn finish(&mut self, handle: UiDocumentHandle) -> Result<(), UiDocumentBuildError> {
        let Some(slot) = self.slot_mut(handle) else { return Err(UiDocumentBuildError::StaleHandle) };
        if slot.retiring || slot.complete {
            return Err(UiDocumentBuildError::StaleHandle);
        }
        let Some(root) = slot.root else { return Err(UiDocumentBuildError::MissingRoot) };
        if slot.nodes.get(&root).is_none() {
            return Err(UiDocumentBuildError::MissingRoot);
        }
        slot.complete = true;
        Ok(())
    }

    fn alias(&mut self, handle: UiDocumentHandle) -> Result<UiDocumentHandle, UiDocumentLeaseError> {
        let Some(slot) = self.slot_mut(handle) else { return Err(UiDocumentLeaseError::StaleHandle) };
        if slot.retiring || !slot.complete {
            return Err(UiDocumentLeaseError::Closing);
        }
        slot.aliases = slot.aliases.checked_add(1).filter(|aliases| *aliases <= UI_DOCUMENT_LEASE_ALIASES).ok_or(UiDocumentLeaseError::AliasCapacity)?;
        Ok(handle)
    }

    fn header(&self, handle: UiDocumentHandle) -> Result<UiDocumentLeaseHeader, UiDocumentLeaseError> {
        let Some(slot) = self.slot(handle) else { return Err(UiDocumentLeaseError::StaleHandle) };
        if slot.retiring || !slot.complete {
            return Err(UiDocumentLeaseError::Closing);
        }
        let Some(surface) = slot.surface.as_ref() else { return Err(UiDocumentLeaseError::Closing) };
        let Some(root) = slot.root else { return Err(UiDocumentLeaseError::Closing) };
        Ok(UiDocumentLeaseHeader { generation: handle.generation, surface: surface.clone(), revision: slot.revision, root, layout_epoch: slot.layout_epoch, node_count: slot.nodes.len() })
    }

    fn page(&self, handle: UiDocumentHandle, index: usize) -> Result<Option<UiDocumentNodePage>, UiDocumentLeaseError> {
        let Some(slot) = self.slot(handle) else { return Err(UiDocumentLeaseError::StaleHandle) };
        if slot.retiring || !slot.complete {
            return Err(UiDocumentLeaseError::Closing);
        }
        let Some(record) = slot.nodes.get_index(index) else { return Ok(None) };
        let record = record.credited_clone().ok_or(UiDocumentLeaseError::PageCapacity)?;
        Ok(Some(UiDocumentNodePage { generation: handle.generation, revision: slot.revision, index, record }))
    }

    fn release(&mut self, handle: UiDocumentHandle) {
        let Some(slot) = self.slot_mut(handle) else { return };
        if slot.retiring || slot.aliases == 0 {
            return;
        }
        let Some(aliases) = slot.aliases.checked_sub(1) else { return };
        slot.aliases = aliases;
        if aliases == 0 {
            slot.retiring = true;
            slot.complete = false;
        }
    }

    fn active(&self, handle: UiDocumentHandle) -> bool {
        self.slot(handle).is_some()
    }

    fn has_retirement(&self) -> bool {
        self.slots.iter().any(|slot| slot.occupied && slot.retiring && !slot.retirement_claimed)
    }
}

#[derive(Debug)]
pub struct UiDocumentBuilder {
    handle: Option<UiDocumentHandle>,
    released: bool,
    claimed: bool,
}

impl UiDocumentBuilder {
    pub fn try_new(generation: u64, surface: SurfaceId, revision: UiRevision, root: Option<UiNodeId>, layout_epoch: u64) -> Result<Self, (UiDocumentBuildError, SurfaceId)> {
        with_ui_document_arena(|arena| arena.reserve(generation, surface, revision, root, layout_epoch)).map(|handle| Self { handle: Some(handle), released: false, claimed: false })
    }

    pub fn try_push(&mut self, record: UiNodeRecord) -> Result<(), (UiDocumentBuildError, UiNodeRecord)> {
        let Some(handle) = self.handle else { return Err((UiDocumentBuildError::StaleHandle, record)) };
        with_ui_document_arena(|arena| arena.push(handle, record))
    }

    pub fn finish(mut self) -> Result<UiDocumentLease, (UiDocumentBuildError, Self)> {
        let Some(handle) = self.handle else { return Err((UiDocumentBuildError::StaleHandle, self)) };
        if let Err(error) = with_ui_document_arena(|arena| arena.finish(handle)) {
            return Err((error, self));
        }
        self.handle = None;
        Ok(UiDocumentLease { handle: Some(handle), released: false, claimed: false })
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("document builder retirement remains valid").complete
    }

    /// 🧵️ Closes only this exact builder and its typed descendants within one admitted grant.
    pub fn close_step_with_grant(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<crate::UiValueRetirementStep, &'static str> {
        close_document_owner(&mut self.handle, &mut self.released, &mut self.claimed, maximum_items, maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handle.is_none()
    }
}

impl Drop for UiDocumentBuilder {
    fn drop(&mut self) {
        hand_back_document_owner(self.handle.take(), self.released, self.claimed);
    }
}

#[derive(Debug)]
pub struct UiDocumentLease {
    handle: Option<UiDocumentHandle>,
    released: bool,
    claimed: bool,
}

impl UiDocumentLease {
    pub fn generation(&self) -> u64 {
        self.handle.map_or(0, |handle| handle.generation)
    }

    pub fn try_alias(&self) -> Result<Self, UiDocumentLeaseError> {
        let handle = self.handle.ok_or(UiDocumentLeaseError::StaleHandle)?;
        let handle = with_ui_document_arena(|arena| arena.alias(handle))?;
        Ok(Self { handle: Some(handle), released: false, claimed: false })
    }

    pub fn header(&self) -> Result<UiDocumentLeaseHeader, UiDocumentLeaseError> {
        let handle = self.handle.ok_or(UiDocumentLeaseError::StaleHandle)?;
        with_ui_document_arena(|arena| arena.header(handle))
    }

    pub fn read_node_page(&self, index: usize) -> Result<Option<UiDocumentNodePage>, UiDocumentLeaseError> {
        let handle = self.handle.ok_or(UiDocumentLeaseError::StaleHandle)?;
        with_ui_document_arena(|arena| arena.page(handle, index))
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("document lease retirement remains valid").complete
    }

    /// 🧵️ Retains the exact final-alias document until every typed descendant is empty.
    pub fn close_step_with_grant(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<crate::UiValueRetirementStep, &'static str> {
        close_document_owner(&mut self.handle, &mut self.released, &mut self.claimed, maximum_items, maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handle.is_none()
    }
}

impl Drop for UiDocumentLease {
    fn drop(&mut self) {
        hand_back_document_owner(self.handle.take(), self.released, self.claimed);
    }
}

pub fn close_ui_document_page_one() -> bool {
    close_ui_document_page_with_grant(1, 4096).expect("queued document retirement remains valid").complete
}
#[path = "../../♻️retirement/🌳️typed/🦀️document.rs"]
mod typed_retirement;
pub use typed_retirement::close_ui_document_page_with_grant;
use typed_retirement::{close_document_owner, hand_back_document_owner};
#[path = "../../⚖️compare/📄️document/🦀️component.rs"]
mod document_component_compare;
pub use document_component_compare::{UiDocumentComponentCompare, UiDocumentCompareAdmission, UiDocumentCompareError};
#[path = "../../📄️document/🎟️assembly/🦀️component.rs"]
mod document_assembly;
pub use document_assembly::{UiDocumentAssembly, UiDocumentAssemblyError, UiDocumentAssemblyErrorKind, UiDocumentAssemblyProgress, UiDocumentRead, UiDocumentRootIdentity};
//#endregion 🪪️DocumentLease

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../📄️document/🎟️assembly/🧪️tests/🦀️.rs"]
mod document_assembly_tests;
#[cfg(test)]
#[path = "../../⚖️compare/📄️document/🧪️tests/🦀️.rs"]
mod document_component_compare_tests;
#[cfg(test)]
#[path = "../../♻️retirement/📋️patch/📨️pending/📄️whole/🧪️tests/🦀️.rs"]
mod pending_whole_patch_tests;

#[cfg(test)]
mod tests {
    use super::*;

    fn text(value: &str) -> crate::UiText {
        crate::UiText::try_from_str(value).expect("bounded fixture")
    }

    #[test]
    fn ui_node_id_allocator_is_monotonic_and_never_reuses() {
        let mut allocator = UiNodeIdAllocator::default();
        let a = allocator.try_allocate().expect("first id");
        let b = allocator.try_allocate().expect("second id");
        let c = allocator.try_allocate().expect("third id");
        assert_eq!(a, UiNodeId(0));
        assert_eq!(b, UiNodeId(1));
        assert_eq!(c, UiNodeId(2));
        assert!(a.0 < b.0 && b.0 < c.0);
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn ui_revision_advances_by_one() {
        let revision = UiRevision(4);
        assert_eq!(revision.try_next(), Some(UiRevision(5)));
    }

    pub(super) fn leaf_record(id: u64, key: &str) -> UiNodeRecord {
        UiNodeRecord {
            id: UiNodeId(id),
            key: text(key),
            component: crate::Component::Separator(crate::SeparatorProps {}),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: UiNodeBindings::default(),
            menu: None,
            children: UiNodeChildren::default(),
        }
    }

    #[test]
    fn document_lease_publishes_complete_generation_and_closes_incrementally() {
        let surface = SurfaceId::try_from("surface").expect("bounded fixture");
        let mut builder = UiDocumentBuilder::try_new(41, surface, UiRevision(7), Some(UiNodeId(1)), 3).expect("fixed document slot");
        builder.try_push(leaf_record(1, "root")).expect("one node page");
        let mut lease = builder.finish().expect("complete root");
        let header = lease.header().expect("live header");
        assert_eq!((header.generation, header.revision, header.root, header.node_count), (41, UiRevision(7), UiNodeId(1), 1));
        let page = lease.read_node_page(0).expect("live lease").expect("root page");
        assert_eq!((page.generation(), page.revision(), page.index(), page.record().id), (41, UiRevision(7), 0, UiNodeId(1)));
        assert!(lease.read_node_page(1).expect("live lease").is_none());
        let mut alias = lease.try_alias().expect("credited alias");
        assert!(!lease.close_step(), "alias retains the document owner");
        while !alias.close_step() {}
        assert!(matches!(lease.header(), Err(UiDocumentLeaseError::StaleHandle)));
    }

    #[test]
    fn document_builder_returns_exact_max_plus_one_node_owner() {
        let surface = SurfaceId::try_from("full").expect("bounded fixture");
        let mut builder = UiDocumentBuilder::try_new(42, surface, UiRevision(1), Some(UiNodeId(0)), 0).expect("fixed document slot");
        for id in 0..UI_DOCUMENT_NODES as u64 {
            builder.try_push(leaf_record(id, "node")).expect("maximum fits");
        }
        let rejected = leaf_record(UI_DOCUMENT_NODES as u64, "overflow");
        let (error, returned) = builder.try_push(rejected).expect_err("maximum plus one rejects");
        assert_eq!(error, UiDocumentBuildError::NodeCapacity);
        assert_eq!(returned.id, UiNodeId(UI_DOCUMENT_NODES as u64));
        drop(builder);
        while !close_ui_document_page_one() {}
    }

    #[test]
    fn document_builder_close_persists_until_terminal_after_one_step() {
        let surface = SurfaceId::try_from("builder-close").expect("bounded fixture");
        let mut builder = UiDocumentBuilder::try_new(43, surface, UiRevision(1), Some(UiNodeId(0)), 0).expect("fixed document slot");
        for id in 0..4 {
            builder.try_push(leaf_record(id, "node")).expect("fixed node owner");
        }
        assert!(!builder.close_step(), "one close opportunity cannot erase a nonterminal builder");
        assert!(!builder.terminal_is_empty());
        while !builder.close_step() {}
        assert!(builder.terminal_is_empty());
    }

    #[test]
    fn ordinary_lease_drop_releases_then_global_closer_reaches_terminal() {
        let surface = SurfaceId::try_from("lease-drop").expect("bounded fixture");
        let mut builder = UiDocumentBuilder::try_new(44, surface, UiRevision(1), Some(UiNodeId(0)), 0).expect("fixed document slot");
        builder.try_push(leaf_record(0, "node")).expect("fixed node owner");
        drop(builder.finish().expect("published lease"));
        while !close_ui_document_page_one() {}
        assert!(close_ui_document_page_one(), "drop law leaves no active retirement owner");
    }
}
//#endregion 🧪️Tests

//#endregion 🔖️Document
