#!/usr/bin/env bun
/** @emoji 🧭️ `@semio-tech/framework-os-dev` task router — Rust plugin OS dev host. */
import { createWriteStream, copyFileSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, statSync, watch, writeFileSync } from "node:fs";
import { EventEmitter } from "node:events";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  probeWgpuDevPort,
  stopTrunkDevPort,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  cargoProfileDir,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { BACKBONE_ENDPOINT_PATH, BLOB_ENDPOINT_PATH, backboneKindFromUri, decodeDocumentPackBytes, encodeDocumentPackBytes } from "@semio-tech/framework-os";
import type { PluginSourceEvent } from "@semio-tech/framework";
import { generatePluginRegistry, isHostPluginFilter, writePlaygroundSession, type PluginRegistryEntry } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts";
import { DEFAULT_HOST_VARIANT } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts";
import {
  ensurePreview2ShimVendorAt,
  hostShimSource,
  PLUGIN_HOST_SHIM_FILE,
  pluginComponentBridgeSource,
  rewriteJcoComponentAssetUrls,
  SHARD_WORKER_FILE,
  shardWorkerSource,
  rewriteJcoAsyncResultLifting,
  rewritePreview2ShimImportSource,
  rewritePreview2ShimImports,
  transpilePluginComponentAsync,
  type PluginWebMaterializeContext,
} from "../../../🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts";
import { defaultExtensionInstallRoot, EXTENSION_INSTALL_META, EXTENSION_WATCH_MARKER } from "../../../🔌️plugin/🏪️store/📜️store.ts";

type OwnedParityImage = { readonly width: number; readonly height: number; readonly data: Uint8Array };

const repoRoot = getWorkspaceRoot();
const pluginOutRoot = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules");

/** @emoji 🧵️ Publishes the single package-agnostic shard worker at `🔌️plugin-modules/_shard/`, the
 * URL `ShardClient` pool members are constructed from (H2 design). Idempotent: rewritten on every
 * plugin build so a source change to `shardWorkerSource` always reaches the browser. */
function publishShardWorker(): void {
  const shardDir = join(pluginOutRoot, "_shard");
  mkdirSync(shardDir, { recursive: true });
  writeFileSync(join(shardDir, SHARD_WORKER_FILE), shardWorkerSource());
}
const extensionOutRoot = defaultExtensionInstallRoot(repoRoot);
const playgroundSessionPath = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🤖️generated/🟦️session.ts");

const PLUGIN_WASM_TARGET = "wasm32-wasip2";
const PLUGIN_WASM_STACK_BYTES = 8 * 1024 * 1024;

/** @emoji 🎯 Ensures the wasip2 rustc target is installed for plugin component builds. */
function ensureWasmTarget(): void {
  const probe = runProbe("rustup", ["target", "list", "--installed"]);
  if (!probe.stdout.includes(PLUGIN_WASM_TARGET)) {
    runCmd("rustup", ["target", "add", PLUGIN_WASM_TARGET]);
  }
}

/** @emoji 🪶️ Cargo profile for wasip2 plugin components — `dev` in agent loops, `wasm-release` when `SEMIO_BUILD_MODE=ship`. */
function pluginWasmProfile(): string {
  return process.env.SEMIO_PLUGIN_PROFILE ?? (semioBuildMode() === "ship" ? "wasm-release" : "dev");
}

function pluginCargoArgs(packageName: string, profile: string): string[] {
  const args = ["rustc", "-p", packageName, "--target", PLUGIN_WASM_TARGET, "--profile", profile, "--", "-C", `link-arg=-zstack-size=${PLUGIN_WASM_STACK_BYTES}`];
  if (process.env.SEMIO_PLUGIN_SYMBOLS === "1") args.push("-C", "strip=none");
  return args;
}

//#region 🔖️PlaygroundVariantResolution
/** @emoji 📚️ Generated playground catalog (variant -> crate pluginId + optional app id), loaded once for this process via `@semio-tech/repo-lib`'s `loadFrameworkOsPlaygroundCatalog` (backed by `framework/plugin/registry/generated/🟦️playgrounds.ts`). */
const playgroundCatalog = loadFrameworkOsPlaygroundCatalog();

/** @emoji 🧭️ A resolved playground filter: the crate pluginId to build/load, plus the app id and shell brand id to inject when the filter matched a catalog variant row. */
type ResolvedPlaygroundFilter = {
  readonly pluginId: string;
  readonly appId?: string;
  readonly brand?: string;
};

/**
 * 🧭️ Resolves `filterPlugin` (a playground variant id like "puzzle5d", or already a bare crate
 * pluginId like "note") against the generated playground catalog: a matching variant row yields
 * its crate pluginId, app id, and brand id, otherwise `filterPlugin` is treated as already being a
 * bare pluginId (existing behavior for single-app crates where variant === pluginId).
 */
function resolvePlaygroundFilter(filterPlugin: string): ResolvedPlaygroundFilter {
  const row = playgroundCatalog.find((entry) => entry.variant === filterPlugin);
  return row ? { pluginId: row.pluginId, appId: row.app, brand: row.brand } : { pluginId: filterPlugin };
}

/** @emoji 🎯️ Resolves a raw filter to the crate pluginId `generatePluginRegistry`'s `filterPlaygroundPlugin` option expects, or `undefined` for the unfiltered/studio case. */
function resolveCatalogFilterPluginId(filterPlugin?: string): string | undefined {
  return filterPlugin && !isHostPluginFilter(filterPlugin) ? resolvePlaygroundFilter(filterPlugin).pluginId : undefined;
}
//#endregion 🔖️PlaygroundVariantResolution

//#region BackboneVitePlugin
/** Lazily imports `bun:sqlite` — a static top-level import breaks Vite's config bundler, which loads this module's exports under Node before the dev server (and its Bun runtime) exists. */
let backboneDatabaseCtor: typeof import("bun:sqlite").Database | undefined;
async function backboneDatabaseCtorLazy(): Promise<typeof import("bun:sqlite").Database> {
  if (!backboneDatabaseCtor) ({ Database: backboneDatabaseCtor } = await import("bun:sqlite"));
  return backboneDatabaseCtor;
}
type BackboneSqliteHandle = InstanceType<typeof import("bun:sqlite").Database>;

/** @emoji 🗄️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): per-path `bun:sqlite` handle cache.
 * `readBackbonePayload`/`writeBackbonePayload` used to `new Database(dbPath)` — and re-run the
 * (idempotent but non-free) `CREATE TABLE IF NOT EXISTS` — on EVERY single read/write request, so a
 * hot dev-editing loop against one folder-backed document reopened the same file every keystroke's
 * autosave. Lifetime: opened once, then held open for the lifetime of THIS dev-server process — never
 * explicitly closed or evicted. A dev session only ever touches a handful of distinct folder URIs (the
 * open studio, plus maybe one or two app documents), so the cache's total size is bounded by session
 * variety, not by request volume; there is no observed need for a size/idle eviction policy for that few
 * long-lived, cheap-to-hold connections. If that assumption ever stops holding (e.g. a scripted session
 * that iterates many distinct folders), add one then — not speculatively here. */
const backboneDbHandles = new Map<string, BackboneSqliteHandle>();

async function backboneDbHandleFor(dbPath: string): Promise<BackboneSqliteHandle> {
  const existing = backboneDbHandles.get(dbPath);
  if (existing) return existing;
  const Database = await backboneDatabaseCtorLazy();
  const db = new Database(dbPath);
  db.run("CREATE TABLE IF NOT EXISTS document (id TEXT PRIMARY KEY, schema TEXT, pack BLOB NOT NULL, spr BLOB NOT NULL, updated_at INTEGER NOT NULL)");
  backboneDbHandles.set(dbPath, db);
  return db;
}

/** @emoji 🗂️ Same convention as `vcs::FolderSqliteStorage` (`.semio/documents.db`, a `document(id,
 * schema, json, updated_at)` table) so a folder-bound studio opened by the browser dev path and a
 * native (wgpu) reader agree on the same file. `documentId` defaults to the studio's own
 * single-document convention (mirrors os-core's `SPACE_FOLDER_DOCUMENT_ID`) when the caller doesn't
 * pass one — app documents (per `OsDocumentRef`) always pass their own id explicitly. */
const SPACE_FOLDER_DOCUMENT_ID = "studio";

async function readBackbonePayload(uri: string, documentId: string | null): Promise<Uint8Array | null> {
  const kind = backboneKindFromUri(uri);
  if (kind === "file") {
    const path = uri.slice("file://".length);
    if (!existsSync(path)) return null;
    return new Uint8Array(readFileSync(path));
  }
  if (kind === "folder") {
    const folder = uri.slice("folder://".length);
    const dbPath = join(folder, ".semio", "documents.db");
    if (!existsSync(dbPath)) return null;
    const db = await backboneDbHandleFor(dbPath);
    const row = db.query("SELECT pack, spr FROM document WHERE id = ?1").get(documentId ?? SPACE_FOLDER_DOCUMENT_ID) as { pack?: Uint8Array; spr?: Uint8Array } | null;
    if (!row?.pack) return null;
    const pack = row.pack instanceof Uint8Array ? row.pack : new Uint8Array(row.pack as ArrayBuffer);
    const spr = row.spr instanceof Uint8Array ? row.spr : new Uint8Array((row.spr ?? []) as ArrayBuffer);
    return encodeDocumentPackBytes(pack, spr);
  }
  return null;
}

async function writeBackbonePayload(uri: string, documentId: string | null, schema: string | null, payload: Uint8Array): Promise<void> {
  const kind = backboneKindFromUri(uri);
  const { pack, spr } = decodeDocumentPackBytes(payload);
  if (kind === "file") {
    const path = uri.slice("file://".length);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, payload);
    return;
  }
  if (kind === "folder") {
    const folder = uri.slice("folder://".length);
    const dbPath = join(folder, ".semio", "documents.db");
    mkdirSync(dirname(dbPath), { recursive: true });
    const db = await backboneDbHandleFor(dbPath);
    db.run("INSERT INTO document (id, schema, pack, spr, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, pack = excluded.pack, spr = excluded.spr, updated_at = excluded.updated_at", [
      documentId ?? SPACE_FOLDER_DOCUMENT_ID,
      schema ?? "",
      pack,
      spr,
      Date.now(),
    ]);
    return;
  }
  throw new Error(`unsupported backbone uri: ${uri}`);
}

/** 👁️ Per-folder-uri debounced watchers feeding every subscribed SSE response for that uri — one
 * `node:fs.watch` per folder regardless of subscriber count. Mirrors `store_sync`'s native
 * `notify` watcher (200ms debounce) so both the dev-browser and native paths agree on cadence. */
const folderWatchSubscribers = new Map<string, Set<{ write: (chunk: string) => void }>>();
const folderWatchHandles = new Map<string, ReturnType<typeof watch>>();
const FOLDER_WATCH_DEBOUNCE_MS = 200;

function subscribeFolderWatch(uri: string, subscriber: { write: (chunk: string) => void }): () => void {
  if (!folderWatchSubscribers.has(uri)) folderWatchSubscribers.set(uri, new Set());
  const subscribers = folderWatchSubscribers.get(uri)!;
  subscribers.add(subscriber);
  if (!folderWatchHandles.has(uri) && backboneKindFromUri(uri) === "folder") {
    const folder = uri.slice("folder://".length);
    mkdirSync(join(folder, ".semio"), { recursive: true });
    let debounceTimer: ReturnType<typeof setTimeout> | undefined;
    const handle = watch(join(folder, ".semio"), { persistent: false }, () => {
      if (debounceTimer) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => {
        for (const sub of folderWatchSubscribers.get(uri) ?? []) sub.write("data: changed\n\n");
      }, FOLDER_WATCH_DEBOUNCE_MS);
    });
    folderWatchHandles.set(uri, handle);
  }
  return () => {
    subscribers.delete(subscriber);
    if (subscribers.size === 0) {
      folderWatchHandles.get(uri)?.close();
      folderWatchHandles.delete(uri);
      folderWatchSubscribers.delete(uri);
    }
  };
}

type BackboneServerRequest = { method?: string; url?: string; on: (event: string, handler: (chunk?: unknown) => void) => void };
type BackboneServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; write: (chunk: string) => void; end: (body?: string | Uint8Array) => void };

/** @emoji 💓️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): both dev SSE endpoints below previously
 * wrote `: connected\n\n` once on connect and nothing else until a real event fired — a quiet dev
 * session (no file edits, no plugin rebuild) could sit for minutes with nothing crossing the wire, which
 * is exactly the shape a browser or an intermediary dev proxy's idle-connection timeout (commonly in the
 * 30-60s range) silently kills with no client-visible `close`/`error` event, leaving the tab's
 * `EventSource` looking "connected" while actually dead. Periodic `: keepalive\n\n` SSE comments (valid
 * per the SSE spec — a line starting with `:` is ignored by `EventSource` but still resets any
 * intermediary's idle timer) fix that. `req.on("close")` already fires reliably on a real disconnect, so
 * clearing this timer there is the only cleanup needed. */
const SSE_KEEPALIVE_INTERVAL_MS = 15_000;

function startSseKeepalive(res: BackboneServerResponse): () => void {
  const timer = setInterval(() => {
    try {
      res.write(": keepalive\n\n");
    } catch {
      clearInterval(timer);
    }
  }, SSE_KEEPALIVE_INTERVAL_MS);
  return () => clearInterval(timer);
}

/** @emoji 💾️ Vite middleware for browser file/folder backbone IO: `GET|PUT ${BACKBONE_ENDPOINT_PATH}?uri=&documentId=&schema=`
 * for read/write, plus `GET ${BACKBONE_ENDPOINT_PATH}/watch?uri=` (SSE) for external-edit notification —
 * `🟦️backbone-🟦️worker.ts`'s folder transport degrades to polling if this endpoint isn't reachable. */
export function semioBackboneVitePlugin() {
  return {
    name: "semio-backbone",
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(BACKBONE_ENDPOINT_PATH)) return next();
        const requestUrl = new URL(req.url, "http://127.0.0.1");
        const uri = requestUrl.searchParams.get("uri");
        if (!uri) {
          res.statusCode = 400;
          res.end("missing uri");
          return;
        }
        if (requestUrl.pathname === `${BACKBONE_ENDPOINT_PATH}/watch`) {
          if (req.method !== "GET") {
            res.statusCode = 405;
            res.end("method not allowed");
            return;
          }
          res.statusCode = 200;
          res.setHeader("content-type", "text/event-stream");
          res.setHeader("cache-control", "no-cache");
          res.setHeader("connection", "keep-alive");
          res.write(": connected\n\n");
          const stopKeepalive = startSseKeepalive(res);
          const unsubscribe = subscribeFolderWatch(uri, res);
          req.on("close", () => {
            stopKeepalive();
            unsubscribe();
          });
          return;
        }
        const documentId = requestUrl.searchParams.get("documentId");
        const schema = requestUrl.searchParams.get("schema");
        if (req.method === "GET") {
          readBackbonePayload(uri, documentId)
            .then((payload) => {
              if (payload == null) {
                res.statusCode = 404;
                res.end("");
                return;
              }
              res.statusCode = 200;
              res.setHeader("content-type", "application/octet-stream");
              res.end(Buffer.from(payload));
            })
            .catch((error) => {
              res.statusCode = 500;
              res.end(String(error));
            });
          return;
        }
        if (req.method === "PUT") {
          const chunks: Buffer[] = [];
          req.on("data", (chunk) => {
            chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(String(chunk)));
          });
          req.on("end", () => {
            const body = Buffer.concat(chunks);
            writeBackbonePayload(uri, documentId, schema, new Uint8Array(body))
              .then(() => {
                res.statusCode = 200;
                res.setHeader("content-type", "application/octet-stream");
                res.end(new Uint8Array());
              })
              .catch((error) => {
                res.statusCode = 500;
                res.end(String(error));
              });
          });
          return;
        }
        res.statusCode = 405;
        res.end("method not allowed");
      });
    },
  };
}
//#endregion BackboneVitePlugin

//#region 🔌️PluginHotSwapVitePlugin
type PluginHotSwapMarker = { readonly pluginId: string; readonly rebuiltAt: number };

/** @emoji 🔌️ Every plugin dir under `root` (default `plugin-modules/`) that has a completed build right
 * now (a `.core*.wasm` present — same convention `collectPluginWasmSizeRows` walks), newest core-wasm
 * mtime as `rebuiltAt`. Backs the SSE endpoint's connect-time `snapshot` event: a browser that connects
 * (or reconnects) after some builds already finished must still learn about them — `.hot-swap` alone
 * only ever holds the single most recent build, not the full history. `root` is overridable so this can
 * be exercised against a throwaway temp dir in-source below rather than the real (build-dependent, so
 * flaky) `plugin-modules/` tree. */
function scanBuiltPluginModules(root: string = pluginOutRoot): readonly PluginHotSwapMarker[] {
  if (!existsSync(root)) return [];
  const rows: PluginHotSwapMarker[] = [];
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name.startsWith("_")) continue;
    const pluginDir = join(root, entry.name);
    let newestMs = 0;
    for (const file of readdirSync(pluginDir)) {
      if (!/\.core\d*\.wasm$/.test(file)) continue;
      newestMs = Math.max(newestMs, statSync(join(pluginDir, file)).mtimeMs);
    }
    if (newestMs > 0) rows.push({ pluginId: entry.name, rebuiltAt: Math.round(newestMs) });
  }
  return rows;
}

/** @emoji 🔌️ Mirrors `@semio-tech/framework`'s `PLUGIN_SOURCE_WATCH_PATH` — kept as a literal here
 * rather than a real (non-`type`) import: `⚙️vite.config.ts` loads this module's exports through Vite's
 * own config loader, which runs under Node's native strip-only TypeScript support rather than esbuild;
 * a genuine runtime import of framework-core's single-file `📦️index.ts` forces Node to parse the WHOLE
 * file, including unrelated parameter-property constructors that strip-only mode rejects
 * (`SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]`) — confirmed by reproducing it locally. `import
 * type` stays safe (fully erased, no runtime import), so `PluginSourceEvent` below is unaffected. */
const PLUGIN_SOURCE_WATCH_PATH = "/plugin-modules/watch";

/** @emoji 🔌️ Vite middleware backing the shell's `createDevPluginSource` (`@semio-tech/framework`):
 * SSE at `PLUGIN_SOURCE_WATCH_PATH`, mirroring `semioBackboneVitePlugin`'s `/watch` endpoint. Sends one
 * `snapshot` on connect ({@link scanBuiltPluginModules}), then a `built` event every time `buildPlugin`
 * overwrites the shared `.hot-swap` marker — `buildPlugin` writes it last, after every other output
 * file, so by the time this fires the plugin's module is actually fetchable. Debounced the same 200ms
 * as `subscribeFolderWatch` above (a burst of writes during one build collapses to a single event). One
 * `fs.watch` on `plugin-modules/` for the whole dev server's lifetime — unlike the backbone plugin's
 * per-uri watchers, there is exactly one watch target here, so it is never torn down. */
export function semioPluginHotSwapVitePlugin() {
  return {
    name: "semio-plugin-hot-swap",
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void } }) {
      const subscribers = new Set<BackboneServerResponse>();
      mkdirSync(pluginOutRoot, { recursive: true });
      const hotSwapMarker = join(pluginOutRoot, ".hot-swap");
      let debounceTimer: ReturnType<typeof setTimeout> | undefined;
      watch(pluginOutRoot, (_eventType, filename) => {
        if (filename !== ".hot-swap") return;
        if (debounceTimer) clearTimeout(debounceTimer);
        debounceTimer = setTimeout(() => {
          if (!existsSync(hotSwapMarker)) return;
          let marker: PluginHotSwapMarker;
          try {
            marker = JSON.parse(readFileSync(hotSwapMarker, "utf8")) as PluginHotSwapMarker;
          } catch {
            return;
          }
          const event: PluginSourceEvent = { kind: "built", pluginId: marker.pluginId, rebuiltAt: marker.rebuiltAt };
          const payload = `data: ${JSON.stringify(event)}\n\n`;
          for (const sub of subscribers) sub.write(payload);
        }, FOLDER_WATCH_DEBOUNCE_MS);
      });
      server.middlewares.use((req, res, next) => {
        if (req.url !== PLUGIN_SOURCE_WATCH_PATH || req.method !== "GET") return next();
        res.statusCode = 200;
        res.setHeader("content-type", "text/event-stream");
        res.setHeader("cache-control", "no-cache");
        res.setHeader("connection", "keep-alive");
        res.write(": connected\n\n");
        const snapshot: PluginSourceEvent = { kind: "snapshot", plugins: scanBuiltPluginModules() };
        res.write(`data: ${JSON.stringify(snapshot)}\n\n`);
        subscribers.add(res);
        const stopKeepalive = startSseKeepalive(res);
        req.on("close", () => {
          stopKeepalive();
          subscribers.delete(res);
        });
      });
    },
  };
}
//#endregion 🔌️PluginHotSwapVitePlugin

//#region 🔖️Blake3
/** 🧬️ Self-contained BLAKE3 (default 32-byte hash mode, no key/context) so the dev-only blob endpoint
 * hashes bytes exactly like `framework/hash/rs`'s `hash_bytes` (Rust `blake3` crate) does natively —
 * content-addressing must agree across the browser dev path and the native/hub `BlobStore`s. No npm
 * `blake3` package exists to lean on (native bindings, and this shared workspace's `bun add` currently
 * can't resolve at all — an unrelated in-progress workspace member), and Bun's built-in `CryptoHasher`
 * doesn't support the algorithm, so this ports the reference tree-hash construction directly from the
 * BLAKE3 spec (https://github.com/BLAKE3-team/BLAKE3/blob/master/reference_impl/reference_impl.rs).
 * Verified byte-for-byte against `semio-framework-hash::hash_bytes` across chunk-boundary inputs
 * (0/1/1023/1024/1025/2048/2049/100000 bytes) before landing — see ticket scratch notes. */
const BLAKE3_IV = new Uint32Array([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);
const BLAKE3_MSG_PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
const BLAKE3_CHUNK_START = 1;
const BLAKE3_CHUNK_END = 2;
const BLAKE3_PARENT = 4;
const BLAKE3_ROOT = 8;
const BLAKE3_CHUNK_LEN = 1024;
const BLAKE3_BLOCK_LEN = 64;
const BLAKE3_OUT_LEN = 32;

function blake3Rotr(x: number, n: number): number {
  return ((x >>> n) | (x << (32 - n))) >>> 0;
}

function blake3G(state: Uint32Array, a: number, b: number, c: number, d: number, mx: number, my: number): void {
  state[a] = (state[a]! + state[b]! + mx) >>> 0;
  state[d] = blake3Rotr(state[d]! ^ state[a]!, 16);
  state[c] = (state[c]! + state[d]!) >>> 0;
  state[b] = blake3Rotr(state[b]! ^ state[c]!, 12);
  state[a] = (state[a]! + state[b]! + my) >>> 0;
  state[d] = blake3Rotr(state[d]! ^ state[a]!, 8);
  state[c] = (state[c]! + state[d]!) >>> 0;
  state[b] = blake3Rotr(state[b]! ^ state[c]!, 7);
}

function blake3RoundFn(state: Uint32Array, m: Uint32Array): void {
  blake3G(state, 0, 4, 8, 12, m[0]!, m[1]!);
  blake3G(state, 1, 5, 9, 13, m[2]!, m[3]!);
  blake3G(state, 2, 6, 10, 14, m[4]!, m[5]!);
  blake3G(state, 3, 7, 11, 15, m[6]!, m[7]!);
  blake3G(state, 0, 5, 10, 15, m[8]!, m[9]!);
  blake3G(state, 1, 6, 11, 12, m[10]!, m[11]!);
  blake3G(state, 2, 7, 8, 13, m[12]!, m[13]!);
  blake3G(state, 3, 4, 9, 14, m[14]!, m[15]!);
}

function blake3Permute(m: Uint32Array): Uint32Array {
  const out = new Uint32Array(16);
  for (let i = 0; i < 16; i++) out[i] = m[BLAKE3_MSG_PERMUTATION[i]!]!;
  return out;
}

function blake3Compress(cv: Uint32Array, block: Uint32Array, counter: number, blockLen: number, flags: number): Uint32Array {
  const state = new Uint32Array(16);
  state.set(cv, 0);
  state.set(BLAKE3_IV.subarray(0, 4), 8);
  state[12] = counter >>> 0;
  state[13] = Math.floor(counter / 2 ** 32) >>> 0;
  state[14] = blockLen;
  state[15] = flags;
  let m = block;
  for (let r = 0; r < 7; r++) {
    blake3RoundFn(state, m);
    if (r < 6) m = blake3Permute(m);
  }
  for (let i = 0; i < 8; i++) {
    state[i] = (state[i]! ^ state[i + 8]!) >>> 0;
    state[i + 8] = (state[i + 8]! ^ cv[i]!) >>> 0;
  }
  return state;
}

function blake3WordsFromBytes(bytes: Uint8Array, offset: number): Uint32Array {
  const words = new Uint32Array(16);
  for (let i = 0; i < 16; i++) {
    const o = offset + i * 4;
    words[i] = (bytes[o]! | (bytes[o + 1]! << 8) | (bytes[o + 2]! << 16) | (bytes[o + 3]! << 24)) >>> 0;
  }
  return words;
}

type Blake3ChunkOutput = { inputCv: Uint32Array; block: Uint32Array; counter: number; blockLen: number; flags: number };

function blake3OutputChainingValue(output: Blake3ChunkOutput): Uint32Array {
  return blake3Compress(output.inputCv, output.block, output.counter, output.blockLen, output.flags).subarray(0, 8);
}

function blake3RootOutputBytes(output: Blake3ChunkOutput, outLen: number): Uint8Array {
  const out = new Uint8Array(outLen);
  let outputBlockCounter = 0;
  let written = 0;
  while (written < outLen) {
    const words = blake3Compress(output.inputCv, output.block, outputBlockCounter, output.blockLen, output.flags | BLAKE3_ROOT);
    for (let i = 0; i < 16 && written < outLen; i++) {
      const w = words[i]!;
      out[written++] = w & 0xff;
      if (written < outLen) out[written++] = (w >>> 8) & 0xff;
      if (written < outLen) out[written++] = (w >>> 16) & 0xff;
      if (written < outLen) out[written++] = (w >>> 24) & 0xff;
    }
    outputBlockCounter++;
  }
  return out;
}

class Blake3ChunkState {
  cv: Uint32Array;
  chunkCounter: number;
  block = new Uint8Array(BLAKE3_BLOCK_LEN);
  blockLen = 0;
  blocksCompressed = 0;
  flags: number;

  constructor(key: Uint32Array, chunkCounter: number, flags: number) {
    this.cv = key.slice();
    this.chunkCounter = chunkCounter;
    this.flags = flags;
  }

  len(): number {
    return this.blocksCompressed * BLAKE3_BLOCK_LEN + this.blockLen;
  }

  startFlag(): number {
    return this.blocksCompressed === 0 ? BLAKE3_CHUNK_START : 0;
  }

  update(input: Uint8Array): void {
    let offset = 0;
    while (offset < input.length) {
      if (this.blockLen === BLAKE3_BLOCK_LEN) {
        const words = blake3WordsFromBytes(this.block, 0);
        this.cv = blake3Compress(this.cv, words, this.chunkCounter, BLAKE3_BLOCK_LEN, this.flags | this.startFlag()).subarray(0, 8);
        this.blocksCompressed++;
        this.block = new Uint8Array(BLAKE3_BLOCK_LEN);
        this.blockLen = 0;
      }
      const take = Math.min(BLAKE3_BLOCK_LEN - this.blockLen, input.length - offset);
      this.block.set(input.subarray(offset, offset + take), this.blockLen);
      this.blockLen += take;
      offset += take;
    }
  }

  output(): Blake3ChunkOutput {
    const words = blake3WordsFromBytes(this.block, 0);
    return { inputCv: this.cv, block: words, counter: this.chunkCounter, blockLen: this.blockLen, flags: this.flags | this.startFlag() | BLAKE3_CHUNK_END };
  }
}

function blake3ParentOutput(leftCv: Uint32Array, rightCv: Uint32Array, key: Uint32Array, flags: number): Blake3ChunkOutput {
  const block = new Uint32Array(16);
  block.set(leftCv, 0);
  block.set(rightCv, 8);
  return { inputCv: key, block, counter: 0, blockLen: BLAKE3_BLOCK_LEN, flags: flags | BLAKE3_PARENT };
}

function blake3ParentCv(leftCv: Uint32Array, rightCv: Uint32Array, key: Uint32Array, flags: number): Uint32Array {
  return blake3OutputChainingValue(blake3ParentOutput(leftCv, rightCv, key, flags));
}

/** 🧮️ Streaming hasher: chunk (1024B) → 16 blocks (64B) chained, chunks merged pairwise into a binary
 * Merkle tree via a "trailing-zero-bits" stack (a Merkle-mountain-range), root-finalized on `digest`. */
class Blake3Hasher {
  private key = BLAKE3_IV;
  private chunkState = new Blake3ChunkState(BLAKE3_IV, 0, 0);
  private cvStack: Uint32Array[] = [];
  private flags = 0;

  private addChunkChainingValue(newCvIn: Uint32Array, totalChunksIn: number): void {
    let newCv = newCvIn;
    let totalChunks = totalChunksIn;
    while ((totalChunks & 1) === 0) {
      const left = this.cvStack.pop()!;
      newCv = blake3ParentCv(left, newCv, this.key, this.flags);
      totalChunks >>>= 1;
    }
    this.cvStack.push(newCv);
  }

  update(input: Uint8Array): void {
    let offset = 0;
    while (offset < input.length) {
      if (this.chunkState.len() === BLAKE3_CHUNK_LEN) {
        const chunkCv = blake3OutputChainingValue(this.chunkState.output());
        const totalChunks = this.chunkState.chunkCounter + 1;
        this.addChunkChainingValue(chunkCv, totalChunks);
        this.chunkState = new Blake3ChunkState(this.key, totalChunks, this.flags);
      }
      const take = Math.min(BLAKE3_CHUNK_LEN - this.chunkState.len(), input.length - offset);
      this.chunkState.update(input.subarray(offset, offset + take));
      offset += take;
    }
  }

  digest(outLen = BLAKE3_OUT_LEN): Uint8Array {
    let output = this.chunkState.output();
    let parentNodesRemaining = this.cvStack.length;
    while (parentNodesRemaining > 0) {
      parentNodesRemaining--;
      output = blake3ParentOutput(this.cvStack[parentNodesRemaining]!, blake3OutputChainingValue(output), this.key, this.flags);
    }
    return blake3RootOutputBytes(output, outLen);
  }
}

/** 🔗️ Hex-encoded BLAKE3 hash of `bytes`, matching `semio_framework_hash::hash_bytes`'s output format. */
function blake3Hex(bytes: Uint8Array): string {
  const hasher = new Blake3Hasher();
  hasher.update(bytes);
  return Buffer.from(hasher.digest()).toString("hex");
}
//#endregion 🔖️Blake3

//#region BlobVitePlugin
let blobDatabaseSingleton: InstanceType<typeof import("bun:sqlite").Database> | undefined;

/** 🗄️ Lazily opens the dev-session-wide content-addressed blob store at `<repoRoot>/.🧬semio/🔗space/blobs.db` —
 * unlike backbone documents, blobs aren't scoped to a per-uri folder (there's no folder in the
 * `write-blob`/`read-blob` WIT signature), so this is one shared table for the whole dev server. */
async function blobDatabase(): Promise<InstanceType<typeof import("bun:sqlite").Database>> {
  if (!blobDatabaseSingleton) {
    const Database = await backboneDatabaseCtorLazy();
    const dbPath = join(repoRoot, ".🧬semio", "🔗space", "blobs.db");
    mkdirSync(dirname(dbPath), { recursive: true });
    blobDatabaseSingleton = new Database(dbPath);
    blobDatabaseSingleton.run("CREATE TABLE IF NOT EXISTS blob (hash TEXT PRIMARY KEY, media_type TEXT NOT NULL, size INTEGER NOT NULL, bytes BLOB NOT NULL)");
  }
  return blobDatabaseSingleton;
}

type BlobServerRequest = { method?: string; url?: string; on: (event: string, handler: (chunk?: unknown) => void) => void };
type BlobServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; end: (body?: string | Buffer) => void };

/** @emoji 📦️ Vite middleware for the dev-only content-addressed blob store: `PUT ${BLOB_ENDPOINT_PATH}?mediaType=`
 * (raw bytes body, BLAKE3-hashed above, returns `{"hash":...}`, idempotent via `INSERT OR IGNORE`) and
 * `GET ${BLOB_ENDPOINT_PATH}/:hash` (raw bytes response, 404 if absent). The browser host-shim's
 * `writeBlob`/`readBlob` (see `hostShimSource`) and `🟦️backbone-🟦️worker.ts`'s IndexedDB cache both talk to
 * this. Mirrors `vcs::FolderSqliteStorage`'s `blobs(hash, media_type, size, bytes)` table/shape. */
export function semioBlobVitePlugin() {
  return {
    name: "semio-blob",
    configureServer(server: { middlewares: { use: (handler: (req: BlobServerRequest, res: BlobServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(BLOB_ENDPOINT_PATH)) return next();
        const requestUrl = new URL(req.url, "http://127.0.0.1");
        if (req.method === "PUT") {
          const chunks: Buffer[] = [];
          req.on("data", (chunk) => {
            chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk as ArrayBuffer));
          });
          req.on("end", () => {
            void (async () => {
              const bytes = Buffer.concat(chunks);
              const mediaType = requestUrl.searchParams.get("mediaType") ?? "application/octet-stream";
              const hash = blake3Hex(new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength));
              const db = await blobDatabase();
              db.run("INSERT OR IGNORE INTO blob (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)", [hash, mediaType, bytes.length, bytes]);
              res.statusCode = 200;
              res.setHeader("content-type", "application/json");
              res.end(JSON.stringify({ hash }));
            })().catch((error) => {
              res.statusCode = 500;
              res.end(String(error));
            });
          });
          return;
        }
        if (req.method === "GET") {
          const hash = requestUrl.pathname.slice(`${BLOB_ENDPOINT_PATH}/`.length);
          if (!hash) {
            res.statusCode = 400;
            res.end("missing hash");
            return;
          }
          void (async () => {
            const db = await blobDatabase();
            const row = db.query("SELECT media_type, bytes FROM blob WHERE hash = ?1").get(hash) as { media_type?: string; bytes?: Uint8Array } | null;
            if (!row) {
              res.statusCode = 404;
              res.end("");
              return;
            }
            res.statusCode = 200;
            res.setHeader("content-type", row.media_type ?? "application/octet-stream");
            res.end(Buffer.from(row.bytes ?? new Uint8Array()));
          })().catch((error) => {
            res.statusCode = 500;
            res.end(String(error));
          });
          return;
        }
        res.statusCode = 405;
        res.end("method not allowed");
      });
    },
  };
}
//#endregion BlobVitePlugin

function preview2ShimVendorDir(): string {
  return join(pluginOutRoot, "_vendor/@bytecodealliance/preview2-shim");
}

function pluginWebMaterializeContext(): PluginWebMaterializeContext {
  return { repoRoot, preview2VendorDir: preview2ShimVendorDir() };
}

function ensurePreview2ShimVendor(): void {
  ensurePreview2ShimVendorAt(preview2ShimVendorDir(), repoRoot);
}

/** @emoji 🫙 Ensures `_vendor/guestslim-typst-fonts.bin` exists for plugin workers' typst text path. */
function ensureGuestSlimTypstFontsAsset(): void {
  const out = join(pluginOutRoot, "_vendor/guestslim-typst-fonts.bin");
  if (existsSync(out) && statSync(out).size > 0) return;
  mkdirSync(dirname(out), { recursive: true });
  const status = runCmdStatus("cargo", ["run", "-p", "semio-framework-os-infinite", "--bin", "dump-guestslim-typst-fonts", "--features", "render", "--", out], { cwd: repoRoot, budgetMs: buildBudgetMs() });
  if (status !== 0 || !existsSync(out)) {
    throw new Error(`guestslim typst fonts asset missing and dump-guestslim-typst-fonts failed (expected ${out})`);
  }
}

/** @emoji 🔚️ Rewrites bare `@bytecodealliance/preview2-shim/*` imports in already-staged plugin JS. */
function rewriteExistingPluginShimImports(): void {
  if (!existsSync(pluginOutRoot)) return;
  const vendor = preview2ShimVendorDir();
  for (const entry of readdirSync(pluginOutRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name.startsWith("_")) continue;
    const pluginDir = join(pluginOutRoot, entry.name);
    for (const file of readdirSync(pluginDir)) {
      if (!file.endsWith(".js")) continue;
      rewritePreview2ShimImports(join(pluginDir, file), vendor);
    }
  }
}

async function readPackageName(cratePath: string): Promise<string> {
  const content = await Bun.file(join(repoRoot, cratePath, "Cargo.toml")).text();
  const match = content.match(/^name = "([^"]+)"/m);
  if (!match) throw new Error(`missing package name in ${cratePath}/Cargo.toml`);
  return match[1]!;
}

/** @emoji 🧹 Drops renamed/orphaned bridge artifacts in a plugin out dir before rewriting current outputs. */
function cleanStalePluginOutputs(outDir: string, jsBase: string, componentBase: string): void {
  if (!existsSync(outDir)) return;
  const keepFiles = new Set(["🟨️host-shim.js"]);
  const currentBridgeFile = `${jsBase}.js`;
  for (const entry of readdirSync(outDir, { withFileTypes: true })) {
    if (entry.isDirectory()) continue;
    if (keepFiles.has(entry.name)) continue;
    if (entry.name === currentBridgeFile) continue;
    if (entry.name.startsWith(`${componentBase}.`)) continue;
    rmSync(join(outDir, entry.name), { force: true });
  }
}

/** @emoji 🛂️ Publishes the checked-in build-time descriptor beside the generated browser module.
 * `fetchDescriptorManifest()` deliberately reads this sibling before any actor is instantiated, so
 * leaving descriptors only at their owner roots makes every otherwise-valid module appear app-less
 * at runtime. Unmigrated crates remain honest: no source descriptor means no staged descriptor. */
function stagePluginDescriptor(target: PluginRegistryEntry, outDir: string, root: string = repoRoot): boolean {
  const ownerRoot = join(root, target.cratePath, "..", "..");
  const descriptorJson = join(ownerRoot, "🔣️descriptor.json");
  if (!existsSync(descriptorJson)) {
    rmSync(join(outDir, "🔣️descriptor.json"), { force: true });
    rmSync(join(outDir, "🛂️descriptor.semio"), { force: true });
    return false;
  }
  copyFileSync(descriptorJson, join(outDir, "🔣️descriptor.json"));
  const descriptorPack = join(ownerRoot, "🛂️descriptor.semio");
  if (existsSync(descriptorPack)) copyFileSync(descriptorPack, join(outDir, "🛂️descriptor.semio"));
  else rmSync(join(outDir, "🛂️descriptor.semio"), { force: true });
  return true;
}

/** @emoji 🔁️ Refreshes descriptor siblings for already-materialized modules on zero-build starts. */
function syncBuiltPluginDescriptors(entries: readonly PluginRegistryEntry[]): void {
  for (const target of entries) {
    const outDir = join(pluginOutRoot, target.pluginId);
    if (existsSync(outDir)) stagePluginDescriptor(target, outDir);
  }
}

/** @emoji 🧹️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): sweeps generated extension-module output
 * (`defaultExtensionInstallRoot()`, entirely gitignored — `.gitignore:90`) left over from BEFORE the
 * `world actor` ABI flip. The SOURCE is already correct — `webMaterialize` (`🏪️store/📜️store.ts`) and
 * `publishBuiltExtension` below both write the CURRENT `hostShimSource()` (only `log`/`nowMs`/
 * `traceSpan`) on every successful materialize — the problem is purely stale disk state from
 * extension crates whose `cargo build` now fails under the new ABI (most haven't migrated their WIT
 * world yet): their old, pre-flip output was written by a materializer version that no longer exists,
 * and a failing rebuild never reaches the overwrite step to replace it, so it sits there and keeps
 * being served. Two invariants make it safe to delete WITHOUT nuking a currently-valid install:
 *   1. `🟨️plugin-worker.js` (the pre-H2 one-worker-per-plugin bootstrap) is never written by ANY
 *      current code path (H2 replaced it with the shared `_shard/` worker) — its mere presence on disk
 *      proves it is stale, unconditionally.
 *   2. `🟨️host-shim.js` IS still written on every successful materialize, so staleness there is
 *      decided by content, not existence: byte-compare against the current `hostShimSource()` and
 *      delete on mismatch — a future successful build rewrites it, same as any other cache miss.
 * Deliberately narrow: does not delete the whole extension directory (its compiled `.wasm`/`.js` may
 * still be old-ABI too, but that is a bigger "should an unbuildable extension's install be evicted
 * outright" product call, flagged as a further finding rather than decided here). Runs once per
 * `preparePluginBuildTargets` call (dev boot + full catalog rebuild), not on every incremental
 * hot-swap. */
function sweepStaleExtensionModuleOutputs(root: string = extensionOutRoot): void {
  if (!existsSync(root)) return;
  const currentHostShim = hostShimSource();
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    const dir = join(root, entry.name);
    const staleWorkerPath = join(dir, "🟨️plugin-worker.js");
    if (existsSync(staleWorkerPath)) rmSync(staleWorkerPath, { force: true });
    const shimPath = join(dir, PLUGIN_HOST_SHIM_FILE);
    if (existsSync(shimPath)) {
      let shimContent: string | undefined;
      try {
        shimContent = readFileSync(shimPath, "utf8");
      } catch {
        shimContent = undefined;
      }
      if (shimContent !== currentHostShim) rmSync(shimPath, { force: true });
    }
  }
}

/** @emoji 🧩️ Mirrors a just-built extension crate from `plugin-modules/` into the runtime `/extensions` install root so catalog loads resolve without a separate `.sxt` install step. */
function publishBuiltExtension(target: PluginRegistryEntry, builtOutDir: string): void {
  if (target.role !== "extension") return;
  if (!existsSync(builtOutDir)) return;
  mkdirSync(extensionOutRoot, { recursive: true });
  const outDir = join(extensionOutRoot, target.pluginId);
  const stagingDir = join(extensionOutRoot, `.staging-${target.pluginId}-${Date.now()}`);
  const retiredDir = join(extensionOutRoot, `.retired-${target.pluginId}-${Date.now()}`);
  cpSync(builtOutDir, stagingDir, { recursive: true });
  for (const file of readdirSync(stagingDir)) {
    if (!file.endsWith(".js")) continue;
    const filePath = join(stagingDir, file);
    const source = readFileSync(filePath, "utf8");
    const rewritten = rewritePreview2ShimImportSource(source, "../../plugin-modules/_vendor/@bytecodealliance/preview2-shim/");
    if (rewritten !== source) writeFileSync(filePath, rewritten);
  }
  const jsBase = target.wasmOut.replace(/\.wasm$/, "");
  const moduleUrl = `/extensions/${target.pluginId}/${jsBase}.js`;
  const installedAt = Date.now();
  const record = {
    extensionId: target.pluginId,
    version: "0.0.0-dev",
    label: target.pluginId,
    extends: target.extends ?? "",
    moduleUrl,
    packageHash: `dev:${installedAt}`,
    installedAt,
  };
  writeFileSync(join(stagingDir, EXTENSION_INSTALL_META), `${JSON.stringify(record, null, 2)}\n`);
  if (existsSync(outDir)) renameSync(outDir, retiredDir);
  renameSync(stagingDir, outDir);
  if (existsSync(retiredDir)) rmSync(retiredDir, { recursive: true, force: true });
  writeFileSync(join(extensionOutRoot, EXTENSION_WATCH_MARKER), `${JSON.stringify({ kind: "installed", extensionId: target.pluginId, version: record.version, installedAt, emittedAt: Date.now() })}\n`);
  console.log(`published extension ${target.pluginId} -> ${moduleUrl}`);
}

/** @emoji 🧩️ Seeds `/extensions` from any extension crates already present under `plugin-modules/` (covers restart without rebuild). */
export function syncBuiltExtensionsToInstallRoot(entries: readonly PluginRegistryEntry[]): void {
  for (const target of entries) {
    if (target.role !== "extension") continue;
    const builtOutDir = join(pluginOutRoot, target.pluginId);
    const jsBase = target.wasmOut.replace(/\.wasm$/, "");
    if (!existsSync(join(builtOutDir, `${jsBase}.js`))) continue;
    publishBuiltExtension(target, builtOutDir);
  }
}

/** @emoji 🛂️ `describe`s one just-built plugin/extension component into `<owner>/` — the plugin/
 * extension OWNER ROOT, sibling of the tracked `🛂️manifest.json` (`📓️design-abi.md` §3, path
 * corrected by D0-descriptor-plumbing per the registrar ruling in `📌️important.md`: NOT
 * `🤖️generated/`, which is globally gitignored and would mean a "checked-in" descriptor could never
 * survive a commit) — best-effort, non-fatal: most plugin crates have not migrated to the new
 * `describe` WIT export yet (W3's `M0`…`M8`/D-packets migrate them one at a time), so every call still
 * fails today against the old-ABI wasm most crates build; failing the whole plugin build over that
 * would regress `dev`/`build` for the entire fleet. `📇️registry:check`'s own descriptor gate is what
 * tracks "not yet migrated" — this step just keeps a migrated crate's descriptor fresh automatically. */
function describeBuiltPlugin(target: PluginRegistryEntry, artifact: string): void {
  const describeScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts");
  const ownerRoot = join(repoRoot, target.cratePath, "..", "..");
  const status = runCmdStatus("bun", [describeScript, "describe", artifact, "--out", ownerRoot], { cwd: repoRoot, budgetMs: buildBudgetMs() });
  if (status !== 0) {
    console.log(`describe skipped for ${target.pluginId} (not yet migrated to the world-actor \`describe\` export, or its wasm isn't built for it) — see 📓️design-abi.md §3`);
  } else {
    console.log(`described ${target.pluginId} -> ${ownerRoot}`);
  }
}

/** @emoji 🎯️ One target's CARGO stage only: `cargo build` + the `describe` emitter (which itself
 * shells out to `cargo build -p semio-framework-plugin-describe` — see that script's doc — so it
 * belongs on the serial/cargo side, not the parallel materialize side below, even though it is not
 * technically compiling `target` itself). Never call two of these concurrently — see
 * `buildPluginCatalog`'s doc for why cargo must stay serial. */
async function buildPluginCargo(target: PluginRegistryEntry): Promise<{ readonly target: PluginRegistryEntry; readonly artifact: string }> {
  const packageName = await readPackageName(target.cratePath);
  const profile = pluginWasmProfile();
  if (runCmdStatus("cargo", pluginCargoArgs(packageName, profile), { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) {
    throw new Error(`plugin build failed: ${target.pluginId}`);
  }
  const cargoTargetRoot = process.env.CARGO_TARGET_DIR ? resolve(repoRoot, process.env.CARGO_TARGET_DIR) : join(repoRoot, "target");
  const artifact = join(cargoTargetRoot, PLUGIN_WASM_TARGET, cargoProfileDir(profile), `${packageName.replace(/-/g, "_")}.wasm`);
  describeBuiltPlugin(target, artifact);
  return { target, artifact };
}

/** @emoji 🎯️ One target's MATERIALIZE stage: jco transpile, `wasm-opt`, bridge/host-shim file
 * emission, extension publish, hot-swap marker — everything downstream of a finished cargo artifact
 * that touches neither the shared `target/` build-directory lock nor the global `~/.cargo`
 * package-cache lock, so it is safe to run several of these at once (see `buildPluginCatalog`). Does
 * NOT call `publishShardWorker()` — that write is identical content for every target in a catalog run,
 * so callers publish it once rather than redundantly per plugin (still "idempotent: rewritten on every
 * plugin build" per its own doc, just once per BUILD rather than once per PLUGIN). */
async function materializePlugin(target: PluginRegistryEntry, artifact: string): Promise<void> {
  const outDir = join(pluginOutRoot, target.pluginId);
  mkdirSync(outDir, { recursive: true });
  const jsBase = target.wasmOut.replace(/\.wasm$/, "");
  const componentBase = `${jsBase}_component`;
  cleanStalePluginOutputs(outDir, jsBase, componentBase);
  writeFileSync(join(outDir, "🟨️host-shim.js"), hostShimSource());
  stagePluginDescriptor(target, outDir);
  // 🪶️ Transpile straight from cargo's own build output — plugin-modules never receives a copy of the
  // full component `.wasm` (see `emitRustArtifacts`'s doc comment). The browser only ever fetches
  // jco's extracted `${componentBase}.core.wasm`, so shipping the untranspiled component alongside it
  // was pure duplicate ~60MB-class weight per plugin; native `os run` now reads straight from `target/`.
  // 🚀️ T-P8: the ASYNC (non-blocking-spawn) transpile — see its doc — is what makes `buildPluginCatalog`'s
  // bounded-parallel materialize stage actually overlap in wall-clock time, not just in scheduling.
  await transpilePluginComponentAsync(artifact, outDir, componentBase, pluginWebMaterializeContext());
  const jsOut = join(outDir, `${jsBase}.js`);
  writeFileSync(jsOut, pluginComponentBridgeSource(componentBase, target.wasmOut));
  // 🧩️ Publish extension artifacts before the hot-swap marker: the browser reloads `/extensions/...`
  // from the SSE event, so the install root must already serve the new files.
  publishBuiltExtension(target, outDir);
  const hotSwapMarker = join(pluginOutRoot, ".hot-swap");
  writeFileSync(hotSwapMarker, `${JSON.stringify({ pluginId: target.pluginId, rebuiltAt: Date.now() })}\n`);
  console.log(`built program ${target.pluginId} (${PLUGIN_WASM_TARGET}, ${pluginWasmProfile()}) -> ${outDir}`);
}

/** @emoji 🎯️ Builds exactly one target end to end (cargo then materialize then the shared shard-worker
 * publish) — used where only one crate is being built at a time, so there is no concurrency to bound:
 * the file-watch rebuild loop (`watchPluginRebuilds`, which deliberately serializes overlapping rebuild
 * requests onto the SAME `target/` cargo lock) and the two-crate collab-e2e prebuild. The full-catalog
 * entry points (`buildPlugins`/`buildPluginsStreaming`) go through `buildPluginCatalog` instead, which
 * pipelines this same pair of stages across many targets. */
async function buildPlugin(target: PluginRegistryEntry): Promise<void> {
  const { artifact } = await buildPluginCargo(target);
  await materializePlugin(target, artifact);
  publishShardWorker();
}

/** @emoji 🧵️ Minimal counting semaphore bounding how many `fn()` calls run concurrently. Local to this
 * file rather than promoted to the shared repo-lib — this packet's ownership (`📌️important.md`
 * registrar-only list) is scoped to `📜️script.ts` and `🌐plugin-web-materialize.ts` only. FIFO wakeup,
 * never reorders which caller gets the next free slot. */
function createConcurrencyLimiter(limit: number): { run: <T>(fn: () => Promise<T>) => Promise<T> } {
  let active = 0;
  const queue: Array<() => void> = [];
  async function acquire(): Promise<void> {
    if (active < limit) {
      active++;
      return;
    }
    await new Promise<void>((wake) => queue.push(wake));
    active++;
  }
  function release(): void {
    active--;
    queue.shift()?.();
  }
  return {
    async run<T>(fn: () => Promise<T>): Promise<T> {
      await acquire();
      try {
        return await fn();
      } finally {
        release();
      }
    },
  };
}

/** @emoji 🧵️ Concurrency cap for the MATERIALIZE stage only (see `buildPluginCatalog`) — jco transpile
 * and `wasm-opt` are each single-process, mostly-single-threaded-per-invocation CPU-bound subprocesses,
 * and each holds a decoded wasm module plus jco's own intermediate JS AST in memory while running. 4 is
 * a deliberately small constant, not tied to `hardwareConcurrency`: unlike the cargo stage (one process
 * for the whole build, sharing rustc's own parallelism internally), materialize concurrency is
 * ~N-processes-at-once, and an unbounded `Promise.all` over a ~20-58-plugin catalog risks the same class
 * of machine-saturation `📌️important.md` records for parallel cargo (174 concurrent processes, 40
 * minutes, nothing produced) — just with jco/wasm-opt instead of rustc. `SEMIO_MATERIALIZE_CONCURRENCY`
 * overrides it for measurement/tuning. */
function materializeConcurrencyLimit(): number {
  const override = process.env.SEMIO_MATERIALIZE_CONCURRENCY;
  if (override) {
    const parsed = Number.parseInt(override, 10);
    if (Number.isFinite(parsed) && parsed > 0) return parsed;
  }
  return 4;
}

/** @emoji 🚰️ Builds a whole catalog of `orderedTargets`: the CARGO stage runs strictly serially, one
 * `cargo build` at a time, in `orderedTargets`' own order — never two overlapping, exactly as before
 * this packet, since parallel `cargo` is the repeatedly-machine-saturating failure mode
 * `📌️important.md` records. The MATERIALIZE stage for each target that finished its cargo build is
 * enqueued into a bounded pool (`materializeConcurrencyLimit()`, default 4) WITHOUT the cargo loop
 * waiting for it — so target N+1's `cargo build` runs concurrently with target N's (and N-1's, up to the
 * cap) jco/wasm-opt/file-emission pass, instead of the old fully-interleaved `buildPlugin` forcing every
 * cargo build to wait out the previous target's ENTIRE materialize pass first. This is the actual fix
 * for the serialized-materialize-stage finding: overlap, not just "run materialize in parallel with
 * itself". `publishShardWorker()` (identical content for every target) is written once at the end
 * rather than once per target. Injectable `cargoFn`/`materializeFn`/`publishShardWorkerFn` so this can
 * be exercised in tests without a real `cargo`/`jco`/`wasm-opt` toolchain or filesystem writes. */
async function buildPluginCatalog(
  orderedTargets: readonly PluginRegistryEntry[],
  cargoFn: (target: PluginRegistryEntry) => Promise<{ readonly artifact: string }> = buildPluginCargo,
  materializeFn: (target: PluginRegistryEntry, artifact: string) => Promise<void> = materializePlugin,
  concurrencyLimit: number = materializeConcurrencyLimit(),
  publishShardWorkerFn: () => void = publishShardWorker,
): Promise<{ readonly failedPluginIds: readonly string[] }> {
  const limiter = createConcurrencyLimiter(concurrencyLimit);
  const failed: string[] = [];
  const materializeTasks: Promise<void>[] = [];
  for (const target of orderedTargets) {
    let cargoResult: { readonly artifact: string };
    try {
      cargoResult = await cargoFn(target);
    } catch (error) {
      failed.push(target.pluginId);
      console.error(`plugin build failed, continuing with remaining targets: ${target.pluginId}`, error);
      continue;
    }
    const { artifact } = cargoResult;
    materializeTasks.push(
      limiter.run(async () => {
        try {
          await materializeFn(target, artifact);
        } catch (error) {
          failed.push(target.pluginId);
          console.error(`plugin materialize failed: ${target.pluginId}`, error);
        }
      }),
    );
  }
  await Promise.all(materializeTasks);
  publishShardWorkerFn();
  return { failedPluginIds: failed };
}

/** 🛑 Rejects incomplete explicit builds after every target has been attempted. */
function assertPluginCatalogComplete(failedPluginIds: readonly string[]): void {
  if (failedPluginIds.length > 0) throw new Error(`plugin catalog build failed: ${failedPluginIds.join(", ")}`);
}

export async function ensurePluginRegistry(filterPlugin?: string): Promise<void> {
  const registryScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts");
  if (runCmdStatus("bun", [registryScript, "generate"], { cwd: repoRoot }) !== 0) throw new Error("plugin registry generation failed");
  const variant = filterPlugin ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT;
  const filterPluginId = resolveCatalogFilterPluginId(filterPlugin);
  syncBuiltPluginDescriptors(generatePluginRegistry(repoRoot, filterPluginId ? { filterPlaygroundPlugin: filterPluginId } : {}));
  writePlaygroundSession(variant, playgroundSessionPath, repoRoot);
}

function resolvePluginBuildTargets(entries: readonly PluginRegistryEntry[], filterPlugin?: string): readonly PluginRegistryEntry[] {
  // 🎯️ `SEMIO_PLUGIN_ONLY=<pluginId>` rebuilds one crate even under a studio host filter (`s`), which
  // otherwise expands to the full catalog — needed for hot-swap iteration on the host itself.
  const only = process.env.SEMIO_PLUGIN_ONLY?.trim();
  if (only) {
    const matched = entries.filter((entry) => entry.pluginId === only);
    if (matched.length === 0) {
      throw new Error(`SEMIO_PLUGIN_ONLY=${JSON.stringify(only)} matched no plugin crates`);
    }
    return matched;
  }
  if (!filterPlugin || isHostPluginFilter(filterPlugin)) return entries;
  if (entries.length === 0) {
    throw new Error(`no program build targets for filter ${JSON.stringify(filterPlugin)}`);
  }
  return entries;
}

/** @emoji 🎯️ Shared setup for every plugin-build entry point below: registry regeneration, output dirs,
 * vendor shims, stale-output cleanup, and the resolved+logged target list — everything a build needs
 * that isn't itself a `cargo build`. Split out of the old monolithic `buildPlugins` so the dev runner's
 * streaming variant can run this fast (no-cargo) prep synchronously before Vite starts, then stream the
 * slow per-crate builds in afterward instead of blocking the first byte on all of them. */
async function preparePluginBuildTargets(filterPlugin?: string): Promise<readonly PluginRegistryEntry[]> {
  ensureWasmTarget();
  await ensurePluginRegistry(filterPlugin);
  const filterPluginId = resolveCatalogFilterPluginId(filterPlugin);
  const catalogEntries = generatePluginRegistry(repoRoot, filterPluginId ? { filterPlaygroundPlugin: filterPluginId } : {});
  mkdirSync(pluginOutRoot, { recursive: true });
  ensurePreview2ShimVendor();
  ensureGuestSlimTypstFontsAsset();
  rewriteExistingPluginShimImports();
  const stalePublicPlugins = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/public/plugin-modules");
  if (existsSync(stalePublicPlugins)) {
    rmSync(stalePublicPlugins, { recursive: true, force: true });
  }
  sweepStaleExtensionModuleOutputs();
  const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin);
  syncBuiltExtensionsToInstallRoot(targets);
  if (filterPlugin && !isHostPluginFilter(filterPlugin)) {
    console.log(`program build scope: ${targets.map((target) => target.pluginId).join(", ")}`);
  } else {
    console.log(`program build scope: all (${targets.length} plugin crates)`);
  }
  return targets;
}

/** @emoji 🎪️ Exported so a multi-variant host (e.g. the mit-bestand demonstrator, which needs every one
 * of its six panes' plugin crates built into the SAME shared `🔌️plugin-modules/` dir rather than one
 * variant's own isolated dev/build) can call this directly per variant instead of shelling out to this
 * script's own CLI once per variant.
 *
 * Every target is attempted before the summary reports failures. Unlike the streaming dev build,
 * an incomplete explicit build rejects so callers cannot mistake stale artifacts for fresh outputs. */
export async function buildPlugins(filterPlugin?: string): Promise<void> {
  ensureAppleDeveloperDir();
  const targets = await preparePluginBuildTargets(filterPlugin);
  const { failedPluginIds } = await buildPluginCatalog(targets);
  const builtCount = targets.length - failedPluginIds.length;
  console.log(`plugin catalog build summary: ${builtCount}/${targets.length} crate(s) produced .wasm`);
  if (failedPluginIds.length > 0) {
    console.log(`plugin catalog build failures (${failedPluginIds.length}): ${failedPluginIds.join(", ")}`);
  }
  assertPluginCatalogComplete(failedPluginIds);
}

/** @emoji 🌊️ Host-plugin-first, best-effort variant of `buildPlugins` for the dev runner's streaming
 * boot (`DevScript`, react renderer only): the shell's boot effect gates only on the host/primary
 * plugin (see os-core's `hostConfig` path), so building it first (and cargo-building it before any
 * other crate) gets the shell out of its "waiting for host program" state fastest — every other crate
 * streams in afterward via the `.hot-swap`/SSE channel, in whatever order the registry lists them.
 * `buildPluginCatalog` keeps the cargo stage itself serial (concurrent `cargo build`s just contend on
 * the shared `target/` lock) but overlaps each target's MATERIALIZE stage with the next target's cargo
 * build (T-P8) — a single broken crate no longer aborts the rest of the catalog either way. */
export async function buildPluginsStreaming(filterPlugin?: string): Promise<void> {
  const targets = await preparePluginBuildTargets(filterPlugin);
  const hostPluginId = resolvePlaygroundFilter(filterPlugin ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT).pluginId;
  const ordered = [...targets].sort((a, b) => (a.pluginId === hostPluginId ? -1 : b.pluginId === hostPluginId ? 1 : 0));
  await buildPluginCatalog(ordered);
}

class PluginBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
    await buildPlugins(filterPlugin || undefined);
  }
}

//#region 🪶️PluginSizeMeasurement
/** @emoji 📏️ One built plugin's jco-extracted core wasm module, section-walked byte-for-byte — no
 * external tooling (`wasm-tools`/`twiggy`) required, so this runs anywhere `bun` runs. */
type PluginWasmSizeBreakdown = {
  readonly totalBytes: number;
  readonly codeBytes: number;
  readonly dataBytes: number;
  readonly nameBytes: number;
  readonly otherCustomBytes: number;
  readonly functionCount: number;
  readonly memoryInitialPages: number | null;
  readonly memoryMaxPages: number | null;
};

type PluginWasmSizeRow = PluginWasmSizeBreakdown & { readonly pluginId: string; readonly file: string };

/** @emoji 🔢️ Reads one unsigned LEB128 varint starting at `offset`; returns the decoded value and the
 * offset just past it. Values here (section sizes, function/page counts) never approach 2^53, so a
 * bigint accumulator collapsed to `Number` is safe and simpler than juggling two code paths. */
function readULEB128(buf: Buffer, offset: number): { readonly value: number; readonly next: number } {
  let result = 0n;
  let shift = 0n;
  let pos = offset;
  for (;;) {
    const byte = buf[pos]!;
    pos += 1;
    result |= BigInt(byte & 0x7f) << shift;
    if ((byte & 0x80) === 0) break;
    shift += 7n;
  }
  return { value: Number(result), next: pos };
}

/** @emoji 📏️ Byte-level breakdown of one core wasm module's sections. Section ids per the wasm binary
 * format: 0=custom (name-prefixed — the "name" custom section is pure debug/dev-tooling weight, see
 * `[profile.wasm-release]`'s `strip = "symbols"`), 5=memory, 10=code, 11=data. Reports only the first
 * declared memory's limits — every plugin here declares exactly one. */
function analyzePluginWasmModule(filePath: string): PluginWasmSizeBreakdown {
  const buf = readFileSync(filePath);
  const totalBytes = buf.byteLength;
  if (buf.length < 8 || buf.readUInt32LE(0) !== 0x6d736100) {
    throw new Error(`not a wasm module: ${filePath}`);
  }
  let offset = 8;
  let codeBytes = 0;
  let dataBytes = 0;
  let nameBytes = 0;
  let otherCustomBytes = 0;
  let functionCount = 0;
  let memoryInitialPages: number | null = null;
  let memoryMaxPages: number | null = null;
  while (offset < buf.length) {
    const sectionId = buf[offset]!;
    offset += 1;
    const sectionSizeRead = readULEB128(buf, offset);
    const sectionSize = sectionSizeRead.value;
    const sectionStart = sectionSizeRead.next;
    const sectionEnd = sectionStart + sectionSize;
    if (sectionId === 10) {
      codeBytes += sectionSize;
      functionCount += readULEB128(buf, sectionStart).value;
    } else if (sectionId === 11) {
      dataBytes += sectionSize;
    } else if (sectionId === 5) {
      const countRead = readULEB128(buf, sectionStart);
      if (countRead.value > 0) {
        const flagsRead = readULEB128(buf, countRead.next);
        const hasMax = (flagsRead.value & 0x01) !== 0;
        const minRead = readULEB128(buf, flagsRead.next);
        memoryInitialPages = minRead.value;
        memoryMaxPages = hasMax ? readULEB128(buf, minRead.next).value : null;
      }
    } else if (sectionId === 0) {
      const nameLenRead = readULEB128(buf, sectionStart);
      const customName = buf.toString("utf8", nameLenRead.next, nameLenRead.next + nameLenRead.value);
      if (customName === "name") nameBytes += sectionSize;
      else otherCustomBytes += sectionSize;
    }
    offset = sectionEnd;
  }
  return { totalBytes, codeBytes, dataBytes, nameBytes, otherCustomBytes, functionCount, memoryInitialPages, memoryMaxPages };
}

const PLUGIN_SIZE_REPORT_PATH = join(pluginOutRoot, ".size-report.json");

/** @emoji 📏️ Every jco-extracted core wasm module currently on disk under `plugin-modules/`, largest
 * first. `_vendor` and other non-plugin dirs are skipped the same way `rewriteExistingPluginShimImports`
 * skips them. */
function collectPluginWasmSizeRows(): PluginWasmSizeRow[] {
  if (!existsSync(pluginOutRoot)) return [];
  const rows: PluginWasmSizeRow[] = [];
  for (const entry of readdirSync(pluginOutRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name.startsWith("_")) continue;
    const pluginDir = join(pluginOutRoot, entry.name);
    for (const file of readdirSync(pluginDir)) {
      if (!/\.core\d*\.wasm$/.test(file)) continue;
      rows.push({ pluginId: entry.name, file, ...analyzePluginWasmModule(join(pluginDir, file)) });
    }
  }
  return rows.sort((a, b) => b.totalBytes - a.totalBytes);
}

function formatPluginSizeBytes(n: number): string {
  return `${(n / (1024 * 1024)).toFixed(2)}MB`;
}

type EngineWasmSizeRow = PluginWasmSizeBreakdown & { readonly engineId: string; readonly file: string };

const ENGINE_SIZE_REPORT_PATH = join(pluginOutRoot, ".engine-size-report.json");

/** @emoji 📏️ Every wasm-bindgen engine's `*_bg.wasm` currently built under `node_modules/@semio-tech/*`
 * (flow-core, node-graph, editor, tiled-map, paint, terrain, board-2d — see `runWasmPackWebBuild`'s
 * `profile` option). These packages are workspace-symlinked (bun links `node_modules/@semio-tech/<pkg>`
 * to the crate dir), so each entry's realpath is resolved before scanning its `pkg/` dir. Reuses
 * `analyzePluginWasmModule` verbatim — it's generic wasm section accounting, not plugin-specific. */
function collectEngineWasmSizeRows(): EngineWasmSizeRow[] {
  const scopeDir = join(repoRoot, "node_modules/@semio-tech");
  if (!existsSync(scopeDir)) return [];
  const rows: EngineWasmSizeRow[] = [];
  for (const entry of readdirSync(scopeDir, { withFileTypes: true })) {
    let pkgDir: string;
    try {
      pkgDir = join(realpathSync(join(scopeDir, entry.name)), "pkg");
    } catch {
      continue;
    }
    if (!existsSync(pkgDir)) continue;
    for (const file of readdirSync(pkgDir)) {
      if (!file.endsWith("_bg.wasm")) continue;
      rows.push({ engineId: entry.name, file, ...analyzePluginWasmModule(join(pkgDir, file)) });
    }
  }
  return rows.sort((a, b) => b.totalBytes - a.totalBytes);
}

/** @emoji 📏️`plugin size` — measures every built plugin's core wasm (total/code/data/name bytes,
 * function count, memory initial/max pages), prints a per-plugin + total report, and persists
 * `.size-report.json` so the next run prints deltas — makes wasm-release/wasm-opt/dedup regressions
 * visible without re-deriving byte counts by hand. */
class PluginSizeScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    const rows = collectPluginWasmSizeRows();
    if (rows.length === 0) {
      console.log("no built plugin core wasm modules found under plugin-modules/ — run `plugin` (build) first");
      return;
    }
    const previousRows: readonly PluginWasmSizeRow[] = existsSync(PLUGIN_SIZE_REPORT_PATH) ? (JSON.parse(readFileSync(PLUGIN_SIZE_REPORT_PATH, "utf8")) as PluginWasmSizeRow[]) : [];
    const previousByKey = new Map(previousRows.map((row) => [`${row.pluginId}/${row.file}`, row]));
    let totalBytes = 0;
    let totalCode = 0;
    let totalData = 0;
    let totalName = 0;
    let totalFunctions = 0;
    console.log(`plugin wasm size report (${rows.length} modules)`);
    for (const row of rows) {
      totalBytes += row.totalBytes;
      totalCode += row.codeBytes;
      totalData += row.dataBytes;
      totalName += row.nameBytes;
      totalFunctions += row.functionCount;
      const previousRow = previousByKey.get(`${row.pluginId}/${row.file}`);
      const delta = previousRow ? row.totalBytes - previousRow.totalBytes : null;
      const deltaLabel = delta === null ? "(new)" : delta === 0 ? "(=)" : `(${delta > 0 ? "+" : ""}${formatPluginSizeBytes(delta)})`;
      const maxLabel = row.memoryMaxPages === null ? "unbounded" : `${row.memoryMaxPages}pg`;
      console.log(
        `  ${row.pluginId.padEnd(16)} total=${formatPluginSizeBytes(row.totalBytes)} code=${formatPluginSizeBytes(row.codeBytes)} data=${formatPluginSizeBytes(row.dataBytes)} name=${formatPluginSizeBytes(row.nameBytes)} fns=${row.functionCount} mem=${row.memoryInitialPages ?? "?"}/${maxLabel} ${deltaLabel}`,
      );
    }
    console.log(`total: ${formatPluginSizeBytes(totalBytes)} (code ${formatPluginSizeBytes(totalCode)}, data ${formatPluginSizeBytes(totalData)}, name ${formatPluginSizeBytes(totalName)}, ${totalFunctions} functions across ${rows.length} modules)`);
    writeFileSync(PLUGIN_SIZE_REPORT_PATH, `${JSON.stringify(rows, null, 2)}\n`);

    const engineRows = collectEngineWasmSizeRows();
    if (engineRows.length === 0) return;
    const previousEngineRows: readonly EngineWasmSizeRow[] = existsSync(ENGINE_SIZE_REPORT_PATH) ? (JSON.parse(readFileSync(ENGINE_SIZE_REPORT_PATH, "utf8")) as EngineWasmSizeRow[]) : [];
    const previousEngineByKey = new Map(previousEngineRows.map((row) => [`${row.engineId}/${row.file}`, row]));
    let engineTotalBytes = 0;
    let engineTotalCode = 0;
    let engineTotalData = 0;
    let engineTotalName = 0;
    console.log(`engine wasm size report (${engineRows.length} modules)`);
    for (const row of engineRows) {
      engineTotalBytes += row.totalBytes;
      engineTotalCode += row.codeBytes;
      engineTotalData += row.dataBytes;
      engineTotalName += row.nameBytes;
      const previousRow = previousEngineByKey.get(`${row.engineId}/${row.file}`);
      const delta = previousRow ? row.totalBytes - previousRow.totalBytes : null;
      const deltaLabel = delta === null ? "(new)" : delta === 0 ? "(=)" : `(${delta > 0 ? "+" : ""}${formatPluginSizeBytes(delta)})`;
      console.log(`  ${row.engineId.padEnd(40)} total=${formatPluginSizeBytes(row.totalBytes)} code=${formatPluginSizeBytes(row.codeBytes)} data=${formatPluginSizeBytes(row.dataBytes)} name=${formatPluginSizeBytes(row.nameBytes)} ${deltaLabel}`);
    }
    console.log(
      `engine total: ${formatPluginSizeBytes(engineTotalBytes)} (code ${formatPluginSizeBytes(engineTotalCode)}, data ${formatPluginSizeBytes(engineTotalData)}, name ${formatPluginSizeBytes(engineTotalName)} across ${engineRows.length} modules)`,
    );
    writeFileSync(ENGINE_SIZE_REPORT_PATH, `${JSON.stringify(engineRows, null, 2)}\n`);
  }
}
//#endregion 🪶️PluginSizeMeasurement

/** @emoji 👀️ A plugin crate's edits alone don't cover every source that feeds its build: multi-crate
 * app families (e.g. `fem/plugin/rs` depending on `fem/2d/rs`/`fem/3d/rs`/`fem/core/rs`, or an
 * example fixture under `fem/2d/example`) live as SIBLING directories under the same top-level app
 * folder, not inside the plugin crate itself. Watching just `target.cratePath` misses them, so a
 * schema or fixture edit never triggers a hot-swap rebuild. Framework-hosted plugin crates
 * (`framework/...`) keep the narrow crate-only watch instead — widening to all of `framework/` would
 * watch the entire monorepo's shared core. Cargo's own `target/` output lives at the repo root and
 * built wasm lands in `framework/os/dev/plugin-modules`, so widening the watch root here
 * cannot cause a rebuild to re-trigger itself. */
function pluginWatchRoot(target: PluginRegistryEntry): string {
  const segments = target.cratePath.split("/");
  const topLevel = segments[0];
  if (topLevel === "🧰️framework" || topLevel === "framework") return join(repoRoot, target.cratePath);
  // 🏛️ Post-restructure: sibling crate families live under `✏️s/🔌️plugins/<p>/...` (was `s/plugin/<p>/...`).
  // Widening to `✏️s/` would watch every plugin's tree on every crate's edit.
  if ((topLevel === "✏️s" || topLevel === "s") && (segments[1] === "🔌️plugins" || segments[1] === "plugins")) {
    return join(repoRoot, segments.slice(0, 3).join("/"));
  }
  return join(repoRoot, topLevel);
}

/** @emoji 👀️ Rebuilds each of `targets` on source change — one `fs.watch` per crate (see
 * `pluginWatchRoot`) feeding a single dirty-set queue that drains serially. Two crates edited in quick
 * succession (or one crate touched again before its own rebuild finishes) used to fire overlapping
 * `void buildPlugin(...)` calls that raced each other against the same `target/` cargo lock; the dirty
 * set collapses any number of change events for one crate into a single pending rebuild, and the drain
 * loop only ever runs one `buildPlugin` at a time. Shared by both the standalone `plugin watch` command
 * and `DevScript`'s streaming boot, which folds this in right after the initial build pass so plugin
 * edits keep hot-swapping the running shell for the rest of the dev session. */
function watchPluginRebuilds(targets: readonly PluginRegistryEntry[]): void {
  const byPluginId = new Map(targets.map((target) => [target.pluginId, target] as const));
  const dirty = new Set<string>();
  let draining = false;

  async function drain(): Promise<void> {
    if (draining) return;
    draining = true;
    try {
      while (dirty.size > 0) {
        const [pluginId] = dirty;
        dirty.delete(pluginId!);
        const target = byPluginId.get(pluginId!);
        if (!target) continue;
        try {
          await buildPlugin(target);
        } catch (error) {
          console.error("program watch rebuild failed", error);
        }
      }
    } finally {
      draining = false;
    }
  }

  for (const target of targets) {
    watch(pluginWatchRoot(target), { recursive: true }, () => {
      dirty.add(target.pluginId);
      void drain();
    });
  }
  console.log("watching plugin crates for hot-swap rebuilds");
}

class PluginWatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
    await buildPlugins(filterPlugin || undefined);
    const filterPluginId = resolveCatalogFilterPluginId(filterPlugin || undefined);
    const catalogEntries = generatePluginRegistry(repoRoot, filterPluginId ? { filterPlaygroundPlugin: filterPluginId } : {});
    const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin || undefined);
    watchPluginRebuilds(targets);
  }
}

/** @emoji 🔎️ Resolves an `engines` crate path (from the playground registry) to its `script.ts` wasm
 * build entry point — most engine crates keep `script.ts` inside the `rs` dir itself, a few (e.g.
 * `flow/core/rs`) keep it one level up next to the crate's TS sibling, so both are tried. */
function engineWasmScriptPath(cratePath: string): string {
  const direct = join(repoRoot, cratePath, "📜️script.ts");
  if (existsSync(direct)) return direct;
  const parent = cratePath.endsWith("/rs") ? cratePath.slice(0, -"/rs".length) : cratePath;
  const parentScript = join(repoRoot, parent, "📜️script.ts");
  if (existsSync(parentScript)) return parentScript;
  throw new Error(`no wasm build script found for engine crate ${cratePath}`);
}

/** @emoji 🍎 Prefer Command Line Tools over an unlicensed Xcode.app so cargo/wasm-pack can link. */
function ensureAppleDeveloperDir(): void {
  if (process.env.FORCE_XCODE === "1") return;
  const clt = "/Library/Developer/CommandLineTools";
  if (!existsSync(clt)) return;
  // Prefer CLT over an installed-but-unlicensed Xcode.app (cargo/cc otherwise die with exit 69).
  process.env.DEVELOPER_DIR = clt;
  const sdk = `${clt}/SDKs/MacOSX.sdk`;
  if (existsSync(sdk)) process.env.SDKROOT = sdk;
}

/** @emoji 🔌️ Builds every wasm engine a react-renderer dev session needs: the framework node-graph +
 * editor host engines unconditionally (shared studio chrome, not any one app), then whatever the
 * active playground variant declares via `engines = […]` on its `[[…playground]]` Cargo.toml row —
 * replaces the previous hardcoded `if (pluginId === "flow" | "gis2d" | "gis3d" | "raster" | "puzzle2d")` branches. */
export async function buildEngineWasm(variant: string, renderer: string): Promise<void> {
  ensureAppleDeveloperDir();
  if (renderer !== "react" || process.env.SKIP_ENGINE_BUILD === "1") return;
  if (process.env.FORCE_ENGINE_BUILD !== "1") {
    const surfacePkgJs = join(repoRoot, "./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/pkg/framework_surface.js");
    const editorPkgWasm = join(repoRoot, "./🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/framework_editor_bg.wasm");
    const flowPkgWasm = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/flow_core_bg.wasm");
    if (existsSync(surfacePkgJs) && existsSync(editorPkgWasm) && existsSync(flowPkgWasm)) {
      console.log("reusing existing engine wasm pkg/ (set FORCE_ENGINE_BUILD=1 to rebuild)");
      return;
    }
  }
  // Each recurses into a crate's own `wasm` script (wasm-pack/cargo build under the hood) — budgeted at
  // the build class rather than the generic command default since those inner builds can legitimately
  // approach [[buildBudgetMs]] themselves.
  const graphScript = join(repoRoot, "./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts");
  if (runCmdStatus("bun", [graphScript, "wasm"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error("framework-surface-node-graph wasm build failed");
  const editorScript = join(repoRoot, "./🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts");
  if (runCmdStatus("bun", [editorScript, "wasm"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error("framework-editor wasm build failed");
  const boardScript = join(repoRoot, "./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts");
  if (runCmdStatus("bun", [boardScript, "wasm"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error("framework-surface-board-2d wasm build failed");
  // React renderer `import("@semio-tech/flow-core")` for `createFlowSession`, so the pkg should exist
  // even when the active playground's `engines = []` (e.g. Aggregator / puzzle).
  //
  // ⚠️ That import is LAZY — it happens inside `createFlowSession`, so a shell whose surfaces never open
  // a flow graph (Home, Space, Writer, …) runs perfectly without it. A broken flow crate must therefore
  // degrade this build, not abort it: throwing here meant one unrelated subsystem's compile errors took
  // down every `dev` server, including both hub user launchers. Warn loudly and continue; anything that
  // actually needs a flow session fails visibly at that point instead.
  const flowCorePkgWasm = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/flow_core_bg.wasm");
  if (!existsSync(flowCorePkgWasm)) {
    const flowCoreScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts");
    if (runCmdStatus("bun", [flowCoreScript, "wasm"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) {
      console.warn("[dev] flow-core wasm build failed — continuing without it; surfaces that open a flow graph will fail until it builds again.");
    }
  }
  const row = playgroundCatalog.find((entry) => entry.variant === variant);
  for (const engineCratePath of row?.engines ?? []) {
    if (engineCratePath === "./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust" || engineCratePath === "./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🟦️typescript/🫀️core") continue;
    const script = engineWasmScriptPath(engineCratePath);
    if (runCmdStatus("bun", [script, "wasm"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error(`${engineCratePath} wasm build failed`);
  }
}

/** @emoji 🐚️ Fixed port for `dev multi` — the multi-shell harness (`🧩️multi.tsx`), free in the 60xx range
 * used by every other os-dev variant/launch.json entry. */
const FRAMEWORK_OS_MULTI_HARNESS_PORT = "6071";

//#region 🔖️PollHelpers
/** @emoji 🏛️ THE RULE (poll census, W6): a deadline-bounded poll is legitimate here if and only if
 * the thing it waits on is EXTERNAL — a TCP port or HTTP endpoint belonging to a process we did not
 * instrument, a lease file another `dev` invocation owns, a filesystem lock — and therefore emits no
 * observable event we could await instead. The moment we hold the resource's own handle (a spawned
 * child's `exit` event, a stream, a promise it already exposes), polling it on a timer is NOT
 * legitimate; await the handle instead ([[awaitChildExit]] below replaced two such polls).
 * [[awaitTcpReady]] / [[awaitHttpOk]] exist for the legitimate case only — real external readiness
 * checks this process has no better signal for. A future poll census should find every `Bun.sleep`
 * loop either inside one of these three helpers, or commented at its call site explaining which
 * external resource it legitimately waits on (see `waitForPluginBuildLeaseReady`'s lease-file poll
 * and `prebuildParityPlugin`'s mkdir-lock poll — both fs-based waits on a PID/lock this process holds
 * no handle for, so neither fits a TCP/HTTP shape). */

type PollOutcome = "ready" | "dead" | "timeout";

/** @emoji ⏳️ Deadline-bounded poll for a TCP `port` on `host` to reach the wanted state — open
 * (`mode: "open"`, the default: something is now listening) or closed (`mode: "closed"`: nothing is
 * listening anymore). Checks every `intervalMs`, capped at `deadlineMs` total from the call, and can
 * race an optional `isDead()` predicate (e.g. `child.exitCode !== null`) so a spawn that already died
 * does not have to wait out the full deadline before its caller finds out. `probe`/`sleep`/`now` are
 * test-only injection points — production callers rely on the defaults ([[isDevPortInUse]]/
 * `Bun.sleep`/`Date.now`). Never throws; callers turn the [[PollOutcome]] into whatever error
 * message fits their own call site. */
async function awaitTcpReady(
  host: string,
  port: number,
  opts: {
    readonly deadlineMs: number;
    readonly intervalMs: number;
    readonly mode?: "open" | "closed";
    readonly isDead?: () => boolean;
    readonly probe?: (host: string, port: number) => boolean;
    readonly sleep?: (ms: number) => Promise<void>;
    readonly now?: () => number;
  },
): Promise<PollOutcome> {
  const mode = opts.mode ?? "open";
  const probe = opts.probe ?? isDevPortInUse;
  const sleep = opts.sleep ?? ((ms: number) => Bun.sleep(ms));
  const now = opts.now ?? Date.now;
  const deadline = now() + opts.deadlineMs;
  while (now() < deadline) {
    const inUse = probe(host, port);
    if (mode === "open" ? inUse : !inUse) return "ready";
    if (opts.isDead?.()) return "dead";
    await sleep(opts.intervalMs);
  }
  return "timeout";
}

/** @emoji 🌐️ Deadline-bounded poll for `url` to answer any HTTP response at all — per THE RULE
 * above, a `fetch` that throws (connection refused, DNS not up yet) just means the server isn't
 * listening yet, not a real failure. Does not inspect `response.ok`; callers that need a specific
 * status/body check the fetched response themselves once they have their own handle to it — this
 * helper only proves *something* is answering on `url`. Shares [[awaitTcpReady]]'s deadline/isDead/
 * injection shape and [[PollOutcome]]. */
async function awaitHttpOk(
  url: string,
  opts: {
    readonly deadlineMs: number;
    readonly intervalMs: number;
    readonly init?: RequestInit;
    readonly isDead?: () => boolean;
    readonly fetchImpl?: typeof fetch;
    readonly sleep?: (ms: number) => Promise<void>;
    readonly now?: () => number;
  },
): Promise<PollOutcome> {
  const fetchImpl = opts.fetchImpl ?? fetch;
  const sleep = opts.sleep ?? ((ms: number) => Bun.sleep(ms));
  const now = opts.now ?? Date.now;
  const deadline = now() + opts.deadlineMs;
  while (now() < deadline) {
    if (opts.isDead?.()) return "dead";
    try {
      await fetchImpl(url, opts.init);
      return "ready";
    } catch {
      await sleep(opts.intervalMs);
    }
  }
  return "timeout";
}

/** @emoji 🧵️ Resolves once `child` exits — Node's own `'exit'` event, not a poll — or with
 * `"timeout"` after `deadlineMs`, whichever comes first. This is what THE RULE above means by
 * "await the handle instead": a `ChildProcess` we spawned already tells us when it exits, so
 * re-checking `child.exitCode` on a `Bun.sleep` timer is exactly the "wired but inert" shape this
 * ticket exists to remove. Handles the case where `child` already exited before this was called (its
 * `exitCode` is set synchronously before `'exit'` fires, so a late listener would otherwise hang
 * forever). `timeoutAfter` is a test-only injection point for the deadline race; production callers
 * keep the real `setTimeout`. */
async function awaitChildExit(child: SpawnDaemonHandle["child"], deadlineMs: number, opts: { readonly timeoutAfter?: (ms: number) => Promise<"timeout"> } = {}): Promise<"exited" | "timeout"> {
  const timeoutAfter = opts.timeoutAfter ?? ((ms: number) => new Promise<"timeout">((resolve) => setTimeout(() => resolve("timeout"), ms)));
  const exited = new Promise<"exited">((resolve) => {
    if (child.exitCode !== null) {
      resolve("exited");
      return;
    }
    child.once("exit", () => resolve("exited"));
  });
  return Promise.race([exited, timeoutAfter(deadlineMs)]);
}
//#endregion 🔖️PollHelpers

//#region 🔖️PluginBuildLease
/** @emoji 🔐️ One `target/semio-dev-leases/plugin-build-<variant>.json` lease file: the single `dev`
 * process (identified by `pid`) currently doing the ~30-crate `ensurePluginRegistry` +
 * `buildEngineWasm` + `buildPluginsStreaming` + `watchPluginRebuilds` sequence for one playground
 * variant. `registryReady` flips once that holder has the registry catalog and engine wasm on disk —
 * the point at which a second `dev` process for the same variant (a different user/port) can safely
 * start its own Vite against the same `🔌️plugin-modules/` output without repeating any cargo work. */
type PluginBuildLease = { readonly pid: number; readonly port: number; readonly startedAt: number; registryReady: boolean };

/** @emoji ⏳️ Follower poll budget for `registryReady` — generous relative to the registry-generate +
 * engine-wasm-reuse-path holder does before setting it (seconds, not the ~30-crate streaming build). */
const PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS = 60_000;

function pluginBuildLeaseDir(): string {
  return join(repoRoot, "target/semio-dev-leases");
}

function pluginBuildLeasePath(variant: string): string {
  return join(pluginBuildLeaseDir(), `plugin-build-${variant}.json`);
}

/** @emoji 💀️ True when `pid` no longer exists on this machine (cross-platform: `process.kill(pid, 0)`
 * is a liveness probe on POSIX and Windows alike, never an actual kill). */
function isPidAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

function readPluginBuildLease(path: string): PluginBuildLease | undefined {
  try {
    return JSON.parse(readFileSync(path, "utf8")) as PluginBuildLease;
  } catch {
    return undefined;
  }
}

/** @emoji 🔐️ Claims the plugin-build lease for `variant`: atomically creates the lease file (`wx` —
 * fails with `EEXIST` when another live holder exists) or takes over a stale one (dead `pid`) in
 * place. Returns `"holder"` for this process, or the live `"follower"` lease otherwise. */
function acquirePluginBuildLease(variant: string, port: number): { readonly role: "holder" } | { readonly role: "follower"; readonly lease: PluginBuildLease } {
  mkdirSync(pluginBuildLeaseDir(), { recursive: true });
  const path = pluginBuildLeasePath(variant);
  for (;;) {
    try {
      writeFileSync(path, JSON.stringify({ pid: process.pid, port, startedAt: Date.now(), registryReady: false } satisfies PluginBuildLease, null, 2), { flag: "wx" });
      return { role: "holder" };
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
    }
    const existing = readPluginBuildLease(path);
    if (!existing || !isPidAlive(existing.pid)) {
      try {
        rmSync(path, { force: true });
      } catch {
        // 🏁️ Raced with another taker's own stale-cleanup; the atomic `wx` retry above is the real gate.
      }
      continue;
    }
    return { role: "follower", lease: existing };
  }
}

/** @emoji ✅️ Flips `registryReady` once the holder's registry catalog + engine wasm are on disk. No-op
 * if this process no longer owns the lease (lost to a stale-takeover race). */
function markPluginBuildLeaseReady(variant: string): void {
  const path = pluginBuildLeasePath(variant);
  const lease = readPluginBuildLease(path);
  if (!lease || lease.pid !== process.pid) return;
  writeFileSync(path, JSON.stringify({ ...lease, registryReady: true } satisfies PluginBuildLease, null, 2));
}

/** @emoji 🕰️ Follower-side wait for the holder's `registryReady` flag, capped at
 * `PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS`. Returns `true` when the holder is ready, when the lease file
 * vanishes (holder released/finished), or when its `pid` dies mid-wait — either way nothing is left to
 * wait on.
 *
 * 🚨️ Returns `false` on timeout rather than throwing. This lease is a build-deduplication OPTIMISATION
 * for the two-user launchers, never a precondition for running `dev` at all: a holder that is merely
 * slow (heavy cargo contention) or wedged must degrade a second `dev` into doing its own build, not
 * abort it. Throwing here made a single stale lease file break the primary `dev` workflow outright. */
async function waitForPluginBuildLeaseReady(variant: string, deadlineMs: number): Promise<boolean> {
  const path = pluginBuildLeasePath(variant);
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const lease = readPluginBuildLease(path);
    if (!lease || lease.registryReady || !isPidAlive(lease.pid)) return true;
    // 🏛️ THE RULE (see 🔖️PollHelpers above): legitimate poll, not routed through a helper — the lease
    // holder is another `dev` process entirely, identified only by a `pid` in this on-disk lease file.
    // We hold no handle to it (no child object, no stream, no promise), only its pid, so there is no
    // event to await; a lease file + `isPidAlive` liveness check is the only signal available.
    await Bun.sleep(500);
  }
  return false;
}

/** @emoji 🧾️ Whether the shared build outputs a follower intends to serve are actually on disk — the
 * generated playground catalog plus a non-empty `🔌️plugin-modules/`. Checked instead of trusting the
 * lease flag alone, so a follower never serves an empty module directory just because some other
 * process claimed readiness. */
function pluginBuildOutputsPresent(): boolean {
  try {
    const modules = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules");
    return existsSync(modules) && readdirSync(modules).length > 0;
  } catch {
    return false;
  }
}

/** @emoji 🪓️ Forcibly takes the lease for this process after a follower gave up waiting, so it can do
 * the build itself. Best-effort: losing the ensuing `wx` race just means somebody else holds it and we
 * build anyway, which is wasteful but always correct. */
function takeOverPluginBuildLease(variant: string, port: number): void {
  const path = pluginBuildLeasePath(variant);
  try {
    rmSync(path, { force: true });
  } catch {
    // 🏁️ A vanished lease is the desired end state either way.
  }
  try {
    mkdirSync(pluginBuildLeaseDir(), { recursive: true });
    writeFileSync(path, JSON.stringify({ pid: process.pid, port, startedAt: Date.now(), registryReady: false } satisfies PluginBuildLease, null, 2));
  } catch {
    // 🏁️ Unwritable lease dir is not a reason to refuse to build.
  }
}

/** @emoji 🔓️ Releases this process's own plugin-build lease (no-op if it was never the holder, or lost
 * the lease to a stale-takeover race) — called from `exit`/`SIGINT` so the next `dev` process for the
 * same variant can immediately claim the lease instead of waiting out a dead holder's timeout. */
function releasePluginBuildLease(variant: string): void {
  const path = pluginBuildLeasePath(variant);
  const lease = readPluginBuildLease(path);
  if (!lease || lease.pid !== process.pid) return;
  try {
    rmSync(path, { force: true });
  } catch {
    // 🏁️ Best-effort: a vanished lease file is already the desired end state.
  }
}
//#endregion 🔖️PluginBuildLease

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    ensureAppleDeveloperDir();
    // 🧵️ The shard worker is one package-agnostic file with no per-plugin cargo dependency — publish
    // it unconditionally before any lease/build branching below so `SKIP_PLUGIN_BUILD=1` (the hub
    // `users` launcher's mode, and `multi`, neither of which ever reaches `buildPlugin`) still leaves
    // `/plugin-modules/_shard/🟨️shard-worker.js` on disk for `PluginRuntime.getShardClient()` to fetch.
    publishShardWorker();
    if (segments[0] === "multi") {
      // 🐚️ The multi-shell harness mounts several already-built playground variants' plugin modules
      // side by side (see `🧩️multi.tsx`) — it doesn't own any one variant's plugin/engine build, so it
      // never triggers `buildPlugins`/`buildEngineWasm` itself (unlike every other `dev <variant>`
      // branch below): run `dev note`/`dev gis2d` (or set `SKIP_PLUGIN_BUILD=1` and build by hand) first
      // if their `🔌️plugin-modules/` output is missing or stale. Leaving `SEMIO_PLUGIN` unset makes the
      // vite config fall back to its studio ("s") default, which serves the whole unfiltered
      // `plugin-modules/` directory — exactly what hosting several distinct plugins at once needs.
      await runViteBunxDev(this.root, segments.slice(1), {
        portEnv: "S_OS_PORT",
        defaultPort: FRAMEWORK_OS_MULTI_HARNESS_PORT,
        fixedPort: true,
        env: { SEMIO_RENDERER: "react", VITE_SEMIO_RENDERER: "react" },
      });
      return;
    }
    const variantSegment = segments[0] && !segments[0].startsWith("-") ? segments[0] : undefined;
    const viteSegments = variantSegment ? segments.slice(1) : segments;
    const filterPlugin = variantSegment ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT;
    const renderer = process.env.SEMIO_RENDERER ?? "react";
    const plugin = filterPlugin;
    const defaultPort = String(frameworkOsPlaygroundDefaultPort(playgroundCatalog, plugin, renderer));
    // 🌊️ React serves over Vite, which only needs the fast (no-`cargo`) registry + playground session
    // regenerated before it starts (`⚙️vite.config.ts` imports the generated catalog at config-eval
    // time) — the ~37-crate plugin build itself streams in AFTER Vite is already listening
    // (`buildPluginsStreaming`, called post-`runViteBunxDev` below), instead of blocking the dev
    // server's first byte on every crate finishing. wgpu (native trunk — no browser runtime to stream
    // installs into) and `SKIP_PLUGIN_BUILD=1` (explicitly asks to skip building) keep the original
    // build-then-serve order.
    const streamPluginBuilds = renderer === "react" && process.env.SKIP_PLUGIN_BUILD !== "1";
    // 🔐️ Two `dev s` processes for the same variant (the hub `users` launchers) must not both run the
    // ~30-crate `buildPluginsStreaming` — only the lease holder does; a follower waits for the holder's
    // registry catalog + engine wasm and then serves its own Vite off the same `🔌️plugin-modules/`.
    const leasePort = Number(process.env.S_OS_PORT || defaultPort);
    const pluginBuildLease = streamPluginBuilds ? acquirePluginBuildLease(plugin, leasePort) : undefined;
    if (pluginBuildLease) {
      const release = (): void => releasePluginBuildLease(plugin);
      process.once("exit", release);
      for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"] as const) {
        process.once(signal, () => {
          release();
          process.exit(signal === "SIGINT" ? 130 : 143);
        });
      }
    }
    // 🔀️ `follower` only survives as a role while the holder genuinely delivers: if it never reports
    // ready within the budget, or reported ready without leaving usable outputs on disk, this process
    // takes the lease over and builds for itself. The lease may cost a duplicated build; it may never
    // cost a broken `dev`.
    let leaseRole = pluginBuildLease?.role;
    if (leaseRole === "follower") {
      console.log(`[dev] plugin builds owned by pid ${pluginBuildLease?.role === "follower" ? pluginBuildLease.lease.pid : "?"} (port ${pluginBuildLease?.role === "follower" ? pluginBuildLease.lease.port : "?"}); serving only`);
      const ready = await waitForPluginBuildLeaseReady(plugin, PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS);
      if (!ready) console.warn(`[dev] plugin-build lease for "${plugin}" did not report ready within ${PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS}ms — building here instead of waiting further`);
      else if (!pluginBuildOutputsPresent()) console.warn(`[dev] plugin-build lease for "${plugin}" reported ready but 🔌️plugin-modules/ is empty — building here`);
      if (!ready || !pluginBuildOutputsPresent()) {
        takeOverPluginBuildLease(plugin, leasePort);
        leaseRole = "holder";
      }
    }
    if (leaseRole === "follower") {
      // 🍽️ Holder delivered: nothing to build, serve its outputs.
    } else if (streamPluginBuilds || process.env.SKIP_PLUGIN_BUILD === "1") {
      await ensurePluginRegistry(filterPlugin);
      await buildEngineWasm(plugin, renderer);
      if (leaseRole === "holder") markPluginBuildLeaseReady(plugin);
    } else {
      await buildPlugins(filterPlugin);
      await buildEngineWasm(plugin, renderer);
    }
    if (renderer === "wgpu") {
      const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
      const port = Number(process.env.S_OS_PORT ?? defaultPort);
      const playUrl = wgpuDevPlayUrl(host, port, plugin);
      if (isDevPortInUse(host, port)) {
        const entry = probeWgpuDevPort(host, port);
        if (entry?.entryPath === "/") {
          console.log(`[dev] Port ${port} already serving wgpu trunk at ${playUrl}`);
          return;
        }
        const occupant = describeDevPortOccupant(port);
        if (occupant?.startsWith("trunk")) {
          console.log(`[dev] Restarting stale trunk on port ${port} (${occupant})`);
          stopTrunkDevPort(port);
          // ⏳️ `stopTrunkDevPort` kills a process this function did not spawn (found via port
          // occupancy, not a held child handle) — no exit event available, so a TCP-freed poll via
          // 🔖️PollHelpers's `awaitTcpReady` is the legitimate signal per THE RULE. Same 40×250ms=10s
          // budget as before; outcome intentionally unchecked — the caller proceeds either way, same
          // as the original attempt-bounded loop did.
          await awaitTcpReady(host, port, { deadlineMs: 10_000, intervalMs: 250, mode: "closed" });
        } else if (entry) {
          console.log(`[dev] Port ${port} already serving legacy wgpu trunk at ${wgpuDevPlayUrl(host, port, plugin, entry.entryPath)}`);
          return;
        } else {
          console.error(`[dev] Port ${port} is already in use${occupant ? ` by ${occupant}` : ""}. Stop that process or set S_OS_PORT.`);
          process.exit(1);
        }
      }
      const wgpuScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts");
      const serveStatus = runCmdStatus("bun", [wgpuScript, "serve"], {
        cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu"),
        env: {
          ...process.env,
          SEMIO_PLUGIN: plugin,
          SEMIO_RENDERER: renderer,
          S_OS_PORT: String(port),
        },
        ...daemonBudgetOpts(),
      });
      if (serveStatus !== 0 && !probeWgpuDevPort(host, port)) {
        throw new Error("wgpu trunk serve failed");
      }
      console.log(`[dev] wgpu trunk serving at ${playUrl}`);
      return;
    }
    const resolvedFilter = resolvePlaygroundFilter(plugin);
    // 🐚️ Start Vite without awaiting so plugin builds can stream in while the browser already has a
    // listening shell; then await the Vite child so this process stays alive for the session.
    const viteDone = runViteBunxDev(this.root, viteSegments, {
      portEnv: "S_OS_PORT",
      defaultPort,
      fixedPort: true,
      env: {
        SEMIO_PLUGIN: plugin,
        SEMIO_RENDERER: renderer,
        VITE_SEMIO_RENDERER: renderer,
        VITE_SEMIO_PLUGIN: resolvedFilter.pluginId,
        ...(resolvedFilter.appId ? { VITE_SEMIO_APP_ID: resolvedFilter.appId } : {}),
        ...(resolvedFilter.brand && !process.env.SEMIO_BRAND ? { SEMIO_BRAND: resolvedFilter.brand } : {}),
        ...frameworkOsLockedPrefsEnv(),
      },
    });
    if (leaseRole === "holder") {
      await buildPluginsStreaming(filterPlugin);
      const filterPluginId = resolveCatalogFilterPluginId(filterPlugin);
      const catalogEntries = generatePluginRegistry(repoRoot, filterPluginId ? { filterPlaygroundPlugin: filterPluginId } : {});
      const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin);
      watchPluginRebuilds(targets);
    }
    await viteDone;
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    process.env.SEMIO_BUILD_MODE = "ship";
    const variantSegment = segments[0] && !segments[0].startsWith("-") ? segments[0] : undefined;
    const viteSegments = variantSegment ? segments.slice(1) : segments;
    const plugin = variantSegment ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT;
    await new PluginBuildScript(this.root).run([plugin]);
    const renderer = process.env.SEMIO_RENDERER ?? "react";
    if (renderer === "wgpu" && process.env.SKIP_WGPU_BUILD !== "1") {
      const wgpuScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts");
      if (runCmdStatus("bun", [wgpuScript, "wasm", "--release"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error("wgpu trunk build failed");
      return;
    }
    await buildEngineWasm(plugin, renderer);
    const resolvedFilter = resolvePlaygroundFilter(plugin);
    const viteStatus = runBunxStatus(["vite", "build", "--config", "⚙️vite.config.ts", ...viteSegments], this.root, {
      ...semioShipEnv(),
      SEMIO_PLUGIN: plugin,
      SEMIO_RENDERER: renderer,
      VITE_SEMIO_RENDERER: renderer,
      VITE_SEMIO_PLUGIN: resolvedFilter.pluginId,
      ...(resolvedFilter.appId ? { VITE_SEMIO_APP_ID: resolvedFilter.appId } : {}),
      ...(resolvedFilter.brand && !process.env.SEMIO_BRAND ? { SEMIO_BRAND: resolvedFilter.brand } : {}),
      ...frameworkOsLockedPrefsEnv(),
    });
    if (viteStatus !== 0) throw new Error("framework OS Vite build failed");
  }
}

const PLUGIN_HOST_MODE_SYMBOLS = ["SEMIO_PLUGIN", "PLAYGROUND_APP_KIND", "hostMode", "pluginFilter"] as const;

function walkRustSources(dir: string, out: string[]): void {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    if (ent.name === "target" || ent.name === "node_modules") continue;
    const abs = join(dir, ent.name);
    if (ent.isDirectory()) {
      walkRustSources(abs, out);
      continue;
    }
    if (ent.name.endsWith(".rs")) out.push(abs);
  }
}

/** 🕵️ Statically scans this playground's own `⚙️vite.config.ts` for its hardcoded
 * `{ find: "...", replacement: path.resolve(repoRoot, "...") }` `resolve.alias` entries and
 * asserts every `replacement` target exists on disk — a plain text/regex scan rather than an
 * `import()` of the config module itself, since that module's default export executes a full
 * `defineConfig({...})` (brand/plugin/renderer resolution, plugin-factory calls with real I/O)
 * as an unconditional side effect of module evaluation, which would be unsafe and slow to
 * trigger merely to read one array. Scanning the source text keeps `⚙️vite.config.ts` itself as
 * the single source of truth (no second, independently-stale-able alias list) while still
 * catching a stale alias before it ships silently. Dynamic mount points (`/plugin-modules`,
 * `/renderer-modules`) aren't in this pattern (they resolve local `const` dir variables, not an
 * inline `path.resolve(repoRoot, "...")` literal) and are intentionally not checked here —
 * `renderer-modules` in particular is a build-output directory that legitimately doesn't exist
 * until a wgpu build populates it. */
async function checkPlaygroundAliasFreshness(): Promise<string[]> {
  const viteConfigPath = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts");
  const source = await Bun.file(viteConfigPath).text();
  const aliasPattern = /\{\s*find:\s*"([^"]+)",\s*replacement:\s*path\.resolve\(repoRoot,\s*"([^"]+)"\)\s*\}/g;
  const failures: string[] = [];
  for (const [, find, relativeTarget] of source.matchAll(aliasPattern)) {
    if (!existsSync(join(repoRoot, relativeTarget))) {
      failures.push(`⚙️vite.config.ts: alias "${find}" -> "${relativeTarget}" does not exist on disk`);
    }
  }
  return failures;
}

/** 🚧️ Pre-existing capability-rule violations, real but predating this lint's revival (ticket
 * 26/08/05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL): the filter this rule ran against used
 * to match zero packages, so these went undetected for as long as they've existed. Grandfathered
 * as WARN (not a `verify gate` failure) so reviving the rule with real teeth doesn't redden the
 * gate for unrelated pre-existing plugin work — mirrors the master ticket's general
 * warn-until-finalization pattern for revived W0 checks. Any violation NOT already listed here
 * still hard-fails the gate; remove an entry once its underlying violation is fixed.
 *
 * The `semio-framework-os` (OS HOST crate) block below is `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s
 * addition: exactly the 17 plugin crates census'd live in 📓️w0-d-sdk-surface.md §3.2 as depending on the
 * host crate, each verified with `grep -rl '^semio-framework-os[[:space:]]*=' ✏️s/🔌️plugins/**​/📦️packages/🦀️rust/Cargo.toml`
 * at seed time. This backlog is APA's own to shrink (its W3/W4 waves move the `semio_framework_os::*`
 * symbols these 17 crates use into the SDK's curated re-export surface and drop the host dep) — it must
 * only ever shrink from here; never add a NEW plugin to this list to silence a fresh violation. */
const KNOWN_CAPABILITY_VIOLATIONS = new Set<string>([
  "semio-s-plugin-writer: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-procedural: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-gis: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-demonstrator: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-process: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-layout: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-cad: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-shooting: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-animate: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-lowpoly: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-remodel: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-note: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-trinity: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-draw: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-raster: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-puzzle: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  "semio-s-plugin-space: forbidden dependency semio-framework-os", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
  // 🚪️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`: predates the std::env/std::process addition — puzzle's
  // build.rs already used std::fs (read_dir/copy/write) under the OLD std::fs/std::net-only check, undeclared
  // (no localBackboneStorage capability), so this was already a live gate failure before this wave touched
  // anything. Seeded here rather than left as an unexplained new-looking regression once std::env joined the
  // same check (build.rs also reads CARGO_MANIFEST_DIR/OUT_DIR via std::env::var).
  "semio-s-plugin-puzzle: uses std::fs/std::net/std::env/std::process without localBackboneStorage capability (✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs)",
]);

class PluginCapabilityLintScript extends BundleScript {
  async run(): Promise<void> {
    const metadataResult = runProbe("cargo", ["metadata", "--format-version", "1", "--no-deps"], { cwd: repoRoot, budgetMs: buildBudgetMs() });
    if (metadataResult.status !== 0) {
      throw new Error(metadataResult.stderr || "cargo metadata failed");
    }
    const metadata = JSON.parse(metadataResult.stdout || "{}") as {
      packages: Array<{
        name: string;
        manifest_path: string;
        dependencies: Array<{ name: string }>;
      }>;
    };
    const registryEntries = generatePluginRegistry(repoRoot);
    const pluginPackageNames = new Map(registryEntries.map((entry) => [entry.packageName, entry.pluginId]));
    const depRules: Record<string, string> = {
      rusqlite: "localBackboneStorage",
      libloading: "forbidden",
      reqwest: "forbidden",
      "web-sys": "forbidden",
      "js-sys": "forbidden",
      egui: "forbidden",
      eframe: "forbidden",
      wgpu: "forbidden",
      "wgpu-core": "forbidden",
      winit: "forbidden",
      // 🚪️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`: the OS HOST crate. A plugin/extension may
      // depend on the plugin SDK, never on the host that hosts it — see 📓️w0-d-sdk-surface.md §3.2 for
      // the current 17-crate backlog, seeded below in KNOWN_CAPABILITY_VIOLATIONS.
      "semio-framework-os": "forbidden",
    };
    // 🕵️ Real registry membership, not a path substring: the pre-emoji-rename
    // `/plugin/rs/Cargo.toml` substring this used to filter on matches zero packages under
    // today's `✏️s/🔌️plugins/**/📦️packages/🦀️rust/Cargo.toml` layout, so every plugin
    // silently skipped this lint entirely — revived in ticket
    // 26/08/05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL.
    let checkedPackageCount = 0;
    const failures: string[] = [];
    for (const pkg of metadata.packages) {
      if (!pluginPackageNames.has(pkg.name)) continue;
      checkedPackageCount++;
      const manifestText = await Bun.file(pkg.manifest_path).text();
      const declared = new Set<string>();
      const metaMatch = manifestText.match(/\[package\.metadata\.semio\][\s\S]*?capabilities\s*=\s*\[([^\]]*)\]/);
      if (metaMatch?.[1]) {
        for (const entry of metaMatch[1].match(/"([^"]+)"/g) ?? []) {
          declared.add(entry.slice(1, -1));
        }
      }
      if (manifestText.includes("local_backbone_storage()") || manifestText.includes("ArtifactKind::Backbone")) {
        declared.add("localBackboneStorage");
      }
      const depNames = new Set(pkg.dependencies.map((dep) => dep.name));
      for (const dep of pkg.dependencies) {
        const otherPluginId = pluginPackageNames.get(dep.name);
        if (otherPluginId && dep.name !== pkg.name) {
          failures.push(`${pkg.name}: cross-plugin dependency on ${dep.name} (${otherPluginId})`);
        }
      }
      for (const [dep, rule] of Object.entries(depRules)) {
        if (!depNames.has(dep)) continue;
        if (rule === "forbidden") {
          failures.push(`${pkg.name}: forbidden dependency ${dep}`);
          continue;
        }
        if (!declared.has(rule)) {
          failures.push(`${pkg.name}: dependency ${dep} requires capability ${rule}`);
        }
      }
      const rustSources: string[] = [];
      walkRustSources(dirname(pkg.manifest_path), rustSources);
      for (const sourcePath of rustSources) {
        const source = await Bun.file(sourcePath).text();
        // 🚪️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`: std::env/std::process joined std::fs/std::net
        // on the same footing — all four are raw-ambient-authority escape hatches a sandboxed plugin
        // must not reach for directly; all four are gated by the same localBackboneStorage capability.
        if (/std::fs::|std::net::|std::env::|std::process::/.test(source) && !declared.has("localBackboneStorage")) {
          failures.push(`${pkg.name}: uses std::fs/std::net/std::env/std::process without localBackboneStorage capability (${relative(repoRoot, sourcePath)})`);
        }
        for (const symbol of PLUGIN_HOST_MODE_SYMBOLS) {
          if (!source.includes(symbol)) continue;
          failures.push(`${pkg.name}: program source references host-mode symbol ${symbol} (${relative(repoRoot, sourcePath)})`);
        }
      }
    }
    const grandfathered = failures.filter((f) => KNOWN_CAPABILITY_VIOLATIONS.has(f));
    const blocking = [...failures.filter((f) => !KNOWN_CAPABILITY_VIOLATIONS.has(f)), ...(await checkPlaygroundAliasFreshness())];
    for (const warning of grandfathered) console.warn(`[plugin-capability-lint] WARN (grandfathered, see spawned fix-it task): ${warning}`);
    if (blocking.length > 0) {
      for (const failure of blocking) console.error(`[plugin-capability-lint] ${failure}`);
      throw new Error(`plugin capability lint failed (${blocking.length} issue(s), ${checkedPackageCount} plugin package(s) evaluated)`);
    }
    console.log(`program capability lint passed (${checkedPackageCount} plugin package(s) evaluated, ${grandfathered.length} grandfathered warning(s))`);
  }
}

//#region 🔖️CapabilityLayeringLint
/** 🗺️ The four layering roles this lint enforces — a subset of `🔣️taxonomy.json`'s `roles` (which also
 * lists `product`/`hub`/`testkit`/`tool`, deliberately out of scope: this lint mirrors exactly the three
 * directions `.dependency-cruiser.cjs`'s `framework-no-s`/`s-modules-no-plugins`/`no-plugin-to-extension-*`
 * already enforce on the TS/JS import graph, not a broader Cargo policy). */
type LayeringRole = "framework" | "s-module" | "plugin" | "extension";
const LAYERING_ROLES = new Set<string>(["framework", "s-module", "plugin", "extension"]);

/** 🚧️ Grandfathered layering violations — real, evidence-backed, and deliberately accepted, NOT
 * pre-existing noise like `KNOWN_CAPABILITY_VIOLATIONS` above. This ticket's one populated entry is C2
 * from `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` (see `📓️w5b-c2-verdict.md`):
 * `semio-s-plugin-procedural` has 7 real Cargo dependencies on `🌊️flow`'s extension crates, and unlinking
 * them needs new runtime infrastructure that does not exist yet (a host-side extension registry wired
 * into a real boot path, guest-side component-extension wiring for all 7 crates, and a resolution for the
 * shared brep-kernel `GeometryHandle` coupling) — tracked as a dedicated follow-up ticket, not mechanical
 * cleanup. Do not add an entry here to silence a failure without the same standard of evidence — every
 * other hit this lint finds is a REAL new violation, not noise.
 *
 * The second entry is `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s: `semio-framework-os-renderer-wgpu`
 * (role `framework`) has a live Cargo dependency on `semio-s-plugin-puzzle` (role `plugin`) — declared at
 * `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml:30`
 * (`puzzle = { path = "…", package = "semio-s-plugin-puzzle" }`). No `puzzle::` call site was found in the
 * wgpu renderer's own sources when this was triaged, so the edge reads as dead/vestigial rather than a
 * real runtime coupling — but this lint deliberately does not try to prove "unused" from source text (a
 * feature-gated or macro-expanded call site would false-negative that check), so it stays a real, accepted
 * exception, not a false positive silenced away. **The real fix is deleting the unused Cargo dependency
 * line from the wgpu renderer's `Cargo.toml`** — that file and `puzzle` are both outside this ticket's
 * boundary (puzzle is held by another concurrent session per `📌️important.md`'s cross-session protocol),
 * so APA does not touch either; this entry keeps the gate green until whoever owns that boundary removes
 * the dependency, at which point this entry should be deleted, not left stale. */
const KNOWN_LAYERING_VIOLATIONS = new Set<string>([
  ...["brep", "math", "primitive", "logic", "dictionary", "list", "text"].map((ext) => `semio-s-plugin-procedural: plugin->extension dependency on semio-s-plugin-flow-extension-${ext}`),
  "semio-framework-os-renderer-wgpu: framework->plugin dependency on semio-s-plugin-puzzle", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
]);

/** 🧱️ `[package.metadata.semio]`'s `role` key, read the same way `PluginCapabilityLintScript`'s own
 * `[package.metadata.semio]` regex above reads `capabilities` — a plain-text scrape, not a TOML parser
 * dependency, for one field. `🔣️taxonomy.json`'s `ecosystems.🦀️rust.marker` documents this table as the
 * SSOT for a crate's role (see `📓️w6-investigation.md`, which used this exact table to settle a real-vs-
 * optics layering question about `semio-framework-os-kernel`). */
function extractSemioRole(manifestText: string): LayeringRole | null {
  const role = manifestText.match(/\[package\.metadata\.semio\][\s\S]*?\brole\s*=\s*"([^"]+)"/)?.[1];
  return role && LAYERING_ROLES.has(role) ? (role as LayeringRole) : null;
}

/** 🧱️ Cargo-metadata-driven counterpart to `.dependency-cruiser.cjs`'s `framework-no-s`/
 * `s-modules-no-plugins`/`no-plugin-to-extension-*` rules: those see only the TS/JS import graph (`compose
 * 🧰️framework ✏️s 🌎️hub ♻️mit-bestand`), so a real *Cargo* dependency edge violating the same three
 * directions (framework→{s-module,plugin,extension}, s-module→{plugin,extension}, plugin→extension) is
 * invisible to them — this is exactly how C2 (`🌀️procedural`→7 `🌊️flow` extension crates) went
 * undetected. Classifies every workspace crate by its own declared `[package.metadata.semio].role` (SSOT,
 * never a directory-path guess) and walks `cargo metadata`'s real dependency edges, `kind: null` (normal/
 * runtime) only — `dev`/`build` edges are test/build-time-only and not a real production coupling (mirrors
 * this file's own `dsl-fixture-sweep`-style "test-only harness, not a runtime violation" precedent). W7 of
 * `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`. Was deliberately NOT wired into `plugin lint`/the
 * root `verify gate` because a dry run surfaced one real, undocumented `framework->plugin` edge
 * (`semio-framework-os-renderer-wgpu` → `semio-s-plugin-puzzle`) nobody had yet evaluated. That finding is
 * now triaged and grandfathered in `KNOWN_LAYERING_VIOLATIONS` above (`26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`),
 * so this now runs as part of `plugin lint` (see the `"plugin"`/`"lint"` router entry below) — the same
 * gate `semio-framework-os-dev:plugin lint` already gets invoked by from the repo-root `verify gate`
 * (`📜️script.ts`), so wiring happens here rather than by touching that file. Still directly runnable
 * standalone too: `bun ./📜️script.ts layer-lint` from this package, or
 * `bun nx run @semio-tech/framework-os-dev:layer-lint`. */
class CapabilityLayeringLintScript extends BundleScript {
  async run(): Promise<void> {
    const metadataResult = runProbe("cargo", ["metadata", "--format-version", "1", "--no-deps"], { cwd: repoRoot, budgetMs: buildBudgetMs() });
    if (metadataResult.status !== 0) {
      throw new Error(metadataResult.stderr || "cargo metadata failed");
    }
    const metadata = JSON.parse(metadataResult.stdout || "{}") as {
      packages: Array<{ name: string; manifest_path: string; dependencies: Array<{ name: string; kind: string | null }> }>;
    };
    const roleByName = new Map<string, LayeringRole>();
    for (const pkg of metadata.packages) {
      const role = extractSemioRole(await Bun.file(pkg.manifest_path).text());
      if (role) roleByName.set(pkg.name, role);
    }
    const forbiddenTargets: Record<LayeringRole, LayeringRole[]> = {
      framework: ["s-module", "plugin", "extension"],
      "s-module": ["plugin", "extension"],
      plugin: ["extension"],
      extension: [],
    };
    const failures: string[] = [];
    let checkedEdgeCount = 0;
    for (const pkg of metadata.packages) {
      const fromRole = roleByName.get(pkg.name);
      if (!fromRole) continue;
      for (const dep of pkg.dependencies) {
        if (dep.kind !== null) continue; // 🕵️ dev/build deps are not a real runtime coupling
        const toRole = roleByName.get(dep.name);
        if (!toRole || dep.name === pkg.name) continue;
        checkedEdgeCount++;
        if (forbiddenTargets[fromRole].includes(toRole)) {
          failures.push(`${pkg.name}: ${fromRole}->${toRole} dependency on ${dep.name}`);
        }
      }
    }
    const grandfathered = failures.filter((f) => KNOWN_LAYERING_VIOLATIONS.has(f));
    const blocking = failures.filter((f) => !KNOWN_LAYERING_VIOLATIONS.has(f));
    for (const warning of grandfathered) console.warn(`[capability-layering-lint] WARN (grandfathered C2, see 📓️w5b-c2-verdict.md): ${warning}`);
    if (blocking.length > 0) {
      for (const failure of blocking) console.error(`[capability-layering-lint] ${failure}`);
      throw new Error(`capability layering lint failed (${blocking.length} issue(s), ${checkedEdgeCount} cross-role edge(s) evaluated)`);
    }
    console.log(`capability layering lint passed (${checkedEdgeCount} cross-role edge(s) evaluated, ${grandfathered.length} grandfathered warning(s))`);
  }
}
//#endregion 🔖️CapabilityLayeringLint

//#region 🔖️PluginIndexExportPathLint
/** 🕳️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s detector for a finding surfaced by
 * `26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY`: every `📦️index.ts` barrel under
 * `✏️s/🔌️plugins/<plugin>/📦️packages/🟦️typescript/` re-exports its `_snapshot`/`_diff`/`_mutations`
 * families as `export * as ... from "<relative path>"`, and the vast majority of those paths were
 * written against the pre-migration `🗿️artifacts/<a>/🧬️schema/…` tree — since migrated to
 * `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/…` — so they now resolve to nothing on disk. Reproduced live:
 * 517/567 relative specifiers dead across 33 plugins (worst: `📕️norm` 180/180, `🧱️block` 36/36,
 * `🧩️puzzle` 36/36).
 *
 * **Deliberately report-only, never wired into `verify`/`plugin lint`.** Unlike
 * `KNOWN_CAPABILITY_VIOLATIONS`/`KNOWN_LAYERING_VIOLATIONS` above (a *hard* gate with a hand-picked,
 * evidence-backed allowlist of pre-existing exceptions), 517 dead specifiers have no sane per-entry
 * grandfather list, and the actual fix — repointing every path at the migrated
 * `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/` shape — is explicitly out of this ticket's boundary (it would
 * mean editing `📦️index.ts`, forbidden here) and remains unowned. So `run()` below never throws: it
 * is only reachable via its own standalone `index-lint` router command / nx target, not folded into
 * any gate the way `layer-lint` was. */
const PLUGIN_BARREL_RELATIVE_EXPORT_PATTERN = /from\s+"(\.[^"]+)"/g;

/** 🧭️ Resolution order this lint checks a barrel's relative specifier against — literal path, then
 * `.ts`/`.tsx`, then a directory's `📦️index.ts`/`index.ts`. Matches how the rest of this toolchain
 * (bundler + `tsc`) would actually resolve the same specifier. */
function resolvesPluginBarrelExport(baseDir: string, spec: string): boolean {
  return [spec, `${spec}.ts`, `${spec}.tsx`, `${spec}/📦️index.ts`, `${spec}/index.ts`].some((candidate) => existsSync(join(baseDir, candidate)));
}

class PluginIndexExportPathLintScript extends BundleScript {
  async run(): Promise<void> {
    const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
    let totalDead = 0;
    let totalAll = 0;
    let pluginsWithDeadPaths = 0;
    for (const pluginId of readdirSync(pluginsRoot).sort()) {
      const indexPath = join(pluginsRoot, pluginId, "📦️packages/🟦️typescript/📦️index.ts");
      if (!existsSync(indexPath)) continue;
      const source = await Bun.file(indexPath).text();
      const baseDir = dirname(indexPath);
      const deadSpecs: string[] = [];
      let total = 0;
      for (const [, spec] of source.matchAll(PLUGIN_BARREL_RELATIVE_EXPORT_PATTERN)) {
        total++;
        if (!resolvesPluginBarrelExport(baseDir, spec)) deadSpecs.push(spec);
      }
      totalAll += total;
      totalDead += deadSpecs.length;
      if (deadSpecs.length === 0) continue;
      pluginsWithDeadPaths++;
      const cause = deadSpecs.some((s) => s.includes("🗿️artifacts/")) ? "likely pre-standards path (🗿️artifacts/<a>/🧬️schema/…) against the migrated 🏅️standards/🔖️<v>/🪆️subsets/✳️<s>/ tree" : "target does not exist on disk";
      console.warn(`[plugin-index-export-path-lint] WARN ${relative(repoRoot, indexPath)}: ${deadSpecs.length}/${total} relative export path(s) resolve to nothing (${cause})`);
    }
    console.log(`plugin index export path lint: ${totalDead}/${totalAll} dead relative export path(s) across ${pluginsWithDeadPaths} plugin(s) — REPORT ONLY, does not gate (26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE)`);
  }
}
//#endregion 🔖️PluginIndexExportPathLint

//#region 🔖️HostHandleReachLint
/** 🕳️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s detector for a violation
 * `PluginCapabilityLintScript`'s ambient-mutability rule structurally cannot see: that rule bans
 * item-scope `thread_local!`/`static mut`/`Mutex`/`RwLock`/`RefCell`/`Cell`/`Atomic*` but deliberately
 * exempts bare `OnceLock`/`OnceCell`/`LazyLock` as write-once-by-type — every artifact's
 * `io_registry` uses `static ENTRIES: OnceLock<Vec<ComposerEntry>>`, and flagging those would drown
 * the signal. But `OnceLock<Vec<ComposerEntry>>` (a plugin caching its own immutable data) and
 * `OnceLock<BrepEngineHost>` (a plugin holding a handle to HOST-owned engine state for the process
 * lifetime) are identical in mutability and entirely different in violation. It is not ambient
 * *mutability*, it is ambient **reach** — `OnceLock` only makes the handle unforgeable after init; it
 * does nothing about a plugin having one at all. So this is a distinct check, not a widened
 * mutability rule (which would only manufacture false positives against the sanctioned registry
 * tables).
 *
 * Confirmed live: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:91-94`
 * (`static HOST: OnceLock<BrepEngineHost>`, constructed via `get_or_init`) and
 * `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:403,415`
 * (`host: BrepEngineHost` struct field, `BrepEngineHost::new(64 * 1024 * 1024)`).
 *
 * **Deliberately report-only, never wired into `verify`/`plugin lint`**, same posture as
 * `PluginIndexExportPathLintScript` above: several sessions are actively running against the gate
 * and this rule will fire on plugins they own. Fixing what it finds is cross-session work (`process`
 * belongs to another session, `cad` to this ticket, and the host-handle model reaches `💻️os/🖥️host`) and is
 * deliberately not attempted here. */
const HOST_ENGINE_HANDLE_TYPES: Readonly<Record<string, string>> = {
  // 🧠️ Host-owned brep engine wrapper — wraps `Mutex<EngineCache>` (byte-budgeted, host-managed compute cache) plus `Mutex<Brep>` kernel
  // session (🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs). It is the current handle-type surface — a plugin holding one
  // reaches directly for host-managed compute-cache/dispatch state instead of going through the WIT
  // `engine-derive`/`engine-read` guest<->host boundary.
  BrepEngineHost: "host-owned brep engine wrapper (byte-budgeted engine-result cache + kernel session) — a process-lifetime handle to host-managed compute state, not a plugin's own data",
  // 🧠️ The host-owned LRU byte-budgeted engine-result cache `BrepEngineHost` wraps
  // (🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs, doc comment: "Host-owned LRU engine
  // result cache with a byte budget"). Holding one directly bypasses `BrepEngineHost`'s own wrapper but
  // reaches for the identical host-managed caching/dispatch authority.
  EngineCache: "host-owned LRU byte-budgeted engine-result cache underlying BrepEngineHost — same ambient reach as holding that wrapper directly, just unwrapped",
};
// 🕵️ Deliberately excludes plain opaque handle VALUES the framework explicitly documents as safe for a
// plugin to hold — `EngineHandle`/`EngineKey`/`GeometryHandle` are unforgeable content-addressed tokens
// ("plugins may store and read, never mint" per `EngineHandle`'s own doc comment), not connections to
// host-side state; and per-document domain models plugins legitimately own outright (`FlowHost`,
// `DagHost`, `GraphHost`, `MapHost`, `RasterHost`, `EditorHost`, `BoardHost` — all rebuilt fresh
// per-call/per-document from a fixture/snapshot, own no cache/arena/budget/connection to shared
// process-lifetime state) are excluded on the same "plain data, not a handle" ground. Also deliberately
// excludes the OS-crate host types (`ArtifactHost`, `SpaceHost`, `PluginHost`, `BackboneWorkerHost`,
// `WasmtimeNodeHost`) — those live inside `semio-framework-os`, already a blanket-forbidden dependency
// under `PluginCapabilityLintScript`'s `depRules`, so a plugin reaching them is already caught (coarser,
// but caught) there; adding them here would be redundant noise, not a new gap. And excludes bare
// `NativeHost`/`WasmHost`/`TestHost` — confirmed by inspection to be generic actor-model types plugins
// (`🖍️draw`, `🧩️puzzle`) *define locally themselves* inside their own `🔄️fsm`/`🌉️wasm` modules (a
// same-named but unrelated generic `Machine`-parameterized abstraction, not an OS host reference) — a
// bare name match on those would false-positive on legitimate plugin-owned code.
const HOST_ENGINE_HANDLE_TYPE_NAMES = Object.keys(HOST_ENGINE_HANDLE_TYPES);
const HOST_ENGINE_HANDLE_TYPE_ALTERNATION = HOST_ENGINE_HANDLE_TYPE_NAMES.join("|");

/** 🎯️ Rule 1 — a `static` of the handle type, any wrapper (`OnceLock`, `LazyLock`, `Mutex`, `RwLock`, or
 * none) per the ticket's explicit scope: unlike `PluginCapabilityLintScript`'s ambient-mutability rule,
 * `OnceLock`/`LazyLock` are NOT exempt here — the wrapper is irrelevant to ambient reach. */
const HOST_HANDLE_STATIC_PATTERN = new RegExp(`\\bstatic\\s+\\w+\\s*:\\s*(?:(?:OnceLock|LazyLock|Mutex|RwLock)\\s*<\\s*)?(${HOST_ENGINE_HANDLE_TYPE_ALTERNATION})\\s*>?`, "g");

/** 🎯️ Rule 2 — a struct field whose declared type names the handle type. The `(?!::)` guard excludes a
 * struct-LITERAL initializer line (`host: BrepEngineHost::new(...)`, rule 3's territory) from also
 * double-counting as a field-declaration hit — those two rules report the same physical violation from
 * two different source lines (the `pub struct` field decl vs. the `impl ... new()` initializer) in the
 * real `process3d` finding, and each should be counted once, not twice. */
const HOST_HANDLE_FIELD_PATTERN = new RegExp(`^\\s*(?:pub(?:\\([^)]*\\))?\\s+)?[a-z_][a-zA-Z0-9_]*\\s*:\\s*(?:(?:Option|Box|Arc|Mutex|RwLock|OnceLock|LazyLock)\\s*<\\s*)*(${HOST_ENGINE_HANDLE_TYPE_ALTERNATION})(?!::)\\b`, "gm");

/** 🎯️ Rule 3 — direct construction of the handle type. */
const HOST_HANDLE_CONSTRUCT_PATTERN = new RegExp(`\\b(${HOST_ENGINE_HANDLE_TYPE_ALTERNATION})::new\\s*\\(`, "g");

/** 🔢️ 1-based line number of `index` within `source`, for actionable breach messages. */
function lineNumberAtIndex(source: string, index: number): number {
  let line = 1;
  for (let i = 0; i < index; i++) if (source.charCodeAt(i) === 10) line++;
  return line;
}

type HostHandleBreach = { readonly relPath: string; readonly line: number; readonly rule: "static" | "field" | "construct"; readonly handleType: string };

/** 🕵️ Scans one Rust source file's text for all three rule sites, tagging each with its 1-based line. */
function scanHostHandleReach(relPath: string, source: string): HostHandleBreach[] {
  const breaches: HostHandleBreach[] = [];
  for (const match of source.matchAll(HOST_HANDLE_STATIC_PATTERN)) {
    breaches.push({ relPath, line: lineNumberAtIndex(source, match.index!), rule: "static", handleType: match[1]! });
  }
  for (const match of source.matchAll(HOST_HANDLE_FIELD_PATTERN)) {
    breaches.push({ relPath, line: lineNumberAtIndex(source, match.index!), rule: "field", handleType: match[1]! });
  }
  for (const match of source.matchAll(HOST_HANDLE_CONSTRUCT_PATTERN)) {
    breaches.push({ relPath, line: lineNumberAtIndex(source, match.index!), rule: "construct", handleType: match[1]! });
  }
  return breaches;
}

class HostHandleReachLintScript extends BundleScript {
  async run(): Promise<void> {
    const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
    let totalBreaches = 0;
    let pluginsWithBreaches = 0;
    for (const entry of readdirSync(pluginsRoot, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      if (!entry.isDirectory()) continue;
      const pluginId = entry.name;
      const pluginDir = join(pluginsRoot, pluginId);
      const rustSources: string[] = [];
      walkRustSources(pluginDir, rustSources);
      const pluginBreaches: HostHandleBreach[] = [];
      for (const sourcePath of rustSources) {
        const source = await Bun.file(sourcePath).text();
        pluginBreaches.push(...scanHostHandleReach(relative(repoRoot, sourcePath), source));
      }
      if (pluginBreaches.length === 0) continue;
      pluginsWithBreaches++;
      totalBreaches += pluginBreaches.length;
      for (const breach of pluginBreaches) {
        const why = HOST_ENGINE_HANDLE_TYPES[breach.handleType];
        const site = breach.rule === "static" ? "static" : breach.rule === "field" ? "struct field" : "direct construction";
        console.warn(
          `[host-handle-reach-lint] WARN ${pluginId}: ${breach.relPath}:${breach.line}: ${site} of handle type ${breach.handleType} — ${why} (ambient REACH into host-owned state, not ambient mutability — a wrapping OnceLock/LazyLock only makes the handle unforgeable after init, it does not gate having one at all)`,
        );
      }
    }
    console.log(
      `host handle reach lint: ${totalBreaches} breach site(s) across ${pluginsWithBreaches} plugin(s) — REPORT ONLY, does not gate (26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE); fixing is cross-session work (process3d is another session's, cad is this ticket's, the trait model reaches 💻️os/🖥️host) and is deliberately not attempted here`,
    );
  }
}
//#endregion 🔖️HostHandleReachLint

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

//#region 🔖️SpaceE2eVerify
/** 🎭️ Playwright end-to-end workflow verification for the `s` studio shell (folded in from the former `.🦑️repo/🎫️tickets/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`). */
const STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS = ["NoCompatibleDevice"];

function spaceE2eAssert(condition: boolean, message: string): void {
  if (!condition) throw new Error(message);
}

function isIgnorableStudioE2ePageError(message: string): boolean {
  return STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS.some((fragment) => message.includes(fragment));
}

async function waitForStudioE2eCondition(page: import("playwright").Page, predicate: (state: { text: string; children: number }) => boolean, label: string, deadline: number): Promise<{ text: string; children: number }> {
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const children = await page.locator("#root *").count();
    if (predicate({ text, children })) return { text, children };
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for ${label}`);
}

async function openStudioE2e(page: import("playwright").Page, deadline: number): Promise<{ text: string; children: number }> {
  await page.keyboard.press("Meta+n");
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const path = await page.evaluate(() => location.pathname);
    const children = await page.locator("#root *").count();
    if (/Catalogue/i.test(text) && /Parameters/i.test(text) && path.startsWith("/spaces/")) {
      return { text, children };
    }
    await page.waitForTimeout(500);
  }
  throw new Error("timeout waiting for studio workspace");
}

async function activateStudioE2eWorkflowWindow(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(200);
}

async function expandStudioE2eWorkflowEngagement(page: import("playwright").Page): Promise<void> {
  await activateStudioE2eWorkflowWindow(page);
  await page.evaluate(() => document.getElementById("framework.window.sWorkflow.search.toggle")?.click());
  await page.waitForSelector("#s-media-catalogue-hint", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromEngagement(page: import("playwright").Page): Promise<string> {
  await expandStudioE2eWorkflowEngagement(page);
  const engagementInput = page.locator("#s-media-catalogue-hint");
  await engagementInput.fill("draw draw");
  await engagementInput.press("Enter");
  await page.waitForTimeout(1500);
  return "engagement";
}

async function openStudioE2eCommandPalette(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(100);
  await page.keyboard.press("Meta+p");
  await page.waitForSelector("[role='dialog'] [data-slot='command-input']", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromPalette(page: import("playwright").Page): Promise<string | null> {
  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("draw");
  await page.waitForTimeout(400);
  const drawSpawn = page
    .locator('[data-slot="command-item"]')
    .filter({ hasText: /Spawn Draw/i })
    .first();
  if (await drawSpawn.count()) {
    await drawSpawn.click();
    return "palette";
  }
  await page.keyboard.press("Escape");
  return null;
}

async function runStudioE2eVerify(baseUrl: string, timeoutMs: number): Promise<void> {
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const pageErrors: string[] = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));

  console.log(`navigating to ${baseUrl}`);
  await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.waitForFunction(() => /home/i.test(document.body.innerText) && /Demo Studio|New Studio/i.test(document.body.innerText) && document.querySelectorAll("#root *").length > 150, { timeout: 120_000 });

  const deadline = Date.now() + timeoutMs;
  const booted = await waitForStudioE2eCondition(page, ({ text }) => /Home/i.test(text) && /Studios|Search/i.test(text) && /Demo Studio|New Studio/i.test(text), "home shell with studios", deadline);
  console.log(`home loaded (${booted.children} nodes)`);
  spaceE2eAssert(/Demo Studio|Studios/i.test(booted.text), "home studios vfs should list seeded studio");

  await openStudioE2e(page, deadline);
  const pathAfterCreate = await page.evaluate(() => location.pathname);
  console.log(`studio loaded at ${pathAfterCreate}`);
  spaceE2eAssert(pathAfterCreate.startsWith("/spaces/"), "studio uri should be under /spaces/");

  await page.waitForFunction(() => document.querySelector(".semio-node-graph-host") != null, { timeout: 30_000 });

  const bodyText = await page.locator("body").innerText();
  spaceE2eAssert(!/Missing window:/i.test(bodyText), "all studio windows should render");
  spaceE2eAssert((await page.locator(".semio-node-graph-host").count()) > 0, "node graph host should render");
  spaceE2eAssert((await page.locator(".semio-text-editor-host").count()) > 0, "compiled dag editor should render");
  console.log("three studio windows rendered");

  let spawnMode: string | null = null;
  try {
    spawnMode = await spawnStudioE2eDrawFromEngagement(page);
    console.log(`spawn via ${spawnMode}`);
  } catch {
    spawnMode = await spawnStudioE2eDrawFromPalette(page);
    spaceE2eAssert(spawnMode === "palette", "draw spawn should work via engagement rail or command palette");
    console.log(`spawn via ${spawnMode}`);
  }

  await page.keyboard.press("Meta+z");
  await page.waitForTimeout(1500);
  console.log("undo issued");

  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("undo");
  await page.waitForTimeout(300);
  spaceE2eAssert((await page.locator('[data-slot="command-item"]').filter({ hasText: "Undo" }).count()) > 0, "undo should be in command palette");
  await paletteInput.fill("checkpoint");
  await page.waitForTimeout(300);
  spaceE2eAssert(
    (await page
      .locator('[data-slot="command-item"]')
      .filter({ hasText: /checkpoint/i })
      .count()) > 0,
    "checkpoint command should be in command palette",
  );
  console.log("studio commands in palette");
  await page.keyboard.press("Escape");

  await page.keyboard.press("Meta+f");
  await page.waitForTimeout(500);
  spaceE2eAssert((await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0, "find palette should open");
  console.log("find palette available");
  await page.keyboard.press("Escape");

  await page.getByRole("button", { name: "← Home" }).click({ force: true });
  await waitForStudioE2eCondition(page, ({ text }) => text.includes("Demo Studio") || text.includes("New Studio"), "home via studio bar", deadline);
  console.log("studio home bar navigation works");

  const demoStudioRow = page.locator('[data-row-id="studio:default"]');
  if (await demoStudioRow.count()) {
    await demoStudioRow.dblclick({ force: true });
    await page.waitForFunction(() => location.pathname.startsWith("/spaces/"), { timeout: 15_000 });
    await waitForStudioE2eCondition(page, ({ text }) => /Catalogue/i.test(text), "opened studio from home vfs", deadline);
    console.log("home vfs open studio works");
  }

  const criticalErrors = pageErrors.filter((message) => !isIgnorableStudioE2ePageError(message));
  if (criticalErrors.length !== pageErrors.length) {
    console.log(`ignored headless gpu errors: ${pageErrors.filter(isIgnorableStudioE2ePageError).join(" | ")}`);
  }
  spaceE2eAssert(criticalErrors.length === 0, `page errors: ${criticalErrors.join(" | ")}`);

  await browser.close();
  console.log("PASS: S studio end-to-end workflows verified");
}
//#endregion 🔖️SpaceE2eVerify

//#region 🔖️CollabE2e
/** 🤝️ Two-user hub+shell end-to-end collaboration proof — ticket
 * `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`, lane 3-C. Boots the real hub plus two
 * independent `s` react dev servers (one per user) and drives them as two separate Playwright browser
 * contexts through the ticket's whole collaboration story: space creation replication, sharing, artifact
 * creation replication, live co-editing, presence, check-in, admin visibility, and hub-restart
 * persistence. Every scenario step is reported individually (`STEP n: PASS/FAIL`) and the run continues
 * past a failing step where it safely can — see the ticket's worker-brief "Reality check" section for
 * why several steps are expected to hit real, still-open upstream gaps this lane does not own. */
const COLLAB_E2E_PORT_MIN = 7400;
const COLLAB_E2E_PORT_MAX = 7498;
const COLLAB_E2E_HUB_BOOT_BUDGET_MS = Number(process.env.COLLAB_E2E_HUB_BOOT_BUDGET_MS ?? 300_000);
const COLLAB_E2E_PREBUILD_BUDGET_MS = Number(process.env.COLLAB_E2E_PREBUILD_BUDGET_MS ?? 1_800_000);
const COLLAB_E2E_DEV_BOOT_BUDGET_MS = Number(process.env.COLLAB_E2E_DEV_BOOT_BUDGET_MS ?? 300_000);
const COLLAB_E2E_ADMIN_TOKEN = "e2e-admin";
const COLLAB_E2E_USER1_EMAIL = "user1@semio.dev";
const COLLAB_E2E_USER2_EMAIL = "user2@semio.dev";

const COLLAB_E2E_STEP_NAMES = [
  "user1 creates a public studio space from Home; user2's Home shows the same row",
  "user1 shares the space with user2 as author; user2 opens /spaces/{id}",
  "user1 creates a writer artifact; the row appears in both tables and opens an editor for user1",
  "user2 opens the same artifact; user1 types and user2 sees the text",
  "#s-presence-peers shows 2 peers in both shells",
  "user1 checks in with a message; history shows it and the space table's updated column moves for both",
  "admin: /admin/api/connections lists both connections with their surfaces; /admin returns HTML",
  "hub restarts against the same OS_HUB_DATA; user2 reloads and the space + artifact are still there",
] as const;

type CollabStepOutcome = { readonly step: number; readonly name: string; readonly pass: boolean; readonly detail: string };

/** 📁️ Ticket folder — scratch logs/screenshots for this lane's own probes, per the worker-brief. */
function collabOutDir(): string {
  const dir = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS");
  mkdirSync(dir, { recursive: true });
  return dir;
}

/** 🔌️ Resolves one collab-e2e port: an explicit env override, else the first free port in
 * `[COLLAB_E2E_PORT_MIN, COLLAB_E2E_PORT_MAX]` not already claimed by an earlier call this run. */
function collabScanPort(envVar: string, taken: Set<number>): number {
  const override = process.env[envVar];
  if (override) {
    const port = Number(override);
    taken.add(port);
    return port;
  }
  for (let port = COLLAB_E2E_PORT_MIN; port <= COLLAB_E2E_PORT_MAX; port++) {
    if (taken.has(port) || isDevPortInUse("127.0.0.1", port)) continue;
    taken.add(port);
    return port;
  }
  throw new Error(`collab e2e: no free port in ${COLLAB_E2E_PORT_MIN}-${COLLAB_E2E_PORT_MAX} for ${envVar}`);
}

/** 🚀️ Spawns the real hub (`bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts dev`, i.e. `cargo run` against the
 * default (sqlite) feature set — never `--all-features`, contract-freeze Amendment 2) on `port` against
 * a fresh `dataDir`, and waits for a real HTTP response before returning. */
async function collabStartHub(port: number, dataDir: string, logPath: string): Promise<SpawnDaemonHandle> {
  const hubScript = join(repoRoot, "./🌎️hub/📦️packages/🦀️rust/📜️script.ts");
  const logStream = createWriteStream(logPath);
  const daemon = spawnDaemon("bun", [hubScript, "dev"], {
    cwd: join(repoRoot, "./🌎️hub/📦️packages/🦀️rust"),
    env: { ...process.env, OS_HUB_PORT: String(port), OS_HUB_DATA: dataDir, OS_HUB_ADMIN_TOKEN: COLLAB_E2E_ADMIN_TOKEN },
    stdio: "pipe",
  });
  daemon.child.stdout?.pipe(logStream);
  daemon.child.stderr?.pipe(logStream);
  const baseUrl = `http://127.0.0.1:${port}`;
  const outcome = await awaitHttpOk(`${baseUrl}/admin/api/overview`, {
    deadlineMs: COLLAB_E2E_HUB_BOOT_BUDGET_MS,
    intervalMs: 500,
    init: { headers: { authorization: `Bearer ${COLLAB_E2E_ADMIN_TOKEN}` } },
    isDead: () => daemon.child.exitCode !== null,
  });
  if (outcome === "ready") return daemon;
  if (outcome === "dead") throw new Error(`hub exited early (code ${daemon.child.exitCode}) — see ${logPath}`);
  daemon.kill();
  logStream.end();
  throw new Error(`hub did not become ready on port ${port} within ${COLLAB_E2E_HUB_BOOT_BUDGET_MS}ms — see ${logPath}`);
}

/** 🎯️ The ONLY plugin crates this scenario touches: `"s"` is the space plugin's own registry
 * `pluginId` (verified: `🤖️generated/🟦️playgrounds.ts`'s `variant: "s"` row carries `pluginId: "s"`,
 * NOT `"space"` — it hosts both the Home and Space apps and is host-first in registry order) and
 * `"writer"` is the stdio-free artifact kind this scenario creates (the brief's other suggestion,
 * `"note"`, is a confirmed pre-existing break — see `collabPrebuildPlugins`'s own doc comment). Building
 * only these two (not the full ~58-crate catalog `buildPluginsStreaming("s")` would otherwise attempt)
 * turns a 20-40 minute run into a sub-minute one and matches the coordinator's own guidance: build just
 * what the scenario needs, per-crate try/catch, then gate on the artifacts actually existing. */
const COLLAB_E2E_REQUIRED_PLUGIN_IDS: readonly string[] = ["s", "writer"];

/** 📁️ The exact `.core.wasm` path `buildPlugin` (this same file, `🔖️PluginSizeMeasurement` region's
 * neighbor) writes for `target` — mirrors its own `jsBase`/`componentBase` derivation so this check
 * looks for precisely what a successful build would have produced, not a guess. */
function collabPluginArtifactPath(target: PluginRegistryEntry): string {
  const jsBase = target.wasmOut.replace(/\.wasm$/, "");
  return join(pluginOutRoot, target.pluginId, `${jsBase}_component.core.wasm`);
}

/** 🧱️ Builds ONLY the plugin crates `COLLAB_E2E_REQUIRED_PLUGIN_IDS` needs, once, in-process — still
 * reuses the SAME `PluginBuildLease` a real `dev` process would (mutual exclusion against a peer
 * session's own `bun dev s`), still per-crate try/catch (continues past one target's failure to attempt
 * the other — matters when, as observed, `"s"` itself fails: without the try/catch `"writer"` would
 * never even get attempted). `preparePluginBuildTargets("s")` still does the necessary prep (registry
 * regen, wasm target ensure, shim vendor, stale-output cleanup) `buildPlugin` depends on — only the
 * ITERATION is narrowed from "all ~58 catalog entries" to just the two this scenario touches; unlike the
 * catalog-wide `buildPluginsStreaming`/non-streaming `buildPlugins`, this never even attempts `animate`/
 * `gis`/`draw`/`fem`/etc., so their many pre-existing, unrelated breakages (confirmed via one full
 * catalog-wide run during this lane's own iteration, `🧪️3-c-collab-e2e-run2.txt`) cost nothing here.
 *
 * **Hard gate, per the coordinator's explicit instruction**: after building, this asserts BOTH required
 * artifacts exist on disk. If `"s"` (the space plugin — Home AND Space apps) is missing, that is a
 * genuine, scenario-fatal blocker (neither app can load in a browser) and this throws with the real
 * compiler error already printed above by the per-target `catch`, never silently continuing to start a
 * browser against a build that cannot possibly serve anything. Confirmed (this lane, this session, via
 * `cargo check -p semio-s-plugin-space --target wasm32-wasip2`, reproduced standalone, see
 * `🧪️3-c-cargo-check-space-wasm.txt`): `semio-s-plugin-space`'s own Cargo.toml requests
 * `semio-framework-os = { features = ["os-host-full"] }` UNCONDITIONALLY (line 37, unchanged since
 * commit `19b970280` 2026-08-11 — `git diff HEAD` on that file shows only lane 1-F's later, unrelated
 * `user_ports` addition, so this is NOT something this ticket introduced); `os-host-full` (`🖥️host/
 * 📦️packages/🦀️rust/Cargo.toml:62`) turns on `semio-framework-os-kernel/sync`, which turns on
 * `tokio/net` (`sync = [..., "tokio/net", ...]`), and `tokio/net` is not one of the five features
 * (`sync,macros,io-util,rt,time`) tokio 1.52.3 supports on any wasm target — `cargo tree -e features -p
 * semio-s-plugin-space --target wasm32-wasip2 -i tokio` traces the exact edge. A host-only capability
 * feature is being requested by a wasm GUEST plugin crate, unconditionally — this is a pre-existing,
 * standing architecture gap in `semio-s-plugin-space`'s own dependency declaration, outside this lane's
 * lease (`🧑️‍💻️dev/📦️packages/🟦️typescript/{📜️script.ts,⚙️vite.config.ts}` + `project.json` only) and
 * squarely lanes 1-E/2-A/2-B's crate, not touched here.
 *
 * `FLOW_CORE_SKIP_WASM_BUILD=1` (flow-core's own pre-existing escape hatch, `🌊️flow/🫀️core/📦️packages/
 * 🦀️rust/📜️script.ts`'s `WasmScript`) is set here, defaulted only — a SEPARATE, real, confirmed,
 * pre-existing, unrelated defect blocks that ONE wasm build too: `semio-framework-os-flow`'s Cargo.toml
 * depends on `semio-framework-ui` with `features = ["wgpu", "wgpu-engine"]`, which pulls in `wgpu`/
 * `vello_encoding`/`hayro-interpret`, which pull `getrandom` 0.3.4 — nothing in that graph enables
 * `getrandom`'s own `wasm_js` Cargo feature for the `wasm32-unknown-unknown` target (`.cargo/config.toml`
 * sets the `--cfg getrandom_backend="wasm_js"` compiler flag, but that alone is not enough; getrandom 0.3
 * also needs the crate-level feature). Confirmed via `git status`/`git log --date=iso` that none of
 * `🌊️flow/**`, `◻2d/**`, `🖱️ui/**` Cargo.toml files are mid-edit right now, so this is standing too, not
 * transient churn — outside this lane's lease and outside `🌊️flow`/`🖱️ui`'s wgpu-NATIVE-renderer
 * breakage the coordinator already flagged as "irrelevant to you" (that one is the native wgpu target;
 * this is the WASM build the REACT renderer needs). Safe to skip for this scenario specifically: the
 * React renderer's own import of `@semio-tech/flow-core` (`buildEngineWasm`'s own comment) is a lazy
 * `import("@semio-tech/flow-core")` inside `createFlowSession`, never called by Home/Space/Writer — so a
 * missing `flow_core_bg.wasm` is inert for every step this harness exercises. Flow/DAG functionality
 * itself stays unverified and broken; flagged precisely here, never silently masked. */
async function collabPrebuildPlugins(): Promise<void> {
  ensureAppleDeveloperDir();
  process.env.FLOW_CORE_SKIP_WASM_BUILD = process.env.FLOW_CORE_SKIP_WASM_BUILD ?? "1";
  const lease = acquirePluginBuildLease("s", 0);
  if (lease.role === "follower") {
    console.log(`[collab-e2e] plugin builds owned by pid ${lease.lease.pid}; waiting for ready`);
    await waitForPluginBuildLeaseReady("s", COLLAB_E2E_PREBUILD_BUDGET_MS);
  } else {
    try {
      await ensurePluginRegistry("s");
      await buildEngineWasm("s", "react");
      const targets = await preparePluginBuildTargets("s");
      const required = targets.filter((target) => COLLAB_E2E_REQUIRED_PLUGIN_IDS.includes(target.pluginId));
      const foundIds = new Set(required.map((target) => target.pluginId));
      for (const pluginId of COLLAB_E2E_REQUIRED_PLUGIN_IDS) {
        if (!foundIds.has(pluginId)) console.error(`[collab-e2e] WARNING: no registry entry for required plugin "${pluginId}" at all — catalog may have changed`);
      }
      for (const target of required) {
        try {
          await buildPlugin(target);
        } catch (error) {
          console.error(`[collab-e2e] required plugin build failed: ${target.pluginId}`, error);
        }
      }
      markPluginBuildLeaseReady("s");
    } finally {
      releasePluginBuildLease("s");
    }
  }
  const targets = await preparePluginBuildTargets("s");
  const missing: string[] = [];
  for (const pluginId of COLLAB_E2E_REQUIRED_PLUGIN_IDS) {
    const target = targets.find((entry) => entry.pluginId === pluginId);
    if (!target || !existsSync(collabPluginArtifactPath(target))) missing.push(pluginId);
  }
  if (missing.length > 0) {
    throw new Error(`collab e2e: required plugin wasm artifact(s) missing after build: ${missing.join(", ")} — see the compiler error printed above (search "required plugin build failed: ${missing[0]}")`);
  }
}

/** ▶️ Spawns one user's `s` react dev server (`SKIP_PLUGIN_BUILD=1` — never touches cargo, only serves
 * whatever `collabPrebuildPlugins` already produced) and waits for its port to accept connections. */
async function collabStartUserDevServer(opts: { readonly port: number; readonly hubUrl: string; readonly user: string; readonly dataDir: string; readonly logPath: string }): Promise<SpawnDaemonHandle> {
  const devScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts");
  const logStream = createWriteStream(opts.logPath);
  const daemon = spawnDaemon("bun", [devScript, "dev"], {
    cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript"),
    env: { ...process.env, SKIP_PLUGIN_BUILD: "1", SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react", S_OS_PORT: String(opts.port), S_HUB_URL: opts.hubUrl, S_USER: opts.user, S_DATA_DIR: opts.dataDir },
    stdio: "pipe",
  });
  daemon.child.stdout?.pipe(logStream);
  daemon.child.stderr?.pipe(logStream);
  const outcome = await awaitTcpReady("127.0.0.1", opts.port, {
    deadlineMs: COLLAB_E2E_DEV_BOOT_BUDGET_MS,
    intervalMs: 500,
    isDead: () => daemon.child.exitCode !== null,
  });
  if (outcome === "ready") return daemon;
  if (outcome === "dead") throw new Error(`dev server for ${opts.user} exited early (code ${daemon.child.exitCode}) — see ${opts.logPath}`);
  daemon.kill();
  logStream.end();
  throw new Error(`dev server for ${opts.user} did not open port ${opts.port} within ${COLLAB_E2E_DEV_BOOT_BUDGET_MS}ms — see ${opts.logPath}`);
}

//#region 🔖️CollabE2eDom
/** 🕹️ Clicks a shell-frozen toolbar-button id (contract §C0: `#s-home-create-space`,
 * `#s-space-create-artifact`) directly — lane 4-F wired these as real, always-present `UiNode::Button`
 * elements above their respective tables (dispatching with no args, which each command's own handler
 * treats as "open the dialog"), replacing the earlier command-palette hunt this harness used before
 * that landed: the palette's arg-carrying-command path opens the bottom-middle command PANEL form, not
 * a `[data-slot="dialog-box"]` modal, so it could never have satisfied `collabWaitForDialog` anyway. */
async function collabClickToolbarButton(page: import("playwright").Page, elementId: string): Promise<void> {
  const button = page.locator(`[id="${elementId}"]`);
  spaceE2eAssert((await button.count()) > 0, `toolbar button #${elementId} does not exist`);
  await button.click();
}

async function collabWaitForDialog(page: import("playwright").Page): Promise<void> {
  await page.locator('[data-slot="dialog-box"]').waitFor({ state: "visible", timeout: 15_000 });
}

async function collabSubmitDialog(page: import("playwright").Page): Promise<void> {
  await page.locator('[id="ui.dialog.submit"]').click();
  await page.locator('[data-slot="dialog-box"]').waitFor({ state: "hidden", timeout: 15_000 });
}

/** 🕹️ Opens a `<Select id={triggerId}>` (Radix, portal-rendered) and clicks the option with `optionText`. */
async function collabSelectOption(page: import("playwright").Page, triggerId: string, optionText: string): Promise<void> {
  await page.locator(`#${triggerId}`).click();
  await page.getByRole("option", { name: optionText, exact: true }).click();
  await page.waitForTimeout(150);
}

async function collabRowIds(page: import("playwright").Page, prefix: "space" | "artifact"): Promise<Set<string>> {
  const ids = await page.locator(`[data-row-id^="${prefix}:"]`).evaluateAll((elements) => elements.map((element) => element.getAttribute("data-row-id") ?? ""));
  return new Set(ids);
}

/** ⏳️ Polls `page` until a `data-row-id` with `prefix` appears that was not in `before`, returning the
 * bare id (prefix stripped). Used for both same-page ("the row appears") and cross-page ("user2 sees the
 * same row") assertions — the caller decides which page to poll. */
async function collabWaitForNewRow(page: import("playwright").Page, prefix: "space" | "artifact", before: ReadonlySet<string>, deadlineMs: number): Promise<string> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const current = await collabRowIds(page, prefix);
    for (const id of current) {
      if (!before.has(id)) return id.slice(prefix.length + 1);
    }
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for a new ${prefix}: row`);
}

async function collabWaitForRow(page: import("playwright").Page, prefix: "space" | "artifact", id: string, deadlineMs: number): Promise<void> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    if ((await page.locator(`[data-row-id="${prefix}:${id}"]`).count()) > 0) return;
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for [data-row-id="${prefix}:${id}"]`);
}

async function collabScreenshot(page: import("playwright").Page, label: string): Promise<void> {
  try {
    await page.screenshot({ path: join(collabOutDir(), `🧪️3-c-${label}.png`) });
  } catch {
    // 🏁️ Best-effort — a screenshot failure must never mask the real assertion failure it was taken for.
  }
}
//#endregion 🔖️CollabE2eDom

/** 🎬️ The whole 8-step scenario, run against two already-booted `s` react dev servers and a live hub.
 * Each step is wrapped so a failure is recorded and the run continues to the next step wherever the
 * remaining steps can still be meaningfully attempted. */
async function collabRunScenario(
  user1: import("playwright").Page,
  user2: import("playwright").Page,
  hubBaseUrl: string,
): Promise<{ readonly results: CollabStepOutcome[]; readonly spaceId: string | undefined; readonly artifactId: string | undefined }> {
  const results: CollabStepOutcome[] = [];
  const record = (step: number, pass: boolean, detail: string): void => {
    results.push({ step, name: COLLAB_E2E_STEP_NAMES[step - 1]!, pass, detail });
    console.log(`STEP ${step}: ${pass ? "PASS" : "FAIL"}: ${COLLAB_E2E_STEP_NAMES[step - 1]} — ${detail}`);
  };

  let spaceId: string | undefined;
  let artifactId: string | undefined;

  // STEP 1
  try {
    const beforeUser1 = await collabRowIds(user1, "space");
    const spaceName = `Collab Studio ${Date.now()}`;
    await collabClickToolbarButton(user1, "s-home-create-space");
    await collabWaitForDialog(user1);
    await user1.locator("#name").fill(spaceName);
    await collabSelectOption(user1, "kind", "Studio");
    await collabSelectOption(user1, "visibility", "Public");
    await collabSubmitDialog(user1);
    spaceId = await collabWaitForNewRow(user1, "space", beforeUser1, 30_000);
    await collabWaitForRow(user2, "space", spaceId, 60_000);
    record(1, true, `space ${spaceId} created and replicated to user2's Home within budget`);
  } catch (error) {
    await collabScreenshot(user1, "step1-user1");
    await collabScreenshot(user2, "step1-user2");
    record(1, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 2
  if (spaceId) {
    try {
      const row = user1.locator(`[data-row-id="space:${spaceId}"]`);
      await row.getByTitle(/share/i).click();
      await collabWaitForDialog(user1);
      await user1.locator("#email").fill(COLLAB_E2E_USER2_EMAIL);
      await collabSelectOption(user1, "role", "Author");
      await collabSubmitDialog(user1);
      await user2.goto(`${new URL(user2.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await user2.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 30_000 });
      record(2, true, `user2 opened /spaces/${spaceId} and the Space app's artifact table rendered`);
    } catch (error) {
      await collabScreenshot(user1, "step2-user1");
      await collabScreenshot(user2, "step2-user2");
      record(2, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(2, false, "skipped — no space id from STEP 1");
  }

  // STEP 3
  if (spaceId) {
    try {
      await user1.goto(`${new URL(user1.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await user1.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 30_000 });
      const beforeUser1 = await collabRowIds(user1, "artifact");
      await collabClickToolbarButton(user1, "s-space-create-artifact");
      await collabWaitForDialog(user1);
      await user1.locator("#name").fill("Collab Writer");
      await collabSelectOption(user1, "kindId", "Writer");
      await collabSubmitDialog(user1);
      artifactId = await collabWaitForNewRow(user1, "artifact", beforeUser1, 30_000);
      await collabWaitForRow(user2, "artifact", artifactId, 30_000);
      const editorOpened = (await user1.locator('textarea, [contenteditable="true"]').count()) > 0;
      spaceE2eAssert(
        editorOpened,
        "no editable text surface appeared for user1 after createArtifact — Effect::ReplayShellCommand{os.open-artifact} is sent WITHOUT documentId (🧰️framework/…/🔌️plugin/🦀️component.rs relay_open_artifact), so ShellHost's applyHostEffects never calls openDocument for the real hub-bound document (lane 3-B, not landed this wave)",
      );
      record(3, true, `artifact ${artifactId} created, row replicated to user2, editor surface present for user1`);
    } catch (error) {
      await collabScreenshot(user1, "step3-user1");
      await collabScreenshot(user2, "step3-user2");
      record(3, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(3, false, "skipped — no space id from STEP 1");
  }

  // STEP 4
  if (spaceId && artifactId) {
    try {
      const row2 = user2.locator(`[data-row-id="artifact:${artifactId}"]`);
      await row2.getByTitle(/open/i).click();
      await user2.waitForTimeout(1_000);
      const editor1 = user1.locator('textarea, [contenteditable="true"]').first();
      const editor2 = user2.locator('textarea, [contenteditable="true"]').first();
      spaceE2eAssert((await editor1.count()) > 0, "user1 has no editable text surface open (see STEP 3)");
      spaceE2eAssert((await editor2.count()) > 0, "user2 has no editable text surface open after clicking the artifact row's open button");
      const probeText = `collab-probe-${Date.now()}`;
      await editor1.click();
      await editor1.type(probeText);
      const deadline = Date.now() + 30_000;
      let seen = "";
      while (Date.now() < deadline) {
        seen = (await editor2.inputValue().catch(() => editor2.innerText().catch(() => ""))) ?? "";
        if (seen.includes(probeText)) break;
        await user2.waitForTimeout(500);
      }
      spaceE2eAssert(
        seen.includes(probeText),
        `user2's editor never showed user1's typed text ${JSON.stringify(probeText)} (last seen: ${JSON.stringify(seen.slice(-200))}) — both editors are likely unbound ephemeral instances rather than the same hub-synced document (same root cause as STEP 3)`,
      );
      record(4, true, "user1's typed text propagated to user2's editor");
    } catch (error) {
      await collabScreenshot(user1, "step4-user1");
      await collabScreenshot(user2, "step4-user2");
      record(4, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(4, false, "skipped — no artifact id from STEP 3");
  }

  // STEP 5
  try {
    const peers1 = user1.locator('[id="s-presence-peers"]');
    const peers2 = user2.locator('[id="s-presence-peers"]');
    spaceE2eAssert(
      (await peers1.count()) > 0,
      "#s-presence-peers does not exist in the React shell (🧰️framework/…/renderer/…/ShellHost/🟦️component.tsx never imports or renders PresenceBar — confirmed by grep; lane 2-D wired presence only into the wgpu Shell, 🧊️component.rs, which per the ticket brief does not compile this wave)",
    );
    spaceE2eAssert((await peers2.count()) > 0, "#s-presence-peers does not exist in user2's shell either");
    const roster1 = await peers1.locator('[data-row-id^="peer:"]').count();
    const roster2 = await peers2.locator('[data-row-id^="peer:"]').count();
    spaceE2eAssert(roster1 === 2, `user1's presence roster has ${roster1} peer(s), expected 2`);
    spaceE2eAssert(roster2 === 2, `user2's presence roster has ${roster2} peer(s), expected 2`);
    record(5, true, "both shells show a 2-peer presence roster");
  } catch (error) {
    await collabScreenshot(user1, "step5-user1");
    await collabScreenshot(user2, "step5-user2");
    record(5, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 6
  if (spaceId && artifactId) {
    try {
      await user1.goto(`${new URL(user1.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      const rowBefore1 =
        (await user1
          .locator(`[data-row-id="artifact:${artifactId}"]`)
          .innerText()
          .catch(() => "")) ?? "";
      const rowBefore2 =
        (await user2
          .locator(`[data-row-id="artifact:${artifactId}"]`)
          .innerText()
          .catch(() => "")) ?? "";
      const historyTab = user1.locator('[data-tab-id="framework.panel.history"]');
      spaceE2eAssert((await historyTab.count()) > 0, "no framework.panel.history tab found — cannot reach #s-checkin");
      await historyTab.click();
      const checkinButton = user1.locator('[id="s-checkin"]');
      await checkinButton.waitFor({ state: "visible", timeout: 10_000 });
      await checkinButton.click();
      const message = `collab check-in ${Date.now()}`;
      await user1.locator('[id="s-checkin-message"]').fill(message);
      const historyEntryVisible = user1.getByText(message, { exact: false });
      await user1.locator('[id="s-checkin-message"]').press("Enter");
      await historyEntryVisible.first().waitFor({ state: "visible", timeout: 15_000 });
      await user1.goto(`${new URL(user1.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      const rowAfter1Deadline = Date.now() + 30_000;
      let rowAfter1 = rowBefore1;
      while (Date.now() < rowAfter1Deadline) {
        rowAfter1 =
          (await user1
            .locator(`[data-row-id="artifact:${artifactId}"]`)
            .innerText()
            .catch(() => "")) ?? "";
        if (rowAfter1 !== rowBefore1) break;
        await user1.waitForTimeout(1_000);
      }
      spaceE2eAssert(rowAfter1 !== rowBefore1, `user1's space table row for ${artifactId} did not change after check-in (before: ${JSON.stringify(rowBefore1)}, after: ${JSON.stringify(rowAfter1)})`);
      await user2.goto(`${new URL(user2.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await collabWaitForRow(user2, "artifact", artifactId, 30_000);
      const rowAfter2Deadline = Date.now() + 30_000;
      let rowAfter2 = rowBefore2;
      while (Date.now() < rowAfter2Deadline) {
        rowAfter2 =
          (await user2
            .locator(`[data-row-id="artifact:${artifactId}"]`)
            .innerText()
            .catch(() => "")) ?? "";
        if (rowAfter2 !== rowBefore2) break;
        await user2.waitForTimeout(1_000);
      }
      spaceE2eAssert(rowAfter2 !== rowBefore2, `user2's space table row for ${artifactId} did not change after user1's check-in (before: ${JSON.stringify(rowBefore2)}, after: ${JSON.stringify(rowAfter2)})`);
      record(6, true, "check-in dispatched and the space table's row changed for both users");
    } catch (error) {
      await collabScreenshot(user1, "step6-user1");
      await collabScreenshot(user2, "step6-user2");
      record(6, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(6, false, "skipped — no space/artifact id from earlier steps");
  }

  // STEP 7
  try {
    const connectionsRes = await fetch(`${hubBaseUrl}/admin/api/connections`, { headers: { authorization: `Bearer ${COLLAB_E2E_ADMIN_TOKEN}` } });
    spaceE2eAssert(connectionsRes.ok, `GET /admin/api/connections returned ${connectionsRes.status}`);
    const connections = (await connectionsRes.json()) as readonly Record<string, unknown>[];
    const text = JSON.stringify(connections);
    spaceE2eAssert(text.includes(COLLAB_E2E_USER1_EMAIL) || text.includes("user1"), `/admin/api/connections does not mention user1: ${text.slice(0, 500)}`);
    spaceE2eAssert(text.includes(COLLAB_E2E_USER2_EMAIL) || text.includes("user2"), `/admin/api/connections does not mention user2: ${text.slice(0, 500)}`);
    const adminRes = await fetch(`${hubBaseUrl}/admin`, { headers: { authorization: `Bearer ${COLLAB_E2E_ADMIN_TOKEN}` } });
    spaceE2eAssert(adminRes.ok, `GET /admin returned ${adminRes.status}`);
    const contentType = adminRes.headers.get("content-type") ?? "";
    spaceE2eAssert(contentType.includes("html"), `GET /admin content-type is ${contentType}, expected html`);
    record(
      7,
      true,
      "/admin/api/connections names both users; /admin returns HTML — note: /admin is a client-rendered SPA shell, so the raw HTML byte stream itself does not literally embed the user names (verified via /admin/api/connections instead)",
    );
  } catch (error) {
    record(7, false, error instanceof Error ? error.message : String(error));
  }

  return { results, spaceId, artifactId };
}

/** 🔁️ STEP 8 — restarts the hub against the SAME `dataDir` and the SAME port, then reloads `user2` and
 * confirms the space + artifact rows survive. Deliberately the SAME port (not a fresh one, unlike lane
 * 3-E's Node-only harness): the browser's `S_HUB_URL` is baked into its bundle at Vite `define`-time
 * (contract §C0), so only a same-port restart lets a plain page reload — not a dev-server restart —
 * reconnect; the temp `dataDir` is what actually proves persistence here, matching the brief's own
 * "restart the hub against the same `OS_HUB_DATA`" wording literally. Runs at the orchestration level
 * (not inside `collabRunScenario`) since it needs the hub daemon handle, not just a base URL. */
async function collabRunRestartStep(opts: {
  readonly record: (step: number, pass: boolean, detail: string) => void;
  readonly hubDaemon: SpawnDaemonHandle;
  readonly hubPort: number;
  readonly hubDataDir: string;
  readonly user2: import("playwright").Page;
  readonly spaceId: string | undefined;
  readonly artifactId: string | undefined;
}): Promise<SpawnDaemonHandle> {
  if (!opts.spaceId || !opts.artifactId) {
    opts.record(8, false, "skipped — no space/artifact id from earlier steps");
    return opts.hubDaemon;
  }
  try {
    opts.hubDaemon.kill();
    // 🧵️ We hold the hub's own `child` handle — await its `exit` event via 🔖️PollHelpers's
    // `awaitChildExit` instead of polling `exitCode` (THE RULE above). Same 30s budget as before.
    const exited = await awaitChildExit(opts.hubDaemon.child, 30_000);
    spaceE2eAssert(exited === "exited", "hub process did not exit within 30s of being killed");
    const portFreed = await awaitTcpReady("127.0.0.1", opts.hubPort, { deadlineMs: 30_000, intervalMs: 250, mode: "closed" });
    spaceE2eAssert(portFreed === "ready", `port ${opts.hubPort} never freed up after the hub exited`);
    const newHubDaemon = await collabStartHub(opts.hubPort, opts.hubDataDir, join(collabOutDir(), "🧪️3-c-hub-restart.txt"));
    await opts.user2.reload({ waitUntil: "domcontentloaded" });
    await opts.user2.goto(`${new URL(opts.user2.url()).origin}/spaces/${opts.spaceId}`, { waitUntil: "domcontentloaded" });
    await collabWaitForRow(opts.user2, "artifact", opts.artifactId, 60_000);
    opts.record(8, true, `hub restarted against the same OS_HUB_DATA (${opts.hubDataDir}) on the same port; user2 still sees space ${opts.spaceId} and artifact ${opts.artifactId} after reload`);
    return newHubDaemon;
  } catch (error) {
    await collabScreenshot(opts.user2, "step8-user2");
    opts.record(8, false, error instanceof Error ? error.message : String(error));
    return opts.hubDaemon;
  }
}

/** 🎬️ Orchestrates the full harness: port scan, temp data dirs, hub boot, plugin prebuild, two `s`
 * react dev servers, two independent Playwright browser contexts, the 8-step scenario, and teardown of
 * every spawned process (hub + both dev servers + browser) even on failure. Writes `STEP n: PASS/FAIL`
 * lines plus a final summary, and sets a non-zero exit code if any step failed. */
async function runCollabE2eVerify(): Promise<void> {
  const outDir = collabOutDir();
  const taken = new Set<number>();
  const hubPort = collabScanPort("S_COLLAB_HUB_PORT", taken);
  const user1Port = collabScanPort("S_COLLAB_USER1_PORT", taken);
  const user2Port = collabScanPort("S_COLLAB_USER2_PORT", taken);
  console.log(`[collab-e2e] ports: hub=${hubPort} user1=${user1Port} user2=${user2Port}`);

  const hubDataDir = mkdtempSync(join(tmpdir(), "semio-collab-hub-"));
  const user1DataDir = mkdtempSync(join(tmpdir(), "semio-collab-u1-"));
  const user2DataDir = mkdtempSync(join(tmpdir(), "semio-collab-u2-"));

  let hubDaemon: SpawnDaemonHandle | undefined;
  let user1Daemon: SpawnDaemonHandle | undefined;
  let user2Daemon: SpawnDaemonHandle | undefined;
  let browser: import("playwright").Browser | undefined;
  const results: CollabStepOutcome[] = [];
  const record = (step: number, pass: boolean, detail: string): void => {
    results.push({ step, name: COLLAB_E2E_STEP_NAMES[step - 1]!, pass, detail });
    console.log(`STEP ${step}: ${pass ? "PASS" : "FAIL"}: ${COLLAB_E2E_STEP_NAMES[step - 1]} — ${detail}`);
  };

  const teardown = async (): Promise<void> => {
    try {
      await browser?.close();
    } catch {
      // 🏁️ Best-effort.
    }
    for (const daemon of [user1Daemon, user2Daemon, hubDaemon]) {
      try {
        daemon?.kill();
      } catch {
        // 🏁️ Best-effort — a teardown failure must never mask the real run's outcome.
      }
    }
  };

  try {
    try {
      hubDaemon = await collabStartHub(hubPort, hubDataDir, join(outDir, "🧪️3-c-hub-boot.txt"));
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] hub failed to boot — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 8; step++) record(step, false, `blocked — hub never became ready: ${message}`);
      throw error;
    }

    try {
      await collabPrebuildPlugins();
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] plugin prebuild failed — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 8; step++) record(step, false, `blocked — plugin prebuild failed: ${message}`);
      throw error;
    }

    const hubBaseUrl = `http://127.0.0.1:${hubPort}`;
    try {
      [user1Daemon, user2Daemon] = await Promise.all([
        collabStartUserDevServer({ port: user1Port, hubUrl: hubBaseUrl, user: COLLAB_E2E_USER1_EMAIL, dataDir: user1DataDir, logPath: join(outDir, "🧪️3-c-user1-dev.txt") }),
        collabStartUserDevServer({ port: user2Port, hubUrl: hubBaseUrl, user: COLLAB_E2E_USER2_EMAIL, dataDir: user2DataDir, logPath: join(outDir, "🧪️3-c-user2-dev.txt") }),
      ]);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] a shell dev server never booted — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 8; step++) record(step, false, `blocked — shells did not boot: ${message}`);
      throw error;
    }

    // 🎭️ Matches `SetupScript`'s own install location (`node_modules/.cache/ms-playwright`) — without
    // this, Playwright falls back to the OS default cache (`~/Library/Caches/ms-playwright`), which can
    // hold a different/older browser revision than the one this repo's `playwright` version expects
    // (confirmed during this lane's own iteration: the default cache had `chromium-1223`, this repo's
    // `playwright` wanted `chromium_headless_shell-1234`, which only exists under the repo-scoped path).
    process.env.PLAYWRIGHT_BROWSERS_PATH = process.env.PLAYWRIGHT_BROWSERS_PATH ?? join(repoRoot, "node_modules", ".cache", "ms-playwright");
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const context1 = await browser.newContext();
    const context2 = await browser.newContext();
    const user1Page = await context1.newPage();
    const user2Page = await context2.newPage();
    const pageErrors: string[] = [];
    /** 🔬️ Full browser-side visibility for whichever page this is attached to: every `console.*` level
     * (not just uncaught exceptions), failed HTTP requests, and the lifecycle + received frames of every
     * WebSocket the page opens (the `/directory/ws` subscription in particular) — printed immediately so
     * they interleave chronologically with the harness's own `STEP n:` lines in the run log, instead of
     * being buffered and dumped out of order at the end. Added per the ticket's w4-h diagnosis: the prior
     * harness only captured `pageerror`, so a silently-caught `console.warn`/`console.error` inside
     * `ShellHost` was invisible even though it was the only place the real failing branch could show up. */
    const attachBrowserDiagnostics = (page: import("playwright").Page, label: string): void => {
      page.on("pageerror", (err) => pageErrors.push(`${label}: ${String(err)}`));
      page.on("console", (msg) => console.log(`[collab-e2e:console] ${label} [${msg.type()}] ${msg.text()}`));
      page.on("requestfailed", (request) => console.log(`[collab-e2e:network] ${label} requestfailed: ${request.method()} ${request.url()} — ${request.failure()?.errorText ?? "unknown"}`));
      page.on("response", (response) => {
        const url = response.url();
        if (!url.includes("/auth/") && !url.includes("/directory/")) return;
        const status = response.status();
        const auth = response.request().headers()["authorization"] ?? "none";
        const postData = response.request().postData() ?? "";
        console.log(`[collab-e2e:network] ${label} response: ${response.request().method()} ${url} auth=${auth} body=${postData.slice(0, 300)} — ${status}`);
      });
      page.on("websocket", (ws) => {
        console.log(`[collab-e2e:ws] ${label} opened: ${ws.url()}`);
        ws.on("framereceived", (frame) => console.log(`[collab-e2e:ws] ${label} recv: ${(typeof frame.payload === "string" ? frame.payload : "<binary>").slice(0, 800)}`));
        ws.on("close", () => console.log(`[collab-e2e:ws] ${label} closed: ${ws.url()}`));
        ws.on("socketerror", (error) => console.log(`[collab-e2e:ws] ${label} socketerror: ${ws.url()} — ${error}`));
      });
    };
    attachBrowserDiagnostics(user1Page, "user1");
    attachBrowserDiagnostics(user2Page, "user2");

    await user1Page.goto(`http://127.0.0.1:${user1Port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await user2Page.goto(`http://127.0.0.1:${user2Port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await user1Page.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 120_000 });
    await user2Page.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 120_000 });
    await user1Page.waitForTimeout(2_000);
    await user2Page.waitForTimeout(2_000);

    const scenario = await collabRunScenario(user1Page, user2Page, hubBaseUrl);
    for (const outcome of scenario.results) results.push(outcome);

    hubDaemon = await collabRunRestartStep({ record, hubDaemon: hubDaemon!, hubPort, hubDataDir, user2: user2Page, spaceId: scenario.spaceId, artifactId: scenario.artifactId });

    const ignorableGpuFragments = ["NoCompatibleDevice"];
    const criticalErrors = pageErrors.filter((message) => !ignorableGpuFragments.some((fragment) => message.includes(fragment)));
    if (criticalErrors.length > 0) console.warn(`[collab-e2e] page errors observed (not a step on their own, informational): ${criticalErrors.join(" | ")}`);
  } finally {
    await teardown();
  }

  const passed = results.filter((outcome) => outcome.pass).length;
  console.log(`[collab-e2e] summary: ${passed}/${results.length} steps passed`);
  for (const outcome of results) console.log(`  STEP ${outcome.step}: ${outcome.pass ? "PASS" : "FAIL"}: ${outcome.name}`);
  if (passed !== results.length) process.exitCode = 1;
}
//#endregion 🔖️CollabE2e

class VerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const port = process.env.S_OS_PORT ?? "6070";
    const studioUrl = process.env.S_STUDIO_URL ?? `http://127.0.0.1:${port}/`;
    const timeoutMs = Number(process.env.S_STUDIO_E2E_TIMEOUT_MS ?? 300_000);
    if (segments[0] === "collab") {
      await runCollabE2eVerify();
      return;
    }
    if (segments[0] === "e2e") {
      await runStudioE2eVerify(studioUrl, timeoutMs);
      console.log(`s studio e2e verify passed (${studioUrl})`);
      return;
    }
    for (const target of generatePluginRegistry(repoRoot)) {
      const packageName = await readPackageName(target.cratePath);
      if (runCmdStatus("cargo", ["test", "--lib", "-p", packageName], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error(`${packageName} tests failed`);
    }
    if (runBunxStatus(["vitest", "run"], join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react")) !== 0) throw new Error("framework-renderer-react tests failed");
    await runStudioE2eVerify(studioUrl, timeoutMs);
    await new PluginCapabilityLintScript(this.root).run([]);
    console.log(`s studio verify passed (${studioUrl})`);
  }
}

//#region 🔬️ParityScript
/** 🔬️wgpu↔React UI-parity verification harness — structural DOM/retained-tree comparison, per-region
 * pixel diffing, and a boot-triage ladder, driven per catalog playground. Ticket:
 * `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/`. */

//#region 🔖️ParityTypes
type ParityRenderer = "react" | "wgpu";
type ParityRect = readonly [number, number, number, number];
type ParityColor = readonly [number, number, number, number];

type ParityNode = {
  readonly path: string;
  readonly kind: string;
  readonly rect: ParityRect;
  readonly text: string | null;
  readonly color: ParityColor | null;
  readonly bg: ParityColor | null;
  readonly fontSize: number | null;
  readonly fontWeight: number | null;
  readonly visible: boolean;
  readonly state: { readonly hovered: boolean; readonly disabled: boolean; readonly selected: boolean };
};

type ParityDump = {
  readonly viewport: { readonly w: number; readonly h: number; readonly dpr: number };
  readonly focusPath: string | null;
  readonly nodes: readonly ParityNode[];
};

type ParityMismatchAxis = "topology" | "text" | "rect" | "color" | "bg" | "fontSize" | "focus";
type ParityMismatch = { readonly path: string; readonly axis: ParityMismatchAxis; readonly react: unknown; readonly wgpu: unknown };
type StructuralResult = { readonly status: "PASS" | "FAIL"; readonly nodeCount: number; readonly mismatches: readonly ParityMismatch[] };

/** 🪜️Boot-triage ladder status — evaluated before any structural/pixel comparison, never conflated with a mismatch.
 * `STALE-BRIDGE` (terra-parity-rebaseline): a variant activation failing because its on-disk jco bridge
 * (`🔌️plugin-modules/<variant>/semio_s_plugin_*.js`) is still the pre-H2 `runSerialized` shape and has no
 * `createActorApi` export — see `🔖️Triage`'s `parityClassifyStaleBridge`. Distinct from every other rung:
 * those are architecture/runtime defects, this is "the fleet hasn't regenerated this bridge yet", which is
 * the EXPECTED state for most of the 58 variants until `sdk-green` lands (📌️important.md, §"48 materialised
 * plugin bridges"). A sweep that reports this as a bare FAIL is not measuring anything real. */
type BootStatus = "PASS" | "SERVER-FAIL" | "BOOT-TIMEOUT" | "ENV-FAIL" | "DUMP-EMPTY" | "BLANK-PAINT" | "STALE-BRIDGE";

type PixelRegionResult = { readonly path: string; readonly ratio: number; readonly threshold: number; readonly diffPng?: string };

type ParityPlaygroundReport = {
  readonly variant: string;
  readonly boot: { readonly react: BootStatus; readonly wgpu: BootStatus; readonly detail?: string };
  readonly structural?: StructuralResult;
  readonly pixel?: { readonly status: "PASS" | "FAIL"; readonly regions: readonly PixelRegionResult[] };
  /** 🎬️See `🔖️ProbeCatalog` — behavioral (interaction-driven) parity, distinct from the static
   * `structural`/`pixel` end-state checks above. Optional: only populated once boot passed (a probe
   * can't drive a page that never finished booting). */
  readonly behavioral?: ProbeRunResult;
  readonly durationMs: number;
};
//#endregion 🔖️ParityTypes

//#region 🔖️StructuralDump
/** 🌳️DOM-side structural walk — every element carrying `data-ui-path` (see `framework/os/renderer/js/react/index.tsx`
 * region `🔖️UiInterpreter`) is one matched node. `text` is only captured for non-container kinds since
 * `textContent` on a container aggregates all descendant text, which would false-positive against wgpu's
 * per-node (non-aggregated) text field. */
const PARITY_CONTAINER_KINDS = new Set(["stack", "field", "section", "group", "tree", "componentScene", "externalSlot"]);

const REACT_DOM_DUMP_SCRIPT = `(() => {
  const CONTAINER_KINDS = new Set(${JSON.stringify([...PARITY_CONTAINER_KINDS])});
  function parseColor(str) {
    const m = /rgba?\\(([^)]+)\\)/.exec(str || "");
    if (!m) return null;
    const parts = m[1].split(",").map((s) => parseFloat(s.trim()));
    if (parts.length < 3) return null;
    return [Math.round(parts[0]), Math.round(parts[1]), Math.round(parts[2]), parts[3] === undefined ? 1 : parts[3]];
  }
  function nearestPath(el) {
    let cur = el;
    while (cur) {
      const p = cur.getAttribute && cur.getAttribute("data-ui-path");
      if (p) return p;
      cur = cur.parentElement;
    }
    return null;
  }
  const nodes = [];
  document.querySelectorAll("[data-ui-path]").forEach((el) => {
    const rect = el.getBoundingClientRect();
    const style = getComputedStyle(el);
    const path = el.getAttribute("data-ui-path");
    const kind = (path.split("/").pop() || "").replace(/\\[.*/, "").replace(/^#.*/, "");
    nodes.push({
      path,
      kind,
      rect: [Math.round(rect.x), Math.round(rect.y), Math.round(rect.width), Math.round(rect.height)],
      text: CONTAINER_KINDS.has(kind) ? null : (el.textContent || "").replace(/\\s+/g, " ").trim() || null,
      color: parseColor(style.color),
      bg: parseColor(style.backgroundColor),
      fontSize: parseFloat(style.fontSize) || null,
      fontWeight: parseFloat(style.fontWeight) || null,
      visible: rect.width > 0 && rect.height > 0 && style.visibility !== "hidden" && style.display !== "none",
      state: {
        hovered: el.matches(":hover"),
        disabled: el.hasAttribute("disabled") || el.getAttribute("aria-disabled") === "true",
        selected: el.getAttribute("aria-selected") === "true" || el.getAttribute("aria-pressed") === "true",
      },
    });
  });
  const active = document.activeElement;
  return JSON.stringify({
    viewport: { w: window.innerWidth, h: window.innerHeight, dpr: window.devicePixelRatio },
    focusPath: active ? nearestPath(active) : null,
    nodes,
  });
})()`;

async function dumpReactStructure(page: import("playwright").Page): Promise<ParityDump> {
  const json = await page.evaluate(REACT_DOM_DUMP_SCRIPT);
  return JSON.parse(json as unknown as string) as ParityDump;
}

/** 🧊️Calls the wasm-bindgen introspection hooks exposed by `framework/os/renderer/wgpu/rs/lib.rs` region
 * `🔬️Introspection`. Reachable at `window.wasmBindings.dumpStructure()`/`dumpFrameStats()` — Trunk's
 * dev-server boot glue (`framework/os/renderer/wgpu/js/🟦️boot.ts`) attaches the wasm module's exports there
 * (the same path `semioWgpuMount`/`uploadIconAtlas` already use), NOT a bespoke global. Returns an
 * empty dump (never throws) when the hooks aren't present yet, so triage can distinguish "not booted"
 * from "no hooks" via `DUMP-EMPTY`. */
async function dumpWgpuStructure(page: import("playwright").Page): Promise<ParityDump> {
  const json = await page.evaluate(() => (window as unknown as { wasmBindings?: { dumpStructure?: () => string } }).wasmBindings?.dumpStructure?.());
  if (!json) return { viewport: { w: 0, h: 0, dpr: 1 }, focusPath: null, nodes: [] };
  return JSON.parse(json) as ParityDump;
}

async function dumpWgpuFrameStats(page: import("playwright").Page): Promise<{ readonly drawCalls: number; readonly quads: number; readonly glyphs: number } | null> {
  const json = await page.evaluate(() => (window as unknown as { wasmBindings?: { dumpFrameStats?: () => string } }).wasmBindings?.dumpFrameStats?.());
  if (!json) return null;
  const stats = JSON.parse(json) as { readonly drawCalls: number; readonly quadCount: number; readonly glyphCount: number };
  return { drawCalls: stats.drawCalls, quads: stats.quadCount, glyphs: stats.glyphCount };
}
//#endregion 🔖️StructuralDump

//#region 🔖️StructuralCompare
const PARITY_RECT_TOLERANCE_PX = 1.5;
const PARITY_COLOR_TOLERANCE = 3;
const PARITY_FONT_SIZE_TOLERANCE_PX = 0.5;
/** 🎨️Scene canvases are rasterized by two different pipelines — structural comparison covers only
 * their rect (placement), never their internal text/color, which is a pixel/behavioral-probe concern. */
const PARITY_SCENE_LEAF_KINDS = new Set(["componentScene", "image"]);

function parityNormalizeText(s: string | null): string | null {
  return s === null ? null : s.normalize("NFC").replace(/\s+/g, " ").trim();
}

function parityColorClose(a: ParityColor | null, b: ParityColor | null): boolean {
  if (a === null || b === null) return a === b;
  return Math.abs(a[0] - b[0]) <= PARITY_COLOR_TOLERANCE && Math.abs(a[1] - b[1]) <= PARITY_COLOR_TOLERANCE && Math.abs(a[2] - b[2]) <= PARITY_COLOR_TOLERANCE;
}

/** 🎨️React's dump reports sRGB `rgb()` CSS values as 0–255 ints; wgpu's `Theme` colors are LINEAR-space
 * 0–1 floats (see `framework/os/renderer/wgpu/rs/lib.rs`'s `🔬️IntrospectionVisualFields` doc comment) —
 * comparing them raw would treat every color as a mismatch. Converts wgpu's linear floats to sRGB
 * 0–255 ints so both sides land in the same space before `parityColorClose`'s byte-scale tolerance applies. */
function parityLinearToSrgbColor(c: ParityColor | null): ParityColor | null {
  if (c === null) return null;
  const toByte = (channel: number): number => {
    const clamped = Math.min(1, Math.max(0, channel));
    const srgb = clamped <= 0.0031308 ? clamped * 12.92 : 1.055 * Math.pow(clamped, 1 / 2.4) - 0.055;
    return Math.round(srgb * 255);
  };
  return [toByte(c[0]), toByte(c[1]), toByte(c[2]), c[3]];
}

function compareParityStructural(reactDump: ParityDump, wgpuDump: ParityDump): StructuralResult {
  const mismatches: ParityMismatch[] = [];
  const reactByPath = new Map(reactDump.nodes.map((n) => [n.path, n]));
  const wgpuByPath = new Map(wgpuDump.nodes.map((n) => [n.path, n]));
  const allPaths = new Set([...reactByPath.keys(), ...wgpuByPath.keys()]);
  for (const path of allPaths) {
    const r = reactByPath.get(path);
    const w = wgpuByPath.get(path);
    if (!r || !w) {
      mismatches.push({ path, axis: "topology", react: r?.kind ?? null, wgpu: w?.kind ?? null });
      continue;
    }
    const isSceneLeaf = PARITY_SCENE_LEAF_KINDS.has(r.kind);
    if (!isSceneLeaf && parityNormalizeText(r.text) !== parityNormalizeText(w.text)) {
      mismatches.push({ path, axis: "text", react: r.text, wgpu: w.text });
    }
    const [rx, ry, rw, rh] = r.rect;
    const [wx, wy, ww, wh] = w.rect;
    if (Math.abs(rx - wx) > PARITY_RECT_TOLERANCE_PX || Math.abs(ry - wy) > PARITY_RECT_TOLERANCE_PX || Math.abs(rw - ww) > PARITY_RECT_TOLERANCE_PX || Math.abs(rh - wh) > PARITY_RECT_TOLERANCE_PX) {
      mismatches.push({ path, axis: "rect", react: r.rect, wgpu: w.rect });
    }
    const wColorSrgb = parityLinearToSrgbColor(w.color);
    const wBgSrgb = parityLinearToSrgbColor(w.bg);
    if (!isSceneLeaf && !parityColorClose(r.color, wColorSrgb)) mismatches.push({ path, axis: "color", react: r.color, wgpu: wColorSrgb });
    if (!isSceneLeaf && !parityColorClose(r.bg, wBgSrgb)) mismatches.push({ path, axis: "bg", react: r.bg, wgpu: wBgSrgb });
    if (r.fontSize !== null && w.fontSize !== null && Math.abs(r.fontSize - w.fontSize) > PARITY_FONT_SIZE_TOLERANCE_PX) {
      mismatches.push({ path, axis: "fontSize", react: r.fontSize, wgpu: w.fontSize });
    }
  }
  if (parityNormalizeText(reactDump.focusPath) !== parityNormalizeText(wgpuDump.focusPath)) {
    mismatches.push({ path: "$focus", axis: "focus", react: reactDump.focusPath, wgpu: wgpuDump.focusPath });
  }
  return { status: mismatches.length === 0 ? "PASS" : "FAIL", nodeCount: allPaths.size, mismatches: mismatches.slice(0, 200) };
}
//#endregion 🔖️StructuralCompare

//#region 🔖️PixelCompare
/** 📐️Pixel-region gate covers only structural *containers* (matches the design's "container-level
 * matched node pairs" — navbar/footer/panel-level regions are shell chrome, out of scope for this
 * pass since they're not part of the UiNode tree) plus scene/image leaves, not every leaf text node —
 * bounding pixel-diff cost to O(containers) rather than O(all nodes) per playground. */
const PARITY_PIXEL_REGION_KINDS = new Set(["stack", "field", "section", "group", "tree", "componentScene", "image"]);
const PARITY_PIXEL_THRESHOLD_DEFAULT = 0.005;
const PARITY_PIXEL_THRESHOLD_SCENE = 0.02;
const OWNED_PARITY_AA_MIN_CONTRAST_SQUARED = 0.0225;
const OWNED_PARITY_AA_MAX_COVERAGE_DELTA = 0.35;
const OWNED_PARITY_DIFF_MISMATCH = Object.freeze({ blue: 64, green: 32, red: 255 });
const OWNED_PARITY_DIFF_ANTIALIAS = Object.freeze({ blue: 0, green: 192, red: 255 });

type OwnedParityPixelOptions = {
  readonly ignoreAntialiasing: boolean;
  readonly threshold: number;
};

/** 🌐️ Decodes PNG bytes into exact RGBA pixels in the already-open parity browser page. */
async function decodeParityScreenshot(page: import("playwright").Page, bytes: Uint8Array): Promise<OwnedParityImage> {
  const decoded = await page.evaluate(async (encoded) => {
    const bitmap = await createImageBitmap(new Blob([Uint8Array.from(encoded)], { type: "image/png" }));
    try {
      const canvas = document.createElement("canvas");
      canvas.width = bitmap.width;
      canvas.height = bitmap.height;
      const context = canvas.getContext("2d");
      if (!context) throw new Error("Canvas 2D context is unavailable for parity PNG decoding");
      context.drawImage(bitmap, 0, 0);
      return { width: bitmap.width, height: bitmap.height, data: Array.from(context.getImageData(0, 0, bitmap.width, bitmap.height).data) };
    } finally {
      bitmap.close();
    }
  }, Array.from(bytes));
  return { width: decoded.width, height: decoded.height, data: Uint8Array.from(decoded.data) };
}

/** ✂️ Copies a bounded row-major RGBA crop without retaining or mutating source storage. */
function cropOwnedParityRgba(image: OwnedParityImage, x: number, y: number, width: number, height: number): Uint8Array {
  if (![image.width, image.height, x, y, width, height].every((value) => Number.isSafeInteger(value) && value >= 0)) throw new Error("Owned parity crop dimensions must be non-negative safe integers");
  const imageBytes = image.width * image.height * 4;
  if (!Number.isSafeInteger(imageBytes) || image.data.length !== imageBytes) throw new Error(`Owned parity image must contain exactly ${imageBytes} RGBA bytes`);
  if (x + width > image.width || y + height > image.height) throw new Error("Owned parity crop must stay within image bounds");
  const cropped = new Uint8Array(width * height * 4);
  const rowBytes = width * 4;
  for (let row = 0; row < height; row++) {
    const sourceOffset = ((y + row) * image.width + x) * 4;
    cropped.set(image.data.subarray(sourceOffset, sourceOffset + rowBytes), row * rowBytes);
  }
  return cropped;
}

/** 🖼️ Encodes exact RGBA pixels as a non-byte-contractual diagnostic PNG in the parity page. */
async function encodeParityDiff(page: import("playwright").Page, data: Uint8Array, width: number, height: number): Promise<Uint8Array> {
  if (!Number.isSafeInteger(width) || !Number.isSafeInteger(height) || width <= 0 || height <= 0 || data.length !== width * height * 4) throw new Error("Owned parity diagnostic must contain positive dimensions and exact RGBA bytes");
  const encoded = await page.evaluate(
    async ({ pixels, width: imageWidth, height: imageHeight }) => {
      const canvas = document.createElement("canvas");
      canvas.width = imageWidth;
      canvas.height = imageHeight;
      const context = canvas.getContext("2d");
      if (!context) throw new Error("Canvas 2D context is unavailable for parity PNG encoding");
      context.putImageData(new ImageData(Uint8ClampedArray.from(pixels), imageWidth, imageHeight), 0, 0);
      const blob = await new Promise<Blob>((resolve, reject) => canvas.toBlob((value) => (value ? resolve(value) : reject(new Error("Canvas could not encode the parity diagnostic PNG"))), "image/png"));
      return Array.from(new Uint8Array(await blob.arrayBuffer()));
    },
    { pixels: Array.from(data), width, height },
  );
  return Uint8Array.from(encoded);
}

function ownedParityByteRangesOverlap(left: Uint8Array, right: Uint8Array): boolean {
  if (left.buffer !== right.buffer) return false;
  const leftEnd = left.byteOffset + left.byteLength;
  const rightEnd = right.byteOffset + right.byteLength;
  return left.byteOffset < rightEnd && right.byteOffset < leftEnd;
}

function ownedParityCompositeChannel(pixels: Uint8Array, offset: number, channel: number): number {
  const alpha = pixels[offset + 3]! / 255;
  return 1 - alpha + (pixels[offset + channel]! / 255) * alpha;
}

function ownedParityDistanceSquared(reference: Uint8Array, referenceOffset: number, candidate: Uint8Array, candidateOffset: number): number {
  const alphaCoverageScale = reference[referenceOffset] === candidate[candidateOffset] && reference[referenceOffset + 1] === candidate[candidateOffset + 1] && reference[referenceOffset + 2] === candidate[candidateOffset + 2] ? 0.5 : 1;
  const red = (ownedParityCompositeChannel(reference, referenceOffset, 0) - ownedParityCompositeChannel(candidate, candidateOffset, 0)) * alphaCoverageScale;
  const green = (ownedParityCompositeChannel(reference, referenceOffset, 1) - ownedParityCompositeChannel(candidate, candidateOffset, 1)) * alphaCoverageScale;
  const blue = (ownedParityCompositeChannel(reference, referenceOffset, 2) - ownedParityCompositeChannel(candidate, candidateOffset, 2)) * alphaCoverageScale;
  return 0.299 * red * red + 0.587 * green * green + 0.114 * blue * blue;
}

function ownedParityWriteMuted(reference: Uint8Array, offset: number, diff: Uint8Array): void {
  const luminance = 0.2126 * ownedParityCompositeChannel(reference, offset, 0) + 0.7152 * ownedParityCompositeChannel(reference, offset, 1) + 0.0722 * ownedParityCompositeChannel(reference, offset, 2);
  const muted = Math.round(255 * (0.75 + luminance * 0.25));
  diff[offset] = muted;
  diff[offset + 1] = muted;
  diff[offset + 2] = muted;
  diff[offset + 3] = 255;
}

function ownedParityWriteMarker(diff: Uint8Array, offset: number, marker: Readonly<{ blue: number; green: number; red: number }>): void {
  diff[offset] = marker.red;
  diff[offset + 1] = marker.green;
  diff[offset + 2] = marker.blue;
  diff[offset + 3] = 255;
}

function ownedParityCoverage(reference: Uint8Array, candidate: Uint8Array, offset: number, lowRed: number, lowGreen: number, lowBlue: number, spanRed: number, spanGreen: number, spanBlue: number, spanSquared: number): boolean {
  const referenceRed = ownedParityCompositeChannel(reference, offset, 0) - lowRed;
  const referenceGreen = ownedParityCompositeChannel(reference, offset, 1) - lowGreen;
  const referenceBlue = ownedParityCompositeChannel(reference, offset, 2) - lowBlue;
  const candidateRed = ownedParityCompositeChannel(candidate, offset, 0) - lowRed;
  const candidateGreen = ownedParityCompositeChannel(candidate, offset, 1) - lowGreen;
  const candidateBlue = ownedParityCompositeChannel(candidate, offset, 2) - lowBlue;
  const referenceCoverage = (referenceRed * spanRed + referenceGreen * spanGreen + referenceBlue * spanBlue) / spanSquared;
  const candidateCoverage = (candidateRed * spanRed + candidateGreen * spanGreen + candidateBlue * spanBlue) / spanSquared;
  if (referenceCoverage <= 0.03 || referenceCoverage >= 0.97 || candidateCoverage <= 0.03 || candidateCoverage >= 0.97 || Math.abs(referenceCoverage - candidateCoverage) > OWNED_PARITY_AA_MAX_COVERAGE_DELTA) return false;
  const referenceResidualRed = referenceRed - referenceCoverage * spanRed;
  const referenceResidualGreen = referenceGreen - referenceCoverage * spanGreen;
  const referenceResidualBlue = referenceBlue - referenceCoverage * spanBlue;
  const candidateResidualRed = candidateRed - candidateCoverage * spanRed;
  const candidateResidualGreen = candidateGreen - candidateCoverage * spanGreen;
  const candidateResidualBlue = candidateBlue - candidateCoverage * spanBlue;
  const referenceResidual = referenceResidualRed * referenceResidualRed + referenceResidualGreen * referenceResidualGreen + referenceResidualBlue * referenceResidualBlue;
  const candidateResidual = candidateResidualRed * candidateResidualRed + candidateResidualGreen * candidateResidualGreen + candidateResidualBlue * candidateResidualBlue;
  return referenceResidual <= spanSquared * 0.01 && candidateResidual <= spanSquared * 0.01;
}

function ownedParityIsAntialiased(reference: Uint8Array, candidate: Uint8Array, width: number, height: number, x: number, y: number, offset: number, thresholdSquared: number): boolean {
  if (x === 0 || y === 0 || x === width - 1 || y === height - 1) return false;
  let lowBlue = 0;
  let lowGreen = 0;
  let lowLuminance = Number.POSITIVE_INFINITY;
  let lowRed = 0;
  let highBlue = 0;
  let highGreen = 0;
  let highLuminance = Number.NEGATIVE_INFINITY;
  let highRed = 0;
  let stableNeighbors = 0;
  const minimumY = Math.max(0, y - 1);
  const maximumY = Math.min(height - 1, y + 1);
  const minimumX = Math.max(0, x - 1);
  const maximumX = Math.min(width - 1, x + 1);
  for (let neighborY = minimumY; neighborY <= maximumY; neighborY++) {
    for (let neighborX = minimumX; neighborX <= maximumX; neighborX++) {
      const neighborOffset = (neighborY * width + neighborX) * 4;
      if (neighborOffset === offset || ownedParityDistanceSquared(reference, neighborOffset, candidate, neighborOffset) > thresholdSquared) continue;
      stableNeighbors += 1;
      const red = (ownedParityCompositeChannel(reference, neighborOffset, 0) + ownedParityCompositeChannel(candidate, neighborOffset, 0)) * 0.5;
      const green = (ownedParityCompositeChannel(reference, neighborOffset, 1) + ownedParityCompositeChannel(candidate, neighborOffset, 1)) * 0.5;
      const blue = (ownedParityCompositeChannel(reference, neighborOffset, 2) + ownedParityCompositeChannel(candidate, neighborOffset, 2)) * 0.5;
      const luminance = 0.2126 * red + 0.7152 * green + 0.0722 * blue;
      if (luminance < lowLuminance) {
        lowBlue = blue;
        lowGreen = green;
        lowLuminance = luminance;
        lowRed = red;
      }
      if (luminance > highLuminance) {
        highBlue = blue;
        highGreen = green;
        highLuminance = luminance;
        highRed = red;
      }
    }
  }
  if (stableNeighbors < 2) return false;
  const spanRed = highRed - lowRed;
  const spanGreen = highGreen - lowGreen;
  const spanBlue = highBlue - lowBlue;
  const spanSquared = spanRed * spanRed + spanGreen * spanGreen + spanBlue * spanBlue;
  const perceptualSpanSquared = 0.2126 * spanRed * spanRed + 0.7152 * spanGreen * spanGreen + 0.0722 * spanBlue * spanBlue;
  return perceptualSpanSquared >= OWNED_PARITY_AA_MIN_CONTRAST_SQUARED && ownedParityCoverage(reference, candidate, offset, lowRed, lowGreen, lowBlue, spanRed, spanGreen, spanBlue, spanSquared);
}

/** 🎨️ Composites sRGB over white, halves pure alpha-coverage deltas, compares sqrt(0.299·dr² + 0.587·dg² + 0.114·db²), and suppresses only bounded shared-edge coverage. */
function compareOwnedParityPixels(referenceRgba: Uint8Array, candidateRgba: Uint8Array, diffRgba: Uint8Array, width: number, height: number, options: OwnedParityPixelOptions): number {
  if (!Number.isSafeInteger(width) || !Number.isSafeInteger(height) || width < 0 || height < 0) throw new Error("Owned parity pixel dimensions must be non-negative safe integers");
  const pixelCount = width * height;
  if (!Number.isSafeInteger(pixelCount) || pixelCount > Math.floor(Number.MAX_SAFE_INTEGER / 4)) throw new Error("Owned parity pixel dimensions exceed the safe byte range");
  const expectedBytes = pixelCount * 4;
  if (referenceRgba.length !== expectedBytes || candidateRgba.length !== expectedBytes || diffRgba.length !== expectedBytes) throw new Error(`Owned parity pixel buffers must each contain exactly ${expectedBytes} RGBA bytes`);
  if (!Number.isFinite(options.threshold) || options.threshold < 0 || options.threshold > 1) throw new Error("Owned parity pixel threshold must be finite and between zero and one");
  if (ownedParityByteRangesOverlap(diffRgba, referenceRgba) || ownedParityByteRangesOverlap(diffRgba, candidateRgba)) throw new Error("Owned parity pixel diff buffer must not overlap either read input");
  const thresholdSquared = options.threshold * options.threshold;
  let firstDifferentPixel = pixelCount;
  for (let pixel = 0; pixel < pixelCount; pixel++) {
    const offset = pixel * 4;
    if (referenceRgba[offset] !== candidateRgba[offset] || referenceRgba[offset + 1] !== candidateRgba[offset + 1] || referenceRgba[offset + 2] !== candidateRgba[offset + 2] || referenceRgba[offset + 3] !== candidateRgba[offset + 3]) {
      firstDifferentPixel = pixel;
      break;
    }
    ownedParityWriteMuted(referenceRgba, offset, diffRgba);
  }
  if (firstDifferentPixel === pixelCount) return 0;
  let mismatched = 0;
  for (let pixel = firstDifferentPixel; pixel < pixelCount; pixel++) {
    const offset = pixel * 4;
    if (referenceRgba[offset] === candidateRgba[offset] && referenceRgba[offset + 1] === candidateRgba[offset + 1] && referenceRgba[offset + 2] === candidateRgba[offset + 2] && referenceRgba[offset + 3] === candidateRgba[offset + 3]) {
      ownedParityWriteMuted(referenceRgba, offset, diffRgba);
      continue;
    }
    const distanceSquared = ownedParityDistanceSquared(referenceRgba, offset, candidateRgba, offset);
    if (distanceSquared <= thresholdSquared) {
      ownedParityWriteMuted(referenceRgba, offset, diffRgba);
      continue;
    }
    const y = Math.floor(pixel / width);
    const x = pixel - y * width;
    if (options.ignoreAntialiasing && ownedParityIsAntialiased(referenceRgba, candidateRgba, width, height, x, y, offset, thresholdSquared)) {
      ownedParityWriteMarker(diffRgba, offset, OWNED_PARITY_DIFF_ANTIALIAS);
      continue;
    }
    mismatched += 1;
    ownedParityWriteMarker(diffRgba, offset, OWNED_PARITY_DIFF_MISMATCH);
  }
  return mismatched;
}

function parityPixelThreshold(kind: string): number {
  return PARITY_SCENE_LEAF_KINDS.has(kind) ? PARITY_PIXEL_THRESHOLD_SCENE : PARITY_PIXEL_THRESHOLD_DEFAULT;
}

async function compareParityRegion(page: import("playwright").Page, reactPng: OwnedParityImage, wgpuPng: OwnedParityImage, node: ParityNode, outDir: string, variant: string): Promise<PixelRegionResult> {
  const [rx, ry, rw, rh] = node.rect;
  const width = Math.max(1, Math.min(Math.round(rw), reactPng.width - Math.round(rx), wgpuPng.width - Math.round(rx)));
  const height = Math.max(1, Math.min(Math.round(rh), reactPng.height - Math.round(ry), wgpuPng.height - Math.round(ry)));
  const threshold = parityPixelThreshold(node.kind);
  if (width <= 0 || height <= 0 || rx < 0 || ry < 0) return { path: node.path, ratio: 0, threshold };
  const reactCrop = cropOwnedParityRgba(reactPng, Math.round(rx), Math.round(ry), width, height);
  const wgpuCrop = cropOwnedParityRgba(wgpuPng, Math.round(rx), Math.round(ry), width, height);
  const diff = new Uint8Array(width * height * 4);
  const mismatched = compareOwnedParityPixels(reactCrop, wgpuCrop, diff, width, height, { threshold: 0.1, ignoreAntialiasing: true });
  const ratio = mismatched / (width * height);
  let diffPng: string | undefined;
  if (ratio > threshold) {
    diffPng = join(outDir, `diff-${variant}-${node.path.replace(/[^a-zA-Z0-9]+/g, "_")}.png`);
    writeFileSync(diffPng, await encodeParityDiff(page, diff, width, height));
  }
  return { path: node.path, ratio, threshold, diffPng };
}
//#endregion 🔖️PixelCompare

//#region 🔖️Triage
const PARITY_BOOT_TIMEOUT_MS = Number(process.env.PARITY_RUNTIME_BOOT_TIMEOUT_MS ?? 180_000);

/** 🧬️terra-parity-rebaseline: the exact TypeError `🌐plugin-web-materialize.ts`'s `loadActor` throws —
 * `const api = await bridge.createActorApi(actorId);` on a module whose export is `undefined` — once it
 * crosses `ShardClient.activate`'s reject (`🧵️shard-client.ts` `entry.reject(graftWorkerStack(...))`) and
 * surfaces as an unhandled rejection on the page. Matched on BOTH the property-access phrasing V8 uses
 * (`bridge.createActorApi is not a function` / `undefined is not an object (evaluating
 * 'bridge.createActorApi')`) and the bare symbol, so a wording change in one engine doesn't silently stop
 * matching in the other. Deliberately narrow — this must not catch unrelated "X is not a function" defects,
 * which are real regressions, not stale fixtures. */
const PARITY_STALE_BRIDGE_RE = /createActorApi/;

/** 🪜️Boot-triage ladder — each rung is a distinct terminal status, never conflated with a structural/pixel mismatch. */
async function triageParityBoot(page: import("playwright").Page, renderer: ParityRenderer, url: string): Promise<{ readonly status: BootStatus; readonly detail?: string }> {
  const pageErrors: string[] = [];
  page.on("pageerror", (e) => pageErrors.push(String(e)));
  page.on("console", (m) => {
    if (m.type() === "error") pageErrors.push(m.text());
  });
  const staleBridgeHit = (): string | undefined => pageErrors.find((e) => PARITY_STALE_BRIDGE_RE.test(e));
  try {
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: 30_000 });
  } catch (e) {
    return { status: "SERVER-FAIL", detail: String(e) };
  }
  if (renderer === "react") {
    try {
      await page.waitForFunction(() => document.querySelectorAll("#root *").length > 20, { timeout: PARITY_BOOT_TIMEOUT_MS });
    } catch {
      const stale = staleBridgeHit();
      return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "BOOT-TIMEOUT", detail: "react #root never populated" };
    }
    const nodeCount = await page.evaluate(() => document.querySelectorAll("[data-ui-path]").length);
    if (nodeCount === 0) {
      const stale = staleBridgeHit();
      return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "DUMP-EMPTY", detail: "no data-ui-path nodes" };
    }
    // 🩹️ The shell itself can mount (>20 root nodes, non-empty dump) while ONE plugin/extension actor
    // inside it fails to activate — that failure never blocks `#root`, so it must be checked even on an
    // otherwise-PASSing boot, or a stale-bridge variant silently reports PASS.
    const stale = staleBridgeHit();
    return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "PASS" };
  }
  try {
    await page.waitForFunction(() => document.querySelector("#semio-wgpu-canvas") != null, { timeout: PARITY_BOOT_TIMEOUT_MS });
  } catch {
    const stale = staleBridgeHit();
    return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "BOOT-TIMEOUT", detail: "wgpu canvas never mounted" };
  }
  if (pageErrors.some((e) => /NoCompatibleDevice|WebGPU/i.test(e))) return { status: "ENV-FAIL", detail: pageErrors.join(" | ") };
  try {
    await page.waitForFunction(() => typeof (window as unknown as { wasmBindings?: { dumpStructure?: unknown } }).wasmBindings?.dumpStructure === "function", { timeout: PARITY_BOOT_TIMEOUT_MS });
  } catch {
    const stale = staleBridgeHit();
    return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "BOOT-TIMEOUT", detail: "wgpu introspection hook never appeared" };
  }
  const dump = await dumpWgpuStructure(page);
  if (dump.nodes.length === 0) {
    const stale = staleBridgeHit();
    return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "DUMP-EMPTY", detail: "wgpu structural dump empty (plugin-bridge/kernel wiring)" };
  }
  const stats = await dumpWgpuFrameStats(page);
  if (stats && stats.drawCalls === 0) return { status: "BLANK-PAINT", detail: "zero draw calls (paint pipeline)" };
  const stale = staleBridgeHit();
  return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "PASS" };
}
//#endregion 🔖️Triage

//#region 🔖️ProbeCatalog
/** 🎬️Behavioral probe system — drives semantically-identical interactions on the react and wgpu
 * pages in lockstep (same click/type/key/drag/wheel sequence on both, each side resolving its OWN
 * click/drag/wheel coordinates from its OWN structural dump so the sequence stays semantically
 * identical even when pixel layout differs slightly) and diffs a fresh `compareParityStructural`
 * after every step. Complements `StructuralCompare`/`PixelCompare` (static end-state) and `Triage`
 * (boot) — this is the only sub-region that actually DRIVES interaction, closing the gap this
 * ticket's `verifyParityVariant` had: it previously only ever checked static boot state. */

type ProbeKeyCombo = string; // 🎹️ Playwright key-combo syntax, e.g. `"Control+p"`, `"Escape"`.

/** 🔎️`exists`/`absent`/`focus`/`text` match a node whose `path` equals OR case-insensitively
 * *contains* the given string (also checked against `kind`) — a probe author usually only knows the
 * semantic identifier ("search"), not the full generated structural path, and loose matching keeps
 * the DSL usable without every catalog entry hardcoding brittle exact paths. */
type ProbeExpectPredicate =
  | { readonly kind: "exists"; readonly path: string }
  | { readonly kind: "absent"; readonly path: string }
  | { readonly kind: "focus"; readonly path: string }
  | { readonly kind: "text"; readonly path: string; readonly equals: string }
  | { readonly kind: "custom"; readonly name: string; readonly check: (dump: ParityDump) => boolean };

type ProbeStep =
  | { readonly kind: "click"; readonly path: string }
  | { readonly kind: "type"; readonly text: string }
  | { readonly kind: "key"; readonly combo: ProbeKeyCombo }
  | { readonly kind: "dragTo"; readonly fromPath: string; readonly toPath: string }
  | { readonly kind: "wheel"; readonly path: string; readonly deltaY: number }
  | { readonly kind: "settle"; readonly ms: number }
  | { readonly kind: "stateTransition" }
  | { readonly kind: "expect"; readonly predicate: ProbeExpectPredicate };

type ProbeStateSnapshot = {
  readonly digest: string;
  readonly nodeCount: number;
};

type ProbeStateEvidence = {
  readonly actionPath: string;
  readonly actionKind: string;
  readonly react: { readonly before: ProbeStateSnapshot; readonly after: ProbeStateSnapshot; readonly changedPaths: readonly string[] };
  readonly wgpu: { readonly before: ProbeStateSnapshot; readonly after: ProbeStateSnapshot; readonly changedPaths: readonly string[] };
};

type ProbeStepStatus = "PASS" | "FAIL" | "SKIP";
type ProbeStepResult = {
  readonly index: number;
  readonly step: ProbeStep;
  readonly status: ProbeStepStatus;
  readonly structural?: StructuralResult;
  readonly state?: ProbeStateEvidence;
  readonly detail?: string;
};
type ProbeRunResult = { readonly status: ProbeStepStatus; readonly steps: readonly ProbeStepResult[] };
type ParityProbeSuite = { readonly name: string; readonly steps: readonly ProbeStep[] };

function parityRectCenter(rect: ParityRect): readonly [number, number] {
  const [x, y, w, h] = rect;
  return [x + w / 2, y + h / 2];
}

async function parityDumpFor(page: import("playwright").Page, renderer: ParityRenderer): Promise<ParityDump> {
  return renderer === "react" ? dumpReactStructure(page) : dumpWgpuStructure(page);
}

function parityFindNodeExact(dump: ParityDump, path: string): ParityNode | null {
  return dump.nodes.find((n) => n.path === path) ?? null;
}

function parityNodeMatches(dump: ParityDump, needle: string): readonly ParityNode[] {
  const lower = needle.toLowerCase();
  return dump.nodes.filter((n) => n.path === needle || n.path.toLowerCase().includes(lower) || n.kind.toLowerCase().includes(lower));
}

//#region 🔖️StateTransitionProbe
const STATE_PROBE_KIND_PRIORITY = ["toggle", "select", "slider", "button", "stack"] as const;
const STATE_PROBE_MAX_CANDIDATES = 12;

type StateProbeCandidate = { readonly path: string; readonly kind: string };

function stateProbeCandidates(reactDump: ParityDump, wgpuDump: ParityDump): StateProbeCandidate[] {
  const wgpuByPath = new Map(wgpuDump.nodes.map((node) => [node.path, node]));
  const priority = new Map<string, number>(STATE_PROBE_KIND_PRIORITY.map((kind, index) => [kind, index]));
  return reactDump.nodes
    .filter((node) => {
      const peer = wgpuByPath.get(node.path);
      return Boolean(
        peer &&
        node.path.includes("#") &&
        priority.has(node.kind) &&
        peer.kind === node.kind &&
        node.visible &&
        peer.visible &&
        !node.state.disabled &&
        !peer.state.disabled &&
        node.rect[2] > 0 &&
        node.rect[3] > 0 &&
        peer.rect[2] > 0 &&
        peer.rect[3] > 0,
      );
    })
    .map((node) => ({ path: node.path, kind: node.kind }))
    .sort((a, b) => (priority.get(a.kind) ?? 99) - (priority.get(b.kind) ?? 99) || a.path.localeCompare(b.path))
    .slice(0, STATE_PROBE_MAX_CANDIDATES);
}

function stateProbeNodeValue(node: ParityNode): string {
  return JSON.stringify({ path: node.path, kind: node.kind, text: parityNormalizeText(node.text), visible: node.visible, disabled: node.state.disabled, selected: node.state.selected });
}

function stateProbeSnapshot(dump: ParityDump): ProbeStateSnapshot {
  const serialized = dump.nodes.map(stateProbeNodeValue).sort().join("\n");
  return { digest: Bun.hash(serialized).toString(16), nodeCount: dump.nodes.length };
}

function stateProbeChangedPaths(before: ParityDump, after: ParityDump): string[] {
  const beforeByPath = new Map(before.nodes.map((node) => [node.path, stateProbeNodeValue(node)]));
  const afterByPath = new Map(after.nodes.map((node) => [node.path, stateProbeNodeValue(node)]));
  const paths = new Set([...beforeByPath.keys(), ...afterByPath.keys()]);
  return [...paths].filter((path) => beforeByPath.get(path) !== afterByPath.get(path)).sort();
}

async function executeStateProbeCandidate(page: import("playwright").Page, renderer: ParityRenderer, candidate: StateProbeCandidate): Promise<{ readonly ok: boolean; readonly detail?: string }> {
  const click = await executeParityStep(page, renderer, { kind: "click", path: candidate.path });
  if (!click.ok) return click;
  if (candidate.kind === "select") {
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
  } else if (candidate.kind === "slider") {
    await page.keyboard.press("ArrowRight");
  }
  return { ok: true };
}

/** 🧭️ Drives an app-declared interactive `UiNode` shared by both renderers and proves that each
 * renderer observes a semantic state change (topology, text, visibility, disabled, or selected).
 * Framework chrome is excluded because only interpreter-owned `data-ui-path`/wgpu paths participate;
 * `#id` further requires an explicit app declaration rather than an anonymous layout node. */
async function runStateTransitionProbe(reactPage: import("playwright").Page, wgpuPage: import("playwright").Page): Promise<{ readonly status: ProbeStepStatus; readonly state?: ProbeStateEvidence; readonly detail?: string }> {
  let reactBefore = await dumpReactStructure(reactPage);
  let wgpuBefore = await dumpWgpuStructure(wgpuPage);
  const candidates = stateProbeCandidates(reactBefore, wgpuBefore);
  if (candidates.length === 0) return { status: "SKIP", detail: "no common enabled app-declared toggle/select/slider/button/activatable-stack node" };

  const attempted: string[] = [];
  for (const candidate of candidates) {
    const [reactAction, wgpuAction] = await Promise.all([executeStateProbeCandidate(reactPage, "react", candidate), executeStateProbeCandidate(wgpuPage, "wgpu", candidate)]);
    if (!reactAction.ok || !wgpuAction.ok) {
      attempted.push(`${candidate.path}: ${reactAction.detail ?? "react ok"}; ${wgpuAction.detail ?? "wgpu ok"}`);
      continue;
    }
    await Promise.all([reactPage.waitForTimeout(350), wgpuPage.waitForTimeout(350)]);
    const [reactAfter, wgpuAfter] = await Promise.all([dumpReactStructure(reactPage), dumpWgpuStructure(wgpuPage)]);
    const reactChangedPaths = stateProbeChangedPaths(reactBefore, reactAfter);
    const wgpuChangedPaths = stateProbeChangedPaths(wgpuBefore, wgpuAfter);
    const evidence: ProbeStateEvidence = {
      actionPath: candidate.path,
      actionKind: candidate.kind,
      react: { before: stateProbeSnapshot(reactBefore), after: stateProbeSnapshot(reactAfter), changedPaths: reactChangedPaths },
      wgpu: { before: stateProbeSnapshot(wgpuBefore), after: stateProbeSnapshot(wgpuAfter), changedPaths: wgpuChangedPaths },
    };
    if (reactChangedPaths.length > 0 && wgpuChangedPaths.length > 0) return { status: "PASS", state: evidence };
    attempted.push(`${candidate.path}: react changed=${reactChangedPaths.length}, wgpu changed=${wgpuChangedPaths.length}`);
    reactBefore = reactAfter;
    wgpuBefore = wgpuAfter;
  }
  return { status: "FAIL", detail: `no candidate produced observable state on both renderers (${attempted.join(" | ")})` };
}
//#endregion 🔖️StateTransitionProbe

/** 🕹️Executes one non-`expect` step against a single page, resolving click/drag/wheel targets from
 * a dump pulled from THAT SAME page immediately beforehand — never the other renderer's dump, and
 * never a stale one — so react/wgpu layout drift never desyncs which element gets hit. */
async function executeParityStep(page: import("playwright").Page, renderer: ParityRenderer, step: Exclude<ProbeStep, { readonly kind: "expect" } | { readonly kind: "stateTransition" }>): Promise<{ readonly ok: boolean; readonly detail?: string }> {
  switch (step.kind) {
    case "click": {
      const node = parityFindNodeExact(await parityDumpFor(page, renderer), step.path);
      if (!node) return { ok: false, detail: `click target not found: ${step.path}` };
      const [cx, cy] = parityRectCenter(node.rect);
      await page.mouse.click(cx, cy);
      return { ok: true };
    }
    case "type":
      await page.keyboard.type(step.text);
      return { ok: true };
    case "key":
      await page.keyboard.press(step.combo);
      return { ok: true };
    case "dragTo": {
      const dump = await parityDumpFor(page, renderer);
      const from = parityFindNodeExact(dump, step.fromPath);
      const to = parityFindNodeExact(dump, step.toPath);
      if (!from || !to) return { ok: false, detail: `dragTo target not found: ${!from ? step.fromPath : step.toPath}` };
      const [fx, fy] = parityRectCenter(from.rect);
      const [tx, ty] = parityRectCenter(to.rect);
      await page.mouse.move(fx, fy);
      await page.mouse.down();
      await page.mouse.move(tx, ty, { steps: 8 });
      await page.mouse.up();
      return { ok: true };
    }
    case "wheel": {
      const node = parityFindNodeExact(await parityDumpFor(page, renderer), step.path);
      if (!node) return { ok: false, detail: `wheel target not found: ${step.path}` };
      const [cx, cy] = parityRectCenter(node.rect);
      await page.mouse.move(cx, cy);
      await page.mouse.wheel(0, step.deltaY);
      return { ok: true };
    }
    case "settle":
      await page.waitForTimeout(step.ms);
      return { ok: true };
  }
}

/** ✅️Evaluates one `expect` predicate against BOTH sides' freshly-pulled dumps — a predicate only
 * passes the step when it holds on react AND wgpu, since the point is cross-renderer parity, not
 * either renderer in isolation. */
function evaluateParityExpect(predicate: ProbeExpectPredicate, reactDump: ParityDump, wgpuDump: ParityDump): { readonly ok: boolean; readonly detail?: string } {
  const checkOne = (dump: ParityDump): { readonly ok: boolean; readonly detail?: string } => {
    switch (predicate.kind) {
      case "exists": {
        const ok = parityNodeMatches(dump, predicate.path).length > 0;
        return { ok, detail: ok ? undefined : `no node matching "${predicate.path}"` };
      }
      case "absent": {
        const ok = parityNodeMatches(dump, predicate.path).length === 0;
        return { ok, detail: ok ? undefined : `node still present matching "${predicate.path}"` };
      }
      case "focus": {
        const focus = dump.focusPath;
        const ok = focus !== null && (focus === predicate.path || focus.toLowerCase().includes(predicate.path.toLowerCase()));
        return { ok, detail: ok ? undefined : `focusPath "${focus ?? "null"}" does not match "${predicate.path}"` };
      }
      case "text": {
        const node = parityNodeMatches(dump, predicate.path)[0];
        const ok = node !== undefined && parityNormalizeText(node.text) === parityNormalizeText(predicate.equals);
        return { ok, detail: ok ? undefined : `text at "${predicate.path}" is "${node?.text ?? "<missing>"}", expected "${predicate.equals}"` };
      }
      case "custom": {
        const ok = predicate.check(dump);
        return { ok, detail: ok ? undefined : `custom predicate "${predicate.name}" failed` };
      }
    }
  };
  const react = checkOne(reactDump);
  const wgpu = checkOne(wgpuDump);
  const ok = react.ok && wgpu.ok;
  return { ok, detail: ok ? undefined : `react: ${react.detail ?? "ok"} | wgpu: ${wgpu.detail ?? "ok"}` };
}

/** 🏃️Runs `steps` on `reactPage`/`wgpuPage` in lockstep — never advances to the next step on either
 * page until the current one finished on both. Non-`expect` steps execute identically on both pages
 * then get a fresh `compareParityStructural` diff; `expect` steps take no page action and just
 * evaluate their predicate against fresh dumps from both. The first `FAIL` halts the run (remaining
 * steps marked `SKIP`) — steps are an ORDERED scenario, not a bag of independent assertions, so a
 * downstream step referencing state a failed step never reached would only add noise. Returns the
 * FULL step trail (not just a final boolean) so a failure is diagnosable by (which step, which axis)
 * — see `ParityMismatchAxis` for the axis vocabulary reused from `StructuralCompare`. */
async function runParityProbe(reactPage: import("playwright").Page, wgpuPage: import("playwright").Page, steps: readonly ProbeStep[]): Promise<ProbeRunResult> {
  const results: ProbeStepResult[] = [];
  let halted = false;
  for (let index = 0; index < steps.length; index++) {
    const step = steps[index];
    if (halted) {
      results.push({ index, step, status: "SKIP" });
      continue;
    }
    if (step.kind === "expect") {
      const reactDump = await dumpReactStructure(reactPage);
      const wgpuDump = await dumpWgpuStructure(wgpuPage);
      const outcome = evaluateParityExpect(step.predicate, reactDump, wgpuDump);
      results.push({ index, step, status: outcome.ok ? "PASS" : "FAIL", detail: outcome.detail });
      if (!outcome.ok) halted = true;
      continue;
    }
    if (step.kind === "stateTransition") {
      const outcome = await runStateTransitionProbe(reactPage, wgpuPage);
      results.push({ index, step, status: outcome.status, state: outcome.state, detail: outcome.detail });
      if (outcome.status === "FAIL") halted = true;
      continue;
    }
    const [reactOutcome, wgpuOutcome] = await Promise.all([executeParityStep(reactPage, "react", step), executeParityStep(wgpuPage, "wgpu", step)]);
    if (!reactOutcome.ok || !wgpuOutcome.ok) {
      results.push({ index, step, status: "FAIL", detail: [reactOutcome.detail, wgpuOutcome.detail].filter(Boolean).join(" | ") });
      halted = true;
      continue;
    }
    const reactDump = await dumpReactStructure(reactPage);
    const wgpuDump = await dumpWgpuStructure(wgpuPage);
    const structural = compareParityStructural(reactDump, wgpuDump);
    results.push({ index, step, status: structural.status, structural });
    if (structural.status === "FAIL") halted = true;
  }
  const status = results.some((r) => r.status === "FAIL") ? "FAIL" : results.some((r) => r.status === "PASS") ? "PASS" : "SKIP";
  return { status, steps: results };
}

async function runParityProbeSuite(reactPage: import("playwright").Page, wgpuPage: import("playwright").Page, suite: ParityProbeSuite): Promise<{ readonly name: string } & ProbeRunResult> {
  const result = await runParityProbe(reactPage, wgpuPage, suite.steps);
  return { name: suite.name, ...result };
}

/** 🐚️Minimal cross-playground smoke suite — command palette open/close is the one interaction every
 * catalog playground exposes IDENTICALLY, via `useActionHotkey("mod+p", ...)` in
 * `framework/os/renderer/js/react/index.tsx` (`mod` accepts `ctrlKey || metaKey`, so `"Control+p"` works
 * regardless of host OS — no need to special-case macOS `"Meta+p"`).
 *
 * KNOWN LIMITATION (confirmed by reading `openStudioE2eCommandPalette` in `🔖️StudioE2eVerify` above,
 * and `UISearch` in `framework/os/renderer/js/react/index.tsx`): the palette is FRAMEWORK CHROME, not
 * `UiNode`-declared app content — React renders it through the owned Command facade (`[role='dialog'] [data-slot=
 * 'command-input']`), which never carries `data-ui-path`, so `REACT_DOM_DUMP_SCRIPT` (see
 * `🔖️StructuralDump`) cannot see it at all. The `exists`/`absent` checks below are therefore
 * expected to be unreliable (likely FAIL on the react side) until the structural dump is extended to
 * also tag framework-chrome overlays — a real, scoped follow-up (would also need mirroring into
 * `framework/os/renderer/wgpu/rs/lib.rs`'s `🔬️Introspection` walk, which is a different file, out of
 * reach from this one). Flagging rather than silently "fixing" by touching either renderer's core
 * dump mechanism unverified, per this pass's own constraint of no live browser run to confirm
 * against. */
const PARITY_SHELL_PROBE_SUITE: ParityProbeSuite = {
  name: "shell",
  steps: [
    { kind: "key", combo: "Control+p" },
    { kind: "settle", ms: 200 },
    { kind: "expect", predicate: { kind: "exists", path: "search" } },
    { kind: "key", combo: "Escape" },
    { kind: "settle", ms: 200 },
    { kind: "expect", predicate: { kind: "absent", path: "search" } },
  ],
};

/** 🧭️Default catalog-wide state-management probe. Unlike `shell`, this drives an explicitly-id'd
 * app surface node and records renderer-specific before/after digests plus every changed path. */
const PARITY_STATE_PROBE_SUITE: ParityProbeSuite = {
  name: "state",
  steps: [{ kind: "stateTransition" }],
};

/** 🗂️Starter catalog — keyed by suite name so `ParityProbeScript`/`verifyParityVariant` can look one
 * up by string. A per-playground text/dnd/scene suite (dragging dock panels, typing into a text
 * editor host, orbiting a 3d scene) is a natural follow-up once `shell` is confirmed working
 * end-to-end against a real live boot — out of scope for this pass per the ticket's own brief. */
const PARITY_PROBE_CATALOG: Readonly<Record<string, ParityProbeSuite>> = {
  state: PARITY_STATE_PROBE_SUITE,
  shell: PARITY_SHELL_PROBE_SUITE,
};
//#endregion 🔖️ProbeCatalog

//#region 🔖️ServerPool
/** 🔌️Harness dev-server pool — clear of the catalog's per-variant 6012–6205 ports so a sweep never
 * collides with another concurrent dev's running playground. One react+wgpu port pair per shard,
 * reused (restart-between-variants) across that shard's playground list. React bakes its program
 * choice at boot via `VITE_SEMIO_PLUGIN` (no runtime `?query=` switch — see `js/index.ts`), so a
 * fresh server per variant is required on both renderers, not just wgpu. */
const PARITY_PORT_BASE = 7300;

function parityPortsForShard(shardIndex: number): { readonly react: number; readonly wgpu: number } {
  const base = PARITY_PORT_BASE + shardIndex * 2;
  return { react: base, wgpu: base + 1 };
}

const PARITY_PORT_POOL_SHARDS = 49; // (7398 - 7300) / 2

/** 🔌️`smoke`/`triage`/`verify` are meant to be run by multiple concurrent agents/sessions — hardcoding
 * shard 0 meant every concurrent invocation collided on the same 7300/7301 pair, producing false
 * `SERVER-FAIL`/`DUMP-EMPTY` results indistinguishable from real failures (found the hard way: several
 * parallel boot-triage agents hit exactly this). Scans the shard pool for the first pair where BOTH
 * ports are actually free right now — no coordination needed between callers. */
function findFreeParityPortPair(): { readonly react: number; readonly wgpu: number } {
  for (let shard = 0; shard < PARITY_PORT_POOL_SHARDS; shard++) {
    const candidate = parityPortsForShard(shard);
    if (!isDevPortInUse("127.0.0.1", candidate.react) && !isDevPortInUse("127.0.0.1", candidate.wgpu)) return candidate;
  }
  throw new Error(`no free parity port pair in the ${PARITY_PORT_POOL_SHARDS}-shard pool (${PARITY_PORT_BASE}-${PARITY_PORT_BASE + PARITY_PORT_POOL_SHARDS * 2})`);
}

function parityDevUrl(renderer: ParityRenderer, variant: string, port: number): string {
  return renderer === "wgpu" ? wgpuDevPlayUrl("127.0.0.1", port, variant) : devServerUrl("127.0.0.1", port);
}

type ParityServerHandle = { readonly daemon: SpawnDaemonHandle; readonly port: number };

/** ⏱️A cold `bun ./📜️script.ts dev` boot can mean compiling the ENTIRE plugin crate catalog (33 crates)
 * plus, for wgpu, a from-scratch trunk/cargo build — many minutes with an empty `target/`, not the
 * ~40-60s a warm-cache boot takes. Default generously; `PARITY_BOOT_BUDGET_MS` overrides for CI/tuning. */
const PARITY_DEV_SERVER_BOOT_BUDGET_MS = Number(process.env.PARITY_BOOT_BUDGET_MS ?? 900_000);

/** 🧱️ Builds and stages one variant exactly once before either renderer starts. React's normal
 * streaming boot and WGPU's blocking boot would otherwise launch duplicate Cargo builds against the
 * shared target directory, spend most of their budget on file locks, and expose a listening port
 * before the app program exists. */
async function prebuildParityPlugin(variant: string): Promise<void> {
  const devScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts");
  const logPath = join(parityOutDir(), `prebuild-${variant}.log`);
  const lockRoot = resolve(process.env.PARITY_CARGO_TARGET_DIR ?? parityOutDir());
  const lockPath = join(lockRoot, ".semio-parity-prebuild-lock");
  mkdirSync(lockRoot, { recursive: true });
  const lockDeadline = Date.now() + PARITY_DEV_SERVER_BOOT_BUDGET_MS;
  while (true) {
    try {
      mkdirSync(lockPath);
      break;
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      if (Date.now() >= lockDeadline) throw new Error(`plugin prebuild lock for ${variant} exceeded ${PARITY_DEV_SERVER_BOOT_BUDGET_MS}ms (${lockPath})`);
      // 🏛️ THE RULE (see 🔖️PollHelpers above): legitimate poll, not routed through a helper — this is
      // a cross-process `mkdir`-as-mutex over a shared target dir; the lock's holder may be a wholly
      // separate `parity` invocation this process never spawned and has no pid/handle/event for. A
      // TCP/HTTP helper would not fit this shape (no port, no HTTP endpoint) even if we wanted one.
      await Bun.sleep(500);
    }
  }
  try {
    const logStream = createWriteStream(logPath);
    const daemon = spawnDaemon("bun", [devScript, "plugin", variant], {
      cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript"),
      env: {
        ...process.env,
        SEMIO_PLUGIN: variant,
        CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? "4",
        ...(process.env.PARITY_CARGO_TARGET_DIR ? { CARGO_TARGET_DIR: process.env.PARITY_CARGO_TARGET_DIR } : {}),
      },
      stdio: "pipe",
    });
    daemon.child.stdout?.pipe(logStream);
    daemon.child.stderr?.pipe(logStream);
    // 🧵️ We hold `daemon.child` — await its `exit` event via `awaitChildExit` (THE RULE above)
    // instead of polling `exitCode`. Same budget as before.
    const exited = await awaitChildExit(daemon.child, PARITY_DEV_SERVER_BOOT_BUDGET_MS);
    if (exited === "timeout") {
      daemon.kill();
      logStream.end();
      throw new Error(`plugin prebuild for ${variant} exceeded ${PARITY_DEV_SERVER_BOOT_BUDGET_MS}ms (see ${logPath})`);
    }
    logStream.end();
    if (daemon.child.exitCode !== 0) throw new Error(`plugin prebuild for ${variant} failed with code ${daemon.child.exitCode} (see ${logPath})`);
  } finally {
    rmSync(lockPath, { recursive: true, force: true });
  }
}

async function startParityDevServer(renderer: ParityRenderer, variant: string, port: number): Promise<ParityServerHandle> {
  const devScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts");
  const logPath = join(parityOutDir(), `boot-${renderer}-${variant}.log`);
  const logStream = createWriteStream(logPath);
  const daemon = spawnDaemon("bun", [devScript, "dev"], {
    cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript"),
    env: {
      ...process.env,
      SKIP_PLUGIN_BUILD: "1",
      SEMIO_PLUGIN: variant,
      SEMIO_RENDERER: renderer,
      SEMIO_PARITY_QUIET_CARGO: "1",
      S_OS_PORT: String(port),
      CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? "4",
      ...(process.env.PARITY_CARGO_TARGET_DIR ? { CARGO_TARGET_DIR: process.env.PARITY_CARGO_TARGET_DIR } : {}),
    },
    stdio: "pipe",
  });
  daemon.child.stdout?.pipe(logStream);
  daemon.child.stderr?.pipe(logStream);
  const outcome = await awaitTcpReady("127.0.0.1", port, {
    deadlineMs: PARITY_DEV_SERVER_BOOT_BUDGET_MS,
    intervalMs: 500,
    isDead: () => daemon.child.exitCode !== null,
  });
  if (outcome === "ready") return { daemon, port };
  if (outcome === "dead") throw new Error(`${renderer} dev server for ${variant} exited early (code ${daemon.child.exitCode}) — see ${logPath}`);
  daemon.kill();
  throw new Error(`${renderer} dev server for ${variant} did not open port ${port} within ${PARITY_DEV_SERVER_BOOT_BUDGET_MS}ms — see ${logPath}`);
}

/** 🧹️Best-effort: kills the spawned wrapper AND whatever ends up bound to the port, since vite/trunk
 * fork their own child processes that a plain wrapper-kill doesn't always reap. */
function stopParityDevServer(handle: ParityServerHandle): void {
  try {
    handle.daemon.kill();
  } catch {
    /* already gone */
  }
  const occupant = describeDevPortOccupant(handle.port);
  const pid = Number(occupant?.match(/PID (\d+)/)?.[1]);
  if (Number.isFinite(pid)) {
    try {
      process.kill(pid, "SIGTERM");
    } catch {
      /* already gone */
    }
  }
}
//#endregion 🔖️ServerPool

//#region 🔖️Report
function parityOutDir(): string {
  const configured = process.env.PARITY_OUT_DIR ?? ".🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY";
  const dir = resolve(repoRoot, configured);
  mkdirSync(dir, { recursive: true });
  return dir;
}

/** 🪜️terra-parity-rebaseline: a playground is `STALE-BRIDGE` if EITHER renderer's boot rung landed
 * there — the point is "not a real regression, don't count it against the architecture", and either
 * side hitting it is sufficient to know that. Checked before the generic `failed` filter below so a
 * stale-bridge variant is never double-counted as both. */
function isParityStaleBridge(r: ParityPlaygroundReport): boolean {
  return r.boot.react === "STALE-BRIDGE" || r.boot.wgpu === "STALE-BRIDGE";
}

function writeParityReport(reports: readonly ParityPlaygroundReport[]): void {
  const outDir = parityOutDir();
  writeFileSync(join(outDir, "parity-report-v2.json"), JSON.stringify(reports, null, 2), "utf8");
  const lines = ["# Wgpu Parity Report (v2 harness)", "", `Generated: ${reports.length} playground(s)`, "", "| Variant | React Boot | Wgpu Boot | Structural | Pixel | State | Action | React Δ | Wgpu Δ |", "|---|---|---|---|---|---|---|---:|---:|"];
  for (const r of reports) {
    const evidence = r.behavioral?.steps.find((step) => step.state)?.state;
    lines.push(
      `| ${r.variant} | ${r.boot.react} | ${r.boot.wgpu} | ${r.structural?.status ?? "-"} | ${r.pixel?.status ?? "-"} | ${r.behavioral?.status ?? "-"} | ${evidence ? `\`${evidence.actionKind}\` \`${evidence.actionPath}\`` : "-"} | ${evidence?.react.changedPaths.length ?? "-"} | ${evidence?.wgpu.changedPaths.length ?? "-"} |`,
    );
  }
  // 🪜️terra-parity-rebaseline: STALE-BRIDGE split OUT of `failed` — see `isParityStaleBridge`'s doc. A
  // blended "X/Y PASS" line conflates "the architecture regressed" with "the fleet hasn't regenerated
  // this bridge yet", which is exactly the false-regression risk 📌️important.md's re-baseline task
  // called out; the three-way split below is what makes a re-run after `sdk-green` lands legible.
  const staleBridge = reports.filter(isParityStaleBridge);
  const failed = reports.filter((r) => !isParityStaleBridge(r) && (r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL"));
  const passed = reports.length - failed.length - staleBridge.length;
  lines.push("", `**${passed}/${reports.length} PASS · ${staleBridge.length}/${reports.length} STALE-BRIDGE (excluded from the architecture verdict) · ${failed.length}/${reports.length} FAIL**`);
  if (staleBridge.length > 0) lines.push("", `Stale-bridge variants (expected until the fleet's own bridge regenerates — 📌️important.md): ${staleBridge.map((r) => r.variant).join(", ")}`);
  writeFileSync(join(outDir, "parity-report-v2.md"), lines.join("\n"), "utf8");
}
//#endregion 🔖️Report

//#region 🔖️Sweep
/** 🎭️ Points playwright at the repo-local browser cache that `📜️script.ts setup` actually populates
 * (`bunx playwright install --with-deps chromium` → `node_modules/.cache/ms-playwright`). Without
 * this, `chromium.launch()` falls back to the user-global `~/Library/Caches/ms-playwright`, which
 * holds whatever an unrelated project installed — here a stale `chromium_headless_shell-1223`
 * against the required `-1234` — and every parity run dies with "Executable doesn't exist"
 * suggesting `npx playwright install`, i.e. a download, for a browser the repo had already
 * installed. The storybook runner (root `📜️script.ts`, `🔖️TestScript`) already sets this.
 *
 * terra-parity-rebaseline: hoisted out of `verifyParityVariant` (the only call site that set this
 * before today) into a shared helper, and now ALSO called from `ParityTriageScript`/`ParityProbeScript`
 * — both launch `chromium` directly without going through `verifyParityVariant` and were dying on the
 * exact same stale-global-cache error, which is what made even a single `parity triage <variant>`
 * unrunnable (measured: `Executable doesn't exist at .../ms-playwright/chromium_headless_shell-1234/...`,
 * exit 1) before this fix — never mind a 58-variant sweep. */
function ensureParityPlaywrightBrowsersPath(): void {
  process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(repoRoot, "node_modules", ".cache", "ms-playwright");
}

async function verifyParityVariant(variant: string, ports: { readonly react: number; readonly wgpu: number }, opts: { readonly skipDev?: boolean } = {}): Promise<ParityPlaygroundReport> {
  const start = Date.now();
  ensureParityPlaywrightBrowsersPath();
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
  let reactServer: ParityServerHandle | undefined;
  let wgpuServer: ParityServerHandle | undefined;
  try {
    if (!opts.skipDev) {
      await prebuildParityPlugin(variant);
      reactServer = await startParityDevServer("react", variant, ports.react);
      wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
    }
    const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
    const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
    const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
    const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
    if (reactBoot.status !== "PASS" || wgpuBoot.status !== "PASS") {
      return { variant, boot: { react: reactBoot.status, wgpu: wgpuBoot.status, detail: reactBoot.detail ?? wgpuBoot.detail }, durationMs: Date.now() - start };
    }
    await reactPage.waitForTimeout(400);
    await wgpuPage.waitForTimeout(400);
    const reactDump = await dumpReactStructure(reactPage);
    const wgpuDump = await dumpWgpuStructure(wgpuPage);
    const structural = compareParityStructural(reactDump, wgpuDump);
    const outDir = parityOutDir();
    const reactPng = await decodeParityScreenshot(reactPage, await reactPage.screenshot());
    const wgpuPng = await decodeParityScreenshot(wgpuPage, await wgpuPage.screenshot());
    const wgpuPaths = new Set(wgpuDump.nodes.map((n) => n.path));
    const regionNodes = reactDump.nodes.filter((n) => PARITY_PIXEL_REGION_KINDS.has(n.kind) && wgpuPaths.has(n.path));
    const regions = await Promise.all(regionNodes.map((n) => compareParityRegion(reactPage, reactPng, wgpuPng, n, outDir, variant)));
    const failingRegions = regions.filter((r) => r.ratio > r.threshold);
    // 🎬️Runs regardless of the structural/pixel outcome above (not gated on their PASS) — behavioral
    // parity is a distinct axis (interaction-driven dynamic state vs. static end-state), and a
    // static mismatch elsewhere shouldn't hide whether app state still transitions correctly. Wrapped
    // defensively: a probe-runner exception (e.g. a page closing mid-step) must not take down the
    // whole `verifyParityVariant` call, only degrade `behavioral` to a diagnosable FAIL.
    let behavioral: ProbeRunResult | undefined;
    try {
      behavioral = await runParityProbe(reactPage, wgpuPage, PARITY_STATE_PROBE_SUITE.steps);
    } catch (e) {
      behavioral = { status: "FAIL", steps: [{ index: 0, step: { kind: "settle", ms: 0 }, status: "FAIL", detail: `probe runner threw: ${String(e)}` }] };
    }
    return {
      variant,
      boot: { react: reactBoot.status, wgpu: wgpuBoot.status },
      structural,
      pixel: { status: failingRegions.length === 0 ? "PASS" : "FAIL", regions: failingRegions },
      behavioral,
      durationMs: Date.now() - start,
    };
  } finally {
    await browser.close();
    if (reactServer) stopParityDevServer(reactServer);
    if (wgpuServer) stopParityDevServer(wgpuServer);
  }
}

class ParitySmokeScript extends BundleScript {
  async run(): Promise<void> {
    const variant = process.env.SEMIO_PLUGIN || DEFAULT_HOST_VARIANT;
    const report = await verifyParityVariant(variant, findFreeParityPortPair());
    console.log(JSON.stringify(report, null, 2));
    if (report.boot.react !== "PASS" || report.boot.wgpu !== "PASS") {
      throw new Error(`parity smoke FAILED: boot react=${report.boot.react} wgpu=${report.boot.wgpu}${report.boot.detail ? ` (${report.boot.detail})` : ""}`);
    }
    console.log(`parity smoke PASS for ${variant}: structural=${report.structural?.status} pixel=${report.pixel?.status} behavioral=${report.behavioral?.status} (${report.durationMs}ms)`);
  }
}

class ParityTriageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variant = segments[0] || process.env.SEMIO_PLUGIN || DEFAULT_HOST_VARIANT;
    const ports = findFreeParityPortPair();
    ensureParityPlaywrightBrowsersPath();
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
    // 🩹️terra-parity-rebaseline: `reactServer`/`wgpuServer` used to be `const`, ASSIGNED BEFORE this
    // `try`, so a throw from the SECOND `startParityDevServer` call (e.g. wgpu's cold cargo build
    // exceeding `PARITY_BOOT_BUDGET_MS`) left the first server's process running with nothing left
    // holding a reference to stop it — confirmed leaked in practice (a react `vite` + a wgpu `trunk`
    // process both still bound to their ports well after the command had exited). `let` + assignment
    // INSIDE the try, guarded in `finally`, is the same safe shape `verifyParityVariant` already uses.
    let reactServer: ParityServerHandle | undefined;
    let wgpuServer: ParityServerHandle | undefined;
    try {
      reactServer = await startParityDevServer("react", variant, ports.react);
      wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
      const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
      const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
      console.log(`triage ${variant}: react=${reactBoot.status}${reactBoot.detail ? ` (${reactBoot.detail})` : ""}`);
      console.log(`triage ${variant}: wgpu=${wgpuBoot.status}${wgpuBoot.detail ? ` (${wgpuBoot.detail})` : ""}`);
    } finally {
      await browser.close();
      if (reactServer) stopParityDevServer(reactServer);
      if (wgpuServer) stopParityDevServer(wgpuServer);
    }
  }
}

/** 🎬️Standalone entry point for JUST the behavioral probe suite — boots both dev servers, triages
 * boot, then runs `PARITY_PROBE_CATALOG[suiteName]` (default `"shell"`) without paying for the
 * structural/pixel comparison `verifyParityVariant` also does. Useful for iterating on a probe suite
 * itself without re-running the (slower) full `verify`. */
class ParityProbeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variant = segments[0] || process.env.SEMIO_PLUGIN || DEFAULT_HOST_VARIANT;
    const suiteName = segments[1] || "state";
    const suite = PARITY_PROBE_CATALOG[suiteName];
    if (!suite) throw new Error(`unknown probe suite: ${suiteName} (known: ${Object.keys(PARITY_PROBE_CATALOG).join(", ")})`);
    const ports = findFreeParityPortPair();
    ensureParityPlaywrightBrowsersPath();
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
    // 🩹️terra-parity-rebaseline: same leak fix as `ParityTriageScript` above — see its comment.
    let reactServer: ParityServerHandle | undefined;
    let wgpuServer: ParityServerHandle | undefined;
    try {
      reactServer = await startParityDevServer("react", variant, ports.react);
      wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
      const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
      const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
      if (reactBoot.status !== "PASS" || wgpuBoot.status !== "PASS") {
        throw new Error(`parity probe FAILED: boot react=${reactBoot.status} wgpu=${wgpuBoot.status}${(reactBoot.detail ?? wgpuBoot.detail) ? ` (${reactBoot.detail ?? wgpuBoot.detail})` : ""}`);
      }
      const result = await runParityProbeSuite(reactPage, wgpuPage, suite);
      console.log(JSON.stringify(result, null, 2));
      console.log(`probe ${variant}/${suiteName}: ${result.status} (${result.steps.length} step(s))`);
      if (result.status !== "PASS") throw new Error(`parity probe ${variant}/${suiteName} FAILED`);
    } finally {
      await browser.close();
      if (reactServer) stopParityDevServer(reactServer);
      if (wgpuServer) stopParityDevServer(wgpuServer);
    }
  }
}

class ParityVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variants = segments.filter((s) => !s.startsWith("--"));
    if (variants.length === 0) throw new Error("usage: parity verify <variant…>");
    const skipDev = process.env.SKIP_DEV === "1";
    const ports = skipDev ? parityPortsForShard(0) : findFreeParityPortPair();
    const reports: ParityPlaygroundReport[] = [];
    for (const variant of variants) {
      const report = await verifyParityVariant(variant, ports, { skipDev });
      reports.push(report);
      console.log(`${variant}: boot=${report.boot.react}/${report.boot.wgpu} structural=${report.structural?.status ?? "-"} pixel=${report.pixel?.status ?? "-"} behavioral=${report.behavioral?.status ?? "-"}`);
    }
    writeParityReport(reports);
    // 🪜️terra-parity-rebaseline: STALE-BRIDGE excluded — see `isParityStaleBridge`'s doc on `writeParityReport`.
    const staleBridge = reports.filter(isParityStaleBridge);
    const failed = reports.filter((r) => !isParityStaleBridge(r) && (r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL"));
    if (staleBridge.length > 0) console.log(`parity verify: ${staleBridge.length}/${reports.length} STALE-BRIDGE, excluded from the pass/fail verdict: ${staleBridge.map((r) => r.variant).join(", ")}`);
    if (failed.length > 0) throw new Error(`parity verify: ${failed.length}/${reports.length} playground(s) failed`);
  }
}

class ParitySweepScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const shardArg = segments.find((s) => s.startsWith("--shard="))?.slice("--shard=".length);
    const [shardIndex, shardCount] = shardArg ? shardArg.split("/").map(Number) : [0, 1];
    const variants = playgroundCatalog.map((r) => r.variant).filter((_, i) => i % (shardCount ?? 1) === (shardIndex ?? 0));
    const reports: ParityPlaygroundReport[] = [];
    for (const variant of variants) {
      let report: ParityPlaygroundReport;
      try {
        report = await verifyParityVariant(variant, parityPortsForShard(shardIndex ?? 0));
      } catch (error) {
        report = { variant, boot: { react: "SERVER-FAIL", wgpu: "SERVER-FAIL", detail: String(error) }, durationMs: 0 };
      }
      reports.push(report);
      console.log(`sweep ${variant}: boot=${report.boot.react}/${report.boot.wgpu} structural=${report.structural?.status ?? "-"} pixel=${report.pixel?.status ?? "-"} behavioral=${report.behavioral?.status ?? "-"}`);
    }
    writeParityReport(reports);
    // 🪜️terra-parity-rebaseline: STALE-BRIDGE excluded — see `isParityStaleBridge`'s doc on `writeParityReport`.
    const staleBridge = reports.filter(isParityStaleBridge);
    const failed = reports.filter((r) => !isParityStaleBridge(r) && (r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL"));
    const passed = reports.length - failed.length - staleBridge.length;
    console.log(`parity sweep complete: ${passed}/${reports.length} PASS · ${staleBridge.length}/${reports.length} STALE-BRIDGE · ${failed.length}/${reports.length} FAIL`);
    if (staleBridge.length > 0) console.log(`stale-bridge (excluded from verdict): ${staleBridge.map((r) => r.variant).join(", ")}`);
    if (failed.length > 0) throw new Error(`parity sweep: ${failed.length}/${reports.length} playground(s) failed`);
  }
}
//#endregion 🔖️Sweep
//#endregion 🔬️ParityScript

//#region 🔖️ScaleFixture
/** 🧫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME F1-scale-fixture (design-workforce.md §3): seeded,
 * deterministic 50-plugin x 50-extension synthetic registry generator — 2550 records (50 x (1 +
 * 50)), proving the whole ticket's scale claim. `💻️os/🧫️fixtures/🔌️scale/` sits outside
 * `taxonomy.pluginAreas` on purpose (verified against `🔣️taxonomy.json`'s `pluginAreas`/
 * `rootDataDirNames` before this region was written), so this generator is entirely independent of
 * `📇️registry:generate`'s production catalog — same freshness (`generate` writes, `check` byte-
 * compares and never writes) idiom, separate data, separate files.
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-workforce.md
 */
const SCALE_FIXTURE_OWNER_REL = "🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale";
const SCALE_FIXTURE_PROFILES = ["idle", "cpu", "ui", "io", "hang", "crash", "stateful"] as const;
type ScaleFixtureProfile = (typeof SCALE_FIXTURE_PROFILES)[number];

type ScaleFixtureActivationEvent = { type: "on-startup-finished" } | { type: "on-command"; id: string } | { type: "on-artifact-kind"; kind: string } | { type: "on-view-visible"; id: string };

/** 📋️ Exact shape the `semio-framework-os-scale-fixture` crate's `FixtureConfig` (`🎭️profile/
 * 🦀️component.rs`) decodes from `instance-open`'s `config` pack — field names match its `serde`
 * `camelCase` rename 1:1 so a real host can hand this object, JSON-encoded, straight through. */
type ScaleFixtureConfig = {
  profile: ScaleFixtureProfile;
  cpuBusyMs: number;
  uiPatchesPerTurn: number;
  hangOverrunMultiplier: number;
  crashAfterTurns: number;
  ioCapabilityId: string;
};

type ScaleFixtureRecord = {
  id: string;
  kind: "plugin" | "extension";
  parentId: string | null;
  activationEvents: ScaleFixtureActivationEvent[];
  quotas: { deadlineMs: number; maxEffects: number; maxPatchBytes: number; maxFrames: number };
  capabilities: string[];
  scaleFixture: ScaleFixtureConfig;
};

type ScaleFixtureRegistry = {
  seed: number;
  pluginCount: number;
  extensionsPerPlugin: number;
  recordCount: number;
  records: ScaleFixtureRecord[];
};

type ScaleFixtureCatalogEntry = {
  pluginId: string;
  profile: ScaleFixtureProfile;
  extensionIds: string[];
  extensionProfileCounts: Record<ScaleFixtureProfile, number>;
};

type ScaleFixtureCatalog = {
  seed: number;
  pluginCount: number;
  extensionsPerPlugin: number;
  totalRecordCount: number;
  profileTotals: Record<ScaleFixtureProfile, number>;
  plugins: ScaleFixtureCatalogEntry[];
};

/** 🎲️ mulberry32 — tiny seedable PRNG, deterministic across platforms/runs (no `crypto`, no
 * `Date.now()`, no engine-dependent `Math.random()`): the whole determinism proof rests on this
 * being the ONLY source of variation in `buildScaleFixtureRegistry`. */
function scaleFixtureRng(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function scaleFixturePick<T>(rng: () => number, options: readonly T[]): T {
  return options[Math.floor(rng() * options.length) % options.length]!;
}

/** 🚀️ ≈5% `on-startup-finished`, the rest split across `on-command`/`on-artifact-kind`/
 * `on-view-visible` (design-workforce.md §3's exact split). */
function scaleFixtureActivationEvent(rng: () => number, index: number): ScaleFixtureActivationEvent {
  if (rng() < 0.05) return { type: "on-startup-finished" };
  const kind = scaleFixturePick(rng, ["on-command", "on-artifact-kind", "on-view-visible"] as const);
  if (kind === "on-command") return { type: "on-command", id: `scale-fixture.command-${index}` };
  if (kind === "on-artifact-kind") return { type: "on-artifact-kind", kind: `scale-fixture.artifact-${index}` };
  return { type: "on-view-visible", id: `scale-fixture.view-${index}` };
}

function scaleFixtureRecordConfig(rng: () => number, id: string): ScaleFixtureConfig {
  return {
    profile: scaleFixturePick(rng, SCALE_FIXTURE_PROFILES),
    cpuBusyMs: 2 + Math.floor(rng() * 8),
    uiPatchesPerTurn: 1 + Math.floor(rng() * 4),
    hangOverrunMultiplier: 2 + Math.floor(rng() * 3),
    crashAfterTurns: 1 + Math.floor(rng() * 3),
    ioCapabilityId: `${id}.io`,
  };
}

function scaleFixtureRecord(rng: () => number, id: string, kind: "plugin" | "extension", parentId: string | null, index: number): ScaleFixtureRecord {
  const scaleFixture = scaleFixtureRecordConfig(rng, id);
  return {
    id,
    kind,
    parentId,
    activationEvents: [scaleFixtureActivationEvent(rng, index)],
    quotas: {
      deadlineMs: 8 + Math.floor(rng() * 24),
      maxEffects: 4 + Math.floor(rng() * 12),
      maxPatchBytes: 1024 + Math.floor(rng() * 7168),
      maxFrames: 1 + Math.floor(rng() * 4),
    },
    capabilities: scaleFixture.profile === "io" ? [scaleFixture.ioCapabilityId] : [],
    scaleFixture,
  };
}

/** 🏗️ Builds the full `pluginCount x (1 + extensionsPerPlugin)`-record registry deterministically
 * from `seed` — fixed iteration order (plugin 0..N, that plugin's own record, then its M
 * extensions 0..M) so two calls with the same `(pluginCount, extensionsPerPlugin, seed)` are
 * byte-identical once serialized (proven by `ScaleFixtureCheckScript`/this packet's acceptance
 * run, not merely asserted). */
function buildScaleFixtureRegistry(pluginCount: number, extensionsPerPlugin: number, seed: number): ScaleFixtureRegistry {
  const rng = scaleFixtureRng(seed);
  const records: ScaleFixtureRecord[] = [];
  for (let p = 0; p < pluginCount; p++) {
    const pluginId = `scale-fixture-plugin-${String(p).padStart(4, "0")}`;
    records.push(scaleFixtureRecord(rng, pluginId, "plugin", null, p));
    for (let e = 0; e < extensionsPerPlugin; e++) {
      const extensionId = `${pluginId}-ext-${String(e).padStart(4, "0")}`;
      records.push(scaleFixtureRecord(rng, extensionId, "extension", pluginId, p * extensionsPerPlugin + e));
    }
  }
  return { seed, pluginCount, extensionsPerPlugin, recordCount: records.length, records };
}

function scaleFixtureZeroProfileCounts(): Record<ScaleFixtureProfile, number> {
  return Object.fromEntries(SCALE_FIXTURE_PROFILES.map((profile) => [profile, 0])) as Record<ScaleFixtureProfile, number>;
}

/** 📚️ Per-plugin rollup (id, own profile, its extensions and THEIR profile distribution) plus a
 * fleet-wide profile total — the "playground catalog" analog `plugin-registry`'s own
 * `🤖️generated/🔣️catalog.json` plays for the production registry, derived here rather than
 * hand-duplicated. */
function buildScaleFixtureCatalog(registry: ScaleFixtureRegistry): ScaleFixtureCatalog {
  const profileTotals = scaleFixtureZeroProfileCounts();
  const plugins: ScaleFixtureCatalogEntry[] = [];
  const byId = new Map<string, ScaleFixtureCatalogEntry>();
  for (const record of registry.records) {
    profileTotals[record.scaleFixture.profile]++;
    if (record.kind === "plugin") {
      const entry: ScaleFixtureCatalogEntry = { pluginId: record.id, profile: record.scaleFixture.profile, extensionIds: [], extensionProfileCounts: scaleFixtureZeroProfileCounts() };
      plugins.push(entry);
      byId.set(record.id, entry);
    }
  }
  for (const record of registry.records) {
    if (record.kind === "extension" && record.parentId) {
      const entry = byId.get(record.parentId);
      if (!entry) continue;
      entry.extensionIds.push(record.id);
      entry.extensionProfileCounts[record.scaleFixture.profile]++;
    }
  }
  return { seed: registry.seed, pluginCount: registry.pluginCount, extensionsPerPlugin: registry.extensionsPerPlugin, totalRecordCount: registry.recordCount, profileTotals, plugins };
}

function scaleFixtureGeneratedDir(repoRoot: string): string {
  return join(repoRoot, SCALE_FIXTURE_OWNER_REL, "🤖️generated");
}

function renderScaleFixtureArtifacts(pluginCount: number, extensionsPerPlugin: number, seed: number): { registryJson: string; catalogJson: string; registry: ScaleFixtureRegistry } {
  const registry = buildScaleFixtureRegistry(pluginCount, extensionsPerPlugin, seed);
  const catalog = buildScaleFixtureCatalog(registry);
  return { registryJson: `${JSON.stringify(registry, null, 2)}\n`, catalogJson: `${JSON.stringify(catalog, null, 2)}\n`, registry };
}

function writeScaleFixtureArtifacts(repoRoot: string, pluginCount: number, extensionsPerPlugin: number, seed: number): ScaleFixtureRegistry {
  const dir = scaleFixtureGeneratedDir(repoRoot);
  mkdirSync(dir, { recursive: true });
  const { registryJson, catalogJson, registry } = renderScaleFixtureArtifacts(pluginCount, extensionsPerPlugin, seed);
  const expected = new Set(["🔣️registry.json", "🔣️catalog.json"]);
  for (const name of readdirSync(dir)) if (!expected.has(name)) rmSync(join(dir, name), { recursive: true, force: true });
  writeFileSync(join(dir, "🔣️registry.json"), registryJson);
  writeFileSync(join(dir, "🔣️catalog.json"), catalogJson);
  console.log(`scale-fixture generate: ${registry.recordCount} records (plugins=${pluginCount} extensions=${extensionsPerPlugin} seed=${seed}) -> ${dir}`);
  return registry;
}

/** ✅️ `plugin-registry:check`'s exact idiom: re-derive from the on-disk registry's own
 * `(pluginCount, extensionsPerPlugin, seed)` and byte-compare — never writes. */
function checkScaleFixtureArtifacts(repoRoot: string): boolean {
  const dir = scaleFixtureGeneratedDir(repoRoot);
  const registryPath = join(dir, "🔣️registry.json");
  const catalogPath = join(dir, "🔣️catalog.json");
  if (!existsSync(registryPath) || !existsSync(catalogPath)) return false;
  const onDiskRegistryJson = readFileSync(registryPath, "utf8");
  const onDiskCatalogJson = readFileSync(catalogPath, "utf8");
  let parsed: ScaleFixtureRegistry;
  try {
    parsed = JSON.parse(onDiskRegistryJson) as ScaleFixtureRegistry;
  } catch {
    return false;
  }
  const { registryJson, catalogJson } = renderScaleFixtureArtifacts(parsed.pluginCount, parsed.extensionsPerPlugin, parsed.seed);
  return registryJson === onDiskRegistryJson && catalogJson === onDiskCatalogJson;
}

function scaleFixtureFlag(segments: readonly string[], name: string, fallback: number): number {
  for (let i = 0; i < segments.length; i++) {
    const seg = segments[i]!;
    if (seg === `--${name}` && segments[i + 1] !== undefined) return Number(segments[i + 1]);
    if (seg.startsWith(`--${name}=`)) return Number(seg.slice(name.length + 3));
  }
  return fallback;
}

class ScaleFixtureGenerateScript extends BundleScript {
  run(segments: string[]): void {
    const pluginCount = scaleFixtureFlag(segments, "plugins", 50);
    const extensionsPerPlugin = scaleFixtureFlag(segments, "extensions", 50);
    const seed = scaleFixtureFlag(segments, "seed", 1);
    writeScaleFixtureArtifacts(this.repoRoot, pluginCount, extensionsPerPlugin, seed);
  }
}

/** 🧾️ Emits the canonical scale-fixture output protocol from the seeded in-memory renderer. */
class ScaleFixturePreviewGeneratedScript extends BundleScript {
  run(segments: string[]): void {
    const pluginCount = scaleFixtureFlag(segments, "plugins", 50);
    const extensionsPerPlugin = scaleFixtureFlag(segments, "extensions", 50);
    const seed = scaleFixtureFlag(segments, "seed", 1);
    const dir = scaleFixtureGeneratedDir(this.repoRoot);
    const rootPath = relative(this.repoRoot, dir).replaceAll("\\", "/").normalize("NFC");
    const { registryJson, catalogJson } = renderScaleFixtureArtifacts(pluginCount, extensionsPerPlugin, seed);
    const nodes = [
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
      { bytesBase64: Buffer.from(catalogJson).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/🔣️catalog.json` },
      { bytesBase64: Buffer.from(registryJson).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/🔣️registry.json` },
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const expected = new Set(["🔣️registry.json", "🔣️catalog.json"]);
    const staleRemovals = (existsSync(dir) ? readdirSync(dir) : []).filter((name) => !expected.has(name)).map((name) => `${rootPath}/${name.normalize("NFC")}`).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
    process.stdout.write(`${JSON.stringify({ contractId: "scale-fixture", nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}

class ScaleFixtureCheckScript extends BundleScript {
  run(): void {
    if (!checkScaleFixtureArtifacts(this.repoRoot)) {
      throw new Error("scale-fixture check: 🤖️generated/{🔣️registry.json,🔣️catalog.json} are stale — run `bun ./📜️script.ts generate scale-fixture`");
    }
    console.log("scale-fixture check: fresh");
  }
}
//#endregion 🔖️ScaleFixture

//#region 🔖️Bench
/** ⚖️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b-bench): `bun ./📜️script.ts bench plugins
 * [--renderer native|react|wgpu] [--count 50] [--extensions 50] [--shards 8] [--out <path>]` — the
 * dev-side harness the root `bench` verb (`/📜️script.ts` `//#region 🔖️BenchScript`) routes into.
 * Budget 1 (registry parse) is measured HERE, directly, in JS — no wasm/kernel involved, so it never
 * needs the native/web split. Budgets 2-8 are measured by the renderer-specific harness: `native`
 * drives `semio-wgpu-native --scale/--scale-wasm/--shards/--report`
 * (`📺️renderer/…/🧊️wgpu/📦️glue.rs`'s `scale_bench` module — real `Kernel`/`ShardLoop`/
 * `WasmtimeRuntime`, real scale-fixture wasm component, see that module's own doc for its honest
 * single-physical-ShardLoop scope note); `react`/`wgpu` (web) drive `//#region 🧪️BenchWebRows` below —
 * NOT `🔬️ParityScript`'s `🔖️ServerPool` (that machinery boots the FULL app against one real plugin
 * variant, a different app than the scale fixture, and needs real fleet wasm this session doesn't have
 * either) — instead the real `ShardClient` runs inside a real headless-Chromium page
 * (`🧪️bench-web-harness.ts`, bundled with `Bun.build`) against real browser `Worker`s running a protocol
 * STUB in place of the not-yet-compiled guest SDK's real worker. Budgets 3/4/6/7/8 are genuine passes of
 * `ShardClient`'s own sharding/heartbeat-trap/checkpoint logic at 100-actor scale; budgets 2/5 are
 * stub-worker timings reported as `pass-stub-worker`/`fail-stub-worker`, never plain `pass`/`fail`,
 * because they exclude real wasm instantiation and guest compute. Any harness failure (no Chromium, bundle
 * error, page timeout) falls every row back to `"skipped"` with the real error — never a fabricated pass.
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-workforce.md §4
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-bench-web-rows-report.md
 */
const BENCH_TICKET_DIR_DEFAULT = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME";

function benchOutDir(): string {
  const configured = process.env.BENCH_OUT_DIR ?? BENCH_TICKET_DIR_DEFAULT;
  const dir = resolve(repoRoot, configured);
  mkdirSync(dir, { recursive: true });
  return dir;
}

function benchTargetDir(): string {
  const dir = join(benchOutDir(), "🎯️target-v1b");
  mkdirSync(dir, { recursive: true });
  return dir;
}

type BenchBudgetDefinition = Readonly<{ id: number; description: string; nativeThreshold?: string; webThreshold?: string }>;

/** 📓️ `design-workforce.md` §4, verbatim, ONE const — descriptions + threshold numbers as data, never
 * scattered literals. The pass/fail MATH for budgets 2-8 lives on whichever side measures them (this
 * table is not re-evaluated here); budget 1's math lives in `benchRegistryRow` right below it. */
const BENCH_BUDGETS: readonly BenchBudgetDefinition[] = [
  { id: 1, description: "Registry: 2550 records parsed, instantiations == 0, < 150ms", nativeThreshold: "150ms", webThreshold: "150ms" },
  { id: 2, description: "Cold boot to first interactive frame, only on-startup-finished actors live", nativeThreshold: "1500ms", webThreshold: "2500ms" },
  { id: 3, description: "Activate 50 plugins + 50 extensions of one plugin: active_actors==100, shards==K, no shard > ceil(100/K)+1" },
  { id: 4, description: "Memory <= K x 512MiB + 256MiB headroom (web Worker count==K); native RSS <= 1.5GiB", nativeThreshold: "1.5GiB RSS" },
  { id: 5, description: "Interactive p95 command->patch, 40 cpu actors saturating background", nativeThreshold: "8ms", webThreshold: "16ms" },
  { id: 6, description: "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", nativeThreshold: "pause <= 250ms" },
  { id: 7, description: "stateful actor LRU-suspended and resumed -> identical state hash" },
  { id: 8, description: "Capability revoked at runtime -> denied completion, actor stays alive, quota counters zero" },
] as const;

function benchFlag(segments: readonly string[], name: string, fallback: string): string {
  for (let i = 0; i < segments.length; i++) {
    const segment = segments[i]!;
    if (segment === `--${name}` && segments[i + 1] !== undefined) return segments[i + 1]!;
    if (segment.startsWith(`--${name}=`)) return segment.slice(name.length + 3);
  }
  return fallback;
}

/** ⏱️ Budget 1 — measured directly: reads+parses the on-disk registry.json this run generated,
 * timing ONLY that (never touches wasm/kernel, so `instantiations == 0` is true by construction, not
 * merely asserted). */
function benchRegistryRow(registryPath: string, expectedRecordCount: number): Record<string, unknown> {
  const t0 = performance.now();
  const raw = readFileSync(registryPath, "utf8");
  const parsed = JSON.parse(raw) as { recordCount: number };
  const elapsedMs = performance.now() - t0;
  const pass = parsed.recordCount === expectedRecordCount && elapsedMs < 150;
  return {
    id: 1,
    description: BENCH_BUDGETS[0]!.description,
    status: pass ? "pass" : "fail",
    measured: { elapsedMs, recordCount: parsed.recordCount, instantiations: 0 },
    threshold: { maxMs: 150, instantiations: 0 },
    note: "measured by this dev script directly (bun readFileSync + JSON.parse) — no wasm/kernel touched",
  };
}

function benchWebSkippedRow(budget: BenchBudgetDefinition, renderer: string, reason: string): Record<string, unknown> {
  return {
    id: budget.id,
    description: budget.description,
    status: "skipped",
    measured: null,
    threshold: budget.webThreshold ?? null,
    note: `${renderer} web-renderer bench row ${budget.id} could not be measured this run: ${reason}`,
  };
}

//#region 🧪️BenchWebRows
/** 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (bench-web-rows): bundles `🧪️bench-web-harness.ts` for
 * the BROWSER with Bun's own bundler (no external bundler dependency), runs it inside a real headless
 * Chromium page via `playwright` (already a repo dependency — the SAME dynamic-import pattern this file's
 * own collab/studio-e2e scripts already use), and merges the raw per-budget measurements back onto
 * `BENCH_BUDGETS`'s id/description/threshold. See that file's own header doc for exactly what is REAL
 * (driven through the genuine `ShardClient`, real browser `Worker`s, real postMessage round trips) versus
 * STUB (no real fleet wasm exists yet — `semio-framework-plugin` does not compile this session — so each
 * worker runs a tiny protocol stub instead of the real generated `shardWorkerSource()`). `renderer` is
 * accepted for parity with the native row's `--renderer` flag and threaded into the report's metadata,
 * but the harness itself is renderer-agnostic: it measures the `ShardClient` transport layer, which react
 * and wgpu(web) share — it does NOT exercise either renderer's own paint/patch path. That gap is stated
 * here rather than silently implied by a `react`/`wgpu`-labelled row. */
async function buildBenchWebHarnessBundle(): Promise<string> {
  const entry = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/🧪️bench-web-harness.ts");
  const result = await Bun.build({ entrypoints: [entry], target: "browser", format: "esm" });
  if (!result.success) throw new Error(`bench-web harness bundle failed: ${result.logs.map((log) => log.message).join("; ")}`);
  const output = result.outputs[0];
  if (!output) throw new Error("bench-web harness bundle produced no output file");
  return await output.text();
}

async function runWebBenchViaHeadlessChromium(pluginIds: readonly string[], firstPluginExtensionIds: readonly string[], shardCount: number): Promise<Record<string, unknown>[]> {
  const bundleJs = await buildBenchWebHarnessBundle();
  const html = `<!doctype html><html><head><meta charset="utf-8"><title>bench-web</title></head><body><script type="module">
${bundleJs}
window.__BENCH_WEB__ = { done: false, rows: null, error: null };
runBenchWebBudgets(${JSON.stringify({ pluginIds, firstPluginExtensionIds, shardCount })})
  .then((rows) => { window.__BENCH_WEB__.rows = rows; window.__BENCH_WEB__.done = true; })
  .catch((error) => { window.__BENCH_WEB__.error = String((error && error.stack) || error); window.__BENCH_WEB__.done = true; });
</script></body></html>`;
  // 🎭️ Matches `StudioE2eScript`'s own install location note above — same repo-scoped Playwright cache.
  process.env.PLAYWRIGHT_BROWSERS_PATH = process.env.PLAYWRIGHT_BROWSERS_PATH ?? join(repoRoot, "node_modules", ".cache", "ms-playwright");
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    await page.setContent(html, { waitUntil: "load" });
    await page.waitForFunction(() => (window as unknown as { __BENCH_WEB__: { done: boolean } }).__BENCH_WEB__.done === true, { timeout: 60_000 });
    const state = await page.evaluate(() => (window as unknown as { __BENCH_WEB__: { rows: Record<string, unknown>[] | null; error: string | null } }).__BENCH_WEB__);
    if (state.error) throw new Error(`bench-web harness page error: ${state.error}`);
    if (!state.rows) throw new Error("bench-web harness produced no rows");
    return state.rows;
  } finally {
    await browser.close();
  }
}

const BENCH_WEB_STUB_STATUS: Readonly<Record<number, { readonly passLabel: string; readonly failLabel: string }>> = {
  2: { passLabel: "pass-stub-worker", failLabel: "fail-stub-worker" },
  5: { passLabel: "pass-stub-worker", failLabel: "fail-stub-worker" },
};

function benchWebMeasuredRow(budget: BenchBudgetDefinition, renderer: string, raw: { readonly id: number; readonly ok: boolean; readonly measured: unknown; readonly note: string }): Record<string, unknown> {
  const stubLabels = BENCH_WEB_STUB_STATUS[budget.id];
  const status = stubLabels ? (raw.ok ? stubLabels.passLabel : stubLabels.failLabel) : raw.ok ? "pass" : "fail";
  return { id: budget.id, description: budget.description, status, measured: raw.measured, threshold: budget.webThreshold ?? null, note: `[${renderer}, harness-driven, see 🧪️bench-web-harness.ts header] ${raw.note}` };
}

/** ▶️ Runs budgets 2-8 for `react`/`wgpu` through the real `ShardClient` + headless-Chromium harness.
 * `registry` supplies the actor id vocabulary (`buildScaleFixtureRegistry`'s own deterministic ids —
 * never invented ones) budget 3 needs: 50 plugin ids + the 50 extension ids belonging to plugin[0]. On
 * ANY harness failure (no Chromium installed, bundle error, page timeout, …) every row falls back to
 * `benchWebSkippedRow` with the real error message — never a silently fabricated pass. */
async function benchWebRows(budgets: readonly BenchBudgetDefinition[], renderer: string, registry: ScaleFixtureRegistry, shardCount: number): Promise<Record<string, unknown>[]> {
  const plugins = registry.records.filter((record) => record.kind === "plugin").map((record) => record.id);
  const firstPluginId = plugins[0];
  if (!firstPluginId) return budgets.map((budget) => benchWebSkippedRow(budget, renderer, "scale-fixture registry has no plugin records"));
  const firstPluginExtensions = registry.records.filter((record) => record.kind === "extension" && record.parentId === firstPluginId).map((record) => record.id);
  try {
    const raw = await runWebBenchViaHeadlessChromium(plugins, firstPluginExtensions, shardCount);
    const byId = new Map(raw.map((row) => [row.id, row]));
    return budgets.map((budget) => {
      const row = byId.get(budget.id);
      return row ? benchWebMeasuredRow(budget, renderer, row) : benchWebSkippedRow(budget, renderer, "harness returned no row for this budget id");
    });
  } catch (error) {
    const reason = error instanceof Error ? (error.stack ?? error.message) : String(error);
    return budgets.map((budget) => benchWebSkippedRow(budget, renderer, reason));
  }
}
//#endregion 🧪️BenchWebRows

class BenchPluginsScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const renderer = benchFlag(segments, "renderer", "native");
    const pluginCount = Number(benchFlag(segments, "count", "50"));
    const extensionsPerPlugin = Number(benchFlag(segments, "extensions", "50"));
    const shardCount = Number(benchFlag(segments, "shards", "8"));
    const outDir = benchOutDir();
    const outPath = benchFlag(segments, "out", join(outDir, `terra-v1b-bench-${renderer}.json`));

    const registryPath = join(outDir, "🔣️bench-registry.json");
    // 🔁️ Reuses `renderScaleFixtureArtifacts` verbatim (`//#region 🔖️ScaleFixture` above) — same
    // deterministic generator the committed `🤖️generated/🔣️registry.json` comes from, just scoped to
    // THIS run's `--count`/`--extensions` rather than whatever happens to be checked in.
    const { registryJson, registry } = renderScaleFixtureArtifacts(pluginCount, extensionsPerPlugin, 1);
    writeFileSync(registryPath, registryJson);

    const rows: Record<string, unknown>[] = [benchRegistryRow(registryPath, registry.recordCount)];

    if (renderer === "native") {
      const targetDir = benchTargetDir();
      const cargoEnv = { ...process.env, CARGO_TARGET_DIR: targetDir };
      console.log(`bench: building scale-fixture wasm (CARGO_TARGET_DIR=${targetDir})`);
      if (runCmdStatus("cargo", ["build", "-p", "semio-framework-os-scale-fixture", "--target", "wasm32-wasip2", "--features", "component-guest"], { cwd: repoRoot, env: cargoEnv, budgetMs: buildBudgetMs() }) !== 0) {
        throw new Error("bench: scale-fixture wasm build failed");
      }
      const wasmPath = join(targetDir, "wasm32-wasip2", "debug", "semio_framework_os_scale_fixture.wasm");
      if (!existsSync(wasmPath)) throw new Error(`bench: expected wasm artifact missing: ${wasmPath}`);
      const nativeReportPath = join(outDir, "🔣️bench-native-raw.json");
      const wgpuScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts");
      console.log(`bench: running native scale-bench (shards=${shardCount})`);
      if (runCmdStatus("bun", [wgpuScript, "native", "--scale", registryPath, "--scale-wasm", wasmPath, "--shards", String(shardCount), "--report", nativeReportPath], { cwd: repoRoot, env: cargoEnv, budgetMs: buildBudgetMs() }) !== 0) {
        throw new Error("bench: native scale-bench run failed");
      }
      const nativeReport = JSON.parse(readFileSync(nativeReportPath, "utf8")) as { budgets: Record<string, unknown>[] };
      rows.push(...nativeReport.budgets);
    } else if (renderer === "react" || renderer === "wgpu") {
      console.log(`bench: running web scale-bench (renderer=${renderer}, shards=${shardCount}) via headless Chromium — see 🧪️bench-web-harness.ts for real-vs-stub scope`);
      rows.push(...(await benchWebRows(BENCH_BUDGETS.slice(1), renderer, registry, shardCount)));
    } else {
      throw new Error(`bench plugins: unknown --renderer ${renderer} (expected native|react|wgpu)`);
    }

    const report = { renderer, pluginCount, extensionsPerPlugin, shardCount, seed: 1, generatedAt: new Date().toISOString(), budgets: rows };
    mkdirSync(dirname(outPath), { recursive: true });
    writeFileSync(outPath, `${JSON.stringify(report, null, 2)}\n`);
    console.log(`bench: wrote report -> ${outPath}`);
    console.log(`bench summary: ${rows.map((r) => `${(r as { id: number }).id}:${(r as { status: string }).status}`).join(" ")}`);
  }
}
//#endregion 🔖️Bench

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("verify", VerifyScript)
  .register(
    "bench",
    class extends BundleScript {
      async run(segments: string[]): Promise<void> {
        const sub = segments[0];
        if (sub === "plugins") return new BenchPluginsScript(this.root).run(segments.slice(1));
        throw new Error(`unknown bench subcommand: ${sub ?? "<none>"} (expected plugins)`);
      }
    },
  )
  .register(
    "generate",
    class extends BundleScript {
      run(segments: string[]): void {
        const sub = segments[0];
        if (sub === "scale-fixture") return new ScaleFixtureGenerateScript(this.root, this.repoRoot).run(segments.slice(1));
        throw new Error(`unknown generate subcommand: ${sub} (expected scale-fixture)`);
      }
    },
  )
  .register(
    "scale-fixture",
    class extends BundleScript {
      run(segments: string[]): void {
        const sub = segments[0];
        if (sub === "check") return new ScaleFixtureCheckScript(this.root, this.repoRoot).run(segments.slice(1));
        throw new Error(`unknown scale-fixture subcommand: ${sub} (expected check)`);
      }
    },
  )
  .register("preview-generated", ScaleFixturePreviewGeneratedScript)
  // 🧱️ Also runs as part of `plugin lint` below (the `"plugin"`/`"lint"` router entry) now that its one
  // finding is triaged — see `CapabilityLayeringLintScript`'s own docstring. Kept independently runnable
  // here too: `bun ./📜️script.ts layer-lint`.
  .register("layer-lint", CapabilityLayeringLintScript)
  // 🕳️ Deliberately NOT folded into `plugin lint`/`verify` — see `PluginIndexExportPathLintScript`'s own
  // docstring for why 517 dead barrel-export paths can't be gated the way `layer-lint` was. Standalone
  // only: `bun ./📜️script.ts index-lint` / `bun nx run @semio-tech/framework-os-dev:index-lint`.
  .register("index-lint", PluginIndexExportPathLintScript)
  // 🕳️ Deliberately NOT folded into `plugin lint`/`verify` either — see `HostHandleReachLintScript`'s own
  // docstring: several sessions are actively running against the gate and this rule will fire on plugins
  // they own. Standalone only: `bun ./📜️script.ts host-handle-lint` /
  // `bun nx run @semio-tech/framework-os-dev:host-handle-lint`.
  .register("host-handle-lint", HostHandleReachLintScript)
  .register(
    "parity",
    class extends BundleScript {
      async run(segments: string[]): Promise<void> {
        const sub = segments[0];
        if (sub === "smoke") return new ParitySmokeScript(this.root).run(segments.slice(1));
        if (sub === "triage") return new ParityTriageScript(this.root).run(segments.slice(1));
        if (sub === "probe") return new ParityProbeScript(this.root).run(segments.slice(1));
        if (sub === "verify") return new ParityVerifyScript(this.root).run(segments.slice(1));
        if (sub === "sweep") return new ParitySweepScript(this.root).run(segments.slice(1));
        throw new Error(`unknown parity subcommand: ${sub} (expected smoke|triage|probe|verify|sweep)`);
      }
    },
  )
  .register(
    "plugin",
    class extends BundleScript {
      async run(segments: string[]): Promise<void> {
        const sub = segments[0];
        if (sub === "watch") return new PluginWatchScript(this.root).run(segments.slice(1));
        if (sub === "lint") {
          // 🚪️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`: folds the Cargo-layering lint into the same
          // `plugin lint` invocation the repo-root `verify gate` already calls unconditionally
          // (`nx run @semio-tech/framework-os-dev:plugin lint`) — this wires layering into the gate without
          // touching the repo-root `📜️script.ts` (owned by a concurrent session this wave). Run the
          // capability lint first (its own error carries a richer per-package count).
          await new PluginCapabilityLintScript(this.root).run(segments.slice(1));
          return new CapabilityLayeringLintScript(this.root).run();
        }
        if (sub === "registry") {
          await ensurePluginRegistry(segments[1] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND);
          return;
        }
        if (sub === "size") return new PluginSizeScript(this.root).run(segments.slice(1));
        // 🐛️`sub` here is the variant filter itself (e.g. `plugin cad`), not a subcommand to strip —
        // slicing it off silently dropped the filter and fell back to building the entire 33-crate
        // catalog for every `bun ./📜️script.ts program <variant>` invocation.
        return new PluginBuildScript(this.root).run(segments);
      }
    },
  );

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
}

if (import.meta.vitest) {
  const { describe, expect, it, beforeEach, afterEach } = import.meta.vitest;

  //#region 🔖️PixelCompare-tests
  describe("compareOwnedParityPixels", () => {
    const options = { ignoreAntialiasing: true, threshold: 0.1 } as const;
    const edge = (middle: number): Uint8Array => {
      const pixels = new Uint8Array(3 * 3 * 4);
      for (let y = 0; y < 3; y++) {
        for (let x = 0; x < 3; x++) {
          const value = x === 0 ? 0 : x === 1 ? middle : 255;
          const offset = (y * 3 + x) * 4;
          pixels[offset] = value;
          pixels[offset + 1] = value;
          pixels[offset + 2] = value;
          pixels[offset + 3] = 255;
        }
      }
      return pixels;
    };

    it("writes a muted row-major identity diff", () => {
      const reference = Uint8Array.from([0, 0, 0, 255, 64, 64, 64, 255, 255, 255, 255, 255]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, reference.slice(), diff, 3, 1, options)).toBe(0);
      expect([...diff]).toEqual([191, 191, 191, 255, 207, 207, 207, 255, 255, 255, 255, 255]);
    });

    it("keeps a grayscale delta of 25 below threshold and counts 26 above it", () => {
      const reference = Uint8Array.from([0, 0, 0, 255, 0, 0, 0, 255]);
      const candidate = Uint8Array.from([25, 25, 25, 255, 26, 26, 26, 255]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 2, 1, options)).toBe(1);
      expect([...diff]).toEqual([191, 191, 191, 255, 255, 32, 64, 255]);
    });

    it("composites alpha over white and ignores invisible RGB", () => {
      const reference = Uint8Array.from([255, 0, 0, 0, 0, 0, 0, 128]);
      const candidate = Uint8Array.from([0, 255, 0, 0, 0, 0, 0, 0]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 2, 1, options)).toBe(1);
      expect([...diff]).toEqual([255, 255, 255, 255, 255, 32, 64, 255]);
    });

    it("marks shared-edge coverage differences as antialiasing", () => {
      const reference = edge(96);
      const candidate = edge(128);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 3, 3, options)).toBe(2);
      expect([...diff.slice(16, 20)]).toEqual([255, 192, 0, 255]);
      expect(compareOwnedParityPixels(reference, candidate, new Uint8Array(reference.length), 3, 3, { ...options, ignoreAntialiasing: false })).toBe(3);
    });

    it("counts a real high-contrast edge displacement", () => {
      const reference = edge(0);
      const candidate = edge(255);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 3, 3, options)).toBe(3);
      expect([...diff.slice(16, 20)]).toEqual([255, 32, 64, 255]);
    });

    it("rejects writable typed-array overlap without corrupting either read input", () => {
      const reference = edge(96);
      const candidate = edge(128);
      const byteLength = reference.byteLength;
      const exactAliasBefore = reference.slice();
      expect(() => compareOwnedParityPixels(reference, candidate, reference, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(reference).toEqual(exactAliasBefore);

      const forwardStorage = new Uint8Array(byteLength + 4);
      const forwardReference = forwardStorage.subarray(0, byteLength);
      const forwardDiff = forwardStorage.subarray(4, byteLength + 4);
      forwardReference.set(reference);
      const forwardBefore = forwardStorage.slice();
      expect(() => compareOwnedParityPixels(forwardReference, candidate, forwardDiff, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(forwardStorage).toEqual(forwardBefore);

      const backwardStorage = new Uint8Array(byteLength + 4);
      const backwardDiff = backwardStorage.subarray(0, byteLength);
      const backwardCandidate = backwardStorage.subarray(4, byteLength + 4);
      backwardCandidate.set(candidate);
      const backwardBefore = backwardStorage.slice();
      expect(() => compareOwnedParityPixels(reference, backwardCandidate, backwardDiff, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(backwardStorage).toEqual(backwardBefore);

      const disjointStorage = new Uint8Array(byteLength * 2);
      const disjointReference = disjointStorage.subarray(0, byteLength);
      const disjointDiff = disjointStorage.subarray(byteLength);
      disjointReference.set(reference);
      expect(compareOwnedParityPixels(disjointReference, candidate, disjointDiff, 3, 3, options)).toBe(2);
      expect([...disjointDiff.slice(16, 20)]).toEqual([255, 192, 0, 255]);

      const readOnlyAliasDiff = new Uint8Array(byteLength);
      expect(compareOwnedParityPixels(reference, reference, readOnlyAliasDiff, 3, 3, options)).toBe(0);
      const empty = new Uint8Array(0);
      expect(compareOwnedParityPixels(empty, empty, empty, 0, 3, options)).toBe(0);

      const retainedDiff = new Uint8Array(byteLength);
      expect(compareOwnedParityPixels(reference, candidate, retainedDiff, 3, 3, options)).toBe(2);
      expect([...retainedDiff.slice(16, 20)]).toEqual([255, 192, 0, 255]);
    });

    it("rejects malformed lengths, dimensions, and thresholds exactly", () => {
      const pixel = new Uint8Array(4);
      expect(() => compareOwnedParityPixels(new Uint8Array(3), pixel, pixel, 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, new Uint8Array(5), pixel, 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, pixel, new Uint8Array(0), 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, -1, 1, options)).toThrow("non-negative safe integers");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, Number.MAX_SAFE_INTEGER, 2, options)).toThrow("safe byte range");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, 1, 1, { ...options, threshold: Number.NaN })).toThrow("finite and between zero and one");
    });

    it("compares a large identical crop within a bounded tooling smoke budget", () => {
      const width = 640;
      const height = 360;
      const reference = new Uint8Array(width * height * 4);
      const candidate = new Uint8Array(reference);
      const diff = new Uint8Array(reference.length);
      const started = performance.now();
      expect(compareOwnedParityPixels(reference, candidate, diff, width, height, options)).toBe(0);
      expect(performance.now() - started).toBeLessThan(2_000);
      expect(diff[0]).toBe(255);
      expect(diff[diff.length - 1]).toBe(255);
    });

    it("retains representative opaque, transparent, text-edge, and scene-gradient counts", () => {
      const opaqueReference = Uint8Array.from([0, 0, 0, 255, 40, 40, 40, 255, 120, 90, 60, 255, 255, 255, 255, 255]);
      const opaqueCandidate = Uint8Array.from([0, 0, 0, 255, 60, 60, 60, 255, 170, 90, 60, 255, 230, 230, 230, 255]);
      const transparentReference = Uint8Array.from([255, 0, 0, 0, 0, 0, 0, 128, 20, 80, 160, 255]);
      const transparentCandidate = Uint8Array.from([0, 255, 0, 0, 0, 0, 0, 0, 20, 80, 160, 192]);
      const edgeReference = edge(96);
      const edgeCandidate = edge(128);
      const gradientReference = new Uint8Array(16 * 4 * 4);
      const gradientCandidate = new Uint8Array(gradientReference.length);
      for (let index = 0; index < 64; index++) {
        const referenceValue = Math.round((index % 16) * 17);
        const candidateValue = Math.min(255, referenceValue + 12);
        const offset = index * 4;
        gradientReference.set([referenceValue, Math.max(0, referenceValue - 20), 255 - referenceValue, 255], offset);
        gradientCandidate.set([candidateValue, Math.max(0, candidateValue - 20), 255 - candidateValue, 255], offset);
      }
      const fixtures = [
        { antialiasMarkers: 0, candidate: opaqueCandidate, height: 1, mismatches: 1, name: "opaque", reference: opaqueReference, width: 4 },
        { antialiasMarkers: 0, candidate: transparentCandidate, height: 1, mismatches: 1, name: "transparent", reference: transparentReference, width: 3 },
        { antialiasMarkers: 1, candidate: edgeCandidate, height: 3, mismatches: 2, name: "text-edge", reference: edgeReference, width: 3 },
        { antialiasMarkers: 0, candidate: gradientCandidate, height: 4, mismatches: 0, name: "scene-gradient", reference: gradientReference, width: 16 },
      ];
      for (const fixture of fixtures) {
        const ownedDiff = new Uint8Array(fixture.reference.length);
        const ownedCount = compareOwnedParityPixels(fixture.reference, fixture.candidate, ownedDiff, fixture.width, fixture.height, options);
        let ownedAntialiasMarkers = 0;
        let ownedMismatchMarkers = 0;
        for (let offset = 0; offset < ownedDiff.length; offset += 4) {
          if (ownedDiff[offset] === 255 && ownedDiff[offset + 1] === 192 && ownedDiff[offset + 2] === 0) ownedAntialiasMarkers += 1;
          if (ownedDiff[offset] === 255 && ownedDiff[offset + 1] === 32 && ownedDiff[offset + 2] === 64) ownedMismatchMarkers += 1;
        }
        expect({ fixture: fixture.name, ownedAntialiasMarkers, ownedCount, ownedMismatchMarkers }).toEqual({
          fixture: fixture.name,
          ownedAntialiasMarkers: fixture.antialiasMarkers,
          ownedCount: fixture.mismatches,
          ownedMismatchMarkers: fixture.mismatches,
        });
      }
    });

    it("crops complete RGBA rows with strict bounds and independent storage", () => {
      const image: OwnedParityImage = { width: 4, height: 3, data: Uint8Array.from({ length: 48 }, (_, index) => index) };
      const before = image.data.slice();
      const crop = cropOwnedParityRgba(image, 1, 1, 2, 2);
      expect([...crop]).toEqual([...image.data.slice(20, 28), ...image.data.slice(36, 44)]);
      crop.fill(255);
      expect(image.data).toEqual(before);
      expect([...cropOwnedParityRgba(image, 4, 3, 0, 0)]).toEqual([]);
      expect(() => cropOwnedParityRgba(image, -1, 0, 1, 1)).toThrow("non-negative safe integers");
      expect(() => cropOwnedParityRgba(image, 3, 2, 2, 1)).toThrow("within image bounds");
      expect(() => cropOwnedParityRgba({ ...image, data: image.data.subarray(0, 47) }, 0, 0, 1, 1)).toThrow("exactly 48 RGBA bytes");
    });

    it("preserves fixed CSS color, alpha, and diagnostic marker pixels through Canvas PNGs", async () => {
      ensureParityPlaywrightBrowsersPath();
      const { chromium } = await import("playwright");
      const browser = await chromium.launch({ headless: true });
      try {
        const page = await browser.newPage({ viewport: { width: 4, height: 3 } });
        await page.setContent(
          '<style>*{margin:0}body{background:transparent}#opaque{position:absolute;width:1px;height:1px;background:rgb(12 34 56)}#alpha{position:absolute;left:1px;width:1px;height:1px;background:rgb(20 40 60 / 50%)}</style><div id="opaque"></div><div id="alpha"></div>',
        );
        const screenshot = await page.screenshot({ omitBackground: true });
        const decoded = await decodeParityScreenshot(page, screenshot);
        expect({ width: decoded.width, height: decoded.height, firstRow: [...decoded.data.slice(0, 16)] }).toEqual({
          width: 4,
          height: 3,
          firstRow: [12, 34, 56, 255, 20, 40, 60, 128, 0, 0, 0, 0, 0, 0, 0, 0],
        });
        expect([...cropOwnedParityRgba(decoded, 0, 0, 2, 1)]).toEqual([12, 34, 56, 255, 20, 40, 60, 128]);
        const diagnostic = Uint8Array.from([12, 34, 56, 255, 20, 40, 60, 128, 255, 32, 64, 255, 255, 192, 0, 255]);
        const roundTrip = await decodeParityScreenshot(page, await encodeParityDiff(page, diagnostic, 2, 2));
        expect({ width: roundTrip.width, height: roundTrip.height, data: [...roundTrip.data] }).toEqual({ width: 2, height: 2, data: [...diagnostic] });
      } finally {
        await browser.close();
      }
    }, 30_000);
  });
  //#endregion 🔖️PixelCompare-tests

  describe("scanBuiltPluginModules (plugin hot-swap SSE snapshot)", () => {
    let root: string;

    beforeEach(() => {
      root = mkdtempSync(join(tmpdir(), "semio-plugin-hot-swap-"));
    });

    afterEach(() => {
      rmSync(root, { recursive: true, force: true });
    });

    it("returns nothing for a missing root", () => {
      expect(scanBuiltPluginModules(join(root, "does-not-exist"))).toEqual([]);
    });

    it("skips a plugin dir with no core wasm output yet", () => {
      mkdirSync(join(root, "note"), { recursive: true });
      writeFileSync(join(root, "note", "🟨️host-shim.js"), "");
      expect(scanBuiltPluginModules(root)).toEqual([]);
    });

    it("skips the shared _vendor dir", () => {
      mkdirSync(join(root, "_vendor"), { recursive: true });
      writeFileSync(join(root, "_vendor", "shim.core.wasm"), "");
      expect(scanBuiltPluginModules(root)).toEqual([]);
    });

    it("reports a built plugin's newest core wasm mtime", () => {
      mkdirSync(join(root, "note"), { recursive: true });
      writeFileSync(join(root, "note", "note_plugin_component.core.wasm"), "");
      const rows = scanBuiltPluginModules(root);
      expect(rows).toHaveLength(1);
      expect(rows[0]!.pluginId).toBe("note");
      expect(rows[0]!.rebuiltAt).toBeGreaterThan(0);
    });

    it("reports one row per plugin dir, largest mtime among multiple core wasm chunks", () => {
      mkdirSync(join(root, "note"), { recursive: true });
      writeFileSync(join(root, "note", "note_plugin_component.core.wasm"), "");
      writeFileSync(join(root, "note", "note_plugin_component.core2.wasm"), "");
      mkdirSync(join(root, "s"), { recursive: true });
      writeFileSync(join(root, "s", "s_plugin_component.core.wasm"), "");
      const rows = scanBuiltPluginModules(root);
      expect(rows.map((row) => row.pluginId).sort()).toEqual(["note", "s"]);
    });
  });

  describe("PluginHotSwapMarker JSON round-trip (SSE `built` event payload)", () => {
    it("parses the exact shape buildPlugin writes to .hot-swap", () => {
      const marker = JSON.parse(`${JSON.stringify({ pluginId: "note", rebuiltAt: 1785789943669 })}\n`) as PluginHotSwapMarker;
      expect(marker).toEqual({ pluginId: "note", rebuiltAt: 1785789943669 });
      const event: PluginSourceEvent = { kind: "built", pluginId: marker.pluginId, rebuiltAt: marker.rebuiltAt };
      expect(event).toEqual({ kind: "built", pluginId: "note", rebuiltAt: 1785789943669 });
    });
  });

  describe("catalog state transition probe", () => {
    const node = (path: string, kind: string, selected = false): ParityNode => ({
      path,
      kind,
      rect: [0, 0, 20, 20],
      text: null,
      color: null,
      bg: null,
      fontSize: null,
      fontWeight: null,
      visible: true,
      state: { hovered: false, disabled: false, selected },
    });
    const dump = (nodes: readonly ParityNode[]): ParityDump => ({ viewport: { w: 100, h: 100, dpr: 1 }, focusPath: null, nodes });

    it("selects only common explicitly-id'd app controls in semantic priority order", () => {
      const react = dump([node("button[0]#run", "button"), node("toggle[1]#enabled", "toggle"), node("stack[2]", "stack")]);
      const wgpu = dump([node("button[0]#run", "button"), node("toggle[1]#enabled", "toggle"), node("select[3]#mode", "select")]);
      expect(stateProbeCandidates(react, wgpu)).toEqual([
        { path: "toggle[1]#enabled", kind: "toggle" },
        { path: "button[0]#run", kind: "button" },
      ]);
    });

    it("records selected-state and topology changes but ignores focus-only movement", () => {
      const before = dump([node("toggle[0]#enabled", "toggle")]);
      const after = { ...dump([node("toggle[0]#enabled", "toggle", true), node("text[1]#status", "text")]), focusPath: "toggle[0]#enabled" };
      expect(stateProbeChangedPaths(before, after)).toEqual(["text[1]#status", "toggle[0]#enabled"]);
      expect(stateProbeSnapshot(before).digest).not.toBe(stateProbeSnapshot(after).digest);
    });
  });

  //#region 🔖️T-P8-tests
  describe("stagePluginDescriptor", () => {
    it("stages descriptor siblings for migrated plugins and leaves unmigrated plugins absent", () => {
      const root = mkdtempSync(join(tmpdir(), "semio-plugin-descriptor-stage-"));
      try {
        const target = {
          pluginId: "demo",
          cratePath: "owner/demo/📦️packages/🦀️rust",
          packageName: "demo",
          wasmOut: "demo.wasm",
          role: "plugin",
          capabilities: [],
          contributes: [],
          consumes: [],
          dependsOn: [],
          activationEvents: [],
          extensionPoints: [],
        } satisfies PluginRegistryEntry;
        const ownerRoot = join(root, "owner/demo");
        const outDir = join(root, "out");
        mkdirSync(ownerRoot, { recursive: true });
        mkdirSync(outDir, { recursive: true });
        writeFileSync(join(ownerRoot, "🔣️descriptor.json"), '{"manifest":{"pluginId":"demo"}}\n');
        writeFileSync(join(ownerRoot, "🛂️descriptor.semio"), "descriptor-pack");
        expect(stagePluginDescriptor(target, outDir, root)).toBe(true);
        expect(readFileSync(join(outDir, "🔣️descriptor.json"), "utf8")).toContain('"pluginId":"demo"');
        expect(readFileSync(join(outDir, "🛂️descriptor.semio"), "utf8")).toBe("descriptor-pack");
        rmSync(join(ownerRoot, "🔣️descriptor.json"));
        expect(stagePluginDescriptor(target, outDir, root)).toBe(false);
        expect(existsSync(join(outDir, "🔣️descriptor.json"))).toBe(false);
        expect(existsSync(join(outDir, "🛂️descriptor.semio"))).toBe(false);
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });
  });

  describe("pluginComponentBridgeSource", () => {
    it("adapts the shard envelope into the canonical jco variant representation", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain('kind === "wake" ? ({ tag: kind }) : ({ tag: kind, val: payload })');
    });

    it("adapts the actor grant into the reactor budget vocabulary", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain("fuel: BigInt(budget.fuel), deadlineMs: budget.wallMs");
      expect(source).toContain("maxFrames: 8");
    });

    it("normalizes the scalar command-ingress wire record without relying on async variant discriminants", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain("function normalizeCommandIngress(status)");
      expect(source).toContain('[0, "idle"], [1, "page-accepted"], [2, "backpressure"], [3, "command-pending"], [4, "command-complete"], [5, "fault"]');
      expect(source).toContain("commandIngress: normalizeCommandIngress(result.commandIngress)");
    });

    it("gives every actor a distinct component module while carrying the rebuild version", () => {
      const source = pluginComponentBridgeSource("plugin_component", "plugin_component.core.wasm");
      expect(source).toContain('componentUrl.searchParams.set("actor", actorId)');
      expect(source).toContain('componentUrl.searchParams.set("v", rebuildVersion)');
      expect(source).toContain("await import(componentUrl.href)");
      expect(source).not.toContain('await import("./plugin_component.js")');
    });
  });

  describe("rewriteJcoComponentAssetUrls", () => {
    it("propagates a component module rebuild version to every extracted core wasm fetch", () => {
      const generated = `const module0 = fetchCompile(new URL('./plugin_component.core.wasm', import.meta.url));
const module1 = fetchCompile(new URL('./plugin_component.core2.wasm', import.meta.url));`;
      const rewritten = rewriteJcoComponentAssetUrls(generated);
      expect(rewritten).toContain("function __semioVersionedComponentAssetUrl(path)");
      expect(rewritten).toContain("const rebuildVersion = new URL(import.meta.url).searchParams.get(\"v\")");
      expect(rewritten).toContain("__semioVersionedComponentAssetUrl('./plugin_component.core.wasm')");
      expect(rewritten).toContain("__semioVersionedComponentAssetUrl('./plugin_component.core2.wasm')");
      expect(rewriteJcoComponentAssetUrls(rewritten)).toBe(rewritten);
    });
  });

  describe("rewriteJcoAsyncResultLifting", () => {
    it("checks the resolved callback memory", () => {
      const jcoGenerated = `function taskReturn(ctx) {
  const memory = ctx.getMemoryFn();
  if (!ctx.memory) {
      _debugLog('missing memory despite indirect param usage', { ctx });
  }
}`;
      const rewritten = rewriteJcoAsyncResultLifting(jcoGenerated);
      expect(rewritten).toContain("if (!memory) {");
      expect(rewriteJcoAsyncResultLifting(rewritten)).toBe(rewritten);
    });

    it("preserves direct descriptor and job results and lifts large turn results indirectly", async () => {
      const { execFileSync } = await import("node:child_process");
      const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧪️fixtures/🔣️async-results.json");
      const generated = execFileSync("node", ["--input-type=module", "--eval", `
        import { parse, transpile } from "@bytecodealliance/jco";
        import { readFileSync } from "node:fs";
        const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
        const results = [];
        for (const test of fixture.cases) {
          const wat = fixture.componentTemplate.replace("{{type}}", test.type).replace("{{params}}", test.params.join(" "));
          const { files } = await transpile(await parse(wat), { name: test.id, noTypescript: true });
          results.push({ ...test, source: new TextDecoder().decode(files[test.id + ".js"]) });
        }
        console.log(JSON.stringify(results));
      `, fixturePath], { cwd: repoRoot, encoding: "utf8", timeout: 20_000, maxBuffer: 8 * 1024 * 1024 });
      for (const test of JSON.parse(generated)) {
        const rewritten = rewriteJcoAsyncResultLifting(test.source);
        expect(rewritten.match(/taskReturn\.bind\([\s\S]*?useDirectParams: (true|false)/)?.[1], test.id).toBe(String(test.direct));
        expect(rewriteJcoAsyncResultLifting(rewritten), test.id).toBe(rewritten);
      }
    }, 30_000);
  });

  describe("rewritePreview2ShimImportSource", () => {
    it("rebases an installed extension from the build tree onto the shared plugin vendor root", () => {
      const staged = `import { environment } from '../_vendor/@bytecodealliance/preview2-shim/cli.js';`;
      expect(rewritePreview2ShimImportSource(staged, "../../plugin-modules/_vendor/@bytecodealliance/preview2-shim/")).toBe(
        `import { environment } from '../../plugin-modules/_vendor/@bytecodealliance/preview2-shim/cli.js';`,
      );
    });
  });

  describe("pluginCargoArgs", () => {
    it("links every actor component with bounded headroom for descriptor and app assembly", () => {
      expect(pluginCargoArgs("semio-s-plugin-procedural", "wasm-release")).toEqual([
        "rustc",
        "-p",
        "semio-s-plugin-procedural",
        "--target",
        "wasm32-wasip2",
        "--profile",
        "wasm-release",
        "--",
        "-C",
        "link-arg=-zstack-size=8388608",
      ]);
    });

    it("can retain actor symbols for a reproducible browser trap diagnosis", () => {
      process.env.SEMIO_PLUGIN_SYMBOLS = "1";
      try {
        expect(pluginCargoArgs("semio-s-plugin-procedural", "wasm-release").slice(-2)).toEqual(["-C", "strip=none"]);
      } finally {
        delete process.env.SEMIO_PLUGIN_SYMBOLS;
      }
    });
  });

  describe("createConcurrencyLimiter (T-P8 bounded-parallel materialize primitive)", () => {
    it("never runs more than `limit` callbacks concurrently, and still runs every one to completion", async () => {
      const limiter = createConcurrencyLimiter(2);
      let active = 0;
      let maxActive = 0;
      const tasks = Array.from({ length: 6 }, (_, i) =>
        limiter.run(async () => {
          active++;
          maxActive = Math.max(maxActive, active);
          await new Promise((r) => setTimeout(r, 5));
          active--;
          return i;
        }),
      );
      const results = await Promise.all(tasks);
      expect(results.slice().sort((a, b) => a - b)).toEqual([0, 1, 2, 3, 4, 5]);
      expect(maxActive).toBeLessThanOrEqual(2);
      expect(maxActive).toBe(2); // proves it actually overlaps, not just an accidental serialization
    });
  });

  describe("buildPluginCatalog (T-P8: cargo stage serial, materialize stage bounded-parallel)", () => {
    it("rejects incomplete explicit builds instead of accepting stale plugin artifacts", () => {
      expect(() => assertPluginCatalogComplete([])).not.toThrow();
      expect(() => assertPluginCatalogComplete(["cargo-fails", "materialize-fails"])).toThrow("plugin catalog build failed: cargo-fails, materialize-fails");
    });

    const fakeTarget = (pluginId: string): PluginRegistryEntry => ({
      pluginId,
      cratePath: "",
      packageName: pluginId,
      wasmOut: `${pluginId}.wasm`,
      role: "plugin",
      capabilities: [],
      contributes: [],
      consumes: [],
      dependsOn: [],
      activationEvents: [],
      extensionPoints: [],
    });

    it("never overlaps two cargo calls, overlaps materialize up to the cap, and emits every plugin", async () => {
      const targets = Array.from({ length: 6 }, (_, i) => fakeTarget(`p${i}`));
      let cargoActive = 0;
      let maxCargoActive = 0;
      let materializeActive = 0;
      let maxMaterializeActive = 0;
      const materialized: string[] = [];
      const cargoFn = async (target: PluginRegistryEntry) => {
        cargoActive++;
        maxCargoActive = Math.max(maxCargoActive, cargoActive);
        await new Promise((r) => setTimeout(r, 5));
        cargoActive--;
        return { artifact: `${target.pluginId}.wasm` };
      };
      const materializeFn = async (target: PluginRegistryEntry) => {
        materializeActive++;
        maxMaterializeActive = Math.max(maxMaterializeActive, materializeActive);
        await new Promise((r) => setTimeout(r, 15));
        materializeActive--;
        materialized.push(target.pluginId);
      };
      let shardWorkerPublishCount = 0;
      const { failedPluginIds } = await buildPluginCatalog(targets, cargoFn, materializeFn, 3, () => {
        shardWorkerPublishCount++;
      });
      expect(failedPluginIds).toEqual([]);
      expect(materialized.slice().sort()).toEqual(targets.map((t) => t.pluginId).sort());
      expect(maxCargoActive).toBe(1); // cargo NEVER overlaps itself
      expect(maxMaterializeActive).toBeGreaterThan(1); // materialize DOES overlap
      expect(maxMaterializeActive).toBeLessThanOrEqual(3); // never past the cap
      expect(shardWorkerPublishCount).toBe(1); // once per catalog build, not once per plugin
    });

    it("continues past both a cargo failure and a materialize failure, reporting each pluginId exactly once", async () => {
      const targets = [fakeTarget("ok"), fakeTarget("cargo-fails"), fakeTarget("materialize-fails")];
      const cargoFn = async (target: PluginRegistryEntry) => {
        if (target.pluginId === "cargo-fails") throw new Error("boom");
        return { artifact: `${target.pluginId}.wasm` };
      };
      const materialized: string[] = [];
      const materializeFn = async (target: PluginRegistryEntry) => {
        if (target.pluginId === "materialize-fails") throw new Error("boom");
        materialized.push(target.pluginId);
      };
      const { failedPluginIds } = await buildPluginCatalog(targets, cargoFn, materializeFn, 4, () => {});
      expect(new Set(failedPluginIds)).toEqual(new Set(["cargo-fails", "materialize-fails"]));
      expect(materialized).toEqual(["ok"]);
    });
  });
  //#endregion 🔖️T-P8-tests

  //#region 🔖️T-P8-sqlite-handle-cache-tests
  describe("backboneDbHandleFor (T-P8 per-path sqlite handle cache)", () => {
    let root: string;

    beforeEach(() => {
      root = mkdtempSync(join(tmpdir(), "semio-backbone-db-cache-"));
    });

    afterEach(() => {
      rmSync(root, { recursive: true, force: true });
    });

    it("returns the SAME handle for the same path across repeated calls", async () => {
      const dbPath = join(root, "a", "documents.db");
      mkdirSync(dirname(dbPath), { recursive: true });
      const first = await backboneDbHandleFor(dbPath);
      const second = await backboneDbHandleFor(dbPath);
      expect(second).toBe(first);
    });

    it("returns DISTINCT handles for distinct paths", async () => {
      const dbPathA = join(root, "a", "documents.db");
      const dbPathB = join(root, "b", "documents.db");
      mkdirSync(dirname(dbPathA), { recursive: true });
      mkdirSync(dirname(dbPathB), { recursive: true });
      const a = await backboneDbHandleFor(dbPathA);
      const b = await backboneDbHandleFor(dbPathB);
      expect(a).not.toBe(b);
    });
  });
  //#endregion 🔖️T-P8-sqlite-handle-cache-tests

  //#region 🔖️T-P8-extension-sweep-tests
  describe("sweepStaleExtensionModuleOutputs (T-P8 pre-ABI-flip generated-output sweep)", () => {
    let root: string;

    beforeEach(() => {
      root = mkdtempSync(join(tmpdir(), "semio-extension-sweep-"));
    });

    afterEach(() => {
      rmSync(root, { recursive: true, force: true });
    });

    it("removes a planted stale 🟨️plugin-worker.js unconditionally (no code path writes that file anymore)", () => {
      const dir = join(root, "flow-extension-text");
      mkdirSync(dir, { recursive: true });
      const staleWorker = join(dir, "🟨️plugin-worker.js");
      writeFileSync(staleWorker, "/** pre-H2 leftover */");
      sweepStaleExtensionModuleOutputs(root);
      expect(existsSync(staleWorker)).toBe(false);
    });

    it("removes a planted stale 🟨️host-shim.js whose content predates the current pure-only ABI", () => {
      const dir = join(root, "flow-extension-text");
      mkdirSync(dir, { recursive: true });
      const staleShim = join(dir, PLUGIN_HOST_SHIM_FILE);
      writeFileSync(staleShim, "/** @generated semio plugin host shim */\nexport function readDocument(handle) { throw `unsupported: ${handle}`; }\n");
      sweepStaleExtensionModuleOutputs(root);
      expect(existsSync(staleShim)).toBe(false);
    });

    it("keeps a 🟨️host-shim.js whose content already matches the current hostShimSource()", () => {
      const dir = join(root, "note");
      mkdirSync(dir, { recursive: true });
      const freshShim = join(dir, PLUGIN_HOST_SHIM_FILE);
      writeFileSync(freshShim, hostShimSource());
      sweepStaleExtensionModuleOutputs(root);
      expect(existsSync(freshShim)).toBe(true);
      expect(readFileSync(freshShim, "utf8")).toBe(hostShimSource());
    });

    it("is a no-op against a missing root", () => {
      expect(() => sweepStaleExtensionModuleOutputs(join(root, "does-not-exist"))).not.toThrow();
    });
  });
  //#endregion 🔖️T-P8-extension-sweep-tests

  //#region 🔖️PollHelpers-tests
  describe("awaitTcpReady (W6 poll-census helper)", () => {
    it("honours its deadline and reports timeout — no real sleeps: fake clock + fake probe", async () => {
      let fakeNow = 0;
      const now = () => fakeNow;
      const sleep = async (ms: number) => {
        fakeNow += ms;
      };
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 1000,
        intervalMs: 250,
        probe: () => false,
        now,
        sleep,
      });
      expect(outcome).toBe("timeout");
      expect(fakeNow).toBeGreaterThanOrEqual(1000);
    });

    it("resolves ready as soon as the injected probe reports the port open", async () => {
      let calls = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        probe: () => {
          calls++;
          return calls >= 3;
        },
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(3);
    });

    it("resolves closed-mode ready once the injected probe reports the port free", async () => {
      let calls = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        mode: "closed",
        probe: () => {
          calls++;
          return calls < 2; // "in use" for the first call, "free" from the second on
        },
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(2);
    });

    it("resolves dead as soon as isDead() reports true, before the deadline", async () => {
      let fakeNow = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        probe: () => false,
        isDead: () => true,
        now: () => fakeNow,
        sleep: async (ms) => {
          fakeNow += ms;
        },
      });
      expect(outcome).toBe("dead");
      expect(fakeNow).toBe(0); // died on the very first check, before any sleep
    });
  });

  describe("awaitHttpOk (W6 poll-census helper)", () => {
    it("honours its deadline and reports timeout — no real sleeps: fake clock + always-throwing fetch", async () => {
      let fakeNow = 0;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 1000,
        intervalMs: 500,
        fetchImpl: (async () => {
          throw new Error("ECONNREFUSED");
        }) as unknown as typeof fetch,
        now: () => fakeNow,
        sleep: async (ms) => {
          fakeNow += ms;
        },
      });
      expect(outcome).toBe("timeout");
    });

    it("resolves ready once the injected fetch stops throwing", async () => {
      let calls = 0;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 10_000,
        intervalMs: 500,
        fetchImpl: (async () => {
          calls++;
          if (calls < 2) throw new Error("ECONNREFUSED");
          return {} as unknown as Response; // value is irrelevant — awaitHttpOk only cares that fetch resolved
        }) as unknown as typeof fetch,
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(2);
    });

    it("resolves dead as soon as isDead() reports true, before attempting to fetch", async () => {
      let fetchCalled = false;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 10_000,
        intervalMs: 500,
        isDead: () => true,
        fetchImpl: (async () => {
          fetchCalled = true;
          return {} as unknown as Response; // value is irrelevant — awaitHttpOk only cares that fetch resolved
        }) as unknown as typeof fetch,
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("dead");
      expect(fetchCalled).toBe(false);
    });
  });

  describe("awaitChildExit (W6 event-driven fix — replaces polling child.exitCode)", () => {
    it("resolves as soon as the child's own 'exit' event fires, without polling", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: null });
      // 🧵️ `timeoutAfter` deliberately never resolves — if `awaitChildExit` secretly depended on a
      // timer/poll to notice the exit (instead of the 'exit' event alone), this promise would never
      // settle and the `await` below would hang until vitest's own test timeout fails the case.
      const resultPromise = awaitChildExit(fakeChild, 30_000, { timeoutAfter: () => new Promise<"timeout">(() => {}) });
      // 🧵️ Simulate Node setting exitCode then emitting 'exit', exactly as a real ChildProcess does.
      (fakeChild as unknown as { exitCode: number | null }).exitCode = 0;
      (fakeChild as unknown as EventEmitter).emit("exit", 0, null);
      const result = await resultPromise;
      expect(result).toBe("exited");
    });

    it("resolves immediately for a child that had already exited before the call", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: 0 });
      // Same never-resolving `timeoutAfter` as above: only the already-set `exitCode` can win this race.
      const result = await awaitChildExit(fakeChild, 30_000, { timeoutAfter: () => new Promise<"timeout">(() => {}) });
      expect(result).toBe("exited");
    });

    it("still times out for a hung child that never emits 'exit' — fake deadline, no real sleep", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: null });
      const result = await awaitChildExit(fakeChild, 30_000, {
        timeoutAfter: async () => "timeout" as const, // resolves instantly, standing in for "deadline reached"
      });
      expect(result).toBe("timeout");
    });
  });
  //#endregion 🔖️PollHelpers-tests
}
