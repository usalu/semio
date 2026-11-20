import {expect,test} from "bun:test";
import {createGraphReadWatchV1,wantedServerPolicyV1} from "../../../🛠️server/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

for(const row of fixture.cases)test(row.name,()=>{
 let time=0;
 const watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>time);
 for(const step of row.steps){
  time=step.at;
  const end=Math.min(step.line.length,73);
  watch.feed(step.line.slice(0,end));
  watch.feed(step.line.slice(end)+"\n");
  expect(watch.silentMilliseconds(),step.label).toBe(step.silent);
  expect(watch.verdict(),step.label).toBe(step.verdict);
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
