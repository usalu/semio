/** ⚓️ Locomotion, footing: what carries a pet and how it loses its ground — the plan of a course, the puff in which a pet vanishes and arrives anew, dropping a pet whose footing went away and installing a planned course. The module as a whole (this folder; the children sit in the sibling folders `📐️constants`, `🛬️courses`, `🛫️departure`, `🏃️motion`, `✋️hand`, `🪜️outings`, `🧳️trips`, `🪄️mischief` and `🚪️leaving`):
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
 * @see ../../🚧️clearance/🟦️.ts — claims, guarded strides, heads, the last resort
 * @see ../../🪢️swing/🟦️.ts — the hand and the parachute
 * @see ../../🏞️terrain/🟦️.ts — falls, hops, landings
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ../🦀️.rs — the Rust twin
 */
import { type Activity, type Claim, type Course, type Ladder, type Slug, type Ticks, type Trip, type Waypoint } from "../../../🧬️schema/🟦️.ts";
import { clipTicks } from "../../🎞️animation/🟦️.ts";
import { DELAY, SLIDE_OFF_SPEED, released } from "../../🚧️clearance/🟦️.ts";
import { clipAt, clipOf, feel, indexOf, release, remove, settle, shift, type Draft } from "../../📝️draft/🟦️.ts";
import { dwellOf } from "../../🧠️behavior/🟦️.ts";
import { GRIP_BUDGET } from "../../🧗️climbing/🟦️.ts";
import { HARD_LANDING } from "../../🪢️swing/🟦️.ts";
import { DIZZY_SPEED } from "../📐️constants/🟦️.ts";

//#region 🔖️Gear
//#region 🔖️Footing
/** 🎟️ A course and the claim of its corridor. */
export type Plan = { readonly course: Course; readonly claim: Claim };

/** 🧾️ The course of an owner, or `null`. */
export function courseOf(courses: readonly Course[], owner: Slug): Course | null {
  for (const course of courses) if (course.owner === owner) return course;
  return null;
}

/** 🧹️ The courses without the one of `owner`. */
export function dropped(courses: readonly Course[], owner: Slug): Course[] {
  const kept: Course[] = [];
  for (const course of courses) if (course.owner !== owner) kept.push(course);
  return kept;
}

/** 🏷️ Whether an owner holds a claim. */
export function claimed(claims: readonly Claim[], owner: Slug): boolean {
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
export function dusted(draft: Draft, index: number, now: Ticks): void {
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
export function arrive(draft: Draft, index: number, course: Course, now: Ticks): boolean {
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

/** 🤕️ Whether a waypoint of free flight (no canopy) reached from the speed `vx`, `vy` bounces off an edge of the stage: it stands at a side edge with its sideways speed turned round, or with its body against the top edge and its rise turned into a fall. Free flight changes its sideways speed nowhere else. */
export function bounced(draft: Draft, index: number, step: Waypoint, vx: number, vy: number): boolean {
  const kind = draft.kinds[index]!;
  const half = kind.size.width / 2;
  if (step.canopy !== null) return false;
  return ((step.x === half || step.x === draft.width - half) && vx * step.vx < 0) || (step.y === kind.size.height && vy < 0 && step.vy >= 0);
}

/** 🧳️ The trips without the one of `owner`. */
export function unpacked(trips: readonly Trip[], owner: Slug): Trip[] {
  const kept: Trip[] = [];
  for (const trip of trips) if (trip.owner !== owner) kept.push(trip);
  return kept;
}

/** 🔓️ The ladders `owner` climbs are free again, and the one it raises and that does not stand yet is taken away. */
export function freed(draft: Draft, owner: Slug, now: Ticks): void {
  if (!draft.ladders.some((ladder) => ladder.rider === owner || (ladder.owner === owner && ladder.since > now))) return;
  const kept: Ladder[] = [];
  for (const ladder of draft.ladders) {
    if (ladder.owner === owner && ladder.since > now) continue;
    kept.push(ladder.rider === owner ? { owner: ladder.owner, wall: ladder.wall, surface: ladder.surface, side: ladder.side, foot: ladder.foot, top: ladder.top, since: ladder.since, until: ladder.until, rider: null } : ladder);
  }
  draft.ladders = kept;
}

/** 🧺️ An actor lets go of everything its gear holds at tick `now`: its trip, its rope and the wall it clings to — off the wall its grip is whole again — and the ladders it climbs or raises ({@link freed}). */
export function ungear(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (draft.trips.some((trip) => trip.owner === body.species)) draft.trips = unpacked(draft.trips, body.species);
  body.rope = null;
  body.pitch = null;
  body.grip = GRIP_BUDGET;
  freed(draft, body.species, now);
}
//#endregion 🔖️Gear
