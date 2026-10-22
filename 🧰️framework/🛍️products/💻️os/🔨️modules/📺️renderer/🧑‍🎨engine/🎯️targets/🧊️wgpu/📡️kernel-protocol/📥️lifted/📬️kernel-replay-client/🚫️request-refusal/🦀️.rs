//! 🚫️ Request and refusal: the mounted product replay request and the refusal token, cause, phase, retained refusal and refusal registry. See [`super`].

use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️Requests/Outcomes
/// 🎫️ Exact typed product owner moved from one successful mounted-session exchange.
#[derive(Debug)]
pub struct MountedProductReplayRequest {
    pub instance: u32,
    pub job: u64,
    pub job_kind: [u8; PRODUCT_REPLAY_KIND_BYTES],
    pub(super) job_kind_length: u16,
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

    pub(super) fn into_retained(mut self) -> (MountedProductReplayRecoveryToken, RetainedMountedProductReplayRequest) {
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
pub(super) enum MountedProductReplayRefusalPhase {
    Retry,
    SplitSpawn,
    Input,
    Kind,
    RemainingEffects,
    RemainingBacking,
    Terminal,
}

pub(super) struct RetainedRefusedProductReplay {
    pub instance: u32,
    pub cause: MountedProductReplayRefusalCause,
    pub selected_index: usize,
    pub spawn: Option<Effect>,
    pub(super) spawn_job: Option<u64>,
    pub(super) spawn_placement: Option<JobPlacement>,
    pub(super) spawn_kind: Option<String>,
    pub(super) spawn_input: Option<Vec<u8>>,
    pub remaining_effects: Option<Vec<Effect>>,
    pub(super) phase: MountedProductReplayRefusalPhase,
}

impl RetainedRefusedProductReplay {
    pub(super) fn observe_identity(&self) {
        let _ = (self.instance, self.cause, self.selected_index, self.spawn_job, self.spawn_placement);
        if let Some(Effect::SpawnJob { job, kind, input, placement }) = self.spawn.as_ref() {
            let _ = (job, kind.as_bytes(), input.as_slice(), placement);
        }
    }

    pub(super) fn retry(&mut self) -> Option<MountedProductReplayRequest> {
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
    pub(super) generation: u64,
    pub instance: Option<u32>,
    pub reserved: bool,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub owner: Option<RetainedRefusedProductReplay>,
    #[cfg(not(any(test, feature = "testkit")))]
    pub(super) owner: Option<RetainedRefusedProductReplay>,
}

struct MountedProductReplayRefusalRegistry {
    pub slots: Box<[MountedProductReplayRefusalSlot; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    pub(super) abandoned: Box<[Option<RetainedRefusedProductReplay>; JOB_PROGRESS_ACTIVE_CAPACITY]>,
    pub cursor: usize,
    pub(super) abandoned_cursor: usize,
}

struct MountedProductReplayRefusalStep {
    pub request: Option<MountedProductReplayRequest>,
    pub progressed: bool,
}

impl MountedProductReplayRefusalRegistry {
    pub(super) fn new() -> Self {
        Self {
            slots: semio_framework_async_slots::boxed_fixed_slots(|| MountedProductReplayRefusalSlot { generation: 0, instance: None, reserved: false, owner: None }),
            abandoned: semio_framework_async_slots::boxed_fixed_slots(|| None),
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
//#endregion
//#endregion 🎠️KernelRuntime
