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

test("an unfamiliar ledger row protects live commitments and only proven stale rows are reclaimed",()=>{
 const db=new Database(":memory:"),now=Date.now();
 try{
  const oracle=db.query("select case when ?1 is null then ?3 when ?1 and ?2 then 1 else 0 end as blocks");
  for(const item of fixture.cases){
   const directory=prepare(mkdtempSync(join(tmpdir(),"capacity-epoch-")));
   try{
    const boot=hostBootAtV1(directory,now),file=join(directory,"grants",fixture.grant.id+".json");
    const row={...fixture.grant,pid:item.live||item.shape==="reused"?process.pid:2147483647,admittedAtMs:now,bootAtMs:item.boot==="current"?boot:boot-3600000,...item.shape==="bad-charge"?{reservedMegabytes:"unknown"}:{}};
    writeFileSync(file,item.shape==="unreadable"?"{":JSON.stringify(row));
    if(item.age==="old")utimesSync(file,new Date(now-policy.wait.staleMilliseconds-1000),new Date(now-policy.wait.staleMilliseconds-1000));
    expect(oracle.get(item.live===null?null:Number(item.live),Number(item.boot==="current"),Number(item.age==="fresh")),item.name).toEqual({blocks:Number(item.blocks)});
    const startOf=item.shape==="reused"?()=>now+6000:undefined;
    if(item.blocks)expect(()=>readGrants(directory,now,policy,startOf),item.name).toThrow("Unrecognized host capacity grant");
    else expect(readGrants(directory,now,policy,startOf),item.name).toEqual([]);
    expect(existsSync(file),item.name).toBe(item.blocks);
   }finally{rmSync(directory,{recursive:true,force:true});}
  }
 }finally{db.close();}
});

test("an unrecognized live grant prevents an otherwise admissible command entering its body",async()=>{
 const directory=prepare(mkdtempSync(join(tmpdir(),"capacity-epoch-"))),now=Date.now();
 try{
  const file=join(directory,"grants",fixture.grant.id+".json");
  writeFileSync(file,JSON.stringify({...fixture.grant,pid:process.pid,bootAtMs:hostBootAtV1(directory,now),admittedAtMs:now}));
  const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:async()=>undefined};
  const request={...retained.cargo.request,id:newHostCapacityIdV1(),priority:"interactive" as const};
  const options={directory,policy,control,request,environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:{...retained.host,version:1 as const,platform:"darwin" as const,pressure:"normal" as const,availableMegabytes:12000,atMs:Date.now()},swappedOutMegabytes:0})};
  let ran=false;
  await expect(withHostCapacityV1(options,async()=>{ran=true;})).rejects.toThrow("Unrecognized host capacity grant");
  expect(()=>readGrants(directory,Date.now(),policy,undefined,undefined,false)).toThrow("Unrecognized host capacity grant");
  expect(ran).toBe(false);
  expect(existsSync(file)).toBe(true);
  expect(readdirSync(join(directory,"queue"))).toEqual([]);
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
