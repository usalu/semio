/** 👀️ Subject adapter of the gaze-tracking case: the pets rig module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🦴️rig/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { type Affine, lookOffset, transform } from "../../🔨️modules/🦴️rig/🟦️.ts";
import type { Point } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://👀️gaze-tracking/🔣️.json";

type Vectors = {
  readonly offsets: readonly { readonly id: string; readonly eye: Point; readonly target: Point; readonly reach: number }[];
  readonly eyes: readonly { readonly id: string; readonly bone: Affine; readonly eye: Point; readonly target: Point; readonly reach: number }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 👁️ Where an eye on a posed bone sits in the pet's frame, and the offset of its pupil towards the target. */
function eyeLook(bone: Affine, eye: Point, target: Point, reach: number): { eye: Point; offset: Point } {
  const carried = transform(bone, eye.x, eye.y);
  return { eye: carried, offset: lookOffset(carried, target, reach) };
}

const VIEW = new DataView(new ArrayBuffer(8));

/** 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits. */
function bits(value: number): string {
  VIEW.setFloat64(0, value);
  return VIEW.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🔬️ The bit patterns of a point. */
function pointBits(point: Point): { x: string; y: string } {
  return { x: bits(point.x), y: bits(point.y) };
}

/** 🧬️ The bit patterns of every offset and of every carried eye with its offset. */
function patterns(document: Vectors): Record<string, Record<string, unknown>> {
  return {
    offsets: Object.fromEntries(document.offsets.map((vector) => [vector.id, pointBits(lookOffset(vector.eye, vector.target, vector.reach))])),
    eyes: Object.fromEntries(
      document.eyes.map((vector) => {
        const look = eyeLook(vector.bone, vector.eye, vector.target, vector.reach);
        return [vector.id, { eye: pointBits(look.eye), offset: pointBits(look.offset) }];
      }),
    ),
  };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "bit-patterns": { subject: (ctx) => ({ projection: patterns(vectors(ctx)) }) },
    offsets: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).offsets.map((vector) => [vector.id, lookOffset(vector.eye, vector.target, vector.reach)])) }) },
    eyes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).eyes.map((vector) => [vector.id, eyeLook(vector.bone, vector.eye, vector.target, vector.reach)])) }) },
  },
});
