//! 👥️ The presence tag and roster laws.
pub use quiz::presence::*;
use crate::validation::{IssueCode, ValidationIssue};
use quiz_schema::{Cursor, CursorState, Identity, Place, PresenceState, Screen, SortingAnswer, ThinkingAnswer, ThinkingState};

#[path = "../../../🔨️modules/👥️presence/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
