/** 🥇️ Force-first support solution: a load the supports cannot balance leaves the smallest unbalanced force and, with that force, the smallest unbalanced moment.
 *
 * THE REFERENCE is numpy (singular value decompositions) in 🐍️.py.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🥇️force-first/🔣️.json — the seeded stiffness matrices and wrenches */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { solveForceFirst, solveSemiDefinite } from "../../🔨️modules/🏗️structure/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Row = { readonly d: readonly number[]; readonly k: number };
type Vectors = { readonly tolerance: number; readonly cases: { readonly name: string; readonly rank: number; readonly rows: readonly Row[]; readonly f: readonly number[] }[] };
const vectors = fixture<Vectors>("🥇️force-first");
const stiffness = (rows: readonly Row[]): number[][] => Array.from({ length: 6 }, (_, i) => Array.from({ length: 6 }, (__, j) => rows.reduce((sum, row) => sum + row.k * row.d[i]! * row.d[j]!, 0)));
const times = (k: readonly (readonly number[])[], u: readonly number[]): number[] => k.map((row) => row.reduce((sum, value, i) => sum + value * u[i]!, 0));
const force = (at: readonly number[], d: readonly number[]): number[] => [...d, at[1]! * d[2]! - at[2]! * d[1]!, at[2]! * d[0]! - at[0]! * d[2]!, at[0]! * d[1]! - at[1]! * d[0]!];

describe("🥇️ a wall-hung body beside the line of its supports", () => {
  const rail = [0, 0.3325, 0];
  const rest = [0, -0.3325, 0];
  const k = stiffness([{ d: force(rail, [1, 0, 0]), k: 1 }, { d: force(rail, [0, 1, 0]), k: 1 }, { d: force(rail, [0, 0, 1]), k: 1 }, { d: force(rest, [0, 0, 1]), k: 1 }]);
  const weight = force([0.0000672, 0.0063, -0.165], [0, -291.7478375, 0]);
  it("leaves no force and the moment of the weight about the rail", () => {
    const solved = solveForceFirst(k, weight);
    const left = times(k, solved.u).map((value, i) => weight[i]! - value);
    for (const i of [0, 1, 2]) expect(Math.abs(left[i]!)).toBeLessThan(1e-9);
    expect(left[5]!).toBeCloseTo(-291.7478375 * 0.0000672, 9);
    expect(solved.unbalanced).toBeCloseTo(291.7478375 * 0.0000672, 9);
  });
  it("differs from the plain projection, which leaves a force", () => {
    const plain = times(k, solveSemiDefinite(k, weight).u).map((value, i) => weight[i]! - value);
    expect(Math.abs(plain[0]!)).toBeGreaterThan(1e-3);
  });
});

describe("🥇️ seeded supports against numpy", () => {
  it("covers every rank below six", () => {
    expect(new Set(vectors.cases.map((c) => c.rank))).toEqual(new Set([1, 2, 3, 4, 5]));
  });
  it("gives the wrench of the lexicographic least squares", () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-force-first-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ cases: vectors.cases.map((c) => ({ k: stiffness(c.rows), f: c.f })) }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🥇️force-first/🐍️.py"), "geometry", [input])) as number[][];
      vectors.cases.forEach((c, n) => {
        const k = stiffness(c.rows);
        const given = times(k, solveForceFirst(k, c.f).u);
        const scale = 1 + Math.hypot(...c.f);
        given.forEach((value, i) => expect(Math.abs(value - reference[n]![i]!), `${c.name} component ${i}`).toBeLessThanOrEqual(vectors.tolerance * scale));
      });
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
  it("never leaves more force than the plain projection", () => {
    for (const c of vectors.cases) {
      const k = stiffness(c.rows);
      const left = (u: readonly number[]): number => Math.hypot(...times(k, u).slice(0, 3).map((value, i) => c.f[i]! - value));
      expect(left(solveForceFirst(k, c.f).u), c.name).toBeLessThanOrEqual(left(solveSemiDefinite(k, c.f).u) + 1e-9 * (1 + Math.hypot(...c.f)));
    }
  });
});

describe("🥇️ supports of full rank", () => {
  it("equal the minimum-norm solution", () => {
    const k = stiffness([[1, 0, 0], [0, 1, 0], [0, 0, 1]].flatMap((d) => [{ d: force([0.3, 0, 0.2], d), k: 1 }, { d: force([-0.4, 0.1, -0.3], d), k: 0.5 }, { d: force([0.1, 0.5, 0.4], d), k: 2 }]));
    const f = [12, -340, 5, 7, -3, 21];
    const [first, plain] = [solveForceFirst(k, f), solveSemiDefinite(k, f)];
    first.u.forEach((value, i) => expect(value).toBeCloseTo(plain.u[i]!, 9));
    expect(first.unbalanced).toBeLessThan(1e-9);
  });
});
