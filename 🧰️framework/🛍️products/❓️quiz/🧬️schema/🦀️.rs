//! 🧬️ Hand-written Rust twin of the normative quiz contract `🧬️schema/🔣️.json` (JSON Schema draft-07).
//!
//! One type per `$defs` entry, same names; fields are `camelCase` on the wire, tagged unions are
//! internally tagged (`kind` for tasks, sheet tasks, answers, results, identities and badge rules;
//! `type` for commands, events and queries) with kebab-case variant names, id-keyed maps are
//! `BTreeMap`s, optional fields are omitted when absent and unknown fields are refused like the
//! schema's `additionalProperties: false`. Constraints serde cannot express (slug patterns, lengths,
//! cardinalities, references) are checked by [`crate::validation`].
//!
//! @see ../🧬️schema/🔣️.json — the normative contract
//! @see ../🧬️schema/🟦️.ts — the TypeScript twin
//! @see <https://json-schema.org/draft-07/json-schema-validation>

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
pub const WIRE_VERSION: WireVersion = 4;

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
    pub description: Text,
    pub tasks: Vec<Task>,
}
//#endregion 🔖️Quiz

//#region 🔖️Catalog
/// 📜️ When a badge is earned: `perfect-quiz` — one run of the quiz scored 1; `perfect-tasks` — every
/// catalog task matching the selector scored 1 in some run; `completed-quizzes` — every catalog quiz
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

/// 🎗️ A badge a learner can earn across the quizzes of a catalog.
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

/// 📚️ The quizzes one site offers, its introduction and the badges spanning its quizzes. Quiz paths
/// are relative to the catalog file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<String>,
    pub schema: String,
    pub id: Slug,
    pub title: Text,
    pub introduction: Introduction,
    pub quizzes: Vec<String>,
    pub badges: Vec<Badge>,
}
//#endregion 🔖️Catalog

//#region 🔖️Sheet
/// 🪪️ A solution-free item: only its id, labels and icon.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetItem {
    pub id: Slug,
    pub label: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<ShortText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

/// 🛞️ One spoke of a profile as a sheet presents it: `unit`, `min` and `max` travel together and are
/// present exactly when the challenge shows the keys; without them the profile values of the sheet's
/// categories are shares of the axis range, `(value − min) / (max − min)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetAxis {
    pub id: Slug,
    pub label: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short: Option<ShortText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

/// 🫙️ A classification task as presented: shuffled categories (without descriptions where the keys are
/// hidden), drawn items in presentation order, and its `seconds` on a timed sheet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetClassificationTask {
    pub id: Slug,
    pub title: Text,
    pub prompt: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub axes: Option<Vec<SheetAxis>>,
    pub categories: Vec<Category>,
    pub items: Vec<SheetItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u64>,
}

/// 🪜️ A sorting task as presented: drawn items in presentation order, never already ascending, their
/// true values as an ascending ladder of `keys` where the challenge shows them (place `i` of the
/// learner's order shows the `i`-th smallest), and its `seconds` on a timed sheet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetSortingTask {
    pub id: Slug,
    pub title: Text,
    pub prompt: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub quantity: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<f64>>,
    pub items: Vec<SheetItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u64>,
}

/// 🎟️ A dimension as presented: its value cards in presentation order, addressed by index, where the
/// challenge shows the keys.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetDimension {
    pub id: Slug,
    pub quantity: Quantity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<f64>>,
}

/// 🧵️ A matching task as presented: drawn items in presentation order, shuffled cards per dimension
/// where the challenge shows the keys, and its `seconds` on a timed sheet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetMatchingTask {
    pub id: Slug,
    pub title: Text,
    pub prompt: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub dimensions: Vec<SheetDimension>,
    pub items: Vec<SheetItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u64>,
}

/// 🎴️ One presented task, tagged by `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SheetTask {
    Classification(SheetClassificationTask),
    Sorting(SheetSortingTask),
    Matching(SheetMatchingTask),
}

impl SheetTask {
    /// 🔖️ The task id.
    pub fn id(&self) -> &Slug {
        match self {
            Self::Classification(SheetClassificationTask { id, .. }) | Self::Sorting(SheetSortingTask { id, .. }) | Self::Matching(SheetMatchingTask { id, .. }) => id,
        }
    }

    /// 🧫️ The task kind.
    pub fn kind(&self) -> TaskKind {
        match self {
            Self::Classification(_) => TaskKind::Classification,
            Self::Sorting(_) => TaskKind::Sorting,
            Self::Matching(_) => TaskKind::Matching,
        }
    }

    /// ⏱️ The time the task allows once it is opened; present exactly on a timed sheet.
    pub fn seconds(&self) -> Option<u64> {
        match self {
            Self::Classification(SheetClassificationTask { seconds, .. }) | Self::Sorting(SheetSortingTask { seconds, .. }) | Self::Matching(SheetMatchingTask { seconds, .. }) => *seconds,
        }
    }

    /// 🧮️ The presented items in presentation order.
    pub fn items(&self) -> &[SheetItem] {
        match self {
            Self::Classification(SheetClassificationTask { items, .. }) | Self::Sorting(SheetSortingTask { items, .. }) | Self::Matching(SheetMatchingTask { items, .. }) => items,
        }
    }
}

/// 📄️ The randomized presentation of a quiz for one run seed at one challenge — a pure function of
/// `(quiz, seed, challenge)` that deals the same items in the same order at every challenge. It never
/// says which item a number belongs to: where the challenge shows the keys a sorting carries its
/// ascending keys and a matching its cards, where it hides them the sheet carries none of those, no
/// category descriptions and no axis numbers, and a profile is only a share of its axis range.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sheet {
    pub quiz: Slug,
    pub seed: u32,
    pub challenge: Challenge,
    pub title: Text,
    pub description: Text,
    pub tasks: Vec<SheetTask>,
}
//#endregion 🔖️Sheet

//#region 🔖️Answer
/// 📥️ Category id per item id.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClassificationAnswer {
    pub assignments: BTreeMap<Slug, Slug>,
}

/// ↕️ Item ids, smallest first; where the sheet task hides the keys, the learner's numeric guess per
/// item id in the quantity's base unit is the answer: `guesses` is present only there, guessed items
/// stand in `order` in non-decreasing guess order and an item without a guess counts as a miss.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortingAnswer {
    pub order: Vec<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guesses: Option<BTreeMap<Slug, f64>>,
}

/// 🧲️ Per dimension id: the card index per item id where the sheet task shows the keys
/// (`assignments`), the guessed number per item id in the quantity's base unit where it hides them
/// (`guesses`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchingAnswer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignments: Option<BTreeMap<Slug, BTreeMap<Slug, usize>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guesses: Option<BTreeMap<Slug, BTreeMap<Slug, f64>>>,
}

/// ✍️ A learner's answer to one task, tagged by `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Answer {
    Classification(ClassificationAnswer),
    Sorting(SortingAnswer),
    Matching(MatchingAnswer),
}
//#endregion 🔖️Answer

//#region 🔖️Result
/// 🪙️ The credit of one classified item; `assigned` is absent when the learner left it unanswered
/// (timed runs only).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClassificationItemResult {
    pub item: Slug,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<Slug>,
    pub correct: Slug,
    pub credit: Score,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Text>,
}

/// 🪧️ One sorted item: its value, zero-based position in the learner's order and zero-based rank in
/// the true ascending order; where the sheet task hid the keys also the learner's `guess` (when there
/// is one) and whether the item is a `miss`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortingItemResult {
    pub item: Slug,
    pub value: f64,
    pub position: usize,
    pub rank: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guess: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miss: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Text>,
}

/// 🔍️ One matched item of one dimension: the assigned card value or the guess (absent when the learner
/// left the item unanswered), the correct value and, where the sheet task hid the keys, whether the
/// item is a `miss`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchingItemResult {
    pub item: Slug,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<f64>,
    pub correct: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miss: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Text>,
}

/// 📊️ The score of one matching dimension and its items in sheet order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionResult {
    pub dimension: Slug,
    pub score: Score,
    pub items: Vec<MatchingItemResult>,
}

/// 📋️ The scored task, tagged by `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum TaskResult {
    Classification { task: Slug, score: Score, items: Vec<ClassificationItemResult> },
    Sorting { task: Slug, score: Score, items: Vec<SortingItemResult> },
    Matching { task: Slug, score: Score, dimensions: Vec<DimensionResult> },
}

impl TaskResult {
    /// 🪢️ The scored task id.
    pub fn task(&self) -> &Slug {
        match self {
            Self::Classification { task, .. } | Self::Sorting { task, .. } | Self::Matching { task, .. } => task,
        }
    }

    /// 🥅️ The task score in `[0, 1]`.
    pub fn score(&self) -> Score {
        match self {
            Self::Classification { score, .. } | Self::Sorting { score, .. } | Self::Matching { score, .. } => *score,
        }
    }
}

/// 🏁️ The scored run: the quiz score is the mean of its task scores and means accuracy at every
/// challenge; `points = score × par` of the challenge; tasks follow the sheet order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunResult {
    pub quiz: Slug,
    pub challenge: Challenge,
    pub score: Score,
    pub points: f64,
    pub tasks: Vec<TaskResult>,
}
//#endregion 🔖️Result

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

//#region 🔖️Presence
/// 🖥️ Every page a learner can be on: `quiz` is the read-only page of one quiz, `learner` the learner's own
/// profile, `badges` every badge, `preferences` the settings; `run` and `results` belong to a run of a quiz.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Screen {
    Introduction,
    Identity,
    Home,
    Quiz,
    Run,
    Results,
    Leaderboard,
    Learner,
    Badges,
    Preferences,
}

/// 🎛️ Every screen in schema order (`SCREENS` in the TypeScript twin).
pub const SCREENS: [Screen; 10] = [Screen::Introduction, Screen::Identity, Screen::Home, Screen::Quiz, Screen::Run, Screen::Results, Screen::Leaderboard, Screen::Learner, Screen::Badges, Screen::Preferences];

/// 📍️ Where a learner is: the screen, for a run or its results the quiz, and for a run the task on screen.
/// Learners at the same place share one presence room.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Place {
    pub screen: Screen,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quiz: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<Slug>,
}

/// ⚓️ A landmark every learner at the same place renders (a card, the leaderboard, a task card), addressed by
/// a stable key `^[a-z0-9]+(?:[:-][a-z0-9]+)*$` of 1…64 characters.
pub type Anchor = String;

/// 🖱️ A pointer position relative (0…1) to the box of an anchor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cursor {
    pub anchor: Anchor,
    pub x: f64,
    pub y: f64,
}

/// 🟢️ Ephemeral shared presence in the catalog-wide room: who is online and where, by public tag only.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceState {
    pub tag: String,
    pub identity: Identity,
    pub place: Place,
    pub active: bool,
}

/// 👆️ Ephemeral shared pointer and keyboard focus in the room of one place; never names an answer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CursorState {
    pub tag: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus: Option<Anchor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag: Option<CursorDrag>,
}

/// ✊️ The item a learner is dragging.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CursorDrag {
    pub item: Slug,
}

/// 💭️ Ephemeral shared draft answers of one learner's open run per task id, published in the thinking room of
/// its quiz; carries the public tag only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThinkingState {
    pub tag: String,
    pub answers: BTreeMap<Slug, ThinkingAnswer>,
}

/// 🗨️ A draft answer as peers can read it, tagged by `kind`: classification and sorting answers are already
/// semantic; matching drafts carry values.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ThinkingAnswer {
    Classification(ClassificationAnswer),
    Sorting(SortingAnswer),
    Matching(ThinkingMatchingAnswer),
}

/// 🧿️ A matching draft in semantic form: per dimension id the value assigned to each item id (card indices
/// point into the publisher's own shuffled cards and mean nothing to peers).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThinkingMatchingAnswer {
    pub values: BTreeMap<Slug, BTreeMap<Slug, f64>>,
}
//#endregion 🔖️Presence

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
