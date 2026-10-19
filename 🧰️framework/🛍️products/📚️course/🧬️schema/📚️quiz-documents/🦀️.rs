//! 📚️ The authored quiz documents: scalars and limits, quizzes with their tasks, and the course with its badges.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

//#region 🔖️Scalars
/// 🐌️ Kebab-case identifier `^[a-z0-9]+(?:-[a-z0-9]+)*$`, 1…64 characters, unique per scope.
pub type Slug = String;

/// 🆔️ 32 lowercase hex characters (128 random bits) naming a learner, run or command.
pub type Id = String;

/// ✒️ A handle as registered and shown: 1…64 code points of Latin letters, ASCII digits, `'` `.` `_`
/// `-` and single spaces between words, with at least one letter or digit; always in NFC.
pub type Handle = String;

/// 🕰️ Milliseconds since the Unix epoch, a safe integer (at most [`MAX_TIMESTAMP`]).
pub type Timestamp = u64;

/// 🌌️ The latest timestamp: 2^53 − 1, the largest integer every language reads exactly.
pub const MAX_TIMESTAMP: Timestamp = (1 << 53) - 1;

/// 🤝️ The version of this contract on the wire: the `version` of every quiz envelope and of every quiz
/// kind a proctor declares; raised with every change of the contract beyond its prose.
pub type WireVersion = u32;

/// 🤝️ The wire version of this contract (`$defs/WireVersion`).
pub const WIRE_VERSION: WireVersion = 8;

/// 💯️ A score or credit in `[0, 1]`.
pub type Score = f64;

/// 🕸️ Values of a category on every axis of its task, keyed by axis id.
pub type Profile = BTreeMap<Slug, f64>;

/// 🌍️ A learner-visible text in every supported language; there is no default language.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    pub en: String,
    pub de: String,
}

/// ✂️ A short form of a label in every supported language, at most [`SHORT_LENGTH`] code points each,
/// that hints name the labelled thing by instead of its label.
pub type ShortText = Text;

/// 🤏️ The most code points a [`ShortText`] holds in each language.
pub const SHORT_LENGTH: usize = 40;

/// 📐️ How distances between values are measured: logarithmic for quantities spanning orders of
/// magnitude, linear otherwise.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Scale {
    Linear,
    Logarithmic,
}

/// 🗃️ The three task kinds a quiz can hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskKind {
    Classification,
    Sorting,
    Matching,
}

/// 🧗️ How demanding a run is, chosen when it starts and fixed for it; each challenge is the one before
/// plus one step: easy shows the keys and hints, medium shows the keys, hard hides them, expert hides
/// them and runs every task against a clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Challenge {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// 🎚️ Every [`Challenge`] in schema order (`CHALLENGES` in the TypeScript twin).
pub const CHALLENGES: [Challenge; 4] = [Challenge::Easy, Challenge::Medium, Challenge::Hard, Challenge::Expert];
//#endregion 🔖️Scalars

//#region 🔖️Quiz
/// 🌡️ A measured quantity: label, unit symbol, distance scale, whether display scales the unit
/// with SI prefixes (W → kW → MW …) and whether amounts of it add up (powers, energies), which lets a
/// hint say how many of one item together make another; `short` names it in a hint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quantity {
    pub label: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<ShortText>,
    pub unit: String,
    pub scale: Scale,
    pub prefixed: bool,
    pub additive: bool,
}

/// ↔️ One quantity a matching task asks for per item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dimension {
    pub id: Slug,
    pub quantity: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

/// 🧭️ One spoke of a profile (spider diagram); values normalise to `(value − min) / (max − min)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Axis {
    pub id: Slug,
    pub label: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<ShortText>,
    pub unit: String,
    pub min: f64,
    pub max: f64,
}

/// 🎞️ The looping microanimation of an icon, still for learners who prefer reduced motion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Motion {
    Bounce,
    Pulse,
    Spin,
    Sway,
    Float,
    Flip,
}

/// 🖼️ The icon of a task, an item, a category or a dimension (shown on its value cards): one emoji
/// grapheme picturing it and the [`Motion`] it plays.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Icon {
    pub emoji: String,
    pub motion: Motion,
}

/// 🗂️ A category of a classification task, optionally with a profile on the task axes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Category {
    pub id: Slug,
    pub label: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<ShortText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
}

/// 🏷️ An item of a classification task and its correct category.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClassificationItem {
    pub id: Slug,
    pub label: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<ShortText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub category: Slug,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Text>,
}

/// 🔢️ An item of a sorting task and its true value; `familiar` marks an everyday thing a learner can
/// picture, which a hint prefers as its reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortingItem {
    pub id: Slug,
    pub label: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<ShortText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub value: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub familiar: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Text>,
}

/// 🧷️ An item of a matching task and its true value per dimension id; `familiar` as on a
/// [`SortingItem`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchingItem {
    pub id: Slug,
    pub label: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<ShortText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub values: BTreeMap<Slug, f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub familiar: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Text>,
}

/// 🧺️ Fixed semantic slots with rotating variants and a per-question time budget.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionPool {
    pub seconds: u32,
    pub slots: Vec<Vec<Slug>>,
}

/// 🧺️ Assign every item to one category; categories may carry profiles on shared axes and a wrong
/// profile earns partial credit by profile similarity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClassificationTask {
    pub id: Slug,
    pub title: Text,
    pub prompt: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub axes: Option<Vec<Axis>>,
    pub categories: Vec<Category>,
    pub items: Vec<ClassificationItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draw: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<QuestionPool>,
}

/// 📶️ Order the items ascending by their quantity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SortingTask {
    pub id: Slug,
    pub title: Text,
    pub prompt: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub quantity: Quantity,
    pub items: Vec<SortingItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draw: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<QuestionPool>,
}

/// 🔗️ For every dimension, assign each item one of the offered value cards (the multiset of the
/// drawn items' true values).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MatchingTask {
    pub id: Slug,
    pub title: Text,
    pub prompt: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub dimensions: Vec<Dimension>,
    pub items: Vec<MatchingItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draw: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<QuestionPool>,
}

/// 🧱️ One task of a quiz, tagged by `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Task {
    Classification(ClassificationTask),
    Sorting(SortingTask),
    Matching(MatchingTask),
}

impl Task {
    /// 🎠️ The fixed semantic slots and time budget, when authored.
    pub fn pool(&self) -> Option<&QuestionPool> {
        match self {
            Self::Classification(task) => task.pool.as_ref(),
            Self::Sorting(task) => task.pool.as_ref(),
            Self::Matching(task) => task.pool.as_ref(),
        }
    }

    /// 🆎️ The task id.
    pub fn id(&self) -> &Slug {
        match self {
            Self::Classification(ClassificationTask { id, .. }) | Self::Sorting(SortingTask { id, .. }) | Self::Matching(MatchingTask { id, .. }) => id,
        }
    }

    /// 🪄️ The task kind.
    pub fn kind(&self) -> TaskKind {
        match self {
            Self::Classification(_) => TaskKind::Classification,
            Self::Sorting(_) => TaskKind::Sorting,
            Self::Matching(_) => TaskKind::Matching,
        }
    }

    /// 📛️ The task title.
    pub fn title(&self) -> &Text {
        match self {
            Self::Classification(ClassificationTask { title, .. }) | Self::Sorting(SortingTask { title, .. }) | Self::Matching(MatchingTask { title, .. }) => title,
        }
    }

    /// 🖼️ The task icon, when the task has one.
    pub fn icon(&self) -> Option<&Icon> {
        match self {
            Self::Classification(ClassificationTask { icon, .. }) | Self::Sorting(SortingTask { icon, .. }) | Self::Matching(MatchingTask { icon, .. }) => icon.as_ref(),
        }
    }
}

/// 🎓️ A quiz: an ordered set of tasks. A run keeps the first task first, randomizes the rest and is scored only as a whole.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Quiz {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<String>,
    pub schema: String,
    pub id: Slug,
    pub emoji: String,
    pub title: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<Text>,
    pub description: Text,
    pub tasks: Vec<Task>,
}
//#endregion 🔖️Quiz

//#region 🔖️Course
/// 📜️ When a badge is earned: `perfect-quiz` — one run of the quiz scored 1; `perfect-tasks` — every
/// course task matching the selector scored 1 in some run; `completed-quizzes` — every course quiz
/// has a submitted run. `challenge` is the least challenge whose runs count; every challenge counts
/// without it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum BadgeRule {
    PerfectQuiz {
        quiz: Slug,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        challenge: Option<Challenge>,
    },
    PerfectTasks {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        task_kind: Option<TaskKind>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        quiz: Option<Slug>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        challenge: Option<Challenge>,
    },
    CompletedQuizzes {},
}

/// 🎗️ A badge a learner can earn across the quizzes of a course.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Badge {
    pub id: Slug,
    pub emoji: String,
    pub label: Text,
    pub description: Text,
    pub rule: BadgeRule,
}

/// 👋️ The onboarding text a site shows on the first visit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Introduction {
    pub title: Text,
    pub paragraphs: Vec<Text>,
}

/// 🧭️ Page names reserved for the course's root navigation.
pub const COURSE_ROOT_IDS: [&str; 11] = ["home", "identity", "introduction", "learner", "intro", "board", "badges", "prefs", "quizzes", "run", "results"];

/// 🗂️ An ordered course group; an empty quiz list marks a placeholder.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Topic {
    pub id: Slug,
    pub emoji: String,
    pub title: Text,
    pub quizzes: Vec<Slug>,
}

/// 📚️ The quizzes one site offers, its introduction and the badges spanning its quizzes. Quiz paths
/// are relative to the course file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Course {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<String>,
    pub schema: String,
    pub id: Slug,
    pub title: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<Text>,
    pub introduction: Introduction,
    pub quizzes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Topic>>,
    pub badges: Vec<Badge>,
}
//#endregion 🔖️Course
