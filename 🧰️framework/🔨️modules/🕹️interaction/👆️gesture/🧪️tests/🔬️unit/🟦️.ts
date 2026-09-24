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
    | "GestureRecognizer"
    | "IDLE_GESTURE_RECOGNIZER"
    | "applyPinchToCamera"
    | "applyPinchToOffsetCamera"
    | "applyPinchToOrbit"
    | "clampZoom"
    | "gestureIsMultiTouch"
    | "gesturePointerDown"
    | "gesturePointerMove"
    | "gesturePointerUp"
    | "pinchFrame"
    | "pinchStep"
    | "pinchWheelDelta"
    | "pinchZoomNotches"
    | "recognizeGesturePointerDown"
    | "recognizeGesturePointerMove"
    | "recognizeGesturePointerUp"
    | "shortestAngleDelta"
    | "zoomAboutPoint"
  >,
  source: TestSource,
): Promise<void> {
  const {
    GestureRecognizer,
    IDLE_GESTURE_RECOGNIZER,
    applyPinchToOrbit,
    applyPinchToOffsetCamera,
    pinchZoomNotches,
    recognizeGesturePointerDown,
    recognizeGesturePointerMove,
    recognizeGesturePointerUp,
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
  const { readFileSync } = await import("node:fs");
  const threeOracle = await import("three");
  const { join } = await import("node:path");
  type FixtureEvent = { readonly type: "down" | "move" | "up"; readonly pointerId: number; readonly x?: number; readonly y?: number };
  type FixtureVerdict = { readonly kind: string } & Readonly<Record<string, number | string>>;
  const fixture = JSON.parse(readFileSync(join(source.directory, "🧫️fixtures", "🤏️recognizer.json"), "utf8")) as {
    readonly recognizer: readonly { readonly name: string; readonly events: readonly FixtureEvent[]; readonly verdicts: readonly FixtureVerdict[] }[];
    readonly notches: readonly { readonly name: string; readonly pendingLogScale: number; readonly scale: number; readonly in: number; readonly out: number; readonly notches: number; readonly remainderScale: number }[];
  };
  const verdictFields = (verdict: ReturnType<InstanceType<typeof GestureRecognizer>["down"]>): Readonly<Record<string, number | string>> =>
    verdict.kind === "pinch" ? { kind: verdict.kind, ...verdict.step } : verdict.kind === "pinchBegin" ? { kind: verdict.kind, ...verdict.frame } : { kind: verdict.kind };

  describe("🧭️ gesture recognizer — 🧫️fixtures/🤏️recognizer.json", () => {
    for (const row of fixture.recognizer) {
      it(`${row.name} (class cell)`, () => {
        const recognizer = new GestureRecognizer();
        row.events.forEach((event, index) => {
          const verdict = event.type === "down" ? recognizer.down({ pointerId: event.pointerId, x: event.x ?? 0, y: event.y ?? 0 }) : event.type === "move" ? recognizer.move({ pointerId: event.pointerId, x: event.x ?? 0, y: event.y ?? 0 }) : recognizer.up(event.pointerId);
          const expected = row.verdicts[index]!;
          const actual = verdictFields(verdict);
          expect(actual.kind, `${row.name} event ${index}`).toBe(expected.kind);
          for (const [key, value] of Object.entries(expected)) {
            if (key === "kind") continue;
            expect(actual[key] as number, `${row.name} event ${index} ${key}`).toBeCloseTo(value as number, 9);
          }
        });
      });
      it(`${row.name} (pure transitions agree with the class cell)`, () => {
        let state = IDLE_GESTURE_RECOGNIZER;
        const recognizer = new GestureRecognizer();
        for (const event of row.events) {
          const pointer = { pointerId: event.pointerId, x: event.x ?? 0, y: event.y ?? 0 };
          const transition = event.type === "down" ? recognizeGesturePointerDown(state, pointer) : event.type === "move" ? recognizeGesturePointerMove(state, pointer) : recognizeGesturePointerUp(state, event.pointerId);
          const verdict = event.type === "down" ? recognizer.down(pointer) : event.type === "move" ? recognizer.move(pointer) : recognizer.up(event.pointerId);
          state = transition.state;
          expect(transition.verdict).toEqual(verdict);
          expect(recognizer.state).toEqual(state);
        }
      });
    }

    it("the latch releases to the idle state once every contact lifted", () => {
      const recognizer = new GestureRecognizer();
      recognizer.down({ pointerId: 1, x: 0, y: 0 });
      recognizer.down({ pointerId: 2, x: 10, y: 0 });
      expect(recognizer.latched).toBe(true);
      recognizer.up(1);
      expect(recognizer.latched).toBe(true);
      expect(recognizer.up(2).kind).toBe("pinchEnd");
      expect(recognizer.state).toBe(IDLE_GESTURE_RECOGNIZER);
      expect(recognizer.pointers).toEqual([]);
    });
  });

  describe("🛰️ orbit pinch law — validated against three.js camera basis (third-party oracle)", () => {
    const { PerspectiveCamera, Vector3 } = threeOracle;
    const bounds = { distance: { min: 0.1, max: 1000 }, zoom: { min: 0.01, max: 100 } };
    const pose = { position: [4, 3, 12] as const, target: [1, -1, 2] as const, up: [0, 1, 0] as const, zoom: 1 };
    const identity = { scale: 1, panX: 0, panY: 0, rotation: 0, centroidX: 0, centroidY: 0 };

    it("a pure two-finger pan moves eye and target by three's screen-space pan (camera matrix columns 0/1)", () => {
      const fov = 50;
      const height = 600;
      const panX = 37;
      const panY = -21;
      const camera = new PerspectiveCamera(fov, 1.5, 0.1, 1000);
      camera.position.set(...pose.position);
      camera.up.set(...pose.up);
      camera.lookAt(new Vector3(...pose.target));
      camera.updateMatrix();
      const offset = new Vector3(...pose.position).sub(new Vector3(...pose.target));
      const targetDistance = offset.length() * Math.tan(((fov / 2) * Math.PI) / 180);
      const left = new Vector3().setFromMatrixColumn(camera.matrix, 0).multiplyScalar(-((2 * panX * targetDistance) / height));
      const up = new Vector3().setFromMatrixColumn(camera.matrix, 1).multiplyScalar((2 * panY * targetDistance) / height);
      const expectedTarget = new Vector3(...pose.target).add(left).add(up);
      const next = applyPinchToOrbit(pose, { ...identity, panX, panY }, { kind: "perspective", fovYRadians: (fov * Math.PI) / 180 }, height, bounds);
      expect(next.target[0]).toBeCloseTo(expectedTarget.x, 9);
      expect(next.target[1]).toBeCloseTo(expectedTarget.y, 9);
      expect(next.target[2]).toBeCloseTo(expectedTarget.z, 9);
      expect(next.position[0] - next.target[0]).toBeCloseTo(offset.x, 9);
      expect(next.position[1] - next.target[1]).toBeCloseTo(offset.y, 9);
      expect(next.position[2] - next.target[2]).toBeCloseTo(offset.z, 9);
    });

    it("spreading the fingers dollies toward the target by the separation ratio, inside the distance bounds", () => {
      const distance = Math.hypot(3, 4, 10);
      const next = applyPinchToOrbit(pose, { ...identity, scale: 2 }, { kind: "perspective", fovYRadians: 1 }, 600, bounds);
      expect(Math.hypot(next.position[0] - next.target[0], next.position[1] - next.target[1], next.position[2] - next.target[2])).toBeCloseTo(distance / 2, 9);
      expect(next.target).toEqual(pose.target);
      const clamped = applyPinchToOrbit(pose, { ...identity, scale: 1e6 }, { kind: "perspective", fovYRadians: 1 }, 600, bounds);
      expect(Math.hypot(clamped.position[0] - clamped.target[0], clamped.position[1] - clamped.target[1], clamped.position[2] - clamped.target[2])).toBeCloseTo(0.1, 9);
    });

    it("an orthographic camera zooms by the factor instead of moving the eye", () => {
      const next = applyPinchToOrbit(pose, { ...identity, scale: 3 }, { kind: "orthographic", frustumHeight: 20 }, 600, bounds);
      expect(next.zoom).toBeCloseTo(3, 12);
      expect(next.position).toEqual(pose.position);
    });

    it("a degenerate pose (eye on target, or looking along up) is returned unchanged", () => {
      const onTarget = { ...pose, position: pose.target };
      expect(applyPinchToOrbit(onTarget, { ...identity, scale: 2 }, { kind: "perspective", fovYRadians: 1 }, 600, bounds)).toBe(onTarget);
      const alongUp = { ...pose, position: [1, 9, 2] as const };
      expect(applyPinchToOrbit(alongUp, { ...identity, scale: 2 }, { kind: "perspective", fovYRadians: 1 }, 600, bounds)).toBe(alongUp);
    });
  });

  describe("🔍️ step-quantized pinch zoom — 🧫️fixtures/🤏️recognizer.json", () => {
    for (const row of fixture.notches) {
      it(row.name, () => {
        const result = pinchZoomNotches(row.pendingLogScale, row.scale, { in: row.in, out: row.out });
        expect(result.notches).toBe(row.notches);
        expect(Math.exp(result.pendingLogScale)).toBeCloseTo(row.remainderScale, 9);
      });
    }

    it("replaying the notches reproduces the pinch scale within one notch", () => {
      let pending = 0;
      let zoom = 1;
      for (const scale of [1.02, 1.03, 1.07, 1.01, 0.97, 1.2, 1.15]) {
        const result = pinchZoomNotches(pending, scale, { in: 1.1, out: 0.9 });
        pending = result.pendingLogScale;
        zoom *= result.notches >= 0 ? 1.1 ** result.notches : 0.9 ** -result.notches;
      }
      const target = 1.02 * 1.03 * 1.07 * 1.01 * 0.97 * 1.2 * 1.15;
      expect(Math.abs(Math.log(zoom / target))).toBeLessThan(Math.log(1 / 0.9));
    });
  });

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

    it("applyPinchToCamera keeps the world point under the PREVIOUS centroid under the new centroid (fingers hold the content)", () => {
      const camera = { x: 0, y: 0, zoom: 1 };
      const viewport = { w: 800, h: 600 };
      const step = { scale: 2, panX: 40, panY: -20, rotation: 0, centroidX: 400, centroidY: 300 };
      const next = applyPinchToCamera(camera, step, viewport, { min: 0.1, max: 12 });
      expect(next.zoom).toBeCloseTo(2, 10);
      expect(next.x).toBeCloseTo(-40, 10);
      expect(next.y).toBeCloseTo(20, 10);
      const toScreen = (cam: { x: number; y: number; zoom: number }, world: { x: number; y: number }) => ({ x: (world.x - cam.x) * cam.zoom + viewport.w / 2, y: (world.y - cam.y) * cam.zoom + viewport.h / 2 });
      const heldWorld = { x: camera.x + (360 - 400) / camera.zoom, y: camera.y + (320 - 300) / camera.zoom };
      expect(toScreen(next, heldWorld).x).toBeCloseTo(400, 10);
      expect(toScreen(next, heldWorld).y).toBeCloseTo(300, 10);
    });

    it("applyPinchToOffsetCamera holds the same finger anchor in the screen-offset transform (screen = world * zoom + camera)", () => {
      const camera = { x: 30, y: -12, zoom: 1.5 };
      const step = { scale: 1.25, panX: -18, panY: 9, rotation: 0, centroidX: 210, centroidY: 140 };
      const next = applyPinchToOffsetCamera(camera, step, { min: 0.1, max: 12 });
      const heldWorld = { x: (step.centroidX - step.panX - camera.x) / camera.zoom, y: (step.centroidY - step.panY - camera.y) / camera.zoom };
      expect(next.zoom).toBeCloseTo(1.875, 12);
      expect(heldWorld.x * next.zoom + next.x).toBeCloseTo(step.centroidX, 10);
      expect(heldWorld.y * next.zoom + next.y).toBeCloseTo(step.centroidY, 10);
      expect(applyPinchToOffsetCamera(camera, { ...step, scale: 1e9 }, { min: 0.1, max: 12 }).zoom).toBe(12);
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
