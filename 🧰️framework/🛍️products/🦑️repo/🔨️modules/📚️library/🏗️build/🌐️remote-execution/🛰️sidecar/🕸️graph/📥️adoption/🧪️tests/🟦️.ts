import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {setTimeout as sleep} from "node:timers/promises";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readyPlatformV1} from "../../../🚦️ready/🟦️.ts";
import type {GenerationHeadCandidateV1} from "../../../../../🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts";
import {createHeadAdoptionLiveProbeV1,createHeadAdoptionTurnV1,headAdoptionPolicyV1,liveForHeadAdoptionV1} from "../🟦️.ts";
import fixture from "./🔣️.json";

const candidate:GenerationHeadCandidateV1={origin:"g1-producer",scope:"all",generation:5,manifest:"b".repeat(64),identity:"c".repeat(64),implementation:"d".repeat(64),platform:"darwin-arm64"};
const signal=new AbortController().signal;

test("head adoption dispatch requires every gate and retries an exact head only after cooldown",async()=>{
 using sqlite=new Database(":memory:");
 const oracle=sqlite.query("WITH gates(eligible) AS (SELECT ?1 AND ?2='normal' AND NOT ?3 AND ?4) SELECT eligible AS starts, eligible AND ?5='complete' AS restores FROM gates");
 for(const row of fixture.cases){
  let starts=0,restores=0,now=0;
  const enter=createHeadAdoptionTurnV1({now:()=>now,eligible:async()=>row.ready&&row.pressure==="normal"&&!row.busy,async withScope(_signal,run){if(row.capacity){starts++;await run({candidate:async()=>row.head==="complete"?candidate:row.head==="selected"?{...candidate,scope:"a".repeat(64)}:undefined,async restore(head){expect(head).toBe(candidate);restores++;return{status:"absent"};}});}}});
  await enter(signal);
  const expected={starts:row.starts,restores:row.restores};
  expect(oracle.get(Number(row.ready),row.pressure,Number(row.busy),Number(row.capacity),row.head),row.id).toEqual(expected);
  expect({starts,restores},row.id).toEqual(expected);
  if(row.head==="complete"&&row.restores===1){await enter(signal);expect(restores).toBe(1);now=headAdoptionPolicyV1.cooldownMilliseconds+1;await enter(signal);expect(restores).toBe(2);}
 }
});

test("overlapping dispatch is coalesced and the current attempt yields to foreground",async()=>{
 let restores=0,eligible=true,entered!:()=>void;
 const ready=new Promise<void>(resolve=>{entered=resolve;});
 const turn=createHeadAdoptionTurnV1({eligible:async()=>eligible,wait:async(_ms,signal)=>sleep(1,undefined,{signal}).catch(()=>undefined),async withScope(signal,run){await run({candidate:async()=>candidate,async restore(){
  restores++;entered();
  await new Promise<void>(done=>signal.addEventListener("abort",()=>done(),{once:true}));
  return{status:"absent"};
 }});}});
 const first=turn(signal);
 await ready;
 await turn(signal);
 expect(restores).toBe(1);
 eligible=false;
 await first;
});

test("the adoption liveness gate requires the local origin's valid and fresh record and never that it is Ready",async()=>{
 const store=mkdtempSync(join(tmpdir(),"graph-adopt-ready-")),origin="g1-origin",now=900000,directory=join(store,"v1","origins",origin);
 try{
  expect(await liveForHeadAdoptionV1(store,origin,signal,()=>now)).toBe(false);
  mkdirSync(directory,{recursive:true});
  const file=join(directory,"ready.json"),ready={version:1,origin,host:"host",platform:readyPlatformV1(),pid:1,state:"ready",digest:"a".repeat(64),ready:true,stage:null,stageIndex:0,stageCount:4,attempts:0,exit:null,refusal:null,reason:null,peakMegabytes:null,tail:[],nextAttemptMs:null,readyMs:1,startedMs:1};
  using sqlite=new Database(":memory:");
  const oracle=sqlite.query("SELECT (?1=?2 AND ?3 BETWEEN 0 AND 600000 AND NOT ?4) AS admit");
  for(const row of fixture.readiness){
   const value={...ready,...row.change,updatedMs:now-row.offset};
   writeFileSync(file,JSON.stringify(value));
   expect(oracle.get(value.origin,origin,row.offset,Number(Object.hasOwn(row.change,"unexpected"))),row.id).toEqual({admit:row.admit});
   expect(await liveForHeadAdoptionV1(store,origin,signal,()=>now),row.id).toBe(Boolean(row.admit));
  }
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("a settled restore reports only its exact outcome and candidate",async()=>{
 const controller=new AbortController(),observed:unknown[]=[],head={origin:candidate.origin,scope:candidate.scope,generation:candidate.generation,manifest:candidate.manifest};
 const outcome={status:"restored",head,generation:6,localManifest:"a".repeat(64)} as const;
 const tick=createHeadAdoptionTurnV1({eligible:async()=>true,async withScope(_signal,run){await run({candidate:async()=>candidate,async restore(){controller.abort();return outcome;}});},async result(result,pin,turn){observed.push([result,pin,turn.aborted]);}});
 await tick(controller.signal);
 expect(observed).toEqual([[outcome,candidate,true]]);
});

test("a stalled Ready read yields on deadline or turn cancellation and keeps its per-path slot",async()=>{
 const store="ready-fixture",origin="g1-origin",now=900000;
 const ready={version:1,origin,host:"host",platform:readyPlatformV1(),pid:1,state:"ready",digest:"a".repeat(64),ready:true,stage:null,stageIndex:0,stageCount:4,attempts:0,exit:null,refusal:null,reason:null,peakMegabytes:null,tail:[],nextAttemptMs:null,readyMs:1,startedMs:1,updatedMs:now};
 const bytes=Buffer.from(JSON.stringify(ready));
 using sqlite=new Database(":memory:");
 const oracle=sqlite.query("SELECT 1+NOT ?1 AS starts");
 for(const row of fixture.readinessProbe){
  let starts=0,settle!:(value:Uint8Array)=>void;
  const firstRead=new Promise<Uint8Array>(resolve=>{settle=resolve;});
  const firstTurn=new AbortController();
  const probe=createHeadAdoptionLiveProbeV1({maximumWaitMilliseconds:50,read:async()=>++starts===1?firstRead:bytes});
  const pending=probe(store,origin,firstTurn.signal,()=>now);
  await sleep(1);
  expect(starts,row.id).toBe(1);
  if(row.cancel==="caller")firstTurn.abort();
  expect(await pending,row.id).toBe(false);
  expect(await probe(store,origin,signal,()=>now),row.id).toBe(false);
  expect(oracle.get(1),row.id).toEqual({starts:row.startsWhileStalled});
  expect(starts,row.id).toBe(row.startsWhileStalled);
  settle(bytes);
  await sleep(1);
  expect(await probe(store,origin,signal,()=>now),row.id).toBe(true);
  expect(oracle.get(0),row.id).toEqual({starts:row.startsAfterSettlement});
  expect(starts,row.id).toBe(row.startsAfterSettlement);
 }
 const oversized=createHeadAdoptionLiveProbeV1({read:async()=>Buffer.alloc(headAdoptionPolicyV1.maximumReadyBytes+1)});
 expect(await oversized(store,origin,signal,()=>now)).toBe(false);
});
