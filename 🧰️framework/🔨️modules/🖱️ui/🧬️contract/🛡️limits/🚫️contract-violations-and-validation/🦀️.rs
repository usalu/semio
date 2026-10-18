//! 🚫️ Contract violations and validation: the `UiContractViolation` catalogue and the snapshot validation walk. See [`super`].

use super::*;

//#region 🔖️Validate
/// ⚠️ One structural invariant a [`crate::UiSnapshot`] fails — every variant here is a whole-document
/// shape property, never a per-patch wire quota (those are [`PatchRejection::QuotaExceeded`]).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "type", rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", tag = "type", rename_all = "camelCase")]
#[expect(clippy::large_enum_variant, reason = "The fixed-capacity diagnostic census owns the bounded duplicate-key witness inline without per-violation allocation.")]
pub enum UiContractViolation {
    CensusCapacity,
    /// 🔁️ `node` is reachable from itself by following `children` — a document must be a tree.
    Cycle {
        node: crate::UiNodeId,
    },
    /// 🧩️ `parent`'s `children` names `child`, but no record with that id exists.
    OrphanChild {
        parent: crate::UiNodeId,
        child: crate::UiNodeId,
    },
    /// 👯️ Two of `parent`'s children share `key` — reconciliation keys must be unique among siblings.
    DuplicateSiblingKey {
        parent: crate::UiNodeId,
        key: crate::UiText,
    },
    /// 📦️ `count` live nodes exceeds [`UiDocumentLimits::max_nodes`] (`max`).
    NodeQuota {
        count: usize,
        max: usize,
    },
    /// 📏️ `node` sits `depth` edges below the root, exceeding [`UiDocumentLimits::max_depth`] (`max`).
    DepthQuota {
        node: crate::UiNodeId,
        depth: usize,
        max: usize,
    },
    /// 🪢️ `node` exists in the node table but is not reachable from the root by following
    /// `children` — including the degenerate case where the root itself names an id with no record.
    DanglingRoot {
        node: crate::UiNodeId,
    },
    /// 🗂️ `node` is a `Container` with `role: Section` nested inside another `Section` — sectioning is
    /// intentionally flat, one level, so a renderer never has to resolve ambiguous nested chrome.
    SectionNested {
        node: crate::UiNodeId,
    },
    /// 🔢️ `node`'s component carries a NaN or infinite numeric field.
    NonFiniteNumber {
        node: crate::UiNodeId,
    },
    /// 🎛️ A Tree row's `inline_toolbar` is not its own direct horizontal Toolbar child, or
    /// the Toolbar does not own one to four direct leaf Buttons.
    InvalidTreeInlineToolbar {
        node: crate::UiNodeId,
        toolbar: crate::UiNodeId,
    },
    /// 🎛️ A Tree section's `header_toolbar` is not its direct horizontal Toolbar child, or the
    /// Toolbar does not own one to four direct leaf Buttons.
    InvalidTreeSectionHeaderToolbar {
        node: crate::UiNodeId,
        toolbar: crate::UiNodeId,
    },
    /// 🎞️ A Tree row's `detail` is not its own direct Surface child.
    InvalidTreeDetail {
        node: crate::UiNodeId,
        detail: crate::UiNodeId,
    },
    /// 🎯️ A TreeItem or TableRow row's verbs and `target` disagree, or the row carries a record
    /// `Trigger::Activate` binding instead of an `activation` verb on its target.
    InvalidRowTarget {
        node: crate::UiNodeId,
    },
    /// 📍️ A Slider's or number field's `snaps` are not strictly ascending inside its bounds — see
    /// [`crate::snaps_are_valid`].
    InvalidSnaps {
        node: crate::UiNodeId,
    },
    /// 🧭️ A Slider's, stepper's or number field's travel, scale, display factor and limits disagree — see
    /// [`crate::number_range_is_valid`].
    InvalidNumberRange {
        node: crate::UiNodeId,
    },
}

/// 🌲️ Validates `snapshot` against `limits`, collecting every [`UiContractViolation`] found rather
/// than stopping at the first — a fuzz corpus or a UI diagnostic wants the whole picture. Returns
/// `Ok(())` only when the document is entirely clean. Short-circuits to just [`UiContractViolation::NodeQuota`]
/// when the node count alone already exceeds the quota, since walking an already-oversized untrusted
/// document is itself part of the attack surface `max_nodes` exists to cut off.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub const UI_DOCUMENT_VIOLATIONS: usize = crate::UI_DOCUMENT_NODES * crate::UI_DOCUMENT_NODES + crate::UI_DOCUMENT_NODES * 6 + 1;

pub type UiContractViolations = crate::UiFixedList<UiContractViolation, UI_DOCUMENT_VIOLATIONS>;

#[cfg(test)]
pub fn validate_snapshot(snapshot: &crate::UiSnapshot, limits: &UiDocumentLimits) -> Result<(), UiContractViolations> {
    to_result(validate_core(Some(snapshot.root), snapshot.nodes.len(), |id| snapshot.nodes.iter().find(|record| record.id == id), snapshot.nodes.iter(), limits))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[cfg(test)]
pub(super) fn validate_state(state: &crate::UiSnapshotState, limits: &UiDocumentLimits) -> Result<(), UiContractViolations> {
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
                            if violations.try_push(UiContractViolation::SectionNested { node: id }).is_err() {
                                return violations;
                            }
                        }
                        if !component_is_finite(&record.component) {
                            if violations.try_push(UiContractViolation::NonFiniteNumber { node: id }).is_err() {
                                return violations;
                            }
                        } else if !component_snaps_are_valid(&record.component) {
                            if violations.try_push(UiContractViolation::InvalidSnaps { node: id }).is_err() {
                                return violations;
                            }
                        } else if !component_number_range_is_valid(&record.component) {
                            if violations.try_push(UiContractViolation::InvalidNumberRange { node: id }).is_err() {
                                return violations;
                            }
                        }
                        if let crate::Component::TreeItem(props) = &record.component {
                            if let Some(toolbar) = props.inline_toolbar.filter(|_| !tree_inline_toolbar_is_valid(record, &mut get)) {
                                if violations.try_push(UiContractViolation::InvalidTreeInlineToolbar { node: id, toolbar }).is_err() {
                                    return violations;
                                }
                            }
                            if let Some(detail) = props.detail.filter(|_| !tree_detail_is_valid(record, &mut get)) {
                                if violations.try_push(UiContractViolation::InvalidTreeDetail { node: id, detail }).is_err() {
                                    return violations;
                                }
                            }
                        }
                        if let crate::Component::TreeSection(props) = &record.component {
                            if let Some(toolbar) = props.header_toolbar.filter(|_| !tree_section_header_toolbar_is_valid(record, &mut get)) {
                                if violations.try_push(UiContractViolation::InvalidTreeSectionHeaderToolbar { node: id, toolbar }).is_err() {
                                    return violations;
                                }
                            }
                        }
                        if !row_target_is_valid(record) {
                            if violations.try_push(UiContractViolation::InvalidRowTarget { node: id }).is_err() {
                                return violations;
                            }
                        }
                        if depth > limits.max_depth {
                            if violations.try_push(UiContractViolation::DepthQuota { node: id, depth, max: limits.max_depth }).is_err() {
                                return violations;
                            }
                            continue;
                        }

                        if on_path.try_push(id).is_err() || stack.try_push(WalkFrame::Exit(id)).is_err() {
                            let _ = violations.try_push(UiContractViolation::CensusCapacity);
                            return violations;
                        }

                        let mut seen_keys = crate::UiFixedList::<&crate::UiText, { crate::UI_DOCUMENT_NODES }>::default();
                        for &child_id in &record.children {
                            match get(child_id) {
                                None => {
                                    if violations.try_push(UiContractViolation::OrphanChild { parent: id, child: child_id }).is_err() {
                                        return violations;
                                    }
                                }
                                Some(child) => {
                                    if seen_keys.iter().any(|key| key.as_str() == child.key.as_str()) {
                                        if violations.try_push(UiContractViolation::DuplicateSiblingKey { parent: id, key: child.key.clone() }).is_err() {
                                            return violations;
                                        }
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
            if violations.try_push(UiContractViolation::DanglingRoot { node: record.id }).is_err() {
                return violations;
            }
        }
    }
    violations
}
//#endregion 🔖️Validate
