//! 🩹️ Patch rejection and apply arena: the `PatchRejection`, the apply phases, handles, retirement and slots and the process-wide patch apply arena. See [`super`].

use super::*;

//#region 🔖️Apply
/// 🚫️ Why [`apply_patch`] rejected a [`crate::UiPatch`] — carries enough detail (both revisions, the
/// exceeded quota with its actual/max, or the full violation list) for the existing `patch-rejected`
/// wire event to explain itself and for the sender to resynchronise.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "type", rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", tag = "type", rename_all = "camelCase")]
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
pub(super) enum UiPatchApplyPhase {
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
pub(super) struct UiPatchApplyHandle {
    pub(super) slot: usize,
    pub(super) epoch: u64,
    pub(super) generation: u64,
}

pub(super) struct UiPatchRetirement {
    first_state: Option<crate::UiSnapshotState>,
    second_state: Option<crate::UiSnapshotState>,
    pub(super) patch: Option<crate::UiPatch>,
    pub(super) record: Option<crate::UiNodeRecord>,
    pub(super) seen: Option<[crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES]>,
    seen_cursor: usize,
}

impl UiPatchRetirement {
    pub(super) fn new(
        first_state: Option<crate::UiSnapshotState>,
        second_state: Option<crate::UiSnapshotState>,
        patch: Option<crate::UiPatch>,
        record: Option<crate::UiNodeRecord>,
        seen: [crate::UiFixedList<crate::UiText, { crate::UI_DOCUMENT_NODES }>; crate::UI_DOCUMENT_NODES],
    ) -> Self {
        Self { first_state, second_state, patch, record, seen: Some(seen), seen_cursor: 0 }
    }

    pub(super) fn retire_one(&mut self) -> bool {
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

#[derive(Default)]
struct UiPatchApplySlot {
    pub(super) epoch: u64,
    pub(super) generation: u64,
    occupied: bool,
    pub(super) retirement: Option<UiPatchRetirement>,
}

impl UiPatchApplySlot {
    const EMPTY: Self = Self { epoch: 0, generation: 0, occupied: false, retirement: None };
}

pub(super) struct UiPatchApplyArena {
    slots: [UiPatchApplySlot; UI_PATCH_APPLY_SLOTS],
    close_cursor: usize,
}

/// 🧊️ `const`, not `Default` — and the static below is a `const`-initialised `Mutex`, not a
/// `LazyLock`. One slot is ~11.7 KiB (its `UiPatchRetirement` carries the validation `seen` matrix),
/// so the arena is ~93.5 KiB; at `opt-level = 0` the old `std::array::from_fn` initialiser
/// materialised that array once per frame of the `from_fn → try_from_fn → try_from_fn_erased` chain
/// plus the `LazyLock` closure's return temporary — ~654 KiB of shadow stack to zero a static, which
/// is most of a guest's tape and overflowed the whole thing during `Event::InstanceOpen`
/// (ticket 26/09/18 slice A2). A `const` initialiser is laid down in the image instead: zero frames,
/// zero copies, zero first-touch cost.
impl UiPatchApplyArena {
    const EMPTY: Self = Self { slots: [const { UiPatchApplySlot::EMPTY }; UI_PATCH_APPLY_SLOTS], close_cursor: 0 };
}

impl Default for UiPatchApplyArena {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl UiPatchApplyArena {
    pub(super) fn reserve(&mut self, generation: u64) -> Option<UiPatchApplyHandle> {
        let slot = self.slots.iter().position(|slot| !slot.occupied)?;
        let epoch = self.slots[slot].epoch.checked_add(1)?;
        self.slots[slot] = UiPatchApplySlot { epoch, generation, occupied: true, retirement: None };
        Some(UiPatchApplyHandle { slot, epoch, generation })
    }

    fn slot_mut(&mut self, handle: UiPatchApplyHandle) -> Option<&mut UiPatchApplySlot> {
        let slot = self.slots.get_mut(handle.slot)?;
        (slot.occupied && slot.epoch == handle.epoch && slot.generation == handle.generation).then_some(slot)
    }

    pub(super) fn handback(&mut self, handle: UiPatchApplyHandle, retirement: UiPatchRetirement) {
        if let Some(slot) = self.slot_mut(handle) {
            slot.retirement = Some(retirement);
        }
    }

    pub(super) fn release(&mut self, handle: UiPatchApplyHandle) {
        let Some(slot) = self.slot_mut(handle) else { return };
        if slot.retirement.is_some() {
            return;
        }
        let epoch = slot.epoch;
        *slot = UiPatchApplySlot { epoch, ..UiPatchApplySlot::default() };
    }

    pub(super) fn retire_one(&mut self) {
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

    pub(super) fn has_retirement(&self) -> bool {
        self.slots.iter().any(|slot| slot.occupied && slot.retirement.is_some())
    }
}

static UI_PATCH_APPLY_ARENA: Mutex<UiPatchApplyArena> = Mutex::new(UiPatchApplyArena::EMPTY);

pub(super) fn with_ui_patch_apply_arena<T>(f: impl FnOnce(&mut UiPatchApplyArena) -> T) -> T {
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
pub(super) struct UiPatchValidateFrame {
    pub(super) id: crate::UiNodeId,
    pub(super) depth: usize,
    pub(super) parent_in_section: bool,
    pub(super) in_section: bool,
    pub(super) child: usize,
    pub(super) stage: u8,
}
//#endregion 🔖️Apply
