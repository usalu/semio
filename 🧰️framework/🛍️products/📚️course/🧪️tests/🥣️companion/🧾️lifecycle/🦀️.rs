//! 🧾️ The learner lifecycle decide/evolve laws and the replay fixtures the read models share.
pub use course::lifecycle::*;
use crate::randomness::run_seed;
use crate::sheet::sheet_of;
use course_schema::{Answer, Course, Challenge, Command, Event, Id, Identity, Limits, Quiz, Rejection, RunStatus, Slug, Timestamp};
use std::collections::{BTreeMap, BTreeSet};

#[path = "../../../🔨️modules/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
