/**
 * 🌎️ The hub-backed {@link PluginSource}: installs a plugin's browser module from the hub's trusted catalog into this
 * device's persisted plugin module store (`🗄️store`), and loads it from there ever after. The hub serves every package's
 * plugin module as a content-addressed bundle of its current catalog generation
 * (`GET /trusted-catalog/plugin-modules[/{bundleSha256}[/{path}]]`). An install downloads the manifest and every file the
 * store does not already hold (content-addressed: modules share their vendored files), verifies each against its length,
 * SHA-256 and BLAKE3 with byte progress and cancellation, and commits the bundle under the store lock — files, then the
 * manifest, then the record. The store's service worker serves `/_semio/plugin-modules/<generation>/<bundle>/<path>` and
 * re-verifies every file on every load, so the page and its plugin workers only ever execute verified bytes, with or
 * without the hub. A record whose bundle lost a file is reinstalled with a notice; a full store fails the install with a
 * notice; modules of superseded generations are collected once no page runs them (a shared Web Lock per bundle).
 * It announces nothing: a hub module is installed when something opens it.
 * A hub document runs on the module of the catalog generation that serves it ({@link HubPluginSourceV1.installProgram},
 * `🔍️resolution`): the store's copy when complete, else this device's staged module when every file is the bundle's own,
 * else the hub's bundle — never a staged module that differs.
 * @see ./🔍️resolution/🟦️.ts
 * @see ./🗄️store/🟦️.ts
 * @see ./👷️service-worker/🟦️.ts
 * @see ../../../../../../../🌎️hub/🏗️bootstrap/🦀️.rs
 */
import { PluginModuleUnavailableError, type PluginModuleAcquired, type PluginModuleAcquisition, type PluginRegistryEntry, type PluginSource } from "@semio-tech/framework";
import {
  decodeTrustedPluginModuleBundleV1,
  PLUGIN_MODULE_STORE_V1,
  pluginModuleStoreRecordV1,
  TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES,
  TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES,
  trustedPluginModuleBundleSha256V1,
  trustedPluginModuleSourceOfEntryV1,
  TrustedPluginModuleRefusalV1,
  validateTrustedPluginModuleIndexV1,
  verifyTrustedPluginModuleFileV1,
  type PluginModuleStoreRecordV1,
  type TrustedPluginModuleBundleV1,
  type TrustedPluginModuleFileV1,
  type TrustedPluginModuleIndexEntryV1,
  type TrustedPluginModuleIndexV1,
} from "./🧬️schema/🟦️.ts";
import { hubProgramIdV1, localPluginModuleRootV1, resolvePluginModuleSourceV1, type PluginModuleSourceV1 } from "./🔍️resolution/🟦️.ts";
import {
  collectPluginModuleStoreGarbageV1,
  commitPluginModuleBundleV1,
  PluginModuleStoreQuotaError,
  readPluginModuleStoreRecordsV1,
  readStoredPluginModuleBundleV1,
  storedPluginModuleBlobPresentV1,
  storedPluginModuleUrlV1,
  storePluginModuleBlobV1,
  type PluginModuleCacheV1,
} from "./🗄️store/🟦️.ts";

/** 🌐️ The shell's same-origin mount of its hub: the dev server forwards it to `S_HUB_URL`, and a
 * deployment fronts the hub there, so module downloads never cross an origin. */
export const HUB_SAME_ORIGIN_MOUNT = "/_semio/hub";
/** 🛣️ The hub routes this source reads, relative to the hub mount. */
export const HUB_PLUGIN_MODULE_ROUTE = "/trusted-catalog/plugin-modules";
/** 🧯️ The largest index this source reads: 4096 entries of well under 1 KiB each. */
const HUB_PLUGIN_MODULE_INDEX_MAX_BYTES = 4 * 1024 * 1024;
/** ⏱️ How long the shell waits for the store's service worker to control the page before hub installs are unavailable. */
const PLUGIN_MODULE_STORE_CONTROL_DEADLINE_MS = 10_000;

/** 🔔️ What the source tells the person using the shell: a stored module that must be reinstalled, or a store that is full. */
export type HubPluginSourceNoticeV1 = Readonly<{ kind: "reinstalling" | "quota-exceeded"; pluginId: string }>;

/** 🔒️ The part of the Web Locks API the source uses: a held shared lock per running bundle and the store lock. */
export type PluginModuleLocksV1 = Readonly<{
  request<T>(name: string, options: Readonly<{ mode: "shared" | "exclusive"; ifAvailable?: boolean }>, callback: (lock: unknown) => Promise<T> | T): Promise<T>;
}>;

/** 👷️ The part of the service worker container the source uses. */
export type PluginModuleStoreWorkerContainerV1 = Readonly<{
  register(url: string, options: Readonly<{ type: "module"; scope: string }>): Promise<unknown>;
  readonly ready: Promise<unknown>;
  readonly controller: unknown;
  addEventListener(type: "controllerchange", listener: () => void, options?: Readonly<{ once: boolean }>): void;
}>;

export type HubPluginSourceOptionsV1 = Readonly<{
  /** The hub mount the shell reaches its hub through ({@link HUB_SAME_ORIGIN_MOUNT}). */
  hubMount: string;
  /** The store's service worker script (a module worker; it must be allowed scope `/`). */
  storeWorkerUrl: string;
  /** The page origin the store's keys live under. */
  origin: string;
  /** Receives every notice the person should see. */
  onNotice: (notice: HubPluginSourceNoticeV1) => void;
  caches: Readonly<{ open(name: string): Promise<PluginModuleCacheV1> }>;
  locks: PluginModuleLocksV1;
  serviceWorker: PluginModuleStoreWorkerContainerV1;
  persist?: () => Promise<boolean>;
  now?: () => number;
}>;

/** 🧩️ One installed hub program: the catalog generation's module of one plugin, loaded from this device's store. */
export type HubProgramModuleV1 = Readonly<{
  programId: string;
  pluginId: string;
  generationId: string;
  bundleSha256: string;
  moduleUrl: string;
  source: PluginModuleSourceV1;
}>;

/** 🌎️ The hub source plus what only it knows: whether its store controls this page, the hub's current catalog of plugin
 * modules, and the programs hub documents run on. */
export type HubPluginSourceV1 = PluginSource & Readonly<{
  /** Registers the store's service worker and waits until it controls the page — before any plugin worker exists. */
  ready(): Promise<void>;
  /** The hub package whose surfaces open this dialect artifact kind (current index, else this device's records). */
  ownerOfDialect(artifactKind: string): Promise<string | undefined>;
  /** The hub's current generation of plugin modules, or `null` while the hub is unreachable. */
  catalog(signal: AbortSignal): Promise<TrustedPluginModuleIndexV1 | null>;
  /** Installs `entry` of `generationId` as a hub program: from the store when it holds the bundle complete, else from
   * the staged module at `localModuleUrl` when every file of the bundle is byte-identical there, else from the hub. */
  installProgram(generationId: string, entry: TrustedPluginModuleIndexEntryV1, localModuleUrl: string | null, acquisition: PluginModuleAcquisition): Promise<HubProgramModuleV1>;
}>;

/** 📥️ Reads one response body within `maximum` bytes, reporting every chunk. */
async function readBounded(response: Response, maximum: number, onChunk: (bytes: number) => void): Promise<Uint8Array> {
  const declared = Number(response.headers.get("content-length") ?? "0");
  if (declared > maximum) throw new TrustedPluginModuleRefusalV1(`trusted plugin module response declares ${declared} bytes against ${maximum}`);
  const reader = response.body?.getReader();
  if (!reader) return new Uint8Array(await response.arrayBuffer());
  const chunks: Uint8Array[] = [];
  let length = 0;
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    length += value.byteLength;
    if (length > maximum) {
      await reader.cancel();
      throw new TrustedPluginModuleRefusalV1(`trusted plugin module response exceeds ${maximum} bytes`);
    }
    chunks.push(value);
    onChunk(value.byteLength);
  }
  const bytes = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return bytes;
}

async function fetchBytes(url: string, maximum: number, signal: AbortSignal, onChunk: (bytes: number) => void = () => {}): Promise<Uint8Array> {
  const response = await fetch(url, { signal });
  if (!response.ok) throw new TrustedPluginModuleRefusalV1(`trusted plugin module ${url} answered HTTP ${response.status}`);
  return readBounded(response, maximum, onChunk);
}

/** 🔗️ One hub file of one bundle, its path segments percent-encoded. */
function hubFileUrl(hubMount: string, bundleSha256: string, path: string): string {
  return `${hubMount}${HUB_PLUGIN_MODULE_ROUTE}/${bundleSha256}/${path.split("/").map(encodeURIComponent).join("/")}`;
}

/** 🌎️ Creates the hub-backed source; see the module documentation for its store contract. */
export function createHubPluginSource(options: HubPluginSourceOptionsV1): HubPluginSourceV1 {
  const now = options.now ?? (() => Date.now());
  const held = new Set<string>();
  let readiness: Promise<void> | null = null;
  const store = () => options.caches.open(PLUGIN_MODULE_STORE_V1.name);
  const ready = (): Promise<void> => {
    readiness ??= (async () => {
      await options.serviceWorker.register(options.storeWorkerUrl, { type: "module", scope: "/" });
      await options.serviceWorker.ready;
      if (options.serviceWorker.controller) return;
      await new Promise<void>((resolve, reject) => {
        const timer = setTimeout(() => reject(new Error("the plugin module store's service worker did not take control")), PLUGIN_MODULE_STORE_CONTROL_DEADLINE_MS);
        options.serviceWorker.addEventListener("controllerchange", () => {
          clearTimeout(timer);
          resolve();
        }, { once: true });
      });
    })();
    return readiness;
  };
  /** 🔒️ Keeps one bundle alive for the life of this page: a superseded generation's collection skips a held bundle. */
  const hold = (bundleSha256: string): void => {
    if (held.has(bundleSha256)) return;
    held.add(bundleSha256);
    void options.locks.request(`${PLUGIN_MODULE_STORE_V1.lockPrefix}${bundleSha256}`, { mode: "shared" }, () => new Promise<never>(() => {}));
  };
  const collect = async (cache: PluginModuleCacheV1, generationId: string): Promise<void> => {
    await options.locks.request(PLUGIN_MODULE_STORE_V1.storeLock, { mode: "exclusive" }, () =>
      collectPluginModuleStoreGarbageV1(cache, options.origin, generationId, (record) =>
        options.locks.request(`${PLUGIN_MODULE_STORE_V1.lockPrefix}${record.entry.bundleSha256}`, { mode: "exclusive", ifAvailable: true }, (lock) => lock !== null),
      ),
    );
  };
  const fetchIndex = async (signal: AbortSignal): Promise<TrustedPluginModuleIndexV1> => {
    const bytes = await fetchBytes(`${options.hubMount}${HUB_PLUGIN_MODULE_ROUTE}`, HUB_PLUGIN_MODULE_INDEX_MAX_BYTES, signal);
    return validateTrustedPluginModuleIndexV1(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes)));
  };
  const acquired = (record: PluginModuleStoreRecordV1): PluginModuleAcquired => {
    hold(record.entry.bundleSha256);
    return { moduleUrl: storedPluginModuleUrlV1(record.generationId, record.entry.bundleSha256, record.entry.entry), rebuiltAt: now() };
  };
  /** 📜️ One bundle's manifest from the hub, held to its content address and to its index entry. */
  const fetchManifest = async (entry: TrustedPluginModuleIndexEntryV1, signal: AbortSignal): Promise<Readonly<{ manifestBytes: Uint8Array; bundle: TrustedPluginModuleBundleV1 }>> => {
    const manifestBytes = await fetchBytes(`${options.hubMount}${HUB_PLUGIN_MODULE_ROUTE}/${entry.bundleSha256}`, TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, signal);
    if (manifestBytes.byteLength !== entry.bundleByteLength || (await trustedPluginModuleBundleSha256V1(manifestBytes)) !== entry.bundleSha256) throw new TrustedPluginModuleRefusalV1("trusted plugin module manifest differs from its content address");
    const bundle = decodeTrustedPluginModuleBundleV1(manifestBytes, trustedPluginModuleSourceOfEntryV1(entry));
    if (bundle.entry !== entry.entry) throw new TrustedPluginModuleRefusalV1("trusted plugin module entry differs from its index");
    return { manifestBytes, bundle };
  };
  /** 🔐️ Downloads every file of the bundle the store lacks from the hub, verifying each with byte progress. */
  const downloadNeeded = async (cache: PluginModuleCacheV1, entry: TrustedPluginModuleIndexEntryV1, bundle: TrustedPluginModuleBundleV1, acquisition: PluginModuleAcquisition): Promise<readonly (readonly [TrustedPluginModuleFileV1, Uint8Array])[]> => {
    const needed: TrustedPluginModuleFileV1[] = [];
    for (const file of bundle.files) if (!(await storedPluginModuleBlobPresentV1(cache, options.origin, file.sha256))) needed.push(file);
    const totalBytes = needed.reduce((sum, file) => sum + file.byteLength, 0);
    let completedBytes = 0;
    acquisition.onProgress?.({ completedBytes, totalBytes });
    const downloaded: (readonly [TrustedPluginModuleFileV1, Uint8Array])[] = [];
    for (const file of needed) {
      acquisition.signal.throwIfAborted();
      const bytes = await fetchBytes(hubFileUrl(options.hubMount, entry.bundleSha256, file.path), Math.min(file.byteLength, TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES), acquisition.signal, (chunk) => {
        completedBytes += chunk;
        acquisition.onProgress?.({ completedBytes, totalBytes });
      });
      if (!(await verifyTrustedPluginModuleFileV1(file, bytes))) throw new TrustedPluginModuleRefusalV1(`trusted plugin module file ${file.path} differs from its manifest`);
      downloaded.push([file, bytes]);
    }
    return downloaded;
  };
  /** 🗄️ Commits verified files, then the manifest, then the record, under the store lock; a full store is noticed. */
  const commit = async (cache: PluginModuleCacheV1, generationId: string, entry: TrustedPluginModuleIndexEntryV1, manifestBytes: Uint8Array, files: readonly (readonly [TrustedPluginModuleFileV1, Uint8Array])[], signal: AbortSignal): Promise<PluginModuleStoreRecordV1> => {
    signal.throwIfAborted();
    const record = pluginModuleStoreRecordV1(generationId, entry, now());
    try {
      await options.locks.request(PLUGIN_MODULE_STORE_V1.storeLock, { mode: "exclusive" }, async () => {
        for (const [file, bytes] of files) if (!(await storedPluginModuleBlobPresentV1(cache, options.origin, file.sha256))) await storePluginModuleBlobV1(cache, options.origin, file, bytes);
        await commitPluginModuleBundleV1(cache, options.origin, record, manifestBytes);
      });
    } catch (error) {
      if (error instanceof PluginModuleStoreQuotaError) options.onNotice({ kind: "quota-exceeded", pluginId: entry.pluginId });
      throw error;
    }
    await options.persist?.().catch(() => false);
    return record;
  };
  /** 🔐️ Installs one bundle from the hub: manifest, every file the store lacks, commit. */
  const install = async (cache: PluginModuleCacheV1, generationId: string, entry: TrustedPluginModuleIndexEntryV1, acquisition: PluginModuleAcquisition): Promise<PluginModuleStoreRecordV1> => {
    const { manifestBytes, bundle } = await fetchManifest(entry, acquisition.signal);
    return commit(cache, generationId, entry, manifestBytes, await downloadNeeded(cache, entry, bundle, acquisition), acquisition.signal);
  };
  /** 📂️ Reads one bundle path from a locally staged module, `null` when it is absent, unreadable or longer than the manifest says. */
  const readLocal = (root: string, signal: AbortSignal) => async (file: TrustedPluginModuleFileV1): Promise<Uint8Array | null> => {
    let response: Response;
    try {
      response = await fetch(`${root}${file.path.split("/").map(encodeURIComponent).join("/")}`, { signal, cache: "no-store" });
    } catch {
      signal.throwIfAborted();
      return null;
    }
    if (!response.ok) {
      await response.body?.cancel();
      return null;
    }
    try {
      return await readBounded(response, file.byteLength, () => {});
    } catch {
      signal.throwIfAborted();
      return null;
    }
  };
  /** 🗄️ Whether this device's store holds the generation's bundle with every file. */
  const storedComplete = async (cache: PluginModuleCacheV1, generationId: string, entry: TrustedPluginModuleIndexEntryV1): Promise<Readonly<{ record: PluginModuleStoreRecordV1 | undefined; complete: boolean }>> => {
    const record = (await readPluginModuleStoreRecordsV1(cache, options.origin)).find((candidate) => candidate.generationId === generationId && candidate.entry.bundleSha256 === entry.bundleSha256);
    const stored = record ? await readStoredPluginModuleBundleV1(cache, options.origin, record) : null;
    return { record, complete: record !== undefined && stored !== null && stored.missing.length === 0 };
  };
  const records = async (): Promise<readonly PluginModuleStoreRecordV1[]> => readPluginModuleStoreRecordsV1(await store(), options.origin);
  const index = async (signal: AbortSignal): Promise<TrustedPluginModuleIndexV1 | null> => {
    try {
      return await fetchIndex(signal);
    } catch {
      signal.throwIfAborted();
      return null;
    }
  };
  const newestRecord = (rows: readonly PluginModuleStoreRecordV1[], pluginId: string) => rows.filter((row) => row.entry.pluginId === pluginId).sort((left, right) => right.installedAtMs - left.installedAtMs)[0];
  return {
    id: "hub",
    ready,
    async list(): Promise<readonly PluginRegistryEntry[]> {
      const current = await index(new AbortController().signal);
      const rows = current?.modules.map((entry) => ({ generationId: current.generationId, entry })) ?? (await records()).map((record) => ({ generationId: record.generationId, entry: record.entry }));
      return rows.map(({ generationId, entry }) => ({ pluginId: entry.pluginId, moduleUrl: storedPluginModuleUrlV1(generationId, entry.bundleSha256, entry.entry), dependencies: entry.dependencies.map((pluginId) => ({ pluginId, version: "*" })) }));
    },
    async ownerOfDialect(artifactKind) {
      const current = await index(new AbortController().signal);
      const entries = current?.modules ?? (await records()).map((record) => record.entry);
      return entries.find((entry) => entry.dialectArtifactKinds.includes(artifactKind))?.pluginId;
    },
    catalog: index,
    async installProgram(generationId, entry, localModuleUrl, acquisition): Promise<HubProgramModuleV1> {
      try {
        await ready();
      } catch (error) {
        throw new PluginModuleUnavailableError("hub", entry.pluginId, error instanceof Error ? error.message : String(error));
      }
      const cache = await store();
      const held = await storedComplete(cache, generationId, entry);
      let source: PluginModuleSourceV1 = "store";
      if (!held.complete) {
        const { manifestBytes, bundle } = await fetchManifest(entry, acquisition.signal);
        const root = localModuleUrl === null ? null : localPluginModuleRootV1(localModuleUrl, bundle.entry);
        const totalBytes = bundle.files.reduce((sum, file) => sum + file.byteLength, 0);
        let completedBytes = 0;
        const resolution = await resolvePluginModuleSourceV1({
          bundle,
          storedComplete: false,
          readLocal: root === null ? null : readLocal(root, acquisition.signal),
          signal: acquisition.signal,
          onVerified: (bytes) => {
            completedBytes += bytes;
            acquisition.onProgress?.({ completedBytes, totalBytes });
          },
        });
        if (held.record) options.onNotice({ kind: "reinstalling", pluginId: entry.pluginId });
        source = resolution.source;
        await commit(cache, generationId, entry, manifestBytes, resolution.source === "local" ? resolution.files : await downloadNeeded(cache, entry, bundle, acquisition), acquisition.signal);
      }
      await collect(cache, generationId);
      hold(entry.bundleSha256);
      return Object.freeze({
        programId: hubProgramIdV1(entry.pluginId, entry.bundleSha256),
        pluginId: entry.pluginId,
        generationId,
        bundleSha256: entry.bundleSha256,
        moduleUrl: storedPluginModuleUrlV1(generationId, entry.bundleSha256, entry.entry),
        source,
      });
    },
    async acquireModule(pluginId, _rebuiltAt, acquisition): Promise<PluginModuleAcquired> {
      try {
        await ready();
      } catch (error) {
        throw new PluginModuleUnavailableError("hub", pluginId, error instanceof Error ? error.message : String(error));
      }
      const cache = await store();
      const stored = await readPluginModuleStoreRecordsV1(cache, options.origin);
      const current = await index(acquisition.signal);
      if (current === null) {
        const record = newestRecord(stored, pluginId);
        const complete = record ? await readStoredPluginModuleBundleV1(cache, options.origin, record) : null;
        if (record && complete && complete.missing.length === 0) return acquired(record);
        throw new PluginModuleUnavailableError("hub", pluginId, "the hub index is unreachable and this device holds no complete copy");
      }
      const entry = current.modules.find((candidate) => candidate.pluginId === pluginId);
      if (!entry) throw new PluginModuleUnavailableError("hub", pluginId, `generation ${current.generationId} carries no plugin module for it`);
      const record = stored.find((candidate) => candidate.generationId === current.generationId && candidate.entry.bundleSha256 === entry.bundleSha256);
      const complete = record ? await readStoredPluginModuleBundleV1(cache, options.origin, record) : null;
      if (record && complete && complete.missing.length === 0) {
        await collect(cache, current.generationId);
        return acquired(record);
      }
      if (record) options.onNotice({ kind: "reinstalling", pluginId });
      const installed = await install(cache, current.generationId, entry, acquisition);
      await collect(cache, current.generationId);
      return acquired(installed);
    },
    subscribe() {
      return () => {};
    },
  };
}
