import{createRequire}from"node:module";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ The byte layouts of the kernel32 structures and the access rights the native Windows memory reader uses.
 * @see ./🧬️schema/🔣️.json */
export const WINDOWS_MEMORY_LAYOUT_V1=schema.definitions.Layout.const;
const L=WINDOWS_MEMORY_LAYOUT_V1;

/** 🤷️ Why Windows memory is unknown: this is no Windows host (`platform`) or a kernel32 call failed (`host-call`, named with its error code); an unknown never raises the pressure of a host.
 * @see ./🧬️schema/🔣️.json */
export type WindowsMemoryUnknownV1=Readonly<{known:false;reason:"platform"|"host-call";call:string;error:number;detail:string;text:Readonly<{en:string;de:string}>}>;

/** 🌡️ The machine facts of `GlobalMemoryStatusEx`: physical and commit (page file) totals and what is still available.
 * @see https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-memorystatusex */
export type WindowsMemoryFactsV1=Readonly<{known:true;totalBytes:number;availableBytes:number;commitLimitBytes:number;commitAvailableBytes:number;loadPercent:number}>|WindowsMemoryUnknownV1;

/** 🧾️ One process of the toolhelp snapshot with the memory of its kernel counters (`null` when the process cannot be opened). */
export type WindowsProcessRowV1=Readonly<{pid:number;parent:number;name:string;workingSetBytes:number|null;privateBytes:number|null}>;

/** 🗂️ The process table of the machine; `unreadable` counts the processes whose memory the kernel refused.
 * @see https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot */
export type WindowsProcessTableV1=Readonly<{known:true;unreadable:number;rows:readonly WindowsProcessRowV1[]}>|WindowsMemoryUnknownV1;

/** 🔌️ The typed port to the native Windows memory facts, injectable in tests. */
export type WindowsMemoryPortV1=Readonly<{memory:()=>WindowsMemoryFactsV1;processes:()=>WindowsProcessTableV1}>;

/** 🛑️ Builds the typed unknown of a Windows answer, in every language. */
export const windowsMemoryUnknownV1=(reason:"platform"|"host-call",call:string,error:number,detail:string=""):WindowsMemoryUnknownV1=>Object.freeze({known:false,reason,call,error,detail,text:Object.freeze(reason==="platform"?{en:"native Windows memory is read on Windows hosts only",de:"der native Windows-Speicher wird nur auf Windows-Rechnern gelesen"}:{en:`the Windows call ${call} failed (error ${error})${detail===""?"":": "+detail}`,de:`der Windows-Aufruf ${call} schlug fehl (Fehler ${error})${detail===""?"":": "+detail}`})});

type Kernel=Readonly<Record<string,(...args:any[])=>any>>;

const loadKernel=():Kernel=>{
 const{dlopen,FFIType:T}=createRequire(import.meta.url)("bun:ffi") as typeof import("bun:ffi");
 return dlopen("kernel32.dll",{
  GlobalMemoryStatusEx:{args:[T.ptr],returns:T.i32},
  CreateToolhelp32Snapshot:{args:[T.u32,T.u32],returns:T.u64},
  Process32FirstW:{args:[T.u64,T.ptr],returns:T.i32},
  Process32NextW:{args:[T.u64,T.ptr],returns:T.i32},
  OpenProcess:{args:[T.u32,T.i32,T.u32],returns:T.u64},
  K32GetProcessMemoryInfo:{args:[T.u64,T.ptr,T.u32],returns:T.i32},
  CloseHandle:{args:[T.u64],returns:T.i32},
  GetLastError:{args:[],returns:T.u32}
 }).symbols as unknown as Kernel;
};

const fail=(kernel:Kernel,call:string):WindowsMemoryUnknownV1=>windowsMemoryUnknownV1("host-call",call,Number(kernel.GetLastError()));

/** 🪟️ Reads the machine facts through `GlobalMemoryStatusEx` of an already loaded kernel32. */
export function readWindowsMemoryV1(kernel:Kernel):WindowsMemoryFactsV1{
 const buffer=Buffer.alloc(L.memoryStatus.size),view=new DataView(buffer.buffer,buffer.byteOffset,buffer.byteLength);view.setUint32(0,L.memoryStatus.size,true);
 if(!kernel.GlobalMemoryStatusEx(buffer))return fail(kernel,"GlobalMemoryStatusEx");
 const big=(offset:number):number=>Number(view.getBigUint64(offset,true));
 return Object.freeze({known:true,totalBytes:big(L.memoryStatus.totalPhysical),availableBytes:big(L.memoryStatus.availablePhysical),commitLimitBytes:big(L.memoryStatus.totalPageFile),commitAvailableBytes:big(L.memoryStatus.availablePageFile),loadPercent:Math.min(100,view.getUint32(L.memoryStatus.load,true))});
}

function readWindowsProcessCountersV1(kernel:Kernel,pid:number):Readonly<{workingSetBytes:number;privateBytes:number}>|undefined{
 if(!Number.isInteger(pid)||pid<1||pid>0xffffffff)return undefined;
 const handle=kernel.OpenProcess(L.queryLimitedInformation,0,pid) as bigint;
 if(handle===0n)return undefined;
 try{
  const counters=Buffer.alloc(L.memoryCounters.size),view=new DataView(counters.buffer,counters.byteOffset,counters.byteLength);
  view.setUint32(0,L.memoryCounters.size,true);
  if(!kernel.K32GetProcessMemoryInfo(handle,counters,L.memoryCounters.size))return undefined;
  return{workingSetBytes:Number(view.getBigUint64(L.memoryCounters.workingSet,true)),privateBytes:Number(view.getBigUint64(L.memoryCounters.privateUsage,true))};
 }finally{kernel.CloseHandle(handle);}
}

/** 🎯️ Opens one exact Windows process, reads its working set and closes its handle without walking the process table. */
export function readWindowsProcessWorkingSetV1(kernel:Kernel,pid:number):number|undefined{
 const bytes=readWindowsProcessCountersV1(kernel,pid)?.workingSetBytes;
 return Number.isSafeInteger(bytes)?bytes:undefined;
}

let workingSetKernel:Kernel|null|undefined;
/** 🪟️ Supplies a single-process working-set measurement through the native Windows capacity backend. */
export function systemWindowsProcessWorkingSetV1(pid:number):number|undefined{
 if(process.platform!=="win32")return undefined;
 if(workingSetKernel===undefined){try{workingSetKernel=loadKernel();}catch{workingSetKernel=null;}}
 if(workingSetKernel===null)return undefined;
 try{return readWindowsProcessWorkingSetV1(workingSetKernel,pid);}catch{return undefined;}
}

/** 🗂️ Walks the toolhelp snapshot of an already loaded kernel32 and reads the memory counters of every process it may open. */
export function readWindowsProcessesV1(kernel:Kernel):WindowsProcessTableV1{
 const invalid=BigInt(L.invalidHandle),snapshot=kernel.CreateToolhelp32Snapshot(L.snapshotProcesses,0) as bigint;
 if(snapshot===invalid||snapshot===0n)return fail(kernel,"CreateToolhelp32Snapshot");
 const entry=Buffer.alloc(L.processEntry.size),view=new DataView(entry.buffer,entry.byteOffset,entry.byteLength),rows:WindowsProcessRowV1[]=[];let unreadable=0;
 try{
  view.setUint32(0,L.processEntry.size,true);
  let more=kernel.Process32FirstW(snapshot,entry) as number;
  if(!more){const error=Number(kernel.GetLastError());if(error!==18)return windowsMemoryUnknownV1("host-call","Process32FirstW",error);}
  while(more){
   const pid=view.getUint32(L.processEntry.processId,true),parent=view.getUint32(L.processEntry.parentProcessId,true);
   let name="";for(let index=0;index<L.processEntry.executableNameCharacters;index++){const unit=view.getUint16(L.processEntry.executableName+index*2,true);if(unit===0)break;name+=String.fromCharCode(unit);}
   const counters=readWindowsProcessCountersV1(kernel,pid);
   if(counters===undefined)unreadable++;
   rows.push(Object.freeze({pid,parent,name,workingSetBytes:counters?.workingSetBytes??null,privateBytes:counters?.privateBytes??null}));
   view.setUint32(0,L.processEntry.size,true);more=kernel.Process32NextW(snapshot,entry) as number;
  }
 }finally{kernel.CloseHandle(snapshot);}
 return Object.freeze({known:true,unreadable,rows:Object.freeze(rows)});
}

/** 🏁️ Makes the port of the running machine: on Windows it calls kernel32 through `bun:ffi` (loaded on first use), anywhere else it answers the typed unknown `platform` without loading any foreign library; a failing load answers `host-call`.
 * @see ./🧪️tests/🟦️.ts */
export function systemWindowsMemoryPortV1(platform:NodeJS.Platform=process.platform):WindowsMemoryPortV1{
 if(platform!=="win32"){const unknown=windowsMemoryUnknownV1("platform","",0);return Object.freeze({memory:()=>unknown,processes:()=>unknown});}
 let kernel:Kernel|WindowsMemoryUnknownV1|undefined;
 const load=():Kernel|WindowsMemoryUnknownV1=>kernel??=((): Kernel|WindowsMemoryUnknownV1=>{try{return loadKernel();}catch(error){return windowsMemoryUnknownV1("host-call","dlopen kernel32.dll",0,String((error as Error)?.message??error).split("\n")[0]!);}})();
 const use=<T>(run:(kernel:Kernel)=>T):T|WindowsMemoryUnknownV1=>{const loaded=load();return"known"in loaded?loaded as WindowsMemoryUnknownV1:run(loaded as Kernel);};
 return Object.freeze({memory:()=>use(readWindowsMemoryV1),processes:()=>use(readWindowsProcessesV1)});
}
