/** 🎞️ Pose and animation: clip sampling against three's keyframe interpolants (linear, discrete, quaternion slerp), looping, joint limits clamped and reported with three's Quaternion as oracle. */
import { Quaternion, QuaternionKeyframeTrack, VectorKeyframeTrack, InterpolateDiscrete, InterpolateLinear, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { poseModel, sampleMaterials } from "../../🟦️.ts";
import type { ModelDocument } from "../../🧬️schema/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";

const data = fixture("🎞️pose-animation");
const doc = data.document as ModelDocument;
const tol = data.tolerance;
const angleY = (q: number[]) => 2 * Math.atan2(q[1]!, q[3]!);
const rot = (axis: number[], a: number): number[] => new Quaternion().setFromAxisAngle(new Vector3(...(axis as [number, number, number])), a).toArray();
const withPose = (id: string, nodes: Record<string, unknown>): ModelDocument => ({ ...doc, poses: [{ id, title: { en: id, de: id }, nodes }] }) as ModelDocument;

describe("animation sampling", () => {
  for (const s of data.samples) {
    it(`${s.animation} at ${s.timeS}`, () => {
      const pose = poseModel(doc, { animation: s.animation, timeS: s.timeS });
      expect(Math.abs(angleY(pose.local.arm!.rotation) - s.armAngleY)).toBeLessThan(tol);
      expect(Math.abs(pose.local.slider!.translation[0] - s.sliderX)).toBeLessThan(tol);
      const tip = pose.world.tip!;
      expect(Math.abs(tip[12]! - Math.cos(s.armAngleY))).toBeLessThan(tol);
      expect(Math.abs(tip[14]! + Math.sin(s.armAngleY))).toBeLessThan(tol);
      expect(pose.issues.map((i) => i.code)).toEqual(s.issues);
      const colour = sampleMaterials(doc, s.animation, s.timeS).skin!.baseColor!;
      colour.forEach((v, k) => expect(Math.abs(v - s.baseColor[k])).toBeLessThan(tol));
    });
  }

  it("matches three's keyframe interpolants", () => {
    const wave = doc.animations.find((a) => a.id === "wave")!;
    const [arm, slider, colour] = wave.channels;
    const q = (new QuaternionKeyframeTrack(".quaternion", arm!.times as number[], arm!.values as number[]) as any).createInterpolant();
    const step = (new VectorKeyframeTrack(".position", slider!.times as number[], slider!.values as number[], InterpolateDiscrete) as any).createInterpolant();
    const lerp = new VectorKeyframeTrack(".color", colour!.times as number[], (colour!.values as number[]).filter((_, i) => i % 4 !== 3), InterpolateLinear);
    expect(lerp.getValueSize()).toBe(3);
    for (const t of [0, 0.25, 0.5, 0.75, 1, 1.3, 1.9]) {
      const pose = poseModel(doc, { animation: "wave", timeS: t });
      const expected = q.evaluate(t);
      const got = pose.local.arm!.rotation;
      const dot = expected[0]! * got[0] + expected[1]! * got[1] + expected[2]! * got[2] + expected[3]! * got[3];
      expect(Math.abs(Math.abs(dot) - 1)).toBeLessThan(1e-6);
      expect(Math.abs(pose.local.slider!.translation[0] - step.evaluate(t)[0]!)).toBeLessThan(1e-6);
      expect(Math.abs(sampleMaterials(doc, "wave", t).skin!.baseColor![0]! - (lerp as any).createInterpolant().evaluate(t)[0]!)).toBeLessThan(1e-6);
    }
  });

  it("returns the rest pose without selector and rejects unknown ids", () => {
    const rest = poseModel(doc);
    expect(rest.world.tip![12]).toBe(1);
    expect(() => poseModel(doc, "ghost")).toThrow();
    expect(() => poseModel(doc, { animation: "ghost", timeS: 0 })).toThrow();
  });
});

describe("joint limits", () => {
  const j = data.joints;
  it("clamps a revolute joint and reports it", () => {
    const pose = poseModel(withPose("p", { arm: { rotation: rot([0, 1, 0], j.armBeyondLimit.angleY) } }), "p");
    expect(Math.abs(angleY(pose.local.arm!.rotation) - j.armBeyondLimit.clampedAngleY)).toBeLessThan(tol);
    expect(pose.issues.map((i) => [i.code, i.path])).toEqual([["joint-limit", "/nodes/1/joint"]]);
  });
  it("drops swing of a revolute joint", () => {
    const pose = poseModel(withPose("p", { arm: { rotation: rot([1, 0, 0], 0.3) } }), "p");
    expect(pose.issues.map((i) => i.code)).toContain("joint-swing");
    expect(pose.local.arm!.rotation.slice(0, 3).map(Math.abs)).toEqual([0, 0, 0]);
  });
  it("clamps a prismatic joint on both sides", () => {
    const hi = poseModel(withPose("p", { slider: { translation: [j.sliderBeyondLimit.x, 0.5, 0] } }), "p");
    expect(Math.abs(hi.local.slider!.translation[0] - j.sliderBeyondLimit.clampedX)).toBeLessThan(tol);
    const lo = poseModel(withPose("p", { slider: { translation: [j.sliderBeyondLimit.slideNegative, 0.5, 0] } }), "p");
    expect(Math.abs(lo.local.slider!.translation[0] - j.sliderBeyondLimit.clampedNegativeX)).toBeLessThan(tol);
    expect(hi.issues.map((i) => i.code)).toEqual(["joint-limit"]);
  });
  it("clamps twist and swing cone of a spherical joint", () => {
    const twist = new Quaternion().setFromAxisAngle(new Vector3(0, 0, 1), j.ballTwist.twistAboutZ);
    const swing = new Quaternion().setFromAxisAngle(new Vector3(1, 0, 0), j.ballSwing.swingAboutX);
    const q = swing.clone().multiply(twist);
    const pose = poseModel(withPose("p", { ball: { rotation: q.toArray() } }), "p");
    const got = new Quaternion().fromArray(pose.local.ball!.rotation);
    const t = 2 * Math.atan2(got.z, got.w);
    expect(Math.abs(t - j.ballTwist.clampedTwist)).toBeLessThan(1e-9);
    const sw = got.clone().multiply(new Quaternion().setFromAxisAngle(new Vector3(0, 0, 1), -t));
    expect(Math.abs(2 * Math.acos(Math.min(1, Math.abs(sw.w))) - j.ballSwing.clampedSwing)).toBeLessThan(1e-9);
    expect(pose.issues.map((i) => i.code)).toEqual(["joint-limit"]);
  });
  it("ignores poses of a fixed joint", () => {
    const pose = poseModel(withPose("p", { locked: { translation: [5, 5, 5] } }), "p");
    expect(pose.local.locked!.translation).toEqual([0, 2, 0]);
    expect(pose.issues.map((i) => i.code)).toEqual(["joint-fixed"]);
  });
});
