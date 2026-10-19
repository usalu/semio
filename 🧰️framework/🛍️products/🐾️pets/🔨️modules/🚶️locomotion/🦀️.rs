//! 🏃️ Locomotion: how a pet gets around and how it loses its ground — setting out for a goal on its perch, striding, hopping and gliding between perches, falling, touching down, being crowded out, waiting for a partner and leaving.
//!
//! A new footing or a new way to travel belongs here: what starts it, its tick of motion and where it ends.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ./⚓️footing/🟦️.ts — the TypeScript twin, split into the child folders `📐️constants`, `⚓️footing`, `🛬️courses`, `🛫️departure`, `🏃️motion`, `✋️hand`, `🪜️outings`, `🧳️trips`, `🪄️mischief` and `🚪️leaving` next to this file

use crate::draft::{beat_of, clip_at, clip_of, hover_of, release, remove, settle, shift, shoulders, Draft, Launch, ORIGIN, RATE};
use crate::spacing::{clearway, hindered, soars, vacancy, COMFORT_GAP, MEET_GAP};
use semio_framework_pets_character::behavior::dwell_of;
use semio_framework_pets_contract::schema::{Activity, Footing, Gait, Perch, Point, Slug, Species, Ticks};
use semio_framework_pets_kinematics::animation::clip_ticks;
use semio_framework_pets_kinematics::terrain::{fall_step, hop_landing, hop_of, hop_step, landing_of, larger, perch_at, smaller, stride_to, HOP_DISTANCE, HOP_HEIGHT};
use semio_framework_pets_kinematics::trigonometry::clamp;

//#region 🔖️Constants
const PATIENCE: Ticks = 1920;
const LEAVE_REACH: f64 = 3.0;
const GLIDE_TICKS: Ticks = 256;
const CLAIM_SPAN: Ticks = 4;
const LANE_RAMP: f64 = 8.0;
const DIZZY_SPEED: f64 = 780.0;
//#endregion 🔖️Constants

#[path = "⚓️footing/🦀️.rs"]
mod footing;
pub use footing::{course_of, drop, unfoot, vanish, Plan};
use footing::{install, plan_of};

#[path = "🛬️courses/🦀️.rs"]
mod courses;
use courses::ways_down;

#[path = "🛫️departure/🦀️.rs"]
mod departure;
pub use departure::{attend, hops_of, launch, paced, stroll};

#[path = "🏃️motion/🦀️.rs"]
mod motion;
pub use motion::{daze, fly, plunge, poof, scoot, slip, stride};
#[cfg(test)]
use motion::guarded;
use motion::{arrive, dusted};

#[path = "✋️hand/🦀️.rs"]
mod hand;
pub(crate) use hand::{give_back, hold, let_go, lift, toss, toss_target};

#[path = "🪜️outings/🦀️.rs"]
mod outings;
pub(crate) use outings::{geared, reaches, set_out, trip_of, Outing, Venture};
use outings::{
    effort_of, embark, grip_now, ladder_ways, lasting, outings_to, ranked, reserve_of, scaling,
    slide_outing, spaced, walk_end, walked, ways_off, Stand,
};

#[path = "🧳️trips/🦀️.rs"]
mod trips;
pub(crate) use trips::{cling, halt, rest, travel};
use trips::ungear;

#[path = "🪄️mischief/🦀️.rs"]
mod mischief;
pub use mischief::{post_to, push_at, unlifted, unpush};

#[path = "🚪️leaving/🦀️.rs"]
mod leaving;
pub use leaving::leave;
