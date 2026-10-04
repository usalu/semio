/** 🎲️ Unit suite of the pets counter-based randomness: numpy's committed seed-sequence words, the laws of a pure keyed draw, the statistics of units and weighted picks, and the ban on platform randomness.
 *
 * @see ../../🟦️.ts — the module under test
 * @see ../../../../🧫️fixtures/🎲️counter-randomness/🔣️.json — numpy's answers (case 🎲️counter-randomness)
 * @see https://numpy.org/doc/stable/reference/random/bit_generators/generated/numpy.random.SeedSequence.html — the reference
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { CAST_STREAM, CHEMISTRY_STREAM, GEAR_STREAM, MISCHIEF_STREAM, ROTATION_STREAM, STAGE_STREAM, randomBetween, randomPick, randomUnit, randomWords, unitOf, weightedIndex } from "../../🟦️.ts";

type Vectors = {
  readonly words: readonly { readonly id: string; readonly key: readonly number[]; readonly count: number; readonly expected: readonly number[] }[];
  readonly units: readonly { readonly id: string; readonly key: readonly number[]; readonly expected: number }[];
  readonly streams: readonly { readonly id: string; readonly seed: number; readonly stream: number; readonly count: number; readonly expected: readonly number[] }[];
  readonly ranges: readonly { readonly id: string; readonly key: readonly number[]; readonly low: number; readonly high: number; readonly expected: number }[];
  readonly picks: readonly { readonly id: string; readonly seed: number; readonly stream: number; readonly count: number; readonly weights: readonly number[]; readonly expected: readonly number[] }[];
};

const VECTORS = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/🎲️counter-randomness/🔣️.json", import.meta.url), "utf8")) as Vectors;
const SOURCE = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
const SEED = 20261002;

/** 🎯️ How many draws the statistics are taken over at the level of the run. */
const DRAWS = sampled(2000, 20000, 100000);

/** 📏️ What a statistical bound stated for 20 000 draws is multiplied by at the level of the run: the deviation of a mean goes with 1 ÷ √draws. */
const SLACK = Math.sqrt(20000 / DRAWS);

/** 🔁️ How many counters of a stream the laws of a single draw are checked on at the level of the run. */
const COUNTERS = sampled(200, 2000, 20000);

/** 🧮️ The number of set bits of an unsigned 32-bit word. */
function bits(word: number): number {
  let count = 0;
  for (let rest = word >>> 0; rest !== 0; rest >>>= 1) count += rest & 1;
  return count;
}

describe("randomWords", () => {
  it("yields the words numpy's seed sequence generates for every committed key", () => {
    expect(VECTORS.words.length).toBeGreaterThan(15);
    for (const vector of VECTORS.words) expect(randomWords(vector.key, vector.count), vector.id).toEqual(vector.expected);
  });

  it("starts the stream of the key [0] with numpy's documented words", () => {
    expect(randomWords([0], 4)).toEqual([2968811710, 3677149159, 745650761, 2884920346]);
  });

  it("yields unsigned 32-bit integers, and as many as asked", () => {
    expect(randomWords([SEED, 1, 2], 0)).toEqual([]);
    const words = randomWords([SEED, 1, 2], 257);
    expect(words.length).toBe(257);
    for (const word of words) expect(Number.isInteger(word) && word >= 0 && word <= 0xffffffff).toBe(true);
  });

  it("is a pure function of the key, and a longer read extends a shorter one", () => {
    const key = [SEED, 3, 99];
    expect(randomWords(key, 12)).toEqual(randomWords([...key], 12));
    expect(randomWords(key, 12).slice(0, 5)).toEqual(randomWords(key, 5));
    expect(key).toEqual([SEED, 3, 99]);
  });

  it("counts missing words as zeros up to the pool of four, and no further", () => {
    expect(randomWords([5], 8)).toEqual(randomWords([5, 0], 8));
    expect(randomWords([5], 8)).toEqual(randomWords([5, 0, 0, 0], 8));
    expect(randomWords([5], 8)).not.toEqual(randomWords([5, 0, 0, 0, 0], 8));
    expect(randomWords([], 8)).toEqual(randomWords([0, 0, 0, 0], 8));
  });

  it("changes about half of the output bits when one key bit flips", () => {
    let flipped = 0;
    let trials = 0;
    for (let position = 0; position < 3; position++) {
      for (let bit = 0; bit < 32; bit++) {
        const key = [SEED, 7, 41];
        const before = randomWords(key, 4);
        key[position] = (key[position]! ^ (1 << bit)) >>> 0;
        const after = randomWords(key, 4);
        for (let index = 0; index < 4; index++) flipped += bits(before[index]! ^ after[index]!);
        trials += 4 * 32;
      }
    }
    expect(flipped / trials).toBeGreaterThan(0.47);
    expect(flipped / trials).toBeLessThan(0.53);
  });

  it("gives neighbouring counters and neighbouring streams unrelated words", () => {
    const seen = new Set<number>();
    for (let stream = 0; stream < 20; stream++) for (let counter = 0; counter < 200; counter++) seen.add(randomWords([SEED, stream, counter], 1)[0]!);
    expect(seen.size).toBe(4000);
  });
});

describe("randomUnit", () => {
  it("is the first word divided by 2³² for every committed key and stream", () => {
    for (const vector of VECTORS.units) {
      expect(randomUnit(vector.key), vector.id).toBe(vector.expected);
      expect(randomUnit(vector.key), vector.id).toBe(randomWords(vector.key, 1)[0]! / 4294967296);
    }
    for (const vector of VECTORS.streams) expect(Array.from({ length: vector.count }, (_, counter) => randomUnit([vector.seed, vector.stream, counter])), vector.id).toEqual(vector.expected);
  });

  it("lies in [0, 1) and is uniform over a stream", () => {
    let sum = 0;
    const tenths = new Array<number>(10).fill(0);
    for (let counter = 0; counter < DRAWS; counter++) {
      const unit = randomUnit([SEED, 0, counter]);
      expect(unit >= 0 && unit < 1).toBe(true);
      sum += unit;
      tenths[Math.floor(unit * 10)]! += 1;
    }
    expect(Math.abs(sum / DRAWS - 0.5)).toBeLessThan(0.01 * SLACK);
    for (const count of tenths) expect(Math.abs(count / DRAWS - 0.1)).toBeLessThan(0.01 * SLACK);
  });
});

describe("randomBetween", () => {
  it("scales the unit draw between the bounds for every committed key", () => {
    for (const vector of VECTORS.ranges) {
      expect(randomBetween(vector.key, vector.low, vector.high), vector.id).toBe(vector.expected);
      expect(randomBetween(vector.key, vector.low, vector.high), vector.id).toBe(vector.low + (vector.high - vector.low) * randomUnit(vector.key));
    }
  });

  it("stays between the bounds in either order and returns the bound of an empty range", () => {
    for (let counter = 0; counter < COUNTERS; counter++) {
      const ascending = randomBetween([SEED, 1, counter], 2, 6);
      const descending = randomBetween([SEED, 1, counter], 6, 2);
      expect(ascending >= 2 && ascending < 6).toBe(true);
      expect(descending > 2 && descending <= 6).toBe(true);
      expect(ascending + descending).toBe(8);
      expect(randomBetween([SEED, 1, counter], 5, 5)).toBe(5);
    }
  });
});

describe("randomPick", () => {
  it("follows numpy's cumulative weights for every committed stream", () => {
    for (const vector of VECTORS.picks) expect(Array.from({ length: vector.count }, (_, counter) => randomPick([vector.seed, vector.stream, counter], vector.weights)), vector.id).toEqual(vector.expected);
  });

  it("answers −1 when no weight is positive", () => {
    expect(randomPick([SEED], [])).toBe(-1);
    expect(randomPick([SEED], [0, 0, 0])).toBe(-1);
    expect(randomPick([SEED], [-1, -2])).toBe(-1);
    expect(randomPick([SEED], [0, Number.NaN, -0])).toBe(-1);
  });

  it("never picks a weight that is not positive", () => {
    for (let counter = 0; counter < COUNTERS; counter++) {
      expect([1, 3]).toContain(randomPick([SEED, 2, counter], [0, 2, -5, 1, 0]));
      expect(randomPick([SEED, 2, counter], [0, 0, 7])).toBe(2);
    }
  });

  it("picks in proportion to the weights", () => {
    const weights = [1, 2, 3, 4];
    const counts = [0, 0, 0, 0];
    for (let counter = 0; counter < DRAWS; counter++) counts[randomPick([SEED, 3, counter], weights)]! += 1;
    for (let index = 0; index < weights.length; index++) expect(Math.abs(counts[index]! / DRAWS - weights[index]! / 10)).toBeLessThan(0.015 * SLACK);
  });

  it("depends on the proportions of the weights only", () => {
    for (let counter = 0; counter < COUNTERS / 4; counter++) expect(randomPick([SEED, 4, counter], [1, 2, 4, 1])).toBe(randomPick([SEED, 4, counter], [8, 16, 32, 8]));
  });

  it("is the one weighted pick of the product: weightedIndex at the unit of the key", () => {
    const weights = [0.5, 0, 1.25, 3, 0, 0.125];
    for (let counter = 0; counter < COUNTERS / 4; counter++) {
      const key = [SEED, 5, counter];
      expect(randomPick(key, weights)).toBe(weightedIndex(weights, randomUnit(key)));
      expect(randomUnit(key)).toBe(unitOf(randomWords(key, 1)[0]!));
    }
    expect([weightedIndex([1, 1, 2], 0), weightedIndex([1, 1, 2], 0.25), weightedIndex([1, 1, 2], 0.5), weightedIndex([1, 1, 2], 1 - 2 ** -32)]).toEqual([0, 1, 2, 2]);
    expect([weightedIndex([], 0.5), weightedIndex([0, -1], 0.5)]).toEqual([-1, -1]);
  });
});

describe("the reserved streams", () => {
  it("turns a word into a unit by one exact division, the largest word staying below 1", () => {
    expect([unitOf(0), unitOf(2147483648), unitOf(4294967295)]).toEqual([0, 0.5, 1 - 2 ** -32]);
  });

  it("names six streams at the top of the 32-bit range, apart from each other and from every species", () => {
    expect([STAGE_STREAM, CAST_STREAM, ROTATION_STREAM, CHEMISTRY_STREAM, GEAR_STREAM, MISCHIEF_STREAM]).toEqual([0xffffffff, 0xfffffffe, 0xfffffffd, 0xfffffffc, 0xfffffffb, 0xfffffffa]);
    const words = [STAGE_STREAM, CAST_STREAM, ROTATION_STREAM, CHEMISTRY_STREAM, GEAR_STREAM, MISCHIEF_STREAM, 0, 1, 2].map((stream) => randomWords([SEED, stream, 0], 1)[0]!);
    expect(new Set(words).size).toBe(words.length);
  });
});

describe("determinism", () => {
  it("the module calls no platform transcendental, random source or clock", () => {
    const banned = ["sin", "cos", "tan", "atan2", "exp", "pow", "hypot", "log", "random"].map((name) => `Math.${name}(`).concat(["Date", "performance"]);
    for (const call of banned) expect(SOURCE.includes(call), call).toBe(false);
  });
});
