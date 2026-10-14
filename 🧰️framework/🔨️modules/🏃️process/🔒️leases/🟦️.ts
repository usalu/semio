import { createHash } from "node:crypto";
import { lstatSync, mkdirSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import queueSchema from "./🧬️schema/📜️ticket.json" with { type: "json" };

export type ResourceAccess = { readonly resource: string; readonly mode: "shared" | "exclusive" };
export type LeaseWait = ResourceAccess & { readonly elapsedMs: number };
export type LeaseOptions = ResourceAccess & {
  readonly directory: string;
  readonly signal: AbortSignal;
  readonly timeoutMs?: number;
  readonly onWait?: (progress: LeaseWait) => void | Promise<void>;
  readonly control?: ResourceLeaseControl;
};
/** 🎛️ Preserves an admitted caller's existing signal, deadline and awaited wait transitions. */
export interface ResourceLeaseControl {
  readonly signal: AbortSignal;
  remainingMilliseconds(): number | null;
  publish(progress: LeaseWait): void | Promise<void>;
  yieldContinuation(): Promise<void>;
}

/** ⏳️ Refuses expired caller authority without replacing its cancellation signal. */
export function checkResourceLeaseControl(control: ResourceLeaseControl): number | null {
  if (!control || !(control.signal instanceof AbortSignal) || typeof control.remainingMilliseconds !== "function" || typeof control.publish !== "function" || typeof control.yieldContinuation !== "function") throw Error("Complete resource admission control required");
  control.signal.throwIfAborted();
  const remaining = control.remainingMilliseconds();
  if (remaining !== null && (!Number.isFinite(remaining) || remaining <= 0)) throw Error("Original compiler admission deadline exhausted");
  return remaining;
}

/** 📏️ Checks the complete resolved materialized path in both portable string measures. */
export function requireMaterializedPathV1(path: string): string {
  const absolute = resolve(path);
  if (absolute.includes("\0") || absolute.length > 256 || [...absolute].length > 256) throw Error("Materialized absolute path exceeds 256 characters");
  return absolute;
}

/** 🗃️ Admits the actual database, every SQLite sidecar and optional queue ticket before any mkdir. */
export function resourceLeasePathV1(directory: string, resource: string, ticket?: string): string {
  const path = requireMaterializedPathV1(join(resolve(directory), createHash("sha256").update(resource).digest("hex") + ".sqlite"));
  for (const suffix of ["-journal", "-wal", "-shm"]) requireMaterializedPathV1(path + suffix);
  if (ticket !== undefined) requireMaterializedPathV1(join(path.replace(/\.sqlite$/u, ".queue"), ticket));
  return path;
}

function remaining(options: LeaseOptions, started: number): number {
  options.signal.throwIfAborted();
  if (options.control && options.control.signal !== options.signal) throw Error("Original lease signal identity required");
  const original = options.control ? checkResourceLeaseControl(options.control) : null;
  const requested = options.timeoutMs === undefined ? Infinity : options.timeoutMs - (Date.now() - started);
  if (options.timeoutMs !== undefined && options.timeoutMs > 0 && requested <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
  return Math.min(original ?? Infinity, requested);
}

async function waitProgress(options: LeaseOptions, progress: LeaseWait, started: number): Promise<void> {
  remaining(options, started);
  if (options.control) {
    await options.control.publish(progress); remaining(options, started);
    await options.control.yieldContinuation(); remaining(options, started);
  }
  if (options.onWait) { await options.onWait(progress); remaining(options, started); }
  else if (!options.control) console.log(`Waiting for ${options.mode} access to ${options.resource}`);
}

export type ResourceLease = ResourceAccess & { release(): void };
type LeaseDatabase = { exec(sql: string): void; read(sql: string, ...values: string[]): Record<string, unknown> | undefined; close(): void };
const APPLICATION_ID = 0x534d4c53;

/** 🗃️ Bun's own SQLite, named through a constant rather than inline. The specifier is a Bun BUILTIN:
 * a bundler asked to analyse this module refuses it outright ("Cannot bundle built-in module"), which
 * failed whole test FILES whose graph reaches this store from a browser-flavoured environment. Holding it
 * in a `const` (whose literal type still gives the import its real module type) plus `@vite-ignore` leaves
 * the resolution to the runtime, which is the only place it was ever meant to happen. */
const BUN_SQLITE_MODULE = "bun:sqlite" as const;

/** 🗃️ Uses each runtime's system SQLite behind the same local interface. */
async function openDatabase(path: string): Promise<LeaseDatabase> {
  if (process.versions.bun) {
    const { Database } = await import(/* @vite-ignore */ BUN_SQLITE_MODULE), database = new Database(path, { create: true });
    return { exec: sql => database.exec(sql), read: (sql, ...values) => database.query(sql).get(...values) as Record<string, unknown> | undefined, close: () => database.close() };
  }
  const { DatabaseSync } = await import("node:sqlite"), database = new DatabaseSync(path);
  return { exec: sql => database.exec(sql), read: (sql, ...values) => database.prepare(sql).get(...values), close: () => database.close() };
}

/** 🛣️ Keeps lock identities on regular paths without following a substituted store. */
function databasePath(directory: string, resource: string): string {
  const admitted = resourceLeasePathV1(directory, resource);
  for (let current = resolve(directory); ; current = dirname(current)) {
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid lease store directory: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(directory, { recursive: true });
  const path = resourceLeasePathV1(realpathSync(directory), resource);
  if (path !== admitted) throw Error("Resource lease materialized path changed");
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
  const started = Date.now(); remaining(options, started);
  const database = await openDatabase(databasePath(options.directory, options.resource));
  let nextProgress = 0, acquired = false;
  try {
    database.exec("PRAGMA busy_timeout = 0; PRAGMA trusted_schema = OFF");
    for (;;) {
      const budget = remaining(options, started);
      if (options.timeoutMs !== 0 && budget <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      try {
        if (database.read("PRAGMA journal_mode")?.journal_mode !== "delete") throw new Error("Resource lease requires rollback journal mode");
        initialize(database, options.resource);
        database.exec(options.mode === "exclusive" ? "BEGIN EXCLUSIVE" : "BEGIN");
        const row = database.read("SELECT version, resource FROM semio_resource_lease WHERE id = 1");
        if (row?.version !== 1 || row.resource !== options.resource || database.read("PRAGMA application_id")?.application_id !== APPLICATION_ID) throw new Error("Resource lease identity mismatch");
        if (remaining(options, started) <= 0 && options.timeoutMs !== 0) throw new Error(`Resource lease timed out: ${options.resource}`);
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
      if (remaining(options, started) <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      if (elapsedMs >= nextProgress) {
        const progress = { resource: options.resource, mode: options.mode, elapsedMs };
        await waitProgress(options, progress, started);
        nextProgress = elapsedMs + 1000;
      }
      await delay(Math.max(1, Math.min(40, remaining(options, started))), undefined, { signal: options.signal });
      remaining(options, started);
    }
  } finally { if (!acquired) database.close(); }
}

/** 🚦️ One waiter's place in a resource's arrival queue: `<directory>/<resource-hash>.queue/<arrivedAtMs>-<pid>-<owner>`. */
export type QueuedLeaseOptions = LeaseOptions & { readonly owner: string; readonly sharedCohort?: boolean };

/** 🎟️ Names one live request using the exact production ticket contract. */
export function resourceQueueTicketV1(request:{readonly arrivedAtMs:number;readonly pid:number;readonly owner:string}):string{
  if(!Number.isSafeInteger(request.arrivedAtMs)||request.arrivedAtMs<queueSchema.properties.arrivedAtMs.minimum||request.arrivedAtMs>queueSchema.properties.arrivedAtMs.maximum||!Number.isSafeInteger(request.pid)||request.pid<queueSchema.properties.pid.minimum||request.pid>queueSchema.properties.pid.maximum||!new RegExp(queueSchema.properties.owner.pattern,"u").test(request.owner))throw Error("Invalid lease queue ticket");
  const rule=queueSchema.definitions.Protocol.const;return `${request.arrivedAtMs.toString(rule.arrivalBase).padStart(rule.arrivalDigits,"0")}-${String(request.pid).padStart(rule.pidDigits,"0")}-${request.owner}`;
}

function arrival(name:string):number{const field=name.split("-")[0]!;return field.length===queueSchema.definitions.Protocol.const.arrivalDigits?Number.parseInt(field,queueSchema.definitions.Protocol.const.arrivalBase):Number(field);}

/** 🔁️ Retries an interrupted filesystem read while preserving cancellation and original permanent errors. */
export async function retryInterruptedFilesystemRead<T>(signal: AbortSignal, read: () => T, control?: ResourceLeaseControl): Promise<T> {
  for (;;) {
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    try { const result = read(); if (control) checkResourceLeaseControl(control); return result; }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EINTR") throw error;
      await delay(1, undefined, { signal }); if (control) checkResourceLeaseControl(control);
    }
  }
}

/** 🚦️ Whether the process that wrote a queue ticket still exists (EPERM means it exists under another user). */
function ticketAlive(pid: number): boolean {
  if (!Number.isSafeInteger(pid) || pid <= 0) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

/** 🚦️ Serves one resource in ARRIVAL order: every waiter files a ticket named by its arrival time, only the oldest live
 * ticket may try the lease, and a ticket whose process is gone is swept, so a crashed waiter never blocks the queue and a
 * long queue (a fleet of agents sharing one wasm build) is neither starved nor overtaken. The lease itself is
 * {@link acquireResourceLease}, so the kernel still releases a crashed holder. The ticket is removed on every exit. */
export async function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {
  options.signal.throwIfAborted();
  if(options.sharedCohort!==undefined&&(typeof options.sharedCohort!=="boolean"||options.sharedCohort&&options.mode!=="shared"))throw Error("Shared lease cohorts require shared access");
  if (!/^[A-Za-z0-9._-]{1,64}$/u.test(options.owner)) throw new Error("Invalid lease queue owner");
  const started = Date.now(); remaining(options, started);
  const ticket = resourceQueueTicketV1({arrivedAtMs:started,pid:process.pid,owner:options.owner});
  const admitted = resourceLeasePathV1(options.directory, options.resource, ticket);
  const queue = databasePath(options.directory, options.resource).replace(/\.sqlite$/u, ".queue");
  if (queue !== admitted.replace(/\.sqlite$/u, ".queue")) throw Error("Resource queue materialized path changed");
  mkdirSync(queue, { recursive: true });
  writeFileSync(join(queue, ticket), JSON.stringify({version:1,pid:process.pid,mode:options.mode,sharedCohort:options.sharedCohort===true}), { flag: "wx" });
  let nextProgress = 0;
  try {
    for (;;) {
      if (remaining(options, started) <= 0 && options.timeoutMs !== 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      const live = (await retryInterruptedFilesystemRead(options.signal, () => readdirSync(queue), options.control)).sort((left,right)=>arrival(left)-arrival(right)||left.localeCompare(right,"en")).filter((name) => {
        const pid = Number(name.split("-")[1]);
        if (name === ticket || ticketAlive(pid)) return true;
        rmSync(join(queue, name), { force: true });
        return false;
      });
      remaining(options, started);
      const cohort=options.sharedCohort===true&&await retryInterruptedFilesystemRead(options.signal,()=>live.slice(0,live.indexOf(ticket)).every(name=>{let claim:unknown;try{claim=JSON.parse(readFileSync(join(queue,name),"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return true;if(error instanceof SyntaxError)return false;throw error;}const row=claim as Record<string,unknown>|undefined;return row!==null&&typeof row==="object"&&Object.keys(row).sort().join(",")==="mode,pid,sharedCohort,version"&&row.version===1&&row.pid===Number(name.split("-")[1])&&row.mode==="shared"&&row.sharedCohort===true;}),options.control);
      remaining(options, started);
      if (live[0] === ticket || cohort) {
        const lease = await acquireResourceLease({ ...options, timeoutMs: options.timeoutMs === undefined ? undefined : Math.max(0, options.timeoutMs - (Date.now() - started)) });
        try { remaining(options, started); } catch (error) { lease.release(); throw error; }
        return { resource: lease.resource, mode: lease.mode, release() {
          try { lease.release(); } finally { rmSync(join(queue, ticket), { force: true }); }
        } };
      }
      const elapsedMs = Date.now() - started;
      if (remaining(options, started) <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      if (elapsedMs >= nextProgress) {
        const progress = { resource: options.resource, mode: options.mode, elapsedMs };
        await waitProgress(options, progress, started);
        nextProgress = elapsedMs + 15_000;
      }
      await delay(Math.max(1, Math.min(250, remaining(options, started))), undefined, { signal: options.signal });
      remaining(options, started);
    }
  } catch (error) {
    rmSync(join(queue, ticket), { force: true });
    throw error;
  }
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
