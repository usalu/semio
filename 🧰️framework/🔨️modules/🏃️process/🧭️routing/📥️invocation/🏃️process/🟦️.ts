import{AsyncLocalStorage}from"node:async_hooks";
import{withLeaseTaskV1}from"../../../🔒️leases/🧭️order/🟦️.ts";
import type{ResourceLeaseControl}from"../../../🔒️leases/🎛️control/🟦️.ts";
import requestSchema from"./📥️issuance/🧬️schema/📥️request.json" with{type:"json"};
import cleanupSchema from"./🧬️schema/🧹️cleanup.json" with{type:"json"};
import type{AcceptedPhysicalCleanupV1,PhysicalCleanupRequestV1,AcceptedPhysicalCleanupFactsV1,AcceptedPhysicalCleanupBodyV1}from"./🧹️cleanup/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import processBudgetSchema from "../../../⏱️budget/🧬️schema/🔣️.json" with {type:"json"};
import progressSchema from "./🧬️schema/📈️progress.json" with { type: "json" };
import sourceAdmissionSchema from "../../../📦️artifacts/🏗️native-build/📥️source/🎟️admission/🧬️schema/🔣️.json" with {type:"json"};
import policySchema from "../🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { checkScriptInvocation, readScriptPolicy, type ScriptInvocation, type ScriptPolicy, type ScriptProgress } from "../🟦️.ts";
import {readScriptProcessJsonV1,readScriptProcessJsonWireV1} from "./📥️json/🟦️.ts";

// [DEBUG] spin-hunt begin
if(process.env.SEMIO_DEBUG_SPIN==="1"){const ring:string[]=[],holder=globalThis as unknown as Record<string,(...args:unknown[])=>unknown>;for(const name of["setImmediate","setTimeout"]){const original=holder[name]!;holder[name]=function(this:unknown,...args:unknown[]){ring.push(name+" "+String(new Error().stack).split("\n").slice(2,8).map(line=>line.trim()).join(" | "));if(ring.length>48)ring.shift();return original.apply(this,args);};}process.on("SIGUSR2",()=>{process.stderr.write("[DEBUG] spin ring\n"+ring.join("\n")+"\n");});}
// [DEBUG] spin-hunt end
/** 📨️ Transfers the original process deadline without rebasing child authority. */
export type ScriptProcessEnvelope = Readonly<{ version: 1; policy: ScriptPolicy; deadlineEpochMilliseconds: number | null; capabilities: object }>;
export const SCRIPT_PROCESS_INVOCATION_ENV = "SEMIO_SCRIPT_PROCESS_INVOCATION";

function progressLanguage(environment:Readonly<Record<string,string|undefined>>):"en"|"de"{
 const declaration=progressSchema.const.sourceAdmission;
 return (declaration.systemVariables.map(name=>environment[name]?.split(/[_.@-]/u)[0]?.toLowerCase()).find(value=>value!==undefined&&Object.hasOwn(declaration.labels,value))??declaration.fallback[0])as"en"|"de";
}

function sourceProviderCounterV1(progress:ScriptProgress):number|null{
 const policy=progressSchema.const.sourceProvider,activity=progress.activity;
 if(progress.stage!=="running"||!activity||activity.mode!=="shared"||!Number.isFinite(activity.elapsedMs)||activity.elapsedMs<0||activity.resource.length>policy.maximumResourceCharacters||!activity.resource.startsWith(policy.resourcePrefix)||progress.command.length>policy.maximumCommandCharacters||!new RegExp(policy.commandPattern,"u").test(progress.command)||typeof progress.owner!=="string"||progress.owner.length>policy.maximumCommandCharacters||Object.keys(progress).sort().join(",")!=="activity,command,owner,stage"||Object.keys(activity).sort().join(",")!=="elapsedMs,mode,resource")return null;
 const text=activity.resource.slice(policy.resourcePrefix.length),count=Number(text);
 return new RegExp(policy.counterPattern,"u").test(text)&&Number.isSafeInteger(count)&&count<=policy.maximumCompleted?count:null;
}

function cacheScanActivityV1(progress:ScriptProgress,policy=progressSchema.const.cacheScan,mode:"shared"|"exclusive"="shared"):boolean{
 const activity=progress.activity;if(progress.stage!=="running"||!activity||activity.mode!==mode||!Number.isFinite(activity.elapsedMs)||activity.elapsedMs<0||activity.elapsedMs>policy.maximumElapsedMilliseconds||activity.resource.length>policy.maximumResourceCharacters||!activity.resource.startsWith(policy.resourcePrefix)||progress.command.length>policy.maximumCommandCharacters||!new RegExp(policy.commandPattern,"u").test(progress.command)||typeof progress.owner!=="string"||progress.owner.length>policy.maximumCommandCharacters||Object.keys(progress).sort().join(",")!=="activity,command,owner,stage"||Object.keys(activity).sort().join(",")!=="elapsedMs,mode,resource")return false;
 const path=activity.resource.slice(policy.resourcePrefix.length);return path.length<=policy.maximumPathCharacters&&new RegExp(policy.pathPattern,"u").test(path);
}

function toolingActivityV1(progress:ScriptProgress):keyof typeof progressSchema.const.tooling.labels.en|null{
 const policy=progressSchema.const.tooling,activity=progress.activity;if(progress.stage!=="running"||!activity||activity.mode!=="shared"||!Number.isFinite(activity.elapsedMs)||activity.elapsedMs<0||activity.elapsedMs>policy.maximumElapsedMilliseconds||activity.resource.length>policy.maximumResourceCharacters||progress.command.length>policy.maximumCommandCharacters||!new RegExp(policy.commandPattern,"u").test(progress.command)||typeof progress.owner!=="string"||progress.owner.length>policy.maximumCommandCharacters||Object.keys(progress).sort().join(",")!=="activity,command,owner,stage"||Object.keys(activity).sort().join(",")!=="elapsedMs,mode,resource")return null;
 if(policy.resources.includes(activity.resource))return activity.resource as keyof typeof policy.labels.en;if(!activity.resource.startsWith(policy.namespacePrefix))return null;const path=activity.resource.slice(policy.namespacePrefix.length);return path.length<=policy.maximumPathCharacters&&new RegExp(policy.pathPattern,"u").test(path)&&new RegExp(policy.namespaceDigestPattern,"u").test(path)?"namespace":null;
}

function dependencyWorkActivityV1(progress:ScriptProgress):{slot:number;family:number;count:number|undefined;elapsed:number}|null{
 const policy=progressSchema.const.dependencyWork,activity=progress.activity;
 if(progress.stage!=="running"||!activity||typeof activity.resource!=="string"||!Number.isFinite(activity.elapsedMs)||activity.elapsedMs<0||activity.elapsedMs>policy.maximumElapsedMilliseconds||activity.resource.length>policy.maximumResourceCharacters||typeof progress.owner!=="string"||progress.owner.length>policy.maximumOwnerCharacters||!new RegExp(policy.ownerPattern,"u").test(progress.owner)||typeof progress.command!=="string"||progress.command.length>policy.maximumCommandCharacters||!new RegExp(policy.commandPattern,"u").test(progress.command)||Object.keys(progress).sort().join(",")!=="activity,command,owner,stage"||Object.keys(activity).sort().join(",")!=="elapsedMs,mode,resource")return null;
 for(const[index,slot]of policy.slots.entries()){
  if(activity.mode!==slot.mode)continue;
  if(!slot.counter){if(activity.resource===slot.resourcePrefix&&activity.elapsedMs===policy.replayElapsedMilliseconds)return{slot:index,family:policy.families.indexOf(slot.family),count:undefined,elapsed:activity.elapsedMs};continue;}
  if(!activity.resource.startsWith(slot.resourcePrefix))continue;
  const text=activity.resource.slice(slot.resourcePrefix.length),count=Number(text);
  if(new RegExp(policy.counterPattern,"u").test(text)&&Number.isSafeInteger(count)&&count<=policy.maximumCompleted)return{slot:index,family:policy.families.indexOf(slot.family),count,elapsed:activity.elapsedMs};
 }
 return null;
}

/** 🪭️ Selects only the fixed plain bookkeeping frames for bounded presentation. */
function selectionStageSlotV1(progress:ScriptProgress):number|null{
 const policy=progressSchema.const.selectionStages;if(Object.keys(progress).sort().join(",")!=="command,owner,stage"||progress.owner!==policy.owner||progress.stage!==policy.stage)return null;const slot=policy.commands.indexOf(progress.command);return slot<0?null:slot;
}
/** 📉️ Coalesces typed presentation while retaining Original callbacks and bounded final observations. */
function createScriptProcessProgressWriterV1(write:(line:string)=>void,environment:Readonly<Record<string,string|undefined>>,now:()=>number=()=>performance.now()){
 type Row={raw:string;message:string|null};type Pending=Row&{rank:number};
 const policy=progressSchema.const.dependencyWork,workPending:(Pending|undefined)[]=policy.slots.map(()=>undefined),counts:(number|undefined)[]=policy.families.map(()=>undefined),elapsed:(number|undefined)[]=policy.families.map(()=>undefined);
 let pending:Row|undefined,key:string|undefined,lastAt:number|undefined,lastClock:number|undefined,lastCount:number|undefined,lastElapsed:number|undefined,frame:string|undefined,workAt:number|undefined,workClock:number|undefined,workState:"unopened"|"paced"|"unclocked"="unopened";
 const emit=(row:Row)=>{write(row.raw);if(row.message!==null)write("[script] "+row.message+"\n");};
 const stagePolicy=progressSchema.const.selectionStages,stageSeen=stagePolicy.commands.map(()=>false);let stageOpen=false,stageAt:number|undefined,stageClock:number|undefined,stageLast:number|undefined,stageLatest:(Row&{slot:number})|undefined;
 const flushStage=()=>{const latest=stageLatest,last=stageLast;stageLatest=undefined;stageLast=undefined;stageOpen=false;stageAt=undefined;stageClock=undefined;stageSeen.fill(false);if(latest&&latest.slot!==last)emit(latest);};
 const ordered=(family?:number,rows=workPending)=>rows.flatMap((row,slot)=>row&&(family===undefined||policy.families.indexOf(policy.slots[slot]!.family)===family)?[{row,slot}]:[]).sort((a,b)=>a.row.rank-b.row.rank||a.slot-b.slot);
 const drain=(rows=workPending,family?:number)=>{for(const{row,slot}of ordered(family,rows)){rows[slot]=undefined;emit(row);}},drainWork=(family?:number)=>drain(workPending,family);
 const remember=(slot:number,row:Row,rows=workPending)=>{rows[slot]=undefined;const previous=ordered(undefined,rows);previous.forEach(({row},rank)=>{row.rank=rank;});rows[slot]={...row,rank:previous.length};};
 const flushOld=()=>{if(pending){drainWork();const row=pending;pending=undefined;emit(row);}},emitOld=(row:Row)=>{drainWork();emit(row);},flush=()=>{flushStage();drainWork();flushOld();};
 return Object.freeze({flush,publish:(progress:ScriptProgress)=>{
  const row={raw:"[script] "+JSON.stringify(progress)+"\n",message:formatScriptProcessActivityV1(progress,environment)},identity=typeof progress.owner==="string"&&progress.owner.length<=policy.maximumOwnerCharacters&&new RegExp(policy.ownerPattern,"u").test(progress.owner)&&typeof progress.command==="string"&&progress.command.length<=policy.maximumCommandCharacters&&new RegExp(policy.commandPattern,"u").test(progress.command)&&policy.frameStages.includes(progress.stage)?JSON.stringify([progress.owner,progress.command,progress.stage]):undefined;
  const stageSlot=selectionStageSlotV1(progress);
  if(stageSlot!==null){
   if(!stageOpen){flush();stageOpen=true;frame=undefined;key=undefined;lastAt=undefined;lastClock=undefined;lastCount=undefined;lastElapsed=undefined;}
   const current=now(),bad=!Number.isFinite(current)||stageClock!==undefined&&current<stageClock;
   if(bad){flushStage();stageOpen=true;stageSeen[stageSlot]=true;stageClock=Number.isFinite(current)?current:undefined;stageAt=stageClock;stageLast=stageSlot;emit(row);return;}
   stageClock=current;if(stageAt===undefined||!stageSeen[stageSlot]){stageSeen[stageSlot]=true;stageAt=current;stageLast=stageSlot;stageLatest=undefined;emit(row);return;}
   stageLatest={...row,slot:stageSlot};if(current-stageAt>=stagePolicy.heartbeatMilliseconds){stageAt=current;stageLast=stageSlot;stageLatest=undefined;emit(row);}return;
  }
  flushStage();
  if(identity!==undefined&&identity!==frame){flush();frame=identity;workAt=undefined;workClock=undefined;workState="unopened";counts.fill(undefined);elapsed.fill(undefined);}
  const work=dependencyWorkActivityV1(progress);
  if(work!==null){
   const current=now(),bad=!Number.isFinite(current)||workClock!==undefined&&current<workClock,reset=work.count!==undefined&&(counts[work.family]!==undefined&&work.count<counts[work.family]!||elapsed[work.family]!==undefined&&work.elapsed<elapsed[work.family]!);
   counts[work.family]=work.count;elapsed[work.family]=work.elapsed;
   if(bad){drainWork();workClock=Number.isFinite(current)?current:undefined;workAt=workClock;workState=workClock===undefined?"unclocked":"paced";emit(row);return;}
   workClock=current;
   if(reset){drainWork(work.family);workAt=current;workState="paced";emit(row);return;}
   if(workState!=="paced"){workAt=current;workState="paced";emit(row);return;}
   remember(work.slot,row);
   if(current-workAt!>=policy.minimumPublicationMilliseconds){workAt=current;drainWork();}
   return;
  }
  const providerCount=sourceProviderCounterV1(progress),cache=cacheScanActivityV1(progress),deleting=cacheScanActivityV1(progress,progressSchema.const.cacheDelete,"exclusive"),tooling=toolingActivityV1(progress),count=providerCount??(deleting||tooling!==null?0:cache?progress.activity!.elapsedMs:null);
  if(count===null){flushOld();key=undefined;lastAt=undefined;lastClock=undefined;lastCount=undefined;lastElapsed=undefined;emitOld(row);return;}
  const current=now(),oldIdentity=progress.owner+"\0"+progress.command+"\0"+(cache||deleting||tooling!==null?progress.activity!.resource:"provider"),oldElapsed=progress.activity!.elapsedMs,reset=oldIdentity!==key||!deleting&&tooling===null&&(lastCount!==undefined&&count<lastCount||lastElapsed!==undefined&&oldElapsed<lastElapsed)||!Number.isFinite(current)||lastClock!==undefined&&current<lastClock;
  if(reset){flushOld();lastAt=undefined;}
  key=oldIdentity;lastClock=Number.isFinite(current)?current:undefined;lastCount=count;lastElapsed=oldElapsed;
  if(lastAt===undefined||current-lastAt>=(tooling!==null?progressSchema.const.tooling:deleting?progressSchema.const.cacheDelete:cache?progressSchema.const.cacheScan:progressSchema.const.sourceProvider).minimumPublicationMilliseconds){pending=undefined;emitOld(row);lastAt=Number.isFinite(current)?current:undefined;}else pending=row;
 }});
}

/** 📈️ Describes long resource activity without inferring whether work or contention caused it. */
export function formatScriptProcessActivityV1(progress:ScriptProgress,environment:Readonly<Record<string,string|undefined>>=process.env):string|null{
 const activity=progress.activity;if(!activity)return null;const declaration=progressSchema.const.sourceAdmission;
 const tooling=progressSchema.const.tooling;if(tooling.resources.includes(activity.resource)||activity.resource.startsWith(tooling.namespacePrefix)){const key=toolingActivityV1(progress);return key===null?null:tooling.labels[progressLanguage(environment)][key].replace("{command}",progress.command);}
 const provider=progressSchema.const.sourceProvider;
 if(activity.resource.startsWith(provider.resourcePrefix)){const count=sourceProviderCounterV1(progress);return count===null?null:provider.labels[progressLanguage(environment)].replace("{command}",progress.command).replace("{completed}",String(count));}
 const deletion=progressSchema.const.cacheDelete;if(activity.resource.startsWith(deletion.resourcePrefix))return cacheScanActivityV1(progress,deletion,"exclusive")?deletion.labels[progressLanguage(environment)].replace("{command}",progress.command).replace("{path}",activity.resource.slice(deletion.resourcePrefix.length)):null;
 const cache=progressSchema.const.cacheScan;if(activity.resource.startsWith(cache.resourcePrefix))return cacheScanActivityV1(progress)?cache.labels[progressLanguage(environment)].replace("{command}",progress.command).replace("{path}",activity.resource.slice(cache.resourcePrefix.length)).replace("{seconds}",String(Math.round(activity.elapsedMs/1000))):null;
 const parser=progressSchema.const.parserSource;
 if(activity.resource.startsWith(parser.resourcePrefix)){
  if(activity.elapsedMs!==0||activity.mode!=="shared"||activity.resource.length>parser.maximumResourceCharacters||progress.command.length>parser.maximumCommandCharacters||!new RegExp(parser.commandPattern,"u").test(progress.command))return null;
  const parts=activity.resource.slice(parser.resourcePrefix.length).split(":"),operation=parts[0]??"",text=parts[1]??"";if(parts.length!==2||!Object.hasOwn(parser.labels.en.operations,operation)||!new RegExp(parser.completedPattern,"u").test(text))return null;
  const completed=Number(text);if(!Number.isSafeInteger(completed)||completed<1||completed>parser.maximumCompleted)return null;
  const labels=parser.labels[progressLanguage(environment)],fields:Record<string,string>={command:progress.command,operation:labels.operations[operation as keyof typeof labels.operations],completed:String(completed),units:labels.units};
  return labels.template.replace(/\{([A-Za-z]+)\}/gu,(_token,key:string)=>fields[key]!);
 }

 const phase=progressSchema.const.graphPhase;
 if(activity.resource.startsWith(phase.resourcePrefix)){
  if(activity.elapsedMs!==0||activity.mode!=="shared"||activity.resource.length>phase.maximumResourceCharacters||progress.command.length>phase.maximumCommandCharacters||!new RegExp(phase.commandPattern,"u").test(progress.command))return null;
  const parts=activity.resource.slice(phase.resourcePrefix.length).split(":"),name=parts[0]??"";if(parts.length!==3||!Object.hasOwn(phase.labels.en.phases,name)||!parts.slice(1).every(part=>new RegExp(phase.countPattern,"u").test(part)))return null;
  const done=Number(parts[1]),total=Number(parts[2]);if(!Number.isSafeInteger(done)||!Number.isSafeInteger(total)||done>total)return null;
  const labels=phase.labels[progressLanguage(environment)],row=labels.phases[name as keyof typeof labels.phases],fields:Record<string,string>={command:progress.command,phase:row.label,units:row.units,done:String(done),total:String(total)};
  return labels.template.replace(/\{([A-Za-z]+)\}/gu,(_token,key:string)=>fields[key]!);
 }
 const queue=progressSchema.const.queueWait;
 if(activity.resource.startsWith(queue.resourcePrefix)){
  if(activity.elapsedMs!==0||activity.resource.length>queue.maximumResourceCharacters||progress.command.length>queue.maximumCommandCharacters||!new RegExp(queue.commandPattern,"u").test(progress.command))return null;
  let value:unknown;try{value=JSON.parse(activity.resource.slice(queue.resourcePrefix.length));}catch{return null;}
  if(validateJsonSchemaSubset(progressSchema.definitions.QueueWait,value).length)return null;
  const row=value as {position:number;queued:number;holderMode:"shared"|"exclusive";holderPid:number;holderSeconds:number;resource:string};if(row.position>row.queued)return null;
  const labels=queue.labels[progressLanguage(environment)],fields:Record<string,string>={command:progress.command,resource:row.resource,mode:labels.modes[activity.mode],position:String(row.position),queued:String(row.queued),holderPid:String(row.holderPid),holderMode:labels.modes[row.holderMode],holderSeconds:String(row.holderSeconds)};
  return labels.template.replace(/\{([A-Za-z]+)\}/gu,(_token,key:string)=>fields[key]!);
 }
 if(activity.resource.startsWith(declaration.resourcePrefix)){
  if(activity.elapsedMs!==0||activity.mode!=="shared"||activity.resource.length>declaration.maximumResourceCharacters||progress.command.length>declaration.maximumCommandCharacters||!/^[A-Za-z0-9:_/.-]+$/u.test(progress.command))return null;
  let value:unknown;try{value=JSON.parse(activity.resource.slice(declaration.resourcePrefix.length));}catch{return null;}
  if(validateJsonSchemaSubset(sourceAdmissionSchema.definitions.progress,value).length)return null;
  const row=value as {phase:"cfg"|"read"|"complete";unit:number;units:number;packageName:string;readOperations:number;files:number;bytes:number};if(row.unit>row.units||row.files>row.readOperations)return null;
  const language=progressLanguage(environment),labels=declaration.labels[language as keyof typeof declaration.labels],fields:Record<string,string>={command:progress.command,phase:labels.phases[row.phase],unit:String(row.unit),units:String(row.units),package:row.packageName,reads:String(row.readOperations),files:String(row.files),bytes:String(row.bytes)};
  return labels.template.replace(/\{([A-Za-z]+)\}/gu,(_token,key:string)=>fields[key]!);
 }
 if(!(activity.elapsedMs>=progressSchema.const.minimumElapsedMilliseconds))return null;
 return `${progress.command} remains active on ${activity.resource} (${activity.mode}, ${Math.round(activity.elapsedMs/1000)} s)`;
}

/** 🔎️ Validates the authored process envelope and its remaining original authority. */
export function readScriptProcessEnvelope(value: unknown, now: number): ScriptProcessEnvelope {
  value=readScriptProcessJsonV1(value);
  if (schema.properties.policy.$ref !== policySchema.$id) throw Error("Original Script policy schema owner required");
  const admittedSchema = { ...schema, properties: { ...schema.properties, policy: policySchema } };
  if (validateJsonSchemaSubset(admittedSchema, value).length) throw Error("Complete original Script process envelope required");
  const envelope = value as ScriptProcessEnvelope;
  if (!Number.isFinite(now)) throw Error("Original Script process clock refused");
  if (envelope.deadlineEpochMilliseconds !== null) {
    const remaining = envelope.deadlineEpochMilliseconds - now;
    if (remaining <= 0 || remaining > envelope.policy.maximumElapsedMilliseconds) throw Error("Original Script process deadline refused");
  }
  return envelope;
}

/** 🌱️ Creates the original process deadline from its explicitly supplied authored policy. */
export function createScriptProcessEnvelope(policy: ScriptPolicy, capabilities: object, now: number): ScriptProcessEnvelope {
  const original = readScriptPolicy(readScriptProcessJsonV1(policy));
  return readScriptProcessEnvelope({ version: 1, policy: original, deadlineEpochMilliseconds: original.maximumElapsedMilliseconds === 0 ? null : now + original.maximumElapsedMilliseconds, capabilities }, now);
}

declare const invocationBrand:unique symbol;
/** 🌱️ Identifies the exact live invocation admitted by the canonical process owner. */
export type IssuedScriptInvocationV1=ScriptInvocation&{readonly[invocationBrand]:true};
type Scope={live:boolean;accepting:boolean;pending:Set<Promise<unknown>>;failures:unknown[];retirements:Set<()=>void>;parent?:Scope};
type Birth=Scope&{invocation:IssuedScriptInvocationV1;envelope:ScriptProcessEnvelope};
type ControlScope=Scope&{birth:Birth;invocation:IssuedScriptInvocationV1;signal:AbortSignal;started:number;requestedMilliseconds:number;command:string;deadlineEpochMilliseconds:number|null;parentControl?:ControlScope};
let received=false;
const invocations=new WeakMap<ScriptInvocation,Birth>(),controls=new WeakMap<ResourceLeaseControl,ControlScope>(),context=new AsyncLocalStorage<ScriptInvocation>();
const scope=(parent?:Scope):Scope=>({live:true,accepting:true,pending:new Set(),failures:[],retirements:new Set(),...(parent?{parent}:{})});
/** 🚪️ A refusal that only reports an owner already closed by an earlier failure; it never names the failure itself. */
export class OriginalAdmissionClosedV1 extends Error{constructor(message:string){super(message);this.name="OriginalAdmissionClosed";}}
/** 🥇️ Names the languages one failure notice is written in: the one the environment selects, otherwise every declared language, none assumed. */
export function firstFailureLanguagesV1(environment:Readonly<Record<string,string|undefined>>):readonly("en"|"de")[]{const declared=Object.keys(progressSchema.const.firstFailure.labels) as ("en"|"de")[],selected=progressSchema.const.firstFailure.systemVariables.map(name=>environment[name]?.split(/[_.@-]/u)[0]?.toLowerCase()).find(value=>value!==undefined&&declared.includes(value as "en"|"de"));return selected===undefined?declared:[selected as "en"|"de"];}
/** 🥇️ Writes the first failure of an owner in every selected language, so a later refusal of that closed owner never masks it. */
export function renderFirstFailureV1(first:unknown,masked:unknown,environment:Readonly<Record<string,string|undefined>>):string{const reason=(value:unknown):string=>String(value instanceof Error?value.message:value).split("\n",1)[0]!.slice(0,progressSchema.const.firstFailure.maximumReasonCharacters);return firstFailureLanguagesV1(environment).map(language=>progressSchema.const.firstFailure.labels[language].replace("{first}",reason(first)).replace("{masked}",reason(masked))).join("\n")+"\n";}
function firstFailureV1(owner:Scope,failure:unknown):unknown{if(!(failure instanceof OriginalAdmissionClosedV1)||!owner.failures.length||owner.failures[0]===failure)return failure;try{process.stderr.write(renderFirstFailureV1(owner.failures[0],failure,process.env));}catch{}return owner.failures[0];}
function observe<T>(work:Promise<T>):Promise<T>{void work.then(()=>{},()=>{});return work;}
function track<T>(owner:Scope,run:()=>Promise<T>):Promise<T>{
 for(let row:Scope|undefined=owner;row;row=row.parent)if(!row.live||!row.accepting)throw new OriginalAdmissionClosedV1("Original operation admission closed");
 const work=Promise.resolve().then(run);
 for(let row:Scope|undefined=owner;row;row=row.parent){const accepted=row;accepted.pending.add(work);void work.then(()=>{accepted.pending.delete(work);},error=>{accepted.accepting=false;if(!accepted.failures.length)accepted.failures.push(error);accepted.pending.delete(work);});}
 return work;
}
async function drain(owner:Scope):Promise<void>{owner.accepting=false;while(owner.pending.size)await Promise.allSettled([...owner.pending]);}
async function settle<T>(owner:Scope,run:()=>Promise<T>,check?:()=>void):Promise<T>{let value:T|undefined,failed=false,failure:unknown;try{value=await run();}catch(error){failed=true;failure=error;}try{await drain(owner);if(failed)throw firstFailureV1(owner,failure);if(owner.failures.length)throw owner.failures[0];check?.();return value as T;}finally{owner.live=false;for(const stop of[...owner.retirements])stop();}}
function birthOf(invocation:ScriptInvocation):Birth{const birth=invocations.get(invocation);if(!birth?.live)throw Error("Live Original process issuance required");return birth;}
function admit(invocation:ScriptInvocation,parent?:ControlScope):Birth{requireIssuedScriptInvocationV1(invocation);const birth=birthOf(invocation);if(!birth.accepting)throw new OriginalAdmissionClosedV1("Original process admission closed");for(let row=parent;row;row=row.parentControl)if(!row.live||!row.accepting)throw new OriginalAdmissionClosedV1("Original resource admission closed");return birth;}
function startProcessDeadlineV1(remaining:()=>number,expire:()=>void,unref:boolean):()=>void{
 let timer:ReturnType<typeof setTimeout>|undefined,stopped=false;
 const stop=()=>{stopped=true;clearTimeout(timer);};
 const arm=()=>{if(stopped)return;const available=remaining();if(available<=0){stop();expire();return;}timer=setTimeout(arm,Math.max(1,Math.min(processBudgetSchema.$defs.TimerMaximumMilliseconds.const,Math.ceil(available))));if(unref)timer.unref();};
 arm();return stop;
}
/** 🔐️ Refuses structural copies and retired process scopes. */
export function requireIssuedScriptInvocationV1(invocation:ScriptInvocation):asserts invocation is IssuedScriptInvocationV1{birthOf(invocation);checkScriptInvocation(invocation);}
/** 🎛️ Retains canonical cancellation and accepted asynchronous work through actual settlement. */
export function withScriptProcessEnvelope<T>(envelope:ScriptProcessEnvelope,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
 try{const admitted=readScriptProcessEnvelope(envelope,Date.now()),incoming=context.getStore();
 if(incoming){const birth=admit(incoming);if(JSON.stringify(admitted)!==JSON.stringify(birth.envelope))throw Error("Nested Original process envelope must preserve its actual parent");return track(birth,()=>run(incoming as IssuedScriptInvocationV1));}
 const started=performance.now(),deadline=admitted.deadlineEpochMilliseconds,initial=deadline===null?null:deadline-Date.now(),controller=new AbortController(),interrupt=()=>controller.abort(Error("Original Script process interrupted")),terminate=()=>controller.abort(Error("Original Script process terminated"));
 const stopTimeout=initial===null?undefined:startProcessDeadlineV1(()=>Math.min(deadline!-Date.now(),initial-(performance.now()-started)),()=>controller.abort(Error("Original Script process deadline exhausted")),true);process.once("SIGINT",interrupt);process.once("SIGTERM",terminate);
 const presentation=createScriptProcessProgressWriterV1(line=>{process.stderr.write(line);},process.env);
 const invocation:ScriptInvocation=Object.freeze({policy:admitted.policy,capabilities:admitted.capabilities,control:Object.freeze({signal:controller.signal,remainingMilliseconds:()=>deadline===null||initial===null?null:Math.max(0,Math.floor(Math.min(deadline-Date.now(),initial-(performance.now()-started)))),publish:presentation.publish,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))})});
 const birth:Birth={...scope(),invocation:invocation as IssuedScriptInvocationV1,envelope:admitted};invocations.set(invocation,birth);
 return observe(context.run(invocation,()=>settle(birth,async()=>{requireIssuedScriptInvocationV1(invocation);return await run(invocation);}).then(value=>{presentation.flush();checkScriptInvocation(invocation);return value;},error=>{try{presentation.flush();}catch{}throw error;}).finally(()=>{stopTimeout?.();process.removeListener("SIGINT",interrupt);process.removeListener("SIGTERM",terminate);})));
 }catch(error){return observe(Promise.reject(error));}
}
const implementationPinKeyV1=Symbol.for("semio.implementationPin.context.v1");
type ImplementationPinV1={active:boolean;authority:Map<unknown,unknown>};
/** 📌️ Commits a process that received its parent's authenticated invocation to the implementation it loaded: the same shared pin context `withImplementationPinV1` of the repository library opens, so every live-source self-check lets this short run finish and what it publishes names the bytes it loaded; a Source edited meanwhile is left to the next invocation. @see ../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs */
function withReceivedImplementationPinV1<T>(run:()=>Promise<T>):Promise<T>{const holder=globalThis as unknown as Record<symbol,AsyncLocalStorage<ImplementationPinV1>|undefined>,store=holder[implementationPinKeyV1]??=new AsyncLocalStorage<ImplementationPinV1>(),parent=store.getStore(),pin:ImplementationPinV1={active:true,authority:parent?.active===true?parent.authority:new Map()};return store.run(pin,run).finally(()=>{pin.active=false;});}
/** 📥️ Receives the real process envelope or reuses its exact live in-process parent, and runs under the pin of the implementation this process loaded. */
export function receiveScriptProcessInvocation<T>(environment:Readonly<Record<string,string|undefined>>,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{try{const wire=environment[SCRIPT_PROCESS_INVOCATION_ENV];if(wire===undefined)throw Error("Original parent Script process invocation required");if(!context.getStore()){if(wire!==process.env[SCRIPT_PROCESS_INVOCATION_ENV])throw Error("Actual process parent envelope required");if(received)throw Error("Original process birth already received");received=true;}return withScriptProcessEnvelope(readScriptProcessEnvelope(readScriptProcessJsonWireV1(wire),Date.now()),invocation=>withReceivedImplementationPinV1(()=>run(invocation)));}catch(error){return observe(Promise.reject(error));}}
function remaining(original:ControlScope):number|null{
 requireIssuedScriptInvocationV1(original.invocation);let value=original.invocation.control.remainingMilliseconds();const now=performance.now();for(let row:ControlScope|undefined=original;row;row=row.parentControl){if(!row.live)throw Error("Live Original resource control issuance required");row.signal.throwIfAborted();if(row.deadlineEpochMilliseconds!==null){const absolute=row.deadlineEpochMilliseconds-Date.now();value=value===null?absolute:Math.min(value,absolute);}if(row.requestedMilliseconds!==0){const child=row.requestedMilliseconds-(now-row.started);value=value===null?child:Math.min(value,child);}}if(value!==null&&value<=0)throw Error("Original resource loan deadline exhausted");return value;
}
/** 🪪️ Authenticates the exact issued control rather than its structural ports. */
export function requireIssuedResourceControlV1(control:ResourceLeaseControl):void{const original=controls.get(control);if(!original||control.signal!==original.signal)throw Error("Live Original resource control issuance required");remaining(original);}
/** 🪟️ Reads only the unchanged capabilities of this exact privately issued live control. */
export function readIssuedResourceControlCapabilitiesV1(control:ResourceLeaseControl):Readonly<ScriptInvocation["capabilities"]>{requireIssuedResourceControlV1(control);return controls.get(control)!.invocation.capabilities;}
/** 📎️ Requires a live control's privately captured exact invocation without issuing new authority. */
export function requireIssuedResourceControlInvocationV1(control:ResourceLeaseControl,invocation:ScriptInvocation):void{requireIssuedScriptInvocationV1(invocation);requireIssuedResourceControlV1(control);if(controls.get(control)!.invocation!==invocation)throw Error("Exact Original control invocation required");}
/** 🧬️ Requires actual inherited restrictions without issuing controls or reconstructing their clocks. */
export function requireIssuedResourceControlRefinementV1(base:ResourceLeaseControl,phase:ResourceLeaseControl):void{requireIssuedResourceControlV1(base);requireIssuedResourceControlV1(phase);const owner=controls.get(base)!,borrower=controls.get(phase)!;if(owner.invocation!==borrower.invocation)throw Error("Exact Original control refinement required");const inherited=new Set<ControlScope>();for(let row:ControlScope|undefined=borrower;row;row=row.parentControl)inherited.add(row);for(let row:ControlScope|undefined=owner;row&&!inherited.has(row);row=row.parentControl)if(row.requestedMilliseconds!==0||row.signal!==(row.parentControl?.signal??row.invocation.control.signal))throw Error("Actual inherited control restriction required");}
/** 📜️ Validates a closed nonrenewable child request. */
export function readOriginalResourceControlRequestV1(value:unknown):Readonly<{version:1;command:string;maximumElapsedMilliseconds:number}>{if(validateJsonSchemaSubset(requestSchema,value).length)throw Error("Closed Original resource control request required");return Object.freeze({...value as{version:1;command:string;maximumElapsedMilliseconds:number}});}
function scheduleControlDeadline(original:ControlScope,controller:AbortController):void{
 const owners:Scope[]=[];for(let row:Scope|undefined=original;row;row=row.parent)owners.push(row);let timer:ReturnType<typeof setTimeout>|undefined,stopped=false;
 const stop=()=>{if(stopped)return;stopped=true;clearTimeout(timer);original.signal.removeEventListener("abort",stop);for(const owner of owners)owner.retirements.delete(stop);};
 const arm=()=>{if(stopped)return;try{const milliseconds=remaining(original);if(milliseconds===null)throw Error("Finite Original control deadline required");timer=setTimeout(arm,Math.max(1,Math.min(2147483647,Math.ceil(milliseconds))));timer.unref();}catch(error){controller.abort(error);stop();}};
 for(const owner of owners)owner.retirements.add(stop);original.signal.addEventListener("abort",stop,{once:true});if(original.signal.aborted)stop();else arm();
}
function controlFor(invocation:ScriptInvocation,command:string,requestedMilliseconds:number,parent?:ControlScope,additionalSignal?:AbortSignal):ResourceLeaseControl{
 readOriginalResourceControlRequestV1({version:1,command,maximumElapsedMilliseconds:requestedMilliseconds});if(additionalSignal!==undefined&&!(additionalSignal instanceof AbortSignal))throw Error("Actual additional cancellation signal required");const birth=admit(invocation,parent),incoming=parent?.signal??invocation.control.signal,controller=requestedMilliseconds===0?undefined:new AbortController(),signals=[incoming,...(additionalSignal?[additionalSignal]:[]),...(controller?[controller.signal]:[])],child:ControlScope={...scope(parent??birth),birth,invocation:invocation as IssuedScriptInvocationV1,command,requestedMilliseconds,started:performance.now(),deadlineEpochMilliseconds:requestedMilliseconds===0?(parent?.deadlineEpochMilliseconds??birth.envelope.deadlineEpochMilliseconds):Math.min(parent?.deadlineEpochMilliseconds??birth.envelope.deadlineEpochMilliseconds??Infinity,Date.now()+requestedMilliseconds),signal:signals.length===1?incoming:AbortSignal.any(signals),...(parent?{parentControl:parent}:{})};
 const control:ResourceLeaseControl=Object.freeze({signal:child.signal,remainingMilliseconds:()=>remaining(child),publish:async(progress:Parameters<ResourceLeaseControl["publish"]>[0])=>{remaining(child);if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1"){}await invocation.control.publish({owner:invocation.policy.owner,command,stage:"running",activity:{resource:progress.resource,mode:progress.mode,elapsedMs:progress.elapsedMs}});remaining(child);},yieldContinuation:async()=>{remaining(child);await invocation.control.yieldContinuation();remaining(child);}});controls.set(control,child);if(controller)scheduleControlDeadline(child,controller);return control;
}
/** 🎚️ Creates a synchronous control only beneath an actual privately live invocation. */
export function createIssuedResourceControlV1(invocation:ScriptInvocation,command:string,requestedMilliseconds:number,additionalSignal?:AbortSignal):ResourceLeaseControl{return controlFor(invocation,command,requestedMilliseconds,undefined,additionalSignal);}
/** 🧷️ Creates a synchronous narrowed control beneath its exact privately issued parent. */
export function createRequestedIssuedResourceControlV1(control:ResourceLeaseControl,requestedMilliseconds:number,additionalSignal?:AbortSignal):ResourceLeaseControl{requireIssuedResourceControlV1(control);const original=controls.get(control)!;return controlFor(original.invocation,original.command,requestedMilliseconds,original,additionalSignal);}
/** 🎛️ Retains one issued resource loan until its callback and accepted children settle. */
export function withIssuedResourceControlV1<T>(invocation:ScriptInvocation,command:string,requestedMilliseconds:number,run:(control:ResourceLeaseControl)=>Promise<T>,additionalSignal?:AbortSignal):Promise<T>{const control=controlFor(invocation,command,requestedMilliseconds,undefined,additionalSignal),child=controls.get(control)!;return track(child.parent!,()=>settle(child,async()=>{remaining(child);const result=await withLeaseTaskV1(()=>run(control));remaining(child);return result;},()=>{remaining(child);}));}
/** 🧷️ Retains one narrowed resource loan without renewing parent time or cancellation. */
export function withRequestedIssuedResourceControlV1<T>(control:ResourceLeaseControl,requestedMilliseconds:number,run:(control:ResourceLeaseControl)=>Promise<T>,additionalSignal?:AbortSignal):Promise<T>{const childControl=createRequestedIssuedResourceControlV1(control,requestedMilliseconds,additionalSignal),child=controls.get(childControl)!;return track(child.parent!,()=>settle(child,async()=>{remaining(child);const result=await withLeaseTaskV1(()=>run(childControl));remaining(child);return result;},()=>{remaining(child);}));}
/** 🛟️ Registers accepted asynchronous custody before its first physical effect and through close. */
export function withIssuedResourceOperationV1<T>(control:ResourceLeaseControl,run:()=>Promise<T>):Promise<T>{requireIssuedResourceControlV1(control);const original=controls.get(control)!;admit(original.invocation,original);return track(original,run);}
type SignalScope={parent:ScriptInvocation;additionalSignal:AbortSignal;stopped:boolean};
const signalScopes=new WeakMap<ScriptInvocation,SignalScope>();
/** 🔗️ Retains narrowed cancellation through callback and accepted-child settlement. */
export function withScriptInvocationSignalV1<T>(invocation:ScriptInvocation,additionalSignal:AbortSignal,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
 const birth=admit(invocation);if(!(additionalSignal instanceof AbortSignal))throw Error("Actual additional cancellation signal required");const signal=AbortSignal.any([invocation.control.signal,additionalSignal]),alias:ScriptInvocation=Object.freeze({policy:invocation.policy,capabilities:invocation.capabilities,control:Object.freeze({...invocation.control,signal})}),child:Birth={...scope(birth),invocation:alias as IssuedScriptInvocationV1,envelope:birth.envelope},stopping:SignalScope={parent:invocation,additionalSignal,stopped:false};invocations.set(alias,child);signalScopes.set(alias,stopping);return track(birth,()=>context.run(alias,()=>settle(child,async()=>{checkScriptInvocation(alias);return run(alias as IssuedScriptInvocationV1);}).then(value=>{requireIssuedScriptInvocationV1(invocation);if(!stopping.stopped)checkScriptInvocation(alias);return value;})));
}
/** 🧬️ Requires private live signal ancestry without issuing or registering invocation authority. */
export function requireIssuedScriptInvocationDescendantV1(ancestor:ScriptInvocation,borrower:ScriptInvocation):void{requireIssuedScriptInvocationV1(ancestor);requireIssuedScriptInvocationV1(borrower);let current=borrower;while(current!==ancestor){const parent=signalScopes.get(current)?.parent;if(!parent)throw Error("Exact Original invocation lineage required");requireIssuedScriptInvocationV1(parent);current=parent;}}
/** 🛑️ Acknowledges handled owned cancellation without retiring or canceling its actual parent. */
export function acknowledgeScriptInvocationSignalStopV1(invocation:ScriptInvocation):void{const signal=signalScopes.get(invocation);if(!signal||!birthOf(invocation).live||!signal.additionalSignal.aborted)throw Error("Actual stopped derived Script signal scope required");requireIssuedScriptInvocationV1(signal.parent);signal.stopped=true;}

/** 📤️ Carries the same admitted deadline into an owned child process environment. */
export function scriptProcessEnvironment(envelope: ScriptProcessEnvelope, environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  return { ...environment, [SCRIPT_PROCESS_INVOCATION_ENV]: JSON.stringify(readScriptProcessEnvelope(envelope, Date.now())) };
}

/** 📤️ Hands the caller's current original invocation, with its remaining deadline, to an owned child process environment. */
export function scriptInvocationEnvironment(invocation: ScriptInvocation, environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  requireIssuedScriptInvocationV1(invocation);
  return scriptProcessEnvironment(birthOf(invocation).envelope,environment);
}

const projectedChildDeadlines=new WeakMap<ControlScope,number>();
/** 📨️ Projects only the actual live phase's remaining deadline into an already owned child's wire DATA. */
export function scriptResourceProcessEnvironmentV1(control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>):Record<string,string|undefined>{
 requireIssuedResourceControlV1(control);const owner=controls.get(control)!,available=remaining(owner),original=owner.birth.envelope;let carried=original;const suppliedWire=environment[SCRIPT_PROCESS_INVOCATION_ENV];if(suppliedWire!==undefined)try{const candidate=readScriptProcessEnvelope(JSON.parse(suppliedWire),Date.now());if(candidate.policy.owner===original.policy.owner)carried=candidate;}catch{}
 if(available===null)return scriptProcessEnvironment(carried,environment);
 let ceiling=Math.min(original.policy.maximumElapsedMilliseconds||Infinity,carried.policy.maximumElapsedMilliseconds||Infinity),deadline=Math.min(original.deadlineEpochMilliseconds??Infinity,carried.deadlineEpochMilliseconds??Infinity);for(let row:ControlScope|undefined=owner;row;row=row.parentControl){if(row.requestedMilliseconds!==0)ceiling=Math.min(ceiling,row.requestedMilliseconds);deadline=Math.min(deadline,row.deadlineEpochMilliseconds??Infinity,projectedChildDeadlines.get(row)??Infinity);}
 if(!Number.isSafeInteger(ceiling)||ceiling<1||ceiling>policySchema.properties.maximumElapsedMilliseconds.maximum)throw Error("Representable actual child phase policy required");const now=Date.now();deadline=Math.min(deadline,now+Math.floor(available));if(!Number.isSafeInteger(deadline)||deadline<=now)throw Error("Original child phase deadline exhausted");
 const envelope=readScriptProcessEnvelope({version:1,policy:readScriptPolicy({...original.policy,maximumElapsedMilliseconds:ceiling}),capabilities:carried.capabilities,deadlineEpochMilliseconds:deadline},now);projectedChildDeadlines.set(owner,deadline);requireIssuedResourceControlV1(control);return scriptProcessEnvironment(envelope,environment);
}

const physicalCleanupOperationLimits=Object.freeze({...cleanupSchema.definitions.OperationProtocol.const});
type PhysicalCleanupScope={live:boolean;started:number;maximumMilliseconds:number|null;deadlineEpochMilliseconds:number|null;facts:AcceptedPhysicalCleanupFactsV1;body:ControlScope;bodyController:AbortController;cleanupController:AbortController;stopBodyTimer?:()=>void;stopWholeTimer?:()=>void;begin?:Promise<void>;phase:"body"|"cleanup";cleanupAccepting:boolean;cleanupPending:Set<Promise<unknown>>;cleanupFailures:unknown[];cleanupAccepted:number;cleanupRefusal?:Promise<never>;settlement:Promise<void>;settle():void};
type PhysicalCleanupBirthFrame={enrollment:AcceptedPhysicalCleanupV1;parent?:PhysicalCleanupBirthFrame};
const cleanups=new WeakMap<AcceptedPhysicalCleanupV1,PhysicalCleanupScope>(),physicalCleanupBirthContext=new AsyncLocalStorage<PhysicalCleanupBirthFrame>();
function cleanupOf(token:AcceptedPhysicalCleanupV1):PhysicalCleanupScope{const accepted=cleanups.get(token);if(!accepted?.live||!accepted.body.birth.live)throw Error("Live accepted physical cleanup enrollment required");return accepted;}
/** 🛟️ Accepts fixed terminal cleanup before effects, carving finite bodies or retaining an ordinary body's Original. */
export function withAcceptedPhysicalCleanupV1<T>(control:ResourceLeaseControl,request:PhysicalCleanupRequestV1,run:AcceptedPhysicalCleanupBodyV1<T>):Promise<T>{
 requireIssuedResourceControlV1(control);request=readScriptProcessJsonV1(request)as PhysicalCleanupRequestV1;if(validateJsonSchemaSubset(cleanupSchema,request).length)throw Error("Closed physical cleanup funding request required");const original=controls.get(control)!,acceptedAtEpochMilliseconds=Date.now(),acceptedAtMonotonicMilliseconds=performance.now(),available=remaining(original),deadlineEpochMilliseconds=available===null?null:Math.min(original.deadlineEpochMilliseconds??Infinity,acceptedAtEpochMilliseconds+Math.floor(available)),reserveMilliseconds=request.reserveMilliseconds;if(deadlineEpochMilliseconds!==null&&(!Number.isSafeInteger(deadlineEpochMilliseconds)||deadlineEpochMilliseconds-acceptedAtEpochMilliseconds<=reserveMilliseconds))throw Error("Finite Original physical cleanup reserve required before effects");
 const bodyDeadlineEpochMilliseconds=deadlineEpochMilliseconds===null?null:deadlineEpochMilliseconds-reserveMilliseconds,cleanupController=new AbortController(),bodyController=new AbortController(),bodyControl=controlFor(original.invocation,original.command,bodyDeadlineEpochMilliseconds===null?0:bodyDeadlineEpochMilliseconds-acceptedAtEpochMilliseconds,original,bodyController.signal),body=controls.get(bodyControl)!,token=Object.freeze({})as AcceptedPhysicalCleanupV1,facts:AcceptedPhysicalCleanupFactsV1=Object.freeze({version:1,kind:request.kind,resource:request.resource,acceptedAtEpochMilliseconds,deadlineEpochMilliseconds,bodyDeadlineEpochMilliseconds,reserveMilliseconds,cleanupSignal:cleanupController.signal}),stopBodyTimer=bodyDeadlineEpochMilliseconds===null?undefined:startProcessDeadlineV1(()=>Math.min(bodyDeadlineEpochMilliseconds-Date.now(),bodyDeadlineEpochMilliseconds-acceptedAtEpochMilliseconds-(performance.now()-acceptedAtMonotonicMilliseconds)),()=>bodyController.abort(Error("Original physical body cutoff reached")),false),stopWholeTimer=deadlineEpochMilliseconds===null?undefined:startProcessDeadlineV1(()=>Math.min(deadlineEpochMilliseconds-Date.now(),deadlineEpochMilliseconds-acceptedAtEpochMilliseconds-(performance.now()-acceptedAtMonotonicMilliseconds)),()=>cleanupController.abort(Error("Original physical cleanup deadline exhausted")),false);body.deadlineEpochMilliseconds=bodyDeadlineEpochMilliseconds;let settle!:()=>void;const settlement=new Promise<void>(done=>{settle=done;}),accepted:PhysicalCleanupScope={settlement,settle,live:true,started:acceptedAtMonotonicMilliseconds,maximumMilliseconds:deadlineEpochMilliseconds===null?null:deadlineEpochMilliseconds-acceptedAtEpochMilliseconds,deadlineEpochMilliseconds,facts,body,bodyController,cleanupController,stopBodyTimer,stopWholeTimer,phase:"body",cleanupAccepting:true,cleanupPending:new Set(),cleanupFailures:[],cleanupAccepted:0};cleanups.set(token,accepted);
 return track(original,()=>physicalCleanupBirthContext.run({enrollment:token,parent:physicalCleanupBirthContext.getStore()},async()=>{let value:T|undefined,failed=false,failure:unknown;try{value=await run(token,bodyControl);}catch(error){failed=true;failure=error;}const failures:unknown[]=failed?[failure]:[];try{await beginAcceptedPhysicalCleanupV1(token);}catch(error){if(!failures.some(value=>Object.is(value,error)))failures.push(error);}accepted.cleanupAccepting=false;while(accepted.cleanupPending.size)await Promise.allSettled([...accepted.cleanupPending]);for(const error of accepted.cleanupFailures)if(!failures.some(value=>Object.is(value,error)))failures.push(error);try{checkAcceptedPhysicalCleanupV1(token);}catch(error){if(!failures.some(value=>Object.is(value,error)))failures.push(error);}finally{accepted.live=false;body.live=false;accepted.stopBodyTimer?.();accepted.stopWholeTimer?.();accepted.settle();}if(failures.length>1)throw new AggregateError(failures,"Original physical body, drain and cleanup settlement failed");if(failures.length)throw failures[0];return value as T;}));
}
/** 🔭️ Observes actual final settlement without authorizing effects, renewal or release. */
export function whenAcceptedPhysicalCleanupSettledV1(token:AcceptedPhysicalCleanupV1):Promise<void>{const accepted=cleanups.get(token);if(!accepted)throw Error("Actual accepted physical cleanup enrollment required");return accepted.settlement;}
/** 🛑️ Refuses an owner waiting for its own terminal callback to return. */
export function requireOutsideAcceptedPhysicalCleanupV1(token:AcceptedPhysicalCleanupV1):void{const accepted=cleanups.get(token);if(!accepted)throw Error("Actual accepted physical cleanup enrollment required");if(accepted.live)for(let frame=physicalCleanupBirthContext.getStore();frame;frame=frame.parent)if(frame.enrollment===token)throw Error("Cannot wait for cleanup settlement inside its actual owner");}
/** 📋️ Reads immutable funding facts from the exact live private enrollment. */
export function readAcceptedPhysicalCleanupV1(token:AcceptedPhysicalCleanupV1):AcceptedPhysicalCleanupFactsV1{return cleanupOf(token).facts;}
/** 🧹️ Stops body admission once and joins accepted body work before any cleanup phase. */
export function beginAcceptedPhysicalCleanupV1(token:AcceptedPhysicalCleanupV1):Promise<void>{const accepted=cleanupOf(token);if(accepted.begin)return accepted.begin;accepted.body.accepting=false;if(accepted.deadlineEpochMilliseconds===null){accepted.started=performance.now();accepted.maximumMilliseconds=accepted.facts.reserveMilliseconds;accepted.deadlineEpochMilliseconds=Date.now()+accepted.facts.reserveMilliseconds;accepted.stopWholeTimer=startProcessDeadlineV1(()=>Math.min(accepted.deadlineEpochMilliseconds!-Date.now(),accepted.maximumMilliseconds!-(performance.now()-accepted.started)),()=>{const error=Error("Original physical cleanup deadline exhausted");accepted.cleanupController.abort(error);accepted.bodyController.abort(error);},false);}accepted.begin=observe((async()=>{await drain(accepted.body);accepted.stopBodyTimer?.();accepted.body.live=false;for(const stop of[...accepted.body.retirements])stop();accepted.phase="cleanup";if(accepted.body.failures.length)throw accepted.body.failures[0];})());return accepted.begin;}
/** ⏱️ Checks only the accepted cleanup phase and its nonrenewable whole-operation deadline. */
export function checkAcceptedPhysicalCleanupV1(token:AcceptedPhysicalCleanupV1):number{const accepted=cleanupOf(token);if(accepted.phase!=="cleanup"||accepted.deadlineEpochMilliseconds===null||accepted.maximumMilliseconds===null)throw Error("Accepted physical body must stop before cleanup");accepted.facts.cleanupSignal.throwIfAborted();const remaining=Math.min(accepted.deadlineEpochMilliseconds-Date.now(),accepted.maximumMilliseconds-(performance.now()-accepted.started));if(remaining<=0)throw Error("Original physical cleanup deadline exhausted");return remaining;}
type PhysicalCleanupFrame={enrollment:AcceptedPhysicalCleanupV1;live:boolean;accepting:boolean;pending:Set<Promise<unknown>>;failures:unknown[];closing?:Promise<void>};
const physicalCleanupContext=new AsyncLocalStorage<PhysicalCleanupFrame>();
/** 🔚️ Joins the current accepted cleanup descendants before its real resource owner releases custody. */
export function closeAcceptedPhysicalCleanupChildrenV1(token:AcceptedPhysicalCleanupV1):Promise<void>{cleanupOf(token);const frame=physicalCleanupContext.getStore();if(!frame||frame.enrollment!==token||!frame.live)throw Error("Actual live cleanup operation frame required");if(frame.closing)return frame.closing;frame.accepting=false;frame.closing=observe((async()=>{while(frame.pending.size)await Promise.allSettled([...frame.pending]);if(frame.failures.length)throw frame.failures[0];})());return frame.closing;}
/** 📨️ Enrolls finite cleanup work before effects and joins every actual accepted descendant without a new control. */
export function withAcceptedPhysicalCleanupOperationV1<T>(token:AcceptedPhysicalCleanupV1,run:()=>Promise<T>):Promise<T>{checkAcceptedPhysicalCleanupV1(token);const scope=cleanupOf(token),current=physicalCleanupContext.getStore(),limits=physicalCleanupOperationLimits,parent=current?.enrollment===token?current:undefined;if(current&&(!parent||!parent.live||!parent.accepting)||!parent&&!scope.cleanupAccepting||scope.cleanupPending.size>=limits.maximumPending||scope.cleanupAccepted>=limits.maximumAccepted||typeof run!=="function"){if(!scope.cleanupRefusal){const error=Error("Accepted physical cleanup operation admission closed");scope.cleanupFailures.push(error);scope.cleanupRefusal=observe(Promise.reject(error));}return scope.cleanupRefusal;}scope.cleanupAccepted++;const frame:PhysicalCleanupFrame={enrollment:token,live:true,accepting:true,pending:new Set(),failures:[]},work=observe(Promise.resolve().then(()=>physicalCleanupContext.run(frame,async()=>{let value:T|undefined,failed=false,failure:unknown;try{checkAcceptedPhysicalCleanupV1(token);value=await run();}catch(error){failed=true;failure=error;}try{await closeAcceptedPhysicalCleanupChildrenV1(token);}catch(error){if(!failed){failed=true;failure=error;}}finally{frame.live=false;}if(failed)throw failure;return value as T;})));scope.cleanupPending.add(work);parent?.pending.add(work);void work.then(()=>{scope.cleanupPending.delete(work);parent?.pending.delete(work);},error=>{scope.cleanupPending.delete(work);if(!scope.cleanupFailures.some(row=>Object.is(row,error)))scope.cleanupFailures.push(error);if(parent){parent.accepting=false;parent.pending.delete(work);if(!parent.failures.length)parent.failures.push(error);}});return work;}

/** 🧬️ Checks only actual captured ancestry for retained custody; it issues no control or ordinary effect permission. */
export function requireAcceptedPhysicalCleanupDescendantV1(token:AcceptedPhysicalCleanupV1,control:ResourceLeaseControl):void{const accepted=cleanupOf(token),scope=controls.get(control);if(scope?.birth!==accepted.body.birth)throw Error("Actual accepted physical body descendant required");for(let row:ControlScope|undefined=scope;row;row=row.parentControl)if(row===accepted.body)return;throw Error("Actual accepted physical body descendant required");}

/** 📨️ Exposes immutable carried data from the exact live parent for domain-owned closed child projection. */
export function readIssuedScriptProcessEnvelopeV1(invocation:ScriptInvocation):ScriptProcessEnvelope{requireIssuedScriptInvocationV1(invocation);return birthOf(invocation).envelope;}

/** 🪪️ Binds a Native owner to this enrollment's exact issued body control before physical effects. */
export function requireAcceptedPhysicalCleanupControlV1(token:AcceptedPhysicalCleanupV1,control:ResourceLeaseControl):void{const accepted=cleanupOf(token);if(controls.get(control)!==accepted.body)throw Error("Exact accepted physical body control required");}


/** 🧬️ Requires exact issued control ancestry independently of equivalent inherited restrictions. */
export function requireIssuedResourceControlDescendantV1(ancestor:ResourceLeaseControl,control:ResourceLeaseControl):void{requireIssuedResourceControlV1(ancestor);requireIssuedResourceControlV1(control);const expected=controls.get(ancestor)!,policy=schema.definitions.ControlDescendant.const;if(!policy.includeSelf||!policy.liveIssuance||policy.restrictionEquivalence)throw Error("Closed actual control lineage policy required");for(let row:ControlScope|undefined=controls.get(control);row;row=row.parentControl)if(row===expected)return;throw Error("Exact issued resource control descendant required");}
