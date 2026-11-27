import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdir,mkdtemp,readdir,readFile,rename,rm,truncate,utimes,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join,relative,sep} from "node:path";
import {fileURLToPath} from "node:url";
import cases from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {buildGraphWriterOptionalPathAdmittedV1,isBuildGraphOptionalCachePathV1} from "../../../../📥️inputs/💾️storage/🟨️.mjs";
import {optionalAcceleratorV1,optionalEntryOfV1,retireBuildGraphOptionalGroupsLruV1} from "../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts";
import {openOwnerCheckpointV1} from "../🟦️.ts";
import {observeOwnerFactsMemoizedV1} from "../../🟦️.ts";
import type * as observationV1 from "../../../../👁️observation/🟨️.mjs";
import {readAdmissionMemoV1} from "../../🧠️admission-memo/🟦️.ts";

const digest=(letter:string)=>letter.repeat(64);
const suffix=(index:number)=>{
 const {owner,targets,inputs}=cases.questions[index]!;
 return owner+"\0"+JSON.stringify([targets,inputs]);
};
const result=(index:number,revision:number)=>({
 facts:{project:{name:cases.questions[index]!.owner,targets:{}},files:["manifest-"+index],dependencies:[]},
 rows:[{kind:"exists"as const,path:"/root/manifest-"+index,fact:revision===1}],
});
const observed=(rows:readonly observationV1.SourceObservationV1[],retained:()=>void):Pick<typeof observationV1,"captureObservedSourceInputsV1"|"retainObservedRowsV1">=>({captureObservedSourceInputsV1:async<T>(observe:()=>T|PromiseLike<T>)=>({value:await observe(),observations:[...rows],authorityObservations:[]}),retainObservedRowsV1:retained});
const port=(signal?:AbortSignal)=>({
 check:()=>signal?.throwIfAborted(),
 yieldContinuation:async()=>signal?.throwIfAborted(),
 publish:(_paths:readonly string[],_plan:unknown,write:()=>void)=>{write();return true;},
});
const fixture=async(body:(directory:string)=>Promise<void>)=>{
 const directory=await mkdtemp(join(tmpdir(),"owner-checkpoint-"));
 try{await body(directory);}finally{await rm(directory,{recursive:true,force:true});}
};

test("an interrupted discovery lends only exact questions holding current observations",()=>fixture(async directory=>{
 const writer=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
 for(const [index,question] of cases.questions.entries())writer.save(suffix(index),result(index,question.revision));
 writer.flush();
 const reopened=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
 const revisions=cases.questions.map(({revision})=>revision);
 for(const {question,revision} of cases.changes)revisions[question]=revision;
 const reused:number[]=[],discovered:number[]=[];
 for(const index of revisions.keys()){
  const capture=result(index,revisions[index]!);
  await observeOwnerFactsMemoizedV1({
   key:cases.scope+"\0"+cases.format+"\0"+suffix(index),
   recover:async()=>reopened.read(suffix(index)),
   persist:entry=>reopened.save(suffix(index),entry),
   hold:async rows=>JSON.stringify(rows)===JSON.stringify(capture.rows),
   observation:observed(capture.rows,()=>{reused.push(index);}),
   observe:async()=>{discovered.push(index);return capture.facts;},
  });
 }
 reopened.flush();
 expect({reused,discovered}).toEqual(cases.expected);
 const database=new Database(":memory:");
 try{
  const oracle=database.query(`SELECT old.key AS question FROM json_each(?) old JOIN json_each(?) actual ON old.key=actual.key WHERE json_extract(old.value,'$.revision')=actual.value ORDER BY old.key`).all(JSON.stringify(cases.questions),JSON.stringify(revisions));
  expect(reused).toEqual(oracle.map(row=>(row as {question:number}).question));
 }finally{database.close();}
 const repaired=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
 expect(await repaired.read(suffix(1))).toEqual(result(1,2));
 expect(await repaired.read("owner-a\0"+JSON.stringify([["preview"],[]]))).toBeUndefined();
}));

test("foreign scopes, formats, corrupt values and interrupted temporary writes are misses",()=>fixture(async directory=>{
 const writer=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
 writer.save(suffix(0),result(0,1));
 writer.save(suffix(1),result(1,1));writer.flush();
 const names=(await readdir(directory)).sort();
 expect(names.length).toBe(2);
 const path=join(directory,names[0]!);
 const bytes=await readFile(path);bytes[bytes.length-1]^=1;await writeFile(path,bytes);
 await writeFile(join(directory,"02.pack.tmp"),bytes);
 const read=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
 expect(await read.read(suffix(0))).toBeUndefined();
 expect(await read.read(suffix(1))).toEqual(result(1,1));
 expect(await (await openOwnerCheckpointV1(directory,digest("c"),cases.format,port())).read(suffix(1))).toBeUndefined();
 expect(await (await openOwnerCheckpointV1(directory,cases.scope,digest("c"),port())).read(suffix(1))).toBeUndefined();
 expect(await readAdmissionMemoV1(path,cases.scope,()=>true)).toBeUndefined();
 const abort=new AbortController();abort.abort(Error("fixture cancelled"));
 await expect(openOwnerCheckpointV1(directory,cases.scope,cases.format,port(abort.signal))).rejects.toThrow("fixture cancelled");
}));

test("a killed worker leaves completed packs and drops only its unflushed tail",()=>fixture(async directory=>{
 const child=Bun.spawn([process.execPath,fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),directory],{
  stdout:"pipe",stderr:"inherit",
 });
 try{
  const stream=child.stdout.getReader();
  try{expect(new TextDecoder().decode((await stream.read()).value)).toBe("ready\n");}finally{stream.releaseLock();}
 }finally{
  child.kill();
  await child.exited;
 }
 const resumed=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
 expect(await resumed.read(suffix(0))).toEqual(result(0,1));
 expect(await resumed.read(suffix(1))).toBeUndefined();
}));

test("alternating scoped writers preserve each other and the cache owns both pack and temporary",()=>fixture(async directory=>{
 const scopeA=cases.scope,scopeB=digest("d");
 const owned=join(directory,"inference"),selections=join(owned,"selections");
 const scoped=(scope:string)=>join(selections,scope,"oc");
 const checked=(scope:string)=>({
  ...port(),
  publish:(paths:readonly string[],plan:{peakBytes:number;retainedDeltaBytes:number},write:()=>void)=>{
   for(const path of paths){
    const name=relative(owned,path).split(sep).join("/");
    expect(isBuildGraphOptionalCachePathV1(name)).toBe(true);
    expect(buildGraphWriterOptionalPathAdmittedV1({kind:"selection",identity:scope},name)).toBe(true);
    expect(buildGraphWriterOptionalPathAdmittedV1({kind:"selection",identity:scope===scopeA?scopeB:scopeA},name)).toBe(false);
    const local=name.slice(("selections/"+scope+"/").length);
    expect(optionalAcceleratorV1(local)).toBe(true);
    expect(optionalEntryOfV1(local)).toBe("oc/00.pack");
   }
   expect(plan.peakBytes).toBe(plan.retainedDeltaBytes);
   write();
   return true;
  },
 });
 for(const scope of [scopeA,scopeB])await mkdir(dirname(scoped(scope)),{recursive:true});
 (await openOwnerCheckpointV1(scoped(scopeA),scopeA,cases.format,checked(scopeA))).save(suffix(0),result(0,1));
 (await openOwnerCheckpointV1(scoped(scopeB),scopeB,cases.format,checked(scopeB))).save(suffix(0),result(0,2));
 expect(await (await openOwnerCheckpointV1(scoped(scopeA),scopeA,cases.format,checked(scopeA))).read(suffix(0))).toEqual(result(0,1));
 expect(await (await openOwnerCheckpointV1(scoped(scopeB),scopeB,cases.format,checked(scopeB))).read(suffix(0))).toEqual(result(0,2));
 expect(isBuildGraphOptionalCachePathV1("oc/00.pack")).toBe(false);
 const pack=join(scoped(scopeA),"00.pack"),temporary=pack+".tmp",old=new Date(Date.now()-3600000);
 await writeFile(temporary,"interrupted pack");
 await Promise.all([utimes(pack,old,old),utimes(temporary,old,old)]);
 const groups:string[]=[];
 const report=await retireBuildGraphOptionalGroupsLruV1(directory,{lease:async(kind,identity)=>{
  groups.push(kind+"/"+identity);
  return {release:()=>{}};
 }},{cancelled:()=>false,progress:()=>{}},{targetBytes:0,exclude:["selections/"+scopeB]});
 expect(groups).toEqual(["selection/"+scopeA]);
 expect(report).toMatchObject({retiredGroups:1,retiredFiles:2});
 expect(await (await openOwnerCheckpointV1(scoped(scopeA),scopeA,cases.format,checked(scopeA))).read(suffix(0))).toBeUndefined();
 expect(await (await openOwnerCheckpointV1(scoped(scopeB),scopeB,cases.format,checked(scopeB))).read(suffix(0))).toEqual(result(0,2));
}));

test("a full journal rolls its oldest slot and oversized leaves are ordinary misses",()=>fixture(async directory=>{
 const writer=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
 writer.save(suffix(0),result(0,1));
 writer.save(suffix(1),result(1,1));writer.flush();
 const [first,second]=(await readdir(directory)).sort();
 const bytes=await readFile(join(directory,second!)),newline=bytes.indexOf(10);
 const header=JSON.parse(bytes.subarray(0,newline).toString("utf8"));
 header.sequence=schema.$defs.Limits.const.slots-1;
 await rename(join(directory,second!),join(directory,"ff.pack"));
 await writeFile(join(directory,"ff.pack"),Buffer.concat([Buffer.from(JSON.stringify(header)+"\n"),bytes.subarray(newline+1)]));
 const reopened=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
 reopened.save(suffix(2),result(2,1));
 expect(await reopened.read(suffix(0))).toBeUndefined();
 expect(await reopened.read(suffix(1))).toEqual(result(1,1));
 expect(await reopened.read(suffix(2))).toEqual(result(2,1));
 expect((await readdir(directory)).sort()).toEqual([first!,"ff.pack"]);

 await truncate(join(directory,"ff.pack"),schema.$defs.Limits.const.maximumPackBytes+1);
 expect(await (await openOwnerCheckpointV1(directory,cases.scope,cases.format,port())).read(suffix(1))).toBeUndefined();
}));

test("a stale complete memo can borrow a newer durable owner repair",()=>fixture(async directory=>{
 const scope=digest("e"),key=scope+"\0"+cases.format+"\0"+suffix(0),stale=result(0,1),fresh=result(0,2);
 const observation=observed(stale.rows,()=>{});
 await observeOwnerFactsMemoizedV1({key,observation,hold:async()=>true,observe:async()=>stale.facts});
 (await openOwnerCheckpointV1(directory,scope,cases.format,port())).save(suffix(0),fresh);
 const checkpoint=await openOwnerCheckpointV1(directory,scope,cases.format,port());
 expect(await observeOwnerFactsMemoizedV1({
  key,observation,recover:()=>checkpoint.read(suffix(0)),
  hold:async rows=>JSON.stringify(rows)===JSON.stringify(fresh.rows),
  observe:async()=>{throw Error("Durable repair should satisfy the stale memo question");},
 })).toEqual(fresh.facts);
}));

test("large owner facts and quota refusal do not publish bytes",()=>fixture(async directory=>{
 let requested=0;
 const checkpoint=await openOwnerCheckpointV1(directory,cases.scope,cases.format,{
  ...port(),
  publish:()=>{requested++;return false;},
 });
 const normal=result(0,1);
 checkpoint.save(suffix(0),{...normal,facts:{...normal.facts,files:["x".repeat(schema.$defs.Limits.const.maximumPackBytes+1)]}});
 expect(requested).toBe(0);
 checkpoint.save(suffix(0),normal);
 expect(requested).toBe(1);
 expect(await readdir(directory)).toEqual([]);
}));

test("a worker killed mid-discovery loses at most the declared flush interval of owners, so a restart resumes instead of starting from zero",()=>fixture(async directory=>{
 expect(schema.$defs.Limits.const.flushMilliseconds).toBeLessThanOrEqual(15000);
 const realNow=Date.now;let clock=realNow();Date.now=()=>clock;
 try{
  const writer=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
  writer.save(suffix(0),result(0,1));
  clock+=schema.$defs.Limits.const.flushMilliseconds;
  writer.save(suffix(1),result(1,1));
  const resumed=await openOwnerCheckpointV1(directory,cases.scope,cases.format,port());
  expect(await resumed.read(suffix(0))).toEqual(result(0,1));
  expect(await resumed.read(suffix(1))).toEqual(result(1,1));
 }finally{Date.now=realNow;}
}));
