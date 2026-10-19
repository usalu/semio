/** 🌍 World 3d picking, framing, shared previews and board world-to-screen math. */
import * as flowSessionLoader from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { worldHoverPaintIdV1 } from "../../🧱️elements/🌐️World3dHost/🧱️instance-layers/🪟️instance-chrome-store/🟦️.ts";
import type { GumballPose } from "@semio-tech/ui-react";
import { resolveClickInstanceIdFromProjected, world3dMarqueePointerCaptureArmed, world3dProjectedAabbContainsClick } from "../../🧱️elements/🌐️World3dHost/🎯️marquee-and-bridges/🟦️.tsx";
import { world3dInstancePickUsesInteractionDomain, world3dSuggestionsGestureArmed, world3dRetainLocalVortexHover, mergeWorldSelectionWithLeftoverV1, world3dSuggestionsGestureConsumesContextMenu, world3dSuggestionsRightDownRoutesOnWindowCapture } from "../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { world3dFrameCameraFromBounds, world3dFrameCameraFromInstances } from "../../🧱️elements/🌐️World3dHost/🔭️scene-schema/🟦️.ts";
import { gumballPreviewOwnedWorldPoint, gumballPreviewOwnedWorldDirection, worldGumballOwnerPoses, worldVorticesWithGumballPreview, worldAttractionsWithGumballPreview } from "../../🧱️elements/🌐️World3dHost/🧮️gumball-math/🟦️.ts";
import { canvasLayerDisplayLabel, worldToScreenLogical, readCanvas2dSurfaceColors, boardDropPreviewJson, boardScreenToWorld, boardWorldToScreen, registerBoard2dPeer, unregisterBoard2dPeer, raycastGroundPoint, clearWorldCatalogueDropPreview, getWorldCatalogueDropPreview, clearWorldSelectionPreview, getWorldSelectionPreview, clearWorldGumballTransformPreview, getWorldGumballTransformPreview, setWorldGumballTransformPreview, subscribeWorldGumballTransformPreview, pushBoardDropPreview, registerWorldCatalogueDropHost, setWorldCatalogueDropPreview, subscribeWorldCatalogueDropPreview, setWorldSelectionPreview, subscribeWorldSelectionPreview, worldCatalogueDropHostContainsPoint, gumballTransformDeltaBetweenPoses } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
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
    // 🎯️ Inside the pane the raycast leads: over an object it paints at once, over nothing it clears at
    // once — both before the `interactionHover` round trip settles and regardless of what the guest's
    // lane still says.
    expect(worldHoverPaintIdV1("bim-1", null)).toBe("bim-1");
    expect(worldHoverPaintIdV1("bim-1", "bim-1")).toBe("bim-1");
    expect(worldHoverPaintIdV1("bim-2", "bim-1")).toBe("bim-2");
    expect(worldHoverPaintIdV1(null, "bim-1")).toBeNull();
    // 🧭️ With no local claim (the pointer left the pane) the guest's hover — an outliner row, a remote
    // presence — is what the pane paints.
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

  it("shares live world selection previews across sibling panes without allowing an idle pane to clear the active gesture", () => {
    clearWorldSelectionPreview("puzzle3d-play");
    const notifications: Array<ReturnType<typeof getWorldSelectionPreview>> = [];
    const unsubscribe = subscribeWorldSelectionPreview(() => notifications.push(getWorldSelectionPreview("puzzle3d-play")));
    const preview = { sourceId: "pane.top", mergedComponentIds: null, mergedInstanceIds: ["object-a", "object-b"] } as const;

    setWorldSelectionPreview("puzzle3d-play", preview);
    clearWorldSelectionPreview("puzzle3d-play", "pane.perspective");
    expect(getWorldSelectionPreview("puzzle3d-play")).toEqual(preview);
    expect(getWorldSelectionPreview("other-controller")).toBeNull();

    setWorldSelectionPreview("puzzle3d-play", { ...preview, mergedInstanceIds: ["object-a", "object-b"] });
    expect(notifications).toEqual([preview]);

    clearWorldSelectionPreview("puzzle3d-play", "pane.top");
    expect(getWorldSelectionPreview("puzzle3d-play")).toBeNull();
    expect(notifications).toEqual([preview, null]);
    unsubscribe();
  });

  it("shares live gumball transform previews across sibling panes without allowing an idle pane to clear the active gesture", () => {
    clearWorldGumballTransformPreview("puzzle3d-play");
    const notifications: Array<ReturnType<typeof getWorldGumballTransformPreview>> = [];
    const unsubscribe = subscribeWorldGumballTransformPreview(() => notifications.push(getWorldGumballTransformPreview("puzzle3d-play")));
    const preview = {
      sourceId: "pane.top",
      transformMode: "transform",
      handleKind: "moveX" as const,
      before: { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] } satisfies GumballPose,
      after: { position: [2, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] } satisfies GumballPose,
      instanceIds: ["object-a"],
      pivot: [0, 0, 0] as const,
    };

    setWorldGumballTransformPreview("puzzle3d-play", preview);
    clearWorldGumballTransformPreview("puzzle3d-play", "pane.perspective");
    expect(getWorldGumballTransformPreview("puzzle3d-play")).toEqual(preview);

    clearWorldGumballTransformPreview("puzzle3d-play", "pane.top");
    expect(getWorldGumballTransformPreview("puzzle3d-play")).toBeNull();
    expect(notifications).toEqual([preview, null]);
    unsubscribe();
  });

  it("mergeWorldSelectionWithLeftoverV1 keeps guest transformMode when leftover still stamps move-era gumball pose", () => {
    const merged = mergeWorldSelectionWithLeftoverV1({ ids: ["object-a"], transformMode: "transform", gumballActive: true, gumballTarget: [1, 2, 3] }, { ids: ["object-a"], hoveredId: null, gumballActive: true, gumballAnchorId: "object-a" }, [
      { id: "object-a", position: [1, 2, 3] },
    ]);
    expect(merged.transformMode).toBe("transform");
  });

  it("gumball rotate handles commit rotateSelection under transform mode even when transformMode was move", () => {
    const base = { mode: "object", ids: ["obj-1"] };
    const before: GumballPose = { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] };
    const after: GumballPose = { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] };
    expect(gumballTransformDeltaBetweenPoses("move", before, after, base, "rotateY")?.action).toBe("rotateSelection");
    const translateAfter: GumballPose = { position: [2, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] };
    expect(gumballPreviewOwnedWorldPoint({ position: [0, 0, 0], rotation: [0, 0, 0, 1] }, "transform", before, translateAfter, "moveX", [1, 0, 0])).toEqual([3, 0, 0]);
  });

  it("previews a multi-object turn and scale of markers about each owner's origin, where the selection leaves land them", () => {
    const close = (actual: readonly number[], expected: readonly number[]) => actual.forEach((value, index) => expect(value).toBeCloseTo(expected[index]!, 9));
    const owners = worldGumballOwnerPoses([
      { id: "a", position: [0, 0, 0], rotation: [0, 0, 0, 1] },
      { id: "b", position: [10, 0, 0], rotation: [0, 0, Math.SQRT1_2, Math.SQRT1_2] },
    ]);
    const quarter: GumballPose = { position: [5, 0, 0], quaternion: [0, 0, Math.SQRT1_2, Math.SQRT1_2], scale: [1, 1, 1] };
    const start: GumballPose = { position: [5, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] };
    const preview = { sourceId: "pane", transformMode: "rotate", handleKind: "rotateZ" as const, before: start, after: quarter, instanceIds: ["a", "b"], pivot: [5, 0, 0] as const };
    const vortices = [
      { fullId: "a:v0", objectId: "a", position: [1, 0, 0] as const, direction: [1, 0, 0] as const },
      { fullId: "b:v0", objectId: "b", position: [11, 0, 0] as const, direction: [1, 0, 0] as const },
      { fullId: "c:v0", objectId: "c", position: [20, 0, 0] as const },
    ];
    const turned = worldVorticesWithGumballPreview(vortices, preview, owners);
    close(turned[0]!.position, [0, 1, 0]);
    close(turned[1]!.position, [10, 1, 0]);
    close(turned[1]!.direction!, [0, 1, 0]);
    expect(turned[2]).toBe(vortices[2]);
    const [attraction] = worldAttractionsWithGumballPreview([{ id: "ab", from: [1, 0, 0], to: [11, 0, 0] }], vortices, preview, owners);
    close(attraction!.from, [0, 1, 0]);
    close(attraction!.to, [10, 1, 0]);
    const stretched: GumballPose = { position: [5, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 1, 1] };
    close(gumballPreviewOwnedWorldPoint(owners.get("b")!, "scale", start, stretched, "scaleX", [10, 1, 0]), [10, 2, 0]);
    close(gumballPreviewOwnedWorldDirection(owners.get("b")!, "scale", start, stretched, "scaleX", [1, 1, 0]), [1 / Math.sqrt(5), 2 / Math.sqrt(5), 0]);
  });

  it("pushes fixture-drop previews to every board2d peer on the same controller", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const calls: { pane: string; method: string; arg: string }[] = [];
    const makePeer = (pane: string) => ({
      session: {
        setDropPreviewJson: (json: string) => calls.push({ pane, method: "setDropPreviewJson", arg: json }),
        clearDropPreview: () => calls.push({ pane, method: "clearDropPreview", arg: "" }),
        renderFrame: () => calls.push({ pane, method: "renderFrame", arg: "" }),
      } as never,
      onPeerGestureEnded: () => {},
    });
    registerBoard2dPeer(scope, "fixture-preview", "pane.source", makePeer("pane.source"));
    registerBoard2dPeer(scope, "fixture-preview", "pane.sibling", makePeer("pane.sibling"));

    const preview = boardDropPreviewJson({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24 }, 10, 20);
    pushBoardDropPreview(scope, "fixture-preview", preview);
    pushBoardDropPreview(scope, "fixture-preview", null);

    expect(calls).toEqual([
      { pane: "pane.source", method: "setDropPreviewJson", arg: preview },
      { pane: "pane.source", method: "renderFrame", arg: "" },
      { pane: "pane.sibling", method: "setDropPreviewJson", arg: preview },
      { pane: "pane.sibling", method: "renderFrame", arg: "" },
      { pane: "pane.source", method: "clearDropPreview", arg: "" },
      { pane: "pane.source", method: "renderFrame", arg: "" },
      { pane: "pane.sibling", method: "clearDropPreview", arg: "" },
      { pane: "pane.sibling", method: "renderFrame", arg: "" },
    ]);

    unregisterBoard2dPeer(scope, "fixture-preview", "pane.source", scope.peers.get("fixture-preview")?.get("pane.source") ?? null);
    unregisterBoard2dPeer(scope, "fixture-preview", "pane.sibling", scope.peers.get("fixture-preview")?.get("pane.sibling") ?? null);
  });

  it("inverts the canonical screen-to-world transform for a fixture drop", () => {
    const cameraJson = JSON.stringify({ x: 120, y: 80, zoom: 2 });
    const world = boardScreenToWorld(cameraJson, { w: 800, h: 600 }, { x: 400, y: 300 });
    expect(world).toEqual({ x: 120, y: 80 });
  });

  it("boardWorldToScreen is the exact inverse of boardScreenToWorld", () => {
    const cameraJson = JSON.stringify({ x: 120, y: 80, zoom: 2 });
    const containerSize = { w: 800, h: 600 };
    // 🎯️ The camera's own world position always maps to the viewport center.
    expect(boardWorldToScreen(cameraJson, containerSize, { x: 120, y: 80 })).toEqual({ x: 400, y: 300 });
    for (const screen of [
      { x: 0, y: 0 },
      { x: 400, y: 300 },
      { x: 733, y: 12 },
    ]) {
      const world = boardScreenToWorld(cameraJson, containerSize, screen);
      expect(world).not.toBeNull();
      const roundTrip = boardWorldToScreen(cameraJson, containerSize, world!);
      expect(roundTrip!.x).toBeCloseTo(screen.x, 5);
      expect(roundTrip!.y).toBeCloseTo(screen.y, 5);
    }
    expect(boardWorldToScreen("not json", containerSize, { x: 0, y: 0 })).toBeNull();
  });

  it("does not use geometry kind as a canvas layer overlay label", () => {
    expect(canvasLayerDisplayLabel({ kind: "circle", id: "node-n1" })).toBe("");
    expect(canvasLayerDisplayLabel({ kind: "circle", id: "node-n1", name: "N1" })).toBe("N1");
    expect(canvasLayerDisplayLabel({ kind: "circle", id: "node-n1", base: { name: "Base" } })).toBe("Base");
  });

  it("maps a world-centered node inside the viewport with canonical camera math", () => {
    const camera = { x: 120, y: 80, zoom: 2 };
    const viewportWidth = 800;
    const viewportHeight = 600;
    const screen = worldToScreenLogical(120, 80, camera, viewportWidth, viewportHeight);
    expect(screen.x).toBeCloseTo(viewportWidth * 0.5, 5);
    expect(screen.y).toBeCloseTo(viewportHeight * 0.5, 5);
    const layersJson = JSON.stringify([
      {
        id: "node-a",
        kind: "circle",
        role: "node",
        color: "#336699",
        selected: true,
        x: 110,
        y: 70,
        width: 20,
        height: 20,
      },
    ]);
    expect(layersJson).toContain('"role":"node"');
    expect(layersJson).toContain('"selected":true');
  });

  it("canvas-2d surface colors follow the light theme canvas token instead of a hardcoded dark fill", () => {
    document.documentElement.classList.remove("dark");
    const colors = readCanvas2dSurfaceColors();
    expect(colors.clear).toBe("rgba(240, 236, 221, 1)");
    expect(colors.clear.toLowerCase()).not.toContain("17, 19, 24");
    expect(colors.grid).toMatch(/^rgba\(/);
  });
});
