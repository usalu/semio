//! 🃏️ What a run is made of: the sheet drawn for a learner, the answers given to it and the results scored from them.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use super::{Category, Challenge, Icon, Quantity, Score, ShortText, Slug, TaskKind, Text};

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
