/** @emoji 🔌️ The dev server's own Vite plugins — backbone document IO, the content-addressed blob
 * endpoint, the plugin hot-swap SSE stream and the production test boundary — kept in a module of
 * their own so `⚙️vite.config.ts` can mount them without pulling `📜️script.ts`'s task router (and
 * through it the repository library's discovery walk) into Vite's config bundle. `bun:sqlite` stays a
 * lazy dynamic import: Vite loads this module's exports under Node before the dev server's Bun
 * runtime exists. */
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, watch, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { BACKBONE_ENDPOINT_PATH, BLOB_ENDPOINT_PATH, backboneKindFromUri, decodeDocumentPackBytes, encodeDocumentPackBytes } from "@semio-tech/framework-os";
import type { PluginSourceEvent } from "@semio-tech/framework";
import { MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, moduleIdForDirectoryName, moduleRoutePath } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { blake3Hex } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";

/** @emoji 🗂️ Repository root derived from this module's own location — the config bundler must not
 * reach `getWorkspaceRoot` (and the discovery walk behind it) just to place two dev databases. */
export const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");

/** @emoji 🔌️ Shared dev-session output root every built plugin module lands in. */
export const PLUGIN_MODULES_ROOT = join(REPO_ROOT, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules");

export type DescriptorRouteGuardSpec = {
  readonly route: string;
  readonly root: string;
  readonly directoryNames: ReadonlySet<string>;
};

export type DescriptorRouteDecision = { readonly kind: "pass" } | { readonly kind: "missing"; readonly moduleDirectory: string };

/** @emoji 🛂️ Resolves only canonical declared module descriptor requests, without decoding arbitrary filesystem paths. */
export function descriptorRouteDecision(url: string | undefined, specs: readonly DescriptorRouteGuardSpec[]): DescriptorRouteDecision {
  if (!url) return { kind: "pass" };
  let pathname: string;
  try {
    pathname = decodeURIComponent(new URL(url, "http://127.0.0.1").pathname);
  } catch {
    return { kind: "pass" };
  }
  for (const spec of specs) {
    const route = spec.route.endsWith("/") ? spec.route : `${spec.route}/`;
    if (!pathname.startsWith(route)) continue;
    const parts = pathname.slice(route.length).split("/");
    if (parts.length !== 2 || parts[1] !== "🔣️.json") return { kind: "pass" };
    const moduleDirectory = parts[0] ?? "";
    if (!spec.directoryNames.has(moduleDirectory) || !existsSync(join(spec.root, moduleDirectory, "🔣️.json"))) return { kind: "missing", moduleDirectory };
    return { kind: "pass" };
  }
  return { kind: "pass" };
}

/** @emoji 🚫️ Prevents a missing plugin descriptor from falling through to Vite's HTML SPA response. */
export function semioDescriptorRouteGuardVitePlugin(specs: readonly DescriptorRouteGuardSpec[]) {
  return {
    name: "semio-descriptor-route-guard",
    enforce: "pre" as const,
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        const decision = descriptorRouteDecision(req.url, specs);
        if (decision.kind === "pass") return next();
        res.statusCode = 404;
        res.setHeader("content-type", "application/json");
        res.end(`${JSON.stringify({ error: "descriptor-not-found", moduleDirectory: decision.moduleDirectory })}\n`);
      });
    },
  };
}

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

export async function backboneDbHandleFor(dbPath: string): Promise<BackboneSqliteHandle> {
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

/** 🧹️ Eliminates in-source test branches before production asset URL collection. */
export function semioProductionTestBoundaryVitePlugin(): { name: string; enforce: "pre"; apply: "build"; transform(source: string, id: string): Promise<{ code: string; map: string } | null> } {
  return {
    name: "semio-production-test-boundary",
    enforce: "pre",
    apply: "build",
    async transform(source, id) {
      if (!source.includes("import.meta.vitest") || !/\.[cm]?[jt]sx?(?:[?#].*)?$/u.test(id)) return null;
      const { transformWithEsbuild } = await import("vite");
      const result = await transformWithEsbuild(source, id, { define: { "import.meta.vitest": "undefined" }, minifySyntax: true, target: "esnext", charset: "utf8", jsx: "preserve", sourcemap: true });
      return { code: result.code, map: JSON.stringify(result.map) };
    },
  };
}

/** @emoji 💾️ Vite middleware for browser file/folder backbone IO: `GET|PUT ${BACKBONE_ENDPOINT_PATH}?uri=&documentId=&schema=`
 * for read/write, plus `GET ${BACKBONE_ENDPOINT_PATH}/watch?uri=` (SSE) for external-edit notification —
 * `🧵️backbone-worker.ts`'s folder transport degrades to polling if this endpoint isn't reachable. */
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
 * (or reconnects) after some builds already finished must still learn about them — `♻️hot-swap.json` alone
 * only ever holds the single most recent build, not the full history. `root` is overridable so this can
 * be exercised against a throwaway temp dir in-source below rather than the real (build-dependent, so
 * flaky) `plugin-modules/` tree. */
export function scanBuiltPluginModules(root: string = PLUGIN_MODULES_ROOT): readonly PluginHotSwapMarker[] {
  if (!existsSync(root)) return [];
  const rows: PluginHotSwapMarker[] = [];
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory() || !moduleIdForDirectoryName(entry.name)) continue;
    const pluginDir = join(root, entry.name);
    let newestMs = 0;
    for (const file of readdirSync(pluginDir)) {
      if (!/\.core\d*\.wasm$/.test(file)) continue;
      newestMs = Math.max(newestMs, statSync(join(pluginDir, file)).mtimeMs);
    }
    const pluginId = moduleIdForDirectoryName(entry.name);
    if (newestMs > 0 && pluginId) rows.push({ pluginId, rebuiltAt: Math.round(newestMs) });
  }
  return rows;
}

/** @emoji 🔌️ OS-owned watcher route supplied explicitly to the neutral kernel source adapter. */
export const PLUGIN_SOURCE_WATCH_PATH = `${MODULE_PLUGIN_ROUTE}/watch`;

/** @emoji 🔌️ Vite middleware backing the shell's `createDevPluginSource` (`@semio-tech/framework`):
 * SSE at `PLUGIN_SOURCE_WATCH_PATH`, mirroring `semioBackboneVitePlugin`'s `/watch` endpoint. Sends one
 * `snapshot` on connect ({@link scanBuiltPluginModules}), then a `built` event every time `buildPlugin`
 * overwrites the shared `♻️hot-swap.json` marker — `buildPlugin` writes it last, after every other output
 * file, so by the time this fires the plugin's module is actually fetchable. Debounced the same 200ms
 * as `subscribeFolderWatch` above (a burst of writes during one build collapses to a single event). One
 * `fs.watch` on `plugin-modules/` for the whole dev server's lifetime — unlike the backbone plugin's
 * per-uri watchers, there is exactly one watch target here, so it is never torn down. */
export function semioPluginHotSwapVitePlugin() {
  return {
    name: "semio-plugin-hot-swap",
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void } }) {
      const subscribers = new Set<BackboneServerResponse>();
      mkdirSync(PLUGIN_MODULES_ROOT, { recursive: true });
      const hotSwapMarker = join(PLUGIN_MODULES_ROOT, MODULE_HOT_SWAP_FILE);
      let debounceTimer: ReturnType<typeof setTimeout> | undefined;
      watch(PLUGIN_MODULES_ROOT, (_eventType, filename) => {
        if (filename !== MODULE_HOT_SWAP_FILE) return;
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
        if (moduleRoutePath(req.url ?? "") !== PLUGIN_SOURCE_WATCH_PATH || req.method !== "GET") return next();
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

//#region BlobVitePlugin
let blobDatabaseSingleton: InstanceType<typeof import("bun:sqlite").Database> | undefined;

/** 🗄️ Lazily opens the dev-session-wide content-addressed blob store at `<repoRoot>/.🧬semio/🔗space/blobs.db` —
 * unlike backbone documents, blobs aren't scoped to a per-uri folder (there's no folder in the
 * `write-blob`/`read-blob` WIT signature), so this is one shared table for the whole dev server. */
async function blobDatabase(): Promise<InstanceType<typeof import("bun:sqlite").Database>> {
  if (!blobDatabaseSingleton) {
    const Database = await backboneDatabaseCtorLazy();
    const dbPath = join(REPO_ROOT, ".🧬semio", "🔗space", "blobs.db");
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
