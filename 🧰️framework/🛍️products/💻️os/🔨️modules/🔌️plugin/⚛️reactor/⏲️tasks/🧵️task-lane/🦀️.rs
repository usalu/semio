//! 🧵️ Task records, resumes, quota, spawn and cancellation of the actor-local task lane.

use crate::{INSTANCE_METADATA, REACTOR_EXECUTOR, TASK_EXECUTOR, TASK_RECORDS, TASK_RESUMES};
use crate::tasks_fixed_slots::{REACTOR_TASK_SLOTS, ReactorFixedSlots};
use std::collections::VecDeque;
use semio_framework_plugin_actor_host::executor;
use crate::tasks_extension_continuation::{host, host_for_instance};

/// 🧵️ A task retains its instance and optional checkpoint restart command.
struct TaskRecord {
    pub instance: u32,
    pub key: Option<String>,
    pub restart: Option<Vec<u8>>,
}
const REACTOR_TASK_KEY_BYTES: usize = 256;
const REACTOR_TASK_LABEL_BYTES: usize = 256;
const REACTOR_TASK_RESTART_BYTES: usize = 64 * 1_024;

#[doc(hidden)]
pub struct TaskRecordRegistry {
    pub slots: ReactorFixedSlots<(executor::TaskId, TaskRecord)>,
}

impl TaskRecordRegistry {
    pub fn new() -> Self {
        Self { slots: ReactorFixedSlots::new() }
    }

    pub fn index(id: executor::TaskId) -> usize {
        id as usize % REACTOR_TASK_SLOTS
    }

    pub fn can_insert(&self, id: executor::TaskId) -> bool {
        self.slots.allocation_admitted && self.slots.get(Self::index(id)).is_none()
    }

    pub fn insert_admitted(&mut self, id: executor::TaskId, record: TaskRecord) {
        debug_assert!(self.can_insert(id));
        self.slots.insert_admitted(Self::index(id), (id, record));
    }

    pub fn remove(&mut self, id: executor::TaskId) -> Option<TaskRecord> {
        let index = Self::index(id);
        if self.slots.get(index).is_none_or(|(candidate, _)| *candidate != id) {
            None
        } else {
            self.slots.take(index).map(|(_, record)| record)
        }
    }

    pub fn find_key(&self, instance: u32, key: &str) -> Option<executor::TaskId> {
        self.slots.iter().find_map(|(id, record)| (record.instance == instance && record.key.as_deref() == Some(key)).then_some(*id))
    }

    pub fn count_instance(&self, instance: u32) -> usize {
        self.slots.iter().filter(|(_, record)| record.instance == instance).count()
    }

    pub fn iter(&self) -> impl Iterator<Item = (executor::TaskId, &TaskRecord)> {
        self.slots.iter().map(|(id, record)| (*id, record))
    }

    pub fn entry_at(&self, index: usize) -> Option<(executor::TaskId, &TaskRecord)> {
        self.slots.get(index).map(|(id, record)| (*id, record))
    }
}

/// 🧵️ A resolved `AsyncTask`, erased to bytes — see `spawn_task`'s doc for why no `Mutation`/
/// `ConfigMutation`/`DraftMutation` generic ever reaches this type. Mirrors `TaskResolution`
/// one-for-one except `Emit` is pre-encoded (the SAME `protocol::encode_ops_vec`/`OpBinary::
/// encode_op` idiom `dispatch_emit`'s own `last_emit_wire` uses) and a task future that resolved
/// `Err(fault)` gets its own variant (never silently dropped).
#[doc(hidden)]
pub enum TaskResumeOutcome {
    Command(Vec<u8>),
    Emit { artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8> },
    Fault(semio_framework::Fault),
}

/// 🧵️ One entry in `TASK_RESUMES` — `meta` is the task's CLONED originating `ActionMeta` (spawn
/// time for a real task resolution; required checkpointed `instance_actor(instance)` at restore time for a
/// `task_restarts` replay, since a checkpoint restart has no "spawn" of its own to snapshot from —
/// see `restore_now`), preserved so the follow-up dispatch stays attributed to the actor that
/// asked for it even if a different one is active by the time it resolves.
#[doc(hidden)]
pub struct PendingResume {
    pub instance: u32,
    pub meta: semio_framework_plugin_decl_emission::ActionMeta,
    pub outcome: TaskResumeOutcome,
}

const REACTOR_RESUME_SLOTS: usize = 1_024;
const REACTOR_RESUME_BYTES: usize = 3 * 64 * 1_024;
const REACTOR_RESUME_ACTOR_BYTES: usize = 256;

impl PendingResume {
    pub fn admitted_bytes(&self) -> Option<usize> {
        let mut bytes = self.meta.actor.len();
        if bytes > REACTOR_RESUME_ACTOR_BYTES {
            return None;
        }
        match &self.outcome {
            TaskResumeOutcome::Command(command) => bytes = bytes.checked_add(command.len())?,
            TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => {
                bytes = bytes.checked_add(artifact_ops.len())?.checked_add(config_ops.len())?.checked_add(draft_ops.len())?;
            }
            TaskResumeOutcome::Fault(fault) => {
                if fault.causes.len() > 16 {
                    return None;
                }
                bytes = bytes.checked_add(fault.code.0.len())?.checked_add(fault.message.len())?;
                for value in [&fault.scope.plugin_id, &fault.scope.app_id, &fault.scope.instance_id, &fault.scope.module, &fault.scope.body_key].into_iter().flatten() {
                    bytes = bytes.checked_add(value.len())?;
                }
                for cause in &fault.causes {
                    bytes = bytes.checked_add(cause.message.len())?;
                    if let Some(code) = &cause.code {
                        bytes = bytes.checked_add(code.0.len())?;
                    }
                }
            }
        }
        (bytes <= REACTOR_RESUME_BYTES).then_some(bytes)
    }
}

#[doc(hidden)]
pub struct FixedResumeQueue {
    pub entries: std::mem::ManuallyDrop<VecDeque<PendingResume>>,
    pub allocation_admitted: bool,
}

impl FixedResumeQueue {
    pub fn new() -> Self {
        let mut entries = VecDeque::new();
        let allocation_admitted = entries.try_reserve_exact(REACTOR_RESUME_SLOTS).is_ok();
        Self { entries: std::mem::ManuallyDrop::new(entries), allocation_admitted }
    }

    #[expect(clippy::result_large_err, reason = "The fixed resume queue returns the admitted view snapshot, task metadata, and payload owner intact without allocating on rejection.")]
    pub fn push(&mut self, value: PendingResume) -> Result<(), PendingResume> {
        if !self.allocation_admitted || self.entries.len() >= REACTOR_RESUME_SLOTS || value.admitted_bytes().is_none() {
            return Err(value);
        }
        self.entries.push_back(value);
        Ok(())
    }

    pub fn push_admitted(&mut self, value: PendingResume) {
        debug_assert!(self.allocation_admitted && self.entries.len() < REACTOR_RESUME_SLOTS && value.admitted_bytes().is_some());
        self.entries.push_back(value);
    }

    pub fn pop(&mut self) -> Option<PendingResume> {
        self.entries.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn begin_cancel_instance(&self) -> usize {
        self.entries.len()
    }

    pub fn cancel_instance_step(&mut self, instance: u32, remaining: &mut usize) -> bool {
        if *remaining == 0 || self.entries.is_empty() {
            *remaining = 0;
            return true;
        }
        let Some(entry) = self.entries.pop_front() else {
            *remaining = 0;
            return true;
        };
        *remaining -= 1;
        if entry.instance != instance {
            self.push_admitted(entry);
        }
        *remaining == 0
    }
}

impl Drop for FixedResumeQueue {
    fn drop(&mut self) {}
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the outstanding-task quota for
/// `instance` — `QuotaSchema.outstanding_requests`, defaulting to 16 when the instance never
/// declared one (or hasn't opened yet, which should not happen in practice: `spawn_task` is only
/// ever reachable from `dispatch_emit`, itself only reachable after `Event::InstanceOpen`).
fn instance_task_quota(instance: u32) -> u64 {
    INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).and_then(|entry| entry.quota.outstanding_requests)).unwrap_or(16)
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): spawns `task` onto this actor's
/// shared `TASK_EXECUTOR`, quota-gated then key-deduped (in that order — a same-key respawn at
/// exactly the quota limit legitimately fails; the caller may retry once the cancelled slot is
/// actually freed on a later turn). Called from `🔌️plugin/🦀️.rs`'s
/// `publish_mounted_typed_operation_unit`, the migrated publication ladder's own task lane, right
/// after a gesture's mutation lanes land — `M`/`C`/`D` are that call's concrete `A::Mutation`/
/// `A::ConfigMutation`/`A::DraftMutation`, monomorphized per app. The moment the task's future
/// resolves, its `TaskResolution` is erased to bytes (`TaskResumeOutcome`, the SAME
/// `last_emit_wire` wire idiom `dispatch_emit` itself uses for its OWN mutation lanes) and queued
/// on `TASK_RESUMES` — no `M`/`C`/`D` generic ever crosses into the executor or the resume queue,
/// which is what lets ALL of this actor's apps (each with its own concrete `A`) share ONE
/// `LocalExecutor`/`TASK_RESUMES` pair.
///
/// 🔑️ Latest-wins dedupe: a task spawned with the same `(instance, key)` as one still live
/// cancels the live one FIRST — its future (and anything it owns, including a parked
/// `RequestFuture`) is dropped without ever completing, so no resume is ever queued for it.
#[doc(hidden)]
pub fn spawn_task<M, C, D>(instance: u32, meta: &semio_framework_plugin_decl_emission::ActionMeta, task: semio_framework_plugin_decl_emission::AsyncTask<M, C, D>) -> Result<(), semio_framework::Fault>
where
    M: ::protocol::OpBinary + 'static,
    C: ::protocol::OpBinary + 'static,
    D: ::protocol::OpBinary + 'static,
{
    let quota = instance_task_quota(instance);
    let live = TASK_RECORDS.with(|records| records.borrow().count_instance(instance) as u64);
    if live >= quota {
        let fault = semio_framework::Fault::new(
            semio_framework::FaultOrigin::Plugin,
            semio_framework::FaultCode::new("plugin.task.quota-exceeded"),
            format!("instance {instance} already has {live} outstanding task(s) (quota {quota}); `{}` was not spawned", task.label),
        );
        task.dispose_cold();
        return Err(fault);
    }

    if task.label.len() > REACTOR_TASK_LABEL_BYTES || task.restart.as_ref().is_some_and(|bytes| bytes.len() > REACTOR_TASK_RESTART_BYTES) {
        task.dispose_cold();
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.task-authority-too-large"), "task label or restart authority exceeds its fixed admitted byte bound"));
    }
    if task.key.as_ref().is_some_and(|key| key.len() > REACTOR_TASK_KEY_BYTES) {
        task.dispose_cold();
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.task.key-too-large"), format!("task key exceeds {REACTOR_TASK_KEY_BYTES} bytes")));
    }
    let (_label, key, restart, run) = task.into_parts();

    if let Some(key) = &key {
        if let Some(previous) = TASK_RECORDS.with(|records| records.borrow().find_key(instance, key)) {
            let detached = TASK_EXECUTOR.with(|executor| executor.detach(previous));
            let record = TASK_RECORDS.with(|records| records.borrow_mut().remove(previous));
            if detached.is_none() && record.is_none() {
                semio_framework_plugin_decl_emission::dispose_cold_task_run(run);
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.task.supersession-pending"), "keyed task supersession found no exact previous owner to dispose"));
            }
            drop(record);
            drop(detached);
        }
    }

    let reservation = match TASK_EXECUTOR.with(|executor| executor.reserve()) {
        Ok(reservation) => reservation,
        Err(message) => {
            semio_framework_plugin_decl_emission::dispose_cold_task_run(run);
            return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.task.executor-capacity"), message));
        }
    };
    let task_id = reservation.id();
    if !TASK_RECORDS.with(|records| records.borrow().can_insert(task_id)) {
        semio_framework_plugin_decl_emission::dispose_cold_task_run(run);
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.task.record-capacity"), "fixed task record authority rejected an executor-reserved direct slot"));
    }
    TASK_RECORDS.with(|records| records.borrow_mut().insert_admitted(task_id, TaskRecord { instance, key, restart }));

    let ctx = semio_framework_plugin_decl_emission::TaskCtx { host: host_for_instance(instance), meta: meta.clone() };
    let resume_instance = instance;
    let resume_meta = meta.clone();
    let mut cold_run = semio_framework_plugin_decl_emission::ColdTaskRun::new(run);
    reservation.install(Box::pin(async move {
        let future = cold_run.take().run(ctx);
        let outcome = match future.await {
            Ok(semio_framework_plugin_decl_emission::TaskResolution::Command(bytes)) => Some(TaskResumeOutcome::Command(bytes)),
            Ok(semio_framework_plugin_decl_emission::TaskResolution::Emit(emit)) => {
                Some(TaskResumeOutcome::Emit { artifact_ops: encode_mutation_lane(&emit.artifact_mutations).await, config_ops: encode_mutation_lane(&emit.config_mutations).await, draft_ops: encode_mutation_lane(&emit.draft_mutations).await })
            }
            Ok(semio_framework_plugin_decl_emission::TaskResolution::Done) => None,
            Err(fault) => Some(TaskResumeOutcome::Fault(fault)),
        };
        TASK_RECORDS.with(|records| drop(records.borrow_mut().remove(task_id)));
        if let Some(outcome) = outcome {
            assert!(TASK_RESUMES.with(|resumes| resumes.borrow_mut().push(PendingResume { instance: resume_instance, meta: resume_meta, outcome })).is_ok(), "fixed task-resume authority is saturated");
        }
    }));

    Ok(())
}

/// 🔀️ The exact wire shape `dispatch_emit`'s own `last_emit_wire` uses for one mutation lane —
/// factored out so `spawn_task`'s `TaskResolution::Emit` erasure and `dispatch_emit` stay
/// byte-identical without one calling the other across the crate's plugin/reactor split.
async fn encode_mutation_lane<T: ::protocol::OpBinary>(ops: &[T]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(ops.len());
    for op in ops.iter() {
        encoded.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
    }
    protocol::encode_ops_vec(&encoded)
}

/// 🚫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `Event::InstanceClose`
/// cancellation — drops every task `instance` owns from `EXECUTOR` (dropping its future, and
/// everything IT owns including any parked `RequestFuture`) and clears the key-dedupe index.
/// Deliberately does NOT touch `REGISTRY`/`RequestRegistry::cancel_instance` — that is a SEPARATE
/// step the caller (`poll`'s `Event::InstanceClose` handling) runs right after this one, since a
/// task's own `RequestFuture` is already gone the moment its owning future is dropped here; the
/// registry sweep is defense-in-depth for a pending request whose task somehow isn't tracked here
/// (there should be none, by construction — every `RequestFuture` is created inside `TaskCtx.host`,
/// itself only ever handed to a task by `spawn_task`).
///
/// 🪞️ Both halves DROP, and one call runs exactly one unit of one of them. The cursor walks the
/// `TASK_RECORDS`/`TASK_EXECUTOR` half first — an `AsyncTask` spawned by `spawn_task` lives there
/// on every build since the task lane became a product lane — and then continues, offset by
/// [`REACTOR_TASK_SLOTS`], into `ReactorExecutor::close_instance_step`, which owns the job/reducer
/// tasks. An instance that spawned no `AsyncTask` skips the whole first half in one unit, so the
/// close ladder of the common case costs exactly what it cost before. Neither half ever polls a
/// cancelled task: a cancelled body must never observe its await resolving, which is also what
/// `RequestRegistry::begin_cancel_instance`'s doc relies on when it retires slots with no wake.
// 🚫️async: E1 pure in-memory sweep over `TASK_RECORDS`/`EXECUTOR`/`TASK_KEYS` (all sync now,
// R9) consumed by `poll`'s sync `world actor` boundary — zero suspension.
#[doc(hidden)]
pub fn cancel_instance_tasks_step(instance: u32, cursor: &mut usize) -> bool {
    if *cursor < REACTOR_TASK_SLOTS {
        if TASK_RECORDS.with(|records| records.borrow().count_instance(instance)) == 0 {
            *cursor = REACTOR_TASK_SLOTS;
        } else {
            let entry = TASK_RECORDS.with(|records| records.borrow().entry_at(*cursor).and_then(|(id, record)| (record.instance == instance).then_some(id)));
            if let Some(id) = entry {
                let detached = TASK_EXECUTOR.with(|executor| executor.detach(id));
                let record = TASK_RECORDS.with(|records| records.borrow_mut().remove(id));
                drop(record);
                drop(detached);
            }
            *cursor += 1;
            return false;
        }
    }
    let mut reactor_cursor = *cursor - REACTOR_TASK_SLOTS;
    let budget = executor::ReactorTaskBudget { operation: 0, generation: 0, cancellation_generation: 0, maximum_units: 1, maximum_bytes: 4_096, deadline: std::time::Instant::now() + std::time::Duration::from_millis(8) };
    let step = REACTOR_EXECUTOR.with(|executor| executor.close_instance_step(instance, &mut reactor_cursor, budget));
    *cursor = REACTOR_TASK_SLOTS + reactor_cursor;
    matches!(step, executor::ReactorTaskStep::Complete)
}

#[doc(hidden)]
#[cfg(any(test, feature = "artifact-app-testing"))]
pub fn cancel_instance_tasks(instance: u32) {
    let mut cursor = 0;
    for _ in 0..REACTOR_TASK_SLOTS {
        let before = cursor;
        if cancel_instance_tasks_step(instance, &mut cursor) || cursor == before {
            break;
        }
    }
}
