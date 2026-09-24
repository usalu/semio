/** @emoji 🔌️ The dev server's own Vite plugins — backbone document IO, the content-addressed blob
 * endpoint, the plugin hot-swap SSE stream and the production test boundary — kept in a module of
 * their own so `⚙️vite.config.ts` can mount them without pulling `📜️script.ts`'s task router (and
 * through it the repository library's discovery walk) into Vite's config bundle. `bun:sqlite` stays a
 * lazy dynamic import: Vite loads this module's exports under Node before the dev server's Bun
 * runtime exists. */
import { spawn } from "node:child_process";
import { createHash, randomBytes } from "node:crypto";
import { chmodSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, watch, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { BACKBONE_ENDPOINT_PATH, BLOB_ENDPOINT_PATH, DOCUMENT_ARCHIVE_MAXIMUM_BYTES, backboneKindFromUri, decodeDocumentArchiveBytes } from "@semio-tech/framework-os";
import type { PluginSourceEvent } from "@semio-tech/framework";
import { MODULE_BRIDGE_FILE, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot, nextActivationReceipt, observeActivationReceipts, pluginModulesRoot, publishActivationReceipt, readActivationReceipt, resolveBootSourceContentHashes, stagedModuleMtime, stagedModuleReportLines, stagedModuleVerdict, writeStagedSourceFreshness, type ActivationReceipt, type StagedModuleFacts } from "../♻️activation/🟦️.ts";
import { blake3Hex } from "../../../../../🔨️modules/🔏️hash/🟦️.ts";
import { requestLocalBrokerSession } from "../../../../../../🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { DEV_LOCAL_HUB_DATA_ENV, DEV_LOCAL_HUB_PROFILE_ENV, DEV_LOCAL_HUB_SESSION_PATH } from "../🚀️local-hub/🏃️execution/🟦️.ts";
import { AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1, AGENT_CREDENTIAL_INSTALL_RECEIPT_SCHEMA_V1, AGENT_CREDENTIAL_INSTALL_SCHEMA_V1, AGENT_CREDENTIAL_SCHEMA_V1, agentCredentialInstallFileNameV1, isAgentDelegationTokenV1 } from "../../📇️directory/🤖️delegations/🟦️.ts";
/** @emoji 📥️ Filename owned by plugin store installation; inlined so the vite-plugin graph does not pull materialization. */
const EXTENSION_INSTALL_META = "📥️install.json";

/** @emoji 🗂️ Repository root derived from this module's own location — the config bundler must not
 * reach `getWorkspaceRoot` (and the discovery walk behind it) just to place two dev databases. */
export const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../..");

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

/** @emoji 👷️ Lets a service worker served from the module graph (the plugin module store's, `🌎️hub-source/👷️service-worker`)
 * control the whole shell: every script a browser fetches AS a service worker (`Service-Worker: script`) is allowed scope
 * `/`. A deployment answers its service worker script with the same header. */
export function semioServiceWorkerScopeVitePlugin() {
  return {
    name: "semio-service-worker-scope",
    enforce: "pre" as const,
    configureServer(server: { middlewares: { use: (handler: (req: { headers?: Record<string, string | string[] | undefined> }, res: { setHeader: (name: string, value: string) => void }, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (req.headers?.["service-worker"] === "script") res.setHeader("Service-Worker-Allowed", "/");
        next();
      });
    },
  };
}

//#region BackboneVitePlugin
/** 🗃️ The Bun builtin, ASSEMBLED at runtime. A bundler asked to ANALYSE this module refuses the
 * specifier outright ("Cannot bundle built-in module"), which failed whole jsdom test files whose
 * graph reaches here; `@vite-ignore` beside a constant was not enough, because a `const` holding a
 * string literal is exactly what the analyser constant-folds back into a static specifier. Joining
 * the two halves leaves nothing to fold, and the module's real type is restored in a TYPE position,
 * which the bundler never sees. Resolution is left to the runtime that actually has it. */
const BUN_SQLITE_MODULE = ["bun", "sqlite"].join(":");

/** Lazily imports `bun:sqlite` — a static top-level import breaks Vite's config bundler, which loads this module's exports under Node before the dev server (and its Bun runtime) exists. */
let backboneDatabaseCtor: typeof import("bun:sqlite").Database | undefined;
async function backboneDatabaseCtorLazy(): Promise<typeof import("bun:sqlite").Database> {
  if (backboneDatabaseCtor) return backboneDatabaseCtor;
  const { Database } = (await import(/* @vite-ignore */ BUN_SQLITE_MODULE)) as typeof import("bun:sqlite");
  backboneDatabaseCtor = Database;
  return Database;
}
type BackboneSqliteHandle = InstanceType<typeof import("bun:sqlite").Database>;

export const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH = `${BACKBONE_ENDPOINT_PATH}/canonical-bootstrap`;
export const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES = DOCUMENT_ARCHIVE_MAXIMUM_BYTES;
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
    db.run("DELETE FROM canonical_bootstrap_archive_stage WHERE document_id = ?1", [documentId]);
    db.run(
      "INSERT INTO canonical_bootstrap_owner (document_id, epoch, capability, state, artifact_schema, descriptor_digest_v1, aggregate_sha256, baseline_frontier_json, updated_at) VALUES (?1, ?2, ?3, 'reserved', ?4, ?5, ?6, ?7, ?8) ON CONFLICT(document_id) DO UPDATE SET epoch = excluded.epoch, capability = excluded.capability, state = excluded.state, artifact_schema = excluded.artifact_schema, descriptor_digest_v1 = excluded.descriptor_digest_v1, aggregate_sha256 = excluded.aggregate_sha256, baseline_frontier_json = excluded.baseline_frontier_json, updated_at = excluded.updated_at",
      [documentId, epoch, capability, request.artifactSchema, request.descriptorDigestV1, request.aggregateSha256, JSON.stringify(request.baselineFrontier), Date.now()],
    );
    return epoch;
  });
  return { schema: "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1", epoch: reserve(), capability };
}

/** 🧱️ Stages one bounded, exactly framed canonical recursive archive without changing folder-visible state. */
export async function stageCanonicalBootstrapFolderMirror(uri: string, documentId: string, control: CanonicalBootstrapFolderMirrorControlV1, payload: Uint8Array): Promise<void> {
  validateCanonicalBootstrapDocumentId(documentId);
  validateCanonicalBootstrapControl(control);
  if (payload.byteLength > CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BODY_BYTES) throw canonicalBootstrapMirrorError("too-large");
  let pack: Uint8Array, spr: Uint8Array;
  try {
    const archive = decodeDocumentArchiveBytes(payload);
    pack = Uint8Array.from(archive.parent_pack);
    spr = Uint8Array.from(archive.parent_spr);
  } catch {
    throw canonicalBootstrapMirrorError("invalid");
  }
  if (pack.byteLength + spr.byteLength > CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES) throw canonicalBootstrapMirrorError("too-large");
  const aggregate = createHash("sha256").update(pack).update(spr).digest("hex");
  const db = await canonicalBootstrapFolderMirrorDb(uri);
  const stage = db.transaction(() => {
    const owner = db.query("SELECT aggregate_sha256 AS aggregateSha256 FROM canonical_bootstrap_owner WHERE document_id = ?1 AND epoch = ?2 AND capability = ?3 AND state = 'reserved'").get(documentId, control.epoch, control.capability) as { aggregateSha256?: string } | null;
    if (owner?.aggregateSha256 !== aggregate) throw canonicalBootstrapMirrorError("conflict");
    db.run("INSERT INTO canonical_bootstrap_archive_stage (document_id, epoch, archive, aggregate_sha256) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(document_id, epoch) DO UPDATE SET archive = excluded.archive, aggregate_sha256 = excluded.aggregate_sha256", [documentId, control.epoch, payload, aggregate]);
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
      .query("SELECT 1 AS present FROM canonical_bootstrap_owner owner JOIN canonical_bootstrap_archive_stage stage ON stage.document_id = owner.document_id AND stage.epoch = owner.epoch WHERE owner.document_id = ?1 AND owner.epoch = ?2 AND owner.capability = ?3 AND owner.state = 'reserved' AND stage.aggregate_sha256 = owner.aggregate_sha256")
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
    db.run("DELETE FROM canonical_bootstrap_archive_stage WHERE document_id = ?1 AND epoch = ?2", [documentId, control.epoch]);
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
  db.run("CREATE TABLE IF NOT EXISTS document_archive (id TEXT PRIMARY KEY, schema TEXT, archive BLOB NOT NULL, updated_at INTEGER NOT NULL)");
  db.run("CREATE TABLE IF NOT EXISTS canonical_bootstrap_owner (document_id TEXT PRIMARY KEY, epoch INTEGER NOT NULL, capability TEXT NOT NULL, state TEXT NOT NULL CHECK (state IN ('reserved', 'published', 'retired', 'generic')), artifact_schema TEXT NOT NULL, descriptor_digest_v1 TEXT NOT NULL, aggregate_sha256 TEXT NOT NULL, baseline_frontier_json TEXT NOT NULL, updated_at INTEGER NOT NULL)");
  db.run("CREATE TABLE IF NOT EXISTS canonical_bootstrap_archive_stage (document_id TEXT NOT NULL, epoch INTEGER NOT NULL, archive BLOB NOT NULL, aggregate_sha256 TEXT NOT NULL, PRIMARY KEY (document_id, epoch))");
  backboneDbHandles.set(dbPath, db);
  return db;
}

/** @emoji 🗂️ Same `.semio/documents.db` convention as `vcs::FolderSqliteStorage` so a folder-bound studio opened by the browser dev path and a
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
      .query("SELECT owner.state, owner.aggregate_sha256 AS expectedAggregate, stage.aggregate_sha256 AS stagedAggregate, stage.archive FROM canonical_bootstrap_owner owner LEFT JOIN canonical_bootstrap_archive_stage stage ON stage.document_id = owner.document_id AND stage.epoch = owner.epoch WHERE owner.document_id = ?1")
      .get(documentId ?? SPACE_FOLDER_DOCUMENT_ID) as { state?: string; expectedAggregate?: string; stagedAggregate?: string; archive?: Uint8Array } | null;
    if (canonical && canonical.state !== "generic") {
      if (canonical.state !== "published" || !canonical.archive) return null;
      const bytes = canonical.archive instanceof Uint8Array ? canonical.archive : new Uint8Array(canonical.archive as ArrayBuffer);
      const archive = decodeDocumentArchiveBytes(bytes);
      const pack = Uint8Array.from(archive.parent_pack), spr = Uint8Array.from(archive.parent_spr);
      if (canonical.stagedAggregate !== canonical.expectedAggregate || createHash("sha256").update(pack).update(spr).digest("hex") !== canonical.expectedAggregate) return null;
      return bytes;
    }
    const row = db.query("SELECT archive FROM document_archive WHERE id = ?1").get(documentId ?? SPACE_FOLDER_DOCUMENT_ID) as { archive?: Uint8Array } | null;
    if (!row?.archive) return null;
    const bytes = row.archive instanceof Uint8Array ? row.archive : new Uint8Array(row.archive as ArrayBuffer);
    decodeDocumentArchiveBytes(bytes);
    return bytes;
  }
  return null;
}

export async function writeBackbonePayload(uri: string, documentId: string | null, schema: string | null, payload: Uint8Array): Promise<void> {
  const kind = backboneKindFromUri(uri);
  decodeDocumentArchiveBytes(payload);
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
        db.run("DELETE FROM canonical_bootstrap_archive_stage WHERE document_id = ?1", [id]);
        db.run("UPDATE canonical_bootstrap_owner SET state = 'generic', updated_at = ?2 WHERE document_id = ?1 AND state = 'retired'", [id, Date.now()]);
      }
      db.run("INSERT INTO document_archive (id, schema, archive, updated_at) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, archive = excluded.archive, updated_at = excluded.updated_at", [id, schema ?? "", payload, Date.now()]);
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

type BackboneServerRequest = { method?: string; url?: string; headers?: Record<string, string | string[] | undefined>; on: (event: string, handler: (chunk?: unknown) => void) => void; off: (event: string, handler: (chunk?: unknown) => void) => void };
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
 * `🏪️store/👷️worker/🟦️.ts`'s folder transport degrades to polling if this endpoint isn't reachable. */
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
export type PluginHotSwapMarker = { readonly pluginId: string; readonly rebuiltAt: number };

/** @emoji 🔌️ Every plugin dir under `root` that has a completed build right now (a `.core*.wasm`
 * present — same convention `collectPluginWasmSizeRows` walks), newest core-wasm mtime as `rebuiltAt`.
 * Backs the SSE endpoint's connect-time `snapshot` event: a browser that connects (or reconnects) after
 * some builds already finished must still learn about them — `♻️hot-swap.json` alone only ever holds the
 * single most recent build, not the full history. `root` is REQUIRED (never defaulted): the one staging
 * root is `pluginModulesRoot(profile)` in `♻️activation/🟦️.ts`, and a default here was how a second,
 * silently drifting module tree stayed alive. */
export function scanBuiltPluginModules(root: string): readonly PluginHotSwapMarker[] {
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
export function semioPluginHotSwapVitePlugin(options: { readonly moduleRoot: string }) {
  return {
    name: "semio-plugin-hot-swap",
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void } }) {
      const subscribers = new Set<BackboneServerResponse>();
      mkdirSync(options.moduleRoot, { recursive: true });
      const hotSwapMarker = join(options.moduleRoot, MODULE_HOT_SWAP_FILE);
      let debounceTimer: ReturnType<typeof setTimeout> | undefined;
      watch(options.moduleRoot, (_eventType, filename) => {
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
        const snapshot: PluginSourceEvent = { kind: "snapshot", plugins: scanBuiltPluginModules(options.moduleRoot) };
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
/** @emoji 🧩️ One watched component; `installDirectory` overrides `<installRoot>/<directoryName>` when a host serves extensions from several activation lanes. */
export type ActivationComponentSpec = Readonly<{ pluginId: string; directoryName: string; role: "plugin" | "extension"; sourceRoot: string; installDirectory?: string; cratePath?: string }>;

/** @emoji 🔎️ Re-runs the staged-module freshness rule against the receipt the dev server just observed and
 * prints one `[stale]` line per component whose served bytes are behind — the live half of the serve-start
 * pass in `📜️script.ts`. A restage that lands while the server runs therefore retires its own warning
 * without a restart, and one that never lands keeps saying so. */
export function reportActivationFreshness(receipt: ActivationReceipt, options: { readonly moduleRoot: string; readonly installRoot: string; readonly components: readonly ActivationComponentSpec[] }): readonly string[] {
  const activatedRows = new Map(receipt.plugins.map((row) => [row.pluginId, row]));
  const facts = options.components.map((component): StagedModuleFacts => {
    const moduleDirectory = join(options.moduleRoot, component.directoryName);
    const receiptRow = activatedRows.get(component.pluginId);
    const installedMeta = join(component.installDirectory ?? join(options.installRoot, component.directoryName), EXTENSION_INSTALL_META);
    let installedPackageHash: string | undefined;
    if (existsSync(installedMeta)) {
      try { installedPackageHash = JSON.parse(readFileSync(installedMeta, "utf8")).packageHash as string; } catch { installedPackageHash = undefined; }
    }
    const hashes = resolveBootSourceContentHashes({
      sourceRoot: component.sourceRoot,
      moduleDirectory,
      receiptSourceContentSha256: (receiptRow as { sourceContentSha256?: string } | undefined)?.sourceContentSha256,
    });
    return {
      pluginId: component.pluginId,
      role: component.role,
      activationTracked: true,
      stagedAtMs: stagedModuleMtime(moduleDirectory),
      newestSourcePath: hashes.newestSourcePath ? relative(REPO_ROOT, hashes.newestSourcePath).split(/[\\/]/).join("/") : undefined,
      sourceContentSha256: hashes.sourceContentSha256,
      stagedSourceContentSha256: hashes.stagedSourceContentSha256,
      receiptArtifactSha256: receiptRow?.artifactSha256,
      installedPackageHash,
    }
  });
  return stagedModuleReportLines(facts.map(stagedModuleVerdict), `bun nx run @semio-tech/framework-os-dev:activate-${receipt.variant}-react-${receipt.profile}`);
}


/** 📡️ Announces explicit Nx activation completion and releases every server-owned subscription. */
export function semioActivationVitePlugin(options: { readonly receiptDirectory: string; readonly moduleRoot: string; readonly installRoot: string; readonly components: readonly ActivationComponentSpec[] }) {
  let dispose = (): void => {};
  let staleness: readonly string[] = [];
  return {
    name: "semio-activation",
    enforce: "pre" as const,
    /** @emoji 📣️ The staged-module verdict belongs in the DEVELOPER's console, not only in the server log
     * they are not reading: a guest module staged behind its own source serves a wire contract the host
     * TypeScript in the same page no longer speaks, and the symptom (`actor-ui-patch.pairing`, a window
     * booting a fallback graph) never names its cause. */
    transformIndexHtml: {
      order: "post" as const,
      handler() {
        if (staleness.length === 0) return [];
        const message = `semio dev · ${staleness.length} staged plugin module(s) are behind their source — the host in this page may speak a newer wire contract than the guest it is talking to:\n${staleness.join("\n")}`;
        return [{ tag: "script", attrs: { type: "module" }, children: `console.warn(${JSON.stringify(message)});` }];
      },
    },
    configureServer(server: {
      middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void };
      httpServer?: { listening: boolean; once: (event: "close" | "listening", listener: () => void) => unknown } | null;
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
        const apply = (): void => {
          staleness = reportActivationFreshness(receipt, options);
          for (const line of staleness) console.warn(line);
          if (previous) {
            if (previous.plugins.map((row) => row.pluginId).join() !== receipt.plugins.map((row) => row.pluginId).join()) server.ws?.send({ type: "full-reload" });
            const prior = new Map(previous.plugins.map((row) => [row.pluginId, row.artifactSha256]));
            for (const row of receipt.plugins) if (prior.get(row.pluginId) !== row.artifactSha256) send({ kind: "built", pluginId: row.pluginId, rebuiltAt: row.rebuiltAt });
          }
          previous = receipt;
        };
        // Serve-start owns the boot freshness pass. Skip the duplicate sync walk on first listen.
        if (!server.httpServer?.listening) {
          previous = receipt;
          return;
        }
        if (previous === undefined) {
          previous = receipt;
          return;
        }
        apply();
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

      const jobs = new Map<string, { controller: AbortController; promise: Promise<void> }>();
      const profile: "dev" | "release" = /(?:^|\/)release(?:\/|$)/.test(options.moduleRoot.replaceAll("\\", "/")) ? "release" : "dev";

      const resolveProject = (component: ActivationComponentSpec): string => {
        const roots = [component.cratePath ? join(REPO_ROOT, component.cratePath) : "", component.sourceRoot, join(component.sourceRoot, "packages")].filter(Boolean);
        for (const base of roots) {
          if (!existsSync(base)) continue;
          try {
            for (const name of readdirSync(base)) {
              if (!name.endsWith("project.json")) continue;
              const parsed = JSON.parse(readFileSync(join(base, name), "utf8")) as { name?: string };
              if (typeof parsed.name === "string" && parsed.name) return parsed.name;
            }
          } catch { /* continue */ }
          try {
            for (const child of readdirSync(base)) {
              const nested = join(base, child);
              if (!existsSync(nested)) continue;
              try {
                for (const name of readdirSync(nested)) {
                  if (!name.endsWith("project.json")) continue;
                  const parsed = JSON.parse(readFileSync(join(nested, name), "utf8")) as { name?: string };
                  if (typeof parsed.name === "string" && parsed.name) return parsed.name;
                }
              } catch { continue; }
            }
          } catch { continue; }
        }
        throw new Error(`No nx project for plugin ${component.pluginId}`);
      };

      const publishOne = async (pluginId: string): Promise<void> => {
        const component = options.components.find((row) => row.pluginId === pluginId);
        if (!component) throw new Error(`Unknown plugin ${pluginId}`);
        const moduleDirectory = join(options.moduleRoot, component.directoryName);
        if (!existsSync(join(moduleDirectory, MODULE_BRIDGE_FILE))) throw new Error(`Module still missing after materialize: ${pluginId}`);
        const sourceContentSha256 = writeStagedSourceFreshness(moduleDirectory, component.sourceRoot);
        const previous = existsSync(join(options.receiptDirectory, ACTIVATION_RECEIPT_FILE)) ? readActivationReceipt(options.receiptDirectory) : undefined;
        const artifactSha256 = createHash("sha256").update(readFileSync(join(moduleDirectory, MODULE_BRIDGE_FILE))).digest("hex");
        const completed = [
          ...(previous?.plugins.filter((row) => row.pluginId !== pluginId) ?? []).map((row) => ({ pluginId: row.pluginId, artifactSha256: row.artifactSha256, sourceContentSha256: (row as { sourceContentSha256?: string }).sourceContentSha256 })),
          { pluginId, artifactSha256, sourceContentSha256 },
        ];
        const variant = previous?.variant ?? process.env.SEMIO_PLUGIN ?? "s";
        const receiptProfile = previous?.profile ?? profile;
        const receipt = nextActivationReceipt(variant, receiptProfile, completed, previous);
        publishActivationReceipt(options.receiptDirectory, receipt);
      };

      const materialize = (pluginId: string): Promise<void> => {
        const existing = jobs.get(pluginId);
        if (existing) return existing.promise;
        const component = options.components.find((row) => row.pluginId === pluginId);
        if (!component) return Promise.reject(new Error(`Unknown plugin ${pluginId}`));
        const controller = new AbortController();
        const promise = (async () => {
          const project = resolveProject(component);
          const target = `${project}:materialize-${profile}`;
          
          for (const [subscriber] of subscribers) {
            try { subscriber.write(`: lazy-activate ${pluginId} via ${target}\n\n`); } catch { /* closed */ }
          }
          console.log(`[lazy-activate] materialize ${pluginId} via ${target}`);
          await new Promise<void>((resolvePromise, reject) => {
            // Restage only: skip `component-*` dependsOn so a missing bridge does not rebuild wasm
            // (and does not queue behind the fleet wasm mutex). Component outputs must already exist.
            const child = spawn("bun", ["nx", "run", target, "--excludeTaskDependencies"], { cwd: REPO_ROOT, env: process.env, stdio: ["ignore", "pipe", "pipe"] });
            const onAbort = (): void => { child.kill("SIGTERM"); };
            controller.signal.addEventListener("abort", onAbort, { once: true });
            let stderr = "";
            const progress = (chunk: Buffer | string): void => {
              const line = String(chunk).trim();
              if (!line) return;
              for (const [subscriber] of subscribers) {
                try { subscriber.write(`: lazy-activate-progress ${pluginId} ${line.slice(0, 200)}\n\n`); } catch { /* closed */ }
              }
            };
            child.stdout?.on("data", progress);
            child.stderr?.on("data", (chunk: Buffer | string) => { stderr += String(chunk); progress(chunk); });
            child.on("error", reject);
            child.on("exit", (code) => {
              controller.signal.removeEventListener("abort", onAbort);
              if (controller.signal.aborted) return reject(new Error(`cancelled ${pluginId}`));
              if (code !== 0) return reject(new Error(`materialize failed ${target}: ${stderr.slice(-500)}`));
              resolvePromise();
            });
          });
          await publishOne(pluginId);
          const rebuiltAt = readActivationReceipt(options.receiptDirectory).plugins.find((row) => row.pluginId === pluginId)?.rebuiltAt ?? Date.now();
          send({ kind: "built", pluginId, rebuiltAt });
        })().finally(() => { jobs.delete(pluginId); });
        jobs.set(pluginId, { controller, promise });
        return promise;
      };

      server.middlewares.use((req, res, next) => {
        const path = moduleRoutePath(req.url ?? "");
        if (!path || req.method !== "GET") return next();
        const prefix = MODULE_PLUGIN_ROUTE.endsWith("/") ? MODULE_PLUGIN_ROUTE : `${MODULE_PLUGIN_ROUTE}/`;
        if (!path.startsWith(prefix)) return next();
        const directoryName = path.slice(prefix.length).split("/")[0] ?? "";
        if (!directoryName || directoryName === "watch") return next();
        const pluginId = moduleIdForDirectoryName(directoryName);
        if (!pluginId) return next();
        if (existsSync(join(options.moduleRoot, directoryName, MODULE_BRIDGE_FILE))) return next();
        const cancel = (): void => { jobs.get(pluginId)?.controller.abort(); };
        req.on("close", cancel);
        void materialize(pluginId).then(() => { req.off("close", cancel); next(); }).catch((error) => {
          req.off("close", cancel);
          res.statusCode = 503;
          res.setHeader("content-type", "application/json");
          res.end(`${JSON.stringify({ error: "lazy-activate-failed", pluginId, detail: String(error) })}\n`);
        });
      });

      const prefetch = (): void => {
        const pending = options.components.filter((row) => !existsSync(join(options.moduleRoot, row.directoryName, MODULE_BRIDGE_FILE)));
        void (async () => {
          for (const row of [...pending].sort((a, b) => (a.pluginId < b.pluginId ? -1 : 1))) {
            if (existsSync(join(options.moduleRoot, row.directoryName, MODULE_BRIDGE_FILE))) continue;
            try { await materialize(row.pluginId); }
            catch (error) { console.warn(`[lazy-activate] prefetch ${row.pluginId}: ${String(error)}`); }
          }
        })();
      };
      server.httpServer?.once("listening", prefetch);
      const priorDispose = dispose;
      dispose = (): void => {
        for (const job of jobs.values()) job.controller.abort();
        jobs.clear();
        priorDispose();
      };
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

//#region 🔖️SourceFreshnessVitePlugins
/** @emoji 🚫️ Repository directory names no dev server may ever watch: version control metadata, the Nx
 * workspace store, the package store, the shared build/cache root, compiled output and generated
 * sources. Tools rewrite millions of files inside them while a dev session is open, and every such write
 * would otherwise be delivered into the dev server's event loop.
 *
 * `🤖️generated` and `.vscode` stay listed for the reason Vite's own `server.watch.ignored` once carried:
 * those files are config dependencies, so reacting to a registry or launch-config rewrite restarts the
 * server in a loop.
 * @see https://github.com/paulmillr/chokidar/blob/3.6.0/lib/fsevents-handler.js */
export const UNWATCHED_REPOSITORY_SEGMENTS: readonly string[] = [".git", ".nx", ".vscode", ".🧬semio", "node_modules", "dist", "target", "🤖️generated", "🗑️generated"];

/** @emoji 👁️ The repository's top-level source directories — every place a dev server's module graph can
 * legitimately import from, with the unwatchable stores above removed. Read from disk rather than
 * hardcoded so a new top-level product directory is watched without touching this module. */
export function repositorySourceWatchRoots(repoRoot: string): readonly string[] {
  const excluded = new Set(UNWATCHED_REPOSITORY_SEGMENTS);
  return readdirSync(repoRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !excluded.has(entry.name)).map((entry) => join(repoRoot, entry.name)).sort();
}

/** @emoji 🧹️ Matches any relative path that crosses an unwatched store, on both `/` and `\` separators.
 * One precompiled test per filesystem event is the whole per-event budget this watcher may spend. */
export function unwatchedRepositoryPathMatcher(): RegExp {
  const alternatives = UNWATCHED_REPOSITORY_SEGMENTS.map((segment) => segment.replaceAll(".", "\\.")).join("|");
  return new RegExp(`(?:^|[\\\\/])(?:${alternatives})(?:[\\\\/]|$)`, "u");
}

/** @emoji 🛰️ Drives Vite's file-change pipeline from `node:fs` recursive watches over the repository's
 * source roots, so `⚙️vite.config.ts` can hand Vite `server.watch: null` and run no chokidar watcher of
 * its own.
 *
 * Vite watches its `root` plus — through `ensureWatchedFile` — every module-graph file outside it, which
 * in this repository is roughly a thousand individual paths spread across several top-level directories.
 * On macOS chokidar answers that by consolidating sibling FSEvents streams upward until ONE stream covers
 * the whole repository, then runs every watched path's prefix filter against every event that stream
 * delivers. A concurrent `cargo` build writing into the shared cache therefore costs the dev server
 * `events × watched paths` string comparisons — measured at 6 291 events per 2 s against 1 316 watched
 * paths, which blocks the event loop for ~2 s at a time, allocates ~600 MB per burst and, once resident
 * memory reaches the runtime's ceiling, wedges the server permanently. `server.watch.ignored` cannot undo
 * this: chokidar consults it only after those prefix filters have already run.
 *
 * Watching the source roots directly keeps the kernel from ever reporting cache, package-store or
 * generated-output writes, and reduces the per-event cost to one regular-expression test. Events are
 * replayed on Vite's own (no-op) watcher emitter, so module invalidation, HMR boundary computation and
 * config-dependency restarts behave exactly as they did with chokidar.
 *
 * 🛰️ An existing FILE is replayed as `add` AND `change`, because macOS reports every write to it —
 * in place and atomic (temp + rename) alike — as `eventType: "rename"`, while Vite invalidates a
 * transformed module only from its `change` handler (`moduleGraph.onFileChange`); its `add` handler
 * recovers previously failed resolves and never touches the module graph. Chokidar told the two apart
 * from its own directory snapshots, which this watcher deliberately does not keep — so it states both
 * facts, which are both true of an atomic save (a new inode appeared, and the module changed) and
 * idempotent for a genuinely new file (nothing imports it yet, so the `change` finds no module).
 * Emitting only `add` served the pre-edit transform for the life of the server, and
 * `SEMIO_VITE_HMR=0` (`hmr: false`) removes the HMR pass that would otherwise have hidden it
 * (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §4.2). */
const REACT_REFRESH_RUNTIME = "/@react-refresh";

/** @emoji ⚛️ Preamble copied from `@vitejs/plugin-react` — semio-host-html replaces the whole document in
 * `transformIndexHtml` `order: "pre"`, so the react plugin's own preamble injection must be reinforced in
 * `order: "post"` or `@react-three/fiber` (and every other JSX dep) throws "can't detect preamble". */
function semioReactRefreshPreambleScript(base: string): string {
  const root = base.endsWith("/") ? base.slice(0, -1) : base;
  return `import { injectIntoGlobalHook } from "${root}${REACT_REFRESH_RUNTIME}";
injectIntoGlobalHook(window);
window.$RefreshReg$ = () => {};
window.$RefreshSig$ = () => (type) => type;`;
}

/** @emoji ⚛️ Aligns Vite 7 / Rolldown OXC JSX refresh with `server.hmr` — `SEMIO_VITE_HMR=0` must not emit
 * `$RefreshReg$` wrappers without the HTML preamble, and HMR-on serves must always ship that preamble even
 * after {@link semioHostHtmlVitePlugin} rebuilds `index.html`. */
export function semioPlaygroundReactRefreshCoherenceVitePlugin() {
  return {
    name: "semio-playground-react-refresh-coherence",
    enforce: "post" as const,
    config(userConfig: { readonly server?: { readonly hmr?: unknown } }, { command }: { readonly command: string }) {
      if (command !== "serve" || userConfig.server?.hmr !== false) return;
      return {
        esbuild: { jsxDev: false },
        oxc: { jsx: { refresh: false } },
        optimizeDeps: { esbuildOptions: { jsxDev: false } },
      };
    },
    transformIndexHtml: {
      order: "post" as const,
      handler(html: string, ctx: { readonly server?: { readonly config: { readonly base?: string; readonly server: { readonly hmr?: unknown } } } }) {
        if (ctx.server?.config.server.hmr === false) return;
        if (html.includes("injectIntoGlobalHook")) return;
        const base = ctx.server?.config.base ?? "/";
        return [{ tag: "script", attrs: { type: "module" }, children: semioReactRefreshPreambleScript(base) }];
      },
    },
  };
}

/** @emoji 🧾️ The `{mtimeMs, size}` pair a transformed module's file carried when the dev server last read
 * it. Two facts rather than one: a same-second rewrite of a different length moves `size` while `mtimeMs`
 * can still round to the same millisecond on some filesystems. */
export type SourceStamp = { readonly mtimeMs: number; readonly size: number };

/** @emoji 🔍️ Remembers what every transformed module's file looked like on disk when its transform was
 * produced, and answers which of them have moved since.
 *
 * ONLY files the dev server has actually transformed are tracked, so "moved" is exactly "the cached
 * transform is out of date": a file the server never read has no cached transform to be stale. The
 * per-directory index makes the answer to "did anything in THIS directory change" cost one stat per
 * tracked sibling, which is what turns a filesystem event naming an editor's temporary file into the
 * invalidation of the module that temporary file was renamed onto. */
export function createSourceFreshnessRegistry() {
  const stamps = new Map<string, SourceStamp>();
  const siblings = new Map<string, Set<string>>();
  const readStamp = (file: string): SourceStamp | null => {
    try {
      const status = statSync(file);
      return { mtimeMs: status.mtimeMs, size: status.size };
    } catch {
      return null;
    }
  };
  const moved = (file: string): boolean => {
    const previous = stamps.get(file);
    if (previous === undefined) return false;
    const current = readStamp(file);
    if (current === null) return false;
    if (current.mtimeMs === previous.mtimeMs && current.size === previous.size) return false;
    stamps.set(file, current);
    return true;
  };
  return {
    record(file: string): void {
      const stamp = readStamp(file);
      if (stamp === null) return;
      const directory = dirname(file);
      const tracked = siblings.get(directory) ?? new Set<string>();
      tracked.add(file);
      siblings.set(directory, tracked);
      stamps.set(file, stamp);
    },
    trackedCount: (): number => stamps.size,
    movedFile: (file: string): boolean => moved(file),
    movedInDirectory: (directory: string): readonly string[] => [...(siblings.get(directory) ?? [])].filter(moved),
    movedEverywhere: (): readonly string[] => [...stamps.keys()].filter(moved),
  };
}

export type SourceFreshnessRegistry = ReturnType<typeof createSourceFreshnessRegistry>;

type FreshnessServer = {
  readonly watcher: { emit(event: string, path: string): boolean };
  readonly httpServer: { once(event: "close", listener: () => void): void } | null;
  readonly environments?: Record<string, { readonly moduleGraph: { onFileChange(file: string): void } }>;
  readonly middlewares?: { use(handler: (request: { url?: string; headers: Record<string, string | string[] | undefined> }, response: unknown, next: () => void) => void): void };
  readonly config?: { readonly root: string; readonly server: { readonly hmr?: unknown } };
};

/** @emoji ♻️ Retires every cached transform of one file, synchronously for the request in flight and then
 * through Vite's own file-change pipeline for everything downstream of it (plugin `watchChange`, HMR
 * boundaries, config-dependency restarts). `onFileChange` walks importers, so an importer that inlined
 * the edited module's output is retired with it. */
function retireStaleModule(server: FreshnessServer, file: string): void {
  for (const environment of Object.values(server.environments ?? {})) environment.moduleGraph.onFileChange(file);
  server.watcher.emit("change", file);
}

export function semioSourceWatchVitePlugin(options: { readonly repoRoot: string; readonly freshness?: Pick<SourceFreshnessRegistry, "movedInDirectory"> }) {
  return {
    name: "semio-source-watch",
    apply: "serve" as const,
    configureServer(server: FreshnessServer) {
      const unwatched = unwatchedRepositoryPathMatcher();
      const freshness = options.freshness;
      const handles = repositorySourceWatchRoots(options.repoRoot).map((root) => watch(root, { recursive: true, persistent: false }, (eventType, name) => {
        if (name === null || unwatched.test(name)) return;
        const path = join(root, name);
        const directory = dirname(path);
        if (eventType !== "rename") {
          server.watcher.emit("change", path);
          for (const moved of freshness?.movedInDirectory(directory) ?? []) if (moved !== path) server.watcher.emit("change", moved);
          return;
        }
        const entry = statSync(path, { throwIfNoEntry: false });
        if (!entry) {
          server.watcher.emit("unlink", path);
          for (const moved of freshness?.movedInDirectory(directory) ?? []) server.watcher.emit("change", moved);
          return;
        }
        if (entry.isDirectory()) {
          server.watcher.emit("addDir", path);
          for (const moved of freshness?.movedInDirectory(path) ?? []) server.watcher.emit("change", moved);
          return;
        }
        server.watcher.emit("add", path);
        server.watcher.emit("change", path);
        for (const moved of freshness?.movedInDirectory(directory) ?? []) if (moved !== path) server.watcher.emit("change", moved);
      }));
      server.httpServer?.once("close", () => {
        for (const handle of handles) handle.close();
      });
    },
  };
}

/** @emoji 🗺️ The absolute file a dev-server request would be transformed from, or `null` for a request no
 * module graph entry can back (virtual ids, client runtime, the index document). `/@fs/` carries the
 * absolute path the module graph is keyed by; everything else is relative to Vite's `root`. */
export function requestedTransformFile(url: string, root: string): string | null {
  const [pathname] = url.split("?");
  if (pathname === "" || pathname === "/" || pathname.endsWith("/")) return null;
  let decoded: string;
  try {
    decoded = decodeURIComponent(pathname);
  } catch {
    return null;
  }
  if (decoded.startsWith("/@fs/")) return decoded.slice("/@fs".length);
  if (decoded.startsWith("/@") || decoded.startsWith("/\0") || decoded.includes("\0")) return null;
  return join(root, decoded);
}

/** @emoji 🛡️ Proves, at request time, that every transform this dev server is about to serve was produced
 * from the bytes currently on disk — the guarantee the filesystem watcher alone cannot give.
 *
 * macOS reports a recursive `fs.watch` event by the path whose directory entry changed, and an atomic
 * save (write a temporary file, `rename` it onto the target) changes the entry of the TEMPORARY file. The
 * edited module's own path is then never named by any event — measured 0 times in 5 at every module depth,
 * for `sed -i ''`, for a rename-into-place and for every editor that saves atomically, which is all of
 * them. `SEMIO_VITE_HMR=0` removes the HMR pass that would otherwise have papered over it, so the dev
 * server keeps serving the pre-edit transform until the process is recycled: a developer moves a slider
 * and the preview runs yesterday's module.
 *
 * {@link semioSourceWatchVitePlugin} now answers such an event by re-stating its whole directory, which
 * repairs the common case at edit time. This plugin is the guarantee underneath it, and it does not
 * depend on any event arriving at all: every module request re-stats the one file behind it, and every
 * document request re-stats the whole transformed set, so the very next request after any edit — by any
 * tool, through any write style, with the watcher armed or not — serves the current file.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts */
export function semioTransformFreshnessVitePlugin(options: { readonly freshness: SourceFreshnessRegistry }) {
  let documentSweep = { verified: 0, retired: 0 };
  return {
    name: "semio-transform-freshness",
    apply: "serve" as const,
    enforce: "pre" as const,
    transform(_code: string, id: string) {
      const [file] = id.split("?");
      if (isAbsolute(file)) options.freshness.record(file);
      return null;
    },
    configureServer(server: FreshnessServer) {
      const root = server.config?.root ?? "";
      server.middlewares?.use((request, _response, next) => {
        const url = request.url ?? "";
        const accept = request.headers.accept;
        if (typeof accept === "string" && accept.includes("text/html")) {
          const stale = options.freshness.movedEverywhere();
          for (const file of stale) retireStaleModule(server, file);
          documentSweep = { verified: options.freshness.trackedCount(), retired: stale.length };
          next();
          return;
        }
        const file = requestedTransformFile(url, root);
        if (file !== null && options.freshness.movedFile(file)) retireStaleModule(server, file);
        next();
      });
    },
    transformIndexHtml: {
      order: "post" as const,
      handler(_html: string, context: { server?: { config: { server: { hmr: unknown } } } }) {
        const hmr = context.server?.config.server.hmr === false ? "off" : "on";
        const banner = `semio dev · transform freshness: stat-guard (every module request + whole graph per document) · hmr ${hmr} · ${documentSweep.verified} modules verified, ${documentSweep.retired} stale transforms retired · serve pid ${process.pid} · document ${new Date().toISOString()}`;
        return [{ tag: "script", attrs: { type: "module" }, children: `console.info(${JSON.stringify(banner)});` }];
      },
    },
  };
}

/** @emoji 🛰️ The dev server's complete "never serve a stale module" contract: the source watcher that
 * pushes edits into Vite's module graph, and the request-time stat guard that verifies what the watcher
 * delivered. They share one {@link createSourceFreshnessRegistry}, so the watcher can resolve a
 * temporary-file event into the module it was renamed onto. Mount both or neither. */
export function semioSourceFreshnessVitePlugins(options: { readonly repoRoot: string }) {
  const freshness = createSourceFreshnessRegistry();
  return [semioTransformFreshnessVitePlugin({ freshness }), semioSourceWatchVitePlugin({ repoRoot: options.repoRoot, freshness })];
}
//#endregion SourceFreshnessVitePlugins

//#region 🛰️AgentBridgeRendezvous
/** @emoji 🛰️ Where a live os session and a `semio-os-mcp` stdio gateway find each other — the exact
 * layout the gateway's own `🌉️mcp/🛰️rendezvous` facet owns (`~/.semio/agent/bridge`). */
export const AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION = 1;
export const AGENT_BRIDGE_OFFER_ENDPOINT_PATH = "/__semio/agent-bridge";

/** @emoji 🏷️ The carrier that points this dev session and one `semio-os-mcp` gateway at a rendezvous
 * of their own instead of the per-user default — the exact twin of the gateway's own
 * `🛰️rendezvous::RENDEZVOUS_DIR_ENV`. It is a directory path, never a credential (the admission proof
 * stays in the owner-only offer file the supervisor reads), and it is spelled with the `S_` prefix
 * the gateway's process-entry seal admits. Without it, `newestLiveAgentBridgeOffer` hands a shell
 * whichever gateway published last, so two agents running at once cross-wire. */
export const AGENT_BRIDGE_RENDEZVOUS_DIR_ENV = "S_AGENT_BRIDGE_DIR";

function agentBridgeRendezvousDir(): string {
  const pinned = process.env[AGENT_BRIDGE_RENDEZVOUS_DIR_ENV];
  if (pinned) return pinned;
  const home = process.env.HOME ?? process.env.USERPROFILE ?? ".";
  return join(home, ".semio", "agent", "bridge");
}

/** @emoji 📨️ The newest gateway offer whose publishing process is still alive — what the browser
 * shell dials. `null` means no MCP gateway is currently offering a bridge, which is an ordinary
 * state (nobody launched one), never an error. */
export function newestLiveAgentBridgeOffer(root: string = agentBridgeRendezvousDir()): { readonly url: string; readonly admissionProof: string; readonly principal: string; readonly pid: number } | null {
  const directory = join(root, "offers");
  if (!existsSync(directory)) return null;
  const offers = readdirSync(directory)
    .filter((name) => name.endsWith(".json"))
    .flatMap((name) => {
      const path = join(directory, name);
      try {
        const offer = JSON.parse(readFileSync(path, "utf8")) as { schemaVersion?: number; url?: string; admissionProof?: string; principal?: string; pid?: number; publishedAtMs?: number };
        if (offer.schemaVersion !== AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION || typeof offer.url !== "string" || typeof offer.admissionProof !== "string" || typeof offer.pid !== "number") return [];
        try {
          process.kill(offer.pid, 0);
        } catch {
          return [];
        }
        return [{ url: offer.url, admissionProof: offer.admissionProof, principal: offer.principal ?? "agent:local", pid: offer.pid, publishedAtMs: offer.publishedAtMs ?? 0 }];
      } catch {
        return [];
      }
    });
  offers.sort((left, right) => right.publishedAtMs - left.publishedAtMs);
  const newest = offers[0];
  return newest ? { url: newest.url, admissionProof: newest.admissionProof, principal: newest.principal, pid: newest.pid } : null;
}

type RendezvousServerRequest = { url?: string; method?: string };
type RendezvousServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; end: (body?: string) => void };

/** @emoji 🛰️ Publishes THIS dev session as a live os session the stdio MCP gateway can discover, and
 * serves the gateway's own offer back to the browser shell on
 * {@link AGENT_BRIDGE_OFFER_ENDPOINT_PATH}. The admission proof never travels through an environment
 * variable or a build-time define: the dev server reads the owner-only offer file and hands it over
 * loopback, on request, exactly like the local supervisor it is.
 *
 * Both halves are removed when the dev server closes, so a gateway that starts later never believes a
 * dead session. */
export function semioAgentBridgeRendezvousVitePlugin(options: { readonly rendezvousRoot?: string; readonly sessionId?: string } = {}) {
  const root = options.rendezvousRoot ?? agentBridgeRendezvousDir();
  const sessionsDir = join(root, "sessions");
  const recordPath = join(sessionsDir, `${process.pid}.json`);
  const removeRecord = (): void => {
    try {
      if (existsSync(recordPath)) rmSync(recordPath, { force: true });
    } catch {
      // best effort — a stale record is swept by the gateway's own liveness probe
    }
  };
  return {
    name: "semio-agent-bridge-rendezvous",
    apply: "serve" as const,
    configureServer(server: { middlewares: { use: (handler: (req: RendezvousServerRequest, res: RendezvousServerResponse, next: () => void) => void) => void }; httpServer?: { once: (event: string, handler: () => void) => void } | null }) {
      mkdirSync(sessionsDir, { recursive: true });
      writeFileSync(
        recordPath,
        JSON.stringify({ schemaVersion: AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION, sessionId: options.sessionId ?? `dev-${process.pid}`, pid: process.pid, shellKind: "react", startedAtMs: Date.now() }, null, 2),
        { mode: 0o600 },
      );
      process.once("exit", removeRecord);
      server.httpServer?.once("close", removeRecord);
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(AGENT_BRIDGE_OFFER_ENDPOINT_PATH)) return next();
        const offer = newestLiveAgentBridgeOffer(root);
        res.statusCode = offer ? 200 : 404;
        res.setHeader("content-type", "application/json");
        res.setHeader("cache-control", "no-store");
        res.end(JSON.stringify(offer ?? { error: "no live semio-os-mcp gateway is offering a bridge" }));
      });
    },
    closeBundle: removeRecord,
  };
}
//#endregion 🛰️AgentBridgeRendezvous

//#region 🔌️AgentCredentialInstall
/** @emoji 🗝️ Where this development host installs agent credentials for MCP clients. A directory path,
 * never a credential, spelled with the `S_` prefix the gateway's process-entry seal admits; unset, the
 * per-user default `~/.semio/agent/credentials` beside the bridge rendezvous. */
export const AGENT_CREDENTIALS_DIR_ENV = "S_AGENT_CREDENTIALS_DIR";

function agentCredentialsDir(): string {
  const pinned = process.env[AGENT_CREDENTIALS_DIR_ENV];
  if (pinned) return pinned;
  const home = process.env.HOME ?? process.env.USERPROFILE ?? ".";
  return join(home, ".semio", "agent", "credentials");
}

type CredentialInstallRequest = { url?: string; method?: string; headers: Record<string, string | string[] | undefined>; on(event: "data", handler: (chunk: Buffer) => void): void; on(event: "end" | "error", handler: () => void): void };

/** @emoji 🔐️ The credential file bytes `semio-os-mcp --credential-file` decodes, checked key for key
 * before anything reaches the disk. */
function agentCredentialFileIsWellFormed(contents: string): boolean {
  try {
    const value = JSON.parse(contents) as Record<string, unknown>;
    return (
      Object.keys(value).sort().join(",") === "audience,hubOrigin,schema,spaceId,token" &&
      value.schema === AGENT_CREDENTIAL_SCHEMA_V1 &&
      typeof value.hubOrigin === "string" &&
      typeof value.spaceId === "string" &&
      (value.audience === "read" || value.audience === "edit") &&
      typeof value.token === "string" &&
      isAgentDelegationTokenV1(value.token)
    );
  } catch {
    return false;
  }
}

/** @emoji 🔌️ Turns an agent delegation into a working MCP client on a development host: the delegation
 * pane posts the one-time credential here, this server writes it owner-only (`0600` in a `0700`
 * directory) under the delegation's own file name, and answers with its absolute path and the launcher
 * that starts `semio-os-mcp` from this checkout (`bun <repo>/📜️script.ts dev mcp stdio os`, which
 * stages the binary itself). `DELETE <endpoint>/<delegationId>` removes it once the delegation is
 * withdrawn. Only same-origin requests are served, so no other site can plant or remove a credential.
 * Contract: `📇️directory/🧬️schema` `AgentCredentialInstallRequestV1` / `AgentCredentialInstallReceiptV1`. */
export function semioAgentCredentialInstallVitePlugin(options: { readonly repoRoot: string; readonly credentialsRoot?: string }) {
  const root = options.credentialsRoot ?? agentCredentialsDir();
  const launcher = { command: process.execPath, args: [join(options.repoRoot, "📜️script.ts"), "dev", "mcp", "stdio", "os"] };
  return {
    name: "semio-agent-credential-install",
    apply: "serve" as const,
    configureServer(server: { middlewares: { use: (handler: (req: CredentialInstallRequest, res: RendezvousServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        const path = req.url?.split("?")[0] ?? "";
        if (path !== AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1 && !path.startsWith(`${AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1}/`)) return next();
        const answer = (status: number, body?: unknown): void => {
          res.statusCode = status;
          res.setHeader("cache-control", "no-store");
          if (body === undefined) return res.end();
          res.setHeader("content-type", "application/json");
          res.end(JSON.stringify(body));
        };
        const site = req.headers["sec-fetch-site"];
        const origin = req.headers.origin;
        const host = req.headers.host;
        if (site !== "same-origin" && !(typeof origin === "string" && typeof host === "string" && (origin === `http://${host}` || origin === `https://${host}`))) return answer(403, { error: "same-origin requests only" });
        if (req.method === "DELETE" && path.length > AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1.length + 1) {
          try {
            rmSync(join(root, agentCredentialInstallFileNameV1(decodeURIComponent(path.slice(AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1.length + 1)))), { force: true });
            return answer(204);
          } catch {
            return answer(400, { error: "invalid delegation id" });
          }
        }
        if (req.method !== "POST" || path !== AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1) return answer(405, { error: "POST a credential or DELETE /<delegationId>" });
        const chunks: Buffer[] = [];
        let bytes = 0;
        let overflow = false;
        req.on("data", (chunk) => {
          bytes += chunk.length;
          if (bytes > 32 * 1024) overflow = true;
          else chunks.push(chunk);
        });
        req.on("error", () => answer(400, { error: "unreadable request" }));
        req.on("end", () => {
          if (overflow) return answer(413, { error: "credential install request too large" });
          try {
            const request = JSON.parse(Buffer.concat(chunks).toString("utf8")) as Record<string, unknown>;
            if (Object.keys(request).sort().join(",") !== "contents,delegationId,schema" || request.schema !== AGENT_CREDENTIAL_INSTALL_SCHEMA_V1 || typeof request.delegationId !== "string" || typeof request.contents !== "string" || !agentCredentialFileIsWellFormed(request.contents)) {
              return answer(400, { error: "not an AgentCredentialInstallRequestV1" });
            }
            const file = join(root, agentCredentialInstallFileNameV1(request.delegationId));
            mkdirSync(root, { recursive: true, mode: 0o700 });
            chmodSync(root, 0o700);
            writeFileSync(file, request.contents, { mode: 0o600 });
            chmodSync(file, 0o600);
            return answer(201, { schema: AGENT_CREDENTIAL_INSTALL_RECEIPT_SCHEMA_V1, credentialPath: file, launcher });
          } catch {
            return answer(400, { error: "not an AgentCredentialInstallRequestV1" });
          }
        });
      });
    },
  };
}
//#endregion 🔌️AgentCredentialInstall

/** @emoji 🎫️ Serves the shell a FRESH development session from the local hub owner's broker on every request, for the
 * profile this serve signs in as — so a shell whose 15-minute local session lapsed, or a second user's serve, claims one
 * with no manual sign-in. 404 when the serve joined a hub without a broker (the shell's own sign-in stays available).
 * @see ../🚀️local-hub/🏃️execution/🟦️.ts */
export function semioLocalHubSessionVitePlugin() {
  return {
    name: "semio-local-hub-session",
    configureServer(server: { middlewares: { use: (handler: (req: { readonly url?: string; readonly method?: string }, res: { statusCode: number; setHeader: (k: string, v: string) => void; end: (body?: string) => void }, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (req.method !== "GET" || req.url?.split("?")[0] !== DEV_LOCAL_HUB_SESSION_PATH) return next();
        const dataDir = process.env[DEV_LOCAL_HUB_DATA_ENV] ?? "";
        const hubUrl = process.env.S_HUB_URL ?? "";
        const profileId = process.env[DEV_LOCAL_HUB_PROFILE_ENV] ?? "";
        void (dataDir && hubUrl && profileId ? requestLocalBrokerSession(dataDir, hubUrl, profileId).catch(() => null) : Promise.resolve(null)).then((session) => {
          res.setHeader("cache-control", "no-store");
          if (session === null) {
            res.statusCode = 404;
            res.end("local-session unavailable");
            return;
          }
          res.statusCode = 200;
          res.setHeader("content-type", "application/json");
          res.end(JSON.stringify({ schema: "semio.os.dev-local-hub-session/v1", token: session.token, userId: session.userId }));
        });
      });
    },
  };
}
