//! @emoji 🛡️ Quotas, `validate_snapshot`, and the one shared transactional `apply_patch`.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! `apply_patch` is the single most important function in the crate: the React DOM store, the GPU
//! frontend, and every future renderer all apply patches through this exact code, which is what makes
//! them agree. It is totally transactional — ops apply to a shadow draft, never to the caller's state,
//! and on ANY rejection the caller's state is byte-for-byte unchanged. Untrusted plugin documents flow
//! through here, so [`UiDocumentLimits`] are a security boundary, not a nicety: an oversized or
//! malformed document is rejected before it costs more than an O(1) or O(node count) check.

use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};

//#region 🔖️Limits
/// 🛡️ Quotas a [`crate::UiSnapshot`]/[`crate::UiPatch`] must stay within. `max_nodes`/`max_depth` bound
/// the document shape and are enforced by [`validate_snapshot`] (surfaced as
/// [`UiContractViolation::NodeQuota`]/[`UiContractViolation::DepthQuota`]); `max_children`/
/// `max_text_bytes`/`max_patch_ops`/`max_patch_bytes` bound one incoming [`crate::UiPatch`] and are
/// enforced directly by [`apply_patch`] (surfaced as [`PatchRejection::QuotaExceeded`]) — rejecting a
/// patch before it is even applied to the shadow draft is cheaper than discovering the violation after.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiDocumentLimits {
    /// 📦️ Total live nodes in one surface. 20 000 comfortably covers the largest known tree (a fully
    /// expanded product tree view or timeline) with headroom, while still bounding a malicious
    /// plugin's flood well below where a `HashMap<UiNodeId, UiNodeRecord>` becomes a memory concern.
    pub max_nodes: usize,
    /// 📏️ Deepest legal node-to-root chain. 128 is far beyond any legitimate UI nesting (the deepest
    /// real shape, a `Tree`/`TreeSection`/`TreeItem` chain, rarely exceeds a few dozen) and doubles as
    /// the traversal's own recursion-depth bound, so it is also the security property that keeps
    /// `validate_snapshot`'s stack-free walk cheap even under adversarial input.
    pub max_depth: usize,
    /// 👶️ Direct children on one node. 4 096 covers the largest legitimate flat list (an unpaginated
    /// tree section or a large `Select`-like listing rendered as children) without letting one node
    /// alone approach `max_nodes`.
    pub max_children: usize,
    /// 📝️ UTF-8 bytes in one component's own text-bearing fields (label/description/value/…). 64 KiB
    /// is generous for authored UI copy (far beyond a label or even a long description) while refusing
    /// to let a single component smuggle an arbitrarily large string through the contract.
    pub max_text_bytes: usize,
    /// 🩹️ Ops in one [`crate::UiPatch`]. 4 096 mirrors `max_children`'s order of magnitude — no
    /// legitimate single reconciliation pass should need more ops than the largest single-node fan-out
    /// this crate already permits.
    pub max_patch_ops: usize,
    /// 📮️ Estimated wire bytes for one [`crate::UiPatch`] (see [`patch_byte_estimate`]). 1 MiB matches
    /// a conservative single-frame transport budget — large enough for a full-surface `Upsert` burst,
    /// small enough that a malicious patch cannot exhaust an actor mailbox in one message.
    pub max_patch_bytes: usize,
}

impl Default for UiDocumentLimits {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn default() -> Self {
        Self { max_nodes: 20_000, max_depth: 128, max_children: 4_096, max_text_bytes: 65_536, max_patch_ops: 4_096, max_patch_bytes: 1_048_576 }
    }
}

/// 🧮️ A rough, dependency-free proxy for a patch's wire cost — this crate has no `pack`/serde-json
/// runtime dependency (see `📦️glue.rs`'s dependency-free guarantee), so this sums UTF-8 byte lengths
/// of the ops' own text-bearing payloads plus a small fixed per-op overhead, rather than actually
/// encoding the patch.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn patch_byte_estimate(patch: &crate::UiPatch) -> usize {
    const OP_OVERHEAD_BYTES: usize = 16;
    patch.ops.iter().map(|op| OP_OVERHEAD_BYTES + op_text_bytes(op)).sum()
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn op_text_bytes(op: &crate::UiPatchOp) -> usize {
    match op {
        crate::UiPatchOp::Upsert(record) => record.key.len() + component_text_bytes(&record.component) + accessibility_text_bytes(&record.accessibility) + bindings_text_bytes(&record.bindings) + menu_text_bytes(&record.menu),
        crate::UiPatchOp::SetComponent { component, .. } => component_text_bytes(component),
        crate::UiPatchOp::SetChildren { children, .. } => children.len() * size_of::<crate::UiNodeId>(),
        crate::UiPatchOp::SetAccessibility { accessibility, .. } => accessibility_text_bytes(accessibility),
        crate::UiPatchOp::SetBindings { bindings, .. } => bindings_text_bytes(bindings),
        crate::UiPatchOp::SetMenu { menu, .. } => menu_text_bytes(menu),
        crate::UiPatchOp::SetLayout { .. } | crate::UiPatchOp::SetActivity { .. } | crate::UiPatchOp::SetStyle { .. } | crate::UiPatchOp::Remove { .. } | crate::UiPatchOp::SetRoot { .. } => 0,
    }
}

/// 📝️ UTF-8 bytes of `spec`'s own text-bearing fields (`label`/`description`/`shortcut`) — mirrors
/// [`component_text_bytes`]'s own scope-limited accounting for the same reason: these are the fields
/// most likely to carry a large user-authored string.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn accessibility_text_bytes(spec: &crate::AccessibilitySpec) -> usize {
    label_bytes(&spec.label) + label_bytes(&spec.description) + spec.shortcut.as_deref().map_or(0, str::len)
}

/// 📝️ UTF-8 bytes of every binding's own `action` identity — `scope`/`name` are the only text an
/// [`crate::ActionBinding`] carries outside its opaque, non-text-bearing `args`/`capability` payload.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn bindings_text_bytes(bindings: &crate::UiNodeBindings) -> usize {
    bindings.iter().map(|binding| binding.action.scope.len() + binding.action.name.len() + binding.capability.as_deref().map_or(0, str::len)).sum()
}

/// 📝️ UTF-8 bytes of `menu`'s own `id` — its only text-bearing field outside the opaque `args`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn menu_text_bytes(menu: &Option<crate::MenuRef>) -> usize {
    menu.as_ref().map_or(0, |menu_ref| menu_ref.id.len())
}

/// 📝️ UTF-8 bytes of `component`'s own text-bearing fields — the fields most likely to carry large
/// user-authored strings (labels, descriptions, values, placeholders). Not exhaustive over every field
/// of every variant; icon keys and other short identifiers are not text content this quota guards.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn component_text_bytes(component: &crate::Component) -> usize {
    use crate::Component::*;
    match component {
        Container(props) => label_bytes(&props.label) + props.description.as_deref().map_or(0, str::len) + props.error.as_deref().map_or(0, str::len),
        Text(props) => props.value.0.len(),
        Button(props) => props.label.0.len(),
        Input(props) => props.value.len() + label_bytes(&props.placeholder),
        Select(props) => props.items.iter().map(|item| item.label.0.len()).sum::<usize>() + label_bytes(&props.placeholder),
        Toggle(props) => label_bytes(&props.text),
        KeyValueList(props) => props.entries.iter().map(|entry| entry.label.0.len() + entry.value.len()).sum(),
        TreeSection(props) => label_bytes(&props.label),
        TreeItem(props) => props.label.0.len() + props.description.as_deref().map_or(0, str::len),
        Image(props) => label_bytes(&props.alt),
        Extension(props) => props.extension.len(),
        Separator(_) | Slider(_) | NumberStepper(_) | Ring(_) | IconSelect(_) | Tree(_) | Surface(_) => 0,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn label_bytes(label: &Option<crate::Label>) -> usize {
    label.as_ref().map_or(0, |label| label.0.len())
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn component_is_finite(component: &crate::Component) -> bool {
    match component {
        crate::Component::Slider(props) => [props.value, props.min, props.max, props.step].into_iter().all(f64::is_finite),
        crate::Component::NumberStepper(props) => [props.value, props.step].into_iter().all(f64::is_finite),
        crate::Component::Ring(props) => props.t.is_finite(),
        crate::Component::Input(props) => [props.min, props.max, props.step].into_iter().flatten().all(|value| value.is_finite()),
        _ => true,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_section(component: &crate::Component) -> bool {
    matches!(component, crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Section, .. }))
}
//#endregion 🔖️Limits

//#region 🔖️Validate
/// ⚠️ One structural invariant a [`crate::UiSnapshot`] fails — every variant here is a whole-document
/// shape property, never a per-patch wire quota (those are [`PatchRejection::QuotaExceeded`]).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiContractViolation {
    CensusCapacity,
    /// 🔁️ `node` is reachable from itself by following `children` — a document must be a tree.
    Cycle { node: crate::UiNodeId },
    /// 🧩️ `parent`'s `children` names `child`, but no record with that id exists.
    OrphanChild { parent: crate::UiNodeId, child: crate::UiNodeId },
    /// 👯️ Two of `parent`'s children share `key` — reconciliation keys must be unique among siblings.
    DuplicateSiblingKey { parent: crate::UiNodeId, key: crate::UiText },
    /// 📦️ `count` live nodes exceeds [`UiDocumentLimits::max_nodes`] (`max`).
    NodeQuota { count: usize, max: usize },
    /// 📏️ `node` sits `depth` edges below the root, exceeding [`UiDocumentLimits::max_depth`] (`max`).
    DepthQuota { node: crate::UiNodeId, depth: usize, max: usize },
    /// 🪢️ `node` exists in the node table but is not reachable from the root by following
    /// `children` — including the degenerate case where the root itself names an id with no record.
    DanglingRoot { node: crate::UiNodeId },
    /// 🗂️ `node` is a `Container` with `role: Section` nested inside another `Section` — sectioning is
    /// intentionally flat, one level, so a renderer never has to resolve ambiguous nested chrome.
    SectionNested { node: crate::UiNodeId },
    /// 🔢️ `node`'s component carries a NaN or infinite numeric field.
    NonFiniteNumber { node: crate::UiNodeId },
}

/// 🌲️ Validates `snapshot` against `limits`, collecting every [`UiContractViolation`] found rather
/// than stopping at the first — a fuzz corpus or a UI diagnostic wants the whole picture. Returns
/// `Ok(())` only when the document is entirely clean. Short-circuits to just [`UiContractViolation::NodeQuota`]
/// when the node count alone already exceeds the quota, since walking an already-oversized untrusted
/// document is itself part of the attack surface `max_nodes` exists to cut off.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub const UI_DOCUMENT_VIOLATIONS: usize = crate::UI_DOCUMENT_NODES * crate::UI_DOCUMENT_NODES + crate::UI_DOCUMENT_NODES * 5 + 1;
pub type UiContractViolations = crate::UiFixedList<UiContractViolation, UI_DOCUMENT_VIOLATIONS>;

#[cfg(test)]
pub fn validate_snapshot(snapshot: &crate::UiSnapshot, limits: &UiDocumentLimits) -> Result<(), UiContractViolations> {
    to_result(validate_core(Some(snapshot.root), snapshot.nodes.len(), |id| snapshot.nodes.iter().find(|record| record.id == id), snapshot.nodes.iter(), limits))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
fn validate_state(state: &crate::UiSnapshotState, limits: &UiDocumentLimits) -> Result<(), UiContractViolations> {
    to_result(validate_core(state.root, state.nodes.len(), |id| state.nodes.get(&id), state.nodes.values(), limits))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
fn to_result(violations: UiContractViolations) -> Result<(), UiContractViolations> {
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

/// 🌲️ One stack frame of the iterative preorder walk `validate_core` runs — explicit, not recursive,
/// so a pathological (but within-quota) tree cannot exhaust the native stack; `Exit` pops the node back
/// off the ancestor path once its subtree is fully processed, which is what makes [`Cycle`] detection
/// (a back-edge into a node still on that path) distinguishable from a node merely visited twice.
///
/// [`Cycle`]: UiContractViolation::Cycle
#[cfg(test)]
enum WalkFrame {
    Enter(crate::UiNodeId, usize, bool),
    Exit(crate::UiNodeId),
}

/// 🌲️ The shared traversal behind [`validate_snapshot`] and [`apply_patch`]'s post-op check — generic
/// over `V: Borrow<UiNodeRecord>` so it runs unmodified against a [`crate::UiSnapshot`]'s borrowed
/// `HashMap<UiNodeId, &UiNodeRecord>` and a [`crate::UiSnapshotState`]'s owned
/// `HashMap<UiNodeId, UiNodeRecord>` alike — one algorithm, never two copies to keep in sync.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
fn validate_core<'a>(
    root: Option<crate::UiNodeId>,
    node_count: usize,
    mut get: impl FnMut(crate::UiNodeId) -> Option<&'a crate::UiNodeRecord>,
    records: impl Iterator<Item = &'a crate::UiNodeRecord>,
    limits: &UiDocumentLimits,
) -> UiContractViolations {
    let mut violations = UiContractViolations::default();
    if node_count > limits.max_nodes {
        let _ = violations.try_push(UiContractViolation::NodeQuota { count: node_count, max: limits.max_nodes });
        return violations;
    }

    let mut visited = crate::UiFixedList::<crate::UiNodeId, { crate::UI_DOCUMENT_NODES }>::default();
    let mut on_path = crate::UiFixedList::<crate::UiNodeId, { crate::UI_DOCUMENT_NODES }>::default();

    if let Some(root_id) = root {
        if get(root_id).is_some() {
            let mut stack = crate::UiFixedList::<WalkFrame, UI_DOCUMENT_VIOLATIONS>::default();
            if stack.try_push(WalkFrame::Enter(root_id, 0, false)).is_err() {
                let _ = violations.try_push(UiContractViolation::CensusCapacity);
                return violations;
            }
            while let Some(frame) = stack.pop() {
                match frame {
                    WalkFrame::Exit(id) => {
                        if on_path.iter().last().is_some_and(|candidate| *candidate == id) {
                            let _ = on_path.pop();
                        }
                    }
                    WalkFrame::Enter(id, depth, parent_in_section) => {
                        if on_path.iter().any(|candidate| *candidate == id) {
                            if violations.try_push(UiContractViolation::Cycle { node: id }).is_err() {
                                return violations;
                            }
                            continue;
                        }
                        if visited.iter().any(|candidate| *candidate == id) {
                            continue;
                        }
                        if visited.try_push(id).is_err() {
                            let _ = violations.try_push(UiContractViolation::CensusCapacity);
                            return violations;
                        }
                        let Some(record) = get(id) else { continue };

                        let in_section = parent_in_section || is_section(&record.component);
                        if parent_in_section && is_section(&record.component) {
                            if violations.try_push(UiContractViolation::SectionNested { node: id }).is_err() { return violations; }
                        }
                        if !component_is_finite(&record.component) {
                            if violations.try_push(UiContractViolation::NonFiniteNumber { node: id }).is_err() { return violations; }
                        }
                        if depth > limits.max_depth {
                            if violations.try_push(UiContractViolation::DepthQuota { node: id, depth, max: limits.max_depth }).is_err() { return violations; }
                            continue;
                        }

                        if on_path.try_push(id).is_err() || stack.try_push(WalkFrame::Exit(id)).is_err() {
                            let _ = violations.try_push(UiContractViolation::CensusCapacity);
                            return violations;
                        }

                        let mut seen_keys = crate::UiFixedList::<&crate::UiText, { crate::UI_DOCUMENT_NODES }>::default();
                        for &child_id in &record.children {
                            match get(child_id) {
                                None => if violations.try_push(UiContractViolation::OrphanChild { parent: id, child: child_id }).is_err() { return violations; },
                                Some(child) => {
                                    if seen_keys.iter().any(|key| key.as_str() == child.key.as_str()) {
                                        if violations.try_push(UiContractViolation::DuplicateSiblingKey { parent: id, key: child.key.clone() }).is_err() { return violations; }
                                    } else if seen_keys.try_push(&child.key).is_err() {
                                        let _ = violations.try_push(UiContractViolation::CensusCapacity);
                                        return violations;
                                    }
                                    let Some(child_depth) = depth.checked_add(1) else {
                                        let _ = violations.try_push(UiContractViolation::CensusCapacity);
                                        return violations;
                                    };
                                    if stack.try_push(WalkFrame::Enter(child_id, child_depth, in_section)).is_err() {
                                        let _ = violations.try_push(UiContractViolation::CensusCapacity);
                                        return violations;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for record in records {
        if !visited.iter().any(|id| *id == record.id) {
            if violations.try_push(UiContractViolation::DanglingRoot { node: record.id }).is_err() { return violations; }
        }
    }
    violations
}
//#endregion 🔖️Validate

//#region 🔖️Apply
/// 🚫️ Why [`apply_patch`] rejected a [`crate::UiPatch`] — carries enough detail (both revisions, the
/// exceeded quota with its actual/max, or the full violation list) for the existing `patch-rejected`
/// wire event to explain itself and for the sender to resynchronise.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PatchRejection {
    AliasCapacity,
    StaleGeneration {
        expected: u64,
        actual: u64,
    },
    Cancelled,
    RevisionMismatch {
        expected: crate::UiRevision,
        actual: crate::UiRevision,
    },
    /// 🕳️ An op named a [`crate::UiNodeId`] that has no record in the receiver's current state — only
    /// possible for ops that mutate an existing node ([`crate::UiPatchOp::SetComponent`]/
    /// [`crate::UiPatchOp::SetLayout`]/[`crate::UiPatchOp::SetActivity`]/
    /// [`crate::UiPatchOp::SetChildren`]/[`crate::UiPatchOp::SetStyle`]/
    /// [`crate::UiPatchOp::SetAccessibility`]/[`crate::UiPatchOp::SetBindings`]/
    /// [`crate::UiPatchOp::SetMenu`]); `Upsert`/`Remove`/`SetRoot` never fail this way.
    UnknownNode {
        id: crate::UiNodeId,
    },
    QuotaExceeded {
        quota: QuotaKind,
        actual: usize,
        max: usize,
    },
    InvariantViolated {
        violations: UiContractViolations,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UiPatchApplyPhase {
    CensusPatch,
    CloneState,
    ApplyOps,
    RemoveSubtree,
    Validate,
    Ready,
    Rejected,
}

pub const UI_PATCH_APPLY_SLOTS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UiPatchApplyHandle {
    slot: usize,
    epoch: u64,
    generation: u64,
}

struct UiPatchRetirement {
    first_state: Option<crate::UiSnapshotState>,
    second_state: Option<crate::UiSnapshotState>,
    patch: Option<crate::UiPatch>,
    record: Option<crate::UiNodeRecord>,
    seen: Option<[crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES]>,
    seen_cursor: usize,
}

impl UiPatchRetirement {
    fn new(
        first_state: Option<crate::UiSnapshotState>,
        second_state: Option<crate::UiSnapshotState>,
        patch: Option<crate::UiPatch>,
        record: Option<crate::UiNodeRecord>,
        seen: [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    ) -> Self {
        Self { first_state, second_state, patch, record, seen: Some(seen), seen_cursor: 0 }
    }

    fn retire_one(&mut self) -> bool {
        if !retire_snapshot_one(&mut self.first_state) {
            return false;
        }
        if !retire_snapshot_one(&mut self.second_state) {
            return false;
        }
        if !retire_patch_one(&mut self.patch) {
            return false;
        }
        if self.record.take().is_some() {
            return false;
        }
        let Some(seen) = self.seen.as_mut() else { return true };
        if !retire_validation_seen_one(seen, &mut self.seen_cursor) {
            return false;
        }
        self.seen.take();
        false
    }
}

struct UiPatchApplySlot {
    epoch: u64,
    generation: u64,
    occupied: bool,
    retirement: Option<UiPatchRetirement>,
}

impl Default for UiPatchApplySlot {
    fn default() -> Self {
        Self { epoch: 0, generation: 0, occupied: false, retirement: None }
    }
}

struct UiPatchApplyArena {
    slots: [UiPatchApplySlot; UI_PATCH_APPLY_SLOTS],
    close_cursor: usize,
}

impl Default for UiPatchApplyArena {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| UiPatchApplySlot::default()), close_cursor: 0 }
    }
}

impl UiPatchApplyArena {
    fn reserve(&mut self, generation: u64) -> Option<UiPatchApplyHandle> {
        let slot = self.slots.iter().position(|slot| !slot.occupied)?;
        let epoch = self.slots[slot].epoch.checked_add(1)?;
        self.slots[slot] = UiPatchApplySlot { epoch, generation, occupied: true, retirement: None };
        Some(UiPatchApplyHandle { slot, epoch, generation })
    }

    fn slot_mut(&mut self, handle: UiPatchApplyHandle) -> Option<&mut UiPatchApplySlot> {
        let slot = self.slots.get_mut(handle.slot)?;
        (slot.occupied && slot.epoch == handle.epoch && slot.generation == handle.generation).then_some(slot)
    }

    fn handback(&mut self, handle: UiPatchApplyHandle, retirement: UiPatchRetirement) {
        if let Some(slot) = self.slot_mut(handle) {
            slot.retirement = Some(retirement);
        }
    }

    fn release(&mut self, handle: UiPatchApplyHandle) {
        let Some(slot) = self.slot_mut(handle) else { return };
        if slot.retirement.is_some() {
            return;
        }
        let epoch = slot.epoch;
        *slot = UiPatchApplySlot { epoch, ..UiPatchApplySlot::default() };
    }

    fn retire_one(&mut self) {
        for offset in 0..UI_PATCH_APPLY_SLOTS {
            let index = (self.close_cursor + offset) % UI_PATCH_APPLY_SLOTS;
            if !self.slots[index].occupied || self.slots[index].retirement.is_none() {
                continue;
            }
            self.close_cursor = (index + 1) % UI_PATCH_APPLY_SLOTS;
            let complete = self.slots[index].retirement.as_mut().is_some_and(UiPatchRetirement::retire_one);
            if complete {
                let epoch = self.slots[index].epoch;
                self.slots[index] = UiPatchApplySlot { epoch, ..UiPatchApplySlot::default() };
            }
            return;
        }
    }

    fn has_retirement(&self) -> bool {
        self.slots.iter().any(|slot| slot.occupied && slot.retirement.is_some())
    }
}

static UI_PATCH_APPLY_ARENA: LazyLock<Mutex<UiPatchApplyArena>> = LazyLock::new(|| Mutex::new(UiPatchApplyArena::default()));

fn with_ui_patch_apply_arena<T>(f: impl FnOnce(&mut UiPatchApplyArena) -> T) -> T {
    let mut arena = UI_PATCH_APPLY_ARENA.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut arena)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiPatchApplyStep {
    MoreWork,
    Ready,
    Rejected,
}

#[derive(Clone, Copy, Debug)]
struct UiPatchValidateFrame {
    id: crate::UiNodeId,
    depth: usize,
    parent_in_section: bool,
    in_section: bool,
    child: usize,
    stage: u8,
}

#[derive(Debug)]
pub struct UiPatchApplyProducer {
    retirement_handle: Option<UiPatchApplyHandle>,
    generation: u64,
    limits: UiDocumentLimits,
    original: Option<crate::UiSnapshotState>,
    draft: Option<crate::UiSnapshotState>,
    patch: Option<crate::UiPatch>,
    phase: UiPatchApplyPhase,
    patch_bytes: usize,
    next_node: usize,
    next_op: usize,
    remove_stack: crate::UiFixedList<crate::UiNodeId, { crate::UI_DOCUMENT_NODES }>,
    remove_record: Option<crate::UiNodeRecord>,
    remove_child: usize,
    validation_started: bool,
    validation_stack: crate::UiFixedList<UiPatchValidateFrame, { crate::UI_DOCUMENT_NODES }>,
    validation_visited: crate::UiFixedList<crate::UiNodeId, { crate::UI_DOCUMENT_NODES }>,
    validation_path: crate::UiFixedList<crate::UiNodeId, { crate::UI_DOCUMENT_NODES }>,
    validation_seen: [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    validation_dangling: usize,
    rejection: Option<PatchRejection>,
}

impl UiPatchApplyProducer {
    pub fn try_new(state: crate::UiSnapshotState, patch: crate::UiPatch, limits: UiDocumentLimits, generation: u64) -> Result<Self, UiPatchApplyRejected> {
        let retirement_handle = with_ui_patch_apply_arena(|arena| arena.reserve(generation));
        if retirement_handle.is_none() {
            return Err(UiPatchApplyRejected::new(None, generation, state, patch, PatchRejection::AliasCapacity));
        }
        if generation == 0 {
            return Err(UiPatchApplyRejected::new(retirement_handle, generation, state, patch, PatchRejection::StaleGeneration { expected: 1, actual: 0 }));
        }
        if patch.base_revision != state.revision {
            let rejection = PatchRejection::RevisionMismatch { expected: state.revision, actual: patch.base_revision };
            return Err(UiPatchApplyRejected::new(retirement_handle, generation, state, patch, rejection));
        }
        if patch.ops.len() > limits.max_patch_ops {
            let rejection = PatchRejection::QuotaExceeded { quota: QuotaKind::PatchOps, actual: patch.ops.len(), max: limits.max_patch_ops };
            return Err(UiPatchApplyRejected::new(retirement_handle, generation, state, patch, rejection));
        }
        let draft = crate::UiSnapshotState::new(state.surface.clone());
        Ok(Self {
            retirement_handle,
            generation,
            limits,
            original: Some(state),
            draft: Some(draft),
            patch: Some(patch),
            phase: UiPatchApplyPhase::CensusPatch,
            patch_bytes: 0,
            next_node: 0,
            next_op: 0,
            remove_stack: crate::UiFixedList::default(),
            remove_record: None,
            remove_child: 0,
            validation_started: false,
            validation_stack: crate::UiFixedList::default(),
            validation_visited: crate::UiFixedList::default(),
            validation_path: crate::UiFixedList::default(),
            validation_seen: std::array::from_fn(|_| crate::UiFixedList::default()),
            validation_dangling: 0,
            rejection: None,
        })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn rejection(&self) -> Option<&PatchRejection> {
        self.rejection.as_ref()
    }

    pub fn drive_one(&mut self, generation: u64, cancelled: bool, deadline_expired: bool) -> UiPatchApplyStep {
        if self.phase == UiPatchApplyPhase::Ready {
            return UiPatchApplyStep::Ready;
        }
        if self.phase == UiPatchApplyPhase::Rejected {
            return UiPatchApplyStep::Rejected;
        }
        if generation != self.generation {
            self.reject(PatchRejection::StaleGeneration { expected: self.generation, actual: generation });
            return UiPatchApplyStep::Rejected;
        }
        if cancelled {
            self.reject(PatchRejection::Cancelled);
            return UiPatchApplyStep::Rejected;
        }
        if deadline_expired {
            return UiPatchApplyStep::MoreWork;
        }
        match self.phase {
            UiPatchApplyPhase::CensusPatch => self.census_patch_one(),
            UiPatchApplyPhase::CloneState => self.clone_state_one(),
            UiPatchApplyPhase::ApplyOps => self.apply_op_one(),
            UiPatchApplyPhase::RemoveSubtree => self.remove_subtree_one(),
            UiPatchApplyPhase::Validate => self.validate_candidate_one(),
            UiPatchApplyPhase::Ready => UiPatchApplyStep::Ready,
            UiPatchApplyPhase::Rejected => UiPatchApplyStep::Rejected,
        }
    }

    fn census_patch_one(&mut self) -> UiPatchApplyStep {
        let Some(patch) = self.patch.as_ref() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        let Some(op) = patch.ops.get(self.next_op) else {
            self.phase = UiPatchApplyPhase::CloneState;
            return UiPatchApplyStep::MoreWork;
        };
        if let crate::UiPatchOp::Remove { id } = op {
            if let Err(id) = self.remove_stack.try_push(*id) {
                self.reject(PatchRejection::UnknownNode { id });
                return UiPatchApplyStep::Rejected;
            }
            let Some(next_op) = self.next_op.checked_add(1) else {
                self.reject(PatchRejection::AliasCapacity);
                return UiPatchApplyStep::Rejected;
            };
            self.next_op = next_op;
            self.phase = UiPatchApplyPhase::RemoveSubtree;
            return UiPatchApplyStep::MoreWork;
        }
        const OP_OVERHEAD_BYTES: usize = 16;
        let Some(bytes) = self.patch_bytes.checked_add(OP_OVERHEAD_BYTES).and_then(|bytes| bytes.checked_add(op_text_bytes(op))) else {
            self.reject(PatchRejection::QuotaExceeded { quota: QuotaKind::PatchBytes, actual: usize::MAX, max: self.limits.max_patch_bytes });
            return UiPatchApplyStep::Rejected;
        };
        if bytes > self.limits.max_patch_bytes {
            self.reject(PatchRejection::QuotaExceeded { quota: QuotaKind::PatchBytes, actual: bytes, max: self.limits.max_patch_bytes });
            return UiPatchApplyStep::Rejected;
        }
        self.patch_bytes = bytes;
        let Some(next_op) = self.next_op.checked_add(1) else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        self.next_op = next_op;
        UiPatchApplyStep::MoreWork
    }

    fn remove_subtree_one(&mut self) -> UiPatchApplyStep {
        if let Some(record) = self.remove_record.as_ref() {
            if let Some(child) = record.children.get(self.remove_child).copied() {
                if let Err(child) = self.remove_stack.try_push(child) {
                    self.reject(PatchRejection::UnknownNode { id: child });
                    return UiPatchApplyStep::Rejected;
                }
                let Some(remove_child) = self.remove_child.checked_add(1) else {
                    self.reject(PatchRejection::AliasCapacity);
                    return UiPatchApplyStep::Rejected;
                };
                self.remove_child = remove_child;
                return UiPatchApplyStep::MoreWork;
            }
            drop(self.remove_record.take());
            self.remove_child = 0;
            return UiPatchApplyStep::MoreWork;
        }
        let Some(id) = self.remove_stack.pop() else {
            self.phase = UiPatchApplyPhase::ApplyOps;
            return UiPatchApplyStep::MoreWork;
        };
        let Some(draft) = self.draft.as_mut() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        self.remove_record = draft.nodes.remove(&id);
        UiPatchApplyStep::MoreWork
    }

    fn clone_state_one(&mut self) -> UiPatchApplyStep {
        let Some(original) = self.original.as_ref() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        let Some(record) = original.nodes.get_index(self.next_node) else {
            let Some(draft) = self.draft.as_mut() else {
                self.reject(PatchRejection::AliasCapacity);
                return UiPatchApplyStep::Rejected;
            };
            draft.root = original.root;
            draft.revision = original.revision;
            self.next_op = 0;
            self.phase = UiPatchApplyPhase::ApplyOps;
            return UiPatchApplyStep::MoreWork;
        };
        let Some(record) = record.credited_clone() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        let Some(draft) = self.draft.as_mut() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        if let Err(record) = draft.nodes.try_insert(record) {
            drop(record);
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        }
        let Some(next_node) = self.next_node.checked_add(1) else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        self.next_node = next_node;
        UiPatchApplyStep::MoreWork
    }

    fn apply_op_one(&mut self) -> UiPatchApplyStep {
        let Some(patch) = self.patch.as_ref() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        let Some(op) = patch.ops.get(self.next_op) else {
            let Some(draft) = self.draft.as_mut() else {
                self.reject(PatchRejection::AliasCapacity);
                return UiPatchApplyStep::Rejected;
            };
            draft.revision = patch.revision;
            self.phase = UiPatchApplyPhase::Validate;
            return UiPatchApplyStep::MoreWork;
        };
        let Some(draft) = self.draft.as_mut() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        if let Err(rejection) = apply_op(draft, op, &self.limits) {
            self.reject(rejection);
            return UiPatchApplyStep::Rejected;
        }
        let Some(next_op) = self.next_op.checked_add(1) else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        self.next_op = next_op;
        UiPatchApplyStep::MoreWork
    }

    fn validate_candidate_one(&mut self) -> UiPatchApplyStep {
        let Some(draft) = self.draft.as_ref() else {
            self.reject(PatchRejection::AliasCapacity);
            return UiPatchApplyStep::Rejected;
        };
        if !self.validation_started {
            self.validation_started = true;
            if draft.nodes.len() > self.limits.max_nodes {
                return self.reject_violation(UiContractViolation::NodeQuota { count: draft.nodes.len(), max: self.limits.max_nodes });
            }
            if let Some(root) = draft.root.filter(|root| draft.nodes.get(root).is_some()) {
                let frame = UiPatchValidateFrame { id: root, depth: 0, parent_in_section: false, in_section: false, child: 0, stage: 0 };
                if self.validation_stack.try_push(frame).is_err() {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                }
            }
            return UiPatchApplyStep::MoreWork;
        }
        if let Some(frame) = self.validation_stack.len().checked_sub(1).and_then(|index| self.validation_stack.get(index)).copied() {
            if frame.stage == 0 {
                if self.validation_path.iter().any(|id| *id == frame.id) {
                    return self.reject_violation(UiContractViolation::Cycle { node: frame.id });
                }
                if self.validation_visited.iter().any(|id| *id == frame.id) {
                    let _ = self.validation_stack.pop();
                    return UiPatchApplyStep::MoreWork;
                }
                if self.validation_visited.try_push(frame.id).is_err() || self.validation_path.try_push(frame.id).is_err() {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                }
                let Some(record) = draft.nodes.get(&frame.id) else {
                    return self.reject_violation(UiContractViolation::DanglingRoot { node: frame.id });
                };
                if frame.parent_in_section && is_section(&record.component) {
                    return self.reject_violation(UiContractViolation::SectionNested { node: frame.id });
                }
                if !component_is_finite(&record.component) {
                    return self.reject_violation(UiContractViolation::NonFiniteNumber { node: frame.id });
                }
                if frame.depth > self.limits.max_depth {
                    return self.reject_violation(UiContractViolation::DepthQuota { node: frame.id, depth: frame.depth, max: self.limits.max_depth });
                }
                let Some(depth_index) = self.validation_stack.len().checked_sub(1) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                self.validation_seen[depth_index] = crate::UiFixedList::default();
                let Some(active) = self.validation_stack.get_mut(depth_index) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                active.in_section = frame.parent_in_section || is_section(&record.component);
                active.stage = 1;
                return UiPatchApplyStep::MoreWork;
            }
            if frame.stage == 1 {
                let Some(record) = draft.nodes.get(&frame.id) else {
                    return self.reject_violation(UiContractViolation::DanglingRoot { node: frame.id });
                };
                let Some(child_id) = record.children.get(frame.child).copied() else {
                    let Some(active_index) = self.validation_stack.len().checked_sub(1) else {
                        return self.reject_violation(UiContractViolation::CensusCapacity);
                    };
                    let Some(active) = self.validation_stack.get_mut(active_index) else {
                        return self.reject_violation(UiContractViolation::CensusCapacity);
                    };
                    active.stage = 2;
                    return UiPatchApplyStep::MoreWork;
                };
                let Some(child) = draft.nodes.get(&child_id) else {
                    return self.reject_violation(UiContractViolation::OrphanChild { parent: frame.id, child: child_id });
                };
                let Some(depth_index) = self.validation_stack.len().checked_sub(1) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                if self.validation_seen[depth_index].iter().any(|key| key.as_str() == child.key.as_str()) {
                    return self.reject_violation(UiContractViolation::DuplicateSiblingKey { parent: frame.id, key: child.key.clone() });
                }
                if self.validation_seen[depth_index].try_push(child.key.clone()).is_err() {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                }
                let Some(depth) = frame.depth.checked_add(1) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                let Some(active) = self.validation_stack.get_mut(depth_index) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                let Some(next_child) = active.child.checked_add(1) else {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                };
                active.child = next_child;
                let child_frame = UiPatchValidateFrame { id: child_id, depth, parent_in_section: frame.in_section, in_section: false, child: 0, stage: 0 };
                if self.validation_stack.try_push(child_frame).is_err() {
                    return self.reject_violation(UiContractViolation::CensusCapacity);
                }
                return UiPatchApplyStep::MoreWork;
            }
            let _ = self.validation_stack.pop();
            if self.validation_path.iter().last().is_some_and(|id| *id == frame.id) {
                let _ = self.validation_path.pop();
            }
            return UiPatchApplyStep::MoreWork;
        }
        if let Some(record) = draft.nodes.get_index(self.validation_dangling) {
            if !self.validation_visited.iter().any(|id| *id == record.id) {
                return self.reject_violation(UiContractViolation::DanglingRoot { node: record.id });
            }
            let Some(next) = self.validation_dangling.checked_add(1) else {
                return self.reject_violation(UiContractViolation::CensusCapacity);
            };
            self.validation_dangling = next;
            return UiPatchApplyStep::MoreWork;
        }
        self.phase = UiPatchApplyPhase::Ready;
        UiPatchApplyStep::Ready
    }

    fn reject_violation(&mut self, violation: UiContractViolation) -> UiPatchApplyStep {
        let mut violations = UiContractViolations::default();
        if violations.try_push(violation).is_err() {
            self.reject(PatchRejection::AliasCapacity);
        } else {
            self.reject(PatchRejection::InvariantViolated { violations });
        }
        UiPatchApplyStep::Rejected
    }

    fn reject(&mut self, rejection: PatchRejection) {
        self.rejection = Some(rejection);
        self.phase = UiPatchApplyPhase::Rejected;
    }

    pub fn take_ready(mut self) -> Result<UiPatchApplyOutcome, Self> {
        if self.phase != UiPatchApplyPhase::Ready || self.remove_record.is_some() {
            return Err(self);
        }
        let (Some(state), Some(previous), Some(patch)) = (self.draft.take(), self.original.take(), self.patch.take()) else { return Err(self) };
        let Some(retirement_handle) = self.retirement_handle.take() else { return Err(self) };
        let validation_seen = std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default()));
        Ok(UiPatchApplyOutcome {
            retirement_handle: Some(retirement_handle),
            generation: self.generation,
            state: Some(state),
            previous: Some(previous),
            patch: Some(patch),
            validation_seen,
            validation_seen_cursor: 0,
        })
    }

    pub fn take_rejected(mut self) -> Result<UiPatchApplyRejected, Self> {
        if self.phase != UiPatchApplyPhase::Rejected {
            return Err(self);
        }
        let (Some(state), Some(patch), Some(rejection)) = (self.original.take(), self.patch.take(), self.rejection.take()) else { return Err(self) };
        let Some(retirement_handle) = self.retirement_handle.take() else { return Err(self) };
        let validation_seen = std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default()));
        Ok(UiPatchApplyRejected {
            retirement_handle: Some(retirement_handle),
            generation: self.generation,
            state: Some(state),
            patch: Some(patch),
            draft: self.draft.take(),
            remove_record: self.remove_record.take(),
            rejection,
            retire_scalar: 0,
            validation_seen,
            validation_seen_cursor: 0,
        })
    }
}

impl Drop for UiPatchApplyProducer {
    fn drop(&mut self) {
        let Some(handle) = self.retirement_handle.take() else { return };
        let seen = std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default()));
        let retirement = UiPatchRetirement::new(self.original.take(), self.draft.take(), self.patch.take(), self.remove_record.take(), seen);
        with_ui_patch_apply_arena(|arena| arena.handback(handle, retirement));
    }
}

#[derive(Debug)]
pub struct UiPatchApplyOutcome {
    retirement_handle: Option<UiPatchApplyHandle>,
    generation: u64,
    state: Option<crate::UiSnapshotState>,
    previous: Option<crate::UiSnapshotState>,
    patch: Option<crate::UiPatch>,
    validation_seen: [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    validation_seen_cursor: usize,
}

impl UiPatchApplyOutcome {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn state(&self) -> Option<&crate::UiSnapshotState> {
        self.state.as_ref()
    }

    pub fn close_step(&mut self) -> bool {
        if !retire_snapshot_one(&mut self.previous) {
            return false;
        }
        if !retire_patch_one(&mut self.patch) {
            return false;
        }
        retire_validation_seen_one(&mut self.validation_seen, &mut self.validation_seen_cursor)
    }

    pub fn take_state(mut self) -> Result<crate::UiSnapshotState, Self> {
        if self.previous.is_some() || self.patch.is_some() || self.validation_seen_cursor < self.validation_seen.len() {
            return Err(self);
        }
        match (self.state.take(), self.retirement_handle.take()) {
            (Some(state), Some(handle)) => {
                with_ui_patch_apply_arena(|arena| arena.release(handle));
                Ok(state)
            }
            _ => Err(self),
        }
    }
}

impl Drop for UiPatchApplyOutcome {
    fn drop(&mut self) {
        let Some(handle) = self.retirement_handle.take() else { return };
        let retirement = UiPatchRetirement::new(
            self.state.take(),
            self.previous.take(),
            self.patch.take(),
            None,
            std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default())),
        );
        with_ui_patch_apply_arena(|arena| arena.handback(handle, retirement));
    }
}

#[derive(Debug)]
pub struct UiPatchApplyRejected {
    retirement_handle: Option<UiPatchApplyHandle>,
    generation: u64,
    state: Option<crate::UiSnapshotState>,
    patch: Option<crate::UiPatch>,
    draft: Option<crate::UiSnapshotState>,
    remove_record: Option<crate::UiNodeRecord>,
    rejection: PatchRejection,
    retire_scalar: u8,
    validation_seen: [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    validation_seen_cursor: usize,
}

impl UiPatchApplyRejected {
    fn new(retirement_handle: Option<UiPatchApplyHandle>, generation: u64, state: crate::UiSnapshotState, patch: crate::UiPatch, rejection: PatchRejection) -> Self {
        Self {
            retirement_handle,
            generation,
            state: Some(state),
            patch: Some(patch),
            draft: None,
            remove_record: None,
            rejection,
            retire_scalar: 0,
            validation_seen: std::array::from_fn(|_| crate::UiFixedList::default()),
            validation_seen_cursor: 0,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn rejection(&self) -> &PatchRejection {
        &self.rejection
    }

    pub fn state(&self) -> Option<&crate::UiSnapshotState> {
        self.state.as_ref()
    }

    pub fn close_step(&mut self) -> bool {
        if !retire_snapshot_one(&mut self.draft) {
            return false;
        }
        if self.remove_record.take().is_some() {
            return false;
        }
        if !retire_patch_one(&mut self.patch) {
            return false;
        }
        if self.retire_scalar == 0 {
            self.retire_scalar = 1;
            return false;
        }
        retire_validation_seen_one(&mut self.validation_seen, &mut self.validation_seen_cursor)
    }

    pub fn take_state(mut self) -> Result<crate::UiSnapshotState, Self> {
        if self.draft.is_some()
            || self.remove_record.is_some()
            || self.patch.is_some()
            || self.retire_scalar == 0
            || self.validation_seen_cursor < self.validation_seen.len()
        {
            return Err(self);
        }
        match (self.state.take(), self.retirement_handle.take()) {
            (Some(state), Some(handle)) => {
                with_ui_patch_apply_arena(|arena| arena.release(handle));
                Ok(state)
            }
            (Some(state), None) => Ok(state),
            _ => Err(self),
        }
    }
}

impl Drop for UiPatchApplyRejected {
    fn drop(&mut self) {
        let Some(handle) = self.retirement_handle.take() else { return };
        let retirement = UiPatchRetirement::new(
            self.state.take(),
            self.draft.take(),
            self.patch.take(),
            self.remove_record.take(),
            std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default())),
        );
        with_ui_patch_apply_arena(|arena| arena.handback(handle, retirement));
    }
}

fn retire_snapshot_one(state: &mut Option<crate::UiSnapshotState>) -> bool {
    let Some(owner) = state.as_mut() else { return true };
    if let Some(id) = owner.nodes.keys().next().copied() {
        drop(owner.nodes.remove(&id));
        return false;
    }
    state.take();
    false
}

fn retire_patch_one(patch: &mut Option<crate::UiPatch>) -> bool {
    let Some(owner) = patch.as_mut() else { return true };
    if owner.ops.pop().is_some() {
        return false;
    }
    patch.take();
    false
}

fn retire_validation_seen_one(
    seen: &mut [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    cursor: &mut usize,
) -> bool {
    while *cursor < seen.len() {
        if seen[*cursor].pop().is_some() {
            return false;
        }
        let Some(next) = cursor.checked_add(1) else { return false };
        *cursor = next;
    }
    true
}

pub fn close_ui_patch_owner_one() -> bool {
    with_ui_patch_apply_arena(UiPatchApplyArena::retire_one);
    with_ui_patch_apply_arena(|arena| !arena.has_retirement())
}

/// 🛡️ Which [`UiDocumentLimits`] field a [`PatchRejection::QuotaExceeded`] names — only the four
/// per-patch quotas `apply_patch` itself enforces directly; `max_nodes`/`max_depth` surface instead as
/// [`UiContractViolation::NodeQuota`]/[`UiContractViolation::DepthQuota`] inside
/// [`PatchRejection::InvariantViolated`], since those are whole-document shape properties only knowable
/// after the draft is built.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QuotaKind {
    Children,
    TextBytes,
    PatchOps,
    PatchBytes,
}

/// 🩹️ Applies `patch` to `state`, totally transactionally: `base_revision` must equal `state`'s
/// current revision; every op then applies to a **shadow draft clone**, never to `state` directly; the
/// draft is validated against `limits` via [`validate_snapshot`]'s shared core; only on success does
/// `state` swap to the draft in one move. On ANY rejection path, `state` is untouched — not partially
/// applied, not touched at all — which is the property that lets a sender resynchronise by trusting
/// `state`'s revision never moved.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
pub fn apply_patch(state: &mut crate::UiSnapshotState, patch: &crate::UiPatch, limits: &UiDocumentLimits) -> Result<(), PatchRejection> {
    if patch.base_revision != state.revision {
        return Err(PatchRejection::RevisionMismatch { expected: state.revision, actual: patch.base_revision });
    }
    if patch.ops.len() > limits.max_patch_ops {
        return Err(PatchRejection::QuotaExceeded { quota: QuotaKind::PatchOps, actual: patch.ops.len(), max: limits.max_patch_ops });
    }
    let estimated_bytes = patch_byte_estimate(patch);
    if estimated_bytes > limits.max_patch_bytes {
        return Err(PatchRejection::QuotaExceeded { quota: QuotaKind::PatchBytes, actual: estimated_bytes, max: limits.max_patch_bytes });
    }

    let mut draft = state.credited_clone().ok_or(PatchRejection::AliasCapacity)?;
    for op in &patch.ops {
        apply_op(&mut draft, op, limits)?;
    }
    draft.revision = patch.revision;

    validate_state(&draft, limits).map_err(|violations| PatchRejection::InvariantViolated { violations })?;
    *state = draft;
    Ok(())
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn apply_op(draft: &mut crate::UiSnapshotState, op: &crate::UiPatchOp, limits: &UiDocumentLimits) -> Result<(), PatchRejection> {
    match op {
        crate::UiPatchOp::Upsert(record) => {
            check_children_quota(record.children.len(), limits)?;
            check_text_quota(&record.component, limits)?;
            let record = record.credited_clone().ok_or(PatchRejection::AliasCapacity)?;
            let actual = crate::UI_DOCUMENT_NODES.checked_add(1).ok_or(PatchRejection::AliasCapacity)?;
            draft.nodes.try_insert(record).map_err(|_| PatchRejection::QuotaExceeded { quota: QuotaKind::Children, actual, max: crate::UI_DOCUMENT_NODES })?;
        }
        crate::UiPatchOp::SetComponent { id, component } => {
            check_text_quota(component, limits)?;
            mutate(draft, *id)?.component = component.credited_clone().ok_or(PatchRejection::AliasCapacity)?;
        }
        crate::UiPatchOp::SetLayout { id, layout } => {
            mutate(draft, *id)?.layout = layout.clone();
        }
        crate::UiPatchOp::SetActivity { id, activity, disabled } => {
            let record = mutate(draft, *id)?;
            record.activity = *activity;
            record.disabled = *disabled;
        }
        crate::UiPatchOp::SetChildren { id, children } => {
            check_children_quota(children.len(), limits)?;
            mutate(draft, *id)?.children = children.clone();
        }
        crate::UiPatchOp::SetStyle { id, style } => {
            mutate(draft, *id)?.style = *style;
        }
        crate::UiPatchOp::SetAccessibility { id, accessibility } => {
            mutate(draft, *id)?.accessibility = accessibility.clone();
        }
        crate::UiPatchOp::SetBindings { id, bindings } => {
            mutate(draft, *id)?.bindings = crate::credited_bindings(bindings).ok_or(PatchRejection::AliasCapacity)?;
        }
        crate::UiPatchOp::SetMenu { id, menu } => {
            mutate(draft, *id)?.menu = match menu.as_ref() {
                Some(menu) => Some(menu.credited_clone().ok_or(PatchRejection::AliasCapacity)?),
                None => None,
            };
        }
        crate::UiPatchOp::Remove { id } => remove_subtree(draft, *id)?,
        crate::UiPatchOp::SetRoot { id } => draft.root = Some(*id),
    }
    Ok(())
}

/// 🕳️ Looks up `id` in `draft` for mutation, or [`PatchRejection::UnknownNode`] if no such record
/// exists — the one place `SetComponent`/`SetLayout`/`SetActivity`/`SetChildren` can fail.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn mutate(draft: &mut crate::UiSnapshotState, id: crate::UiNodeId) -> Result<&mut crate::UiNodeRecord, PatchRejection> {
    draft.nodes.get_mut(&id).ok_or(PatchRejection::UnknownNode { id })
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn check_children_quota(count: usize, limits: &UiDocumentLimits) -> Result<(), PatchRejection> {
    if count > limits.max_children {
        Err(PatchRejection::QuotaExceeded { quota: QuotaKind::Children, actual: count, max: limits.max_children })
    } else {
        Ok(())
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn check_text_quota(component: &crate::Component, limits: &UiDocumentLimits) -> Result<(), PatchRejection> {
    let bytes = component_text_bytes(component);
    if bytes > limits.max_text_bytes {
        Err(PatchRejection::QuotaExceeded { quota: QuotaKind::TextBytes, actual: bytes, max: limits.max_text_bytes })
    } else {
        Ok(())
    }
}

/// 🧹️ Removes `id` and every node reachable from it via `children` — an iterative, explicit-stack
/// walk over the draft's OWN current children pointers, so `Remove` deletes the whole orphaned subtree
/// a caller meant to discard, not just the one named node left dangling as a still-referenced orphan.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn remove_subtree(draft: &mut crate::UiSnapshotState, id: crate::UiNodeId) -> Result<(), PatchRejection> {
    let mut stack = crate::UiFixedList::<crate::UiNodeId, UI_DOCUMENT_VIOLATIONS>::default();
    stack.try_push(id).map_err(|_| PatchRejection::AliasCapacity)?;
    while let Some(current) = stack.pop() {
        if let Some(record) = draft.nodes.remove(&current) {
            for child in record.children {
                stack.try_push(child).map_err(|_| PatchRejection::AliasCapacity)?;
            }
        }
    }
    Ok(())
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn text(value: &str) -> crate::UiText {
        crate::UiText::try_from_str(value).expect("bounded fixture")
    }

    fn surface() -> crate::SurfaceId {
        crate::SurfaceId::try_from("surface").expect("bounded fixture")
    }

    fn leaf(id: u64, key: &str) -> crate::UiNodeRecord {
        crate::UiNodeRecord {
            id: crate::UiNodeId(id),
            key: text(key),
            component: crate::Component::Separator(crate::SeparatorProps {}),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: crate::UiNodeBindings::default(),
            menu: None,
            children: crate::UiNodeChildren::default(),
        }
    }

    fn state_with_root() -> crate::UiSnapshotState {
        let mut state = crate::UiSnapshotState::new(surface());
        state.root = Some(crate::UiNodeId(0));
        state.nodes.try_insert(leaf(0, "root")).expect("fixed root");
        state
    }

    fn patch_with(base: crate::UiRevision, revision: crate::UiRevision, ops: crate::UiPatchOps) -> crate::UiPatch {
        crate::UiPatch { surface: surface(), base_revision: base, revision, ops }
    }

    fn drive_ready(mut producer: UiPatchApplyProducer, generation: u64) -> UiPatchApplyOutcome {
        loop {
            match producer.drive_one(generation, false, false) {
                UiPatchApplyStep::MoreWork => {}
                UiPatchApplyStep::Ready => return producer.take_ready().unwrap_or_else(|_| panic!("ready producer")),
                UiPatchApplyStep::Rejected => panic!("unexpected rejection: {:?}", producer.rejection()),
            }
        }
    }

    #[test]
    fn retained_patch_applies_one_census_node_op_and_validation_owner_per_opportunity() {
        let state = state_with_root();
        let mut ops = crate::UiPatchOps::default();
        ops.try_push(crate::UiPatchOp::SetActivity { id: crate::UiNodeId(0), activity: crate::Activity::Loading, disabled: true }).expect("one op");
        let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), ops);
        let mut producer = UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), 11).expect("retained producer");
        assert_eq!(producer.drive_one(11, false, true), UiPatchApplyStep::MoreWork, "expired deadline leaves the first census owner untouched");
        let mut outcome = drive_ready(producer, 11);
        assert_eq!(outcome.state().map(crate::UiSnapshotState::revision), Some(crate::UiRevision(1)));
        assert!(outcome.state().and_then(|state| state.get(crate::UiNodeId(0))).is_some_and(|record| record.disabled));
        while !outcome.close_step() {}
        let state = outcome.take_state().expect("closed outcome returns exact candidate");
        assert_eq!(state.revision, crate::UiRevision(1));
    }

    #[test]
    fn retained_patch_max_plus_one_returns_exact_state_and_patch() {
        let state = state_with_root();
        let mut ops = crate::UiPatchOps::default();
        ops.try_push(crate::UiPatchOp::SetRoot { id: crate::UiNodeId(0) }).expect("one op");
        let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), ops);
        let limits = UiDocumentLimits { max_patch_ops: 0, ..UiDocumentLimits::default() };
        let mut rejected = UiPatchApplyProducer::try_new(state, patch, limits, 12).expect_err("maximum plus one rejects before cloning");
        assert!(matches!(rejected.rejection(), PatchRejection::QuotaExceeded { quota: QuotaKind::PatchOps, actual: 1, max: 0 }));
        assert_eq!(rejected.state().map(crate::UiSnapshotState::revision), Some(crate::UiRevision(0)));
        while !rejected.close_step() {}
        assert_eq!(rejected.take_state().expect("exact original state").revision, crate::UiRevision(0));
    }

    #[test]
    fn retained_patch_cancel_stale_and_deadline_preserve_owner() {
        for (actual_generation, cancelled) in [(14, false), (13, true)] {
            let state = state_with_root();
            let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), crate::UiPatchOps::default());
            let mut producer = UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), 13).expect("retained producer");
            assert_eq!(producer.drive_one(actual_generation, cancelled, false), UiPatchApplyStep::Rejected);
            let mut rejected = producer.take_rejected().unwrap_or_else(|_| panic!("rejected owner"));
            while !rejected.close_step() {}
            assert_eq!(rejected.take_state().expect("exact original state").revision, crate::UiRevision(0));
        }
    }

    #[test]
    fn retained_patch_remove_advances_one_node_or_child_per_opportunity() {
        let mut state = state_with_root();
        let mut root_children = crate::UiNodeChildren::default();
        root_children.try_push(crate::UiNodeId(1)).expect("one child");
        state.nodes.get_mut(&crate::UiNodeId(0)).expect("root").children = root_children;
        let mut branch = leaf(1, "branch");
        branch.component = crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None });
        branch.children.try_push(crate::UiNodeId(2)).expect("one grandchild");
        state.nodes.try_insert(branch).expect("branch");
        state.nodes.try_insert(leaf(2, "leaf")).expect("leaf");
        let mut ops = crate::UiPatchOps::default();
        ops.try_push(crate::UiPatchOp::Remove { id: crate::UiNodeId(1) }).expect("remove");
        ops.try_push(crate::UiPatchOp::SetChildren { id: crate::UiNodeId(0), children: crate::UiNodeChildren::default() }).expect("detach");
        let mut outcome = drive_ready(UiPatchApplyProducer::try_new(state, patch_with(crate::UiRevision(0), crate::UiRevision(1), ops), UiDocumentLimits::default(), 15).expect("producer"), 15);
        assert_eq!(outcome.state().map(|state| state.nodes.len()), Some(1));
        while !outcome.close_step() {}
        let _ = outcome.take_state().expect("exact candidate");
    }

    #[test]
    fn retained_patch_duplicate_and_deep_validation_are_cursorized() {
        let mut state = state_with_root();
        let mut children = crate::UiNodeChildren::default();
        children.try_push(crate::UiNodeId(1)).expect("first");
        children.try_push(crate::UiNodeId(2)).expect("second");
        state.nodes.get_mut(&crate::UiNodeId(0)).expect("root").children = children;
        state.nodes.try_insert(leaf(1, "duplicate")).expect("first leaf");
        state.nodes.try_insert(leaf(2, "duplicate")).expect("second leaf");
        let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), crate::UiPatchOps::default());
        let mut producer = UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), 16).expect("producer");
        loop {
            match producer.drive_one(16, false, false) {
                UiPatchApplyStep::MoreWork => {}
                UiPatchApplyStep::Rejected => break,
                UiPatchApplyStep::Ready => panic!("duplicate sibling key must reject"),
            }
        }
        assert!(matches!(producer.rejection(), Some(PatchRejection::InvariantViolated { .. })));
        drop(producer);
        while !close_ui_patch_owner_one() {}
    }

    #[test]
    fn patch_handback_arena_max_plus_one_refuses_without_reusing_a_live_generation() {
        let mut arena = UiPatchApplyArena::default();
        let mut handles = [None; UI_PATCH_APPLY_SLOTS];
        for (index, handle) in handles.iter_mut().enumerate() {
            *handle = arena.reserve(index as u64 + 1);
            assert!(handle.is_some());
        }
        assert!(arena.reserve(99).is_none());
        for handle in handles.into_iter().flatten() {
            arena.release(handle);
        }
        let replacement = arena.reserve(100).expect("released slot");
        assert_eq!(replacement.generation, 100);
        assert!(handles.into_iter().flatten().all(|old| old.epoch != replacement.epoch || old.slot != replacement.slot));
    }

    #[test]
    fn abandoned_patch_owner_moves_to_incremental_handback_and_reopens_capacity() {
        while !close_ui_patch_owner_one() {}
        let state = state_with_root();
        let mut ops = crate::UiPatchOps::default();
        ops.try_push(crate::UiPatchOp::SetRoot { id: crate::UiNodeId(0) }).expect("one op");
        let patch = patch_with(crate::UiRevision(0), crate::UiRevision(1), ops);
        let mut producer = UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), 17).expect("reserved producer");
        assert_eq!(producer.drive_one(17, false, false), UiPatchApplyStep::MoreWork);
        drop(producer);
        assert!(!close_ui_patch_owner_one());
        while !close_ui_patch_owner_one() {}
        let replacement = UiPatchApplyProducer::try_new(
            state_with_root(),
            patch_with(crate::UiRevision(0), crate::UiRevision(1), crate::UiPatchOps::default()),
            UiDocumentLimits::default(),
            18,
        )
        .expect("handback released capacity");
        drop(replacement);
        while !close_ui_patch_owner_one() {}
    }
}
//#endregion 🧪️Tests
