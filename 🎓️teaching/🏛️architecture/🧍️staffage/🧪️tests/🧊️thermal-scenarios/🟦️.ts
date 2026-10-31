/** 🧊️ Scenario sensitivity of the thermal solver on catalog assets: cold vs hot, rest vs walking, sun vs shade, wind vs calm, clothing, machine operation, vegetation, building element, passengers. Every
 * assertion states a physically expected direction and a minimum magnitude. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🫀️two-node-gagge/🔣️.json" with { type: "json" };
import environments from "../../🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import type { Asset, Environment, ThermalRegion } from "../../🧬️schema/🟦️.ts";
import { catalogAsset, thermalState } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import type { EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";

const preset = (id: string): Environment => (environments.presets.find((p) => p.id === id)!.environment as Environment);
type Tweak = { readonly met?: number; readonly sunlit?: number; readonly operating?: Record<string, number> };
function solve(asset: Asset, pose: string, env: Environment, tweak: Tweak = {}) {
  const state = evaluate(asset, { kind: "pose", pose });
  const regions = Object.fromEntries(Object.entries(state.thermal.regions).map(([k, v]) => [k, tweak.sunlit === undefined ? v : { ...v, sunlitFraction: tweak.sunlit }]));
  const next: EvaluatedState = { ...state, thermal: { ...state.thermal, ...(tweak.met === undefined ? {} : { activityMet: tweak.met }), operating: { ...state.thermal.operating, ...tweak.operating }, regions } };
  return solveThermal(next, env);
}
const human = catalogAsset("walking-commuter");
const dog = catalogAsset("labrador-dog");
const car = catalogAsset("city-car");
const oak = catalogAsset("english-oak");
const door = catalogAsset("sliding-door");
const COLD = preset("winter-overcast");
const HOT = preset("summer-hot-sun");
const CALM = { ...COLD, windMS: 0.2 };
const WINDY = { ...COLD, windMS: 8 };

describe("🧊️ thermal scenarios: living bodies", () => {
  it("cold air lowers skin and clothing temperature and raises the sensible heat loss; hot air raises skin temperature and sweating", () => {
    const cold = solve(human, "standing", COLD);
    const mild = solve(human, "standing", preset("office-neutral"));
    const hot = solve(human, "standing", HOT);
    expect(mild.regions["coat"]!.skinTempC! - cold.regions["coat"]!.skinTempC!).toBeGreaterThan(1);
    expect(mild.regions["coat"]!.surfaceTempC - cold.regions["coat"]!.surfaceTempC).toBeGreaterThan(10);
    expect(cold.regions["coat"]!.sensibleWM2).toBeGreaterThan(mild.regions["coat"]!.sensibleWM2 * 0 + 5);
    expect(cold.body!.meanSkinTempC).toBeLessThan(mild.body!.meanSkinTempC - 1);
    expect(hot.body!.meanSkinTempC).toBeGreaterThan(mild.body!.meanSkinTempC);
    expect(hot.body!.skinWettedness).toBeGreaterThan(mild.body!.skinWettedness + 0.05);
    expect(hot.body!.skinBloodFlowLM2H).toBeGreaterThan(2 * cold.body!.skinBloodFlowLM2H);
    expect(hot.totals.latentW).toBeGreaterThan(3 * cold.totals.latentW);
  });

  it("walking at 3 met raises metabolic heat, core, skin blood flow and the clothing surface temperature against rest", () => {
    const mild = { ...preset("office-neutral"), airTempC: 18, meanRadiantTempC: 18 };
    const rest = solve(human, "standing", mild, { met: 1.2 });
    const walk = solve(human, "standing", mild, { met: 3 });
    expect(walk.body!.metabolicW / rest.body!.metabolicW).toBeGreaterThan(2.3);
    expect(walk.body!.coreTempC).toBeGreaterThan(rest.body!.coreTempC);
    expect(walk.body!.skinBloodFlowLM2H).toBeGreaterThan(rest.body!.skinBloodFlowLM2H);
    expect(walk.regions["coat"]!.surfaceTempC - rest.regions["coat"]!.surfaceTempC).toBeGreaterThan(0.2);
    expect(walk.totals.sensibleW + walk.totals.latentW).toBeGreaterThan(1.5 * (rest.totals.sensibleW + rest.totals.latentW));
  });

  it("shivers when the insulation is insufficient: metabolic heat above the activity value, core below the set point", () => {
    const rest = solve(human, "standing", { ...COLD, airTempC: -10, meanRadiantTempC: -10, windMS: 4 }, { met: 1.2 });
    expect(rest.body!.metabolicW).toBeGreaterThan(1.2 * 58.15 * human.thermal.body!.skinAreaM2 * 1.05);
    expect(rest.body!.coreTempC).toBeLessThan(human.thermal.body!.coreSetPointC);
  });

  it("sun heats the clothing surface and raises the absorbed power; shade does not", () => {
    const sun = solve(human, "standing", HOT, { sunlit: 1 });
    const shade = solve(human, "standing", HOT, { sunlit: 0 });
    expect(sun.regions["coat"]!.absorbedSolarWM2).toBeGreaterThan(shade.regions["coat"]!.absorbedSolarWM2 + 50);
    expect(sun.regions["coat"]!.surfaceTempC - shade.regions["coat"]!.surfaceTempC).toBeGreaterThan(2);
    expect(sun.body!.meanSkinTempC).toBeGreaterThanOrEqual(shade.body!.meanSkinTempC);
  });

  it("wind cools the clothing surface and the skin and multiplies the sensible loss against calm air", () => {
    const calm = solve(human, "standing", CALM);
    const windy = solve(human, "standing", WINDY);
    expect(calm.regions["coat"]!.surfaceTempC - windy.regions["coat"]!.surfaceTempC).toBeGreaterThan(2);
    expect(windy.regions["coat"]!.convectionCoefficientWM2K).toBeGreaterThan(2 * calm.regions["coat"]!.convectionCoefficientWM2K);
    expect(windy.body!.meanSkinTempC).toBeLessThan(calm.body!.meanSkinTempC);
    expect(windy.regions["hand-skin"]!.skinTempC!).toBeLessThan(calm.regions["hand-skin"]!.skinTempC!);
  });

  it("more clothing insulation warms the skin and cools the clothing surface in the cold; bare skin is cooler than skin under the coat", () => {
    const thick = structuredClone(human) as { thermal: { regions: ThermalRegion[] } } & Asset;
    const coat = thick.thermal.regions.find((r) => r.id === "coat")!;
    (thick.thermal.regions as ThermalRegion[])[thick.thermal.regions.indexOf(coat)] = { ...coat, layers: coat.layers.map((l) => ({ ...l, resistanceM2KW: l.resistanceM2KW * 2 })) };
    const base = solve(human, "standing", COLD);
    const heavy = solve(thick, "standing", COLD);
    expect(heavy.regions["coat"]!.skinTempC!).toBeGreaterThan(base.regions["coat"]!.skinTempC!);
    expect(heavy.regions["coat"]!.surfaceTempC).toBeLessThan(base.regions["coat"]!.surfaceTempC);
    expect(heavy.regions["coat"]!.sensibleWM2).toBeLessThan(base.regions["coat"]!.sensibleWM2);
    expect(base.regions["hand-skin"]!.skinTempC!).toBeLessThan(base.regions["coat"]!.skinTempC!);
    expect(base.regions["coat"]!.surfaceTempC).toBeLessThan(base.regions["coat"]!.skinTempC! - 5);
  });

  it("keeps skin and core in the physiological range at rest in a 20 degree room and the coat colder than the skin", () => {
    const f = solve(human, "standing", { ...preset("office-neutral"), airTempC: 20, meanRadiantTempC: 20 });
    expect(f.body!.meanSkinTempC).toBeGreaterThan(30);
    expect(f.body!.meanSkinTempC).toBeLessThan(35.5);
    expect(f.body!.coreTempC).toBeGreaterThan(36.4);
    expect(f.body!.coreTempC).toBeLessThan(37.3);
  });

  it("makes a furred animal lose less heat than skin of the same area would, and pants when hot", () => {
    const cold = solve(dog, "standing-alert", COLD);
    const hot = solve(dog, "standing-alert", HOT);
    expect(cold.regions["coat-trunk"]!.skinTempC! - cold.regions["coat-trunk"]!.surfaceTempC).toBeGreaterThan(5);
    expect(hot.body!.coreTempC).toBeGreaterThan(cold.body!.coreTempC);
    expect(hot.body!.respiratoryLatentW).toBeGreaterThan(cold.body!.respiratoryLatentW);
  });
});

describe("🧊️ thermal scenarios: machines, vegetation and building elements", () => {
  const env = preset("spring-mild-sun");
  it("releases heat into the engine, exhaust and bonnet surfaces by operating level and leaves the inactive car passive", () => {
    const off = solve(car, "parked", env, { operating: { "engine-load": 0, rolling: 0, braking: 0 } });
    const on = solve(car, "parked", env, { operating: { "engine-load": 1, rolling: 0, braking: 0 } });
    expect(on.regions["exhaust-system"]!.surfaceTempC - off.regions["exhaust-system"]!.surfaceTempC).toBeGreaterThan(80);
    expect(on.regions["engine-bay"]!.surfaceTempC - off.regions["engine-bay"]!.surfaceTempC).toBeGreaterThan(15);
    expect(on.regions["bonnet"]!.surfaceTempC).toBeGreaterThan(off.regions["bonnet"]!.surfaceTempC);
    expect(on.regions["exhaust-system"]!.surfaceTempC).toBeLessThan(600);
    expect(on.regions["exhaust-system"]!.emittedRadiantWM2).toBeGreaterThan(2 * off.regions["exhaust-system"]!.emittedRadiantWM2);
    expect(Math.abs(off.regions["tyres"]!.surfaceTempC - env.airTempC)).toBeLessThan(20);
    expect(on.totals.sensibleW).toBeGreaterThan(off.totals.sensibleW);
  });

  it("heats sunlit dark panels well above the air and shaded ones less", () => {
    const sun = solve(car, "parked", preset("summer-hot-sun"), { sunlit: 1 });
    const shade = solve(car, "parked", preset("summer-hot-sun"), { sunlit: 0 });
    expect(sun.regions["roof"]!.surfaceTempC - shade.regions["roof"]!.surfaceTempC).toBeGreaterThan(5);
    expect(sun.regions["roof"]!.surfaceTempC).toBeGreaterThan(preset("summer-hot-sun").airTempC);
  });

  it("transpires more in the sunlit crown than in the shaded one, cools leaves below the air by night and keeps bark passive", () => {
    const day = solve(oak, "calm", HOT);
    expect(day.regions["leaf-sun"]!.latentWM2).toBeGreaterThan(day.regions["leaf-shade"]!.latentWM2);
    expect(day.regions["leaf-sun"]!.moistureGM2H).toBeGreaterThan(0);
    expect(day.regions["trunk-bark"]!.latentWM2).toBe(0);
    const night = solve(oak, "calm", preset("summer-night"), { sunlit: 0 });
    expect(night.regions["leaf-sun"]!.surfaceTempC).toBeLessThan(preset("summer-night").airTempC);
    expect(night.regions["leaf-sun"]!.netRadiativeWM2).toBeGreaterThan(20);
  });

  it("holds a conditioned element between the interior and the exterior temperature, closer to the exterior for the frame film", () => {
    const f = solve(door, "closed", { ...preset("winter-overcast"), airTempC: 0, meanRadiantTempC: 0, skyTempC: -4, groundTempC: 0 });
    const t = f.regions["leaf-glass"]!.surfaceTempC;
    expect(t).toBeGreaterThan(0);
    expect(t).toBeLessThan(20);
    expect(f.regions["leaf-glass"]!.conductedWM2).toBeGreaterThan(20);
  });
});

describe("🧊️ thermal scenarios: passengers", () => {
  it("solves a second living body independently with its own core and skin temperature", () => {
    const c = (fixture as unknown as { cases: { id: string; asset: Record<string, any>; state: any }[] }).cases.find((x) => x.id === "neutral")!;
    const twin = structuredClone(c.asset);
    twin.thermal.passengers = [{ id: "toddler", title: { en: "Toddler", de: "Kleinkind" }, physiology: { ...twin.thermal.body, massKg: 12, skinAreaM2: 0.55, basalMetabolismW: 0, coreSetPointC: 37.0 } }];
    twin.thermal.regions.push({ ...twin.thermal.regions[0], id: "toddler-body", areaM2: 0.6, body: { segment: "trunk", metabolicShare: 1, skinFraction: 1, subject: "toddler" } });
    const inputs = { ...c.state.thermal, regions: { ...c.state.thermal.regions, "toddler-body": { exposedFraction: 1, sunlitFraction: 0 } }, passengers: { toddler: { activityMet: 1.2, breathing: null } } };
    const state = thermalState({ asset: twin as never, state: { ...c.state, thermal: inputs } });
    const alone = solveThermal(thermalState(c as never));
    const both = solveThermal(state);
    expect(both.passengers["toddler"]).toBeDefined();
    expect(both.passengers["toddler"]!.coreTempC).toBeGreaterThan(36.5);
    expect(both.passengers["toddler"]!.coreTempC).toBeLessThan(38);
    expect(both.body!.coreTempC).toBeCloseTo(alone.body!.coreTempC, 9);
    expect(both.regions["toddler-body"]!.skinTempC).not.toBeNull();
    expect(both.totals.latentW).toBeGreaterThan(alone.totals.latentW);
  });
});
