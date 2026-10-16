//! ⏲️ Task spawn, records, resumes and extension invocation queue of the plugin reactor.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[path = "🧮️fixed-slots/🦀️.rs"]
pub mod tasks_fixed_slots;
#[path = "📇️instance-metadata/🦀️.rs"]
pub mod tasks_instance_metadata;
#[path = "🧵️task-lane/🦀️.rs"]
pub mod tasks_task_lane;
#[path = "🔗️extension-continuation/🦀️.rs"]
pub mod tasks_extension_continuation;
#[path = "🧷️render-bindings/🦀️.rs"]
pub mod tasks_render_bindings;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
use std::cell::RefCell;
use semio_framework_plugin_actor_host::{executor, requests};
use tasks_instance_metadata::{InstanceMetadataRegistry};
use tasks_render_bindings::{JobRenderBindingRegistry};
use tasks_task_lane::{FixedResumeQueue, TaskRecordRegistry};

semio_framework_plugin_abi::component_persistent_local! {
    /// 📮️ One `RequestRegistry` per actor (today: shared process-wide, matching the "one actor per
    /// app instance is the default" granularity design-abi.md §4 names — a multi-instance pooled
    /// actor is opt-in first-party-only future work, out of this wave).
    pub static REGISTRY: requests::RequestRegistry = requests::RequestRegistry::new();
    pub static REACTOR_EXECUTOR: executor::ReactorExecutor = executor::ReactorExecutor::new();
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the actor-local executor
    /// every `AsyncTask` `spawn_task` admits runs on — the COLD half of this actor's two
    /// executors, for futures that park on a `RequestFuture` across host turns rather than
    /// burning a bounded work budget like `REACTOR_EXECUTOR`'s reducer/job tasks do. Driven once
    /// per turn by `turn::poll`, woken by `Event::Timer` and by `RequestRegistry::resolve`.
    pub static TASK_EXECUTOR: executor::ColdFutureExecutor = executor::ColdFutureExecutor::new();
    /// 🪪️ Every instance this actor currently has open — `(id, app_id)`, in `InstanceOpen` order.
    /// Used by `📸️checkpoint`.
    pub static INSTANCE_METADATA: RefCell<InstanceMetadataRegistry> = RefCell::new(InstanceMetadataRegistry::new());
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): every `AsyncTask` this actor
    /// currently has spawned on `EXECUTOR`, keyed by its `executor::TaskId` slot — the bookkeeping
    /// `spawn_task`'s quota gate counts against, `cancel_instance_tasks` (`Event::InstanceClose`)
    /// walks, and `checkpoint_now` reads `restart` out of.
    pub static TASK_RECORDS: RefCell<TaskRecordRegistry> = RefCell::new(TaskRecordRegistry::new());
    /// 📬️ Resolved-but-not-yet-redispatched `AsyncTask` follow-ups, type-erased to bytes at
    /// resolution time (see `spawn_task`'s doc) — drained by `drain_task_resumes`, called from
    /// `poll` right after `run_until_idle`. Also the target of a checkpoint `restore`'s replayed
    /// `task_restarts` (`restore_now`), so a restart is just an ordinary `Command` resume queued
    /// one call earlier than usual.
    pub static TASK_RESUMES: RefCell<FixedResumeQueue> = RefCell::new(FixedResumeQueue::new());
    pub static JOB_RENDER_BINDINGS: RefCell<JobRenderBindingRegistry> = RefCell::new(JobRenderBindingRegistry::new());
}
