/** 👆️ What the learner's hand does to a pet: the press that becomes a click, a hold or a pick-up, the heat of repeated attention, and the three path gestures — circling round a pet, stroking over it, shaking it while it is held.
 *
 * Every recogniser is a small state value and a pure step: `(state, sample, tick, guards) → { state, cue }`. The
 * stage keeps the states (one press and one shake per stage, one hover and one warmth per actor) and steps them
 * once per tick with the sample-and-hold pointer; a tick is 1/64 s, positions are viewport pixels with the y axis
 * pointing down. Nothing here measures an angle or a velocity vector: circling is counted in quadrants with a
 * Schmitt trigger on both axes, stroking and shaking in reversals between running extremes, and every comparison
 * of lengths is a comparison of squares. Only `+ − × ÷`, `abs`, `min`, `max` and comparisons are used, in the
 * order written here, so the Rust twin reproduces every decision bit for bit.
 *
 * A press alone changes nothing (WCAG 2.5.2): it only arms. What it becomes is decided by what follows — a release
 * (click), time (hold), movement beyond the slop (pick-up), or a cancellation (undo).
 *
 * @see ../../🧬️schema/🟦️.ts — `Cue`, `Point`, `Rect`, `Ticks`
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-cancellation.html
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html
 */

import { type Circling, type Cue, type Hover, type Point, type Pointer, type Press, type Rect, type Shaking, type Stroking, TICKS_PER_SECOND, type Tier, type Ticks, type Warmth } from "../../🧬️schema/🟦️.ts";

//#region 🔖️Guards
/** 🛡️ What silences the recognisers for one step: the pointer lies over a `control` (an interactive element, or a button the pets did not take is down), the page `scrolled` since the last step, the stage is `quiet` (a time of concentration) or `still`. */
export type Guards = { readonly control: boolean; readonly scrolled: boolean; readonly quiet: boolean; readonly still: boolean };

/** 🟢️ No guard is up. */
export const UNGUARDED: Guards = { control: false, scrolled: false, quiet: false, still: false };

/** 📜️ How long circling and stroking stay deaf after the page scrolled under the pointer, in ticks (0.25 s). */
export const SCROLL_TICKS = 16;
//#endregion 🔖️Guards

//#region 🔖️Press
/** 🖱️ How far a mouse or a pen may move after the press before the press is a pick-up, in pixels (the drag threshold of desktop systems). */
export const SLOP_FINE = 6;

/** 📱️ How far a finger may move after the press before the press is a pick-up, in pixels (touch slop plus the jitter of a finger). */
export const SLOP_COARSE = 10;

/** ⏳️ How long a press rests before it is a hold, in ticks (0.44 s, the long-press time of touch systems). */
export const HOLD_TICKS = 28;

/** 💤️ No press. */
export const IDLE: Press = { phase: "idle", x: 0, y: 0, since: 0, slop: 0 };

/** 📥️ What happens to a press: the shell's `pressed`, `dragged`, `released` and `cancelled` events as they are, and the passing of a tick. */
export type PressInput =
  | { readonly kind: "pressed"; readonly x: number; readonly y: number; readonly pointer: Pointer }
  | { readonly kind: "dragged"; readonly x: number; readonly y: number }
  | { readonly kind: "released"; readonly x: number; readonly y: number }
  | { readonly kind: "cancelled" }
  | { readonly kind: "ticked" };

/** 📣️ What a press turned out to be: a `click`, the beginning (`hold`) and the end (`unhold`) of a resting press, a pick-up (`lift`; the grip is the press's point), the release of a picked-up pet (`drop`; at the input's point), or an `abort` that undoes whatever the press had begun. */
export type PressSignal = "click" | "hold" | "unhold" | "lift" | "drop" | "abort";

/** 🪜️ One step of the press: the next state and what the step decided, if anything. */
export type PressStep = { readonly state: Press; readonly signal: PressSignal | null };

/** 📏️ The slop of a pointer type in pixels. */
export function slopOf(pointer: Pointer): number {
  return pointer === "touch" ? SLOP_COARSE : SLOP_FINE;
}

/** 🕹️ One step of the press machine.
 *
 * - `pressed` arms and decides nothing; over a `control` it is not taken at all. A press that arrives while another is
 *   still open aborts the open one first (a release was lost) and arms anew.
 * - Armed: `released` before {@link HOLD_TICKS} is a `click`; `dragged` to the slop or beyond is a `lift`; a tick
 *   {@link HOLD_TICKS} after the press is a `hold`. A hold ends with `unhold` on release and becomes a `lift` when the
 *   pointer leaves the slop after all.
 * - Lifted: `released` is a `drop`; movement is the stage's business (it reads the pointer).
 * - `cancelled` (Escape, pointer cancel, lost capture, blur, hidden, pause) aborts every open press; so does a `still`
 *   stage, and a scroll aborts a press that has not lifted anything (the pet moved away under the pointer).
 * - `quiet` changes nothing: clicks and pick-ups work in a time of concentration.
 */
export function pressStep(press: Press, input: PressInput, tick: Ticks, guards: Guards): PressStep {
  const open = press.phase !== "idle";
  if (guards.still) return { state: IDLE, signal: open ? "abort" : null };
  if (input.kind === "pressed") return { state: guards.control ? IDLE : { phase: "armed", x: input.x, y: input.y, since: tick, slop: slopOf(input.pointer) }, signal: open ? "abort" : null };
  if (!open) return { state: press, signal: null };
  if (input.kind === "cancelled") return { state: IDLE, signal: "abort" };
  if (press.phase === "lifted") return input.kind === "released" ? { state: IDLE, signal: "drop" } : { state: press, signal: null };
  if (guards.scrolled) return { state: IDLE, signal: "abort" };
  if (input.kind === "released") return { state: IDLE, signal: press.phase === "holding" ? "unhold" : tick - press.since < HOLD_TICKS ? "click" : null };
  if (input.kind === "dragged") {
    const dx = input.x - press.x;
    const dy = input.y - press.y;
    return dx * dx + dy * dy >= press.slop * press.slop ? { state: { phase: "lifted", x: press.x, y: press.y, since: press.since, slop: press.slop }, signal: "lift" } : { state: press, signal: null };
  }
  if (press.phase === "armed" && tick - press.since >= HOLD_TICKS) return { state: { phase: "holding", x: press.x, y: press.y, since: press.since, slop: press.slop }, signal: "hold" };
  return { state: press, signal: null };
}

/** ⏰️ The tick at which an armed press becomes a hold, so the stage stays awake for it; `null` when no press is armed. */
export function pressDue(press: Press): Ticks | null {
  return press.phase === "armed" ? press.since + HOLD_TICKS : null;
}
//#endregion 🔖️Press

//#region 🔖️Heat
/** 🔥️ The heat one click adds. */
export const HEAT_CLICK = 1;

/** 🫶️ The heat one tick of holding adds: 2 per second, so a hold from cold has had enough after about five seconds. */
export const HEAT_HOLD = 0.03125;

/** 💧️ The heat that leaks away per second. */
export const HEAT_LEAK = 0.5;

/** 🙋️ Up to this heat a click is answered with a hello. */
export const HEAT_HELLO = 1;

/** 🎩️ Up to this heat a click is answered with a trick; above it with a purr. */
export const HEAT_TRICK = 3;

/** 🛑️ From this heat on the pet has had enough. */
export const HEAT_ENOUGH = 7;

/** 🙈️ How long a pet that has had enough answers nothing but a glance, in ticks (8 s). */
export const ENOUGH_TICKS = 512;

/** 🕊️ The heat a pet starts with again once it has forgiven: the next click is a trick, not a hello. */
export const HEAT_FORGIVEN = 2;

/** 💆️ The attention that warms a pet: a click, or one tick of a resting press. */
export type Caress = "click" | "hold";

/** 🧊️ An actor nobody has touched yet. */
export const COLD: Warmth = { heat: 0, since: 0, until: 0, tier: "hello", run: 0, tricks: 0 };

/** 📉️ What is left at `tick` of the `heat` measured at `since`: it leaks {@link HEAT_LEAK} per second and never falls below zero. */
export function heatAt(heat: number, since: Ticks, tick: Ticks): number {
  return Math.max(heat - (Math.max(tick - since, 0) * HEAT_LEAK) / TICKS_PER_SECOND, 0);
}

/** 📈️ The heat right after a caress at `tick`: what is left of `heat` since `since`, plus {@link HEAT_CLICK} or {@link HEAT_HOLD}. */
export function heatAfter(heat: number, since: Ticks, tick: Ticks, caress: Caress): number {
  return heatAt(heat, since, tick) + (caress === "hold" ? HEAT_HOLD : HEAT_CLICK);
}

/** 🏷️ The tier of a heat: `hello` up to {@link HEAT_HELLO}, `trick` up to {@link HEAT_TRICK}, `purr` below {@link HEAT_ENOUGH}, `enough` from there on. */
export function tierOf(heat: number): Tier {
  return heat <= HEAT_HELLO ? "hello" : heat <= HEAT_TRICK ? "trick" : heat < HEAT_ENOUGH ? "purr" : "enough";
}

/** 💓️ The warmth after a caress at `tick`, with the answer in its `tier`, `run` and `tricks`.
 *
 * A click is answered by the tier of the heat it leaves; a tick of holding is a purr until the heat says enough.
 * The caress that reaches {@link HEAT_ENOUGH} is answered with `enough` (run 1: the pet turns away); for
 * {@link ENOUGH_TICKS} from then on every caress is ignored and only counted (`enough`, run 2, 3, …: a glance), and
 * after that the pet is back at {@link HEAT_FORGIVEN} and leaks from there. Nothing is ever taken away for it.
 */
export function warmthAfter(warmth: Warmth, tick: Ticks, caress: Caress): Warmth {
  if (tick < warmth.until) return { heat: warmth.heat, since: warmth.since, until: warmth.until, tier: "enough", run: warmth.run + 1, tricks: warmth.tricks };
  const heat = heatAfter(warmth.heat, warmth.since, tick, caress);
  const tier: Tier = heat >= HEAT_ENOUGH ? "enough" : caress === "hold" ? "purr" : tierOf(heat);
  if (tier === "enough") return { heat: HEAT_FORGIVEN, since: tick + ENOUGH_TICKS, until: tick + ENOUGH_TICKS, tier, run: 1, tricks: warmth.tricks };
  return { heat, since: tick, until: warmth.until, tier, run: tier === warmth.tier ? warmth.run + 1 : 1, tricks: tier === "trick" ? warmth.tricks + 1 : warmth.tricks };
}
//#endregion 🔖️Heat

//#region 🔖️Circling
/** 🍩️ How far outside half the body's larger side the band of a circle begins, in pixels: nearer to the centre the pointer is on the pet, not round it. */
export const CIRCLE_MARGIN = 8;

/** 🪐️ Where the band of a circle ends, in multiples of the body's larger side from its centre. */
export const CIRCLE_REACH = 3.4;

/** 🧲️ The Schmitt trigger of both axes, in pixels: a coordinate changes its sign only beyond this distance from the axis, so the tremor of a hand on an axis does not count quarter turns back and forth. */
export const CIRCLE_HYSTERESIS = 4;

/** ⭕️ The quarter turns of one circle: the fifth axis crossing is the one the lap began with, so the pointer has gone all the way round. */
export const CIRCLE_QUARTERS = 5;

/** 🏎️ The least ticks per quarter turn on average over a lap (4 turns per second): what is faster is a scribble. */
export const CIRCLE_FAST = 4;

/** 🐌️ The most ticks between two quarter turns (0.4 turns per second): a lap that stalls longer starts anew. */
export const CIRCLE_SLOW = 40;

/** 🥚️ How much farther from the centre the farthest point of a lap may be than its nearest: a lap is round, not a dash past the pet. */
export const CIRCLE_ROUND = 2.5;

/** 🔗️ How much the distances from the centre at the two ends of a lap may differ, as a factor: a circle comes back to where it began, a loop of aimless wiggling rarely does. */
export const CIRCLE_CLOSE = 1.375;

/** 🔙️ The quarter turns a lap may take against its direction. */
export const CIRCLE_AGAINST = 1;

/** 🚪️ How long the pointer may leave the band before the circle is forgotten, in ticks. */
export const CIRCLE_OUT_TICKS = 8;

/** 🧘️ How long circling stays deaf after a circle, in ticks (2 s). */
export const CIRCLE_REST = 128;

/** 🎯️ One step of circling: the next state and the completed circle, if any. */
export type CircleStep = { readonly state: Circling; readonly cue: Extract<Cue, "circle" | "countercircle"> | null };

/** 🧹️ No circle in progress, deaf until the tick `rest` (the reset). */
export function noCircling(rest: Ticks): Circling {
  return { live: false, inside: 0, sx: 0, sy: 0, px: 0, py: 0, turn: 0, quarters: 0, steps: 0, against: 0, first: 0, last: 0, open: 0, near: 0, far: 0, rest };
}

/** 🧭️ The quadrant of two signs in clockwise order as seen on screen: 0 top right, 1 bottom right, 2 bottom left, 3 top left. */
function quadrantOf(sx: number, sy: number): number {
  return sx > 0 ? (sy < 0 ? 0 : 1) : sy > 0 ? 2 : 3;
}

/** 🔁️ One tick of circling round `body` with the pointer at `pointer`.
 *
 * 1. Guards: over a control, in a quiet or still stage, for {@link SCROLL_TICKS} after a scroll and until `rest`
 *    nothing is followed.
 * 2. Band: the pointer is followed while its distance from the body's centre lies between
 *    `max(width, height) ÷ 2 + CIRCLE_MARGIN` and `max(width, height) × CIRCLE_REACH`; it may leave for
 *    {@link CIRCLE_OUT_TICKS}.
 * 3. Quarter turns: each offset coordinate keeps its sign until it is more than {@link CIRCLE_HYSTERESIS} beyond the
 *    axis. The two signs name a quadrant; moving to the next quadrant clockwise is +1, to the previous −1, to the
 *    opposite one ±2 by the sign of `previous × current` (the cross product), and nothing when that is zero.
 * 4. Lap: the first quarter turn sets the direction. A lap starts anew when more than {@link CIRCLE_SLOW} ticks
 *    pass without a quarter turn, or with the quarter turn that would be more than {@link CIRCLE_AGAINST} against
 *    its direction.
 * 5. Circle: at {@link CIRCLE_QUARTERS} net quarter turns the lap is a circle when it took at least
 *    {@link CIRCLE_FAST} ticks per quarter turn after the first, its farthest point is at most
 *    {@link CIRCLE_ROUND} times as far from the centre as its nearest, and it ends at most {@link CIRCLE_CLOSE} times
 *    as far from the centre as it began, or as near; otherwise the lap starts anew. A circle is cued once —
 *    `circle` clockwise as seen on screen, `countercircle` the other way — and followed by {@link CIRCLE_REST}
 *    ticks of rest.
 */
export function circleStep(circling: Circling, pointer: Point, body: Rect, tick: Ticks, guards: Guards): CircleStep {
  if (guards.control || guards.quiet || guards.still) return { state: noCircling(circling.rest), cue: null };
  if (guards.scrolled) return { state: noCircling(Math.max(circling.rest, tick + SCROLL_TICKS)), cue: null };
  if (tick < circling.rest) return { state: noCircling(circling.rest), cue: null };
  const rx = pointer.x - (body.x + body.width / 2);
  const ry = pointer.y - (body.y + body.height / 2);
  const radius = rx * rx + ry * ry;
  const reach = Math.max(body.width, body.height);
  const inner = reach / 2 + CIRCLE_MARGIN;
  const outer = reach * CIRCLE_REACH;
  const banded = radius >= inner * inner && radius <= outer * outer;
  if (!banded && (!circling.live || tick - circling.inside > CIRCLE_OUT_TICKS)) return { state: noCircling(circling.rest), cue: null };
  const inside = banded ? tick : circling.inside;
  const sx = rx > CIRCLE_HYSTERESIS ? 1 : rx < 0 - CIRCLE_HYSTERESIS ? -1 : circling.sx;
  const sy = ry > CIRCLE_HYSTERESIS ? 1 : ry < 0 - CIRCLE_HYSTERESIS ? -1 : circling.sy;
  const known = circling.sx !== 0 && circling.sy !== 0 && sx !== 0 && sy !== 0;
  const turned = known ? quadrantOf(sx, sy) - quadrantOf(circling.sx, circling.sy) : 0;
  const quarter = turned < 0 ? turned + 4 : turned;
  const cross = circling.px * ry - circling.py * rx;
  const step = quarter === 1 ? 1 : quarter === 3 ? -1 : quarter === 2 ? (cross > 0 ? 2 : cross < 0 ? -2 : 0) : 0;
  const size = Math.abs(step);
  const way = step > 0 ? 1 : -1;
  const lost = (quarter === 2 && step === 0) || (circling.steps > 0 && tick - circling.last > CIRCLE_SLOW);
  const fresh = lost || circling.steps === 0 || (step !== 0 && way !== circling.turn && circling.against + size > CIRCLE_AGAINST);
  const turn = step === 0 ? (fresh ? 0 : circling.turn) : fresh ? way : circling.turn;
  const quarters = step === 0 ? (fresh ? 0 : circling.quarters) : fresh ? size : way === circling.turn ? circling.quarters + size : circling.quarters - size;
  const steps = step === 0 ? (fresh ? 0 : circling.steps) : fresh ? size : circling.steps + size;
  const against = fresh ? 0 : step !== 0 && way !== circling.turn ? circling.against + size : circling.against;
  const first = fresh ? tick : circling.first;
  const last = fresh || step !== 0 ? tick : circling.last;
  const open = fresh ? radius : circling.open;
  const near = fresh ? radius : Math.min(circling.near, radius);
  const far = fresh ? radius : Math.max(circling.far, radius);
  if (quarters < CIRCLE_QUARTERS) return { state: { live: true, inside, sx, sy, px: rx, py: ry, turn, quarters, steps, against, first, last, open, near, far, rest: circling.rest }, cue: null };
  const brisk = tick - first >= CIRCLE_FAST * (steps - 1);
  const round = far <= CIRCLE_ROUND * CIRCLE_ROUND * near;
  const closed = radius <= CIRCLE_CLOSE * CIRCLE_CLOSE * open && open <= CIRCLE_CLOSE * CIRCLE_CLOSE * radius;
  if (brisk && round && closed) return { state: noCircling(tick + CIRCLE_REST), cue: turn > 0 ? "circle" : "countercircle" };
  return { state: { live: true, inside, sx, sy, px: rx, py: ry, turn: 0, quarters: 0, steps: 0, against: 0, first: tick, last: tick, open: radius, near: radius, far: radius, rest: circling.rest }, cue: null };
}
//#endregion 🔖️Circling

//#region 🔖️Stroking
/** 🧤️ How far beyond the body the zone of a stroke reaches on every side, in pixels. */
export const STROKE_MARGIN = 10;

/** 🪢️ How far the pointer has to come back from its running extreme before a reversal is taken, in body widths. */
export const STROKE_HYSTERESIS = 0.15;

/** 📐️ The least length of a stroke, in body widths. */
export const STROKE_LENGTH = 0.45;

/** 🐢️ The least average speed of a stroke in pixels per second, the time it rested at its beginning included. */
export const STROKE_SLOW = 60;

/** 🐇️ The greatest average speed of a stroke in pixels per second: what is faster is a wipe across the pet. */
export const STROKE_FAST = 900;

/** ⛰️ How far a stroke may wander up and down, as a part of its length: petting goes across the pet, a pointer that roams over it does not. */
export const STROKE_SLANT = 0.5;

/** 🧮️ The strokes in a row that make a petting. */
export const STROKE_SEGMENTS = 3;

/** 🪟️ The ticks within which those strokes begin and end (1.5 s). */
export const STROKE_WINDOW = 96;

/** ⏸️ How long the pointer may rest without a new extreme before stroking starts anew, in ticks. */
export const STROKE_PAUSE = 48;

/** 🫳️ One step of stroking: the next state and the completed petting, if any. */
export type StrokeStep = { readonly state: Stroking; readonly cue: Extract<Cue, "stroke"> | null };

/** 🧽️ No petting in progress, deaf until the tick `rest` (the reset). */
export function noStroking(rest: Ticks): Stroking {
  return { live: false, way: 0, from: 0, began: 0, peak: 0, reached: 0, top: 0, bottom: 0, topSince: 0, bottomSince: 0, count: 0, first: 0, second: 0, rest };
}

/** 🐈️ One tick of stroking over `body` with the pointer at `pointer`.
 *
 * 1. Guards as for circling. Zone: the body grown by {@link STROKE_MARGIN} on every side; leaving it forgets
 *    everything, and so does resting for more than {@link STROKE_PAUSE} ticks without a new extreme.
 * 2. Strokes are horizontal runs between reversals. A run follows its extreme; when the pointer has come back from
 *    the extreme by {@link STROKE_HYSTERESIS} body widths, the run ended at the extreme and the next one began there.
 *    The run that led into the zone, or that follows a rest, is no stroke: it began at no reversal.
 * 3. A stroke counts when it is at least {@link STROKE_LENGTH} body widths long, its middle lies over the body, it
 *    wandered up and down by at most {@link STROKE_SLANT} of its length, and its average speed — its length over the
 *    ticks from the previous extreme to its own — lies between {@link STROKE_SLOW} and {@link STROKE_FAST}. A stroke
 *    that does not count ends the row.
 * 4. {@link STROKE_SEGMENTS} counting strokes in a row, from the beginning of the first to the end of the last within
 *    {@link STROKE_WINDOW} ticks, cue `stroke`; the row then starts again, so petting on cues again and again. A row
 *    that took longer drops its oldest stroke.
 */
export function strokeStep(stroking: Stroking, pointer: Point, body: Rect, tick: Ticks, guards: Guards): StrokeStep {
  if (guards.control || guards.quiet || guards.still) return { state: noStroking(stroking.rest), cue: null };
  if (guards.scrolled) return { state: noStroking(Math.max(stroking.rest, tick + SCROLL_TICKS)), cue: null };
  if (tick < stroking.rest) return { state: noStroking(stroking.rest), cue: null };
  const rx = pointer.x - (body.x + body.width / 2);
  const ry = pointer.y - (body.y + body.height / 2);
  if (Math.abs(rx) > body.width / 2 + STROKE_MARGIN || Math.abs(ry) > body.height / 2 + STROKE_MARGIN) return { state: noStroking(stroking.rest), cue: null };
  if (!stroking.live || tick - stroking.reached > STROKE_PAUSE) return { state: { live: true, way: 0, from: rx, began: tick, peak: rx, reached: tick, top: ry, bottom: ry, topSince: ry, bottomSince: ry, count: -1, first: 0, second: 0, rest: stroking.rest }, cue: null };
  const hysteresis = STROKE_HYSTERESIS * body.width;
  const topSince = Math.min(stroking.topSince, ry);
  const bottomSince = Math.max(stroking.bottomSince, ry);
  const run = rx - stroking.from;
  const onward = stroking.way === 0 ? Math.abs(run) >= hysteresis : (rx - stroking.peak) * stroking.way > 0;
  if (onward) return { state: { live: true, way: stroking.way === 0 ? (run > 0 ? 1 : -1) : stroking.way, from: stroking.from, began: stroking.began, peak: rx, reached: tick, top: Math.min(stroking.top, topSince), bottom: Math.max(stroking.bottom, bottomSince), topSince: ry, bottomSince: ry, count: stroking.count, first: stroking.first, second: stroking.second, rest: stroking.rest }, cue: null };
  if (stroking.way === 0 || (stroking.peak - rx) * stroking.way < hysteresis) return { state: { live: true, way: stroking.way, from: stroking.from, began: stroking.began, peak: stroking.peak, reached: stroking.reached, top: stroking.top, bottom: stroking.bottom, topSince, bottomSince, count: stroking.count, first: stroking.first, second: stroking.second, rest: stroking.rest }, cue: null };
  const length = Math.abs(stroking.peak - stroking.from);
  const speed = (length * TICKS_PER_SECOND) / Math.max(stroking.reached - stroking.began, 1);
  const counts = length >= STROKE_LENGTH * body.width && Math.abs(stroking.peak + stroking.from) <= body.width && stroking.bottom - stroking.top <= STROKE_SLANT * length && speed >= STROKE_SLOW && speed <= STROKE_FAST;
  const count = counts && stroking.count >= 0 ? stroking.count + 1 : 0;
  const first = count === 1 ? stroking.began : stroking.first;
  const second = count === 2 ? stroking.began : stroking.second;
  const petted = count >= STROKE_SEGMENTS && stroking.reached - first <= STROKE_WINDOW;
  const late = count >= STROKE_SEGMENTS && !petted;
  return {
    state: { live: true, way: 0 - stroking.way, from: stroking.peak, began: stroking.reached, peak: rx, reached: tick, top: topSince, bottom: bottomSince, topSince: ry, bottomSince: ry, count: petted ? 0 : late ? STROKE_SEGMENTS - 1 : count, first: late ? second : first, second: late ? stroking.began : second, rest: stroking.rest },
    cue: petted ? "stroke" : null,
  };
}
//#endregion 🔖️Stroking

//#region 🔖️Shaking
/** 🎢️ The least length of one swing of a shake, in body heights. */
export const SHAKE_AMPLITUDE = 0.75;

/** 🪃️ How far the grip has to come back from its farthest point before a reversal is taken, in pixels. */
export const SHAKE_HYSTERESIS = 8;

/** 💨️ The least average speed of one swing in pixels per second (a sine that peaks at 300 px/s). */
export const SHAKE_SPEED = 190;

/** 🔀️ The reversals that make a shake. */
export const SHAKE_REVERSALS = 4;

/** ⏲️ The ticks within which those reversals lie (1 s). */
export const SHAKE_WINDOW = 64;

/** 🛋️ How long the grip may go without a reversal before shaking starts anew, in ticks. */
export const SHAKE_PAUSE = 48;

/** 😵️ How long shaking stays deaf after a shake, in ticks (2 s, as long as the pet is dizzy). */
export const SHAKE_REST = 128;

/** 🎲️ One step of shaking: the next state and the completed shake, if any. */
export type ShakeStep = { readonly state: Shaking; readonly cue: Extract<Cue, "shake"> | null };

/** 🧼️ No shake in progress, deaf until the tick `rest` (the reset; also the state of a hand that holds nothing). */
export function noShaking(rest: Ticks): Shaking {
  return { live: false, ax: 0, ay: 0, at: 0, fx: 0, fy: 0, reached: 0, count: 0, mark1: 0, mark2: 0, mark3: 0, rest };
}

/** 🥤️ One tick of shaking a held pet of the height `height` with the grip (the pointer) at `grip`.
 *
 * 1. Only a `still` stage silences it (a pet can be picked up in a quiet one); until `rest` nothing is followed.
 * 2. A swing runs from the latest reversal to the farthest point reached from there. When the grip has come back
 *    from that point by {@link SHAKE_HYSTERESIS} pixels, against the direction of the swing, the swing ended there
 *    and the next one began. More than {@link SHAKE_PAUSE} ticks without a reversal start anew.
 * 3. A swing counts when it is at least {@link SHAKE_AMPLITUDE} body heights long and at least {@link SHAKE_SPEED}
 *    fast on average. A swing that does not count ends the row.
 * 4. {@link SHAKE_REVERSALS} counting reversals in a row with the first and the last at most {@link SHAKE_WINDOW}
 *    ticks apart cue `shake`, followed by {@link SHAKE_REST} ticks of rest.
 */
export function shakeStep(shaking: Shaking, grip: Point, height: number, tick: Ticks, guards: Guards): ShakeStep {
  if (guards.still || tick < shaking.rest) return { state: noShaking(shaking.rest), cue: null };
  if (!shaking.live || tick - shaking.at > SHAKE_PAUSE) return { state: { live: true, ax: grip.x, ay: grip.y, at: tick, fx: grip.x, fy: grip.y, reached: tick, count: 0, mark1: 0, mark2: 0, mark3: 0, rest: shaking.rest }, cue: null };
  const sx = shaking.fx - shaking.ax;
  const sy = shaking.fy - shaking.ay;
  const span = sx * sx + sy * sy;
  const dx = grip.x - shaking.ax;
  const dy = grip.y - shaking.ay;
  if (dx * dx + dy * dy > span) return { state: { live: true, ax: shaking.ax, ay: shaking.ay, at: shaking.at, fx: grip.x, fy: grip.y, reached: tick, count: shaking.count, mark1: shaking.mark1, mark2: shaking.mark2, mark3: shaking.mark3, rest: shaking.rest }, cue: null };
  const bx = grip.x - shaking.fx;
  const by = grip.y - shaking.fy;
  if (bx * bx + by * by < SHAKE_HYSTERESIS * SHAKE_HYSTERESIS || bx * sx + by * sy >= 0) return { state: shaking, cue: null };
  const amplitude = SHAKE_AMPLITUDE * height;
  const pace = SHAKE_SPEED * Math.max(shaking.reached - shaking.at, 1);
  const counts = span >= amplitude * amplitude && span * TICKS_PER_SECOND * TICKS_PER_SECOND >= pace * pace;
  const count = counts ? shaking.count + 1 : 0;
  if (count >= SHAKE_REVERSALS && shaking.reached - shaking.mark1 <= SHAKE_WINDOW) return { state: noShaking(tick + SHAKE_REST), cue: "shake" };
  return { state: { live: true, ax: shaking.fx, ay: shaking.fy, at: shaking.reached, fx: grip.x, fy: grip.y, reached: tick, count, mark1: counts ? shaking.mark2 : 0, mark2: counts ? shaking.mark3 : 0, mark3: counts ? shaking.reached : 0, rest: shaking.rest }, cue: null };
}
//#endregion 🔖️Shaking

//#region 🔖️Hover
/** 🎁️ One step of the hover gestures: the next state and the gesture completed, if any. */
export type HoverStep = { readonly state: Hover; readonly cue: Extract<Cue, "circle" | "countercircle" | "stroke"> | null };

/** 🍃️ No gesture in progress, both deaf until the tick `rest` (the reset). */
export function noHover(rest: Ticks): Hover {
  return { circling: noCircling(rest), stroking: noStroking(rest) };
}

/** 🎬️ One tick of both hover gestures for one actor, one gesture at a time: a circle silences both for its rest, a petting forgets the circle in progress and goes on. */
export function hoverStep(hover: Hover, pointer: Point, body: Rect, tick: Ticks, guards: Guards): HoverStep {
  const circled = circleStep(hover.circling, pointer, body, tick, guards);
  if (circled.cue !== null) return { state: { circling: circled.state, stroking: noStroking(circled.state.rest) }, cue: circled.cue };
  const stroked = strokeStep(hover.stroking, pointer, body, tick, guards);
  if (stroked.cue !== null) return { state: { circling: noCircling(circled.state.rest), stroking: stroked.state }, cue: stroked.cue };
  return { state: { circling: circled.state, stroking: stroked.state }, cue: null };
}

/** 👁️ Whether a gesture is under way for this actor, so the stage keeps ticking at its full rate until it is decided. */
export function hoverBusy(hover: Hover): boolean {
  return hover.circling.steps > 0 || hover.stroking.way !== 0;
}

/** 🌪️ Whether the pointer is circling this actor at tick `tick`: a lap has begun (a quarter turn at least) and its last quarter turn is no older than {@link CIRCLE_SLOW} — the circle holds the pet's attention, so it sets off on nothing of its own until the circle is made, given up or timed out. */
export function circled(hover: Hover, tick: Ticks): boolean {
  return hover.circling.steps > 0 && tick - hover.circling.last <= CIRCLE_SLOW;
}
//#endregion 🔖️Hover
