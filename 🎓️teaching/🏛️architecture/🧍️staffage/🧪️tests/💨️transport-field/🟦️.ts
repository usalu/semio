/** 💨️ Transport field: world-space parcels (head turn, moving source, wind change), play = scrub bitwise, no teleport, category sources (breath, exhaust, plume, transpiration), streamlines vs pathlines, condensation, cancellation. */
import { describe, expect, it } from "vitest";
import { SCENARIOS, scenario } from "../../🧫️fixtures/💨️transport/🟦️.ts";
import type { EvaluationQuery, TransportField } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { createTrack } from "../../🔨️modules/💨️transport/🧮️parcels/🟦️.ts";
import { PLUME_SOURCE_EXCESS_K, emissionsAt, STEPS_PER_S } from "../../🔨️modules/💨️transport/🔥️sources/🟦️.ts";
import { moistFrom, enthalpy, volumePerDryKg } from "../../🔨️modules/💨️transport/🌫️air/🟦️.ts";
import { respirationState } from "../../🔨️modules/💨️transport/🫁️respiration/🟦️.ts";
import { DEFAULT_TURBULENCE } from "../../🔨️modules/💨️transport/🟦️.ts";

const calm = SCENARIOS.environments["calm-cold"];
const rest = (t: number): EvaluationQuery => ({ kind: "pose", pose: "rest", timeS: t });
const walk = (t: number): EvaluationQuery => ({ kind: "clip", clip: "walk", timeS: t });
const angleOf = (a: { x: number; y: number }, b: { x: number; y: number }) => (Math.atan2(b.y - a.y, b.x - a.x) * 180) / Math.PI;

function bits(value: unknown): string {
  const buffer = new Float64Array(1);
  const words: string[] = [];
  const walkValue = (v: unknown): void => {
    if (typeof v === "number") {
      buffer[0] = v;
      words.push(Array.from(new Uint8Array(buffer.buffer)).join(","));
    } else if (Array.isArray(v)) v.forEach(walkValue);
    else if (v !== null && typeof v === "object") for (const key of Object.keys(v).sort()) (words.push(key), walkValue((v as Record<string, unknown>)[key]));
    else words.push(String(v));
  };
  walkValue(value);
  return words.join("|");
}

const headTurn = { angleKeysDeg: [[0, 0], [3, 0], [4, 90], [10, 90]] as [number, number][] };
const walking = { rootX: { x0: 0, vx: 1.3 } };

describe("💨️ world-space parcels", () => {
  it("keeps emitted parcels fixed in the world when the head turns and rotates only new emission", () => {
    const s = scenario("human", headTurn);
    const early = solveTransport(s.asset, rest(3.5), calm, s.options);
    const late = solveTransport(s.asset, rest(6), calm, s.options);
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
    const direction = (t: number) => solveTransport(s.asset, rest(t), calm, s.options).emitters.find((e) => e.emitter === "nose")!.directionRad;
    expect(((direction(6) - direction(2)) * 180) / Math.PI).toBeCloseTo(90, 9);
  });

  it("emits along the head direction of the birth instant, not the current one", () => {
    const s = scenario("human", headTurn);
    const field = solveTransport(s.asset, rest(9.3), calm, s.options);
    const first = (born: number) => {
      const p = field.parcels.filter((q) => q.emitter === "nose").sort((a, b) => Math.abs(a.bornS - born) - Math.abs(b.bornS - born))[0]!;
      const line = field.pathlines.find((l) => l.parcel === p.id)!;
      return { bornS: p.bornS, angle: angleOf(line.points[0]!, line.points[1]!) };
    };
    const old = first(2.4);
    const fresh = first(7.6);
    expect(old.bornS).toBeLessThan(3);
    expect(fresh.bornS).toBeGreaterThan(7);
    expect(old.angle).toBeGreaterThan(-90);
    expect(old.angle).toBeLessThan(-35);
    expect(fresh.angle).toBeGreaterThan(5);
    expect(fresh.angle).toBeLessThan(55);
  });

  it("leaves a world-fixed trail behind a moving source across clip loops", () => {
    const s = scenario("human", walking);
    const field = solveTransport(s.asset, walk(9), calm, s.options);
    const nose = field.parcels.filter((p) => p.emitter === "nose" || p.emitter === "mouth");
    expect(nose.length).toBeGreaterThan(30);
    const births = nose.map((p) => ({ born: p.bornS, x0: field.pathlines.find((l) => l.parcel === p.id)!.points[0]!.x }));
    expect(Math.max(...births.map((b) => b.born))).toBeGreaterThan(2.4);
    for (const b of births) expect(b.x0).toBeCloseTo(1.3 * b.born + 0.09, 9);
    const span = Math.max(...births.map((b) => b.x0)) - Math.min(...births.map((b) => b.x0));
    expect(span).toBeGreaterThan(5);
    const oldest = nose.sort((a, b) => b.ageS - a.ageS)[0]!;
    const home = 1.3 * oldest.bornS + 0.09;
    expect(Math.abs(oldest.at.x - home)).toBeLessThan(1.5);
    expect(1.3 * 9 + 0.09 - oldest.at.x).toBeGreaterThan(4);
  });

  it("bends only subsequent motion when the wind of the timeline changes", () => {
    const s = scenario("human");
    const gust = solveTransport(s.asset, rest(9), SCENARIOS.environments.gust, s.options);
    const still = solveTransport(s.asset, rest(9), calm, s.options);
    let before = 0;
    let after = 0;
    for (const line of gust.pathlines) {
      const other = still.pathlines.find((l) => l.parcel === line.parcel);
      if (other === undefined) continue;
      const p = gust.parcels.find((q) => q.id === line.parcel)!;
      line.points.forEach((pt, i) => {
        const t = p.bornS + (line.ages[i] ?? 0);
        if (t < 5.95 && i < other.points.length - 1) {
          expect(pt).toEqual(other.points[i]);
          before++;
        }
      });
      if (p.ageS > 2) {
        expect(line.points[line.points.length - 1]!.x).toBeGreaterThan(other.points[other.points.length - 1]!.x);
        after++;
      }
    }
    expect(before).toBeGreaterThan(30);
    expect(after).toBeGreaterThan(3);
  });

  it("is bitwise identical whether played at any frame rate, scrubbed, or evaluated cold", () => {
    const s = scenario("human", { ...headTurn, ...walking });
    const times = [1.55, 2.5, 3.0, 4.2, 5.04, 6.4];
    const reference = new Map(times.map((t) => [t, bits(solveTransport(s.asset, walk(t), calm, scenario("human", { ...headTurn, ...walking }).options))]));
    for (const fps of [24, 25, 30, 60]) {
      const played = scenario("human", { ...headTurn, ...walking });
      for (let i = 0; i <= 7 * fps; i++) {
        const t = i / fps;
        const field = solveTransport(played.asset, walk(t), calm, played.options);
        const hit = times.find((x) => Math.abs(x - t) < 1e-12);
        if (hit !== undefined) expect(bits(field)).toBe(reference.get(hit));
      }
      for (const t of [...times].reverse()) expect(bits(solveTransport(played.asset, walk(t), calm, played.options))).toBe(reference.get(t));
    }
  });

  it("never teleports: per-step displacement is bounded by speed and the turbulent step", () => {
    const s = scenario("human", { ...headTurn, ...walking });
    const options = { ...s.options, turbulence: { intensity: 0, floorMS: 1e-3, lagrangianTimeS: 1 }, pathlines: false, streamlines: false };
    let previous = new Map(solveTransport(s.asset, walk(1.1), calm, options).parcels.map((p) => [p.id, p]));
    let worst = 0;
    for (let n = 1; n <= 160; n++) {
      const field = solveTransport(s.asset, walk(1.1 + n / STEPS_PER_S), calm, options);
      const next = new Map(field.parcels.map((p) => [p.id, p]));
      for (const [id, p] of next) {
        const q = previous.get(id);
        if (q === undefined) continue;
        const speed = Math.max(Math.hypot(p.velocity.x, p.velocity.y), Math.hypot(q.velocity.x, q.velocity.y));
        const step = Math.hypot(p.at.x - q.at.x, p.at.y - q.at.y);
        worst = Math.max(worst, step / (speed / STEPS_PER_S + 1e-4));
      }
      previous = next;
    }
    expect(worst).toBeLessThan(1.5);
  });

  it("is deterministic in the seed and differs between seeds", () => {
    const s = scenario("human");
    const a = solveTransport(s.asset, rest(4), calm, { ...s.options, seed: 7 });
    const b = solveTransport(s.asset, rest(4), calm, { ...s.options, seed: 7 });
    const c = solveTransport(s.asset, rest(4), calm, { ...s.options, seed: 8 });
    expect(bits(a)).toBe(bits(b));
    expect(bits(a)).not.toBe(bits(c));
  });
});

describe("💨️ category sources", () => {
  it("emits breath only while exhaling, nose for the nose pattern, the mouth for the mouth pattern", () => {
    const s = scenario("human");
    const field = solveTransport(s.asset, rest(9.9), calm, s.options);
    const breath = field.parcels.filter((p) => p.emitter === "nose" || p.emitter === "mouth");
    expect(breath.length).toBeGreaterThan(0);
    expect(new Set(breath.map((p) => p.emitter))).toEqual(new Set(["nose"]));
    for (const p of breath) {
      const before = respirationState(s.asset, rest(p.bornS - 1 / 60))!;
      const after = respirationState(s.asset, rest(p.bornS + 1 / 60))!;
      expect(before.flowM3S > 0 || after.flowM3S > 0).toBe(true);
    }
    const active = { kind: "pose", pose: "active", timeS: 4 } as const;
    expect(new Set(solveTransport(s.asset, active, calm, s.options).parcels.filter((p) => p.emitter !== "plume").map((p) => p.emitter))).toEqual(new Set(["mouth"]));
  });

  it("releases exactly the exhaled tidal volume per breath through the emitters", () => {
    const s = scenario("human");
    const track = createTrack(s.asset, rest(0), calm, { maxAgeS: 30, seed: 1, layer: null, turbulence: DEFAULT_TURBULENCE }, s.options.evaluate, s.options.thermal);
    const volume = (emitter: number) => {
      let v = 0;
      for (let k = 0; k < 150; k++) for (const e of emissionsAt(track.world, emitter, k)) v += e.md * volumePerDryKg(moistFrom(e.h, e.wt, 101325).tC, e.wt, 101325);
      return v * 1000;
    };
    expect(volume(0)).toBeCloseTo(0.5, 9);
    expect(volume(1)).toBeCloseTo(0, 12);
  });

  it("follows the operating level of an exhaust through ignition and shutdown", () => {
    const s = scenario("machine");
    const state = (t: number) => solveTransport(s.asset, { kind: "pose", pose: "idle", timeS: t }, calm, s.options);
    expect(state(1).parcels.length).toBe(0);
    expect(state(1).emitters[0]!.active).toBe(false);
    expect(state(2.5).emitters[0]!.volumeFlowM3S).toBeCloseTo(0.025, 12);
    expect(state(2.5).emitters[0]!.temperatureC).toBeCloseTo(5 + 75, 9);
    expect(state(5).emitters[0]!.volumeFlowM3S).toBeCloseTo(0.05, 12);
    expect(state(5).emitters[0]!.exitVelocityMS).toBeCloseTo(0.05 / (Math.PI * 0.025 ** 2), 9);
    const field = state(12);
    expect(field.emitters[0]!.active).toBe(false);
    const born = field.parcels.map((p) => p.bornS);
    expect(Math.min(...born)).toBeGreaterThan(2);
    expect(Math.max(...born)).toBeLessThan(9.1);
    expect(field.parcels.every((p) => p.carries === "airflow")).toBe(true);
  });

  it("releases the solved sensible heat of the plume region as a buoyant stream", () => {
    const s = scenario("human");
    const track = createTrack(s.asset, rest(0), calm, { maxAgeS: 30, seed: 1, layer: null, turbulence: DEFAULT_TURBULENCE }, s.options.evaluate, s.options.thermal);
    let heat = 0;
    const emissions = Array.from({ length: 50 }, (_, k) => emissionsAt(track.world, 2, k)).flat();
    for (const e of emissions) heat += e.md * (e.h - enthalpy(5, track.world.ambientAt(1).wv, 0));
    expect(heat / ((50 * 12) / STEPS_PER_S)).toBeCloseTo(60, -0.5);
    expect(PLUME_SOURCE_EXCESS_K).toBe(2);
    const field = solveTransport(s.asset, rest(8), calm, s.options);
    const plume = field.parcels.filter((p) => p.emitter === "plume");
    expect(plume.length).toBeGreaterThan(30);
    const rising = plume.filter((p) => p.ageS > 2);
    expect(rising.every((p) => p.velocity.y > 0)).toBe(true);
    expect(Math.max(...plume.map((p) => p.at.y))).toBeGreaterThan(2.2);
  });

  it("releases transpiration as vapour only, distributed over the crown anchors, with the latent flux of the region", () => {
    const s = scenario("tree");
    const env = { ...SCENARIOS.assets.tree.thermal.environment };
    const field = solveTransport(s.asset, { kind: "pose", pose: "summer", timeS: 12 }, env, s.options);
    expect(field.parcels.length).toBeGreaterThan(20);
    expect(field.parcels.every((p) => p.carries === "vapour")).toBe(true);
    expect(field.emitters[0]!.exitVelocityMS).toBe(0);
    for (const p of field.parcels) expect(Math.abs(p.excessK)).toBeLessThan(1e-6);
    const starts = new Set(field.pathlines.map((l) => l.points[0]!.x.toFixed(6)));
    expect(starts.size).toBe(6);
    expect(Math.min(...[...starts].map(Number))).toBeCloseTo(-1.5, 9);
    expect(Math.max(...[...starts].map(Number))).toBeCloseTo(1.5, 9);
    const track = createTrack(s.asset, { kind: "pose", pose: "summer" }, env, { maxAgeS: 30, seed: 1, layer: null, turbulence: DEFAULT_TURBULENCE }, s.options.evaluate, s.options.thermal);
    const wa = track.world.ambientAt(1).wv;
    let vapour = 0;
    for (let k = 0; k < 20; k++) for (const e of emissionsAt(track.world, 0, k)) vapour += e.md * (e.wt - wa);
    const lambda = 2.501e6 - 2361 * 22;
    expect(vapour / (20 * 0.5)).toBeCloseTo(150 / lambda, 8);
  });

  it("models visible condensation only when the mixture is supersaturated", () => {
    const s = scenario("human");
    const cold = solveTransport(s.asset, rest(2.6), calm, s.options);
    expect(cold.condensation.length).toBeGreaterThan(3);
    expect(cold.condensation.length).toBe(cold.parcels.filter((p) => p.condensateKgPerKg > 0).length);
    for (const c of cold.condensation) expect(c.condensateKgPerKg).toBeGreaterThan(0);
    const warm = solveTransport(s.asset, rest(2.6), { ...calm, airTempC: 30, relativeHumidity: 0.5 }, s.options);
    expect(warm.condensation.length).toBe(0);
  });
});

describe("💨️ streamlines are not pathlines", () => {
  it("traces straight horizontal lines at the wind speed where only wind acts", () => {
    const s = scenario("tree");
    const field = solveTransport(s.asset, { kind: "pose", pose: "summer", timeS: 0 }, { ...SCENARIOS.assets.tree.thermal.environment, windMS: 2, windDirectionDeg: 0 }, { ...s.options, pathlines: false });
    expect(field.streamlines.length).toBeGreaterThan(3);
    for (const line of field.streamlines) {
      for (const p of line.points) expect(p.y).toBeCloseTo(line.seed.y, 12);
      for (const v of line.speedMS) expect(v).toBeCloseTo(2, 12);
      for (let i = 1; i < line.points.length; i++) expect(line.points[i]!.x - line.points[i - 1]!.x).toBeCloseTo(0.02, 12);
    }
  });

  it("differs from the pathline of a parcel after a wind change", () => {
    const s = scenario("machine");
    const field = solveTransport(s.asset, { kind: "pose", pose: "idle", timeS: 9.4 }, SCENARIOS.environments.gust, s.options);
    const line = field.pathlines.filter((l) => l.points.length > 20).sort((a, b) => b.points.length - a.points.length)[0]!;
    const first = angleOf(line.points[0]!, line.points[2]!);
    const last = angleOf(line.points[line.points.length - 3]!, line.points[line.points.length - 1]!);
    expect(Math.abs(first - last)).toBeGreaterThan(10);
    expect(field.streamlines.length).toBeGreaterThan(0);
    expect(field.streamlines.some((l) => l.points.length > 10)).toBe(true);
  });

  it("draws streamlines into the nostril during inhalation", () => {
    const s = scenario("human");
    const bare = { ...s.asset, thermal: { ...s.asset.thermal, emitters: s.asset.thermal.emitters.filter((e) => e.kind === "breath") } };
    const field = solveTransport(bare, rest(0.8), calm, { ...s.options, pathlines: false });
    expect(field.respiration!.phase).toBe("inhale");
    const nose = field.emitters.find((e) => e.emitter === "nose")!;
    const near = field.streamlines.filter((l) => Math.hypot(l.seed.x - nose.at.x, l.seed.y - nose.at.y) < 0.05);
    expect(near.length).toBe(4);
    for (const line of near) {
      const distance = (p: { x: number; y: number }) => Math.hypot(p.x - nose.at.x, p.y - nose.at.y);
      expect(distance(line.points[line.points.length - 1]!)).toBeLessThan(distance(line.seed) + 1e-12);
      expect(distance(line.points[0]!)).toBeGreaterThan(distance(line.seed));
    }
  });
});

describe("💨️ job control", () => {
  it("reports progress up to the total and cancels on an aborted signal", () => {
    const s = scenario("human");
    const seen: [number, number][] = [];
    solveTransport(s.asset, rest(5), calm, { ...s.options, progress: (d, t) => seen.push([d, t]) });
    expect(seen.length).toBeGreaterThan(2);
    expect(seen[seen.length - 1]![0]).toBe(seen[seen.length - 1]![1]);
    const controller = new AbortController();
    controller.abort();
    expect(() => solveTransport(s.asset, rest(5), calm, { ...s.options, signal: controller.signal })).toThrow(/cancelled/);
  });
});

export type { TransportField };

describe("💨️ passengers", () => {
  it("releases the passenger's breath from its own emitter with its own tidal volume, independent of the carrier's", () => {
    const s = scenario("carrier");
    const field = solveTransport(s.asset, { kind: "pose", pose: "rest", timeS: 9.7 }, calm, s.options);
    const own = new Set(field.parcels.map((p) => p.emitter));
    expect(own.has("toddler-nose")).toBe(true);
    expect(own.has("nose")).toBe(true);
    const track = createTrack(s.asset, rest(0), calm, { maxAgeS: 30, seed: 1, layer: null, turbulence: DEFAULT_TURBULENCE }, s.options.evaluate, s.options.thermal);
    let v = 0;
    for (let k = 0; k < 450; k++) for (const e of emissionsAt(track.world, 3, k)) v += e.md * volumePerDryKg(moistFrom(e.h, e.wt, 101325).tC, e.wt, 101325);
    expect(v * 1000).toBeCloseTo(0.12 * 6, 9);
  });
});
