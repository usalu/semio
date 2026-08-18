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
use semio_framework::kernel::{Budget, Event, TurnResult};
use semio_framework_actor::{ActorId, Envelope, Payload, ShardTransport};
use std::collections::HashMap;
use std::sync::Arc;

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
}

impl ShardLoop {
    pub fn new(runtime: Arc<dyn GuestRuntime>, transport: Box<dyn ShardTransport>) -> Self {
        Self { runtime, transport, instances: HashMap::new() }
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
        let mut events_by_actor: HashMap<u64, Vec<Event>> = HashMap::new();
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
                Ok(result) => ShardOutcome::Turn { actor: actor_id, result },
                Err(fault) => ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) },
            };
            self.send_outcome(&outcome)?;
            driven += 1;
        }

        for (actor_id, job) in jobs_by_actor {
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") })?;
                continue;
            };
            let job_budget = JobBudget { fuel: 50_000_000, deadline_ms: 200 };
            let outcome = match self.runtime.step_job(instance, job, job_budget) {
                Ok(step) => ShardOutcome::Job { actor: actor_id, job, step },
                Err(fault) => ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) },
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
}
