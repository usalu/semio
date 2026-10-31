import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {linkSync,mkdirSync,mkdtempSync,rmSync,symlinkSync,truncateSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {createNativeDirectoryMeterV1,nativeDirectoryProtocolV1} from "../../🟦️.ts";
import {createNativeDirectoryUsageV1,measureNativeDirectoryUsageAsyncV1,measureNativeDirectoryUsageV1,nativeDirectoryEntryBoundV1,type NativeDirectoryWalkPortV1} from "../🟦️.ts";

const protocol=nativeDirectoryProtocolV1(),oracle=new Ajv({strict:true});
const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"use-"));};
const sh=(...args:string[])=>Bun.spawnSync(args,{stdout:"pipe",stderr:"pipe"});
const python=`import os,stat,sys,json
seen=set();total={"allocatedBytes":0,"logicalBytes":0,"entries":0}
def walk(path):
    row=os.lstat(path);total["entries"]+=1
    if stat.S_ISREG(row.st_mode) and row.st_nlink>1:
        key=(row.st_dev,row.st_ino)
        if key in seen: return
        seen.add(key)
    total["allocatedBytes"]+=row.st_blocks*512
    if stat.S_ISREG(row.st_mode): total["logicalBytes"]+=row.st_size
    if stat.S_ISDIR(row.st_mode):
        for name in os.listdir(path): walk(os.path.join(path,name))
walk(sys.argv[1]);print(json.dumps(total))`;
const hasPython=testPythonV1()!==null;

test("the entry bound follows the envelope and never drops below the floor, as an independent python formula says",()=>{
 const check=oracle.compile({type:"object",additionalProperties:false,required:["physicalBytes","entries"],properties:{physicalBytes:{type:"integer",minimum:1},entries:{type:"integer",minimum:1}}});
 for(const row of fixture.bounds){
  expect(check(row),JSON.stringify(check.errors)).toBe(true);expect(nativeDirectoryEntryBoundV1(row.physicalBytes),String(row.physicalBytes)).toBe(row.entries);
  if(hasPython){const answer=sh(testPythonV1()??"python3","-c",`import math,sys;b=int(sys.argv[1]);print(max(${protocol.maximumEntries},math.ceil(b/${protocol.usage.blockBytes})*${protocol.usage.entryFactor}))`,String(row.physicalBytes));expect(Number(answer.stdout.toString())).toBe(row.entries);}
 }
 expect(()=>nativeDirectoryEntryBoundV1(0)).toThrow("Positive physical envelope");
 expect(schema.definitions.Protocol.const.usage.concurrency).toBeGreaterThan(0);
});

test("the asynchronous walk equals the synchronous walk, an independent python walk and du on trees with hard links, links and holes",async()=>{
 const base=scratch();try{
  for(const row of fixture.trees){
   const root=join(base,row.id);mkdirSync(root);
   for(const name of row.directories)mkdirSync(join(root,name),{recursive:true});
   for(const[path,size]of row.files)writeFileSync(join(root,String(path)),Buffer.alloc(size as number,1));
   const generated=(row as {generated?:{directory:string;count:number;bytes:number}}).generated;
   if(generated)for(let index=0;index<generated.count;index++)writeFileSync(join(root,generated.directory,"f"+index),Buffer.alloc(generated.bytes,2));
   for(const[from,to]of row.hardLinks)linkSync(join(root,from!),join(root,to!));
   for(const[target,path]of row.symlinks)symlinkSync(target!.startsWith("/")?target!:join(root,target!),join(root,path!));
   for(const[path,size]of row.sparse){writeFileSync(join(root,String(path)),"");truncateSync(join(root,String(path)),Number(size));}
   const parallel=await measureNativeDirectoryUsageAsyncV1(root),serial=measureNativeDirectoryUsageV1(root);
   expect(parallel,row.id).toEqual(serial);
   if(process.platform!=="win32"){const du=Number(sh("du","-sk",root).stdout.toString().split(/\s/u)[0]);expect(Math.ceil(parallel.allocatedBytes/1024),row.id).toBe(du);}
   if(hasPython)expect(JSON.parse(sh(testPythonV1()??"python3","-c",python,root).stdout.toString()),row.id).toEqual({allocatedBytes:parallel.allocatedBytes,logicalBytes:parallel.logicalBytes,entries:parallel.entries});
  }
  const wide=await measureNativeDirectoryUsageAsyncV1(join(base,"wide"));expect(wide.entries).toBe(602);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the walk refuses at the entry bound, accepts exactly the bound and counts a vanished entry as freed space",async()=>{
 const base=scratch();try{
  const root=join(base,"t");mkdirSync(root);for(let index=0;index<30;index++)writeFileSync(join(root,"f"+index),"x");
  await expect(measureNativeDirectoryUsageAsyncV1(root,{maximumEntries:30})).rejects.toThrow("entry bound");
  expect((await measureNativeDirectoryUsageAsyncV1(root,{maximumEntries:31})).entries).toBe(31);
  const stats=new Map<string,Awaited<ReturnType<NativeDirectoryWalkPortV1["stat"]>>>(),port:NativeDirectoryWalkPortV1={
   list:async path=>path===root?["gone","kept"]:[],
   stat:async path=>{const row=stats.get(path);if(!row){const error=Object.assign(Error("gone"),{code:"ENOENT"});throw error;}return row;}
  };
  const file=(blocks:number)=>({isFile:()=>true,isDirectory:()=>false,nlink:1n,blocks:BigInt(blocks),size:BigInt(blocks*512),dev:1n,ino:BigInt(blocks)});
  stats.set(resolve(root),{...file(8),isFile:()=>false,isDirectory:()=>true});stats.set(join(root,"kept"),file(8));
  const row=await measureNativeDirectoryUsageAsyncV1(root,{port});expect(row).toEqual({allocatedBytes:8192,logicalBytes:4096,entries:2});
  await expect(measureNativeDirectoryUsageAsyncV1(join(base,"absent"))).rejects.toThrow("ENOENT");
  expect(measureNativeDirectoryUsageV1(root,31).entries).toBe(31);expect(()=>measureNativeDirectoryUsageV1(root,30)).toThrow("entry bound");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the walk stops on cancellation and does not block the event loop while it runs",async()=>{
 const base=scratch();try{
  const root=join(base,"big");mkdirSync(root);for(let directory=0;directory<20;directory++){mkdirSync(join(root,"d"+directory));for(let file=0;file<250;file++)writeFileSync(join(root,"d"+directory,"f"+file),"y");}
  let ticks=0,running=true;const spin=()=>{if(!running)return;ticks++;setImmediate(spin);};setImmediate(spin);
  const row=await measureNativeDirectoryUsageAsyncV1(root);running=false;
  expect(row.entries).toBe(1+20+5000);expect(ticks,"the loop must run between filesystem batches").toBeGreaterThan(20);
  const controller=new AbortController();let calls=0;
  const port:NativeDirectoryWalkPortV1={list:async()=>Array.from({length:400},(_,index)=>"n"+index),stat:async()=>{calls++;if(calls===20)controller.abort(new Error("stop now"));return {isFile:()=>false,isDirectory:()=>false,nlink:1n,blocks:0n,size:0n,dev:1n,ino:BigInt(calls)};}};
  const first={isFile:()=>false,isDirectory:()=>true,nlink:1n,blocks:0n,size:0n,dev:1n,ino:0n};
  const guarded:NativeDirectoryWalkPortV1={list:port.list,stat:async path=>path===resolve(root)?first:port.stat(path)};
  await expect(measureNativeDirectoryUsageAsyncV1(root,{signal:controller.signal,port:guarded})).rejects.toThrow("stop now");
  expect(calls,"no new filesystem calls are issued long after the signal").toBeLessThan(20+protocol.usage.concurrency*protocol.usage.batch+protocol.usage.batch);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("simultaneous measurements are coalesced: any number of callers cost at most two walks and no caller sees a walk that began before its own write",async()=>{
 const flightSchema=oracle.compile({type:"object",additionalProperties:false,required:["id","diskAtStart","script","starts","results"],properties:{id:{type:"string"},diskAtStart:{type:"integer"},script:{type:"array",items:{type:"string",pattern:"^(arrive:[A-Z]|abort:[A-Z]|write:[0-9]+|finish|fail)$"}},starts:{type:"integer"},aborts:{type:"integer"},results:{type:"object",additionalProperties:{anyOf:[{type:"integer"},{enum:["error","aborted"]}]}}}});
 for(const row of fixture.flights){
  expect(flightSchema(row),row.id+JSON.stringify(flightSchema.errors)).toBe(true);
  let disk=row.diskAtStart,starts=0,aborts=0;const live:{value:number;settle:(ok:boolean)=>void}[]=[],controllers=new Map<string,AbortController>(),results=new Map<string,Promise<number>>();
  const cell=createNativeDirectoryUsageV1({root:"/unused",maximumEntries:10,measure:(_,signal)=>new Promise<number>((done,fail)=>{starts++;const value=disk,entry={value,settle:(ok:boolean)=>ok?done(value):fail(new Error("walk failed"))};live.push(entry);signal.addEventListener("abort",()=>{aborts++;live.splice(live.indexOf(entry),1);fail(signal.reason);},{once:true});})});
  for(const step of row.script){
   const[kind,arg]=step.split(":");
   if(kind==="arrive"){const controller=new AbortController();controllers.set(arg!,controller);const call=cell.fresh(controller.signal);call.catch(()=>undefined);results.set(arg!,call);}
   else if(kind==="write")disk+=Number(arg);
   else if(kind==="abort")controllers.get(arg!)!.abort(new Error("caller left"));
   else{const entry=live.shift();expect(entry,row.id+" has a walk to settle").toBeDefined();entry!.settle(kind==="finish");}
   await new Promise(done=>setTimeout(done,2));
  }
  const outcome:Record<string,number|string>={};
  for(const[name,call]of results)outcome[name]=await call.then(value=>value,error=>(error as Error).message==="caller left"?"aborted":"error");
  expect(outcome,row.id).toEqual(row.results as Record<string,number|string>);expect(starts,row.id).toBe(row.starts);expect(aborts,row.id).toBe((row as {aborts?:number}).aborts??0);
 }
});

test("a recent measurement is served from the cell and a recorded number never moves backwards in time",async()=>{
 let clock=1000,walks=0;const cell=createNativeDirectoryUsageV1({root:"/unused",maximumEntries:10,now:()=>clock,measure:async()=>{walks++;return 42;}});
 expect(cell.peek()).toBeUndefined();expect(await cell.recent(5000)).toBe(42);expect(walks).toBe(1);
 clock+=4000;expect(await cell.recent(5000)).toBe(42);expect(walks).toBe(1);
 clock+=2000;expect(await cell.recent(5000)).toBe(42);expect(walks).toBe(2);
 cell.record(7,clock-50);expect(cell.peek()!.bytes).toBe(42);cell.record(7,clock+1);expect(cell.peek()!.bytes).toBe(7);
});

test("the capacity meter shares one walk between simultaneous commits and tolerates a cancelled waiter",async()=>{
 let walks=0,disk=0;const cell=createNativeDirectoryUsageV1({root:"/unused",maximumEntries:10,measure:async()=>{walks++;await new Promise(done=>setTimeout(done,20));return disk;}});
 const meter=createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:33554432,maximumPhysicalBytes:34603008,usage:cell});
 disk=1048576;const commits=Array.from({length:6},()=>meter.admit({mode:"commit",requestBytes:0}));await Promise.all(commits);expect(walks).toBeLessThanOrEqual(2);
 disk=67108864;await expect(meter.admit({mode:"commit",requestBytes:0})).rejects.toThrow(protocol.refusals.capacity);
 const controller=new AbortController(),waiting=meter.admit({mode:"commit",requestBytes:0},controller.signal);controller.abort(new Error("caller left"));await expect(waiting).rejects.toThrow("caller left");
 disk=0;await meter.admit({mode:"commit",requestBytes:0});
});

import {Database} from "bun:sqlite";
test("accepted stat failures drain before exact primary rejection escapes the actual walker",async()=>{
 const database=new Database(":memory:");try{for(const row of fixture.drain){
  const root=resolve("walk-model"),primary=row.reason==="undefined"?undefined:Error("exact accepted stat failure"),controller=new AbortController();let release!:()=>void,entered!:()=>void,completed=0,returned=false,rejected=false,actual:unknown;
  const pending=new Promise<void>(yes=>{release=yes;}),started=new Promise<void>(yes=>{entered=yes;}),directory={isFile:()=>false,isDirectory:()=>true,nlink:1n,blocks:0n,size:0n,dev:1n,ino:1n},file={...directory,isDirectory:()=>false,isFile:()=>true,blocks:1n,size:1n};
  const port:NativeDirectoryWalkPortV1={list:async()=>["failed","pending"],stat:async path=>{if(path===root)return directory;if(path===join(root,"failed")){completed++;throw primary;}entered();await pending;completed++;return file;}};
  const work=measureNativeDirectoryUsageAsyncV1(root,{port,signal:controller.signal}).then(value=>{returned=true;actual=value;},error=>{returned=true;rejected=true;actual=error;});
  try{await started;await new Promise<void>(done=>setImmediate(done));const eligible=(database.query("select ? = ? as settled").get(completed,row.accepted) as {settled:number}).settled===1;expect(eligible,row.id).toBe(false);expect(returned,row.id+" must retain every accepted stat").toBe(eligible);}finally{release();await work;}
  expect(completed,row.id).toBe(row.accepted);expect(rejected,row.id).toBe(true);expect(actual,row.id).toBe(primary);expect((database.query("select ? = ? as settled").get(completed,row.accepted) as {settled:number}).settled).toBe(1);
 }}finally{database.close();}
});

import {readFileSync} from "node:fs";
import {posix,win32} from "node:path";
import ts from "typescript";
import {transformSync} from "esbuild";
import {evaluateExtractedFunctionV1} from "../../../../../../🧪️testing/🧩️extracted-function/🟦️.ts";
test("the actual walk admits atomic entry names before child stat effects",async()=>{
 const root=resolve("entry-model"),policy=protocol.usage.entry,host=process.platform==="win32"?"win32":"linux";
 for(const row of fixture.entryNames.filter(row=>row.platform===host)){
  const calls:string[]=[],directory={isFile:()=>false,isDirectory:()=>true,nlink:1n,blocks:0n,size:0n,dev:1n,ino:1n},file={...directory,isDirectory:()=>false,isFile:()=>true};let actual:unknown;
  const port:NativeDirectoryWalkPortV1={list:async()=>[row.name],stat:async path=>{if(path===root)return directory;calls.push(path);return file;}};
  try{await measureNativeDirectoryUsageAsyncV1(root,{port,maximumEntries:3});}catch(error){actual=error;}
  if(row.valid){expect(actual,row.name).toBeUndefined();expect(calls).toEqual([join(root,row.name)]);}else{expect(actual,row.name).toBeInstanceOf(Error);expect((actual as Error).message,row.name).toBe(policy.refusal);expect(calls,row.name).toEqual([]);}
 }
});
test("the private atomic append equals independent POSIX and Windows path oracles without Unicode normalization",()=>{
 const source=ts.createSourceFile("usage.ts",readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),node=source.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="appendNativeDirectoryEntryV1");expect(node,"actual fast append required").toBeDefined();
 const append=evaluateExtractedFunctionV1(transformSync(node!.getText(source),{loader:"ts",format:"cjs"}).code,"appendNativeDirectoryEntryV1",{protocol});
 for(const row of fixture.entryNames){const system=row.platform==="win32"?win32:posix,parent=system.resolve(row.parent),prefix=parent.endsWith(system.sep)?parent:parent+system.sep;if(row.valid)expect(append(prefix,row.name,row.platform==="win32")).toBe(system.join(parent,row.name));else expect(()=>append(prefix,row.name,row.platform==="win32")).toThrow(protocol.usage.entry.refusal);}
 const compose=fixture.entryNames.filter(row=>row.name==="é"||row.name==="é").map(row=>append("/data/",row.name,false));expect(compose[0]).not.toBe(compose[1]);
});
test("the system walk preserves exact scalar metadata and required precision observations without ignored errors",async()=>{
 const source=ts.createSourceFile("usage.ts",readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),node=source.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="systemNativeDirectoryWalkV1");expect(node,"actual exact metadata system port required").toBeDefined();
 for(const row of fixture.metadata){
  const calls:string[]=[],stop=new AbortController(),failure=Error("exact metadata failure"),abort=Error("metadata preparation cancelled"),numeric:{dev:number;ino:number;nlink:number;blocks:number;size:number}&Record<string,number>={dev:1,ino:2,nlink:1,blocks:8,size:4};numeric[row.field]=row.value;const flags={isFile:()=>true,isDirectory:()=>false},number={...flags,...numeric},exact={...flags,dev:1n,ino:9007199254740993n,nlink:1n,blocks:8n,size:4n};
  const factory=evaluateExtractedFunctionV1(transformSync(node!.getText(source),{loader:"ts",format:"cjs"}).code,"systemNativeDirectoryWalkV1",{protocol,listEntries:async()=>[],statEntry:async(_path:unknown,options?:{bigint?:boolean})=>{calls.push(options?.bigint?"bigint":"number");if(row.kind==="initial-error"||row.kind==="exact-error"&&options?.bigint)throw failure;if(row.kind==="cancel")stop.abort(abort);return options?.bigint?exact:number;}}),port:NativeDirectoryWalkPortV1=factory(stop.signal);let observed:unknown,caught:unknown;
  try{observed=await port.stat("first");}catch(error){caught=error;}
  if(row.kind==="initial-error"||row.kind==="exact-error"||row.kind==="cancel"){expect(caught,row.id).toBe(row.kind==="cancel"?abort:failure);expect(calls,row.id).toEqual(row.kind==="exact-error"?["number","bigint"]:["number"]);continue;}
  expect(caught,row.id).toBeUndefined();expect(observed,row.id).toBe(row.kind==="safe"?number:exact);expect(await port.stat("next"),row.id).toBe(row.kind==="safe"?number:exact);expect(calls,row.id).toEqual(row.kind==="safe"?["number","number"]:["number","bigint","bigint"]);
  if(row.kind==="safe")for(const field of ["dev","ino","nlink","blocks","size"])expect(BigInt((observed as Record<string,number>)[field]!),row.id).toBe(BigInt(numeric[field]!));
 }
});

test("actual system stat observations replay neutral files against independent Python metadata",async()=>{
 const root=scratch();try{const files=fixture.trees[0]!.files.filter(([name])=>!String(name).includes("/"));for(const[name,bytes]of files)writeFileSync(join(root,String(name)),Buffer.alloc(Number(bytes)));const actual=await measureNativeDirectoryUsageAsyncV1(root);expect(actual.entries).toBe(files.length+1);expect(actual.logicalBytes).toBe(files.reduce((sum,[,bytes])=>sum+Number(bytes),0));expect(hasPython).toBe(true);const answer=sh(testPythonV1()??"python3","-c",python,root);expect(answer.exitCode,answer.stderr.toString()).toBe(0);expect(actual).toEqual(JSON.parse(answer.stdout.toString()));}finally{rmSync(root,{recursive:true,force:true});}
});
