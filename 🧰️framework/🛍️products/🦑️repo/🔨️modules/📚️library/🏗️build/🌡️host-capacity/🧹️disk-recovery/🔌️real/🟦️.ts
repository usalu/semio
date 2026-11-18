import {spawn} from "node:child_process";
import {readdir,stat} from "node:fs/promises";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {freeSpaceSyncV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/🟦️.ts";
import {handleTextV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {formatDiskBytesV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/💽️disk/🟦️.ts";
import {DISK_RECOVERY_POLICY_V1,DISK_RECOVERY_TEXTS_V1,DISK_WRITERS_V1,requireDiskWritersV1,type DiskWriterRowV1,type DiskWriterV1} from "../🟦️.ts";
import {sweepLeakedSandboxImagesV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/🟦️.ts";
import {ownerPoolStagingIoV1,ownerPoolStagingSweepV1} from "../../../🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧹️staging-sweep/🟦️.ts";
import {casHostAreasV1} from "../../../🗄️cas/📂️areas/🟦️.ts";
import {collectCasV1} from "../../../🗄️cas/🧹️collector/🟦️.ts";
import {casPartitionPolicyForV1,measureCasPartitionV1} from "../../../🗄️cas/🌡️partition/🟦️.ts";
import {CAS_POLICY_V1} from "../../../🗄️cas/🧬️schema/🟦️.ts";
import {casUsageV1} from "../../../🗄️cas/🗃️store/📏️usage/🟦️.ts";
import {applyBoundV1,boundedBytesV1,DISK_BOUNDS_V1,selectExpiredV1,selectOlderThanV1,selectRotatedV1,selectTemporaryV1} from "../🧺️bounds/🟦️.ts";

/** 🔌️ Where the real writers live and what they may touch: the workspace, the home of the user, the temporary directory, the user id, the clock, the liveness of a process and the one spawn of the product's own collector; tests replace every one of them. */
export type RealDiskContextV1=Readonly<{workspace:string;home:string;tmp:string;uid:number;environment:Readonly<Record<string,string|undefined>>;now?:()=>number;alive?:(pid:number)=>boolean;dryRun?:boolean;ledger?:(signal?:AbortSignal)=>Promise<void>}>;

const expand=(template:string,context:RealDiskContextV1):string=>template.replaceAll("{workspace}",context.workspace).replaceAll("{home}",context.home).replaceAll("{tmp}",context.tmp.replace(/\/+$/u,"")).replaceAll("{uid}",String(context.uid));
const aliveByDefault=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🚀️ Runs `repo cache gc` of this workspace as the product's own collector and waits for it, in a clean environment that marks it as a child of the recovery so that it never starts a recovery itself. */
export async function runLedgerCollectorV1(workspace:string,environment:Readonly<Record<string,string|undefined>>,signal?:AbortSignal):Promise<void>{
 const{graphCollectorEnvironmentV1}=await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts"),script=fileURLToPath(new URL("../../../📜️script.ts",import.meta.url));
 await new Promise<void>((accept,reject)=>{
  const child=spawn(process.execPath,[script,"repo","cache","gc"],{cwd:workspace,env:{...graphCollectorEnvironmentV1(environment),[DISK_RECOVERY_POLICY_V1.environmentChild]:"1"},stdio:"ignore",windowsHide:true,signal});
  child.on("error",reject);child.on("exit",code=>code===0?accept():reject(Error("repo cache gc exited with "+code)));
 });
}

/** 🗂️ Binds the registry to the real collectors of this machine: the in-process ones (rotation, age bounds, staging sweep, image sweep, the host store) and the one spawn of `repo cache gc` that serves every ledger row once; the manual rows stay without a collector and are only listed. */
export function realDiskWritersV1(context:RealDiskContextV1,rows:readonly DiskWriterRowV1[]=requireDiskWritersV1(DISK_WRITERS_V1)):readonly DiskWriterV1[]{
 const now=context.now??Date.now,alive=context.alive??aliveByDefault,dryRun=context.dryRun===true,limit=DISK_RECOVERY_POLICY_V1.measureEntryLimit,bounds=DISK_BOUNDS_V1;
 let ledgerRan=false;
 const ledger=async(signal?:AbortSignal):Promise<number>=>{if(ledgerRan)return 0;ledgerRan=true;await(context.ledger??(abort=>runLedgerCollectorV1(context.workspace,context.environment,abort)))(signal);return 0;};
 const tree=(root:string)=>async(signal?:AbortSignal)=>(await boundedBytesV1(root,limit,signal)).bytes;
 const temporaryRoot=context.tmp;
 const bind=(row:DiskWriterRowV1):DiskWriterV1=>{
  const root=expand(row.root,context),base:DiskWriterV1={row,root,measure:tree(root)};
  switch(row.id){
   case "stale-waves":return{...base,collect:async({signal})=>(await applyBoundV1({directory:root,limit,dryRun,signal,select:entries=>selectExpiredV1(entries,now(),bounds.waves)})).freedBytes};
   case "replica-lost":return{...base,collect:async({signal})=>(await applyBoundV1({directory:root,limit,dryRun,signal,select:entries=>selectExpiredV1(entries,now(),bounds.replicaLost)})).freedBytes};
   case "footprint-logs":return{...base,collect:async({signal})=>(await applyBoundV1({directory:root,limit,dryRun,signal,select:entries=>selectRotatedV1(entries,now(),bounds.footprintLogs)})).freedBytes};
   case "read-verdicts":return{...base,collect:async({signal})=>(await applyBoundV1({directory:root,limit,dryRun,signal,select:entries=>selectOlderThanV1(entries,now(),bounds.readVerdicts.maximumAgeMilliseconds)})).freedBytes};
   case "leaked-staging":return{...base,measure:async signal=>{let bytes=0;for(const name of await readdir(temporaryRoot).catch(()=>[] as string[]))if(name.startsWith(bounds.temporaryDirectories.prefix))bytes+=(await boundedBytesV1(join(temporaryRoot,name),limit,signal)).bytes;return bytes;},collect:async({signal})=>{
    if(!dryRun)ownerPoolStagingSweepV1({parents:[temporaryRoot],own:"",io:ownerPoolStagingIoV1});
    return (await applyBoundV1({directory:temporaryRoot,limit,dryRun,signal,select:entries=>selectTemporaryV1(entries,now(),bounds.temporaryDirectories,alive)})).freedBytes;
   }};
   case "leaked-images":return{...base,measure:async()=>0,collect:async()=>{if(process.platform!=="darwin"||dryRun)return 0;sweepLeakedSandboxImagesV1({under:context.workspace});return 0;}};
   case "cas-lru":return{...base,measure:async()=>(await casUsageV1(root)).bytes,collect:async({signal})=>{
    const partition=measureCasPartitionV1(root);
    return (await collectCasV1({root,policy:casPartitionPolicyForV1(CAS_POLICY_V1,partition),areas:await casHostAreasV1(context.home),partition,signal,dryRun})).freedBytes;
   }};
   case "image-slack":return{...base,measure:async()=>{let slack=0;for(const name of await readdir(root).catch(()=>[] as string[])){const image=await stat(join(root,name,"fixed.dmg")).catch(()=>undefined);if(image===undefined)continue;const inside=freeSpaceSyncV1(join(root,name,"m")),used=inside?.totalBytes!==undefined?inside.totalBytes-inside.availableBytes:0;slack+=Math.max(0,image.blocks*512-used);}return slack;}};
   default:return row.collector==="ledger"&&row.relief==="host"?{...base,collect:({signal})=>ledger(signal)}:base;
  }
 };
 return Object.freeze(rows.map(bind));
}

/** 🧺️ Applies the in-process bounds of the machine (waves, footprint logs, read verdicts, staging and temporary directories, the files the replica overwrote) as `repo cache gc` does after the ledger: each bound answers the bytes it freed, or would free in a dry run, and the line that says so in the language asked for. */
export async function collectMachineBoundsV1(context:RealDiskContextV1,language:"en"|"de",signal?:AbortSignal):Promise<Readonly<{lines:readonly string[];freed:Readonly<Record<string,number>>}>>{
 const lines:string[]=[],freed:Record<string,number>={};
 for(const writer of realDiskWritersV1(context).filter(item=>item.row.relief==="host"&&["age-prune","log-rotate","staging-sweep"].includes(item.row.collector)&&item.collect!==undefined).sort((first,second)=>first.row.priority-second.row.priority)){
  signal?.throwIfAborted();
  const bytes=await writer.collect!({needBytes:0,signal}).catch(()=>0);
  freed[writer.row.id]=bytes;lines.push(handleTextV1(DISK_RECOVERY_TEXTS_V1,context.dryRun===true?"bounds.dry":"bounds.freed",language,{id:writer.row.id,freed:formatDiskBytesV1(bytes)}));
 }
 return{lines,freed};
}
