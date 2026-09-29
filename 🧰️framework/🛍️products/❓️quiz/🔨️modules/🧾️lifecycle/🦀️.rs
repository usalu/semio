//! 🧾️ The learner lifecycle as pure `decide`/`evolve` pairs (design §8), shared by both cores; the
//! proctor wraps them in framework deciders. The roster actor owns the handle index and identifies
//! learners; one learner actor per learner owns its runs, answers, submissions and badges. `at` is
//! always the decision time.
//!
//! @see ../../🧬️schema/🔣️.json — `Command`, `Event`, `Rejection`
//! @see ../🧾️lifecycle/🟦️.ts — the TypeScript twin

use crate::badges::earned_badges;
use crate::randomness::run_seed;
use crate::schema::{Answer, BadgeAward, Catalog, Command, Event, Id, Identity, Quiz, Rejection, RunResult, RunStatus, Slug, Timestamp};
use crate::scoring::score_run;
use crate::sheet::sheet_of;
use crate::validation::{answer_complete, answer_rejection};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// 🪞️ A handle as displayed (trimmed, inner whitespace runs collapsed to one space) and as keyed
/// (the display lowercased); pseudonyms and names share one key space.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedHandle {
    pub display: String,
    pub key: String,
}

/// 🧽️ Normalize a handle; `None` unless the display has 1…64 characters (code points). Whitespace is
/// the Unicode `White_Space` property (`\p{White_Space}` in the TypeScript twin).
pub fn normalize_handle(handle: &str) -> Option<NormalizedHandle> {
    let display = handle.split(char::is_whitespace).filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" ");
    (1..=64).contains(&display.chars().count()).then(|| NormalizedHandle { key: display.to_lowercase(), display })
}

/// 🎬️ The outcome of one decision: the events to commit, or why the command is refused.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Decision {
    Events(Vec<Event>),
    Rejection(Rejection),
}

/// 📇️ The roster actor's state: the learner holding each handle key.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterState {
    pub handles: BTreeMap<String, Id>,
}

/// 🛂️ Identify a learner: anonymous learners are always registered anew, a free handle registers the
/// command's learner under the display handle, a claimed handle recalls its learner. Any other command
/// decides nothing.
pub fn decide_roster(state: &RosterState, command: &Command, now: Timestamp) -> Decision {
    let Command::IdentifyLearner { learner, identity, .. } = command else {
        return Decision::Events(Vec::new());
    };
    let (Identity::Pseudonym { handle } | Identity::Name { handle }) = identity else {
        return Decision::Events(vec![Event::LearnerRegistered { learner: learner.clone(), identity: Identity::Anonymous, at: now }]);
    };
    let Some(normalized) = normalize_handle(handle) else {
        return Decision::Rejection(Rejection::HandleInvalid);
    };
    Decision::Events(vec![match state.handles.get(&normalized.key) {
        Some(claimed) => Event::LearnerRecalled { learner: claimed.clone(), at: now },
        None => Event::LearnerRegistered {
            learner: learner.clone(),
            identity: match identity {
                Identity::Name { .. } => Identity::Name { handle: normalized.display },
                _ => Identity::Pseudonym { handle: normalized.display },
            },
            at: now,
        },
    }])
}

/// 🌾️ The roster before any learner registered.
pub fn empty_roster_state() -> RosterState {
    RosterState::default()
}

/// 🗳️ Fold a roster event: a learner registered under a pseudonym or name claims its handle key.
pub fn evolve_roster(state: &mut RosterState, event: &Event) {
    if let Event::LearnerRegistered { learner, identity: Identity::Pseudonym { handle } | Identity::Name { handle }, .. } = event {
        if let Some(normalized) = normalize_handle(handle) {
            state.handles.insert(normalized.key, learner.clone());
        }
    }
}

/// 🏃️ One run of a learner as the learner actor remembers it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunState {
    pub run: Id,
    pub quiz: Slug,
    pub revision: String,
    pub seed: u32,
    pub status: RunStatus,
    pub answers: BTreeMap<Slug, Answer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<RunResult>,
    pub started_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<Timestamp>,
}

/// 🎒️ The learner actor's state: identity once registered, runs in start order, badges in award
/// order and the time of the latest fact.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LearnerState {
    pub learner: Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    pub runs: Vec<RunState>,
    pub badges: Vec<BadgeAward>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<Timestamp>,
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
    LearnerState { learner: learner.to_string(), identity: None, runs: Vec::new(), badges: Vec::new(), last_activity: None }
}

/// 🚚️ A quiz as the proctor loaded it, with its revision (SHA-256 of the file bytes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadedQuiz {
    pub quiz: Quiz,
    pub revision: String,
}

/// 🌐️ What a learner decision may read besides the state: the decision time, the catalog and its
/// quizzes by id.
#[derive(Clone, Copy, Debug)]
pub struct LearnerContext<'a> {
    pub now: Timestamp,
    pub catalog: &'a Catalog,
    pub quizzes: &'a BTreeMap<Slug, LoadedQuiz>,
}

/// 🧑‍⚖️ Decide `start-run`, `record-answer` and `submit-run` for the learner (design §8); any other
/// command decides nothing. Starting a run under an id the learner already used is refused with
/// `run-open` or `run-closed`. Events are addressed to `state.learner`.
pub fn decide_learner(state: &LearnerState, command: &Command, context: &LearnerContext<'_>) -> Decision {
    let outcome = match command {
        Command::IdentifyLearner { .. } => Ok(Vec::new()),
        Command::StartRun { learner, run, quiz, .. } => start_run(state, learner, run, quiz, context),
        Command::RecordAnswer { run, task, answer, .. } => record_answer(state, run, task, answer, context),
        Command::SubmitRun { run, .. } => submit_run(state, run, context),
    };
    outcome.map_or_else(Decision::Rejection, Decision::Events)
}

fn start_run(state: &LearnerState, learner: &Id, run: &Id, quiz: &Slug, context: &LearnerContext<'_>) -> Result<Vec<Event>, Rejection> {
    if state.identity.is_none() || *learner != state.learner {
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
    state.last_activity = Some(state.last_activity.map_or(event.at(), |last| last.max(event.at())));
    match event {
        Event::LearnerRegistered { identity, .. } => state.identity = Some(identity.clone()),
        Event::LearnerRecalled { .. } => {}
        Event::RunStarted { run, quiz, revision, seed, at, .. } => state.runs.push(RunState { run: run.clone(), quiz: quiz.clone(), revision: revision.clone(), seed: *seed, status: RunStatus::Open, answers: BTreeMap::new(), result: None, started_at: *at, submitted_at: None }),
        Event::RunVoided { run, .. } => {
            if let Some(found) = state.run_mut(run) {
                found.status = RunStatus::Voided;
            }
        }
        Event::AnswerRecorded { run, task, answer, .. } => {
            if let Some(found) = state.run_mut(run) {
                found.answers.insert(task.clone(), answer.clone());
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
