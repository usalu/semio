//! 🧵️ `ShardLoop` — `design-runtime.md` §2/§"ShardTransport": the loop a thread shard runs
//! in-process. Owns a set of live [`super::GuestInstance`]s, pulls [`semio_framework_actor::Envelope`]s
//! off a [`semio_framework_actor::ShardTransport`], groups them per actor, drives
//! [`super::GuestRuntime::execute_turn`]/[`super::GuestRuntime::step_job`], and sends the resulting
//! [`semio_framework::kernel::TurnResult`]/[`super::JobStep`] back over the SAME transport as bytes.
//!
//! Written so the identical type can later be driven over stdio by a helper process (packet P1,
//! `ProcessTransport`) — the only thing that changes between "thread shard" and "process shard" is
//! which [`ShardTransport`] impl `ShardLoop::new` receives; this file never branches on which one it
//! got. `ProcessTransport` itself is out of this packet's scope (`📌️important.md`'s sequencing:
//! "`semio-shard` `[[bin]]` runs over stdio" is P1, not B1b) — this is the seam, not the process.

#[cfg(test)]
use super::{MockGuestRuntime, PackageHash, PackageId, PackageRef};
use super::{GuestInstance, GuestRuntime, JobBudget, JobStep, PluginHostError, TurnFault};
use semio_framework::kernel::{Budget, Effect, Event, JobPlacement, RequestOutcome, TurnResult};
use semio_framework_actor::{ActorId, Envelope, Payload, ShardTransport};
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

/// ⛽️ `jobs.wit`'s `job-budget` for every job step `ShardLoop::pump` self-drives from an
/// `Effect::SpawnJob` admission — mirrors `🖥️host/🦀️component.rs`'s `PostTurnRelay::
/// RELAY_JOB_BUDGET` constant (same value, different call site: that one backs the three
/// hardcoded-kind synchronous relay, this one backs the generic, resumable, one-step-per-`pump`
/// path `Effect::SpawnJob` itself was always missing — `📓️terra-M5-report.md` §4(a)).
const JOB_STEP_BUDGET: JobBudget = JobBudget { fuel: 50_000_000, deadline_ms: 200 };

/// 📤️ One outcome `ShardLoop` sends back over the transport — tagged so a caller on the OTHER end
/// (a `ShardClient`, per `design-runtime.md` §"Web shard"/`ShardTable`) can tell a full turn result
/// apart from a single job step without probing the bytes.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShardOutcome {
    Turn { actor: u64, result: TurnResult },
    Job { actor: u64, job: u64, step: JobStep },
    Fault { actor: u64, message: String },
}

/// 🧵️ design-runtime.md §2. One `ShardLoop` per shard (an OS thread today, a `[[bin]]` process in
/// P1) — never shared across shards, since [`super::GuestRuntime`] instances are `Send + Sync` but a
/// [`GuestInstance`] is pinned to whichever shard activated it (`ShardTable`'s own pinning rule).
pub struct ShardLoop {
    runtime: Arc<dyn GuestRuntime>,
    transport: Box<dyn ShardTransport>,
    instances: HashMap<u64, GuestInstance>,
    /// 💼️ `(actor, job)` pairs admitted from an `Effect::SpawnJob` and not yet `Done`/`Failed` —
    /// MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME J1, the generic host-side executor
    /// `📓️terra-M5-report.md` §4(a) found missing entirely: nothing previously read a
    /// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ..}` and spawned/drove a job
    /// for it outside the three hardcoded kinds `PluginInstanceHandle::run_job_to_completion`
    /// calls directly. `pump()` steps every entry here exactly once per call — never loops a job
    /// to completion internally — so a job needing N steps needs N `pump()` calls, which is what
    /// proves resumability rather than a single-shot call.
    running_jobs: BTreeSet<(u64, u64)>,
    /// 📨️ `Event::JobCompleted` synthesized when a `running_jobs` entry reaches `Done`/`Failed`,
    /// queued per originating actor and delivered at the TOP of the NEXT `pump()` call (merged
    /// into that call's `events_by_actor`) — so a job's own actor sees the completion on its next
    /// turn even if no other envelope ever arrives for it, exactly the way a real `Event::
    /// Completed` would reach the guest's `RequestRegistry` (`job == req.0`, see `🌐host/
    /// 🦀️component.rs`'s `Host::spawn_job` / `⚛️reactor/🦀️component.rs`'s `Event::JobCompleted`
    /// routing step).
    pending_completions: HashMap<u64, Vec<Event>>,
}

impl ShardLoop {
    pub fn new(runtime: Arc<dyn GuestRuntime>, transport: Box<dyn ShardTransport>) -> Self {
        Self { runtime, transport, instances: HashMap::new(), running_jobs: BTreeSet::new(), pending_completions: HashMap::new() }
    }

    /// 📌️ Adds an already-instantiated actor to this shard's live set — called once per
    /// `Kernel::activate` that lands on this shard. `actor.0` (the bit-packed `u64`) is the map key
    /// throughout this type: `Envelope.to`/`ShardOutcome`'s tag both carry the SAME raw id, so no
    /// `RuntimeActorId` round-trip is needed at the boundary.
    pub fn register(&mut self, actor: ActorId, instance: GuestInstance) {
        self.instances.insert(actor.0, instance);
    }

    pub fn is_registered(&self, actor: ActorId) -> bool {
        self.instances.contains_key(&actor.0)
    }

    /// ✂️ Releases an actor's instance (generation change on restart, or a real unload) — calls
    /// [`super::GuestRuntime::drop_instance`] so the pooling allocator reclaims its slab.
    pub fn unregister(&mut self, actor: ActorId) {
        if let Some(instance) = self.instances.remove(&actor.0) {
            self.runtime.drop_instance(instance);
        }
        self.running_jobs.retain(|&(job_actor, _)| job_actor != actor.0);
        self.pending_completions.remove(&actor.0);
    }

    pub fn actor_count(&self) -> usize {
        self.instances.len()
    }

    /// 🌀️ Drains every envelope CURRENTLY buffered on the transport (never blocks past that — a
    /// shard must keep polling other actors, not stall on one slow producer), groups them by
    /// destination actor preserving arrival order (`execute_turn` takes `events: &[Event]`, i.e. one
    /// call per actor per pump, not per envelope), and runs exactly one `execute_turn`/`step_job` per
    /// actor that had at least one envelope. Returns the number of actors driven this pump.
    ///
    /// `Payload::Event`'s bytes are this file's own JSON encoding of `semio_framework::kernel::Event`
    /// — `semio_framework_actor::Payload`'s own doc comment calls this "pack-encoded", the eventual
    /// intended format once `🎠️kernel` grows a `pack_encode`/`pack_decode` for `Event`/`TurnResult`
    /// (not yet built, `🎠️kernel` is out of this packet's `path_scope`); JSON is what every OTHER
    /// wire boundary in this crate already uses (`IoRouter`/`EffectEventMarshal`), so this is a
    /// documented, consistent placeholder, not an invented one-off.
    pub fn pump(&mut self, budget_for: impl Fn(ActorId) -> Budget) -> Result<usize, PluginHostError> {
        // 💼️ Completions queued by the PREVIOUS `pump()` call (bottom of this function) are
        // delivered as ordinary events on THIS call — the same channel envelope-sourced events
        // arrive on, so the originating actor's `execute_turn` sees `Event::JobCompleted` exactly
        // like any other inbound event, with no separate delivery path.
        let mut events_by_actor: HashMap<u64, Vec<Event>> = std::mem::take(&mut self.pending_completions);
        let mut jobs_by_actor: Vec<(u64, u64)> = Vec::new();

        while let Some(bytes) = self.transport.recv() {
            let mut pos = 0usize;
            let envelope = Envelope::pack_decode(&bytes, &mut pos).map_err(|error| PluginHostError::Plugin(format!("ShardLoop::pump: malformed envelope: {error:?}")))?;
            match envelope.payload {
                Payload::Event(event_bytes) => {
                    let event: Event = serde_json::from_slice(&event_bytes)?;
                    events_by_actor.entry(envelope.to.0).or_default().push(event);
                }
                Payload::JobStep { job } => jobs_by_actor.push((envelope.to.0, job)),
                // 🚧️ `Suspend`/`Resume`/`Cancel` are real `Payload` variants (`semio_framework_actor`,
                // A1) with no `GuestRuntime` counterpart yet — `checkpoint`/`restore` exist on the
                // trait but nothing here decides WHEN to call them (that is the scheduler's job,
                // `design-runtime.md` §"FailurePolicy"/`Kernel::suspend`/`resume`, not built in this
                // packet). Documented gap, not a silent no-op: surfaced as a `ShardOutcome::Fault` so
                // a caller sees it rather than the envelope silently vanishing.
                other @ (Payload::Suspend { .. } | Payload::Resume { .. } | Payload::Cancel(_)) => {
                    self.send_outcome(&ShardOutcome::Fault { actor: envelope.to.0, message: format!("ShardLoop::pump: {other:?} has no GuestRuntime dispatch yet (needs Kernel::suspend/resume/cancel, not built in this packet)") })?;
                }
            }
        }

        let mut driven = 0usize;
        for (actor_id, events) in events_by_actor {
            let actor = ActorId(actor_id);
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") })?;
                continue;
            };
            let outcome = match self.runtime.execute_turn(instance, &events, budget_for(actor)) {
                Ok(result) => {
                    // 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1): the generic `Effect::
                    // SpawnJob`/`Effect::CancelJob` admission this packet closes — see
                    // `running_jobs`'s own doc comment. `placement` (inline/isolated/exclusive)
                    // is accepted but not yet acted on: routing to a DIFFERENT pooled/exclusive
                    // instance needs the actor pool `Kernel::activate`/`ShardTable` builds
                    // (`design-runtime.md` §1, `🎭️actor`/`T1-tasks` territory, not `🔌️plugin/**`)
                    // — every placement runs on the SAME instance that spawned it in this wave, a
                    // documented gap, not a silently faked one.
                    for effect in &result.effects {
                        match effect {
                            Effect::SpawnJob { job, kind, input, .. } => match self.runtime.start_job(instance, *job, kind, input.clone()) {
                                Ok(()) => {
                                    self.running_jobs.insert((actor_id, *job));
                                }
                                Err(fault) => {
                                    self.pending_completions.entry(actor_id).or_default().push(Event::JobCompleted { job: *job, result: RequestOutcome::Err(start_job_fault_bytes(&fault)) });
                                }
                            },
                            Effect::CancelJob { job } => {
                                if self.running_jobs.remove(&(actor_id, *job)) {
                                    let _ = self.runtime.cancel_job(instance, *job);
                                }
                            }
                            _ => {}
                        }
                    }
                    ShardOutcome::Turn { actor: actor_id, result }
                }
                Err(fault) => ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) },
            };
            self.send_outcome(&outcome)?;
            driven += 1;
        }

        // 💼️ Step every job still live — both envelope-driven (`Payload::JobStep`, explicit
        // external re-arming) and self-tracked (`running_jobs`, admitted from a `SpawnJob` effect
        // THIS pump or an earlier one — including one admitted just above, in the SAME pump call
        // that started it). ONE `step_job` per job per `pump()`, deliberately never a loop to
        // completion here (that is `PluginInstanceHandle::run_job_to_completion`'s DIFFERENT,
        // deliberately-synchronous relay for the three hardcoded io/infer kinds) — a job needing N
        // steps needs N `pump()` calls, which is the entire resumability proof.
        let mut to_step: Vec<(u64, u64)> = jobs_by_actor;
        for &pair in &self.running_jobs {
            if !to_step.contains(&pair) {
                to_step.push(pair);
            }
        }

        for (actor_id, job) in to_step {
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") })?;
                continue;
            };
            let outcome = match self.runtime.step_job(instance, job, JOB_STEP_BUDGET) {
                Ok(step) => {
                    match &step {
                        JobStep::Running(_) => {}
                        JobStep::Done(bytes) => {
                            self.running_jobs.remove(&(actor_id, job));
                            self.pending_completions.entry(actor_id).or_default().push(Event::JobCompleted { job, result: RequestOutcome::Ok(bytes.clone()) });
                        }
                        JobStep::Failed(bytes) => {
                            self.running_jobs.remove(&(actor_id, job));
                            self.pending_completions.entry(actor_id).or_default().push(Event::JobCompleted { job, result: RequestOutcome::Err(bytes.clone()) });
                        }
                    }
                    ShardOutcome::Job { actor: actor_id, job, step }
                }
                Err(fault) => {
                    self.running_jobs.remove(&(actor_id, job));
                    self.pending_completions.entry(actor_id).or_default().push(Event::JobCompleted { job, result: RequestOutcome::Err(start_job_fault_bytes(&fault)) });
                    ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) }
                }
            };
            self.send_outcome(&outcome)?;
            driven += 1;
        }

        Ok(driven)
    }

    fn send_outcome(&self, outcome: &ShardOutcome) -> Result<(), PluginHostError> {
        let bytes = serde_json::to_vec(outcome)?;
        self.transport.send(&bytes);
        Ok(())
    }

    pub fn heartbeat(&self) -> u64 {
        self.transport.heartbeat()
    }
}

fn turn_fault_message(fault: &TurnFault) -> String {
    fault.to_string()
}

/// 🧯️ Encodes a host-side `TurnFault` (a `start-job` admission failure, or a `step_job` runtime
/// fault) into the same `dsl::encode_fault_bytes` wire shape `Event::JobCompleted{result: Err
/// (bytes), ..}`'s bytes always carry — every other fault-bearing `RequestOutcome::Err` in this
/// crate already uses this encoding, so the guest's `crate::host::outcome_to_result` decodes it
/// exactly like a normal `Event::Completed` failure, with no special-casing for jobs.
fn start_job_fault_bytes(fault: &TurnFault) -> Vec<u8> {
    dsl::encode_fault_bytes(&semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("job.host-fault"), fault.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// 🧵️ In-process, single-actor loopback transport — an `mpsc`-free stand-in for
    /// `design-runtime.md`'s `ThreadTransport`, precise enough to exercise `ShardLoop::pump`'s real
    /// drain/group/dispatch/send logic end to end without needing a real thread. Its two buffers are
    /// `Arc<Mutex<..>>` INTERNALLY (not the struct itself behind an `Arc`) so `LoopbackProbe::new`
    /// can hand `ShardLoop::new` sole ownership of a `Box<dyn ShardTransport>` while keeping a
    /// separate handle that can still inspect `outbound` afterward — `impl ShardTransport for
    /// Arc<LoopbackTransport>` would hit `E0117` (neither `Arc` nor `ShardTransport` is local to this
    /// crate, and `Arc` is not `#[fundamental]` the way `Box` is).
    #[derive(Default)]
    struct LoopbackTransport {
        inbound: Arc<Mutex<Vec<Vec<u8>>>>,
        outbound: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    struct LoopbackProbe {
        inbound: Arc<Mutex<Vec<Vec<u8>>>>,
        outbound: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    impl LoopbackTransport {
        /// Returns `(the transport ShardLoop::new takes ownership of, a probe this test keeps)`.
        fn paired() -> (Self, LoopbackProbe) {
            let transport = Self::default();
            let probe = LoopbackProbe { inbound: transport.inbound.clone(), outbound: transport.outbound.clone() };
            (transport, probe)
        }
    }

    impl LoopbackProbe {
        fn push_inbound(&self, bytes: Vec<u8>) {
            self.inbound.lock().expect("loopback lock").push(bytes);
        }
        fn take_outbound(&self) -> Vec<Vec<u8>> {
            std::mem::take(&mut *self.outbound.lock().expect("loopback lock"))
        }
    }

    impl ShardTransport for LoopbackTransport {
        fn send(&self, bytes: &[u8]) {
            self.outbound.lock().expect("loopback lock").push(bytes.to_vec());
        }
        fn recv(&self) -> Option<Vec<u8>> {
            self.inbound.lock().expect("loopback lock").pop()
        }
        fn heartbeat(&self) -> u64 {
            0
        }
        fn kill(&self) {}
    }

    fn encode_event_envelope(to: ActorId, seq: u64, event: &Event) -> Vec<u8> {
        let envelope = Envelope { to, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event(serde_json::to_vec(event).expect("encode event")) };
        let mut bytes = Vec::new();
        envelope.pack_encode(&mut bytes);
        bytes
    }

    #[test]
    fn pump_drives_one_turn_per_actor_and_reports_it_as_a_shard_outcome() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(7);
        let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([1u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("mock instantiate");
        let mut scripted = MockGuestRuntime::idle_turn();
        scripted.fuel_used = 42;
        mock.script_turn(actor, scripted);

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose));

        let mut shard = ShardLoop::new(mock.clone(), Box::new(transport));
        shard.register(actor, instance);
        assert!(shard.is_registered(actor));

        let driven = shard.pump(|_| Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("pump succeeds");
        assert_eq!(driven, 1, "exactly one actor had a buffered envelope");

        let outbound = probe.take_outbound();
        assert_eq!(outbound.len(), 1, "one ShardOutcome sent back");
        let outcome: ShardOutcome = serde_json::from_slice(&outbound[0]).expect("decode outcome");
        match outcome {
            ShardOutcome::Turn { actor: reported, result } => {
                assert_eq!(reported, 7);
                assert_eq!(result.fuel_used, 42, "the scripted turn's own fuel_used must round-trip through ShardOutcome");
            }
            other => panic!("expected ShardOutcome::Turn, got {other:?}"),
        }
    }

    #[test]
    fn pump_reports_an_envelope_for_an_unregistered_actor_as_a_fault_not_a_silent_drop() {
        let mock = Arc::new(MockGuestRuntime::new());
        let (transport, probe) = LoopbackTransport::paired();
        let stranger = ActorId(99);
        probe.push_inbound(encode_event_envelope(stranger, 1, &Event::InstanceClose));

        let mut shard = ShardLoop::new(mock, Box::new(transport));
        let driven = shard.pump(|_| Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("pump succeeds even with an unknown actor");
        assert_eq!(driven, 0, "an envelope for an unregistered actor drives nothing");

        let outbound = probe.take_outbound();
        assert_eq!(outbound.len(), 1);
        let outcome: ShardOutcome = serde_json::from_slice(&outbound[0]).expect("decode outcome");
        assert!(matches!(outcome, ShardOutcome::Fault { actor, .. } if actor == 99), "must surface as a Fault naming the actor, not vanish");
    }

    #[test]
    fn unregister_drops_the_instance_and_shrinks_actor_count() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(3);
        let package = PackageRef { package: PackageId("gif".to_string()), hash: PackageHash([2u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("mock instantiate");
        let (transport, _probe) = LoopbackTransport::paired();
        let mut shard = ShardLoop::new(mock, Box::new(transport));
        shard.register(actor, instance);
        assert_eq!(shard.actor_count(), 1);
        shard.unregister(actor);
        assert_eq!(shard.actor_count(), 0);
        assert!(!shard.is_registered(actor));
    }

    /// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME J1's headline acceptance test — the mechanism
    /// `📓️terra-M5-report.md` §4(a) found entirely missing: "no code anywhere reads a
    /// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ...}` and spawns/drives a job
    /// for it". Spawns a job from a scripted turn's own emitted effect, steps it across THREE
    /// separate `pump()` calls (`Running`, `Running`, `Done` — never a single-shot call, which is
    /// what actually proves the `JobBudget` mechanism resumes rather than just completing once),
    /// and observes the completion reach the ORIGINATING actor as a real `Event::JobCompleted` on
    /// a LATER `execute_turn` call — not merely that `step_job` returned `Done` in isolation.
    #[test]
    fn spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(21);
        let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([9u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("mock instantiate");

        let job_id = 777u64;
        let mut spawning_turn = MockGuestRuntime::idle_turn();
        spawning_turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: b"seed-frames".to_vec(), placement: JobPlacement::Isolated });
        mock.script_turn(actor, spawning_turn);
        // 🔀️ `run_job_to_completion`'s own two-arm shape (Running.../Done) but with TWO `Running`
        // steps first — the resumability proof: a job that finished on step 1 would not
        // distinguish "the budget mechanism resumed it" from "it happened to be a one-shot call".
        mock.script_job_step(actor, JobStep::Running(None));
        mock.script_job_step(actor, JobStep::Running(Some(b"halfway".to_vec())));
        mock.script_job_step(actor, JobStep::Done(b"reconstruction-complete".to_vec()));
        // 🔚️ Whatever `execute_turn` call eventually receives the `Event::JobCompleted` (pump 4,
        // below) still needs a scripted outcome to return — an ordinary idle turn is enough since
        // this test only asserts what EVENTS that call was given, not its own output.
        mock.script_turn(actor, MockGuestRuntime::idle_turn());

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose));

        let mut shard = ShardLoop::new(mock.clone(), Box::new(transport));
        shard.register(actor, instance);

        // Pump 1: runs the spawning turn, admits `Effect::SpawnJob` (`start_job`), and — because
        // the job lands in `running_jobs` before the step phase runs — takes its FIRST step in
        // this SAME pump (`Running`).
        let driven1 = shard.pump(|_| Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("pump 1");
        assert_eq!(driven1, 2, "one turn (the spawn) plus one job step (the first Running) this pump");

        // Pump 2 and 3: no new envelopes at all — the job is driven PURELY from `running_jobs`,
        // proving `pump()` self-drives an admitted job without needing external re-arming.
        let driven2 = shard.pump(|_| Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("pump 2");
        assert_eq!(driven2, 1, "only the second Running step — no envelope, so no turn this pump");
        let driven3 = shard.pump(|_| Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("pump 3");
        assert_eq!(driven3, 1, "the terminal Done step");

        // Pump 4: still no new envelope — but the Done step queued an `Event::JobCompleted` for
        // delivery, so the actor is driven ONE more time purely to receive it.
        let driven4 = shard.pump(|_| Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("pump 4");
        assert_eq!(driven4, 1, "the queued completion drives one more turn, with no job left to step");

        let outbound = probe.take_outbound();
        let outcomes: Vec<ShardOutcome> = outbound.iter().map(|bytes| serde_json::from_slice(bytes).expect("decode outcome")).collect();
        let job_outcomes: Vec<&JobStep> = outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ShardOutcome::Job { job, step, .. } if *job == job_id => Some(step),
                _ => None,
            })
            .collect();
        assert_eq!(job_outcomes.len(), 3, "exactly three step_job calls were made — the resumability proof");
        assert!(matches!(job_outcomes[0], JobStep::Running(None)));
        assert!(matches!(job_outcomes[1], JobStep::Running(Some(bytes)) if bytes == b"halfway"));
        assert!(matches!(job_outcomes[2], JobStep::Done(bytes) if bytes == b"reconstruction-complete"));

        // 🎯️ The actual end-to-end proof: the ORIGINATING actor's `execute_turn` was, at some
        // point, handed a real `Event::JobCompleted{job: 777, result: Ok(..)}` — not merely that
        // `step_job` internally returned `Done` (`job_outcomes` above already showed that; this is
        // the part M5 found completely missing: nothing delivered it back).
        let completed = mock.observed_events(actor).into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));
        match completed {
            Some(Event::JobCompleted { result: RequestOutcome::Ok(bytes), .. }) => {
                assert_eq!(bytes, b"reconstruction-complete", "the Done step's own bytes must round-trip into the delivered completion event");
            }
            other => panic!("expected Event::JobCompleted{{job: 777, result: Ok(..)}} to have reached the originating actor's execute_turn, got {other:?}"),
        }
    }

    /// 🛑️ `Effect::CancelJob` removes a job from `running_jobs` in the SAME turn it is seen, so a
    /// job cancelled before its first step is never stepped at all — no stray `ShardOutcome::Job`
    /// for it, ever.
    #[test]
    fn cancel_job_effect_stops_a_job_before_it_is_ever_stepped() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(22);
        let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([10u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("mock instantiate");

        let job_id = 888u64;
        let mut turn = MockGuestRuntime::idle_turn();
        turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        turn.effects.push(Effect::CancelJob { job: job_id });
        mock.script_turn(actor, turn);

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose));
        let mut shard = ShardLoop::new(mock, Box::new(transport));
        shard.register(actor, instance);

        let driven = shard.pump(|_| Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("pump");
        assert_eq!(driven, 1, "only the turn itself — the job was cancelled before the step phase, so no step_job call happened");

        let outbound = probe.take_outbound();
        let outcomes: Vec<ShardOutcome> = outbound.iter().map(|bytes| serde_json::from_slice(bytes).expect("decode outcome")).collect();
        assert!(!outcomes.iter().any(|outcome| matches!(outcome, ShardOutcome::Job { .. })), "a cancelled-before-first-step job must never produce a ShardOutcome::Job");
    }
}
