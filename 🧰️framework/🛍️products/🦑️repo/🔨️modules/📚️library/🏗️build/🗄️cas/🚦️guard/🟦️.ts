import {homedir} from "node:os";
import {isDiskFullV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts";
import {refuseLowSpaceV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/💽️disk/🟦️.ts";
import {casHostAreasV1} from "../📂️areas/🟦️.ts";
import {collectCasV1,readCasUsageHintV1,type CasCollectionV1} from "../🧹️collector/🟦️.ts";
import {casPartitionUsedFractionV1,decideCasEnforcementV1,measureCasPartitionV1,type CasEnforcementDecisionV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
import {CAS_POLICY_V1,requireCasPolicyV1,type CasPolicyV1} from "../🧬️schema/🟦️.ts";
import {casUsageV1} from "../🗃️store/📏️usage/🟦️.ts";

/** 🔌️ What a guard touches, replaceable by tests: the clock, the partition measurement (`undefined` is "not measurable"), the collection and the scan of the store. */
export type CasGuardPortsV1=Readonly<{
 now?():number;home?:string;
 report?(facts:Readonly<{partition:string;usedFraction:number}>):void;
 partition?(root:string):CasPartitionV1|undefined;
 collect?(request:Readonly<{root:string;policy:CasPolicyV1;partition:CasPartitionV1|undefined;signal?:AbortSignal}>):Promise<CasCollectionV1>;
 usage?(root:string):Promise<number>;
}>;
/** 🚦️ The admission of one store root: judges a write or an adoption before it adds to the partition, counts what this process added, and runs the collection that keeps the cap and the partition watermarks. */
export type CasGuardV1=Readonly<{
 settle():Promise<void>;
 admit(incomingBytes:number,signal?:AbortSignal):Promise<CasEnforcementDecisionV1>;
 admitSync(incomingBytes:number):CasEnforcementDecisionV1;
 account(bytes:number):void;
 relieve(error:unknown):void;
 enforce(signal?:AbortSignal):Promise<CasCollectionV1|undefined>;
}>;

type StateV1={base?:number;admitted:number;hintAt?:number;checkedAt?:number;collectedAt?:number;running?:Promise<CasCollectionV1|undefined>;measuring?:Promise<void>;failure?:unknown};
const states=new Map<string,StateV1>();
const stateOf=(root:string):StateV1=>{let state=states.get(root);if(!state){state={admitted:0};states.set(root,state);}return state;};

/** 🚦️ Opens the guard of a store root. The state (bytes added since the last finished collection, the moment of the last collection, the running collection) is shared by every guard of the root in this process, so a collection is single-flight however many stores write, and a collection that ran within the minimum age is not started again (nothing younger is evictable, so it could free nothing more). A write waits for a collection only when the partition is at its critical share or the store is above its cap; at the high watermarks the collection runs in the background and the write goes on. The cap is judged on the usage hint of the last finished collection plus what this process added since; a store without a hint is measured in the background. @see ../🌡️partition/🟦️.ts */
export function createCasGuardV1(options:Readonly<{root:string;policy?:CasPolicyV1}&CasGuardPortsV1>):CasGuardV1{
 const root=options.root,policy=requireCasPolicyV1(options.policy??CAS_POLICY_V1),now=options.now??Date.now,home=options.home??homedir(),state=stateOf(root);
 const measure=options.partition??measureCasPartitionV1,usage=options.usage??(async(path:string)=>(await casUsageV1(path)).bytes);
 const collect=options.collect??(async request=>collectCasV1({root:request.root,policy:request.policy,areas:await casHostAreasV1(home),partition:request.partition,signal:request.signal}));
 const collected=():boolean=>state.collectedAt!==undefined&&now()-state.collectedAt<policy.minAgeMs;
 const run=(signal?:AbortSignal):Promise<CasCollectionV1|undefined>=>{
  state.running??=(async()=>{
   try{const result=await collect({root,policy,partition:measure(root),signal});state.hintAt=undefined;state.checkedAt=undefined;state.admitted=0;return result;}
   catch(error){state.failure=error;return undefined;}
   finally{state.collectedAt=now();state.running=undefined;}
  })();
  return state.running;
 };
 const measureStore=():void=>{
  state.measuring??=usage(root).then(bytes=>{state.base=bytes;state.admitted=0;state.hintAt=now();},()=>undefined).finally(()=>{state.measuring=undefined;});
 };
 const storeBytes=async():Promise<number|undefined>=>{
  const at=now();
  if(state.checkedAt===undefined||at-state.checkedAt>=policy.touchGranularityMs){
   state.checkedAt=at;const hint=await readCasUsageHintV1(root);
   if(hint&&(state.hintAt===undefined||hint.atMs>state.hintAt)){state.hintAt=hint.atMs;state.base=hint.bytes;state.admitted=0;}
  }
  return state.base===undefined?undefined:state.base+state.admitted;
 };
 const decide=(storeBytesNow:number|undefined,incomingBytes:number):{decision:CasEnforcementDecisionV1;partition:CasPartitionV1|undefined}=>{
  const partition=measure(root);
  return{partition,decision:decideCasEnforcementV1({policy,storeBytes:storeBytesNow,incomingBytes,partition,collected:collected()})};
 };
 const refuse=(partition:CasPartitionV1|undefined):never=>{throw refuseLowSpaceV1({partition:partition?.partition??root,freeBytes:partition?.freeBytes??0,totalBytes:partition?.totalBytes??1,limitFraction:policy.partition.criticalUsedFraction});};
 const guard:CasGuardV1={
  async admit(incomingBytes,signal){
   for(let pass=0;pass<3;pass++){
    signal?.throwIfAborted();
    const{decision,partition}=decide(await storeBytes(),incomingBytes);
    if(decision.action==="refuse")return refuse(partition);
    if(decision.action==="collect-now"){
     if(partition&&state.running===undefined)options.report?.({partition:partition.partition,usedFraction:casPartitionUsedFractionV1(partition,incomingBytes)});
     await run(signal);continue;
    }
    if(decision.action==="collect-background")void run();
    if(decision.action==="measure")measureStore();
    return decision;
   }
   return decideCasEnforcementV1({policy,storeBytes:undefined,incomingBytes,partition:undefined,collected:true});
  },
  admitSync(incomingBytes){
   const{decision,partition}=decide(state.base===undefined?undefined:state.base+state.admitted,incomingBytes);
   if(decision.action==="collect-now"||decision.action==="collect-background")void run();
   if(decision.action==="measure")measureStore();
   if(decision.action==="refuse"||decision.action==="collect-now"&&decision.reason==="partition-critical")return refuse(partition);
   return decision;
  },
  async settle(){await state.running;await state.measuring;},
  account(bytes){state.admitted+=bytes;},
  relieve(error){if(isDiskFullV1(error))void run();},
  enforce:signal=>run(signal),
 };
 return Object.freeze(guard);
}

/** ⏱️ One tick of the service that keeps the host store inside its cap and its partition inside its watermarks: it judges like a write of no bytes, so it collects (down to the low watermarks) only when the partition or the store is above its high watermark, measures a store without a usage hint, waits for what it started and honours the cancellation of the service. A partition that stays at its critical share after the collection raises the typed low-space refusal, which the service logs. */
export async function casEnforceTickV1(options:Readonly<{root:string;policy?:CasPolicyV1;signal?:AbortSignal}&CasGuardPortsV1>):Promise<void>{
 const guard=createCasGuardV1(options);
 await guard.admit(0,options.signal);await guard.settle();
}

/** 🧪️ Forgets the shared state of every root; for tests that open many roots in one process. */
export const resetCasGuardsV1=():void=>{states.clear();};
