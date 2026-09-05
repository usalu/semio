//! 📨️ Exact pending patch roots shared by native and component reactor dispatch.

use semio_framework_ui_contract::{self as ui_contract, UiPatch};
use semio_framework_ui_runtime::{SurfaceReconcileReadyPatch, SurfaceReconcilePublishedPatch, SurfaceReconcilePublishedAck};
use std::cell::RefCell;

//#region 📨️PendingPatchAuthority
const PENDING_PATCH_CAPACITY: usize = semio_framework_ui_runtime::SURFACE_RECONCILE_ADMISSION_SLOTS;

enum PendingPatchOwner {
    Reconcile(SurfaceReconcileReadyPatch),
    External(ui_contract::UiPendingPatch),
}

struct PendingPatchSlot {
    sequence: u64,
    instance: Option<u32>,
    owner: PendingPatchOwner,
    published: Option<SurfaceReconcilePublishedPatch>,
    acknowledgement: Option<SurfaceReconcilePublishedAck>,
    acknowledged: bool,
    emitted: bool,
}

#[derive(Clone, Copy)]
struct ClosingPending { key: super::instance_lifetime::NativeCloseKey, active: bool, complete: bool }

pub(super) struct PendingPatchAuthority {
    slots: [Option<PendingPatchSlot>; PENDING_PATCH_CAPACITY],
    closing_instances: [Option<ClosingPending>; PENDING_PATCH_CAPACITY],
    turn_handback: ui_contract::UiPendingPatch,
    turn_handback_instance: Option<u32>,
    next_sequence: u64,
    exhausted: bool,
}

impl PendingPatchAuthority {
    pub(super) fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), closing_instances: [None; PENDING_PATCH_CAPACITY], turn_handback: Default::default(), turn_handback_instance: None, next_sequence: 0, exhausted: false }
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

    pub(super) fn has_capacity(&self) -> bool {
        !self.exhausted && self.slots.iter().any(Option::is_none)
    }

    pub(super) fn push_reconcile(&mut self, owner: SurfaceReconcileReadyPatch) -> Result<(), SurfaceReconcileReadyPatch> {
        let instance = owner.surface().and_then(|surface| parse_surface_instance(&surface.0));
        if self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == instance) { return Err(owner); }
        let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(owner) };
        let Some(sequence) = self.reserve_sequence() else { return Err(owner) };
        self.slots[index] = Some(PendingPatchSlot { sequence, instance, owner: PendingPatchOwner::Reconcile(owner), published: None, acknowledgement: None, acknowledged: false, emitted: false });
        Ok(())
    }

    pub(super) fn push_external(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
        let instance = parse_surface_instance(&patch.surface.0);
        if self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == instance) { return Err(patch); }
        let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(patch) };
        let Some(sequence) = self.reserve_sequence() else { return Err(patch) };
        let mut owner = ui_contract::UiPendingPatch::default();
        *owner.source_mut().expect("new pending patch accepts its exact source") = Some(patch);
        self.slots[index] = Some(PendingPatchSlot { sequence, instance, owner: PendingPatchOwner::External(owner), published: None, acknowledgement: None, acknowledged: false, emitted: false });
        Ok(())
    }

    pub(super) fn take_one(&mut self, admitted_bytes: usize) -> Result<Option<UiPatch>, &'static str> {
        if !self.turn_handback.terminal_is_empty() && self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == self.turn_handback_instance) { return Ok(None); }
        if let Some(patch) = self.turn_handback.source_mut()?.take() {
            self.turn_handback_instance = None;
            return Ok(Some(patch));
        }
        let Some(index) =
            self.slots.iter().enumerate().filter(|(_, slot)| slot.as_ref().is_some_and(|slot| !slot.emitted && !self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == slot.instance))).min_by_key(|(_, slot)| slot.as_ref().map(|slot| slot.sequence)).map(|(index, _)| index) else { return Ok(None); };
        let slot = self.slots[index].as_mut().ok_or("pending publication source disappeared")?;
        match &mut slot.owner {
            PendingPatchOwner::Reconcile(owner) => {
                if owner.publish_into(&mut self.turn_handback, &mut slot.published, admitted_bytes)? == 0 { return Ok(None); }
                self.turn_handback_instance = slot.instance;
                slot.emitted = true;
                Ok(None)
            }
            PendingPatchOwner::External(patch) => {
                if admitted_bytes < std::mem::size_of::<UiPatch>() { return Ok(None); }
                *self.turn_handback.source_mut()? = patch.source_mut()?.take();
                self.turn_handback_instance = slot.instance;
                slot.emitted = true;
                Ok(None)
            }
        }
    }

    pub(super) fn hand_back_turn(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
        if !self.turn_handback.terminal_is_empty() { return Err(patch); }
        let Ok(source) = self.turn_handback.source_mut() else { return Err(patch); };
        self.turn_handback_instance = parse_surface_instance(&patch.surface.0);
        *source = Some(patch);
        Ok(())
    }

    pub(super) fn apply_published_ack(&mut self, surface: &str, revision: u64, admitted_bytes: usize, advance: impl FnOnce(&SurfaceReconcilePublishedAck) -> Result<bool, &'static str>) -> Result<bool, &'static str> {
        let Some(slot) = self.slots.iter_mut().flatten().find(|slot| slot.published.as_ref().is_some_and(|owner| owner.matches(surface, revision)) || slot.acknowledgement.as_ref().is_some_and(|owner| owner.surface().is_some_and(|exact| exact.0.as_str() == surface) && owner.revision().0 == revision)) else { return Ok(false); };
        if slot.acknowledged { return Ok(true); }
        if slot.acknowledgement.is_none() && !SurfaceReconcilePublishedPatch::acknowledge_into(&mut slot.published, &mut slot.acknowledgement, surface, revision, admitted_bytes)? { return Ok(false); }
        slot.acknowledged = advance(slot.acknowledgement.as_ref().ok_or("published ACK source disappeared")?)?;
        Ok(slot.acknowledged)
    }

    pub(super) fn close_instance_step(&mut self, instance: u32, maximum_items: usize, maximum_bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(ui_contract::UiValueRetirementStep::default()); }
        if self.turn_handback_instance == Some(instance) {
            let mut step = self.turn_handback.close_step(maximum_items, maximum_bytes)?;
            if self.turn_handback.terminal_is_empty() {
                self.turn_handback_instance = None;
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
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(ui_contract::UiValueRetirementStep::default()); }
        let slot = self.slots[index].as_mut().ok_or("exact pending patch slot disappeared")?;
        let mut step = if let Some(ack) = slot.acknowledgement.as_mut() {
            let step = ack.close_step_with_grant(maximum_items, maximum_bytes)?;
            if step.complete && ack.terminal_is_empty() { slot.acknowledgement = None; }
            ui_contract::UiValueRetirementStep { complete: false, ..step }
        } else if let Some(published) = slot.published.as_mut() {
            let step = published.close_step_with_grant(maximum_items, maximum_bytes)?;
            if step.complete && published.terminal_is_empty() { slot.published = None; }
            ui_contract::UiValueRetirementStep { complete: false, ..step }
        } else { match &mut slot.owner {
            PendingPatchOwner::Reconcile(owner) => owner.close_step_with_grant(maximum_items, maximum_bytes)?,
            PendingPatchOwner::External(patch) => {
                let mut step = patch.close_step(maximum_items, maximum_bytes)?;
                step.complete &= patch.terminal_is_empty();
                step
            }
        } };
        if step.complete {
            self.slots[index] = None;
        }
        step.complete = false;
        Ok(step)
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

    pub(super) fn close_step(&mut self) -> Result<bool, &'static str> {
        if let Some(index) = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.acknowledged)) {
            self.close_slot_step(index, 1, 4096)?;
            return Ok(false);
        }
        let Some(index) = self.closing_instances.iter().position(|closing| closing.is_some_and(|closing| closing.active && !closing.complete)) else { return Ok(true) };
        let Some(closing) = self.closing_instances[index] else { return Err("pending patch close reservation disappeared") };
        if self.close_instance_step(closing.key.instance(), 1, 4096)?.complete {
            self.closing_instances[index].as_mut().expect("exact retained pending patch receipt").complete = true;
        }
        Ok(false)
    }

    pub(super) fn has_unpublished(&self) -> bool {
        !self.turn_handback.terminal_is_empty() || self.slots.iter().flatten().any(|slot| !slot.emitted || slot.acknowledged) || self.closing_instances.iter().any(Option::is_some)
    }
}

#[cfg(test)]
mod instance_lifetime_patch_close_tests {
    use super::*;

    fn patch(surface: &str) -> UiPatch {
        UiPatch { surface: ui_contract::SurfaceId::try_from(surface).unwrap(), base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(1), ops: Default::default() }
    }

    #[test]
    fn guest_instance_lifecycle_pending_patch_handback_preserves_rejected_owner_and_exact_bytes() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🚪️lifetime/🧫️fixture/🔣️.json")).unwrap();
        for grant in fixture["pendingPatch"]["grants"].as_array().unwrap() {
            let mut pending = PendingPatchAuthority::new();
            let first = fixture["pendingPatch"]["surfaces"][0].as_str().unwrap();
            let second = fixture["pendingPatch"]["surfaces"][1].as_str().unwrap();
            pending.hand_back_turn(patch(first)).unwrap();
            let rejected = pending.hand_back_turn(patch(second)).unwrap_err();
            assert_eq!(serde_json::to_value(&rejected).unwrap()["surface"], second);
            let mut rejected_owner = ui_contract::UiPendingPatch::default();
            *rejected_owner.source_mut().unwrap() = Some(rejected);
            while !rejected_owner.terminal_is_empty() { rejected_owner.close_step(1, 4096).unwrap(); }
            let key = super::super::instance_lifetime::NativeCloseKey::fixture(7, 1);
            pending.reserve_close_instance(key).unwrap();
            pending.activate_close_instance(key).unwrap();
            let before = serde_json::to_value(pending.turn_handback.get().unwrap()).unwrap();
            assert_eq!(pending.close_instance_step(7, 0, 4096).unwrap(), ui_contract::UiValueRetirementStep::default());
            assert_eq!(pending.close_instance_step(7, 1, 0).unwrap(), ui_contract::UiValueRetirementStep::default());
            assert_eq!(serde_json::to_value(pending.turn_handback.get().unwrap()).unwrap(), before);
            let mut bytes = 0;
            let grant = grant.as_u64().unwrap() as usize;
            for turn in 0..1024 {
                let step = pending.close_instance_step(7, 1, grant).unwrap();
                assert!(step.released_items <= 1 && step.released_bytes <= grant);
                bytes += step.released_bytes;
                if step.complete { break; }
                assert!(turn < 1023);
            }
            assert_eq!(bytes, first.as_bytes().len());
            assert!(pending.turn_handback.terminal_is_empty());
            assert!(!pending.close_instance_complete(key).unwrap());
            pending.close_step().unwrap();
            assert!(pending.close_instance_complete(key).unwrap());
            pending.release_close_instance(key).unwrap();
        }
    }

    #[test]
    fn guest_instance_lifecycle_pending_patch_unwind_keeps_the_exact_typed_cursor_mounted() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🚪️lifetime/🧫️fixture/🔣️.json")).unwrap();
        let mut pending = PendingPatchAuthority::new();
        pending.push_external(patch(fixture["pendingPatch"]["surfaces"][0].as_str().unwrap())).unwrap();
        let key = super::super::instance_lifetime::NativeCloseKey::fixture(7, 1);
        pending.reserve_close_instance(key).unwrap();
        pending.activate_close_instance(key).unwrap();
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pending.close_instance_step(7, 1, 1).unwrap();
            panic!("injected after partial typed retirement");
        }));
        assert!(failure.is_err());
        assert_eq!(pending.slots[0].is_some(), fixture["pendingPatch"]["faultLeavesStructuralOwner"].as_bool().unwrap());
        assert!(!pending.close_instance_complete(key).unwrap());
        for turn in 0..4096 {
            pending.close_step().unwrap();
            if pending.close_instance_complete(key).unwrap() { break; }
            assert!(turn < 4095);
        }
        pending.release_close_instance(key).unwrap();
        assert!(pending.slots.iter().all(Option::is_none));
    }

    #[test]
    fn instance_lifetime_pending_patch_keeps_scope_after_payload_surface_retires() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json")).unwrap();
        let mut pending = PendingPatchAuthority::new();
        pending.push_external(UiPatch { surface: ui_contract::SurfaceId::try_from("7:retained").unwrap(), base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(1), ops: Default::default() }).unwrap();
        let key = super::super::instance_lifetime::NativeCloseKey::fixture(7, 1);
        pending.reserve_close_instance(key).unwrap();
        pending.activate_close_instance(key).unwrap();
        if let Some(PendingPatchSlot { owner: PendingPatchOwner::External(owner), .. }) = pending.slots[0].as_mut() { owner.source_mut().unwrap().as_mut().unwrap().surface = Default::default(); }
        for turn in 0..1024 {
            pending.close_step().unwrap();
            if pending.slots[0].is_none() { break; }
            assert!(turn < 1023);
        }
        assert_eq!(pending.slots[0].is_none(), fixture["nativeCases"]["scopeAfterSurfaceClear"].as_bool().unwrap());
        assert!(!pending.close_step().unwrap());
        assert!(pending.close_step().unwrap());
        assert!(pending.close_instance_complete(key).unwrap());
        pending.release_close_instance(key).unwrap();
        assert!(pending.close_instance_complete(key).is_err());
    }
}

crate::component_persistent_local! {
    static PENDING_PATCHES: RefCell<PendingPatchAuthority> = RefCell::new(PendingPatchAuthority::new());
}

/// 🪪️ Surfaces are named `"<instance>:<body-key>"` in this wave (no dedicated `surface-ref`
/// bookkeeping table yet — `ui.wit`'s `surface-ref` record exists at the WIT boundary, but the
/// Rust-side `kernel::UiPatch.surface` is still a plain `String` per A3's landed shape).
pub(super) fn parse_surface_instance(surface: &str) -> Option<u32> {
    surface.split(':').next()?.parse().ok()
}


pub(super) fn with_state<R>(use_state: impl FnOnce(&RefCell<PendingPatchAuthority>) -> R) -> R { PENDING_PATCHES.with(use_state) }
//#endregion 📨️PendingPatchAuthority
