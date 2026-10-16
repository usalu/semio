//! 📏️ The partial-credit scoring laws.
pub use quiz::scoring::*;
use crate::challenge::points;
use quiz_schema::{Answer, ClassificationAnswer, ClassificationTask, Quiz, Scale, Sheet, SheetClassificationTask, SheetMatchingTask, SheetTask, Slug, SortingAnswer, SortingTask, Task, TaskResult};
use std::collections::BTreeMap;

#[path = "../../../🔨️modules/📏️scoring/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
