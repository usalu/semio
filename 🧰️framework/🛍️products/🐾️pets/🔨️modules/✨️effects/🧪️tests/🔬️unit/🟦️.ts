/** ✨️ Unit suite of the pets effects: the published words of the hash and its statistics, births and lives against a count over every index, the five motions against numpy's committed answers and against their own laws, the caps, the end of an emitter, and the ban on platform transcendentals.
 *
 * @see ../../🟦️.ts — the module under test
 * @see ../../../../🧫️fixtures/✨️particle-motion/🔣️.json — numpy's answers (case ✨️particle-motion)
 * @see https://nullprogram.com/blog/2018/07/31/ — `lowbias32`
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { DRIFTS, type Drift } from "../../../../🧬️schema/🟦️.ts";
import { randomUnit, unitOf } from "../../../🎲️randomness/🟦️.ts";
import {
  BURST_DRAG,
  BURST_GRAVITY,
  DRIFT_MEANDER,
  EMITTER_CAP,
  type Emission,
  FALL_SWAY,
  ORBIT_DEPTH,
  ORBIT_FADE,
  ORBIT_SQUASH,
  type Particle,
  STAGE_CAP,
  TURN_RADIANS,
  UP,
  bornAt,
  capped,
  emitterEnds,
  emitterKey,
  lifeTicks,
  lowbias32,
  mix,
  particlesOf,
  periodOf,
  scattered,
  swarmOf,
  unit,
} from "../../🟦️.ts";

type Run = { readonly id: string; readonly emitter: Emission; readonly since: number; readonly until: number | null; readonly key: number };
type Vectors = {
  readonly hashes: readonly { readonly id: string; readonly low?: number; readonly chain?: readonly number[]; readonly expected: number }[];
  readonly uniformity: readonly { readonly id: string; readonly key: number; readonly lane: number; readonly first: number; readonly count: number; readonly expected: { readonly total: number; readonly bins: readonly number[] } }[];
  readonly births: readonly (Run & { readonly first: number; readonly last: number; readonly births: number; readonly expected: { readonly life: number; readonly swarm: number; readonly period: number; readonly born: readonly number[]; readonly ages: readonly (readonly number[])[] } })[];
  readonly motions: readonly (Run & { readonly origin: { readonly x: number; readonly y: number }; readonly facing: 1 | -1; readonly ticks: readonly number[]; readonly expected: readonly (readonly Particle[])[] })[];
  readonly caps: readonly { readonly id: string; readonly ages: readonly number[]; readonly cap: number; readonly expected: readonly number[] }[];
  readonly ends: readonly { readonly id: string; readonly emitter: Emission; readonly since: number; readonly until: number | null; readonly keys: readonly number[]; readonly expected: number | null }[];
};

const VECTORS = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/✨️particle-motion/🔣️.json", import.meta.url), "utf8")) as Vectors;
const SOURCE = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
const SEED = 20261003;
const ORIGIN = { x: 412.5, y: 233.25 };
const CONTINUOUS = ["fall", "rise", "drift"] as const;

/** 🎯️ How many draws the statistics of the hash are taken over at the level of the run. */
const DRAWS = sampled(4000, 40000, 400000);

/** 📏️ What a statistical bound stated for 40 000 draws is multiplied by at the level of the run: the deviation of a mean goes with 1 ÷ √draws. */
const SLACK = Math.sqrt(40000 / DRAWS);

/** 🎰️ How many drawn emitters the laws of births and motions are checked on at the level of the run. */
const EMITTERS = sampled(12, 60, 300);

/** ⏱️ How many ticks of every drawn emitter are looked at. */
const TICKS = sampled(160, 400, 800);

/** 🧮️ The number of set bits of an unsigned 32-bit word. */
function bits(word: number): number {
  let count = 0;
  for (let rest = word >>> 0; rest !== 0; rest >>>= 1) count += rest & 1;
  return count;
}

/** 🎲️ A drawn emitter of the given motion: counts from 1 to 32, lives from a tick to two seconds, speeds up to 200, any spread. */
function drawn(motion: Drift, counter: number): Emission {
  const draw = (lane: number) => randomUnit([SEED, lane, counter]);
  return { motion, count: 1 + Math.floor(draw(1) * 32), life: Math.floor(draw(2) * 128 + 1) / 64, speed: Math.floor(draw(3) * 200), spread: Math.floor(draw(4) * 9) / 8 };
}

/** 🧾️ The ages a continuous emitter must show at `tick`, eldest first, by looking at the birth of every particle from the first on (`births`, in the order of the indices): born, not yet dead, born before `until`, and the youngest `count` of those. */
function counted(emitter: Emission, births: readonly number[], until: number | null, tick: number): number[] {
  const life = lifeTicks(emitter);
  const ages: number[] = [];
  for (const born of births) if (born <= tick && tick - born < life && (until === null || born < until)) ages.push(tick - born);
  return ages.slice(Math.max(0, ages.length - swarmOf(emitter)));
}

/** 🟰️ Whether two lists of whole numbers are the same. */
function same(left: readonly number[], right: readonly number[]): boolean {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

/** 🤏️ Holds two lists of particles to each other: ages exactly, everything else within `tolerance`. */
function alike(produced: readonly Particle[], expected: readonly Particle[], label: string, tolerance: number): void {
  const wrong: string[] = [];
  if (produced.length !== expected.length) wrong.push(`${produced.length} particles instead of ${expected.length}`);
  for (let index = 0; index < Math.min(produced.length, expected.length); index++) {
    if (produced[index]!.age !== expected[index]!.age) wrong.push(`[${index}].age ${produced[index]!.age} instead of ${expected[index]!.age}`);
    for (const field of ["x", "y", "scale", "rotation", "opacity"] as const) if (!(Math.abs(produced[index]![field] - expected[index]![field]) <= tolerance)) wrong.push(`[${index}].${field} ${produced[index]![field]} instead of ${expected[index]![field]}`);
  }
  expect(wrong, label).toEqual([]);
}

describe("the hash", () => {
  it("reproduces the published words", () => {
    expect([lowbias32(1), lowbias32(0xdeadbeef), mix(0, 0), mix(1, 2), mix(mix(7, 3), 5)]).toEqual([0x688990c0, 0xe628c683, 0xe577f3aa, 0xb111e030, 0x5ab36a78]);
  });

  it("yields numpy's words for every committed word and chain", () => {
    expect(VECTORS.hashes.length).toBeGreaterThan(12);
    for (const vector of VECTORS.hashes) expect(vector.low !== undefined ? lowbias32(vector.low) : vector.chain!.slice(1).reduce((word, link) => mix(word, link), vector.chain![0]! >>> 0), vector.id).toBe(vector.expected);
  });

  it("leaves 0 at 0 in lowbias32 and has a single pair that mixes to 0", () => {
    expect(lowbias32(0)).toBe(0);
    expect(mix(0, 0)).not.toBe(0);
    expect(mix(0x9e3779b9, 0xffffffff)).toBe(0);
  });

  it("yields unsigned 32-bit words and reads its arguments modulo 2³²", () => {
    for (let counter = 0; counter < 500; counter++) {
      const word = mix(Math.floor(randomUnit([SEED, 0, counter]) * 2 ** 32), counter);
      expect(Number.isInteger(word) && word >= 0 && word <= 0xffffffff).toBe(true);
    }
    expect(mix(-1, 0)).toBe(mix(0xffffffff, 0));
    expect(mix(2 ** 32 + 5, 2 ** 32 + 7)).toBe(mix(5, 7));
    expect(lowbias32(2 ** 32 + 1)).toBe(lowbias32(1));
  });

  it("changes about half of the output bits when one input bit flips", () => {
    let flipped = 0;
    let trials = 0;
    for (let counter = 0; counter < 40; counter++) {
      const first = Math.floor(randomUnit([SEED, 1, counter]) * 2 ** 32);
      const second = Math.floor(randomUnit([SEED, 2, counter]) * 2 ** 32);
      for (let bit = 0; bit < 32; bit++) {
        flipped += bits(mix(first, second) ^ mix((first ^ (1 << bit)) >>> 0, second)) + bits(mix(first, second) ^ mix(first, (second ^ (1 << bit)) >>> 0));
        trials += 64;
      }
    }
    expect(flipped / trials).toBeGreaterThan(0.47);
    expect(flipped / trials).toBeLessThan(0.53);
  });

  it("turns a word into a unit with the randomness module's own division", () => {
    expect(unit).toBe(unitOf);
    expect([unit(0), unit(2147483648), unit(4294967295)]).toEqual([0, 0.5, 1 - 2 ** -32]);
    expect(scattered(7, 3, 5)).toBe(0x5ab36a78 / 4294967296);
  });

  it("gives every lane a uniform spread over the indices", () => {
    for (let lane = 0; lane < 5; lane++) {
      let sum = 0;
      let squares = 0;
      let outside = 0;
      const tenths = new Array<number>(10).fill(0);
      for (let index = 0; index < DRAWS; index++) {
        const value = scattered(SEED, index, lane);
        outside += value >= 0 && value < 1 ? 0 : 1;
        sum += value;
        squares += value * value;
        tenths[Math.floor(value * 10)]! += 1;
      }
      expect(outside, `lane ${lane}`).toBe(0);
      expect(Math.abs(sum / DRAWS - 0.5), `lane ${lane}`).toBeLessThan(0.008 * SLACK);
      expect(Math.abs(squares / DRAWS - (sum / DRAWS) ** 2 - 1 / 12), `lane ${lane}`).toBeLessThan(0.004 * SLACK);
      for (const count of tenths) expect(Math.abs(count / DRAWS - 0.1), `lane ${lane}`).toBeLessThan(0.008 * SLACK);
    }
  });

  it("sums and bins every committed run as numpy does", () => {
    for (const vector of VECTORS.uniformity) {
      if (vector.count > sampled(70000, 300000, 300000)) continue;
      const bins = new Array<number>(16).fill(0);
      let total = 0;
      for (let index = vector.first; index < vector.first + vector.count; index++) {
        const word = mix(mix(vector.key, index), vector.lane);
        total += word;
        bins[Math.floor(unit(word) * 16)]! += 1;
      }
      expect({ total, bins }, vector.id).toEqual(vector.expected);
    }
  });

  it("keeps the lanes of one particle and the particles of one lane unrelated", () => {
    let across = 0;
    let along = 0;
    for (let index = 0; index < DRAWS; index++) {
      across += (scattered(SEED, index, 1) - 0.5) * (scattered(SEED, index, 2) - 0.5);
      along += (scattered(SEED, index, 1) - 0.5) * (scattered(SEED, index + 1, 1) - 0.5);
    }
    expect(Math.abs(across / DRAWS) * 12).toBeLessThan(0.03 * SLACK);
    expect(Math.abs(along / DRAWS) * 12).toBeLessThan(0.03 * SLACK);
  });

  it("keys every run of every emitter apart", () => {
    const keys = new Set<number>();
    for (let species = 0; species < 20; species++) for (let emitter = 0; emitter < 4; emitter++) for (let since = 0; since < 50; since++) keys.add(emitterKey(SEED, species, emitter, since * 64));
    expect(keys.size).toBe(4000);
    expect(emitterKey(SEED, 3, 0, 128)).toBe(mix(mix(mix(SEED, 3), 0), 128));
    expect(emitterKey(20261002, 3, 0, 128)).toBe(VECTORS.hashes.find((vector) => vector.id === "mix-emitter-key")!.expected);
  });
});

describe("births", () => {
  it("counts a life in ticks, a swarm in whole particles and a period that fits the swarm into a life", () => {
    const of = (count: number, life: number): Emission => ({ motion: "fall", count, life, speed: 0, spread: 0 });
    expect([lifeTicks(of(1, 1)), lifeTicks(of(1, 1.6)), lifeTicks(of(1, 0.3)), lifeTicks(of(1, 0.008)), lifeTicks(of(1, 0.001)), lifeTicks(of(1, 0.4))]).toEqual([64, 102, 19, 1, 1, 26]);
    expect([swarmOf(of(1, 1)), swarmOf(of(32, 1)), swarmOf(of(40, 1)), swarmOf(of(0, 1)), swarmOf(of(-3, 1)), swarmOf(of(2.9, 1))]).toEqual([1, 32, EMITTER_CAP, 0, 0, 2]);
    expect([periodOf(of(6, 1)), periodOf(of(32, 1)), periodOf(of(64, 1)), periodOf(of(1, 1)), periodOf(of(32, 0.25)), periodOf(of(5, 1)), periodOf(of(0, 1))]).toEqual([11, 2, 2, 64, 1, 13, 64]);
    expect(EMITTER_CAP).toBe(32);
  });

  it("puts every birth into its own slot, in the order of the indices", () => {
    for (let counter = 0; counter < EMITTERS; counter++) {
      const emitter = drawn("rise", counter);
      const period = periodOf(emitter);
      const wrong: number[] = [];
      let last = -1;
      for (let index = 0; index < 200; index++) {
        const born = bornAt(emitter, 100, counter, index);
        if (!(born >= 100 + index * period && born < 100 + (index + 1) * period && born > last)) wrong.push(index);
        last = born;
      }
      expect(wrong, `emitter ${counter}`).toEqual([]);
    }
  });

  it("shows exactly the particles a count over every index finds alive", () => {
    let seen = 0;
    let full = 0;
    for (let counter = 0; counter < EMITTERS; counter++) {
      for (const motion of CONTINUOUS) {
        const emitter = drawn(motion, counter);
        const since = Math.floor(randomUnit([SEED, 5, counter]) * 50);
        const until = counter % 4 === 0 ? null : since + Math.floor(randomUnit([SEED, 6, counter]) * TICKS);
        const births = Array.from({ length: Math.floor(TICKS / periodOf(emitter)) + 2 }, (_, index) => bornAt(emitter, since, counter, index));
        const wrong: number[] = [];
        for (let tick = since - 3; tick < since + TICKS; tick++) {
          const ages = particlesOf(emitter, ORIGIN, 1, since, until, tick, counter).map((particle) => particle.age);
          if (!same(ages, counted(emitter, births, until, tick))) wrong.push(tick);
          seen += ages.length;
          full += ages.length === swarmOf(emitter) ? 1 : 0;
        }
        expect(wrong, `${motion} ${counter}`).toEqual([]);
      }
    }
    expect(seen).toBeGreaterThan(EMITTERS * TICKS);
    expect(full).toBeGreaterThan(0);
  });

  it("never shows more than the count, nothing before the start, the eldest first, and no birth at or after the stop", () => {
    for (let counter = 0; counter < EMITTERS; counter++) {
      for (const motion of DRIFTS) {
        const emitter = drawn(motion, counter);
        const until = 40 + 3 * counter;
        expect(particlesOf(emitter, ORIGIN, 1, 20, until, 19, counter)).toEqual([]);
        const wrong: string[] = [];
        for (let tick = 20; tick < 20 + TICKS; tick += 3) {
          const particles = particlesOf(emitter, ORIGIN, 1, 20, until, tick, counter);
          if (particles.length > emitter.count) wrong.push(`${particles.length} particles at ${tick}`);
          for (let index = 1; index < particles.length; index++) if (particles[index]!.age > particles[index - 1]!.age) wrong.push(`a younger particle before an elder at ${tick}`);
          if (motion === "fall" || motion === "rise" || motion === "drift") for (const particle of particles) if (tick - particle.age >= until || tick - particle.age < 20) wrong.push(`a birth at ${tick - particle.age}`);
        }
        expect(wrong, `${motion} ${counter}`).toEqual([]);
      }
    }
  });

  it("stores nothing: the same question has the same answer in any order of asking", () => {
    const emitter: Emission = { motion: "rise", count: 9, life: 1.2, speed: 40, spread: 0.3 };
    const forwards = Array.from({ length: 300 }, (_, tick) => particlesOf(emitter, ORIGIN, 1, 10, 200, tick, 77));
    for (let tick = 299; tick >= 0; tick -= 7) expect(particlesOf(emitter, ORIGIN, 1, 10, 200, tick, 77)).toEqual(forwards[tick]);
    expect(particlesOf(emitter, ORIGIN, 1, 10, 200, 150, 78)).not.toEqual(forwards[150]);
    expect(emitter).toEqual({ motion: "rise", count: 9, life: 1.2, speed: 40, spread: 0.3 });
  });

  it("follows numpy's tick-by-tick simulation for every committed emitter", () => {
    expect(VECTORS.births.length).toBeGreaterThan(15);
    for (const vector of VECTORS.births) {
      expect({ life: lifeTicks(vector.emitter), swarm: swarmOf(vector.emitter), period: periodOf(vector.emitter) }, vector.id).toEqual({ life: vector.expected.life, swarm: vector.expected.swarm, period: vector.expected.period });
      expect(Array.from({ length: vector.births }, (_, index) => bornAt(vector.emitter, vector.since, vector.key, index)), vector.id).toEqual(vector.expected.born);
      const wrong: number[] = [];
      for (let tick = vector.first; tick <= vector.last; tick++) if (!same(particlesOf(vector.emitter, { x: 0, y: 0 }, 1, vector.since, vector.until, tick, vector.key).map((particle) => particle.age), vector.expected.ages[tick - vector.first]!)) wrong.push(tick);
      expect(wrong, vector.id).toEqual([]);
    }
  });
});

describe("motions", () => {
  it("places every committed particle where numpy does", () => {
    expect(VECTORS.motions.length).toBeGreaterThan(20);
    for (const motion of DRIFTS) expect(VECTORS.motions.some((vector) => vector.emitter.motion === motion), motion).toBe(true);
    for (const vector of VECTORS.motions) vector.ticks.forEach((tick, index) => alike(particlesOf(vector.emitter, vector.origin, vector.facing, vector.since, vector.until, tick, vector.key), vector.expected[index]!, `${vector.id} at ${tick}`, 1e-9));
  });

  it("keeps every opacity in [0, 1], every scale positive and every number finite", () => {
    const wrong: string[] = [];
    let seen = 0;
    for (let counter = 0; counter < EMITTERS; counter++) {
      for (const motion of DRIFTS) {
        const emitter = drawn(motion, counter);
        for (let tick = 0; tick < TICKS; tick += 2) {
          for (const particle of particlesOf(emitter, ORIGIN, counter % 2 === 0 ? 1 : -1, 0, 100, tick, counter)) {
            seen += 1;
            if (!(particle.opacity >= 0 && particle.opacity <= 1)) wrong.push(`${motion} ${counter} at ${tick}: opacity ${particle.opacity}`);
            if (!(particle.scale > 0 && particle.scale <= 1.2)) wrong.push(`${motion} ${counter} at ${tick}: scale ${particle.scale}`);
            if (!(Number.isFinite(particle.x) && Number.isFinite(particle.y) && Number.isFinite(particle.rotation))) wrong.push(`${motion} ${counter} at ${tick}: not finite`);
          }
        }
      }
    }
    expect(wrong).toEqual([]);
    expect(seen).toBeGreaterThan(EMITTERS * 100);
  });

  it("mirrors a pet that faces left about the emitter's origin and moves with the origin", () => {
    for (let counter = 0; counter < EMITTERS; counter++) {
      for (const motion of DRIFTS) {
        const emitter = drawn(motion, counter);
        const right = particlesOf(emitter, ORIGIN, 1, 0, 90, 40 + counter, counter);
        const left = particlesOf(emitter, ORIGIN, -1, 0, 90, 40 + counter, counter);
        const moved = particlesOf(emitter, { x: ORIGIN.x + 100, y: ORIGIN.y - 50 }, 1, 0, 90, 40 + counter, counter);
        expect(left.length).toBe(right.length);
        for (let index = 0; index < right.length; index++) {
          expect(left[index]!.x - ORIGIN.x).toBeCloseTo(ORIGIN.x - right[index]!.x, 9);
          expect(left[index]!.y).toBeCloseTo(right[index]!.y, 9);
          expect(left[index]!.scale).toBeCloseTo(right[index]!.scale, 9);
          expect([left[index]!.opacity, left[index]!.age]).toEqual([right[index]!.opacity, right[index]!.age]);
          expect(Math.cos(left[index]!.rotation * TURN_RADIANS)).toBeCloseTo(motion === "burst" || motion === "drift" ? -Math.cos(right[index]!.rotation * TURN_RADIANS) : Math.cos(right[index]!.rotation * TURN_RADIANS), 9);
          expect(moved[index]!.x - right[index]!.x).toBeCloseTo(100, 9);
          expect(moved[index]!.y - right[index]!.y).toBeCloseTo(-50, 9);
        }
      }
    }
  });

  it("lets a particle fall straight down at its own steady speed when nothing scatters it", () => {
    const emitter: Emission = { motion: "fall", count: 6, life: 1, speed: 90, spread: 0 };
    const earlier = particlesOf(emitter, ORIGIN, 1, 0, null, 500, 5);
    const later = particlesOf(emitter, ORIGIN, 1, 0, null, 516, 5);
    expect(earlier.length).toBeGreaterThan(3);
    for (const particle of earlier) {
      const same = later.find((other) => other.age === particle.age + 16);
      expect(Math.abs(particle.x - ORIGIN.x)).toBeLessThanOrEqual(FALL_SWAY);
      expect(Math.abs(particle.rotation)).toBe(0);
      expect(particle.y).toBeGreaterThanOrEqual(ORIGIN.y);
      if (same === undefined) continue;
      expect((same.y - particle.y) * 4).toBeGreaterThanOrEqual(90 * 0.9 - 1e-9);
      expect((same.y - particle.y) * 4).toBeLessThanOrEqual(90 * 1.1 + 1e-9);
    }
  });

  it("lets a particle rise straight up, popping in and fading out", () => {
    const emitter: Emission = { motion: "rise", count: 4, life: 1, speed: 50, spread: 0 };
    for (let tick = 300; tick < 364; tick++) {
      for (const particle of particlesOf(emitter, ORIGIN, 1, 0, null, tick, 9)) {
        expect(particle.x).toBe(ORIGIN.x);
        expect(Math.abs(particle.rotation)).toBe(0);
        expect(particle.y).toBeLessThanOrEqual(ORIGIN.y);
        expect(ORIGIN.y - particle.y).toBeLessThanOrEqual((50 * particle.age) / 64 + 1e-9);
        expect(ORIGIN.y - particle.y).toBeGreaterThanOrEqual((0.7 * 50 * particle.age) / 64 - 1e-9);
        expect(particle.scale).toBe(particle.age >= 8 ? 1 : 0.5 + 0.5 * (particle.age / 8) ** 2 * (3 - (2 * particle.age) / 8));
        if (particle.age === 0) expect(particle.opacity).toBe(0);
        if (particle.age >= 6 && particle.age <= 38) expect(particle.opacity).toBe(1);
      }
    }
  });

  it("lets a particle drift straight ahead, meandering a little across its path", () => {
    const emitter: Emission = { motion: "drift", count: 4, life: 1, speed: 40, spread: 0 };
    for (let tick = 300; tick < 364; tick += 5) {
      for (const particle of particlesOf(emitter, ORIGIN, 1, 0, null, tick, 9)) {
        expect(Math.abs(particle.rotation)).toBe(0);
        expect(Math.abs(particle.y - ORIGIN.y)).toBeLessThanOrEqual(DRIFT_MEANDER);
        expect(particle.x - ORIGIN.x).toBeGreaterThanOrEqual((0.5 * 40 * particle.age) / 64 - 1e-9);
        expect(particle.x - ORIGIN.x).toBeLessThanOrEqual((40 * particle.age) / 64 + 1e-9);
      }
    }
  });

  it("throws a burst from the origin, every particle into its own slice of the fan, braked like an exponential drag", () => {
    const emitter: Emission = { motion: "burst", count: 12, life: 1.6, speed: 160, spread: 0.5 };
    const start = particlesOf(emitter, ORIGIN, 1, 30, null, 30, 21);
    expect(start.length).toBe(12);
    for (const [index, particle] of start.entries()) {
      expect([particle.x, particle.y, particle.scale, particle.opacity, particle.age]).toEqual([ORIGIN.x, ORIGIN.y, 1, 1, 0]);
      expect(particle.rotation).toBeGreaterThanOrEqual(UP + (index / 12 - 0.5) * 0.5);
      expect(particle.rotation).toBeLessThan(UP + ((index + 1) / 12 - 0.5) * 0.5);
    }
    for (let age = 1; age < 102; age += 4) {
      const seconds = age / 64;
      const flown = BURST_DRAG * (1 - Math.exp(-seconds / BURST_DRAG));
      for (const [index, particle] of particlesOf(emitter, ORIGIN, 1, 30, null, 30 + age, 21).entries()) {
        const pace = 160 * (0.45 + 0.55 * scattered(21, index, 2));
        const heading = start[index]!.rotation * TURN_RADIANS;
        expect(Math.abs(particle.x - (ORIGIN.x + pace * Math.cos(heading) * flown))).toBeLessThanOrEqual(0.02 * 160 * BURST_DRAG);
        expect(Math.abs(particle.y - (ORIGIN.y + pace * Math.sin(heading) * flown + 0.5 * BURST_GRAVITY * seconds * seconds))).toBeLessThanOrEqual(0.02 * 160 * BURST_DRAG);
        expect(particle.opacity).toBeCloseTo(1 - (age / 102) ** 2, 12);
        expect(particle.scale).toBeCloseTo(1 - (0.6 * age) / 102, 12);
      }
    }
    expect(particlesOf(emitter, ORIGIN, 1, 30, null, 131, 21).length).toBe(12);
    expect(particlesOf(emitter, ORIGIN, 1, 30, null, 132, 21)).toEqual([]);
    expect(particlesOf(emitter, ORIGIN, 1, 30, 10, 60, 21).length).toBe(12);
  });

  it("keeps an orbit on its squashed ring, one lap per life, evenly spaced, fading in and out", () => {
    const emitter: Emission = { motion: "orbit", count: 5, life: 1, speed: 94, spread: 1 };
    const radius = 94 / TURN_RADIANS;
    for (let tick = 108; tick < 300; tick += 7) {
      const ring = particlesOf(emitter, ORIGIN, 1, 100, null, tick, 3);
      const lap = particlesOf(emitter, ORIGIN, 1, 100, null, tick + 64, 3);
      expect(ring.length).toBe(5);
      for (const [index, particle] of ring.entries()) {
        const across = (particle.x - ORIGIN.x) / radius;
        const depth = (particle.y - ORIGIN.y) / (radius * ORBIT_SQUASH);
        expect(across * across + depth * depth).toBeCloseTo(1, 9);
        expect(particle.scale).toBeCloseTo(1 + ORBIT_DEPTH * depth, 9);
        expect([particle.rotation, particle.opacity, particle.age]).toEqual([0, 1, tick - 100]);
        expect(lap[index]!.x).toBeCloseTo(particle.x, 9);
        expect(lap[index]!.y).toBeCloseTo(particle.y, 9);
        const angle = ((index / 5 + (tick - 100) / 64) % 1) * TURN_RADIANS;
        expect(across).toBeCloseTo(Math.cos(angle), 9);
        expect(depth).toBeCloseTo(Math.sin(angle), 9);
      }
    }
    expect(particlesOf(emitter, ORIGIN, 1, 100, null, 100, 3).map((particle) => particle.opacity)).toEqual([0, 0, 0, 0, 0]);
    expect(particlesOf(emitter, ORIGIN, 1, 100, 400, 400, 3).map((particle) => particle.opacity)).toEqual([1, 1, 1, 1, 1]);
    expect(particlesOf(emitter, ORIGIN, 1, 100, 400, 404, 3)[0]!.opacity).toBe(0.5);
    expect(particlesOf(emitter, ORIGIN, 1, 100, 400, 400 + ORBIT_FADE - 1, 3).length).toBe(5);
    expect(particlesOf(emitter, ORIGIN, 1, 100, 400, 400 + ORBIT_FADE, 3)).toEqual([]);
  });
});

describe("caps", () => {
  /** 🧱️ Particles known by their place in the list (`x`) and their age. */
  const aged = (ages: readonly number[]): Particle[] => ages.map((age, position) => ({ x: position, y: 0, scale: 1, rotation: 0, opacity: 1, age }));

  it("keeps the survivors numpy's stable sort keeps for every committed cap", () => {
    expect(VECTORS.caps.length).toBeGreaterThan(10);
    for (const vector of VECTORS.caps) expect(capped(aged(vector.ages), vector.cap).map((particle) => particle.x), vector.id).toEqual(vector.expected);
  });

  it("returns the very list when nothing has to go", () => {
    const particles = aged([3, 1, 2]);
    expect(capped(particles, 3)).toBe(particles);
    expect(capped(particles, STAGE_CAP)).toBe(particles);
    expect(capped(particles, 2)).not.toBe(particles);
    expect(particles.map((particle) => particle.age)).toEqual([3, 1, 2]);
  });

  it("keeps the youngest in their order, the earlier of two of one age, for any drawn crowd", () => {
    for (let counter = 0; counter < sampled(40, 400, 4000); counter++) {
      const size = Math.floor(randomUnit([SEED, 7, counter]) * 60);
      const cap = Math.floor(randomUnit([SEED, 8, counter]) * 40);
      const particles = aged(Array.from({ length: size }, (_, index) => Math.floor(randomUnit([SEED, 9, counter * 64 + index]) * 12)));
      const order = particles.map((particle) => particle.x).sort((left, right) => particles[left]!.age - particles[right]!.age || left - right);
      expect(capped(particles, cap).map((particle) => particle.x)).toEqual(order.slice(0, cap).sort((left, right) => left - right));
    }
  });

  it("holds a whole stage of full emitters to 160 particles, the oldest emitters giving way", () => {
    expect(STAGE_CAP).toBe(160);
    const emitter: Emission = { motion: "orbit", count: 32, life: 1, speed: 60, spread: 1 };
    const stage = Array.from({ length: 8 }, (_, index) => particlesOf(emitter, ORIGIN, 1, index * 10, null, 200, index)).flat();
    expect(stage.length).toBe(256);
    const shown = capped(stage, STAGE_CAP);
    expect(shown.length).toBe(160);
    expect(shown).toEqual(stage.slice(96));
  });
});

describe("emitterEnds", () => {
  it("names numpy's tick for every committed emitter", () => {
    expect(VECTORS.ends.length).toBeGreaterThan(12);
    for (const vector of VECTORS.ends) expect(emitterEnds(vector.emitter, vector.since, vector.until), vector.id).toBe(vector.expected);
  });

  it("is the first tick from which nothing is alive, and no end while the emitter runs", () => {
    for (let counter = 0; counter < EMITTERS; counter++) {
      for (const motion of DRIFTS) {
        const emitter = drawn(motion, counter);
        const since = 10 + counter;
        const until = since - 5 + Math.floor(randomUnit([SEED, 10, counter]) * 200);
        const end = emitterEnds(emitter, since, until)!;
        expect(end).toBeGreaterThanOrEqual(since);
        expect(end).toBeLessThanOrEqual(Math.max(since, until) + Math.max(lifeTicks(emitter), ORBIT_FADE));
        const alive: number[] = [];
        for (let tick = end; tick < end + 2 * lifeTicks(emitter) + 3; tick++) if (particlesOf(emitter, ORIGIN, 1, since, until, tick, counter).length > 0) alive.push(tick);
        expect(alive, `${motion} ${counter}`).toEqual([]);
        if ((motion === "burst" || motion === "orbit") && end > since) expect(particlesOf(emitter, ORIGIN, 1, since, until, end - 1, counter).length).toBe(emitter.count);
        expect(emitterEnds(emitter, since, null)).toBe(motion === "burst" ? since + lifeTicks(emitter) : null);
      }
    }
  });

  it("ends a continuous emitter within a period and a tick of its last particle", () => {
    for (let counter = 0; counter < EMITTERS; counter++) {
      const emitter = drawn("fall", counter);
      const end = emitterEnds(emitter, 0, 300)!;
      let last = -1;
      for (let tick = 0; tick < end; tick++) if (particlesOf(emitter, ORIGIN, 1, 0, 300, tick, counter).length > 0) last = tick;
      expect(end - 1 - last).toBeLessThanOrEqual(2 * periodOf(emitter));
    }
  });
});

describe("determinism", () => {
  it("the module calls no platform transcendental, random source or clock", () => {
    const banned = ["sin", "cos", "tan", "atan2", "exp", "pow", "hypot", "log", "random"].map((name) => `Math.${name}(`).concat(["Date", "performance", "console"]);
    for (const call of banned) expect(SOURCE.includes(call), call).toBe(false);
  });
});
