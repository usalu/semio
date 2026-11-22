/** 🌊️ Flow field: the Eulerian field reconstructed from parcels matches the numpy re-implementation, mixes overlapping streams instead of summing their temperatures (two jets through one outlet carry the temperature of one and twice its velocity), carries the volume flux of the similarity plume, draws ambient air in like the continuous line sink (scipy quad), lets no air through the ground, and its streams and tracers match scipy's integration of the same field in arc length and in time. */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🌊️flow-field/🔣️.json" with { type: "json" };
import type { Emitter, Environment } from "../../🧬️schema/🟦️.ts";
import type { Parcel, Vec3 } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { ambientAt } from "../../🔨️modules/💨️transport/🌬️ambient/🟦️.ts";
import { CENTRELINE_SHARE, flowSampler, streamAt, streamFrom } from "../../🔨️modules/💨️transport/🌊️flow/🟦️.ts";
import { ALPHA_PLUME } from "../../🔨️modules/💨️transport/🧮️parcels/🟦️.ts";

type Sample = { at: number[]; u: number[]; inducedMS: number; excessK: number; vapourExcessKgPerKg: number };
type Stream = { seed: number[]; points: number[][]; travelS: number[]; advected: { timeS: number; at: number[] }[] };
type Expected = { samples: Sample[]; lineSink?: { at: number[]; radialInflowMS: number }[]; flux?: { heightM: number; sampledM3S: number; similarityM3S: number; inducedM3S: number }[]; streams: Stream[] };
const expected = fixture.expected as unknown as Record<string, Expected>;
const tolerance = fixture.tolerances;
const vec = (p: readonly number[]): Vec3 => ({ x: p[0]!, y: p[1]!, z: p[2]! });
const ambient = ambientAt(fixture.ambient as unknown as Environment, 0, null, fixture.turbulence);
const samplerOf = (id: string) => flowSampler(fixture.cases.find((c) => c.id === id)!.parcels as unknown as Parcel[], fixture.emitters as unknown as Emitter[], ambient, []);
const EVERYWHERE = { min: { x: -50, y: 0, z: -50 }, max: { x: 50, y: 50, z: 50 } };

describe("🌊️ flow field", () => {
  it.each(fixture.cases.map((c) => c.id))("matches the numpy field of the %s at every sample point", (id) => {
    const sample = samplerOf(id);
    for (const row of expected[id]!.samples) {
      const got = sample(vec(row.at));
      expect(Math.abs(got.u.x - row.u[0]!)).toBeLessThan(tolerance.velocityMS);
      expect(Math.abs(got.u.y - row.u[1]!)).toBeLessThan(tolerance.velocityMS);
      expect(Math.abs(got.u.z - row.u[2]!)).toBeLessThan(tolerance.velocityMS);
      expect(Math.abs(got.inducedMS - row.inducedMS)).toBeLessThan(tolerance.velocityMS);
      expect(Math.abs(got.excessK - row.excessK)).toBeLessThan(tolerance.scalar);
      expect(Math.abs(got.vapourExcessKgPerKg - row.vapourExcessKgPerKg)).toBeLessThan(tolerance.scalar);
    }
  });

  it("gives two jets through one outlet the temperature of one and twice its velocity", () => {
    const twin = samplerOf("twin");
    const jet = samplerOf("jet");
    const pair = fixture.cases.find((c) => c.id === "twin")!;
    const hottest = Math.max(...(pair.parcels as unknown as Parcel[]).map((p) => p.excessK));
    for (const at of (pair as unknown as { axisPoints: number[][] }).axisPoints) {
      const both = twin(vec(at));
      const one = jet(vec(at));
      expect(one.excessK).toBeGreaterThan(1);
      expect(Math.abs(both.excessK - one.excessK)).toBeLessThan(1e-9);
      expect(Math.abs(both.vapourExcessKgPerKg - one.vapourExcessKgPerKg)).toBeLessThan(1e-12);
    }
    for (const at of pair.points) {
      const both = twin(vec(at));
      const one = jet(vec(at));
      expect(Math.hypot(both.u.x - 2 * one.u.x, both.u.y - 2 * one.u.y, both.u.z - 2 * one.u.z)).toBeLessThan(1e-9);
      expect(both.excessK).toBeLessThanOrEqual(CENTRELINE_SHARE * hottest);
      expect(both.excessK).toBeGreaterThanOrEqual(one.excessK - 1e-9);
      expect(both.excessK).toBeLessThanOrEqual(2 * one.excessK + 1e-9);
    }
    const edge = vec([2, 1.2, 0.5]);
    expect(Math.abs(twin(edge).excessK - 2 * jet(edge).excessK)).toBeLessThan(1e-9);
  });

  it("draws ambient air into the plume like the continuous line sink with its ground image", () => {
    const sample = samplerOf("plume");
    for (const row of expected.plume!.lineSink!) {
      const u = sample(vec(row.at)).u;
      const r = Math.hypot(row.at[0]!, row.at[2]!);
      const inflow = -(u.x * row.at[0]! + u.z * row.at[2]!) / r;
      expect(inflow).toBeGreaterThan(0);
      expect(Math.abs(inflow / row.radialInflowMS - 1)).toBeLessThan(tolerance.lineSinkRelative);
    }
  });

  it("carries the volume flux of the similarity plume, plus what the entrainment sinks drive through the same disc, at every height", () => {
    const sample = samplerOf("plume");
    for (const row of expected.plume!.flux!) {
      const reach = (4 * 6 * ALPHA_PLUME * row.heightM) / 5;
      let flux = 0;
      for (let i = 1; i <= 100; i++) {
        const r0 = (reach * (i - 1)) / 100;
        const r1 = (reach * i) / 100;
        flux += ((sample({ x: r0, y: row.heightM, z: 0 }).u.y * 2 * Math.PI * r0 + sample({ x: r1, y: row.heightM, z: 0 }).u.y * 2 * Math.PI * r1) / 2) * (r1 - r0);
      }
      expect(Math.abs(flux / row.sampledM3S - 1)).toBeLessThan(1e-6);
      expect(Math.abs(flux / (row.similarityM3S + row.inducedM3S) - 1)).toBeLessThan(tolerance.fluxRelative);
    }
  });

  it("lets no air through the ground", () => {
    for (const id of fixture.cases.map((c) => c.id)) {
      const sample = samplerOf(id);
      for (const x of [-2, -0.3, 0.05, 0.4, 1.7]) for (const z of [-0.6, 0, 0.9]) expect(Math.abs(sample({ x, y: 0, z }).u.y)).toBeLessThan(1e-12);
    }
  });

  it.each(fixture.cases.map((c) => c.id))("traces the streams of the %s like solve_ivp and moves tracers like particles", (id) => {
    const sample = samplerOf(id);
    let advected = 0;
    for (const row of expected[id]!.streams) {
      const stream = streamFrom(sample, { at: vec(row.seed), role: "ambient", emitter: null }, fixture.stream.stepM, fixture.stream.points, EVERYWHERE)!;
      expect(stream).not.toBeNull();
      const at = stream.points.findIndex((p) => p.x === row.seed[0] && p.y === row.seed[1] && p.z === row.seed[2]);
      expect(at).toBe(0);
      const count = Math.min(fixture.stream.points, stream.points.length - 1 - at);
      expect(count).toBeGreaterThan(10);
      for (const i of [0, Math.floor(count / 2), count - 1]) {
        const p = stream.points[at + 1 + i]!;
        expect(Math.hypot(p.x - row.points[i]![0]!, p.y - row.points[i]![1]!, p.z - row.points[i]![2]!)).toBeLessThan(tolerance.streamM);
        expect(Math.abs((stream.travelS[at + 1 + i]! - stream.travelS[at]!) / row.travelS[i]! - 1)).toBeLessThan(tolerance.travelRelative);
      }
      const reachS = stream.travelS[at + count]! - stream.travelS[at]!;
      for (const moved of row.advected.filter((m) => m.timeS < reachS)) {
        const here = streamAt(stream, stream.travelS[at]! + moved.timeS).at;
        expect(Math.hypot(here.x - moved.at[0]!, here.y - moved.at[1]!, here.z - moved.at[2]!)).toBeLessThan(tolerance.advectedM);
        advected++;
      }
    }
    expect(advected).toBeGreaterThan(0);
  });
});
