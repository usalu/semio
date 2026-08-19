//! ⚛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4): the turn loop —
//! `reactor::poll`'s real implementation. Ties together `🧵️executor` (async task scheduling),
//! `📮️requests` (the host-effect request/completion registry), `🩹️patches` (revisioned UI diffing),
//! `💼️jobs` (the absorbed `semio.io-run`/`semio.io-sniff` cold job kinds), `📸️checkpoint`, and
//! `🌐host` (the async `host::*` API surface plugin/extension code awaits).
//!
//! Converts between the WIT-generated `semio::framework::{effects,events,ui,reactor}::*` types
//! (crossing the component boundary) and the Rust SSOT `semio_framework::kernel::{Effect, Event,
//! UiPatch, PatchOp, TurnResult, TurnStatus, Budget}` (packet A3, landed in
//! `🎠️kernel/🦀️component.rs` while this packet was in flight). `app-command` events route through
//! the EXISTING `plugin_runtime::plugin_exchange` dispatcher unchanged (design-abi.md §4) — this
//! module never reimplements command dispatch, only translates its `AppFrame` output into
//! `Effect`/`UiPatch`.

#[path = "🧵️executor/🦀️component.rs"]
pub mod executor;
#[path = "📮️requests/🦀️component.rs"]
pub mod requests;
#[path = "🩹️patches/🦀️component.rs"]
pub mod patches;
#[path = "💼️jobs/🦀️component.rs"]
pub mod jobs;
#[path = "📸️checkpoint/🦀️component.rs"]
pub mod checkpoint;

// 🧬️ Only `wit_bridge` below (component-guest/-extension-guest wasm32-wasip2) consumes these —
// a plain native build never reaches the WIT-boundary translation code, so unlike `RefCell` these
// two must be gated identically to `wit_bridge` itself or they warn as unused on native.
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::{Effect, Event, MessageEndpoint, PatchOp, RequestOutcome, TurnStatus, UiPatch};
use std::cell::RefCell;
// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: `HashMap`/`VecDeque` back `TASK_RECORDS`/
// `TASK_KEYS`/`TASK_RESUMES`/`INSTANCE_QUOTAS` below, which are deliberately UNGATED (native
// `dispatch_emit` tests spawn tasks too — see `🔌️plugin/🦀️component.rs`'s `spawn_task` call site),
// unlike the wasm-only import above.
use std::collections::{HashMap, VecDeque};

thread_local! {
    /// 🩹️ One `PatchTracker` shared by every instance this actor hosts (surfaces are already
    /// namespaced by their own `surface` string, which today embeds the instance — see
    /// `render_surface`'s key).
    static PATCHES: patches::PatchTracker = patches::PatchTracker::new();
    /// 📮️ One `RequestRegistry` per actor (today: shared process-wide, matching the "one actor per
    /// app instance is the default" granularity design-abi.md §4 names — a multi-instance pooled
    /// actor is opt-in first-party-only future work, out of this wave).
    static REGISTRY: requests::RequestRegistry = requests::RequestRegistry::new();
    static EXECUTOR: executor::LocalExecutor = executor::LocalExecutor::new();
    /// 🪪️ Every instance this actor currently has open — `(id, app_id)`, in `InstanceOpen` order.
    /// Used by `📸️checkpoint`.
    static OPEN_INSTANCES: RefCell<Vec<(u32, String)>> = const { RefCell::new(Vec::new()) };
    /// ⏱️ Live timer ids this actor has armed via `Effect::SetTimer`, carried into the checkpoint
    /// pack (design-abi.md §4).
    static ARMED_TIMERS: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): every `AsyncTask` this actor
    /// currently has spawned on `EXECUTOR`, keyed by its `executor::TaskId` slot — the bookkeeping
    /// `spawn_task`'s quota gate counts against, `cancel_instance_tasks` (`Event::InstanceClose`)
    /// walks, and `checkpoint_now` reads `restart` out of.
    static TASK_RECORDS: RefCell<HashMap<executor::TaskId, TaskRecord>> = RefCell::new(HashMap::new());
    /// 🔑️ Latest-wins dedupe index: `(instance, key)` → the live `TaskId` spawned under that key,
    /// for `AsyncTask::keyed(..)`. Absent unless a task actually declared a key.
    static TASK_KEYS: RefCell<HashMap<(u32, String), executor::TaskId>> = RefCell::new(HashMap::new());
    /// 📬️ Resolved-but-not-yet-redispatched `AsyncTask` follow-ups, type-erased to bytes at
    /// resolution time (see `spawn_task`'s doc) — drained by `drain_task_resumes`, called from
    /// `poll` right after `run_until_idle`. Also the target of a checkpoint `restore`'s replayed
    /// `task_restarts` (`restore_now`), so a restart is just an ordinary `Command` resume queued
    /// one call earlier than usual.
    static TASK_RESUMES: RefCell<VecDeque<PendingResume>> = RefCell::new(VecDeque::new());
    /// 🎛️ Per-instance `QuotaSchema`, real values decoded off `Event::InstanceOpen.quotas`
    /// (previously always defaulted — see `wit_event_to_kernel`'s `InstanceOpen` arm) and dropped
    /// again on `Event::InstanceClose`. `spawn_task`'s quota gate is the first real reader.
    static INSTANCE_QUOTAS: RefCell<HashMap<u32, semio_framework::kernel::QuotaSchema>> = RefCell::new(HashMap::new());
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): one `AsyncTask` this actor's
/// `LocalExecutor` currently owns, keyed by its `executor::TaskId` slot. `label` is diagnostic-only
/// today (no telemetry/checkpoint surface reads it back yet — an honest, named gap, not a silent
/// unused field: kept because a task with no human-readable name is a debugging dead end the
/// moment more than one is ever live on an instance).
struct TaskRecord {
    instance: u32,
    key: Option<String>,
    #[allow(dead_code)]
    label: String,
    restart: Option<Vec<u8>>,
}

/// 🧵️ A resolved `AsyncTask`, erased to bytes — see `spawn_task`'s doc for why no `Mutation`/
/// `ConfigMutation`/`DraftMutation` generic ever reaches this type. Mirrors `TaskResolution`
/// one-for-one except `Emit` is pre-encoded (the SAME `protocol::encode_ops_vec`/`OpBinary::
/// encode_op` idiom `dispatch_emit`'s own `last_emit_wire` uses) and a task future that resolved
/// `Err(fault)` gets its own variant (never silently dropped).
enum TaskResumeOutcome {
    Command(Vec<u8>),
    Emit { artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8> },
    Fault(semio_framework::Fault),
}

/// 🧵️ One entry in `TASK_RESUMES` — `meta` is the task's CLONED originating `ActionMeta` (spawn
/// time for a real task resolution; best-effort `instance_actor(instance)` at restore time for a
/// `task_restarts` replay, since a checkpoint restart has no "spawn" of its own to snapshot from —
/// see `restore_now`), preserved so the follow-up dispatch stays attributed to the actor that
/// asked for it even if a different one is active by the time it resolves.
struct PendingResume {
    instance: u32,
    meta: crate::app::ActionMeta,
    outcome: TaskResumeOutcome,
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the outstanding-task quota for
/// `instance` — `QuotaSchema.outstanding_requests`, defaulting to 16 when the instance never
/// declared one (or hasn't opened yet, which should not happen in practice: `spawn_task` is only
/// ever reachable from `dispatch_emit`, itself only reachable after `Event::InstanceOpen`).
async fn instance_task_quota(instance: u32) -> u64 {
    INSTANCE_QUOTAS.with(|quotas| quotas.borrow().get(&instance).and_then(|schema| schema.outstanding_requests)).unwrap_or(16)
}

/// 🌐️ Every `host::Host` handle vended to plugin/extension code shares this actor's one
/// `RequestRegistry`, scoped to `instance` (`RequestRegistry::for_instance` — design-abi.md §4's
/// per-request instance tagging, so `Event::InstanceClose` can cancel exactly this instance's
/// pending host round-trips and no other's) — see `host::Host::new`.
pub async fn host_for_instance(instance: u32) -> crate::host::Host {
    REGISTRY.with(|registry| crate::host::Host::new(registry.for_instance(instance)))
}

/// 🌐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the instance-agnostic sibling of
/// `host_for_instance` — a `Host` scoped to instance 0, the SAME "no instance tag declared" default
/// `RequestRegistry::for_instance`'s own doc names. `⚛️reactor/💼️jobs/🦀️component.rs::spawn_job`
/// calls this (as `crate::reactor::host()`, zero args): a job is actor-global, not tied to one open
/// instance the way an `AsyncTask` is, so it has no `instance: u32` to scope by in the first place.
pub async fn host() -> crate::host::Host {
    REGISTRY.with(|registry| crate::host::Host::new(registry.clone()))
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): spawns `task` onto this actor's
/// shared `LocalExecutor`, quota-gated then key-deduped (in that order — a same-key respawn at
/// exactly the quota limit legitimately fails; the caller may retry once the cancelled slot is
/// actually freed on a later turn). Called from `🔌️plugin/🦀️component.rs`'s `dispatch_emit`, right
/// after a gesture's mutation lanes land — `M`/`C`/`D` are that call's concrete `A::Mutation`/
/// `A::ConfigMutation`/`A::DraftMutation`, monomorphized per app. The moment the task's future
/// resolves, its `TaskResolution` is erased to bytes (`TaskResumeOutcome`, the SAME
/// `last_emit_wire` wire idiom `dispatch_emit` itself uses for its OWN mutation lanes) and queued
/// on `TASK_RESUMES` — no `M`/`C`/`D` generic ever crosses into the executor or the resume queue,
/// which is what lets ALL of this actor's apps (each with its own concrete `A`) share ONE
/// `LocalExecutor`/`TASK_RESUMES` pair.
pub async fn spawn_task<M, C, D>(instance: u32, meta: &crate::app::ActionMeta, task: crate::app::AsyncTask<M, C, D>) -> Result<(), semio_framework::Fault>
where
    M: ::protocol::OpBinary + 'static,
    C: ::protocol::OpBinary + 'static,
    D: ::protocol::OpBinary + 'static,
{
    let quota = instance_task_quota(instance);
    let live = TASK_RECORDS.with(|records| records.borrow().values().filter(|record| record.instance == instance).count() as u64);
    if live >= quota {
        return Err(semio_framework::Fault::new(
            semio_framework::FaultOrigin::Plugin,
            semio_framework::FaultCode::new("plugin.task.quota-exceeded"),
            format!("instance {instance} already has {live} outstanding task(s) (quota {quota}); `{}` was not spawned", task.label),
        ));
    }

    let (label, key, restart, run) = task.into_parts();

    // 🔑️ Latest-wins dedupe: a task spawned with the same `(instance, key)` as one still live
    // cancels the live one FIRST — its future (and anything it owns, including a parked
    // `RequestFuture`) is dropped without ever completing, so no resume is ever queued for it.
    if let Some(key) = &key {
        if let Some(old_id) = TASK_KEYS.with(|keys| keys.borrow().get(&(instance, key.clone())).copied()) {
            EXECUTOR.with(|executor| executor.cancel(old_id));
            TASK_RECORDS.with(|records| {
                records.borrow_mut().remove(&old_id);
            });
        }
    }

    let ctx = crate::app::TaskCtx { host: host_for_instance(instance), meta: meta.clone() };
    let future = run(ctx);
    let resume_instance = instance;
    let resume_meta = meta.clone();
    let dedupe_key = key.clone();

    let task_id = EXECUTOR.with(|executor| {
        executor.spawn_with_id(move |id| {
            Box::pin(async move {
                let outcome = match future.await {
                    Ok(crate::app::TaskResolution::Command(bytes)) => Some(TaskResumeOutcome::Command(bytes)),
                    Ok(crate::app::TaskResolution::Emit(emit)) => Some(TaskResumeOutcome::Emit {
                        artifact_ops: encode_mutation_lane(&emit.artifact_mutations),
                        config_ops: encode_mutation_lane(&emit.config_mutations),
                        draft_ops: encode_mutation_lane(&emit.draft_mutations),
                    }),
                    Ok(crate::app::TaskResolution::Done) => None,
                    Err(fault) => Some(TaskResumeOutcome::Fault(fault)),
                };
                TASK_RECORDS.with(|records| {
                    records.borrow_mut().remove(&id);
                });
                if let Some(key) = &dedupe_key {
                    TASK_KEYS.with(|keys| {
                        let mut keys = keys.borrow_mut();
                        if keys.get(&(resume_instance, key.clone())) == Some(&id) {
                            keys.remove(&(resume_instance, key.clone()));
                        }
                    });
                }
                if let Some(outcome) = outcome {
                    TASK_RESUMES.with(|resumes| resumes.borrow_mut().push_back(PendingResume { instance: resume_instance, meta: resume_meta, outcome }));
                }
            })
        })
    });

    TASK_RECORDS.with(|records| {
        records.borrow_mut().insert(task_id, TaskRecord { instance, key: key.clone(), label, restart });
    });
    if let Some(key) = key {
        TASK_KEYS.with(|keys| {
            keys.borrow_mut().insert((instance, key), task_id);
        });
    }

    Ok(())
}

/// 🔀️ The exact wire shape `dispatch_emit`'s own `last_emit_wire` uses for one mutation lane —
/// factored out so `spawn_task`'s `TaskResolution::Emit` erasure and `dispatch_emit` stay
/// byte-identical without one calling the other across the crate's plugin/reactor split.
async fn encode_mutation_lane<T: ::protocol::OpBinary>(ops: &[T]) -> Vec<u8> {
    protocol::encode_ops_vec(&ops.iter().map(|op| ::protocol::OpBinary::encode_op(op).unwrap_or_default()).collect::<Vec<_>>())
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
pub(crate) async fn cancel_instance_tasks(instance: u32) {
    let ids: Vec<executor::TaskId> = TASK_RECORDS.with(|records| records.borrow().iter().filter(|(_, record)| record.instance == instance).map(|(id, _)| *id).collect());
    for id in ids {
        EXECUTOR.with(|executor| executor.cancel(id));
        let removed_key = TASK_RECORDS.with(|records| records.borrow_mut().remove(&id).and_then(|record| record.key));
        if let Some(key) = removed_key {
            TASK_KEYS.with(|keys| {
                keys.borrow_mut().remove(&(instance, key));
            });
        }
    }
}

/// 📸️ `checkpoint::checkpoint` body — unconditional (no WIT type in its signature, only
/// `Vec<u8>`/kernel types), unlike `poll`/the `wit_*`/`kernel_*_to_wit` bridge below.
pub async fn checkpoint_now() -> Result<Vec<u8>, semio_framework::Fault> {
    let instances = OPEN_INSTANCES.with(|open| open.borrow().clone());
    let timers = ARMED_TIMERS.with(|timers| timers.borrow().clone());
    let pending = REGISTRY.with(|registry| registry.pending_ids().into_iter().map(|id| id.0).collect());
    // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): the task itself is never
    // serialized (`TASK_RECORDS`/`EXECUTOR` are process memory, not pack state) — only the
    // `restart` command bytes of every LIVE task that declared one via `.restartable(..)` survive
    // into the pack, one `TaskRestart{instance, command}` per such task.
    let task_restarts: Vec<checkpoint::TaskRestart> = TASK_RECORDS.with(|records| {
        records
            .borrow()
            .values()
            .filter_map(|record| record.restart.as_ref().map(|command| checkpoint::TaskRestart { instance: record.instance, command: command.clone() }))
            .collect()
    });
    checkpoint::checkpoint(&instances, timers, pending, task_restarts)
}

/// 📸️ `checkpoint::restore` body — re-arms the timer list from the restored pack;
/// `pending_requests` are intentionally NOT re-parked (design-abi.md §4: async tasks are marked
/// re-run-on-restore, not resumed as though the host round-trip were still in flight).
/// `task_restarts` ARE re-dispatched, though not synchronously here: each one is queued onto
/// `TASK_RESUMES` as an ordinary `Command` resume (the SAME resume path a live task's own
/// `TaskResolution::Command` takes), drained by the first `poll` after restore — restoring is a
/// pure state-load, it must not itself re-enter app dispatch.
pub async fn restore_now(state: &[u8]) -> Result<(), semio_framework::Fault> {
    let pack = checkpoint::restore(state)?;
    OPEN_INSTANCES.with(|open| {
        *open.borrow_mut() = pack.instances();
    });
    ARMED_TIMERS.with(|timers| {
        *timers.borrow_mut() = pack.timers().to_vec();
    });
    TASK_RESUMES.with(|resumes| {
        let mut resumes = resumes.borrow_mut();
        for restart in pack.task_restarts() {
            let meta = crate::app::ActionMeta { actor: crate::plugin_runtime::instance_actor(restart.instance), instance_id: restart.instance };
            resumes.push_back(PendingResume { instance: restart.instance, meta, outcome: TaskResumeOutcome::Command(restart.command.clone()) });
        }
    });
    Ok(())
}

/// 🧬️ Everything below crosses the wasm component boundary — gated identically to `component`
/// (`🦀️component.rs` at crate root) since it names `crate::component::component::exports::...` types that
/// simply do not exist outside a `component-guest`/`component-extension-guest` wasm32-wasip2
/// build (mirrors the OLD `host_port`'s per-function `#[cfg(...)]` pattern, just hoisted to one
/// module instead of repeated per function).
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub use wit_bridge::poll;
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): re-exported so the (future)
/// `world actor-async` runner can call the SAME `drain_task_resumes` `poll` uses — one
/// implementation of "how a resumed `AsyncTask` re-enters the reducer", not two.
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub use wit_bridge::drain_task_resumes;

#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
mod wit_bridge {
    use super::*;

    /// 🧭️ `reactor`/`jobs`/`checkpoint`/`describe` are the only interfaces `world actor` directly
    /// `export`s, so wit-bindgen only aliases THEIR top-level types under `exports::…`. `effects`/
    /// `events`/`ui`/`types` are merely `use`d by `reactor.wit` (design-abi.md §1/§4) — their own
    /// payload records live at the plain (non-`exports::`) path alongside the `pure` import, one
    /// level down from where the nesting stops being re-aliased. Verified empirically: a
    /// deliberately wrong `wit::OpenWindowEffect` import made `cargo check --target wasm32-wasip2
    /// --features component-guest` emit `help: consider importing … effects::OpenWindowEffect`
    /// (and the `events`/`ui` siblings the same way) — not guessed.
    use crate::component::component::semio::framework::effects as wit_effects;
    use crate::component::component::semio::framework::events as wit_events;
    use crate::component::component::semio::framework::types as wit_types;
    use crate::component::component::semio::framework::ui as wit_ui;
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: the raw WIT event, aliased for `poll`'s
    /// `Event::InstanceClose` handling below — the KERNEL `Event::InstanceClose` (SSOT, `🎠️kernel`,
    /// not this packet's file) carries no instance id, so the raw payload must be read BEFORE
    /// `wit_event_to_kernel` erases it.
    use crate::component::component::exports::semio::framework::reactor::Event as WitReactorEvent;

/// ▶️ The real `reactor::poll` body — see module doc for the shape. `events`/`budget` are the
/// WIT-generated types from `exports::semio::framework::reactor`; the return is that same
/// module's `TurnResult`.
pub async fn poll(events: Vec<crate::component::component::exports::semio::framework::reactor::Event>, budget: crate::component::component::exports::semio::framework::reactor::Budget) -> Result<crate::component::component::exports::semio::framework::reactor::TurnResult, semio_framework::Fault> {
    let mut app_commands: HashMap<u32, Vec<Vec<u8>>> = HashMap::new();
    let mut dirty_render: Vec<(u32, String)> = Vec::new();

    for event in events {
        // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `Event::InstanceClose`
        // cancellation happens here, off the RAW wit payload's `instance` id, before `event` is
        // consumed by `wit_event_to_kernel` below (which cannot carry it — see the `use` doc
        // above). Tasks are cancelled BEFORE the registry sweep so a task's own parked
        // `RequestFuture` is already gone by the time `cancel_instance` runs (that call is then
        // defense-in-depth, not the primary cleanup — see its own doc).
        if let WitReactorEvent::InstanceClose(ref payload) = event {
            let numeric_instance = payload.instance;
            cancel_instance_tasks(numeric_instance);
            REGISTRY.with(|registry| registry.cancel_instance(numeric_instance));
            INSTANCE_QUOTAS.with(|quotas| {
                quotas.borrow_mut().remove(&numeric_instance);
            });
        }
        match wit_event_to_kernel(event) {
            Event::InstanceOpen { instance, app_id, actor, quotas, .. } => {
                let numeric_instance = instance.0.parse::<u32>().unwrap_or(0);
                let _ = crate::plugin_runtime::plugin_create_app_with_id(numeric_instance, &app_id.0);
                // 🪪️ Channel v12 (A4) retired the `AppCommand::Hello` handshake that used to record
                // this — lifecycle now arrives here as `Event::InstanceOpen` (design-abi.md §4).
                crate::plugin_runtime::set_instance_actor(numeric_instance, actor);
                OPEN_INSTANCES.with(|open| open.borrow_mut().push((numeric_instance, app_id.0)));
                // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): `spawn_task`'s
                // quota gate is the first real reader — `quotas` used to be silently discarded here
                // (`wit_event_to_kernel`'s `InstanceOpen` arm always built a `default()`).
                INSTANCE_QUOTAS.with(|store| {
                    store.borrow_mut().insert(numeric_instance, quotas);
                });
            }
            Event::InstanceClose => {}
            Event::AppCommandEvent { instance, command, .. } => {
                app_commands.entry(instance.0.parse::<u32>().unwrap_or(0)).or_default().push(command);
            }
            Event::SurfaceVisible { surface } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    dirty_render.push((instance, surface));
                }
            }
            Event::SurfaceHidden { .. } | Event::SurfaceResized { .. } => {}
            Event::PatchAck { surface, revision } => {
                PATCHES.with(|patches| patches.mark_ack(&surface, revision));
            }
            Event::PatchRejected { surface, .. } => {
                PATCHES.with(|patches| patches.mark_rejected(&surface));
            }
            Event::Completed { req, result } => {
                REGISTRY.with(|registry| registry.resolve(req, crate::host::outcome_to_result(result)));
            }
            Event::HttpChunk { req, bytes, done } => {
                // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): used to discard every
                // non-final chunk outright (`if done { resolve(req, Ok(bytes)) }` — every earlier
                // `bytes` was simply dropped on the floor, silent data loss for any multi-chunk
                // response). `append_chunk` accumulates instead; `cap` is the owning instance's
                // `QuotaSchema.message_bytes` (default 64 MiB when unset/unknown — matches
                // `instance_task_quota`'s own `unwrap_or` fallback idiom above).
                REGISTRY.with(|registry| {
                    let cap = registry
                        .instance_of(req)
                        .and_then(|instance| INSTANCE_QUOTAS.with(|quotas| quotas.borrow().get(&instance).and_then(|schema| schema.message_bytes)))
                        .unwrap_or(64 * 1024 * 1024) as usize;
                    registry.append_chunk(req, bytes, done, cap);
                });
            }
            Event::JobProgress { .. } => {}
            Event::JobCompleted { job, result } => {
                // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): a job spawned
                // through `host::jobs::spawn` (`🌐host/🦀️component.rs`) allocates its `job` id from
                // THE SAME `RequestRegistry` counter as every other awaitable `host::*` call — the
                // `Effect::SpawnJob{job, ..}` this actor emitted carried `job == req.0` — so
                // `Event::JobCompleted{job, result}` resolves the identical parked `RequestFuture`
                // an `Event::Completed{req, result}` would, closing the "no `req`-per-job
                // correlation table yet" gap `📓️terra-M5-report.md` §4 named (no separate table
                // needed: the request id already IS the job id).
                REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(job), crate::host::outcome_to_result(result)));
            }
            Event::Message { .. } => {}
            Event::Timer { id } => {
                ARMED_TIMERS.with(|timers| timers.borrow_mut().retain(|armed| *armed != id));
                EXECUTOR.with(|executor| executor.wake(id));
            }
            Event::Wake => {}
            Event::Request { .. } => {}
            Event::Activate { .. } | Event::SuspendRequest | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}
        }
    }

    // 🔀️ "app-command → the existing PluginApp dispatch unchanged" (design-abi.md §4): batched
    // per-instance through the SAME `plugin_exchange` the old `exchange` WIT export called.
    let mut effects: Vec<Effect> = Vec::new();
    for (instance, commands) in app_commands {
        match crate::plugin_runtime::plugin_exchange(instance, &commands) {
            Ok(output) => {
                for frame_bytes in output.frames {
                    route_app_frame(instance, &frame_bytes, &mut effects);
                }
                // 🧬️ Channel v12 (A4) removed `AppFrame::Effects`/`Events` — `plugin_exchange` now
                // hands these back directly (design-abi.md §2/§4: effects/events travel straight into
                // `TurnResult`, never wrapped as a frame), so they're decoded here instead of through
                // `route_app_frame`.
                for one in &output.effects {
                    if let Ok(effect) = decode_wire_effect(one) {
                        effects.push(effect);
                    }
                }
                for one in &output.events {
                    if let Ok(event) = decode_wire_app_event(one) {
                        effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
                    }
                }
            }
            Err(fault) => effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) }),
        }
    }

    for (instance, surface) in dirty_render {
        if let Ok(body) = crate::plugin_runtime::plugin_render(instance, "window", "{}") {
            if let Some(patch) = PATCHES.with(|patches| patches.diff(&surface, body)) {
                // Collected into `ui_patches` below via a second pass so `effects` above stays the
                // single accumulation point for the non-UI half of the turn.
                PENDING_PATCHES.with(|pending| pending.borrow_mut().push(patch));
            }
        }
    }

    let more_work = EXECUTOR.with(|executor| executor.run_until_idle(64));
    effects.extend(REGISTRY.with(|registry| registry.drain()));

    // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumed `AsyncTask` follow-
    // ups (and any replayed `task_restarts` from a `restore` before this turn) — AFTER
    // `run_until_idle` so a task that resolved just now is redispatched the SAME turn, not the
    // next one. A resume can itself spawn more tasks (`dispatch_emit` runs for real), so the
    // executor may have fresh ready work by the time this returns — folded into `more_work` below
    // rather than requiring a second `run_until_idle` pass this turn (the next `poll` picks it up).
    let resumes_remain = drain_task_resumes(&mut effects, 64);
    let more_work = more_work || resumes_remain || EXECUTOR.with(|executor| executor.has_ready());

    let ui_patches: Vec<UiPatch> = PENDING_PATCHES.with(|pending| std::mem::take(&mut *pending.borrow_mut()));
    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };

    let result = semio_framework::kernel::TurnResult { ui_patches, effects, next_wake: ARMED_TIMERS.with(|timers| timers.borrow().first().copied()), status, fuel_used: 0 };
    Ok(kernel_turn_result_to_wit(result, budget))
}

thread_local! {
    static PENDING_PATCHES: RefCell<Vec<UiPatch>> = const { RefCell::new(Vec::new()) };
}

/// 🪪️ Surfaces are named `"<instance>:<body-key>"` in this wave (no dedicated `surface-ref`
/// bookkeeping table yet — `ui.wit`'s `surface-ref` record exists at the WIT boundary, but the
/// Rust-side `kernel::UiPatch.surface` is still a plain `String` per A3's landed shape).
async fn parse_surface_instance(surface: &str) -> Option<u32> {
    surface.split(':').next()?.parse().ok()
}

/// 🔀️ `AppFrame::UiPatch` → a real `kernel::UiPatch` passthrough into `PENDING_PATCHES` (the wire
/// frame is already `UiPatch`-shaped field-for-field — channel v12/A4 — so this is a decode, not a
/// render); `AppFrame::Effects`/`Events` no longer exist as frames (`poll` decodes
/// `plugin_exchange`'s `PluginExchangeOutput.effects`/`.events` directly instead — see there);
/// `AppFrame::UiSnapshotEnd` has no consumer yet in this wave (patches apply incrementally, no
/// snapshot-boundary bookkeeping); everything else → `Effect::SendMessage` to the shell, matching
/// design-abi.md §2's table verbatim.
async fn route_app_frame(instance: u32, frame_bytes: &[u8], effects: &mut Vec<Effect>) {
    let Ok(frame) = protocol::decode_app_frame(frame_bytes) else {
        return;
    };
    match frame {
        protocol::AppFrame::UiPatch { surface, kind, revision, base_revision, ops, .. } => {
            let Ok(ops_value) = store::pack_rt::decode_wire_value(&ops) else { return };
            let Ok(ops) = dsl::from_dsl_value::<Vec<PatchOp>>(ops_value) else { return };
            PENDING_PATCHES.with(|pending| pending.borrow_mut().push(UiPatch { surface, kind, revision, base_revision, ops }));
        }
        protocol::AppFrame::UiSnapshotEnd { .. } => {}
        other => {
            let payload = protocol::encode_app_frame(&other);
            effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload });
        }
    }
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): drains `TASK_RESUMES` (bounded
/// to `max_rounds` entries — the SAME defensive-cap shape `run_until_idle` uses, so an endlessly
/// respawning follow-up chain cannot stall a turn forever) and routes each resolved task's outcome
/// back into the SAME instance's `dyn PluginApp` via `plugin_runtime::plugin_resume_task`: a
/// `Command` resume through the existing typed-command dispatch, an `Emit` resume through a
/// decode + `dispatch_emit`, and a `Fault` resume straight to the shell as a message. Either
/// dispatch path's frames are fed through the SAME `route_app_frame` every other frame this turn
/// goes through — one implementation, not two (the eventual `world actor-async` runner calls this
/// SAME function, which is why it is `pub`, not `pub(crate)`).
///
/// Returns whether entries remain queued (the round cap was hit) — folded into `poll`'s
/// `turn-status::more-work` so a saturated resume queue is never silently dropped.
pub async fn drain_task_resumes(effects: &mut Vec<Effect>, max_rounds: u32) -> bool {
    for _ in 0..max_rounds {
        let Some(resume) = TASK_RESUMES.with(|resumes| resumes.borrow_mut().pop_front()) else {
            return false;
        };
        let input = match resume.outcome {
            TaskResumeOutcome::Fault(fault) => {
                effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(resume.instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) });
                continue;
            }
            TaskResumeOutcome::Command(bytes) => crate::plugin_runtime::TaskResumeInput::Command(bytes),
            TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => crate::plugin_runtime::TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops },
        };
        let output = crate::plugin_runtime::plugin_resume_task(resume.instance, &resume.meta, input);
        for frame_bytes in output.frames {
            route_app_frame(resume.instance, &frame_bytes, effects);
        }
        for one in &output.effects {
            if let Ok(effect) = decode_wire_effect(one) {
                effects.push(effect);
            }
        }
        for one in &output.events {
            if let Ok(event) = decode_wire_app_event(one) {
                effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
            }
        }
    }
    !TASK_RESUMES.with(|resumes| resumes.borrow().is_empty())
}

async fn decode_wire_effect(bytes: &[u8]) -> Result<Effect, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    dsl::from_dsl_value(value).map_err(|_| ())
}

async fn decode_wire_app_event(bytes: &[u8]) -> Result<semio_framework::kernel::AppEvent, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    dsl::from_dsl_value(value).map_err(|_| ())
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: decodes `instance-open-event.quotas` (a wire
/// `pack`) into the real `QuotaSchema` — see `wit_event_to_kernel`'s `InstanceOpen` arm. Falls
/// back to `default()` (no field set — read as "no limit declared" by every reader, e.g.
/// `instance_task_quota`'s `unwrap_or(16)`) on a decode failure rather than failing `InstanceOpen`
/// outright.
async fn decode_wire_quotas(bytes: &[u8]) -> semio_framework::kernel::QuotaSchema {
    store::pack_rt::decode_wire_value(bytes).ok().and_then(|value| dsl::from_dsl_value(value).ok()).unwrap_or_default()
}

/// 🔀️ WIT `event` → kernel `Event`. Thin field-for-field translation — the WIT side already
/// mirrors the kernel shape (see `📓️design-abi.md` §2 / `events.wit`'s own doc comments).
async fn wit_event_to_kernel(event: crate::component::component::exports::semio::framework::reactor::Event) -> Event {
    use crate::component::component::exports::semio::framework::reactor::Event as W;
    match event {
        W::InstanceOpen(payload) => Event::InstanceOpen {
            instance: semio_framework::kernel::PluginInstanceId(payload.instance.to_string()),
            app_id: semio_framework::kernel::AppInstanceId(payload.app_id),
            actor: payload.actor,
            config: payload.config,
            assets: payload.assets,
            capabilities: Vec::new(),
            // 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): previously always
            // `default()` — `spawn_task`'s quota gate is the first real reader of this field, so a
            // decode failure (malformed/empty pack) also falls back to `default()` rather than
            // failing `InstanceOpen` outright; a missing quota is "no limit declared", not a fault.
            quotas: decode_wire_quotas(&payload.quotas),
        },
        W::InstanceClose(_) => Event::InstanceClose,
        W::Activate(payload) => Event::Activate { reason: wit_activation_to_kernel(payload.reason) },
        W::SuspendRequest(_) => Event::SuspendRequest,
        W::CapabilityChanged(_) => Event::SuspendRequest,
        W::QuotaChanged(_) => Event::SuspendRequest,
        W::AppCommand(payload) => Event::AppCommandEvent { instance: semio_framework::kernel::PluginInstanceId(payload.instance.to_string()), seq: payload.seq, command: payload.command },
        W::SurfaceVisible(payload) => Event::SurfaceVisible { surface: format!("{}:{}", payload.surface.instance, "window") },
        W::SurfaceHidden(payload) => Event::SurfaceHidden { surface: format!("{}:{}", payload.surface.instance, "window") },
        W::SurfaceResized(payload) => Event::SurfaceResized { surface: format!("{}:{}", payload.surface.instance, "window"), width: payload.width, height: payload.height },
        W::PatchAck(payload) => Event::PatchAck { surface: format!("{}:{}", payload.surface.instance, "window"), revision: payload.revision },
        W::PatchRejected(payload) => Event::PatchRejected { surface: format!("{}:{}", payload.surface.instance, "window"), revision: payload.revision, reason: payload.reason },
        W::Completed(payload) => Event::Completed { req: semio_framework::kernel::RequestId(payload.req), result: wit_completion_to_kernel(payload.outcome) },
        W::HttpChunk(payload) => Event::HttpChunk { req: semio_framework::kernel::RequestId(payload.req), bytes: payload.params.bytes, done: payload.params.done },
        W::JobProgress(payload) => Event::JobProgress { job: payload.job, progress: Some(payload.progress) },
        W::JobCompleted(payload) => Event::JobCompleted { job: payload.job, result: wit_completion_to_kernel(payload.outcome) },
        W::Message(payload) => Event::Message { source: wit_endpoint_to_kernel(payload.source), payload: payload.payload },
        W::Timer(payload) => Event::Timer { id: payload.id },
        W::Wake => Event::Wake,
        W::Request(payload) => Event::Request { req: semio_framework::kernel::RequestId(payload.req), from: wit_endpoint_to_kernel(payload.params.origin), capability: payload.params.capability, payload: payload.params.payload },
    }
}

async fn wit_activation_to_kernel(reason: wit_events::ActivationEvent) -> semio_framework::kernel::ActivationEvent {
    use wit_events::ActivationEvent as W;
    match reason {
        W::OnCommand(id) => semio_framework::kernel::ActivationEvent::OnCommand { id },
        W::OnViewVisible(id) => semio_framework::kernel::ActivationEvent::OnViewVisible { id },
        W::OnFileType(ext) => semio_framework::kernel::ActivationEvent::OnFileType { ext },
        W::OnArtifactKind(kind) => semio_framework::kernel::ActivationEvent::OnArtifactKind { kind },
        W::OnExtensionRequest(point) => semio_framework::kernel::ActivationEvent::OnExtensionRequest { point },
        W::OnStartupFinished => semio_framework::kernel::ActivationEvent::OnStartupFinished,
    }
}

async fn wit_completion_to_kernel(result: wit_events::CompletionResult) -> RequestOutcome {
    use wit_events::CompletionResult as W;
    match result {
        W::Ok(bytes) => RequestOutcome::Ok(bytes),
        W::Fault(bytes) => RequestOutcome::Err(bytes),
    }
}

async fn wit_endpoint_to_kernel(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    use wit_types::MessageEndpoint as W;
    match endpoint {
        W::Shell(instance) => MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        W::Backbone(uri) => MessageEndpoint::Backbone { uri },
        W::PluginInstance(instance) => MessageEndpoint::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        W::Extension(id) => MessageEndpoint::Extension { id },
        W::Topic(name) => MessageEndpoint::Topic { name },
    }
}

/// 🔀️ kernel `TurnResult` → WIT `turn-result`. `budget` is currently unused beyond documenting
/// the seam — `max-effects`/`max-patch-bytes` capping is real, mechanical follow-up work (design-
/// abi.md §4's "capped by `max-effects`, overflow carries over") not yet wired into this wave.
async fn kernel_turn_result_to_wit(result: semio_framework::kernel::TurnResult, _budget: crate::component::component::exports::semio::framework::reactor::Budget) -> crate::component::component::exports::semio::framework::reactor::TurnResult {
    use crate::component::component::exports::semio::framework::reactor as wit;
    wit::TurnResult {
        ui_patches: result.ui_patches.into_iter().map(kernel_ui_patch_to_wit).collect(),
        effects: result.effects.into_iter().map(kernel_effect_to_wit).collect(),
        next_wake: result.next_wake,
        status: match result.status {
            TurnStatus::Idle => wit::TurnStatus::Idle,
            TurnStatus::MoreWork => wit::TurnStatus::MoreWork,
            TurnStatus::CheckpointReady => wit::TurnStatus::CheckpointReady,
            TurnStatus::Faulted(bytes) => wit::TurnStatus::Faulted(bytes),
        },
        fuel_used: result.fuel_used,
    }
}

async fn kernel_ui_patch_to_wit(patch: UiPatch) -> crate::component::component::exports::semio::framework::reactor::UiPatch {
    use crate::component::component::exports::semio::framework::reactor as wit;
    let instance: u32 = patch.surface.split(':').next().and_then(|s| s.parse().ok()).unwrap_or(0);
    wit::UiPatch {
        surface: wit_ui::SurfaceRef { instance, surface: 0 },
        kind: patch.kind,
        revision: patch.revision,
        base_revision: patch.base_revision,
        ops: patch.ops.into_iter().map(kernel_patch_op_to_wit).collect(),
    }
}

async fn kernel_patch_op_to_wit(op: PatchOp) -> wit_ui::PatchOp {
    let encode_node = |node: &ui_wgpu::wgpu::UiNode| store::pack_rt::encode_wire_value(&dsl::to_dsl_value(node).unwrap_or(dsl::DslValue::Null));
    match op {
        PatchOp::Replace { path, node } => wit_ui::PatchOp::Replace(wit_ui::PatchReplace { path: path_to_indices(&path), node: encode_node(&node) }),
        PatchOp::InsertChild { path, index, node } => wit_ui::PatchOp::InsertChild(wit_ui::PatchInsertChild { path: path_to_indices(&path), index, node: encode_node(&node) }),
        PatchOp::RemoveChild { path, index } => wit_ui::PatchOp::RemoveChild(wit_ui::PatchRemoveChild { path: path_to_indices(&path), index }),
        PatchOp::SetProps { path, props } => wit_ui::PatchOp::SetProps(wit_ui::PatchSetProps { path: path_to_indices(&path), props }),
    }
}

/// 🩹️ `kernel::PatchOp.path` is a `String` (A3's landed shape); `ui.wit`'s `path: list<u32>` is a
/// node-identity index path. This wave's `📸️patches` only ever emits the ROOT path (`""`, full-
/// body replace — see that module's scope note), so this always yields an empty list; a real
/// index-path encoding is follow-up work alongside the real (non-full-body) differ.
async fn path_to_indices(_path: &str) -> Vec<u32> {
    Vec::new()
}

/// 🔀️ kernel `Effect` → WIT `effect`. Field-for-field per `📓️design-abi.md` §2's table; complex
/// Rust-only field types (`WindowKindId`, `DslValue`, `MediaType`, `ClipboardFragment`, ...) are
/// wire-encoded through the SAME `store::pack_rt::encode_wire_value`/`dsl::to_dsl_value` idiom
/// every existing host boundary in this crate already uses.
async fn kernel_effect_to_wit(effect: Effect) -> crate::component::component::exports::semio::framework::reactor::Effect {
    use crate::component::component::exports::semio::framework::reactor as wit;
    async fn pack<T: serde::Serialize>(value: &T) -> Vec<u8> {
        store::pack_rt::encode_wire_value(&dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null))
    }
    match effect {
        Effect::OpenWindow { req, kind, params } => wit::Effect::OpenWindow(wit_effects::OpenWindowEffect { req: req.0, params: wit_effects::OpenWindowParams { kind: kind.0, params: pack(&params) } }),
        Effect::CloseWindow { window } => wit::Effect::CloseWindow(wit_effects::CloseWindowEffect { window: window.0 as u64 }),
        Effect::Notify { message } => wit::Effect::Notify(wit_effects::NotifyEffect { message }),
        Effect::ClipboardWrite { fragment } => wit::Effect::ClipboardWrite(wit_effects::ClipboardWriteEffect { fragment: pack(&fragment) }),
        Effect::RequestSync => wit::Effect::RequestSync,
        Effect::Navigate { uri } => wit::Effect::Navigate(wit_effects::NavigateEffect { uri }),
        Effect::LoadDocument { pack: doc_pack, spr } => wit::Effect::LoadDocument(wit_effects::LoadDocumentEffect { doc_pack, spr }),
        Effect::OpenExternalUrl { url } => wit::Effect::OpenExternalUrl(wit_effects::OpenExternalUrlEffect { url }),
        Effect::SetPanel { panel_json } => wit::Effect::SetPanel(wit_effects::SetPanelEffect { panel_json }),
        Effect::DownloadMediaExport { filename, mime_type, data, encoding } => wit::Effect::DownloadMediaExport(wit_effects::DownloadMediaExportEffect { filename, mime_type, data, encoding }),
        Effect::IconRenderExport { items } => wit::Effect::IconRenderExport(wit_effects::IconRenderExportEffect { items: pack(&items) }),
        Effect::RequestFileOpen { req, accept, read_as, import_action, multiple } => wit::Effect::RequestFileOpen(wit_effects::RequestFileOpenEffect { req: req.0, params: wit_effects::RequestFileOpenParams { accept, read_as, multiple, import_action } }),
        Effect::RequestMediaFrames { req, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args } => {
            wit::Effect::RequestMediaFrames(wit_effects::RequestMediaFramesEffect { req: req.0, params: wit_effects::RequestMediaFramesParams { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args: args.map(|value| pack(&value)) } })
        }
        Effect::SpawnPluginInstance { req, plugin_id, app_id, os_instance_id, label, document_json } => wit::Effect::SpawnPluginInstance(wit_effects::SpawnPluginInstanceEffect { req: req.0, params: wit_effects::SpawnPluginInstanceParams { plugin_id, app_id, os_instance_id, label, document_json } }),
        Effect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => wit::Effect::OpenPluginInstance(wit_effects::OpenPluginInstanceEffect { plugin_id, app_id, os_instance_id }),
        Effect::SetActiveUtility { window_id, utility_id } => wit::Effect::SetActiveUtility(wit_effects::SetActiveUtilityEffect { window_id, utility_id }),
        Effect::SetActiveTool { tool_id } => wit::Effect::SetActiveTool(wit_effects::SetActiveToolEffect { tool_id }),
        Effect::OpenDialog { req, dialog_id, args } => wit::Effect::OpenDialog(wit_effects::OpenDialogEffect { req: req.0, params: wit_effects::OpenDialogParams { dialog_id, args: args.map(|value| pack(&value)) } }),
        Effect::DispatchAction { req, action, args, delay_ms } => wit::Effect::DispatchAction(wit_effects::DispatchActionEffect { req: req.0, params: wit_effects::DispatchActionParams { action, args: args.map(|value| pack(&value)), delay_ms } }),
        Effect::ReplayShellCommand { action_id, args } => wit::Effect::ReplayShellCommand(wit_effects::ReplayShellCommandEffect { action_id, args: args.map(|value| pack(&value)) }),
        Effect::PatchWorld3dChrome { selection_json, vortices_json, document_selected_ids, document_highlighted_ids } => wit::Effect::PatchWorld3dChrome(wit_effects::PatchWorld3dChromeEffect { selection_json, vortices_json, document_selected_ids, document_highlighted_ids }),
        Effect::InvokeExtension { req, extension_id, capability, request_json } => wit::Effect::InvokeExtension(wit_effects::InvokeExtensionEffect { req: req.0, params: wit_effects::InvokeExtensionParams { extension_id, capability, payload: request_json.into_bytes() } }),
        Effect::SendMessage { target, payload } => wit::Effect::SendMessage(wit_effects::SendMessageEffect { target: kernel_endpoint_to_wit(target), payload }),
        Effect::PublishEvent { topic, payload } => wit::Effect::PublishEvent(wit_effects::PublishEventEffect { topic, payload }),
        Effect::BlobWrite { req, media_type, bytes } => wit::Effect::BlobWrite(wit_effects::BlobWriteEffect { req: req.0, params: wit_effects::BlobWriteParams { media_type: pack(&media_type), bytes } }),
        Effect::BlobLoad { req, hash } => wit::Effect::BlobLoad(wit_effects::BlobLoadEffect { req: req.0, params: wit_effects::BlobLoadParams { hash } }),
        Effect::HttpRequest { req, method, url, headers, body, stream } => wit::Effect::HttpRequest(wit_effects::HttpRequestEffect { req: req.0, params: wit_effects::HttpParams { method, url, headers, body, streaming: stream } }),
        Effect::DocumentRead { req, doc, lane } => wit::Effect::DocumentRead(wit_effects::DocumentReadEffect { req: req.0, params: wit_effects::DocumentReadParams { doc: doc.0 as u64, lane } }),
        Effect::DocumentWrite { req, doc, lane, ops } => wit::Effect::DocumentWrite(wit_effects::DocumentWriteEffect { req: req.0, params: wit_effects::DocumentWriteParams { doc: doc.0 as u64, lane, ops } }),
        Effect::LinkResolve { req, link } => wit::Effect::LinkResolve(wit_effects::LinkResolveEffect { req: req.0, link: link.into_bytes() }),
        Effect::RegistryQuery { req, kind, filter } => wit::Effect::RegistryQuery(wit_effects::RegistryQueryEffect { req: req.0, params: wit_effects::RegistryQueryParams { kind, filter: filter.map(|value| pack(&value)).unwrap_or_default() } }),
        Effect::IoCompose { req, key, sources } => wit::Effect::IoCompose(wit_effects::IoComposeEffect { req: req.0, params: wit_effects::IoComposeParams { key: key.into_bytes(), sources: pack(&sources) } }),
        Effect::CacheDerive { req, engine_id, input } => wit::Effect::CacheDerive(wit_effects::CacheDeriveEffect { req: req.0, params: wit_effects::CacheDeriveParams { engine_id, input } }),
        Effect::CacheRead { req, engine_id, key } => wit::Effect::CacheRead(wit_effects::CacheReadEffect { req: req.0, params: wit_effects::CacheReadParams { engine_id, key: key.into_bytes() } }),
        Effect::SetTimer { id, after_ms, repeat } => {
            ARMED_TIMERS.with(|timers| timers.borrow_mut().push(id));
            wit::Effect::SetTimer(wit_effects::SetTimerEffect { id, after_ms: after_ms as u32, repeat })
        }
        Effect::SpawnJob { job, kind, input, placement } => wit::Effect::SpawnJob(wit_effects::SpawnJobEffect { job, kind, input, placement: kernel_placement_to_wit(placement) }),
        Effect::CancelJob { job } => wit::Effect::CancelJob(wit_effects::CancelJobEffect { job }),
        Effect::Respond { req, result } => wit::Effect::Respond(wit_effects::RespondEffect { req: req.0, outcome: kernel_outcome_to_wit_respond(result) }),
        Effect::StorageRead { req, key } => wit::Effect::StorageRead(wit_effects::StorageReadEffect { req: req.0, params: wit_effects::StorageReadParams { key } }),
        Effect::StorageWrite { req, key, bytes } => wit::Effect::StorageWrite(wit_effects::StorageWriteEffect { req: req.0, params: wit_effects::StorageWriteParams { key, value: bytes } }),
        Effect::StorageDelete { req, key } => wit::Effect::StorageDelete(wit_effects::StorageDeleteEffect { req: req.0, params: wit_effects::StorageDeleteParams { key } }),
        Effect::RequestCapability { req, capability } => wit::Effect::RequestCapability(wit_effects::RequestCapabilityEffect { req: req.0, params: wit_effects::RequestCapabilityParams { id: capability.id.0, scope: capability.scope, reason: capability.reason, optional: capability.optional } }),
        Effect::ReleaseCapability { id } => wit::Effect::ReleaseCapability(wit_effects::ReleaseCapabilityEffect { id: id.0 }),
        Effect::Subscribe { topic } => wit::Effect::Subscribe(wit_effects::SubscribeEffect { topic }),
        Effect::Unsubscribe { topic } => wit::Effect::Unsubscribe(wit_effects::SubscribeEffect { topic }),
    }
}

async fn kernel_endpoint_to_wit(endpoint: MessageEndpoint) -> wit_types::MessageEndpoint {
    match endpoint {
        MessageEndpoint::Shell { instance } => wit_types::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
        MessageEndpoint::Backbone { uri } => wit_types::MessageEndpoint::Backbone(uri),
        MessageEndpoint::PluginInstance { id } => wit_types::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
        MessageEndpoint::Extension { id } => wit_types::MessageEndpoint::Extension(id),
        MessageEndpoint::Topic { name } => wit_types::MessageEndpoint::Topic(name),
    }
}

async fn kernel_placement_to_wit(placement: semio_framework::kernel::JobPlacement) -> wit_effects::JobPlacement {
    match placement {
        semio_framework::kernel::JobPlacement::Inline => wit_effects::JobPlacement::Inline,
        semio_framework::kernel::JobPlacement::Isolated => wit_effects::JobPlacement::Isolated,
        semio_framework::kernel::JobPlacement::Exclusive => wit_effects::JobPlacement::Exclusive,
    }
}

async fn kernel_outcome_to_wit_respond(result: RequestOutcome) -> wit_effects::RespondResult {
    match result {
        RequestOutcome::Ok(bytes) => wit_effects::RespondResult::Ok(bytes),
        RequestOutcome::Err(bytes) => wit_effects::RespondResult::Fault(bytes),
    }
}

} // mod wit_bridge

/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: test-only hooks into this actor's UNGATED
/// per-actor state (`EXECUTOR`/`REGISTRY`/`TASK_RESUMES`) — `poll`'s real event routing
/// (`Event::Completed` → `REGISTRY::resolve`, `run_until_idle`, `drain_task_resumes`) lives inside
/// `wit_bridge`, which is wasm32-wasip2-only and cannot be exercised by a native `cargo test`. This
/// module gives native tests (this crate's own, and `plugin_runtime`'s `test_push_instance`-backed
/// integration tests) the SAME three primitives `poll` itself drives, so the mechanism — spawn,
/// park, get resolved by an injected completion, resume — is exercised end-to-end without needing
/// a wasm32-wasip2 build. `pub(crate)`, `#[cfg(test)]`-gated: never part of the real API surface.
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    /// ▶️ The exact `run_until_idle` call `poll` makes after routing events, exposed directly.
    pub(crate) async fn run_until_idle(max_iterations: u32) -> bool {
        EXECUTOR.with(|executor| executor.run_until_idle(max_iterations))
    }

    /// ✅️ The exact `REGISTRY::resolve` call `poll`'s `Event::Completed` arm makes, exposed
    /// directly — the native stand-in for "an injected `Event::Completed`".
    pub(crate) async fn resolve_request(id: u64, result: Result<Vec<u8>, semio_framework::Fault>) {
        REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(id), result));
    }

    /// 📬️ Pops one `TASK_RESUMES` entry, erased to `plugin_runtime::TaskResumeInput` (a `Fault`
    /// resolution surfaces as `Err` here instead — `drain_task_resumes` frames that straight to
    /// the shell without ever reaching `plugin_runtime`, so a test asserting on it never needs
    /// `plugin_resume_task` at all).
    pub(crate) async fn pop_task_resume() -> Option<(u32, crate::app::ActionMeta, Result<crate::plugin_runtime::TaskResumeInput, semio_framework::Fault>)> {
        TASK_RESUMES.with(|resumes| resumes.borrow_mut().pop_front()).map(|resume| {
            let input = match resume.outcome {
                TaskResumeOutcome::Command(bytes) => Ok(crate::plugin_runtime::TaskResumeInput::Command(bytes)),
                TaskResumeOutcome::Emit { artifact_ops, config_ops, draft_ops } => Ok(crate::plugin_runtime::TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops }),
                TaskResumeOutcome::Fault(fault) => Err(fault),
            };
            (resume.instance, resume.meta, input)
        })
    }

    pub(crate) async fn task_count_for_instance(instance: u32) -> usize {
        TASK_RECORDS.with(|records| records.borrow().values().filter(|record| record.instance == instance).count())
    }

    pub(crate) async fn task_key_is_live(instance: u32, key: &str) -> bool {
        TASK_KEYS.with(|keys| keys.borrow().contains_key(&(instance, key.to_string())))
    }

    pub(crate) async fn set_instance_quota(instance: u32, outstanding_requests: u64) {
        INSTANCE_QUOTAS.with(|quotas| {
            quotas.borrow_mut().insert(instance, semio_framework::kernel::QuotaSchema { outstanding_requests: Some(outstanding_requests), ..Default::default() });
        });
    }

    pub(crate) async fn pending_request_count() -> usize {
        REGISTRY.with(|registry| registry.pending_ids().len())
    }

    /// 🚫️ The exact `RequestRegistry::cancel_instance` call `poll`'s `Event::InstanceClose` arm
    /// makes right after `cancel_instance_tasks`, exposed directly for a test to run the SAME
    /// two-step sequence natively.
    pub(crate) async fn cancel_instance_registry_requests(instance: u32) -> usize {
        REGISTRY.with(|registry| registry.cancel_instance(instance))
    }
}
