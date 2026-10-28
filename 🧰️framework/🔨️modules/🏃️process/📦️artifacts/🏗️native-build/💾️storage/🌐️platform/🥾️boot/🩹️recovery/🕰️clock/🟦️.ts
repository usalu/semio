import{createHash}from"node:crypto";
import{readFileSync}from"node:fs";
import{captureOwnedCommandV1}from"../../../../../../../🎛️owned-execution/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../🔒️leases/🟦️.ts";
import{nativeStorageWindowsProgramPathV1,pinNativeStorageProgramV1}from"../../../../🧰️programs/🟦️.ts";
import{readNativeBootSessionV1}from"../../🟦️.ts";
import{parseNativeBootEpochV1}from"../📋️plan/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with{type:"json"};
import type{NativeBootClockV1}from"../📡️observe/🟦️.ts";
import{readWindowsBootEpochNativeV1}from"./🪟️native/🟦️.ts";
const epoch=schema.definitions.Protocol.const.bootEpoch;
let primed:NativeBootClockV1|undefined,priming:Promise<NativeBootClockV1>|undefined;

/** 🐧️ Milliseconds between the host boot and the start of process 1 of this pid namespace, read from the start tick of `/proc/1/stat`: zero on a machine, the age of the container at its start inside one, so records of a restarted container count as records of an earlier boot even though the host never rebooted. */
export function linuxNamespaceOffsetMsV1(initStat:string):number{
 const close=initStat.lastIndexOf(")"),ticks=Number(initStat.slice(close+2).trim().split(/\s+/u)[19]);
 if(close<0||!Number.isSafeInteger(ticks)||ticks<0)throw Error("Exact init start tick required");
 return Math.round(ticks*1000/epoch.linux.clockTicksPerSecond);
}

/** 🔌️ The host reads of the boot epoch: the in-process Windows call, a command (its text), a file, and the pinned PowerShell path; a test replaces them to run every host's route anywhere. */
export type NativeBootEpochPortsV1=Readonly<{native:()=>number|null;command:(program:string,args:readonly string[])=>Promise<string>;file:(path:string)=>string;powershell:()=>string}>;

/** 🕰️ Reads the epoch of the host boot in milliseconds: macOS from `sysctl kern.boottime`, Linux from `/proc/stat`, Windows from the in-process system call and only when that is unavailable from a PowerShell process (the same instant, parsed from the same documented text). */
export async function readHostBootEpochMsV1(platform:"darwin"|"linux"|"win32",ports:NativeBootEpochPortsV1):Promise<number>{
 if(platform==="win32"){const direct=ports.native();if(direct!==null)return direct;}
 const text=platform==="darwin"?await ports.command(epoch.darwin.program,epoch.darwin.arguments):platform==="win32"?await ports.command(ports.powershell(),epoch.win32.arguments):ports.file(epoch.linux.file),hostEpochMs=parseNativeBootEpochV1(platform,text);
 if(hostEpochMs===null)throw Error("Exact boot epoch text required");return hostEpochMs;
}

/** 🕰️ Reads which boot is running and when it began; both are memoized for the process because neither can change inside it. */
export function readNativeBootClockV1(control:ResourceLeaseControl,cwd:string):Promise<NativeBootClockV1>{
 if(primed)return Promise.resolve(primed);
 return priming??=(async()=>{
  try{
   const platform=process.platform;if(platform!=="darwin"&&platform!=="linux"&&platform!=="win32")throw Error("Boot clock unavailable on this platform");
   const hostEpochMs=await readHostBootEpochMsV1(platform,{native:()=>readWindowsBootEpochNativeV1(),command:(program,args)=>captureOwnedCommandV1(program,args,cwd,{control,output:{maximumBytes:epoch.maximumBytes,maximumLines:4}}),file:path=>readFileSync(path,"utf8"),powershell:()=>pinNativeStorageProgramV1(nativeStorageWindowsProgramPathV1("powershell",process.env))});
   const bootEpochMs=platform==="linux"?hostEpochMs+linuxNamespaceOffsetMsV1(readFileSync(epoch.linux.namespaceFile,"utf8")):hostEpochMs;
   const hex=createHash("sha256").update("boot:"+bootEpochMs).digest("hex").slice(0,32).toUpperCase(),bootSession=platform==="win32"?`${hex.slice(0,8)}-${hex.slice(8,12)}-${hex.slice(12,16)}-${hex.slice(16,20)}-${hex.slice(20,32)}`:await readNativeBootSessionV1(control,cwd);
   return primed=Object.freeze({bootSession,bootEpochMs});
  }finally{priming=undefined;}
 })();
}
