//! 👥️ The presence wire: screens, places, cursors, drags and the thinking states shared live.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use super::{ClassificationAnswer, Identity, Slug, SortingAnswer};

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
