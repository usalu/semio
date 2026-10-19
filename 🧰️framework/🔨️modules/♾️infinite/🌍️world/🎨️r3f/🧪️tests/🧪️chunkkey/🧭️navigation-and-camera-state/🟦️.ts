/** 🧭️ The chunk key, chunk visibility, orbit mouse buttons and navigation gestures suites, and the orbit camera view state and gizmo direction suites of the infinite world r3f module. */

export async function registerNavigationTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: import("../../../🟦️.tsx").WorldR3fTestDependencies): Promise<void> {
  const { MOUSE, ThreeOrbitControls, ThreeOrthographicCamera, Vector3, applyWorldOrbitMouseButtonsIdle, chunkDistanceVisible, chunkKey, classifyWorldNavigationGestures, computeOrbitCameraViewState, resolveOrbitGizmoViewFromDirection, resolveWorldOrbitMouseButtonsIdle, resolveWorldOrbitRightMouseAction, shouldAssignWorldOrbitRightMouse } = dependencies;
  const { describe, expect, it } = vitest;

  type WorldNavigationSnapshot = import("../../../🟦️.tsx").WorldNavigationSnapshot;

  describe("chunkKey", () => {
    it("buckets origins by chunk size", () => {
      expect(chunkKey([0, 0, 0], 256)).toBe("0|0|0");
      expect(chunkKey([256, 0, 0], 256)).toBe("1|0|0");
    });
  });

  describe("chunkDistanceVisible", () => {
    it("uses hysteresis for exit", () => {
      const cam = new Vector3(0, 0, 0);
      const center = new Vector3(500, 0, 0);
      expect(chunkDistanceVisible({ camPos: cam, chunkCenter: center, chunkSize: 256, maxDist: 100, wasVisible: false })).toBe(false);
      expect(chunkDistanceVisible({ camPos: cam, chunkCenter: center, chunkSize: 256, maxDist: 1000, wasVisible: true })).toBe(true);
    });
  });

  describe("resolveWorldOrbitMouseButtonsIdle", () => {
    it("maps an unmodified middle click to pan in every projection", () => {
      expect(resolveWorldOrbitMouseButtonsIdle("orthographic")).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
      expect(resolveWorldOrbitMouseButtonsIdle("perspective")).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
    });

    it("pans an orthographic plan camera on an unmodified middle drag", () => {
      const canvas = document.createElement("canvas");
      Object.defineProperties(canvas, {
        clientWidth: { value: 400 },
        clientHeight: { value: 300 },
        setPointerCapture: { value: () => undefined },
        releasePointerCapture: { value: () => undefined },
      });
      const camera = new ThreeOrthographicCamera(-200, 200, 150, -150, 0.1, 1_000);
      camera.position.set(0, 0, 10);
      camera.lookAt(0, 0, 0);
      const controls = new ThreeOrbitControls(camera, canvas);
      applyWorldOrbitMouseButtonsIdle(controls, "orthographic");
      const offsetBefore = camera.position.clone().sub(controls.target);
      const pointer = (kind: string, x: number, y: number, button: number) => {
        const event = new MouseEvent(kind, { bubbles: true, button, clientX: x, clientY: y });
        Object.defineProperties(event, { pointerId: { value: 1 }, pointerType: { value: "mouse" }, pageX: { value: x }, pageY: { value: y } });
        return event;
      };
      canvas.dispatchEvent(pointer("pointerdown", 100, 100, 1));
      document.dispatchEvent(pointer("pointermove", 140, 125, 1));
      document.dispatchEvent(pointer("pointerup", 140, 125, 1));
      const offsetAfter = camera.position.clone().sub(controls.target);
      expect(controls.target.length()).toBeGreaterThan(0);
      expect(offsetAfter.distanceTo(offsetBefore)).toBeLessThan(1e-9);
      controls.dispose();
    });

    it("always maps middle click to pan when rotation is disabled (plan/oblique/one-point projections), even in the orthographic family", () => {
      expect(resolveWorldOrbitMouseButtonsIdle("orthographic", false)).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
      expect(resolveWorldOrbitMouseButtonsIdle("perspective", false)).toEqual({ LEFT: null, MIDDLE: MOUSE.PAN, RIGHT: null });
    });
  });

  describe("classifyWorldNavigationGestures", () => {
    const base: WorldNavigationSnapshot = { position: [0, 0, 10], target: [0, 0, 0], zoom: 1, projection: "perspective" };

    it("detects pan when only the target translates", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [5, 0, 10], target: [5, 0, 0] };
      expect(classifyWorldNavigationGestures(base, after)).toEqual(["pan"]);
    });

    it("detects zoom via distance change in perspective projection", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [0, 0, 5] };
      expect(classifyWorldNavigationGestures(base, after)).toEqual(["zoom"]);
    });

    it("detects zoom via the zoom factor in orthographic projection, ignoring unchanged distance", () => {
      const orthoBase: WorldNavigationSnapshot = { ...base, projection: "orthographic", zoom: 1 };
      const after: WorldNavigationSnapshot = { ...orthoBase, zoom: 1.5 };
      expect(classifyWorldNavigationGestures(orthoBase, after)).toEqual(["zoom"]);
    });

    it("detects orbit when the camera direction around the target rotates", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [10, 0, 0] };
      expect(classifyWorldNavigationGestures(base, after)).toEqual(["orbit"]);
    });

    it("returns no gestures when every delta is below threshold", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [0, 0, 10.001], target: [0.001, 0, 0] };
      expect(classifyWorldNavigationGestures(base, after)).toEqual([]);
    });

    it("never misreads a pure pan as orbit, and a pure orbit as pan", () => {
      const panned: WorldNavigationSnapshot = { ...base, position: [5, 0, 10], target: [5, 0, 0] };
      expect(classifyWorldNavigationGestures(base, panned)).not.toContain("orbit");
      const orbited: WorldNavigationSnapshot = { ...base, position: [10, 0, 0] };
      expect(classifyWorldNavigationGestures(base, orbited)).not.toContain("pan");
    });

    it("can report multiple gestures from a single combined movement", () => {
      const after: WorldNavigationSnapshot = { ...base, position: [10, 0, 0], target: [0, 0, 0] };
      const gestures = classifyWorldNavigationGestures({ ...base, position: [0, 0, 20] }, after);
      expect(gestures).toContain("orbit");
      expect(gestures).toContain("zoom");
    });
  });

  describe("resolveWorldOrbitRightMouseAction", () => {
    it("reserves plain right click for context menu and maps modifiers to orbit and pan", () => {
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: false, shiftKey: false })).toBeNull();
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: true, shiftKey: false }, "perspective")).toBe(MOUSE.ROTATE);
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: true, shiftKey: false }, "orthographic")).toBe(MOUSE.ROTATE);
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: false, shiftKey: true })).toBe(MOUSE.PAN);
      expect(resolveWorldOrbitRightMouseAction({ button: 2, altKey: true, shiftKey: true })).toBe(MOUSE.PAN);
      expect(resolveWorldOrbitRightMouseAction({ button: 0, altKey: true, shiftKey: false })).toBeNull();
    });
  });

  describe("shouldAssignWorldOrbitRightMouse", () => {
    it("suppresses orbit's own button assignment when onRightPointerDown returns false", () => {
      const event = { clientX: 10, clientY: 20 } as PointerEvent;
      expect(shouldAssignWorldOrbitRightMouse(event, () => false)).toBe(false);
      expect(shouldAssignWorldOrbitRightMouse(event, () => true)).toBe(true);
      expect(shouldAssignWorldOrbitRightMouse(event, undefined)).toBe(true);
    });
  });

  describe("computeOrbitCameraViewState", () => {
    it("places top view directly above the target with orthographic projection", () => {
      const state = computeOrbitCameraViewState("top", { target: [0, 0, 40], distance: 800 });
      expect(state).toMatchObject({ position: [0, 0, 840], target: [0, 0, 40], up: [0, 1, 0], projection: "orthographic", zoom: 50 });
    });

    it("maps north to +Y and perspective to an oblique direction", () => {
      const north = computeOrbitCameraViewState("north", { target: [0, 0, 0], distance: 100 });
      expect(north.position).toEqual([0, 100, 0]);
      expect(north.projection).toBe("orthographic");
      const perspective = computeOrbitCameraViewState("perspective", { target: [0, 0, 0], distance: 100 });
      expect(perspective.projection).toBe("perspective");
      expect(Math.hypot(...perspective.position)).toBeGreaterThan(90);
    });
  });

  describe("resolveOrbitGizmoViewFromDirection", () => {
    it("maps dominant axis clicks to orthographic orbit views", () => {
      expect(resolveOrbitGizmoViewFromDirection({ x: 1, y: 0.1, z: 0.05 })).toBe("right");
      expect(resolveOrbitGizmoViewFromDirection({ x: -1, y: 0, z: 0 })).toBe("left");
      expect(resolveOrbitGizmoViewFromDirection({ x: 0.1, y: 0.2, z: 1 })).toBe("top");
      expect(resolveOrbitGizmoViewFromDirection({ x: 0, y: 0, z: -1 })).toBe("bottom");
      expect(resolveOrbitGizmoViewFromDirection({ x: 0, y: -1, z: 0.1 })).toBe("front");
      expect(resolveOrbitGizmoViewFromDirection({ x: 0.05, y: 1, z: 0.1 })).toBe("back");
    });
  });
}
