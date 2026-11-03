/** 🏗️ Seeded three-dimensional rigid trees (lateral offsets, sagittal angles, abduction, twist, loads with all force components and couples) against numpy: the root reaction, every joint load and member cuts, in all six components, with explicit left-handed to right-handed conversion in the oracle.
 * @see ./🐍️.py — the numpy oracle that writes `🧫️fixtures/🏗️rigid-trees-3d/🔣️.json`
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import { buildState, loadCase, type StructureCase } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";

type Row = Record<string, number>;
type Tree = { readonly joints: Record<string, Row>; readonly stations: Record<string, Row[]>; readonly reaction: Row; readonly frames: Record<string, { origin: number[]; u: number[]; v: number[]; w: number[] }> };
const fixture = loadCase("🏗️rigid-trees-3d") as unknown as { trees: StructureCase[]; oracle: { values: Record<string, Tree> } };
const FORCE = ["fxN", "fyN", "fzN"] as const;
const MOMENT = ["mxNm", "myNm", "mzNm"] as const;
const CUT = ["axialN", "shearN", "shearZN", "torsionNm", "momentNm", "momentLateralNm"] as const;

describe("seeded three-dimensional rigid trees against numpy", () => {
  it("covers lateral offsets, spread and twist in every tree", () => {
    expect(fixture.trees.length).toBeGreaterThanOrEqual(8);
    for (const tree of fixture.trees) {
      expect(tree.bones.some((b) => (b as { z?: number }).z !== undefined && Math.abs((b as { z?: number }).z!) > 0.01)).toBe(true);
      expect(tree.bones.some((b) => Math.abs((b as { abductionDeg?: number }).abductionDeg ?? 0) > 5)).toBe(true);
      expect(tree.bones.some((b) => Math.abs((b as { twistDeg?: number }).twistDeg ?? 0) > 5)).toBe(true);
    }
  });
  for (const tree of fixture.trees) {
    const expected = fixture.oracle.values[tree.case]!;
    const state = buildState(tree);
    const result = solveStructure(state);
    const scale = 1 + [...result.segmentWeights, ...result.applied].reduce((a, f) => a + Math.hypot(f.fxN, f.fyN, f.fzN) + Math.hypot(f.mxNm, f.myNm, f.mzNm), 0);
    const close = (a: number, b: number, tol = 1e-9): boolean => Math.abs(a - b) <= tol * Math.max(scale, Math.abs(b));
    describe(tree.case, () => {
      it("places every bone frame like the matrix product of the oracle", () => {
        for (const [id, f] of Object.entries(expected.frames)) {
          const frame = state.bones[id]!.frame;
          [frame.o, frame.u, frame.v, frame.w].forEach((vec, i) => [vec.x, vec.y, vec.z].forEach((c, k) => expect(Math.abs(c - [f.origin, f.u, f.v, f.w][i]![k]!)).toBeLessThan(1e-9)));
        }
      });
      it("reacts all six components and balances all six residual components", () => {
        const r = result.reactions[0]!;
        for (const key of [...FORCE, ...MOMENT]) expect(close(r[key], expected.reaction[key]!), `${key}: ${r[key]} vs ${expected.reaction[key]}`).toBe(true);
        for (const [key, v] of Object.entries(result.residual)) expect(Math.abs(v) / scale, key).toBeLessThan(1e-9);
      });
      it("transmits the oracle force and moment vector, torsion and both bendings through every joint", () => {
        for (const [id, e] of Object.entries(expected.joints)) {
          const j = result.joints[id]!;
          for (const key of [...FORCE, ...MOMENT, "axialN", "shearN", "shearZN", "torsionNm", "momentNm", "momentLateralNm"] as const) expect(close(j[key], e[key]!), `${id} ${key}: ${j[key]} vs ${e[key]}`).toBe(true);
        }
      });
      it("cuts every member with the oracle axial, both shears, torsion and both bending moments", () => {
        for (const [id, rows] of Object.entries(expected.stations)) {
          const stations = result.members[id]!.stations;
          for (const e of rows) {
            const s = stations.filter((x) => Math.abs(x.s - e.s!) < 1e-9).at(-1)!;
            for (const key of CUT) expect(close(s[key], e[key]!), `${id} ${key} at ${e.s}: ${s[key]} vs ${e[key]}`).toBe(true);
          }
        }
      });
    });
  }
});
