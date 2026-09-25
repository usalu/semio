/** 🗄️ The persisted plugin module store (`🌎️hub-source/🗄️store`) and the hub source on top of it, replayed from the language-agnostic
 * fixture `🌎️hub-source/🧫️fixtures/🗄️store/🔣️.json` against an in-memory Cache Storage: module URLs, every-load re-verification,
 * self-repair of a corrupt entry, collection of superseded generations, and the source's install, reuse, reinstall, quota,
 * offline and hub-only-plugin paths. Ajv is the independent oracle for every record the fixture carries. */
import { describe, expect, it, vi } from "vitest";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { PluginModuleUnavailableError } from "@semio-tech/framework";
import { PLUGIN_MODULE_STORE_V1, validatePluginModuleStoreRecordV1, type PluginModuleStoreRecordV1 } from "../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🟦️.ts";
import {
  collectPluginModuleStoreGarbageV1,
  commitPluginModuleBundleV1,
  parseStoredPluginModuleUrlV1,
  pluginModuleStoreKeysV1,
  PluginModuleStoreQuotaError,
  readPluginModuleStoreRecordsV1,
  serveStoredPluginModuleFileV1,
  storedPluginModuleUrlV1,
  storePluginModuleBlobV1,
  type PluginModuleCacheV1,
} from "../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🗄️store/🟦️.ts";
import { createHubPluginSource, HUB_PLUGIN_MODULE_ROUTE, type HubPluginSourceNoticeV1, type PluginModuleLocksV1 } from "../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🟦️.ts";

const here = (path: string) => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));
const fixture = here("../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧫️fixtures/🗄️store/🔣️.json");
const storeSchema = here("../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🔣️.json");
const hubSchema = here("../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json");
const origin: string = fixture.origin;
type BundleId = "noteA" | "noteB" | "drawB";
const hexBytes = (value: string) => Uint8Array.from(value.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));
const utf8 = (value: string) => new TextEncoder().encode(value);

/** 🧪️ An in-memory Cache Storage cache: exact-URL keys, byte bodies, an optional byte quota. */
class MemoryCache implements PluginModuleCacheV1 {
  readonly entries = new Map<string, Uint8Array>();
  constructor(private readonly quotaBytes = Number.POSITIVE_INFINITY) {}
  async match(request: string) {
    const bytes = this.entries.get(request);
    return bytes === undefined ? undefined : new Response(bytes.slice() as Uint8Array<ArrayBuffer>);
  }
  async put(request: string, response: Response) {
    const bytes = new Uint8Array(await response.arrayBuffer());
    const used = [...this.entries.entries()].reduce((sum, [key, value]) => sum + (key === request ? 0 : value.byteLength), 0);
    if (used + bytes.byteLength > this.quotaBytes) throw new DOMException("quota", "QuotaExceededError");
    this.entries.set(request, bytes);
  }
  async delete(request: string) {
    return this.entries.delete(request);
  }
  async keys() {
    return [...this.entries.keys()].map((url) => ({ url }));
  }
}

const record = (id: BundleId): PluginModuleStoreRecordV1 => validatePluginModuleStoreRecordV1(fixture.records[id]);
const keys = pluginModuleStoreKeysV1(origin);

/** 🧪️ Stores whole bundles the way an install does: files, then manifest, then record. */
async function seed(cache: MemoryCache, ids: readonly BundleId[]) {
  for (const id of ids) {
    const bundle = fixture.bundles[id];
    const manifest = JSON.parse(bundle.manifestUtf8);
    for (const file of manifest.files) await storePluginModuleBlobV1(cache, origin, file, hexBytes(bundle.contents[file.path]));
    await commitPluginModuleBundleV1(cache, origin, record(id), utf8(bundle.manifestUtf8));
  }
}

const recordKey = (id: BundleId) => keys.record(fixture.records[id].generationId, fixture.bundles[id].entry.bundleSha256);

describe("🗄️ plugin module store", () => {
  it("every fixture record satisfies the store schema by Ajv and by the TS twin", () => {
    const ajv = new Ajv({ strict: false, allErrors: true }).addSchema({ ...hubSchema, $ref: undefined }).addSchema(storeSchema);
    const oracle = ajv.getSchema(`${storeSchema.$id}#/$defs/PluginModuleStoreRecordV1`)!;
    for (const id of ["noteA", "noteB", "drawB"] as const) {
      expect(oracle(fixture.records[id]), JSON.stringify(oracle.errors)).toBe(true);
      expect(validatePluginModuleStoreRecordV1(fixture.records[id])).toEqual(fixture.records[id]);
    }
    expect(oracle({ ...fixture.records.noteA, generationId: "A1".repeat(32) })).toBe(false);
    expect(() => validatePluginModuleStoreRecordV1({ ...fixture.records.noteA, generationId: "A1".repeat(32) })).toThrow();
    expect(PLUGIN_MODULE_STORE_V1).toEqual(storeSchema["x-semio-constants"]);
  });

  it("names every stored file by generation, bundle and canonical path, and reads nothing else as one", () => {
    for (const row of fixture.urls) {
      const address = { generationId: row.generationId, bundleSha256: fixture.bundles[row.bundle as BundleId].entry.bundleSha256, path: row.path };
      expect(storedPluginModuleUrlV1(address.generationId, address.bundleSha256, address.path)).toBe(row.url);
      expect(parseStoredPluginModuleUrlV1(row.url)).toEqual(address);
    }
    for (const pathname of fixture.hostilePaths) expect(parseStoredPluginModuleUrlV1(pathname), pathname).toBe(null);
  });

  for (const testCase of fixture.serveCases) {
    it(`serves ${testCase.id}`, async () => {
      const cache = new MemoryCache();
      await seed(cache, testCase.store);
      const bundle = fixture.bundles.noteB;
      const manifest = JSON.parse(bundle.manifestUtf8);
      const [change, target] = String(testCase.change).split(/:(.*)/u);
      if (change === "tamper") cache.entries.set(keys.blob(manifest.files.find((file: any) => file.path === target).sha256), utf8("tampered"));
      if (change === "drop") cache.entries.delete(keys.blob(manifest.files.find((file: any) => file.path === target).sha256));
      if (change === "tamper-manifest") cache.entries.set(keys.manifest(bundle.entry.bundleSha256), utf8(bundle.manifestUtf8.replace("0.1.0", "0.1.1")));
      for (const [index, path] of testCase.paths.entries()) {
        const answer = await serveStoredPluginModuleFileV1(cache, origin, storedPluginModuleUrlV1(testCase.generationId ?? fixture.records.noteB.generationId, bundle.entry.bundleSha256, path));
        expect(answer.status, path).toBe(testCase.status);
        if (answer.status === 200) {
          expect(answer.body).toEqual(hexBytes(bundle.contents[path]));
          if (testCase.contentTypes) expect(answer.contentType).toBe(testCase.contentTypes[index]);
        } else expect(answer.problem).toBe(testCase.problem);
      }
      for (const removal of testCase.remove) {
        const [kind, id, path] = removal.split(":");
        const key = kind === "record" ? recordKey(id as BundleId) : kind === "manifest" ? keys.manifest(fixture.bundles[id as BundleId].entry.bundleSha256) : keys.blob(manifest.files.find((file: any) => file.path === path).sha256);
        expect(cache.entries.has(key), removal).toBe(false);
      }
      if (testCase.remove.length === 0 && testCase.status === 200) expect(cache.entries.has(recordKey("noteB"))).toBe(true);
    });
  }

  for (const testCase of fixture.collectCases) {
    it(`collects ${testCase.id}`, async () => {
      const cache = new MemoryCache();
      await seed(cache, testCase.store);
      const held = new Set(testCase.held.map((id: BundleId) => fixture.bundles[id].entry.bundleSha256));
      const collection = await collectPluginModuleStoreGarbageV1(cache, origin, testCase.current, async (candidate) => !held.has(candidate.entry.bundleSha256));
      expect([...collection.records].sort()).toEqual(testCase.removedRecords.map(recordKey).sort());
      expect((await readPluginModuleStoreRecordsV1(cache, origin)).map((row) => keys.record(row.generationId, row.entry.bundleSha256)).sort()).toEqual(testCase.keptRecords.map(recordKey).sort());
      const live = new Set<string>();
      for (const id of testCase.keptRecords as BundleId[]) for (const file of JSON.parse(fixture.bundles[id].manifestUtf8).files) live.add(keys.blob(file.sha256));
      expect([...cache.entries.keys()].filter((key) => key.startsWith(keys.blobPrefix)).sort()).toEqual([...live].sort());
      for (const id of testCase.keptRecords as BundleId[]) for (const file of JSON.parse(fixture.bundles[id].manifestUtf8).files) {
        const answer = await serveStoredPluginModuleFileV1(cache, origin, storedPluginModuleUrlV1(fixture.records[id].generationId, fixture.bundles[id].entry.bundleSha256, file.path));
        expect(answer.status, `${id} ${file.path}`).toBe(200);
      }
    });
  }

  it("refuses a file that differs from its manifest and names a full store as such", async () => {
    const manifest = JSON.parse(fixture.bundles.noteB.manifestUtf8);
    await expect(storePluginModuleBlobV1(new MemoryCache(), origin, manifest.files[0], utf8("other"))).rejects.toThrow(/differs from its manifest/u);
    await expect(seed(new MemoryCache(16), ["noteB"])).rejects.toBeInstanceOf(PluginModuleStoreQuotaError);
    await expect(commitPluginModuleBundleV1(new MemoryCache(), origin, record("noteB"), utf8(fixture.bundles.noteB.manifestUtf8))).rejects.toThrow(/is not stored/u);
  });
});

describe("🌎️ hub plugin source on the store", () => {
  const hubMount = "/_semio/hub";
  /** 🌐️ A hub serving one generation's index, manifests and files; `offline` refuses the index; `staged` is this device's
   * locally staged module tree under `/plugin-modules/` (a `null` file is absent). */
  const hub = (generationId: string, ids: readonly BundleId[], options: { offline?: boolean; staged?: Readonly<Record<string, string | null>> } = {}) => {
    const downloads: string[] = [];
    const localReads: string[] = [];
    const fetch = async (input: string, init?: RequestInit) => {
      init?.signal?.throwIfAborted();
      const path = decodeURIComponent(new URL(input, origin).pathname);
      if (path.startsWith("/plugin-modules/")) {
        const staged = options.staged?.[path.slice("/plugin-modules/".length)];
        localReads.push(path.slice("/plugin-modules/".length));
        return typeof staged === "string" ? new Response(hexBytes(staged) as Uint8Array<ArrayBuffer>) : new Response(null, { status: 404 });
      }
      if (path === `${hubMount}${HUB_PLUGIN_MODULE_ROUTE}`) {
        if (options.offline) throw new TypeError("fetch failed");
        return new Response(JSON.stringify({ schema: "semio.hub.trusted-plugin-module-index/v1", generationId, modules: ids.map((id) => fixture.bundles[id].entry).sort((a: any, b: any) => (a.pluginId < b.pluginId ? -1 : 1)) }));
      }
      for (const id of ids) {
        const bundle = fixture.bundles[id];
        const prefix = `${hubMount}${HUB_PLUGIN_MODULE_ROUTE}/${bundle.entry.bundleSha256}`;
        if (path === prefix) return new Response(utf8(bundle.manifestUtf8) as Uint8Array<ArrayBuffer>);
        if (path.startsWith(`${prefix}/`) && bundle.contents[path.slice(prefix.length + 1)] !== undefined) {
          downloads.push(path.slice(prefix.length + 1));
          return new Response(hexBytes(bundle.contents[path.slice(prefix.length + 1)]) as Uint8Array<ArrayBuffer>);
        }
      }
      return new Response(null, { status: 404 });
    };
    return { fetch, downloads, localReads };
  };
  /** 🔒️ In-process Web Locks: shared locks held forever, exclusive `ifAvailable` answers `null` while any is held. */
  const memoryLocks = (): PluginModuleLocksV1 & { held: Set<string> } => {
    const held = new Set<string>();
    return {
      held,
      async request(name, options, callback) {
        if (options.mode === "shared") {
          held.add(name);
          return callback({});
        }
        return callback(options.ifAvailable && held.has(name) ? null : {});
      },
    };
  };
  const device = (cache = new MemoryCache()) => ({ cache, notices: [] as HubPluginSourceNoticeV1[] });
  const source = (state: ReturnType<typeof device>, locks = memoryLocks()) =>
    createHubPluginSource({
      hubMount,
      storeWorkerUrl: "/store-worker.js",
      origin,
      onNotice: (notice) => state.notices.push(notice),
      caches: { open: async () => state.cache },
      locks,
      serviceWorker: { register: async () => undefined, ready: Promise.resolve(), controller: {}, addEventListener: () => {} },
      now: () => 1_790_000_000_000,
    });
  const acquire = async (state: ReturnType<typeof device>, served: ReturnType<typeof hub>, pluginId = "note", locks = memoryLocks()) => {
    const progress: { completedBytes: number; totalBytes: number }[] = [];
    vi.stubGlobal("fetch", served.fetch);
    try {
      const acquired = await source(state, locks).acquireModule(pluginId, undefined, { signal: new AbortController().signal, onProgress: (row) => progress.push(row) });
      return { acquired, progress };
    } finally {
      vi.unstubAllGlobals();
    }
  };
  const generationB: string = fixture.generations.b;

  it("installs once into the store with byte progress, then loads from the store and downloads nothing", async () => {
    const state = device();
    const first = hub(generationB, ["noteB", "drawB"]);
    const { acquired, progress } = await acquire(state, first);
    expect(acquired.moduleUrl).toBe(storedPluginModuleUrlV1(generationB, fixture.bundles.noteB.entry.bundleSha256, "🗒️note/🌉️bridge.js"));
    const bytes = Object.values(fixture.bundles.noteB.contents as Record<string, string>).reduce((sum, value) => sum + value.length / 2, 0);
    expect(progress.at(-1)).toEqual({ completedBytes: bytes, totalBytes: bytes });
    expect((await serveStoredPluginModuleFileV1(state.cache, origin, acquired.moduleUrl)).status).toBe(200);
    const again = hub(generationB, ["noteB", "drawB"]);
    await acquire(state, again);
    expect(again.downloads).toEqual([]);
    const draw = hub(generationB, ["noteB", "drawB"]);
    await acquire(state, draw, "draw");
    expect(draw.downloads.some((path) => path.startsWith("🪞️vendor/")), "vendored files are shared by content address").toBe(false);
    expect(state.notices).toEqual([]);
  });

  it("reinstalls a stored bundle that lost a file, with a notice, and installs from this device while the hub is unreachable", async () => {
    const state = device();
    await acquire(state, hub(generationB, ["noteB"]));
    const manifest = JSON.parse(fixture.bundles.noteB.manifestUtf8);
    state.cache.entries.delete(keys.blob(manifest.files.find((file: any) => file.path === "🗒️note/🌉️bridge.js").sha256));
    const repair = hub(generationB, ["noteB"]);
    await acquire(state, repair);
    expect(state.notices).toEqual([{ kind: "reinstalling", pluginId: "note" }]);
    expect(repair.downloads).toEqual(["🗒️note/🌉️bridge.js"]);
    const { acquired } = await acquire(state, hub(generationB, ["noteB"], { offline: true }));
    expect((await serveStoredPluginModuleFileV1(state.cache, origin, acquired.moduleUrl)).status).toBe(200);
    await expect(acquire(device(), hub(generationB, ["noteB"], { offline: true }))).rejects.toBeInstanceOf(PluginModuleUnavailableError);
  });

  it("names a full store with a notice and commits nothing", async () => {
    const state = device(new MemoryCache(64));
    await expect(acquire(state, hub(generationB, ["noteB"]))).rejects.toBeInstanceOf(PluginModuleStoreQuotaError);
    expect(state.notices).toEqual([{ kind: "quota-exceeded", pluginId: "note" }]);
    expect(await readPluginModuleStoreRecordsV1(state.cache, origin)).toEqual([]);
  });

  it("collects a superseded generation once no page runs it, and keeps it while one does", async () => {
    const state = device();
    const runningPage = memoryLocks();
    await acquire(state, hub(fixture.generations.a, ["noteA"]), "note", runningPage);
    await acquire(state, hub(generationB, ["noteB"]), "note", runningPage);
    expect((await readPluginModuleStoreRecordsV1(state.cache, origin)).map((row) => row.generationId).sort()).toEqual([fixture.generations.a, generationB]);
    await acquire(state, hub(generationB, ["noteB"]), "note", memoryLocks());
    expect((await readPluginModuleStoreRecordsV1(state.cache, origin)).map((row) => row.generationId)).toEqual([generationB]);
  });

  const install = async (state: ReturnType<typeof device>, served: ReturnType<typeof hub>, localModuleUrl: string | null) => {
    vi.stubGlobal("fetch", served.fetch);
    try {
      return await source(state).installProgram(generationB, fixture.bundles.noteB.entry, localModuleUrl, { signal: new AbortController().signal });
    } finally {
      vi.unstubAllGlobals();
    }
  };
  const stagedEntry = `/plugin-modules/${encodeURIComponent("🗒️note")}/${encodeURIComponent("🌉️bridge.js")}?v=1`;
  const staged = (edit: (files: Record<string, string | null>) => void = () => {}) => {
    const files: Record<string, string | null> = { ...fixture.bundles.noteB.contents };
    edit(files);
    return files;
  };
  const noteFiles = Object.keys(fixture.bundles.noteB.contents).length;

  it("runs a hub document on the hub's module when this device stages a different one, then on its stored copy", async () => {
    const state = device();
    const stale = hub(generationB, ["noteB"], { staged: staged((files) => { files["🗒️note/🌉️bridge.js"] = "00"; }) });
    const program = await install(state, stale, stagedEntry);
    expect(program).toEqual({
      programId: `note@${fixture.bundles.noteB.entry.bundleSha256}`,
      pluginId: "note",
      generationId: generationB,
      bundleSha256: fixture.bundles.noteB.entry.bundleSha256,
      moduleUrl: storedPluginModuleUrlV1(generationB, fixture.bundles.noteB.entry.bundleSha256, "🗒️note/🌉️bridge.js"),
      source: "hub",
    });
    expect(stale.downloads.length).toBe(noteFiles);
    const served = await serveStoredPluginModuleFileV1(state.cache, origin, program.moduleUrl);
    expect(served.status === 200 ? served.body : null).toEqual(hexBytes(fixture.bundles.noteB.contents["🗒️note/🌉️bridge.js"]));
    const again = hub(generationB, ["noteB"], { staged: staged() });
    expect((await install(state, again, stagedEntry)).source).toBe("store");
    expect([again.downloads, again.localReads]).toEqual([[], []]);
  });

  it("runs a hub document on this device's staged module when every file is the bundle's own, downloading no file", async () => {
    const state = device();
    const matching = hub(generationB, ["noteB"], { staged: staged() });
    const program = await install(state, matching, stagedEntry);
    expect(program.source).toBe("local");
    expect(matching.downloads).toEqual([]);
    expect(matching.localReads.length).toBe(noteFiles);
    for (const path of Object.keys(fixture.bundles.noteB.contents)) expect((await serveStoredPluginModuleFileV1(state.cache, origin, storedPluginModuleUrlV1(generationB, fixture.bundles.noteB.entry.bundleSha256, path))).status, path).toBe(200);
  });

  it("runs a hub document on the hub's module when nothing is staged, or the staged entry is another", async () => {
    const missing = hub(generationB, ["noteB"]);
    expect((await install(device(), missing, null)).source).toBe("hub");
    expect(missing.localReads).toEqual([]);
    const otherEntry = hub(generationB, ["noteB"], { staged: staged() });
    expect((await install(device(), otherEntry, `/plugin-modules/${encodeURIComponent("🗒️note")}/index.js`)).source).toBe("hub");
    expect(otherEntry.localReads).toEqual([]);
    expect(otherEntry.downloads.length).toBe(noteFiles);
  });

  it("finds and lists a hub-only plugin the local build never registered", async () => {
    vi.stubGlobal("fetch", hub(generationB, ["noteB", "drawB"]).fetch);
    try {
      const hubOnly = source(device());
      expect(await hubOnly.ownerOfDialect("s.draw.drawing")).toBe("draw");
      expect(await hubOnly.ownerOfDialect("s.unknown.kind")).toBe(undefined);
      expect((await hubOnly.list()).map((entry) => entry.pluginId)).toEqual(["draw", "note"]);
    } finally {
      vi.unstubAllGlobals();
    }
  });
});
