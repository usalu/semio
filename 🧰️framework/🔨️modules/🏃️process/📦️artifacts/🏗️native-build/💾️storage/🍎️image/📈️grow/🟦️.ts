import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import sparseSchema from"../🪶️sparse/🧬️schema/🔣️.json" with { type: "json" };
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import{nativeImageGrowRetryV1,nativeImageSparseAdmissionBytesV1}from"../🪶️sparse/🟦️.ts";
import{createNativeImageDarwinReclaimPortV1,nativeImageBoundedRunnerV1,type NativeImageReclaimHooksV1,type NativeImageReclaimPortV1}from"../♻️reclaim/🟦️.ts";
import{invalidateNativeImageRosterV1}from"../📡️roster/🟦️.ts";
const protocol=schema.definitions.Protocol.const,image=imageSchema.definitions.ImageProtocol.const,sparse=sparseSchema.definitions.Protocol.const;
type CommandRunnerV1=(program:string,args:readonly string[])=>Promise<string>;
/** 📈️ Names the closed protocol of growing an existing image. */
export const NATIVE_IMAGE_GROW_V1=protocol;
/** ⚖️ Decides what one grow does from three numbers: `already` when the image holds at least the target, `host-short` when the host volume cannot hold the whole larger envelope plus the initial extent (`hdiutil resize` refuses any size beyond the free host bytes although the backing stays sparse), else `grow`. Pure and language-agnostic. */
export function nativeImageGrowAdmissionV1(facts:Readonly<{currentBytes:number;targetBytes:number;hostFreeBytes:number|undefined}>):"already"|"grow"|"host-short"{
 if(!Number.isSafeInteger(facts.currentBytes)||!Number.isSafeInteger(facts.targetBytes)||facts.targetBytes%image.unitBytes!==0||facts.targetBytes<image.minimumFilesystemBytes)throw Error("Unit-aligned Native image sizes required");
 if(facts.targetBytes<=facts.currentBytes)return"already";
 return facts.hostFreeBytes!==undefined&&facts.hostFreeBytes<nativeImageSparseAdmissionBytesV1(facts.targetBytes)-facts.currentBytes?"host-short":"grow";
}
/** 🧮️ Builds the closed `hdiutil resize` arguments that stretch the backing of an existing image to `filesystemBytes` without writing a byte. */
export function nativeImageGrowArgumentsV1(filesystemBytes:number,backing:string):readonly string[]{
 if(!Number.isSafeInteger(filesystemBytes)||filesystemBytes%image.unitBytes!==0||filesystemBytes<image.minimumFilesystemBytes)throw Error("Unit-aligned Native image size required");
 return Object.freeze([sparse.growVerb,sparse.growSizeFlag,String(filesystemBytes/image.unitBytes)+sparse.growUnit,backing]);
}
/** 🧾️ Computes the receipt fields a grown image has: the file holds the filesystem plus the trailer, the envelope is the filesystem plus one unit. Pure. */
export function nativeImageGrownReceiptV1(receipt:Readonly<Record<string,unknown>>,filesystemBytes:number):Readonly<Record<string,unknown>>{
 return Object.freeze({...receipt,size:filesystemBytes+protocol.trailerBytes,filesystemBytes,maximumPhysicalBytes:filesystemBytes+image.unitBytes});
}
/** 🔌️ Is a reclaim port that can also stretch the backing of its image while it is detached. */
export type NativeImageGrowPortV1=NativeImageReclaimPortV1&Readonly<{resize(filesystemBytes:number):Promise<void>}>;
/** 🍎️ Builds the macOS port: the reclaim port (attach through `diskutil`, detach through `hdiutil`) plus `hdiutil resize`, retried while another image operation holds the backing (status 35). */
export function createNativeImageDarwinGrowPortV1(given:Parameters<typeof createNativeImageDarwinReclaimPortV1>[0]):NativeImageGrowPortV1{
 const run:CommandRunnerV1=nativeImageBoundedRunnerV1(given.run,{milliseconds:given.commandMilliseconds,...given.signal===undefined?{}:{signal:given.signal}});
 return Object.freeze({...createNativeImageDarwinReclaimPortV1(given),async resize(filesystemBytes:number){await nativeImageGrowRetryV1(async()=>{await run(protocol.resizeProgram,[...nativeImageGrowArgumentsV1(filesystemBytes,given.image)]);});}});
}
/** 📊️ Reports the backing size before and after one grow cycle. */
export type NativeImageGrowOutcomeV1=Readonly<{beforeBytes:number;afterBytes:number}>;
/** 📈️ Grows an idle image in place: it detaches the image when attached (a busy image refuses and nothing changes), stretches the backing to `filesystemBytes` without writing a byte, attaches it again (the volume takes the whole size at attach) and leaves it as it found it; the image is always attached again after a failure or a cancellation, so a refused or cancelled grow never leaves the volume away. Progress is reported in four steps, cancellation is honored between them. */
export async function growNativeImageV1(port:NativeImageGrowPortV1,hooks:NativeImageReclaimHooksV1&Readonly<{backingBytes():number}>,filesystemBytes:number):Promise<NativeImageGrowOutcomeV1>{
 const total=protocol.progressSteps,progress=hooks.progress??(()=>{});
 hooks.check();progress(0,total);
 const beforeBytes=hooks.backingBytes(),prior=await port.attachedDevice();
 if(prior!==null){await port.detach(prior);invalidateNativeImageRosterV1();}
 const failures:unknown[]=[];
 try{
  progress(1,total);hooks.check();
  await port.resize(filesystemBytes);
  progress(2,total);
 }catch(error){failures.push(error);}
 try{
  const device=await port.attachedDevice("cleanup");
  if(port.mountsCanonically===true&&prior!==null){if(device===null)await port.attach("cleanup");}
  else if(device!==null)await port.detach(device,"cleanup");
  port.dispose();
 }catch(error){failures.push(error);}finally{invalidateNativeImageRosterV1();}
 if(failures.length>1)throw new AggregateError(failures,"Native image grow and cleanup failed",{cause:failures[0]});
 if(failures.length===1)throw failures[0];
 progress(3,total);hooks.check();progress(total,total);
 return Object.freeze({beforeBytes,afterBytes:hooks.backingBytes()});
}
/** 💬️ Renders one closed grow message in one language. */
export function nativeImageGrowTextV1(language:"en"|"de",key:keyof typeof protocol.messages,values:Readonly<Record<string,string|number>>):string{
 return protocol.messages[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]??""));
}
