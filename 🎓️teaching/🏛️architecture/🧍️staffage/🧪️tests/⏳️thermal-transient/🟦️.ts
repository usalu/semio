/** ⏳️ Transient thermal response: first-order response with heat capacity against scipy `solve_ivp` of the nonlinear balance, monotone approach without oscillation, purity (scrub = play),
 * periodic response of a looping clip, slow body core against fast skin, invariance under the drawing scale. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/⏳️thermal-transient/🔣️.json" with { type: "json" };
import environments from "../../🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import type { Environment } from "../../🧬️schema/🟦️.ts";
import { type ThermalFixture, catalogAsset, thermalState } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { historyPlan, relax, solveThermal, solveThermalAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import type { EnvironmentTimeline } from "../../🔨️modules/🔗️contracts/🟦️.ts";

type Expected = { initialC: number; finalSteadyC: number; responseC: number[] };
const fx = fixture as unknown as ThermalFixture & { timesS: number[]; expected: Record<string, Expected> };
const env = fx.asset.thermal.environment as Environment;
const inputsAt = (t: number) => ({ activityMet: 0, externalWorkW: 0, operating: { power: t > 0 ? 1 : 0 }, regions: { "sunlit-plate": { exposedFraction: 1, sunlitFraction: t > 0 ? 1 : 0 }, "heated-plate": { exposedFraction: 1, sunlitFraction: 0 } }, emitters: {}, breathing: null, passengers: {} });
const at = (t: number) => thermalState({ asset: fx.asset, state: { timeS: t, thermal: inputsAt(t), anchors: {}, bones: {} } });
const solveAt = (t: number) => solveThermal(at(t), env, { history: (time) => ({ inputs: inputsAt(time) }) });

describe("⏳️ transient response", () => {
  it.each(["sunlit-plate", "heated-plate"])("follows the nonlinear oracle step response of %s", (id) => {
    const e = fx.expected[id]!;
    fx.timesS.forEach((t, i) => expect(Math.abs(solveAt(t).regions[id]!.surfaceTempC - e.responseC[i]!)).toBeLessThan(fx.tolerances.responseK!));
    expect(Math.abs(solveAt(0).regions[id]!.surfaceTempC - e.initialC)).toBeLessThan(fx.tolerances.steadyK!);
    expect(Math.abs(solveAt(3600).regions[id]!.steadyTempC - e.finalSteadyC)).toBeLessThan(fx.tolerances.steadyK!);
  });

  it("approaches the steady state monotonically without overshoot or oscillation", () => {
    for (const id of ["sunlit-plate", "heated-plate"]) {
      let last = -Infinity;
      for (let t = 5; t <= 4000; t += 47) {
        const r = solveAt(t).regions[id]!;
        expect(r.surfaceTempC).toBeGreaterThanOrEqual(last - 1e-9);
        expect(r.surfaceTempC).toBeLessThanOrEqual(r.steadyTempC + 1e-6);
        last = r.surfaceTempC;
      }
    }
  });

  it("responds with the time constant C/G: 63 % of the step after one time constant, within the nonlinearity", () => {
    for (const id of ["sunlit-plate", "heated-plate"]) {
      const e = fx.expected[id]!;
      const tau = solveAt(3600).regions[id]!.timeConstantS;
      const frac = (solveAt(tau).regions[id]!.surfaceTempC - e.initialC) / (e.finalSteadyC - e.initialC);
      expect(frac).toBeGreaterThan(0.55);
      expect(frac).toBeLessThan(0.72);
      expect(tau).toBeGreaterThan(100);
    }
  });

  it("is the exact exponential for constant forcing and unconditionally stable", () => {
    expect(relax(0, 10, 100, 100)).toBeCloseTo(10 * (1 - Math.exp(-1)), 12);
    expect(relax(0, 10, 1e9, 1)).toBe(10);
    expect(relax(3, 10, 5, 0)).toBe(10);
  });

  it("plans the history on an absolute grid: the same plan for nearby times, a pure function of the time", () => {
    const a = historyPlan(5000, 600);
    const b = historyPlan(5001, 600);
    expect(b.times.slice(0, -1).every((t, i) => t === a.times[i] || i >= a.times.length - 1)).toBe(true);
    expect(historyPlan(5000, 600)).toEqual(a);
    expect(a.times[a.times.length - 1]).toBe(5000);
    expect(a.times.length).toBeLessThanOrEqual(181);
  });

  it("gives the same temperatures when scrubbing in any order as when playing forward", () => {
    const forward = [200, 700, 1500, 3000].map((t) => solveAt(t).regions["heated-plate"]!.surfaceTempC);
    const shuffled = [3000, 200, 1500, 700].map((t) => solveAt(t).regions["heated-plate"]!.surfaceTempC);
    expect(shuffled[1]).toBe(forward[0]);
    expect(shuffled[3]).toBe(forward[1]);
    expect(shuffled[2]).toBe(forward[2]);
    expect(shuffled[0]).toBe(forward[3]);
  });
});

describe("⏳️ transient response of bodies and clips", () => {
  const human = catalogAsset("walking-commuter");
  const room = environments.presets.find((p) => p.id === "office-neutral")!.environment as Environment;
  const cold = environments.presets.find((p) => p.id === "winter-overcast")!.environment as Environment;
  const timeline: EnvironmentTimeline = { keys: [{ t: 0, environment: room, ease: "step" }, { t: 1, environment: cold, ease: "step" }] };

  it("cools the skin within minutes and the core only over hours after stepping into the cold", () => {
    const state = (t: number) => ({ ...evaluate(human, { kind: "pose", pose: "standing" }), timeS: t });
    const skin = [0, 300, 900, 3600].map((t) => solveThermal(state(t), timeline).regions["coat"]!.skinTempC!);
    const core = [0, 300, 900, 3600].map((t) => solveThermal(state(t), timeline).body!.coreTempC);
    for (let i = 1; i < skin.length; i++) expect(skin[i]!).toBeLessThanOrEqual(skin[i - 1]! + 1e-9);
    expect(skin[0]! - skin[2]!).toBeGreaterThan(0.5);
    expect(Math.abs(core[0]! - core[1]!)).toBeLessThan(0.2);
    expect(Math.abs(core[3]! - core[0]!)).toBeLessThan(1.5);
  });

  it("gives the periodic response of a looping clip: one period later the same state", () => {
    const clip = human.clips.find((c) => c.loop)!;
    const a = solveThermalAt(human, { kind: "clip", clip: clip.id, timeS: 3 * clip.durationS + 0.3 }, cold);
    const b = solveThermalAt(human, { kind: "clip", clip: clip.id, timeS: 4 * clip.durationS + 0.3 }, cold);
    expect(Math.abs(a.regions["coat"]!.surfaceTempC - b.regions["coat"]!.surfaceTempC)).toBeLessThan(0.05);
    expect(Math.abs(a.body!.coreTempC - b.body!.coreTempC)).toBeLessThan(0.05);
  });

  it("is a pure function of the absolute time: repeated and interleaved clip queries agree exactly", () => {
    const clip = human.clips.find((c) => c.loop)!;
    const q = (t: number) => solveThermalAt(human, { kind: "clip", clip: clip.id, timeS: t }, cold).regions["coat"]!.surfaceTempC;
    const x = q(2.5);
    q(7.1);
    expect(q(2.5)).toBe(x);
  });

  it("does not depend on the drawing scale", () => {
    const query = { kind: "pose", pose: "standing" } as const;
    const base = solveThermal(evaluate(human, query), cold);
    for (const denominator of [200, 100, 50] as const) {
      const state = evaluate(human, query, { kind: "scale", scale: { denominator } });
      expect(solveThermal(state, cold)).toEqual(base);
    }
  });
});
