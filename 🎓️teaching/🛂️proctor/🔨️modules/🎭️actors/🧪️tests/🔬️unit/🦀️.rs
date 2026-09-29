use super::*;
use crate::catalog::tests::fixture;
use quiz::Identity;
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
        target: command_target(command, TENANT),
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

/// 💯️ The perfect answer to one presented task, read off the quiz's solutions.
pub(crate) fn perfect(quiz: &quiz::Quiz, presented: &quiz::SheetTask) -> quiz::Answer {
    use quiz::{Answer, ClassificationAnswer, MatchingAnswer, SheetTask, SortingAnswer, Task};
    let task = quiz.tasks.iter().find(|task| task.id() == presented.id()).expect("the sheet task exists");
    match (task, presented) {
        (Task::Sorting(task), SheetTask::Sorting(sheet)) => {
            let mut order: Vec<&quiz::SortingItem> = task.items.iter().filter(|item| sheet.items.iter().any(|shown| shown.id == item.id)).collect();
            order.sort_by(|left, right| left.value.total_cmp(&right.value));
            Answer::Sorting(SortingAnswer { order: order.iter().map(|item| item.id.clone()).collect() })
        }
        (Task::Classification(task), SheetTask::Classification(sheet)) => Answer::Classification(ClassificationAnswer { assignments: sheet.items.iter().map(|shown| (shown.id.clone(), task.items.iter().find(|item| item.id == shown.id).expect("item").category.clone())).collect() }),
        (Task::Matching(task), SheetTask::Matching(sheet)) => Answer::Matching(MatchingAnswer {
            assignments: sheet
                .dimensions
                .iter()
                .map(|dimension| {
                    let mut used = std::collections::BTreeSet::new();
                    let cards = sheet
                        .items
                        .iter()
                        .map(|shown| {
                            let value = task.items.iter().find(|item| item.id == shown.id).expect("item").values[&dimension.id];
                            let card = (0..dimension.cards.len()).find(|card| dimension.cards[*card] == value && !used.contains(card)).expect("a card of that value");
                            used.insert(card);
                            (shown.id.clone(), card)
                        })
                        .collect();
                    (dimension.id.clone(), cards)
                })
                .collect(),
        }),
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

#[test]
fn the_wire_names_follow_the_design() {
    let start = Command::StartRun { id: id(2), learner: ADA.into(), run: id(3), quiz: "power".into() };
    assert_eq!(command_kind(&start), "quiz.start-run");
    assert_eq!(command_target(&start, TENANT), learner_key(TENANT, ADA));
    assert_eq!(command_target(&identify(ADA, Identity::Anonymous), TENANT), ActorKey { tenant: TenantId(TENANT.into()), kind: "quiz-roster".into(), id: "roster".into() });
    assert_eq!(event_kind(&Event::LearnerRecalled { learner: ADA.into(), at: 1 }), "quiz.learner-recalled");
    assert_eq!(rejection(quiz::Rejection::RunIncomplete), Rejection::Invalid { detail: "run-incomplete".into() });
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
    assert!(refused(|envelope| envelope.version = 2).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.scope = Scope("other".into())).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.kind = "quiz.start-run".into()).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.idempotency_key = None).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.target = learner_key(TENANT, ADA)).starts_with("envelope-mismatch"));
    assert!(refused(|envelope| envelope.payload = b"{}".to_vec()).starts_with("command-malformed"));
}

#[tokio::test]
async fn the_roster_registers_recalls_and_refuses_through_the_framework_port() {
    let roster = RosterDecider { tenant: TENANT.into() };
    let mut state = ActorState::default();
    let registered = roster.decide(&state, &envelope(&identify(ADA, Identity::Pseudonym { handle: " Ada  Lovelace ".into() })), &context(10)).await;
    assert_eq!(events(&registered), vec![Event::LearnerRegistered { learner: ADA.into(), identity: Identity::Pseudonym { handle: "Ada Lovelace".into() }, at: 10 }]);
    let Decision::Emit { events: records, .. } = &registered else { unreachable!() };
    assert_eq!((records[0].kind.as_str(), &records[0].stream), ("quiz.learner-registered", &roster_key(TENANT)));
    roster.evolve(&mut state, &records[0]).await;
    let recalled = roster.decide(&state, &envelope(&identify(BOB, Identity::Name { handle: "ada lovelace".into() })), &context(11)).await;
    assert_eq!(events(&recalled), vec![Event::LearnerRecalled { learner: ADA.into(), at: 11 }]);
    let invalid = roster.decide(&state, &envelope(&identify(BOB, Identity::Pseudonym { handle: "   ".into() })), &context(12)).await;
    assert_eq!(invalid, Decision::Reject(Rejection::Invalid { detail: "handle-invalid".into() }));
}

#[tokio::test]
async fn enrollment_relays_roster_facts_to_the_learner_once() {
    let learner = LearnerDecider { catalog: Arc::new(fixture()) };
    let registered = EventRecord { stream: roster_key(TENANT), seq: 4, hlc: HybridLogicalClock { millis: 10, counter: 0 }, kind: "quiz.learner-registered".into(), payload: serde_json::to_vec(&Event::LearnerRegistered { learner: ADA.into(), identity: Identity::Anonymous, at: 10 }).unwrap() };
    let relay = enrollment(&registered).expect("a registration is relayed");
    assert_eq!((relay.kind.as_str(), &relay.target, relay.idempotency_key.clone()), (ENROLL, &learner_key(TENANT, ADA), Some(IdempotencyKey(format!("enroll:{TENANT}:roster:4")))));
    let mut state = ActorState::default();
    let first = learner.decide(&state, &relay, &context(20)).await;
    assert_eq!(events(&first), vec![Event::LearnerRegistered { learner: ADA.into(), identity: Identity::Anonymous, at: 10 }]);
    let Decision::Emit { events: records, .. } = &first else { unreachable!() };
    learner.evolve(&mut state, &records[0]).await;
    assert!(events(&learner.decide(&state, &relay, &context(21)).await).is_empty());
    let mut forged = relay.clone();
    forged.principal = Principal::Anonymous;
    assert!(matches!(learner.decide(&state, &forged, &context(22)).await, Decision::Reject(Rejection::Unauthorized { .. })));
    let own = EventRecord { stream: learner_key(TENANT, ADA), ..registered };
    assert!(enrollment(&own).is_none());
}

#[tokio::test]
async fn a_learner_runs_only_once_enrolled() {
    let learner = LearnerDecider { catalog: Arc::new(fixture()) };
    let start = Command::StartRun { id: id(5), learner: ADA.into(), run: id(6), quiz: "power".into() };
    assert_eq!(learner.decide(&ActorState::default(), &envelope(&start), &context(30)).await, Decision::Reject(Rejection::Invalid { detail: "unknown-learner".into() }));
    let mut state = ActorState::default();
    let enrolled = EventRecord { stream: learner_key(TENANT, ADA), seq: 1, hlc: HybridLogicalClock::default(), kind: "quiz.learner-registered".into(), payload: serde_json::to_vec(&Event::LearnerRegistered { learner: ADA.into(), identity: Identity::Anonymous, at: 1 }).unwrap() };
    learner.evolve(&mut state, &enrolled).await;
    let started = events(&learner.decide(&state, &envelope(&start), &context(31)).await);
    assert!(matches!(&started[..], [Event::RunStarted { quiz, seed, at: 31, .. }] if quiz == "power" && *seed == quiz::run_seed(&id(6))));
}
