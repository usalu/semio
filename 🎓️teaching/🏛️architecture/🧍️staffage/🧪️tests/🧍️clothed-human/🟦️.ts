/** 🧍️ Clothed human: core-resistance surface temperatures against the brentq / ISO 7730 oracle, respiration against ASHRAE, activity response. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🧍️clothed-human/🔣️.json" with { type: "json" };
import { thermalState, type ThermalFixture } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";

type Region = Record<string, number>;
const fx = fixture as unknown as ThermalFixture & { expected: { regions: Record<string, Region>; totals: { emitterSensibleW: number; emitterLatentW: number } } };
const field = solveThermal(thermalState(fx));

describe("🧍️ clothed human", () => {
  it("solves the clothing and skin surface temperatures within the tolerance", () => {
    for (const [id, e] of Object.entries(fx.expected.regions)) expect(Math.abs(field.regions[id]!.surfaceTempC - e["surfaceTempC"]!)).toBeLessThan(fx.tolerances.surfaceTempK!);
  });

  it("matches the oracle fluxes and convection coefficient", () => {
    for (const [id, e] of Object.entries(fx.expected.regions)) {
      const r = field.regions[id]!;
      for (const [key, value] of [["radiantFluxWM2", r.radiantFluxWM2], ["convectiveFluxWM2", r.convectiveFluxWM2], ["latentFluxWM2", r.latentFluxWM2], ["netFluxWM2", r.netFluxWM2], ["convectionCoefficientWM2K", r.convectionCoefficientWM2K]] as const)
        expect(Math.abs(value - e[key]!)).toBeLessThanOrEqual(fx.tolerances.fluxRelative! * Math.abs(e[key]!) + 1e-6);
    }
  });

  it("closes the balance: conduction from the core equals radiation, convection and evaporation", () => {
    const c = field.regions["hands"]!;
    expect(c.netFluxWM2).toBeCloseTo(c.radiantFluxWM2 + c.convectiveFluxWM2 + c.latentFluxWM2, 6);
    expect(c.netFluxWM2).toBeCloseTo((35 - c.surfaceTempC) / 0.05, 6);
  });

  it("evaporates only where the skin is wet (minimum wettedness 0.06 on skin, none through clothing)", () => {
    expect(field.regions["clothing"]!.latentFluxWM2).toBe(0);
    expect(field.regions["hands"]!.latentFluxWM2).toBeGreaterThan(0);
  });

  it("assigns ASHRAE respiratory loss to the breath emitter", () => {
    const { emitterSensibleW, emitterLatentW } = fx.expected.totals;
    expect(Math.abs(field.totals.emitterSensibleW - emitterSensibleW)).toBeLessThanOrEqual(fx.tolerances.totalsRelative! * Math.abs(emitterSensibleW) + 0.01);
    expect(Math.abs(field.totals.emitterLatentW - emitterLatentW)).toBeLessThanOrEqual(fx.tolerances.totalsRelative! * Math.abs(emitterLatentW) + 0.01);
  });

  it("warms the skin with activity and respects overrides", () => {
    const active = solveThermal(thermalState({ ...fx, state: { ...fx.state, thermal: { ...fx.state.thermal, activityMet: 3 } } }));
    expect(active.regions["hands"]!.surfaceTempC).toBeGreaterThan(field.regions["hands"]!.surfaceTempC);
    const fixed = solveThermal(thermalState({ ...fx, state: { ...fx.state, thermal: { ...fx.state.thermal, regions: { hands: { exposedFraction: 0.5, surfaceTempC: 30 } } } } }));
    expect(fixed.regions["hands"]!.surfaceTempC).toBe(30);
    expect(fixed.regions["hands"]!.convectiveW).toBeCloseTo(fixed.regions["hands"]!.convectiveFluxWM2 * 0.05, 9);
  });

  it("cools the surface in wind and cold air", () => {
    const cold = solveThermal(thermalState(fx), { ...fx.asset.thermal.environment, airTempC: 0, meanRadiantTempC: 0, windMS: 3 });
    expect(cold.regions["clothing"]!.surfaceTempC).toBeLessThan(field.regions["clothing"]!.surfaceTempC);
  });
});
