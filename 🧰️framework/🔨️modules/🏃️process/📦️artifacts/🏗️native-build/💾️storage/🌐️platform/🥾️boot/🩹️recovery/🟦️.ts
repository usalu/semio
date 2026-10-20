import{appendNativeBootLedgerV1,loadNativeBootLedgerV1,nativeBootLedgerCommittedV1}from"./📒️ledger/🟦️.ts";
import{nativeBootPlanDigestV1,planNativeBootRecoveryV1,type NativeBootFactV1,type NativeBootObservationV1,type NativeBootPlanV1,type NativeBootRetainedV1,type NativeBootStepV1}from"./📋️plan/🟦️.ts";
export{bindNativeBackingIdentityV1,bindNativeIdentitiesV1,decideNativeBootFactV1,nativeBackingIdentityProjectionV1,nativeBootPlanDigestV1,parseNativeBootEpochV1,planNativeBootRecoveryV1}from"./📋️plan/🟦️.ts";
export type{NativeBootFactV1,NativeBootObservationV1,NativeBootPlanV1,NativeBootRetainedV1,NativeBootStepV1}from"./📋️plan/🟦️.ts";
export{appendNativeBootLedgerV1,loadNativeBootLedgerV1,nativeBootLedgerCommittedV1,nativeBootLedgerPathV1,readNativeBootLedgerV1}from"./📒️ledger/🟦️.ts";

/** 🔌️ Supplies observation and idempotent effects; the executor never touches a path itself. */
export interface NativeBootPortV1{
 observe():Promise<NativeBootObservationV1>;
 observeScope(scope:string):Promise<readonly NativeBootFactV1[]>;
 execute(step:NativeBootStepV1):Promise<"done"|"skipped">;
}

/** 📈️ Reports bounded progress of one recovery; `total` is the step count of the first plan. */
export type NativeBootProgressV1=Readonly<{version:1;stage:"observe"|"step"|"verify"|"commit";done:number;total:number;action?:NativeBootStepV1["action"];target?:string}>;

/** 🏁️ Reports what one recovery changed and what it deliberately left alone. */
export type NativeBootResultV1=Readonly<{version:1;bootSession:string;outcome:"clean"|"recovered";done:number;skipped:number;refused:readonly NativeBootRetainedV1[];plan:string}>;

/** 🧯️ Wraps the input of one recovery run. */
export interface NativeBootRecoveryInputV1{readonly port:NativeBootPortV1;readonly ledgerPath:string;readonly now:()=>number;readonly signal?:AbortSignal;readonly onProgress?:(event:NativeBootProgressV1)=>void}

const key=(step:Pick<NativeBootStepV1,"family"|"scope"|"action"|"target">)=>[step.family,step.scope,step.action,step.target].join("\u0000");
const refused=(plan:NativeBootPlanV1)=>plan.retained.filter(row=>row.verdict==="refuse");
const bounded=new Set<NativeBootStepV1["action"]>(["retire-settled-claims"]);

/** ♻️ Replans from live facts, journals every effect before the next one and commits only after a verified fixed point; any crash or cancel leaves a plan the next run recomputes. */
export async function recoverNativeBootV1(input:NativeBootRecoveryInputV1):Promise<NativeBootResultV1>{
 const{port,ledgerPath,now,signal,onProgress}=input,progress=(event:Omit<NativeBootProgressV1,"version">)=>onProgress?.(Object.freeze({version:1 as const,...event}));
 signal?.throwIfAborted();
 const observed=await port.observe();signal?.throwIfAborted();
 const first=planNativeBootRecoveryV1(observed),digest=nativeBootPlanDigestV1(first),boot=observed.bootSession,total=first.steps.length;
 progress({stage:"observe",done:0,total});
 if(!total&&nativeBootLedgerCommittedV1(ledgerPath,boot))return Object.freeze({version:1 as const,bootSession:boot,outcome:"clean" as const,done:0,skipped:0,refused:Object.freeze(refused(first)),plan:digest});
 appendNativeBootLedgerV1(ledgerPath,{v:1,type:"begin",boot,at:now(),steps:total,plan:digest});
 let done=0,skipped=0;const executed=new Set<string>();
 for(const step of first.steps){
  progress({stage:"step",done:done+skipped,total,action:step.action,target:step.target});
  if(signal?.aborted){appendNativeBootLedgerV1(ledgerPath,{v:1,type:"abort",boot,at:now(),index:step.index,reason:"cancelled"});signal.throwIfAborted();}
  let outcome:"done"|"skipped",detail="";
  try{
   const facts=await port.observeScope(step.scope),current=planNativeBootRecoveryV1({...observed,facts});
   if(current.steps.some(row=>key(row)===key(step))){outcome=await port.execute(step);}else{outcome="skipped";detail="changed";}
  }catch(error){appendNativeBootLedgerV1(ledgerPath,{v:1,type:"abort",boot,at:now(),index:step.index,reason:signal?.aborted?"cancelled":"failed"});throw error;}
  appendNativeBootLedgerV1(ledgerPath,{v:1,type:"step",boot,index:step.index,action:step.action,target:step.target,outcome,detail});
  if(outcome==="done"){done++;if(!bounded.has(step.action))executed.add(key(step));}else skipped++;
 }
 signal?.throwIfAborted();
 progress({stage:"verify",done:done+skipped,total});
 const after=planNativeBootRecoveryV1(await port.observe());
 if(after.steps.some(row=>executed.has(key(row)))){appendNativeBootLedgerV1(ledgerPath,{v:1,type:"abort",boot,at:now(),index:total,reason:"failed"});throw Error("Boot recovery did not reach a fixed point");}
 appendNativeBootLedgerV1(ledgerPath,{v:1,type:"commit",boot,at:now(),done,skipped,retained:after.retained.length});
 progress({stage:"commit",done:done+skipped,total});
 return Object.freeze({version:1 as const,bootSession:boot,outcome:total?"recovered" as const:"clean" as const,done,skipped,refused:Object.freeze(refused(after)),plan:digest});
}

/** 🩺️ Runs a recovery only when this boot has not been healed yet or a cheap residue probe finds pending work; otherwise costs one small file read. */
export async function healNativeBootV1(input:Readonly<{ledgerPath:string;bootSession:string;residue:()=>boolean;recover:()=>Promise<NativeBootResultV1>}>):Promise<"healthy"|NativeBootResultV1>{
 if(!input.residue()&&nativeBootLedgerCommittedV1(input.ledgerPath,input.bootSession))return"healthy";
 return input.recover();
}

/** 📖️ Reports the ledger state of a scope without changing it. */
export function readNativeBootLedgerStateV1(ledgerPath:string):ReturnType<typeof loadNativeBootLedgerV1>["state"]{return loadNativeBootLedgerV1(ledgerPath).state;}
