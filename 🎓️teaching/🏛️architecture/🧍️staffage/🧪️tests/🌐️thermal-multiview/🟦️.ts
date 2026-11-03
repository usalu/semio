/** 🌐️ Thermal and transport multiview: one physical state per instant, three projections. The thermal field and the air transport are solved in world metres (ahead x, up y, left z) and only projected to the elevation (x, y), the front (z, y) and the plan (x, z), so every region carries the same value, legend and range in all three views, parcels, pathlines and emitters sit at the same world place, a moving emitter (jogger breath, generator exhaust) projects consistently, wind drifts parcels sideways in the plan, and scrubbing equals playing.
 *
 * THE REFERENCE is numpy (🐍️.py): the passive wind drift of a zero-momentum parcel, the orthographic projection of world points and the symmetric auto range.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, Environment, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { DrawingItem, DrawingScene, EvaluationQuery, TransportField, TransportLayer, Vec3 } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { areaWeighted, environmentAt, historyPlan, solveThermalAt, thermalSession } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { DEFAULT_TURBULENCE, solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { ambientAt } from "../../🔨️modules/💨️transport/🌬️ambient/🟦️.ts";
import { SCENARIOS, scenario } from "../../🧫️fixtures/💨️transport/🟦️.ts";
import { STAFFAGE_ROOT, requireTool, python } from "../🧰️support/🧰️.ts";
import { catalog } from "../🧊️three-views/🧰️.ts";

const VIEWPOINTS: readonly Viewpoint[] = ["elevation", "front", "plan"];
const LAYERS: readonly TransportLayer[] = ["parcels", "pathlines", "streamlines", "condensation"];
const DENOMINATOR = 50;
const select = { kind: "scale", scale: { denominator: DENOMINATOR } } as const;
const assets = catalog();
const assetOf = (id: string): Asset => assets.find((a) => a.id === id)!;
const itemsOf = (scene: DrawingScene): DrawingItem[] => scene.layers.flatMap((l) => l.items);
const modelOf = (scene: DrawingScene, x: number, y: number) => ({ u: (x - scene.metadata.origin.xMm) / scene.metadata.mmPerMetre, v: (scene.metadata.origin.yMm - y) / scene.metadata.mmPerMetre });
const centre = (item: DrawingItem): { x: number; y: number } => {
  const points = item.kind === "path" ? item.commands.flatMap((c) => (c.op === "Z" ? [] : [{ x: c.x, y: c.y }])) : [];
  const xs = points.map((p) => p.x);
  const ys = points.map((p) => p.y);
  return { x: (Math.min(...xs) + Math.max(...xs)) / 2, y: (Math.min(...ys) + Math.max(...ys)) / 2 };
};
const parcelIndex = (item: DrawingItem): number => Number(item.id.split(":").at(-1));
const world = (viewpoint: Viewpoint, p: Vec3): { u: number; v: number } => (viewpoint === "front" ? { u: p.z, v: p.y } : viewpoint === "plan" ? { u: p.x, v: p.z } : { u: p.x, v: p.y });

const CASES: { readonly name: string; readonly asset: string; readonly query: EvaluationQuery }[] = [
  { name: "jogger breath on the run cycle", asset: "jogger", query: { kind: "clip", clip: "run-cycle", timeS: 2.3 } },
  { name: "diesel generator exhaust on the load cycle", asset: "diesel-generator", query: { kind: "clip", clip: "load-cycle", timeS: 4.1 } },
];

const thermalScenes = (asset: Asset, query: EvaluationQuery, range: { mode: "shared" } | { mode: "auto" }) =>
  Object.fromEntries(VIEWPOINTS.map((viewpoint) => [viewpoint, depict(evaluate(asset, query, select), { ...defaultDepictionOptions("thermal", "en", DENOMINATOR, viewpoint, "light"), range })])) as Record<Viewpoint, DrawingScene>;

const valuesByRegion = (scene: DrawingScene): Map<string, number[]> => {
  const out = new Map<string, number[]>();
  for (const item of itemsOf(scene)) if (item.kind === "path" && item.value !== undefined && item.region !== undefined && !item.id.startsWith("halo:")) out.set(item.region, [...(out.get(item.region) ?? []), item.value.value]);
  return out;
};

describe("🌐️ thermal view in elevation, front and plan", () => {
  for (const c of CASES)
    for (const mode of ["shared", "auto"] as const)
      it(`gives every region one value, one legend and one range in all three views: ${c.name} (${mode})`, () => {
        const asset = assetOf(c.asset);
        const field = solveThermalAt(asset, c.query, undefined);
        const scenes = thermalScenes(asset, c.query, { mode });
        const byView = VIEWPOINTS.map((v) => valuesByRegion(scenes[v]));
        const regions = new Set(byView.flatMap((m) => [...m.keys()]));
        expect(regions.size).toBeGreaterThan(2);
        let shared = 0;
        for (const id of regions) {
          const seen = byView.flatMap((m) => m.get(id) ?? []);
          for (const value of seen) expect(value).toBeCloseTo(field.regions[id]!.surfaceTempC, 9);
          if (byView.filter((m) => m.has(id)).length >= 2) shared++;
        }
        expect(shared).toBeGreaterThan(0);
        const legend = (v: Viewpoint) => scenes[v].legends.map(({ title, quantity, unit, min, max, stops, convention, range, conditions }) => ({ title, quantity, unit, min, max, stops, convention, range, conditions }));
        expect(legend("front")).toEqual(legend("elevation"));
        expect(legend("plan")).toEqual(legend("elevation"));
        expect(scenes.plan.metadata.timeS).toBe(scenes.elevation.metadata.timeS);
        expect(scenes.front.metadata.timeS).toBe(scenes.elevation.metadata.timeS);
      });

  it("names the regions a viewpoint cannot see instead of dropping them silently", () => {
    const asset = assetOf("jogger");
    const query: EvaluationQuery = { kind: "clip", clip: "run-cycle", timeS: 0.2 };
    const scenes = thermalScenes(asset, query, { mode: "shared" });
    const all = new Set(asset.thermal.regions.map((r) => r.id));
    for (const viewpoint of VIEWPOINTS) {
      const drawn = new Set(valuesByRegion(scenes[viewpoint]).keys());
      const missing = asset.thermal.regions.filter((r) => all.has(r.id) && !drawn.has(r.id));
      const note = scenes[viewpoint].legends[0]!.interpretation.en;
      for (const region of missing) expect(note).toContain(region.title.en);
    }
  });
});

describe("🌐️ transport overlay in elevation, front and plan", () => {
  for (const c of CASES)
    it(`projects one world transport field to the same place in all three views: ${c.name}`, () => {
      const asset = assetOf(c.asset);
      const state = evaluate(asset, c.query, select);
      const field = solveTransport(asset, c.query, asset.thermal.environment, { maxAgeS: 4 });
      expect(field.parcels.length).toBeGreaterThan(10);
      expect(field.pathlines.length).toBeGreaterThan(5);
      for (const viewpoint of VIEWPOINTS) {
        const scene = depict(state, { ...defaultDepictionOptions("light-solid", "en", DENOMINATOR, viewpoint), transport: LAYERS }, { transport: field });
        const items = itemsOf(scene).filter((i) => i.id.startsWith("parcel:"));
        expect(items.length).toBe(field.parcels.length);
        items.forEach((item) => {
          const i = parcelIndex(item);
          const at = centre(item);
          const m = modelOf(scene, at.x, at.y);
          const w = world(viewpoint, field.parcels[i]!.at);
          expect(m.u).toBeCloseTo(w.u, 5);
          expect(m.v).toBeCloseTo(w.v, 5);
        });
        const lines = itemsOf(scene).filter((i) => i.id.startsWith("pathline:"));
        expect(lines.length).toBe(field.pathlines.filter((l) => l.points.length >= 2).length);
      }
    });

  it("solves the same world parcels whichever view asks for them", () => {
    const asset = assetOf("jogger");
    const query: EvaluationQuery = { kind: "clip", clip: "run-cycle", timeS: 2.3 };
    const state = evaluate(asset, query, select);
    const direct = solveTransport(asset, query, asset.thermal.environment, { maxAgeS: 30 });
    const ids = (scene: DrawingScene) => itemsOf(scene).filter((i) => i.id.startsWith("parcel:")).map((i) => i.id).sort();
    const scenes = VIEWPOINTS.map((viewpoint) => depict(state, { ...defaultDepictionOptions("light-solid", "en", DENOMINATOR, viewpoint), transport: ["parcels"] }));
    expect(ids(scenes[1]!)).toEqual(ids(scenes[0]!));
    expect(ids(scenes[2]!)).toEqual(ids(scenes[0]!));
    expect(ids(scenes[0]!).length).toBe(direct.parcels.length);
  });

  it("draws the legends of the transport layers identically in every view", () => {
    const asset = assetOf("jogger");
    const query: EvaluationQuery = { kind: "clip", clip: "run-cycle", timeS: 2.3 };
    const state = evaluate(asset, query, select);
    const legends = VIEWPOINTS.map((viewpoint) => depict(state, { ...defaultDepictionOptions("light-solid", "en", DENOMINATOR, viewpoint), transport: LAYERS }).legends);
    expect(legends[1]).toEqual(legends[0]);
    expect(legends[2]).toEqual(legends[0]);
    expect(legends[0]!.length).toBeGreaterThan(1);
  });

  it("draws an emitter pointing at the viewer end-on instead of as a zero-length arrow", () => {
    const asset = assetOf("excavator");
    const query: EvaluationQuery = { kind: "pose", pose: asset.presentation.defaultPose, timeS: 5 };
    const state = evaluate(asset, query, select);
    const field = solveTransport(asset, query, asset.thermal.environment, { maxAgeS: 2 });
    const stack = field.emitters.find((e) => e.emitter === "exhaust-stack")!;
    expect(stack.direction.y).toBeGreaterThan(0.99);
    const plan = depict(state, { ...defaultDepictionOptions("light-solid", "en", DENOMINATOR, "plan"), transport: ["parcels"] }, { transport: field });
    const marker = itemsOf(plan).find((i) => i.id === "emitter:exhaust-stack")!;
    expect(marker.kind === "path" && marker.commands.some((c) => c.op === "C")).toBe(true);
  });
});

describe("🌐️ wind, moving emitters and time in three dimensions", () => {
  const human = scenario("human");
  const calm = SCENARIOS.environments["calm-cold"];

  it("drifts a parcel sideways with a wind bearing towards the left and not at all along the other axes", () => {
    const tree = scenario("tree");
    const wind: Environment = { ...SCENARIOS.assets.tree.thermal.environment, windMS: 2, windDirectionDeg: 90 };
    const field = solveTransport(tree.asset, { kind: "pose", pose: "summer", timeS: 12 }, wind, { ...tree.options, turbulence: { intensity: 0, floorMS: 1e-6, lagrangianTimeS: 1 }, maxAgeS: 6 });
    const line = [...field.pathlines].sort((a, b) => b.points.length - a.points.length)[0]!;
    expect(line.points.length).toBeGreaterThan(10);
    const last = line.points[line.points.length - 1]!;
    expect(Math.abs(last.z - line.points[0]!.z)).toBeGreaterThan(1);
    expect(Math.abs(last.x - line.points[0]!.x)).toBeLessThan(1e-4);
  });

  it("carries a moving emitter and its older parcels consistently in world space", () => {
    const s = scenario("human", { rootX: { x0: 0, vx: 0 }, rootZ: { z0: 0.2, vz: 0.8 }, yawKeysDeg: [[0, 0], [3, 0], [4, 90], [10, 90]] });
    const nose = (t: number) => solveTransport(s.asset, { kind: "pose", pose: "rest", timeS: t }, calm, s.options).emitters.find((e) => e.emitter === "nose")!;
    expect(nose(1).at.z - nose(0).at.z).toBeCloseTo(0.8, 9);
    expect(nose(2.5).at.z - nose(1.5).at.z).toBeCloseTo(0.8, 9);
    const before = nose(0).direction;
    const after = nose(10).direction;
    expect(after.x).toBeCloseTo(0, 9);
    expect(after.z).toBeCloseTo(before.x, 9);
    expect(after.y).toBeCloseTo(before.y, 9);
    const early = solveTransport(s.asset, { kind: "pose", pose: "rest", timeS: 2.6 }, calm, s.options);
    const late = solveTransport(s.asset, { kind: "pose", pose: "rest", timeS: 3.4 }, calm, s.options);
    let compared = 0;
    for (const line of early.pathlines) {
      const later = late.pathlines.find((l) => l.parcel === line.parcel);
      if (later === undefined) continue;
      for (let i = 0; i < line.points.length - 1; i++) {
        expect(later.points[i]).toEqual(line.points[i]);
        compared++;
      }
    }
    expect(compared).toBeGreaterThan(20);
    const zs = early.parcels.map((p) => p.at.z);
    expect(Math.max(...zs) - Math.min(...zs)).toBeGreaterThan(0.1);
  });

  it("fans the breath of a nozzle in both transverse planes with the same half angle", () => {
    const field = solveTransport(human.asset, { kind: "pose", pose: "rest", timeS: 2.6 }, calm, { ...human.options, turbulence: { intensity: 0, floorMS: 1e-6, lagrangianTimeS: 1 } });
    const nose = field.parcels.filter((p) => p.emitter === "nose");
    expect(nose.length).toBeGreaterThan(10);
    const spread = (pick: (v: Vec3) => number) => Math.sqrt(nose.reduce((sum, p) => sum + pick(p.velocity) ** 2, 0) / nose.length);
    expect(spread((v) => v.z)).toBeGreaterThan(0);
    expect(spread((v) => v.z)).toBeLessThan(2 * spread((v) => v.y) + 1);
  });

  it("is bitwise the same whether the field is played frame by frame, scrubbed or evaluated cold", () => {
    const s = scenario("human", { rootZ: { z0: 0, vz: 0.6 }, yawKeysDeg: [[0, 0], [1, 0], [2, 45], [10, 45]], rootX: { x0: 0, vx: 1.3 } });
    const buffer = new Float64Array(1);
    const bits = (field: TransportField): string =>
      JSON.stringify(field, (_k, v) => {
        if (typeof v !== "number") return v;
        buffer[0] = v;
        return Array.from(new Uint8Array(buffer.buffer)).join(",");
      });
    const at = (t: number): EvaluationQuery => ({ kind: "pose", pose: "rest", timeS: t });
    const played: string[] = [];
    for (let i = 0; i <= 30; i++) played.push(bits(solveTransport(s.asset, at(2 + i / 15), calm, { ...s.options, maxAgeS: 4 })));
    const scrubbed = [30, 7, 19, 0, 30].map((i) => bits(solveTransport(s.asset, at(2 + i / 15), calm, { ...s.options, maxAgeS: 4 })));
    expect(scrubbed).toEqual([played[30], played[7], played[19], played[0], played[30]]);
    const cold = scenario("human", { rootZ: { z0: 0, vz: 0.6 }, yawKeysDeg: [[0, 0], [1, 0], [2, 45], [10, 45]], rootX: { x0: 0, vx: 1.3 } });
    expect(bits(solveTransport(cold.asset, at(2 + 12 / 15), calm, { ...cold.options, maxAgeS: 4 }))).toBe(played[12]);
  });
});

describe("🌐️ one environment, one clock, one weighting", () => {
  const jogger = assetOf("jogger");
  const base = jogger.thermal.environment;
  const timeline = { keys: [{ t: 0, environment: { ...base, windMS: 2, windDirectionDeg: 0 }, ease: "linear" as const }, { t: 20, environment: { ...base, windMS: 2, windDirectionDeg: 180, airTempC: base.airTempC - 10 }, ease: "linear" as const }] };

  it("blends opposite winds as a velocity vector through calm in thermal and transport alike", () => {
    expect(environmentAt(timeline, 10).windMS).toBeCloseTo(0, 9);
    expect(environmentAt(timeline, 5).windMS).toBeCloseTo(1, 9);
    expect(environmentAt(timeline, 5).windDirectionDeg).toBeCloseTo(0, 9);
    expect(ambientAt(timeline, 5, null, DEFAULT_TURBULENCE).environment).toEqual(environmentAt(timeline, 5));
    const wind = ambientAt(timeline, 15, null, DEFAULT_TURBULENCE).windAt(1);
    expect(wind.x).toBeCloseTo(-1, 9);
    expect(wind.y).toBe(0);
    expect(wind.z).toBeCloseTo(0, 9);
  });

  it("keeps the thermal history continuous across a clip loop boundary", () => {
    const period = jogger.clips.find((c) => c.id === "run-cycle")!.durationS;
    const at = (timeS: number) => solveThermalAt(jogger, { kind: "clip", clip: "run-cycle", timeS }, timeline);
    for (const loop of [3, 8]) {
      const before = at(loop * period - 1e-3);
      const after = at(loop * period + 1e-3);
      for (const id of Object.keys(before.regions)) expect(Math.abs(after.regions[id]!.surfaceTempC - before.regions[id]!.surfaceTempC)).toBeLessThan(0.05);
    }
  });

  it("states the clamped neutral grey in the legend instead of the unclamped air temperature", () => {
    const query: EvaluationQuery = { kind: "pose", pose: jogger.presentation.defaultPose, timeS: 0 };
    const hot = depict(evaluate(jogger, query, select), { ...defaultDepictionOptions("thermal", "en", DENOMINATOR, "plan", "light"), range: { mode: "fixed", min: 30, max: 40 } });
    const legend = hot.legends[0]!;
    expect(legend.convention.en).toContain("30.0 °C");
    expect(legend.convention.en).toContain("outside the fixed range");
    expect(legend.convention.de).toContain("außerhalb des festen Bereichs");
  });

  it("states in both languages what the absence of a hatch means, in every view and for thermal and transport", () => {
    const query: EvaluationQuery = { kind: "pose", pose: jogger.presentation.defaultPose, timeS: 0 };
    const state = evaluate(jogger, query, select);
    for (const viewpoint of VIEWPOINTS) {
      const on = depict(state, { ...defaultDepictionOptions("thermal", "en", DENOMINATOR, viewpoint, "light"), patterns: true }).legends[0]!;
      expect(on.convention.en).toContain("unhatched = within ±35 %");
      expect(on.convention.de).toContain("unschraffiert = innerhalb von ±35 %");
      const off = depict(state, { ...defaultDepictionOptions("thermal", "en", DENOMINATOR, viewpoint, "light"), patterns: false }).legends[0]!;
      expect(off.convention.en).toContain("Hatching is off");
      expect(off.convention.de).toContain("Schraffur ist aus");
      const field = solveTransport(jogger, { kind: "clip", clip: "run-cycle", timeS: 2.3 }, jogger.thermal.environment, { maxAgeS: 4 });
      const air = depict(evaluate(jogger, { kind: "clip", clip: "run-cycle", timeS: 2.3 }, select), { ...defaultDepictionOptions("light-solid", "en", DENOMINATOR, viewpoint), transport: ["parcels"] }, { transport: field }).legends.find((l) => l.quantity === "excess-temperature")!;
      expect(air.convention.en).toContain("unhatched = within ±35 %");
      expect(air.convention.de).toContain("unschraffiert = innerhalb von ±35 %");
    }
  });

  it("weights merged regions by area times exposed fraction", () => {
    const field = solveThermalAt(jogger, { kind: "pose", pose: jogger.presentation.defaultPose, timeS: 0 }, undefined);
    const ids = ["head-skin", "hair"];
    const w = (id: string) => jogger.thermal.regions.find((r) => r.id === id)!.areaM2 * Math.max(field.regions[id]!.exposedFraction, 1e-9);
    const expected = (w(ids[0]!) * field.regions[ids[0]!]!.surfaceTempC + w(ids[1]!) * field.regions[ids[1]!]!.surfaceTempC) / (w(ids[0]!) + w(ids[1]!));
    expect(areaWeighted(jogger.thermal.regions, field, ids, "surface-temperature")).toBeCloseTo(expected, 12);
  });

  it("places emitters by the 3D frame of a spread or twisted bone, consistently with the anchors", () => {
    const crane = assetOf("tower-crane");
    const query: EvaluationQuery = { kind: "pose", pose: "parked-weathervane", timeS: 0 };
    const state = evaluate(crane, query, select);
    const field = solveTransport(crane, query, crane.thermal.environment, { maxAgeS: 1 });
    expect(crane.thermal.emitters.length).toBeGreaterThan(0);
    for (const emitter of crane.thermal.emitters) {
      const placed = state.anchors[emitter.anchor]!;
      const out = field.emitters.find((e) => e.emitter === emitter.id)!;
      expect(out.at.x).toBeCloseTo(placed.at.x, 12);
      expect(out.at.y).toBeCloseTo(placed.at.y, 12);
      expect(out.at.z).toBeCloseTo(placed.z, 12);
      expect(Math.hypot(out.direction.x, out.direction.y, out.direction.z)).toBeCloseTo(1, 12);
    }
  });
});

describe("🌐️ transient thermal grid and start", () => {
  const jogger = assetOf("jogger");
  const base = jogger.thermal.environment;
  const ramp = { keys: [{ t: 0, environment: base, ease: "linear" as const }, { t: 1500, environment: { ...base, airTempC: base.airTempC + 6, solarWM2: 300 }, ease: "linear" as const }] };
  const times = [0.4, 7.3, 31.9, 250.6, 900.1, 2400.7];
  const buffer = new Float64Array(1);
  const bits = (field: ReturnType<typeof solveThermalAt>): string =>
    JSON.stringify(Object.entries(field.regions).map(([id, r]) => [id, ...[r.surfaceTempC, r.skinTempC ?? 0, r.sensibleW, r.latentW].map((v) => ((buffer[0] = v), Array.from(new Uint8Array(buffer.buffer)).join(",")))]));
  const query = (timeS: number): EvaluationQuery => ({ kind: "clip", clip: "run-cycle", timeS });

  it("gives bit-identical temperatures cold, played in order with a session and scrubbed in reverse with a session", () => {
    const cold = times.map((t) => bits(solveThermalAt(jogger, query(t), ramp)));
    const played = thermalSession();
    const forward = times.map((t) => bits(solveThermalAt(jogger, query(t), ramp, { session: played })));
    const scrubbed = thermalSession();
    const backward = [...times].reverse().map((t) => bits(solveThermalAt(jogger, query(t), ramp, { session: scrubbed }))).reverse();
    expect(forward).toEqual(cold);
    expect(backward).toEqual(cold);
  }, 300_000);

  it("plans the same grid and the same start for every query of one epoch, whatever its time", () => {
    const plans = [100, 101.3, 150, 199.9].map((t) => historyPlan(t, 300));
    expect(new Set(plans.map((p) => p.stepS)).size).toBe(1);
    expect(new Set(plans.map((p) => p.startS)).size).toBe(1);
    const step = plans[0]!.stepS;
    for (const p of plans) for (const time of p.times.slice(0, -1)) expect(time / step).toBeCloseTo(Math.round(time / step), 9);
    expect(historyPlan(5, 300).startS).toBe(0);
  });
});

describe("🌐️ numpy reference", () => {
  const ready = requireTool("uv");

  it("agrees with numpy on the wind drift, the three projections and the auto range", { timeout: 300_000 }, () => {
    const tree = scenario("tree");
    const drift = [0, 30, 90, 180].map((bearingDeg) => {
      const wind: Environment = { ...SCENARIOS.assets.tree.thermal.environment, windMS: 1.5, windDirectionDeg: bearingDeg };
      const field = solveTransport(tree.asset, { kind: "pose", pose: "summer", timeS: 9 }, wind, { ...tree.options, turbulence: { intensity: 0, floorMS: 1e-6, lagrangianTimeS: 1 }, maxAgeS: 5 });
      const line = [...field.pathlines].sort((a, b) => b.points.length - a.points.length)[0]!;
      return { bearingDeg, windMS: 1.5, x0: line.points[0]!.x, z0: line.points[0]!.z, agesS: line.ages, observed: line.points.map((p) => [p.x, p.z]) };
    });
    const jogger = assetOf("jogger");
    const query: EvaluationQuery = { kind: "clip", clip: "run-cycle", timeS: 2.3 };
    const field = solveTransport(jogger, query, jogger.thermal.environment, { maxAgeS: 4 });
    const k = 1000 / DENOMINATOR;
    const state = evaluate(jogger, query, select);
    const points = field.parcels.map((p) => [p.at.x, p.at.y, p.at.z]);
    const scenesByView = Object.fromEntries(VIEWPOINTS.map((viewpoint) => [viewpoint, depict(state, { ...defaultDepictionOptions("light-solid", "en", DENOMINATOR, viewpoint), transport: ["parcels"] }, { transport: field })])) as Record<Viewpoint, DrawingScene>;
    const thermal = solveThermalAt(jogger, query, undefined);
    const values = jogger.thermal.regions.map((r) => thermal.regions[r.id]!.surfaceTempC);
    const ranges = [{ centre: thermal.environment.airTempC, minSpan: 1, values }];
    const directory = mkdtempSync(join(tmpdir(), "staffage-multiview-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ drift, projection: { k, points }, ranges }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🌐️thermal-multiview/🐍️.py"), ["numpy"], [input])) as { drift: number[][][]; projection: Record<Viewpoint, number[][]>; ranges: number[][] };
      drift.forEach((d, i) => d.observed.forEach((p, j) => (expect(p[0]).toBeCloseTo(reference.drift[i]![j]![0]!, 4), expect(p[1]).toBeCloseTo(reference.drift[i]![j]![1]!, 4))));
      for (const viewpoint of VIEWPOINTS) {
        const scene = scenesByView[viewpoint];
        const items = itemsOf(scene).filter((i) => i.id.startsWith("parcel:"));
        items.forEach((item) => {
          const at = centre(item);
          const expected = reference.projection[viewpoint][parcelIndex(item)]!;
          expect(at.x - scene.metadata.origin.xMm).toBeCloseTo(expected[0]!, 5);
          expect(at.y - scene.metadata.origin.yMm).toBeCloseTo(expected[1]!, 5);
        });
      }
      const auto = thermalScenes(jogger, query, { mode: "auto" });
      for (const viewpoint of VIEWPOINTS) {
        const legend = auto[viewpoint].legends[0]!;
        expect(legend.min).toBeCloseTo(reference.ranges[0]![0]!, 9);
        expect(legend.max).toBeCloseTo(reference.ranges[0]![1]!, 9);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});
