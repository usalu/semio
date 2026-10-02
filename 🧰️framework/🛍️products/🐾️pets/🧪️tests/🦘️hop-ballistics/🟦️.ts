/** 🦘️ Subject adapter of the hop-ballistics case: the constants, `fallStep`, `landingOf`, `hopOf`, `hopStep` and `hopLanding` of `@semio-tech/pets` answer every committed vector tick by tick.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🏞️terrain/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { FALL_SPEED, GRAVITY, HOP_CLEARANCE, HOP_DISTANCE, HOP_HEIGHT, HOP_STEEPNESS, HOP_TICKS, fallStep, hopLanding, hopOf, hopStep, landingOf } from "../../🔨️modules/🏞️terrain/🟦️.ts";
import type { Perch, Point } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🦘️hop-ballistics/🔣️.json";

type Ends = { readonly id: string; readonly from: Point; readonly to: Point };
type Vectors = {
  readonly falls: readonly { readonly id: string; readonly y: number; readonly vy: number; readonly ticks: number }[];
  readonly landings: readonly { readonly id: string; readonly perches: readonly Perch[]; readonly sweeps: readonly { readonly x: number; readonly fromY: number; readonly toY: number }[] }[];
  readonly drops: readonly { readonly id: string; readonly perches: readonly Perch[]; readonly start: { readonly x: number; readonly y: number; readonly vy: number }; readonly limit: number }[];
  readonly hops: readonly Ends[];
  readonly flights: readonly Ends[];
  readonly routes: readonly (Ends & { readonly perches: readonly Perch[] })[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🔢️ The index of an answered perch in the list it was answered from, `null` for none. */
function indexOf(perches: readonly Perch[], perch: Perch | null): number | null {
  return perch === null ? null : perches.indexOf(perch);
}

/** 🍂️ The heights and speeds of a fall after every tick. */
function fallen(y: number, vy: number, ticks: number): { heights: number[]; speeds: number[] } {
  const heights: number[] = [];
  const speeds: number[] = [];
  let fall = { y, vy };
  for (let tick = 0; tick < ticks; tick++) {
    fall = fallStep(fall.y, fall.vy);
    heights.push(fall.y);
    speeds.push(fall.vy);
  }
  return { heights, speeds };
}

/** 🪨️ A fall until it lands or its ticks run out: the perch, the ticks it took and the height it ends at. */
function dropped(perches: readonly Perch[], start: { readonly x: number; readonly y: number; readonly vy: number }, limit: number): { perch: number | null; ticks: number; y: number } {
  let fall = { y: start.y, vy: start.vy };
  for (let tick = 1; tick <= limit; tick++) {
    const next = fallStep(fall.y, fall.vy);
    const landing = landingOf(perches, start.x, fall.y, next.y);
    if (landing !== null) return { perch: perches.indexOf(landing), ticks: tick, y: landing.y };
    fall = next;
  }
  return { perch: null, ticks: limit, y: fall.y };
}

/** 🎈️ The feet of a granted hop after every tick, and its apex. */
function flown(from: Point, to: Point): { ticks: number; path: [number, number][]; apex: number } {
  const hop = hopOf(from, to);
  if (hop === null) throw new Error("the hop is out of reach");
  const path: [number, number][] = [];
  let flight = { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
  for (let left = hop.ticks; left >= 1; left--) {
    flight = hopStep(flight.x, flight.y, flight.vx, flight.vy, to, left);
    path.push([flight.x, flight.y]);
  }
  return { ticks: hop.ticks, path, apex: Math.min(...path.map(([, y]) => y)) };
}

/** 🎯️ The index of the perch a granted hop really ends on. */
function routed(perches: readonly Perch[], from: Point, to: Point): number | null {
  const hop = hopOf(from, to);
  if (hop === null) throw new Error("the hop is out of reach");
  return indexOf(perches, hopLanding(perches, from, to, hop));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    constants: { subject: () => ({ projection: { gravity: GRAVITY, fallSpeed: FALL_SPEED, hopClearance: HOP_CLEARANCE, hopSteepness: HOP_STEEPNESS, hopHeight: HOP_HEIGHT, hopDistance: HOP_DISTANCE, hopTicks: HOP_TICKS } }) },
    falls: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).falls.map((vector) => [vector.id, fallen(vector.y, vector.vy, vector.ticks)])) }) },
    landings: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).landings.map((vector) => [vector.id, vector.sweeps.map((sweep) => indexOf(vector.perches, landingOf(vector.perches, sweep.x, sweep.fromY, sweep.toY)))])) }) },
    drops: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).drops.map((vector) => [vector.id, dropped(vector.perches, vector.start, vector.limit)])) }) },
    hops: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).hops.map((vector) => [vector.id, hopOf(vector.from, vector.to)])) }) },
    flights: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).flights.map((vector) => [vector.id, flown(vector.from, vector.to)])) }) },
    routes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).routes.map((vector) => [vector.id, routed(vector.perches, vector.from, vector.to)])) }) },
  },
});
