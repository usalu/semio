//! 🐾️ Hand-written Rust twin of the normative pets contract `🧬️schema/🔣️.json` (JSON Schema draft-07): menageries of rigged species, their bonds and casts, the stage they act on, its events and the frames it projects.
//!
//! One type per `$defs` entry, same names; fields are `camelCase` on the wire, tagged unions are
//! internally tagged on `kind`, optional fields are omitted when absent, members that may be `null`
//! must still be present, tuples of a fixed length are arrays (`Ease`, the two ends of a `Bond`) and
//! unknown fields are refused like the schema's `additionalProperties: false`. Constraints serde
//! cannot express (slug and colour patterns, lengths, ranges, the `schema` identifiers, references)
//! are checked by [`crate::validation`].
//!
//! Numbers: every `number` is an `f64`; [`Ticks`] are `i64` (whole ticks, exact as `f64` up to 2⁵³);
//! a seed and a draw counter are `u32`, because they are words of a randomness key and the counters
//! wrap at 32 bits like the TypeScript twin's `>>> 0`. Closed sets of numbers are enums that carry
//! their number on the wire ([`Facing`], [`Rate`]); the count of a `ticked` event is unsigned, and the
//! clock of a stage or a frame is refused when it is negative.
//!
//! @see ../🧬️schema/🔣️.json — the normative contract
//! @see ../🧬️schema/🟦️.ts — the TypeScript twin
//! @see <https://json-schema.org/draft-07/json-schema-validation>
//!
//! The types sit in the child folders next to this file, one per section, and are re-exported here by glob so the paths stay `schema::Name`:
//! `🏷️scalars`, `🦴️rig-and-animation`, `🐾️menagerie-and-terrain`, `🎬️stage` and `🎞️frame`.

use serde::{Deserialize, Deserializer, Serialize};

#[path = "🏷️scalars/🦀️.rs"]
mod scalars;
pub use scalars::*;

#[path = "🦴️rig-and-animation/🦀️.rs"]
mod rig_and_animation;
pub use rig_and_animation::*;

#[path = "🐾️menagerie-and-terrain/🦀️.rs"]
mod menagerie_and_terrain;
pub use menagerie_and_terrain::*;

#[path = "🎬️stage/🦀️.rs"]
mod stage;
pub use stage::*;

#[path = "🎞️frame/🦀️.rs"]
mod frame;
pub use frame::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
