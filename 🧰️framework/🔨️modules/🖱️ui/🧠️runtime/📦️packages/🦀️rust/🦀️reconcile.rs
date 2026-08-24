//! @emoji ♻️ Keyed reconciliation of a [`crate::ComponentTree`] into a minimal transactional
//! [`ui_contract::UiPatch`] — the conversion from the builder-side, id-less, recursive tree
//! [`crate::present`] produces into the flat, id-keyed [`ui_contract`] document every renderer reads.
//!
//! The one property that makes the emitted patches worth anything: identity comes from
//! **`(parent, key)`, never from position**. A [`SurfaceReconciler`] retains its own shadow copy of
//! what the receiver has (mirroring [`ui_contract::UiSnapshotState`]) plus a `(parent, key) → id`
//! index, so a node keeps its [`ui_contract::UiNodeId`] across reorders, insertions and removals of
//! its siblings — which is exactly the property that lets renderer-side state (scroll offset, focus,
//! a DOM node, a GPU cache entry) survive a re-present instead of being torn down and rebuilt every
//! frame the way the old `PatchTracker` full-body-`Replace` stub forced.
//!
//! The frame path uses [`SurfaceReconcileCursor`] internally: presentation discovery, identity
//! allocation, postorder record diffing, and stale-tree removal each advance one node at a time.
//! Plain synchronous calls are cooperative scheduler slices, not a hidden run-to-completion frame.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::mem::{size_of, take};
use std::sync::{LazyLock, Mutex};

//#region 🔖️Identity

/// 🔑️ A node's reconciliation identity: which parent it hangs under (`None` only for the root, which
/// has no parent) plus its own sibling `key`. Two [`crate::TreeNode`]s presented on different frames
/// with the same identity are the SAME node as far as reconciliation is concerned, regardless of what
/// position either occupied among its siblings — this is the one invariant every other rule here
/// exists to preserve.
type NodeIdentity = (Option<ui_contract::UiNodeId>, String);

/// 🔑️ `node`'s identity under `parent`, as looked up in / inserted into [`SurfaceReconciler::key_index`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
fn identity_of(parent: Option<ui_contract::UiNodeId>, node: &crate::TreeNode) -> NodeIdentity {
    (parent, node.key.clone())
}

/// 🚨️ Panics naming the first duplicate sibling key found in `children`. [`crate::ComponentTree`]'s
/// own constructor already asserts this on every level of a tree built through `TreeNode::with_children`
/// / `ComponentTree::new`, but `ComponentTree { root }`'s field is `pub`, so a caller can hand this
/// reconciler a tree that skipped that constructor entirely — this is therefore genuine defense in
/// depth, not a redundant re-check, and it is what makes a duplicate key a loud authoring-bug panic
/// here too rather than one key silently shadowing the other during matching.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
fn assert_unique_child_keys(parent: ui_contract::UiNodeId, children: &[crate::TreeNode]) {
    let mut seen: HashSet<&str> = HashSet::with_capacity(children.len());
    for child in children {
        assert!(seen.insert(child.key.as_str()), "🚫️ duplicate sibling key {:?} under parent {parent:?} — reconciliation keys must be unique among siblings", child.key);
    }
}

//#endregion 🔖️Identity

//#region 🔖️Reconciler

/// ♻️ Keyed differ for one render surface. Owns a shadow copy of what the receiver has (`retained`,
/// `root`) plus the `(parent, key) → id` index (`key_index`) that carries every node's identity across
/// frames, and the monotonic `allocator` that mints an id for a node the first time it is ever seen.
/// A completed reconcile is the only place any of these four change together.
#[derive(Debug)]
pub struct SurfaceReconciler {
    surface: ui_contract::SurfaceId,
    revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    retained: HashMap<ui_contract::UiNodeId, ui_contract::UiNodeRecord>,
    key_index: HashMap<NodeIdentity, ui_contract::UiNodeId>,
    root: Option<ui_contract::UiNodeId>,
    retire_scalar: u8,
    persistent_credit: Option<SurfaceReconcileCredit>,
}

impl SurfaceReconciler {
    /// 🌱️ A reconciler for `surface` with no retained state yet — the next [`Self::reconcile`] call
    /// necessarily emits a full `SetRoot` plus one `Upsert` per node, exactly as [`Self::mark_rejected`]
    /// arranges for an existing reconciler to do again.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(surface: impl Into<ui_contract::SurfaceId>) -> Self {
        Self { surface: surface.into(), revision: ui_contract::UiRevision::default(), allocator: ui_contract::UiNodeIdAllocator::default(), retained: HashMap::new(), key_index: HashMap::new(), root: None, retire_scalar: 0, persistent_credit: None }
    }

    /// ♻️ Diffs `tree` against this reconciler's retained state, mutating that state to match and
    /// returning the minimal [`ui_contract::UiPatch`] that carries the difference — or `None` when
    /// `tree` is structurally and semantically identical to what was last presented, so an idle surface
    /// produces no wire traffic at all. `base_revision` is the revision the receiver is assumed to be
    /// at; `revision` is one past it — this reconciler never emits a gap or a repeat.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub fn reconcile(&mut self, tree: &crate::ComponentTree) -> Option<ui_contract::UiPatch> {
        let mut ops = Vec::new();
        let previous_root = self.root;
        let new_root_id = self.diff_node(None, &tree.root, &mut ops);

        if previous_root != Some(new_root_id) {
            if let Some(stale_root) = previous_root {
                self.remove_subtree(None, stale_root, &mut ops);
            }
            ops.push(ui_contract::UiPatchOp::SetRoot { id: new_root_id });
            self.root = Some(new_root_id);
        }

        if ops.is_empty() {
            return None;
        }
        let base_revision = self.revision;
        self.revision = self.revision.next();
        Some(ui_contract::UiPatch { surface: self.surface.clone(), base_revision, revision: self.revision, ops })
    }

    /// 📸️ The complete current state as a fresh [`ui_contract::UiSnapshot`] — what a new subscriber
    /// receives instead of a patch stream. `root` falls back to [`ui_contract::UiNodeId::default`] when
    /// nothing has ever been reconciled yet; `nodes` is then empty too, so that sentinel never resolves
    /// to a real record.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn snapshot(&self) -> ui_contract::UiSnapshot {
        ui_contract::UiSnapshot { surface: self.surface.clone(), revision: self.revision, root: self.root.unwrap_or_default(), nodes: self.retained.values().cloned().collect(), layout_epoch: 0 }
    }

    /// 🧬️ Returns the retained scalar revision without cloning the retained document.
    pub fn revision(&self) -> ui_contract::UiRevision {
        self.revision
    }

    /// 🪪️ Returns the reconciler's surface identity without allocating a second owner.
    pub fn surface(&self) -> &ui_contract::SurfaceId {
        &self.surface
    }

    /// 🔄️ Forces the next [`Self::reconcile`] to emit a full re-send — the recovery path for the
    /// existing `patch-rejected` wire event. Drops every retained node and resets the assumed receiver
    /// revision to zero (mirroring the fresh, empty document a rejection leaves the receiver at); the
    /// [`ui_contract::UiNodeIdAllocator`] is deliberately left untouched, so the re-sent nodes get IDs
    /// that continue monotonically rather than reusing any id a stale renderer reference might still
    /// name.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub fn mark_rejected(&mut self) {
        self.retained.clear();
        self.key_index.clear();
        self.root = None;
        self.revision = ui_contract::UiRevision::default();
    }

    fn retire_one(&mut self) -> bool {
        if let Some(id) = self.retained.keys().next().copied() {
            self.retained.remove(&id);
            return false;
        }
        if let Some(identity) = self.key_index.keys().next().cloned() {
            self.key_index.remove(&identity);
            return false;
        }
        match self.retire_scalar {
            0 => self.root = None,
            1 => self.surface.0.clear(),
            2 => self.revision = ui_contract::UiRevision::default(),
            3 => self.allocator = ui_contract::UiNodeIdAllocator::default(),
            4 => {
                if let Some(credit) = self.persistent_credit.take() {
                    release_surface_reconcile(credit);
                }
            }
            _ => return true,
        }
        self.retire_scalar += 1;
        self.retire_scalar >= 5
    }
}

//#endregion 🔖️Reconciler

//#region ⏭️ResumableReconcile

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceReconcileStage {
    TraversePresentation,
    AllocateIdentities,
    DiffRecords,
    RemoveStale,
    Finalize,
}

#[derive(Debug)]
pub(crate) enum SurfaceReconcileStep {
    Yield { nodes: usize, bytes: usize },
    Complete { reconciler: SurfaceReconciler, patch: Option<ui_contract::UiPatch> },
    Fault(SurfaceReconcileFault),
}

/// 🚧️ Fixed admission bounds for one retained reconciliation authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceReconcileLimits {
    pub max_nodes: usize,
    pub max_items: usize,
    pub max_bytes: usize,
    pub max_identifier_bytes: usize,
}

impl Default for SurfaceReconcileLimits {
    fn default() -> Self {
        Self { max_nodes: 4_096, max_items: 32_769, max_bytes: 2 * 1_024 * 1_024, max_identifier_bytes: 256 }
    }
}

/// 📏️ Credits observed while advancing one retained reconciliation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceReconcileUsage {
    pub nodes: usize,
    pub items: usize,
    pub bytes: usize,
}

impl SurfaceReconcileUsage {
    fn include(&mut self, nodes: usize, items: usize, bytes: usize) -> bool {
        let Some(next_nodes) = self.nodes.checked_add(nodes) else { return false };
        let Some(next_items) = self.items.checked_add(items) else { return false };
        let Some(next_bytes) = self.bytes.checked_add(bytes) else { return false };
        self.nodes = next_nodes;
        self.items = next_items;
        self.bytes = next_bytes;
        true
    }

    fn fits(self, limits: SurfaceReconcileLimits) -> bool {
        self.nodes <= limits.max_nodes && self.items <= limits.max_items && self.bytes <= limits.max_bytes
    }
}

/// 🚫️ Typed terminal reason retaining the exact in-progress authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileFault {
    DuplicateSiblingKey,
    IdentifierBytes { actual: usize, max: usize },
    Credits { usage: SurfaceReconcileUsage, limits: SurfaceReconcileLimits },
    PageBytes { actual: usize, max: usize },
    ValueDepth { actual: usize, max: usize },
    StaleGeneration { expected: u64, actual: u64 },
    Cancelled,
}

struct FlatPresentedNode {
    parent: Option<usize>,
    node: crate::TreeNode,
    child_ids: Vec<ui_contract::UiNodeId>,
}

struct PresentationFrame {
    index: usize,
    children: std::vec::IntoIter<crate::TreeNode>,
}

struct RemovalFrame {
    id: ui_contract::UiNodeId,
    next_child: usize,
}

const SURFACE_RECONCILE_VALUE_DEPTH: usize = 64;
const SURFACE_RECONCILE_SEMANTIC_COPIES: usize = 3;

#[derive(Clone, Copy)]
enum SurfaceSemanticValueFrame {
    Value(*const ui_contract::UiValue),
    List { values: *const Vec<ui_contract::UiValue>, entry: usize, admitted: bool },
    Map { values: *const BTreeMap<String, ui_contract::UiValue>, entry: usize, admitted: bool },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SurfaceSemanticUsage {
    items: usize,
    bytes: usize,
}

enum SurfaceSemanticCensusStep {
    Progress(SurfaceSemanticUsage),
    Complete,
    Fault(SurfaceReconcileFault),
}

struct SurfaceSemanticCensusCursor {
    field: u8,
    container: u8,
    entry: usize,
    binding: usize,
    action: u8,
    data_attribute: u8,
    string_byte: usize,
    depth: usize,
    value_stack: [Option<SurfaceSemanticValueFrame>; SURFACE_RECONCILE_VALUE_DEPTH],
}

impl Default for SurfaceSemanticCensusCursor {
    fn default() -> Self {
        Self { field: 0, container: 0, entry: 0, binding: 0, action: 0, data_attribute: 0, string_byte: 0, depth: 0, value_stack: [None; SURFACE_RECONCILE_VALUE_DEPTH] }
    }
}

impl SurfaceSemanticCensusCursor {
    fn owner(&mut self, bytes: usize) -> SurfaceSemanticUsage {
        self.string_byte = bytes.saturating_mul(SURFACE_RECONCILE_SEMANTIC_COPIES);
        SurfaceSemanticUsage { items: SURFACE_RECONCILE_SEMANTIC_COPIES, bytes: 0 }
    }

    fn backing<T>(&self, capacity: usize) -> SurfaceSemanticUsage {
        SurfaceSemanticUsage { items: SURFACE_RECONCILE_SEMANTIC_COPIES, bytes: capacity.saturating_mul(size_of::<T>()).saturating_mul(SURFACE_RECONCILE_SEMANTIC_COPIES) }
    }

    fn push_value(&mut self, value: &ui_contract::UiValue) -> Result<(), SurfaceReconcileFault> {
        if self.depth == SURFACE_RECONCILE_VALUE_DEPTH {
            return Err(SurfaceReconcileFault::ValueDepth { actual: self.depth.saturating_add(1), max: SURFACE_RECONCILE_VALUE_DEPTH });
        }
        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::Value(std::ptr::from_ref(value)));
        self.depth += 1;
        Ok(())
    }

    fn value_step(&mut self) -> Option<SurfaceSemanticCensusStep> {
        let frame = self.value_stack.get_mut(self.depth.checked_sub(1)?)?.take()?;
        match frame {
            SurfaceSemanticValueFrame::Value(value) => {
                self.depth -= 1;
                let value = unsafe { &*value };
                match value {
                    ui_contract::UiValue::Null | ui_contract::UiValue::Bool(_) | ui_contract::UiValue::Number(_) => Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())),
                    ui_contract::UiValue::Text(value) => Some(SurfaceSemanticCensusStep::Progress(self.owner(value.capacity()))),
                    ui_contract::UiValue::List(values) => {
                        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::List { values: std::ptr::from_ref(values), entry: 0, admitted: false });
                        self.depth += 1;
                        Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()))
                    }
                    ui_contract::UiValue::Map(values) => {
                        self.value_stack[self.depth] = Some(SurfaceSemanticValueFrame::Map { values: std::ptr::from_ref(values), entry: 0, admitted: false });
                        self.depth += 1;
                        Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()))
                    }
                }
            }
            SurfaceSemanticValueFrame::List { values, mut entry, admitted } => {
                let values = unsafe { &*values };
                if !admitted {
                    self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::List { values: std::ptr::from_ref(values), entry, admitted: true });
                    return Some(SurfaceSemanticCensusStep::Progress(self.backing::<ui_contract::UiValue>(values.capacity())));
                }
                let Some(value) = values.get(entry) else {
                    self.depth -= 1;
                    return Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()));
                };
                entry += 1;
                self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::List { values: std::ptr::from_ref(values), entry, admitted });
                Some(match self.push_value(value) {
                    Ok(()) => SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()),
                    Err(fault) => SurfaceSemanticCensusStep::Fault(fault),
                })
            }
            SurfaceSemanticValueFrame::Map { values, mut entry, admitted } => {
                let values = unsafe { &*values };
                if !admitted {
                    self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::Map { values: std::ptr::from_ref(values), entry, admitted: true });
                    return Some(SurfaceSemanticCensusStep::Progress(self.backing::<(String, ui_contract::UiValue)>(values.len())));
                }
                let Some((key, value)) = values.iter().nth(entry) else {
                    self.depth -= 1;
                    return Some(SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default()));
                };
                entry += 1;
                self.value_stack[self.depth - 1] = Some(SurfaceSemanticValueFrame::Map { values: std::ptr::from_ref(values), entry, admitted });
                if let Err(fault) = self.push_value(value) {
                    return Some(SurfaceSemanticCensusStep::Fault(fault));
                }
                Some(SurfaceSemanticCensusStep::Progress(self.owner(key.capacity())))
            }
        }
    }

    fn bindings_step(&mut self, bindings: &[ui_contract::ActionBinding]) -> SurfaceSemanticCensusStep {
        let Some(binding) = bindings.get(self.binding) else {
            self.binding = 0;
            self.action = 0;
            return SurfaceSemanticCensusStep::Complete;
        };
        let usage = match self.action {
            0 => self.owner(binding.action.scope.capacity()),
            1 => self.owner(binding.action.name.capacity()),
            2 => {
                if let Some(args) = &binding.args {
                    if let Err(fault) = self.push_value(args) {
                        return SurfaceSemanticCensusStep::Fault(fault);
                    }
                }
                SurfaceSemanticUsage::default()
            }
            3 => binding.capability.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())),
            _ => {
                self.binding += 1;
                self.action = 0;
                return SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default());
            }
        };
        self.action += 1;
        SurfaceSemanticCensusStep::Progress(usage)
    }

    fn component_step(&mut self, component: &ui_contract::Component) -> SurfaceSemanticCensusStep {
        use ui_contract::Component::*;
        let progress = |usage| SurfaceSemanticCensusStep::Progress(usage);
        match component {
            Container(props) => {
                let usage = match self.container {
                    0 => props.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity())),
                    1 => props.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())),
                    2 => props.error.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())),
                    3 => props.drop_overlay.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.title.0.capacity())),
                    4 => props.drop_overlay.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.hint.0.capacity())),
                    5 => props.drop_overlay.as_ref().and_then(|value| value.accept.as_ref()).map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Text(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.owner(props.value.0.capacity()))
                }
                1 => {
                    self.container = 2;
                    progress(props.data_attributes.as_ref().map_or_else(SurfaceSemanticUsage::default, |values| self.backing::<(String, String)>(values.capacity())))
                }
                2 => {
                    let Some((key, value)) = props.data_attributes.as_ref().and_then(|values| values.iter().nth(self.entry)) else { return SurfaceSemanticCensusStep::Complete };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.owner(key.capacity())
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.owner(value.capacity())
                    };
                    progress(usage)
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Button(props) => {
                let usage = match self.container {
                    0 => self.owner(props.icon.capacity()),
                    1 => self.owner(props.label.0.capacity()),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Separator(_) | NumberStepper(_) => SurfaceSemanticCensusStep::Complete,
            Input(props) => {
                let usage = match self.container {
                    0 => self.owner(props.value.capacity()),
                    1 => props.placeholder.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity())),
                    2 => props.commit.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())),
                    3 => props.accept.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Select(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.owner(props.value.capacity()))
                }
                1 => {
                    self.container = 2;
                    progress(self.backing::<ui_contract::SelectItem>(props.items.capacity()))
                }
                2 => {
                    let Some(item) = props.items.get(self.entry) else {
                        self.container = 3;
                        return progress(SurfaceSemanticUsage::default());
                    };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.owner(item.value.capacity())
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.owner(item.label.0.capacity())
                    };
                    progress(usage)
                }
                3 => {
                    self.container = 4;
                    progress(props.placeholder.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity())))
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Toggle(props) => {
                let usage = match self.container {
                    0 => self.owner(props.icon.capacity()),
                    1 => props.text.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity())),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            KeyValueList(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.backing::<ui_contract::KeyValueEntry>(props.entries.capacity()))
                }
                1 => {
                    let Some(entry) = props.entries.get(self.entry) else { return SurfaceSemanticCensusStep::Complete };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.owner(entry.label.0.capacity())
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.owner(entry.value.capacity())
                    };
                    progress(usage)
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Slider(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.unit.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            Ring(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(self.owner(props.orb_id.capacity()))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            IconSelect(props) => {
                let usage = match self.container {
                    0 => self.owner(props.value.capacity()),
                    1 => self.owner(props.classifier_kind.capacity()),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Tree(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.interaction_domain.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            TreeSection(props) => {
                self.container += 1;
                if self.container == 1 {
                    progress(props.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity())))
                } else {
                    SurfaceSemanticCensusStep::Complete
                }
            }
            TreeItem(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.owner(props.label.0.capacity()))
                }
                1 => {
                    self.container = 2;
                    progress(props.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())))
                }
                2 => {
                    self.container = 3;
                    progress(props.icon.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())))
                }
                3 => {
                    self.container = 4;
                    progress(props.drag_data.as_ref().map_or_else(SurfaceSemanticUsage::default, |values| self.backing::<(String, String)>(values.capacity())))
                }
                4 => {
                    let Some((key, value)) = props.drag_data.as_ref().and_then(|values| values.iter().nth(self.entry)) else {
                        self.container = 5;
                        self.entry = 0;
                        return progress(SurfaceSemanticUsage::default());
                    };
                    let usage = if self.data_attribute == 0 {
                        self.data_attribute = 1;
                        self.owner(key.capacity())
                    } else {
                        self.data_attribute = 0;
                        self.entry += 1;
                        self.owner(value.capacity())
                    };
                    progress(usage)
                }
                5 => {
                    self.container = 6;
                    progress(self.backing::<ui_contract::RowAction>(props.row_actions.capacity()))
                }
                6 => {
                    let Some(action) = props.row_actions.get(self.entry) else { return SurfaceSemanticCensusStep::Complete };
                    let step = match self.data_attribute {
                        0 => progress(self.owner(action.icon.capacity())),
                        1 => progress(action.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity()))),
                        _ => self.bindings_step(std::slice::from_ref(&action.action)),
                    };
                    if matches!(step, SurfaceSemanticCensusStep::Complete) {
                        self.data_attribute = 0;
                        self.entry += 1;
                        return progress(SurfaceSemanticUsage::default());
                    }
                    self.data_attribute += 1;
                    step
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Image(props) => {
                let usage = match self.container {
                    0 => self.owner(props.src.capacity()),
                    1 => props.alt.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity())),
                    _ => return SurfaceSemanticCensusStep::Complete,
                };
                self.container += 1;
                progress(usage)
            }
            Surface(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.owner(props.doc_schema.capacity()))
                }
                1 => {
                    self.container = 2;
                    progress(self.backing::<u8>(props.doc.bytes.capacity()))
                }
                2 => {
                    self.container = 3;
                    progress(self.backing::<ui_contract::ActionBinding>(props.bindings.capacity()))
                }
                3 => self.bindings_step(&props.bindings),
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Extension(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.owner(props.extension.capacity()))
                }
                1 => {
                    self.container = 2;
                    match self.push_value(&props.props) {
                        Ok(()) => progress(SurfaceSemanticUsage::default()),
                        Err(fault) => SurfaceSemanticCensusStep::Fault(fault),
                    }
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
        }
    }

    fn step(&mut self, node: &crate::TreeNode) -> SurfaceSemanticCensusStep {
        if self.string_byte > 0 {
            self.string_byte -= 1;
            return SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage { items: 0, bytes: 1 });
        }
        if let Some(step) = self.value_step() {
            return step;
        }
        match self.field {
            0 => {
                self.field = 1;
                SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage { items: SURFACE_RECONCILE_SEMANTIC_COPIES, bytes: size_of::<crate::TreeNode>().saturating_mul(SURFACE_RECONCILE_SEMANTIC_COPIES) })
            }
            1 => {
                self.field = 2;
                SurfaceSemanticCensusStep::Progress(self.owner(node.key.capacity()))
            }
            2 => match self.component_step(&node.component) {
                SurfaceSemanticCensusStep::Complete => {
                    self.field = 3;
                    self.container = 0;
                    self.entry = 0;
                    self.binding = 0;
                    self.action = 0;
                    self.data_attribute = 0;
                    SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
                }
                step => step,
            },
            3 => {
                self.field = 4;
                SurfaceSemanticCensusStep::Progress(node.accessibility.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity())))
            }
            4 => {
                self.field = 5;
                SurfaceSemanticCensusStep::Progress(node.accessibility.description.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.0.capacity())))
            }
            5 => {
                self.field = 6;
                SurfaceSemanticCensusStep::Progress(node.accessibility.shortcut.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.owner(value.capacity())))
            }
            6 => {
                self.field = 7;
                SurfaceSemanticCensusStep::Progress(self.backing::<ui_contract::ActionBinding>(node.bindings.capacity()))
            }
            7 => match self.bindings_step(&node.bindings) {
                SurfaceSemanticCensusStep::Complete => {
                    self.field = 8;
                    SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
                }
                step => step,
            },
            8 => {
                self.field = 9;
                SurfaceSemanticCensusStep::Progress(node.menu.as_ref().map_or_else(SurfaceSemanticUsage::default, |menu| self.owner(menu.id.capacity())))
            }
            9 => {
                self.field = 10;
                if let Some(args) = node.menu.as_ref().and_then(|menu| menu.args.as_ref()) {
                    if let Err(fault) = self.push_value(args) {
                        return SurfaceSemanticCensusStep::Fault(fault);
                    }
                }
                SurfaceSemanticCensusStep::Progress(SurfaceSemanticUsage::default())
            }
            10 => {
                self.field = 11;
                SurfaceSemanticCensusStep::Progress(self.backing::<crate::TreeNode>(node.children.capacity()))
            }
            _ => SurfaceSemanticCensusStep::Complete,
        }
    }
}

/// ⏭️ Persistent one-node-at-a-time traversal and keyed differ for a single presented surface.
/// The retained reconciler is read-only until `Complete`; abandoning this cursor therefore abandons
/// every candidate identity, record, operation, and revision together.
pub(crate) struct SurfaceReconcileCursor {
    stage: SurfaceReconcileStage,
    surface: ui_contract::SurfaceId,
    base_revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    old_root: Option<ui_contract::UiNodeId>,
    pending_root: Option<crate::TreeNode>,
    traversal: Vec<PresentationFrame>,
    flat: Vec<FlatPresentedNode>,
    postorder: Vec<usize>,
    seen: HashSet<(Option<usize>, String)>,
    ids: Vec<ui_contract::UiNodeId>,
    allocate_index: usize,
    diff_index: usize,
    new_retained: HashMap<ui_contract::UiNodeId, ui_contract::UiNodeRecord>,
    new_key_index: HashMap<NodeIdentity, ui_contract::UiNodeId>,
    remove_next: Option<ui_contract::UiNodeId>,
    removal: Vec<RemovalFrame>,
    ops: Vec<ui_contract::UiPatchOp>,
    limits: SurfaceReconcileLimits,
    usage: SurfaceReconcileUsage,
    held_node: Option<(Option<usize>, Box<crate::TreeNode>)>,
    semantic_census: Option<SurfaceSemanticCensusCursor>,
    semantic_usage: SurfaceSemanticUsage,
    fault: Option<SurfaceReconcileFault>,
}

impl SurfaceReconcileCursor {
    pub(crate) fn new(tree: crate::ComponentTree, current: &SurfaceReconciler) -> Self {
        Self::new_with_limits(tree, current, SurfaceReconcileLimits::default())
    }

    pub(crate) fn new_with_limits(tree: crate::ComponentTree, current: &SurfaceReconciler, limits: SurfaceReconcileLimits) -> Self {
        Self {
            stage: SurfaceReconcileStage::TraversePresentation,
            surface: current.surface.clone(),
            base_revision: current.revision,
            allocator: current.allocator.clone(),
            old_root: current.root,
            pending_root: Some(tree.root),
            traversal: Vec::new(),
            flat: Vec::new(),
            postorder: Vec::new(),
            seen: HashSet::new(),
            ids: Vec::new(),
            allocate_index: 0,
            diff_index: 0,
            new_retained: HashMap::new(),
            new_key_index: HashMap::new(),
            remove_next: None,
            removal: Vec::new(),
            ops: Vec::new(),
            limits,
            usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: 0 },
            held_node: None,
            semantic_census: None,
            semantic_usage: SurfaceSemanticUsage::default(),
            fault: None,
        }
    }

    pub(crate) fn step(&mut self, current: &SurfaceReconciler) -> SurfaceReconcileStep {
        if let Some(fault) = self.fault.clone() {
            return SurfaceReconcileStep::Fault(fault);
        }
        if current.revision != self.base_revision {
            let fault = SurfaceReconcileFault::StaleGeneration { expected: self.base_revision.0, actual: current.revision.0 };
            self.fault = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        let step = match self.stage {
            SurfaceReconcileStage::TraversePresentation => {
                if self.held_node.is_none() {
                    if let Some(root) = self.pending_root.take() {
                        self.held_node = Some((None, Box::new(root)));
                        self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                        self.semantic_usage = SurfaceSemanticUsage::default();
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 };
                    }
                    if let Some(frame) = self.traversal.last_mut() {
                        if let Some(child) = frame.children.next() {
                            self.held_node = Some((Some(frame.index), Box::new(child)));
                            self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                            self.semantic_usage = SurfaceSemanticUsage::default();
                            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<usize>() };
                        }
                        let complete = self.traversal.pop().expect("traversal frame existed");
                        self.postorder.push(complete.index);
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<usize>() };
                    }
                    self.stage = SurfaceReconcileStage::AllocateIdentities;
                    return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 };
                }
                if let Some((parent, node)) = self.held_node.as_ref() {
                    let key_bytes = node.key.len();
                    if key_bytes > self.limits.max_identifier_bytes {
                        let fault = SurfaceReconcileFault::IdentifierBytes { actual: key_bytes, max: self.limits.max_identifier_bytes };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if self.flat.len() >= self.limits.max_nodes {
                        let usage = SurfaceReconcileUsage { nodes: self.flat.len().saturating_add(1), items: self.usage.items, bytes: self.usage.bytes };
                        let fault = SurfaceReconcileFault::Credits { usage, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    let semantic = self.semantic_census.as_mut().expect("held node owns its semantic census").step(node);
                    match semantic {
                        SurfaceSemanticCensusStep::Progress(delta) => {
                            self.semantic_usage.items = self.semantic_usage.items.saturating_add(delta.items);
                            self.semantic_usage.bytes = self.semantic_usage.bytes.saturating_add(delta.bytes);
                            let node_page_bytes = size_of::<FlatPresentedNode>().saturating_add(self.semantic_usage.bytes);
                            let projected = SurfaceReconcileUsage { nodes: self.usage.nodes.saturating_add(1), items: self.usage.items.saturating_add(self.semantic_usage.items), bytes: self.usage.bytes.saturating_add(node_page_bytes) };
                            if node_page_bytes > SURFACE_RECONCILE_PAGE_BYTES {
                                let fault = SurfaceReconcileFault::PageBytes { actual: node_page_bytes, max: SURFACE_RECONCILE_PAGE_BYTES };
                                self.fault = Some(fault.clone());
                                return SurfaceReconcileStep::Fault(fault);
                            }
                            if !projected.fits(self.limits) {
                                let fault = SurfaceReconcileFault::Credits { usage: projected, limits: self.limits };
                                self.fault = Some(fault.clone());
                                return SurfaceReconcileStep::Fault(fault);
                            }
                            return SurfaceReconcileStep::Yield { nodes: 0, bytes: delta.bytes };
                        }
                        SurfaceSemanticCensusStep::Fault(fault) => {
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                        SurfaceSemanticCensusStep::Complete => {}
                    }
                    let (parent, node) = self.held_node.take().expect("completed census retains exact node");
                    let mut node = *node;
                    self.semantic_census = None;
                    let node_page_bytes = size_of::<FlatPresentedNode>().saturating_add(self.semantic_usage.bytes);
                    let projected = SurfaceReconcileUsage { nodes: self.usage.nodes.saturating_add(1), items: self.usage.items.saturating_add(self.semantic_usage.items), bytes: self.usage.bytes.saturating_add(node_page_bytes) };
                    if !self.seen.insert((parent, node.key.clone())) {
                        self.held_node = Some((parent, Box::new(node)));
                        self.fault = Some(SurfaceReconcileFault::DuplicateSiblingKey);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::DuplicateSiblingKey);
                    }
                    let child_count = node.children.len();
                    let children = take(&mut node.children).into_iter();
                    let index = self.flat.len();
                    self.flat.push(FlatPresentedNode { parent, node, child_ids: Vec::new() });
                    self.traversal.push(PresentationFrame { index, children });
                    self.usage = projected;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                } else {
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::AllocateIdentities => {
                if self.allocate_index < self.flat.len() {
                    let parent_index = self.flat[self.allocate_index].parent;
                    let parent = parent_index.map(|index| self.ids[index]);
                    let key = self.flat[self.allocate_index].node.key.clone();
                    let key_bytes = key.len();
                    let identity = (parent, key);
                    let id = current.key_index.get(&identity).copied().unwrap_or_else(|| self.allocator.allocate());
                    self.new_key_index.insert(identity, id);
                    self.ids.push(id);
                    if let Some(parent) = parent_index {
                        self.flat[parent].child_ids.push(id);
                    }
                    self.allocate_index += 1;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<NodeIdentity>().saturating_add(key_bytes) }
                } else {
                    self.stage = SurfaceReconcileStage::DiffRecords;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::DiffRecords => {
                if self.diff_index < self.flat.len() {
                    let index = self.postorder[self.diff_index];
                    let id = self.ids[index];
                    let children = take(&mut self.flat[index].child_ids);
                    let flat = &self.flat[index];
                    let transition = current.retained.get(&id).and_then(|record| record.transition);
                    let record = build_record(id, &flat.node, children, transition);
                    if let Some(old) = current.retained.get(&id) {
                        self.ops.extend(diff_record(&self.surface, old, &record));
                    } else {
                        self.ops.push(ui_contract::UiPatchOp::Upsert(record.clone()));
                    }
                    self.new_retained.insert(id, record);
                    self.diff_index += 1;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                } else {
                    self.remove_next = self.old_root;
                    self.stage = SurfaceReconcileStage::RemoveStale;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::RemoveStale => {
                if let Some(id) = self.remove_next.take() {
                    if self.new_retained.contains_key(&id) {
                        self.removal.push(RemovalFrame { id, next_child: 0 });
                    } else {
                        self.ops.push(ui_contract::UiPatchOp::Remove { id });
                    }
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiNodeId>() }
                } else if let Some(frame) = self.removal.last_mut() {
                    let child = current.retained.get(&frame.id).and_then(|record| record.children.get(frame.next_child)).copied();
                    if let Some(child) = child {
                        frame.next_child += 1;
                        self.remove_next = Some(child);
                    } else {
                        self.removal.pop();
                    }
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiNodeId>() }
                } else {
                    self.stage = SurfaceReconcileStage::Finalize;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::Finalize => {
                let new_root = self.ids.first().copied();
                if self.old_root != new_root {
                    if let Some(id) = new_root {
                        self.ops.push(ui_contract::UiPatchOp::SetRoot { id });
                    }
                }
                let revision = if self.ops.is_empty() { self.base_revision } else { self.base_revision.next() };
                let patch = if self.ops.is_empty() { None } else { Some(ui_contract::UiPatch { surface: self.surface.clone(), base_revision: self.base_revision, revision, ops: take(&mut self.ops) }) };
                let reconciler = SurfaceReconciler {
                    surface: self.surface.clone(),
                    revision,
                    allocator: self.allocator.clone(),
                    retained: take(&mut self.new_retained),
                    key_index: take(&mut self.new_key_index),
                    root: new_root,
                    retire_scalar: 0,
                    persistent_credit: None,
                };
                SurfaceReconcileStep::Complete { reconciler, patch }
            }
        };
        if let SurfaceReconcileStep::Yield { nodes, bytes } = step {
            if bytes > SURFACE_RECONCILE_PAGE_BYTES {
                let fault = SurfaceReconcileFault::PageBytes { actual: bytes, max: SURFACE_RECONCILE_PAGE_BYTES };
                self.fault = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
            let items = nodes.saturating_mul(8);
            if !self.usage.include(nodes, items, bytes) || !self.usage.fits(self.limits) {
                let fault = SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits };
                self.fault = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
        }
        step
    }

    fn clear_fault(&mut self) {
        self.fault = None;
    }

    fn retire_one(&mut self) -> bool {
        if let Some((_, mut node)) = self.held_node.take() {
            let children = take(&mut node.children).into_iter();
            self.traversal.push(PresentationFrame { index: 0, children });
            self.semantic_census = None;
            return false;
        }
        if let Some(mut node) = self.pending_root.take() {
            let children = take(&mut node.children).into_iter();
            self.traversal.push(PresentationFrame { index: 0, children });
            return false;
        }
        if let Some(frame) = self.traversal.last_mut() {
            if let Some(child) = frame.children.next() {
                self.held_node = Some((None, Box::new(child)));
                self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                return false;
            }
            self.traversal.pop();
            return false;
        }
        if self.flat.pop().is_some() || self.postorder.pop().is_some() || self.ids.pop().is_some() || self.removal.pop().is_some() || self.ops.pop().is_some() {
            return false;
        }
        if let Some(key) = self.seen.iter().next().cloned() {
            self.seen.remove(&key);
            return false;
        }
        if let Some(id) = self.new_retained.keys().next().copied() {
            self.new_retained.remove(&id);
            return false;
        }
        if let Some(identity) = self.new_key_index.keys().next().cloned() {
            self.new_key_index.remove(&identity);
            return false;
        }
        self.remove_next = None;
        self.fault = None;
        true
    }
}

//#region 🎟️RetainedAuthority

pub const SURFACE_RECONCILE_ADMISSION_SLOTS: usize = 64;
pub const SURFACE_RECONCILE_PAGE_BYTES: usize = 16 * 1_024;
pub const SURFACE_RECONCILE_AGGREGATE_BYTES: usize = 8 * 1_024 * 1_024;
pub const SURFACE_RECONCILE_AGGREGATE_ITEMS: usize = 131_076;

#[derive(Clone, Copy, Debug, Default)]
struct SurfaceReconcileAdmissionSlot {
    epoch: u64,
    items: usize,
    bytes: usize,
    occupied: bool,
}

struct SurfaceReconcileAdmissionLedger {
    slots: [SurfaceReconcileAdmissionSlot; SURFACE_RECONCILE_ADMISSION_SLOTS],
    items: usize,
    bytes: usize,
}

impl Default for SurfaceReconcileAdmissionLedger {
    fn default() -> Self {
        Self { slots: [SurfaceReconcileAdmissionSlot::default(); SURFACE_RECONCILE_ADMISSION_SLOTS], items: 0, bytes: 0 }
    }
}

#[derive(Debug)]
struct SurfaceReconcileCredit {
    slot: usize,
    epoch: u64,
    items: usize,
    bytes: usize,
}

static SURFACE_RECONCILE_ADMISSION: LazyLock<Mutex<SurfaceReconcileAdmissionLedger>> = LazyLock::new(|| Mutex::new(SurfaceReconcileAdmissionLedger::default()));

fn reserve_surface_reconcile(limits: SurfaceReconcileLimits) -> Option<SurfaceReconcileCredit> {
    if limits.max_nodes > SurfaceReconcileLimits::default().max_nodes
        || limits.max_items > SurfaceReconcileLimits::default().max_items
        || limits.max_bytes > SurfaceReconcileLimits::default().max_bytes
        || limits.max_identifier_bytes > SurfaceReconcileLimits::default().max_identifier_bytes
    {
        return None;
    }
    let mut ledger = SURFACE_RECONCILE_ADMISSION.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let next_items = ledger.items.checked_add(limits.max_items)?;
    let next_bytes = ledger.bytes.checked_add(limits.max_bytes)?;
    if next_items > SURFACE_RECONCILE_AGGREGATE_ITEMS || next_bytes > SURFACE_RECONCILE_AGGREGATE_BYTES {
        return None;
    }
    let slot = ledger.slots.iter().position(|slot| !slot.occupied)?;
    let epoch = ledger.slots[slot].epoch.checked_add(1)?;
    if epoch == 0 {
        return None;
    }
    ledger.slots[slot] = SurfaceReconcileAdmissionSlot { epoch, items: limits.max_items, bytes: limits.max_bytes, occupied: true };
    ledger.items = next_items;
    ledger.bytes = next_bytes;
    Some(SurfaceReconcileCredit { slot, epoch, items: limits.max_items, bytes: limits.max_bytes })
}

fn release_surface_reconcile(credit: SurfaceReconcileCredit) {
    let mut ledger = SURFACE_RECONCILE_ADMISSION.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(slot) = ledger.slots.get_mut(credit.slot) else { return };
    if !slot.occupied || slot.epoch != credit.epoch || slot.items != credit.items || slot.bytes != credit.bytes {
        return;
    }
    let items = slot.items;
    let bytes = slot.bytes;
    slot.occupied = false;
    slot.items = 0;
    slot.bytes = 0;
    ledger.items = ledger.items.saturating_sub(items);
    ledger.bytes = ledger.bytes.saturating_sub(bytes);
}

/// 🎫️ Pre-materialization aggregate reservation transferred into exactly one live job.
pub struct SurfaceReconcileReservation {
    generation: u64,
    limits: SurfaceReconcileLimits,
    credit: Option<SurfaceReconcileCredit>,
}

impl SurfaceReconcileReservation {
    pub fn try_new(generation: u64) -> Option<Self> {
        if generation == 0 {
            return None;
        }
        let limits = SurfaceReconcileLimits::default();
        Some(Self { generation, limits, credit: Some(reserve_surface_reconcile(limits)?) })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

impl Drop for SurfaceReconcileReservation {
    fn drop(&mut self) {
        if let Some(credit) = self.credit.take() {
            release_surface_reconcile(credit);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SurfaceReconcileJobPhase {
    Drive,
    RetireCursor,
    RetirePrevious,
    Ready,
    Fault,
    Closing,
}

struct SurfaceReconcileRetained {
    generation: u64,
    phase: SurfaceReconcileJobPhase,
    current: Option<SurfaceReconciler>,
    source: Option<crate::ComponentTree>,
    cursor: Option<SurfaceReconcileCursor>,
    candidate: Option<SurfaceReconciler>,
    patch: Option<ui_contract::UiPatch>,
    retire_root: Option<crate::TreeNode>,
    retire_forest: Vec<std::vec::IntoIter<crate::TreeNode>>,
    fault: Option<SurfaceReconcileFault>,
    credit: Option<SurfaceReconcileCredit>,
    handback_next: Option<Box<SurfaceReconcileRetained>>,
}

impl SurfaceReconcileRetained {
    fn close_step(&mut self) -> bool {
        self.phase = SurfaceReconcileJobPhase::Closing;
        if self.fault.take().is_some() {
            return false;
        }
        if let Some(patch) = self.patch.as_mut() {
            if patch.ops.pop().is_some() {
                return false;
            }
            self.patch = None;
            return false;
        }
        if let Some(candidate) = self.candidate.as_mut() {
            if !candidate.retire_one() {
                return false;
            }
            self.candidate = None;
            return false;
        }
        if let Some(cursor) = self.cursor.as_mut() {
            if !cursor.retire_one() {
                return false;
            }
            self.cursor = None;
            return false;
        }
        if let Some(current) = self.current.as_mut() {
            if !current.retire_one() {
                return false;
            }
            self.current = None;
            return false;
        }
        if let Some(tree) = self.source.take() {
            self.retire_root = Some(tree.root);
            return false;
        }
        if let Some(mut node) = self.retire_root.take() {
            self.retire_forest.push(take(&mut node.children).into_iter());
            return false;
        }
        if let Some(children) = self.retire_forest.last_mut() {
            if let Some(child) = children.next() {
                self.retire_root = Some(child);
            } else {
                self.retire_forest.pop();
            }
            return false;
        }
        if let Some(credit) = self.credit.take() {
            release_surface_reconcile(credit);
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.current.is_none()
            && self.source.is_none()
            && self.cursor.is_none()
            && self.candidate.is_none()
            && self.patch.is_none()
            && self.retire_root.is_none()
            && self.retire_forest.is_empty()
            && self.fault.is_none()
            && self.credit.is_none()
            && self.handback_next.is_none()
    }
}

static SURFACE_RECONCILE_TERMINALS: LazyLock<Mutex<Option<Box<SurfaceReconcileRetained>>>> = LazyLock::new(|| Mutex::new(None));

fn handback_surface_reconcile(mut state: Box<SurfaceReconcileRetained>) {
    let mut terminals = SURFACE_RECONCILE_TERMINALS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    state.handback_next = terminals.take();
    *terminals = Some(state);
}

/// 🚦️ One admitted reconciliation opportunity result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileJobStep {
    MoreWork,
    Ready,
    Fault,
}

/// 🧵️ Generation-keyed by-value reconciliation job advanced once per worker grant.
pub struct SurfaceReconcileJob {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl SurfaceReconcileJob {
    pub fn try_new(current: SurfaceReconciler, tree: crate::ComponentTree, generation: u64) -> Result<Self, SurfaceReconcileRejected> {
        Self::try_new_with_limits(current, tree, generation, SurfaceReconcileLimits::default())
    }

    pub fn try_new_with_limits(current: SurfaceReconciler, tree: crate::ComponentTree, generation: u64, limits: SurfaceReconcileLimits) -> Result<Self, SurfaceReconcileRejected> {
        let surface_bytes = current.surface.0.len();
        let credit = if surface_bytes <= limits.max_identifier_bytes { reserve_surface_reconcile(limits) } else { None };
        let Some(credit) = credit else {
            return Err(SurfaceReconcileRejected {
                state: Some(Box::new(SurfaceReconcileRetained {
                    generation,
                    phase: SurfaceReconcileJobPhase::Fault,
                    current: Some(current),
                    source: Some(tree),
                    cursor: None,
                    candidate: None,
                    patch: None,
                    retire_root: None,
                    retire_forest: Vec::new(),
                    fault: Some(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: surface_bytes }, limits }),
                    credit: None,
                    handback_next: None,
                })),
            });
        };
        let cursor = SurfaceReconcileCursor::new_with_limits(tree, &current, limits);
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                generation,
                phase: SurfaceReconcileJobPhase::Drive,
                current: Some(current),
                source: None,
                cursor: Some(cursor),
                candidate: None,
                patch: None,
                retire_root: None,
                retire_forest: Vec::new(),
                fault: None,
                credit: Some(credit),
                handback_next: None,
            })),
        })
    }

    pub fn try_new_reserved(current: SurfaceReconciler, tree: crate::ComponentTree, mut reservation: SurfaceReconcileReservation) -> Result<Self, SurfaceReconcileRejected> {
        let generation = reservation.generation;
        let limits = reservation.limits;
        let surface_bytes = current.surface.0.len();
        if surface_bytes > limits.max_identifier_bytes {
            return Err(SurfaceReconcileRejected {
                state: Some(Box::new(SurfaceReconcileRetained {
                    generation,
                    phase: SurfaceReconcileJobPhase::Fault,
                    current: Some(current),
                    source: Some(tree),
                    cursor: None,
                    candidate: None,
                    patch: None,
                    retire_root: None,
                    retire_forest: Vec::new(),
                    fault: Some(SurfaceReconcileFault::IdentifierBytes { actual: surface_bytes, max: limits.max_identifier_bytes }),
                    credit: reservation.credit.take(),
                    handback_next: None,
                })),
            });
        }
        let cursor = SurfaceReconcileCursor::new_with_limits(tree, &current, limits);
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                generation,
                phase: SurfaceReconcileJobPhase::Drive,
                current: Some(current),
                source: None,
                cursor: Some(cursor),
                candidate: None,
                patch: None,
                retire_root: None,
                retire_forest: Vec::new(),
                fault: None,
                credit: reservation.credit.take(),
                handback_next: None,
            })),
        })
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn base_revision(&self) -> ui_contract::UiRevision {
        self.state.as_ref().and_then(|state| state.current.as_ref()).map_or(ui_contract::UiRevision::default(), SurfaceReconciler::revision)
    }

    pub fn drive_one(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> SurfaceReconcileJobStep {
        let Some(state) = self.state.as_mut() else { return SurfaceReconcileJobStep::Fault };
        if state.phase == SurfaceReconcileJobPhase::Ready {
            return SurfaceReconcileJobStep::Ready;
        }
        if state.phase == SurfaceReconcileJobPhase::Fault || state.phase == SurfaceReconcileJobPhase::Closing {
            return SurfaceReconcileJobStep::Fault;
        }
        if cx.generation().0 != state.generation {
            state.fault = Some(SurfaceReconcileFault::StaleGeneration { expected: state.generation, actual: cx.generation().0 });
            state.phase = SurfaceReconcileJobPhase::Fault;
            return SurfaceReconcileJobStep::Fault;
        }
        if cx.is_cancelled() {
            state.fault = Some(SurfaceReconcileFault::Cancelled);
            state.phase = SurfaceReconcileJobPhase::Fault;
            return SurfaceReconcileJobStep::Fault;
        }
        if cx.should_yield() {
            return SurfaceReconcileJobStep::MoreWork;
        }
        let outcome = match state.phase {
            SurfaceReconcileJobPhase::Drive => {
                let Some(current) = state.current.as_ref() else {
                    state.phase = SurfaceReconcileJobPhase::Fault;
                    return SurfaceReconcileJobStep::Fault;
                };
                let Some(cursor) = state.cursor.as_mut() else {
                    state.phase = SurfaceReconcileJobPhase::Fault;
                    return SurfaceReconcileJobStep::Fault;
                };
                match cursor.step(current) {
                    SurfaceReconcileStep::Yield { .. } => SurfaceReconcileJobStep::MoreWork,
                    SurfaceReconcileStep::Complete { reconciler, patch } => {
                        state.candidate = Some(reconciler);
                        state.patch = patch;
                        state.phase = SurfaceReconcileJobPhase::RetireCursor;
                        SurfaceReconcileJobStep::MoreWork
                    }
                    SurfaceReconcileStep::Fault(fault) => {
                        state.fault = Some(fault);
                        state.phase = SurfaceReconcileJobPhase::Fault;
                        SurfaceReconcileJobStep::Fault
                    }
                }
            }
            SurfaceReconcileJobPhase::RetireCursor => {
                if state.cursor.as_mut().is_some_and(|cursor| cursor.retire_one()) {
                    state.cursor = None;
                    state.phase = SurfaceReconcileJobPhase::RetirePrevious;
                }
                SurfaceReconcileJobStep::MoreWork
            }
            SurfaceReconcileJobPhase::RetirePrevious => {
                if state.current.as_mut().is_some_and(|current| current.retire_one()) {
                    state.current = None;
                    state.phase = SurfaceReconcileJobPhase::Ready;
                    SurfaceReconcileJobStep::Ready
                } else {
                    SurfaceReconcileJobStep::MoreWork
                }
            }
            SurfaceReconcileJobPhase::Ready => SurfaceReconcileJobStep::Ready,
            SurfaceReconcileJobPhase::Fault | SurfaceReconcileJobPhase::Closing => SurfaceReconcileJobStep::Fault,
        };
        cx.consume_fuel(1);
        if cx.is_cancelled() {
            state.fault = Some(SurfaceReconcileFault::Cancelled);
            state.phase = SurfaceReconcileJobPhase::Fault;
            return SurfaceReconcileJobStep::Fault;
        }
        outcome
    }

    pub fn fault(&self) -> Option<&SurfaceReconcileFault> {
        self.state.as_ref().and_then(|state| state.fault.as_ref())
    }

    pub fn take_ready(mut self) -> Result<(SurfaceReconciler, Option<ui_contract::UiPatch>), Self> {
        let ready = self.state.as_ref().is_some_and(|state| state.phase == SurfaceReconcileJobPhase::Ready);
        if !ready {
            return Err(self);
        }
        let mut state = self.state.take().expect("ready reconciliation retained state");
        let mut reconciler = state.candidate.take().expect("ready reconciliation candidate");
        let patch = state.patch.take();
        reconciler.persistent_credit = state.credit.take();
        Ok((reconciler, patch))
    }

    pub fn into_terminal(mut self) -> SurfaceReconcileTerminal {
        SurfaceReconcileTerminal { state: self.state.take() }
    }
}

impl Drop for SurfaceReconcileJob {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

/// 🔄️ Exact pre-admission owner returned without snapshot/tree cloning.
pub struct SurfaceReconcileRejected {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl std::fmt::Debug for SurfaceReconcileRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("SurfaceReconcileRejected").field("generation", &self.generation()).finish()
    }
}

impl SurfaceReconcileRejected {
    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn retry(mut self, limits: SurfaceReconcileLimits) -> Result<SurfaceReconcileJob, Self> {
        let Some(mut state) = self.state.take() else { return Err(self) };
        let (Some(current), Some(tree)) = (state.current.take(), state.source.take()) else {
            self.state = Some(state);
            return Err(self);
        };
        match SurfaceReconcileJob::try_new_with_limits(current, tree, state.generation, limits) {
            Ok(job) => Ok(job),
            Err(mut rejected) => {
                if let Some(next) = rejected.state.as_mut() {
                    next.fault = state.fault.take();
                }
                Err(rejected)
            }
        }
    }

    pub fn take_sources(&mut self) -> Option<(SurfaceReconciler, crate::ComponentTree)> {
        let state = self.state.as_mut()?;
        let current = state.current.take()?;
        let Some(tree) = state.source.take() else {
            state.current = Some(current);
            return None;
        };
        state.fault = None;
        Some((current, tree))
    }

    pub fn close_step(&mut self) -> bool {
        self.state.as_mut().is_none_or(SurfaceReconcileRetained::close_step)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.as_ref().is_none_or(SurfaceReconcileRetained::terminal_is_empty)
    }

    pub fn into_terminal(mut self) -> SurfaceReconcileTerminal {
        SurfaceReconcileTerminal { state: self.state.take() }
    }
}

impl Drop for SurfaceReconcileRejected {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

/// 🧹️ Public fault/cancel/close authority; each close grant retires one retained owner.
pub struct SurfaceReconcileTerminal {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl SurfaceReconcileTerminal {
    pub fn from_sources(current: SurfaceReconciler, tree: crate::ComponentTree, generation: u64) -> Self {
        Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: Some(current),
                source: Some(tree),
                cursor: None,
                candidate: None,
                patch: None,
                retire_root: None,
                retire_forest: Vec::new(),
                fault: None,
                credit: None,
                handback_next: None,
            })),
        }
    }

    pub fn from_reserved_sources(mut current: SurfaceReconciler, tree: crate::ComponentTree, mut reservation: SurfaceReconcileReservation) -> Self {
        current.persistent_credit = reservation.credit.take();
        Self::from_sources(current, tree, reservation.generation)
    }

    pub fn from_patch(patch: ui_contract::UiPatch, generation: u64) -> Self {
        Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: None,
                source: None,
                cursor: None,
                candidate: None,
                patch: Some(patch),
                retire_root: None,
                retire_forest: Vec::new(),
                fault: None,
                credit: None,
                handback_next: None,
            })),
        }
    }

    pub fn from_reconciler(reconciler: SurfaceReconciler, generation: u64) -> Self {
        Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: Some(reconciler),
                source: None,
                cursor: None,
                candidate: None,
                patch: None,
                retire_root: None,
                retire_forest: Vec::new(),
                fault: None,
                credit: None,
                handback_next: None,
            })),
        }
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn fault(&self) -> Option<&SurfaceReconcileFault> {
        self.state.as_ref().and_then(|state| state.fault.as_ref())
    }

    pub fn resume(mut self, generation: u64) -> Result<SurfaceReconcileJob, Self> {
        let Some(state) = self.state.as_mut() else { return Err(self) };
        if state.generation != generation || state.cursor.is_none() || state.current.is_none() {
            return Err(self);
        }
        state.fault = None;
        state.cursor.as_mut().map(SurfaceReconcileCursor::clear_fault);
        state.phase = SurfaceReconcileJobPhase::Drive;
        Ok(SurfaceReconcileJob { state: self.state.take() })
    }

    pub fn close_step(&mut self) -> bool {
        self.state.as_mut().is_none_or(SurfaceReconcileRetained::close_step)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.as_ref().is_none_or(SurfaceReconcileRetained::terminal_is_empty)
    }
}

impl Drop for SurfaceReconcileTerminal {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

pub fn take_surface_reconcile_terminal(generation: u64) -> Option<SurfaceReconcileTerminal> {
    let mut terminals = SURFACE_RECONCILE_TERMINALS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut link = &mut *terminals;
    while link.as_ref().is_some_and(|state| state.generation != generation) {
        link = &mut link.as_mut().expect("handback link exists").handback_next;
    }
    let mut state = link.take()?;
    *link = state.handback_next.take();
    Some(SurfaceReconcileTerminal { state: Some(state) })
}

//#endregion 🎟️RetainedAuthority

fn estimate_record_bytes(record: &ui_contract::UiNodeRecord) -> usize {
    size_of::<ui_contract::UiNodeRecord>().saturating_add(record.key.len()).saturating_add(record.children.len().saturating_mul(size_of::<ui_contract::UiNodeId>()))
}

fn diff_record(_surface: &ui_contract::SurfaceId, old: &ui_contract::UiNodeRecord, new: &ui_contract::UiNodeRecord) -> Vec<ui_contract::UiPatchOp> {
    let id = new.id;
    let mut targeted = Vec::with_capacity(8);
    if old.component != new.component {
        targeted.push(ui_contract::UiPatchOp::SetComponent { id, component: new.component.clone() });
    }
    if old.layout != new.layout {
        targeted.push(ui_contract::UiPatchOp::SetLayout { id, layout: new.layout.clone() });
    }
    if old.activity != new.activity || old.disabled != new.disabled {
        targeted.push(ui_contract::UiPatchOp::SetActivity { id, activity: new.activity, disabled: new.disabled });
    }
    if old.children != new.children {
        targeted.push(ui_contract::UiPatchOp::SetChildren { id, children: new.children.clone() });
    }
    if old.style != new.style {
        targeted.push(ui_contract::UiPatchOp::SetStyle { id, style: new.style });
    }
    if old.accessibility != new.accessibility {
        targeted.push(ui_contract::UiPatchOp::SetAccessibility { id, accessibility: new.accessibility.clone() });
    }
    if old.bindings != new.bindings {
        targeted.push(ui_contract::UiPatchOp::SetBindings { id, bindings: new.bindings.clone() });
    }
    if old.menu != new.menu {
        targeted.push(ui_contract::UiPatchOp::SetMenu { id, menu: new.menu.clone() });
    }
    if targeted.is_empty() {
        return targeted;
    }
    targeted
}

//#endregion ⏭️ResumableReconcile

//#region 🔖️Diff

#[cfg(test)]
impl SurfaceReconciler {
    /// ♻️ Resolves `node`'s identity under `parent` against [`Self::key_index`]: a hit reuses the
    /// existing id and diffs field-by-field via [`Self::diff_existing`]; a miss mints a fresh id via
    /// the allocator and inserts the node wholesale via one `Upsert` — the only two ways any node ever
    /// enters `ops`. Returns the id `node` now has, whichever path was taken.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn diff_node(&mut self, parent: Option<ui_contract::UiNodeId>, node: &crate::TreeNode, ops: &mut Vec<ui_contract::UiPatchOp>) -> ui_contract::UiNodeId {
        let identity = identity_of(parent, node);
        if let Some(&id) = self.key_index.get(&identity) {
            self.diff_existing(id, node, ops);
            id
        } else {
            let id = self.allocator.allocate();
            self.key_index.insert(identity, id);
            let child_ids = self.diff_children(id, &[], &node.children, ops);
            let record = build_record(id, node, child_ids, None);
            self.retained.insert(id, record.clone());
            ops.push(ui_contract::UiPatchOp::Upsert(record));
            id
        }
    }

    /// ♻️ Diffs `node` against the retained record already at `id`, choosing the narrowest
    /// representation of the change. Children are diffed first (post-order), since the parent's own
    /// children list — and therefore whether `SetChildren` fires — depends on ids children may have
    /// only just been minted with. Every one of the eight field groups
    /// (`component`/`layout`/`activity`+`disabled`/`children`/`style`/`accessibility`/`bindings`/
    /// `menu`) now has its own [`ui_contract::UiPatchOp`] setter, so a change touching only one group
    /// always emits exactly that one op, deterministically, with no byte comparison — that determinism
    /// is load-bearing: it is what keeps a same-size reorder (`SetChildren` alone) from ever being
    /// second-guessed into an `Upsert` merely because the wire-cost estimator does not itself price a
    /// record's `children` list. Only once **more than one** group changed does this weigh a full
    /// `Upsert` against the targeted ops it would replace, via [`Self::estimate_bytes`], and picks
    /// whichever is actually smaller on the wire — so `Upsert` is reserved for a genuinely new node
    /// ([`Self::diff_node`]'s other arm) or for a multi-group change so broad that one full record
    /// beats several targeted ops.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn diff_existing(&mut self, id: ui_contract::UiNodeId, node: &crate::TreeNode, ops: &mut Vec<ui_contract::UiPatchOp>) {
        let old = self.retained.get(&id).cloned().expect("🚫️ key_index names an id with no retained record");
        let new_child_ids = self.diff_children(id, &old.children, &node.children, ops);

        let mut targeted = Vec::new();
        if old.component != node.component {
            targeted.push(ui_contract::UiPatchOp::SetComponent { id, component: node.component.clone() });
        }
        if old.layout != node.layout {
            targeted.push(ui_contract::UiPatchOp::SetLayout { id, layout: node.layout.clone() });
        }
        if old.activity != node.activity || old.disabled != node.disabled {
            targeted.push(ui_contract::UiPatchOp::SetActivity { id, activity: node.activity, disabled: node.disabled });
        }
        if old.children != new_child_ids {
            targeted.push(ui_contract::UiPatchOp::SetChildren { id, children: new_child_ids.clone() });
        }
        if old.style != node.style {
            targeted.push(ui_contract::UiPatchOp::SetStyle { id, style: node.style });
        }
        if old.accessibility != node.accessibility {
            targeted.push(ui_contract::UiPatchOp::SetAccessibility { id, accessibility: node.accessibility.clone() });
        }
        if old.bindings != node.bindings {
            targeted.push(ui_contract::UiPatchOp::SetBindings { id, bindings: node.bindings.clone() });
        }
        if old.menu != node.menu {
            targeted.push(ui_contract::UiPatchOp::SetMenu { id, menu: node.menu.clone() });
        }

        if targeted.is_empty() {
            return;
        }

        let record = build_record(id, node, new_child_ids, old.transition);
        let upsert = ui_contract::UiPatchOp::Upsert(record.clone());
        let use_upsert = targeted.len() > 1 && self.estimate_bytes(std::slice::from_ref(&upsert)) < self.estimate_bytes(&targeted);

        self.retained.insert(id, record);
        if use_upsert {
            ops.push(upsert);
        } else {
            ops.extend(targeted);
        }
    }

    /// 💰️ Wire-cost estimate for `candidate_ops`, delegated to [`ui_contract::patch_byte_estimate`]
    /// via a throwaway single-purpose [`ui_contract::UiPatch`] — the byte-accounting logic (including
    /// which fields even count as "text") lives once, in the contract crate that also enforces
    /// `max_patch_bytes`, and is never duplicated here.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn estimate_bytes(&self, candidate_ops: &[ui_contract::UiPatchOp]) -> usize {
        let probe = ui_contract::UiPatch { surface: self.surface.clone(), base_revision: ui_contract::UiRevision::default(), revision: ui_contract::UiRevision::default(), ops: candidate_ops.to_vec() };
        ui_contract::patch_byte_estimate(&probe)
    }

    /// 👶️ Diffs `new_children` against `old_child_ids` under `parent_id`, matching purely by
    /// `(parent_id, key)` — never by position — so reordering, inserting, and removing siblings each
    /// touch only the ids actually affected. Every old child whose id is not among the freshly diffed
    /// ids is removed as a whole subtree. Returns the new children list in `new_children`'s order,
    /// ready to become the parent's own `children` field.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn diff_children(&mut self, parent_id: ui_contract::UiNodeId, old_child_ids: &[ui_contract::UiNodeId], new_children: &[crate::TreeNode], ops: &mut Vec<ui_contract::UiPatchOp>) -> Vec<ui_contract::UiNodeId> {
        assert_unique_child_keys(parent_id, new_children);

        let mut new_ids = Vec::with_capacity(new_children.len());
        for child in new_children {
            new_ids.push(self.diff_node(Some(parent_id), child, ops));
        }

        let retained_ids: HashSet<ui_contract::UiNodeId> = new_ids.iter().copied().collect();
        for &old_id in old_child_ids {
            if !retained_ids.contains(&old_id) {
                self.remove_subtree(Some(parent_id), old_id, ops);
            }
        }

        new_ids
    }

    /// 🗑️ Emits one `Remove` for `id` — the contract's own [`ui_contract::apply_patch`] deletes the
    /// whole subtree on the receiver side — and mirrors that locally via [`Self::purge_subtree`], so
    /// this reconciler's own `retained`/`key_index` never accumulate an orphan for a node the receiver
    /// no longer has either.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn remove_subtree(&mut self, parent: Option<ui_contract::UiNodeId>, id: ui_contract::UiNodeId, ops: &mut Vec<ui_contract::UiPatchOp>) {
        ops.push(ui_contract::UiPatchOp::Remove { id });
        self.purge_subtree(parent, id);
    }

    /// 🧹️ Removes `id` and every node reachable from it via its own retained `children`, purging both
    /// `retained` and `key_index` for each — the local mirror of [`ui_contract::apply_patch`]'s
    /// `remove_subtree`. `id` is never handed back to the allocator, so it can never be reused.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn purge_subtree(&mut self, parent: Option<ui_contract::UiNodeId>, id: ui_contract::UiNodeId) {
        if let Some(record) = self.retained.remove(&id) {
            self.key_index.remove(&(parent, record.key));
            for child_id in record.children {
                self.purge_subtree(Some(id), child_id);
            }
        }
    }
}

/// 🏗️ Assembles a complete [`ui_contract::UiNodeRecord`] for `node` at `id` with `children` already
/// resolved to ids and `transition` carried over verbatim — [`crate::TreeNode`] has no `transition`
/// field of its own (see `🦀️present.rs`'s module doc: it is builder-side and never diffs against a
/// previous tree), so this reconciler is the one place a record's `transition` is set, and it never
/// invents one: `None` for a freshly seen node, whatever the retained record already carried for an
/// existing one. Driving `Introducing`/`Celebrating` from presence data is out of this packet's scope.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build_record(id: ui_contract::UiNodeId, node: &crate::TreeNode, children: Vec<ui_contract::UiNodeId>, transition: Option<ui_contract::TransitionHint>) -> ui_contract::UiNodeRecord {
    ui_contract::UiNodeRecord {
        id,
        key: node.key.clone(),
        component: node.component.clone(),
        layout: node.layout.clone(),
        style: node.style,
        activity: node.activity,
        disabled: node.disabled,
        transition,
        accessibility: node.accessibility.clone(),
        bindings: node.bindings.clone(),
        menu: node.menu.clone(),
        children,
    }
}

//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Fixtures
    fn leaf(key: &str) -> crate::TreeNode {
        crate::TreeNode::new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {}))
    }

    fn text(key: &str, value: &str) -> crate::TreeNode {
        crate::TreeNode::new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::from(value), emphasize: None, data_attributes: None }))
    }

    fn container(key: &str, children: Vec<crate::TreeNode>) -> crate::TreeNode {
        crate::TreeNode::new(key, ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
            .with_children(children)
    }

    fn tree(root: crate::TreeNode) -> crate::ComponentTree {
        crate::ComponentTree::new(root)
    }

    fn styled(node: crate::TreeNode, tone: ui_contract::Tone) -> crate::TreeNode {
        crate::TreeNode { style: ui_contract::StyleSpec { tone, ..Default::default() }, ..node }
    }

    fn with_shortcut(node: crate::TreeNode, shortcut: &str) -> crate::TreeNode {
        crate::TreeNode { accessibility: ui_contract::AccessibilitySpec { shortcut: Some(shortcut.into()), ..Default::default() }, ..node }
    }

    fn with_binding(node: crate::TreeNode, scope: &str, name: &str) -> crate::TreeNode {
        crate::TreeNode { bindings: vec![ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::v1(scope, name), args: None, capability: None }], ..node }
    }

    fn with_menu(node: crate::TreeNode, menu_id: &str) -> crate::TreeNode {
        crate::TreeNode { menu: Some(ui_contract::MenuRef { id: menu_id.into(), args: None }), ..node }
    }

    fn id_of(snapshot: &ui_contract::UiSnapshot, key: &str) -> ui_contract::UiNodeId {
        snapshot.nodes.iter().find(|record| record.key == key).unwrap_or_else(|| panic!("no node keyed {key:?} in snapshot")).id
    }

    fn assert_snapshot_matches_state(snapshot: &ui_contract::UiSnapshot, state: &ui_contract::UiSnapshotState) {
        assert_eq!(snapshot.revision, state.revision);
        assert_eq!(Some(snapshot.root), state.root);
        assert_eq!(snapshot.nodes.len(), state.nodes.len(), "snapshot/state node-count mismatch");
        for record in &snapshot.nodes {
            assert_eq!(state.nodes.get(&record.id), Some(record), "record {:?} diverges between snapshot and applied state", record.id);
        }
    }

    fn reconcile_resumable(current: &SurfaceReconciler, component_tree: crate::ComponentTree) -> (SurfaceReconciler, Option<ui_contract::UiPatch>, usize) {
        let mut cursor = SurfaceReconcileCursor::new(component_tree, current);
        let mut yields = 0;
        loop {
            match cursor.step(current) {
                SurfaceReconcileStep::Yield { .. } => yields += 1,
                SurfaceReconcileStep::Complete { reconciler, patch } => return (reconciler, patch, yields),
                SurfaceReconcileStep::Fault(fault) => panic!("unexpected reconcile fault: {fault:?}"),
            }
        }
    }
    //#endregion 🔖️Fixtures

    //#region ⏭️ResumableCursor
    #[test]
    fn resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics() {
        let component_tree = tree(container("root", vec![text("a", "hello"), container("b", vec![leaf("x"), leaf("y")])]));
        let mut direct = SurfaceReconciler::new("s");
        let expected_patch = direct.reconcile(&component_tree).expect("initial direct patch");

        let current = SurfaceReconciler::new("s");
        let (resumed, actual_patch, yields) = reconcile_resumable(&current, component_tree);

        assert_eq!(actual_patch, Some(expected_patch));
        let actual = resumed.snapshot();
        let expected = direct.snapshot();
        assert_eq!(actual.revision, expected.revision);
        assert_eq!(actual.root, expected.root);
        assert_eq!(actual.nodes.len(), expected.nodes.len());
        assert!(expected.nodes.iter().all(|record| actual.nodes.contains(record)));
        assert!(yields >= 15, "five nodes must cross traversal, identity, and diff cursors");
    }

    #[test]
    fn abandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged() {
        let mut current = SurfaceReconciler::new("s");
        current.reconcile(&tree(container("root", vec![leaf("baseline")]))).expect("baseline");
        let before = current.snapshot();
        let children = (0..2_000).map(|index| leaf(&format!("item-{index}"))).collect();
        let mut cursor = SurfaceReconcileCursor::new(tree(container("root", children)), &current);

        for _ in 0..1_000 {
            assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
        }
        drop(cursor);

        assert_eq!(current.snapshot(), before, "cancellation or supersession must discard only candidate state");
    }

    #[test]
    fn every_large_tree_cursor_slice_stays_below_eight_milliseconds() {
        use std::time::{Duration, Instant};

        let children = (0..2_000).map(|index| leaf(&format!("item-{index}"))).collect();
        let current = SurfaceReconciler::new("s");
        let mut cursor = SurfaceReconcileCursor::new(tree(container("root", children)), &current);
        let mut yields = 0;
        loop {
            let started = Instant::now();
            let step = cursor.step(&current);
            let elapsed = started.elapsed();
            assert!(elapsed < Duration::from_millis(8), "one node cursor slice took {elapsed:?}");
            match step {
                SurfaceReconcileStep::Yield { nodes, .. } => {
                    assert!(nodes <= 1);
                    yields += 1;
                }
                SurfaceReconcileStep::Complete { reconciler, patch } => {
                    assert_eq!(reconciler.snapshot().nodes.len(), 2_001);
                    assert!(patch.is_some());
                    break;
                }
                SurfaceReconcileStep::Fault(fault) => panic!("unexpected reconcile fault: {fault:?}"),
            }
        }
        assert!(yields >= 6_003, "every presented node crosses three independent cursor phases");
    }

    #[test]
    fn identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation() {
        let surface = "s".repeat(SurfaceReconcileLimits::default().max_identifier_bytes + 1);
        let tree = tree(leaf("exact"));
        let pointer = tree.root.key.as_ptr();
        let mut rejected = match SurfaceReconcileJob::try_new(SurfaceReconciler::new(surface), tree, 71) {
            Ok(_) => panic!("identifier + 1 must reject"),
            Err(rejected) => rejected,
        };
        let (_, returned) = rejected.take_sources().expect("exact rejected owners");
        assert_eq!(returned.root.key.as_ptr(), pointer);
        assert!(rejected.close_step());
        assert!(rejected.terminal_is_empty());
    }

    #[test]
    fn dynamic_semantic_page_plus_one_faults_before_key_or_record_clone() {
        let mut value = String::with_capacity(SURFACE_RECONCILE_PAGE_BYTES);
        value.extend(std::iter::repeat_n('x', SURFACE_RECONCILE_PAGE_BYTES));
        let node = crate::TreeNode::new("exact", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(value), emphasize: None, data_attributes: Some(vec![("semantic".into(), "payload".into())]) }));
        let pointer = node.key.as_ptr();
        let current = SurfaceReconciler::new("s");
        let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
        assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
        assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Fault(SurfaceReconcileFault::PageBytes { .. })));
        let retained = cursor.held_node.as_ref().expect("exact unmaterialized node remains retained");
        assert_eq!(retained.1.key.as_ptr(), pointer);
        assert!(cursor.flat.is_empty());
        assert!(cursor.seen.is_empty());
        assert!(cursor.new_retained.is_empty());
        assert!(cursor.ops.is_empty());
    }

    #[test]
    fn semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged() {
        fn expired_now() -> u64 {
            10
        }
        let generation = 7_001;
        let mut zero = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("zero")), generation).expect("admitted");
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(0, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_ms,
            &mut sequence,
        );
        assert_eq!(zero.drive_one(&mut context), SurfaceReconcileJobStep::MoreWork);
        let cursor = zero.state.as_ref().and_then(|state| state.cursor.as_ref()).expect("cursor retained");
        assert!(cursor.pending_root.is_some());
        assert!(cursor.held_node.is_none());

        let generation = 7_002;
        let mut expired = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("deadline")), generation).expect("admitted");
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, 10),
            semio_framework_job::root_cancel_token(),
            expired_now,
            &mut sequence,
        );
        assert_eq!(expired.drive_one(&mut context), SurfaceReconcileJobStep::MoreWork);
        assert!(expired.state.as_ref().and_then(|state| state.cursor.as_ref()).is_some_and(|cursor| cursor.pending_root.is_some() && cursor.held_node.is_none()));
    }

    #[test]
    fn semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion() {
        let wide = (0..2_048).map(|index| ui_contract::UiValue::Text(format!("value-{index}"))).collect();
        let node = crate::TreeNode::new("wide", ui_contract::Component::Extension(ui_contract::ExtensionProps { extension: "fixture".into(), props: ui_contract::UiValue::List(wide) }));
        let current = SurfaceReconciler::new("s");
        let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
        for _ in 0..32 {
            assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
        }
        assert!(cursor.flat.is_empty(), "a wide value cannot complete census in one grant");

        let mut deep = ui_contract::UiValue::Null;
        for _ in 0..=SURFACE_RECONCILE_VALUE_DEPTH {
            deep = ui_contract::UiValue::List(vec![deep]);
        }
        let node = crate::TreeNode::new("deep", ui_contract::Component::Extension(ui_contract::ExtensionProps { extension: "fixture".into(), props: deep }));
        let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
        let mut fault = None;
        for _ in 0..512 {
            if let SurfaceReconcileStep::Fault(found) = cursor.step(&current) {
                fault = Some(found);
                break;
            }
        }
        assert!(matches!(fault, Some(SurfaceReconcileFault::ValueDepth { .. })));
        assert!(cursor.flat.is_empty());
    }

    #[test]
    fn persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement() {
        let generation = 7_003;
        let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("credit")), generation).expect("admitted");
        let mut sequence = 0;
        for _ in 0..4_096 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::allocate_operation_id(),
                semio_framework_job::Generation(generation),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_ms,
                &mut sequence,
            );
            if job.drive_one(&mut context) == SurfaceReconcileJobStep::Ready {
                break;
            }
        }
        let (reconciler, _) = match job.take_ready() {
            Ok(ready) => ready,
            Err(_) => panic!("ready owner"),
        };
        assert!(reconciler.persistent_credit.is_some(), "take_ready transfers rather than releases credit");
        let mut terminal = SurfaceReconcileTerminal::from_reconciler(reconciler, generation);
        assert!(!terminal.close_step());
        while !terminal.terminal_is_empty() {
            terminal.close_step();
        }
    }

    #[test]
    fn public_drop_handback_is_lossless_at_terminal_cap_and_plus_one() {
        let first = 80_000;
        for offset in 0..=SURFACE_RECONCILE_ADMISSION_SLOTS {
            drop(SurfaceReconcileTerminal::from_reconciler(SurfaceReconciler::new(format!("drop-{offset}")), first + offset as u64));
        }
        for offset in 0..=SURFACE_RECONCILE_ADMISSION_SLOTS {
            let mut terminal = take_surface_reconcile_terminal(first + offset as u64).expect("every intrusive handback owner remains recoverable");
            while !terminal.terminal_is_empty() {
                terminal.close_step();
            }
        }
    }

    #[test]
    fn stale_cancel_and_drop_handoff_preserve_public_terminal_ownership() {
        let generation = 8_001;
        let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("exact")), generation).expect("admitted");
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation + 1),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_ms,
            &mut sequence,
        );
        assert_eq!(job.drive_one(&mut context), SurfaceReconcileJobStep::Fault);
        drop(job);
        let mut terminal = take_surface_reconcile_terminal(generation).expect("drop handback is observable");
        assert!(matches!(terminal.fault(), Some(SurfaceReconcileFault::StaleGeneration { .. })));
        for _ in 0..32 {
            if terminal.close_step() && terminal.terminal_is_empty() {
                break;
            }
        }
        assert!(terminal.terminal_is_empty());

        let generation = 8_002;
        let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("cancel")), generation).expect("admitted");
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            cancel,
            semio_framework_job::default_now_ms,
            &mut sequence,
        );
        assert_eq!(job.drive_one(&mut context), SurfaceReconcileJobStep::Fault);
        let mut terminal = job.into_terminal();
        assert!(matches!(terminal.fault(), Some(SurfaceReconcileFault::Cancelled)));
        for _ in 0..32 {
            if terminal.close_step() && terminal.terminal_is_empty() {
                break;
            }
        }
        assert!(terminal.terminal_is_empty());
    }
    //#endregion ⏭️ResumableCursor

    //#region 🔖️FirstReconcileAndIdempotence
    #[test]
    fn first_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent() {
        let mut reconciler = SurfaceReconciler::new("s");
        let component_tree = tree(container("root", vec![leaf("a"), leaf("b")]));

        let patch = reconciler.reconcile(&component_tree).expect("first reconcile must emit a patch");
        assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
        assert_eq!(patch.revision, ui_contract::UiRevision(1));
        assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(_))).count(), 3);
        assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::SetRoot { .. })).count(), 1);

        assert!(reconciler.reconcile(&component_tree).is_none(), "an unchanged tree must emit no patch");
    }
    //#endregion 🔖️FirstReconcileAndIdempotence

    //#region 🔖️TargetedOps
    #[test]
    fn changing_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![text("a", "hello"), leaf("b")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![text("a", "world"), leaf("b")]))).expect("a changed leaf must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetComponent { id, component } => {
                assert_eq!(*id, target_id);
                assert_eq!(component, &ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::from("world"), emphasize: None, data_attributes: None }));
            }
            other => panic!("expected SetComponent (not Upsert), got {other:?}"),
        }
    }

    #[test]
    fn reordering_siblings_preserves_every_id_and_emits_only_set_children() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("b"), leaf("c")]))).unwrap();
        let before = reconciler.snapshot();
        let (a_id, b_id, c_id) = (id_of(&before, "a"), id_of(&before, "b"), id_of(&before, "c"));

        let patch = reconciler.reconcile(&tree(container("root", vec![leaf("c"), leaf("a"), leaf("b")]))).expect("a reorder must emit a patch");
        assert_eq!(patch.ops.len(), 1);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetChildren { children, .. } => assert_eq!(children, &vec![c_id, a_id, b_id]),
            other => panic!("expected SetChildren, got {other:?}"),
        }

        let after = reconciler.snapshot();
        assert_eq!(id_of(&after, "a"), a_id);
        assert_eq!(id_of(&after, "b"), b_id);
        assert_eq!(id_of(&after, "c"), c_id);
    }

    #[test]
    fn inserting_a_middle_sibling_preserves_the_others_ids() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("c")]))).unwrap();
        let before = reconciler.snapshot();
        let (a_id, c_id) = (id_of(&before, "a"), id_of(&before, "c"));

        let patch = reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("b"), leaf("c")]))).expect("an insertion must emit a patch");
        assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::Upsert(record) if record.key == "b")));

        let after = reconciler.snapshot();
        assert_eq!(id_of(&after, "a"), a_id);
        assert_eq!(id_of(&after, "c"), c_id);
    }

    #[test]
    fn changed_component_with_unchanged_layout_emits_set_component_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();

        let patch = reconciler.reconcile(&tree(container("root", vec![text("a", "now text")]))).expect("a component change must emit a patch");
        assert_eq!(patch.ops.len(), 1);
        assert!(matches!(patch.ops[0], ui_contract::UiPatchOp::SetComponent { .. }), "expected SetComponent, got {:?}", patch.ops[0]);
    }

    /// 🎨️ The finding this packet exists to fix: a style-only change on a leaf with everything else
    /// unchanged must emit exactly one `SetStyle`, never a whole-node `Upsert`.
    #[test]
    fn changing_only_style_emits_exactly_one_set_style_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![styled(leaf("a"), ui_contract::Tone::Danger)]))).expect("a style change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetStyle { id, style } => {
                assert_eq!(*id, target_id);
                assert_eq!(style.tone, ui_contract::Tone::Danger);
            }
            other => panic!("expected SetStyle (not Upsert), got {other:?}"),
        }
    }

    #[test]
    fn changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![with_shortcut(leaf("a"), "Ctrl+S")]))).expect("an accessibility change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetAccessibility { id, accessibility } => {
                assert_eq!(*id, target_id);
                assert_eq!(accessibility.shortcut.as_deref(), Some("Ctrl+S"));
            }
            other => panic!("expected SetAccessibility (not Upsert), got {other:?}"),
        }
    }

    #[test]
    fn changing_only_bindings_emits_exactly_one_set_bindings_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![with_binding(leaf("a"), "scope", "name")]))).expect("a bindings change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetBindings { id, bindings } => {
                assert_eq!(*id, target_id);
                assert_eq!(bindings.len(), 1);
            }
            other => panic!("expected SetBindings (not Upsert), got {other:?}"),
        }
    }

    #[test]
    fn changing_only_menu_emits_exactly_one_set_menu_not_upsert() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let patch = reconciler.reconcile(&tree(container("root", vec![with_menu(leaf("a"), "menu")]))).expect("a menu change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::SetMenu { id, menu } => {
                assert_eq!(*id, target_id);
                assert_eq!(menu.as_ref().map(|menu| menu.id.as_str()), Some("menu"));
            }
            other => panic!("expected SetMenu (not Upsert), got {other:?}"),
        }
    }

    /// 💰️ Once several groups change at once, [`SurfaceReconciler::estimate_bytes`] weighs a single
    /// `Upsert` against the pile of targeted ops it would replace — here five groups change on a leaf
    /// whose new component/accessibility still carry no real text, so the targeted ops' fixed per-op
    /// overhead alone outweighs one full-record `Upsert`, and `Upsert` wins.
    #[test]
    fn changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let target_id = id_of(&reconciler.snapshot(), "a");

        let mut changed =
            crate::TreeNode::new("a", ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }));
        changed.style = ui_contract::StyleSpec { tone: ui_contract::Tone::Danger, ..Default::default() };
        changed.activity = ui_contract::Activity::Loading;
        changed.disabled = true;
        changed.accessibility = ui_contract::AccessibilitySpec { hidden: true, ..Default::default() };

        let patch = reconciler.reconcile(&tree(container("root", vec![changed]))).expect("a multi-group change must emit a patch");
        assert_eq!(patch.ops.len(), 1, "expected one Upsert to beat several targeted ops, got {:?}", patch.ops);
        match &patch.ops[0] {
            ui_contract::UiPatchOp::Upsert(record) => assert_eq!(record.id, target_id),
            other => panic!("expected Upsert, got {other:?}"),
        }
    }
    //#endregion 🔖️TargetedOps

    //#region 🔖️Removal
    #[test]
    fn removing_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![container("mid", vec![leaf("x"), leaf("y")]), leaf("z")]))).unwrap();
        let mid_id = id_of(&reconciler.snapshot(), "mid");

        let patch = reconciler.reconcile(&tree(container("root", vec![leaf("z")]))).expect("a removal must emit a patch");
        let removes: Vec<_> = patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Remove { .. })).collect();
        assert_eq!(removes.len(), 1);
        assert!(matches!(removes[0], ui_contract::UiPatchOp::Remove { id } if *id == mid_id));

        let after = reconciler.snapshot();
        assert!(!after.nodes.iter().any(|record| matches!(record.key.as_str(), "mid" | "x" | "y")), "removed subtree must leave no orphan");
        assert_eq!(after.nodes.len(), 2, "only root and z should remain");
    }

    #[test]
    fn ids_are_never_reused_after_removal() {
        let mut reconciler = SurfaceReconciler::new("s");
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let removed_id = id_of(&reconciler.snapshot(), "a");

        reconciler.reconcile(&tree(container("root", vec![]))).unwrap();
        reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
        let reinserted_id = id_of(&reconciler.snapshot(), "a");

        assert_ne!(reinserted_id, removed_id, "a fresh node at a previously-used key must never reuse a removed id");
    }
    //#endregion 🔖️Removal

    //#region 🔖️Rejection
    #[test]
    fn mark_rejected_then_reconcile_emits_a_full_resend() {
        let mut reconciler = SurfaceReconciler::new("s");
        let component_tree = tree(container("root", vec![leaf("a"), leaf("b")]));
        reconciler.reconcile(&component_tree).unwrap();
        assert!(reconciler.reconcile(&component_tree).is_none());

        reconciler.mark_rejected();
        let patch = reconciler.reconcile(&component_tree).expect("resend after rejection must emit a patch");
        assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
        assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(_))).count(), 3);
        assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::SetRoot { .. })));
    }
    //#endregion 🔖️Rejection

    //#region 🔖️DuplicateKeys
    #[test]
    fn duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed() {
        let mut reconciler = SurfaceReconciler::new("s");
        let root = crate::TreeNode { children: vec![leaf("a"), leaf("a")], ..leaf("root") };
        let component_tree = crate::ComponentTree { root };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reconciler.reconcile(&component_tree)));
        assert!(result.is_err(), "a duplicate sibling key must panic, not silently shadow");
    }
    //#endregion 🔖️DuplicateKeys

    //#region 🔖️RoundTripProperty
    /// 🔁️ The property that matters most: every patch this reconciler ever emits must apply cleanly
    /// through the contract's own [`ui_contract::apply_patch`], and doing so must reproduce
    /// [`SurfaceReconciler::snapshot`] exactly. Exercised across a sequence of trees that each mutate a
    /// different axis (reorder, insert, remove, text change, nested restructure, collapse-to-one-child,
    /// style-only, accessibility-only, bindings-only, menu-only, and a multi-group change that should
    /// fall back to `Upsert`) so this one test would catch a producer/consumer disagreement in any of
    /// them — including the four field-targeted ops this packet adds.
    #[test]
    fn round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot() {
        let mut reconciler = SurfaceReconciler::new("s");
        let mut receiver_state = ui_contract::UiSnapshotState::new(ui_contract::SurfaceId::from("s"));
        let limits = ui_contract::UiDocumentLimits::default();

        let frames = vec![
            tree(container("root", vec![leaf("a"), leaf("b")])),
            tree(container("root", vec![leaf("b"), leaf("a"), text("c", "hi")])),
            tree(container("root", vec![text("c", "bye"), container("mid", vec![leaf("d")])])),
            tree(container("root", vec![container("mid", vec![leaf("d"), leaf("e")])])),
            tree(container("root", vec![leaf("solo")])),
            tree(container("root", vec![styled(leaf("solo"), ui_contract::Tone::Primary)])),
            tree(container("root", vec![with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K")])),
            tree(container("root", vec![with_binding(with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K"), "scope", "name")])),
            tree(container("root", vec![with_menu(with_binding(with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K"), "scope", "name"), "menu")])),
            tree(container("root", vec![leaf("solo"), leaf("solo2")])),
        ];

        for component_tree in &frames {
            if let Some(patch) = reconciler.reconcile(component_tree) {
                ui_contract::apply_patch(&mut receiver_state, &patch, &limits).expect("every emitted patch must apply cleanly against the contract's own apply_patch");
                ui_contract::validate_snapshot(&reconciler.snapshot(), &limits).expect("every reconciled state must remain a valid document");
            }
            assert_snapshot_matches_state(&reconciler.snapshot(), &receiver_state);
        }
    }
    //#endregion 🔖️RoundTripProperty
}
//#endregion 🧪️Tests
