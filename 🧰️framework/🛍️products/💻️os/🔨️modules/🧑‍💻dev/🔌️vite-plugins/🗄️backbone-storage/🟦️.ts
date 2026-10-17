/** 🗄️ The backbone document storage of the dev server: the per-path sqlite handle cache, the backbone payload reads and writes, and the folder change watch. */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, watch, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { backboneKindFromUri, decodeDocumentArchiveBytes } from "@semio-tech/framework-os";
import { backboneDatabaseCtorLazy, type BackboneSqliteHandle } from "../🗃️sqlite-constructor/🟦️.ts";

//#region BackboneVitePlugin
export function canonicalBootstrapMirrorError(code: "invalid" | "conflict" | "too-large"): Error {
  return Object.assign(new Error(`canonical bootstrap folder mirror ${code}`), { code });
}

/** 🗄️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): per-path `bun:sqlite` handle cache.
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

/** 🗂️ Same `.semio/documents.db` convention as `vcs::FolderSqliteStorage` so a folder-bound studio opened by the browser dev path and a
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

/** 👁️ One debounced `node:fs.watch` of a `folder://` backbone's `.semio` directory, calling `changed` once per burst — the live
 * source of the `backbone.folder` stream route (one per watched uri while its route instance lives). Mirrors `store_sync`'s
 * native `notify` watcher (200 ms debounce) so the dev-browser and native paths agree on cadence. */
const FOLDER_WATCH_DEBOUNCE_MS = 200;

export function watchFolderChanges(uri: string, changed: () => void): () => void {
  const folder = join(uri.slice("folder://".length), ".semio");
  mkdirSync(folder, { recursive: true });
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;
  const handle = watch(folder, { persistent: false }, () => {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(changed, FOLDER_WATCH_DEBOUNCE_MS);
  });
  return () => {
    if (debounceTimer) clearTimeout(debounceTimer);
    handle.close();
  };
}
//#endregion BackboneVitePlugin
