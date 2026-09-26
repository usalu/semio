// #region 🧲️Header
/** @emoji 🪶️ The idle budget of every wasm canvas surface family, replayed from `🧫️fixtures/🪶️surface-idle-frames/🔣️.json`
 * against the REAL host components mounted by React DOM (the third-party renderer) with a stub wasm session on a faked
 * frame + wall clock: a settled surface paints no frame while nothing changes, a change paints it again, unmount stops
 * it. A host that repaints every animation frame fails every `idle` step. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { createElement, type ReactElement } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ActionDescriptor, UiComponentSceneNode } from "@semio-tech/framework";
import { Board2dHost } from "../../🧱️elements/🖥️Board2dHost/🟦️.tsx";
import { Paint2dHost } from "../../🧱️elements/🖌️Paint2dHost/🟦️.tsx";
import * as sessionLoader from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { BoardSessionFactoryContext, createBoardPeerScope, type Board2dWasmSession, type RasterWasmSession } from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import fixture from "../../🧫️fixtures/🪶️surface-idle-frames/🔣️.json";
// #endregion 🔌️Adapters

// #region 🧪️Harness
type RenderBounds = { readonly min: number; readonly max: number };
type Step = { readonly do: string; readonly advanceMs: number; readonly renders: RenderBounds };
type Family = { readonly family: string; readonly host: string; readonly steps: readonly Step[] };
type StubSession = { readonly renders: () => number };

const CAMERA_JSON = JSON.stringify({ x: 0, y: 0, zoom: 1 });

/** 🧪️ A wasm session reduced to a renderFrame counter: every other method is a no-op answering the neutral value its
 * caller parses, so the hosts run their real lifecycle, sync and scheduling code without a GPU. */
function stubSession<Session>(): Session & StubSession {
  let renders = 0;
  const answers: Record<string, unknown> = {
    renderFrame: () => {
      renders += 1;
    },
    renders: () => renders,
    attach_canvas: async () => undefined,
    attachCanvas: async () => undefined,
    gpuReady: () => true,
    parseFixtureJson: () => true,
    drainEventsJson: () => "[]",
    cameraJson: () => CAMERA_JSON,
    navigatorFitCameraJson: () => CAMERA_JSON,
    pickTargetsAtScreenJson: () => "[]",
    defersDescriptorSyncFromJs: () => false,
  };
  return new Proxy(answers, {
    get(target, key) {
      if (key === "then") return undefined;
      return target[key as string] ?? (() => undefined);
    },
  }) as unknown as Session & StubSession;
}

function board2dNode(gridVisible: boolean): UiComponentSceneNode {
  return {
    controllerId: "controller",
    surfaceId: "surface",
    board2d: {
      fixtureJson: JSON.stringify({ nodes: [], edges: [] }),
      cameraJson: CAMERA_JSON,
      glyphCatalogsJson: "{}",
      selectionJson: "[]",
      interactive: true,
      selectionMethod: "rectangle",
      gridVisible,
      gridSnapEnabled: false,
      gridFactor: 1,
      suggestionOffset: 0,
      brushWeightsJson: "{}",
      placementCompatibilityJson: "{}",
      lodMode: "automatic",
    },
  } as unknown as UiComponentSceneNode;
}

function paint2dNode(brushSize: number): UiComponentSceneNode {
  return {
    controllerId: "controller",
    surfaceId: "surface",
    paint2d: { documentSyncJson: JSON.stringify({ layers: [] }), assetsJson: "{}", cameraJson: CAMERA_JSON, selectionJson: "[]", activeUtility: "select", brushSize, brushOpacity: 1, brushColor: "#2878dc", brushHardness: 1, viewMode: "composite" },
  } as unknown as UiComponentSceneNode;
}

const onAction = (_descriptor: ActionDescriptor) => undefined;

/** 🏗️ The host element of one family, before (`changed = false`) and after its one change. */
function familyElement(family: string, session: StubSession, changed: boolean): ReactElement {
  if (family === "board2d") {
    const factory = { pluginId: "test", appId: "test", instanceId: 1, create: async () => session as unknown as Board2dWasmSession, scope: createBoardPeerScope() };
    return createElement(BoardSessionFactoryContext.Provider, { value: factory }, createElement(Board2dHost, { node: board2dNode(!changed), onAction }));
  }
  if (family === "paint2d") return createElement(Paint2dHost, { node: paint2dNode(changed ? 24 : 12), onAction });
  throw new Error(`surface-idle-frames: no host for family ${family}`);
}
// #endregion 🧪️Harness

// #region 🪶️IdleLaws
const restore: (() => void)[] = [];
beforeEach(() => {
  vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "Date"] });
  const rect = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "getBoundingClientRect");
  const width = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "clientWidth");
  const height = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "clientHeight");
  HTMLElement.prototype.getBoundingClientRect = () => ({ x: 0, y: 0, left: 0, top: 0, right: 800, bottom: 600, width: 800, height: 600, toJSON: () => ({}) }) as DOMRect;
  Object.defineProperty(HTMLElement.prototype, "clientWidth", { configurable: true, get: () => 800 });
  Object.defineProperty(HTMLElement.prototype, "clientHeight", { configurable: true, get: () => 600 });
  const observer = (globalThis as { ResizeObserver?: unknown }).ResizeObserver;
  (globalThis as { ResizeObserver?: unknown }).ResizeObserver = class {
    observe(): void {}
    disconnect(): void {}
  };
  restore.push(() => {
    if (rect) Object.defineProperty(HTMLElement.prototype, "getBoundingClientRect", rect);
    if (width) Object.defineProperty(HTMLElement.prototype, "clientWidth", width);
    if (height) Object.defineProperty(HTMLElement.prototype, "clientHeight", height);
    (globalThis as { ResizeObserver?: unknown }).ResizeObserver = observer;
  });
});

afterEach(() => {
  cleanup();
  for (const undo of restore.splice(0)) undo();
  vi.restoreAllMocks();
  vi.useRealTimers();
});

describe("🪶️ every wasm canvas surface family is idle after it settles (surface-idle-frames fixture)", () => {
  for (const entry of fixture.families as readonly Family[]) {
    it(`${entry.family} (${entry.host})`, async () => {
      const session = stubSession<RasterWasmSession>();
      vi.spyOn(sessionLoader, "createRasterSession").mockResolvedValue(session);
      let view: ReturnType<typeof render> | null = null;
      for (const step of entry.steps) {
        const before = session.renders();
        await act(async () => {
          if (step.do === "mount") view = render(familyElement(entry.family, session, false));
          else if (step.do === "change") view!.rerender(familyElement(entry.family, session, true));
          else if (step.do === "unmount") view!.unmount();
        });
        await act(async () => {
          await vi.advanceTimersByTimeAsync(step.advanceMs);
        });
        const painted = session.renders() - before;
        expect(painted, `${entry.family} / ${step.do}: ${painted} renders`).toBeGreaterThanOrEqual(step.renders.min);
        expect(painted, `${entry.family} / ${step.do}: ${painted} renders`).toBeLessThanOrEqual(step.renders.max);
      }
    });
  }
});
// #endregion 🪶️IdleLaws
