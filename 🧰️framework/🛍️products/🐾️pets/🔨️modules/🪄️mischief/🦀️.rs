//! 🪄️ Mischief of the pets with things of the page, as pure arithmetic — the Rust twin of the mischief module: which marked elements (fixtures) fit a species, which one is picked, when a pet may start, where it works, how the lifted copy of the fixture moves, and how the pet is thrown off when the learner takes the element back.
//!
//! Nothing here moves a real element. The page marks what a pet may play with by a key in the vocabulary of the
//! menagerie's `grounds` (`<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`); the survey reports those as fixtures;
//! the render target shows a copy of the fixture and moves the copy by what [`lift_at`] says.
//!
//! **The choice never reveals an answer** (rule 7 of the second round). Which fixture is pushed depends on two things
//! only: whether a ground of the species equals the fixture's key or is a prefix of it at a `/` ([`fits`]), and a
//! number in [0, 1) the stage draws ([`chosen_fixture`]). [`fixture_for`] reads nothing of an item but its key
//! ([`Keyed`]) and [`chosen_fixture`] reads nothing of its candidates at all — the types say so.
//!
//! Every expression is the twin's, term for term and in its order; `Math.max`/`Math.min` are the terrain's
//! `larger`/`smaller`, `0 − x` stays `0.0 − x`, and a side or a facing is [`Facing`] (`1` right, `−1` left). The
//! twin's `null` is `None`; ticks are whole `i64`.
//!
//! @see ../🪄️mischief/🟦️.ts — the TypeScript twin
//! @see ../../🧬️schema/🦀️.rs — `Fixture`, `Perch`, `Pitch` (the free stretch of a wall), `PetMode`, `Point`, `Facing`, `Footing`
//! @see ../🏞️terrain/🦀️.rs — `larger`, `smaller`
//! @see ../🧗️climbing/🦀️.rs — `Toss`, a velocity with which an actor is thrown into the air
//! @see ../📐️trigonometry/🦀️.rs — `sin_turns`, `smoothstep`, `clamp`

use crate::climbing::Toss;
use crate::schema::{Facing, Fixture, Footing, Perch, PetMode, Pitch, Point, Ticks, Turns};
use crate::terrain::{larger, smaller};
use crate::trigonometry::{clamp, sin_turns, smoothstep};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🖥️ The least width of the stage in pixels at which mischief happens: narrower pages have no room beside their cards.
pub const MISCHIEF_WIDTH: f64 = 1024.0;

/// 🧘️ The ticks the learner must have been still before a pet starts mischief: 12 seconds.
pub const MISCHIEF_PATIENCE: Ticks = 768;

/// 🤫️ The ticks the learner must have been still in a time of concentration: 30 seconds.
pub const MISCHIEF_PATIENCE_QUIET: Ticks = 1920;

/// 🐢️ The ticks between the end of one lift and the start of the next on a calm stage: 3 minutes.
pub const MISCHIEF_COOLDOWN_CALM: Ticks = 11520;

/// 🐇️ The ticks between the end of one lift and the start of the next on a lively stage: 45 seconds.
pub const MISCHIEF_COOLDOWN_LIVELY: Ticks = 2880;

/// 📏️ How many pixels may lie between a fixture's end and the wall or the perch its pusher works from.
pub const STATION_GAP: f64 = 24.0;

/// 🪡️ How many pixels a fixture's end may stick out beyond the wall beside it and still count as inside.
pub const STATION_SLACK: f64 = 2.0;

/// 🪜️ How many pixels below a fixture's lower edge a perch may lie and still be at its height.
pub const STATION_STEP: f64 = 12.0;

/// 🚪️ The least room in pixels beyond a fixture's far end that is worth a shove.
pub const LIFT_ROOM: f64 = 12.0;

/// 💪️ The ticks the pusher braces before the copy moves; the copy appears during them.
pub const LIFT_BRACE: Ticks = 24;

/// 👉️ The ticks of the shove out of the stack.
pub const LIFT_SHOVE: Ticks = 40;

/// 🫨️ The ticks the copy wobbles where the shove left it.
pub const LIFT_WOBBLE: Ticks = 48;

/// 🧍️ The ticks the copy rests outside its stack, motionless: 8 seconds.
pub const LIFT_HOLD: Ticks = 512;

/// 👈️ The ticks of putting the copy back.
pub const LIFT_RETURN: Ticks = 56;

/// 🌫️ The ticks over which the copy appears at the start of a lift and vanishes at its end, each time lying exactly on the element.
pub const LIFT_FADE: Ticks = 8;

/// ↩️ The ticks after the start of a lift at which putting back begins.
pub const LIFT_RETURNS: Ticks = LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE + LIFT_HOLD;

/// ⏱️ The ticks a whole lift lasts, from the first trace of the copy to its last: 10.75 seconds.
pub const LIFT_TICKS: Ticks = LIFT_RETURNS + LIFT_RETURN + LIFT_FADE;

/// ⛔️ The ticks no lift may exceed: 20 seconds.
pub const LIFT_LIMIT: Ticks = 1280;

/// 🤏️ The share of the granted room every shove travels at least; the stage's draw decides the rest.
pub const LIFT_LEAST: f64 = 0.6;

/// 🧲️ How many pixels the copy gives towards the pusher while it braces, and rocks to and fro after the shove.
pub const LIFT_GIVE: f64 = 1.5;

/// 🎈️ The most pixels the copy ever leaves its row upwards or downwards, and the most an end of it rises when it tilts.
pub const LIFT_RISE: f64 = 2.0;

/// 📐️ The most the copy ever tilts, in turns (1.44°); a wide copy tilts less, so that its ends rise by [`LIFT_RISE`] at most.
pub const LIFT_TILT: f64 = 0.004;

/// 🔔️ How many times the copy rocks to and fro while it wobbles.
pub const LIFT_WOBBLES: f64 = 2.0;

/// 🥧️ The radians of half a turn (`3.141592653589793`, the twin's literal): turns the rise of a copy's end into its tilt.
pub const HALF_TURN_RADIANS: f64 = std::f64::consts::PI;

/// 🏌️ The least sideways speed in pixels per second with which a pusher is thrown off.
pub const THROW_SPEED: f64 = 120.0;

/// 🌬️ How much faster than [`THROW_SPEED`] the stage's draw can make the throw, in pixels per second.
pub const THROW_SPREAD: f64 = 80.0;

/// 🚀️ The upward speed in pixels per second with which a pusher is thrown off.
pub const THROW_LIFT: f64 = 220.0;
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🏷️ Anything that carries a key in the vocabulary of the grounds — the one thing of an item the choice may read.
pub trait Keyed {
    /// 🔑️ The key of the item.
    fn key(&self) -> &str;
}

impl Keyed for Fixture {
    /// 🗝️ The key of a fixture among the grounds of the menagerie.
    fn key(&self) -> &str {
        &self.key
    }
}

/// 🚦️ Everything the gates of mischief look at: whether the learner permits it, whether the pointer is fine, the width of the stage, its mode, whether it is a time of concentration, whether a lift is in progress, the tick now, the tick of the learner's last input and the tick the last lift ended.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Circumstances {
    pub permitted: bool,
    pub fine: bool,
    pub width: f64,
    pub mode: PetMode,
    pub quiet: bool,
    pub lifting: bool,
    pub tick: Ticks,
    pub stirred: Ticks,
    pub rested: Ticks,
}

/// 🏗️ Where a pusher works: on a perch or on a wall (`footing` is [`Footing::Perch`] or [`Footing::Wall`]; then `wall` names it), of which surface, the line it works at (`x`: the wall, or the point of the perch nearest to the fixture), the height of its feet, the way it shoves (`side`: right or left; it stands on the other side and faces this way) and the room beyond the fixture's far end.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Station {
    pub footing: Footing,
    pub wall: Option<String>,
    pub surface: String,
    pub x: f64,
    pub y: f64,
    pub side: Facing,
    pub room: f64,
}

/// 🪞️ How the copy of a lifted fixture lies relative to the element: shifted by `dx` and `dy` pixels, tilted by `tilt` about its centre, drawn with `opacity`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lift {
    pub dx: f64,
    pub dy: f64,
    pub tilt: Turns,
    pub opacity: f64,
}

/// 🛌️ No lift: the copy lies on its element and is not drawn.
pub const NO_LIFT: Lift = Lift { dx: 0.0, dy: 0.0, tilt: 0.0, opacity: 0.0 };
//#endregion 🔖️Types

//#region 🔖️Choice
/// 🧩️ Whether a ground covers a key: the two are equal, or the ground is a prefix of the key that ends where a `/` of the key begins — `heating` covers `heating/u-values`, never `heating-load`; an empty ground covers nothing.
pub fn fits(ground: &str, key: &str) -> bool {
    if ground.is_empty() || !key.starts_with(ground) {
        return false;
    }
    key.len() == ground.len() || key.as_bytes()[ground.len()] == b'/'
}

/// 🗂️ The fixtures a species may play with, in the survey's order: those whose key one of the species' grounds covers ([`fits`]). Only the key of an item is read, once.
pub fn fixture_for<'a, Item: Keyed>(grounds: &[String], fixtures: &'a [Item]) -> Vec<&'a Item> {
    fixtures
        .iter()
        .filter(|fixture| {
            let key = fixture.key();
            grounds.iter().any(|ground| fits(ground, key))
        })
        .collect()
}

/// 🎯️ The fixture a pet picks among those that fit it: candidate `⌊unit × count⌋` held inside the list (a unit that is not a number picks the first), every candidate as likely as any other; `None` without candidates. The pick has two inputs and no others — the candidates, which the topic alone selected, and a number in [0, 1) the stage drew — and reads no property of any candidate, so it cannot follow a value, a correctness flag or an answer (rule 7).
pub fn chosen_fixture<Item>(candidates: &[Item], unit: f64) -> Option<&Item> {
    let count = candidates.len();
    if count == 0 {
        return None;
    }
    let place = (unit * count as f64).floor();
    candidates.get(if place >= count as f64 {
        count - 1
    } else if place > 0.0 {
        place as usize
    } else {
        0
    })
}
//#endregion 🔖️Choice

//#region 🔖️Gates
/// ⏳️ The ticks the learner must have been still: [`MISCHIEF_PATIENCE`], or [`MISCHIEF_PATIENCE_QUIET`] in a time of concentration.
pub fn patience_of(quiet: bool) -> Ticks {
    if quiet {
        MISCHIEF_PATIENCE_QUIET
    } else {
        MISCHIEF_PATIENCE
    }
}

/// 🧊️ The ticks a mode leaves between two lifts; `None` for a still stage, where there is no mischief at all.
pub fn cooldown_of(mode: PetMode) -> Option<Ticks> {
    match mode {
        PetMode::Calm => Some(MISCHIEF_COOLDOWN_CALM),
        PetMode::Lively => Some(MISCHIEF_COOLDOWN_LIVELY),
        PetMode::Still => None,
    }
}

/// 🕰️ The first tick at which the gates of time are open — the learner has been still long enough and the mode's cooldown since the last lift has passed —, or `None` while another gate is shut: no consent, a coarse pointer, a narrow or still stage, or a lift in progress (whose end sets `rested` anew).
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn allowed_from(circumstances: Circumstances) -> Option<Ticks> {
    let cooldown = cooldown_of(circumstances.mode)?;
    if !circumstances.permitted || !circumstances.fine || circumstances.lifting || !(circumstances.width >= MISCHIEF_WIDTH) {
        return None;
    }
    Some((circumstances.stirred + patience_of(circumstances.quiet)).max(circumstances.rested + cooldown))
}

/// 🚥️ Whether a pet may start mischief now: the learner permits it, the pointer is fine, the stage is at least [`MISCHIEF_WIDTH`] wide and not still, no lift is in progress, the learner has not stirred for [`patience_of`] ticks and the last lift ended [`cooldown_of`] ticks ago or earlier.
pub fn allowed(circumstances: Circumstances) -> bool {
    allowed_from(circumstances).is_some_and(|from| circumstances.tick >= from)
}
//#endregion 🔖️Gates

//#region 🔖️Station
/// 🏆️ The station with more room of the two, the earlier one when they have the same; a station without [`LIFT_ROOM`] never counts.
fn roomier(best: Option<Station>, next: Station) -> Option<Station> {
    if next.room >= LIFT_ROOM && best.as_ref().is_none_or(|best| next.room > best.room) {
        Some(next)
    } else {
        best
    }
}

/// 🧭️ Where a pusher works on a fixture, or `None` when nothing is beside it: a perch at the fixture's height (above its upper edge by nothing, below its lower edge by [`STATION_STEP`] at most) that ends within [`STATION_GAP`] of one of its ends, or a free stretch of a wall that runs beside one of its ends (the fixture's end within [`STATION_GAP`] inside the wall, the stretch overlapping the fixture's height; the feet at the fixture's lower edge, held inside the stretch).
///
/// A pusher on the left shoves to the right and the other way round; the room of a station is what lies between the
/// fixture's far end and the edge of the stage (`width`). The station with the most room wins — the side with room —,
/// a perch before a wall and then the survey's order when rooms are equal. Pass only the pitches the species can
/// climb: none for a species without that gear.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn station_for(fixture: &Fixture, pitches: &[Pitch], perches: &[Perch], width: f64) -> Option<Station> {
    let left = fixture.x;
    let right = fixture.x + fixture.width;
    let top = fixture.y;
    let bottom = fixture.y + fixture.height;
    let mut best = None;
    for perch in perches {
        if !(perch.y > top && perch.y <= bottom + STATION_STEP) {
            continue;
        }
        if perch.x0 < left && left - smaller(perch.x1, left) <= STATION_GAP {
            best = roomier(best, Station { footing: Footing::Perch, wall: None, surface: perch.surface.clone(), x: smaller(perch.x1, left), y: perch.y, side: Facing::Right, room: width - right });
        }
        if perch.x1 > right && larger(perch.x0, right) - right <= STATION_GAP {
            best = roomier(best, Station { footing: Footing::Perch, wall: None, surface: perch.surface.clone(), x: larger(perch.x0, right), y: perch.y, side: Facing::Left, room: left });
        }
    }
    for pitch in pitches {
        if !(pitch.y0 < bottom && pitch.y1 > top) {
            continue;
        }
        let y = clamp(bottom, pitch.y0, pitch.y1);
        if pitch.side == Facing::Left && left - pitch.x >= 0.0 - STATION_SLACK && left - pitch.x <= STATION_GAP {
            best = roomier(best, Station { footing: Footing::Wall, wall: Some(pitch.wall.clone()), surface: pitch.surface.clone(), x: pitch.x, y, side: Facing::Right, room: width - right });
        }
        if pitch.side == Facing::Right && pitch.x - right >= 0.0 - STATION_SLACK && pitch.x - right <= STATION_GAP {
            best = roomier(best, Station { footing: Footing::Wall, wall: Some(pitch.wall.clone()), surface: pitch.surface.clone(), x: pitch.x, y, side: Facing::Left, room: left });
        }
    }
    best
}
//#endregion 🔖️Station

//#region 🔖️Lift
/// 🪃️ `value` on the side `side`: itself to the right, its opposite to the left, never a negative zero.
fn toward(side: Facing, value: f64) -> f64 {
    match side {
        Facing::Right => value,
        Facing::Left => 0.0 - value,
    }
}

/// 🎢️ The copy of a lifted fixture at `tick`, for a lift that began at `since`: the way it is shoved (`side`), the room granted to it in pixels (the station's room, at most the pusher's width), the width of the fixture (`span`) and a number in [0, 1) the stage drew.
///
/// The copy travels `room × (LIFT_LEAST + (1 − LIFT_LEAST) × unit)` along x and nowhere else to speak of:
/// 1. brace ([`LIFT_BRACE`] ticks): it appears over [`LIFT_FADE`] ticks, lying exactly on its element, then gives [`LIFT_GIVE`] px towards the pusher and back;
/// 2. shove ([`LIFT_SHOVE`]): it slides out with `smoothstep`, rising [`LIFT_RISE`] px and tipping on the way;
/// 3. wobble ([`LIFT_WOBBLE`]): it rocks [`LIFT_WOBBLES`] times, ever less, and comes to rest;
/// 4. hold ([`LIFT_HOLD`]): it rests, exactly `travel` away, level;
/// 5. put back ([`LIFT_RETURN`]): it slides home the way it came;
/// 6. it vanishes over [`LIFT_FADE`] ticks, lying exactly on its element.
///
/// `dy` never leaves ±[`LIFT_RISE`], `tilt` never ±[`LIFT_TILT`] (less for a wide fixture: its ends rise by
/// [`LIFT_RISE`] at most). Before `since` and from [`lift_ends`] on there is no lift: [`NO_LIFT`].
pub fn lift_at(since: Ticks, tick: Ticks, side: Facing, room: f64, span: f64, unit: f64) -> Lift {
    let age = tick - since;
    if !(0..LIFT_TICKS).contains(&age) {
        return NO_LIFT;
    }
    let travel = larger(0.0, room) * (LIFT_LEAST + (1.0 - LIFT_LEAST) * unit);
    let lean = if span > 0.0 { smaller(LIFT_TILT, LIFT_RISE / (HALF_TURN_RADIANS * span)) } else { LIFT_TILT };
    if age < LIFT_FADE {
        return Lift { dx: 0.0, dy: 0.0, tilt: 0.0, opacity: smoothstep(age as f64 / LIFT_FADE as f64) };
    }
    if age < LIFT_BRACE {
        return Lift { dx: toward(side, 0.0 - LIFT_GIVE * sin_turns((age - LIFT_FADE) as f64 / (LIFT_BRACE - LIFT_FADE) as f64 / 2.0)), dy: 0.0, tilt: 0.0, opacity: 1.0 };
    }
    if age < LIFT_BRACE + LIFT_SHOVE {
        let phase = (age - LIFT_BRACE) as f64 / LIFT_SHOVE as f64;
        let hump = sin_turns(phase / 2.0);
        return Lift { dx: toward(side, travel * smoothstep(phase)), dy: 0.0 - LIFT_RISE * hump, tilt: toward(side, lean * hump), opacity: 1.0 };
    }
    if age < LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE {
        let phase = (age - LIFT_BRACE - LIFT_SHOVE) as f64 / LIFT_WOBBLE as f64;
        let calm = 1.0 - smoothstep(phase);
        let rock = sin_turns(LIFT_WOBBLES * phase) * calm;
        return Lift { dx: toward(side, travel + LIFT_GIVE * rock), dy: 0.5 * LIFT_RISE * sin_turns(2.0 * LIFT_WOBBLES * phase) * calm, tilt: toward(side, 0.0 - lean * rock), opacity: 1.0 };
    }
    if age < LIFT_RETURNS {
        return Lift { dx: toward(side, travel), dy: 0.0, tilt: 0.0, opacity: 1.0 };
    }
    if age < LIFT_RETURNS + LIFT_RETURN {
        let phase = (age - LIFT_RETURNS) as f64 / LIFT_RETURN as f64;
        let hump = sin_turns(phase / 2.0);
        return Lift { dx: toward(side, travel * (1.0 - smoothstep(phase))), dy: 0.0 - LIFT_RISE * hump, tilt: toward(side, 0.0 - lean * hump), opacity: 1.0 };
    }
    Lift { dx: 0.0, dy: 0.0, tilt: 0.0, opacity: 1.0 - smoothstep((age - LIFT_RETURNS - LIFT_RETURN) as f64 / LIFT_FADE as f64) }
}

/// 🏁️ The first tick at which a lift that began at `since` is over: [`LIFT_TICKS`] later.
pub fn lift_ends(since: Ticks) -> Ticks {
    since + LIFT_TICKS
}

/// ⏰️ The next tick after `tick` at which a stage has to look at a lift that began at `since` again, or `None` when the lift is over: its start before it has begun, the next tick while the copy moves, the start of putting back while it rests (between the two the copy does not change).
pub fn lift_wake(since: Ticks, tick: Ticks) -> Option<Ticks> {
    let age = tick - since;
    if age >= LIFT_TICKS {
        return None;
    }
    if age < 0 {
        return Some(since);
    }
    Some(if (LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE..LIFT_RETURNS).contains(&age) { since + LIFT_RETURNS } else { tick + 1 })
}
//#endregion 🔖️Lift

//#region 🔖️Reclaim
/// 💨️ The velocity with which a pusher at `pusher` is thrown off when the learner takes the fixture back: away from the fixture's middle at [`THROW_SPEED`] plus up to [`THROW_SPREAD`] by the stage's draw, and upwards at [`THROW_LIFT`]; a pusher exactly at the middle goes right.
pub fn thrown_off(pusher: Point, fixture: &Fixture, unit: f64) -> Toss {
    let speed = THROW_SPEED + THROW_SPREAD * unit;
    Toss { vx: if pusher.x < fixture.x + fixture.width / 2.0 { 0.0 - speed } else { speed }, vy: 0.0 - THROW_LIFT }
}
//#endregion 🔖️Reclaim

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
