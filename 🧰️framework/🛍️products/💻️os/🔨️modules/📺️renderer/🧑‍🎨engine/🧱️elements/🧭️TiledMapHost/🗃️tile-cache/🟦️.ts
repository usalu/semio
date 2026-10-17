/** 🗃️ TiledMapHost tile cache primitives: the byte-bounded LRU, the bounded set and the leading and trailing debounce. */
//#region 🔖️TiledMapHost
//#region TileCachePrimitives
export type ByteLru = { get(key: string): ArrayBuffer | undefined; set(key: string, value: ArrayBuffer): void; delete(key: string): void; clear(): void };

/** 🪺️ Insertion-order `Map` doubling as an LRU: `get`/`set` both re-insert the key so it sorts last,
 * and `set` evicts from the front (least-recently-used) until `maxBytes` is respected. No external
 * library — this is the idiomatic minimal LRU over a plain `Map`. */
export function createByteLru(maxBytes: number): ByteLru {
  const entries = new Map<string, ArrayBuffer>();
  let totalBytes = 0;
  const touch = (key: string, value: ArrayBuffer): void => {
    entries.delete(key);
    entries.set(key, value);
  };
  const evictUntilWithinBudget = (): void => {
    for (const [key, value] of entries) {
      if (totalBytes <= maxBytes) break;
      entries.delete(key);
      totalBytes -= value.byteLength;
    }
  };
  return {
    get(key) {
      const value = entries.get(key);
      if (value === undefined) return undefined;
      touch(key, value);
      return value;
    },
    set(key, value) {
      const existing = entries.get(key);
      if (existing !== undefined) totalBytes -= existing.byteLength;
      touch(key, value);
      totalBytes += value.byteLength;
      evictUntilWithinBudget();
    },
    delete(key) {
      const existing = entries.get(key);
      if (existing === undefined) return;
      entries.delete(key);
      totalBytes -= existing.byteLength;
    },
    clear() {
      entries.clear();
      totalBytes = 0;
    },
  };
}

export type BoundedSet = { has(key: string): boolean; add(key: string): void; clear(): void };

/** 🪺️ Insertion-order `Set` bounded by entry count, evicting the oldest key once `maxEntries` is
 * exceeded — used for the tile-miss records, which carry no byte weight of their own. */
export function createBoundedSet(maxEntries: number): BoundedSet {
  const keys = new Set<string>();
  return {
    has: (key) => keys.has(key),
    add(key) {
      keys.delete(key);
      keys.add(key);
      for (const oldest of keys) {
        if (keys.size <= maxEntries) break;
        keys.delete(oldest);
      }
    },
    clear() {
      keys.clear();
    },
  };
}

export type LeadingTrailingDebounce = { call(): void; dispose(): void };

/** 🥁️ Fires `run` immediately on the leading edge whenever no window is open, then coalesces every
 * further `call()` made before the window closes into exactly one trailing `run` at close — so a
 * sustained burst (e.g. a drag) still gets its first tile refresh instantly instead of waiting for
 * the whole gesture to end, the way a trailing-only `setTimeout` debounce would. */
export function createLeadingTrailingDebounce(run: () => void, waitMs: number): LeadingTrailingDebounce {
  let timer: ReturnType<typeof setTimeout> | null = null;
  let trailingQueued = false;
  const armWindow = (): void => {
    timer = setTimeout(() => {
      timer = null;
      if (trailingQueued) {
        trailingQueued = false;
        run();
        armWindow();
      }
    }, waitMs);
  };
  return {
    call() {
      if (timer === null) {
        run();
        armWindow();
        return;
      }
      trailingQueued = true;
    },
    dispose() {
      if (timer !== null) {
        clearTimeout(timer);
        timer = null;
      }
      trailingQueued = false;
    },
  };
}
//#endregion TileCachePrimitives
//#endregion 🔖️TiledMapHost
