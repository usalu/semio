//! 🃏️ The solution-free sheet assembly laws and the fixtures the other laws share.
pub use quiz::sheet::*;
use crate::randomness::{Mt19937, shuffle};
use quiz_schema::{Challenge, MatchingTask, Quiz, Sheet, SheetAxis, SheetItem, SheetTask, SortingItem, Task, Text};

#[path = "../../../🔨️modules/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
