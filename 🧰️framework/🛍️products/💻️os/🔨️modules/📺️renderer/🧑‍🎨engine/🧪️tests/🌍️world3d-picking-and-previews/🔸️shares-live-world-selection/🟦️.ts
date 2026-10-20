/** 🔸️ The shares live world selection tests of the framework renderer hosts suite. */
import * as flowSessionLoader from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { expect, it } from "vitest";
import type { GumballPose } from "@semio-tech/ui-react";
import { mergeWorldSelectionWithLeftoverV1 } from "../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { gumballPreviewOwnedWorldPoint, gumballPreviewOwnedWorldDirection, worldGumballOwnerPoses, worldVorticesWithGumballPreview, worldAttractionsWithGumballPreview } from "../../../🧱️elements/🌐️World3dHost/🧮️gumball-math/🟦️.ts";
import { canvasLayerDisplayLabel, worldToScreenLogical, boardDropPreviewJson, boardScreenToWorld, boardWorldToScreen, registerBoard2dPeer, unregisterBoard2dPeer, clearWorldSelectionPreview, getWorldSelectionPreview, clearWorldGumballTransformPreview, getWorldGumballTransformPreview, setWorldGumballTransformPreview, subscribeWorldGumballTransformPreview, pushBoardDropPreview, setWorldSelectionPreview, subscribeWorldSelectionPreview, gumballTransformDeltaBetweenPoses } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔹 Registers the shares live world selection tests of the framework renderer hosts suite. */
export function registerSharesLiveWorldSelectionTests(): void {
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
}
