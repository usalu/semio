/** 🎛️ The projection gizmo spec, the orbit view and kind switch specs, the template selection and mode options, the rig seed and the orbit view templates and layout descriptors suites of the infinite world r3f module. */
import { act, createElement } from "react";
import { createRoot } from "react-dom/client";

export async function registerGizmoSpecAndViewTemplateTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: import("../../../🟦️.tsx").WorldR3fTestDependencies): Promise<void> {
  const { ORBIT_CAMERA_VIEW_COMMAND, ThreePerspectiveCamera, Vector3, WORLD_PROJECTION_KINDS, WorldOrbitCameraViewRigSeedApply, applyOrbitProjectionToCameraState, computeWorldProjectionPose, createOrbitCameraViewLayoutDescriptors, createOrbitCameraViewTemplates, createWorldProjectionTemplates, dispatchProjectionGizmoHit, orbitCameraViewRigApplyToken, orbitViewToWorldProjectionSpec, resolveOrbitCameraViewFromTemplateId, resolveProjectionGizmoSpec, shouldApplyOrbitCameraViewRigSeed, worldProjectionDefaults, worldProjectionFamily, worldProjectionKindSwitchSpec, worldProjectionModeOptions, worldProjectionSpecToOrbitView, worldProjectionSwitchTreeItems, worldProjectionTemplateApplySpec, worldProjectionTemplateSelectionId } = dependencies;
  const { describe, expect, it, vi } = vitest;

  type OrbitCameraViewId = import("../../../🟦️.tsx").OrbitCameraViewId;
  type WorldProjectionSpec = import("../../../🟦️.tsx").WorldProjectionSpec;

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
      const items = worldProjectionSwitchTreeItems("framework.worldOrbit.projection.puzzle3dMainPerspective", templates, () => undefined);
      expect(items.map((row) => row.id)).toEqual(["framework.worldOrbit.projection.puzzle3dMainPerspective.parallel", "framework.worldOrbit.projection.puzzle3dMainPerspective.perspective"]);
      expect(items[0]!.items!.map((row) => row.label)).toEqual(["Orthographic", "Axonometric", "Oblique"]);
      expect(items[0]!.items![0]!.items).toBeUndefined();
      expect(items[0]!.items![1]!.items!.map((row) => row.label)).toEqual(["Isometric", "Dimetric", "Trimetric"]);
      expect(items[1]!.items!.map((row) => row.label)).toEqual(["1-Point", "2-Point", "3-Point", "Curvilinear"]);
    });

    it("qualifies every switch row id by the owning pane so two world surfaces share none", () => {
      const templates = createWorldProjectionTemplates({ controllerId: "demo" });
      const rowIds = (paneId: string): string[] => {
        const walk = (rows: readonly { readonly id: string; readonly items?: readonly unknown[] }[]): string[] =>
          rows.flatMap((row) => [row.id, ...walk((row.items ?? []) as readonly { readonly id: string; readonly items?: readonly unknown[] }[])]);
        return walk(worldProjectionSwitchTreeItems(paneId, templates, () => undefined));
      };
      const top = rowIds("framework.worldOrbit.projection.puzzle3dMainTop");
      const perspective = rowIds("framework.worldOrbit.projection.puzzle3dMainPerspective");
      expect(top.length).toBeGreaterThan(8);
      expect(top.length).toBe(perspective.length);
      expect(top.filter((id) => perspective.includes(id))).toEqual([]);
      expect(new Set([...top, ...perspective]).size).toBe(top.length + perspective.length);
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
    it("applies when the seed token changes and stays idle for the same mounted rig", () => {
      const token = orbitCameraViewRigApplyToken("win-a:3", "perspective");
      const camera = {};
      const controls = {};
      expect(shouldApplyOrbitCameraViewRigSeed(null, token, null, camera, null, controls)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(token, token, camera, camera, controls, controls)).toBe(false);
      expect(shouldApplyOrbitCameraViewRigSeed(token, orbitCameraViewRigApplyToken("win-a:4", "perspective"), camera, camera, controls, controls)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(token, orbitCameraViewRigApplyToken("win-a:3", "orthographic"), camera, camera, controls, controls)).toBe(true);
    });

    it("re-applies the same seed to replacement camera and controls instances", () => {
      const token = orbitCameraViewRigApplyToken("preview", "perspective");
      const cameraA = {};
      const cameraB = {};
      const controlsA = {};
      const controlsB = {};
      expect(shouldApplyOrbitCameraViewRigSeed(token, token, cameraA, cameraA, controlsA, controlsB)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(token, token, cameraA, cameraB, controlsA, controlsA)).toBe(true);
      expect(shouldApplyOrbitCameraViewRigSeed(token, token, cameraB, cameraB, controlsB, controlsB)).toBe(false);
    });

    it("restores the accepted pose when mounted camera and controls instances are replaced", async () => {
      (globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
      const container = document.createElement("div");
      const root = createRoot(container);
      const state = { position: [8, 9, 10] as const, target: [4, 5, 6] as const, up: [0, 1, 0] as const, zoom: 3, projection: "perspective" as const };
      const cameraA = new ThreePerspectiveCamera();
      const controlsA = { target: new Vector3(), update: vi.fn() };
      const invalidate = vi.fn();
      await act(async () => root.render(createElement(WorldOrbitCameraViewRigSeedApply, { camera: cameraA, controls: controlsA, invalidate, state, seedKey: "accepted:7" })));
      expect(cameraA.position.toArray()).toEqual([8, 9, 10]);
      expect(controlsA.target.toArray()).toEqual([4, 5, 6]);

      const cameraB = new ThreePerspectiveCamera();
      const controlsB = { target: new Vector3(), update: vi.fn() };
      await act(async () => root.render(createElement(WorldOrbitCameraViewRigSeedApply, { camera: cameraB, controls: controlsB, invalidate, state, seedKey: "accepted:7" })));
      expect(cameraB.position.toArray()).toEqual([8, 9, 10]);
      expect(controlsB.target.toArray()).toEqual([4, 5, 6]);

      const controlsC = { target: new Vector3(), update: vi.fn() };
      await act(async () => root.render(createElement(WorldOrbitCameraViewRigSeedApply, { camera: cameraB, controls: controlsC, invalidate, state, seedKey: "accepted:7" })));
      expect(controlsC.target.toArray()).toEqual([4, 5, 6]);
      expect(invalidate).toHaveBeenCalledTimes(3);
      await act(async () => root.unmount());
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
}
