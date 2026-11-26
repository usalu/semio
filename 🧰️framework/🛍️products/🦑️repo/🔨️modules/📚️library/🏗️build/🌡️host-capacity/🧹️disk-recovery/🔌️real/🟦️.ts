import {spawn} from "node:child_process";
import {processRecordAliveV1} from "../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import {statSync} from "node:fs";
import {readdir} from "node:fs/promises";
import {join} from "node:path";
import {machinePathV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {IMAGE_RECOVERY_VOLUME_ENV_V1,type ImageRecoveryVolumeV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧭️volume/🟦️.ts";
import resetSchema from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧬️schema/🔣️.json" with {type:"json"};
import {fileURLToPath} from "node:url";
import {planNativeShardReliefV1,readNativeShardsV1,relieveNativeShardsV1} from "../🧱️native-shards/🟦️.ts";
import {nativeShardHoldV1} from "../🧱️native-shards/🔒️hold/🟦️.ts";
import {freeSpaceSyncV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/🟦️.ts";
import {handleTextV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {formatDiskBytesV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/💽️disk/🟦️.ts";
import {DISK_RECOVERY_POLICY_V1,DISK_RECOVERY_TEXTS_V1,DISK_WRITERS_V1,diskReliefV1,requireDiskWritersV1,type DiskLanguageV1,type DiskWriterRowV1,type DiskWriterV1} from "../🟦️.ts";
import {sweepWorkspaceSandboxImagesV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/🟦️.ts";
import {ownerPoolStagingIoV1,ownerPoolStagingSweepV1} from "../../../🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧹️staging-sweep/🟦️.ts";
import {casHostAreasV1} from "../../../🗄️cas/📂️areas/🟦️.ts";
import {collectImageSlackV1,describeImageSlackV1,measureImagesV1,type ImageOutcomeV1,type ImageReclaimOutcomeV1} from "../🖼️image-slack/🟦️.ts";
import {collectReplicaStoreV1,readReplicaStoreRoleV1,type ReplicaCompactionV1} from "../🗃️replica-store/🟦️.ts";
import {collectCasV1} from "../../../🗄️cas/🧹️collector/🟦️.ts";
import {casPartitionPolicyForV1,measureCasPartitionV1} from "../../../🗄️cas/🌡️partition/🟦️.ts";
import {CAS_POLICY_V1} from "../../../🗄️cas/🧬️schema/🟦️.ts";
import {casUsageV1} from "../../../🗄️cas/🗃️store/📏️usage/🟦️.ts";
import {applyBoundV1,boundedBytesV1,DISK_BOUNDS_V1,finishedReadVerdictV1,selectExpiredV1,selectRotatedV1,selectTemporaryV1} from "../🧺️bounds/🟦️.ts";

/** 🔌️ Where the real writers live and what they may touch: the workspace, the home of the user, the temporary directory, the user id, the clock, the liveness of a process and the one spawn of the product's own collector; tests replace every one of them. */
export type RealDiskContextV1=Readonly<{workspace:string;home:string;tmp:string;uid:number;environment:Readonly<Record<string,string|undefined>>;now?:()=>number;alive?:(pid:number)=>boolean;dryRun?:boolean;ledger?:(signal?:AbortSignal,reclaimBytes?:number,areas?:string)=>Promise<void>;imageBacked?:boolean;language?:DiskLanguageV1;report?:(line:string)=>void;pressure?:ImagePressureV1;reclaimPressure?:ImagePressureV1&Readonly<{freeBytes:()=>number|undefined}>;retire?:(partition:string,signal?:AbortSignal,recovery?:ImageRecoveryVolumeV1)=>Promise<ImageOutcomeV1>;reclaim?:(partition:string,signal?:AbortSignal,recovery?:ImageRecoveryVolumeV1)=>Promise<ImageReclaimOutcomeV1>;compact?:(storeRoot:string,originId:string,signal?:AbortSignal)=>Promise<ReplicaCompactionV1>;free?:()=>number|undefined}>;

/** 💽️ The path whose measured host pressure selects the image volume and target bytes. */
type ImagePressureV1=Readonly<{path:string;requiredFreeBytes:number}>;

const expand=(template:string,context:RealDiskContextV1):string=>template.replaceAll("{store}",readReplicaStoreRoleV1(context.workspace).storeRoot??"").replaceAll("{workspace}",context.workspace).replaceAll("{home}",context.home).replaceAll("{tmp}",context.tmp.replace(/\/+$/u,"")).replaceAll("{uid}",String(context.uid));
const aliveByDefault=(pid:number):boolean=>processRecordAliveV1(pid);

/** 🚀️ Runs `repo cache gc` of this workspace as the product's own collector and waits for it, in a clean environment that marks it as a child of the recovery so that it never starts a recovery itself. */
export async function runLedgerCollectorV1(workspace:string,environment:Readonly<Record<string,string|undefined>>,signal?:AbortSignal,reclaimBytes?:number,areas?:string):Promise<void>{
 const{graphCollectorEnvironmentV1}=await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts"),script=fileURLToPath(new URL("../../../📜️script.ts",import.meta.url));
 await new Promise<void>((accept,reject)=>{
  const child=spawn(process.execPath,[script,"repo","cache","gc",...areas===undefined?[]:["--area="+areas],...reclaimBytes!==undefined&&reclaimBytes>0?["--reclaim="+Math.ceil(reclaimBytes)]:[]],{cwd:workspace,env:{...graphCollectorEnvironmentV1(environment),[DISK_RECOVERY_POLICY_V1.environmentChild]:"1"},stdio:"ignore",windowsHide:true,signal});
  child.on("error",reject);child.on("exit",code=>code===0?accept():reject(Error("repo cache gc exited with "+code)));
 });
}

/** 🖼️ Retires one idle elastic image through the product's own exclusive non-waiting regrow (`repo run repo-cache-reset:cache-regrow-<partition>`), in a clean environment that marks the child of the recovery; the child prints one JSON line with the outcome (`retired`, `held` while a command holds the image mapped, `journal` when the journal does not admit whole retirement) and is killed with the signal, which leaves the image whole or retired, never half. */
export async function runImageRegrowV1(workspace:string,partition:string,environment:Readonly<Record<string,string|undefined>>,signal?:AbortSignal,recovery?:ImageRecoveryVolumeV1):Promise<ImageOutcomeV1>{
 const{graphCollectorEnvironmentV1}=await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts"),script=fileURLToPath(new URL("../../../📜️script.ts",import.meta.url));
 return new Promise<ImageOutcomeV1>(accept=>{
  let out="";const child=spawn(process.execPath,[script,"repo","run","repo-cache-reset:cache-regrow-"+partition],{cwd:workspace,env:{...graphCollectorEnvironmentV1(environment),[DISK_RECOVERY_POLICY_V1.environmentChild]:"1",...(recovery?{[IMAGE_RECOVERY_VOLUME_ENV_V1]:JSON.stringify(recovery)}:{})},stdio:["ignore","pipe","ignore"],windowsHide:true,signal});
  child.stdout!.on("data",chunk=>{out=(out+String(chunk)).slice(-4096);});
  child.on("error",error=>accept({kind:"failed",message:error.message}));
  child.on("close",code=>{
   if(code!==0){accept({kind:"failed",message:"regrow exited with "+code});return;}
   const line=out.split("\n").reverse().find(item=>item.includes('"repository-cache-regrow"'));
   try{
    const receipt=JSON.parse(line??"null") as {version?:number;kind?:string;partition?:string;outcome?:string}|null,kind=receipt?.outcome;
    if(receipt?.version===1&&receipt.kind==="repository-cache-regrow"&&receipt.partition===partition&&(kind==="retired"||kind==="held"||kind==="journal"||kind==="skipped")){accept({kind});return;}
   }catch{}
   accept({kind:"failed",message:"regrow answered no exact outcome"});
  });
 });
}

/** 🔎️ Interprets a terminal receipt for the requested image; only a settled, measured cycle can establish that it returned no backing blocks. */
export function readImageReclaimReceiptV1(value:unknown,partition:string):ImageReclaimOutcomeV1{
 if(validateJsonSchemaSubset({$defs:resetSchema.$defs,$ref:"#/$defs/ImageReclaimReceipt"},value).length)return{kind:"unconfirmed"};
 type SkipReason=(typeof resetSchema.$defs.ImageReclaimSkipReason.enum)[number];
 const receipt=value as Readonly<{partition:string;outcome:"skipped"|"reclaimed";reason?:SkipReason;settled:boolean;beforeBytes:number;afterBytes:number}>,{beforeBytes,afterBytes,reason}=receipt;
 if(receipt.partition!==partition)return{kind:"unconfirmed"};
 if(receipt.outcome==="skipped")return reason!==undefined&&(resetSchema.$defs.ImageReclaimSkipReason.enum as readonly string[]).includes(reason)&&!receipt.settled&&beforeBytes===0&&afterBytes===0?{kind:"skipped",reason}:{kind:"unconfirmed"};
 if(reason!==undefined)return{kind:"unconfirmed"};
 if(!receipt.settled||beforeBytes===0||afterBytes>beforeBytes)return{kind:"unconfirmed"};
 return{kind:afterBytes<beforeBytes?"reclaimed":"unchanged"};
}

/** ♻️ Returns the blocks of one idle bloated image in place through the product's own exclusive non-waiting reclaim (`repo run repo-cache-reset:cache-reclaim-<partition>`: detach, attach once through the UNMAP-capable stack, settle, detach), in a clean environment that marks the child of the recovery; the child prints one JSON line (`reclaimed`, or `skipped` with the observed reason) and is killed with the signal, which leaves the image detached and whole. */
export async function runImageReclaimV1(workspace:string,partition:string,environment:Readonly<Record<string,string|undefined>>,signal?:AbortSignal,recovery?:ImageRecoveryVolumeV1):Promise<ImageReclaimOutcomeV1>{
 const{graphCollectorEnvironmentV1}=await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts"),script=fileURLToPath(new URL("../../../📜️script.ts",import.meta.url));
 return new Promise<ImageReclaimOutcomeV1>(accept=>{
  let out="";const child=spawn(process.execPath,[script,"repo","run","repo-cache-reset:cache-reclaim-"+partition],{cwd:workspace,env:{...graphCollectorEnvironmentV1(environment),[DISK_RECOVERY_POLICY_V1.environmentChild]:"1",...(recovery?{[IMAGE_RECOVERY_VOLUME_ENV_V1]:JSON.stringify(recovery)}:{})},stdio:["ignore","pipe","ignore"],windowsHide:true,signal});
  child.stdout!.on("data",chunk=>{out=(out+String(chunk)).slice(-4096);});
  child.on("error",error=>accept({kind:"failed",message:error.message}));
  child.on("close",code=>{
    if(code!==0){accept({kind:"failed",message:"reclaim exited with "+code});return;}
    const line=out.split("\n").reverse().find(item=>item.includes('"repository-cache-reclaim"'));
    try{accept(readImageReclaimReceiptV1(JSON.parse(line??"null"),partition));}catch{accept({kind:"unconfirmed"});}
   });
 });
}

/** 🗃️ Compacts the replica store in this process through the store's own compaction in apply mode, with the compaction settings of the topology (`replica.compaction`); the import is lazy so that only a group server loads the replica module. */
export async function compactReplicaStoreV1(storeRoot:string,originId:string,signal?:AbortSignal):Promise<ReplicaCompactionV1>{
 const replica=await import("../../../../../🤝️shared-worktree/🔁️replica/🟦️.ts"),topology=(await import("../../../../../🤝️shared-worktree/🗺️topology/🔣️.json",{with:{type:"json"}})).default as {replica:{compaction:Parameters<typeof replica.fleetCompactV1>[2]}};
 const result=await replica.fleetCompactV1(storeRoot,originId,topology.replica.compaction,"apply",signal,()=>undefined,Date.now,{blobs:true});
 return{skipped:result.skipped,bytes:result.blobBytes+result.segmentBytes};
}

/** 🗂️ Binds the registry to the real collectors of this machine: the in-process ones (rotation, age bounds, staging sweep, image sweep, the host store) and the one spawn of `repo cache gc` that serves every ledger row once; the manual rows stay without a collector and are only listed. */
export function realDiskWritersV1(context:RealDiskContextV1,rows:readonly DiskWriterRowV1[]=requireDiskWritersV1(DISK_WRITERS_V1)):readonly DiskWriterV1[]{
 const now=context.now??Date.now,alive=context.alive??aliveByDefault,dryRun=context.dryRun===true,limit=DISK_RECOVERY_POLICY_V1.measureEntryLimit,bounds=DISK_BOUNDS_V1;
 const ledgerRan=new Set<string>();
 const ledger=async(signal?:AbortSignal,reclaimBytes?:number,areas?:string):Promise<number>=>{const key=areas??"*";if(ledgerRan.has(key))return 0;ledgerRan.add(key);await(context.ledger??((abort,need,only)=>runLedgerCollectorV1(context.workspace,context.environment,abort,need,only)))(signal,reclaimBytes,areas);return 0;};
 const imageBacked=context.imageBacked??process.platform==="darwin";
 const tree=(root:string)=>async(signal?:AbortSignal)=>(await boundedBytesV1(root,limit,signal)).bytes;
 const temporaryRoot=context.tmp;
 const bind=(declared:DiskWriterRowV1):DiskWriterV1=>{
  const row:DiskWriterRowV1={...declared,relief:diskReliefV1(declared,imageBacked)},root=expand(row.root,context),base:DiskWriterV1={row,root,measure:tree(root)};
  switch(row.id){
   case "stale-waves":return{...base,collect:async({signal})=>(await applyBoundV1({directory:root,limit,dryRun,signal,select:entries=>selectExpiredV1(entries,now(),bounds.waves)})).freedBytes};
   case "task-views":{
    const place=async():Promise<string>=>{const{taskViewsDirectoryV1}=await import("../../../🏞️task-view/🟦️.ts");return taskViewsDirectoryV1(context.workspace);};
    return{...base,measure:async signal=>(await boundedBytesV1(await place(),limit,signal)).bytes,collect:async({signal})=>{const{collectTaskViewsV1}=await import("../../../🏞️task-view/🟦️.ts");return (await collectTaskViewsV1({directory:await place(),dryRun,signal,now,alive})).freedBytes;}};
   }
   case "replica-lost":return{...base,collect:async({signal})=>(await applyBoundV1({directory:root,limit,dryRun,signal,select:entries=>selectExpiredV1(entries,now(),bounds.replicaLost)})).freedBytes};
   case "footprint-logs":return{...base,collect:async({signal})=>(await applyBoundV1({directory:root,limit,dryRun,signal,select:entries=>selectRotatedV1(entries,now(),bounds.footprintLogs)})).freedBytes};
   case "read-verdicts":return{...base,collect:async({signal})=>{
    const candidate=finishedReadVerdictV1(now(),bounds.readVerdicts,alive);
    return (await applyBoundV1({directory:root,limit,dryRun,signal,candidate,select:entries=>entries.filter(candidate).map(entry=>entry.name)})).freedBytes;
   }};
   case "leaked-staging":return{...base,measure:async signal=>{let bytes=0;for(const name of await readdir(temporaryRoot).catch(()=>[] as string[]))if(name.startsWith(bounds.temporaryDirectories.prefix))bytes+=(await boundedBytesV1(join(temporaryRoot,name),limit,signal)).bytes;return bytes;},collect:async({signal})=>{
    if(!dryRun)ownerPoolStagingSweepV1({parents:[temporaryRoot],own:"",io:ownerPoolStagingIoV1});
    return (await applyBoundV1({directory:temporaryRoot,limit,dryRun,signal,select:entries=>selectTemporaryV1(entries,now(),bounds.temporaryDirectories,alive)})).freedBytes;
   }};
   case "nx-native-data":return{...base,measure:async()=>readNativeShardsV1(context.workspace).reduce((sum,fact)=>sum+planNativeShardReliefV1([fact]).reduce((bytes,item)=>bytes+item.bytes,0),0),collect:async({signal})=>(await relieveNativeShardsV1({facts:readNativeShardsV1(context.workspace),hold:nativeShardHoldV1(context.workspace),language:context.language??"en",dryRun,signal})).freedBytes};
   case "leaked-images":return{...base,measure:async()=>0,collect:async({signal})=>{if(process.platform==="darwin"&&!dryRun)await sweepWorkspaceSandboxImagesV1(context.workspace,signal);return 0;}};
   case "cas-lru":return{...base,measure:async signal=>(await casUsageV1(root,signal)).bytes,collect:async({signal})=>{
    const partition=measureCasPartitionV1(root);
    return (await collectCasV1({root,policy:casPartitionPolicyForV1(CAS_POLICY_V1,partition),role:"recovery",areas:await casHostAreasV1(context.home),partition,signal,dryRun})).freedBytes;
   }};
   case "image-slack":return{...base,measure:async()=>process.platform==="darwin"?describeImageSlackV1(measureImagesV1(context.workspace)).bytes:0,describe:async()=>{const detail=describeImageSlackV1(measureImagesV1(context.workspace));return{root:detail.root,command:detail.command};},collect:async({needBytes,signal})=>{
    if(process.platform!=="darwin"||dryRun)return 0;
    const pressure=context.pressure??context.reclaimPressure;
    let recovery:ImageRecoveryVolumeV1|undefined;
    if(pressure)try{recovery={version:1,device:statSync(pressure.path).dev,requiredFreeBytes:pressure.requiredFreeBytes};}catch{return 0;}
    const retirement=context.pressure===undefined?undefined:recovery;
    const free=context.free??context.reclaimPressure?.freeBytes??(()=>freeSpaceSyncV1(pressure?.path??machinePathV1(context.workspace,".cache"))?.availableBytes);
    if(context.reclaimPressure!==undefined&&free()===undefined)return 0;
    const retire=(partition:string,abort?:AbortSignal)=>context.retire?.(partition,abort,retirement)??runImageRegrowV1(context.workspace,partition,context.environment,abort,retirement);
    const reclaim=(partition:string,abort?:AbortSignal)=>context.reclaim?.(partition,abort,recovery)??runImageReclaimV1(context.workspace,partition,context.environment,abort,recovery);
    return (await collectImageSlackV1({rows:measureImagesV1(context.workspace),targetFreeBytes:recovery?.requiredFreeBytes??(free()??0)+needBytes,pressureDevice:recovery?.device,inspectDetached:retirement!==undefined,freeBytes:free,reclaim,retire,language:context.language??"en",report:context.report,signal})).freedBytes;
   }};
   case "replica-store":{const role=readReplicaStoreRoleV1(context.workspace);return{...base,root:role.storeRoot??"",measure:async signal=>{if(role.kind!=="server")return 0;const{storeReportV1,compactionDefaultsV1}=await import("../../../../../🤝️shared-worktree/🗜️compaction/🟦️.ts"),report=await storeReportV1(role.storeRoot!,compactionDefaultsV1,now(),signal,true);return report.reclaimable.bytes+(report.reclaimable.blobs?.bytes??0);},describe:async()=>({root:role.storeRoot??"",command:role.kind==="worker"?"on "+role.groupId+"-server: "+row.command:row.command}),collect:async({signal})=>{
    if(dryRun)return 0;
    const free=context.free??(()=>freeSpaceSyncV1(context.workspace)?.availableBytes),before=free()??0;
    await collectReplicaStoreV1({role,compact:context.compact??compactReplicaStoreV1,language:context.language??"en",report:context.report,signal});
    return Math.max(0,(free()??before)-before);
   }};}
   default:return row.collector==="ledger"&&(row.relief==="host"||row.relief==="image")?{...base,collect:({needBytes,signal})=>ledger(signal,needBytes,/--area=([a-z0-9,-]+)/u.exec(row.command)?.[1])}:base;
  }
 };
 return Object.freeze(rows.map(bind));
}

/** 🧺️ Applies the in-process bounds of the machine (waves, footprint logs, read verdicts, staging and temporary directories, the files the replica overwrote) as `repo cache gc` does after the ledger: each bound answers the bytes it freed, or would free in a dry run, and the line that says so in the language asked for. */
export async function collectMachineBoundsV1(context:RealDiskContextV1,language:"en"|"de",signal?:AbortSignal):Promise<Readonly<{lines:readonly string[];freed:Readonly<Record<string,number>>}>>{
 const lines:string[]=[],freed:Record<string,number>={};
 for(const writer of realDiskWritersV1(context).filter(item=>(item.row.relief==="host"||item.row.collector==="native-shards")&&["age-prune","log-rotate","staging-sweep","native-shards"].includes(item.row.collector)&&item.collect!==undefined).sort((first,second)=>first.row.priority-second.row.priority)){
  signal?.throwIfAborted();
  const bytes=await writer.collect!({needBytes:0,signal}).catch(()=>0);
  freed[writer.row.id]=bytes;lines.push(handleTextV1(DISK_RECOVERY_TEXTS_V1,context.dryRun===true?"bounds.dry":"bounds.freed",language,{id:writer.row.id,freed:formatDiskBytesV1(bytes)}));
 }
 return{lines,freed};
}
