/** 🌡️ Radiation exchange: emitted flux εσT⁴, net longwave exchange and its sky/ground/surroundings split, secant coefficient and region power against the CODATA / scipy oracle. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🌡️blackbody-exchange/🔣️.json" with { type: "json" };
import { type ThermalFixture, thermalState } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { emittedFlux, radiantEnvironmentC, radiationCoefficient, radiationFlux, radiationSplit, solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";

type Expected = { emittedWM2: number; netWM2: number; netW: number; splitWM2: { sky: number; ground: number; surroundings: number }; coefficientWM2K: number };
const fx = fixture as unknown as ThermalFixture & { expected: Record<string, Expected> };
const state = thermalState(fx);
const field = solveThermal(state);
const close = (a: number, b: number) => expect(Math.abs(a - b)).toBeLessThanOrEqual(fx.tolerances.relative! * Math.abs(b) + fx.tolerances.absoluteWM2!);

describe("🌡️ radiation exchange", () => {
  it("matches the oracle emitted flux, net flux, power and coefficient of every region", () => {
    for (const [id, e] of Object.entries(fx.expected)) {
      const r = field.regions[id]!;
      close(r.emittedRadiantWM2, e.emittedWM2);
      close(r.netRadiativeWM2, e.netWM2);
      close(r.netRadiativeW, e.netW);
      expect(Math.abs(r.radiationCoefficientWM2K - e.coefficientWM2K)).toBeLessThanOrEqual(fx.tolerances.relative! * e.coefficientWM2K);
    }
  });

  it("splits the net exchange into sky, ground and surroundings like the oracle and sums to the net flux", () => {
    for (const [id, e] of Object.entries(fx.expected)) {
      const s = radiationSplit(field, id);
      close(s.sky, e.splitWM2.sky);
      close(s.ground, e.splitWM2.ground);
      close(s.surroundings, e.splitWM2.surroundings);
      expect(s.sky + s.ground + s.surroundings).toBeCloseTo(field.regions[id]!.netRadiativeWM2, 9);
    }
  });

  it("gives 75.4 W/m² for ε 0.95, 33 °C against 20 °C", () => {
    expect(Math.abs(radiationFlux(0.95, 33, 20) / 75.4025 - 1)).toBeLessThan(0.0005);
  });

  it("is zero at equal temperatures, uses the 4εσT³ tangent there and emits εσT⁴", () => {
    expect(radiationFlux(0.95, 20, 20)).toBe(0);
    expect(radiationCoefficient(0.95, 20, 20)).toBeCloseTo(4 * 0.95 * 5.670374419e-8 * 293.15 ** 3, 12);
    expect(emittedFlux(1, 0)).toBeCloseTo(5.670374419e-8 * 273.15 ** 4, 9);
  });

  it("reports loss positive and gain negative and the emitted flux independent of the environment", () => {
    expect(field.regions["warm-plate"]!.netRadiativeWM2).toBeGreaterThan(0);
    expect(field.regions["cold-wall"]!.netRadiativeWM2).toBeLessThan(0);
    const colder = solveThermal(state, { ...fx.asset.thermal.environment, skyTempC: -40, meanRadiantTempC: -10 });
    expect(colder.regions["warm-plate"]!.emittedRadiantWM2).toBe(field.regions["warm-plate"]!.emittedRadiantWM2);
    expect(colder.regions["warm-plate"]!.netRadiativeWM2).toBeGreaterThan(field.regions["warm-plate"]!.netRadiativeWM2);
  });

  it("equals the single effective radiant temperature of the view-factor weighted environment", () => {
    const env = fx.asset.thermal.environment;
    const r = fx.asset.thermal.regions.find((x) => x.id === "cold-wall")!;
    const eff = radiantEnvironmentC(env, r.geometry, r.skyViewFactor);
    expect(radiationFlux(0.95, 5, eff)).toBeCloseTo(field.regions["cold-wall"]!.netRadiativeWM2, 9);
  });

  it("sums region powers into the totals", () => {
    expect(field.totals.netRadiativeW).toBeCloseTo(Object.values(field.regions).reduce((s, r) => s + r.netRadiativeW, 0), 9);
  });
});
