import Ajv from "ajv";
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {TAKE_POLICY_V1,createTakeLedgerV1,takeAdmittedV1,takeConcurrencyV1,takeLanesV1, takePassV1,takePollV1,takePollWaitV1,takeSplitV1} from "../🟨️.mjs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import cancellation from "./🧫️fixtures/take-cancellation.json" with {type:"json"};
import {createGraphFleetClientV1} from "../../../../../🪪️graph-state/🛰️exchange/📡️client/🟦️.ts";
import {createProjectFleetPortV1,runTakeLookV1} from "../../../../../🪪️graph-state/📦️project-records/🟦️.ts";

/** 📥️ Contract: the rules of taking records before inferring are pure and equal an independent Python model (the pause before a look at the cache, the report line of the ledger in both languages); a pass looks at every owner once and repeats only the looks the cache did not answer; the poll grows its pause, starts again when a record arrived and ends when asked.
 * @see ../🟨️.mjs */
const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url));

test("a take deadline aborts the transport and waits for it to release its admission",async()=>{
 const db=new Database(":memory:");
 try{
  for(const row of cancellation){
   const pass=new AbortController(),admission={flying:0},directory=here("./");
   let begin!:()=>void,release!:()=>void,transportSignal:AbortSignal|undefined,closed=false,settled=false;
   const started=new Promise<void>(resolve=>{begin=resolve;}),drain=new Promise<void>(resolve=>{release=resolve;});
   const request=async(_url:unknown,init?:RequestInit)=>{
    transportSignal=init?.signal??undefined;
    begin();
    try{
     await drain;
     transportSignal?.throwIfAborted();
     return new Response(null,{status:404});
    }finally{closed=true;}
   };
   const client=createGraphFleetClientV1({endpoint:"https://fixture.invalid",instance:"graph",token:undefined,upload:false,stateDirectory:directory},{fetch:request as typeof fetch});
   const port=createProjectFleetPortV1({root:directory,environment:{},client,owner:"fixture",platform:"macos"})!;
   const attempt=takeAdmittedV1((name,signal)=>runTakeLookV1(async look=>{
    await port.gate({cachePath:here("./test-record"),fleet:name,keyText:"[]",exportRecord:()=>undefined,import:async()=>false});
    return look.outcome==="fault"?"fault":"miss";
   },signal),()=>undefined,pass.signal,admission);
   const task=takePassV1({names:["fixture"],attempt,signal:pass.signal}).finally(()=>{settled=true;});
   await started;
   try{
    if(row.cancel)pass.abort(Error("Take deadline"));
    expect(transportSignal?.aborted,row.id).toBe(row.cancel);
    expect(admission.flying,row.id).toBe(1);
    expect(settled,row.id).toBe(false);
   }finally{release();await task;}
   expect(closed,row.id).toBe(true);
   expect(admission.flying,row.id).toBe(0);
   const oracle=db.query("select not ?1 recorded").get(Number(row.cancel))as{recorded:number};
   expect((await task).has("fixture"),row.id).toBe(row.recorded);
   expect((await task).has("fixture"),row.id).toBe(Boolean(oracle.recorded));
  }
 }finally{db.close();}
});

test("the pause, the line and the policy equal the schema, the fixture and the Python oracle",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate({$ref:"#/definitions/Policy",definitions:schema.definitions},schema.definitions.Policy.const)).toBe(true);
 const run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🧫️fixtures/🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const oracle=JSON.parse(run.stdout) as {waits:number[];lines:string[];lanes:number[];splits:{light:string[];heavy:string[]}[]};
 expect(fixture.rounds.map(round=>takePollWaitV1(round))).toEqual(oracle.waits);
 expect(oracle.waits[0]).toBe(TAKE_POLICY_V1.pollFirstMilliseconds);expect(oracle.waits.at(-1)).toBe(TAKE_POLICY_V1.pollMaximumMilliseconds);
 expect(oracle.waits.every((value,index)=>index===0||value>=oracle.waits[index-1]!)).toBe(true);
 const lines=fixture.ledgers.map(row=>{const ledger=createTakeLedgerV1(row.total);for(const [name,state] of Object.entries(row.states))ledger.mark(name,state as never);ledger.want(row.wanted);ledger.run(row.running);return ledger.line(row.language);});
 expect(lines).toEqual(oracle.lines);
 expect(fixture.cuts.map(cut=>takeConcurrencyV1(cut??Infinity))).toEqual(oracle.lanes);
 expect(fixture.splits.map(row=>{const heavy=new Set(row.heavy);return takeSplitV1(row.names,name=>heavy.has(name));})).toEqual(oracle.splits);
 expect(oracle.splits[0]).toEqual({light:["o0","o2","o3","o5"],heavy:["o1","o4"]});
});

test("a pass under a cut never has more looks in flight than the cut allows, an uncut pass the declared concurrency",async()=>{
 for(const cut of [Infinity,4,2,1]){
  let flying=0,peak=0;
  await takePassV1({names:Array.from({length:60},(_,index)=>"o"+index),concurrency:takeConcurrencyV1(cut),attempt:async()=>{flying++;peak=Math.max(peak,flying);await new Promise(resolve=>setTimeout(resolve,2));flying--;return "miss";}});
  expect(peak).toBe(Math.min(TAKE_POLICY_V1.concurrency,cut));
 }
});

test("the counts of a ledger: an owner counts once by its last outcome, taking an owner ends its wish",()=>{
 const ledger=createTakeLedgerV1(5);
 ledger.mark("a","present");ledger.mark("b","miss");ledger.mark("c","stale");ledger.mark("d","miss");ledger.want(["b","c","d"]);ledger.run(2);
 expect(ledger.counts()).toMatchObject({present:1,fetched:0,stale:1,wanted:3,running:2,done:1,missing:4});
 ledger.mark("b","fetched");ledger.mark("c","inferred");
 expect(ledger.counts()).toMatchObject({present:1,fetched:1,here:1,stale:0,wanted:1,done:3,missing:2});
 expect(ledger.line("en")).toBe("owners 3 of 5 done: 1 present, 1 fetched from the fleet, 1 inferred here (2 running), 1 wanted from the fleet, 0 stale in the fleet");
 expect(ledger.line("de")).toBe("Projekte 3 von 5 fertig: 1 vorhanden, 1 aus der Flotte geholt, 1 hier abgeleitet (2 laufen), 1 von der Flotte erwartet, 0 in der Flotte veraltet");
});

test("a pass looks once at every owner with the bounded concurrency and repeats only the faults, a fault that remains is a miss",async()=>{
 const names=Array.from({length:40},(_,index)=>"o"+index),looks=new Map<string,number>();let live=0,peak=0;const sleeps:number[]=[];
 const outcomes=await takePassV1({names,concurrency:5,faultRetries:2,faultWaitMilliseconds:100,sleep:async ms=>{sleeps.push(ms);},attempt:async name=>{
  live++;peak=Math.max(peak,live);await new Promise(done=>setTimeout(done,1));live--;looks.set(name,(looks.get(name)??0)+1);
  if(name==="o3")return(looks.get(name)??0)<2?"fault":"fetched";
  if(name==="o4")return"fault";
  if(name==="o5")throw new Error("boom");
  return name==="o6"?"stale":name==="o7"?"miss":"present";
 }});
 expect(peak).toBeLessThanOrEqual(5);expect(peak).toBeGreaterThan(1);
 expect(outcomes.get("o3")).toBe("fetched");expect(outcomes.get("o4")).toBe("miss");expect(outcomes.get("o5")).toBe("miss");expect(outcomes.get("o6")).toBe("stale");expect(outcomes.get("o0")).toBe("present");
 expect(looks.get("o0")).toBe(1);expect(looks.get("o3")).toBe(2);expect(looks.get("o4")).toBe(3);expect(looks.get("o6")).toBe(1);
 expect(sleeps).toEqual([100,200]);expect(outcomes.size).toBe(40);
});

test("the poll grows its pause, starts again after an arrival, rotates through the queue and ends when asked",async()=>{
 const waits:number[]=[],queue=Array.from({length:130},(_,index)=>"m"+index),arrivedNames:string[]=[],seen=new Set<string>();let stopped=false,round=0;
 const result=await takePollV1({queue:()=>queue.filter(name=>!arrivedNames.includes(name)),stop:()=>stopped,
  sleep:async ms=>{waits.push(ms);if(++round>9)stopped=true;},
  attempt:async name=>{seen.add(name);return name==="m60"&&round>=4?"fetched":"miss";},
  arrived:name=>{arrivedNames.push(name);}});
 expect(arrivedNames).toEqual(["m60"]);expect(result.found).toBe(1);
 expect(waits.slice(0,4)).toEqual([2000,3000,4500,6750]);
 const reset=waits.findIndex((value,index)=>index>0&&value<waits[index-1]!);
 expect(reset).toBeGreaterThan(3);expect(waits[reset]).toBe(2000);expect(waits[reset+1]).toBe(3000);
 expect(seen.size).toBe(130);
});

test("the lanes of a take pass hold every owner once, local records first in their own lane, and leave an empty lane out",()=>{
 const names=Array.from({length:12},(_,index)=>"o"+index),present=new Set(["o0","o3","o4","o9"]);
 const lanes=takeLanesV1(names,name=>present.has(name));
 expect(lanes).toEqual([["o0","o3","o4","o9"],["o1","o2","o5","o6","o7","o8","o10","o11"]]);
 expect(lanes.flat().sort()).toEqual([...names].sort());
 expect(takeLanesV1(names,()=>true)).toEqual([names]);expect(takeLanesV1(names,()=>false)).toEqual([names]);expect(takeLanesV1([],()=>true)).toEqual([]);
});

test("a look starts only after the footprint gate admitted it unless it is the only one in flight, never without a gate, and an aborted admission ends the look",async()=>{
 const events:string[]=[];let open=false,release=()=>{};
 const gate={admit:async()=>{events.push("admit");if(!open)await new Promise<void>(resolve=>{release=resolve;});events.push("admitted");}};
 let finishFirst=()=>{};
 const look=takeAdmittedV1(async(name:string)=>{events.push("look "+name);if(name==="a")await new Promise<void>(resolve=>{finishFirst=resolve;});return name;},()=>gate);
 const first=look("a");await new Promise(resolve=>setTimeout(resolve,5));
 expect(events,"the only look in flight is admitted although the gate holds").toEqual(["look a"]);
 const second=look("b");await new Promise(resolve=>setTimeout(resolve,5));
 expect(events,"a second look waits for the gate").toEqual(["look a","admit"]);
 open=true;release();expect(await second).toBe("b");
 expect(events).toEqual(["look a","admit","admitted","look b"]);
 finishFirst();expect(await first).toBe("a");
 const idle=takeAdmittedV1(async(name:string)=>name,()=>({admit:async()=>{events.push("never");await new Promise(()=>undefined);}}));
 expect(await idle("c"),"after every look ended the next one is admitted at once").toBe("c");expect(events.includes("never")).toBe(false);
 expect(await takeAdmittedV1(async(name:string)=>name,()=>undefined)("b")).toBe("b");
 const state={flying:1},stopped=takeAdmittedV1(async(name:string)=>name,()=>({admit:async(signal?:AbortSignal)=>{signal?.throwIfAborted();}}),AbortSignal.abort(),state);
 await expect(stopped("c")).rejects.toBeDefined();expect(state.flying,"a refused admission leaves the count as it was").toBe(1);
 const gated=[],counting={admit:async()=>{gated.push(1);}},shared={flying:0};
 const outcomes=await takePassV1({names:["x","y","z","w"],concurrency:2,attempt:takeAdmittedV1(async(name:string)=>{await new Promise(resolve=>setTimeout(resolve,3));return name==="y"?"present":"miss";},()=>counting,undefined,shared) as never});
 expect([...outcomes.entries()]).toEqual([["x","miss"],["y","present"],["z","miss"],["w","miss"]]);
 expect(shared.flying).toBe(0);expect(gated.length,"every look but those that found the pass idle asks the gate").toBeGreaterThanOrEqual(2);expect(gated.length).toBeLessThanOrEqual(3);
});
