import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {existsSync,lstatSync,readdirSync,realpathSync,statSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import type {PartitionFactsV1,StorageFactsV1} from "../../🟦️.ts";
import {readJsonFileV1,type DoctorPortsV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1,DoctorRepairRunV1} from "../📋️contract/🟦️.ts";
import {tryImageLeaseV1} from "./🔒️image-lease/🟦️.ts";
import {requireMaterializedPathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireNativeImageHostOwnersV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/👤️owners/🟦️.ts";
import {attachNativeImageV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🔗️attach/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {nativeStorageBackendsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🟦️.ts";
import {nativeImageInventoryV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📥️information/🟦️.ts";
import {nativeImageStrandedMountV1,recoverNativeImageStrandedV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🏚️stranded/🟦️.ts";

const BACKING="fixed.dmg",MOUNT="m",PUBLICATION=".publication",HDIUTIL="/usr/bin/hdiutil";
const entry=(path:string)=>{try{return lstatSync(path);}catch{return undefined;}};

/** 💾️ Reads one partition from its receipt, its backing file, its mount and its publication residue, without opening it. */
function partition(ports:DoctorPortsV1,id:string,directory:string,roster:Record<string,unknown>|undefined):PartitionFactsV1{
 const outer=entry(directory);
 if(!outer)return {id,state:"absent",capacityBytes:null,freeBytes:null,imageBytes:null,backing:"none",publication:"none",publicationAgeSeconds:null,image:"none"};
 const receiptPath=join(directory,"image.json"),receiptEntry=entry(receiptPath),receipt=receiptEntry?readJsonFileV1(receiptPath,65536) as {version?:unknown}|undefined:undefined;
 const image:PartitionFactsV1["image"]=receiptEntry===undefined?"none":receipt&&typeof receipt==="object"&&receipt.version===1?"ok":"damaged";
 const mount=join(directory,MOUNT),inner=entry(mount),directoryHost=ports.platform!=="darwin",backingPath=join(directory,BACKING),backingEntry=entry(backingPath);
 let state:PartitionFactsV1["state"],capacityBytes:number|null=null,freeBytes:number|null=null;
 if(directoryHost)state=inner?.isDirectory()?"directory":"absent";
 else if(inner?.isDirectory()&&inner.dev!==outer.dev){
  state="mounted";
  const space=nativeFreeSpaceSyncV1(mount);if(space){capacityBytes=space.totalBytes??null;freeBytes=space.availableBytes;}else state="detached";
 }else state=backingEntry?"detached":"absent";
 let strandedFacts:Pick<PartitionFactsV1,"attachedAt"|"device">|undefined;
 if(state==="detached"&&roster){
  try{
   const real=realpathSync(directory),stranded=nativeImageStrandedMountV1(roster,join(real,BACKING),join(real,MOUNT));
   if(stranded){state="misplaced";strandedFacts={attachedAt:stranded.mountPoints.join(", ")||null,device:stranded.device};}
  }catch{}
 }
 const residue=["intent","receipt"].find(name=>existsSync(join(directory,PUBLICATION,name+".json"))) as "intent"|"receipt"|undefined;
 let age:number|null=null;
 if(residue)try{age=Math.max(0,Math.floor((ports.now()-statSync(join(directory,PUBLICATION,residue+".json")).mtimeMs)/1000));}catch{age=0;}
 return {id,state,...strandedFacts,capacityBytes,freeBytes,imageBytes:backingEntry?backingEntry.size:null,backing:directoryHost?"none":backingEntry?"present":"missing",publication:residue??"none",publicationAgeSeconds:age,image};
}

/** 📡️ Reads the kernel inventory of attached images where the platform has images; unreadable reads as absent so the probe never fails on it. */
async function roster(ports:DoctorPortsV1):Promise<Record<string,unknown>|undefined>{
 if(ports.platform!=="darwin")return undefined;
 try{return nativeImageInventoryV1(await ports.command(HDIUTIL,["info","-plist"]));}catch{return undefined;}
}

/** 🔗️ Whether the `node_modules` of the checkout is a link whose target does not exist, so that no package import can resolve until the store behind it is attached. */
function loaderBlocked(root:string):boolean{
 const link=join(root,"node_modules");
 return entry(link)?.isSymbolicLink()===true&&!existsSync(link);
}

/** 📍️ The directory of one partition, below the machine state when the machine declares one. */
const directoryOf=(root:string,relative:string):string=>machinePathV1(root,...relative.split("/"));

/** 💽️ Reads every partition and the loader state of one machine. */
async function readStorage(ports:DoctorPortsV1):Promise<StorageFactsV1>{
 const images=await roster(ports),rows=Object.entries(budget.partitions).map(([id,row])=>partition(ports,id,directoryOf(ports.root,row.directory),images));
 let host=machinePathV1(ports.root,".cache");while(!existsSync(host)&&dirname(host)!==host)host=dirname(host);
 const hostSpace=nativeFreeSpaceSyncV1(host);
 return {hostFreeBytes:hostSpace?.availableBytes??null,hostTotalBytes:hostSpace?.totalBytes??null,loaderBlocked:loaderBlocked(ports.root),partitions:rows};
}

/** 🧭️ One partition the image repair touches and the steps it takes: a misplaced image is detached from its stray mount and attached at its mount point, a detached image is attached. The leases image comes first because the lease directory of every other image lives inside it. Pure and language-agnostic.
 * @see ./🧪️tests/🟦️.ts */
export type ImageRepairStepV1=Readonly<{id:string;steps:readonly ("detach"|"attach")[]}>;
export function imageRepairPlanV1(storage:Pick<StorageFactsV1,"loaderBlocked"|"partitions">|undefined):readonly ImageRepairStepV1[]{
 const targets=(storage?.partitions??[]).filter(row=>row.image!=="damaged"&&(row.state==="misplaced"||row.state==="detached"));
 return targets.map(row=>({id:row.id,steps:row.state==="misplaced"?["detach","attach"] as const:["attach"] as const})).sort((left,right)=>Number(right.id==="leases")-Number(left.id==="leases"));
}

/** 🔌️ Whether the kernel lists the image as attached at exactly its mount point. */
function attachedAtMount(inventory:Record<string,unknown>,image:string,mount:string):boolean{
 const rows=Array.isArray(inventory.images)?inventory.images as Record<string,unknown>[]:[];
 return rows.some(row=>row?.["image-path"]===image&&Array.isArray(row["system-entities"])&&(row["system-entities"] as Record<string,unknown>[]).some(entity=>entity?.["mount-point"]===mount));
}

/** 🩹️ Brings one image back to its mount point under its exclusive lease: the stray mount is detached without force (a busy device refuses), then the image attaches at `<directory>/m` with owners on through the storage library's own attach arguments. */
async function repairImage(ports:DoctorPortsV1,item:ImageRepairStepV1,directory:string):Promise<"done"|"busy"|"settled">{
 const lease=await tryImageLeaseV1(ports,item.id,directory);
 if(lease==="busy")return "busy";
 try{
  const real=realpathSync(directory),image=join(real,BACKING),mount=join(real,MOUNT),inventory=async()=>nativeImageInventoryV1(await ports.command(HDIUTIL,["info","-plist"]));
  const stray=await recoverNativeImageStrandedV1({inventory,detach:async device=>{await ports.command(HDIUTIL,["detach",device]);}},image,mount);
  if(stray===undefined&&attachedAtMount(await inventory(),image,mount))return "settled";
  const inner=lstatSync(mount),outer=lstatSync(real);
  if(!inner.isDirectory()||inner.dev!==outer.dev||readdirSync(mount).length>0)throw Error(`The mount point ${mount} is not an empty, unmounted directory / Der Einhängepunkt ${mount} ist kein leeres, nicht eingehängtes Verzeichnis`);
  requireNativeImageHostOwnersV1(await ports.command("/sbin/mount",[]),real);
  await attachNativeImageV1((program,args)=>ports.command(program,[...args]),mount,image);
  if(statSync(mount).dev===outer.dev)throw Error(`The image did not attach at ${mount} / Das Image wurde nicht unter ${mount} eingehängt`);
  return "done";
 }finally{lease?.release();}
}

/** 💿️ Re-attaches the images that a crashed or cancelled flow left detached or at a stray mount point, each under its exclusive lease; a dry run only counts them. Only macOS has attachable images. */
async function repairImages(ports:DoctorPortsV1,dryRun:boolean):Promise<DoctorRepairRunV1>{
 const imageBackend=nativeStorageBackendsV1.find(row=>row.platforms.includes(ports.platform)&&row.capabilities.includes("mounted-volume"));
 if(imageBackend===undefined){const backend=nativeStorageBackendsV1.find(row=>row.platforms.includes(ports.platform))?.id??"none";return {removed:0,bytes:0,status:"unavailable",reason:`Not applicable on ${ports.platform}: its partitions are backed by the ${backend} backend, plain directories with no image to attach, so no image can be detached or misplaced / Nicht anwendbar auf ${ports.platform}: Die Partitionen liegen im Backend ${backend}, gewöhnliche Verzeichnisse ohne einzuhängendes Image, daher kann kein Image ausgehängt oder fehlplatziert sein`};}
 const storage=await readStorage(ports),plan=imageRepairPlanV1(storage);
 if(plan.length===0)return {removed:0,bytes:0,status:"nothing"};
 if(dryRun)return {removed:plan.length,bytes:0};
 const relative=budget.partitions as Record<string,{directory:string}>;
 let repaired=0;const notes:string[]=[];let failure:unknown;
 for(const item of plan){
  try{
   const outcome=await repairImage(ports,item,requireMaterializedPathV1(directoryOf(resolve(ports.root),relative[item.id]!.directory)));
   if(outcome==="done")repaired++;else if(outcome==="busy")notes.push(`${item.id}: a reclaim or an opener holds the image / ${item.id}: ein Reclaim oder eine Öffnung hält das Image`);
  }catch(error){if(ports.signal.aborted)throw error;failure??=error;notes.push(`${item.id}: ${(error instanceof Error?error.message:String(error)).split("\n")[0]!.slice(0,200)}`);}
 }
 if(repaired===0&&failure!==undefined)throw failure;
 if(repaired===0&&notes.length>0)return {removed:0,bytes:0,status:"unavailable",reason:notes.join("; ")};
 return {removed:repaired,bytes:0,...notes.length>0?{reason:notes.join("; ")}:{}};
}

/** 💽️ The disk-level facts: image state, free space against budget, unsettled publications and the free space of the disk that holds the caches; the repair puts stranded and detached images back at their mount point. */
export const storageProbeV1:DoctorProbeV1=Object.freeze({
 area:"storage",phase:1,
 repairs:[Object.freeze({id:"storage-images",area:"storage" as const,run:(ports:DoctorPortsV1,_context:unknown,dryRun:boolean)=>repairImages(ports,dryRun)})],
 async read(ports){return {sections:{storage:await readStorage(ports)}};},
});
