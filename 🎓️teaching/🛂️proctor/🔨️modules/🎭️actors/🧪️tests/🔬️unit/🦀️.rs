use super::*;
use crate::catalog::tests::fixture;
use quiz::{Challenge, Identity};
use server::contract::HybridLogicalClock;

pub(crate) const TENANT: &str = "proctor-fixture";
pub(crate) const ADA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
pub(crate) const BOB: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

/// 📨️ The §9a envelope of one quiz command, as a client sends it.
pub(crate) fn envelope(command: &Command) -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId(command.id().clone()),
        kind: command_kind(command),
        version: WIRE_VERSION,
        target: command_target(command, TENANT).unwrap_or_else(|_| learner_key(TENANT, command.learner())),
        scope: Scope(TENANT.to_string()),
        principal: Principal::Anonymous,
        session: None,
        device: None,
        payload: serde_json::to_vec(command).unwrap(),
        causal_frontier: None,
        client_hlc: HybridLogicalClock::default(),
        expected_revision: None,
        idempotency_key: Some(IdempotencyKey(command.id().clone())),
        capability_proof: None,
        trace: TraceContext::default(),
    }
}

pub(crate) fn id(seed: u8) -> String {
    format!("{seed:032x}")
}

/// 🛃️ An admission of the fixture catalog under the default caps, counting `learners`.
pub(crate) fn admission(learners: u64) -> Arc<Admission> {
    Arc::new(Admission::new(TENANT, Arc::new(AtomicU64::new(learners))))
}

/// 💯️ The perfect answer to one presented task, read off the quiz's solutions: the keys assigned
/// where the sheet shows them, the true values guessed where it hides them.
pub(crate) fn perfect(quiz: &quiz::Quiz, presented: &quiz::SheetTask) -> quiz::Answer {
    use quiz::{Answer, ClassificationAnswer, MatchingAnswer, SheetTask, SortingAnswer, Task};
    let task = quiz.tasks.iter().find(|task| task.id() == presented.id()).expect("the sheet task exists");
    match (task, presented) {
        (Task::Sorting(task), SheetTask::Sorting(sheet)) => {
            let mut order: Vec<&quiz::SortingItem> = task.items.iter().filter(|item| sheet.items.iter().any(|shown| shown.id == item.id)).collect();
            order.sort_by(|left, right| left.value.total_cmp(&right.value));
            let guesses = sheet.keys.is_none().then(|| order.iter().map(|item| (item.id.clone(), item.value)).collect());
            Answer::Sorting(SortingAnswer { order: order.iter().map(|item| item.id.clone()).collect(), guesses })
        }
        (Task::Classification(task), SheetTask::Classification(sheet)) => Answer::Classification(ClassificationAnswer { assignments: sheet.items.iter().map(|shown| (shown.id.clone(), task.items.iter().find(|item| item.id == shown.id).expect("item").category.clone())).collect() }),
        (Task::Matching(task), SheetTask::Matching(sheet)) => {
            let value = |item: &str, dimension: &str| task.items.iter().find(|defined| defined.id == item).expect("item").values[dimension];
            let assignments = sheet.dimensions.iter().filter_map(|dimension| {
                let cards = dimension.cards.as_ref()?;
                let mut used = std::collections::BTreeSet::new();
                let assigned = sheet.items.iter().map(|shown| {
                    let card = (0..cards.len()).find(|card| cards[*card] == value(&shown.id, &dimension.id) && !used.contains(card)).expect("a card of that value");
                    used.insert(card);
                    (shown.id.clone(), card)
                });
                Some((dimension.id.clone(), assigned.collect()))
            });
            let guesses = sheet.dimensions.iter().filter(|dimension| dimension.cards.is_none()).map(|dimension| (dimension.id.clone(), sheet.items.iter().map(|shown| (shown.id.clone(), value(&shown.id, &dimension.id))).collect()));
            let (assignments, guesses): (std::collections::BTreeMap<_, _>, std::collections::BTreeMap<_, _>) = (assignments.collect(), guesses.collect());
            Answer::Matching(MatchingAnswer { assignments: (!assignments.is_empty()).then_some(assignments), guesses: (!guesses.is_empty()).then_some(guesses) })
        }
        _ => panic!("the sheet task has the quiz task's kind"),
    }
}

fn context(millis: u64) -> DecisionContext {
    DecisionContext { now: HybridLogicalClock { millis, counter: 0 }, principal: Principal::Anonymous, scope: Scope(TENANT.to_string()) }
}

fn identify(learner: &str, identity: Identity) -> Command {
    Command::IdentifyLearner { id: id(1), learner: learner.to_string(), identity }
}

fn events(decision: &Decision) -> Vec<Event> {
    let Decision::Emit { events, .. } = decision else { panic!("expected events, got {decision:?}") };
    events.iter().map(|record| serde_json::from_slice(&record.payload).unwrap()).collect()
}

fn invalid(detail: &str) -> Decision {
    Decision::Reject(Rejection::Invalid { detail: detail.into() })
}

fn registration(stream: ActorKey, learner: &str, identity: Identity) -> EventRecord {
    EventRecord { stream, seq: 1, hlc: HybridLogicalClock { millis: 10, counter: 0 }, kind: "quiz.learner-registered".into(), payload: serde_json::to_vec(&Event::LearnerRegistered { learner: learner.into(), identity, at: 10 }).unwrap() }
}

#[test]
fn the_wire_names_follow_the_design() {
    let start = Command::StartRun { id: id(2), learner: ADA.into(), run: id(3), quiz: "power".into(), challenge: Challenge::Medium, at: 0 };
    assert_eq!(command_kind(&start), "quiz.start-run");
    assert_eq!(command_target(&start, TENANT), Ok(learner_key(TENANT, ADA)));
    assert_eq!(command_target(&identify(ADA, Identity::Anonymous), TENANT), Ok(learner_key(TENANT, ADA)));
    assert_eq!(command_target(&identify(ADA, Identity::Name { handle: " Ada  Lovelace ".into() }), TENANT), Ok(ActorKey { tenant: TenantId(TENANT.into()), kind: "quiz-handle".into(), id: "616461206c6f76656c616365".into() }));
    assert_eq!(command_target(&identify(ADA, Identity::Name { handle: "A\u{200b}da".into() }), TENANT), Err(quiz::Rejection::HandleInvalid));
    assert_eq!(handle_key(TENANT, "ada lovelace").id, quiz::handle_actor_id("ada lovelace"));
    assert_eq!(event_kind(&Event::RunVoided { learner: ADA.into(), run: id(3), at: 1 }), "quiz.run-voided");
    assert_eq!(rejection(quiz::Rejection::RunIncomplete), Rejection::Invalid { detail: "run-incomplete".into() });
    assert_eq!(enrollment_key(TENANT, &handle_key(TENANT, "ada").id), format!("enroll:{TENANT}:616461"));
}

#[test]
fn an_envelope_must_agree_with_its_command() {
    let command = identify(ADA, Identity::Anonymous);
    assert_eq!(admitted(&envelope(&command), TENANT), Ok(command.clone()));
    let refused = |mutate: fn(&mut CommandEnvelope)| {
        let mut changed = envelope(&command);
        mutate(&mut changed);
        match admitted(&changed, TENANT) {
            Err(Rejection::Invalid { detail }) => detail,
            other => panic!("expected a refusal, got {other:?}"),
        }
    };
    assert!(refused(|envelope| envelope.version = WIRE_VERSION - 1).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.version = WIRE_VERSION + 1).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.scope = Scope("other".into())).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.kind = "quiz.start-run".into()).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.idempotency_key = None).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.target = learner_key(TENANT, BOB)).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.target = handle_key(TENANT, "ada")).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.payload = b"{}".to_vec()).starts_with("command-malformed"));
}

#[test]
fn no_malformed_id_and_no_refused_handle_is_admitted() {
    let admission = admission(0);
    let refusal = |command: &Command, target: ActorKey| {
        let mut sent = envelope(command);
        sent.target = target;
        admission.admit(&sent)
    };
    let megabyte = "a".repeat(1 << 20);
    for learner in ["ADA", "enroll:proctor-fixture:roster:1", megabyte.as_str(), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n", ""] {
        let start = Command::StartRun { id: id(2), learner: learner.into(), run: id(3), quiz: "power".into(), challenge: Challenge::Medium, at: 0 };
        assert_eq!(refusal(&start, learner_key(TENANT, learner)), Err(Rejection::Invalid { detail: "id-invalid".into() }), "{:?}", &learner[..learner.len().min(40)]);
        assert_eq!(refusal(&identify(learner, Identity::Anonymous), learner_key(TENANT, learner)), Err(Rejection::Invalid { detail: "id-invalid".into() }));
    }
    let run = Command::StartRun { id: id(2), learner: ADA.into(), run: "../run".into(), quiz: "power".into(), challenge: Challenge::Medium, at: 0 };
    assert_eq!(admission.admit(&envelope(&run)), Err(Rejection::Invalid { detail: "id-invalid".into() }));
    let quiz = Command::StartRun { id: id(2), learner: ADA.into(), run: id(3), quiz: "Power".into(), challenge: Challenge::Medium, at: 0 };
    assert_eq!(admission.admit(&envelope(&quiz)), Err(Rejection::Invalid { detail: "id-invalid".into() }));
    let mut keyed = envelope(&Command::SubmitRun { id: id(2), learner: ADA.into(), run: id(3) });
    keyed.command_id = CommandId(format!("enroll:{TENANT}:616461"));
    assert!(matches!(admission.admit(&keyed), Err(Rejection::Invalid { detail }) if detail.starts_with("envelope-mismatch")));
    for handle in ["   ", "A\u{200b}da", "\u{202e}adA", "Ade\u{301}", "\u{410}da", "..."] {
        let named = identify(ADA, Identity::Name { handle: handle.into() });
        assert_eq!(refusal(&named, handle_key(TENANT, "ada")), Err(Rejection::Invalid { detail: "handle-invalid".into() }), "{handle:?}");
    }
    let named = identify(ADA, Identity::Name { handle: "Ada".into() });
    assert_eq!(admission.admit(&envelope(&named)), Ok(()));
    assert!(matches!(refusal(&named, handle_key(TENANT, "bob")), Err(Rejection::Invalid { detail }) if detail.starts_with("envelope-mismatch")));
    for (kind, target) in [(HANDLE, megabyte.as_str()), (HANDLE, "ADA"), (HANDLE, "616"), (HANDLE, "ff"), (HANDLE, ""), (LEARNER, "616461"), ("quiz-roster", "roster"), ("", ADA)] {
        let misaddressed = ActorKey { tenant: TenantId(TENANT.into()), kind: kind.into(), id: target.into() };
        assert_eq!((addressable(&misaddressed), refusal(&named, misaddressed)), (false, Err(Rejection::Invalid { detail: "id-invalid".into() })), "{kind}/{:?}", &target[..target.len().min(40)]);
    }
    let longest = "\u{1e01}".repeat(quiz::HANDLE_MAX);
    assert_eq!((handle_key(TENANT, &longest).id.len(), addressable(&handle_key(TENANT, &longest)), admission.admit(&envelope(&identify(ADA, Identity::Name { handle: longest })))), (384, true, Ok(())), "the longest handle has an addressable actor");
    let mut unread = envelope(&named);
    unread.target = ActorKey { tenant: TenantId(TENANT.into()), kind: LEARNER.into(), id: "not-an-id".into() };
    unread.payload = b"not json".to_vec();
    assert_eq!(admission.admit(&unread), Err(Rejection::Invalid { detail: "id-invalid".into() }), "a target no actor can have is refused before the payload is read");
}

#[test]
fn a_full_proctor_admits_no_registration_but_every_other_command() {
    let full = admission(3);
    full.cap(Limits { learners: 3, ..quiz::DEFAULT_LIMITS });
    assert_eq!(full.caps().learners, 3);
    assert_eq!(full.admit(&envelope(&identify(ADA, Identity::Anonymous))), Err(Rejection::Invalid { detail: "roster-full".into() }));
    assert_eq!(full.admit(&envelope(&identify(ADA, Identity::Name { handle: "Ada".into() }))), Err(Rejection::Invalid { detail: "roster-full".into() }));
    assert_eq!(full.admit(&envelope(&Command::StartRun { id: id(2), learner: ADA.into(), run: id(3), quiz: "power".into(), challenge: Challenge::Medium, at: 0 })), Ok(()));
    full.cap(Limits { learners: 4, ..quiz::DEFAULT_LIMITS });
    assert_eq!(full.admit(&envelope(&identify(ADA, Identity::Anonymous))), Ok(()));
}

#[tokio::test]
async fn a_handle_registers_its_first_claim_and_refuses_the_rest_through_the_framework_port() {
    let handle = HandleDecider { admission: admission(0) };
    assert_eq!((handle.actor_kind().await, handle.state_format().await), (HANDLE, STATE_FORMAT));
    let mut state = ActorState::default();
    let claim = envelope(&identify(ADA, Identity::Pseudonym { handle: " Ada  Lovelace ".into() }));
    let registered = handle.decide(&state, &claim, &context(10)).await;
    assert_eq!(events(&registered), vec![Event::LearnerRegistered { learner: ADA.into(), identity: Identity::Pseudonym { handle: "Ada Lovelace".into() }, at: 10 }]);
    let Decision::Emit { events: records, .. } = &registered else { unreachable!() };
    assert_eq!((records[0].kind.as_str(), &records[0].stream), ("quiz.learner-registered", &handle_key(TENANT, "ada lovelace")));
    handle.evolve(&mut state, &EventRecord { seq: 1, ..records[0].clone() }).await;
    assert_eq!(stored::<HandleState>(&state), Stored::State(HandleState { key: "ada lovelace".into(), holder: Some(ADA.into()) }));
    assert_eq!(handle.decide(&state, &envelope(&identify(BOB, Identity::Name { handle: "ada lovelace".into() })), &context(11)).await, invalid("handle-claimed"));
    assert_eq!(handle.decide(&ActorState::default(), &envelope(&identify(BOB, Identity::Pseudonym { handle: "   ".into() })), &context(12)).await, invalid("handle-invalid"));
}

#[tokio::test]
async fn enrollment_relays_a_handle_registration_to_its_learner_once() {
    let learner = LearnerDecider { catalog: Arc::new(fixture()), admission: admission(0) };
    let identity = Identity::Name { handle: "Ada Lovelace".into() };
    let registered = registration(handle_key(TENANT, "ada lovelace"), ADA, identity.clone());
    let relay = enrollment(&registered).expect("a handle registration is relayed");
    let key = format!("enroll:{TENANT}:616461206c6f76656c616365");
    assert_eq!((relay.kind.as_str(), &relay.target, relay.command_id.0.as_str(), relay.idempotency_key.clone()), (ENROLL, &learner_key(TENANT, ADA), key.as_str(), Some(IdempotencyKey(key.clone()))));
    assert!(!key.contains(ADA), "the key of an enrollment names its handle, never its learner");
    assert_eq!(learner.admission.admit(&relay), Ok(()));
    let renamed = enrollment(&registration(handle_key(TENANT, "countess"), ADA, Identity::Name { handle: "Countess".into() })).expect("a second handle of the same learner");
    assert_ne!(renamed.command_id, relay.command_id, "every handle relays under its own key");
    let mut borrowed = renamed.clone();
    borrowed.command_id = relay.command_id.clone();
    borrowed.idempotency_key = relay.idempotency_key.clone();
    assert!(matches!(learner.admission.admit(&borrowed), Err(Rejection::Invalid { .. })), "a relay under another handle's key is none");
    let mut state = ActorState::default();
    let first = learner.decide(&state, &relay, &context(20)).await;
    assert_eq!(events(&first), vec![Event::LearnerRegistered { learner: ADA.into(), identity, at: 10 }]);
    let Decision::Emit { events: records, .. } = &first else { unreachable!() };
    learner.evolve(&mut state, &EventRecord { seq: 1, ..records[0].clone() }).await;
    assert!(events(&learner.decide(&state, &relay, &context(21)).await).is_empty());
    assert!(events(&learner.decide(&state, &renamed, &context(21)).await).is_empty(), "a registered learner keeps the identity it has");
    let mut forged = relay.clone();
    forged.principal = Principal::Anonymous;
    assert!(matches!(learner.decide(&state, &forged, &context(22)).await, Decision::Reject(Rejection::Unauthorized { .. })));
    assert!(matches!(learner.admission.admit(&forged), Err(Rejection::Unauthorized { .. })));
    let mut rekeyed = relay.clone();
    rekeyed.idempotency_key = Some(IdempotencyKey("enroll:other".into()));
    assert!(matches!(learner.admission.admit(&rekeyed), Err(Rejection::Invalid { .. })));
    let mut misaddressed = relay.clone();
    misaddressed.target = learner_key(TENANT, BOB);
    assert!(matches!(learner.admission.admit(&misaddressed), Err(Rejection::Invalid { .. })));
    assert!(enrollment(&EventRecord { stream: learner_key(TENANT, ADA), ..registered.clone() }).is_none(), "a learner's own registration is not relayed");
    assert!(enrollment(&registration(handle_key(TENANT, "x"), ADA, Identity::Anonymous)).is_some_and(|anonymous| learner.admission.admit(&anonymous).is_err()), "an anonymous registration is never an enrollment");
}

#[tokio::test]
async fn an_anonymous_learner_registers_in_its_own_stream_and_runs_only_once_registered() {
    let learner = LearnerDecider { catalog: Arc::new(fixture()), admission: admission(0) };
    assert_eq!((learner.actor_kind().await, learner.state_format().await), (LEARNER, STATE_FORMAT));
    let start = Command::StartRun { id: id(5), learner: ADA.into(), run: id(6), quiz: "power".into(), challenge: Challenge::Medium, at: 33 };
    assert_eq!(learner.decide(&ActorState::default(), &envelope(&start), &context(30)).await, invalid("unknown-learner"));
    let mut state = ActorState::default();
    let registered = learner.decide(&state, &envelope(&identify(ADA, Identity::Anonymous)), &context(31)).await;
    assert_eq!(events(&registered), vec![Event::LearnerRegistered { learner: ADA.into(), identity: Identity::Anonymous, at: 31 }]);
    let Decision::Emit { events: records, .. } = &registered else { unreachable!() };
    learner.evolve(&mut state, &EventRecord { seq: 1, ..records[0].clone() }).await;
    assert_eq!(learner.decide(&state, &envelope(&identify(ADA, Identity::Anonymous)), &context(32)).await, invalid("learner-exists"));
    let started = events(&learner.decide(&state, &envelope(&start), &context(33)).await);
    assert!(matches!(&started[..], [Event::RunStarted { quiz, seed, at: 33, .. }] if quiz == "power" && *seed == quiz::run_seed(&id(6))));
}

#[tokio::test]
async fn the_caps_in_force_reach_the_learner_decisions() {
    let learner = LearnerDecider { catalog: Arc::new(fixture()), admission: admission(0) };
    learner.admission.cap(Limits { answers_per_run: 1, ..quiz::DEFAULT_LIMITS });
    let mut state = ActorState::default();
    learner.evolve(&mut state, &registration(learner_key(TENANT, ADA), ADA, Identity::Anonymous)).await;
    let run = id(6);
    let start = Command::StartRun { id: id(5), learner: ADA.into(), run: run.clone(), quiz: "power".into(), challenge: Challenge::Medium, at: 40 };
    let Decision::Emit { events: started, .. } = learner.decide(&state, &envelope(&start), &context(40)).await else { panic!("the run starts") };
    learner.evolve(&mut state, &EventRecord { seq: 2, ..started[0].clone() }).await;
    let catalog = fixture();
    let power = &catalog.current()["power"].quiz;
    let sheet = quiz::sheet_of(power, quiz::run_seed(&run), Challenge::Medium);
    let answer = |seed: u8| Command::RecordAnswer { id: id(seed), learner: ADA.into(), run: run.clone(), task: sheet.tasks[0].id().clone(), answer: perfect(power, &sheet.tasks[0]), at: 41 };
    let Decision::Emit { events: recorded, .. } = learner.decide(&state, &envelope(&answer(7)), &context(41)).await else { panic!("the first answer is recorded") };
    learner.evolve(&mut state, &EventRecord { seq: 3, ..recorded[0].clone() }).await;
    assert_eq!(learner.decide(&state, &envelope(&answer(8)), &context(42)).await, invalid("answers-exhausted"));
    learner.admission.cap(Limits { runs_per_quiz: 3, ..quiz::DEFAULT_LIMITS });
    for (seed, challenge) in [(20, Challenge::Easy), (21, Challenge::Medium)] {
        let switch = Command::StartRun { id: id(seed), learner: ADA.into(), run: id(seed + 10), quiz: "power".into(), challenge, at: 43 };
        let Decision::Emit { events: switched, .. } = learner.decide(&state, &envelope(&switch), &context(43)).await else { panic!("switching to {challenge:?} voids the open run") };
        for (offset, record) in switched.iter().enumerate() {
            learner.evolve(&mut state, &EventRecord { seq: 4 + u64::from(seed) * 2 + offset as u64, ..record.clone() }).await;
        }
    }
    let again = Command::StartRun { id: id(22), learner: ADA.into(), run: id(32), quiz: "power".into(), challenge: Challenge::Hard, at: 44 };
    assert_eq!(learner.decide(&state, &envelope(&again), &context(44)).await, invalid("runs-exhausted"), "voided runs count against the cap, so switching back and forth stops");
}

#[tokio::test]
async fn the_clock_of_a_timed_run_runs_from_the_instants_the_device_claims_and_the_proctors_clock_only_bounds_their_lead() {
    let learner = LearnerDecider { catalog: Arc::new(fixture()), admission: admission(0) };
    let mut state = ActorState::default();
    learner.evolve(&mut state, &registration(learner_key(TENANT, ADA), ADA, Identity::Anonymous)).await;
    let mut seq = 1;
    let mut decide = async |state: &mut ActorState, command: Command, millis: u64| {
        let decision = learner.decide(state, &envelope(&command), &context(millis)).await;
        if let Decision::Emit { events: records, .. } = &decision {
            for record in records {
                seq += 1;
                learner.evolve(state, &EventRecord { seq, ..record.clone() }).await;
            }
        }
        decision
    };
    let (run, plain) = (id(6), id(7));
    assert!(matches!(&events(&decide(&mut state, Command::StartRun { id: id(5), learner: ADA.into(), run: run.clone(), quiz: "homes".into(), challenge: Challenge::Expert, at: 1_000 }, 1_000).await)[..], [Event::RunStarted { challenge: Challenge::Expert, at: 1_000, .. }]));
    let catalog = fixture();
    let homes = &catalog.current()["homes"].quiz;
    let sheet = quiz::sheet_of(homes, quiz::run_seed(&run), Challenge::Expert);
    let (task, seconds) = (sheet.tasks[0].id().clone(), sheet.tasks[0].seconds().expect("an expert sheet task has a clock"));
    let open = |seed: u8, at: u64| Command::OpenTask { id: id(seed), learner: ADA.into(), run: run.clone(), task: task.clone(), at };
    let answer = |seed: u8, at: u64| Command::RecordAnswer { id: id(seed), learner: ADA.into(), run: run.clone(), task: task.clone(), answer: perfect(homes, &sheet.tasks[0]), at };
    assert_eq!(decide(&mut state, answer(8, 1_500), 1_500).await, invalid("task-unopened"));
    assert_eq!(decide(&mut state, Command::OpenTask { id: id(9), learner: ADA.into(), run: run.clone(), task: "ghost".into(), at: 1_500 }, 1_500).await, invalid("unknown-task"));
    assert_eq!(events(&decide(&mut state, open(10, 500), 2_000).await), vec![Event::TaskOpened { learner: ADA.into(), run: run.clone(), task: task.clone(), at: 1_000 }], "an opening is never earlier than the run's start");
    assert_eq!(decide(&mut state, open(11, 2_500), 2_500).await, invalid("already-opened"), "a task opens once and a repeat stores no receipt");
    let limit = 1_000 + seconds * 1_000;
    assert_eq!(events(&decide(&mut state, answer(12, 10), 3_000).await), vec![Event::AnswerRecorded { learner: ADA.into(), run: run.clone(), task: task.clone(), answer: perfect(homes, &sheet.tasks[0]), at: 1_000 }], "an answer is never earlier than its task's opening");
    assert!(matches!(&events(&decide(&mut state, answer(13, limit), limit + 60_000).await)[..], [Event::AnswerRecorded { at, .. }] if *at == limit), "an answer the device made in time counts however late it arrives");
    assert_eq!(decide(&mut state, answer(14, limit + 1), 3_000).await, invalid("time-up"), "an answer the device made too late is refused however early it arrives: the proctor's own clock is no verdict");
    let ahead = id(17);
    assert!(matches!(&events(&decide(&mut state, Command::StartRun { id: id(19), learner: ADA.into(), run: ahead.clone(), quiz: "power".into(), challenge: Challenge::Expert, at: limit + 61_000 }, limit + 61_000).await)[..], [Event::RunStarted { .. }]));
    assert!(matches!(&events(&decide(&mut state, Command::OpenTask { id: id(20), learner: ADA.into(), run: ahead.clone(), task: "appliances".into(), at: limit + 61_000 + 240_000 }, limit + 61_000).await)[..], [Event::TaskOpened { at, .. }] if *at == limit + 61_000 + 240_000), "a device clock four minutes ahead of the proctor's is taken as it claims");
    assert!(matches!(&events(&decide(&mut state, Command::OpenTask { id: id(21), learner: ADA.into(), run: ahead, task: "sources".into(), at: limit + 9_000_000 }, limit + 61_000).await)[..], [Event::TaskOpened { at, .. }] if *at == limit + 61_000 + quiz::CLOCK_LEAD), "a claim more than five minutes ahead of the proctor's clock is lowered to that lead");
    assert!(matches!(&events(&decide(&mut state, Command::StartRun { id: id(15), learner: ADA.into(), run: plain.clone(), quiz: "power".into(), challenge: Challenge::Hard, at: limit + 61_000 }, limit + 61_000).await)[..], [Event::RunVoided { .. }, Event::RunStarted { challenge: Challenge::Hard, .. }]), "another challenge voids the open run of the quiz");
    assert_eq!(decide(&mut state, Command::OpenTask { id: id(16), learner: ADA.into(), run: plain, task: "appliances".into(), at: limit + 61_000 }, limit + 61_000).await, invalid("run-untimed"));
}

#[tokio::test]
async fn an_event_or_state_that_does_not_decode_poisons_the_actor_loudly() {
    let learner = LearnerDecider { catalog: Arc::new(fixture()), admission: admission(0) };
    let mut state = ActorState::default();
    learner.evolve(&mut state, &registration(learner_key(TENANT, ADA), ADA, Identity::Anonymous)).await;
    let healthy = state.bytes.clone();
    let recalled = EventRecord { stream: learner_key(TENANT, ADA), seq: 2, hlc: HybridLogicalClock::default(), kind: "quiz.learner-recalled".into(), payload: br#"{"type":"learner-recalled","learner":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","at":5}"#.to_vec() };
    learner.evolve(&mut state, &recalled).await;
    assert_ne!(state.bytes, healthy, "the fact is not skipped");
    let Stored::Corrupt(corrupt) = stored::<LearnerState>(&state) else { panic!("the actor is poisoned") };
    assert!(corrupt.contains("event 2 of quiz-learner/") && corrupt.contains("quiz.learner-recalled") && corrupt.contains("learner-recalled"), "{corrupt}");
    let start = Command::StartRun { id: id(5), learner: ADA.into(), run: id(6), quiz: "power".into(), challenge: Challenge::Medium, at: 0 };
    let Decision::Reject(Rejection::ActorUnavailable { detail }) = learner.decide(&state, &envelope(&start), &context(50)).await else { panic!("a poisoned actor decides nothing") };
    assert!(detail.starts_with("actor-corrupt: event 2 of quiz-learner/"), "{detail}");
    let poisoned = state.bytes.clone();
    learner.evolve(&mut state, &registration(learner_key(TENANT, ADA), ADA, Identity::Anonymous)).await;
    assert_eq!(state.bytes, poisoned, "a poisoned actor stays poisoned");
    let handle = HandleDecider { admission: admission(0) };
    let mut garbage = ActorState { bytes: b"\xff not json".to_vec(), ..ActorState::default() };
    assert!(matches!(stored::<HandleState>(&garbage), Stored::Corrupt(_)));
    let claim = envelope(&identify(ADA, Identity::Name { handle: "Ada".into() }));
    assert!(matches!(handle.decide(&garbage, &claim, &context(60)).await, Decision::Reject(Rejection::ActorUnavailable { .. })));
    handle.evolve(&mut garbage, &registration(handle_key(TENANT, "ada"), ADA, Identity::Name { handle: "Ada".into() })).await;
    assert_eq!(garbage.bytes, b"\xff not json".to_vec());
    let mut misnamed = ActorState::default();
    handle.evolve(&mut misnamed, &registration(ActorKey { tenant: TenantId(TENANT.into()), kind: HANDLE.into(), id: "not hex".into() }, ADA, Identity::Name { handle: "Ada".into() })).await;
    assert!(matches!(stored::<HandleState>(&misnamed), Stored::Corrupt(_)));
}
