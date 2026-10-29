/** 🫁️ Breath plume: the direction follows the bone rotation, looping duty reproduces parcels after one period, excess decays monotonically, parcels stay within a metre. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🫁️breath-plume/🔣️.json" with { type: "json" };
import { thermalState, type ThermalFixture } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { saturationVapourPressurePa, solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";

const fx = fixture as unknown as ThermalFixture & { expected: { reference: [number, number][]; endpoint: [number, number] } | null };
const env = (windMS: number) => ({ ...fx.asset.thermal.environment, windMS });
const solve = (windMS = 0, timeS = fx.state.timeS, angleDeg = 20) => solveThermal(thermalState(fx, { timeS, bones: { head: { angleDeg } } }), env(windMS));
const angle = (a: { x: number; y: number }, b: { x: number; y: number }) => (Math.atan2(b.y - a.y, b.x - a.x) * 180) / Math.PI;

describe("🫁️ breath plume", () => {
  it("starts along bone angle plus emitter direction and rotates with the bone", () => {
    const first = (a: number) => { const s = (solve(0, 1.9, a).streamlines[0] as { points: { x: number; y: number }[] }).points; return angle(s[0]!, s[1]!); };
    expect(first(20)).toBeGreaterThan(8);
    expect(first(20)).toBeLessThan(13);
    expect(Math.abs(first(50) - first(20) - 30)).toBeLessThan(fx.tolerances.directionDeg!);
  });

  it("emits at the anchor world position", () => {
    const s = solve().streamlines[0]!;
    expect(s.points[0]).toEqual({ x: 0.1, y: 1.62 });
  });

  it("reproduces parcels one period later (looping duty)", () => {
    for (const w of [0, 1.5]) {
      const a = solve(w, 1.9).parcels;
      const b = solve(w, 5.9).parcels;
      expect(b.length).toBe(a.length);
      a.forEach((p, i) => {
        expect(b[i]!.at.x).toBeCloseTo(p.at.x, 9);
        expect(b[i]!.at.y).toBeCloseTo(p.at.y, 9);
        expect(b[i]!.ageS).toBeCloseTo(p.ageS, 9);
      });
    }
  });

  it("is deterministic and scrub-consistent", () => {
    expect(solve(1, 3.3)).toEqual(solve(1, 3.3));
  });

  it("decays the temperature excess monotonically from the source excess", () => {
    const s = solve().streamlines[0]!;
    expect(s.excessK[0]).toBeCloseTo(26, 6);
    for (let i = 1; i < s.excessK.length; i++) expect(s.excessK[i]!).toBeLessThanOrEqual(s.excessK[i - 1]! + 1e-12);
    const parcels = [...solve().parcels].sort((p, q) => p.ageS - q.ageS);
    for (let i = 1; i < parcels.length; i++) expect(parcels[i]!.excessK).toBeLessThanOrEqual(parcels[i - 1]!.excessK + 1e-12);
  });

  it("dilutes vapour excess like the temperature excess", () => {
    const f = solve();
    const q = 0.622 * 0.8 * saturationVapourPressurePa(8) / (101325 - 0.378 * 0.8 * saturationVapourPressurePa(8));
    for (const p of f.parcels) expect((p.vapourKgPerKg - q) / (0.033 - q)).toBeCloseTo(p.excessK / 26, 6);
  });

  it("keeps every parcel within 1.2 m at 0 wind and 2 m at 2 m/s wind", () => {
    const reach = (w: number, t: number) => Math.max(0, ...solve(w, t).parcels.map((p) => Math.hypot(p.at.x - 0.1, p.at.y - 1.62)));
    for (const t of [0.3, 1.9, 2.3, 2.5, 3.1, 5.9]) {
      expect(reach(0, t)).toBeLessThan(1.2);
      expect(reach(2, t)).toBeLessThan(2);
    }
  });

  it("matches the solve_ivp oracle polyline", () => {
    if (!fx.expected) return;
    const ref = fx.expected.reference;
    const dist = (p: { x: number; y: number }) => Math.min(...ref.slice(1).map(([x1, y1], i) => {
      const [x0, y0] = ref[i]!;
      const [dx, dy] = [x1 - x0, y1 - y0];
      const t = Math.max(0, Math.min(1, ((p.x - x0) * dx + (p.y - y0) * dy) / (dx * dx + dy * dy || 1)));
      return Math.hypot(p.x - (x0 + t * dx), p.y - (y0 + t * dy));
    }));
    const s = solve().streamlines[0]!;
    for (const p of s.points) expect(dist(p)).toBeLessThan(fx.tolerances.polylineM!);
    const end = s.points[s.points.length - 1]!;
    expect(Math.hypot(end.x - fx.expected.endpoint[0], end.y - fx.expected.endpoint[1])).toBeLessThan(fx.tolerances.endpointM!);
  });
});
