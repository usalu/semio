import type {ScriptInvocation} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readJavascriptDependencySelectedRetirementCommandV1,type JavascriptDependencySelectedRetirementCommandV1,type JavascriptDependencySelectedRetirementSnapshotV1} from "../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts";
import {javascriptInstallSchemaV1} from "../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧱️schemas/🟦️.ts";
import {planSelectedRepositoryDependencyGenerationsV1,maintainSelectedRepositoryDependencyGenerationV1,repositoryCacheImagePublicationOptionsV1} from "../../🟦️.ts";
import {retryWhileDependencyStoreBusyV1,dependencyStoreWaitPortsForV1,type DependencyStoreWaitPortsV1} from "../💰️funding/🟦️.ts";
import {refuseCachingV1,sayCachingV1} from "../../../../../../../📖️texts/🟦️.ts";
import {isRepoErrorV1,type RepoErrorV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {observeRepositoryDependencyMaintenanceParentV1} from "../../📸️observation/🟦️.ts";
import retirementSchema from "./🧬️schema/🔣️.json" with {type:"json"};

import {materializeSchemaV1,schemaViewV1} from "../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🧩️materialize/🟦️.ts";
import type {ManagerDescriptorV1} from "../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🟦️.ts";

const selectedRetirementSchema=materializeRepositoryDependencyRetirementSchemaV1(javascriptInstallSchemaV1.$defs.ManagerProtocol.manager.descriptor),retirementWireSchema={...selectedRetirementSchema,$defs:{...selectedRetirementSchema.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}},retirementProtocol=selectedRetirementSchema.$defs.Protocol.const;
/** 🧩️ Resolves exact payload metadata names through the selected manager descriptor. */
function materializeRepositoryDependencyRetirementSchemaV1(descriptor:ManagerDescriptorV1):typeof retirementSchema{return materializeSchemaV1(retirementSchema,schemaViewV1(descriptor)) as typeof retirementSchema;}
interface Candidate{readonly prefix:string;readonly generation:string|null;readonly status:"active"|"inactive"|"unqualified"|"unready"|"retained-unready"|"unready-metadata";readonly receiptFingerprint:string|null;readonly rootStamp:string;readonly roles:readonly string[];readonly ownerReferences:readonly Readonly<{scope:string;generation:string}>[]}
interface Plan{readonly artifact:JavascriptDependencySelectedRetirementCommandV1["artifact"];readonly current:Readonly<{sha256:string;scopes:readonly Readonly<{generation:string}>[]}>;readonly generations:Readonly<{names:readonly string[]}>;readonly candidates:readonly Candidate[]}
interface Settlement{readonly selected:JavascriptDependencySelectedRetirementSnapshotV1;readonly retired:boolean;readonly operation:"observe"|"retire";readonly journalSha256:string;readonly journalBytes:number}

/** 🔌️ The owner calls and the wait ports of the retirement driver, so that a store in any state can be rehearsed without the native owners. */
export interface RetirementOwnersV1{observe(invocation:ScriptInvocation,control:ResourceLeaseControl):Promise<Readonly<{parent:unknown}>>;plan(invocation:ScriptInvocation,control:ResourceLeaseControl,value:unknown):Promise<unknown>;maintain(invocation:ScriptInvocation,control:ResourceLeaseControl,value:unknown):Promise<unknown>;waits(invocation:ScriptInvocation,control:ResourceLeaseControl):DependencyStoreWaitPortsV1}

/** 🏛️ The genuine owners and the wait ports of the dependency store with the bounded retirement wait of the schema; a rehearsal binds a model of a store in any state instead. */
export const retirementOwnersV1:RetirementOwnersV1=Object.freeze({observe:observeRepositoryDependencyMaintenanceParentV1,plan:planSelectedRepositoryDependencyGenerationsV1,maintain:maintainSelectedRepositoryDependencyGenerationV1,waits:(invocation:ScriptInvocation,control:ResourceLeaseControl)=>{const options=repositoryCacheImagePublicationOptionsV1(invocation,control,"dependencies");return Object.freeze({...dependencyStoreWaitPortsForV1(()=>({directory:options.leaseDirectory,resource:"native-fixed-image:"+options.directory})),limitMilliseconds:retirementProtocol.maximumBusyWaitMilliseconds});}});

/** 🔎️ Whether an owner answered that the dependency store parent is held by another process: exactly `{acquired:false}`, any other shape is data to be validated, never a wait. */
function busyRepositoryDependencyOwnerResultV1(value:unknown):boolean{return typeof value==="object"&&value!==null&&(value as{acquired?:unknown}).acquired===false&&Object.keys(value).length===1;}

/** ♻️ Retires only proven obsolete physical prefixes through genuine fresh plan, observe and retire owners, preserving the retained whole charge. A store held by another process is waited for with a progress line that names the holder and, past the bounded retirement wait, the rest is deferred to the next setup; every phase replans from the store, so an interrupted or half-retired store resumes where it stopped. */
export async function retireObsoleteRepositoryDependencyGenerationsV1(invocation:ScriptInvocation,control:ResourceLeaseControl,owners:RetirementOwnersV1=retirementOwnersV1):Promise<readonly string[]>{
 const retired:string[]=[],started=performance.now(),check=()=>{checkResourceLeaseControl(control);control.signal.throwIfAborted();},waits=owners.waits(invocation,control),acquire=(call:()=>Promise<unknown>):Promise<unknown>=>retryWhileDependencyStoreBusyV1(control,async()=>{const value=await call();return busyRepositoryDependencyOwnerResultV1(value)?Object.freeze({acquired:false as const}):Object.freeze({acquired:true as const,value});},waits);
 let moved=0;
 try{
  for(;;){
   try{
   check();const plan=readRepositoryDependencyRetirementPlanV1(await acquire(async()=>owners.plan(invocation,control,{version:1,purpose:"plan-current-dependency-generations",selectedPrefix:null,parent:(await owners.observe(invocation,control)).parent}))),candidate=selectRepositoryDependencyRetirementCandidateV1(plan);check();
   if(candidate===null)return Object.freeze(retired);if(retired.includes(candidate.prefix))throw Error("Retired dependency generation reappeared");if(retired.length>=retirementProtocol.maximumRetirements)throw Error("Dependency obsolete retirement count exhausted");
  const selection=candidate.status==="inactive"||candidate.status==="retained-unready"?{kind:candidate.status==="inactive"?"inactive-generation"as const:"retained-unready"as const,prefix:candidate.prefix,generation:candidate.generation!,receiptFingerprint:candidate.receiptFingerprint!,rootStamp:candidate.rootStamp,currentBindingsSha256:plan.current.sha256}:{kind:candidate.status==="unready"?"unready-generation"as const:candidate.status==="unready-metadata"?"unready-metadata"as const:"empty-generation"as const,prefix:candidate.prefix,rootStamp:candidate.rootStamp,currentBindingsSha256:plan.current.sha256};
   sayCachingV1("dependency-store.retiring",{prefix:candidate.prefix,retired:retired.length});
   const snapshot=readRepositoryDependencyRetirementSettlementV1(await acquire(async()=>owners.maintain(invocation,control,readJavascriptDependencySelectedRetirementCommandV1({version:1,operation:"observe",selection,parent:(await owners.observe(invocation,control)).parent,artifact:plan.artifact,expected:null}))),"observe",selection,plan.artifact);check();
   readRepositoryDependencyRetirementSettlementV1(await acquire(async()=>owners.maintain(invocation,control,readJavascriptDependencySelectedRetirementCommandV1({version:1,operation:"retire",selection,parent:(await owners.observe(invocation,control)).parent,artifact:plan.artifact,expected:snapshot.selected}))),"retire",selection,plan.artifact);retired.push(candidate.prefix);moved=0;await control.publish({resource:retirementProtocol.progressResource+":"+retired.length,mode:"exclusive",elapsedMs:performance.now()-started});await control.yieldContinuation();check();
   }catch(error){
    const found=bindingsMovedV1(error);if(found===null)throw error;
    if(++moved>retirementProtocol.maximumBindingsReplans){sayCachingV1("dependency-store.retirement-moved",{limit:retirementProtocol.maximumBindingsReplans,retired:retired.length},"warn");return Object.freeze(retired);}
    sayCachingV1("dependency-store.bindings-replanned",{planned:found.values.planned,current:found.values.current,attempt:moved+1,limit:retirementProtocol.maximumBindingsReplans+1},"warn");
   }
  }
 }catch(error){
  if(rosterRefusalV1(error)){sayCachingV1("dependency-store.roster-unknown",{detail:String((error as Error).message).replace(/^.*?required:?\s*/u,"").slice(0,200),retired:retired.length},"warn");return Object.freeze(retired);}
  if(isRepoErrorV1(error)&&error.key==="caching.dependency-store.busy-timeout"){sayCachingV1("dependency-store.retirement-deferred",{seconds:Number(error.values.seconds),retired:retired.length},"warn");return Object.freeze(retired);}
  throw error;
 }
}

/** 🧾️ Whether a refusal says that the dependency store holds a name that is no generation or capsule: retiring obsolete generations is best effort, so it is told and left for a later setup instead of failing the whole stage. */
export function rosterRefusalV1(error:unknown):boolean{return error instanceof AggregateError?error.errors.some(rosterRefusalV1):error instanceof Error&&/Exact known current dependency generation roster required/u.test(error.message);}

/** 🌀️ Returns the stale binding refusal when every reported failure permits a fresh retirement plan. */
export function bindingsMovedV1(error:unknown):RepoErrorV1|null{
 if(isRepoErrorV1(error))return error.key==="caching.dependency-store.bindings-moved"?error:null;
 if(error instanceof AggregateError){
  let moved:RepoErrorV1|null=null;
  for(const part of error.errors){
   const found=bindingsMovedV1(part);
   if(found===null)return null;
   moved??=found;
  }
  return moved;
 }
 return null;
}

/** 📋️ Accepts only complete acquired qualified plan DATA; a busy or malformed observation never becomes an empty plan. */
function readRepositoryDependencyRetirementPlanV1(value:unknown):Plan{
 const text=JSON.stringify(value),bytes=typeof text==="string"?Buffer.byteLength(text):0,limit=retirementProtocol.maximumPlanBytes,held=typeof value==="object"&&value!==null&&(value as{acquired?:unknown}).acquired===false,fail=(reason:string,violation:string):never=>{throw refuseCachingV1("failed","dependency-store.retirement-plan-incomplete",{reason,bytes,violation});};
 if(held)fail("not-acquired","the store was held by another process");if(typeof text!=="string")fail("malformed","no JSON answer");if(bytes>limit)fail("oversized","more than "+limit+" bytes");
 const violations=validateJsonSchemaSubset(retirementWireSchema.$defs.AcquiredPlan as never,value,retirementWireSchema as never);if(violations.length)fail("malformed",String(violations[0]).slice(0,200));return(value as{value:Plan}).value;
}

/** 🌳️ Orders known obsolete references without treating graph metadata as deletion authority. */
function selectRepositoryDependencyRetirementCandidateV1(plan:Plan):Candidate|null{
 const prefixes=new Set(plan.candidates.map(row=>row.prefix)),ready=plan.candidates.filter(row=>row.status==="active"||row.status==="inactive"),known=new Map(ready.map(row=>[row.generation!,row])),current=new Set(plan.current.scopes.map(row=>row.generation));
 if(prefixes.size!==plan.candidates.length||known.size!==ready.length||plan.generations.names.length!==prefixes.size||new Set(plan.generations.names).size!==prefixes.size||plan.generations.names.some(name=>!prefixes.has(name))||!current.size||current.size!==plan.current.scopes.length||[...current].some(generation=>!known.has(generation)))throw Error("Complete unique current dependency graph required");
 const incoming=new Set<string>();
 for(const row of plan.candidates){
  if(row.status!=="active"&&row.status!=="inactive"){
   if(row.ownerReferences.length)throw Error("Never-ready roles cannot issue artifact references");
   if(row.status==="retained-unready"||row.status==="unready-metadata"){
    if(!/^\.p[a-f0-9]{8}$/u.test(row.prefix))throw Error("Fixed capsule namespace required");
    if(row.status==="retained-unready"){
     if(typeof row.generation!=="string"||!/^[a-f0-9]{64}$/u.test(row.generation)||row.receiptFingerprint===null||!row.roles.includes("i")||!row.roles.includes("p")||row.roles.some(name=>!retirementProtocol.capsuleRoles.includes(name)))throw Error("Current intent-bearing capsule DATA required");
    }else if(row.generation!==null||row.receiptFingerprint!==null||row.roles.some(name=>!retirementProtocol.capsuleMetadataRoles.includes(name)))throw Error("Fixed metadata-only capsule DATA required");
   }else{
    if(!/^[a-f0-9]{20}$/u.test(row.prefix)||row.generation!==null||row.receiptFingerprint!==null)throw Error("Exact never-ready generation DATA required");
    if(row.status==="unqualified"?row.roles.length!==0:row.status!=="unready"||!row.roles.length||row.roles.some(name=>!retirementProtocol.unreadyPayloadRoles.includes(name)))throw Error("Unknown or unqualified dependency artifact retained");
   }
   continue;
  }
  if(row.generation===null||row.generation.slice(0,20)!==row.prefix||row.receiptFingerprint===null||(row.status==="active")!==current.has(row.generation))throw Error("Exact qualified dependency graph status required");
  for(const reference of row.ownerReferences){
   if(row.status==="active"&&!current.has(reference.generation)||!known.has(reference.generation)&&(row.status!=="inactive"||prefixes.has(reference.generation.slice(0,20))))throw Error("Unknown or obsolete live dependency reference retained");
   incoming.add(reference.generation);
  }
 }
 const obsolete=plan.candidates.filter(row=>row.status!=="active");if(!obsolete.length)return null;
 const eligible=obsolete.filter(row=>row.status!=="inactive"||!incoming.has(row.generation!)).sort((left,right)=>left.prefix<right.prefix?-1:left.prefix>right.prefix?1:0);
 if(!eligible.length)throw Error("Retained obsolete dependency reference cycle requires a closed cohort owner");return eligible[0]!;
}

/** 🎯️ Revalidates exact selected owner settlement DATA before constructing another real command. */
function readRepositoryDependencyRetirementSettlementV1(value:unknown,operation:"observe"|"retire",selection:JavascriptDependencySelectedRetirementCommandV1["selection"],artifact:JavascriptDependencySelectedRetirementCommandV1["artifact"]):Settlement{
 const violations=validateJsonSchemaSubset(retirementWireSchema.$defs.AcquiredSettlement as never,value,retirementWireSchema as never);if(violations.length)throw refuseCachingV1("failed","dependency-store.retirement-settlement-incomplete",{operation,prefix:selection.prefix,violation:String(violations[0]).slice(0,200)});const result=(value as{value:Settlement}).value,snapshot=result.selected;
 if(result.operation!==operation||result.retired!==(operation==="retire")||result.journalSha256!==artifact.journalSha256||result.journalBytes!==artifact.journalBytes||snapshot.kind!==selection.kind||snapshot.prefix!==selection.prefix||snapshot.rootStamp!==selection.rootStamp||snapshot.currentBindingsSha256!==selection.currentBindingsSha256||snapshot.generation!==("generation"in selection?selection.generation:null)||snapshot.receiptFingerprint!==("receiptFingerprint"in selection?selection.receiptFingerprint:null))throw Error("Exact unchanged-funding dependency retirement settlement required");return result;
}
