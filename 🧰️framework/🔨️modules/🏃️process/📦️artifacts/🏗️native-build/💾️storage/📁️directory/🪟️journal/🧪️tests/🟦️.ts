import {expect,test} from "bun:test";
import Ajv from "ajv";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,renameSync,rmSync,symlinkSync,writeFileSync,existsSync} from "node:fs";
import {basename,join} from "node:path";
import {tmpdir} from "node:os";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {activateNativeDirectoryV1,allocateNativeDirectoryV1,deactivateNativeDirectoryV1,exchangeNativeDirectoriesV1,nativeDirectoryProtocolV1,nativeDirectorySwapPendingV1,observeNativeDirectoryIdentityV1,observeNativeDirectoryStateV1,recoverNativeDirectoryV1,type NativeDirectoryIdentityV1} from "../../🟦️.ts";
import {createNativeDirectoryJournaledPortV1,moveNativeDirectoryWithRetryV1,nativeDirectorySwapIntentBytesV1,readNativeDirectorySwapIntentV1} from "../🟦️.ts";
import type {NativeWindowsHostPortV1} from "../../🪟️host/🟦️.ts";

const protocol=nativeDirectoryProtocolV1();
const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?process.env.SEMIO_TEST_ARTIFACT_DIR:tmpdir();mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"win-"));};
class Crash extends Error{}
type Script={crash?:{call:number;when:"before"|"after"};failMoves?:readonly number[];error?:number;flushError?:number;hook?:(entry:string)=>void};
const simulatedHost=(script:Script={})=>{
 const log:string[]=[];let calls=0,moves=0;
 const enter=(entry:string)=>{calls++;log.push(entry);script.hook?.(entry);if(script.crash?.call===calls&&script.crash.when==="before")throw new Crash(entry);};
 const leave=()=>{if(script.crash?.call===calls&&script.crash.when==="after")throw new Crash("after");};
 const host:NativeWindowsHostPortV1={
  moveFile:(from,to)=>{enter("move:"+basename(from)+">"+basename(to));moves++;if(script.failMoves?.includes(moves))return{ok:false,error:script.error??32};renameSync(from,to);leave();return{ok:true,error:0};},
  flushDirectory:path=>{enter("flush:"+basename(path));leave();return script.flushError?{ok:false,error:script.flushError}:{ok:true,error:0};},
  setCaseSensitive:()=>({opened:true,error:0,status:0}),
  availableBytes:()=>({ok:true,error:0,bytes:0}),
  openExclusive:()=>({ok:true,error:0})
 };
 return{host,log};
};
const names=(directory:string)=>readdirSync(directory).sort();
const partition=(base:string,id:string)=>{
 const directory=join(base,id);mkdirSync(directory);mkdirSync(join(directory,"m"));mkdirSync(join(directory,"v"));writeFileSync(join(directory,"v","marker"),"volume");
 return{directory,placeholder:observeNativeDirectoryIdentityV1(join(directory,"m")),volume:observeNativeDirectoryIdentityV1(join(directory,"v"))};
};
const holder=(directory:string,placeholder:NativeDirectoryIdentityV1,volume:NativeDirectoryIdentityV1)=>{
 const root=observeNativeDirectoryIdentityV1(join(directory,"m")),other=observeNativeDirectoryIdentityV1(join(directory,"v"));
 const name=(identity:NativeDirectoryIdentityV1)=>identity.ino===placeholder.ino?"placeholder":identity.ino===volume.ino?"volume":"foreign";
 return{m:name(root),v:name(other)};
};

test("the intent receipt is canonical, closed and agrees with Ajv",()=>{
 const oracle=new Ajv({strict:true});oracle.addSchema(schema,"directory");const intent=oracle.getSchema("directory#/definitions/SwapIntent")!;
 const identity={dev:"16777234",ino:"12345",mode:16877},value={version:1 as const,kind:"directory-swap" as const,first:"m",second:"v",temporary:"m.old",identities:{first:identity,second:{...identity,ino:"12346"}}};
 expect(intent(value)).toBe(true);const bytes=nativeDirectorySwapIntentBytesV1(value);expect(readNativeDirectorySwapIntentV1(bytes)).toEqual(value);
 expect(()=>readNativeDirectorySwapIntentV1(Buffer.from(JSON.stringify(value,null,1)))).toThrow("Canonical directory swap intent");
 for(const bad of[{...value,extra:1},{...value,first:"a/b"},{...value,identities:{first:identity}},{...value,kind:"other"},{...value,temporary:""}]){expect(intent(bad)).toBe(false);expect(()=>nativeDirectorySwapIntentBytesV1(bad as never)).toThrow("Closed directory swap intent");}
 expect(()=>readNativeDirectorySwapIntentV1(Buffer.alloc(protocol.swap.maximumIntentBytes+1))).toThrow("Bounded directory swap intent");
});

test("a completed swap issues exactly the declared host sequence and keeps the volume content reachable",()=>{
 const base=scratch();try{
  const p=partition(base,"p"),{host,log}=simulatedHost(),port=createNativeDirectoryJournaledPortV1(host);
  port.swap(join(p.directory,"m"),join(p.directory,"v"));
  expect(log).toEqual(fixture.hostSequence);expect(names(p.directory)).toEqual(["m","v"]);expect(readFileSync(join(p.directory,"m","marker"),"utf8")).toBe("volume");expect(holder(p.directory,p.placeholder,p.volume)).toEqual({m:"volume",v:"placeholder"});expect(port.pending(p.directory)).toBe(false);
  port.swap(join(p.directory,"m"),join(p.directory,"v"));expect(holder(p.directory,p.placeholder,p.volume)).toEqual({m:"placeholder",v:"volume"});
  expect(()=>port.swap(join(p.directory,"m"),join(base,"elsewhere"))).toThrow(protocol.refusals.swapSiblings);
  expect(()=>port.swap(join(p.directory,"m"),join(p.directory,"m"))).toThrow(protocol.refusals.swapSiblings);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("readers see the root name missing only between the first and second rename",()=>{
 const base=scratch();try{
  const p=partition(base,"p"),seen:{before:string;rootExists:boolean}[]=[],{host}=simulatedHost({hook:entry=>{if(entry.startsWith("move:"))seen.push({before:entry,rootExists:existsSync(join(p.directory,"m"))});}});
  createNativeDirectoryJournaledPortV1(host).swap(join(p.directory,"m"),join(p.directory,"v"));
  expect(seen).toEqual(fixture.readerWindow);expect(existsSync(join(p.directory,"m"))).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});

const build=(base:string,row:{id:string;layout:Record<string,string|null>;intent:string})=>{
 const directory=join(base,row.id);mkdirSync(directory);const made=new Map<string,NativeDirectoryIdentityV1>();
 for(const[name,tag]of Object.entries(row.layout)){if(!tag)continue;const path=join(directory,name);mkdirSync(path);writeFileSync(join(path,"marker"),tag==="B"?"volume":tag);made.set(tag,observeNativeDirectoryIdentityV1(path));}
 const a=made.get("A")??observeNativeDirectoryIdentityV1((()=>{const path=join(directory,"_a");mkdirSync(path);return path;})()),b=made.get("B")??observeNativeDirectoryIdentityV1((()=>{const path=join(directory,"_b");mkdirSync(path);return path;})());
 const intent=join(directory,protocol.swap.intent),record={version:1 as const,kind:"directory-swap" as const,first:"m",second:"v",temporary:row.intent==="wrong-temporary"?"x.old":protocol.swap.temporary,identities:{first:a,second:b}},bytes=nativeDirectorySwapIntentBytesV1(record);
 if(row.intent==="canonical"||row.intent==="wrong-temporary")writeFileSync(intent,bytes);
 else if(row.intent==="torn")writeFileSync(intent,bytes.subarray(0,17));
 else if(row.intent==="empty")writeFileSync(intent,"");
 else if(row.intent==="symlink")symlinkSync(directory,intent);
 else if(row.intent==="extra-field")writeFileSync(intent,JSON.stringify({...record,extra:true})+"\n");
 for(const spare of["_a","_b"])if(existsSync(join(directory,spare)))rmSync(join(directory,spare),{recursive:true});
 return{directory,a,b};
};
const holderOf=(directory:string,a:NativeDirectoryIdentityV1,b:NativeDirectoryIdentityV1,name:string)=>{const path=join(directory,name);if(!existsSync(path))return null;const identity=observeNativeDirectoryIdentityV1(path);return identity.ino===a.ino?"A":identity.ino===b.ino?"B":"X";};

test("recovery from every interrupted on-disk state completes or rolls back from the receipt",()=>{
 const base=scratch();try{
  for(const row of fixture.states){
   const{directory,a,b}=build(base,row),port=createNativeDirectoryJournaledPortV1(simulatedHost().host);
   expect(port.pending(directory),row.id).toBe(row.intent!=="absent");
   expect(port.recover(directory),row.id).toBe(row.outcome as never);
   expect(names(directory),row.id).toEqual(["m","v"]);expect([holderOf(directory,a,b,"m"),holderOf(directory,a,b,"v")],row.id).toEqual([row.final.m,row.final.v]);expect(port.pending(directory),row.id).toBe(false);
   expect(port.recover(directory),row.id).toBe("clean");
  }
  for(const row of fixture.refused){
   const{directory}=build(base,row),port=createNativeDirectoryJournaledPortV1(simulatedHost().host),before=names(directory);
   expect(()=>port.recover(directory),row.id).toThrow(protocol.refusals.swapState);expect(names(directory),row.id).toEqual(before);
  }
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a crash before or after any host call is recovered by the next opener to the original or the swapped state",()=>{
 const base=scratch();try{
  for(const row of fixture.crashMatrix){
   const id=`c${row.call}-${row.when}`,p=partition(base,id),{host}=simulatedHost({crash:{call:row.call,when:row.when as "before"|"after"}}),port=createNativeDirectoryJournaledPortV1(host);
   expect(()=>port.swap(join(p.directory,"m"),join(p.directory,"v")),id).toThrow(Crash);
   const restarted=createNativeDirectoryJournaledPortV1(simulatedHost().host);
   expect(restarted.pending(p.directory),id).toBe(row.pending);
   if(row.pending)expect(()=>observeNativeDirectoryStateV1(p.directory,restarted),id).toThrow(protocol.refusals.interrupted);
   expect(restarted.recover(p.directory),id).toBe(row.outcome as never);
   expect(names(p.directory),id).toEqual(["m","v"]);expect(holder(p.directory,p.placeholder,p.volume),id).toEqual(row.swapped?{m:"volume",v:"placeholder"}:{m:"placeholder",v:"volume"});
   expect(readFileSync(join(p.directory,row.swapped?"m":"v","marker"),"utf8"),id).toBe("volume");expect(restarted.pending(p.directory),id).toBe(false);
   restarted.swap(join(p.directory,"m"),join(p.directory,"v"));expect(holder(p.directory,p.placeholder,p.volume),id).toEqual(row.swapped?{m:"placeholder",v:"volume"}:{m:"volume",v:"placeholder"});
  }
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a refused rename rolls back, or leaves the receipt for recovery once the first rename is half-way",()=>{
 const base=scratch();try{
  for(const row of fixture.failures){
   const p=partition(base,row.id),{host,log}=simulatedHost({failMoves:row.call===1&&[5,32].includes(row.error)?Array.from({length:protocol.swap.retry.attempts},(_,index)=>index+1):[row.call],error:row.error}),port=createNativeDirectoryJournaledPortV1(host);
   let failure:Error|undefined;try{port.swap(join(p.directory,"m"),join(p.directory,"v"));}catch(error){failure=error as Error;}
   expect(failure?.message,row.id).toContain(protocol.refusals.windowsRename);expect(failure?.message,row.id).toContain(row.message);expect(failure!.message.includes(protocol.refusals.windowsOpenHandle),row.id).toBe(row.hint);
   if(row.restores){expect(names(p.directory),row.id).toEqual(["m","v"]);expect(holder(p.directory,p.placeholder,p.volume),row.id).toEqual({m:"placeholder",v:"volume"});expect(port.pending(p.directory),row.id).toBe(false);}
   else{expect(port.pending(p.directory),row.id).toBe(true);expect(log.filter(entry=>entry.startsWith("move:")).length,row.id).toBe(row.call);expect(createNativeDirectoryJournaledPortV1(simulatedHost().host).recover(p.directory),row.id).toBe("completed");expect(holder(p.directory,p.placeholder,p.volume),row.id).toEqual({m:"volume",v:"placeholder"});}
  }
  const p=partition(base,"rollback-refused"),{host}=simulatedHost({failMoves:[2,3],error:5}),port=createNativeDirectoryJournaledPortV1(host);
  expect(()=>port.swap(join(p.directory,"m"),join(p.directory,"v"))).toThrow(protocol.refusals.interrupted);expect(port.pending(p.directory)).toBe(true);
  expect(names(p.directory)).toEqual([protocol.swap.intent,protocol.swap.temporary,"v"].sort());
  expect(createNativeDirectoryJournaledPortV1(simulatedHost().host).recover(p.directory)).toBe("completed");expect(holder(p.directory,p.placeholder,p.volume)).toEqual({m:"volume",v:"placeholder"});
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a second swap or a stray temporary name is refused until the interrupted one is recovered",()=>{
 const base=scratch();try{
  const p=partition(base,"p"),crashing=createNativeDirectoryJournaledPortV1(simulatedHost({crash:{call:3,when:"before"}}).host);
  expect(()=>crashing.swap(join(p.directory,"m"),join(p.directory,"v"))).toThrow(Crash);
  expect(()=>createNativeDirectoryJournaledPortV1(simulatedHost().host).swap(join(p.directory,"m.old"),join(p.directory,"v"))).toThrow(protocol.refusals.interrupted);
  recoverNativeDirectoryV1(p.directory,createNativeDirectoryJournaledPortV1(simulatedHost().host));
  const stray=partition(base,"stray");mkdirSync(join(stray.directory,protocol.swap.temporary));
  expect(()=>createNativeDirectoryJournaledPortV1(simulatedHost().host).swap(join(stray.directory,"m"),join(stray.directory,"v"))).toThrow(protocol.refusals.interrupted);expect(names(stray.directory)).toEqual(["m","m.old","v"]);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("flush errors are tolerated only for the declared Win32 codes and the write-through renames still run",()=>{
 const base=scratch();try{
  for(const row of fixture.flush){
   const p=partition(base,row.id),port=createNativeDirectoryJournaledPortV1(simulatedHost({flushError:row.error}).host);
   if(row.tolerated){port.swap(join(p.directory,"m"),join(p.directory,"v"));expect(holder(p.directory,p.placeholder,p.volume),row.id).toEqual({m:"volume",v:"placeholder"});}
   else{expect(()=>port.swap(join(p.directory,"m"),join(p.directory,"v")),row.id).toThrow("Directory flush failed (Win32 error "+row.error+")");expect(createNativeDirectoryJournaledPortV1(simulatedHost().host).recover(p.directory),row.id).toBe("rolled-back");expect(holder(p.directory,p.placeholder,p.volume),row.id).toEqual({m:"placeholder",v:"volume"});}
  }
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the full partition lifecycle runs over the journaled port and refuses to derive a state while a swap is interrupted",()=>{
 const base=scratch();try{
  const directory=join(base,"life");mkdirSync(join(directory,"m"),{recursive:true});const{host}=simulatedHost(),port=createNativeDirectoryJournaledPortV1(host);
  allocateNativeDirectoryV1({directory,filesystemBytes:33554432,maximumPhysicalBytes:34603008},()=>true,port);
  expect(observeNativeDirectoryStateV1(directory,port)).toBe("inactive");activateNativeDirectoryV1(directory,port);expect(observeNativeDirectoryStateV1(directory,port)).toBe("attached");writeFileSync(join(directory,"m","journal"),"payload");
  const crashing=createNativeDirectoryJournaledPortV1(simulatedHost({crash:{call:3,when:"after"}}).host);
  expect(()=>deactivateNativeDirectoryV1(directory,crashing)).toThrow(Crash);expect(nativeDirectorySwapPendingV1(directory,port)).toBe(true);
  expect(()=>observeNativeDirectoryStateV1(directory,port)).toThrow(protocol.refusals.interrupted);expect(()=>activateNativeDirectoryV1(directory,port)).toThrow(protocol.refusals.interrupted);
  expect(recoverNativeDirectoryV1(directory,port)).toBe("completed");expect(observeNativeDirectoryStateV1(directory,port)).toBe("inactive");expect(readFileSync(join(directory,"v","journal"),"utf8")).toBe("payload");
  activateNativeDirectoryV1(directory,port);expect(readFileSync(join(directory,"m","journal"),"utf8")).toBe("payload");expect(names(directory)).toEqual(["fixed.dir","m","v"]);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("exchangeNativeDirectoriesV1 verifies the swap through the journaled port and flushes the parent",()=>{
 const base=scratch();try{
  const p=partition(base,"p"),{host,log}=simulatedHost(),after=exchangeNativeDirectoriesV1(join(p.directory,"m"),join(p.directory,"v"),createNativeDirectoryJournaledPortV1(host));
  expect(after.first).toEqual(p.volume);expect(after.second).toEqual(p.placeholder);expect(log.at(-1)).toBe("flush:p");expect(lstatSync(join(p.directory,"m")).isDirectory()).toBe(true);
  const lying=createNativeDirectoryJournaledPortV1(simulatedHost().host),p2=partition(base,"lying");
  expect(()=>exchangeNativeDirectoriesV1(join(p2.directory,"m"),join(p2.directory,"v"),{...lying,swap:()=>{}})).toThrow("Ambiguous directory exchange");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a sharing violation on the first rename is retried with a bounded backoff, any other error is not",()=>{
 const base=scratch();try{
  const retry=protocol.swap.retry;
  for(const error of[32,5]){
   const p=partition(base,"retry-"+error),{host}=simulatedHost({failMoves:[1,2],error}),slept:number[]=[];
   const result=moveNativeDirectoryWithRetryV1(host,join(p.directory,"m"),join(p.directory,protocol.swap.temporary),ms=>slept.push(ms));
   expect(result.ok,String(error)).toBe(true);expect(slept).toEqual([retry.backoffMilliseconds,retry.backoffMilliseconds*2]);
  }
  const stuck=partition(base,"stuck"),{host:always}=simulatedHost({failMoves:Array.from({length:20},(_,index)=>index+1),error:32}),sleeps:number[]=[];
  expect(moveNativeDirectoryWithRetryV1(always,join(stuck.directory,"m"),join(stuck.directory,protocol.swap.temporary),ms=>sleeps.push(ms))).toEqual({ok:false,error:32});expect(sleeps.length).toBe(retry.attempts-1);
  const absent=partition(base,"absent"),{host:missing}=simulatedHost({failMoves:[1],error:2}),none:number[]=[];
  expect(moveNativeDirectoryWithRetryV1(missing,join(absent.directory,"m"),join(absent.directory,protocol.swap.temporary),ms=>none.push(ms)).ok).toBe(false);expect(none).toEqual([]);
 }finally{rmSync(base,{recursive:true,force:true});}
});
