/** 🧮️ Properties of the evaluated state: determinism, shape flattening, load conversion, scales, bounds and kinematics.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧮️evaluation-state/🔣️.json — the vectors
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { clipOf, evaluate, poseOf } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const fixture = read("../../🧫️fixtures/🧮️evaluation-state/🔣️.json");
const asset = read("../../🧫️fixtures/🚶️reference-walker/🔣️.json") as Asset;

describe("🧮️ evaluation state", () => {
  it("is bit-for-bit deterministic", () => {
    for (const query of [{ kind: "pose", pose: "stand" }, { kind: "clip", clip: "walk", timeS: 0.337 }, { kind: "transition", from: "stand", to: "carry-bag", durationS: 0.8, timeS: 0.3 }] as const) {
      const strip = (state: ReturnType<typeof evaluate>) => JSON.stringify({ ...state, asset: undefined });
      expect(strip(evaluate(asset, query))).toBe(strip(evaluate(asset, query)));
    }
  });

  it("flattens capsules and ellipses finely enough and keeps authored order", () => {
    const state = evaluate(asset, { kind: "pose", pose: "stand" });
    state.shapes.elevation.forEach((evaluated, index) => {
      expect(evaluated.shape.id).toBe(asset.geometry.elevation[index]!.id);
      const kind = evaluated.shape.kind;
      if (kind === "capsule") expect(evaluated.rings[0]!.length).toBeGreaterThanOrEqual(fixture.ringMinimums.capsule);
      if (kind === "ellipse") expect(evaluated.rings[0]!.length).toBeGreaterThanOrEqual(fixture.ringMinimums.ellipse);
      expect(evaluated.closed).toBe(evaluated.shape.kind === "poly" ? evaluated.shape.closed : true);
    });
    expect(state.shapes.plan.length).toBe(asset.geometry.plan!.length);
  });

  it("stands the pose on the ground and reports bounds that match the nominal dimensions", () => {
    const state = evaluate(asset, { kind: "pose", pose: fixture.boundsPose });
    expect(state.bounds.minY).toBeLessThan(0.005);
    expect(state.bounds.minY).toBeGreaterThan(-0.05);
    expect(state.bounds.maxY - state.bounds.minY).toBeCloseTo(asset.dimensions.heightM, 1);
    expect(state.frame).toBe(0);
    expect(state.kinematics.comVelocity).toEqual({ x: 0, y: 0 });
  });

  it("converts a distributed load to eight exact resultants", () => {
    const { pose, load, expected } = fixture.distributedLoad;
    const patched: Asset = { ...asset, poses: asset.poses.map((p) => (p.id === pose ? { ...p, loads: [load] } : p)) };
    const state = evaluate(patched, { kind: "pose", pose });
    const forces = state.loads.filter((f) => f.id.startsWith(`${load.id}#`));
    expect(forces.length).toBe(expected.count);
    expect(forces.reduce((s, f) => s + f.fxN, 0)).toBeCloseTo(expected.totalFxN, 9);
    expect(forces.reduce((s, f) => s + f.fyN, 0)).toBeCloseTo(expected.totalFyN, 9);
    const origin = state.bones.torso!.origin;
    const tip = state.bones.torso!.tip;
    const along = ((tip.x - origin.x) * 0 + (tip.y - origin.y)) / Math.hypot(tip.x - origin.x, tip.y - origin.y);
    const total = forces.reduce((s, f) => s + Math.hypot(f.fxN, f.fyN), 0);
    const centroid = forces.reduce((s, f) => s + Math.hypot(f.fxN, f.fyN) * Math.hypot(f.at.x - origin.x, f.at.y - origin.y), 0) / total;
    expect(along).toBeCloseTo(1, 6);
    expect(centroid).toBeCloseTo(expected.centroidAlongBoneM, 9);
  });

  it("scales loads by their track", () => {
    const { clip, timeS, load, fyN } = fixture.loadScale;
    const state = evaluate(asset, { kind: "clip", clip, timeS });
    expect(state.loads.find((f) => f.id === load)!.fyN).toBeCloseTo(fyN, 3);
    expect(state.loads.find((f) => f.id === load)!.at).toEqual(state.anchors["bag-strap"]!.at);
  });

  it("averages the centre-of-mass velocity of a planted loop to the advance per second", () => {
    const { clip, tolerance } = fixture.periodicMeanVelocity;
    const c = clipOf(asset, clip);
    let sum = 0;
    const n = 240;
    for (let i = 0; i < n; i++) sum += evaluate(asset, { kind: "clip", clip, timeS: (i / n) * c.durationS + 0.0007 }).kinematics.comVelocity.x;
    expect(sum / n).toBeCloseTo(c.advanceM! / c.durationS, 1);
    expect(Math.abs(sum / n - c.advanceM! / c.durationS)).toBeLessThan(tolerance * 10);
  });

  it("reports unknown ids with the available ones", () => {
    expect(() => poseOf(asset, fixture.unknown.pose)).toThrow(/reference-walker.*stand/);
    expect(() => evaluate(asset, { kind: "clip", clip: fixture.unknown.clip, timeS: 0 })).toThrow(/walk, idle-breathing/);
  });
});
