import{mkdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import type{HostGrantV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ The numbers of the readiness stage claim.
 * @see ./🧬️schema/🔣️.json */
export const READY_STAGE_CLAIM_POLICY_V1=schema.definitions.Policy.const;
const P=READY_STAGE_CLAIM_POLICY_V1;

/** 🧳️ What the readiness stage of a machine is entitled to hold and holds now (megabytes of its process tree), written by the readiness loop beside the host capacity files and read by the owner puller and the remote-execution agent. */
export type ReadyStageClaimV1=Readonly<{version:1;stage:string;pid:number;entitlementMegabytes:number;observedMegabytes:number;atMs:number}>;

/** 🕰️ One earlier run of a stage as the readiness timings keep it: only what the entitlement learns from. */
export type ReadyStageRunV1=Readonly<{stage:string;outcome:string;peakMegabytes:number|null}>;

/** 🪑️ The memory a stage keeps for itself while it runs on a host of `totalMegabytes`: the `percentile` of the peaks of its completed earlier runs plus the margin (a failed, stalled or interrupted run grew under a thrashing host and teaches nothing, as for every learned size), `shareBasisPoints` of the machine while no run completed, never below `minimumMegabytes` and never above `maximumShareBasisPoints` of the machine, so a 16 GB worker keeps a third of itself for the stage and a 64 GB one does not hoard.
 * @see ../../🧪️tests/🪑️stage-claim/🐍️.py */
export function readyStageEntitlementV1(input:Readonly<{totalMegabytes:number;stage:string;runs:readonly ReadyStageRunV1[]}>,policy:typeof P=P):number{
 const ceiling=Math.floor(input.totalMegabytes*policy.maximumShareBasisPoints/10000),peaks=input.runs.filter(run=>run.stage===input.stage&&run.outcome==="ok"&&run.peakMegabytes!==null&&Number.isFinite(run.peakMegabytes)&&run.peakMegabytes>0).slice(-policy.samples).map(run=>run.peakMegabytes!).sort((a,b)=>a-b);
 const wanted=peaks.length===0?Math.floor(input.totalMegabytes*policy.shareBasisPoints/10000):Math.ceil(peaks[Math.max(0,Math.ceil(peaks.length*policy.percentile/100)-1)]!*(100+policy.marginPercent)/100);
 return Math.min(ceiling,Math.max(policy.minimumMegabytes,wanted));
}

/** 🧺️ Whether a claim still speaks for a running stage: written within `freshMilliseconds`, so a loop that died never keeps memory back for ever. */
export const readyStageClaimFreshV1=(claim:ReadyStageClaimV1,nowMs:number,policy:typeof P=P):boolean=>nowMs-claim.atMs<=policy.freshMilliseconds;

/** 🎒️ The part of the entitlement the stage has not grown into yet: what the others must leave free besides what the machine already reports as used; none for a stale claim.
 * @see ../../🧪️tests/🪑️stage-claim/🐍️.py */
export const readyStageHoldV1=(claim:ReadyStageClaimV1|undefined,nowMs:number,policy:typeof P=P):number=>claim===undefined||!readyStageClaimFreshV1(claim,nowMs,policy)?0:Math.max(0,claim.entitlementMegabytes-claim.observedMegabytes);

/** 🪪️ The stage as one grant of the machine's own work: it books a core and the memory it holds now, observed, so it is counted once.
 * @see readyStageHoldV1 */
export const readyStageGrantV1=(claim:ReadyStageClaimV1|undefined,nowMs:number,policy:typeof P=P):HostGrantV1|undefined=>claim===undefined||!readyStageClaimFreshV1(claim,nowMs,policy)?undefined:{id:"ready-stage",operation:"ready-stage",jobs:1,cpu:1,reservedMegabytes:claim.observedMegabytes,admittedAtMs:claim.atMs,observedMegabytes:claim.observedMegabytes,pid:claim.pid,bootAtMs:0,label:"ready-stage"};

/** 🧷️ A host view with the running stage added, so the puller and the agent read one machine whose stage will still grow: the stage is a grant, and the memory it is entitled to but has not grown into is taken from the available memory of the host (the used memory of the sample is what the machine holds now, the hold is what it will hold soon). Every admission built on the view (the pool's children, the agent's offloads, the load report) then leaves the stage its room without any rule of its own. A view without a fresh claim is returned as it is.
 * @see ../../🧪️tests/🪑️stage-claim/🐍️.py */
export function withReadyStageV1<V extends Readonly<{host:Readonly<{availableMegabytes:number}>;grants:readonly HostGrantV1[]}>>(view:V,claim:ReadyStageClaimV1|undefined,nowMs:number,policy:typeof P=P):V&Readonly<{stage?:ReadyStageClaimV1}>{
 const grant=readyStageGrantV1(claim,nowMs,policy);
 if(grant===undefined||claim===undefined)return view;
 return{...view,host:{...view.host,availableMegabytes:Math.max(0,view.host.availableMegabytes-readyStageHoldV1(claim,nowMs,policy))},grants:[...view.grants.filter(row=>row.id!==grant.id),grant],stage:claim};
}

/** 📖️ Reads the claim of this machine, `undefined` for a missing, unreadable, malformed or stale file. */
export function readReadyStageClaimV1(directory:string,nowMs:number=Date.now(),policy:typeof P=P):ReadyStageClaimV1|undefined{
 try{
  const value=JSON.parse(readFileSync(join(directory,policy.fileName),"utf8"));
  if(validateJsonSchemaSubset(schema.definitions.Claim,value,schema).length!==0)return undefined;
  return readyStageClaimFreshV1(value as ReadyStageClaimV1,nowMs,policy)?value as ReadyStageClaimV1:undefined;
 }catch{return undefined;}
}

/** 💾️ Writes the claim atomically (a temporary file, then a rename), so a reader never sees half of it. */
export function writeReadyStageClaimV1(directory:string,claim:ReadyStageClaimV1,policy:typeof P=P):void{
 mkdirSync(directory,{recursive:true});
 const file=join(directory,policy.fileName),temporary=`${file}.${process.pid}.tmp`;
 writeFileSync(temporary,JSON.stringify(claim));
 renameSync(temporary,file);
}

/** 🧹️ Removes the claim of the stage that ended (only when it is still this stage's), so the memory goes back to the others at once. */
export function clearReadyStageClaimV1(directory:string,pid:number,policy:typeof P=P):void{
 try{
  const file=join(directory,policy.fileName),value=JSON.parse(readFileSync(file,"utf8")) as{pid?:unknown};
  if(value.pid===pid)rmSync(file,{force:true});
 }catch{}
}

/** 📣️ The publisher of one stage run: `observe` writes the claim when the tree size moved by `changeMegabytes` or `writeMilliseconds` passed (the loop samples every few seconds and a file write per sample would be noise), `clear` removes it. The entitlement is read once from the stage's earlier runs. */
export function createReadyStageClaimPublisherV1(input:Readonly<{directory:string;stage:string;pid:number;totalMegabytes:number;runs:()=>readonly ReadyStageRunV1[];now?:()=>number}>,policy:typeof P=P):Readonly<{observe(observedMegabytes:number):void;clear():void}>{
 const now=input.now??Date.now;
 let entitlement:number|undefined,lastMegabytes=-Infinity,lastAt=-Infinity;
 return{
  observe(observedMegabytes){
   const at=now(),megabytes=Math.max(0,Math.round(observedMegabytes));
   if(Math.abs(megabytes-lastMegabytes)<policy.changeMegabytes&&at-lastAt<policy.writeMilliseconds)return;
   try{
    entitlement??=readyStageEntitlementV1({totalMegabytes:input.totalMegabytes,stage:input.stage,runs:input.runs()},policy);
    writeReadyStageClaimV1(input.directory,{version:1,stage:input.stage,pid:input.pid,entitlementMegabytes:entitlement,observedMegabytes:megabytes,atMs:at},policy);
    lastMegabytes=megabytes;lastAt=at;
   }catch{}
  },
  clear(){clearReadyStageClaimV1(input.directory,input.pid,policy);}
 };
}
