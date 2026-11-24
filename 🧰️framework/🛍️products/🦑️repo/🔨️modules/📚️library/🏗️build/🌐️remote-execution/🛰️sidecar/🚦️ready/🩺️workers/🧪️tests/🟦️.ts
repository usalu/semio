import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {READY_OFFLINE_SECONDS_V1,readReadyWorkersV1,readyRecentRunsV1} from "../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 🩺️ Contract: the fleet store says what each worker really is: a machine without any sign of life is offline whatever its last record claims, an origin that never wrote a record is absent. */
const P=schema.definitions.Policy.const,now=1_800_000_000_000;
const origin=(store:string,name:string,files:Readonly<Record<string,{content:unknown;ageMs:number}>>):void=>{
 const directory=join(store,"v1","origins",name);mkdirSync(directory,{recursive:true});
 for(const [file,{content,ageMs}] of Object.entries(files)){const path=join(directory,file);writeFileSync(path,typeof content==="string"?content:JSON.stringify(content));utimesSync(path,(now-ageMs)/1000,(now-ageMs)/1000);}
};
const record=(state:string,ageMs:number,extra:Record<string,unknown>={})=>({content:{version:1,origin:"x",platform:"linux",state,ready:false,stage:"tools",stageIndex:4,stageCount:5,attempts:9,refusal:null,updatedMs:now-ageMs,...extra},ageMs});

test("a stale running record of a machine that went away is offline, a live one working, a silent sidecar silent, and an origin with only its enrolment absent",()=>{
 const store=mkdtempSync(join(tmpdir(),"semio-workers-"));
 try{
  origin(store,"g1-m04",{"ready.json":record("running",86_400_000),"pool.json":{content:{},ageMs:86_400_000}});
  origin(store,"g1-m01",{"ready.json":record("running",5_000,{platform:"macos",stage:"workspace",stageIndex:5}),"pool.json":{content:{},ageMs:3_000}});
  origin(store,"g1-m12",{"ready.json":record("backoff",3_600_000,{refusal:{code:"stage-timeout",en:"e",de:"d"}}),"attest.json":{content:{},ageMs:2_000}});
  origin(store,"g1-m09",{"fleet-auth.json":{content:{},ageMs:700_000_000}});
  origin(store,"g1-server",{"pool.json":{content:{},ageMs:1_000}});
  const rows=readReadyWorkersV1({store,nowMs:now}),by=(name:string)=>rows.find(row=>row.origin===name)!;
  expect(rows.map(row=>row.origin)).toEqual(["g1-m01","g1-m04","g1-m09","g1-m12","g1-server"]);
  expect(by("g1-m04")).toMatchObject({liveness:"offline",recorded:"running",stage:"tools",attempts:9,recordAgeSeconds:86_400,heartbeatAgeSeconds:86_400});
  expect(by("g1-m01")).toMatchObject({liveness:"working",platform:"macos",stage:"workspace",stageIndex:5,recordAgeSeconds:5,heartbeatAgeSeconds:3});
  expect(by("g1-m12")).toMatchObject({liveness:"silent",refusal:{code:"stage-timeout"}});
  expect(by("g1-m09")).toMatchObject({liveness:"absent",recorded:null,recordAgeSeconds:null,heartbeatAgeSeconds:null});
  expect(by("g1-server")).toMatchObject({liveness:"absent",heartbeatAgeSeconds:1});
  expect(READY_OFFLINE_SECONDS_V1).toBe(P.offlineMilliseconds/1000);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("a missing store, a broken record and a record of a wrong shape give no row, absent or the safe default and never throw",()=>{
 expect(readReadyWorkersV1({store:join(tmpdir(),"semio-workers-none-"+process.pid),nowMs:now})).toEqual([]);
 const store=mkdtempSync(join(tmpdir(),"semio-workers-"));
 try{
  origin(store,"g1-m02",{"ready.json":{content:"{not json",ageMs:1_000}});
  origin(store,"g1-m03",{"ready.json":{content:{state:7,ready:false,updatedMs:now-1_000,stageIndex:-3,refusal:{code:1}},ageMs:1_000},"pool.json":{content:{},ageMs:1_000}});
  const rows=readReadyWorkersV1({store,nowMs:now});
  expect(rows[0]).toMatchObject({origin:"g1-m02",liveness:"absent"});
  expect(rows[1]).toMatchObject({origin:"g1-m03",liveness:"idle",stage:null,stageIndex:0,refusal:null});
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("the recent runs of a stage count how they ended inside the window, per cause, and a worker row carries them from its published timings",()=>{
 const run=(stage:string,outcome:string,agoHours:number,seconds:number)=>({stage,outcome,endedMs:now-agoHours*3_600_000,seconds});
 const timings={version:1,origin:"g1-m01",runs:[run("workspace","stalled",1,900),run("workspace","stalled",2,1_100),run("workspace","stage-failed",3,60),run("workspace","stage-timeout",4,7_000),run("workspace","ok",5,3_000),run("workspace","stalled",13,5_000),run("tools","stalled",1,10)]};
 expect(readyRecentRunsV1(timings,"workspace",now)).toEqual({hours:12,ok:1,failed:1,stalled:2,timedOut:1,seconds:12_060});
 expect(readyRecentRunsV1(timings,"workspace",now,0.5)).toBeNull();
 expect(readyRecentRunsV1(timings,"workspace",now,1.5)).toEqual({hours:1.5,ok:0,failed:0,stalled:1,timedOut:0,seconds:900});
 expect(readyRecentRunsV1(timings,"tools",now,12)).toEqual({hours:12,ok:0,failed:0,stalled:1,timedOut:0,seconds:10});
 expect(readyRecentRunsV1(timings,null,now)).toBeNull();
 expect(readyRecentRunsV1({runs:"x"},"workspace",now)).toBeNull();
 expect(readyRecentRunsV1(null,"workspace",now)).toBeNull();
 expect(readyRecentRunsV1({runs:[null,{stage:"workspace",endedMs:"soon",outcome:"ok"},7]},"workspace",now)).toBeNull();
 const store=mkdtempSync(join(tmpdir(),"semio-workers-"));
 try{
  origin(store,"g1-m01",{"ready.json":record("running",5_000,{platform:"macos",stage:"workspace",stageIndex:5}),"ready-timings.json":{content:timings,ageMs:60_000},"pool.json":{content:{},ageMs:3_000}});
  origin(store,"g1-m02",{"ready.json":record("running",5_000,{stage:"workspace"}),"ready-timings.json":{content:"{torn",ageMs:60_000},"pool.json":{content:{},ageMs:3_000}});
  origin(store,"g1-m03",{"ready.json":record("running",5_000,{stage:"workspace"}),"pool.json":{content:{},ageMs:3_000}});
  const rows=readReadyWorkersV1({store,nowMs:now}),by=(name:string)=>rows.find(row=>row.origin===name)!;
  expect(by("g1-m01").recent).toEqual({hours:12,ok:1,failed:1,stalled:2,timedOut:1,seconds:12_060});
  expect(by("g1-m02").recent).toBeNull();
  expect(by("g1-m03").recent).toBeNull();
 }finally{rmSync(store,{recursive:true,force:true});}
});
