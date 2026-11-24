import {createHash} from "node:crypto";
import {stat} from "node:fs/promises";
import {homedir} from "node:os";
import {resolve} from "node:path";
import {isDiskFullV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts";
import {refuseLowSpaceV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/💽️disk/🟦️.ts";
import {casHostAreasV1} from "../📂️areas/🟦️.ts";
import {collectCasV1,readCasUsageHintV1,type CasAreaSourceV1,type CasCollectionV1,type CasUsageHintV1} from "../🧹️collector/🟦️.ts";
import {casPartitionNeedBytesV1,casPartitionPolicyForV1,casReserveBytesV1,casPartitionUsedFractionV1,decideCasEnforcementV1,measureCasPartitionV1,type CasEnforcementDecisionV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
import {CAS_POLICY_V1,requireCasPolicyV1,type CasPolicyV1} from "../🧬️schema/🟦️.ts";
import {stampCasTransientsV1} from "../📌️pins/🟦️.ts";
import {casUsageV1} from "../🗃️store/📏️usage/🟦️.ts";
import {casTextV1} from "../🧬️schema/🟦️.ts";

/** 🔌️ What a guard touches, replaceable by tests: the clock, the partition measurement (`undefined` is "not measurable"), the collection and the scan of the store. */
export type CasGuardPortsV1=Readonly<{
 now?():number;home?:string;refuseOverCap?:boolean;
 report?(facts:Readonly<{partition:string;usedFraction:number}>):void;
 partition?(root:string):CasPartitionV1|undefined;
 collect?(request:Readonly<{root:string;policy:CasPolicyV1;partition:CasPartitionV1|undefined;signal?:AbortSignal}>):Promise<CasCollectionV1>;
 usage?(root:string):Promise<number>;
}>;
/** 🧯️ A write refused because the store stays over its cap after the bounded synchronous collection; the caller carries on without the cache. */
export class CasCapRefusalV1 extends Error{
 readonly usedBytes:number;readonly capBytes:number;readonly reserve?:Readonly<{role:string;reserveBytes:number;freeBytes:number}>;
 constructor(usedBytes:number,capBytes:number,reserve?:Readonly<{role:string;reserveBytes:number;freeBytes:number}>){
  const gib=(bytes:number):number=>Math.round(bytes/1073741824*100)/100,values={usedGib:gib(usedBytes),capGib:gib(capBytes),...reserve?{role:reserve.role,reserveGib:gib(reserve.reserveBytes),freeGib:gib(reserve.freeBytes)}:{}},key=reserve?"reserveRefused":"capRefused";
  super(casTextV1(key,"en",values)+" / "+casTextV1(key,"de",values));
  this.name="CasCapRefusalV1";this.usedBytes=usedBytes;this.capBytes=capBytes;if(reserve)this.reserve=reserve;
 }
}
/** 🚦️ The admission of one store root: judges a write or an adoption before it adds to the partition, counts what this process added, and runs the collection that keeps the cap and the partition watermarks. */
export type CasGuardV1=Readonly<{
 settle():Promise<void>;
 admit(incomingBytes:number,signal?:AbortSignal,transient?:boolean):Promise<CasEnforcementDecisionV1>;
 admitSync(incomingBytes:number):CasEnforcementDecisionV1;
 account(bytes:number):void;
 remeasure(signal?:AbortSignal):Promise<void>;
 relieve(error:unknown):void;
 enforce(signal?:AbortSignal):Promise<CasCollectionV1|undefined>;
 sweep(signal?:AbortSignal,stepBytes?:number):Promise<CasCollectionV1|undefined>;
}>;

type FlightV1={owner:object;scope:string;automatic:boolean;task:Promise<CasCollectionV1|undefined>};
type StateV1={base?:number;admitted:number;hintAt?:number;checkedAt?:number;hint?:CasUsageHintV1;checking?:Promise<void>;scope?:{key:string;task:Promise<string|undefined>};critical?:CasUsageHintV1;running?:FlightV1;direct?:FlightV1;measuring?:Promise<void>;background?:Promise<void>;criticalBackground?:Promise<void>;sweeping?:Promise<CasCollectionV1|undefined>};
const nativeCollector={};
const states=new Map<string,StateV1>();
const stateOf=(root:string):StateV1=>{let state=states.get(root);if(!state){state={admitted:0};states.set(root,state);}return state;};

const partitionScopeV1=(policy:CasPolicyV1,partition:CasPartitionV1|undefined):readonly unknown[]=>partition===undefined?[]:[partition.partition,partition.totalBytes,casPartitionNeedBytesV1(partition,policy)>0,casPartitionUsedFractionV1(partition)>=policy.partition.criticalUsedFraction];
const policyScopeV1=(policy:CasPolicyV1):readonly unknown[]=>[policy.version,policy.capBytes,policy.highWatermark,policy.lowWatermark,policy.minAgeMs,policy.fanOutHexLength,policy.partition.highUsedFraction,policy.partition.lowUsedFraction,policy.partition.criticalUsedFraction,policy.leaseMaxMs,policy.touchGranularityMs,policy.temporaryMaxAgeMs,policy.ledgerQuietMs,policy.lockStaleMs,policy.progressMs,policy.checkEvery,policy.areas.map(row=>[row.id,row.capBytes,row.maxAgeMs])];
const directoryScopeV1=async(path:string):Promise<readonly string[]|null|undefined>=>{
 try{const info=await stat(path,{bigint:true});return info.isDirectory()&&info.ino!==0n?[String(info.dev),String(info.ino)]:undefined;}
 catch(error){return ["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException)?.code??"")?null:undefined;}
};
const hostAreasV1=async(home:string):Promise<{areas:readonly CasAreaSourceV1[];known:boolean}>=>{
 let known=true;const areas=await casHostAreasV1(home,()=>{known=false;});return{areas,known};
};
async function automaticScopeV1(root:string,policy:CasPolicyV1,partition:CasPartitionV1|undefined,areas:readonly CasAreaSourceV1[]):Promise<string|undefined>{
 const owner=await directoryScopeV1(root);if(!owner)return undefined;
 const digest=createHash("sha256").update(JSON.stringify(["cas-automatic-v1",resolve(root),owner,policyScopeV1(policy),partitionScopeV1(policy,partition)]));
 for(const area of areas){
  const identity=await directoryScopeV1(area.directory);if(identity===undefined)return undefined;
  digest.update(JSON.stringify([area.id,resolve(area.directory),identity,area.skip?.source??null,area.skip?.flags??null]));
 }
 return digest.digest("hex");
}

/** ⏱️ A timer that can be stopped without keeping the process alive. */
const delay=(milliseconds:number):{done:Promise<undefined>;stop():void}=>{
 let timer:ReturnType<typeof setTimeout>|undefined;
 const done=new Promise<undefined>(accept=>{timer=setTimeout(()=>accept(undefined),milliseconds);timer.unref?.();});
 return{done,stop:()=>clearTimeout(timer)};
};

/** ⏳️ Lets a writer stop waiting when its own signal fires while the shared collection it waited for goes on: a collection of a large store takes longer than one write may wait, and a scan cancelled by every timed-out write would never finish. */
const within=<T>(task:Promise<T>,signal?:AbortSignal):Promise<T>=>signal===undefined?task:new Promise<T>((accept,reject)=>{
 if(signal.aborted){reject(signal.reason);return;}
 const stop=()=>reject(signal.reason);
 signal.addEventListener("abort",stop,{once:true});
 task.then(value=>{signal.removeEventListener("abort",stop);accept(value);},error=>{signal.removeEventListener("abort",stop);reject(error);});
});

/** 🚦️ Guards writes and shares one running collection per store root. Only completed scans of the relevant scope give cooldown credit. A critical write waits for relief or receives the low-space refusal. High watermarks start background work. Reserve rule: a write that would leave less than the hard free-space reserve of the partition, transient inputs included, is admitted only after the shared unreferenced sweep (expired inputs and data nobody references; never a held, pinned, leased or linked object) freed enough to keep the reserve, else it is refused: below the reserve the store never grows and never evicts a held entry to make room. @see ../🌡️partition/🟦️.ts */
export function createCasGuardV1(options:Readonly<{root:string;policy?:CasPolicyV1}&CasGuardPortsV1>):CasGuardV1{
 const root=options.root,policy=requireCasPolicyV1(options.policy??CAS_POLICY_V1),now=options.now??Date.now,home=options.home??homedir(),state=stateOf(root);
 const measure=options.partition??measureCasPartitionV1,usage=options.usage??(async(path:string)=>(await casUsageV1(path)).bytes);
 const owner=options.collect===undefined?nativeCollector:{};
 let customCompletion:{scope:string;atMs:number}|undefined;
 const fresh=(atMs:number,at=now()):boolean=>atMs<=at&&at-atMs<policy.minAgeMs;
 const quoteScope=(quote:ReturnType<typeof decide>):string=>JSON.stringify([home,policyScopeV1(quote.effective),partitionScopeV1(quote.effective,quote.partition),quote.decision.reason]);
 const collect=async(request:Parameters<NonNullable<CasGuardPortsV1["collect"]>>[0],automatic:boolean,completedAutomatic:(hint:CasUsageHintV1)=>void)=>{
  if(options.collect!==undefined)return options.collect(request);
  const {areas,known}=await hostAreasV1(home);
  return collectCasV1({root,policy:request.policy,incremental:automatic,role:automatic?"guard-background":"guard-admission",areas,partition:request.partition,signal:request.signal,now,
   ...automatic&&known&&request.policy.minAgeMs>0?{automaticScope:()=>automaticScopeV1(root,request.policy,request.partition,areas),completedAutomatic}:{}});
 };
 const run=(quote:ReturnType<typeof decide>,signal?:AbortSignal,automatic=true):FlightV1=>{
  if(state.running!==undefined)return state.running;
  const scope=quoteScope(quote),critical=quote.decision.reason==="partition-critical";
  const task=Promise.resolve().then(async()=>{
   let stamp:CasUsageHintV1|undefined;
   const result=await collect({root,policy:quote.effective,partition:quote.partition,signal},automatic,hint=>{stamp=hint;});
   if(result.status!=="locked"&&!result.partial&&!result.dryRun&&!result.lockless&&!signal?.aborted){
    state.checkedAt=undefined;
    if(automatic){
     if(options.collect!==undefined)customCompletion={scope,atMs:now()};
     else if(critical&&stamp!==undefined)state.critical=stamp;
    }
   }
   return result;
  }).catch(()=>undefined).finally(()=>{state.running=undefined;});
  return state.running={owner,scope,automatic,task};
 };
 const matches=(flight:FlightV1,scope:string):boolean=>flight.owner===owner&&flight.scope===scope;
 const direct=(signal?:AbortSignal):Promise<CasCollectionV1|undefined>=>{
  try{
   signal?.throwIfAborted();
   const quote=decide(bytes(),0),running=state.running;
   if(running===undefined)return run(quote,signal,false).task;
   const scope=quoteScope(quote);
   if(matches(running,scope))return running.task;
   if(state.direct!==undefined)return matches(state.direct,scope)?state.direct.task:Promise.resolve(undefined);
   const task=running.task.then(()=>{
    signal?.throwIfAborted();
    const current=decide(bytes(),0),next=state.running;
    if(next!==undefined&&!matches(next,quoteScope(current)))return undefined;
    return run(current,signal,false).task;
   }).catch(()=>undefined).finally(()=>{state.direct=undefined;});
   state.direct={owner,scope,automatic:false,task};
   return task;
  }catch{return Promise.resolve(undefined);}
 };
 const measureStore=():void=>{
  state.measuring??=usage(root).then(bytes=>{state.base=bytes;state.admitted=0;state.hintAt=now();},()=>undefined).finally(()=>{state.measuring=undefined;});
 };
 const bytes=()=>state.base===undefined?undefined:state.base+state.admitted;
 const storeBytes=async():Promise<number|undefined>=>{
  const at=now();
  if(state.checking===undefined&&(state.checkedAt===undefined||at<state.checkedAt||at-state.checkedAt>=policy.touchGranularityMs)){
   state.checkedAt=at;state.scope=undefined;
   state.checking=readCasUsageHintV1(root).then(hint=>{
    state.hint=hint;
    if(hint===undefined)return;
    const measuredAt=Math.min(hint.atMs,now());
    if(state.hintAt===undefined||measuredAt>state.hintAt){state.hintAt=measuredAt;state.base=hint.bytes;state.admitted=0;}
   }).finally(()=>{state.checking=undefined;});
  }
  if(state.checking!==undefined)await state.checking;
  return bytes();
 };
 const decide=(storeBytesNow:number|undefined,incomingBytes:number)=>{
  const partition=measure(root),effective=casPartitionPolicyForV1(policy,partition,storeBytesNow);
  return{partition,effective,decision:decideCasEnforcementV1({policy:effective,storeBytes:storeBytesNow,incomingBytes,partition,collected:false})};
 };
 const usableCompletion=(quote:ReturnType<typeof decide>,hint:CasUsageHintV1|undefined,at=now()):hint is CasUsageHintV1&{automaticScope:string}=>{
  if(!hint?.automaticScope||!fresh(hint.atMs,at))return false;
  switch(quote.decision.reason){
   case "cap-exceeded":return hint.bytes>quote.effective.capBytes;
   case "cap-high":return (bytes()??0)-hint.bytes<quote.effective.incrementBytes/2;
   case "partition-high":return casPartitionNeedBytesV1(quote.partition,quote.effective)>0;
   case "partition-critical":return state.critical?.atMs===hint.atMs&&state.critical.automaticScope===hint.automaticScope;
   default:return false;
  }
 };
 const locallyCovered=(quote:ReturnType<typeof decide>):boolean=>customCompletion!==undefined&&fresh(customCompletion.atMs)&&customCompletion.scope===quoteScope(quote);
 const verifyCompletion=async(quote:ReturnType<typeof decide>,incomingBytes:number,signal?:AbortSignal)=>{
  if(locallyCovered(quote))return {quote,covered:true};
  const hint=state.hint;if(!usableCompletion(quote,hint))return {quote,covered:false};
  const partitionKey=JSON.stringify(partitionScopeV1(quote.effective,quote.partition)),key=JSON.stringify([hint.automaticScope,home,policyScopeV1(quote.effective),partitionKey]);
  if(state.scope?.key!==key)state.scope={key,task:hostAreasV1(home).then(({areas,known})=>known?automaticScopeV1(root,quote.effective,quote.partition,areas):undefined)};
  const match=await state.scope.task===hint.automaticScope;
  signal?.throwIfAborted();
  const current=decide(bytes(),incomingBytes);
  const covered=match&&state.hint===hint&&usableCompletion(quote,hint)&&current.decision.reason===quote.decision.reason&&JSON.stringify(partitionScopeV1(current.effective,current.partition))===partitionKey;
  return {quote:current,covered};
 };
 const refuse=(partition:CasPartitionV1|undefined):never=>{throw refuseLowSpaceV1({partition:partition?.partition??root,freeBytes:partition?.freeBytes??0,totalBytes:partition?.totalBytes??1,limitFraction:casPartitionPolicyForV1(policy,partition,undefined).partition.criticalUsedFraction});};
 const afterCollection=(quote:ReturnType<typeof decide>,incomingBytes:number)=>decideCasEnforcementV1({
  policy:quote.effective,storeBytes:bytes(),incomingBytes,partition:quote.partition,collected:true
 });
 const guard:CasGuardV1={
  async admit(incomingBytes,signal,transient=false){
   let triedCap=false,triedCritical=false,waitedForOtherScope=false,fullTried=false,attempts=0;
   for(;;){
    signal?.throwIfAborted();
    let quote=decide(await storeBytes(),incomingBytes),covered=false;
    signal?.throwIfAborted();
    if(quote.decision.action==="collect-now"&&!(quote.decision.reason==="partition-critical"?triedCritical:triedCap))
     ({quote,covered}=await verifyCompletion(quote,incomingBytes,signal));
    const {decision,partition}=quote;
    const reserveBytes=partition===undefined?0:casReserveBytesV1(policy,partition),intoReserve=partition!==undefined&&partition.freeBytes-incomingBytes<reserveBytes;
    if(options.refuseOverCap===true&&(decision.reason==="cap-exceeded"&&(!transient||intoReserve)||intoReserve&&decision.reason==="partition-high")){
     if(fullTried||attempts>=3||usableCompletion(quote,state.hint))throw new CasCapRefusalV1((bytes()??0)+incomingBytes,quote.effective.capBytes,intoReserve?{role:partition!.role??policy.sizing.defaultRole,reserveBytes,freeBytes:partition!.freeBytes}:undefined);
     triedCap=true;attempts++;
     if(intoReserve&&options.collect===undefined){
      const sweeping=state.sweeping??=guard.sweep(undefined,Number.POSITIVE_INFINITY).finally(()=>{state.sweeping=undefined;}),budget=delay(quote.effective.admissionBudgetMs);
      try{fullTried=await within(Promise.race([sweeping.then(()=>true),budget.done.then(()=>false)]),signal);}finally{budget.stop();}
      continue;
     }
     const flight=run(quote,undefined,false);fullTried=!flight.automatic;
     const task=flight.task,budget=delay(quote.effective.admissionBudgetMs);
     try{await within(Promise.race([task,budget.done]),signal);}finally{budget.stop();}
     continue;
    }
    if(decision.action==="collect-now"){
     const critical=decision.reason==="partition-critical";
     if((critical?triedCritical:triedCap)||covered)
      return critical?refuse(partition):afterCollection(quote,incomingBytes);
     const running=state.running;
     if(running!==undefined&&(!running.automatic||!matches(running,quoteScope(quote)))){
      if(waitedForOtherScope)return critical?refuse(partition):afterCollection(quote,incomingBytes);
      waitedForOtherScope=true;await running.task;continue;
     }
     if(critical)triedCritical=true;else triedCap=true;
     if(partition&&running===undefined)options.report?.({partition:partition.partition,usedFraction:casPartitionUsedFractionV1(partition,incomingBytes)});
     await within(run(quote).task,signal);continue;
    }
    if(decision.action==="collect-background"){
     if(triedCap||triedCritical||locallyCovered(quote))return afterCollection(quote,incomingBytes);
     if(decision.reason==="cap-exceeded"&&usableCompletion(quote,state.hint)){const checked=await verifyCompletion(quote,incomingBytes,signal);if(checked.covered)return afterCollection(checked.quote,incomingBytes);if(checked.quote.decision.reason!=="cap-exceeded"){quote=checked.quote;continue;}}
     if(!usableCompletion(quote,state.hint))void run(quote);
     else state.background??=verifyCompletion(quote,incomingBytes).then(async next=>{
      if(!next.covered&&!signal?.aborted&&(next.quote.decision.action==="collect-now"||next.quote.decision.action==="collect-background"))await run(next.quote).task;
     }).finally(()=>{state.background=undefined;});
    }
    if(decision.action==="measure")measureStore();
    return decision;
   }
  },
  admitSync(incomingBytes){
   const{decision,partition}=decide(bytes(),incomingBytes);
   if(decision.action==="collect-now"&&decision.reason==="partition-critical"){
    state.criticalBackground??=guard.admit(incomingBytes).then(()=>undefined,()=>undefined).finally(()=>{state.criticalBackground=undefined;});
    return refuse(partition);
   }
   if(decision.action==="collect-now"||decision.action==="collect-background"||decision.action==="measure"){
    state.background??=storeBytes().then(async bytesNow=>{
     const {quote,covered}=await verifyCompletion(decide(bytesNow,incomingBytes),incomingBytes);
     if(quote.decision.action==="measure")measureStore();
     else if(!covered&&(quote.decision.action==="collect-now"||quote.decision.action==="collect-background"))await run(quote).task;
    }).finally(()=>{state.background=undefined;});
   }
   return decision;
  },
  async settle(){await state.criticalBackground;await state.background;await state.direct?.task;await state.running?.task;await state.measuring;},
  account(bytes){state.admitted+=bytes;},
  async remeasure(signal){const measured=await usage(root);signal?.throwIfAborted();state.base=measured;state.admitted=0;state.hintAt=now();state.checkedAt=now();},
  relieve(error){if(isDiskFullV1(error))void direct();},
  enforce:direct,
  async sweep(signal,stepBytes=policy.sweepStepBytes){
   if(options.collect!==undefined)return undefined;
   const partition=measure(root),effective=casPartitionPolicyForV1(policy,partition,bytes());
   return collectCasV1({root,policy:effective,role:"guard-sweep",areas:[],unreferencedOnly:true,unreferencedStepBytes:stepBytes,signal,now}).catch(()=>undefined);
  },
 };
 return Object.freeze(guard);
}

/** ⏱️ One tick of the service that keeps the host store inside its cap and its partition inside its watermarks: while the partition's free space is below the hard reserve it sweeps first, without a step bound and without the collection lock (the unreferenced pass never touches a held, pinned, leased or linked object), so a release build's writes never outrun it; it then measures the store itself only when no sweep left a usage hint within two sweep periods (a walk of the store is the most expensive read there is), then judges like a write of no bytes, so it collects (down to the low watermarks) only when the partition or the store is above its high watermark, measures a store without a usage hint, waits for what it started and honours the cancellation of the service. A partition that stays at its critical share after the collection raises the typed low-space refusal, which the service logs. Independent of every watermark it then sweeps (`guard.sweep`, every `sweepEveryMs`, at once again while a step freed its full `sweepStepBytes`): objects nobody references and expired transient inputs, older than the in-flight grace, go in bounded steps, never a held, pinned, leased or linked entry. */
export async function casEnforceTickV1(options:Readonly<{root:string;policy?:CasPolicyV1;signal?:AbortSignal;sweep?:boolean}&CasGuardPortsV1>):Promise<void>{
 const guard=createCasGuardV1(options),partition=(options.partition??measureCasPartitionV1)(options.root);
 if(options.sweep!==false&&partition!==undefined&&partition.freeBytes<casReserveBytesV1(requireCasPolicyV1(options.policy??CAS_POLICY_V1),partition)){await guard.sweep(options.signal,Number.POSITIVE_INFINITY);sweepState.set(options.root,{atMs:(options.now??Date.now)(),more:false});}
 const policy=requireCasPolicyV1(options.policy??CAS_POLICY_V1),at=(options.now??Date.now)(),last=sweptAt.get(options.root),hint=options.usage===undefined?await readCasUsageHintV1(options.root):undefined;
 if(hint===undefined||at<hint.atMs||at-hint.atMs>policy.sweepEveryMs*2)await guard.remeasure(options.signal);
 await guard.admit(0,options.signal);await guard.settle();
 if((last===undefined||at<last||at-last>=policy.transientLeaseMs)&&await stampCasTransientsV1(options.root)!==""){sweptAt.set(options.root,at);await guard.enforce(options.signal);await guard.settle();}
 const swept=sweepState.get(options.root);
 if(options.sweep!==false&&(swept===undefined||swept.more||at<swept.atMs||at-swept.atMs>=policy.sweepEveryMs)){
  const result=await guard.sweep(options.signal);
  sweepState.set(options.root,{atMs:at,more:result!==undefined&&result.status==="collected"&&!result.partial&&result.freedBytes>=policy.sweepStepBytes*0.9});
 }
}
const sweptAt=new Map<string,number>();
const sweepState=new Map<string,{atMs:number;more:boolean}>();

/** 🧪️ Forgets the shared state of every root; for tests that open many roots in one process. */
export const resetCasGuardsV1=():void=>{states.clear();sweptAt.clear();sweepState.clear();};
