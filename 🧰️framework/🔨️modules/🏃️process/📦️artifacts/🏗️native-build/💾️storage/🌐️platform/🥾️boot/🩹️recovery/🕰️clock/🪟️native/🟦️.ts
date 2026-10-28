import {runtimeSupportsV1} from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import {runtimeNativeSystemV1} from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

const native=schema.definitions.Protocol.const.bootEpoch.win32.native;

/** 🪟️ Turns the `BootTime` of a `SYSTEM_TIMEOFDAY_INFORMATION` record (a FILETIME: 100 ns ticks since 1601-01-01 UTC, little endian) into Unix epoch milliseconds, flooring the sub-millisecond ticks exactly like the textual path that parses `LastBootUpTime`; null for a time that cannot be a boot time. See https://learn.microsoft.com/windows/win32/api/minwinbase/ns-minwinbase-filetime */
export function windowsBootEpochMsFromTimeOfDayV1(record:Uint8Array):number|null{
 if(record.byteLength<native.bootTimeOffset+8)return null;
 const ticks=new DataView(record.buffer,record.byteOffset,record.byteLength).getBigInt64(native.bootTimeOffset,true),epoch=ticks/10000n-BigInt(native.fileTimeEpochOffsetMs);
 return ticks>0n&&epoch>0n&&epoch<BigInt(Number.MAX_SAFE_INTEGER)?Number(epoch):null;
}

/** 🕰️ Reads the boot time of the host in one in-process system call (`NtQuerySystemInformation(SystemTimeOfDayInformation)`), the value the management service reports as the last boot-up time, instead of starting a PowerShell for it (a process start plus a CIM query, hundreds of milliseconds on every `repo` process). Null when the host is not Windows, the runtime has no foreign function interface, the library or the call is refused (the caller then asks the shell); a record shorter than the current layout is retried with the older length. See https://learn.microsoft.com/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation */
export function readWindowsBootEpochNativeV1(system?:Pick<ReturnType<typeof runtimeNativeSystemV1>,"dlopen"|"ptr">,platform:string=process.platform):number|null{
 if(platform!=="win32"||system===undefined&&!runtimeSupportsV1("ffi"))return null;
 let library:ReturnType<ReturnType<typeof runtimeNativeSystemV1>["dlopen"]>|undefined;
 try{
  const host=system??runtimeNativeSystemV1();library=host.dlopen(native.library,{[native.symbol]:{args:["u32","ptr","u32","ptr"],returns:"i32"}});
  for(const length of native.lengths){
   const record=Buffer.alloc(length),returned=Buffer.alloc(4),status=library.symbols[native.symbol]!(native.class,host.ptr(record),length,host.ptr(returned)) as number;
   if(status>=0)return windowsBootEpochMsFromTimeOfDayV1(record);
  }
  return null;
 }catch{return null;}
 finally{try{library?.close();}catch{}}
}
