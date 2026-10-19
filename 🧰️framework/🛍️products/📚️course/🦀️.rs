//! 📚️ Course product façade — render-independent declarative courses and their pure core: the contract
//! types, bit-exact randomness, the solution-free sheet of a run, validation, partial-credit scoring,
//! badges, the learner lifecycle as `decide`/`evolve` pairs and the read models a proctor serves.
//! Every function is pure and shared vocabulary with the TypeScript core (`@semio-tech/course`).
//!
//! @see README.md — the domain model
//! @see 🧬️schema/🔣️.json — the normative contract

pub use crate::badges::*;
pub use crate::challenge::*;
pub use crate::lifecycle::*;
pub use crate::presence::*;
pub use crate::randomness::*;
pub use crate::scoring::*;
pub use crate::sheet::*;
pub use crate::validation::*;
pub use crate::views::*;
pub use course_schema as schema;
pub use course_schema::{
    Answer, Badge, Best, Course, CourseView, Challenge, ClassificationAnswer, ClassificationItemResult, Command, CrowdCount, CrowdItem, CrowdScores, CrowdTask, CrowdView, CursorState, DimensionResult, Event, HandleHolder, HandleView, Identity,
    Leaderboard, LeaderboardPeriod, LeaderboardRow, LearnerView, Limits, MatchingAnswer, MatchingItemResult, Place, PresenceState, Query, Quiz, Rejection, RunResult, RunStatus, RunView, Scale, Screen, SheetTask, SortingAnswer, SortingItem,
    SortingItemResult, Task, TaskKind, TaskResult, ThinkingAnswer, ThinkingState, Timestamp, CHALLENGES, CROWD_SCORE_BINS, DEFAULT_LIMITS, LEADERBOARD_PERIODS, LEADERBOARD_TOP, MAX_TIMESTAMP, WIRE_VERSION,
};
