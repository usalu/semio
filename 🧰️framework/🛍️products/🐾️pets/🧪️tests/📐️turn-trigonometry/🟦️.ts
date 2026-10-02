/** 📐️ Subject adapter of the turn-trigonometry case: the pets trigonometry module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/📐️trigonometry/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { clamp, cosTurns, lerp, sinTurns, smoothstep } from "../../🔨️modules/📐️trigonometry/🟦️.ts";

const VECTORS = "shared://📐️turn-trigonometry/🔣️.json";

type Vectors = {
  readonly angles: readonly { readonly id: string; readonly numerator: number; readonly denominator: number }[];
  readonly sweeps: readonly { readonly id: string; readonly denominator: number; readonly first: number; readonly last: number }[];
  readonly clamps: readonly { readonly id: string; readonly value: number; readonly low: number; readonly high: number }[];
  readonly lerps: readonly { readonly id: string; readonly from: number; readonly to: number; readonly amount: number }[];
  readonly smoothsteps: readonly { readonly id: string; readonly amount: number }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

const VIEW = new DataView(new ArrayBuffer(8));

/** 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits. */
function bits(value: number): string {
  VIEW.setFloat64(0, value);
  return VIEW.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🌊️ Sines and cosines of `n ÷ denominator` turns for every `n` from `first` to `last`. */
function sweep(denominator: number, first: number, last: number): { sines: number[]; cosines: number[] } {
  const sines: number[] = [];
  const cosines: number[] = [];
  for (let step = first; step <= last; step++) {
    sines.push(sinTurns(step / denominator));
    cosines.push(cosTurns(step / denominator));
  }
  return { sines, cosines };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    angles: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).angles.map((vector) => [vector.id, { sine: sinTurns(vector.numerator / vector.denominator), cosine: cosTurns(vector.numerator / vector.denominator) }])) }) },
    "bit-patterns": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).angles.map((vector) => [vector.id, { sine: bits(sinTurns(vector.numerator / vector.denominator)), cosine: bits(cosTurns(vector.numerator / vector.denominator)) }])) }) },
    sweeps: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).sweeps.map((vector) => [vector.id, sweep(vector.denominator, vector.first, vector.last)])) }) },
    clamps: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).clamps.map((vector) => [vector.id, clamp(vector.value, vector.low, vector.high)])) }) },
    lerps: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).lerps.map((vector) => [vector.id, lerp(vector.from, vector.to, vector.amount)])) }) },
    smoothsteps: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).smoothsteps.map((vector) => [vector.id, smoothstep(vector.amount)])) }) },
  },
});
