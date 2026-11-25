import{createRequire}from"node:module";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ The call, flavor and byte layout of the native macOS footprint read.
 * @see ./🧬️schema/🔣️.json */
export const DARWIN_FOOTPRINT_LAYOUT_V1=schema.definitions.Layout.const;
const L=DARWIN_FOOTPRINT_LAYOUT_V1;

/** 🔌️ The typed port to the native footprint: the bytes the kernel charges one process (`phys_footprint`), `null` when the process is gone or not ours to read. Injectable in tests. */
export type DarwinFootprintPortV1=Readonly<{footprint:(pid:number)=>number|null}>;

/** 🧩️ The one call of libproc the port needs, as `bun:ffi` and the tests fake it. */
export type DarwinRusageKernelV1=Readonly<{proc_pid_rusage:(pid:number,flavor:number,buffer:Buffer)=>number}>;

/** 🍏️ Reads the footprint of one process through an already loaded libproc: 0 from the call and a safe integer at the footprint offset, anything else is `null`.
 * @see https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/proc_pid_rusage.3.html */
export function readDarwinFootprintV1(kernel:DarwinRusageKernelV1,pid:number,buffer:Buffer=Buffer.alloc(L.size)):number|null{
 if(!Number.isInteger(pid)||pid<1||kernel.proc_pid_rusage(pid,L.flavor,buffer)!==0)return null;
 const bytes=Number(new DataView(buffer.buffer,buffer.byteOffset,buffer.byteLength).getBigUint64(L.physicalFootprint,true));
 return Number.isSafeInteger(bytes)?bytes:null;
}

const loadKernel=():DarwinRusageKernelV1=>{
 const{dlopen,FFIType:T,ptr}=createRequire(import.meta.url)("bun:ffi") as typeof import("bun:ffi");
 const symbols=dlopen(L.library,{proc_pid_rusage:{args:[T.i32,T.i32,T.ptr],returns:T.i32}}).symbols;
 return{proc_pid_rusage:(pid,flavor,buffer)=>symbols.proc_pid_rusage(pid,flavor,ptr(buffer))};
};

let system:DarwinFootprintPortV1|null|undefined;
/** 🏁️ Makes the port of the running machine: on macOS it calls libproc through `bun:ffi` (loaded on first use, one reused buffer, about two microseconds per process); anywhere else, outside bun or where libproc cannot be loaded it answers `null`, and the caller reads the footprint another way. */
export function systemDarwinFootprintPortV1(platform:NodeJS.Platform=process.platform):DarwinFootprintPortV1|null{
 if(platform!=="darwin")return null;
 if(system===undefined)try{const kernel=loadKernel(),buffer=Buffer.alloc(L.size);system=Object.freeze({footprint:pid=>{try{return readDarwinFootprintV1(kernel,pid,buffer);}catch{return null;}}});}catch{system=null;}
 return system;
}
