/** ⚖️ The evaluated instants the pipeline keeps are bounded by what they weigh, across all assets: a kept instant of a heavy figure holds its evaluated shapes, its scenes and their markup, and a bound by count per asset lets a page of live figures keep gigabytes.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🧮️pipeline/🟦️.ts — `evaluateState`, `STATE_BUDGET_POINTS` */
import { describe, expect, it } from "vitest";
import type { Asset, EvaluatedState } from "@teaching/staffage";
import { STATE_BUDGET_POINTS, evaluateState, evaluatedStateLoad } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { sampleFigure, walker } from "../🧰️support/🧰️.ts";

const selector = { kind: "scale", scale: { denominator: 100 } } as const;
const instant = (asset: Asset, frame: number): EvaluatedState => evaluateState(asset, { kind: "clip", clip: asset.presentation.defaultClip, timeS: 1000 + frame / 30 }, selector);
const points = (state: EvaluatedState): number => state.shapes.elevation.reduce((sum, shape) => sum + shape.rings.reduce((ring, at) => ring + at.length, 0), 0);

describe("⚖️ state budget", () => {
  it("never keeps more evaluated points than the budget and evaluates a dropped instant again", () => {
    const first = instant(walker, 0);
    const each = points(first);
    expect(each).toBeGreaterThan(0);
    const frames = Math.ceil((STATE_BUDGET_POINTS / each) * 1.5);
    let peak = 0;
    for (let frame = 1; frame <= frames; frame += 1) {
      instant(walker, frame);
      peak = Math.max(peak, evaluatedStateLoad().points);
    }
    expect(peak).toBeLessThanOrEqual(STATE_BUDGET_POINTS);
    expect(peak).toBeGreaterThan(STATE_BUDGET_POINTS / 2);
    expect(evaluatedStateLoad().states).toBeLessThan(frames);
    const again = instant(walker, 0);
    expect(again).not.toBe(first);
    expect(points(again)).toBe(each);
    expect(again.bounds).toEqual(first.bounds);
  });

  it("returns the kept state of a recurring instant", () => {
    const kept = instant(walker, 5000);
    for (let frame = 5001; frame < 5010; frame += 1) instant(walker, frame);
    const before = evaluatedStateLoad();
    expect(instant(walker, 5000)).toBe(kept);
    expect(evaluatedStateLoad()).toEqual(before);
  });

  it("holds one budget for all assets together", () => {
    const frames = Math.ceil(STATE_BUDGET_POINTS / Math.max(1, points(instant(walker, 9000))));
    let last: readonly [EvaluatedState, EvaluatedState] | undefined;
    for (let frame = 9001; frame <= 9000 + frames; frame += 1) {
      last = [instant(walker, frame), instant(sampleFigure, frame)];
      expect(evaluatedStateLoad().points).toBeLessThanOrEqual(STATE_BUDGET_POINTS);
    }
    expect(instant(walker, 9000 + frames)).toBe(last![0]);
    expect(instant(sampleFigure, 9000 + frames)).toBe(last![1]);
  });
});
