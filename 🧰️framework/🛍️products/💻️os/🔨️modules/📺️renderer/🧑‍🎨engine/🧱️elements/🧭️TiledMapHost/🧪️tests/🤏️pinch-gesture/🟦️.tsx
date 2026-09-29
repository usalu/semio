// #region 🧲️Header
/** 🤏️ Mounted `TiledMapHost` multi-touch laws: a second contact retires the marquee,
 * live camera changes stay local, the last lift publishes once, and the recognizer admits a fresh
 * successor contact. The coordinates and expected camera are shared with the native renderer. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { ActionDescriptor, UiComponentSceneNode } from "@semio-tech/framework";
import surfacePinchFixture from "../../../../🧫️fixtures/🤏️surface-pinch/🔣️.json";
import type { MapWasmSession } from "../../../🪪️WasmSessionLoader/🟦️.tsx";

const { createMapSessionMock } = vi.hoisted(() => ({ createMapSessionMock: vi.fn() }));

vi.mock("../../../🪪️WasmSessionLoader/🟦️.tsx", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../../../🪪️WasmSessionLoader/🟦️.tsx")>()),
  createMapSession: createMapSessionMock,
}));

import { TiledMapHost } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🧪️Harness
type StubCamera = { x: number; y: number; zoom: number };

function createStubSession(): MapWasmSession & { readonly calls: string[]; readonly camera: StubCamera } {
  const camera: StubCamera = { ...surfacePinchFixture.gestures.spread.initialCamera };
  const calls: string[] = [];
  const call = (name: string) => () => void calls.push(name);
  return {
    calls,
    camera,
    attachCanvas: async () => void calls.push("attachCanvas"),
    setSize: call("setSize"),
    renderFrame: call("renderFrame"),
    setCamera: (x, y, zoom) => {
      calls.push("setCamera");
      Object.assign(camera, { x, y, zoom });
    },
    cameraJson: () => JSON.stringify(camera),
    cameraLimitsJson: () => JSON.stringify({ min: 0.05, max: 64 }),
    fitWorldCamera: call("fitWorldCamera"),
    reclampCamera: call("reclampCamera"),
    pointerDownScreen: call("pointerDownScreen"),
    pointerMoveScreen: call("pointerMoveScreen"),
    pointerUpScreen: call("pointerUpScreen"),
    wheelScreen: call("wheelScreen"),
    syncMapJson: call("syncMapJson"),
    uploadTile: call("uploadTile"),
    uploadVectorTile: call("uploadVectorTile"),
    hasTile: () => false,
    hasVectorTile: () => false,
    visibleTilesJson: () => "[]",
    visibleVectorTilesJson: () => "[]",
    visibleTilesRevision: () => 0,
    visibleVectorTilesRevision: () => 0,
    prefetchTilesJson: () => "[]",
    prefetchVectorTilesJson: () => "[]",
    setRenderMode: call("setRenderMode"),
    setVectorStyle: call("setVectorStyle"),
    setLodMode: call("setLodMode"),
    setLayerVisibilityJson: call("setLayerVisibilityJson"),
    setLayerStrokeScaleJson: call("setLayerStrokeScaleJson"),
    syncInteraction: call("syncInteraction"),
    featuresInRectJson: () => JSON.stringify({ positions: [], routes: [] }),
    featuresInPolygonJson: () => JSON.stringify({ positions: [], routes: [] }),
    hitTestFeatureJson: () => "null",
    featureScreenJson: () => "null",
    positionScreenJson: () => "null",
    currentLodJson: () => "{}",
    setMapThemeJson: call("setMapThemeJson"),
    gpuReady: () => true,
    free: call("free"),
  };
}

function tiledMapNode(): UiComponentSceneNode {
  return {
    type: "componentScene",
    controllerId: "controller",
    surfaceId: "surface",
    componentKind: "tiled-map",
    tiledMap: {
      mapFixtureJson: JSON.stringify({ positions: [], routes: [] }),
      cameraJson: JSON.stringify(surfacePinchFixture.gestures.spread.initialCamera),
      renderMode: "vector",
      vectorStyle: "colored",
      lodMode: "automatic",
      tileUrlTemplate: "/tiles/{z}/{x}/{y}.png",
      vectorTileUrlTemplate: "/tiles/{z}/{x}/{y}.pbf",
      layerVisibilityJson: "{}",
      layerStrokeScaleJson: "{}",
      selectionJson: "[]",
      hoverJson: "{}",
      selectionMethod: "rectangle",
      selectionMode: "feature",
    },
  } as UiComponentSceneNode;
}

function pointer(type: string, pointerId: number, clientX: number, clientY: number): PointerEvent {
  return new PointerEvent(type, { bubbles: true, cancelable: true, pointerId, clientX, clientY, pointerType: "touch", button: 0, buttons: 1 });
}

function stubLayout(container: HTMLElement): void {
  const { width, height } = surfacePinchFixture.viewport;
  for (const element of [container, ...Array.from(container.querySelectorAll("*"))] as HTMLElement[]) {
    element.getBoundingClientRect = () => ({ x: 0, y: 0, top: 0, left: 0, right: width, bottom: height, width, height, toJSON: () => ({}) }) as DOMRect;
    Object.defineProperty(element, "clientWidth", { configurable: true, value: width });
    Object.defineProperty(element, "clientHeight", { configurable: true, value: height });
  }
}

async function mountMap(session: MapWasmSession): Promise<{ readonly canvas: HTMLCanvasElement; readonly actions: { action: string; args?: unknown }[] }> {
  const actions: { action: string; args?: unknown }[] = [];
  createMapSessionMock.mockResolvedValueOnce(session);
  const view = render(createElement(TiledMapHost, { node: tiledMapNode(), onAction: (descriptor: ActionDescriptor) => void actions.push({ action: descriptor.action, args: descriptor.args }) }));
  stubLayout(view.container as HTMLElement);
  await act(async () => {
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
  });
  expect((session as ReturnType<typeof createStubSession>).calls).toContain("attachCanvas");
  return { canvas: view.container.querySelector("canvas")!, actions };
}
// #endregion 🧪️Harness

// #region 🤏️PinchLaws
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  createMapSessionMock.mockReset();
});

describe("🤏️ tiled map two-finger gesture", () => {
  it("retires marquee ownership, changes the camera silently, then publishes once after the final lift", async () => {
    const session = createStubSession();
    const { canvas, actions } = await mountMap(session);
    const row = surfacePinchFixture.gestures.spread;
    const map = surfacePinchFixture.surfaces.tiledMap;

    act(() => canvas.dispatchEvent(pointer("pointerdown", row.down[0].pointerId, row.down[0].x, row.down[0].y)));
    const callsBeforeTransfer = session.calls.length;
    act(() => canvas.dispatchEvent(pointer("pointerdown", row.down[1].pointerId, row.down[1].x, row.down[1].y)));
    expect(session.calls.slice(callsBeforeTransfer).filter((call) => call === "pointerUpScreen")).toHaveLength(map.transfer.syntheticPointerUps);

    act(() => {
      for (const event of row.move) window.dispatchEvent(pointer("pointermove", event.pointerId, event.x, event.y));
    });
    expect(session.camera.zoom).toBeCloseTo(row.expectedCamera.zoom, 6);
    expect(session.camera.x).toBeCloseTo(row.expectedCamera.x, 6);
    expect(session.camera.y).toBeCloseTo(row.expectedCamera.y, 6);
    expect(actions.filter((entry) => entry.action === "setCamera")).toHaveLength(map.moves.cameraPublications);
    expect(session.calls.slice(callsBeforeTransfer).filter((call) => call === "pointerMoveScreen")).toHaveLength(map.moves.pointerMoves);

    const callsBeforeLifts = session.calls.length;
    act(() => {
      window.dispatchEvent(pointer("pointerup", row.move[1].pointerId, row.move[1].x, row.move[1].y));
      window.dispatchEvent(pointer("pointerup", row.move[0].pointerId, row.move[0].x, row.move[0].y));
    });
    expect(session.calls.slice(callsBeforeLifts).filter((call) => call === "pointerUpScreen")).toHaveLength(map.lifts.pointerUps);
    expect(actions.filter((entry) => entry.action === "setCamera")).toHaveLength(map.lifts.cameraPublications);
    expect(actions.filter((entry) => entry.action === "interactionSelect" || entry.action === "clearSelection")).toHaveLength(map.lifts.selectionPublications);

    act(() => canvas.dispatchEvent(pointer("pointerdown", 3, 100, 100)));
    act(() => window.dispatchEvent(pointer("pointerup", 3, 100, 100)));
    expect(actions.filter((entry) => entry.action === "clearSelection")).toHaveLength(1);
  });

  it("translates a rigid contact pair without changing zoom", async () => {
    const session = createStubSession();
    const { canvas } = await mountMap(session);
    const row = surfacePinchFixture.gestures.translation;
    act(() => {
      for (const event of row.down) canvas.dispatchEvent(pointer("pointerdown", event.pointerId, event.x, event.y));
      for (const event of row.move) window.dispatchEvent(pointer("pointermove", event.pointerId, event.x, event.y));
    });
    expect(session.camera.x).toBeCloseTo(row.expectedCamera.x, 6);
    expect(session.camera.y).toBeCloseTo(row.expectedCamera.y, 6);
    expect(session.camera.zoom).toBeCloseTo(row.expectedCamera.zoom, 6);
  });
});
// #endregion 🤏️PinchLaws
