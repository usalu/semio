//! 🏃️ `ShardExecutor` — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants). Owns exactly
//! ONE [`super::ShardLoop`], its [`ThreadTransport`] end, and the shared [`super::GuestRuntime`]
//! handle on a DEDICATED OS thread — `design-runtime.md` §2's "K shards run in parallel" made real.
//! Today every native caller (`KernelThreadState` in the wgpu-native glue, the `semio-shard` child
//! binary) drives a single `ShardLoop` from ITS OWN loop, so K shard labels still serialize behind
//! one physical loop — this is exactly why bench budget 5 ("K shards run in parallel") has been
//! unmeasurable. `ShardExecutor` is the seam a future multi-shard `ShardTable` router wires
//! through — landed ahead of its caller, the same shape `GuestRuntime`/`MockGuestRuntime` landed in
//! before `WasmtimeRuntime` existed.
//!
//! Native-only: [`ThreadTransport`] is `std::sync::mpsc`-backed (host-supplied, per the actor
//! crate's own purity rule — transports live outside that crate's pure core) and this file spawns
//! a REAL OS thread, which is exactly the kind of transport/thread-owning glue that rule pushes out
//! of `🎭️actor` and into this crate.

use super::{ShardLoop, ShardTransports};
use crate::{GuestInstance, GuestRuntimes};
use semio_framework_actor::{ActorId, ShardTransport, ThreadTransport};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

/// ⏳️ How long the loop parks on [`ThreadTransport::recv_deadline`] before re-checking
/// `running_jobs`/`pending_completions` even with nothing new on the wire — job-stepping and
/// queued `Event::JobCompleted` delivery are both entirely self-driven by [`ShardLoop::pump`], so
/// the loop must wake periodically on its own, not only when a frame arrives.
const PARK_TIMEOUT: Duration = Duration::from_millis(5);

/// ⏳️ terra-shard-routing: ceiling on how long [`ShardExecutor::register`] blocks its caller,
/// waiting on the ack described on [`RegisterRequest`] — bounds the wait for a wedged/dead executor
/// thread
/// (the ack `Sender` living inside a still-buffered `RegisterRequest` is dropped, unblocking
/// `recv_timeout` immediately, the instant the executor thread actually exits and drops
/// `register_rx`) without ever hanging `activate` forever. Comfortably above `PARK_TIMEOUT` (the
/// longest a healthy executor ever takes to notice a fresh request) so a live shard never times out.
const REGISTER_ACK_TIMEOUT: Duration = Duration::from_secs(5);

/// 🔌️ Local newtype so [`ShardExecutor::spawn`] can hand [`ShardLoop::new`] an `Arc<ThreadTransport>`
/// as a [`super::ShardTransports::SharedThread`] variant while ALSO keeping its own `Arc` clone to
/// call the concrete, trait-external [`ThreadTransport::recv_deadline`] on the exact same channel
/// for parking — `impl ShardTransport for Arc<ThreadTransport>` directly would hit `E0117`
/// (neither type is local to this crate; `🧵️shard/🦀️component.rs`'s own `LoopbackProbe` doc
/// comment already names this same orphan-rule constraint for `Arc<LoopbackTransport>`).
pub(crate) struct SharedThreadTransport(Arc<ThreadTransport>);

impl ShardTransport for SharedThreadTransport {
    async fn send(&self, bytes: &[u8]) {
        self.0.send(bytes).await;
    }
    async fn recv(&self) -> Option<Vec<u8>> {
        self.0.recv().await
    }
    async fn heartbeat(&self) -> u64 {
        self.0.heartbeat().await
    }
    async fn kill(&self) {
        self.0.kill().await;
    }
}

/// 🆕️ terra-shard-routing: one [`ShardExecutor::register`] call, in flight. Carries an `ack` so the
/// caller can BLOCK until the executor thread has actually applied `shard.register(actor,
/// instance)` — see [`ShardExecutor::register`]'s own doc for the race this rendezvous closes.
struct RegisterRequest {
    actor: ActorId,
    instance: GuestInstance,
    ack: mpsc::Sender<()>,
}

/// 🏃️ Owns one [`ShardLoop`] on its own dedicated OS thread. `spawn` takes ownership of one end of
/// a [`ThreadTransport::new_pair`] duplex link; the caller keeps the mirror end to send
/// [`super::ShardFrame`]s (`Register`/`Unregister`/`Grant`/`Envelope`) and receive
/// `super::ShardOutcome`s — exactly the same wire `KernelThreadState` (wgpu-native glue) already
/// drives a single `ShardLoop` over today, just now with the loop itself living on its own thread.
pub struct ShardExecutor {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    /// 🆕️ terra-kernel-loop: lets a caller hand a FRESH `GuestInstance` to an ALREADY-RUNNING
    /// executor — `spawn`'s own `initial` list only covers actors known before the thread starts;
    /// the real native kernel loop (`🎯️targets/🧊️wgpu/🎠️runtime.rs`) activates actors continuously
    /// (every `Kernel::activate` call, e.g. a plugin app opened mid-session) and each one must be
    /// registered on whichever shard `ShardTable::pin` already assigned it to. `GuestInstance` is
    /// `Send` (this file's own `initial: Vec<(ActorId, GuestInstance)>` parameter already proves
    /// that by moving it into the spawned thread), so handing one across an ordinary channel is
    /// exactly as sound as `spawn`'s existing up-front handoff — just later.
    ///
    /// 🐛️ terra-shard-routing: carries a [`RegisterRequest`], not a bare `(ActorId, GuestInstance)`
    /// tuple anymore — see [`ShardExecutor::register`]'s doc for why the ack is required.
    register_tx: mpsc::Sender<RegisterRequest>,
}

impl ShardExecutor {
    /// ▶️ Spawns the thread and starts pumping immediately. `initial` is registered on the
    /// `ShardLoop` BEFORE the park/pump loop starts — a `GuestInstance` cannot be registered from
    /// OUTSIDE once the loop owns it (it lives entirely on the executor's own thread from that
    /// point on, mirroring how `KernelThreadState` — the wgpu-native host's single-`ShardLoop`
    /// design — also only ever calls `ShardLoop::register` from the SAME thread that owns the
    /// loop), so any actor this executor must serve is instantiated by the caller and handed in
    /// here, up front. For actors activated LATER, see [`Self::register`].
    pub async fn spawn(runtime: Arc<GuestRuntimes>, transport: ThreadTransport, initial: Vec<(ActorId, GuestInstance)>) -> Self {
        let transport = Arc::new(transport);
        let park = transport.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_loop = stop.clone();
        let (register_tx, register_rx) = mpsc::channel::<RegisterRequest>();
        let handle = std::thread::Builder::new()
            .name("semio-shard-executor".to_string())
            .spawn(move || {
                // 👶️ host-dedyn: ONE `block_on` at the thread root — this whole closure body IS
                // the thread's own executor from here down (R4 clause 4: "shard/actor thread roots
                // for as long as a thread-loop backend exists"). `park.recv_deadline` below is a
                // genuinely blocking `mpsc` call, sound to run inside a `block_on`'d future because
                // nothing else needs to make progress on this thread while it waits. Construction
                // and the up-front `initial` registration moved inside this same block so
                // `ShardLoop::new`/`register` (both `async` post-conversion) have somewhere to
                // `.await` — still registered BEFORE the park/pump loop starts, unchanged semantics.
                semio_framework_async::block_on(async {
                    let mut shard = ShardLoop::new(runtime, ShardTransports::SharedThread(SharedThreadTransport(transport))).await;
                    for (actor, instance) in initial {
                        shard.register(actor, instance).await;
                    }
                    while !stop_loop.load(Ordering::SeqCst) {
                        // 🐛️ terra-shard-routing: drains every pending `register()` request BEFORE
                        // this iteration's park/pump, applying it AND acking it — this alone does
                        // NOT make a `ShardFrame::Grant` sent right after `register()` returns safe
                        // (the bug this packet fixed): `register_tx`/`register_rx` and the
                        // `ShardFrame` transport are two INDEPENDENT channels, so a request queued
                        // here while THIS thread is already parked at `park.recv_deadline` below
                        // sits undrained until the NEXT iteration — but a `ShardFrame::Grant` for
                        // that same actor, sent moments later, wakes that SAME park call immediately
                        // and reaches `pump_primed` first. The ack in `Self::register` is what
                        // actually closes the race: it makes the caller BLOCK until this drain has
                        // run and applied the request, so any frame the caller sends afterward is
                        // real program-order-after the registration, not merely queued "before" it
                        // on an unrelated channel.
                        while let Ok(RegisterRequest { actor, instance, ack }) = register_rx.try_recv() {
                            shard.register(actor, instance).await;
                            let _ = ack.send(());
                        }
                        // 🅿️ Park until a frame arrives or `PARK_TIMEOUT` elapses — `pump_primed`
                        // never blocks on its own (its own drain loop only reads what is ALREADY
                        // buffered), so without this the loop would busy-spin exactly like the
                        // `semio-shard` `[[bin]]`'s own `sleep(5ms)` loop does today.
                        let primed = park.recv_deadline(PARK_TIMEOUT);
                        if let Err(error) = shard.pump_primed(primed.await).await {
                            eprintln!("[shard-executor] pump error: {error}");
                        }
                    }
                });
            })
            .expect("spawn shard executor thread");
        Self { stop, handle: Some(handle), register_tx }
    }

    /// 🆕️ terra-kernel-loop: registers `instance` on this ALREADY-RUNNING executor's `ShardLoop`,
    /// from any thread — see the `register_tx` field doc for why this is sound.
    ///
    /// 🐛️ terra-shard-routing (fix): BLOCKS until the executor thread has actually applied the
    /// registration (bounded by [`REGISTER_ACK_TIMEOUT`]), instead of firing the request and
    /// returning immediately. The previous fire-and-forget version let `ParallelRuntime::activate`
    /// return before the shard thread had drained `register_rx`, so a caller that immediately
    /// `submit`+`tick_and_dispatch`ed a `ShardFrame::Grant`/`Suspend`/`Resume` for that SAME actor
    /// could have it reach `ShardLoop::pump_primed` first — the executor thread was parked on
    /// `ThreadTransport::recv_deadline` (a channel `register_tx` never wakes), so the Grant arrived,
    /// got pumped, and found no entry in `shard.instances` yet: `"actor N is not registered on this
    /// shard"`. Blocking for the ack turns "queued on an unrelated channel before" into a genuine
    /// happens-before the caller's own next send — see the `spawn` loop's own doc for the exact
    /// interleaving this closes. Silently returns (mirroring `ShardFrame` sends over a `killed`
    /// transport) if the executor thread has already stopped or the wait exceeds
    /// `REGISTER_ACK_TIMEOUT`; a caller that needs to know registration succeeded should check
    /// [`Self::is_running`] first.
    pub async fn register(&self, actor: ActorId, instance: GuestInstance) {
        let (ack, ack_rx) = mpsc::channel();
        if self.register_tx.send(RegisterRequest { actor, instance, ack }).is_err() {
            return;
        }
        let _ = ack_rx.recv_timeout(REGISTER_ACK_TIMEOUT);
    }

    /// ⏹️ Signals the loop to stop after its current `pump_primed` call (bounded by
    /// `PARK_TIMEOUT`, so this returns promptly) and joins the thread. [`Drop`] calls this too, so
    /// a `ShardExecutor` going out of scope never leaks a running thread.
    // 🚫️async: E1 pure sync bookkeeping (`AtomicBool::store` + a genuinely-blocking
    // `JoinHandle::join`) — zero suspension points, no `.await` anywhere in this body even before
    // this tag. Its sole production caller is `impl Drop for ShardExecutor` below, which per R2 is
    // language-fixed sync (the external `Drop` trait cannot be async) and so cannot bridge with
    // `.await` — R9: reverted to sync rather than wrapping the Drop call in `block_on` for a future
    // that carries no real suspension to bridge in the first place.
    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }

    /// 🩺️ Whether the executor thread is still running — `false` once `stop()` has joined it.
    pub async fn is_running(&self) -> bool {
        self.handle.is_some()
    }
}

impl Drop for ShardExecutor {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GuestRuntime, MockGuestRuntime, PackageHash, PackageId, PackageRef};
    use semio_framework::kernel::Budget;
    use semio_framework_actor::{ActorId, Envelope, Payload, ShardKind, ShardTable};

    async fn encode_frame(frame: super::super::ShardFrame) -> Vec<u8> {
        let mut bytes = Vec::new();
        frame.pack_encode(&mut bytes).await;
        bytes
    }

    /// 🎯️ End-to-end proof: a `ShardExecutor` running on its OWN thread actually drives a turn for
    /// an actor registered on it (via `spawn`'s `initial` list), in response to a `Grant` sent
    /// over the transport — not merely that the thread starts and stops cleanly.
    #[semio_framework_async_macros::async_test]
    async fn shard_executor_drives_a_turn_for_a_registered_actor_from_its_own_thread() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(101);
        let package = PackageRef { package: PackageId("executor-smoke".to_string()), hash: PackageHash([30u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        let mut scripted = MockGuestRuntime::idle_turn().await;
        scripted.fuel_used = 77;
        mock.script_turn(actor, scripted).await;

        let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
        let executor = ShardExecutor::spawn(Arc::new(GuestRuntimes::Mock(mock.clone())), shard_side, vec![(actor, instance)]).await;
        assert!(executor.is_running().await);

        let envelope = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq: 1, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(&semio_framework::kernel::Event::InstanceClose).unwrap() } };
        let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
        semio_framework_async::block_on(kernel_side.send(&semio_framework_async::block_on(encode_frame(super::super::ShardFrame::Grant { actor, budget: budget.await, envelopes: vec![envelope] }))));

        let outcome_bytes = recv_with_retries(&kernel_side, 200).await;
        let outcome: super::super::ShardOutcome = serde_json::from_slice(&outcome_bytes).expect("decode outcome");
        match outcome {
            super::super::ShardOutcome::Turn { actor: reported, result } => {
                assert_eq!(reported, actor.0);
                assert_eq!(result.fuel_used, 77, "the scripted turn's own fuel_used must round-trip through the executor's own thread, proving it genuinely ran pump_primed there, not merely spawned and sat idle");
            }
            other => panic!("expected ShardOutcome::Turn from the executor's own thread, got {other:?}"),
        }
    }

    async fn recv_with_retries(transport: &ThreadTransport, attempts: u32) -> Vec<u8> {
        for _ in 0..attempts {
            if let Some(bytes) = transport.recv_deadline(Duration::from_millis(20)).await {
                return bytes;
            }
        }
        panic!("no outcome received within {attempts} retries");
    }

    /// 🎯️ PROPERTY (terra-shard-routing): for K real `ShardExecutor` threads and N actors pinned by
    /// the SAME [`ShardTable::pin`] a real `Kernel::activate` uses, every actor's own `Grant` must
    /// arrive at a shard where THAT actor is already registered — never `ShardOutcome::Fault`.
    /// Reproduces the exact pipeline `ParallelRuntime::activate` (register) immediately followed by
    /// `submit`+`tick_and_dispatch` (Grant) produces in production, with no artificial delay between
    /// the two — before this packet's fix (`ShardExecutor::register` returning before the executor
    /// thread had actually applied it), this reliably reproduced budget 2/3's own bench faults
    /// ("actor N is not registered on this shard") because a `Grant` sent right after `register()`
    /// returned could reach a still-parked executor thread's `pump_primed` BEFORE that thread's next
    /// loop iteration drained the registration — see `Self::register`'s own doc for the full
    /// interleaving. A mechanism test asserting one hand-timed round trip would not have caught this
    /// (it did not, twice, per this ticket's own `important.md`) — this drives many actors back to
    /// back across several shards with zero slack, which is what actually exercises the race window.
    #[semio_framework_async_macros::async_test]
    async fn every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards() {
        const SHARDS: u16 = 4;
        const ACTORS: usize = 200;

        let mock = Arc::new(MockGuestRuntime::new().await);
        let mut kernel_sides = Vec::new();
        let mut executors = Vec::new();
        for _ in 0..SHARDS {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            executors.push(ShardExecutor::spawn(Arc::new(GuestRuntimes::Mock(mock.clone())), shard_side, Vec::new()).await);
            kernel_sides.push(kernel_side);
        }
        let mut shards = ShardTable::new(ShardKind::Thread, SHARDS, 0).await;
        let package = PackageRef { package: PackageId("grant-routing-property".to_string()), hash: PackageHash([55u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let grant_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive).await;

        let mut expected_shard = Vec::with_capacity(ACTORS);
        for i in 0..ACTORS {
            let actor = ActorId::new(0, 0, i as u32, 0).await;
            let shard_id = shards.pin(actor).await;
            expected_shard.push(shard_id);
            let instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate");
            mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
            // ⏳️ Synchronous register (this packet's fix), THEN an immediate dispatch — no sleep, no
            // retry, matching real production ordering exactly.
            executors[shard_id.0 as usize].register(actor, instance).await;
            let envelope = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq: i as u64 + 1, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(&semio_framework::kernel::Event::InstanceClose).unwrap() } };
            semio_framework_async::block_on(kernel_sides[shard_id.0 as usize].send(&semio_framework_async::block_on(encode_frame(super::super::ShardFrame::Grant { actor, budget: grant_budget, envelopes: vec![envelope] }))));
        }

        for (i, shard_id) in expected_shard.into_iter().enumerate() {
            let actor = ActorId::new(0, 0, i as u32, 0).await;
            let bytes = recv_with_retries(&kernel_sides[shard_id.0 as usize], 200).await;
            let outcome: super::super::ShardOutcome = serde_json::from_slice(&bytes).expect("decode outcome");
            match outcome {
                super::super::ShardOutcome::Turn { actor: reported, .. } => assert_eq!(reported, actor.0, "actor {i}'s own Grant must resolve to a Turn for ITSELF"),
                other => panic!("actor {i} on shard {}: expected Turn, got {other:?} — every actor's own Grant must arrive at a shard where it is already registered", shard_id.0),
            }
        }
    }

    /// 🎯️ PROPERTY (terra-shard-routing): a suspend→resume round trip must land back on a shard
    /// where the actor is registered. Mirrors the wgpu bench's own budget 7 shape (`glue.rs`'s
    /// `budget_7_stateful`): checkpoint an actor, then register a FRESH instance for the SAME actor
    /// id (the "resumed elsewhere" half of LRU-suspend) and dispatch `Payload::Resume` immediately —
    /// with the pre-fix async `register()`, this could reach the executor thread before the fresh
    /// instance was applied, producing exactly budget 7's `resumed: false` (a `ShardOutcome::Fault`
    /// where a `Resumed` was expected) rather than a genuine restore failure.
    #[semio_framework_async_macros::async_test]
    async fn suspend_then_resume_round_trip_lands_on_a_shard_where_the_actor_is_registered() {
        const SHARDS: u16 = 4;
        const ACTORS: usize = 60;

        let mock = Arc::new(MockGuestRuntime::new().await);
        let mut kernel_sides = Vec::new();
        let mut executors = Vec::new();
        for _ in 0..SHARDS {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            executors.push(ShardExecutor::spawn(Arc::new(GuestRuntimes::Mock(mock.clone())), shard_side, Vec::new()).await);
            kernel_sides.push(kernel_side);
        }
        let mut shards = ShardTable::new(ShardKind::Thread, SHARDS, 0).await;
        let package = PackageRef { package: PackageId("suspend-resume-property".to_string()), hash: PackageHash([77u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };

        for i in 0..ACTORS {
            let actor = ActorId::new(0, 0, i as u32, 0).await;
            let shard_id = shards.pin(actor).await;
            let kernel_side = &kernel_sides[shard_id.0 as usize];
            let instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate");
            executors[shard_id.0 as usize].register(actor, instance).await;

            let suspend = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: 1, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Suspend { checkpoint: true } };
            semio_framework_async::block_on(kernel_side.send(&semio_framework_async::block_on(encode_frame(super::super::ShardFrame::Envelope(suspend)))));
            let outcome: super::super::ShardOutcome = serde_json::from_slice(&recv_with_retries(kernel_side, 200).await).expect("decode outcome");
            let state = match outcome {
                super::super::ShardOutcome::Checkpoint { actor: reported, state } => {
                    assert_eq!(reported, actor.0);
                    state
                }
                other => panic!("actor {i}: expected Checkpoint outcome for Suspend, got {other:?} — a just-registered actor's own Suspend must never fault"),
            };

            // The "resumed elsewhere" half: a FRESH instance for the SAME actor id, registered and
            // immediately resumed — no slack, the exact interleaving that used to race.
            let fresh_instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate (fresh)");
            executors[shard_id.0 as usize].register(actor, fresh_instance).await;
            let resume = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Resume { checkpoint: Some(state) } };
            semio_framework_async::block_on(kernel_side.send(&semio_framework_async::block_on(encode_frame(super::super::ShardFrame::Envelope(resume)))));
            let outcome: super::super::ShardOutcome = serde_json::from_slice(&recv_with_retries(kernel_side, 200).await).expect("decode outcome");
            match outcome {
                super::super::ShardOutcome::Resumed { actor: reported } => assert_eq!(reported, actor.0),
                other => panic!("actor {i}: expected Resumed outcome, got {other:?} — a Resume dispatched right after register() must find the actor already registered, never fault"),
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn stop_joins_the_thread_and_is_idempotent_with_drop() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let (_kernel_side, shard_side) = ThreadTransport::new_pair().await;
        let mut executor = ShardExecutor::spawn(Arc::new(GuestRuntimes::Mock(mock)), shard_side, vec![]).await;
        assert!(executor.is_running().await);
        executor.stop();
        assert!(!executor.is_running().await, "stop() must join the thread");
        // 🎯️ A second `stop()` (and then `Drop`) must not panic — `handle.take()` makes both a
        // no-op once the thread is already joined.
        executor.stop();
    }
}
