/** 🍃️ Penman–Monteith: the combination equation with the FAO-56 reference-crop resistances against the published example 18 and pyet `pm_fao56`. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🍃️penman-monteith/🔣️.json" with { type: "json" };
import { penmanMonteith } from "../../🔨️modules/🌡️thermal/🟦️.ts";

type Case = { id: string; tmean: number; u2: number; rn: number; ea: number; es: number; elevation: number; pyet: number; published?: number };
const fx = fixture as unknown as { tolerances: { relative: number; publishedRelative: number }; cases: Case[] };
const FAO_LAMBDA = 2.45e6;
const pressureKpa = (z: number): number => 101.3 * ((293 - 0.0065 * z) / 293) ** 5.26;

function etMmPerDay(c: Case, surfaceResistance = 70): number {
  const le = penmanMonteith({ netRadiationWM2: (c.rn * 1e6) / 86400, groundHeatWM2: 0, airTempC: c.tmean, pressurePa: pressureKpa(c.elevation) * 1000, vapourPressurePa: c.ea * 1000, vapourPressureDeficitPa: (c.es - c.ea) * 1000, aerodynamicResistanceSM: 208 / c.u2, surfaceResistanceSM: surfaceResistance });
  return (le * 86400) / FAO_LAMBDA;
}

describe("🍃️ Penman–Monteith", () => {
  it.each(fx.cases.map((c) => c.id))("matches pyet pm_fao56 for %s", (id) => {
    const c = fx.cases.find((x) => x.id === id)!;
    expect(Math.abs(etMmPerDay(c) / c.pyet - 1)).toBeLessThan(fx.tolerances.relative);
  });

  it("reproduces the published FAO-56 example 18 (3.88 mm/day)", () => {
    const c = fx.cases.find((x) => x.id === "fao56-example-18")!;
    expect(Math.abs(etMmPerDay(c) / c.published! - 1)).toBeLessThan(fx.tolerances.publishedRelative);
  });

  it("falls with the surface resistance and tends to the energy-limited bound when the surface is free water", () => {
    const c = fx.cases[0]!;
    expect(etMmPerDay(c, 200)).toBeLessThan(etMmPerDay(c, 70));
    expect(etMmPerDay(c, 70)).toBeLessThan(etMmPerDay(c, 0));
    expect(etMmPerDay(c, 1e6)).toBeLessThan(0.05);
  });
});
