/** 🚦️ Contract: with every worker busy, the slice pool serves its queue by urgency (0 most urgent, 5 when none or an invalid one is named), equal urgency in arrival order, and a client that joins a queued load with a more urgent priority lifts that load.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "./🧬️schema.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {createSlicePoolV1,slicePriorityV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const until=async(check:()=>boolean)=>{for(let wait=0;!check()&&wait<300;wait++)await new Promise(done=>setTimeout(done,10));expect(check()).toBe(true);};

test("the slice pool serves waiting loads by urgency and keeps arrival order within one urgency",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 expect([undefined,null,-1,10,1.5,"1",0,9,4].map(slicePriorityV1)).toEqual([5,5,5,5,5,5,0,9,4]);
 for(const row of fixture.cases){
  const started:string[]=[],finish=new Map<string,()=>void>();
  const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,heartbeatMilliseconds:20,graceMilliseconds:50,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
   reaper:{reap:async()=>0,killNow:()=>undefined},
   reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
   createWorker:(_url:URL,data)=>{const name=String(data.name);const listeners:Record<string,(value:unknown)=>void>={};started.push(name);finish.set(name,()=>listeners.message?.({type:"done",graph:{}}));return{pid:undefined,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
  const ask=(name:string,priority:number|null)=>pool.run({root:"/tmp/"+name,name,arguments:[name],...priority===null?{}:{priority}}),runs:Promise<unknown>[]=[ask("busy",0)];
  await until(()=>started.length===1);
  for(const waiting of row.waiting)runs.push(ask(waiting.name,waiting.priority));
  await until(()=>pool.stats().queued===row.waiting.length);
  for(const join of row.joins)runs.push(ask(join.name,join.priority));
  await new Promise(done=>setTimeout(done,30));
  finish.get("busy")!();
  for(let index=0;index<row.order.length;index++){await until(()=>started.length===index+2);expect(started[index+1],row.id).toBe(row.order[index]);finish.get(row.order[index])!();}
  await Promise.all(runs);await pool.close();
  expect(started.slice(1),row.id).toEqual(row.order);
 }
});
