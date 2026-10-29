/** 🍃️ Leaf transpiration: Penman–Monteith leaf balance and a passive sunlit panel against the brentq oracle. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🍃️leaf-transpiration/🔣️.json" with { type: "json" };
import { thermalState, type ThermalFixture } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import type { Environment } from "../../🧬️schema/🟦️.ts";

type Region = Record<string, number>;
const fx = fixture as unknown as ThermalFixture & { environments: Record<string, Environment>; expected: Record<string, Record<string, Region>> };
const solve = (name: string) => solveThermal(thermalState(fx), fx.environments[name]!);

describe("🍃️ leaf transpiration", () => {
  for (const name of ["windy-sun", "still-sun", "breezy-shade"]) {
    it(`matches the oracle temperature and latent flux in ${name}`, () => {
      const field = solve(name);
      for (const [id, e] of Object.entries(fx.expected[name]!)) {
        expect(Math.abs(field.regions[id]!.surfaceTempC - e["surfaceTempC"]!)).toBeLessThan(fx.tolerances.surfaceTempK!);
        expect(Math.abs(field.regions[id]!.latentFluxWM2 - e["latentFluxWM2"]!)).toBeLessThanOrEqual(fx.tolerances.latentRelative! * e["latentFluxWM2"]! + 1e-6);
        expect(Math.abs(field.regions[id]!.convectiveFluxWM2 - e["convectiveFluxWM2"]!)).toBeLessThanOrEqual(fx.tolerances.fluxRelative! * Math.abs(e["convectiveFluxWM2"]!) + 1e-3);
      }
    });
  }

  it("closes the leaf balance: absorbed solar equals radiation, convection and latent heat", () => {
    const r = solve("windy-sun").regions["leaf"]!;
    expect(r.netFluxWM2).toBeCloseTo(0, 6);
    expect(r.latentFluxWM2).toBeGreaterThan(0);
  });

  it("keeps the transpiring leaf cooler than the passive panel under the same sun", () => {
    const f = solve("windy-sun");
    expect(f.regions["leaf"]!.surfaceTempC).toBeLessThan(f.regions["panel"]!.surfaceTempC);
  });

  it("closes the passive panel balance and heats it above air in still sun", () => {
    const f = solve("still-sun");
    expect(f.regions["panel"]!.netFluxWM2).toBeCloseTo(0, 6);
    expect(f.regions["panel"]!.surfaceTempC).toBeGreaterThan(fx.environments["still-sun"]!.airTempC);
  });

  it("stops transpiring when the stomata close", () => {
    const closed = { ...fx, asset: { ...fx.asset, thermal: { ...fx.asset.thermal, regions: fx.asset.thermal.regions.map((r) => (r.id === "leaf" ? { ...r, stomatalConductanceMS: 0 } : r)) } } };
    const f = solveThermal(thermalState(closed), fx.environments["windy-sun"]!);
    expect(f.regions["leaf"]!.latentFluxWM2).toBe(0);
    expect(f.regions["leaf"]!.surfaceTempC).toBeGreaterThan(solve("windy-sun").regions["leaf"]!.surfaceTempC);
  });
});
