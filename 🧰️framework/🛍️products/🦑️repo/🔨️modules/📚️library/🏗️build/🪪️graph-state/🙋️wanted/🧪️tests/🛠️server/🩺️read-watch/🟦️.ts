import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readReadWatchCheckpointV1} from "../../../🟦️.ts";
import {createGraphReadWatchV1,wantedServerPolicyV1} from "../../../🛠️server/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import installSchema from "../../../../../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json" with {type:"json"};

for(const row of fixture.cases)test(row.name,()=>{
 let time=0;
 const watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
 let resumed=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
 for(const step of row.steps){
  time=step.at;
  const end=Math.min(step.line.length,73);
  watch.feed(step.line.slice(0,end));
  watch.feed(step.line.slice(end)+"\n");
  expect(watch.silentMilliseconds(),step.label).toBe(step.silent);
  expect<unknown>(watch.verdict(),step.label).toBe(step.verdict);
  resumed.feed(step.line+"\n");
  expect(resumed.silentMilliseconds(),step.label).toBe(step.silent);
  expect<unknown>(resumed.verdict(),step.label).toBe(step.verdict);
  resumed=createGraphReadWatchV1(wantedServerPolicyV1,()=>time,undefined,0,resumed.observation());
 }
});

test("the live phase producer renews on repeated observation growth only",async()=>{
 const {createGraphPhaseClockV1}=await import("../../../../../🕸️graph/⏱️phases/🟨️.mjs");
 let time=0;
 const watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
 const clock=createGraphPhaseClockV1({now:()=>time,minimumMilliseconds:0,publish:(resource:string)=>watch.feed("[script] "+JSON.stringify({activity:{resource}})+"\n")});
 await clock.begin("inference",4791);
 await clock.advance("inference",4305);
 const report=(rows:number,seconds:number,name="@semio-tech/writer-writer-rs")=>clock.running("inference",[{name,kind:"language:rust",seconds,rows}],{key:"waiting",used:12220,budget:10560});
 time=5000;await report(43,9000);
 time=100000;await report(43,9100);
 expect(watch.silentMilliseconds()).toBe(100000);
 time=1400000;await report(50,10300);
 expect(watch.silentMilliseconds()).toBe(0);
 time=1500000;await report(200,10400,"other owner");
 expect(watch.silentMilliseconds()).toBe(100000);
 time=2900000;await report(50,11700);
 expect(watch.verdict()).toBe("stalled");
});

test("bounded tracking rejects oversize lines, retains tracked owners, and does not admit new names by eviction",()=>{
 let time=0;
 const policy={...wantedServerPolicyV1,readProgress:{...wantedServerPolicyV1.readProgress,maximumOwners:1,maximumPhases:1}};
 const watch=createGraphReadWatchV1(policy,()=>time);
 const progress=(name:string,rows:number)=>'[script] {"activity":{"resource":"build:graph-phase:inference:1:2:1:5:'+name+'|project|5|'+rows+'"}}';
 watch.feed(progress("writer",43)+"\n");
 time=80;
 watch.feed("x".repeat(policy.readProgress.maximumLineCharacters+1));
 watch.feed(progress("writer",50)+"\n");
 watch.feed(progress("other",51)+"\n"+progress("other",52)+"\n");
 watch.feed('[script] {"activity":{"resource":"build:graph-phase:files:1:1"}}\n');
 expect(watch.silentMilliseconds()).toBe(80);
 watch.feed(progress("writer",50)+"\n");
 expect(watch.silentMilliseconds()).toBe(0);
 expect(watch.counters()).toEqual({"build:graph-phase:inference":[1,2]});
});

test("cold read completed source operations and exact startup completions survive handoff",()=>{
 const {idle,ceiling,steps}=fixture.startup,sqlite=new Database(":memory:");
 try{
  const expected=sqlite.query(`WITH steps AS (
   SELECT cast(key AS integer) AS n, json_extract(value,'$.at') AS at,
    json_extract(value,'$.key') AS name, json_extract(value,'$.done') AS done FROM json_each(?)
  ), fresh AS (
   SELECT s.n,s.at FROM steps s WHERE s.done>coalesce((SELECT max(p.done) FROM steps p WHERE p.name=s.name AND p.n<s.n),0)
  ), quiet AS (
   SELECT s.n,s.at,s.at-coalesce((SELECT max(f.at) FROM fresh f WHERE f.n<=s.n),0) AS silent FROM steps s
  ) SELECT silent,CASE WHEN at>=? THEN 'too-long' WHEN silent>=? THEN 'stalled' ELSE 'go' END AS verdict FROM quiet ORDER BY n`).all(JSON.stringify(steps),ceiling,idle);
  let time=0,watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
  const actual=steps.map(step=>{
   time=step.at;
   watch.feed(step.line+"\n");
   const result={silent:watch.silentMilliseconds(),verdict:watch.verdict()};
   const checkpoint=readReadWatchCheckpointV1({version:1,runner:1,read:2,startedMs:0,offset:20,device:"1",inode:"2",prefix:"0".repeat(64),observation:watch.observation()});
   expect(checkpoint,step.line).toBeDefined();
   watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time,undefined,0,checkpoint!.observation);
   return result;
  });
  expect<unknown>(actual).toEqual(expected);
 }finally{sqlite.close();}
});


test("an owned registry repair starts an inference epoch across watchdog handoff",()=>{
 const {steps,idle,ceiling}=fixture.repair,sqlite=new Database(":memory:");
 try{
  const expected=sqlite.query(`WITH entries AS (
   SELECT cast(key AS integer) AS n,json_extract(value,'$.at') AS at,json_extract(value,'$.kind') AS kind,
    json_extract(value,'$.epoch') AS epoch,json_extract(value,'$.done') AS done FROM json_each(?)
  ), fresh AS (
   SELECT s.n,s.at FROM entries s WHERE s.done>coalesce((SELECT max(p.done) FROM entries p
    WHERE p.epoch=s.epoch AND p.kind=s.kind AND p.n<s.n),0)
  ), quiet AS (
   SELECT s.n,s.at,s.at-coalesce((SELECT max(f.at) FROM fresh f WHERE f.n<=s.n),0) AS silent FROM entries s
  ) SELECT silent,CASE WHEN at>=? THEN 'too-long' WHEN silent>=? THEN 'stalled' ELSE 'go' END AS verdict FROM quiet ORDER BY n`).all(JSON.stringify(steps),ceiling,idle);
  let time=0,watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
  const withoutRepair=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
  const actual=steps.map(step=>{
   time=step.at;
   watch.feed(step.line+"\n");
   if(!step.line.startsWith("[graph-repair] "))withoutRepair.feed(step.line+"\n");
   if(step.at===1500300){
    expect(withoutRepair.verdict()).toBe("stalled");
    expect(watch.verdict()).toBe("go");
   }
   const result={silent:watch.silentMilliseconds(),verdict:watch.verdict()};
   const checkpoint=readReadWatchCheckpointV1({version:1,runner:1,read:2,startedMs:0,offset:20,device:"1",inode:"2",prefix:"0".repeat(64),observation:watch.observation()});
   expect(checkpoint,step.line).toBeDefined();
   watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time,undefined,0,checkpoint!.observation);
   return result;
  });
  expect<unknown>(actual).toEqual(expected);
  expect(watch.counters()).toEqual({"build:graph-phase:inference":[67,4810],"owners:ledger":[66,4810]});
 }finally{sqlite.close();}
});


test("owned workspace edge checks survive registry repair, replay and handoff",()=>{
 const rows=fixture.workspaceEdges;
 const sqlite=new Database(":memory:");
 try{
  const expected=sqlite.query(`WITH eligible AS (
   SELECT step.key AS n, json_extract(step.value,'$.at') AS at,
    CASE WHEN NOT EXISTS(SELECT 1 FROM json_each(step.value) field WHERE field.key NOT IN ('at','entries'))
    THEN json_extract(step.value,'$.entries') END AS entries FROM json_each(?) step
  ), progressed AS (
   SELECT s.n,s.at FROM eligible s WHERE s.entries>coalesce((SELECT max(p.entries) FROM eligible p WHERE p.n<s.n),0)
  )
  SELECT s.at-coalesce((SELECT max(p.at) FROM progressed p WHERE p.n<=s.n),0) AS silent FROM eligible s ORDER BY s.n`).all(JSON.stringify(rows));
  let time=0,watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
  const actual=rows.map(({at,entries,repair,owner="repository:build",command="repository:command",stage="running",mode="exclusive",elapsedMs=0})=>{
   time=at;
   const row={owner,command,stage,activity:{resource:"dependencies:workspace-edges:"+entries,mode,elapsedMs}};
   watch.feed((repair?'[graph-repair] {"round":1,"registry":"followed","roots":["a"],"adopted":true,"moved":false}':"[script] "+JSON.stringify(row))+"\n");
   const result={silent:watch.silentMilliseconds()};
   const checkpoint=readReadWatchCheckpointV1({version:1,runner:1,read:2,startedMs:0,offset:20,device:"1",inode:"2",prefix:"0".repeat(64),observation:watch.observation()});
   expect(checkpoint,row.activity.resource).toBeDefined();
   watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time,undefined,0,checkpoint!.observation);
   return result;
  });
  expect<unknown>(actual).toEqual(expected);
 }finally{sqlite.close();}
});

test("owned Store traversal advances across operations while lower producer retries preserve quiet",()=>{
 const rows=fixture.storeOwnership,sqlite=new Database(":memory:");
 const {storeOwnershipBatch,storeOwnershipPattern}=wantedServerPolicyV1.readProgress;
 expect(storeOwnershipBatch).toBe(installSchema.$defs.InstalledPathProtocol.const.progressWorkUnits);
 for(const operation of installSchema.$defs.CurrentOwnershipProgress.properties.operation.enum)
  expect(new RegExp(storeOwnershipPattern,"u").test("dependencies:store-ownership:"+operation+":"+storeOwnershipBatch)).toBe(true);
 try{
  const expected=sqlite.query(`WITH entries AS (
   SELECT key AS n,json_extract(value,'$.at') AS at,json_extract(value,'$.done') AS done,
    json_extract(value,'$.elapsedMs') AS age,json_extract(value,'$.operation') AS operation,json_extract(value,'$.mode') AS mode FROM json_each(?)
  ), eligible AS (SELECT n,at,done FROM entries WHERE age>=0 AND done%64=0 AND mode IS NULL AND
    operation IN ('list','state','readText','membership','frontier','selection','publication')),
   fresh AS (SELECT s.n,s.at FROM eligible s WHERE s.done>coalesce((SELECT max(p.done) FROM eligible p WHERE p.n<s.n),0))
   SELECT at-coalesce((SELECT max(f.at) FROM fresh f WHERE f.n<=s.n),0) AS silent FROM entries s ORDER BY n`).all(JSON.stringify(rows));
  let time=0,watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
  const actual=rows.map(({at,operation,done,elapsedMs,mode="exclusive"})=>{
   time=at;
   const row={owner:"repository:build",command:"repository:command",stage:"running",activity:{resource:"dependencies:store-ownership:"+operation+":"+done,mode,elapsedMs}};
   watch.feed((operation===undefined?"ordinary output":"[script] "+JSON.stringify(row))+"\n");
   const result={silent:watch.silentMilliseconds()};
   const checkpoint=readReadWatchCheckpointV1({version:1,runner:1,read:2,startedMs:0,offset:20,device:"1",inode:"2",prefix:"0".repeat(64),observation:watch.observation()});
   expect(checkpoint,row.activity.resource).toBeDefined();
   watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time,undefined,0,checkpoint!.observation);
   return result;
  });
  expect<unknown>(actual).toEqual(expected);
  expect(watch.verdict()).toBe("stalled");
 }finally{sqlite.close();}
});

test("completed preservation ownership checks carry a long detached read through the quiet limit",()=>{
 const rows=fixture.preservationOwnership,sqlite=new Database(":memory:");
 try{
  const expected=sqlite.query(`WITH entries AS (
    SELECT key AS n,json_extract(value,'$.at') AS at,json_extract(value,'$.done') AS done,
      json_extract(value,'$.elapsedMs') AS age,json_extract(value,'$.mode') AS mode,
      json_extract(value,'$.resource') AS resource,json_extract(value,'$.command') AS command FROM json_each(?)
   ), eligible AS (
    SELECT n,at,done FROM entries WHERE done%64=0 AND age IS NULL AND mode IS NULL AND resource IS NULL AND command IS NULL
   ), fresh AS (
    SELECT s.n,s.at FROM eligible s WHERE s.done>coalesce((SELECT max(p.done) FROM eligible p WHERE p.n<s.n),0)
   ), quiet AS (
    SELECT at-coalesce((SELECT max(f.at) FROM fresh f WHERE f.n<=s.n),0) AS silent FROM entries s
   ) SELECT silent,CASE WHEN silent>=? THEN 'stalled' ELSE 'go' END AS verdict FROM quiet`).all(JSON.stringify(rows),wantedServerPolicyV1.inferStallMilliseconds);
  let time=0,watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
  const actual=rows.map(({at,done,mode="exclusive",elapsedMs=0,resource="preservation-ownership",command="repository:command"})=>{
   time=at;
   const row={owner:"repository:build",command,stage:"running",activity:{resource:"dependencies:"+resource+":"+done,mode,elapsedMs}};
   watch.feed("[script] "+JSON.stringify(row)+"\n");
   const result={silent:watch.silentMilliseconds(),verdict:watch.verdict()};
   const checkpoint=readReadWatchCheckpointV1({version:1,runner:1,read:2,startedMs:0,offset:20,device:"1",inode:"2",prefix:"0".repeat(64),observation:watch.observation()});
   expect(checkpoint,row.activity.resource).toBeDefined();
   watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time,undefined,0,checkpoint!.observation);
   return result;
  });
  expect<unknown>(actual).toEqual(expected);
  expect(watch.counters()).toEqual({"dependencies:preservation-ownership":[600832,600832]});
 }finally{sqlite.close();}
});
