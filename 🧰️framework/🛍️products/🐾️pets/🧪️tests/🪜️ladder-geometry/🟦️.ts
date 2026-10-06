/** 🪜️ Subject adapter of the ladder-geometry case: `ladderFor` with its measures, `ladderTo` with the feet at its exit, the climb along a ladder, `ladderHolds` and `spillOf` of the climbing module of `@semio-tech/pets` answer every committed vector tick by tick.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🧗️climbing/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { LADDER_DESCENT, LADDER_DISMOUNT_TICKS, LADDER_EXIT, LADDER_FLAT, LADDER_FOLLOW, LADDER_FOOTING, LADDER_GIRTH, LADDER_HORNS, LADDER_IDLE, LADDER_LEAN, LADDER_LIFE, LADDER_MOUNT_TICKS, LADDER_RAISE_TICKS, LADDER_RAMP, LADDER_RISE, LADDER_SHIFT, LADDER_SHORT, LADDER_STEEP, LADDER_TALL, LADDER_TUCK, RUNG_SPACING, TOPPLE_DAMPING, TOPPLE_PUSH, TOPPLE_STEP, TOPPLE_STIFFNESS, ladderAt, ladderExit, ladderFor, ladderHolds, ladderLanding, ladderLean, ladderLength, ladderPhase, ladderRungs, ladderStep, ladderTo, spillOf } from "../../🔨️modules/🧗️climbing/🟦️.ts";
import type { Perch, Pitch, Rect, Size } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🪜️ladder-geometry/🔣️.json";
const PATIENCE = 1024;

type Ladder = NonNullable<ReturnType<typeof ladderFor>>;
type Vectors = {
  readonly placements: readonly { readonly id: string; readonly size: Size; readonly low: Perch; readonly high: Perch; readonly pitch: Pitch; readonly keepouts: readonly Rect[] }[];
  readonly leans: readonly { readonly id: string; readonly size: Size; readonly low: Perch; readonly pitch: Pitch; readonly keepouts: readonly Rect[] }[];
  readonly climbs: readonly { readonly id: string; readonly size: Size; readonly ladder: Ladder; readonly down: boolean }[];
  readonly surveys: readonly { readonly id: string; readonly size: Size; readonly ladder: Ladder; readonly perches: readonly Perch[]; readonly pitches: readonly Pitch[]; readonly keepouts: readonly Rect[] }[];
  readonly spills: readonly { readonly id: string; readonly size: Size; readonly ladder: Ladder; readonly heights: readonly number[] }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🏗️ The ladder of a placement with its measures, `null` when none stands. */
function placed(vector: Vectors["placements"][number]): unknown {
  const ladder = ladderFor(vector.low, vector.high, vector.pitch, vector.keepouts, vector.size);
  return ladder === null ? null : { length: ladderLength(ladder), rungs: ladderRungs(ladder), lean: ladderLean(ladder), exit: ladderExit(ladder, vector.size), landing: ladderLanding(ladder, vector.size), ladder };
}

/** 🪤️ The ladder leaned against a wall to take hold of it, with the feet of a climber at its exit, `null` when none stands. */
function leaned(vector: Vectors["leans"][number]): unknown {
  const ladder = ladderTo(vector.low, vector.pitch, vector.keepouts, vector.size);
  return ladder === null ? null : { ladder, exit: ladderAt(ladder, ladderExit(ladder, vector.size)) };
}

/** 🐛️ The distance, the feet and the phase of the clip after every tick of a climb between the foot of a ladder and its exit. */
function climbed(vector: Vectors["climbs"][number]): { ticks: number; travels: number[]; feet: [number, number][]; phases: number[] } {
  const exit = ladderExit(vector.ladder, vector.size);
  const goal = vector.down ? 0 : exit;
  const travels: number[] = [];
  let travel = vector.down ? exit : 0;
  while (travel !== goal && travels.length < PATIENCE) {
    travel = ladderStep(travel, goal, travels.length);
    travels.push(travel);
  }
  return { ticks: travels.length, travels, feet: travels.map((reached) => ladderAt(vector.ladder, reached)).map((point) => [point.x, point.y]), phases: travels.map((reached) => ladderPhase(reached)) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    constants: { subject: () => ({ projection: { ladderLean: LADDER_LEAN, ladderSteep: LADDER_STEEP, ladderFlat: LADDER_FLAT, ladderShort: LADDER_SHORT, ladderTall: LADDER_TALL, ladderTuck: LADDER_TUCK, ladderHorns: LADDER_HORNS, rungSpacing: RUNG_SPACING, ladderFooting: LADDER_FOOTING, ladderGirth: LADDER_GIRTH, ladderRise: LADDER_RISE, ladderDescent: LADDER_DESCENT, ladderRamp: LADDER_RAMP, ladderExit: LADDER_EXIT, ladderFollow: LADDER_FOLLOW, ladderShift: LADDER_SHIFT, ladderMountTicks: LADDER_MOUNT_TICKS, ladderDismountTicks: LADDER_DISMOUNT_TICKS, ladderRaiseTicks: LADDER_RAISE_TICKS, ladderIdle: LADDER_IDLE, ladderLife: LADDER_LIFE, toppleStiffness: TOPPLE_STIFFNESS, toppleDamping: TOPPLE_DAMPING, topplePush: TOPPLE_PUSH, toppleStep: TOPPLE_STEP } }) },
    placements: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).placements.map((vector) => [vector.id, placed(vector)])) }) },
    leans: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).leans.map((vector) => [vector.id, leaned(vector)])) }) },
    climbs: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).climbs.map((vector) => [vector.id, climbed(vector)])) }) },
    surveys: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).surveys.map((vector) => [vector.id, ladderHolds(vector.ladder, vector.perches, vector.pitches, vector.keepouts, vector.size)])) }) },
    spills: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).spills.map((vector) => [vector.id, vector.heights.map((height) => spillOf(vector.ladder, height, vector.size))])) }) },
  },
});
