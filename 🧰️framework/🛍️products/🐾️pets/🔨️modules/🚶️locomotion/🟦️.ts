/** 🏃️ Locomotion: what carries a pet and how it gets around — its footing (a perch, the air, its parachute, the learner's hand, a head, a wall, a ladder, a rope), setting out for a goal on its perch and striding there without ever walking into anybody, scooting to a new seat after a survey, planned motion through the air (a hop, a glide, a fall, a throw, a descent under the parachute, the way back after a cancelled pick-up) on a corridor it has claimed and inside the stage (its left, right and top edges are walls a flier bounces off; at the bottom edge it is gone), trips with its gear (climbing walls and across the gaps between them, carrying, raising and climbing a ladder, firing a grappling hook and reeling itself up, resting on a wall, sliding down it) on a claimed corridor too, hanging from the hand, sliding off a head, vanishing in a puff when nothing legal is left, waiting for a partner and leaving.
 *
 * Every motion is checked before it is committed. A walker, a scooter and a rider on a head move only as far as
 * `guardedStride` lets them; a pet in the hand is pushed out of whatever it is dragged into (`pushedOut`). Whatever
 * flies plans its whole course first — tick by tick, landing included — and takes it only when `claimClear` says that
 * nobody is in its way, now or later; its claim then keeps everybody else out of the corridor and off its landing spot.
 * A pet whose ground went away plans its way down: as it flies, towards the free column over where it lands, with the
 * air control of `STEERINGS`, bounced back or simply let go when it was thrown; it lands on a perch or on another
 * pet's head, whose top is a one-way platform it then slides off. A pet that owns a parachute and would land hard
 * opens it on the way down — decided on the landing its course really makes — and, thrown at the ground too close to
 * it for the canopy to open, brakes: it never lands hard. While no plan is clear it waits where it is for `DELAY`
 * ticks; when even that place is not free it vanishes in a puff and arrives anew. A trip with gear is planned whole the
 * same way — every foothold from the walk to the gear to where it ends (`Trip`) — and taken only when its claim is
 * clear; whatever interrupts it gives it up where the actor is ({@link halt}).
 *
 * A new footing or a new way to travel belongs here: what starts it, its tick of motion and where it ends. A planned
 * motion through the air is a `Course` installed with {@link install}; any other planned motion claims its corridor
 * with `claimOf` and `claimClear` of `🚧️clearance` into the claims of the draft; motion without a plan is guarded.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🚧️clearance/🟦️.ts — claims, guarded strides, heads, the last resort
 * @see ../🪢️swing/🟦️.ts — the hand and the parachute
 * @see ../🏞️terrain/🟦️.ts — falls, hops, landings
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ./🦀️.rs — the Rust twin
 */

import { TICKS_PER_SECOND, type Activity, type Canopy, type Claim, type Course, type Extent, type Foothold, type Ladder, type Perch, type Pitch, type Point, type Rope, type Shot, type Slug, type Species, type Ticks, type Trip, type Turns, type Waypoint } from "../../🧬️schema/🟦️.ts";
import { clipTicks, springStep } from "../🎞️animation/🟦️.ts";
import { randomWords, unitOf } from "../🎲️randomness/🟦️.ts";
import { DELAY, MARGIN, PUSHES, SCOOT_HASTE, SLIDE_OFF_SPEED, STEERINGS, claimClear, claimOf, columnOver, freeAmong, grown, guardedStride, headUnder, laneLift, liftOnto, meets, mustPoof, obstaclesOf, pushedOut, released, restsOn, scootFraction, scooted, shifted, slideOf, slideStride, spotOn, type Body as Solid } from "../🚧️clearance/🟦️.ts";
import { HOP_DISTANCE, HOP_HEIGHT, fallStep, hopLanding, hopOf, hopStep, landingOf, perchAt, strideTo } from "../🏞️terrain/🟦️.ts";
import { COMFORT_GAP, bodiesOf, clearway, extentAt, extentOf, obstaclesFor, soars, wallLeanOf } from "../📏️spacing/🟦️.ts";
import { clamp, smoothstep } from "../📐️trigonometry/🟦️.ts";
import { TURN_TICKS, actorKey, beatOf, clipAt, clipOf, facingTo, feel, hoverOf, indexOf, release, remove, settle, shift, turn, type Body, type Draft, type Launch } from "../📝️draft/🟦️.ts";
import { dwellOf } from "../🧠️behavior/🟦️.ts";
import { GRIP_BUDGET, GRIP_CLIMB, GRIP_HANG, HOOK_RETURN, HOOK_SPEED, LADDER_DISMOUNT_TICKS, LADDER_IDLE, LADDER_LIFE, LADDER_MOUNT_TICKS, LADDER_RAISE_TICKS, MUZZLE_FORWARD, MUZZLE_HEIGHT, ROPE_AIM_TICKS, ROPE_HOIST_TICKS, ROPE_MARGIN, ROPE_MISS_CHANCE, ROPE_REST, ROPE_SHRUG_TICKS, ROPE_TUG_TICKS, WALL_GRAB_TICKS, chainOf, clingOf, clings, footOf, gripFor, haulOf, haulStep, hoistPath, hookTicks, ladderAt, ladderExit, ladderLanding, ladderLength, ladderStep, ladderTo, landingFor, ledgeOf, missOf, rimFor, rimOf, routeOf, shotFor, sighted, slideStep, wallCost, wallPath, type LadderStand, type WallHold } from "../🧗️climbing/🟦️.ts";
import { CHUTE_HEADROOM, CHUTE_OPENING, CHUTE_REFLEX, HARD_LANDING, REEL_LEAST, canopyOf, chuteOf, chuteStep, chuteWind, hangOf, hangStep, leanOf } from "../🪢️swing/🟦️.ts";
import { LIFT_TICKS } from "../🪄️mischief/🟦️.ts";
import type { Post } from "../🗓️schedule/🟦️.ts";

//#region 🔖️Constants
const WAITING = 1920;
const LEAVE_REACH = 3;
const GLIDE_TICKS = 256;
const COURSE_TICKS = 1024;
const CLAIM_SPAN = 4;
const TUMBLE_TICKS = 24;
const WIND_PHASE = 0.37;
const OPEN_STIFFNESS = 220;
const OPEN_DAMPING = 12;
const DIZZY_SPEED = 780;
const RETURN_SPEED = 384;
const RETURN_LEAST = 12;
const LANE_RAMP = 8;
const TOSS_TOP = 16;
const TOSS_TICKS = 96;
const STEER_MOST = 240;
const BRACES = 4;
const LONGEST = 4096;
const REST_RETRY = 32;
const REST_LEAST = 128;
const REST_HALVINGS = 3;
const EXPLORE_SHARE = 0.5;
const MAX_LADDERS = 2;
const POST_UNITS: readonly number[] = [0, 0, 1];

/** 🏓️ The share of its speed a pet in flight keeps when it bounces off an edge of the stage — the left, the right or the top. */
export const BOUNCE = 0.5;

/** 🌫️ How many ticks the dust of a pet that vanished stays on the stage (0.75 s): its `Puff` is dropped then. */
export const PUFF_TICKS = 48;
//#endregion 🔖️Constants

//#region 🔖️Footing
/** 🎟️ A course and the claim of its corridor. */
export type Plan = { readonly course: Course; readonly claim: Claim };

/** 🧾️ The course of an owner, or `null`. */
export function courseOf(courses: readonly Course[], owner: Slug): Course | null {
  for (const course of courses) if (course.owner === owner) return course;
  return null;
}

/** 🧹️ The courses without the one of `owner`. */
function dropped(courses: readonly Course[], owner: Slug): Course[] {
  const kept: Course[] = [];
  for (const course of courses) if (course.owner !== owner) kept.push(course);
  return kept;
}

/** 🏷️ Whether an owner holds a claim. */
function claimed(claims: readonly Claim[], owner: Slug): boolean {
  for (const claim of claims) if (claim.owner === owner) return true;
  return false;
}

/** 🏡️ An actor that has nowhere left to be is gone at once (always `false`): it lets go of its partner and arrives anew, spaced from the others, once it is wanted and a perch has room. */
export function vanish(draft: Draft, index: number, now: Ticks): boolean {
  release(draft, index, now);
  remove(draft, index);
  return false;
}

/** 🌪️ An actor is gone from where it is in a puff of dust at tick `now`: the stage keeps the dust where its body was and counts the poof — every one, whether it is the last resort ({@link poof}), a fall through the bottom edge of the stage ({@link arrive}) or a slip over to a post its gear has no way to ({@link popTo}), so that how rare poofs are is measured on all of them. */
function dusted(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  draft.puffs = [...draft.puffs, { x: body.x, y: body.y - kind.size.height / 2, width: kind.size.width, height: kind.size.height, tick: now }];
  draft.poofs = draft.poofs + 1;
}

/** 💨️ The last resort: an actor that has no legal place left vanishes on the spot in a puff of dust ({@link dusted}; always `false`) — it is no body any more, lets go of its partner and arrives anew at a free place once it is wanted and a perch has room. */
export function poof(draft: Draft, index: number, now: Ticks): boolean {
  dusted(draft, index, now);
  return vanish(draft, index, now);
}

/** 🕳️ An actor loses its footing at tick `now` with the velocity `vx`, `vy`: it gives up its partner, its claim, its course and whatever its gear holds ({@link ungear}), takes up `activity` and plans its way down in its turn ({@link plunge}); a parachute that is out stays out, a tilt it has rights itself on the way. It may stay where it is for `DELAY` ticks while no way down is clear. */
export function unfoot(draft: Draft, index: number, vx: number, vy: number, activity: Activity, now: Ticks): void {
  const body = draft.actors[index]!;
  release(draft, index, now);
  draft.claims = released(draft.claims, body.species);
  draft.courses = dropped(draft.courses, body.species);
  ungear(draft, index, now);
  if (body.activity !== activity) {
    shift(draft, index, activity, now);
    body.clip = clipAt(draft.kinds[index]!, activity, 0);
  }
  body.footing = body.chute === null ? "air" : "chute";
  body.perch = null;
  body.host = null;
  body.hang = null;
  body.vx = vx;
  body.vy = vy;
  body.goal = body.x;
  body.until = now + DELAY;
}

/** ⬇️ An actor loses its perch and starts to fall. */
export function drop(draft: Draft, index: number, now: Ticks): void {
  unfoot(draft, index, 0, 0, "fall", now);
}

/** 🛫️ An actor takes a plan at tick `now`: its course and its claim are on the table, it is in the air (under its parachute when one is out) doing `activity` until the course ends, and goes for the course's last waypoint. */
export function install(draft: Draft, index: number, plan: Plan, activity: Activity, now: Ticks): void {
  const body = draft.actors[index]!;
  const course = plan.course;
  draft.claims = [...released(draft.claims, body.species), plan.claim];
  draft.courses = [...dropped(draft.courses, body.species), course];
  if (body.activity !== activity) {
    shift(draft, index, activity, now);
    body.clip = clipAt(draft.kinds[index]!, activity, 0);
  }
  body.footing = body.chute === null ? "air" : "chute";
  body.perch = null;
  body.host = null;
  body.hang = null;
  body.until = course.from + course.steps.length - 1;
  body.goal = course.steps[course.steps.length - 1]!.x;
}

/** 🤯️ Whether a course bumps its pet against an edge of the stage hard enough to leave it dizzy: one of its waypoints bounces off the left, the right or the top edge ({@link bounced}) while the pet flew into that edge at `DIZZY_SPEED` or faster. */
function bumped(draft: Draft, index: number, course: Course): boolean {
  for (let at = 1; at < course.steps.length; at++) {
    const before = course.steps[at - 1]!;
    const step = course.steps[at]!;
    if (bounced(draft, index, step, before.vx, before.vy) && (before.vx * step.vx < 0 ? Math.abs(before.vx) : Math.abs(before.vy)) >= DIZZY_SPEED) return true;
  }
  return false;
}

/** 🛬️ An actor comes to the end of its course at tick `now`: its claim and its course are done; at the bottom edge of the stage, or wherever a course ends `away`, it is gone in a puff of dust ({@link dusted}: not a last resort, but a poof the stage counts); on a head it stands and starts to slide off, and the head it landed on feels trampled; on a perch it stands — at rest at once after a parachute, else landing for the length of its landing clip (0.3 s when that loops or is missing) and, after a hard landing from far up or a hard bump against an edge of the stage on the way ({@link bumped}), dizzy afterwards for a moment ({@link daze}). A fall or a throw that ends is felt: floated down, landed, or dropped hard. `false` when the actor is gone. */
function arrive(draft: Draft, index: number, course: Course, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  draft.claims = released(draft.claims, body.species);
  draft.courses = dropped(draft.courses, body.species);
  if (course.ending === "away") {
    dusted(draft, index, now);
    return vanish(draft, index, now);
  }
  const flown = body.activity === "fall" || body.activity === "tumble" || body.activity === "glide";
  const floated = body.chute !== null;
  body.chute = null;
  body.tilt = 0;
  body.vx = 0;
  body.vy = 0;
  body.goal = body.x;
  if (course.ending === "head") {
    const host = indexOf(draft.actors, course.landing);
    shift(draft, index, "slide", now);
    body.clip = clipAt(kind, "slide", 0);
    body.footing = "head";
    body.perch = null;
    body.host = course.landing;
    body.until = now + dwellOf("slide", draft.mode, 0);
    body.vx = host >= 0 && body.x < draft.actors[host]!.x ? 0 - SLIDE_OFF_SPEED : SLIDE_OFF_SPEED;
    if (host >= 0) feel(draft, host, "trampled", now);
    return true;
  }
  body.footing = "perch";
  body.perch = course.landing;
  body.host = null;
  if (floated) {
    settle(draft, index, now);
    if (flown) feel(draft, index, "floated", now);
    return true;
  }
  shift(draft, index, "land", now);
  body.clip = clipAt(kind, "land", 0);
  const clip = clipOf(kind, body.clip);
  body.until = now + (clip !== null && !clip.loop ? clipTicks(clip) : dwellOf("land", draft.mode, 0));
  if (!flown) return true;
  const hard = course.touch > HARD_LANDING;
  body.vy = hard && course.touch >= DIZZY_SPEED ? course.touch : bumped(draft, index, course) ? DIZZY_SPEED : 0;
  feel(draft, index, hard ? "dropped" : "landed", now);
  return true;
}

/** 😵️ An actor that came down hard from far up is dizzy at tick `now` when its landing ends: for the shortest dwell of `dizzy`, with its dizzy clip. */
export function daze(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  shift(draft, index, "dizzy", now);
  body.clip = clipAt(draft.kinds[index]!, "dizzy", 0);
  body.until = now + dwellOf("dizzy", draft.mode, 0);
  body.vx = 0;
  body.vy = 0;
}
//#endregion 🔖️Footing

//#region 🔖️Courses
/** 🩳️ The perches as far as the feet of a body may go on them whose footprint reaches `half` to either side: a landing is never moved onto the end of a perch. */
function narrowed(perches: readonly Perch[], half: number): Perch[] {
  const fitting: Perch[] = [];
  for (const perch of perches) if (perch.x0 + half <= perch.x1 - half) fitting.push({ surface: perch.surface, x0: perch.x0 + half, x1: perch.x1 - half, y: perch.y });
  return fitting;
}

/** 🌀️ The tilt of a tumbling body `ticks` after it was let go with the tilt `tilt`: it rights itself, eased, over `TUMBLE_TICKS`. */
function tumbled(tilt: Turns, ticks: Ticks): Turns {
  return ticks >= TUMBLE_TICKS ? 0 : tilt * (1 - smoothstep(ticks / TUMBLE_TICKS));
}

/** 🪟️ How far the feet at `(x, y)` are above the first thing below them: a perch its footprint fits on, or the top of a body that stays where it is; `Infinity` over nothing. */
function headroom(perches: readonly Perch[], statics: readonly Solid[], x: number, y: number, hover: number): number {
  let least = Infinity;
  for (const perch of perches) if (perch.x0 <= x && x <= perch.x1 && perch.y - hover >= y && perch.y - hover - y < least) least = perch.y - hover - y;
  for (const body of statics) {
    const top = body.extent.y0 - hover - MARGIN;
    if (body.extent.x0 < x && x < body.extent.x1 && top >= y && top - y < least) least = top - y;
  }
  return least;
}

/** 🧭️ The course of an actor from `start` (where its feet are and how fast they move at the end of tick `from − 1`, its tilt and the canopy of its parachute, if one is out) through the air, tick by tick: falling with `fallStep` and drifting sideways at its speed — held inside the stage: the left, the right and the top edge are walls it bounces off, keeping `BOUNCE` of its speed (a canopy only stops at the sides) — until, `decision` ticks in (never when −1), it decides to open its parachute; `CHUTE_REFLEX` ticks later the canopy opens and the pet sinks under it with `chuteStep`, steering for `target`, swaying in a wind of its own and flaring over the last bit above the ground. A tumbling drawing rights itself. It ends on the first perch its footprint comes down on or on the first head it comes down on (`headUnder` among `statics`, the bodies that stay where they are), whichever is higher, upright and at rest there; with its feet on the bottom edge of the stage when it would fall through it — so its body never leaves the stage —, or after `COURSE_TICKS`, `away`. */
function charted(draft: Draft, index: number, start: Waypoint, from: Ticks, decision: Ticks, target: number, statics: readonly Solid[]): Course {
  const kind = draft.kinds[index]!;
  const owner = draft.actors[index]!.species;
  const half = kind.size.width / 2;
  const hover = hoverOf(kind);
  const chute = chuteOf(kind.size.height);
  const phase = WIND_PHASE * draft.streams[index]!;
  const perches = narrowed(draft.perches, half);
  const steps: Waypoint[] = [];
  let x = start.x;
  let y = start.y;
  let vx = start.vx;
  let vy = start.vy;
  let canopy: Canopy | null = start.canopy;
  let before = extentAt(owner, kind, start, start.tilt, canopy === null ? null : { x: canopy.x, y: canopy.y });
  for (let tick = 0; tick < COURSE_TICKS; tick++) {
    if (canopy === null && decision >= 0 && tick >= decision + CHUTE_REFLEX) canopy = canopyOf({ x, y }, vx, vy, chute);
    let nextX: number;
    let nextY: number;
    let nextVx: number;
    let nextVy: number;
    let next: Canopy | null = null;
    if (canopy !== null) {
      next = chuteStep(canopy, chute, target, headroom(perches, statics, x, y, hover), chuteWind(from + tick, phase));
      nextX = next.bob.x;
      nextY = next.bob.y;
      nextVx = (nextX - x) * TICKS_PER_SECOND;
      nextVy = (nextY - y) * TICKS_PER_SECOND;
    } else {
      const fallen = fallStep(y, vy);
      nextX = x + vx / TICKS_PER_SECOND;
      nextY = fallen.y;
      nextVx = vx;
      nextVy = fallen.vy;
    }
    const held = clamp(nextX, half, draft.width - half);
    if (held !== nextX) {
      const push = held - nextX;
      nextX = held;
      nextVx = next === null ? 0 - nextVx * BOUNCE : 0;
      if (next !== null) next = { x: next.x + push, y: next.y, vx: 0, vy: next.vy, bob: { x: next.bob.x + push, y: next.bob.y }, previous: { x: next.previous.x + push, y: next.previous.y } };
    }
    if (next === null && nextY - kind.size.height < 0) {
      nextY = kind.size.height;
      if (nextVy < 0) nextVy = 0 - nextVy * BOUNCE;
    }
    const tilt = tumbled(start.tilt, tick + 1);
    const after = extentAt(owner, kind, { x: nextX, y: nextY }, tilt, next === null ? null : { x: next.x, y: next.y });
    const perch = landingOf(perches, nextX, y + hover, nextY + hover);
    const host = headUnder(before, after, owner, statics);
    if (perch !== null || host !== null) {
      const onPerch = perch === null ? Infinity : perch.y - hover;
      const onHead = host === null ? Infinity : nextY + liftOnto(extentAt(owner, kind, { x: nextX, y: nextY }, 0, null), host.extent);
      const head = host !== null && onHead <= onPerch;
      steps.push({ x: nextX, y: head ? onHead : onPerch, vx: 0, vy: 0, tilt: 0, canopy: null });
      return { owner, from, steps, ending: head ? "head" : "perch", landing: head ? host!.owner : perch!.surface, touch: nextVy };
    }
    if (nextY > draft.height) {
      steps.push({ x: nextX, y: draft.height, vx: nextVx, vy: nextVy, tilt, canopy: next });
      return { owner, from, steps, ending: "away", landing: "", touch: nextVy };
    }
    steps.push({ x: nextX, y: nextY, vx: nextVx, vy: nextVy, tilt, canopy: next });
    x = nextX;
    y = nextY;
    vx = nextVx;
    vy = nextVy;
    canopy = next;
    before = after;
  }
  return { owner, from, steps, ending: "away", landing: "", touch: vy };
}

/** 🪂️ When a pet that owns a parachute decides to open it, on a course charted without one: never when it does not own one, when a canopy is out already, when the course leaves the stage or lands no harder than `HARD_LANDING`; else at the first tick at which it falls at `CHUTE_OPENING` or faster with `CHUTE_HEADROOM` or more above where it lands, or — thrown up so close to its landing that it never falls that fast with that much room — at the first tick at which it no longer rises with that much room; never when there is no such tick (it was thrown down at the ground from too close: it brakes instead, {@link wayDown}). The decision is taken tick by tick on the landing the course really makes, not on a prediction. −1 for never. */
function decisionOf(kind: Species, start: Waypoint, plain: Course): Ticks {
  if (!kind.gear.includes("parachute") || start.canopy !== null || plain.ending === "away" || !(plain.touch > HARD_LANDING)) return -1;
  const ground = plain.steps[plain.steps.length - 1]!.y;
  for (let tick = 0; tick < plain.steps.length; tick++) {
    const state = tick === 0 ? start : plain.steps[tick - 1]!;
    if (state.vy >= CHUTE_OPENING && ground - state.y >= CHUTE_HEADROOM) return tick;
  }
  for (let tick = 0; tick < plain.steps.length; tick++) {
    const state = tick === 0 ? start : plain.steps[tick - 1]!;
    if (state.vy >= 0 && ground - state.y >= CHUTE_HEADROOM) return tick;
  }
  return -1;
}

/** 🎫️ The plan of a course: the course and the claim of its corridor, `CLAIM_SPAN` ticks to a slice, resting on its last waypoint (nowhere when it ends `away`). */
export function planOf(draft: Draft, index: number, course: Course): Plan {
  const kind = draft.kinds[index]!;
  const extents: Extent[] = [];
  for (const step of course.steps) extents.push(extentAt(course.owner, kind, step, step.tilt, step.canopy === null ? null : { x: step.canopy.x, y: step.canopy.y }));
  return { course, claim: claimOf(course.owner, course.from, extents, CLAIM_SPAN, course.ending === "away" ? null : extents[extents.length - 1]!) };
}

/** 🍂️ The plan of a way down from `start` beginning at tick `from`: the course as charted and, when the pet would land hard where its parachute could help, the course with the parachute opened at the tick of {@link decisionOf}, steering for where the plain course lands (`target` is what a canopy that is out already steers for). A pet that owns a parachute never lands harder than `HARD_LANDING`: when even so it would — thrown down at the ground too close to it for the canopy to open in time —, it brakes at once and keeps only half of its downward speed, again and again for `BRACES` rounds, and at last none, until it comes down softly. */
function wayDown(draft: Draft, index: number, start: Waypoint, from: Ticks, statics: readonly Solid[], target: number): Plan {
  const kind = draft.kinds[index]!;
  let begin = start;
  for (let brace = 0; ; brace++) {
    const plain = charted(draft, index, begin, from, -1, target, statics);
    const decision = decisionOf(kind, begin, plain);
    const course = decision < 0 ? plain : charted(draft, index, begin, from, decision, plain.steps[plain.steps.length - 1]!.x, statics);
    if (brace >= BRACES || !kind.gear.includes("parachute") || begin.canopy !== null || course.ending === "away" || !(course.touch > HARD_LANDING) || !(begin.vy > 0)) return planOf(draft, index, course);
    begin = { x: begin.x, y: begin.y, vx: begin.vx, vy: brace + 1 >= BRACES ? 0 : begin.vy / 2, tilt: begin.tilt, canopy: null };
  }
}

/** 🧗️ The ways down of an actor from where it is, beginning at tick `from`, in the order tried, and the first one whose claim is clear (`null` when none is): as it flies; towards the free column over the perch it would land on; with the air control of `STEERINGS` added to its sideways speed; and, when it was thrown, bounced back at half its sideways speed and simply let go with the air control. Under an open parachute it steers for where it would land and for the places `STEERINGS` pixels to either side. Only bodies that stand on a perch without a plan serve as heads. */
export function waysDown(draft: Draft, index: number, from: Ticks): { readonly chosen: Plan | null; readonly plans: readonly Plan[] } {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const bodies = bodiesOf(draft.actors, draft.kinds);
  const statics: Solid[] = [];
  for (let other = 0; other < draft.actors.length; other++) if (other !== index && draft.actors[other]!.footing === "perch" && !claimed(draft.claims, draft.actors[other]!.species)) statics.push(bodies[other]!);
  const start: Waypoint = { x: body.x, y: body.y, vx: body.vx, vy: body.vy, tilt: body.tilt, canopy: body.chute === null ? null : body.chute.canopy };
  const plans: Plan[] = [];
  const straight = wayDown(draft, index, start, from, statics, body.x);
  plans.push(straight);
  if (claimClear(straight.claim, bodies, draft.claims)) return { chosen: straight, plans };
  const tries: Waypoint[] = [];
  const targets: number[] = [];
  if (start.canopy !== null) {
    for (let at = 1; at < STEERINGS.length; at++) {
      tries.push(start);
      targets.push(body.x + STEERINGS[at]!);
    }
  } else {
    const course = straight.course;
    const end = course.steps[course.steps.length - 1]!;
    const perch = course.ending === "perch" ? perchAt(draft.perches, course.landing, end.x) : null;
    if (perch !== null) {
      const column = columnOver(perch, body.x, extentAt(body.species, kind, { x: body.x, y: body.y }, 0, null), perch.y - hoverOf(kind) - body.y, kind.size.width / 2, obstaclesOf(body.species, bodies, draft.claims, from));
      const steer = column === null ? 0 : clamp(((column - body.x) * TICKS_PER_SECOND) / course.steps.length, 0 - STEER_MOST, STEER_MOST);
      if (steer !== 0) {
        tries.push({ x: start.x, y: start.y, vx: body.vx + steer, vy: start.vy, tilt: start.tilt, canopy: null });
        targets.push(body.x);
      }
    }
    for (let at = 1; at < STEERINGS.length; at++) {
      tries.push({ x: start.x, y: start.y, vx: body.vx + STEERINGS[at]!, vy: start.vy, tilt: start.tilt, canopy: null });
      targets.push(body.x);
    }
    if (body.vx !== 0 || body.vy !== 0) {
      tries.push({ x: start.x, y: start.y, vx: 0 - body.vx / 2, vy: start.vy, tilt: start.tilt, canopy: null });
      targets.push(body.x);
      for (const steer of STEERINGS) {
        tries.push({ x: start.x, y: start.y, vx: steer, vy: 0, tilt: start.tilt, canopy: null });
        targets.push(body.x);
      }
    }
  }
  for (let at = 0; at < tries.length; at++) {
    const plan = wayDown(draft, index, tries[at]!, from, statics, targets[at]!);
    plans.push(plan);
    if (claimClear(plan.claim, bodies, draft.claims)) return { chosen: plan, plans };
  }
  return { chosen: null, plans };
}
//#endregion 🔖️Courses

//#region 🔖️Departure
/** 🥾️ Where a walk from `x` towards `goal` really ends: a hopping gait covers ground in whole hops of its clip (speed × the length of the clip), as many as fit before the goal; every other gait goes all the way. */
export function paced(kind: Species, clip: Slug | null, x: number, goal: number): number {
  const beat = beatOf(kind, clipOf(kind, clip));
  if (beat === 1) return goal;
  const hop = (Math.max(kind.locomotion.speed, 0) * beat) / TICKS_PER_SECOND;
  if (!(hop > 0)) return x;
  const hops = Math.floor(Math.abs(goal - x) / hop);
  return goal >= x ? x + hops * hop : x - hops * hop;
}

/** 🚶️ An actor sets out for a goal on its perch; it turns towards it first when it faces the other way. */
export function stroll(draft: Draft, index: number, goal: number, clip: Slug | null, now: Ticks): void {
  const body = draft.actors[index]!;
  shift(draft, index, "walk", now);
  body.goal = goal;
  body.until = now + dwellOf("walk", draft.mode, 0);
  body.clip = clip;
}

/** 🧍️ An actor waits for its partner: idle, turning towards it, for as long as the stage has patience. */
export function attend(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  shift(draft, index, "idle", now);
  body.until = now + WAITING;
  body.clip = clipAt(draft.kinds[index]!, "idle", 0);
  body.goal = body.x;
}

/** 🦘️ Where an actor could hop to: per other perch (in perch order) the free spot nearest to the place nearest to it that keeps half a body from the ends — free of every body, corridor and landing spot on stage —, when `hopOf` grants the hop, the flight really ends on that perch and its arc covers nothing that must stay free ({@link soars}); a floating gait glides there in a straight line at twice its speed when the place is within the reach of a hop and four seconds. Whether the corridor of the hop is clear is asked when it is taken ({@link launch}). */
export function hopsOf(draft: Draft, index: number): Launch[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const half = kind.size.width / 2;
  const hover = hoverOf(kind);
  const obstacles = obstaclesFor(draft, index, draft.tick);
  const launches: Launch[] = [];
  for (const perch of draft.perches) {
    if (perch.surface === body.perch && perch.x0 <= body.x && body.x <= perch.x1) continue;
    const low = perch.x0 + half;
    const high = perch.x1 - half;
    if (high < low) continue;
    const inset = Math.min(half, (high - low) / 2);
    const aimed = clamp(body.x, low + inset, high - inset);
    const x = spotOn(perch, aimed, extentAt(body.species, kind, { x: aimed, y: perch.y - hover }, 0, null), half, obstacles);
    if (x === null) continue;
    if (kind.locomotion.gait === "float") {
      const dx = x - body.x;
      const dy = perch.y - hover - body.y;
      if (Math.abs(dx) > HOP_DISTANCE || Math.abs(dy) > HOP_HEIGHT) continue;
      const ticks = Math.floor((Math.sqrt(dx * dx + dy * dy) * TICKS_PER_SECOND) / (2 * Math.max(kind.locomotion.speed, 1)) + 0.5);
      if (ticks < 1 || ticks > GLIDE_TICKS) continue;
      launches.push({ x, y: perch.y - hover, surface: perch.surface, vx: (dx * TICKS_PER_SECOND) / ticks, vy: (dy * TICKS_PER_SECOND) / ticks, ticks });
      continue;
    }
    const from = { x: body.x, y: body.y };
    const to = { x, y: perch.y };
    const hop = hopOf(from, to);
    if (hop === null) continue;
    const landing = hopLanding(draft.perches, from, to, hop);
    if (landing === null || landing.surface !== perch.surface || landing.x0 !== perch.x0 || !soars(draft, kind, from, to, hop)) continue;
    launches.push({ x, y: perch.y, surface: perch.surface, vx: hop.vx, vy: hop.vy, ticks: hop.ticks });
  }
  return launches;
}

/** 🚀️ An actor takes a hop it chose at tick `now` (a launch of {@link hopsOf}) with `clip`, from the next tick on: a hopping or walking gait flies the arc of `hopStep`, a floating gait glides straight at its own altitude or, when that corridor is not clear, in its high lane (`laneLift`), eased in and out over `LANE_RAMP` ticks. Only a course whose claim is clear is taken; `false` when none is (the actor stays where it is). */
export function launch(draft: Draft, index: number, chosen: Launch, clip: Slug | null, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const owner = body.species;
  const courses: Course[] = [];
  if (kind.locomotion.gait === "float") {
    for (const lane of [0, laneLift(kind.size.height, MARGIN)]) {
      const steps: Waypoint[] = [];
      for (let tick = 1; tick <= chosen.ticks; tick++) {
        const ramp = Math.min(1, tick / LANE_RAMP, (chosen.ticks - tick) / LANE_RAMP);
        steps.push(tick === chosen.ticks ? { x: chosen.x, y: chosen.y, vx: 0, vy: 0, tilt: 0, canopy: null } : { x: body.x + ((chosen.x - body.x) * tick) / chosen.ticks, y: body.y + ((chosen.y - body.y) * tick) / chosen.ticks - lane * ramp, vx: chosen.vx, vy: chosen.vy, tilt: 0, canopy: null });
      }
      courses.push({ owner, from: now + 1, steps, ending: "perch", landing: chosen.surface, touch: 0 });
    }
  } else {
    const steps: Waypoint[] = [];
    const to = { x: chosen.x, y: chosen.y };
    let x = body.x;
    let y = body.y;
    let vy = chosen.vy;
    for (let left = chosen.ticks; left >= 1; left--) {
      const flight = hopStep(x, y, chosen.vx, vy, to, left);
      steps.push({ x: flight.x, y: flight.y, vx: flight.vx, vy: flight.vy, tilt: 0, canopy: null });
      x = flight.x;
      y = flight.y;
      vy = flight.vy;
    }
    courses.push({ owner, from: now + 1, steps, ending: "perch", landing: chosen.surface, touch: vy });
  }
  const bodies = bodiesOf(draft.actors, draft.kinds);
  for (const course of courses) {
    const plan = planOf(draft, index, course);
    if (!claimClear(plan.claim, bodies, draft.claims)) continue;
    install(draft, index, plan, "hop", now);
    body.clip = clip;
    return true;
  }
  return false;
}
//#endregion 🔖️Departure

//#region 🔖️Motion
/** 🛡️ How far an actor may move sideways this tick when it wants to move `wanted` px: `guardedStride` against everybody else, every corridor that is left and every landing spot. */
function guarded(draft: Draft, index: number, wanted: number, now: Ticks): number {
  return guardedStride(extentOf(draft.actors[index]!, draft.kinds[index]!), wanted, obstaclesFor(draft, index, now));
}

/** 👣️ One tick of a walk, guarded. On every beat — every tick for a walking or floating gait, the start of every hop for a hopping one — the walk ends when the goal is no farther than one stride (the actor steps onto it as far as nobody is in the way) or when somebody or something planned is in the way of the next beat; otherwise a stride towards the goal follows, as far as nothing is in its way. A hopping gait held up in mid-hop finishes that hop on the spot. The walk also ends when the stage loses patience. At its end a leaver starts to fade, an actor with a partner waits for it and anyone else comes to rest. Nobody walks into or past anybody whose height it shares. */
export function stride(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const beat = beatOf(kind, clipOf(kind, body.clip));
  const step = Math.max(kind.locomotion.speed, 0) / TICKS_PER_SECOND;
  let done = now >= body.until;
  if (!done && (now - body.since) % beat === 0) {
    const gap = body.goal - body.x;
    if (Math.abs(gap) <= step) {
      const allowed = guarded(draft, index, gap, now);
      body.x = allowed === gap ? body.goal : body.x + allowed;
      done = true;
    } else {
      const far = step * beat;
      const ahead = clamp(body.goal, body.x - far, body.x + far) - body.x;
      done = guarded(draft, index, ahead, now) !== ahead;
    }
  }
  if (!done) {
    const next = strideTo(body.x, body.goal, kind.locomotion.speed);
    const wanted = next - body.x;
    const allowed = guarded(draft, index, wanted, now);
    if (allowed === wanted) {
      body.x = next;
      return;
    }
    body.x = body.x + allowed;
    body.goal = body.x;
    if (beat > 1) return;
  }
  if (body.leaving) {
    shift(draft, index, "idle", now);
    body.clip = clipAt(kind, "idle", 0);
    body.goal = body.x;
    return;
  }
  if (body.partner !== null) {
    attend(draft, index, now);
    return;
  }
  settle(draft, index, now);
}

/** 🚌️ The scooters of every surface move to their seats together at tick `now` (a survey re-seated them): each covers the one share of its way that the longest way allows at `SCOOT_HASTE` times the walking speed of the slowest of them (`scootFraction`), and only when every new place is free of everybody outside the group, every corridor that is left and every landing spot — two valid seatings mixed are valid, so the group never overlaps within itself. Arrived, they come to rest. A group held up past its patience gives up: whoever stands on its perch rests where it is, whoever does not falls. */
export function scoot(draft: Draft, now: Ticks): void {
  const surfaces: string[] = [];
  for (const body of draft.actors) if (body.activity === "scoot" && body.footing === "perch" && body.perch !== null && !surfaces.includes(body.perch)) surfaces.push(body.perch);
  for (const surface of surfaces) {
    const members: number[] = [];
    for (let index = 0; index < draft.actors.length; index++) if (draft.actors[index]!.activity === "scoot" && draft.actors[index]!.footing === "perch" && draft.actors[index]!.perch === surface) members.push(index);
    if (members.length === 0) continue;
    const shifts: number[] = [];
    let slowest = Infinity;
    for (const member of members) {
      shifts.push(draft.actors[member]!.goal - draft.actors[member]!.x);
      slowest = Math.min(slowest, Math.max(draft.kinds[member]!.locomotion.speed, 1));
    }
    const fraction = scootFraction(shifts, (SCOOT_HASTE * slowest) / TICKS_PER_SECOND);
    const others: Solid[] = [];
    const bodies = bodiesOf(draft.actors, draft.kinds);
    for (let index = 0; index < bodies.length; index++) if (!members.includes(index)) others.push(bodies[index]!);
    const obstacles = obstaclesOf("", others, draft.claims, now);
    let free = true;
    const places: number[] = [];
    for (const member of members) {
      const body = draft.actors[member]!;
      const place = scooted(body.x, body.goal, fraction);
      places.push(place);
      if (!freeAmong(shifted(bodies[member]!.extent, place - body.x, 0), obstacles)) free = false;
    }
    if (free) {
      for (let at = 0; at < members.length; at++) draft.actors[members[at]!]!.x = places[at]!;
      if (fraction >= 1) for (const member of members) settle(draft, member, now);
      continue;
    }
    if (now < draft.actors[members[0]!]!.until) continue;
    for (let at = members.length - 1; at >= 0; at--) {
      const member = members[at]!;
      const body = draft.actors[member]!;
      if (perchAt(draft.perches, surface, body.x) !== null) settle(draft, member, now);
      else drop(draft, member, now);
    }
  }
}

/** 🛷️ One tick of an actor on a head: while its host still carries it (`restsOn`) it slides off to its side (the sign of `vx`), guarded and between the edges of the stage, faster tick by tick (`slideStride` since `since`), turning round when it cannot move; once its host no longer carries it it falls with the speed of its slide. `false` when the actor is gone. */
export function slip(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const host = body.host === null ? -1 : indexOf(draft.actors, body.host);
  const box = extentOf(body, kind);
  if (host < 0 || !restsOn(box, extentOf(draft.actors[host]!, draft.kinds[host]!))) {
    unfoot(draft, index, body.vx, 0, "fall", now);
    return plunge(draft, index, now);
  }
  const side = body.vx < 0 ? -1 : 1;
  const ticks = now - body.since;
  const slid = slideOf(box, side, ticks, 0, draft.width, obstaclesFor(draft, index, now));
  body.x = body.x + slid.stride;
  body.goal = body.x;
  if (slid.side === side) body.vx = side * slideStride(ticks) * TICKS_PER_SECOND;
  else {
    body.since = now;
    body.vx = slid.side * SLIDE_OFF_SPEED;
  }
  return true;
}

/** 🤕️ Whether a waypoint of free flight (no canopy) reached from the speed `vx`, `vy` bounces off an edge of the stage: it stands at a side edge with its sideways speed turned round, or with its body against the top edge and its rise turned into a fall. Free flight changes its sideways speed nowhere else. */
function bounced(draft: Draft, index: number, step: Waypoint, vx: number, vy: number): boolean {
  const kind = draft.kinds[index]!;
  const half = kind.size.width / 2;
  if (step.canopy !== null) return false;
  return ((step.x === half || step.x === draft.width - half) && vx * step.vx < 0) || (step.y === kind.size.height && vy < 0 && step.vy >= 0);
}

/** 🛩️ One tick of an actor in the air: along its course (its feet, speed and tilt; its parachute opens where the course opens it and springs open, `OPEN_STIFFNESS`, `OPEN_DAMPING`; where it bounces off an edge of the stage it feels the bump, `bonked`), and on its last waypoint it arrives; without a course it plans its way down ({@link plunge}). `false` when the actor is gone. */
export function fly(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const course = courseOf(draft.courses, body.species);
  if (course === null) return plunge(draft, index, now);
  const last = course.steps.length - 1;
  const at = now - course.from;
  if (at < 0) return true;
  const step = course.steps[at > last ? last : at]!;
  if (at <= last && bounced(draft, index, step, body.vx, body.vy)) feel(draft, index, "bonked", now);
  body.x = step.x;
  body.y = step.y;
  body.vx = step.vx;
  body.vy = step.vy;
  body.tilt = step.tilt;
  if (step.canopy !== null) {
    const chute = body.chute;
    if (chute === null) {
      body.chute = { since: now, open: 0, opening: 0, canopy: step.canopy };
      body.footing = "chute";
      if (body.activity !== "glide") {
        shift(draft, index, "glide", now);
        body.clip = clipAt(draft.kinds[index]!, "glide", 0);
      }
    } else {
      const spring = springStep(chute.open, chute.opening, 1, OPEN_STIFFNESS, OPEN_DAMPING);
      body.chute = { since: chute.since, open: spring.position, opening: spring.velocity, canopy: step.canopy };
    }
  }
  if (at < last) return true;
  return arrive(draft, index, course, now);
}

/** 🌬️ An actor without footing or course plans its way down at tick `now` ({@link waysDown}) and takes the first clear one at once; while none is clear it stays where it is until `until` (a wait), as long as that place is free; then, with nothing legal left, it vanishes in a puff (`mustPoof`, {@link poof}). `false` when the actor is gone. */
export function plunge(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const way = waysDown(draft, index, now);
  if (way.chosen !== null) {
    install(draft, index, way.chosen, body.activity, now);
    return fly(draft, index, now);
  }
  const stay = now < body.until ? extentOf(body, draft.kinds[index]!) : null;
  if (!mustPoof(body.species, stay, [], bodiesOf(draft.actors, draft.kinds), draft.claims, now)) return true;
  return poof(draft, index, now);
}
//#endregion 🔖️Motion

//#region 🔖️Hand
/** 🚦️ Whoever planned to pass or to come to rest where the body of an actor now is — counting on it to be gone by then — gives its plan up at tick `now`, because the hand took that actor out of its own plan: a flier plans anew in its turn, with the speed it has, from where it is, keeping its activity; a traveller with its gear gives its trip up ({@link halt}) and, on its perch, comes to rest. */
function yielded(draft: Draft, index: number, now: Ticks): void {
  const owner = draft.actors[index]!.species;
  const box = extentOf(draft.actors[index]!, draft.kinds[index]!);
  for (let other = 0; other < draft.actors.length; other++) {
    const flier = draft.actors[other]!;
    if (other === index) continue;
    const own: Claim[] = [];
    for (const claim of draft.claims) if (claim.owner === flier.species) own.push(claim);
    if (own.length === 0 || freeAmong(box, obstaclesOf(owner, [], own, now))) continue;
    if (tripOf(draft.trips, flier.species) === null) unfoot(draft, other, flier.vx, flier.vy, flier.activity, now);
    else {
      halt(draft, other, now);
      if (flier.footing === "perch") settle(draft, other, now);
    }
  }
}

/** ✋️ The learner — or a deed of the learner (`toss`) — picks an actor up at tick `now`: it lets go of its partner, of its claim and course, of whatever carried it and whatever its gear held ({@link ungear}), and hangs by its scruff (`hangOf` with the species' `grip` as the rod) in the hand (footing `hand`, activity `hang`); where its feet stood when it was picked up from a perch is remembered (`origin`), and the pointer's trail starts anew. Whoever planned to pass where it hangs plans anew ({@link yielded}). */
export function lift(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  release(draft, index, now);
  draft.claims = released(draft.claims, body.species);
  draft.courses = dropped(draft.courses, body.species);
  ungear(draft, index, now);
  draft.origin = body.footing === "perch" ? { x: body.x, y: body.y } : null;
  draft.trail = [];
  if (body.activity !== "hang") shift(draft, index, "hang", now);
  body.clip = clipAt(kind, "hang", 0);
  body.footing = "hand";
  body.perch = null;
  body.host = null;
  body.chute = null;
  body.tilt = 0;
  body.hang = hangOf({ x: body.x, y: body.y }, kind.grip);
  body.vx = 0;
  body.vy = 0;
  body.goal = body.x;
  body.until = now + dwellOf("hang", draft.mode, 0);
  yielded(draft, index, now);
}

/** 🦧️ One tick of an actor in the hand at tick `now`: its grip follows `target` (held inside the stage) and its body swings below it (`hangStep`), tilted by `leanOf`; a body dragged into somebody or into a corridor or landing spot is pushed out of it (`pushedOut`, grip and feet together); when that fails, or would leave the stage (its drawing, or its upright box at its feet), it keeps its place and stops swinging. */
export function hold(draft: Draft, index: number, target: Point, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const hang = body.hang;
  if (hang === null) return;
  const next = hangStep(hang, { x: clamp(target.x, 0, draft.width), y: clamp(target.y, 0, draft.height) }, kind.grip);
  const tilt = leanOf(next.grip, next.bob, kind.grip);
  const box = extentAt(body.species, kind, next.bob, tilt, null);
  const push = pushedOut(box, obstaclesFor(draft, index, now), PUSHES);
  const moved = push === null ? null : shifted(box, push.x, push.y);
  const half = kind.size.width / 2;
  if (push === null || moved === null || moved.x0 + MARGIN < 0 || moved.x1 - MARGIN > draft.width || moved.y0 + MARGIN < 0 || moved.y1 - MARGIN > draft.height || next.bob.x + push.x - half < 0 || next.bob.x + push.x + half > draft.width || next.bob.y + push.y - kind.size.height < 0 || next.bob.y + push.y > draft.height) {
    body.hang = { grip: { x: hang.grip.x, y: hang.grip.y, vx: 0, vy: 0 }, bob: hang.bob, previous: hang.bob };
    return;
  }
  body.hang = { grip: { x: next.grip.x + push.x, y: next.grip.y + push.y, vx: next.grip.vx, vy: next.grip.vy }, bob: { x: next.bob.x + push.x, y: next.bob.y + push.y }, previous: { x: next.previous.x + push.x, y: next.previous.y + push.y } };
  body.x = next.bob.x + push.x;
  body.y = next.bob.y + push.y;
  body.tilt = tilt;
}

/** 🎈️ The hand lets an actor go at tick `now` with the velocity `velocity` (`releaseVelocity` of the pointer's trail, or nothing): it tumbles through the air, righting itself, and plans its way down in its turn — with air control towards a free spot, and its parachute where it would land hard. */
export function letGo(draft: Draft, index: number, velocity: Point, now: Ticks): void {
  draft.trail = [];
  unfoot(draft, index, velocity.x, velocity.y, "tumble", now);
}

/** ↩️ A pick-up is called off at tick `now` (Escape, a lost pointer, a hidden page): the actor glides back, straight and eased, to the free spot nearest to where it was picked up on that perch, at `RETURN_SPEED` and in no fewer than `RETURN_LEAST` ticks, righting itself on the way — on a corridor whose claim is clear; else, or when it was not picked up from a perch, it is simply let go. */
export function giveBack(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const origin = draft.origin;
  draft.origin = null;
  const hover = hoverOf(kind);
  let perch: Perch | null = null;
  if (origin !== null) for (const candidate of draft.perches) if (perch === null && candidate.y - hover === origin.y && candidate.x0 <= origin.x && origin.x <= candidate.x1) perch = candidate;
  const spot = origin === null || perch === null ? null : spotOn(perch, origin.x, extentAt(body.species, kind, origin, 0, null), kind.size.width / 2, obstaclesFor(draft, index, now));
  if (origin === null || perch === null || spot === null) {
    letGo(draft, index, { x: 0, y: 0 }, now);
    return;
  }
  const dx = spot - body.x;
  const dy = origin.y - body.y;
  const ticks = Math.max(Math.floor((Math.sqrt(dx * dx + dy * dy) * TICKS_PER_SECOND) / RETURN_SPEED) + 1, RETURN_LEAST);
  const steps: Waypoint[] = [];
  for (let tick = 1; tick <= ticks; tick++) {
    const share = smoothstep(tick / ticks);
    steps.push(tick === ticks ? { x: spot, y: origin.y, vx: 0, vy: 0, tilt: 0, canopy: null } : { x: body.x + dx * share, y: body.y + dy * share, vx: 0, vy: 0, tilt: body.tilt * (1 - share), canopy: null });
  }
  const plan = planOf(draft, index, { owner: body.species, from: now + 1, steps, ending: "perch", landing: perch.surface, touch: 0 });
  if (!claimClear(plan.claim, bodiesOf(draft.actors, draft.kinds), draft.claims)) {
    letGo(draft, index, { x: 0, y: 0 }, now);
    return;
  }
  draft.trail = [];
  release(draft, index, now);
  install(draft, index, plan, "glide", now);
}

/** 🎯️ Where the hand of a toss carries an actor at the top of the stage: above where it was picked up, its body `TOSS_TOP` px below the top edge. */
export function tossTarget(draft: Draft, index: number): Point {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  return { x: body.goal, y: TOSS_TOP + kind.size.height - kind.grip };
}

/** 🎾️ The learner tosses an actor with a deed at tick `now` (the keyboard's way to see a parachute): it is picked up like by the hand ({@link lift}), carried to the top of the stage ({@link tossTarget}) for `TOSS_TICKS` and let go there without a throw. */
export function toss(draft: Draft, index: number, now: Ticks): void {
  lift(draft, index, now);
  draft.origin = null;
  draft.actors[index]!.until = now + TOSS_TICKS;
}
//#endregion 🔖️Hand

//#region 🔖️Gear
/** 🎒️ Whether a species gets around with gear: it owns what it takes to climb, a ladder or a grappling gun, and it does not float — a floater changes its lanes instead. */
export function geared(kind: Species): boolean {
  return kind.locomotion.gait !== "float" && (kind.gear.includes("climb") || kind.gear.includes("ladder") || kind.gear.includes("grapple"));
}

/** 🗻️ The trip of an owner, or `null`. */
export function tripOf(trips: readonly Trip[], owner: Slug): Trip | null {
  for (const trip of trips) if (trip.owner === owner) return trip;
  return null;
}

/** 🧳️ The trips without the one of `owner`. */
function unpacked(trips: readonly Trip[], owner: Slug): Trip[] {
  const kept: Trip[] = [];
  for (const trip of trips) if (trip.owner !== owner) kept.push(trip);
  return kept;
}

/** 🪺️ Where on a perch the feet of a body `half` wide keep the whole body on it, `[x0 + half, x1 − half]`; `null` on a perch narrower than the body. */
function roomOn(perch: Perch, half: number): readonly [number, number] | null {
  const low = perch.x0 + half;
  const high = perch.x1 - half;
  return low <= high ? [low, high] : null;
}

/** ⛺️ A trip as planned, with what taking it sets up: the ladder it raises (`null`: none) and the tick that ladder stands up, and the grappling rope it fires (`null`: none). */
export type Outing = { readonly trip: Trip; readonly raise: LadderStand | null; readonly stands: Ticks; readonly rope: Rope | null };

/** 🧿️ Where an actor can set out for with its gear: a perch (wherever its way arrives on it) or a spot on a wall — a pitch and the height of the feet there, or, with `null`, a height the stage draws. */
export type Venture = { readonly kind: "perch"; readonly perch: Perch } | { readonly kind: "wall"; readonly pitch: Pitch; readonly y: number | null };

/** 💸️ The grip a foothold of a trip costs: `GRIP_HANG` sliding down a wall, `GRIP_CLIMB` for anything else on a wall, nothing off a wall. */
function effortOf(step: Foothold): number {
  return step.footing !== "wall" ? 0 : step.activity === "slide" ? GRIP_HANG : GRIP_CLIMB;
}

/** 🔋️ What is left at tick `now` of the grip of an actor that rests on a wall: hanging still costs `GRIP_HANG` per tick since it began to rest (`since`), never below nothing. */
function gripNow(body: Body, now: Ticks): number {
  return Math.max(body.grip - GRIP_HANG * (now - body.since), 0);
}

/** 🧩️ Where a leg of a trip sets out from: the perch an actor stands on, where its feet are there, the way it faces and the grip it has — the actor itself before its first leg, where the leg before put it before every other. */
export type Stand = { readonly perch: Perch; readonly x: number; readonly y: number; readonly facing: 1 | -1; readonly grip: number };

/** 🛂️ The stand of an actor on `perch` as it is: its feet, the way it faces and its grip (whole on a perch). */
function standOf(body: Body, perch: Perch): Stand {
  return { perch, x: body.x, y: body.y, facing: body.facing, grip: body.grip };
}

/** 🧲️ The way an actor faces while it holds on to a pitch or climbs a ladder of the wall of `side`: towards the wall (a wall's `side` is the side of its element it bounds, so the wall is on the other side of the climber). */
function wallward(leaning: { readonly side: 1 | -1 }): 1 | -1 {
  return leaning.side > 0 ? -1 : 1;
}

/** 🚸️ The footholds of a walk along the perch of `stand` to `to`, doing `activity` (walking, or carrying its ladder) at the speed of `kind`, facing the way it goes; when that is against the way it faces it first stands for `TURN_TICKS` while its drawing turns round. `null` when it cannot walk there: no speed, or not within 4096 ticks. */
function walked(stand: Stand, kind: Species, to: number, activity: Activity): Foothold[] | null {
  const steps: Foothold[] = [];
  if (to === stand.x) return steps;
  if (!(kind.locomotion.speed > 0)) return null;
  const facing = facingTo(to, stand.x);
  if (facing !== stand.facing) for (let tick = 0; tick < TURN_TICKS; tick++) steps.push({ x: stand.x, y: stand.y, footing: "perch", perch: stand.perch.surface, activity, facing, hold: -1 });
  let x = stand.x;
  for (let tick = 0; x !== to; tick++) {
    if (tick >= LONGEST) return null;
    x = strideTo(x, to, kind.locomotion.speed);
    steps.push({ x, y: stand.y, footing: "perch", perch: stand.perch.surface, activity, facing, hold: -1 });
  }
  return steps;
}

/** 🪵️ The footholds of a climb along the ladder `stand` from `travel` to `goal` (`ladderStep`, the feet at `ladderAt`), facing the wall it leans against, appended to `steps`. */
function rungs(stand: LadderStand, travel: number, goal: number, steps: Foothold[]): void {
  let along = travel;
  for (let tick = 0; along !== goal && tick < LONGEST; tick++) {
    along = ladderStep(along, goal, tick);
    const feet = ladderAt(stand, along);
    steps.push({ x: feet.x, y: feet.y, footing: "ladder", perch: null, activity: "climb", facing: wallward(stand), hold: -1 });
  }
}

/** 🧱️ An outing along the wall line `chain` whose first foothold comes at tick `from`: the footholds `lead` that bring the actor to where it takes hold of `chain[start]` (`entry`: a grab beside the wall from its feet at (`x`, `y`) — on a perch or at the exit of a ladder —; a hang over the rim from the ledge on top; or clinging there already with its feet at (`x`, `y`)), the way along the line to `goal` on `chain[end]` (`wallPath`: climbing and lunging, then mantling), facing the wall all along, and its end — over the rim onto the perch that crowns `chain[end]` (`mantle`), stepping off onto `foot` at its height and onto as much of it as the whole body needs (eased over `WALL_GRAB_TICKS`, still holding the wall), or resting on the wall. `null` when the grip it has when the way begins (`grip`) does not last for it (`wallCost`, and the step off), when the perch it ends on has no room for its body there, or when there is no way at all. */
function scaling(draft: Draft, index: number, lead: readonly Foothold[], chain: readonly Pitch[], start: number, entry: "grab" | "hang" | "cling", x: number, y: number, end: number, goal: number, mantle: boolean, foot: Perch | null, grip: number, from: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const half = kind.size.width / 2;
  const path = wallPath(chain, start, entry, x, y, end, goal, mantle, kind.size);
  const steps: Foothold[] = [...lead];
  for (const clamber of path) steps.push({ x: clamber.x, y: clamber.y, footing: "wall", perch: null, activity: clamber.work === "mantle" ? "mantle" : "climb", facing: wallward(chain[clamber.hold]!), hold: clamber.hold });
  let ending: "perch" | "wall" = "wall";
  let landing = "";
  let cost = wallCost(path);
  if (mantle) {
    const top = rimFor(chain[end]!, draft.perches, kind.size);
    const room = top === null ? null : roomOn(top, half);
    const ledge = ledgeOf(chain[end]!, kind.size);
    if (top === null || room === null || ledge < room[0] || ledge > room[1]) return null;
    ending = "perch";
    landing = top.surface;
  } else if (foot !== null) {
    const room = roomOn(foot, half);
    if (room === null) return null;
    const off = clingOf(chain[end]!, kind.size);
    const to = clamp(off, room[0], room[1]);
    if (to !== off) {
      for (let tick = 1; tick <= WALL_GRAB_TICKS; tick++) steps.push({ x: tick === WALL_GRAB_TICKS ? to : off + (to - off) * smoothstep(tick / WALL_GRAB_TICKS), y: foot.y, footing: "wall", perch: null, activity: "climb", facing: wallward(chain[end]!), hold: end });
      cost = cost + WALL_GRAB_TICKS * GRIP_CLIMB;
    }
    ending = "perch";
    landing = foot.surface;
  }
  if (steps.length === 0 || cost > grip) return null;
  return { trip: { owner: body.species, from, steps, pitches: chain, ladder: null, ending, landing, grip: grip - cost }, raise: null, stands: 0, rope: null };
}

/** 🏔️ An outing of an actor at `stand` along the wall line `chain`, its first foothold at tick `from`: it walks along its perch to where it takes hold of `chain[start]` — beside the wall as near to `hold` as its whole body stays on the perch, or to the ledge on top for a hold over the rim — and goes on as {@link scaling} says, with the grip it has there. `null` when its body has no room where it must stand. */
function fromPerch(draft: Draft, index: number, stand: Stand, chain: readonly Pitch[], start: number, hold: WallHold, end: number, goal: number, mantle: boolean, foot: Perch | null, from: Ticks): Outing | null {
  const kind = draft.kinds[index]!;
  const room = roomOn(stand.perch, kind.size.width / 2);
  if (room === null) return null;
  const at = hold.over ? ledgeOf(chain[start]!, kind.size) : clamp(hold.x, room[0], room[1]);
  if (at < room[0] || at > room[1]) return null;
  const lead = walked(stand, kind, at, "walk");
  if (lead === null) return null;
  return scaling(draft, index, lead, chain, start, hold.over ? "hang" : "grab", at, stand.y, end, goal, mantle, foot, stand.grip, from);
}

/** 🏗️ The footholds that bring an actor at `stand` onto the ladder `ladder` and up to its exit (`ladderExit`), its first foothold at tick `from`: the walk to the foot — carrying its own ladder there and raising it over `LADDER_RAISE_TICKS` when it `raise`s it —, stepping onto it over `LADDER_MOUNT_TICKS` and the climb along its rails; with the tick the ladder it raises stands up (0 when it raises none). `null` when it cannot stand at the foot. */
function ladderLead(draft: Draft, index: number, stand: Stand, ladder: LadderStand, raise: boolean, from: Ticks): { readonly steps: Foothold[]; readonly stands: Ticks } | null {
  const kind = draft.kinds[index]!;
  const room = roomOn(stand.perch, kind.size.width / 2);
  if (room === null) return null;
  const at = clamp(ladder.foot.x, room[0], room[1]);
  const lead = walked(stand, kind, at, raise ? "carry" : "walk");
  if (lead === null) return null;
  const steps: Foothold[] = [...lead];
  const facing = wallward(ladder);
  let stands = 0;
  if (raise) {
    for (let tick = 0; tick < LADDER_RAISE_TICKS; tick++) steps.push({ x: at, y: stand.y, footing: "perch", perch: stand.perch.surface, activity: "carry", facing, hold: -1 });
    stands = from + steps.length;
  }
  for (let tick = 1; tick <= LADDER_MOUNT_TICKS; tick++) steps.push({ x: tick === LADDER_MOUNT_TICKS ? ladder.foot.x : at + (ladder.foot.x - at) * smoothstep(tick / LADDER_MOUNT_TICKS), y: ladder.foot.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
  rungs(ladder, 0, ladderExit(ladder, kind.size), steps);
  return { steps, stands };
}

/** 🪤️ An outing of an actor at `stand` that raises its own ladder against `chain[start]` to take hold of the wall from it (`lean`, of `ladderTo`), its first foothold at tick `from`: up the ladder ({@link ladderLead}), then a grab from its exit onto the wall and on along the line as {@link scaling} says. The ladder is its own, raised for this trip. `null` when there is no such way. */
function leanOuting(draft: Draft, index: number, stand: Stand, chain: readonly Pitch[], start: number, lean: LadderStand, end: number, goal: number, mantle: boolean, foot: Perch | null, from: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const up = ladderLead(draft, index, stand, lean, true, from);
  if (up === null) return null;
  const exit = ladderAt(lean, ladderExit(lean, kind.size));
  const outing = scaling(draft, index, up.steps, chain, start, "grab", exit.x, exit.y, end, goal, mantle, foot, stand.grip, from);
  if (outing === null) return null;
  const trip = outing.trip;
  return { trip: { owner: trip.owner, from: trip.from, steps: trip.steps, pitches: trip.pitches, ladder: body.species, ending: trip.ending, landing: trip.landing, grip: trip.grip }, raise: lean, stands: up.stands, rope: null };
}

/** 🛤️ The way of an actor that clings to `chain[start]` with its feet at (`x`, `y`) and `grip` left down the ladder `ladder` that leans against a pitch of its line, its first foothold at tick `from`: along the line to the height of the ladder's exit, over onto the ladder there (eased over `WALL_GRAB_TICKS`), down its rails and off onto as much of the perch at its foot as the whole body needs. `null` when the ladder leans on no pitch of the line, its exit is out of the hands' reach there, its foot stands on no perch with room, or the grip does not last for the way along the line. */
function ladderDown(draft: Draft, index: number, chain: readonly Pitch[], start: number, x: number, y: number, grip: number, ladder: Ladder, from: Ticks): Outing | null {
  const kind = draft.kinds[index]!;
  const size = kind.size;
  let at = -1;
  for (let line = 0; line < chain.length && at < 0; line++) if (chain[line]!.wall === ladder.wall && chain[line]!.side === ladder.side && chain[line]!.y0 <= ladder.top.y && ladder.top.y <= chain[line]!.y1) at = line;
  const exit = ladderExit(ladder, size);
  const door = ladderAt(ladder, exit);
  if (at < 0 || !clings(chain[at]!, door.y, size)) return null;
  let ground: Perch | null = null;
  for (const perch of draft.perches) if (ground === null && perch.surface === ladder.surface && perch.y === ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1) ground = perch;
  const below = ground === null ? null : roomOn(ground, size.width / 2);
  if (ground === null || below === null) return null;
  const along = scaling(draft, index, [], chain, start, "cling", x, y, at, door.y, false, null, grip, from);
  if (along === null) return null;
  const facing = wallward(ladder);
  const steps: Foothold[] = [...along.trip.steps];
  const off = clingOf(chain[at]!, size);
  for (let tick = 1; tick <= WALL_GRAB_TICKS; tick++) steps.push({ x: tick === WALL_GRAB_TICKS ? door.x : off + (door.x - off) * smoothstep(tick / WALL_GRAB_TICKS), y: door.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
  rungs(ladder, exit, 0, steps);
  const foot = clamp(ladder.foot.x, below[0], below[1]);
  if (foot !== ladder.foot.x) for (let tick = 1; tick <= LADDER_MOUNT_TICKS; tick++) steps.push({ x: tick === LADDER_MOUNT_TICKS ? foot : ladder.foot.x + (foot - ladder.foot.x) * smoothstep(tick / LADDER_MOUNT_TICKS), y: ladder.foot.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
  return { trip: { owner: along.trip.owner, from, steps, pitches: chain, ladder: ladder.owner, ending: "perch", landing: ground.surface, grip: along.trip.grip }, raise: null, stands: 0, rope: null };
}

/** 🛗️ The ways an actor that clings to `chain[start]` with its feet at (`x`, `y`) and `grip` left gets off its wall line at tick `now`, the cheapest first (among equals: down before up, the nearer pitch first): down the line onto a perch that passes the foot of one of its pitches, up the line over the rim onto the perch that crowns one of them, or down a ladder that stands against one of them ({@link ladderDown}) — the way a pet that came up by a ladder goes back. */
function waysOff(draft: Draft, index: number, chain: readonly Pitch[], start: number, x: number, y: number, grip: number, now: Ticks): Outing[] {
  const kind = draft.kinds[index]!;
  const ways: Outing[] = [];
  for (let at = start; at < chain.length; at++) {
    for (const perch of draft.perches) {
      const hold = gripFor(perch, chain[at]!, kind.size);
      if (hold === null || hold.over) continue;
      const way = scaling(draft, index, [], chain, start, "cling", x, y, at, perch.y, false, perch, grip, now + 1);
      if (way !== null) ways.push(way);
    }
  }
  for (let at = start; at >= 0; at--) {
    if (rimFor(chain[at]!, draft.perches, kind.size) === null) continue;
    const way = scaling(draft, index, [], chain, start, "cling", x, y, at, rimOf(chain[at]!, kind.size), true, null, grip, now + 1);
    if (way !== null) ways.push(way);
  }
  for (const ladder of draft.ladders) {
    if (ladder.since > now || ladder.until <= now) continue;
    const way = ladderDown(draft, index, chain, start, x, y, grip, ladder, now + 1);
    if (way !== null) ways.push(way);
  }
  const sorted: Outing[] = [];
  for (const way of ways) {
    let at = sorted.length;
    while (at > 0 && sorted[at - 1]!.trip.grip < way.trip.grip) at--;
    sorted.splice(at, 0, way);
  }
  return sorted;
}

/** 🪫️ The grip the cheapest way off its wall line costs an actor that clings to `chain[start]` with its feet at (`x`, `y`); nothing when it has no way off — then it can only slide. */
function reserveOf(draft: Draft, index: number, chain: readonly Pitch[], start: number, x: number, y: number, now: Ticks): number {
  const ways = waysOff(draft, index, chain, start, x, y, GRIP_BUDGET, now);
  return ways.length === 0 ? 0 : GRIP_BUDGET - ways[0]!.trip.grip;
}

/** ⚖️ Whether an outing that ends resting on a wall leaves its actor the grip for a rest of `REST_LEAST` ticks there and the cheapest way off the wall line of its last pitch after it. */
function lasting(draft: Draft, index: number, outing: Outing, now: Ticks): boolean {
  const trip = outing.trip;
  const last = trip.steps[trip.steps.length - 1]!;
  const pitch = trip.pitches[last.hold]!;
  const chain = chainOf(pitch, draft.pitches, draft.kinds[index]!.size);
  return trip.grip >= reserveOf(draft, index, chain, chain.indexOf(pitch), last.x, last.y, now) + GRIP_HANG * REST_LEAST;
}

/** 🦺️ The body of the owner of `trip`, of `kind`, at one of its footholds: upright, or leaning towards the wall its hands hold there (`wallLeanOf`) — the body the stage keeps apart once it stands there. */
function bodyAt(kind: Species, trip: Trip, step: Foothold): Extent {
  return extentAt(trip.owner, kind, step, wallLeanOf(kind, step.footing, step.hold >= 0 ? trip.pitches[step.hold]! : null, step.x), null);
}

/** 🛏️ Whether an outing that ends resting on a wall leaves its actor `COMFORT_GAP` away from every other body there: walls are spaced like perches. */
function spaced(draft: Draft, index: number, outing: Outing): boolean {
  const trip = outing.trip;
  const room = grown(bodyAt(draft.kinds[index]!, trip, trip.steps[trip.steps.length - 1]!), COMFORT_GAP);
  for (const other of bodiesOf(draft.actors, draft.kinds)) if (other.owner !== trip.owner && meets(room, other.extent)) return false;
  return true;
}

/** 🏕️ The outings of an actor at `stand` to a spot on the wall line of `pitch`, their first foothold at tick `from`: for every pitch of the line it takes hold of from its perch — from below beside the wall, over the rim from the perch on top, or, when it climbs and carries a ladder, from the exit of its own ladder raised against that pitch (`ladderTo`) —, the way to the spot — the one asked for (`y` on `pitch`), or one drawn with the units `place` and `height`: a pitch of the line on its way (from the one it takes hold of to the end of the line it climbs towards) and a height on it; else nearer and nearer to where it took hold on that pitch, halving the way up to `REST_HALVINGS` times — when what is left of its grip there lasts for a rest and the cheapest way off the line ({@link lasting}) and its body keeps the spacing of a perch from everybody else ({@link spaced}). */
function restSpots(draft: Draft, index: number, stand: Stand, pitch: Pitch, y: number | null, place: number, height: number, from: Ticks, now: Ticks): Outing[] {
  const kind = draft.kinds[index]!;
  const size = kind.size;
  const chain = chainOf(pitch, draft.pitches, size);
  const leans = kind.gear.includes("ladder") && kind.gear.includes("climb");
  const outings: Outing[] = [];
  for (let start = 0; start < chain.length; start++) {
    const holds: { readonly hold: WallHold; readonly lean: LadderStand | null }[] = [];
    const hold = gripFor(stand.perch, chain[start]!, size);
    if (hold !== null) holds.push({ hold, lean: null });
    const lean = leans ? ladderTo(stand.perch, chain[start]!, draft.keepouts, size) : null;
    if (lean !== null) holds.push({ hold: { x: clingOf(chain[start]!, size), y: ladderAt(lean, ladderExit(lean, size)).y, over: false }, lean });
    for (const entered of holds) {
      const over = entered.hold.over;
      const entry = over ? rimOf(chain[start]!, size) : entered.hold.y;
      const spots: (readonly [number, number])[] = [];
      if (y !== null) spots.push([chain.indexOf(pitch), y]);
      else {
        const ahead = over ? chain.length - start : start + 1;
        const offset = Math.min(Math.floor(place * ahead), ahead - 1);
        const at = over ? start + offset : start - offset;
        const near = at === start ? entry : over ? rimOf(chain[at]!, size) : footOf(chain[at]!, size);
        const far = over ? footOf(chain[at]!, size) : rimOf(chain[at]!, size);
        spots.push([at, near + (far - near) * height]);
        const end = over ? footOf(chain[start]!, size) : rimOf(chain[start]!, size);
        let share = 1;
        for (let halving = 0; halving < REST_HALVINGS; halving++) {
          share = share / 2;
          spots.push([start, entry + (end - entry) * share]);
        }
      }
      for (const [at, spot] of spots) {
        const outing = entered.lean === null ? fromPerch(draft, index, stand, chain, start, entered.hold, at, spot, false, null, from) : leanOuting(draft, index, stand, chain, start, entered.lean, at, spot, false, null, from);
        if (outing === null || !lasting(draft, index, outing, now) || !spaced(draft, index, outing)) continue;
        outings.push(outing);
        break;
      }
    }
  }
  return outings;
}

/** 🪜️ An outing over the ladder `ladder` of `owner` from the perch of `stand` to `to`, its first foothold at tick `from`. Up: onto the ladder and up to where it steps over ({@link ladderLead}: carrying its own ladder there and raising it with `raise`) and over the rim onto the ledge (`hoistPath` to `ladderLanding` over `LADDER_DISMOUNT_TICKS`). Down: the walk to the ledge on top, the way over the rim onto the ladder (that step over backwards), the climb down to its foot and the step off onto as much of the perch as the whole body needs. It faces the wall the ladder leans against while it is on the ladder. `null` when a perch has no room for its body where the way needs it. */
function ladderOuting(draft: Draft, index: number, stand: Stand, ladder: LadderStand, owner: Slug, up: boolean, to: Perch, raise: boolean, from: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const size = kind.size;
  const start = roomOn(stand.perch, size.width / 2);
  const goal = roomOn(to, size.width / 2);
  if (start === null || goal === null) return null;
  const exit = ladderExit(ladder, size);
  const landing = ladderLanding(ladder, size);
  const top = ladderAt(ladder, exit);
  const facing = wallward(ladder);
  const steps: Foothold[] = [];
  let stands = 0;
  if (up) {
    if (landing.x < goal[0] || landing.x > goal[1]) return null;
    const lead = ladderLead(draft, index, stand, ladder, raise, from);
    if (lead === null) return null;
    steps.push(...lead.steps);
    stands = lead.stands;
    for (let tick = 1; tick <= LADDER_DISMOUNT_TICKS; tick++) {
      const feet = hoistPath(top, landing, size.height, tick / LADDER_DISMOUNT_TICKS);
      steps.push({ x: feet.x, y: feet.y, footing: "ladder", perch: null, activity: "mantle", facing, hold: -1 });
    }
  } else {
    if (landing.x < start[0] || landing.x > start[1]) return null;
    const lead = walked(stand, kind, landing.x, "walk");
    if (lead === null) return null;
    steps.push(...lead);
    for (let tick = 1; tick <= LADDER_DISMOUNT_TICKS; tick++) {
      const feet = hoistPath(top, landing, size.height, 1 - tick / LADDER_DISMOUNT_TICKS);
      steps.push({ x: feet.x, y: feet.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
    }
    rungs(ladder, exit, 0, steps);
    const off = clamp(ladder.foot.x, goal[0], goal[1]);
    if (off !== ladder.foot.x) for (let tick = 1; tick <= LADDER_MOUNT_TICKS; tick++) steps.push({ x: tick === LADDER_MOUNT_TICKS ? off : ladder.foot.x + (off - ladder.foot.x) * smoothstep(tick / LADDER_MOUNT_TICKS), y: ladder.foot.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
  }
  return { trip: { owner: body.species, from, steps, pitches: [], ladder: owner, ending: "perch", landing: to.surface, grip: stand.grip }, raise: raise ? ladder : null, stands, rope: null };
}

/** 🎋️ The ways on of an actor that holds on to the ladder `ladder` it climbs, the nearer first: up to where it steps over and over the rim onto the perch that crowns the wall it leans against, or down to its foot and onto as much of the perch it stands on as the whole body needs. */
function ladderWays(draft: Draft, index: number, ladder: Ladder, now: Ticks): Outing[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const size = kind.size;
  const half = size.width / 2;
  const travel = clamp((body.y - ladder.foot.y) / (ladder.top.y - ladder.foot.y), 0, 1) * ladderLength(ladder);
  const exit = ladderExit(ladder, size);
  const landing = ladderLanding(ladder, size);
  const ways: Outing[] = [];
  let crown: Perch | null = null;
  for (const pitch of draft.pitches) if (crown === null && pitch.wall === ladder.wall && pitch.side === ladder.side) crown = rimFor(pitch, draft.perches, size);
  const above = crown === null ? null : roomOn(crown, half);
  const facing = wallward(ladder);
  if (crown !== null && above !== null && landing.x >= above[0] && landing.x <= above[1]) {
    const steps: Foothold[] = [];
    rungs(ladder, travel, exit, steps);
    const top = ladderAt(ladder, exit);
    for (let tick = 1; tick <= LADDER_DISMOUNT_TICKS; tick++) {
      const feet = hoistPath(top, landing, size.height, tick / LADDER_DISMOUNT_TICKS);
      steps.push({ x: feet.x, y: feet.y, footing: "ladder", perch: null, activity: "mantle", facing, hold: -1 });
    }
    ways.push({ trip: { owner: body.species, from: now + 1, steps, pitches: [], ladder: ladder.owner, ending: "perch", landing: crown.surface, grip: body.grip }, raise: null, stands: 0, rope: null });
  }
  let ground: Perch | null = null;
  for (const perch of draft.perches) if (ground === null && perch.surface === ladder.surface && perch.y === ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1) ground = perch;
  const below = ground === null ? null : roomOn(ground, half);
  if (ground !== null && below !== null) {
    const steps: Foothold[] = [];
    rungs(ladder, travel, 0, steps);
    const off = clamp(ladder.foot.x, below[0], below[1]);
    if (off !== ladder.foot.x) for (let tick = 1; tick <= LADDER_MOUNT_TICKS; tick++) steps.push({ x: tick === LADDER_MOUNT_TICKS ? off : ladder.foot.x + (off - ladder.foot.x) * smoothstep(tick / LADDER_MOUNT_TICKS), y: ladder.foot.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
    if (steps.length === 0) steps.push({ x: body.x, y: body.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
    const way: Outing = { trip: { owner: body.species, from: now + 1, steps, pitches: [], ladder: ladder.owner, ending: "perch", landing: ground.surface, grip: body.grip }, raise: null, stands: 0, rope: null };
    if (ways.length > 0 && travel < exit - travel) ways.unshift(way);
    else ways.push(way);
  }
  return ways;
}

/** 🪝️ An outing up a grappling rope from the perch of `stand` to `to`, its first foothold at tick `from`: the walk to `at`, aiming over `ROPE_AIM_TICKS`, the flight of the hook (`hookTicks` at `HOOK_SPEED`), the tug over `ROPE_TUG_TICKS`, the haul up the rope (`haulStep` from `haulOf` until `REEL_LEAST` heights of rope are left) and the hoist onto the perch over `ROPE_HOIST_TICKS` (to `landingFor`, onto as much of the perch as the whole body needs), facing the way it shoots from the aim on. With `miss` the shot is aimed past the edge (`missOf`, where that line stays inside the stage and is clear but for what lies under the hook, as for a hit: `sighted`): the hook flies there and is pulled back (`HOOK_RETURN`) and the actor shrugs over `ROPE_SHRUG_TICKS` where it stands. The rope is fired when the aim is done. `null` when the perch it hoists itself onto has no room for its body. */
function ropeOuting(draft: Draft, index: number, stand: Stand, shot: Shot, to: Perch, at: number, miss: boolean, from: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const size = kind.size;
  const goal = roomOn(to, size.width / 2);
  if (goal === null) return null;
  const lead = walked(stand, kind, at, "walk");
  if (lead === null) return null;
  const perch = stand.perch.surface;
  const facing = shot.facing;
  const steps: Foothold[] = [...lead];
  for (let tick = 0; tick < ROPE_AIM_TICKS; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "aim", facing, hold: -1 });
  const fire = from + steps.length;
  const point = missOf(shot, to);
  const under = { x: shot.hook.x, y: to.y };
  const missing = miss && point.x >= 0 && point.x <= draft.width && sighted(shot.muzzle, point, draft.keepouts, ROPE_MARGIN, under, under);
  const target = missing ? point : shot.hook;
  const out = hookTicks(shot.muzzle, target, HOOK_SPEED);
  for (let tick = 0; tick < out; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "aim", facing, hold: -1 });
  const dx = target.x - shot.muzzle.x;
  const dy = target.y - shot.muzzle.y;
  const rope: Rope = { shot: missing ? { surface: shot.surface, facing: shot.facing, muzzle: shot.muzzle, hook: target, length: shot.length, reel: shot.reel } : shot, since: fire, length: Math.sqrt(dx * dx + dy * dy), hand: shot.muzzle, before: shot.muzzle, caught: !missing };
  if (missing) {
    const back = hookTicks(target, shot.muzzle, HOOK_RETURN);
    for (let tick = 0; tick < back; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "aim", facing, hold: -1 });
    for (let tick = 0; tick < ROPE_SHRUG_TICKS; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "shrug", facing, hold: -1 });
    return { trip: { owner: body.species, from, steps, pitches: [], ladder: null, ending: "perch", landing: perch, grip: stand.grip }, raise: null, stands: 0, rope };
  }
  for (let tick = 0; tick < ROPE_TUG_TICKS; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "aim", facing, hold: -1 });
  let haul = haulOf(shot, shot.length, size);
  const least = REEL_LEAST * size.height;
  for (let tick = 0; haul.rope > least && tick < LONGEST; tick++) {
    haul = haulStep(shot, haul, tick, size);
    steps.push({ x: haul.x, y: haul.y, footing: "rope", perch: null, activity: "reel", facing, hold: -1 });
  }
  const landed = landingFor(shot, to, size);
  const spot = { x: clamp(landed.x, goal[0], goal[1]), y: landed.y };
  for (let tick = 1; tick <= ROPE_HOIST_TICKS; tick++) {
    const feet = hoistPath({ x: haul.x, y: haul.y }, spot, size.height, tick / ROPE_HOIST_TICKS);
    steps.push({ x: feet.x, y: feet.y, footing: "rope", perch: null, activity: "mantle", facing, hold: -1 });
  }
  return { trip: { owner: body.species, from, steps, pitches: [], ladder: null, ending: "perch", landing: to.surface, grip: stand.grip }, raise: null, stands: 0, rope };
}

/** 🛝️ The slide of an actor down the wall it clings to, from where it is, with `grip` left: sliding (`slideStep`) down to the highest perch below its feet it reaches before its hands leave the wall at its foot (`footOf`), and stepping off onto as much of it as the whole body needs — or down to the foot of the wall, where it lets go. `null` when it is there already. */
function slideOuting(draft: Draft, index: number, grip: number, now: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const pitch = body.pitch;
  if (pitch === null) return null;
  const bottom = footOf(pitch, kind.size);
  const below = landingOf(draft.perches, body.x, body.y, bottom);
  const floor = below === null ? bottom : below.y;
  const steps: Foothold[] = [];
  let slide = { y: body.y, vy: 0 };
  const facing = wallward(pitch);
  for (let tick = 0; slide.y !== floor && tick < LONGEST; tick++) {
    slide = slideStep(slide.y, slide.vy, floor);
    steps.push({ x: body.x, y: slide.y, footing: "wall", perch: null, activity: "slide", facing, hold: 0 });
  }
  let ending: "perch" | "air" = "air";
  let landing = "";
  const room = below === null ? null : roomOn(below, kind.size.width / 2);
  if (below !== null && room !== null) {
    const to = clamp(body.x, room[0], room[1]);
    if (to !== body.x) for (let tick = 1; tick <= WALL_GRAB_TICKS; tick++) steps.push({ x: tick === WALL_GRAB_TICKS ? to : body.x + (to - body.x) * smoothstep(tick / WALL_GRAB_TICKS), y: below.y, footing: "wall", perch: null, activity: "slide", facing, hold: 0 });
    ending = "perch";
    landing = below.surface;
  }
  if (steps.length === 0) return null;
  let cost = 0;
  for (const step of steps) cost = cost + effortOf(step);
  return { trip: { owner: body.species, from: now + 1, steps, pitches: [pitch], ladder: null, ending, landing, grip: Math.max(grip - cost, 0) }, raise: null, stands: 0, rope: null };
}

/** 📶️ `outing` put among `outings` (quickest first, the fewest ticks) after every one as quick as it. */
function ranked(outings: Outing[], outing: Outing): void {
  let at = outings.length;
  while (at > 0 && outings[at - 1]!.trip.steps.length > outing.trip.steps.length) at--;
  outings.splice(at, 0, outing);
}

/** 🔗️ Two outings one after the other, the second setting out where the first ended: its footholds after those of the first (their holds counted on past the pitches of the first), the pitches of both, the ladder and the rope either uses, the end and the grip of the second. `null` when both use a ladder or both a rope: a trip uses one ladder and one rope at most. */
function joined(first: Outing, second: Outing): Outing | null {
  if ((first.trip.ladder !== null && second.trip.ladder !== null) || (first.rope !== null && second.rope !== null)) return null;
  const offset = first.trip.pitches.length;
  const steps: Foothold[] = [...first.trip.steps];
  for (const step of second.trip.steps) steps.push(step.hold < 0 ? step : { x: step.x, y: step.y, footing: step.footing, perch: step.perch, activity: step.activity, facing: step.facing, hold: step.hold + offset });
  return { trip: { owner: first.trip.owner, from: first.trip.from, steps, pitches: [...first.trip.pitches, ...second.trip.pitches], ladder: first.trip.ladder ?? second.trip.ladder, ending: second.trip.ending, landing: second.trip.landing, grip: second.trip.grip }, raise: first.raise ?? second.raise, stands: first.raise !== null ? first.stands : second.stands, rope: first.rope ?? second.rope };
}

/** 🧷️ The outings in one leg of an actor at `stand` to a venture, their first foothold at tick `from` (`now` the tick it decides at), the quickest first (the fewest ticks, in the order `routeOf` offers them among equals). To a perch: every way `routeOf` offers it with its gear (with the grip of the stand, over the ladders that stand, are free and stand on), as a trip — a ladder of its own only while fewer than two stand and none of its own; a shot from a place where its whole body stays on its perch (shot anew from there when the offered one is not), aimed past the edge with `miss`. To a spot on a wall: {@link restSpots}, with the units `place` and `height` of `units`. */
function legsTo(draft: Draft, index: number, stand: Stand, venture: Venture, units: readonly number[], miss: boolean, from: Ticks, now: Ticks): Outing[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (venture.kind === "wall") return restSpots(draft, index, stand, venture.pitch, venture.y, units[0]!, units[1]!, from, now);
  const to = venture.perch;
  const standing: Ladder[] = [];
  for (const ladder of draft.ladders) if (ladder.rider === null && ladder.since <= now && ladder.until > now) standing.push(ladder);
  const legs = routeOf(stand.x, stand.perch, to, kind.gear, kind.size, stand.grip, draft.pitches, standing, draft.keepouts) ?? [];
  const outings: Outing[] = [];
  for (const leg of legs) {
    let outing: Outing | null = null;
    if (leg.means === "ladder") {
      const owner = standing.find((ladder) => ladder === leg.ladder)?.owner;
      if (owner !== undefined) outing = ladderOuting(draft, index, stand, leg.ladder, owner, leg.up, to, false, from);
    } else if (leg.means === "wall") {
      const chain = chainOf(leg.pitch, draft.pitches, kind.size);
      outing = fromPerch(draft, index, stand, chain, chain.indexOf(leg.pitch), leg.hold, chain.indexOf(leg.exit), leg.goal, !leg.hold.over, leg.hold.over ? to : null, from);
    } else if (leg.means === "raise") {
      if (draft.ladders.length < MAX_LADDERS && !draft.ladders.some((ladder) => ladder.owner === body.species)) outing = ladderOuting(draft, index, stand, leg.ladder, body.species, true, to, true, from);
    } else {
      const room = roomOn(stand.perch, kind.size.width / 2);
      const at = room === null ? leg.at : clamp(leg.at, room[0], room[1]);
      const shot = at === leg.at ? leg.shot : shotFor({ x: at, y: stand.perch.y }, [to], draft.keepouts, kind.size);
      if (room !== null && shot !== null) outing = ropeOuting(draft, index, stand, shot, to, at, miss, from);
    }
    if (outing !== null) ranked(outings, outing);
  }
  return outings;
}

/** 🗺️ The outings of an actor that stands on a perch to a venture at tick `now`, in the order it prefers them, the quickest first: the ways in one leg ({@link legsTo}; a shot aimed past the edge when the unit `miss` is below `ROPE_MISS_CHANCE`); only when there is none, the ways in two legs — a leg to another perch (by a ladder, a wall or a rope, never a miss), then a leg from where that one ends to the venture ({@link joined}), planned whole, so a pet whose gear reaches nothing from where it stands goes there by a perch on the way. The units `[place, height, miss]` are draws of the decision on the gear stream. */
export function outingsTo(draft: Draft, index: number, venture: Venture, units: readonly number[], now: Ticks): Outing[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (!geared(kind) || body.footing !== "perch" || body.perch === null) return [];
  const from = perchAt(draft.perches, body.perch, body.x);
  if (from === null) return [];
  const stand = standOf(body, from);
  const direct = legsTo(draft, index, stand, venture, units, units[2]! < ROPE_MISS_CHANCE, now + 1, now);
  if (direct.length > 0) return direct;
  const outings: Outing[] = [];
  for (const between of draft.perches) {
    if (between === from || (venture.kind === "perch" && between === venture.perch)) continue;
    for (const first of legsTo(draft, index, stand, { kind: "perch", perch: between }, units, false, now + 1, now)) {
      const last = first.trip.steps[first.trip.steps.length - 1]!;
      const arrival: Stand = { perch: between, x: last.x, y: last.y, facing: last.facing, grip: first.trip.grip };
      for (const second of legsTo(draft, index, arrival, venture, units, false, first.trip.from + first.trip.steps.length, now)) {
        const both = joined(first, second);
        if (both !== null) ranked(outings, both);
      }
    }
  }
  return outings;
}

/** 🌉️ Whether an actor of `kind` standing at `x` on `from` can get to `to` with its gear at tick `now`, in one leg or by one perch on the way (`routeOf` over the ladders that stand free, a whole grip), as {@link outingsTo} would plan it. */
export function reaches(draft: Draft, kind: Species, x: number, from: Perch, to: Perch, now: Ticks): boolean {
  const standing: Ladder[] = [];
  for (const ladder of draft.ladders) if (ladder.rider === null && ladder.since <= now && ladder.until > now) standing.push(ladder);
  if (routeOf(x, from, to, kind.gear, kind.size, GRIP_BUDGET, draft.pitches, standing, draft.keepouts) !== null) return true;
  for (const between of draft.perches) {
    if (between === from || between === to || routeOf(x, from, between, kind.gear, kind.size, GRIP_BUDGET, draft.pitches, standing, draft.keepouts) === null) continue;
    if (routeOf((between.x0 + between.x1) / 2, between, to, kind.gear, kind.size, GRIP_BUDGET, draft.pitches, standing, draft.keepouts) !== null) return true;
  }
  return false;
}

/** 🚩️ An actor sets out at tick `now` on the first outing it can take to one of `ventures`: from the venture the unit `pick` points at, round and round, every outing in the order {@link outingsTo} prefers it (with the units `rest` = `[place, height, miss]`), the first one that is clear ({@link embark}). `false` when none is. B5 sends a pusher to its station this way (a `wall` venture with the height of the station). */
export function setOut(draft: Draft, index: number, ventures: readonly Venture[], pick: number, rest: readonly number[], now: Ticks): boolean {
  const count = ventures.length;
  if (count === 0) return false;
  const first = Math.min(Math.floor(pick * count), count - 1);
  for (let turn = 0; turn < count; turn++) {
    for (const outing of outingsTo(draft, index, ventures[(first + turn) % count]!, rest, now)) if (embark(draft, index, outing, now)) return true;
  }
  return false;
}

/** 📦️ Whether the body of an actor of `kind` with its feet at a foothold lies wholly inside the stage. */
function inside(draft: Draft, kind: Species, step: Foothold): boolean {
  const half = kind.size.width / 2;
  return step.x - half >= 0 && step.x + half <= draft.width && step.y - kind.size.height >= 0 && step.y <= draft.height;
}

/** 📜️ The claim of a trip: the body at every foothold ({@link bodyAt}), `CLAIM_SPAN` ticks to a slice, resting on the last one — nowhere when it lets go in the air there. */
function tripClaim(draft: Draft, index: number, trip: Trip): Claim {
  const kind = draft.kinds[index]!;
  const extents: Extent[] = [];
  for (const step of trip.steps) extents.push(bodyAt(kind, trip, step));
  return claimOf(trip.owner, trip.from, extents, CLAIM_SPAN, trip.ending === "air" ? null : extents[extents.length - 1]!);
}

/** 👟️ The goal of a walk of a trip that begins with the foothold `at`: where the run of walking or carrying footholds on a perch that it begins ends. */
function walkEnd(trip: Trip, at: number): number {
  let goal = trip.steps[at]!.x;
  for (let next = at; next < trip.steps.length; next++) {
    const step = trip.steps[next]!;
    if (step.footing !== "perch" || (step.activity !== "walk" && step.activity !== "carry")) break;
    goal = step.x;
  }
  return goal;
}

/** ⛵️ An actor takes an outing at tick `now` — only when its body stays inside the stage at every foothold, the ladder it raises is the stage's second at most and its own first, the ladder it climbs stands, is free (or its own already) and stands `LADDER_IDLE` beyond the trip, and the claim of the trip is clear (`claimClear`). It lets go of its partner; on a wall it sets out with what its rest left of its grip; the claim and the trip are on the table; the ladder it raises stands up when the raise is done and is taken away `LADDER_IDLE` after the trip, `LADDER_LIFE` after it stood at the latest; the ladder it climbs is its own until it gets off and stands `LADDER_IDLE` longer (`LADDER_LIFE` at most); its rope is out (fired when the aim is done); it faces the way of its first foothold and walks for the end of the walk it begins with ({@link walkEnd}); its time is up when the trip ends. `false` when it does not take it. */
export function embark(draft: Draft, index: number, outing: Outing, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const trip = outing.trip;
  const owner = body.species;
  for (const step of trip.steps) if (!inside(draft, kind, step)) return false;
  const end = trip.from + trip.steps.length - 1;
  if (outing.raise !== null) {
    if (draft.ladders.length >= MAX_LADDERS || draft.ladders.some((ladder) => ladder.owner === owner)) return false;
  } else if (trip.ladder !== null) {
    const ladder = draft.ladders.find((entry) => entry.owner === trip.ladder);
    if (ladder === undefined || (ladder.rider !== null && ladder.rider !== owner) || ladder.since > now || ladder.since + LADDER_LIFE < end + LADDER_IDLE) return false;
  }
  const claim = tripClaim(draft, index, trip);
  if (!claimClear(claim, bodiesOf(draft.actors, draft.kinds), draft.claims)) return false;
  release(draft, index, now);
  if (body.footing === "wall") body.grip = gripNow(body, now);
  const raise = outing.raise;
  if (raise !== null) draft.ladders = [...draft.ladders, { owner, wall: raise.wall, surface: raise.surface, side: raise.side, foot: raise.foot, top: raise.top, since: outing.stands, until: Math.min(outing.stands + LADDER_LIFE, end + LADDER_IDLE), rider: owner }];
  else if (trip.ladder !== null) draft.ladders = draft.ladders.map((ladder) => (ladder.owner === trip.ladder ? { owner: ladder.owner, wall: ladder.wall, surface: ladder.surface, side: ladder.side, foot: ladder.foot, top: ladder.top, since: ladder.since, until: Math.min(ladder.since + LADDER_LIFE, Math.max(ladder.until, end + LADDER_IDLE)), rider: owner } : ladder));
  draft.claims = [...released(draft.claims, owner), claim];
  draft.courses = dropped(draft.courses, owner);
  draft.trips = [...unpacked(draft.trips, owner), trip];
  body.rope = outing.rope;
  body.until = end;
  const first = trip.steps[0]!;
  body.goal = first.footing === "perch" && (first.activity === "walk" || first.activity === "carry") ? walkEnd(trip, 0) : body.x;
  turn(body, first.facing, now);
  return true;
}

/** 🚂️ One tick of an actor on a trip at tick `now`: it takes its foothold of the tick — where its feet are and how fast they moved, what carries it (its perch, which may be a perch on the way between two legs, a wall, a ladder or a rope), the pitch its hands hold, the grip it has left (every foothold on a wall costs its effort), the way it faces (it turns round where a foothold faces the other way), what it does (a walk goes for where its run of walking footholds ends; it feels the miss of its hook when it shrugs, `missed`) —; on the haul its rope runs from the hook to its hands, a missed rope is in again once the hook is back and a rope it climbed once it stands on a perch again; at its last foothold it arrives ({@link alight}). An actor whose activity something else changed since the last tick (a click, a trick, a mode that stops walkers) gives its trip up there ({@link halt}). `false` when the actor is gone. */
export function travel(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const trip = tripOf(draft.trips, body.species);
  if (trip === null) return true;
  const at = now - trip.from;
  if (at < 0) return true;
  const last = trip.steps.length - 1;
  const was = at > 0 ? trip.steps[(at > last ? last : at) - 1]! : null;
  if (was !== null && body.activity !== was.activity) {
    halt(draft, index, now);
    return true;
  }
  const step = trip.steps[at > last ? last : at]!;
  body.vx = (step.x - body.x) * TICKS_PER_SECOND;
  body.vy = (step.y - body.y) * TICKS_PER_SECOND;
  body.x = step.x;
  body.y = step.y;
  body.footing = step.footing;
  body.perch = step.perch;
  body.pitch = step.hold >= 0 ? trip.pitches[step.hold]! : null;
  body.grip = Math.max(body.grip - effortOf(step), 0);
  if (step.activity !== body.activity) {
    shift(draft, index, step.activity, now);
    body.clip = clipAt(kind, step.activity, 0);
    if (step.activity === "walk" || step.activity === "carry") body.goal = walkEnd(trip, at > last ? last : at);
    if (step.activity === "shrug") feel(draft, index, "missed", now);
  }
  if (step.facing !== body.facing) turn(body, step.facing, now);
  const rope = body.rope;
  if (rope !== null && !rope.caught && now >= rope.since + hookTicks(rope.shot.muzzle, rope.shot.hook, HOOK_SPEED) + hookTicks(rope.shot.hook, rope.shot.muzzle, HOOK_RETURN)) body.rope = null;
  else if (rope !== null && rope.caught && now > rope.since && step.footing === "perch" && step.activity !== "aim") body.rope = null;
  else if (rope !== null && step.footing === "rope") {
    const hand = { x: step.x + rope.shot.facing * MUZZLE_FORWARD * kind.size.width, y: step.y - MUZZLE_HEIGHT * kind.size.height };
    const dx = rope.shot.hook.x - hand.x;
    const dy = rope.shot.hook.y - hand.y;
    body.rope = { shot: rope.shot, since: rope.since, length: Math.sqrt(dx * dx + dy * dy), hand, before: rope.hand, caught: rope.caught };
  }
  if (at < last) return true;
  return alight(draft, index, trip, now);
}

/** 🏁️ An actor comes to the end of its trip at tick `now`: the claim and the trip are done, the ladder it climbed is free and its rope is in; letting go in the air it falls ({@link plunge}); on a wall it rests there with the grip it has left ({@link rest}, for a dwell of `idle`); on a perch it stands — whole of grip — and comes to rest: after a miss for `ROPE_REST` at least, after any other way with its gear proud of it (`climbed`). `false` when the actor is gone. */
function alight(draft: Draft, index: number, trip: Trip, now: Ticks): boolean {
  const body = draft.actors[index]!;
  draft.claims = released(draft.claims, body.species);
  draft.trips = unpacked(draft.trips, body.species);
  freed(draft, body.species, now);
  body.rope = null;
  if (trip.ending === "air") {
    unfoot(draft, index, body.vx, body.vy, "fall", now);
    return plunge(draft, index, now);
  }
  if (trip.ending === "wall") {
    body.footing = "wall";
    body.perch = null;
    body.grip = trip.grip;
    const words = randomWords(actorKey(draft, index), 1);
    rest(draft, index, dwellOf("idle", draft.mode, unitOf(words[0]!)), now);
    return true;
  }
  body.footing = "perch";
  body.perch = trip.landing;
  body.pitch = null;
  body.grip = GRIP_BUDGET;
  settle(draft, index, now);
  if (trip.steps[trip.steps.length - 1]!.activity === "shrug") body.until = Math.max(body.until, now + ROPE_REST);
  else if (trip.steps.some((step) => step.footing !== "perch")) feel(draft, index, "climbed", now);
  return true;
}

/** 🦥️ An actor holds on to its wall or its ladder at tick `now` and rests there for `span` ticks — on a wall no longer than what is left of its grip lasts beside the reserve for the cheapest way off its line (`GRIP_HANG` per tick) —, in the pose of its climb (`climb`, paused: the clip turns with the distance climbed), with the grip it has now; at least a tick. Once its time is up it moves on by itself ({@link cling}). B5 keeps a pusher at its station on a wall this way and then sets it to `push` for the same time. */
export function rest(draft: Draft, index: number, span: Ticks, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  shift(draft, index, "climb", now);
  body.clip = clipAt(kind, "climb", 0);
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
  let until = now + span;
  const pitch = body.pitch;
  if (body.footing === "wall" && pitch !== null) {
    const chain = chainOf(pitch, draft.pitches, kind.size);
    const spare = body.grip - reserveOf(draft, index, chain, chain.indexOf(pitch), body.x, body.y, now);
    until = Math.min(until, now + Math.floor(spare / GRIP_HANG));
  }
  body.until = Math.max(until, now + 1);
}

/** 🏞️ A spot on its wall line an actor that clings to `chain[start]` with `grip` left climbs on to (the units `place` and `height` draw the pitch of the line and the height on it), when what is left of its grip there lasts for a rest and the cheapest way off ({@link lasting}) and it keeps its spacing there ({@link spaced}); `null` otherwise. */
function explored(draft: Draft, index: number, chain: readonly Pitch[], start: number, grip: number, place: number, height: number, now: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const size = draft.kinds[index]!.size;
  const at = Math.min(Math.floor(place * chain.length), chain.length - 1);
  const y = rimOf(chain[at]!, size) + (footOf(chain[at]!, size) - rimOf(chain[at]!, size)) * height;
  const outing = scaling(draft, index, [], chain, start, "cling", body.x, body.y, at, y, false, null, grip, now);
  return outing !== null && lasting(draft, index, outing, now) && spaced(draft, index, outing) ? outing : null;
}

/** ➰️ An actor whose rest on a wall or a ladder is over moves on at tick `now`. On a ladder: up or down, the nearer first ({@link ladderWays}); while neither is clear it holds on for `REST_RETRY` more ticks and the ladder stands `LADDER_IDLE` beyond that — but no longer than `LADDER_LIFE`: then it lets go with a fright. On a wall, with what is left of its grip: in a lively stage outside a time of concentration, one time in two (`EXPLORE_SHARE`, an actor draw), to another spot of its line ({@link explored}); else, or when that is not clear, the cheapest way off its line that is clear ({@link waysOff}); while none is clear it holds on for `REST_RETRY` more ticks as long as its grip lasts for that and the cheapest way off; then it slides down ({@link slideOuting}); when even that is not clear it holds on while it still can and at last lets go with a fright (`startled`). `false` when the actor is gone. */
function onward(draft: Draft, index: number, ladder: Ladder | null, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (ladder !== null) {
    for (const way of ladderWays(draft, index, ladder, now)) if (embark(draft, index, way, now)) return true;
    if (ladder.since + LADDER_LIFE <= now + REST_RETRY) {
      feel(draft, index, "startled", now);
      unfoot(draft, index, 0, 0, "fall", now);
      return plunge(draft, index, now);
    }
    draft.ladders = draft.ladders.map((entry) => (entry.owner === ladder.owner ? { owner: entry.owner, wall: entry.wall, surface: entry.surface, side: entry.side, foot: entry.foot, top: entry.top, since: entry.since, until: Math.min(entry.since + LADDER_LIFE, Math.max(entry.until, now + REST_RETRY + LADDER_IDLE)), rider: entry.rider } : entry));
    body.until = now + REST_RETRY;
    return true;
  }
  const pitch = body.pitch!;
  const grip = gripNow(body, now);
  const chain = chainOf(pitch, draft.pitches, kind.size);
  const start = chain.indexOf(pitch);
  const ways = waysOff(draft, index, chain, start, body.x, body.y, grip, now);
  const tries: Outing[] = [];
  if (draft.mode === "lively" && !draft.quiet) {
    const units = randomWords(actorKey(draft, index), 3).map(unitOf);
    if (units[0]! < EXPLORE_SHARE) {
      const spot = explored(draft, index, chain, start, grip, units[1]!, units[2]!, now);
      if (spot !== null) tries.push(spot);
    }
  }
  for (const way of ways) tries.push(way);
  for (const outing of tries) if (embark(draft, index, outing, now)) return true;
  const reserve = ways.length === 0 ? 0 : grip - ways[0]!.trip.grip;
  if (ways.length > 0 && grip - GRIP_HANG * REST_RETRY >= reserve) {
    body.until = now + REST_RETRY;
    return true;
  }
  const slide = slideOuting(draft, index, grip, now);
  if (slide !== null && embark(draft, index, slide, now)) return true;
  if (grip - GRIP_HANG * REST_RETRY > 0) {
    body.until = now + REST_RETRY;
    return true;
  }
  feel(draft, index, "startled", now);
  unfoot(draft, index, 0, 0, "fall", now);
  return plunge(draft, index, now);
}

/** 🐨️ One tick of an actor that holds on to a wall or a ladder without a trip: once its rest is over it moves on ({@link onward}); one that no longer has the wall (`pitch` lost) or the ladder it held lets go and falls. `false` when the actor is gone. */
export function cling(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const ladder = body.footing === "ladder" ? (draft.ladders.find((entry) => entry.rider === body.species) ?? null) : null;
  if ((body.footing === "wall" && body.pitch === null) || (body.footing === "ladder" && ladder === null)) {
    unfoot(draft, index, 0, 0, "fall", now);
    return plunge(draft, index, now);
  }
  if (now < body.until) return true;
  return onward(draft, index, ladder, now);
}

/** 🛑️ An actor gives its trip up at tick `now` — the stage moved under it, the learner's hand or another actor's plan took its place, or something set it off on another activity: its claim is released. On its rope it lets go and falls; on a wall or a ladder it holds on where it is, with the grip its trip has left it, and looks for a way on at the next tick ({@link rest}) — a ladder it left for the wall is free again —; on its perch it stays as it is, its rope in and the ladder it was going to climb or raise free. */
export function halt(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (tripOf(draft.trips, body.species) === null) return;
  draft.claims = released(draft.claims, body.species);
  draft.trips = unpacked(draft.trips, body.species);
  if (body.footing === "rope") {
    unfoot(draft, index, body.vx, body.vy, "fall", now);
    return;
  }
  body.rope = null;
  if (body.footing === "ladder") {
    rest(draft, index, 1, now);
    return;
  }
  freed(draft, body.species, now);
  if (body.footing === "wall") rest(draft, index, 1, now);
}

/** 🔓️ The ladders `owner` climbs are free again, and the one it raises and that does not stand yet is taken away. */
function freed(draft: Draft, owner: Slug, now: Ticks): void {
  if (!draft.ladders.some((ladder) => ladder.rider === owner || (ladder.owner === owner && ladder.since > now))) return;
  const kept: Ladder[] = [];
  for (const ladder of draft.ladders) {
    if (ladder.owner === owner && ladder.since > now) continue;
    kept.push(ladder.rider === owner ? { owner: ladder.owner, wall: ladder.wall, surface: ladder.surface, side: ladder.side, foot: ladder.foot, top: ladder.top, since: ladder.since, until: ladder.until, rider: null } : ladder);
  }
  draft.ladders = kept;
}

/** 🧺️ An actor lets go of everything its gear holds at tick `now`: its trip, its rope and the wall it clings to — off the wall its grip is whole again — and the ladders it climbs or raises ({@link freed}). */
function ungear(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (draft.trips.some((trip) => trip.owner === body.species)) draft.trips = unpacked(draft.trips, body.species);
  body.rope = null;
  body.pitch = null;
  body.grip = GRIP_BUDGET;
  freed(draft, body.species, now);
}
//#endregion 🔖️Gear

//#region 🔖️Mischief
/** 🧘️ Whether an outing that ends holding on to a wall leaves its actor the grip to hang there still for `span` ticks (`GRIP_HANG` per tick). What is left afterwards may not last for the way back up: a pet that played spent its grip, and slides down its wall when it is over. */
function holding(outing: Outing, span: Ticks): boolean {
  return outing.trip.grip >= GRIP_HANG * span;
}

/** 📬️ An actor that stands on a perch sets out at tick `now` for its post, to work there for `span` ticks after it arrives: to a post on a wall on the first clear outing {@link outingsTo} offers (over the rim from the perch on top, up from a perch beside the wall or from the exit of its own ladder raised against it, by a perch on the way when nothing reaches it from where it stands; lunges included) that leaves it the grip for that ({@link holding}); to a post on its perch on a walk along it — a trip, claimed like every other, that ends standing there; where that walk is not clear it does not go, since it never passes another on its perch. With `slip`, where no way to a post elsewhere is clear — the post lies out of reach of its gear — it slips over in a puff ({@link popTo}), the last resort the stage tries only when no pet that could play can get to its post with its gear. The tick it stands at its post — the end of its trip, or `now` when it stands there already or slipped over —, or `null` when it cannot get there at all (then nothing changes). */
export function postTo(draft: Draft, index: number, post: Post, side: 1 | -1, span: Ticks, slip: boolean, now: Ticks): Ticks | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (post.kind === "wall") {
    for (const outing of outingsTo(draft, index, { kind: "wall", pitch: post.pitch, y: post.y }, POST_UNITS, now)) if (holding(outing, span) && embark(draft, index, outing, now)) return outing.trip.from + outing.trip.steps.length - 1;
  } else if (body.footing === "perch" && body.perch === post.perch.surface) {
    const steps = walked(standOf(body, post.perch), kind, post.x, "walk");
    if (steps !== null && steps.length === 0) return now;
    const trip: Trip | null = steps === null ? null : { owner: body.species, from: now + 1, steps, pitches: [], ladder: null, ending: "perch", landing: post.perch.surface, grip: body.grip };
    return trip !== null && embark(draft, index, { trip, raise: null, stands: 0, rope: null }, now) ? trip.from + trip.steps.length - 1 : null;
  }
  return slip && popTo(draft, index, post, side, span, now) ? now : null;
}

/** 💫️ An actor slips over to its post at tick `now` when its gear has no way there: it is gone from where it stood in a puff of dust ({@link dusted}: not a last resort, but a poof the stage counts) and stands at its post at once, see-through — it fades in over the next ticks like an arrival —, facing `side`; on a wall it holds on whole of grip, resting in the pose of its climb for `span` ticks, on a perch it comes to rest. Only when its body there lies inside the stage, a comfortable gap from every other body and off every corridor and landing spot somebody claimed. `false` when it cannot (then nothing changes). */
function popTo(draft: Draft, index: number, post: Post, side: 1 | -1, span: Ticks, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const feet: Foothold = post.kind === "wall" ? { x: clingOf(post.pitch, kind.size), y: post.y, footing: "wall", perch: null, activity: "climb", facing: side, hold: 0 } : { x: post.x, y: post.perch.y - hoverOf(kind), footing: "perch", perch: post.perch.surface, activity: "idle", facing: side, hold: -1 };
  const extent = extentAt(body.species, kind, feet, wallLeanOf(kind, feet.footing, post.kind === "wall" ? post.pitch : null, feet.x), null);
  const room = grown(extent, COMFORT_GAP);
  if (!inside(draft, kind, feet) || !freeAmong(extent, obstaclesFor(draft, index, now)) || bodiesOf(draft.actors, draft.kinds).some((other) => other.owner !== body.species && meets(room, other.extent))) return false;
  dusted(draft, index, now);
  release(draft, index, now);
  ungear(draft, index, now);
  body.x = feet.x;
  body.y = feet.y;
  body.goal = feet.x;
  body.vx = 0;
  body.vy = 0;
  body.tilt = 0;
  body.opacity = 0;
  body.facing = side;
  body.faced = now;
  body.host = null;
  if (post.kind === "wall") {
    body.footing = "wall";
    body.perch = null;
    body.pitch = post.pitch;
    rest(draft, index, span, now);
    body.until = now + span;
    return true;
  }
  body.footing = "perch";
  body.perch = post.perch.surface;
  settle(draft, index, now);
  return true;
}

/** 🤜️ An actor at its post begins to push at tick `now`, until `until`, shoving the way `side` says — when it is ready: idle on its perch, or holding on to its wall at rest in the pose of its climb with the grip to hang there until then; not on a trip, not leaving, without a partner. It faces the way it shoves, stands still and plays its push clip. `false` when it is not ready (then nothing changes). */
export function pushAt(draft: Draft, index: number, side: 1 | -1, until: Ticks, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (body.leaving || body.partner !== null || tripOf(draft.trips, body.species) !== null) return false;
  const clinging = body.footing === "wall" && body.activity === "climb" && body.pitch !== null;
  if (!clinging && !(body.footing === "perch" && body.activity === "idle")) return false;
  if (clinging) {
    const grip = gripNow(body, now);
    if (grip < GRIP_HANG * (until - now)) return false;
    body.grip = grip;
  }
  shift(draft, index, "push", now);
  body.clip = clipAt(kind, "push", 0);
  body.until = until;
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
  turn(body, side, now);
  return true;
}

/** 🫷️ An actor stops pushing at tick `now` without being thrown off: on its wall it holds on in the pose of its climb and moves on from the next tick ({@link rest}), on its perch it comes to rest. An actor that does not push stays as it is. */
export function unpush(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (body.activity !== "push") return;
  if (body.footing === "wall" && body.pitch !== null) rest(draft, index, 1, now);
  else settle(draft, index, now);
}

/** 🧯️ The prank of the stage ends quietly at tick `now` — its element went away or moved, the learner no longer permits mischief, the stage turns still —: a pet on its way to its post goes on to where it was going, a pusher stops ({@link unpush}), the copy is gone and the mode's cooldown counts from now (`rested`). A pusher the learner threw off keeps its prank until it is down again. */
export function unlifted(draft: Draft, now: Ticks): void {
  const lift = draft.lift;
  if (lift === null || now - lift.since > LIFT_TICKS) return;
  draft.lift = null;
  draft.rested = now;
  const index = indexOf(draft.actors, lift.pusher);
  if (index >= 0) unpush(draft, index, now);
}
//#endregion 🔖️Mischief

//#region 🔖️Leaving
/** 👋️ An actor starts to leave: it lets go of its partner and walks to the nearer end of its perch when that is within three of its widths and nobody stands in the way, then fades; farther away, hemmed in or off its perch, it fades where it is — on a trip with its gear it goes on with it while it fades, unless it is still on its perch: then it gives the trip up ({@link halt}). */
export function leave(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  release(draft, index, now);
  body.leaving = true;
  if (body.footing !== "perch" || body.perch === null) return;
  halt(draft, index, now);
  const perch = perchAt(draft.perches, body.perch, body.x);
  const half = kind.size.width / 2;
  const low = perch === null ? body.x : perch.x0 + half;
  const high = perch === null ? body.x : perch.x1 - half;
  const end = body.x - low <= high - body.x ? low : high;
  const [clearLow, clearHigh] = clearway(draft, index, COMFORT_GAP, -1);
  const clip = clipAt(kind, "walk", 0);
  const goal = paced(kind, clip, body.x, end);
  if (goal !== body.x && end >= clearLow && end <= clearHigh && Math.abs(end - body.x) <= LEAVE_REACH * kind.size.width) {
    stroll(draft, index, goal, clip, now);
    return;
  }
  shift(draft, index, "idle", now);
  body.clip = clipAt(kind, "idle", 0);
  body.goal = body.x;
}
//#endregion 🔖️Leaving
