import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readReadyWorkersV1} from "../../🟦️.ts";

/** 💓️ Contract: a machine whose replica still publishes its head is alive, so a readiness record that stopped beating while attest and pool are old is `silent` (stale readiness beat), never `offline`; a machine with no sign of life at all stays `offline`. */
const now=1_800_000_000_000;
const origin=(store:string,name:string,files:Readonly<Record<string,{content:unknown;ageMs:number}>>):void=>{
 const directory=join(store,"v1","origins",name);mkdirSync(directory,{recursive:true});
 for(const [file,{content,ageMs}] of Object.entries(files)){const path=join(directory,file);writeFileSync(path,JSON.stringify(content));utimesSync(path,(now-ageMs)/1000,(now-ageMs)/1000);}
};
const record=(ageMs:number)=>({content:{version:1,origin:"x",platform:"windows",state:"running",ready:false,stage:"dependencies",stageIndex:3,stageCount:5,attempts:49,refusal:null,updatedMs:now-ageMs,beatMilliseconds:30_000},ageMs});

test("a stopped readiness beat next to a live head is silent, with no head it is offline",()=>{
 const store=mkdtempSync(join(tmpdir(),"semio-head-"));
 try{
  origin(store,"g1-m05",{"ready.json":record(3*3_600_000),"attest.json":{content:{},ageMs:2_700_000},"head.json":{content:{},ageMs:4_000}});
  origin(store,"g1-m04",{"ready.json":record(3*3_600_000),"attest.json":{content:{},ageMs:2_700_000}});
  const rows=readReadyWorkersV1({store,nowMs:now}),by=(name:string)=>rows.find(row=>row.origin===name)!;
  expect(by("g1-m05")).toMatchObject({liveness:"silent",recorded:"running",recordAgeSeconds:10_800,heartbeatAgeSeconds:4});
  expect(by("g1-m04").liveness).toBe("offline");
 }finally{rmSync(store,{recursive:true,force:true});}
});
