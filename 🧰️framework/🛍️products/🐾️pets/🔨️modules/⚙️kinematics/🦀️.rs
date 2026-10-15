//! ⚙️ The numerical and kinematic ground of the pets: deterministic trigonometry in turns, counter-based randomness, the forward kinematics of a rig, animation (clips, springs, blinks) and terrain (perches, walls, strides, falls and hops). Pure functions over the contract, bit for bit the results of the TypeScript twin.
//!
//! @see ../../README.md — the domain model
//! @see ../../🧬️schema/🔣️.json — the normative contract

#[path = "../📐️trigonometry/🦀️.rs"]
pub mod trigonometry;

#[path = "../🎲️randomness/🦀️.rs"]
pub mod randomness;

#[path = "../🦴️rig/🦀️.rs"]
pub mod rig;

#[path = "../🎞️animation/🦀️.rs"]
pub mod animation;

#[path = "../🏞️terrain/🦀️.rs"]
pub mod terrain;
