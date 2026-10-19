import{win32}from"node:path";
import{createRequire}from"node:module";
import{nativeDirectoryProtocolV1}from"../📋️contract/🟦️.ts";
const protocol=nativeDirectoryProtocolV1(),windows=protocol.exchange.windows;

/** 🧵️ The exact Win32 and NT calls the directory partition needs, as one replaceable call table (the FFI call-sequence port). */
export type NativeWindowsBindingV1=Readonly<{
 moveFileEx(from:Buffer,to:Buffer,flags:number):number;
 createFile(path:Buffer,access:number,share:number,disposition:number,flags:number):bigint;
 flushFileBuffers(handle:bigint):number;
 closeHandle(handle:bigint):number;
 setCaseInformation(handle:bigint,status:Buffer,information:Buffer,bytes:number,informationClass:number):number;
 getDiskFreeSpaceEx(path:Buffer,available:Buffer):number;
 lastError():number;
 close():void;
}>;
/** 🪟️ Reports one Win32 result without throwing: success flag and the thread's last error. */
export type NativeWindowsResultV1=Readonly<{ok:boolean;error:number}>;
/** 🪟️ The semantic host operations of a Windows directory partition. */
export type NativeWindowsHostPortV1=Readonly<{
 moveFile(from:string,to:string):NativeWindowsResultV1;
 flushDirectory(path:string):NativeWindowsResultV1;
 setCaseSensitive(path:string):Readonly<{opened:boolean;error:number;status:number}>;
 availableBytes(path:string):Readonly<{ok:boolean;error:number;bytes:number}>;
 openExclusive(path:string):NativeWindowsResultV1;
}>;

/** 📏️ Projects one canonical absolute Windows path into the `\\?\` namespace so no call is bound by MAX_PATH. */
export function nativeWindowsExtendedPathV1(path:string):string{
 if(typeof path!=="string"||!path||/[\u0000-\u001f"<>|*?]/u.test(path.replace(/^\\\\\?\\/u,""))||path.includes("/"))throw Error("Canonical absolute Windows path required");
 if(path.startsWith(windows.extendedPrefix)){if(!/^\\\\\?\\(?:[A-Za-z]:\\|UNC\\[^\\]+\\[^\\]+)/u.test(path)||win32.normalize(path)!==path)throw Error("Canonical extended Windows path required");return path;}
 const drive=/^[A-Za-z]:\\/u.test(path),share=/^\\\\[^\\?.]+\\[^\\]+/u.test(path);
 if(!drive&&!share||win32.normalize(path)!==path||win32.resolve(path)!==path||path.split("\\").some(part=>part==="."||part===".."))throw Error("Canonical absolute Windows path required");
 const result=drive?windows.extendedPrefix+path:windows.extendedUncPrefix+path.slice(2);if(result.length>windows.maximumExtendedUnits)throw Error("Windows path exceeds the extended-length limit");return result;
}
/** 🔤️ Encodes one string as the zero-terminated UTF-16LE buffer every wide Win32 entry point takes. */
export const nativeWindowsUtf16V1=(text:string):Buffer=>Buffer.from(text+"\0","utf16le");

/** 🧩️ Builds the semantic host port over a call table; no handle or error number ever leaves this function. */
export function createNativeWindowsHostPortV1(binding:NativeWindowsBindingV1):NativeWindowsHostPortV1{
 const invalid=BigInt(windows.invalidHandle),open=(path:string,access:number,flags:number,share:number=windows.createFile.shareAll)=>{const handle=binding.createFile(nativeWindowsUtf16V1(nativeWindowsExtendedPathV1(path)),access,share,windows.createFile.openExisting,flags);return handle===0n||handle===invalid?undefined:handle;};
 const finish=(handle:bigint,ok:boolean,error:number):NativeWindowsResultV1=>{const closed=binding.closeHandle(handle)===1;return Object.freeze(closed?{ok,error}:ok?{ok:false,error:binding.lastError()}:{ok,error});};
 return Object.freeze({
  moveFile:(from:string,to:string)=>binding.moveFileEx(nativeWindowsUtf16V1(nativeWindowsExtendedPathV1(from)),nativeWindowsUtf16V1(nativeWindowsExtendedPathV1(to)),windows.moveWriteThrough)===1?Object.freeze({ok:true,error:0}):Object.freeze({ok:false,error:binding.lastError()}),
  flushDirectory:(path:string)=>{const handle=open(path,(windows.createFile.genericRead|windows.createFile.genericWrite)>>>0,windows.createFile.backupSemantics|windows.createFile.openReparsePoint);if(handle===undefined)return Object.freeze({ok:false,error:binding.lastError()});const ok=binding.flushFileBuffers(handle)===1;return finish(handle,ok,ok?0:binding.lastError());},
  openExclusive:(path:string)=>{const handle=open(path,windows.createFile.fileReadAttributes,windows.createFile.backupSemantics,0);if(handle===undefined)return Object.freeze({ok:false,error:binding.lastError()});return finish(handle,true,0);},
  availableBytes:(path:string)=>{const available=Buffer.alloc(8);return binding.getDiskFreeSpaceEx(nativeWindowsUtf16V1(nativeWindowsExtendedPathV1(path)),available)===1?Object.freeze({ok:true,error:0,bytes:Number(available.readBigUInt64LE(0))}):Object.freeze({ok:false,error:binding.lastError(),bytes:0});},
  setCaseSensitive:(path:string)=>{const handle=open(path,windows.createFile.fileWriteAttributes,windows.createFile.backupSemantics);if(handle===undefined)return Object.freeze({opened:false,error:binding.lastError(),status:0});const information=Buffer.alloc(windows.caseSensitive.informationBytes);information.writeUInt32LE(windows.caseSensitive.flag,0);const status=binding.setCaseInformation(handle,Buffer.alloc(windows.caseSensitive.statusBlockBytes),information,information.length,windows.caseSensitive.informationClass)>>>0;binding.closeHandle(handle);return Object.freeze({opened:true,error:0,status});}
 });
}

/** 🧲️ Maps one NTSTATUS from the case-sensitivity call to its precise refusal, or returns on success. */
export function requireNativeWindowsCaseStatusV1(result:Readonly<{opened:boolean;error:number;status:number}>):void{
 const refusals=protocol.refusals,statuses=protocol.windowsStatuses;
 if(!result.opened)throw Error(result.error===protocol.windowsErrors.accessDenied?refusals.caseAccess:"Per-directory case sensitivity could not open the directory (Win32 error "+result.error+")");
 if(result.status===0)return;
 if(result.status===statuses.directoryNotEmpty)throw Error(refusals.caseNotEmpty);
 if(result.status===statuses.notSupported)throw Error(refusals.caseFeature);
 if(result.status===statuses.invalidInfoClass||result.status===statuses.invalidParameter)throw Error(refusals.caseBuild);
 if(result.status===statuses.accessDenied)throw Error(refusals.caseAccess);
 throw Error("Per-directory case sensitivity failed (NTSTATUS 0x"+result.status.toString(16).padStart(8,"0")+")");
}

/** 🔓️ Loads kernel32 and ntdll through bun:ffi; anything but Windows x64 or arm64 under Bun is a precise refusal. */
export function openNativeWindowsBindingV1():NativeWindowsBindingV1{
 if(process.platform!=="win32"||!process.versions.bun)throw Error("Actual Windows Bun receiver required");
 const{dlopen,FFIType:f}=createRequire(import.meta.url)("bun:ffi")as typeof import("bun:ffi"),kernel=dlopen(windows.libraries.kernel,{MoveFileExW:{args:[f.ptr,f.ptr,f.u32],returns:f.i32},CreateFileW:{args:[f.ptr,f.u32,f.u32,f.ptr,f.u32,f.u32,f.u64],returns:f.u64},FlushFileBuffers:{args:[f.u64],returns:f.i32},CloseHandle:{args:[f.u64],returns:f.i32},GetLastError:{args:[],returns:f.u32},GetDiskFreeSpaceExW:{args:[f.ptr,f.ptr,f.ptr,f.ptr],returns:f.i32}}),nt=(()=>{try{return dlopen(windows.libraries.nt,{NtSetInformationFile:{args:[f.u64,f.ptr,f.ptr,f.u32,f.u32],returns:f.i32}});}catch(error){kernel.close();throw error;}})();
 const k=kernel.symbols as unknown as Record<string,(...args:unknown[])=>unknown>,n=nt.symbols as unknown as Record<string,(...args:unknown[])=>unknown>;
 return Object.freeze({
  moveFileEx:(from,to,flags)=>Number(k.MoveFileExW!(from,to,flags)),
  createFile:(path,access,share,disposition,flags)=>BigInt(k.CreateFileW!(path,access,share,null,disposition,flags,0n)as bigint),
  flushFileBuffers:handle=>Number(k.FlushFileBuffers!(handle)),
  closeHandle:handle=>Number(k.CloseHandle!(handle)),
  setCaseInformation:(handle,status,information,bytes,informationClass)=>Number(n.NtSetInformationFile!(handle,status,information,bytes,informationClass)),
  getDiskFreeSpaceEx:(path,available)=>Number(k.GetDiskFreeSpaceExW!(path,available,null,null)),
  lastError:()=>Number(k.GetLastError!()),
  close:()=>{let failure:unknown,failed=false;for(const library of[kernel,nt])try{library.close();}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;}
 });
}
let systemHost:NativeWindowsHostPortV1|undefined;
/** 🪟️ The process-wide host port over the real kernel; loaded once and kept for the life of the process like libc. */
export function nativeWindowsSystemHostPortV1():NativeWindowsHostPortV1{return systemHost??=createNativeWindowsHostPortV1(openNativeWindowsBindingV1());}

/** 📏️ Reads machine-wide LongPathsEnabled through RegGetValueW; an absent value means disabled. */
export function readNativeWindowsLongPathsEnabledV1(call?:(hive:bigint,key:Buffer,value:Buffer,flags:number,data:Buffer,size:Buffer)=>number):boolean{
 const policy=windows.longPaths;let library:{close():void}|undefined;
 if(!call){if(process.platform!=="win32"||!process.versions.bun)throw Error("Actual Windows Bun registry receiver required");const{dlopen,FFIType:f}=createRequire(import.meta.url)("bun:ffi")as typeof import("bun:ffi"),opened=dlopen(windows.libraries.registry,{RegGetValueW:{args:[f.i64,f.ptr,f.ptr,f.u32,f.ptr,f.ptr,f.ptr],returns:f.i32}});library=opened;call=(hive,key,value,flags,data,size)=>Number((opened.symbols as unknown as Record<string,(...args:unknown[])=>unknown>).RegGetValueW!(hive,key,value,flags,null,data,size));}
 let result:boolean|undefined,failed=false,failure:unknown;
 try{const data=Buffer.alloc(policy.dataBytes),size=Buffer.alloc(4);size.writeUInt32LE(policy.dataBytes,0);const status=call(BigInt(policy.hive),nativeWindowsUtf16V1(policy.key),nativeWindowsUtf16V1(policy.value),policy.flags,data,size);if(status===policy.notFound)result=false;else if(status!==0||size.readUInt32LE(0)!==policy.dataBytes)throw Error("Windows LongPathsEnabled registry read failed: "+status);else result=data.readUInt32LE(0)===1;}catch(error){failed=true;failure=error;}
 try{library?.close();}catch(error){if(!failed){failed=true;failure=error;}}
 if(failed)throw failure;return result!;
}
