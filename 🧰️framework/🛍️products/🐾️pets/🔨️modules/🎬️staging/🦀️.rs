//! 🎬️ The stage that folds events into frames: the draft it works on, spacing, schedule, attention, locomotion, sociability, choice, population, the clock and the projection of a stage into a frame. The stage is the only module the packages export; the others are its parts.
//!
//! @see ../../README.md — the domain model
//! @see ../../🧬️schema/🔣️.json — the normative contract

#[path = "../📝️draft/🦀️.rs"]
mod draft;

#[path = "../📏️spacing/🦀️.rs"]
pub mod spacing;

#[path = "../🗓️schedule/🦀️.rs"]
mod schedule;

#[path = "../👀️attention/🦀️.rs"]
mod attention;

#[path = "../🚶️locomotion/🦀️.rs"]
mod locomotion;

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
