
use super::*;
use {NullEmit, Profile};

fn generous_capacities() -> MailboxCapacities {
    MailboxCapacities::uniform(64)
}

//#region 🔖️Mailbox
#[test]
fn system_and_recovery_lanes_drain_strictly_ahead_of_the_drr_lanes() {
    let (address, receiver) = mailbox::<&'static str>(generous_capacities());
    address.try_send(Priority::Preview, "preview").unwrap();
    address.try_send(Priority::Command, "command").unwrap();
    address.try_send(Priority::Recovery, "recovery").unwrap();
    address.try_send(Priority::System, "system").unwrap();

    assert_eq!(receiver.try_recv().unwrap().payload, "system");
    assert_eq!(receiver.try_recv().unwrap().payload, "recovery");
    assert_eq!(receiver.try_recv().unwrap().payload, "command");
    assert_eq!(receiver.try_recv().unwrap().payload, "preview");
    assert!(receiver.try_recv().is_none());
}

#[test]
fn drr_serves_a_lane_a_run_of_messages_proportional_to_its_weight() {
    let (address, receiver) = mailbox::<(Priority, u32)>(generous_capacities());
    for i in 0..32 {
        address.try_send(Priority::Command, (Priority::Command, i)).unwrap();
        address.try_send(Priority::Query, (Priority::Query, i)).unwrap();
    }

    // Command's weight (16) is exactly double Query's (8): the DRR cursor must drain 16
    // consecutive Command messages before it ever yields to Query, then 8 consecutive Query
    // messages before wrapping back — this is deterministic given the implementation, not a
    // statistical approximation, so assert the exact boundary.
    for _ in 0..16 {
        assert_eq!(receiver.try_recv().unwrap().priority, Priority::Command);
    }
    for _ in 0..8 {
        assert_eq!(receiver.try_recv().unwrap().priority, Priority::Query);
    }
    for _ in 0..16 {
        assert_eq!(receiver.try_recv().unwrap().priority, Priority::Command);
    }
}

#[test]
fn preview_lane_sheds_its_own_oldest_message_and_never_blocks_or_errors() {
    let mut capacities = MailboxCapacities::uniform(64);
    capacities.set(Priority::Preview, 2);
    let (address, receiver) = mailbox::<u32>(capacities);

    for value in 0..3 {
        assert!(address.try_send(Priority::Preview, value).is_ok(), "preview sends must never fail under pressure");
    }

    assert_eq!(address.shed_preview_count(), 1);
    assert_eq!(receiver.try_recv().unwrap().payload, 1, "the oldest preview (0) must have been shed");
    assert_eq!(receiver.try_recv().unwrap().payload, 2);
    assert!(receiver.try_recv().is_none());
}

#[test]
fn non_preview_lane_rejects_try_send_when_full_instead_of_shedding() {
    let mut capacities = MailboxCapacities::uniform(64);
    capacities.set(Priority::Command, 1);
    let (address, _receiver) = mailbox::<u32>(capacities);

    address.try_send(Priority::Command, 1).unwrap();
    match address.try_send(Priority::Command, 2) {
        Err(TrySendError::Full(payload)) => assert_eq!(payload, 2, "the rejected payload must be handed back"),
        _ => panic!("expected TrySendError::Full for a saturated non-sheddable lane"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn non_preview_lane_send_future_blocks_until_a_recv_frees_a_slot() {
    let mut capacities = MailboxCapacities::uniform(64);
    capacities.set(Priority::Command, 1);
    let (address, receiver) = mailbox::<u32>(capacities);
    address.try_send(Priority::Command, 1).unwrap();

    let blocked_address = address;
    let sender_thread = std::thread::spawn(move || blocked_address.send_blocking(Priority::Command, 2));

    // No sleep/race here: `try_admit`'s register-then-re-check pattern makes this correct
    // regardless of whether the spawned thread has even started polling yet — either it
    // observes the freed slot on its own first poll, or it parks and this pop's wake() call
    // reaches it. See `SendFuture::poll`'s doc.
    let freed = receiver.try_recv();
    assert_eq!(freed.map(|e| e.payload), Some(1));
    assert!(sender_thread.join().unwrap().is_ok(), "the blocked send must complete once a slot frees");
    assert_eq!(receiver.try_recv().unwrap().payload, 2);
}

#[test]
fn closing_a_mailbox_drains_remaining_messages_then_resolves_recv_to_none() {
    let (address, receiver) = mailbox::<u32>(generous_capacities());
    address.try_send(Priority::Command, 1).unwrap();
    address.close();
    assert!(matches!(address.try_send(Priority::Command, 2), Err(TrySendError::Closed(2))));
    assert_eq!(receiver.try_recv().unwrap().payload, 1, "already-queued messages survive a close");
    assert!(receiver.try_recv().is_none());
}

#[test]
fn a_stale_generation_address_is_rejected_loudly_instead_of_enqueuing_silently() {
    let (address, _receiver) = mailbox::<u32>(generous_capacities());
    let stale = address.clone();
    let bumped = address.inner.bump_generation();

    match stale.try_send(Priority::Command, 7) {
        Err(TrySendError::Stale(payload, expected, actual)) => {
            assert_eq!(payload, 7);
            assert_eq!(expected, bumped);
            assert_eq!(actual, GenerationId::INITIAL);
        }
        _ => panic!("expected TrySendError::Stale for an address bound to a superseded generation"),
    }
}
//#endregion 🔖️Mailbox

//#region 🔖️Reply
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn oneshot_reply_round_trips_a_value() {
    let (tx, rx) = oneshot::<i32>();
    let sender_thread = std::thread::spawn(move || tx.send(42));
    assert_eq!(block_on(rx), Ok(42));
    sender_thread.join().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn oneshot_reply_resolves_closed_when_sender_is_dropped_without_sending() {
    let (tx, rx) = oneshot::<i32>();
    drop(tx);
    assert_eq!(block_on(rx), Err(DbError::Closed));
}
//#endregion 🔖️Reply

//#region 🔖️Actor
#[cfg(not(target_arch = "wasm32"))]
enum EchoMessage {
    Double(i32, ReplySender<i32>),
    Crash,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct EchoActor;

#[cfg(not(target_arch = "wasm32"))]
impl Actor for EchoActor {
    type Message = EchoMessage;

    fn handle(&mut self, msg: Self::Message, _ctx: &mut ActorContext<Self::Message>) -> Result<(), DbError> {
        match msg {
            EchoMessage::Double(value, reply) => {
                reply.send(value * 2);
                Ok(())
            }
            EchoMessage::Crash => panic!("intentional test crash"),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn actor_pool(workers: usize) -> Arc<semio_framework_async::WorkerPool> {
    Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig { process_kind: semio_framework_async::ProcessKind::InteractiveNative, cores: workers + 1, interactive_reserve: true }))
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ask_pattern_round_trips_through_a_bounded_actor_turn() {
    let pool = actor_pool(2);
    let supervisor = Supervisor::new(RestartStrategy::OneForOne, generous_capacities(), pool.clone(), Arc::new(NullEmit), EchoActor::default, 1);
    let address = supervisor.address(0);
    let reply = address.ask_blocking(Priority::Command, |tx| EchoMessage::Double(21, tx));
    assert_eq!(reply, Ok(42));
    drop(supervisor);
    pool.shutdown();
}
//#endregion 🔖️Actor

//#region 🔖️Supervision
#[cfg(not(target_arch = "wasm32"))]
fn reap_until_decided(supervisor: &Supervisor<EchoActor>) -> SupervisionDecision {
    for _ in 0..200 {
        if let Some(decision) = supervisor.reap() {
            return decision;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    panic!("supervisor never reaped a terminal outcome within the test's bound");
}

#[test]
fn restart_strategy_decide_is_the_pure_law_wasm_clean_paths_rely_on() {
    assert_eq!(RestartStrategy::OneForOne.decide(3), SupervisionDecision::RestartOne(3));
    assert_eq!(RestartStrategy::OneForAll.decide(0), SupervisionDecision::RestartAll);
    assert_eq!(RestartStrategy::Escalate.decide(1), SupervisionDecision::Escalate);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn one_for_one_restarts_only_the_failed_child_and_bumps_only_its_generation() {
    let pool = actor_pool(2);
    let supervisor = Supervisor::new(RestartStrategy::OneForOne, generous_capacities(), pool.clone(), Arc::new(NullEmit), EchoActor::default, 2);
    let stale_child0 = supervisor.address(0);
    let untouched_child1_generation = supervisor.generation(1);

    supervisor.address(0).send_blocking(Priority::Command, EchoMessage::Crash).unwrap();
    let decision = reap_until_decided(&supervisor);

    assert_eq!(decision, SupervisionDecision::RestartOne(0));
    assert_eq!(supervisor.generation(0), GenerationId::INITIAL.next());
    assert_eq!(supervisor.generation(1), untouched_child1_generation);
    assert!(matches!(stale_child0.try_send(Priority::Command, EchoMessage::Crash), Err(TrySendError::Stale(_, _, _))), "an Address captured before the restart must fail loudly rather than talk to a dead incarnation");

    let fresh_reply = supervisor.address(0).ask_blocking(Priority::Command, |tx| EchoMessage::Double(5, tx));
    assert_eq!(fresh_reply, Ok(10), "the restarted incarnation must be alive and answering");
    drop(supervisor);
    pool.shutdown();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn one_for_all_restarts_every_child_and_bumps_every_generation() {
    let pool = actor_pool(2);
    let supervisor = Supervisor::new(RestartStrategy::OneForAll, generous_capacities(), pool.clone(), Arc::new(NullEmit), EchoActor::default, 3);

    supervisor.address(1).send_blocking(Priority::Command, EchoMessage::Crash).unwrap();
    let decision = reap_until_decided(&supervisor);

    assert_eq!(decision, SupervisionDecision::RestartAll);
    for index in 0..3 {
        assert_eq!(supervisor.generation(index), GenerationId::INITIAL.next(), "child {index} must also have restarted");
    }
    drop(supervisor);
    pool.shutdown();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn escalate_reports_the_failure_without_restarting_anything() {
    let pool = actor_pool(2);
    let supervisor = Supervisor::new(RestartStrategy::Escalate, generous_capacities(), pool.clone(), Arc::new(NullEmit), EchoActor::default, 1);

    supervisor.address(0).send_blocking(Priority::Command, EchoMessage::Crash).unwrap();
    let decision = reap_until_decided(&supervisor);

    assert_eq!(decision, SupervisionDecision::Escalate);
    assert_eq!(supervisor.generation(0), GenerationId::INITIAL, "Escalate must not bump the generation itself");
    drop(supervisor);
    pool.shutdown();
}
//#endregion 🔖️Supervision

//#region 🔖️Config
#[test]
fn mailbox_from_config_honors_the_profile_default_capacities() {
    let config = DbConfig::for_profile(Profile::Test);
    let (address, _receiver) = mailbox_from_config::<u32>(&config);
    assert_eq!(address.generation(), GenerationId::INITIAL);
}
//#endregion 🔖️Config
