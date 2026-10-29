import{initializeResourceLeaseContinuationV1,requireResourceLeaseContinuationCatalogV1,readResourceLeaseContinuationV1,writeResourceLeaseContinuationV1,resourceLeaseContinuationStartV1,type ResourceLeaseContinuationKindV1,type ResourceLeaseTransactionPhaseV1}from"./♻️retirement/🧭️continuation/🟦️.ts";
import cleanupLeaseSchema from"./🧹️cleanup/🧬️schema/🔣️.json" with{type:"json"};
import metadataSchema from "./📦️metadata/🧬️schema/🔣️.json" with {type:"json"};
import {captureResourceLeaseCandidateV1,requireResourceLeaseCandidateV1,readOwnedResourceLeaseV1,resourceLeaseBusyV1,resourceLeaseRetirementNamesV1,resourceLeaseRetirementScanLimitsV1,resourceLeaseRetirementIdlePolicyV1,finishResourceLeaseCleanupV1,openResourceLeaseDatabaseV1,ResourceLeaseAdmissionFinalizerV1,type ResourceLeaseDatabaseV1,type ResourceLeaseCandidateV1} from "./♻️retirement/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { AsyncLocalStorage } from "node:async_hooks";
import {beginLeaseOrderV1,classifyLeaseResourceV1,forgetLeaseV1,currentLeaseLineageV1,heldLeasesV1,holdsSharedInProcessV1,leaseOrderPolicyV1,leaseTaskIdV1,type LeaseOrderGuardV1} from "./🧭️order/🟦️.ts";
export {LeaseOrderErrorV1,formatLeaseViolationV1,leaseInheritanceEnvironmentV1,leaseInvocationChainV1,otherLeaseHoldersV1,withLeaseTaskV1} from "./🧭️order/🟦️.ts";
import {readIssuedResourceControlCapabilitiesV1,requireIssuedResourceControlV1,requireAcceptedPhysicalCleanupDescendantV1,requireAcceptedPhysicalCleanupControlV1,checkAcceptedPhysicalCleanupV1,noteAcceptedPhysicalCleanupProgressV1,readAcceptedPhysicalCleanupV1,requireIssuedResourceControlRefinementV1,requireIssuedResourceControlDescendantV1,withAcceptedPhysicalCleanupOperationV1,closeAcceptedPhysicalCleanupChildrenV1} from "./🦥️routing/🟦️.ts";
import type {AcceptedPhysicalCleanupV1} from "../🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🟦️.ts";
import { appendFileSync, closeSync, lstatSync, openSync, statSync, mkdirSync, readdirSync, readFileSync, realpathSync, rmdirSync, rmSync, statfsSync, unlinkSync, utimesSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";
import {leaseDatabaseUnopenableBusyV1,leaseFileBusyV1,leaseFilePortsV1,leaseRemovalOptionsV1,leaseUnopenablePausesV1,settleLeaseFileV1,type LeaseFilePortsV1} from "./🪟️windows/🟦️.ts";

/** 🛑️ The refusal texts load when a refusal happens, never at startup: every entry that takes a lease stays inside its module budget. */
const refuseProcessV1 = (...input: Parameters<typeof import("../📖️texts/🟦️.ts").refuseProcessV1>): ReturnType<typeof import("../📖️texts/🟦️.ts").refuseProcessV1> => (createRequire(import.meta.url)("../📖️texts/🟦️.ts") as typeof import("../📖️texts/🟦️.ts")).refuseProcessV1(...input);
import {checkResourceLeaseControl,isSystemLeaseLinkV1,requireMaterializedPathV1,type LeaseWait,type ResourceAccess,type ResourceLeaseControl} from "./🎛️control/🟦️.ts";
export {checkResourceLeaseControl,watchResourceLeaseControlDeadlineV1,fenceMaterializedPathsV1,requireMaterializedPathV1,type LeaseWait,type ResourceAccess,type ResourceLeaseControl} from "./🎛️control/🟦️.ts";
import { waitingDelayV1 as delay } from "../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import queueSchema from "./🧬️schema/📜️ticket.json" with { type: "json" };
import {queueMayProceedV1,type QueueTicketV1} from "./⚖️fairness/🟦️.ts";
import {currentLeasePriorityV1,leaseRealArrivalV1,leaseTicketArrivalV1,type LeasePriorityV1} from "./📈️standing/🟦️.ts";
export {currentLeasePriorityV1,leasePriorityClassV1,leaseOvertakeWindowV1,leaseRealArrivalV1,leaseTicketArrivalV1,type LeasePriorityV1} from "./📈️standing/🟦️.ts";
import {ensureLeaseYieldWatcherV1,observeLeaseHolderV1,type LeaseHolderNoticeV1} from "./📈️standing/🤝️yield/🟦️.ts";
import {agentSessionAddressOfEnvironmentV1} from "../🪪️agent-sessions/🗄️store/🟦️.ts";
import waitPolicySchema from "./⏳️wait/🧬️schema/📜️policy.json" with { type: "json" };
import boundedSchema from "./📦️bounded/🧬️schema/🔣️.json" with {type:"json"};
import {compileJsonSchemaSubset} from "../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { freeSpaceSyncV1 } from "../🆓️free-space/🟦️.ts";

const retirementSchema=queueSchema.definitions.RetirementPolicy,custodySchema=queueSchema.definitions.ClaimCustodyPolicy,retentionSchema=queueSchema.definitions.ClaimRetentionPolicy,boundedContract=compileJsonSchemaSubset(boundedSchema);

export type LeaseOptions = ResourceAccess & {
  readonly directory: string;
  readonly signal: AbortSignal;
  readonly timeoutMs?: number;
  readonly onWait?: (progress: LeaseWait) => void | Promise<void>;
  readonly control?: ResourceLeaseControl;
  readonly maximumDatabaseBytes?: number;
  /** ⌛️ How long a request that must not wait for its resource (timeout 0) still waits for the directory admission gate, which is held only for microseconds. Only callers that wait for the gate as part of the queue protocol pass it; a real try-lock passes nothing and stays a single attempt. */
  readonly admissionPatienceMs?: number;
};
/** 📂️ Admits the actual database, every SQLite sidecar and optional queue ticket before any mkdir. */
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
 const pending=Promise.resolve().then(()=>{remaining(options,started);return continuation();});let failed=false,failure:unknown;
 try{await Promise.race([pending,interrupted]);remaining(options,started);}catch(error){failed=true;failure=error;}
 finally{const drained=await drainLeaseContinuation(pending,options.resource,failed?failure:undefined);if(drained.failed&&!failed){failed=true;failure=drained.failure;}options.signal.removeEventListener("abort",cancel);if(timer!==undefined)clearTimeout(timer);}
 if(failed)throw failure;
}
/** 🚰️ Waits for an accepted continuation to settle before the lease database may close, but at most `continuationDrainMilliseconds` after the wait ended: a continuation that settles inside the bound is joined (its outcome counts), one that does not is abandoned with its late outcome ignored and a notice in English and German, so a cancelled or expired wait never hangs on a callback that never returns. */
async function drainLeaseContinuation(pending:Promise<void>,resource:string,reason:unknown):Promise<{failed:boolean;failure?:unknown}>{
 const bound=waitPolicySchema.definitions.Policy.const.continuationDrainMilliseconds,late=Symbol("late");let timer:ReturnType<typeof setTimeout>|undefined;
 try{
  const outcome=await Promise.race([pending.then(()=>undefined,(error:unknown)=>({error})),new Promise<symbol>(done=>{timer=setTimeout(()=>done(late),bound);})]);
  if(outcome===late){
   void pending.catch(()=>undefined);
   const messages=waitPolicySchema.definitions.Messages.const.drainAbandoned,text=String((reason as {message?:unknown}|undefined)?.message??reason??"").slice(0,200);
   try{process.stderr.write((["en","de"] as const).map(language=>messages[language].replace("{resource}",resource.slice(-waitPolicy.maximumReportedResourceCharacters)).replace("{milliseconds}",String(bound)).replace("{reason}",text)).join("\n")+"\n");}catch{}
   return{failed:false};
  }
  return outcome===undefined?{failed:false}:{failed:true,failure:(outcome as {error:unknown}).error};
 }finally{if(timer!==undefined)clearTimeout(timer);}
}

async function waitProgress(options: LeaseOptions, progress: LeaseWait, started: number,cleanup?:CleanupLeaseScope): Promise<void> {
  remaining(options, started,cleanup);
  if(cleanup)noteAcceptedPhysicalCleanupProgressV1(cleanup.enrollment);
  const control=options.control,onWait=options.onWait;
  if (control) {
    await waitForLeaseContinuation(options,started,()=>control.publish(progress));
    await waitForLeaseContinuation(options,started,()=>control.yieldContinuation());
  }
  if (onWait) await waitForLeaseContinuation(options,started,()=>onWait(progress));
  else if (!control) console.error(`Waiting for ${options.mode} access to ${options.resource}`);
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
/** 🗑️ Captures actual custody before cleanup; copied leases, foreign controls and late captures grant nothing. */
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
/** 🎒️ Carries no release port and resolves only to its privately retained exclusive transaction. */
export type ResourceLeaseWriterProofV1=Readonly<{readonly [writerProofBrand]:true}>;
const writerProofs=new WeakMap<ResourceLeaseWriterProofV1,{readonly lease:ResourceLease;readonly issued:IssuedResourceLeaseV1}>();
/** ⏺️ Derives a nonreleasing proof only from this exact original writer and accepted control. */
export function captureResourceLeaseWriterProofV1(lease:ResourceLease,directory:string,resource:string,control:ResourceLeaseControl):ResourceLeaseWriterProofV1{
 requireResourceLeaseCustodyV1(lease,directory,resource,control);const issued=originalLeases.get(lease)!;if(issued.control!==control||issued.directory!==resolve(directory)||issued.resource!==resource)throw Error("Exact original writer binding required");const proof=Object.freeze({})as ResourceLeaseWriterProofV1;writerProofs.set(proof,{lease,issued});return proof;
}
/** 📝️ Requires the immutable original directory, logical key, guard path and the live holder control or an issued descendant of it. */
export function requireResourceLeaseWriterProofV1(proof:ResourceLeaseWriterProofV1,binding:Readonly<{directory:string;resource:string;path:string}>,control:ResourceLeaseControl):void{
 checkResourceLeaseControl(control);const held=writerProofs.get(proof),issued=held?.issued,holder=issued?.control;let related=holder===control;if(holder&&!related)try{checkResourceLeaseControl(holder);requireIssuedResourceControlDescendantV1(holder,control);related=true;}catch{}if(!held||issued?.phase!=="held"||issued.mode!=="exclusive"||!holder||!related||issued.signal!==holder.signal||issued.directory!==resolve(binding.directory)||issued.resource!==binding.resource||issued.path!==binding.path||held.lease.mode!==issued.mode||held.lease.resource!==issued.resource)throw Error("Exact live original writer proof required");
}
/** ☑️ Checks the original live lease loan under its exact accepted outer body after a nested owner has settled. */
export function requireAcceptedResourceLeaseRetainedBodyV1(loan:AcceptedResourceLeaseCleanupV1,enrollment:AcceptedPhysicalCleanupV1,binding:Readonly<{directory:string;resource:string;mode:"shared"|"exclusive"}>,bodyControl:ResourceLeaseControl,control:ResourceLeaseControl):void{requireAcceptedPhysicalCleanupControlV1(enrollment,bodyControl);requireIssuedResourceControlRefinementV1(bodyControl,control);checkResourceLeaseControl(bodyControl);checkResourceLeaseControl(control);const scope=cleanupLoans.get(loan),issued=scope?.original;if(!scope||scope.enrollment!==enrollment||issued?.phase!=="held"||scope.used||scope.active||issued.directory!==resolve(binding.directory)||issued.resource!==binding.resource||issued.mode!==binding.mode)throw Error("Actual retained outer-body lease required");cleanupLeaseIdentity(scope);}
/** ✅️ Checks still-retained exclusion only within its exact already accepted cleanup phase. */
export function requireAcceptedPhysicalCleanupWriterProofV1(proof:ResourceLeaseWriterProofV1,binding:Readonly<{directory:string;resource:string;path:string}>,bodyControl:ResourceLeaseControl,enrollment:AcceptedPhysicalCleanupV1):void{
 requireAcceptedPhysicalCleanupDescendantV1(enrollment,bodyControl);checkAcceptedPhysicalCleanupV1(enrollment);const held=writerProofs.get(proof),issued=held?.issued,facts=readAcceptedPhysicalCleanupV1(enrollment);if(!held||issued?.phase!=="held"||issued.mode!=="exclusive"||issued.control!==bodyControl||issued.signal!==bodyControl.signal||issued.directory!==resolve(binding.directory)||issued.resource!==binding.resource||issued.path!==binding.path||facts.resource!==binding.resource||held.lease.mode!==issued.mode||held.lease.resource!==issued.resource)throw Error("Exact retained cleanup writer proof required");
}
/** 💎️ Requires the exact live system transaction issued for this original catalog writer. */
export function requireResourceLeaseCustodyV1(lease:ResourceLease,directory:string,resource:string,control:ResourceLeaseControl):void{checkResourceLeaseControl(control);const issued=originalLeases.get(lease);if(issued?.phase!=="held"||issued.path!==resourceLeasePathV1(directory,resource)||issued.signal!==control.signal||issued.mode!=="exclusive"||lease.mode!==issued.mode||lease.resource!==resource)throw Error("Original live catalog writer custody required");}

type LeaseDatabase = ResourceLeaseDatabaseV1;
const APPLICATION_ID = 0x534d4c53;

/** 🗜️ Gives system SQLite a closed page limit before its first bootstrap write. */
export function resourceLeaseMaximumPagesV1(maximumDatabaseBytes:number):number{if(boundedContract.validate({version:1,maximumDatabaseBytes}).length||maximumDatabaseBytes%4096!==0)throw Error("Closed bootstrap SQLite capacity required");return maximumDatabaseBytes/4096;}

/** 🗃️ Uses the hosting runtime's system SQLite through the owned runtime connection class. */
function openDatabase(path: string, readonly=false,create=!readonly): Promise<LeaseDatabase> {return openResourceLeaseDatabaseV1(path,readonly,create);}
/** 🪟️ Opens the database of one acquisition; a Windows host that reports the file unopenable only because a scanner or a retiring peer still holds it is retried on the declared pauses (a try-lock waits at most those pauses once), and a fault that outlasts them is explained like any other unopenable store. */
async function openLeaseDatabaseWaitingV1(path:string,options:LeaseOptions,started:number,cleanup?:CleanupLeaseScope):Promise<LeaseDatabase>{
 for(let attempt=0;;attempt++){
  try{return await openDatabase(path);}
  catch(error){
   if(!leaseDatabaseUnopenableBusyV1(error))throw error;
   if(attempt>=leaseUnopenablePausesV1.length)throw explainUnopenableLeaseV1(error,path);
   const budget=remaining(options,started,cleanup);
   await delay(Math.max(1,Math.min(leaseUnopenablePausesV1[attempt]!,budget)),undefined,{signal:options.signal}).catch(failure=>{options.signal.throwIfAborted();throw failure;});
  }
 }
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

/** 🩺️ Names the physical cause of an unopenable lease file: a sibling create probe, the database state and the free space of its directory. */
function explainUnopenableLeaseV1(error:unknown,path:string):Error{
  const facts:string[]=[],probe=path+".probe";
  try{closeSync(openSync(probe,"wx"));unlinkSync(probe);facts.push("create probe ok");}catch(cause){facts.push("create probe "+((cause as NodeJS.ErrnoException).code??String(cause)));}
  try{const stat=lstatSync(path);facts.push(`database ${stat.size} bytes nlink ${stat.nlink}`);}catch(cause){facts.push("database "+((cause as NodeJS.ErrnoException).code??String(cause)));}
  try{const stat=statfsSync(dirname(path));facts.push(`free ${stat.bavail * stat.bsize} bytes ${stat.ffree} files`);}catch(cause){facts.push("statfs "+((cause as NodeJS.ErrnoException).code??String(cause)));}
  return new Error(`Resource lease database cannot be opened: ${path} (${facts.join(", ")})`,{cause:error});
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

/** ⏸️ Waits cancellably for shared or exclusive access; release only after the protected operation stops. */
async function acquireUngatedResourceLease(options: LeaseOptions,started:number,onUnclosed?:(database:LeaseDatabase)=>void,cleanup?:CleanupLeaseScope): Promise<ResourceLease> {
  options=Object.freeze({...options});
  options.signal.throwIfAborted();
  if (typeof options.resource !== "string" || !options.resource.length || options.resource.length > 4096 || options.resource.includes("\0")) throw refuseProcessV1("invalid", "leases.invalid-resource");
  if (!["shared", "exclusive"].includes(options.mode)) throw refuseProcessV1("invalid", "leases.invalid-access-mode");
  if (options.timeoutMs !== undefined && (!Number.isFinite(options.timeoutMs) || options.timeoutMs < 0)) throw refuseProcessV1("invalid", "leases.invalid-timeout");
  remaining(options, started,cleanup);
  if(options.resource===retentionSchema.properties.resource.const&&options.maximumDatabaseBytes!==undefined&&options.maximumDatabaseBytes>retentionSchema.properties.maximumDatabaseBytes.const)throw Error("Fixed retirement catalog capacity required");const maximumDatabaseBytes=options.maximumDatabaseBytes??(options.resource===retentionSchema.properties.resource.const?retentionSchema.properties.maximumDatabaseBytes.const:undefined),maximumPages=maximumDatabaseBytes===undefined?undefined:resourceLeaseMaximumPagesV1(maximumDatabaseBytes),path=databasePath(options.directory,options.resource);
  if(maximumPages!==undefined){try{if(lstatSync(path).size>maximumDatabaseBytes!)throw Error("Original bootstrap SQLite exceeds capacity");}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}
  const database = await openLeaseDatabaseWaitingV1(path, options, started, cleanup);
  let nextProgress = 0, acquired = false,failed=false,failure:unknown,unopenable=0;
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
          touchResourceLeaseUseV1(path,options.resource);issued.phase="closing";database.close();released = true;issued.phase="closed";
        } };touchResourceLeaseUseV1(path,options.resource);originalLeases.set(lease,issued);return lease;
      } catch (error) {
        try { database.exec("ROLLBACK"); } catch {}
        const code = error as { code?: string; errcode?: number; errno?: number };
        if (code.code !== "SQLITE_BUSY" && code.errcode !== 5 && code.errno !== 5 && !(leaseDatabaseUnopenableBusyV1(error) && ++unopenable <= leaseUnopenablePausesV1.length)) throw code.code === "SQLITE_CANTOPEN" ? explainUnopenableLeaseV1(error, path) : error;
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
/** 🕰️ Stamps the last use of an ordinary resource database while its holder still excludes the sweep, so the idle clock of the retirement is the file's own modification time. */
function touchResourceLeaseUseV1(path:string,resource:string):void{if(internalLeaseResource(resource))return;try{const row=lstatSync(path,{throwIfNoEntry:false});if(!row||Date.now()-row.mtimeMs<resourceLeaseRetirementIdlePolicyV1.useResolutionMilliseconds)return;const now=new Date();utimesSync(path,now,now);}catch{}}
function requireAdmission(gate:ResourceLease,directory:string,exclusive=false):void{const held=originalLeases.get(gate);if(held?.phase!=="held"||held.directory!==resolve(directory)||held.resource!==admissionResource||gate.resource!==admissionResource||gate.mode!==held.mode||exclusive&&held.mode!=="exclusive")throw Error("Original directory admission custody required");}
const admissionFinalizer=new ResourceLeaseAdmissionFinalizerV1<ResourceLease>();
const catalogAdmissions=new WeakMap<ResourceLease,ResourceLease>();
function protectAdmission(gate:ResourceLease,database:LeaseDatabase):void{admissionFinalizer.protect(gate,database);}
function releaseAdmission(gate:ResourceLease|undefined):void{admissionFinalizer.release(gate);}
/** 🚪️ Identifies the exact bounded admission request without granting or renewing custody. */
export interface ResourceLeaseAdmissionTimeoutV1{readonly version:1;readonly operation:"open"|"queue-publish"|"queue-sweep"|"claims-read"|"resource-retirement"|"claims-retirement";readonly directory:string;readonly resource:string;readonly requestedMode:ResourceAccess["mode"];readonly mode:ResourceAccess["mode"];readonly elapsedMs:number;readonly admissionElapsedMs:number;readonly timeoutMs:number|null;readonly remainingMsAtEntry:number|null;}
async function directoryAdmission(options:LeaseOptions,mode:ResourceAccess["mode"],started:number,cleanup?:CleanupLeaseScope,operation:ResourceLeaseAdmissionTimeoutV1["operation"]="open"):Promise<ResourceLease>{
 const entered=Date.now(),patience=!cleanup&&options.timeoutMs===0?options.admissionPatienceMs??0:0;admissionFinalizer.retry();try{return await acquireUngatedResourceLease({...options,...patience>0?{timeoutMs:patience}:{},resource:admissionResource,mode,maximumDatabaseBytes:retirementSchema.properties.maximumDatabaseBytes.const},patience>0?entered:started,undefined,cleanup);}catch(error){if(error instanceof Error&&error.message==="Resource lease timed out: "+admissionResource&&Object.isExtensible(error)&&!Object.hasOwn(error,"resourceLeaseAdmission")){const now=Math.max(entered,Date.now()),context:ResourceLeaseAdmissionTimeoutV1=Object.freeze({version:1,operation,directory:resolve(options.directory),resource:options.resource,requestedMode:options.mode,mode,elapsedMs:Math.max(0,now-started),admissionElapsedMs:now-entered,timeoutMs:options.timeoutMs??null,remainingMsAtEntry:options.timeoutMs===undefined?null:Math.max(0,options.timeoutMs-Math.max(0,entered-started))});Object.defineProperty(error,"resourceLeaseAdmission",{value:context,enumerable:true});}throw error;}
}
async function acquireUnderAdmission(options:LeaseOptions,gate:ResourceLease,started:number,cleanup?:CleanupLeaseScope):Promise<ResourceLease>{requireAdmission(gate,options.directory);return acquireUngatedResourceLease(options,started,database=>protectAdmission(gate,database),cleanup);}
function closeDatabase(database:LeaseDatabase,gate?:ResourceLease):void{try{database.close();}catch(error){if(gate)protectAdmission(gate,database);throw error;}}
function releaseUnderAdmission(lease:ResourceLease|undefined,gate:ResourceLease):void{if(!lease)return;try{lease.release();}catch(error){protectAdmission(gate,originalLeases.get(lease)!.database);throw error;}}

/** 🔑️ Fences every opener until actual identity SELECT or failed connection close settles. */
export function acquireResourceLease(options:LeaseOptions):Promise<ResourceLease>{return traceLeaseV1(options,()=>acquireOrderedResourceLeaseV1(options));}
/** 🆕️ Tries a fresh claim without waiting for a holder, allowing only the declared bounded directory admission under its original control. */
export function acquireFreshResourceLeaseV1(options:Omit<LeaseOptions,"timeoutMs"|"admissionPatienceMs">):Promise<ResourceLease>{return acquireResourceLease({...options,timeoutMs:0,admissionPatienceMs:leaseOrderPolicyV1().gate.admissionPatienceMilliseconds});}
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
/** 🔃️ Fences every opener until actual identity SELECT or failed connection close settles; a request that breaks the declared lock order or would wait while holding fails fast with a diagnostic. */
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
  if(!path)return admitResourceLeaseV1(options,await acquire());
  const startedMs=Date.now(),lease=await acquire(),acquiredMs=Date.now();
  if(acquiredMs-startedMs>=100)try{appendFileSync(path,JSON.stringify({version:1,resource:options.resource,mode:options.mode,startedMs,acquiredMs,pid:process.pid})+"\n");}catch{}
  return admitResourceLeaseV1(options,lease);
}
/** 🔤️ Admission guards of the lease layer, registered by name in a process-wide set: a guard receives every granted exclusive lease and every wait round of a queued exclusive request, and a guard that throws makes the holder yield the resource (a granted lease is released before the error leaves, a waiting ticket is withdrawn). */
export type ResourceLeaseAdmissionV1=Readonly<{directory:string;resource:string;mode:"shared"|"exclusive";phase:"admitted"|"waiting"}>;
const admissionGuardRegistryV1=Symbol.for("semio.resourceLeaseAdmissionGuards.v1");
function runAdmissionGuardsV1(options:Pick<LeaseOptions,"directory"|"resource"|"mode">,phase:ResourceLeaseAdmissionV1["phase"]):void{
  const guards=(globalThis as Record<symbol,Set<(admission:ResourceLeaseAdmissionV1)=>void>|undefined>)[admissionGuardRegistryV1];
  if(!guards?.size||options.mode!=="exclusive"||typeof options.resource!=="string")return;
  const admission:ResourceLeaseAdmissionV1=Object.freeze({directory:options.directory,resource:options.resource,mode:options.mode,phase});
  for(const guard of [...guards])guard(admission);
}
function admitResourceLeaseV1(options:Pick<LeaseOptions,"directory"|"resource"|"mode">,lease:ResourceLease):ResourceLease{
  if(options.mode==="exclusive")ensureLeaseYieldWatcherV1();
  try{runAdmissionGuardsV1(options,"admitted");}catch(error){finishResourceLeaseCleanupV1(true,error,[()=>lease.release()]);throw error;}
  return lease;
}
async function acquireResourceLeaseBody(options:LeaseOptions,cleanup?:CleanupLeaseScope):Promise<ResourceLease>{
 options=Object.freeze({...options});options.signal.throwIfAborted();if(options.resource===admissionResource)throw refuseProcessV1("refused", "leases.admission-resource-reserved");if(typeof options.resource!=="string"||!options.resource.length||options.resource.length>4096||options.resource.includes("\0"))throw refuseProcessV1("invalid", "leases.invalid-resource");if(!["shared","exclusive"].includes(options.mode))throw refuseProcessV1("invalid", "leases.invalid-access-mode");if(options.timeoutMs!==undefined&&(!Number.isFinite(options.timeoutMs)||options.timeoutMs<0))throw refuseProcessV1("invalid", "leases.invalid-timeout");const started=Date.now();remaining(options,started,cleanup);
 if(!cleanup&&options.timeoutMs!==0&&leaseOrderPolicyV1().gate.releaseWhileWaiting)return acquireWaitingOutsideGate(options,started);
 return acquireHoldingGate(options,started,cleanup);
}
/** 🐢️ Waits for a busy lease without holding the directory admission gate: each attempt takes the gate shared, tries once and gives the gate back, so retirement and every other opener are never blocked by a waiter. */
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
 try{gate=await directoryAdmission(options,"shared",started,cleanup);if(cleanup)cleanupLeaseIdentity(cleanup);lease=await acquireUnderAdmission(options,gate,started,cleanup);}catch(error){failed=true;failure=error;if(error instanceof Error&&error.message==="Resource lease timed out: "+admissionResource){failure=Error("Resource lease timed out: "+options.resource,{cause:error});const context=Object.getOwnPropertyDescriptor(error,"resourceLeaseAdmission");if(context)Object.defineProperty(failure,"resourceLeaseAdmission",context);}}
 try{releaseAdmission(gate);}catch(error){if(!failed){failed=true;failure=error;}try{lease?.release();}catch{if(gate&&lease)protectAdmission(gate,originalLeases.get(lease)!.database);}}
 if(failed)throw failure;return lease!;
}

/** 🚦️ One waiter's place in a resource's arrival queue: `<directory>/<resource-hash>.queue/<arrivedAtMs>-<pid>-<owner>`. */
export type QueuedLeaseOptions = LeaseOptions & { readonly owner: string; readonly sharedCohort?: boolean; readonly yielding?: boolean; readonly priority?: LeasePriorityV1 };

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

/** 🎫️ Names the independently held original queue custody before publishing any arrival ticket. */
export function resourceQueueClaimV1(directory:string,resource:string,ticket:string):Readonly<{directory:string;resource:string}>{queueTicketRequest(ticket);const claimDirectory=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const)),claimResource=custodySchema.properties.resourcePrefix.const+createHash("sha256").update(resource).digest("hex")+":"+ticket;resourceLeasePathV1(claimDirectory,claimResource);return Object.freeze({directory:claimDirectory,resource:claimResource});}

/** 📚️ Derives one canonical claim catalog per data namespace for every opener and retirement. */
export function resourceQueueCatalogPathV1(directory:string):string{return resourceLeasePathV1(directory,retentionSchema.properties.resource.const);}

function retirementCatalogDatabase(catalog:ResourceLease,directory:string,control:ResourceLeaseControl,gate?:ResourceLease):LeaseDatabase{const admission=catalogAdmissions.get(catalog)??gate;if(!admission)throw Error("Actual retirement catalog admission required");requireAdmission(admission,directory,true);if(originalLeases.get(admission)?.control!==control)throw Error("Exact retirement admission control required");requireResourceLeaseCustodyV1(catalog,directory,retentionSchema.properties.resource.const,control);const held=originalLeases.get(catalog);if(held?.control!==control)throw Error("Exact retirement catalog control required");return held.database;}
function persistRetirementContinuation(catalog:ResourceLease,directory:string,control:ResourceLeaseControl,kind:ResourceLeaseContinuationKindV1,after:string,gate?:ResourceLease):void{const database=retirementCatalogDatabase(catalog,directory,control,gate),issued=originalLeases.get(catalog)!,admission=catalogAdmissions.get(catalog)??gate!;issued.phase="transition";try{writeResourceLeaseContinuationV1(database,kind,after);checkResourceLeaseControl(control);requireAdmission(admission,directory,true);database.exec("COMMIT");checkResourceLeaseControl(control);requireAdmission(admission,directory,true);database.exec("BEGIN EXCLUSIVE");issued.phase="held";retirementCatalogDatabase(catalog,directory,control,gate);if(readResourceLeaseContinuationV1(database,kind)!==after)throw Error("Original committed retirement cursor required");}catch(error){issued.phase="transition";throw error;}}
async function queueClaimCatalog(options:LeaseOptions,mode:"shared"|"exclusive",started=Date.now(),operation:ResourceLeaseAdmissionTimeoutV1["operation"]="queue-publish"):Promise<ResourceLease>{
 let gate:ResourceLease|undefined,catalog:ResourceLease|undefined,failed=false,failure:unknown;try{gate=await directoryAdmission(options,mode,started,undefined,operation);catalog=await acquireUnderAdmission({...options,resource:retentionSchema.properties.resource.const,mode,maximumDatabaseBytes:retentionSchema.properties.maximumDatabaseBytes.const},gate,started);const held=catalog,admission=gate,combined:ResourceLease={resource:held.resource,mode:held.mode,release(){finishResourceLeaseCleanupV1(false,undefined,[()=>releaseUnderAdmission(held,admission),()=>releaseAdmission(admission)]);}};originalLeases.set(combined,originalLeases.get(held)!);catalogAdmissions.set(combined,admission);return combined;}catch(error){failed=true;failure=error;throw error;}finally{if(failed)finishResourceLeaseCleanupV1(failed,failure,[()=>{if(catalog&&gate)releaseUnderAdmission(catalog,gate);else catalog?.release();},()=>releaseAdmission(gate)]);}
}
type ClaimIdentity=Readonly<{path:string;resource:string;queueTicketPath:string;device:number;inode:number;allocatedBytes:number}>;
async function readClaimIdentity(directory:string,candidate:ResourceLeaseCandidateV1,control:ResourceLeaseControl,gate:ResourceLease):Promise<ClaimIdentity|undefined>{
 checkResourceLeaseControl(control);const claims=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const));requireAdmission(gate,claims);if(candidate.directory!==claims||!/^[a-f0-9]{64}\.sqlite$/u.test(basename(candidate.path)))throw Error("Exact queue claim candidate required");let database:LeaseDatabase|undefined,failed=false,failure:unknown;try{database=await openDatabase(candidate.path,true);checkResourceLeaseControl(control);database.exec("PRAGMA busy_timeout=0;PRAGMA trusted_schema=OFF;BEGIN");const resource=readOwnedResourceLeaseV1(database,claims,candidate.path);if(!resource)return;const identity=new RegExp("^"+custodySchema.properties.resourcePrefix.const+"([a-f0-9]{64}):("+queueSchema.definitions.Protocol.const.ticketPattern.slice(1,-1)+")$","u").exec(resource);if(!identity)throw Error("Original queue claim path identity required");queueTicketRequest(identity[2]!);checkResourceLeaseControl(control);requireResourceLeaseCandidateV1(candidate);return Object.freeze({path:candidate.path,resource,queueTicketPath:requireMaterializedPathV1(join(directory,identity[1]+".queue",identity[2]!)),device:candidate.file.dev,inode:candidate.file.ino,allocatedBytes:candidate.allocatedBytes});}catch(error){failed=true;failure=error;throw error;}finally{if(database)finishResourceLeaseCleanupV1(failed,failure,[()=>closeDatabase(database!,gate)]);}
}

/** 🔎️ Protects the complete read-only claim connection in exact parent-before-child admission custody. */
export async function readResourceQueueClaimIdentityV1(directory:string,name:string,control:ResourceLeaseControl):Promise<ClaimIdentity|undefined>{
 const started=Date.now(),options:LeaseOptions={directory,resource:retentionSchema.properties.resource.const,mode:"shared",signal:control.signal,control};let catalog:ResourceLease|undefined,gate:ResourceLease|undefined,failed=false,failure:unknown;try{catalog=await queueClaimCatalog(options,"shared",started,"claims-read");gate=await directoryAdmission({...options,directory:join(directory,custodySchema.properties.claimDirectory.const)},"shared",started,undefined,"claims-read");checkResourceLeaseControl(control);const claims=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const));requireAdmission(gate,claims);if(!/^[a-f0-9]{64}\.sqlite$/u.test(name))throw Error("Exact queue claim filename required");const candidate=captureResourceLeaseCandidateV1(claims,name);return candidate?await readClaimIdentity(directory,candidate,control,gate):undefined;}catch(error){failed=true;failure=error;throw error;}finally{finishResourceLeaseCleanupV1(failed,failure,[()=>releaseAdmission(gate),()=>catalog?.release()]);}
}

/** 📊️ Reports immediate physical databases examined and safely retired in one bounded turn. */
export type ResourceLeaseRetirementV1=Readonly<{examined:number;retired:number;allocatedBytes:number}>;
/** ⏱️ Bounds one idle retirement slice: the idle clock, the probes per slice and the patience for the directory gate. */
export type ResourceLeaseRetirementLimitsV1=Readonly<{idleMilliseconds?:number;nowMilliseconds?:number;maximumCandidates?:number;admissionPatienceMs?:number;files?:Partial<LeaseFilePortsV1>}>;
/** 💤️ A slice also reports the queue directories it removed, the entries it kept and whether the cursor wrapped (a full pass). */
export type ResourceLeaseIdleRetirementV1=ResourceLeaseRetirementV1&Readonly<{queues:number;kept:number;complete:boolean}>;
function databaseRejection(error:unknown):boolean{const value=error!==null&&typeof error==="object"?error as{code?:string;errcode?:number;errno?:number}:undefined;return ["SQLITE_ERROR","SQLITE_CORRUPT","SQLITE_NOTADB","SQLITE_SCHEMA"].includes(value?.code??"")||[1,11,17,26].includes((value?.errcode??value?.errno??-1)&255);}
function recoverRetirementHeadroom(error:unknown,directory:string,headroom:ReturnType<typeof import("./📦️metadata/🟦️.ts").retainResourceQueueMetadataHeadroomV1>|undefined):void{if(!headroom)throw error;const code=error!==null&&typeof error==="object"?(error as NodeJS.ErrnoException).code:undefined;if(code!=="ENOSPC"&&!(code==="EFAULT"&&statfsSync(directory).bavail<8))throw error;headroom.recover();}
async function retireResources(directory:string,control:ResourceLeaseControl,timeoutMs:number,resource?:string,limits:ResourceLeaseRetirementLimitsV1={}):Promise<ResourceLeaseIdleRetirementV1>{
 checkResourceLeaseControl(control);const started=Date.now(),options:LeaseOptions={directory,resource:"resource-retirement",mode:"exclusive",signal:control.signal,control,timeoutMs,...limits.admissionPatienceMs===undefined?{}:{admissionPatienceMs:limits.admissionPatienceMs}},idle=limits.idleMilliseconds??0,ports:LeaseFilePortsV1=Object.freeze({...leaseFilePortsV1,...limits.files}),now=limits.nowMilliseconds??started,slice=Math.min(limits.maximumCandidates??Infinity,retirementSchema.properties.maximumCandidatesPerTurn.const),tolerated=(error:unknown)=>["ENOENT","ENOTEMPTY"].includes((error as NodeJS.ErrnoException|undefined)?.code??"")||leaseFileBusyV1(error,ports.platform);let gate:ResourceLease|undefined,catalog:ResourceLease|undefined,headroom:ReturnType<typeof import("./📦️metadata/🟦️.ts").retainResourceQueueMetadataHeadroomV1>|undefined,failed=false,failure:unknown;let examined=0,retired=0,allocatedBytes=0,queues=0,kept=0,complete=true;
 try{gate=await directoryAdmission(options,"exclusive",started,undefined,"resource-retirement");catalog=await acquireUnderAdmission({...options,resource:retentionSchema.properties.resource.const,timeoutMs:0,maximumDatabaseBytes:retentionSchema.properties.maximumDatabaseBytes.const},gate,started);if(lstatSync(join(directory,metadataSchema.properties.filename.const),{throwIfNoEntry:false}))headroom=(await import("./📦️metadata/🟦️.ts")).retainResourceQueueMetadataHeadroomV1(directory,catalog,control,requireResourceLeaseCustodyV1);const names=resource===undefined?await resourceLeaseRetirementNamesV1(directory,control):[resourceLeasePathV1(directory,resource).slice(resolve(directory).length+1)],admissionPath=resourceLeasePathV1(directory,admissionResource),catalogPath=resourceQueueCatalogPathV1(directory);
 const selected=[...new Set(names.flatMap(name=>/^[a-f0-9]{64}\.(sqlite|queue)$/u.test(name)?[name.slice(0,64)+".sqlite"]:[]))].sort(),prior=resource===undefined?readResourceLeaseContinuationV1(retirementCatalogDatabase(catalog,directory,control,gate),"resources"):"";let scanned=0,at=resource===undefined?resourceLeaseContinuationStartV1(selected,prior):0,last="";for(;at<selected.length;at++){const name=selected[at]!;if(examined>=slice)break;last=name;remaining(options,started);if(++scanned%resourceLeaseRetirementScanLimitsV1.entriesPerYield===0){if(scanned===resourceLeaseRetirementScanLimitsV1.entriesPerYield||scanned%resourceLeaseRetirementScanLimitsV1.entriesPerProgress===0)await control.publish({resource:"resource-retirement-scan:"+scanned,mode:"exclusive",elapsedMs:Math.max(0,Date.now()-started)});await control.yieldContinuation();remaining(options,started);}const path=join(directory,name);if(path===admissionPath||path===catalogPath)continue;
  const queue=path.replace(/\.sqlite$/u,".queue"),waiting=lstatSync(queue,{throwIfNoEntry:false});if(waiting&&(!waiting.isDirectory()||waiting.isSymbolicLink()||readdirSync(queue).length>0)){kept++;continue;}
  const candidate=captureResourceLeaseCandidateV1(directory,name);
  if(!candidate){if(waiting&&lstatSync(path,{throwIfNoEntry:false})===undefined){if(idle>0&&now-waiting.mtimeMs<idle)kept++;else{examined++;try{requireAdmission(gate,directory,true);rmdirSync(queue);queues++;}catch(error){if(!tolerated(error))throw error;}}}continue;}
  if(idle>0&&now-Math.max(candidate.file.mtimeMs,waiting?.mtimeMs??0)<idle){kept++;continue;}
  examined++;let database:LeaseDatabase|undefined,probeFailed=false,probeFailure:unknown;
  try{requireAdmission(gate,directory,true);database=await openDatabase(path,false,false);remaining(options,started);database.exec("PRAGMA busy_timeout=0;PRAGMA trusted_schema=OFF");if(database.read("PRAGMA application_id")?.application_id!==APPLICATION_ID){kept++;continue;}database.exec("BEGIN EXCLUSIVE");const logical=readOwnedResourceLeaseV1(database,directory,path);if(logical===undefined||logical.startsWith(custodySchema.properties.resourcePrefix.const)||resource!==undefined&&logical!==resource){kept++;continue;}remaining(options,started);requireResourceLeaseCandidateV1(candidate);closeDatabase(database,gate);database=undefined;const remove=()=>{requireAdmission(gate!,directory,true);requireResourceLeaseCandidateV1(candidate);ports.unlink(path);};const removal=await settleLeaseFileV1(()=>{try{remove();}catch(error){recoverRetirementHeadroom(error,directory,headroom);remove();}},ports,control.signal);if(!removal.settled){kept++;continue;}retired++;allocatedBytes+=candidate.allocatedBytes;if(waiting)try{rmdirSync(queue);queues++;}catch(error){if(!tolerated(error))throw error;}
  }catch(error){probeFailed=true;probeFailure=error;if(!resourceLeaseBusyV1(error)&&!databaseRejection(error))throw error;probeFailed=false;kept++;}finally{if(database)finishResourceLeaseCleanupV1(probeFailed,probeFailure,[()=>closeDatabase(database!,gate)]);}
  await control.yieldContinuation();checkResourceLeaseControl(control);
 }complete=resource!==undefined||at>=selected.length;if(resource===undefined){const after=at===selected.length?"":last;if(after!==prior){headroom?.recover();persistRetirementContinuation(catalog,directory,control,"resources",after,gate);}}return Object.freeze({examined,retired,allocatedBytes,queues,kept,complete});
 }catch(error){failed=true;failure=error;throw error;}finally{finishResourceLeaseCleanupV1(failed,failure,[()=>headroom?.recheck(),()=>{if(catalog&&gate)releaseUnderAdmission(catalog,gate);else catalog?.release();},()=>releaseAdmission(gate)]);}
}

/** ♻️ Retires only authenticated idle immediate databases while exact directory and catalog writers remain retained. */
export async function retireResourceLeasesV1(directory:string,control:ResourceLeaseControl,timeoutMs=0):Promise<ResourceLeaseRetirementV1>{const{examined,retired,allocatedBytes}=await retireResources(resolve(directory),control,timeoutMs);return Object.freeze({examined,retired,allocatedBytes});}
/** ⏲️ Retires one bounded slice of resource databases that were idle for the declared period and removes their empty queue directories; waiters, holders and recent use keep an entry, and the persistent cursor continues the next slice where this one stopped. */
export function retireIdleResourceLeasesV1(directory:string,control:ResourceLeaseControl,timeoutMs:number,limits:ResourceLeaseRetirementLimitsV1):Promise<ResourceLeaseIdleRetirementV1>{return retireResources(resolve(directory),control,timeoutMs,undefined,limits);}
/** 🎯️ Retires one exact settled owner through the same protected close-to-unlink protocol. */
export function retireResourceLeaseV1(directory:string,resource:string,control:ResourceLeaseControl,timeoutMs=0):Promise<ResourceLeaseRetirementV1>{resourceLeasePathV1(directory,resource);if(typeof resource!=="string"||!resource.length||resource.length>4096||resource.includes("\0"))throw Error("Invalid retirement resource");return retireResources(resolve(directory),control,timeoutMs,resource);}

/** 🔧️ Retires settled claims under data admission, queue catalog and exact claims admission in fixed order. */
export async function retireResourceQueueClaimsV1(directory:string,control:ResourceLeaseControl,timeoutMs=0,limits:Pick<ResourceLeaseRetirementLimitsV1,"admissionPatienceMs">={}):Promise<ResourceLeaseRetirementV1>{
 checkResourceLeaseControl(control);const started=Date.now(),options:LeaseOptions={directory,resource:retentionSchema.properties.resource.const,mode:"exclusive",signal:control.signal,control,timeoutMs,...limits.admissionPatienceMs===undefined?{}:{admissionPatienceMs:limits.admissionPatienceMs}};let catalog:ResourceLease|undefined,gate:ResourceLease|undefined,headroom:ReturnType<typeof import("./📦️metadata/🟦️.ts").retainResourceQueueMetadataHeadroomV1>|undefined,failed=false,failure:unknown;let examined=0,retired=0,allocatedBytes=0;
 try{try{catalog=await queueClaimCatalog(options,"exclusive",started,"claims-retirement");gate=await directoryAdmission({...options,directory:join(directory,custodySchema.properties.claimDirectory.const),timeoutMs:0},"exclusive",started,undefined,"claims-retirement");}catch(error){if(resourceLeaseBusyV1(error))return Object.freeze({examined,retired,allocatedBytes});throw error;}headroom=(await import("./📦️metadata/🟦️.ts")).retainResourceQueueMetadataHeadroomV1(directory,catalog,control,requireResourceLeaseCustodyV1);
 const claims=requireMaterializedPathV1(join(directory,custodySchema.properties.claimDirectory.const)),admissionName=basename(resourceLeasePathV1(claims,admissionResource)),names=(await resourceLeaseRetirementNamesV1(claims,control)).filter(name=>/^[a-f0-9]{64}\.sqlite$/u.test(name)&&name!==admissionName),prior=readResourceLeaseContinuationV1(retirementCatalogDatabase(catalog,directory,control),"claims");let scanned=0,at=resourceLeaseContinuationStartV1(names,prior),last="";
 for(;at<names.length;at++){const name=names[at]!;if(examined>=retentionSchema.properties.maximumClaimsPerTurn.const)break;last=name;remaining(options,started);if(++scanned%resourceLeaseRetirementScanLimitsV1.entriesPerYield===0){if(scanned===resourceLeaseRetirementScanLimitsV1.entriesPerYield||scanned%resourceLeaseRetirementScanLimitsV1.entriesPerProgress===0)await control.publish({resource:"claim-retirement-scan:"+scanned,mode:"exclusive",elapsedMs:Math.max(0,Date.now()-started)});await control.yieldContinuation();remaining(options,started);}requireAdmission(gate,claims,true);const admitted=captureResourceLeaseCandidateV1(claims,name);if(!admitted)continue;examined++;let held:ResourceLease|undefined,probeFailed=false,probeFailure:unknown;try{const row=await readClaimIdentity(directory,admitted,control,gate);if(!row)continue;held=await acquireUnderAdmission({directory:dirname(row.path),resource:row.resource,mode:"exclusive",signal:control.signal,control,timeoutMs:0,maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const},gate,started);remaining(options,started);const candidate=captureResourceLeaseCandidateV1(dirname(row.path),name);if(!candidate||candidate.file.dev!==row.device||candidate.file.ino!==row.inode)throw Error("Original queue claim inode changed");const remove=()=>{try{unlinkSync(row.queueTicketPath);}catch(error){if((error as NodeJS.ErrnoException|undefined)?.code!=="ENOENT")throw error;}releaseUnderAdmission(held,gate!);held=undefined;requireAdmission(gate!,dirname(row.path),true);requireResourceLeaseCandidateV1(candidate);unlinkSync(row.path);};try{remove();}catch(error){recoverRetirementHeadroom(error,directory,headroom);remove();}retired++;allocatedBytes+=row.allocatedBytes;}catch(error){probeFailed=true;probeFailure=error;if(!resourceLeaseBusyV1(error)&&!databaseRejection(error))throw error;}finally{finishResourceLeaseCleanupV1(probeFailed&&!resourceLeaseBusyV1(probeFailure)&&!databaseRejection(probeFailure),probeFailure,[()=>{if(held)releaseUnderAdmission(held,gate!);}]);}await control.yieldContinuation();checkResourceLeaseControl(control);
 }const after=at===names.length?"":last;if(after!==prior){headroom?.recover();persistRetirementContinuation(catalog,directory,control,"claims",after);}return Object.freeze({examined,retired,allocatedBytes});
 }catch(error){failed=true;failure=error;throw error;}finally{finishResourceLeaseCleanupV1(failed,failure,[()=>headroom?.recheck(),()=>releaseAdmission(gate),()=>catalog?.release()]);}
}

/** ⏳️ Closed poll, sweep and progress cadence declared by `⏳️wait/🧬️schema/📜️policy.json`. */
const waitPolicy=waitPolicySchema.definitions.Policy.const;
type QueuedTurn={readonly options:QueuedLeaseOptions;readonly queue:string;readonly ticket:string;readonly claim:ResourceLease;inner?:ResourceLease;consumed:boolean};
const queuedTurns=new WeakMap<ResourceLease,QueuedTurn>();
type QueueWaitInfo={version:1;position:number;queued:number;holderMode:"shared"|"exclusive";holderPid:number;holderSeconds:number;resource:string}&Partial<LeaseHolderNoticeV1>;
type QueueRow={pid:number;mode:"shared"|"exclusive";sharedCohort:boolean;yielding:boolean;priority:LeasePriorityV1};

function parseQueueTicketRow(name:string,text:string):QueueRow|undefined{
  let row:Record<string,unknown>|undefined;try{row=JSON.parse(text);}catch{return undefined;}
  if(row===null||typeof row!=="object"||Object.keys(row).filter(key=>key!=="yielding"&&key!=="priority").sort().join(",")!=="mode,pid,sharedCohort,version"||row.version!==1||row.pid!==queueTicketRequest(name).pid||(row.mode!=="shared"&&row.mode!=="exclusive")||typeof row.sharedCohort!=="boolean"||"yielding" in row&&row.yielding!==true||"priority" in row&&row.priority!=="interactive"&&row.priority!=="background")return undefined;
  return{pid:row.pid as number,mode:row.mode,sharedCohort:row.sharedCohort,yielding:row.yielding===true,priority:(row.priority as LeasePriorityV1|undefined)??"agent"};
}
function queueTicketRow(queue:string,name:string):QueueRow|undefined{
  let text:string;try{text=readFileSync(join(queue,name),"utf8");}catch{return undefined;}
  return parseQueueTicketRow(name,text);
}
/** 🧮️ Reads one earlier ticket for the turn decision: a released ticket counts for nothing, a malformed one blocks like a young exclusive writer. */
function queueAheadV1(queue:string,name:string,now:number):QueueTicketV1|undefined{
  let text:string;try{text=readFileSync(join(queue,name),"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
  const request=queueTicketRequest(name),row=parseQueueTicketRow(name,text);
  return Object.freeze(row?{pid:row.pid,mode:row.mode,sharedCohort:row.sharedCohort,yielding:row.yielding,arrivedAtMs:leaseRealArrivalV1(row.priority,request.arrivedAtMs)}:{pid:request.pid,mode:"exclusive" as const,sharedCohort:false,yielding:false,arrivedAtMs:now});
}
function queueWaitInfo(options:QueuedLeaseOptions,queue:string,live:readonly string[],ticket:string,queued:number):QueueWaitInfo|undefined{
  const position=live.indexOf(ticket)+1;if(position<2)return undefined;
  const head=live[0]!,row=queueTicketRow(queue,head);if(!row)return undefined;
  const limit=waitPolicy.maximumReportedResourceCharacters,resource=options.resource.length>limit?"…"+options.resource.slice(-(limit-1)):options.resource;
  return{version:1,position,queued:Math.max(queued,position),holderMode:row.mode,holderPid:row.pid,holderSeconds:Math.max(0,Math.floor((Date.now()-leaseRealArrivalV1(row.priority,arrival(head)))/1000)),resource};
}
/** 👀️ Adds the holder's state, phase, counters and time left to a queue notice and lets the waiter ask a stale or stuck holder to yield (never fails the wait). */
async function observedQueueWait(options:QueuedLeaseOptions,queue:string,head:string,info:QueueWaitInfo):Promise<QueueWaitInfo>{
  try{
    const row=queueTicketRow(queue,head);if(!row)return info;
    const notice=await observeLeaseHolderV1({directory:resolve(options.directory),resource:options.resource,headPid:row.pid,headMode:row.mode,headArrivalMs:leaseRealArrivalV1(row.priority,arrival(head))});
    return notice?{...info,...notice}:info;
  }catch{return info;}
}
/** 🪦️ Retires one predecessor ticket only when its independent kernel claim proves its requester gone. */
async function reapQueueTicket(options:QueuedLeaseOptions,queue:string,name:string,pendingOptions:()=>LeaseOptions):Promise<boolean>{
  const sweepCatalog=await queueClaimCatalog(pendingOptions(),"shared",Date.now(),"queue-sweep");let retired:ResourceLease|undefined;
  try{retired=await acquireResourceLease({...resourceQueueClaimV1(options.directory,options.resource,name),mode:"exclusive",signal:options.signal,control:options.control,timeoutMs:0,maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});rmSync(join(queue,name),leaseRemovalOptionsV1());return true;}
  catch(error){if(!String(error).includes("Resource lease timed out"))throw error;return false;}
  finally{retired?.release();sweepCatalog.release();}
}
function wrapQueuedLease(lease:ResourceLease,turn:QueuedTurn):ResourceLease{
  const queued:ResourceLease={resource:lease.resource,mode:lease.mode,release(){
    if(turn.consumed)return;
    try{lease.release();}finally{try{rmSync(join(turn.queue,turn.ticket),leaseRemovalOptionsV1());}finally{turn.claim.release();}}
  }};
  originalLeases.set(queued,originalLeases.get(lease)!);queuedTurns.set(queued,turn);turn.inner=lease;return queued;
}

/** 📥️ Serves original kernel-held tickets in arrival order; an exclusive SQLite claim alone proves abandoned custody. */
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
  if(options.yielding!==undefined&&(typeof options.yielding!=="boolean"||options.yielding&&options.mode!=="exclusive"))throw refuseProcessV1("invalid", "leases.yielding-requires-exclusive");
  if (!/^[A-Za-z0-9._-]{1,64}$/u.test(options.owner)) throw refuseProcessV1("invalid", "leases.invalid-queue-owner");
  const started = Date.now(); remaining(options, started);
  const pendingOptions=():LeaseOptions=>({...options,timeoutMs:options.timeoutMs===undefined?undefined:Math.max(0,remaining(options,started))});
  if(options.control&&claimSweepDueV1(lastClaimSweep,resolve(options.directory),Date.now(),claimSweepIntervalV1(leaseOrderPolicyV1().gate,leaseStoreFreeRatioV1(options.directory)),options.timeoutMs))await retireResourceQueueClaimsV1(options.directory,options.control,0);
  remaining(options,started);
  const priority=options.priority??currentLeasePriorityV1(agentSessionAddressOfEnvironmentV1()!==undefined),ticket = resourceQueueTicketV1({arrivedAtMs:leaseTicketArrivalV1(priority,started),pid:process.pid,owner:options.owner});
  const admitted = resourceLeasePathV1(options.directory, options.resource, ticket);
  const queue = databasePath(options.directory, options.resource).replace(/\.sqlite$/u, ".queue");
  if (queue !== admitted.replace(/\.sqlite$/u, ".queue")) throw Error("Resource queue materialized path changed");
  const patience=leaseOrderPolicyV1().gate.admissionPatienceMilliseconds;
  const catalog=await queueClaimCatalog({...pendingOptions(),admissionPatienceMs:patience},"shared");let claim:ResourceLease;
  try{claim=await acquireResourceLease({...pendingOptions(),admissionPatienceMs:patience,...resourceQueueClaimV1(options.directory,options.resource,ticket),mode:"shared",maximumDatabaseBytes:custodySchema.properties.maximumDatabaseBytes.const});try{remaining(options,started);mkdirSync(queue,{recursive:true});writeFileSync(join(queue,ticket),JSON.stringify({version:1,pid:process.pid,mode:options.mode,sharedCohort:options.sharedCohort===true,...options.yielding===true?{yielding:true}:{},...priority==='agent'?{}:{priority}}),{flag:"wx"});}catch(error){claim.release();throw error;}}finally{catalog.release();}
  return awaitQueuedTurn(options,started,queue,ticket,claim,guard);
}

const lastClaimSweep=new Map<string,number>();
/** 🧹️ Whether this process may start another retirement sweep of settled queue claims. The sweep takes the directory admission gate exclusively for a moment, so it never runs for a caller that must not wait (timeout 0) and runs at most once per `gate.claimSweepIntervalMilliseconds` per directory; a skipped caller does not move the clock. */
export function claimSweepDueV1(sweeps:Map<string,number>,directory:string,nowMilliseconds:number,intervalMilliseconds:number,timeoutMs:number|undefined):boolean{
  if(timeoutMs===0)return false;
  const last=sweeps.get(directory);
  if(intervalMilliseconds>0&&last!==undefined&&nowMilliseconds-last<intervalMilliseconds)return false;
  sweeps.set(directory,nowMilliseconds);return true;
}
/** 🌡️ The fraction of the lease store's filesystem that is still free, or `undefined` when it cannot be measured (the call fails or reports no blocks); every caller treats an unmeasured store as under pressure, never as free. */
export function leaseStoreFreeRatioV1(directory:string):number|undefined{const space=freeSpaceSyncV1(directory);return space?.totalBytes?space.availableBytes/space.totalBytes:undefined;}
/** ⏰️ The sweep interval of the gate policy: the pressure interval while the free fraction of the lease store is below the policy ratio, the ordinary interval otherwise; an unknown fraction counts as nothing free, so a policy ratio of 0 still switches the pressure off. */
export function claimSweepIntervalV1(gate:Readonly<{claimSweepIntervalMilliseconds:number;claimSweepPressureFreeRatio:number;claimSweepPressureIntervalMilliseconds:number}>,freeRatio:number|undefined):number{return (freeRatio??0)<gate.claimSweepPressureFreeRatio?Math.min(gate.claimSweepIntervalMilliseconds,gate.claimSweepPressureIntervalMilliseconds):gate.claimSweepIntervalMilliseconds;}
/** 📬️ Whether an exclusive request is queued on a lease: only then can a further shared request of a holder be stuck behind a writer, so only then does it skip the queue; otherwise it queues normally and stays upgradeable. */
function exclusiveTicketQueued(directory:string,resource:string):boolean{
  try{const queue=resourceLeasePathV1(directory,resource).replace(/\.sqlite$/u,".queue");for(const name of readdirSync(queue))if(queueTicketRow(queue,name)?.mode==="exclusive")return true;}catch{}
  return false;
}
/** ⬆️ Converts a held queued shared lease into an exclusive one at its original arrival position; the shared hold is consumed, so a failed upgrade holds nothing and concurrent upgraders cannot deadlock each other. */
export async function upgradeQueuedResourceLease(lease:ResourceLease,limits:{readonly timeoutMs?:number}={}):Promise<ResourceLease>{
  const turn=queuedTurns.get(lease),issued=originalLeases.get(lease);
  if(!turn||turn.consumed||!turn.inner||lease.mode!=="shared"||issued?.phase!=="held")throw Error("Live queued shared lease required for upgrade");
  if(limits.timeoutMs!==undefined&&(!Number.isFinite(limits.timeoutMs)||limits.timeoutMs<0))throw refuseProcessV1("invalid", "leases.invalid-timeout");
  const{sharedCohort:_cohort,timeoutMs:_timeout,...base}=turn.options,options:QueuedLeaseOptions=Object.freeze({...base,mode:"exclusive",...(limits.timeoutMs===undefined?{}:{timeoutMs:limits.timeoutMs})});
  const started=Date.now();remaining(options,started);forgetLeaseV1(lease);const guard=orderGuard(options,true);
  const name=join(turn.queue,turn.ticket),temporary=join(dirname(turn.queue),turn.ticket+".u");
  try{const kept=queueTicketRow(turn.queue,turn.ticket)?.priority;writeFileSync(temporary,JSON.stringify({version:1,pid:process.pid,mode:"exclusive",sharedCohort:false,...kept===undefined||kept==="agent"?{}:{priority:kept}}),{flag:"wx"});const moved=await settleLeaseFileV1(()=>leaseFilePortsV1.rename(temporary,name));if(!moved.settled)throw moved.error;}catch(error){rmSync(temporary,leaseRemovalOptionsV1());throw error;}
  turn.consumed=true;
  try{turn.inner.release();}catch(error){try{rmSync(name,leaseRemovalOptionsV1());}finally{turn.claim.release();}throw error;}
  let upgraded:ResourceLease;try{upgraded=await awaitQueuedTurn(options,started,turn.queue,turn.ticket,turn.claim,guard);}catch(error){guard?.finish();throw error;}
  guard?.hold(upgraded);return admitResourceLeaseV1(options,upgraded);
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
      const now=Date.now(),before=live.slice(0,live.indexOf(ticket)),requester={pid:process.pid,mode:options.mode,sharedCohort:options.sharedCohort===true,yielding:options.yielding===true,holdsShared:options.mode==="shared"&&before.length>0&&leaseOrderPolicyV1().fairness.reentrantShared!==false&&holdsSharedInProcessV1(resolve(options.directory),options.resource)},
        ahead=await retryInterruptedFilesystemRead(options.signal,()=>before.map(name=>queueAheadV1(queue,name,now)).filter((row):row is QueueTicketV1=>row!==undefined),options.control);
      remaining(options, started);
      if (live.includes(ticket) && queueMayProceedV1(requester,ahead,now,leaseOrderPolicyV1().fairness.writerPreferenceMilliseconds??Infinity)) {
        const acquire = (): Promise<ResourceLease> => acquireResourceLeaseBody({ ...options, admissionPatienceMs: leaseOrderPolicyV1().gate.admissionPatienceMilliseconds, timeoutMs: options.timeoutMs === undefined ? undefined : Math.max(0, options.timeoutMs - (Date.now() - started)) });
        const lease = await (guard ? trackedAcquisition.run({ guard, directory: resolve(options.directory), resource: options.resource }, acquire) : acquire());
        try { remaining(options, started); } catch (error) { lease.release(); throw error; }
        return wrapQueuedLease(lease,{options,queue,ticket,claim,consumed:false});
      }
      const elapsedMs = Date.now() - started;
      if (remaining(options, started) <= 0) throw new Error(`Resource lease timed out: ${options.resource}`);
      await guard?.contention(elapsedMs);
      if (elapsedMs >= nextProgress) {
        runAdmissionGuardsV1(options,"waiting");
        const progress = { resource: options.resource, mode: options.mode, elapsedMs },waitInfo=queueWaitInfo(options,queue,live,ticket,names.length),info=waitInfo?await observedQueueWait(options,queue,live[0]!,waitInfo):undefined,control=options.control;
        await waitProgress(options, progress, started);
        if(info&&control)await waitForLeaseContinuation(options,started,()=>control.publish({resource:"lease-queue:"+JSON.stringify(info),mode:options.mode,elapsedMs:0}));
        nextProgress = elapsedMs + waitPolicy.progressIntervalMilliseconds;
      }
      await delay(Math.max(1, Math.min(poll, remaining(options, started))), undefined, { signal: options.signal }).catch(error=>{options.signal.throwIfAborted();throw error;});
      poll=Math.min(waitPolicy.pollMaximumMilliseconds,Math.ceil(poll*waitPolicy.pollGrowthNumerator/waitPolicy.pollGrowthDenominator));
      remaining(options, started);
    }
  } catch (error) {
    try{rmSync(join(queue, ticket), leaseRemovalOptionsV1());}finally{claim.release();}
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
/** 🔐️ Orders a resource set by declared lock rank then name, takes shared and exclusive access through the fair arrival queue (readers join cohorts, writers are not starved, a task re-entering a lease it already holds bypasses the queue) and releases in reverse order on every exit. */
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
