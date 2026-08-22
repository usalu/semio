//! @emoji 📄️ `UiSnapshot` / `UiNodeRecord` / `UiNodeId` / `UiPatch` / `UiPatchOp` and the revision
//! model — the flat, id-keyed document every renderer reads and every reconciler writes. No type in
//! this file nests another node inline; a node only ever refers to a child by [`UiNodeId`], which is
//! what keeps the whole surface schema-projectable (see the crate's `📦️glue.rs` header).
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖️Document

//#region 🆔️Ids
/// 🪧️ A render surface address — today's dotted strings, e.g. `"note.play.navigator"`.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SurfaceId(pub String);

impl From<String> for SurfaceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for SurfaceId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
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
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// 🏭️ Per-surface monotonic [`UiNodeId`] source — the only legitimate way to mint one. Never resets
/// and never yields the same id twice, so an id is a stable identity for the lifetime of its surface.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiNodeIdAllocator(u64);

impl UiNodeIdAllocator {
    /// 🏭️ Mints the next unused [`UiNodeId`] for this surface.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn allocate(&mut self) -> UiNodeId {
        let id = UiNodeId(self.0);
        self.0 += 1;
        id
    }
}
//#endregion 🆔️Ids

//#region 🌳️Snapshot
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiNodeRecord {
    pub id: UiNodeId,
    /// 🔑️ Reconciliation key, unique only among this node's own siblings (not surface-wide).
    pub key: String,
    pub component: crate::Component,
    pub layout: crate::LayoutSpec,
    pub style: crate::StyleSpec,
    pub activity: crate::Activity,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<TransitionHint>,
    pub accessibility: crate::AccessibilitySpec,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bindings: Vec<crate::ActionBinding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<crate::MenuRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<UiNodeId>,
}

/// 📸️ A complete, self-contained render of one surface at one revision — the payload a fresh
/// subscriber receives before any [`UiPatch`] applies. `nodes` is an unordered flat table; tree shape
/// lives entirely in `root` plus each record's own `children`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSnapshot {
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: UiNodeId,
    pub nodes: Vec<UiNodeRecord>,
    /// 📐️ Bumped by the layout engine whenever geometry may have changed for reasons a patch does not
    /// itself carry (e.g. a host window resize) — renderers use this to decide whether cached layout
    /// results are still trustworthy without diffing every record.
    pub layout_epoch: u64,
}
//#endregion 🌳️Snapshot

//#region 🩹️Patch
/// 🩹️ One mutation to a single node (or the root pointer) in an already-received [`UiSnapshot`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        children: Vec<UiNodeId>,
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
        bindings: Vec<crate::ActionBinding>,
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

/// 🩹️ A revisioned batch of [`UiPatchOp`]s. Applies atomically: `base_revision` must equal the
/// receiver's current revision or the whole batch is rejected (never partially applied), and success
/// advances the receiver to `revision`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiPatch {
    pub surface: SurfaceId,
    pub base_revision: UiRevision,
    pub revision: UiRevision,
    pub ops: Vec<UiPatchOp>,
}
//#endregion 🩹️Patch

//#region 🗄️SnapshotState
/// 🗄️ The retained receiver-side projection of a surface: current revision, root, and every live
/// node keyed by id. Read-only here by design — the transactional `apply_patch` (base-revision check
/// → shadow map → validate → commit-or-reject-whole) is packet `contract-action`'s in `🦀️limits.rs`;
/// this type only has to be public and constructible for that code to build on top of it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UiSnapshotState {
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub root: Option<UiNodeId>,
    pub nodes: HashMap<UiNodeId, UiNodeRecord>,
}

impl UiSnapshotState {
    /// 🌱️ An empty state for `surface`, at revision zero with no root yet.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(surface: SurfaceId) -> Self {
        Self { surface, revision: UiRevision::default(), root: None, nodes: HashMap::new() }
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

    /// 👶️ A node's direct children, or an empty slice for an unknown or childless id.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn children_of(&self, id: UiNodeId) -> &[UiNodeId] {
        self.nodes.get(&id).map_or(&[], |record| record.children.as_slice())
    }

    /// 🌲️ Depth-first ids rooted at `id` (`id` itself first), via an explicit stack — no recursive
    /// call, matching the flat-table design this whole crate is built around.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn iter_subtree(&self, id: UiNodeId) -> UiSubtreeIter<'_> {
        UiSubtreeIter { state: self, stack: vec![id] }
    }
}

impl From<UiSnapshot> for UiSnapshotState {
    fn from(snapshot: UiSnapshot) -> Self {
        Self { surface: snapshot.surface, revision: snapshot.revision, root: Some(snapshot.root), nodes: snapshot.nodes.into_iter().map(|record| (record.id, record)).collect() }
    }
}

/// 🌲️ Iterator produced by [`UiSnapshotState::iter_subtree`] — preorder, stack-driven, non-recursive.
pub struct UiSubtreeIter<'a> {
    state: &'a UiSnapshotState,
    stack: Vec<UiNodeId>,
}

impl<'a> Iterator for UiSubtreeIter<'a> {
    type Item = UiNodeId;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn next(&mut self) -> Option<UiNodeId> {
        let id = self.stack.pop()?;
        if let Some(record) = self.state.nodes.get(&id) {
            for child in record.children.iter().rev() {
                self.stack.push(*child);
            }
        }
        Some(id)
    }
}
//#endregion 🗄️SnapshotState

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_node_id_allocator_is_monotonic_and_never_reuses() {
        let mut allocator = UiNodeIdAllocator::default();
        let a = allocator.allocate();
        let b = allocator.allocate();
        let c = allocator.allocate();
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
        assert_eq!(revision.next(), UiRevision(5));
    }

    fn leaf_record(id: u64, key: &str) -> UiNodeRecord {
        UiNodeRecord {
            id: UiNodeId(id),
            key: key.into(),
            component: crate::Component::Separator(crate::SeparatorProps {}),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: Vec::new(),
            menu: None,
            children: Vec::new(),
        }
    }

    /// 🌳️ A 3-level nested snapshot (root → container → leaf) round-trips through JSON byte-for-byte:
    /// serialize, deserialize, re-serialize, compare bytes — catching any field silently dropped or
    /// re-defaulted along the way, not merely a structural `==`.
    #[test]
    fn snapshot_three_levels_round_trips_byte_identically() {
        let grandchild = leaf_record(2, "grandchild");
        let mut child = leaf_record(1, "child");
        child.component =
            crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Group, label: Some(crate::Label::from("Group")), description: None, required: None, error: None, default_open: Some(true), drop_overlay: None });
        child.children = vec![UiNodeId(2)];
        let mut root = leaf_record(0, "root");
        root.component = crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None });
        root.children = vec![UiNodeId(1)];

        let snapshot = UiSnapshot { surface: SurfaceId::from("note.play.navigator"), revision: UiRevision(1), root: UiNodeId(0), nodes: vec![root, child, grandchild], layout_epoch: 0 };

        let first = serde_json::to_string(&snapshot).expect("serialize");
        let deserialized: UiSnapshot = serde_json::from_str(&first).expect("deserialize");
        let second = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(first, second);
        assert_eq!(snapshot, deserialized);
    }

    #[test]
    fn snapshot_state_iter_subtree_is_preorder_depth_first() {
        let mut root = leaf_record(0, "root");
        root.children = vec![UiNodeId(1), UiNodeId(2)];
        let mut a = leaf_record(1, "a");
        a.children = vec![UiNodeId(3)];
        let b = leaf_record(2, "b");
        let leaf = leaf_record(3, "leaf");

        let state: UiSnapshotState = UiSnapshot { surface: SurfaceId::from("s"), revision: UiRevision(0), root: UiNodeId(0), nodes: vec![root, a, b, leaf], layout_epoch: 0 }.into();

        let order: Vec<u64> = state.iter_subtree(UiNodeId(0)).map(|id| id.0).collect();
        assert_eq!(order, vec![0, 1, 3, 2]);
        assert_eq!(state.children_of(UiNodeId(1)), &[UiNodeId(3)]);
        assert_eq!(state.root(), Some(UiNodeId(0)));
        assert_eq!(state.revision(), UiRevision(0));
    }

    #[allow(clippy::needless_pass_by_value)]
    fn patch_op_round_trips(op: UiPatchOp) {
        let first = serde_json::to_string(&op).expect("serialize");
        let deserialized: UiPatchOp = serde_json::from_str(&first).expect("deserialize");
        let second = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(first, second);
        assert_eq!(op, deserialized);
    }

    #[test]
    fn every_patch_op_variant_round_trips() {
        patch_op_round_trips(UiPatchOp::Upsert(leaf_record(9, "n")));
        patch_op_round_trips(UiPatchOp::SetComponent { id: UiNodeId(9), component: crate::Component::Text(crate::TextProps { value: crate::Label::from("hi"), emphasize: None, data_attributes: None }) });
        patch_op_round_trips(UiPatchOp::SetLayout { id: UiNodeId(9), layout: Default::default() });
        patch_op_round_trips(UiPatchOp::SetActivity { id: UiNodeId(9), activity: Default::default(), disabled: true });
        patch_op_round_trips(UiPatchOp::SetChildren { id: UiNodeId(9), children: vec![UiNodeId(10), UiNodeId(11)] });
        patch_op_round_trips(UiPatchOp::SetStyle { id: UiNodeId(9), style: crate::StyleSpec { tone: crate::Tone::Danger, ..Default::default() } });
        patch_op_round_trips(UiPatchOp::SetAccessibility { id: UiNodeId(9), accessibility: crate::AccessibilitySpec { shortcut: Some("Ctrl+S".into()), ..Default::default() } });
        patch_op_round_trips(UiPatchOp::SetBindings { id: UiNodeId(9), bindings: vec![crate::ActionBinding { trigger: crate::Trigger::Activate, action: crate::ActionId::v1("scope", "name"), args: None, capability: None }] });
        patch_op_round_trips(UiPatchOp::SetMenu { id: UiNodeId(9), menu: Some(crate::MenuRef { id: "menu".into(), args: None }) });
        patch_op_round_trips(UiPatchOp::SetMenu { id: UiNodeId(9), menu: None });
        patch_op_round_trips(UiPatchOp::Remove { id: UiNodeId(9) });
        patch_op_round_trips(UiPatchOp::SetRoot { id: UiNodeId(9) });
    }

    /// 🪞️ Locks the TypeScript rendering of every wire-critical newtype against what serde actually
    /// puts on the wire. These two are generated by different machinery from the same struct, so
    /// nothing but an assertion keeps them honest. The owned projection records the transparent
    /// wire payloads explicitly so a `u64` cannot silently become a JavaScript `bigint`.
    #[cfg(feature = "typegen")]
    #[test]
    fn wire_critical_newtypes_render_as_their_transparent_payload() {
        let rendered = crate::schema_metadata::render_typescript();
        assert!(rendered.contains("export type SurfaceId = string;"));
        assert!(rendered.contains("export type UiNodeId = number;"));
        assert!(rendered.contains("export type UiRevision = number;"));
        assert!(rendered.contains("export type Label = string;"));
    }
}
//#endregion 🧪️Tests

//#endregion 🔖️Document
