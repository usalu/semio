import{chmodSync,lstatSync,mkdtempSync,rmdirSync,statfsSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import{attachNativeImageV1}from"../🔗️attach/🟦️.ts";
import{nativeImageInformationV1}from"../📥️information/🟦️.ts";
import{invalidateNativeImageRosterV1}from"../📡️roster/🟦️.ts";
import{nativeImageHdiutilProgramV1}from"../🛠️hdiutil/🟦️.ts";
import{nativeImageContainerFreeBytesSyncV1,nativeImageForceDetachArgumentsV1,nativeImageRefreshMechanismV1,nativeImageVolumeLatchedV1}from"../⚖️availability/🟦️.ts";
import{nativeImageGuardedMountModeV1}from"../../🌐️platform/📋️contract/📂️mount/🟦️.ts";
const protocol=imageSchema.definitions.ImageProtocol.const.reclaim;
/** 📏️ Names the three numbers that decide when host blocks held by an image count as bloat. */
export type NativeImageReclaimRuleV1=Readonly<{reclaimFloorBytes:number;reclaimRatioPercent:number;reclaimTightHostFreeBytes:number}>;
/** 🧲️ Decides whether an image holds host blocks it no longer uses: the host allocation exceeds the bytes used inside by at least the floor and by the ratio; while the host volume is nearly full the floor is an eighth of it and the ratio no longer applies, because every returned block counts. Pure and language-agnostic. */
export function nativeImageReclaimDueV1(rule:NativeImageReclaimRuleV1,allocatedBytes:number,usedBytes:number,hostFreeBytes:number):boolean{
 const tight=hostFreeBytes<rule.reclaimTightHostFreeBytes,floor=tight?Math.floor(rule.reclaimFloorBytes/8):rule.reclaimFloorBytes;
 return allocatedBytes-usedBytes>=floor&&(tight||allocatedBytes*100>=usedBytes*rule.reclaimRatioPercent);
}
/** 🧭️ Names how an operating system returns freed blocks to the host, or null where the repository has no image that could hold any. */
export function nativeImageReclaimMechanismV1(platform:string):Readonly<{mechanism:string;program:string;detachProgram:string;imageName:string}>|null{
 return (protocol.mechanisms as Record<string,{mechanism:string;program:string;detachProgram:string;imageName:string}>)[platform==="win32"?"windows":platform]??null;
}
/** 🔌️ Supplies the effects of one reclaim cycle; every platform implements the same steps; `volumeAvailableBytes` reads what the mounted volume reports as available; `trim` is the explicit step of the platforms whose attach does not trim by itself; `mountsCanonically` says that `attach` mounts the image at its own mount point instead of a private scratch mount, so the cycle can give an image that was attached back attached and no exit path leaves it anywhere else. */
export type NativeImageReclaimPortV1=Readonly<{attachedDevice(phase?:"cleanup"):Promise<string|null>;detach(device:string,phase?:"cleanup"):Promise<void>;attach(phase?:"cleanup"):Promise<void>;trim?():Promise<void>;allocatedBytes():number;dispose():void;mountsCanonically?:boolean;volumeAvailableBytes?():number|undefined}>;
/** 🚦️ Supplies cancellation and progress to the cycle. */
export type NativeImageReclaimHooksV1=Readonly<{check:()=>void;progress?:(done:number,total:number)=>void;sleep?:(milliseconds:number)=>Promise<void>}>;
/** 📊️ Reports the host allocation before and after one cycle and whether it settled within the bound. */
export type NativeImageReclaimOutcomeV1=Readonly<{beforeBytes:number;afterBytes:number;settled:boolean}>;
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
/** ♻️ Returns the blocks freed inside an idle image to the host without touching its bytes, its size or its identity: it detaches the image when attached (a busy image refuses and nothing changes), attaches it through the UNMAP-capable stack so the volume trims its free space, waits until the host allocation stops shrinking and detaches again; the image stays inactive for its next opener. Cancellation is honored between steps and between polls, and cleanup attempts to detach the exact image after attachment is attempted: the private scratch mount belongs to this cycle, so the cleanup detach is forced when the plain one fails, and the macOS port waits up to the settle polls for a late device after an attach that failed (its child may still be attaching) before it reports none. A port that mounts canonically gives an image that was attached back attached at its mount point on every exit path (a failed or cancelled attach is retried by the cleanup, which no cancellation reaches), so the links into it never dangle; an image that still stays attached at a foreign mount point (a killed process runs no cleanup) is recovered by `repo doctor --fix --only=storage` and by the next opening (see `nativeImageStrandedMountV1`). The caller holds the exclusive image lease. */
export async function reclaimNativeImageV1(port:NativeImageReclaimPortV1,hooks:NativeImageReclaimHooksV1):Promise<NativeImageReclaimOutcomeV1>{
 const total=protocol.progressSteps,sleep=hooks.sleep??pause,progress=hooks.progress??(()=>{});
 hooks.check();progress(0,total);
 const beforeBytes=port.allocatedBytes(),prior=await port.attachedDevice();
 if(prior!==null){await port.detach(prior);invalidateNativeImageRosterV1();}
 let settled=false;
 const failures:unknown[]=[];
 try{
  progress(1,total);hooks.check();
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
  if(port.mountsCanonically===true&&prior!==null){if(device===null)await port.attach("cleanup");}
  else if(device!==null)await port.detach(device,"cleanup");
  port.dispose();
 }catch(error){failures.push(error);}finally{invalidateNativeImageRosterV1();}
 if(failures.length>1)throw new AggregateError(failures,"Native image reclaim and cleanup failed",{cause:failures[0]});
 if(failures.length===1)throw failures[0];
 const afterBytes=port.allocatedBytes();progress(total,total);
 return Object.freeze({beforeBytes,afterBytes,settled});
}

/** 🚦️ Supplies cancellation, progress and the two reserve steps to a refresh cycle: `before` runs while the image is still attached and gives the volume room to flush, `after` runs once it is attached again. */
export type NativeImageRefreshHooksV1=NativeImageReclaimHooksV1&Readonly<{before?:()=>void;after?:()=>void}>;
/** 📊️ Reports what the volume reported as available before and after one refresh cycle; `refreshed` is true when the volume was attached again by the cycle. */
export type NativeImageRefreshOutcomeV1=Readonly<{beforeAvailableBytes:number|undefined;afterAvailableBytes:number|undefined;refreshed:boolean}>;
/** 🔄️ Makes a mounted volume sample its available space again without touching the bytes, the size or the identity of its image: it detaches the attached image (a busy refusal escalates only on a port built `quiescent`, whose caller holds the exclusive image lease) and attaches it at its own mount point, so every exit path gives the image back attached and the links into it never dangle; an image that was not attached has no stale figure and is left alone. Cancellation is honored between the steps and the cleanup attaches again when the cycle left it detached. The port must mount canonically. The caller holds the exclusive image lease.
 * @see ../⚖️availability/🟦️.ts */
export async function refreshNativeImageV1(port:NativeImageReclaimPortV1,hooks:NativeImageRefreshHooksV1):Promise<NativeImageRefreshOutcomeV1>{
 if(port.mountsCanonically!==true)throw Error("Canonical image mount required for a refresh");
 const progress=hooks.progress??(()=>{}),total=3;
 hooks.check();progress(0,total);
 const beforeAvailableBytes=port.volumeAvailableBytes?.(),prior=await port.attachedDevice();
 if(prior===null){progress(total,total);return Object.freeze({beforeAvailableBytes,afterAvailableBytes:beforeAvailableBytes,refreshed:false});}
 const failures:unknown[]=[];
 try{hooks.before?.();}catch(error){failures.push(error);}
 let refreshed=false;
 try{
  if(failures.length===0){
   hooks.check();progress(1,total);await port.detach(prior);invalidateNativeImageRosterV1();
   hooks.check();progress(2,total);await port.attach();invalidateNativeImageRosterV1();refreshed=true;
  }
 }catch(error){failures.push(error);}
 try{
  const device=await port.attachedDevice("cleanup");
  if(device===null)await port.attach("cleanup");
  port.dispose();
 }catch(error){failures.push(error);}finally{invalidateNativeImageRosterV1();}
 try{hooks.after?.();}catch(error){failures.push(error);}
 if(failures.length>1)throw new AggregateError(failures,"Native image refresh and cleanup failed",{cause:failures[0]});
 if(failures.length===1)throw failures[0];
 progress(total,total);
 return Object.freeze({beforeAvailableBytes,afterAvailableBytes:port.volumeAvailableBytes?.(),refreshed});
}

/** 🧹️ Removes only the empty private mount directory after the image is detached. */
function removeReclaimScratchV1(scratch:string):void{
 try{rmdirSync(scratch);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
}
/** 🍎️ Builds the macOS port of one image: attach through `diskutil image attach` at a private scratch mount point, detach the whole device through `hdiutil detach`, allocation from the blocks of the backing file. A `quiescent` port belongs to a caller that holds the exclusive image lease, so a busy refusal escalates without waiting for a latched volume: `hdiutil detach -force`, then `diskutil unmount force` of the mount point and `hdiutil detach -force` again. See https://ss64.com/mac/diskutil.html */
export function createNativeImageDarwinReclaimPortV1(given:Readonly<{image:string;cwd:string;mountPoint?:string;run:CommandRunnerV1;cleanupRun?:CommandRunnerV1;latched?:(mountPoint:string)=>boolean;quiescent?:boolean;sleep?:(milliseconds:number)=>Promise<void>;signal?:AbortSignal;commandMilliseconds?:number}>):NativeImageReclaimPortV1{
 const bound=(run:CommandRunnerV1,signal=given.signal)=>nativeImageBoundedRunnerV1(run,{milliseconds:given.commandMilliseconds,...signal===undefined?{}:{signal}}),options={...given,run:bound(given.run),...given.cleanupRun===undefined?{}:{cleanupRun:bound(given.cleanupRun,undefined)}};
 let scratch:string|undefined,mounted:string|undefined,failedAttach=false;
 const latched=options.latched??((mountPoint:string)=>{try{const volume=statfsSync(mountPoint),host=statfsSync(dirname(options.image));return nativeImageVolumeLatchedV1(Number(volume.bavail)*volume.bsize,nativeImageContainerFreeBytesSyncV1(mountPoint),Number(host.bavail)*host.bsize);}catch{return false;}});
 const runner=(phase?:"cleanup")=>phase===undefined?options.run:options.cleanupRun??options.run;
 const find=async(phase?:"cleanup")=>{
   const information=nativeImageInformationV1(await runner(phase)(nativeImageHdiutilProgramV1,["info","-plist"])),rows=Array.isArray(information.images)?information.images as Record<string,unknown>[]:[];
   const row=rows.find(candidate=>candidate["image-path"]===options.image),entities=row?.["system-entities"];
   const device=Array.isArray(entities)&&entities.length?(entities[0] as Record<string,unknown>)["dev-entry"]:undefined,point=Array.isArray(entities)?entities.map(entity=>(entity as Record<string,unknown>)["mount-point"]).find(value=>typeof value==="string"):undefined;
   mounted=typeof point==="string"?point:undefined;
   return typeof device==="string"?device:null;
 };
 return Object.freeze({
  attachedDevice:async(phase?:"cleanup")=>{
   const found=await find(phase);
   if(found!==null||phase!=="cleanup"||!failedAttach)return found;
   failedAttach=false;
   for(let poll=0;poll<protocol.settleStablePolls;poll++){
    await(options.sleep??pause)(protocol.settlePollMilliseconds);
    const late=await find(phase);if(late!==null)return late;
   }
   return null;
  },
  async detach(device:string,phase?:"cleanup"){
   try{await runner(phase)(nativeImageHdiutilProgramV1,["detach",device]);}
   catch(error){
    const busy=/busy/iu.test(error instanceof Error?error.message:String(error));
    if(phase!=="cleanup"&&(!busy||mounted===undefined||options.quiescent!==true&&!latched(mounted)))throw error;
    const point=mounted;
    try{await runner(phase)(nativeImageHdiutilProgramV1,[...nativeImageForceDetachArgumentsV1,device]);}
    catch(forced){
     const unmount=options.quiescent===true?nativeImageRefreshMechanismV1("darwin"):null;
     if(unmount===null||point===undefined)throw forced;
     await runner(phase)(unmount.forceUnmountProgram,[...unmount.forceUnmountArguments,point]);
     try{await runner(phase)(nativeImageHdiutilProgramV1,[...nativeImageForceDetachArgumentsV1,device]);}
     catch(last){if(await find(phase)!==null)throw last;}
    }
   }
   guardDetachedNativeImageMountV1(options.mountPoint);
  },
  async attach(phase?:"cleanup"){const target=options.mountPoint??(scratch=mkdtempSync(join(tmpdir(),protocol.scratchPrefix)));try{await attachNativeImageV1(runner(phase),target,options.image);}catch(error){failedAttach=true;throw error;}},
  allocatedBytes(){return lstatSync(options.image).blocks*512;},
  volumeAvailableBytes(){const point=options.mountPoint??mounted;if(point===undefined)return undefined;try{const row=statfsSync(point);return Number(row.bavail)*row.bsize;}catch{return undefined;}},
  dispose(){if(scratch!==undefined){removeReclaimScratchV1(scratch);scratch=undefined;}},
  mountsCanonically:options.mountPoint!==undefined
 });
}

/** 🛡️ Makes the plain directory a detached image leaves behind at its mount point read-only (its mode without the write bits, 0555 for the usual 0755): a writer that resolves a path below it fails loudly with EACCES instead of silently filling the host disk, the next attach mounts over it as before, and the mount contract admits exactly this guarded mode as the same underlying directory, so a guarded image reopens after a detach or a reboot. A mounted or absent mount point is left alone.
 * @see ../../🌐️platform/📋️contract/📂️mount/🟦️.ts */
export function guardDetachedNativeImageMountV1(mountPoint:string|undefined):void{
 if(mountPoint===undefined)return;
 try{const row=lstatSync(mountPoint);if(row.isDirectory()&&!row.isSymbolicLink()&&row.dev===lstatSync(dirname(mountPoint)).dev)chmodSync(mountPoint,nativeImageGuardedMountModeV1(row.mode)&0o7777);}catch{}
}
const fill=(template:readonly string[],values:Readonly<Record<string,string>>):string[]=>template.map(part=>part.replace(/<([A-Za-z]+)>/gu,(_,key)=>values[key]??_));
type CommandRunnerV1=(program:string,args:readonly string[])=>Promise<string>;
/** ⏱️ A command of the image cycle that ended by its deadline or by cancellation, bilingual, naming the program and the seconds. */
export class NativeImageCommandInterruptedV1 extends Error{
 readonly program:string;readonly reason:"deadline"|"cancelled";
 constructor(program:string,args:readonly string[],reason:"deadline"|"cancelled",milliseconds:number){
  const text=[program,...args].join(" ").slice(0,200),seconds=Math.round(milliseconds/1000);
  super(reason==="deadline"?`Image command ${text} exceeded its deadline of ${seconds} s / Image-Befehl ${text} überschritt seine Frist von ${seconds} s`:`Image command ${text} was cancelled / Image-Befehl ${text} wurde abgebrochen`);
  this.name="NativeImageCommandInterruptedV1";this.program=program;this.reason=reason;
 }
}
/** ⏱️ Gives every command of the cycle (`losetup`, `mount`, `fstrim`, `umount`, `findmnt`, `hdiutil`, PowerShell) the deadline `commandMilliseconds` of the protocol and cancellation by `signal`: a command that does not end in time or is cancelled rejects with {@link NativeImageCommandInterruptedV1} and its result is dropped. The timer is injectable so the rule runs under a fake clock.
 * @see ./🧪️tests/🟦️.ts */
export function nativeImageBoundedRunnerV1(run:CommandRunnerV1,options:Readonly<{milliseconds?:number;signal?:AbortSignal;setTimer?:(run:()=>void,milliseconds:number)=>unknown;clearTimer?:(timer:unknown)=>void}>={}):CommandRunnerV1{
 const milliseconds=options.milliseconds??protocol.commandMilliseconds,setTimer=options.setTimer??((work,delay)=>setTimeout(work,delay)),clearTimer=options.clearTimer??(timer=>clearTimeout(timer as ReturnType<typeof setTimeout>));
 return(program,args)=>new Promise<string>((resolve,reject)=>{
  if(options.signal?.aborted){reject(new NativeImageCommandInterruptedV1(program,args,"cancelled",0));return;}
  let finished=false;
  const finish=(settle:()=>void)=>{if(finished)return;finished=true;clearTimer(timer);options.signal?.removeEventListener("abort",abort);settle();};
  const abort=()=>finish(()=>reject(new NativeImageCommandInterruptedV1(program,args,"cancelled",0)));
  const timer=setTimer(()=>finish(()=>reject(new NativeImageCommandInterruptedV1(program,args,"deadline",milliseconds))),milliseconds);
  options.signal?.addEventListener("abort",abort,{once:true});
  run(program,args).then(value=>finish(()=>resolve(value)),error=>finish(()=>reject(error)));
 });
}
/** 🐧️ Builds the Linux port of one loop-backed image file: `losetup` finds and attaches the loop device, `mount -o discard` mounts it at a private scratch mount point, `fstrim` returns the free blocks of the volume and the loop layer punches holes into the sparse file (FALLOC_FL_PUNCH_HOLE); detach unmounts every mount of the device (a busy one refuses) and releases the loop device; allocation from the blocks of the backing file. See https://man7.org/linux/man-pages/man8/fstrim.8.html */
export function createNativeImageLinuxReclaimPortV1(given:Readonly<{image:string;mountPoint?:string;run:CommandRunnerV1;allocated?:(image:string)=>number;signal?:AbortSignal;commandMilliseconds?:number}>):NativeImageReclaimPortV1{
 const options={...given,run:nativeImageBoundedRunnerV1(given.run,{milliseconds:given.commandMilliseconds,...given.signal===undefined?{}:{signal:given.signal}})},linux=protocol.linux;let scratch:string|undefined,device:string|undefined;
 const first=(text:string)=>text.split(/\r?\n/u).map(row=>row.trim()).find(row=>row!=="")??null;
 return Object.freeze({
  async attachedDevice(){return first(await options.run(linux.losetup,fill(linux.associatedArguments,{image:options.image})));},
  async detach(target:string){
   const mounts=(await options.run(linux.findmnt,fill(linux.mountsArguments,{device:target}))).split(/\r?\n/u).map(row=>row.trim()).filter(row=>row!=="");
   for(const mountPoint of mounts.reverse())await options.run(linux.umount,fill(linux.unmountArguments,{mountPoint}));
   await options.run(linux.losetup,fill(linux.detachArguments,{device:target}));
  },
  async attach(){
   scratch=options.mountPoint??mkdtempSync(join(tmpdir(),protocol.scratchPrefix));
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
  dispose(){if(scratch!==undefined){if(options.mountPoint===undefined)removeReclaimScratchV1(scratch);scratch=undefined;}device=undefined;},
  mountsCanonically:options.mountPoint!==undefined
 });
}
/** 🪟️ Builds the Windows port of one dynamic VHDX: PowerShell reports whether the file is attached, `Mount-VHD -ReadOnly -NoDriveLetter` attaches it without a drive letter, `Optimize-VHD -Mode Full` compacts it from the unused blocks of the volume and `Dismount-VHD` detaches it; the host allocation of a dynamic VHDX is its file size. See https://learn.microsoft.com/powershell/module/hyper-v/optimize-vhd */
export function createNativeImageWindowsReclaimPortV1(given:Readonly<{image:string;run:CommandRunnerV1;allocated?:(image:string)=>number;signal?:AbortSignal;commandMilliseconds?:number}>):NativeImageReclaimPortV1{
 const options={...given,run:nativeImageBoundedRunnerV1(given.run,{milliseconds:given.commandMilliseconds,...given.signal===undefined?{}:{signal:given.signal}})},windows=protocol.windows,script=(text:string)=>options.run(windows.program,[...windows.arguments,fill([text],{image:options.image.replaceAll("'","''")})[0]!]);
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
export function createNativeImageReclaimPortV1(platform:string,options:Readonly<{image:string;cwd:string;mountPoint?:string;run:CommandRunnerV1;cleanupRun?:CommandRunnerV1;quiescent?:boolean;signal?:AbortSignal;commandMilliseconds?:number}>):NativeImageReclaimPortV1|null{
 if(platform==="darwin")return createNativeImageDarwinReclaimPortV1(options);
 if(platform==="linux")return createNativeImageLinuxReclaimPortV1(options);
 if(platform==="windows"||platform==="win32")return createNativeImageWindowsReclaimPortV1(options);
 return null;
}
