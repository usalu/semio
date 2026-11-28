import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {hostBootAtV1,prepare,readGrants} from "../🎫️gate/📖️status/🟦️.ts";
import {newHostCapacityIdV1,withHostCapacityV1} from "../🎫️gate/🟦️.ts";
import {readHostCapacityPolicyV1} from "../📏️decision/🟦️.ts";
import fixture from "./📊️ledger-update.json";
import retained from "./📊️retained.json";
import buildPolicy from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";

const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity);

test("an unfamiliar ledger row is isolated as a held reservation of its live owner and only proven stale rows are reclaimed",()=>{
 const db=new Database(":memory:"),now=Date.now();
 try{
  const oracle=db.query("select case when ?1 is null then ?3 when ?1 and ?2 then 1 else 0 end as held");
  for(const item of fixture.cases){
   const directory=prepare(mkdtempSync(join(tmpdir(),"capacity-epoch-")));
   try{
    const boot=hostBootAtV1(directory,now),file=join(directory,"grants",fixture.grant.id+".json");
    const row={...fixture.grant,pid:item.live||item.shape==="reused"?process.pid:2147483647,admittedAtMs:now,bootAtMs:item.boot==="current"?boot:boot-3600000,...item.shape==="bad-charge"?{reservedMegabytes:"unknown"}:{}};
    writeFileSync(file,item.shape==="unreadable"?"{":JSON.stringify(row));
    if(item.age==="old")utimesSync(file,new Date(now-policy.wait.staleMilliseconds-1000),new Date(now-policy.wait.staleMilliseconds-1000));
    expect(oracle.get(item.live===null?null:Number(item.live),Number(item.boot==="current"),Number(item.age==="fresh")),item.name).toEqual({held:Number(item.held)});
    const startOf=item.shape==="reused"?()=>now+6000:undefined,rows=readGrants(directory,now,policy,startOf);
    expect(rows.map(grant=>grant.reservedMegabytes),item.name).toEqual(item.held?[item.megabytes]:[]);
    if(item.held)expect(rows[0]!.foreign,item.name).toMatchObject({format:null,relation:"unversioned",declared:item.megabytes===6144});
    expect(existsSync(file),item.name).toBe(Boolean(item.held));
   }finally{rmSync(directory,{recursive:true,force:true});}
  }
 }finally{db.close();}
});

test("a waiter behind a live grant of another format is admitted beside it, names its pid and format in both languages and never waits for it",async()=>{
 const directory=prepare(mkdtempSync(join(tmpdir(),"capacity-epoch-"))),now=Date.now();
 try{
  const file=join(directory,"grants",fixture.grant.id+".json");
  writeFileSync(file,JSON.stringify({...fixture.grant,pid:process.pid,bootAtMs:hostBootAtV1(directory,now),admittedAtMs:now}));
  const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:async()=>undefined};
  const sample=async()=>({host:{...retained.host,version:1 as const,platform:"darwin" as const,pressure:"normal" as const,availableMegabytes:12000,atMs:Date.now()},swappedOutMegabytes:0});
  const fast=readHostCapacityPolicyV1({...policy,wait:{...policy.wait,pollMilliseconds:10,progressMilliseconds:20}});
  for(const [language,words] of [["en_US.UTF-8",["older format","none recorded",`pid ${process.pid}`]],["de_DE.UTF-8",["älteres Format","keines vermerkt",`pid ${process.pid}`]]] as const){
   const lines:string[]=[],request={...retained.cargo.request,id:newHostCapacityIdV1(),priority:"interactive" as const};
   let ran=false;
   await withHostCapacityV1({directory,policy:fast,control,request,environment:{LANG:language},write:(line:string)=>lines.push(line),sample},async()=>{ran=true;});
   expect(ran,language).toBe(true);
   const warning=lines.filter(line=>line.includes(fixture.grant.id));
   expect(warning,language).toHaveLength(1);
   for(const word of words)expect(warning[0],language).toContain(word);
   expect(existsSync(file),"the foreign row is left to its owner").toBe(true);
   expect(readdirSync(join(directory,"queue"))).toEqual([]);
  }
  expect(readGrants(directory,Date.now(),policy,undefined,undefined,false).map(grant=>grant.foreign?.relation)).toEqual(["unversioned"]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("holder heartbeats stay current even while the accounting scan is blocked",async()=>{
 const directory=prepare(mkdtempSync(join(tmpdir(),"capacity-epoch-"))),now=Date.now();
 const fast=readHostCapacityPolicyV1({...policy,wait:{...policy.wait,heartbeatMilliseconds:20}});
 try{
  const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:async()=>undefined};
  const request={...retained.cargo.request,id:newHostCapacityIdV1(),priority:"interactive" as const};
  const sample=async()=>({host:{...retained.host,version:1 as const,platform:"darwin" as const,pressure:"normal" as const,availableMegabytes:12000,atMs:Date.now()},swappedOutMegabytes:0});
  await withHostCapacityV1({directory,policy:fast,control,request,environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample},async handle=>{
   const file=join(directory,"grants",fixture.grant.id+".json"),own=join(directory,"grants",handle.id+".json");
   writeFileSync(file,JSON.stringify({...fixture.grant,pid:process.pid,bootAtMs:hostBootAtV1(directory,now),admittedAtMs:now}));
   const previous=readFileSync(own,"utf8"),old=new Date(now-fast.wait.staleMilliseconds-1000);
   utimesSync(own,old,old);
   for(let attempt=0;attempt<40&&statSync(own).mtimeMs<=old.getTime();attempt++)await Bun.sleep(10);
   expect(statSync(own).mtimeMs).toBeGreaterThanOrEqual(now);
   expect(readFileSync(own,"utf8")).toBe(previous);
   expect(existsSync(file)).toBe(true);
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
});
