//! 🏞️ Where pets stand, walk, fall and hop: perches cut out of the surveyed surfaces, strides along them, the swept
//! one-way landing and ballistic flight in ticks (design §4.5).
//!
//! Positions are viewport pixels with the y axis pointing down, velocities are pixels per second and time is whole
//! ticks of 1/64 s. Everything is built from `+ − × ÷`, `sqrt`, `abs`, `floor`, the larger and the smaller of two
//! numbers and comparisons, evaluated in the order of the TypeScript twin and never fused (`mul_add`), so both cores
//! yield the same bits. The larger and the smaller of two numbers follow JavaScript's `Math.max` and `Math.min`, not
//! `f64::max` and `f64::min`: a NaN operand yields NaN and +0 counts as larger than −0.
//!
//! Flight is integrated per tick by semi-implicit Euler, velocity first: `vy ← min(vy + GRAVITY ÷ 64, FALL_SPEED)`,
//! then `y ← y + vy ÷ 64` and `x ← x + vx ÷ 64`. Below the terminal speed the height after `k` ticks is therefore
//! `y + vy·k ÷ 64 + GRAVITY·k·(k + 1) ÷ 8192`. [`hop_of`] solves this sum, not the continuous arc, for the launch
//! velocity, so a hop it grants ends on its target after exactly its ticks, and [`hop_step`] writes the target itself
//! on the last tick, which removes the rounding the sum has gathered on the way.
//!
//! A hop in flight, per tick: `hop_step(x, y, vx, vy, to, left)` with the ticks left to fly, counting the one being
//! stepped (`hop.ticks` on the first tick, 1 on the last), then `landing_of(perches, next.x, y, next.y)`; a landing is
//! never found while the actor rises, because perches are one-way platforms. [`hop_landing`] runs exactly this loop
//! ahead of time.
//!
//! @see ../../🧬️schema/🦀️.rs — `Point`, `Rect`, `Surface`, `Perch`, `Ticks`, `TICKS_PER_SECOND`
//! @see ../🏞️terrain/🟦️.ts — the TypeScript twin

use crate::schema::{Perch, Point, Rect, Surface, Ticks, TICKS_PER_SECOND};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🍎️ The downward acceleration of everything in flight, in pixels per second squared: a drop of 100 px takes a third of a second.
pub const GRAVITY: f64 = 1800.0;

/// 🪨️ The terminal speed of a fall in pixels per second, reached after half a second or 225 px.
pub const FALL_SPEED: f64 = 900.0;

/// 🌈️ How far the apex of a hop rises above the higher of its two ends at least, in pixels.
pub const HOP_CLEARANCE: f64 = 12.0;

/// 📐️ The least apex rise of a hop per pixel of horizontal distance: 3/8 launches a level hop at about 56°.
pub const HOP_STEEPNESS: f64 = 0.375;

/// 🏔️ The highest apex of a hop above its launch in pixels; a perch more than `HOP_HEIGHT − HOP_CLEARANCE` above is too high.
pub const HOP_HEIGHT: f64 = 84.0;

/// 📏️ The widest hop in pixels of horizontal distance.
pub const HOP_DISTANCE: f64 = 160.0;

/// ⏳️ The longest hop in ticks (0.75 s).
pub const HOP_TICKS: Ticks = 48;

const RATE: f64 = TICKS_PER_SECOND as f64;
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🪂️ A height and a vertical speed: what one tick of falling yields.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fall {
    pub y: f64,
    pub vy: f64,
}

/// 🦘️ The launch of a hop: its velocity in pixels per second and the ticks it stays in the air.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hop {
    pub vx: f64,
    pub vy: f64,
    pub ticks: Ticks,
}

/// 🕊️ An actor in the air: where its feet are and how fast they move, in pixels and pixels per second.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flight {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

type Stretch = (f64, f64);
//#endregion 🔖️Types

//#region 🔖️Extremes
/// 🔼️ The larger of two numbers as JavaScript's `Math.max` answers it: NaN when either is NaN, and +0 rather than −0.
pub(crate) fn larger(left: f64, right: f64) -> f64 {
    if left.is_nan() || right.is_nan() {
        f64::NAN
    } else if left > right || (left == right && left.is_sign_positive()) {
        left
    } else {
        right
    }
}

/// 🔽️ The smaller of two numbers as JavaScript's `Math.min` answers it: NaN when either is NaN, and −0 rather than +0.
pub(crate) fn smaller(left: f64, right: f64) -> f64 {
    if left.is_nan() || right.is_nan() {
        f64::NAN
    } else if left < right || (left == right && left.is_sign_negative()) {
        left
    } else {
        right
    }
}
//#endregion 🔖️Extremes

//#region 🔖️Perches
/// ⛔️ Whether `keepout` reaches into the headroom band `[top, bottom)` above a surface; a box without area blocks nothing.
fn blocks(keepout: &Rect, top: f64, bottom: f64) -> bool {
    keepout.width > 0.0 && larger(top, keepout.y) < smaller(bottom, keepout.y + keepout.height)
}

/// ✂️ `stretches` without the open interval `(low, high)`, still in ascending order; what only touches an end takes nothing away.
fn cut(stretches: &[Stretch], low: f64, high: f64) -> Vec<Stretch> {
    let mut kept = Vec::with_capacity(stretches.len() + 1);
    for &(x0, x1) in stretches {
        if high <= x0 || low >= x1 {
            kept.push((x0, x1));
        } else {
            if low > x0 {
                kept.push((x0, low));
            }
            if high < x1 {
                kept.push((high, x1));
            }
        }
    }
    kept
}

/// 🪺️ The free stretches of every surface with `clearance ≤ y ≤ height`: its extent clipped to `[0, width]`, minus the x-extent of every keep-out that reaches into the band `[y − clearance, y)` above it (keep-outs in list order), keeping stretches at least `minimum` wide; in surface order, then ascending x.
pub fn perches_of(surfaces: &[Surface], keepouts: &[Rect], width: f64, height: f64, clearance: f64, minimum: f64) -> Vec<Perch> {
    let mut perches = Vec::new();
    for surface in surfaces {
        if surface.y < clearance || surface.y > height {
            continue;
        }
        let left = larger(surface.x0, 0.0);
        let right = smaller(surface.x1, width);
        if right <= left {
            continue;
        }
        let mut stretches = vec![(left, right)];
        for keepout in keepouts {
            if blocks(keepout, surface.y - clearance, surface.y) {
                stretches = cut(&stretches, keepout.x, keepout.x + keepout.width);
            }
        }
        for (x0, x1) in stretches {
            if x1 - x0 >= minimum {
                perches.push(Perch { surface: surface.id.clone(), x0, x1, y: surface.y });
            }
        }
    }
    perches
}

/// 📍️ The first perch of `surface` that carries `x`, both ends included; `None` ≙ TypeScript `null`.
pub fn perch_at<'a>(perches: &'a [Perch], surface: &str, x: f64) -> Option<&'a Perch> {
    perches.iter().find(|perch| perch.surface == surface && perch.x0 <= x && x <= perch.x1)
}

/// 🧲️ The perch whose nearest point is closest to `(x, y)` by squared distance, the first one among equals; `None` without perches.
pub fn nearest_perch(perches: &[Perch], x: f64, y: f64) -> Option<&Perch> {
    let mut nearest = None;
    let mut least = f64::INFINITY;
    for perch in perches {
        let dx = larger(larger(perch.x0 - x, 0.0), x - perch.x1);
        let dy = perch.y - y;
        let distance = dx * dx + dy * dy;
        if distance < least {
            nearest = Some(perch);
            least = distance;
        }
    }
    nearest
}
//#endregion 🔖️Perches

//#region 🔖️Walking
/// 👣️ The next x one tick later on the way to `goal` at `speed` pixels per second: a step of `max(speed, 0) ÷ 64`, or the goal itself when it is no farther than that.
pub fn stride_to(x: f64, goal: f64, speed: f64) -> f64 {
    let step = larger(speed, 0.0) / RATE;
    let gap = goal - x;
    if gap > step {
        x + step
    } else if gap < 0.0 - step {
        x - step
    } else {
        goal
    }
}
//#endregion 🔖️Walking

//#region 🔖️Falling
/// ⬇️ One tick of falling, velocity first: `vy ← min(vy + GRAVITY ÷ 64, FALL_SPEED)`, then `y ← y + vy ÷ 64`.
pub fn fall_step(y: f64, vy: f64) -> Fall {
    let speed = smaller(vy + GRAVITY / RATE, FALL_SPEED);
    Fall { y: y + speed / RATE, vy: speed }
}

/// 🛬️ The highest perch crossed at `x` on the way down from `from_y` to `to_y`, both heights and both ends of the perch included, the first one among equals; `None` when none is crossed, as always while rising (`to_y < from_y`).
pub fn landing_of(perches: &[Perch], x: f64, from_y: f64, to_y: f64) -> Option<&Perch> {
    let mut landing: Option<&Perch> = None;
    for perch in perches {
        if perch.x0 <= x && x <= perch.x1 && from_y <= perch.y && perch.y <= to_y && landing.is_none_or(|held| perch.y < held.y) {
            landing = Some(perch);
        }
    }
    landing
}
//#endregion 🔖️Falling

//#region 🔖️Hopping
/// 🚀️ The ballistic hop from `from` to `to`, or `None` when out of reach.
///
/// 1. `rise = max(HOP_CLEARANCE − min(dy, 0), |dx| × HOP_STEEPNESS)` is the apex above the launch, so the apex clears
///    both ends; too high when it exceeds `HOP_HEIGHT`, too far when `|dx|` exceeds `HOP_DISTANCE`.
/// 2. `ticks = floor((sqrt(2 × rise ÷ GRAVITY) + sqrt(2 × (rise + dy) ÷ GRAVITY)) × 64 + 0.5)` is the time of that
///    arc up and down, rounded to whole ticks; too long when it exceeds `HOP_TICKS`.
/// 3. `vy = (dy − GRAVITY × ticks × (ticks + 1) ÷ 8192) × 64 ÷ ticks` makes the per-tick sum end on `to.y`; a landing
///    faster than `FALL_SPEED` is a fall, not a hop. `vx = dx × 64 ÷ ticks`.
///
/// Every limit is tested as "within", so a point that is not a finite number is out of reach as well.
pub fn hop_of(from: Point, to: Point) -> Option<Hop> {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let rise = larger(HOP_CLEARANCE - smaller(dy, 0.0), dx.abs() * HOP_STEEPNESS);
    let reachable = rise <= HOP_HEIGHT && dx.abs() <= HOP_DISTANCE;
    if !reachable {
        return None;
    }
    let seconds = ((2.0 * rise) / GRAVITY).sqrt() + ((2.0 * (rise + dy)) / GRAVITY).sqrt();
    let ticks = (seconds * RATE + 0.5).floor();
    let brief = ticks <= HOP_TICKS as f64;
    if !brief {
        return None;
    }
    let vy = ((dy - (GRAVITY * ticks * (ticks + 1.0)) / (2.0 * RATE * RATE)) * RATE) / ticks;
    let gentle = vy + ticks * (GRAVITY / RATE) <= FALL_SPEED;
    if !gentle {
        return None;
    }
    Some(Hop { vx: (dx * RATE) / ticks, vy, ticks: ticks as Ticks })
}

/// 🎈️ One tick of a hop towards `to` with `ticks` left to fly, counting this one: the fall step for the height and `x ← x + vx ÷ 64`, and on the last tick (`ticks ≤ 1`) the target itself.
pub fn hop_step(x: f64, y: f64, vx: f64, vy: f64, to: Point, ticks: Ticks) -> Flight {
    let fallen = fall_step(y, vy);
    if ticks > 1 {
        Flight { x: x + vx / RATE, y: fallen.y, vx, vy: fallen.vy }
    } else {
        Flight { x: to.x, y: to.y, vx, vy: fallen.vy }
    }
}

/// 🎯️ The perch on which `hop` from `from` towards `to` really ends: the first landing of its per-tick flight, which is another perch than the target's when one lies in the way down; `None` when the flight lands nowhere.
pub fn hop_landing(perches: &[Perch], from: Point, to: Point, hop: Hop) -> Option<&Perch> {
    let mut flight = Flight { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
    for left in (1..=hop.ticks).rev() {
        let next = hop_step(flight.x, flight.y, flight.vx, flight.vy, to, left);
        let landing = landing_of(perches, next.x, flight.y, next.y);
        if landing.is_some() {
            return landing;
        }
        flight = next;
    }
    None
}
//#endregion 🔖️Hopping

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
