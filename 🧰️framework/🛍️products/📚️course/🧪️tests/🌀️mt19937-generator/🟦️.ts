import { describe, expect, it } from "vitest";
import { Mt19937, fnv1a32, runSeed, shuffle, uniformIndex, type RandomSource } from "../../📦️packages/🟦️typescript/🟦️.ts";

/** 🐍️ Raw MT19937 outputs of numpy 2.4.3 (`MT19937()._legacy_seeding(seed)` = `init_genrand`, then `random_raw`), recorded by the ticket script `mt19937-numpy-vectors.py`. */
const NUMPY_VECTORS: Readonly<Record<number, { readonly first: readonly number[]; readonly thousandth: number }>> = {
  0: { first: [2357136044, 2546248239, 3071714933, 3626093760, 2588848963], thousandth: 3043451800 },
  1: { first: [1791095845, 4282876139, 3093770124, 4005303368, 491263], thousandth: 548926898 },
  42: { first: [1608637542, 3421126067, 4083286876, 787846414, 3143890026], thousandth: 1946654618 },
  5489: { first: [3499211612, 581869302, 3890346734, 3586334585, 545404204], thousandth: 1341017984 },
  2166136261: { first: [3450883027, 1146584251, 2190115248, 106034287, 2175477703], thousandth: 3124343348 },
  4294967295: { first: [419326371, 479346978, 3918654476, 2416749639, 3388880820], thousandth: 2673539693 },
};

/** 📼️ A source replaying recorded outputs, independent of the generator under test. */
function replay(values: readonly number[]): RandomSource & { readonly consumed: () => number } {
  let index = 0;
  return {
    next: () => {
      if (index >= values.length) throw new Error("replay exhausted");
      return values[index++]!;
    },
    consumed: () => index,
  };
}

/** 🔢️ The first `count` outputs of a fresh generator. */
function outputs(seed: number, count: number): number[] {
  const random = new Mt19937(seed);
  return Array.from({ length: count }, () => random.next());
}

describe("fnv1a32", () => {
  it("matches the FNV reference vectors", () => {
    expect(fnv1a32("")).toBe(2166136261);
    expect(fnv1a32("a")).toBe(3826002220);
    expect(fnv1a32("foobar")).toBe(0xbf9cf968);
  });

  it("hashes the UTF-8 bytes, not UTF-16 code units", () => {
    const manual = [...new TextEncoder().encode("ä🎲")].reduce((hash, byte) => Number((BigInt((hash ^ byte) >>> 0) * 16777619n) % 4294967296n), 2166136261);
    expect(fnv1a32("ä🎲")).toBe(manual);
    expect(fnv1a32("ä")).not.toBe(fnv1a32("ä".normalize("NFD")));
  });

  it("seeds a run by its id and always yields an unsigned 32-bit integer", () => {
    const run = "0123456789abcdef0123456789abcdef";
    expect(runSeed(run)).toBe(fnv1a32(run));
    for (const text of ["", "a", run, "ffffffffffffffffffffffffffffffff", "Ωmega"]) {
      const seed = fnv1a32(text);
      expect(Number.isInteger(seed) && seed >= 0 && seed <= 4294967295).toBe(true);
    }
  });
});

describe("Mt19937", () => {
  it("yields 3499211612 first and 4123659995 as the 10000th output for seed 5489 (C++ [rand.predef])", () => {
    const values = outputs(5489, 10000);
    expect(values[0]).toBe(3499211612);
    expect(values[9999]).toBe(4123659995);
  });

  it("agrees with numpy's init_genrand generator on every recorded seed", () => {
    for (const [seed, vector] of Object.entries(NUMPY_VECTORS)) {
      const values = outputs(Number(seed), 1000);
      expect(values.slice(0, 5)).toEqual(vector.first);
      expect(values[999]).toBe(vector.thousandth);
    }
  });

  it("reduces the seed modulo 2³²", () => {
    expect(outputs(-1, 3)).toEqual(outputs(4294967295, 3));
    expect(outputs(4294967296, 3)).toEqual(outputs(0, 3));
  });

  it("is deterministic and yields unsigned 32-bit integers across twists", () => {
    const values = outputs(7, 2000);
    expect(values).toEqual(outputs(7, 2000));
    expect(values.every((value) => Number.isInteger(value) && value >= 0 && value <= 4294967295)).toBe(true);
  });
});

describe("uniformIndex", () => {
  it("returns 0 for n = 1 without drawing", () => {
    const random = replay([]);
    expect(uniformIndex(random, 1)).toBe(0);
    expect(random.consumed()).toBe(0);
  });

  it("reduces an accepted draw modulo n", () => {
    expect(uniformIndex(replay([3499211612]), 5)).toBe(2);
    expect(uniformIndex(replay([581869302]), 4)).toBe(2);
  });

  it("rejects draws at or above 2³² − (2³² mod n)", () => {
    const n = 3 * 2 ** 30;
    const random = replay([3 * 2 ** 30, 4294967295, 5]);
    expect(uniformIndex(random, n)).toBe(5);
    expect(random.consumed()).toBe(3);
    expect(uniformIndex(replay([4294967294]), 3)).toBe(4294967294 % 3);
    const rejected = replay([4294967295, 7]);
    expect(uniformIndex(rejected, 3)).toBe(1);
    expect(rejected.consumed()).toBe(2);
  });

  it("stays in range and is close to uniform", () => {
    const random = new Mt19937(2024);
    const counts = [0, 0, 0, 0, 0, 0];
    for (let i = 0; i < 60000; i++) counts[uniformIndex(random, 6)]!++;
    expect(counts.every((count) => count > 9500 && count < 10500)).toBe(true);
    for (let n = 2; n < 50; n++) expect(uniformIndex(random, n)).toBeLessThan(n);
  });
});

describe("shuffle", () => {
  it("runs Fisher–Yates from the end with one draw per position", () => {
    const random = replay(NUMPY_VECTORS[5489]!.first);
    expect(shuffle(random, ["a", "b", "c", "d", "e"])).toEqual(["a", "b", "d", "e", "c"]);
    expect(random.consumed()).toBe(4);
    expect(shuffle(new Mt19937(5489), ["a", "b", "c", "d", "e"])).toEqual(["a", "b", "d", "e", "c"]);
  });

  it("draws nothing for empty and single-item lists", () => {
    const random = replay([]);
    expect(shuffle(random, [])).toEqual([]);
    expect(shuffle(random, ["only"])).toEqual(["only"]);
    expect(random.consumed()).toBe(0);
  });

  it("is a deterministic permutation that leaves its input alone", () => {
    const items = Array.from({ length: 20 }, (_, index) => index);
    const first = shuffle(new Mt19937(42), items);
    expect(shuffle(new Mt19937(42), items)).toEqual(first);
    expect([...first].sort((left, right) => left - right)).toEqual(items);
    expect(first).not.toEqual(items);
    expect(items).toEqual(Array.from({ length: 20 }, (_, index) => index));
  });

  it("reaches every permutation of three items about equally often", () => {
    const counts = new Map<string, number>();
    for (let seed = 0; seed < 6000; seed++) {
      const key = shuffle(new Mt19937(seed), ["a", "b", "c"]).join("");
      counts.set(key, (counts.get(key) ?? 0) + 1);
    }
    expect(counts.size).toBe(6);
    expect([...counts.values()].every((count) => count > 850 && count < 1150)).toBe(true);
  });
});
