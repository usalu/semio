// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/TiledMapHost/component.test.ts
/** 🧪️ Pure-logic tests for `TiledMapHost`'s tile-refresh perf primitives (ticket
 * 26/08/29/GIS-MAP-END-TO-END): the byte-budgeted LRU (`createByteLru`), the bounded miss-key set
 * (`createBoundedSet`), the leading+trailing refresh debounce (`createLeadingTrailingDebounce`), and
 * `MapRenderer`'s `hasTile`-guarded upload path. Included in the React renderer's long test suite. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterEach, describe, expect, it, vi } from "vitest";
import { auditTime, defer, firstValueFrom, from, mergeMap, NEVER, of, retry, Subject, throwError, timeout } from "rxjs";
import { mapFeatureHoverActionArgs, mapFeatureSelectionActionArgs, parseMapOverlayIndex, resolveMapInteractionSync, TILE_REFRESH_DEBOUNCE_MS } from "../../📐️map-parsing/🟦️.ts";
import { createByteLru, createBoundedSet, createLeadingTrailingDebounce } from "../../🗃️tile-cache/🟦️.ts";
import { MapRenderer } from "../../🖼️map-renderer/🟦️.ts";
import type { MapWasmSession } from "../../../🪪️WasmSessionLoader/🟦️.tsx";
import repaintFixture from "../🧫️fixtures/🔁️repaint/🔣️.json";
import tileRefreshFixture from "../🧫️fixtures/📡️tile-refresh/🔣️.json";
import lifecycleFixture from "@semio-tech/framework-os/modules/renderer/engine/testing/fixtures/tiled-map-gesture-lifecycle.json";
// #endregion 🔌️Adapters

const bytes = (n: number): ArrayBuffer => new ArrayBuffer(n);

describe("MapRenderer idle appearance updates", () => {
  afterEach(() => vi.useRealTimers());

  it.each(repaintFixture.cases)("coalesces $name changes into one paint after the frame loop becomes idle", async ({ method, args }) => {
    vi.useFakeTimers();
    const renderFrame = vi.fn();
    const session = {
      renderFrame, visibleTilesRevision: () => 0, visibleVectorTilesRevision: () => 0,
      visibleTilesJson: () => "[]", visibleVectorTilesJson: () => "[]",
      prefetchTilesJson: () => "[]", prefetchVectorTilesJson: () => "[]",
      setRenderMode: vi.fn(), setVectorStyle: vi.fn(), setLayerVisibilityJson: vi.fn(), setLodMode: vi.fn(),
      setLayerStrokeScaleJson: vi.fn(), syncInteraction: vi.fn(), free: vi.fn(),
    } as unknown as MapWasmSession;
    const renderer = new MapRenderer("/osm/{z}/{x}/{y}.png", "/vt/{z}/{x}/{y}.pbf", session);
    const updates = new Subject<void>();
    const referenceFrame = vi.fn();
    const subscription = updates.pipe(auditTime(0)).subscribe(referenceFrame);
    try {
      renderer.startLoop();
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      const initialFrames = renderFrame.mock.calls.length;
      expect(initialFrames).toBeGreaterThan(0);
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      expect(renderFrame).toHaveBeenCalledTimes(initialFrames);
      for (let index = 0; index < repaintFixture.updatesPerBatch; index++) {
        Reflect.apply(Reflect.get(renderer, method), renderer, args);
        updates.next();
      }
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      expect(referenceFrame).toHaveBeenCalledTimes(repaintFixture.expectedFramesPerBatch);
      const updatedFrames = initialFrames + referenceFrame.mock.calls.length;
      expect(renderFrame).toHaveBeenCalledTimes(updatedFrames);
      await vi.advanceTimersByTimeAsync(repaintFixture.settleMs);
      expect(renderFrame).toHaveBeenCalledTimes(updatedFrames);
    } finally {
      subscription.unsubscribe();
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
    expect(mapFeatureHoverActionArgs("path-2")).toEqual({ domainId: "features", channel: "pointer", targets: JSON.stringify([{ granularity: "feature", id: "path-2" }]) });
    expect(mapFeatureHoverActionArgs(null)).toEqual({ domainId: "features", channel: "pointer", targets: "[]" });
  });

  const layers = [
    { id: "zones", geometry: "area" },
    { id: "paths", geometry: "line", granularity: "path" },
    { id: "pins", geometry: "point", granularity: "pin" },
  ];

  it("syncs the guest's published selection back into the wasm session under the declared granularity", () => {
    expect(resolveMapInteractionSync('{"pins":["pin-1"],"paths":[]}', "null", layers)).toEqual({ granularity: "pin", selectedIdsJson: '["pin-1"]' });
    expect(resolveMapInteractionSync('{"pins":["pin-1"],"paths":["path-2"]}', '{"kind":"path","id":"path-2"}', layers)).toEqual({ granularity: "path", selectedIdsJson: '["path-2"]', hoveredId: "path-2" });
    expect(resolveMapInteractionSync('{"pins":[],"paths":[]}', '{"kind":"pin","id":"pin-1"}', layers)).toEqual({ granularity: "pin", selectedIdsJson: "[]", hoveredId: "pin-1" });
    expect(resolveMapInteractionSync('{"zones":["zone-1"]}', "null", layers)).toEqual({ granularity: "path", selectedIdsJson: "[]" });
    expect(resolveMapInteractionSync("{}", "null", [])).toEqual({ granularity: "feature", selectedIdsJson: "[]" });
  });

  it("indexes the policy's overlay layers and the popup facts of pickable features only", () => {
    const index = parseMapOverlayIndex(
      JSON.stringify({
        overlays: { pins: [{ id: "pin-1", label: "Pin", icon: "crosshair", link: "https://example.org" }, { id: 7 }], zones: [{ id: "zone-1", label: "Zone" }] },
        policy: { overlays: [{ id: "zones", geometry: "area" }, { id: "pins", geometry: "point", granularity: "pin" }] },
      }),
    );
    expect(index.layers).toEqual([{ id: "zones", geometry: "area" }, { id: "pins", geometry: "point", granularity: "pin" }]);
    expect([...index.meta.keys()]).toEqual(["pin"]);
    expect(index.meta.get("pin")?.get("pin-1")).toEqual({ label: "Pin", icon: "crosshair", link: "https://example.org" });
    expect(parseMapOverlayIndex("not json")).toEqual({ layers: [], meta: new Map() });
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

//#region 🔖️TileRefresh
describe("MapRenderer tile refresh", () => {
  const { first, prefetch, latest, settleMs } = tileRefreshFixture;

  async function createRenderer(mode: "image" | "vector" = "image", initialRows = [first]) {
    let visible = initialRows;
    let nearby = [prefetch];
    let revision = 1;
    const upload = vi.fn();
    const session = {
      attachCanvas: async () => undefined,
      setRenderMode: () => undefined,
      visibleTilesRevision: () => revision,
      visibleVectorTilesRevision: () => revision,
      visibleTilesJson: () => JSON.stringify(visible),
      visibleVectorTilesJson: () => JSON.stringify(visible),
      prefetchTilesJson: () => JSON.stringify(nearby),
      prefetchVectorTilesJson: () => JSON.stringify(nearby),
      hasTile: () => false,
      hasVectorTile: () => false,
      uploadTile: upload,
      uploadVectorTile: upload,
      free: vi.fn(),
    };
    const renderer = new MapRenderer("/osm/{z}/{x}/{y}.png", "/vt/{z}/{x}/{y}.pbf", session as unknown as MapWasmSession);
    await renderer.attach({ width: 0, height: 0 } as HTMLCanvasElement, 256, 256, 1);
    renderer.setRenderMode(mode);
    return {
      renderer, session, upload,
      move() { visible = [latest]; nearby = []; revision++; },
      clearPrefetch() { nearby = []; },
    };
  }

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  it("fills each freed request slot while the first visible tile is stalled", async () => {
    vi.useFakeTimers();
    const { zoom: z, y, count, concurrency, completed } = tileRefreshFixture.pipeline;
    const rows = Array.from({ length: count }, (_, x) => ({ z, x, y, key: `${z}/${x}/${y}` }));
    const { renderer, upload, clearPrefetch } = await createRenderer("image", rows);
    clearPrefetch();
    const pending = rows.map(() => Promise.withResolvers<Response>());
    const referenceStarted: string[] = [];
    const reference = from(rows).pipe(mergeMap((row) => {
      referenceStarted.push(row.key);
      return pending[row.x].promise;
    }, concurrency)).subscribe();
    const started: string[] = [];
    const fetchMock = vi.fn((url: string, _options?: RequestInit) => {
      const row = rows.find(({ key }) => url.endsWith(`/${key}.png`))!;
      started.push(row.key);
      return pending[row.x].promise;
    });
    vi.stubGlobal("fetch", fetchMock);
    try {
      void renderer.refreshTiles();
      expect(started).toEqual(referenceStarted);
      expect(started).toHaveLength(concurrency);
      for (const [order, index] of completed.entries()) {
        pending[index].resolve(new Response(bytes(4)));
        await vi.advanceTimersByTimeAsync(0);
        expect(started).toEqual(referenceStarted);
        expect(started).toHaveLength(concurrency + order + 1);
      }
      expect(upload).toHaveBeenCalledTimes(completed.length);
      renderer.dispose();
      fetchMock.mock.calls.forEach(([, options], index) => {
        expect(options?.signal?.aborted).toBe(!completed.includes(index));
      });
      expect(vi.getTimerCount()).toBe(0);
      for (const request of pending) request.resolve(new Response(bytes(4)));
      await vi.advanceTimersByTimeAsync(0);
      expect(upload).toHaveBeenCalledTimes(completed.length);
      expect(started).toHaveLength(concurrency + completed.length);
    } finally {
      renderer.dispose();
      reference.unsubscribe();
      for (const request of pending) request.resolve(new Response(bytes(4)));
    }
  });

  it.each(tileRefreshFixture.deadline.stages)("retries a stalled tile response at the %s deadline", async (stage) => {
    vi.useFakeTimers();
    const { renderer, upload, clearPrefetch } = await createRenderer();
    clearPrefetch();
    const fetchMock = vi.fn(async (_url: string, { signal }: { signal: AbortSignal }) => new Response(bytes(4)))
      .mockImplementationOnce(async (_url, { signal }) => {
        const stall = () => new Promise<never>((_, reject) => {
          signal.addEventListener("abort", () => reject(signal.reason), { once: true });
        });
        if (stage === "headers") return stall();
        return Object.assign(new Response(bytes(4)), { arrayBuffer: stall });
      });
    vi.stubGlobal("fetch", fetchMock);
    const { timeoutMs, retryDelayMs } = tileRefreshFixture.deadline;
    let referenceTimedOut = false;
    const reference = NEVER.pipe(timeout(timeoutMs)).subscribe({ error: () => { referenceTimedOut = true; } });
    try {
      void renderer.refreshTiles();
      await vi.advanceTimersByTimeAsync(timeoutMs - 1);
      const firstRequestSignal = fetchMock.mock.calls[0][1].signal;
      expect(fetchMock).toHaveBeenCalledOnce();
      expect(firstRequestSignal.aborted).toBe(false);
      await vi.advanceTimersByTimeAsync(1);
      expect(referenceTimedOut).toBe(true);
      expect(firstRequestSignal.aborted).toBe(referenceTimedOut);
      await vi.advanceTimersByTimeAsync(retryDelayMs);
      expect(fetchMock).toHaveBeenCalledTimes(2);
      expect(upload).toHaveBeenCalledExactlyOnceWith(first.z, first.x, first.y, new Uint8Array(4));
      expect(vi.getTimerCount()).toBe(0);
    } finally {
      reference.unsubscribe();
      renderer.dispose();
    }
  });

  it.each([["visible", first], ["prefetch", prefetch]] as const)("starts the new viewport while obsolete %s fetches are pending", async (_, stale) => {
    vi.useFakeTimers();
    const { renderer, move, upload } = await createRenderer();
    const pending = Promise.withResolvers<Response>();
    const fetchMock = vi.fn((url: string, _options?: RequestInit) =>
      url.endsWith(`/${stale.key}.png`) ? pending.promise : Promise.resolve(new Response(bytes(4))),
    );
    vi.stubGlobal("fetch", fetchMock);
    try {
      renderer.scheduleRefreshTiles();
      await vi.advanceTimersByTimeAsync(0);
      const staleFetch = fetchMock.mock.calls.find(([url]) => url.endsWith(`/${stale.key}.png`));
      expect(staleFetch).toBeDefined();
      move();
      renderer.scheduleRefreshTiles();
      await vi.advanceTimersByTimeAsync(TILE_REFRESH_DEBOUNCE_MS);
      expect(staleFetch?.[1]?.signal?.aborted).toBe(true);
      expect(upload).toHaveBeenCalledWith(latest.z, latest.x, latest.y, new Uint8Array(4));
      pending.resolve(new Response(bytes(4)));
      await vi.advanceTimersByTimeAsync(0);
      expect(upload).not.toHaveBeenCalledWith(stale.z, stale.x, stale.y, new Uint8Array(4));
    } finally {
      pending.resolve(new Response(bytes(4)));
      renderer.dispose();
    }
  });

  it.each(tileRefreshFixture.cases)("handles $name while the viewport stays idle", async ({ mode, statuses, upload: expectedUpload }) => {
    vi.useFakeTimers();
    const { renderer, upload, clearPrefetch } = await createRenderer(mode === "image" ? "image" : "vector");
    clearPrefetch();
    const statusAt = (attempt: number) => statuses[Math.min(attempt, statuses.length) - 1];
    let referenceAttempts = 0;
    const reference = defer(() => {
      const status = statusAt(++referenceAttempts);
      return status === 200 ? of(true) : throwError(() => status);
    }).pipe(retry({
      count: 2,
      delay: (status) => status === 404 || status === 410 ? throwError(() => status) : of(null),
    }));
    expect(await firstValueFrom(reference).catch(() => false)).toBe(expectedUpload);
    expect(referenceAttempts).toBe(statuses.length);
    const fetchMock = vi.fn(async () => {
      const status = statusAt(fetchMock.mock.calls.length);
      if (status === null) throw new TypeError("network unavailable");
      return new Response(status === 200 ? bytes(4) : null, { status });
    });
    vi.stubGlobal("fetch", fetchMock);
    try {
      void renderer.refreshTiles();
      await vi.advanceTimersByTimeAsync(settleMs);
      expect(fetchMock).toHaveBeenCalledTimes(referenceAttempts);
      if (expectedUpload) expect(upload).toHaveBeenCalledExactlyOnceWith(first.z, first.x, first.y, new Uint8Array(4));
      else expect(upload).not.toHaveBeenCalled();
      if (statuses.length === 1) {
        await renderer.refreshTiles();
        expect(fetchMock).toHaveBeenCalledTimes(1);
      }
      expect(vi.getTimerCount()).toBe(0);
    } finally {
      renderer.dispose();
    }
  });

  it("clears pending retries before freeing the session", async () => {
    vi.useFakeTimers();
    const { renderer, session, clearPrefetch, upload } = await createRenderer();
    clearPrefetch();
    const fetchMock = vi.fn(async () => new Response(null, { status: 503 }));
    vi.stubGlobal("fetch", fetchMock);
    void renderer.refreshTiles();
    await vi.advanceTimersByTimeAsync(0);
    expect(vi.getTimerCount()).toBe(1);
    renderer.dispose();
    expect(vi.getTimerCount()).toBe(0);
    await vi.advanceTimersByTimeAsync(settleMs);
    expect(fetchMock).toHaveBeenCalledOnce();
    expect(session.free).toHaveBeenCalledOnce();
    expect(upload).not.toHaveBeenCalled();
  });
});
//#endregion 🔖️TileRefresh
