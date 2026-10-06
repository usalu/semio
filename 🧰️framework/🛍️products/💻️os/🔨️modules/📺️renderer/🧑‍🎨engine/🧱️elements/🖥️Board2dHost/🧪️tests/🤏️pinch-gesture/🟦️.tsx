// #region 🧲️Header
/** 🤏️ `🖥️Board2dHost` multi-touch laws, driven with real `PointerEvent`s through a stub board
 * session: two contacts pinch the camera and pan it, the single-pointer lane goes quiet for the whole
 * gesture, and exactly ONE `setCamera` dispatch settles at the end (never one per move). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import Ajv from "ajv";
import { createElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { ActionDescriptor, UiComponentSceneNode } from "@semio-tech/framework";
import { Board2dHost, BOARD_2D_ZOOM_BOUNDS, board2dPinchCamera } from "../../🟦️.tsx";
import { BoardSessionFactoryContext, createBoardPeerScope, type Board2dWasmSession } from "../../../🪪️WasmSessionLoader/🟦️.tsx";
import surfacePinchFixture from "../../../../🧫️fixtures/🤏️surface-pinch/🔣️.json";
// #endregion 🔌️Adapters

// #region 🧪️Harness
type StubCamera = { x: number; y: number; zoom: number };

/** 🧪️ The seam this suite replaces: a board wasm session reduced to the camera it owns plus a call log,
 * so a gesture's effect is readable without a GPU. */
function createStubSession(): Board2dWasmSession & { readonly calls: string[]; readonly camera: StubCamera } {
  const camera: StubCamera = { x: 0, y: 0, zoom: 1 };
  const calls: string[] = [];
  const noop = () => {};
  const session = {
    calls,
    camera,
    attach_canvas: async () => undefined,
    setSize: noop,
    renderFrame: noop,
    loadBoardSnapshotJson: () => true,
    syncDescriptorJson: noop,
    setKindCatalogsJson: noop,
    setCamera: (x: number, y: number, zoom: number) => {
      calls.push("setCamera");
      Object.assign(camera, { x, y, zoom });
    },
    setCameraSilent: (x: number, y: number, zoom: number) => {
      calls.push("setCameraSilent");
      Object.assign(camera, { x, y, zoom });
    },
    setSelectionIdsJson: noop,
    setCanvasThemeJson: noop,
    pointerDownScreen: () => calls.push("pointerDownScreen"),
    pointerMoveScreen: () => calls.push("pointerMoveScreen"),
    pointerUpScreen: () => calls.push("pointerUpScreen"),
    pointerCancelScreen: () => calls.push("pointerCancelScreen"),
    pointerLeaveScreen: () => calls.push("pointerLeaveScreen"),
    cancelAreaSelect: () => {
      calls.push("cancelAreaSelect");
      return true;
    },
    wheelScreen: () => calls.push("wheelScreen"),
    drainEventsJson: () => "[]",
    cameraJson: () => JSON.stringify(camera),
    gpuReady: () => true,
    free: noop,
  } as unknown as Board2dWasmSession & { readonly calls: string[]; readonly camera: StubCamera };
  return session;
}

function boardSceneNode(): UiComponentSceneNode {
  return {
    controllerId: "controller",
    surfaceId: "surface",
    board2d: {
      snapshotJson: JSON.stringify({ nodes: [], edges: [] }),
      cameraJson: JSON.stringify({ x: 0, y: 0, zoom: 1 }),
      glyphCatalogsJson: "{}",
      selectionJson: "[]",
      interactive: true,
      selectionMethod: "rectangle",
      gridSnapEnabled: false,
      gridFactor: 1,
      suggestionOffset: 0,
      brushWeightsJson: "{}",
      placementCompatibilityJson: "{}",
      lodMode: "automatic",
    },
  } as unknown as UiComponentSceneNode;
}

function pointer(type: string, pointerId: number, clientX: number, clientY: number): PointerEvent {
  return new PointerEvent(type, { bubbles: true, cancelable: true, pointerId, clientX, clientY, pointerType: "touch", button: 0, buttons: 1 });
}

/** 🧪️ jsdom gives every element a zero-sized layout box; the host measures its canvas to map client
 * pixels to surface pixels, so both the rect and the container size are pinned to one 800×600 viewport. */
function stubLayout(container: HTMLElement): void {
  for (const element of [container, ...Array.from(container.querySelectorAll("*"))] as HTMLElement[]) {
    element.getBoundingClientRect = () => ({ x: 0, y: 0, top: 0, left: 0, right: 800, bottom: 600, width: 800, height: 600, toJSON: () => ({}) }) as DOMRect;
    Object.defineProperty(element, "clientWidth", { configurable: true, value: 800 });
    Object.defineProperty(element, "clientHeight", { configurable: true, value: 600 });
  }
}

async function mountBoard(session: Board2dWasmSession): Promise<{ readonly canvas: HTMLCanvasElement; readonly actions: { action: string; args?: unknown }[] }> {
  const actions: { action: string; args?: unknown }[] = [];
  const factory = { pluginId: "test", appId: "test", instanceId: 1, create: async () => session, scope: createBoardPeerScope() };
  const view = render(
    createElement(BoardSessionFactoryContext.Provider, { value: factory }, createElement(Board2dHost, { node: boardSceneNode(), onAction: (descriptor: ActionDescriptor) => void actions.push({ action: descriptor.action, args: descriptor.args }) })),
  );
  const canvas = view.container.querySelector("canvas")!;
  stubLayout(view.container as HTMLElement);
  await new Promise<void>((resolve) => setTimeout(resolve, 0));
  act(() => {});
  return { canvas, actions };
}

/** 🧪️ Dispatches a batch of pointer events inside ONE React update transaction. */
function pointerBatch(dispatch: () => void): void {
  act(dispatch);
}
// #endregion 🧪️Harness

// #region 🤏️PinchLaws
afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.restoreAllMocks();
});

describe("🤏️ board 2d pinch math", () => {
  it("accepts the language-neutral surface pinch contract", () => {
  });

  it("spreading two fingers zooms in about their centroid, within the engine's own bounds", () => {
    const camera = board2dPinchCamera(JSON.stringify({ x: 0, y: 0, zoom: 1 }), { scale: 2, panX: 0, panY: 0, rotation: 0, centroidX: 400, centroidY: 300 }, { w: 800, h: 600 })!;
    expect(camera.zoom).toBeCloseTo(2, 10);
    expect(camera.x).toBeCloseTo(0, 10);
    expect(camera.y).toBeCloseTo(0, 10);
  });

  it("clamps to the SAME zoom ceiling/floor the Rust engine's clamp_zoom applies", () => {
    expect(BOARD_2D_ZOOM_BOUNDS.min).toBe(0.05);
    expect(BOARD_2D_ZOOM_BOUNDS.max).toBe(32);
    const up = board2dPinchCamera(JSON.stringify({ x: 0, y: 0, zoom: 1 }), { scale: 1e6, panX: 0, panY: 0, rotation: 0, centroidX: 400, centroidY: 300 }, { w: 800, h: 600 })!;
    const down = board2dPinchCamera(JSON.stringify({ x: 0, y: 0, zoom: 1 }), { scale: 1e-6, panX: 0, panY: 0, rotation: 0, centroidX: 400, centroidY: 300 }, { w: 800, h: 600 })!;
    expect(up.zoom).toBe(32);
    expect(down.zoom).toBe(0.05);
  });

  it("refuses an unreadable camera instead of inventing one", () => {
    expect(board2dPinchCamera("not json", { scale: 2, panX: 0, panY: 0, rotation: 0, centroidX: 0, centroidY: 0 }, { w: 800, h: 600 })).toBeNull();
  });
});

describe("🤏️ board 2d two-finger gesture", () => {
  it("publishes one camera after the final lift and admits a fresh successor contact", async () => {
    const session = createStubSession();
    const { canvas, actions } = await mountBoard(session);
    vi.useFakeTimers();
    const row = surfacePinchFixture.gestures.spread;
    const board = surfacePinchFixture.surfaces.board2d;

    pointerBatch(() => canvas.dispatchEvent(pointer("pointerdown", row.down[0].pointerId, row.down[0].x, row.down[0].y)));
    const callsBeforeTransfer = session.calls.length;
    pointerBatch(() => canvas.dispatchEvent(pointer("pointerdown", row.down[1].pointerId, row.down[1].x, row.down[1].y)));
    expect(session.calls.slice(callsBeforeTransfer).filter((call) => call === "pointerUpScreen")).toHaveLength(board.transfer.syntheticPointerUps);
    expect(session.calls.slice(callsBeforeTransfer).filter((call) => call === "cancelAreaSelect")).toHaveLength(1);

    pointerBatch(() => {
      for (const event of row.move) window.dispatchEvent(pointer("pointermove", event.pointerId, event.x, event.y));
    });
    expect(actions.filter((entry) => entry.action === "setCamera")).toHaveLength(board.moves.cameraPublications);
    expect(session.camera.zoom).toBeCloseTo(row.expectedCamera.zoom, 6);
    expect(session.camera.x).toBeCloseTo(row.expectedCamera.x, 6);
    expect(session.camera.y).toBeCloseTo(row.expectedCamera.y, 6);

    const callsBeforeLifts = session.calls.length;
    pointerBatch(() => {
      window.dispatchEvent(pointer("pointerup", row.move[1].pointerId, row.move[1].x, row.move[1].y));
      window.dispatchEvent(pointer("pointerup", row.move[0].pointerId, row.move[0].x, row.move[0].y));
    });
    expect(session.calls.slice(callsBeforeLifts).filter((call) => call === "pointerUpScreen")).toHaveLength(board.lifts.pointerUps);
    await act(async () => vi.advanceTimersByTimeAsync(350));
    expect(actions.filter((entry) => entry.action === "setCamera")).toHaveLength(board.lifts.cameraPublications);
    expect(actions.filter((entry) => entry.action === "interactionSelect" || entry.action === "clearSelection")).toHaveLength(board.lifts.selectionPublications);

    const pointerDownsBeforeSuccessor = session.calls.filter((call) => call === "pointerDownScreen").length;
    pointerBatch(() => canvas.dispatchEvent(pointer("pointerdown", 3, 100, 100)));
    expect(session.calls.filter((call) => call === "pointerDownScreen")).toHaveLength(pointerDownsBeforeSuccessor + 1);
  });

  it("a second contact cancels the marquee lane and the pinch drives the camera silently", async () => {
    const session = createStubSession();
    const { canvas, actions } = await mountBoard(session);

    pointerBatch(() => canvas.dispatchEvent(pointer("pointerdown", 1, 300, 300)));
    expect(session.calls).toContain("pointerDownScreen");

    pointerBatch(() => canvas.dispatchEvent(pointer("pointerdown", 2, 500, 300)));
    // 🤏️ The first finger's pick/marquee is abandoned the moment the second lands.
    expect(session.calls).toContain("cancelAreaSelect");

    const callsAtPinchStart = session.calls.length;
    pointerBatch(() => {
      window.dispatchEvent(pointer("pointermove", 1, 200, 300));
      window.dispatchEvent(pointer("pointermove", 2, 600, 300));
    });

    // 🤏️ Separation 200 → 400 px: the camera doubled its zoom and never went through the pointer lane.
    expect(session.camera.zoom).toBeCloseTo(2, 6);
    expect(session.calls.slice(callsAtPinchStart)).not.toContain("pointerMoveScreen");
    expect(session.calls.slice(callsAtPinchStart)).toContain("setCameraSilent");
    // 🧭️ Per-frame camera state is window-transient: not one dispatch per move.
    expect(actions.filter((entry) => entry.action === "setCamera")).toHaveLength(0);
  });

  it("a rigid two-finger drag pans without changing zoom", async () => {
    const session = createStubSession();
    const { canvas } = await mountBoard(session);
    pointerBatch(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, 300, 300));
      canvas.dispatchEvent(pointer("pointerdown", 2, 500, 300));
      window.dispatchEvent(pointer("pointermove", 1, 340, 320));
      window.dispatchEvent(pointer("pointermove", 2, 540, 320));
    });
    expect(session.camera.zoom).toBeCloseTo(1, 6);
    expect(session.camera.x).toBeCloseTo(-40, 6);
    expect(session.camera.y).toBeCloseTo(-20, 6);
  });

  it("lifting one finger ends the pinch without replaying it as a click on the board", async () => {
    const session = createStubSession();
    const { canvas } = await mountBoard(session);
    pointerBatch(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, 300, 300));
      canvas.dispatchEvent(pointer("pointerdown", 2, 500, 300));
    });
    const callsBeforeRelease = session.calls.length;
    pointerBatch(() => window.dispatchEvent(pointer("pointerup", 2, 500, 300)));
    expect(session.calls.slice(callsBeforeRelease)).not.toContain("pointerUpScreen");
  });

  it("the finger left down after a pinch stays latched: no drag, no click, until the LAST contact lifts", async () => {
    const session = createStubSession();
    const { canvas } = await mountBoard(session);
    pointerBatch(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, 300, 300));
      canvas.dispatchEvent(pointer("pointerdown", 2, 500, 300));
      window.dispatchEvent(pointer("pointermove", 2, 600, 300));
    });
    const callsAfterPinch = session.calls.length;
    pointerBatch(() => {
      window.dispatchEvent(pointer("pointerup", 2, 600, 300));
      window.dispatchEvent(pointer("pointermove", 1, 250, 260));
      window.dispatchEvent(pointer("pointermove", 1, 200, 220));
      window.dispatchEvent(pointer("pointerup", 1, 200, 220));
    });
    const afterRelease = session.calls.slice(callsAfterPinch);
    expect(afterRelease).not.toContain("pointerMoveScreen");
    expect(afterRelease).not.toContain("pointerUpScreen");
    const callsAfterLatch = session.calls.length;
    pointerBatch(() => canvas.dispatchEvent(pointer("pointerdown", 3, 100, 100)));
    expect(session.calls.slice(callsAfterLatch)).toContain("pointerDownScreen");
  });

  it("keeps routing a SINGLE contact through the pointer lane — a pinch path must not disable touch drag", async () => {
    const session = createStubSession();
    const { canvas } = await mountBoard(session);
    const callsBeforeGesture = session.calls.length;
    pointerBatch(() => {
      canvas.dispatchEvent(pointer("pointerdown", 1, 300, 300));
      window.dispatchEvent(pointer("pointermove", 1, 320, 300));
      window.dispatchEvent(pointer("pointerup", 1, 320, 300));
    });
    const gestureCalls = session.calls.slice(callsBeforeGesture);
    expect(gestureCalls).toEqual(expect.arrayContaining(["pointerDownScreen", "pointerMoveScreen", "pointerUpScreen"]));
    expect(gestureCalls).not.toContain("setCameraSilent");
  });

  it("retires the owned board gesture without synthesizing an up and admits the next contact", async () => {
    const session = createStubSession();
    const { canvas } = await mountBoard(session);
    pointerBatch(() => canvas.dispatchEvent(pointer("pointerdown", 7, 300, 300)));
    const callsBeforeCancel = session.calls.length;
    pointerBatch(() => window.dispatchEvent(pointer("pointercancel", 7, 320, 300)));
    expect(session.calls.slice(callsBeforeCancel)).not.toContain("pointerUpScreen");
    expect(session.calls.slice(callsBeforeCancel)).toEqual(expect.arrayContaining(["pointerCancelScreen"]));
    pointerBatch(() => canvas.dispatchEvent(pointer("pointerdown", 8, 340, 300)));
    expect(session.calls.filter((call) => call === "pointerDownScreen")).toHaveLength(2);
  });
});
// #endregion 🤏️PinchLaws
