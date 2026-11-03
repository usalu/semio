/** 🫕️ Phase change: the temperature of a heated region holds at the phase-change point and the surplus power becomes converted mass, against scipy `solve_ivp` with a terminal event (CoolProp air
 * properties and IAPWS-95 saturation pressure and latent heat), with the energy-balance closure `P t = A C ΔT + A ∫q_out + L m` and the steam emitter of the electric kettle following the evaporation.
 * @see ../../🧫️fixtures/🫕️phase-change/🔣️.json — the oracle document
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🫕️phase-change/🔣️.json" with { type: "json" };
import environments from "../../🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import type { Environment, PhaseChange } from "../../🧬️schema/🟦️.ts";
import { type ThermalFixtureAsset, catalogAsset, thermalState } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { solveThermal, solveThermalAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

type Row = { tempC: number; phaseKg: number; phaseW: number; closureJM2: number };
type Case = { id: string; transition: "boiling" | "melting"; pointC: number; timesS: number[]; asset: ThermalFixtureAsset; expected: { initialC: number; crossingS: number; latentHeatJKg: number; plateauW: number; plateauKgS: number; rows: Row[] } };
const cases = fixture.cases as unknown as Case[];
const tol = fixture.tolerances;
const kettle = cases.find((c) => c.id === "kettle-water")!;

const inputsAt = (c: Case, t: number) => ({ activityMet: 0, externalWorkW: 0, operating: { power: t > 0 ? 1 : 0 }, regions: { [c.id]: { exposedFraction: 1, sunlitFraction: 0 } }, emitters: {}, breathing: null, passengers: {} });
const stateAt = (c: Case, t: number, asset: ThermalFixtureAsset = c.asset) => thermalState({ asset, state: { timeS: t, thermal: inputsAt(c, t), anchors: {}, bones: {} } });
const environmentOf = (c: Case, asset: ThermalFixtureAsset = c.asset) => asset.thermal.environment as Environment;
const solveAt = (c: Case, t: number, asset: ThermalFixtureAsset = c.asset) => solveThermal(stateAt(c, t, asset), environmentOf(c, asset), { history: (time) => ({ inputs: inputsAt(c, time) }) }).regions[c.id]!;
const without = (c: Case): ThermalFixtureAsset => ({ ...c.asset, materials: c.asset.materials.map(({ phaseChange: _dropped, ...rest }) => rest) });
const area = (c: Case): number => c.asset.thermal.regions[0]!.areaM2;

describe.each(cases)("🫕️ phase change of $id ($transition)", (c) => {
  it("follows the oracle temperature and converted mass", () => {
    c.timesS.forEach((t, i) => {
      const e = c.expected.rows[i]!;
      const r = solveAt(c, t);
      expect(Math.abs(r.surfaceTempC - e.tempC), `T(${t})`).toBeLessThan(tol.tempK);
      expect(Math.abs(r.phaseChangedKg - e.phaseKg) * 1000, `m(${t})`).toBeLessThan(tol.massG);
      expect(Math.abs(r.phaseChangeW - e.phaseW), `P(${t})`).toBeLessThanOrEqual(tol.powerRel * c.expected.plateauW);
    });
  });

  it("never exceeds the point, holds it exactly on the plateau and starts converting only there", () => {
    for (let t = 1; t <= 900; t += 11) {
      const r = solveAt(c, t);
      expect(r.surfaceTempC).toBeLessThanOrEqual(c.pointC + 1e-9);
      expect(r.steadyTempC).toBeLessThanOrEqual(c.pointC + 1e-9);
      if (t > c.expected.crossingS + 70) {
        expect(r.surfaceTempC).toBe(c.pointC);
        expect(r.phaseChangeKgS).toBeGreaterThan(0);
      }
      if (t < c.expected.crossingS - 70) expect(r.phaseChangeW).toBe(0);
    }
  });

  it("balances the power on the plateau: conducted heat = radiation + convection + latent (including the conversion) and the mass rate is the conversion over the latent heat", () => {
    const r = solveAt(c, c.timesS[c.timesS.length - 1]!);
    const heat = r.conductedWM2 * area(c);
    expect(Math.abs(heat - (r.netRadiativeW + r.sensibleW + r.latentW)) / heat).toBeLessThan(1e-9);
    expect(Math.abs(r.phaseChangeKgS * c.asset.materials[0]!.phaseChange!.latentHeatJKg - r.phaseChangeW)).toBeLessThan(1e-9 * heat);
    expect(r.evaporationKgS).toBeGreaterThanOrEqual(r.phaseChangeKgS);
    expect(Math.abs(r.phaseChangeW - c.expected.plateauW) / c.expected.plateauW).toBeLessThan(tol.powerRel);
  });

  it("accumulates the converted mass as the plateau rate times the elapsed time", () => {
    const a = solveAt(c, c.timesS[c.timesS.length - 2]!);
    const b = solveAt(c, c.timesS[c.timesS.length - 1]!);
    const dt = c.timesS[c.timesS.length - 1]! - c.timesS[c.timesS.length - 2]!;
    expect(Math.abs(b.phaseChangedKg - a.phaseChangedKg - b.phaseChangeKgS * dt) / (b.phaseChangeKgS * dt)).toBeLessThan(0.02);
  });

  it("closes the energy balance of the oracle at every time", () => {
    const power = c.asset.thermal.regions[0]!.operating!.heatW / area(c);
    c.expected.rows.forEach((e, i) => expect(Math.abs(e.closureJM2)).toBeLessThan(tol.closureRel * power * c.timesS[i]!));
  });

  it("is the steady state of constant inputs: the point, no accumulated mass", () => {
    const r = solveThermal(stateAt(c, 10), environmentOf(c)).regions[c.id]!;
    expect(r.surfaceTempC).toBe(c.pointC);
    expect(r.steadyTempC).toBe(c.pointC);
    expect(r.phaseChangedKg).toBe(0);
    expect(Math.abs(r.phaseChangeW - c.expected.plateauW) / c.expected.plateauW).toBeLessThan(tol.powerRel);
  });

  it("overshoots the point without a phase change, so the cap is the phase change and nothing else", () => {
    const r = solveAt(c, 900, without(c));
    expect(r.surfaceTempC).toBeGreaterThan(c.pointC + 1);
    expect(r.phaseChangeW).toBe(0);
    expect(r.phaseChangedKg).toBe(0);
  });

  it("lets the region override the material", () => {
    const region = { ...c.asset.thermal.regions[0]!, phaseChange: { transition: c.transition, pointC: c.pointC - 30, latentHeatJKg: 1e6 } satisfies PhaseChange };
    const r = solveAt(c, 900, { ...c.asset, thermal: { ...c.asset.thermal, regions: [region] } });
    expect(r.surfaceTempC).toBe(c.pointC - 30);
    expect(Math.abs(r.phaseChangeKgS * 1e6 - r.phaseChangeW)).toBeLessThan(1e-9);
  });
});

describe("🫕️ constants against CoolProp", () => {
  it("declares water at its boiling point and latent heat at 1 atm", () => {
    const water = catalogAsset("electric-kettle").materials.find((m) => m.id === "water")!.phaseChange!;
    expect(water.transition).toBe("boiling");
    expect(Math.abs(water.pointC - fixture.coolprop.boilingPointC)).toBeLessThan(tol.pointK);
    expect(Math.abs(water.latentHeatJKg / fixture.coolprop.latentHeatBoilingJKg - 1)).toBeLessThan(tol.latentRel);
    expect(Math.abs(kettle.asset.materials[0]!.phaseChange!.latentHeatJKg / fixture.coolprop.latentHeatBoilingJKg - 1)).toBeLessThan(tol.latentRel);
  });
});

describe("🫕️ electric kettle at a constant 2 kW", () => {
  const asset = catalogAsset("electric-kettle");
  const room = environments.presets.find((p) => p.id === "office-neutral")!.environment as Environment;
  const clip = (timeS: number) => ({ kind: "clip", clip: "boil", timeS }) as const;
  const water = (timeS: number) => solveThermalAt(asset, clip(timeS), room).regions["water"]!;

  it("holds the element at its rated level for the whole clip", () => {
    for (const t of [0.5, 1, 2, 60, 120, 183, 190, 200]) expect(evaluate(asset, clip(t)).thermal.operating["power"]).toBe(Math.min(1, t));
  });

  it("heats to 100 degC in about three minutes and holds it", () => {
    expect(water(120).surfaceTempC).toBeLessThan(80);
    expect(water(170).surfaceTempC).toBeLessThan(97);
    for (const t of [185, 190, 195, 200]) expect(water(t).surfaceTempC).toBe(100);
    expect(water(184).phaseChangeW).toBeGreaterThan(1500);
  });

  it("evaporates about 0.85 g/s from the boil on and a few hundredths of a gram per second before", () => {
    expect(water(100).evaporationKgS * 1000).toBeLessThan(0.05);
    const boiling = water(195);
    expect(boiling.evaporationKgS * 1000).toBeGreaterThan(0.75);
    expect(boiling.evaporationKgS * 1000).toBeLessThan(0.85);
    expect(boiling.phaseChangedKg * 1000).toBeGreaterThan(5);
  });

  it("lets the steam vent follow the evaporation rate", () => {
    const steam = (timeS: number) => solveTransport(asset, clip(timeS), room, { pathlines: false, streamlines: false, maxAgeS: 1 }).emitters.find((e) => e.emitter === "steam")!;
    const before = steam(100);
    const after = steam(195);
    expect(after.volumeFlowM3S).toBeGreaterThan(0);
    expect(after.volumeFlowM3S / Math.max(before.volumeFlowM3S, 1e-12)).toBeGreaterThan(10);
    expect(Math.abs(after.volumeFlowM3S / before.volumeFlowM3S / (water(195).evaporationKgS / water(100).evaporationKgS) - 1)).toBeLessThan(0.15);
  });
});
