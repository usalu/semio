//! 🧾️ The shared learner-lifecycle fixtures: identities, course, perfect answers, replay of the shared vectors.

use crate::sheet::{quiz, text};
use course::lifecycle::*;
use course::randomness::run_seed;
use course::sheet::sheet_of;
use course_schema::{Answer, Badge, BadgeRule, Course, Challenge, ClassificationAnswer, Command, Event, Id, Identity, Introduction, Limits, MatchingAnswer, Quiz, SheetTask, Slug, SortingAnswer, Task, TaskKind, Timestamp, DEFAULT_LIMITS};
use std::collections::{BTreeMap, BTreeSet};

pub const ALICE: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
pub const BOB: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
pub const RUN: &str = "0123456789abcdef0123456789abcdef";
pub const REVISION: &str = "1111111111111111111111111111111111111111111111111111111111111111";
pub const REVISED: &str = "2222222222222222222222222222222222222222222222222222222222222222";

pub fn course() -> Course {
    let badge = |id: &str, rule: BadgeRule| Badge { id: id.to_string(), emoji: "🏅".to_string(), label: text(id), description: text(id), rule };
    Course {
        assignments: None,
        unlocks: None,
        json_schema: None,
        schema: "semio.course/v1".to_string(),
        id: "architecture".to_string(),
        title: text("Architecture"),
        short: None,
        introduction: Introduction { title: text("Welcome"), paragraphs: vec![text("Hello")] },
        quizzes: vec!["energy/🔣️.json".to_string()],
        topics: None,
        badges: vec![badge("perfect-energy", BadgeRule::PerfectQuiz { quiz: "energy".to_string(), challenge: None }), badge("sorter", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None, challenge: None }), badge("done", BadgeRule::CompletedQuizzes {})],
    }
}

pub fn quizzes(revision: &str) -> BTreeMap<Slug, LoadedQuiz> {
    BTreeMap::from([("energy".to_string(), LoadedQuiz { quiz: quiz(), revision: revision.to_string() })])
}

pub fn id(seed: u8) -> Id {
    format!("{seed:032x}")
}

pub fn fold(state: &mut LearnerState, decision: &Decision) -> Vec<Event> {
    let Decision::Events(events) = decision else { return Vec::new() };
    for event in events {
        evolve_learner(state, event);
    }
    events.clone()
}

pub fn step(state: &mut LearnerState, command: &Command, context: &LearnerContext<'_>) -> Vec<Event> {
    let decision = decide_learner(state, command, context);
    fold(state, &decision)
}

pub fn registered(learner: &str, at: Timestamp) -> LearnerState {
    let mut state = empty_learner_state(learner);
    evolve_learner(&mut state, &Event::LearnerRegistered { learner: learner.to_string(), identity: Identity::Pseudonym { handle: learner[..4].to_string() }, at });
    state
}

pub fn perfect(quiz: &Quiz, sheet_task: &SheetTask) -> Answer {
    let task = quiz.tasks.iter().find(|task| task.id() == sheet_task.id());
    match (task, sheet_task) {
        (Some(Task::Classification(task)), SheetTask::Classification(sheet)) => Answer::Classification(ClassificationAnswer { assignments: sheet.items.iter().filter_map(|item| task.items.iter().find(|candidate| candidate.id == item.id).map(|candidate| (item.id.clone(), candidate.category.clone()))).collect() }),
        (Some(Task::Sorting(task)), SheetTask::Sorting(sheet)) => {
            let mut order: Vec<(usize, &course_schema::SortingItem)> = task.items.iter().enumerate().filter(|(_, item)| sheet.items.iter().any(|presented| presented.id == item.id)).collect();
            order.sort_by(|a, b| a.1.value.total_cmp(&b.1.value).then(a.0.cmp(&b.0)));
            Answer::Sorting(SortingAnswer { order: order.iter().map(|(_, item)| item.id.clone()).collect(), guesses: sheet.keys.is_none().then(|| order.iter().map(|(_, item)| (item.id.clone(), item.value)).collect()) })
        }
        (Some(Task::Matching(task)), SheetTask::Matching(sheet)) => {
            let value = |item: &course_schema::SheetItem, dimension: &str| task.items.iter().find(|candidate| candidate.id == item.id)?.values.get(dimension).copied();
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

pub fn command_start_at(learner: &str, run: &str, challenge: Challenge, at: Timestamp) -> Command {
    Command::StartRun { id: id(1), learner: learner.to_string(), origin: None, run: run.to_string(), quiz: "energy".to_string(), challenge, at }
}

pub fn command_start(learner: &str, run: &str, at: Timestamp) -> Command {
    command_start_at(learner, run, Challenge::Medium, at)
}

pub fn play_perfect_at(state: &mut LearnerState, run: &str, challenge: Challenge, quizzes: &BTreeMap<Slug, LoadedQuiz>, course: &Course, now: Timestamp) -> Vec<Event> {
    let context = LearnerContext { now, course, quizzes, limits: &DEFAULT_LIMITS };
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

pub fn play_perfect(state: &mut LearnerState, run: &str, quizzes: &BTreeMap<Slug, LoadedQuiz>, course: &Course, now: Timestamp) -> Vec<Event> {
    play_perfect_at(state, run, Challenge::Medium, quizzes, course, now)
}

pub fn replay(vectors: &serde_json::Value, sequence: &serde_json::Value) -> (Vec<Decision>, LearnerState, BTreeMap<Slug, LoadedQuiz>) {
    use course_schema::testkit::{entries, typed};
    let course: Course = typed(&vectors["course"]);
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
        let context = LearnerContext { now: step["now"].as_u64().unwrap_or_default(), course: &course, quizzes: &quizzes, limits: &limits };
        let decision = decide_learner(&state, &typed(&step["command"]), &context);
        fold(&mut state, &decision);
        decisions.push(decision);
    }
    let quizzes = loaded(&revisions);
    (decisions, state, quizzes)
}
