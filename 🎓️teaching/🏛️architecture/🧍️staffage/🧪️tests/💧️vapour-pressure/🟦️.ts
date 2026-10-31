/** 💧️ Vapour pressure: Magnus saturation pressure and humidity conversions against IAPWS-95 / CoolProp. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/💧️vapour-pressure/🔣️.json" with { type: "json" };
import { saturationPressurePa, specificHumidity, vapourPressurePa } from "../../🔨️modules/🌡️thermal/🟦️.ts";

type Fx = { tolerances: { relative: number; humidityRelative: number }; humidity: { pressurePa: number; relativeHumidity: number }; expected: { saturation: { tempC: number; pascal: number }[]; specific: { tempC: number; specificHumidity: number; vapourPressurePa: number }[] } };
const fx = fixture as unknown as Fx;

describe("💧️ vapour pressure", () => {
  it("matches the IAPWS-95 saturation pressure within the tolerance over 0 … 60 °C", () => {
    for (const { tempC, pascal } of fx.expected.saturation) expect(Math.abs(saturationPressurePa(tempC) / pascal - 1)).toBeLessThan(fx.tolerances.relative);
  });

  it("is monotonic in temperature", () => {
    let last = 0;
    for (let t = -20; t <= 60; t += 0.5) {
      const p = saturationPressurePa(t);
      expect(p).toBeGreaterThan(last);
      last = p;
    }
  });

  it("converts relative humidity to vapour pressure and specific humidity", () => {
    for (const s of fx.expected.specific) {
      const env = { airTempC: s.tempC, meanRadiantTempC: s.tempC, relativeHumidity: fx.humidity.relativeHumidity, windMS: 0, windDirectionDeg: 0, solarWM2: 0, pressurePa: fx.humidity.pressurePa };
      expect(Math.abs(vapourPressurePa(env) / s.vapourPressurePa - 1)).toBeLessThan(fx.tolerances.relative);
      expect(Math.abs(specificHumidity(vapourPressurePa(env), env.pressurePa) / s.specificHumidity - 1)).toBeLessThan(fx.tolerances.humidityRelative);
    }
  });
});
