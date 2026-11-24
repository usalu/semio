import schema from"../👟️kick/🧬️schema/🔣️.json" with { type: "json" };
import{nativeImageInventoryV1,nativeImageInformationV1}from"../📥️information/🟦️.ts";
import{nativeImageAttachArgumentsV1,nativeImageAttachProgramV1}from"../👤️owners/🟦️.ts";
type CommandRunnerV1=(program:string,args:readonly string[])=>Promise<string>;
type RecordV1=Record<string,unknown>;
const mechanism=schema.definitions.Protocol.const.mechanisms.darwin,hdiutil=mechanism.ownerProgram,list=(value:unknown):RecordV1[]=>Array.isArray(value)?value.filter((row):row is RecordV1=>row!==null&&typeof row==="object"):[];
const fill=(template:readonly string[],values:Readonly<Record<string,string>>):string[]=>template.map(part=>part.replace(/<([A-Za-z]+)>/gu,(_,key)=>values[key]??_));
const rowOf=async(run:CommandRunnerV1,image:string):Promise<RecordV1|undefined>=>list(nativeImageInventoryV1(await run(hdiutil,[...mechanism.ownerArguments])).images).find(row=>row[mechanism.imagePathKey]===image);
/** 🧽️ Deletes every sibling volume `semio-trim` that a killed in-place kick left in the containers of an image: the image attaches without a mount (a volume of a failed attach never mounts), each container is listed through `diskutil apfs list` and the sibling volumes are deleted, and an image that this call attached, or that holds no mount, is detached again. Returns whether a volume was deleted; an image that is mounted is never touched, its sibling volume may belong to a kick that is running. macOS only; the volume is the kick's own scratch volume and holds nothing.
 * @see ../👟️kick/🟦️.ts */
export async function removeNativeImageSiblingVolumesV1(run:CommandRunnerV1,image:string):Promise<boolean>{
 let row=await rowOf(run,image),own=false;
 if(row===undefined){await run(nativeImageAttachProgramV1,["image","attach","--noMount","--nobrowse",image]);row=await rowOf(run,image);own=true;}
 if(row===undefined)return false;
 const entities=list(row[mechanism.entitiesKey]),devices=entities.map(entity=>entity[mechanism.entryKey]).filter((value):value is string=>typeof value==="string"&&/^\/dev\/disk[0-9]+$/u.test(value)),mounted=entities.some(entity=>typeof entity["mount-point"]==="string");
 if(mounted)return false;
 let deleted=false;
 try{
  for(const device of devices){
   let containers:RecordV1[]=[];try{containers=list(nativeImageInformationV1(await run(mechanism.program,fill(mechanism.listArguments,{container:device.slice(5)})))[mechanism.containersKey]);}catch{continue;}
   for(const volume of containers.flatMap(container=>list(container[mechanism.volumesKey])))if(volume[mechanism.nameKey]===mechanism.volumeName&&typeof volume[mechanism.deviceKey]==="string"){await run(mechanism.program,fill(mechanism.deleteArguments,{device:volume[mechanism.deviceKey] as string}));deleted=true;}
  }
 }finally{if((own||!mounted)&&devices[0]!==undefined)await run(hdiutil,["detach",devices[0]]);}
 return deleted;
}
/** 🔗️ Attaches an image at its mount point and recovers from the one fault that blocks it without a trace in the image: a sibling volume `semio-trim` of a killed kick makes `diskutil image attach --mountPoint` fail with "Failed to mount the volume"; the call deletes the sibling volumes and attaches again, any other failure stays the original error. */
export async function attachNativeImageV1(run:CommandRunnerV1,root:string,image:string):Promise<string>{
 try{return await run(nativeImageAttachProgramV1,[...nativeImageAttachArgumentsV1(root,image)]);}
 catch(error){
  let recovered=false;try{recovered=await removeNativeImageSiblingVolumesV1(run,image);}catch{recovered=false;}
  if(!recovered)throw error;
  return run(nativeImageAttachProgramV1,[...nativeImageAttachArgumentsV1(root,image)]);
 }
}
