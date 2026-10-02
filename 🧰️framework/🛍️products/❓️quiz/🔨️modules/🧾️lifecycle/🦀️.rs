//! 🧾️ The learner lifecycle as pure `decide`/`evolve` pairs (design §8), shared by both cores; the
//! proctor wraps them in framework deciders. One handle actor per handle key registers a pseudonym or
//! name exactly once; one learner actor per learner owns its registration (directly when anonymous),
//! runs, answers, submissions and badges. `at` is always the decision time.
//!
//! Every decision first holds the command to its id and slug shapes (`id-invalid`), so no malformed id
//! reaches an event, and to the caps of [`Limits`], so no stream grows without bound. A claimed handle
//! is recalled by a read, never by a command: recalling writes nothing.
//!
//! @see ../../🧬️schema/🔣️.json — `Command`, `Event`, `Rejection`, `Limits`
//! @see ../🧾️lifecycle/🟦️.ts — the TypeScript twin

use crate::badges::earned_badges;
use crate::randomness::run_seed;
use crate::schema::{Answer, BadgeAward, Catalog, Command, Event, Id, Identity, Limits, Quiz, Rejection, RunResult, RunStatus, Slug, Timestamp};
use crate::scoring::score_run;
use crate::sheet::sheet_of;
use crate::validation::{answer_complete, answer_rejection, command_rejection, normalize_handle};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// 🎬️ The outcome of one decision: the events to commit, or why the command is refused.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Decision {
    Events(Vec<Event>),
    Rejection(Rejection),
}

/// 📇️ One handle key and the learner holding it, once claimed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandleState {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<Id>,
}

/// 🌾️ A handle key nobody claimed yet.
pub fn empty_handle_state(key: &str) -> HandleState {
    HandleState { key: key.to_string(), holder: None }
}

/// 🚧️ `roster-full` once a proctor holds its cap of learners: the check a registration passes before
/// it is decided.
pub fn registration_rejection(learners: u64, limits: &Limits) -> Option<Rejection> {
    (learners >= limits.learners).then_some(Rejection::RosterFull)
}

/// 🛂️ Register the command's learner under a free handle; refuse a handle outside the policy or of
/// another key (`handle-invalid`), an anonymous identity (it has no handle), a claimed handle
/// (`handle-claimed`) and any other command (`id-invalid` when malformed, else `handle-invalid`).
pub fn decide_handle(state: &HandleState, command: &Command, now: Timestamp) -> Decision {
    if let Some(malformed) = command_rejection(command) {
        return Decision::Rejection(malformed);
    }
    let Command::IdentifyLearner { learner, identity: Identity::Pseudonym { handle } | Identity::Name { handle }, .. } = command else {
        return Decision::Rejection(Rejection::HandleInvalid);
    };
    let Some(normalized) = normalize_handle(handle).filter(|normalized| normalized.key == state.key) else {
        return Decision::Rejection(Rejection::HandleInvalid);
    };
    if state.holder.is_some() {
        return Decision::Rejection(Rejection::HandleClaimed);
    }
    let identity = match command {
        Command::IdentifyLearner { identity: Identity::Name { .. }, .. } => Identity::Name { handle: normalized.display },
        _ => Identity::Pseudonym { handle: normalized.display },
    };
    Decision::Events(vec![Event::LearnerRegistered { learner: learner.clone(), identity, at: now }])
}

/// 🗳️ Fold a handle event: the learner registered under a pseudonym or name holds the handle from
/// then on.
pub fn evolve_handle(state: &mut HandleState, event: &Event) {
    if let Event::LearnerRegistered { learner, identity: Identity::Pseudonym { .. } | Identity::Name { .. }, .. } = event {
        state.holder = Some(learner.clone());
    }
}

/// 🏃️ One run of a learner as the learner actor remembers it; `recorded` counts every answer
/// recorded, also the replaced ones.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunState {
    pub run: Id,
    pub quiz: Slug,
    pub revision: String,
    pub seed: u32,
    pub status: RunStatus,
    pub answers: BTreeMap<Slug, Answer>,
    pub recorded: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<RunResult>,
    pub started_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<Timestamp>,
}

/// 🎒️ The learner actor's state: identity once registered, runs in start order and badges in award
/// order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LearnerState {
    pub learner: Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    pub runs: Vec<RunState>,
    pub badges: Vec<BadgeAward>,
}

impl LearnerState {
    /// 🔦️ The run with id `run`.
    pub fn run(&self, run: &str) -> Option<&RunState> {
        self.runs.iter().find(|candidate| candidate.run == run)
    }

    fn run_mut(&mut self, run: &str) -> Option<&mut RunState> {
        self.runs.iter_mut().find(|candidate| candidate.run == run)
    }
}

/// 🫥️ The state of a learner nothing has happened to yet.
pub fn empty_learner_state(learner: &str) -> LearnerState {
    LearnerState { learner: learner.to_string(), identity: None, runs: Vec::new(), badges: Vec::new() }
}

/// 🚚️ A quiz as the proctor loaded it, with its revision (SHA-256 of the file bytes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadedQuiz {
    pub quiz: Quiz,
    pub revision: String,
}

/// 🌐️ What a learner decision may read besides the state: the decision time, the catalog, its quizzes
/// by id and the caps.
#[derive(Clone, Copy, Debug)]
pub struct LearnerContext<'a> {
    pub now: Timestamp,
    pub catalog: &'a Catalog,
    pub quizzes: &'a BTreeMap<Slug, LoadedQuiz>,
    pub limits: &'a Limits,
}

/// 🧑‍⚖️ Decide a command of this learner (design §8): the registration of an anonymous learner,
/// `start-run`, `record-answer` or `submit-run`. A command of malformed ids is `id-invalid`, a command
/// of another learner `unknown-learner`. Starting a run under an id the learner already used is
/// refused with `run-open` or `run-closed`. Events are addressed to `state.learner`.
pub fn decide_learner(state: &LearnerState, command: &Command, context: &LearnerContext<'_>) -> Decision {
    if let Some(malformed) = command_rejection(command) {
        return Decision::Rejection(malformed);
    }
    if *command.learner() != state.learner {
        return Decision::Rejection(Rejection::UnknownLearner);
    }
    let outcome = match command {
        Command::IdentifyLearner { identity, .. } => register(state, identity, context),
        Command::StartRun { run, quiz, .. } => start_run(state, run, quiz, context),
        Command::RecordAnswer { run, task, answer, .. } => record_answer(state, run, task, answer, context),
        Command::SubmitRun { run, .. } => submit_run(state, run, context),
    };
    outcome.map_or_else(Decision::Rejection, Decision::Events)
}

fn register(state: &LearnerState, identity: &Identity, context: &LearnerContext<'_>) -> Result<Vec<Event>, Rejection> {
    if *identity != Identity::Anonymous {
        return Err(Rejection::HandleInvalid);
    }
    if state.identity.is_some() {
        return Err(Rejection::LearnerExists);
    }
    Ok(vec![Event::LearnerRegistered { learner: state.learner.clone(), identity: Identity::Anonymous, at: context.now }])
}

fn start_run(state: &LearnerState, run: &Id, quiz: &Slug, context: &LearnerContext<'_>) -> Result<Vec<Event>, Rejection> {
    if state.identity.is_none() {
        return Err(Rejection::UnknownLearner);
    }
    let current = context.quizzes.get(quiz).ok_or(Rejection::UnknownQuiz)?;
    if let Some(existing) = state.run(run) {
        return Err(if existing.status == RunStatus::Open { Rejection::RunOpen } else { Rejection::RunClosed });
    }
    let open = state.runs.iter().find(|candidate| candidate.quiz == *quiz && candidate.status == RunStatus::Open);
    if open.is_some_and(|open| open.revision == current.revision) {
        return Err(Rejection::RunOpen);
    }
    let submitted = || state.runs.iter().filter(|candidate| candidate.status == RunStatus::Submitted);
    if submitted().count() as u64 >= context.limits.runs || submitted().filter(|candidate| candidate.quiz == *quiz).count() as u64 >= context.limits.runs_per_quiz {
        return Err(Rejection::RunsExhausted);
    }
    let started = Event::RunStarted { learner: state.learner.clone(), run: run.clone(), quiz: quiz.clone(), revision: current.revision.clone(), seed: run_seed(run), at: context.now };
    Ok(match open {
        Some(open) => vec![Event::RunVoided { learner: state.learner.clone(), run: open.run.clone(), at: context.now }, started],
        None => vec![started],
    })
}

fn open_run<'a>(state: &'a LearnerState, run: &Id) -> Result<&'a RunState, Rejection> {
    let found = state.run(run).ok_or(Rejection::UnknownRun)?;
    if found.status == RunStatus::Open {
        Ok(found)
    } else {
        Err(Rejection::RunClosed)
    }
}

fn current_quiz<'a>(found: &RunState, context: &LearnerContext<'a>) -> Option<&'a Quiz> {
    context.quizzes.get(&found.quiz).filter(|current| current.revision == found.revision).map(|current| &current.quiz)
}

fn record_answer(state: &LearnerState, run: &Id, task: &Slug, answer: &Answer, context: &LearnerContext<'_>) -> Result<Vec<Event>, Rejection> {
    let found = open_run(state, run)?;
    let quiz = current_quiz(found, context).ok_or(Rejection::QuizRevised)?;
    if found.recorded >= context.limits.answers_per_run {
        return Err(Rejection::AnswersExhausted);
    }
    let sheet = sheet_of(quiz, found.seed);
    let sheet_task = sheet.tasks.iter().find(|candidate| candidate.id() == task).ok_or(Rejection::UnknownTask)?;
    if let Some(rejection) = answer_rejection(sheet_task, answer) {
        return Err(rejection);
    }
    Ok(vec![Event::AnswerRecorded { learner: state.learner.clone(), run: run.clone(), task: task.clone(), answer: answer.clone(), at: context.now }])
}

fn submit_run(state: &LearnerState, run: &Id, context: &LearnerContext<'_>) -> Result<Vec<Event>, Rejection> {
    let found = open_run(state, run)?;
    let learner = &state.learner;
    let Some(quiz) = current_quiz(found, context) else {
        return Ok(vec![Event::RunVoided { learner: learner.clone(), run: run.clone(), at: context.now }]);
    };
    let sheet = sheet_of(quiz, found.seed);
    if !sheet.tasks.iter().all(|task| answer_complete(task, found.answers.get(task.id()))) {
        return Err(Rejection::RunIncomplete);
    }
    let result = score_run(quiz, &sheet, &found.answers).ok_or(Rejection::RunIncomplete)?;
    let results: Vec<&RunResult> = state.runs.iter().filter_map(|candidate| candidate.result.as_ref()).chain([&result]).collect();
    let held: BTreeSet<Slug> = state.badges.iter().map(|award| award.badge.clone()).collect();
    let quizzes: Vec<&Quiz> = context.quizzes.values().map(|current| &current.quiz).collect();
    let earned = earned_badges(&context.catalog.badges, &quizzes, &results, &held);
    let mut events = vec![Event::RunSubmitted { learner: learner.clone(), run: run.clone(), result, at: context.now }];
    events.extend(earned.into_iter().map(|badge| Event::BadgeAwarded { learner: learner.clone(), badge, run: run.clone(), at: context.now }));
    Ok(events)
}

/// 🔁️ Fold one fact of this learner into the state; facts of other learners are ignored.
pub fn evolve_learner(state: &mut LearnerState, event: &Event) {
    if *event.learner() != state.learner {
        return;
    }
    match event {
        Event::LearnerRegistered { identity, .. } => state.identity = Some(identity.clone()),
        Event::RunStarted { run, quiz, revision, seed, at, .. } => state.runs.push(RunState { run: run.clone(), quiz: quiz.clone(), revision: revision.clone(), seed: *seed, status: RunStatus::Open, answers: BTreeMap::new(), recorded: 0, result: None, started_at: *at, submitted_at: None }),
        Event::RunVoided { run, .. } => {
            if let Some(found) = state.run_mut(run) {
                found.status = RunStatus::Voided;
            }
        }
        Event::AnswerRecorded { run, task, answer, .. } => {
            if let Some(found) = state.run_mut(run) {
                found.answers.insert(task.clone(), answer.clone());
                found.recorded += 1;
            }
        }
        Event::RunSubmitted { run, result, at, .. } => {
            if let Some(found) = state.run_mut(run) {
                found.status = RunStatus::Submitted;
                found.result = Some(result.clone());
                found.submitted_at = Some(*at);
            }
        }
        Event::BadgeAwarded { badge, run, at, .. } => state.badges.push(BadgeAward { badge: badge.clone(), run: run.clone(), at: *at }),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
