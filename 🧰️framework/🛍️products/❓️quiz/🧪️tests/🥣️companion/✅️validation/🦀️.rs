//! ✅️ The answer, completeness and schema validation laws.
pub use quiz::validation::*;
use quiz_schema::{Answer, Badge, Catalog, Command, MAX_TIMESTAMP, Query, Quiz, Rejection, SHORT_LENGTH, Scale, SheetItem, SheetTask, Task, Text};
use std::collections::BTreeMap;
pub use quiz::validation::testkit::*;

#[path = "../../../🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
