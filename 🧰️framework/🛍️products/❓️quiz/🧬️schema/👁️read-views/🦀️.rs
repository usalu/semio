//! 👁️ The read-model views and queries: catalog, run and learner views, hints, leaderboards, handles and the crowd.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use super::{Answer, Challenge, Handle, Icon, Id, Identity, Introduction, RunResult, Score, Sheet, Slug, TaskKind, Text, Timestamp};

//#region 🔖️Views
/// 🗒️ A task as the catalog lists it: id, kind, title and icon only.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogTaskView {
    pub id: Slug,
    pub kind: TaskKind,
    pub title: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

/// 📘️ A quiz as the catalog lists it, without items or solutions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogQuizView {
    pub id: Slug,
    pub emoji: String,
    pub title: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<Text>,
    pub description: Text,
    pub tasks: Vec<CatalogTaskView>,
}

/// 🎖️ A badge as the catalog lists it, without its rule.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogBadgeView {
    pub id: Slug,
    pub emoji: String,
    pub label: Text,
    pub description: Text,
}

/// 🗺️ The solution-free catalog a client renders.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogView {
    pub id: Slug,
    pub title: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<Text>,
    pub introduction: Introduction,
    pub quizzes: Vec<CatalogQuizView>,
    pub badges: Vec<CatalogBadgeView>,
}

/// 🚦️ Where a run stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    Open,
    Submitted,
    Voided,
}

/// ⚖️ The key assigned to `item` misses its value; questions the relation the learner's keys claim
/// between it and `other` of the same task (and `dimension`, set for matching): the ratio
/// `factor = key(item) / key(other)` on a logarithmic scale, the `difference = key(item) − key(other)`
/// on a linear one (exactly one of them), and its [`Verdict`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompareHint {
    pub item: Slug,
    pub other: Slug,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub difference: Option<f64>,
    pub verdict: Verdict,
}

/// 🧑‍⚖️ On which side of a claimed relation the truth lies: `under` — further out in the same
/// direction ("only"); `over` — closer in, the same direction ("really"); `reversed` — the other
/// direction ("larger than"), told without a number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Verdict {
    Under,
    Over,
    Reversed,
}

/// 🚥️ Every [`Verdict`] in schema order (`VERDICTS` in the TypeScript twin).
pub const VERDICTS: [Verdict; 3] = [Verdict::Under, Verdict::Over, Verdict::Reversed];

/// 🔺️ An item assigned to `category` (not its own) whose profile lies farther from its own than the
/// reach of `axis`, the axis with the largest gap relative to its reach; questioned against `other`, an
/// item placed in its own category, where the placement claims the item lies on the `above` side of it
/// on that axis while its own profile lies on the other side, else against the assigned profile's value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileHint {
    pub item: Slug,
    pub category: Slug,
    pub axis: Slug,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub above: Option<bool>,
}

/// 👯️ An item in a category not its own, questioned against `other`: put in the same category
/// although their own categories differ (`together`), or apart although they share one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupHint {
    pub item: Slug,
    pub other: Slug,
    pub together: bool,
}

/// ❔️ An item assigned to `category`, not its own, with no other item to question it against.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CategoryHint {
    pub item: Slug,
    pub category: Slug,
}

/// 💡️ What an open run at a challenge that hints asks the learner about one item of the recorded
/// answer of a task, tagged by `kind`: never the truth, always a concrete relation the answer claims; a
/// pure function of the task, its sheet task and the answer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Hint {
    Compare(CompareHint),
    Profile(ProfileHint),
    Group(GroupHint),
    Category(CategoryHint),
}

/// 🌟️ The best submitted run of a quiz: the one with the most points (the earliest of equals), with
/// its challenge and its score.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Best {
    pub challenge: Challenge,
    pub score: Score,
    pub points: f64,
}

/// 📺️ One run as its learner sees it: the sheet, the recorded answers and, once submitted, the result;
/// on a timed run also when each opened task was opened, and while it is open at a challenge that
/// hints the hints per task (only tasks with hints, absent without any).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunView {
    pub run: Id,
    pub learner: Id,
    pub quiz: Slug,
    pub status: RunStatus,
    pub sheet: Sheet,
    pub answers: BTreeMap<Slug, Answer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<RunResult>,
    pub started_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opened: Option<BTreeMap<Slug, Timestamp>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hints: Option<BTreeMap<Slug, Vec<Hint>>>,
}

/// 🗞️ One run in a learner's history, with its score and points once submitted.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunSummary {
    pub run: Id,
    pub quiz: Slug,
    pub challenge: Challenge,
    pub status: RunStatus,
    pub started_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<Score>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<Timestamp>,
}

/// 🥇️ A badge a learner holds, the run that earned it and when.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BadgeAward {
    pub badge: Slug,
    pub run: Id,
    pub at: Timestamp,
}

/// 🙋️ One learner's identity, runs (newest first), badges, best run per quiz and total points.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearnerView {
    pub learner: Id,
    pub identity: Identity,
    pub runs: Vec<RunSummary>,
    pub badges: Vec<BadgeAward>,
    pub best: BTreeMap<Slug, Best>,
    pub total: f64,
}

/// 📈️ One public leaderboard row. It never carries the learner id — without passwords that id is the
/// learner's only credential — but its `tag`: FNV-1a of the id as 8 lowercase hex digits. `best` is
/// the best run per quiz by points whatever the challenge, `total` their points, `runs` counts the
/// submitted runs in the scope of the leaderboard, `last_activity` is the last of them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LeaderboardRow {
    pub rank: usize,
    pub tag: String,
    pub identity: Identity,
    pub total: f64,
    pub reached_at: Timestamp,
    pub best: BTreeMap<Slug, Best>,
    pub badges: Vec<Slug>,
    pub runs: usize,
    pub last_activity: Timestamp,
}

/// 🔝️ How many rows a [`Leaderboard`] carries at most.
pub const LEADERBOARD_TOP: usize = 100;

/// 🗓️ Which runs a leaderboard counts by when they were submitted: those of the current day, ISO week
/// (from Monday) or month — calendar periods in UTC around the proctor's clock — or all of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LeaderboardPeriod {
    Daily,
    Weekly,
    Monthly,
    AllTime,
}

/// 🗓️ Every [`LeaderboardPeriod`] in the order they are offered.
pub const LEADERBOARD_PERIODS: [LeaderboardPeriod; 4] = [LeaderboardPeriod::Daily, LeaderboardPeriod::Weekly, LeaderboardPeriod::Monthly, LeaderboardPeriod::AllTime];

/// 🪟️ The time a leaderboard period spans: a run counts when it was submitted at or after `from` and
/// before `until`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeaderboardWindow {
    pub from: Timestamp,
    pub until: Timestamp,
}

/// 🏵️ One leaderboard: the learners with at least one submitted run in its scope — the runs submitted
/// inside `window` (every run when there is none), of `quiz` only when it names one — ordered by total
/// descending, then badge count descending, then `reachedAt` ascending, then learner id ascending;
/// rank is the 1-based position. Every row is made of the runs in scope only. `rows` holds the top
/// [`LEADERBOARD_TOP`] only, `learners` counts every ranked learner, `submissions` every run submitted
/// in the catalog whatever the period and quiz, and `own` is the caller's row when the query names a
/// ranked learner — also when it is inside the top.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Leaderboard {
    pub period: LeaderboardPeriod,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quiz: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<LeaderboardWindow>,
    pub rows: Vec<LeaderboardRow>,
    pub learners: usize,
    pub submissions: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub own: Option<LeaderboardRow>,
}

/// 🔦️ Who holds a handle: the normalized display of the asked handle and, when it is claimed, its holder.
/// Recalling a handle is this read; it writes nothing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandleView {
    pub display: Handle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<HandleHolder>,
}

/// 🤝️ The learner holding a handle and the identity it registered.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandleHolder {
    pub learner: Id,
    pub identity: Identity,
}

/// 🔭️ The reads a proctor answers; `leaderboard` names its period, may name the one quiz it counts and
/// the caller, `handle` carries the handle as typed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum Query {
    Catalog,
    Learner {
        learner: Id,
    },
    Run {
        run: Id,
    },
    Leaderboard {
        period: LeaderboardPeriod,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        quiz: Option<Slug>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        learner: Option<Id>,
    },
    Crowd {
        quiz: Slug,
    },
    Handle {
        handle: String,
    },
}

impl Query {
    /// 🔣️ The wire `type` tag, e.g. `leaderboard`.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Catalog => "catalog",
            Self::Learner { .. } => "learner",
            Self::Run { .. } => "run",
            Self::Leaderboard { .. } => "leaderboard",
            Self::Crowd { .. } => "crowd",
            Self::Handle { .. } => "handle",
        }
    }
}

/// ➕️ How often one category or value was given for an item: a category id (classification) or a value in
/// JSON number syntax (matching).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrowdCount {
    pub key: String,
    pub count: usize,
}

/// 🙌️ How often an item was answered and how: counts in ascending key order, or for sortings the mean normalized
/// position (0 smallest … 1 largest) beside `places`, how often the learners put it at each place a sheet of the
/// task presents (sums to `answers`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CrowdItem {
    pub item: Slug,
    pub answers: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counts: Option<Vec<CrowdCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_position: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub places: Option<Vec<usize>>,
}

/// 🔟️ How many bins a [`CrowdScores`] has: `[0, 10)`, `[10, 20)`, … `[80, 90)`, `[90, 100]` in whole percent.
pub const CROWD_SCORE_BINS: usize = 10;

/// 📉️ How many scores fell into each of the [`CROWD_SCORE_BINS`] bins, lowest bin first.
pub type CrowdScores = [usize; CROWD_SCORE_BINS];

/// 🧶️ The crowd of one task (one per dimension for matching): the scores of the results that count for it (the
/// dimension's scores for a matching) and its answered items in definition order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrowdTask {
    pub task: Slug,
    pub kind: TaskKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Slug>,
    pub scores: CrowdScores,
    pub items: Vec<CrowdItem>,
}

/// 🌈️ What the learners answered and scored in the submitted runs of one quiz: the run scores (summing to `runs`),
/// then aggregated per task (and dimension) and item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrowdView {
    pub quiz: Slug,
    pub runs: usize,
    pub scores: CrowdScores,
    pub tasks: Vec<CrowdTask>,
}
//#endregion 🔖️Views
