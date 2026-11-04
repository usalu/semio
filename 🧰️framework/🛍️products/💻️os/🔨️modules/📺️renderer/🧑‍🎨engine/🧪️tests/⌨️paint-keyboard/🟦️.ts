import { describe, expect, it } from "vitest";
import { STROKE_POINT_CAP, STROKE_WARNING_POINTS, cursorAnnouncement, keyboardCameraStep, keyboardMarquee, keyboardStampPath, rolloverStroke, stepKeyboardCursor, strokeLimitState } from "../../🧱️elements/🖌️Paint2dHost/✍️editing/⌨️keyboard/🟦️.ts";

const extent = { width: 4, height: 3 };

describe("keyboard pixel cursor", () => {
  it("moves one pixel per arrow key and clamps at the image border", () => {
    expect(stepKeyboardCursor({ x: 1, y: 1 }, "ArrowRight", extent)).toEqual({ x: 2, y: 1 });
    expect(stepKeyboardCursor({ x: 1, y: 1 }, "ArrowUp", extent)).toEqual({ x: 1, y: 0 });
    expect(stepKeyboardCursor({ x: 0, y: 0 }, "ArrowLeft", extent)).toEqual({ x: 0, y: 0 });
    expect(stepKeyboardCursor({ x: 3, y: 2 }, "ArrowDown", extent)).toEqual({ x: 3, y: 2 });
    expect(stepKeyboardCursor({ x: 1, y: 1 }, "ArrowRight", extent, 10)).toEqual({ x: 3, y: 1 });
  });
  it("ignores every other key", () => {
    expect(stepKeyboardCursor({ x: 1, y: 1 }, "a", extent)).toBeNull();
  });
  it("stamps through the pixel center as a two-point stroke so it reuses the pointer stroke payload", () => {
    expect(keyboardStampPath({ x: 2, y: 1 })).toEqual([[2.5, 1.5], [2.5, 1.5]]);
  });
  it("builds an inclusive marquee that covers anchor and cursor pixels in any direction", () => {
    expect(keyboardMarquee({ x: 1, y: 1 }, { x: 3, y: 2 })).toEqual({ kind: "rectangle", x: 1, y: 1, width: 3, height: 2 });
    expect(keyboardMarquee({ x: 3, y: 2 }, { x: 1, y: 1 })).toEqual({ kind: "rectangle", x: 1, y: 1, width: 3, height: 2 });
    expect(keyboardMarquee({ x: 2, y: 2 }, { x: 2, y: 2 })).toEqual({ kind: "rectangle", x: 2, y: 2, width: 1, height: 1 });
  });
  it("announces a one-based position in the user's language", () => {
    expect(cursorAnnouncement("en", { x: 0, y: 2 }, extent)).toBe("Pixel column 1 of 4, row 3 of 3");
    expect(cursorAnnouncement("de", { x: 0, y: 2 }, extent)).toBe("Pixel Spalte 1 von 4, Zeile 3 von 3");
  });
});

describe("keyboard camera", () => {
  const camera = { x: 10, y: 20, zoom: 2 };
  it("zooms with + and - and resets with 0", () => {
    expect(keyboardCameraStep(camera, "+")!.zoom).toBeCloseTo(2.5);
    expect(keyboardCameraStep(camera, "=")!.zoom).toBeCloseTo(2.5);
    expect(keyboardCameraStep(camera, "-")!.zoom).toBeCloseTo(1.6);
    expect(keyboardCameraStep(camera, "0")).toEqual({ x: 10, y: 20, zoom: 1 });
  });
  it("keeps zoom inside the camera bounds", () => {
    expect(keyboardCameraStep({ x: 0, y: 0, zoom: 32 }, "+")!.zoom).toBe(32);
    expect(keyboardCameraStep({ x: 0, y: 0, zoom: 0.05 }, "-")!.zoom).toBe(0.05);
  });
  it("pans with arrows by a fixed screen distance regardless of zoom", () => {
    expect(keyboardCameraStep(camera, "ArrowRight", 40)).toEqual({ x: 30, y: 20, zoom: 2 });
    expect(keyboardCameraStep(camera, "ArrowUp", 40)).toEqual({ x: 10, y: 0, zoom: 2 });
  });
  it("leaves other keys alone", () => {
    expect(keyboardCameraStep(camera, "x")).toBeNull();
  });
});

describe("stroke limit", () => {
  const point = (index: number): [number, number] => [index, 0];
  it("warns before the cap and reports full at the cap", () => {
    expect(strokeLimitState(STROKE_WARNING_POINTS - 1)).toBe("ok");
    expect(strokeLimitState(STROKE_WARNING_POINTS)).toBe("warn");
    expect(strokeLimitState(STROKE_POINT_CAP)).toBe("full");
  });
  it("does nothing below the cap", () => {
    expect(rolloverStroke(Array.from({ length: STROKE_POINT_CAP - 1 }, (_, index) => point(index)), 0)).toBeNull();
  });
  it("streams the unsent points at the cap and continues from the last point without losing any", () => {
    const points = Array.from({ length: STROKE_POINT_CAP }, (_, index) => point(index));
    const result = rolloverStroke(points, 16)!;
    expect(result.batch).toEqual(points.slice(16));
    expect(result.points).toEqual([points[STROKE_POINT_CAP - 1]]);
    expect(result.sent).toBe(1);
  });
});
