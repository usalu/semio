/** 🔄️ Looping clips with continuous rotation: closing keys, whole turns, seam continuity and the validator rule.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🔄️continuous-rotation/🔣️.json — the wheel
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { clipOf, sampleClip } from "../../🔨️modules/🎞️animation/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔄️continuous-rotation/🔣️.json", import.meta.url), "utf8"));
const wheel = fixture.asset as Asset;
const e: number = fixture.epsilonS;
const angle = (asset: Asset, t: number) => evaluate(asset, { kind: "clip", clip: fixture.clip, timeS: t }).bones[fixture.bone]!.offset.angleDeg;
const velocity = (asset: Asset, t: number) => ({ before: (angle(asset, t) - angle(asset, t - e)) / e, after: (angle(asset, t + e) - angle(asset, t)) / e });
const withWheel = (joint: object): Asset => ({ ...wheel, rig: { ...wheel.rig, bones: wheel.rig.bones.map((b) => (b.id === fixture.bone ? { ...b, joint } : b)) } }) as Asset;

describe("🔄️ continuous rotation", () => {
  it("validates the spinning wheel", () => {
    expect(validateAsset(wheel)).toEqual([]);
  });

  it("differs by exactly 720 degrees one loop later", () => {
    const loop = fixture.durationS;
    for (const t of [0, 0.1, 0.37, 0.999, 1, 1.5, 1.9999]) {
      expect(angle(wheel, t + loop) - angle(wheel, t)).toBeCloseTo(360 * fixture.turnsPerLoop, 9);
      expect(angle(wheel, t + 3 * loop) - angle(wheel, t)).toBeCloseTo(3 * 360 * fixture.turnsPerLoop, 9);
      expect(angle(wheel, t - loop) - angle(wheel, t)).toBeCloseTo(-360 * fixture.turnsPerLoop, 9);
    }
  });

  it("turns at a constant angular velocity through keys and seam", () => {
    for (const t of [0.37, 0.999, 1.5, 2.37]) expect(angle(wheel, t)).toBeCloseTo(fixture.angularSpeedDegS * t, 9);
    for (const t of [1, 2, 4, 5]) {
      const { before, after } = velocity(wheel, t);
      expect(Math.abs(before - after)).toBeLessThan(fixture.velocityToleranceDegS);
      expect(after).toBeCloseTo(fixture.angularSpeedDegS, 1);
    }
  });

  it("keeps the kinematics continuous across the seam", () => {
    const speed = (fixture.angularSpeedDegS * Math.PI) / 180;
    for (const t of [0.999, 1.999, 2, 2.0005, 6.5]) {
      const state = evaluate(wheel, { kind: "clip", clip: fixture.clip, timeS: t });
      const a = state.kinematics.segmentAccelerations[fixture.bone]!;
      expect(Math.hypot(a.x, a.y)).toBeCloseTo(speed * speed * fixture.comRadiusM, 2);
      expect(Math.hypot(state.kinematics.comVelocity.x, state.kinematics.comVelocity.y)).toBeCloseTo(speed * fixture.comRadiusM, 3);
    }
  });

  it("samples the offset with the cycle difference added", () => {
    const clip = clipOf(wheel, fixture.clip);
    expect(sampleClip(wheel, clip, 3.25).offsets[fixture.bone]!.angleDeg - sampleClip(wheel, clip, 1.25).offsets[fixture.bone]!.angleDeg).toBeCloseTo(720, 9);
    expect(sampleClip(wheel, clip, 3.25).timeS).toBeCloseTo(1.25, 12);
  });

  it("reports a closing key that differs on a limited joint, or by a fraction of a turn", () => {
    const limited = validateAsset(withWheel({ type: "revolute", minDeg: -1000, maxDeg: 1000 }));
    expect(limited.map((i) => i.code)).toContain("loop-closing");
    const fraction = JSON.parse(JSON.stringify(wheel)) as Asset;
    (fraction.clips[0]!.keys[2]!.bones as Record<string, { angleDeg: number }>).wheel!.angleDeg = 700;
    expect(validateAsset(fraction).map((i) => i.code)).toContain("loop-closing");
    const turn = JSON.parse(JSON.stringify(wheel)) as Asset;
    (turn.clips[0]!.keys[2]!.bones as Record<string, { angleDeg: number }>).wheel!.angleDeg = -360;
    expect(validateAsset(turn)).toEqual([]);
  });

  it("wraps back to the first key when there is no closing key", () => {
    const open = JSON.parse(JSON.stringify(wheel)) as Asset;
    (open.clips as Asset["clips"][number][])[0] = { ...open.clips[0]!, keys: open.clips[0]!.keys.slice(0, 2) };
    expect(validateAsset(open)).toEqual([]);
    const at = (t: number) => angle(open, t);
    expect(at(2 - 1e-9)).toBeCloseTo(at(2 + 1e-9), 6);
    expect(at(0)).toBeCloseTo(0, 9);
    expect(at(1)).toBeCloseTo(360, 9);
    expect(at(2 + 1)).toBeCloseTo(at(1), 9);
  });
});
