//! ✅️ The answer, completeness and schema validation laws.
pub use course::validation::*;
use course_schema::{Answer, Badge, Course, Command, MAX_TIMESTAMP, Query, Quiz, Rejection, SHORT_LENGTH, Scale, SheetItem, SheetTask, Task, Text};
use std::collections::BTreeMap;
pub use course::validation::testkit::*;

#[path = "../../../🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
