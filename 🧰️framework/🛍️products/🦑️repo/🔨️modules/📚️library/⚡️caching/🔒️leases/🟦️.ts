import { createHash } from "node:crypto";
import { lstatSync, mkdirSync, realpathSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";

export type ResourceAccess = { readonly resource: string; readonly mode: "shared" | "exclusive" };
export type LeaseWait = ResourceAccess & { readonly elapsedMs: number };
export type LeaseOptions = ResourceAccess & {
  readonly directory: string;
  readonly signal: AbortSignal;
  readonly timeoutMs?: number;
  readonly onWait?: (progress: LeaseWait) => void;
};
export type ResourceLease = ResourceAccess & { release(): void };
type LeaseDatabase = { exec(sql: string): void; read(sql: string, ...values: string[]): Record<string, unknown> | undefined; close(): void };
const APPLICATION_ID = 0x534d4c53;

/** 🗃️ Uses each runtime's system SQLite behind the same local interface. */
async function openDatabase(path: string): Promise<LeaseDatabase> {
  if (process.versions.bun) {
    const { Database } = await import("bun:sqlite"), database = new Database(path, { create: true });
    return { exec: sql => database.exec(sql), read: (sql, ...values) => database.query(sql).get(...values) as Record<string, unknown> | undefined, close: () => database.close() };
  }
  const { DatabaseSync } = await import("node:sqlite"), database = new DatabaseSync(path);
  return { exec: sql => database.exec(sql), read: (sql, ...values) => database.prepare(sql).get(...values), close: () => database.close() };
}

/** 🛣️ Keeps lock identities on regular paths without following a substituted store. */
function databasePath(directory: string, resource: string): string {
  for (let current = resolve(directory); ; current = dirname(current)) {
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid lease store directory: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(directory, { recursive: true });
  const path = join(realpathSync(directory), createHash("sha256").update(resource).digest("hex") + ".sqlite");
  try { const stat = lstatSync(path); if (!stat.isFile() || stat.isSymbolicLink() || stat.nlink !== 1) throw new Error(`Invalid resource lease database: ${path}`); }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
  return path;
}

/** 🪪️ Initializes one immutable resource identity under SQLite's exclusive transaction. */
function initialize(database: LeaseDatabase, resource: string): void {
  const identity = database.read("PRAGMA application_id")?.application_id;
  if (identity === APPLICATION_ID) return;
  if (identity !== 0) throw new Error("Foreign resource lease database");
  database.exec("BEGIN EXCLUSIVE");
  try {
    const current = database.read("PRAGMA application_id")?.application_id;
    if (current !== APPLICATION_ID) {
      if (current !== 0) throw new Error("Foreign resource lease database");
      if (database.read("SELECT count(*) AS count FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%'")?.count !== 0) throw new Error("Unowned resource lease database");
      database.exec(`CREATE TABLE semio_resource_lease (id INTEGER PRIMARY KEY CHECK (id = 1), version INTEGER NOT NULL CHECK (version = 1), resource TEXT NOT NULL); PRAGMA application_id = ${APPLICATION_ID}`);
      database.read("INSERT INTO semio_resource_lease VALUES (1, 1, ?) RETURNING resource", resource);
    }
    database.exec("COMMIT");
  } catch (error) { database.exec("ROLLBACK"); throw error; }
}

/** 🔒️ Waits cancellably for shared or exclusive access; release only after the protected operation stops. */
export async function acquireResourceLease(options: LeaseOptions): Promise<ResourceLease> {
  options.signal.throwIfAborted();
  if (typeof options.resource !== "string" || !options.resource.length || options.resource.length > 4096 || options.resource.includes("\0")) throw new Error("Invalid lease resource");
  if (!["shared", "exclusive"].includes(options.mode)) throw new Error("Invalid resource access mode");
  if (options.timeoutMs !== undefined && (!Number.isFinite(options.timeoutMs) || options.timeoutMs < 0)) throw new Error("Invalid resource lease timeout");
  const database = await openDatabase(databasePath(options.directory, options.resource)), started = Date.now();
  let nextProgress = 0, acquired = false;
  try {
    database.exec("PRAGMA busy_timeout = 0; PRAGMA trusted_schema = OFF");
    for (;;) {
      options.signal.throwIfAborted();
      try {
        if (database.read("PRAGMA journal_mode")?.journal_mode !== "delete") throw new Error("Resource lease requires rollback journal mode");
        initialize(database, options.resource);
        database.exec(options.mode === "exclusive" ? "BEGIN EXCLUSIVE" : "BEGIN");
        const row = database.read("SELECT version, resource FROM semio_resource_lease WHERE id = 1");
        if (row?.version !== 1 || row.resource !== options.resource || database.read("PRAGMA application_id")?.application_id !== APPLICATION_ID) throw new Error("Resource lease identity mismatch");
        options.signal.throwIfAborted();
        acquired = true;
        let released = false;
        return { resource: options.resource, mode: options.mode, release() {
          if (released) return;
          released = true;
          try { database.exec("ROLLBACK"); } finally { database.close(); }
        } };
      } catch (error) {
        try { database.exec("ROLLBACK"); } catch {}
        const code = error as { code?: string; errcode?: number; errno?: number };
        if (code.code !== "SQLITE_BUSY" && code.errcode !== 5 && code.errno !== 5) throw error;
      }
      const elapsedMs = Date.now() - started;
      if (elapsedMs >= (options.timeoutMs ?? Infinity)) throw new Error(`Resource lease timed out: ${options.resource}`);
      if (elapsedMs >= nextProgress) {
        const progress = { resource: options.resource, mode: options.mode, elapsedMs };
        if (options.onWait) options.onWait(progress);
        else console.log(`Waiting for ${options.mode} access to ${options.resource}`);
        nextProgress = elapsedMs + 1000;
      }
      await delay(Math.min(40, (options.timeoutMs ?? Infinity) - elapsedMs), undefined, { signal: options.signal });
    }
  } finally { if (!acquired) database.close(); }
}

/** 🪢️ Orders a resource set consistently and releases it in reverse order on every exit. */
export async function withResourceLeases<T>(options: Omit<LeaseOptions, "resource" | "mode"> & { readonly resources: readonly ResourceAccess[] }, operation: () => Promise<T>): Promise<T> {
  const resources = new Map<string, ResourceAccess["mode"]>(), leases: ResourceLease[] = [];
  for (const entry of options.resources) resources.set(entry.resource, resources.get(entry.resource) === "exclusive" ? "exclusive" : entry.mode);
  try {
    for (const resource of [...resources.keys()].sort()) leases.push(await acquireResourceLease({ ...options, resource, mode: resources.get(resource)! }));
    options.signal.throwIfAborted();
    return await operation();
  } finally { for (const lease of leases.reverse()) lease.release(); }
}
