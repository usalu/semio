//! 🧾️ Reservations, retirement and witnesses: the texture admission, the cancel and stage faults, the reservation and stage claim with their retirement and close cursors and the operation witness ledger. See [`super`].

use super::*;

pub struct RasterTextureAdmission {
    pub(super) key: RasterTextureKey,
    pub(super) witness: RasterTextureWitness,
    pub(super) content_identity: RasterContentIdentity,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) bytes: usize,
    pub(super) staged_index: usize,
    pub(super) nonce: u64,
}

#[expect(clippy::large_enum_variant, reason = "Cancellation refusal returns the exact original admission without allocating another owner.")]
pub enum RasterTextureCancelFault {
    Returned { fault: &'static str, admission: RasterTextureAdmission },
    Retained(&'static str),
}

#[expect(clippy::large_enum_variant, reason = "Failed staging returns the exact admitted texture and view without allocating during refusal.")]
pub enum RasterTextureStageFault {
    Returned { fault: &'static str, admission: RasterTextureAdmission, texture: gpu_device::Texture, view: gpu_device::TextureView },
    Retained(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RasterTextureReservation {
    pub(super) key: RasterTextureKey,
    pub(super) witness: RasterTextureWitness,
    pub(super) content_identity: RasterContentIdentity,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) bytes: usize,
    pub(super) staged_index: usize,
    pub(super) nonce: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RasterTextureStageClaim {
    pub(super) reservation: RasterTextureReservation,
    pub(super) candidate: RasterTextureWitness,
    pub(super) staged_index: usize,
    pub(super) staged_nonce: u64,
}

impl RasterTextureReservation {
    pub(super) fn matches(&self, admission: &RasterTextureAdmission) -> bool {
        self.key == admission.key
            && self.witness == admission.witness
            && self.content_identity == admission.content_identity
            && self.width == admission.width
            && self.height == admission.height
            && self.bytes == admission.bytes
            && self.staged_index == admission.staged_index
            && self.nonce == admission.nonce
    }
}

pub(super) fn claim_raster_stage_tuple(
    reservation: Option<RasterTextureReservation>,
    candidate: Option<RasterTextureWitness>,
    staged_occupied: bool,
    admission: &RasterTextureAdmission,
    expected: RasterTextureWitness,
) -> Result<RasterTextureStageClaim, &'static str> {
    if admission.witness != expected {
        return Err("raster operation authority was stale before GPU allocation");
    }
    let Some(reservation) = reservation else {
        return Err("raster texture reservation was missing before GPU allocation");
    };
    if !reservation.matches(admission) {
        return Err("raster texture reservation was stale before GPU allocation");
    }
    if candidate != Some(expected) {
        return Err("raster candidate witness changed before GPU allocation");
    }
    if staged_occupied {
        return Err("raster staged slot changed before GPU allocation");
    }
    Ok(RasterTextureStageClaim { reservation, candidate: expected, staged_index: admission.staged_index, staged_nonce: admission.nonce })
}

pub(super) struct RasterTextureReservationRetirement {
    pub(super) key: Option<RasterTextureKey>,
    pub(super) content_identity: Option<RasterContentIdentity>,
    pub(super) scene_revision: Option<u64>,
    pub(super) preview_generation: Option<u64>,
    pub(super) operation: Option<u64>,
    pub(super) width: Option<u32>,
    pub(super) height: Option<u32>,
    pub(super) bytes: Option<usize>,
    pub(super) staged_index: Option<usize>,
    pub(super) nonce: Option<u64>,
}

impl RasterTextureReservationRetirement {
    pub(super) fn new(reservation: RasterTextureReservation) -> Self {
        Self {
            key: Some(reservation.key),
            content_identity: Some(reservation.content_identity),
            scene_revision: Some(reservation.witness.scene_revision),
            preview_generation: Some(reservation.witness.preview_generation),
            operation: Some(reservation.witness.operation),
            width: Some(reservation.width),
            height: Some(reservation.height),
            bytes: Some(reservation.bytes),
            staged_index: Some(reservation.staged_index),
            nonce: Some(reservation.nonce),
        }
    }

    pub(super) fn step(&mut self) -> RasterTextureCleanupStep {
        if self.key.take().is_some() {
            return RasterTextureCleanupStep::root();
        }
        if self.content_identity.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.scene_revision.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.preview_generation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.operation.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.width.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.height.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.bytes.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_index.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.nonce.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.key.is_none()
            && self.content_identity.is_none()
            && self.scene_revision.is_none()
            && self.preview_generation.is_none()
            && self.operation.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.bytes.is_none()
            && self.staged_index.is_none()
            && self.nonce.is_none()
    }
}

impl RasterTextureAdmission {
    pub(super) fn into_retirement(self) -> RasterTextureReservationRetirement {
        RasterTextureReservationRetirement::new(RasterTextureReservation {
            key: self.key,
            witness: self.witness,
            content_identity: self.content_identity,
            width: self.width,
            height: self.height,
            bytes: self.bytes,
            staged_index: self.staged_index,
            nonce: self.nonce,
        })
    }
}

pub(super) struct RasterTextureReservationCloseCursor {
    pub(super) reservation_retirement: Option<RasterTextureReservationRetirement>,
    pub(super) admission_retirement: Option<RasterTextureReservationRetirement>,
}

impl RasterTextureReservationCloseCursor {
    pub(super) fn reservation(reservation: RasterTextureReservation) -> Self {
        Self { reservation_retirement: Some(RasterTextureReservationRetirement::new(reservation)), admission_retirement: None }
    }

    pub(super) fn cancelled(reservation: RasterTextureReservation, admission: RasterTextureAdmission) -> Self {
        Self { reservation_retirement: Some(RasterTextureReservationRetirement::new(reservation)), admission_retirement: Some(admission.into_retirement()) }
    }

    pub(super) fn rejected(admission: RasterTextureAdmission) -> Self {
        Self { reservation_retirement: None, admission_retirement: Some(admission.into_retirement()) }
    }

    pub(super) fn step(&mut self) -> RasterTextureCleanupStep {
        if let Some(retirement) = self.reservation_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster reservation must be terminal-empty");
                self.reservation_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        if let Some(retirement) = self.admission_retirement.as_mut() {
            let step = retirement.step();
            if matches!(step, RasterTextureCleanupStep::Complete) {
                assert!(retirement.terminal_is_empty(), "completed raster admission must be terminal-empty");
                self.admission_retirement = None;
                return RasterTextureCleanupStep::retained();
            }
            return step;
        }
        RasterTextureCleanupStep::Complete
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.reservation_retirement.is_none() && self.admission_retirement.is_none()
    }

    pub(super) fn retained_bytes(&self) -> usize {
        self.reservation_retirement.as_ref().and_then(|retirement| retirement.bytes).unwrap_or(0) + self.admission_retirement.as_ref().and_then(|retirement| retirement.bytes).unwrap_or(0)
    }

    pub(super) fn retained_items(&self) -> usize {
        usize::from(self.reservation_retirement.is_some()) + usize::from(self.admission_retirement.is_some())
    }
}

pub(super) struct RasterTextureStageClaimRetirement {
    pub(super) reservation: RasterTextureReservationRetirement,
    pub(super) candidate: RasterTextureWitnessSlot,
    pub(super) staged_index: Option<usize>,
    pub(super) staged_nonce: Option<u64>,
}

impl RasterTextureStageClaimRetirement {
    pub(super) fn new(claim: RasterTextureStageClaim) -> Self {
        let mut candidate = RasterTextureWitnessSlot::default();
        candidate.set(claim.candidate);
        Self { reservation: RasterTextureReservationRetirement::new(claim.reservation), candidate, staged_index: Some(claim.staged_index), staged_nonce: Some(claim.staged_nonce) }
    }

    pub(super) fn step(&mut self) -> RasterTextureCleanupStep {
        let reservation = self.reservation.step();
        if !matches!(reservation, RasterTextureCleanupStep::Complete) {
            return reservation;
        }
        if !self.candidate.is_empty() {
            self.candidate.retire_one();
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_index.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        if self.staged_nonce.take().is_some() {
            return RasterTextureCleanupStep::scalar();
        }
        RasterTextureCleanupStep::Complete
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.reservation.terminal_is_empty() && self.candidate.is_empty() && self.staged_index.is_none() && self.staged_nonce.is_none()
    }
}

#[derive(Default)]
pub(super) struct RasterTextureWitnessSlot {
    pub(super) scene_revision: Option<u64>,
    pub(super) preview_generation: Option<u64>,
    pub(super) operation: Option<u64>,
}

impl RasterTextureWitnessSlot {
    pub(super) fn get(&self) -> Option<RasterTextureWitness> {
        Some(RasterTextureWitness { scene_revision: self.scene_revision?, preview_generation: self.preview_generation?, operation: self.operation? })
    }

    pub(super) fn set(&mut self, witness: RasterTextureWitness) {
        self.scene_revision = Some(witness.scene_revision);
        self.preview_generation = Some(witness.preview_generation);
        self.operation = Some(witness.operation);
    }

    pub(super) fn retire_one(&mut self) -> bool {
        if self.scene_revision.take().is_some() {
            return false;
        }
        if self.preview_generation.take().is_some() {
            return false;
        }
        if self.operation.take().is_some() {
            return false;
        }
        true
    }

    pub(super) fn is_empty(&self) -> bool {
        self.scene_revision.is_none() && self.preview_generation.is_none() && self.operation.is_none()
    }
}

/// 🌱 What one raster operation still owes when its retirement is admitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterWitnessAdmission {
    /// 🌱 The operation admitted no raster at all. An engine surface whose first raster has not been
    /// reserved yet owes NOTHING — that is "not yet", and it must never read as "stale".
    Nothing,
    /// ♻️ The operation owns witnesses a bounded retirement has to consume.
    Retire,
}

/// 🪪️ The candidate/presentation witness pair one raster operation owns, and the bounded retirement
/// that consumes it — device-free, so the lifecycle law can drive the production authority itself.
///
/// ⚖️ Admission is a ONE-TIME fact, not a per-step invariant. The retirement retires these very slots
/// one field at a time (`RasterTextureWitnessSlot::retire_one`), so a half-consumed slot compares
/// unequal to the operation that owns it; re-reading the pair on every later step made a commit refuse
/// its own progress with `raster commit candidate witness was stale`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The law is
/// `🖱️ui/🧪️tests/🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json`.
#[derive(Default)]
pub struct RasterOperationWitnessLedger {
    pub(super) candidate: RasterTextureWitnessSlot,
    pub(super) presenting: RasterTextureWitnessSlot,
    pub(super) retiring: Option<RasterTextureWitness>,
    candidate_retired: bool,
    presenting_retired: bool,
}

impl RasterOperationWitnessLedger {
    pub fn candidate(&self) -> Option<RasterTextureWitness> {
        self.candidate.get()
    }

    pub fn presenting(&self) -> Option<RasterTextureWitness> {
        self.presenting.get()
    }

    pub fn candidate_is_empty(&self) -> bool {
        self.candidate.is_empty()
    }

    pub fn presenting_is_empty(&self) -> bool {
        self.presenting.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.candidate.is_empty() && self.presenting.is_empty() && self.retiring.is_none() && !self.candidate_retired && !self.presenting_retired
    }

    /// 🧭️ The operation whose retirement already owns these slots, if any.
    pub fn retiring(&self) -> Option<RasterTextureWitness> {
        self.retiring
    }

    pub fn candidate_occupied_by(&self, witness: RasterTextureWitness) -> bool {
        self.candidate.get().is_some_and(|current| current != witness)
    }

    /// 🎟️ Claims the candidate slot for this operation; a slot already carrying it is left untouched
    /// so every raster of one frame shares one candidate.
    pub fn admit_candidate(&mut self, witness: RasterTextureWitness) {
        if self.candidate.get().is_none() {
            self.candidate.set(witness);
        }
    }

    pub fn arm_presentation(&mut self, witness: RasterTextureWitness) -> Result<(), &'static str> {
        if self.candidate.get() != Some(witness) {
            return Err("raster candidate witness was stale before presentation");
        }
        if !self.presenting.is_empty() {
            return Err("raster presentation witness was already occupied");
        }
        self.presenting.set(witness);
        Ok(())
    }

    /// ⚖️ Reads what this operation owes. TOTAL: an operation whose own retirement already owns the
    /// slots reads `Retire` however far that retirement has consumed them — a half-empty slot is its
    /// cursor's progress, never a foreign authority's staleness.
    pub fn retirement_admission(&self, witness: RasterTextureWitness) -> Result<RasterWitnessAdmission, &'static str> {
        if let Some(retiring) = self.retiring {
            return if retiring == witness { Ok(RasterWitnessAdmission::Retire) } else { Err("raster retirement authority was occupied") };
        }
        if self.candidate.is_empty() && self.presenting.is_empty() {
            return Ok(RasterWitnessAdmission::Nothing);
        }
        if self.candidate.get() != Some(witness) {
            return Err(if self.candidate.is_empty() { "raster retirement candidate was already retired" } else { "raster retirement candidate witness was stale" });
        }
        if !self.presenting.is_empty() && self.presenting.get() != Some(witness) {
            return Err("raster retirement presentation witness was stale");
        }
        Ok(RasterWitnessAdmission::Retire)
    }

    /// 🎟️ Admits this operation's retirement and hands it the slots. Idempotent for the operation
    /// that already owns them, so a bounded step may re-enter as often as its budget requires.
    pub fn begin_retirement(&mut self, witness: RasterTextureWitness) -> Result<RasterWitnessAdmission, &'static str> {
        let admission = self.retirement_admission(witness)?;
        if admission == RasterWitnessAdmission::Retire {
            self.retiring = Some(witness);
        }
        Ok(admission)
    }

    /// ♻️ Retires ONE witness field per call; `true` once both slots are terminal and the ledger is
    /// ready for the next operation.
    pub fn retire_step(&mut self) -> bool {
        if !self.candidate_retired {
            self.candidate_retired = self.candidate.retire_one();
            return false;
        }
        if !self.presenting_retired {
            self.presenting_retired = self.presenting.retire_one();
            return false;
        }
        self.candidate_retired = false;
        self.presenting_retired = false;
        self.retiring = None;
        true
    }
}
