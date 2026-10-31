/** 💨️ Transport on the real catalog assets: walking commuter (loop boundaries, play = scrub, head pose change), excavator (ignition, idle, rated), oak (transpiration only) and representation invariance. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Asset, Environment } from "../../🧬️schema/🟦️.ts";
import type { EvaluationQuery } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";

const load = (path: string): Asset => {
  const asset = (JSON.parse(readFileSync(new URL(`../../🗂️catalog/${path}/🔣️.json`, import.meta.url), "utf8")) as { asset: Asset }).asset;
  return asset.thermal.passengers === undefined ? { ...asset, thermal: { ...asset.thermal, passengers: [] } } : asset;
};
const commuter = load("👤️humans/walking-commuter");
const excavator = load("🏗️machinery/excavator");
const oak = load("🌳️vegetation/english-oak");
const cold: Environment = { airTempC: 2, meanRadiantTempC: 2, relativeHumidity: 0.9, windMS: 0.4, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
const walk = (t: number): EvaluationQuery => ({ kind: "clip", clip: "walk-cycle", timeS: t });
const json = (x: unknown): string => JSON.stringify(x);

describe("💨️ real assets", () => {
  it("evaluates the walking commuter play = scrub bit-identically across clip loops", () => {
    const times = [0.4, 1.19, 1.2, 1.21, 2.5, 3.7, 7.3];
    const played = times.map((t) => json(solveTransport(commuter, walk(t), cold)));
    const cold2 = structuredClone(commuter);
    const scrubbed = [...times].reverse().map((t) => json(solveTransport(cold2, walk(t), cold))).reverse();
    expect(scrubbed).toEqual(played);
  });

  it("keeps parcels continuous over the loop boundary of the 1.2 s walk cycle", () => {
    const h = 1 / 120;
    let compared = 0;
    for (const boundary of [1.2, 2.4, 3.6]) {
      const before = solveTransport(commuter, walk(boundary - h), cold, { streamlines: false });
      const after = solveTransport(commuter, walk(boundary + h), cold, { streamlines: false });
      for (const p of before.parcels) {
        const q = after.parcels.find((c) => c.id === p.id);
        if (q === undefined) continue;
        const speed = Math.hypot(q.velocity.x, q.velocity.y);
        expect(Math.hypot(q.at.x - p.at.x, q.at.y - p.at.y)).toBeLessThan(2 * (speed + 0.5) * 2 * h + 1e-3);
        compared++;
      }
    }
    expect(compared).toBeGreaterThan(60);
  });

  it("emits breath only from the nostrils or mouth of the head and never from the plume anchor", () => {
    const field = solveTransport(commuter, walk(6), cold, { streamlines: false });
    const emitters = new Set(field.parcels.map((p) => p.emitter));
    expect(emitters.has("nose-breath") || emitters.has("mouth-breath")).toBe(true);
    expect(field.parcels.filter((p) => p.emitter === "body-plume").every((p) => p.carries === "airflow")).toBe(true);
    const nose = field.emitters.find((e) => e.emitter === "nose-breath")!;
    const state = evaluate(commuter, walk(6));
    expect(nose.at).toEqual(state.anchors["nostrils"]!.at);
  });

  it("starts the excavator exhaust at ignition and scales it with the operating level", () => {
    const ramp: Asset = {
      ...excavator,
      clips: [{ ...excavator.clips.find((c) => c.id === "start-and-idle")!, id: "ramp", durationS: 14, tracks: [{ target: "operating:engine-load", ease: "linear", keys: [{ t: 0, v: 0 }, { t: 0.8, v: 0.5 }, { t: 1.5, v: 0.2 }, { t: 6, v: 0.2 }, { t: 8, v: 1 }, { t: 14, v: 1 }] }, { target: "emitter:exhaust-stack", ease: "step", keys: [{ t: 0, v: 0 }, { t: 0.8, v: 1 }] }] }],
    };
    const at = (t: number) => solveTransport(ramp, { kind: "clip", clip: "ramp", timeS: t }, cold, { streamlines: false });
    expect(at(0.6).parcels.length).toBe(0);
    const bornEarliest = Math.min(...at(3).parcels.map((p) => p.bornS));
    expect(bornEarliest).toBeGreaterThanOrEqual(0.8 - 1e-9);
    const idle = at(5).emitters[0]!;
    const rated = at(10).emitters[0]!;
    expect(rated.volumeFlowM3S / idle.volumeFlowM3S).toBeCloseTo(5, 6);
    expect(rated.temperatureC).toBeGreaterThan(idle.temperatureC);
    expect(at(10).parcels.every((p) => p.carries === "airflow")).toBe(true);
  });

  it("gives the oak distributed vapour-only transpiration and never a breathing effect", () => {
    expect(oak.thermal.respiration).toBeUndefined();
    const field = solveTransport(oak, { kind: "clip", clip: "wind-sway", timeS: 12 }, oak.thermal.environment, { streamlines: false });
    expect(field.respiration).toBeNull();
    expect(field.parcels.length).toBeGreaterThan(20);
    expect(field.parcels.every((p) => p.carries === "vapour")).toBe(true);
    expect(new Set(field.parcels.map((p) => p.id.split("#")[0])).size).toBeGreaterThanOrEqual(8);
    expect(field.condensation.length).toBe(0);
  });

  it("is independent of the representation the state is evaluated with", () => {
    const sel = (d: number) => ({ evaluate: (a: Asset, q: EvaluationQuery) => evaluate(a, q, { kind: "scale", scale: { denominator: d } }) });
    const a = json(solveTransport(commuter, walk(5), cold, sel(500)));
    const b = json(solveTransport(structuredClone(commuter), walk(5), cold, sel(50)));
    const c = json(solveTransport(structuredClone(commuter), walk(5), cold));
    expect(a).toEqual(b);
    expect(a).toEqual(c);
  });
});
