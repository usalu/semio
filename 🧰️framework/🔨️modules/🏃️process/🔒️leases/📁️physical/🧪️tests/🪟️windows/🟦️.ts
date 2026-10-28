import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createPosixPhysicalLockBackendV1,createWindowsPhysicalLockBackendV1,holdPhysicalFileLocksV1,physicalLockBackendV1,windowsExtendedPathV1,type PhysicalLockBackendV1} from "../../🟦️.ts";
import {testPythonV1} from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";

const oracle=join(import.meta.dir,"🐍️oracle","🐍️.py"),python=testPythonV1()??Bun.which("python3")??"python3";
const python3=(...args:string[]):unknown=>JSON.parse(spawnSync(python,["-I",oracle,...args],{encoding:"utf8"}).stdout);
const win=schema.definitions.Windows.const;
const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,published:[] as string[],publish(progress:{resource:string}){this.published.push(progress.resource);},yieldContinuation:async()=>undefined};
const ROOT="C:\\build\\debug\\",FIRST=ROOT+"a\\.cargo-build-lock",SECOND=ROOT+"b\\.cargo-build-lock";

/** 🧪️ A model of the kernel32 file and byte-range lock calls the backend makes: per-handle ownership, `LOCKFILE_FAIL_IMMEDIATELY` answering ERROR_LOCK_VIOLATION (33) for any other holder and for a second lock through the same handle, locks dropped when their handle closes, the structure written with the layout the independent oracle computed. */
function kernelModelV1(layout:Record<string,number>){
 type Row={volume:number;high:number;low:number;attributes:number;owner:bigint|null};
 const files=new Map<string,Row>(),handles=new Map<bigint,string>(),pointers=new Map<number,Buffer>(),calls:string[]=[],holders=new Set<bigint>(),state={error:0,next:100n,pointer:4096,libraries:0,failLock:0,failOpen:0,stuck:false};
 const name=(pointer:number):string=>{const bytes=pointers.get(pointer)!,text=bytes.toString("utf16le").replace(/\0.*$/su,"");expect(text.startsWith("\\\\?\\")).toBe(true);return text.slice(4);};
 const system={
  ptr:(view:ArrayBufferView):number=>{state.pointer+=8;pointers.set(state.pointer,Buffer.from(view.buffer,view.byteOffset,view.byteLength));return state.pointer;},
  dlopen:(library:string,symbols:Record<string,unknown>)=>{
   expect(library).toBe(win.library);expect(Object.keys(symbols).sort()).toEqual(["CloseHandle","CreateFileW","GetFileInformationByHandle","GetLastError","LockFileEx","UnlockFileEx"]);state.libraries++;
   const fail=(code:number,value:number|bigint):number|bigint=>{state.error=code;return value;};
   return {close:()=>{state.libraries--;},symbols:{
    CreateFileW:(pointer:number,access:number,share:number,security:null,creation:number,flags:number,template:bigint)=>{
     calls.push("open");expect([win.access,win.metadataAccess]).toContain(access);expect(share).toBe(7);expect(security).toBeNull();expect(creation).toBe(3);expect(flags).toBe(win.reparseFlag);expect(template).toBe(0n);
     if(state.failOpen>0&&--state.failOpen===0)return fail(5,-1n);
     const key=name(pointer);if(!files.has(key))return fail(2,-1n);const handle=state.next++;handles.set(handle,key);return handle;},
    GetFileInformationByHandle:(handle:bigint,pointer:number)=>{
     const key=handles.get(handle);if(key===undefined)return fail(6,0);const row=files.get(key)!,bytes=pointers.get(pointer)!;expect(bytes.length).toBe(layout.bytes);bytes.fill(0);
     bytes.writeUInt32LE(row.attributes,layout.attributes);bytes.writeUInt32LE(row.volume,layout.volume);bytes.writeUInt32LE(1,layout.links);bytes.writeUInt32LE(row.high,layout.indexHigh);bytes.writeUInt32LE(row.low,layout.indexLow);return 1;},
    LockFileEx:(handle:bigint,flags:number,reserved:number,low:number,high:number,pointer:number)=>{
     calls.push("lock");expect(flags).toBe(win.exclusive|win.failImmediately);expect(reserved).toBe(0);expect([low,high]).toEqual([4294967295,4294967295]);expect(pointers.get(pointer)!.length).toBeGreaterThanOrEqual(layout.overlapped);expect(pointers.get(pointer)!.every(byte=>byte===0)).toBe(true);
     if(state.failLock>0&&--state.failLock===0)state.stuck=true;
     if(state.stuck)return fail(5,0);
     const row=files.get(handles.get(handle)!)!;if(row.owner!==null)return fail(33,0);row.owner=handle;return 1;},
    UnlockFileEx:(handle:bigint,reserved:number,low:number,high:number)=>{calls.push("unlock");expect([reserved,low,high]).toEqual([0,4294967295,4294967295]);const row=files.get(handles.get(handle)!)!;if(row.owner!==handle)return fail(158,0);row.owner=null;return 1;},
    CloseHandle:(handle:bigint)=>{calls.push("close");const key=handles.get(handle);if(key===undefined)return fail(6,0);const row=files.get(key)!;if(row.owner===handle)row.owner=null;handles.delete(handle);return 1;},
    GetLastError:()=>state.error,
   }};
  },
 };
 const add=(path:string,identity:Partial<Row>={})=>files.set(path,{volume:7,high:1,low:files.size+1,attributes:0x80,owner:null,...identity});
 const holder=(path:string):(()=>void)=>{const handle=state.next++;handles.set(handle,path);holders.add(handle);const row=files.get(path)!;if(row.owner!==null)throw Error("model holder collision");row.owner=handle;return()=>{if(row.owner===handle)row.owner=null;handles.delete(handle);holders.delete(handle);};};
 const backendOwned=():number=>[...files.values()].filter(row=>row.owner!==null&&!holders.has(row.owner)).length+[...handles.keys()].filter(handle=>!holders.has(handle)).length*100+state.libraries*10000;
 return {system,files,handles,calls,state,add,holder,backendOwned};
}

test("the structure layouts and the extended-path rule equal the independent oracle (SDK field lists, ntpath)",()=>{
 const layout=python3("layout") as Record<string,number>;
 expect(win.information).toEqual({bytes:layout.bytes,attributes:layout.attributes,volume:layout.volume,links:layout.links,indexHigh:layout.indexHigh,indexLow:layout.indexLow});
 expect(win.overlappedBytes).toBeGreaterThanOrEqual(layout.overlapped as number);
 const paths=["C:\\a\\b","d:\\x y\\ü\\z","\\\\srv\\share\\dir\\f","\\\\?\\C:\\already","\\\\?\\UNC\\srv\\share\\f","relative\\x","C:drive-relative"];
 const expected=python3("extended",...paths) as (string|null)[];
 paths.forEach((path,index)=>{if(expected[index]===null)expect(()=>windowsExtendedPathV1(path),path).toThrow("Absolute Windows path required");else expect(windowsExtendedPathV1(path),path).toBe(expected[index]!);});
 expect(win.contentionError).toBe(33);expect(win.invalidHandle).toBe("-1");expect(win.exclusive|win.failImmediately).toBe(3);
});

test("backend selection: posix and Windows hosts get a mechanism, every other host is refused before a file is touched",()=>{
 expect(()=>physicalLockBackendV1("freebsd")).toThrow("Qualified physical compiler file lock backend required");
 expect(()=>physicalLockBackendV1("aix")).toThrow("Qualified physical compiler file lock backend required");
 const model=kernelModelV1(python3("layout") as Record<string,number>);expect(typeof physicalLockBackendV1("win32",model.system as never).tryLock).toBe("function");expect(model.state.libraries).toBe(1);
});

/** 📋️ One scenario table, run on any backend: what the caller observes, never how the backend does it. */
async function scenariosV1(make:()=>PhysicalLockBackendV1,paths:readonly [string,string],hold:(path:string)=>()=>void,after:()=>unknown):Promise<Record<string,unknown>>{
 const out:Record<string,unknown>={},request=(wait:boolean)=>({version:1 as const,wait});
 let locks=await holdPhysicalFileLocksV1(paths,control,request(false),make());locks.check();out.takeBoth=[locks.paths.length,locks.pid===process.pid];
 locks.close();locks.close();out.closeTwice="ok";out.afterClose=String((()=>{try{locks.check();return"checked";}catch(error){return(error as Error).message;}})());out.released=after();
 const release=hold(paths[1]);
 try{await holdPhysicalFileLocksV1(paths,control,request(false),make());out.heldNoWait="acquired";}catch(error){out.heldNoWait=(error as Error).message;}
 out.leakedAfterRefusal=after();
 const waiting=holdPhysicalFileLocksV1(paths,control,request(true),make());setTimeout(release,120);
 locks=await waiting;out.waitedThenTook=locks.paths.length;locks.close();out.releasedAfterWait=after();
 return out;
}

test("Windows backend: the scenario table equals the real POSIX flock backend (same observable behaviour on both mechanisms)",async()=>{
 const layout=python3("layout") as Record<string,number>,model=kernelModelV1(layout);model.add(FIRST);model.add(SECOND);
 const windows=await scenariosV1(()=>createWindowsPhysicalLockBackendV1(model.system as never),[FIRST,SECOND],path=>model.holder(path),()=>model.backendOwned());
 const directory=mkdtempSync(join(tmpdir(),"semio-physical-")),first=join(directory,"a.lock"),second=join(directory,"b.lock");
 try{
  writeFileSync(first,"a");writeFileSync(second,"b");
  const open:PhysicalLockBackendV1[]=[],posix=await scenariosV1(()=>createPosixPhysicalLockBackendV1(),[first,second],path=>{const holder=createPosixPhysicalLockBackendV1();open.push(holder);holder.retain(path);expect(holder.tryLock()).toBe(true);return()=>holder.close();},()=>0);
  expect({...windows,released:0,leakedAfterRefusal:0,releasedAfterWait:0}).toEqual({...posix,released:0,leakedAfterRefusal:0,releasedAfterWait:0});
  expect(windows.heldNoWait).toBe("Original physical compiler lock remains held");expect(windows.afterClose).toBe("Physical compiler lock owner closed");
 }finally{rmSync(directory,{recursive:true,force:true});}
 expect(model.handles.size,"every handle closed").toBe(0);expect(model.state.libraries,"every library closed").toBe(0);expect(windows.released).toBe(0);expect(windows.leakedAfterRefusal).toBe(0);expect(windows.releasedAfterWait).toBe(0);
});

test("Windows backend: one lock per handle, so a second handle of the same process is excluded like another process (Cargo, a scanner holding the range)",async()=>{
 const model=kernelModelV1(python3("layout") as Record<string,number>);model.add(FIRST);
 const held=createWindowsPhysicalLockBackendV1(model.system as never),second=createWindowsPhysicalLockBackendV1(model.system as never);
 held.retain(FIRST);expect(held.tryLock()).toBe(true);second.retain(FIRST);expect(second.tryLock()).toBe(false);second.close();held.close();
 expect([...model.files.values()][0]!.owner).toBeNull();expect(model.handles.size).toBe(0);expect(model.calls.filter(call=>call==="unlock").length).toBe(1);
});

test("Windows backend refusals close every handle: missing file, junction or symlink, directory, replaced file, unexpected lock error",async()=>{
 const layout=python3("layout") as Record<string,number>,request={version:1 as const,wait:false};
 const run=async(prepare:(model:ReturnType<typeof kernelModelV1>)=>void,act:(locks:Awaited<ReturnType<typeof holdPhysicalFileLocksV1>>|null,model:ReturnType<typeof kernelModelV1>)=>void,paths=[FIRST,SECOND]):Promise<string>=>{
  const model=kernelModelV1(layout);model.add(FIRST);model.add(SECOND);prepare(model);let message="ok";
  try{const locks=await holdPhysicalFileLocksV1(paths,control,request,createWindowsPhysicalLockBackendV1(model.system as never));try{act(locks,model);locks.check();}finally{locks.close();}}catch(error){message=(error as Error).message;}
  expect(model.handles.size,message).toBe(0);expect(model.state.libraries,message).toBe(0);expect([...model.files.values()].every(row=>row.owner===null),message).toBe(true);return message;
 };
 expect(await run(model=>{model.files.delete(SECOND);},()=>undefined)).toBe("Physical compiler lock file cannot be opened (Windows error 2)");
 expect(await run(model=>{model.state.failOpen=2;},()=>undefined)).toBe("Physical compiler lock file cannot be opened (Windows error 5)");
 expect(await run(model=>{model.files.get(SECOND)!.attributes=0x400;},()=>undefined)).toBe("Physical compiler lock identity changed");
 expect(await run(model=>{model.files.get(FIRST)!.attributes=0x10;},()=>undefined)).toBe("Physical compiler lock identity changed");
 expect(await run(model=>{model.state.failLock=2;},()=>undefined)).toBe("Physical compiler lock admission failed (Windows error 5)");
 expect(await run(()=>undefined,(_locks,model)=>{model.files.get(FIRST)!.low=99;})).toBe("Original physical compiler lock identity changed");
 expect(await run(()=>undefined,(_locks,model)=>{model.files.get(FIRST)!.volume=8;})).toBe("Original physical compiler lock identity changed");
 expect(await run(()=>undefined,()=>undefined)).toBe("ok");
});

test("the real Windows kernel and an independent observer agree (Python msvcrt.locking): runs only on a Windows host",async()=>{
 if(process.platform!=="win32"){expect(physicalLockBackendV1("win32",kernelModelV1(python3("layout") as Record<string,number>).system as never)).toBeDefined();return;}
 const {provePhysicalFileLocksV1}=await import("../🟦️.ts");
 const directory=mkdtempSync(join(tmpdir(),"semio-physical-"));try{await provePhysicalFileLocksV1(directory,control);}finally{rmSync(directory,{recursive:true,force:true});}
},30000);
