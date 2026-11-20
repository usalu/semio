import{lstatSync,mkdtempSync,rmdirSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import{nativeImageAttachArgumentsV1,nativeImageAttachProgramV1}from"../👤️owners/🟦️.ts";
import{nativeImageInformationV1}from"../📥️information/🟦️.ts";
import{invalidateNativeImageRosterV1}from"../📡️roster/🟦️.ts";
import{nativeImageHdiutilProgramV1}from"../🛠️hdiutil/🟦️.ts";
const protocol=imageSchema.definitions.ImageProtocol.const.reclaim;
/** 📏️ Names the three numbers that decide when host blocks held by an image count as bloat. */
export type NativeImageReclaimRuleV1=Readonly<{reclaimFloorBytes:number;reclaimRatioPercent:number;reclaimTightHostFreeBytes:number}>;
/** 🧲️ Decides whether an image holds host blocks it no longer uses: the host allocation exceeds the bytes used inside by at least the floor (an eighth of it while the host volume is nearly full) and by the ratio. Pure and language-agnostic. */
export function nativeImageReclaimDueV1(rule:NativeImageReclaimRuleV1,allocatedBytes:number,usedBytes:number,hostFreeBytes:number):boolean{
 const floor=hostFreeBytes<rule.reclaimTightHostFreeBytes?Math.floor(rule.reclaimFloorBytes/8):rule.reclaimFloorBytes;
 return allocatedBytes-usedBytes>=floor&&allocatedBytes*100>=usedBytes*rule.reclaimRatioPercent;
}
/** 🧭️ Names how an operating system returns freed blocks to the host, or null where the repository has no image that could hold any. */
export function nativeImageReclaimMechanismV1(platform:string):Readonly<{mechanism:string;program:string;detachProgram:string;imageName:string}>|null{
 return (protocol.mechanisms as Record<string,{mechanism:string;program:string;detachProgram:string;imageName:string}>)[platform==="win32"?"windows":platform]??null;
}
/** 🔌️ Supplies the effects of one reclaim cycle; every platform implements the same steps; `trim` is the explicit step of the platforms whose attach does not trim by itself. */
export type NativeImageReclaimPortV1=Readonly<{attachedDevice(phase?:"cleanup"):Promise<string|null>;detach(device:string,phase?:"cleanup"):Promise<void>;attach():Promise<void>;trim?():Promise<void>;allocatedBytes():number;dispose():void}>;
/** 🚦️ Supplies cancellation and progress to the cycle. */
export type NativeImageReclaimHooksV1=Readonly<{check:()=>void;progress?:(done:number,total:number)=>void;sleep?:(milliseconds:number)=>Promise<void>}>;
/** 📊️ Reports the host allocation before and after one cycle and whether it settled within the bound. */
export type NativeImageReclaimOutcomeV1=Readonly<{beforeBytes:number;afterBytes:number;settled:boolean}>;
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
/** ♻️ Returns the blocks freed inside an idle image to the host without touching its bytes, its size or its identity: it detaches the image when attached (a busy image refuses and nothing changes), attaches it through the UNMAP-capable stack so the volume trims its free space, waits until the host allocation stops shrinking and detaches again; the image stays inactive for its next opener. Cancellation is honored between steps and between polls, and cleanup attempts to detach the exact image after attachment is attempted. The caller holds the exclusive image lease. */
export async function reclaimNativeImageV1(port:NativeImageReclaimPortV1,hooks:NativeImageReclaimHooksV1):Promise<NativeImageReclaimOutcomeV1>{
 const total=protocol.progressSteps,sleep=hooks.sleep??pause,progress=hooks.progress??(()=>{});
 hooks.check();progress(0,total);
 const beforeBytes=port.allocatedBytes(),prior=await port.attachedDevice();
 if(prior!==null){await port.detach(prior);invalidateNativeImageRosterV1();}
 progress(1,total);hooks.check();
 let settled=false;
 const failures:unknown[]=[];
 try{
  await port.attach();invalidateNativeImageRosterV1();
  progress(2,total);
  if(port.trim){hooks.check();await port.trim();}
  let last=port.allocatedBytes(),stable=0,waited=0;
  while(stable<protocol.settleStablePolls&&waited<protocol.maximumSettleMilliseconds){
   hooks.check();await sleep(protocol.settlePollMilliseconds);waited+=protocol.settlePollMilliseconds;
   const now=port.allocatedBytes();stable=now===last?stable+1:0;last=now;
  }
  settled=stable>=protocol.settleStablePolls;progress(3,total);
 }catch(error){failures.push(error);}
 try{
  const device=await port.attachedDevice("cleanup");
  if(device!==null)await port.detach(device,"cleanup");
  port.dispose();
 }catch(error){failures.push(error);}finally{invalidateNativeImageRosterV1();}
 if(failures.length>1)throw new AggregateError(failures,"Native image reclaim and cleanup failed",{cause:failures[0]});
 if(failures.length===1)throw failures[0];
 const afterBytes=port.allocatedBytes();progress(total,total);
 return Object.freeze({beforeBytes,afterBytes,settled});
}

/** 🧹️ Removes only the empty private mount directory after the image is detached. */
function removeReclaimScratchV1(scratch:string):void{
 try{rmdirSync(scratch);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
}
/** 🍎️ Builds the macOS port of one image: attach through `diskutil image attach` at a private scratch mount point, detach the whole device through `hdiutil detach`, allocation from the blocks of the backing file. See https://ss64.com/mac/diskutil.html */
export function createNativeImageDarwinReclaimPortV1(options:Readonly<{image:string;cwd:string;run:CommandRunnerV1;cleanupRun?:CommandRunnerV1}>):NativeImageReclaimPortV1{
 let scratch:string|undefined;
 const runner=(phase?:"cleanup")=>phase===undefined?options.run:options.cleanupRun??options.run;
 return Object.freeze({
  async attachedDevice(phase?:"cleanup"){
   const information=nativeImageInformationV1(await runner(phase)(nativeImageHdiutilProgramV1,["info","-plist"])),rows=Array.isArray(information.images)?information.images as Record<string,unknown>[]:[];
   const row=rows.find(candidate=>candidate["image-path"]===options.image),entities=row?.["system-entities"];
   const device=Array.isArray(entities)&&entities.length?(entities[0] as Record<string,unknown>)["dev-entry"]:undefined;
   return typeof device==="string"?device:null;
  },
  async detach(device:string,phase?:"cleanup"){await runner(phase)(nativeImageHdiutilProgramV1,["detach",device]);},
  async attach(){scratch=mkdtempSync(join(tmpdir(),protocol.scratchPrefix));await options.run(nativeImageAttachProgramV1,[...nativeImageAttachArgumentsV1(scratch,options.image)]);},
  allocatedBytes(){return lstatSync(options.image).blocks*512;},
  dispose(){if(scratch!==undefined){removeReclaimScratchV1(scratch);scratch=undefined;}}
 });
}

const fill=(template:readonly string[],values:Readonly<Record<string,string>>):string[]=>template.map(part=>part.replace(/<([A-Za-z]+)>/gu,(_,key)=>values[key]??_));
type CommandRunnerV1=(program:string,args:readonly string[])=>Promise<string>;
/** 🐧️ Builds the Linux port of one loop-backed image file: `losetup` finds and attaches the loop device, `mount -o discard` mounts it at a private scratch mount point, `fstrim` returns the free blocks of the volume and the loop layer punches holes into the sparse file (FALLOC_FL_PUNCH_HOLE); detach unmounts every mount of the device (a busy one refuses) and releases the loop device; allocation from the blocks of the backing file. See https://man7.org/linux/man-pages/man8/fstrim.8.html */
export function createNativeImageLinuxReclaimPortV1(options:Readonly<{image:string;run:CommandRunnerV1;allocated?:(image:string)=>number}>):NativeImageReclaimPortV1{
 const linux=protocol.linux;let scratch:string|undefined,device:string|undefined;
 const first=(text:string)=>text.split(/\r?\n/u).map(row=>row.trim()).find(row=>row!=="")??null;
 return Object.freeze({
  async attachedDevice(){return first(await options.run(linux.losetup,fill(linux.associatedArguments,{image:options.image})));},
  async detach(target:string){
   const mounts=(await options.run(linux.findmnt,fill(linux.mountsArguments,{device:target}))).split(/\r?\n/u).map(row=>row.trim()).filter(row=>row!=="");
   for(const mountPoint of mounts.reverse())await options.run(linux.umount,fill(linux.unmountArguments,{mountPoint}));
   await options.run(linux.losetup,fill(linux.detachArguments,{device:target}));
  },
  async attach(){
   scratch=mkdtempSync(join(tmpdir(),protocol.scratchPrefix));
   device=first(await options.run(linux.losetup,fill(linux.attachArguments,{image:options.image})))??undefined;
   if(device===undefined)throw Error("Loop device required for "+options.image);
   try{await options.run(linux.mount,fill(linux.mountArguments,{device,mountPoint:scratch}));}
   catch(error){
    try{await options.run(linux.losetup,fill(linux.detachArguments,{device}));}
    catch(cleanup){throw new AggregateError([error,cleanup],"Native loop mount and release failed",{cause:error});}
    throw error;
   }
  },
  async trim(){if(scratch===undefined)throw Error("Mounted image required");await options.run(linux.fstrim,fill(linux.trimArguments,{mountPoint:scratch}));},
  allocatedBytes(){return(options.allocated??(image=>lstatSync(image).blocks*512))(options.image);},
  dispose(){if(scratch!==undefined){removeReclaimScratchV1(scratch);scratch=undefined;}device=undefined;}
 });
}
/** 🪟️ Builds the Windows port of one dynamic VHDX: PowerShell reports whether the file is attached, `Mount-VHD -ReadOnly -NoDriveLetter` attaches it without a drive letter, `Optimize-VHD -Mode Full` compacts it from the unused blocks of the volume and `Dismount-VHD` detaches it; the host allocation of a dynamic VHDX is its file size. See https://learn.microsoft.com/powershell/module/hyper-v/optimize-vhd */
export function createNativeImageWindowsReclaimPortV1(options:Readonly<{image:string;run:CommandRunnerV1;allocated?:(image:string)=>number}>):NativeImageReclaimPortV1{
 const windows=protocol.windows,script=(text:string)=>options.run(windows.program,[...windows.arguments,fill([text],{image:options.image.replaceAll("'","''")})[0]!]);
 return Object.freeze({
  async attachedDevice(){return /^true$/iu.test((await script(windows.attachedScript)).trim())?options.image:null;},
  async detach(){await script(windows.detachScript);},
  async attach(){await script(windows.attachScript);},
  async trim(){await script(windows.optimizeScript);},
  allocatedBytes(){return(options.allocated??(image=>lstatSync(image).size))(options.image);},
  dispose(){}
 });
}
/** 🧭️ Chooses the port of an operating system for one image: the same cycle on every platform, null where the repository has no image that could hold freed blocks. */
export function createNativeImageReclaimPortV1(platform:string,options:Readonly<{image:string;cwd:string;run:CommandRunnerV1;cleanupRun?:CommandRunnerV1}>):NativeImageReclaimPortV1|null{
 if(platform==="darwin")return createNativeImageDarwinReclaimPortV1(options);
 if(platform==="linux")return createNativeImageLinuxReclaimPortV1(options);
 if(platform==="windows"||platform==="win32")return createNativeImageWindowsReclaimPortV1(options);
 return null;
}
