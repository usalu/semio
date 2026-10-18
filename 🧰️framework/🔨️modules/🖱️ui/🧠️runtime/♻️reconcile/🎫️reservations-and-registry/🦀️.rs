//! 🎫️ Reservations and registry: the reconcile admission limits, the reservation and its refusals, the registry census and the job phase. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
//#region 🎟️RetainedAuthority
pub const SURFACE_RECONCILE_ADMISSION_SLOTS: usize = ui_contract::UI_RESIDENT_SLOTS;

pub const SURFACE_RECONCILE_AGGREGATE_BYTES: usize = ui_contract::UI_RESIDENT_AGGREGATE_BYTES;

pub const SURFACE_RECONCILE_AGGREGATE_ITEMS: usize = ui_contract::UI_RESIDENT_AGGREGATE_ITEMS;

pub(super) fn reserve_surface_reconcile(limits: SurfaceReconcileLimits) -> Option<ui_contract::UiResidentPermit> {
    if !surface_reconcile_limits_admissible(limits) { return None; }
    if !register_surface_reconcile_backing(SURFACE_RECONCILE_PAGE_BYTES).ok()? { return None; }
    reserve_surface_reconcile_floor(limits)
}

pub(super) fn register_surface_reconcile_backing(admitted_bytes: usize) -> Result<bool, ui_contract::UiResidentFault> {
    ui_contract::UiResidentPermit::try_register_runtime_backing(size_of::<LazyLock<Mutex<SurfaceReconcileHandbackRegistry>>>() + SurfaceReconcileOutputs::static_backing_bytes(), admitted_bytes)
}

/// 🎫️ Pre-materialization aggregate reservation transferred into exactly one live job.
pub struct SurfaceReconcileReservation {
    pub(super) generation: u64,
    pub(super) limits: SurfaceReconcileLimits,
    pub(super) credit: Option<ui_contract::UiResidentPermit>,
    pub(super) handback: Option<SurfaceReconcileHandbackReservation>,
    pub(super) output_handback: Option<SurfaceReconcileHandbackReservation>,
}

/// 🩺️ WHICH of the three process-wide tables refused one render reservation.
///
/// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56: a refusal here reached the reactor census as the
/// single opaque word `registry-reservation-unavailable`, so a guest livelocked in a refuse/defer cycle
/// (wave B54 §6.4: `more-work` streak 1 008 with every slot clean) could not say whether the aggregate
/// resident CREDIT ledger was exhausted, the handback registry was full, or the generation was invalid —
/// three different defects with three different fixes. Every one of them is named now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileReservationRefusal {
    Generation,
    ResidentCredit,
    HandbackSlots,
    OutputHandbackSlots,
}

impl SurfaceReconcileReservationRefusal {
    pub const fn reason(self) -> &'static str {
        match self {
            Self::Generation => "registry-generation-invalid",
            Self::ResidentCredit => "registry-resident-credit-exhausted",
            Self::HandbackSlots => "registry-handback-slots-exhausted",
            Self::OutputHandbackSlots => "registry-output-handback-slots-exhausted",
        }
    }
}

/// 📊️ What the two process-wide reconcile tables currently hold, for the reactor census.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceReconcileRegistryCensus {
    pub resident_items: usize,
    pub resident_bytes: usize,
    pub resident_slots: usize,
    pub resident_aggregate_bytes: usize,
    pub handback_free: usize,
    pub handback_slots: usize,
}

/// 📊️ Reads both reconcile registries without reserving anything, so a refusal can be priced
/// against what is actually held rather than guessed at. Answers the zero census when either table is
/// contended — a read that must never itself block a turn.
pub fn surface_reconcile_registry_census() -> SurfaceReconcileRegistryCensus {
    let resident = ui_contract::UiResidentPermit::snapshot().unwrap_or_default();
    let handback_free = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry.free_len,
        Err(std::sync::TryLockError::WouldBlock) => usize::MAX,
        Err(std::sync::TryLockError::Poisoned(poisoned)) => poisoned.into_inner().free_len,
    };
    SurfaceReconcileRegistryCensus {
        resident_items: resident.items,
        resident_bytes: resident.bytes,
        resident_slots: resident.used_slots,
        resident_aggregate_bytes: ui_contract::UI_RESIDENT_AGGREGATE_BYTES,
        handback_free,
        handback_slots: SURFACE_RECONCILE_HANDBACK_SLOTS,
    }
}

impl SurfaceReconcileReservation {
    pub fn try_new(generation: u64) -> Result<Self, SurfaceReconcileReservationRefusal> {
        if generation == 0 {
            return Err(SurfaceReconcileReservationRefusal::Generation);
        }
        let limits = SurfaceReconcileLimits::default();
        let credit = reserve_surface_reconcile(limits).ok_or(SurfaceReconcileReservationRefusal::ResidentCredit)?;
        let Some(handback) = reserve_surface_reconcile_handback(generation) else {
            release_surface_reconcile(credit);
            return Err(SurfaceReconcileReservationRefusal::HandbackSlots);
        };
        let Some(output_handback) = reserve_surface_reconcile_handback(generation) else {
            release_surface_reconcile_handback(handback);
            release_surface_reconcile(credit);
            return Err(SurfaceReconcileReservationRefusal::OutputHandbackSlots);
        };
        Ok(Self { generation, limits, credit: Some(credit), handback: Some(handback), output_handback: Some(output_handback) })
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
        if let Some(handback) = self.handback.take() {
            release_surface_reconcile_handback(handback);
        }
        if let Some(handback) = self.output_handback.take() {
            release_surface_reconcile_handback(handback);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SurfaceReconcileJobPhase {
    Drive,
    RetireCursor,
    RetirePrevious,
    SealCandidate,
    Ready,
    Fault,
    Closing,
}
//#endregion 🎟️RetainedAuthority
//#endregion ⏭️ResumableReconcile
