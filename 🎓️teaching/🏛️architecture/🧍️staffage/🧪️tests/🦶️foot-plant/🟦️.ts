/** 🦶️ Planted-foot root motion of the reference walker.
 *
 * @see ./🥒️.feature — the scenario
 * @see ../../🧫️fixtures/🦶️foot-plant/🔣️.json — the tolerances
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { clipOf, sampleClip, spanActive } from "../../🔨️modules/🎞️animation/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🦶️foot-plant/🔣️.json", import.meta.url), "utf8"));
const asset = (JSON.parse(readFileSync(new URL("../../🧫️fixtures/👤️sample-figure/🔣️.json", import.meta.url), "utf8")) as { asset: Asset }).asset;
const clip = clipOf(asset, fixture.clip);
const dt = 1 / fixture.stepsPerSecond;
const total = Math.round(clip.durationS * fixture.cycles * fixture.stepsPerSecond);
const inHandover = (t: number) => fixture.handovers.some((h: { fromS: number; toS: number }) => t >= h.fromS - 1e-9 && t <= h.toS + 1e-9);

describe("🦶️ foot plant", () => {
  it("holds planted anchors still within every span outside the handovers", () => {
    const held = new Map<string, number>();
    let worst = 0;
    for (let k = 0; k <= total; k++) {
      const t = k * dt + 0.00123;
      const local = t - Math.floor(t / clip.durationS) * clip.durationS;
      const cycle = Math.floor(t / clip.durationS);
      const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS: t });
      clip.contacts.forEach((span, index) => {
        const key = `${cycle}:${index}`;
        if (!spanActive(span, local, clip.durationS)) return;
        if (inHandover(local)) {
          held.delete(key);
          return;
        }
        const x = state.anchors[span.anchor]!.at.x;
        if (!held.has(key)) held.set(key, x);
        worst = Math.max(worst, Math.abs(x - held.get(key)!));
      });
    }
    expect(worst).toBeLessThan(fixture.driftToleranceM);
  });

  it("keeps the root continuous at handovers and the loop seam, with bounded acceleration", () => {
    const xs: number[] = [];
    for (let k = -2; k <= total + 2; k++) xs.push(sampleClip(asset, clip, k * dt + 0.0001).root.x);
    for (let k = 2; k < xs.length; k++) {
      expect(Math.abs((xs[k]! - 2 * xs[k - 1]! + xs[k - 2]!) / (dt * dt))).toBeLessThan(fixture.maxRootAccelerationMS2);
    }
    const travel = xs[xs.length - 3]! - xs[2]!;
    expect(travel / (clip.durationS * fixture.cycles)).toBeCloseTo(clip.advanceM! / clip.durationS, 2);
    const seam = sampleClip(asset, clip, clip.durationS - 1e-9).root.x - sampleClip(asset, clip, 0).root.x;
    expect(Math.abs(seam - clip.advanceM!)).toBeLessThan(fixture.handoverToleranceM);
  });

  it("puts the lowest active contact anchor on the ground", () => {
    for (let k = 0; k <= total; k += 7) {
      const t = k * dt + 0.0007;
      const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS: t });
      const lowest = Math.min(...state.contacts.map((id) => state.anchors[id]!.at.y));
      expect(Math.abs(lowest)).toBeLessThan(fixture.groundToleranceM);
    }
  });
});
