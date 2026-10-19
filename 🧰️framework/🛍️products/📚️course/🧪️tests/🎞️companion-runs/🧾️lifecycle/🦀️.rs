//! 🧾️ The learner lifecycle decide/evolve laws.
pub use course::lifecycle::*;
use crate::randomness::run_seed;
use crate::sheet::sheet_of;
use course_schema::{Answer, Challenge, Command, Event, Identity, Limits, Rejection, RunStatus, Slug, Timestamp};
use std::collections::BTreeMap;

#[path = "../../../🔨️modules/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
