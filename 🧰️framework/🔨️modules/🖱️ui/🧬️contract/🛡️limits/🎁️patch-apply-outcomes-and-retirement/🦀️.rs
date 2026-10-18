//! 🎁️ Patch apply outcomes and retirement: the producer drop, the apply outcome, the rejected patch, the snapshot, patch and validation retirement and the quota kinds. See [`super`].

use super::*;

//#region 🔖️Apply
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
    pub(super) retirement_handle: Option<UiPatchApplyHandle>,
    pub(super) generation: u64,
    pub(super) state: Option<crate::UiSnapshotState>,
    pub(super) previous: Option<crate::UiSnapshotState>,
    pub(super) patch: Option<crate::UiPatch>,
    pub(super) validation_seen: [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    pub(super) validation_seen_cursor: usize,
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

    #[expect(clippy::result_large_err, reason = "State extraction retains the exact outcome owner until its previous payloads and validation backing are retired.")]
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
        let retirement = UiPatchRetirement::new(self.state.take(), self.previous.take(), self.patch.take(), None, std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default())));
        with_ui_patch_apply_arena(|arena| arena.handback(handle, retirement));
    }
}

#[derive(Debug)]
pub struct UiPatchApplyRejected {
    pub(super) retirement_handle: Option<UiPatchApplyHandle>,
    pub(super) generation: u64,
    pub(super) state: Option<crate::UiSnapshotState>,
    pub(super) patch: Option<crate::UiPatch>,
    pub(super) draft: Option<crate::UiSnapshotState>,
    pub(super) remove_record: Option<crate::UiNodeRecord>,
    pub(super) rejection: PatchRejection,
    pub(super) retire_scalar: u8,
    pub(super) validation_seen: [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    pub(super) validation_seen_cursor: usize,
}

impl UiPatchApplyRejected {
    pub(super) fn new(retirement_handle: Option<UiPatchApplyHandle>, generation: u64, state: crate::UiSnapshotState, patch: crate::UiPatch, rejection: PatchRejection) -> Self {
        Self { retirement_handle, generation, state: Some(state), patch: Some(patch), draft: None, remove_record: None, rejection, retire_scalar: 0, validation_seen: std::array::from_fn(|_| crate::UiFixedList::default()), validation_seen_cursor: 0 }
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

    #[expect(clippy::result_large_err, reason = "State extraction retains the exact outcome owner until its previous payloads and validation backing are retired.")]
    pub fn take_state(mut self) -> Result<crate::UiSnapshotState, Self> {
        if self.draft.is_some() || self.remove_record.is_some() || self.patch.is_some() || self.retire_scalar == 0 || self.validation_seen_cursor < self.validation_seen.len() {
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
        let retirement = UiPatchRetirement::new(self.state.take(), self.draft.take(), self.patch.take(), self.remove_record.take(), std::mem::replace(&mut self.validation_seen, std::array::from_fn(|_| crate::UiFixedList::default())));
        with_ui_patch_apply_arena(|arena| arena.handback(handle, retirement));
    }
}

pub(super) fn retire_snapshot_one(state: &mut Option<crate::UiSnapshotState>) -> bool {
    let Some(owner) = state.as_mut() else { return true };
    let id = owner.nodes.keys().next().copied();
    if let Some(id) = id {
        drop(owner.nodes.remove(&id));
        return false;
    }
    state.take();
    false
}

pub(super) fn retire_patch_one(patch: &mut Option<crate::UiPatch>) -> bool {
    let Some(owner) = patch.as_mut() else { return true };
    if owner.ops.pop().is_some() {
        return false;
    }
    patch.take();
    false
}

pub(super) fn retire_validation_seen_one(seen: &mut [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES], cursor: &mut usize) -> bool {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum QuotaKind {
    Children,
    TextBytes,
    PatchOps,
    PatchBytes,
}
//#endregion 🔖️Apply
