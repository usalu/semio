/** ⚖️ Equilibrium of seeded random rigid trees: residual below 1e-9 relative, reactions equal to the numpy oracle, joint loads and member cuts consistent with the subtree resultants.
 * @see ./🐍️.py — the numpy oracle that writes `🧫️fixtures/⚖️equilibrium/🔣️.json`
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import { buildState, loadCase, near, type StructureCase } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";

const fixture = loadCase("⚖️equilibrium") as unknown as { trees: (StructureCase & { kindOfSupports: string })[]; oracle: { values: Record<string, { fxN: number; fyN: number; momentNm: number }[]> } };

describe("seeded random rigid trees", () => {
  it("covers every support family", () => {
    expect(fixture.trees.length).toBeGreaterThanOrEqual(18);
    expect(new Set(fixture.trees.map((t) => t.kindOfSupports))).toEqual(new Set(["fixed", "pin-roller", "three-pins"]));
  });
  for (const tree of fixture.trees) {
    const result = solveStructure(buildState(tree));
    const forces = [...result.segmentWeights, ...result.applied];
    const scale = 1 + forces.reduce((a, f) => a + Math.hypot(f.fxN, f.fyN), 0);
    const moments = 1 + forces.reduce((a, f) => a + Math.abs(f.at.x * f.fyN - f.at.y * f.fxN) + Math.abs(f.momentNm), 0);
    describe(`${tree.case} (${tree.kindOfSupports})`, () => {
      it("has a residual below 1e-9 relative", () => {
        expect(Math.hypot(result.residual.fxN, result.residual.fyN) / scale).toBeLessThan(1e-9);
        expect(Math.abs(result.residual.momentNm) / moments).toBeLessThan(1e-9);
      });
      it("reproduces the oracle reactions", () => {
        const expected = fixture.oracle.values[tree.case]!;
        result.reactions.forEach((r, i) => {
          expect(near(r.fxN, expected[i]!.fxN, 1e-9, 1e-9 * scale)).toBe(true);
          expect(near(r.fyN, expected[i]!.fyN, 1e-9, 1e-9 * scale)).toBe(true);
          if (tree.kindOfSupports === "fixed") expect(near(r.momentNm, expected[i]!.momentNm, 1e-9, 1e-9 * moments)).toBe(true);
        });
      });
      it("balances every bone against its children at the joints", () => {
        for (const b of tree.bones) {
          const own = [...result.segmentWeights, ...result.applied].filter((f) => f.bone === b.id);
          const kids = tree.bones.filter((c) => c.parent === b.id).map((c) => result.joints[c.id]!);
          const reactions = result.reactions.filter((r) => tree.anchors.find((a) => a.id === r.anchor)!.bone === b.id);
          if (!b.parent) continue;
          const expectedFy = kids.reduce((a, j) => a + j.fyN, 0) - own.reduce((a, f) => a + f.fyN, 0) - reactions.reduce((a, r) => a + r.fyN, 0);
          expect(near(result.joints[b.id]!.fyN, expectedFy, 1e-9, 1e-9 * scale)).toBe(true);
        }
      });
      it("has a vanishing resultant at the root cut", () => {
        const root = result.members[tree.bones[0]!.id]!;
        const first = root.stations[0]!;
        expect(Math.hypot(first.axialN, first.shearN) / scale).toBeLessThan(1e-9);
        expect(Math.abs(first.momentNm) / moments).toBeLessThan(1e-9);
      });
      it("reports at least 24 stations per member", () => {
        for (const m of Object.values(result.members)) expect(m.stations.length).toBeGreaterThanOrEqual(24);
      });
    });
  }
});
