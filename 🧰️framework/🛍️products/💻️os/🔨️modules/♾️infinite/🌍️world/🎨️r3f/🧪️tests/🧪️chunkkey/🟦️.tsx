type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { BoxGeometry, HalfFloatType, LineBasicMaterial, LinearFilter, LinearSRGBColorSpace, MOUSE, Matrix4, Mesh, ORBIT_CAMERA_VIEW_COMMAND, Object3D, ThreeOrbitControls, ThreeOrthographicCamera, ThreePerspectiveCamera, Vector3, WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS, WORLD_CURVILINEAR_FRAGMENT_SHADER, WORLD_LOD_GRID_COVERAGE_MARGIN, WORLD_LOD_REFERENCE_FOV_DEG, WORLD_MESH_OUTLINE_USER_DATA_KEY, WORLD_ORBIT_CAMERA_MIN_FAR, WORLD_PROJECTION_COMMAND, WORLD_PROJECTION_KINDS, WORLD_REFERENCE_SELECTED_CONTENT_OPACITY, adaptiveOrbitCameraFar, applyOrbitProjectionToCameraState, applyWorldMeshEdgeBorders, applyWorldOrbitMouseButtonsIdle, applyWorldReferenceTransform, applyWorldVolumeTransform, cameraGridFadeDistance, cameraGridVisibleRadius, chunkDistanceVisible, chunkKey, classifyWorldNavigationGestures, computeOrbitCameraViewState, computeWorldProjectionPose, createOrbitCameraViewLayoutDescriptors, createOrbitCameraViewTemplates, createWorldProjectionTemplates, decodeWorldProjectionTemplateId, dispatchProjectionGizmoHit, encodeWorldProjectionTemplateId, floatingOriginRebase, frameWorldProjectionPose, lodFromCameraDistance, lodGridStepWorld, lodOrbitDistanceForCamera, orbitCameraDistance, orbitCameraViewGumballPlane, orbitCameraViewRigApplyToken, orbitViewToWorldProjectionSpec, patchWorldReferenceProps, projectionGizmoHeadFillColor, projectionGizmoHitVisualState, resetWorldMeshBorderColorCache, resolveOrbitCameraViewFromTemplateId, resolveOrbitGizmoViewFromDirection, resolveProjectionGizmoSpec, resolveProjectionGizmoVisualPalette, resolveWorldOrbitMouseButtonsIdle, resolveWorldOrbitRightMouseAction, sceneHostPort, shouldApplyOrbitCameraViewRigSeed, shouldAssignWorldOrbitRightMouse, tokenHex, worldCurvilinearUnproject, worldEntityInspectable, worldEntityRenderMode, worldEntityRendered, worldEntitySelectable, worldMeshBorderColor, worldObliqueShearMatrix, worldProjectionDefaults, worldProjectionFamily, worldProjectionGoalMatrix, worldProjectionGumballPlane, worldProjectionKindSwitchSpec, worldProjectionMatchedOrthoZoom, worldProjectionMatchedPerspectiveDistance, worldProjectionModeOptions, worldProjectionMorphMatrix, worldProjectionOrbitConstraints, worldProjectionPerspectiveFov, worldProjectionSnapZoom, worldProjectionSpecIconId, worldProjectionSpecLabel, worldProjectionSpecToOrbitView, worldProjectionSwitchTreeItems, worldProjectionTemplateApplySpec, worldProjectionTemplateSelectionId, worldProjectionTransitionPose, worldReferenceAppearance, worldSceneContentBounds, worldSceneContentBoundsKey, worldVolumesContainAabb } = dependencies;
  type OrbitCameraViewId = any;
  type ProjectionGizmoHit = any;
  type WorldNavigationSnapshot = any;
  type WorldProjectionSpec = any;
  type WorldProjectionTemplateDescriptor = any;
  type WorldReferenceProps = any;
  type WorldVolumeProps = any;

  const { describe, expect, it } = vitest;

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

  describe("resolveProjectionGizmoSpec", () => {
    it("dispatches a repeated gizmo orientation so an orbited camera snaps back", () => {
      const current = { mode: { kind: "orthographic" as const }, orientation: { type: "cardinal" as const, view: "top" as const } };
      const selectedSpecs: WorldProjectionSpec[] = [];
      const selectedViews: OrbitCameraViewId[] = [];
      dispatchProjectionGizmoHit({ type: "face", axis: "z", sign: 1 }, current, (spec) => selectedSpecs.push(spec), (view) => selectedViews.push(view));
      expect(selectedSpecs).toEqual([current]);
      expect(selectedViews).toEqual(["top"]);
    });

    it("sets orientation only and preserves the active projection mode", () => {
      expect(resolveProjectionGizmoSpec({ type: "face", axis: "z", sign: 1 }, { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })).toEqual({
        mode: { kind: "threePoint", fov: 50 },
        orientation: { type: "cardinal", view: "top" },
      });
      expect(resolveProjectionGizmoSpec({ type: "face", axis: "x", sign: -1 }, { mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } })).toEqual({
        mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 },
        orientation: { type: "cardinal", view: "left" },
      });
      expect(resolveProjectionGizmoSpec({ type: "corner", quadrant: "se", hemisphere: "upper" }, { mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } })).toEqual({
        mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" },
        orientation: { type: "corner", quadrant: "se", hemisphere: "upper" },
      });
      expect(resolveProjectionGizmoSpec({ type: "center" }, { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } })).toEqual({
        mode: { kind: "orthographic" },
        orientation: { type: "free" },
      });
      expect(resolveProjectionGizmoSpec({ type: "center" }, { mode: { kind: "twoPoint", fov: 42, verticalShift: 0 }, orientation: { type: "cardinal", view: "top" } })).toEqual({
        mode: { kind: "twoPoint", fov: 42, verticalShift: 0 },
        orientation: { type: "free" },
      });
    });

    it("preserves axonometric mode params when snapping corners", () => {
      const dimetric = { mode: { kind: "axonometric" as const, variant: "dimetric" as const, angleA: 15, angleB: 15 }, orientation: { type: "corner" as const, quadrant: "ne" as const, hemisphere: "upper" as const } };
      expect(resolveProjectionGizmoSpec({ type: "corner", quadrant: "sw", hemisphere: "lower" }, dimetric)).toEqual({
        mode: dimetric.mode,
        orientation: { type: "corner", quadrant: "sw", hemisphere: "lower" },
      });
    });

    it("composes every mode with gizmo top orientation (top works for fisheye, 2pt, axo, …)", () => {
      const top = { type: "cardinal" as const, view: "top" as const };
      for (const kind of WORLD_PROJECTION_KINDS) {
        const withTop = { mode: worldProjectionDefaults(kind).mode, orientation: top };
        const pose = computeWorldProjectionPose(withTop, { target: [0, 0, 0], distance: 100 });
        expect(pose.position[2]).toBeGreaterThan(0);
        expect(pose.projectionSpec).toEqual(withTop);
      }
      const fishTop = resolveProjectionGizmoSpec({ type: "face", axis: "z", sign: 1 }, worldProjectionDefaults("curvilinear"));
      expect(fishTop).toEqual({ mode: worldProjectionDefaults("curvilinear").mode, orientation: { type: "cardinal", view: "top" } });
    });
  });

  describe("orbitViewToWorldProjectionSpec / worldProjectionSpecToOrbitView", () => {
    it("round-trips orthographic and isometric orbit views", () => {
      expect(orbitViewToWorldProjectionSpec("front")).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } });
      expect(orbitViewToWorldProjectionSpec("isometricNw")).toEqual({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "nw", hemisphere: "upper" } });
      expect(worldProjectionSpecToOrbitView({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "right" } })).toBe("right");
      expect(worldProjectionSpecToOrbitView({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "se", hemisphere: "upper" } })).toBe("isometricSe");
    });
  });

  describe("worldProjectionKindSwitchSpec", () => {
    it("emits defaults for every projection kind", () => {
      for (const kind of WORLD_PROJECTION_KINDS) {
        expect(worldProjectionKindSwitchSpec(kind).mode.kind).toBe(kind);
      }
      expect(worldProjectionKindSwitchSpec("orthographic")).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } });
    });

    it("preserves perspective fov when switching kinds", () => {
      expect(worldProjectionKindSwitchSpec("twoPoint", { mode: { kind: "threePoint", fov: 72 }, orientation: { type: "free" } })).toEqual({ mode: { kind: "twoPoint", fov: 72, verticalShift: 0 }, orientation: { type: "free" } });
    });

    it("preserves gizmo orientation when switching mode", () => {
      expect(worldProjectionKindSwitchSpec("orthographic", { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } })).toEqual({
        mode: { kind: "orthographic" },
        orientation: { type: "cardinal", view: "front" },
      });
      expect(worldProjectionKindSwitchSpec("curvilinear", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toEqual({
        mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" },
        orientation: { type: "cardinal", view: "top" },
      });
    });
  });

  describe("worldProjectionTemplateSelectionId / worldProjectionTemplateApplySpec / worldProjectionSwitchTreeItems", () => {
    it("selects the same leaf ids the display template tree uses", () => {
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } })).toBe("orthographic");
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } })).toBe("orthographic");
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "axonometric", variant: "dimetric", angleA: 15, angleB: 15 }, orientation: { type: "corner", quadrant: "sw", hemisphere: "lower" } })).toBe("axonometric-dimetric");
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "onePoint", fov: 40 }, orientation: { type: "cardinal", view: "left" } })).toBe("one-point");
      expect(worldProjectionTemplateSelectionId({ mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "panini" }, orientation: { type: "free" } })).toBe("curvilinear");
    });

    it("preserves gizmo orientation when applying a template mode", () => {
      const axo = { mode: { kind: "axonometric" as const, variant: "isometric" as const, angleA: 30, angleB: 30 }, orientation: { type: "corner" as const, quadrant: "nw" as const, hemisphere: "lower" as const } };
      expect(worldProjectionTemplateApplySpec({ mode: { kind: "axonometric", variant: "dimetric", angleA: 15, angleB: 15 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }, axo)).toEqual({
        mode: { kind: "axonometric", variant: "dimetric", angleA: 15, angleB: 15 },
        orientation: { type: "corner", quadrant: "nw", hemisphere: "lower" },
      });
      expect(worldProjectionTemplateApplySpec({ mode: { kind: "onePoint", fov: 50 }, orientation: { type: "cardinal", view: "front" } }, { mode: { kind: "onePoint", fov: 72 }, orientation: { type: "cardinal", view: "top" } })).toEqual({
        mode: { kind: "onePoint", fov: 50 },
        orientation: { type: "cardinal", view: "top" },
      });
    });

    it("mirrors createWorldProjectionTemplates labels and ids in the switch tree", () => {
      const templates = createWorldProjectionTemplates({ controllerId: "demo" });
      const items = worldProjectionSwitchTreeItems(templates, () => undefined);
      expect(items.map((row) => row.id)).toEqual(["parallel", "perspective"]);
      expect(items[0]!.items!.map((row) => row.label)).toEqual(["Orthographic", "Axonometric", "Oblique"]);
      expect(items[0]!.items![0]!.items).toBeUndefined();
      expect(items[0]!.items![1]!.items!.map((row) => row.label)).toEqual(["Isometric", "Dimetric", "Trimetric"]);
      expect(items[1]!.items!.map((row) => row.label)).toEqual(["1-Point", "2-Point", "3-Point", "Curvilinear"]);
    });
  });

  describe("worldProjectionModeOptions", () => {
    it("lists axonometric/oblique/curvilinear flat variants for callers that still want a ribbon", () => {
      expect(worldProjectionModeOptions({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toEqual([]);
      expect(worldProjectionModeOptions({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } }).map((row) => row.label)).toEqual(["Iso", "Di", "Tri"]);
      expect(worldProjectionModeOptions({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } }).map((row) => row.label)).toEqual(["Cab", "Cav", "Mil"]);
      expect(worldProjectionModeOptions({ mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } }).map((row) => row.label)).toEqual(["Fish", "Pan"]);
    });
  });

  describe("resolveOrbitCameraViewFromTemplateId", () => {
    it("maps display-tree template ids to orbit views", () => {
      expect(resolveOrbitCameraViewFromTemplateId("top")).toBe("top");
      expect(resolveOrbitCameraViewFromTemplateId("orthographic-2d")).toBe("top");
      expect(resolveOrbitCameraViewFromTemplateId("perspective")).toBe("perspective");
      expect(resolveOrbitCameraViewFromTemplateId("missing")).toBeNull();
    });
  });

  describe("shouldApplyOrbitCameraViewRigSeed", () => {
    it("applies only when the seed token changes", () => {
      const token = orbitCameraViewRigApplyToken("win-a:3", "perspective");
      expect(shouldApplyOrbitCameraViewRigSeed(null, token)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(token, token)).toBe(false);
      expect(shouldApplyOrbitCameraViewRigSeed(token, orbitCameraViewRigApplyToken("win-a:4", "perspective"))).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(token, orbitCameraViewRigApplyToken("win-a:3", "orthographic"))).toBe(true);
    });

    it("re-applies when orbit controls become ready after the rig camera mounts", () => {
      const beforeControls = `${orbitCameraViewRigApplyToken("preview", "perspective")}:controls:0`;
      const afterControls = `${orbitCameraViewRigApplyToken("preview", "perspective")}:controls:1`;
      expect(shouldApplyOrbitCameraViewRigSeed(null, beforeControls)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(beforeControls, afterControls)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(afterControls, afterControls)).toBe(false);
    });
  });

  describe("applyOrbitProjectionToCameraState", () => {
    it("applies orthographic zoom defaults while preserving pose", () => {
      const state = applyOrbitProjectionToCameraState({ position: [100, 0, 50], target: [0, 0, 0], zoom: 1, projection: "perspective" }, "orthographic");
      expect(state.projection).toBe("orthographic");
      expect(state.zoom).toBe(50);
      expect(state.position).toEqual([100, 0, 50]);
    });
  });

  describe("createOrbitCameraViewTemplates", () => {
    it("emits the orthographic/perspective template tree", () => {
      const templates = createOrbitCameraViewTemplates({ controllerId: "demo" });
      expect(templates.map((row) => row.id)).toEqual(["orthographic", "perspective"]);
      const ortho2d = templates[0]!.children![0]!;
      expect(ortho2d.id).toBe("orthographic-2d");
      expect(ortho2d.children!.map((row) => row.id)).toEqual(["top", "bottom", "front", "back", "right", "left"]);
      const isometry = templates[0]!.children![1]!.children![0]!;
      expect(isometry.children!.map((row) => row.id)).toEqual(["isometricNe", "isometricNw", "isometricSe", "isometricSw"]);
      expect(templates[0]).toMatchObject({ controllerId: "demo", command: ORBIT_CAMERA_VIEW_COMMAND, args: { view: "top" } });
    });

    it("still supports flat custom view lists", () => {
      const templates = createOrbitCameraViewTemplates({ controllerId: "demo", views: ["top", "front"] });
      expect(templates.map((row) => row.id)).toEqual(["top", "front"]);
    });
  });

  describe("createOrbitCameraViewLayoutDescriptors", () => {
    it("includes grouped single and quad layouts", () => {
      const layouts = createOrbitCameraViewLayoutDescriptors();
      expect(layouts.some((row) => row.id === "view-quad-standard")).toBe(true);
      expect(layouts.find((row) => row.id === "view-single-top")?.groupPath).toEqual(["Single", "2D"]);
    });

    it("sizes Top | Perspective as one-third / two-thirds", () => {
      const layout = createOrbitCameraViewLayoutDescriptors().find((row) => row.id === "view-dual-plan-perspective");
      expect(layout?.arrangement).toEqual({
        kind: "row",
        panes: [
          { view: "top", size: 100 / 3 },
          { view: "perspective", size: 200 / 3 },
        ],
      });
    });
  });

  describe("worldProjectionFamily", () => {
    it("treats orthographic/axonometric/oblique as parallel and everything else as perspective", () => {
      expect(worldProjectionFamily({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toBe("parallel");
      expect(worldProjectionFamily({ mode: { kind: "axonometric", variant: "isometric", angleA: 30, angleB: 30 }, orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" } })).toBe("parallel");
      expect(worldProjectionFamily({ mode: { kind: "oblique", variant: "cavalier", angle: 45, depthScale: 1 }, orientation: { type: "cardinal", view: "front" } })).toBe("parallel");
      expect(worldProjectionFamily({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })).toBe("perspective");
      expect(worldProjectionFamily({ mode: { kind: "curvilinear", fov: 120, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } })).toBe("perspective");
      expect(worldProjectionFamily(undefined)).toBe("perspective");
    });
  });

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
      const bounds = worldSceneContentBounds([], [{ origin: [7, 0, 0.01], widthWorld: 50 }]);
      expect(bounds).toEqual({ center: [7, 0, 0.01], halfExtent: [25, 25, 0.5] });
      const state = frameWorldProjectionPose({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } }, bounds!, { viewportWidth: 400, viewportHeight: 800, padding: 1.35 });
      expect(state.target).toEqual([7, 0, 0.01]);
      expect(state.position[0]).toBe(7);
      expect(state.position[1]).toBe(0);
      expect(state.position[2]).toBeGreaterThan(0.01);
      expect(state.projection).toBe("orthographic");
      // visible half-width = (viewportWidth/2) / zoom = 25 * 1.35 ⇒ zoom = 200 / 33.75
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

  describe("worldProjectionSpecIconId", () => {
    it("maps orthographic and three-point specs to distinct projection icons", () => {
      expect(worldProjectionSpecIconId(worldProjectionDefaults("orthographic"))).toBe("projection-orthographic");
      expect(worldProjectionSpecIconId(worldProjectionDefaults("threePoint"))).toBe("projection-three-point");
      expect(worldProjectionSpecIconId(worldProjectionDefaults("orthographic"))).not.toBe(worldProjectionSpecIconId(worldProjectionDefaults("threePoint")));
    });
  });

  describe("createWorldProjectionTemplates", () => {
    it("emits Parallel/Perspective mode tree without gizmo orientations", () => {
      const templates = createWorldProjectionTemplates({ controllerId: "demo" });
      expect(templates.map((row) => row.id)).toEqual(["parallel", "perspective"]);
      const [parallel, perspective] = templates;
      expect(parallel!.children!.map((row) => row.id)).toEqual(["orthographic", "axonometric", "oblique"]);
      expect(parallel!.children![0]!.children).toBeUndefined();
      expect(parallel!.children![1]!.children!.map((row) => row.label)).toEqual(["Isometric", "Dimetric", "Trimetric"]);
      expect(parallel!.children![2]!.children!.map((row) => row.label)).toEqual(["Cabinet", "Cavalier", "Military"]);
      expect(perspective!.children!.map((row) => row.label)).toEqual(["1-Point", "2-Point", "3-Point", "Curvilinear"]);
      expect(parallel!.children![0]!.iconId).toBe("projection-orthographic");
      expect(perspective!.children![2]!.iconId).toBe("projection-three-point");
      expect(templates[0]).toMatchObject({ controllerId: "demo", command: WORLD_PROJECTION_COMMAND });
    });
  });

  describe("encodeWorldProjectionTemplateId / decodeWorldProjectionTemplateId", () => {
    it("round-trips a spec through the template id string", () => {
      const spec: WorldProjectionSpec = { mode: { kind: "axonometric", variant: "dimetric", angleA: 15, angleB: 15 }, orientation: { type: "corner", quadrant: "nw", hemisphere: "upper" } };
      const encoded = encodeWorldProjectionTemplateId(spec);
      expect(typeof encoded).toBe("string");
      expect(decodeWorldProjectionTemplateId(encoded)).toEqual(spec);
    });

    it("returns null for undefined, unrelated, or malformed template ids", () => {
      expect(decodeWorldProjectionTemplateId(undefined)).toBeNull();
      expect(decodeWorldProjectionTemplateId("top")).toBeNull();
      expect(decodeWorldProjectionTemplateId("world-projection:not-json")).toBeNull();
    });
  });

  describe("worldProjectionSpecLabel", () => {
    it("matches the exact labels createWorldProjectionTemplates uses for every leaf, so a dragged pane's title matches the tree entry it came from", () => {
      const templates = createWorldProjectionTemplates({ controllerId: "demo" });
      const collectLeafLabels = (nodes: readonly WorldProjectionTemplateDescriptor[]): readonly [string, WorldProjectionSpec][] =>
        nodes.flatMap((node) => (node.children?.length ? collectLeafLabels(node.children) : [[node.label, node.args.spec] as const]));
      for (const [label, spec] of collectLeafLabels(templates)) {
        expect(worldProjectionSpecLabel(spec)).toBe(label);
      }
      expect(worldProjectionSpecLabel({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } })).toBe("Orthographic");
    });
  });

  describe("floatingOriginRebase", () => {
    it("subtracts anchor", () => {
      expect(floatingOriginRebase([1000, 2000, 3], [1000, 1990, 0])).toEqual([0, 10, 3]);
    });
  });

  describe("lodFromCameraDistance", () => {
    it("maps orbit distance to scale ratio", () => {
      expect(lodFromCameraDistance(100, 100)).toBe(1);
      expect(lodFromCameraDistance(20000, 100)).toBe(200);
    });
  });

  describe("lodOrbitDistanceForCamera", () => {
    it("keeps perspective eye distance and maps orthographic zoom through the matched-perspective inverse so scroll retunes LOD", () => {
      const perspective = new ThreePerspectiveCamera(50, 16 / 9, 0.1, 500_000);
      perspective.position.set(0, -600, 200);
      expect(lodOrbitDistanceForCamera(perspective, 640, 720)).toBeCloseTo(640, 6);

      const orthographic = new ThreeOrthographicCamera(-360, 360, 360, -360, 0.1, 500_000);
      orthographic.position.set(0, 0, 600);
      orthographic.zoom = 50;
      const near = lodOrbitDistanceForCamera(orthographic, 600, 720);
      orthographic.zoom = 2;
      const far = lodOrbitDistanceForCamera(orthographic, 600, 720);
      expect(near).toBeCloseTo(worldProjectionMatchedPerspectiveDistance(WORLD_LOD_REFERENCE_FOV_DEG, 50, 720), 6);
      expect(far).toBeCloseTo(worldProjectionMatchedPerspectiveDistance(WORLD_LOD_REFERENCE_FOV_DEG, 2, 720), 6);
      expect(far).toBeGreaterThan(near * 10);
      expect(lodFromCameraDistance(far, 100)).toBeGreaterThan(lodFromCameraDistance(near, 100));
      expect(lodGridStepWorld(lodFromCameraDistance(far, 100), 10)).toBeGreaterThan(lodGridStepWorld(lodFromCameraDistance(near, 100), 10)!);
    });
  });

  describe("lodGridStepWorld", () => {
    it("keeps the configured spacing at close range and stays sparse at every larger LOD", () => {
      expect(lodGridStepWorld(1, 7.5)).toBe(7.5);
      expect(lodGridStepWorld(2, 10)).toBe(10);
      expect(lodGridStepWorld(10, 10)).toBe(50);
      expect(lodGridStepWorld(50, 10)).toBe(250);
      expect(lodGridStepWorld(500, 10)).toBe(2_500);
      expect(lodGridStepWorld(5_000, 10)).toBe(25_000);
      expect(lodGridStepWorld(Number.POSITIVE_INFINITY, 10)).toBeNull();
    });
  });

  describe("cameraGridFadeDistance", () => {
    it("grows with camera Z while fading fully before the far clipping plane", () => {
      const camera = new ThreePerspectiveCamera(45, 16 / 9, 0.1, 500_000);
      camera.position.set(0, -100, 100);
      const near = cameraGridFadeDistance(camera, 0, 10, { width: 800, height: 450 });
      camera.position.z = 2_000;
      const far = cameraGridFadeDistance(camera, 0, 10, { width: 800, height: 450 });
      expect(far).toBeGreaterThan(near);
      expect(far).toBeGreaterThanOrEqual(cameraGridVisibleRadius(camera, 0, { width: 800, height: 450 }) * WORLD_LOD_GRID_COVERAGE_MARGIN);
    });

    it("grows with orthographic zoom-out so the plane always overfills the viewport corners", () => {
      const camera = new ThreeOrthographicCamera(-400, 400, 300, -300, 0.1, 500_000);
      camera.position.set(0, 0, 600);
      const viewport = { width: 800, height: 600 };
      camera.zoom = 40;
      const zoomedInRadius = cameraGridVisibleRadius(camera, 0, viewport);
      const zoomedIn = cameraGridFadeDistance(camera, 0, 10, viewport);
      camera.zoom = 0.25;
      const zoomedOutRadius = cameraGridVisibleRadius(camera, 0, viewport);
      const zoomedOut = cameraGridFadeDistance(camera, 0, 10, viewport);
      expect(zoomedIn).toBeGreaterThanOrEqual(zoomedInRadius * WORLD_LOD_GRID_COVERAGE_MARGIN);
      expect(zoomedOut).toBeGreaterThan(zoomedIn);
      expect(zoomedOut).toBeGreaterThanOrEqual(zoomedOutRadius * WORLD_LOD_GRID_COVERAGE_MARGIN);
    });
  });

  describe("cameraGridVisibleRadius", () => {
    it("scales inversely with orthographic zoom and with perspective distance", () => {
      const orthographic = new ThreeOrthographicCamera(-400, 400, 300, -300, 0.1, 500_000);
      orthographic.zoom = 20;
      const near = cameraGridVisibleRadius(orthographic, 0, { width: 800, height: 600 });
      orthographic.zoom = 5;
      const far = cameraGridVisibleRadius(orthographic, 0, { width: 800, height: 600 });
      expect(far).toBeCloseTo(near * 4, 6);

      const perspective = new ThreePerspectiveCamera(50, 800 / 600, 0.1, 500_000);
      perspective.position.set(0, 0, 100);
      const close = cameraGridVisibleRadius(perspective, 0, { width: 800, height: 600 });
      perspective.position.z = 400;
      const distant = cameraGridVisibleRadius(perspective, 0, { width: 800, height: 600 });
      expect(distant).toBeCloseTo(close * 4, 6);
    });
  });

  describe("adaptiveOrbitCameraFar", () => {
    it("keeps the far plane ahead of extreme orbit zoom in stable power-of-two bands", () => {
      expect(adaptiveOrbitCameraFar(100)).toBe(524_288);
      expect(adaptiveOrbitCameraFar(2_000)).toBe(2_097_152);
      expect(adaptiveOrbitCameraFar(20_000)).toBe(33_554_432);
    });
  });

  describe("worldMeshBorderColor", () => {
    it("returns an srgb-compatible color", () => {
      resetWorldMeshBorderColorCache();
      const color = worldMeshBorderColor();
      expect(color.length).toBeGreaterThan(0);
      expect(color).not.toMatch(/^oklab\(/iu);
    });
  });

  describe("applyWorldMeshEdgeBorders", () => {
    it("adds one outline child per mesh", () => {
      const { BoxGeometry } = sceneHostPort.three;
      const root = new Object3D();
      const mesh = new Mesh(new BoxGeometry(1, 1, 1), new LineBasicMaterial());
      root.add(mesh);
      applyWorldMeshEdgeBorders(root, "#336699");
      expect(mesh.children).toHaveLength(1);
      expect(mesh.children[0]?.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY]).toBe(true);
      applyWorldMeshEdgeBorders(root, "#336699");
      expect(mesh.children).toHaveLength(1);
    });
  });

  describe("applyWorldReferenceTransform", () => {
    it("writes gumball pose onto reference props", () => {
      const base: WorldReferenceProps = {
        id: "ref-a",
        source: { url: "/infinite-fixture/🖼️sketch.png", mediaKind: "image" },
        origin: [0, 0, 0],
      };
      const next = applyWorldReferenceTransform(base, {
        position: [1, 2, 3],
        quaternion: [0, 0, 0, 1],
        scale: [2, 2, 2],
      });
      expect(next.origin).toEqual([1, 2, 3]);
      expect(next.scale).toEqual([2, 2, 2]);
    });
  });

  describe("worldReferenceAppearance", () => {
    it("returns hover and selection fills with halved content opacity when selected", () => {
      expect(worldReferenceAppearance({ asHover: false, showSelectedOutline: false }, 1)).toMatchObject({
        backgroundColor: null,
        contentOpacity: 1,
        outlineColor: null,
      });
      expect(worldReferenceAppearance({ asHover: true, showSelectedOutline: false }, 0.8)).toMatchObject({
        backgroundColor: expect.stringMatching(/^#/),
        contentOpacity: 0.8,
        outlineColor: expect.stringMatching(/^#/),
      });
      expect(worldReferenceAppearance({ asHover: true, showSelectedOutline: true }, 1)).toMatchObject({
        backgroundColor: tokenHex("primary"),
        contentOpacity: WORLD_REFERENCE_SELECTED_CONTENT_OPACITY,
        outlineColor: tokenHex("primary"),
      });
    });
  });

  describe("applyWorldVolumeTransform", () => {
    it("writes gumball pose onto volume props", () => {
      const base: WorldVolumeProps = { id: "vol-a", origin: [0, 0, 0] };
      const next = applyWorldVolumeTransform(base, {
        position: [1, 2, 3],
        quaternion: [0, 0, 0, 1],
        scale: [4, 6, 8],
      });
      expect(next.origin).toEqual([1, 2, 3]);
      expect(next.scale).toEqual([4, 6, 8]);
    });
  });

  describe("worldVolumesContainAabb", () => {
    it("returns true when no volumes are defined", () => {
      expect(worldVolumesContainAabb([], [0, 0, 0], [1, 1, 1])).toBe(true);
    });

    it("accepts an AABB fully inside a unit volume at the origin", () => {
      const volumes: WorldVolumeProps[] = [{ id: "v1", origin: [0, 0, 0], scale: [10, 10, 10] }];
      expect(worldVolumesContainAabb(volumes, [-2, -2, -2], [2, 2, 2])).toBe(true);
    });

    it("rejects an AABB that extends outside the volume", () => {
      const volumes: WorldVolumeProps[] = [{ id: "v1", origin: [0, 0, 0], scale: [4, 4, 4] }];
      expect(worldVolumesContainAabb(volumes, [-3, -3, -3], [3, 3, 3])).toBe(false);
    });
  });

  describe("worldReferencePose", () => {
    it("patchWorldReferenceProps updates origin rotation and scale", () => {
      const base: WorldReferenceProps = {
        id: "ref-a",
        source: { url: "/plan.png", mediaKind: "image" },
        origin: [0, 0, 0],
      };
      expect(patchWorldReferenceProps(base, "origin", [1, 2, 3])?.origin).toEqual([1, 2, 3]);
      const rotated = patchWorldReferenceProps(base, "rotation", [0, 90, 0]);
      expect(rotated?.orientation?.length).toBe(4);
      expect(patchWorldReferenceProps(base, "scaleUniform", 2)?.scale).toBe(2);
      expect(patchWorldReferenceProps(base, "widthWorld", 42)?.widthWorld).toBe(42);
    });
  });

  describe("worldEntityFlags", () => {
    it("treats hidden and locked entities as non-selectable", () => {
      expect(worldEntitySelectable(undefined)).toBe(true);
      expect(worldEntitySelectable({ hidden: true })).toBe(false);
      expect(worldEntitySelectable({ locked: true })).toBe(false);
    });

    it("excludes locked entities from canvas pick so a click equals background", () => {
      expect(worldEntitySelectable({ locked: true })).toBe(false);
      expect(worldEntityInspectable({ locked: true })).toBe(true);
    });

    it("treats hidden entities as non-inspectable and locked entities as inspectable", () => {
      expect(worldEntityInspectable(undefined)).toBe(true);
      expect(worldEntityInspectable({ hidden: true })).toBe(false);
      expect(worldEntityInspectable({ locked: true })).toBe(true);
    });

    it("reveals hidden entities on demand", () => {
      expect(worldEntityRendered({ hidden: true }, false)).toBe(false);
      expect(worldEntityRendered({ hidden: true }, true)).toBe(true);
      expect(worldEntityRendered({ locked: true }, false)).toBe(true);
    });

    it("resolves render mode for hidden reveal and locked dim", () => {
      expect(worldEntityRenderMode({ hidden: true }, { revealed: true, hovered: false })).toMatchObject({
        visible: true,
        asHover: true,
        dim: false,
        showSelectedOutline: false,
      });
      expect(worldEntityRenderMode({ locked: true }, { hovered: true, selected: true })).toMatchObject({
        visible: true,
        asHover: false,
        dim: true,
        showSelectedOutline: false,
      });
      expect(worldEntityRenderMode({ hidden: true, locked: true }, { revealed: true, hovered: true, selected: true })).toMatchObject({
        visible: true,
        asHover: true,
        dim: false,
        showSelectedOutline: false,
      });
    });
  });

}
