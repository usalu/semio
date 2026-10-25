//! 👥️ The presence wire: screens, places, cursors, drags and the thinking states shared live.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use super::{Answer, Best, Challenge, ClassificationAnswer, CrowdView, Hint, Identity, Leaderboard, LeaderboardPeriod, RunResult, RunStatus, Score, Sheet, Slug, SortingAnswer, Timestamp};

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

/// 🟢️ Ephemeral shared presence in the course-wide room: who is online and where, by public tag only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceState {
    pub tag: String,
    pub identity: Identity,
    pub place: Place,
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub view: Option<PresencePresentation>,
}

/// 🪞️ The public run presentation, preserving randomized sheet and answers while omitting private ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresenceRun {
    pub quiz: Slug,
    pub status: RunStatus,
    pub sheet: Sheet,
    pub answers: BTreeMap<Slug, Answer>,
    pub started_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub result: Option<RunResult>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub submitted_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub opened: Option<BTreeMap<Slug, Timestamp>>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub hints: Option<BTreeMap<Slug, Vec<Hint>>>,
}

/// 📐️ A normalized scroll or camera position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresencePoint { pub x: f64, pub y: f64 }

/// 🎨️ Public display choices affecting the followed presentation, admitted against the normative enums.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresencePreferences {
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub theme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub text_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub show_cursors: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub others: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub animate_icons: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub icons_chosen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub pets: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub pets_chosen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub pets_liveliness: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub pets_play: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub pets_mischief: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub challenges: Option<BTreeMap<Slug, Challenge>>,
}

/// 🏃️ A public history entry without its private run link.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresenceRunSummary {
    pub quiz: Slug,
    pub challenge: Challenge,
    pub status: RunStatus,
    pub started_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub score: Option<Score>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub points: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub submitted_at: Option<Timestamp>,
}

/// 🎗️ A public badge award without its private run link.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceBadgeAward { pub badge: Slug, pub at: Timestamp }

/// 🏅️ The visible public profile, excluding private learner and run ids.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceProfile { pub runs: Vec<PresenceRunSummary>, pub badges: Vec<PresenceBadgeAward>, pub best: BTreeMap<Slug, Best>, pub total: f64 }

/// 📊️ The scope and period of the leaderboard being viewed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceBoard {
    pub period: LeaderboardPeriod,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub quiz: Option<Slug>,
}

/// ↕️ The public leaderboard's selected sort key and direction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceSort { pub key: String, pub direction: String }

/// ⏳️ A visible submission dialog or progress state, without executable commands.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceSubmission {
    pub phase: String,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub done: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub total: Option<u16>,
}

/// 🔭️ The public overview grid and navigation mode resolved on the publishing device.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceLayout { pub grid: String, pub mode: String }

/// 🪟️ Ephemeral screen presentation followed without modifying the follower's saved state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresencePresentation {
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub home: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub swipe: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub layout: Option<PresenceLayout>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub reveal: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub revealed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub task_page: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub viewport: Option<PresencePoint>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub camera: Option<PresencePoint>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub preferences: Option<PresencePreferences>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub run: Option<PresenceRun>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub profile: Option<PresenceProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub asked: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub board: Option<PresenceBoard>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub sort: Option<PresenceSort>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub submission: Option<PresenceSubmission>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub leaderboard: Option<Leaderboard>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub leaderboard_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub overall: Option<Leaderboard>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub overall_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub crowd: Option<CrowdView>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub awards: Option<Vec<Slug>>,
}

/// 👆️ Ephemeral shared pointer and keyboard focus in the room of one place; never names an answer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CursorState {
    pub tag: String,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub place: Option<Place>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub home: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub cursor: Option<Cursor>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
    pub focus: Option<Anchor>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present")]
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
fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}
//#endregion 🔖️Presence
