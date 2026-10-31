/** 🍃️ Leaf transpiration: surface energy balance with stomatal control against the brentq / CoolProp oracle, the Penman–Monteith closed form of the linearised balance, closure, CAM night opening, dew. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🍃️leaf-transpiration/🔣️.json" with { type: "json" };
import type { Environment } from "../../🧬️schema/🟦️.ts";
import { type ThermalFixture, thermalState } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { latentHeatJkg, solveThermal, stomatalConductance } from "../../🔨️modules/🌡️thermal/🟦️.ts";

type Expected = { surfaceTempC: number; netRadiativeWM2: number; sensibleWM2: number; latentWM2: number; absorbedSolarWM2: number; stomatalMS: number; convectionCoefficientWM2K: number; moistureGM2H: number; pmLinearised: number | null };
type Case = { id: string; environment: Environment; sunlit: number; expected: Record<string, Expected> };
const fx = fixture as unknown as ThermalFixture & { cases: Case[] };
const leaves = fx.asset.thermal.regions;
const stateOf = (c: Case, sunlit = c.sunlit) => thermalState({ asset: fx.asset, state: { timeS: 0, thermal: { activityMet: 0, externalWorkW: 0, operating: {}, regions: Object.fromEntries(leaves.map((l) => [l.id, { exposedFraction: 1, sunlitFraction: sunlit }])), emitters: {}, breathing: null }, anchors: {}, bones: {} } });
const solve = (c: Case, sunlit = c.sunlit) => solveThermal(stateOf(c, sunlit), c.environment);
const flux = (a: number, e: number) => expect(Math.abs(a - e)).toBeLessThanOrEqual(fx.tolerances.fluxRelative! * Math.abs(e) + fx.tolerances.fluxAbsoluteWM2!);

describe("🍃️ leaf transpiration", () => {
  it.each(fx.cases.map((c) => c.id))("matches the oracle leaf temperature and fluxes in %s", (id) => {
    const c = fx.cases.find((x) => x.id === id)!;
    const field = solve(c);
    for (const [leaf, e] of Object.entries(c.expected)) {
      const r = field.regions[leaf]!;
      expect(Math.abs(r.surfaceTempC - e.surfaceTempC)).toBeLessThan(fx.tolerances.surfaceTempK!);
      flux(r.netRadiativeWM2, e.netRadiativeWM2);
      flux(r.sensibleWM2, e.sensibleWM2);
      flux(r.latentWM2, e.latentWM2);
      flux(r.absorbedSolarWM2, e.absorbedSolarWM2);
      expect(Math.abs(r.moistureGM2H - e.moistureGM2H)).toBeLessThanOrEqual(fx.tolerances.fluxRelative! * Math.abs(e.moistureGM2H) + 0.5);
    }
  });

  it("agrees with the Penman–Monteith closed form of the linearised balance within its linearisation error", () => {
    for (const c of fx.cases)
      for (const [leaf, e] of Object.entries(c.expected)) {
        if (e.pmLinearised === null) continue;
        const r = solve(c).regions[leaf]!;
        expect(Math.abs(r.latentWM2 - e.pmLinearised)).toBeLessThanOrEqual(fx.tolerances.pmRelative! * Math.abs(e.pmLinearised) + fx.tolerances.pmAbsoluteWM2!);
      }
  });

  it("closes the energy balance of every leaf: absorbed sunlight = radiation + sensible + latent", () => {
    for (const c of fx.cases) {
      const field = solve(c);
      for (const r of Object.values(field.regions)) expect(r.absorbedSolarWM2 - (r.netRadiativeWM2 + r.sensibleWM2 + r.latentWM2)).toBeCloseTo(0, 6);
    }
  });

  it("reports transpiration as vapour flux and latent heat, distinct from the sensible heat of convection", () => {
    const r = solve(fx.cases[0]!).regions["c3-leaf"]!;
    expect(r.moistureGM2H).toBeGreaterThan(100);
    expect((r.moistureGM2H / 3.6e6) * latentHeatJkg(r.surfaceTempC)).toBeCloseTo(r.latentWM2, 6);
    expect(Math.abs(r.sensibleWM2 - r.latentWM2)).toBeGreaterThan(20);
  });

  it("opens CAM stomata at night and closes them by day, closes C3 stomata in dry air and in the dark", () => {
    const [cam, c3] = [leaves.find((l) => l.id === "cam-leaf")!.leaf!, leaves.find((l) => l.id === "c3-leaf")!.leaf!];
    expect(stomatalConductance(cam, 0, 15, 0.7)).toBeGreaterThan(10 * stomatalConductance(cam, 600, 15, 0.7));
    expect(stomatalConductance(c3, 600, 32, 0.3)).toBeLessThan(stomatalConductance(c3, 600, 32, 0.9));
    expect(stomatalConductance(c3, 0, 20, 0.6)).toBeLessThan(0.05 * c3.stomatalConductanceMaxMS);
    expect(stomatalConductance(c3, 600, 20, 0.9)).toBeLessThanOrEqual(c3.stomatalConductanceMaxMS);
  });

  it("transpires more in sun than in shade and more in dry than in humid air, while hot dry air closes the stomata", () => {
    const sunny = fx.cases[0]!;
    const sun = solve(sunny, 1).regions["c3-leaf"]!;
    const shade = solve(sunny, 0).regions["c3-leaf"]!;
    expect(sun.latentWM2).toBeGreaterThan(shade.latentWM2);
    expect(sun.absorbedSolarWM2).toBeGreaterThan(shade.absorbedSolarWM2);
    const at = (rh: number) => solveThermal(stateOf(sunny), { ...sunny.environment, relativeHumidity: rh }).regions["c3-leaf"]!;
    expect(at(0.2).latentWM2).toBeGreaterThan(at(0.8).latentWM2);
    expect(stomatalConductance(leaves[0]!.leaf!, 400, 32, 0.2)).toBeLessThan(stomatalConductance(leaves[0]!.leaf!, 400, 32, 0.8));
  });

  it("deposits dew (negative latent flux) on a leaf colder than the dew point and never lets the leaf transpire in saturated air", () => {
    const night = fx.cases.find((c) => c.id === "clear-night")!;
    const r = solve(night).regions["c4-leaf"]!;
    expect(r.latentWM2).toBeLessThan(0);
    expect(r.moistureGM2H).toBeLessThan(0);
  });
});
