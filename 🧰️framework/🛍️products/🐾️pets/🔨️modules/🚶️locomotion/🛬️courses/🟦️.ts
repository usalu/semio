/** 🛬️ Locomotion, courses: how a pet whose ground went away plans its way down — tick by tick, landing included, with the air control of the steerings, the parachute and the one-way platform of another pet's head.
 *
 * @see ../⚓️footing/🟦️.ts — the plan and the installed course
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */
import { TICKS_PER_SECOND, type Canopy, type Course, type Extent, type Perch, type Species, type Ticks, type Turns, type Waypoint } from "../../../🧬️schema/🟦️.ts";
import { MARGIN, STEERINGS, claimClear, claimOf, columnOver, headUnder, liftOnto, obstaclesOf, type Body as Solid } from "../../🚧️clearance/🟦️.ts";
import { fallStep, landingOf, perchAt } from "../../🏞️terrain/🟦️.ts";
import { bodiesOf, extentAt } from "../../📏️spacing/🟦️.ts";
import { clamp, smoothstep } from "../../📐️trigonometry/🟦️.ts";
import { hoverOf, type Draft } from "../../📝️draft/🟦️.ts";
import { CHUTE_HEADROOM, CHUTE_OPENING, CHUTE_REFLEX, HARD_LANDING, canopyOf, chuteOf, chuteStep, chuteWind } from "../../🪢️swing/🟦️.ts";
import { COURSE_TICKS, CLAIM_SPAN, TUMBLE_TICKS, WIND_PHASE, STEER_MOST, BRACES, BOUNCE } from "../📐️constants/🟦️.ts";
import { type Plan, claimed } from "../⚓️footing/🟦️.ts";

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
