/** 🎣️ Subject adapter of the grapple-reach case: `shotFor`, `hookStep`, the hauls `zipStep` and `swayStep`, `shotHolds`, `landingFor`, `missOf` and `routeOf` of the climbing module of `@semio-tech/pets` answer every committed vector tick by tick.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🧗️climbing/🟦️.ts
 * @see ../../🔨️modules/🪢️swing/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { HOOK_INSET, HOOK_LIFT, HOOK_RETURN, HOOK_SPEED, type Haul, MUZZLE_FORWARD, MUZZLE_HEIGHT, ROPE_AIM_TICKS, ROPE_DETOUR, ROPE_ELEVATION, ROPE_FOLLOW, ROPE_HOIST_TICKS, ROPE_LONG, ROPE_MARGIN, ROPE_MISS_CHANCE, ROPE_MISS_OVERSHOOT, ROPE_RECOIL_TICKS, ROPE_REST, ROPE_RISE, ROPE_SHORT, ROPE_SHRUG_TICKS, ROPE_SULK, ROPE_TUG_TICKS, ZIP_RAMP, ZIP_SLANT, ZIP_SPEED, haulOf, hookStep, hookTicks, type ladderFor, landingFor, missOf, routeOf, shotFor, shotHolds, swayStep, zipStep } from "../../🔨️modules/🧗️climbing/🟦️.ts";
import { REEL_LEAST } from "../../🔨️modules/🪢️swing/🟦️.ts";
import type { Gear, Perch, Pitch, Point, Rect, Size } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🎣️grapple-reach/🔣️.json";
const PATIENCE = 1024;

type Shot = NonNullable<ReturnType<typeof shotFor>>;
type Ladder = NonNullable<ReturnType<typeof ladderFor>>;
type Hauled = { readonly id: string; readonly size: Size; readonly shot: Shot };
type Vectors = {
  readonly shots: readonly { readonly id: string; readonly size: Size; readonly feet: Point; readonly perches: readonly Perch[]; readonly keepouts: readonly Rect[] }[];
  readonly flights: readonly { readonly id: string; readonly from: Point; readonly to: Point; readonly speed: number }[];
  readonly zips: readonly Hauled[];
  readonly swings: readonly Hauled[];
  readonly surveys: readonly { readonly id: string; readonly shot: Shot; readonly perches: readonly Perch[]; readonly keepouts: readonly Rect[] }[];
  readonly landings: readonly { readonly id: string; readonly size: Size; readonly shot: Shot; readonly perch: Perch }[];
  readonly routes: readonly { readonly id: string; readonly size: Size; readonly gear: readonly Gear[]; readonly grip: number; readonly x: number; readonly from: number; readonly to: number; readonly perches: readonly Perch[]; readonly pitches: readonly Pitch[]; readonly ladders: readonly Ladder[]; readonly keepouts: readonly Rect[] }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🏹️ The ticks of a flight and the hook after every one of them, from tick 0 to one tick after its arrival. */
function flown(from: Point, to: Point, speed: number): { ticks: number; path: [number, number][] } {
  const ticks = hookTicks(from, to, speed);
  return { ticks, path: Array.from({ length: ticks + 2 }, (_, tick) => hookStep(from, to, speed, tick)).map((point) => [point.x, point.y]) };
}

/** 🎬️ The rope, the hands and the feet of a haul after every tick until the least rope is left. */
function hauled(vector: Hauled, step: (shot: Shot, haul: Haul, ticks: number, size: Size) => Haul): { ticks: number; ropes: number[]; hands: [number, number][]; feet: [number, number][] } {
  const least = REEL_LEAST * vector.size.height;
  const states: Haul[] = [];
  let haul = haulOf(vector.shot, vector.shot.length, vector.size);
  while (haul.rope > least && states.length < PATIENCE) {
    haul = step(vector.shot, haul, states.length, vector.size);
    states.push(haul);
  }
  return { ticks: states.length, ropes: states.map((state) => state.rope), hands: states.map((state) => [state.hand.x, state.hand.y]), feet: states.map((state) => [state.x, state.y]) };
}

/** 🗾️ The ways between two perches of a stage, a standing ladder and a pitch named by their index. */
function routed(vector: Vectors["routes"][number]): unknown {
  const legs = routeOf(vector.x, vector.perches[vector.from]!, vector.perches[vector.to]!, vector.gear, vector.size, vector.grip, vector.pitches, vector.ladders, vector.keepouts);
  if (legs === null) return null;
  return legs.map((leg) => (leg.means === "ladder" ? { means: leg.means, at: leg.at, up: leg.up, ladder: vector.ladders.indexOf(leg.ladder) } : leg.means === "wall" ? { means: leg.means, at: leg.at, hold: leg.hold, goal: leg.goal, pitch: vector.pitches.indexOf(leg.pitch), exit: vector.pitches.indexOf(leg.exit) } : leg));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    constants: { subject: () => ({ projection: { hookSpeed: HOOK_SPEED, hookReturn: HOOK_RETURN, ropeShort: ROPE_SHORT, ropeLong: ROPE_LONG, ropeElevation: ROPE_ELEVATION, ropeMargin: ROPE_MARGIN, ropeDetour: ROPE_DETOUR, ropeFollow: ROPE_FOLLOW, ropeRise: ROPE_RISE, muzzleForward: MUZZLE_FORWARD, muzzleHeight: MUZZLE_HEIGHT, hookInset: HOOK_INSET, hookLift: HOOK_LIFT, zipSlant: ZIP_SLANT, zipSpeed: ZIP_SPEED, zipRamp: ZIP_RAMP, ropeAimTicks: ROPE_AIM_TICKS, ropeRecoilTicks: ROPE_RECOIL_TICKS, ropeTugTicks: ROPE_TUG_TICKS, ropeHoistTicks: ROPE_HOIST_TICKS, ropeShrugTicks: ROPE_SHRUG_TICKS, ropeMissChance: ROPE_MISS_CHANCE, ropeMissOvershoot: ROPE_MISS_OVERSHOOT, ropeRest: ROPE_REST, ropeSulk: ROPE_SULK } }) },
    shots: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).shots.map((vector) => [vector.id, shotFor(vector.feet, vector.perches, vector.keepouts, vector.size)])) }) },
    flights: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).flights.map((vector) => [vector.id, flown(vector.from, vector.to, vector.speed)])) }) },
    zips: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).zips.map((vector) => [vector.id, hauled(vector, zipStep)])) }) },
    swings: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).swings.map((vector) => [vector.id, hauled(vector, swayStep)])) }) },
    surveys: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).surveys.map((vector) => [vector.id, shotHolds(vector.shot, vector.perches, vector.keepouts)])) }) },
    landings: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).landings.map((vector) => [vector.id, { landing: landingFor(vector.shot, vector.perch, vector.size), miss: missOf(vector.shot, vector.perch) }])) }) },
    routes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).routes.map((vector) => [vector.id, routed(vector)])) }) },
  },
});
