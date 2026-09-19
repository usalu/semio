// #region 🧲️Header
/** @emoji 🧪️ Unit suite for `👆️gesture` — `pointerId`-keyed tracking, pinch diffing, and the
 * wheel/camera conversions every viewport host shares. */
// #endregion 🧲️Header

type TestSource = { readonly directory: string; readonly url: string };
type GestureModule = typeof import("../../🟦️.ts");

export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: Pick<
    GestureModule,
    | "EMPTY_GESTURE_POINTERS"
    | "GESTURE_MULTI_TOUCH_POINTERS"
    | "IDENTITY_PINCH_STEP"
    | "PINCH_WHEEL_PIXELS_PER_DOUBLING"
    | "applyPinchToCamera"
    | "clampZoom"
    | "gestureIsMultiTouch"
    | "gesturePointerDown"
    | "gesturePointerMove"
    | "gesturePointerUp"
    | "pinchFrame"
    | "pinchStep"
    | "pinchWheelDelta"
    | "shortestAngleDelta"
    | "zoomAboutPoint"
  >,
  _source: TestSource,
): Promise<void> {
  const {
    EMPTY_GESTURE_POINTERS,
    GESTURE_MULTI_TOUCH_POINTERS,
    IDENTITY_PINCH_STEP,
    PINCH_WHEEL_PIXELS_PER_DOUBLING,
    applyPinchToCamera,
    clampZoom,
    gestureIsMultiTouch,
    gesturePointerDown,
    gesturePointerMove,
    gesturePointerUp,
    pinchFrame,
    pinchStep,
    pinchWheelDelta,
    shortestAngleDelta,
    zoomAboutPoint,
  } = dependencies;

  const { describe, expect, it } = vitest;

  const twoFingers = (ax: number, ay: number, bx: number, by: number) =>
    gesturePointerDown(gesturePointerDown(EMPTY_GESTURE_POINTERS, { pointerId: 1, x: ax, y: ay }), { pointerId: 2, x: bx, y: by });

  describe("👆️gesture pointer tracking", () => {
    it("keeps contacts in down-order so the primary pointer never changes identity", () => {
      const state = gesturePointerDown(twoFingers(0, 0, 10, 0), { pointerId: 3, x: 5, y: 5 });
      expect(state.pointers.map((pointer) => pointer.pointerId)).toEqual([1, 2, 3]);
    });

    it("re-seats a duplicate pointerdown in place instead of tracking the same finger twice", () => {
      const state = gesturePointerDown(twoFingers(0, 0, 10, 0), { pointerId: 1, x: 4, y: 4 });
      expect(state.pointers).toHaveLength(2);
      expect(state.pointers[0]).toEqual({ pointerId: 1, x: 4, y: 4 });
    });

    it("ignores a move for an untracked pointer — a hover must never fabricate a contact", () => {
      const state = gesturePointerMove(EMPTY_GESTURE_POINTERS, { pointerId: 9, x: 1, y: 1 });
      expect(state).toBe(EMPTY_GESTURE_POINTERS);
    });

    it("moves only the addressed contact", () => {
      const state = gesturePointerMove(twoFingers(0, 0, 10, 0), { pointerId: 2, x: 20, y: 0 });
      expect(state.pointers).toEqual([
        { pointerId: 1, x: 0, y: 0 },
        { pointerId: 2, x: 20, y: 0 },
      ]);
    });

    it("pointerUp drops one contact and returns the same object for an unknown id", () => {
      const both = twoFingers(0, 0, 10, 0);
      expect(gesturePointerUp(both, 2).pointers.map((pointer) => pointer.pointerId)).toEqual([1]);
      expect(gesturePointerUp(both, 77)).toBe(both);
    });

    it("gestureIsMultiTouch flips exactly at GESTURE_MULTI_TOUCH_POINTERS", () => {
      expect(GESTURE_MULTI_TOUCH_POINTERS).toBe(2);
      expect(gestureIsMultiTouch(EMPTY_GESTURE_POINTERS)).toBe(false);
      expect(gestureIsMultiTouch(gesturePointerDown(EMPTY_GESTURE_POINTERS, { pointerId: 1, x: 0, y: 0 }))).toBe(false);
      expect(gestureIsMultiTouch(twoFingers(0, 0, 10, 0))).toBe(true);
    });
  });

  describe("👆️gesture pinch frame", () => {
    it("answers null below two contacts", () => {
      expect(pinchFrame(EMPTY_GESTURE_POINTERS)).toBeNull();
      expect(pinchFrame(gesturePointerDown(EMPTY_GESTURE_POINTERS, { pointerId: 1, x: 3, y: 4 }))).toBeNull();
    });

    it("reads centroid, separation and angle from the first two contacts", () => {
      const frame = pinchFrame(twoFingers(0, 0, 10, 0))!;
      expect(frame.centroidX).toBe(5);
      expect(frame.centroidY).toBe(0);
      expect(frame.distance).toBe(10);
      expect(frame.angle).toBe(0);
    });

    it("ignores a third finger so mid-pinch the anchor does not teleport", () => {
      const two = twoFingers(0, 0, 10, 0);
      const three = gesturePointerDown(two, { pointerId: 3, x: 500, y: 500 });
      expect(pinchFrame(three)).toEqual(pinchFrame(two));
    });
  });

  describe("👆️gesture pinch step", () => {
    it("reports the separation ratio as scale (spread = zoom in)", () => {
      const step = pinchStep(pinchFrame(twoFingers(0, 0, 10, 0))!, pinchFrame(twoFingers(0, 0, 20, 0))!);
      expect(step.scale).toBeCloseTo(2, 10);
    });

    it("reports centroid travel as pan and leaves scale at 1 for a rigid two-finger drag", () => {
      const step = pinchStep(pinchFrame(twoFingers(0, 0, 10, 0))!, pinchFrame(twoFingers(4, 7, 14, 7))!);
      expect(step.scale).toBeCloseTo(1, 10);
      expect(step.panX).toBeCloseTo(4, 10);
      expect(step.panY).toBeCloseTo(7, 10);
      expect(step.rotation).toBeCloseTo(0, 10);
    });

    it("reports a signed rotation for a twist", () => {
      const step = pinchStep(pinchFrame(twoFingers(0, 0, 10, 0))!, pinchFrame(twoFingers(0, 0, 0, 10))!);
      expect(step.rotation).toBeCloseTo(Math.PI / 2, 10);
    });

    it("collapses a degenerate (coincident-contact) frame to no scale and no rotation", () => {
      const step = pinchStep(pinchFrame(twoFingers(5, 5, 5, 5))!, pinchFrame(twoFingers(6, 6, 26, 6))!);
      expect(step.scale).toBe(1);
      expect(step.rotation).toBe(0);
      expect(step.panX).toBeCloseTo(11, 10);
    });

    it("IDENTITY_PINCH_STEP changes nothing when applied to a camera", () => {
      const camera = { x: 3, y: -2, zoom: 1.5 };
      const next = applyPinchToCamera(camera, IDENTITY_PINCH_STEP, { w: 800, h: 600 }, { min: 0.1, max: 12 });
      expect(next.zoom).toBeCloseTo(camera.zoom, 10);
      expect(next.x).toBeCloseTo(camera.x, 10);
      expect(next.y).toBeCloseTo(camera.y, 10);
    });
  });

  describe("👆️gesture angle seam", () => {
    it("crossing ±π reports the SHORT way round, never a full turn", () => {
      expect(shortestAngleDelta(Math.PI - 0.1, -Math.PI + 0.1)).toBeCloseTo(0.2, 10);
      expect(shortestAngleDelta(-Math.PI + 0.1, Math.PI - 0.1)).toBeCloseTo(-0.2, 10);
    });

    it("stays inside (-π, π] and reports π (not -π) for an exact half turn", () => {
      expect(shortestAngleDelta(0, Math.PI)).toBeCloseTo(Math.PI, 10);
      expect(shortestAngleDelta(0, -Math.PI)).toBeCloseTo(Math.PI, 10);
      expect(shortestAngleDelta(0, 0)).toBeCloseTo(0, 10);
    });
  });

  describe("👆️gesture zoom conversions", () => {
    it("pinchWheelDelta inverts the exponential wheel zoom: doubling = one negative doubling-worth", () => {
      expect(pinchWheelDelta(2)).toBeCloseTo(-PINCH_WHEEL_PIXELS_PER_DOUBLING, 10);
      expect(pinchWheelDelta(0.5)).toBeCloseTo(PINCH_WHEEL_PIXELS_PER_DOUBLING, 10);
      expect(pinchWheelDelta(1)).toBeCloseTo(0, 10);
    });

    it("pinchWheelDelta refuses a non-finite or non-positive scale", () => {
      expect(pinchWheelDelta(0)).toBe(0);
      expect(pinchWheelDelta(-1)).toBe(0);
      expect(pinchWheelDelta(Number.NaN)).toBe(0);
    });

    it("clampZoom bounds the factor and falls back to min for nonsense", () => {
      expect(clampZoom(5, { min: 0.1, max: 4 })).toBe(4);
      expect(clampZoom(0.01, { min: 0.1, max: 4 })).toBe(0.1);
      expect(clampZoom(Number.NaN, { min: 0.1, max: 4 })).toBe(0.1);
      expect(clampZoom(-3, { min: 0.1, max: 4 })).toBe(0.1);
    });

    it("zoomAboutPoint keeps the world point under the anchor fixed", () => {
      const camera = { x: 10, y: -4, zoom: 2 };
      const viewport = { w: 800, h: 600 };
      const anchor = { x: 200, y: 450 };
      const worldAt = (cam: { x: number; y: number; zoom: number }) => ({
        x: cam.x + (anchor.x - viewport.w / 2) / cam.zoom,
        y: cam.y + (anchor.y - viewport.h / 2) / cam.zoom,
      });
      const before = worldAt(camera);
      const after = worldAt(zoomAboutPoint(camera, anchor, viewport, 6.5));
      expect(after.x).toBeCloseTo(before.x, 10);
      expect(after.y).toBeCloseTo(before.y, 10);
    });

    it("applyPinchToCamera zooms about the centroid, then pans by the centroid travel in world units", () => {
      const camera = { x: 0, y: 0, zoom: 1 };
      const viewport = { w: 800, h: 600 };
      const step = { scale: 2, panX: 40, panY: -20, rotation: 0, centroidX: 400, centroidY: 300 };
      const next = applyPinchToCamera(camera, step, viewport, { min: 0.1, max: 12 });
      expect(next.zoom).toBeCloseTo(2, 10);
      expect(next.x).toBeCloseTo(-20, 10);
      expect(next.y).toBeCloseTo(10, 10);
    });

    it("applyPinchToCamera never leaves the surface's zoom bounds", () => {
      const viewport = { w: 800, h: 600 };
      const bounds = { min: 0.5, max: 3 };
      const inflated = applyPinchToCamera({ x: 0, y: 0, zoom: 2 }, { scale: 100, panX: 0, panY: 0, rotation: 0, centroidX: 400, centroidY: 300 }, viewport, bounds);
      const collapsed = applyPinchToCamera({ x: 0, y: 0, zoom: 2 }, { scale: 0.001, panX: 0, panY: 0, rotation: 0, centroidX: 400, centroidY: 300 }, viewport, bounds);
      expect(inflated.zoom).toBe(3);
      expect(collapsed.zoom).toBe(0.5);
    });
  });
}
