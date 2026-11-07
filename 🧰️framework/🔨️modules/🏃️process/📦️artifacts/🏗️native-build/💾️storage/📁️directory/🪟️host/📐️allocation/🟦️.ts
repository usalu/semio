import{runtimeSupportsV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import{runtimeNativeSystemV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";
import{nativeWindowsExtendedPathV1,nativeWindowsUtf16V1}from"../🟦️.ts";
import{nativeDirectoryProtocolV1}from"../../📋️contract/🟦️.ts";
import schema from"./🧬️schema/🔣️.json" with{type:"json"};

const policy=schema.definitions.Policy.const;
/** 📐️ Validates the bounded allocation unit before it is used for physical accounting. */
export function nativeWindowsAllocationUnitFromV1(sectors:number,sectorBytes:number):number|null{const power=(value:number)=>Number.isSafeInteger(value)&&value>0&&Number.isInteger(Math.log2(value));if(!power(sectors)||!power(sectorBytes)||sectors>policy.maximumSectorsPerCluster||sectorBytes>policy.maximumSectorBytes)return null;const bytes=sectors*sectorBytes;return Number.isSafeInteger(bytes)&&bytes<=policy.maximumAllocationBytes?bytes:null;}
/** 💽️ Reads the actual volume allocation unit through the Windows kernel without changing the volume. */
export function nativeWindowsAllocationUnitV1(path:string):Readonly<{version:1;volume:string;sectorsPerCluster:number;bytesPerSector:number;allocationBytes:number}>{
 if(process.platform!=="win32"||!runtimeSupportsV1("ffi"))throw Error("Actual Windows Bun allocation receiver required");
 const input=nativeWindowsUtf16V1(nativeWindowsExtendedPathV1(path)),volume=Buffer.alloc((policy.maximumVolumeUnits+1)*2),sectors=Buffer.alloc(4),sectorBytes=Buffer.alloc(4),free=Buffer.alloc(4),total=Buffer.alloc(4),{dlopen,FFIType:f}=runtimeNativeSystemV1(),library=dlopen(nativeDirectoryProtocolV1().exchange.windows.libraries.kernel,{GetVolumePathNameW:{args:[f.ptr,f.ptr,f.u32],returns:f.i32},GetDiskFreeSpaceW:{args:[f.ptr,f.ptr,f.ptr,f.ptr,f.ptr],returns:f.i32},GetLastError:{args:[],returns:f.u32}});
 let failed=false,failure:unknown,result:Readonly<{version:1;volume:string;sectorsPerCluster:number;bytesPerSector:number;allocationBytes:number}>|undefined;
 try{const calls=library.symbols as unknown as Record<string,(...args:unknown[])=>unknown>;if(Number(calls.GetVolumePathNameW!(input,volume,policy.maximumVolumeUnits+1))!==1)throw Error("Windows volume path query failed: "+Number(calls.GetLastError!()));let end=0;while(end<volume.length&&volume.readUInt16LE(end)!==0)end+=2;if(end===0||end>=volume.length)throw Error("Bounded Windows volume path required");const name=volume.subarray(0,end).toString("utf16le");if(Number(calls.GetDiskFreeSpaceW!(volume,sectors,sectorBytes,free,total))!==1)throw Error("Windows allocation unit query failed: "+Number(calls.GetLastError!()));const sectorsPerCluster=sectors.readUInt32LE(0),bytesPerSector=sectorBytes.readUInt32LE(0),allocationBytes=nativeWindowsAllocationUnitFromV1(sectorsPerCluster,bytesPerSector);if(allocationBytes===null)throw Error("Bounded Windows allocation unit required");result=Object.freeze({version:1,volume:name,sectorsPerCluster,bytesPerSector,allocationBytes});}catch(error){failed=true;failure=error;}try{library.close();}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;return result!;
}
