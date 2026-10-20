import{createRequire}from"node:module";
import{nativeWindowsExtendedPathV1,nativeWindowsUtf16V1}from"../../📁️directory/🪟️host/🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
const policy=schema.$defs.Policy.const,info=policy.information;
/** 🧷️ Names a pinned Windows object without exporting a platform handle. */
export type ConditionalWindowsIdentityV1=Readonly<{volume:string;file:string;directory:boolean;links:number;bytes:number;attributes:number;access:string;write:string;change:string}>;
/** 🪟️ Limits the conditional host to identity, explicit disposition and once-only handle closure. */
export type ConditionalWindowsPinV1=Readonly<{identity:ConditionalWindowsIdentityV1;check():void;remove():void;close():void}>;
/** 🧰️ A firstparty call table keeps system FFI behind a replaceable boundary. */
export type ConditionalWindowsBindingV1=Readonly<{open(path:Buffer,access:number,share:number,flags:number):bigint;information(handle:bigint,kind:number,data:Buffer):boolean;disposition(handle:bigint,kind:number,data:Buffer):boolean;filesystem(handle:bigint,data:Buffer):boolean;fileType(handle:bigint):number;driveType(root:Buffer):number;closeHandle(handle:bigint):boolean;error():number;close():void}>;
/** 🔖️ Preserves exact native refusal codes alongside ordinary Error causes. */
export class ConditionalWindowsRefusalV1 extends Error{constructor(readonly operation:string,readonly nativeCode:number,options?:ErrorOptions){super("Windows conditional retirement "+operation+" refused ("+nativeCode+")",options);}}
const failure=(binding:ConditionalWindowsBindingV1,operation:string)=>new ConditionalWindowsRefusalV1(operation,binding.error());
/** 🧲️ Creates no-census conditional file custody over exact Win32 calls. */
export function createConditionalWindowsPortV1(binding:ConditionalWindowsBindingV1):Readonly<{pin(path:string,candidate:boolean):ConditionalWindowsPinV1;close():void}>{
 const invalid=BigInt(policy.createFile.invalidHandle);let ended=false,live=0;
 const observe=(handle:bigint):ConditionalWindowsIdentityV1=>{
  const id=Buffer.alloc(info.fileIdBytes),standard=Buffer.alloc(info.standardBytes),attributes=Buffer.alloc(info.attributeTagBytes),basic=Buffer.alloc(info.basicBytes),filesystem=Buffer.alloc(32);
  for(const[kind,data]of[[info.fileId,id],[info.standard,standard],[info.attributeTag,attributes],[info.basic,basic]]as const)if(!binding.information(handle,kind,data))throw failure(binding,"identity");
  if(!binding.filesystem(handle,filesystem))throw failure(binding,"filesystem");
  if(filesystem.toString("utf16le").replace(/\0.*$/u,"")!==policy.filesystem)throw Error("Qualified NTFS conditional retirement required");
  if(binding.fileType(handle)!==policy.fileTypeDisk)throw Error("Ordinary Windows disk object required");
  const flags=attributes.readUInt32LE(0),links=standard.readUInt32LE(16),extent=standard.readBigInt64LE(8),directory=standard[21]===1,file=id.subarray(8).toString("hex");
  if(flags&policy.flags.reparse||directory!==Boolean(flags&policy.flags.directory)||!directory&&links!==1||extent<0n||extent>BigInt(Number.MAX_SAFE_INTEGER)||/^0+$/u.test(file))throw Error("Exact single-link no-reparse Windows object required");
  return Object.freeze({volume:id.readBigUInt64LE(0).toString(),file,directory,links,bytes:directory?0:Number(extent),attributes:flags,access:basic.readBigInt64LE(8).toString(),write:basic.readBigInt64LE(16).toString(),change:basic.readBigInt64LE(24).toString()});
 };
 const same=(a:ConditionalWindowsIdentityV1,b:ConditionalWindowsIdentityV1)=>a.volume===b.volume&&a.file===b.file&&a.directory===b.directory&&a.links===b.links&&a.attributes===b.attributes&&(a.directory||a.bytes===b.bytes&&a.write===b.write&&a.change===b.change);
 const closeChecked=(handle:bigint)=>{if(!binding.closeHandle(handle))throw failure(binding,"close");live--;};
 return Object.freeze({pin:(path,candidate)=>{
  if(ended||path.length>policy.maximumPathLength||!/^([A-Za-z]):\\/u.test(path)||binding.driveType(nativeWindowsUtf16V1(path.slice(0,3)))!==policy.driveTypeFixed)throw Error("Bounded live Windows conditional owner required");const wide=nativeWindowsUtf16V1(nativeWindowsExtendedPathV1(path)),flags=policy.createFile.openReparsePoint|policy.createFile.backupSemantics;
  const handle=binding.open(wide,policy.createFile.readAttributes|(candidate?policy.createFile.deleteAccess:0),candidate?0:policy.createFile.ancestorShare,flags);
  if(handle===0n||handle===invalid)throw failure(binding,"open");live++;let retained=false;
  try{const identity=observe(handle);if(candidate&&identity.attributes&policy.flags.readonly)throw Error("Readonly Windows artifacts are not conditionally retired");let closed=false,disposed=false;
   const check=()=>{if(closed||ended)throw Error("Original Windows conditional handle closed");if(!same(identity,observe(handle)))throw Error("Pinned Windows conditional identity changed");const named=binding.open(wide,policy.createFile.readAttributes,7,flags);if(named===0n||named===invalid)throw failure(binding,"named identity");live++;let primary:unknown,failed=false;try{if(!same(identity,observe(named)))throw Error("Named Windows conditional identity changed");}catch(error){failed=true;primary=error;}try{closeChecked(named);}catch(error){if(failed)throw new AggregateError([primary,error],"Windows conditional identity and close failed",{cause:primary});throw error;}if(failed)throw primary;};
   const pin=Object.freeze({identity,check,remove:()=>{if(!candidate||disposed)throw Error("Single accepted Windows conditional disposition required");check();const data=Buffer.alloc(info.dispositionBytes);data.writeUInt32LE(policy.flags.delete|policy.flags.forceImageSectionCheck,0);if(!binding.disposition(handle,info.dispositionEx,data))throw failure(binding,"disposition");disposed=true;},close:()=>{if(closed)return;closeChecked(handle);closed=true;}});
   retained=true;return pin;
  }catch(error){if(!retained)try{closeChecked(handle);}catch(cleanup){throw new AggregateError([error,cleanup],"Windows conditional acquisition and close failed",{cause:error});}throw error;}
 },close:()=>{if(ended)return;if(live!==0)throw Error("Every Windows conditional handle must settle before its library");binding.close();ended=true;}});
}
/** 🧿️ Loads only public kernel32 APIs for Windows x64 or arm64 under the owned Bun runtime. */
export function openConditionalWindowsSystemPortV1():ReturnType<typeof createConditionalWindowsPortV1>{
 if(process.platform!=="win32"||!process.versions.bun||!["x64","arm64"].includes(process.arch))throw Error("Actual supported Windows Bun host required");
 const{dlopen,FFIType:f}=createRequire(import.meta.url)("bun:ffi")as typeof import("bun:ffi"),library=dlopen("kernel32.dll",{CreateFileW:{args:[f.ptr,f.u32,f.u32,f.ptr,f.u32,f.u32,f.u64],returns:f.u64},GetFileInformationByHandleEx:{args:[f.u64,f.i32,f.ptr,f.u32],returns:f.i32},SetFileInformationByHandle:{args:[f.u64,f.i32,f.ptr,f.u32],returns:f.i32},GetVolumeInformationByHandleW:{args:[f.u64,f.ptr,f.u32,f.ptr,f.ptr,f.ptr,f.ptr,f.u32],returns:f.i32},GetFileType:{args:[f.u64],returns:f.u32},GetDriveTypeW:{args:[f.ptr],returns:f.u32},CloseHandle:{args:[f.u64],returns:f.i32},GetLastError:{args:[],returns:f.u32}});
 const symbols=library.symbols as unknown as Record<string,(...args:unknown[])=>unknown>;
 return createConditionalWindowsPortV1({open:(path,access,share,flags)=>BigInt(symbols.CreateFileW!(path,access,share,null,policy.createFile.openExisting,flags,0n)as bigint),information:(handle,kind,data)=>Number(symbols.GetFileInformationByHandleEx!(handle,kind,data,data.length))===1,disposition:(handle,kind,data)=>Number(symbols.SetFileInformationByHandle!(handle,kind,data,data.length))===1,filesystem:(handle,data)=>Number(symbols.GetVolumeInformationByHandleW!(handle,null,0,null,null,null,data,data.length/2))===1,fileType:handle=>Number(symbols.GetFileType!(handle)),driveType:root=>Number(symbols.GetDriveTypeW!(root)),closeHandle:handle=>Number(symbols.CloseHandle!(handle))===1,error:()=>Number(symbols.GetLastError!()),close:()=>library.close()});
}
