/** 🌤️ Flow and environment: in every reference environment preset the dew point of the flow legend matches CoolProp and psychrolib, breath fogs exactly where CoolProp's mixing line is supersaturated, a body warmer than the air sends its plume up, a colder one down and an isothermal one none, and a machine that is switched off moves no air. */
import { describe, expect, it } from "vitest";
import presets from "../../🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🌊️flow-environments/🔣️.json" with { type: "json" };
import { SCENARIOS, scenario } from "../../🧫️fixtures/💨️transport/🟦️.ts";
import type { Asset, Environment } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedState, FlowField, ThermalField } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";

type Row = { preset: string; airTempC: number; relativeHumidity: number; dewPointC: number; dewPointPsychrolibC: number; supersaturationKgPerKg: number; breathFogs: boolean; decisive: boolean; plumeHeatW: number; plume: number };
const expected = fixture.expected as unknown as Row[];
const environmentOf = (id: string): Environment => (presets.presets as unknown as { id: string; environment: Environment }[]).find((p) => p.id === id)!.environment;
const base = scenario("human");
const cooling = (state: EvaluatedState, environment: Environment): ThermalField => {
  const sensibleW = fixture.body.conductanceWK * (fixture.body.surfaceTempC - environment.airTempC);
  return { ...base.options.thermal(state, environment), regions: { torso: { region: "torso", sensibleW, latentW: 0, surfaceTempC: fixture.body.surfaceTempC } }, totals: { netRadiativeW: 0, sensibleW, latentW: 0, moistureGH: 0, emitterSensibleW: 0, emitterLatentW: 0 } } as unknown as ThermalField;
};
const only = (asset: Asset, kinds: readonly string[]): Asset => ({ ...asset, thermal: { ...asset.thermal, emitters: asset.thermal.emitters.filter((e) => kinds.includes(e.kind)) } });

function flowOf(asset: Asset, environment: Environment, thermal = base.options.thermal): FlowField {
  return solveTransport(asset, { kind: "pose", pose: "rest", timeS: fixture.developS }, environment, { evaluate: base.options.evaluate, thermal, maxAgeS: fixture.developS, pathlines: false, flow: { domain: fixture.domain } }).flow!;
}

describe("🌤️ flow in every reference environment", () => {
  it("covers every preset of the catalog", () => {
    expect(expected.map((row) => row.preset)).toEqual(presets.presets.map((p) => p.id));
    expect(expected.some((row) => row.decisive && row.breathFogs)).toBe(true);
    expect(expected.some((row) => row.decisive && !row.breathFogs)).toBe(true);
    expect(new Set(expected.map((row) => row.plume))).toEqual(new Set([1, -1]));
  });

  it.each(expected.map((row) => row.preset))("states the dew point and shows fog only below it in %s", (id) => {
    const row = expected.find((r) => r.preset === id)!;
    const flow = flowOf(only(base.asset, ["breath"]), environmentOf(id));
    const allowed = row.dewPointC < 0 ? fixture.tolerances.frostPointK : fixture.tolerances.dewPointK;
    expect(Math.abs(flow.ambient.dewPointC - row.dewPointC)).toBeLessThan(allowed);
    expect(Math.abs(flow.ambient.dewPointC - row.dewPointPsychrolibC)).toBeLessThan(allowed);
    if (row.dewPointC < 0) expect(flow.ambient.dewPointC).toBeLessThanOrEqual(row.dewPointC);
    expect(flow.puffs.length).toBeGreaterThan(0);
    expect(flow.puffs.every((p) => p.vapourExcessKgPerKg >= -1e-12)).toBe(true);
    if (row.decisive) expect(flow.puffs.some((p) => p.condensateKgPerKg > 1e-7)).toBe(row.breathFogs);
  });

  it.each(expected.map((row) => row.preset))("sends the body plume the way of the temperature difference in %s", (id) => {
    const row = expected.find((r) => r.preset === id)!;
    const flow = flowOf(only(base.asset, ["plume"]), environmentOf(id), cooling);
    expect(flow.puffs.length).toBeGreaterThan(0);
    expect(flow.puffs.every((p) => Math.sign(p.excessK) === row.plume)).toBe(true);
    const heights = flow.puffs.map((p) => p.at.y);
    const source = SCENARIOS.kinematics.human.pivotY;
    const travel = environmentOf(id).windMS < 0.5 ? fixture.travelM.calm : fixture.travelM.windy;
    if (row.plume > 0) expect(Math.max(...heights)).toBeGreaterThan(source + travel);
    else expect(Math.min(...heights)).toBeLessThan(source - travel);
    for (const stream of flow.streams.filter((s) => s.role === "source")) {
      const rise = stream.points[stream.points.length - 1]!.y - stream.points[0]!.y;
      if (Math.abs(rise) > 0.2 && environmentOf(id).windMS < 0.5) expect(Math.sign(rise)).toBe(row.plume);
    }
  });

  it("shows no flow around an isothermal body or a machine that is off", () => {
    const isothermal = { ...environmentOf("office-still"), airTempC: fixture.body.surfaceTempC };
    const still = flowOf(only(base.asset, ["plume"]), isothermal, cooling);
    expect(still.streams).toEqual([]);
    expect(still.puffs).toEqual([]);
    const machine = scenario("machine", { operating: { engine: [[0, 0]] } });
    const off = solveTransport(machine.asset, { kind: "pose", pose: "idle", timeS: fixture.developS }, environmentOf("office-still"), { ...machine.options, maxAgeS: fixture.developS, flow: { domain: fixture.domain } }).flow!;
    expect(off.streams).toEqual([]);
    expect(off.puffs).toEqual([]);
  });
});
