/** 🏃️ Locomotion, motion: the guarded stride of a walker, the scoot of a scooter, the slip of a rider on a head and the flight and plunge of a pet in the air.
 *
 * @see ../🛬️courses/🟦️.ts — the ways down
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */
import { TICKS_PER_SECOND, type Ticks } from "../../../🧬️schema/🟦️.ts";
import { springStep } from "../../🎞️animation/🟦️.ts";
import { SCOOT_HASTE, SLIDE_OFF_SPEED, freeAmong, guardedStride, mustPoof, obstaclesOf, restsOn, scootFraction, scooted, shifted, slideOf, slideStride, type Body as Solid } from "../../🚧️clearance/🟦️.ts";
import { perchAt, strideTo } from "../../🏞️terrain/🟦️.ts";
import { bodiesOf, extentOf, obstaclesFor } from "../../📏️spacing/🟦️.ts";
import { clamp } from "../../📐️trigonometry/🟦️.ts";
import { beatOf, clipAt, clipOf, feel, indexOf, settle, shift, type Draft } from "../../📝️draft/🟦️.ts";
import { OPEN_STIFFNESS, OPEN_DAMPING } from "../📐️constants/🟦️.ts";
import { courseOf, poof, unfoot, drop, install, arrive, bounced } from "../⚓️footing/🟦️.ts";
import { waysDown } from "../🛬️courses/🟦️.ts";
import { attend } from "../🛫️departure/🟦️.ts";

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
