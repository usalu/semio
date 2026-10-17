//! 📬️ Kernel replay vocabulary: mounted product replay admission, recovery and refusal registries, typed operation result pages and the exchange and response slots.
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
struct RawSpawnJobOwner {
    pub job: u64,
    kind: Option<String>,
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
    fn from_effect(selected_index: usize, effect: Effect) -> Self {
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

    fn acknowledge_fixed_witness(&mut self) {
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

    fn acknowledge_replay_boundary(&mut self, terminal: JobReplayRecordHeader, accepted_replay_ordinal: Option<u64>) -> Result<(), &'static str> {
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

    fn reject(&mut self) {
        if self.disposition == RawSpawnJobDisposition::Pending {
            self.disposition = RawSpawnJobDisposition::Rejected;
        }
    }

    fn observe_identity(&self) {
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

struct RetainedMountedProductReplayRequest {
    pub instance: u32,
    pub job_kind: [u8; PRODUCT_REPLAY_KIND_BYTES],
    job_kind_length: u16,
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
    generation: u64,
    pub reserved: bool,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub owner: Option<MountedProductReplayRecoveryOwner>,
    #[cfg(not(any(test, feature = "testkit")))]
    owner: Option<MountedProductReplayRecoveryOwner>,
}

struct MountedProductReplayRecoveryRegistry {
    pub slots: Box<[MountedProductReplayRecoverySlot; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    abandoned: Box<[Option<MountedProductReplayRecoveryOwner>; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    pub cursor: usize,
    abandoned_cursor: usize,
}

impl MountedProductReplayRecoveryRegistry {
    fn new() -> Self {
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

/// 🎫️ Exact typed product owner moved from one successful mounted-session exchange.
#[derive(Debug)]
pub struct MountedProductReplayRequest {
    pub instance: u32,
    pub job: u64,
    pub job_kind: [u8; PRODUCT_REPLAY_KIND_BYTES],
    job_kind_length: u16,
    pub request: JobReplayRequest,
    pub placement: JobPlacement,
    pub raw: Option<RawSpawnJobOwner>,
    pub recovery: Option<MountedProductReplayRecoveryToken>,
}

impl MountedProductReplayRequest {
    pub fn from_admitted_effect(instance: u32, selected_index: usize, effect: Effect, recovery: MountedProductReplayRecoveryToken) -> Self {
        let mut raw = RawSpawnJobOwner::from_effect(selected_index, effect);
        let request = JobReplayRequest::from_spawn(raw.kind(), raw.input());
        let mut job_kind = [0; PRODUCT_REPLAY_KIND_BYTES];
        job_kind[..raw.kind().len()].copy_from_slice(raw.kind().as_bytes());
        raw.acknowledge_fixed_witness();
        Self { instance, job: raw.job, job_kind, job_kind_length: raw.kind().len() as u16, request, placement: raw.placement, raw: Some(raw), recovery: Some(recovery) }
    }

    pub fn job_kind(&self) -> &[u8] {
        &self.job_kind[..usize::from(self.job_kind_length)]
    }

    pub fn rejection_reason(&self) -> String {
        format!("mounted product replay authority rejected for instance {}, job {}", self.instance, self.job)
    }

    pub fn raw(&self) -> &RawSpawnJobOwner {
        self.raw.as_ref().expect("mounted product replay request retains exact raw SpawnJob")
    }

    pub fn raw_mut(&mut self) -> &mut RawSpawnJobOwner {
        self.raw.as_mut().expect("mounted product replay request retains exact mutable raw SpawnJob")
    }

    fn into_retained(mut self) -> (MountedProductReplayRecoveryToken, RetainedMountedProductReplayRequest) {
        let recovery = self.recovery.take().expect("mounted product replay recovery authority transfers once");
        let raw = self.raw.take().expect("mounted product replay raw SpawnJob transfers once");
        let retained = RetainedMountedProductReplayRequest { instance: self.instance, job_kind: self.job_kind, job_kind_length: self.job_kind_length, request: self.request, placement: self.placement, raw };
        (recovery, retained)
    }

    pub fn retire(mut self) {
        self.raw_mut().reject();
        let (recovery, retained) = self.into_retained();
        mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").publish(recovery, MountedProductReplayRecoveryOwner::Request(retained));
    }
}

impl Drop for MountedProductReplayRequest {
    fn drop(&mut self) {
        let Some(recovery) = self.recovery.take() else { return };
        let Some(mut raw) = self.raw.take() else { return };
        raw.reject();
        let retained = RetainedMountedProductReplayRequest { instance: self.instance, job_kind: self.job_kind, job_kind_length: self.job_kind_length, request: self.request, placement: self.placement, raw };
        mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").publish(recovery, MountedProductReplayRecoveryOwner::Request(retained));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MountedProductReplayRefusalToken {
    pub index: usize,
    pub generation: u64,
    pub instance: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedProductReplayRefusalCause {
    KindCapacity,
    InputCapacity,
    RecoveryCapacity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MountedProductReplayRefusalPhase {
    Retry,
    SplitSpawn,
    Input,
    Kind,
    RemainingEffects,
    RemainingBacking,
    Terminal,
}

struct RetainedRefusedProductReplay {
    pub instance: u32,
    pub cause: MountedProductReplayRefusalCause,
    pub selected_index: usize,
    pub spawn: Option<Effect>,
    spawn_job: Option<u64>,
    spawn_placement: Option<JobPlacement>,
    spawn_kind: Option<String>,
    spawn_input: Option<Vec<u8>>,
    pub remaining_effects: Option<Vec<Effect>>,
    phase: MountedProductReplayRefusalPhase,
}

impl RetainedRefusedProductReplay {
    fn observe_identity(&self) {
        let _ = (self.instance, self.cause, self.selected_index, self.spawn_job, self.spawn_placement);
        if let Some(Effect::SpawnJob { job, kind, input, placement }) = self.spawn.as_ref() {
            let _ = (job, kind.as_bytes(), input.as_slice(), placement);
        }
    }

    fn retry(&mut self) -> Option<MountedProductReplayRequest> {
        if self.phase != MountedProductReplayRefusalPhase::Retry {
            return None;
        }
        let recovery = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve()?;
        let Some(spawn) = self.spawn.take() else {
            self.phase = MountedProductReplayRefusalPhase::SplitSpawn;
            let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").release(recovery);
            return None;
        };
        let request = MountedProductReplayRequest::from_admitted_effect(self.instance, self.selected_index, spawn, recovery);
        self.phase = MountedProductReplayRefusalPhase::RemainingEffects;
        Some(request)
    }

    pub fn close_step(&mut self) -> bool {
        self.observe_identity();
        match self.phase {
            MountedProductReplayRefusalPhase::Retry => {
                self.phase = MountedProductReplayRefusalPhase::SplitSpawn;
            }
            MountedProductReplayRefusalPhase::SplitSpawn => {
                let Some(effect) = self.spawn.take() else {
                    self.phase = MountedProductReplayRefusalPhase::RemainingEffects;
                    return false;
                };
                let Effect::SpawnJob { job, kind, input, placement } = effect else { unreachable!("product replay refusal retains the selected SpawnJob") };
                self.spawn_job = Some(job);
                self.spawn_placement = Some(placement);
                self.spawn_kind = Some(kind);
                self.spawn_input = Some(input);
                self.phase = MountedProductReplayRefusalPhase::Input;
            }
            MountedProductReplayRefusalPhase::Input => {
                drop(self.spawn_input.take().expect("refused product input backing closes once"));
                self.phase = MountedProductReplayRefusalPhase::Kind;
            }
            MountedProductReplayRefusalPhase::Kind => {
                drop(self.spawn_kind.take().expect("refused product kind backing closes once"));
                self.phase = MountedProductReplayRefusalPhase::RemainingEffects;
            }
            MountedProductReplayRefusalPhase::RemainingEffects => {
                let remaining = self.remaining_effects.as_mut().expect("refused product outcome remainder is retained");
                if let Some(effect) = remaining.pop() {
                    drop(effect);
                } else {
                    self.phase = MountedProductReplayRefusalPhase::RemainingBacking;
                }
            }
            MountedProductReplayRefusalPhase::RemainingBacking => {
                drop(self.remaining_effects.take().expect("refused product outcome backing closes once"));
                self.phase = MountedProductReplayRefusalPhase::Terminal;
            }
            MountedProductReplayRefusalPhase::Terminal => {}
        }
        self.phase == MountedProductReplayRefusalPhase::Terminal
    }
}

struct MountedProductReplayRefusalSlot {
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub generation: u64,
    #[cfg(not(any(test, feature = "testkit")))]
    generation: u64,
    pub instance: Option<u32>,
    pub reserved: bool,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub owner: Option<RetainedRefusedProductReplay>,
    #[cfg(not(any(test, feature = "testkit")))]
    owner: Option<RetainedRefusedProductReplay>,
}

struct MountedProductReplayRefusalRegistry {
    pub slots: Box<[MountedProductReplayRefusalSlot; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    abandoned: Box<[Option<RetainedRefusedProductReplay>; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    pub cursor: usize,
    abandoned_cursor: usize,
}

struct MountedProductReplayRefusalStep {
    pub request: Option<MountedProductReplayRequest>,
    pub progressed: bool,
}

impl MountedProductReplayRefusalRegistry {
    fn new() -> Self {
        Self {
            slots: semio_framework_async::boxed_fixed_slots(|| MountedProductReplayRefusalSlot { generation: 0, instance: None, reserved: false, owner: None }),
            abandoned: semio_framework_async::boxed_fixed_slots(|| None),
            cursor: 0,
            abandoned_cursor: 0,
        }
    }

    pub fn reserve(&mut self, instance: u32) -> Option<MountedProductReplayRefusalToken> {
        let index = self.slots.iter().position(|slot| !slot.reserved && slot.owner.is_none())?;
        let generation = self.slots[index].generation.checked_add(1).filter(|generation| *generation != 0)?;
        self.slots[index].generation = generation;
        self.slots[index].instance = Some(instance);
        self.slots[index].reserved = true;
        Some(MountedProductReplayRefusalToken { index, generation, instance })
    }

    pub fn release(&mut self, token: MountedProductReplayRefusalToken) -> bool {
        let Some(slot) = self.slots.get_mut(token.index) else { return false };
        if slot.generation != token.generation || slot.instance != Some(token.instance) || !slot.reserved || slot.owner.is_some() {
            return false;
        }
        slot.instance = None;
        slot.reserved = false;
        true
    }

    pub fn publish(&mut self, token: MountedProductReplayRefusalToken, owner: RetainedRefusedProductReplay) {
        if let Some(slot) = self.slots.get_mut(token.index) {
            if slot.generation == token.generation && slot.instance == Some(owner.instance) && slot.reserved && slot.owner.is_none() {
                slot.owner = Some(owner);
                return;
            }
        }
        let slot = self.abandoned.iter_mut().find(|slot| slot.is_none()).expect("pre-reserved refusal authority keeps one discoverable product owner per bridge call");
        *slot = Some(owner);
    }

    pub fn step_one(&mut self, retry: bool) -> MountedProductReplayRefusalStep {
        if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.is_some()) {
            self.cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
            let owner = self.slots[index].owner.as_mut().expect("selected product refusal owner");
            if retry {
                if let Some(request) = owner.retry() {
                    return MountedProductReplayRefusalStep { request: Some(request), progressed: true };
                }
                if owner.phase == MountedProductReplayRefusalPhase::Retry {
                    return MountedProductReplayRefusalStep { request: None, progressed: true };
                }
            }
            let terminal = owner.close_step();
            if terminal {
                let terminal = self.slots[index].owner.take().expect("terminal product refusal owner");
                assert_eq!(terminal.phase, MountedProductReplayRefusalPhase::Terminal, "product refusal terminal identity changed before release");
                self.slots[index].instance = None;
                self.slots[index].reserved = false;
                drop(terminal);
            }
            return MountedProductReplayRefusalStep { request: None, progressed: true };
        }
        let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].is_some()) else {
            return MountedProductReplayRefusalStep { request: None, progressed: false };
        };
        self.abandoned_cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
        let owner = self.abandoned[index].as_mut().expect("selected abandoned product refusal owner");
        let terminal = owner.close_step();
        if terminal {
            drop(self.abandoned[index].take().expect("terminal abandoned product refusal owner"));
        }
        MountedProductReplayRefusalStep { request: None, progressed: true }
    }

    pub fn has_work(&self) -> bool {
        self.slots.iter().any(|slot| slot.owner.is_some()) || self.abandoned.iter().any(Option::is_some)
    }

    pub fn has_any(&self) -> bool {
        self.slots.iter().any(|slot| slot.reserved || slot.owner.is_some()) || self.abandoned.iter().any(Option::is_some)
    }

    pub fn has_instance(&self, instance: u32) -> bool {
        self.slots.iter().any(|slot| slot.instance == Some(instance)) || self.abandoned.iter().flatten().any(|owner| owner.instance == instance)
    }

    pub fn close_instance_one(&mut self, instance: u32) -> bool {
        if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.as_ref().is_some_and(|owner| owner.instance == instance)) {
            self.cursor = index;
            let _ = self.step_one(false);
            return true;
        }
        if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].as_ref().is_some_and(|owner| owner.instance == instance)) {
            self.abandoned_cursor = index;
            let _ = self.step_one(false);
            return true;
        }
        false
    }
}

pub fn mounted_product_replay_refusal_registry() -> &'static Mutex<MountedProductReplayRefusalRegistry> {
    static REGISTRY: OnceLock<Mutex<MountedProductReplayRefusalRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(MountedProductReplayRefusalRegistry::new()))
}

pub struct MountedProductReplayAdmissionPermit {
    pub token: Option<MountedProductReplayRefusalToken>,
}

impl MountedProductReplayAdmissionPermit {
    pub fn release(mut self) {
        let token = self.token.take().expect("product admission permit releases once");
        assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").release(token), "product admission permit release keeps exact generation");
    }

    fn refuse(mut self, cause: MountedProductReplayRefusalCause, selected_index: usize, spawn: Effect, remaining_effects: Vec<Effect>) -> RefusedProductReplay {
        let token = self.token.take().expect("product admission permit transfers once");
        RefusedProductReplay { token: Some(token), instance: token.instance, cause, selected_index, spawn: Some(spawn), remaining_effects: Some(remaining_effects) }
    }
}

impl Drop for MountedProductReplayAdmissionPermit {
    fn drop(&mut self) {
        let Some(token) = self.token.take() else { return };
        let _ = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").release(token);
    }
}

pub struct RefusedProductReplay {
    pub token: Option<MountedProductReplayRefusalToken>,
    pub instance: u32,
    pub cause: MountedProductReplayRefusalCause,
    pub selected_index: usize,
    pub spawn: Option<Effect>,
    pub remaining_effects: Option<Vec<Effect>>,
}

impl RefusedProductReplay {
    pub fn rejection_reason(&self) -> &'static str {
        match self.cause {
            MountedProductReplayRefusalCause::KindCapacity => "mounted product replay job kind exceeds its fixed authority",
            MountedProductReplayRefusalCause::InputCapacity => "mounted product replay input exceeds its fixed page authority",
            MountedProductReplayRefusalCause::RecoveryCapacity => "mounted product replay recovery capacity is full",
        }
    }

    fn into_retained(mut self) -> (MountedProductReplayRefusalToken, RetainedRefusedProductReplay) {
        let token = self.token.take().expect("product refusal recovery authority transfers once");
        let phase = if self.cause == MountedProductReplayRefusalCause::RecoveryCapacity { MountedProductReplayRefusalPhase::Retry } else { MountedProductReplayRefusalPhase::SplitSpawn };
        let retained = RetainedRefusedProductReplay {
            instance: self.instance,
            cause: self.cause,
            selected_index: self.selected_index,
            spawn: self.spawn.take(),
            spawn_job: None,
            spawn_placement: None,
            spawn_kind: None,
            spawn_input: None,
            remaining_effects: self.remaining_effects.take(),
            phase,
        };
        (token, retained)
    }

    pub fn retire(self) {
        let (token, retained) = self.into_retained();
        mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").publish(token, retained);
    }
}

impl Drop for RefusedProductReplay {
    fn drop(&mut self) {
        let Some(token) = self.token.take() else { return };
        let retained = RetainedRefusedProductReplay {
            instance: self.instance,
            cause: self.cause,
            selected_index: self.selected_index,
            spawn: self.spawn.take(),
            spawn_job: None,
            spawn_placement: None,
            spawn_kind: None,
            spawn_input: None,
            remaining_effects: self.remaining_effects.take(),
            phase: if self.cause == MountedProductReplayRefusalCause::RecoveryCapacity { MountedProductReplayRefusalPhase::Retry } else { MountedProductReplayRefusalPhase::SplitSpawn },
        };
        mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").publish(token, retained);
    }
}

pub enum MountedProductReplayAdmission {
    None,
    Admitted(MountedProductReplayRequest),
    Refused(RefusedProductReplay),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MountedProductReplayCheckpoint {
    pub ordinal: u64,
    pub digest: u64,
    pub pages: u8,
    pub applied_progress: u64,
}

pub struct MountedProductReplayClaim {
    pub request: Option<MountedProductReplayRequest>,
    pub actor: ActorId,
    pub record_cursor: usize,
    pub checkpoint: Option<MountedProductReplayCheckpoint>,
}

impl MountedProductReplayClaim {
    pub fn request(&self) -> &MountedProductReplayRequest {
        self.request.as_ref().expect("mounted product replay claim retains request")
    }

    pub fn instance(&self) -> u32 {
        self.request().instance
    }

    pub fn job(&self) -> u64 {
        self.request().job
    }

    pub fn retire(mut self) {
        let mut request = self.request.take().expect("mounted product replay claim retires once");
        request.raw_mut().reject();
        let (recovery, request) = request.into_retained();
        mounted_product_replay_recovery_registry()
            .lock()
            .expect("product replay recovery lock")
            .publish(recovery, MountedProductReplayRecoveryOwner::Claim { request, actor: self.actor, record_cursor: self.record_cursor, checkpoint: self.checkpoint });
    }
}

impl Drop for MountedProductReplayClaim {
    fn drop(&mut self) {
        let Some(mut request) = self.request.take() else { return };
        request.raw_mut().reject();
        let (recovery, request) = request.into_retained();
        mounted_product_replay_recovery_registry()
            .lock()
            .expect("product replay recovery lock")
            .publish(recovery, MountedProductReplayRecoveryOwner::Claim { request, actor: self.actor, record_cursor: self.record_cursor, checkpoint: self.checkpoint });
    }
}

#[derive(Clone, Copy)]
pub struct MountedProductReplayExpected {
    pub instance: u32,
    pub actor: ActorId,
    pub turn: JobTurn,
    pub request: JobReplayRequest,
    pub placement: JobPlacement,
    pub route: JobReplayRoute,
    pub checkpoint: Option<MountedProductReplayCheckpoint>,
    pub terminal: JobReplayRecordHeader,
    pub process_worker_count: u16,
    pub process_worker_slot: u16,
    pub logical_worker_count: u16,
    pub logical_worker_slot: u16,
    pub begin: bool,
    pub restore_start_ordinal: Option<u64>,
}

/// 🪪️ Full mounted identity qualified against the retained shard log before replay.
#[derive(Debug)]
pub struct MountedProductReplayAuthority {
    pub request_owner: Option<MountedProductReplayRequest>,
    pub actor: ActorId,
    pub route: JobReplayRoute,
    pub operation: JobOperation,
    pub generation: u64,
    pub seed: u64,
    pub checkpoint: Option<MountedProductReplayCheckpoint>,
    pub terminal: JobReplayRecordHeader,
    pub worker_count: u16,
    pub worker_slot: u16,
    pub begin: bool,
    pub restore_start_ordinal: Option<u64>,
    pub profile_cursor: usize,
    pub profile_started: bool,
}

impl MountedProductReplayAuthority {
    pub fn request_owner(&self) -> &MountedProductReplayRequest {
        self.request_owner.as_ref().expect("mounted product replay authority retains request")
    }

    fn request_owner_mut(&mut self) -> &mut MountedProductReplayRequest {
        self.request_owner.as_mut().expect("mounted product replay authority retains mutable request")
    }

    pub fn instance(&self) -> u32 {
        self.request_owner().instance
    }

    pub fn job(&self) -> u64 {
        self.request_owner().job
    }

    pub fn validate(self, expected: MountedProductReplayExpected) -> Result<Self, Self> {
        let request_owner = self.request_owner();
        let kind = std::str::from_utf8(request_owner.job_kind()).ok();
        let kind_identity = kind.map(|kind| JobReplayRequest::from_spawn(kind, &[]));
        let raw_request = JobReplayRequest::from_spawn(request_owner.raw().kind(), request_owner.raw().input());
        let valid = request_owner.instance == expected.instance
            && request_owner.job == expected.turn.job
            && request_owner.raw().job == request_owner.job
            && request_owner.raw().kind().as_bytes() == request_owner.job_kind()
            && raw_request == request_owner.request
            && request_owner.raw().placement == request_owner.placement
            && kind_identity.is_some_and(|kind| kind.controller == expected.request.controller && kind.tool == expected.request.tool && kind.schema == expected.request.schema && kind.version == expected.request.version)
            && request_owner.request == expected.request
            && request_owner.placement == expected.placement
            && self.actor == expected.actor
            && self.operation == expected.turn.operation
            && self.generation == expected.turn.operation.generation
            && self.seed == expected.turn.operation.seed
            && self.route == expected.route
            && self.checkpoint == expected.checkpoint
            && self.terminal == expected.terminal
            && expected.terminal.worker_count == expected.process_worker_count
            && expected.terminal.worker_slot == expected.process_worker_slot
            && self.worker_count == expected.logical_worker_count
            && self.worker_slot == expected.logical_worker_slot
            && self.begin == expected.begin
            && self.restore_start_ordinal == expected.restore_start_ordinal;
        if valid {
            Ok(self)
        } else {
            Err(self)
        }
    }

    pub fn acknowledge_raw_retirement_boundary(&mut self) -> Result<(), &'static str> {
        if self.profile_cursor != PRODUCT_REPLAY_PROFILE_COUNT || self.profile_started {
            return Err("raw product retirement requires every logical replay profile to reach terminal ACK");
        }
        let terminal = self.terminal;
        let accepted_replay_ordinal = self.restore_start_ordinal;
        self.request_owner_mut().raw_mut().acknowledge_replay_boundary(terminal, accepted_replay_ordinal)
    }

    pub fn retire(mut self) {
        let mut request = self.request_owner.take().expect("mounted product replay authority retires once");
        request.raw_mut().reject();
        let (recovery, request) = request.into_retained();
        mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").publish(
            recovery,
            MountedProductReplayRecoveryOwner::Authority {
                request,
                actor: self.actor,
                route: self.route,
                operation: self.operation,
                generation: self.generation,
                seed: self.seed,
                checkpoint: self.checkpoint,
                terminal: self.terminal,
                worker_count: self.worker_count,
                worker_slot: self.worker_slot,
                begin: self.begin,
                restore_start_ordinal: self.restore_start_ordinal,
                profile_cursor: self.profile_cursor,
                profile_started: self.profile_started,
            },
        );
    }
}

impl Drop for MountedProductReplayAuthority {
    fn drop(&mut self) {
        let Some(mut request) = self.request_owner.take() else { return };
        request.raw_mut().reject();
        let (recovery, request) = request.into_retained();
        mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").publish(
            recovery,
            MountedProductReplayRecoveryOwner::Authority {
                request,
                actor: self.actor,
                route: self.route,
                operation: self.operation,
                generation: self.generation,
                seed: self.seed,
                checkpoint: self.checkpoint,
                terminal: self.terminal,
                worker_count: self.worker_count,
                worker_slot: self.worker_slot,
                begin: self.begin,
                restore_start_ordinal: self.restore_start_ordinal,
                profile_cursor: self.profile_cursor,
                profile_started: self.profile_started,
            },
        );
    }
}

pub fn mounted_product_replay_profile(process_default: u16, cursor: usize) -> Option<u16> {
    [1, 2, 4, process_default].get(cursor).copied().filter(|worker_count| *worker_count != 0)
}

pub fn mounted_product_replay_worker_slot(process_slot: u16, worker_count: u16) -> Option<u16> {
    (worker_count != 0 && process_slot != u16::MAX).then_some(process_slot % worker_count)
}

//#region 📬️TypedOperationResultPage
/// 📬️ One typed-operation result page a guest publishes to its shell (`semio.typed-operation-page.v1`),
/// and the exact ACK (`semio.typed-operation-ack.v1`) the guest waits for before it retires the page
/// and continues the operation. The kernel thread acknowledges every page inside the turn settle
/// that received it ([`KernelPoolState::run_turn`]), exactly as the React host acknowledges inside
/// `settlePluginTurn` (`typedOperationAcknowledgements`), and hands the pages to the caller on
/// [`ExchangeOutcome::typed_results`]. Before this, the page waited in a renderer exchange no
/// native consumer ever read, so every reserved tool job (undo, redo, checkpoint, copy, paste…)
/// stalled its guest in `MoreWork` (ticket 26/09/23 slice WG8, measured on block2d's undo).
pub const TYPED_OPERATION_RESULT_PAGE_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedOperationResultToken {
    pub receiver: u32,
    pub operation: u64,
    pub generation: u64,
    pub sequence: u32,
    pub attempt: u8,
}

#[derive(Clone)]
pub struct TypedOperationResultPage {
    pub token: TypedOperationResultToken,
    pub lane: u8,
    pub bytes: [u8; TYPED_OPERATION_RESULT_PAGE_BYTES],
    pub len: usize,
}

impl TypedOperationResultPage {
    const PAGE_MAGIC: &'static [u8] = b"semio.typed-operation-page.v1\0";
    const ACK_MAGIC: &'static [u8] = b"semio.typed-operation-ack.v1\0";
    /// 🛤️ Highest `TypedOperationResultLane` discriminant the guest writes at byte 25 of a
    /// page (`🔌️plugin/🦀️.rs`: Artifact 0 … Fault 11, Interaction 12, WindowTransient 13,
    /// WindowConfig 14). Capped at 12 this demux refused the guest's OWN mesh publication lane and
    /// fell the page through to `decode_app_frame`, which cannot read it either
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END); the neutral lane fixture is the oracle.
    const LANE_MAX: u8 = 14;

    pub fn try_copy_from(token: TypedOperationResultToken, lane: u8, bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err("typed-operation result exceeds renderer fixed page authority");
        }
        let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: bytes.len() };
        page.bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(page)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn decode_guest_message(bytes: &[u8]) -> Option<Self> {
        let body = bytes.strip_prefix(Self::PAGE_MAGIC)?;
        if body.len() < 30 {
            return None;
        }
        let token = TypedOperationResultToken {
            receiver: u32::from_le_bytes(body[0..4].try_into().ok()?),
            operation: u64::from_le_bytes(body[4..12].try_into().ok()?),
            generation: u64::from_le_bytes(body[12..20].try_into().ok()?),
            sequence: u32::from_le_bytes(body[20..24].try_into().ok()?),
            attempt: body[24],
        };
        let lane = body[25];
        if lane > Self::LANE_MAX {
            return None;
        }
        let len = u32::from_le_bytes(body[26..30].try_into().ok()?) as usize;
        if len > TYPED_OPERATION_RESULT_PAGE_BYTES || body.len() != 30 + len {
            return None;
        }
        Self::try_copy_from(token, lane, &body[30..]).ok()
    }

    pub fn encode_ack(token: TypedOperationResultToken) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::ACK_MAGIC.len() + 25);
        bytes.extend_from_slice(Self::ACK_MAGIC);
        bytes.extend_from_slice(&token.receiver.to_le_bytes());
        bytes.extend_from_slice(&token.operation.to_le_bytes());
        bytes.extend_from_slice(&token.generation.to_le_bytes());
        bytes.extend_from_slice(&token.sequence.to_le_bytes());
        bytes.push(token.attempt);
        bytes
    }
}

#[cfg(test)]
#[path = "../../../../../🧪️tests/🗞️typed-result-page/🦀️.rs"]
mod typed_result_page_tests;

//#endregion 📬️TypedOperationResultPage

pub struct ExchangeSurfaceDocument {
    pub surface: SurfaceId,
    pub document: UiDocumentLease,
}

pub struct ExchangeOutcome {
    pub frames: Vec<protocol::AppFrame>,
    /// 🖼️ Surfaces this turn repainted or retained on desync — reconciled against the kernel
    /// pool state machine's retained tree (`KernelPoolState::retained`); see that field's doc for the
    /// full-body-vs-desync policy.
    pub surfaces: UiFixedList<ExchangeSurfaceDocument, UI_DOCUMENT_LEASE_SLOTS>,
    /// 🧾️ Every effect this turn produced that was NOT one of the `Effect::SendMessage{target:
    /// Shell{..}}` entries already unpacked into `frames` above — `📓️design-abi.md` §2's
    /// replacement for the deleted `AppFrame::Effects` wrapper: effects now travel as real
    /// `kernel::Effect` values on `TurnResult.effects` directly, not re-encoded as an `AppFrame`.
    pub effects: Vec<Effect>,
    pub command_ingress: semio_framework::kernel::CommandIngressStatus,
    /// 📬️ Every typed-operation result page this exchange's turns published to the shell, in
    /// publication order — each already acknowledged to the guest inside the settle.
    pub typed_results: Vec<TypedOperationResultPage>,
}

impl ExchangeOutcome {
    /// 🤫️ Whether this turn carried nothing a caller could act on — no frame, effect, typed result
    /// or surface document and an idle command ingress. The native twin of the browser drive's
    /// `shardTurnCarriesNothingV1` (`🎭️actor/🖼️wire-turn/🟦️.ts`) at this host's level, where
    /// patches have already become surface documents and shell messages frames.
    pub fn carries_nothing(&self) -> bool {
        self.frames.is_empty() && self.effects.is_empty() && self.surfaces.is_empty() && self.typed_results.is_empty() && matches!(self.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle)
    }

    /// ➕️ Folds a continuation turn's outcome into this one: frames, effects and typed results in
    /// order, the latest command-ingress status a turn actually reported, and each surface's LATER
    /// document (a continuation repainting a surface supersedes the earlier lease, which is handed
    /// back by its own `Drop`). An `Idle` ingress reports nothing: the acknowledgement turn after a
    /// page turn answers `Idle`, and letting it overwrite the page's `CommandComplete` made
    /// [`KernelPoolState::exchange_commands`] resend the page for ever — measured as one
    /// `addHandleKind` applied 64 times, then 6 466 resends in 83 min, on native block2d (ticket
    /// 26/09/23 slice WG8, `wp-wg8/generated/journey-{9,10}`).
    pub fn absorb(&mut self, later: ExchangeOutcome) -> Result<(), String> {
        self.frames.extend(later.frames);
        self.effects.extend(later.effects);
        self.typed_results.extend(later.typed_results);
        if !matches!(later.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle) {
            self.command_ingress = later.command_ingress;
        }
        for entry in later.surfaces {
            if let Some(index) = self.surfaces.iter().position(|earlier| earlier.surface == entry.surface) {
                let _ = self.surfaces.swap_remove(index);
            }
            self.surfaces.try_push(entry).map_err(|_| "kernel: continuation surfaces exceed the exchange's fixed surface capacity".to_string())?;
        }
        Ok(())
    }

    pub fn take_surface(&mut self, surface: &str) -> Option<UiDocumentLease> {
        let index = self.surfaces.iter().position(|entry| entry.surface.as_ref() == surface)?;
        self.surfaces.swap_remove(index).map(|entry| entry.document)
    }

    pub fn take_product_replay_authority(&mut self, instance: u32, permit: MountedProductReplayAdmissionPermit) -> MountedProductReplayAdmission {
        let Some(index) = self.effects.iter().position(|effect| matches!(effect, Effect::SpawnJob { .. })) else {
            permit.release();
            return MountedProductReplayAdmission::None;
        };
        let (kind, input) = match &self.effects[index] {
            Effect::SpawnJob { kind, input, .. } => (kind.as_str(), input.as_slice()),
            _ => unreachable!("selected product replay effect remains SpawnJob"),
        };
        let cause = if kind.len() > PRODUCT_REPLAY_KIND_BYTES {
            Some(MountedProductReplayRefusalCause::KindCapacity)
        } else if input.len() > PRODUCT_REPLAY_INPUT_BYTES {
            Some(MountedProductReplayRefusalCause::InputCapacity)
        } else {
            None
        };
        let recovery = cause.is_none().then(|| mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve()).flatten();
        if let Some(recovery) = recovery {
            let spawn = self.effects.remove(index);
            let request = MountedProductReplayRequest::from_admitted_effect(instance, index, spawn, recovery);
            permit.release();
            return MountedProductReplayAdmission::Admitted(request);
        }
        let cause = cause.unwrap_or(MountedProductReplayRefusalCause::RecoveryCapacity);
        let spawn = self.effects.remove(index);
        let remaining_effects = std::mem::take(&mut self.effects);
        MountedProductReplayAdmission::Refused(permit.refuse(cause, index, spawn, remaining_effects))
    }
}

pub enum KernelOutcome {
    Created(Result<u32, ProgramFault>),
    Exchanged(Result<ExchangeOutcome, ProgramFault>),
    ProductReplayMounted(Result<(), MountedProductReplayRequest>),
}
//#endregion

//#region 🔖️KernelFuture — the leaf `Future` every `ProgramBridgeEntry` async method awaits
#[derive(Default)]
struct ResponseState {
    pub result: Option<KernelOutcome>,
    pub waker: Option<Waker>,
}

#[derive(Default)]
pub struct ResponseSlot {
    pub state: Mutex<ResponseState>,
    pub abandoned: std::sync::atomic::AtomicBool,
}

impl ResponseSlot {
    /// 🛑️ Whether the requester dropped its future before an outcome reached it — the explicit cancel of
    /// a request whose owner follows it with progress and a cancel (a document open's create request).
    pub fn abandoned(&self) -> bool {
        self.abandoned.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn deliver(&self, outcome: KernelOutcome) {
        let waker = {
            let mut state = self.state.lock().expect("response slot lock");
            state.result = Some(outcome);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

//#endregion 🔖️KernelFuture — the leaf `Future` every `ProgramBridgeEntry` async method awaits
//#endregion 🎠️KernelRuntime
