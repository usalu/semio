//! 🚧️ Unit suite of the clearance twin: every function against the cases of the TypeScript suite, against brute force on a lattice (the same generated cases, drawn from the same 32-bit generator), and the reference world of the TypeScript suite, in which the rules of the module, and nothing else, keep six to ten pets apart while they walk, hop, glide, fall, land on heads, are dragged and thrown by a hand and lose, shrink and move their perches.
//!
//! The world is the TypeScript suite's, expression for expression, so a run of it is a pure function of its seed and
//! rules in both languages (`d2_check_world.rs` of ticket 2026/10/02/QUIZ-PETS compares their bodies tick by tick).
//! Levels: the tests outside a submodule are fundamental, `quick` and `exhaustive` run more cases and the research
//! prototype's 24 worlds of 30 000 ticks.
//!
//! @see ../../🦀️.rs — the twin under test
//! @see ../../🟦️.ts, ../🌍️reference-world/🟦️.ts — the TypeScript twin and the reference world of its suite
//! @see ../../../🏞️terrain/🦀️.rs — the perches, falls and hops the world moves by
//! @see ../../../🎲️randomness/🦀️.rs — the draws of the world
//!
//! The suite is split into the sibling folders `🌍️reference-world` (the world), `🧰️clearance-tools` (the tools of the cases and the generated cases),
//! `📦️boxes-and-places`, `📜️claims-and-order`, `🪑️seating-and-heads` and `🏛️world-laws` (the tests); this file mounts them.

use super::*;
use semio_framework_pets_contract::schema::Ending;
use semio_framework_pets_kinematics::randomness::{random_words, unit_of};
use semio_framework_pets_kinematics::terrain::{fall_step, hop_landing, hop_of, hop_step, landing_of, stride_to, Flight};

#[path = "../🌍️reference-world/🦀️.rs"]
mod reference_world;
pub(crate) use reference_world::*;

#[path = "../🧰️clearance-tools/🦀️.rs"]
mod clearance_tools;
use clearance_tools::*;

#[path = "../📦️boxes-and-places/🦀️.rs"]
mod boxes_and_places;

#[path = "../📜️claims-and-order/🦀️.rs"]
mod claims_and_order;

#[path = "../🪑️seating-and-heads/🦀️.rs"]
mod seating_and_heads;

#[path = "../🏛️world-laws/🦀️.rs"]
mod world_laws;
