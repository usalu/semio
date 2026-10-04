/** 🧗️ Subject adapter of the wall-climbing case: `wallsOf`, `wallAt`, `nearestWall`, `segmentHits` and `segmentClear` of the terrain and the holds, climbs, grips, slides, mantles and wall routes of the climbing module of `@semio-tech/pets` answer every committed vector tick by tick.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🏞️terrain/🟦️.ts
 * @see ../../🔨️modules/🧗️climbing/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { WALL_LIP, nearestWall, segmentClear, segmentHits, wallAt, wallsOf } from "../../🔨️modules/🏞️terrain/🟦️.ts";
import { CLIMB_DESCENT, CLIMB_RAMP, CLIMB_RISE, CROSS_REACH, type Effort, GRIP_BITE, GRIP_BUDGET, GRIP_CLIMB, GRIP_HANG, GRIP_REST, GRIP_SPACING, HAND_HEIGHT, HOIST_HUMP, HOIST_RISE, LUNGE_TICKS, MANTLE_INSET, MANTLE_TICKS, SLIDE_GAIN, SLIDE_SPEED, SLIDE_START, SLIP_LIFT, SLIP_PUSH, WALL_FOLLOW, WALL_GRAB_TICKS, WALL_HANG_TICKS, chainOf, climbPhase, climbStep, climbTicks, clingOf, crossable, footOf, gripFor, gripStep, hoistPath, ledgeOf, mantlePath, rimFor, rimOf, routeOf, slideStep, slipOf, wallCost, wallHolds, wallPath } from "../../🔨️modules/🧗️climbing/🟦️.ts";
import type { Perch, Pitch, Point, Rect, Size } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🧗️wall-climbing/🔣️.json";

type Wall = Parameters<typeof wallsOf>[0][number];
type Vectors = {
  readonly layouts: readonly { readonly id: string; readonly width: number; readonly height: number; readonly clearance: number; readonly minimum: number; readonly walls: readonly Wall[]; readonly keepouts: readonly Rect[] }[];
  readonly stretches: readonly { readonly id: string; readonly pitches: readonly Pitch[]; readonly queries: readonly { readonly wall: string; readonly y: number }[] }[];
  readonly nearests: readonly { readonly id: string; readonly pitches: readonly Pitch[]; readonly points: readonly Point[] }[];
  readonly sights: readonly { readonly id: string; readonly margin: number; readonly rects: readonly Rect[]; readonly segments: readonly { readonly from: Point; readonly to: Point }[] }[];
  readonly holds: readonly { readonly id: string; readonly size: Size; readonly perches: readonly Perch[]; readonly pitches: readonly Pitch[] }[];
  readonly surveys: readonly { readonly id: string; readonly size: Size; readonly pitch: Pitch; readonly y: number; readonly pitches: readonly Pitch[] }[];
  readonly climbs: readonly { readonly id: string; readonly size: Size; readonly pitch: Pitch; readonly from: number; readonly goal: number }[];
  readonly grips: readonly { readonly id: string; readonly grip: number; readonly runs: readonly (readonly [Effort, number])[] }[];
  readonly slides: readonly { readonly id: string; readonly y: number; readonly vy: number; readonly floor: number; readonly ticks: number }[];
  readonly mantles: readonly { readonly id: string; readonly size: Size; readonly pitch: Pitch }[];
  readonly hoists: readonly { readonly id: string; readonly from: Point; readonly to: Point; readonly height: number; readonly ticks: number }[];
  readonly routes: readonly { readonly id: string; readonly size: Size; readonly grip: number; readonly x: number; readonly from: number; readonly to: number; readonly perches: readonly Perch[]; readonly pitches: readonly Pitch[] }[];
  readonly crossings: readonly { readonly id: string; readonly size: Size; readonly pitches: readonly Pitch[]; readonly ways: readonly { readonly pitch: number; readonly entry: "grab" | "hang" | "cling"; readonly x: number; readonly y: number; readonly end: number; readonly goal: number; readonly mantle: boolean }[] }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🔢️ The index of an answered entry in the list it was answered from, `null` for none. */
function indexOf<Entry>(entries: readonly Entry[], entry: Entry | null): number | null {
  return entry === null ? null : entries.indexOf(entry);
}

/** 🧤️ The hold of every perch on every pitch of a stage, the perch that crowns every pitch, and the measures of every pitch. */
function held(stage: Vectors["holds"][number]): unknown {
  return {
    grips: stage.perches.map((perch) => stage.pitches.map((pitch) => gripFor(perch, pitch, stage.size))),
    rims: stage.pitches.map((pitch) => indexOf(stage.perches, rimFor(pitch, stage.perches, stage.size))),
    measures: stage.pitches.map((pitch) => ({ cling: clingOf(pitch, stage.size), ledge: ledgeOf(pitch, stage.size), rim: rimOf(pitch, stage.size), foot: footOf(pitch, stage.size) })),
  };
}

/** 🧐️ The pitch that still carries a climber after a survey, and the throw when none does. */
function surveyed(vector: Vectors["surveys"][number]): unknown {
  const pitch = wallHolds(vector.pitch, vector.pitches, vector.y, vector.size);
  return { pitch: indexOf(vector.pitches, pitch), slip: pitch === null ? slipOf(vector.pitch) : null };
}

/** 🐜️ The height and the phase of the clip after every tick of a climb that arrives, and its ticks. */
function climbed(vector: Vectors["climbs"][number]): { ticks: number; heights: number[]; phases: number[] } {
  const ticks = climbTicks(vector.from, vector.goal);
  const heights: number[] = [];
  let height = vector.from;
  for (let tick = 0; tick < ticks && ticks <= GRIP_BUDGET; tick++) {
    height = climbStep(height, vector.goal, tick);
    heights.push(height);
  }
  return { ticks, heights, phases: heights.map((reached) => climbPhase(vector.pitch, reached, vector.size)) };
}

/** 🔋️ The grip after every run of ticks of one effort. */
function gripped(grip: number, runs: readonly (readonly [Effort, number])[]): number[] {
  const grips: number[] = [];
  let left = grip;
  for (const [effort, ticks] of runs) {
    for (let tick = 0; tick < ticks; tick++) left = gripStep(left, effort);
    grips.push(left);
  }
  return grips;
}

/** 🧈️ The heights and speeds of a slide after every tick. */
function slid(y: number, vy: number, floor: number, ticks: number): { heights: number[]; speeds: number[] } {
  const heights: number[] = [];
  const speeds: number[] = [];
  let slide = { y, vy };
  for (let tick = 0; tick < ticks; tick++) {
    slide = slideStep(slide.y, slide.vy, floor);
    heights.push(slide.y);
    speeds.push(slide.vy);
  }
  return { heights, speeds };
}

/** 🦘️ The feet after every tick of a path of `ticks` ticks, the start first. */
function traced(ticks: number, path: (phase: number) => Point): [number, number][] {
  return Array.from({ length: ticks + 1 }, (_, tick) => path(tick / ticks)).map((point) => [point.x, point.y]);
}

/** 🌉️ Which pitch of a stage lunges to which, the wall line of every pitch as indices, and every committed way along a line tick by tick (`[x, y, hold, work]`, the hold an index into its line) with the grip it costs. */
function crossed(vector: Vectors["crossings"][number]): unknown {
  const { pitches, size } = vector;
  return {
    crossable: pitches.map((from) => pitches.map((to) => crossable(from, to, size))),
    chains: pitches.map((pitch) => chainOf(pitch, pitches, size).map((member) => pitches.indexOf(member))),
    ways: vector.ways.map((wish) => {
      const chain = chainOf(pitches[wish.pitch]!, pitches, size);
      const path = wallPath(chain, chain.indexOf(pitches[wish.pitch]!), wish.entry, wish.x, wish.y, wish.end, wish.goal, wish.mantle, size);
      return { path: path.map((clamber) => [clamber.x, clamber.y, clamber.hold, clamber.work]), cost: wallCost(path) };
    }),
  };
}

/** 🗺️ The walls between two perches of a stage for an actor with the gear to climb. */
function routed(vector: Vectors["routes"][number]): unknown {
  const legs = routeOf(vector.x, vector.perches[vector.from]!, vector.perches[vector.to]!, ["climb"], vector.size, vector.grip, vector.pitches, [], []);
  return legs === null ? null : legs.map((leg) => (leg.means === "wall" ? { at: leg.at, hold: leg.hold, goal: leg.goal, pitch: vector.pitches.indexOf(leg.pitch), exit: vector.pitches.indexOf(leg.exit) } : leg));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    constants: { subject: () => ({ projection: { wallLip: WALL_LIP, handHeight: HAND_HEIGHT, climbRise: CLIMB_RISE, climbDescent: CLIMB_DESCENT, climbRamp: CLIMB_RAMP, gripSpacing: GRIP_SPACING, gripBudget: GRIP_BUDGET, gripClimb: GRIP_CLIMB, gripHang: GRIP_HANG, gripRest: GRIP_REST, gripBite: GRIP_BITE, wallFollow: WALL_FOLLOW, slideStart: SLIDE_START, slideGain: SLIDE_GAIN, slideSpeed: SLIDE_SPEED, slipPush: SLIP_PUSH, slipLift: SLIP_LIFT, wallGrabTicks: WALL_GRAB_TICKS, wallHangTicks: WALL_HANG_TICKS, mantleTicks: MANTLE_TICKS, mantleInset: MANTLE_INSET, hoistHump: HOIST_HUMP, hoistRise: HOIST_RISE, crossReach: CROSS_REACH, lungeTicks: LUNGE_TICKS } }) },
    pitches: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).layouts.map((layout) => [layout.id, wallsOf(layout.walls, layout.keepouts, layout.width, layout.height, layout.clearance, layout.minimum)])) }) },
    stretches: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).stretches.map((stand) => [stand.id, stand.queries.map((query) => indexOf(stand.pitches, wallAt(stand.pitches, query.wall, query.y)))])) }) },
    nearest: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).nearests.map((near) => [near.id, near.points.map((point) => indexOf(near.pitches, nearestWall(near.pitches, point.x, point.y)))])) }) },
    sights: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).sights.map((vector) => [vector.id, vector.segments.map((segment) => ({ hits: vector.rects.map((rect) => segmentHits(segment.from, segment.to, rect, vector.margin)), clear: segmentClear(segment.from, segment.to, vector.rects, vector.margin) }))])) }) },
    holds: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).holds.map((stage) => [stage.id, held(stage)])) }) },
    surveys: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).surveys.map((vector) => [vector.id, surveyed(vector)])) }) },
    climbs: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).climbs.map((vector) => [vector.id, climbed(vector)])) }) },
    grips: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).grips.map((vector) => [vector.id, gripped(vector.grip, vector.runs)])) }) },
    slides: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).slides.map((vector) => [vector.id, slid(vector.y, vector.vy, vector.floor, vector.ticks)])) }) },
    mantles: {
      subject: (ctx) => ({
        projection: Object.fromEntries([...vectors(ctx).mantles.map((vector) => [vector.id, traced(MANTLE_TICKS, (phase) => mantlePath(vector.pitch, vector.size, phase))] as const), ...vectors(ctx).hoists.map((vector) => [vector.id, traced(vector.ticks, (phase) => hoistPath(vector.from, vector.to, vector.height, phase))] as const)]),
      }),
    },
    routes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).routes.map((vector) => [vector.id, routed(vector)])) }) },
    crossings: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).crossings.map((vector) => [vector.id, crossed(vector)])) }) },
  },
});
