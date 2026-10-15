import { createHash } from "node:crypto";
import { lstatSync, mkdirSync, readdirSync, readFileSync, realpathSync, rmSync, statfsSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import queueSchema from "./🧬️schema/📜️ticket.json" with { type: "json" };
import custodySchema from "./🧬️schema/🎟️custody.json" with {type:"json"};
import retentionSchema from "./🧬️schema/🎟️retention.json" with {type:"json"};
import boundedSchema from "./📦️bounded/🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../🧬️schema/✅️validator/🟦️.ts";

export type ResourceAccess = { readonly resource: string; readonly mode: "shared" | "exclusive" };
export type LeaseWait = ResourceAccess & { readonly elapsedMs: number };
export type LeaseOptions = ResourceAccess & {
  readonly directory: string;
  readonly signal: AbortSignal;
  readonly timeoutMs?: number;
  readonly onWait?: (progress: LeaseWait) => void | Promise<void>;
  readonly control?: ResourceLeaseControl;
  readonly maximumDatabaseBytes?: number;
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
const originalLeases=new WeakMap<ResourceLease,{path:string;signal:AbortSignal;live:boolean;mode:ResourceAccess["mode"]}>();
/** 🪪️ Requires the exact live system transaction issued for this original catalog writer. */
export function requireResourceLeaseCustodyV1(lease:ResourceLease,directory:string,resource:string,control:ResourceLeaseControl):void{checkResourceLeaseControl(control);const issued=originalLeases.get(lease);if(!issued?.live||issued.path!==resourceLeasePathV1(directory,resource)||issued.signal!==control.signal||issued.mode!=="exclusive"||lease.mode!==issued.mode||lease.resource!==resource)throw Error("Original live catalog writer custody required");}

type LeaseDatabase = { exec(sql: string): void; read(sql: string, ...values: string[]): Record<string, unknown> | undefined; close(): void };
const APPLICATION_ID = 0x534d4c53;

/** 🗜️ Gives system SQLite a closed page limit before its first bootstrap write. */
export function resourceLeaseMaximumPagesV1(maximumDatabaseBytes:number):number{if(validateJsonSchemaSubset(boundedSchema,{version:1,maximumDatabaseBytes}).length||maximumDatabaseBytes%4096!==0)throw Error("Closed bootstrap SQLite capacity required");return maximumDatabaseBytes/4096;}

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
  } catch (error) { try{database.exec("ROLLBACK");}catch{}throw error; }
}

/** 📖️ Requires the privately issued shared mode through the exact live original reader. */
export function requireResourceLeaseReadCustodyV1(lease:ResourceLease,directory:string,resource:string,control:ResourceLeaseControl):void{checkResourceLeaseControl(control);const issued=originalLeases.get(lease);if(!issued?.live||issued.mode!=="shared"||issued.path!==resourceLeasePathV1(directory,resource)||issued.signal!==control.signal||lease.mode!==issued.mode||lease.resource!==resource)throw Error("Original live resource reader custody required");}

/** 🔒️ Waits cancellably for shared or exclusive access; release only after the protected operation stops. */
export async function acquireResourceLease(options: LeaseOptions): Promise<ResourceLease> {
  options.signal.throwIfAborted();
  if (typeof options.resource !== "string" || !options.resource.length || options.resource.length > 4096 || options.resource.includes("\0")) throw new Error("Invalid lease resource");
  if (!["shared", "exclusive"].includes(options.mode)) throw new Error("Invalid resource access mode");
  if (options.timeoutMs !== undefined && (!Number.isFinite(options.timeoutMs) || options.timeoutMs < 0)) throw new Error("Invalid resource lease timeout");
  const started = Date.now(); remaining(options, started);
  const maximumPages=options.maximumDatabaseBytes===undefined?undefined:resourceLeaseMaximumPagesV1(options.maximumDatabaseBytes),path=databasePath(options.directory,options.resource);
  if(maximumPages!==undefined){try{if(lstatSync(path).size>options.maximumDatabaseBytes!)throw Error("Original bootstrap SQLite exceeds capacity");}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}
  const database = await openDatabase(path);
  let nextProgress = 0, acquired = false;
  try {
    database.exec("PRAGMA busy_timeout = 0; PRAGMA trusted_schema = OFF");
    for (;;) {
      const budget = remaining(options, started);
      if (options.timeoutMs !== 0 && budget <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      try {
    if(maximumPages!==undefined){database.exec("PRAGMA page_size=4096");if(database.read("PRAGMA page_size")?.page_size!==4096||Number(database.read(`PRAGMA max_page_count=${maximumPages}`)?.max_page_count)!==maximumPages)throw Error("Actual bootstrap SQLite page cap refused");}
        if (database.read("PRAGMA journal_mode")?.journal_mode !== "delete") throw new Error("Resource lease requires rollback journal mode");
        initialize(database, options.resource);
        database.exec(options.mode === "exclusive" ? "BEGIN EXCLUSIVE" : "BEGIN");
        const row = database.read("SELECT version, resource FROM semio_resource_lease WHERE id = 1");
        if (row?.version !== 1 || row.resource !== options.resource || database.read("PRAGMA application_id")?.application_id !== APPLICATION_ID) throw new Error("Resource lease identity mismatch");
        if (remaining(options, started) <= 0 && options.timeoutMs !== 0) throw new Error(`Resource lease timed out: ${options.resource}`);
        acquired = true;
        let released = false;
        const issued={path,signal:options.signal,live:true,mode:options.mode};const lease:ResourceLease={ resource: options.resource, mode: options.mode, release() {
          if (released) return;
          released = true;issued.live=false;
          try { database.exec("ROLLBACK"); } finally { database.close(); }
        } };originalLeases.set(lease,issued);return lease;
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

function arrival(name:string):number{const field=name.split("-")[0]!;return Number.parseInt(field,queueSchema.definitions.Protocol.const.arrivalBase);}

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

/** 🎟️ Names the independently held original queue custody before publishing any arrival ticket. */
export function resourceQueueClaimV1(directory:string,resource:string,ticket:string):Readonly<{directory:string;resource:string}>{const claimDirectory=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const)),claimResource=custodySchema.properties.resourcePrefix.const+createHash("sha256").update(resource).digest("hex")+":"+ticket;resourceLeasePathV1(claimDirectory,claimResource);return Object.freeze({directory:claimDirectory,resource:claimResource});}

/** 🧭️ Derives one canonical claim catalog per data namespace for every opener and retirement. */
export function resourceQueueCatalogPathV1(directory:string):string{return resourceLeasePathV1(directory,retentionSchema.properties.resource.const);}

async function queueClaimCatalog(options:LeaseOptions,mode:"shared"|"exclusive"):Promise<ResourceLease>{return acquireResourceLease({directory:options.directory,resource:retentionSchema.properties.resource.const,mode,signal:options.signal,control:options.control,onWait:options.onWait,timeoutMs:options.timeoutMs,maximumDatabaseBytes:retentionSchema.properties.maximumDatabaseBytes.const});}

/** 🔎️ Reads exact owned claim identity without issuing retirement or inventing storage custody. */
export async function readResourceQueueClaimIdentityV1(directory:string,name:string,control:ResourceLeaseControl):Promise<Readonly<{path:string;resource:string;queueTicketPath:string;device:number;inode:number;allocatedBytes:number}>|undefined>{
 checkResourceLeaseControl(control);if(!/^[a-f0-9]{64}\.sqlite$/u.test(name))throw Error("Exact queue claim filename required");const claims=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const)),path=requireMaterializedPathV1(join(claims,name));let before;try{before=lstatSync(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1||before.size>custodySchema.properties.maximumDatabaseBytes.const)throw Error("Original regular bounded queue claim required");
 const database=await openDatabase(path);try{if(database.read("PRAGMA application_id")?.application_id!==APPLICATION_ID)return undefined;const row=database.read("SELECT version,resource FROM semio_resource_lease WHERE id=1");if(row?.version!==1||typeof row.resource!=="string")throw Error("Original queue claim identity required");const resource=row.resource,identity=new RegExp("^"+custodySchema.properties.resourcePrefix.const+"([a-f0-9]{64}):([a-f0-9]{12}-[0-9]{10}-[A-Za-z0-9._-]{1,64})$","u").exec(resource);if(!identity||resourceLeasePathV1(claims,resource)!==path)throw Error("Original queue claim path identity required");return Object.freeze({path,resource,queueTicketPath:requireMaterializedPathV1(join(directory,identity[1]+".queue",identity[2]!)),device:before.dev,inode:before.ino,allocatedBytes:before.blocks*512});}finally{database.close();}
}

/** ♻️ Closes settled original claim handles before unlinking while canonical catalog custody excludes every opener. */
export async function retireResourceQueueClaimsV1(directory:string,control:ResourceLeaseControl,timeoutMs?:number):Promise<Readonly<{examined:number;retired:number;allocatedBytes:number}>>{
 checkResourceLeaseControl(control);const started=Date.now(),options={directory,resource:retentionSchema.properties.resource.const,mode:"exclusive" as const,signal:control.signal,control,timeoutMs},catalog=await queueClaimCatalog(options,"exclusive");let examined=0,retired=0,allocatedBytes=0;
 let headroom:ReturnType<typeof import("./📦️metadata/🟦️.ts").retainResourceQueueMetadataHeadroomV1>|undefined;
 try{headroom=(await import("./📦️metadata/🟦️.ts")).retainResourceQueueMetadataHeadroomV1(directory,catalog,control);let names:string[];try{names=readdirSync(requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const)));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return {examined,retired,allocatedBytes};throw error;}
  for(const name of names.filter(name=>/^[a-f0-9]{64}\.sqlite$/u.test(name)).sort().slice(0,retentionSchema.properties.maximumClaimsPerTurn.const)){
   remaining(options,started);let held:ResourceLease|undefined;try{const row=await readResourceQueueClaimIdentityV1(directory,name,control);if(!row)continue;examined++;held=await acquireResourceLease({directory:dirname(row.path),resource:row.resource,mode:"exclusive",signal:control.signal,control,timeoutMs:0,maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});remaining(options,started);const before=lstatSync(row.path);if(before.dev!==row.device||before.ino!==row.inode)throw Error("Original queue claim inode changed");const retire=()=>{try{unlinkSync(row.queueTicketPath);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}held?.release();held=undefined;const settled=lstatSync(row.path);if(settled.dev!==row.device||settled.ino!==row.inode)throw Error("Released queue claim inode changed");unlinkSync(row.path);};try{retire();}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code!=="ENOSPC"&&!(code==="EFAULT"&&statfsSync(directory).bavail<8))throw error;headroom!.recover();retire();}retired++;allocatedBytes+=row.allocatedBytes;
   }catch(error){const code=error as {code?:string;errcode?:number;errno?:number};if(!String(error).includes("Resource lease timed out")&&code.code!=="SQLITE_BUSY"&&code.errcode!==5&&code.errno!==5)throw error;}finally{held?.release();}
   await control.yieldContinuation();checkResourceLeaseControl(control);
  }return Object.freeze({examined,retired,allocatedBytes});
 }finally{try{headroom?.recheck();}finally{catalog.release();}}
}

/** 🚦️ Serves original kernel-held tickets in arrival order; an exclusive SQLite claim alone proves abandoned custody. */
export async function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {
  options.signal.throwIfAborted();
  if("claimCatalogDirectory" in options)throw Error("Queue claim catalog selectors are forbidden");
  if(options.sharedCohort!==undefined&&(typeof options.sharedCohort!=="boolean"||options.sharedCohort&&options.mode!=="shared"))throw Error("Shared lease cohorts require shared access");
  if (!/^[A-Za-z0-9._-]{1,64}$/u.test(options.owner)) throw new Error("Invalid lease queue owner");
  const started = Date.now(); remaining(options, started);
  const pendingOptions=():LeaseOptions=>({...options,timeoutMs:options.timeoutMs===undefined?undefined:Math.max(0,remaining(options,started))});
  if(options.control)await retireResourceQueueClaimsV1(options.directory,options.control,pendingOptions().timeoutMs);
  remaining(options,started);
  const ticket = resourceQueueTicketV1({arrivedAtMs:started,pid:process.pid,owner:options.owner});
  const admitted = resourceLeasePathV1(options.directory, options.resource, ticket);
  const queue = databasePath(options.directory, options.resource).replace(/\.sqlite$/u, ".queue");
  if (queue !== admitted.replace(/\.sqlite$/u, ".queue")) throw Error("Resource queue materialized path changed");
  mkdirSync(queue, { recursive: true });
  const catalog=await queueClaimCatalog(pendingOptions(),"shared");let claim:ResourceLease;
  try{claim=await acquireResourceLease({...pendingOptions(),...resourceQueueClaimV1(options.directory,options.resource,ticket),mode:"shared",maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});try{remaining(options,started);writeFileSync(join(queue,ticket),JSON.stringify({version:1,pid:process.pid,mode:options.mode,sharedCohort:options.sharedCohort===true}),{flag:"wx"});}catch(error){claim.release();throw error;}}finally{catalog.release();}
  let nextProgress = 0;
  try {
    for (;;) {
      if (remaining(options, started) <= 0 && options.timeoutMs !== 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      const live:string[]=[];for(const name of(await retryInterruptedFilesystemRead(options.signal,()=>readdirSync(queue),options.control)).sort((left,right)=>arrival(left)-arrival(right)||left.localeCompare(right,"en"))){
       if(name===ticket){live.push(name);continue;}const sweepCatalog=await queueClaimCatalog(pendingOptions(),"shared");let retired:ResourceLease|undefined;try{retired=await acquireResourceLease({...resourceQueueClaimV1(options.directory,options.resource,name),mode:"exclusive",signal:options.signal,control:options.control,timeoutMs:0,maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});rmSync(join(queue,name),{force:true});}catch(error){if(!String(error).includes("Resource lease timed out"))throw error;live.push(name);}finally{retired?.release();sweepCatalog.release();}
      }
      remaining(options, started);
      const cohort=options.sharedCohort===true&&await retryInterruptedFilesystemRead(options.signal,()=>live.slice(0,live.indexOf(ticket)).every(name=>{let claim:unknown;try{claim=JSON.parse(readFileSync(join(queue,name),"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return true;if(error instanceof SyntaxError)return false;throw error;}const row=claim as Record<string,unknown>|undefined;return row!==null&&typeof row==="object"&&Object.keys(row).sort().join(",")==="mode,pid,sharedCohort,version"&&row.version===1&&row.pid===Number(name.split("-")[1])&&row.mode==="shared"&&row.sharedCohort===true;}),options.control);
      remaining(options, started);
      if (live[0] === ticket || cohort) {
        const lease = await acquireResourceLease({ ...options, timeoutMs: options.timeoutMs === undefined ? undefined : Math.max(0, options.timeoutMs - (Date.now() - started)) });
        try { remaining(options, started); } catch (error) { lease.release(); throw error; }
        return { resource: lease.resource, mode: lease.mode, release() {
          try { lease.release(); } finally { try{rmSync(join(queue, ticket), { force: true });}finally{claim.release();} }
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
    try{rmSync(join(queue, ticket), { force: true });}finally{claim.release();}
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
