//#region 🗄️RendererModuleCache
/** @emoji 🗄️ Persists the COMPILED renderer `WebAssembly.Module` across reloads, so only the first boot on
 * a given artifact pays for compiling it.
 *
 * The renderer wasm is 76 048 601 B. Compiling it is the single most expensive thing a cold boot does, it
 * is browser-owned work no chunking can shorten, and it is repeated in full on every reload — which is what
 * put a healthy Worker past the boot watchdog's window in the first place (`../🫀️boot-liveness/🟦️.ts`).
 * A `WebAssembly.Module` is structured-cloneable in Chromium and Gecko, so IndexedDB can hold the compiled
 * artifact itself rather than its bytes, and a second boot instantiates in milliseconds.
 *
 * 🔑️ Identity. The record is validated against the artifact's HTTP validator — `ETag` when the server
 * offers one, else `Last-Modified` + `Content-Length` — read by a `HEAD` that costs one round trip and
 * precedes the 76 MB body. A content digest would be the stronger key, but Web Crypto has no streaming
 * digest: hashing would mean retaining the whole body purely to hash it, spending more than the compile the
 * cache saves and defeating `compileStreaming`. The validator is the identity the server itself asserts, a
 * changed artifact never matches it, and the tag is recorded on the row so a reader can see what was
 * matched. `staleCacheTag` exists for the boot to report the miss it took.
 *
 * 🧯️ Every entry point fails soft and never throws into the boot: a browser that refuses to clone a
 * `Module` (WebKit), a private window with no IndexedDB, a quota rejection and a corrupt row all read as
 * "no cache", and the boot compiles exactly as it did before. */

const CACHE_DATABASE = "semio-wgpu-renderer-modules";
const CACHE_STORE = "modules";
const CACHE_VERSION = 1;
/** @emoji ⏳️ A cache lookup must never be the thing that stalls a boot: IndexedDB blocks behind another
 * tab's pending upgrade, and a boot that waits on it forever is worse than one that compiles. */
const CACHE_OPERATION_TIMEOUT_MS = 5_000;

/** @emoji 🗄️ One cached compiled renderer artifact. */
export type RendererModuleCacheHit = {
  readonly module: WebAssembly.Module;
  readonly tag: string;
  readonly byteLength: number;
  readonly storedAtMs: number;
};

type CacheRow = { readonly tag: string; readonly byteLength: number; readonly storedAtMs: number; readonly module: WebAssembly.Module };

function cacheKey(url: string): string {
  try {
    return new URL(url, "http://renderer.invalid/").pathname;
  } catch {
    return url;
  }
}

function withTimeout<T>(operation: Promise<T>): Promise<T | undefined> {
  return Promise.race([
    operation.catch(() => undefined),
    new Promise<undefined>((resolve) => setTimeout(() => resolve(undefined), CACHE_OPERATION_TIMEOUT_MS)),
  ]);
}

function requested<T>(request: IDBRequest<T>): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error("indexeddb request failed"));
  });
}

async function openCache(): Promise<IDBDatabase | undefined> {
  const factory = (globalThis as { indexedDB?: IDBFactory }).indexedDB;
  if (!factory) return undefined;
  return withTimeout(
    new Promise<IDBDatabase>((resolve, reject) => {
      const request = factory.open(CACHE_DATABASE, CACHE_VERSION);
      request.onupgradeneeded = () => {
        if (!request.result.objectStoreNames.contains(CACHE_STORE)) request.result.createObjectStore(CACHE_STORE);
      };
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error ?? new Error("indexeddb open failed"));
      request.onblocked = () => reject(new Error("indexeddb open blocked"));
    }),
  );
}

/** @emoji 🔑️ The artifact's server-asserted identity, via one `HEAD`. An empty answer means the server
 * offered no validator at all, which disables the cache rather than risking a stale module. */
export async function rendererArtifactTag(url: string): Promise<string> {
  try {
    const response = await fetch(url, { method: "HEAD", cache: "no-cache" });
    if (!response.ok) return "";
    const etag = response.headers.get("etag") ?? "";
    if (etag) return `etag:${etag}`;
    const modified = response.headers.get("last-modified") ?? "";
    const length = response.headers.get("content-length") ?? "";
    return modified && length ? `mtime:${modified}:${length}` : "";
  } catch {
    return "";
  }
}

/** @emoji 🗄️ The compiled module for this artifact, or `undefined` for every kind of miss. */
export async function readCachedRendererModule(url: string, tag: string): Promise<RendererModuleCacheHit | undefined> {
  if (!tag) return undefined;
  const database = await openCache();
  if (!database) return undefined;
  try {
    const row = await withTimeout(requested<CacheRow | undefined>(database.transaction(CACHE_STORE, "readonly").objectStore(CACHE_STORE).get(cacheKey(url))));
    if (!row || row.tag !== tag || !(row.module instanceof WebAssembly.Module)) return undefined;
    return { module: row.module, tag: row.tag, byteLength: row.byteLength, storedAtMs: row.storedAtMs };
  } catch {
    return undefined;
  } finally {
    database.close();
  }
}

/** @emoji 🗄️ Stores the compiled module, replacing whatever this artifact's slot held. Answers whether the
 * write actually landed, so the boot reports a real cache state rather than an intent. */
export async function writeCachedRendererModule(url: string, tag: string, module: WebAssembly.Module, byteLength: number, nowMs: number): Promise<boolean> {
  if (!tag) return false;
  const database = await openCache();
  if (!database) return false;
  try {
    const store = database.transaction(CACHE_STORE, "readwrite").objectStore(CACHE_STORE);
    const row: CacheRow = { tag, byteLength, storedAtMs: nowMs, module };
    return (await withTimeout(requested<IDBValidKey>(store.put(row, cacheKey(url))))) !== undefined;
  } catch {
    return false;
  } finally {
    database.close();
  }
}

/** @emoji 🧹️ Drops this artifact's slot — used when a compile succeeds against a tag the cache disagrees
 * with, so a wrong row can never outlive the boot that noticed it. */
export async function evictCachedRendererModule(url: string): Promise<void> {
  const database = await openCache();
  if (!database) return;
  try {
    await withTimeout(requested<undefined>(database.transaction(CACHE_STORE, "readwrite").objectStore(CACHE_STORE).delete(cacheKey(url))));
  } catch {
    return;
  } finally {
    database.close();
  }
}
//#endregion 🗄️RendererModuleCache
