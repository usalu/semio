//! 📨️ Exact pending patch roots shared by native and component reactor dispatch.

use semio_framework::kernel::ActorUiPatchReceipt;
use semio_framework_ui_contract::{self as ui_contract, UiPatch};
use semio_framework_ui_runtime::{SurfaceReconcilePublishedAck, SurfaceReconcilePublishedPatch, SurfaceReconcileReadyPatch};
use std::cell::RefCell;

//#region 📨️PendingPatchAuthority
const PENDING_PATCH_CAPACITY: usize = semio_framework_ui_runtime::SURFACE_RECONCILE_ADMISSION_SLOTS;

enum PendingPatchOwner {
    Reconcile(SurfaceReconcileReadyPatch),
    External(ui_contract::UiPendingPatch),
}

struct IssuedPatchAck {
    receipt: ActorUiPatchReceipt,
    surface: ui_contract::SurfaceId,
    revision: u64,
    committed: bool,
}

impl IssuedPatchAck {
    fn matches(&self, receipt: ActorUiPatchReceipt, surface: &str, revision: u64) -> bool {
        self.committed && self.receipt == receipt && self.surface.0.as_str() == surface && self.revision == revision
    }
}

struct PendingPatchSlot {
    sequence: u64,
    instance: Option<u32>,
    owner: PendingPatchOwner,
    published: Option<SurfaceReconcilePublishedPatch>,
    acknowledgement: Option<SurfaceReconcilePublishedAck>,
    acknowledged: bool,
    emitted: bool,
    issued: Option<IssuedPatchAck>,
    rejection_requested: bool,
}

#[derive(Clone, Copy)]
struct ClosingPending {
    key: super::instance_lifetime::NativeCloseKey,
    active: bool,
    complete: bool,
}

pub(super) struct PendingPatchAuthority {
    slots: [Option<PendingPatchSlot>; PENDING_PATCH_CAPACITY],
    closing_instances: [Option<ClosingPending>; PENDING_PATCH_CAPACITY],
    turn_handback: ui_contract::UiPendingPatch,
    turn_handback_instance: Option<u32>,
    turn_handback_sequence: Option<u64>,
    next_sequence: u64,
    exhausted: bool,
}

impl PendingPatchAuthority {
    pub(super) fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), closing_instances: [None; PENDING_PATCH_CAPACITY], turn_handback: Default::default(), turn_handback_instance: None, turn_handback_sequence: None, next_sequence: 0, exhausted: false }
    }

    pub(super) fn reserve_sequence(&mut self) -> Option<u64> {
        if self.exhausted || self.slots.iter().all(Option::is_some) {
            return None;
        }
        let sequence = self.next_sequence.checked_add(1)?;
        self.next_sequence = sequence;
        self.exhausted = sequence == u64::MAX;
        Some(sequence)
    }

    #[cfg(test)]
    pub(super) fn receipt_is_issued(&self, receipt: ActorUiPatchReceipt) -> bool {
        self.slots.iter().flatten().any(|slot| slot.issued.as_ref().is_some_and(|issued| issued.committed && issued.receipt == receipt))
    }

    pub(super) fn has_capacity(&self) -> bool {
        !self.exhausted && self.slots.iter().any(Option::is_none)
    }

    #[expect(clippy::result_large_err, reason = "Refusal must hand back the exact retained patch owner without allocating or releasing its publication credit.")]
    pub(super) fn push_reconcile(&mut self, owner: SurfaceReconcileReadyPatch) -> Result<(), SurfaceReconcileReadyPatch> {
        let instance = owner.surface().and_then(|surface| parse_surface_instance(&surface.0));
        if self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == instance) {
            return Err(owner);
        }
        let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(owner) };
        let Some(sequence) = self.reserve_sequence() else { return Err(owner) };
        self.slots[index] = Some(PendingPatchSlot { sequence, instance, owner: PendingPatchOwner::Reconcile(owner), published: None, acknowledgement: None, acknowledged: false, emitted: false, issued: None, rejection_requested: false });
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal must hand back the exact retained patch owner without allocating or releasing its publication credit.")]
    pub(super) fn push_external(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
        let instance = parse_surface_instance(&patch.surface.0);
        if self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == instance) {
            return Err(patch);
        }
        let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(patch) };
        let Some(sequence) = self.reserve_sequence() else { return Err(patch) };
        let mut owner = ui_contract::UiPendingPatch::default();
        *owner.source_mut().expect("new pending patch accepts its exact source") = Some(patch);
        self.slots[index] = Some(PendingPatchSlot { sequence, instance, owner: PendingPatchOwner::External(owner), published: None, acknowledgement: None, acknowledged: false, emitted: false, issued: None, rejection_requested: false });
        Ok(())
    }

    pub(super) fn take_one(&mut self, admitted_bytes: usize) -> Result<Option<UiPatch>, &'static str> {
        if !self.turn_handback.terminal_is_empty() && self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == self.turn_handback_instance) {
            return Ok(None);
        }
        if let Some(patch) = self.turn_handback.source_mut()?.take() {
            return Ok(Some(patch));
        }
        if self.turn_handback_sequence.is_some() {
            return Err("pending patch turn is already borrowed");
        }
        let Some(index) = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.as_ref().is_some_and(|slot| !slot.emitted && !self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == slot.instance)))
            .min_by_key(|(_, slot)| slot.as_ref().map(|slot| slot.sequence))
            .map(|(index, _)| index)
        else {
            return Ok(None);
        };
        let slot = self.slots[index].as_mut().ok_or("pending publication source disappeared")?;
        match &mut slot.owner {
            PendingPatchOwner::Reconcile(owner) => {
                if owner.publish_into(&mut self.turn_handback, &mut slot.published, admitted_bytes)? == 0 {
                    return Ok(None);
                }
                self.turn_handback_instance = slot.instance;
                self.turn_handback_sequence = Some(slot.sequence);
                slot.emitted = true;
                Ok(None)
            }
            PendingPatchOwner::External(patch) => {
                if admitted_bytes < size_of::<UiPatch>() {
                    return Ok(None);
                }
                *self.turn_handback.source_mut()? = patch.source_mut()?.take();
                self.turn_handback_instance = slot.instance;
                self.turn_handback_sequence = Some(slot.sequence);
                slot.emitted = true;
                Ok(None)
            }
        }
    }

    #[expect(clippy::result_large_err, reason = "Refusal must hand back the exact retained patch owner without allocating or releasing its publication credit.")]
    pub(super) fn hand_back_turn(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
        if !self.turn_handback.terminal_is_empty() || self.turn_handback_sequence.is_none() || self.turn_handback_instance != parse_surface_instance(&patch.surface.0) {
            return Err(patch);
        }
        if let Some(slot) = self.slots.iter_mut().flatten().find(|slot| Some(slot.sequence) == self.turn_handback_sequence) {
            if slot.issued.as_ref().is_some_and(|issued| issued.committed) {
                return Err(patch);
            }
            slot.issued = None;
        } else {
            return Err(patch);
        }
        let Ok(source) = self.turn_handback.source_mut() else {
            return Err(patch);
        };
        self.turn_handback_instance = parse_surface_instance(&patch.surface.0);
        *source = Some(patch);
        Ok(())
    }

    pub(super) fn stage_emission(&mut self, receipt: ActorUiPatchReceipt, patch: &UiPatch) -> Result<(), &'static str> {
        if !receipt.is_valid() || Some(receipt.lifetime.instance_id) != self.turn_handback_instance || parse_surface_instance(&patch.surface.0) != self.turn_handback_instance {
            return Err("patch receipt names another lifetime");
        }
        let slot = self.slots.iter_mut().flatten().find(|slot| Some(slot.sequence) == self.turn_handback_sequence).ok_or("pending patch emission source absent")?;
        if slot.issued.is_some() {
            return Err("pending patch emission already staged");
        }
        slot.issued = Some(IssuedPatchAck { receipt, surface: patch.surface.clone(), revision: patch.revision.0, committed: false });
        Ok(())
    }

    pub(super) fn commit_emission(&mut self) {
        let sequence = self.turn_handback_sequence.take().expect("prepared patch emission sequence");
        let slot = self.slots.iter_mut().flatten().find(|slot| slot.sequence == sequence).expect("prepared patch emission owner");
        let issued = slot.issued.as_mut().expect("prepared patch receipt");
        assert!(!issued.committed && self.turn_handback.terminal_is_empty());
        issued.committed = true;
        self.turn_handback_instance = None;
    }

    pub(super) fn apply_issued_ack(&mut self, receipt: ActorUiPatchReceipt, surface: &str, revision: u64, admitted_bytes: usize, advance: impl FnOnce(&SurfaceReconcilePublishedAck) -> Result<bool, &'static str>) -> Result<bool, &'static str> {
        let Some(slot) = self.slots.iter_mut().flatten().find(|slot| !slot.acknowledged && !slot.rejection_requested && slot.issued.as_ref().is_some_and(|issued| issued.matches(receipt, surface, revision))) else {
            return Ok(false);
        };
        if matches!(slot.owner, PendingPatchOwner::External(_)) {
            slot.acknowledged = true;
        } else {
            if slot.acknowledgement.is_none() && !SurfaceReconcilePublishedPatch::acknowledge_into(&mut slot.published, &mut slot.acknowledgement, surface, revision, admitted_bytes)? {
                return Ok(false);
            }
            slot.acknowledged = advance(slot.acknowledgement.as_ref().ok_or("published ACK source disappeared")?)?;
        }
        if slot.acknowledged {
            slot.issued = None;
        }
        Ok(slot.acknowledged)
    }

    pub(super) fn apply_issued_rejection(&mut self, receipt: ActorUiPatchReceipt, surface: &str, revision: u64, reject: impl FnOnce(u64) -> bool) -> bool {
        let Some(slot) = self.slots.iter_mut().flatten().find(|slot| !slot.acknowledged && slot.issued.as_ref().is_some_and(|issued| issued.matches(receipt, surface, revision))) else {
            return false;
        };
        slot.rejection_requested = true;
        Self::reject_slot(slot, reject)
    }

    fn reject_slot(slot: &mut PendingPatchSlot, reject: impl FnOnce(u64) -> bool) -> bool {
        if matches!(slot.owner, PendingPatchOwner::Reconcile(_)) {
            let generation = slot.published.as_ref().map(SurfaceReconcilePublishedPatch::generation).or_else(|| slot.acknowledgement.as_ref().map(SurfaceReconcilePublishedAck::generation));
            let Some(generation) = generation else {
                return false;
            };
            if !reject(generation) {
                return false;
            }
        }
        slot.issued = None;
        slot.acknowledged = true;
        slot.rejection_requested = false;
        true
    }

    pub(super) fn advance_rejection(&mut self, reject: impl FnOnce(&str, u64) -> bool) -> bool {
        let Some(slot) = self.slots.iter_mut().flatten().find(|slot| slot.rejection_requested && slot.issued.is_some()) else {
            return false;
        };
        let surface = slot.issued.as_ref().expect("retained rejection receipt").surface.clone();
        Self::reject_slot(slot, |generation| reject(&surface.0, generation))
    }

    pub(super) fn close_instance_step(&mut self, instance: u32, maximum_items: usize, maximum_bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(ui_contract::UiValueRetirementStep::default());
        }
        if self.turn_handback_instance == Some(instance) {
            let mut step = self.turn_handback.close_step(maximum_items, maximum_bytes)?;
            if self.turn_handback.terminal_is_empty() {
                self.turn_handback_instance = None;
                self.turn_handback_sequence = None;
                self.turn_handback = Default::default();
            }
            step.complete = false;
            return Ok(step);
        }
        let Some(index) = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == Some(instance))) else {
            return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() });
        };
        self.close_slot_step(index, maximum_items, maximum_bytes)
    }

    fn close_slot_step(&mut self, index: usize, maximum_items: usize, maximum_bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(ui_contract::UiValueRetirementStep::default());
        }
        let slot = self.slots[index].as_mut().ok_or("exact pending patch slot disappeared")?;
        let mut step = if let Some(ack) = slot.acknowledgement.as_mut() {
            let step = ack.close_step_with_grant(maximum_items, maximum_bytes)?;
            if step.complete && ack.terminal_is_empty() {
                slot.acknowledgement = None;
            }
            ui_contract::UiValueRetirementStep { complete: false, ..step }
        } else if let Some(published) = slot.published.as_mut() {
            let step = published.close_step_with_grant(maximum_items, maximum_bytes)?;
            if step.complete && published.terminal_is_empty() {
                slot.published = None;
            }
            ui_contract::UiValueRetirementStep { complete: false, ..step }
        } else {
            match &mut slot.owner {
                PendingPatchOwner::Reconcile(owner) => owner.close_step_with_grant(maximum_items, maximum_bytes)?,
                PendingPatchOwner::External(patch) => {
                    let mut step = patch.close_step(maximum_items, maximum_bytes)?;
                    step.complete &= patch.terminal_is_empty();
                    step
                }
            }
        };
        if step.complete {
            self.slots[index] = None;
        }
        step.complete = false;
        Ok(step)
    }

    pub(super) fn preflight_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        if let Some(closing) = self.closing_instances.iter().flatten().find(|closing| closing.key.instance() == key.instance()) {
            return if closing.key == key { Ok(()) } else { Err("pending patch reservation belongs to another allocation") };
        }
        if self.closing_instances.iter().all(Option::is_some) {
            return Err("pending patch close reservations are full");
        }
        Ok(())
    }

    pub(super) fn reserve_close_instance(&mut self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        if let Some(closing) = self.closing_instances.iter().flatten().find(|closing| closing.key.instance() == key.instance()) {
            return if closing.key == key { Ok(()) } else { Err("pending patch reservation belongs to another allocation") };
        }
        let slot = self.closing_instances.iter_mut().find(|slot| slot.is_none()).ok_or("pending patch close reservations are full")?;
        *slot = Some(ClosingPending { key, active: false, complete: false });
        Ok(())
    }

    pub(super) fn activate_close_instance(&mut self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        let closing = self.closing_instances.iter_mut().flatten().find(|closing| closing.key == key).ok_or("exact pending patch close reservation missing")?;
        closing.active = true;
        for slot in self.slots.iter_mut().flatten().filter(|slot| slot.instance == Some(key.instance())) {
            slot.issued = None;
            slot.rejection_requested = false;
        }
        Ok(())
    }

    pub(super) fn close_instance_complete(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<bool, &'static str> {
        self.closing_instances.iter().flatten().find(|closing| closing.key == key).map(|closing| closing.complete).ok_or("exact pending patch close receipt missing")
    }

    pub(super) fn release_close_instance(&mut self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        let closing = self.closing_instances.iter_mut().find(|closing| closing.is_some_and(|closing| closing.key == key && closing.complete)).ok_or("exact pending patch close receipt is not terminal")?;
        *closing = None;
        Ok(())
    }

    /// 🧹️ Retires one unit of the oldest acknowledged publication, or of a closing instance, against
    /// the caller's grant. The grant is the whole point: an acknowledged slot keeps
    /// [`Self::has_unpublished`] TRUE, which keeps the reactor turn in `MoreWork`, which costs the
    /// host one turn ROUND TRIP per unit — so a unit priced at one item retires a document-scaled
    /// world-3d patch (≈ 460 `UiText` slices across 11 lane carriers) one host round trip at a time.
    /// Measured 2026-09-10 (W-S2): 24.3 s and 8 799 worker messages between the Nakagin example click
    /// and the repaint. Every other stage of the same turn is priced per PAGE; this one now is too.
    pub(super) fn close_step(&mut self, items: usize, bytes: usize) -> Result<bool, &'static str> {
        if let Some(index) = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.acknowledged)) {
            self.close_slot_step(index, items, bytes)?;
            return Ok(false);
        }
        let Some(index) = self.closing_instances.iter().position(|closing| closing.is_some_and(|closing| closing.active && !closing.complete)) else { return Ok(true) };
        let Some(closing) = self.closing_instances[index] else { return Err("pending patch close reservation disappeared") };
        if self.close_instance_step(closing.key.instance(), items, bytes)?.complete {
            self.closing_instances[index].as_mut().expect("exact retained pending patch receipt").complete = true;
        }
        Ok(false)
    }

    /// 🐞️ Temporary trace summary of the publication slots, read by the reactor's more-work streak trace.
    pub(super) fn debug_state(&self) -> String {
        let slots: Vec<String> = self
            .slots
            .iter()
            .flatten()
            .map(|slot| format!("{}:{}{}{}{}{}", slot.sequence, if slot.emitted { "e" } else { "-" }, if slot.acknowledged { "a" } else { "-" }, match &slot.issued { Some(issued) if issued.committed => "c", Some(_) => "i", None => "-" }, if slot.rejection_requested { "r" } else { "-" }, if slot.published.is_some() { "p" } else { "-" }))
            .collect();
        format!("slots=[{}] handback_empty={} handback_sequence={:?} exhausted={} closing={}", slots.join(","), self.turn_handback.terminal_is_empty(), self.turn_handback_sequence, self.exhausted, self.closing_instances.iter().flatten().count())
    }

    pub(super) fn has_unpublished(&self) -> bool {
        !self.turn_handback.terminal_is_empty() || self.slots.iter().flatten().any(|slot| !slot.emitted || slot.acknowledged || slot.rejection_requested) || self.closing_instances.iter().any(Option::is_some)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🩹️receipt/🦀️.rs"]
mod issued_receipt_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️instance-lifetime-patch-close/🦀️.rs"]
mod instance_lifetime_patch_close_tests;

crate::component_persistent_local! {
    static PENDING_PATCHES: RefCell<PendingPatchAuthority> = RefCell::new(PendingPatchAuthority::new());
}

/// 🪪️ Surfaces are named `"<instance>:<body-key>"` in this wave (no dedicated `surface-ref`
/// bookkeeping table yet — `ui.wit`'s `surface-ref` record exists at the WIT boundary, but the
/// Rust-side `kernel::UiPatch.surface` is still a plain `String` per A3's landed shape).
pub(super) fn parse_surface_instance(surface: &str) -> Option<u32> {
    surface.split(':').next()?.parse().ok()
}

pub(super) fn with_state<R>(use_state: impl FnOnce(&RefCell<PendingPatchAuthority>) -> R) -> R {
    PENDING_PATCHES.with(use_state)
}
//#endregion 📨️PendingPatchAuthority
