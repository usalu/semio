//! 🃏️ The solution-free sheet assembly laws.
pub use quiz::sheet::*;
use crate::randomness::{Mt19937, shuffle};
use quiz_schema::{Challenge, Quiz, Sheet, SheetAxis, SheetItem, SheetTask, SortingItem, Task};

#[path = "../../../🔨️modules/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
