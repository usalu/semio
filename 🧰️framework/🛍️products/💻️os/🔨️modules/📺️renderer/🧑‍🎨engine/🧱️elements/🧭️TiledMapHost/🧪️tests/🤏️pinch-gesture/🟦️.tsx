// #region 🧲️Header
/** 🤏️ Mounted `TiledMapHost` touch laws: a second contact retires the single-contact gesture,
 * live camera changes stay local, the last lift publishes once, and the recognizer admits a fresh
 * successor contact. The coordinates and expected camera are shared with the native renderer. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { Vector2 } from "three/src/math/Vector2.js";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { ActionDescriptor, UiComponentSceneNode } from "@semio-tech/framework";
import surfacePinchFixture from "@semio-tech/framework-os/modules/renderer/engine/testing/fixtures/surface-pinch.json";
import touchFixture from "../../../../🧪️tests/🧫️fixtures/👆️tiled-map-touch/🔣️.json";
import keyboardFixture from "../🧫️fixtures/⌨️map-navigation/🔣️.json";
import mouseFixture from "../🧫️fixtures/🖱️mouse-pan/🔣️.json";
import repaintFixture from "../🧫️fixtures/🔁️repaint/🔣️.json";
import wheelFixture from "../🧫️fixtures/🔎️wheel-zoom/🔣️.json";
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

function createStubSession(initialCamera: StubCamera = surfacePinchFixture.gestures.spread.initialCamera, feature: { kind: string; id: string } | null = null) {
  const camera = { ...initialCamera };
  const paintedCamera = { ...initialCamera };
  const calls: string[] = [];
  const call = (name: string) => () => void calls.push(name);
  let pan: { origin: StubCamera; start: Vector2 } | undefined;
  return {
    calls,
    camera,
    paintedCamera,
    attachCanvas: async () => void calls.push("attachCanvas"),
    attachCanvas2d: () => void calls.push("attachCanvas2d"),
    setSize: call("setSize"),
    renderFrame: () => {
      calls.push("renderFrame");
      Object.assign(paintedCamera, camera);
    },
    setCamera: (x, y, zoom) => {
      calls.push("setCamera");
      Object.assign(camera, { x, y, zoom });
    },
    cameraJson: () => JSON.stringify(camera),
    cameraLimitsJson: () => JSON.stringify({ min: 0.05, max: 64 }),
    fitWorldCamera: call("fitWorldCamera"),
    reclampCamera: call("reclampCamera"),
    pointerDownScreen: vi.fn((x: number, y: number, button: number) => {
      calls.push("pointerDownScreen");
      if (button === 1) pan = { origin: { ...camera }, start: new Vector2(x, y) };
    }),
    pointerMoveScreen: vi.fn((x: number, y: number) => {
      calls.push("pointerMoveScreen");
      if (!pan) return;
      const center = new Vector2(pan.origin.x, pan.origin.y).addScaledVector(new Vector2(x, y).sub(pan.start), -1 / camera.zoom);
      Object.assign(camera, { x: center.x, y: center.y });
    }),
    pointerUpScreen: vi.fn((_x: number, _y: number) => {
      calls.push("pointerUpScreen");
      pan = undefined;
    }),
    wheelScreen: vi.fn((x: number, y: number, delta: number) => {
      calls.push("wheelScreen");
      const { width, height } = surfacePinchFixture.viewport;
      const offset = new Vector2(x - width / 2, y - height / 2);
      const zoom = camera.zoom * wheelFixture.zoomFactor ** -Math.sign(delta);
      const center = new Vector2(camera.x, camera.y).addScaledVector(offset, 1 / camera.zoom - 1 / zoom);
      Object.assign(camera, { x: center.x, y: center.y, zoom });
    }),
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
    featuresInRectJson: vi.fn(() => "{}"),
    featuresInPolygonJson: () => "{}",
    hitTestFeatureJson: vi.fn((_x: number, _y: number) => JSON.stringify(feature)),
    featureScreenJson: () => "null",
    currentLodJson: () => "{}",
    setMapThemeJson: call("setMapThemeJson"),
    rendererReady: () => true,
    free: call("free"),
  } satisfies MapWasmSession & { calls: string[]; camera: StubCamera; paintedCamera: StubCamera };
}

function tiledMapNode(camera: StubCamera): UiComponentSceneNode {
  return {
    type: "componentScene",
    controllerId: "controller",
    surfaceId: "surface",
    componentKind: "tiled-map",
    tiledMap: {
      mapDescriptorJson: JSON.stringify({ overlays: {} }),
      cameraJson: JSON.stringify(camera),
      renderMode: "vector",
      vectorStyle: "default",
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

function pointer(type: string, pointerId: number, clientX: number, clientY: number, input: PointerEventInit = {}): PointerEvent {
  return new PointerEvent(type, { bubbles: true, cancelable: true, pointerId, clientX, clientY, pointerType: "touch", button: 0, buttons: 1, ...input });
}

function stubLayout(container: HTMLElement): void {
  const { width, height } = surfacePinchFixture.viewport;
  for (const element of [container, ...Array.from(container.querySelectorAll("*"))] as HTMLElement[]) {
    element.getBoundingClientRect = () => ({ x: 0, y: 0, top: 0, left: 0, right: width, bottom: height, width, height, toJSON: () => ({}) }) as DOMRect;
    Object.defineProperty(element, "clientWidth", { configurable: true, value: width });
    Object.defineProperty(element, "clientHeight", { configurable: true, value: height });
  }
}

async function mountMap(session: ReturnType<typeof createStubSession>, waitForAttach = () => new Promise<void>((resolve) => setTimeout(resolve, 0))) {
  const actions: { action: string; args?: unknown }[] = [];
  createMapSessionMock.mockResolvedValueOnce(session);
  const view = render(createElement(TiledMapHost, { node: tiledMapNode(session.camera), onAction: (descriptor: ActionDescriptor) => void actions.push({ action: descriptor.action, args: descriptor.args }) }));
  stubLayout(view.container as HTMLElement);
  const canvas = view.container.querySelector("canvas")!;
  const captures = new Set<number>();
  canvas.setPointerCapture = (id) => void captures.add(id);
  canvas.hasPointerCapture = (id) => captures.has(id);
  canvas.releasePointerCapture = (id) => void captures.delete(id);
  await act(waitForAttach);
  expect(session.calls).toContain("attachCanvas");
  return { canvas, actions, captures };
}
// #endregion 🧪️Harness

// #region 🤏️PinchLaws
afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.restoreAllMocks();
  createMapSessionMock.mockReset();
});

describe("⌨️ tiled map keyboard navigation", () => {
  it("exposes a focusable map and routes arrows and zoom through the same camera session", async () => {
    const { camera, pan, zoom, ignored } = keyboardFixture;
    const session = createStubSession(camera);
    const { canvas, actions } = await mountMap(session);
    const { width, height } = surfacePinchFixture.viewport;
    const center = new Vector2(width / 2, height / 2);
    const reference = new Vector2(camera.x, camera.y);
    const press = (init: KeyboardEventInit) => {
      const event = new KeyboardEvent("keydown", { ...init, cancelable: true, bubbles: true });
      act(() => canvas.dispatchEvent(event));
      return event.defaultPrevented;
    };
    expect(canvas.getAttribute("role")).toBe("application");
    expect(canvas.getAttribute("aria-label")).toBeTruthy();
    canvas.focus();
    expect(document.activeElement).toBe(canvas);
    for (const { key, offset } of pan) {
      expect(press({ key })).toBe(true);
      reference.addScaledVector(new Vector2(offset[0], offset[1]), -1 / camera.zoom);
      expect(session.camera).toEqual({ x: reference.x, y: reference.y, zoom: camera.zoom });
    }
    for (const { key, wheelDelta } of zoom) {
      expect(press({ key })).toBe(true);
      expect(session.wheelScreen).toHaveBeenLastCalledWith(center.x, center.y, wheelDelta);
    }
    for (const init of ignored) expect(press(init)).toBe(false);
    expect(actions.filter(action => action.action === "setCamera")).toHaveLength(pan.length + zoom.length);
  });
});

describe("🔎️ tiled map zoom repaint", () => {
  it.each(wheelFixture.cases)("paints $input zoom from idle before the camera action returns through the scene", async ({ input, centerOffset, deltaY, expectedCamera }) => {
    vi.useFakeTimers();
    const { initialCamera } = wheelFixture;
    const session = createStubSession(initialCamera);
    const { canvas, actions } = await mountMap(session, async () => { await vi.advanceTimersByTimeAsync(0); });
    const { width, height } = surfacePinchFixture.viewport;
    const x = width / 2 + centerOffset[0];
    const y = height / 2 + centerOffset[1];
    const settle = () => act(async () => { await vi.advanceTimersByTimeAsync(repaintFixture.settleMs); });
    const frames = () => session.calls.filter((call) => call === "renderFrame").length;

    await settle();
    const idleFrames = frames();
    expect(idleFrames).toBeGreaterThan(0);
    await settle();
    expect(frames()).toBe(idleFrames);

    const event = input === "wheel"
      ? new WheelEvent("wheel", { bubbles: true, cancelable: true, clientX: x, clientY: y, deltaY })
      : new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key: "=" });
    act(() => canvas.dispatchEvent(event));
    expect(event.defaultPrevented).toBe(true);
    expect(session.wheelScreen).toHaveBeenCalledExactlyOnceWith(x, y, deltaY);
    expect(session.camera).toEqual(expectedCamera);
    expect(actions).toEqual([{ action: "setCamera", args: { surfaceId: "surface", camera: expectedCamera } }]);
    expect(session.paintedCamera).toEqual(initialCamera);

    await settle();
    expect(session.paintedCamera).toEqual(expectedCamera);
    expect(frames()).toBe(idleFrames + repaintFixture.expectedFramesPerBatch);
  });
});

describe("🤏️ tiled map two-finger gesture", () => {
  it("retires a pending single contact, changes the camera silently, then publishes once after the final lift", async () => {
    vi.useFakeTimers();
    const session = createStubSession();
    const { canvas, actions } = await mountMap(session, async () => { await vi.advanceTimersByTimeAsync(0); });
    const row = surfacePinchFixture.gestures.spread;
    const map = surfacePinchFixture.surfaces.tiledMap;
    const frames = () => session.calls.filter((call) => call === "renderFrame").length;
    const settle = () => act(async () => { await vi.advanceTimersByTimeAsync(repaintFixture.settleMs); });
    await settle();
    const idleFrames = frames();

    act(() => canvas.dispatchEvent(pointer("pointerdown", row.down[0].pointerId, row.down[0].x, row.down[0].y)));
    const callsBeforeTransfer = session.calls.length;
    act(() => canvas.dispatchEvent(pointer("pointerdown", row.down[1].pointerId, row.down[1].x, row.down[1].y)));
    expect(session.calls.slice(callsBeforeTransfer).filter((call) => call === "pointerUpScreen")).toHaveLength(map.transfer.syntheticPointerUps);
    await settle();
    expect(frames()).toBe(idleFrames);

    act(() => {
      for (const event of row.move) window.dispatchEvent(pointer("pointermove", event.pointerId, event.x, event.y));
    });
    expect(session.camera.zoom).toBeCloseTo(row.expectedCamera.zoom, 6);
    expect(session.camera.x).toBeCloseTo(row.expectedCamera.x, 6);
    expect(session.camera.y).toBeCloseTo(row.expectedCamera.y, 6);
    expect(actions.filter((entry) => entry.action === "setCamera")).toHaveLength(map.moves.cameraPublications);
    expect(session.calls.slice(callsBeforeTransfer).filter((call) => call === "pointerMoveScreen")).toHaveLength(map.moves.pointerMoves);
    await settle();
    expect(session.paintedCamera).toEqual(session.camera);
    expect(frames()).toBe(idleFrames + repaintFixture.expectedFramesPerBatch);

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

describe("👆️ tiled map single-contact gesture", () => {
  it("picks the feature under the end of a small-motion tap without panning or marquee", async () => {
    const { start, end, feature } = touchFixture.tap;
    const session = createStubSession(touchFixture.drag.initialCamera, feature);
    const { canvas, actions } = await mountMap(session);

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, start.x, start.y));
      window.dispatchEvent(pointer("pointermove", 1, end.x, end.y));
      window.dispatchEvent(pointer("pointerup", 1, end.x, end.y));
    });

    expect(session.pointerDownScreen).not.toHaveBeenCalled();
    expect(session.featuresInRectJson).not.toHaveBeenCalled();
    expect(session.hitTestFeatureJson).toHaveBeenCalledExactlyOnceWith(end.x, end.y);
    expect(actions.filter(({ action }) => action === "interactionSelect")).toEqual([
      { action: "interactionSelect", args: expect.objectContaining({ method: "pick", targets: JSON.stringify([{ granularity: feature.kind, id: feature.id }]) }) },
    ]);
  });

  it("moves the camera with the finger and publishes the final position once", async () => {
    const { initialCamera, start, move, end, moveCamera, finalCamera } = touchFixture.drag;
    const session = createStubSession(initialCamera, touchFixture.tap.feature);
    const { canvas, actions, captures } = await mountMap(session);
    actions.length = 0;

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, start.x, start.y));
      window.dispatchEvent(pointer("pointermove", 1, move.x, move.y));
    });
    expect(session.camera).toEqual(moveCamera);
    expect(actions).toEqual([]);

    act(() => window.dispatchEvent(pointer("pointerup", 1, end.x, end.y)));
    expect(session.camera).toEqual(finalCamera);
    expect(session.pointerDownScreen).toHaveBeenCalledExactlyOnceWith(start.x, start.y, 1);
    expect(session.pointerUpScreen).toHaveBeenCalledTimes(1);
    expect(session.hitTestFeatureJson).not.toHaveBeenCalled();
    expect(session.featuresInRectJson).not.toHaveBeenCalled();
    expect(captures.size).toBe(0);
    expect(actions).toEqual([{ action: "setCamera", args: { surfaceId: "surface", camera: finalCamera } }]);
  });

  it("suppresses selection when a drag returns to its start", async () => {
    const { initialCamera, start, move } = touchFixture.drag;
    const session = createStubSession(initialCamera, touchFixture.tap.feature);
    const { canvas, actions } = await mountMap(session);

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, start.x, start.y));
      window.dispatchEvent(pointer("pointermove", 1, move.x, move.y));
      window.dispatchEvent(pointer("pointerup", 1, start.x, start.y));
    });
    expect(session.camera).toEqual(initialCamera);
    expect(session.hitTestFeatureJson).not.toHaveBeenCalled();
    expect(actions.filter(({ action }) => action === "interactionSelect" || action === "clearSelection")).toEqual([]);
  });

  it("starts a pan when the release sample reaches the threshold", async () => {
    const { initialCamera, start } = touchFixture.drag;
    const { end, finalCamera } = touchFixture.thresholdRelease;
    const session = createStubSession(initialCamera, touchFixture.tap.feature);
    const { canvas, actions } = await mountMap(session);
    actions.length = 0;

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, start.x, start.y));
      window.dispatchEvent(pointer("pointerup", 1, end.x, end.y));
    });
    expect(session.camera).toEqual(finalCamera);
    expect(session.hitTestFeatureJson).not.toHaveBeenCalled();
    expect(actions.map(({ action }) => action)).toEqual(["setCamera"]);
  });

  it("transfers a live pan to pinch and ignores the remaining finger until release", async () => {
    const { initialCamera, start, move, moveCamera } = touchFixture.drag;
    const session = createStubSession(initialCamera, touchFixture.tap.feature);
    const { canvas, actions, captures } = await mountMap(session);
    actions.length = 0;

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, start.x, start.y));
      window.dispatchEvent(pointer("pointermove", 1, move.x, move.y));
      canvas.dispatchEvent(pointer("pointerdown", 2, 500, 300));
    });
    expect(session.camera).toEqual(moveCamera);
    expect(session.pointerUpScreen).toHaveBeenCalledTimes(1);
    expect(captures).toEqual(new Set([1, 2]));
    expect(actions).toEqual([]);

    act(() => {
      window.dispatchEvent(pointer("pointermove", 2, 600, 300));
      window.dispatchEvent(pointer("pointercancel", 2, 600, 300));
      window.dispatchEvent(pointer("pointermove", 1, 500, 400));
    });
    expect(session.camera.zoom).toBeGreaterThan(initialCamera.zoom);
    expect(session.pointerMoveScreen).toHaveBeenCalledTimes(1);
    expect(actions).toEqual([]);

    act(() => window.dispatchEvent(pointer("pointerup", 1, 500, 400)));
    expect(session.pointerUpScreen).toHaveBeenCalledTimes(1);
    expect(session.hitTestFeatureJson).not.toHaveBeenCalled();
    expect(captures.size).toBe(0);
    expect(actions.map(({ action }) => action)).toEqual(["setCamera"]);
  });

  it("finishes a pan on lost capture and accepts a fresh tap", async () => {
    const { initialCamera, start, move, moveCamera } = touchFixture.drag;
    const session = createStubSession(initialCamera, touchFixture.tap.feature);
    const { canvas, actions, captures } = await mountMap(session);
    actions.length = 0;

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, start.x, start.y));
      window.dispatchEvent(pointer("pointermove", 1, move.x, move.y));
      captures.delete(1);
      canvas.dispatchEvent(pointer("lostpointercapture", 1, 0, 0));
      window.dispatchEvent(pointer("pointerup", 1, move.x, move.y));
      canvas.dispatchEvent(pointer("pointerdown", 2, start.x, start.y));
      window.dispatchEvent(pointer("pointerup", 2, start.x, start.y));
    });
    expect(session.camera).toEqual(moveCamera);
    expect(session.pointerUpScreen).toHaveBeenCalledTimes(1);
    expect(actions.map(({ action }) => action)).toEqual(["setCamera", "interactionSelect"]);
  });
});

describe("🖱️ tiled map mouse gesture", () => {
  it.each(["pointerup", "pointercancel", "lostpointercapture"])("commits the local pan once on %s", async (terminalEvent) => {
    const { initialCamera, start, moves, end, cameraPublicationsDuringDrag, cameraPublicationsAfterDrag } = mouseFixture;
    const session = createStubSession(initialCamera);
    const { canvas, actions, captures } = await mountMap(session);
    const pointerId = 1;
    const middle = { pointerType: "mouse", button: 1, buttons: 4 };
    const release = { ...middle, buttons: 0 };
    const cameraPublications = () => actions.filter(({ action }) => action === "setCamera");

    act(() => canvas.dispatchEvent(pointer("pointerdown", pointerId, start.x, start.y, middle)));
    expect(captures.has(pointerId)).toBe(true);
    for (const move of moves) {
      act(() => window.dispatchEvent(pointer("pointermove", pointerId, move.x, move.y, { ...middle, button: -1 })));
      expect(session.camera).toEqual(move.camera);
    }
    expect(cameraPublications()).toHaveLength(cameraPublicationsDuringDrag);

    act(() => {
      if (terminalEvent === "lostpointercapture") captures.delete(pointerId);
      const target = terminalEvent === "lostpointercapture" ? canvas : window;
      target.dispatchEvent(pointer(terminalEvent, pointerId, end.x, end.y, release));
      window.dispatchEvent(pointer("pointerup", pointerId, end.x, end.y, release));
    });
    const terminalSample = terminalEvent === "pointerup" ? end : moves.at(-1)!;
    expect(session.pointerUpScreen).toHaveBeenCalledExactlyOnceWith(terminalSample.x, terminalSample.y);
    expect(session.camera).toEqual(terminalSample.camera);
    expect(captures.has(pointerId)).toBe(false);
    expect(cameraPublications()).toHaveLength(cameraPublicationsAfterDrag);
    expect(cameraPublications()[0]?.args).toEqual({ surfaceId: "surface", camera: terminalSample.camera });
  });

  it.each([
    { gesture: "pan", button: 1, buttons: 4, cameraPublications: 1 },
    { gesture: "marquee", button: 0, buttons: 1, cameraPublications: 0 },
  ])("returns to idle and clears a cancelled $gesture", async ({ button, buttons, cameraPublications }) => {
    vi.useFakeTimers();
    const { initialCamera, start, end } = mouseFixture;
    const { cancelledPointer, settleMs } = repaintFixture;
    const { pointerId, point } = cancelledPointer;
    const session = createStubSession(initialCamera);
    const { canvas, actions, captures } = await mountMap(session, async () => { await vi.advanceTimersByTimeAsync(0); });
    const input = { pointerType: "mouse", button, buttons };
    const frames = () => session.calls.filter((call) => call === "renderFrame").length;
    const settle = () => act(async () => { await vi.advanceTimersByTimeAsync(settleMs); });

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", pointerId, start.x, start.y, input));
      window.dispatchEvent(pointer("pointermove", pointerId, end.x, end.y, { ...input, button: -1 }));
    });
    await settle();
    if (button === 1) {
      expect(session.paintedCamera).toEqual(session.camera);
      const previousFrames = frames();
      await settle();
      expect(frames()).toBe(previousFrames);
    }
    act(() => {
      window.dispatchEvent(pointer(cancelledPointer.event, pointerId, point.x, point.y, { ...input, buttons: 0 }));
      window.dispatchEvent(pointer("pointerup", pointerId, end.x, end.y, { ...input, buttons: 0 }));
    });
    expect(captures.has(pointerId)).toBe(false);
    expect(actions.map(({ action }) => action)).toEqual(Array(cameraPublications).fill("setCamera"));
    expect(session.featuresInRectJson).not.toHaveBeenCalled();
    await settle();
    const settledFrames = frames();
    await settle();
    expect(frames()).toBe(settledFrames);
  });

  it("still uses primary for marquee and middle for pan", async () => {
    const { initialCamera, start, move, moveCamera } = touchFixture.drag;
    const session = createStubSession(initialCamera);
    const { canvas } = await mountMap(session);
    const primary = { pointerType: "mouse", button: 0, buttons: 1 };
    const middle = { pointerType: "mouse", button: 1, buttons: 4 };

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, start.x, start.y, primary));
      window.dispatchEvent(pointer("pointermove", 1, move.x, move.y, primary));
      window.dispatchEvent(pointer("pointerup", 1, move.x, move.y, primary));
    });
    expect(session.featuresInRectJson).toHaveBeenCalledExactlyOnceWith(start.x, start.y, move.x, move.y, false);
    expect(session.camera).toEqual(initialCamera);

    act(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, start.x, start.y, middle));
      window.dispatchEvent(pointer("pointermove", 1, move.x, move.y, middle));
      window.dispatchEvent(pointer("pointerup", 1, move.x, move.y, middle));
    });
    expect(session.camera).toEqual(moveCamera);
    expect(session.pointerDownScreen).toHaveBeenCalledExactlyOnceWith(start.x, start.y, 1);
  });
});
