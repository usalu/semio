/** 🏷️ The projection icon ids, templates, template id codec and labels, and the floating origin, level of detail, grid and far plane suites of the infinite world r3f module. */

export async function registerTemplateLabelAndLodTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: import("../../../🟦️.tsx").WorldR3fTestDependencies): Promise<void> {
  const { ThreeOrthographicCamera, ThreePerspectiveCamera, WORLD_LOD_GRID_COVERAGE_MARGIN, WORLD_LOD_REFERENCE_FOV_DEG, WORLD_PROJECTION_COMMAND, adaptiveOrbitCameraFar, cameraGridFadeDistance, cameraGridVisibleRadius, createWorldProjectionTemplates, decodeWorldProjectionTemplateId, encodeWorldProjectionTemplateId, floatingOriginRebase, lodFromCameraDistance, lodGridStepWorld, lodOrbitDistanceForCamera, worldProjectionDefaults, worldProjectionMatchedPerspectiveDistance, worldProjectionSpecIconId, worldProjectionSpecLabel } = dependencies;
  const { describe, expect, it } = vitest;

  type WorldProjectionSpec = import("../../../🟦️.tsx").WorldProjectionSpec;
  type WorldProjectionTemplateDescriptor = import("../../../🟦️.tsx").WorldProjectionTemplateDescriptor;

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
}
