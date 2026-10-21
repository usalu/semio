import{initializeResourceLeaseContinuationV1,requireResourceLeaseContinuationCatalogV1,readResourceLeaseContinuationV1,writeResourceLeaseContinuationV1,resourceLeaseContinuationStartV1,type ResourceLeaseContinuationKindV1,type ResourceLeaseTransactionPhaseV1}from"./♻️retirement/🧭️continuation/🟦️.ts";
import cleanupLeaseSchema from"./🧹️cleanup/🧬️schema/🔣️.json" with{type:"json"};
import metadataSchema from "./📦️metadata/🧬️schema/🔣️.json" with {type:"json"};
import {captureResourceLeaseCandidateV1,requireResourceLeaseCandidateV1,readOwnedResourceLeaseV1,resourceLeaseBusyV1,resourceLeaseRetirementNamesV1,resourceLeaseRetirementScanLimitsV1,finishResourceLeaseCleanupV1,ResourceLeaseAdmissionFinalizerV1,type ResourceLeaseDatabaseV1,type ResourceLeaseCandidateV1} from "./♻️retirement/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { AsyncLocalStorage } from "node:async_hooks";
import {beginLeaseOrderV1,classifyLeaseResourceV1,forgetLeaseV1,currentLeaseLineageV1,heldLeasesV1,holdsSharedInProcessV1,leaseOrderPolicyV1,leaseTaskIdV1,type LeaseOrderGuardV1} from "./🧭️order/🟦️.ts";
export {LeaseOrderErrorV1,formatLeaseViolationV1,leaseInheritanceEnvironmentV1,withLeaseTaskV1} from "./🧭️order/🟦️.ts";
import {readIssuedResourceControlCapabilitiesV1,requireIssuedResourceControlV1,requireAcceptedPhysicalCleanupDescendantV1,requireAcceptedPhysicalCleanupControlV1,checkAcceptedPhysicalCleanupV1,readAcceptedPhysicalCleanupV1,requireIssuedResourceControlRefinementV1,withAcceptedPhysicalCleanupOperationV1,closeAcceptedPhysicalCleanupChildrenV1} from "../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type {AcceptedPhysicalCleanupV1} from "../🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🟦️.ts";
import { appendFileSync, lstatSync, statSync, mkdirSync, readdirSync, readFileSync, realpathSync, renameSync, rmSync, statfsSync, unlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import {checkResourceLeaseControl,isSystemLeaseLinkV1,requireMaterializedPathV1,type LeaseWait,type ResourceAccess,type ResourceLeaseControl} from "./🎛️control/🟦️.ts";
export {checkResourceLeaseControl,requireMaterializedPathV1,type LeaseWait,type ResourceAccess,type ResourceLeaseControl} from "./🎛️control/🟦️.ts";
import { setTimeout as delay } from "node:timers/promises";
import queueSchema from "./🧬️schema/📜️ticket.json" with { type: "json" };
import waitPolicySchema from "./⏳️wait/🧬️schema/📜️policy.json" with { type: "json" };
import boundedSchema from "./📦️bounded/🧬️schema/🔣️.json" with {type:"json"};
import {compileJsonSchemaSubset} from "../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const retirementSchema=queueSchema.definitions.RetirementPolicy,custodySchema=queueSchema.definitions.ClaimCustodyPolicy,retentionSchema=queueSchema.definitions.ClaimRetentionPolicy,boundedContract=compileJsonSchemaSubset(boundedSchema);

export type LeaseOptions = ResourceAccess & {
  readonly directory: string;
  readonly signal: AbortSignal;
  readonly timeoutMs?: number;
  readonly onWait?: (progress: LeaseWait) => void | Promise<void>;
  readonly control?: ResourceLeaseControl;
  readonly maximumDatabaseBytes?: number;
};
/** 🗃️ Admits the actual database, every SQLite sidecar and optional queue ticket before any mkdir. */
export function resourceLeasePathV1(directory: string, resource: string, ticket?: string): string {
  const path = requireMaterializedPathV1(join(resolve(directory), createHash("sha256").update(resource).digest("hex") + ".sqlite"));
  for (const suffix of ["-journal", "-wal", "-shm"]) requireMaterializedPathV1(path + suffix);
  if (ticket !== undefined) requireMaterializedPathV1(join(path.replace(/\.sqlite$/u, ".queue"), ticket));
  return path;
}

function remaining(options: LeaseOptions, started: number,cleanup?:CleanupLeaseScope): number {
  if(cleanup){const available=checkAcceptedPhysicalCleanupV1(cleanup.enrollment);if(options.control||options.signal!==readAcceptedPhysicalCleanupV1(cleanup.enrollment).cleanupSignal||options.directory!==cleanup.directory||options.resource!==cleanup.resource&&options.resource!==admissionResource)throw Error("Exact accepted cleanup lease request required");return Math.min(available,options.timeoutMs===undefined?Infinity:options.timeoutMs-(Date.now()-started));}
  options.signal.throwIfAborted();
  if (options.control && options.control.signal !== options.signal) throw Error("Original lease signal identity required");
  const original = options.control ? checkResourceLeaseControl(options.control) : null;
  const requested = options.timeoutMs === undefined ? Infinity : options.timeoutMs - (Date.now() - started);
  if (options.timeoutMs !== undefined && options.timeoutMs > 0 && requested <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
  return Math.min(original ?? Infinity, requested);
}

/** 🛎️ Awaits one observer under its unchanged signal, timeout and original deadline. */
async function waitForLeaseContinuation(options:LeaseOptions,started:number,continuation:()=>void|Promise<void>):Promise<void>{
 const budget=remaining(options,started);let timer:ReturnType<typeof setTimeout>|undefined,cancel!:()=>void;
 const interrupted=new Promise<never>((_,reject)=>{
  cancel=()=>reject(options.signal.reason);options.signal.addEventListener("abort",cancel,{once:true});
  const arm=(milliseconds:number):void=>{if(Number.isFinite(milliseconds))timer=setTimeout(()=>{
   try{const left=remaining(options,started);if(left<=0)throw Error(`Resource lease timed out: ${options.resource}`);arm(left);}catch(error){reject(error);}
  },Math.max(1,Math.min(milliseconds,2147483647)));};
  arm(budget);if(options.signal.aborted)cancel();
 });
 try{await Promise.race([Promise.resolve().then(()=>{remaining(options,started);return continuation();}),interrupted]);remaining(options,started);}
 finally{options.signal.removeEventListener("abort",cancel);if(timer!==undefined)clearTimeout(timer);}
}

async function waitProgress(options: LeaseOptions, progress: LeaseWait, started: number,cleanup?:CleanupLeaseScope): Promise<void> {
  remaining(options, started,cleanup);
  const control=options.control,onWait=options.onWait;
  if (control) {
    await waitForLeaseContinuation(options,started,()=>control.publish(progress));
    await waitForLeaseContinuation(options,started,()=>control.yieldContinuation());
  }
  if (onWait) await waitForLeaseContinuation(options,started,()=>onWait(progress));
  else if (!control) console.log(`Waiting for ${options.mode} access to ${options.resource}`);
}

export type ResourceLease = ResourceAccess & { release(): void };
type IssuedResourceLeaseV1={readonly path:string;readonly directory:string;readonly resource:string;readonly database:LeaseDatabase;readonly signal:AbortSignal;readonly control:ResourceLeaseControl|undefined;readonly mode:ResourceAccess["mode"];phase:ResourceLeaseTransactionPhaseV1};
const originalLeases=new WeakMap<ResourceLease,IssuedResourceLeaseV1>();
declare const cleanupLeaseBrand:unique symbol,cleanupWriterBrand:unique symbol;
/** 🛟️ Carries only a privately retained exact preexisting kernel guard into its birth-funded cleanup. */
export type AcceptedResourceLeaseCleanupV1=Readonly<{[cleanupLeaseBrand]:true}>;
/** 🔏️ Authorizes no ordinary resource control and has no releasing port. */
export type AcceptedResourceLeaseCleanupWriterV1=Readonly<{[cleanupWriterBrand]:true}>;
type CleanupLeaseScope={enrollment:AcceptedPhysicalCleanupV1;original:IssuedResourceLeaseV1;directory:string;resource:string;path:string;dev:number;ino:number;directoryDev:number;directoryIno:number;used:boolean;active:boolean;lease?:ResourceLease};
const unresolvedCleanupWriters=new Set<CleanupLeaseScope>();
const cleanupLimits=Object.freeze({...cleanupLeaseSchema.definitions.Protocol.const}),cleanupLoans=new WeakMap<AcceptedResourceLeaseCleanupV1,CleanupLeaseScope>(),cleanupWriters=new WeakMap<AcceptedResourceLeaseCleanupWriterV1,CleanupLeaseScope>(),cleanupEnrollments=new WeakMap<AcceptedPhysicalCleanupV1,Set<CleanupLeaseScope>>();
function cleanupLeaseIdentity(scope:CleanupLeaseScope):void{const directory=lstatSync(scope.directory);if(!directory.isDirectory()||directory.isSymbolicLink()||directory.dev!==scope.directoryDev||directory.ino!==scope.directoryIno)throw Error("Original cleanup lease directory required");const row=lstatSync(scope.path);if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||row.dev!==scope.dev||row.ino!==scope.ino||row.size>cleanupLimits.maximumDatabaseBytes)throw Error("Original cleanup lease database identity required");}
function relatedCleanupControls(leaseControl:ResourceLeaseControl,bodyControl:ResourceLeaseControl):void{try{requireIssuedResourceControlRefinementV1(leaseControl,bodyControl);}catch{requireIssuedResourceControlRefinementV1(bodyControl,leaseControl);}}
/** 🧷️ Captures actual custody before cleanup; copied leases, foreign controls and late captures grant nothing. */
export function acceptResourceLeaseCleanupV1(lease:ResourceLease,directory:string,resource:string,leaseControl:ResourceLeaseControl,bodyControl:ResourceLeaseControl,enrollment:AcceptedPhysicalCleanupV1):AcceptedResourceLeaseCleanupV1{requireAcceptedPhysicalCleanupControlV1(enrollment,bodyControl);checkResourceLeaseControl(bodyControl);checkResourceLeaseControl(leaseControl);relatedCleanupControls(leaseControl,bodyControl);return captureResourceLeaseCleanupV1(lease,directory,resource,leaseControl,enrollment);}
/** 🧬️ Enrolls only a genuine live lease below this exact accepted body, without exposing or renewing its control. */
export function acceptResourceLeaseDescendantCleanupV1(lease:ResourceLease,directory:string,resource:string,leaseControl:ResourceLeaseControl,enrollment:AcceptedPhysicalCleanupV1):AcceptedResourceLeaseCleanupV1{requireIssuedResourceControlV1(leaseControl);checkResourceLeaseControl(leaseControl);requireAcceptedPhysicalCleanupDescendantV1(enrollment,leaseControl);return captureResourceLeaseCleanupV1(lease,directory,resource,leaseControl,enrollment);}
function captureResourceLeaseCleanupV1(lease:ResourceLease,directory:string,resource:string,leaseControl:ResourceLeaseControl,enrollment:AcceptedPhysicalCleanupV1):AcceptedResourceLeaseCleanupV1{const issued=originalLeases.get(lease),loans=cleanupEnrollments.get(enrollment)??new Set<CleanupLeaseScope>();if(issued?.phase!=="held"||issued.control!==leaseControl||issued.signal!==leaseControl.signal||issued.directory!==resolve(directory)||issued.resource!==resource||issued.path!==resourceLeasePathV1(directory,resource)||lease.resource!==issued.resource||lease.mode!==issued.mode||loans.size>=cleanupLimits.maximumLoans||[...loans].some(row=>row.original===issued))throw Error("Exact original lease cleanup enrollment required");const row=lstatSync(issued.path),folder=lstatSync(issued.directory),scope:CleanupLeaseScope={enrollment,original:issued,directory:issued.directory,resource:issued.resource,path:issued.path,dev:row.dev,ino:row.ino,directoryDev:folder.dev,directoryIno:folder.ino,used:false,active:false};cleanupLeaseIdentity(scope);const token=Object.freeze({})as AcceptedResourceLeaseCleanupV1;loans.add(scope);cleanupEnrollments.set(enrollment,loans);cleanupLoans.set(token,scope);return token;}

/** 🛡️ Requires the actual exclusive guard acquired solely for this exact nonrenewable cleanup loan. */
export function requireAcceptedResourceLeaseCleanupWriterV1(proof:AcceptedResourceLeaseCleanupWriterV1,loan:AcceptedResourceLeaseCleanupV1):void{const scope=cleanupLoans.get(loan),writer=cleanupWriters.get(proof),issued=scope?.lease?originalLeases.get(scope.lease):undefined;if(!scope||writer!==scope||!scope.active||scope.original.phase!=="closed"||issued?.phase!=="held"||issued.mode!=="exclusive"||issued.control!==undefined||issued.signal!==readAcceptedPhysicalCleanupV1(scope.enrollment).cleanupSignal||issued.directory!==scope.directory||issued.resource!==scope.resource||issued.path!==scope.path)throw Error("Actual accepted cleanup writer required");checkAcceptedPhysicalCleanupV1(scope.enrollment);cleanupLeaseIdentity(scope);}
/** 🪢️ Keeps an already held exact guard useful only to its original physical cleanup owner. */
export function requireAcceptedResourceLeaseRetainedCleanupV1(loan:AcceptedResourceLeaseCleanupV1,enrollment:AcceptedPhysicalCleanupV1,binding:Readonly<{directory:string;resource:string;mode:"shared"|"exclusive"}>):void{const scope=cleanupLoans.get(loan),issued=scope?.original;if(!scope||scope.enrollment!==enrollment||issued?.phase!=="held"||scope.used||scope.active||issued.directory!==resolve(binding.directory)||issued.resource!==binding.resource||issued.mode!==binding.mode)throw Error("Actual retained cleanup lease required");checkAcceptedPhysicalCleanupV1(enrollment);cleanupLeaseIdentity(scope);}
/** 🔒️ Reacquires only its released original guard, joining the exact cleanup callback before descriptor release. */
export function withAcceptedResourceLeaseCleanupWriterV1<T>(loan:AcceptedResourceLeaseCleanupV1,body:(proof:AcceptedResourceLeaseCleanupWriterV1)=>Promise<T>):Promise<T>{const scope=releasedCleanupScope(loan,body,false);return withAcceptedPhysicalCleanupOperationV1(scope.enrollment,()=>withCleanupWriter(loan,body));}
/** 🧱️ Retains actual exclusion on unresolved terminal failure without exposing a release or renewed clock. */
export function withAcceptedResourceLeaseCleanupRetirementWriterV1<T>(loan:AcceptedResourceLeaseCleanupV1,body:(proof:AcceptedResourceLeaseCleanupWriterV1)=>Promise<T>):Promise<T>{const scope=releasedCleanupScope(loan,body,true);return withAcceptedPhysicalCleanupOperationV1(scope.enrollment,()=>withCleanupWriter(loan,body,true));}
function releasedCleanupScope(loan:AcceptedResourceLeaseCleanupV1,body:unknown,retainFailure:boolean):CleanupLeaseScope{const scope=cleanupLoans.get(loan);if(!scope||scope.used||scope.original.phase!=="closed"||typeof body!=="function"||retainFailure&&unresolvedCleanupWriters.size>=cleanupLimits.maximumUnresolvedWriters)throw Error("Released original cleanup lease required");return scope;}
async function withCleanupWriter<T>(loan:AcceptedResourceLeaseCleanupV1,body:(proof:AcceptedResourceLeaseCleanupWriterV1)=>Promise<T>,retainFailure=false):Promise<T>{const scope=releasedCleanupScope(loan,body,retainFailure);const remaining=checkAcceptedPhysicalCleanupV1(scope.enrollment);cleanupLeaseIdentity(scope);scope.used=true;scope.lease=await acquireResourceLeaseBody({directory:scope.directory,resource:scope.resource,mode:"exclusive",signal:readAcceptedPhysicalCleanupV1(scope.enrollment).cleanupSignal,timeoutMs:remaining,maximumDatabaseBytes:cleanupLimits.maximumDatabaseBytes},scope);scope.active=true;const proof=Object.freeze({})as AcceptedResourceLeaseCleanupWriterV1;cleanupWriters.set(proof,scope);let failed=false,failure:unknown,value:T|undefined;try{requireAcceptedResourceLeaseCleanupWriterV1(proof,loan);value=await body(proof);requireAcceptedResourceLeaseCleanupWriterV1(proof,loan);}catch(error){failed=true;failure=error;}try{await closeAcceptedPhysicalCleanupChildrenV1(scope.enrollment);}catch(error){if(!failed){failed=true;failure=error;}}scope.active=false;if(failed&&retainFailure){unresolvedCleanupWriters.add(scope);throw failure;}try{scope.lease!.release();}catch(error){if(failed&&!Object.is(failure,error))throw new AggregateError([failure,error],"Accepted cleanup work and exact guard release failed");throw error;}if(failed)throw failure;return value as T;}
declare const writerProofBrand:unique symbol;
/** 🎟️ Carries no release port and resolves only to its privately retained exclusive transaction. */
export type ResourceLeaseWriterProofV1=Readonly<{readonly [writerProofBrand]:true}>;
const writerProofs=new WeakMap<ResourceLeaseWriterProofV1,{readonly lease:ResourceLease;readonly issued:IssuedResourceLeaseV1}>();
/** 🔏️ Derives a nonreleasing proof only from this exact original writer and accepted control. */
export function captureResourceLeaseWriterProofV1(lease:ResourceLease,directory:string,resource:string,control:ResourceLeaseControl):ResourceLeaseWriterProofV1{
 requireResourceLeaseCustodyV1(lease,directory,resource,control);const issued=originalLeases.get(lease)!;if(issued.control!==control||issued.directory!==resolve(directory)||issued.resource!==resource)throw Error("Exact original writer binding required");const proof=Object.freeze({})as ResourceLeaseWriterProofV1;writerProofs.set(proof,{lease,issued});return proof;
}
/** 🛡️ Requires the immutable original directory, logical key, guard path and live control. */
export function requireResourceLeaseWriterProofV1(proof:ResourceLeaseWriterProofV1,binding:Readonly<{directory:string;resource:string;path:string}>,control:ResourceLeaseControl):void{
 checkResourceLeaseControl(control);const held=writerProofs.get(proof),issued=held?.issued;if(!held||issued?.phase!=="held"||issued.mode!=="exclusive"||issued.control!==control||issued.signal!==control.signal||issued.directory!==resolve(binding.directory)||issued.resource!==binding.resource||issued.path!==binding.path||held.lease.mode!==issued.mode||held.lease.resource!==issued.resource)throw Error("Exact live original writer proof required");
}
/** 🪢️ Checks the original live lease loan under its exact accepted outer body after a nested owner has settled. */
export function requireAcceptedResourceLeaseRetainedBodyV1(loan:AcceptedResourceLeaseCleanupV1,enrollment:AcceptedPhysicalCleanupV1,binding:Readonly<{directory:string;resource:string;mode:"shared"|"exclusive"}>,bodyControl:ResourceLeaseControl,control:ResourceLeaseControl):void{requireAcceptedPhysicalCleanupControlV1(enrollment,bodyControl);requireIssuedResourceControlRefinementV1(bodyControl,control);checkResourceLeaseControl(bodyControl);checkResourceLeaseControl(control);const scope=cleanupLoans.get(loan),issued=scope?.original;if(!scope||scope.enrollment!==enrollment||issued?.phase!=="held"||scope.used||scope.active||issued.directory!==resolve(binding.directory)||issued.resource!==binding.resource||issued.mode!==binding.mode)throw Error("Actual retained outer-body lease required");cleanupLeaseIdentity(scope);}
/** 🛟️ Checks still-retained exclusion only within its exact already accepted cleanup phase. */
export function requireAcceptedPhysicalCleanupWriterProofV1(proof:ResourceLeaseWriterProofV1,binding:Readonly<{directory:string;resource:string;path:string}>,bodyControl:ResourceLeaseControl,enrollment:AcceptedPhysicalCleanupV1):void{
 requireAcceptedPhysicalCleanupDescendantV1(enrollment,bodyControl);checkAcceptedPhysicalCleanupV1(enrollment);const held=writerProofs.get(proof),issued=held?.issued,facts=readAcceptedPhysicalCleanupV1(enrollment);if(!held||issued?.phase!=="held"||issued.mode!=="exclusive"||issued.control!==bodyControl||issued.signal!==bodyControl.signal||issued.directory!==resolve(binding.directory)||issued.resource!==binding.resource||issued.path!==binding.path||facts.resource!==binding.resource||held.lease.mode!==issued.mode||held.lease.resource!==issued.resource)throw Error("Exact retained cleanup writer proof required");
}
/** 🪪️ Requires the exact live system transaction issued for this original catalog writer. */
export function requireResourceLeaseCustodyV1(lease:ResourceLease,directory:string,resource:string,control:ResourceLeaseControl):void{checkResourceLeaseControl(control);const issued=originalLeases.get(lease);if(issued?.phase!=="held"||issued.path!==resourceLeasePathV1(directory,resource)||issued.signal!==control.signal||issued.mode!=="exclusive"||lease.mode!==issued.mode||lease.resource!==resource)throw Error("Original live catalog writer custody required");}

type LeaseDatabase = ResourceLeaseDatabaseV1;
const APPLICATION_ID = 0x534d4c53;

/** 🗜️ Gives system SQLite a closed page limit before its first bootstrap write. */
export function resourceLeaseMaximumPagesV1(maximumDatabaseBytes:number):number{if(boundedContract.validate({version:1,maximumDatabaseBytes}).length||maximumDatabaseBytes%4096!==0)throw Error("Closed bootstrap SQLite capacity required");return maximumDatabaseBytes/4096;}

/** 🗃️ Bun's own SQLite, named through a constant rather than inline. The specifier is a Bun BUILTIN:
 * a bundler asked to analyse this module refuses it outright ("Cannot bundle built-in module"), which
 * failed whole test FILES whose graph reaches this store from a browser-flavoured environment. Holding it
 * in a `const` (whose literal type still gives the import its real module type) plus `@vite-ignore` leaves
 * the resolution to the runtime, which is the only place it was ever meant to happen. */
const BUN_SQLITE_MODULE = "bun:sqlite" as const;

/** 🗃️ Uses each runtime's system SQLite behind the same local interface. */
async function openDatabase(path: string, readonly=false,create=!readonly): Promise<LeaseDatabase> {
  if (process.versions.bun) {
    const { Database } = await import(/* @vite-ignore */ BUN_SQLITE_MODULE), database = new Database(path, { readonly, readwrite:!readonly, create });let closed=false;
    return { exec: sql => database.exec(sql), read: (sql, ...values) => {const statement=database.prepare(sql);try{return statement.get(...values) as Record<string, unknown>|undefined;}finally{statement.finalize();}}, close: () => {if(!closed){database.close(true);closed=true;}} };
  }
  const { DatabaseSync } = await import("node:sqlite"), database = new DatabaseSync(path,{readOnly:readonly});let closed=false;
  return { exec: sql => database.exec(sql), read: (sql, ...values) => database.prepare(sql).get(...values), close: () => {if(!closed){database.close();closed=true;}} };
}

/** 🛣️ Keeps lock identities on regular paths without following a substituted store. */
function databasePath(directory: string, resource: string): string {
  const admitted = resourceLeasePathV1(directory, resource);
  let system = false;
  for (let current = resolve(directory); ; current = dirname(current)) {
    try {
      const stat = lstatSync(current);
      if (stat.isSymbolicLink() && isSystemLeaseLinkV1(current, stat) && statSync(current).isDirectory()) system = true;
      else if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid lease store directory: ${current}`);
    }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(directory, { recursive: true });
  const real = resourceLeasePathV1(realpathSync(directory), resource);
  if (!system && real !== admitted) throw Error("Resource lease materialized path changed");
  const path = system ? admitted : real;
  try { const stat = lstatSync(path); if (!stat.isFile() || stat.isSymbolicLink() || stat.nlink !== 1) throw new Error(`Invalid resource lease database: ${path}`); }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
  return path;
}

/** 🪪️ Initializes one immutable resource identity under SQLite's exclusive transaction. */
function initialize(database: LeaseDatabase, resource: string): void {
  const identity = database.read("PRAGMA application_id")?.application_id;
  if (identity === APPLICATION_ID) {if(resource===retentionSchema.properties.resource.const)requireResourceLeaseContinuationCatalogV1(database);return;}
  if (identity !== 0) throw new Error("Foreign resource lease database");
  database.exec("BEGIN EXCLUSIVE");
  try {
    const current = database.read("PRAGMA application_id")?.application_id;
    if (current !== APPLICATION_ID) {
      if (current !== 0) throw new Error("Foreign resource lease database");
      if (database.read("SELECT count(*) AS count FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%'")?.count !== 0) throw new Error("Unowned resource lease database");
      database.exec(`CREATE TABLE semio_resource_lease (id INTEGER PRIMARY KEY CHECK (id = 1), version INTEGER NOT NULL CHECK (version = 1), resource TEXT NOT NULL); PRAGMA application_id = ${APPLICATION_ID}`);
      database.read("INSERT INTO semio_resource_lease VALUES (1, 1, ?) RETURNING resource", resource);
      if(resource===retentionSchema.properties.resource.const)initializeResourceLeaseContinuationV1(database);
    }
    database.exec("COMMIT");
  } catch (error) { try{database.exec("ROLLBACK");}catch{}throw error; }
}

/** 📖️ Requires the privately issued shared mode through the exact live original reader. */
export function requireResourceLeaseReadCustodyV1(lease:ResourceLease,directory:string,resource:string,control:ResourceLeaseControl):void{checkResourceLeaseControl(control);const issued=originalLeases.get(lease);if(issued?.phase!=="held"||issued.mode!=="shared"||issued.control!==control||issued.directory!==resolve(directory)||issued.resource!==resource||issued.path!==resourceLeasePathV1(directory,resource)||issued.signal!==control.signal||lease.mode!==issued.mode||lease.resource!==resource)throw Error("Original live resource reader custody required");}

/** 🔒️ Waits cancellably for shared or exclusive access; release only after the protected operation stops. */
async function acquireUngatedResourceLease(options: LeaseOptions,started:number,onUnclosed?:(database:LeaseDatabase)=>void,cleanup?:CleanupLeaseScope): Promise<ResourceLease> {
  options=Object.freeze({...options});
  options.signal.throwIfAborted();
  if (typeof options.resource !== "string" || !options.resource.length || options.resource.length > 4096 || options.resource.includes("\0")) throw new Error("Invalid lease resource");
  if (!["shared", "exclusive"].includes(options.mode)) throw new Error("Invalid resource access mode");
  if (options.timeoutMs !== undefined && (!Number.isFinite(options.timeoutMs) || options.timeoutMs < 0)) throw new Error("Invalid resource lease timeout");
  remaining(options, started,cleanup);
  if(options.resource===retentionSchema.properties.resource.const&&options.maximumDatabaseBytes!==undefined&&options.maximumDatabaseBytes>retentionSchema.properties.maximumDatabaseBytes.const)throw Error("Fixed retirement catalog capacity required");const maximumDatabaseBytes=options.maximumDatabaseBytes??(options.resource===retentionSchema.properties.resource.const?retentionSchema.properties.maximumDatabaseBytes.const:undefined),maximumPages=maximumDatabaseBytes===undefined?undefined:resourceLeaseMaximumPagesV1(maximumDatabaseBytes),path=databasePath(options.directory,options.resource);
  if(maximumPages!==undefined){try{if(lstatSync(path).size>maximumDatabaseBytes!)throw Error("Original bootstrap SQLite exceeds capacity");}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}
  const database = await openDatabase(path);
  let nextProgress = 0, acquired = false,failed=false,failure:unknown;
  try {
    remaining(options,started,cleanup);database.exec("PRAGMA busy_timeout = 0; PRAGMA trusted_schema = OFF");
    for (;;) {
      const budget = remaining(options, started,cleanup);
      if (options.timeoutMs !== 0 && budget <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      try {
    if(maximumPages!==undefined){database.exec("PRAGMA page_size=4096");if(database.read("PRAGMA page_size")?.page_size!==4096||Number(database.read(`PRAGMA max_page_count=${maximumPages}`)?.max_page_count)!==maximumPages)throw Error("Actual bootstrap SQLite page cap refused");}
        if (database.read("PRAGMA journal_mode")?.journal_mode !== "delete") throw new Error("Resource lease requires rollback journal mode");
        initialize(database, options.resource);
        database.exec(options.mode === "exclusive" ? "BEGIN EXCLUSIVE" : "BEGIN");
        const row = database.read("SELECT version, resource FROM semio_resource_lease WHERE id = 1");
        if (row?.version !== 1 || row.resource !== options.resource || database.read("PRAGMA application_id")?.application_id !== APPLICATION_ID) throw new Error("Resource lease identity mismatch");
        if (remaining(options, started,cleanup) <= 0 && options.timeoutMs !== 0) throw new Error(`Resource lease timed out: ${options.resource}`);
        acquired = true;
        let released = false;
        const issued:IssuedResourceLeaseV1={path,directory:resolve(options.directory),resource:options.resource,database,control:options.control,signal:options.signal,phase:"held",mode:options.mode};const lease:ResourceLease={ resource: options.resource, mode: options.mode, release() {
          if (released) return;
          issued.phase="closing";database.close();released = true;issued.phase="closed";
        } };originalLeases.set(lease,issued);return lease;
      } catch (error) {
        try { database.exec("ROLLBACK"); } catch {}
        const code = error as { code?: string; errcode?: number; errno?: number };
        if (code.code !== "SQLITE_BUSY" && code.errcode !== 5 && code.errno !== 5) throw error;
      }
      const elapsedMs = Date.now() - started;
      if (remaining(options, started,cleanup) <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      if (elapsedMs >= nextProgress) {
        const progress = { resource: options.resource, mode: options.mode, elapsedMs };
        await waitProgress(options, progress, started,cleanup);
        nextProgress = elapsedMs + 1000;
      }
      await delay(Math.max(1, Math.min(40, remaining(options, started,cleanup))), undefined, { signal: options.signal }).catch(error=>{options.signal.throwIfAborted();throw error;});
      remaining(options, started,cleanup);
    }
  } catch(error){failed=true;failure=error;throw error;} finally { if (!acquired) finishResourceLeaseCleanupV1(failed,failure,[()=>{try{database.close();}catch(error){onUnclosed?.(database);throw error;}}]); }
}

const admissionResource=retirementSchema.properties.admissionResource.const;
function requireAdmission(gate:ResourceLease,directory:string,exclusive=false):void{const held=originalLeases.get(gate);if(held?.phase!=="held"||held.directory!==resolve(directory)||held.resource!==admissionResource||gate.resource!==admissionResource||gate.mode!==held.mode||exclusive&&held.mode!=="exclusive")throw Error("Original directory admission custody required");}
const admissionFinalizer=new ResourceLeaseAdmissionFinalizerV1<ResourceLease>();
const catalogAdmissions=new WeakMap<ResourceLease,ResourceLease>();
function protectAdmission(gate:ResourceLease,database:LeaseDatabase):void{admissionFinalizer.protect(gate,database);}
function releaseAdmission(gate:ResourceLease|undefined):void{admissionFinalizer.release(gate);}
async function directoryAdmission(options:LeaseOptions,mode:ResourceAccess["mode"],started:number,cleanup?:CleanupLeaseScope):Promise<ResourceLease>{admissionFinalizer.retry();return acquireUngatedResourceLease({...options,resource:admissionResource,mode,maximumDatabaseBytes:retirementSchema.properties.maximumDatabaseBytes.const},started,undefined,cleanup);}
async function acquireUnderAdmission(options:LeaseOptions,gate:ResourceLease,started:number,cleanup?:CleanupLeaseScope):Promise<ResourceLease>{requireAdmission(gate,options.directory);return acquireUngatedResourceLease(options,started,database=>protectAdmission(gate,database),cleanup);}
function closeDatabase(database:LeaseDatabase,gate?:ResourceLease):void{try{database.close();}catch(error){if(gate)protectAdmission(gate,database);throw error;}}
function releaseUnderAdmission(lease:ResourceLease|undefined,gate:ResourceLease):void{if(!lease)return;try{lease.release();}catch(error){protectAdmission(gate,originalLeases.get(lease)!.database);throw error;}}

/** 🚪️ Fences every opener until actual identity SELECT or failed connection close settles. */
export function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{return traceLeaseV1(options,()=>acquireOrderedResourceLeaseV1(options));}
const trackedAcquisition=new AsyncLocalStorage<{readonly guard:LeaseOrderGuardV1;readonly directory:string;readonly resource:string}>();
const untrackedTask:object={};
const internalLeaseResource=(resource:string):boolean=>resource===admissionResource||resource===retentionSchema.properties.resource.const||resource.startsWith(custodySchema.properties.resourcePrefix.const)||resource==="resource-retirement";
function leaseTaskOf(options:Pick<LeaseOptions,"control"|"signal">):Readonly<{task:number;ancestors:readonly number[]}>{
 const lineage=currentLeaseLineageV1();
 if(lineage)return{task:lineage[0]!,ancestors:lineage.slice(1)};
 const key:object=options.control!==null&&typeof options.control==="object"?options.control:options.signal!==null&&typeof options.signal==="object"?options.signal:untrackedTask;
 return{task:leaseTaskIdV1(key),ancestors:[]};
}
/** 🧭️ Begins the order bookkeeping of one public acquisition; internal queue, claim and admission leases are not tracked. */
function orderGuard(options:LeaseOptions,queued:boolean):LeaseOrderGuardV1|undefined{
 if(typeof options.resource!=="string"||!options.resource.length||internalLeaseResource(options.resource))return undefined;
 return beginLeaseOrderV1({directory:resolve(options.directory),resource:options.resource,mode:options.mode,nonblocking:options.timeoutMs===0,bounded:options.timeoutMs!==undefined,...leaseTaskOf(options),queued});
}
/** 🚪️ Fences every opener until actual identity SELECT or failed connection close settles; a request that breaks the declared lock order or would wait while holding fails fast with a diagnostic. */
async function acquireOrderedResourceLeaseV1(options:LeaseOptions):Promise<ResourceLease>{
 const guard=orderGuard(options,false);
 if(!guard)return acquireResourceLeaseBody(options);
 let lease:ResourceLease;
 try{lease=await trackedAcquisition.run({guard,directory:resolve(options.directory),resource:options.resource},()=>acquireResourceLeaseBody(options));}catch(error){guard.finish();throw error;}
 guard.hold(lease);return lease;
}
/** 📼 Appends one row `{version,resource,mode,startedMs,acquiredMs,pid}` to the file named by `SEMIO_LEASE_TRACE` for every acquisition that waited at least 100 ms, so a harness can measure lock waits exactly instead of from the 15 s progress rows; never fails an acquisition. */
async function traceLeaseV1(options:LeaseOptions,acquire:()=>Promise<ResourceLease>):Promise<ResourceLease>{
  const path=process.env.SEMIO_LEASE_TRACE;
  if(!path)return acquire();
  const startedMs=Date.now(),lease=await acquire(),acquiredMs=Date.now();
  if(acquiredMs-startedMs>=100)try{appendFileSync(path,JSON.stringify({version:1,resource:options.resource,mode:options.mode,startedMs,acquiredMs,pid:process.pid})+"\n");}catch{}
  return lease;
}
async function acquireResourceLeaseBody(options:LeaseOptions,cleanup?:CleanupLeaseScope):Promise<ResourceLease>{
 options=Object.freeze({...options});options.signal.throwIfAborted();if(options.resource===admissionResource)throw Error("Directory admission resource is reserved");if(typeof options.resource!=="string"||!options.resource.length||options.resource.length>4096||options.resource.includes("\0"))throw Error("Invalid lease resource");if(!["shared","exclusive"].includes(options.mode))throw Error("Invalid resource access mode");if(options.timeoutMs!==undefined&&(!Number.isFinite(options.timeoutMs)||options.timeoutMs<0))throw Error("Invalid resource lease timeout");const started=Date.now();remaining(options,started,cleanup);
 if(!cleanup&&options.timeoutMs!==0&&leaseOrderPolicyV1().gate.releaseWhileWaiting)return acquireWaitingOutsideGate(options,started);
 return acquireHoldingGate(options,started,cleanup);
}
/** 🚰️ Waits for a busy lease without holding the directory admission gate: each attempt takes the gate shared, tries once and gives the gate back, so retirement and every other opener are never blocked by a waiter. */
async function acquireWaitingOutsideGate(options:LeaseOptions,started:number):Promise<ResourceLease>{
 const policy=leaseOrderPolicyV1().gate,tracked=trackedAcquisition.getStore(),guard=tracked&&tracked.resource===options.resource&&tracked.directory===resolve(options.directory)?tracked.guard:undefined;
 let nextProgress=0,poll=policy.pollInitialMilliseconds;
 for(;;){
  try{return await acquireHoldingGate({...options,timeoutMs:0},Date.now());}
  catch(error){if(!(error instanceof Error)||error.message!=="Resource lease timed out: "+options.resource)throw error;}
  const elapsedMs=Date.now()-started;
  if(remaining(options,started)<=0)throw new Error(`Resource lease timed out: ${options.resource}`);
  if(elapsedMs>=nextProgress){await waitProgress(options,{resource:options.resource,mode:options.mode,elapsedMs},started);nextProgress=elapsedMs+1000;}
  if(guard)await guard.contention(elapsedMs);
  await delay(Math.max(1,Math.min(poll,remaining(options,started))),undefined,{signal:options.signal}).catch(error=>{options.signal.throwIfAborted();throw error;});
  poll=Math.min(policy.pollMaximumMilliseconds,Math.ceil(poll*3/2));remaining(options,started);
 }
}
async function acquireHoldingGate(options:LeaseOptions,started:number,cleanup?:CleanupLeaseScope):Promise<ResourceLease>{
 let gate:ResourceLease|undefined,lease:ResourceLease|undefined,failed=false,failure:unknown;
 try{gate=await directoryAdmission(options,"shared",started,cleanup);if(cleanup)cleanupLeaseIdentity(cleanup);lease=await acquireUnderAdmission(options,gate,started,cleanup);}catch(error){failed=true;failure=error;if(error instanceof Error&&error.message==="Resource lease timed out: "+admissionResource)failure=Error("Resource lease timed out: "+options.resource);}
 try{releaseAdmission(gate);}catch(error){if(!failed){failed=true;failure=error;}try{lease?.release();}catch{if(gate&&lease)protectAdmission(gate,originalLeases.get(lease)!.database);}}
 if(failed)throw failure;return lease!;
}

/** 🚦️ One waiter's place in a resource's arrival queue: `<directory>/<resource-hash>.queue/<arrivedAtMs>-<pid>-<owner>`. */
export type QueuedLeaseOptions = LeaseOptions & { readonly owner: string; readonly sharedCohort?: boolean };

/** 🎟️ Names one live request using the exact production ticket contract. */
export function resourceQueueTicketV1(request:{readonly arrivedAtMs:number;readonly pid:number;readonly owner:string}):string{
  if(!Number.isSafeInteger(request.arrivedAtMs)||request.arrivedAtMs<queueSchema.properties.arrivedAtMs.minimum||request.arrivedAtMs>queueSchema.properties.arrivedAtMs.maximum||!Number.isSafeInteger(request.pid)||request.pid<queueSchema.properties.pid.minimum||request.pid>queueSchema.properties.pid.maximum||!new RegExp(queueSchema.properties.owner.pattern,"u").test(request.owner))throw Error("Invalid lease queue ticket");
  const rule=queueSchema.definitions.Protocol.const;return `${request.arrivedAtMs.toString(rule.arrivalBase).padStart(rule.arrivalDigits,"0")}-${request.pid.toString(rule.pidBase).padStart(rule.pidDigits,"0")}-${request.owner}`;
}

function queueTicketRequest(name:string):{arrivedAtMs:number;pid:number;owner:string}{const rule=queueSchema.definitions.Protocol.const;if(!new RegExp(rule.ticketPattern,"u").test(name))throw Error("Canonical queue ticket required");const parts=name.split("-"),request={arrivedAtMs:Number.parseInt(parts[0]!,rule.arrivalBase),pid:Number.parseInt(parts[1]!,rule.pidBase),owner:parts.slice(2).join("-")};if(resourceQueueTicketV1(request)!==name)throw Error("Canonical bounded queue ticket required");return request;}
function arrival(name:string):number{return queueTicketRequest(name).arrivedAtMs;}

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
export function resourceQueueClaimV1(directory:string,resource:string,ticket:string):Readonly<{directory:string;resource:string}>{queueTicketRequest(ticket);const claimDirectory=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const)),claimResource=custodySchema.properties.resourcePrefix.const+createHash("sha256").update(resource).digest("hex")+":"+ticket;resourceLeasePathV1(claimDirectory,claimResource);return Object.freeze({directory:claimDirectory,resource:claimResource});}

/** 🧭️ Derives one canonical claim catalog per data namespace for every opener and retirement. */
export function resourceQueueCatalogPathV1(directory:string):string{return resourceLeasePathV1(directory,retentionSchema.properties.resource.const);}

function retirementCatalogDatabase(catalog:ResourceLease,directory:string,control:ResourceLeaseControl,gate?:ResourceLease):LeaseDatabase{const admission=catalogAdmissions.get(catalog)??gate;if(!admission)throw Error("Actual retirement catalog admission required");requireAdmission(admission,directory,true);if(originalLeases.get(admission)?.control!==control)throw Error("Exact retirement admission control required");requireResourceLeaseCustodyV1(catalog,directory,retentionSchema.properties.resource.const,control);const held=originalLeases.get(catalog);if(held?.control!==control)throw Error("Exact retirement catalog control required");return held.database;}
function persistRetirementContinuation(catalog:ResourceLease,directory:string,control:ResourceLeaseControl,kind:ResourceLeaseContinuationKindV1,after:string,gate?:ResourceLease):void{const database=retirementCatalogDatabase(catalog,directory,control,gate),issued=originalLeases.get(catalog)!,admission=catalogAdmissions.get(catalog)??gate!;issued.phase="transition";try{writeResourceLeaseContinuationV1(database,kind,after);checkResourceLeaseControl(control);requireAdmission(admission,directory,true);database.exec("COMMIT");checkResourceLeaseControl(control);requireAdmission(admission,directory,true);database.exec("BEGIN EXCLUSIVE");issued.phase="held";retirementCatalogDatabase(catalog,directory,control,gate);if(readResourceLeaseContinuationV1(database,kind)!==after)throw Error("Original committed retirement cursor required");}catch(error){issued.phase="transition";throw error;}}
async function queueClaimCatalog(options:LeaseOptions,mode:"shared"|"exclusive",started=Date.now()):Promise<ResourceLease>{
 let gate:ResourceLease|undefined,catalog:ResourceLease|undefined,failed=false,failure:unknown;try{gate=await directoryAdmission(options,mode,started);catalog=await acquireUnderAdmission({...options,resource:retentionSchema.properties.resource.const,mode,maximumDatabaseBytes:retentionSchema.properties.maximumDatabaseBytes.const},gate,started);const held=catalog,admission=gate,combined:ResourceLease={resource:held.resource,mode:held.mode,release(){finishResourceLeaseCleanupV1(false,undefined,[()=>releaseUnderAdmission(held,admission),()=>releaseAdmission(admission)]);}};originalLeases.set(combined,originalLeases.get(held)!);catalogAdmissions.set(combined,admission);return combined;}catch(error){failed=true;failure=error;throw error;}finally{if(failed)finishResourceLeaseCleanupV1(failed,failure,[()=>{if(catalog&&gate)releaseUnderAdmission(catalog,gate);else catalog?.release();},()=>releaseAdmission(gate)]);}
}
type ClaimIdentity=Readonly<{path:string;resource:string;queueTicketPath:string;device:number;inode:number;allocatedBytes:number}>;
async function readClaimIdentity(directory:string,candidate:ResourceLeaseCandidateV1,control:ResourceLeaseControl,gate:ResourceLease):Promise<ClaimIdentity|undefined>{
 checkResourceLeaseControl(control);const claims=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const));requireAdmission(gate,claims);if(candidate.directory!==claims||!/^[a-f0-9]{64}\.sqlite$/u.test(basename(candidate.path)))throw Error("Exact queue claim candidate required");let database:LeaseDatabase|undefined,failed=false,failure:unknown;try{database=await openDatabase(candidate.path,true);checkResourceLeaseControl(control);database.exec("PRAGMA busy_timeout=0;PRAGMA trusted_schema=OFF;BEGIN");const resource=readOwnedResourceLeaseV1(database,claims,candidate.path);if(!resource)return;const identity=new RegExp("^"+custodySchema.properties.resourcePrefix.const+"([a-f0-9]{64}):("+queueSchema.definitions.Protocol.const.ticketPattern.slice(1,-1)+")$","u").exec(resource);if(!identity)throw Error("Original queue claim path identity required");queueTicketRequest(identity[2]!);checkResourceLeaseControl(control);requireResourceLeaseCandidateV1(candidate);return Object.freeze({path:candidate.path,resource,queueTicketPath:requireMaterializedPathV1(join(directory,identity[1]+".queue",identity[2]!)),device:candidate.file.dev,inode:candidate.file.ino,allocatedBytes:candidate.allocatedBytes});}catch(error){failed=true;failure=error;throw error;}finally{if(database)finishResourceLeaseCleanupV1(failed,failure,[()=>closeDatabase(database!,gate)]);}
}

/** 🔎️ Protects the complete read-only claim connection in exact parent-before-child admission custody. */
export async function readResourceQueueClaimIdentityV1(directory:string,name:string,control:ResourceLeaseControl):Promise<ClaimIdentity|undefined>{
 const started=Date.now(),options:LeaseOptions={directory,resource:retentionSchema.properties.resource.const,mode:"shared",signal:control.signal,control};let catalog:ResourceLease|undefined,gate:ResourceLease|undefined,failed=false,failure:unknown;try{catalog=await queueClaimCatalog(options,"shared",started);gate=await directoryAdmission({...options,directory:join(directory,custodySchema.properties.claimDirectory.const)},"shared",started);checkResourceLeaseControl(control);const claims=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const));requireAdmission(gate,claims);if(!/^[a-f0-9]{64}\.sqlite$/u.test(name))throw Error("Exact queue claim filename required");const candidate=captureResourceLeaseCandidateV1(claims,name);return candidate?await readClaimIdentity(directory,candidate,control,gate):undefined;}catch(error){failed=true;failure=error;throw error;}finally{finishResourceLeaseCleanupV1(failed,failure,[()=>releaseAdmission(gate),()=>catalog?.release()]);}
}

/** 📊️ Reports immediate physical databases examined and safely retired in one bounded turn. */
export type ResourceLeaseRetirementV1=Readonly<{examined:number;retired:number;allocatedBytes:number}>;
function databaseRejection(error:unknown):boolean{const value=error!==null&&typeof error==="object"?error as{code?:string;errcode?:number;errno?:number}:undefined;return ["SQLITE_ERROR","SQLITE_CORRUPT","SQLITE_NOTADB","SQLITE_SCHEMA"].includes(value?.code??"")||[1,11,17,26].includes((value?.errcode??value?.errno??-1)&255);}
function recoverRetirementHeadroom(error:unknown,directory:string,headroom:ReturnType<typeof import("./📦️metadata/🟦️.ts").retainResourceQueueMetadataHeadroomV1>|undefined):void{if(!headroom)throw error;const code=error!==null&&typeof error==="object"?(error as NodeJS.ErrnoException).code:undefined;if(code!=="ENOSPC"&&!(code==="EFAULT"&&statfsSync(directory).bavail<8))throw error;headroom.recover();}
async function retireResources(directory:string,control:ResourceLeaseControl,timeoutMs:number,resource?:string):Promise<ResourceLeaseRetirementV1>{
 checkResourceLeaseControl(control);const started=Date.now(),options:LeaseOptions={directory,resource:"resource-retirement",mode:"exclusive",signal:control.signal,control,timeoutMs};let gate:ResourceLease|undefined,catalog:ResourceLease|undefined,headroom:ReturnType<typeof import("./📦️metadata/🟦️.ts").retainResourceQueueMetadataHeadroomV1>|undefined,failed=false,failure:unknown;let examined=0,retired=0,allocatedBytes=0;
 try{gate=await directoryAdmission(options,"exclusive",started);catalog=await acquireUnderAdmission({...options,resource:retentionSchema.properties.resource.const,timeoutMs:0,maximumDatabaseBytes:retentionSchema.properties.maximumDatabaseBytes.const},gate,started);if(lstatSync(join(directory,metadataSchema.properties.filename.const),{throwIfNoEntry:false}))headroom=(await import("./📦️metadata/🟦️.ts")).retainResourceQueueMetadataHeadroomV1(directory,catalog,control,requireResourceLeaseCustodyV1);const names=resource===undefined?await resourceLeaseRetirementNamesV1(directory,control):[resourceLeasePathV1(directory,resource).slice(resolve(directory).length+1)],admissionPath=resourceLeasePathV1(directory,admissionResource),catalogPath=resourceQueueCatalogPathV1(directory);
 const selected=names.filter(name=>/^[a-f0-9]{64}\.sqlite$/u.test(name)),prior=resource===undefined?readResourceLeaseContinuationV1(retirementCatalogDatabase(catalog,directory,control,gate),"resources"):"";let scanned=0,at=resource===undefined?resourceLeaseContinuationStartV1(selected,prior):0,last="";for(;at<selected.length;at++){const name=selected[at]!;if(examined>=retirementSchema.properties.maximumCandidatesPerTurn.const)break;last=name;remaining(options,started);if(++scanned%resourceLeaseRetirementScanLimitsV1.entriesPerYield===0){if(scanned===resourceLeaseRetirementScanLimitsV1.entriesPerYield||scanned%resourceLeaseRetirementScanLimitsV1.entriesPerProgress===0)await control.publish({resource:"resource-retirement-scan:"+scanned,mode:"exclusive",elapsedMs:Math.max(0,Date.now()-started)});await control.yieldContinuation();remaining(options,started);}const path=join(directory,name);if(path===admissionPath||path===catalogPath)continue;const candidate=captureResourceLeaseCandidateV1(directory,name);if(!candidate)continue;examined++;let database:LeaseDatabase|undefined,probeFailed=false,probeFailure:unknown;
  try{requireAdmission(gate,directory,true);database=await openDatabase(path,false,false);remaining(options,started);database.exec("PRAGMA busy_timeout=0;PRAGMA trusted_schema=OFF");if(database.read("PRAGMA application_id")?.application_id!==APPLICATION_ID)continue;database.exec("BEGIN EXCLUSIVE");const logical=readOwnedResourceLeaseV1(database,directory,path);if(logical===undefined||logical.startsWith(custodySchema.properties.resourcePrefix.const)||resource!==undefined&&logical!==resource)continue;remaining(options,started);requireResourceLeaseCandidateV1(candidate);closeDatabase(database,gate);database=undefined;const remove=()=>{requireAdmission(gate!,directory,true);requireResourceLeaseCandidateV1(candidate);unlinkSync(path);};try{remove();}catch(error){recoverRetirementHeadroom(error,directory,headroom);remove();}retired++;allocatedBytes+=candidate.allocatedBytes;
  }catch(error){probeFailed=true;probeFailure=error;if(!resourceLeaseBusyV1(error)&&!databaseRejection(error))throw error;probeFailed=false;}finally{if(database)finishResourceLeaseCleanupV1(probeFailed,probeFailure,[()=>closeDatabase(database!,gate)]);}
  await control.yieldContinuation();checkResourceLeaseControl(control);
 }if(resource===undefined){const after=at===selected.length?"":last;if(after!==prior){headroom?.recover();persistRetirementContinuation(catalog,directory,control,"resources",after,gate);}}return Object.freeze({examined,retired,allocatedBytes});
 }catch(error){failed=true;failure=error;throw error;}finally{finishResourceLeaseCleanupV1(failed,failure,[()=>headroom?.recheck(),()=>{if(catalog&&gate)releaseUnderAdmission(catalog,gate);else catalog?.release();},()=>releaseAdmission(gate)]);}
}

/** ♻️ Retires only authenticated idle immediate databases while exact directory and catalog writers remain retained. */
export function retireResourceLeasesV1(directory:string,control:ResourceLeaseControl,timeoutMs=0):Promise<ResourceLeaseRetirementV1>{return retireResources(resolve(directory),control,timeoutMs);}
/** 🎯️ Retires one exact settled owner through the same protected close-to-unlink protocol. */
export function retireResourceLeaseV1(directory:string,resource:string,control:ResourceLeaseControl,timeoutMs=0):Promise<ResourceLeaseRetirementV1>{resourceLeasePathV1(directory,resource);if(typeof resource!=="string"||!resource.length||resource.length>4096||resource.includes("\0"))throw Error("Invalid retirement resource");return retireResources(resolve(directory),control,timeoutMs,resource);}

/** ♻️ Retires settled claims under data admission, queue catalog and exact claims admission in fixed order. */
export async function retireResourceQueueClaimsV1(directory:string,control:ResourceLeaseControl,timeoutMs=0):Promise<ResourceLeaseRetirementV1>{
 checkResourceLeaseControl(control);const started=Date.now(),options:LeaseOptions={directory,resource:retentionSchema.properties.resource.const,mode:"exclusive",signal:control.signal,control,timeoutMs};let catalog:ResourceLease|undefined,gate:ResourceLease|undefined,headroom:ReturnType<typeof import("./📦️metadata/🟦️.ts").retainResourceQueueMetadataHeadroomV1>|undefined,failed=false,failure:unknown;let examined=0,retired=0,allocatedBytes=0;
 try{try{catalog=await queueClaimCatalog(options,"exclusive",started);gate=await directoryAdmission({...options,directory:join(directory,custodySchema.properties.claimDirectory.const),timeoutMs:0},"exclusive",started);}catch(error){if(resourceLeaseBusyV1(error))return Object.freeze({examined,retired,allocatedBytes});throw error;}headroom=(await import("./📦️metadata/🟦️.ts")).retainResourceQueueMetadataHeadroomV1(directory,catalog,control,requireResourceLeaseCustodyV1);
 const claims=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const)),admissionName=basename(resourceLeasePathV1(claims,admissionResource)),names=(await resourceLeaseRetirementNamesV1(claims,control)).filter(name=>/^[a-f0-9]{64}\.sqlite$/u.test(name)&&name!==admissionName),prior=readResourceLeaseContinuationV1(retirementCatalogDatabase(catalog,directory,control),"claims");let scanned=0,at=resourceLeaseContinuationStartV1(names,prior),last="";
 for(;at<names.length;at++){const name=names[at]!;if(examined>=retentionSchema.properties.maximumClaimsPerTurn.const)break;last=name;remaining(options,started);if(++scanned%resourceLeaseRetirementScanLimitsV1.entriesPerYield===0){if(scanned===resourceLeaseRetirementScanLimitsV1.entriesPerYield||scanned%resourceLeaseRetirementScanLimitsV1.entriesPerProgress===0)await control.publish({resource:"claim-retirement-scan:"+scanned,mode:"exclusive",elapsedMs:Math.max(0,Date.now()-started)});await control.yieldContinuation();remaining(options,started);}requireAdmission(gate,claims,true);const admitted=captureResourceLeaseCandidateV1(claims,name);if(!admitted)continue;examined++;let held:ResourceLease|undefined,probeFailed=false,probeFailure:unknown;try{const row=await readClaimIdentity(directory,admitted,control,gate);if(!row)continue;held=await acquireUnderAdmission({directory:dirname(row.path),resource:row.resource,mode:"exclusive",signal:control.signal,control,timeoutMs:0,maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const},gate,started);remaining(options,started);const candidate=captureResourceLeaseCandidateV1(dirname(row.path),name);if(!candidate||candidate.file.dev!==row.device||candidate.file.ino!==row.inode)throw Error("Original queue claim inode changed");const remove=()=>{try{unlinkSync(row.queueTicketPath);}catch(error){if((error as NodeJS.ErrnoException|undefined)?.code!=="ENOENT")throw error;}releaseUnderAdmission(held,gate!);held=undefined;requireAdmission(gate!,dirname(row.path),true);requireResourceLeaseCandidateV1(candidate);unlinkSync(row.path);};try{remove();}catch(error){recoverRetirementHeadroom(error,directory,headroom);remove();}retired++;allocatedBytes+=row.allocatedBytes;}catch(error){probeFailed=true;probeFailure=error;if(!resourceLeaseBusyV1(error)&&!databaseRejection(error))throw error;}finally{finishResourceLeaseCleanupV1(probeFailed&&!resourceLeaseBusyV1(probeFailure)&&!databaseRejection(probeFailure),probeFailure,[()=>{if(held)releaseUnderAdmission(held,gate!);}]);}await control.yieldContinuation();checkResourceLeaseControl(control);
 }const after=at===names.length?"":last;if(after!==prior){headroom?.recover();persistRetirementContinuation(catalog,directory,control,"claims",after);}return Object.freeze({examined,retired,allocatedBytes});
 }catch(error){failed=true;failure=error;throw error;}finally{finishResourceLeaseCleanupV1(failed,failure,[()=>headroom?.recheck(),()=>releaseAdmission(gate),()=>catalog?.release()]);}
}

/** ⏳️ Closed poll, sweep and progress cadence declared by `⏳️wait/🧬️schema/📜️policy.json`. */
const waitPolicy=waitPolicySchema.definitions.Policy.const;
type QueuedTurn={readonly options:QueuedLeaseOptions;readonly queue:string;readonly ticket:string;readonly claim:ResourceLease;inner?:ResourceLease;consumed:boolean};
const queuedTurns=new WeakMap<ResourceLease,QueuedTurn>();
type QueueWaitInfo={version:1;position:number;queued:number;holderMode:"shared"|"exclusive";holderPid:number;holderSeconds:number;resource:string};

function queueTicketRow(queue:string,name:string):{pid:number;mode:"shared"|"exclusive"}|undefined{
  let row:Record<string,unknown>|undefined;try{row=JSON.parse(readFileSync(join(queue,name),"utf8"));}catch{return undefined;}
  if(row===null||typeof row!=="object"||Object.keys(row).sort().join(",")!=="mode,pid,sharedCohort,version"||row.version!==1||row.pid!==queueTicketRequest(name).pid||(row.mode!=="shared"&&row.mode!=="exclusive")||typeof row.sharedCohort!=="boolean")return undefined;
  return{pid:row.pid as number,mode:row.mode};
}
function queueWaitInfo(options:QueuedLeaseOptions,queue:string,live:readonly string[],ticket:string,queued:number):QueueWaitInfo|undefined{
  const position=live.indexOf(ticket)+1;if(position<2)return undefined;
  const head=live[0]!,row=queueTicketRow(queue,head);if(!row)return undefined;
  const limit=waitPolicy.maximumReportedResourceCharacters,resource=options.resource.length>limit?"…"+options.resource.slice(-(limit-1)):options.resource;
  return{version:1,position,queued:Math.max(queued,position),holderMode:row.mode,holderPid:row.pid,holderSeconds:Math.max(0,Math.floor((Date.now()-arrival(head))/1000)),resource};
}
/** 🧹 Retires one predecessor ticket only when its independent kernel claim proves its requester gone. */
async function reapQueueTicket(options:QueuedLeaseOptions,queue:string,name:string,pendingOptions:()=>LeaseOptions):Promise<boolean>{
  const sweepCatalog=await queueClaimCatalog(pendingOptions(),"shared");let retired:ResourceLease|undefined;
  try{retired=await acquireResourceLease({...resourceQueueClaimV1(options.directory,options.resource,name),mode:"exclusive",signal:options.signal,control:options.control,timeoutMs:0,maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});rmSync(join(queue,name),{force:true});return true;}
  catch(error){if(!String(error).includes("Resource lease timed out"))throw error;return false;}
  finally{retired?.release();sweepCatalog.release();}
}
function wrapQueuedLease(lease:ResourceLease,turn:QueuedTurn):ResourceLease{
  const queued:ResourceLease={resource:lease.resource,mode:lease.mode,release(){
    if(turn.consumed)return;
    try{lease.release();}finally{try{rmSync(join(turn.queue,turn.ticket),{force:true});}finally{turn.claim.release();}}
  }};
  originalLeases.set(queued,originalLeases.get(lease)!);queuedTurns.set(queued,turn);turn.inner=lease;return queued;
}

/** 🚦️ Serves original kernel-held tickets in arrival order; an exclusive SQLite claim alone proves abandoned custody. */
export function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {return traceLeaseV1(options,()=>acquireOrderedQueuedResourceLeaseV1(options));}
async function acquireOrderedQueuedResourceLeaseV1(options: QueuedLeaseOptions): Promise<ResourceLease> {
  if(options.mode==="shared"&&typeof options.resource==="string"&&!internalLeaseResource(options.resource)&&leaseOrderPolicyV1().fairness.reentrantShared!==false&&holdsSharedInProcessV1(resolve(options.directory),options.resource)&&exclusiveTicketQueued(options.directory,options.resource)){const{owner:_owner,sharedCohort:_cohort,...raw}=options;return acquireResourceLease(raw);}
  const guard=orderGuard(options,true);
  if(!guard)return acquireQueuedResourceLeaseBody(options);
  let lease:ResourceLease;
  try{lease=await acquireQueuedResourceLeaseBody(options,guard);}catch(error){guard.finish();throw error;}
  guard.hold(lease);return lease;
}
async function acquireQueuedResourceLeaseBody(options: QueuedLeaseOptions,guard?:LeaseOrderGuardV1): Promise<ResourceLease> {
  options=Object.freeze({...options});
  options.signal.throwIfAborted();
  if("claimCatalogDirectory" in options)throw Error("Queue claim catalog selectors are forbidden");
  if(options.sharedCohort!==undefined&&(typeof options.sharedCohort!=="boolean"||options.sharedCohort&&options.mode!=="shared"))throw Error("Shared lease cohorts require shared access");
  if (!/^[A-Za-z0-9._-]{1,64}$/u.test(options.owner)) throw new Error("Invalid lease queue owner");
  const started = Date.now(); remaining(options, started);
  const pendingOptions=():LeaseOptions=>({...options,timeoutMs:options.timeoutMs===undefined?undefined:Math.max(0,remaining(options,started))});
  if(options.control)await retireResourceQueueClaimsV1(options.directory,options.control,0);
  remaining(options,started);
  const ticket = resourceQueueTicketV1({arrivedAtMs:started,pid:process.pid,owner:options.owner});
  const admitted = resourceLeasePathV1(options.directory, options.resource, ticket);
  const queue = databasePath(options.directory, options.resource).replace(/\.sqlite$/u, ".queue");
  if (queue !== admitted.replace(/\.sqlite$/u, ".queue")) throw Error("Resource queue materialized path changed");
  mkdirSync(queue, { recursive: true });
  const catalog=await queueClaimCatalog(pendingOptions(),"shared");let claim:ResourceLease;
  try{claim=await acquireResourceLease({...pendingOptions(),...resourceQueueClaimV1(options.directory,options.resource,ticket),mode:"shared",maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});try{remaining(options,started);writeFileSync(join(queue,ticket),JSON.stringify({version:1,pid:process.pid,mode:options.mode,sharedCohort:options.sharedCohort===true}),{flag:"wx"});}catch(error){claim.release();throw error;}}finally{catalog.release();}
  return awaitQueuedTurn(options,started,queue,ticket,claim,guard);
}

/** 🎟️ Whether an exclusive request is queued on a lease: only then can a further shared request of a holder be stuck behind a writer, so only then does it skip the queue; otherwise it queues normally and stays upgradeable. */
function exclusiveTicketQueued(directory:string,resource:string):boolean{
  try{const queue=resourceLeasePathV1(directory,resource).replace(/\.sqlite$/u,".queue");for(const name of readdirSync(queue))if(queueTicketRow(queue,name)?.mode==="exclusive")return true;}catch{}
  return false;
}
/** ⬆️ Converts a held queued shared lease into an exclusive one at its original arrival position; the shared hold is consumed, so a failed upgrade holds nothing and concurrent upgraders cannot deadlock each other. */
export async function upgradeQueuedResourceLease(lease:ResourceLease,limits:{readonly timeoutMs?:number}={}):Promise<ResourceLease>{
  const turn=queuedTurns.get(lease),issued=originalLeases.get(lease);
  if(!turn||turn.consumed||!turn.inner||lease.mode!=="shared"||issued?.phase!=="held")throw Error("Live queued shared lease required for upgrade");
  if(limits.timeoutMs!==undefined&&(!Number.isFinite(limits.timeoutMs)||limits.timeoutMs<0))throw Error("Invalid resource lease timeout");
  const{sharedCohort:_cohort,timeoutMs:_timeout,...base}=turn.options,options:QueuedLeaseOptions=Object.freeze({...base,mode:"exclusive",...(limits.timeoutMs===undefined?{}:{timeoutMs:limits.timeoutMs})});
  const started=Date.now();remaining(options,started);forgetLeaseV1(lease);const guard=orderGuard(options,true);
  const name=join(turn.queue,turn.ticket),temporary=join(dirname(turn.queue),turn.ticket+".u");
  try{writeFileSync(temporary,JSON.stringify({version:1,pid:process.pid,mode:"exclusive",sharedCohort:false}),{flag:"wx"});renameSync(temporary,name);}catch(error){rmSync(temporary,{force:true});throw error;}
  turn.consumed=true;
  try{turn.inner.release();}catch(error){try{rmSync(name,{force:true});}finally{turn.claim.release();}throw error;}
  let upgraded:ResourceLease;try{upgraded=await awaitQueuedTurn(options,started,turn.queue,turn.ticket,turn.claim,guard);}catch(error){guard?.finish();throw error;}
  guard?.hold(upgraded);return upgraded;
}

async function awaitQueuedTurn(options:QueuedLeaseOptions,started:number,queue:string,ticket:string,claim:ResourceLease,guard?:LeaseOrderGuardV1):Promise<ResourceLease>{
  const pendingOptions=():LeaseOptions=>({...options,timeoutMs:options.timeoutMs===undefined?undefined:Math.max(0,remaining(options,started))});
  const swept=new Map<string,number>();let nextProgress=0,poll=waitPolicy.pollInitialMilliseconds;
  try {
    for (;;) {
      if (remaining(options, started) <= 0 && options.timeoutMs !== 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      const names=(await retryInterruptedFilesystemRead(options.signal,()=>readdirSync(queue),options.control)).sort((left,right)=>arrival(left)-arrival(right)||left.localeCompare(right,"en")),own=names.indexOf(ticket),live=own<0?[...names]:names.slice(0,own);
      while(live.length){
        const name=live[live.length-1]!,at=Date.now(),last=swept.get(name);
        if(last!==undefined&&at-last<waitPolicy.sweepIntervalMilliseconds)break;
        swept.set(name,at);if(!await reapQueueTicket(options,queue,name,pendingOptions))break;
        swept.delete(name);live.pop();
      }
      if(own>=0)live.push(ticket);
      remaining(options, started);
      const cohort=options.sharedCohort===true&&await retryInterruptedFilesystemRead(options.signal,()=>live.slice(0,live.indexOf(ticket)).every(name=>{let claim:unknown;try{claim=JSON.parse(readFileSync(join(queue,name),"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return true;if(error instanceof SyntaxError)return false;throw error;}const row=claim as Record<string,unknown>|undefined;return row!==null&&typeof row==="object"&&Object.keys(row).sort().join(",")==="mode,pid,sharedCohort,version"&&row.version===1&&row.pid===queueTicketRequest(name).pid&&row.mode==="shared"&&row.sharedCohort===true;}),options.control);
      remaining(options, started);
      if (live[0] === ticket || cohort) {
        const lease = await acquireResourceLeaseBody({ ...options, timeoutMs: options.timeoutMs === undefined ? undefined : Math.max(0, options.timeoutMs - (Date.now() - started)) });
        try { remaining(options, started); } catch (error) { lease.release(); throw error; }
        return wrapQueuedLease(lease,{options,queue,ticket,claim,consumed:false});
      }
      const elapsedMs = Date.now() - started;
      if (remaining(options, started) <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      await guard?.contention(elapsedMs);
      if (elapsedMs >= nextProgress) {
        const progress = { resource: options.resource, mode: options.mode, elapsedMs },info=options.control?queueWaitInfo(options,queue,live,ticket,names.length):undefined,control=options.control;
        await waitProgress(options, progress, started);
        if(info&&control)await waitForLeaseContinuation(options,started,()=>control.publish({resource:"lease-queue:"+JSON.stringify(info),mode:options.mode,elapsedMs:0}));
        nextProgress = elapsedMs + waitPolicy.progressIntervalMilliseconds;
      }
      await delay(Math.max(1, Math.min(poll, remaining(options, started))), undefined, { signal: options.signal }).catch(error=>{options.signal.throwIfAborted();throw error;});
      poll=Math.min(waitPolicy.pollMaximumMilliseconds,Math.ceil(poll*waitPolicy.pollGrowthNumerator/waitPolicy.pollGrowthDenominator));
      remaining(options, started);
    }
  } catch (error) {
    try{rmSync(join(queue, ticket), { force: true });}finally{claim.release();}
    throw error;
  }
}

function orderedLeaseResources(resources:readonly string[]):string[]{
 const policy=leaseOrderPolicyV1();
 return [...resources].sort((left,right)=>classifyLeaseResourceV1(policy,left).rank-classifyLeaseResourceV1(policy,right).rank||(left<right?-1:left>right?1:0));
}
function acquireOrdered(options:LeaseOptions&{readonly resources?:unknown}):Promise<ResourceLease>{
 const{resources:_resources,...request}=options;
 if(leaseOrderPolicyV1().fairness.queuedWithResourceLeases&&request.timeoutMs!==0&&request.maximumDatabaseBytes===undefined&&!internalLeaseResource(request.resource)&&!(()=>{const owner=leaseTaskOf(request);return heldLeasesV1(owner.task,owner.ancestors);})().some(row=>row.directory===resolve(request.directory)&&row.resource===request.resource))return acquireQueuedResourceLease({...request,owner:randomBytes(12).toString("hex"),...(request.mode==="shared"?{sharedCohort:true}:{})});
 return acquireResourceLease(request);
}
/** 🪢️ Orders a resource set by declared lock rank then name, takes shared and exclusive access through the fair arrival queue (readers join cohorts, writers are not starved, a task re-entering a lease it already holds bypasses the queue) and releases in reverse order on every exit. */
export async function withResourceLeases<T>(options: Omit<LeaseOptions, "resource" | "mode"> & { readonly resources: readonly ResourceAccess[] }, operation: () => Promise<T>): Promise<T> {
  const resources = new Map<string, ResourceAccess["mode"]>(), leases: ResourceLease[] = [];
  let failed=false,failure:unknown;
  for (const entry of options.resources) resources.set(entry.resource, resources.get(entry.resource) === "exclusive" ? "exclusive" : entry.mode);
  try {
    for (const resource of orderedLeaseResources([...resources.keys()])) leases.push(await acquireOrdered({ ...options, resource, mode: resources.get(resource)! }));
    options.signal.throwIfAborted();
    return await operation();
  } catch(error){failed=true;failure=error;throw error;} finally { finishResourceLeaseCleanupV1(failed,failure,leases.reverse().map(lease=>()=>lease.release())); }
}

/** 🧷️ Authenticates a preaccepted exact image anchor without renewing its stopped ordinary control. */
export function requireResourceLeaseCleanupContinuationV1(lease:ResourceLease,loan:AcceptedResourceLeaseCleanupV1,enrollment:AcceptedPhysicalCleanupV1):boolean{const scope=cleanupLoans.get(loan),issued=originalLeases.get(lease);if(!scope||scope.enrollment!==enrollment||scope.original.mode!=="exclusive"||scope.used||scope.active||!issued||issued.directory!==scope.directory||issued.resource!==scope.resource||issued.path!==scope.path||lease.resource!==issued.resource||lease.mode!==issued.mode)throw Error("Exact preaccepted lease continuation required");checkAcceptedPhysicalCleanupV1(enrollment);if(issued.phase!=="held"&&issued.phase!=="closed")throw Error("Confirmed lease cleanup continuation state required");cleanupLeaseIdentity(scope);return issued.phase==="held";}
