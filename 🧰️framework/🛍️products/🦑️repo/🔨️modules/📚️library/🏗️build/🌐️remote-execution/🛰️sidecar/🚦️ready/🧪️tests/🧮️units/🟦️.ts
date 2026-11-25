/** 🧮️ Contract: a stage timing row says how many compile units the run restored (from this machine, from the fleet) and compiled, measured as the growth of the unit counters between its start and its end; an unmeasured run says nothing, and the row passes the `Timings` schema the store publishes.
 * @see ../../🧬️schema/🔣️.json
 * @see ../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,readFileSync,readdirSync,rmSync,statSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{countSkipV1,countUnitV1}from"../../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts";
import{machinePathV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{READY_POLICY_V1 as P,createReadyLeaseV1,createReadyPortsV1,emptyReadyStateV1,readyDigestsV1,readyTimingUnitsV1,runReadyV1,type ReadyLocalStateV1,type ReadyPortsV1,type ReadyTimingV1}from"../../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`ready-units-${label}-`));scratch.push(path);return path;};
const count=(root:string,kind:"localHit"|"remoteHit"|"miss"|"stored",times:number)=>{for(let index=0;index<times;index++)countUnitV1(root,kind);};

test("a stage's units are the growth of the counters between its start and its end, equal to an independent byte count",()=>{
 const root=temp("grow"),ports=createReadyPortsV1({root,environment:{},origin:"g1-m98",group:"g1",say:()=>undefined});
 count(root,"localHit",3);count(root,"miss",2);
 const before=ports.units!();
 count(root,"localHit",5);count(root,"remoteHit",7);count(root,"miss",11);count(root,"stored",4);for(let index=0;index<2;index++)countSkipV1(root,"read-only");
 const after=ports.units!(),row=readyTimingUnitsV1(before,after);
 expect(row).toEqual({units:{restoredLocal:5,restoredFleet:7,compiled:11,stored:4,skipped:2}});
 const directory=machinePathV1(root,".cache","f","unit-counters");
 const python=spawnSync("python3",["-I","-c","import json,os,sys\nd=sys.argv[1]\nsize=lambda n:os.path.getsize(os.path.join(d,n)) if os.path.exists(os.path.join(d,n)) else 0\nskip=sum(os.path.getsize(os.path.join(d,'skip',n)) for n in os.listdir(os.path.join(d,'skip')))\nprint(json.dumps({k:size(k) for k in ['localHit','remoteHit','miss','stored']}|{'skipped':skip}))",directory],{encoding:"utf8"});
 expect(python.status).toBe(0);
 expect(JSON.parse(python.stdout)).toEqual({localHit:8,remoteHit:7,miss:13,stored:4,skipped:2});
 expect(Object.fromEntries(readdirSync(directory).filter(name=>name!=="since"&&name!=="skip").map(name=>[name,statSync(join(directory,name)).size]))).toEqual({localHit:8,remoteHit:7,miss:13,stored:4});
});

test("a run without a start or an end reading carries no units, and an emptied counter directory never counts negative",()=>{
 expect(readyTimingUnitsV1(null,{localHit:1,remoteHit:0,miss:0,stored:0,skipped:0})).toEqual({});
 expect(readyTimingUnitsV1({localHit:1,remoteHit:0,miss:0,stored:0,skipped:0},null)).toEqual({});
 expect(readyTimingUnitsV1({localHit:9,remoteHit:9,miss:9,stored:9,skipped:9},{localHit:0,remoteHit:0,miss:0,stored:0,skipped:0})).toEqual({units:{restoredLocal:0,restoredFleet:0,compiled:0,stored:0,skipped:0}});
});

test("a timing row with units is written to the store and validates against the Timings schema; a row with a negative count is refused",()=>{
 const store=temp("store"),root=temp("root"),ports=createReadyPortsV1({root,environment:{SEMIO_FLEET_STORE_ROOT:store},origin:"g1-m98",group:"g1",say:()=>undefined});
 const row={stage:"workspace",startedMs:1000,endedMs:9000,seconds:8,exit:0,outcome:"ok" as const,peakMegabytes:1,attempts:0,digest:"abc",units:{restoredLocal:5,restoredFleet:7,compiled:11,stored:4,skipped:2}};
 ports.timing!(row);
 const doc=JSON.parse(readFileSync(join(store,"v1","origins","g1-m98",P.timingsName),"utf8")) as {runs:{units:unknown}[]};
 expect(doc.runs[0]!.units).toEqual(row.units);
 expect(validateJsonSchemaSubset(schema.definitions.Timings,doc,schema)).toEqual([]);
 expect(validateJsonSchemaSubset(schema.definitions.Timings,{...doc,runs:[{...row,units:{...row.units,compiled:-1}}]},schema).length).toBeGreaterThan(0);
 expect(validateJsonSchemaSubset(schema.definitions.Timings,{...doc,runs:[{...row,units:{restoredLocal:1}}]},schema).length).toBeGreaterThan(0);
});

test("the readiness loop records the units a stage run restored and compiled in its timing row, and none for a stage whose start was not measured",async()=>{
 const digest=readyDigestsV1({platform:"linux",hash:()=>"b".repeat(64)}),controller=new AbortController(),rows:ReadyTimingV1[]=[];
 let state:ReadyLocalStateV1=emptyReadyStateV1(),now=1_000_000,marker=false,runs=0,restored=0,compiled=0,readings=0,measured=true;
 const ports:ReadyPortsV1={
  server:false,now:()=>now,jitter:0,cores:4,
  sleep:async()=>{now+=1000;await new Promise<void>(done=>setTimeout(done,2));controller.abort();},
  digests:async()=>digest,
  state:{read:()=>structuredClone(state),write:next=>{state=structuredClone(next);}},
  marker:{exists:()=>marker,write:()=>{marker=true;}},
  repoPresent:()=>true,human:()=>false,
  lease:createReadyLeaseV1({directory:temp("lease"),owner:"one",now:()=>Date.now()}),
  memory:()=>({availableMegabytes:8192,ok:true}),
  units:()=>{readings++;return measured?{localHit:restored,remoteHit:0,miss:compiled,stored:0,skipped:0}:null;},
  run:async()=>{runs++;restored+=40;compiled+=2;return{exit:runs===1?1:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:now,peakMegabytes:1,tail:[]};},
  status:{update:()=>undefined},say:()=>undefined,timing:row=>{rows.push(row);},
 };
 await runReadyV1(ports,controller.signal);
 expect(rows.length).toBeGreaterThan(0);
 expect(rows[0]!.units).toEqual({restoredLocal:40,restoredFleet:0,compiled:2,stored:0,skipped:0});
 measured=false;rows.length=0;
 const second=new AbortController();state=emptyReadyStateV1();
 await runReadyV1({...ports,sleep:async()=>{now+=1000;second.abort();},run:async()=>({exit:0,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:false,startedMs:now,peakMegabytes:1,tail:[]})},second.signal);
 expect(rows.length).toBeGreaterThan(0);expect(rows[0]!.units).toBeUndefined();
});
