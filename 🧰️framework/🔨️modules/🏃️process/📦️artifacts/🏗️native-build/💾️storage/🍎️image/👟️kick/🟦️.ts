import{closeSync,fsyncSync,lstatSync,mkdtempSync,openSync,rmSync,rmdirSync,statfsSync,writeSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import{nativeImageInformationV1}from"../📥️information/🟦️.ts";
import{nativeImageBoundedRunnerV1,nativeImageReclaimDueV1,type NativeImageReclaimRuleV1}from"../♻️reclaim/🟦️.ts";
const protocol=schema.definitions.Protocol.const;
type CommandRunnerV1=(program:string,args:readonly string[])=>Promise<string>;
type MechanismV1=Readonly<{kind:string}>;
/** 👟️ The authored protocol of the in-place return of freed blocks.
 * @see ./🧬️schema/🔣️.json */
export const NATIVE_IMAGE_KICK_V1=protocol;
/** 🧭️ Names how an operating system returns freed blocks while the image stays mounted and in use (`sibling-volume`, `fstrim-mounted`), or `offline-only` where it cannot (Windows: a dynamic VHDX compacts only while dismounted), or null where the repository has no image. */
export function nativeImageKickMechanismV1(platform:string):MechanismV1|null{return(protocol.mechanisms as Record<string,MechanismV1>)[platform==="win32"?"windows":platform]??null;}
/** 🧲️ Decides whether an in-place kick is worth its two seconds: the image holds more host blocks than it uses by the reclaim rule (the same floor, ratio and tight-host relief as the offline cycle) and the last kick is at least `minimumIntervalMilliseconds` ago. Pure and language-agnostic. */
export function nativeImageKickDueV1(rule:NativeImageReclaimRuleV1,facts:Readonly<{allocatedBytes:number;usedBytes:number;hostFreeBytes:number;sinceLastKickMilliseconds:number}>,minimumIntervalMilliseconds:number=protocol.minimumIntervalMilliseconds):boolean{
 return facts.sinceLastKickMilliseconds>=minimumIntervalMilliseconds&&nativeImageReclaimDueV1(rule,facts.allocatedBytes,facts.usedBytes,facts.hostFreeBytes);
}
/** 🔌️ Supplies the effects of one kick; `ensure` prepares what the platform needs (macOS: the sibling volume of the container, created online once), `kick` makes the filesystem flush its trim, `allocatedBytes` reads the host blocks of the backing and `dispose` undoes every effect of the cycle, also after a failure. */
export type NativeImageKickPortV1=Readonly<{ensure():Promise<void>;kick():Promise<void>;allocatedBytes():number;dispose():Promise<void>;canary?():Readonly<{bytes:number;allocatedAfterWrite:number}>|undefined}>;
/** 🚦️ Supplies cancellation, progress and the settle sleep to the kick. */
export type NativeImageKickHooksV1=Readonly<{check:()=>void;progress?:(done:number,total:number)=>void;sleep?:(milliseconds:number)=>Promise<void>}>;
/** 📊️ The host allocation before and after one kick, whether it stopped shrinking within the bound and whether the kick provably works: a port with a canary (a test file written and deleted in the sibling volume) is effective when at least `canaryReturnedPercent` of it came back, a port without one when the allocation fell. */
export type NativeImageKickOutcomeV1=Readonly<{beforeBytes:number;afterBytes:number;settled:boolean;effective:boolean;canaryBytes?:number;canaryReturnedBytes?:number}>;
/** 🔢️ How many bytes came back after the canary was written: the sum of every decrease between consecutive allocation samples starting at the allocation right after the write. Builds that keep growing the backing add increases, which this ignores, so a busy image cannot hide the return. Pure and language-agnostic. */
export function nativeImageKickReturnedBytesV1(allocatedAfterWrite:number,samples:readonly number[]):number{let previous=allocatedAfterWrite,returned=0;for(const sample of samples){returned+=Math.max(0,previous-sample);previous=sample;}return returned;}
/** 🔢️ Whether a canary of `bytes` came back: the returned bytes reach the percentage of it. Pure and language-agnostic. */
export function nativeImageKickCanaryReturnedV1(canary:Readonly<{bytes:number}>,returnedBytes:number,percent:number=protocol.canaryReturnedPercent):boolean{return returnedBytes*100>=canary.bytes*percent;}
/** 💬️ The bilingual line of an image whose kick is ineffective. */
export function nativeImageKickIneffectiveTextV1(language:"en"|"de",values:Readonly<{image:string;returned:string;written:string}>):string{return protocol.messages.ineffective[language].replace(/\{(\w+)\}/gu,(_,key:string)=>values[key as keyof typeof values]??"");}
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const fill=(template:readonly string[],values:Readonly<Record<string,string>>):string[]=>template.map(part=>part.replace(/<([A-Za-z]+)>/gu,(_,key)=>values[key]??_));
/** 👟️ Returns the blocks the mounted image freed without detaching it, resizing it or touching its busy volume: prepares the platform, kicks, waits until the host allocation stops shrinking, and always disposes. Cancellation is honored between steps and polls; a failure of the kick and of the disposal are both retained. */
export async function kickNativeImageV1(port:NativeImageKickPortV1,hooks:NativeImageKickHooksV1):Promise<NativeImageKickOutcomeV1>{
 const total=protocol.progressSteps,sleep=hooks.sleep??pause,progress=hooks.progress??(()=>{}),failures:unknown[]=[],samples:number[]=[];
 hooks.check();progress(0,total);
 const beforeBytes=port.allocatedBytes();let settled=false;
 try{
  await port.ensure();hooks.check();progress(1,total);
  await port.kick();progress(2,total);
  let last=port.allocatedBytes(),stable=0,waited=0;samples.push(last);
  while(stable<protocol.settleStablePolls&&waited<protocol.maximumSettleMilliseconds){
   hooks.check();await sleep(protocol.settlePollMilliseconds);waited+=protocol.settlePollMilliseconds;
   const now=port.allocatedBytes();samples.push(now);stable=now===last?stable+1:0;last=now;
  }
  settled=stable>=protocol.settleStablePolls;
 }catch(error){failures.push(error);}
 try{await port.dispose();}catch(error){failures.push(error);}
 if(failures.length>1)throw new AggregateError(failures,"Native image kick and disposal failed",{cause:failures[0]});
 if(failures.length===1)throw failures[0];
 progress(total,total);
 const afterBytes=port.allocatedBytes(),canary=port.canary?.(),returned=canary===undefined?0:nativeImageKickReturnedBytesV1(canary.allocatedAfterWrite,[...samples,afterBytes]);
 return Object.freeze({beforeBytes,afterBytes,settled,effective:canary===undefined?afterBytes<beforeBytes:nativeImageKickCanaryReturnedV1(canary,returned),...canary===undefined?{}:{canaryBytes:canary.bytes,canaryReturnedBytes:returned}});
}
/** 🍎️ Builds the macOS port: `diskutil info` names the APFS container of the mount point, `diskutil apfs list` finds the sibling volume `semio-trim` and `diskutil apfs addVolume -nomount` creates it online when absent (a container below the minimum size refuses). The kick mounts that sibling at a private scratch mount point and unmounts it: the unmount makes the container flush its trim to the UNMAP-capable image stack while the main volume stays mounted and in use. Disposal unmounts the sibling again (forced: only this cycle ever uses it), deletes it (an image with a second volume cannot be attached at a mount point again: `diskutil image attach --mountPoint` works only for an image with a single mountable entity) and removes the scratch directory. See https://ss64.com/mac/diskutil.html */
export function createNativeImageDarwinKickPortV1(given:Readonly<{image:string;mountPoint:string;run:CommandRunnerV1;cleanupRun?:CommandRunnerV1;allocated?:(image:string)=>number;signal?:AbortSignal;commandMilliseconds?:number}>):NativeImageKickPortV1{
 const mechanism=protocol.mechanisms.darwin,bound=(run:CommandRunnerV1,signal?:AbortSignal)=>nativeImageBoundedRunnerV1(run,{milliseconds:given.commandMilliseconds??protocol.commandMilliseconds,...signal===undefined?{}:{signal}}),run=bound(given.run,given.signal),cleanup=bound(given.cleanupRun??given.run);
 let device:string|undefined,scratch:string|undefined,mounted=false,measured:Readonly<{bytes:number;allocatedAfterWrite:number}>|undefined;
 const allocated=()=>(given.allocated??(image=>lstatSync(image).blocks*512))(given.image);
 const probe=(directory:string)=>{
  if(lstatSync(directory).dev===lstatSync(dirname(directory)).dev)return;
  const room=statfsSync(dirname(given.image));
  if(Number(room.bavail)*room.bsize<protocol.canaryBytes*protocol.canaryHostFreeMultiplier)return;
  const file=join(directory,protocol.canaryFile),handle=openSync(file,"w");
  try{const chunk=Buffer.alloc(1048576,7);for(let done=0;done<protocol.canaryBytes;done+=chunk.length)writeSync(handle,chunk);fsyncSync(handle);}finally{closeSync(handle);}
  measured=Object.freeze({bytes:protocol.canaryBytes,allocatedAfterWrite:allocated()});
  rmSync(file);const directoryHandle=openSync(directory,"r");try{fsyncSync(directoryHandle);}finally{closeSync(directoryHandle);}
 };
 const find=async(container:string):Promise<string|undefined>=>{
  const containers=nativeImageInformationV1(await run(mechanism.program,fill(mechanism.listArguments,{container})))[mechanism.containersKey];
  for(const row of Array.isArray(containers)?containers as Record<string,unknown>[]:[])for(const volume of Array.isArray(row[mechanism.volumesKey])?row[mechanism.volumesKey] as Record<string,unknown>[]:[])if(volume[mechanism.nameKey]===mechanism.volumeName&&typeof volume[mechanism.deviceKey]==="string")return volume[mechanism.deviceKey] as string;
  return undefined;
 };
 return Object.freeze({
  async ensure(){
   const container=nativeImageInformationV1(await run(mechanism.program,fill(mechanism.infoArguments,{mountPoint:given.mountPoint})))[mechanism.containerKey];
   if(typeof container!=="string"||!/^disk\d+$/u.test(container))throw Error("APFS container of "+given.mountPoint+" required");
   const rows=nativeImageInformationV1(await run(mechanism.ownerProgram,[...mechanism.ownerArguments]))[mechanism.imagesKey],image=(Array.isArray(rows)?rows as Record<string,unknown>[]:[]).find(row=>row[mechanism.imagePathKey]===given.image),entities=image?.[mechanism.entitiesKey];
   if(!Array.isArray(entities)||!(entities as Record<string,unknown>[]).some(entity=>entity[mechanism.entryKey]==="/dev/"+container))throw Error("APFS container "+container+" does not belong to the image "+given.image);
   device=await find(container);
   if(device===undefined){await run(mechanism.program,fill(mechanism.addArguments,{container,filesystem:mechanism.filesystem,name:mechanism.volumeName}));device=await find(container);}
   if(device===undefined)throw Error("Sibling volume "+mechanism.volumeName+" required in "+container);
  },
  async kick(){
   if(device===undefined)throw Error("Ensured sibling volume required");
   scratch=mkdtempSync(join(tmpdir(),protocol.scratchPrefix));
   await run(mechanism.program,fill(mechanism.mountArguments,{scratch,device}));mounted=true;
   try{probe(scratch);}catch{measured=undefined;}
   await run(mechanism.program,fill(mechanism.unmountArguments,{device}));mounted=false;
  },
  allocatedBytes:allocated,
  canary:()=>measured,
  async dispose(){
   try{
    if(mounted&&device!==undefined){try{await cleanup(mechanism.program,fill(mechanism.unmountArguments,{device}));}catch{await cleanup(mechanism.program,fill(mechanism.forceUnmountArguments,{device}));}mounted=false;}
    if(!mounted&&device!==undefined){await cleanup(mechanism.program,fill(mechanism.deleteArguments,{device}));device=undefined;}
   }
   finally{if(scratch!==undefined&&!mounted){try{rmdirSync(scratch);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}scratch=undefined;}}
  }
 });
}
/** 🐧️ Builds the Linux port: the image is a sparse loop file mounted with `discard`, `fstrim` on the mounted filesystem returns its free blocks and the loop layer punches holes into the file (FALLOC_FL_PUNCH_HOLE); nothing is prepared and nothing needs undoing. See https://man7.org/linux/man-pages/man8/fstrim.8.html */
export function createNativeImageLinuxKickPortV1(given:Readonly<{image:string;mountPoint:string;run:CommandRunnerV1;allocated?:(image:string)=>number;signal?:AbortSignal;commandMilliseconds?:number}>):NativeImageKickPortV1{
 const mechanism=protocol.mechanisms.linux,run=nativeImageBoundedRunnerV1(given.run,{milliseconds:given.commandMilliseconds??protocol.commandMilliseconds,...given.signal===undefined?{}:{signal:given.signal}});
 return Object.freeze({
  async ensure(){},
  async kick(){await run(mechanism.program,fill(mechanism.trimArguments,{mountPoint:given.mountPoint}));},
  allocatedBytes(){return(given.allocated??(image=>lstatSync(image).blocks*512))(given.image);},
  async dispose(){}
 });
}
/** 🧭️ Chooses the port of an operating system for one mounted image: the same kick on macOS and Linux, null on Windows (`offline-only`, the caller falls back to the exclusive drain-window cycle) and on every platform without an image. */
export function createNativeImageKickPortV1(platform:string,options:Readonly<{image:string;mountPoint:string;run:CommandRunnerV1;cleanupRun?:CommandRunnerV1;signal?:AbortSignal;commandMilliseconds?:number}>):NativeImageKickPortV1|null{
 if(platform==="darwin")return createNativeImageDarwinKickPortV1(options);
 if(platform==="linux")return createNativeImageLinuxKickPortV1(options);
 return null;
}
