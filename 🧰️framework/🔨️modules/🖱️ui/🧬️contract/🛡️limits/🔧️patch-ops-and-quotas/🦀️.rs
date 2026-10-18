//! 🔧️ Patch ops and quotas: the patch application, the single operation application, the mutation helper, the children and text quotas and the subtree removal. See [`super`].

use super::*;

//#region 🔖️Apply
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
pub(super) fn apply_op(draft: &mut crate::UiSnapshotState, op: &crate::UiPatchOp, limits: &UiDocumentLimits) -> Result<(), PatchRejection> {
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
