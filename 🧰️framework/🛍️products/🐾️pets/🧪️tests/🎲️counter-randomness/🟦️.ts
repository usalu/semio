/** 🎲️ Subject adapter of the counter-randomness case: the pets randomness module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🎲️randomness/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { randomBetween, randomPick, randomUnit, randomWords } from "../../🔨️modules/🎲️randomness/🟦️.ts";

const VECTORS = "shared://🎲️counter-randomness/🔣️.json";

type Vectors = {
  readonly words: readonly { readonly id: string; readonly key: readonly number[]; readonly count: number }[];
  readonly units: readonly { readonly id: string; readonly key: readonly number[] }[];
  readonly streams: readonly { readonly id: string; readonly seed: number; readonly stream: number; readonly count: number }[];
  readonly ranges: readonly { readonly id: string; readonly key: readonly number[]; readonly low: number; readonly high: number }[];
  readonly picks: readonly { readonly id: string; readonly seed: number; readonly stream: number; readonly count: number; readonly weights: readonly number[] }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🧵️ One answer per counter `0 … count − 1` of the stream `[seed, stream, counter]`. */
function counted<T>(seed: number, stream: number, count: number, draw: (key: readonly number[]) => T): T[] {
  return Array.from({ length: count }, (_, counter) => draw([seed, stream, counter]));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    words: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).words.map((vector) => [vector.id, randomWords(vector.key, vector.count)])) }) },
    units: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).units.map((vector) => [vector.id, randomUnit(vector.key)])) }) },
    streams: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).streams.map((vector) => [vector.id, counted(vector.seed, vector.stream, vector.count, randomUnit)])) }) },
    ranges: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).ranges.map((vector) => [vector.id, randomBetween(vector.key, vector.low, vector.high)])) }) },
    picks: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).picks.map((vector) => [vector.id, counted(vector.seed, vector.stream, vector.count, (key) => randomPick(key, vector.weights))])) }) },
  },
});
