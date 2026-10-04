//! 🧗️ How pets get to a perch they can neither walk nor hop to: up and down the free side walls of the surveyed elements, on a ladder leant against a wall, and on a grappling rope hooked to an edge above.
//!
//! Positions are viewport pixels with the y axis pointing down, speeds are pixels per second and time is whole ticks of
//! 1/64 s. Lengths of a body are fractions of the species `Size` (`width` across, `height` up from the feet); the
//! starting values are those of the research brief for pets of 40 to 56 px. Everything is built from `+ − × ÷`,
//! `sqrt`, `abs`, `floor`, the larger and the smaller of two numbers (JavaScript's `Math.max` and `Math.min`, the
//! terrain's `larger` and `smaller`) and comparisons, evaluated in the order of the TypeScript twin and never fused
//! (`mul_add`), so both cores yield the same bits.
//!
//! Nothing here keeps state. An actor on a wall is its pitch and the height of its feet, on a ladder the distance it
//! has climbed along the rails, on a rope the length of rope left; the stage stores those numbers and asks this module
//! for the next one. Travel is clipped by distance, never by time: the phase of the climbing clip is a function of
//! where the hands are ([`climb_phase`], [`ladder_phase`]), so limbs never slide on what they hold.
//!
//! Whatever a rope or a ladder touches is not in its own way: a keep-out that holds the point a line stands on or
//! ends at (the box of the surface under a hook, the box behind the rim a ladder leans on) is passed over when the
//! line is tested ([`ladder_for`], [`shot_for`]).
//!
//! A wall, per tick: `climb_step(y, goal, ticks)` with the ticks since the climb began, then `grip_step(grip,
//! Effort::Climb)`; at [`rim_of`] the mantle, `mantle_path(pitch, size, ticks ÷ MANTLE_TICKS)`. A ladder:
//! `ladder_step(travel, goal, ticks)` and `ladder_at(ladder, travel)`, then [`hoist_path`] from the exit to
//! [`ladder_landing`]. A rope: `hook_step(shot.muzzle, shot.hook, HOOK_SPEED, ticks)` until [`hook_ticks`], then from
//! `haul_of(shot, shot.length, size)` on `haul_step(shot, haul, ticks, size)` until `REEL_LEAST` heights of rope are
//! left, then [`hoist_path`] to [`landing_for`]. After every survey [`wall_holds`], [`ladder_holds`] and
//! [`shot_holds`] say whether what carries an actor is still there.
//!
//! The pitches of one wall line — the side edges of a column of cards, the same side, no more than `WALL_FOLLOW`
//! apart — are one way up and down: a climber lunges across the gap between two of them when its hands reach the
//! lowest hold of the upper one within `CROSS_REACH` heights from the top of the lower one ([`crossable`],
//! [`lunge_path`]), and [`wall_path`] is the whole way, tick by tick, from taking hold to the mantle.
//!
//! The ladder of this module is [`LadderStand`], the five fields the stage's `Ladder` carries besides its owner, its
//! lifetime and its rider; a pitch and a shot are the schema's. An answered pitch, perch or standing ladder, and a
//! member of a wall line, is a reference into the list it was asked of, like the TypeScript twin answers the very
//! object.
//!
//! @see ../🏞️terrain/🦀️.rs — `walls_of`, `segment_hits`, `stride_to`, `larger`, `smaller`
//! @see ../🪢️swing/🦀️.rs — `reel_step`, the swing on a rope that is reeled in, and `REEL_LEAST`, the rope a haul ends with
//! @see ../📐️trigonometry/🦀️.rs — `clamp`, `smoothstep`
//! @see ../../🧬️schema/🦀️.rs — `Gear`, `Perch`, `Pitch`, `Point`, `Rect`, `Size`, `Shot`, `Reeling`, `Facing`
//! @see ../🧗️climbing/🟦️.ts — the TypeScript twin

use crate::schema::{Facing, Gear, Perch, Pitch, Point, Rect, Reeling, Shot, Size, Ticks, TICKS_PER_SECOND};
use crate::swing::{reel_step, REEL_LEAST};
use crate::terrain::{larger, segment_hits, smaller, stride_to, Fall};
use crate::trigonometry::{clamp, smoothstep};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// ✋️ How high above its feet an actor holds on, as a fraction of its height: the mantle begins when the hands reach the rim.
pub const HAND_HEIGHT: f64 = 0.85;

/// ⬆️ The speed of climbing up a wall in pixels per second (0.62 body heights).
pub const CLIMB_RISE: f64 = 30.0;

/// ⬇️ The speed of climbing down a wall in pixels per second.
pub const CLIMB_DESCENT: f64 = 40.0;

/// 🛫️ Over how many ticks a climb on a wall gathers its speed.
pub const CLIMB_RAMP: Ticks = 6;

/// 🤲️ The distance between two holds on a wall as a fraction of the height; two holds, one per hand, are one cycle of the climbing clip.
pub const GRIP_SPACING: f64 = 0.3;

/// 🔋️ The grip a rested actor has, in ticks of climbing (6 s): enough for 150 px of wall and the mantle.
pub const GRIP_BUDGET: f64 = 384.0;

/// 💪️ The grip one tick of climbing, mantling or hanging over a rim costs.
pub const GRIP_CLIMB: f64 = 1.0;

/// 🐒️ The grip one tick of hanging still or sliding costs.
pub const GRIP_HANG: f64 = 0.25;

/// 🛌️ The grip one tick on a perch gives back.
pub const GRIP_REST: f64 = 2.0;

/// 🦷️ How many pixels of a pitch must lie under the hands: the hands never hold the very end of a free stretch.
pub const GRIP_BITE: f64 = 8.0;

/// 🧲️ How far a wall may move sideways between two surveys, in pixels, and still carry its climber.
pub const WALL_FOLLOW: f64 = 6.0;

/// 🛝️ The speed a slide down a wall starts with, in pixels per second.
pub const SLIDE_START: f64 = 30.0;

/// 📈️ The acceleration of a slide in pixels per second squared.
pub const SLIDE_GAIN: f64 = 600.0;

/// 🚒️ The fastest slide in pixels per second.
pub const SLIDE_SPEED: f64 = 160.0;

/// 💨️ The sideways speed with which a wall that moved or vanished throws its climber off, in pixels per second, away from the wall.
pub const SLIP_PUSH: f64 = 140.0;

/// 🤸️ The upward speed of that throw in pixels per second.
pub const SLIP_LIFT: f64 = 160.0;

/// 🤏️ The ticks of taking hold of a wall from its foot.
pub const WALL_GRAB_TICKS: Ticks = 8;

/// 🦥️ The ticks of lowering oneself over a rim onto the wall below.
pub const WALL_HANG_TICKS: Ticks = 10;

/// 🏋️ The ticks of a mantle from a wall over its rim onto the perch.
pub const MANTLE_TICKS: Ticks = 28;

/// 📥️ How far beyond half a body width inside the rim a mantle ends, in pixels.
pub const MANTLE_INSET: f64 = 8.0;

/// 🐫️ How far above its end a hoist rises before it settles, as a fraction of the height.
pub const HOIST_HUMP: f64 = 0.1;

/// 🌅️ The share of a hoist over which the body rises; the way across begins when this share of the hoist is left.
pub const HOIST_RISE: f64 = 0.65;

/// 📐️ The lean of a ladder its owner aims for: foot distance ÷ rise, the 4 : 1 rule of real ladders.
pub const LADDER_LEAN: f64 = 0.25;

/// 🗼️ The steepest lean a ladder stands at.
pub const LADDER_STEEP: f64 = 0.14;

/// 🛬️ The flattest lean a ladder stands at.
pub const LADDER_FLAT: f64 = 0.4;

/// 🪑️ The least rise of a ladder as a fraction of the height; lower edges are hopped onto.
pub const LADDER_SHORT: f64 = 0.8;

/// 🦒️ The greatest rise of a ladder as a fraction of the height.
pub const LADDER_TALL: f64 = 3.6;

/// 👇️ How far under the rim a ladder touches its wall, in pixels (0.15 heights of a 48 px pet): a measure of the ladder, the same for whoever climbs it.
pub const LADDER_TUCK: f64 = 7.0;

/// 🦌️ How far the rails reach beyond that contact, in pixels; drawn, never climbed.
pub const LADDER_HORNS: f64 = 14.0;

/// 🪜️ The distance between two rungs in pixels (0.22 heights of a 48 px pet); two rungs are one cycle of the climbing clip.
pub const RUNG_SPACING: f64 = 10.5;

/// 🦶️ Half the footprint of a ladder on its perch in pixels: the foot stays this far from both ends.
pub const LADDER_FOOTING: f64 = 10.0;

/// 🥖️ Half the thickness of a ladder in pixels: keep-outs are grown by it when its line is tested.
pub const LADDER_GIRTH: f64 = 6.0;

/// 🚀️ The speed of climbing up a ladder in pixels per second (2.5 rungs of a 48 px pet).
pub const LADDER_RISE: f64 = 26.0;

/// 🪂️ The speed of climbing down a ladder in pixels per second.
pub const LADDER_DESCENT: f64 = 34.0;

/// 🏁️ Over how many ticks a climb on a ladder gathers its speed.
pub const LADDER_RAMP: Ticks = 6;

/// 🚪️ How far before the top of a ladder its climber steps over onto the perch, as a fraction of the height.
pub const LADDER_EXIT: f64 = 0.4;

/// 🧷️ How far the top of a ladder may move between two surveys, in pixels, and be followed.
pub const LADDER_FOLLOW: f64 = 8.0;

/// 🌋️ How far the perch under a ladder may move up or down between two surveys, in pixels, before the ladder falls.
pub const LADDER_SHIFT: f64 = 12.0;

/// 🐎️ The ticks of stepping onto a ladder.
pub const LADDER_MOUNT_TICKS: Ticks = 8;

/// 🛹️ The ticks of stepping from a ladder over the rim onto the perch.
pub const LADDER_DISMOUNT_TICKS: Ticks = 24;

/// 🏗️ The ticks of raising a ladder from the perch against its wall, and of lowering it again.
pub const LADDER_RAISE_TICKS: Ticks = 30;

/// 💤️ The ticks after its last use at which a standing ladder is taken away (20 s).
pub const LADDER_IDLE: Ticks = 1280;

/// ⌛️ The ticks a ladder stands at most (120 s).
pub const LADDER_LIFE: Ticks = 7680;

/// 🌀️ The stiffness of the spring that topples a ladder about its foot.
pub const TOPPLE_STIFFNESS: f64 = 150.0;

/// 🧽️ The damping of that spring.
pub const TOPPLE_DAMPING: f64 = 24.0;

/// 👋️ The sideways speed with which a toppling ladder throws its climber clear, in pixels per second, away from the wall.
pub const TOPPLE_PUSH: f64 = 70.0;

/// 🩴️ The rise above the foot of a ladder, as a fraction of the height, below which its climber steps off a toppling ladder instead of falling.
pub const TOPPLE_STEP: f64 = 0.5;

/// 🏹️ The speed of a flying hook in pixels per second (10 px per tick).
pub const HOOK_SPEED: f64 = 640.0;

/// ↩️ The speed of a hook that is pulled back after a miss, in pixels per second.
pub const HOOK_RETURN: f64 = 1280.0;

/// 🧵️ The shortest rope as a fraction of the height.
pub const ROPE_SHORT: f64 = 0.8;

/// 🧶️ The longest rope as a fraction of the height.
pub const ROPE_LONG: f64 = 3.2;

/// 🌄️ The least rise of a rope per pixel of its length (about 25°): a flatter rope would scrape the face of a card.
pub const ROPE_ELEVATION: f64 = 0.42;

/// 📏️ By how many pixels keep-outs are grown when the line of a rope is tested.
pub const ROPE_MARGIN: f64 = 2.0;

/// 🔀️ What a pixel of sideways distance costs beside a pixel of rope when the hook point is chosen.
pub const ROPE_DETOUR: f64 = 0.3;

/// 🪝️ How far the edge under a hook may move up or down between two surveys, in pixels, before the hook loses it.
pub const ROPE_FOLLOW: f64 = 8.0;

/// 🏔️ The least height of a perch above the feet for a rope, as a fraction of the height; lower perches are hopped onto.
pub const ROPE_RISE: f64 = 0.8;

/// 🔫️ How far in front of the feet the muzzle of the gun is, as a fraction of the width.
pub const MUZZLE_FORWARD: f64 = 0.3;

/// 🎚️ How far above the feet the muzzle is, as a fraction of the height; the hands hold the rope there.
pub const MUZZLE_HEIGHT: f64 = 0.65;

/// 📌️ How far beyond half a body width inside the end of a perch a hook bites, in pixels.
pub const HOOK_INSET: f64 = 4.0;

/// 🎈️ How far above the edge a hook bites, in pixels.
pub const HOOK_LIFT: f64 = 1.0;

/// ⚡️ The widest slant of a rope that is reeled in straight, sideways distance ÷ rise; a wider one swings.
pub const ZIP_SLANT: f64 = 0.35;

/// 🚡️ The speed of reeling straight up a rope in pixels per second.
pub const ZIP_SPEED: f64 = 150.0;

/// 🎢️ Over how many ticks a zip gathers its speed.
pub const ZIP_RAMP: Ticks = 10;

/// 🎯️ The ticks of aiming before a shot.
pub const ROPE_AIM_TICKS: Ticks = 14;

/// 💥️ The ticks of the recoil after a shot.
pub const ROPE_RECOIL_TICKS: Ticks = 6;

/// 🪢️ The ticks of the tug that tests a hook before reeling.
pub const ROPE_TUG_TICKS: Ticks = 4;

/// 🛗️ The ticks of the last hand-over-hand metre and the mantle onto the perch.
pub const ROPE_HOIST_TICKS: Ticks = 46;

/// 🤷️ The ticks of the shrug after a miss.
pub const ROPE_SHRUG_TICKS: Ticks = 40;

/// 🐸️ The farthest lunge from one pitch of a wall line to the next, as a fraction of the height: the hands reach from the top of the lower pitch to the lowest hold of the upper one (`GRIP_BITE` above its end) across a gap of up to this many heights less the bite, and drop as far.
pub const CROSS_REACH: f64 = 1.5;

/// 🦗️ The ticks of a lunge from one pitch of a wall line to the next, up or down.
pub const LUNGE_TICKS: Ticks = 16;

/// 🎲️ The chance of a shot that is aimed past the edge for charm.
pub const ROPE_MISS_CHANCE: f64 = 0.1;

/// 🙈️ How far past the end of the perch such a shot is aimed, in pixels.
pub const ROPE_MISS_OVERSHOOT: f64 = 14.0;

/// ⏲️ The ticks before the next shot after a miss (4 s).
pub const ROPE_REST: Ticks = 256;

/// ⏰️ The ticks before the next shot after two misses in a row (60 s).
pub const ROPE_SULK: Ticks = 3840;

const PATIENCE: Ticks = 1024;
const LONGEST: Ticks = 4096;
const RATE: f64 = TICKS_PER_SECOND as f64;
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🧤️ Where the feet of an actor are once it has taken hold of a pitch, and whether it came over the rim from the perch on top (`over`) or from a perch at the foot of the wall.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WallHold {
    pub x: f64,
    pub y: f64,
    pub over: bool,
}

/// 🏃️ What an actor on a wall is doing with its grip: climbing, hanging still or resting on a perch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Effort {
    Climb,
    Hang,
    Rest,
}

/// 🤾️ A velocity in pixels per second with which an actor is thrown into the air.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Toss {
    pub vx: f64,
    pub vy: f64,
}

/// 🎣️ An actor that is reeled in: the rope left between its hands and the hook, where its hands are and where they were a tick before, and where its feet are.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Haul {
    pub rope: f64,
    pub hand: Point,
    pub before: Point,
    pub x: f64,
    pub y: f64,
}

/// 🧰️ A ladder as it stands: the wall it leans against and the side that wall faces, the surface it stands on, its foot on that surface and its top on the wall — the stage's `Ladder` without its owner, its lifetime and its rider.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LadderStand {
    pub wall: String,
    pub surface: String,
    pub side: Facing,
    pub foot: Point,
    pub top: Point,
}

/// 🖐️ What the hands do at one tick of a way along a wall: take hold from a perch beside it, lower the body over the rim from the perch on top, climb, lunge to the next pitch of the wall line, or mantle over the rim onto the perch on top.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Handwork {
    Grab,
    Hang,
    Climb,
    Lunge,
    Mantle,
}

/// 👣️ One tick of a way along a wall: where the feet are at its end, which pitch of the wall line the hands hold or reach for (its index in the line) and what they do.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Clamber {
    pub x: f64,
    pub y: f64,
    pub hold: usize,
    pub work: Handwork,
}

/// 🔑️ How a way along a wall begins: taking hold from a perch beside the wall (`grab`), lowering the body over the rim from the perch on top (`hang`), or holding on already (`cling`) — the `entry` of the TypeScript twin's `wallPath`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Entry {
    Grab,
    Hang,
    Cling,
}

/// 🗺️ One way from a perch to another: where on its perch the actor walks to first (`at`) and what it uses from there — a ladder that stands (`up` or down), a wall line (the pitch it takes hold of and the hold it takes, the pitch at whose rim or foot its climb ends and the height it ends at there), a ladder of its own raised on the spot, or a shot of its grappling gun. A standing ladder and a pitch are the very entries of the lists the way was asked of.
#[derive(Clone, Debug, PartialEq)]
pub enum Leg<'a> {
    Ladder { at: f64, ladder: &'a LadderStand, up: bool },
    Wall { at: f64, pitch: &'a Pitch, hold: WallHold, exit: &'a Pitch, goal: f64 },
    Raise { at: f64, ladder: LadderStand },
    Grapple { at: f64, shot: Shot },
}
//#endregion 🔖️Types

//#region 🔖️Measures
/// 🧮️ The least whole number that is not below `value`, by `floor` alone.
fn ceiling(value: f64) -> f64 {
    0.0 - (0.0 - value).floor()
}

/// 🍰️ The part of `value` beyond its whole number, in [0, 1).
fn fraction(value: f64) -> f64 {
    value - value.floor()
}

/// 🐢️ The speed in the tick that begins `ticks` ticks after a start, gathered over `ramp` ticks: `speed × smoothstep((ticks + 1) ÷ ramp)`.
fn gathered(speed: f64, ticks: Ticks, ramp: Ticks) -> f64 {
    speed * smoothstep((ticks + 1) as f64 / ramp as f64)
}

/// 📦️ Whether `rect` has an area and holds `point`, its edges included.
fn holds(rect: &Rect, point: Point) -> bool {
    rect.width > 0.0 && rect.height > 0.0 && rect.x <= point.x && point.x <= rect.x + rect.width && rect.y <= point.y && point.y <= rect.y + rect.height
}

/// 🔭️ Whether the segment from `from` to `to` hits none of the keep-outs grown by `margin`, passing over every keep-out that holds `first` or `second`: what a line stands on or ends at is not in its way.
pub fn sighted(from: Point, to: Point, keepouts: &[Rect], margin: f64, first: Point, second: Point) -> bool {
    keepouts.iter().all(|keepout| holds(keepout, first) || holds(keepout, second) || !segment_hits(from, to, keepout, margin))
}
//#endregion 🔖️Measures

//#region 🔖️Hoisting
/// 🦘️ The feet at `phase` 0…1 of a hoist from `from` over an edge onto `to`: the body rises over the first `HOIST_RISE` of the way to `HOIST_HUMP` heights above its end, crosses over the last `HOIST_RISE` of it and settles onto the end over the rest, each part eased by `smoothstep`; `from` itself at 0 and before, `to` itself at 1 and after. A descent over an edge is the same path with the phase running back.
pub fn hoist_path(from: Point, to: Point, height: f64, phase: f64) -> Point {
    if phase >= 1.0 {
        return Point { x: to.x, y: to.y };
    }
    let hump = HOIST_HUMP * height;
    let rise = smoothstep(phase / HOIST_RISE);
    let across = smoothstep((phase - (1.0 - HOIST_RISE)) / HOIST_RISE);
    let settle = smoothstep((phase - HOIST_RISE) / (1.0 - HOIST_RISE));
    Point { x: from.x + (to.x - from.x) * across, y: from.y + (to.y - hump - from.y) * rise + hump * settle }
}
//#endregion 🔖️Hoisting

//#region 🔖️Walls
/// 🦎️ The x of the feet of an actor that clings to `pitch`: half its width out on the air side of the wall.
pub fn cling_of(pitch: &Pitch, size: Size) -> f64 {
    pitch.x + (pitch.side.sign() * size.width) / 2.0
}

/// 🏝️ The x of the spot on top of the wall where a mantle ends: half a width and `MANTLE_INSET` inside the rim.
pub fn ledge_of(pitch: &Pitch, size: Size) -> f64 {
    pitch.x - pitch.side.sign() * (size.width / 2.0 + MANTLE_INSET)
}

/// 🧢️ The height of the feet at which the hands reach the top of `pitch`: the highest an actor climbs on it.
pub fn rim_of(pitch: &Pitch, size: Size) -> f64 {
    pitch.y0 + HAND_HEIGHT * size.height
}

/// 🥾️ The height of the feet at which the hands hold `GRIP_BITE` above the lower end of `pitch`: the lowest an actor hangs on it.
pub fn foot_of(pitch: &Pitch, size: Size) -> f64 {
    pitch.y1 - GRIP_BITE + HAND_HEIGHT * size.height
}

/// 🦀️ Whether an actor with its feet at height `y` has its hands on `pitch`: between [`rim_of`] and [`foot_of`], both included.
pub fn clings(pitch: &Pitch, y: f64, size: Size) -> bool {
    rim_of(pitch, size) <= y && y <= foot_of(pitch, size)
}

/// 👑️ Whether `perch` lies on top of `pitch`: on the surface the wall belongs to, at the height the pitch begins at, and carrying the ledge beside the rim.
pub fn crowns(perch: &Perch, pitch: &Pitch, size: Size) -> bool {
    let ledge = ledge_of(pitch, size);
    perch.surface == pitch.surface && perch.y == pitch.y0 && perch.x0 <= ledge && ledge <= perch.x1
}

/// 🏆️ The first perch that crowns `pitch`: where a mantle from it ends; `None` when the rim carries nobody.
pub fn rim_for<'a>(pitch: &Pitch, perches: &'a [Perch], size: Size) -> Option<&'a Perch> {
    perches.iter().find(|perch| crowns(perch, pitch, size))
}

/// 🫴️ The hold an actor on `perch` takes on `pitch`, or `None` when it cannot: from the perch that crowns the pitch it lowers itself over the rim (`over`, feet at [`rim_of`]); from any other perch it must be able to stand at [`cling_of`] with its hands on the pitch, and keeps its height.
pub fn grip_for(perch: &Perch, pitch: &Pitch, size: Size) -> Option<WallHold> {
    let x = cling_of(pitch, size);
    let rim = rim_of(pitch, size);
    if crowns(perch, pitch, size) {
        return clings(pitch, rim, size).then_some(WallHold { x, y: rim, over: true });
    }
    (perch.x0 <= x && x <= perch.x1 && clings(pitch, perch.y, size)).then_some(WallHold { x, y: perch.y, over: false })
}

/// 🧐️ The pitch that still carries an actor at height `y` after a survey: the first of `pitches` on the same wall and side, no more than `WALL_FOLLOW` aside of `pitch`, that the actor clings to; `None` when the wall moved away, vanished or is no longer free there — the actor slips.
pub fn wall_holds<'a>(pitch: &Pitch, pitches: &'a [Pitch], y: f64, size: Size) -> Option<&'a Pitch> {
    pitches.iter().find(|next| next.wall == pitch.wall && next.side == pitch.side && (next.x - pitch.x).abs() <= WALL_FOLLOW && clings(next, y, size))
}

/// 🍌️ The velocity with which an actor that lost `pitch` is thrown off: `SLIP_PUSH` away from the wall and `SLIP_LIFT` up.
pub fn slip_of(pitch: &Pitch) -> Toss {
    Toss { vx: pitch.side.sign() * SLIP_PUSH, vy: 0.0 - SLIP_LIFT }
}

/// 🔌️ The grip one tick later: climbing costs `GRIP_CLIMB`, hanging `GRIP_HANG`, never below 0; resting gives `GRIP_REST` back, never beyond `GRIP_BUDGET`.
pub fn grip_step(grip: f64, effort: Effort) -> f64 {
    match effort {
        Effort::Rest => smaller(grip + GRIP_REST, GRIP_BUDGET),
        Effort::Climb => larger(grip - GRIP_CLIMB, 0.0),
        Effort::Hang => larger(grip - GRIP_HANG, 0.0),
    }
}

/// 🐜️ The height of the feet one tick later on the way to `goal`, `ticks` ticks after the climb began: up at `CLIMB_RISE`, down at `CLIMB_DESCENT`, gathered over `CLIMB_RAMP` ticks, and the goal itself as soon as it is within one step.
pub fn climb_step(y: f64, goal: f64, ticks: Ticks) -> f64 {
    stride_to(y, goal, gathered(if goal < y { CLIMB_RISE } else { CLIMB_DESCENT }, ticks, CLIMB_RAMP))
}

/// ⏱️ How many ticks the climb from `y` to `goal` takes; `GRIP_BUDGET + 1` when no rested grip lasts for it.
pub fn climb_ticks(y: f64, goal: f64) -> Ticks {
    let mut height = y;
    let mut ticks: Ticks = 0;
    while height != goal && (ticks as f64) <= GRIP_BUDGET {
        height = climb_step(height, goal, ticks);
        ticks += 1;
    }
    ticks
}

/// 🎡️ The phase 0…1 of the climbing clip at height `y` on `pitch`: one cycle per two holds of `GRIP_SPACING` heights, counted up the wall from its top, so the hands meet the same spots of the wall on every pass.
pub fn climb_phase(pitch: &Pitch, y: f64, size: Size) -> f64 {
    fraction((pitch.y0 - y) / (2.0 * GRIP_SPACING * size.height))
}

/// 🧈️ One tick of sliding down a wall, velocity first: `vy ← min(max(vy, SLIDE_START) + SLIDE_GAIN ÷ 64, SLIDE_SPEED)`, then `y ← min(y + vy ÷ 64, floor)` — the slide ends at `floor`, the perch below or [`foot_of`].
pub fn slide_step(y: f64, vy: f64, floor: f64) -> Fall {
    let speed = smaller(larger(vy, SLIDE_START) + SLIDE_GAIN / RATE, SLIDE_SPEED);
    Fall { y: smaller(y + speed / RATE, floor), vy: speed }
}

/// 🧘️ The feet at `phase` 0…1 of the mantle from `pitch` over its rim onto the perch that crowns it: [`hoist_path`] from the highest hold to the ledge.
pub fn mantle_path(pitch: &Pitch, size: Size, phase: f64) -> Point {
    hoist_path(Point { x: cling_of(pitch, size), y: rim_of(pitch, size) }, Point { x: ledge_of(pitch, size), y: pitch.y0 }, size.height, phase)
}

/// 🌉️ Whether an actor of `size` lunges from `from` to `to`: the two stand on one wall line (the same side, no more than `WALL_FOLLOW` apart), one lies wholly above the other, each holds the actor somewhere (its rim not below its foot), and the hands cross the gap from the top of the lower one to the lowest hold of the upper one, `GRIP_BITE` above its end, within `CROSS_REACH` heights.
pub fn crossable(from: &Pitch, to: &Pitch, size: Size) -> bool {
    if from.side != to.side || (to.x - from.x).abs() > WALL_FOLLOW {
        return false;
    }
    let upward = to.y1 <= from.y0;
    let downward = from.y1 <= to.y0;
    if !upward && !downward {
        return false;
    }
    let (upper, lower) = if upward { (to, from) } else { (from, to) };
    rim_of(upper, size) <= foot_of(upper, size) && rim_of(lower, size) <= foot_of(lower, size) && lower.y0 - upper.y1 + GRIP_BITE <= CROSS_REACH * size.height
}

/// 🐇️ The feet at `phase` 0…1 of a lunge from `from` to `to`: [`hoist_path`] from the hold of `from` nearest to `to` (its rim when `to` lies above, its foot when below) to the hold of `to` nearest to `from` (its foot, or its rim), half a width out from each wall.
pub fn lunge_path(from: &Pitch, to: &Pitch, size: Size, phase: f64) -> Point {
    let upward = to.y1 <= from.y0;
    let start = Point { x: cling_of(from, size), y: if upward { rim_of(from, size) } else { foot_of(from, size) } };
    let end = Point { x: cling_of(to, size), y: if upward { foot_of(to, size) } else { rim_of(to, size) } };
    hoist_path(start, end, size.height, phase)
}

/// ⛓️ The wall line an actor of `size` climbs from `pitch` without letting go: `pitch` and every pitch of `pitches` it reaches by lunges from one to the next ([`crossable`]) — above it the one with the lowest end, below it the one with the highest top, the first among equals —, from the top down.
pub fn chain_of<'a>(pitch: &'a Pitch, pitches: &'a [Pitch], size: Size) -> Vec<&'a Pitch> {
    let mut chain = vec![pitch];
    loop {
        let top = chain[0];
        let mut above: Option<&Pitch> = None;
        for next in pitches {
            if next.y1 <= top.y0 && crossable(top, next, size) && above.is_none_or(|held| next.y1 > held.y1) {
                above = Some(next);
            }
        }
        let Some(above) = above else {
            break;
        };
        chain.insert(0, above);
    }
    loop {
        let bottom = chain[chain.len() - 1];
        let mut below: Option<&Pitch> = None;
        for next in pitches {
            if next.y0 >= bottom.y1 && crossable(bottom, next, size) && below.is_none_or(|held| next.y0 < held.y0) {
                below = Some(next);
            }
        }
        let Some(below) = below else {
            break;
        };
        chain.push(below);
    }
    chain
}

/// 🪨️ The way of an actor of `size` along the wall line `chain` (its pitches from the top down, as [`chain_of`] answers them), tick by tick.
///
/// It takes hold of `chain[from]` — from a perch beside the wall where its feet stand at (`x`, `y`), stepping onto the
/// hold over `WALL_GRAB_TICKS` (`grab`); over the rim from the perch on top, lowering itself along the mantle path
/// backwards over `WALL_HANG_TICKS` (`hang`, `x` and `y` are not read); or holding on there already with its feet at
/// height `y` (`cling`) —, climbs on every pitch from where it holds to the end of that pitch on its way (the rim
/// upwards, the foot downwards; every climb gathers its speed anew) and lunges across every gap ([`lunge_path`] over
/// `LUNGE_TICKS`) until it reaches `goal` on `chain[to]`, and with `mantle` it mantles over the rim of `chain[to]` onto
/// the ledge ([`mantle_path`] over `MANTLE_TICKS`). A climb that does not arrive within 4096 ticks is cut there.
#[allow(clippy::too_many_arguments)]
pub fn wall_path(chain: &[&Pitch], from: usize, entry: Entry, x: f64, y: f64, to: usize, goal: f64, mantle: bool, size: Size) -> Vec<Clamber> {
    let mut path = Vec::new();
    let first = chain[from];
    let hold = cling_of(first, size);
    let mut height = y;
    match entry {
        Entry::Grab => {
            for tick in 1..=WALL_GRAB_TICKS {
                path.push(Clamber { x: if tick == WALL_GRAB_TICKS { hold } else { x + (hold - x) * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64) }, y, hold: from, work: Handwork::Grab });
            }
        }
        Entry::Hang => {
            for tick in 1..=WALL_HANG_TICKS {
                let feet = mantle_path(first, size, 1.0 - tick as f64 / WALL_HANG_TICKS as f64);
                path.push(Clamber { x: feet.x, y: feet.y, hold: from, work: Handwork::Hang });
            }
            height = rim_of(first, size);
        }
        Entry::Cling => {}
    }
    let mut at = from;
    loop {
        let pitch = chain[at];
        let cling = cling_of(pitch, size);
        let end = if at == to {
            goal
        } else if at > to {
            rim_of(pitch, size)
        } else {
            foot_of(pitch, size)
        };
        let mut tick: Ticks = 0;
        while height != end && tick < LONGEST {
            height = climb_step(height, end, tick);
            path.push(Clamber { x: cling, y: height, hold: at, work: Handwork::Climb });
            tick += 1;
        }
        if at == to {
            break;
        }
        let next = if at > to { at - 1 } else { at + 1 };
        for tick in 1..=LUNGE_TICKS {
            let feet = lunge_path(pitch, chain[next], size, tick as f64 / LUNGE_TICKS as f64);
            path.push(Clamber { x: feet.x, y: feet.y, hold: next, work: Handwork::Lunge });
        }
        height = if at > to { foot_of(chain[next], size) } else { rim_of(chain[next], size) };
        at = next;
    }
    if mantle {
        for tick in 1..=MANTLE_TICKS {
            let feet = mantle_path(chain[to], size, tick as f64 / MANTLE_TICKS as f64);
            path.push(Clamber { x: feet.x, y: feet.y, hold: to, work: Handwork::Mantle });
        }
    }
    path
}

/// 🪫️ The grip a way along a wall costs: `GRIP_CLIMB` for every tick of it — taking hold, lowering itself over a rim, climbing, lunging and mantling all hold the wall.
pub fn wall_cost(path: &[Clamber]) -> f64 {
    path.len() as f64 * GRIP_CLIMB
}
//#endregion 🔖️Walls

//#region 🔖️Ladders
/// 🪡️ The distance between two points.
fn span(from: Point, to: Point) -> f64 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    (dx * dx + dy * dy).sqrt()
}

/// 🧍️ The ladder with its foot at `x` on `low` and its top `LADDER_TUCK` under the top of `pitch`, or `None` when it cannot stand so: the rise within `LADDER_SHORT`…`LADDER_TALL` heights, the contact on the pitch, the footprint on the perch, the lean within `LADDER_STEEP`…`LADDER_FLAT` on the air side, and its line clear of every keep-out grown by `LADDER_GIRTH`.
fn stood(low: &Perch, pitch: &Pitch, x: f64, keepouts: &[Rect], size: Size) -> Option<LadderStand> {
    let top = Point { x: pitch.x, y: pitch.y0 + LADDER_TUCK };
    let foot = Point { x, y: low.y };
    let rise = foot.y - top.y;
    let lean = (x - pitch.x) * pitch.side.sign();
    let fits = LADDER_SHORT * size.height <= rise && rise <= LADDER_TALL * size.height && top.y <= pitch.y1 && low.x0 + LADDER_FOOTING <= x && x <= low.x1 - LADDER_FOOTING && LADDER_STEEP * rise <= lean && lean <= LADDER_FLAT * rise;
    (fits && sighted(foot, top, keepouts, LADDER_GIRTH, foot, top)).then(|| LadderStand { wall: pitch.wall.clone(), surface: low.surface.clone(), side: pitch.side, foot, top })
}

/// 🔨️ The ladder an actor of `size` raises on `low` against `pitch` to reach `high`, or `None`: `high` must crown the pitch, and the foot stands where the lean is `LADDER_LEAN`, or as near to that as the perch lets it.
pub fn ladder_for(low: &Perch, high: &Perch, pitch: &Pitch, keepouts: &[Rect], size: Size) -> Option<LadderStand> {
    if !crowns(high, pitch, size) {
        return None;
    }
    let wanted = pitch.x + pitch.side.sign() * LADDER_LEAN * (low.y - (pitch.y0 + LADDER_TUCK));
    stood(low, pitch, clamp(wanted, low.x0 + LADDER_FOOTING, low.x1 - LADDER_FOOTING), keepouts, size)
}

/// 🩺️ The ladder as it stands after a survey, or `None` when it topples: its foot keeps its x on a perch of its surface that moved no more than `LADDER_SHIFT` up or down, its top follows a pitch of its wall that still has a perch on top, no farther than `LADDER_FOLLOW`, and it still stands by the rules of [`ladder_for`] for its owner's `size`.
pub fn ladder_holds(ladder: &LadderStand, perches: &[Perch], pitches: &[Pitch], keepouts: &[Rect], size: Size) -> Option<LadderStand> {
    for low in perches {
        if low.surface != ladder.surface || (low.y - ladder.foot.y).abs() > LADDER_SHIFT {
            continue;
        }
        for pitch in pitches {
            if pitch.wall != ladder.wall || pitch.side != ladder.side || rim_for(pitch, perches, size).is_none() {
                continue;
            }
            if let Some(next) = stood(low, pitch, ladder.foot.x, keepouts, size) {
                if span(ladder.top, next.top) <= LADDER_FOLLOW {
                    return Some(next);
                }
            }
        }
    }
    None
}

/// 🎋️ The length of a ladder from its foot to its top.
pub fn ladder_length(ladder: &LadderStand) -> f64 {
    span(ladder.foot, ladder.top)
}

/// 🎼️ The number of rungs of a ladder of finite length: one per `RUNG_SPACING` of its length.
pub fn ladder_rungs(ladder: &LadderStand) -> u32 {
    (ladder_length(ladder) / RUNG_SPACING).floor() as u32
}

/// 🍕️ The lean of a ladder: the distance of its foot from the wall ÷ its rise.
pub fn ladder_lean(ladder: &LadderStand) -> f64 {
    (ladder.foot.x - ladder.top.x).abs() / (ladder.foot.y - ladder.top.y)
}

/// 🛎️ The distance along a ladder at which a climber of `size` steps over onto the perch: `LADDER_EXIT` heights before its top, never before its foot.
pub fn ladder_exit(ladder: &LadderStand, size: Size) -> f64 {
    larger(ladder_length(ladder) - LADDER_EXIT * size.height, 0.0)
}

/// 🐛️ The distance climbed along a ladder one tick later on the way to `goal`, `ticks` ticks after the climb began: up at `LADDER_RISE`, down at `LADDER_DESCENT`, gathered over `LADDER_RAMP` ticks, and the goal itself as soon as it is within one step.
pub fn ladder_step(travel: f64, goal: f64, ticks: Ticks) -> f64 {
    stride_to(travel, goal, gathered(if goal > travel { LADDER_RISE } else { LADDER_DESCENT }, ticks, LADDER_RAMP))
}

/// 📍️ The feet of a climber that has climbed `travel` pixels along a ladder from its foot: the foot itself at 0 and before, the top itself at its length and beyond.
pub fn ladder_at(ladder: &LadderStand, travel: f64) -> Point {
    let length = ladder_length(ladder);
    let climbed = travel > 0.0;
    if !climbed {
        return Point { x: ladder.foot.x, y: ladder.foot.y };
    }
    if travel >= length {
        return Point { x: ladder.top.x, y: ladder.top.y };
    }
    let share = travel / length;
    Point { x: ladder.foot.x + (ladder.top.x - ladder.foot.x) * share, y: ladder.foot.y + (ladder.top.y - ladder.foot.y) * share }
}

/// 🎠️ The phase 0…1 of the climbing clip after `travel` pixels along a ladder: one cycle per two rungs, so hands and feet meet the rungs.
pub fn ladder_phase(travel: f64) -> f64 {
    fraction(travel / (2.0 * RUNG_SPACING))
}

/// 🏖️ Where a climber of `size` stands once it has stepped off the top of a ladder: on the rim the ladder leans against, half a width and `MANTLE_INSET` inside it.
pub fn ladder_landing(ladder: &LadderStand, size: Size) -> Point {
    Point { x: ladder.top.x - ladder.side.sign() * (size.width / 2.0 + MANTLE_INSET), y: ladder.top.y - LADDER_TUCK }
}

/// 🎳️ What a toppling ladder does to a climber whose feet are at height `y`: `None` when it is less than `TOPPLE_STEP` heights above the foot and steps off, else the velocity that throws it clear of the wall.
pub fn spill_of(ladder: &LadderStand, y: f64, size: Size) -> Option<Toss> {
    if ladder.foot.y - y < TOPPLE_STEP * size.height {
        None
    } else {
        Some(Toss { vx: ladder.side.sign() * TOPPLE_PUSH, vy: 0.0 })
    }
}
//#endregion 🔖️Ladders

//#region 🔖️Rope
/// 🔦️ The shot from `feet` at the point `x` of the edge of `perch`, or `None`: the actor faces the point, the muzzle is `MUZZLE_FORWARD` widths in front of the feet and `MUZZLE_HEIGHT` heights above them, the hook bites `HOOK_LIFT` above the edge; the rope must be `ROPE_SHORT`…`ROPE_LONG` heights long, rise at least `ROPE_ELEVATION` per pixel of its length and miss every keep-out grown by `ROPE_MARGIN`, except what lies under the hook. A rope no more slanted than `ZIP_SLANT` is reeled in straight, any other swings.
fn aimed(feet: Point, perch: &Perch, x: f64, keepouts: &[Rect], size: Size) -> Option<Shot> {
    let facing = if x < feet.x { Facing::Left } else { Facing::Right };
    let muzzle = Point { x: feet.x + facing.sign() * MUZZLE_FORWARD * size.width, y: feet.y - MUZZLE_HEIGHT * size.height };
    let hook = Point { x, y: perch.y - HOOK_LIFT };
    let across = (hook.x - muzzle.x).abs();
    let rise = muzzle.y - hook.y;
    let length = span(muzzle, hook);
    let under = Point { x, y: perch.y };
    let reaches = ROPE_SHORT * size.height <= length && length <= ROPE_LONG * size.height && rise >= ROPE_ELEVATION * length;
    (reaches && sighted(muzzle, hook, keepouts, ROPE_MARGIN, under, under)).then(|| Shot { surface: perch.surface.clone(), facing, muzzle, hook, length, reel: if across <= ZIP_SLANT * rise { Reeling::Zip } else { Reeling::Swing } })
}

/// 🎇️ The best shot of the grappling gun from `feet`, or `None` when no edge is in reach: of every perch at least `ROPE_RISE` heights above the feet the two ends, each half a width and `HOOK_INSET` inside, and the point between them nearest to the feet (the middle of a perch too narrow for that) are tried, and the shot with the least `length + ROPE_DETOUR × sideways distance` wins, the first one among equals.
pub fn shot_for(feet: Point, perches: &[Perch], keepouts: &[Rect], size: Size) -> Option<Shot> {
    let inset = size.width / 2.0 + HOOK_INSET;
    let mut best = None;
    let mut least = f64::INFINITY;
    for perch in perches {
        let raised = feet.y - perch.y >= ROPE_RISE * size.height;
        if !raised {
            continue;
        }
        let low = perch.x0 + inset;
        let high = perch.x1 - inset;
        let (spots, count) = if low <= high { ([low, high, clamp(feet.x, low, high)], 3) } else { ([(perch.x0 + perch.x1) / 2.0, 0.0, 0.0], 1) };
        for &spot in &spots[..count] {
            let Some(shot) = aimed(feet, perch, spot, keepouts, size) else {
                continue;
            };
            let cost = shot.length + ROPE_DETOUR * (shot.hook.x - shot.muzzle.x).abs();
            if cost < least {
                best = Some(shot);
                least = cost;
            }
        }
    }
    best
}

/// 🧿️ The shot as it holds after a survey, or `None` when the hook lost its edge: a perch of its surface must still carry the hook's x no more than `ROPE_FOLLOW` above or below, and the line from the muzzle to the hook on that edge must still be clear; the hook follows the edge, the rope takes its new length.
pub fn shot_holds(shot: &Shot, perches: &[Perch], keepouts: &[Rect]) -> Option<Shot> {
    for perch in perches {
        let carries = perch.x0 <= shot.hook.x && shot.hook.x <= perch.x1;
        if perch.surface != shot.surface || !carries || (perch.y - HOOK_LIFT - shot.hook.y).abs() > ROPE_FOLLOW {
            continue;
        }
        let hook = Point { x: shot.hook.x, y: perch.y - HOOK_LIFT };
        let under = Point { x: hook.x, y: perch.y };
        if sighted(shot.muzzle, hook, keepouts, ROPE_MARGIN, under, under) {
            return Some(Shot { surface: shot.surface.clone(), facing: shot.facing, muzzle: shot.muzzle, hook, length: span(shot.muzzle, hook), reel: shot.reel });
        }
    }
    None
}

/// 🫥️ Where a shot that is meant to miss is aimed: `ROPE_MISS_OVERSHOOT` past the end of `perch` that is nearer to the hook, at the height of the hook — the hook flies there, finds nothing and is pulled back.
pub fn miss_of(shot: &Shot, perch: &Perch) -> Point {
    Point { x: if shot.hook.x - perch.x0 <= perch.x1 - shot.hook.x { perch.x0 - ROPE_MISS_OVERSHOOT } else { perch.x1 + ROPE_MISS_OVERSHOOT }, y: shot.hook.y }
}

/// 🕰️ How many ticks a hook flies from `from` to `to` at `speed` pixels per second, for a finite distance and a positive speed.
pub fn hook_ticks(from: Point, to: Point, speed: f64) -> Ticks {
    ceiling((span(from, to) * RATE) / speed) as Ticks
}

/// 🪃️ The hook `ticks` ticks after it left `from` for `to` on a straight line at `speed` pixels per second: `from` itself at 0 and before, `to` itself from [`hook_ticks`] on. The flight out is `hook_step(muzzle, hook, HOOK_SPEED, ticks)`, the way back after a miss `hook_step(tip, muzzle, HOOK_RETURN, ticks)`.
pub fn hook_step(from: Point, to: Point, speed: f64, ticks: Ticks) -> Point {
    let length = span(from, to);
    if ticks <= 0 {
        return Point { x: from.x, y: from.y };
    }
    if (ticks as f64) >= ceiling((length * RATE) / speed) {
        return Point { x: to.x, y: to.y };
    }
    let share = (speed * ticks as f64) / RATE / length;
    Point { x: from.x + (to.x - from.x) * share, y: from.y + (to.y - from.y) * share }
}

/// 🐟️ The actor whose hands hold the rope of `shot` at `hand` with `rope` pixels left to the hook: its feet hang under the hands as they stood under the muzzle.
fn hung(shot: &Shot, rope: f64, hand: Point, before: Point, size: Size) -> Haul {
    Haul { rope, hand, before, x: hand.x - shot.facing.sign() * MUZZLE_FORWARD * size.width, y: hand.y + MUZZLE_HEIGHT * size.height }
}

/// 🐠️ The point of the taut line of `shot` that lies `rope` pixels from the hook towards the muzzle.
fn lined(shot: &Shot, rope: f64) -> Point {
    let share = rope / shot.length;
    Point { x: shot.hook.x + (shot.muzzle.x - shot.hook.x) * share, y: shot.hook.y + (shot.muzzle.y - shot.hook.y) * share }
}

/// 🎏️ The actor at rest on the taut line of `shot` with `rope` pixels of rope left: with the whole length of the shot it is where it stood when the hook bit, which is how every haul begins.
pub fn haul_of(shot: &Shot, rope: f64, size: Size) -> Haul {
    let hand = lined(shot, rope);
    hung(shot, rope, hand, hand, size)
}

/// 🚠️ One tick of reeling straight up the rope of `shot`, `ticks` ticks after the haul began: the rope shortens by `ZIP_SPEED ÷ 64`, gathered over `ZIP_RAMP` ticks, but not below `REEL_LEAST` heights, where the hoist onto the perch begins; a rope that is shorter already keeps its length. The hands stay on the taut line.
pub fn zip_step(shot: &Shot, haul: Haul, ticks: Ticks, size: Size) -> Haul {
    let rope = larger(haul.rope - gathered(ZIP_SPEED, ticks, ZIP_RAMP) / RATE, smaller(REEL_LEAST * size.height, haul.rope));
    hung(shot, rope, lined(shot, rope), haul.hand, size)
}

/// 🎪️ One tick of swinging on the rope of `shot` while it is reeled in, `ticks` ticks after the haul began: the hands are the pendulum of the swing module's `reel_step` under the hook, down to a rope of `REEL_LEAST` heights.
pub fn sway_step(shot: &Shot, haul: Haul, ticks: Ticks, size: Size) -> Haul {
    let reel = reel_step(shot.hook, haul.hand, haul.before, haul.rope, REEL_LEAST * size.height, ticks);
    hung(shot, reel.length, reel.bob, haul.hand, size)
}

/// 🎬️ One tick of the haul up the rope of `shot`, `ticks` ticks after it began: [`zip_step`] on a rope that is reeled in straight, [`sway_step`] on one that swings.
pub fn haul_step(shot: &Shot, haul: Haul, ticks: Ticks, size: Size) -> Haul {
    match shot.reel {
        Reeling::Zip => zip_step(shot, haul, ticks, size),
        Reeling::Swing => sway_step(shot, haul, ticks, size),
    }
}

/// 🧭️ How many ticks the haul up the whole rope of `shot` takes until `REEL_LEAST` heights of rope are left; 0 for a rope no longer than that.
pub fn haul_ticks(shot: &Shot, size: Size) -> Ticks {
    let least = REEL_LEAST * size.height;
    let mut haul = haul_of(shot, shot.length, size);
    let mut ticks: Ticks = 0;
    while haul.rope > least && ticks < PATIENCE {
        haul = haul_step(shot, haul, ticks, size);
        ticks += 1;
    }
    ticks
}

/// 🏕️ Where an actor of `size` stands once it has hoisted itself up the rope of `shot` onto `perch`: half a width and `MANTLE_INSET` from the hook towards the middle of the perch, never beyond its ends.
pub fn landing_for(shot: &Shot, perch: &Perch, size: Size) -> Point {
    let inward = if shot.hook.x <= (perch.x0 + perch.x1) / 2.0 { 1.0 } else { -1.0 };
    Point { x: clamp(shot.hook.x + inward * (size.width / 2.0 + MANTLE_INSET), perch.x0, perch.x1), y: perch.y }
}
//#endregion 🔖️Rope

//#region 🔖️Routes
/// 🛤️ Whether a ladder stands on `perch`: on its surface, at its height, with its foot between its ends.
fn rests(ladder: &LadderStand, perch: &Perch) -> bool {
    perch.surface == ladder.surface && perch.y == ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1
}

/// 🚏️ The way over a standing ladder from `from` to `to`, or `None`: up when it stands on `from` and leans against a pitch that `to` crowns, down the other way round.
fn ridden<'a>(ladder: &'a LadderStand, from: &Perch, to: &Perch, pitches: &[Pitch], size: Size) -> Option<Leg<'a>> {
    for pitch in pitches {
        if pitch.wall != ladder.wall || pitch.side != ladder.side {
            continue;
        }
        if rests(ladder, from) && crowns(to, pitch, size) {
            return Some(Leg::Ladder { at: ladder.foot.x, ladder, up: true });
        }
        if crowns(from, pitch, size) && rests(ladder, to) {
            return Some(Leg::Ladder { at: ledge_of(pitch, size), ladder, up: false });
        }
    }
    None
}

/// 🧱️ The way over the wall line of `pitch` from `from` to `to`, or `None`: from a hold at the foot of `pitch` up the line to the rim of the nearest pitch on the way that `to` crowns, or over the rim of `pitch` that `from` crowns down the line to the nearest pitch on the way where `to` passes its foot — when `grip` lasts for the whole way ([`wall_path`], [`wall_cost`]). On a single pitch that is the climb and the mantle or the hang at its rim.
fn scaled<'a>(pitch: &'a Pitch, from: &Perch, to: &Perch, size: Size, grip: f64, pitches: &'a [Pitch]) -> Option<Leg<'a>> {
    let hold = grip_for(from, pitch, size)?;
    let chain = chain_of(pitch, pitches, size);
    let start = chain.iter().position(|member| std::ptr::eq(*member, pitch))?;
    if hold.over {
        for (at, &exit) in chain.iter().enumerate().skip(start) {
            let Some(landing) = grip_for(to, exit, size) else {
                continue;
            };
            if landing.over {
                continue;
            }
            return (wall_cost(&wall_path(&chain, start, Entry::Hang, hold.x, hold.y, at, landing.y, false, size)) <= grip).then_some(Leg::Wall { at: ledge_of(pitch, size), pitch, hold, exit, goal: landing.y });
        }
        return None;
    }
    for (at, &exit) in chain.iter().enumerate().take(start + 1).rev() {
        if !crowns(to, exit, size) {
            continue;
        }
        let rim = rim_of(exit, size);
        return (wall_cost(&wall_path(&chain, start, Entry::Grab, hold.x, hold.y, at, rim, true, size)) <= grip).then_some(Leg::Wall { at: hold.x, pitch, hold, exit, goal: rim });
    }
    None
}

/// 🗾️ The ways an actor of `size` that stands at `x` on `from` can take to `to` with its `gear` and its `grip`, most preferred first, or `None` when there is none: a ladder that stands between the two (for whoever owns any gear but a parachute), a wall line (gear `climb`; one leg per pitch it takes hold of), a ladder of its own raised against a wall `to` crowns (gear `ladder`, and only where none stands), a shot of its grappling gun at the edge of `to` (gear `grapple`) from where it stands or from the point of its perch nearest to an end or to the middle of `to`. Every leg is clear of the keep-outs; whether it is clear of the other actors is the stage's to judge.
#[allow(clippy::too_many_arguments)]
pub fn route_of<'a>(x: f64, from: &Perch, to: &Perch, gear: &[Gear], size: Size, grip: f64, pitches: &'a [Pitch], ladders: &'a [LadderStand], keepouts: &[Rect]) -> Option<Vec<Leg<'a>>> {
    let mut legs = Vec::new();
    if gear.contains(&Gear::Climb) || gear.contains(&Gear::Ladder) || gear.contains(&Gear::Grapple) {
        legs.extend(ladders.iter().filter_map(|ladder| ridden(ladder, from, to, pitches, size)));
    }
    let joined = !legs.is_empty();
    if gear.contains(&Gear::Climb) {
        legs.extend(pitches.iter().filter_map(|pitch| scaled(pitch, from, to, size, grip, pitches)));
    }
    if gear.contains(&Gear::Ladder) && !joined {
        legs.extend(pitches.iter().filter_map(|pitch| ladder_for(from, to, pitch, keepouts, size)).map(|ladder| Leg::Raise { at: ladder.foot.x, ladder }));
    }
    if gear.contains(&Gear::Grapple) {
        let stands = [x, clamp(to.x0, from.x0, from.x1), clamp(to.x1, from.x0, from.x1), clamp((to.x0 + to.x1) / 2.0, from.x0, from.x1)];
        if let Some(leg) = stands.into_iter().find_map(|stand| shot_for(Point { x: stand, y: from.y }, std::slice::from_ref(to), keepouts, size).map(|shot| Leg::Grapple { at: stand, shot })) {
            legs.push(leg);
        }
    }
    (!legs.is_empty()).then_some(legs)
}
//#endregion 🔖️Routes

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
