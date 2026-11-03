/** 🚰️ Water-bearing regions: a `hydronic` plate follows the circuit water and never exceeds the supply temperature, a `coupled` wall follows the water of its source region below the fill level and takes its heat from it, against scipy
 * `solve_ivp` of the nonlinear lumped balances with CoolProp air properties; plus the catalog radiator, kettle and pot.
 * @see ../../🧫️fixtures/🚰️hydronic-coupling/🔣️.json — the oracle document
 * @see ./🥒️.feature — the language-agnostic specification */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🚰️hydronic-coupling/🔣️.json" with { type: "json" };
import environments from "../../🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import type { Environment } from "../../🧬️schema/🟦️.ts";
import { type ThermalFixtureAsset, catalogAsset, thermalState } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { solveThermal, solveThermalAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { circuitWaterC } from "../../🔨️modules/🌡️thermal/🧱️balance/🟦️.ts";

const tol = fixture.tolerances;
type Plate = (typeof fixture.plates)[number];
const env = (asset: ThermalFixtureAsset) => asset.thermal.environment as Environment;
const inputs = (key: string, level: number, regions: readonly string[]) => ({ activityMet: 0, externalWorkW: 0, operating: { [key]: level }, regions: Object.fromEntries(regions.map((r) => [r, { exposedFraction: 1, sunlitFraction: 0 }])), emitters: {}, breathing: null, passengers: {} });
const solve = (asset: ThermalFixtureAsset, key: string, level: number, regions: readonly string[], t: number) =>
  solveThermal(thermalState({ asset, state: { timeS: t, thermal: inputs(key, level, regions), anchors: {}, bones: {} } }), env(asset), { history: (time) => ({ inputs: inputs(key, time > 0 ? level : 0, regions) }) });

describe.each(fixture.plates)("🚰️ hydronic plate $id", (c: Plate) => {
  const asset = c.asset as unknown as ThermalFixtureAsset;
  const plate = (t: number) => solve(asset, "heating", c.level, ["plate"], t).regions["plate"]!;

  it("takes the circuit water of its place along the flow from the table", () => {
    expect(circuitWaterC(asset.thermal.regions[0]!.hydronic!, c.level, env(asset).airTempC)).toBeCloseTo(c.waterC, 9);
  });

  it("follows the oracle response", () => {
    fixture.plateTimesS.forEach((t, i) => expect(Math.abs(plate(t).surfaceTempC - c.responseC[i]!), `T(${t})`).toBeLessThan(tol.plateK));
  });

  it("settles on the oracle steady state, below its water and the supply", () => {
    const steady = solveThermal(thermalState({ asset, state: { timeS: 1, thermal: inputs("heating", c.level, ["plate"]), anchors: {}, bones: {} } }), env(asset)).regions["plate"]!;
    expect(Math.abs(steady.surfaceTempC - c.steadyC)).toBeLessThan(tol.steadyK);
    expect(steady.surfaceTempC).toBeLessThan(c.waterC);
    expect(steady.surfaceTempC).toBeLessThanOrEqual(c.supplyC);
    for (const t of fixture.plateTimesS) expect(plate(t).surfaceTempC).toBeLessThanOrEqual(c.supplyC);
  });
});

describe("🫗️ coupled wall", () => {
  const c = fixture.coupled;
  const asset = c.asset as unknown as ThermalFixtureAsset;
  const at = (t: number) => solve(asset, "power", 1, ["water", "wall"], t).regions;

  it("follows the oracle water and wall temperatures including the boiling cap", () => {
    c.timesS.forEach((t, i) => {
      const r = at(t);
      expect(Math.abs(r["water"]!.surfaceTempC - c.expected.rows[i]!.waterC), `water(${t})`).toBeLessThan(tol.waterK);
      expect(Math.abs(r["wall"]!.surfaceTempC - c.expected.rows[i]!.wallC), `wall(${t})`).toBeLessThan(tol.wallK);
    });
  });

  it("holds the wall within a few kelvin of the water on the plateau, at the oracle steady wall temperature", () => {
    const r = at(300);
    expect(r["water"]!.surfaceTempC).toBe(c.pointC);
    expect(c.pointC - r["wall"]!.surfaceTempC).toBeLessThan(tol.wetGapK);
    expect(Math.abs(r["wall"]!.steadyTempC - c.expected.wallSteadyC)).toBeLessThan(tol.steadyK);
  });

  it("takes the heat of the wall from the water: the plateau conversion is the heating less the losses and the wall", () => {
    const r = at(300);
    expect(Math.abs(r["water"]!.phaseChangeW - c.expected.plateauW) / c.expected.plateauW).toBeLessThan(tol.plateauRel);
  });

  it("never lets the wall exceed the water", () => {
    for (let t = 5; t <= 300; t += 25) expect(at(t)["wall"]!.surfaceTempC).toBeLessThanOrEqual(at(t)["water"]!.surfaceTempC + 1e-9);
  });
});

describe("🌡️ catalog water-bearing assets", () => {
  const room = environments.presets.find((p) => p.id === "indoor-heated-winter")!.environment as Environment;
  const radiator = catalogAsset("panel-radiator");
  const supply = Math.max(...radiator.thermal.regions.flatMap((r) => r.hydronic?.rows.map((row) => row.supplyTempC) ?? []));

  it.each(["off", "night", "comfort", "rated"])("keeps every radiator surface at or below the supply temperature in pose %s", (pose) => {
    const field = solveThermalAt(radiator, { kind: "pose", pose }, room);
    for (const r of radiator.thermal.regions.filter((x) => x.model === "hydronic")) {
      expect(field.regions[r.id]!.surfaceTempC, r.id).toBeLessThanOrEqual(supply);
      expect(field.regions[r.id]!.steadyTempC, r.id).toBeLessThanOrEqual(supply);
    }
  });

  it("falls from the supply to the return along the front plate and delivers about the EN 442 output", () => {
    const rated = solveThermalAt(radiator, { kind: "pose", pose: "rated" }, room).regions;
    expect(rated["panel-front-upper"]!.surfaceTempC).toBeGreaterThan(rated["panel-front-mid"]!.surfaceTempC);
    expect(rated["panel-front-mid"]!.surfaceTempC).toBeGreaterThan(rated["panel-front-lower"]!.surfaceTempC);
    expect(rated["panel-front-upper"]!.surfaceTempC).toBeLessThan(75);
    const watts = Object.entries(rated).reduce((sum, [id, f]) => sum + f.conductedWM2 * radiator.thermal.regions.find((r) => r.id === id)!.areaM2, 0);
    expect(watts).toBeGreaterThan(1050);
    expect(watts).toBeLessThan(1300);
  });

  it.each([
    ["electric-kettle", "shell", "boil", 195],
    ["cooking-pot", "wall", "heat-to-boil", 340],
  ])("lets the %s wall follow the water within a few kelvin at the plateau", (id, wall, clip, t) => {
    const field = solveThermalAt(catalogAsset(id), { kind: "clip", clip, timeS: t }, environments.presets.find((p) => p.id === "office-neutral")!.environment as Environment);
    expect(field.regions["water"]!.surfaceTempC).toBe(100);
    expect(100 - field.regions[wall]!.steadyTempC).toBeLessThan(tol.wetGapK);
    expect(100 - field.regions[wall]!.steadyTempC).toBeGreaterThan(0);
  });
});
