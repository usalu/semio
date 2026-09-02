// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/TiledMapHost/component.test.ts
/** @emoji 🧪️ Pure-logic tests for `TiledMapHost`'s tile-refresh perf primitives (ticket
 * 26/08/29/GIS-MAP-END-TO-END): the byte-budgeted LRU (`createByteLru`), the bounded miss-key set
 * (`createBoundedSet`), the leading+trailing refresh debounce (`createLeadingTrailingDebounce`), and
 * `MapRenderer`'s `hasTile`-guarded upload path. Not wired into `@semio-tech/framework-renderer-react`'s
 * own nx `test` target (its `vitest.config.ts` `root` is the `⚛️react` package dir, a sibling of —
 * not an ancestor of — `🧱️elements/`) — run directly, e.g.
 * `bunx vitest run --config 🧪️vitest.config.ts ../../../../🧱️elements/TiledMapHost/🧪️component.test.ts`
 * from the `⚛️react` package dir, mirroring `AgentBridge`/`TaskManager`'s colocated test files. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterEach, describe, expect, it, vi } from "vitest";
import { createByteLru, createBoundedSet, createLeadingTrailingDebounce, MapRenderer } from "./🟦️.tsx";
import type { MapWasmSession } from "../WasmSessionLoader/🟦️.tsx";
// #endregion 🔌️Adapters

const bytes = (n: number): ArrayBuffer => new ArrayBuffer(n);

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
