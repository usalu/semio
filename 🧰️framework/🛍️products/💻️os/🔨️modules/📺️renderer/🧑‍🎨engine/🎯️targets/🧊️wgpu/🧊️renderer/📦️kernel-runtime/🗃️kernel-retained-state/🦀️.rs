//! 🗃️ Kernel retained state: retained document exchange and surfaces, pending rejections, mounted replay routes, command document retirement and the pool state head.
use semio_framework_ui_contract_validation::{UiDocumentLimits,UiPatchApplyOutcome,UiPatchApplyProducer,UiPatchApplyRejected,UiPatchApplyStep};
use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️KernelPoolState
//#region 📄️RetainedDocumentExchange
enum RetainedPatchApply {
    Applying(UiPatchApplyProducer),
    Ready(UiPatchApplyOutcome),
    Rejected(UiPatchApplyRejected),
}

struct RetainedDocumentBuild {
    revision: UiRevision,
    cursor: usize,
    builder: Option<UiDocumentBuilder>,
    rejected_record: Option<ui_contract::UiNodeRecord>,
    pub closing: bool,
}
//#endregion 📄️RetainedDocumentExchange

pub struct RetainedSurface {
    pub state: Option<UiSnapshotState>,
    pub patch: Option<RetainedPatchApply>,
    pub queued_patches: UiFixedList<KernelUiPatch, UI_DOCUMENT_LEASE_SLOTS>,
    pub rejected_patches: UiFixedList<KernelUiPatch, UI_DOCUMENT_LEASE_SLOTS>,
    pub build: Option<RetainedDocumentBuild>,
    pub build_pending: bool,
    published: Option<UiDocumentLease>,
    pub closing: Option<UiDocumentLease>,
    pub exchange_closing: Option<UiDocumentLease>,
    patch_close_complete: bool,
    /// 🎫️ The authority the guest issued with the patch this surface is currently applying —
    /// `TurnResult::ui_patch_receipt`, which `ActorUiPatchReceipt::validate_pairing` pins 1:1 to
    /// the turn's single patch. A rejection has to echo it back on `Event::PatchRejected`.
    pub patch_receipt: Option<semio_framework::kernel::ActorUiPatchReceipt>,
}

impl RetainedSurface {
    pub fn new(surface: SurfaceId) -> Self {
        Self {
            state: Some(UiSnapshotState::new(surface)),
            patch: None,
            queued_patches: UiFixedList::default(),
            rejected_patches: UiFixedList::default(),
            build: None,
            build_pending: false,
            published: None,
            closing: None,
            exchange_closing: None,
            patch_close_complete: false,
            patch_receipt: None,
        }
    }

    pub fn admit_patch(&mut self, patch: KernelUiPatch) -> Result<(), KernelUiPatch> {
        if self.patch.is_some() || self.state.is_none() {
            return self.queued_patches.try_push(patch);
        }
        let reservation = match reserve_seq() {
            Ok(reservation) => reservation,
            Err(_) => return Err(patch),
        };
        let state = self.state.take().expect("retained surface state is admitted once");
        let generation = reservation.generation();
        let owner = match UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), generation) {
            Ok(producer) => RetainedPatchApply::Applying(producer),
            Err(rejected) => RetainedPatchApply::Rejected(rejected),
        };
        let committed = reservation.commit().is_ok();
        self.patch = Some(owner);
        if !committed {
            return Ok(());
        }
        Ok(())
    }

    pub fn advance_patch_one(&mut self) -> Option<(UiRevision, ui_contract::UiText)> {
        if let Some(patch) = self.rejected_patches.get_mut(0) {
            if patch.ops.pop().is_some() {
                return None;
            }
            let _ = self.rejected_patches.swap_remove(0);
            return Some((UiRevision::default(), ui_contract::UiText::try_from_str("retained patch admission capacity exhausted").unwrap_or_default()));
        }
        if self.patch.is_none() {
            if let Some(patch) = self.queued_patches.swap_remove(0) {
                if let Err(patch) = self.admit_patch(patch) {
                    let _ = self.queued_patches.try_push(patch);
                }
            }
        }
        let current = self.patch.take()?;
        match current {
            RetainedPatchApply::Applying(mut producer) => match producer.drive_one(producer.generation(), false, false) {
                UiPatchApplyStep::MoreWork => self.patch = Some(RetainedPatchApply::Applying(producer)),
                UiPatchApplyStep::Ready => match producer.take_ready() {
                    Ok(outcome) => self.patch = Some(RetainedPatchApply::Ready(outcome)),
                    Err(producer) => self.patch = Some(RetainedPatchApply::Applying(producer)),
                },
                UiPatchApplyStep::Rejected => match producer.take_rejected() {
                    Ok(rejected) => self.patch = Some(RetainedPatchApply::Rejected(rejected)),
                    Err(producer) => self.patch = Some(RetainedPatchApply::Applying(producer)),
                },
            },
            RetainedPatchApply::Ready(mut outcome) => {
                if !outcome.close_step() {
                    self.patch = Some(RetainedPatchApply::Ready(outcome));
                } else {
                    match outcome.take_state() {
                        Ok(state) => {
                            self.state = Some(state);
                            self.build_pending = true;
                        }
                        Err(outcome) => self.patch = Some(RetainedPatchApply::Ready(outcome)),
                    }
                }
            }
            RetainedPatchApply::Rejected(mut rejected) => {
                let revision = rejected.state().map_or(UiRevision::default(), UiSnapshotState::revision);
                let reason = ui_contract::UiText::try_from_str("retained patch rejected").unwrap_or_default();
                if !rejected.close_step() {
                    self.patch = Some(RetainedPatchApply::Rejected(rejected));
                } else {
                    match rejected.take_state() {
                        Ok(state) => self.state = Some(state),
                        Err(rejected) => self.patch = Some(RetainedPatchApply::Rejected(rejected)),
                    }
                }
                return Some((revision, reason));
            }
        }
        None
    }

    pub fn begin_document_build(&mut self) {
        if self.build.is_some() || !self.build_pending {
            return;
        }
        let Some(state) = self.state.as_ref() else { return };
        let Ok(reservation) = reserve_seq() else { return };
        let generation = reservation.generation();
        let builder = match UiDocumentBuilder::try_new(generation, state.surface.clone(), state.revision, state.root, state.revision.0) {
            Ok(builder) => builder,
            Err((_fault, _surface)) => {
                let _ = ui_contract::close_ui_document_page_one();
                return;
            }
        };
        let mut build = RetainedDocumentBuild { revision: state.revision, cursor: 0, builder: Some(builder), rejected_record: None, closing: false };
        if reservation.commit().is_err() {
            build.closing = true;
            self.build = Some(build);
            return;
        }
        self.build_pending = false;
        self.build = Some(build);
    }

    pub fn advance_document_one(&mut self) {
        if let Some(closing) = self.exchange_closing.as_mut() {
            if closing.close_step() && closing.terminal_is_empty() {
                self.exchange_closing = None;
            }
            return;
        }
        if let Some(closing) = self.closing.as_mut() {
            if closing.close_step() && closing.terminal_is_empty() {
                self.closing = None;
            }
            return;
        }
        self.begin_document_build();
        let Some(build) = self.build.as_mut() else { return };
        if build.closing {
            let Some(builder) = build.builder.as_mut() else { return };
            if builder.close_step() && builder.terminal_is_empty() {
                build.builder = None;
                if build.rejected_record.take().is_some() {
                    return;
                }
                self.build = None;
            }
            return;
        }
        let Some(state) = self.state.as_ref() else { return };
        if state.revision != build.revision {
            build.closing = true;
            return;
        }
        if let Some(record) = state.nodes.get_index(build.cursor) {
            let Some(record) = record.credited_clone() else {
                build.closing = true;
                return;
            };
            let Some(builder) = build.builder.as_mut() else { return };
            if let Err((_fault, record)) = builder.try_push(record) {
                build.rejected_record = Some(record);
                build.closing = true;
                return;
            }
            let Some(cursor) = build.cursor.checked_add(1) else {
                build.closing = true;
                return;
            };
            build.cursor = cursor;
            return;
        }
        let Some(builder) = build.builder.take() else { return };
        match builder.finish() {
            Ok(document) => {
                self.closing = self.published.replace(document);
                self.build = None;
            }
            Err((_fault, builder)) => {
                build.builder = Some(builder);
                build.closing = true;
            }
        }
    }

    pub fn try_alias(&self) -> Option<UiDocumentLease> {
        self.published.as_ref()?.try_alias().ok()
    }

    /// 🔎️ Whether a document is currently published on this surface, so a pool can tell a surface that never rendered from one that did.
    pub fn has_published(&self) -> bool {
        self.published.is_some()
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(current) = self.patch.take() {
            match current {
                RetainedPatchApply::Applying(mut producer) => {
                    let _ = producer.drive_one(producer.generation(), true, false);
                    self.patch = Some(match producer.take_rejected() {
                        Ok(rejected) => RetainedPatchApply::Rejected(rejected),
                        Err(producer) => RetainedPatchApply::Applying(producer),
                    });
                }
                RetainedPatchApply::Ready(mut outcome) => {
                    if outcome.close_step() {
                        match outcome.take_state() {
                            Ok(state) => self.state = Some(state),
                            Err(outcome) => self.patch = Some(RetainedPatchApply::Ready(outcome)),
                        }
                    } else {
                        self.patch = Some(RetainedPatchApply::Ready(outcome));
                    }
                }
                RetainedPatchApply::Rejected(mut rejected) => {
                    if rejected.close_step() {
                        match rejected.take_state() {
                            Ok(state) => self.state = Some(state),
                            Err(rejected) => self.patch = Some(RetainedPatchApply::Rejected(rejected)),
                        }
                    } else {
                        self.patch = Some(RetainedPatchApply::Rejected(rejected));
                    }
                }
            }
            return false;
        }
        if let Some(patch) = self.queued_patches.swap_remove(0) {
            if let Err(patch) = self.admit_patch(patch) {
                let _ = self.queued_patches.try_push(patch);
            }
            return false;
        }
        if let Some(patch) = self.rejected_patches.get_mut(0) {
            if patch.ops.pop().is_none() {
                let _ = self.rejected_patches.swap_remove(0);
            }
            return false;
        }
        if let Some(build) = self.build.as_mut() {
            build.closing = true;
            if let Some(builder) = build.builder.as_mut() {
                if builder.close_step() && builder.terminal_is_empty() {
                    build.builder = None;
                    if build.rejected_record.take().is_some() {
                        return false;
                    }
                    self.build = None;
                }
            }
            return false;
        }
        if self.build_pending {
            self.build_pending = false;
            return false;
        }
        if let Some(document) = self.published.as_mut() {
            if document.close_step() && document.terminal_is_empty() {
                self.published = None;
            }
            return false;
        }
        if let Some(document) = self.closing.as_mut() {
            if document.close_step() && document.terminal_is_empty() {
                self.closing = None;
            }
            return false;
        }
        if let Some(document) = self.exchange_closing.as_mut() {
            if document.close_step() && document.terminal_is_empty() {
                self.exchange_closing = None;
            }
            return false;
        }
        if let Some(state) = self.state.as_mut() {
            let next_node = state.nodes.keys().next().copied();
            if let Some(id) = next_node {
                drop(state.nodes.remove(&id));
                return false;
            }
            self.state = None;
            return false;
        }
        if !self.patch_close_complete {
            self.patch_close_complete = semio_framework_ui_contract_validation::close_ui_patch_owner_one();
            return false;
        }
        ui_contract::close_ui_document_page_one()
    }
}

pub const RETAINED_SURFACE_CAPACITY: usize = 64;

pub struct RetainedSurfaceSlot {
    pub generation: u64,
    pub instance: u32,
    pub surface: SurfaceId,
    pub owner: RetainedSurface,
}

pub struct RetainedSurfaceRegistry {
    pub slots: Box<[Option<RetainedSurfaceSlot>; RETAINED_SURFACE_CAPACITY]>,
    next_generation: u64,
    generation_exhausted: bool,
    close_cursor: usize,
}

impl RetainedSurfaceRegistry {
    pub fn new() -> Self {
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| None), next_generation: 0, generation_exhausted: false, close_cursor: 0 }
    }

    pub fn index(&self, instance: u32, surface: &SurfaceId) -> Option<usize> {
        self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == instance && slot.surface == *surface))
    }

    pub fn get(&self, instance: u32, surface: &SurfaceId) -> Option<&RetainedSurfaceSlot> {
        self.index(instance, surface).and_then(|index| self.slots[index].as_ref())
    }

    fn get_mut(&mut self, instance: u32, surface: &SurfaceId) -> Option<&mut RetainedSurfaceSlot> {
        let index = self.index(instance, surface)?;
        self.slots[index].as_mut()
    }

    pub fn try_admit(&mut self, instance: u32, surface: SurfaceId) -> Result<usize, SurfaceId> {
        if let Some(index) = self.index(instance, &surface) {
            return Ok(index);
        }
        if self.generation_exhausted {
            return Err(surface);
        }
        let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(surface) };
        let Some(generation) = self.next_generation.checked_add(1).filter(|generation| *generation != 0) else { return Err(surface) };
        let owner = RetainedSurface::new(surface.clone());
        self.slots[index] = Some(RetainedSurfaceSlot { generation, instance, surface, owner });
        self.next_generation = generation;
        self.generation_exhausted = generation == u64::MAX;
        Ok(index)
    }

    pub fn advance_instance_close_one(&mut self, instance: u32) -> bool {
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % RETAINED_SURFACE_CAPACITY;
        let Some(slot) = self.slots[index].as_mut().filter(|slot| slot.instance == instance) else {
            return !self.slots.iter().flatten().any(|slot| slot.instance == instance);
        };
        if slot.owner.close_step() {
            self.slots[index] = None;
        }
        false
    }

    pub fn advance_realm_close_one(&mut self) -> bool {
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % RETAINED_SURFACE_CAPACITY;
        let Some(slot) = self.slots[index].as_mut() else { return self.slots.iter().all(Option::is_none) };
        if slot.owner.close_step() {
            self.slots[index] = None;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }
}

pub struct PendingSurfaceRejection {
    pub generation: u64,
    pub instance: u32,
    pub surface: SurfaceId,
    pub revision: UiRevision,
    pub reason: ui_contract::UiText,
    pub patch: Option<KernelUiPatch>,
    /// 🎫️ The issued-patch authority the guest must see back on `Event::PatchRejected`; `None`
    /// when the rejection has no admitted turn behind it (a surface that never reached a slot).
    pub receipt: Option<semio_framework::kernel::ActorUiPatchReceipt>,
}

pub struct PendingSurfaceRejectionRegistry {
    pub slots: Box<[Option<PendingSurfaceRejection>; RETAINED_SURFACE_CAPACITY]>,
    close_cursor: usize,
}

impl PendingSurfaceRejectionRegistry {
    pub fn new() -> Self {
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| None), close_cursor: 0 }
    }

    pub fn index(&self, instance: u32, surface: &SurfaceId) -> Option<usize> {
        self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == instance && slot.surface == *surface))
    }

    pub fn get(&self, instance: u32, surface: &SurfaceId) -> Option<&PendingSurfaceRejection> {
        self.index(instance, surface).and_then(|index| self.slots[index].as_ref())
    }

    pub fn remove(&mut self, instance: u32, surface: &SurfaceId) -> Option<PendingSurfaceRejection> {
        let index = self.index(instance, surface)?;
        self.slots[index].take()
    }

    pub fn try_admit(&mut self, rejection: PendingSurfaceRejection) -> Result<(), PendingSurfaceRejection> {
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else { return Err(rejection) };
        *slot = Some(rejection);
        Ok(())
    }

    pub fn try_upsert(&mut self, rejection: PendingSurfaceRejection) -> Result<(), PendingSurfaceRejection> {
        if let Some(index) = self.index(rejection.instance, &rejection.surface) {
            if self.slots[index].as_ref().is_some_and(|current| current.patch.is_some()) {
                return Err(rejection);
            }
            self.slots[index] = Some(rejection);
            return Ok(());
        }
        self.try_admit(rejection)
    }

    pub fn take_instance_one(&mut self, instance: u32) -> Option<PendingSurfaceRejection> {
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % RETAINED_SURFACE_CAPACITY;
        let slot = self.slots[index].as_mut().filter(|slot| slot.instance == instance)?;
        if let Some(patch) = slot.patch.as_mut() {
            if patch.ops.pop().is_none() {
                slot.patch = None;
            }
            return None;
        }
        self.slots[index].take()
    }

    pub fn take_any_one(&mut self) -> Option<PendingSurfaceRejection> {
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % RETAINED_SURFACE_CAPACITY;
        if let Some(patch) = self.slots[index].as_mut().and_then(|slot| slot.patch.as_mut()) {
            if patch.ops.pop().is_none() {
                if let Some(slot) = self.slots[index].as_mut() {
                    slot.patch = None;
                }
            }
            return None;
        }
        self.slots[index].take()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }

    pub fn instance_is_empty(&self, instance: u32) -> bool {
        !self.slots.iter().flatten().any(|slot| slot.instance == instance)
    }
}

pub struct PendingJobProgressPresentation {
    pub token: JobProgressPresentationToken,
    pub receipt: JobProgressReceipt,
}

#[derive(Clone, Copy)]
pub struct MountedReplayRouteSeed {
    pub actor: ActorId,
    pub plugin: [u8; 32],
    pub package: [u8; 32],
    pub window: u64,
    pub artifact: [u8; 32],
}

impl MountedReplayRouteSeed {
    pub fn compose(self, request: JobReplayRequest) -> JobReplayRoute {
        JobReplayRoute {
            plugin: self.plugin,
            package: self.package,
            controller: request.controller,
            tool: request.tool,
            window: self.window,
            artifact: self.artifact,
            request_schema: request.schema,
            request_version: request.version,
            request_digest: request.digest,
        }
    }
}

pub struct MountedJobReplay {
    pub actor: ActorId,
    pub authority: JobTurn,
    pub request: JobReplayRequest,
    pub placement: JobPlacement,
    pub log: ManuallyDrop<JobReplayLog>,
    pub captured: Option<JobPublication>,
    pub policy: Option<JobReplayPublicationPolicy>,
    pub terminal_seen: bool,
    pub replay_requested: bool,
    pub replay_started: bool,
    pub replay_submit_sequence: Option<u64>,
    pub accepted_replay_sequence: Option<u64>,
    pub replay_worker_count: u16,
    pub replay_worker_slot: u16,
    pub recovery: Option<MountedReplayRecoveryToken>,
}

impl MountedJobReplay {
    pub fn pending_replay_start_is_exact(&self, worker_count: u16, worker_slot: u16) -> bool {
        self.replay_requested && !self.replay_started && self.replay_worker_count == worker_count && self.replay_worker_slot == worker_slot
    }

    pub fn replay_submission_sequence(&mut self) -> Result<u64, String> {
        match self.replay_submit_sequence {
            Some(sequence) => Ok(sequence),
            None => {
                let sequence = next_seq()?;
                self.replay_submit_sequence = Some(sequence);
                Ok(sequence)
            }
        }
    }

    pub fn accept_replay_submission(&mut self, sequence: u64) -> Result<(), String> {
        if self.replay_started || self.replay_submit_sequence != Some(sequence) {
            return Err("kernel: accepted replay start does not match the retained submission identity".to_string());
        }
        self.replay_started = true;
        self.replay_submit_sequence = None;
        self.accepted_replay_sequence = Some(sequence);
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct MountedReplayRecoveryToken {
    pub index: usize,
    pub epoch: u64,
}

struct MountedReplayRecoveryOwner {
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub generation: u64,
    #[cfg(not(any(test, feature = "testkit")))]
    generation: u64,
    pub actor: ActorId,
    pub job: u64,
    pub log: Option<JobReplayLog>,
    pub captured: Option<JobPublication>,
    publication_phase: u8,
}

struct MountedReplayRecoverySlot {
    pub epoch: u64,
    pub reserved: bool,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub owner: Option<MountedReplayRecoveryOwner>,
    #[cfg(not(any(test, feature = "testkit")))]
    owner: Option<MountedReplayRecoveryOwner>,
}

pub struct MountedReplayRecoveryRegistry {
    pub slots: Box<[MountedReplayRecoverySlot; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    abandoned: Box<[Option<MountedReplayRecoveryOwner>; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    cursor: usize,
    abandoned_cursor: usize,
}

impl MountedReplayRecoveryRegistry {
    pub fn new() -> Self {
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| MountedReplayRecoverySlot { epoch: 0, reserved: false, owner: None }), abandoned: semio_framework_async_slots::boxed_fixed_slots(|| None), cursor: 0, abandoned_cursor: 0 }
    }

    pub fn reserve(&mut self) -> Option<MountedReplayRecoveryToken> {
        let index = self.slots.iter().position(|slot| !slot.reserved && slot.owner.is_none())?;
        let epoch = self.slots[index].epoch.checked_add(1)?;
        self.slots[index].epoch = epoch;
        self.slots[index].reserved = true;
        Some(MountedReplayRecoveryToken { index, epoch })
    }

    pub fn release(&mut self, token: MountedReplayRecoveryToken) -> bool {
        let Some(slot) = self.slots.get_mut(token.index) else { return false };
        if slot.epoch != token.epoch || !slot.reserved || slot.owner.is_some() {
            return false;
        }
        slot.reserved = false;
        true
    }

    pub fn publish(&mut self, token: MountedReplayRecoveryToken, owner: MountedReplayRecoveryOwner) {
        if let Some(slot) = self.slots.get_mut(token.index) {
            if slot.epoch == token.epoch && slot.reserved && slot.owner.is_none() {
                slot.owner = Some(owner);
                return;
            }
        }
        let slot = self.abandoned.iter_mut().find(|slot| slot.is_none()).expect("pre-reserved recovery makes at most one discoverable owner reachable per mounted shell");
        *slot = Some(owner);
    }

    pub fn close_one(&mut self) -> bool {
        if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.is_some()) {
            self.cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
            if mounted_replay_recovery_owner_close_one(self.slots[index].owner.as_mut().expect("published replay recovery owner")) {
                let terminal = self.slots[index].owner.take().expect("terminal replay recovery owner");
                debug_assert!(terminal.log.is_none() && terminal.captured.is_none());
                self.slots[index].reserved = false;
            }
            return true;
        }
        let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].is_some()) else {
            return false;
        };
        self.abandoned_cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
        if mounted_replay_recovery_owner_close_one(self.abandoned[index].as_mut().expect("discoverable abandoned replay owner")) {
            let terminal = self.abandoned[index].take().expect("terminal abandoned replay owner");
            debug_assert!(terminal.log.is_none() && terminal.captured.is_none());
        }
        true
    }

    pub fn has_close_work(&self) -> bool {
        self.slots.iter().any(|slot| slot.owner.is_some()) || self.abandoned.iter().any(Option::is_some)
    }
}

fn mounted_replay_recovery_owner_close_one(owner: &mut MountedReplayRecoveryOwner) -> bool {
    if let Some(publication) = owner.captured.as_mut() {
        let released = match (&mut publication.outcome, owner.publication_phase) {
            (semio_framework_actor::JobStepOutcome::PreviewReady { preview }, 0) => std::mem::take(preview).capacity(),
            (semio_framework_actor::JobStepOutcome::CheckpointReady { checkpoint }, 0) => std::mem::take(&mut checkpoint.state).capacity(),
            (semio_framework_actor::JobStepOutcome::Complete { candidate }, 0) => std::mem::take(&mut candidate.state).capacity(),
            (semio_framework_actor::JobStepOutcome::Complete { candidate }, 1) => std::mem::take(&mut candidate.output).capacity(),
            (semio_framework_actor::JobStepOutcome::Fault { detail }, 0) => std::mem::take(detail).capacity(),
            _ => 0,
        };
        let phases = match &publication.outcome {
            semio_framework_actor::JobStepOutcome::Complete { .. } => 2,
            semio_framework_actor::JobStepOutcome::PreviewReady { .. } | semio_framework_actor::JobStepOutcome::CheckpointReady { .. } | semio_framework_actor::JobStepOutcome::Fault { .. } => 1,
            semio_framework_actor::JobStepOutcome::Yield | semio_framework_actor::JobStepOutcome::Cancelled => 0,
        };
        owner.publication_phase += 1;
        if owner.publication_phase >= phases {
            owner.captured = None;
        }
        let _ = released;
        return false;
    }
    if let Some(log) = owner.log.as_mut() {
        log.begin_close();
        let now = semio_framework_job::default_now_us();
        let mut preview_sequence = owner.job;
        let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(owner.actor.0),
            semio_framework_job::Generation(owner.generation),
            now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US, JOB_REPLAY_STEP_GRANT)).unwrap_or(semio_framework_job::StepBudget::new(0, 0, JOB_REPLAY_STEP_GRANT)),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
            &mut retained_progress,
        );
        let _ = log.close_step(&mut context);
        if log.terminal_is_empty() {
            owner.log = None;
        }
        return false;
    }
    true
}

pub fn mounted_replay_recovery_registry() -> &'static Mutex<MountedReplayRecoveryRegistry> {
    static REGISTRY: OnceLock<Mutex<MountedReplayRecoveryRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(MountedReplayRecoveryRegistry::new()))
}

impl Drop for MountedJobReplay {
    fn drop(&mut self) {
        let Some(token) = self.recovery.take() else { return };
        if self.log.terminal_is_empty() && self.captured.is_none() && self.policy.is_none() {
            unsafe { ManuallyDrop::drop(&mut self.log) };
            assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").release(token));
            return;
        }
        let owner =
            MountedReplayRecoveryOwner { generation: self.authority.operation.generation, actor: self.actor, job: self.authority.job, log: Some(unsafe { ManuallyDrop::take(&mut self.log) }), captured: self.captured.take(), publication_phase: 0 };
        mounted_replay_recovery_registry().lock().expect("replay recovery lock").publish(token, owner);
    }
}

pub struct ClosingKernelApp {
    pub instance: u32,
    pub actors: [ActorId; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub actor_count: usize,
    pub replay_begin_cursor: usize,
    pub begin_cursor: usize,
    pub unregister_cursor: usize,
    pub unregister_refusal: Option<semio_framework_actor::pack::PackError>,
    pub route_cursor: usize,
}

impl ClosingKernelApp {
    pub fn contains(&self, actor: ActorId) -> bool {
        self.actors[..self.actor_count].contains(&actor)
    }
}

//#region 📦️CommandDocumentRetirement
pub const COMMAND_DOCUMENT_RETIREMENT_CAPACITY: usize = UI_DOCUMENT_LEASE_SLOTS * ui_contract::UI_DOCUMENT_LEASE_ALIASES as usize;

#[derive(Clone, Copy)]
struct CommandDocumentReservationToken {
    index: usize,
    pub epoch: u64,
    batch: u64,
    generation: u64,
}

pub struct CommandDocumentPageReservation {
    tokens: UiFixedList<CommandDocumentReservationToken, UI_DOCUMENT_LEASE_SLOTS>,
}

pub enum CommandDocumentRetirementState {
    Reserved { epoch: u64, batch: u64, generation: u64 },
    Pending { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
    Closing { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
}

pub struct CommandDocumentRetirementRegistry {
    pub slots: Box<[Option<CommandDocumentRetirementState>; COMMAND_DOCUMENT_RETIREMENT_CAPACITY]>,
    epochs: [u64; COMMAND_DOCUMENT_RETIREMENT_CAPACITY],
    pub cursor: usize,
}

impl CommandDocumentRetirementRegistry {
    pub fn new() -> Self {
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| None), epochs: [0; COMMAND_DOCUMENT_RETIREMENT_CAPACITY], cursor: 0 }
    }

    pub fn try_reserve_page(&mut self, batch: u64, generation: u64) -> Result<CommandDocumentPageReservation, ()> {
        let mut candidates = UiFixedList::<CommandDocumentReservationToken, UI_DOCUMENT_LEASE_SLOTS>::default();
        for index in 0..COMMAND_DOCUMENT_RETIREMENT_CAPACITY {
            if self.slots[index].is_some() {
                continue;
            }
            let Some(epoch) = self.epochs[index].checked_add(1) else { continue };
            candidates.try_push(CommandDocumentReservationToken { index, epoch, batch, generation }).map_err(|_| ())?;
            if candidates.len() == UI_DOCUMENT_LEASE_SLOTS {
                break;
            }
        }
        if candidates.len() != UI_DOCUMENT_LEASE_SLOTS {
            return Err(());
        }
        for token in &candidates {
            self.epochs[token.index] = token.epoch;
            self.slots[token.index] = Some(CommandDocumentRetirementState::Reserved { epoch: token.epoch, batch, generation });
        }
        Ok(CommandDocumentPageReservation { tokens: candidates })
    }

    pub fn admit(&mut self, reservation: &mut CommandDocumentPageReservation, owner: ExchangeSurfaceDocument) {
        let token = reservation.tokens.pop().expect("one fixed command-document destination is reserved for every fixed turn-result surface");
        self.slots[token.index] = Some(CommandDocumentRetirementState::Pending { epoch: token.epoch, batch: token.batch, generation: token.generation, surface: owner.surface, document: owner.document });
    }

    pub fn release(&mut self, reservation: &mut CommandDocumentPageReservation) {
        while let Some(token) = reservation.tokens.pop() {
            let matches =
                self.slots[token.index].as_ref().is_some_and(|state| matches!(state, CommandDocumentRetirementState::Reserved { epoch, batch, generation } if *epoch == token.epoch && *batch == token.batch && *generation == token.generation));
            if matches {
                self.slots[token.index] = None;
            }
        }
    }

    pub fn begin_close_batch(&mut self, batch: u64, generation: u64) {
        for slot in &mut self.slots {
            let Some(state) = slot.take() else { continue };
            *slot = match state {
                CommandDocumentRetirementState::Reserved { batch: owner_batch, generation: owner_generation, .. } if owner_batch == batch && owner_generation == generation => None,
                CommandDocumentRetirementState::Pending { epoch, batch: owner_batch, generation: owner_generation, surface, document } if owner_batch == batch && owner_generation == generation => {
                    Some(CommandDocumentRetirementState::Closing { epoch, batch: owner_batch, generation: owner_generation, surface, document })
                }
                state => Some(state),
            };
        }
    }

    pub fn begin_close_instance(&mut self, batch: u64) {
        for slot in &mut self.slots {
            let Some(state) = slot.take() else { continue };
            *slot = match state {
                CommandDocumentRetirementState::Reserved { batch: owner_batch, .. } if owner_batch == batch => None,
                CommandDocumentRetirementState::Pending { epoch, batch: owner_batch, generation, surface, document } if owner_batch == batch => {
                    Some(CommandDocumentRetirementState::Closing { epoch, batch: owner_batch, generation, surface, document })
                }
                state => Some(state),
            };
        }
    }

    pub fn publish_batch(&mut self, batch: u64, generation: u64, output: &mut UiFixedList<ExchangeSurfaceDocument, UI_DOCUMENT_LEASE_SLOTS>) {
        for slot in &mut self.slots {
            let Some(state) = slot.take() else { continue };
            *slot = match state {
                CommandDocumentRetirementState::Pending { epoch, batch: owner_batch, generation: owner_generation, surface, document } if owner_batch == batch && owner_generation == generation => {
                    let owner = ExchangeSurfaceDocument { surface, document };
                    match output.try_push(owner) {
                        Ok(()) => None,
                        Err(owner) => Some(CommandDocumentRetirementState::Closing { epoch, batch: owner_batch, generation: owner_generation, surface: owner.surface, document: owner.document }),
                    }
                }
                state => Some(state),
            };
        }
    }

    pub fn close_one(&mut self) -> bool {
        let index = self.cursor;
        self.cursor = (self.cursor + 1) % COMMAND_DOCUMENT_RETIREMENT_CAPACITY;
        let Some(CommandDocumentRetirementState::Closing { document, .. }) = self.slots[index].as_mut() else { return false };
        let terminal = document.close_step() && document.terminal_is_empty();
        if terminal {
            let terminal = self.slots[index].take().expect("terminal command document retirement slot");
            assert!(matches!(&terminal, CommandDocumentRetirementState::Closing { document, .. } if document.terminal_is_empty()), "command document retirement witness changed before removal");
        }
        true
    }

    pub fn has_close_work(&self) -> bool {
        self.slots.iter().flatten().any(|state| matches!(state, CommandDocumentRetirementState::Closing { .. }))
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn batch_is_empty(&self, batch: u64, generation: u64) -> bool {
        !self.slots.iter().flatten().any(|state| match state {
            CommandDocumentRetirementState::Reserved { batch: owner_batch, generation: owner_generation, .. }
            | CommandDocumentRetirementState::Pending { batch: owner_batch, generation: owner_generation, .. }
            | CommandDocumentRetirementState::Closing { batch: owner_batch, generation: owner_generation, .. } => *owner_batch == batch && *owner_generation == generation,
        })
    }

    pub fn instance_is_empty(&self, batch: u64) -> bool {
        !self.slots.iter().flatten().any(|state| match state {
            CommandDocumentRetirementState::Reserved { batch: owner_batch, .. } | CommandDocumentRetirementState::Pending { batch: owner_batch, .. } | CommandDocumentRetirementState::Closing { batch: owner_batch, .. } => *owner_batch == batch,
        })
    }
}
//#endregion 📦️CommandDocumentRetirement

/// 🧰️ One framework-reserved tool job (`Effect::SpawnJob` of kind
/// `semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND`: undo, redo, copy/paste, selection and
/// interaction verbs) this host drives to its end, the native twin of the React host's
/// `driveReservedToolJob` (`🧱️elements/🔌️PluginRuntime/🟦️.tsx`). `step` is the exact shard-minted
/// [`JobTurn`] of its next `Payload::JobStep`; `None` once the job ended or its spawn was refused, while
/// the shard's deferred `Event::JobCompleted` turn (an outcome no grant asked for) is still owed.
#[derive(Clone, Copy)]
pub struct ReservedToolJob {
    pub actor: ActorId,
    pub job: u64,
    pub step: Option<JobTurn>,
}

//#endregion
//#endregion 🎠️KernelRuntime
