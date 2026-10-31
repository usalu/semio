/** ☁️ Condensation: saturation, enthalpy and the mixing-line condensate of exhaled and ambient air against CoolProp. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/☁️condensation/🔣️.json" with { type: "json" };
import { enthalpy, humidityRatio, moistFrom, saturationPressurePa, saturationRatio } from "../../🔨️modules/💨️transport/🌫️air/🟦️.ts";

type Mixing = { ambientTempC: number; ambientRelativeHumidity: number; fraction: number; tempC: number; liquid: number; saturated: boolean };
const expected = fixture.expected as unknown as { mixing: Mixing[]; saturation: { tempC: number; humidityRatio: number; enthalpy: number }[]; ambient: { tempC: number; humidityRatio: number; enthalpy: number }[] };
const p = fixture.pressurePa;
const state = (tC: number, rh: number) => {
  const w = humidityRatio(rh * saturationPressurePa(tC), p);
  return { w, h: enthalpy(tC, w, 0) };
};
const mixed = (row: Mixing) => {
  const e = state(fixture.exhaled.tempC, fixture.exhaled.relativeHumidity);
  const a = state(row.ambientTempC, row.ambientRelativeHumidity);
  return moistFrom(row.fraction * e.h + (1 - row.fraction) * a.h, row.fraction * e.w + (1 - row.fraction) * a.w, p);
};

describe("☁️ condensation", () => {
  it("matches CoolProp saturation humidity ratio", () => {
    for (const s of expected.saturation) expect(Math.abs(saturationRatio(s.tempC, p) / s.humidityRatio - 1)).toBeLessThan(fixture.tolerances.saturationRel);
  });

  it("matches CoolProp enthalpy of saturated air up to the reference-state constant", () => {
    const offset = expected.saturation[0]!.enthalpy - enthalpy(expected.saturation[0]!.tempC, saturationRatio(expected.saturation[0]!.tempC, p), 0);
    for (const s of expected.saturation) expect(Math.abs(enthalpy(s.tempC, saturationRatio(s.tempC, p), 0) + offset - s.enthalpy)).toBeLessThan(fixture.tolerances.enthalpyRel * Math.abs(s.enthalpy) + 150);
  });

  it("finds the mixing-line temperature and condensate of CoolProp", () => {
    let condensing = 0;
    for (const row of expected.mixing) {
      const ours = mixed(row);
      expect(Math.abs(ours.tC - row.tempC)).toBeLessThan(fixture.tolerances.tempK);
      expect(Math.abs(ours.wl - row.liquid) * 1000).toBeLessThan(fixture.tolerances.condensateGKg);
      expect(ours.wl > 0).toBe(row.saturated);
      if (row.saturated) condensing++;
    }
    expect(condensing).toBeGreaterThan(10);
  });

  it("conserves total water and enthalpy through the adjustment", () => {
    for (const row of expected.mixing) {
      const e = state(fixture.exhaled.tempC, fixture.exhaled.relativeHumidity);
      const a = state(row.ambientTempC, row.ambientRelativeHumidity);
      const h = row.fraction * e.h + (1 - row.fraction) * a.h;
      const wt = row.fraction * e.w + (1 - row.fraction) * a.w;
      const m = moistFrom(h, wt, p);
      expect(m.wv + m.wl).toBeCloseTo(wt, 14);
      expect(enthalpy(m.tC, m.wv, m.wl)).toBeCloseTo(h, 6);
    }
  });

  it("condenses cold ambient mixtures and not warm humid dilutions", () => {
    const cold = expected.mixing.find((r) => r.ambientTempC === 0 && r.fraction === 0.3)!;
    expect(mixed(cold).wl).toBeGreaterThan(0);
    const warm = expected.mixing.find((r) => r.ambientTempC === 20 && r.fraction === 0.05)!;
    expect(mixed(warm).wl).toBe(0);
  });
});
