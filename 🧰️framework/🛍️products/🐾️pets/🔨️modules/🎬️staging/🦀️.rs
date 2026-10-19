//! 🎬️ The stage that folds events into frames: the draft it works on, spacing, schedule, attention, locomotion, sociability, choice, population, the clock and the projection of a stage into a frame. The stage is the only module the packages export; the others are its parts.
//!
//! @see ../../README.md — the domain model
//! @see ../../🧬️schema/🔣️.json — the normative contract

#[path = "../💞️sociability/🦀️.rs"]
mod sociability;

#[path = "../🎯️choice/🦀️.rs"]
mod choice;

#[path = "../👥️population/🦀️.rs"]
mod population;

#[path = "../🕰️clock/🦀️.rs"]
mod clock;

#[path = "../🎥️projection/🦀️.rs"]
mod projection;

#[path = "../🎪️stage/🦀️.rs"]
pub mod stage;

#[cfg(test)]
#[path = "../📝️draft/🧪️tests/🔬️unit/🦀️.rs"]
mod draft_tests;

#[cfg(test)]
#[path = "../🗓️schedule/🧪️tests/🔬️unit/🦀️.rs"]
mod schedule_tests;

#[cfg(test)]
#[path = "../👀️attention/🧪️tests/🔬️unit/🦀️.rs"]
mod attention_tests;

#[cfg(test)]
#[path = "../🚶️locomotion/🧪️tests/🔬️unit/🦀️.rs"]
mod locomotion_tests;
