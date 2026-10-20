/** 🟠️ The introduction rect relative host phase of registerUiintroductionAppearance2AndMoreTests. */
import type { buildCelebrateStampDurationState } from "../🔶️celebrate-stamp-duration/🟦️.tsx";

/** 🟡 Registers the introduction rect relative host tests of registerUiintroductionAppearance2AndMoreTests. */
export function registerIntroductionRectRelativeHostTests({ describe, it, expect, introductionRectRelativeToHost, introductionPointRelativeToHost, clampIntroductionInfoBoxPosition, INTRODUCTION_INFO_BOX_GAP_PX, resolveIntroductionPlacement }: Pick<Awaited<ReturnType<typeof buildCelebrateStampDurationState>>, "describe" | "it" | "expect" | "introductionRectRelativeToHost" | "introductionPointRelativeToHost" | "clampIntroductionInfoBoxPosition" | "INTRODUCTION_INFO_BOX_GAP_PX" | "resolveIntroductionPlacement">) {
  describe("introductionRectRelativeToHost", () => {
    it("subtracts the host origin from a viewport rect so absolute overlays survive CSS-transformed shells", () => {
      expect(introductionRectRelativeToHost({ top: 120, left: 80, width: 40, height: 20 }, { top: 100, left: 50, width: 400, height: 300 })).toEqual({
        top: 20,
        left: 30,
        width: 40,
        height: 20,
      });
    });
  });

  describe("introductionPointRelativeToHost", () => {
    it("subtracts the host origin from a viewport point", () => {
      expect(introductionPointRelativeToHost({ x: 250, y: 180 }, { top: 100, left: 50, width: 400, height: 300 })).toEqual({ x: 200, y: 80 });
    });
  });

  describe("clampIntroductionInfoBoxPosition", () => {
    const viewport = { width: 800, height: 600 };
    const boxSize = { width: 100, height: 50 };

    it("lets direct manipulation meet every viewport border exactly", () => {
      expect(clampIntroductionInfoBoxPosition({ top: -100, left: -100 }, boxSize, viewport, 0)).toEqual({ top: 0, left: 0 });
      expect(clampIntroductionInfoBoxPosition({ top: 1_000, left: 1_000 }, boxSize, viewport, 0)).toEqual({ top: 550, left: 700 });
    });

    it("preserves the authored placement inset", () => {
      expect(clampIntroductionInfoBoxPosition({ top: -100, left: -100 }, boxSize, viewport, INTRODUCTION_INFO_BOX_GAP_PX)).toEqual({ top: 16, left: 16 });
      expect(clampIntroductionInfoBoxPosition({ top: 1_000, left: 1_000 }, boxSize, viewport, INTRODUCTION_INFO_BOX_GAP_PX)).toEqual({ top: 534, left: 684 });
    });
  });

  describe("resolveIntroductionPlacement", () => {
    const viewport = { width: 800, height: 600 };
    const boxSize = { width: 100, height: 50 };

    it("centers when there is no anchor rect, or placement is center", () => {
      expect(resolveIntroductionPlacement("auto", null, boxSize, viewport)).toEqual({ top: 275, left: 350 });
      expect(resolveIntroductionPlacement("center", { top: 10, left: 10, width: 20, height: 20 }, boxSize, viewport)).toEqual({ top: 275, left: 350 });
    });

    it("keeps a centered box inside the viewport, so its Skip and Next controls stay clickable", () => {
      const oversized = resolveIntroductionPlacement("center", null, { width: 1585, height: 1041 }, { width: 1425, height: 885 });
      expect(oversized.top).toBeGreaterThanOrEqual(0);
      expect(oversized.left).toBeGreaterThanOrEqual(0);
      const tiny = resolveIntroductionPlacement("auto", null, boxSize, { width: 0, height: 0 });
      expect(tiny.top).toBeGreaterThanOrEqual(0);
      expect(tiny.left).toBeGreaterThanOrEqual(0);
    });

    it("auto picks the side with the most free space", () => {
      const nearTop = { top: 0, left: 350, width: 100, height: 20 };
      const placedBelow = resolveIntroductionPlacement("auto", nearTop, boxSize, viewport);
      expect(placedBelow.top).toBeGreaterThan(nearTop.top + nearTop.height);
    });

    it("explicit placements position relative to the anchor and stay within the viewport", () => {
      const anchor = { top: 275, left: 350, width: 100, height: 20 };
      const right = resolveIntroductionPlacement("right", anchor, boxSize, viewport);
      expect(right.left).toBeGreaterThan(anchor.left + anchor.width);
      const left = resolveIntroductionPlacement("left", anchor, boxSize, viewport);
      expect(left.left).toBeLessThan(anchor.left);
      const nearEdge = resolveIntroductionPlacement("right", { top: 10, left: 780, width: 10, height: 10 }, boxSize, viewport);
      expect(nearEdge.left).toBeLessThanOrEqual(viewport.width - boxSize.width);
    });
  });
}
