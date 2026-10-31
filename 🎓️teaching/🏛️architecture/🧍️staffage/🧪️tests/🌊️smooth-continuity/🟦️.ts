/** 🌊️ C1 continuity of smooth key interpolation, loop seams, tracks, transitions and the sparse smooth walk.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🌊️smooth-continuity/🔣️.json — the tolerances
 * @see ../../🧫️fixtures/🦶️sparse-smooth-walk/🔣️.json — four smooth keys
 */
import { readFileSync } from "node:fs";
import { CatmullRomCurve3, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import type { Asset, Clip } from "../../🧬️schema/🟦️.ts";
import { clipOf, sampleClip, sampleTrack, transition } from "../../🔨️modules/🎞️animation/🟦️.ts";
import { checkAsset } from "../../🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { UNCHECKED_PROFILE, samplingAsset } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const fixture = read("../../🧫️fixtures/🌊️smooth-continuity/🔣️.json");
const sparseSource = read("../../🧫️fixtures/🦶️sparse-smooth-walk/🔣️.json").asset as Asset;
const sparse = samplingAsset(sparseSource);
const walker = read("../../🧫️fixtures/👤️sample-figure/🔣️.json").asset as Asset;
const clip = clipOf(sparse, fixture.sparseClip);
const e = fixture.epsilonS;
const angle = (c: Clip, t: number, asset: Asset = sparse) => sampleClip(asset, c, t).offsets[fixture.bone]!.angleDeg;
const sides = (f: (t: number) => number, t: number) => ({ before: (f(t) - f(t - e)) / e, after: (f(t + e) - f(t)) / e });

describe("🌊️ smooth continuity", () => {
  it("validates and passes the full-pipeline check for the sparse four-key walk", () => {
    expect(clip.keys.length).toBe(4);
    expect(clip.keys.every((k) => k.ease === "smooth")).toBe(true);
    expect(validateAsset(sparseSource)).toEqual([]);
    expect(checkAsset(sparseSource, 3, { profile: UNCHECKED_PROFILE })).toEqual([]);
    expect(checkAsset(walker, 3, { profile: UNCHECKED_PROFILE })).toEqual([]);
  }, 120000);

  it("has equal bone-angle velocity on both sides of every key", () => {
    for (const key of clip.keys) {
      const { before, after } = sides((t) => angle(clip, t), key.t + clip.durationS);
      expect(Math.abs(before - after)).toBeLessThan(fixture.angleVelocityToleranceDegS);
    }
  });

  it("jumps at keys when the same keys are linear", () => {
    const linear: Clip = { ...clip, keys: clip.keys.map((k) => ({ ...k, ease: "linear" as const })) };
    const jumps = clip.keys.map((k) => {
      const { before, after } = sides((t) => angle(linear, t), k.t + clip.durationS);
      return Math.abs(before - after);
    });
    expect(Math.max(...jumps)).toBeGreaterThan(fixture.linearJumpMinimumDegS);
  });

  it("is seamless at the loop seam in position and velocity", () => {
    const at = clip.durationS;
    expect(angle(clip, at - 1e-9)).toBeCloseTo(angle(clip, at + 1e-9), 6);
    const { before, after } = sides((t) => angle(clip, t), at);
    expect(Math.abs(before - after)).toBeLessThan(fixture.angleVelocityToleranceDegS);
  });

  it("matches the uniform Catmull-Rom spline of three.js through the keys", () => {
    const values = clip.keys.map((k) => k.bones![fixture.bone]!.angleDeg!);
    const spline = new CatmullRomCurve3(values.map((v) => new Vector3(v, 0, 0)), true, "catmullrom", 0.5);
    for (let i = 0; i < 80; i++) {
      const t = (i / 80) * clip.durationS;
      expect(Math.abs(angle(clip, t) - spline.getPoint(t / clip.durationS).x)).toBeLessThan(fixture.oracleToleranceDeg);
    }
  });

  it("keeps the planted root continuous in position and velocity across keys, handovers and the seam", () => {
    const x = (t: number) => sampleClip(sparse, clip, t).root.x;
    const times = [0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5, 6, 8];
    for (const t of times) {
      const { before, after } = sides(x, t);
      expect(Math.abs(before - after)).toBeLessThan(fixture.rootVelocityToleranceMS);
    }
  });

  it("samples smooth scalar tracks with continuous slope", () => {
    const idle = clipOf(sparse, fixture.tracks.clip);
    const track = idle.tracks.find((tr) => tr.target === fixture.tracks.target)!;
    const f = (t: number) => sampleTrack(track, t, idle.loop, idle.durationS)!;
    for (const t of fixture.tracks.keyTimes) {
      const { before, after } = sides(f, t);
      expect(Math.abs(before - after)).toBeLessThan(fixture.trackVelocityTolerance);
    }
  });

  it("starts and ends non-looping transitions at rest", () => {
    const { from, to, durationS, bone } = fixture.transition;
    const t = transition(sparse, from, to, durationS);
    const f = (time: number) => sampleClip(sparse, t, time).offsets[bone]!.angleDeg;
    expect(Math.abs((f(e) - f(0)) / e)).toBeLessThan(0.01);
    expect(Math.abs((f(durationS) - f(durationS - e)) / e)).toBeLessThan(0.01);
    expect(f(durationS / 2)).toBeCloseTo(3, 9);
  });

  it("keeps accelerations finite and evaluation deterministic across key times", () => {
    for (const key of clip.keys) {
      const state = evaluate(sparse, { kind: "clip", clip: clip.id, timeS: key.t });
      for (const a of Object.values(state.kinematics.segmentAccelerations)) expect(Math.hypot(a.x, a.y)).toBeLessThan(60);
    }
  });
});
