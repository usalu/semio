//! 🪢️ Everything that hangs: a pet dangling from the learner's hand, a pet under its parachute and a pet on a rope that is reeled in — one constrained step for all three, the spring that carries the grip, the throw at the release, and the descent under a canopy.
//!
//! Positions are stage pixels with the y axis pointing down, velocities are pixels per second and time is whole ticks
//! of 1/64 s. Everything is built from `+ − × ÷`, `sqrt` and comparisons (and `atan_turns`, `sin_turns`,
//! `spring_step`, which are built from the same), evaluated in the order of the TypeScript twin with literal constants
//! and never fused (`mul_add`), so both cores yield the same bits.
//!
//! The core is [`swing_step`]: one Verlet step of a point on a rod or a rope below an anchor that may move. The
//! constraint force acts along the radius of the tick before and its size is the smaller root of a quadratic (SHAKE),
//! which makes the step second order and symplectic: a free swing keeps its amplitude for minutes and follows the
//! pendulum's differential equation closely. The usual shortcut — step freely, then pull the point back onto the
//! circle — is only first order and eats a quarter of the energy of a swing in its first half period at this tick
//! length; it is deliberately absent. A swing dies by the `damping` factor alone, which is what makes it tunable.
//!
//! A held pet, per tick: `hang_step(hang, pointer, length)`, then `lean_of(grip, hang.bob, length)` for the tilt of
//! its drawing; at the release `throw_velocity(hang, samples)` with the last pointer positions, one per tick. A falling
//! pet, per tick: `chute_opens(vy, height)`; [`CHUTE_REFLEX`] ticks of plain falling later `canopy_of(feet, vx, vy,
//! chute)`, then `chute_step(canopy, chute, target, remaining, wind)` until it lands. A pet on a rope: `reel_step`.
//!
//! @see <https://en.wikipedia.org/wiki/Constraint_(computational_chemistry)#The_SHAKE_algorithm> — SHAKE (Ryckaert, Ciccotti and Berendsen 1977), the constraint step of [`swing_step`]
//! @see <https://theorangeduck.com/page/spring-roll-call#critical> — the half-life form of a critically damped spring, behind the follow spring
//! @see <https://android.googlesource.com/platform/frameworks/native/+/master/libs/input/VelocityTracker.cpp> — the least-squares release velocity of a touch, the model of [`ring_velocity`]
//! @see ../🎞️animation/🦀️.rs — `spring_step`
//! @see ../🏞️terrain/🦀️.rs — `GRAVITY`, `FALL_SPEED`
//! @see ../📐️trigonometry/🦀️.rs — `atan_turns`, `sin_turns`
//! @see ../../🧬️schema/🦀️.rs — `Point`, `Grip`, `Hang`, `Canopy`, `Ticks`, `Turns`
//! @see ../🪢️swing/🟦️.ts — the TypeScript twin

use crate::animation::spring_step;
use crate::schema::{Canopy, Grip, Hang, Point, Ticks, Turns};
use crate::terrain::{FALL_SPEED, GRAVITY};
use crate::trigonometry::{atan_turns, sin_turns};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 📏️ The rod of a held pet as a share of its height: the feet hang 0.8 heights below the grip (38 px for a pet of 48).
pub const HANG_ROD: f64 = 0.8;

/// 🧲️ The gravity a held pet swings under, in pixels per second squared: four times `GRAVITY`, because the hand that drags it accelerates many times harder than a fall; an ordinary drag then leans it by 15° to 50° instead of pinning it to the cone (a flick still reaches it), and a swing on a rod of 38 px takes 0.46 s.
pub const HANG_GRAVITY: f64 = 7200.0;

/// 🍯️ The share of its speed a held pet keeps per tick: its swing halves every 13.5 ticks and has died below 3° after about two seconds.
pub const HANG_DAMPING: f64 = 0.95;

/// 🍦️ The cosine of the widest lean of a held pet: its feet stay at least half a rod below the grip, so it never leans beyond 60°.
pub const HANG_CONE: f64 = 0.5;

/// 🧷️ The stiffness of the spring that carries the grip towards the pointer, in 1/s²: with [`FOLLOW_DAMPING`] a critically damped spring of a half-life of 0.06 s. A resting grip has covered half of a jump of its target after 4 ticks and never overshoots; behind a pointer in steady motion it trails by 70 ms of travel.
pub const FOLLOW_STIFFNESS: f64 = 534.0;

/// 🛋️ The damping of the spring that carries the grip, in 1/s, the companion of [`FOLLOW_STIFFNESS`].
pub const FOLLOW_DAMPING: f64 = 46.0;

/// ⚖️ The weights of the last seven pointer samples, oldest first, whose sum divided by [`RELEASE_DIVISOR`] is the slope at the newest sample of the parabola fitted to all seven by least squares, in pixels per tick. They add up to 0.
pub const RELEASE_WEIGHTS: [f64; 7] = [7.0, -2.0, -7.0, -8.0, -5.0, 2.0, 13.0];

/// ➗️ What the weighted sum of [`RELEASE_WEIGHTS`] is divided by.
pub const RELEASE_DIVISOR: f64 = 28.0;

/// 🥶️ How many equal samples at the end mean the pointer has stopped (47 ms): its velocity then counts as zero.
pub const RELEASE_STALE: usize = 3;

/// 🤝️ How much of the pointer's velocity the grip has not caught up with yet is handed to a thrown pet on top of its own.
pub const THROW_SHARE: f64 = 0.5;

/// 🐌️ The slowest throw in pixels per second; below it a release is a plain letting go, without velocity.
pub const THROW_LEAST: f64 = 70.0;

/// 🚀️ The fastest throw in pixels per second; a faster release keeps its direction and is slowed to this. A throw straight up at this speed rises 114 px.
pub const THROW_MOST: f64 = 640.0;

/// 🎈️ The fastest a throw may rise, in pixels per second; the upward part beyond it is cut.
pub const THROW_RISE: f64 = 520.0;

/// 💥️ The impact speed in pixels per second above which a landing is hard: reached from rest after a fall of 100 px, about two body heights. A parachute opens for it.
pub const HARD_LANDING: f64 = 600.0;

/// 🚪️ The least falling speed in pixels per second at which a parachute opens: reached after 9 ticks (20 px) of falling from rest, so a pet that merely steps off a low perch never flashes a canopy.
pub const CHUTE_OPENING: f64 = 240.0;

/// 🪟️ The least height above the landing in pixels a parachute needs to be of any use.
pub const CHUTE_HEADROOM: f64 = 56.0;

/// ⏱️ How many ticks a pet keeps falling after it decided to open its parachute — it looks up and tugs the cord. The most sensitive number of the descent (MECH §2.2): every tick of it is a tick of falling at the full gravity.
pub const CHUTE_REFLEX: Ticks = 6;

/// 📉️ The share of the gap to its terminal speed a canopy keeps per tick: `exp(−1 ÷ (64 × 0.18))`, a time constant of 0.18 s.
pub const CHUTE_FACTOR: f64 = 0.9168553557320289;

/// 🍂️ The terminal speed under a canopy in pixels per second per pixel of body height: 96 px/s for a pet of 48.
pub const CHUTE_DESCENT: f64 = 2.0;

/// 🧭️ How hard a canopy steers towards its landing spot: the sideways speed it wants per pixel of distance, in 1/s.
pub const CHUTE_STEER_GAIN: f64 = 1.2;

/// ⛵️ The fastest a canopy drifts sideways on purpose, in pixels per second per pixel of body height.
pub const CHUTE_STEER_SPEED: f64 = 1.2;

/// 🛞️ The share of the gap to the wanted sideways speed a canopy closes per tick.
pub const CHUTE_STEER_EASE: f64 = 0.06;

/// 🧵️ The cords of a parachute as a share of the body height: the pet hangs 0.9 heights below the canopy.
pub const CHUTE_ROD: f64 = 0.9;

/// 🪶️ The gravity a pet sways under below its canopy, in pixels per second squared: half of `GRAVITY`, a sway of 1.4 s.
pub const CHUTE_GRAVITY: f64 = 900.0;

/// 🌫️ The share of its speed a pet under a canopy keeps per tick; it acts on the speed over the stage, so a canopy that drifts sideways drags its pet behind it, by 9° at its fastest drift.
pub const CHUTE_DAMPING: f64 = 0.97;

/// 🛬️ The height above the landing, as a share of the body height, below which a canopy flares: the descent slows evenly to half of its speed at the touch.
pub const CHUTE_FLARE: f64 = 0.25;

/// 💨️ The strongest sideways push of the wind on a canopy in pixels per second.
pub const CHUTE_WIND: f64 = 10.0;

/// 🎐️ How fast the wind swings back and forth, in turns per second.
pub const CHUTE_WIND_RATE: f64 = 0.35;

/// 🎣️ How fast a swinging pet reels its rope in, in pixels per second, once it has spun up.
pub const REEL_SPEED: f64 = 60.0;

/// 📈️ Over how many ticks the reel spins up to [`REEL_SPEED`].
pub const REEL_RAMP: Ticks = 10;

/// 🤏️ The shortest rope a pet swings on, as a share of its height; reeling stops there, because a pendulum that keeps shortening spins up without bound.
pub const REEL_LEAST: f64 = 0.9;

/// 🚦️ The fastest a pet moves on a reeled rope, in pixels per second: 8.125 px per tick.
pub const REEL_CAP: f64 = 520.0;

/// 🕰️ The share of its speed a pet on a rope keeps per tick: a swing halves in three seconds.
pub const REEL_DAMPING: f64 = 0.9965;

const TAUT: f64 = 1.000001;
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🎒️ The measures of a species' parachute in pixels and pixels per second: its terminal speed, the fastest sideways drift it steers with, the height it flares at and the length of its cords.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chute {
    pub terminal: f64,
    pub reach: f64,
    pub flare: f64,
    pub length: f64,
}

/// 🧶️ A pet on a rope after one tick of reeling: where it is and how long the rope is now.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reel {
    pub bob: Point,
    pub length: f64,
}
//#endregion 🔖️Types

//#region 🔖️Swing
/// ⛓️ Where a point that hangs on a rod or a rope of `length` is one tick later, while its anchor moves from `anchor_before` to `anchor_now`; `bob` is where the point is and `previous` where it was a tick ago.
///
/// The point first flies freely: it keeps `damping` of its step and falls by `gravity ÷ 4096` (pixels per second
/// squared times the square of a tick). Then it is pulled back along the radius it had before the step, from
/// `anchor_before` to `bob`, by the smaller root of the quadratic that puts it at `length` from `anchor_now` — so the
/// result lies exactly that far from the anchor whenever such a point exists, and as near to it as that line comes
/// otherwise (an anchor that jumped sideways by more than the rod). A slack rope (`rope` set and the free point
/// within `length` of the anchor) pulls nothing; a rod always does. Because the pull is central, shortening `length`
/// from tick to tick keeps the angular momentum about the anchor, like a pendulum on a shortened string. The damping
/// acts on the step of the tick before, half a tick earlier than a drag would: the swing is that of a pendulum with
/// the drag `128·(1 − damping) ÷ (1 + damping)` per second under a gravity `2 ÷ (1 + damping)` times as strong.
#[allow(clippy::too_many_arguments)]
pub fn swing_step(anchor_before: Point, anchor_now: Point, bob: Point, previous: Point, length: f64, gravity: f64, damping: f64, rope: bool) -> Point {
    let free_x = bob.x + (bob.x - previous.x) * damping;
    let free_y = bob.y + (bob.y - previous.y) * damping + gravity / 4096.0;
    let radius_x = bob.x - anchor_before.x;
    let radius_y = bob.y - anchor_before.y;
    let reach_x = free_x - anchor_now.x;
    let reach_y = free_y - anchor_now.y;
    let radius = radius_x * radius_x + radius_y * radius_y;
    let along = reach_x * radius_x + reach_y * radius_y;
    let reach = reach_x * reach_x + reach_y * reach_y;
    if rope && reach <= length * length {
        return Point { x: free_x, y: free_y };
    }
    let root = along * along - radius * (reach - length * length);
    let pull = (along - (if root > 0.0 { root } else { 0.0 }).sqrt()) / (if radius > 1e-9 { radius } else { 1e-9 });
    Point { x: free_x - pull * radius_x, y: free_y - pull * radius_y }
}
//#endregion 🔖️Swing

//#region 🔖️Hand
/// 🔻️ The feet of a held pet kept on their rod and inside the cone below the grip: a point whose direction from `anchor` leans beyond [`HANG_CONE`] is put on the edge of the cone on its own side at `length`, a point farther than `length` is drawn in along its direction, and every other point — one on the grip itself too — is returned as it is.
pub fn cone_clamp(anchor: Point, bob: Point, length: f64) -> Point {
    let dx = bob.x - anchor.x;
    let dy = bob.y - anchor.y;
    let span = dx * dx + dy * dy;
    if dy < 0.0 || dy * dy < HANG_CONE * HANG_CONE * span {
        let drop = HANG_CONE * length;
        let side = (length * length - drop * drop).sqrt();
        return Point { x: if dx < 0.0 { anchor.x - side } else { anchor.x + side }, y: anchor.y + drop };
    }
    if span <= length * length * TAUT {
        return bob;
    }
    let taut = length / span.sqrt();
    Point { x: anchor.x + dx * taut, y: anchor.y + dy * taut }
}

/// 👣️ The grip one tick later on its way to `target` (the pointer, held inside the stage): `spring_step` on each axis with [`FOLLOW_STIFFNESS`] and [`FOLLOW_DAMPING`].
pub fn follow_step(grip: Grip, target: Point) -> Grip {
    let x = spring_step(grip.x, grip.vx, target.x, FOLLOW_STIFFNESS, FOLLOW_DAMPING);
    let y = spring_step(grip.y, grip.vy, target.y, FOLLOW_STIFFNESS, FOLLOW_DAMPING);
    Grip { x: x.position, y: y.position, vx: x.velocity, vy: y.velocity }
}

/// 🪝️ A pet at the moment it is picked up: gripped `length` above its feet, everything at rest. The grip then travels to the pointer through [`follow_step`], which is the lift.
pub fn hang_of(feet: Point, length: f64) -> Hang {
    Hang { grip: Grip { x: feet.x, y: feet.y - length, vx: 0.0, vy: 0.0 }, bob: feet, previous: feet }
}

/// 🦧️ A held pet one tick later: the grip follows `target`, the feet swing below it on a rod of `length` under [`HANG_GRAVITY`] and [`HANG_DAMPING`], and [`cone_clamp`] keeps them on the rod and inside the cone.
pub fn hang_step(hang: Hang, target: Point, length: f64) -> Hang {
    let grip = follow_step(hang.grip, target);
    let held = Point { x: grip.x, y: grip.y };
    let swung = swing_step(Point { x: hang.grip.x, y: hang.grip.y }, held, hang.bob, hang.previous, length, HANG_GRAVITY, HANG_DAMPING, false);
    Hang { grip, bob: cone_clamp(held, swung, length), previous: hang.bob }
}

/// 🗼️ The tilt of a body that hangs from `anchor` with its feet at `bob`, in turns: the rotation that carries a body hanging straight down onto it, in the sense of the rig (positive turns the x axis towards the y axis, clockwise on a screen). 0 straight down, negative while the feet trail to the right, positive to the left; off by at most 2e-6 turns.
pub fn lean_of(anchor: Point, bob: Point, length: f64) -> Turns {
    atan_turns((anchor.x - bob.x) / length, (bob.y - anchor.y) / length)
}
//#endregion 🔖️Hand

//#region 🔖️Release
/// 🧊️ Whether the last [`RELEASE_STALE`] samples are one point: the pointer has stopped. Fewer samples have not stopped.
fn stale(samples: &[Point]) -> bool {
    let count = samples.len();
    if count < RELEASE_STALE {
        return false;
    }
    let newest = samples[count - 1];
    (2..=RELEASE_STALE).all(|back| samples[count - back].x == newest.x && samples[count - back].y == newest.y)
}

/// 💍️ The velocity of the pointer at its newest sample in pixels per second, from its positions at the last seven ticks, oldest first: the slope of the parabola fitted to them by least squares, which is the sum of [`RELEASE_WEIGHTS`] times the samples, divided by [`RELEASE_DIVISOR`], times 64.
///
/// Every sample is taken relative to the newest one before it is weighted (the weights add up to 0, so the fit is
/// the same), which makes a pointer at rest yield exactly zero wherever it rests. When fewer than seven samples are
/// given the oldest one stands in for the missing ones — the pointer is taken to have rested there — and no sample
/// at all is no velocity. Samples before the last seven are ignored.
pub fn ring_velocity(samples: &[Point]) -> Point {
    let Some(&newest) = samples.last() else {
        return Point { x: 0.0, y: 0.0 };
    };
    let mut x = 0.0;
    let mut y = 0.0;
    for (index, &weight) in RELEASE_WEIGHTS.iter().take(RELEASE_WEIGHTS.len() - 1).enumerate() {
        let sample = samples[(samples.len() + index).saturating_sub(RELEASE_WEIGHTS.len())];
        x += weight * (sample.x - newest.x);
        y += weight * (sample.y - newest.y);
    }
    Point { x: (x * 64.0) / RELEASE_DIVISOR, y: (y * 64.0) / RELEASE_DIVISOR }
}

/// 🤾️ The throw a velocity at the release becomes, in pixels per second: nothing below [`THROW_LEAST`], slowed to [`THROW_MOST`] along its direction above it, and then never rising faster than [`THROW_RISE`].
pub fn throw_of(velocity: Point) -> Point {
    let speed = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
    if speed < THROW_LEAST {
        return Point { x: 0.0, y: 0.0 };
    }
    let scale = if speed > THROW_MOST { THROW_MOST / speed } else { 1.0 };
    let y = velocity.y * scale;
    Point { x: velocity.x * scale, y: if y < 0.0 - THROW_RISE { 0.0 - THROW_RISE } else { y } }
}

/// 🖐️ The throw of the pointer alone at the release, from its last samples, one per tick, oldest first: [`throw_of`] of [`ring_velocity`], and nothing when the pointer had stopped (its last [`RELEASE_STALE`] samples are equal).
pub fn release_velocity(samples: &[Point]) -> Point {
    if stale(samples) {
        Point { x: 0.0, y: 0.0 }
    } else {
        throw_of(ring_velocity(samples))
    }
}

/// 🥏️ The throw of a held pet at the release: the velocity of its feet (their last step times 64) plus [`THROW_SHARE`] of what the pointer does and the grip has not caught up with yet (the pointer's velocity, zero once it stopped, less the grip's), through [`throw_of`].
pub fn throw_velocity(hang: Hang, samples: &[Point]) -> Point {
    let ring = if stale(samples) { Point { x: 0.0, y: 0.0 } } else { ring_velocity(samples) };
    throw_of(Point { x: (hang.bob.x - hang.previous.x) * 64.0 + THROW_SHARE * (ring.x - hang.grip.vx), y: (hang.bob.y - hang.previous.y) * 64.0 + THROW_SHARE * (ring.y - hang.grip.vy) })
}
//#endregion 🔖️Release

//#region 🔖️Parachute
/// ☄️ The speed in pixels per second at which something that moves at `vy` now would hit a landing `height` pixels below without a parachute: `sqrt(vy² + 2 × GRAVITY × height)`, never beyond `FALL_SPEED`; a landing that is not below counts as reached.
pub fn impact_speed(vy: f64, height: f64) -> f64 {
    let speed = (vy * vy + 2.0 * GRAVITY * (if height > 0.0 { height } else { 0.0 })).sqrt();
    if speed < FALL_SPEED {
        speed
    } else {
        FALL_SPEED
    }
}

/// 🪂️ Whether a pet that falls at `vy` with a landing `height` pixels below its feet opens its parachute now: it falls at [`CHUTE_OPENING`] or faster, has [`CHUTE_HEADROOM`] or more below it, and would land harder than [`HARD_LANDING`]. From rest that is every drop of 102 px or more.
pub fn chute_opens(vy: f64, height: f64) -> bool {
    vy >= CHUTE_OPENING && height >= CHUTE_HEADROOM && impact_speed(vy, height) > HARD_LANDING
}

/// 🧮️ The measures of the parachute of a species of `height` pixels: [`CHUTE_DESCENT`], [`CHUTE_STEER_SPEED`], [`CHUTE_FLARE`] and [`CHUTE_ROD`] times that height.
pub fn chute_of(height: f64) -> Chute {
    Chute { terminal: CHUTE_DESCENT * height, reach: CHUTE_STEER_SPEED * height, flare: CHUTE_FLARE * height, length: CHUTE_ROD * height }
}

/// 🌂️ A parachute at the tick it opens above a pet whose feet are at `feet` and move at `vx`, `vy`: the canopy holds the cords their length above the feet and moves as the pet does, so nothing sways yet.
pub fn canopy_of(feet: Point, vx: f64, vy: f64, chute: Chute) -> Canopy {
    Canopy { x: feet.x, y: feet.y - chute.length, vx, vy, bob: feet, previous: Point { x: feet.x - vx / 64.0, y: feet.y - vy / 64.0 } }
}

/// 🦅️ The share of its descent speed a canopy keeps `remaining` pixels above the landing: all of it at `flare` and above, half of it at the touch and below, evenly in between.
pub fn flare_of(remaining: f64, flare: f64) -> f64 {
    if remaining >= flare {
        return 1.0;
    }
    if remaining <= 0.0 {
        return 0.5;
    }
    0.5 + (0.5 * remaining) / flare
}

/// 🌬️ The sideways push of the wind on a canopy in pixels per second, `ticks` after the stage began: [`CHUTE_WIND`] times the sine of [`CHUTE_WIND_RATE`] turns per second, shifted by `phase` turns so every pet has a wind of its own.
pub fn chute_wind(ticks: Ticks, phase: Turns) -> f64 {
    CHUTE_WIND * sin_turns((CHUTE_WIND_RATE * ticks as f64) / 64.0 + phase)
}

/// 🕊️ A pet under its open parachute one tick later; `target` is the x it steers for, `remaining` the height of its feet above the landing before the step, and `wind` the push of [`chute_wind`].
///
/// The descent closes the gap to the terminal speed by [`CHUTE_FACTOR`] (`vy ← terminal + (vy − terminal) ×
/// factor`, the exact step of a linear drag, stable for every speed). The sideways speed closes
/// [`CHUTE_STEER_EASE`] of the gap to the speed it wants, [`CHUTE_STEER_GAIN`] times the distance to the target held
/// within the reach of the chute. The canopy then moves by `vx + wind` and by `vy` times [`flare_of`], both divided by
/// 64, and the pet swings below it by [`swing_step`] on the cords as a rod, under [`CHUTE_GRAVITY`] and
/// [`CHUTE_DAMPING`]. The speed of the touch is the last step of the canopy times 64.
pub fn chute_step(canopy: Canopy, chute: Chute, target: f64, remaining: f64, wind: f64) -> Canopy {
    let vy = chute.terminal + (canopy.vy - chute.terminal) * CHUTE_FACTOR;
    let pull = CHUTE_STEER_GAIN * (target - canopy.x);
    let wanted = if pull < 0.0 - chute.reach {
        0.0 - chute.reach
    } else if pull > chute.reach {
        chute.reach
    } else {
        pull
    };
    let vx = canopy.vx + (wanted - canopy.vx) * CHUTE_STEER_EASE;
    let x = canopy.x + (vx + wind) / 64.0;
    let y = canopy.y + (vy * flare_of(remaining, chute.flare)) / 64.0;
    let bob = swing_step(Point { x: canopy.x, y: canopy.y }, Point { x, y }, canopy.bob, canopy.previous, chute.length, CHUTE_GRAVITY, CHUTE_DAMPING, false);
    Canopy { x, y, vx, vy, bob, previous: canopy.bob }
}
//#endregion 🔖️Parachute

//#region 🔖️Reel
/// 🪀️ A pet that swings on a rope hooked at `anchor` while it reels the rope in, one tick later; `age` counts the ticks reeled before this one and `least` is the shortest rope it swings on ([`REEL_LEAST`] times its height).
///
/// The rope shortens by the reel speed of the tick — [`REEL_SPEED`] once [`REEL_RAMP`] ticks have passed, the share
/// `(age + 1) ÷ REEL_RAMP` of it before — but never below `least`, and a rope that is shorter than that already keeps
/// its length. The pet then swings by [`swing_step`] on the shortened rope under `GRAVITY` and [`REEL_DAMPING`]; a
/// step longer than [`REEL_CAP`] ÷ 64 pixels is cut to that length along its direction and, where that leaves the pet
/// beyond the rope, drawn in to the rope's length, so a step is never longer than the cap plus what the reel takes
/// in. A pet reeled from 154 px at 85° reaches 722 px/s without the cap and 520 px/s with it; without the least length
/// the shortening pendulum spins up without bound (MECH §3.3).
pub fn reel_step(anchor: Point, bob: Point, previous: Point, length: f64, least: f64, age: Ticks) -> Reel {
    let speed = if age + 1 < REEL_RAMP { (REEL_SPEED * (age + 1) as f64) / REEL_RAMP as f64 } else { REEL_SPEED };
    let floor = if length < least { length } else { least };
    let wound = length - speed / 64.0;
    let rope = if wound < floor { floor } else { wound };
    let swung = swing_step(anchor, anchor, bob, previous, rope, GRAVITY, REEL_DAMPING, true);
    let dx = swung.x - bob.x;
    let dy = swung.y - bob.y;
    let step = dx * dx + dy * dy;
    let cap = REEL_CAP / 64.0;
    if step <= cap * cap {
        return Reel { bob: swung, length: rope };
    }
    let slowed = cap / step.sqrt();
    let x = bob.x + dx * slowed;
    let y = bob.y + dy * slowed;
    let span = (x - anchor.x) * (x - anchor.x) + (y - anchor.y) * (y - anchor.y);
    if span <= rope * rope {
        return Reel { bob: Point { x, y }, length: rope };
    }
    let taut = rope / span.sqrt();
    Reel { bob: Point { x: anchor.x + (x - anchor.x) * taut, y: anchor.y + (y - anchor.y) * taut }, length: rope }
}
//#endregion 🔖️Reel

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
