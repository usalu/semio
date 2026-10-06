/** 🎲️ Subject adapter of the seeded-randomness case: `@semio-tech/quiz` answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🎲️randomness/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { Mt19937, fnv1a32, runSeed, shuffle, uniformIndex } from "../../📦️packages/🟦️typescript/🟦️.ts";

const VECTORS = "shared://🎲️seeded-randomness/🔣️.json";

type Vectors = {
  readonly hashes: readonly { readonly id: string; readonly text: string }[];
  readonly runSeeds: readonly { readonly id: string; readonly run: string }[];
  readonly rawOutputs: readonly { readonly id: string; readonly seed: number; readonly skip: number; readonly count: number }[];
  readonly uniformDraws: readonly { readonly id: string; readonly seed: number; readonly bounds: readonly number[] }[];
  readonly shuffles: readonly { readonly id: string; readonly seed: number; readonly length: number }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🔢️ The words after skipping. */
function words(seed: number, skip: number, count: number): number[] {
  const random = new Mt19937(seed);
  return Array.from({ length: skip + count }, () => random.next()).slice(skip);
}

/** 🎯️ Sequential bounded draws, then the next raw word. */
function draws(seed: number, bounds: readonly number[]): { draws: number[]; next: number } {
  const random = new Mt19937(seed);
  const drawn = bounds.map((bound) => uniformIndex(random, bound));
  return { draws: drawn, next: random.next() };
}

/** 🃏️ A shuffled index range, then the next raw word. */
function permutation(seed: number, length: number): { permutation: number[]; next: number } {
  const random = new Mt19937(seed);
  const shuffled = shuffle(random, Array.from({ length }, (_, index) => index));
  return { permutation: shuffled, next: random.next() };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    hashes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).hashes.map((vector) => [vector.id, fnv1a32(vector.text)])) }) },
    "run-seeds": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).runSeeds.map((vector) => [vector.id, runSeed(vector.run)])) }) },
    "raw-outputs": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).rawOutputs.map((vector) => [vector.id, words(vector.seed, vector.skip, vector.count)])) }) },
    "uniform-draws": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).uniformDraws.map((vector) => [vector.id, draws(vector.seed, vector.bounds)])) }) },
    shuffles: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).shuffles.map((vector) => [vector.id, permutation(vector.seed, vector.length)])) }) },
  },
});
