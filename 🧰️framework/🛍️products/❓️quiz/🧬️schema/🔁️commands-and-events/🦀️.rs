//! 🔁️ The learner lifecycle on the wire: identities and limits, commands, rejections and events.

use serde::{Deserialize, Serialize};
use super::{Answer, Challenge, Handle, Id, RunResult, Slug, Timestamp};

//#region 🔖️Lifecycle
/// 🎭️ How a learner appears. Anonymous learners are always new; pseudonyms and names share one
/// handle namespace and are recalled without a password. The handle is the normalized display.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Identity {
    Anonymous,
    Pseudonym { handle: Handle },
    Name { handle: Handle },
}

/// 🖋️ An identity as a learner asks for it: the handle as typed (at most 256 code points), which the
/// proctor normalizes or refuses. Same shape as [`Identity`]; only the handle's state differs.
pub type IdentityClaim = Identity;

/// 🧢️ The caps a proctor decides with, so that no learner and no client grows its state without bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Limits {
    pub learners: u64,
    pub runs_per_quiz: u64,
    pub runs: u64,
    pub answers_per_run: u64,
}

/// 🛟️ The caps of a proctor nobody configured: far beyond a real class (300 learners playing every
/// quiz dozens of times). The registrations are sized against what they cost a proctor to keep — a
/// few kilobytes of disk each — and against the pace one client address may register at, so that
/// the cap is weeks away from any one of them.
pub const DEFAULT_LIMITS: Limits = Limits { learners: 100_000, runs_per_quiz: 200, runs: 1_000, answers_per_run: 2_000 };

impl Default for Limits {
    /// 🧯️ [`DEFAULT_LIMITS`].
    fn default() -> Self {
        DEFAULT_LIMITS
    }
}

/// 📨️ Learner intent. Every command carries a client-generated id so a retry after a connection
/// shortage is applied exactly once. `identify-learner` registers a learner: an anonymous one in its
/// own stream, a pseudonym or name in the stream of its handle key. `start-run` names the challenge of
/// the run; at another challenge than the open run of the quiz it voids that run. `open-task` starts
/// the clock of a task of a timed run, once (`already-opened`). `start-run`, `open-task` and
/// `record-answer` carry `at`, the instant the learner acted by the device's clock: a decider lowers it
/// to five minutes (`CLOCK_LEAD`) past its own clock, so a claim dated further ahead buys no time; the
/// run starts at its `at`, and a decider raises the `at` of an opening or an answer to the run's start
/// (or the task's opening). Beyond that lead it never consults its own clock for a time verdict. An
/// `at` beyond [`MAX_TIMESTAMP`]
/// decodes but is refused as `id-invalid`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum Command {
    IdentifyLearner { id: Id, learner: Id, identity: IdentityClaim },
    StartRun { id: Id, learner: Id, run: Id, quiz: Slug, challenge: Challenge, at: Timestamp },
    OpenTask { id: Id, learner: Id, run: Id, task: Slug, at: Timestamp },
    RecordAnswer { id: Id, learner: Id, run: Id, task: Slug, answer: Answer, at: Timestamp },
    SubmitRun { id: Id, learner: Id, run: Id },
}

impl Command {
    /// 🔑️ The client-generated command id, also the idempotency key.
    pub fn id(&self) -> &Id {
        match self {
            Self::IdentifyLearner { id, .. } | Self::StartRun { id, .. } | Self::OpenTask { id, .. } | Self::RecordAnswer { id, .. } | Self::SubmitRun { id, .. } => id,
        }
    }

    /// 🧑‍🎓️ The learner the command is addressed to (for `identify-learner`, the id a new learner receives).
    pub fn learner(&self) -> &Id {
        match self {
            Self::IdentifyLearner { learner, .. } | Self::StartRun { learner, .. } | Self::OpenTask { learner, .. } | Self::RecordAnswer { learner, .. } | Self::SubmitRun { learner, .. } => learner,
        }
    }

    /// 🔤️ The wire `type` tag, e.g. `start-run`.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::IdentifyLearner { .. } => "identify-learner",
            Self::StartRun { .. } => "start-run",
            Self::OpenTask { .. } => "open-task",
            Self::RecordAnswer { .. } => "record-answer",
            Self::SubmitRun { .. } => "submit-run",
        }
    }
}

/// 🚫️ Why a command was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Rejection {
    UnknownLearner,
    UnknownQuiz,
    UnknownRun,
    UnknownTask,
    RunOpen,
    RunClosed,
    RunIncomplete,
    AnswerInvalid,
    QuizRevised,
    HandleInvalid,
    HandleClaimed,
    IdInvalid,
    LearnerExists,
    RosterFull,
    RunsExhausted,
    AnswersExhausted,
    RunUntimed,
    TaskUnopened,
    TimeUp,
    AlreadyOpened,
}

impl Rejection {
    /// 🔡️ The wire string, e.g. `run-open`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnknownLearner => "unknown-learner",
            Self::UnknownQuiz => "unknown-quiz",
            Self::UnknownRun => "unknown-run",
            Self::UnknownTask => "unknown-task",
            Self::RunOpen => "run-open",
            Self::RunClosed => "run-closed",
            Self::RunIncomplete => "run-incomplete",
            Self::AnswerInvalid => "answer-invalid",
            Self::QuizRevised => "quiz-revised",
            Self::HandleInvalid => "handle-invalid",
            Self::HandleClaimed => "handle-claimed",
            Self::IdInvalid => "id-invalid",
            Self::LearnerExists => "learner-exists",
            Self::RosterFull => "roster-full",
            Self::RunsExhausted => "runs-exhausted",
            Self::AnswersExhausted => "answers-exhausted",
            Self::RunUntimed => "run-untimed",
            Self::TaskUnopened => "task-unopened",
            Self::TimeUp => "time-up",
            Self::AlreadyOpened => "already-opened",
        }
    }
}

/// 📰️ Facts of the learner stream. The quiz revision is the content hash the run was started against;
/// a run is voided because its quiz was revised or the learner started the quiz at another challenge;
/// the clock of a task of a timed run runs from its `task-opened`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum Event {
    LearnerRegistered { learner: Id, identity: Identity, at: Timestamp },
    RunStarted { learner: Id, run: Id, quiz: Slug, challenge: Challenge, revision: String, seed: u32, at: Timestamp },
    RunVoided { learner: Id, run: Id, at: Timestamp },
    TaskOpened { learner: Id, run: Id, task: Slug, at: Timestamp },
    AnswerRecorded { learner: Id, run: Id, task: Slug, answer: Answer, at: Timestamp },
    RunSubmitted { learner: Id, run: Id, result: RunResult, at: Timestamp },
    BadgeAwarded { learner: Id, badge: Slug, run: Id, at: Timestamp },
}

impl Event {
    /// 👤️ The learner whose stream the fact belongs to.
    pub fn learner(&self) -> &Id {
        match self {
            Self::LearnerRegistered { learner, .. }
            | Self::RunStarted { learner, .. }
            | Self::RunVoided { learner, .. }
            | Self::TaskOpened { learner, .. }
            | Self::AnswerRecorded { learner, .. }
            | Self::RunSubmitted { learner, .. }
            | Self::BadgeAwarded { learner, .. } => learner,
        }
    }

    /// ⏲️ When the fact was decided.
    pub fn at(&self) -> Timestamp {
        match self {
            Self::LearnerRegistered { at, .. }
            | Self::RunStarted { at, .. }
            | Self::RunVoided { at, .. }
            | Self::TaskOpened { at, .. }
            | Self::AnswerRecorded { at, .. }
            | Self::RunSubmitted { at, .. }
            | Self::BadgeAwarded { at, .. } => *at,
        }
    }

    /// 🔠️ The wire `type` tag, e.g. `run-started`.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::LearnerRegistered { .. } => "learner-registered",
            Self::RunStarted { .. } => "run-started",
            Self::RunVoided { .. } => "run-voided",
            Self::TaskOpened { .. } => "task-opened",
            Self::AnswerRecorded { .. } => "answer-recorded",
            Self::RunSubmitted { .. } => "run-submitted",
            Self::BadgeAwarded { .. } => "badge-awarded",
        }
    }
}
//#endregion 🔖️Lifecycle
