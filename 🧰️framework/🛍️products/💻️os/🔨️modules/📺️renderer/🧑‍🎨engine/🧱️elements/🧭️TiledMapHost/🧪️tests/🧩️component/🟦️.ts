// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/TiledMapHost/component.test.ts
/** @emoji 🧪️ Pure-logic tests for `TiledMapHost`'s tile-refresh perf primitives (ticket
 * 26/08/29/GIS-MAP-END-TO-END): the byte-budgeted LRU (`createByteLru`), the bounded miss-key set
 * (`createBoundedSet`), the leading+trailing refresh debounce (`createLeadingTrailingDebounce`), and
 * `MapRenderer`'s `hasTile`-guarded upload path. Included in the React renderer's long test suite. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterEach, describe, expect, it, vi } from "vitest";
import ts from "typescript";
import { createByteLru, createBoundedSet, createLeadingTrailingDebounce, mapFeatureHoverActionArgs, mapFeatureSelectionActionArgs, MapRenderer, resolveMapInteractionSync } from "../../🟦️.tsx";
import type { MapWasmSession } from "../../../🪪️WasmSessionLoader/🟦️.tsx";
import repaintFixture from "./🧫️repaint.json";
import lifecycleFixture from "../../../../🧫️fixtures/♻️tiled-map-gesture-lifecycle/🔣️.json";
import rendererSource from "../../🟦️.tsx?raw";
// #endregion 🔌️Adapters

const bytes = (n: number): ArrayBuffer => new ArrayBuffer(n);

describe("MapRenderer idle appearance updates", () => {
  afterEach(() => vi.useRealTimers());

  it.each(repaintFixture.cases)("repaints $name after the frame loop becomes idle", async ({ method, args }) => {
    vi.useFakeTimers();
    const renderFrame = vi.fn();
    const session = {
      renderFrame, visibleTilesRevision: () => 0, visibleVectorTilesRevision: () => 0,
      visibleTilesJson: () => "[]", visibleVectorTilesJson: () => "[]",
      prefetchTilesJson: () => "[]", prefetchVectorTilesJson: () => "[]",
      setRenderMode: vi.fn(), setVectorStyle: vi.fn(), setLayerVisibilityJson: vi.fn(),
      setLayerStrokeScaleJson: vi.fn(), syncInteraction: vi.fn(), free: vi.fn(),
    } as unknown as MapWasmSession;
    const renderer = new MapRenderer("/osm/{z}/{x}/{y}.png", "/vt/{z}/{x}/{y}.pbf", session);
    try {
      renderer.startLoop();
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      const initialFrames = renderFrame.mock.calls.length;
      expect(initialFrames).toBeGreaterThan(0);
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      expect(renderFrame).toHaveBeenCalledTimes(initialFrames);
      Reflect.apply(Reflect.get(renderer, method), renderer, args);
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      expect(renderFrame.mock.calls.length).toBeGreaterThan(initialFrames);
      const updatedFrames = renderFrame.mock.calls.length;
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      expect(renderFrame).toHaveBeenCalledTimes(updatedFrames);
    } finally {
      renderer.dispose();
    }
  });

  it.each([
    { name: "pan", leftDown: false, middleDown: true },
    { name: "marquee", leftDown: true, middleDown: false },
  ])("returns to idle without publishing feature selection when a $name pointer is cancelled", async ({ leftDown, middleDown }) => {
    vi.useFakeTimers();
    const renderFrame = vi.fn();
    const pointerUpScreen = vi.fn();
    const session = {
      renderFrame, pointerUpScreen, visibleTilesRevision: () => 0, visibleVectorTilesRevision: () => 0,
      visibleTilesJson: () => "[]", visibleVectorTilesJson: () => "[]",
      prefetchTilesJson: () => "[]", prefetchVectorTilesJson: () => "[]", free: vi.fn(),
    } as unknown as MapWasmSession;
    const renderer = new MapRenderer("/osm/{z}/{x}/{y}.png", "/vt/{z}/{x}/{y}.pbf", session);
    const source = ts.createSourceFile("map.tsx", rendererSource, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
    let initializer = "";
    const visit = (node: ts.Node): void => {
      if (ts.isVariableDeclaration(node) && node.name.getText(source) === "onPointerCancel") initializer = node.initializer!.getText(source);
      ts.forEachChild(node, visit);
    };
    visit(source);
    expect(initializer).not.toBe("");
    const callback = ts.transpileModule(`const callback = ${initializer};`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    const pointer = { current: { leftDown, middleDown } };
    const panningRef = { current: true };
    const resetMarquee = vi.fn();
    const releasePointerCapture = vi.fn();
    const mirrorSessionCameraToReact = vi.fn();
    const emitFeatureSelection = vi.fn();
    const cancel = new Function("rendererRef", "pointer", "panningRef", "resetMarquee", "canvas", "mirrorSessionCameraToReact", "clientToLocal", "emitFeatureSelection", `${callback}\nreturn callback;`)(
      { current: renderer },
      pointer,
      panningRef,
      resetMarquee,
      { hasPointerCapture: () => true, releasePointerCapture },
      mirrorSessionCameraToReact,
      () => repaintFixture.cancelledPan.point,
      emitFeatureSelection,
    );
    const target = new EventTarget();
    target.addEventListener(repaintFixture.cancelledPan.event, cancel);
    try {
      renderer.startLoop();
      renderer.beginContinuousInteraction(repaintFixture.cancelledPan.reason);
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      const event = new Event(repaintFixture.cancelledPan.event);
      Object.defineProperty(event, "pointerId", { value: repaintFixture.cancelledPan.pointerId });
      target.dispatchEvent(event);
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      expect(pointer.current.leftDown).toBe(false);
      expect(pointer.current.middleDown).toBe(false);
      expect(panningRef.current).toBe(false);
      expect(pointerUpScreen).toHaveBeenCalledExactlyOnceWith(repaintFixture.cancelledPan.point.x, repaintFixture.cancelledPan.point.y);
      expect(resetMarquee).toHaveBeenCalledOnce();
      expect(releasePointerCapture).toHaveBeenCalledExactlyOnceWith(repaintFixture.cancelledPan.pointerId);
      expect(mirrorSessionCameraToReact).toHaveBeenCalledOnce();
      expect(emitFeatureSelection).not.toHaveBeenCalled();
      const frames = renderFrame.mock.calls.length;
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      expect(renderFrame).toHaveBeenCalledTimes(frames);
    } finally {
      renderer.dispose();
    }
  });

  it("preserves one Map owner across the same React key and retires it before a keyed successor", async () => {
    const { createElement, useEffect, useRef } = await import("react");
    const { fireEvent, render } = await import("@testing-library/react");
    const owners: Array<{ active: boolean; renderer: MapRenderer; free: ReturnType<typeof vi.fn> }> = [];
    function Probe() {
      const owner = useRef<(typeof owners)[number] | null>(null);
      if (owner.current === null) {
        const free = vi.fn();
        const session = { free } as unknown as MapWasmSession;
        owner.current = { active: false, renderer: new MapRenderer("/osm/{z}/{x}/{y}.png", "/vt/{z}/{x}/{y}.pbf", session), free };
        owners.push(owner.current);
      }
      useEffect(() => () => owner.current?.renderer.dispose(), []);
      return createElement("button", { "aria-label": "Map gesture owner", onPointerDown: () => { owner.current!.active = true; } });
    }
    const original = lifecycleFixture.owner.key;
    const successor = lifecycleFixture.cases.find(({ name }) => name === "key-replacement")!.next!.key;
    const view = render(createElement(Probe, { key: original }));
    fireEvent.pointerDown(view.getByRole("button", { name: "Map gesture owner" }));
    expect(owners).toHaveLength(1);
    expect(owners[0]!.active).toBe(true);
    view.rerender(createElement(Probe, { key: original }));
    expect(owners).toHaveLength(1);
    expect(owners[0]!.active).toBe(true);
    view.rerender(createElement(Probe, { key: successor }));
    expect(owners).toHaveLength(2);
    expect(owners[0]!.free).toHaveBeenCalledOnce();
    expect(owners[1]!.active).toBe(false);
    view.unmount();
    expect(owners[1]!.free).toHaveBeenCalledOnce();
  });
});

//#region 🔖️InteractionDispatch
describe("feature interaction dispatch args", () => {
  it("addresses interactionSelect at the features domain's feature granularity", () => {
    expect(mapFeatureSelectionActionArgs(["pos-1", "route-2"], "replace", "pick")).toEqual({
      domainId: "features",
      targets: JSON.stringify([
        { granularity: "feature", id: "pos-1" },
        { granularity: "feature", id: "route-2" },
      ]),
      merge: "replace",
      method: "pick",
    });
  });

  it("de-duplicates ids and carries the marquee method through, mirroring world3dSelectionActionArgs", () => {
    const args = mapFeatureSelectionActionArgs(["pos-1", "pos-1"], "additive", "lasso");
    expect(JSON.parse(args.targets)).toEqual([{ granularity: "feature", id: "pos-1" }]);
    expect(args.merge).toBe("additive");
    expect(args.method).toBe("lasso");
  });

  it("publishes interactionHover on the pointer channel, and an empty targets list clears it", () => {
    expect(mapFeatureHoverActionArgs("route-2")).toEqual({ domainId: "features", channel: "pointer", targets: JSON.stringify([{ granularity: "feature", id: "route-2" }]) });
    expect(mapFeatureHoverActionArgs(null)).toEqual({ domainId: "features", channel: "pointer", targets: "[]" });
  });

  it("syncs the guest's published selection back into the wasm session under the right granularity", () => {
    expect(resolveMapInteractionSync('{"positions":["pos-1"],"routes":[]}', "null")).toEqual({ granularity: "position", selectedIdsJson: '["pos-1"]' });
    expect(resolveMapInteractionSync('{"positions":[],"routes":["route-2"]}', '{"kind":"route","id":"route-2"}')).toEqual({ granularity: "route", selectedIdsJson: '["route-2"]', hoveredId: "route-2" });
    expect(resolveMapInteractionSync('{"positions":[],"routes":[]}', '{"kind":"position","id":"pos-1"}')).toEqual({ granularity: "position", selectedIdsJson: "[]", hoveredId: "pos-1" });
  });
});
//#endregion 🔖️InteractionDispatch

//#region 🔖️ByteLru
describe("createByteLru", () => {
  it("evicts the least-recently-used entry first once the byte budget is exceeded", () => {
    const lru = createByteLru(10);
    lru.set("a", bytes(5));
    lru.set("b", bytes(5));
    lru.set("c", bytes(5));
    expect(lru.get("a")).toBeUndefined();
    expect(lru.get("b")).toBeDefined();
    expect(lru.get("c")).toBeDefined();
  });

  it("treats a get() as a use, protecting the entry from the next eviction", () => {
    const lru = createByteLru(10);
    lru.set("a", bytes(5));
    lru.set("b", bytes(5));
    lru.get("a");
    lru.set("c", bytes(5));
    expect(lru.get("b")).toBeUndefined();
    expect(lru.get("a")).toBeDefined();
    expect(lru.get("c")).toBeDefined();
  });

  it("respects the byte budget rather than an entry count", () => {
    const lru = createByteLru(20);
    lru.set("big", bytes(18));
    lru.set("small", bytes(4));
    expect(lru.get("big")).toBeUndefined();
    expect(lru.get("small")).toBeDefined();
  });

  it("clear() empties the cache and resets the byte total", () => {
    const lru = createByteLru(10);
    lru.set("a", bytes(5));
    lru.clear();
    lru.set("b", bytes(8));
    expect(lru.get("a")).toBeUndefined();
    expect(lru.get("b")).toBeDefined();
  });
});
//#endregion 🔖️ByteLru

//#region 🔖️BoundedSet
describe("createBoundedSet", () => {
  it("evicts the oldest key once the entry cap is exceeded", () => {
    const set = createBoundedSet(2);
    set.add("x");
    set.add("y");
    set.add("z");
    expect(set.has("x")).toBe(false);
    expect(set.has("y")).toBe(true);
    expect(set.has("z")).toBe(true);
  });

  it("clear() empties the set", () => {
    const set = createBoundedSet(2);
    set.add("x");
    set.clear();
    expect(set.has("x")).toBe(false);
  });
});
//#endregion 🔖️BoundedSet

//#region 🔖️LeadingTrailingDebounce
describe("createLeadingTrailingDebounce", () => {
  afterEach(() => vi.useRealTimers());

  it("fires on the leading edge, synchronously, the first time it is called", () => {
    vi.useFakeTimers();
    const run = vi.fn();
    const debounced = createLeadingTrailingDebounce(run, 100);
    debounced.call();
    expect(run).toHaveBeenCalledTimes(1);
    debounced.dispose();
  });

  it("coalesces a burst of calls within the window into a single trailing call", () => {
    vi.useFakeTimers();
    const run = vi.fn();
    const debounced = createLeadingTrailingDebounce(run, 100);
    debounced.call();
    for (let i = 0; i < 9; i += 1) debounced.call();
    expect(run).toHaveBeenCalledTimes(1);
    vi.advanceTimersByTime(100);
    expect(run).toHaveBeenCalledTimes(2);
    vi.advanceTimersByTime(100);
    expect(run).toHaveBeenCalledTimes(2);
    debounced.dispose();
  });

  it("keeps firing roughly every window during a sustained burst, never more than once per window", () => {
    vi.useFakeTimers();
    const run = vi.fn();
    const debounced = createLeadingTrailingDebounce(run, 100);
    debounced.call();
    for (let window = 0; window < 5; window += 1) {
      debounced.call();
      debounced.call();
      vi.advanceTimersByTime(100);
    }
    expect(run).toHaveBeenCalledTimes(6);
    debounced.dispose();
  });

  it("dispose() stops any pending trailing call", () => {
    vi.useFakeTimers();
    const run = vi.fn();
    const debounced = createLeadingTrailingDebounce(run, 100);
    debounced.call();
    debounced.call();
    debounced.dispose();
    vi.advanceTimersByTime(200);
    expect(run).toHaveBeenCalledTimes(1);
  });
});
//#endregion 🔖️LeadingTrailingDebounce

//#region 🔖️HasTileGuardedUpload
function createFakeMapSession(): { session: MapWasmSession; uploadTileCount: () => number } {
  let uploadTileCount = 0;
  let hasTileFlag = false;
  const session = {
    attachCanvas: async () => undefined,
    setRenderMode: () => undefined,
    visibleTilesJson: () => '[{"z":1,"x":2,"y":3,"key":"1/2/3"}]',
    visibleTilesRevision: () => 1,
    prefetchTilesJson: () => "[]",
    prefetchVectorTilesJson: () => "[]",
    hasTile: () => hasTileFlag,
    uploadTile: () => {
      uploadTileCount += 1;
      hasTileFlag = true;
    },
  } as unknown as MapWasmSession;
  return { session, uploadTileCount: () => uploadTileCount };
}

describe("MapRenderer hasTile-guarded upload", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("calls uploadTile once for a tile across two refreshes once hasTile reports it already present", async () => {
    const { session, uploadTileCount } = createFakeMapSession();
    const fetchMock = vi.fn(async () => ({ ok: true, arrayBuffer: async () => bytes(4) }) as unknown as Response);
    vi.stubGlobal("fetch", fetchMock);
    const renderer = new MapRenderer("/osm/{z}/{x}/{y}.png", "/vt/{z}/{x}/{y}.pbf", session);
    const canvas = { width: 0, height: 0 } as unknown as HTMLCanvasElement;
    await renderer.attach(canvas, 256, 256, 1);
    renderer.setRenderMode("image");
    await renderer.refreshTiles();
    await renderer.refreshTiles();
    expect(uploadTileCount()).toBe(1);
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });
});
//#endregion 🔖️HasTileGuardedUpload
