import{resolve}from"node:path";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../🎛️control/🟦️.ts";
import{leaseStoreFreeRatioV1,retireIdleResourceLeasesV1,retireResourceQueueClaimsV1}from"../../🟦️.ts";
import{resourceLeaseBusyV1,resourceLeaseRetirementIdlePolicyV1,resourceLeaseRetirementMessagesV1}from"../🟦️.ts";
import{retireNestedLeaseStoresV1}from"../🧱️nested/🟦️.ts";
const policy=resourceLeaseRetirementIdlePolicyV1;

/** 🧹️ What one incremental sweep of a lease directory retired, kept and measured. */
export type ResourceLeaseSweepV1=Readonly<{version:1;directory:string;pressure:boolean;idleMilliseconds:number;databases:number;queues:number;claims:number;stores:number;kept:number;slices:number;freedBytes:number;freeRatioBefore:number|null;freeRatioAfter:number|null;complete:boolean;busy:boolean;cancelled:boolean}>;
/** ⏱️ Picks the idle clock and slice size of the store: the pressure values while less than the declared share of the filesystem is free; an unknown share counts as nothing free. */
export function resourceLeaseSweepModeV1(freeRatio:number|undefined):Readonly<{pressure:boolean;idleMilliseconds:number;candidates:number}>{const pressure=(freeRatio??0)<policy.pressureFreeRatio;return Object.freeze({pressure,idleMilliseconds:pressure?policy.pressureIdleMilliseconds:policy.idleMilliseconds,candidates:pressure?policy.pressureCandidatesPerSlice:policy.candidatesPerSlice});}

/** ♻️ Sweeps a lease directory in small slices under the ordinary lease protocol, so it runs while the fleet holds leases: settled claims first (their tickets then leave the queues empty), then the idle resource databases and their empty queues, then the abandoned nested stores. Every slice takes the directory gate for a few milliseconds only and persists its cursor, cancellation and the control's deadline are honoured between candidates, and a busy gate postpones the rest to the next caller. */
export async function sweepResourceLeaseStoreV1(directory:string,control:ResourceLeaseControl,options:Readonly<{nowMilliseconds?:number;budgetMilliseconds?:number;freeRatio?:number}>={}):Promise<ResourceLeaseSweepV1>{
 checkResourceLeaseControl(control);const root=resolve(directory),started=Date.now(),now=options.nowMilliseconds??started,budget=options.budgetMilliseconds??policy.budgetMilliseconds,before=options.freeRatio??leaseStoreFreeRatioV1(root),mode=resourceLeaseSweepModeV1(before),patience={admissionPatienceMs:policy.admissionPatienceMilliseconds};
 let databases=0,queues=0,claims=0,stores=0,kept=0,slices=0,freed=0,complete=false,busy=false,cancelled=false;
 const stage=(name:string)=>control.publish({resource:"lease-sweep:"+name,mode:"exclusive",elapsedMs:Math.max(0,Date.now()-started)});
 try{
  await stage("claims");
  for(let turn=0;turn<policy.claimTurnsPerSweep&&Date.now()-started<budget;turn++){const row=await retireResourceQueueClaimsV1(root,control,0,patience);claims+=row.retired;freed+=row.allocatedBytes;if(!row.retired)break;await control.yieldContinuation();}
  await stage("resources");
  while(slices<policy.slicesPerSweep&&Date.now()-started<budget){
   checkResourceLeaseControl(control);const row=await retireIdleResourceLeasesV1(root,control,0,{idleMilliseconds:mode.idleMilliseconds,nowMilliseconds:now,maximumCandidates:mode.candidates,...patience});
   slices++;databases+=row.retired;queues+=row.queues;kept+=row.kept;freed+=row.allocatedBytes;
   if(row.complete){complete=true;break;}
   await control.yieldContinuation();
  }
  await stage("nested");
  const nested=await retireNestedLeaseStoresV1(root,control,{nowMilliseconds:now});stores+=nested.retired;kept+=nested.kept;freed+=nested.allocatedBytes;
 }catch(error){
  if(control.signal.aborted)cancelled=true;
  else if(resourceLeaseBusyV1(error))busy=true;
  else throw error;
 }
 return Object.freeze({version:1,directory:root,pressure:mode.pressure,idleMilliseconds:mode.idleMilliseconds,databases,queues,claims,stores,kept,slices,freedBytes:freed,freeRatioBefore:before??null,freeRatioAfter:leaseStoreFreeRatioV1(root)??null,complete,busy,cancelled});
}

/** 🗣️ Renders the refusal of a sweep that failed for another reason than a busy gate or a cancellation. */
export function formatResourceLeaseSweepFailureV1(language:"en"|"de",error:unknown):string{return resourceLeaseRetirementMessagesV1.failed[language].replace("{reason}",error instanceof Error?error.message.slice(0,200):String(error).slice(0,200));}
/** 🔒️ Renders a sweep as lines in the chosen language; a quiet sweep that retired nothing renders nothing. */
export function formatResourceLeaseSweepV1(language:"en"|"de",sweep:ResourceLeaseSweepV1):readonly string[]{
 const text=(key:keyof typeof resourceLeaseRetirementMessagesV1,values:Readonly<Record<string,string|number>>)=>resourceLeaseRetirementMessagesV1[key][language].replace(/\{([A-Za-z]+)\}/gu,(match,name:string)=>name in values?String(values[name]):match),lines:string[]=[];
 if(sweep.pressure)lines.push(sweep.freeRatioBefore===null?text("pressureUnmeasured",{idle:Math.round(sweep.idleMilliseconds/1000)}):text("pressure",{used:Math.round((1-sweep.freeRatioBefore)*100),idle:Math.round(sweep.idleMilliseconds/1000)}));
 if(sweep.databases+sweep.queues+sweep.claims+sweep.stores>0)lines.push(text("swept",{databases:sweep.databases,queues:sweep.queues,claims:sweep.claims,stores:sweep.stores,freed:(sweep.freedBytes/1048576).toFixed(1)+" MiB",free:sweep.freeRatioAfter===null?"?":Math.round(sweep.freeRatioAfter*100)}));
 if(sweep.cancelled)lines.push(text("cancelled",{slices:sweep.slices}));
 else if(sweep.busy)lines.push(text("busy",{}));
 return lines;
}
