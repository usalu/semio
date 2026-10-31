/** 🫀️ Two-node thermoregulation: core and skin temperature, wettedness, skin blood flow, heat flows and respiration after 60 minutes against pythermalcomfort `two_nodes_gagge`. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🫀️two-node-gagge/🔣️.json" with { type: "json" };
import type { Environment } from "../../🧬️schema/🟦️.ts";
import { type ThermalFixture, thermalState } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { simulateBody, solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";

type Expected = { t_core: number; t_skin: number; w: number; m_bl: number; e_skin: number; q_sensible: number; q_res: number };
type Case = ThermalFixture["asset"] extends never ? never : { id: string; asset: ThermalFixture["asset"]; state: ThermalFixture["state"]; expected: Expected };
const fx = fixture as unknown as { tolerances: Record<string, number>; cases: Case[] };
const AREA = 1.8258;
const run = (c: Case, start: "neutral" | "steady" = "neutral") => { const state = thermalState(c); return simulateBody(state.asset, state.thermal, c.asset.thermal.environment as Environment, 3600, start, 60); };

describe("🫀️ two-node thermoregulation", () => {
  it.each(fx.cases.map((c) => c.id))("matches pythermalcomfort after 60 minutes in %s", (id) => {
    const c = fx.cases.find((x) => x.id === id)!;
    const r = run(c);
    const t = fx.tolerances;
    expect(Math.abs(r.field.coreTempC - c.expected.t_core)).toBeLessThan(t["coreK"]!);
    expect(Math.abs(r.field.meanSkinTempC - c.expected.t_skin)).toBeLessThan(t["skinK"]!);
    expect(Math.abs(r.field.skinWettedness - c.expected.w)).toBeLessThan(t["wettedness"]!);
    expect(Math.abs(r.field.skinBloodFlowLM2H - c.expected.m_bl)).toBeLessThan(t["bloodFlowRelative"]! * c.expected.m_bl + t["bloodFlowAbsolute"]!);
    expect(Math.abs(r.diag.regions[0]!.dryWM2 - c.expected.q_sensible)).toBeLessThan(t["sensibleWM2"]!);
    expect(Math.abs(r.diag.regions[0]!.evapWM2 - c.expected.e_skin)).toBeLessThan(t["evaporationWM2"]!);
    expect(Math.abs(r.field.respiratoryLatentW / AREA - c.expected.q_res)).toBeLessThan(t["respiratoryWM2"]!);
  });

  it("orders the thermoregulatory responses like the library across the conditions", () => {
    const r = Object.fromEntries(fx.cases.map((c) => [c.id, run(c)]));
    expect(r["hot-sweating"]!.field.skinWettedness).toBeGreaterThan(r["warm"]!.field.skinWettedness);
    expect(r["warm"]!.field.skinWettedness).toBeGreaterThan(r["neutral"]!.field.skinWettedness);
    expect(r["cold-windy"]!.field.meanSkinTempC).toBeLessThan(r["cool"]!.field.meanSkinTempC);
    expect(r["cool"]!.field.skinBloodFlowLM2H).toBeLessThan(r["neutral"]!.field.skinBloodFlowLM2H);
    expect(r["walking"]!.field.coreTempC).toBeGreaterThan(r["neutral"]!.field.coreTempC);
    expect(r["walking"]!.field.metabolicW / AREA).toBeCloseTo(2 * 58.2, 6);
  });

  it("starts the transient at the neutral set points and approaches the steady state monotonically", () => {
    const c = fx.cases.find((x) => x.id === "warm")!;
    const built = thermalState(c);
    const asset = built.asset;
    const env = c.asset.thermal.environment as Environment;
    const start = simulateBody(asset, built.thermal, env, 0);
    expect(start.field.coreTempC).toBe(36.8);
    expect(start.field.meanSkinTempC).toBe(33.7);
    const steady = simulateBody(asset, built.thermal, env, 0, "steady").field.coreTempC;
    let last = start.field.coreTempC;
    for (const minutes of [10, 30, 60, 240, 720]) {
      const core = simulateBody(asset, built.thermal, env, minutes * 60).field.coreTempC;
      expect(core).toBeGreaterThanOrEqual(last - 1e-9);
      expect(core).toBeLessThanOrEqual(steady + 1e-6);
      last = core;
    }
    expect(Math.abs(last - steady)).toBeLessThan(0.02);
  });

  it("balances the steady body: metabolic heat = respiration + skin loss + zero storage", () => {
    for (const c of fx.cases) {
      const s = run(c, "steady");
      const skinLoss = s.diag.regions.reduce((sum, d) => sum + (d.dryWM2 + d.evapWM2) * AREA, 0);
      expect(Math.abs(s.field.storageW)).toBeLessThan(0.01);
      expect(s.field.metabolicW - s.field.respiratorySensibleW - s.field.respiratoryLatentW - skinLoss).toBeCloseTo(0, 2);
    }
  });

  it("reports the skin beneath the clothing distinct from the clothing surface", () => {
    const c = fx.cases.find((x) => x.id === "cool")!;
    const field = solveThermal(thermalState(c), c.asset.thermal.environment as Environment);
    const r = field.regions["body"]!;
    expect(r.skinTempC!).toBeGreaterThan(r.surfaceTempC + 5);
    expect(r.surfaceTempC).toBeGreaterThan(c.asset.thermal.environment.airTempC);
  });
});
