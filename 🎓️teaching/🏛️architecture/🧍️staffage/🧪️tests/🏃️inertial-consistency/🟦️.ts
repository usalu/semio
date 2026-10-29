/** 🏃️ Inertial consistency: for a moving body the reactions sum to Σ m (g + a) (d'Alembert), friction is capped at μ·N, flight carries no reaction and the quasi-static twin ignores the accelerations.
 * @see ./🐍️.py — the numpy oracle that writes `🧫️fixtures/🏃️inertial-consistency/🔣️.json`
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import { buildState, loadCase, near } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";

const doc = loadCase("🏃️inertial-consistency");
const values = doc.oracle.values as Record<string, Record<string, number>>;
const scenario = (id: string) => doc.scenarios.find((s) => s.id === id)!;
const solve = (id: string) => solveStructure(buildState(doc, scenario(id)));
const reaction = (id: string, side: string) => solve(id).reactions.find((r) => r.anchor === `foot-${side}`)!;

function required(id: string): { x: number; y: number } {
  const sc = scenario(id);
  const dynamic = (sc.dynamics ?? doc.dynamics) === "inertial";
  return doc.bones.reduce((acc, b) => {
    const a = dynamic ? (sc.accelerations?.[b.id] ?? { x: 0, y: 0 }) : { x: 0, y: 0 };
    return { x: acc.x + (b.mass?.kg ?? 0) * a.x, y: acc.y + (b.mass?.kg ?? 0) * (doc.gravityMS2 + a.y) };
  }, { x: 0, y: 0 });
}

describe("inertial consistency", () => {
  it("balances Σ reactions = Σ m (g + a) for the moving body", () => {
    const result = solve("moving");
    const need = required("moving");
    expect(result.dynamics).toBe("inertial");
    expect(near(result.reactions.reduce((a, r) => a + r.fyN, 0), need.y, 1e-9)).toBe(true);
    expect(near(result.reactions.reduce((a, r) => a + r.fxN, 0), need.x, 1e-9)).toBe(true);
    expect(near(need.y, values["moving"]!["expectedSumFyN"]!, 1e-9)).toBe(true);
    expect(Math.hypot(result.residual.fxN, result.residual.fyN)).toBeLessThan(1e-9 * need.y);
    expect(Math.abs(result.residual.momentNm)).toBeLessThan(1e-9 * need.y * 2);
  });
  it("matches the numpy foot reactions of the moving body", () => {
    const v = values["moving"]!;
    expect(near(reaction("moving", "left").fyN, v["normalLeftN"]!, 1e-9)).toBe(true);
    expect(near(reaction("moving", "right").fyN, v["normalRightN"]!, 1e-9)).toBe(true);
    expect(near(reaction("moving", "left").fxN, v["tangentLeftN"]!, 1e-9)).toBe(true);
    expect(near(reaction("moving", "right").fxN, v["tangentRightN"]!, 1e-9)).toBe(true);
  });
  it("lists d'Alembert forces -m a and states the inertial model", () => {
    const result = solve("moving");
    expect(result.inertialForces.length).toBeGreaterThanOrEqual(16);
    const fx = result.inertialForces.reduce((a, f) => a + f.fxN, 0);
    expect(near(fx, -required("moving").x, 1e-9)).toBe(true);
    expect(result.notes.some((n) => n.en.includes("inertial (d'Alembert"))).toBe(true);
  });
  it("caps friction at μ·N and reports the unbalanced horizontal residual", () => {
    const v = values["friction-saturated"]!;
    const result = solve("friction-saturated");
    for (const side of ["left", "right"]) {
      const r = reaction("friction-saturated", side);
      expect(Math.abs(r.fxN)).toBeLessThanOrEqual(0.2 * r.fyN * (1 + 1e-12));
      expect(near(r.fyN, v[`normal${side === "left" ? "Left" : "Right"}N`]!, 1e-9)).toBe(true);
      expect(near(r.fxN, v[`tangent${side === "left" ? "Left" : "Right"}N`]!, 1e-9)).toBe(true);
    }
    expect(near(result.residual.fxN, v["residualFxN"]!, 1e-9)).toBe(true);
    expect(Math.abs(result.residual.fxN)).toBeGreaterThan(1);
    expect(result.notes.some((n) => n.en.includes("slides"))).toBe(true);
    expect(result.notes.some((n) => n.en.includes("Unbalanced"))).toBe(true);
  });
  it("carries no reaction in flight and stays balanced for a = -g", () => {
    const result = solve("flight");
    expect(result.reactions).toHaveLength(0);
    expect(Math.hypot(result.residual.fxN, result.residual.fyN)).toBeLessThan(1e-9 * 1000);
    expect(Math.abs(result.residual.momentNm)).toBeLessThan(1e-9 * 1000);
  });
  it("states the unsupported flight phase honestly", () => {
    const result = solve("flight");
    expect(result.notes.some((n) => n.en.includes("Unsupported") && n.de.includes("Ungestützt"))).toBe(true);
  });
  it("keeps the contact closest to the centre-of-mass projection when every contact would pull", () => {
    const result = solve("pulled-down");
    const active = result.reactions.filter((r) => r.active);
    expect(active).toHaveLength(1);
    expect(active[0]!.anchor).toBe(values["pulled-down"]!["keptFoot"] as unknown as string);
    expect(result.notes.some((n) => n.en.includes("closest to the centre-of-mass projection"))).toBe(true);
  });
  it("ignores accelerations when quasi-static", () => {
    const result = solve("quasi-static");
    const weight = doc.bones.reduce((a, b) => a + (b.mass?.kg ?? 0), 0) * doc.gravityMS2;
    expect(result.dynamics).toBe("quasi-static");
    expect(result.inertialForces).toHaveLength(0);
    expect(near(result.reactions.reduce((a, r) => a + r.fyN, 0), weight, 1e-9)).toBe(true);
    expect(near(result.reactions.reduce((a, r) => a + r.fxN, 0), 0, 1e-9, 1e-9)).toBe(true);
    expect(result.notes.some((n) => n.en.includes("quasi-static"))).toBe(true);
  });
});
