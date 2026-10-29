/** 🏗️ Cantilever (L = 2 m, tip load 1 kN, self-weight off): reactions, hogging moment, constant shear, stress and tip deflection against anastruct and the closed form PL³/3EI.
 * @see ./🐍️.py — the anastruct oracle that writes `🧫️fixtures/🏗️cantilever/🔣️.json`
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import { buildState, loadCase, near } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";

const doc = loadCase("🏗️cantilever");
const oracle = doc.oracle.values as Record<string, number>;
const result = solveStructure(buildState(doc));
const beam = result.members["beam"]!;

describe("cantilever against anastruct", () => {
  it("reacts R_y = +P and M = +PL counter-clockwise at the wall", () => {
    const wall = result.reactions[0]!;
    expect(wall.active).toBe(true);
    expect(near(wall.fyN, 1000, 1e-12)).toBe(true);
    expect(near(wall.momentNm, 2000, 1e-12)).toBe(true);
    expect(Math.abs(wall.fxN)).toBeLessThan(1e-9);
    expect(near(wall.fyN, oracle["reactionFyN"]!, 1e-9)).toBe(true);
    expect(near(wall.momentNm, oracle["reactionMomentNm"]!, 1e-9)).toBe(true);
  });
  it("is in equilibrium", () => {
    expect(Math.hypot(result.residual.fxN, result.residual.fyN)).toBeLessThan(1e-9 * 1000);
    expect(Math.abs(result.residual.momentNm)).toBeLessThan(1e-9 * 2000);
  });
  it("draws M(x) = -P(L-x) hogging and constant shear V = +P", () => {
    expect(beam.stations.length).toBeGreaterThanOrEqual(24);
    for (const s of beam.stations.filter((x) => x.s > 1e-9)) expect(near(s.momentNm, -1000 * (2 - s.s), 1e-9, 1e-9)).toBe(true);
    for (const s of beam.stations.filter((x) => x.s > 1e-9 && x.s < 2 - 1e-9)) expect(near(s.shearN, 1000, 1e-9)).toBe(true);
    const [inclusive, exclusive] = beam.stations.filter((x) => x.s < 1e-9);
    expect(Math.abs(inclusive!.momentNm)).toBeLessThan(1e-9);
    expect(near(exclusive!.momentNm, oracle["rootMomentNm"]!, 1e-9)).toBe(true);
    expect(near(exclusive!.shearN, oracle["shearN"]!, 1e-9)).toBe(true);
    expect(beam.stations.every((s) => Math.abs(s.axialN) < 1e-9)).toBe(true);
  });
  it("gives the extreme-fibre stress ± M c / I", () => {
    const root = beam.stations.filter((x) => x.s < 1e-9)[1]!;
    expect(near(root.stressMaxPa!, oracle["rootStressPa"]!, 1e-9)).toBe(true);
    expect(near(root.stressMinPa!, -oracle["rootStressPa"]!, 1e-9)).toBe(true);
  });
  it("deflects PL³/3EI at the tip within 0.5 % of anastruct", () => {
    const tip = beam.deflection!.at(-1)!;
    expect(tip.x).toBeCloseTo(2, 12);
    expect(Math.abs(tip.y / oracle["tipDeflectionM"]! - 1)).toBeLessThan(5e-3);
    expect(Math.abs(tip.y / oracle["analyticTipDeflectionM"]! - 1)).toBeLessThan(1e-4);
    expect(tip.y).toBeLessThan(0);
    expect(beam.deflection![0]).toEqual({ x: 0, y: 0 });
  });
  it("labels the model quasi-static in both languages", () => {
    expect(result.notes.some((n) => n.en.includes("quasi-static"))).toBe(true);
    expect(result.notes.some((n) => n.de.includes("quasi-statisch"))).toBe(true);
    expect(result.dynamics).toBe("quasi-static");
    expect(result.inertialForces).toHaveLength(0);
  });
});
