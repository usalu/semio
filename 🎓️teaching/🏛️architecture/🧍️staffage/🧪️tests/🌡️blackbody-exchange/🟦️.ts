/** 🌡️ Blackbody exchange: grey-body radiation against the CODATA oracle, with the secant radiation coefficient. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🌡️blackbody-exchange/🔣️.json" with { type: "json" };
import { thermalState, type ThermalFixture } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { radiationCoefficient, radiationFlux, solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";

const fx = fixture as unknown as ThermalFixture & { expected: Record<string, { radiantFluxWM2: number; radiationCoefficientWM2K: number; radiantW: number }> };
const field = solveThermal(thermalState(fx));
const close = (a: number, b: number, rel: number, abs = 0) => expect(Math.abs(a - b)).toBeLessThanOrEqual(rel * Math.abs(b) + abs);

describe("🌡️ blackbody exchange", () => {
  it("matches the oracle flux, coefficient and power for every region", () => {
    for (const [id, e] of Object.entries(fx.expected)) {
      const r = field.regions[id]!;
      close(r.radiantFluxWM2, e.radiantFluxWM2, fx.tolerances.relative!, fx.tolerances.absoluteWM2);
      close(r.radiationCoefficientWM2K, e.radiationCoefficientWM2K, fx.tolerances.relative!);
      close(r.radiantW, e.radiantW, fx.tolerances.relative!, fx.tolerances.absoluteWM2);
    }
  });

  it("gives 75.4 W/m² for ε 0.95, 33 °C against 20 °C", () => {
    close(radiationFlux(0.95, 33, 20), 75.4025, 0.005);
  });

  it("is zero at equal temperatures and uses the 4εσT³ tangent there", () => {
    expect(radiationFlux(0.95, 20, 20)).toBe(0);
    close(radiationCoefficient(0.95, 20, 20), 4 * 0.95 * 5.670374419e-8 * 293.15 ** 3, 1e-12);
  });

  it("reports loss positive and gain negative", () => {
    expect(field.regions["warm"]!.radiantFluxWM2).toBeGreaterThan(0);
    expect(field.regions["cold"]!.radiantFluxWM2).toBeLessThan(0);
  });

  it("sums region powers into the totals", () => {
    close(field.totals.radiantW, Object.values(field.regions).reduce((s, r) => s + r.radiantW, 0), 1e-12);
  });
});
