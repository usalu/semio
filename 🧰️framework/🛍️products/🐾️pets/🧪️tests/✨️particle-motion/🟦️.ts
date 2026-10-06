/** ✨️ Subject adapter of the particle-motion case: the pets effects module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/✨️effects/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type Emission, type Particle, bornAt, capped, emitterEnds, lifeTicks, lowbias32, mix, particlesOf, periodOf, swarmOf, unit } from "../../🔨️modules/✨️effects/🟦️.ts";

const VECTORS = "shared://✨️particle-motion/🔣️.json";
const BINS = 16;

type Run = { readonly emitter: Emission; readonly since: number; readonly until: number | null; readonly key: number };

type Vectors = {
  readonly hashes: readonly { readonly id: string; readonly low?: number; readonly chain?: readonly number[] }[];
  readonly uniformity: readonly { readonly id: string; readonly key: number; readonly lane: number; readonly first: number; readonly count: number }[];
  readonly births: readonly (Run & { readonly id: string; readonly first: number; readonly last: number; readonly births: number })[];
  readonly motions: readonly (Run & { readonly id: string; readonly origin: { readonly x: number; readonly y: number }; readonly facing: 1 | -1; readonly ticks: readonly number[] })[];
  readonly caps: readonly { readonly id: string; readonly ages: readonly number[]; readonly cap: number }[];
  readonly ends: readonly { readonly id: string; readonly emitter: Emission; readonly since: number; readonly until: number | null }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 📊️ The sum of the words and the histogram of the units a lane yields for `count` indices from `first` on, over sixteen equal bins. */
function spread(key: number, lane: number, first: number, count: number): { total: number; bins: number[] } {
  const bins = new Array<number>(BINS).fill(0);
  let total = 0;
  for (let index = first; index < first + count; index++) {
    const word = mix(mix(key, index), lane);
    total += word;
    bins[Math.floor(unit(word) * BINS)]! += 1;
  }
  return { total, bins };
}

/** 🗓️ The births of the first particles and, tick by tick, the ages of those alive, eldest first. */
function lifetimes(run: Run, first: number, last: number, births: number): { life: number; swarm: number; period: number; born: number[]; ages: number[][] } {
  const ages: number[][] = [];
  for (let tick = first; tick <= last; tick++) ages.push(particlesOf(run.emitter, { x: 0, y: 0 }, 1, run.since, run.until, tick, run.key).map((particle) => particle.age));
  return { life: lifeTicks(run.emitter), swarm: swarmOf(run.emitter), period: periodOf(run.emitter), born: Array.from({ length: births }, (_, index) => bornAt(run.emitter, run.since, run.key, index)), ages };
}

/** ✂️ The positions of the particles that survive a cap, the particles being known by their ages alone. */
function kept(ages: readonly number[], cap: number): number[] {
  const particles: Particle[] = ages.map((age, position) => ({ x: position, y: 0, scale: 1, rotation: 0, opacity: 1, age }));
  return capped(particles, cap).map((particle) => particle.x);
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    hashes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).hashes.map((vector) => [vector.id, vector.low !== undefined ? lowbias32(vector.low) : vector.chain!.slice(1).reduce(mix, vector.chain![0]! >>> 0)])) }) },
    uniformity: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).uniformity.map((vector) => [vector.id, spread(vector.key, vector.lane, vector.first, vector.count)])) }) },
    births: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).births.map((vector) => [vector.id, lifetimes(vector, vector.first, vector.last, vector.births)])) }) },
    motions: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).motions.map((vector) => [vector.id, vector.ticks.map((tick) => particlesOf(vector.emitter, vector.origin, vector.facing, vector.since, vector.until, tick, vector.key))])) }) },
    caps: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).caps.map((vector) => [vector.id, kept(vector.ages, vector.cap)])) }) },
    ends: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).ends.map((vector) => [vector.id, emitterEnds(vector.emitter, vector.since, vector.until)])) }) },
  },
});
