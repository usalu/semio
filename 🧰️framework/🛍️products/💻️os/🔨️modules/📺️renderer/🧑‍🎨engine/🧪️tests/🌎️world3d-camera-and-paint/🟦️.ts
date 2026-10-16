/** 🌎 World 3d empty state, camera framing, mesh paint and context target resolution. */
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { worldCameraReportTargetV1 } from "@semio-tech/infinite-world-r3f";
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { worldInstanceHighlighted } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import { world3dFrameCameraFromBounds, world3dFrameDistanceForRadius, world3dBoundsRadius, world3dAutoFitOwed, world3dAutoFitKey, worldVortexHitProxy, worldInstanceMeshRaycast, applyWorldInstanceMeshRaycast } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";
import { surfaceContextMenuTitleKey, World3dHost, worldGhostMeshUrl, mergeWorldViewportCamera, resolveMeshStyle, resolveMeshSelectionPreviewStyle, semanticColorsFromPalette, celebrateWorldInstances, isWorldInstanceCelebrating, isCurveOnlyWorldMesh, meshBoundsCorners, resolveVortexPointerDownIntent, worldMeshMaterialRevision, worldVortexMaterialRevision, resolveWorldMergeMode, resolveWorldContextMenuTarget, world3dContextMenuSurfaceV1, shouldReattachWorldViewportCamera, worldCameraPoseApproxEqual, buildWorldCameraDispatchArgs, worldCameraSetCameraDispatchArgs, world3dViewportCameraSeedKey, world3dFitProjectionContent, world3dFramingInstances, world3dFrameVisibleOverlayOffered, world3dProjectionContentFrameMounted, world3dCameraDomJson, worldInstancePickBlocked, parseWorldTerrainStyle } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { suggestionMenuOwnsWindow } from "../../🧱️elements/🎣️suggestion-submenu/🟦️.ts";
import { worldSceneContentBounds, worldSceneContentBoundsKey } from "@semio-tech/infinite-world-r3f";
import { noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
  afterEach(() => cleanup());

  it("renders world 3d empty state without mounting r3f canvas", () => {
    const markup = renderToStaticMarkup(
      createElement(World3dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle.play.world",
          controllerId: "puzzle-play",
          componentKind: "world-3d",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-world-3d-empty");
    expect(markup).not.toContain("data-orbit-view-gizmo");
  });

  it("marks every non-empty world-3d host with the bottom-right orbit view gizmo", () => {
    const markup = renderToStaticMarkup(
      createElement(World3dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle.3d.play.viewport",
          controllerId: "puzzle3d-play",
          componentKind: "world-3d",
          world3d: {
            cameraJson: '{"position":[4,4,4],"target":[0,0,0],"zoom":1}',
            meshesJson: "[]",
            instancesJson: "[]",
            selectionJson: "{}",
            vorticesJson: "[]",
            attractionsJson: "[]",
            interactionJson: '{"activeUtility":"select"}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-world-3d-host");
    expect(markup).toContain('data-orbit-view-gizmo=""');
    expect(markup).toContain('data-surface-id="puzzle.3d.play.viewport"');
    expect(markup).toContain("data-world-projection-kind-switch");
  });

  it("mirrors the world-3d camera pose into a stable rounded dom json attribute", () => {
    const pose = { position: [1.000004, 2, 3], target: [0, 0, 0], zoom: 1, up: [0, 0, 1], projection: "perspective", fov: 50 } as const;
    const first = world3dCameraDomJson(pose);
    expect(first).toBe(world3dCameraDomJson({ ...pose, position: [1.000001, 2, 3] }));
    expect(JSON.parse(first)).toEqual({ position: [1, 2, 3], target: [0, 0, 0], up: [0, 0, 1], zoom: 1, fov: 50, projection: "perspective" });
    expect(world3dCameraDomJson({ ...pose, position: [9, 2, 3] })).not.toBe(first);
    expect(world3dCameraDomJson({ ...pose, target: [0, 0, 4] })).not.toBe(first);
    expect(world3dCameraDomJson({ ...pose, zoom: 2 })).not.toBe(first);
    expect(JSON.parse(world3dCameraDomJson({ position: [0, 0, 0], target: [0, 0, 0], zoom: 1 })).up).toBe(null);
  });

  it("keeps world-3d orbit camera seed local per viewport once detached", () => {
    const sceneCamera = '{"position":[1,2,3],"target":[0,0,0],"zoom":1}';
    expect(world3dViewportCameraSeedKey(sceneCamera, 0)).toBe(sceneCamera);
    expect(world3dViewportCameraSeedKey(sceneCamera, 1)).toBe("viewport:1");
    expect(world3dViewportCameraSeedKey(sceneCamera, 2)).toBe("viewport:2");
    expect(world3dViewportCameraSeedKey(sceneCamera, 0)).not.toBe(world3dViewportCameraSeedKey(sceneCamera, 1));
    expect(shouldReattachWorldViewportCamera(sceneCamera, sceneCamera)).toBe(false);
    expect(shouldReattachWorldViewportCamera(sceneCamera, '{"position":[9,9,9],"target":[0,0,0],"zoom":1}')).toBe(true);
    const merged = mergeWorldViewportCamera(
      { position: [1, 2, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: true, projectionFrame: "preserveCamera", up: [0, 0, 1] },
      { position: [4, 5, 6], target: [1, 1, 1], zoom: 2, projection: "orthographic", up: [0, 1, 0] },
    );
    expect(merged.projectionFrame).toBe("preserveCamera");
    expect(merged.position).toEqual([4, 5, 6]);
    expect(merged.target).toEqual([1, 1, 1]);
    expect(merged.zoom).toBe(2);
    expect(merged.projection).toBe("orthographic");
    expect(merged.fov).toBe(45);
    expect(merged.explicitProjection).toBe(true);
    expect(merged.up).toEqual([0, 1, 0]);
  });

  it("world3dFitProjectionContent pauses fill-driven projection reframes while the user is navigating or fill is armed", () => {
    expect(world3dFitProjectionContent(false, false, true)).toBe(true);
    expect(world3dFitProjectionContent(true, false, true)).toBe(false);
    expect(world3dFitProjectionContent(false, true, true)).toBe(false);
    expect(world3dFitProjectionContent(false, false, true, true)).toBe(false);
    expect(world3dFitProjectionContent(false, false, false)).toBe(false);
    expect(world3dFitProjectionContent(false, false, true, false, "preserveCamera")).toBe(false);
    expect(world3dProjectionContentFrameMounted(true, false, false)).toBe(true);
    expect(world3dProjectionContentFrameMounted(true, false, true)).toBe(false);
    expect(world3dProjectionContentFrameMounted(false, true, true)).toBe(false);
    expect(world3dProjectionContentFrameMounted(false, true, false)).toBe(true);
  });

  it("world3dFramingInstances keeps fill provisional placements out of projection framing bounds", () => {
    const committed = worldSceneContentBounds([{ position: [0, 0, 0] }]);
    const withProvisional = worldSceneContentBounds(
      world3dFramingInstances([
        { id: "a", position: [0, 0, 0] },
        { id: "b", position: [40, -12, 3], provisional: true },
      ]),
    );
    expect(worldSceneContentBoundsKey(withProvisional)).toBe(worldSceneContentBoundsKey(committed));
  });

  it("world3dFrameVisibleOverlayOffered never paints the retired frame-visible overlay", () => {
    expect(world3dFrameVisibleOverlayOffered(null)).toBe(false);
    expect(world3dFrameVisibleOverlayOffered({ enabled: false })).toBe(false);
    expect(world3dFrameVisibleOverlayOffered({ enabled: true })).toBe(false);
    expect(
      world3dFrameVisibleOverlayOffered({
        enabled: true,
        boundsMin: [0, 0, 0],
        boundsMax: [1, 1, 1],
      }),
    ).toBe(false);
  });

  it("world3dFrameDistanceForRadius stands the eye off far enough that the whole bounding sphere projects inside the frustum", () => {
    for (const [fov, aspect] of [
      [45, 1],
      [45, 1.8],
      [45, 0.6],
      [30, 2.2],
      [60, 1.3],
    ] as const) {
      const radius = 1.7320508;
      const distance = world3dFrameDistanceForRadius(radius, fov, aspect);
      const vertical = ((fov * Math.PI) / 180) * 0.5;
      const horizontal = Math.atan(Math.tan(vertical) * aspect);
      expect(Math.asin(radius / distance)).toBeLessThan(Math.min(vertical, horizontal));
      expect(distance).toBeLessThan((radius / Math.sin(Math.min(vertical, horizontal))) * 1.25);
    }
  });

  it("world3dBoundsRadius is the bounding-sphere radius, never half the longest edge", () => {
    expect(world3dBoundsRadius([0, 0, 0], [2, 2, 2])).toBeCloseTo(Math.sqrt(3), 6);
    expect(world3dBoundsRadius([-1.2, -1.2, -1.2], [1.5, 1.5, 1.5])).toBeCloseTo((Math.sqrt(3) * 2.7) / 2, 6);
  });

  it("a camera report never invents a target", () => {
    expect(worldCameraReportTargetV1({ mouseButtons: {}, target: { x: 1.5, y: -2, z: 3 } as never })).toEqual({ x: 1.5, y: -2, z: 3 });
    expect(worldCameraReportTargetV1(null)).toBeNull();
    expect(worldCameraReportTargetV1(undefined)).toBeNull();
    expect(worldCameraReportTargetV1({ mouseButtons: {} })).toBeNull();
    expect(worldCameraReportTargetV1({ mouseButtons: {}, target: { x: Number.NaN, y: 0, z: 0 } as never })).toBeNull();
    expect(worldCameraReportTargetV1({ mouseButtons: {}, target: { x: 0, y: Number.POSITIVE_INFINITY, z: 0 } as never })).toBeNull();
    expect(worldCameraReportTargetV1({ mouseButtons: {}, target: { x: 0, y: 0, z: 0 } as never })).toEqual({ x: 0, y: 0, z: 0 });
  });

  it("world3dAutoFitOwed frames the first delivery of a document and never yanks a camera the user moved on it", () => {
    expect(world3dAutoFitOwed("", "7:a", false)).toBe(true);
    expect(world3dAutoFitOwed("7:a", "7:a", false)).toBe(false);
    expect(world3dAutoFitOwed("", "7:a", true)).toBe(false);
    expect(world3dAutoFitOwed("7:a", "7:b", true)).toBe(false);
    expect(world3dAutoFitOwed("7:a", "9:a", false)).toBe(true);
  });

  it("world3dAutoFitKey tracks document revision and published bounds, not scene mesh roster churn", () => {
    expect(world3dAutoFitKey(7, "seed", null)).toBe("7:seed");
    expect(world3dAutoFitKey(7, "seed", null)).toBe(world3dAutoFitKey(7, "seed", null));
    const bounds: [readonly number[], readonly number[]] = [
      [0, 0, 0],
      [2, 2, 3],
    ];
    expect(world3dAutoFitKey(7, "seed", bounds)).toBe("7:seed:0,0,0:2,2,3");
    expect(world3dAutoFitOwed(world3dAutoFitKey(7, "seed", null), world3dAutoFitKey(7, "seed", null), false)).toBe(false);
  });

  it("a framed camera puts every corner of the delivered box inside the viewport, for every bundled generation3d example", () => {
    const examples: readonly (readonly [string, readonly [number, number, number], readonly [number, number, number]])[] = [
      ["hexagonal-mushroom-column", [-0.5, -0.43301, 0], [0.5, 0.43301, 6]],
      ["rectangle-extrude-volume", [0, 0, 0], [2, 2, 3]],
      ["face-sweep-extrude", [0, 0, 0], [2, 1.5, 4]],
      ["box-shell-preview", [0, 0, 0], [2, 2, 2]],
      ["box-fillet-preview", [0, 0, 0], [2, 2, 2]],
      ["rectangle-wire-preview", [0, 0, 0], [2, 1.5, 0]],
      ["sphere-box-fuse", [-1.2, -1.2, -1.2], [1.5, 1.5, 1.5]],
      ["sphere-cut-with-torus", [-2.2, -2.2, -2.2], [2.2, 2.2, 2.2]],
    ];
    const seed = { position: [4, -4, 3] as [number, number, number], target: [0, 0, 0] as [number, number, number], zoom: 1, projection: "perspective" as const, fov: 45, explicitProjection: false, projectionFrame: "content" as const };
    for (const [name, minimum, maximum] of examples) {
      for (const aspect of [1.7, 1.0, 0.7]) {
        const center: [number, number, number] = [(minimum[0] + maximum[0]) / 2, (minimum[1] + maximum[1]) / 2, (minimum[2] + maximum[2]) / 2];
        const framed = world3dFrameCameraFromBounds(center, world3dBoundsRadius(minimum, maximum), seed, undefined, aspect);
        const forward = [framed.target[0] - framed.position[0], framed.target[1] - framed.position[1], framed.target[2] - framed.position[2]];
        const forwardLength = Math.hypot(...forward);
        const unitForward = forward.map((value) => value / forwardLength);
        const worldUp = [0, 0, 1];
        const right = [unitForward[1] * worldUp[2] - unitForward[2] * worldUp[1], unitForward[2] * worldUp[0] - unitForward[0] * worldUp[2], unitForward[0] * worldUp[1] - unitForward[1] * worldUp[0]];
        const rightLength = Math.hypot(...right);
        const unitRight = right.map((value) => value / rightLength);
        const unitUp = [unitRight[1] * unitForward[2] - unitRight[2] * unitForward[1], unitRight[2] * unitForward[0] - unitRight[0] * unitForward[2], unitRight[0] * unitForward[1] - unitRight[1] * unitForward[0]];
        const halfVertical = Math.tan(((seed.fov * Math.PI) / 180) * 0.5);
        for (const corner of [
          [minimum[0], minimum[1], minimum[2]],
          [minimum[0], minimum[1], maximum[2]],
          [minimum[0], maximum[1], minimum[2]],
          [minimum[0], maximum[1], maximum[2]],
          [maximum[0], minimum[1], minimum[2]],
          [maximum[0], minimum[1], maximum[2]],
          [maximum[0], maximum[1], minimum[2]],
          [maximum[0], maximum[1], maximum[2]],
        ]) {
          const relative = [corner[0] - framed.position[0], corner[1] - framed.position[1], corner[2] - framed.position[2]];
          const depth = relative[0] * unitForward[0] + relative[1] * unitForward[1] + relative[2] * unitForward[2];
          expect(depth).toBeGreaterThan(0);
          const ndcY = (relative[0] * unitUp[0] + relative[1] * unitUp[1] + relative[2] * unitUp[2]) / (depth * halfVertical);
          const ndcX = (relative[0] * unitRight[0] + relative[1] * unitRight[1] + relative[2] * unitRight[2]) / (depth * halfVertical * aspect);
          expect(`${name}@${aspect} x=${ndcX.toFixed(3)}`).toBe(`${name}@${aspect} x=${Math.max(Math.min(ndcX, 0.999), -0.999).toFixed(3)}`);
          expect(`${name}@${aspect} y=${ndcY.toFixed(3)}`).toBe(`${name}@${aspect} y=${Math.max(Math.min(ndcY, 0.999), -0.999).toFixed(3)}`);
        }
      }
    }
  });

  it("buildWorldCameraDispatchArgs carries position/target/zoom/up but never a projection field", () => {
    const withUp = buildWorldCameraDispatchArgs({ position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1], projection: "orthographic" });
    expect(withUp).toEqual({ position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1] });
    expect(withUp).not.toHaveProperty("projection");
    expect(withUp).not.toHaveProperty("projectionSpec");

    const withoutUp = buildWorldCameraDispatchArgs({ position: [1, 2, 3], target: [0, 0, 0], zoom: 1, projection: "perspective" });
    expect(withoutUp).toEqual({ position: [1, 2, 3], target: [0, 0, 0], zoom: 1 });
    expect(withoutUp).not.toHaveProperty("up");
    expect(withoutUp).not.toHaveProperty("projection");
  });

  it("worldCameraSetCameraDispatchArgs nests the camera pose under a `camera` key, never flat alongside windowId", () => {
    const args = worldCameraSetCameraDispatchArgs("puzzle.3d.play.viewport", { position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1], projection: "orthographic" });
    expect(args).toEqual({ windowId: "puzzle.3d.play.viewport", camera: { position: [1, 2, 3], target: [0, 0, 0], zoom: 2, up: [0, 0, 1] } });
    expect(args).not.toHaveProperty("position");
    expect(args).not.toHaveProperty("target");
    expect(args).not.toHaveProperty("zoom");
    expect(args).not.toHaveProperty("up");
  });

  it("worldCameraPoseApproxEqual matches exact poses and float-noise, rejects a genuinely different pose", () => {
    const base = { position: [1, 2, 3] as const, target: [0, 0, 0] as const, zoom: 1 };
    expect(worldCameraPoseApproxEqual(base, { position: [1, 2, 3], target: [0, 0, 0], zoom: 1 })).toBe(true);
    expect(worldCameraPoseApproxEqual(base, { position: [1.0000001, 2.0000001, 3], target: [0, 0, 1e-9], zoom: 1.0000001 })).toBe(true);
    expect(worldCameraPoseApproxEqual(base, { position: [9, 2, 3], target: [0, 0, 0], zoom: 1 })).toBe(false);
    expect(worldCameraPoseApproxEqual(base, { position: [1, 2, 3], target: [0, 0, 0], zoom: 5 })).toBe(false);
  });

  it("shouldReattachWorldViewportCamera suppresses a self-echo of the last dispatched camera but not a genuinely different pose", () => {
    const previous = '{"position":[1,2,3],"target":[0,0,0],"zoom":1}';
    const dispatched = { position: [4, 5, 6] as const, target: [0, 0, 0] as const, zoom: 2 };
    const echoedJson = '{"position":[4.0000001,5,6],"target":[0,0,0],"zoom":2}';
    const differentJson = '{"position":[9,9,9],"target":[0,0,0],"zoom":1}';
    expect(shouldReattachWorldViewportCamera(previous, echoedJson, dispatched)).toBe(false);
    expect(shouldReattachWorldViewportCamera(previous, differentJson, dispatched)).toBe(true);
    expect(shouldReattachWorldViewportCamera(previous, previous, dispatched)).toBe(false);
    expect(shouldReattachWorldViewportCamera(previous, differentJson, null)).toBe(true);
  });

  it("preserves projectionSpec.view from gizmo snaps instead of clobbering to top", () => {
    const merged = mergeWorldViewportCamera(
      { position: [0, 0, 10], target: [0, 0, 0], zoom: 50, projection: "orthographic", fov: 45, explicitProjection: true, projectionFrame: "content", projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } } },
      { position: [0, -600, 0], target: [0, 0, 0], zoom: 50, projection: "orthographic", projectionSpec: { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } } },
    );
    expect(merged.projectionSpec).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } });
  });

  it("accepts extended world 3d scene fields", () => {
    const node = {
      type: "componentScene",
      surfaceId: "puzzle.3d.play.viewport",
      controllerId: "puzzle3d-play",
      componentKind: "world-3d",
      world3d: {
        cameraJson: "{}",
        meshesJson: "[]",
        instancesJson: "[]",
        selectionJson: "{}",
        vorticesJson: "[]",
        attractionsJson: "[]",
        targetVolumesJson: "[]",
        referencesJson: "[]",
        brushPreviewJson: undefined,
        interactionJson: '{"activeUtility":"select"}',
        engagementPreviewJson: '[{"kind":"point","role":"origin","position":[0,0,0]},{"kind":"box-preview","role":"preview","cornerA":[0,0,0],"cornerB":[2,2,0]}]',
        // 🖱️ Context menus are no longer pushed as scene JSON — a right-click round-trips through
        // `requestContextMenu`/`ContextMenuItemSpec` on demand instead (see `openSurfaceContextMenu`,
        // renderer `🟦️.tsx`), so `World3dScene` has no `contextMenuJson` field to cover here.
        statusJson: '{"computing":true,"label":"Evaluating"}',
        terrainJson: '{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":1.5,"colorRamp":"hypsometric","minZoom":6,"maxZoom":14}',
      },
    };
    expect(node.world3d?.meshesJson).toBe("[]");
    expect(node.world3d?.vorticesJson).toBe("[]");
    expect(node.world3d?.interactionJson).toContain("select");
    expect(node.world3d?.engagementPreviewJson).toContain("box-preview");
    expect(node.world3d?.statusJson).toContain("computing");
    expect(node.world3d?.terrainJson).toContain("hypsometric");
  });

  it("parses GIS 3D terrain style JSON, defaulting missing fields, and rejects a missing tileUrlTemplate", () => {
    expect(parseWorldTerrainStyle(undefined)).toBeNull();
    expect(parseWorldTerrainStyle("not json")).toBeNull();
    expect(parseWorldTerrainStyle('{"projectOriginLon":1}')).toBeNull();
    const style = parseWorldTerrainStyle('{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":2}');
    expect(style).toMatchObject({
      tileUrlTemplate: "/dem/{z}/{x}/{y}.png",
      projectOriginLon: 9.7382,
      projectOriginLat: 52.3759,
      exaggeration: 2,
      colorRamp: "hypsometric",
      minZoom: 6,
      maxZoom: 14,
    });
  });

  it("blocks instance picking for brush and volume brush engagements but not move", () => {
    expect(worldInstancePickBlocked("brush")).toBe(true);
    expect(worldInstancePickBlocked("volumeBrush")).toBe(true);
    expect(worldInstancePickBlocked("surfaceBrush")).toBe(true);
    expect(worldInstancePickBlocked("move")).toBe(false);
    expect(worldInstancePickBlocked(undefined)).toBe(false);
  });

  it("keeps the vortex hit proxy visible so Three's raycaster does not skip it", () => {
    expect(worldVortexHitProxy(0.36)).toEqual({ visible: true, radius: 0.36 });
    expect(worldVortexHitProxy(0.1).radius).toBeGreaterThanOrEqual(0.36);
    expect(worldVortexHitProxy().visible).toBe(true);
  });

  it("clears instance mesh raycasts when pick is blocked so sibling vortex markers stay hittable", () => {
    expect(worldInstanceMeshRaycast(true)).toBeUndefined();
    expect(worldInstanceMeshRaycast(false)).toEqual(expect.any(Function));
    const mesh = { isMesh: true, raycast: "keep" as unknown };
    const root = { traverse: (fn: (object: typeof mesh) => void) => fn(mesh) };
    applyWorldInstanceMeshRaycast(root, false, "keep");
    expect(mesh.raycast).toEqual(expect.any(Function));
    applyWorldInstanceMeshRaycast(root, true, "keep");
    expect(mesh.raycast).toBe("keep");
  });

  it("resolves vortex pointer-down to select in brush or vertex mode and click-or-drag otherwise", () => {
    expect(resolveVortexPointerDownIntent(true)).toBe("select");
    expect(resolveVortexPointerDownIntent(false, "vertex")).toBe("select");
    expect(resolveVortexPointerDownIntent(false)).toBe("click-or-drag");
    expect(resolveVortexPointerDownIntent(false, "mesh")).toBe("click-or-drag");
  });

  it("scopes suggestion menu ownership to the opening world window so sibling panes stay interactive", () => {
    expect(suggestionMenuOwnsWindow(null, "puzzle3d-main-top")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: false, windowId: "puzzle3d-main-top" }, "puzzle3d-main-top")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: true, windowId: "puzzle3d-main-top" }, "puzzle3d-main-top")).toBe(true);
    expect(suggestionMenuOwnsWindow({ open: true, windowId: "puzzle3d-main-top" }, "puzzle3d-main-perspective")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: true }, "puzzle3d-main-perspective")).toBe(true);
  });

  it("revisions vortex materials when selection or hover state changes", () => {
    expect(worldVortexMaterialRevision()).toBe("neutral");
    expect(worldVortexMaterialRevision(false, true)).toBe("hovered");
    expect(worldVortexMaterialRevision(true, true)).toBe("selected");
  });

  it("revisions world mesh materials when style kind changes so deselection clears selected paint", () => {
    expect(worldMeshMaterialRevision("selected")).toBe("selected");
    expect(worldMeshMaterialRevision("neutral")).toBe("neutral");
    expect(worldMeshMaterialRevision("hovered")).toBe("hovered");
    expect(worldMeshMaterialRevision(resolveMeshStyle({ selected: false, hovered: false }))).toBe("neutral");
  });

  it("uses the world surface selection mode instead of a stale shared invertive mode", () => {
    // 🐚️ "invertive" here plays the role the old page-global `__selectionMode` used to (a shell's
    // persistent toolbar toggle) — passed explicitly now via the `persistentMode` param instead of a
    // `globalThis` singleton, but the priority rule under test is unchanged: an explicitly *configured*
    // world-surface mode still wins over it.
    expect(resolveWorldMergeMode("replace", {}, "invertive")).toBe("replace");
    expect(resolveWorldMergeMode("replace", { shiftKey: true }, "invertive")).toBe("additive");
    expect(resolveWorldMergeMode("invertive", {}, "invertive")).toBe("invertive");
  });

  it("resolves mesh style by priority: disabled > celebrated > selected > highlighted > hovered > neutral", () => {
    expect(resolveMeshStyle({})).toBe("neutral");
    expect(resolveMeshStyle({ hovered: true })).toBe("hovered");
    expect(resolveMeshStyle({ hovered: true, highlighted: true })).toBe("highlighted");
    expect(resolveMeshStyle({ highlighted: true, selected: true })).toBe("selected");
    expect(resolveMeshStyle({ selected: true, celebrating: true })).toBe("celebrated");
    expect(resolveMeshStyle({ celebrating: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ selected: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ disabled: true, selected: true, highlighted: true, hovered: true, celebrating: true })).toBe("disabled");
  });

  it("paints the guest-stamped highlight (a history draft's reference) whether or not the host's own kind hover is on", () => {
    expect(worldInstanceHighlighted(false, { highlighted: true })).toBe(true);
    expect(worldInstanceHighlighted(true, {})).toBe(true);
    expect(worldInstanceHighlighted(false, {})).toBe(false);
    expect(resolveMeshSelectionPreviewStyle({ hovered: true, highlighted: worldInstanceHighlighted(false, { highlighted: true }) })).toBe("highlighted");
  });

  it("celebrateWorldInstances stamps ids and cancel clears them so paint prefers celebrated over selected", () => {
    const cancel = celebrateWorldInstances(["drop-1"], 60_000);
    expect(isWorldInstanceCelebrating("drop-1")).toBe(true);
    expect(resolveMeshStyle({ selected: true, celebrating: isWorldInstanceCelebrating("drop-1") })).toBe("celebrated");
    cancel();
    expect(isWorldInstanceCelebrating("drop-1")).toBe(false);
    expect(resolveMeshStyle({ selected: true, celebrating: isWorldInstanceCelebrating("drop-1") })).toBe("selected");
  });

  it("maps edge hover to line paint so coplanar edges stay distinct from face hover fill", () => {
    const palette = {
      neutral: { meshColor: "#111111", lineColor: "#222222", emissiveIntensity: 0, opacity: 1 },
      hovered: { meshColor: "#aaaaaa", lineColor: "#333333", emissiveIntensity: 0.08, opacity: 1 },
      selected: { meshColor: "#0000ff", lineColor: "#0000ff", emissiveIntensity: 0.35, opacity: 1 },
      highlighted: { meshColor: "#00ff00", lineColor: "#00ff00", emissiveIntensity: 0.2, opacity: 1 },
      celebrated: { meshColor: "#ff00ff", lineColor: "#ff00ff", emissiveIntensity: 0.55, opacity: 1 },
      disabled: { meshColor: "#999999", lineColor: "#888888", emissiveIntensity: 0, opacity: 0.45 },
    } as Parameters<typeof semanticColorsFromPalette>[0];
    const colors = semanticColorsFromPalette(palette);
    expect(colors.hover).toBe("#00ff00");
    expect(colors.edgeHover).toBe("#00ff00");
    expect(colors.edgeHover).toBe(colors.hover);
    expect(colors.select).toBe("#0000ff");
  });

  it("treats centerline meshes without shaded triangles as curve-only instances", () => {
    expect(isCurveOnlyWorldMesh({ indices: [], edgePositions: [0, 0, 0, 1, 0, 0] })).toBe(true);
    expect(isCurveOnlyWorldMesh({ indices: [0, 1, 2], edgePositions: [0, 0, 0, 1, 0, 0] })).toBe(false);
    expect(isCurveOnlyWorldMesh({ indices: [], edgePositions: [] })).toBe(false);
  });

  it("derives marquee bounds from edge samples when positions are empty", () => {
    const corners = meshBoundsCorners({
      positions: [],
      normals: [],
      indices: [],
      edgePositions: [0, 0, 0, 2, 4, 6],
    } as Parameters<typeof meshBoundsCorners>[0]);
    expect(corners).toContainEqual([0, 0, 0]);
    expect(corners).toContainEqual([2, 4, 6]);
  });

  it("renders the new group selection as active and only objects leaving the old selection as highlighted", () => {
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, false)).toBe("highlighted");
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, false)).toBe("neutral");
    expect(resolveMeshSelectionPreviewStyle({ selected: true, disabled: true }, false)).toBe("disabled");
  });

  it("resolves catalogue-drop ghost mesh URLs even when the kind is not yet among scene meshes", () => {
    // 👻️ A dragged catalogue kind must load its own meshUrl directly — requiring a scene mesh match left a
    // kind not placed yet invisible in 3D.
    expect(worldGhostMeshUrl({ meshUrl: "/meshes/new-kind.glb" }, [])).toBe("/meshes/new-kind.glb");
    expect(worldGhostMeshUrl({ meshUrl: "/meshes/placed.glb" }, [{ url: "/meshes/placed.glb" }])).toBe("/meshes/placed.glb");
    expect(worldGhostMeshUrl({}, [{ url: "/meshes/placed.glb" }])).toBeUndefined();
  });

  it("resolves the right-click context menu target by priority: vortex, then object, then reference", () => {
    expect(resolveWorldContextMenuTarget({ hoveredVortexFullId: "seed-left-001:v0" }, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({
      kind: "vortex",
      id: "seed-left-001:v0",
    });
    expect(resolveWorldContextMenuTarget({}, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "reference:ref-1" })).toEqual({ kind: "reference", id: "ref-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "obj-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, {})).toBeNull();
  });

  it("carries the right-clicked world entity as the request's own hit, so no target-recording dispatch precedes the menu", () => {
    expect(world3dContextMenuSurfaceV1({ kind: "vortex", id: "seed-left-001:v0" }, { ids: ["seed-left-001"], componentIds: [] })).toEqual({
      hits: [{ domain: "vortex", id: "seed-left-001:v0" }],
      selection: [{ domain: "object", ids: ["seed-left-001"] }],
    });
    expect(world3dContextMenuSurfaceV1(null, { ids: [], componentIds: [7] })).toEqual({ hits: [], selection: [{ domain: "feature", ids: ["7"] }] });
    expect(world3dContextMenuSurfaceV1(null, { ids: [], componentIds: [] })).toEqual({ hits: [], selection: [] });
  });

  it("titles context menus from the specific hit before falling back to the surface", () => {
    const request = (domain?: string, kind = "world3d") => ({
      menu: { id: kind, args: null },
      surface: { surfaceId: "surface", kind, hits: domain ? [{ domain, id: "target" }] : [] },
    });

    expect(surfaceContextMenuTitleKey(request("vortex"))).toBe("ui.surfaceContextMenu.vortex");
    expect(surfaceContextMenuTitleKey(request("object"))).toBe("ui.surfaceContextMenu.object");
    expect(surfaceContextMenuTitleKey(request("reference"))).toBe("ui.surfaceContextMenu.reference");
    expect(surfaceContextMenuTitleKey(request())).toBe("ui.surfaceContextMenu.scene");
    expect(surfaceContextMenuTitleKey(request(undefined, "board2d"))).toBe("ui.surfaceContextMenu.board");
  });

  it("covers every target domain emitted by current surface pickers", () => {
    const domains = ["architecture", "attraction", "block", "edge", "entry", "feature", "group", "handle", "layer", "node", "object", "part", "path", "pixel", "position", "reference", "route", "row", "slider", "vortex"];
    for (const domain of domains) {
      expect(surfaceContextMenuTitleKey({ menu: { id: "surface", args: null }, surface: { surfaceId: "surface", kind: "unknown", hits: [{ domain, id: "target" }] } })).toBe(`ui.surfaceContextMenu.${domain}`);
    }
  });
});
