/** 🌉️ Simply supported beam (L = 4 m, self-weight UDL 1 kN/m plus 5 kN at midspan): reactions, moment diagram and midspan deflection against anastruct and the closed forms.
 * @see ./🐍️.py — the anastruct oracle that writes `🧫️fixtures/🌉️simply-supported/🔣️.json`
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import { buildState, loadCase, near } from "../../🔨️modules/🏗️structure/🧰️harness/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";

const doc = loadCase("🌉️simply-supported");
const oracle = doc.oracle.values as Record<string, number> & { stationMomentNm: Record<string, number> };
const result = solveStructure(buildState(doc));
const beam = result.members["beam"]!;
const at = (s: number) => beam.stations.filter((x) => Math.abs(x.s - s) < 1e-9);

describe("simply supported beam against anastruct", () => {
  it("splits the load evenly between pin and roller", () => {
    const [pin, roller] = result.reactions;
    expect(near(pin!.fyN, oracle["reactionLeftFyN"]!, 1e-9)).toBe(true);
    expect(near(roller!.fyN, oracle["reactionRightFyN"]!, 1e-9)).toBe(true);
    expect(near(pin!.fyN + roller!.fyN, 9000, 1e-12)).toBe(true);
    expect(Math.abs(pin!.fxN)).toBeLessThan(1e-9);
    expect(Math.abs(pin!.momentNm)).toBeLessThan(1e-9);
  });
  it("is in equilibrium", () => {
    expect(Math.abs(result.residual.fyN)).toBeLessThan(1e-9 * 9000);
    expect(Math.abs(result.residual.momentNm)).toBeLessThan(1e-9 * 36000);
  });
  it("reaches the maximum sagging moment at midspan", () => {
    const max = Math.max(...beam.stations.map((s) => s.momentNm));
    expect(near(max, 7000, 1e-9)).toBe(true);
    expect(Math.abs(max / -oracle["maxMomentNm"]! - 1)).toBeLessThan(5e-3);
    expect(near(max, -oracle["analyticMaxMomentNm"]!, 1e-9)).toBe(true);
    expect(at(2).length).toBe(2);
  });
  it("matches the closed-form moment at stations", () => {
    for (const [s, m] of Object.entries(oracle.stationMomentNm)) expect(near(at(Number(s)).at(-1)!.momentNm, -m, 1e-9, 1e-7)).toBe(true);
  });
  it("jumps the shear by the midspan load", () => {
    const [before, after] = at(2);
    expect(near(before!.shearN - after!.shearN, 5000, 1e-9)).toBe(true);
  });
  it("deflects within 0.5 % of anastruct and 1e-4 of the closed form at midspan", () => {
    const mid = beam.deflection!.find((p) => Math.abs(p.x - 2) < 1e-9)!;
    expect(Math.abs(mid.y / oracle["midDeflectionM"]! - 1)).toBeLessThan(5e-3);
    expect(Math.abs(mid.y / oracle["analyticMidDeflectionM"]! - 1)).toBeLessThan(1e-4);
    expect(beam.deflection![0]!.y).toBeCloseTo(0, 12);
    expect(beam.deflection!.at(-1)!.y).toBeCloseTo(0, 12);
  });
  it("discretises self-weight into at least 16 sub-forces", () => {
    expect(result.segmentWeights.length).toBeGreaterThanOrEqual(16);
    expect(near(result.segmentWeights.reduce((a, f) => a - f.fyN, 0), 4000, 1e-12)).toBe(true);
    expect(near(result.centreOfMass.x, 2, 1e-12)).toBe(true);
  });
});
