/** 🪄️ Mischief of the pets with things of the page, as pure arithmetic: which marked elements (fixtures) fit a species, which one is picked, when a pet may start, where it works, how the lifted copy of the fixture moves, and how the pet is thrown off when the learner takes the element back.
 *
 * Nothing here moves a real element. The page marks what a pet may play with by a key in the vocabulary of the
 * menagerie's `grounds` (`<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`); the survey reports those as fixtures;
 * the render target shows a copy of the fixture and moves the copy by what {@link liftAt} says, while the element
 * itself keeps its place and only turns transparent.
 *
 * **The choice never reveals an answer** (rule 7 of the second round). Which fixture is pushed depends on two things
 * only: whether a ground of the species equals the fixture's key or is a prefix of it at a `/` ({@link fits}), and a
 * number in [0, 1) the stage draws ({@link chosenFixture}). How far it is pushed depends on a second such number
 * and on the room beside it ({@link liftAt}); when it is pushed depends on gates of time, size and consent
 * ({@link allowed}). No function of this module receives a value, a correctness flag or an answer of the learner,
 * and none reads a property of a fixture other than its key and its box.
 *
 * Etiquette: one lift at a time, only when the learner permits it, only on a wide page with a fine pointer, only
 * after the learner has been still for a while (longer in a time of concentration), never more often than the mode
 * allows, a few pixels up and down at most, and over within {@link LIFT_TICKS}. Taking the element back — the
 * pointer over it, a press, focus, a key, an input, a change, a drag — ends the lift at once; that is the render
 * target's to notice and the stage's to answer with {@link thrownOff}.
 *
 * @see ../../🧬️schema/🟦️.ts — `Perch`, `PetMode`, `Point`, `Ticks`, `Turns`
 * @see ../🏞️terrain/🟦️.ts — `Pitch`, the free stretch of a wall
 * @see ../🧗️climbing/🟦️.ts — `Toss`, a velocity with which an actor is thrown into the air
 * @see ../📐️trigonometry/🟦️.ts — `sinTurns`, `smoothstep`, `clamp`
 */

import type { Fixture, Perch, PetMode, Pitch, Point, Ticks, Turns } from "../../🧬️schema/🟦️.ts";
import { clamp, sinTurns, smoothstep } from "../📐️trigonometry/🟦️.ts";
import type { Toss } from "../🧗️climbing/🟦️.ts";

//#region 🔖️Constants
/** 🖥️ The least width of the stage in pixels at which mischief happens: narrower pages have no room beside their cards. */
export const MISCHIEF_WIDTH = 1024;

/** 🧘️ The ticks the learner must have been still before a pet starts mischief: 12 seconds. */
export const MISCHIEF_PATIENCE = 768;

/** 🤫️ The ticks the learner must have been still in a time of concentration: 30 seconds. */
export const MISCHIEF_PATIENCE_QUIET = 1920;

/** 🐢️ The ticks between the end of one lift and the start of the next on a calm stage: 3 minutes. */
export const MISCHIEF_COOLDOWN_CALM = 11520;

/** 🐇️ The ticks between the end of one lift and the start of the next on a lively stage: 45 seconds. */
export const MISCHIEF_COOLDOWN_LIVELY = 2880;

/** 📏️ How many pixels may lie between a fixture's end and the wall or the perch its pusher works from. */
export const STATION_GAP = 24;

/** 🪡️ How many pixels a fixture's end may stick out beyond the wall beside it and still count as inside. */
export const STATION_SLACK = 2;

/** 🪜️ How many pixels below a fixture's lower edge a perch may lie and still be at its height. */
export const STATION_STEP = 12;

/** 🚪️ The least room in pixels beyond a fixture's far end that is worth a shove. */
export const LIFT_ROOM = 12;

/** 💪️ The ticks the pusher braces before the copy moves; the copy appears during them. */
export const LIFT_BRACE = 24;

/** 👉️ The ticks of the shove out of the stack. */
export const LIFT_SHOVE = 40;

/** 🫨️ The ticks the copy wobbles where the shove left it. */
export const LIFT_WOBBLE = 48;

/** 🧍️ The ticks the copy rests outside its stack, motionless: 8 seconds. */
export const LIFT_HOLD = 512;

/** 👈️ The ticks of putting the copy back. */
export const LIFT_RETURN = 56;

/** 🌫️ The ticks over which the copy appears at the start of a lift and vanishes at its end, each time lying exactly on the element. */
export const LIFT_FADE = 8;

/** ↩️ The ticks after the start of a lift at which putting back begins. */
export const LIFT_RETURNS = LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE + LIFT_HOLD;

/** ⏱️ The ticks a whole lift lasts, from the first trace of the copy to its last: 10.75 seconds. */
export const LIFT_TICKS = LIFT_RETURNS + LIFT_RETURN + LIFT_FADE;

/** ⛔️ The ticks no lift may exceed: 20 seconds. */
export const LIFT_LIMIT = 1280;

/** 🤏️ The share of the granted room every shove travels at least; the stage's draw decides the rest. */
export const LIFT_LEAST = 0.6;

/** 🧲️ How many pixels the copy gives towards the pusher while it braces, and rocks to and fro after the shove. */
export const LIFT_GIVE = 1.5;

/** 🎈️ The most pixels the copy ever leaves its row upwards or downwards, and the most an end of it rises when it tilts. */
export const LIFT_RISE = 2;

/** 📐️ The most the copy ever tilts, in turns (1.44°); a wide copy tilts less, so that its ends rise by {@link LIFT_RISE} at most. */
export const LIFT_TILT = 0.004;

/** 🔔️ How many times the copy rocks to and fro while it wobbles. */
export const LIFT_WOBBLES = 2;

/** 🥧️ The radians of half a turn: turns the rise of a copy's end into its tilt. */
export const HALF_TURN_RADIANS = 3.141592653589793;

/** 🏌️ The least sideways speed in pixels per second with which a pusher is thrown off. */
export const THROW_SPEED = 120;

/** 🌬️ How much faster than {@link THROW_SPEED} the stage's draw can make the throw, in pixels per second. */
export const THROW_SPREAD = 80;

/** 🚀️ The upward speed in pixels per second with which a pusher is thrown off. */
export const THROW_LIFT = 220;
//#endregion 🔖️Constants

//#region 🔖️Types
/** 🏷️ Anything that carries a key in the vocabulary of the grounds. */
type Keyed = { readonly key: string };

/** 🚦️ Everything the gates of mischief look at: whether the learner permits it, whether the pointer is fine, the width of the stage, its mode, whether it is a time of concentration, whether a lift is in progress, the tick now, the tick of the learner's last input and the tick the last lift ended. */
export type Circumstances = {
  readonly permitted: boolean;
  readonly fine: boolean;
  readonly width: number;
  readonly mode: PetMode;
  readonly quiet: boolean;
  readonly lifting: boolean;
  readonly tick: Ticks;
  readonly stirred: Ticks;
  readonly rested: Ticks;
};

/** 🏗️ Where a pusher works: on a perch or on a wall (then `wall` names it), of which surface, the line it works at (`x`: the wall, or the point of the perch nearest to the fixture), the height of its feet, the way it shoves (`side`: 1 to the right, −1 to the left; it stands on the other side and faces this way) and the room beyond the fixture's far end. */
export type Station = { readonly footing: "perch" | "wall"; readonly wall: string | null; readonly surface: string; readonly x: number; readonly y: number; readonly side: 1 | -1; readonly room: number };

/** 🪞️ How the copy of a lifted fixture lies relative to the element: shifted by `dx` and `dy` pixels, tilted by `tilt` about its centre, drawn with `opacity`. */
export type Lift = { readonly dx: number; readonly dy: number; readonly tilt: Turns; readonly opacity: number };

/** 🛌️ No lift: the copy lies on its element and is not drawn. */
export const NO_LIFT: Lift = { dx: 0, dy: 0, tilt: 0, opacity: 0 };
//#endregion 🔖️Types

//#region 🔖️Choice
/** 🧩️ Whether a ground covers a key: the two are equal, or the ground is a prefix of the key that ends where a `/` of the key begins — `heating` covers `heating/u-values`, never `heating-load`; an empty ground covers nothing. */
export function fits(ground: string, key: string): boolean {
  if (ground.length === 0 || !key.startsWith(ground)) return false;
  return key.length === ground.length || key[ground.length] === "/";
}

/** 🗂️ The fixtures a species may play with, in the survey's order: those whose key one of the species' grounds covers ({@link fits}). Only the key of a fixture is read. */
export function fixtureFor<Item extends Keyed>(grounds: readonly string[], fixtures: readonly Item[]): Item[] {
  const fitting: Item[] = [];
  for (const fixture of fixtures) {
    const key = fixture.key;
    for (const ground of grounds) {
      if (!fits(ground, key)) continue;
      fitting.push(fixture);
      break;
    }
  }
  return fitting;
}

/** 🎯️ The fixture a pet picks among those that fit it: candidate `⌊unit × count⌋` held inside the list (a unit that is not a number picks the first), every candidate as likely as any other; `null` without candidates. The pick has two inputs and no others — the candidates, which the topic alone selected, and a number in [0, 1) the stage drew — and reads no property of any candidate, so it cannot follow a value, a correctness flag or an answer (rule 7). */
export function chosenFixture<Item>(candidates: readonly Item[], unit: number): Item | null {
  const count = candidates.length;
  if (count === 0) return null;
  const place = Math.floor(unit * count);
  return candidates[place >= count ? count - 1 : place > 0 ? place : 0]!;
}
//#endregion 🔖️Choice

//#region 🔖️Gates
/** ⏳️ The ticks the learner must have been still: {@link MISCHIEF_PATIENCE}, or {@link MISCHIEF_PATIENCE_QUIET} in a time of concentration. */
export function patienceOf(quiet: boolean): Ticks {
  return quiet ? MISCHIEF_PATIENCE_QUIET : MISCHIEF_PATIENCE;
}

/** 🧊️ The ticks a mode leaves between two lifts; `null` for a still stage, where there is no mischief at all. */
export function cooldownOf(mode: PetMode): Ticks | null {
  return mode === "calm" ? MISCHIEF_COOLDOWN_CALM : mode === "lively" ? MISCHIEF_COOLDOWN_LIVELY : null;
}

/** 🕰️ The first tick at which the gates of time are open — the learner has been still long enough and the mode's cooldown since the last lift has passed —, or `null` while another gate is shut: no consent, a coarse pointer, a narrow or still stage, or a lift in progress (whose end sets `rested` anew). */
export function allowedFrom(circumstances: Circumstances): Ticks | null {
  const cooldown = cooldownOf(circumstances.mode);
  if (cooldown === null || !circumstances.permitted || !circumstances.fine || circumstances.lifting || !(circumstances.width >= MISCHIEF_WIDTH)) return null;
  return Math.max(circumstances.stirred + patienceOf(circumstances.quiet), circumstances.rested + cooldown);
}

/** 🚥️ Whether a pet may start mischief now: the learner permits it, the pointer is fine, the stage is at least {@link MISCHIEF_WIDTH} wide and not still, no lift is in progress, the learner has not stirred for {@link patienceOf} ticks and the last lift ended {@link cooldownOf} ticks ago or earlier. */
export function allowed(circumstances: Circumstances): boolean {
  const from = allowedFrom(circumstances);
  return from !== null && circumstances.tick >= from;
}
//#endregion 🔖️Gates

//#region 🔖️Station
/** 🏆️ The station with more room of the two, the earlier one when they have the same; a station without {@link LIFT_ROOM} never counts. */
function roomier(best: Station | null, next: Station): Station | null {
  return next.room >= LIFT_ROOM && (best === null || next.room > best.room) ? next : best;
}

/** 🧭️ Where a pusher works on a fixture, or `null` when nothing is beside it: a perch at the fixture's height (above its upper edge by nothing, below its lower edge by {@link STATION_STEP} at most) that ends within {@link STATION_GAP} of one of its ends, or a free stretch of a wall that runs beside one of its ends (the fixture's end within {@link STATION_GAP} inside the wall, the stretch overlapping the fixture's height; the feet at the fixture's lower edge, held inside the stretch).
 *
 * A pusher on the left shoves to the right and the other way round; the room of a station is what lies between the
 * fixture's far end and the edge of the stage (`width`). The station with the most room wins — the side with room —,
 * a perch before a wall and then the survey's order when rooms are equal. Pass only the pitches the species can
 * climb: none for a species without that gear.
 */
export function stationFor(fixture: Fixture, pitches: readonly Pitch[], perches: readonly Perch[], width: number): Station | null {
  const left = fixture.x;
  const right = fixture.x + fixture.width;
  const top = fixture.y;
  const bottom = fixture.y + fixture.height;
  let best: Station | null = null;
  for (const perch of perches) {
    if (!(perch.y > top && perch.y <= bottom + STATION_STEP)) continue;
    if (perch.x0 < left && left - Math.min(perch.x1, left) <= STATION_GAP) best = roomier(best, { footing: "perch", wall: null, surface: perch.surface, x: Math.min(perch.x1, left), y: perch.y, side: 1, room: width - right });
    if (perch.x1 > right && Math.max(perch.x0, right) - right <= STATION_GAP) best = roomier(best, { footing: "perch", wall: null, surface: perch.surface, x: Math.max(perch.x0, right), y: perch.y, side: -1, room: left });
  }
  for (const pitch of pitches) {
    if (!(pitch.y0 < bottom && pitch.y1 > top)) continue;
    const y = clamp(bottom, pitch.y0, pitch.y1);
    if (pitch.side === -1 && left - pitch.x >= 0 - STATION_SLACK && left - pitch.x <= STATION_GAP) best = roomier(best, { footing: "wall", wall: pitch.wall, surface: pitch.surface, x: pitch.x, y, side: 1, room: width - right });
    if (pitch.side === 1 && pitch.x - right >= 0 - STATION_SLACK && pitch.x - right <= STATION_GAP) best = roomier(best, { footing: "wall", wall: pitch.wall, surface: pitch.surface, x: pitch.x, y, side: -1, room: left });
  }
  return best;
}
//#endregion 🔖️Station

//#region 🔖️Lift
/** 🪃️ `value` on the side `side`: itself to the right, its opposite to the left, never a negative zero. */
function toward(side: 1 | -1, value: number): number {
  return side > 0 ? value : 0 - value;
}

/** 🎢️ The copy of a lifted fixture at `tick`, for a lift that began at `since`: the way it is shoved (`side`), the room granted to it in pixels (the station's room, at most the pusher's width), the width of the fixture (`span`) and a number in [0, 1) the stage drew.
 *
 * The copy travels `room × (LIFT_LEAST + (1 − LIFT_LEAST) × unit)` along x and nowhere else to speak of:
 * 1. brace ({@link LIFT_BRACE} ticks): it appears over {@link LIFT_FADE} ticks, lying exactly on its element, then gives {@link LIFT_GIVE} px towards the pusher and back;
 * 2. shove ({@link LIFT_SHOVE}): it slides out with `smoothstep`, rising {@link LIFT_RISE} px and tipping on the way;
 * 3. wobble ({@link LIFT_WOBBLE}): it rocks {@link LIFT_WOBBLES} times, ever less, and comes to rest;
 * 4. hold ({@link LIFT_HOLD}): it rests, exactly `travel` away, level;
 * 5. put back ({@link LIFT_RETURN}): it slides home the way it came;
 * 6. it vanishes over {@link LIFT_FADE} ticks, lying exactly on its element.
 *
 * `dy` never leaves ±{@link LIFT_RISE}, `tilt` never ±{@link LIFT_TILT} (less for a wide fixture: its ends rise by
 * {@link LIFT_RISE} at most). Before `since` and from {@link liftEnds} on there is no lift: all zero.
 */
export function liftAt(since: Ticks, tick: Ticks, side: 1 | -1, room: number, span: number, unit: number): Lift {
  const age = tick - since;
  if (age < 0 || age >= LIFT_TICKS) return NO_LIFT;
  const travel = Math.max(0, room) * (LIFT_LEAST + (1 - LIFT_LEAST) * unit);
  const lean = span > 0 ? Math.min(LIFT_TILT, LIFT_RISE / (HALF_TURN_RADIANS * span)) : LIFT_TILT;
  if (age < LIFT_FADE) return { dx: 0, dy: 0, tilt: 0, opacity: smoothstep(age / LIFT_FADE) };
  if (age < LIFT_BRACE) return { dx: toward(side, 0 - LIFT_GIVE * sinTurns((age - LIFT_FADE) / (LIFT_BRACE - LIFT_FADE) / 2)), dy: 0, tilt: 0, opacity: 1 };
  if (age < LIFT_BRACE + LIFT_SHOVE) {
    const phase = (age - LIFT_BRACE) / LIFT_SHOVE;
    const hump = sinTurns(phase / 2);
    return { dx: toward(side, travel * smoothstep(phase)), dy: 0 - LIFT_RISE * hump, tilt: toward(side, lean * hump), opacity: 1 };
  }
  if (age < LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE) {
    const phase = (age - LIFT_BRACE - LIFT_SHOVE) / LIFT_WOBBLE;
    const calm = 1 - smoothstep(phase);
    const rock = sinTurns(LIFT_WOBBLES * phase) * calm;
    return { dx: toward(side, travel + LIFT_GIVE * rock), dy: 0.5 * LIFT_RISE * sinTurns(2 * LIFT_WOBBLES * phase) * calm, tilt: toward(side, 0 - lean * rock), opacity: 1 };
  }
  if (age < LIFT_RETURNS) return { dx: toward(side, travel), dy: 0, tilt: 0, opacity: 1 };
  if (age < LIFT_RETURNS + LIFT_RETURN) {
    const phase = (age - LIFT_RETURNS) / LIFT_RETURN;
    const hump = sinTurns(phase / 2);
    return { dx: toward(side, travel * (1 - smoothstep(phase))), dy: 0 - LIFT_RISE * hump, tilt: toward(side, 0 - lean * hump), opacity: 1 };
  }
  return { dx: 0, dy: 0, tilt: 0, opacity: 1 - smoothstep((age - LIFT_RETURNS - LIFT_RETURN) / LIFT_FADE) };
}

/** 🏁️ The first tick at which a lift that began at `since` is over: {@link LIFT_TICKS} later. */
export function liftEnds(since: Ticks): Ticks {
  return since + LIFT_TICKS;
}

/** ⏰️ The next tick after `tick` at which a stage has to look at a lift that began at `since` again, or `null` when the lift is over: its start before it has begun, the next tick while the copy moves, the start of putting back while it rests (between the two the copy does not change). */
export function liftWake(since: Ticks, tick: Ticks): Ticks | null {
  const age = tick - since;
  if (age >= LIFT_TICKS) return null;
  if (age < 0) return since;
  return age >= LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE && age < LIFT_RETURNS ? since + LIFT_RETURNS : tick + 1;
}
//#endregion 🔖️Lift

//#region 🔖️Reclaim
/** 💨️ The velocity with which a pusher at `pusher` is thrown off when the learner takes the fixture back: away from the fixture's middle at {@link THROW_SPEED} plus up to {@link THROW_SPREAD} by the stage's draw, and upwards at {@link THROW_LIFT}; a pusher exactly at the middle goes right. */
export function thrownOff(pusher: Point, fixture: Fixture, unit: number): Toss {
  const speed = THROW_SPEED + THROW_SPREAD * unit;
  return { vx: pusher.x < fixture.x + fixture.width / 2 ? 0 - speed : speed, vy: 0 - THROW_LIFT };
}
//#endregion 🔖️Reclaim
