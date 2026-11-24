import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {existsSync,lstatSync,readdirSync,realpathSync,renameSync,statSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import type {PartitionFactsV1,StorageFactsV1} from "../../🟦️.ts";
import {readJsonFileV1,type DoctorPortsV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1,DoctorRepairRunV1} from "../📋️contract/🟦️.ts";
import {tryImageLeaseV1} from "./🔒️image-lease/🟦️.ts";
import {hostShareCapacityV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📐️elastic/🟦️.ts";
import {NATIVE_IMAGE_GROW_V1,createNativeImageDarwinGrowPortV1,growNativeImageV1,nativeImageGrowAdmissionV1,nativeImageGrowTextV1,nativeImageGrownReceiptV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📈️grow/🟦️.ts";
import {requireMaterializedPathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireNativeImageHostOwnersV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/👤️owners/🟦️.ts";
import {attachNativeImageV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🔗️attach/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {nativeStorageBackendsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🟦️.ts";
import {nativeImageInventoryV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📥️information/🟦️.ts";
import {nativeImageStrandedMountV1,recoverNativeImageStrandedV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🏚️stranded/🟦️.ts";

const BACKING="fixed.dmg",MOUNT="m",PUBLICATION=".publication",HDIUTIL="/usr/bin/hdiutil";
const entry=(path:string)=>{try{return lstatSync(path);}catch{return undefined;}};

/** 📏️ The bytes the writers of one partition need on this platform, as the cache policy declares them (`needBytes`), `null` for a partition without a declared need. */
const declaredNeedV1=(id:string,platform:string):number|null=>(budget.partitions as Record<string,{needBytes?:Record<string,number>}>)[id]?.needBytes?.[platform]??null;

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
 return {id,state,...strandedFacts,capacityBytes,freeBytes,imageBytes:backingEntry?backingEntry.size:null,backing:directoryHost?"none":backingEntry?"present":"missing",publication:residue??"none",publicationAgeSeconds:age,image,needBytes:directoryHost?null:declaredNeedV1(id,ports.platform)};
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

/** 🧭️ The partitions the grow repair touches: a mounted image of a partition with a declared need whose filesystem plus one unit is below it. Pure and language-agnostic.
 * @see ./🧪️tests/🟦️.ts */
export function imageGrowPlanV1(storage:Pick<StorageFactsV1,"partitions"> | undefined,trailerBytes:number,unitBytes:number):readonly string[]{
 return (storage?.partitions??[]).filter(row=>row.image!=="damaged"&&row.state==="mounted"&&row.imageBytes!==null&&typeof row.needBytes==="number"&&row.needBytes>0&&row.imageBytes-trailerBytes+unitBytes<row.needBytes).map(row=>row.id);
}

/** 📈️ What one grow pass did with one image: `grown` (with the bytes added), or `skipped` for `held` (an opener keeps the image), `current` (it already holds the target) or `unfit` (the host cannot hold the larger envelope: the numbers in MiB, `needed` being the free bytes the host lacks), each with the bilingual line. */
export type ImageGrowRowV1=Readonly<{id:string;state:"grown";addedBytes:number}|{id:string;state:"skipped";kind:"held"|"current"|"unfit";values:Readonly<Record<string,number|string>>;note:string}|{id:string;state:"failed";note:string}>;

/** 📈️ Grows the images that are smaller than the declared need of this platform, each under its exclusive lease (tried once, a running opener keeps the image and nothing changes): the image is detached while idle, its backing is stretched without writing a byte, it is attached again and its receipt is rewritten with the new sizes; progress and cancellation follow the signal of the ports. A host that cannot hold the larger envelope is named unfit and the image stays as it is. A dry run only names them. Only macOS has images (an empty answer elsewhere). */
export async function growImagesV1(ports:DoctorPortsV1,dryRun:boolean):Promise<readonly ImageGrowRowV1[]>{
 if(ports.platform!=="darwin")return [];
 const plan=imageGrowPlanV1(await readStorage(ports),NATIVE_IMAGE_GROW_V1.trailerBytes,NATIVE_IMAGE_GROW_V1.unitBytes),relative=budget.partitions as Record<string,{directory:string;capacityBytes:number;hostSharePercent?:number}>,rows:ImageGrowRowV1[]=[];
 for(const id of plan){
  if(dryRun){rows.push({id,state:"grown",addedBytes:0});continue;}
  try{rows.push(await growImage(ports,id,requireMaterializedPathV1(directoryOf(resolve(ports.root),relative[id]!.directory)),relative[id]!));}
  catch(error){if(ports.signal.aborted)throw error;rows.push({id,state:"failed",note:`${id}: ${(error instanceof Error?error.message:String(error)).split("\n")[0]!.slice(0,200)}`});}
 }
 return Object.freeze(rows);
}

/** 📈️ The repair of the images below their need: the pass above as one repair run. */
async function repairGrow(ports:DoctorPortsV1,dryRun:boolean):Promise<DoctorRepairRunV1>{
 if(ports.platform!=="darwin")return {removed:0,bytes:0,status:"unavailable",reason:`Not applicable on ${ports.platform}: its partitions are plain directories with no image to grow / Nicht anwendbar auf ${ports.platform}: Die Partitionen sind gewöhnliche Verzeichnisse ohne wachsendes Image`};
 const rows=await growImagesV1(ports,dryRun);
 if(rows.length===0)return {removed:0,bytes:0,status:"nothing"};
 const grown=rows.filter(row=>row.state==="grown"),notes=rows.flatMap(row=>row.state==="grown"?[]:[row.note]),failed=rows.find(row=>row.state==="failed");
 if(dryRun)return {removed:rows.length,bytes:0};
 if(grown.length===0&&failed!==undefined)throw Error(failed.note);
 if(grown.length===0)return {removed:0,bytes:0,status:"unavailable",reason:notes.join("; ")};
 return {removed:grown.length,bytes:grown.reduce((sum,row)=>sum+(row.state==="grown"?row.addedBytes:0),0),...notes.length>0?{reason:notes.join("; ")}:{}};
}

/** 📈️ Grows one image: the target is the declared need or the host share of the free bytes, whichever is larger, never above the declared capacity. */
async function growImage(ports:DoctorPortsV1,id:string,directory:string,row:Readonly<{capacityBytes:number;hostSharePercent?:number}>):Promise<ImageGrowRowV1>{
 const both=(key:"held"|"current"|"unfit",values:Readonly<Record<string,number|string>>)=>nativeImageGrowTextV1("en",key,values)+" / "+nativeImageGrowTextV1("de",key,values);
 const lease=await tryImageLeaseV1(ports,id,directory);
 if(lease==="busy"){const values={image:id};return {id,state:"skipped",kind:"held",values,note:both("held",values)};}
 try{
  const real=realpathSync(directory),image=join(real,BACKING),mount=join(real,MOUNT),receiptPath=join(real,"image.json"),receipt=readJsonFileV1(receiptPath,65536) as Record<string,unknown>|undefined,MiB=NATIVE_IMAGE_GROW_V1.unitBytes;
  if(receipt===undefined||typeof receipt.filesystemBytes!=="number")throw Error(`The receipt of ${id} is unreadable / Der Beleg von ${id} ist nicht lesbar`);
  const need=declaredNeedV1(id,ports.platform)??0,hostFree=nativeFreeSpaceSyncV1(real)?.availableBytes,capacity=hostShareCapacityV1(row.capacityBytes,row.hostSharePercent??0,hostFree??0,need),target=capacity-MiB,current=receipt.filesystemBytes;
  const verdict=nativeImageGrowAdmissionV1({currentBytes:current,targetBytes:target,hostFreeBytes:hostFree});
  const values={image:id,from:Math.round(current/MiB),to:Math.round(target/MiB),free:Math.floor((hostFree??0)/MiB),needed:Math.ceil((target+NATIVE_IMAGE_GROW_V1.initialBytes-current)/MiB)};
  if(verdict==="already")return {id,state:"skipped",kind:"current",values,note:both("current",values)};
  if(verdict==="host-short")return {id,state:"skipped",kind:"unfit",values,note:both("unfit",values)};
  const port=createNativeImageDarwinGrowPortV1({image,cwd:real,mountPoint:mount,run:(program,args)=>ports.command(program,[...args]),signal:ports.signal});
  await growNativeImageV1(port,{check:()=>ports.signal.throwIfAborted(),progress:(done,total)=>ports.progress(`storage-grow:${id}:${done}/${total}`),backingBytes:()=>lstatSync(image).size},target);
  const temporary=receiptPath+".grow",written=nativeImageGrownReceiptV1(receipt,target);
  writeFileSync(temporary,JSON.stringify(written),{mode:0o600});renameSync(temporary,receiptPath);
  return {id,state:"grown",addedBytes:target-current};
 }finally{lease?.release();}
}

/** 💽️ The disk-level facts: image state, free space against budget, unsettled publications and the free space of the disk that holds the caches; the repair puts stranded and detached images back at their mount point. */
export const storageProbeV1:DoctorProbeV1=Object.freeze({
 area:"storage",phase:1,
 repairs:[Object.freeze({id:"storage-images",area:"storage" as const,run:(ports:DoctorPortsV1,_context:unknown,dryRun:boolean)=>repairImages(ports,dryRun)}),Object.freeze({id:"storage-grow",area:"storage" as const,run:(ports:DoctorPortsV1,_context:unknown,dryRun:boolean)=>repairGrow(ports,dryRun)})],
 async read(ports){return {sections:{storage:await readStorage(ports)}};},
});
