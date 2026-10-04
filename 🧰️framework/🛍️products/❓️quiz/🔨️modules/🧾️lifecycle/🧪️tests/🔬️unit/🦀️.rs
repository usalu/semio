//! 🎞️ Unit tests of the lifecycle: handles, roster and learner decision tables, challenges, the clock of timed runs and the shared sequences; also the replay kit.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::challenge::CLOCK_LEAD;
use crate::schema::{Badge, BadgeRule, ClassificationAnswer, Introduction, MatchingAnswer, SheetTask, SortingAnswer, Task, TaskKind, CHALLENGES, DEFAULT_LIMITS, MAX_TIMESTAMP};
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
        badges: vec![badge("perfect-energy", BadgeRule::PerfectQuiz { quiz: "energy".to_string(), challenge: None }), badge("sorter", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None, challenge: None }), badge("done", BadgeRule::CompletedQuizzes {})],
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
            Answer::Sorting(SortingAnswer { order: order.iter().map(|(_, item)| item.id.clone()).collect(), guesses: sheet.keys.is_none().then(|| order.iter().map(|(_, item)| (item.id.clone(), item.value)).collect()) })
        }
        (Some(Task::Matching(task)), SheetTask::Matching(sheet)) => {
            let value = |item: &crate::schema::SheetItem, dimension: &str| task.items.iter().find(|candidate| candidate.id == item.id)?.values.get(dimension).copied();
            let hidden = sheet.dimensions.iter().any(|dimension| dimension.cards.is_none());
            let guesses = sheet.dimensions.iter().map(|dimension| (dimension.id.clone(), sheet.items.iter().filter_map(|item| Some((item.id.clone(), value(item, &dimension.id)?))).collect())).collect();
            let assignments = sheet
                .dimensions
                .iter()
                .map(|dimension| {
                    let (deck, mut used) = (dimension.cards.as_deref().unwrap_or_default(), BTreeSet::new());
                    let cards = sheet
                        .items
                        .iter()
                        .filter_map(|item| {
                            let value = value(item, &dimension.id)?;
                            let card = (0..deck.len()).find(|&card| deck[card] == value && !used.contains(&card))?;
                            used.insert(card);
                            Some((item.id.clone(), card))
                        })
                        .collect();
                    (dimension.id.clone(), cards)
                })
                .collect();
            Answer::Matching(if hidden { MatchingAnswer { assignments: None, guesses: Some(guesses) } } else { MatchingAnswer { assignments: Some(assignments), guesses: None } })
        }
        _ => Answer::Sorting(SortingAnswer { order: Vec::new(), guesses: None }),
    }
}

pub(crate) fn command_start_at(learner: &str, run: &str, challenge: Challenge, at: Timestamp) -> Command {
    Command::StartRun { id: id(1), learner: learner.to_string(), run: run.to_string(), quiz: "energy".to_string(), challenge, at }
}

pub(crate) fn command_start(learner: &str, run: &str, at: Timestamp) -> Command {
    command_start_at(learner, run, Challenge::Medium, at)
}

pub(crate) fn play_perfect_at(state: &mut LearnerState, run: &str, challenge: Challenge, quizzes: &BTreeMap<Slug, LoadedQuiz>, catalog: &Catalog, now: Timestamp) -> Vec<Event> {
    let context = LearnerContext { now, catalog, quizzes, limits: &DEFAULT_LIMITS };
    let learner = state.learner.clone();
    let mut events = step(state, &command_start_at(&learner, run, challenge, context.now), &context);
    let current = &quizzes["energy"].quiz;
    let sheet = sheet_of(current, run_seed(run), challenge);
    for task in &sheet.tasks {
        if task.seconds().is_some() {
            events.extend(step(state, &Command::OpenTask { id: id(4), learner: learner.clone(), run: run.to_string(), task: task.id().clone(), at: now }, &context));
        }
        let command = Command::RecordAnswer { id: id(2), learner: learner.clone(), run: run.to_string(), task: task.id().clone(), answer: perfect(current, task), at: now };
        events.extend(step(state, &command, &context));
    }
    events.extend(step(state, &Command::SubmitRun { id: id(3), learner, run: run.to_string() }, &context));
    events
}

pub(crate) fn play_perfect(state: &mut LearnerState, run: &str, quizzes: &BTreeMap<Slug, LoadedQuiz>, catalog: &Catalog, now: Timestamp) -> Vec<Event> {
    play_perfect_at(state, run, Challenge::Medium, quizzes, catalog, now)
}

#[test]
fn start_run_follows_the_decision_table() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let context = LearnerContext { now: 100, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    assert_eq!(decide_learner(&empty_learner_state(ALICE), &command_start(ALICE, RUN, context.now), &context), Decision::Rejection(Rejection::UnknownLearner));
    let mut state = registered(ALICE, 1);
    assert_eq!(decide_learner(&state, &command_start(BOB, RUN, context.now), &context), Decision::Rejection(Rejection::UnknownLearner));
    let unknown = Command::StartRun { id: id(1), learner: ALICE.to_string(), run: RUN.to_string(), quiz: "cooling".to_string(), challenge: Challenge::Medium, at: 100 };
    assert_eq!(decide_learner(&state, &unknown, &context), Decision::Rejection(Rejection::UnknownQuiz));
    let started = decide_learner(&state, &command_start(ALICE, RUN, context.now), &context);
    assert_eq!(started, Decision::Events(vec![Event::RunStarted { learner: ALICE.to_string(), run: RUN.to_string(), quiz: "energy".to_string(), challenge: Challenge::Medium, revision: REVISION.to_string(), seed: run_seed(RUN), at: 100 }]));
    fold(&mut state, &started);
    assert_eq!(decide_learner(&state, &command_start(ALICE, RUN, context.now), &context), Decision::Rejection(Rejection::RunOpen));
    assert_eq!(decide_learner(&state, &command_start(ALICE, &id(7), context.now), &context), Decision::Rejection(Rejection::RunOpen));
    let revised = quizzes_revised();
    let context = LearnerContext { now: 200, catalog: &catalog, quizzes: &revised, limits: &DEFAULT_LIMITS };
    assert_eq!(
        decide_learner(&state, &command_start(ALICE, &id(7), context.now), &context),
        Decision::Events(vec![
            Event::RunVoided { learner: ALICE.to_string(), run: RUN.to_string(), at: 200 },
            Event::RunStarted { learner: ALICE.to_string(), run: id(7), quiz: "energy".to_string(), challenge: Challenge::Medium, revision: REVISED.to_string(), seed: run_seed(&id(7)), at: 200 },
        ])
    );
    state.runs[0].status = RunStatus::Submitted;
    assert_eq!(decide_learner(&state, &command_start(ALICE, RUN, context.now), &context), Decision::Rejection(Rejection::RunClosed));
}

fn quizzes_revised() -> BTreeMap<Slug, LoadedQuiz> {
    quizzes(REVISED)
}

fn submit(run: &str) -> Command {
    Command::SubmitRun { id: id(3), learner: ALICE.to_string(), run: run.to_string() }
}

fn open(run: &str, task: &str, at: Timestamp) -> Command {
    Command::OpenTask { id: id(4), learner: ALICE.to_string(), run: run.to_string(), task: task.to_string(), at }
}

fn record(run: &str, task: &str, answer: Answer, at: Timestamp) -> Command {
    Command::RecordAnswer { id: id(2), learner: ALICE.to_string(), run: run.to_string(), task: task.to_string(), answer, at }
}

fn solved(challenge: Challenge, task: &str) -> Answer {
    let sheet = sheet_of(&quiz(), run_seed(RUN), challenge);
    sheet.tasks.iter().find(|candidate| candidate.id() == task).map_or_else(|| unreachable!(), |sheet_task| perfect(&quiz(), sheet_task))
}

#[test]
fn a_run_starts_at_the_challenge_of_its_command_and_keeps_it() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let context = LearnerContext { now: 100, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    for challenge in CHALLENGES {
        let mut state = registered(ALICE, 1);
        let started = decide_learner(&state, &command_start_at(ALICE, RUN, challenge, context.now), &context);
        assert_eq!(started, Decision::Events(vec![Event::RunStarted { learner: ALICE.to_string(), run: RUN.to_string(), quiz: "energy".to_string(), challenge, revision: REVISION.to_string(), seed: run_seed(RUN), at: 100 }]));
        fold(&mut state, &started);
        assert_eq!((state.runs[0].challenge, state.runs[0].opened.len(), state.runs[0].seed), (challenge, 0, run_seed(RUN)));
        if challenge == Challenge::Expert {
            continue;
        }
        let plain = Answer::Sorting(SortingAnswer { order: presented(challenge, "power"), guesses: None });
        let shown = challenge <= Challenge::Medium;
        assert_eq!(decide_learner(&state, &record(RUN, "power", plain.clone(), 100), &context), Decision::Events(vec![Event::AnswerRecorded { learner: ALICE.to_string(), run: RUN.to_string(), task: "power".to_string(), answer: plain, at: 100 }]), "{challenge:?}");
        assert_eq!(matches!(decide_learner(&state, &record(RUN, "power", solved(Challenge::Hard, "power"), 100), &context), Decision::Events(_)), !shown, "the sheet of a decision is the sheet at the run's challenge: {challenge:?}");
        assert_eq!(matches!(decide_learner(&state, &record(RUN, "buildings", solved(Challenge::Medium, "buildings"), 100), &context), Decision::Events(_)), shown, "{challenge:?}");
    }
}

fn presented(challenge: Challenge, task: &str) -> Vec<Slug> {
    sheet_of(&quiz(), run_seed(RUN), challenge).tasks.iter().find(|candidate| candidate.id() == task).map(|sheet_task| sheet_task.items().iter().map(|item| item.id.clone()).collect()).unwrap_or_default()
}

#[test]
fn starting_a_quiz_at_another_challenge_voids_its_open_run() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let context = LearnerContext { now: 100, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let mut state = registered(ALICE, 1);
    step(&mut state, &command_start_at(ALICE, RUN, Challenge::Expert, context.now), &context);
    assert_eq!(decide_learner(&state, &command_start_at(ALICE, &id(7), Challenge::Expert, context.now), &context), Decision::Rejection(Rejection::RunOpen));
    assert_eq!(decide_learner(&state, &command_start_at(ALICE, RUN, Challenge::Easy, context.now), &context), Decision::Rejection(Rejection::RunOpen), "a run id already used is refused before anything is voided");
    let later = LearnerContext { now: 250, ..context };
    let switched = decide_learner(&state, &command_start_at(ALICE, &id(7), Challenge::Easy, later.now), &later);
    assert_eq!(
        switched,
        Decision::Events(vec![
            Event::RunVoided { learner: ALICE.to_string(), run: RUN.to_string(), at: 250 },
            Event::RunStarted { learner: ALICE.to_string(), run: id(7), quiz: "energy".to_string(), challenge: Challenge::Easy, revision: REVISION.to_string(), seed: run_seed(&id(7)), at: 250 },
        ])
    );
    fold(&mut state, &switched);
    assert_eq!(state.runs.iter().map(|run| (run.status, run.challenge)).collect::<Vec<_>>(), [(RunStatus::Voided, Challenge::Expert), (RunStatus::Open, Challenge::Easy)]);
    assert_eq!(decide_learner(&state, &command_start_at(ALICE, &id(8), Challenge::Easy, later.now), &later), Decision::Rejection(Rejection::RunOpen));
    assert_eq!(decide_learner(&state, &open(RUN, "power", 250), &later), Decision::Rejection(Rejection::RunClosed));
    for challenge in [Challenge::Medium, Challenge::Hard, Challenge::Expert] {
        let Decision::Events(events) = decide_learner(&state, &command_start_at(ALICE, &id(8), challenge, later.now), &later) else { panic!("{challenge:?} was refused") };
        assert_eq!(events.iter().map(Event::type_name).collect::<Vec<_>>(), ["run-voided", "run-started"]);
    }
    let revised = quizzes_revised();
    let stale = LearnerContext { now: 300, quizzes: &revised, ..context };
    let Decision::Events(events) = decide_learner(&state, &command_start_at(ALICE, &id(8), Challenge::Easy, stale.now), &stale) else { unreachable!() };
    assert_eq!(events.iter().map(Event::type_name).collect::<Vec<_>>(), ["run-voided", "run-started"], "a stale open run is voided whatever the challenge");
    let limits = Limits { runs: 0, ..DEFAULT_LIMITS };
    assert_eq!(decide_learner(&state, &command_start_at(ALICE, &id(8), Challenge::Hard, later.now), &LearnerContext { limits: &limits, ..later }), Decision::Rejection(Rejection::RunsExhausted), "the caps hold before a run is voided");
}

#[test]
fn open_task_follows_the_decision_table() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let context = LearnerContext { now: 500, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let mut state = registered(ALICE, 1);
    assert_eq!(decide_learner(&state, &open(RUN, "power", 400), &context), Decision::Rejection(Rejection::UnknownRun));
    for challenge in [Challenge::Easy, Challenge::Medium, Challenge::Hard] {
        let mut untimed = registered(ALICE, 1);
        step(&mut untimed, &command_start_at(ALICE, RUN, challenge, context.now), &context);
        assert_eq!(decide_learner(&untimed, &open(RUN, "power", 500), &context), Decision::Rejection(Rejection::RunUntimed), "{challenge:?}");
        assert_eq!(decide_learner(&untimed, &open(RUN, "lighting", 500), &context), Decision::Rejection(Rejection::RunUntimed), "a run without a clock refuses before the task is looked up");
    }
    step(&mut state, &command_start_at(ALICE, RUN, Challenge::Expert, 300), &LearnerContext { now: 300, ..context });
    assert_eq!(decide_learner(&state, &open(RUN, "lighting", 400), &context), Decision::Rejection(Rejection::UnknownTask));
    assert_eq!(decide_learner(&state, &open(RUN, "Power", 400), &context), Decision::Rejection(Rejection::IdInvalid));
    assert_eq!(decide_learner(&state, &Command::OpenTask { id: id(4), learner: BOB.to_string(), run: RUN.to_string(), task: "power".to_string(), at: 400 }, &context), Decision::Rejection(Rejection::UnknownLearner));
    let revised = quizzes_revised();
    assert_eq!(decide_learner(&state, &open(RUN, "power", 400), &LearnerContext { quizzes: &revised, ..context }), Decision::Rejection(Rejection::QuizRevised));
    let opened = |at: Timestamp| Decision::Events(vec![Event::TaskOpened { learner: ALICE.to_string(), run: RUN.to_string(), task: "power".to_string(), at }]);
    for (acted_at, counted) in [(400, 400), (300, 300), (500, 500), (0, 300), (299, 300), (501, 501), (300_200, 300_200)] {
        for now in [200, 500, 900_000] {
            assert_eq!(decide_learner(&state, &open(RUN, "power", acted_at), &LearnerContext { now, ..context }), opened(counted), "acted at {acted_at}, decided at {now}: within the lead the decision time never enters the instant");
        }
    }
    for (acted_at, now, counted) in [(300_201, 200, 300_200), (MAX_TIMESTAMP, 200, 300_200), (MAX_TIMESTAMP, 900_000, 1_200_000), (3_600_000, 0, 300_000), (3_600_000, 0, CLOCK_LEAD)] {
        assert_eq!(decide_learner(&state, &open(RUN, "power", acted_at), &LearnerContext { now, ..context }), opened(counted), "acted at {acted_at}, decided at {now}: a claim beyond the lead is lowered to it");
    }
    for beyond in [MAX_TIMESTAMP + 1, u64::MAX] {
        assert_eq!(decide_learner(&state, &open(RUN, "power", beyond), &context), Decision::Rejection(Rejection::IdInvalid), "an instant beyond a safe integer is malformed");
    }
    let first = decide_learner(&state, &open(RUN, "power", 400), &context);
    fold(&mut state, &first);
    assert_eq!(state.runs[0].opened, BTreeMap::from([("power".to_string(), 400)]));
    for again in [400, 450, 0] {
        assert_eq!(decide_learner(&state, &open(RUN, "power", again), &context), Decision::Rejection(Rejection::AlreadyOpened), "a task opens once and keeps its first opening");
    }
    step(&mut state, &open(RUN, "buildings", 480), &context);
    assert_eq!(state.runs[0].opened, BTreeMap::from([("buildings".to_string(), 480), ("power".to_string(), 400)]));
    step(&mut state, &submit(RUN), &context);
    assert_eq!(decide_learner(&state, &open(RUN, "standards", 490), &context), Decision::Rejection(Rejection::RunClosed));
}

#[test]
fn a_timed_run_takes_answers_for_opened_tasks_within_their_seconds() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let at = |now: Timestamp| LearnerContext { now, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let mut state = registered(ALICE, 1);
    step(&mut state, &command_start_at(ALICE, RUN, Challenge::Expert, 1_000), &at(1_000));
    let answer = solved(Challenge::Expert, "power");
    let recorded = |at: Timestamp| Decision::Events(vec![Event::AnswerRecorded { learner: ALICE.to_string(), run: RUN.to_string(), task: "power".to_string(), answer: solved(Challenge::Expert, "power"), at }]);
    assert_eq!(decide_learner(&state, &record(RUN, "power", answer.clone(), 2_000), &at(2_000)), Decision::Rejection(Rejection::TaskUnopened));
    assert_eq!(decide_learner(&state, &record(RUN, "lighting", answer.clone(), 2_000), &at(2_000)), Decision::Rejection(Rejection::UnknownTask), "an unknown task is refused before the clock is asked");
    step(&mut state, &open(RUN, "power", 10_000), &at(10_000));
    let limit = 10_000 + 78 * 1_000;
    assert_eq!(sheet_of(&quiz(), run_seed(RUN), Challenge::Expert).tasks.iter().find(|task| task.id() == "power").and_then(SheetTask::seconds), Some(78));
    assert_eq!(decide_learner(&state, &record(RUN, "standards", solved(Challenge::Expert, "standards"), 11_000), &at(11_000)), Decision::Rejection(Rejection::TaskUnopened));
    for now in [5_000, 10_000, limit, limit + 1, 900_000, u64::MAX] {
        for (acted_at, counted) in [(20_000, 20_000), (10_000, 10_000), (limit, limit), (limit - 1, limit - 1), (0, 10_000), (9_999, 10_000), (70_000, 70_000)] {
            assert_eq!(decide_learner(&state, &record(RUN, "power", answer.clone(), acted_at), &at(now)), recorded(counted), "acted at {acted_at}, decided at {now}");
        }
        for acted_at in [limit + 1, 900_000, MAX_TIMESTAMP] {
            assert_eq!(decide_learner(&state, &record(RUN, "power", answer.clone(), acted_at), &at(now)), Decision::Rejection(Rejection::TimeUp), "acted at {acted_at}, decided at {now}");
        }
    }
    let invalid = Answer::Sorting(SortingAnswer { order: Vec::new(), guesses: None });
    assert_eq!(decide_learner(&state, &record(RUN, "power", invalid.clone(), limit), &at(limit)), Decision::Rejection(Rejection::AnswerInvalid));
    assert_eq!(decide_learner(&state, &record(RUN, "power", invalid, limit + 1), &at(limit + 1)), Decision::Rejection(Rejection::TimeUp), "time is up before the answer is looked at");
    step(&mut state, &record(RUN, "power", answer.clone(), 30_000), &at(900_000));
    assert_eq!((state.runs[0].recorded, state.runs[0].answers.get("power")), (1, Some(&answer)));
}

#[test]
fn an_untimed_run_raises_the_instant_of_an_answer_to_its_start_and_lowers_it_to_the_lead() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let at = |now: Timestamp| LearnerContext { now, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    for challenge in [Challenge::Easy, Challenge::Medium, Challenge::Hard] {
        let mut state = registered(ALICE, 1);
        step(&mut state, &command_start_at(ALICE, RUN, challenge, 1_000), &at(1_000));
        for (acted_at, now, counted) in [(5_000, 9_000, 5_000), (0, 9_000, 1_000), (999, 9_000, 1_000), (9_001, 9_000, 9_001), (309_000, 9_000, 309_000), (309_001, 9_000, 309_000), (MAX_TIMESTAMP, 9_000, 309_000), (5_000, 500, 5_000), (500, 500, 1_000), (5_000, u64::MAX, 5_000)] {
            let Decision::Events(events) = decide_learner(&state, &record(RUN, "power", solved(challenge, "power"), acted_at), &at(now)) else { panic!("{challenge:?} refused an answer acted at {acted_at}") };
            assert_eq!(events.iter().map(Event::at).collect::<Vec<_>>(), [counted], "{challenge:?}: acted at {acted_at}, decided at {now}");
        }
    }
}

#[test]
fn a_timed_run_is_submitted_as_it_stands_and_an_untimed_one_only_complete() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let context = LearnerContext { now: 300, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    for challenge in [Challenge::Easy, Challenge::Medium, Challenge::Hard] {
        let mut state = registered(ALICE, 1);
        step(&mut state, &command_start_at(ALICE, RUN, challenge, context.now), &context);
        assert_eq!(decide_learner(&state, &submit(RUN), &context), Decision::Rejection(Rejection::RunIncomplete), "{challenge:?}");
        step(&mut state, &record(RUN, "power", solved(challenge, "power"), 300), &context);
        step(&mut state, &record(RUN, "standards", solved(challenge, "standards"), 300), &context);
        assert_eq!(decide_learner(&state, &submit(RUN), &context), Decision::Rejection(Rejection::RunIncomplete), "{challenge:?}");
    }
    let mut hard = registered(ALICE, 1);
    step(&mut hard, &command_start_at(ALICE, RUN, Challenge::Hard, context.now), &context);
    for task in ["standards", "buildings"] {
        step(&mut hard, &record(RUN, task, solved(Challenge::Hard, task), 300), &context);
    }
    let Answer::Sorting(whole) = solved(Challenge::Hard, "power") else { unreachable!() };
    let mut guesses = whole.guesses.clone().unwrap_or_default();
    guesses.pop_last();
    step(&mut hard, &record(RUN, "power", Answer::Sorting(SortingAnswer { guesses: Some(guesses), ..whole }), 300), &context);
    assert_eq!((hard.runs[0].recorded, decide_learner(&hard, &submit(RUN), &context)), (3, Decision::Rejection(Rejection::RunIncomplete)), "a sorting that hides the keys waits for a guess per item");
    let mut empty = registered(ALICE, 1);
    step(&mut empty, &command_start_at(ALICE, RUN, Challenge::Expert, context.now), &context);
    let events = step(&mut empty, &submit(RUN), &context);
    assert_eq!(events.iter().map(Event::type_name).collect::<Vec<_>>(), ["run-submitted", "badge-awarded"]);
    let Some(Event::RunSubmitted { result, at, .. }) = events.first() else { unreachable!() };
    assert_eq!((result.score, result.points, result.challenge, *at, result.tasks.len()), (0.0, 0.0, Challenge::Expert, 300, 3));
    assert_eq!(events.iter().filter_map(|event| if let Event::BadgeAwarded { badge, .. } = event { Some(badge.as_str()) } else { None }).collect::<Vec<_>>(), ["done"]);
    let mut partial = registered(ALICE, 1);
    step(&mut partial, &command_start_at(ALICE, RUN, Challenge::Expert, context.now), &context);
    step(&mut partial, &open(RUN, "power", 300), &context);
    step(&mut partial, &record(RUN, "power", solved(Challenge::Expert, "power"), 300), &context);
    step(&mut partial, &open(RUN, "standards", 300), &context);
    let events = step(&mut partial, &submit(RUN), &LearnerContext { now: 900_000_000, ..context });
    let Some(Event::RunSubmitted { result, .. }) = events.first() else { unreachable!() };
    assert_eq!((result.score, result.points), (1.0 / 3.0, 1.0 / 3.0 * 400.0));
    assert_eq!(events.iter().filter_map(|event| if let Event::BadgeAwarded { badge, .. } = event { Some(badge.as_str()) } else { None }).collect::<Vec<_>>(), ["sorter", "done"]);
    assert_eq!((partial.runs[0].status, partial.runs[0].submitted_at), (RunStatus::Submitted, Some(900_000_000)));
}

#[test]
fn a_perfect_run_scores_one_and_earns_its_par_at_every_challenge() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    for (challenge, par) in CHALLENGES.into_iter().zip([100.0, 200.0, 300.0, 400.0]) {
        let mut state = registered(ALICE, 1);
        let events = play_perfect_at(&mut state, RUN, challenge, &quizzes, &catalog, 700);
        let answering: &[&str] = if challenge == Challenge::Expert { &["task-opened", "answer-recorded"] } else { &["answer-recorded"] };
        let expected: Vec<&str> = std::iter::once("run-started").chain(answering.iter().copied().cycle().take(3 * answering.len())).chain(["run-submitted", "badge-awarded", "badge-awarded", "badge-awarded"]).collect();
        assert_eq!(events.iter().map(Event::type_name).collect::<Vec<_>>(), expected, "{challenge:?}");
        let Some(result) = &state.runs[0].result else { unreachable!() };
        assert_eq!((result.score, result.points, result.challenge), (1.0, par, challenge));
        assert_eq!(state.runs[0].opened.len(), if challenge == Challenge::Expert { 3 } else { 0 });
    }
}

#[test]
fn a_badge_with_a_least_challenge_waits_for_a_run_that_meets_it() {
    let mut catalog = catalog();
    catalog.badges[0].rule = BadgeRule::PerfectQuiz { quiz: "energy".to_string(), challenge: Some(Challenge::Hard) };
    catalog.badges[1].rule = BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None, challenge: Some(Challenge::Medium) };
    let quizzes = quizzes(REVISION);
    let awarded = |events: &[Event]| events.iter().filter_map(|event| if let Event::BadgeAwarded { badge, .. } = event { Some(badge.clone()) } else { None }).collect::<Vec<_>>();
    let mut state = registered(ALICE, 1);
    assert_eq!(awarded(&play_perfect_at(&mut state, &id(11), Challenge::Easy, &quizzes, &catalog, 100)), ["done"]);
    assert_eq!(awarded(&play_perfect_at(&mut state, &id(12), Challenge::Medium, &quizzes, &catalog, 200)), ["sorter"]);
    assert_eq!(awarded(&play_perfect_at(&mut state, &id(13), Challenge::Expert, &quizzes, &catalog, 300)), ["perfect-energy"]);
    assert_eq!(awarded(&play_perfect_at(&mut state, &id(14), Challenge::Hard, &quizzes, &catalog, 400)), Vec::<String>::new());
}

#[test]
fn record_answer_follows_the_decision_table() {
    let catalog = catalog();
    let quizzes = quizzes(REVISION);
    let context = LearnerContext { now: 100, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let mut state = registered(ALICE, 1);
    let record = |run: &str, task: &str, answer: Answer| record(run, task, answer, 100);
    let sorting = |ids: &[&str]| Answer::Sorting(SortingAnswer { order: ids.iter().map(|id| (*id).to_string()).collect(), guesses: None });
    assert_eq!(decide_learner(&state, &record(RUN, "power", sorting(&[])), &context), Decision::Rejection(Rejection::UnknownRun));
    step(&mut state, &command_start(ALICE, RUN, context.now), &context);
    let sheet = sheet_of(&quiz(), run_seed(RUN), Challenge::Medium);
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
    step(&mut state, &command_start(ALICE, RUN, context.now), &context);
    let submit = submit(RUN);
    assert_eq!(decide_learner(&state, &submit, &context), Decision::Rejection(Rejection::RunIncomplete));
    let mut replay = registered(ALICE, 1);
    let events = play_perfect(&mut replay, RUN, &quizzes, &catalog, 300);
    let types: Vec<&str> = events.iter().map(Event::type_name).collect();
    assert_eq!(types, ["run-started", "answer-recorded", "answer-recorded", "answer-recorded", "run-submitted", "badge-awarded", "badge-awarded", "badge-awarded"]);
    let awarded: Vec<&str> = events.iter().filter_map(|event| if let Event::BadgeAwarded { badge, .. } = event { Some(badge.as_str()) } else { None }).collect();
    assert_eq!(awarded, ["perfect-energy", "sorter", "done"]);
    let Some(Event::RunSubmitted { result, .. }) = events.iter().find(|event| event.type_name() == "run-submitted") else { unreachable!() };
    assert_eq!((result.score, result.points, result.challenge), (1.0, 200.0, Challenge::Medium));
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
    step(&mut state, &command_start(ALICE, RUN, 10), &LearnerContext { now: 10, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS });
    let revised = quizzes_revised();
    let decision = decide_learner(&state, &submit(RUN), &LearnerContext { now: 20, catalog: &catalog, quizzes: &revised, limits: &DEFAULT_LIMITS });
    assert_eq!(decision, Decision::Events(vec![Event::RunVoided { learner: ALICE.to_string(), run: RUN.to_string(), at: 20 }]));
    fold(&mut state, &decision);
    assert_eq!(state.runs[0].status, RunStatus::Voided);
}

#[test]
fn evolve_ignores_foreign_facts_and_takes_the_latest_identity() {
    let mut state = registered(ALICE, 1);
    evolve_learner(&mut state, &Event::LearnerRegistered { learner: ALICE.to_string(), identity: Identity::Anonymous, at: 2 });
    evolve_learner(&mut state, &Event::RunStarted { learner: BOB.to_string(), run: RUN.to_string(), quiz: "energy".to_string(), challenge: Challenge::Expert, revision: REVISION.to_string(), seed: 1, at: 3 });
    evolve_learner(&mut state, &Event::TaskOpened { learner: ALICE.to_string(), run: RUN.to_string(), task: "power".to_string(), at: 4 });
    assert_eq!((state.identity.clone(), state.runs.len()), (Some(Identity::Anonymous), 0));
    evolve_learner(&mut state, &Event::RunStarted { learner: ALICE.to_string(), run: RUN.to_string(), quiz: "energy".to_string(), challenge: Challenge::Expert, revision: REVISION.to_string(), seed: 1, at: 5 });
    evolve_learner(&mut state, &Event::TaskOpened { learner: BOB.to_string(), run: RUN.to_string(), task: "power".to_string(), at: 6 });
    evolve_learner(&mut state, &Event::TaskOpened { learner: ALICE.to_string(), run: id(9), task: "power".to_string(), at: 7 });
    assert_eq!((state.runs[0].challenge, state.runs[0].opened.len()), (Challenge::Expert, 0));
    evolve_learner(&mut state, &Event::TaskOpened { learner: ALICE.to_string(), run: RUN.to_string(), task: "power".to_string(), at: 8 });
    assert_eq!(state.runs[0].opened, BTreeMap::from([("power".to_string(), 8)]));
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
    assert_eq!(decide_handle(&free, &command_start(BOB, RUN, 9), 9), Decision::Rejection(Rejection::HandleInvalid));
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
    assert_eq!(decide_learner(&state, &command_start(ALICE, "run-1", context.now), &context), Decision::Rejection(Rejection::IdInvalid));
}

#[test]
fn caps_refuse_registrations_starts_and_answers_beyond_them() {
    assert_eq!(registration_rejection(DEFAULT_LIMITS.learners - 1, &DEFAULT_LIMITS), None);
    assert_eq!(registration_rejection(DEFAULT_LIMITS.learners, &DEFAULT_LIMITS), Some(Rejection::RosterFull));
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let limits = Limits { learners: 10, runs_per_quiz: 1, runs: 1, answers_per_run: 3 };
    let capped = LearnerContext { now: 10, catalog: &catalog, quizzes: &quizzes, limits: &limits };
    let mut state = registered(ALICE, 1);
    step(&mut state, &command_start(ALICE, RUN, capped.now), &capped);
    let sheet = sheet_of(&quiz(), run_seed(RUN), Challenge::Medium);
    for task in &sheet.tasks {
        step(&mut state, &record(RUN, task.id(), perfect(&quiz(), task), 10), &capped);
    }
    assert_eq!(state.runs[0].recorded, 3);
    let again = record(RUN, sheet.tasks[0].id(), perfect(&quiz(), &sheet.tasks[0]), 10);
    assert_eq!(decide_learner(&state, &again, &capped), Decision::Rejection(Rejection::AnswersExhausted));
    step(&mut state, &submit(RUN), &capped);
    assert_eq!(state.runs[0].status, RunStatus::Submitted);
    assert_eq!(decide_learner(&state, &command_start(ALICE, &id(8), capped.now), &capped), Decision::Rejection(Rejection::RunsExhausted));
    let open = LearnerContext { limits: &DEFAULT_LIMITS, ..capped };
    assert!(matches!(decide_learner(&state, &command_start(ALICE, &id(8), open.now), &open), Decision::Events(_)));
}

#[test]
fn switching_the_challenge_counts_every_started_run_against_the_caps() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    for limits in [Limits { learners: 10, runs_per_quiz: 4, runs: 100, answers_per_run: 3 }, Limits { learners: 10, runs_per_quiz: 100, runs: 4, answers_per_run: 3 }] {
        let context = LearnerContext { now: 10, catalog: &catalog, quizzes: &quizzes, limits: &limits };
        let mut state = registered(ALICE, 1);
        for (index, challenge) in [Challenge::Easy, Challenge::Medium, Challenge::Easy, Challenge::Medium].into_iter().enumerate() {
            let events = step(&mut state, &command_start_at(ALICE, &id(20 + index as u8), challenge, 10), &context);
            assert_eq!(events.last().map(Event::type_name), Some("run-started"), "{limits:?}: start {index}");
        }
        assert_eq!(state.runs.iter().filter(|run| run.status == RunStatus::Voided).count(), 3);
        assert_eq!(decide_learner(&state, &command_start_at(ALICE, &id(30), Challenge::Hard, 10), &context), Decision::Rejection(Rejection::RunsExhausted), "{limits:?}: voided and open runs count");
    }
}

#[test]
fn a_run_starts_at_the_instant_the_device_claims_whenever_it_is_decided() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let at = |now: Timestamp| LearnerContext { now, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let (started, opening, seconds) = (1_000_000, 1_005_000, 78_000);
    for delivered in [started, started + 73_000, started + 900_000] {
        let mut state = registered(ALICE, 1);
        let events = step(&mut state, &command_start_at(ALICE, RUN, Challenge::Expert, started), &at(delivered));
        assert_eq!(events.iter().map(Event::at).collect::<Vec<_>>(), [started], "delivered at {delivered}");
        assert_eq!(step(&mut state, &open(RUN, "power", opening), &at(delivered)).iter().map(Event::at).collect::<Vec<_>>(), [opening]);
        let answer = solved(Challenge::Expert, "power");
        assert_eq!(decide_learner(&state, &record(RUN, "power", answer.clone(), opening + seconds + 1), &at(delivered)), Decision::Rejection(Rejection::TimeUp), "delivered at {delivered}: no extra time after sync");
        assert!(matches!(decide_learner(&state, &record(RUN, "power", answer, opening + seconds), &at(delivered)), Decision::Events(_)));
    }
    let mut state = registered(ALICE, 1);
    step(&mut state, &command_start_at(ALICE, RUN, Challenge::Expert, started), &at(started + 73_000));
    assert_eq!(step(&mut state, &open(RUN, "power", 0), &at(started + 73_000)).iter().map(Event::at).collect::<Vec<_>>(), [started], "an opening is raised to the device's start, not to the decision time");
    assert_eq!(decide_learner(&registered(ALICE, 1), &command_start(ALICE, RUN, MAX_TIMESTAMP + 1), &at(1)), Decision::Rejection(Rejection::IdInvalid));
    let mut ahead = registered(ALICE, 1);
    assert_eq!(step(&mut ahead, &command_start_at(ALICE, RUN, Challenge::Expert, started + 3_600_000), &at(started)).iter().map(Event::at).collect::<Vec<_>>(), [started + CLOCK_LEAD], "a start dated an hour ahead starts at the lead");
    assert_eq!(step(&mut ahead, &open(RUN, "power", started + 3_600_000), &at(started + 1_000)).iter().map(Event::at).collect::<Vec<_>>(), [started + 1_000 + CLOCK_LEAD], "so does an opening");
    assert_eq!(decide_learner(&ahead, &record(RUN, "power", solved(Challenge::Expert, "power"), started + 3_600_000 + 1_000), &at(started + 1_800_000)), Decision::Rejection(Rejection::TimeUp), "half an hour later the claimed hour buys no time");
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
    assert_eq!(serde_json::to_string(&Decision::Events(vec![Event::TaskOpened { learner: ALICE.to_string(), run: RUN.to_string(), task: "power".to_string(), at: 7 }])).ok().as_deref(), Some(r#"{"events":[{"type":"task-opened","learner":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","run":"0123456789abcdef0123456789abcdef","task":"power","at":7}]}"#));
    for rejection in [Rejection::RunUntimed, Rejection::TaskUnopened, Rejection::TimeUp, Rejection::AlreadyOpened] {
        assert_eq!(serde_json::to_string(&Decision::Rejection(rejection)).ok(), Some(format!(r#"{{"rejection":"{}"}}"#, rejection.as_str())));
    }
}

#[test]
fn a_run_state_always_carries_its_challenge_and_its_opened_tasks() {
    let (catalog, quizzes) = (catalog(), quizzes(REVISION));
    let context = LearnerContext { now: 50, catalog: &catalog, quizzes: &quizzes, limits: &DEFAULT_LIMITS };
    let mut state = registered(ALICE, 4);
    step(&mut state, &command_start_at(ALICE, RUN, Challenge::Hard, context.now), &context);
    let stored = serde_json::to_value(&state.runs[0]).unwrap_or_default();
    assert_eq!(stored, serde_json::json!({"run": RUN, "quiz": "energy", "challenge": "hard", "revision": REVISION, "seed": run_seed(RUN), "status": "open", "answers": {}, "recorded": 0, "opened": {}, "startedAt": 50}));
    assert_eq!(serde_json::from_value::<RunState>(stored.clone()).ok().as_ref(), Some(&state.runs[0]));
    for member in ["challenge", "opened"] {
        let mut without = stored.clone();
        if let Some(object) = without.as_object_mut() {
            object.remove(member);
        }
        assert!(serde_json::from_value::<RunState>(without).is_err(), "{member} is required");
    }
    let mut expert = registered(ALICE, 4);
    play_perfect_at(&mut expert, RUN, Challenge::Expert, &quizzes, &catalog, 60);
    let stored = serde_json::to_value(&expert).unwrap_or_default();
    assert_eq!(stored["runs"][0]["opened"], serde_json::json!({"buildings": 60, "power": 60, "standards": 60}));
    assert_eq!((stored["runs"][0]["challenge"].as_str(), stored["runs"][0]["result"]["points"].as_f64()), (Some("expert"), Some(400.0)));
    assert_eq!(serde_json::from_value::<LearnerState>(stored).ok(), Some(expert));
}
