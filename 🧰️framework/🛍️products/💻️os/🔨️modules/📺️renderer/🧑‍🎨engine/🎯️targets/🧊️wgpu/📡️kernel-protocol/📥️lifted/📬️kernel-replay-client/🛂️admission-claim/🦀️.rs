//! 🛂️ Admission and claim: the admission permit, the refused replay, the admission, checkpoint and claim, the expected record and the replay authority. See [`super`].

use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️Requests/Outcomes
pub struct MountedProductReplayAdmissionPermit {
    pub token: Option<MountedProductReplayRefusalToken>,
}

impl MountedProductReplayAdmissionPermit {
    pub fn release(mut self) {
        let token = self.token.take().expect("product admission permit releases once");
        assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").release(token), "product admission permit release keeps exact generation");
    }

    pub(super) fn refuse(mut self, cause: MountedProductReplayRefusalCause, selected_index: usize, spawn: Effect, remaining_effects: Vec<Effect>) -> RefusedProductReplay {
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

    pub(super) fn into_retained(mut self) -> (MountedProductReplayRefusalToken, RetainedRefusedProductReplay) {
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
//#endregion
//#endregion 🎠️KernelRuntime
