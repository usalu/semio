
use super::*;
use crate::{GuestRuntime, JobStep, MockGuestRuntime, PackageHash, PackageId, PackageRef};
use semio_framework::kernel::{Budget, Effect, JobPlacement};
use semio_framework_actor::{ActorId, Envelope, JobOperation, JobReplayRequest, JobTurn, Payload, ShardKind, ShardTable};
use semio_framework_async::{ProcessKind, WorkerPoolConfig};
use std::time::Duration;

/// 🧵️ Verifies the language-neutral stack-authority fixture against every shard owner
/// registry and the native executor values moved across asynchronous boundaries.
#[semio_framework_async_macros::async_test]
async fn shard_stack_authority_matches_the_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧱️stack-authority.json")).expect("stack authority fixture");
    let maximum = fixture["maximumInlineBytes"].as_u64().expect("maximum inline bytes") as usize;
    let registry = |id: &str| fixture["registries"].as_array().expect("registry rows").iter().find(|row| row["id"] == id).expect("stack authority row");
    let seeds = registry("shard-replay-seeds");
    let refusals = registry("shard-replay-refusals");
    let deferred = registry("shard-deferred-owner-ring");
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(seeds["capacity"].as_u64(), Some(super::super::JOB_REPLAY_SEED_SLOT_CAPACITY as u64));
    assert_eq!(refusals["capacity"].as_u64(), Some(super::super::JOB_REPLAY_REFUSAL_SLOT_CAPACITY as u64));
    assert_eq!(deferred["capacity"].as_u64(), Some(SHARD_DEFERRED_ITEMS as u64));
    assert!([seeds, refusals, deferred].iter().all(|row| row["storage"] == "heap"));
    let (_, shard_side) = ThreadTransport::new_pair().await;
    let shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await))), ShardTransports::SharedThread(SharedThreadTransport(Arc::new(shard_side)))).await;
    assert_eq!(shard.replay_seeds.len(), super::super::JOB_REPLAY_SEED_SLOT_CAPACITY);
    assert_eq!(shard.replay_seed_refusals.len(), super::super::JOB_REPLAY_REFUSAL_SLOT_CAPACITY);
    assert_eq!(FixedOwnerRing::<u8, SHARD_DEFERRED_ITEMS>::new(SHARD_DEFERRED_BYTES).slots.len(), SHARD_DEFERRED_ITEMS);
    assert!(std::mem::size_of::<ShardLoop>() <= maximum);
    assert!(std::mem::size_of::<ShardExecutorState>() <= maximum);
    assert!(std::mem::size_of::<ShardExecutor>() <= maximum);
}

async fn encode_frame(frame: super::super::ShardFrame) -> Vec<u8> {
    let mut bytes = Vec::new();
    frame.pack_encode(&mut bytes).await;
    bytes
}

#[semio_framework_async_macros::async_test]
async fn registration_acknowledgement_and_terminal_refusal_preserve_exact_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔁️lifecycle/🧫️fixture/🔣️.json")).unwrap();
    for row in fixture["registrationStages"].as_array().unwrap() {
        let stage = row.as_str().unwrap();
        let mock = Arc::new(MockGuestRuntime::new().await);
        let runtime = Arc::new(GuestRuntimes::Mock(Arc::clone(&mock)));
        let pool = test_pool();
        let executor = ShardExecutor::new(Arc::clone(&pool), runtime, Vec::new(), OutcomeSink::new()).await;
        let actor = ActorId(717);
        let package = PackageRef { package: PackageId("registration-owner".into()), hash: PackageHash([0; 32]) };
        let compiled = mock.compile(&package, &[]).await.unwrap();
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.unwrap();
        if stage == "wrong-actor" {
            let RegistrationAdmission::Refused(owner) = executor.register(ActorId(718), instance).await else { panic!("mismatched guest identity must return its exact owner") };
            assert_eq!(owner.actor, ActorId(718));
            assert_eq!(owner.instance.actor, actor);
            assert_eq!(owner.reason, super::super::ShardRegistrationReason::WrongActor);
            {
                let state = executor.state.lock().unwrap();
                let shard = state.shard.as_ref().unwrap();
                assert!(shard.current_allocation(actor.0).is_none());
                assert!(shard.current_allocation(718).is_none());
                assert_eq!(shard.next_registration, 1);
            }
            mock.drop_instance(owner.instance).await;
        } else if stage == "closed" {
            pool.shutdown().unwrap();
            let RegistrationAdmission::Refused(owner) = executor.register(actor, instance).await else { panic!("closed registration must return its owner") };
            assert_eq!(owner.actor, actor);
            assert_eq!(owner.instance.actor, actor);
            assert_eq!(owner.reason, super::super::ShardRegistrationReason::Stopped);
            mock.drop_instance(owner.instance).await;
        } else {
            executor.scheduled.store(true, Ordering::Release);
            let shard = executor.state.lock().unwrap().shard.take().unwrap();
            let mut pending = Box::pin(executor.register(actor, instance));
            let mut context = std::task::Context::from_waker(std::task::Waker::noop());
            assert!(pending.as_mut().poll(&mut context).is_pending());
            assert_eq!(executor.state.lock().unwrap().registrations.len, 1);
            if stage == "pending" {
                executor.state.lock().unwrap().shard = Some(shard);
                Arc::clone(&executor).run(executor.epoch.load(Ordering::Acquire));
                assert!(matches!(pending.await, RegistrationAdmission::Admitted(_)));
                assert_eq!(executor.state.lock().unwrap().registrations.len, 0);
                assert!(executor.state.lock().unwrap().shard.as_ref().unwrap().current_allocation(actor.0).is_some());
                assert_eq!(mock.drop_admissions.load(Ordering::Acquire), 0);
            } else {
                drop(pending);
                executor.closed.store(true, Ordering::Release);
                executor.refuse_pending_registrations();
                assert_eq!(executor.state.lock().unwrap().registrations.len, 1);
                let (returned_actor, returned) = executor.take_unclaimed_registration().expect("cancelled receiver remains discoverable");
                assert_eq!(returned_actor, actor);
                assert_eq!(returned.actor, actor);
                mock.drop_instance(returned).await;
                executor.state.lock().unwrap().shard = Some(shard);
            }
        }
        assert_eq!(executor.state.lock().unwrap().registrations.bytes, 0);
        assert_eq!(mock.drop_admissions.load(Ordering::Acquire), usize::from(stage != "pending"));
        eprintln!("[DEBUG] registration ownership stage={stage} hidden-credit=0 exact-owner=1");
    }
}

#[semio_framework_async_macros::async_test]
async fn terminal_registration_reply_wakes_outside_the_state_lock() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔁️lifecycle/🧫️fixture/🔣️.json")).unwrap();
    struct Probe {
        executor: std::sync::Weak<ShardExecutor>,
        lockable: AtomicBool,
        woke: AtomicBool,
    }
    impl std::task::Wake for Probe {
        fn wake(self: Arc<Self>) {
            let executor = self.executor.upgrade().unwrap();
            self.lockable.store(executor.state.try_lock().is_ok(), Ordering::Release);
            self.woke.store(true, Ordering::Release);
        }
    }
    let mock = Arc::new(MockGuestRuntime::new().await);
    let executor = ShardExecutor::new(test_pool(), Arc::new(GuestRuntimes::Mock(Arc::clone(&mock))), Vec::new(), OutcomeSink::new()).await;
    executor.scheduled.store(true, Ordering::Release);
    let shard = executor.state.lock().unwrap().shard.take().unwrap();
    let actor = ActorId(718);
    let package = PackageRef { package: PackageId("registration-wake".into()), hash: PackageHash([0; 32]) };
    let compiled = mock.compile(&package, &[]).await.unwrap();
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.unwrap();
    let probe = Arc::new(Probe { executor: Arc::downgrade(&executor), lockable: AtomicBool::new(false), woke: AtomicBool::new(false) });
    let waker = std::task::Waker::from(Arc::clone(&probe));
    let mut context = std::task::Context::from_waker(&waker);
    let mut pending = Box::pin(executor.register(actor, instance));
    assert!(pending.as_mut().poll(&mut context).is_pending());
    executor.closed.store(true, Ordering::Release);
    executor.refuse_pending_registrations();
    assert!(probe.woke.load(Ordering::Acquire));
    assert_eq!(probe.lockable.load(Ordering::Acquire), fixture["terminalWakeOutsideLock"].as_bool().unwrap(), "registration receiver must be able to re-enter the executor");
    let RegistrationAdmission::Refused(owner) = pending.await else { panic!("terminal refusal must return exact owner") };
    assert_eq!(owner.instance.actor, actor);
    mock.drop_instance(owner.instance).await;
    executor.state.lock().unwrap().shard = Some(shard);
    eprintln!("[DEBUG] registration terminal reply inline-wake=1 mutex-free=1 exact-owner=1");
}

#[semio_framework_async_macros::async_test]
async fn admitted_registration_reply_wakes_outside_the_state_lock() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔁️lifecycle/🧫️fixture/🔣️.json")).unwrap();
    struct Probe {
        executor: std::sync::Weak<ShardExecutor>,
        lockable: AtomicBool,
        woke: AtomicBool,
    }
    impl std::task::Wake for Probe {
        fn wake(self: Arc<Self>) {
            let executor = self.executor.upgrade().unwrap();
            self.lockable.store(executor.state.try_lock().is_ok(), Ordering::Release);
            self.woke.store(true, Ordering::Release);
        }
    }
    let mock = Arc::new(MockGuestRuntime::new().await);
    let executor = ShardExecutor::new(test_pool(), Arc::new(GuestRuntimes::Mock(Arc::clone(&mock))), Vec::new(), OutcomeSink::new()).await;
    executor.scheduled.store(true, Ordering::Release);
    let shard = executor.state.lock().unwrap().shard.take().unwrap();
    let actor = ActorId(718);
    let package = PackageRef { package: PackageId("registration-wake".into()), hash: PackageHash([0; 32]) };
    let compiled = mock.compile(&package, &[]).await.unwrap();
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.unwrap();
    let probe = Arc::new(Probe { executor: Arc::downgrade(&executor), lockable: AtomicBool::new(false), woke: AtomicBool::new(false) });
    let waker = std::task::Waker::from(Arc::clone(&probe));
    let mut context = std::task::Context::from_waker(&waker);
    let mut pending = Box::pin(executor.register(actor, instance));
    assert!(pending.as_mut().poll(&mut context).is_pending());
    executor.state.lock().unwrap().shard = Some(shard);
    Arc::clone(&executor).run(executor.epoch.load(Ordering::Acquire));
    assert!(probe.woke.load(Ordering::Acquire));
    assert_eq!(probe.lockable.load(Ordering::Acquire), fixture["admittedWakeOutsideLock"].as_bool().unwrap(), "registration receiver must be able to re-enter the executor");
    let RegistrationAdmission::Admitted(allocation) = pending.await else { panic!("queued registration must acknowledge physical ownership") };
    assert_eq!(allocation.key(semio_framework_actor::ShardId(0)).actor, actor);
    let mut shard = executor.state.lock().unwrap().shard.take().unwrap();
    shard.unregister(actor).await;
    assert_eq!(mock.drop_admissions.load(Ordering::Acquire), 1);
    executor.state.lock().unwrap().shard = Some(shard);
    eprintln!("[DEBUG] registration admitted reply inline-wake=1 mutex-free=1 exact-owner=1");
}

fn test_pool() -> Arc<WorkerPool> {
    Arc::new(WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 4)))
}

fn wait_for_one(outcomes: &Arc<OutcomeSink>) -> ShardOutcome {
    let mut got = outcomes.wait_for(1, Duration::from_secs(2));
    got.pop().unwrap_or_else(|| panic!("no outcome received within the wait window"))
}

/// 🎯️ End-to-end proof: a `ShardExecutor`'s turn is genuinely driven by a `WorkerPool` job (no
/// dedicated thread of its own) in response to `send_frame` — not merely that construction
/// succeeds.
#[semio_framework_async_macros::async_test]
async fn shard_executor_drives_a_turn_for_a_registered_actor_via_the_worker_pool() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = ActorId(101);
    let package = PackageRef { package: PackageId("executor-smoke".to_string()), hash: PackageHash([30u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    let mut scripted = MockGuestRuntime::idle_turn().await;
    scripted.fuel_used = 77;
    mock.script_turn(actor, scripted).await;

    let pool = test_pool();
    let outcomes = OutcomeSink::new();
    let executor = ShardExecutor::new(pool, Arc::new(GuestRuntimes::Mock(mock.clone())), vec![(actor, instance)], outcomes.clone()).await;

    let envelope = Envelope {
        to: actor,
        from: semio_framework_actor::Origin::Kernel,
        lane: semio_framework_actor::Lane::Interactive,
        seq: 1,
        deadline_ms: None,
        coalesce: None,
        cancel_of: None,
        payload: Payload::Event { bytes: serde_json::to_vec(&super::super::fixture_instance_close_event()).unwrap() },
    };
    let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
    let bytes = encode_frame(super::super::ShardFrame::Grant { actor, budget, envelopes: vec![envelope] }).await;
    executor.send_frame(bytes, semio_framework_actor::Lane::Interactive).await;

    match wait_for_one(&outcomes) {
        ShardOutcome::Turn { actor: reported, result } => {
            assert_eq!(reported, actor.0);
            assert_eq!(result.usage.fuel, 77, "the scripted turn's own fuel_used must round-trip through the pool job, proving it genuinely ran pump() there");
        }
        other => panic!("expected ShardOutcome::Turn from the pool job, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn fifo_ingress_selects_interactive_before_earlier_background_without_unbounded_drain() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/♻️relay-lifecycle.json")).expect("neutral relay lifecycle fixture");
    let case = &fixture["ingressCases"][0];
    let frames = case["frames"].as_array().expect("ingress frames");
    let actor_number = |index: usize| u32::try_from(frames[index]["actor"].as_u64().expect("actor")).expect("actor fits");
    let lane = |index: usize| match frames[index]["lane"].as_str().expect("lane") {
        "interactive" => semio_framework_actor::Lane::Interactive,
        "user-visible" => semio_framework_actor::Lane::UserVisible,
        "background" => semio_framework_actor::Lane::Background,
        "maintenance" => semio_framework_actor::Lane::Maintenance,
        other => panic!("unknown neutral lane {other}"),
    };
    let mock = Arc::new(MockGuestRuntime::new().await);
    let background = ActorId::new(0, 0, actor_number(0), 0).await;
    let interactive = ActorId::new(0, 0, actor_number(1), 0).await;
    let package = PackageRef { package: PackageId("executor-lane-priority".to_string()), hash: PackageHash([31u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
    let background_instance = mock.instantiate(&compiled, background, &[], &instantiate_budget).await.expect("background instance");
    let interactive_instance = mock.instantiate(&compiled, interactive, &[], &instantiate_budget).await.expect("interactive instance");
    let mut background_turn = MockGuestRuntime::idle_turn().await;
    background_turn.fuel_used = frames[0]["fuel"].as_u64().expect("background fuel");
    mock.script_turn(background, background_turn).await;
    let mut interactive_turn = MockGuestRuntime::idle_turn().await;
    interactive_turn.fuel_used = frames[1]["fuel"].as_u64().expect("interactive fuel");
    mock.script_turn(interactive, interactive_turn).await;

    let pool = Arc::new(WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1)));
    let (entered_tx, entered_rx) = std::sync::mpsc::sync_channel(1);
    let (release_tx, release_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit(
        PoolLane::Interactive,
        Box::new(move || {
            let _ = entered_tx.send(());
            let _ = release_rx.recv();
        }),
    );
    entered_rx.recv_timeout(Duration::from_secs(2)).expect("pool blocker entered");

    let outcomes = OutcomeSink::new();
    let executor = ShardExecutor::new(pool.clone(), Arc::new(GuestRuntimes::Mock(mock)), vec![(background, background_instance), (interactive, interactive_instance)], outcomes.clone()).await;
    let envelope = |actor, lane, seq| Envelope {
        to: actor,
        from: semio_framework_actor::Origin::Kernel,
        lane,
        seq,
        deadline_ms: None,
        coalesce: None,
        cancel_of: None,
        payload: Payload::Event { bytes: serde_json::to_vec(&super::super::fixture_instance_close_event()).expect("event") },
    };
    let background_lane = lane(0);
    let interactive_lane = lane(1);
    let background_frame = encode_frame(super::super::ShardFrame::Grant {
        actor: background,
        budget: semio_framework_actor::lane_defaults::budget_for(background_lane),
        envelopes: vec![envelope(background, background_lane, frames[0]["sequence"].as_u64().expect("background sequence"))],
    })
    .await;
    let interactive_frame = encode_frame(super::super::ShardFrame::Grant {
        actor: interactive,
        budget: semio_framework_actor::lane_defaults::budget_for(interactive_lane),
        envelopes: vec![envelope(interactive, interactive_lane, frames[1]["sequence"].as_u64().expect("interactive sequence"))],
    })
    .await;
    assert!(matches!(executor.send_frame(background_frame, background_lane).await, FrameIngress::Admitted));
    assert!(matches!(executor.send_frame(interactive_frame, interactive_lane).await, FrameIngress::Admitted));
    release_tx.send(()).expect("release pool blocker");

    let collected = outcomes.wait_for(2, Duration::from_secs(2));
    assert_eq!(collected.len(), 2);
    let expected = case["expectedActors"].as_array().expect("expected ingress actors");
    assert_eq!(case["maxFramesPerDrive"].as_u64(), Some(1));
    assert!(matches!(&collected[0], ShardOutcome::Turn { actor, result } if u64::from(ActorId(*actor).ordinal()) == expected[0].as_u64().expect("first expected actor") && result.usage.fuel == frames[1]["fuel"].as_u64().expect("interactive fuel")));
    assert!(matches!(&collected[1], ShardOutcome::Turn { actor, result } if u64::from(ActorId(*actor).ordinal()) == expected[1].as_u64().expect("second expected actor") && result.usage.fuel == frames[0]["fuel"].as_u64().expect("background fuel")));
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn mounted_fixed_replay_uses_the_same_shard_guest_route_at_one_two_four_and_host_default_workers() {
    let _replay_authority = super::super::replay_test_authority();
    let host_default = std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1);
    for worker_count in [1, 2, 4, host_default] {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(103);
        let job = 107;
        let kind = "action-bus.fixture".to_string();
        let input = vec![1, 3, 5, 7];
        let request = JobReplayRequest::from_spawn(&kind, &input);
        let package = PackageRef { package: PackageId(format!("replay-workers-{worker_count}")), hash: PackageHash([worker_count as u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        let mut scripted = MockGuestRuntime::idle_turn().await;
        scripted.effects.push(Effect::SpawnJob { job, kind, input, placement: JobPlacement::Inline });
        mock.script_turn(actor, scripted).await;

        let pool = Arc::new(WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, worker_count)));
        let outcomes = OutcomeSink::new();
        let executor = ShardExecutor::new(pool.clone(), Arc::new(GuestRuntimes::Mock(mock.clone())), vec![(actor, instance)], outcomes.clone()).await;
        let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
        let event = Envelope {
            to: actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: serde_json::to_vec(&semio_framework::kernel::Event::Wake).expect("wake") },
        };
        let submit_started = std::time::Instant::now();
        executor.send_frame(encode_frame(super::super::ShardFrame::Grant { actor, budget, envelopes: vec![event] }).await, semio_framework_actor::Lane::Interactive).await;
        assert!(submit_started.elapsed() < Duration::from_millis(8), "one mounted replay ingress opportunity exceeded 8ms");
        assert!(matches!(wait_for_one(&outcomes), ShardOutcome::Turn { actor: reported, .. } if reported == actor.0));

        let ready_deadline = std::time::Instant::now() + Duration::from_secs(2);
        let mut ready = false;
        while std::time::Instant::now() < ready_deadline {
            ready = executor.state.lock().unwrap_or_else(PoisonError::into_inner).shard.as_ref().is_some_and(|shard| shard.running_jobs.contains(&(actor.0, job)));
            if ready {
                break;
            }
            std::thread::yield_now();
        }
        assert!(ready, "fixed replay seed activation did not reach the mounted running set");
        let operation = JobOperation { operation: 1, base_revision: 0, generation: u64::from(actor.generation()) + 1, preview_sequence: 0, seed: 1u64.rotate_left(17) ^ actor.0 ^ job };
        let turn = JobTurn { job, operation, step_sequence: 0 };
        mock.script_job_step(actor, JobStep::Running { progress: Some(vec![11, 13]) }).await;
        let step = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep { turn } };
        executor.send_frame(encode_frame(super::super::ShardFrame::Grant { actor, budget, envelopes: vec![step] }).await, semio_framework_actor::Lane::Interactive).await;
        let original = wait_for_one(&outcomes);
        assert!(matches!(&original, ShardOutcome::Job { request: observed, publication, .. } if *observed == request && matches!(&publication.outcome, semio_framework_actor::JobStepOutcome::PreviewReady { preview } if *preview == [11, 13])));

        let replay = Envelope {
            to: actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: 3,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::JobReplay { turn, request, worker_count: u16::try_from(worker_count).expect("worker count"), worker_slot: 0 },
        };
        executor.send_frame(encode_frame(super::super::ShardFrame::Grant { actor, budget, envelopes: vec![replay] }).await, semio_framework_actor::Lane::Interactive).await;
        assert!(matches!(wait_for_one(&outcomes), ShardOutcome::Resumed { actor: reported, operation: observed } if reported == actor.0 && observed == operation));
        mock.script_job_step(actor, JobStep::Done { output: vec![11, 13] }).await;
        mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
        let replay_step = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq: 4, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep { turn } };
        executor.send_frame(encode_frame(super::super::ShardFrame::Grant { actor, budget, envelopes: vec![replay_step] }).await, semio_framework_actor::Lane::Interactive).await;
        let replayed = outcomes.wait_for(2, Duration::from_secs(2));
        assert!(
                replayed.iter().any(|outcome| matches!(outcome, ShardOutcome::Job { request: observed, publication, .. } if *observed == request && matches!(&publication.outcome, semio_framework_actor::JobStepOutcome::Complete { candidate } if candidate.output == [11, 13]))),
                "replay step did not emit the expected terminal job publication: {replayed:?}"
            );
        assert!(replayed.iter().any(|outcome| matches!(outcome, ShardOutcome::Turn { actor: reported, .. } if *reported == actor.0)), "replay step did not emit the actor turn: {replayed:?}");

        executor.send_frame(encode_frame(super::super::ShardFrame::Unregister { actor }).await, semio_framework_actor::Lane::Maintenance).await;
        let close_deadline = std::time::Instant::now() + Duration::from_secs(2);
        let mut terminal = false;
        while std::time::Instant::now() < close_deadline {
            terminal = executor.state.lock().unwrap_or_else(PoisonError::into_inner).shard.as_ref().is_some_and(|shard| shard.replay_seeds.iter().all(Option::is_none));
            if terminal {
                break;
            }
            std::thread::yield_now();
        }
        assert!(terminal, "unregister must incrementally close every retained replay owner");
        pool.shutdown();
    }
}

/// 🎯️ PROPERTY (terra-shard-routing, re-verified under the pool-scheduled model): for K real
/// `ShardExecutor`s and N actors pinned by the SAME [`ShardTable::pin`] a real `Kernel::activate`
/// uses, every actor's own `Grant` must arrive at a shard where THAT actor is already registered
/// — never `ShardOutcome::Fault`. `register` is now synchronous under `state`'s own mutex (no
/// ack rendezvous needed — see `ShardExecutor::register`'s own doc for why the old race cannot
/// recur), so this reproduces the identical zero-slack register-then-dispatch pipeline
/// `NativeKernelRuntime::activate` + `tick_and_dispatch` produce in production.
#[semio_framework_async_macros::async_test]
async fn every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards() {
    const SHARDS: u16 = 4;
    const ACTORS: usize = 200;

    let mock = Arc::new(MockGuestRuntime::new().await);
    let pool = test_pool();
    let outcomes = OutcomeSink::new();
    let mut executors = Vec::new();
    for _ in 0..SHARDS {
        executors.push(ShardExecutor::new(pool.clone(), Arc::new(GuestRuntimes::Mock(mock.clone())), Vec::new(), outcomes.clone()).await);
    }
    let mut shards = ShardTable::new(ShardKind::Native, SHARDS, 0).await;
    let package = PackageRef { package: PackageId("grant-routing-property".to_string()), hash: PackageHash([55u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
    let grant_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);

    for i in 0..ACTORS {
        let actor = ActorId::new(0, 0, i as u32, 0).await;
        let shard_id = shards.pin(actor).await;
        let instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate");
        mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
        executors[shard_id.0 as usize].register(actor, instance).await;
        let envelope = Envelope {
            to: actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: i as u64 + 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: serde_json::to_vec(&super::super::fixture_instance_close_event()).unwrap() },
        };
        let bytes = encode_frame(super::super::ShardFrame::Grant { actor, budget: grant_budget, envelopes: vec![envelope] }).await;
        executors[shard_id.0 as usize].send_frame(bytes, semio_framework_actor::Lane::Interactive).await;
    }

    let collected = outcomes.wait_for(ACTORS, Duration::from_secs(10));
    assert_eq!(collected.len(), ACTORS, "every one of {ACTORS} actors must produce exactly one outcome");
    for outcome in collected {
        match outcome {
            ShardOutcome::Turn { .. } => {}
            other => panic!("expected Turn for every actor, got {other:?} — every actor's own Grant must arrive at a shard where it is already registered"),
        }
    }
}

/// 🎯️ PROPERTY: a suspend→resume round trip must land back on a shard where the actor is
/// registered, under the pool-scheduled model.
#[semio_framework_async_macros::async_test]
async fn suspend_then_resume_round_trip_lands_on_a_shard_where_the_actor_is_registered() {
    const SHARDS: u16 = 4;
    const ACTORS: usize = 60;

    let mock = Arc::new(MockGuestRuntime::new().await);
    let pool = test_pool();
    let outcomes = OutcomeSink::new();
    let mut executors = Vec::new();
    for _ in 0..SHARDS {
        executors.push(ShardExecutor::new(pool.clone(), Arc::new(GuestRuntimes::Mock(mock.clone())), Vec::new(), outcomes.clone()).await);
    }
    let mut shards = ShardTable::new(ShardKind::Native, SHARDS, 0).await;
    let package = PackageRef { package: PackageId("suspend-resume-property".to_string()), hash: PackageHash([77u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };

    for i in 0..ACTORS {
        let actor = ActorId::new(0, 0, i as u32, 0).await;
        let shard_id = shards.pin(actor).await;
        let executor = &executors[shard_id.0 as usize];
        let instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate");
        executor.register(actor, instance).await;

        let operation = JobOperation { operation: actor.0, base_revision: 0, generation: actor.generation() as u64, preview_sequence: 0, seed: actor.0 };
        let suspend = Envelope {
            to: actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Background,
            seq: 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Suspend { operation, applied_progress: i as u64 },
        };
        executor.send_frame(encode_frame(super::super::ShardFrame::Envelope(suspend)).await, semio_framework_actor::Lane::Background).await;
        let state = match wait_for_one(&outcomes) {
            ShardOutcome::Checkpoint { actor: reported, operation: reported_operation, checkpoint } => {
                assert_eq!(reported, actor.0);
                assert_eq!(reported_operation, operation);
                assert_eq!(checkpoint.applied_progress, i as u64);
                checkpoint
            }
            other => panic!("actor {i}: expected Checkpoint outcome for Suspend, got {other:?} — a just-registered actor's own Suspend must never fault"),
        };

        let retire = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Cancel { seq: 2 } };
        assert!(matches!(executor.send_frame(encode_frame(super::super::ShardFrame::Envelope(retire)).await, semio_framework_actor::Lane::Background).await, FrameIngress::Admitted));
        assert!(matches!(wait_for_one(&outcomes), ShardOutcome::Cancelled { actor: retired } if retired == actor.0));
        let fresh_instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate (fresh)");
        assert!(matches!(executor.register(actor, fresh_instance).await, RegistrationAdmission::Admitted(_)));
        let resume =
            Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Resume { operation, checkpoint: state } };
        executor.send_frame(encode_frame(super::super::ShardFrame::Envelope(resume)).await, semio_framework_actor::Lane::Background).await;
        match wait_for_one(&outcomes) {
            ShardOutcome::Resumed { actor: reported, operation: reported_operation } => {
                assert_eq!(reported, actor.0);
                assert_eq!(reported_operation, operation);
            }
            other => panic!("actor {i}: expected Resumed outcome, got {other:?} — a Resume dispatched right after register() must find the actor already registered, never fault"),
        }
    }
}

/// 🎯️ A burst of `send_frame` calls that all race BEFORE the first pump job even starts must
/// still each produce exactly one outcome — the single-flight `scheduled`/`epoch` protocol must
/// neither drop a frame nor deadlock when every caller sees `scheduled` already true.
#[semio_framework_async_macros::async_test]
async fn concurrent_send_frame_bursts_never_drop_an_outcome() {
    const ACTORS: usize = 32;
    let mock = Arc::new(MockGuestRuntime::new().await);
    let pool = test_pool();
    let outcomes = OutcomeSink::new();
    let package = PackageRef { package: PackageId("burst-property".to_string()), hash: PackageHash([9u8; 32]) };
    let compiled = mock.compile(&package, &[]).await.expect("mock compile");
    let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
    let mut initial = Vec::new();
    let mut actors = Vec::new();
    for i in 0..ACTORS {
        let actor = ActorId::new(0, 0, i as u32, 0).await;
        let instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate");
        mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
        initial.push((actor, instance));
        actors.push(actor);
    }
    let executor = ShardExecutor::new(pool, Arc::new(GuestRuntimes::Mock(mock.clone())), initial, outcomes.clone()).await;
    let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);

    let mut handles = Vec::new();
    for (i, actor) in actors.into_iter().enumerate() {
        let executor = Arc::clone(&executor);
        handles.push(std::thread::spawn(move || {
            let envelope = Envelope {
                to: actor,
                from: semio_framework_actor::Origin::Kernel,
                lane: semio_framework_actor::Lane::Interactive,
                seq: i as u64 + 1,
                deadline_ms: None,
                coalesce: None,
                cancel_of: None,
                payload: Payload::Event { bytes: serde_json::to_vec(&super::super::fixture_instance_close_event()).unwrap() },
            };
            let bytes = semio_framework_async::block_on(encode_frame(super::super::ShardFrame::Grant { actor, budget, envelopes: vec![envelope] }));
            semio_framework_async::block_on(executor.send_frame(bytes, semio_framework_actor::Lane::Interactive));
        }));
    }
    for handle in handles {
        handle.join().expect("sender thread panicked");
    }

    let collected = outcomes.wait_for(ACTORS, Duration::from_secs(10));
    assert_eq!(collected.len(), ACTORS, "every concurrently-sent frame must still produce exactly one outcome — none dropped by the single-flight race");
}

#[test]
fn pending_drive_wake_and_wake_storm_claim_exactly_one_schedule() {
    let queued = AtomicBool::new(false);
    assert!(claim_drive_wake(7, 7, &queued), "the first matching readiness wake owns scheduling");
    for _ in 0..64 {
        assert!(!claim_drive_wake(7, 7, &queued), "a wake storm cannot claim a second schedule");
    }
    queued.store(false, Ordering::Release);
    assert!(!claim_drive_wake(8, 7, &queued), "a stale drive generation cannot schedule or mutate the current cursor");
    assert!(!queued.load(Ordering::Acquire));
}

#[test]
fn retry_trigger_is_one_shot_until_the_generation_owner_releases_it() {
    let armed = AtomicBool::new(false);
    assert!(claim_one_shot(&armed));
    assert!(!claim_one_shot(&armed), "concurrent saturation notices coalesce behind one timer authority");
    armed.store(false, Ordering::Release);
    assert!(claim_one_shot(&armed), "the callback may arm one later bounded generation");
}

#[semio_framework_async_macros::async_test]
async fn terminal_ingress_returns_the_exact_late_frame_before_transport_or_epoch_mutation() {
    let pool = test_pool();
    let executor = ShardExecutor::new(pool.clone(), Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await))), Vec::new(), OutcomeSink::new()).await;
    pool.shutdown();
    let raw = vec![4, 3, 2, 1];
    let owner = match executor.send_frame(raw.clone(), semio_framework_actor::Lane::Maintenance).await {
        FrameIngress::Rejected(owner) => owner,
        FrameIngress::Admitted => panic!("shutdown ingress cannot admit a late frame"),
    };
    assert_eq!(owner.reason(), IngressCloseReason::Shutdown);
    assert_eq!(owner.into_frame(), raw, "late ingress returns the exact original owner");
    assert_eq!(executor.epoch.load(Ordering::Acquire), 0, "rejected late ingress cannot mutate the admitted epoch");
    assert!(executor.kernel_side.try_recv_now().is_none(), "rejected late ingress never reaches transport ownership");
}

#[semio_framework_async_macros::async_test]
async fn over_capacity_ingress_hands_back_bytes_plus_one_exactly() {
    let executor = ShardExecutor::new(test_pool(), Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await))), Vec::new(), OutcomeSink::new()).await;
    let raw = vec![7; SHARD_FRAME_MAX_BYTES + 1];
    let owner = match executor.send_frame(raw, semio_framework_actor::Lane::Maintenance).await {
        FrameIngress::Rejected(owner) => owner,
        FrameIngress::Admitted => panic!("raw Grant credit plus one cannot be admitted"),
    };
    assert_eq!(owner.reason(), IngressCloseReason::OverCapacity);
    assert_eq!(owner.into_frame().len(), SHARD_FRAME_MAX_BYTES + 1);
    assert_eq!(executor.epoch.load(Ordering::Acquire), 0);
    assert!(executor.kernel_side.try_recv_now().is_none());
}

#[semio_framework_async_macros::async_test]
async fn occupied_terminal_overflow_rejects_plus_one_and_plus_two_before_mutation() {
    let executor = ShardExecutor::new(test_pool(), Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await))), Vec::new(), OutcomeSink::new()).await;
    executor.terminal_overflow_occupied.store(true, Ordering::Release);
    let first = vec![0xf1, 1];
    let second = vec![0xf2, 2];
    let first_owner = match executor.send_frame(first.clone(), semio_framework_actor::Lane::Maintenance).await {
        FrameIngress::Rejected(owner) => owner,
        FrameIngress::Admitted => panic!("occupied terminal overflow cannot accept plus one"),
    };
    let second_owner = match executor.send_frame(second.clone(), semio_framework_actor::Lane::Maintenance).await {
        FrameIngress::Rejected(owner) => owner,
        FrameIngress::Admitted => panic!("occupied terminal overflow cannot accept plus two"),
    };
    assert_eq!(first_owner.reason(), IngressCloseReason::TerminalCapacity);
    assert_eq!(first_owner.into_frame(), first, "plus one returns its exact owner");
    assert_eq!(second_owner.reason(), IngressCloseReason::TerminalCapacity);
    second_owner.close();
    assert_eq!(executor.epoch.load(Ordering::Acquire), 0, "overflow rejection accepts no epoch");
    assert!(executor.kernel_side.try_recv_now().is_none(), "overflow rejection mutates no transport owner");
}
