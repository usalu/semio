//! 🃏️ The solution-free sheet assembly laws.
pub use course::sheet::*;
use crate::randomness::{Mt19937, shuffle};
use course_schema::{Challenge, Quiz, Sheet, SheetAxis, SheetItem, SheetTask, SortingItem, Task};

#[path = "../../../🔨️modules/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
