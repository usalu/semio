/** 📏️ Bodies and the room between them: the solid box of every pet as it stands, hangs, tumbles or sinks under its canopy (`extentAt`, `extentOf`, `bodiesOf`), what an actor has to keep clear of (`obstaclesFor`), where a newcomer can arrive and where a walker may set out to without coming near anybody (`roomsFor`, `quarters`, `clearway`), and whether the arc of a hop keeps clear of what must stay free (`soars`) — with the gap the stage keeps between bodies, `COMFORT_GAP`.
 *
 * A body is the box of `🚧️clearance`: the size box of the species above its feet and the hover beneath them, turned
 * with the drawing about the scruff while it hangs, tumbles or leans towards the wall it clings to ({@link wallLeanOf}),
 * united with its canopy while a parachute is out, and `MARGIN` on every side, so two bodies that touch keep the
 * comfortable gap between their drawings — the body holds the drawing whatever its posture. Every actor on stage is a
 * body — a newcomer that fades in and a leaver that fades out too; a pet that vanished is none. The invariant of the
 * stage: at the end of every tick no two bodies overlap.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🚧️clearance/🟦️.ts — bodies, claims, free places and the invariant
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ./🦀️.rs — the Rust twin
 */

import { type Actor, type Extent, type Footing, type Pitch, type Point, type Species, type Ticks, type Turns } from "../../🧬️schema/🟦️.ts";
import { MARGIN, SEAM, UPRIGHT, bodyOf, obstaclesOf, united, type Body as Solid } from "../🚧️clearance/🟦️.ts";
import { hopStep, perchAt, type Flight, type Hop } from "../🏞️terrain/🟦️.ts";
import { clamp, cosTurns, sinTurns } from "../📐️trigonometry/🟦️.ts";
import { carve, crowdOn, hoverOf, shoulders, type Draft, type Room } from "../📝️draft/🟦️.ts";
import { CHUTE_ROD } from "../🪢️swing/🟦️.ts";
import { clingOf } from "../🧗️climbing/🟦️.ts";

//#region 🔖️Constants
/** 🛋️ The gap between the drawings of two bodies that touch, in pixels: twice the margin of a body. */
export const COMFORT_GAP = 8;

/** 🧗️ How far an actor that clings to a wall leans towards it, in turns: its drawing and its body. */
export const WALL_LEAN = 0.04;

/** 🌂️ How far the canopy of a parachute reaches out to either side of the scruff, in widths of the species (the plain canopy of the depiction). */
const CANOPY_SPAN = 0.75;

/** 🎈️ How far above the scruff the rim of an open canopy floats, in heights of the species. */
const CANOPY_RISE = 0.7;

/** ⛰️ How far the canopy rises above its rim at most, in heights of the species: the dome of the plain canopy, stretched by a quarter while it snaps open. */
const CANOPY_DOME = 0.5625;
//#endregion 🔖️Constants

//#region 🔖️Bodies
/** 🙃️ The box of a body whose drawing is turned by `sine` and `cosine` (of its tilt) about its scruff, the feet being at `feet`: the size box and the hover beneath it, turned about the scruff — `grip` above where the feet would stand upright —, and `MARGIN` on every side. */
function turned(feet: Point, sine: number, cosine: number, kind: Species): Extent {
  const grip = kind.grip;
  const half = kind.size.width / 2;
  const pivotX = feet.x + grip * sine;
  const pivotY = feet.y - grip * cosine;
  const top = grip - kind.size.height;
  const bottom = grip + hoverOf(kind);
  const across = half * Math.abs(cosine);
  const down = half * Math.abs(sine);
  const leftTop = 0 - sine * top;
  const leftBottom = 0 - sine * bottom;
  const highTop = cosine * top;
  const highBottom = cosine * bottom;
  return {
    x0: pivotX + Math.min(leftTop, leftBottom) - across - MARGIN,
    y0: pivotY + Math.min(highTop, highBottom) - down - MARGIN,
    x1: pivotX + Math.max(leftTop, leftBottom) + across + MARGIN,
    y1: pivotY + Math.max(highTop, highBottom) + down + MARGIN,
  };
}

/** ☂️ The box of the open canopy above a pet whose feet are at `feet` while its canopy holds the cords at `canopy`: the canopy of the depiction — its rim `CANOPY_RISE` heights above the scruff, its dome up to `CANOPY_DOME` heights above the rim, `CANOPY_SPAN` widths to either side —, leaning with the sway of the cords about the scruff, and `MARGIN` on every side. */
function canopyExtent(feet: Point, canopy: Point, kind: Species): Extent {
  const height = kind.size.height;
  const cords = CHUTE_ROD * height;
  const sine = (canopy.x - feet.x) / cords;
  const cosine = (feet.y - canopy.y) / cords;
  const pivotX = feet.x;
  const pivotY = feet.y - kind.grip;
  const span = CANOPY_SPAN * kind.size.width;
  const rim = 0 - CANOPY_RISE * height;
  const dome = rim - CANOPY_DOME * height;
  const across = span * Math.abs(cosine);
  const down = span * Math.abs(sine);
  const leftRim = 0 - sine * rim;
  const leftDome = 0 - sine * dome;
  const highRim = cosine * rim;
  const highDome = cosine * dome;
  return {
    x0: pivotX + Math.min(leftRim, leftDome) - across - MARGIN,
    y0: pivotY + Math.min(highRim, highDome) - down - MARGIN,
    x1: pivotX + Math.max(leftRim, leftDome) + across + MARGIN,
    y1: pivotY + Math.max(highRim, highDome) + down + MARGIN,
  };
}

/** 📐️ The body of an actor of `kind` named `owner` with its feet at `feet`, its drawing tilted by `tilt` about the scruff and, while a parachute is out, its canopy holding the cords at `canopy`: upright it is `bodyOf` with the size box, the hover and `MARGIN`; tilted it is that box turned about the scruff; a canopy adds its own box. */
export function extentAt(owner: string, kind: Species, feet: Point, tilt: Turns, canopy: Point | null): Extent {
  const body = tilt === 0 ? bodyOf(owner, feet, kind.size, hoverOf(kind), UPRIGHT, MARGIN).extent : turned(feet, sinTurns(tilt), cosTurns(tilt), kind);
  return canopy === null ? body : united(body, canopyExtent(feet, canopy, kind));
}

/** 🦎️ How far a body of `kind` with its feet at `x` on `footing`, holding `pitch` (or `null`), leans towards its wall, in turns: on a wall {@link WALL_LEAN} towards the wall of its pitch (a wall's `side` is the side of its element it bounds, so the wall is on the other side of the climber) while its feet are on the line it clings to (`clingOf`), less and less as they leave that line and nothing once they are half a width from it — a body that mantles over the rim onto the top or steps off onto a perch beside the foot rights itself on the way; nothing off a wall. The drawing leans as far as the body. */
export function wallLeanOf(kind: Species, footing: Footing, pitch: Pitch | null, x: number): Turns {
  if (footing !== "wall" || pitch === null) return 0;
  const share = clamp(1 - Math.abs(x - clingOf(pitch, kind.size)) / (kind.size.width / 2), 0, 1);
  return (pitch.side > 0 ? -1 : 1) * WALL_LEAN * share;
}

/** 🧸️ The body of an actor as it is: {@link extentAt} of its feet, its tilt with the lean towards the wall it clings to ({@link wallLeanOf}) and the canopy of its parachute. */
export function extentOf(actor: Actor, kind: Species): Extent {
  const chute = actor.chute;
  return extentAt(actor.species, kind, { x: actor.x, y: actor.y }, actor.tilt + wallLeanOf(kind, actor.footing, actor.pitch, actor.x), chute === null ? null : { x: chute.canopy.x, y: chute.canopy.y });
}

/** 👪️ The bodies of the actors, in their order: every actor on stage is one. */
export function bodiesOf(actors: readonly Actor[], kinds: readonly Species[]): Solid[] {
  const bodies: Solid[] = [];
  for (let index = 0; index < actors.length; index++) bodies.push({ owner: actors[index]!.species, extent: extentOf(actors[index]!, kinds[index]!) });
  return bodies;
}

/** 🧱️ Everything an actor has to stay clear of from tick `now` on: the bodies of all others, every corridor of a plan that is not over yet and every place where a plan comes to rest (`obstaclesOf`). */
export function obstaclesFor(draft: Draft, index: number, now: Ticks): Extent[] {
  return obstaclesOf(draft.actors[index]!.species, bodiesOf(draft.actors, draft.kinds), draft.claims, now);
}
//#endregion 🔖️Bodies

//#region 🔖️Room
/** 🪂️ Whether the arc of a hop keeps clear of what must stay free: on every tick on which the feet are above both ends of the hop (lower down the body is beside the things it hops between), the box of the body touches no keep-out and stays below the top of the stage. */
export function soars(draft: Draft, kind: Species, from: Point, to: Point, hop: Hop): boolean {
  const half = kind.size.width / 2;
  const ridge = Math.min(from.y, to.y);
  let flight: Flight = { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
  for (let left = hop.ticks; left > 1; left--) {
    flight = hopStep(flight.x, flight.y, flight.vx, flight.vy, to, left);
    if (flight.y >= ridge) continue;
    const top = flight.y - kind.size.height;
    if (top < 0) return false;
    for (const keepout of draft.keepouts) {
      if (keepout.width > 0 && keepout.height > 0 && Math.max(flight.x - half, keepout.x) < Math.min(flight.x + half, keepout.x + keepout.width) && Math.max(top, keepout.y) < Math.min(flight.y, keepout.y + keepout.height)) return false;
    }
  }
  return true;
}

/** 🛣️ The stretch `[low, high]` of its perch an actor can walk on without its body coming closer than `gap` (and a seam) to anybody else on that surface (`except` aside; a walker or a scooter counts for the whole way it still has to go). The stretch always holds the place where the actor stands, and is that place alone for an actor that does not stand on a perch. */
export function clearway(draft: Draft, index: number, gap: number, except: number): readonly [number, number] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const perch = body.footing !== "perch" || body.perch === null ? null : perchAt(draft.perches, body.perch, body.x);
  if (perch === null) return [body.x, body.x];
  let low = perch.x0 + kind.size.width / 2;
  let high = perch.x1 - kind.size.width / 2;
  for (let other = 0; other < draft.actors.length; other++) {
    const neighbour = draft.actors[other]!;
    if (other === index || other === except || neighbour.footing !== "perch" || neighbour.perch !== body.perch) continue;
    const room = shoulders(draft.kinds[other]!, kind) + gap + SEAM;
    const walks = neighbour.activity === "walk" || neighbour.activity === "scoot";
    const left = walks && neighbour.goal < neighbour.x ? neighbour.goal : neighbour.x;
    const right = walks && neighbour.goal > neighbour.x ? neighbour.goal : neighbour.x;
    if (right <= body.x) low = Math.max(low, right + room);
    else if (left >= body.x) high = Math.min(high, left - room);
    else {
      low = body.x;
      high = body.x;
    }
  }
  return [Math.min(low, body.x), Math.max(high, body.x)];
}

/** 🛏️ Where a species could arrive: per perch that carries it the stretches where its feet can stand with its body free of every body on stage, every corridor of a plan and every place where a plan comes to rest (a seam to spare), and how many stand on the perch. A perch without such a stretch is full. */
export function roomsFor(draft: Draft, kind: Species): Room[] {
  const rooms: Room[] = [];
  const half = kind.size.width / 2;
  const hover = hoverOf(kind);
  const obstacles = obstaclesOf(kind.id, bodiesOf(draft.actors, draft.kinds), draft.claims, draft.tick);
  for (const perch of draft.perches) {
    if (perch.x1 - half < perch.x0 + half) continue;
    const box = bodyOf(kind.id, { x: 0, y: perch.y - hover }, kind.size, hover, UPRIGHT, MARGIN).extent;
    let stretches: number[] = [perch.x0 + half, perch.x1 - half];
    for (const obstacle of obstacles) {
      if (!(box.y0 < obstacle.y1 && obstacle.y0 < box.y1)) continue;
      stretches = carve(stretches, obstacle.x0 - box.x1 - SEAM, obstacle.x1 - box.x0 + SEAM);
    }
    if (stretches.length === 0) continue;
    let span = 0;
    for (let index = 0; index < stretches.length; index += 2) span = span + (stretches[index + 1]! - stretches[index]!);
    rooms.push({ perch, stretches, span, crowd: crowdOn(draft, perch) });
  }
  return rooms;
}

/** 🏘️ The rooms a newcomer chooses among, so that a company spreads out over what the stage offers: those that hold the fewest actors, and of these the ones on the ground — the lowest perches of the stage — only when no higher one is as empty. Pets stand on things before they stand beneath them. */
export function quarters(draft: Draft, rooms: readonly Room[]): Room[] {
  let fewest = Infinity;
  for (const room of rooms) if (room.crowd < fewest) fewest = room.crowd;
  let ground = 0 - Infinity;
  for (const perch of draft.perches) if (perch.y > ground) ground = perch.y;
  const emptiest: Room[] = [];
  const raised: Room[] = [];
  for (const room of rooms) {
    if (room.crowd !== fewest) continue;
    emptiest.push(room);
    if (room.perch.y < ground) raised.push(room);
  }
  return raised.length > 0 ? raised : emptiest;
}
//#endregion 🔖️Room
