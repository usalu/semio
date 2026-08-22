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
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

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
fn bindings_text_bytes(bindings: &[crate::ActionBinding]) -> usize {
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
    /// 🔁️ `node` is reachable from itself by following `children` — a document must be a tree.
    Cycle { node: crate::UiNodeId },
    /// 🧩️ `parent`'s `children` names `child`, but no record with that id exists.
    OrphanChild { parent: crate::UiNodeId, child: crate::UiNodeId },
    /// 👯️ Two of `parent`'s children share `key` — reconciliation keys must be unique among siblings.
    DuplicateSiblingKey { parent: crate::UiNodeId, key: String },
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
pub fn validate_snapshot(snapshot: &crate::UiSnapshot, limits: &UiDocumentLimits) -> Result<(), Vec<UiContractViolation>> {
    let by_id: HashMap<crate::UiNodeId, &crate::UiNodeRecord> = snapshot.nodes.iter().map(|record| (record.id, record)).collect();
    to_result(validate_core(Some(snapshot.root), &by_id, limits))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn validate_state(state: &crate::UiSnapshotState, limits: &UiDocumentLimits) -> Result<(), Vec<UiContractViolation>> {
    to_result(validate_core(state.root, &state.nodes, limits))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn to_result(violations: Vec<UiContractViolation>) -> Result<(), Vec<UiContractViolation>> {
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
enum WalkFrame {
    Enter(crate::UiNodeId, usize, bool),
    Exit(crate::UiNodeId),
}

/// 🌲️ The shared traversal behind [`validate_snapshot`] and [`apply_patch`]'s post-op check — generic
/// over `V: Borrow<UiNodeRecord>` so it runs unmodified against a [`crate::UiSnapshot`]'s borrowed
/// `HashMap<UiNodeId, &UiNodeRecord>` and a [`crate::UiSnapshotState`]'s owned
/// `HashMap<UiNodeId, UiNodeRecord>` alike — one algorithm, never two copies to keep in sync.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn validate_core<V: Borrow<crate::UiNodeRecord>>(root: Option<crate::UiNodeId>, nodes: &HashMap<crate::UiNodeId, V>, limits: &UiDocumentLimits) -> Vec<UiContractViolation> {
    let mut violations = Vec::new();
    if nodes.len() > limits.max_nodes {
        violations.push(UiContractViolation::NodeQuota { count: nodes.len(), max: limits.max_nodes });
        return violations;
    }

    let mut visited: HashSet<crate::UiNodeId> = HashSet::new();
    let mut on_path: HashSet<crate::UiNodeId> = HashSet::new();

    if let Some(root_id) = root {
        if nodes.contains_key(&root_id) {
            let mut stack = vec![WalkFrame::Enter(root_id, 0, false)];
            while let Some(frame) = stack.pop() {
                match frame {
                    WalkFrame::Exit(id) => {
                        on_path.remove(&id);
                    }
                    WalkFrame::Enter(id, depth, parent_in_section) => {
                        if on_path.contains(&id) {
                            violations.push(UiContractViolation::Cycle { node: id });
                            continue;
                        }
                        if !visited.insert(id) {
                            continue;
                        }
                        let Some(record) = nodes.get(&id).map(V::borrow) else { continue };

                        let in_section = parent_in_section || is_section(&record.component);
                        if parent_in_section && is_section(&record.component) {
                            violations.push(UiContractViolation::SectionNested { node: id });
                        }
                        if !component_is_finite(&record.component) {
                            violations.push(UiContractViolation::NonFiniteNumber { node: id });
                        }
                        if depth > limits.max_depth {
                            violations.push(UiContractViolation::DepthQuota { node: id, depth, max: limits.max_depth });
                            continue;
                        }

                        on_path.insert(id);
                        stack.push(WalkFrame::Exit(id));

                        let mut seen_keys: HashSet<&str> = HashSet::new();
                        for &child_id in &record.children {
                            match nodes.get(&child_id).map(V::borrow) {
                                None => violations.push(UiContractViolation::OrphanChild { parent: id, child: child_id }),
                                Some(child) => {
                                    if !seen_keys.insert(child.key.as_str()) {
                                        violations.push(UiContractViolation::DuplicateSiblingKey { parent: id, key: child.key.clone() });
                                    }
                                    stack.push(WalkFrame::Enter(child_id, depth + 1, in_section));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for id in nodes.keys() {
        if !visited.contains(id) {
            violations.push(UiContractViolation::DanglingRoot { node: *id });
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
        violations: Vec<UiContractViolation>,
    },
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

    let mut draft = state.clone();
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
            draft.nodes.insert(record.id, record.clone());
        }
        crate::UiPatchOp::SetComponent { id, component } => {
            check_text_quota(component, limits)?;
            mutate(draft, *id)?.component = component.clone();
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
            mutate(draft, *id)?.bindings = bindings.clone();
        }
        crate::UiPatchOp::SetMenu { id, menu } => {
            mutate(draft, *id)?.menu = menu.clone();
        }
        crate::UiPatchOp::Remove { id } => remove_subtree(draft, *id),
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
fn remove_subtree(draft: &mut crate::UiSnapshotState, id: crate::UiNodeId) {
    let mut stack = vec![id];
    while let Some(current) = stack.pop() {
        if let Some(record) = draft.nodes.remove(&current) {
            stack.extend(record.children);
        }
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};

    fn leaf(id: u64, key: &str) -> crate::UiNodeRecord {
        crate::UiNodeRecord {
            id: crate::UiNodeId(id),
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

    fn container(id: u64, key: &str, role: crate::ContainerRole, children: &[u64]) -> crate::UiNodeRecord {
        let mut record = leaf(id, key);
        record.component = crate::Component::Container(crate::ContainerProps { role, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None });
        record.children = children.iter().copied().map(crate::UiNodeId).collect();
        record
    }

    fn state_with(root: u64, records: Vec<crate::UiNodeRecord>) -> crate::UiSnapshotState {
        let mut state = crate::UiSnapshotState::new(crate::SurfaceId::from("s"));
        state.root = Some(crate::UiNodeId(root));
        for record in records {
            state.nodes.insert(record.id, record);
        }
        state
    }

    /// #️⃣️ A deterministic test-only fingerprint — `UiSnapshotState` cannot derive `Hash` (its
    /// `Component`s carry `f64` fields), so this hashes each record's stable `Debug` text instead.
    fn fingerprint(state: &crate::UiSnapshotState) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        state.surface.hash(&mut hasher);
        state.revision.hash(&mut hasher);
        state.root.hash(&mut hasher);
        let mut ids: Vec<_> = state.nodes.keys().copied().collect();
        ids.sort();
        for id in ids {
            id.hash(&mut hasher);
            format!("{:?}", state.nodes[&id]).hash(&mut hasher);
        }
        hasher.finish()
    }

    fn assert_unchanged<F: FnOnce(&mut crate::UiSnapshotState) -> Result<(), PatchRejection>>(state: &crate::UiSnapshotState, run: F) -> PatchRejection {
        let before = fingerprint(state);
        let mut mutated = state.clone();
        let rejection = run(&mut mutated).expect_err("expected rejection");
        assert_eq!(fingerprint(&mutated), before, "state must be byte-for-byte unchanged after a rejected patch");
        rejection
    }

    //#region 🔖️ApplyPatchHappyPath
    #[test]
    fn apply_patch_advances_revision_and_applies_every_op_kind() {
        let mut state = state_with(0, vec![container(0, "root", crate::ContainerRole::Plain, &[1]), leaf(1, "a")]);
        let limits = UiDocumentLimits::default();

        let patch = crate::UiPatch {
            surface: state.surface.clone(),
            base_revision: state.revision,
            revision: state.revision.next(),
            ops: vec![
                crate::UiPatchOp::Upsert(leaf(2, "b")),
                crate::UiPatchOp::SetChildren { id: crate::UiNodeId(0), children: vec![crate::UiNodeId(1), crate::UiNodeId(2)] },
                crate::UiPatchOp::SetComponent { id: crate::UiNodeId(1), component: crate::Component::Text(crate::TextProps { value: crate::Label::from("hi"), emphasize: None, data_attributes: None }) },
                crate::UiPatchOp::SetLayout { id: crate::UiNodeId(1), layout: crate::LayoutSpec::Leaf(crate::LeafLayout { width: crate::Sizing::Fill, height: crate::Sizing::Hug }) },
                crate::UiPatchOp::SetActivity { id: crate::UiNodeId(1), activity: crate::Activity::Loading, disabled: true },
                crate::UiPatchOp::SetStyle { id: crate::UiNodeId(1), style: crate::StyleSpec { tone: crate::Tone::Danger, ..Default::default() } },
                crate::UiPatchOp::SetAccessibility { id: crate::UiNodeId(1), accessibility: crate::AccessibilitySpec { shortcut: Some("Ctrl+S".into()), ..Default::default() } },
                crate::UiPatchOp::SetBindings { id: crate::UiNodeId(1), bindings: vec![crate::ActionBinding { trigger: crate::Trigger::Activate, action: crate::ActionId::v1("scope", "name"), args: None, capability: None }] },
                crate::UiPatchOp::SetMenu { id: crate::UiNodeId(1), menu: Some(crate::MenuRef { id: "menu".into(), args: None }) },
                crate::UiPatchOp::SetRoot { id: crate::UiNodeId(0) },
            ],
        };

        apply_patch(&mut state, &patch, &limits).expect("patch should apply");
        assert_eq!(state.revision, crate::UiRevision(1));
        assert_eq!(state.children_of(crate::UiNodeId(0)), &[crate::UiNodeId(1), crate::UiNodeId(2)]);
        assert!(state.nodes.contains_key(&crate::UiNodeId(2)));
        assert_eq!(state.nodes[&crate::UiNodeId(1)].activity, crate::Activity::Loading);
        assert!(state.nodes[&crate::UiNodeId(1)].disabled);
        assert_eq!(state.nodes[&crate::UiNodeId(1)].style.tone, crate::Tone::Danger);
        assert_eq!(state.nodes[&crate::UiNodeId(1)].accessibility.shortcut.as_deref(), Some("Ctrl+S"));
        assert_eq!(state.nodes[&crate::UiNodeId(1)].bindings.len(), 1);
        assert_eq!(state.nodes[&crate::UiNodeId(1)].menu.as_ref().map(|menu| menu.id.as_str()), Some("menu"));
    }

    #[test]
    fn apply_patch_remove_deletes_whole_orphaned_subtree() {
        let mut state = state_with(0, vec![container(0, "root", crate::ContainerRole::Plain, &[1]), container(1, "mid", crate::ContainerRole::Group, &[2, 3]), leaf(2, "a"), leaf(3, "b")]);
        let limits = UiDocumentLimits::default();
        let patch = crate::UiPatch {
            surface: state.surface.clone(),
            base_revision: state.revision,
            revision: state.revision.next(),
            ops: vec![crate::UiPatchOp::Remove { id: crate::UiNodeId(1) }, crate::UiPatchOp::SetChildren { id: crate::UiNodeId(0), children: vec![] }],
        };

        apply_patch(&mut state, &patch, &limits).expect("patch should apply");
        assert!(!state.nodes.contains_key(&crate::UiNodeId(1)));
        assert!(!state.nodes.contains_key(&crate::UiNodeId(2)));
        assert!(!state.nodes.contains_key(&crate::UiNodeId(3)));
        assert_eq!(state.nodes.len(), 1);
    }
    //#endregion 🔖️ApplyPatchHappyPath

    //#region 🔖️RejectionIsTotal
    #[test]
    fn stale_base_revision_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![leaf(0, "root")]);
        let limits = UiDocumentLimits::default();
        let rejection = assert_unchanged(&state, |draft| {
            let patch = crate::UiPatch { surface: draft.surface.clone(), base_revision: crate::UiRevision(99), revision: crate::UiRevision(100), ops: vec![] };
            apply_patch(draft, &patch, &limits)
        });
        assert!(matches!(rejection, PatchRejection::RevisionMismatch { expected: crate::UiRevision(0), actual: crate::UiRevision(99) }));
    }

    #[test]
    fn op_referencing_unknown_node_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![leaf(0, "root")]);
        let limits = UiDocumentLimits::default();
        let rejection = assert_unchanged(&state, |draft| {
            let patch = crate::UiPatch {
                surface: draft.surface.clone(),
                base_revision: draft.revision,
                revision: draft.revision.next(),
                ops: vec![crate::UiPatchOp::SetActivity { id: crate::UiNodeId(404), activity: crate::Activity::Idle, disabled: false }],
            };
            apply_patch(draft, &patch, &limits)
        });
        assert!(matches!(rejection, PatchRejection::UnknownNode { id: crate::UiNodeId(404) }));
    }

    /// 🕳️ Each of the four new field-targeted setters fails the same way as the original four — one
    /// rejection per op, state byte-for-byte unchanged — never a partial write to a nonexistent node.
    #[test]
    fn new_field_targeted_ops_referencing_unknown_node_are_rejected_and_state_unchanged() {
        let state = state_with(0, vec![leaf(0, "root")]);
        let limits = UiDocumentLimits::default();
        let candidate_ops = vec![
            crate::UiPatchOp::SetStyle { id: crate::UiNodeId(404), style: crate::StyleSpec { tone: crate::Tone::Danger, ..Default::default() } },
            crate::UiPatchOp::SetAccessibility { id: crate::UiNodeId(404), accessibility: crate::AccessibilitySpec { shortcut: Some("x".into()), ..Default::default() } },
            crate::UiPatchOp::SetBindings { id: crate::UiNodeId(404), bindings: vec![crate::ActionBinding { trigger: crate::Trigger::Activate, action: crate::ActionId::v1("scope", "name"), args: None, capability: None }] },
            crate::UiPatchOp::SetMenu { id: crate::UiNodeId(404), menu: Some(crate::MenuRef { id: "menu".into(), args: None }) },
        ];
        for op in candidate_ops {
            let rejection = assert_unchanged(&state, |draft| {
                let patch = crate::UiPatch { surface: draft.surface.clone(), base_revision: draft.revision, revision: draft.revision.next(), ops: vec![op.clone()] };
                apply_patch(draft, &patch, &limits)
            });
            assert!(matches!(rejection, PatchRejection::UnknownNode { id: crate::UiNodeId(404) }), "op {op:?} should reject as UnknownNode");
        }
    }

    #[test]
    fn cycle_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![container(0, "root", crate::ContainerRole::Plain, &[1]), container(1, "a", crate::ContainerRole::Plain, &[])]);
        let limits = UiDocumentLimits::default();
        let rejection = assert_unchanged(&state, |draft| {
            let patch = crate::UiPatch { surface: draft.surface.clone(), base_revision: draft.revision, revision: draft.revision.next(), ops: vec![crate::UiPatchOp::SetChildren { id: crate::UiNodeId(1), children: vec![crate::UiNodeId(0)] }] };
            apply_patch(draft, &patch, &limits)
        });
        match rejection {
            PatchRejection::InvariantViolated { violations } => assert!(violations.iter().any(|v| matches!(v, UiContractViolation::Cycle { node: crate::UiNodeId(0) } | UiContractViolation::Cycle { node: crate::UiNodeId(1) }))),
            other => panic!("expected InvariantViolated, got {other:?}"),
        }
    }

    #[test]
    fn duplicate_sibling_key_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![container(0, "root", crate::ContainerRole::Plain, &[1, 2]), leaf(1, "same"), leaf(2, "other")]);
        let limits = UiDocumentLimits::default();
        let rejection = assert_unchanged(&state, |draft| {
            let patch = crate::UiPatch { surface: draft.surface.clone(), base_revision: draft.revision, revision: draft.revision.next(), ops: vec![crate::UiPatchOp::Upsert(leaf(2, "same"))] };
            apply_patch(draft, &patch, &limits)
        });
        match rejection {
            PatchRejection::InvariantViolated { violations } => assert!(violations.iter().any(|v| matches!(v, UiContractViolation::DuplicateSiblingKey { .. }))),
            other => panic!("expected InvariantViolated, got {other:?}"),
        }
    }

    #[test]
    fn max_patch_ops_quota_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![leaf(0, "root")]);
        let limits = UiDocumentLimits { max_patch_ops: 1, ..UiDocumentLimits::default() };
        let rejection = assert_unchanged(&state, |draft| {
            let patch = crate::UiPatch { surface: draft.surface.clone(), base_revision: draft.revision, revision: draft.revision.next(), ops: vec![crate::UiPatchOp::Upsert(leaf(1, "a")), crate::UiPatchOp::Upsert(leaf(2, "b"))] };
            apply_patch(draft, &patch, &limits)
        });
        assert!(matches!(rejection, PatchRejection::QuotaExceeded { quota: QuotaKind::PatchOps, actual: 2, max: 1 }));
    }

    #[test]
    fn max_patch_bytes_quota_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![leaf(0, "root")]);
        let limits = UiDocumentLimits { max_patch_bytes: 4, ..UiDocumentLimits::default() };
        let rejection = assert_unchanged(&state, |draft| {
            let mut big = leaf(1, "a");
            big.component = crate::Component::Text(crate::TextProps { value: crate::Label::from("a fairly long piece of text"), emphasize: None, data_attributes: None });
            let patch = crate::UiPatch { surface: draft.surface.clone(), base_revision: draft.revision, revision: draft.revision.next(), ops: vec![crate::UiPatchOp::Upsert(big)] };
            apply_patch(draft, &patch, &limits)
        });
        assert!(matches!(rejection, PatchRejection::QuotaExceeded { quota: QuotaKind::PatchBytes, .. }));
    }

    #[test]
    fn max_children_quota_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![leaf(0, "root")]);
        let limits = UiDocumentLimits { max_children: 1, ..UiDocumentLimits::default() };
        let rejection = assert_unchanged(&state, |draft| {
            let patch = crate::UiPatch {
                surface: draft.surface.clone(),
                base_revision: draft.revision,
                revision: draft.revision.next(),
                ops: vec![crate::UiPatchOp::SetChildren { id: crate::UiNodeId(0), children: vec![crate::UiNodeId(1), crate::UiNodeId(2)] }],
            };
            apply_patch(draft, &patch, &limits)
        });
        assert!(matches!(rejection, PatchRejection::QuotaExceeded { quota: QuotaKind::Children, actual: 2, max: 1 }));
    }

    #[test]
    fn max_text_bytes_quota_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![leaf(0, "root")]);
        let limits = UiDocumentLimits { max_text_bytes: 4, ..UiDocumentLimits::default() };
        let rejection = assert_unchanged(&state, |draft| {
            let big = crate::Component::Text(crate::TextProps { value: crate::Label::from("way too long"), emphasize: None, data_attributes: None });
            let patch = crate::UiPatch { surface: draft.surface.clone(), base_revision: draft.revision, revision: draft.revision.next(), ops: vec![crate::UiPatchOp::SetComponent { id: crate::UiNodeId(0), component: big }] };
            apply_patch(draft, &patch, &limits)
        });
        assert!(matches!(rejection, PatchRejection::QuotaExceeded { quota: QuotaKind::TextBytes, .. }));
    }

    #[test]
    fn max_nodes_quota_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![leaf(0, "root")]);
        let limits = UiDocumentLimits { max_nodes: 1, ..UiDocumentLimits::default() };
        let rejection = assert_unchanged(&state, |draft| {
            let patch = crate::UiPatch { surface: draft.surface.clone(), base_revision: draft.revision, revision: draft.revision.next(), ops: vec![crate::UiPatchOp::Upsert(leaf(1, "a"))] };
            apply_patch(draft, &patch, &limits)
        });
        match rejection {
            PatchRejection::InvariantViolated { violations } => assert!(violations.iter().any(|v| matches!(v, UiContractViolation::NodeQuota { count: 2, max: 1 }))),
            other => panic!("expected InvariantViolated, got {other:?}"),
        }
    }

    #[test]
    fn max_depth_quota_is_rejected_and_state_unchanged() {
        let state = state_with(0, vec![container(0, "root", crate::ContainerRole::Plain, &[1]), container(1, "mid", crate::ContainerRole::Plain, &[])]);
        let limits = UiDocumentLimits { max_depth: 0, ..UiDocumentLimits::default() };
        let rejection = assert_unchanged(&state, |draft| {
            let patch =
                crate::UiPatch { surface: draft.surface.clone(), base_revision: draft.revision, revision: draft.revision.next(), ops: vec![crate::UiPatchOp::SetActivity { id: crate::UiNodeId(1), activity: crate::Activity::Idle, disabled: false }] };
            apply_patch(draft, &patch, &limits)
        });
        match rejection {
            PatchRejection::InvariantViolated { violations } => assert!(violations.iter().any(|v| matches!(v, UiContractViolation::DepthQuota { node: crate::UiNodeId(1), .. }))),
            other => panic!("expected InvariantViolated, got {other:?}"),
        }
    }
    //#endregion 🔖️RejectionIsTotal

    //#region 🔖️ValidateSnapshot
    #[test]
    fn validate_snapshot_catches_dangling_child_reference() {
        let snapshot = crate::UiSnapshot { surface: crate::SurfaceId::from("s"), revision: crate::UiRevision(0), root: crate::UiNodeId(0), nodes: vec![container(0, "root", crate::ContainerRole::Plain, &[404])], layout_epoch: 0 };
        let violations = validate_snapshot(&snapshot, &UiDocumentLimits::default()).expect_err("expected violations");
        assert!(violations.iter().any(|v| matches!(v, UiContractViolation::OrphanChild { parent: crate::UiNodeId(0), child: crate::UiNodeId(404) })));
    }

    #[test]
    fn validate_snapshot_catches_node_unreachable_from_root() {
        let snapshot = crate::UiSnapshot { surface: crate::SurfaceId::from("s"), revision: crate::UiRevision(0), root: crate::UiNodeId(0), nodes: vec![leaf(0, "root"), leaf(99, "stray")], layout_epoch: 0 };
        let violations = validate_snapshot(&snapshot, &UiDocumentLimits::default()).expect_err("expected violations");
        assert!(violations.iter().any(|v| matches!(v, UiContractViolation::DanglingRoot { node: crate::UiNodeId(99) })));
    }

    #[test]
    fn validate_snapshot_catches_section_nested_in_section() {
        let snapshot = crate::UiSnapshot {
            surface: crate::SurfaceId::from("s"),
            revision: crate::UiRevision(0),
            root: crate::UiNodeId(0),
            nodes: vec![container(0, "root", crate::ContainerRole::Section, &[1]), container(1, "inner", crate::ContainerRole::Section, &[])],
            layout_epoch: 0,
        };
        let violations = validate_snapshot(&snapshot, &UiDocumentLimits::default()).expect_err("expected violations");
        assert!(violations.iter().any(|v| matches!(v, UiContractViolation::SectionNested { node: crate::UiNodeId(1) })));
    }

    #[test]
    fn validate_snapshot_catches_non_finite_number() {
        let mut root = leaf(0, "root");
        root.component = crate::Component::Slider(crate::SliderProps { value: f64::NAN, min: 0.0, max: 1.0, step: 0.1, unit: None });
        let snapshot = crate::UiSnapshot { surface: crate::SurfaceId::from("s"), revision: crate::UiRevision(0), root: crate::UiNodeId(0), nodes: vec![root], layout_epoch: 0 };
        let violations = validate_snapshot(&snapshot, &UiDocumentLimits::default()).expect_err("expected violations");
        assert!(violations.iter().any(|v| matches!(v, UiContractViolation::NonFiniteNumber { node: crate::UiNodeId(0) })));
    }

    #[test]
    fn validate_snapshot_clean_document_is_ok() {
        let snapshot = crate::UiSnapshot { surface: crate::SurfaceId::from("s"), revision: crate::UiRevision(0), root: crate::UiNodeId(0), nodes: vec![container(0, "root", crate::ContainerRole::Plain, &[1]), leaf(1, "a")], layout_epoch: 0 };
        assert_eq!(validate_snapshot(&snapshot, &UiDocumentLimits::default()), Ok(()));
    }
    //#endregion 🔖️ValidateSnapshot

    //#region 🔖️Roundtrips
    #[test]
    fn ui_document_limits_default_round_trips() {
        let limits = UiDocumentLimits::default();
        let json = serde_json::to_string(&limits).expect("serialize");
        let back: UiDocumentLimits = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(limits, back);
    }

    #[test]
    fn every_violation_variant_round_trips() {
        let violations = vec![
            UiContractViolation::Cycle { node: crate::UiNodeId(1) },
            UiContractViolation::OrphanChild { parent: crate::UiNodeId(1), child: crate::UiNodeId(2) },
            UiContractViolation::DuplicateSiblingKey { parent: crate::UiNodeId(1), key: "k".into() },
            UiContractViolation::NodeQuota { count: 5, max: 4 },
            UiContractViolation::DepthQuota { node: crate::UiNodeId(1), depth: 5, max: 4 },
            UiContractViolation::DanglingRoot { node: crate::UiNodeId(1) },
            UiContractViolation::SectionNested { node: crate::UiNodeId(1) },
            UiContractViolation::NonFiniteNumber { node: crate::UiNodeId(1) },
        ];
        for violation in violations {
            let json = serde_json::to_string(&violation).expect("serialize");
            let back: UiContractViolation = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(violation, back);
        }
    }

    #[test]
    fn every_patch_rejection_variant_round_trips() {
        let rejections = vec![
            PatchRejection::RevisionMismatch { expected: crate::UiRevision(1), actual: crate::UiRevision(2) },
            PatchRejection::UnknownNode { id: crate::UiNodeId(9) },
            PatchRejection::QuotaExceeded { quota: QuotaKind::PatchBytes, actual: 10, max: 4 },
            PatchRejection::InvariantViolated { violations: vec![UiContractViolation::Cycle { node: crate::UiNodeId(1) }] },
        ];
        for rejection in rejections {
            let json = serde_json::to_string(&rejection).expect("serialize");
            let back: PatchRejection = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(rejection, back);
        }
    }
    //#endregion 🔖️Roundtrips
}
//#endregion 🧪️Tests
