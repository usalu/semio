/** 🚗️ Exhaust in wind: the plume bends downwind, calm air is symmetric, wind mirrors, the path matches the solve_ivp oracle. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🚗️exhaust-wind/🔣️.json" with { type: "json" };
import { thermalState, type ThermalFixture } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";

type Case = { reference: [number, number][]; endpoint: [number, number]; length: number };
const fx = fixture as unknown as ThermalFixture & { expected: { cases: Record<string, Case>; environments: Record<string, { windMS: number; windDirectionDeg: number }> } };
const solve = (name: string) => solveThermal(thermalState(fx), { ...fx.asset.thermal.environment, ...fx.expected.environments[name]! });
const [x0, y0] = [1.2, 3.2];

const distance = (ref: [number, number][], p: { x: number; y: number }) =>
  Math.min(...ref.slice(1).map(([x1, y1], i) => {
    const [ax, ay] = ref[i]!;
    const [dx, dy] = [x1 - ax, y1 - ay];
    const t = Math.max(0, Math.min(1, ((p.x - ax) * dx + (p.y - ay) * dy) / (dx * dx + dy * dy || 1)));
    return Math.hypot(p.x - (ax + t * dx), p.y - (ay + t * dy));
  }));

describe("🚗️ exhaust in wind", () => {
  it("rises straight up in calm air and stays on the axis", () => {
    const s = solve("calm").streamlines[0]!;
    for (const p of s.points) expect(Math.abs(p.x - x0)).toBeLessThan(fx.tolerances.symmetryM!);
    for (let i = 1; i < s.points.length; i++) expect(s.points[i]!.y).toBeGreaterThanOrEqual(s.points[i - 1]!.y);
  });

  it("bends downwind: x grows with the wind and the plume is lower than in calm air", () => {
    const calm = solve("calm").streamlines[0]!;
    const right = solve("wind-right").streamlines[0]!;
    const left = solve("wind-left").streamlines[0]!;
    for (let i = 1; i < right.points.length; i++) {
      expect(right.points[i]!.x).toBeGreaterThanOrEqual(right.points[i - 1]!.x);
      expect(left.points[i]!.x).toBeLessThanOrEqual(left.points[i - 1]!.x);
    }
    expect(right.points[23]!.x).toBeGreaterThan(x0 + 0.5);
    expect(right.points[23]!.y).toBeLessThan(calm.points[23]!.y);
  });

  it("mirrors exactly between opposite winds", () => {
    const right = solve("wind-right").streamlines[0]!;
    const left = solve("wind-left").streamlines[0]!;
    right.points.forEach((p, i) => {
      expect(left.points[i]!.x - x0).toBeCloseTo(-(p.x - x0), 9);
      expect(left.points[i]!.y).toBeCloseTo(p.y, 9);
    });
  });

  for (const name of ["calm", "wind-right", "wind-left"]) {
    it(`follows the solve_ivp reference in ${name}`, () => {
      const c = fx.expected.cases[name]!;
      const s = solve(name).streamlines[0]!;
      for (const p of s.points) expect(distance(c.reference, p)).toBeLessThan(fx.tolerances.polylineM!);
      const end = s.points[23]!;
      expect(Math.hypot(end.x - c.endpoint[0], end.y - c.endpoint[1])).toBeLessThan(fx.tolerances.endpointRelative! * c.length + 1e-3);
    });
  }

  it("decays the excess temperature from 180 K monotonically and counts exhaust enthalpy", () => {
    const f = solve("calm");
    const s = f.streamlines[0]!;
    expect(s.excessK[0]).toBeCloseTo(180, 6);
    for (let i = 1; i < s.excessK.length; i++) expect(s.excessK[i]!).toBeLessThanOrEqual(s.excessK[i - 1]! + 1e-12);
    expect(f.totals.emitterSensibleW).toBeGreaterThan(0);
  });

  it("is silent when the emitter strength is zero", () => {
    const f = solveThermal(thermalState(fx, { thermal: { ...fx.state.thermal, emitters: { stack: 0 } } }));
    expect(f.streamlines).toHaveLength(0);
    expect(f.parcels).toHaveLength(0);
  });
});
