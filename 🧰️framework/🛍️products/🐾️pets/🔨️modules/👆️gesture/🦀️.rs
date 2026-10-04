//! 👆️ What the learner's hand does to a pet — the Rust twin of `🟦️.ts`: the press that becomes a click, a hold or a pick-up, the heat of repeated attention, and the three path gestures — circling round a pet, stroking over it, shaking it while it is held.
//!
//! Every recogniser is a small state value and a pure step: `(state, sample, tick, guards) → { state, cue }`. The
//! stage keeps the states (one press and one shake per stage, one hover and one warmth per actor) and steps them
//! once per tick with the sample-and-hold pointer; a tick is 1/64 s, positions are viewport pixels with the y axis
//! pointing down. Nothing here measures an angle or a velocity vector: circling is counted in quadrants with a
//! Schmitt trigger on both axes, stroking and shaking in reversals between running extremes, and every comparison
//! of lengths is a comparison of squares. Only `+ − × ÷`, `abs`, JavaScript's `Math.min`/`Math.max` (`smaller`/`larger`
//! of the terrain twin) and comparisons are used, in the order of the TypeScript twin, so both twins reproduce every
//! decision bit for bit. Ticks, signs and counters are whole numbers (`Ticks`, `i64`), every other number an `f64`.
//!
//! A press alone changes nothing (WCAG 2.5.2): it only arms. What it becomes is decided by what follows — a release
//! (click), time (hold), movement beyond the slop (pick-up), or a cancellation (undo).
//!
//! @see ./🟦️.ts — the TypeScript twin
//! @see ../../🧬️schema/🦀️.rs — `Press`, `Warmth`, `Circling`, `Stroking`, `Shaking`, `Hover`, `Cue`, `Tier`, `Pointer`
//! @see <https://www.w3.org/WAI/WCAG22/Understanding/pointer-cancellation.html>
//! @see <https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html>

use crate::schema::{Circling, Cue, Dragged, Hover, Point, Pointer, Press, PressPhase, Pressed, Rect, Released, Shaking, Stroking, Ticks, Tier, Warmth, TICKS_PER_SECOND};
use crate::terrain::{larger, smaller};
use serde::{Deserialize, Serialize};
use std::cmp::max;

//#region 🔖️Guards
/// 🛡️ What silences the recognisers for one step: the pointer lies over a `control` (an interactive element, or a button the pets did not take is down), the page `scrolled` since the last step, the stage is `quiet` (a time of concentration) or `still`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Guards {
    pub control: bool,
    pub scrolled: bool,
    pub quiet: bool,
    pub still: bool,
}

/// 🟢️ No guard is up.
pub const UNGUARDED: Guards = Guards { control: false, scrolled: false, quiet: false, still: false };

/// 📜️ How long circling and stroking stay deaf after the page scrolled under the pointer, in ticks (0.25 s).
pub const SCROLL_TICKS: Ticks = 16;
//#endregion 🔖️Guards

//#region 🔖️Press
/// 🖱️ How far a mouse or a pen may move after the press before the press is a pick-up, in pixels (the drag threshold of desktop systems).
pub const SLOP_FINE: f64 = 6.0;

/// 📱️ How far a finger may move after the press before the press is a pick-up, in pixels (touch slop plus the jitter of a finger).
pub const SLOP_COARSE: f64 = 10.0;

/// ⏳️ How long a press rests before it is a hold, in ticks (0.44 s, the long-press time of touch systems).
pub const HOLD_TICKS: Ticks = 28;

/// 💤️ No press.
pub const IDLE: Press = Press { phase: PressPhase::Idle, x: 0.0, y: 0.0, since: 0, slop: 0.0 };

/// 📥️ What happens to a press: the shell's `pressed`, `dragged`, `released` and `cancelled` events as they are, and the passing of a tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PressInput {
    Pressed(Pressed),
    Dragged(Dragged),
    Released(Released),
    Cancelled,
    Ticked,
}

/// 📣️ What a press turned out to be: a `click`, the beginning (`hold`) and the end (`unhold`) of a resting press, a pick-up (`lift`; the grip is the press's point), the release of a picked-up pet (`drop`; at the input's point), or an `abort` that undoes whatever the press had begun.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PressSignal {
    Click,
    Hold,
    Unhold,
    Lift,
    Drop,
    Abort,
}

/// 🪜️ One step of the press: the next state and what the step decided, if anything.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PressStep {
    pub state: Press,
    pub signal: Option<PressSignal>,
}

/// 📏️ The slop of a pointer type in pixels.
pub fn slop_of(pointer: Pointer) -> f64 {
    if pointer == Pointer::Touch {
        SLOP_COARSE
    } else {
        SLOP_FINE
    }
}

/// 🕹️ One step of the press machine.
///
/// - `pressed` arms and decides nothing; over a `control` it is not taken at all. A press that arrives while another is
///   still open aborts the open one first (a release was lost) and arms anew.
/// - Armed: `released` before [`HOLD_TICKS`] is a `click`; `dragged` to the slop or beyond is a `lift`; a tick
///   [`HOLD_TICKS`] after the press is a `hold`. A hold ends with `unhold` on release and becomes a `lift` when the
///   pointer leaves the slop after all.
/// - Lifted: `released` is a `drop`; movement is the stage's business (it reads the pointer).
/// - `cancelled` (Escape, pointer cancel, lost capture, blur, hidden, pause) aborts every open press; so does a `still`
///   stage, and a scroll aborts a press that has not lifted anything (the pet moved away under the pointer).
/// - `quiet` changes nothing: clicks and pick-ups work in a time of concentration.
pub fn press_step(press: Press, input: PressInput, tick: Ticks, guards: Guards) -> PressStep {
    let open = press.phase != PressPhase::Idle;
    if guards.still {
        return PressStep { state: IDLE, signal: open.then_some(PressSignal::Abort) };
    }
    if let PressInput::Pressed(pressed) = input {
        return PressStep { state: if guards.control { IDLE } else { Press { phase: PressPhase::Armed, x: pressed.x, y: pressed.y, since: tick, slop: slop_of(pressed.pointer) } }, signal: open.then_some(PressSignal::Abort) };
    }
    if !open {
        return PressStep { state: press, signal: None };
    }
    if input == PressInput::Cancelled {
        return PressStep { state: IDLE, signal: Some(PressSignal::Abort) };
    }
    if press.phase == PressPhase::Lifted {
        return if matches!(input, PressInput::Released(_)) { PressStep { state: IDLE, signal: Some(PressSignal::Drop) } } else { PressStep { state: press, signal: None } };
    }
    if guards.scrolled {
        return PressStep { state: IDLE, signal: Some(PressSignal::Abort) };
    }
    if let PressInput::Released(_) = input {
        return PressStep {
            state: IDLE,
            signal: if press.phase == PressPhase::Holding {
                Some(PressSignal::Unhold)
            } else if tick - press.since < HOLD_TICKS {
                Some(PressSignal::Click)
            } else {
                None
            },
        };
    }
    if let PressInput::Dragged(dragged) = input {
        let dx = dragged.x - press.x;
        let dy = dragged.y - press.y;
        return if dx * dx + dy * dy >= press.slop * press.slop {
            PressStep { state: Press { phase: PressPhase::Lifted, x: press.x, y: press.y, since: press.since, slop: press.slop }, signal: Some(PressSignal::Lift) }
        } else {
            PressStep { state: press, signal: None }
        };
    }
    if press.phase == PressPhase::Armed && tick - press.since >= HOLD_TICKS {
        return PressStep { state: Press { phase: PressPhase::Holding, x: press.x, y: press.y, since: press.since, slop: press.slop }, signal: Some(PressSignal::Hold) };
    }
    PressStep { state: press, signal: None }
}

/// ⏰️ The tick at which an armed press becomes a hold, so the stage stays awake for it; `None` when no press is armed.
pub fn press_due(press: Press) -> Option<Ticks> {
    (press.phase == PressPhase::Armed).then_some(press.since + HOLD_TICKS)
}
//#endregion 🔖️Press

//#region 🔖️Heat
/// 🔥️ The heat one click adds.
pub const HEAT_CLICK: f64 = 1.0;

/// 🫶️ The heat one tick of holding adds: 2 per second, so a hold from cold has had enough after about five seconds.
pub const HEAT_HOLD: f64 = 0.03125;

/// 💧️ The heat that leaks away per second.
pub const HEAT_LEAK: f64 = 0.5;

/// 🙋️ Up to this heat a click is answered with a hello.
pub const HEAT_HELLO: f64 = 1.0;

/// 🎩️ Up to this heat a click is answered with a trick; above it with a purr.
pub const HEAT_TRICK: f64 = 3.0;

/// 🛑️ From this heat on the pet has had enough.
pub const HEAT_ENOUGH: f64 = 7.0;

/// 🙈️ How long a pet that has had enough answers nothing but a glance, in ticks (8 s).
pub const ENOUGH_TICKS: Ticks = 512;

/// 🕊️ The heat a pet starts with again once it has forgiven: the next click is a trick, not a hello.
pub const HEAT_FORGIVEN: f64 = 2.0;

/// 💆️ The attention that warms a pet: a click, or one tick of a resting press.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Caress {
    Click,
    Hold,
}

/// 🧊️ An actor nobody has touched yet.
pub const COLD: Warmth = Warmth { heat: 0.0, since: 0, until: 0, tier: Tier::Hello, run: 0, tricks: 0 };

/// 📉️ What is left at `tick` of the `heat` measured at `since`: it leaks [`HEAT_LEAK`] per second and never falls below zero.
pub fn heat_at(heat: f64, since: Ticks, tick: Ticks) -> f64 {
    larger(heat - (max(tick - since, 0) as f64 * HEAT_LEAK) / TICKS_PER_SECOND as f64, 0.0)
}

/// 📈️ The heat right after a caress at `tick`: what is left of `heat` since `since`, plus [`HEAT_CLICK`] or [`HEAT_HOLD`].
pub fn heat_after(heat: f64, since: Ticks, tick: Ticks, caress: Caress) -> f64 {
    heat_at(heat, since, tick) + if caress == Caress::Hold { HEAT_HOLD } else { HEAT_CLICK }
}

/// 🏷️ The tier of a heat: `hello` up to [`HEAT_HELLO`], `trick` up to [`HEAT_TRICK`], `purr` below [`HEAT_ENOUGH`], `enough` from there on.
pub fn tier_of(heat: f64) -> Tier {
    if heat <= HEAT_HELLO {
        Tier::Hello
    } else if heat <= HEAT_TRICK {
        Tier::Trick
    } else if heat < HEAT_ENOUGH {
        Tier::Purr
    } else {
        Tier::Enough
    }
}

/// 💓️ The warmth after a caress at `tick`, with the answer in its `tier`, `run` and `tricks`.
///
/// A click is answered by the tier of the heat it leaves; a tick of holding is a purr until the heat says enough.
/// The caress that reaches [`HEAT_ENOUGH`] is answered with `enough` (run 1: the pet turns away); for
/// [`ENOUGH_TICKS`] from then on every caress is ignored and only counted (`enough`, run 2, 3, …: a glance), and
/// after that the pet is back at [`HEAT_FORGIVEN`] and leaks from there. Nothing is ever taken away for it.
pub fn warmth_after(warmth: Warmth, tick: Ticks, caress: Caress) -> Warmth {
    if tick < warmth.until {
        return Warmth { heat: warmth.heat, since: warmth.since, until: warmth.until, tier: Tier::Enough, run: warmth.run + 1, tricks: warmth.tricks };
    }
    let heat = heat_after(warmth.heat, warmth.since, tick, caress);
    let tier = if heat >= HEAT_ENOUGH {
        Tier::Enough
    } else if caress == Caress::Hold {
        Tier::Purr
    } else {
        tier_of(heat)
    };
    if tier == Tier::Enough {
        return Warmth { heat: HEAT_FORGIVEN, since: tick + ENOUGH_TICKS, until: tick + ENOUGH_TICKS, tier, run: 1, tricks: warmth.tricks };
    }
    Warmth { heat, since: tick, until: warmth.until, tier, run: if tier == warmth.tier { warmth.run + 1 } else { 1 }, tricks: if tier == Tier::Trick { warmth.tricks + 1 } else { warmth.tricks } }
}
//#endregion 🔖️Heat

//#region 🔖️Circling
/// 🍩️ How far outside half the body's larger side the band of a circle begins, in pixels: nearer to the centre the pointer is on the pet, not round it.
pub const CIRCLE_MARGIN: f64 = 8.0;

/// 🪐️ Where the band of a circle ends, in multiples of the body's larger side from its centre.
pub const CIRCLE_REACH: f64 = 3.4;

/// 🧲️ The Schmitt trigger of both axes, in pixels: a coordinate changes its sign only beyond this distance from the axis, so the tremor of a hand on an axis does not count quarter turns back and forth.
pub const CIRCLE_HYSTERESIS: f64 = 4.0;

/// ⭕️ The quarter turns of one circle: the fifth axis crossing is the one the lap began with, so the pointer has gone all the way round.
pub const CIRCLE_QUARTERS: i64 = 5;

/// 🏎️ The least ticks per quarter turn on average over a lap (4 turns per second): what is faster is a scribble.
pub const CIRCLE_FAST: Ticks = 4;

/// 🐌️ The most ticks between two quarter turns (0.4 turns per second): a lap that stalls longer starts anew.
pub const CIRCLE_SLOW: Ticks = 40;

/// 🥚️ How much farther from the centre the farthest point of a lap may be than its nearest: a lap is round, not a dash past the pet.
pub const CIRCLE_ROUND: f64 = 2.5;

/// 🔗️ How much the distances from the centre at the two ends of a lap may differ, as a factor: a circle comes back to where it began, a loop of aimless wiggling rarely does.
pub const CIRCLE_CLOSE: f64 = 1.375;

/// 🔙️ The quarter turns a lap may take against its direction.
pub const CIRCLE_AGAINST: i64 = 1;

/// 🚪️ How long the pointer may leave the band before the circle is forgotten, in ticks.
pub const CIRCLE_OUT_TICKS: Ticks = 8;

/// 🧘️ How long circling stays deaf after a circle, in ticks (2 s).
pub const CIRCLE_REST: Ticks = 128;

/// 🎯️ One step of circling: the next state and the completed circle (`circle` or `countercircle`), if any.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleStep {
    pub state: Circling,
    pub cue: Option<Cue>,
}

/// 🧹️ No circle in progress, deaf until the tick `rest` (the reset).
pub fn no_circling(rest: Ticks) -> Circling {
    Circling { live: false, inside: 0, sx: 0, sy: 0, px: 0.0, py: 0.0, turn: 0, quarters: 0, steps: 0, against: 0, first: 0, last: 0, open: 0.0, near: 0.0, far: 0.0, rest }
}

/// 🧭️ The quadrant of two signs in clockwise order as seen on screen: 0 top right, 1 bottom right, 2 bottom left, 3 top left.
fn quadrant_of(sx: i64, sy: i64) -> i64 {
    if sx > 0 {
        if sy < 0 {
            0
        } else {
            1
        }
    } else if sy > 0 {
        2
    } else {
        3
    }
}

/// 🔁️ One tick of circling round `body` with the pointer at `pointer`.
///
/// 1. Guards: over a control, in a quiet or still stage, for [`SCROLL_TICKS`] after a scroll and until `rest`
///    nothing is followed.
/// 2. Band: the pointer is followed while its distance from the body's centre lies between
///    `max(width, height) ÷ 2 + CIRCLE_MARGIN` and `max(width, height) × CIRCLE_REACH`; it may leave for
///    [`CIRCLE_OUT_TICKS`].
/// 3. Quarter turns: each offset coordinate keeps its sign until it is more than [`CIRCLE_HYSTERESIS`] beyond the
///    axis. The two signs name a quadrant; moving to the next quadrant clockwise is +1, to the previous −1, to the
///    opposite one ±2 by the sign of `previous × current` (the cross product), and nothing when that is zero.
/// 4. Lap: the first quarter turn sets the direction. A lap starts anew when more than [`CIRCLE_SLOW`] ticks
///    pass without a quarter turn, or with the quarter turn that would be more than [`CIRCLE_AGAINST`] against
///    its direction.
/// 5. Circle: at [`CIRCLE_QUARTERS`] net quarter turns the lap is a circle when it took at least
///    [`CIRCLE_FAST`] ticks per quarter turn after the first, its farthest point is at most
///    [`CIRCLE_ROUND`] times as far from the centre as its nearest, and it ends at most [`CIRCLE_CLOSE`] times
///    as far from the centre as it began, or as near; otherwise the lap starts anew. A circle is cued once —
///    `circle` clockwise as seen on screen, `countercircle` the other way — and followed by [`CIRCLE_REST`]
///    ticks of rest.
pub fn circle_step(circling: Circling, pointer: Point, body: Rect, tick: Ticks, guards: Guards) -> CircleStep {
    if guards.control || guards.quiet || guards.still {
        return CircleStep { state: no_circling(circling.rest), cue: None };
    }
    if guards.scrolled {
        return CircleStep { state: no_circling(max(circling.rest, tick + SCROLL_TICKS)), cue: None };
    }
    if tick < circling.rest {
        return CircleStep { state: no_circling(circling.rest), cue: None };
    }
    let rx = pointer.x - (body.x + body.width / 2.0);
    let ry = pointer.y - (body.y + body.height / 2.0);
    let radius = rx * rx + ry * ry;
    let reach = larger(body.width, body.height);
    let inner = reach / 2.0 + CIRCLE_MARGIN;
    let outer = reach * CIRCLE_REACH;
    let banded = radius >= inner * inner && radius <= outer * outer;
    if !banded && (!circling.live || tick - circling.inside > CIRCLE_OUT_TICKS) {
        return CircleStep { state: no_circling(circling.rest), cue: None };
    }
    let inside = if banded { tick } else { circling.inside };
    let sx = if rx > CIRCLE_HYSTERESIS {
        1
    } else if rx < 0.0 - CIRCLE_HYSTERESIS {
        -1
    } else {
        circling.sx
    };
    let sy = if ry > CIRCLE_HYSTERESIS {
        1
    } else if ry < 0.0 - CIRCLE_HYSTERESIS {
        -1
    } else {
        circling.sy
    };
    let known = circling.sx != 0 && circling.sy != 0 && sx != 0 && sy != 0;
    let turned = if known { quadrant_of(sx, sy) - quadrant_of(circling.sx, circling.sy) } else { 0 };
    let quarter = if turned < 0 { turned + 4 } else { turned };
    let cross = circling.px * ry - circling.py * rx;
    let step: i64 = if quarter == 1 {
        1
    } else if quarter == 3 {
        -1
    } else if quarter == 2 {
        if cross > 0.0 {
            2
        } else if cross < 0.0 {
            -2
        } else {
            0
        }
    } else {
        0
    };
    let size = step.abs();
    let way = if step > 0 { 1 } else { -1 };
    let lost = (quarter == 2 && step == 0) || (circling.steps > 0 && tick - circling.last > CIRCLE_SLOW);
    let fresh = lost || circling.steps == 0 || (step != 0 && way != circling.turn && circling.against + size > CIRCLE_AGAINST);
    let turn = if step == 0 {
        if fresh {
            0
        } else {
            circling.turn
        }
    } else if fresh {
        way
    } else {
        circling.turn
    };
    let quarters = if step == 0 {
        if fresh {
            0
        } else {
            circling.quarters
        }
    } else if fresh {
        size
    } else if way == circling.turn {
        circling.quarters + size
    } else {
        circling.quarters - size
    };
    let steps = if step == 0 {
        if fresh {
            0
        } else {
            circling.steps
        }
    } else if fresh {
        size
    } else {
        circling.steps + size
    };
    let against = if fresh {
        0
    } else if step != 0 && way != circling.turn {
        circling.against + size
    } else {
        circling.against
    };
    let first = if fresh { tick } else { circling.first };
    let last = if fresh || step != 0 { tick } else { circling.last };
    let open = if fresh { radius } else { circling.open };
    let near = if fresh { radius } else { smaller(circling.near, radius) };
    let far = if fresh { radius } else { larger(circling.far, radius) };
    if quarters < CIRCLE_QUARTERS {
        return CircleStep { state: Circling { live: true, inside, sx, sy, px: rx, py: ry, turn, quarters, steps, against, first, last, open, near, far, rest: circling.rest }, cue: None };
    }
    let brisk = tick - first >= CIRCLE_FAST * (steps - 1);
    let round = far <= CIRCLE_ROUND * CIRCLE_ROUND * near;
    let closed = radius <= CIRCLE_CLOSE * CIRCLE_CLOSE * open && open <= CIRCLE_CLOSE * CIRCLE_CLOSE * radius;
    if brisk && round && closed {
        return CircleStep { state: no_circling(tick + CIRCLE_REST), cue: Some(if turn > 0 { Cue::Circle } else { Cue::Countercircle }) };
    }
    CircleStep { state: Circling { live: true, inside, sx, sy, px: rx, py: ry, turn: 0, quarters: 0, steps: 0, against: 0, first: tick, last: tick, open: radius, near: radius, far: radius, rest: circling.rest }, cue: None }
}
//#endregion 🔖️Circling

//#region 🔖️Stroking
/// 🧤️ How far beyond the body the zone of a stroke reaches on every side, in pixels.
pub const STROKE_MARGIN: f64 = 10.0;

/// 🪢️ How far the pointer has to come back from its running extreme before a reversal is taken, in body widths.
pub const STROKE_HYSTERESIS: f64 = 0.15;

/// 📐️ The least length of a stroke, in body widths.
pub const STROKE_LENGTH: f64 = 0.45;

/// 🐢️ The least average speed of a stroke in pixels per second, the time it rested at its beginning included.
pub const STROKE_SLOW: f64 = 60.0;

/// 🐇️ The greatest average speed of a stroke in pixels per second: what is faster is a wipe across the pet.
pub const STROKE_FAST: f64 = 900.0;

/// ⛰️ How far a stroke may wander up and down, as a part of its length: petting goes across the pet, a pointer that roams over it does not.
pub const STROKE_SLANT: f64 = 0.5;

/// 🧮️ The strokes in a row that make a petting.
pub const STROKE_SEGMENTS: i64 = 3;

/// 🪟️ The ticks within which those strokes begin and end (1.5 s).
pub const STROKE_WINDOW: Ticks = 96;

/// ⏸️ How long the pointer may rest without a new extreme before stroking starts anew, in ticks.
pub const STROKE_PAUSE: Ticks = 48;

/// 🫳️ One step of stroking: the next state and the completed petting (`stroke`), if any.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrokeStep {
    pub state: Stroking,
    pub cue: Option<Cue>,
}

/// 🧽️ No petting in progress, deaf until the tick `rest` (the reset).
pub fn no_stroking(rest: Ticks) -> Stroking {
    Stroking { live: false, way: 0, from: 0.0, began: 0, peak: 0.0, reached: 0, top: 0.0, bottom: 0.0, top_since: 0.0, bottom_since: 0.0, count: 0, first: 0, second: 0, rest }
}

/// 🐈️ One tick of stroking over `body` with the pointer at `pointer`.
///
/// 1. Guards as for circling. Zone: the body grown by [`STROKE_MARGIN`] on every side; leaving it forgets
///    everything, and so does resting for more than [`STROKE_PAUSE`] ticks without a new extreme.
/// 2. Strokes are horizontal runs between reversals. A run follows its extreme; when the pointer has come back from
///    the extreme by [`STROKE_HYSTERESIS`] body widths, the run ended at the extreme and the next one began there.
///    The run that led into the zone, or that follows a rest, is no stroke: it began at no reversal.
/// 3. A stroke counts when it is at least [`STROKE_LENGTH`] body widths long, its middle lies over the body, it
///    wandered up and down by at most [`STROKE_SLANT`] of its length, and its average speed — its length over the
///    ticks from the previous extreme to its own — lies between [`STROKE_SLOW`] and [`STROKE_FAST`]. A stroke
///    that does not count ends the row.
/// 4. [`STROKE_SEGMENTS`] counting strokes in a row, from the beginning of the first to the end of the last within
///    [`STROKE_WINDOW`] ticks, cue `stroke`; the row then starts again, so petting on cues again and again. A row
///    that took longer drops its oldest stroke.
pub fn stroke_step(stroking: Stroking, pointer: Point, body: Rect, tick: Ticks, guards: Guards) -> StrokeStep {
    if guards.control || guards.quiet || guards.still {
        return StrokeStep { state: no_stroking(stroking.rest), cue: None };
    }
    if guards.scrolled {
        return StrokeStep { state: no_stroking(max(stroking.rest, tick + SCROLL_TICKS)), cue: None };
    }
    if tick < stroking.rest {
        return StrokeStep { state: no_stroking(stroking.rest), cue: None };
    }
    let rx = pointer.x - (body.x + body.width / 2.0);
    let ry = pointer.y - (body.y + body.height / 2.0);
    if rx.abs() > body.width / 2.0 + STROKE_MARGIN || ry.abs() > body.height / 2.0 + STROKE_MARGIN {
        return StrokeStep { state: no_stroking(stroking.rest), cue: None };
    }
    if !stroking.live || tick - stroking.reached > STROKE_PAUSE {
        return StrokeStep { state: Stroking { live: true, way: 0, from: rx, began: tick, peak: rx, reached: tick, top: ry, bottom: ry, top_since: ry, bottom_since: ry, count: -1, first: 0, second: 0, rest: stroking.rest }, cue: None };
    }
    let hysteresis = STROKE_HYSTERESIS * body.width;
    let top_since = smaller(stroking.top_since, ry);
    let bottom_since = larger(stroking.bottom_since, ry);
    let run = rx - stroking.from;
    let onward = if stroking.way == 0 { run.abs() >= hysteresis } else { (rx - stroking.peak) * (stroking.way as f64) > 0.0 };
    if onward {
        let way = if stroking.way != 0 {
            stroking.way
        } else if run > 0.0 {
            1
        } else {
            -1
        };
        return StrokeStep {
            state: Stroking {
                live: true,
                way,
                from: stroking.from,
                began: stroking.began,
                peak: rx,
                reached: tick,
                top: smaller(stroking.top, top_since),
                bottom: larger(stroking.bottom, bottom_since),
                top_since: ry,
                bottom_since: ry,
                count: stroking.count,
                first: stroking.first,
                second: stroking.second,
                rest: stroking.rest,
            },
            cue: None,
        };
    }
    if stroking.way == 0 || (stroking.peak - rx) * (stroking.way as f64) < hysteresis {
        return StrokeStep {
            state: Stroking {
                live: true,
                way: stroking.way,
                from: stroking.from,
                began: stroking.began,
                peak: stroking.peak,
                reached: stroking.reached,
                top: stroking.top,
                bottom: stroking.bottom,
                top_since,
                bottom_since,
                count: stroking.count,
                first: stroking.first,
                second: stroking.second,
                rest: stroking.rest,
            },
            cue: None,
        };
    }
    let length = (stroking.peak - stroking.from).abs();
    let speed = (length * TICKS_PER_SECOND as f64) / max(stroking.reached - stroking.began, 1) as f64;
    let counts = length >= STROKE_LENGTH * body.width && (stroking.peak + stroking.from).abs() <= body.width && stroking.bottom - stroking.top <= STROKE_SLANT * length && (STROKE_SLOW..=STROKE_FAST).contains(&speed);
    let count = if counts && stroking.count >= 0 { stroking.count + 1 } else { 0 };
    let first = if count == 1 { stroking.began } else { stroking.first };
    let second = if count == 2 { stroking.began } else { stroking.second };
    let petted = count >= STROKE_SEGMENTS && stroking.reached - first <= STROKE_WINDOW;
    let late = count >= STROKE_SEGMENTS && !petted;
    let row = if petted {
        0
    } else if late {
        STROKE_SEGMENTS - 1
    } else {
        count
    };
    StrokeStep {
        state: Stroking {
            live: true,
            way: 0 - stroking.way,
            from: stroking.peak,
            began: stroking.reached,
            peak: rx,
            reached: tick,
            top: top_since,
            bottom: bottom_since,
            top_since: ry,
            bottom_since: ry,
            count: row,
            first: if late { second } else { first },
            second: if late { stroking.began } else { second },
            rest: stroking.rest,
        },
        cue: petted.then_some(Cue::Stroke),
    }
}
//#endregion 🔖️Stroking

//#region 🔖️Shaking
/// 🎢️ The least length of one swing of a shake, in body heights.
pub const SHAKE_AMPLITUDE: f64 = 0.75;

/// 🪃️ How far the grip has to come back from its farthest point before a reversal is taken, in pixels.
pub const SHAKE_HYSTERESIS: f64 = 8.0;

/// 💨️ The least average speed of one swing in pixels per second (a sine that peaks at 300 px/s).
pub const SHAKE_SPEED: f64 = 190.0;

/// 🔀️ The reversals that make a shake.
pub const SHAKE_REVERSALS: i64 = 4;

/// ⏲️ The ticks within which those reversals lie (1 s).
pub const SHAKE_WINDOW: Ticks = 64;

/// 🛋️ How long the grip may go without a reversal before shaking starts anew, in ticks.
pub const SHAKE_PAUSE: Ticks = 48;

/// 😵️ How long shaking stays deaf after a shake, in ticks (2 s, as long as the pet is dizzy).
pub const SHAKE_REST: Ticks = 128;

/// 🎲️ One step of shaking: the next state and the completed shake (`shake`), if any.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShakeStep {
    pub state: Shaking,
    pub cue: Option<Cue>,
}

/// 🧼️ No shake in progress, deaf until the tick `rest` (the reset; also the state of a hand that holds nothing).
pub fn no_shaking(rest: Ticks) -> Shaking {
    Shaking { live: false, ax: 0.0, ay: 0.0, at: 0, fx: 0.0, fy: 0.0, reached: 0, count: 0, mark1: 0, mark2: 0, mark3: 0, rest }
}

/// 🥤️ One tick of shaking a held pet of the height `height` with the grip (the pointer) at `grip`.
///
/// 1. Only a `still` stage silences it (a pet can be picked up in a quiet one); until `rest` nothing is followed.
/// 2. A swing runs from the latest reversal to the farthest point reached from there. When the grip has come back
///    from that point by [`SHAKE_HYSTERESIS`] pixels, against the direction of the swing, the swing ended there
///    and the next one began. More than [`SHAKE_PAUSE`] ticks without a reversal start anew.
/// 3. A swing counts when it is at least [`SHAKE_AMPLITUDE`] body heights long and at least [`SHAKE_SPEED`]
///    fast on average. A swing that does not count ends the row.
/// 4. [`SHAKE_REVERSALS`] counting reversals in a row with the first and the last at most [`SHAKE_WINDOW`]
///    ticks apart cue `shake`, followed by [`SHAKE_REST`] ticks of rest.
pub fn shake_step(shaking: Shaking, grip: Point, height: f64, tick: Ticks, guards: Guards) -> ShakeStep {
    if guards.still || tick < shaking.rest {
        return ShakeStep { state: no_shaking(shaking.rest), cue: None };
    }
    if !shaking.live || tick - shaking.at > SHAKE_PAUSE {
        return ShakeStep { state: Shaking { live: true, ax: grip.x, ay: grip.y, at: tick, fx: grip.x, fy: grip.y, reached: tick, count: 0, mark1: 0, mark2: 0, mark3: 0, rest: shaking.rest }, cue: None };
    }
    let sx = shaking.fx - shaking.ax;
    let sy = shaking.fy - shaking.ay;
    let span = sx * sx + sy * sy;
    let dx = grip.x - shaking.ax;
    let dy = grip.y - shaking.ay;
    if dx * dx + dy * dy > span {
        return ShakeStep {
            state: Shaking { live: true, ax: shaking.ax, ay: shaking.ay, at: shaking.at, fx: grip.x, fy: grip.y, reached: tick, count: shaking.count, mark1: shaking.mark1, mark2: shaking.mark2, mark3: shaking.mark3, rest: shaking.rest },
            cue: None,
        };
    }
    let bx = grip.x - shaking.fx;
    let by = grip.y - shaking.fy;
    if bx * bx + by * by < SHAKE_HYSTERESIS * SHAKE_HYSTERESIS || bx * sx + by * sy >= 0.0 {
        return ShakeStep { state: shaking, cue: None };
    }
    let amplitude = SHAKE_AMPLITUDE * height;
    let pace = SHAKE_SPEED * max(shaking.reached - shaking.at, 1) as f64;
    let counts = span >= amplitude * amplitude && span * TICKS_PER_SECOND as f64 * TICKS_PER_SECOND as f64 >= pace * pace;
    let count = if counts { shaking.count + 1 } else { 0 };
    if count >= SHAKE_REVERSALS && shaking.reached - shaking.mark1 <= SHAKE_WINDOW {
        return ShakeStep { state: no_shaking(tick + SHAKE_REST), cue: Some(Cue::Shake) };
    }
    ShakeStep {
        state: Shaking {
            live: true,
            ax: shaking.fx,
            ay: shaking.fy,
            at: shaking.reached,
            fx: grip.x,
            fy: grip.y,
            reached: tick,
            count,
            mark1: if counts { shaking.mark2 } else { 0 },
            mark2: if counts { shaking.mark3 } else { 0 },
            mark3: if counts { shaking.reached } else { 0 },
            rest: shaking.rest,
        },
        cue: None,
    }
}
//#endregion 🔖️Shaking

//#region 🔖️Hover
/// 🎁️ One step of the hover gestures: the next state and the gesture completed (`circle`, `countercircle` or `stroke`), if any.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HoverStep {
    pub state: Hover,
    pub cue: Option<Cue>,
}

/// 🍃️ No gesture in progress, both deaf until the tick `rest` (the reset).
pub fn no_hover(rest: Ticks) -> Hover {
    Hover { circling: no_circling(rest), stroking: no_stroking(rest) }
}

/// 🎬️ One tick of both hover gestures for one actor, one gesture at a time: a circle silences both for its rest, a petting forgets the circle in progress and goes on.
pub fn hover_step(hover: Hover, pointer: Point, body: Rect, tick: Ticks, guards: Guards) -> HoverStep {
    let circled = circle_step(hover.circling, pointer, body, tick, guards);
    if circled.cue.is_some() {
        return HoverStep { state: Hover { circling: circled.state, stroking: no_stroking(circled.state.rest) }, cue: circled.cue };
    }
    let stroked = stroke_step(hover.stroking, pointer, body, tick, guards);
    if stroked.cue.is_some() {
        return HoverStep { state: Hover { circling: no_circling(circled.state.rest), stroking: stroked.state }, cue: stroked.cue };
    }
    HoverStep { state: Hover { circling: circled.state, stroking: stroked.state }, cue: None }
}

/// 👁️ Whether a gesture is under way for this actor, so the stage keeps ticking at its full rate until it is decided.
pub fn hover_busy(hover: &Hover) -> bool {
    hover.circling.steps > 0 || hover.stroking.way != 0
}
//#endregion 🔖️Hover

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
