import Ajv from "ajv";
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {FENCE_BUDGET_POLICY_V1 as P,fenceBudgetDoneV1,fenceBudgetStartV1,fenceDueV1} from "../🟦️.ts";

/** 🧮️ Contract: a stage whose full fence takes minutes pays for a handful of full fences, not for one at every site and every 64th binding, and a machine whose fence is instant behaves as before. The fixture is language-agnostic; a Python oracle replays it; the platform laws run the same stage with the fence cost of each platform. */
const ajv=new Ajv({strict:false});
const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url));
const replay=()=>fixture.runs.map(run=>{let state=fenceBudgetStartV1();return run.steps.map(step=>{const verdict=fenceDueV1(state,{nowMs:step.nowMs,final:step.final},P);state=verdict.due?fenceBudgetDoneV1(verdict.state,{startMs:step.nowMs,endMs:step.nowMs+step.durationMs}):verdict.state;return verdict.due;});});

type Site=Readonly<{final:boolean;binding:boolean}>;
const stageSites=(bindings:number,interval:number):Site[]=>[...Array.from({length:8},()=>({final:false,binding:false})),{final:true,binding:false},...Array.from({length:Math.floor(bindings/interval)},()=>({final:false,binding:true})),{final:true,binding:false}];
function stage(bindings:number,interval:number,fenceMs:number,bindingMs:number,budgeted:boolean):{full:number;fenceMs:number;totalMs:number}{
 let now=0,state=fenceBudgetStartV1(),full=0,spent=0;
 for(const site of stageSites(bindings,interval)){
  now+=site.binding?interval*bindingMs:5000;
  const verdict=budgeted?fenceDueV1(state,{nowMs:now,final:site.final},P):{due:true,state};
  if(verdict.due){full++;spent+=fenceMs;state=fenceBudgetDoneV1(verdict.state,{startMs:now,endMs:now+fenceMs});now+=fenceMs;}else state=verdict.state;
 }
 return {full,fenceMs:spent,totalMs:now};
}

test("the fixture, the policy and the schema agree",()=>{
 expect(ajv.validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
 expect(ajv.validate({$ref:"#/definitions/Policy",definitions:schema.definitions},P)).toBe(true);
});

test("the fixture decides as the steps say",()=>{
 const rows=replay();
 fixture.runs.forEach((run,index)=>run.steps.forEach((step,at)=>expect(rows[index]![at],run.name+": "+step.name).toBe(step.due)));
});

test("the Python oracle replays the runs and agrees on every step",()=>{
 const run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(replay());
});

test("a final fence is full whatever the clock says, and a skipped fence only counts",()=>{
 const first=fenceDueV1(fenceBudgetStartV1(),{nowMs:0,final:false},P),state=fenceBudgetDoneV1(first.state,{startMs:0,endMs:60000}),skipped=fenceDueV1(state,{nowMs:61000,final:false},P),final=fenceDueV1(skipped.state,{nowMs:61001,final:true},P);
 expect(first.due).toBe(true);expect(skipped.due).toBe(false);expect(skipped.state.skipped).toBe(1);expect(final.due).toBe(true);
});

for(const platform of fixture.platforms)test("a stage of 1500 bindings on "+platform.name+" ("+platform.fenceMilliseconds/1000+" s per full fence) pays for a handful of fences",()=>{
 const old=stage(1500,64,platform.fenceMilliseconds,platform.bindingMilliseconds,false),now=stage(1500,64,platform.fenceMilliseconds,platform.bindingMilliseconds,true);
 expect(old.full).toBe(stageSites(1500,64).length);
 expect(now.full).toBeLessThan(old.full);
 expect(now.fenceMs).toBeLessThanOrEqual(old.fenceMs);
 if(platform.fenceMilliseconds>=60000){
  expect(old.fenceMs).toBeGreaterThan(2*3600*1000);
  expect(now.full).toBeLessThanOrEqual(6);
  expect(now.totalMs).toBeLessThan(old.totalMs/4);
  expect(now.totalMs).toBeLessThan(3600*1000);
 }
});

test("a machine whose fence is instant pays for every fence as before",()=>{
 const now=stage(1500,64,0,300,true);
 expect(now.full).toBe(stageSites(1500,64).length);
});

test("the producer takes every interim fence through the budget and keeps its two final fences full",()=>{
 const source=readFileSync(here("../../💾️storage/🟦️.ts"),"utf8");
 expect(source).toContain("fenceDueV1");
 expect(source).toContain("fullFence=async()=>");
 expect((source.match(/await finalFence\(\)/gu)??[]).length).toBe(1);
 expect((source.match(/verifyReady=async\(\)=>\{[^]*?await finalFence\(\);\}/gu)??[]).length).toBe(1);
 expect((source.match(/await verifyReady\(\)/gu)??[]).length).toBe(2);
 expect(source).toContain("frontierBatchPaths:");
});
