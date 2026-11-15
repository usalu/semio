import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostCapacityHandleV1,type HostCapacityOptionsV1,type HostGrantV1,type HostRequestV1,type HostSampleV1} from "../../../🟦️.ts";
import decisionFixture from "../../🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:600000},memory:{...decisionFixture.policy.memory,rampMilliseconds:600000},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hl-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const machine={available:9000};
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:16384,availableMegabytes:machine.available,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const options=(directory:string,request:HostRequestV1,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request,control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),...over});
const show=(id:string):HostRequestV1=>({id,operation:"show",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:6553,perJobMegabytes:0,cpuPerJob:1,label:"projects"});
const workers=(holder:string|undefined):HostRequestV1=>({id:newHostCapacityIdV1(),operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:8,fixedMegabytes:0,perJobMegabytes:1536,cpuPerJob:1,elastic:true,label:"graph-farm",...holder===undefined?{}:{holder}});
const rows=(directory:string):HostGrantV1[]=>readdirSync(join(directory,"grants")).filter(name=>name.endsWith(".json")).map(name=>JSON.parse(readFileSync(join(directory,"grants",name),"utf8")) as HostGrantV1);

test("Through the real ledger on a 16 GB host the workers of a graph build are admitted inside its reservation, in English and German, and a request without a holder still waits for memory",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[];
 machine.available=11000;
 let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 let holderHandle!:HostCapacityHandleV1;
 const holder=withHostCapacityV1(options(directory,show(newHostCapacityIdV1()),{observe:async()=>1100}),async handle=>{holderHandle=handle;await hold;});
 for(let tries=0;holderHandle===undefined&&tries<200;tries++)await sleep(10);
 expect(holderHandle.reservedMegabytes).toBe(6553);
 machine.available=6800;
 const asked=workers(holderHandle.id);
 let seen!:HostCapacityHandleV1;
 await withHostCapacityV1(options(directory,asked,{observe:async()=>0,write:line=>english.push(line)}),async handle=>{
  seen=handle;
  const mine=rows(directory).find(row=>row.id===handle.id)!;
  expect(mine.holder,"the grant names the holder it lives in").toBe(holderHandle.id);
  expect(mine.reservedMegabytes).toBe(3072);
  expect(rows(directory).filter(row=>row.holder===undefined).map(row=>row.reservedMegabytes).reduce((a,b)=>a+b,0),"the host books the reservation of the holder once").toBe(6553);
  const second=workers(holderHandle.id);
  const waiting=withHostCapacityV1(options(directory,second,{observe:async()=>0,control:control(AbortSignal.timeout(500))}),async()=>"admitted").then(()=>"admitted",()=>"waited");
  expect(await waiting,"the third worker finds nothing left to lend and waits for memory like any request").toBe("waited");
 });
 expect(seen.jobs).toBe(2);expect(seen.reservedMegabytes).toBe(3072);
 expect(english.at(-1)).toMatch(/^\[capacity\] admitted exec inside the reservation of projects: 2 job\(s\), 3072 MB lent/u);
 await withHostCapacityV1(options(directory,workers(holderHandle.id),{observe:async()=>0,environment:{LANG:"de_DE.UTF-8"},write:line=>german.push(line)}),async()=>undefined);
 expect(german.at(-1)).toMatch(/^\[capacity\] exec innerhalb der Reservierung von projects zugelassen: 2 Job\(s\), 3072 MB aus dem ungenutzten Teil geliehen/u);
 const plain=await withHostCapacityV1(options(directory,workers(undefined),{observe:async()=>0,control:control(AbortSignal.timeout(500))}),async()=>"admitted").then(()=>"admitted",()=>"waited");
 expect(plain,"without a holder the request is charged to the host and waits").toBe("waited");
 release();await holder;
 expect(rows(directory),"everything is released").toEqual([]);
});
