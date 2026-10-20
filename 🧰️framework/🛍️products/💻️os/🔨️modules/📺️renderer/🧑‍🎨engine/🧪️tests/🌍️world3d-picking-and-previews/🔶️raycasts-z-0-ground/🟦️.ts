/** 🔶️ The raycasts z 0 ground tests of the framework renderer hosts suite. */
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, expect, it } from "vitest";
import { worldHoverPaintIdV1 } from "../../../🧱️elements/🌐️World3dHost/🧱️instance-layers/🪟️instance-chrome-store/🟦️.ts";
import { resolveClickInstanceIdFromProjected, world3dMarqueePointerCaptureArmed, world3dProjectedAabbContainsClick } from "../../../🧱️elements/🌐️World3dHost/🎯️marquee-and-bridges/🟦️.tsx";
import { world3dInstancePickUsesInteractionDomain, world3dSuggestionsGestureArmed, world3dRetainLocalVortexHover, world3dSuggestionsGestureConsumesContextMenu, world3dSuggestionsRightDownRoutesOnWindowCapture } from "../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { world3dFrameCameraFromBounds, world3dFrameCameraFromInstances } from "../../../🧱️elements/🌐️World3dHost/🔭️scene-schema/🟦️.ts";
import { raycastGroundPoint, clearWorldCatalogueDropPreview, getWorldCatalogueDropPreview, registerWorldCatalogueDropHost, setWorldCatalogueDropPreview, subscribeWorldCatalogueDropPreview, worldCatalogueDropHostContainsPoint } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔷 Registers the raycasts z 0 ground tests of the framework renderer hosts suite. */
export function registerRaycastsZ0GroundTests(): void {
  afterEach(() => cleanup());
  it("raycasts the Z=0 ground under orthographic top and perspective cameras", async () => {
    const { sceneHostPort } = await import("@semio-tech/ui-react");
    const { OrthographicCamera, PerspectiveCamera } = sceneHostPort.three;
    const rect = { left: 0, top: 0, width: 200, height: 100, right: 200, bottom: 100 } as DOMRect;

    const ortho = new OrthographicCamera(-100, 100, 50, -50, 0.1, 1000);
    ortho.position.set(0, 0, 100);
    ortho.up.set(0, 1, 0);
    ortho.lookAt(0, 0, 0);
    ortho.updateMatrixWorld(true);
    ortho.updateProjectionMatrix();

    const orthoCenter = raycastGroundPoint(100, 50, rect, ortho);
    expect(orthoCenter).not.toBeNull();
    expect(orthoCenter![0]).toBeCloseTo(0, 5);
    expect(orthoCenter![1]).toBeCloseTo(0, 5);
    expect(orthoCenter![2]).toBeCloseTo(0, 5);

    const orthoRight = raycastGroundPoint(150, 50, rect, ortho);
    expect(orthoRight).not.toBeNull();
    expect(orthoRight![0]).toBeCloseTo(50, 5);
    expect(orthoRight![1]).toBeCloseTo(0, 5);
    expect(orthoRight![2]).toBeCloseTo(0, 5);

    const orthoUp = raycastGroundPoint(100, 25, rect, ortho);
    expect(orthoUp).not.toBeNull();
    expect(orthoUp![0]).toBeCloseTo(0, 5);
    expect(orthoUp![1]).toBeCloseTo(25, 5);
    expect(orthoUp![2]).toBeCloseTo(0, 5);

    const perspective = new PerspectiveCamera(50, 2, 0.1, 1000);
    perspective.position.set(0, 0, 10);
    perspective.up.set(0, 1, 0);
    perspective.lookAt(0, 0, 0);
    perspective.updateMatrixWorld(true);
    perspective.updateProjectionMatrix();

    const perspectiveCenter = raycastGroundPoint(100, 50, rect, perspective);
    expect(perspectiveCenter).not.toBeNull();
    expect(perspectiveCenter![0]).toBeCloseTo(0, 4);
    expect(perspectiveCenter![1]).toBeCloseTo(0, 4);
    expect(perspectiveCenter![2]).toBeCloseTo(0, 4);
  });
  it("a host click pick hits the nearest projected instance AABB and misses empty space", () => {
    const table = {
      id: "table",
      corners: [
        [400, 300],
        [600, 300],
        [400, 380],
        [600, 380],
      ] as const,
      depth: 10,
    };
    const far = {
      id: "far",
      corners: [
        [400, 300],
        [600, 300],
        [400, 380],
        [600, 380],
      ] as const,
      depth: 40,
    };
    expect(world3dProjectedAabbContainsClick({ x: 500, y: 340 }, table.corners)).toBe(true);
    expect(world3dProjectedAabbContainsClick({ x: 10, y: 10 }, table.corners)).toBe(false);
    expect(resolveClickInstanceIdFromProjected({ x: 500, y: 340 }, [table, far])).toBe("table");
    expect(resolveClickInstanceIdFromProjected({ x: 10, y: 10 }, [table, far])).toBeNull();
    expect(world3dMarqueePointerCaptureArmed(0)).toBe(false);
    expect(world3dMarqueePointerCaptureArmed(4)).toBe(false);
    expect(world3dMarqueePointerCaptureArmed(5)).toBe(true);
    expect(world3dInstancePickUsesInteractionDomain(undefined)).toBe(false);
    expect(world3dInstancePickUsesInteractionDomain({})).toBe(false);
    expect(world3dInstancePickUsesInteractionDomain({ interactionId: "port@0" })).toBe(true);
  });
  it("paints the pane's own raycast hover before the guest echoes it, and the guest's when the pointer is elsewhere", () => {
    expect(worldHoverPaintIdV1("bim-1", null)).toBe("bim-1");
    expect(worldHoverPaintIdV1("bim-1", "bim-1")).toBe("bim-1");
    expect(worldHoverPaintIdV1("bim-2", "bim-1")).toBe("bim-2");
    expect(worldHoverPaintIdV1(null, "bim-1")).toBeNull();
    expect(worldHoverPaintIdV1(undefined, "bim-1")).toBe("bim-1");
    expect(worldHoverPaintIdV1(undefined, null)).toBeNull();
  });
  it("arms the suggestions gesture from a host vortex hover without a guest InteractionView", () => {
    expect(world3dSuggestionsGestureArmed(true, "table@in")).toBe(true);
    expect(world3dSuggestionsGestureArmed(true, null)).toBe(false);
    expect(world3dSuggestionsGestureArmed(false, "table@in")).toBe(false);
    expect(world3dRetainLocalVortexHover("seed-left-001:v0", null)).toBe("seed-left-001:v0");
    expect(world3dRetainLocalVortexHover("seed-left-001:v0", undefined)).toBe("seed-left-001:v0");
    expect(world3dRetainLocalVortexHover("seed-left-001:v0", "seed-left-001:v1")).toBe("seed-left-001:v1");
    expect(world3dRetainLocalVortexHover(null, null)).toBeNull();
    expect(world3dSuggestionsGestureConsumesContextMenu(true)).toBe(true);
    expect(world3dSuggestionsGestureConsumesContextMenu(false)).toBe(false);
    expect(world3dSuggestionsGestureArmed(true, world3dRetainLocalVortexHover("seed-left-001:v3", null))).toBe(true);
    expect(world3dSuggestionsRightDownRoutesOnWindowCapture(2, true)).toBe(true);
    expect(world3dSuggestionsRightDownRoutesOnWindowCapture(2, false)).toBe(false);
    expect(world3dSuggestionsRightDownRoutesOnWindowCapture(0, true)).toBe(false);
  });
  it("frames world instances onto the table centroid without a guest selection", () => {
    const camera = { position: [40, -40, 30] as [number, number, number], target: [0, 0, 0] as [number, number, number], zoom: 1, projection: "perspective" as const, fov: 45, explicitProjection: false, projectionFrame: "content" as const };
    const framed = world3dFrameCameraFromInstances([{ id: "seed-left-001", x: 2, y: -1, z: 0.4 }], camera);
    expect(framed.target[0]).toBeCloseTo(2);
    expect(framed.target[1]).toBeCloseTo(-1);
    expect(framed.target[2]).toBeCloseTo(0.4);
    const far = Math.hypot(camera.position[0] - 2, camera.position[1] + 1, camera.position[2] - 0.4);
    const near = Math.hypot(framed.position[0] - 2, framed.position[1] + 1, framed.position[2] - 0.4);
    expect(near).toBeLessThan(far * 0.2);
    expect(near).toBeGreaterThan(1);
  });
  it("frames a table-sized world AABB without leaving the current look direction", () => {
    const camera = { position: [40, -40, 30] as [number, number, number], target: [0, 0, 0] as [number, number, number], zoom: 1, projection: "perspective" as const, fov: 45, explicitProjection: false, projectionFrame: "content" as const };
    const framed = world3dFrameCameraFromBounds([0, 0, 0.4], 1.2, camera);
    expect(framed.target[0]).toBeCloseTo(0);
    expect(framed.target[1]).toBeCloseTo(0);
    expect(framed.target[2]).toBeCloseTo(0.4);
    const distance = Math.hypot(framed.position[0], framed.position[1], framed.position[2] - 0.4);
    expect(distance).toBeGreaterThan(2);
    expect(distance).toBeLessThan(8);
    const farDir = [40, -40, 30];
    const nearDir = [framed.position[0], framed.position[1], framed.position[2] - 0.4];
    const farLen = Math.hypot(...farDir);
    const nearLen = Math.hypot(...nearDir);
    expect(nearDir[0] / nearLen).toBeCloseTo(farDir[0] / farLen, 1);
    expect(nearDir[1] / nearLen).toBeCloseTo(farDir[1] / farLen, 1);
  });
  it("shares the world catalogue drop preview across all registered hosts", () => {
    clearWorldCatalogueDropPreview("puzzle3d-play");
    const notifications: Array<ReturnType<typeof getWorldCatalogueDropPreview>> = [];
    const unsub = subscribeWorldCatalogueDropPreview(() => {
      notifications.push(getWorldCatalogueDropPreview("puzzle3d-play"));
    });
    const unregisterA = registerWorldCatalogueDropHost("puzzle3d-play", "pane.a", (x, y) => x >= 0 && x < 100 && y >= 0 && y < 100);
    const unregisterB = registerWorldCatalogueDropHost("puzzle3d-play", "pane.b", (x, y) => x >= 100 && x < 200 && y >= 0 && y < 100);

    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 50, 50)).toBe(true);
    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 150, 50)).toBe(true);
    expect(worldCatalogueDropHostContainsPoint("puzzle3d-play", 250, 50)).toBe(false);
    expect(worldCatalogueDropHostContainsPoint("other-controller", 50, 50)).toBe(false);

    setWorldCatalogueDropPreview("puzzle3d-play", { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] });
    expect(getWorldCatalogueDropPreview("puzzle3d-play")).toEqual({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] });
    expect(getWorldCatalogueDropPreview("other-controller")).toBeNull();
    setWorldCatalogueDropPreview("puzzle3d-play", { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [3, 4, 0] });
    expect(getWorldCatalogueDropPreview("puzzle3d-play")?.origin).toEqual([3, 4, 0]);
    clearWorldCatalogueDropPreview("puzzle3d-play");
    expect(getWorldCatalogueDropPreview("puzzle3d-play")).toBeNull();
    expect(notifications).toEqual([{ objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [1, 2, 0] }, { objectKind: "Capsule", meshUrl: "puzzle3d://capsule", origin: [3, 4, 0] }, null]);

    unsub();
    unregisterA();
    unregisterB();
  });
}
