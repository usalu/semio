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

/// 📨️ The exact publication phase ONE pending slot is in, so "does the guest owe another turn for
/// this slot" is a property of a named state rather than a re-derived conjunction of five booleans.
///
/// 🐛️ Until 2026-09-14 the reactor read the booleans directly and the two states the alternation
/// actually visits ([`Self::Delivered`] and [`Self::Acknowledged`]) were indistinguishable from
/// [`Self::Queued`] — so a slot whose patch was already in the host's hands, and a slot the host had
/// already acknowledged, both answered `MoreWork` and each cost one host ROUND TRIP that carried
/// nothing (`📓️reactor-reconcile-spin-2026-09-14.md` §1: 46 of 115 reconcile-armed turns were this
/// two-state ping-pong with every retained-surface family empty).
///
/// The law: [`Self::Queued`], [`Self::Acknowledged`] and [`Self::Rejecting`] are the guest's own work
/// and arm the turn; [`Self::Issued`] is HOST-blocked and must not, because the answer arrives as its
/// own event-carrying turn; [`Self::Delivered`] is this very turn's output and is over by the time the
/// status is decided.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum PendingPatchPhase {
    Queued,
    Delivered,
    Issued,
    Acknowledged,
    Rejecting,
}

impl PendingPatchPhase {
    #[cfg(test)]
    pub(super) const fn guest_owes_turn(self) -> bool {
        matches!(self, Self::Queued | Self::Acknowledged | Self::Rejecting)
    }
}

impl PendingPatchSlot {
    fn phase(&self) -> PendingPatchPhase {
        if self.acknowledged {
            return PendingPatchPhase::Acknowledged;
        }
        if self.rejection_requested {
            return PendingPatchPhase::Rejecting;
        }
        if !self.emitted {
            return PendingPatchPhase::Queued;
        }
        if self.issued.is_some() {
            return PendingPatchPhase::Issued;
        }
        PendingPatchPhase::Delivered
    }
}

#[derive(Clone, Copy)]
struct ClosingPending {
    key: super::instance_lifetime::NativeCloseKey,
    active: bool,
    complete: bool,
}

/// 📤️ One slot of the OUTBOUND page a single turn may carry: the cell a publication is moved
/// through, and the pending sequence it belongs to for as long as this turn borrows it.
///
/// 🧾️ A cell is paired with a page entry by the entry's OWN `(surface, revision)`, recorded on the
/// cell the moment the patch leaves it. A cell that still holds a patch is a publication this turn did
/// NOT deliver — it keeps its sequence and travels on the next turn, and it is not part of what a
/// receipt may be staged against.
#[derive(Default)]
struct TurnPatchCell {
    patch: ui_contract::UiPendingPatch,
    sequence: Option<u64>,
    /// 🧾️ The `(surface, revision)` of the patch that LEFT this cell for the turn's page — the cell's
    /// own identity on the wire, and the only thing that pairs a page entry back to its slot.
    ///
    /// 🐛️ The pairing used to be POSITIONAL ("cells are borrowed in ascending index and a refused page
    /// is drained in publication order"), which held only while no cell survived a turn. A page cut
    /// short by its byte budget returns its last patch through [`PendingPatchAuthority::hand_back_turn`]
    /// into that cell, which then DOES survive — and from the next turn on, index order and
    /// publication order were two different orders.
    published: Option<(ui_contract::SurfaceId, u64)>,
}

pub(super) struct PendingPatchAuthority {
    slots: [Option<PendingPatchSlot>; PENDING_PATCH_CAPACITY],
    closing_instances: [Option<ClosingPending>; PENDING_PATCH_CAPACITY],
    turn_handbacks: [TurnPatchCell; semio_framework::kernel::UI_TURN_PATCHES_MAXIMUM],
    turn_handback_instance: Option<u32>,
    next_sequence: u64,
    exhausted: bool,
}

impl PendingPatchAuthority {
    pub(super) fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), closing_instances: [None; PENDING_PATCH_CAPACITY], turn_handbacks: std::array::from_fn(|_| TurnPatchCell::default()), turn_handback_instance: None, next_sequence: 0, exhausted: false }
    }

    fn instance_is_closing(&self, instance: Option<u32>) -> bool {
        self.closing_instances.iter().flatten().any(|closing| Some(closing.key.instance()) == instance)
    }

    /// 🧾️ Whether this turn's page already carries a patch for `surface`.
    ///
    /// 🐛️ Pending slots are minted per reconcile pass and per external push, never per SURFACE, so one
    /// `toolRunStart` leaves two of them for `framework.panel.toolRun` — the action's own panel
    /// reconcile (`Ready to start` → `Running`) and the run's first progress reconcile — and
    /// [`Self::take_one`] put both in ONE page, as `A` (revision n) and `A'` (base n). Those two are a
    /// CHAIN, not a batch: a host can only open `A'` after `A`'s wire, input page and receipt outbox
    /// have closed, so a page carrying both forced it to admit `A'` against `A`'s still-open cell and
    /// the intake refused with `Busy instance surface owner` — after which that surface's cell stayed
    /// busy and every later refresh, `toolRunAbort` and `toolRunFinalize` of the instance was rejected
    /// (peer ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS, `abort-finalize-midrun-*.txt`, :6013
    /// 2026-09-15 17:58). The wire contract is therefore AT MOST ONE PATCH PER SURFACE PER TURN, and
    /// the second slot is DEFERRED rather than folded: it keeps its own sequence and rides the very
    /// next turn, in publication order, exactly once.
    fn turn_page_carries(&self, surface: &ui_contract::SurfaceId) -> bool {
        self.turn_handbacks.iter().any(|cell| cell.published.as_ref().is_some_and(|(published, _)| published.0 == surface.0))
    }

    fn slot_surface(slot: &PendingPatchSlot) -> Option<&ui_contract::SurfaceId> {
        match &slot.owner {
            PendingPatchOwner::Reconcile(owner) => owner.surface(),
            PendingPatchOwner::External(pending) => pending.get().map(|patch| &patch.surface),
        }
    }

    /// 🧾️ The sequences this turn's page is authorized to stage a receipt against: a cell that is
    /// borrowed AND whose patch has LEFT it, which is exactly the set of publications the host is
    /// being handed.
    ///
    /// 🐛️ It used to be every borrowed cell, patch or no patch — and a page cut short by its byte
    /// budget or its capacity returns the patch that did not fit through
    /// [`Self::hand_back_turn`], which puts it back INTO its borrowed cell. Staging then counted a
    /// cell whose patch was never delivered, `stage_emission` answered
    /// `"pending patch emission left a borrowed publication unstaged"`, and the whole turn faulted as
    /// `plugin.reactor-close-authority`. Small surfaces never cut a page, so the generation3d door
    /// stayed green while puzzle 3d's world-3d surface trapped on its first turns (reported by
    /// INTERACTIVE-TOOLS-VISIBLE-PROCESS on :6013, 2026-09-15). The retained cell keeps its sequence,
    /// so its patch travels on the NEXT turn through the same slot, in order, exactly once.
    pub(super) fn borrowed_sequences(&self) -> impl Iterator<Item = u64> + '_ {
        self.turn_handbacks.iter().filter(|cell| cell.published.is_some()).filter_map(|cell| cell.sequence)
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

    /// 📤️ Hands THIS turn the NEXT patch of its batch, publishing it out of its slot in the same call.
    ///
    /// 🐛️ The reconcile and external arms used to `publish_into` the turn handback and answer `None`,
    /// so the patch left the guest only on the NEXT call — one whole host round trip per published
    /// surface whose only work was moving a patch from `turn_handback` into `UiTurnPatches`, measured
    /// as the `<n>:e---p / handback_sequence=Some(n)` half of the two-state ping-pong in
    /// `📓️reactor-reconcile-spin-2026-09-14.md` §1. `publish_into` is atomic (it moves the whole
    /// source or nothing), so the extraction belongs to the same call; the handback cell keeps its
    /// role for [`Self::hand_back_turn`], which is what a REFUSED output returns through.
    ///
    /// 🐛️ It also answered `"pending patch turn is already borrowed"` for the SECOND call of one
    /// turn, because the authority had exactly one handback cell — the pending half of the
    /// `UI_TURN_PATCHES_MAXIMUM = 1` floor. It now has one cell per patch the turn page can carry, and
    /// the batch is single-instance because ONE `ui-patch-receipt` authorizes all of it.
    pub(super) fn take_one(&mut self, admitted_bytes: usize) -> Result<Option<UiPatch>, &'static str> {
        if self.instance_is_closing(self.turn_handback_instance) && self.turn_handbacks.iter().any(|cell| !cell.patch.terminal_is_empty()) {
            return Ok(None);
        }
        // 🧾️ Retained cells first, OLDEST publication sequence first — a cell holding a patch is one a
        // previous turn's page could not carry, and publication order is the pending sequence, never
        // the cell index.
        let retained = self
            .turn_handbacks
            .iter()
            .enumerate()
            .filter(|(_, cell)| !cell.patch.terminal_is_empty())
            .filter(|(_, cell)| cell.patch.get().is_none_or(|patch| !self.turn_page_carries(&patch.surface)))
            .min_by_key(|(_, cell)| cell.sequence)
            .map(|(index, _)| index);
        if let Some(index) = retained {
            if let Some(patch) = self.turn_handbacks[index].patch.source_mut()?.take() {
                // 🧾️ A cell retained across turns still names its slot, and THIS turn is the one
                // publishing it — so the batch this turn stages must name that slot's instance,
                // exactly as if the patch had been taken out of the slot here.
                self.turn_handback_instance = parse_surface_instance(&patch.surface.0);
                self.turn_handbacks[index].published = Some((patch.surface.clone(), patch.revision.0));
                return Ok(Some(patch));
            }
        }
        let Some(cell) = self.turn_handbacks.iter().position(|cell| cell.sequence.is_none()) else {
            return Ok(None);
        };
        let carried = self.turn_handback_instance;
        let Some(index) = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.as_ref().is_some_and(|slot| !slot.emitted && carried.is_none_or(|instance| slot.instance == Some(instance)) && !self.instance_is_closing(slot.instance) && Self::slot_surface(slot).is_none_or(|surface| !self.turn_page_carries(surface))))
            .min_by_key(|(_, slot)| slot.as_ref().map(|slot| slot.sequence))
            .map(|(index, _)| index)
        else {
            return Ok(None);
        };
        let (slots, handbacks) = (&mut self.slots, &mut self.turn_handbacks);
        let slot = slots[index].as_mut().ok_or("pending publication source disappeared")?;
        match &mut slot.owner {
            PendingPatchOwner::Reconcile(owner) => {
                if owner.publish_into(&mut handbacks[cell].patch, &mut slot.published, admitted_bytes)? == 0 {
                    return Ok(None);
                }
            }
            PendingPatchOwner::External(patch) => {
                if admitted_bytes < size_of::<UiPatch>() {
                    return Ok(None);
                }
                *handbacks[cell].patch.source_mut()? = patch.source_mut()?.take();
            }
        }
        // 🧾️ `publish_into` and the external move are both ATOMIC — the whole source or nothing — so a
        // cell that just reported bytes and yet hands out no patch is a broken publication contract,
        // not a paged one. It is refused BY NAME instead of leaving a borrowed cell nobody can stage a
        // receipt against, which is how that shape used to present (`plugin.reactor-close-authority:
        // pending patch emission left a borrowed publication unstaged`).
        let Some(patch) = handbacks[cell].patch.source_mut()?.take() else {
            return Err("pending publication reported bytes and handed out no patch");
        };
        self.turn_handback_instance = slot.instance;
        handbacks[cell].sequence = Some(slot.sequence);
        handbacks[cell].published = Some((patch.surface.clone(), patch.revision.0));
        slot.emitted = true;
        Ok(Some(patch))
    }

    /// 📥️ Returns one REFUSED patch of this turn's batch to the cell it came out of, matched by the
    /// patch's OWN `(surface, revision)` rather than by position.
    ///
    /// 🐛️ It used to take "the first borrowed-and-empty cell", which is this patch's own only while
    /// cell index and publication order are the same order. A page cut short by its byte budget hands
    /// its last patch back into a cell that then SURVIVES the turn, and from the next turn on those two
    /// orders differ — so the positional rule would have returned a patch to a stranger's slot.
    #[expect(clippy::result_large_err, reason = "Refusal must hand back the exact retained patch owner without allocating or releasing its publication credit.")]
    pub(super) fn hand_back_turn(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
        if self.turn_handback_instance != parse_surface_instance(&patch.surface.0) {
            return Err(patch);
        }
        let Some(cell) = self.turn_handbacks.iter().position(|cell| cell.sequence.is_some() && cell.patch.terminal_is_empty() && cell.published.as_ref().is_some_and(|(surface, revision)| surface.0 == patch.surface.0 && *revision == patch.revision.0)) else {
            return Err(patch);
        };
        let sequence = self.turn_handbacks[cell].sequence;
        if let Some(slot) = self.slots.iter_mut().flatten().find(|slot| Some(slot.sequence) == sequence) {
            if slot.issued.as_ref().is_some_and(|issued| issued.committed) {
                return Err(patch);
            }
            slot.issued = None;
        } else {
            return Err(patch);
        }
        let Ok(source) = self.turn_handbacks[cell].patch.source_mut() else {
            return Err(patch);
        };
        *source = Some(patch);
        // 🧾️ The patch is back in the cell, so this turn did NOT deliver it: the cell keeps its
        // sequence (it travels on the next turn, through the same slot, exactly once) and stops being
        // something a receipt may be staged against.
        self.turn_handbacks[cell].published = None;
        Ok(())
    }

    /// 🧾️ Stages the ONE receipt that authorizes this turn's whole batch against every slot whose
    /// patch this turn actually DELIVERED, each page entry matched to its cell by its own
    /// `(surface, revision)`.
    ///
    /// 🐛️ The pairing used to be positional — the `i`-th page entry against the `i`-th borrowed cell —
    /// and the count it checked was every borrowed cell. A page cut short by its byte budget or its
    /// capacity returns the patch that did not fit through [`Self::hand_back_turn`], which puts it back
    /// INTO its borrowed cell, so the count saw a publication the host was never handed and the whole
    /// turn faulted as `plugin.reactor-close-authority: pending patch emission left a borrowed
    /// publication unstaged`. Small surfaces never cut a page, which is why the generation3d door
    /// stayed green while puzzle 3d's world-3d surface trapped on its first turns (reported by
    /// INTERACTIVE-TOOLS-VISIBLE-PROCESS on :6013, 2026-09-15).
    pub(super) fn stage_emission<'a>(&mut self, receipt: ActorUiPatchReceipt, patches: impl Iterator<Item = &'a UiPatch>) -> Result<(), &'static str> {
        if !receipt.is_valid() || Some(receipt.lifetime.instance_id) != self.turn_handback_instance {
            return Err("patch receipt names another lifetime");
        }
        let mut staged = 0usize;
        for patch in patches {
            if parse_surface_instance(&patch.surface.0) != self.turn_handback_instance {
                return Err("patch receipt names another lifetime");
            }
            let sequence = self
                .turn_handbacks
                .iter()
                .find(|cell| cell.published.as_ref().is_some_and(|(surface, revision)| surface.0 == patch.surface.0 && *revision == patch.revision.0))
                .and_then(|cell| cell.sequence)
                .ok_or("pending patch emission source absent")?;
            let slot = self.slots.iter_mut().flatten().find(|slot| slot.sequence == sequence).ok_or("pending patch emission source absent")?;
            if slot.issued.is_some() {
                return Err("pending patch emission already staged");
            }
            slot.issued = Some(IssuedPatchAck { receipt, surface: patch.surface.clone(), revision: patch.revision.0, committed: false });
            staged += 1;
        }
        if staged != self.borrowed_sequences().count() {
            return Err("pending patch emission left a borrowed publication unstaged");
        }
        Ok(())
    }

    pub(super) fn commit_emission(&mut self) {
        let sequences: Vec<u64> = self.borrowed_sequences().collect();
        assert!(!sequences.is_empty(), "prepared patch emission sequence");
        for sequence in sequences {
            let slot = self.slots.iter_mut().flatten().find(|slot| slot.sequence == sequence).expect("prepared patch emission owner");
            let issued = slot.issued.as_mut().expect("prepared patch receipt");
            assert!(!issued.committed);
            issued.committed = true;
        }
        // 🧾️ Only the cells this turn DELIVERED are released. A cell still holding a patch a cut page
        // could not carry keeps its sequence and its slot, and travels on the next turn.
        for cell in &mut self.turn_handbacks {
            if cell.published.is_some() {
                cell.sequence = None;
                cell.published = None;
            }
        }
        assert!(self.turn_handbacks.iter().all(|cell| cell.published.is_none()), "a committed emission releases every delivered cell");
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
            if let Some(cell) = self.turn_handbacks.iter().position(|cell| !cell.patch.terminal_is_empty()) {
                let mut step = self.turn_handbacks[cell].patch.close_step(maximum_items, maximum_bytes)?;
                if self.turn_handbacks[cell].patch.terminal_is_empty() {
                    self.turn_handbacks[cell] = TurnPatchCell::default();
                }
                step.complete = false;
                return Ok(step);
            }
            self.turn_handback_instance = None;
            for cell in &mut self.turn_handbacks {
                cell.sequence = None;
                cell.published = None;
            }
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
    /// [`Self::has_retiring`] TRUE, which keeps the reactor turn in `MoreWork`, which costs the
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
            .map(|slot| format!("{}:{:?}:{}{}{}{}{}", slot.sequence, slot.phase(), if slot.emitted { "e" } else { "-" }, if slot.acknowledged { "a" } else { "-" }, match &slot.issued { Some(issued) if issued.committed => "c", Some(_) => "i", None => "-" }, if slot.rejection_requested { "r" } else { "-" }, if slot.published.is_some() { "p" } else { "-" }))
            .collect();
        format!(
            "slots=[{}] handback_empty={} handback_sequences={:?} exhausted={} closing={}",
            slots.join(","),
            self.turn_handbacks.iter().all(|cell| cell.patch.terminal_is_empty()),
            self.borrowed_sequences().collect::<Vec<_>>(),
            self.exhausted,
            self.closing_instances.iter().flatten().count()
        )
    }

    /// 📬️ Whether a patch is queued for a FUTURE turn to hand out. A [`PendingPatchPhase::Issued`]
    /// slot is deliberately NOT one: it is waiting on the host, whose answer arrives as its own
    /// event-carrying turn, so counting it here only buys a round trip that carries nothing.
    pub(super) fn has_undelivered(&self) -> bool {
        self.turn_handbacks.iter().any(|cell| !cell.patch.terminal_is_empty()) || self.slots.iter().flatten().any(|slot| slot.phase() == PendingPatchPhase::Queued)
    }

    /// 🧹️ Whether a slot or a closing instance still owes RETIREMENT. The turn drives this itself
    /// (`PATCH_CLOSE_UNITS_PER_TURN` units, twice per turn since 2026-09-14 — once before the events
    /// and once after the acknowledgements they carry), so it is true here only when a retirement run
    /// was cut short by the turn's own wall deadline.
    pub(super) fn has_retiring(&self) -> bool {
        self.slots.iter().flatten().any(|slot| matches!(slot.phase(), PendingPatchPhase::Acknowledged | PendingPatchPhase::Rejecting)) || self.closing_instances.iter().any(Option::is_some)
    }

    /// 📨️ The union of the two, stated independently through [`PendingPatchPhase::guest_owes_turn`]
    /// so the partition itself is assertable: every publication state whose progress is the GUEST's
    /// own is exactly [`Self::has_undelivered`] ∪ [`Self::has_retiring`], and nothing else arms.
    #[cfg(test)]
    pub(super) fn has_unpublished(&self) -> bool {
        self.turn_handbacks.iter().any(|cell| !cell.patch.terminal_is_empty()) || self.slots.iter().flatten().any(|slot| slot.phase().guest_owes_turn()) || self.closing_instances.iter().any(Option::is_some)
    }

    /// 🔍️ The patch parked in ONE borrowed turn cell, for the laws that weigh a publication's own
    /// retirement without reaching into the authority's storage.
    #[cfg(test)]
    pub(super) fn borrowed_patch(&self, cell: usize) -> Option<&UiPatch> {
        self.turn_handbacks.get(cell).and_then(|cell| cell.patch.get())
    }

    #[cfg(test)]
    pub(super) fn phases(&self) -> Vec<PendingPatchPhase> {
        self.slots.iter().flatten().map(PendingPatchSlot::phase).collect()
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
