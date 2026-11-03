/** 🛋️ ISO 7730 comfort: PMV and PPD against pythermalcomfort `pmv_ppd_iso`, clo and met conversions (ISO 9920, ISO 8996), validity limits and the clothing insulation read back from an asset. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🛋️pmv-comfort/🔣️.json" with { type: "json" };
import gagge from "../../🧫️fixtures/🫀️two-node-gagge/🔣️.json" with { type: "json" };
import type { Asset, Environment } from "../../🧬️schema/🟦️.ts";
import { CLO_M2KW, MET_WM2_ISO, bodyComfort, clothingInsulationClo, cloToM2KW, m2KWToClo, metToWM2, pmvPpd, relativeAirSpeedMS, sensationOf } from "../../🔨️modules/🌡️thermal/🟦️.ts";

type Case = { id: string; inputs: { tdb: number; tr: number; v: number; rh: number; met: number; clo: number }; expected: { pmv: number; ppd: number; tsv: string } };
const fx = fixture as unknown as { tolerances: { pmv: number; ppdPercent: number }; cases: Case[] };
const run = (c: Case) => pmvPpd({ airTempC: c.inputs.tdb, meanRadiantTempC: c.inputs.tr, relativeAirSpeedMS: c.inputs.v, relativeHumidity: c.inputs.rh / 100, met: c.inputs.met, clo: c.inputs.clo });

describe("🛋️ ISO 7730 comfort", () => {
  it.each(fx.cases.map((c) => c.id))("matches pythermalcomfort PMV and PPD in %s", (id) => {
    const c = fx.cases.find((x) => x.id === id)!;
    const r = run(c);
    expect(Math.abs(r.pmv - c.expected.pmv)).toBeLessThan(fx.tolerances.pmv);
    expect(Math.abs(r.ppdPercent - c.expected.ppd)).toBeLessThan(fx.tolerances.ppdPercent);
    expect(r.sensation).toBe(c.expected.tsv.toLowerCase().replace(" ", "-"));
  });

  it("converts clo and met with the ISO constants", () => {
    expect(CLO_M2KW).toBe(0.155);
    expect(MET_WM2_ISO).toBe(58.15);
    expect(cloToM2KW(1)).toBeCloseTo(0.155, 12);
    expect(m2KWToClo(cloToM2KW(0.73))).toBeCloseTo(0.73, 12);
    expect(metToWM2(1.2)).toBeCloseTo(69.78, 10);
  });

  it("adds the ISO 7730 relative air speed only above 58 W/m2", () => {
    expect(relativeAirSpeedMS(0.1, 0.9)).toBeCloseTo(0.1, 12);
    expect(relativeAirSpeedMS(0.1, 1)).toBeCloseTo(0.1 + 0.0052 * 0.15, 12);
    expect(relativeAirSpeedMS(0.1, 2)).toBeCloseTo(0.1 + 0.0052 * (2 * 58.15 - 58), 12);
  });

  it("flags inputs outside the ISO 7730 validity limits and marks the neutral office valid", () => {
    expect(run(fx.cases[0]!).valid).toBe(true);
    const cold = run(fx.cases.find((c) => c.id === "outdoor-cold-extrapolated")!);
    expect(cold.valid).toBe(false);
    expect(cold.violations).toEqual(expect.arrayContaining(["air temperature outside 10..30 C", "mean radiant temperature outside 10..40 C", "PMV outside -2..+2"]));
  });

  it("responds monotonically: warmer air, more activity and more clothing raise the vote, wind lowers it", () => {
    const base = { airTempC: 22, meanRadiantTempC: 22, relativeAirSpeedMS: 0.1, relativeHumidity: 0.5, met: 1.2, clo: 0.6 };
    const pmv = (patch: Partial<typeof base>) => pmvPpd({ ...base, ...patch }).pmv;
    expect(pmv({ airTempC: 24 })).toBeGreaterThan(pmv({}));
    expect(pmv({ meanRadiantTempC: 30 })).toBeGreaterThan(pmv({}));
    expect(pmv({ met: 1.8 })).toBeGreaterThan(pmv({}));
    expect(pmv({ clo: 1 })).toBeGreaterThan(pmv({}));
    expect(pmv({ relativeAirSpeedMS: 0.8 })).toBeLessThan(pmv({}));
  });

  it("never lets PPD fall below 5 percent and orders the sensation classes", () => {
    for (const c of fx.cases) expect(run(c).ppdPercent).toBeGreaterThanOrEqual(5);
    expect(["cold", "cool", "slightly-cool", "neutral", "slightly-warm", "warm", "hot"]).toEqual([-3, -2, -1, 0, 1, 2, 3].map(sensationOf));
  });

  it("reads the clo of the Gagge asset back from its clothing layer and feeds the same comfort", () => {
    for (const c of (gagge as unknown as { cases: { inputs: { clo: number; tdb: number; v: number; rh: number; met: number }; asset: { thermal: Asset["thermal"] } }[] }).cases) {
      const asset = { thermal: { ...c.asset.thermal, passengers: [] } } as unknown as Asset;
      expect(clothingInsulationClo(asset)).toBeCloseTo(c.inputs.clo, 9);
      const env = c.asset.thermal.environment as Environment;
      const r = bodyComfort(asset, env, c.inputs.met)!;
      const direct = pmvPpd({ airTempC: env.airTempC, meanRadiantTempC: env.meanRadiantTempC, relativeAirSpeedMS: relativeAirSpeedMS(env.windMS, c.inputs.met), relativeHumidity: env.relativeHumidity, met: c.inputs.met, clo: c.inputs.clo });
      expect(r.pmv).toBeCloseTo(direct.pmv, 9);
    }
  });

  it("returns null for an asset without a body", () => {
    const asset = { thermal: { body: undefined, passengers: [], regions: [] } } as unknown as Asset;
    expect(clothingInsulationClo(asset)).toBeNull();
  });
});
