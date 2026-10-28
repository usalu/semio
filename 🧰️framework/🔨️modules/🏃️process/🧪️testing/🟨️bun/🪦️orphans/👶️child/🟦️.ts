import{platform as osPlatform}from"node:os";
import{Worker}from"node:worker_threads";
import{runtimeNativeSystemV1}from"../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";
import{runtimeNativeLibcV1}from"../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/📚️libc/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};

/** 📏️ The authored policy and the words of the guard (kept apart from the runner side, which loads far more: this file is all a test process pays for). */
export const testOrphansProtocolV1=schema.definitions.Protocol.const;
/** 💬️ The two languages of the guard's words. */
export const testOrphansMessagesV1=schema.definitions.Messages.const;
/** ✏️ Fills the `{name}` holes of a message. */
export const fillTestOrphansMessageV1=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);

/** 🎚️ The open files one test process may hold: `SEMIO_TEST_OPEN_FILES_CEILING` when it is a whole number not below the minimum, else the authored ceiling. */
export function openFilesCeilingV1(environment:Readonly<Record<string,string|undefined>>=process.env):number{
 const value=Number(environment[testOrphansProtocolV1.openFilesVariable]);
 return Number.isSafeInteger(value)&&value>=testOrphansProtocolV1.minimumOpenFilesCeiling?value:testOrphansProtocolV1.openFilesCeiling;
}

/** 🛡️ Lowers the soft limit of open files of THIS process to `ceiling` (never raises it, never touches the hard limit) through `setrlimit`; answers the limit now in force, or nothing where the host has no such limit (Windows) or the call is refused. See https://man7.org/linux/man-pages/man2/getrlimit.2.html */
export function limitOpenFilesV1(ceiling:number,platform:string=osPlatform()):number|undefined{
 const resource=(testOrphansProtocolV1.resourceLimit as Readonly<Record<string,number>>)[platform];
 if(resource===undefined)return undefined;
 try{
  const system=runtimeNativeSystemV1(),library=system.dlopen(runtimeNativeLibcV1(),{getrlimit:{args:["i32","ptr"],returns:"i32"},setrlimit:{args:["i32","ptr"],returns:"i32"}}),buffer=Buffer.alloc(16);
  try{
   if(library.symbols.getrlimit(resource,system.ptr(buffer))!==0)return undefined;
   const current=buffer.readBigUInt64LE(0);
   if(current<=BigInt(ceiling))return Number(current);
   buffer.writeBigUInt64LE(BigInt(ceiling),0);
   return library.symbols.setrlimit(resource,system.ptr(buffer))===0?ceiling:undefined;
  }finally{library.close();}
 }catch{return undefined;}
}

/** 🪦️ The child side of the guard, started by the test preload: caps the open files of this `bun test` process and ends its whole tree when its parent is gone (a worker thread checks, so a main thread blocked in a synchronous child call is still caught); answers nothing when `SEMIO_TEST_ORPHAN_GUARD=off`. */
export function startTestOrphanGuardV1(environment:Readonly<Record<string,string|undefined>>=process.env):Readonly<{ceiling:number;limit:number|undefined;watching:boolean;stop:()=>void}>|undefined{
 if(environment[testOrphansProtocolV1.guardSwitch]==="off")return undefined;
 const ceiling=openFilesCeilingV1(environment),limit=limitOpenFilesV1(ceiling),parent=process.ppid;
 if(parent<=1)return Object.freeze({ceiling,limit,watching:false,stop:()=>{}});
 const worker=new Worker(new URL("../👷️parent/📜️script.ts",import.meta.url),{workerData:{parent,pid:process.pid,interval:testOrphansProtocolV1.parentPollMilliseconds}});
 worker.unref();
 return Object.freeze({ceiling,limit,watching:true,stop:()=>{void worker.terminate();}});
}
