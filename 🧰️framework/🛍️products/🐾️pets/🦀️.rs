//! 🐾️ Pets product façade — render-independent deterministic pets and their pure core: the contract types
//! (menageries of rigged species, the stage, its events and frames), validation, trigonometry in turns,
//! counter-based randomness, the forward kinematics of a rig, clips, springs and blinks, perches, strides, falls
//! and hops, the character of a pet in numbers, and the stage that folds events into frames. Every function is
//! pure, shares its vocabulary with the TypeScript core (`@semio-tech/pets`, snake_case here) and evaluates the
//! same expressions in the same order, so both cores yield the same bits.
//!
//! @see README.md — the domain model
//! @see 🧬️schema/🔣️.json — the normative contract

pub use crate::animation::*;
pub use crate::behavior::*;
pub use crate::randomness::*;
pub use crate::rig::*;
pub use crate::schema::*;
pub use crate::stage::*;
pub use crate::terrain::*;
pub use crate::trigonometry::*;
pub use crate::validation::*;
