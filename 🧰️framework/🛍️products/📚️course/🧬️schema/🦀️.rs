//! 🧬️ Hand-written Rust twin of the normative quiz contract `🧬️schema/🔣️.json` (JSON Schema draft-07).
//!
//! One type per `$defs` entry, same names; fields are `camelCase` on the wire, tagged unions are
//! internally tagged (`kind` for tasks, sheet tasks, answers, results, identities and badge rules;
//! `type` for commands, events and queries) with kebab-case variant names, id-keyed maps are
//! `BTreeMap`s, optional fields are omitted when absent and unknown fields are refused like the
//! schema's `additionalProperties: false`. Constraints serde cannot express (slug patterns, lengths,
//! cardinalities, references) are checked by the `validation` module of `semio-framework-course`.
//!
//! @see ../🧬️schema/🔣️.json — the normative contract
//! @see ../🧬️schema/🟦️.ts — the TypeScript twin
//! @see <https://json-schema.org/draft-07/json-schema-validation>

#[cfg(any(test, feature = "testkit"))]
use serde::Serialize;
#[cfg(test)]
use std::collections::BTreeMap;

#[path = "📚️quiz-documents/🦀️.rs"]
mod documents;
pub use documents::{Assignment, AssignmentDocument, AssignmentDocuments, Axis, Badge, BadgeRule, CHALLENGES, COURSE_ROOT_IDS, Course, Category, Challenge, ClassificationItem, ClassificationTask, Dimension, Handle, Icon, Id, Introduction, MAX_TIMESTAMP, MatchingItem, MatchingTask, Motion, Profile, Quantity, QuestionPool, Quiz, SHORT_LENGTH, Scale, Score, ShortText, Slug, SortingItem, SortingTask, Task, TaskKind, Text, Timestamp, Topic, Unlocks, WIRE_VERSION, WireVersion};

#[path = "🃏️sheets-and-results/🦀️.rs"]
mod sheets_and_results;
pub use sheets_and_results::{Answer, ClassificationAnswer, ClassificationItemResult, DimensionResult, MatchingAnswer, MatchingItemResult, RunResult, Sheet, SheetAxis, SheetClassificationTask, SheetDimension, SheetItem, SheetMatchingTask, SheetSortingTask, SheetTask, SortingAnswer, SortingItemResult, TaskResult};

#[path = "🔁️commands-and-events/🦀️.rs"]
mod commands_and_events;
pub use commands_and_events::{Command, DEFAULT_LIMITS, Event, Identity, IdentityClaim, Limits, Rejection};

#[path = "👁️read-views/🦀️.rs"]
mod read_views;
pub use read_views::{BadgeAward, Best, CROWD_SCORE_BINS, CourseBadgeView, CourseQuizView, CourseTaskView, CourseView, CategoryHint, CompareHint, CrowdCount, CrowdItem, CrowdScores, CrowdTask, CrowdView, GroupHint, HandleHolder, HandleView, Hint, LEADERBOARD_PERIODS, LEADERBOARD_TOP, Leaderboard, LeaderboardPeriod, LeaderboardRow, LeaderboardWindow, LearnerView, ProfileHint, Query, RunStatus, RunSummary, RunView, VERDICTS, Verdict};

#[path = "👥️presence-wire/🦀️.rs"]
mod presence_wire;
pub use presence_wire::{Anchor, Cursor, CursorDrag, CursorState, Place, PresenceBadgeAward, PresenceBoard, PresenceLayout, PresencePoint, PresencePreferences, PresencePresentation, PresenceProfile, PresenceRun, PresenceRunSummary, PresenceSort, PresenceState, PresenceSubmission, SCREENS, Screen, ThinkingAnswer, ThinkingMatchingAnswer, ThinkingState};

//#region 🧰️Testkit

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

//#endregion 🧰️Testkit

//#region 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

#[path = "🖥️mockups/🦀️.rs"]
mod mockups;
pub use mockups::*;
