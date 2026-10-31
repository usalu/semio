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
//! The types sit in the child folders next to this file, one per section, and are explicitly re-exported here so the paths stay `schema::Name`:
//! `🏷️scalars`, `🦴️rig-and-animation`, `🐾️menagerie-and-terrain`, `🎬️stage` and `🎞️frame`.

use serde::{Deserialize, Deserializer, Serialize};

#[path = "🏷️scalars/🦀️.rs"]
mod scalars;
pub use scalars::Slug;
pub use scalars::Text;
pub use scalars::LANGUAGES;
pub use scalars::Degrees;
pub use scalars::Turns;
pub use scalars::Ticks;
pub use scalars::TICKS_PER_SECOND;
pub use scalars::Color;
pub use scalars::Paint;
pub use scalars::PAINTS;
pub use scalars::Channel;
pub use scalars::CHANNELS;
pub use scalars::Gait;
pub use scalars::GAITS;
pub use scalars::Activity;
pub use scalars::ACTIVITIES;
pub use scalars::PetMode;
pub use scalars::PET_MODES;
pub use scalars::Mood;
pub use scalars::MOODS;
pub use scalars::Footing;
pub use scalars::FOOTINGS;
pub use scalars::Gear;
pub use scalars::GEARS;
pub use scalars::Cue;
pub use scalars::CUES;
pub use scalars::Drift;
pub use scalars::DRIFTS;
pub use scalars::Deed;
pub use scalars::DEEDS;
pub use scalars::Pointer;
pub use scalars::POINTERS;
pub use scalars::PressPhase;
pub use scalars::PRESS_PHASES;
pub use scalars::Tier;
pub use scalars::TIERS;
pub use scalars::MENAGERIE_SCHEMA;
pub use scalars::ENSEMBLE_SCHEMA;

#[path = "🦴️rig-and-animation/🦀️.rs"]
mod rig_and_animation;
pub use rig_and_animation::Bone;
pub use rig_and_animation::PathShape;
pub use rig_and_animation::EllipseShape;
pub use rig_and_animation::RectShape;
pub use rig_and_animation::LineShape;
pub use rig_and_animation::Shape;
pub use rig_and_animation::Part;
pub use rig_and_animation::Eye;
pub use rig_and_animation::Mouth;
pub use rig_and_animation::Face;
pub use rig_and_animation::Ease;
pub use rig_and_animation::Key;
pub use rig_and_animation::Track;
pub use rig_and_animation::Clip;
pub use rig_and_animation::Repertoire;

#[path = "🐾️menagerie-and-terrain/🦀️.rs"]
mod menagerie_and_terrain;
pub use menagerie_and_terrain::Size;
pub use menagerie_and_terrain::Palette;
pub use menagerie_and_terrain::Locomotion;
pub use menagerie_and_terrain::Temperament;
pub use menagerie_and_terrain::Tint;
pub use menagerie_and_terrain::Emitter;
pub use menagerie_and_terrain::SpeciesState;
pub use menagerie_and_terrain::Trick;
pub use menagerie_and_terrain::Purr;
pub use menagerie_and_terrain::Species;
pub use menagerie_and_terrain::Bond;
pub use menagerie_and_terrain::Cast;
pub use menagerie_and_terrain::Trait;
pub use menagerie_and_terrain::Party;
pub use menagerie_and_terrain::PARTIES;
pub use menagerie_and_terrain::Effect;
pub use menagerie_and_terrain::Placement;
pub use menagerie_and_terrain::PLACEMENTS;
pub use menagerie_and_terrain::Reaction;
pub use menagerie_and_terrain::Menagerie;
pub use menagerie_and_terrain::Ensemble;
pub use menagerie_and_terrain::Point;
pub use menagerie_and_terrain::Rect;
pub use menagerie_and_terrain::Surface;
pub use menagerie_and_terrain::Perch;
pub use menagerie_and_terrain::Wall;
pub use menagerie_and_terrain::Pitch;
pub use menagerie_and_terrain::Fixture;

#[path = "🎬️stage/🦀️.rs"]
mod stage;
pub use stage::Ticked;
pub use stage::Pointed;
pub use stage::Unpointed;
pub use stage::Glanced;
pub use stage::Surveyed;
pub use stage::Summoned;
pub use stage::Tuned;
pub use stage::Hushed;
pub use stage::Over;
pub use stage::OVERS;
pub use stage::Pressed;
pub use stage::Dragged;
pub use stage::Released;
pub use stage::Cancelled;
pub use stage::Reclaimed;
pub use stage::Stirred;
pub use stage::Scrolled;
pub use stage::Played;
pub use stage::Permitted;
pub use stage::StageEvent;
pub use stage::Gaze;
pub use stage::Needs;
pub use stage::Facing;
pub use stage::FACINGS;
pub use stage::Press;
pub use stage::Warmth;
pub use stage::Circling;
pub use stage::Stroking;
pub use stage::Shaking;
pub use stage::Hover;
pub use stage::Grip;
pub use stage::Hang;
pub use stage::Canopy;
pub use stage::Parachute;
pub use stage::Reeling;
pub use stage::REELINGS;
pub use stage::Shot;
pub use stage::Rope;
pub use stage::Feeling;
pub use stage::Plume;
pub use stage::Extent;
pub use stage::Slice;
pub use stage::Claim;
pub use stage::Waypoint;
pub use stage::Ending;
pub use stage::ENDINGS;
pub use stage::Course;
pub use stage::Foothold;
pub use stage::Arrival;
pub use stage::ARRIVALS;
pub use stage::Trip;
pub use stage::Puff;
pub use stage::Actor;
pub use stage::Rapport;
pub use stage::Cooling;
pub use stage::Pledge;
pub use stage::Ladder;
pub use stage::Prank;
pub use stage::Stage;

#[path = "🎞️frame/🦀️.rs"]
mod frame;
pub use frame::EyeFrame;
pub use frame::ChuteTool;
pub use frame::RopeTool;
pub use frame::HookTool;
pub use frame::GunTool;
pub use frame::LadderTool;
pub use frame::ToolFrame;
pub use frame::ActorFrame;
pub use frame::Rate;
pub use frame::RATES;
pub use frame::LadderFrame;
pub use frame::ParticleFrame;
pub use frame::LiftFrame;
pub use frame::PuffFrame;
pub use frame::Frame;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
