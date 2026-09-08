/** @emoji 🔌️ The dev server's own Vite plugins — backbone document IO, the content-addressed blob
 * endpoint, the plugin hot-swap SSE stream and the production test boundary — kept in a module of
 * their own so `⚙️vite.config.ts` can mount them without pulling `📜️script.ts`'s task router (and
 * through it the repository library's discovery walk) into Vite's config bundle. `bun:sqlite` stays a
 * lazy dynamic import: Vite loads this module's exports under Node before the dev server's Bun
 * runtime exists. */
import { createHash, randomBytes } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, watch, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { BACKBONE_ENDPOINT_PATH, BLOB_ENDPOINT_PATH, backboneKindFromUri, decodeDocumentPackBytes, encodeDocumentPackBytes } from "@semio-tech/framework-os";
import type { PluginSourceEvent } from "@semio-tech/framework";
import { MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, moduleIdForDirectoryName, moduleRoutePath } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { observeActivationReceipts, type ActivationReceipt } from "../../♻️activation/🟦️.ts";
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

export const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH = `${BACKBONE_ENDPOINT_PATH}/canonical-bootstrap`;
export const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES = 64 * 1024 * 1024;
const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES = CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES + 10;
const SHA256_HEX = /^[0-9a-f]{64}$/u;

export type CanonicalBootstrapFolderMirrorFrontierV1 = {
  readonly documentId: string;
  readonly headEditOrdinal: number;
  readonly headEditId: string;
  readonly lastCommitSeq: number;
  readonly chainSha256: string;
};

export type CanonicalBootstrapFolderMirrorReserveV1 = {
  readonly schema: "semio.backbone.canonical-bootstrap-folder-mirror-reserve/v1";
  readonly artifactSchema: string;
  readonly descriptorDigestV1: string;
  readonly aggregateSha256: string;
  readonly baselineFrontier: CanonicalBootstrapFolderMirrorFrontierV1;
};

export type CanonicalBootstrapFolderMirrorOwnerV1 = {
  readonly schema: "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1";
  readonly epoch: number;
  readonly capability: string;
};

type CanonicalBootstrapFolderMirrorControlV1 = { readonly epoch: number; readonly capability: string };

function canonicalBootstrapMirrorError(code: "invalid" | "conflict" | "too-large"): Error {
  return Object.assign(new Error(`canonical bootstrap folder mirror ${code}`), { code });
}

function canonicalBootstrapFolderMirrorDbPath(uri: string): string {
  const folder = uri.slice("folder://".length);
  if (backboneKindFromUri(uri) !== "folder" || !isAbsolute(folder)) throw canonicalBootstrapMirrorError("invalid");
  return join(folder, ".semio", "documents.db");
}

function validateCanonicalBootstrapDocumentId(documentId: string): void {
  if (documentId.length === 0 || new TextEncoder().encode(documentId).byteLength > 512 || documentId.includes("\0")) throw canonicalBootstrapMirrorError("invalid");
}

function validateCanonicalBootstrapFrontier(frontier: CanonicalBootstrapFolderMirrorFrontierV1, documentId: string): void {
  const chainIsCanonical = SHA256_HEX.test(frontier?.chainSha256 ?? "");
  const chainIsZero = chainIsCanonical && frontier.chainSha256 === "0".repeat(64);
  const genesis =
    frontier?.documentId === documentId &&
    frontier.headEditOrdinal === 0 &&
    frontier.headEditId === "" &&
    frontier.lastCommitSeq === 0 &&
    chainIsZero;
  const edited =
    frontier?.documentId === documentId &&
    Number.isSafeInteger(frontier.headEditOrdinal) &&
    frontier.headEditOrdinal > 0 &&
    Number.isSafeInteger(frontier.lastCommitSeq) &&
    frontier.lastCommitSeq > 0 &&
    typeof frontier.headEditId === "string" &&
    frontier.headEditId.length > 0 &&
    new TextEncoder().encode(frontier.headEditId).byteLength <= 512 &&
    !/\p{Cc}/u.test(frontier.headEditId) &&
    chainIsCanonical &&
    !chainIsZero;
  if (!genesis && !edited) throw canonicalBootstrapMirrorError("invalid");
}

function validateCanonicalBootstrapReserve(request: CanonicalBootstrapFolderMirrorReserveV1, documentId: string): void {
  if (
    request?.schema !== "semio.backbone.canonical-bootstrap-folder-mirror-reserve/v1" ||
    typeof request.artifactSchema !== "string" ||
    request.artifactSchema.length === 0 ||
    !SHA256_HEX.test(request.descriptorDigestV1) ||
    !SHA256_HEX.test(request.aggregateSha256)
  )
    throw canonicalBootstrapMirrorError("invalid");
  validateCanonicalBootstrapFrontier(request.baselineFrontier, documentId);
}

function validateCanonicalBootstrapControl(control: CanonicalBootstrapFolderMirrorControlV1): void {
  if (!Number.isSafeInteger(control?.epoch) || control.epoch < 1 || typeof control.capability !== "string" || !SHA256_HEX.test(control.capability)) throw canonicalBootstrapMirrorError("invalid");
}

async function canonicalBootstrapFolderMirrorDb(uri: string): Promise<BackboneSqliteHandle> {
  const dbPath = canonicalBootstrapFolderMirrorDbPath(uri);
  mkdirSync(dirname(dbPath), { recursive: true });
  return backboneDbHandleFor(dbPath);
}

/** 🪪️ Mints and persists the sole current folder-mirror epoch; reserving a successor makes every prior stage and published pair non-current in the same transaction. */
export async function reserveCanonicalBootstrapFolderMirror(uri: string, documentId: string, request: CanonicalBootstrapFolderMirrorReserveV1): Promise<CanonicalBootstrapFolderMirrorOwnerV1> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapReserve(request, documentId);
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const capability = randomBytes(32).toString("hex");
  const reserve = db.transaction(() => {
    const prior = db.query("SELECT epoch FROM canonical_bootstrap_owner WHERE document_id = ?1").get(documentId) as { epoch?: number } | null;
    const epoch = Number(prior?.epoch ?? 0) + 1;
    if (!Number.isSafeInteger(epoch)) throw canonicalBootstrapMirrorError("conflict");
    db.run("DELETE FROM canonical_bootstrap_stage WHERE document_id = ?1", [documentId]);
    db.run(
      "INSERT INTO canonical_bootstrap_owner (document_id, epoch, capability, state, artifact_schema, descriptor_digest_v1, aggregate_sha256, baseline_frontier_json, updated_at) VALUES (?1, ?2, ?3, 'reserved', ?4, ?5, ?6, ?7, ?8) ON CONFLICT(document_id) DO UPDATE SET epoch = excluded.epoch, capability = excluded.capability, state = excluded.state, artifact_schema = excluded.artifact_schema, descriptor_digest_v1 = excluded.descriptor_digest_v1, aggregate_sha256 = excluded.aggregate_sha256, baseline_frontier_json = excluded.baseline_frontier_json, updated_at = excluded.updated_at",
      [documentId, epoch, capability, request.artifactSchema, request.descriptorDigestV1, request.aggregateSha256, JSON.stringify(request.baselineFrontier), Date.now()],
    );
    return epoch;
  });
  return { schema: "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1", epoch: reserve(), capability };
}

/** 🧱️ Stages one bounded, exactly framed canonical pair without changing folder-visible state. */
export async function stageCanonicalBootstrapFolderMirror(uri: string, documentId: string, control: CanonicalBootstrapFolderMirrorControlV1, payload: Uint8Array): Promise<void> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapControl(control);
  if (payload.byteLength > CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES) throw canonicalBootstrapMirrorError("too-large");
  let pack: Uint8Array, spr: Uint8Array;
  try {
    ({ pack, spr } = decodeDocumentPackBytes(payload));
  } catch {
    throw canonicalBootstrapMirrorError("invalid");
  }
  if (pack.byteLength + spr.byteLength > CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES) throw canonicalBootstrapMirrorError("too-large");
  const aggregate = createHash("sha256").update(pack).update(spr).digest("hex");
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const stage = db.transaction(() => {
    const owner = db.query("SELECT aggregate_sha256 AS aggregateSha256 FROM canonical_bootstrap_owner WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state = 'reserved'").get(documentId, control.epoch, control.capability) as { aggregateSha256?: string } | null;
    if (owner?.aggregateSha256 !== aggregate) throw canonicalBootstrapMirrorError("conflict");
    db.run("INSERT INTO canonical_bootstrap_stage (document_id, epoch, pack, spr, aggregate_sha256) VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(document_id, epoch) DO UPDATE SET pack = excluded.pack, spr = excluded.spr, aggregate_sha256 = excluded.aggregate_sha256", [documentId, control.epoch, pack, spr, aggregate]);
  });
  stage();
}

/** 📣️ Makes a staged pair current only while the exact server-minted epoch remains reserved. */
export async function publishCanonicalBootstrapFolderMirror(uri: string, documentId: string, control: CanonicalBootstrapFolderMirrorControlV1): Promise<void> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapControl(control);
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const publish = db.transaction(() => {
    const stage = db
      .query("SELECT 1 AS present FROM canonical_bootstrap_owner owner JOIN canonical_bootstrap_stage stage ON stage.document_id = owner.document_id AND stage.epoch = owner.epoch WHERE owner.document_id = ?1 AND owner.epoch = ?2 AND owner.capability = ?3 AND owner.state = 'reserved' AND stage.aggregate_sha256 = owner.aggregate_sha256")
      .get(documentId, control.epoch, control.capability) as { present?: number } | null;
    if (stage?.present !== 1) throw canonicalBootstrapMirrorError("conflict");
    const changed = db.run("UPDATE canonical_bootstrap_owner SET state = 'published', updated_at = ?4 WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state = 'reserved'", [documentId, control.epoch, control.capability, Date.now()]);
    if (changed.changes !== 1) throw canonicalBootstrapMirrorError("conflict");
  });
  publish();
}

/** 🧹️ Retires only the exact current epoch; a stale owner cannot hide or alter its successor. */
export async function retireCanonicalBootstrapFolderMirror(uri: string, documentId: string, control: CanonicalBootstrapFolderMirrorControlV1): Promise<void> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapControl(control);
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const retire = db.transaction(() => {
    const changed = db.run("UPDATE canonical_bootstrap_owner SET state = 'retired', updated_at = ?4 WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state IN ('reserved', 'published')", [documentId, control.epoch, control.capability, Date.now()]);
    if (changed.changes !== 1) throw canonicalBootstrapMirrorError("conflict");
    db.run("DELETE FROM canonical_bootstrap_stage WHERE document_id = ?1 AND epoch = ?2", [documentId, control.epoch]);
  });
  retire();
}

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
  db.run("CREATE TABLE IF NOT EXISTS canonical_bootstrap_owner (document_id TEXT PRIMARY KEY, epoch INTEGER NOT NULL, capability TEXT NOT NULL, state TEXT NOT NULL CHECK (state IN ('reserved', 'published', 'retired', 'generic')), artifact_schema TEXT NOT NULL, descriptor_digest_v1 TEXT NOT NULL, aggregate_sha256 TEXT NOT NULL, baseline_frontier_json TEXT NOT NULL, updated_at INTEGER NOT NULL)");
  db.run("CREATE TABLE IF NOT EXISTS canonical_bootstrap_stage (document_id TEXT NOT NULL, epoch INTEGER NOT NULL, pack BLOB NOT NULL, spr BLOB NOT NULL, aggregate_sha256 TEXT NOT NULL, PRIMARY KEY (document_id, epoch))");
  backboneDbHandles.set(dbPath, db);
  return db;
}

/** @emoji 🗂️ Same convention as `vcs::FolderSqliteStorage` (`.semio/documents.db`, a `document(id,
 * schema, json, updated_at)` table) so a folder-bound studio opened by the browser dev path and a
 * native (wgpu) reader agree on the same file. `documentId` defaults to the studio's own
 * single-document convention (mirrors os-core's `SPACE_FOLDER_DOCUMENT_ID`) when the caller doesn't
 * pass one — app documents (per `OsDocumentRef`) always pass their own id explicitly. */
const SPACE_FOLDER_DOCUMENT_ID = "studio";

export async function readBackbonePayload(uri: string, documentId: string | null): Promise<Uint8Array | null> {
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
    const canonical = db
      .query("SELECT owner.state, owner.aggregate_sha256 AS expectedAggregate, stage.aggregate_sha256 AS stagedAggregate, stage.pack, stage.spr FROM canonical_bootstrap_owner owner LEFT JOIN canonical_bootstrap_stage stage ON stage.document_id = owner.document_id AND stage.epoch = owner.epoch WHERE owner.document_id = ?1")
      .get(documentId ?? SPACE_FOLDER_DOCUMENT_ID) as { state?: string; expectedAggregate?: string; stagedAggregate?: string; pack?: Uint8Array; spr?: Uint8Array } | null;
    if (canonical && canonical.state !== "generic") {
      if (canonical.state !== "published" || !canonical.pack) return null;
      const pack = canonical.pack instanceof Uint8Array ? canonical.pack : new Uint8Array(canonical.pack as ArrayBuffer);
      const spr = canonical.spr instanceof Uint8Array ? canonical.spr : new Uint8Array((canonical.spr ?? []) as ArrayBuffer);
      if (canonical.stagedAggregate !== canonical.expectedAggregate || createHash("sha256").update(pack).update(spr).digest("hex") !== canonical.expectedAggregate) return null;
      return encodeDocumentPackBytes(pack, spr);
    }
    const row = db.query("SELECT pack, spr FROM document WHERE id = ?1").get(documentId ?? SPACE_FOLDER_DOCUMENT_ID) as { pack?: Uint8Array; spr?: Uint8Array } | null;
    if (!row?.pack) return null;
    const pack = row.pack instanceof Uint8Array ? row.pack : new Uint8Array(row.pack as ArrayBuffer);
    const spr = row.spr instanceof Uint8Array ? row.spr : new Uint8Array((row.spr ?? []) as ArrayBuffer);
    return encodeDocumentPackBytes(pack, spr);
  }
  return null;
}

export async function writeBackbonePayload(uri: string, documentId: string | null, schema: string | null, payload: Uint8Array): Promise<void> {
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
    const id = documentId ?? SPACE_FOLDER_DOCUMENT_ID;
    const write = db.transaction(() => {
      const canonical = db.query("SELECT state FROM canonical_bootstrap_owner WHERE document_id = ?1").get(id) as { state?: string } | null;
      if (canonical?.state === "reserved" || canonical?.state === "published") throw canonicalBootstrapMirrorError("conflict");
      if (canonical?.state === "retired") {
        db.run("DELETE FROM canonical_bootstrap_stage WHERE document_id = ?1", [id]);
        db.run("UPDATE canonical_bootstrap_owner SET state = 'generic', updated_at = ?2 WHERE document_id = ?1 AND state = 'retired'", [id, Date.now()]);
      }
      db.run("INSERT INTO document (id, schema, pack, spr, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, pack = excluded.pack, spr = excluded.spr, updated_at = excluded.updated_at", [id, schema ?? "", pack, spr, Date.now()]);
    });
    write();
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

type BackboneServerRequest = { method?: string; url?: string; headers?: Record<string, string | string[] | undefined>; on: (event: string, handler: (chunk?: unknown) => void) => void };
type BackboneServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; write: (chunk: string) => void; end: (body?: string | Uint8Array) => void };

function canonicalBootstrapMirrorControlFromHeaders(headers: BackboneServerRequest["headers"]): CanonicalBootstrapFolderMirrorControlV1 {
  const epochSource = headers?.["x-semio-canonical-bootstrap-epoch"];
  const authorization = headers?.authorization;
  if (Array.isArray(epochSource) || Array.isArray(authorization) || !/^[1-9][0-9]*$/u.test(epochSource ?? "") || !authorization?.startsWith("SemioFolderBootstrap ")) throw canonicalBootstrapMirrorError("invalid");
  const epoch = Number(epochSource);
  const control = { epoch, capability: authorization.slice("SemioFolderBootstrap ".length) };
  validateCanonicalBootstrapControl(control);
  return control;
}

function canonicalBootstrapMirrorContentType(headers: BackboneServerRequest["headers"]): string | null {
  const source = headers?.["content-type"];
  return typeof source === "string" ? source.trim().toLowerCase() : null;
}

function canonicalBootstrapMirrorStatus(error: unknown): number {
  const code = typeof error === "object" && error !== null && "code" in error ? (error as { code?: unknown }).code : undefined;
  return code === "conflict" ? 409 : code === "too-large" ? 413 : 400;
}

function collectBackboneRequestBody(req: BackboneServerRequest, limit: number, complete: (body: Uint8Array | null) => void): void {
  const chunks: Buffer[] = [];
  let length = 0;
  let exceeded = false;
  req.on("data", (chunk) => {
    if (exceeded) return;
    const bytes = Buffer.isBuffer(chunk) ? chunk : Buffer.from(String(chunk));
    length += bytes.byteLength;
    if (length > limit) {
      exceeded = true;
      chunks.length = 0;
      return;
    }
    chunks.push(bytes);
  });
  req.on("end", () => complete(exceeded ? null : new Uint8Array(Buffer.concat(chunks))));
}

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
        if (requestUrl.pathname.startsWith(`${CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH}/`)) {
          const action = requestUrl.pathname.slice(CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH.length + 1);
          if (documentId === null || backboneKindFromUri(uri) !== "folder" || requestUrl.searchParams.size !== 2 || !["reserve", "stage", "publish", "retire"].includes(action)) {
            res.statusCode = 400;
            res.end("invalid canonical bootstrap folder mirror request");
            return;
          }
          if ((action === "stage" && req.method !== "PUT") || (action !== "stage" && req.method !== "POST")) {
            res.statusCode = 405;
            res.end("method not allowed");
            return;
          }
          const expectedContentType = action === "reserve" ? "application/json" : action === "stage" ? "application/octet-stream" : null;
          if (canonicalBootstrapMirrorContentType(req.headers) !== expectedContentType) {
            res.statusCode = 415;
            res.end("unsupported media type");
            return;
          }
          const limit = action === "reserve" ? 8_192 : action === "stage" ? CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES : 0;
          collectBackboneRequestBody(req, limit, (body) => {
            void (async () => {
              if (body === null) throw canonicalBootstrapMirrorError("too-large");
              if (action === "reserve") {
                const source = new TextDecoder("utf-8", { fatal: true }).decode(body);
                const parsed = JSON.parse(source) as CanonicalBootstrapFolderMirrorReserveV1;
                if (JSON.stringify(parsed) !== source) throw canonicalBootstrapMirrorError("invalid");
                const owner = await reserveCanonicalBootstrapFolderMirror(uri, documentId, parsed);
                res.statusCode = 201;
                res.setHeader("content-type", "application/json");
                res.setHeader("cache-control", "no-store");
                res.end(`${JSON.stringify(owner)}\n`);
                return;
              }
              const control = canonicalBootstrapMirrorControlFromHeaders(req.headers);
              if (action === "stage") await stageCanonicalBootstrapFolderMirror(uri, documentId, control, body);
              else if (action === "publish") await publishCanonicalBootstrapFolderMirror(uri, documentId, control);
              else await retireCanonicalBootstrapFolderMirror(uri, documentId, control);
              res.statusCode = 204;
              res.end();
            })().catch((error) => {
              res.statusCode = canonicalBootstrapMirrorStatus(error);
              res.setHeader("content-type", "application/json");
              res.end(`${JSON.stringify({ error: error instanceof Error ? error.message : "canonical bootstrap folder mirror invalid" })}\n`);
            });
          });
          return;
        }
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
          collectBackboneRequestBody(req, CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES, (body) => {
            if (body === null) {
              res.statusCode = 413;
              res.end("payload too large");
              return;
            }
            writeBackbonePayload(uri, documentId, schema, body)
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
/** 📡️ Announces explicit Nx activation completion and releases every server-owned subscription. */
export function semioActivationVitePlugin(options: { readonly receiptDirectory: string }) {
  let dispose = (): void => {};
  return {
    name: "semio-activation",
    configureServer(server: {
      middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void };
      httpServer?: { once: (event: "close", listener: () => void) => unknown } | null;
      ws?: { send: (message: { type: "full-reload" }) => void };
    }) {
      dispose();
      const subscribers = new Map<BackboneServerResponse, () => void>();
      let previous: ActivationReceipt | undefined;
      const send = (event: PluginSourceEvent): void => {
        const text = `data: ${JSON.stringify(event)}\n\n`;
        for (const [response, stop] of subscribers) {
          try { response.write(text); } catch { stop(); subscribers.delete(response); }
        }
      };
      const observer = observeActivationReceipts(options.receiptDirectory, (receipt) => {
        if (previous) {
          if (previous.plugins.map((row) => row.pluginId).join() !== receipt.plugins.map((row) => row.pluginId).join()) server.ws?.send({ type: "full-reload" });
          const prior = new Map(previous.plugins.map((row) => [row.pluginId, row.artifactSha256]));
          for (const row of receipt.plugins) if (prior.get(row.pluginId) !== row.artifactSha256) send({ kind: "built", pluginId: row.pluginId, rebuiltAt: row.rebuiltAt });
        }
        previous = receipt;
      }, (error) => console.error("Activation receipt failed:", error));
      dispose = (): void => {
        observer.close();
        for (const [response, stop] of subscribers) { stop(); response.end(); }
        subscribers.clear();
      };
      server.httpServer?.once("close", dispose);
      server.middlewares.use((req, res, next) => {
        if (moduleRoutePath(req.url ?? "") !== PLUGIN_SOURCE_WATCH_PATH || req.method !== "GET") return next();
        res.statusCode = 200;
        res.setHeader("content-type", "text/event-stream");
        res.setHeader("cache-control", "no-cache");
        res.setHeader("connection", "keep-alive");
        res.write(": connected\n\n");
        const event: PluginSourceEvent = { kind: "snapshot", plugins: observer.snapshot().plugins.map(({ pluginId, rebuiltAt }) => ({ pluginId, rebuiltAt })) };
        res.write(`data: ${JSON.stringify(event)}\n\n`);
        const stop = startSseKeepalive(res);
        subscribers.set(res, stop);
        req.on("close", () => { stop(); subscribers.delete(res); });
      });
    },
    closeBundle(): void { dispose(); },
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
