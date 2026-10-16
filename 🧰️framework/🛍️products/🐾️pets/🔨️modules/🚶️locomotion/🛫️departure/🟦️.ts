/** 🛫️ Locomotion, departure: setting out for a goal on the pet's perch, strolling, attending and the hop or glide that launches a pet from one perch to the next.
 *
 * @see ../🛬️courses/🟦️.ts — the plan of a launch
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */
import { TICKS_PER_SECOND, type Course, type Slug, type Species, type Ticks, type Waypoint } from "../../../🧬️schema/🟦️.ts";
import { MARGIN, claimClear, laneLift, spotOn } from "../../🚧️clearance/🟦️.ts";
import { HOP_DISTANCE, HOP_HEIGHT, hopLanding, hopOf, hopStep } from "../../🏞️terrain/🟦️.ts";
import { bodiesOf, extentAt, obstaclesFor, soars } from "../../📏️spacing/🟦️.ts";
import { clamp } from "../../📐️trigonometry/🟦️.ts";
import { beatOf, clipAt, clipOf, hoverOf, shift, type Draft, type Launch } from "../../📝️draft/🟦️.ts";
import { dwellOf } from "../../🧠️behavior/🟦️.ts";
import { WAITING, GLIDE_TICKS, LANE_RAMP } from "../📐️constants/🟦️.ts";
import { install } from "../⚓️footing/🟦️.ts";
import { planOf } from "../🛬️courses/🟦️.ts";

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
