/** 🟣️ The sample bezier segments phase of registerUiintroductionAppearance2AndMoreTests. */
import type { buildCelebrateStampDurationState } from "../🔶️celebrate-stamp-duration/🟦️.tsx";

/** 🟤 Registers the sample bezier segments tests of registerUiintroductionAppearance2AndMoreTests. */
export function registerSampleBezierSegmentsTests({ describe, it, sampleBezierSegments, expect, ndcToViewportPoint, introductionDemoResolveVisual }: Pick<Awaited<ReturnType<typeof buildCelebrateStampDurationState>>, "describe" | "it" | "sampleBezierSegments" | "expect" | "ndcToViewportPoint" | "introductionDemoResolveVisual">) {
  describe("sampleBezierSegments", () => {
    it("samples move/line segments verbatim and quad/cubic curves through their control points", () => {
      const linePoints = sampleBezierSegments([
        { kind: "move", to: [0, 0] },
        { kind: "line", to: [10, 0] },
      ]);
      expect(linePoints).toEqual([
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ]);

      const quadPoints = sampleBezierSegments(
        [
          { kind: "move", to: [0, 0] },
          { kind: "quad", ctrl: [5, 10], to: [10, 0] },
        ],
        2,
      );
      expect(quadPoints).toHaveLength(3);
      expect(quadPoints[1].x).toBeCloseTo(5, 5);
      expect(quadPoints[1].y).toBeCloseTo(5, 5);
      expect(quadPoints[2]).toEqual({ x: 10, y: 0 });
    });

    it("ignores unsampleable segment kinds without throwing", () => {
      expect(sampleBezierSegments([{ kind: "close" }])).toEqual([]);
      expect(sampleBezierSegments([])).toEqual([]);
    });
  });

  describe("ndcToViewportPoint", () => {
    it("maps NDC corners and center to viewport pixels", () => {
      const rect = { left: 100, top: 50, width: 200, height: 100 };
      expect(ndcToViewportPoint({ x: 0, y: 0 }, rect)).toEqual({ x: 200, y: 100 });
      expect(ndcToViewportPoint({ x: -1, y: -1 }, rect)).toEqual({ x: 100, y: 150 });
      expect(ndcToViewportPoint({ x: 1, y: 1 }, rect)).toEqual({ x: 300, y: 50 });
    });
  });

  describe("introductionDemoResolveVisual", () => {
    it("maps each gesture kind to a unique button, modifier set, and feedback family", () => {
      const at = { kind: "screenNormalized" as const, x: 0.5, y: 0.5 };
      expect(introductionDemoResolveVisual({ kind: "leftClick", at })).toEqual({ button: "left", modifiers: [], feedback: "leftClick", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "rightClick", at })).toEqual({ button: "right", modifiers: [], feedback: "rightClick", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "doubleClick", at })).toEqual({ button: "left", modifiers: [], feedback: "doubleClick", showDoubleChip: true });
      expect(introductionDemoResolveVisual({ kind: "scroll", at, deltaY: -100 })).toEqual({ button: "wheel", modifiers: [], feedback: "scroll", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "drag", from: at, to: at })).toEqual({ button: "left", modifiers: [], feedback: "dragLeft", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "drag", from: at, to: at, button: "middle" })).toEqual({ button: "middle", modifiers: [], feedback: "dragMiddle", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "orbit", from: at, to: at })).toEqual({ button: "right", modifiers: ["alt"], feedback: "orbit", showDoubleChip: false });
    });
  });
}
