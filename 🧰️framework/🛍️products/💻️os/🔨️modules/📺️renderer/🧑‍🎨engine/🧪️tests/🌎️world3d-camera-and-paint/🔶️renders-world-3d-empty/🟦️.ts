/** 🔶️ The renders world 3d empty tests of the framework renderer hosts suite. */
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { worldCameraReportTargetV1 } from "@semio-tech/infinite-world-r3f";
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, expect, it } from "vitest";
import { world3dFrameDistanceForRadius, world3dBoundsRadius, world3dAutoFitOwed, world3dAutoFitKey } from "../../../🧱️elements/🌐️World3dHost/🔭️scene-schema/🟦️.ts";
import { World3dHost, mergeWorldViewportCamera, shouldReattachWorldViewportCamera, world3dViewportCameraSeedKey, world3dFitProjectionContent, world3dFramingInstances, world3dFrameVisibleOverlayOffered, world3dProjectionContentFrameMounted, world3dCameraDomJson } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { worldSceneContentBounds, worldSceneContentBoundsKey } from "@semio-tech/infinite-world-r3f";
import { noopAction } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 🔷 Registers the renders world 3d empty tests of the framework renderer hosts suite. */
export function registerRendersWorld3dEmptyTests(): void {
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
}
