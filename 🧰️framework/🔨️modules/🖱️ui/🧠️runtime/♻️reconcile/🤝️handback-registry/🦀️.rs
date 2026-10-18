//! 🤝️ Handback registry: the handback slots, keys, reservations and registry with the reserve, release, rebind and acquire operations. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
//#region 🎟️RetainedAuthority
pub const SURFACE_RECONCILE_HANDBACK_SLOTS: usize = SURFACE_RECONCILE_ADMISSION_SLOTS * 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceReconcileHandbackKey {
    pub(super) slot: usize,
    pub(super) epoch: u64,
    pub(super) generation: u64,
}

#[derive(Debug)]
pub(super) struct SurfaceReconcileHandbackReservation {
    pub(super) key: SurfaceReconcileHandbackKey,
}

#[derive(Default)]
pub(super) struct SurfaceReconcileHandbackSlot {
    pub(super) epoch: u64,
    pub(super) generation: u64,
    pub(super) reserved: bool,
    pub(super) queued: bool,
    pub(super) state: Option<Box<SurfaceReconcileRetained>>,
}

pub(super) struct SurfaceReconcileHandbackRegistry {
    pub(super) slots: [SurfaceReconcileHandbackSlot; SURFACE_RECONCILE_HANDBACK_SLOTS],
    pub(super) free: [usize; SURFACE_RECONCILE_HANDBACK_SLOTS],
    pub(super) free_len: usize,
    pub(super) retirement: [usize; SURFACE_RECONCILE_HANDBACK_SLOTS],
    pub(super) retirement_head: usize,
    pub(super) retirement_len: usize,
}

impl Default for SurfaceReconcileHandbackRegistry {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| SurfaceReconcileHandbackSlot::default()),
            free: std::array::from_fn(|index| SURFACE_RECONCILE_HANDBACK_SLOTS - 1 - index),
            free_len: SURFACE_RECONCILE_HANDBACK_SLOTS,
            retirement: [usize::MAX; SURFACE_RECONCILE_HANDBACK_SLOTS],
            retirement_head: 0,
            retirement_len: 0,
        }
    }
}

pub(super) static SURFACE_RECONCILE_HANDBACKS: LazyLock<Mutex<SurfaceReconcileHandbackRegistry>> = LazyLock::new(|| Mutex::new(SurfaceReconcileHandbackRegistry::default()));

pub(super) fn reserve_surface_reconcile_handback(generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    if generation == 0 {
        return None;
    }
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if registry.free_len == 0 {
        return None;
    }
    let slot = registry.free[registry.free_len - 1];
    let epoch = registry.slots[slot].epoch.checked_add(1)?;
    registry.free_len -= 1;
    registry.slots[slot] = SurfaceReconcileHandbackSlot { epoch, generation, reserved: true, queued: false, state: None };
    Some(SurfaceReconcileHandbackReservation { key: SurfaceReconcileHandbackKey { slot, epoch, generation } })
}

pub(super) fn try_reserve_surface_reconcile_handback(generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    if generation == 0 { return None; }
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return None,
        Err(std::sync::TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
    };
    if registry.free_len == 0 { return None; }
    let slot = registry.free[registry.free_len - 1];
    let epoch = registry.slots[slot].epoch.checked_add(1)?;
    registry.free_len -= 1;
    registry.slots[slot] = SurfaceReconcileHandbackSlot { epoch, generation, reserved: true, queued: false, state: None };
    Some(SurfaceReconcileHandbackReservation { key: SurfaceReconcileHandbackKey { slot, epoch, generation } })
}

#[expect(clippy::needless_pass_by_value, reason = "Releasing a reservation consumes its unique slot authority so callers cannot reuse it.")]
pub(super) fn release_surface_reconcile_handback_in(registry: &mut SurfaceReconcileHandbackRegistry, reservation: SurfaceReconcileHandbackReservation) {
    let should_free = {
        let Some(slot) = registry.slots.get_mut(reservation.key.slot) else { return };
        if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() {
            return;
        }
        slot.reserved = false;
        slot.generation = 0;
        !slot.queued
    };
    if should_free {
        let index = registry.free_len;
        registry.free[index] = reservation.key.slot;
        registry.free_len += 1;
    }
}

pub(super) fn release_surface_reconcile_handback(reservation: SurfaceReconcileHandbackReservation) {
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    release_surface_reconcile_handback_in(&mut registry, reservation);
}

pub(super) fn release_surface_reconcile_handback_one(owner: &mut Option<SurfaceReconcileHandbackReservation>) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
    let Some(reservation) = owner.as_ref() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(Default::default()),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    let slot = registry.slots.get(reservation.key.slot).ok_or("surface handback slot is invalid")?;
    if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() { return Err("surface handback is not the exact retained owner"); }
    let free = !slot.queued;
    if free && registry.free_len >= SURFACE_RECONCILE_HANDBACK_SLOTS { return Err("surface handback free list exhausted"); }
    let slot = &mut registry.slots[reservation.key.slot];
    slot.reserved = false; slot.generation = 0;
    if free { let index = registry.free_len; registry.free[index] = reservation.key.slot; registry.free_len += 1; }
    owner.take();
    Ok(ui_contract::UiValueRetirementStep { complete: true, progressed: true, released_items: 1, released_bytes: 0 })
}

fn rebind_surface_reconcile_handback(mut reservation: SurfaceReconcileHandbackReservation, generation: u64) -> Result<SurfaceReconcileHandbackReservation, SurfaceReconcileHandbackReservation> {
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(slot) = registry.slots.get_mut(reservation.key.slot) else { return Err(reservation) };
    if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() || generation == 0 {
        return Err(reservation);
    }
    slot.generation = generation;
    reservation.key.generation = generation;
    Ok(reservation)
}

pub(super) fn acquire_surface_reconcile_handback(owner: &mut Option<SurfaceReconcileHandbackReservation>, generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    let Some(reservation) = owner.take() else { return reserve_surface_reconcile_handback(generation) };
    match rebind_surface_reconcile_handback(reservation, generation) {
        Ok(reservation) => Some(reservation),
        Err(reservation) => {
            *owner = Some(reservation);
            None
        }
    }
}

pub(super) fn acquire_reserved_surface_reconcile_handback(owner: &mut Option<SurfaceReconcileHandbackReservation>, reserved: &mut Option<SurfaceReconcileHandbackReservation>, generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    if owner.is_none() {
        return reserved.take();
    }
    acquire_surface_reconcile_handback(owner, generation)
}
//#endregion 🎟️RetainedAuthority
//#endregion ⏭️ResumableReconcile
