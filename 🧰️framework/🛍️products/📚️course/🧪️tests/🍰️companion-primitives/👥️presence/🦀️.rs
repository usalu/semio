//! 👥️ The presence tag and roster laws.
pub use course::presence::*;
use crate::validation::{IssueCode, ValidationIssue};
use course_schema::{Cursor, CursorState, Identity, Place, PresenceState, Screen, SortingAnswer, ThinkingAnswer, ThinkingState};

#[path = "../../../🔨️modules/👥️presence/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
