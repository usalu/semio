import {constants} from "node:fs";
import {open} from "node:fs/promises";
import {join,resolve} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import type {GenerationHeadCandidateV1} from "../../../../🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts";
import {publishWholeGenerationAdoptionReceiptV1,mirrorWholeGenerationAdoptionReceiptV1,stageWholeGenerationAdoptionIntentV1,recoverWholeGenerationAdoptionReceiptV1} from "../../../../🕸️graph/🕰️generations/🪪️adoption/🟦️.ts";
import type {RestoreExpectedWholeGenerationOutcomeV1} from "../../../../🕸️graph/🕰️generations/🔌️load/🟨️.mjs";
import type {RemoteExecutionSidecarContextV1} from "../../🧰️shared/🟦️.ts";
import {readyPlatformV1,READY_POLICY_V1,READY_STATUS_SCHEMA_V1,type ReadyStatusV1} from "../../🚦️ready/🟦️.ts";
import readySchema from "../../🚦️ready/🧬️schema/🔣️.json" with {type:"json"};
import {READY_TREE_POLICY_V1} from "../../🚦️ready/🌲️process-tree/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readLocalFleetCacheFactsV1,resolveRemoteCacheV1} from "../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {createFleetRegistryV1,loadFleetSignerV1} from "../../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {readRemoteExecutionHostViewV1} from "../../../🌡️load/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../../🌡️host-capacity/🎛️control/🟦️.ts";
import {hostFloorMegabytesV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {machinePathV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {graphRefreshCustodyHeldV1} from "../🔬️stall/🟦️.ts";
import {liveGraphReadV1} from "../🔭️read/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

export const headAdoptionPolicyV1=Object.freeze(schema.definitions.Policy.const);
type Scope=Readonly<{candidate():Promise<GenerationHeadCandidateV1|undefined>;restore(head:GenerationHeadCandidateV1):Promise<RestoreExpectedWholeGenerationOutcomeV1>}>;
type Ports=Readonly<{
 eligible(signal:AbortSignal,active:boolean):Promise<boolean>;
 withScope(signal:AbortSignal,run:(scope:Scope)=>Promise<void>):Promise<void>;
 result?(outcome:RestoreExpectedWholeGenerationOutcomeV1,candidate:GenerationHeadCandidateV1,signal:AbortSignal):void|Promise<void>;
 fault?(error:unknown):void;
 now?:()=>number;
 wait?:(milliseconds:number,signal:AbortSignal)=>Promise<unknown>;
}>;

const policy=headAdoptionPolicyV1;
const wait=(milliseconds:number,signal:AbortSignal)=>sleep(milliseconds,undefined,{signal,ref:false}).catch(()=>undefined);
const keyOf=(head:GenerationHeadCandidateV1)=>JSON.stringify([head.identity,head.platform,head.implementation,head.origin,head.scope,head.generation,head.manifest]);

/** 🎯️ Dispatches one complete whole head while readiness and foreground admission continue to hold. */
export function createHeadAdoptionTurnV1(ports:Ports):(signal:AbortSignal)=>Promise<void>{
 const now=ports.now??Date.now,pause=ports.wait??wait;
 let running=false,last:Readonly<{key:string;at:number}>|undefined;
 return async signal=>{
  if(running||signal.aborted)return;
  running=true;
  const stop=new AbortController(),active=AbortSignal.any([signal,stop.signal,AbortSignal.timeout(policy.turnMilliseconds)]);
  let monitor:Promise<void>|undefined;
  try{
   if(!await ports.eligible(active,false)||active.aborted)return;
   monitor=(async()=>{while(!active.aborted){await pause(policy.watchMilliseconds,active);if(!active.aborted&&!await ports.eligible(active,true).catch(()=>false))stop.abort();}})();
   await ports.withScope(active,async scope=>{
    if(active.aborted||!await ports.eligible(active,true))return;
    const candidate=await scope.candidate();
    if(!candidate||candidate.scope!=="all"||active.aborted)return;
    const key=keyOf(candidate);
    if(last?.key===key&&now()-last.at<policy.cooldownMilliseconds)return;
    if(!await ports.eligible(active,true)||active.aborted)return;
    last={key,at:now()};
    const outcome=await scope.restore(candidate);
    if(outcome.status==="restored"||!active.aborted)await ports.result?.(outcome,candidate,active);
   });
  }catch(error){if(!active.aborted)ports.fault?.(error);}
  finally{stop.abort();try{await monitor;}finally{running=false;}}
 };
}

type ReadyRead=(path:string,maximumBytes:number,signal:AbortSignal)=>Promise<Uint8Array|undefined>;
type ReadyObservation=Readonly<{signal:AbortSignal;bytes:Promise<Uint8Array|undefined>}>;

async function readReadyBytes(path:string,maximumBytes:number,signal:AbortSignal):Promise<Uint8Array|undefined>{
 signal.throwIfAborted();
 const file=await open(path,constants.O_RDONLY|(constants.O_NONBLOCK??0));
 try{
  signal.throwIfAborted();
  const status=await file.stat();
  signal.throwIfAborted();
  if(!status.isFile()||status.size>maximumBytes)return;
  const bytes=Buffer.alloc(status.size+1);
  let size=0;
  while(size<bytes.length){
   const {bytesRead}=await file.read(bytes,size,bytes.length-size,size);
   signal.throwIfAborted();
   if(!bytesRead)break;
   size+=bytesRead;
  }
  if(size===status.size)return bytes.subarray(0,size);
 }finally{await file.close();}
}

/** 🚦️ Admits a machine whose own daemon record is valid, of this origin and platform and fresh, whatever stage it runs: a machine that is not Ready yet needs the graph to become Ready, so Ready can never be the price of adopting it. Retains one read slot even if the filesystem outlasts its deadline. */
export function createHeadAdoptionLiveProbeV1(input:Readonly<{read?:ReadyRead;maximumWaitMilliseconds?:number}>={}){
 const read=input.read??readReadyBytes,maximumWait=input.maximumWaitMilliseconds??policy.readyProbeMilliseconds;
 const pending=new Map<string,ReadyObservation>();
 return async (store:string,origin:string,signal:AbortSignal,now:()=>number=Date.now):Promise<boolean>=>{
  if(signal.aborted)return false;
  const path=resolve(store,"v1","origins",origin,READY_POLICY_V1.statusName);
  let observation=pending.get(path);
  if(!observation){
   const bounded=AbortSignal.any([signal,AbortSignal.timeout(maximumWait)]);
   const bytes=Promise.resolve().then(()=>read(path,policy.maximumReadyBytes,bounded)).catch(()=>undefined).finally(()=>{pending.delete(path);});
   observation={signal:bounded,bytes};
   pending.set(path,observation);
  }
  const observed=AbortSignal.any([signal,observation.signal]);
  if(observed.aborted)return false;
  let cancel!:()=>void;
  try{
   const aborted=new Promise<undefined>(done=>{cancel=()=>done(undefined);observed.addEventListener("abort",cancel,{once:true});});
   const bytes=await Promise.race([observation.bytes,aborted]);
   if(observed.aborted||!bytes||bytes.length>policy.maximumReadyBytes)return false;
   const status=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes)) as ReadyStatusV1,elapsed=now()-status.updatedMs;
   return validateJsonSchemaSubset(READY_STATUS_SCHEMA_V1,status,readySchema).length===0&&status.origin===origin&&status.platform===readyPlatformV1()&&elapsed>=0&&elapsed<=READY_TREE_POLICY_V1.foregroundFreshMilliseconds;
  }catch{return false;}
  finally{observed.removeEventListener("abort",cancel);}
 };
}

/** 🕰️ Shares one process-local liveness probe across adoption, foreground monitoring and receipt mirroring. */
export const liveForHeadAdoptionV1=createHeadAdoptionLiveProbeV1();

async function withHeadCapacity<T>(megabytes:number,environment:NodeJS.ProcessEnv,signal:AbortSignal,report:(megabytes:number)=>void,body:()=>Promise<T>):Promise<T>{
 const {reserveGraphPlanCapacityV1}=await import("../../../../🌡️host-capacity/🟦️.ts"),pending=new AbortController(),cancel=()=>pending.abort(signal.reason);
 signal.addEventListener("abort",cancel,{once:true});
 let grant:Awaited<ReturnType<typeof reserveGraphPlanCapacityV1>>;
 try{signal.throwIfAborted();grant=await reserveGraphPlanCapacityV1({megabytes,minimumMegabytes:megabytes,environment,signal:pending.signal});}
 finally{signal.removeEventListener("abort",cancel);}
 try{signal.throwIfAborted();if(grant.megabytes!==megabytes)throw Error("Exact graph adoption capacity grant required");grant.attach(process.pid);report(grant.megabytes);return await body();}
 finally{report(0);grant.release();}
}

async function openAdoptionScope(root:string,origin:string,storeRoot:string,localDirectory:string,environment:NodeJS.ProcessEnv,signal:AbortSignal,report:(megabytes:number)=>void,run:(scope:Scope)=>Promise<void>):Promise<void>{
 const routing=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{acquireResourceLease}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts"),{scriptCompilerAdmissionControlV1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts");
 const owner="graph:head-adoption",resource=owner+":"+root,receiptLocation={root,store:storeRoot,origin,localDirectory,signal};
 const envelope=routing.createScriptProcessEnvelope(routing.readScriptPolicy({version:1,owner,maximumElapsedMilliseconds:policy.turnMilliseconds}),{workspaceRoot:root},Date.now());
 await routing.withScriptProcessEnvelope(envelope,original=>routing.withScriptInvocationSignalV1(original,signal,async invocation=>{
  const control=scriptCompilerAdmissionControlV1(invocation,owner,0),adoptionSignal=AbortSignal.any([signal,control.signal]),directory=machinePathV1(root,".cache/b/l/m/data");
  const lease=await acquireResourceLease({directory,resource,mode:"exclusive",control,signal,timeoutMs:0}).catch(error=>{if(error instanceof Error&&error.message==="Resource lease timed out: "+resource)return undefined;throw error;});
  if(!lease)return;
  try{
   const {withImplementationPinV1}=await import("../../../../📥️inputs/🔏️revision/🟨️.mjs");
   await withImplementationPinV1(async()=>{
    const prepared=await withHeadCapacity(policy.probeMegabytes,environment,signal,report,async()=>{
     const [{readBuildOwnerRegistryV1},{readBuildOwnerRegistrySourceV1},{generationHostV1},{createGenerationStoreV1},{implementationDigestV1,restoreExpectedWholeGenerationV1},{fleetOriginsV1,remoteGenerationExchangeV1}]=await Promise.all([
      import("../../../../🕸️graph/🚪️admission/🟦️.ts"),import("../../../../🕸️graph/📥️inference/🟨️.mjs"),import("../../../../🕸️graph/🕰️host/🟦️.ts"),import("../../../../🕸️graph/🕰️generations/🟨️.mjs"),import("../../../../🕸️graph/🕰️generations/🔌️load/🟨️.mjs"),import("../../../../🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts"),
     ]);
     const registry=readBuildOwnerRegistryV1(JSON.parse(readBuildOwnerRegistrySourceV1(root))),host=generationHostV1(root,invocation,control,registry);
     try{
      const enrolled=fleetOriginsV1(root),listed=enrolled.all.filter(peer=>peer!==origin),group=readLocalFleetCacheFactsV1(root)?.facts.group;
      if(enrolled.self!==origin||!group||listed.length===0||listed.length>policy.maximumProbeOrigins)return undefined;
      const authority=createFleetRegistryV1({storeRoot,group,pinsPath:join(localDirectory,"head-adoption-pins.json")});
      await authority.refresh();
      if(!authority.key(origin,"cache-read"))return undefined;
      const peers=listed.filter(peer=>authority.key(peer,"cache-write")!==undefined);
      if(!peers.length)return undefined;
      const identity=await host.identity(),implementation=implementationDigestV1(await host.implementationRows()),exchange=remoteGenerationExchangeV1({environment,workspace:root,identity:async()=>identity,origins:{self:origin,all:peers},serving:false});
      if(!exchange)return undefined;
      host.scope(true);
      const candidate=await host.withShared(async(store,phase)=>{
       const options=host.storeOptions(store,phase),local=await createGenerationStoreV1(options).read(signal);
       if(local?.manifest.identity===identity&&local.manifest.implementation===implementation){
        if(local.pointer.manifest!==null)await recoverWholeGenerationAdoptionReceiptV1({...receiptLocation,local:{generation:local.pointer.generation,localManifest:local.pointer.manifest}});
        return undefined;
       }
       return (await exchange.look({root:options.root,fs:options.fs,signal,implementation,requireComplete:true})).candidate;
      });
      return candidate&&candidate.origin!==origin?{candidate,host:{...host,exchange},restoreExpectedWholeGenerationV1}:undefined;
     }finally{await host.release();}
    });
    if(!prepared)return;
    await run({
     candidate:async()=>prepared.candidate,
     restore:head=>withHeadCapacity(policy.grantMegabytes,environment,signal,report,async()=>{
      try{
       if(!await liveForHeadAdoptionV1(storeRoot,origin,adoptionSignal))return{status:"busy"} as const;
       return await prepared.restoreExpectedWholeGenerationV1(prepared.host,head,{beforePublication:async local=>{
        if(!await liveForHeadAdoptionV1(storeRoot,origin,adoptionSignal))throw Error("Foreground work resumed during graph adoption");
        await stageWholeGenerationAdoptionIntentV1({...receiptLocation,candidate:head,local});
       }});
      }finally{await prepared.host.release();}
     }),
    });
   });
  }finally{lease.release();}
 }));
}

/** 🌱️ Runs the head-only local lane beside Wanted without changing its serving or refresh policy. */
export async function runHeadAdoptionLaneV1(context:RemoteExecutionSidecarContextV1,input:Readonly<{environment:NodeJS.ProcessEnv;store:string|undefined;origin:string;serveTotalMegabytes:number;steerDirectory:string}>):Promise<void>{
 const {root,signal}=context,{environment,store,origin,serveTotalMegabytes,steerDirectory}=input;
 if(!store||!BUILD_HOST_CAPACITY_POLICY_V1.enabled||environment.SEMIO_HOST_CAPACITY==="off"||environment.SEMIO_HOST_CAPACITY_GRANT!==undefined)return;
 const {repositoryCacheBudgetV1}=await import("../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts"),{nativeDirectoryHostAvailableBytesV1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts");
 const receiptLocation={root,store,origin,localDirectory:steerDirectory};
 const graph=repositoryCacheBudgetV1(root).partitions.graph,generationsRoot=join(graph.directory,"m","generations"),leaseDirectory=machinePathV1(root,".cache/b/l/m/data");
 let heldMegabytes=0;
 const tick=createHeadAdoptionTurnV1({
  async eligible(turn,active){
   if(turn.aborted||context.work?.restartPending||context.work?.busy||!await liveForHeadAdoptionV1(store,origin,turn))return false;
   const view=await readRemoteExecutionHostViewV1(environment),host=view.host,floor=hostFloorMegabytesV1(BUILD_HOST_CAPACITY_POLICY_V1,host);
   if(host.totalMegabytes<policy.minimumHostMegabytes||host.totalMegabytes>=serveTotalMegabytes||host.pressure!=="normal"||host.availableMegabytes<floor+(active?0:policy.grantMegabytes)||view.waiters.some(row=>!active||row.pid!==process.pid)||view.grants.some(row=>!active||row.pid!==process.pid))return false;
   if(active)return heldMegabytes===0||process.memoryUsage.rss()/1048576<heldMegabytes;
   if(liveGraphReadV1(steerDirectory)?.status===null||graphRefreshCustodyHeldV1({generationsRoot,leaseDirectory,environment,signal:turn}))return false;
   const copy=readLocalFleetCacheFactsV1(root),resolved=copy&&resolveRemoteCacheV1(environment,{root,copy});
   if(loadFleetSignerV1(root,"cache-read")?.originId!==origin||resolved?.tokenFrom!=="fleet-identity"||resolved.endpointFrom!=="fleet-facts"||!(copy?.facts.cache.transport==="sealed"||new URL(resolved.contract.endpoint).protocol==="https:"))return false;
   return nativeDirectoryHostAvailableBytesV1(graph.directory)>=graph.filesystemBytes;
  },
  withScope:(turn,run)=>openAdoptionScope(root,origin,store,steerDirectory,environment,turn,megabytes=>{heldMegabytes=megabytes;},run),
  async result(outcome,candidate,turn){
   if(outcome.status!=="restored")return;
   const receipt=await publishWholeGenerationAdoptionReceiptV1({...receiptLocation,candidate,outcome,signal:turn});
   if(receipt)context.write(`graph-head adopted origin=${receipt.head.origin} producerGeneration=${receipt.head.generation} scope=${receipt.head.scope} manifest=${receipt.head.manifest} consumer=${receipt.destination} localGeneration=${receipt.localGeneration} localManifest=${receipt.localManifest}`);
  },
  fault:error=>context.write("graph-head adoption / Graphstand-Übernahme "+String(error instanceof Error?error.message:error).slice(0,200)),
 });
 while(!signal.aborted){
  await tick(signal);
  if(!context.work?.restartPending&&await liveForHeadAdoptionV1(store,origin,signal))await mirrorWholeGenerationAdoptionReceiptV1({...receiptLocation,signal});
  await wait(policy.roundMilliseconds,signal);
 }
}
