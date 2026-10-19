//! 🔁️ Spawn job and recovery: the replay constants, the raw spawn job owner and the mounted product replay recovery token, owner and registry. See [`super`].

use super::*;

pub const PRODUCT_REPLAY_KIND_BYTES: usize = 256;

//#region 🎠️KernelRuntime
//#region 🔖️Requests/Outcomes
const PRODUCT_REPLAY_INPUT_PAGE_CAPACITY: usize = semio_framework_actor::JOB_REPLAY_RECORD_CAPACITY;

pub const PRODUCT_REPLAY_INPUT_BYTES: usize = semio_framework_actor::JOB_REPLAY_PAGE_BYTES * PRODUCT_REPLAY_INPUT_PAGE_CAPACITY;

pub const PRODUCT_REPLAY_PROFILE_COUNT: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawSpawnJobDisposition {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RawSpawnJobClosePhase {
    Input,
    Kind,
    Terminal,
}

#[derive(Debug)]
pub(super) struct RawSpawnJobOwner {
    pub job: u64,
    pub(super) kind: Option<String>,
    pub input: Option<Vec<u8>>,
    pub placement: JobPlacement,
    pub selected_index: usize,
    pub fixed_witness_acked: bool,
    pub mount_acked: bool,
    pub qualification_acked: bool,
    pub replay_acked: bool,
    checkpoint_ordinal: Option<u64>,
    checkpoint_digest: Option<u64>,
    pub terminal_ordinal: Option<u64>,
    pub terminal_prefix: Option<u64>,
    pub accepted_replay_ordinal: Option<u64>,
    pub disposition: RawSpawnJobDisposition,
    close_phase: RawSpawnJobClosePhase,
}

impl RawSpawnJobOwner {
    pub(super) fn from_effect(selected_index: usize, effect: Effect) -> Self {
        let Effect::SpawnJob { job, kind, input, placement } = effect else { unreachable!("admitted product source remains SpawnJob") };
        Self {
            job,
            kind: Some(kind),
            input: Some(input),
            placement,
            selected_index,
            fixed_witness_acked: false,
            mount_acked: false,
            qualification_acked: false,
            replay_acked: false,
            checkpoint_ordinal: None,
            checkpoint_digest: None,
            terminal_ordinal: None,
            terminal_prefix: None,
            accepted_replay_ordinal: None,
            disposition: RawSpawnJobDisposition::Pending,
            close_phase: RawSpawnJobClosePhase::Input,
        }
    }

    pub fn kind(&self) -> &str {
        self.kind.as_deref().expect("raw product kind remains owned before retirement")
    }

    pub fn input(&self) -> &[u8] {
        self.input.as_deref().expect("raw product input remains owned before retirement")
    }

    pub(super) fn acknowledge_fixed_witness(&mut self) {
        self.fixed_witness_acked = true;
    }

    pub fn acknowledge_mount(&mut self) {
        assert!(self.fixed_witness_acked && self.disposition == RawSpawnJobDisposition::Pending, "raw product mount follows exact fixed witness construction");
        self.mount_acked = true;
    }

    pub fn acknowledge_qualification(&mut self, checkpoint: Option<MountedProductReplayCheckpoint>) {
        assert!(self.mount_acked && self.disposition == RawSpawnJobDisposition::Pending, "raw product qualification follows accepted mount");
        self.qualification_acked = true;
        self.checkpoint_ordinal = checkpoint.map(|checkpoint| checkpoint.ordinal);
        self.checkpoint_digest = checkpoint.map(|checkpoint| checkpoint.digest);
    }

    pub(super) fn acknowledge_replay_boundary(&mut self, terminal: JobReplayRecordHeader, accepted_replay_ordinal: Option<u64>) -> Result<(), &'static str> {
        if !self.fixed_witness_acked || !self.mount_acked || !self.qualification_acked || self.disposition != RawSpawnJobDisposition::Pending || accepted_replay_ordinal.is_none() {
            return Err("raw product backing cannot retire before fixed, mount, qualification, and replay acknowledgements");
        }
        self.replay_acked = true;
        self.terminal_ordinal = Some(terminal.ordinal);
        self.terminal_prefix = Some(terminal.prefix_digest);
        self.accepted_replay_ordinal = accepted_replay_ordinal;
        self.disposition = RawSpawnJobDisposition::Accepted;
        Ok(())
    }

    pub(super) fn reject(&mut self) {
        if self.disposition == RawSpawnJobDisposition::Pending {
            self.disposition = RawSpawnJobDisposition::Rejected;
        }
    }

    pub(super) fn observe_identity(&self) {
        let _ = (
            self.job,
            self.kind.as_deref(),
            self.input.as_deref(),
            self.placement,
            self.selected_index,
            self.fixed_witness_acked,
            self.mount_acked,
            self.qualification_acked,
            self.replay_acked,
            self.checkpoint_ordinal,
            self.checkpoint_digest,
            self.terminal_ordinal,
            self.terminal_prefix,
            self.accepted_replay_ordinal,
            self.disposition,
        );
    }

    pub fn close_step(&mut self) -> bool {
        self.observe_identity();
        assert!(self.disposition != RawSpawnJobDisposition::Pending, "raw product backing closes only after accepted ACK boundary or exact rejection");
        match self.close_phase {
            RawSpawnJobClosePhase::Input => {
                drop(self.input.take().expect("raw admitted product input backing closes once"));
                self.close_phase = RawSpawnJobClosePhase::Kind;
            }
            RawSpawnJobClosePhase::Kind => {
                drop(self.kind.take().expect("raw admitted product kind backing closes once"));
                self.close_phase = RawSpawnJobClosePhase::Terminal;
            }
            RawSpawnJobClosePhase::Terminal => {}
        }
        self.close_phase == RawSpawnJobClosePhase::Terminal
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MountedProductReplayRecoveryToken {
    pub index: usize,
    pub generation: u64,
}

pub(super) struct RetainedMountedProductReplayRequest {
    pub instance: u32,
    pub job_kind: [u8; PRODUCT_REPLAY_KIND_BYTES],
    pub(super) job_kind_length: u16,
    pub request: JobReplayRequest,
    pub placement: JobPlacement,
    pub raw: RawSpawnJobOwner,
}

pub enum MountedProductReplayRecoveryOwner {
    Request(RetainedMountedProductReplayRequest),
    Claim {
        request: RetainedMountedProductReplayRequest,
        actor: ActorId,
        record_cursor: usize,
        checkpoint: Option<MountedProductReplayCheckpoint>,
    },
    Authority {
        request: RetainedMountedProductReplayRequest,
        actor: ActorId,
        route: JobReplayRoute,
        operation: JobOperation,
        generation: u64,
        seed: u64,
        checkpoint: Option<MountedProductReplayCheckpoint>,
        terminal: JobReplayRecordHeader,
        worker_count: u16,
        worker_slot: u16,
        begin: bool,
        restore_start_ordinal: Option<u64>,
        profile_cursor: usize,
        profile_started: bool,
    },
}

impl MountedProductReplayRecoveryOwner {
    pub fn instance(&self) -> u32 {
        match self {
            Self::Request(request) | Self::Claim { request, .. } | Self::Authority { request, .. } => request.instance,
        }
    }

    fn observe_close_identity(&self) {
        match self {
            Self::Request(request) => {
                let _ = (&request.job_kind, request.job_kind_length, request.request, request.placement);
                request.raw.observe_identity();
            }
            Self::Claim { request, actor, record_cursor, checkpoint } => {
                let _ = (&request.job_kind, request.job_kind_length, request.request, request.placement, actor, record_cursor, checkpoint);
                request.raw.observe_identity();
            }
            Self::Authority { request, actor, route, operation, generation, seed, checkpoint, terminal, worker_count, worker_slot, begin, restore_start_ordinal, profile_cursor, profile_started } => {
                let _ = (
                    &request.job_kind,
                    request.job_kind_length,
                    request.request,
                    request.placement,
                    actor,
                    route,
                    operation,
                    generation,
                    seed,
                    checkpoint,
                    terminal,
                    worker_count,
                    worker_slot,
                    begin,
                    restore_start_ordinal,
                    profile_cursor,
                    profile_started,
                );
                request.raw.observe_identity();
            }
        }
    }

    pub fn close_step(&mut self) -> bool {
        self.observe_close_identity();
        match self {
            Self::Request(request) | Self::Claim { request, .. } | Self::Authority { request, .. } => request.raw.close_step(),
        }
    }
}

struct MountedProductReplayRecoverySlot {
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub generation: u64,
    #[cfg(not(any(test, feature = "testkit")))]
    pub(super) generation: u64,
    pub reserved: bool,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub owner: Option<MountedProductReplayRecoveryOwner>,
    #[cfg(not(any(test, feature = "testkit")))]
    pub(super) owner: Option<MountedProductReplayRecoveryOwner>,
}

struct MountedProductReplayRecoveryRegistry {
    pub slots: Box<[MountedProductReplayRecoverySlot; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    pub(super) abandoned: Box<[Option<MountedProductReplayRecoveryOwner>; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    pub cursor: usize,
    pub(super) abandoned_cursor: usize,
}

impl MountedProductReplayRecoveryRegistry {
    pub(super) fn new() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| MountedProductReplayRecoverySlot { generation: 0, reserved: false, owner: None }), abandoned: semio_framework_async::boxed_fixed_slots(|| None), cursor: 0, abandoned_cursor: 0 }
    }

    pub fn reserve(&mut self) -> Option<MountedProductReplayRecoveryToken> {
        let index = self.slots.iter().position(|slot| !slot.reserved && slot.owner.is_none())?;
        let generation = self.slots[index].generation.checked_add(1).filter(|generation| *generation != 0)?;
        self.slots[index].generation = generation;
        self.slots[index].reserved = true;
        Some(MountedProductReplayRecoveryToken { index, generation })
    }

    pub fn release(&mut self, token: MountedProductReplayRecoveryToken) -> bool {
        let Some(slot) = self.slots.get_mut(token.index) else { return false };
        if slot.generation != token.generation || !slot.reserved || slot.owner.is_some() {
            return false;
        }
        slot.reserved = false;
        true
    }

    pub fn publish(&mut self, token: MountedProductReplayRecoveryToken, owner: MountedProductReplayRecoveryOwner) {
        if let Some(slot) = self.slots.get_mut(token.index) {
            if slot.generation == token.generation && slot.reserved && slot.owner.is_none() {
                slot.owner = Some(owner);
                return;
            }
        }
        let slot = self.abandoned.iter_mut().find(|slot| slot.is_none()).expect("pre-reserved product replay recovery keeps one discoverable exact owner per shell");
        *slot = Some(owner);
    }

    pub fn close_one(&mut self) -> bool {
        if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.is_some()) {
            self.cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
            if self.slots[index].owner.as_mut().expect("selected product replay recovery owner").close_step() {
                let terminal = self.slots[index].owner.take().expect("terminal product replay recovery owner");
                terminal.observe_close_identity();
                drop(terminal);
                self.slots[index].reserved = false;
            }
            return true;
        }
        let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].is_some()) else {
            return false;
        };
        self.abandoned_cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
        if self.abandoned[index].as_mut().expect("selected abandoned product replay owner").close_step() {
            let terminal = self.abandoned[index].take().expect("terminal abandoned product replay owner");
            terminal.observe_close_identity();
            drop(terminal);
        }
        true
    }

    pub fn has_close_work(&self) -> bool {
        self.slots.iter().any(|slot| slot.owner.is_some()) || self.abandoned.iter().any(Option::is_some)
    }

    pub fn has_instance(&self, instance: u32) -> bool {
        self.slots.iter().filter_map(|slot| slot.owner.as_ref()).chain(self.abandoned.iter().flatten()).any(|owner| owner.instance() == instance)
    }

    pub fn close_instance_one(&mut self, instance: u32) -> bool {
        if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.as_ref().is_some_and(|owner| owner.instance() == instance)) {
            self.cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
            if self.slots[index].owner.as_mut().expect("selected instance product replay recovery owner").close_step() {
                let terminal = self.slots[index].owner.take().expect("terminal instance product replay recovery owner");
                terminal.observe_close_identity();
                drop(terminal);
                self.slots[index].reserved = false;
            }
            return true;
        }
        let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].as_ref().is_some_and(|owner| owner.instance() == instance)) else {
            return false;
        };
        self.abandoned_cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
        if self.abandoned[index].as_mut().expect("selected abandoned instance product replay owner").close_step() {
            let terminal = self.abandoned[index].take().expect("terminal abandoned instance product replay owner");
            terminal.observe_close_identity();
            drop(terminal);
        }
        true
    }
}

pub fn mounted_product_replay_recovery_registry() -> &'static Mutex<MountedProductReplayRecoveryRegistry> {
    static REGISTRY: OnceLock<Mutex<MountedProductReplayRecoveryRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(MountedProductReplayRecoveryRegistry::new()))
}
//#endregion
//#endregion 🎠️KernelRuntime
