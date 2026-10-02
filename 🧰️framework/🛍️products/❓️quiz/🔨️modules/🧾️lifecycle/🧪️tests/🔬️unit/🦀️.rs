//! 🎞️ Unit tests of the lifecycle: handles, roster and learner decision tables and the shared sequences; also the replay kit.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::schema::{Badge, BadgeRule, ClassificationAnswer, Introduction, MatchingAnswer, SheetTask, SortingAnswer, Task, TaskKind, DEFAULT_LIMITS};
use crate::sheet::tests::{quiz, text};

pub(crate) const ALICE: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
pub(crate) const BOB: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
pub(crate) const RUN: &str = "0123456789abcdef0123456789abcdef";
pub(crate) const REVISION: &str = "1111111111111111111111111111111111111111111111111111111111111111";
pub(crate) const REVISED: &str = "2222222222222222222222222222222222222222222222222222222222222222";

pub(crate) fn catalog() -> Catalog {
    let badge = |id: &str, rule: BadgeRule| Badge { id: id.to_string(), emoji: "🏅".to_string(), label: text(id), description: text(id), rule };
    Catalog {
        json_schema: None,
        schema: "semio.quiz.catalog/v1".to_string(),
        id: "architecture".to_string(),
        title: text("Architecture"),
        introduction: Introduction { title: text("Welcome"), paragraphs: vec![text("Hello")] },
        quizzes: vec!["energy/🔣️.json".to_string()],
        badges: vec![badge("perfect-energy", BadgeRule::PerfectQuiz { quiz: "energy".to_string() }), badge("sorter", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None }), badge("done", BadgeRule::CompletedQuizzes)],
    }
}

pub(crate) fn quizzes(revision: &str) -> BTreeMap<Slug, LoadedQuiz> {
    BTreeMap::from([("energy".to_string(), LoadedQuiz { quiz: quiz(), revision: revision.to_string() })])
}

pub(crate) fn id(seed: u8) -> Id {
    format!("{seed:032x}")
}

pub(crate) fn fold(state: &mut LearnerState, decision: &Decision) -> Vec<Event> {
    let Decision::Events(events) = decision else { return Vec::new() };
    for event in events {
        evolve_learner(state, event);
    }
    events.clone()
}

pub(crate) fn step(state: &mut LearnerState, command: &Command, context: &LearnerContext<'_>) -> Vec<Event> {
    let decision = decide_learner(state, command, context);
    fold(state, &decision)
}

pub(crate) fn registered(learner: &str, at: Timestamp) -> LearnerState {
    let mut state = empty_learner_state(learner);
    evolve_learner(&mut state, &Event::LearnerRegistered { learner: learner.to_string(), identity: Identity::Pseudonym { handle: learner[..4].to_string() }, at });
    state
}

pub(crate) fn perfect(quiz: &Quiz, sheet_task: &SheetTask) -> Answer {
    let task = quiz.tasks.iter().find(|task| task.id() == sheet_task.id());
    match (task, sheet_task) {
        (Some(Task::Classification(task)), SheetTask::Classification(sheet)) => Answer::Classification(ClassificationAnswer { assignments: sheet.items.iter().filter_map(|item| task.items.iter().find(|candidate| candidate.id == item.id).map(|candidate| (item.id.clone(), candidate.category.clone()))).collect() }),
        (Some(Task::Sorting(task)), SheetTask::Sorting(sheet)) => {
            let mut order: Vec<(usize, &crate::schema::SortingItem)> = task.items.iter().enumerate().filter(|(_, item)| sheet.items.iter().any(|presented| presented.id == item.id)).collect();
            order.sort_by(|a, b| a.1.value.total_cmp(&b.1.value).then(a.0.cmp(&b.0)));
            Answer::Sorting(SortingAnswer { order: order.iter().map(|(_, item)| item.id.clone()).collect(), guesses: BTreeMap::new() })
        }
        (Some(Task::Matching(task)), SheetTask::Matching(sheet)) => Answer::Matching(MatchingAnswer {
            assignments: sheet
                .dimensions
                .iter()
                .map(|dimension| {
                    let mut used = BTreeSet::new();
                    let cards = sheet
                        .items
                        .iter()
                        .filter_map(|item| {
                            let value = task.items.iter().find(|candidate| candidate.id == item.id)?.values.get(&dimension.id)?;
                            let card = (0..dimension.cards.len()).find(|&card| dimension.cards[card] == *value && !used.contains(&card))?;
                            used.insert(card);
                            Some((item.id.clone(), card))
                        })
                        .collect();
                    (dimension.id.clone(), cards)
                })
                .collect(),
        }),
        _ => Answer::Sorting(SortingAnswer { order: Vec::new(), guesses: BTreeMap::new() }),
    }
}

pub(crate) fn command_start(learner: &str, run: &str) -> Command {
    Command::StartRun { id: id(1), learner: learner.to_string(), run: run.to_string(), quiz: "energy".to_string() }
}

pub(crate) fn play_perfect(state: &mut LearnerState, run: &str, quizzes: &BTreeMap<Slug, LoadedQuiz>, catalog: &Catalog, now: Timestamp) -> Vec<Event> {
    let context = LearnerContext { now, catalog, quizzes, limits: &DEFAULT_LIMITS };
    let learner = state.learner.clone();
    let mut events = step(state, &command_start(&learner, run), &context);
    let current = &quizzes["energy"].quiz;
    let sheet = sheet_of(current, run_seed(run));
    for task in &sheet.tasks {
        let command = Command::RecordAnswer { id: id(2), learner: learner.clone(), run: run.to_string(), task: task.id().clone(), answer: perfect(current, task) };
        events.extend(step(state, &command, &context));
    }
    events.extend(step(state, &Command::SubmitRun { id: id(3), learner, run: run.to_string() }, &context));
    events
}

#[test]
fn start_run_follows_the_decision_table() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let context = LearnerContext { now: 100, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    assert_eq!(decide_learner(&empty_learner_state(ALICE), &command_start(ALICE, RUN), &context), Decision::Rejection(Rejection::UnknownLearner));
    let mut state = registered(ALICE, 1);
    assert_eq!(decide_learner(&state, &command_start(BOB, RUN), &context), Decision::Rejection(Rejection::UnknownLearner));
    let unknown = Command::StartRun { id: id(1), learner: ALICE.to_string(), run: RUN.to_string(), quiz: "cooling".to_string() };
    assert_eq!(decide_learner(&state, &unknown, &context), Decision::Rejection(Rejection::UnknownQuiz));
    let started = decide_learner(&state, &command_start(ALICE, RUN), &context);
    assert_eq!(started, Decision::Events(vec![Event::RunStarted { learner: ALICE.to_string(), run: RUN.to_string(), quiz: "energy".to_string(), revision: REVISION.to_string(), seed: run_seed(RUN), at: 100 }]));
    fold(&mut state, &started);
    assert_eq!(decide_learner(&state, &command_start(ALICE, RUN), &context), Decision::Rejection(Rejection::RunOpen));
    assert_eq!(decide_learner(&state, &command_start(ALICE, &id(7)), &context), Decision::Rejection(Rejection::RunOpen));
    let revised = quizzes_revised();
    let context = LearnerContext { now: 200, catalog: &catalog, quizzes: &revised, limits: &DEFAULT_LIMITS };
    assert_eq!(
        decide_learner(&state, &command_start(ALICE, &id(7)), &context),
        Decision::Events(vec![
            Event::RunVoided { learner: ALICE.to_string(), run: RUN.to_string(), at: 200 },
            Event::RunStarted { learner: ALICE.to_string(), run: id(7), quiz: "energy".to_string(), revision: REVISED.to_string(), seed: run_seed(&id(7)), at: 200 },
        ])
    );
    state.runs[0].status = RunStatus::Submitted;
    assert_eq!(decide_learner(&state, &command_start(ALICE, RUN), &context), Decision::Rejection(Rejection::RunClosed));
}

fn quizzes_revised() -> BTreeMap<Slug, LoadedQuiz> {
    quizzes(REVISED)
}

#[test]
fn record_answer_follows_the_decision_table() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let context = LearnerContext { now: 100, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let mut state = registered(ALICE, 1);
    let record = |run: &str, task: &str, answer: Answer| Command::RecordAnswer { id: id(2), learner: ALICE.to_string(), run: run.to_string(), task: task.to_string(), answer };
    let sorting = |ids: &[&str]| Answer::Sorting(SortingAnswer { order: ids.iter().map(|id| (*id).to_string()).collect(), guesses: BTreeMap::new() });
    assert_eq!(decide_learner(&state, &record(RUN, "power", sorting(&[])), &context), Decision::Rejection(Rejection::UnknownRun));
    step(&mut state, &command_start(ALICE, RUN), &context);
    let sheet = sheet_of(&quiz(), run_seed(RUN));
    let Some(SheetTask::Sorting(power)) = sheet.tasks.iter().find(|task| task.id() == "power") else { unreachable!() };
    let presented: Vec<&str> = power.items.iter().map(|item| item.id.as_str()).collect();
    assert_eq!(decide_learner(&state, &record(RUN, "lighting", sorting(&presented)), &context), Decision::Rejection(Rejection::UnknownTask));
    assert_eq!(decide_learner(&state, &record(RUN, "power", sorting(&presented[1..])), &context), Decision::Rejection(Rejection::AnswerInvalid));
    let recorded = decide_learner(&state, &record(RUN, "power", sorting(&presented)), &context);
    assert_eq!(recorded, Decision::Events(vec![Event::AnswerRecorded { learner: ALICE.to_string(), run: RUN.to_string(), task: "power".to_string(), answer: sorting(&presented), at: 100 }]));
    fold(&mut state, &recorded);
    let mut reversed = presented.clone();
    reversed.reverse();
    step(&mut state, &record(RUN, "power", sorting(&reversed)), &context);
    assert_eq!(state.runs[0].answers.get("power"), Some(&sorting(&reversed)));
    let revised = quizzes_revised();
    let stale = LearnerContext { now: 150, catalog: &catalog, quizzes: &revised, limits: &DEFAULT_LIMITS };
    assert_eq!(decide_learner(&state, &record(RUN, "power", sorting(&presented)), &stale), Decision::Rejection(Rejection::QuizRevised));
    evolve_learner(&mut state, &Event::RunVoided { learner: ALICE.to_string(), run: RUN.to_string(), at: 160 });
    assert_eq!(decide_learner(&state, &record(RUN, "power", sorting(&presented)), &context), Decision::Rejection(Rejection::RunClosed));
}

#[test]
fn submit_run_scores_awards_badges_and_closes_the_run() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let context = LearnerContext { now: 300, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let mut state = registered(ALICE, 1);
    step(&mut state, &command_start(ALICE, RUN), &context);
    let submit = Command::SubmitRun { id: id(3), learner: ALICE.to_string(), run: RUN.to_string() };
    assert_eq!(decide_learner(&state, &submit, &context), Decision::Rejection(Rejection::RunIncomplete));
    let mut replay = registered(ALICE, 1);
    let events = play_perfect(&mut replay, RUN, &quizzes, &catalog, 300);
    let types: Vec<&str> = events.iter().map(Event::type_name).collect();
    assert_eq!(types, ["run-started", "answer-recorded", "answer-recorded", "answer-recorded", "run-submitted", "badge-awarded", "badge-awarded", "badge-awarded"]);
    let awarded: Vec<&str> = events.iter().filter_map(|event| if let Event::BadgeAwarded { badge, .. } = event { Some(badge.as_str()) } else { None }).collect();
    assert_eq!(awarded, ["perfect-energy", "sorter", "done"]);
    let Some(Event::RunSubmitted { result, .. }) = events.iter().find(|event| event.type_name() == "run-submitted") else { unreachable!() };
    assert_eq!(result.score, 1.0);
    assert_eq!((replay.runs[0].status, replay.runs[0].submitted_at, replay.badges.len(), replay.runs[0].recorded), (RunStatus::Submitted, Some(300), 3, 3));
    assert_eq!(decide_learner(&replay, &submit, &context), Decision::Rejection(Rejection::RunClosed));
    let second = play_perfect(&mut replay, &id(8), &quizzes, &catalog, 400);
    assert!(second.iter().all(|event| event.type_name() != "badge-awarded"));
}

#[test]
fn submit_run_voids_a_run_of_a_revised_quiz() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let mut state = registered(ALICE, 1);
    step(&mut state, &command_start(ALICE, RUN), &LearnerContext { now: 10, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS });
    let revised = quizzes_revised();
    let decision = decide_learner(&state, &Command::SubmitRun { id: id(3), learner: ALICE.to_string(), run: RUN.to_string() }, &LearnerContext { now: 20, catalog: &catalog, quizzes: &revised, limits: &DEFAULT_LIMITS });
    assert_eq!(decision, Decision::Events(vec![Event::RunVoided { learner: ALICE.to_string(), run: RUN.to_string(), at: 20 }]));
    fold(&mut state, &decision);
    assert_eq!(state.runs[0].status, RunStatus::Voided);
}

#[test]
fn evolve_ignores_foreign_facts_and_takes_the_latest_identity() {
    let mut state = registered(ALICE, 1);
    evolve_learner(&mut state, &Event::LearnerRegistered { learner: ALICE.to_string(), identity: Identity::Anonymous, at: 2 });
    evolve_learner(&mut state, &Event::RunStarted { learner: BOB.to_string(), run: RUN.to_string(), quiz: "energy".to_string(), revision: REVISION.to_string(), seed: 1, at: 3 });
    assert_eq!((state.identity, state.runs.len()), (Some(Identity::Anonymous), 0));
}

pub(crate) fn replay(vectors: &serde_json::Value, sequence: &serde_json::Value) -> (Vec<Decision>, LearnerState, BTreeMap<Slug, LoadedQuiz>) {
    use crate::schema::tests::{entries, typed};
    let catalog: Catalog = typed(&vectors["catalog"]);
    let quizzes: Vec<Quiz> = entries(&vectors["quizzes"]).iter().map(typed).collect();
    let mut revisions: BTreeMap<Slug, String> = typed(&vectors["revisions"]);
    let loaded = |revisions: &BTreeMap<Slug, String>| quizzes.iter().map(|quiz| (quiz.id.clone(), LoadedQuiz { quiz: quiz.clone(), revision: revisions.get(&quiz.id).cloned().unwrap_or_default() })).collect::<BTreeMap<_, _>>();
    let limits: Limits = typed(sequence.get("limits").unwrap_or(&vectors["limits"]));
    let mut state = empty_learner_state(sequence["learner"].as_str().unwrap_or_default());
    for event in entries(&sequence["given"]) {
        evolve_learner(&mut state, &typed(event));
    }
    let mut decisions = Vec::new();
    for step in entries(&sequence["steps"]) {
        if let Some(changed) = step.get("revisions") {
            revisions.extend(typed::<BTreeMap<Slug, String>>(changed));
        }
        let quizzes = loaded(&revisions);
        let context = LearnerContext { now: step["now"].as_u64().unwrap_or_default(), catalog: &catalog, quizzes: &quizzes, limits: &limits };
        let decision = decide_learner(&state, &typed(&step["command"]), &context);
        fold(&mut state, &decision);
        decisions.push(decision);
    }
    let quizzes = loaded(&revisions);
    (decisions, state, quizzes)
}


#[test]
fn a_handle_registers_its_first_claim_and_refuses_every_later_one() {
    let key = "ada lovelace";
    let identify = |learner: &str, identity: Identity| Command::IdentifyLearner { id: id(9), learner: learner.to_string(), identity };
    let mut handle = empty_handle_state(key);
    let first = decide_handle(&handle, &identify(ALICE, Identity::Name { handle: " Ada  Lovelace ".to_string() }), 5);
    assert_eq!(first, Decision::Events(vec![Event::LearnerRegistered { learner: ALICE.to_string(), identity: Identity::Name { handle: "Ada Lovelace".to_string() }, at: 5 }]));
    if let Decision::Events(events) = &first {
        events.iter().for_each(|event| evolve_handle(&mut handle, event));
    }
    assert_eq!(handle, HandleState { key: key.to_string(), holder: Some(ALICE.to_string()) });
    assert_eq!(decide_handle(&handle, &identify(BOB, Identity::Pseudonym { handle: "ADA LOVELACE".to_string() }), 6), Decision::Rejection(Rejection::HandleClaimed));
    assert_eq!(decide_handle(&handle, &identify(ALICE, Identity::Name { handle: "Ada Lovelace".to_string() }), 7), Decision::Rejection(Rejection::HandleClaimed));
    let free = empty_handle_state(key);
    assert_eq!(decide_handle(&free, &identify(BOB, Identity::Anonymous), 7), Decision::Rejection(Rejection::HandleInvalid));
    assert_eq!(decide_handle(&free, &identify(BOB, Identity::Pseudonym { handle: "   ".to_string() }), 8), Decision::Rejection(Rejection::HandleInvalid));
    assert_eq!(decide_handle(&free, &identify(BOB, Identity::Pseudonym { handle: "Bob".to_string() }), 8), Decision::Rejection(Rejection::HandleInvalid));
    assert_eq!(decide_handle(&free, &identify("bob", Identity::Pseudonym { handle: "Ada Lovelace".to_string() }), 8), Decision::Rejection(Rejection::IdInvalid));
    assert_eq!(decide_handle(&free, &command_start(BOB, RUN), 9), Decision::Rejection(Rejection::HandleInvalid));
    let mut anonymous = empty_handle_state(key);
    evolve_handle(&mut anonymous, &Event::LearnerRegistered { learner: BOB.to_string(), identity: Identity::Anonymous, at: 10 });
    assert_eq!(anonymous.holder, None);
    assert_eq!(serde_json::from_str::<HandleState>(r#"{"corrupt":"x"}"#).ok(), None);
}

#[test]
fn an_anonymous_learner_registers_once_in_its_own_stream() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let context = LearnerContext { now: 5, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let identify = |learner: &str, identity: Identity| Command::IdentifyLearner { id: id(9), learner: learner.to_string(), identity };
    let mut state = empty_learner_state(ALICE);
    let decision = decide_learner(&state, &identify(ALICE, Identity::Anonymous), &context);
    assert_eq!(decision, Decision::Events(vec![Event::LearnerRegistered { learner: ALICE.to_string(), identity: Identity::Anonymous, at: 5 }]));
    fold(&mut state, &decision);
    assert_eq!(decide_learner(&state, &identify(ALICE, Identity::Anonymous), &context), Decision::Rejection(Rejection::LearnerExists));
    assert_eq!(decide_learner(&empty_learner_state(ALICE), &identify(ALICE, Identity::Name { handle: "Ada".to_string() }), &context), Decision::Rejection(Rejection::HandleInvalid));
    assert_eq!(decide_learner(&empty_learner_state(ALICE), &identify(BOB, Identity::Anonymous), &context), Decision::Rejection(Rejection::UnknownLearner));
    assert_eq!(decide_learner(&state, &command_start(ALICE, "run-1"), &context), Decision::Rejection(Rejection::IdInvalid));
}

#[test]
fn caps_refuse_registrations_starts_and_answers_beyond_them() {
    assert_eq!(registration_rejection(DEFAULT_LIMITS.learners - 1, &DEFAULT_LIMITS), None);
    assert_eq!(registration_rejection(DEFAULT_LIMITS.learners, &DEFAULT_LIMITS), Some(Rejection::RosterFull));
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let limits = Limits { learners: 10, runs_per_quiz: 1, runs: 1, answers_per_run: 3 };
    let capped = LearnerContext { now: 10, catalog: &catalog, quizzes: &quizzes, limits: &limits };
    let mut state = registered(ALICE, 1);
    step(&mut state, &command_start(ALICE, RUN), &capped);
    let sheet = sheet_of(&quiz(), run_seed(RUN));
    for task in &sheet.tasks {
        step(&mut state, &Command::RecordAnswer { id: id(2), learner: ALICE.to_string(), run: RUN.to_string(), task: task.id().clone(), answer: perfect(&quiz(), task) }, &capped);
    }
    assert_eq!(state.runs[0].recorded, 3);
    let again = Command::RecordAnswer { id: id(2), learner: ALICE.to_string(), run: RUN.to_string(), task: sheet.tasks[0].id().clone(), answer: perfect(&quiz(), &sheet.tasks[0]) };
    assert_eq!(decide_learner(&state, &again, &capped), Decision::Rejection(Rejection::AnswersExhausted));
    step(&mut state, &Command::SubmitRun { id: id(3), learner: ALICE.to_string(), run: RUN.to_string() }, &capped);
    assert_eq!(state.runs[0].status, RunStatus::Submitted);
    assert_eq!(decide_learner(&state, &command_start(ALICE, &id(8)), &capped), Decision::Rejection(Rejection::RunsExhausted));
    let open = LearnerContext { limits: &DEFAULT_LIMITS, ..capped };
    assert!(matches!(decide_learner(&state, &command_start(ALICE, &id(8)), &open), Decision::Events(_)));
}

#[test]
fn shared_registrations_and_quotas_of_the_python_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    let vectors = fixture("learner-lifecycle");
    for sequence in entries(&vectors["registrations"]).iter().chain(entries(&vectors["malformed"]["registrations"])) {
        let mut handle = empty_handle_state(sequence["key"].as_str().unwrap_or_default());
        for (index, step) in entries(&sequence["steps"]).iter().enumerate() {
            let decision = decide_handle(&handle, &typed(&step["command"]), step["now"].as_u64().unwrap_or_default());
            assert_close(&format!("registrations/{}/{index}", sequence["id"]), &json(&decision), &step["expected"]);
            if let Decision::Events(events) = &decision {
                events.iter().for_each(|event| evolve_handle(&mut handle, event));
            }
        }
    }
    for vector in entries(&vectors["quotas"]) {
        assert_close(&format!("quotas/{}", vector["id"]), &json(&registration_rejection(vector["learners"].as_u64().unwrap_or_default(), &typed::<Limits>(&vector["limits"]))), &vector["expected"]);
    }
}

#[test]
fn shared_learner_decisions_of_the_python_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json};
    let vectors = fixture("learner-lifecycle");
    for sequence in entries(&vectors["learners"]).iter().chain(entries(&vectors["malformed"]["learners"])) {
        let (decisions, _, _) = replay(&vectors, sequence);
        for (index, (decision, step)) in decisions.iter().zip(entries(&sequence["steps"])).enumerate() {
            assert_close(&format!("learners/{}/{index}", sequence["id"]), &json(decision), &step["expected"]);
        }
    }
}

#[test]
fn decisions_serialize_as_events_or_rejection() {
    assert_eq!(serde_json::to_string(&Decision::Rejection(Rejection::RunOpen)).ok().as_deref(), Some(r#"{"rejection":"run-open"}"#));
    assert_eq!(serde_json::to_string(&Decision::Events(vec![Event::RunVoided { learner: ALICE.to_string(), run: RUN.to_string(), at: 1 }])).ok().as_deref(), Some(r#"{"events":[{"type":"run-voided","learner":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","run":"0123456789abcdef0123456789abcdef","at":1}]}"#));
    let state = registered(ALICE, 4);
    let json = serde_json::to_string(&state).unwrap_or_default();
    assert_eq!(serde_json::from_str::<LearnerState>(&json).ok(), Some(state));
    assert_eq!(serde_json::from_str::<Event>(r#"{"type":"learner-recalled","learner":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","at":1}"#).ok(), None);
}
