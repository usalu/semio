/** 📐️ The projection gumball plane, pose, transition, field of view, ortho zoom, matrices, gizmo hover, framing, orbit constraints and curvilinear suites of the infinite world r3f module. */

export async function registerProjectionPoseAndMatrixTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: import("../../../🟦️.tsx").WorldR3fTestDependencies): Promise<void> {
  const { HalfFloatType, LinearFilter, LinearSRGBColorSpace, Matrix4, ThreeOrthographicCamera, ThreePerspectiveCamera, WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS, WORLD_CURVILINEAR_FRAGMENT_SHADER, WORLD_ORBIT_CAMERA_MIN_FAR, computeWorldProjectionPose, frameWorldProjectionPose, orbitCameraDistance, orbitCameraViewGumballPlane, projectionGizmoHeadFillColor, projectionGizmoHitVisualState, resolveProjectionGizmoVisualPalette, worldCurvilinearUnproject, worldObliqueShearMatrix, worldProjectionGoalMatrix, worldProjectionGumballPlane, worldProjectionMatchedOrthoZoom, worldProjectionMatchedPerspectiveDistance, worldProjectionMorphMatrix, worldProjectionOrbitConstraints, worldProjectionPerspectiveFov, worldProjectionSnapZoom, worldProjectionTransitionPose, worldSceneContentBounds, worldSceneContentBoundsKey } = dependencies;
  const { describe, expect, it } = vitest;

  type ProjectionGizmoHit = import("../../../🟦️.tsx").ProjectionGizmoHit;

  describe("worldProjectionGumballPlane", () => {
    it("maps planar orthographic views to drafting planes and leaves 3D projections unconstrained", () => {
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "bottom" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } })).toBe("xz");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "back" } })).toBe("xz");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "left" } })).toBe("yz");
      expect(worldProjectionGumballPlane({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "right" } })).toBe("yz");
      expect(worldProjectionGumballPlane({ mode: { kind: "oblique", variant: "military", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "plan" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } })).toBe("xz");
      expect(worldProjectionGumballPlane({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } })).toBe("xz");
      expect(worldProjectionGumballPlane({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "left" } })).toBe("yz");
      expect(worldProjectionGumballPlane({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "top" } })).toBe("xy");
      expect(worldProjectionGumballPlane({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } })).toBeUndefined();
      expect(worldProjectionGumballPlane({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })).toBeUndefined();
      expect(worldProjectionGumballPlane(undefined)).toBeUndefined();
    });

    it("maps legacy orbit camera view ids the same way", () => {
      expect(orbitCameraViewGumballPlane("top")).toBe("xy");
      expect(orbitCameraViewGumballPlane("front")).toBe("xz");
      expect(orbitCameraViewGumballPlane("right")).toBe("yz");
      expect(orbitCameraViewGumballPlane("isometricNe")).toBeUndefined();
      expect(orbitCameraViewGumballPlane("perspective")).toBeUndefined();
      expect(orbitCameraViewGumballPlane(undefined)).toBeUndefined();
    });
  });

  describe("computeWorldProjectionPose", () => {
    it("places plan/top directly above the target with orthographic projection", () => {
      const state = computeWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }, { target: [0, 0, 40], distance: 800 });
      expect(state).toMatchObject({ position: [0, 0, 840], target: [0, 0, 40], up: [0, 1, 0], projection: "orthographic", zoom: 50 });
      const plan = computeWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } }, { target: [0, 0, 40], distance: 800 });
      expect(plan.position).toEqual(state.position);
    });

    it("accepts an explicit orthographic zoom including values below the legacy default", () => {
      const state = computeWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }, { target: [7, 0, 0], distance: 100, zoom: 6.4 });
      expect(state.zoom).toBe(6.4);
    });

    it("preserves live parallel zoom across gizmo snaps and upgrades perspective unit zoom", () => {
      expect(worldProjectionSnapZoom({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, 12.5, true)).toBe(12.5);
      expect(worldProjectionSnapZoom({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } }, 1, true)).toBe(1);
      expect(worldProjectionSnapZoom({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "sw", hemisphere: "upper" } }, 1, false)).toBe(50);
      expect(worldProjectionSnapZoom({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }, 50, true)).toBe(50);
    });

    it("derives the classic 35.264/45 isometric direction from the 30/30 axis angles", () => {
      const state = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, { target: [0, 0, 0], distance: 10 });
      expect(state.position[2] / 10).toBeCloseTo(Math.sin((35.264 * Math.PI) / 180), 3);
      const azimuth = (Math.atan2(state.position[0], state.position[1]) * 180) / Math.PI;
      expect(azimuth).toBeCloseTo(45, 2);
    });

    it("mirrors quadrant sign flips for axonometric corners", () => {
      const ne = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, { target: [0, 0, 0], distance: 10 });
      const sw = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "sw", hemisphere: "upper" } }, { target: [0, 0, 0], distance: 10 });
      expect(sw.position[0]).toBeCloseTo(-ne.position[0], 5);
      expect(sw.position[1]).toBeCloseTo(-ne.position[1], 5);
    });

    it("places lower-hemisphere axonometric corners below the target", () => {
      const upper = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, { target: [0, 0, 0], distance: 10 });
      const lower = computeWorldProjectionPose({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "lower" } }, { target: [0, 0, 0], distance: 10 });
      expect(upper.position[2]).toBeGreaterThan(0);
      expect(lower.position[2]).toBeLessThan(0);
      expect(lower.position[0]).toBeCloseTo(upper.position[0], 5);
      expect(lower.position[1]).toBeCloseTo(upper.position[1], 5);
    });

    it("keeps oblique cabinet/cavalier at the front pose and rotates military's up vector by its angle", () => {
      const cavalier = computeWorldProjectionPose({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } }, { target: [0, 0, 0], distance: 10 });
      expect(cavalier.position).toEqual([0, -10, 0]);
      const military = computeWorldProjectionPose({ mode: { kind: "oblique", variant: "military", angle: 30, depthScale: 1 }, orientation: { type: "cardinal", view: "plan" } }, { target: [0, 0, 0], distance: 10 });
      expect(military.position).toEqual([0, 0, 10]);
      expect(military.up![0]).toBeCloseTo(Math.sin((30 * Math.PI) / 180), 5);
    });

    it("reports perspective projection for the perspective family", () => {
      const threePoint = computeWorldProjectionPose({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }, { target: [0, 0, 0], distance: 100 });
      expect(threePoint.projection).toBe("perspective");
      expect(Math.hypot(...threePoint.position)).toBeGreaterThan(90);
    });
  });

  describe("worldProjectionTransitionPose", () => {
    it("keeps eye and target when only mode changes with the same orientation", () => {
      const live = {
        position: [12, -8, 40] as const,
        target: [3, 5, 10] as const,
        up: [0, 0, 1] as const,
        zoom: 6.4,
        isOrthographic: true,
        projectionSpec: { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "top" } } as const,
      };
      const pending = { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } } as const;
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.position).toEqual(live.position);
      expect(next.target).toEqual(live.target);
      expect(next.up).toEqual(live.up);
      expect(next.projection).toBe("orthographic");
      expect(next.projectionSpec).toEqual(pending);
      expect(next.zoom).toBe(6.4);
    });

    it("re-looks from the same target and distance when orientation changes", () => {
      const live = {
        position: [40, 20, 80] as const,
        target: [5, 5, 5] as const,
        up: [0, 0, 1] as const,
        zoom: 1,
        isOrthographic: false,
        projectionSpec: { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } } as const,
      };
      const pending = { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "top" } } as const;
      const distance = orbitCameraDistance({ position: [...live.position], target: [...live.target], zoom: live.zoom });
      const expected = computeWorldProjectionPose(pending, { target: [...live.target], distance, zoom: 1 });
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.target).toEqual(live.target);
      expect(next.position).toEqual(expected.position);
      expect(next.up).toEqual(expected.up);
      expect(next.projectionSpec).toEqual(pending);
    });

    it("matches apparent scale (not the legacy zoom-50 default) for a persp→ortho mode-only switch when a viewport is supplied", () => {
      const live = {
        position: [0, -100, 0] as const,
        target: [0, 0, 0] as const,
        up: [0, 0, 1] as const,
        zoom: 1,
        isOrthographic: false,
        projectionSpec: { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } } as const,
        viewport: { width: 800, height: 600 },
      };
      const pending = { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } } as const;
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.position).toEqual(live.position);
      expect(next.zoom).toBeCloseTo(worldProjectionMatchedOrthoZoom(50, 100, 600), 6);
      expect(next.zoom).not.toBe(50);
    });

    it("never moves the camera for an ortho→persp mode-only switch even when a viewport is supplied", () => {
      const live = {
        position: [0, -50, 0] as const,
        target: [0, 0, 0] as const,
        up: [0, 0, 1] as const,
        zoom: 10,
        isOrthographic: true,
        projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } } as const,
        viewport: { width: 800, height: 600 },
      };
      const pending = { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } } as const;
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.position).toEqual(live.position);
      expect(next.zoom).toBe(live.zoom);
    });

    it("never moves the camera across a perspective FOV change with the same orientation even when a viewport is supplied", () => {
      const live = {
        position: [0, -100, 0] as const,
        target: [0, 0, 0] as const,
        up: [0, 0, 1] as const,
        zoom: 1,
        isOrthographic: false,
        projectionSpec: { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } } as const,
        viewport: { width: 800, height: 600 },
      };
      const pending = { mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" as const }, orientation: { type: "cardinal", view: "front" } } as const;
      const next = worldProjectionTransitionPose(pending, live);
      expect(next.position).toEqual(live.position);
    });
  });

  describe("worldProjectionPerspectiveFov", () => {
    it("defaults threePoint/onePoint/twoPoint fov and caps curvilinear at 160°", () => {
      expect(worldProjectionPerspectiveFov({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })).toBe(50);
      expect(worldProjectionPerspectiveFov({ mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } })).toBe(120);
      expect(worldProjectionPerspectiveFov({ mode: { kind: "curvilinear", fov: 200, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } })).toBe(160);
    });
  });

  describe("worldProjectionMatchedOrthoZoom", () => {
    it("scales inversely with distance and directly with viewport height", () => {
      const zoom = worldProjectionMatchedOrthoZoom(50, 120, 720);
      expect(worldProjectionMatchedOrthoZoom(50, 240, 720)).toBeCloseTo(zoom / 2, 6);
      expect(worldProjectionMatchedOrthoZoom(50, 120, 1440)).toBeCloseTo(zoom * 2, 6);
    });

    it("round-trips with worldProjectionMatchedPerspectiveDistance", () => {
      const zoom = worldProjectionMatchedOrthoZoom(50, 180, 720);
      expect(worldProjectionMatchedPerspectiveDistance(50, zoom, 720)).toBeCloseTo(180, 6);
      expect(worldProjectionMatchedOrthoZoom(50, worldProjectionMatchedPerspectiveDistance(50, 40, 900), 900)).toBeCloseTo(40, 6);
    });
  });

  describe("worldObliqueShearMatrix", () => {
    it("shears cavalier/cabinet by depthScale·angle and military by a fixed unit length at a right angle", () => {
      const cavalier = worldObliqueShearMatrix({ kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 });
      expect(cavalier.elements[8]).toBeCloseTo(-Math.cos(Math.PI / 4), 5);
      expect(cavalier.elements[9]).toBeCloseTo(-Math.sin(Math.PI / 4), 5);
      const military = worldObliqueShearMatrix({ kind: "oblique", variant: "military", angle: 30, depthScale: 1 });
      expect(military.elements[8]).toBeCloseTo(0, 5);
      expect(military.elements[9]).toBeCloseTo(-1, 5);
    });

    it("ramps shear elements linearly with strength", () => {
      const full = worldObliqueShearMatrix({ kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, 1);
      const half = worldObliqueShearMatrix({ kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, 0.5);
      expect(half.elements[8]).toBeCloseTo(full.elements[8] * 0.5, 5);
      expect(half.elements[9]).toBeCloseTo(full.elements[9] * 0.5, 5);
    });
  });

  describe("worldProjectionGoalMatrix", () => {
    const viewport = { width: 800, height: 600 };

    it("matches a real OrthographicCamera's pixel frustum for parallel specs", () => {
      const spec = { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } } as const;
      const goal = worldProjectionGoalMatrix(spec, { zoom: 12.5, viewport, near: 0.2, far: WORLD_ORBIT_CAMERA_MIN_FAR });
      const cam = new ThreeOrthographicCamera(viewport.width / -2, viewport.width / 2, viewport.height / 2, viewport.height / -2, 0.2, WORLD_ORBIT_CAMERA_MIN_FAR);
      cam.zoom = 12.5;
      cam.updateProjectionMatrix();
      for (let i = 0; i < 16; i++) expect(goal.elements[i]).toBeCloseTo(cam.projectionMatrix.elements[i], 6);
    });

    it("matches a real PerspectiveCamera for perspective specs", () => {
      const spec = { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } } as const;
      const goal = worldProjectionGoalMatrix(spec, { zoom: 1, viewport, near: 0.2, far: WORLD_ORBIT_CAMERA_MIN_FAR });
      const cam = new ThreePerspectiveCamera(50, viewport.width / viewport.height, 0.2, WORLD_ORBIT_CAMERA_MIN_FAR);
      cam.zoom = 1;
      cam.updateProjectionMatrix();
      for (let i = 0; i < 16; i++) expect(goal.elements[i]).toBeCloseTo(cam.projectionMatrix.elements[i], 6);
    });

    it("post-multiplies the oblique shear onto the orthographic base", () => {
      const spec = { mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } } as const;
      const goal = worldProjectionGoalMatrix(spec, { zoom: 1, viewport });
      const base = worldProjectionGoalMatrix({ mode: { kind: "orthographic" }, orientation: spec.orientation }, { zoom: 1, viewport });
      const expected = base.clone().multiply(worldObliqueShearMatrix(spec.mode));
      for (let i = 0; i < 16; i++) expect(goal.elements[i]).toBeCloseTo(expected.elements[i], 6);
    });

    it("shifts the two-point vertical projection element", () => {
      const spec = { mode: { kind: "twoPoint", fov: 50, verticalShift: 0.3 }, orientation: { type: "free" } } as const;
      const goal = worldProjectionGoalMatrix(spec, { zoom: 1, viewport });
      const base = worldProjectionGoalMatrix({ mode: { kind: "threePoint", fov: 50 }, orientation: spec.orientation }, { zoom: 1, viewport });
      expect(goal.elements[9]).toBeCloseTo(base.elements[9] + 0.3, 6);
    });
  });

  describe("worldProjectionMorphMatrix", () => {
    it("equals from exactly at t=0 and closely approximates to at t=1", () => {
      const from = new Matrix4().makePerspective(-1, 1, 1, -1, 0.1, 100);
      const to = new Matrix4().makeOrthographic(-1, 1, 1, -1, 0.1, 100);
      expect(worldProjectionMorphMatrix(from, to, 0).elements).toEqual(from.elements);
      const atOne = worldProjectionMorphMatrix(from, to, 1);
      for (let i = 0; i < 16; i++) expect(atOne.elements[i]).toBeCloseTo(to.elements[i], 10);
    });

    it("interpolates every element linearly", () => {
      const from = new Matrix4().identity();
      const to = new Matrix4().identity().multiplyScalar(2);
      const mid = worldProjectionMorphMatrix(from, to, 0.5);
      for (let i = 0; i < 16; i++) expect(mid.elements[i]).toBeCloseTo((from.elements[i] + to.elements[i]) / 2, 10);
    });

    it("ramps oblique shear to half-magnitude at t=0.5 when morphing from an unsheared ortho matrix", () => {
      const viewport = { width: 800, height: 600 };
      const from = worldProjectionGoalMatrix({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } }, { zoom: 10, viewport });
      const goal = worldProjectionGoalMatrix({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } }, { zoom: 10, viewport });
      const mid = worldProjectionMorphMatrix(from, goal, 0.5);
      expect(mid.elements[8]).toBeCloseTo(goal.elements[8] * 0.5, 6);
      expect(mid.elements[9]).toBeCloseTo(goal.elements[9] * 0.5, 6);
    });
  });

  describe("projectionGizmoHover", () => {
    const topFace: ProjectionGizmoHit = { type: "face", axis: "z", sign: 1 };
    const freeCenter: ProjectionGizmoHit = { type: "center" };
    const palette = resolveProjectionGizmoVisualPalette();

    it("marks the hovered hit and dims the rest", () => {
      expect(projectionGizmoHitVisualState(topFace, topFace)).toBe("hover");
      expect(projectionGizmoHitVisualState(freeCenter, topFace)).toBe("dimmed");
      expect(projectionGizmoHitVisualState(topFace, null)).toBe("idle");
    });

    it("brightens axis and neutral fills on hover", () => {
      const axisHover = projectionGizmoHeadFillColor("#ff344f", "hover", palette, false);
      const neutralHover = projectionGizmoHeadFillColor("#9aa0ab", "hover", palette, true);
      expect(axisHover).not.toBe("#ff344f");
      expect(neutralHover).toBe(palette.neutralHover);
      expect(palette).not.toHaveProperty("labelColor");
    });
  });

  describe("frameWorldProjectionPose", () => {
    it("centers top orthographic on reference bounds and fits width in the viewport", () => {
      expect(worldSceneContentBounds([], [{ origin: [0, 0, 0] }])).toEqual({ center: [0, 0, 0], halfExtent: [5, 5, 0.5] });
      const bounds = worldSceneContentBounds([], [{ origin: [7, 0, 0.01], widthWorld: 50 }]);
      expect(bounds).toEqual({ center: [7, 0, 0.01], halfExtent: [25, 25, 0.5] });
      const state = frameWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }, bounds!, { viewportWidth: 400, viewportHeight: 800, padding: 1.35 });
      expect(state.target).toEqual([7, 0, 0.01]);
      expect(state.position[0]).toBe(7);
      expect(state.position[1]).toBe(0);
      expect(state.position[2]).toBeGreaterThan(0.01);
      expect(state.projection).toBe("orthographic");
      expect(state.zoom).toBeCloseTo(200 / (25 * 1.35), 5);
    });

    it("worldSceneContentBoundsKey changes when fill expands the scene footprint", () => {
      const seed = worldSceneContentBounds([{ position: [0, 0, 0] }]);
      const filled = worldSceneContentBounds([{ position: [0, 0, 0] }, { position: [40, -12, 3] }]);
      expect(worldSceneContentBoundsKey(seed)).not.toBe(worldSceneContentBoundsKey(filled));
      expect(worldSceneContentBoundsKey(filled)).toBe(worldSceneContentBoundsKey(filled));
    });
  });

  describe("worldProjectionOrbitConstraints", () => {
    it("locks rotation for plan, oblique, and one-point; locks polar for two-point; frees the rest", () => {
      expect(worldProjectionOrbitConstraints({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } }).rotate).toBe(false);
      expect(worldProjectionOrbitConstraints({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }).rotate).toBe(true);
      expect(worldProjectionOrbitConstraints({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } }).rotate).toBe(false);
      expect(worldProjectionOrbitConstraints({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } }).rotate).toBe(false);
      const twoPoint = worldProjectionOrbitConstraints({ mode: { kind: "twoPoint", fov: 50, verticalShift: 0 }, orientation: { type: "free" } });
      expect(twoPoint.minPolar).toBeCloseTo(Math.PI / 2);
      expect(twoPoint.maxPolar).toBeCloseTo(Math.PI / 2);
      expect(worldProjectionOrbitConstraints({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }).rotate).toBe(true);
    });
  });

  describe("worldCurvilinearUnproject", () => {
    it("is the identity mapping at strength 0 (rectilinear passthrough)", () => {
      const mode = { kind: "curvilinear" as const, fov: 120, strength: 0, mapping: "fisheye" as const };
      const [x, y] = worldCurvilinearUnproject([0.3, 0.2], mode);
      expect(x).toBeCloseTo(0.3, 5);
      expect(y).toBeCloseTo(0.2, 5);
    });
  });

  describe("WorldCurvilinearPass capture quality", () => {
    it("stores the capture in linear working space with linear filtering (no nearest-pixelation)", () => {
      expect(WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS.magFilter).toBe(LinearFilter);
      expect(WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS.minFilter).toBe(LinearFilter);
      expect(WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS.colorSpace).toBe(LinearSRGBColorSpace);
      expect(WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS.type).toBe(HalfFloatType);
    });

    it("applies Three output color-space conversion on the fisheye blit", () => {
      expect(WORLD_CURVILINEAR_FRAGMENT_SHADER).toContain("#include <colorspace_fragment>");
      expect(WORLD_CURVILINEAR_FRAGMENT_SHADER).toContain("texture2D(tCapture, sourceUv)");
    });
  });
}
