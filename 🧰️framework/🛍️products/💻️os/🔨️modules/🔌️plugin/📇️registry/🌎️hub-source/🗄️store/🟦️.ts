/**
 * 🗄️ The persisted local-only store of hub-installed plugin modules — one Cache Storage cache, content-addressed:
 * every file once under `blob/<sha256>`, every manifest under `manifest/<bundleSha256>`, and one record per catalog
 * generation and bundle under `record/<generationId>/<bundleSha256>`, written LAST so a record always names a complete
 * bundle. The store's service worker serves `/_semio/plugin-modules/<generationId>/<bundleSha256>/<path>` from it and
 * re-verifies the manifest and the file (length, SHA-256, BLAKE3) on every load; a file that no longer verifies is
 * deleted with its record, so the next acquisition reinstalls it. Shared by the page and the service worker; every
 * function takes the cache and the origin, so the law replays it against an in-memory cache.
 * @see ../🧬️schema/🔣️.json
 * @see ../🧫️fixtures/🗄️store/🔣️.json
 */
import {
  decodeTrustedPluginModuleBundleV1,
  PLUGIN_MODULE_STORE_V1,
  trustedPluginModuleBundleSha256V1,
  trustedPluginModuleSourceOfEntryV1,
  validatePluginModuleStoreRecordV1,
  validTrustedPluginModulePathV1,
  verifyTrustedPluginModuleFileV1,
  type PluginModuleStoreRecordV1,
  type TrustedPluginModuleBundleV1,
  type TrustedPluginModuleFileV1,
} from "../🧬️schema/🟦️.ts";

/** 🗄️ The part of a Cache Storage `Cache` the store uses. */
export type PluginModuleCacheV1 = Readonly<{
  match(request: string): Promise<Response | undefined>;
  put(request: string, response: Response): Promise<void>;
  delete(request: string): Promise<boolean>;
  keys(): Promise<readonly Readonly<{ url: string }>[]>;
}>;

/** 🚫️ The store cannot hold what an install needs: the platform refused the write for lack of space. */
export class PluginModuleStoreQuotaError extends Error {
  override readonly name = "PluginModuleStoreQuotaError";
}

const DIGEST = /^[0-9a-f]{64}$/u;

/** 🔑️ The store's request keys, absolute under `origin`. */
export function pluginModuleStoreKeysV1(origin: string) {
  const root = `${origin}${PLUGIN_MODULE_STORE_V1.storeRoot}`;
  return Object.freeze({
    recordPrefix: `${root}/record/`,
    manifestPrefix: `${root}/manifest/`,
    blobPrefix: `${root}/blob/`,
    record: (generationId: string, bundleSha256: string) => `${root}/record/${generationId}/${bundleSha256}`,
    manifest: (bundleSha256: string) => `${root}/manifest/${bundleSha256}`,
    blob: (sha256: string) => `${root}/blob/${sha256}`,
  });
}

/** 🔗️ The same-origin URL one stored module file is loaded from, its path segments percent-encoded. */
export function storedPluginModuleUrlV1(generationId: string, bundleSha256: string, path: string): string {
  return `${PLUGIN_MODULE_STORE_V1.serveRoute}/${generationId}/${bundleSha256}/${path.split("/").map(encodeURIComponent).join("/")}`;
}

/** 🔗️ Reads a stored-module URL path back; anything that is not exactly one generation, one bundle and one canonical path is not one. */
export function parseStoredPluginModuleUrlV1(pathname: string): Readonly<{ generationId: string; bundleSha256: string; path: string }> | null {
  const prefix = `${PLUGIN_MODULE_STORE_V1.serveRoute}/`;
  if (!pathname.startsWith(prefix)) return null;
  const segments = pathname.slice(prefix.length).split("/");
  if (segments.length < 3) return null;
  const [generationId, bundleSha256, ...rest] = segments;
  let path: string;
  try {
    path = rest.map((segment) => decodeURIComponent(segment)).join("/");
  } catch {
    return null;
  }
  if (!DIGEST.test(generationId!) || !DIGEST.test(bundleSha256!) || rest.some((segment) => segment.length === 0) || !validTrustedPluginModulePathV1(path)) return null;
  return Object.freeze({ generationId: generationId!, bundleSha256: bundleSha256!, path });
}

/** 🏷️ The media type a module file is loaded with, from its extension — the hub serves the same types. */
export function storedPluginModuleContentTypeV1(path: string): string {
  const extension = path.slice(path.lastIndexOf(".") + 1);
  if (extension === "js" || extension === "mjs") return "text/javascript; charset=utf-8";
  if (extension === "wasm") return "application/wasm";
  if (extension === "json") return "application/json";
  return "application/octet-stream";
}

async function bytesOf(response: Response | undefined): Promise<Uint8Array | null> {
  return response === undefined ? null : new Uint8Array(await response.arrayBuffer());
}

/** 📇️ Every valid record in the store; a record that does not validate is deleted, never trusted. */
export async function readPluginModuleStoreRecordsV1(cache: PluginModuleCacheV1, origin: string): Promise<readonly PluginModuleStoreRecordV1[]> {
  const keys = pluginModuleStoreKeysV1(origin);
  const records: PluginModuleStoreRecordV1[] = [];
  for (const request of await cache.keys()) {
    if (!request.url.startsWith(keys.recordPrefix)) continue;
    try {
      const record = validatePluginModuleStoreRecordV1(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode((await bytesOf(await cache.match(request.url)))!)));
      if (request.url !== keys.record(record.generationId, record.entry.bundleSha256)) throw new Error("record key differs from its content");
      records.push(record);
    } catch {
      await cache.delete(request.url);
    }
  }
  return records;
}

/** 🧩️ A record's manifest, re-verified against its content address and its package; `null` when absent or not verifying. */
export async function readStoredPluginModuleBundleV1(cache: PluginModuleCacheV1, origin: string, record: PluginModuleStoreRecordV1): Promise<Readonly<{ bundle: TrustedPluginModuleBundleV1; missing: readonly TrustedPluginModuleFileV1[] }> | null> {
  const keys = pluginModuleStoreKeysV1(origin);
  const bytes = await bytesOf(await cache.match(keys.manifest(record.entry.bundleSha256)));
  if (bytes === null || (await trustedPluginModuleBundleSha256V1(bytes)) !== record.entry.bundleSha256) return null;
  let bundle: TrustedPluginModuleBundleV1;
  try {
    bundle = decodeTrustedPluginModuleBundleV1(bytes, trustedPluginModuleSourceOfEntryV1(record.entry));
  } catch {
    return null;
  }
  const missing: TrustedPluginModuleFileV1[] = [];
  for (const file of bundle.files) if ((await cache.match(keys.blob(file.sha256))) === undefined) missing.push(file);
  return Object.freeze({ bundle, missing: Object.freeze(missing) });
}

/** 📦️ Whether a file with this content address is already stored (another module may share it). */
export async function storedPluginModuleBlobPresentV1(cache: PluginModuleCacheV1, origin: string, sha256: string): Promise<boolean> {
  return (await cache.match(pluginModuleStoreKeysV1(origin).blob(sha256))) !== undefined;
}

async function put(cache: PluginModuleCacheV1, request: string, bytes: Uint8Array, contentType: string): Promise<void> {
  try {
    await cache.put(request, new Response(bytes as Uint8Array<ArrayBuffer>, { headers: { "content-type": contentType, "content-length": String(bytes.byteLength) } }));
  } catch (error) {
    if (error instanceof Error && (error.name === "QuotaExceededError" || /quota/iu.test(error.message))) throw new PluginModuleStoreQuotaError(`the plugin module store is full: ${error.message}`);
    throw error;
  }
}

/** 📦️ Stores one verified file under its content address. The caller has verified it against its manifest. */
export async function storePluginModuleBlobV1(cache: PluginModuleCacheV1, origin: string, file: TrustedPluginModuleFileV1, bytes: Uint8Array): Promise<void> {
  if (!(await verifyTrustedPluginModuleFileV1(file, bytes))) throw new Error(`plugin module file ${file.path} differs from its manifest`);
  await put(cache, pluginModuleStoreKeysV1(origin).blob(file.sha256), bytes, "application/octet-stream");
}

/** 🧾️ Commits one complete bundle: its manifest, then its record. Refuses while any listed file is absent. */
export async function commitPluginModuleBundleV1(cache: PluginModuleCacheV1, origin: string, record: PluginModuleStoreRecordV1, manifestBytes: Uint8Array): Promise<void> {
  const keys = pluginModuleStoreKeysV1(origin);
  if ((await trustedPluginModuleBundleSha256V1(manifestBytes)) !== record.entry.bundleSha256) throw new Error("plugin module manifest differs from its content address");
  const bundle = decodeTrustedPluginModuleBundleV1(manifestBytes, trustedPluginModuleSourceOfEntryV1(record.entry));
  for (const file of bundle.files) if (!(await storedPluginModuleBlobPresentV1(cache, origin, file.sha256))) throw new Error(`plugin module file ${file.path} is not stored`);
  await put(cache, keys.manifest(record.entry.bundleSha256), manifestBytes, "application/json");
  await put(cache, keys.record(record.generationId, record.entry.bundleSha256), new TextEncoder().encode(JSON.stringify(record)), "application/json");
}

/** 📤️ What serving one stored-module path answers. */
export type StoredPluginModuleAnswerV1 =
  | Readonly<{ status: 200; body: Uint8Array; contentType: string }>
  | Readonly<{ status: 404; problem: "plugin-module-store.not-a-module-path" | "plugin-module-store.missing" | "plugin-module-store.corrupt" }>;

/**
 * 📤️ Serves one stored module file, re-verifying the record, the manifest and the file on every load. A file or manifest
 * that no longer verifies is deleted together with its record, so the next acquisition reinstalls the bundle.
 */
export async function serveStoredPluginModuleFileV1(cache: PluginModuleCacheV1, origin: string, pathname: string): Promise<StoredPluginModuleAnswerV1> {
  const address = parseStoredPluginModuleUrlV1(pathname);
  if (address === null) return { status: 404, problem: "plugin-module-store.not-a-module-path" };
  const keys = pluginModuleStoreKeysV1(origin);
  const recordKey = keys.record(address.generationId, address.bundleSha256);
  const recordBytes = await bytesOf(await cache.match(recordKey));
  if (recordBytes === null) return { status: 404, problem: "plugin-module-store.missing" };
  const corrupt = async (...extra: string[]): Promise<StoredPluginModuleAnswerV1> => {
    for (const request of [recordKey, ...extra]) await cache.delete(request);
    return { status: 404, problem: "plugin-module-store.corrupt" };
  };
  let record: PluginModuleStoreRecordV1;
  try {
    record = validatePluginModuleStoreRecordV1(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(recordBytes)));
  } catch {
    return corrupt();
  }
  if (record.generationId !== address.generationId || record.entry.bundleSha256 !== address.bundleSha256) return corrupt();
  const stored = await readStoredPluginModuleBundleV1(cache, origin, record);
  if (stored === null) return corrupt(keys.manifest(address.bundleSha256));
  const file = stored.bundle.files.find((candidate) => candidate.path === address.path);
  if (file === undefined) return { status: 404, problem: "plugin-module-store.missing" };
  const bytes = await bytesOf(await cache.match(keys.blob(file.sha256)));
  if (bytes === null) return corrupt();
  if (!(await verifyTrustedPluginModuleFileV1(file, bytes))) return corrupt(keys.blob(file.sha256));
  return { status: 200, body: bytes, contentType: storedPluginModuleContentTypeV1(file.path) };
}

/** 🧹️ What one collection removed. */
export type PluginModuleStoreCollectionV1 = Readonly<{ records: readonly string[]; manifests: readonly string[]; blobs: readonly string[] }>;

/**
 * 🧹️ Removes every record of a superseded generation that `release` lets go of (a page still running that bundle keeps it),
 * then every manifest no remaining record names and every file no remaining manifest lists. Run under the store lock.
 */
export async function collectPluginModuleStoreGarbageV1(cache: PluginModuleCacheV1, origin: string, currentGenerationId: string, release: (record: PluginModuleStoreRecordV1) => Promise<boolean>): Promise<PluginModuleStoreCollectionV1> {
  const keys = pluginModuleStoreKeysV1(origin);
  const removedRecords: string[] = [], removedManifests: string[] = [], removedBlobs: string[] = [];
  const kept: PluginModuleStoreRecordV1[] = [];
  for (const record of await readPluginModuleStoreRecordsV1(cache, origin)) {
    if (record.generationId !== currentGenerationId && (await release(record))) {
      await cache.delete(keys.record(record.generationId, record.entry.bundleSha256));
      removedRecords.push(keys.record(record.generationId, record.entry.bundleSha256));
    } else kept.push(record);
  }
  const liveManifests = new Set(kept.map((record) => keys.manifest(record.entry.bundleSha256)));
  const liveBlobs = new Set<string>();
  for (const record of kept) {
    const stored = await readStoredPluginModuleBundleV1(cache, origin, record);
    for (const file of stored?.bundle.files ?? []) liveBlobs.add(keys.blob(file.sha256));
  }
  for (const request of await cache.keys()) {
    if (request.url.startsWith(keys.manifestPrefix) && !liveManifests.has(request.url)) {
      await cache.delete(request.url);
      removedManifests.push(request.url);
    } else if (request.url.startsWith(keys.blobPrefix) && !liveBlobs.has(request.url)) {
      await cache.delete(request.url);
      removedBlobs.push(request.url);
    }
  }
  return Object.freeze({ records: Object.freeze(removedRecords), manifests: Object.freeze(removedManifests), blobs: Object.freeze(removedBlobs) });
}
