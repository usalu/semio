/** 🎞️ Eases, wrapping, frame numbers, key sampling, tracks, transitions and determinism of clips.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🎞️clip-sampling/🔣️.json — the vectors
 */
import { readFileSync } from "node:fs";
import { easeLinear, easeQuadIn, easeQuadOut } from "d3-ease";
import { MathUtils } from "three";
import { describe, expect, it } from "vitest";
import type { Asset, Ease } from "../../🧬️schema/🟦️.ts";
import { clipOf, contactsAt, ease, frameAt, phaseAt, sampleClip, wrapTime } from "../../🔨️modules/🎞️animation/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎞️clip-sampling/🔣️.json", import.meta.url), "utf8"));
const asset = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🚶️reference-walker/🔣️.json", import.meta.url), "utf8")) as Asset;

describe("🎞️ clip sampling", () => {
  it("evaluates the ease curves", () => {
    for (const { ease: kind, t, value } of fixture.eases) expect(ease(kind as Ease, t)).toBeCloseTo(value, 12);
  });

  it("agrees with three.js smoothstep and d3-ease on a dense grid", () => {
    for (let i = 0; i <= 100; i++) {
      const t = i / 100;
      expect(ease("smooth", t)).toBeCloseTo(MathUtils.smoothstep(t, 0, 1), 12);
      expect(ease("in", t)).toBeCloseTo(easeQuadIn(t), 12);
      expect(ease("out", t)).toBeCloseTo(easeQuadOut(t), 12);
      expect(ease("linear", t)).toBeCloseTo(easeLinear(t), 12);
    }
  });

  it("wraps looping times and clamps others", () => {
    for (const { t, duration, loop, value } of fixture.wrap) expect(wrapTime(t, duration, loop)).toBeCloseTo(value, 12);
  });

  it("numbers frames by floor(t · fps + 1e-9)", () => {
    for (const { t, fps, frame } of fixture.frames) expect(frameAt(t, fps)).toBe(frame);
    expect(sampleClip(asset, clipOf(asset, "idle-breathing"), 7 / 30).frame).toBe(7);
  });

  it("interpolates bone offsets between keys by the eased weight", () => {
    for (const s of fixture.keySamples) {
      const sample = sampleClip(asset, clipOf(asset, s.clip), s.t);
      for (const [bone, deg] of Object.entries(s.bones)) expect(sample.offsets[bone]!.angleDeg).toBeCloseTo(deg as number, 12);
      for (const [id, k] of Object.entries(s.loadScale ?? {})) expect(sample.loadScales[id]).toBeCloseTo(k as number, 12);
    }
  });

  it("samples scalar tracks and binds them to the thermal inputs", () => {
    for (const { clip, t, target, value } of fixture.tracks) {
      const sample = sampleClip(asset, clipOf(asset, clip), t);
      if (target.startsWith("emitter:")) expect(sample.thermal.emitters[target.slice(8)]).toBeCloseTo(value, 12);
      else expect(sample.loadScales[target.slice(5)]).toBeCloseTo(value, 12);
    }
  });

  it("blends a transition and clamps outside its duration", () => {
    const { from, to, durationS, atHalf, clampedBefore, clampedAfter } = fixture.transition;
    const at = (timeS: number) => evaluate(asset, { kind: "transition", from, to, durationS, timeS });
    const half = at(0.5);
    for (const [bone, deg] of Object.entries(atHalf.bones)) expect(half.bones[bone]!.offset.angleDeg).toBeCloseTo(deg as number, 12);
    expect(half.thermal.activityMet).toBeCloseTo(atHalf.activityMet, 12);
    expect(half.loads[0]!.fyN).toBeCloseTo(atHalf.loadFyN, 3);
    expect(half.contacts).toEqual(atHalf.contacts);
    expect(at(-1).bones.torso!.offset.angleDeg).toBe(clampedBefore);
    expect(at(5).bones.torso!.offset.angleDeg).toBe(-3);
    expect(at(5).timeS).toBe(durationS);
    expect(clampedAfter).toBe(1);
  });

  it("looks up phases and contacts", () => {
    for (const { clip, t, phase } of fixture.phases) expect(phaseAt(clipOf(asset, clip), t)).toBe(phase);
    for (const { clip, t, anchors } of fixture.contacts) expect(contactsAt(clipOf(asset, clip), t)).toEqual(anchors);
  });

  it("is deterministic and periodic", () => {
    for (const clip of asset.clips) {
      for (const t of [0, 0.123, 0.5, 0.987]) {
        const a = evaluate(asset, { kind: "clip", clip: clip.id, timeS: t });
        const b = evaluate(asset, { kind: "clip", clip: clip.id, timeS: t });
        expect(JSON.stringify(a.bones)).toBe(JSON.stringify(b.bones));
        expect(JSON.stringify(a.kinematics)).toBe(JSON.stringify(b.kinematics));
        const next = evaluate(asset, { kind: "clip", clip: clip.id, timeS: t + 3 * clip.durationS });
        expect(next.frame).toBe(a.frame);
        expect(next.phase).toBe(a.phase);
        const advance = (clip.rootMotion === "plant" ? clip.advanceM ?? 0 : 0) * 3;
        for (const id of Object.keys(a.bones)) {
          expect(next.bones[id]!.origin.x - a.bones[id]!.origin.x).toBeCloseTo(advance, 6);
          expect(next.bones[id]!.origin.y).toBeCloseTo(a.bones[id]!.origin.y, 6);
          expect(next.bones[id]!.angleRad).toBeCloseTo(a.bones[id]!.angleRad, 6);
        }
      }
    }
  });
});
